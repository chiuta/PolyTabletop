//! Familia „război pe hartă”: 12 teritorii pe o grilă 3×4, doi comandanți.
//! `fronturi_mari`: venit pe valoarea teritoriilor, atac cu pierderi egale, câștigi luând capitala.
//! `tratat_fragil`: fiecare teritoriu e un centru; forța = unități + sprijin de la vecini
//! prieteni; câștigi cu 7 teritorii din 12. Fără zaruri: totul e determinist.
//! Variante originale, inspirate doar de mecanica generală „control de teritorii”.
#![allow(clippy::needless_range_loop, clippy::manual_is_multiple_of)]

use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    Fronts,
    Treaty,
}

pub struct Spec {
    pub id: &'static str,
    pub mode: Mode,
}

pub const SPECS: &[Spec] = &[
    Spec {
        id: "fronturi_mari",
        mode: Mode::Fronts,
    },
    Spec {
        id: "tratat_fragil",
        mode: Mode::Treaty,
    },
];

const N: usize = 12;
const VALUE: [i32; N] = [2, 3, 2, 1, 2, 1, 1, 2, 1, 2, 3, 2];
const CAP: [usize; 2] = [1, 10];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WMove {
    Place(u8),
    Move { from: u8, to: u8, all: bool },
    End,
}

#[derive(Clone)]
pub struct War {
    spec: &'static Spec,
    owner: [Option<Player>; N],
    units: [u8; N],
    side: Player,
    to_place: u8,
    acts: u8,
    turns: u32,
    plies: u32,
    winner: Option<Player>,
}

fn adj(a: usize) -> Vec<usize> {
    let (x, y) = (a % 3, a / 3);
    let mut v = vec![];
    if x > 0 {
        v.push(a - 1);
    }
    if x < 2 {
        v.push(a + 1);
    }
    if y > 0 {
        v.push(a - 3);
    }
    if y < 3 {
        v.push(a + 3);
    }
    v
}

impl War {
    pub fn new(spec: &'static Spec) -> Self {
        let mut owner = [None; N];
        let mut units = [0u8; N];
        for c in 0..3 {
            owner[c] = Some(Player::One);
            owner[9 + c] = Some(Player::Two);
            units[c] = 2;
            units[9 + c] = 2;
        }
        for t in 3..9 {
            units[t] = 1 + (t % 2) as u8;
        }
        let mut g = War {
            spec,
            owner,
            units,
            side: Player::One,
            to_place: 0,
            acts: 3,
            turns: 0,
            plies: 0,
            winner: None,
        };
        g.start_turn();
        g
    }

    fn mine(&self, p: Player) -> impl Iterator<Item = usize> + '_ {
        (0..N).filter(move |&i| self.owner[i] == Some(p))
    }

    fn start_turn(&mut self) {
        self.acts = 3;
        self.to_place = match self.spec.mode {
            Mode::Fronts => (self.mine(self.side).map(|i| VALUE[i]).sum::<i32>() / 2).max(1) as u8,
            Mode::Treaty => {
                let centers = self.mine(self.side).count() as i32;
                let have: i32 = self.mine(self.side).map(|i| self.units[i] as i32).sum();
                (centers - have).clamp(0, 3) as u8
            }
        };
    }

    fn strength(&self, p: Player, target: usize, from: Option<usize>, n: u8) -> i32 {
        match self.spec.mode {
            Mode::Fronts => n as i32,
            Mode::Treaty => {
                let sup = adj(target)
                    .into_iter()
                    .filter(|&a| Some(a) != from && self.owner[a] == Some(p) && self.units[a] >= 2)
                    .count() as i32;
                n as i32 + sup
            }
        }
    }

    fn check_win(&mut self) {
        match self.spec.mode {
            Mode::Fronts => {
                for (k, &c) in CAP.iter().enumerate() {
                    let owner = if k == 0 { Player::One } else { Player::Two };
                    if self.owner[c] != Some(owner) {
                        self.winner = Some(owner.other());
                    }
                }
            }
            Mode::Treaty => {
                for p in [Player::One, Player::Two] {
                    if self.mine(p).count() >= 7 {
                        self.winner = Some(p);
                    }
                }
            }
        }
    }

    fn score(&self, p: Player) -> i32 {
        self.mine(p)
            .map(|i| VALUE[i] * 3 + self.units[i] as i32)
            .sum()
    }
}

impl Game for War {
    type Move = WMove;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<WMove> {
        if self.outcome().is_some() {
            return vec![];
        }
        let mut v = Vec::new();
        if self.to_place > 0 {
            let homes: Vec<usize> = self
                .mine(self.side)
                .filter(|&i| match self.spec.mode {
                    Mode::Fronts => true,
                    Mode::Treaty => {
                        if self.side == Player::One {
                            i < 3
                        } else {
                            i >= 9
                        }
                    }
                })
                .collect();
            let homes = if homes.is_empty() {
                self.mine(self.side).collect()
            } else {
                homes
            };
            for i in homes {
                v.push(WMove::Place(i as u8));
            }
            if v.is_empty() {
                v.push(WMove::End);
            }
            return v;
        }
        if self.acts > 0 {
            for a in self.mine(self.side) {
                let u = self.units[a];
                if u < 2 {
                    continue;
                }
                for b in adj(a) {
                    if self.spec.mode == Mode::Treaty {
                        v.push(WMove::Move {
                            from: a as u8,
                            to: b as u8,
                            all: false,
                        });
                        continue;
                    }
                    v.push(WMove::Move {
                        from: a as u8,
                        to: b as u8,
                        all: true,
                    });
                    if u >= 3 {
                        v.push(WMove::Move {
                            from: a as u8,
                            to: b as u8,
                            all: false,
                        });
                    }
                }
            }
        }
        v.push(WMove::End);
        v
    }

    fn apply(&mut self, m: &WMove) {
        self.plies += 1;
        match *m {
            WMove::Place(t) => {
                self.units[t as usize] += self.to_place;
                self.to_place = 0;
            }
            WMove::End => {
                self.side = self.side.other();
                self.turns += 1;
                self.start_turn();
            }
            WMove::Move { from, to, all } => {
                let (a, b) = (from as usize, to as usize);
                let n = if all { self.units[a] - 1 } else { 1 };
                self.units[a] -= n;
                self.acts -= 1;
                let me = self.side;
                if self.owner[b] == Some(me) {
                    self.units[b] += n;
                } else {
                    let d = self.units[b];
                    let att = self.strength(me, b, Some(a), n);
                    let def = match self.spec.mode {
                        Mode::Fronts => d as i32,
                        Mode::Treaty => match self.owner[b] {
                            Some(o) => self.strength(o, b, None, d),
                            None => d as i32,
                        },
                    };
                    match self.spec.mode {
                        Mode::Fronts => {
                            if att > def {
                                self.owner[b] = Some(me);
                                self.units[b] = (att - def) as u8;
                            } else {
                                self.units[b] = (def - att) as u8;
                                if self.units[b] == 0 {
                                    self.units[b] = 1;
                                }
                            }
                        }
                        Mode::Treaty => {
                            if att > def {
                                self.owner[b] = Some(me);
                                self.units[b] = n;
                            } else if att < def {
                                // atacatorul pierde unitatea; apărătorul rămâne
                            } else {
                                self.units[a] += n; // egalitate: respins
                            }
                        }
                    }
                }
                if self.owner[a] == Some(me) && self.units[a] == 0 {
                    self.units[a] = 1;
                }
                self.check_win();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if let Some(w) = self.winner {
            return Some(Outcome::Win(w));
        }
        if self.turns >= 60 || self.plies >= 700 {
            let (a, b) = (self.score(Player::One), self.score(Player::Two));
            return Some(match a.cmp(&b) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                _ => Outcome::Draw,
            });
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        self.score(p) - self.score(p.other())
    }

    fn render(&self) -> String {
        let mut s = String::new();
        for y in 0..4 {
            for x in 0..3 {
                let i = y * 3 + x;
                let o = match self.owner[i] {
                    Some(Player::One) => "J1",
                    Some(Player::Two) => "J2",
                    None => "--",
                };
                s.push_str(&format!(
                    "[{:>2} {} u{} v{}] ",
                    i + 1,
                    o,
                    self.units[i],
                    VALUE[i]
                ));
            }
            s.push('\n');
        }
        s.push_str(&format!(
            "La rând: J{}; de plasat {}; acțiuni rămase {}; scor J1 {} / J2 {}\n",
            self.side.idx() + 1,
            self.to_place,
            self.acts,
            self.score(Player::One),
            self.score(Player::Two)
        ));
        s.push_str("Mutări: «pun 4», «mut 4 5 toti|unu», «gata».\n");
        s
    }

    fn format_move(&self, m: &WMove) -> String {
        match *m {
            WMove::Place(t) => format!("pun {}", t + 1),
            WMove::End => "gata".into(),
            WMove::Move { from, to, all } => format!(
                "mut {} {} {}",
                from + 1,
                to + 1,
                if self.spec.mode == Mode::Treaty {
                    "unu"
                } else if all {
                    "toti"
                } else {
                    "unu"
                }
            ),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(2)
    }
}

pub fn make(id: &str) -> Box<dyn polytabletop_core::GameObj> {
    Box::new(War::new(SPECS.iter().find(|s| s.id == id).expect("spec")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    #[test]
    fn random_games_end() {
        for spec in SPECS {
            for seed in 1..=10u64 {
                let mut g = War::new(spec);
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
    fn ai_moves_are_legal() {
        polytabletop_core::ai::set_time_budget_override_ms(Some(5));
        for spec in SPECS {
            let mut g = War::new(spec);
            let mut rng = Rng::new(3);
            let mut n = 0;
            while g.outcome().is_none() && n < 120 {
                let t = polytabletop_core::GameObj::ai_move(&g, rng.next_u64()).unwrap();
                assert!(polytabletop_core::GameObj::legal_moves(&g).contains(&t));
                polytabletop_core::GameObj::play(&mut g, &t).unwrap();
                n += 1;
            }
        }
    }
}
