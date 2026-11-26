//! Motor „construcție de pachet” (lista B4): mâinile sunt deschise (publice), piața e publică.
//! Moduri: `h` = puterea lovește autoritatea adversarului, `r` = puterea avansează pe un drum,
//! `c` = puterea coboară într-o peșteră (puncte la adâncime). AI-ul vede doar starea curentă
//! (adâncime 0) și nu cunoaște ordinea viitoare a pachetelor.
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

pub struct DbSpec {
    pub title: &'static str,
    pub mode: char,
    pub market: usize,
    pub goal: i32,
    pub turns: u32,
    pub depth: u32,
}

#[derive(Clone, Copy)]
struct Card {
    cost: i32,
    coin: i32,
    pow: i32,
    vp: i32,
}

const START_COIN: Card = Card {
    cost: 0,
    coin: 1,
    pow: 0,
    vp: 0,
};
const START_POW: Card = Card {
    cost: 0,
    coin: 0,
    pow: 1,
    vp: 0,
};

#[derive(Clone)]
pub struct Db {
    sp: &'static DbSpec,
    deck: [Vec<Card>; 2],
    hand: [Vec<Card>; 2],
    disc: [Vec<Card>; 2],
    pool: [i32; 2],
    power: [i32; 2],
    bought_vp: [i32; 2],
    prog: [i32; 2],
    auth: [i32; 2],
    queue: Vec<Card>,
    market: Vec<Card>,
    turns: [u32; 2],
    side: Player,
    rng: Rng,
}

impl Db {
    pub fn new(sp: &'static DbSpec) -> Self {
        Self::with_seed(sp, seed_now())
    }
    pub fn with_seed(sp: &'static DbSpec, seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut queue = Vec::new();
        for _ in 0..40 {
            let cost = 2 + rng.below(6) as i32;
            let mut c = Card {
                cost,
                coin: 0,
                pow: 0,
                vp: 0,
            };
            match rng.below(3) {
                0 => c.coin = 1 + cost / 3,
                1 => c.pow = 1 + cost / 2,
                _ => {
                    c.coin = cost / 4;
                    c.pow = cost / 4;
                    c.vp = 1 + cost / 3;
                }
            }
            queue.push(c);
        }
        let mut d = Db {
            sp,
            deck: [Vec::new(), Vec::new()],
            hand: [Vec::new(), Vec::new()],
            disc: [Vec::new(), Vec::new()],
            pool: [0; 2],
            power: [0; 2],
            bought_vp: [0; 2],
            prog: [0; 2],
            auth: [sp.goal; 2],
            queue,
            market: Vec::new(),
            turns: [0; 2],
            side: Player::One,
            rng,
        };
        for p in 0..2 {
            for i in 0..10 {
                d.deck[p].push(if i < 7 { START_COIN } else { START_POW });
            }
            let mut dk = std::mem::take(&mut d.deck[p]);
            shuf(&mut dk, &mut d.rng);
            d.deck[p] = dk;
        }
        d.refill();
        d.draw(0);
        d
    }
    fn refill(&mut self) {
        while self.market.len() < self.sp.market && !self.queue.is_empty() {
            let c = self.queue.remove(0);
            self.market.push(c);
        }
    }
    fn draw(&mut self, p: usize) {
        for _ in 0..5 {
            if self.deck[p].is_empty() {
                let mut d = std::mem::take(&mut self.disc[p]);
                shuf(&mut d, &mut self.rng);
                self.deck[p] = d;
            }
            if let Some(c) = self.deck[p].pop() {
                self.hand[p].push(c);
            }
        }
    }
    fn quality(&self, p: usize) -> i32 {
        self.deck[p]
            .iter()
            .chain(self.hand[p].iter())
            .chain(self.disc[p].iter())
            .map(|c| 2 * c.cost + c.vp)
            .sum()
    }
    fn score(&self, p: usize) -> i32 {
        match self.sp.mode {
            'h' => self.auth[p] * 2 - self.auth[1 - p] * 2,
            'r' => self.prog[p] * 3,
            _ => self.prog[p] + self.bought_vp[p],
        }
    }
    fn won(&self, p: usize) -> bool {
        match self.sp.mode {
            'h' => self.auth[1 - p] <= 0,
            'r' => self.prog[p] >= self.sp.goal,
            _ => false,
        }
    }
    fn apply_power(&mut self, me: usize) {
        let w = self.power[me];
        self.power[me] = 0;
        match self.sp.mode {
            'h' => self.auth[1 - me] -= w,
            'r' => self.prog[me] += w,
            _ => self.prog[me] += w / 2,
        }
    }
}

impl Mini for Db {
    fn side(&self) -> Player {
        self.side
    }
    fn depth(&self) -> u32 {
        self.sp.depth
    }
    fn moves(&self) -> Vec<String> {
        let me = pi(self.side);
        let mut v = vec!["fin".to_string()];
        let mut seen: Vec<(i32, i32, i32, i32)> = Vec::new();
        for (i, c) in self.hand[me].iter().enumerate() {
            let k = (c.cost, c.coin, c.pow, c.vp);
            if !seen.contains(&k) {
                seen.push(k);
                v.push(format!("juc {i}"));
            }
        }
        for (j, c) in self.market.iter().enumerate() {
            if c.cost <= self.pool[me] {
                v.push(format!("cump {j}"));
            }
        }
        v
    }
    fn step(&mut self, m: &str) {
        let me = pi(self.side);
        if let Some(i) = m.strip_prefix("juc ") {
            let c = self.hand[me].remove(i.parse().unwrap());
            self.pool[me] += c.coin;
            self.power[me] += c.pow;
            self.disc[me].push(c);
        } else if let Some(j) = m.strip_prefix("cump ") {
            let c = self.market.remove(j.parse().unwrap());
            self.pool[me] -= c.cost;
            self.bought_vp[me] += if self.sp.mode == 'h' || self.sp.mode == 'r' {
                0
            } else {
                c.vp
            };
            self.disc[me].push(c);
            self.refill();
        } else {
            // fin: puterea acumulată se aplică, restul mâinii se aruncă, tragi 5
            self.apply_power(me);
            let h = std::mem::take(&mut self.hand[me]);
            self.disc[me].extend(h);
            self.pool[me] = 0;
            self.turns[me] += 1;
            self.side = self.side.other();
            let o = 1 - me;
            if self.hand[o].is_empty() {
                self.draw(o);
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        for p in 0..2 {
            if self.won(p) {
                return Some(Outcome::Win(if p == 0 { Player::One } else { Player::Two }));
            }
        }
        if self.turns[0] >= self.sp.turns && self.turns[1] >= self.sp.turns {
            return Some(by_score(self.score(0), self.score(1)));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let f =
            |q: usize| self.quality(q) + 4 * self.score(q) + 3 * self.pool[q] + 4 * self.power[q];
        // doar informații publice ale jucătorului (mâna adversarului nu contează)
        let mine = f(a);
        let theirs = self.quality(1 - a) + 4 * self.score(1 - a);
        mine - theirs
    }
    fn show(&self) -> String {
        let mut s = format!(
            "{} — tura {}/{}\n",
            self.sp.title,
            self.turns[0].min(self.turns[1]) + 1,
            self.sp.turns
        );
        for (j, c) in self.market.iter().enumerate() {
            s.push_str(&format!(
                "  piață [{j}] cost {} monedă {} putere {} puncte {}\n",
                c.cost, c.coin, c.pow, c.vp
            ));
        }
        for p in 0..2 {
            let h: Vec<String> = self.hand[p]
                .iter()
                .map(|c| format!("{}/{}", c.coin, c.pow))
                .collect();
            s.push_str(&format!(
                "J{}: mână (deschisă) [{}] | monede {} putere {} | progres {} autoritate {} puncte-cărți {} | pachet {}+{}\n",
                p + 1,
                h.join(" "),
                self.pool[p],
                self.power[p],
                self.prog[p],
                self.auth[p],
                self.bought_vp[p],
                self.deck[p].len(),
                self.disc[p].len()
            ));
        }
        s.push_str(&format!(
            "La mutare: {} (juc i | cump j | fin)\n",
            pname(self.side)
        ));
        s
    }
}
