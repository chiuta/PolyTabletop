//! Planetele Verzi: doi coloniști terraformează 8 planete de 7 tipuri (inel de tipuri 0..6).
//! Fiecare are un tip-casă. «colonizez k» costă (1 + distanța pe inel dintre tipul planetei și tipul-casă
//! minus nivelul de terraformare) minereu și 2 credite; prima planetă e liberă oriunde, următoarele
//! trebuie să fie vecine cu una a ta (inel de planete). «cercetez» (3 cunoștințe) urcă nivelul (max 3).
//! «transform» schimbă 2 minereu în 1 credit. «colectez» ia venitul. Venit de bază la fiecare mutare
//! (indiferent de acțiune): +1 minereu pe planetă, +1 cunoștință la 2 planete, +1 credit. Câștigă, după 14
//! mutări fiecare, cine are mai multe puncte: 4/planetă, 5/nivel, 1 la 2 cunoștințe. Variantă originală.
#![allow(clippy::needless_range_loop, clippy::manual_is_multiple_of)]

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const P: usize = 8;
const TYPES: [u8; P] = [0, 2, 4, 6, 1, 3, 5, 0];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mv {
    Colonize(u8),
    Research,
    Convert,
    Collect,
}

#[derive(Clone)]
pub struct Planete {
    owner: [Option<Player>; P],
    ore: [i32; 2],
    credits: [i32; 2],
    know: [i32; 2],
    level: [i32; 2],
    side: Player,
    turns: u32,
}

impl Default for Planete {
    fn default() -> Self {
        Self::new()
    }
}

fn dist(a: u8, b: u8) -> i32 {
    let d = (a as i32 - b as i32).rem_euclid(7);
    d.min(7 - d)
}

impl Planete {
    pub fn new() -> Self {
        Planete {
            owner: [None; P],
            ore: [4; 2],
            credits: [4; 2],
            know: [0; 2],
            level: [0; 2],
            side: Player::One,
            turns: 0,
        }
    }

    fn home(p: usize) -> u8 {
        if p == 0 {
            0
        } else {
            4
        }
    }

    fn count(&self, p: usize) -> i32 {
        let me = if p == 0 { Player::One } else { Player::Two };
        self.owner.iter().filter(|&&o| o == Some(me)).count() as i32
    }

    fn cost(&self, p: usize, k: usize) -> i32 {
        (1 + dist(TYPES[k], Self::home(p)) - self.level[p]).max(1)
    }

    fn score(&self, p: usize) -> i32 {
        self.count(p) * 4 + self.level[p] * 5 + self.know[p] / 2
    }
}

impl Game for Planete {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if self.outcome().is_some() {
            return vec![];
        }
        let p = self.side.idx();
        let mut v = vec![];
        let first = self.count(p) == 0;
        for k in 0..P {
            if self.owner[k].is_some() {
                continue;
            }
            let near = self.owner[(k + 1) % P] == Some(self.side)
                || self.owner[(k + P - 1) % P] == Some(self.side);
            if (first || near) && self.ore[p] >= self.cost(p, k) && self.credits[p] >= 2 {
                v.push(Mv::Colonize(k as u8));
            }
        }
        if self.know[p] >= 3 && self.level[p] < 3 {
            v.push(Mv::Research);
        }
        if self.ore[p] >= 2 {
            v.push(Mv::Convert);
        }
        v.push(Mv::Collect);
        v
    }

    fn apply(&mut self, m: &Mv) {
        let p = self.side.idx();
        match *m {
            Mv::Colonize(k) => {
                self.ore[p] -= self.cost(p, k as usize);
                self.credits[p] -= 2;
                self.owner[k as usize] = Some(self.side);
            }
            Mv::Research => {
                self.know[p] -= 3;
                self.level[p] += 1;
            }
            Mv::Convert => {
                self.ore[p] -= 2;
                self.credits[p] += 1;
            }
            Mv::Collect => {
                self.ore[p] += 2;
                self.credits[p] += 1;
            }
        }
        self.ore[p] += self.count(p);
        self.know[p] += self.count(p) / 2 + 1;
        self.credits[p] += 1;
        self.side = self.side.other();
        self.turns += 1;
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.turns < 28 {
            return None;
        }
        Some(match self.score(0).cmp(&self.score(1)) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            _ => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let f = |q: usize| self.score(q) * 4 + self.ore[q] + self.credits[q] + self.know[q];
        f(p.idx()) - f(p.other().idx())
    }

    fn render(&self) -> String {
        let mut s = String::from("Planete (tip): ");
        for k in 0..P {
            s.push_str(&format!(
                "{}[t{} {}] ",
                k + 1,
                TYPES[k],
                match self.owner[k] {
                    Some(Player::One) => "J1",
                    Some(Player::Two) => "J2",
                    None => "..",
                }
            ));
        }
        s.push('\n');
        for q in 0..2 {
            s.push_str(&format!(
                "J{}{} (casă t{}): minereu {}, credite {}, cunoștințe {}, nivel {}, scor {}\n",
                q + 1,
                if self.side.idx() == q {
                    " (la rând)"
                } else {
                    ""
                },
                Self::home(q),
                self.ore[q],
                self.credits[q],
                self.know[q],
                self.level[q],
                self.score(q)
            ));
        }
        s.push_str(&format!(
            "Mutarea {}/28. Mutări: «colonizez k», «cercetez», «transform», «colectez».\n",
            self.turns + 1
        ));
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        match *m {
            Mv::Colonize(k) => format!("colonizez {}", k + 1),
            Mv::Research => "cercetez".into(),
            Mv::Convert => "transform".into(),
            Mv::Collect => "colectez".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    #[test]
    fn random_games_end() {
        for seed in 1..=30u64 {
            let mut g = Planete::new();
            let mut rng = Rng::new(seed);
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
                assert!(g.ore.iter().all(|&x| x >= 0) && g.credits.iter().all(|&x| x >= 0));
            }
        }
    }

    #[test]
    fn ai_beats_random() {
        polytabletop_core::ai::set_time_budget_override_ms(Some(5));
        let mut wins = 0;
        for seed in 1..=6u64 {
            let mut g = Planete::new();
            let mut rng = Rng::new(seed);
            while g.outcome().is_none() {
                if g.side == Player::One {
                    let t = polytabletop_core::GameObj::ai_move(&g, rng.next_u64()).unwrap();
                    polytabletop_core::GameObj::play(&mut g, &t).unwrap();
                } else {
                    let mv = g.legal_moves();
                    g.apply(&mv[rng.below(mv.len())]);
                }
            }
            if g.outcome() == Some(Outcome::Win(Player::One)) {
                wins += 1;
            }
        }
        assert!(wins >= 5, "{wins}");
    }
}
