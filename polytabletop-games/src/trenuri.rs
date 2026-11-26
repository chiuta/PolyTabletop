//! Trenuri de domino (variantă proprie cu două persoane, informație ascunsă): jocul de domino
//! dublu-9 fără fișa de mijloc: dublul 9–9 e „gara”, din care pornesc trenurile. Fiecare
//! jucător are un tren propriu și există un tren public, deschis tuturor; trenul adversarului
//! se deschide doar dacă acela a trebuit să treacă la tura lui (are „semn” pe tren). Pe rând
//! pui o piesă al cărei capăt se potrivește cu capătul liber al unui tren permis. Un dublu
//! pus te obligă să mai pui o piesă pe același tren (dacă nu poți, tragi una și, dacă nici
//! atunci, treci; dublul rămâne de împlinit). Dacă nu ai nicio piesă potrivită, tragi din
//! stoc (`trag`) și poți pune acea piesă, altfel treci (`pas`, semn pe trenul tău). Câștigă
//! cine rămâne primul fără piese; dacă amândoi trec la rând cu stocul gol, câștigă cine are mai
//! puține puncte în mână. Informație ascunsă: vezi doar mâna ta.
//! Mutări: `6-4 m` (piesa cu capătul 6 spre tren, urmat de 4; m = trenul meu, p = public,
//! a = trenul adversarului), `trag`, `pas`.
#![allow(clippy::all)]

use crate::cards::{handover_banner, Viewer};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

type Tile = (u8, u8);

#[derive(Clone)]
pub struct Trenuri {
    hand: [Vec<Tile>; 2],
    yard: Vec<Tile>,
    ends: [u8; 3], // trenul lui J1, trenul lui J2, public
    len: [u8; 3],
    marker: [bool; 2],
    must: Option<usize>,
    drew: bool,
    side: Player,
    prev: Player,
    passes: u8,
    won: Option<Player>,
    viewer: Viewer,
    ply: u32,
}

impl Default for Trenuri {
    fn default() -> Self {
        Self::new()
    }
}

fn pips(h: &[Tile]) -> i32 {
    h.iter().map(|t| t.0 as i32 + t.1 as i32).sum()
}

impl Trenuri {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut all: Vec<Tile> = vec![];
        for a in 0..=9u8 {
            for b in a..=9 {
                if (a, b) != (9, 9) {
                    all.push((a, b));
                }
            }
        }
        for i in (1..all.len()).rev() {
            all.swap(i, rng.below(i + 1));
        }
        let mut h0: Vec<Tile> = all.drain(..10).collect();
        let mut h1: Vec<Tile> = all.drain(..10).collect();
        h0.sort();
        h1.sort();
        Trenuri {
            hand: [h0, h1],
            yard: all,
            ends: [9; 3],
            len: [0; 3],
            marker: [false; 2],
            must: None,
            drew: false,
            side: Player::One,
            prev: Player::One,
            passes: 0,
            won: None,
            viewer: Viewer::Fixed(Player::One),
            ply: 0,
        }
    }
    fn trains(&self, p: usize) -> Vec<(usize, char)> {
        let mut v = vec![(p, 'm'), (2, 'p')];
        if self.marker[1 - p] {
            v.push((1 - p, 'a'));
        }
        v
    }
    fn plays(&self) -> Vec<(Tile, usize, char)> {
        let p = self.side.idx();
        let mut v = vec![];
        for &(t, c) in &self.trains(p) {
            if let Some(m) = self.must {
                if m != t {
                    continue;
                }
            }
            let e = self.ends[t];
            for &tile in &self.hand[p] {
                let o = if tile.0 == e {
                    Some((tile.0, tile.1))
                } else if tile.1 == e {
                    Some((tile.1, tile.0))
                } else {
                    None
                };
                if let Some(o) = o {
                    let x = (o, t, c);
                    if !v.contains(&x) {
                        v.push(x);
                    }
                }
            }
        }
        v
    }
    fn over(&self) -> bool {
        self.won.is_some() || self.passes >= 2
    }
}

impl Game for Trenuri {
    // (capăt, rest, tren) ; capăt 100 = trag, 101 = pas
    type Move = (u8, u8, u8);
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<(u8, u8, u8)> {
        if self.over() || self.ply > 400 {
            return vec![];
        }
        let mut v: Vec<(u8, u8, u8)> = self
            .plays()
            .into_iter()
            .map(|(t, tr, _)| (t.0, t.1, tr as u8))
            .collect();
        if v.is_empty() || self.drew {
            if !self.drew && !self.yard.is_empty() && v.is_empty() {
                v.push((100, 0, 0));
            } else {
                v.push((101, 0, 0));
            }
        }
        if self.drew && !v.contains(&(101, 0, 0)) {
            v.push((101, 0, 0));
        }
        v
    }
    fn apply(&mut self, m: &(u8, u8, u8)) {
        let p = self.side.idx();
        self.ply += 1;
        match m.0 {
            100 => {
                let t = self.yard.pop().unwrap();
                self.hand[p].push(t);
                self.hand[p].sort();
                self.drew = true;
            }
            101 => {
                self.marker[p] = true;
                self.passes += 1;
                self.drew = false;
                self.prev = self.side;
                self.side = self.side.other();
            }
            _ => {
                let t = m.2 as usize;
                let tile = if m.0 <= m.1 { (m.0, m.1) } else { (m.1, m.0) };
                let k = self.hand[p].iter().position(|&x| x == tile).unwrap();
                self.hand[p].remove(k);
                self.ends[t] = m.1;
                self.len[t] += 1;
                if t == p {
                    self.marker[p] = false;
                }
                self.passes = 0;
                self.drew = false;
                if self.hand[p].is_empty() {
                    self.won = Some(self.side);
                    return;
                }
                if m.0 == m.1 {
                    self.must = Some(t);
                } else {
                    if self.must == Some(t) {
                        self.must = None;
                    }
                    self.prev = self.side;
                    self.side = self.side.other();
                    return;
                }
                if self.must == Some(t) && m.0 != m.1 {
                    self.must = None;
                }
            }
        }
        if self.passes >= 2 && self.yard.is_empty() {
            // blocat
        }
    }
    fn outcome(&self) -> Option<Outcome> {
        if let Some(w) = self.won {
            return Some(Outcome::Win(w));
        }
        if self.passes >= 2 || self.ply > 400 {
            let (a, b) = (pips(&self.hand[0]), pips(&self.hand[1]));
            return Some(match a.cmp(&b) {
                std::cmp::Ordering::Less => Outcome::Win(Player::One),
                std::cmp::Ordering::Greater => Outcome::Win(Player::Two),
                _ => Outcome::Draw,
            });
        }
        None
    }
    fn evaluate(&self, p: Player) -> i32 {
        // doar ce vede `p`: mâna proprie, dimensiunea mâinii adversarului, semnele
        let me = p.idx();
        let mut s = -pips(&self.hand[me]) * 3 - self.hand[me].len() as i32 * 12
            + self.hand[1 - me].len() as i32 * 12;
        if self.marker[me] {
            s -= 10;
        }
        if self.marker[1 - me] {
            s += 6;
        }
        s += self.len[me] as i32 * 2;
        s
    }
    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = match who {
            Some(p) => Viewer::Fixed(p),
            None => Viewer::HotSeat,
        };
    }
    fn render(&self) -> String {
        let mut s = String::new();
        if self.prev != self.side && !self.over() {
            if let Viewer::HotSeat = self.viewer {
                s.push_str(&handover_banner(self.side));
            }
        }
        s.push_str("Gara: 9–9. Capete libere: ");
        s.push_str(&format!(
            "tren J1 {}{}, tren J2 {}{}, public {}\n",
            self.ends[0],
            if self.marker[0] { " (deschis)" } else { "" },
            self.ends[1],
            if self.marker[1] { " (deschis)" } else { "" },
            self.ends[2]
        ));
        s.push_str(&format!(
            "Stoc: {} piese. Piese puse: J1 {}, J2 {}, public {}\n",
            self.yard.len(),
            self.len[0],
            self.len[1],
            self.len[2]
        ));
        if let Some(t) = self.must {
            s.push_str(&format!(
                "Dublu de împlinit pe trenul {}.\n",
                ["J1", "J2", "public"][t]
            ));
        }
        let v = self.viewer.who(self.side);
        let fmt = |h: &[Tile]| {
            h.iter()
                .map(|t| format!("{}-{}", t.0, t.1))
                .collect::<Vec<_>>()
                .join(" ")
        };
        s.push_str(&format!(
            "Piesele tale (J{}): {}\n",
            v.idx() + 1,
            fmt(&self.hand[v.idx()])
        ));
        s.push_str(&format!(
            "J{} are {} piese\n",
            v.other().idx() + 1,
            self.hand[v.other().idx()].len()
        ));
        if self.outcome().is_some() {
            s.push_str(&format!(
                "Mâinile la final: J1 {} | J2 {}\n",
                fmt(&self.hand[0]),
                fmt(&self.hand[1])
            ));
        }
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        s
    }
    fn format_move(&self, m: &(u8, u8, u8)) -> String {
        match m.0 {
            100 => "trag".into(),
            101 => "pas".into(),
            _ => {
                let p = self.side.idx();
                let c = if m.2 as usize == p {
                    'm'
                } else if m.2 == 2 {
                    'p'
                } else {
                    'a'
                };
                format!("{}-{} {}", m.0, m.1, c)
            }
        }
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::b1_util::testing::check;
    #[test]
    fn deal_and_hidden() {
        let g = Trenuri::with_seed(1);
        assert_eq!(g.hand[0].len(), 10);
        assert_eq!(g.yard.len(), 34);
        let mut a = g.clone();
        a.set_viewer(Some(Player::One));
        let mut b = g.clone();
        b.set_viewer(Some(Player::Two));
        let h1 = g.hand[0]
            .iter()
            .map(|t| format!("{}-{}", t.0, t.1))
            .collect::<Vec<_>>()
            .join(" ");
        assert!(a.render().contains(&h1));
        assert!(!b.render().contains(&h1));
        // hot-seat: arată mâna celui la mutare
        let mut h = g.clone();
        h.set_viewer(None);
        assert_eq!(
            h.render()
                .lines()
                .find(|l| l.starts_with("Piesele tale"))
                .map(|l| l.contains("J1")),
            Some(true)
        );
    }
    #[test]
    fn double_forces_follow_up() {
        let mut g = Trenuri::with_seed(2);
        g.hand[0] = vec![(9, 9 - 0), (9, 3), (9, 1), (2, 2)];
        g.hand[0][0] = (8, 8);
        g.hand[0].push((8, 9));
        g.ends[0] = 8;
        g.apply(&(8, 8, 0));
        assert_eq!(g.must, Some(0));
        assert_eq!(g.to_move(), Player::One);
        assert!(g.legal_moves().iter().all(|m| m.2 == 0));
    }
    #[test]
    fn random_and_ai() {
        check(Trenuri::with_seed, 700);
    }
}
