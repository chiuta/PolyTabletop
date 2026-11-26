#![allow(clippy::needless_range_loop, clippy::manual_range_contains)]
//! „Zaruri cosmice” — joc de zaruri pe stil „Cosmic Wimpout” (nume comercial), 2 jucători.
//!
//! Reguli proprii, simplificate din ideea publică a jocului „arunci și te oprești la timp”:
//!
//! * Cinci zaruri cu fețele 2, 3, 4, 5, 10 și **Soarele** (☼ = joker). Arunci (`arunc`), apoi
//!   păstrezi o parte din zaruri care punctează integral (`tin 10 5`).
//! * Puncte: un 10 = 100, un 5 = 50; trei zaruri la fel = 100 × fața (trei de 10 = 1000);
//!   Soarele ține loc de orice față într-un triplet (trei Sori = trei de 10 = 1000) și singur nu valorează nimic.
//! * Dacă nicio parte a aruncării nu punctează, ești „în vânt” (**wimpout**): pierzi punctele turei.
//! * După ce păstrezi, ori `continui` (arunci zarurile rămase) ori `banca` (punctele turei intră
//!   în total). Dacă păstrezi toate cele cinci zaruri, le arunci iar pe toate („flash”).
//! * Primul la 2000 de puncte câștigă.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const TARGET: i32 = 2000;
/// Fețe: 2,3,4,5,10 și 0 = Soare.
const FACES: [u8; 6] = [2, 3, 4, 5, 10, 0];

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Roll,
    Keep(Vec<u8>),
    Continue,
    Bank,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Start,
    Pick,
    Decide,
    RollPending,
}

#[derive(Clone)]
pub struct Cosmic {
    rng: Rng,
    total: [i32; 2],
    turn: i32,
    free: u8,
    rolled: Vec<u8>,
    side: Player,
    phase: Phase,
    log: String,
}

impl Default for Cosmic {
    fn default() -> Self {
        Self::new()
    }
}

fn face_idx(f: u8) -> usize {
    FACES.iter().position(|&x| x == f).unwrap()
}

fn triple_val(f: u8) -> i32 {
    if f == 0 {
        500
    } else {
        f as i32 * 100
    }
}

/// Cel mai bun scor în care **toate** zarurile din `d` contribuie (None dacă nu se poate).
pub fn score_all(d: &[u8]) -> Option<i32> {
    let mut c = [0u8; 6];
    for &x in d {
        c[face_idx(x)] += 1;
    }
    best(c)
}

fn best(c: [u8; 6]) -> Option<i32> {
    if c.iter().all(|&x| x == 0) {
        return Some(0);
    }
    let mut res: Option<i32> = None;
    let mut upd = |v: Option<i32>, add: i32| {
        if let Some(v) = v {
            res = Some(res.map_or(v + add, |r| r.max(v + add)));
        }
    };
    // singuri
    if c[4] > 0 {
        let mut n = c;
        n[4] -= 1;
        upd(best(n), 100);
    }
    if c[3] > 0 {
        let mut n = c;
        n[3] -= 1;
        upd(best(n), 50);
    }
    // triplete (cu 0..=3 sori)
    for f in 0..6 {
        let fv = FACES[f];
        for suns in 0..=3u8 {
            if f == 5 && suns > 0 {
                continue;
            }
            let need = 3 - suns;
            let need_f = if f == 5 { 3 } else { need };
            if c[f] >= need_f && c[5] >= suns {
                let mut n = c;
                n[f] -= need_f;
                n[5] -= suns;
                upd(best(n), triple_val(fv));
            }
        }
    }
    res
}

impl Cosmic {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Cosmic {
            rng: Rng::new(seed),
            total: [0, 0],
            turn: 0,
            free: 5,
            rolled: Vec::new(),
            side: Player::One,
            phase: Phase::Start,
            log: String::new(),
        }
    }

    fn end_turn(&mut self) {
        self.turn = 0;
        self.free = 5;
        self.rolled.clear();
        self.phase = Phase::Start;
        self.side = self.side.other();
    }

    fn keeps(&self) -> Vec<Vec<u8>> {
        let mut r = self.rolled.clone();
        r.sort_unstable();
        let n = r.len();
        let mut out: Vec<Vec<u8>> = Vec::new();
        for mask in 1u32..(1 << n) {
            let sub: Vec<u8> = (0..n)
                .filter(|i| mask & (1 << i) != 0)
                .map(|i| r[i])
                .collect();
            if score_all(&sub).is_some() && !out.contains(&sub) {
                out.push(sub);
            }
        }
        out
    }
}

fn face_text(f: u8) -> String {
    if f == 0 {
        "S".into()
    } else {
        f.to_string()
    }
}

impl Game for Cosmic {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        match self.phase {
            Phase::Start | Phase::RollPending => vec![Move::Roll],
            Phase::Pick => self.keeps().into_iter().map(Move::Keep).collect(),
            Phase::Decide => vec![Move::Continue, Move::Bank],
        }
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        match m {
            Move::Roll => {
                self.rolled = (0..self.free).map(|_| FACES[self.rng.below(6)]).collect();
                self.rolled.sort_unstable();
                if self.keeps().is_empty() {
                    self.log = format!(
                        "J{} aruncă {} — în vânt! pierde {}",
                        self.side.idx() + 1,
                        self.rolled
                            .iter()
                            .map(|&f| face_text(f))
                            .collect::<Vec<_>>()
                            .join(" "),
                        self.turn
                    );
                    self.end_turn();
                } else {
                    self.phase = Phase::Pick;
                }
            }
            Move::Keep(k) => {
                self.turn += score_all(k).unwrap();
                self.free -= k.len() as u8;
                if self.free == 0 {
                    self.free = 5;
                    self.log = "flash: arunci din nou toate zarurile".into();
                }
                self.rolled.clear();
                self.phase = Phase::Decide;
            }
            Move::Continue => self.phase = Phase::RollPending,
            Move::Bank => {
                self.total[self.side.idx()] += self.turn;
                self.log = format!("J{} bancă {}", self.side.idx() + 1, self.turn);
                self.end_turn();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.total[0] >= TARGET {
            Some(Outcome::Win(Player::One))
        } else if self.total[1] >= TARGET {
            Some(Outcome::Win(Player::Two))
        } else {
            None
        }
    }

    fn evaluate(&self, p: Player) -> i32 {
        let (me, op) = (p.idx(), 1 - p.idx());
        let base = self.total[me] - self.total[op];
        let mover_sign = if self.side == p { 1 } else { -1 };
        let v = match self.phase {
            Phase::Start => return base * 100 / 100,
            Phase::Pick | Phase::RollPending => self.turn + (self.free as i32) * 8,
            Phase::Decide => {
                let w = [0, 67, 44, 28, 15, 7][(self.free as usize).min(5)];
                let cont = (100 - w) * (self.turn + 25 * self.free as i32 + 40) / 100;
                cont.max(self.turn)
            }
        };
        let own = self.total[self.side.idx()];
        let _ = own;
        base + mover_sign * v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Zaruri cosmice — total: J1 {}, J2 {} (țintă {})\n",
            self.total[0], self.total[1], TARGET
        );
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        if self.outcome().is_some() {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        s.push_str(&format!(
            "Tura lui J{}: puncte în tură {} | zaruri libere {}\n",
            self.side.idx() + 1,
            self.turn,
            self.free
        ));
        if self.phase == Phase::Pick {
            s.push_str(&format!(
                "Aruncat: {}  (S = Soare)\n",
                self.rolled
                    .iter()
                    .map(|&f| face_text(f))
                    .collect::<Vec<_>>()
                    .join(" ")
            ));
        }
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Roll => "arunc".into(),
            Move::Keep(k) => format!(
                "tin {}",
                k.iter()
                    .map(|&f| face_text(f))
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            Move::Continue => "continui".into(),
            Move::Bank => "banca".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scoring() {
        assert_eq!(score_all(&[10]), Some(100));
        assert_eq!(score_all(&[5, 10]), Some(150));
        assert_eq!(score_all(&[4]), None);
        assert_eq!(score_all(&[3, 3, 3]), Some(300));
        assert_eq!(score_all(&[3, 3, 0]), Some(300));
        assert_eq!(score_all(&[10, 10, 10]), Some(1000));
        assert_eq!(score_all(&[0]), None);
        assert_eq!(score_all(&[0, 0, 0]), Some(1000)); // trei Sori = trei de 10
        assert_eq!(score_all(&[2, 2, 2, 10, 5]), Some(350));
    }

    #[test]
    fn wimpout_loses_turn_points() {
        let mut g = Cosmic::with_seed(1);
        g.turn = 300;
        g.free = 1;
        g.phase = Phase::RollPending;
        let mut found = false;
        for s in 0..40u64 {
            let mut h = g.clone();
            h.rng = Rng::new(s);
            h.apply(&Move::Roll);
            if h.phase == Phase::Start {
                assert_eq!(h.total, [0, 0]);
                assert_eq!(h.side, Player::Two);
                found = true;
            }
        }
        assert!(found);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=8u64 {
            let mut g = Cosmic::with_seed(seed);
            let mut rng = Rng::new(seed + 5);
            let mut n = 0;
            while g.outcome().is_none() && n < 4000 {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed}");
        }
    }
}
