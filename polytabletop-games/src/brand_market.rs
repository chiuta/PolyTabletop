//! Familia „piață”: strângi resurse și cumperi cărți din ofertă; unele cărți dau reduceri
//! permanente sau venit pe tură. Variante originale ale mecanicii generale „motor de resurse”.
//! Cărțile noi sunt generate la cerere cu un zar re-însămânțat la `clone()` (AI-ul nu le vede).
#![allow(clippy::needless_range_loop, clippy::manual_is_multiple_of)]

use crate::w7_util::Dice;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Take {
    /// 3 resurse de feluri diferite (sau mai puține, dacă nu mai ai loc)
    Three,
    /// 2 resurse de același fel
    Two,
    /// o singură resursă, la alegere, plus una aleasă la întâmplare de zar
    OneLuck,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Bonus {
    None,
    Discount,
    Income,
}

pub struct Spec {
    pub id: &'static str,
    pub kinds: usize,
    pub offer: usize,
    pub target: i32,
    pub take: Take,
    pub bonus: Bonus,
    pub swap: bool,
    pub limit: u8,
    pub max_cost: u8,
}

pub const SPECS: &[Spec] = &[
    Spec {
        id: "duel_giuvaeruri",
        kinds: 4,
        offer: 4,
        target: 14,
        take: Take::Three,
        bonus: Bonus::Discount,
        swap: false,
        limit: 10,
        max_cost: 3,
    },
    Spec {
        id: "atelier_mecanisme",
        kinds: 4,
        offer: 4,
        target: 12,
        take: Take::OneLuck,
        bonus: Bonus::Discount,
        swap: false,
        limit: 8,
        max_cost: 3,
    },
    Spec {
        id: "caravana_aromelor",
        kinds: 4,
        offer: 5,
        target: 16,
        take: Take::Two,
        bonus: Bonus::None,
        swap: true,
        limit: 10,
        max_cost: 4,
    },
    Spec {
        id: "golemii_atelier",
        kinds: 5,
        offer: 5,
        target: 18,
        take: Take::Two,
        bonus: Bonus::None,
        swap: true,
        limit: 12,
        max_cost: 4,
    },
    Spec {
        id: "lampioane_lac",
        kinds: 4,
        offer: 4,
        target: 13,
        take: Take::OneLuck,
        bonus: Bonus::None,
        swap: true,
        limit: 9,
        max_cost: 3,
    },
    Spec {
        id: "rivalii_colonizarii",
        kinds: 5,
        offer: 4,
        target: 12,
        take: Take::Three,
        bonus: Bonus::Income,
        swap: false,
        limit: 10,
        max_cost: 3,
    },
    Spec {
        id: "ferma_animalelor",
        kinds: 4,
        offer: 4,
        target: 14,
        take: Take::Two,
        bonus: Bonus::Income,
        swap: false,
        limit: 10,
        max_cost: 3,
    },
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Card {
    cost: [u8; 5],
    vp: u8,
    bonus: u8,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MMove {
    Take3(u8, u8, u8),
    Take2(u8),
    Take1(u8),
    Buy(u8),
    Swap(u8),
    Drop(u8),
}

#[derive(Clone)]
pub struct Market {
    spec: &'static Spec,
    stock: [[u8; 5]; 2],
    bonus: [[u8; 5]; 2],
    vp: [i32; 2],
    bought: [u32; 2],
    offer: Vec<Card>,
    side: Player,
    turns: u32,
    dice: Dice,
}

const SYM: [char; 5] = ['R', 'V', 'A', 'G', 'N'];

impl Market {
    pub fn new(spec: &'static Spec) -> Self {
        Self::with_seed(spec, Rng::from_clock().next_u64())
    }

    pub fn with_seed(spec: &'static Spec, seed: u64) -> Self {
        let mut m = Market {
            spec,
            stock: [[0; 5]; 2],
            bonus: [[0; 5]; 2],
            vp: [0; 2],
            bought: [0; 2],
            offer: vec![],
            side: Player::One,
            turns: 0,
            dice: Dice::new(seed),
        };
        for _ in 0..spec.offer {
            let c = m.gen_card();
            m.offer.push(c);
        }
        m
    }

    fn gen_card(&mut self) -> Card {
        let k = self.spec.kinds;
        let mut cost = [0u8; 5];
        let kinds_used = 1 + self.dice.below(2.min(k));
        let mut total = 0u32;
        for _ in 0..kinds_used {
            let i = self.dice.below(k);
            let n = 1 + self.dice.below(self.spec.max_cost as usize) as u8;
            cost[i] += n;
            total += n as u32;
        }
        let vp = (total as i32 - 2 + self.dice.below(4) as i32).clamp(0, 6) as u8;
        let bonus = if self.spec.bonus != Bonus::None && self.dice.below(10) < 6 {
            self.dice.below(k) as u8
        } else {
            255
        };
        Card { cost, vp, bonus }
    }

    fn total(&self, p: usize) -> u8 {
        self.stock[p][..self.spec.kinds].iter().sum()
    }

    fn price(&self, p: usize, c: &Card) -> Option<[u8; 5]> {
        let mut pay = [0u8; 5];
        for i in 0..self.spec.kinds {
            let disc = if self.spec.bonus == Bonus::Discount {
                self.bonus[p][i]
            } else {
                0
            };
            let need = c.cost[i].saturating_sub(disc);
            if self.stock[p][i] < need {
                return None;
            }
            pay[i] = need;
        }
        Some(pay)
    }
}

impl Game for Market {
    type Move = MMove;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<MMove> {
        if self.outcome().is_some() {
            return vec![];
        }
        let p = self.side.idx();
        let k = self.spec.kinds as u8;
        let room = self.spec.limit.saturating_sub(self.total(p));
        let mut v = Vec::new();
        for (i, c) in self.offer.iter().enumerate() {
            if self.price(p, c).is_some() {
                v.push(MMove::Buy(i as u8));
            }
        }
        if room > 0 {
            match self.spec.take {
                Take::Three => {
                    for a in 0..k {
                        for b in a + 1..k {
                            for c in b + 1..k {
                                v.push(MMove::Take3(a, b, c));
                            }
                        }
                    }
                }
                Take::Two => {
                    if room >= 2 {
                        v.extend((0..k).map(MMove::Take2));
                    } else {
                        v.extend((0..k).map(MMove::Take1));
                    }
                }
                Take::OneLuck => v.extend((0..k).map(MMove::Take1)),
            }
        }
        if self.spec.swap {
            for a in 0..k - 1 {
                if self.stock[p][a as usize] >= 2 {
                    v.push(MMove::Swap(a));
                }
            }
        }
        if v.is_empty() {
            for a in 0..k {
                if self.stock[p][a as usize] > 0 {
                    v.push(MMove::Drop(a));
                }
            }
        }
        if v.is_empty() {
            v.push(MMove::Drop(0));
        }
        v
    }

    fn apply(&mut self, m: &MMove) {
        let p = self.side.idx();
        let lim = self.spec.limit;
        match *m {
            MMove::Take3(a, b, c) => {
                for x in [a, b, c] {
                    if self.total(p) < lim {
                        self.stock[p][x as usize] += 1;
                    }
                }
            }
            MMove::Take2(a) => self.stock[p][a as usize] += 2,
            MMove::Take1(a) => {
                self.stock[p][a as usize] += 1;
                if self.spec.take == Take::OneLuck && self.total(p) < lim {
                    let r = self.dice.below(self.spec.kinds);
                    self.stock[p][r] += 1;
                }
            }
            MMove::Swap(a) => {
                self.stock[p][a as usize] -= 2;
                self.stock[p][a as usize + 1] += 1;
            }
            MMove::Drop(a) => {
                if self.stock[p][a as usize] > 0 {
                    self.stock[p][a as usize] -= 1;
                }
            }
            MMove::Buy(i) => {
                let c = self.offer[i as usize];
                if let Some(pay) = self.price(p, &c) {
                    for j in 0..5 {
                        self.stock[p][j] -= pay[j];
                    }
                }
                self.vp[p] += c.vp as i32;
                self.bought[p] += 1;
                if c.bonus != 255 {
                    self.bonus[p][c.bonus as usize] += 1;
                }
                self.offer[i as usize] = self.gen_card();
            }
        }
        if self.spec.bonus == Bonus::Income && !matches!(m, MMove::Buy(_)) {
            for j in 0..self.spec.kinds {
                let add = self.bonus[p][j].min(2);
                let room = lim.saturating_sub(self.total(p));
                self.stock[p][j] += add.min(room);
            }
        }
        self.side = self.side.other();
        self.turns += 1;
    }

    fn outcome(&self) -> Option<Outcome> {
        let done = self.turns >= 160
            || (self.turns % 2 == 0
                && self.turns > 0
                && self.vp.iter().any(|&v| v >= self.spec.target));
        if !done {
            return None;
        }
        Some(match self.vp[0].cmp(&self.vp[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            _ => match self.bought[0].cmp(&self.bought[1]) {
                std::cmp::Ordering::Less => Outcome::Win(Player::One),
                std::cmp::Ordering::Greater => Outcome::Win(Player::Two),
                _ => Outcome::Draw,
            },
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let side = |q: usize| {
            self.vp[q] * 10
                + self.stock[q].iter().map(|&x| x as i32).sum::<i32>()
                + self.bonus[q].iter().map(|&x| x as i32).sum::<i32>() * 4
        };
        side(p.idx()) - side(p.other().idx())
    }

    fn render(&self) -> String {
        let k = self.spec.kinds;
        let mut s = String::new();
        for q in 0..2 {
            s.push_str(&format!(
                "Jucătorul {}{}: {} puncte; resurse ",
                q + 1,
                if self.side.idx() == q {
                    " (la rând)"
                } else {
                    ""
                },
                self.vp[q]
            ));
            for i in 0..k {
                s.push_str(&format!("{}{}", SYM[i], self.stock[q][i]));
                if self.bonus[q][i] > 0 {
                    s.push_str(&format!("(+{})", self.bonus[q][i]));
                }
                s.push(' ');
            }
            s.push('\n');
        }
        s.push_str(&format!("Ținta: {} puncte. Oferta:\n", self.spec.target));
        for (i, c) in self.offer.iter().enumerate() {
            s.push_str(&format!("  {}: cost ", i + 1));
            for j in 0..k {
                if c.cost[j] > 0 {
                    s.push_str(&format!("{}{} ", SYM[j], c.cost[j]));
                }
            }
            s.push_str(&format!("-> {}p", c.vp));
            if c.bonus != 255 {
                s.push_str(&format!(
                    " {}{}",
                    if self.spec.bonus == Bonus::Income {
                        "venit "
                    } else {
                        "reducere "
                    },
                    SYM[c.bonus as usize]
                ));
            }
            s.push('\n');
        }
        s
    }

    fn format_move(&self, m: &MMove) -> String {
        match *m {
            MMove::Take3(a, b, c) => format!(
                "ia {}{}{}",
                SYM[a as usize].to_ascii_lowercase(),
                SYM[b as usize].to_ascii_lowercase(),
                SYM[c as usize].to_ascii_lowercase()
            ),
            MMove::Take2(a) => format!("ia2 {}", SYM[a as usize].to_ascii_lowercase()),
            MMove::Take1(a) => format!("ia1 {}", SYM[a as usize].to_ascii_lowercase()),
            MMove::Buy(i) => format!("cumpar {}", i + 1),
            MMove::Swap(a) => format!("schimb {}", SYM[a as usize].to_ascii_lowercase()),
            MMove::Drop(a) => format!("las {}", SYM[a as usize].to_ascii_lowercase()),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(2)
    }
}

pub fn make(id: &str) -> Box<dyn polytabletop_core::GameObj> {
    Box::new(Market::new(
        SPECS.iter().find(|s| s.id == id).expect("spec"),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_games_end() {
        for spec in SPECS {
            for seed in 1..=10u64 {
                let mut g = Market::with_seed(spec, seed);
                let mut rng = Rng::new(seed);
                let mut n = 0;
                while g.outcome().is_none() {
                    let mv = g.legal_moves();
                    assert!(!mv.is_empty());
                    let m = mv[rng.below(mv.len())];
                    g.apply(&m);
                    n += 1;
                    assert!(n < 400, "{}", spec.id);
                }
            }
        }
    }

    #[test]
    fn ai_does_not_cheat_and_beats_random() {
        polytabletop_core::ai::set_time_budget_override_ms(Some(5));
        let (mut wins, mut games) = (0, 0);
        for spec in SPECS {
            for seed in 1..=6u64 {
                let mut g = Market::with_seed(spec, seed);
                let mut rng = Rng::new(seed + 7);
                while g.outcome().is_none() {
                    if g.side == Player::One {
                        let t = polytabletop_core::GameObj::ai_move(&g, rng.next_u64()).unwrap();
                        assert!(polytabletop_core::GameObj::legal_moves(&g).contains(&t));
                        polytabletop_core::GameObj::play(&mut g, &t).unwrap();
                    } else {
                        let mv = g.legal_moves();
                        let m = mv[rng.below(mv.len())];
                        g.apply(&m);
                    }
                }
                games += 1;
                if g.outcome() == Some(Outcome::Win(Player::One)) {
                    wins += 1;
                }
            }
        }
        eprintln!("market: AI {wins}/{games}");
        assert!(wins * 10 >= games * 7, "{wins}/{games}");
    }
}
