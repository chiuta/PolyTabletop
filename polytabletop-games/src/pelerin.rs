//! Drumul Pelerinului — călătorie pe o potecă cu popasuri (variantă originală).
//!
//! Poteca are 30 de popasuri; la 6, 13, 20 și 29 sunt hanuri (obligatorii: nu poți trece de un
//! han fără să te oprești). Muți mereu cel aflat în urmă (la egalitate, cel sosit mai târziu).
//! Alegi un popas liber înaintea ta (`mut k`) și îi aplici efectul: priveliște (+1, +2, +3, +4
//! puncte, în ordine), atelier (`cumpar` 2 monede: suveniruri de tipuri diferite: 1, 3, 5, 7
//! puncte cumulat; sau `treci`), izvor (+2), fermă (+3 monede), templu (`dau 1/2/3` monede = tot
//! atâtea puncte, sau `dau 0`), han (`mananc` 2 monede = 3 puncte o singură dată pe han, sau `sar`).
//! La han pot sta amândoi. Câștigă cine are mai multe puncte (monedele rămase: 1 punct la 3).
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const N: usize = 30;
const INNS: [usize; 4] = [6, 13, 20, 29];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum K {
    Inn,
    View,
    Shop,
    Spring,
    Farm,
    Temple,
}

#[derive(Clone)]
pub struct Pelerin {
    kinds: Vec<K>,
    pos: [i32; 2],
    arr: [u32; 2],
    clock: u32,
    pts: [i32; 2],
    coins: [i32; 2],
    views: [i32; 2],
    souv: [i32; 2],
    done: [bool; 2],
    pending: Option<K>,
}

impl Pelerin {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut pool: Vec<K> = Vec::new();
        for _ in 0..6 {
            pool.extend([K::View, K::Shop, K::Spring, K::Farm, K::Temple]);
        }
        shuf(&mut pool, &mut rng);
        let mut kinds = Vec::new();
        for i in 0..N {
            if INNS.contains(&i) {
                kinds.push(K::Inn);
            } else {
                kinds.push(pool.pop().unwrap());
            }
        }
        Pelerin {
            kinds,
            pos: [-1, -1],
            arr: [0, 0],
            clock: 0,
            pts: [0; 2],
            coins: [3, 3],
            views: [0; 2],
            souv: [0; 2],
            done: [false; 2],
            pending: None,
        }
    }
    fn mover(&self) -> usize {
        if let Some(_p) = self.pending {
            return self.acting();
        }
        match (self.done[0], self.done[1]) {
            (true, _) => 1,
            (_, true) => 0,
            _ => {
                if self.pos[0] < self.pos[1]
                    || (self.pos[0] == self.pos[1] && self.arr[0] > self.arr[1])
                {
                    0
                } else {
                    1
                }
            }
        }
    }
    fn acting(&self) -> usize {
        // jucătorul care tocmai a ajuns: cel cu `arr` maxim
        if self.arr[0] > self.arr[1] {
            0
        } else {
            1
        }
    }
    fn occupied(&self, k: usize, me: usize) -> bool {
        self.pos[1 - me] == k as i32 && !INNS.contains(&k)
    }
    fn total(&self, p: usize) -> i32 {
        self.pts[p] + self.coins[p] / 3
    }
}

impl Default for Pelerin {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Pelerin {
    fn depth(&self) -> u32 {
        3
    }
    fn side(&self) -> Player {
        if self.mover() == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn moves(&self) -> Vec<String> {
        let me = self.mover();
        if let Some(k) = self.pending {
            let c = self.coins[me];
            return match k {
                K::Shop => {
                    let mut v = vec!["treci".to_string()];
                    if c >= 2 {
                        v.push("cumpar".into());
                    }
                    v
                }
                K::Temple => (0..=3.min(c)).map(|i| format!("dau {}", i)).collect(),
                K::Inn => {
                    let mut v = vec!["sar".to_string()];
                    if c >= 2 {
                        v.push("mananc".into());
                    }
                    v
                }
                _ => vec!["ok".into()],
            };
        }
        let p = self.pos[me];
        let mut v = Vec::new();
        for k in (p + 1) as usize..N {
            if !self.occupied(k, me) {
                v.push(format!("mut {}", k));
            }
            if INNS.contains(&k) {
                break;
            }
        }
        v
    }
    fn step(&mut self, m: &str) {
        let me = self.mover();
        if let Some(k) = self.pending.take() {
            match (k, m) {
                (K::Shop, "cumpar") => {
                    self.coins[me] -= 2;
                    self.souv[me] += 1;
                    self.pts[me] += [0, 1, 2, 2, 2][self.souv[me].min(4) as usize];
                }
                (K::Temple, x) => {
                    let n: i32 = x[4..].parse().unwrap();
                    self.coins[me] -= n;
                    self.pts[me] += n;
                }
                (K::Inn, "mananc") => {
                    self.coins[me] -= 2;
                    self.pts[me] += 3;
                }
                _ => {}
            }
            if self.pos[me] as usize == N - 1 {
                self.done[me] = true;
            }
            return;
        }
        let k: usize = m[4..].parse().unwrap();
        self.pos[me] = k as i32;
        self.clock += 1;
        self.arr[me] = self.clock;
        match self.kinds[k] {
            K::View => {
                self.views[me] += 1;
                self.pts[me] += self.views[me].min(4);
            }
            K::Spring => self.pts[me] += 2,
            K::Farm => self.coins[me] += 3,
            kind => self.pending = Some(kind),
        }
        if self.pending.is_none() && k == N - 1 {
            self.done[me] = true;
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.done[0] && self.done[1] {
            return Some(by_score(self.total(0), self.total(1)));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        10 * (self.total(a) - self.total(1 - a)) + self.coins[a] - self.coins[1 - a]
    }
    fn show(&self) -> String {
        let mut s = String::from("Drumul Pelerinului — popasuri: V=priveliște A=atelier I=izvor F=fermă T=templu H=han\n");
        for (i, k) in self.kinds.iter().enumerate() {
            let ch = match k {
                K::Inn => 'H',
                K::View => 'V',
                K::Shop => 'A',
                K::Spring => 'I',
                K::Farm => 'F',
                K::Temple => 'T',
            };
            s.push_str(&format!("{}{}", i, ch));
            for p in 0..2 {
                if self.pos[p] == i as i32 {
                    s.push_str(if p == 0 { "[1]" } else { "[2]" });
                }
            }
            s.push(' ');
        }
        s.push('\n');
        for p in 0..2 {
            s.push_str(&format!(
                "J{}: poziție {} | puncte {} | monede {} | priveliști {} | suveniruri {}\n",
                p + 1,
                self.pos[p],
                self.pts[p],
                self.coins[p],
                self.views[p],
                self.souv[p]
            ));
        }
        s.push_str(&format!(
            "La mutare: {}{}\n",
            pname(self.side()),
            if self.pending.is_some() {
                " (alege efectul popasului)"
            } else {
                " (mut k)"
            }
        ));
        s
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        crate::brand_util::check(Pelerin::with_seed, 300);
    }
    #[test]
    fn cannot_skip_inn() {
        let g = Pelerin::with_seed(4);
        let m = g.moves();
        assert!(m.iter().all(|x| x[4..].parse::<usize>().unwrap() <= 6));
    }
}
