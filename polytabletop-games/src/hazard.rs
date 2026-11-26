//! Hazard (Marea Britanie / Orientul Mijlociu), strămoșul jocului craps, 2 jucători.
//!
//! * Aruncătorul (caster) strigă un **main** între 5 și 9 și o miză (`main 7 10`); celălalt jucător e banca.
//! * Prima aruncare cu două zaruri: suma = main → **nick** (aruncătorul câștigă); 2 sau 3 („crabs”) →
//!   pierde; 11 și 12 depind de main (main 5/9: ambele pierd; main 6/8: 11 pierde, 12 nick; main 7: 11 nick,
//!   12 pierde); orice altă sumă devine **chance**.
//! * Apoi se aruncă până iese chance (aruncătorul câștigă) sau main (pierde).
//! * Aruncătorul păstrează zarurile până pierde de trei ori la rând, apoi trec la celălalt.
//! * 100 de jetoane fiecare, 20 de mâini (sau până faliment); câștigă cine are mai multe jetoane.
//!
//! Mutări: `main 7 10` (main 5–9, miză 5/10/20), `arunc`.

use crate::q2_util::{by_score, end_line, risk};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const START: i32 = 100;
const HANDS: u8 = 20;
const STAKES: [i32; 3] = [5, 10, 20];

#[derive(Clone, PartialEq, Debug)]
pub enum Move {
    Call(u8, i32),
    Roll,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum First {
    Nick,
    Out,
    Chance,
}

fn first(main: u8, s: u8) -> First {
    if s == main {
        return First::Nick;
    }
    match s {
        2 | 3 => First::Out,
        11 => {
            if main == 7 {
                First::Nick
            } else {
                First::Out
            }
        }
        12 => {
            if main == 6 || main == 8 {
                First::Nick
            } else {
                First::Out
            }
        }
        _ => First::Chance,
    }
}

fn ways(s: u8) -> f64 {
    (6 - (s as i32 - 7).abs()).max(0) as f64
}

/// Probabilitatea ca aruncătorul să câștige cu acest main.
pub fn p_win(main: u8) -> f64 {
    let mut p = 0.0;
    for s in 2..=12u8 {
        let w = ways(s) / 36.0;
        match first(main, s) {
            First::Nick => p += w,
            First::Out => {}
            First::Chance => p += w * ways(s) / (ways(s) + ways(main)),
        }
    }
    p
}

#[derive(Clone)]
pub struct Hazard {
    rng: Rng,
    chips: [i32; 2],
    caster: Player,
    main: u8,
    chance: u8,
    stake: i32,
    losses: u8,
    hands: u8,
    last: String,
}

impl Default for Hazard {
    fn default() -> Self {
        Self::new()
    }
}

impl Hazard {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        Hazard {
            rng: Rng::new(seed),
            chips: [START; 2],
            caster: Player::One,
            main: 0,
            chance: 0,
            stake: 0,
            losses: 0,
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
    fn settle(&mut self, win: bool, why: &str) {
        let (c, b) = (self.caster.idx(), 1 - self.caster.idx());
        if win {
            self.chips[c] += self.stake;
            self.chips[b] -= self.stake;
            self.losses = 0;
            self.last = format!("{why}: aruncătorul câștigă {}.", self.stake);
        } else {
            self.chips[c] -= self.stake;
            self.chips[b] += self.stake;
            self.losses += 1;
            self.last = format!("{why}: aruncătorul pierde {}.", self.stake);
            if self.losses >= 3 {
                self.caster = self.caster.other();
                self.losses = 0;
                self.last.push_str(" Trei pierderi la rând: zarurile trec.");
            }
        }
        self.hands += 1;
        self.main = 0;
        self.chance = 0;
        self.stake = 0;
    }
}

impl Game for Hazard {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.caster
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over() {
            return Vec::new();
        }
        if self.main > 0 {
            return vec![Move::Roll];
        }
        let cap = self.chips[0].min(self.chips[1]);
        let mut v = Vec::new();
        for m in 5..=9u8 {
            for s in STAKES {
                if s <= cap {
                    v.push(Move::Call(m, s));
                }
            }
        }
        if v.is_empty() {
            v.push(Move::Call(7, cap));
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        match *m {
            Move::Call(main, s) => {
                self.main = main;
                self.stake = s;
                self.last.clear();
            }
            Move::Roll => {
                let (a, b) = (self.rng.d6(), self.rng.d6());
                let s = a + b;
                let txt = format!("{a}+{b}={s}");
                if self.chance == 0 {
                    match first(self.main, s) {
                        First::Nick => self.settle(true, &format!("{txt} nick")),
                        First::Out => self.settle(false, &format!("{txt} aruncat afară")),
                        First::Chance => {
                            self.chance = s;
                            self.last = format!("{txt}: chance {s}, main {}", self.main);
                        }
                    }
                } else if s == self.chance {
                    self.settle(true, &format!("{txt} chance"));
                } else if s == self.main {
                    self.settle(false, &format!("{txt} main"));
                } else {
                    self.last = format!("{txt}: se aruncă din nou");
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
        if self.main > 0 && self.chance == 0 {
            let ev = ((2.0 * p_win(self.main) - 1.0) * self.stake as f64 * 100.0) as i32;
            let who = if self.caster == p { 1 } else { -1 };
            v += who * ev + who * risk(self.stake, diff * who);
        }
        v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Hazard — mâna {}/{HANDS}. Jetoane: J1 {}, J2 {}. Aruncător: J{} (pierderi la rând: {})\n",
            (self.hands + 1).min(HANDS),
            self.chips[0],
            self.chips[1],
            self.caster.idx() + 1,
            self.losses
        );
        if self.main > 0 {
            s.push_str(&format!("Main {} | miza {}", self.main, self.stake));
            if self.chance > 0 {
                s.push_str(&format!(" | chance {}", self.chance));
            }
            s.push('\n');
        }
        if !self.last.is_empty() {
            s.push_str(&format!("({})\n", self.last));
        }
        s.push_str(&end_line(self.outcome(), self.caster));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Call(a, s) => format!("main {a} {s}"),
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
    fn table_and_probabilities() {
        assert_eq!(first(5, 5), First::Nick);
        assert_eq!(first(5, 12), First::Out);
        assert_eq!(first(6, 12), First::Nick);
        assert_eq!(first(7, 11), First::Nick);
        assert_eq!(first(9, 11), First::Out);
        assert_eq!(first(7, 3), First::Out);
        // main 7: dezavantajul aruncătorului ≈ 1,4 % (Wikipedia)
        assert!((p_win(7) - 0.4930).abs() < 0.002, "{}", p_win(7));
        assert!(p_win(7) > p_win(5));
    }

    #[test]
    fn random_games_end_conserving_chips() {
        for seed in 1..=20u64 {
            let mut g = Hazard::with_seed(seed);
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
    fn ai_picks_main_seven_without_peeking() {
        let a = Hazard::with_seed(1);
        let mut b = a.clone();
        b.rng = Rng::new(77);
        let m = polytabletop_core::GameObj::ai_move(&a, 2).unwrap();
        assert_eq!(Some(m.clone()), polytabletop_core::GameObj::ai_move(&b, 2));
        assert!(m.starts_with("main 7"), "{m}");
    }
}
