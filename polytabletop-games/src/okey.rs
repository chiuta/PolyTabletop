//! Okey (Turcia), adaptat pentru 2 jucători, din regulile publice ale jocului.
//!
//! * 106 de plăci: 4 culori (`r` roșu, `y` galben, `b` albastru, `k` negru) × numerele 1–13,
//!   în două exemplare, plus 2 „okey false” (`f`). Se întoarce o placă-indicator; **okey**
//!   (joker) este placa de aceeași culoare cu numărul următor (13 → 1); ambele ei exemplare
//!   sunt jokere. Okey-ul fals ține locul plăcii okey, dar ca placă obișnuită.
//! * Jucătorul 1 primește 15 plăci și începe aruncând una; Jucătorul 2 primește 14. La rândul
//!   tău tragi o placă din teanc (`trage`) sau iei ultima placă aruncată de adversar (`ia`), apoi
//!   arunci una (`ata-r5`). Pachetul gol înseamnă remiză.
//! * Câștigi când cele 14 plăci rămase se împart complet în **serii** (≥ 3 numere consecutive
//!   de aceeași culoare; 1 poate urma după 13) și **grupe** (3–4 plăci cu același număr și culori
//!   diferite); jokerul ține loc de orice. Declari cu `bitir-r5` (arunci placa r5 și închizi).
//!
//! Adaptări (documentate): 2 jucători în loc de 4; fără punctaj pe rânduri (o singură mână);
//! fără închiderea cu perechi și fără dublarea pentru încheierea cu okey.
//!
//! Informație ascunsă: mâna adversarului și teancul; AI-ul evaluează doar mâna lui, plăcile
//! aruncate (publice) și numărul plăcilor adversarului; placa abia trasă nu e folosită la alegere.

use crate::cards::Viewer;
use crate::cards52::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::collections::HashMap;

const COLORS: [char; 4] = ['r', 'y', 'b', 'k'];
const FAKE_A: u8 = 104;

fn is_fake(id: u8) -> bool {
    id >= FAKE_A
}

/// culoare, număr (1..=13) al plăcii naturale (pentru fals: nu se folosește aici)
fn face(id: u8) -> (u8, u8) {
    let x = id % 52;
    (x / 13, x % 13 + 1)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Draw,
    Take,
    Discard(u8),
    Finish(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Draw,
    Discard,
}

#[derive(Clone)]
pub struct Okey {
    viewer: Viewer,
    stock: Vec<u8>,
    hands: [Vec<u8>; 2],
    discards: [Vec<u8>; 2],
    indicator: u8,
    okey: (u8, u8),
    side: Player,
    phase: Phase,
    fresh: u8,
    winner: Option<Option<Player>>,
}

impl Default for Okey {
    fn default() -> Self {
        Self::new()
    }
}

/// Numărătoare pe (culoare, număr) + jokere.
#[derive(Clone, Hash, PartialEq, Eq)]
struct Cnt {
    c: [[u8; 14]; 4],
    wild: u8,
}

fn first_tile(c: &Cnt) -> Option<(usize, usize)> {
    for col in 0..4 {
        for n in 1..=13 {
            if c.c[col][n] > 0 {
                return Some((col, n));
            }
        }
    }
    None
}

/// Numărul maxim de plăci acoperite de serii/grupe complete (cu jokerele folosite).
fn cover(c: &Cnt, memo: &mut HashMap<Cnt, u8>) -> u8 {
    if let Some(&v) = memo.get(c) {
        return v;
    }
    let Some((col, n)) = first_tile(c) else {
        memo.insert(c.clone(), 0);
        return 0;
    };
    // 1) placa rămâne neacoperită
    let mut skip = c.clone();
    skip.c[col][n] -= 1;
    let mut best = cover(&skip, memo);
    // 2) grupă (set): număr n, culori diferite
    {
        let others: Vec<usize> = (0..4).filter(|&k| k != col && c.c[k][n] > 0).collect();
        // alegem subseturi de culori din `others` (0..=3) completate cu jokere până la 3 sau 4
        for mask in 0..(1u32 << others.len()) {
            let used: Vec<usize> = (0..others.len())
                .filter(|&i| mask >> i & 1 == 1)
                .map(|i| others[i])
                .collect();
            let have = 1 + used.len();
            for size in [3usize, 4] {
                if have > size {
                    continue;
                }
                let need = (size - have) as u8;
                if need > c.wild {
                    continue;
                }
                // jokerele ocupă culori libere: în total culorile folosite ≤ 4
                if size > 4 {
                    continue;
                }
                let mut nx = c.clone();
                nx.c[col][n] -= 1;
                for &k in &used {
                    nx.c[k][n] -= 1;
                }
                nx.wild -= need;
                let v = size as u8 + cover(&nx, memo);
                best = best.max(v);
            }
        }
    }
    // 3) serie: poziții 1..=14 (14 = numărul 1 după 13)
    let poss: Vec<usize> = if n == 1 { vec![1, 14] } else { vec![n] };
    for pos in poss {
        for s in 1..=pos {
            if pos - s > 12 {
                continue;
            }
            for e in pos.max(s + 2)..=14 {
                if e - s + 1 < 3 || e - s + 1 > 13 {
                    continue;
                }
                let mut nx = c.clone();
                let mut ok = true;
                let mut wilds = 0u8;
                for p in s..=e {
                    if p == pos {
                        nx.c[col][n] -= 1;
                        continue;
                    }
                    let num = if p == 14 { 1 } else { p };
                    if nx.c[col][num] > 0 {
                        nx.c[col][num] -= 1;
                    } else {
                        wilds += 1;
                    }
                }
                if wilds > nx.wild {
                    ok = false;
                }
                if ok {
                    nx.wild -= wilds;
                    let v = (e - s + 1) as u8 + cover(&nx, memo);
                    best = best.max(v);
                }
            }
        }
    }
    memo.insert(c.clone(), best);
    best
}

impl Okey {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut st: Vec<u8> = (0..106).collect();
        shuffle(&mut st, &mut rng);
        let mut indicator = st.pop().unwrap();
        while is_fake(indicator) {
            st.insert(0, indicator);
            indicator = st.pop().unwrap();
        }
        let (c, n) = face(indicator);
        let okey = (c, if n == 13 { 1 } else { n + 1 });
        let mut h0 = Vec::new();
        let mut h1 = Vec::new();
        for _ in 0..14 {
            h0.push(st.pop().unwrap());
            h1.push(st.pop().unwrap());
        }
        h0.push(st.pop().unwrap());
        Okey {
            viewer: Viewer::Fixed(Player::One),
            stock: st,
            hands: [h0, h1],
            discards: [Vec::new(), Vec::new()],
            indicator,
            okey,
            side: Player::One,
            phase: Phase::Discard,
            fresh: 0,
            winner: None,
        }
    }

    pub fn okey_text(&self) -> String {
        format!("{}{}", COLORS[self.okey.0 as usize], self.okey.1)
    }

    fn is_wild(&self, id: u8) -> bool {
        !is_fake(id) && face(id) == self.okey
    }

    fn tile_face(&self, id: u8) -> (u8, u8) {
        if is_fake(id) {
            self.okey
        } else {
            face(id)
        }
    }

    pub fn tile_text(&self, id: u8) -> String {
        if is_fake(id) {
            "f".into()
        } else {
            let (c, n) = face(id);
            format!("{}{}", COLORS[c as usize], n)
        }
    }

    fn counts(&self, tiles: &[u8]) -> Cnt {
        let mut c = Cnt {
            c: [[0; 14]; 4],
            wild: 0,
        };
        for &t in tiles {
            if self.is_wild(t) {
                c.wild += 1;
            } else {
                let (col, n) = self.tile_face(t);
                c.c[col as usize][n as usize] += 1;
            }
        }
        c
    }

    /// Scor euristic al unei mâini de 14 plăci.
    fn score(&self, tiles: &[u8]) -> i32 {
        let c = self.counts(tiles);
        let mut memo = HashMap::new();
        let cov = cover(&c, &mut memo) as i32;
        let mut pairs = 0;
        for col in 0..4 {
            for n in 1..=13 {
                let a = c.c[col][n] as i32;
                if a == 0 {
                    continue;
                }
                if n < 13 {
                    pairs += a.min(c.c[col][n + 1] as i32);
                }
                if n < 12 {
                    pairs += a.min(c.c[col][n + 2] as i32);
                }
                for k in col + 1..4 {
                    pairs += a.min(c.c[k][n] as i32);
                }
            }
        }
        12 * cov + 2 * pairs + 6 * c.wild as i32
    }

    fn wins_with(&self, tiles: &[u8]) -> bool {
        let c = self.counts(tiles);
        let mut memo = HashMap::new();
        cover(&c, &mut memo) as usize == tiles.len()
    }

    fn can_finish_with(&self, hand: &[u8], discard: u8) -> bool {
        let mut rest = hand.to_vec();
        let pos = rest.iter().position(|&t| t == discard).unwrap();
        rest.remove(pos);
        self.wins_with(&rest)
    }

    pub fn hand_of(&self, p: Player) -> &[u8] {
        &self.hands[p.idx()]
    }

    pub fn stock_len(&self) -> usize {
        self.stock.len()
    }
}

impl Game for Okey {
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
        if self.winner.is_some() {
            return Vec::new();
        }
        let me = self.side.idx();
        match self.phase {
            Phase::Draw => {
                let mut v = vec![Move::Draw];
                if !self.discards[1 - me].is_empty() {
                    v.push(Move::Take);
                }
                v
            }
            Phase::Discard => {
                let mut seen: Vec<String> = Vec::new();
                let mut v = Vec::new();
                let mut fin = Vec::new();
                for &t in &self.hands[me] {
                    let key = self.tile_text(t);
                    if seen.contains(&key) {
                        continue;
                    }
                    seen.push(key);
                    if self.can_finish_with(&self.hands[me], t) {
                        fin.push(Move::Finish(t));
                    }
                    v.push(Move::Discard(t));
                }
                fin.extend(v);
                fin
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        let me = self.side.idx();
        self.fresh = 0;
        match *m {
            Move::Draw => {
                if let Some(t) = self.stock.pop() {
                    self.hands[me].push(t);
                    self.fresh = 1;
                    self.phase = Phase::Discard;
                } else {
                    self.winner = Some(None);
                }
            }
            Move::Take => {
                let t = self.discards[1 - me].pop().expect("discard to take");
                self.hands[me].push(t);
                self.phase = Phase::Discard;
            }
            Move::Discard(t) | Move::Finish(t) => {
                let pos = self.hands[me].iter().position(|&x| x == t).expect("tile");
                self.hands[me].remove(pos);
                self.discards[me].push(t);
                if matches!(m, Move::Finish(_)) {
                    self.winner = Some(Some(self.side));
                    return;
                }
                self.side = self.side.other();
                self.phase = Phase::Draw;
                if self.stock.is_empty() && self.discards[me].is_empty() {
                    self.winner = Some(None);
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.winner.map(|w| match w {
            Some(p) => Outcome::Win(p),
            None => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        if let Some(w) = self.winner {
            return match w {
                Some(q) if q == p => 1_000_000,
                Some(_) => -1_000_000,
                None => 0,
            };
        }
        let h = &self.hands[p.idx()];
        let known: Vec<u8> = if p == self.side && self.fresh > 0 {
            h[..h.len() - self.fresh as usize].to_vec()
        } else {
            h.clone()
        };
        let base = if known.len() % 2 == 1 {
            // 15 plăci: arunc cea mai puțin utilă
            let mut best = i32::MIN;
            let mut seen: Vec<u8> = Vec::new();
            for (i, &t) in known.iter().enumerate() {
                let key = if is_fake(t) { 200 } else { t % 52 };
                if seen.contains(&key) {
                    continue;
                }
                seen.push(key);
                let mut r = known.clone();
                r.remove(i);
                if self.wins_with(&r) {
                    return 500_000;
                }
                best = best.max(self.score(&r));
            }
            best
        } else {
            self.score(&known)
        };
        let unknown_bonus = if p == self.side && self.fresh > 0 {
            6
        } else {
            0
        };
        base + unknown_bonus
    }

    fn render(&self) -> String {
        let v = self.viewer.who(self.side);
        let mut s = String::new();
        s.push_str(&format!(
            "Indicator: {}   Okey (joker): {}   Teanc: {} plăci\n",
            self.tile_text(self.indicator),
            self.okey_text(),
            self.stock.len()
        ));
        for p in 0..2 {
            let top = self.discards[p].last().map(|&t| self.tile_text(t));
            s.push_str(&format!(
                "Aruncate J{}: {} (ultima: {})\n",
                p + 1,
                self.discards[p].len(),
                top.unwrap_or_else(|| "—".into())
            ));
        }
        s.push_str(&format!(
            "J{} (ascuns) ține {} plăci.\n",
            v.other().idx() + 1,
            self.hands[v.other().idx()].len()
        ));
        let mut h = self.hands[v.idx()].clone();
        h.sort_unstable_by_key(|&t| {
            if is_fake(t) {
                (9, 0)
            } else {
                let (c, n) = face(t);
                (c, n)
            }
        });
        let txt: Vec<String> = h
            .iter()
            .map(|&t| {
                if self.is_wild(t) {
                    format!("{}*", self.tile_text(t))
                } else {
                    self.tile_text(t)
                }
            })
            .collect();
        s.push_str(&format!(
            "Mâna ta (Jucător {}): {}   (* = okey/joker, f = okey fals)\n",
            v.idx() + 1,
            txt.join(" ")
        ));
        match self.winner {
            Some(Some(p)) => s.push_str(&format!(
                "Jucătorul {} a închis mâna și câștigă.\n",
                p.idx() + 1
            )),
            Some(None) => s.push_str("Teancul s-a golit: remiză.\n"),
            None => s.push_str(&format!(
                "La mutare: Jucător {} ({})\n",
                self.side.idx() + 1,
                if self.phase == Phase::Draw {
                    "trage sau ia"
                } else {
                    "aruncă sau închide"
                }
            )),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match *m {
            Move::Draw => "trage".into(),
            Move::Take => "ia".into(),
            Move::Discard(t) => format!("ata-{}", self.tile_text(t)),
            Move::Finish(t) => format!("bitir-{}", self.tile_text(t)),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn nat(color: u8, n: u8) -> u8 {
        color * 13 + n - 1
    }

    fn game(okey: (u8, u8)) -> Okey {
        let mut g = Okey::with_seed(1);
        g.okey = okey;
        g.hands = [Vec::new(), Vec::new()];
        g.discards = [Vec::new(), Vec::new()];
        g.winner = None;
        g.fresh = 0;
        g
    }

    #[test]
    fn deal_sizes() {
        let g = Okey::with_seed(5);
        assert_eq!(g.hands[0].len(), 15);
        assert_eq!(g.hands[1].len(), 14);
        assert_eq!(g.stock.len(), 106 - 29 - 1);
        assert!(!is_fake(g.indicator));
        let (c, n) = face(g.indicator);
        assert_eq!(g.okey, (c, if n == 13 { 1 } else { n + 1 }));
    }

    #[test]
    fn run_and_set_win() {
        let g = game((0, 5)); // okey = r5
                              // r1 r2 r3 | y7 b7 k7 | b10 b11 b12 b13 | k1 k2 k3 (14)
        let hand = vec![
            nat(0, 1),
            nat(0, 2),
            nat(0, 3),
            nat(1, 7),
            nat(2, 7),
            nat(3, 7),
            nat(2, 10),
            nat(2, 11),
            nat(2, 12),
            nat(2, 13),
            nat(3, 1),
            nat(3, 2),
            nat(3, 3),
            // al 14-lea: complet de la k3... folosim 13 + un k4
            nat(3, 4),
        ];
        assert!(g.wins_with(&hand));
    }

    #[test]
    fn wild_fills_a_gap_and_fake_is_natural() {
        let g = game((0, 5)); // okey r5 (jokerul real = id r5)
                              // y1 y2 [joker] y4 : joker pe 3
        let w = nat(0, 5);
        let hand = vec![nat(1, 1), nat(1, 2), w, nat(1, 4)];
        assert!(g.wins_with(&hand));
        // placa falsă e r5 natural: r4 r5(fals) r6 e serie validă fără joker
        let hand2 = vec![nat(0, 4), FAKE_A, nat(0, 6)];
        assert!(g.wins_with(&hand2));
        // dar y4 y6 + fals (r5) nu formează serie
        let hand3 = vec![nat(1, 4), FAKE_A, nat(1, 6)];
        assert!(!g.wins_with(&hand3));
    }

    #[test]
    fn one_can_follow_thirteen_but_not_wrap_further() {
        let g = game((0, 5));
        assert!(g.wins_with(&[nat(1, 12), nat(1, 13), nat(1, 1)]));
        assert!(!g.wins_with(&[nat(1, 13), nat(1, 1), nat(1, 2)]));
    }

    #[test]
    fn set_needs_distinct_colors() {
        let g = game((0, 5));
        assert!(!g.wins_with(&[nat(1, 7), nat(1, 7) + 52, nat(2, 7)]));
        assert!(g.wins_with(&[nat(1, 7), nat(3, 7), nat(2, 7)]));
        assert!(g.wins_with(&[nat(1, 7), nat(3, 7), nat(2, 7), nat(0, 7)]));
    }

    #[test]
    fn finish_is_offered_only_when_closing() {
        let mut g = game((0, 5));
        g.side = Player::One;
        g.phase = Phase::Discard;
        // 14 plăci câștigătoare + o placă în plus (b1)
        g.hands[0] = vec![
            nat(0, 1),
            nat(0, 2),
            nat(0, 3),
            nat(1, 7),
            nat(2, 7),
            nat(3, 7),
            nat(2, 10),
            nat(2, 11),
            nat(2, 12),
            nat(2, 13),
            nat(3, 1),
            nat(3, 2),
            nat(3, 3),
            nat(3, 4),
            nat(2, 1),
        ];
        let mv = g.legal_moves();
        assert!(mv
            .iter()
            .any(|m| matches!(m, Move::Finish(t) if *t == nat(2, 1))));
        let t = nat(2, 1);
        g.apply(&Move::Finish(t));
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn take_gives_the_opponents_last_discard() {
        let mut g = Okey::with_seed(2);
        let t = g.hands[0][0];
        g.apply(&Move::Discard(t));
        assert_eq!(g.to_move(), Player::Two);
        assert!(g.legal_moves().contains(&Move::Take));
        g.apply(&Move::Take);
        assert_eq!(g.hands[1].len(), 15);
        assert!(g.hands[1].contains(&t));
    }

    #[test]
    fn evaluate_ignores_the_opponents_tiles_and_the_fresh_one() {
        let g = Okey::with_seed(9);
        for p in [Player::One, Player::Two] {
            let q = p.other().idx();
            let mut h = g.clone();
            let (a, b) = (h.hands[q][0], h.stock[3]);
            h.hands[q][0] = b;
            h.stock[3] = a;
            assert_eq!(g.evaluate(p), h.evaluate(p));
        }
        // aceeași mână cu altă placă trasă: evaluarea celui care trage nu se schimbă
        let mut d1 = Okey::with_seed(10);
        let t = d1.hands[0][0];
        d1.apply(&Move::Discard(t));
        let mut d2 = d1.clone();
        let l = d2.stock.len();
        d2.stock.swap(l - 1, 0);
        d1.apply(&Move::Draw);
        d2.apply(&Move::Draw);
        assert_eq!(d1.evaluate(Player::Two), d2.evaluate(Player::Two));
    }

    #[test]
    fn render_hides_the_opponent() {
        let g = Okey::with_seed(5);
        assert!(g.render().contains("J2 (ascuns) ține 14 plăci"));
    }

    #[test]
    fn random_games_terminate() {
        for seed in 1..=6u64 {
            let mut g = Okey::with_seed(seed);
            let mut rng = Rng::new(seed ^ 0x99);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 2000);
            }
        }
    }

    #[test]
    fn ai_plays_a_full_game_quickly() {
        use polytabletop_core::GameObj;
        let mut g: Box<dyn GameObj> = Box::new(Okey::with_seed(3));
        let mut n = 0;
        while g.outcome().is_none() && n < 400 {
            let mv = g.ai_move(n).unwrap();
            g.play(&mv).unwrap();
            n += 1;
        }
        assert!(n > 4);
    }
}
