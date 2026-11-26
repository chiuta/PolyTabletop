//! Poteca Turmelor: doi crescători duc vite spre depozitul de la capătul potecii (12 trepte).
//! Pe rând faci O acțiune: «merg n» (1–3 trepte; dacă aterizezi pe treapta adversarului îi plătești
//! 1 monedă), «iau r» (cumperi o vită din rasa r = 1..4 cu 2 monede, doar pe treptele-piață 3, 6, 9),
//! sau «lucrez» (+2 monede). Ajuns la depozit (treapta 12) livrezi automat: 3 puncte pentru fiecare rasă
//! diferită + 1 pentru fiecare vită, +3 monede, și te întorci la start. După 3 livrări câștigi dacă
//! ai mai multe puncte (la egalitate, mai mulți bani). Variantă originală, determinist.
#![allow(clippy::needless_range_loop, clippy::manual_is_multiple_of)]

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const END: u8 = 12;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mv {
    Walk(u8),
    Buy(u8),
    Work,
}

#[derive(Clone)]
pub struct Poteca {
    pos: [u8; 2],
    herd: [[u8; 4]; 2],
    money: [i32; 2],
    points: [i32; 2],
    deliveries: [u8; 2],
    side: Player,
    turns: u32,
}

impl Default for Poteca {
    fn default() -> Self {
        Self::new()
    }
}

impl Poteca {
    pub fn new() -> Self {
        Poteca {
            pos: [0; 2],
            herd: [[0; 4]; 2],
            money: [4; 2],
            points: [0; 2],
            deliveries: [0; 2],
            side: Player::One,
            turns: 0,
        }
    }

    fn herd_value(&self, p: usize) -> i32 {
        let h = &self.herd[p];
        h.iter().filter(|&&x| x > 0).count() as i32 * 3 + h.iter().map(|&x| x as i32).sum::<i32>()
    }
}

impl Game for Poteca {
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
        for n in 1..=3 {
            if self.pos[p] + n <= END {
                v.push(Mv::Walk(n));
            }
        }
        if self.money[p] >= 2 && [3, 6, 9].contains(&self.pos[p]) {
            for r in 0..4 {
                v.push(Mv::Buy(r));
            }
        }
        v.push(Mv::Work);
        v
    }

    fn apply(&mut self, m: &Mv) {
        let p = self.side.idx();
        match *m {
            Mv::Walk(n) => {
                self.pos[p] += n;
                let at = self.pos[p];
                if at < END && self.pos[1 - p] == at {
                    let pay = self.money[p].min(1);
                    self.money[p] -= pay;
                    self.money[1 - p] += pay;
                }
                if at == END {
                    self.points[p] += self.herd_value(p);
                    self.money[p] += 3;
                    self.herd[p] = [0; 4];
                    self.pos[p] = 0;
                    self.deliveries[p] += 1;
                }
            }
            Mv::Buy(r) => {
                self.money[p] -= 2;
                self.herd[p][r as usize] += 1;
            }
            Mv::Work => self.money[p] += 2,
        }
        self.side = self.side.other();
        self.turns += 1;
    }

    fn outcome(&self) -> Option<Outcome> {
        let done = self.deliveries.iter().all(|&d| d >= 3) || self.turns >= 300;
        if !done {
            return None;
        }
        Some(
            match (self.points[0], self.money[0]).cmp(&(self.points[1], self.money[1])) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                _ => Outcome::Draw,
            },
        )
    }

    fn evaluate(&self, p: Player) -> i32 {
        let f = |q: usize| {
            self.points[q] * 3
                + self.herd_value(q) * 2
                + self.money[q]
                + self.pos[q] as i32 / 2
                + self.deliveries[q] as i32 * 2
        };
        f(p.idx()) - f(p.other().idx())
    }

    fn render(&self) -> String {
        let mut s = String::new();
        for q in 0..2 {
            s.push_str(&format!(
                "J{}{}: treapta {}/12, bani {}, puncte {}, livrări {}/3, turmă {:?}\n",
                q + 1,
                if self.side.idx() == q {
                    " (la rând)"
                } else {
                    ""
                },
                self.pos[q],
                self.money[q],
                self.points[q],
                self.deliveries[q],
                self.herd[q]
            ));
        }
        s.push_str("Piețe la treptele 3, 6, 9. Mutări: «merg 2», «iau 1», «lucrez».\n");
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        match *m {
            Mv::Walk(n) => format!("merg {n}"),
            Mv::Buy(r) => format!("iau {}", r + 1),
            Mv::Work => "lucrez".into(),
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
            let mut g = Poteca::new();
            let mut rng = Rng::new(seed);
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
            }
        }
    }

    #[test]
    fn ai_beats_random() {
        polytabletop_core::ai::set_time_budget_override_ms(Some(5));
        let mut wins = 0;
        for seed in 1..=6u64 {
            let mut g = Poteca::new();
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
