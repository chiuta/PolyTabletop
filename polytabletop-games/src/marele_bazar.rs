//! Marele Bazar — neguțători pe o grilă de locații (variantă originală, 2 jucători).
//!
//! Tablă 4×4 de locații amestecate: 4 Piețe (câte o marfă: +2 mărfuri de culoarea lor), 3 Bănci
//! (+4 lire), 3 Ateliere (vinzi toate mărfurile din culoarea cea mai numeroasă, 2 lire bucata, max 4),
//! 2 Palate (plătești câte 1 marfă din 3 culori ca să iei un rubin), 2 Bijutieri (un rubin pentru 9
//! lire + 2 pentru fiecare rubin deja luat) și 2 Fântâni (+1 marfă din culoarea cea mai rară). Mutarea
//! `mut a1>b2`: neguțătorul tău se mută la 1–2 pași ortogonali (nu pe același pătrat cu
//! adversarul decât plătindu-i 2 lire), apoi se aplică automat acțiunea locației. Câștigă primul
//! cu 5 rubine; după 100 de mutări, cine are mai multe rubine și apoi lire.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Loc {
    Market(u8),
    Bank,
    Shop,
    Palace,
    Jeweler,
    Fountain,
}

#[derive(Clone)]
pub struct MareleBazar {
    loc: [Loc; 16],
    at: [usize; 2],
    goods: [[i32; 4]; 2],
    lira: [i32; 2],
    ruby: [i32; 2],
    side: Player,
    ply: u32,
}

fn nm(i: usize) -> String {
    format!("{}{}", (b'a' + (i % 4) as u8) as char, i / 4 + 1)
}
fn pm(s: &str) -> usize {
    let b = s.as_bytes();
    (b[1] - b'1') as usize * 4 + (b[0] - b'a') as usize
}

impl MareleBazar {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut v = vec![
            Loc::Market(0),
            Loc::Market(1),
            Loc::Market(2),
            Loc::Market(3),
            Loc::Bank,
            Loc::Bank,
            Loc::Bank,
            Loc::Shop,
            Loc::Shop,
            Loc::Shop,
            Loc::Palace,
            Loc::Palace,
            Loc::Jeweler,
            Loc::Jeweler,
            Loc::Fountain,
            Loc::Fountain,
        ];
        shuf(&mut v, &mut rng);
        let mut loc = [Loc::Bank; 16];
        loc.copy_from_slice(&v);
        MareleBazar {
            loc,
            at: [0, 15],
            goods: [[1; 4]; 2],
            lira: [3, 3],
            ruby: [0; 2],
            side: Player::One,
            ply: 0,
        }
    }
    fn jewel_price(&self) -> i32 {
        9 + 2 * (self.ruby[0] + self.ruby[1])
    }
    fn act(&mut self, me: usize) {
        match self.loc[self.at[me]] {
            Loc::Market(c) => self.goods[me][c as usize] = (self.goods[me][c as usize] + 2).min(8),
            Loc::Bank => self.lira[me] += 4,
            Loc::Shop => {
                let (c, &n) = self.goods[me]
                    .iter()
                    .enumerate()
                    .max_by_key(|&(_, &n)| n)
                    .unwrap();
                let k = n.min(4);
                self.goods[me][c] -= k;
                self.lira[me] += 2 * k;
            }
            Loc::Palace => {
                let mut idx: Vec<usize> = (0..4).filter(|&c| self.goods[me][c] > 0).collect();
                if idx.len() >= 3 {
                    idx.sort_by_key(|&c| -self.goods[me][c]);
                    for &c in &idx[..3] {
                        self.goods[me][c] -= 1;
                    }
                    self.ruby[me] += 1;
                }
            }
            Loc::Jeweler => {
                let pr = self.jewel_price();
                if self.lira[me] >= pr {
                    self.lira[me] -= pr;
                    self.ruby[me] += 1;
                }
            }
            Loc::Fountain => {
                let (c, _) = self.goods[me]
                    .iter()
                    .enumerate()
                    .min_by_key(|&(_, &n)| n)
                    .unwrap();
                self.goods[me][c] = (self.goods[me][c] + 1).min(8);
            }
        }
    }
}

impl Default for MareleBazar {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for MareleBazar {
    fn depth(&self) -> u32 {
        3
    }
    fn side(&self) -> Player {
        self.side
    }
    fn moves(&self) -> Vec<String> {
        let me = pi(self.side);
        let (r, c) = ((self.at[me] / 4) as i32, (self.at[me] % 4) as i32);
        let mut v = Vec::new();
        for i in 0..16usize {
            let (a, b) = ((i / 4) as i32, (i % 4) as i32);
            let d = (a - r).abs() + (b - c).abs();
            if (1..=2).contains(&d) {
                if i == self.at[1 - me] && self.lira[me] < 2 {
                    continue;
                }
                v.push(format!("mut {}>{}", nm(self.at[me]), nm(i)));
            }
        }
        v
    }
    fn step(&mut self, m: &str) {
        let me = pi(self.side);
        self.ply += 1;
        let (_, to) = m[4..].split_once('>').unwrap();
        let to = pm(to);
        if to == self.at[1 - me] {
            self.lira[me] -= 2;
            self.lira[1 - me] += 2;
        }
        self.at[me] = to;
        self.act(me);
        self.side = self.side.other();
    }
    fn result(&self) -> Option<Outcome> {
        for p in 0..2 {
            if self.ruby[p] >= 5 {
                return Some(Outcome::Win(if p == 0 { Player::One } else { Player::Two }));
            }
        }
        if self.ply >= 100 {
            return Some(by_score(
                self.ruby[0] * 1000 + self.lira[0],
                self.ruby[1] * 1000 + self.lira[1],
            ));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let f = |i: usize| {
            100 * self.ruby[i] + 3 * self.lira[i] + 4 * self.goods[i].iter().sum::<i32>()
        };
        f(a) - f(1 - a)
    }
    fn show(&self) -> String {
        let lab = |l: Loc| match l {
            Loc::Market(c) => format!("Piață{}", c + 1),
            Loc::Bank => "Bancă".into(),
            Loc::Shop => "Atelier".into(),
            Loc::Palace => "Palat".into(),
            Loc::Jeweler => "Bijutier".into(),
            Loc::Fountain => "Fântână".into(),
        };
        let mut s = String::from("Marele Bazar — a..d coloane, 1..4 rânduri\n");
        for r in 0..4 {
            for c in 0..4 {
                let i = r * 4 + c;
                let mut mk = String::new();
                if self.at[0] == i {
                    mk.push('1');
                }
                if self.at[1] == i {
                    mk.push('2');
                }
                s.push_str(&format!(
                    "{:<10}",
                    format!(
                        "{}{}{}",
                        lab(self.loc[i]),
                        if mk.is_empty() { "" } else { "*" },
                        mk
                    )
                ));
            }
            s.push('\n');
        }
        for p in 0..2 {
            s.push_str(&format!(
                "J{}: rubine {} | lire {} | mărfuri {:?}\n",
                p + 1,
                self.ruby[p],
                self.lira[p],
                self.goods[p]
            ));
        }
        s.push_str(&format!(
            "Bijutier: {} lire | La mutare: {} (mut a1>b2)\n",
            self.jewel_price(),
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
        crate::brand_util::check(MareleBazar::with_seed, 200);
    }
}
