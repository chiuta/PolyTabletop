//! Rețeaua Luminii: duel economic. 8 orașe pe un inel (+2 legături), 4 centrale la vânzare,
//! combustibil la preț variabil. Pe rând faci O acțiune: «centrala i» (cumperi o centrală, ai cel mult 2),
//! «combustibil c|p|e» (cumperi o unitate: cărbune/petrol; «e» = energie verde, nu costă), «oras k»
//! (conectezi orașul k: 5 + costul legăturii cu rețeaua ta, +4 dacă îl are deja adversarul),
//! «produc» (arzi combustibil, alimentezi min(orașe, capacitate) orașe și încasezi 4 + 3 pe oraș)
//! sau «pas». Primul cu 7 orașe conectate declanșează sfârșitul: câștigă cine alimentează mai
//! multe orașe în ultima producție, apoi cine are mai mulți bani. Variantă originală, determinist.
#![allow(clippy::needless_range_loop, clippy::manual_is_multiple_of)]

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const CITIES: usize = 8;
const EDGES: [(u8, u8, i32); 10] = [
    (0, 1, 2),
    (1, 2, 3),
    (2, 3, 2),
    (3, 4, 3),
    (4, 5, 2),
    (5, 6, 3),
    (6, 7, 2),
    (7, 0, 3),
    (0, 4, 5),
    (2, 6, 5),
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Plant {
    cap: u8,
    fuel: u8, // 0 cărbune, 1 petrol, 2 verde
    price: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mv {
    BuyPlant(u8),
    Fuel(u8),
    Connect(u8),
    Produce,
    Pass,
}

#[derive(Clone)]
pub struct Retea {
    money: [i32; 2],
    cities: [[bool; CITIES]; 2],
    plants: [Vec<Plant>; 2],
    fuel: [[u8; 2]; 2],
    supply: [u8; 2],
    market: Vec<Plant>,
    queue: Vec<Plant>,
    side: Player,
    ending: Option<Player>,
    final_power: [i32; 2],
    last_prod: [u32; 2],
    turns: u32,
}

impl Default for Retea {
    fn default() -> Self {
        Self::new()
    }
}

fn plant_list() -> Vec<Plant> {
    let mut v = vec![];
    for (i, &(cap, fuel)) in [
        (1u8, 0u8),
        (2, 0),
        (2, 1),
        (1, 2),
        (3, 0),
        (2, 2),
        (3, 1),
        (4, 0),
        (3, 2),
        (4, 1),
        (5, 0),
        (5, 1),
    ]
    .iter()
    .enumerate()
    {
        v.push(Plant {
            cap,
            fuel,
            price: 3 + i as i32 * 2,
        });
    }
    v
}

impl Retea {
    pub fn new() -> Self {
        let mut queue = plant_list();
        let market: Vec<Plant> = queue.drain(0..4).collect();
        Retea {
            money: [30; 2],
            cities: [[false; CITIES]; 2],
            plants: [vec![], vec![]],
            fuel: [[0; 2]; 2],
            supply: [6, 6],
            market,
            queue,
            side: Player::One,
            ending: None,
            final_power: [0; 2],
            last_prod: [0; 2],
            turns: 0,
        }
    }

    fn fuel_price(&self, f: u8) -> i32 {
        match self.supply[f as usize] {
            0 => 99,
            1..=2 => 4,
            3..=4 => 3,
            _ => 2,
        }
    }

    fn n_cities(&self, p: usize) -> usize {
        self.cities[p].iter().filter(|&&x| x).count()
    }

    fn connect_cost(&self, p: usize, k: usize) -> Option<i32> {
        if self.cities[p][k] {
            return None;
        }
        let base = if self.cities[1 - p][k] { 9 } else { 5 };
        if self.n_cities(p) == 0 {
            return Some(base);
        }
        let mut best = None;
        for &(a, b, c) in EDGES.iter() {
            let (a, b) = (a as usize, b as usize);
            if (a == k && self.cities[p][b]) || (b == k && self.cities[p][a]) {
                best = Some(best.map_or(c, |x: i32| x.min(c)));
            }
        }
        best.map(|c| base + c)
    }

    fn capacity(&self, p: usize) -> i32 {
        self.plants[p].iter().map(|pl| pl.cap as i32).sum()
    }

    fn power(&self, p: usize) -> i32 {
        // câte orașe poți alimenta acum, cu combustibilul din stoc
        let mut left = self.fuel[p];
        let mut cap = 0;
        for pl in &self.plants[p] {
            if pl.fuel == 2 {
                cap += pl.cap as i32;
            } else if left[pl.fuel as usize] >= 1 {
                left[pl.fuel as usize] -= 1;
                cap += pl.cap as i32;
            }
        }
        cap.min(self.n_cities(p) as i32)
    }
}

impl Game for Retea {
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
        for (i, pl) in self.market.iter().enumerate() {
            if pl.price <= self.money[p] && !self.plants[p].contains(pl) {
                v.push(Mv::BuyPlant(i as u8));
            }
        }
        let held: i32 = self.fuel[p].iter().map(|&x| x as i32).sum();
        for f in 0..2u8 {
            let burns = self.plants[p].iter().filter(|pl| pl.fuel == f).count() as i32;
            if burns > 0
                && held < 4
                && self.fuel_price(f) <= self.money[p]
                && self.supply[f as usize] > 0
            {
                v.push(Mv::Fuel(f));
            }
        }
        for k in 0..CITIES {
            if let Some(c) = self.connect_cost(p, k) {
                if c <= self.money[p] {
                    v.push(Mv::Connect(k as u8));
                }
            }
        }
        if self.power(p) > 0 && self.turns >= self.last_prod[p] + 10 {
            v.push(Mv::Produce);
        }
        v.push(Mv::Pass);
        v
    }

    fn apply(&mut self, m: &Mv) {
        let p = self.side.idx();
        match *m {
            Mv::BuyPlant(i) => {
                let pl = self.market.remove(i as usize);
                self.money[p] -= pl.price;
                if self.plants[p].len() == 2 {
                    let worst = if self.plants[p][0].cap <= self.plants[p][1].cap {
                        0
                    } else {
                        1
                    };
                    self.plants[p].remove(worst);
                }
                self.plants[p].push(pl);
                if !self.queue.is_empty() {
                    self.market.push(self.queue.remove(0));
                }
                self.market.sort_by_key(|x| x.price);
            }
            Mv::Fuel(f) => {
                self.money[p] -= self.fuel_price(f);
                self.supply[f as usize] -= 1;
                self.fuel[p][f as usize] += 1;
            }
            Mv::Connect(k) => {
                let c = self.connect_cost(p, k as usize).unwrap_or(0);
                self.money[p] -= c;
                self.cities[p][k as usize] = true;
                if self.n_cities(p) >= 7 && self.ending.is_none() {
                    self.ending = Some(self.side);
                }
            }
            Mv::Produce => {
                let n = self.power(p);
                let mut left = self.fuel[p];
                let mut cap = 0;
                for pl in &self.plants[p] {
                    if pl.fuel == 2 {
                        cap += pl.cap as i32;
                    } else if left[pl.fuel as usize] >= 1 {
                        left[pl.fuel as usize] -= 1;
                        cap += pl.cap as i32;
                    }
                }
                self.fuel[p] = left;
                self.money[p] += 4 + 3 * n;
                let _ = cap;
                self.final_power[p] = n;
                self.last_prod[p] = self.turns;
            }
            Mv::Pass => {}
        }
        for f in 0..2 {
            if self.turns % 4 == 3 && self.supply[f] < 6 {
                self.supply[f] += 1;
            }
        }
        self.side = self.side.other();
        self.turns += 1;
    }

    fn outcome(&self) -> Option<Outcome> {
        let over = self.turns >= 240 || (self.ending.is_some() && self.turns % 2 == 0);
        if !over {
            return None;
        }
        let f = |p: usize| {
            (
                self.n_cities(p).min(self.capacity(p) as usize),
                self.money[p],
            )
        };
        Some(match f(0).cmp(&f(1)) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            _ => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let f = |q: usize| {
            (self.n_cities(q) as i32).min(self.capacity(q)) * 14
                + self.n_cities(q) as i32 * 6
                + self.capacity(q) * 3
                + self.power(q) * 3
                + self.money[q]
        };
        f(p.idx()) - f(p.other().idx())
    }

    fn render(&self) -> String {
        let mut s = String::new();
        for q in 0..2 {
            s.push_str(&format!(
                "J{}{}: bani {}, orașe {:?}, centrale {:?}, combustibil c{} p{}\n",
                q + 1,
                if self.side.idx() == q {
                    " (la rând)"
                } else {
                    ""
                },
                self.money[q],
                (0..CITIES)
                    .filter(|&k| self.cities[q][k])
                    .map(|k| k + 1)
                    .collect::<Vec<_>>(),
                self.plants[q]
                    .iter()
                    .map(|x| format!("{}/{}:{}", x.cap, ["c", "p", "e"][x.fuel as usize], x.price))
                    .collect::<Vec<_>>(),
                self.fuel[q][0],
                self.fuel[q][1]
            ));
        }
        s.push_str(&format!(
            "Piață centrale: {:?}\nPreț combustibil: c{} p{} (stoc {}/{})\n",
            self.market
                .iter()
                .map(|x| format!("{}/{}:{}", x.cap, ["c", "p", "e"][x.fuel as usize], x.price))
                .collect::<Vec<_>>(),
            self.fuel_price(0),
            self.fuel_price(1),
            self.supply[0],
            self.supply[1]
        ));
        s.push_str("Mutări: «centrala i», «combustibil c|p», «oras k», «produc», «pas».\n");
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        match *m {
            Mv::BuyPlant(i) => format!("centrala {}", i + 1),
            Mv::Fuel(f) => format!("combustibil {}", ["c", "p"][f as usize]),
            Mv::Connect(k) => format!("oras {}", k + 1),
            Mv::Produce => "produc".into(),
            Mv::Pass => "pas".into(),
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
    fn random_games_end_and_money_never_negative() {
        for seed in 1..=30u64 {
            let mut g = Retea::new();
            let mut rng = Rng::new(seed);
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
                assert!(g.money.iter().all(|&m| m >= 0));
            }
        }
    }

    #[test]
    fn ai_beats_random() {
        polytabletop_core::ai::set_time_budget_override_ms(Some(5));
        let mut wins = 0;
        for seed in 1..=6u64 {
            let mut g = Retea::new();
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
        assert!(wins >= 4, "{wins}");
    }
}
