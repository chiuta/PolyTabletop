#![allow(clippy::needless_range_loop, clippy::manual_range_contains)]
//! Ô ăn quan (Vietnam) — joc de semănat și capturat cu două „mandarine”, 2 jucători.
//!
//! * Tabla: un inel de 12 gropi. Pe fiecare latură lungă 5 gropi mici (dân) cu câte 5 pietre; la
//!   capete două gropi mari (quan), fiecare cu un mandarin (10 puncte). Jucătorul 1 are rândul de
//!   jos (coloanele 1–5 de la stânga la dreapta), jucătorul 2 rândul de sus.
//! * Mutare: alegi o groapă mică proprie nevidă și un sens, **în sensul acelor de ceasornic** (`3+`)
//!   sau **invers** (`3-`). Iei toate pietrele și le semeni câte una în gropile următoare.
//! * După ultima piatră: dacă groapa următoare e mică și nevidă, iei pietrele ei și continui să semeni;
//!   dacă e un quan, mutarea se oprește; dacă e goală, iar cea de după ea e plină (inclusiv un quan),
//!   o **capturezi** (pietrele + 10 pentru mandarin) și, dacă urmează iar o groapă goală și apoi una
//!   plină, capturezi și pe aceea. Altfel mutarea se termină.
//! * Dacă rândul tău e gol, mutarea e să pui 5 pietre (din câștigurile tale, câte una pe groapă);
//!   dacă n-ai 5, jocul se termină. Jocul se termină și când ambii mandarini sunt capturați
//!   (după 600 de mutări se numără oricum). La final pietrele rămase pe rândul fiecăruia sunt
//!   ale lui; câștigă cine are mai multe puncte.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const CAP: u32 = 600;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Sow(u8, bool),
    Refill,
}

#[derive(Clone)]
pub struct OAnQuan {
    /// inelul de 12 gropi: 0 = quan stânga, 1..=5 rândul lui J2 (stânga→dreapta), 6 = quan dreapta,
    /// 7..=11 rândul lui J1 (dreapta→stânga)
    s: [u8; 12],
    mandarin: [bool; 2],
    score: [u32; 2],
    side: Player,
    ply: u32,
    ended: bool,
    log: String,
}

impl Default for OAnQuan {
    fn default() -> Self {
        Self::new()
    }
}

fn is_quan(i: usize) -> bool {
    i == 0 || i == 6
}

/// Indicele de pe inel al coloanei `col` (1..=5) a jucătorului `p`.
fn cell_of(p: Player, col: usize) -> usize {
    match p {
        Player::One => 12 - col,
        Player::Two => col,
    }
}

impl OAnQuan {
    pub fn new() -> Self {
        let mut s = [5u8; 12];
        s[0] = 0;
        s[6] = 0;
        OAnQuan {
            s,
            mandarin: [true, true],
            score: [0, 0],
            side: Player::One,
            ply: 0,
            ended: false,
            log: String::new(),
        }
    }

    pub fn scores(&self) -> (u32, u32) {
        (self.score[0], self.score[1])
    }

    fn quan_value(&self, i: usize) -> u32 {
        let k = if i == 0 { 0 } else { 1 };
        self.s[i] as u32 + if self.mandarin[k] { 10 } else { 0 }
    }

    fn quan_filled(&self, i: usize) -> bool {
        self.quan_value(i) > 0
    }

    fn filled(&self, i: usize) -> bool {
        if is_quan(i) {
            self.quan_filled(i)
        } else {
            self.s[i] > 0
        }
    }

    fn take(&mut self, i: usize, p: usize) {
        if is_quan(i) {
            let k = if i == 0 { 0 } else { 1 };
            self.score[p] += self.quan_value(i);
            self.mandarin[k] = false;
        } else {
            self.score[p] += self.s[i] as u32;
        }
        self.s[i] = 0;
    }

    fn row_empty(&self, p: Player) -> bool {
        (1..=5).all(|c| self.s[cell_of(p, c)] == 0)
    }

    fn finish(&mut self) {
        self.ended = true;
        for p in [Player::One, Player::Two] {
            for c in 1..=5 {
                let i = cell_of(p, c);
                self.score[p.idx()] += self.s[i] as u32;
                self.s[i] = 0;
            }
        }
    }

    fn step(&mut self, mut pos: usize, dirp: bool, mut hand: u32) {
        let d: i32 = if dirp { 1 } else { -1 };
        let nx = |i: usize| ((i as i32 + d).rem_euclid(12)) as usize;
        let me = self.side.idx();
        loop {
            while hand > 0 {
                pos = nx(pos);
                self.s[pos] += 1;
                hand -= 1;
            }
            let next = nx(pos);
            if self.filled(next) {
                if is_quan(next) {
                    return;
                }
                hand = self.s[next] as u32;
                self.s[next] = 0;
                pos = next;
                continue;
            }
            // groapa următoare e goală: capturi în lanț
            let mut at = next;
            loop {
                let cap = nx(at);
                if !self.filled(cap) {
                    return;
                }
                self.take(cap, me);
                at = nx(cap);
                if self.filled(at) {
                    return;
                }
            }
        }
    }
}

impl Game for OAnQuan {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        if self.row_empty(self.side) {
            return vec![Move::Refill];
        }
        let mut v = Vec::new();
        for c in 1..=5u8 {
            if self.s[cell_of(self.side, c as usize)] > 0 {
                v.push(Move::Sow(c, true));
                v.push(Move::Sow(c, false));
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        self.ply += 1;
        let me = self.side.idx();
        self.log.clear();
        match *m {
            Move::Refill => {
                self.score[me] -= 5;
                for c in 1..=5 {
                    self.s[cell_of(self.side, c)] = 1;
                }
                self.log = format!("J{} pune 5 pietre din câștiguri", me + 1);
            }
            Move::Sow(c, cw) => {
                let i = cell_of(self.side, c as usize);
                let hand = self.s[i] as u32;
                self.s[i] = 0;
                let before = self.score[me];
                self.step(i, cw, hand);
                if self.score[me] > before {
                    self.log = format!("J{} capturează {}", me + 1, self.score[me] - before);
                }
            }
        }
        if !self.mandarin[0] && !self.mandarin[1] && !self.quan_filled(0) && !self.quan_filled(6) {
            self.finish();
            return;
        }
        if !self.mandarin[0] && !self.mandarin[1] {
            self.finish();
            return;
        }
        self.side = self.side.other();
        if (self.row_empty(self.side) && self.score[self.side.idx()] < 5) || self.ply >= CAP {
            self.finish();
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.ended {
            return None;
        }
        Some(match self.score[0].cmp(&self.score[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let o = p.other();
        let side_stones = |q: Player| (1..=5).map(|c| self.s[cell_of(q, c)] as i32).sum::<i32>();
        let mut v = (self.score[p.idx()] as i32 - self.score[o.idx()] as i32) * 10;
        v += side_stones(p) * 4 - side_stones(o) * 4;
        v
    }

    fn render(&self) -> String {
        let q = |i: usize| -> String {
            let k = if i == 0 { 0 } else { 1 };
            format!(
                "[{}{}]",
                if self.mandarin[k] { "M+" } else { "" },
                self.s[i]
            )
        };
        let mut s = String::new();
        s.push_str("        J2:  1   2   3   4   5\n");
        s.push_str(&format!("  {}", q(0)));
        for c in 1..=5 {
            s.push_str(&format!("  {:>2} ", self.s[c]));
        }
        s.push_str(&format!(" {}\n", q(6)));
        s.push_str("        ");
        for c in 1..=5 {
            s.push_str(&format!(" {:>2}  ", self.s[cell_of(Player::One, c)]));
        }
        s.push_str("\n        J1:  1   2   3   4   5     (M = mandarin 10)\n");
        s.push_str(&format!(
            "Puncte: J1 {}, J2 {}\n",
            self.score[0], self.score[1]
        ));
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        if !self.ended {
            s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Sow(c, true) => format!("{c}+"),
            Move::Sow(c, false) => format!("{c}-"),
            Move::Refill => "pune5".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    #[test]
    fn start_ten_moves() {
        let g = OAnQuan::new();
        assert_eq!(g.legal_moves().len(), 10);
        assert_eq!(g.s.iter().map(|&x| x as u32).sum::<u32>(), 50);
    }

    #[test]
    fn capture_after_empty_square() {
        let mut g = OAnQuan::new();
        g.s = [0; 12];
        g.mandarin = [false, false];
        g.s[11] = 1; // J1 col 1: semăn spre 10 (sens -)... 10 devine 1, urmează 9 goală, 8 plină
        g.s[8] = 4;
        g.s[1] = 3; // ca să nu se termine jocul
        g.apply(&Move::Sow(1, false));
        // 10 primește 1 (nu era goală după semănat), urmează 9 goală => se continuă? 10 e umplută acum; next=9 goală => capture 8
        // 4 capturate + piatra rămasă pe rândul lui (jocul se încheie: ambii mandarini lipsesc)
        assert_eq!(g.s[8], 0);
        assert_eq!(g.score[0], 5);
    }

    #[test]
    fn mandarin_capture_scores_ten_plus_stones() {
        let mut g = OAnQuan::new();
        g.s = [0; 12];
        g.s[0] = 2; // quan stânga cu 2 pietre + mandarin
        g.s[11] = 0;
        g.s[10] = 1; // J1 col 2: semăn 10→11 apoi 0 e plin (quan) => se oprește (nu capturează)
        g.s[3] = 3;
        g.apply(&Move::Sow(2, true));
        assert_eq!(g.score[0], 0);
        // acum captură: J1 col 1 gol? ring 11 are 1 acum; semănăm 11 → 0? ia sens +: 11→0 (quan) sosește 1, apoi 1 (J2 rând) gol? ...
        let mut h = OAnQuan::new();
        h.s = [0; 12];
        h.s[0] = 2;
        h.s[9] = 1; // col 3: 9 →10 (sens +? +1 e 10), 10 gol, 11 gol...
        h.s[3] = 1;
        h.s[10] = 0;
        h.s[11] = 0;
        // sens + de la 9: aterizează pe 10; următoarea 11 goală => capturăm 0? nu (11 goală, 0 e după 11)
        h.apply(&Move::Sow(3, true));
        assert_eq!(h.score[0], 12); // quan 0 (10 + 2) capturat peste groapa goală 11
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=6u64 {
            let mut g = OAnQuan::new();
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() && n < 2000 {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed}");
        }
    }
}
