//! Elasticul (Chinese jump rope / „elastic”), 2 jucători — joc de sărituri pe niveluri.
//!
//! * Elasticul urcă prin 7 niveluri (1 glezne, 2 genunchi, 3 coapse, 4 talie, 5 piept, 6 subsuori, 7 gât).
//! * La fiecare nivel sare întâi J1, apoi J2. Alegi `usor` (șanse 92% − 8%·nivel, câștigi nivelul în puncte)
//!   sau `greu` (șanse 80% − 10%·nivel, câștigi 2·nivel+1 puncte); apoi forțata `sari` aruncă zarul.
//! * Cine greșește e eliminat și nu mai sare. Jocul se termină când ambii sunt eliminați sau după nivelul 7;
//!   mai multe puncte câștigă, egal = remiză.
//!
//! Mutări: `usor` | `greu`, apoi `sari`.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const LEVELS: u8 = 7;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Easy,
    Hard,
    Jump,
}

#[derive(Clone)]
pub struct Elasticul {
    rng: Rng,
    level: u8,
    side: Player,
    out: [bool; 2],
    score: [i32; 2],
    pending: Option<bool>,
    log: String,
}

impl Default for Elasticul {
    fn default() -> Self {
        Self::new()
    }
}

fn chance(level: u8, hard: bool) -> i32 {
    let l = level as i32;
    if hard {
        80 - 10 * l
    } else {
        92 - 8 * l
    }
}

impl Elasticul {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        Elasticul {
            rng: Rng::new(seed),
            level: 1,
            side: Player::One,
            out: [false; 2],
            score: [0, 0],
            pending: None,
            log: String::new(),
        }
    }
    fn over(&self) -> bool {
        self.level > LEVELS || (self.out[0] && self.out[1])
    }
    fn advance(&mut self) {
        // alege următorul jucător neeliminat; nivelul crește după J2
        let from = self.side;
        let mut next = from.other();
        if from == Player::Two {
            self.level += 1;
        }
        if self.out[next.idx()] {
            if self.out[from.idx()] {
                return;
            }
            // celălalt e eliminat: același jucător sare din nou, la nivelul următor dacă a trecut J2
            if next == Player::Two {
                self.level += 1;
            }
            next = from;
        }
        self.side = next;
    }
}

impl Game for Elasticul {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over() {
            return vec![];
        }
        if self.pending.is_some() {
            return vec![Move::Jump];
        }
        vec![Move::Easy, Move::Hard]
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Easy => self.pending = Some(false),
            Move::Hard => self.pending = Some(true),
            Move::Jump => {
                let hard = self.pending.take().unwrap();
                let ok = (self.rng.below(100) as i32) < chance(self.level, hard);
                let pts = if hard {
                    2 * self.level as i32 + 1
                } else {
                    self.level as i32
                };
                if ok {
                    self.score[self.side.idx()] += pts;
                    self.log = format!(
                        "J{} sare peste nivelul {} (+{pts}).",
                        self.side.idx() + 1,
                        self.level
                    );
                } else {
                    self.out[self.side.idx()] = true;
                    self.log = format!(
                        "J{} a greșit la nivelul {} și e eliminat.",
                        self.side.idx() + 1,
                        self.level
                    );
                }
                self.advance();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over() {
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
        let mut v = (self.score[p.idx()] - self.score[o.idx()]) * 10;
        if self.out[o.idx()] && !self.out[p.idx()] {
            v += 15;
        }
        if self.out[p.idx()] && !self.out[o.idx()] {
            v -= 15;
        }
        if let Some(h) = self.pending {
            if self.side == p {
                let pts = if h {
                    2 * self.level as i32 + 1
                } else {
                    self.level as i32
                };
                v += pts * chance(self.level, h) / 10;
            }
        }
        v
    }

    fn render(&self) -> String {
        let names = [
            "glezne", "genunchi", "coapse", "talie", "piept", "subsuori", "gât",
        ];
        let mut s = format!(
            "Elasticul — nivel {} ({}) | J1 {} p{} | J2 {} p{}\n",
            self.level.min(LEVELS),
            names[(self.level.min(LEVELS) - 1) as usize],
            self.score[0],
            if self.out[0] { " (eliminat)" } else { "" },
            self.score[1],
            if self.out[1] { " (eliminat)" } else { "" }
        );
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        match self.outcome() {
            Some(Outcome::Win(p)) => s.push_str(&format!("Jucătorul {} câștigă.\n", p.idx() + 1)),
            Some(Outcome::Draw) => s.push_str("Remiză.\n"),
            None => {
                if let Some(h) = self.pending {
                    s.push_str(&format!(
                        "Salt {} la nivelul {}: scrie «sari». Șanse: {}%.\n",
                        if h { "greu" } else { "ușor" },
                        self.level,
                        chance(self.level, h)
                    ));
                } else {
                    s.push_str(&format!(
                        "Șanse la nivelul {}: ușor {}% | greu {}%\n",
                        self.level,
                        chance(self.level, false),
                        chance(self.level, true)
                    ));
                }
                s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
            }
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Easy => "usor",
            Move::Hard => "greu",
            Move::Jump => "sari",
        }
        .into()
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
        for seed in 0..50u64 {
            let mut g = Elasticul::with_seed(seed);
            let mut r = Rng::new(seed + 1);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[r.below(m.len())]);
                n += 1;
                assert!(n < 100);
            }
        }
    }

    #[test]
    fn eliminated_player_is_skipped() {
        let mut g = Elasticul::with_seed(1);
        g.out[1] = true;
        g.side = Player::One;
        g.apply(&Move::Easy);
        g.apply(&Move::Jump);
        if !g.out[0] {
            assert_eq!(g.side, Player::One);
            assert_eq!(g.level, 2);
        }
    }
}
