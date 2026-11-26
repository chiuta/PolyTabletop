#![allow(
    clippy::needless_range_loop,
    clippy::manual_is_multiple_of,
    clippy::manual_range_contains,
    clippy::too_many_arguments,
    clippy::doc_lazy_continuation,
    clippy::unnecessary_unwrap,
    clippy::unnecessary_cast,
    clippy::type_complexity
)]
//! Sic Bo (China / Macao) — duel de pariuri pe trei zaruri, 2 jucători.
//!
//! Reguli folosite (tabla standard de pariuri, plăți „X la 1”):
//! * Trei zaruri. Pariuri: `mic` (suma 4–10, pierde la triplu) 1:1; `mare` (11–17, pierde la
//!   triplu) 1:1; `cifra N` — pe câte zaruri apare N: 1:1, 2:1 sau 3:1; `dublu N` (cel puțin două
//!   zaruri cu N) 10:1; `triplu N` (toate trei) 180:1; `oricare-triplu` 30:1; `suma S` (4 și 17:
//!   60:1; 5, 16: 30:1; 6, 15: 17:1; 7, 14: 12:1; 8, 13: 8:1; 9–12: 6:1); `pereche a-b` (două
//!   cifre diferite apar printre zaruri) 5:1.
//! * Varianta de duel (adaptare, nu regulă tradițională): fără bancă-jucător, ambii pariază
//!   contra „casei” cu 100 de jetoane, 12 runde; câștigă cine are mai multe jetoane la final.
//!   Ordinea de pariere alternează pe runde; al doilea vede pariurile primului. Cel mult 6
//!   pariuri pe rundă și jucător, mize 1 / 5 / 10 / 25.
//! * Mutări: `mic:5`, `mare:5`, `cifra:3:5`, `dublu:3:5`, `triplu:3:5`, `oricare-triplu:5`,
//!   `suma:9:5`, `pereche:1-2:5`, `fin` (termini pariurile), `arunc`, `urmatoarea`.
//! * Zarurile sunt deterministe din sămânță, dar se aruncă doar la mutarea forțată `arunc`;
//!   AI-ul nu le vede dinainte. AI: probabilitate aproximativă de a termina în față, din
//!   valoarea așteptată și varianța exactă a pariurilor puse.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const ROUNDS: u8 = 12;
pub const START: i32 = 100;
const MAX_BETS: usize = 6;
const AMOUNTS: [i32; 4] = [1, 5, 10, 25];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Bet {
    Small,
    Big,
    Single(u8),
    Double(u8),
    Triple(u8),
    AnyTriple,
    Total(u8),
    Pair(u8, u8),
}

/// Câștigul brut la 1 de miză (miza inclusă) sau 0 dacă pierde.
pub fn payout(b: Bet, d: [u8; 3]) -> i32 {
    let sum: u8 = d.iter().sum();
    let triple = d[0] == d[1] && d[1] == d[2];
    let cnt = |n: u8| d.iter().filter(|&&x| x == n).count() as i32;
    match b {
        Bet::Small => i32::from(!triple && (4..=10).contains(&sum)) * 2,
        Bet::Big => i32::from(!triple && (11..=17).contains(&sum)) * 2,
        Bet::Single(n) => {
            let k = cnt(n);
            if k > 0 {
                1 + k
            } else {
                0
            }
        }
        Bet::Double(n) => {
            if cnt(n) >= 2 {
                11
            } else {
                0
            }
        }
        Bet::Triple(n) => {
            if cnt(n) == 3 {
                181
            } else {
                0
            }
        }
        Bet::AnyTriple => {
            if triple {
                31
            } else {
                0
            }
        }
        Bet::Total(s) => {
            if sum == s {
                1 + total_odds(s)
            } else {
                0
            }
        }
        Bet::Pair(a, c) => {
            if cnt(a) > 0 && cnt(c) > 0 {
                6
            } else {
                0
            }
        }
    }
}

pub fn total_odds(s: u8) -> i32 {
    match s {
        4 | 17 => 60,
        5 | 16 => 30,
        6 | 15 => 17,
        7 | 14 => 12,
        8 | 13 => 8,
        _ => 6,
    }
}

/// (valoare așteptată, varianță) pentru miza 1, exact peste cele 216 rezultate.
pub fn ev_var(b: Bet) -> (f64, f64) {
    let mut e = 0.0;
    let mut e2 = 0.0;
    for a in 1..=6u8 {
        for c in 1..=6u8 {
            for x in 1..=6u8 {
                let g = payout(b, [a, c, x]) as f64 - 1.0;
                e += g;
                e2 += g * g;
            }
        }
    }
    e /= 216.0;
    e2 /= 216.0;
    (e, e2 - e * e)
}

fn bet_text(b: Bet) -> String {
    match b {
        Bet::Small => "mic".into(),
        Bet::Big => "mare".into(),
        Bet::Single(n) => format!("cifra:{n}"),
        Bet::Double(n) => format!("dublu:{n}"),
        Bet::Triple(n) => format!("triplu:{n}"),
        Bet::AnyTriple => "oricare-triplu".into(),
        Bet::Total(s) => format!("suma:{s}"),
        Bet::Pair(a, c) => format!("pereche:{a}-{c}"),
    }
}

fn all_bets() -> Vec<Bet> {
    let mut v = vec![Bet::Small, Bet::Big, Bet::AnyTriple];
    for n in 1..=6 {
        v.push(Bet::Single(n));
        v.push(Bet::Double(n));
        v.push(Bet::Triple(n));
    }
    for s in 4..=17 {
        v.push(Bet::Total(s));
    }
    for a in 1..=6 {
        for c in (a + 1)..=6 {
            v.push(Bet::Pair(a, c));
        }
    }
    v
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Place(Bet, i32),
    Done,
    Roll,
    Next,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Betting,
    Roll,
    Resolved,
}

#[derive(Clone)]
pub struct SicBo {
    chips: [i32; 2],
    bets: [Vec<(Bet, i32)>; 2],
    round: u8,
    /// Cine pariază primul în runda curentă.
    first: Player,
    /// A terminat primul pariurile?
    first_done: bool,
    side: Player,
    phase: Phase,
    dice: [u8; 3],
    last: String,
    rng: Rng,
}

impl Default for SicBo {
    fn default() -> Self {
        Self::new()
    }
}

impl SicBo {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        SicBo {
            chips: [START; 2],
            bets: [Vec::new(), Vec::new()],
            round: 1,
            first: Player::One,
            first_done: false,
            side: Player::One,
            phase: Phase::Betting,
            dice: [0; 3],
            last: String::new(),
            rng: Rng::new(seed),
        }
    }
    pub fn chips(&self) -> (i32, i32) {
        (self.chips[0], self.chips[1])
    }
    pub fn set_dice_for_test(&mut self, d: [u8; 3]) {
        self.dice = d;
    }
    fn over(&self) -> bool {
        self.round > ROUNDS
    }
    /// Valoarea așteptată a jetoanelor lui `p` după rezolvare și varianța pariurilor puse.
    fn exp_var(&self, p: usize) -> (f64, f64) {
        let mut e = self.chips[p] as f64;
        let mut v = 0.0;
        for &(b, a) in &self.bets[p] {
            let (be, bv) = ev_var(b);
            e += a as f64 * (1.0 + be);
            v += (a as f64).powi(2) * bv;
        }
        (e, v)
    }
}

impl Game for SicBo {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over() {
            return Vec::new();
        }
        match self.phase {
            Phase::Roll => vec![Move::Roll],
            Phase::Resolved => vec![Move::Next],
            Phase::Betting => {
                let me = self.side.idx();
                let mut v = vec![Move::Done];
                if self.bets[me].len() < MAX_BETS {
                    for b in all_bets() {
                        for a in AMOUNTS {
                            if a <= self.chips[me] {
                                v.push(Move::Place(b, a));
                            }
                        }
                    }
                }
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        match *m {
            Move::Place(b, a) => {
                let me = self.side.idx();
                self.chips[me] -= a;
                self.bets[me].push((b, a));
            }
            Move::Done => {
                if !self.first_done {
                    self.first_done = true;
                    self.side = self.first.other();
                } else {
                    self.phase = Phase::Roll;
                    self.side = self.first;
                }
            }
            Move::Roll => {
                self.dice = [self.rng.d6(), self.rng.d6(), self.rng.d6()];
                let mut parts = Vec::new();
                for p in 0..2 {
                    let mut won = 0;
                    let mut staked = 0;
                    for &(b, a) in &self.bets[p] {
                        won += a * payout(b, self.dice);
                        staked += a;
                    }
                    self.chips[p] += won;
                    parts.push(format!("J{}: {:+}", p + 1, won - staked));
                }
                let sum: u8 = self.dice.iter().sum();
                self.last = format!(
                    "Runda {}: zaruri {}-{}-{} (suma {}) → {}",
                    self.round,
                    self.dice[0],
                    self.dice[1],
                    self.dice[2],
                    sum,
                    parts.join(", ")
                );
                self.phase = Phase::Resolved;
            }
            Move::Next => {
                self.round += 1;
                self.bets = [Vec::new(), Vec::new()];
                self.first = self.first.other();
                self.first_done = false;
                self.side = self.first;
                self.phase = Phase::Betting;
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over() {
            return None;
        }
        Some(match self.chips[0].cmp(&self.chips[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let (me, op) = (p.idx(), p.other().idx());
        if self.phase != Phase::Betting {
            return (self.chips[me] - self.chips[op]) * 100;
        }
        let (em, vm) = self.exp_var(me);
        let (eo, vo) = self.exp_var(op);
        let left = (ROUNDS - self.round.min(ROUNDS)) as f64;
        let var = vm + vo + 150.0 * (left + 1.0);
        let z = (em - eo) / var.sqrt();
        let prob = 0.5 + 0.5 * (1.2 * z).tanh();
        (prob * 100_000.0) as i32
            + crate::p2_util::noise(
                crate::p2_util::sig_of(&[self.chips[0] as u64, self.chips[1] as u64, vm as u64]),
                800,
            )
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Sic Bo — runda {}/{} | jetoane: J1 {}, J2 {}\n",
            self.round.min(ROUNDS),
            ROUNDS,
            self.chips[0],
            self.chips[1]
        );
        if !self.last.is_empty() {
            s.push_str(&format!("{}\n", self.last));
        }
        if self.over() {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        for p in 0..2 {
            let t: Vec<String> = self.bets[p]
                .iter()
                .map(|&(b, a)| format!("{}×{}", bet_text(b), a))
                .collect();
            s.push_str(&format!(
                "Pariurile J{}: {}\n",
                p + 1,
                if t.is_empty() {
                    "—".into()
                } else {
                    t.join(", ")
                }
            ));
        }
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Place(b, a) => format!("{}:{}", bet_text(*b), a),
            Move::Done => "fin".into(),
            Move::Roll => "arunc".into(),
            Move::Next => "urmatoarea".into(),
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
    fn payouts_table() {
        assert_eq!(payout(Bet::Small, [1, 2, 3]), 2); // 6
        assert_eq!(payout(Bet::Small, [2, 2, 2]), 0); // triplu
        assert_eq!(payout(Bet::Big, [6, 5, 1]), 2);
        assert_eq!(payout(Bet::Big, [6, 6, 6]), 0);
        assert_eq!(payout(Bet::Single(4), [4, 4, 1]), 3); // 2:1 + miza
        assert_eq!(payout(Bet::Single(4), [4, 4, 4]), 4);
        assert_eq!(payout(Bet::Single(4), [1, 2, 3]), 0);
        assert_eq!(payout(Bet::Double(3), [3, 3, 5]), 11);
        assert_eq!(payout(Bet::Double(3), [3, 3, 3]), 11);
        assert_eq!(payout(Bet::Triple(5), [5, 5, 5]), 181);
        assert_eq!(payout(Bet::AnyTriple, [2, 2, 2]), 31);
        assert_eq!(payout(Bet::Total(4), [1, 1, 2]), 61);
        assert_eq!(payout(Bet::Total(10), [3, 3, 4]), 7);
        assert_eq!(payout(Bet::Pair(1, 2), [1, 2, 6]), 6);
        assert_eq!(payout(Bet::Pair(1, 2), [1, 1, 6]), 0);
    }

    #[test]
    fn house_edges_are_known() {
        // mic/mare: 2.78%; oricare-triplu 13.89%; cifra 7.87%; dublu 18.52%
        let (e, _) = ev_var(Bet::Small);
        assert!((e + 0.0278).abs() < 0.001, "{e}");
        let (e, _) = ev_var(Bet::AnyTriple);
        assert!((e + 0.1389).abs() < 0.001, "{e}");
        let (e, _) = ev_var(Bet::Single(3));
        assert!((e + 0.0787).abs() < 0.001, "{e}");
        let (e, _) = ev_var(Bet::Double(3));
        assert!((e + 0.1852).abs() < 0.002, "{e}");
    }

    #[test]
    fn chips_conserved_in_staking_and_resolution() {
        let mut g = SicBo::with_seed(1);
        g.apply(&Move::Place(Bet::Small, 10));
        assert_eq!(g.chips().0, 90);
        g.apply(&Move::Done);
        assert_eq!(g.side, Player::Two);
        g.apply(&Move::Done);
        assert_eq!(g.legal_moves(), vec![Move::Roll]);
        g.apply(&Move::Roll);
        let d = g.dice;
        let want = 90 + 10 * payout(Bet::Small, d);
        assert_eq!(g.chips().0, want);
        assert_eq!(g.chips().1, 100);
        g.apply(&Move::Next);
        assert_eq!(g.round, 2);
        assert_eq!(g.side, Player::Two); // ordinea alternează
    }

    #[test]
    fn cannot_bet_more_than_chips_and_max_bets() {
        let mut g = SicBo::with_seed(2);
        g.chips[0] = 3;
        let mv = g.legal_moves();
        assert!(mv.iter().all(|m| match m {
            Move::Place(_, a) => *a <= 3,
            _ => true,
        }));
        let mut g = SicBo::with_seed(2);
        for _ in 0..MAX_BETS {
            g.apply(&Move::Place(Bet::Big, 1));
        }
        assert_eq!(g.legal_moves(), vec![Move::Done]);
    }

    #[test]
    fn deterministic_and_finishes() {
        let mut a = SicBo::with_seed(7);
        let mut b = SicBo::with_seed(7);
        let mut rng = Rng::new(1);
        for _ in 0..2000 {
            let ma = Game::legal_moves(&a);
            if ma.is_empty() {
                break;
            }
            let m = ma[rng.below(ma.len())];
            a.apply(&m);
            b.apply(&m);
        }
        assert!(a.outcome().is_some());
        assert_eq!(a.chips(), b.chips());
    }

    #[test]
    fn ai_plays_to_the_end() {
        for seed in 1..4u64 {
            let mut g = SicBo::with_seed(seed);
            let mut n = 0;
            while g.outcome().is_none() && n < 3000 {
                let m = polytabletop_core::ai::best_move(&g, seed + n).unwrap();
                g.apply(&m);
                n += 1;
            }
            assert!(g.outcome().is_some());
        }
    }

    #[test]
    fn trailing_ai_takes_more_risk_than_leading() {
        let mut lead = SicBo::with_seed(3);
        lead.chips = [150, 60];
        lead.round = 11;
        let mut trail = lead.clone();
        trail.chips = [60, 150];
        let risk = |g: &SicBo| {
            let mut g = g.clone();
            let mut v = 0.0;
            for _ in 0..3 {
                let m = polytabletop_core::ai::best_move(&g, 1).unwrap();
                if let Move::Place(b, a) = m {
                    v += a as f64 * ev_var(b).1.sqrt();
                    g.apply(&m);
                } else {
                    break;
                }
            }
            v
        };
        assert!(risk(&trail) > risk(&lead));
    }
}
