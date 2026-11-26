#![allow(clippy::manual_is_multiple_of)]
//! Sticla (Spin the bottle) — joc de petrecere cu sticla învârtită, versiune familială pentru 2 jucători.
//!
//! Șase locuri în cerc: J1 ocupă 1, 3, 5 și J2 ocupă 2, 4, 6. Pe rând, jucătorul-„învârtitor” învârte sticla
//! (mutarea forțată `invart`, rezultat aleator). Sticla arată un loc; învârtitorul alege apoi dificultatea
//! `nivel 1`, `nivel 2` sau `nivel 3` a unei mici probe amuzante (de ex. imită un animal, spune un cântec). Proprietarul
//! locului ales (poate fi chiar învârtitorul) răspunde cu `fac` (încearcă proba) sau `refuz` (−1 punct). Când încearcă, mutarea forțată
//! `incerc` stabilește la întâmplare reușita: șanse 90% / 60% / 30% pentru nivelurile 1/2/3; reușita
//! aduce atâtea puncte cât nivelul, eșecul costă 1. 8 ture (4 pentru fiecare ca învârtitor); câștigă scorul mai mare.
//! Nu există informație ascunsă; AI-ul nu vede rezultatul sticlei sau al probei înainte de decizie.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const TURNS: u8 = 8;
const CHANCE: [i32; 3] = [90, 60, 30];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Spin,
    Level(u8),
    Do,
    Refuse,
    Try,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Spin,
    Level,
    Respond,
    Resolve,
    Over,
}

#[derive(Clone)]
pub struct Sticla {
    rng: Rng,
    turn: u8,
    scores: [i32; 2],
    target: u8,
    level: u8,
    phase: Phase,
    log: String,
}

impl Default for Sticla {
    fn default() -> Self {
        Self::new()
    }
}

impl Sticla {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Sticla {
            rng: Rng::new(seed),
            turn: 1,
            scores: [0; 2],
            target: 0,
            level: 1,
            phase: Phase::Spin,
            log: String::new(),
        }
    }

    pub fn scores(&self) -> (i32, i32) {
        (self.scores[0], self.scores[1])
    }

    fn spinner(&self) -> Player {
        if self.turn % 2 == 1 {
            Player::One
        } else {
            Player::Two
        }
    }

    /// proprietarul locului (locurile 0..6: pare = J1, impare = J2)
    fn owner(&self) -> Player {
        if self.target % 2 == 0 {
            Player::One
        } else {
            Player::Two
        }
    }

    fn ev(level: u8) -> i32 {
        let p = CHANCE[level as usize - 1];
        level as i32 * p - (100 - p)
    }
}

impl Game for Sticla {
    type Move = Move;

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Respond | Phase::Resolve => self.owner(),
            _ => self.spinner(),
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        match self.phase {
            Phase::Over => Vec::new(),
            Phase::Spin => vec![Move::Spin],
            Phase::Level => vec![Move::Level(1), Move::Level(2), Move::Level(3)],
            Phase::Respond => vec![Move::Do, Move::Refuse],
            Phase::Resolve => vec![Move::Try],
        }
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        match *m {
            Move::Spin => {
                self.target = self.rng.below(6) as u8;
                self.log = format!(
                    "Sticla arată locul {} (J{})",
                    self.target + 1,
                    self.owner().idx() + 1
                );
                self.phase = Phase::Level;
            }
            Move::Level(l) => {
                self.level = l;
                self.phase = Phase::Respond;
            }
            Move::Refuse => {
                self.scores[self.owner().idx()] -= 1;
                self.log = format!("J{} refuză proba (-1)", self.owner().idx() + 1);
                self.next_turn();
            }
            Move::Do => self.phase = Phase::Resolve,
            Move::Try => {
                let o = self.owner().idx();
                if (self.rng.below(100) as i32) < CHANCE[self.level as usize - 1] {
                    self.scores[o] += self.level as i32;
                    self.log = format!(
                        "J{} reușește proba de nivel {} (+{})",
                        o + 1,
                        self.level,
                        self.level
                    );
                } else {
                    self.scores[o] -= 1;
                    self.log = format!("J{} ratează proba (-1)", o + 1);
                }
                self.next_turn();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        Some(match self.scores[0].cmp(&self.scores[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let mut v = (self.scores[p.idx()] - self.scores[p.other().idx()]) * 100;
        match self.phase {
            Phase::Respond | Phase::Resolve => {
                let sign = if self.owner() == p { 1 } else { -1 };
                v += sign * Self::ev(self.level);
            }
            _ => {}
        }
        v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Sticla — tura {}/{} | scor: J1 {}, J2 {}\n",
            self.turn, TURNS, self.scores[0], self.scores[1]
        );
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        let ring: Vec<String> = (0..6u8)
            .map(|i| {
                format!(
                    "{}{}",
                    if self.phase != Phase::Spin && self.phase != Phase::Over && i == self.target {
                        "*"
                    } else {
                        ""
                    },
                    format_args!("{}(J{})", i + 1, if i % 2 == 0 { 1 } else { 2 })
                )
            })
            .collect();
        s.push_str(&format!("Cerc: {}\n", ring.join("  ")));
        match self.phase {
            Phase::Over => s.push_str("Joc încheiat.\n"),
            Phase::Spin => s.push_str(&format!(
                "J{} învârte sticla: scrie «invart».\n",
                self.spinner().idx() + 1
            )),
            Phase::Level => s.push_str(&format!(
                "J{} alege «nivel 1», «nivel 2» sau «nivel 3».\n",
                self.spinner().idx() + 1
            )),
            Phase::Respond => s.push_str(&format!(
                "J{}: proba de nivel {} — «fac» sau «refuz»?\n",
                self.owner().idx() + 1,
                self.level
            )),
            Phase::Resolve => s.push_str("Scrie «incerc».\n"),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Spin => "invart".into(),
            Move::Level(l) => format!("nivel {l}"),
            Move::Do => "fac".into(),
            Move::Refuse => "refuz".into(),
            Move::Try => "incerc".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

impl Sticla {
    fn next_turn(&mut self) {
        if self.turn >= TURNS {
            self.phase = Phase::Over;
        } else {
            self.turn += 1;
            self.phase = Phase::Spin;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flow_and_scores() {
        let mut g = Sticla::with_seed(1);
        g.apply(&Move::Spin);
        assert_eq!(g.to_move(), Player::One);
        g.apply(&Move::Level(2));
        let owner = g.owner();
        assert_eq!(g.to_move(), owner);
        g.apply(&Move::Refuse);
        assert_eq!(g.scores[owner.idx()], -1);
        assert_eq!(g.turn, 2);
        assert_eq!(g.to_move(), Player::Two);
    }

    #[test]
    fn ev_values() {
        assert_eq!(Sticla::ev(1), 90 - 10);
        assert_eq!(Sticla::ev(2), 120 - 40);
        assert_eq!(Sticla::ev(3), 90 - 70);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=6u64 {
            let mut g = Sticla::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut k = 0;
            while g.outcome().is_none() && k < 4000 {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
                k += 1;
            }
            assert!(g.outcome().is_some());
        }
    }
}
