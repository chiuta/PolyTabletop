//! Motor „constructor de așezări/tablou” (lista B4): piață publică de cărți, cumperi cu monede,
//! cărțile dau venit și puncte. Coada de cărți e publică (fără informație ascunsă).
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

pub struct TbSpec {
    pub title: &'static str,
    pub market: usize,
    pub deck: usize,
    /// s = reducere pentru culoarea deținută, z = rasez, l = împrumut
    pub fl: &'static str,
    pub turns: u32,
    pub setb: i32,
    pub inc_mul: i32,
    pub depth: u32,
}

#[derive(Clone, Copy)]
struct Card {
    cost: i32,
    inc: i32,
    vp: i32,
    col: u8,
}

#[derive(Clone)]
pub struct Tb {
    sp: &'static TbSpec,
    queue: Vec<Card>,
    market: Vec<Card>,
    own: [Vec<Card>; 2],
    coins: [i32; 2],
    debt: [i32; 2],
    turns: [u32; 2],
    side: Player,
}

impl Tb {
    pub fn new(sp: &'static TbSpec) -> Self {
        Self::with_seed(sp, seed_now())
    }
    pub fn with_seed(sp: &'static TbSpec, seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut queue = Vec::new();
        for _ in 0..sp.deck {
            let cost = 2 + rng.below(7) as i32;
            let inc = rng.below(1 + cost as usize / 3) as i32;
            let vp = ((cost + 1) / 2 - inc).max(0) + rng.below(2) as i32;
            queue.push(Card {
                cost,
                inc,
                vp,
                col: rng.below(4) as u8,
            });
        }
        let mut t = Tb {
            sp,
            queue,
            market: Vec::new(),
            own: [Vec::new(), Vec::new()],
            coins: [5, 6],
            debt: [0; 2],
            turns: [0; 2],
            side: Player::One,
        };
        t.refill();
        t
    }
    fn refill(&mut self) {
        while self.market.len() < self.sp.market && !self.queue.is_empty() {
            let c = self.queue.remove(0);
            self.market.push(c);
        }
    }
    fn price(&self, p: usize, c: &Card) -> i32 {
        let d = if self.sp.fl.contains('s') {
            self.own[p].iter().filter(|o| o.col == c.col).count() as i32
        } else {
            0
        };
        (c.cost - d).max(1)
    }
    fn income(&self, p: usize) -> i32 {
        self.own[p].iter().map(|c| c.inc).sum::<i32>() * self.sp.inc_mul
    }
    fn score(&self, p: usize) -> i32 {
        let mut cnt = [0; 4];
        for c in &self.own[p] {
            cnt[c.col as usize] += 1;
        }
        self.own[p].iter().map(|c| c.vp).sum::<i32>()
            + self.sp.setb * cnt.iter().copied().min().unwrap()
            - 3 * self.debt[p]
            + self.coins[p] / 4
    }
}

impl Mini for Tb {
    fn side(&self) -> Player {
        self.side
    }
    fn depth(&self) -> u32 {
        self.sp.depth
    }
    fn moves(&self) -> Vec<String> {
        let me = pi(self.side);
        let mut v = Vec::new();
        for (i, c) in self.market.iter().enumerate() {
            if self.price(me, c) <= self.coins[me] {
                v.push(format!("compro {i}"));
            }
        }
        v.push("recolt".into());
        if self.sp.fl.contains('l') && self.debt[me] < 2 {
            v.push("imprumut".into());
        }
        if self.sp.fl.contains('z') && self.coins[me] >= 2 {
            for i in 0..self.own[1 - me].len() {
                v.push(format!("rasez {i}"));
            }
        }
        v
    }
    fn step(&mut self, m: &str) {
        let me = pi(self.side);
        if let Some(i) = m.strip_prefix("compro ") {
            let i: usize = i.parse().unwrap();
            let c = self.market.remove(i);
            self.coins[me] -= self.price(me, &c);
            self.own[me].push(c);
            self.refill();
        } else if m == "recolt" {
            self.coins[me] += 3;
        } else if m == "imprumut" {
            self.coins[me] += 4;
            self.debt[me] += 1;
        } else if let Some(i) = m.strip_prefix("rasez ") {
            let i: usize = i.parse().unwrap();
            self.coins[me] -= 2;
            self.own[1 - me].remove(i);
        }
        self.coins[me] += self.income(me);
        self.turns[me] += 1;
        self.side = self.side.other();
    }
    fn result(&self) -> Option<Outcome> {
        let done = self.turns[0] >= self.sp.turns && self.turns[1] >= self.sp.turns;
        let empty = self.market.is_empty() && self.queue.is_empty();
        if done || empty {
            return Some(by_score(self.score(0), self.score(1)));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let f = |q: usize| {
            let left = self.sp.turns.saturating_sub(self.turns[q]) as i32;
            4 * self.score(q) + self.income(q) * left.min(8) + self.coins[q]
        };
        f(a) - f(1 - a)
    }
    fn show(&self) -> String {
        let mut s = format!(
            "{} — tura {}/{}\n",
            self.sp.title,
            self.turns[0].min(self.turns[1]) + 1,
            self.sp.turns
        );
        for (i, c) in self.market.iter().enumerate() {
            s.push_str(&format!(
                "  [{i}] cost {} venit {} p{} culoarea {}\n",
                c.cost, c.inc, c.vp, c.col
            ));
        }
        let nx: Vec<String> = self
            .queue
            .iter()
            .take(3)
            .map(|c| format!("{}/{}/{}", c.cost, c.inc, c.vp))
            .collect();
        s.push_str(&format!("Urmează (public): {}\n", nx.join(" ")));
        for p in 0..2 {
            s.push_str(&format!(
                "J{}: monede {} venit {} cărți {} datorii {} scor {}\n",
                p + 1,
                self.coins[p],
                self.income(p),
                self.own[p].len(),
                self.debt[p],
                self.score(p)
            ));
        }
        s.push_str(&format!(
            "La mutare: {} (compro i | recolt{}{})\n",
            pname(self.side),
            if self.sp.fl.contains('l') {
                " | imprumut"
            } else {
                ""
            },
            if self.sp.fl.contains('z') {
                " | rasez i"
            } else {
                ""
            }
        ));
        s
    }
}
