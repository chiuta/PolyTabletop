//! Târgul Mării: negustori în 8 porturi pe un inel; fiecare port produce o marfă (port i → marfa i mod 4,
//! preț de vânzare 1,2,3,4). Fiecare jucător are 4 cărți de acțiune: FONDEZ (pui o casă într-un port
//! liber, vecin cu una a ta; prima oriunde; costă 2 monede, prima gratis), PRODUC (fiecare casă dă o
//! unitate din marfa portului, maxim 6 din fiecare), VÂND (vinzi toate mărfurile; prețul mărfii × cât de multe
//! case ai în acel fel de port) și RECUPEREZ (cărțile jucate se întorc în mână, +1 monedă). O carte
//! jucată nu mai e disponibilă până la recuperare. Când un jucător și-a pus 6 case, runda finală
//! se încheie și se numără: 3 puncte/casă, +1 pe fel de marfă din care ai o casă, +1 la 3 monede.
//! Variantă originală.
#![allow(clippy::needless_range_loop, clippy::manual_is_multiple_of)]

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const PORTS: usize = 8;
const PRICE: [i32; 4] = [1, 2, 3, 4];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mv {
    Found(u8),
    Produce,
    Sell,
    Recall,
}

#[derive(Clone)]
pub struct Targ {
    house: [[bool; PORTS]; 2],
    goods: [[u8; 4]; 2],
    coins: [i32; 2],
    avail: [[bool; 4]; 2],
    side: Player,
    ending: bool,
    turns: u32,
}

impl Default for Targ {
    fn default() -> Self {
        Self::new()
    }
}

impl Targ {
    pub fn new() -> Self {
        Targ {
            house: [[false; PORTS]; 2],
            goods: [[0; 4]; 2],
            coins: [3; 2],
            avail: [[true; 4]; 2],
            side: Player::One,
            ending: false,
            turns: 0,
        }
    }

    fn houses(&self, p: usize) -> usize {
        self.house[p].iter().filter(|&&x| x).count()
    }

    fn kinds(&self, p: usize) -> i32 {
        (0..4)
            .filter(|&g| (0..PORTS).any(|i| self.house[p][i] && i % 4 == g))
            .count() as i32
    }

    fn score(&self, p: usize) -> i32 {
        self.houses(p) as i32 * 3 + self.kinds(p) + self.coins[p] / 3
    }

    fn free_ports(&self, p: usize) -> Vec<usize> {
        let first = self.houses(p) == 0;
        (0..PORTS)
            .filter(|&i| {
                !self.house[0][i]
                    && !self.house[1][i]
                    && (first
                        || self.house[p][(i + 1) % PORTS]
                        || self.house[p][(i + PORTS - 1) % PORTS])
            })
            .collect()
    }
}

impl Game for Targ {
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
        if self.avail[p][0] && (self.houses(p) == 0 || self.coins[p] >= 2) && self.houses(p) < 6 {
            for i in self.free_ports(p) {
                v.push(Mv::Found(i as u8));
            }
        }
        if self.avail[p][1] && self.houses(p) > 0 {
            v.push(Mv::Produce);
        }
        if self.avail[p][2] && self.goods[p].iter().any(|&g| g > 0) {
            v.push(Mv::Sell);
        }
        if self.avail[p].iter().any(|&a| !a) {
            v.push(Mv::Recall);
        }
        if v.is_empty() {
            v.push(Mv::Recall);
        }
        v
    }

    fn apply(&mut self, m: &Mv) {
        let p = self.side.idx();
        match *m {
            Mv::Found(i) => {
                if self.houses(p) > 0 {
                    self.coins[p] -= 2;
                }
                self.house[p][i as usize] = true;
                self.avail[p][0] = false;
                if self.houses(p) >= 6 {
                    self.ending = true;
                }
            }
            Mv::Produce => {
                for i in 0..PORTS {
                    if self.house[p][i] {
                        let g = &mut self.goods[p][i % 4];
                        *g = (*g + 1).min(6);
                    }
                }
                self.avail[p][1] = false;
            }
            Mv::Sell => {
                for g in 0..4 {
                    let n = (0..PORTS)
                        .filter(|&i| self.house[p][i] && i % 4 == g)
                        .count() as i32;
                    self.coins[p] += self.goods[p][g] as i32 * PRICE[g] * n.max(1);
                    self.goods[p][g] = 0;
                }
                self.avail[p][2] = false;
            }
            Mv::Recall => {
                self.avail[p] = [true; 4];
                self.coins[p] += 1;
            }
        }
        self.side = self.side.other();
        self.turns += 1;
    }

    fn outcome(&self) -> Option<Outcome> {
        let over = self.turns >= 200 || (self.ending && self.turns % 2 == 0);
        if !over {
            return None;
        }
        Some(match self.score(0).cmp(&self.score(1)) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            _ => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let f = |q: usize| {
            self.score(q) * 3
                + self.goods[q]
                    .iter()
                    .zip(PRICE)
                    .map(|(&g, pr)| g as i32 * pr)
                    .sum::<i32>()
                    / 2
                + self.coins[q] / 2
        };
        f(p.idx()) - f(p.other().idx())
    }

    fn render(&self) -> String {
        let mut s = String::from("Porturi (marfă): ");
        for i in 0..PORTS {
            s.push_str(&format!(
                "{}[m{} {}] ",
                i + 1,
                i % 4 + 1,
                if self.house[0][i] {
                    "J1"
                } else if self.house[1][i] {
                    "J2"
                } else {
                    ".."
                }
            ));
        }
        s.push('\n');
        for q in 0..2 {
            s.push_str(&format!(
                "J{}{}: monede {}, mărfuri {:?}, case {}/6, scor {}, cărți libere {:?}\n",
                q + 1,
                if self.side.idx() == q {
                    " (la rând)"
                } else {
                    ""
                },
                self.coins[q],
                self.goods[q],
                self.houses(q),
                self.score(q),
                self.avail[q]
            ));
        }
        s.push_str("Mutări: «fondez k», «produc», «vand», «recuperez».\n");
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        match *m {
            Mv::Found(i) => format!("fondez {}", i + 1),
            Mv::Produce => "produc".into(),
            Mv::Sell => "vand".into(),
            Mv::Recall => "recuperez".into(),
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
            let mut g = Targ::new();
            let mut rng = Rng::new(seed);
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
                assert!(g.coins.iter().all(|&c| c >= 0));
            }
        }
    }

    #[test]
    fn ai_beats_random() {
        polytabletop_core::ai::set_time_budget_override_ms(Some(5));
        let mut wins = 0;
        for seed in 1..=6u64 {
            let mut g = Targ::new();
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
