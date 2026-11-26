//! Domino cubanez (set dublu-9, 55 de piese), 2 jucători, cu stoc.
//!
//! Reguli folosite (tradiționale):
//! * Fiecare primește 10 piese; celelalte 35 rămân în **stoc** (ascuns).
//! * Începe cine are cel mai mare dublu (altfel cea mai mare piesă după sumă); prima
//!   mutare este obligatoriu acea piesă.
//! * Se așază o piesă la un capăt al lanțului (`S` = stânga, `D` = dreapta) cu
//!   numărul potrivit. Cine nu are piesă potrivită **pescuiește** din stoc (`pesc`,
//!   câte una, până poate juca); cu stocul gol și fără piesă spune `pas`.
//! * Runda se termină când cineva rămâne fără piese (câștigă și ia suma punctelor
//!   rămase adversarului) sau la **blocaj** (doi pași la rând): câștigă cine are
//!   mai puține puncte în mână și ia diferența; la egalitate = remiză.
//!
//! Mutări: `3-4:S`, `3-4:D` (prima piesă: cel mai mare dublu al celor doi, de ex. `9-9`), `pesc`, `pas`. Piesa se scrie
//! cu numărul mic întâi. Informația ascunsă și AI-ul: vezi `cards.rs`.

use crate::cards::{handover_banner, pl, shuffle, Viewer};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub type Tile = (u8, u8); // (mic, mare)

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum End {
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Place(Tile, End),
    Draw,
    Pass,
}

fn tile_text(t: Tile) -> String {
    format!("{}-{}", t.0, t.1)
}

fn join_tiles(v: &[Tile]) -> String {
    if v.is_empty() {
        return "—".into();
    }
    v.iter()
        .map(|&t| tile_text(t))
        .collect::<Vec<_>>()
        .join(" ")
}

fn pips(t: Tile) -> i32 {
    (t.0 + t.1) as i32
}

#[derive(Clone)]
pub struct Domino9 {
    stock: Vec<Tile>,
    hands: [Vec<Tile>; 2],
    /// Lanțul de la stânga la dreapta, fiecare piesă orientată (stânga, dreapta).
    chain: Vec<Tile>,
    side: Player,
    start: Tile,
    passes: u8,
    fresh: [u8; 2],
    winner: Option<Player>,
    blocked: bool,
    viewer: Viewer,
    handover: bool,
}

impl Default for Domino9 {
    fn default() -> Self {
        Self::new()
    }
}

impl Domino9 {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut all: Vec<Tile> = Vec::new();
        for a in 0..=9u8 {
            for b in a..=9u8 {
                all.push((a, b));
            }
        }
        shuffle(&mut all, &mut rng);
        let h1: Vec<Tile> = all.drain(..10).collect();
        let h2: Vec<Tile> = all.drain(..10).collect();
        let stock = all;
        // cine începe: cel mai mare dublu, altfel cea mai mare piesă
        let key = |t: Tile| (t.0 == t.1, pips(t), t.1);
        let b1 = *h1.iter().max_by_key(|&&t| key(t)).expect("tile");
        let b2 = *h2.iter().max_by_key(|&&t| key(t)).expect("tile");
        let (side, start) = if key(b1) >= key(b2) {
            (Player::One, b1)
        } else {
            (Player::Two, b2)
        };
        Domino9 {
            stock,
            hands: [h1, h2],
            chain: Vec::new(),
            side,
            start,
            passes: 0,
            fresh: [0, 0],
            winner: None,
            blocked: false,
            viewer: Viewer::Fixed(Player::One),
            handover: false,
        }
    }

    pub fn with_viewer(mut self, v: Viewer) -> Self {
        self.viewer = v;
        self
    }

    pub fn hot_seat(self) -> Self {
        self.with_viewer(Viewer::HotSeat)
    }

    fn ends(&self) -> Option<(u8, u8)> {
        Some((self.chain.first()?.0, self.chain.last()?.1))
    }

    fn over(&self) -> bool {
        self.winner.is_some() || self.blocked
    }

    fn hand_pips(&self, p: Player) -> i32 {
        self.hands[p.idx()].iter().map(|&t| pips(t)).sum()
    }

    /// Rezultatul rundei dacă s-a terminat: (câștigător, puncte).
    pub fn score(&self) -> Option<(Option<Player>, i32)> {
        if let Some(w) = self.winner {
            return Some((Some(w), self.hand_pips(w.other())));
        }
        if self.blocked {
            let (a, b) = (self.hand_pips(Player::One), self.hand_pips(Player::Two));
            return Some(match a.cmp(&b) {
                std::cmp::Ordering::Less => (Some(Player::One), b - a),
                std::cmp::Ordering::Greater => (Some(Player::Two), a - b),
                std::cmp::Ordering::Equal => (None, 0),
            });
        }
        None
    }
}

impl Game for Domino9 {
    type Move = Move;

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = match who {
            Some(p) => Viewer::Fixed(p),
            None => Viewer::HotSeat,
        };
    }

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over() {
            return Vec::new();
        }
        let hand = &self.hands[self.side.idx()];
        let Some((l, r)) = self.ends() else {
            return vec![Move::Place(self.start, End::Left)];
        };
        let mut v = Vec::new();
        for &t in hand {
            if t.0 == l || t.1 == l {
                v.push(Move::Place(t, End::Left));
            }
            if t.0 == r || t.1 == r {
                v.push(Move::Place(t, End::Right));
            }
        }
        if v.is_empty() {
            v.push(if self.stock.is_empty() {
                Move::Pass
            } else {
                Move::Draw
            });
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        self.fresh = [0, 0];
        self.handover = false;
        let me = self.side;
        match *m {
            Move::Place(t, end) => {
                let h = &mut self.hands[me.idx()];
                let pos = h.iter().position(|&x| x == t).expect("tile in hand");
                h.remove(pos);
                match (self.ends(), end) {
                    (None, _) => self.chain.push(t),
                    (Some((l, _)), End::Left) => {
                        let o = if t.1 == l { t } else { (t.1, t.0) };
                        self.chain.insert(0, o);
                    }
                    (Some((_, r)), End::Right) => {
                        let o = if t.0 == r { t } else { (t.1, t.0) };
                        self.chain.push(o);
                    }
                }
                self.passes = 0;
                if self.hands[me.idx()].is_empty() {
                    self.winner = Some(me);
                } else {
                    self.side = me.other();
                    self.handover = true;
                }
            }
            Move::Draw => {
                if let Some(t) = self.stock.pop() {
                    self.hands[me.idx()].push(t);
                    self.fresh[me.idx()] = 1;
                }
            }
            Move::Pass => {
                self.passes += 1;
                if self.passes >= 2 {
                    self.blocked = true;
                } else {
                    self.side = me.other();
                    self.handover = true;
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        let (w, _) = self.score()?;
        Some(match w {
            Some(p) => Outcome::Win(p),
            None => Outcome::Draw,
        })
    }

    /// Doar informație vizibilă lui `p`: lanțul, mâna lui `p` (fără piesa abia
    /// pescuită în simulare) și *numărul* pieselor adversarului. Din piesele
    /// văzute (lanț + mâna proprie) estimează câte piese necunoscute se potrivesc
    /// la capete; nu citește mâna adversarului sau stocul.
    fn evaluate(&self, p: Player) -> i32 {
        let q = p.other();
        let hand = &self.hands[p.idx()];
        let known = &hand[..hand.len().saturating_sub(self.fresh[p.idx()] as usize)];
        let mut s = 8 * (self.hands[q.idx()].len() as i32 - hand.len() as i32);
        s -= known.iter().map(|&t| pips(t)).sum::<i32>();
        if let Some((l, r)) = self.ends() {
            let mut seen = [0i32; 10];
            for &t in self.chain.iter().chain(known.iter()) {
                seen[t.0 as usize] += 1;
                if t.1 != t.0 {
                    seen[t.1 as usize] += 1;
                }
            }
            for e in [l, r] {
                // piese necunoscute care conțin numărul e (fiecare număr apare în 10 piese)
                let unseen = 10 - seen[e as usize];
                s -= 2 * unseen;
                if known.iter().any(|&t| t.0 == e || t.1 == e) {
                    s += 3;
                }
            }
        }
        s
    }

    fn render(&self) -> String {
        let mut s = String::new();
        if self.handover && !self.over() {
            if let Viewer::HotSeat = self.viewer {
                s.push_str(&handover_banner(self.side));
            }
        }
        s.push_str(&format!("Domino — stoc: {} piese\n", self.stock.len()));
        let chain: Vec<String> = self
            .chain
            .iter()
            .map(|&(a, b)| format!("{a}|{b}"))
            .collect();
        s.push_str(&format!(
            "Lanț: {}\n",
            if chain.is_empty() {
                "—".to_string()
            } else {
                chain.join(" ")
            }
        ));
        if let Some((w, pts)) = self.score() {
            s.push_str(&format!(
                "Mâna J1: {} | Mâna J2: {}\n",
                join_tiles(&self.hands[0]),
                join_tiles(&self.hands[1])
            ));
            match w {
                Some(p) => s.push_str(&format!(
                    "{} Jucătorul {} ia {} puncte.\n",
                    if self.blocked { "Blocaj:" } else { "Domino9!" },
                    pl(p),
                    pts
                )),
                None => s.push_str("Blocaj: egalitate.\n"),
            }
            return s;
        }
        let v = self.viewer.who(self.side);
        s.push_str(&format!(
            "Mâna Jucătorului {}: {}\n",
            pl(v),
            join_tiles(&self.hands[v.idx()])
        ));
        s.push_str(&format!(
            "Jucătorul {}: {} piese\n",
            pl(v.other()),
            self.hands[v.other().idx()].len()
        ));
        s.push_str(&format!("La mutare: Jucătorul {}\n", pl(self.side)));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Place(t, e) => {
                if self.chain.is_empty() {
                    tile_text(*t)
                } else {
                    format!(
                        "{}:{}",
                        tile_text(*t),
                        if *e == End::Left { 'S' } else { 'D' }
                    )
                }
            }
            Move::Draw => "pesc".into(),
            Move::Pass => "pas".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    #[test]
    fn set_is_55_tiles_10_each_35_stock() {
        let g = Domino9::with_seed(1);
        assert_eq!(g.hands[0].len(), 10);
        assert_eq!(g.hands[1].len(), 10);
        assert_eq!(g.stock.len(), 35);
        let mut all: Vec<Tile> = g.hands[0]
            .iter()
            .chain(&g.hands[1])
            .chain(&g.stock)
            .copied()
            .collect();
        all.sort();
        all.dedup();
        assert_eq!(all.len(), 55);
        assert!(all.contains(&(9, 9)));
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=15u64 {
            let mut g = Domino9::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())].clone());
                n += 1;
                assert!(n < 400);
            }
        }
    }

    #[test]
    fn ai_does_not_read_the_opponent_or_stock() {
        let a = Domino9::with_seed(2);
        let mut b = a.clone();
        b.hands[1].reverse();
        b.stock.reverse();
        assert_eq!(
            polytabletop_core::GameObj::ai_move(&a, 1),
            polytabletop_core::GameObj::ai_move(&b, 1)
        );
    }
}
