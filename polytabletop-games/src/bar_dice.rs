#![allow(clippy::needless_range_loop, clippy::manual_range_contains)]
//! „Zaruri de bar” (Bar dice) — joc global de zaruri de la tejghea, 2 jucători.
//!
//! * Cinci zaruri, cel mult trei aruncări pe tură. După fiecare aruncare păstrezi cel puțin un zar
//!   (`tin 6 5`), restul îl arunci din nou (`arunc`); poți păstra toate zarurile și te oprești.
//!   La a treia aruncare păstrezi tot ce a ieșit.
//! * Ca să punctezi trebuie să ai **Vasul (6), Căpitanul (5) și Echipajul (4)**; celelalte două
//!   zaruri sunt **încărcătura** și valorează suma lor (2–12). Fără cele trei, tura valorează 0.
//! * Fiecare rundă: ambii joacă o tură, încărcătura mai mare ia runda (egal = nimic). Primul la
//!   4 runde sau, după 9 runde, cine are mai multe câștigă.
//!
//! Zarurile se aruncă la mutarea forțată `arunc`: AI-ul alege ce păstrează fără să vadă viitorul.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const MAX_ROUNDS: u8 = 9;
const TARGET: u8 = 4;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Roll,
    Keep(Vec<u8>),
}

#[derive(Clone)]
pub struct BarDice {
    rng: Rng,
    held: Vec<u8>,
    rolled: Vec<u8>,
    rolls: u8,
    side: Player,
    first: Player,
    round: u8,
    wins: [u8; 2],
    result: [Option<i32>; 2],
    log: String,
}

impl Default for BarDice {
    fn default() -> Self {
        Self::new()
    }
}

/// Încărcătura (0 dacă nu ai 6, 5 și 4).
pub fn cargo(d: &[u8]) -> i32 {
    let mut v = d.to_vec();
    for need in [6u8, 5, 4] {
        match v.iter().position(|&x| x == need) {
            Some(i) => {
                v.remove(i);
            }
            None => return 0,
        }
    }
    v.iter().map(|&x| x as i32).sum()
}

fn missing(d: &[u8]) -> i32 {
    let mut v = d.to_vec();
    let mut m = 0;
    for need in [6u8, 5, 4] {
        match v.iter().position(|&x| x == need) {
            Some(i) => {
                v.remove(i);
            }
            None => m += 1,
        }
    }
    m
}

impl BarDice {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        BarDice {
            rng: Rng::new(seed),
            held: Vec::new(),
            rolled: Vec::new(),
            rolls: 0,
            side: Player::One,
            first: Player::One,
            round: 1,
            wins: [0, 0],
            result: [None, None],
            log: String::new(),
        }
    }

    fn over(&self) -> bool {
        self.wins[0] >= TARGET || self.wins[1] >= TARGET || self.round > MAX_ROUNDS
    }

    fn finish_turn(&mut self) {
        let p = self.side.idx();
        let c = cargo(&self.held);
        self.result[p] = Some(c);
        self.log = format!("J{} termină cu {:?} → încărcătură {}", p + 1, self.held, c);
        self.held.clear();
        self.rolled.clear();
        self.rolls = 0;
        if let (Some(a), Some(b)) = (self.result[0], self.result[1]) {
            let msg = if a > b {
                self.wins[0] += 1;
                "J1 ia runda"
            } else if b > a {
                self.wins[1] += 1;
                "J2 ia runda"
            } else {
                "runda egală"
            };
            self.log = format!("{} ({} vs {}): {}", self.log, a, b, msg);
            self.result = [None, None];
            self.round += 1;
            self.first = self.first.other();
            self.side = self.first;
        } else {
            self.side = self.side.other();
        }
    }

    /// Estimare (×10) a încărcăturii finale pentru zarurile păstrate.
    fn estimate(&self, held: &[u8], rolls_left: u8) -> i32 {
        let free = 5 - held.len() as i32;
        let miss = missing(held);
        if free == 0 {
            return cargo(held) * 10;
        }
        let mut v = 0i32;
        let mut h = held.to_vec();
        for need in [6u8, 5, 4] {
            if let Some(i) = h.iter().position(|&x| x == need) {
                h.remove(i);
            }
        }
        v += h.iter().map(|&x| x as i32).sum::<i32>() * 10;
        let spare = (free - miss).max(0);
        v += spare * 35;
        if rolls_left == 0 {
            return if miss > 0 { 0 } else { v };
        }
        // probabilitate grosieră de a găsi ce lipsește
        let ok = match (miss, rolls_left) {
            (0, _) => 100,
            (1, 1) => 40,
            (1, _) => 70,
            (2, 1) => 12,
            (2, _) => 35,
            (_, 1) => 3,
            _ => 15,
        };
        v * ok / 100
    }
}

impl Game for BarDice {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over() {
            return Vec::new();
        }
        if self.rolled.is_empty() {
            return vec![Move::Roll];
        }
        if self.rolls >= 3 {
            let mut r = self.rolled.clone();
            r.sort_unstable();
            return vec![Move::Keep(r)];
        }
        let mut r = self.rolled.clone();
        r.sort_unstable();
        let n = r.len();
        let mut out: Vec<Vec<u8>> = Vec::new();
        for mask in 1u32..(1 << n) {
            let sub: Vec<u8> = (0..n)
                .filter(|i| mask & (1 << i) != 0)
                .map(|i| r[i])
                .collect();
            if !out.contains(&sub) {
                out.push(sub);
            }
        }
        out.into_iter().map(Move::Keep).collect()
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        match m {
            Move::Roll => {
                let n = 5 - self.held.len();
                self.rolled = (0..n).map(|_| self.rng.d6()).collect();
                self.rolled.sort_unstable();
                self.rolls += 1;
            }
            Move::Keep(k) => {
                self.held.extend(k.iter().copied());
                self.held.sort_unstable();
                let mut left = self.rolled.clone();
                for x in k {
                    if let Some(i) = left.iter().position(|y| y == x) {
                        left.remove(i);
                    }
                }
                self.rolled.clear();
                if self.held.len() >= 5 {
                    self.finish_turn();
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over() {
            return None;
        }
        Some(match self.wins[0].cmp(&self.wins[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let sign = if self.side == p { 1 } else { -1 };
        let mut held = self.held.clone();
        let mut left = 3 - self.rolls.min(3);
        if !self.rolled.is_empty() {
            // decizie în curs (după Keep rezultatul e deja în `held`)
            held.extend(self.rolled.iter().copied());
            left = 0;
        }
        let mut v = self.estimate(&held, left);
        if self.held.is_empty() && self.rolled.is_empty() {
            v = 0;
        }
        // dacă adversarul a terminat deja, compară cu rezultatul lui
        if let Some(o) = self.result[self.side.other().idx()] {
            v -= o * 10;
        }
        sign * v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Zaruri de bar — runda {}/{} | runde câștigate: J1 {}, J2 {} (primul la {})\n",
            self.round.min(MAX_ROUNDS),
            MAX_ROUNDS,
            self.wins[0],
            self.wins[1],
            TARGET
        );
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        if !self.over() {
            s.push_str(&format!(
                "Tura lui J{}: păstrate {:?} | aruncare {}/3: {:?}\n",
                self.side.idx() + 1,
                self.held,
                self.rolls,
                self.rolled
            ));
            if let Some(o) = self.result[self.side.other().idx()] {
                s.push_str(&format!("Adversarul a făcut {o}.\n"));
            }
            s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        } else {
            s.push_str("Joc încheiat.\n");
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Roll => "arunc".into(),
            Move::Keep(k) => format!(
                "tin {}",
                k.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
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
    fn cargo_scores() {
        assert_eq!(cargo(&[6, 5, 4, 3, 2]), 5);
        assert_eq!(cargo(&[6, 6, 5, 4, 6]), 12);
        assert_eq!(cargo(&[6, 5, 3, 3, 2]), 0);
        assert_eq!(cargo(&[5, 4, 4, 1, 1]), 0);
    }

    #[test]
    fn third_roll_forces_keep_all() {
        let mut g = BarDice::with_seed(1);
        g.rolls = 3;
        g.rolled = vec![1, 2, 3];
        g.held = vec![6, 5];
        assert_eq!(g.legal_moves(), vec![Move::Keep(vec![1, 2, 3])]);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=10u64 {
            let mut g = BarDice::with_seed(seed);
            let mut rng = Rng::new(seed + 5);
            let mut n = 0;
            while g.outcome().is_none() && n < 4000 {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
            }
            assert!(g.outcome().is_some());
        }
    }
}
