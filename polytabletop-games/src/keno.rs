//! Keno (China / cazinouri), 2 jucători în întrecere, din regulile publice.
//!
//! * Numerele 1–80. Într-o rundă fiecare jucător alege **5 numere** (J1, apoi J2); apoi se extrag 20 de numere
//!   din 80. Punctele rundei = tabelul de plată (propriul nostru, pentru 5 numere alese): 2 nimeriri = 1,
//!   3 = 3, 4 = 20, 5 = 400 de puncte; 0–1 nimeriri = 0.
//! * Se joacă 5 runde; câștigă cine are mai multe puncte (egal = remiză).
//! * Extragerea e o mutare forțată (`extrag`) făcută după ce ambii au ales, cu generatorul jocului, deci AI-ul
//!   nu poate ști numerele viitoare.
//!
//! Mutări: `n17` (alegi numărul 17; numerele alese se dau în ordine crescătoare), `random` (completezi
//! aleatoriu până la 5), `extrag`.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const ROUNDS: u8 = 5;
const SPOTS: usize = 5;
const PAY: [i32; 6] = [0, 0, 1, 3, 20, 400];

#[derive(Clone, PartialEq, Debug)]
pub enum Move {
    Pick(u8),
    Random,
    Draw,
}

#[derive(Clone)]
pub struct Keno {
    rng: Rng,
    picks: [Vec<u8>; 2],
    score: [i32; 2],
    round: u8,
    side: Player,
    drawn: Vec<u8>,
    log: String,
}

impl Default for Keno {
    fn default() -> Self {
        Self::new()
    }
}

impl Keno {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Keno {
            rng: Rng::new(seed),
            picks: [Vec::new(), Vec::new()],
            score: [0, 0],
            round: 0,
            side: Player::One,
            drawn: Vec::new(),
            log: String::new(),
        }
    }

    pub fn scores(&self) -> [i32; 2] {
        self.score
    }

    fn over(&self) -> bool {
        self.round >= ROUNDS
    }

    fn drawing(&self) -> bool {
        self.picks[0].len() == SPOTS && self.picks[1].len() == SPOTS
    }
}

impl Game for Keno {
    type Move = Move;

    fn to_move(&self) -> Player {
        if self.drawing() {
            Player::One
        } else {
            self.side
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over() {
            return Vec::new();
        }
        if self.drawing() {
            return vec![Move::Draw];
        }
        let mine = &self.picks[self.side.idx()];
        let last = mine.last().copied().unwrap_or(0);
        let mut v: Vec<Move> = (last + 1..=80).map(Move::Pick).collect();
        // trebuie să mai rămână destule numere mai mari pentru a completa 5
        let need = SPOTS - mine.len();
        v.retain(|m| match m {
            Move::Pick(n) => 80 - *n as usize >= need - 1,
            _ => true,
        });
        v.push(Move::Random);
        v
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Pick(n) => {
                self.picks[self.side.idx()].push(*n);
            }
            Move::Random => {
                let me = self.side.idx();
                while self.picks[me].len() < SPOTS {
                    let n = self.rng.below(80) as u8 + 1;
                    if !self.picks[me].contains(&n) {
                        self.picks[me].push(n);
                    }
                }
                self.picks[me].sort_unstable();
            }
            Move::Draw => {
                let mut pool: Vec<u8> = (1..=80).collect();
                self.drawn.clear();
                for _ in 0..20 {
                    let k = self.rng.below(pool.len());
                    self.drawn.push(pool.swap_remove(k));
                }
                self.drawn.sort_unstable();
                let mut log = String::new();
                for p in 0..2 {
                    let hits = self.picks[p]
                        .iter()
                        .filter(|n| self.drawn.contains(n))
                        .count();
                    self.score[p] += PAY[hits];
                    log.push_str(&format!("J{}: {} nimeriri (+{}). ", p + 1, hits, PAY[hits]));
                }
                self.log = log;
                self.picks = [Vec::new(), Vec::new()];
                self.round += 1;
                self.side = Player::One;
                return;
            }
        }
        let me = self.side.idx();
        if self.picks[me].len() == SPOTS && self.side == Player::One {
            self.side = Player::Two;
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
        // extragerea e necunoscută: contează doar scorul curent
        (self.score[p.idx()] - self.score[p.other().idx()]) * 10
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Keno — runda {}/{ROUNDS}, scor J1 {} – J2 {}\n",
            (self.round + 1).min(ROUNDS),
            self.score[0],
            self.score[1]
        );
        if !self.log.is_empty() {
            s.push_str(&format!(
                "Ultima extragere: {:?}\n{}\n",
                self.drawn, self.log
            ));
        }
        for p in 0..2 {
            s.push_str(&format!("J{} a ales: {:?}\n", p + 1, self.picks[p]));
        }
        if self.over() {
            s.push_str("Joc încheiat.\n");
        } else if self.drawing() {
            s.push_str("Gata de extragere: scrie «extrag».\n");
        } else {
            s.push_str(&format!(
                "J{} alege (încă {} numere): nXX în ordine crescătoare sau «random».\n",
                self.side.idx() + 1,
                SPOTS - self.picks[self.side.idx()].len()
            ));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Pick(n) => format!("n{n}"),
            Move::Random => "random".into(),
            Move::Draw => "extrag".into(),
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
    fn five_rounds_and_scores() {
        let mut g = Keno::with_seed(7);
        let mut rng = Rng::new(3);
        let mut n = 0;
        while g.outcome().is_none() {
            let m = g.legal_moves();
            g.apply(&m[rng.below(m.len())]);
            n += 1;
            assert!(n < 500);
        }
        assert_eq!(g.round, 5);
    }

    #[test]
    fn picks_are_ascending_and_completable() {
        let g2 = Keno::with_seed(1);
        assert!(!g2.legal_moves().contains(&Move::Pick(78)));
        assert!(g2.legal_moves().contains(&Move::Pick(76)));
    }
}
