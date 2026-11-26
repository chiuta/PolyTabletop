//! Shove ha'penny (Marea Britanie), 2 jucători — simulare a jocului de îndemânare.
//!
//! * Masa are 9 „paturi” (benzi) numerotate 1–9. Fiecare jucător împinge pe rând câte o monedă (5 pe rundă).
//! * Un jucător „umple” un pat când are în el 3 monede; o monedă aterizată într-un pat deja plin pentru
//!   împingător (3 ale lui) se creditează adversarului, dacă acesta are în pat mai puțin de 3; altfel e fără
//!   efect. Câștigă primul care are 3 monede în fiecare dintre cele 9 paturi.
//! * Fiindcă jocul real cere îndemânare, o împingere e **simulată**: alegi patul țintă și tipul loviturii;
//!   rezultatul se trage cu generatorul jocului. `precis` (`p4`): 50% exact, 15% un pat prea scurt, 10% un pat
//!   prea lung, 20% pe linie (nu contează), 5% cade de pe masă; `puternic` (`f4`): 40% exact, 5% scurt, 25%
//!   lung, 15% pe linie, 15% cade. O monedă care ar trece de patul 9 cade de pe masă.
//! * Împingerea se face în doi pași: alegi ținta, apoi mutarea forțată `lansez` o aruncă (AI-ul nu vede
//!   rezultatul înainte să aleagă). După 60 de runde fără câștigător câștigă cine a umplut mai mult
//!   (suma pătratelor cu plafon 3).
//!
//! Mutări: `p4`, `f7` (țintă + tip), apoi `lansez`.
#![allow(clippy::if_same_then_else)]

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const BEDS: usize = 9;
const COINS: u8 = 5;
const MAX_ROUNDS: u16 = 60;

#[derive(Clone, PartialEq, Debug)]
pub enum Move {
    Aim(u8, bool),
    Release,
}

#[derive(Clone)]
pub struct ShoveHaPenny {
    rng: Rng,
    beds: [[u8; BEDS]; 2],
    side: Player,
    thrown: [u8; 2],
    pending: Option<(u8, bool)>,
    round: u16,
    log: String,
}

impl Default for ShoveHaPenny {
    fn default() -> Self {
        Self::new()
    }
}

impl ShoveHaPenny {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        ShoveHaPenny {
            rng: Rng::new(seed),
            beds: [[0; BEDS]; 2],
            side: Player::One,
            thrown: [0, 0],
            pending: None,
            round: 0,
            log: String::new(),
        }
    }

    fn full(&self, p: usize) -> bool {
        self.beds[p].iter().all(|&c| c >= 3)
    }

    fn filled(&self, p: usize) -> i32 {
        self.beds[p].iter().map(|&c| c.min(3) as i32).sum()
    }
}

impl Game for ShoveHaPenny {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        if self.pending.is_some() {
            return vec![Move::Release];
        }
        let mut v = Vec::new();
        for b in 1..=BEDS as u8 {
            v.push(Move::Aim(b, false));
            v.push(Move::Aim(b, true));
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Aim(b, strong) => {
                self.pending = Some((*b, *strong));
            }
            Move::Release => {
                let (b, strong) = self.pending.take().unwrap();
                let me = self.side.idx();
                let r = self.rng.below(100);
                // praguri cumulative: exact, scurt, lung, linie, jos
                let (e, s, l, ln) = if strong {
                    (40, 45, 70, 85)
                } else {
                    (50, 65, 75, 95)
                };
                let landed: Option<i32> = if r < e {
                    Some(b as i32)
                } else if r < s {
                    Some(b as i32 - 1)
                } else if r < l {
                    Some(b as i32 + 1)
                } else if r < ln {
                    None
                } else {
                    None
                };
                let off = r >= ln;
                let mut log = format!(
                    "J{}: țintă {b}{} → ",
                    me + 1,
                    if strong { " (puternic)" } else { "" }
                );
                match landed {
                    Some(x) if (1..=BEDS as i32).contains(&x) => {
                        let i = x as usize - 1;
                        if self.beds[me][i] < 3 {
                            self.beds[me][i] += 1;
                            log.push_str(&format!("patul {x}: +1 pentru J{}", me + 1));
                        } else if self.beds[1 - me][i] < 3 {
                            self.beds[1 - me][i] += 1;
                            log.push_str(&format!("patul {x} plin: +1 pentru J{}", 2 - me));
                        } else {
                            log.push_str(&format!("patul {x}: fără efect"));
                        }
                    }
                    _ => {
                        if off {
                            log.push_str("a căzut de pe masă");
                        } else if landed.is_none() {
                            log.push_str("pe linie, nu contează");
                        } else {
                            log.push_str("în afara patului, nu contează");
                        }
                    }
                }
                self.log = log;
                self.thrown[me] += 1;
                self.side = self.side.other();
                if self.thrown[0] >= COINS && self.thrown[1] >= COINS {
                    self.thrown = [0, 0];
                    self.round += 1;
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        let (a, b) = (self.full(0), self.full(1));
        if a && !b {
            return Some(Outcome::Win(Player::One));
        }
        if b && !a {
            return Some(Outcome::Win(Player::Two));
        }
        if a && b {
            return Some(Outcome::Draw);
        }
        if self.round >= MAX_ROUNDS {
            return Some(match self.filled(0).cmp(&self.filled(1)) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                std::cmp::Ordering::Equal => Outcome::Draw,
            });
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut v = (self.filled(me) - self.filled(1 - me)) * 10;
        // țintele încă deschise valorează mai mult
        if let Some((b, _)) = self.pending {
            let i = b as usize - 1;
            let open_me = self.beds[me][i] < 3;
            let open_opp = self.beds[1 - me][i] < 3;
            if self.side == p {
                v += if open_me {
                    5
                } else if open_opp {
                    2
                } else {
                    -5
                };
            }
        }
        v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Shove ha'penny — runda {}/{MAX_ROUNDS}, monede aruncate în rundă: J1 {}/{COINS}, J2 {}/{COINS}\n",
            self.round + 1,
            self.thrown[0],
            self.thrown[1]
        );
        s.push_str("Pat: ");
        for b in 1..=BEDS {
            s.push_str(&format!("{b:>3}"));
        }
        s.push('\n');
        for p in 0..2 {
            s.push_str(&format!("J{}:  ", p + 1));
            for b in 0..BEDS {
                s.push_str(&format!("{:>3}", self.beds[p][b]));
            }
            s.push('\n');
        }
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        match self.outcome() {
            Some(Outcome::Win(p)) => s.push_str(&format!("Jucătorul {} câștigă.\n", p.idx() + 1)),
            Some(Outcome::Draw) => s.push_str("Remiză.\n"),
            None => {
                if self.pending.is_some() {
                    s.push_str("Țintă aleasă: scrie «lansez».\n");
                }
                s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
            }
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Aim(b, false) => format!("p{b}"),
            Move::Aim(b, true) => format!("f{b}"),
            Move::Release => "lansez".into(),
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
    fn random_games_end() {
        for seed in 1..=5u64 {
            let mut g = ShoveHaPenny::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 3000);
            }
        }
    }

    #[test]
    fn overflow_credits_the_opponent() {
        let mut g = ShoveHaPenny::with_seed(1);
        g.beds[0][3] = 3;
        let mut hits = 0;
        for _ in 0..200 {
            let mut c = g.clone();
            c.apply(&Move::Aim(4, false));
            c.apply(&Move::Release);
            if c.beds[1][3] == 1 {
                hits += 1;
            }
            assert_eq!(c.beds[0][3], 3);
        }
        assert!(hits > 20);
    }
}
