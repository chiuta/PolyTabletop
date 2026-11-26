//! Debarcaderul Norocos: joc de risc la un port. La rând tragi bărci una câte una («trag»): fiecare
//! barcă are un pavilion (4 feluri) și o încărcătură de 1–3 monede. Dacă apare al doilea pavilion
//! de același fel în tura asta, barca se scufundă: pierzi tot ce ai tras și tura se termină. Poți
//! oricând, după cel puțin o barcă, să te oprești («opresc») și să încasezi încărcăturile plus
//! 1 monedă pentru fiecare dulgher angajat. La începutul turei, cu 6 monede, poți angaja un dulgher
//! («angajez»: +1 influență). Câștigă primul cu 4 influențe; după 120 de ture decide influența, apoi banii.
//! Variantă originală. Cărțile sunt trase la întâmplare cu un zar re-însămânțat la `clone()`.
#![allow(clippy::needless_range_loop, clippy::manual_is_multiple_of)]

use crate::w7_util::Dice;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mv {
    Draw,
    Stop,
    Hire,
}

#[derive(Clone)]
pub struct Debarcader {
    coins: [i32; 2],
    infl: [i32; 2],
    seen: [bool; 4],
    haul: i32,
    drawn: u8,
    last: Option<(u8, u8)>,
    busted: bool,
    side: Player,
    turns: u32,
    dice: Dice,
}

impl Default for Debarcader {
    fn default() -> Self {
        Self::new()
    }
}

impl Debarcader {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Debarcader {
            coins: [0; 2],
            infl: [0; 2],
            seen: [false; 4],
            haul: 0,
            drawn: 0,
            last: None,
            busted: false,
            side: Player::One,
            turns: 0,
            dice: Dice::new(seed),
        }
    }

    fn end_turn(&mut self) {
        self.seen = [false; 4];
        self.haul = 0;
        self.drawn = 0;
        self.busted = false;
        self.side = self.side.other();
        self.turns += 1;
    }
}

impl Game for Debarcader {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if self.outcome().is_some() {
            return vec![];
        }
        let mut v = vec![Mv::Draw];
        if self.drawn > 0 {
            v.push(Mv::Stop);
        } else if self.coins[self.side.idx()] >= 6 {
            v.push(Mv::Hire);
        }
        v
    }

    fn apply(&mut self, m: &Mv) {
        let p = self.side.idx();
        match *m {
            Mv::Hire => {
                self.coins[p] -= 6;
                self.infl[p] += 1;
            }
            Mv::Stop => {
                self.coins[p] += self.haul + self.infl[p];
                self.last = None;
                self.end_turn();
            }
            Mv::Draw => {
                let suit = self.dice.below(4) as u8;
                let val = 1 + self.dice.below(3) as u8;
                self.last = Some((suit, val));
                if self.seen[suit as usize] {
                    self.busted = true;
                    self.end_turn();
                } else {
                    self.seen[suit as usize] = true;
                    self.haul += val as i32;
                    self.drawn += 1;
                    if self.drawn == 4 {
                        // toate pavilioanele diferite: oprire forțată cu bonus
                        self.coins[p] += self.haul + self.infl[p] + 2;
                        self.end_turn();
                    }
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        for p in [Player::One, Player::Two] {
            if self.infl[p.idx()] >= 4 {
                return Some(Outcome::Win(p));
            }
        }
        if self.turns >= 120 {
            let f = |p: usize| (self.infl[p], self.coins[p]);
            return Some(match f(0).cmp(&f(1)) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                _ => Outcome::Draw,
            });
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let f = |q: usize| self.infl[q] * 7 + self.coins[q];
        let mine = f(p.idx()) + if self.side == p { self.haul } else { 0 };
        mine - f(p.other().idx()) - if self.side != p { self.haul } else { 0 }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::Mcts(400)
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Tura {} | J1: {} monede, {} influență | J2: {} monede, {} influență | la rând J{}\n",
            self.turns + 1,
            self.coins[0],
            self.infl[0],
            self.coins[1],
            self.infl[1],
            self.side.idx() + 1
        );
        s.push_str(&format!(
            "Pe cheu: {} bărci, încărcătură {}, pavilioane văzute {:?}",
            self.drawn, self.haul, self.seen
        ));
        if let Some((a, b)) = self.last {
            s.push_str(&format!(" | ultima: pavilion {} încărcătură {}", a + 1, b));
        }
        s.push_str("\nMutări: «trag», «opresc», «angajez».\n");
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        match m {
            Mv::Draw => "trag",
            Mv::Stop => "opresc",
            Mv::Hire => "angajez",
        }
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_games_end() {
        for seed in 1..=30u64 {
            let mut g = Debarcader::with_seed(seed);
            let mut rng = Rng::new(seed);
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
            }
        }
    }

    #[test]
    fn ai_legal_and_hires() {
        polytabletop_core::ai::set_time_budget_override_ms(Some(5));
        let mut g = Debarcader::with_seed(5);
        let mut rng = Rng::new(5);
        let mut n = 0;
        while g.outcome().is_none() && n < 400 {
            let t = polytabletop_core::GameObj::ai_move(&g, rng.next_u64()).unwrap();
            assert!(polytabletop_core::GameObj::legal_moves(&g).contains(&t));
            polytabletop_core::GameObj::play(&mut g, &t).unwrap();
            n += 1;
        }
        assert!(g.infl[0] + g.infl[1] > 0);
    }
}
