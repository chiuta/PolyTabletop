//! Vameșul din Poartă — bluf cu geanta de negustor (variantă originală, 2 jucători).
//!
//! Șase runde; rolul alternează: negustor și vameș. Marfa: 0 mere (valoare 2), 1 pâine (3),
//! 2 brânză (3) și 3 contrabandă (6). Negustorul (6 cărți în mână, ascunse) pune în geantă 1–4
//! cărți și declară public un singur fel legal și numărul: `sac a,b,c,d>t` (a mere, b pâine,
//! c brânză, d contrabandă; t = 0, 1 sau 2 felul declarat). Vameșul: `las` sau `inspectez`;
//! apoi mutarea forțată `verific`. Geanta inspectată și cinstită: vameșul plătește 2 monede pe carte;
//! altfel negustorul plătește 2 monede (4 pentru contrabandă) pentru fiecare carte care nu e de felul
//! declarat, iar acelea se confiscă. Geanta neinspectată trece întreagă. Punctaj = monedele (start 10)
//! + valoarea mărfii trecute. Informație ascunsă: vezi doar mâna ta și, ca negustor, geanta ta.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const VAL: [i32; 4] = [2, 3, 3, 6];
const PEN: [i32; 4] = [2, 2, 2, 4];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ph {
    Pack,
    Decide,
    Resolve,
}

#[derive(Clone)]
pub struct Vamesul {
    viewer: Option<Player>,
    hand: [[u8; 4]; 2],
    bag: [u8; 4],
    decl: usize,
    inspected: bool,
    gold: [i32; 2],
    stall: [i32; 2],
    round: u32,
    ph: Ph,
    lies: u32,
    revealed: u32,
    rng: Rng,
}

impl Vamesul {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut g = Vamesul {
            viewer: None,
            hand: [[0; 4]; 2],
            bag: [0; 4],
            decl: 0,
            inspected: false,
            gold: [10; 2],
            stall: [0; 2],
            round: 0,
            ph: Ph::Pack,
            lies: 0,
            revealed: 0,
            rng: Rng::new(seed),
        };
        g.refill(0);
        g.refill(1);
        g
    }
    fn refill(&mut self, p: usize) {
        while self.hand[p].iter().map(|&x| x as u32).sum::<u32>() < 6 {
            let r = self.rng.below(11);
            let k = if r < 3 {
                0
            } else if r < 6 {
                1
            } else if r < 9 {
                2
            } else {
                3
            };
            self.hand[p][k] += 1;
        }
    }
    fn merchant(&self) -> usize {
        (self.round % 2) as usize
    }
    fn n_bag(&self) -> i32 {
        self.bag.iter().map(|&x| x as i32).sum()
    }
    /// Efectul unei decizii asupra negustorului (monede + mărfuri trecute): (delta_negustor, delta_vames_monede)
    fn outcome(&self, inspected: bool) -> (i32, i32) {
        if !inspected {
            let v: i32 = (0..4).map(|k| self.bag[k] as i32 * VAL[k]).sum();
            return (v, 0);
        }
        let mut pen = 0;
        let mut pass = 0;
        for k in 0..4 {
            if k == self.decl {
                pass += self.bag[k] as i32 * VAL[k];
            } else {
                pen += self.bag[k] as i32 * PEN[k];
            }
        }
        if pen == 0 {
            (pass + 2 * self.n_bag(), -2 * self.n_bag())
        } else {
            (pass - pen, pen)
        }
    }
    fn lie_rate(&self) -> i32 {
        // procent estimat de bagaje necinstite (prior 50%)
        (100 * (self.lies as i32 * 2 + 1)) / (self.revealed as i32 * 2 + 2)
    }
    fn total(&self, p: usize) -> i32 {
        self.gold[p] + self.stall[p]
    }
}

impl Default for Vamesul {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Vamesul {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        let m = self.merchant();
        let who = match self.ph {
            Ph::Pack => m,
            Ph::Decide => 1 - m,
            Ph::Resolve => 1 - m,
        };
        if who == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            Ph::Pack => {
                let h = self.hand[self.merchant()];
                let mut v = Vec::new();
                for a in 0..=h[0].min(4) {
                    for b in 0..=h[1].min(4 - a) {
                        for c in 0..=h[2].min(4 - a - b) {
                            for d in 0..=h[3].min(4 - a - b - c) {
                                if a + b + c + d >= 1 {
                                    for t in 0..3 {
                                        v.push(format!("sac {},{},{},{}>{}", a, b, c, d, t));
                                    }
                                }
                            }
                        }
                    }
                }
                v
            }
            Ph::Decide => vec!["las".into(), "inspectez".into()],
            Ph::Resolve => vec!["verific".into()],
        }
    }
    fn step(&mut self, m: &str) {
        let me = self.merchant();
        match self.ph {
            Ph::Pack => {
                let (c, t) = m[4..].split_once('>').unwrap();
                let v: Vec<u8> = c.split(',').map(|x| x.parse().unwrap()).collect();
                for k in 0..4 {
                    self.hand[me][k] -= v[k];
                }
                self.bag.copy_from_slice(&v);
                self.decl = t.parse().unwrap();
                self.ph = Ph::Decide;
            }
            Ph::Decide => {
                self.inspected = m == "inspectez";
                self.ph = Ph::Resolve;
            }
            Ph::Resolve => {
                if self.inspected {
                    self.revealed += 1;
                    let honest = (0..4).all(|k| k == self.decl || self.bag[k] == 0);
                    if !honest {
                        self.lies += 1;
                    }
                    let n = self.n_bag();
                    self.stall[me] += self.bag[self.decl] as i32 * VAL[self.decl];
                    if honest {
                        self.gold[me] += 2 * n;
                        self.gold[1 - me] -= 2 * n;
                    } else {
                        let pen: i32 = (0..4)
                            .filter(|&k| k != self.decl)
                            .map(|k| self.bag[k] as i32 * PEN[k])
                            .sum();
                        self.gold[me] -= pen;
                        self.gold[1 - me] += pen;
                    }
                } else {
                    self.stall[me] += self.outcome(false).0;
                }
                self.bag = [0; 4];
                self.refill(me);
                self.round += 1;
                self.ph = Ph::Pack;
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.round >= 6 {
            return Some(by_score(self.total(0), self.total(1)));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let me = self.merchant();
        let base = 10 * (self.total(a) - self.total(1 - a));
        let delta = match self.ph {
            Ph::Pack => 0,
            Ph::Decide => {
                if a == me {
                    // știu geanta; estimez rata de inspecție 50%
                    let (dm_i, _) = self.outcome(true);
                    let (dm_n, _) = self.outcome(false);
                    5 * (dm_i + dm_n)
                } else {
                    0
                }
            }
            Ph::Resolve => {
                if a == me {
                    let (dm, _) = self.outcome(self.inspected);
                    10 * dm
                } else if self.inspected {
                    // estimare publică: necinstit cu probabilitatea lie_rate
                    let n = self.n_bag();
                    let pl = self.lie_rate();
                    // vameșul câștigă ~3n dacă e necinstit, pierde 2n dacă e cinstit
                    (pl * 3 * n - (100 - pl) * 2 * n) / 10
                } else {
                    let n = self.n_bag();
                    let pl = self.lie_rate();
                    // lasă să treacă: negustorul câștigă ~3n legal sau ~5n contrabandă
                    -((100 - pl) * 3 * n + pl * 5 * n) / 10
                }
            }
        };
        base + delta
    }
    fn show(&self) -> String {
        let me = self.merchant();
        let viewer = self.viewer.map(pi).unwrap_or(pi(self.side()));
        let mut s = format!(
            "Vameșul din Poartă — runda {}/6 | negustor: J{}, vameș: J{}\n",
            (self.round + 1).min(6),
            me + 1,
            2 - me
        );
        for p in 0..2 {
            s.push_str(&format!(
                "J{}: monede {} | marfă trecută {}\n",
                p + 1,
                self.gold[p],
                self.stall[p]
            ));
        }
        if viewer == me {
            s.push_str(&format!(
                "Mâna ta (mere, pâine, brânză, contrabandă): {:?}\n",
                self.hand[me]
            ));
            if self.ph != Ph::Pack {
                s.push_str(&format!("Geanta ta: {:?}\n", self.bag));
            }
        } else {
            s.push_str("Mâna negustorului e ascunsă.\n");
        }
        if self.ph != Ph::Pack {
            s.push_str(&format!(
                "Declarat: {} × {}\n",
                self.n_bag(),
                ["mere", "pâine", "brânză"][self.decl]
            ));
        }
        s.push_str(&format!(
            "La mutare: {} (faza {:?}: sac a,b,c,d>t | las | inspectez | verific)\n",
            pname(self.side()),
            self.ph
        ));
        s
    }
    fn view(&mut self, w: Option<Player>) {
        self.viewer = w;
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        crate::brand_util::check(Vamesul::with_seed, 100);
    }
    #[test]
    fn honest_inspected_pays_merchant() {
        let mut g = Vamesul::with_seed(3);
        g.hand[0] = [2, 2, 1, 1];
        g.step("sac 2,0,0,0>0");
        g.step("inspectez");
        g.step("verific");
        assert_eq!(g.gold, [14, 6]);
        assert_eq!(g.stall[0], 4);
    }
    #[test]
    fn viewer_hides_bag_from_inspector() {
        let mut g = Vamesul::with_seed(3);
        g.hand[0] = [2, 2, 1, 1];
        g.step("sac 0,0,0,1>0");
        g.view(Some(Player::Two));
        assert!(!g.show().contains("Geanta ta"));
        g.view(Some(Player::One));
        assert!(g.show().contains("Geanta ta"));
    }
    #[test]
    fn inspector_ai_does_not_read_the_bag() {
        let mut a = Vamesul::with_seed(3);
        a.hand[0] = [2, 2, 1, 1];
        a.step("sac 2,0,0,0>0");
        let mut b = Vamesul::with_seed(3);
        b.hand[0] = [2, 2, 1, 2];
        b.step("sac 0,0,0,2>0");
        for seed in 1..6 {
            assert_eq!(
                crate::brand_util::ai_of(&a, seed),
                crate::brand_util::ai_of(&b, seed)
            );
        }
    }
}
