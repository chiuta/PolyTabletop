#![allow(clippy::doc_lazy_continuation)]
//! Schocken (Germania), joc de zaruri cu trei zaruri și 13 jetoane, pentru doi jucători.
//!
//! * Clasamentul: Schock Out (1-1-1) > Schock (1-1-x, mai mare x = mai bun) > General (trei la fel)
//!   > Straße (trei consecutive) > aruncare obișnuită (se compară cel mai mare zar, apoi următorul).
//! * Într-o rundă, fiecare aruncă cel mult de trei ori (la a doua și a treia aruncare re-arunci
//!   zarurile alese). Primul aruncător hotărăște câte aruncări au voie (cât a folosit el); cel de-al
//!   doilea nu poate depăși acest număr. Zarurile sunt vizibile (fără aruncări „oarbe”).
//! * Cine are aruncarea mai slabă pierde runda (la egalitate pierde cel care a aruncat ultimul) și
//!   ia jetoane după aruncarea câștigătoare: Schock x = x jetoane, General 3, Straße 2, obișnuită 1,
//!   Schock Out = toate cele 13 (cât există). Jetoanele se iau din stiva de mijloc, iar când stiva e
//!   goală, de la câștigător. Cine a pierdut runda aruncă primul în următoarea.
//! * O repriză (Hälfte) se termină când un jucător ține toate cele 13 jetoane: acela o pierde.
//!   Cine pierde două reprize pierde meciul (varianta pe două reprize, fără repriza de finală
//!   „Finale”; pentru doi jucători).
//! Mutări: `arunc`, `stop`, `re 1`, `re 13` (re-arunci zarurile din pozițiile indicate), `decontare`.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const CHIPS: i32 = 13;

pub fn rank(h: [u8; 3]) -> u32 {
    let mut d = h;
    d.sort_unstable_by(|a, b| b.cmp(a));
    if d == [1, 1, 1] {
        return 10_000;
    }
    if d[1] == 1 && d[2] == 1 {
        return 9_000 + d[0] as u32 * 10;
    }
    if d[0] == d[1] && d[1] == d[2] {
        return 8_000 + d[0] as u32 * 10;
    }
    if d[0] == d[1] + 1 && d[1] == d[2] + 1 {
        return 7_000 + d[2] as u32 * 10;
    }
    d[0] as u32 * 100 + d[1] as u32 * 10 + d[2] as u32
}

fn chips_of(h: [u8; 3]) -> i32 {
    let r = rank(h);
    match r {
        10_000 => CHIPS,
        9_000..=9_999 => (r - 9_000) as i32 / 10,
        8_000..=8_999 => 3,
        7_000..=7_999 => 2,
        _ => 1,
    }
}

fn name(h: [u8; 3]) -> String {
    let mut d = h;
    d.sort_unstable_by(|a, b| b.cmp(a));
    let t = format!("{}-{}-{}", d[0], d[1], d[2]);
    match rank(h) {
        10_000 => format!("{t} (Schock Out)"),
        9_000..=9_999 => format!("{t} (Schock)"),
        8_000..=8_999 => format!("{t} (General)"),
        7_000..=7_999 => format!("{t} (Straße)"),
        _ => t,
    }
}

/// Probabilitatea ca o aruncare uniformă să fie strict sub `h` (egal = jumătate).
fn rating(h: [u8; 3]) -> f64 {
    let r = rank(h);
    let mut s = 0.0;
    for a in 1..=6u8 {
        for b in 1..=6u8 {
            for c in 1..=6u8 {
                let o = rank([a, b, c]);
                if o < r {
                    s += 1.0;
                } else if o == r {
                    s += 0.5;
                }
            }
        }
    }
    s / 216.0
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Throw,
    Decide,
    Pending(u8),
    Settle,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Roll,
    Stop,
    Reroll(u8),
    Settle,
}

#[derive(Clone)]
pub struct Schocken {
    rng: Rng,
    stack: i32,
    hold: [i32; 2],
    lost: [u8; 2],
    starter: Player,
    roller: Player,
    cur: [u8; 3],
    throws: u8,
    max_throws: u8,
    first: Option<[u8; 3]>,
    phase: Phase,
    moves: u32,
    log: String,
}

impl Default for Schocken {
    fn default() -> Self {
        Self::new()
    }
}

impl Schocken {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Schocken {
            rng: Rng::new(seed),
            stack: CHIPS,
            hold: [0; 2],
            lost: [0; 2],
            starter: Player::One,
            roller: Player::One,
            cur: [0; 3],
            throws: 0,
            max_throws: 3,
            first: None,
            phase: Phase::Throw,
            moves: 0,
            log: String::new(),
        }
    }

    fn over(&self) -> bool {
        self.lost.iter().any(|&l| l >= 2) || self.moves > 6000
    }

    fn hand_value(&self, p: Player, h: [u8; 3]) -> f64 {
        match self.first {
            Some(f) if p != self.starter => {
                if rank(h) > rank(f) {
                    1.0
                } else {
                    0.0
                }
            }
            _ => rating(h),
        }
    }

    fn mask_positions(mask: u8) -> String {
        (0..3)
            .filter(|i| mask & (1 << i) != 0)
            .map(|i| (i + 1).to_string())
            .collect()
    }
}

impl Game for Schocken {
    type Move = Move;

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Throw | Phase::Decide => self.roller,
            _ => Player::Two,
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over() {
            return Vec::new();
        }
        match self.phase {
            Phase::Throw | Phase::Pending(_) => vec![Move::Roll],
            Phase::Settle => vec![Move::Settle],
            Phase::Decide => {
                let mut v = vec![Move::Stop];
                if self.throws < self.max_throws {
                    v.extend((1..8u8).map(Move::Reroll));
                }
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        self.moves += 1;
        match *m {
            Move::Roll => {
                match self.phase {
                    Phase::Throw => {
                        for d in self.cur.iter_mut() {
                            *d = self.rng.d6();
                        }
                    }
                    Phase::Pending(mask) => {
                        for i in 0..3 {
                            if mask & (1 << i) != 0 {
                                self.cur[i] = self.rng.d6();
                            }
                        }
                    }
                    _ => {}
                }
                self.cur.sort_unstable_by(|a, b| b.cmp(a));
                self.throws += 1;
                self.phase = Phase::Decide;
            }
            Move::Reroll(mask) => self.phase = Phase::Pending(mask),
            Move::Stop => {
                if self.first.is_none() {
                    self.first = Some(self.cur);
                    self.max_throws = self.throws;
                    self.roller = self.roller.other();
                    self.throws = 0;
                    self.phase = Phase::Throw;
                } else {
                    self.phase = Phase::Settle;
                }
            }
            Move::Settle => {
                let f = self.first.take().unwrap();
                let s = self.cur;
                let second = self.roller;
                let first_p = second.other();
                let (winner, loser, wh) = if rank(s) > rank(f) {
                    (second, first_p, s)
                } else {
                    (first_p, second, f)
                };
                let v = chips_of(wh);
                let take_stack = v.min(self.stack);
                self.stack -= take_stack;
                self.hold[loser.idx()] += take_stack;
                let rest = (v - take_stack).min(self.hold[winner.idx()]);
                self.hold[winner.idx()] -= rest;
                self.hold[loser.idx()] += rest;
                self.log = format!(
                    "{} bate {}: J{} ia {} jetoane.",
                    name(wh),
                    name(if wh == s { f } else { s }),
                    loser.idx() + 1,
                    take_stack + rest
                );
                self.starter = loser;
                self.roller = loser;
                self.max_throws = 3;
                self.throws = 0;
                self.phase = Phase::Throw;
                for p in 0..2 {
                    if self.hold[p] == CHIPS {
                        self.lost[p] += 1;
                        self.log.push_str(&format!(" J{} pierde repriza.", p + 1));
                        self.stack = CHIPS;
                        self.hold = [0; 2];
                    }
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over() {
            return None;
        }
        Some(match self.lost[0].cmp(&self.lost[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::Two),
            std::cmp::Ordering::Less => Outcome::Win(Player::One),
            std::cmp::Ordering::Equal => match self.hold[0].cmp(&self.hold[1]) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::Two),
                std::cmp::Ordering::Less => Outcome::Win(Player::One),
                std::cmp::Ordering::Equal => Outcome::Draw,
            },
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let q = p.other();
        let chips = 10 * (self.hold[q.idx()] - self.hold[p.idx()])
            + 400 * (self.lost[q.idx()] as i32 - self.lost[p.idx()] as i32);
        let v = match self.phase {
            Phase::Pending(mask) if self.roller == p => {
                let free: Vec<usize> = (0..3).filter(|i| mask & (1 << i) != 0).collect();
                let n = 6usize.pow(free.len() as u32);
                let mut tot = 0.0;
                for k in 0..n {
                    let mut h = self.cur;
                    let mut kk = k;
                    for &i in &free {
                        h[i] = (kk % 6) as u8 + 1;
                        kk /= 6;
                    }
                    tot += self.hand_value(p, h);
                }
                tot / n as f64
            }
            Phase::Decide if self.roller == p => self.hand_value(p, self.cur),
            Phase::Settle => {
                let w = self.hand_value(self.roller, self.cur);
                if self.roller == p {
                    w
                } else {
                    1.0 - w
                }
            }
            Phase::Throw if self.roller != p && self.first.is_some() => rating(self.first.unwrap()),
            _ => 0.5,
        };
        chips + ((v - 0.5) * 100.0) as i32
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Schocken — reprize pierdute: J1 {}, J2 {}. Stivă {}; J1 ține {}, J2 ține {}.\n",
            self.lost[0], self.lost[1], self.stack, self.hold[0], self.hold[1]
        );
        s.push_str(&format!(
            "Runda: aruncă J{} (aruncări {}/{}).\n",
            self.roller.idx() + 1,
            self.throws,
            self.max_throws
        ));
        if let Some(f) = self.first {
            s.push_str(&format!("Primul jucător a făcut: {}.\n", name(f)));
        }
        if self.throws > 0 {
            s.push_str(&format!(
                "Zaruri (poz. 1–3): {} {} {} — {}\n",
                self.cur[0],
                self.cur[1],
                self.cur[2],
                name(self.cur)
            ));
        }
        match self.phase {
            Phase::Decide => s.push_str("`stop` sau `re 13` (re-arunci zarurile 1 și 3).\n"),
            Phase::Settle => s.push_str("Scrie «decontare».\n"),
            _ => {}
        }
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Roll => "arunc".into(),
            Move::Stop => "stop".into(),
            Move::Reroll(mask) => format!("re {}", Self::mask_positions(*mask)),
            Move::Settle => "decontare".into(),
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
    fn ranking_order() {
        assert!(rank([1, 1, 1]) > rank([1, 1, 6]));
        assert!(rank([1, 1, 2]) > rank([6, 6, 6]));
        assert!(rank([2, 2, 2]) > rank([4, 5, 6]));
        assert!(rank([4, 5, 6]) > rank([6, 5, 4]) - 1);
        assert!(rank([2, 3, 4]) > rank([6, 6, 5]));
        assert_eq!(chips_of([1, 1, 4]), 4);
        assert_eq!(chips_of([3, 3, 3]), 3);
        assert_eq!(chips_of([4, 3, 2]), 2);
        assert_eq!(chips_of([6, 5, 1]), 1);
    }

    #[test]
    fn random_games_end_and_chips_are_conserved() {
        for seed in 1..=15u64 {
            let mut g = Schocken::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
                assert_eq!(g.stack + g.hold[0] + g.hold[1], CHIPS);
                n += 1;
                assert!(n < 20000);
            }
        }
    }

    #[test]
    fn ai_keeps_a_schock_and_rerolls_junk() {
        let mut g = Schocken::with_seed(1);
        g.cur = [1, 1, 5];
        g.throws = 1;
        g.phase = Phase::Decide;
        let m = polytabletop_core::GameObj::ai_move(&g, 3).unwrap();
        assert_eq!(m, "stop");
        g.cur = [1, 1, 6];
        let m = polytabletop_core::GameObj::ai_move(&g, 3).unwrap();
        assert_eq!(m, "stop");
        g.cur = [5, 3, 2];
        let m = polytabletop_core::GameObj::ai_move(&g, 3).unwrap();
        assert!(m.starts_with("re"), "{m}");
    }
}
