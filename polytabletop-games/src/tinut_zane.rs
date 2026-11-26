//! Ținutul Zânelor — călătorie cu cărți de transport pe o hartă de 10 orașe (variantă originală).
//!
//! Hartă: 10 orașe în cerc, cu drumuri întărite de câteva scurtături. Fiecare drum are un teren
//! (pădure, râu, munte, deșert) și un cost de 1 sau 2 cărți de transport de acel tip. Fiecare
//! oraș are o valoare (1–3) pe care o ia primul care ajunge acolo. Pe tură: `ia t` (o carte de tip t din piața
//! de 4 cărți deschise, apoi se reface din pachet; max 8 cărți) sau `mut X` (te muți în orașul X
//! plătind cărțile potrivite); `descarto t` dacă ai 8 cărți și nu poți muta. Ambii pornesc din orașul 1.
//! Se termină când 8 orașe au fost revendicate sau după 80 de mutări. Câștigă cine are mai multe puncte.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const TY: [&str; 4] = ["padure", "rau", "munte", "desert"];

#[derive(Clone)]
pub struct TinutZane {
    edges: Vec<(usize, usize, u8, u8)>, // a,b,tip,cost
    value: [u8; 10],
    claimed: [Option<usize>; 10],
    at: [usize; 2],
    hand: [[u8; 4]; 2],
    market: Vec<u8>,
    deck: Vec<u8>,
    pts: [i32; 2],
    side: Player,
    ply: u32,
}

impl TinutZane {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut edges = Vec::new();
        for i in 0..10 {
            edges.push((i, (i + 1) % 10, rng.below(4) as u8, 1 + rng.below(2) as u8));
        }
        let mut k = 0;
        while k < 5 {
            let (a, b) = (rng.below(10), rng.below(10));
            if a != b
                && !edges
                    .iter()
                    .any(|&(x, y, _, _)| (x == a && y == b) || (x == b && y == a))
            {
                edges.push((a, b, rng.below(4) as u8, 1 + rng.below(2) as u8));
                k += 1;
            }
        }
        let mut value = [0u8; 10];
        for v in value.iter_mut() {
            *v = 1 + rng.below(3) as u8;
        }
        let mut deck: Vec<u8> = (0..80).map(|i| (i % 4) as u8).collect();
        shuf(&mut deck, &mut rng);
        let market: Vec<u8> = (0..4).map(|_| deck.pop().unwrap()).collect();
        let mut hand = [[0u8; 4]; 2];
        for h in hand.iter_mut() {
            for _ in 0..3 {
                h[deck.pop().unwrap() as usize] += 1;
            }
        }
        let mut claimed = [None; 10];
        claimed[0] = Some(2);
        TinutZane {
            edges,
            value,
            claimed,
            at: [0, 0],
            hand,
            market,
            deck,
            pts: [0; 2],
            side: Player::One,
            ply: 0,
        }
    }
    fn hsize(&self, p: usize) -> u8 {
        self.hand[p].iter().sum()
    }
    fn nbrs(&self, c: usize) -> Vec<(usize, u8, u8)> {
        self.edges
            .iter()
            .filter_map(|&(a, b, t, k)| {
                if a == c {
                    Some((b, t, k))
                } else if b == c {
                    Some((a, t, k))
                } else {
                    None
                }
            })
            .collect()
    }
    fn nclaimed(&self) -> usize {
        self.claimed.iter().filter(|c| c.is_some()).count() - 1
    }
}

impl Default for TinutZane {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for TinutZane {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        self.side
    }
    fn moves(&self) -> Vec<String> {
        let me = pi(self.side);
        let mut v = Vec::new();
        for (to, t, k) in self.nbrs(self.at[me]) {
            if self.hand[me][t as usize] >= k {
                v.push(format!("mut {}", to + 1));
            }
        }
        if self.hsize(me) < 8 {
            let mut seen = [false; 4];
            for &c in &self.market {
                if !seen[c as usize] {
                    seen[c as usize] = true;
                    v.push(format!("ia {}", TY[c as usize]));
                }
            }
        }
        if v.is_empty() {
            for t in 0..4 {
                if self.hand[me][t] > 0 {
                    v.push(format!("descarto {}", TY[t]));
                }
            }
        }
        if v.is_empty() {
            v.push("pas".into());
        }
        v
    }
    fn step(&mut self, m: &str) {
        let me = pi(self.side);
        self.ply += 1;
        let (cmd, arg) = m.split_once(' ').unwrap_or((m, ""));
        match cmd {
            "mut" => {
                let to = arg.parse::<usize>().unwrap() - 1;
                let (_, t, k) = self
                    .nbrs(self.at[me])
                    .into_iter()
                    .find(|x| x.0 == to && self.hand[me][x.1 as usize] >= x.2)
                    .unwrap();
                self.hand[me][t as usize] -= k;
                self.at[me] = to;
                if self.claimed[to].is_none() {
                    self.claimed[to] = Some(me);
                    self.pts[me] += self.value[to] as i32;
                }
            }
            "ia" => {
                let t = TY.iter().position(|&x| x == arg).unwrap() as u8;
                let i = self.market.iter().position(|&x| x == t).unwrap();
                self.market.remove(i);
                self.hand[me][t as usize] += 1;
                if let Some(c) = self.deck.pop() {
                    self.market.push(c);
                }
            }
            "descarto" => {
                let t = TY.iter().position(|&x| x == arg).unwrap();
                self.hand[me][t] -= 1;
            }
            _ => {}
        }
        self.side = self.side.other();
    }
    fn result(&self) -> Option<Outcome> {
        if self.nclaimed() >= 8 || self.ply >= 80 || self.market.is_empty() {
            return Some(by_score(self.pts[0], self.pts[1]));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let reach = |i: usize| -> i32 {
            self.nbrs(self.at[i])
                .iter()
                .filter(|&&(to, t, k)| {
                    self.claimed[to].is_none() && self.hand[i][t as usize] + 1 >= k
                })
                .map(|&(to, _, _)| self.value[to] as i32)
                .max()
                .unwrap_or(0)
        };
        10 * (self.pts[a] - self.pts[1 - a])
            + 2 * (self.hsize(a) as i32 - self.hsize(1 - a) as i32)
            + reach(a)
            - reach(1 - a)
    }
    fn show(&self) -> String {
        let mut s = String::from(
            "Ținutul Zânelor — orașe 1..10 (valoare, [primul]); drumuri a-b teren/cost\n",
        );
        for c in 0..10 {
            s.push_str(&format!(
                "{}: v{}{} | ",
                c + 1,
                self.value[c],
                match self.claimed[c] {
                    Some(2) => " start".to_string(),
                    Some(p) => format!(" [J{}]", p + 1),
                    None => String::new(),
                }
            ));
        }
        s.push('\n');
        for &(a, b, t, k) in &self.edges {
            s.push_str(&format!("{}-{}:{}/{}  ", a + 1, b + 1, TY[t as usize], k));
        }
        s.push('\n');
        let mk: Vec<&str> = self.market.iter().map(|&c| TY[c as usize]).collect();
        s.push_str(&format!(
            "Piață: {} | pachet {}\n",
            mk.join(", "),
            self.deck.len()
        ));
        for p in 0..2 {
            let h: Vec<String> = (0..4)
                .map(|t| format!("{}×{}", TY[t], self.hand[p][t]))
                .collect();
            s.push_str(&format!(
                "J{}: în orașul {} | puncte {} | cărți {}\n",
                p + 1,
                self.at[p] + 1,
                self.pts[p],
                h.join(" ")
            ));
        }
        s.push_str(&format!(
            "La mutare: {} (ia t | mut X | descarto t)\n",
            pname(self.side)
        ));
        s
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        crate::brand_util::check(TinutZane::with_seed, 200);
    }
}
