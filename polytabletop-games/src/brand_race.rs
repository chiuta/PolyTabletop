//! Familia „cursă”: curse pe un traseu liniar, cu cărți de viteză deschise.
//! `peloton`: câte doi alergători pe jucător, aspirație și oboseală.
//! `cursa_motoare`: un bolid, curbe cu limită de viteză, căldură și supraturare.
//! Variante originale; cărțile noi se trag dintr-un zar re-însămânțat la `clone()`.
#![allow(clippy::needless_range_loop, clippy::manual_is_multiple_of)]

use crate::w7_util::Dice;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    Peloton,
    Heat,
}

pub struct Spec {
    pub id: &'static str,
    pub mode: Mode,
    pub len: i32,
    pub curves: &'static [i32],
    pub limit: u8,
}

pub const SPECS: &[Spec] = &[
    Spec {
        id: "peloton",
        mode: Mode::Peloton,
        len: 44,
        curves: &[],
        limit: 0,
    },
    Spec {
        id: "cursa_motoare",
        mode: Mode::Heat,
        len: 40,
        curves: &[8, 17, 26, 34],
        limit: 4,
    },
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RaMove {
    pub rider: u8,
    pub card: u8,
    pub boost: bool,
}

#[derive(Clone)]
pub struct Race {
    spec: &'static Spec,
    pos: [[i32; 2]; 2],
    hand: [[[u8; 3]; 2]; 2],
    tired: [[u8; 2]; 2],
    heat: [u8; 2],
    out: [bool; 2],
    side: Player,
    winner: Option<Player>,
    turns: u32,
    dice: Dice,
}

impl Race {
    pub fn new(spec: &'static Spec) -> Self {
        Self::with_seed(spec, Rng::from_clock().next_u64())
    }

    pub fn with_seed(spec: &'static Spec, seed: u64) -> Self {
        let mut g = Race {
            spec,
            pos: [[0; 2]; 2],
            hand: [[[0; 3]; 2]; 2],
            tired: [[0; 2]; 2],
            heat: [0; 2],
            out: [false; 2],
            side: Player::One,
            winner: None,
            turns: 0,
            dice: Dice::new(seed),
        };
        for p in 0..2 {
            for r in 0..g.riders() {
                for c in 0..3 {
                    g.hand[p][r][c] = g.draw(p, r);
                }
            }
        }
        g
    }

    fn riders(&self) -> usize {
        if self.spec.mode == Mode::Peloton {
            2
        } else {
            1
        }
    }

    fn draw(&mut self, p: usize, r: usize) -> u8 {
        match self.spec.mode {
            Mode::Peloton => {
                let t = self.tired[p][r] as usize;
                if self.dice.below(10 + t) >= 10 {
                    return 2;
                }
                // alergătorul 0 = sprinter (explozie), 1 = rulor (regulat)
                const SPRINT: [u8; 10] = [2, 3, 4, 5, 5, 5, 6, 7, 8, 9];
                const ROLL: [u8; 10] = [3, 3, 4, 4, 4, 5, 5, 5, 6, 7];
                let i = self.dice.below(10);
                if r == 0 {
                    SPRINT[i]
                } else {
                    ROLL[i]
                }
            }
            Mode::Heat => 1 + self.dice.below(5) as u8,
        }
    }

    fn crossed_curve(&self, from: i32, to: i32) -> bool {
        self.spec.curves.iter().any(|&c| from < c && to >= c)
    }
}

impl Game for Race {
    type Move = RaMove;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<RaMove> {
        if self.outcome().is_some() {
            return vec![];
        }
        let p = self.side.idx();
        let mut v = Vec::new();
        for r in 0..self.riders() {
            for c in 0..3 {
                // cărțile identice dau aceeași mutare: păstrăm prima
                if (0..c).any(|d| self.hand[p][r][d] == self.hand[p][r][c]) {
                    continue;
                }
                v.push(RaMove {
                    rider: r as u8,
                    card: c as u8,
                    boost: false,
                });
                if self.spec.mode == Mode::Heat && self.heat[p] < 5 {
                    v.push(RaMove {
                        rider: r as u8,
                        card: c as u8,
                        boost: true,
                    });
                }
            }
        }
        v
    }

    fn apply(&mut self, m: &RaMove) {
        let p = self.side.idx();
        let (r, c) = (m.rider as usize, m.card as usize);
        let val = self.hand[p][r][c] as i32;
        let from = self.pos[p][r];
        let mut step = val;
        match self.spec.mode {
            Mode::Peloton => {
                let mut to = from + step;
                // aspirație: dacă te oprești imediat în spatele cuiva, primești +1
                if (0..2).any(|q| (0..2).any(|k| (q, k) != (p, r) && self.pos[q][k] == to + 1)) {
                    to += 1;
                }
                let lead = (0..2).all(|q| (0..2).all(|k| self.pos[q][k] < to));
                if lead && val >= 5 && self.tired[p][r] < 6 {
                    self.tired[p][r] += 1;
                }
                self.pos[p][r] = to;
                if to >= self.spec.len {
                    self.winner = Some(self.side);
                }
            }
            Mode::Heat => {
                if m.boost {
                    step += 2;
                    self.heat[p] += 1;
                }
                if val <= 2 && self.heat[p] > 0 && !m.boost {
                    self.heat[p] -= 1;
                }
                let mut to = from + step;
                if (0..2).any(|q| q != p && self.pos[q][0] == to + 1) {
                    to += 1;
                }
                if self.crossed_curve(from, to) && step > self.spec.limit as i32 {
                    self.heat[p] += (step - self.spec.limit as i32) as u8;
                }
                self.pos[p][0] = to;
                if self.heat[p] >= 6 {
                    self.out[p] = true;
                    self.winner = Some(self.side.other());
                } else if to >= self.spec.len {
                    self.winner = Some(self.side);
                }
            }
        }
        self.hand[p][r][c] = self.draw(p, r);
        self.side = self.side.other();
        self.turns += 1;
    }

    fn outcome(&self) -> Option<Outcome> {
        if let Some(w) = self.winner {
            return Some(Outcome::Win(w));
        }
        if self.turns >= 200 {
            let a: i32 = self.pos[0].iter().sum();
            let b: i32 = self.pos[1].iter().sum();
            return Some(match a.cmp(&b) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                _ => Outcome::Draw,
            });
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let f = |q: usize| -> i32 {
            let d: i32 = self.pos[q].iter().sum();
            d * 3 - self.heat[q] as i32 * 5 - self.tired[q].iter().map(|&t| t as i32).sum::<i32>()
        };
        f(p.idx()) - f(p.other().idx())
    }

    fn render(&self) -> String {
        let mut s = String::new();
        for q in 0..2 {
            for r in 0..self.riders() {
                s.push_str(&format!(
                    "J{} alergător {}: loc {}/{}  cărți {:?}",
                    q + 1,
                    r + 1,
                    self.pos[q][r],
                    self.spec.len,
                    self.hand[q][r]
                ));
                if self.spec.mode == Mode::Peloton {
                    s.push_str(&format!("  oboseală {}", self.tired[q][r]));
                } else {
                    s.push_str(&format!("  căldură {}/6", self.heat[q]));
                }
                s.push_str(if q == self.side.idx() {
                    "  <- la rând\n"
                } else {
                    "\n"
                });
            }
        }
        if !self.spec.curves.is_empty() {
            s.push_str(&format!(
                "Curbe la {:?}: peste viteza {} primești căldură.\n",
                self.spec.curves, self.spec.limit
            ));
        }
        s.push_str(
            "Mutare: «a1 b» = alergător a, cartea 1 (b = supraturare +2 viteză, +1 căldură).\n",
        );
        s
    }

    fn format_move(&self, m: &RaMove) -> String {
        let who = if self.riders() == 2 {
            ['s', 'r'][m.rider as usize].to_string()
        } else {
            "c".to_string()
        };
        format!("{}{}{}", who, m.card + 1, if m.boost { "+" } else { "" })
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(3)
    }
}

pub fn make(id: &str) -> Box<dyn polytabletop_core::GameObj> {
    Box::new(Race::new(SPECS.iter().find(|s| s.id == id).expect("spec")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_games_end() {
        for spec in SPECS {
            for seed in 1..=15u64 {
                let mut g = Race::with_seed(spec, seed);
                let mut rng = Rng::new(seed);
                while g.outcome().is_none() {
                    let mv = g.legal_moves();
                    assert!(!mv.is_empty());
                    let m = mv[rng.below(mv.len())];
                    g.apply(&m);
                }
            }
        }
    }

    #[test]
    fn ai_beats_random_mostly() {
        polytabletop_core::ai::set_time_budget_override_ms(Some(5));
        for spec in SPECS {
            let mut wins = 0;
            for seed in 1..=6u64 {
                let mut g = Race::with_seed(spec, seed);
                let mut rng = Rng::new(seed + 40);
                while g.outcome().is_none() {
                    if g.side == Player::One {
                        let t = polytabletop_core::GameObj::ai_move(&g, rng.next_u64()).unwrap();
                        polytabletop_core::GameObj::play(&mut g, &t).unwrap();
                    } else {
                        let mv = g.legal_moves();
                        let m = mv[rng.below(mv.len())];
                        g.apply(&m);
                    }
                }
                if g.outcome() == Some(Outcome::Win(Player::One)) {
                    wins += 1;
                }
            }
            assert!(wins >= 4, "{} {wins}/6", spec.id);
        }
    }
}
