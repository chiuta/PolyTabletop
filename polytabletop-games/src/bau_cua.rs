//! Bầu cua cá cọp (Vietnam), 2 jucători, din regulile publice ale jocului de zaruri.
//!
//! * Trei zaruri, fiecare cu șase simboluri: **cerb** (nai), **dovleac** (bầu), **cocoș** (gà), **pește** (cá),
//!   **crab** (cua), **creveți** (tôm). Un jucător este **bancherul**, celălalt **parierul**; rolurile alternează
//!   în fiecare rundă.
//! * Parierul pune mize (5 sau 10 jetoane) pe unul sau mai multe simboluri (maxim o miză pe simbol), apoi
//!   `gata`; bancherul aruncă zarurile (`arunc`, mutare forțată). Pentru fiecare zar cu simbolul parat, bancherul
//!   plătește miza (1:1) și parierul își păstrează miza; mizele pe simboluri care nu au ieșit se pierd către
//!   bancher.
//! * 12 runde, fiecare pornește cu 50 de jetoane; câștigă cine are mai multe la final (sau cine nu mai poate
//!   plăti). Miza totală dintr-o rundă nu poate depăși jetoanele parierului sau ale bancherului.
//!
//! Mutări: `cerb 5`, `peste 10`, `gata`, `arunc`. Simboluri: cerb, dovleac, cocos, peste, crab, creveti.
#![allow(clippy::manual_is_multiple_of)]

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const SYMS: [&str; 6] = ["cerb", "dovleac", "cocos", "peste", "crab", "creveti"];
const ROUNDS: u8 = 12;
const START: i32 = 50;

#[derive(Clone, PartialEq, Debug)]
pub enum Move {
    Bet(u8, i32),
    Done,
    Roll,
}

#[derive(Clone)]
pub struct BauCua {
    rng: Rng,
    chips: [i32; 2],
    round: u8,
    bets: [i32; 6],
    rolling: bool,
    log: String,
}

impl Default for BauCua {
    fn default() -> Self {
        Self::new()
    }
}

impl BauCua {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        BauCua {
            rng: Rng::new(seed),
            chips: [START; 2],
            round: 0,
            bets: [0; 6],
            rolling: false,
            log: String::new(),
        }
    }

    pub fn chips(&self) -> [i32; 2] {
        self.chips
    }

    fn banker(&self) -> Player {
        if self.round % 2 == 0 {
            Player::Two
        } else {
            Player::One
        }
    }

    fn bettor(&self) -> Player {
        self.banker().other()
    }

    fn over(&self) -> bool {
        self.round >= ROUNDS || self.chips[0] <= 0 || self.chips[1] <= 0
    }

    fn staked(&self) -> i32 {
        self.bets.iter().sum()
    }
}

impl Game for BauCua {
    type Move = Move;

    fn to_move(&self) -> Player {
        if self.rolling {
            self.banker()
        } else {
            self.bettor()
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over() {
            return Vec::new();
        }
        if self.rolling {
            return vec![Move::Roll];
        }
        let cap = self.chips[self.bettor().idx()].min(self.chips[self.banker().idx()] / 3);
        let mut v = Vec::new();
        for s in 0..6u8 {
            if self.bets[s as usize] == 0 {
                for stake in [5, 10] {
                    if self.staked() + stake <= cap {
                        v.push(Move::Bet(s, stake));
                    }
                }
            }
        }
        v.push(Move::Done);
        v
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Bet(s, k) => self.bets[*s as usize] = *k,
            Move::Done => {
                if self.staked() == 0 {
                    // fără mize: runda trece
                    self.round += 1;
                    self.log = "Nicio miză: runda trece.".into();
                } else {
                    self.rolling = true;
                }
            }
            Move::Roll => {
                let dice: Vec<usize> = (0..3).map(|_| self.rng.below(6)).collect();
                let (bt, bk) = (self.bettor().idx(), self.banker().idx());
                let mut net = 0;
                for s in 0..6 {
                    if self.bets[s] > 0 {
                        let hits = dice.iter().filter(|&&d| d == s).count() as i32;
                        net += if hits == 0 {
                            -self.bets[s]
                        } else {
                            hits * self.bets[s]
                        };
                    }
                }
                self.chips[bt] += net;
                self.chips[bk] -= net;
                self.log = format!(
                    "Zaruri: {} — parierul J{} {:+} jetoane.",
                    dice.iter().map(|&d| SYMS[d]).collect::<Vec<_>>().join(", "),
                    bt + 1,
                    net
                );
                self.bets = [0; 6];
                self.rolling = false;
                self.round += 1;
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
        // speranța pariului: fiecare miză are valoare așteptată negativă pentru parior (≈ −8%)
        let mut v = (self.chips[p.idx()] - self.chips[p.other().idx()]) * 100;
        let ev = -(self.staked() * 8);
        if self.bettor() == p {
            v += ev;
        } else {
            v -= ev;
        }
        v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Bầu cua cá cọp — runda {}/{ROUNDS}; jetoane J1 {}, J2 {}\nBancher: J{}, parior: J{}\n",
            (self.round + 1).min(ROUNDS),
            self.chips[0],
            self.chips[1],
            self.banker().idx() + 1,
            self.bettor().idx() + 1
        );
        let bets: Vec<String> = (0..6)
            .filter(|&i| self.bets[i] > 0)
            .map(|i| format!("{} {}", SYMS[i], self.bets[i]))
            .collect();
        s.push_str(&format!(
            "Mize: {}\n",
            if bets.is_empty() {
                "—".into()
            } else {
                bets.join(", ")
            }
        ));
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        match self.outcome() {
            Some(Outcome::Win(p)) => s.push_str(&format!("Jucătorul {} câștigă.\n", p.idx() + 1)),
            Some(Outcome::Draw) => s.push_str("Remiză.\n"),
            None => s.push_str(&format!(
                "La mutare: Jucătorul {}\n",
                self.to_move().idx() + 1
            )),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Bet(s, k) => format!("{} {}", SYMS[*s as usize], k),
            Move::Done => "gata".into(),
            Move::Roll => "arunc".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn games_end_and_chips_are_conserved() {
        for seed in 1..=20u64 {
            let mut g = BauCua::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                g.apply(&m[rng.below(m.len())]);
                assert_eq!(g.chips[0] + g.chips[1], 2 * START);
                n += 1;
                assert!(n < 1000);
            }
        }
    }

    #[test]
    fn payout_per_matching_die() {
        let mut g = BauCua::with_seed(4);
        g.bets[0] = 10;
        g.rolling = true;
        let before = g.chips;
        let mut c = g.clone();
        c.apply(&Move::Roll);
        let net = c.chips[c.bettor_prev()] - before[c.bettor_prev()];
        assert!(net == -10 || net == 10 || net == 20 || net == 30);
    }

    impl BauCua {
        fn bettor_prev(&self) -> usize {
            // în runda 0 parierul e J1 (bancher J2); după Roll runda crește
            0
        }
    }
}
