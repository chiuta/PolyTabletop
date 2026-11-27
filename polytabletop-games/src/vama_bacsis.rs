//! Vama cu Bacșiș — negustor, vameș și mită (variantă originală, 2 jucători).
//!
//! 6 runde; rolurile alternează. Negustorul are în mână mărfuri legale de trei feluri (0, 1, 2: valori 2, 3, 4) și
//! contrabandă (tip 3, valoare 7). El pune 1–3 cărți în sac `sac a,b,c,d` (numărul de cărți din fiecare tip), apoi
//! declară `decl t m`: tipul legal t pretins pentru toate cărțile și un bacșiș m (0–3 monede). Vameșul
//! alege `las` (primește bacșișul, sacul trece întreg) sau `inspectez` (bacșișul se întoarce); mutarea forțată `verific`
//! deschide sacul: dacă totul e tipul declarat, vameșul plătește 2 monede pe carte; altfel negustorul plătește 3
//! pe fiecare carte nepotrivită (confiscată), iar restul trece. Scor = monede + valoarea mărfii trecute (start: 10 monede).
//! Informație ascunsă: mâna și sacul negustorului; AI-ul vameș nu le citește.
use crate::b6_util::pl;
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const VAL: [i32; 4] = [2, 3, 4, 7];
const ROUNDS: u32 = 6;
const NAMES: [&str; 4] = ["grâu", "mere", "brânză", "contrabandă"];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ph {
    Pack,
    Decl,
    Resp,
    Resolve(bool),
}

#[derive(Clone)]
pub struct Vama {
    viewer: Option<Player>,
    hand: [[i32; 4]; 2],
    bag: [i32; 4],
    decl: usize,
    bribe: i32,
    ph: Ph,
    round: u32,
    coins: [i32; 2],
    goods: [i32; 2],
    rng: Rng,
}

fn compositions(hand: &[i32; 4]) -> Vec<[i32; 4]> {
    let mut v = Vec::new();
    for a in 0..=hand[0].min(3) {
        for b in 0..=hand[1].min(3 - a) {
            for c in 0..=hand[2].min(3 - a - b) {
                for d in 0..=hand[3].min(3 - a - b - c) {
                    if a + b + c + d >= 1 {
                        v.push([a, b, c, d]);
                    }
                }
            }
        }
    }
    v
}

impl Vama {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut g = Vama {
            viewer: None,
            hand: [[0; 4]; 2],
            bag: [0; 4],
            decl: 0,
            bribe: 0,
            ph: Ph::Pack,
            round: 0,
            coins: [10; 2],
            goods: [0; 2],
            rng: Rng::new(seed),
        };
        g.refill(0);
        g.refill(1);
        g
    }
    fn refill(&mut self, p: usize) {
        while self.hand[p].iter().sum::<i32>() < 6 {
            let t = match self.rng.below(100) {
                0..=29 => 0,
                30..=54 => 1,
                55..=74 => 2,
                _ => 3,
            };
            self.hand[p][t] += 1;
        }
    }
    fn merchant(&self) -> usize {
        (self.round % 2) as usize
    }
    fn size(&self) -> i32 {
        self.bag.iter().sum()
    }
    fn bad(&self) -> i32 {
        self.size() - self.bag[self.decl]
    }
    fn total(&self, p: usize) -> i32 {
        self.coins[p] + self.goods[p]
    }
}

impl Default for Vama {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Vama {
    fn depth(&self) -> u32 {
        if self.ph == Ph::Pack {
            2
        } else {
            0
        }
    }
    fn side(&self) -> Player {
        match self.ph {
            Ph::Pack | Ph::Decl => pl(self.merchant()),
            Ph::Resp => pl(1 - self.merchant()),
            Ph::Resolve(_) => pl(1 - self.merchant()),
        }
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            Ph::Pack => compositions(&self.hand[self.merchant()])
                .iter()
                .map(|c| format!("sac {},{},{},{}", c[0], c[1], c[2], c[3]))
                .collect(),
            Ph::Decl => {
                let mut v = Vec::new();
                for t in 0..3 {
                    for m in 0..=3.min(self.coins[self.merchant()]) {
                        v.push(format!("decl {t} {m}"));
                    }
                }
                v
            }
            Ph::Resp => vec!["las".into(), "inspectez".into()],
            Ph::Resolve(_) => vec!["verific".into()],
        }
    }
    fn step(&mut self, m: &str) {
        let me = self.merchant();
        match self.ph {
            Ph::Pack => {
                let mut c = [0; 4];
                for (i, x) in m[4..].split(',').enumerate() {
                    c[i] = x.parse().unwrap();
                }
                self.bag = c;
                self.ph = Ph::Decl;
            }
            Ph::Decl => {
                let mut it = m[5..].split(' ');
                self.decl = it.next().unwrap().parse().unwrap();
                self.bribe = it.next().unwrap().parse().unwrap();
                self.ph = Ph::Resp;
            }
            Ph::Resp => {
                self.ph = Ph::Resolve(m == "inspectez");
            }
            Ph::Resolve(inspect) => {
                let sh = 1 - me;
                let n = self.size();
                let value: i32 = (0..4).map(|t| self.bag[t] * VAL[t]).sum();
                if !inspect {
                    self.coins[me] -= self.bribe;
                    self.coins[sh] += self.bribe;
                    self.goods[me] += value;
                } else if self.bad() == 0 {
                    let pay = (2 * n).min(self.coins[sh].max(0));
                    self.coins[sh] -= pay;
                    self.coins[me] += pay;
                    self.goods[me] += value;
                } else {
                    let fine = (3 * self.bad()).min(self.coins[me].max(0));
                    self.coins[me] -= fine;
                    self.coins[sh] += fine;
                    self.goods[me] += self.bag[self.decl] * VAL[self.decl];
                }
                for t in 0..4 {
                    self.hand[me][t] -= self.bag[t];
                }
                self.refill(me);
                self.bag = [0; 4];
                self.round += 1;
                self.ph = Ph::Pack;
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.round >= ROUNDS {
            return Some(by_score(self.total(0), self.total(1)));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let base = 100 * (self.total(a) - self.total(1 - a));
        let me = self.merchant();
        match self.ph {
            Ph::Resp if a == me => {
                // probabilitatea de inspecție presupusă scade cu bacșișul
                let q = (55 - 10 * self.bribe).max(10);
                let n = self.size();
                let value: i32 = (0..4).map(|t| self.bag[t] * VAL[t]).sum();
                let honest = self.bad() == 0;
                let insp = if honest {
                    2 * n + value
                } else {
                    -3 * self.bad() + self.bag[self.decl] * VAL[self.decl]
                };
                let pass = value - self.bribe;
                base + (q * insp + (100 - q) * pass)
            }
            Ph::Resolve(inspect) if a != me => {
                let n = self.size();
                // bănuiala vameșului crește cu bacșișul și cu mărimea sacului
                let lie = (30 + 20 * self.bribe + 5 * n).min(90);
                if inspect {
                    base + (lie * 3 * 7 * n / 10 - (100 - lie) * 2 * n)
                } else {
                    base + 100 * self.bribe
                }
            }
            _ => base,
        }
    }
    fn show(&self) -> String {
        let viewer = self.viewer.map(pi).unwrap_or(pi(self.side()));
        let mut s = format!(
            "Vama cu Bacșiș — runda {}/{} | negustor: J{} | monede J1 {}, J2 {} | marfă J1 {}, J2 {}\n",
            (self.round + 1).min(ROUNDS),
            ROUNDS,
            self.merchant() + 1,
            self.coins[0],
            self.coins[1],
            self.goods[0],
            self.goods[1]
        );
        if viewer == self.merchant() {
            let h: Vec<String> = (0..4)
                .map(|t| format!("{}={} ×{}", t, NAMES[t], self.hand[viewer][t]))
                .collect();
            s.push_str(&format!("Mâna ta: {}\n", h.join(", ")));
            if self.ph != Ph::Pack {
                s.push_str(&format!("Sacul tău: {:?}\n", self.bag));
            }
        }
        if matches!(self.ph, Ph::Resp | Ph::Resolve(_)) {
            s.push_str(&format!(
                "Declarat: {} cărți de tipul {} ({}) | bacșiș {}\n",
                self.size(),
                self.decl,
                NAMES[self.decl],
                self.bribe
            ));
        }
        s.push_str(&format!(
            "La mutare: {} (sac a,b,c,d | decl t m | las | inspectez | verific)\n",
            pname(self.side())
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
        crate::brand_util::check(Vama::with_seed, 100);
    }
    #[test]
    fn sheriff_ai_ignores_the_bag_and_hand() {
        let mut g = Vama::with_seed(5);
        g.step("sac 1,0,0,0");
        g.step("decl 0 1");
        let mut h = g.clone();
        h.bag = [0, 0, 0, 1];
        h.hand[0] = [0, 0, 0, 6];
        for seed in 1..6 {
            assert_eq!(ai_of(&g, seed), ai_of(&h, seed));
        }
        let mut v = g.clone();
        v.view(Some(Player::Two));
        assert!(!v.show().contains("Mâna ta"));
        v.view(Some(Player::One));
        assert!(v.show().contains("Mâna ta"));
    }
}

#[cfg(test)]
mod a1_blind {
    use super::*;
    use crate::ai_audit_a1::audit;
    use crate::brand_util::B;

    #[test]
    fn ai_ignores_opponent_hand_bag_and_future_draws_at_all_levels() {
        audit(
            "vama_bacsis",
            |s| B(Vama::with_seed(s)),
            1..=4,
            1,
            40,
            |g, salt| {
                let me = pi(Mini::side(&g.0));
                let mut r = Rng::new(salt);
                let m = g.0.merchant();
                if me != m {
                    // vameșul nu vede mâna negustorului, nici conținutul sacului (doar mărimea)
                    let n = g.0.size();
                    let mut bag = [0i32; 4];
                    for _ in 0..n {
                        bag[r.below(4)] += 1;
                    }
                    let mut hand = bag;
                    while hand.iter().sum::<i32>() < 6 {
                        hand[r.below(4)] += 1;
                    }
                    g.0.bag = bag;
                    g.0.hand[m] = hand;
                } else {
                    let mut hand = [0i32; 4];
                    for _ in 0..6 {
                        hand[r.below(4)] += 1;
                    }
                    g.0.hand[1 - m] = hand;
                }
                g.0.rng = Rng::new(salt ^ 9);
            },
        );
    }
}
