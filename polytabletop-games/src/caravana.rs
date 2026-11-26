//! Caravana de Mirodenii — negoț de mărfuri între doi neguțători (variantă originală).
//!
//! Șase mărfuri (diamante, aur, argint, mătase, mirodenii, piele) și cămile. Piața are 5 cărți.
//! Tura (o acțiune): `ia M` (o marfă din piață în mână, maxim 7 cărți), `camile` (toate cămilele
//! din piață în herghelia ta), `schimb A:B` (dai din mână o marfă A sau o cămilă `cam` și iei
//! marfa B din piață), `vand M n` (vinzi n cărți de aceeași marfă; diamantele, aurul și argintul
//! se vând doar câte cel puțin 2) și iei jetoane de valoare descrescătoare; la 3/4/5+ cărți vândute
//! primești un bonus. Mâinile sunt deschise. Jocul se termină când 3 feluri de jetoane s-au golit sau
//! pachetul nu mai poate reface piața. Cine are mai multe cămile primește +5. Câștigă cine are
//! mai multe puncte.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const NAMES: [&str; 7] = [
    "diamant",
    "aur",
    "argint",
    "matase",
    "mirodenii",
    "piele",
    "cam",
];
const TOK: [&[i32]; 6] = [
    &[7, 7, 5, 5, 5],
    &[6, 6, 5, 5, 5],
    &[5, 5, 5, 5, 5],
    &[5, 3, 3, 2, 2, 1, 1],
    &[5, 3, 3, 2, 2, 1, 1],
    &[4, 3, 2, 1, 1, 1, 1, 1, 1],
];

#[derive(Clone)]
pub struct Caravana {
    market: Vec<u8>,
    deck: Vec<u8>,
    hand: [[u8; 6]; 2],
    camels: [u8; 2],
    tokens: Vec<Vec<i32>>,
    bonus: [Vec<i32>; 3],
    score: [i32; 2],
    side: Player,
    ply: u32,
}

fn idx(n: &str) -> usize {
    NAMES.iter().position(|&x| x == n).unwrap()
}

impl Caravana {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck: Vec<u8> = Vec::new();
        for (t, n) in [(0u8, 6), (1, 6), (2, 6), (3, 8), (4, 8), (5, 10), (6, 8)] {
            for _ in 0..n {
                deck.push(t);
            }
        }
        shuf(&mut deck, &mut rng);
        let mut market = vec![6u8, 6, 6];
        for _ in 0..2 {
            market.push(deck.pop().unwrap());
        }
        let mut g = Caravana {
            market,
            deck,
            hand: [[0; 6]; 2],
            camels: [0; 2],
            tokens: TOK.iter().map(|t| t.to_vec()).collect(),
            bonus: [Vec::new(), Vec::new(), Vec::new()],
            score: [0; 2],
            side: Player::One,
            ply: 0,
        };
        for b in 0..3 {
            let mut v: Vec<i32> = match b {
                0 => vec![1, 1, 2, 2, 3, 3],
                1 => vec![4, 4, 5, 5, 6, 6],
                _ => vec![8, 8, 9, 10, 10],
            };
            shuf(&mut v, &mut rng);
            g.bonus[b] = v;
        }
        for p in 0..2 {
            for _ in 0..5 {
                g.draw_to(p);
            }
        }
        g
    }
    fn draw_to(&mut self, p: usize) {
        if let Some(c) = self.deck.pop() {
            if c == 6 {
                self.camels[p] += 1;
            } else {
                self.hand[p][c as usize] += 1;
            }
        }
    }
    fn refill(&mut self) {
        while self.market.len() < 5 {
            match self.deck.pop() {
                Some(c) => self.market.push(c),
                None => break,
            }
        }
    }
    fn hand_size(&self, p: usize) -> u8 {
        self.hand[p].iter().sum()
    }
    fn ended(&self) -> bool {
        self.tokens.iter().filter(|t| t.is_empty()).count() >= 3
            || self.market.len() < 5
            || self.ply > 300
    }
    fn final_scores(&self) -> [i32; 2] {
        let mut s = self.score;
        if self.camels[0] > self.camels[1] {
            s[0] += 5;
        } else if self.camels[1] > self.camels[0] {
            s[1] += 5;
        }
        s
    }
}

impl Default for Caravana {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Caravana {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        self.side
    }
    fn moves(&self) -> Vec<String> {
        let me = pi(self.side);
        let mut v = Vec::new();
        let hs = self.hand_size(me);
        for t in 0..6u8 {
            if self.market.contains(&t) && hs < 7 {
                v.push(format!("ia {}", NAMES[t as usize]));
            }
        }
        if self.market.contains(&6) {
            v.push("camile".into());
        }
        for b in 0..6u8 {
            if !self.market.contains(&b) {
                continue;
            }
            for a in 0..7u8 {
                let have = if a == 6 {
                    self.camels[me] > 0
                } else {
                    self.hand[me][a as usize] > 0
                };
                if a != b && have {
                    v.push(format!(
                        "schimb {}:{}",
                        NAMES[a as usize], NAMES[b as usize]
                    ));
                }
            }
        }
        for t in 0..6 {
            let n = self.hand[me][t];
            let min = if t < 3 { 2 } else { 1 };
            for k in min..=n {
                if !self.tokens[t].is_empty() {
                    v.push(format!("vand {} {}", NAMES[t], k));
                }
            }
        }
        if v.is_empty() {
            // piața doar cămile și mâna plină: aruncă
            for t in 0..6u8 {
                if self.hand[me][t as usize] > 0 {
                    v.push(format!("arunc {}", NAMES[t as usize]));
                }
            }
        }
        v
    }
    fn step(&mut self, m: &str) {
        let me = pi(self.side);
        self.ply += 1;
        let (cmd, rest) = m.split_once(' ').unwrap_or((m, ""));
        match cmd {
            "ia" => {
                let t = idx(rest) as u8;
                let i = self.market.iter().position(|&x| x == t).unwrap();
                self.market.remove(i);
                self.hand[me][t as usize] += 1;
                self.refill();
            }
            "camile" => {
                let n = self.market.iter().filter(|&&x| x == 6).count() as u8;
                self.market.retain(|&x| x != 6);
                self.camels[me] += n;
                self.refill();
            }
            "schimb" => {
                let (a, b) = rest.split_once(':').unwrap();
                let (a, b) = (idx(a), idx(b));
                if a == 6 {
                    self.camels[me] -= 1;
                } else {
                    self.hand[me][a] -= 1;
                }
                let i = self.market.iter().position(|&x| x as usize == b).unwrap();
                self.market[i] = a as u8;
                self.hand[me][b] += 1;
            }
            "vand" => {
                let (t, n) = rest.split_once(' ').unwrap();
                let (t, n) = (idx(t), n.parse::<usize>().unwrap());
                self.hand[me][t] -= n as u8;
                for _ in 0..n {
                    if let Some(x) = self.tokens[t].first().copied() {
                        self.score[me] += x;
                        self.tokens[t].remove(0);
                    }
                }
                if n >= 3 {
                    let b = (n - 3).min(2);
                    if let Some(x) = self.bonus[b].pop() {
                        self.score[me] += x;
                    }
                }
            }
            _ => {
                let t = idx(rest);
                self.hand[me][t] -= 1;
            }
        }
        self.side = self.side.other();
    }
    fn result(&self) -> Option<Outcome> {
        if self.ended() {
            let s = self.final_scores();
            return Some(by_score(s[0], s[1]));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let f = |i: usize| {
            let mut s = 10 * self.score[i] + 4 * self.camels[i] as i32;
            for t in 0..6 {
                let n = self.hand[i][t] as i32;
                s += 3 * n * (6 - t as i32) / 2;
            }
            s
        };
        f(a) - f(1 - a)
    }
    fn show(&self) -> String {
        let mk: Vec<&str> = self.market.iter().map(|&c| NAMES[c as usize]).collect();
        let mut s = format!(
            "Caravana de Mirodenii — piață: {} | pachet {}\n",
            mk.join(", "),
            self.deck.len()
        );
        for p in 0..2 {
            let h: Vec<String> = (0..6)
                .filter(|&t| self.hand[p][t] > 0)
                .map(|t| format!("{}×{}", NAMES[t], self.hand[p][t]))
                .collect();
            s.push_str(&format!(
                "J{}: puncte {} | cămile {} | mână: {}\n",
                p + 1,
                self.score[p],
                self.camels[p],
                h.join(" ")
            ));
        }
        let tk: Vec<String> = (0..6)
            .map(|t| format!("{}:{}", NAMES[t], self.tokens[t].len()))
            .collect();
        s.push_str(&format!(
            "Jetoane rămase: {}\nLa mutare: {} (ia M | camile | schimb A:B | vand M n)\n",
            tk.join(" "),
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
        crate::brand_util::check(Caravana::with_seed, 400);
    }
}
