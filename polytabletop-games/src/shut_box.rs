#![allow(clippy::needless_range_loop, clippy::manual_range_contains)]
//! „Închide cutia” (Shut the Box) — joc tradițional de zaruri cu plăcuțe 1–9, 2 jucători.
//!
//! * Fiecare jucător are cutia lui cu plăcuțele 1–9 deschise. La rândul tău arunci zarurile
//!   (`arunc`): două zaruri, sau unul singur dacă 7, 8 și 9 sunt deja închise.
//! * Închizi plăcuțe a căror sumă e exact aruncarea (`inchid 1+2+6`). Apoi arunci din nou.
//!   Când nicio combinație nu se potrivește, tura se termină și scorul tău e suma plăcuțelor
//!   rămase deschise. Cutia complet închisă = „cutie închisă”: câștigi pe loc.
//! * 5 runde; câștigă cel mai mic total (egal = remiză).
//!
//! Informație: zarurile se aruncă la mutarea forțată `arunc`, deci AI-ul nu le vede în avans.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const ROUNDS: u8 = 5;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Roll,
    Close(Vec<u8>),
}

#[derive(Clone)]
pub struct ShutBox {
    rng: Rng,
    open: [bool; 9],
    dice: Option<(u8, u8)>,
    side: Player,
    round: u8,
    totals: [i32; 2],
    shut: Option<Player>,
    log: String,
}

impl Default for ShutBox {
    fn default() -> Self {
        Self::new()
    }
}

fn subsets(open: &[bool; 9], target: u8) -> Vec<Vec<u8>> {
    let tiles: Vec<u8> = (1..=9u8).filter(|&t| open[t as usize - 1]).collect();
    let mut out = Vec::new();
    for mask in 1u32..(1 << tiles.len()) {
        let mut s = 0u8;
        let mut v = Vec::new();
        for (i, &t) in tiles.iter().enumerate() {
            if mask & (1 << i) != 0 {
                s += t;
                v.push(t);
            }
        }
        if s == target {
            out.push(v);
        }
    }
    out
}

impl ShutBox {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        ShutBox {
            rng: Rng::new(seed),
            open: [true; 9],
            dice: None,
            side: Player::One,
            round: 1,
            totals: [0, 0],
            shut: None,
            log: String::new(),
        }
    }

    fn over(&self) -> bool {
        self.shut.is_some() || self.round > ROUNDS
    }

    fn open_sum(&self) -> i32 {
        (1..=9).filter(|&t| self.open[t - 1]).sum::<usize>() as i32
    }

    fn one_die(&self) -> bool {
        !self.open[6] && !self.open[7] && !self.open[8]
    }

    fn end_turn(&mut self) {
        let s = self.open_sum();
        self.totals[self.side.idx()] += s;
        self.log = format!("J{} încheie tura cu {} puncte", self.side.idx() + 1, s);
        self.open = [true; 9];
        self.dice = None;
        if self.side == Player::Two {
            self.round += 1;
        }
        self.side = self.side.other();
    }
}

impl Game for ShutBox {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over() {
            return Vec::new();
        }
        match self.dice {
            None => vec![Move::Roll],
            Some((a, b)) => subsets(&self.open, a + b)
                .into_iter()
                .map(Move::Close)
                .collect(),
        }
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        match m {
            Move::Roll => {
                let a = self.rng.d6();
                let b = if self.one_die() { 0 } else { self.rng.d6() };
                self.dice = Some((a, b));
                if subsets(&self.open, a + b).is_empty() {
                    let l = format!(
                        "J{} aruncă {} — nicio combinație",
                        self.side.idx() + 1,
                        a + b
                    );
                    self.end_turn();
                    self.log = format!("{}; {}", l, self.log);
                }
            }
            Move::Close(v) => {
                for &t in v {
                    self.open[t as usize - 1] = false;
                }
                self.dice = None;
                if self.open_sum() == 0 {
                    self.shut = Some(self.side);
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if let Some(p) = self.shut {
            return Some(Outcome::Win(p));
        }
        if self.round <= ROUNDS {
            return None;
        }
        Some(match self.totals[0].cmp(&self.totals[1]) {
            std::cmp::Ordering::Less => Outcome::Win(Player::One),
            std::cmp::Ordering::Greater => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        // Mai puține puncte deschise = mai bine; plăcuțele mari închise primele.
        let mut v = -self.open_sum() * 10;
        let low_open = (1..=6).filter(|&t| self.open[t - 1]).count() as i32;
        v += low_open * 4;
        if self.side == p {
            v
        } else {
            -v
        }
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Închide cutia — runda {}/{} | totaluri: J1 {}, J2 {}\n",
            self.round.min(ROUNDS),
            ROUNDS,
            self.totals[0],
            self.totals[1]
        );
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        let boxs: Vec<String> = (1..=9)
            .map(|t| {
                if self.open[t - 1] {
                    t.to_string()
                } else {
                    "·".into()
                }
            })
            .collect();
        s.push_str(&format!(
            "Cutia lui J{}: [{}]\n",
            self.side.idx() + 1,
            boxs.join(" ")
        ));
        if let Some((a, b)) = self.dice {
            s.push_str(&format!(
                "Zaruri: {}{} = {}\n",
                a,
                if b > 0 {
                    format!(" + {}", b)
                } else {
                    String::new()
                },
                a + b
            ));
        }
        if let Some(p) = self.shut {
            s.push_str(&format!("CUTIE ÎNCHISĂ! J{} câștigă.\n", p.idx() + 1));
        } else if !self.over() {
            s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Roll => "arunc".into(),
            Move::Close(v) => format!(
                "inchid {}",
                v.iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join("+")
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
    fn subsets_sum() {
        let o = [true; 9];
        let v = subsets(&o, 9);
        assert!(v.contains(&vec![9]) && v.contains(&vec![1, 2, 6]) && v.contains(&vec![4, 5]));
        assert!(v
            .iter()
            .all(|s| s.iter().map(|&x| x as u32).sum::<u32>() == 9));
        let mut o2 = [false; 9];
        o2[0] = true;
        assert!(subsets(&o2, 5).is_empty());
    }

    #[test]
    fn no_combination_ends_turn() {
        let mut g = ShutBox::with_seed(3);
        g.open = [true, false, false, false, false, false, false, false, false];
        g.rng = Rng::new(1);
        g.apply(&Move::Roll);
        // doar plăcuța 1 deschisă: aruncarea (cel puțin 1 zar, 1..12) fără 1 => tura se termină
        assert!(g.side == Player::Two || g.dice.is_some());
    }

    #[test]
    fn random_games_end_and_ai_legal() {
        for seed in 1..=10u64 {
            let mut g = ShutBox::with_seed(seed);
            let mut rng = Rng::new(seed + 11);
            let mut n = 0;
            while g.outcome().is_none() && n < 5000 {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
            }
            assert!(g.outcome().is_some());
        }
    }

    #[test]
    fn shut_wins() {
        let mut g = ShutBox::with_seed(2);
        g.open = [false; 9];
        g.open[2] = true;
        g.dice = Some((1, 2));
        g.apply(&Move::Close(vec![3]));
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }
}
