//! Duel cu Gloanțe Ascunse — alegeri simultane (variantă originală, 2 jucători).
//!
//! Fiecare jucător are un încărcător de 8 cărți în ordine ascunsă (5 încărcate, 3 oarbe), pe care îl vede doar el.
//! Câte 8 runde, fiecare alege în secret `foc` (trage cartea de sus în adversar), `scut` (te aperi; maximum 2 în joc)
//! sau `pas`. Mutarea forțată `rezolv` dezvăluie: un foc cu carte încărcată rănește adversarul dacă acesta nu
//! s-a ferit cu scutul (cărțile trase devin publice). 3 vieți fiecare; la final câștigă cine are mai multe vieți.
//! Informație ascunsă: ordinea propriului încărcător și alegerea primului; AI-ul nu le citește pe ale adversarului.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const MAG: usize = 8;
const LIVE: usize = 5;
const ROUNDS: u32 = 8;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ph {
    First,
    Second,
    Resolve,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Act {
    Foc,
    Scut,
    Pas,
}

#[derive(Clone)]
pub struct Duel {
    viewer: Option<Player>,
    mag: [Vec<bool>; 2],
    idx: [usize; 2],
    lives: [i32; 2],
    shields: [i32; 2],
    fired_live: [usize; 2],
    choice: [Option<Act>; 2],
    ph: Ph,
    round: u32,
}

impl Duel {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut m: [Vec<bool>; 2] = [Vec::new(), Vec::new()];
        for p in 0..2 {
            let mut v: Vec<bool> = (0..MAG).map(|i| i < LIVE).collect();
            shuf(&mut v, &mut rng);
            m[p] = v;
        }
        Duel {
            viewer: None,
            mag: m,
            idx: [0; 2],
            lives: [3; 2],
            shields: [2; 2],
            fired_live: [0; 2],
            choice: [None; 2],
            ph: Ph::First,
            round: 0,
        }
    }
    /// Probabilitatea (în sutimi) ca următoarea carte a lui `p` să fie încărcată, din informația publică.
    fn p_live(&self, p: usize) -> i32 {
        let left = MAG - self.idx[p];
        if left == 0 {
            return 0;
        }
        100 * (LIVE - self.fired_live[p]) as i32 / left as i32
    }
}

impl Default for Duel {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Duel {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        match self.ph {
            Ph::First | Ph::Resolve => Player::One,
            Ph::Second => Player::Two,
        }
    }
    fn moves(&self) -> Vec<String> {
        if self.ph == Ph::Resolve {
            return vec!["rezolv".into()];
        }
        let p = pi(self.side());
        let mut v = vec!["pas".to_string()];
        if self.idx[p] < MAG {
            v.push("foc".into());
        }
        if self.shields[p] > 0 {
            v.push("scut".into());
        }
        v
    }
    fn step(&mut self, m: &str) {
        match self.ph {
            Ph::First | Ph::Second => {
                let p = pi(self.side());
                self.choice[p] = Some(match m {
                    "foc" => Act::Foc,
                    "scut" => Act::Scut,
                    _ => Act::Pas,
                });
                self.ph = if self.ph == Ph::First {
                    Ph::Second
                } else {
                    Ph::Resolve
                };
            }
            Ph::Resolve => {
                let ch = [self.choice[0].unwrap(), self.choice[1].unwrap()];
                let mut dmg = [0; 2];
                for p in 0..2 {
                    if ch[p] == Act::Scut {
                        self.shields[p] -= 1;
                    }
                    if ch[p] == Act::Foc {
                        let live = self.mag[p][self.idx[p]];
                        self.idx[p] += 1;
                        if live {
                            self.fired_live[p] += 1;
                            if ch[1 - p] != Act::Scut {
                                dmg[1 - p] += 1;
                            }
                        }
                    }
                }
                for p in 0..2 {
                    self.lives[p] -= dmg[p];
                }
                self.choice = [None; 2];
                self.round += 1;
                self.ph = Ph::First;
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.lives[0] <= 0 || self.lives[1] <= 0 || self.round >= ROUNDS {
            return Some(match self.lives[0].cmp(&self.lives[1]) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                std::cmp::Ordering::Equal => Outcome::Draw,
            });
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let base = 1000 * (self.lives[a] - self.lives[1 - a]);
        let Some(c) = self.choice[a] else { return base };
        let o = 1 - a;
        // presupunem că adversarul trage ~70% din timp și se ferește ~20% dacă mai are scuturi
        let opp_hits = 70 * self.p_live(o) / 100;
        let opp_shield = if self.shields[o] > 0 { 20 } else { 0 };
        match c {
            Act::Foc if self.mag[a][self.idx[a]] => base + 10 * (100 - opp_shield),
            Act::Foc => base + 100,
            Act::Scut => base + 10 * opp_hits - 150,
            Act::Pas => base,
        }
    }
    fn show(&self) -> String {
        let viewer = self.viewer.map(pi).unwrap_or(pi(self.side()));
        let mag: String = (self.idx[viewer]..MAG)
            .map(|i| if self.mag[viewer][i] { 'I' } else { 'o' })
            .collect();
        format!(
            "Duel cu Gloanțe Ascunse — runda {}/{} | vieți J1 {}, J2 {} | scuturi J1 {}, J2 {}\nCărți trase încărcate: J1 {}, J2 {} | rămase în încărcătorul fiecăruia: J1 {}, J2 {}\nÎncărcătorul tău (J{}), de sus în jos: {} (I = încărcată, o = oarbă)\nLa mutare: {} (foc | scut | pas | rezolv)\n",
            (self.round + 1).min(ROUNDS),
            ROUNDS,
            self.lives[0],
            self.lives[1],
            self.shields[0],
            self.shields[1],
            self.fired_live[0],
            self.fired_live[1],
            MAG - self.idx[0],
            MAG - self.idx[1],
            viewer + 1,
            mag,
            pname(self.side())
        )
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
        crate::brand_util::check(Duel::with_seed, 100);
    }
    #[test]
    fn ai_ignores_the_other_magazine_and_choice() {
        let mut g = Duel::with_seed(2);
        g.step("foc");
        let mut h = g.clone();
        h.mag[0].reverse();
        h.choice[0] = Some(Act::Scut);
        for seed in 1..6 {
            assert_eq!(ai_of(&g, seed), ai_of(&h, seed));
        }
        let mut v = g.clone();
        v.view(Some(Player::Two));
        assert!(v.show().contains("Încărcătorul tău (J2)"));
        assert!(!v.show().contains("Încărcătorul tău (J1)"));
    }
}
