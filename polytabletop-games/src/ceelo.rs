//! Cee-lo (China / SUA), zarurile de stradă, 2 jucători: bancher și parior.
//!
//! * Trei zaruri. **Câștig automat**: 4-5-6 sau trei egale; **pierdere automată**: 1-2-3;
//!   o pereche + un zar diferit = **punct** (zarul diferit, 1–6); orice altceva = se rearuncă.
//! * Pariorul alege miza (`pariez 5/10/20`). Bancherul aruncă până iese un rezultat (`arunc`); dacă are
//!   punct, pariorul aruncă și el până iese un rezultat. Punctul mai mare câștigă, egal = remiză (miza
//!   rămâne). Banca trece la celălalt când bancherul pierde.
//! * 50 de jetoane fiecare, 16 mâini; câștigă cine are mai multe jetoane (cine rămâne fără, pierde).
//!
//! Aruncarea e mereu o mutare separată (`arunc`), deci AI-ul nu vede zarurile viitoare.

use crate::q2_util::{by_score, end_line, risk};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const START: i32 = 50;
const HANDS: u8 = 16;
const STAKES: [i32; 3] = [5, 10, 20];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Bet,
    BankRoll,
    PlayerRoll,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Move {
    Bet(i32),
    Roll,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Res {
    Win,
    Lose,
    Point(u8),
    Again,
}

pub fn classify(d: [u8; 3]) -> Res {
    let mut s = d;
    s.sort_unstable();
    if s == [4, 5, 6] || s[0] == s[2] {
        Res::Win
    } else if s == [1, 2, 3] {
        Res::Lose
    } else if s[0] == s[1] {
        Res::Point(s[2])
    } else if s[1] == s[2] {
        Res::Point(s[0])
    } else {
        Res::Again
    }
}

#[derive(Clone)]
pub struct CeeLo {
    rng: Rng,
    chips: [i32; 2],
    banker: Player,
    phase: Phase,
    stake: i32,
    point: u8,
    hands: u8,
    last: String,
}

impl Default for CeeLo {
    fn default() -> Self {
        Self::new()
    }
}

impl CeeLo {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        CeeLo {
            rng: Rng::new(seed),
            chips: [START; 2],
            banker: Player::One,
            phase: Phase::Bet,
            stake: 0,
            point: 0,
            hands: 0,
            last: String::new(),
        }
    }
    pub fn chips(&self) -> [i32; 2] {
        self.chips
    }
    fn over(&self) -> bool {
        self.hands >= HANDS || self.chips.iter().any(|&c| c <= 0)
    }
    fn better(&self) -> Player {
        self.banker.other()
    }
    fn settle(&mut self, banker_wins: Option<bool>, why: &str) {
        let (b, o) = (self.banker.idx(), self.better().idx());
        match banker_wins {
            Some(true) => {
                self.chips[b] += self.stake;
                self.chips[o] -= self.stake;
                self.last = format!("{why} — bancherul câștigă {}.", self.stake);
            }
            Some(false) => {
                self.chips[b] -= self.stake;
                self.chips[o] += self.stake;
                self.last = format!("{why} — pariorul câștigă {}; banca trece.", self.stake);
                self.banker = self.banker.other();
            }
            None => self.last = format!("{why} — remiză, miza rămâne."),
        }
        self.hands += 1;
        self.stake = 0;
        self.phase = Phase::Bet;
    }
    fn roll3(&mut self) -> [u8; 3] {
        [self.rng.d6(), self.rng.d6(), self.rng.d6()]
    }
}

impl Game for CeeLo {
    type Move = Move;

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Bet | Phase::PlayerRoll => self.better(),
            Phase::BankRoll => self.banker,
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over() {
            return Vec::new();
        }
        match self.phase {
            Phase::Bet => {
                let cap = self.chips[0].min(self.chips[1]);
                let mut v: Vec<Move> = STAKES
                    .iter()
                    .filter(|&&s| s <= cap)
                    .map(|&s| Move::Bet(s))
                    .collect();
                if v.is_empty() {
                    v.push(Move::Bet(cap));
                }
                v
            }
            _ => vec![Move::Roll],
        }
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Bet(s) => {
                self.stake = *s;
                self.phase = Phase::BankRoll;
                self.last.clear();
            }
            Move::Roll => {
                let d = self.roll3();
                let txt = format!("{} {} {}", d[0], d[1], d[2]);
                let r = classify(d);
                if self.phase == Phase::BankRoll {
                    match r {
                        Res::Win => {
                            self.settle(Some(true), &format!("Bancher {txt}: câștig automat"))
                        }
                        Res::Lose => self.settle(Some(false), &format!("Bancher {txt}: 1-2-3")),
                        Res::Point(p) => {
                            self.point = p;
                            self.phase = Phase::PlayerRoll;
                            self.last = format!("Bancher {txt}: punct {p}");
                        }
                        Res::Again => self.last = format!("Bancher {txt}: se rearuncă"),
                    }
                } else {
                    match r {
                        Res::Win => {
                            self.settle(Some(false), &format!("Parior {txt}: câștig automat"))
                        }
                        Res::Lose => self.settle(Some(true), &format!("Parior {txt}: 1-2-3")),
                        Res::Point(p) => {
                            let w = match p.cmp(&self.point) {
                                std::cmp::Ordering::Greater => Some(false),
                                std::cmp::Ordering::Less => Some(true),
                                std::cmp::Ordering::Equal => None,
                            };
                            self.settle(
                                w,
                                &format!("Parior {txt}: punct {p} contra {}", self.point),
                            );
                        }
                        Res::Again => self.last = format!("Parior {txt}: se rearuncă"),
                    }
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.over() {
            Some(by_score(self.chips[0], self.chips[1]))
        } else {
            None
        }
    }

    fn evaluate(&self, p: Player) -> i32 {
        let diff = self.chips[p.idx()] - self.chips[1 - p.idx()];
        let mut v = diff * 100;
        if self.stake > 0 && self.phase == Phase::BankRoll {
            v += risk(self.stake, diff);
        }
        v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Cee-lo — mâna {}/{HANDS}. Jetoane: J1 {}, J2 {}. Bancher: J{}\n",
            (self.hands + 1).min(HANDS),
            self.chips[0],
            self.chips[1],
            self.banker.idx() + 1
        );
        if self.stake > 0 {
            s.push_str(&format!("Miza: {}", self.stake));
            if self.phase == Phase::PlayerRoll {
                s.push_str(&format!(" | punctul bancherului: {}", self.point));
            }
            s.push('\n');
        }
        if !self.last.is_empty() {
            s.push_str(&format!("({})\n", self.last));
        }
        s.push_str(&end_line(self.outcome(), self.to_move()));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Bet(s) => format!("pariez {s}"),
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
    fn classification() {
        assert_eq!(classify([6, 4, 5]), Res::Win);
        assert_eq!(classify([3, 3, 3]), Res::Win);
        assert_eq!(classify([3, 1, 2]), Res::Lose);
        assert_eq!(classify([2, 2, 5]), Res::Point(5));
        assert_eq!(classify([6, 1, 6]), Res::Point(1));
        assert_eq!(classify([1, 2, 4]), Res::Again);
    }

    #[test]
    fn random_games_conserve_chips() {
        for seed in 1..=20u64 {
            let mut g = CeeLo::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                g.apply(&m[rng.below(m.len())].clone());
                assert_eq!(g.chips[0] + g.chips[1], 2 * START);
                n += 1;
                assert!(n < 5000);
            }
        }
    }

    #[test]
    fn ai_bet_does_not_depend_on_rng() {
        let a = CeeLo::with_seed(1);
        let mut b = a.clone();
        b.rng = Rng::new(99);
        assert_eq!(
            polytabletop_core::GameObj::ai_move(&a, 4),
            polytabletop_core::GameObj::ai_move(&b, 4)
        );
    }
}
