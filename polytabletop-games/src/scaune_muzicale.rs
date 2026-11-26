#![allow(clippy::manual_is_multiple_of)]
//! Scaunele muzicale (Musical chairs) — adaptare pe ture pentru 2 jucători.
//!
//! Jocul clasic elimină copii când muzica se oprește. Aici doi dansatori se învârt pe un cerc de 8 locuri,
//! iar în fiecare rundă există **un singur scaun** (loc marcat, ales la întâmplare). La mutare
//! faci un pas de 1, 2 sau 3 locuri în sens orar (nu poți ateriza pe locul adversarului; nu poți sta pe loc,
//! așa că cine e pe scaun trebuie să plece). După fiecare mutare muzica se poate opri: mutarea forțată
//! `asculta` hotărăște la întâmplare (1 șansă din 5) dacă muzica a încetat. Când se oprește, cine stă pe
//! scaun câștigă runda (punct); dacă nimeni nu e pe scaun, muzica reîncepe. Primul la 3 puncte câștigă.
//! Jocul nu are informație ascunsă; AI-ul nu vede în avans oprirea muzicii (rezolvată într-o mutare forțată).

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const RING: u8 = 8;
const TARGET: u8 = 3;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Step(u8),
    Listen,
}

#[derive(Clone)]
pub struct Scaune {
    rng: Rng,
    pos: [u8; 2],
    chair: u8,
    points: [u8; 2],
    side: Player,
    pending: bool,
    log: String,
}

impl Default for Scaune {
    fn default() -> Self {
        Self::new()
    }
}

impl Scaune {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Scaune {
            rng: Rng::new(seed),
            pos: [0, 4],
            chair: 2,
            points: [0; 2],
            side: Player::One,
            pending: false,
            log: String::new(),
        };
        g.new_round();
        g
    }

    pub fn points(&self) -> (u8, u8) {
        (self.points[0], self.points[1])
    }

    fn new_round(&mut self) {
        self.pos = [0, RING / 2];
        self.chair = [2u8, 3, 5, 6][self.rng.below(4)];
        self.pending = false;
    }
}

impl Game for Scaune {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        if self.pending {
            return vec![Move::Listen];
        }
        let me = self.side.idx();
        let other = self.pos[1 - me];
        (1..=3u8)
            .filter(|&k| (self.pos[me] + k) % RING != other)
            .map(Move::Step)
            .collect()
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        match *m {
            Move::Step(k) => {
                let me = self.side.idx();
                self.pos[me] = (self.pos[me] + k) % RING;
                self.pending = true;
            }
            Move::Listen => {
                self.pending = false;
                if self.rng.below(5) == 0 {
                    let on: Vec<usize> = (0..2).filter(|&i| self.pos[i] == self.chair).collect();
                    if let Some(&w) = on.first() {
                        self.points[w] += 1;
                        self.log = format!("Muzica s-a oprit: J{} stă pe scaun (+1)", w + 1);
                        if self.points[w] < TARGET {
                            self.new_round();
                        }
                    } else {
                        self.log = "Muzica s-a oprit, dar nimeni nu e pe scaun: continuă".into();
                    }
                }
                self.side = self.side.other();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.points[0] >= TARGET {
            Some(Outcome::Win(Player::One))
        } else if self.points[1] >= TARGET {
            Some(Outcome::Win(Player::Two))
        } else {
            None
        }
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let o = 1 - me;
        let mut v = (self.points[me] as i32 - self.points[o] as i32) * 1000;
        if self.pos[me] == self.chair {
            v += 400;
        }
        if self.pos[o] == self.chair {
            v -= 400;
        }
        let dist = |a: u8| ((self.chair + RING - a) % RING) as i32;
        v += 10 * (dist(self.pos[o]) - dist(self.pos[me]));
        v
    }

    fn render(&self) -> String {
        let mut cells = Vec::new();
        for i in 0..RING {
            let mut c = String::new();
            if i == self.chair {
                c.push('#');
            }
            if self.pos[0] == i {
                c.push_str("J1");
            }
            if self.pos[1] == i {
                c.push_str("J2");
            }
            if c.is_empty() {
                c.push('.');
            }
            cells.push(format!("{i}:{c}"));
        }
        let mut s = format!(
            "Scaunele muzicale — J1 {} : {} J2 (primul la {})\n",
            self.points[0], self.points[1], TARGET
        );
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        s.push_str(&format!("Cerc (# = scaunul): {}\n", cells.join("  ")));
        if self.outcome().is_none() {
            if self.pending {
                s.push_str("Muzica continuă? Scrie «asculta».\n");
            }
            s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Step(k) => format!("pas{k}"),
            Move::Listen => "asculta".into(),
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
    fn cannot_land_on_opponent_and_must_move() {
        let mut g = Scaune::with_seed(1);
        g.pos = [1, 3];
        let mv = g.legal_moves();
        assert!(!mv.contains(&Move::Step(2)));
        assert!(mv.contains(&Move::Step(1)) && mv.contains(&Move::Step(3)));
        g.apply(&Move::Step(1));
        assert_eq!(g.legal_moves(), vec![Move::Listen]);
    }

    #[test]
    fn chair_holder_scores_when_music_stops() {
        let mut g = Scaune::with_seed(1);
        g.chair = 2;
        g.pos = [2, 6];
        g.pending = true;
        for _ in 0..200 {
            g.pending = true;
            g.side = Player::One;
            g.apply(&Move::Listen);
            if g.points[0] > 0 {
                return;
            }
            g.pos = [2, 6];
        }
        panic!("muzica nu s-a oprit niciodată");
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=6u64 {
            let mut g = Scaune::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut k = 0;
            while g.outcome().is_none() && k < 4000 {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
                k += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed}");
        }
    }
}
