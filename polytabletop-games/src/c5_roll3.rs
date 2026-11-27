//! Motor „zaruri de construcție” (lista C5, 2 jucători): șase zaruri cu cinci resurse și un joker
//! (aur). Trei aruncări (`arunc`, apoi `reia m` pentru zarurile cu bit 1, sau `gata`), apoi cheltui
//! zarurile pe construcții din lanțul propriu (`constr x`): drumuri, așezări (cer drumul cu același
//! număr) și orașe (cer așezarea cu același număr). Tura fără nicio construcție costă 1 glorie.
//! Câștigă primul la `goal` glorie. Zarul e re-însămânțat la `clone()`.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use crate::w7_util::Dice;
use polytabletop_core::{Outcome, Player};

pub struct RollSpec {
    pub title: &'static str,
    pub goal: i32,
    /// costuri pe resurse [lemn, argilă, oaie, grâu, minereu] și glorie, pentru drum / așezare / oraș
    pub road: ([i32; 5], i32),
    pub town: ([i32; 5], i32),
    pub city: ([i32; 5], i32),
    pub n: usize,
    pub cap: u32,
}

const RN: [&str; 6] = ["lemn", "argilă", "oaie", "grâu", "minereu", "aur"];

#[derive(Clone, Copy, PartialEq, Eq)]
enum Ph {
    Roll,
    Keep,
    Build,
}

#[derive(Clone)]
pub struct Roll3 {
    sp: &'static RollSpec,
    built: [Vec<bool>; 2], // 3n: drumuri, așezări, orașe
    vp: [i32; 2],
    d: [u8; 6],
    spent: [bool; 6],
    re: [bool; 6],
    left: u8,
    did: bool,
    ph: Ph,
    side: usize,
    dice: Dice,
    plies: u32,
}

impl Roll3 {
    pub fn new(sp: &'static RollSpec) -> Self {
        Self::with_seed(sp, seed_now())
    }
    pub fn with_seed(sp: &'static RollSpec, seed: u64) -> Self {
        Roll3 {
            sp,
            built: [vec![false; 3 * sp.n], vec![false; 3 * sp.n]],
            vp: [0; 2],
            d: [0; 6],
            spent: [false; 6],
            re: [true; 6],
            left: 3,
            did: false,
            ph: Ph::Roll,
            side: 0,
            dice: Dice::new(seed),
            plies: 0,
        }
    }
    fn item(&self, x: usize) -> ([i32; 5], i32) {
        match x / self.sp.n {
            0 => self.sp.road,
            1 => self.sp.town,
            _ => self.sp.city,
        }
    }
    fn allowed(&self, p: usize, x: usize) -> bool {
        if self.built[p][x] {
            return false;
        }
        let n = self.sp.n;
        let k = x % n;
        match x / n {
            0 => k == 0 || self.built[p][k - 1],
            1 => self.built[p][k],
            _ => self.built[p][n + k],
        }
    }
    fn cnt(&self, only_kept: bool) -> [i32; 6] {
        let mut c = [0; 6];
        for i in 0..6 {
            if self.spent[i] || (only_kept && self.re[i]) {
                continue;
            }
            c[self.d[i] as usize] += 1;
        }
        c
    }
    fn afford(c: &[i32; 6], cost: &[i32; 5]) -> bool {
        let need: i32 = (0..5).map(|i| (cost[i] - c[i]).max(0)).sum();
        need <= c[5]
    }
    fn pay(&mut self, cost: [i32; 5]) {
        for r in 0..5 {
            let mut need = cost[r];
            for i in 0..6 {
                if need > 0 && !self.spent[i] && self.d[i] as usize == r {
                    self.spent[i] = true;
                    need -= 1;
                }
            }
            for i in 0..6 {
                if need > 0 && !self.spent[i] && self.d[i] == 5 {
                    self.spent[i] = true;
                    need -= 1;
                }
            }
        }
    }
    fn end_turn(&mut self) {
        if !self.did {
            self.vp[self.side] = (self.vp[self.side] - 1).max(0);
        }
        self.did = false;
        self.side = 1 - self.side;
        self.left = 3;
        self.re = [true; 6];
        self.spent = [false; 6];
        self.ph = Ph::Roll;
    }
    fn next_goal_value(&self, c: &[f32; 6], p: usize) -> f32 {
        // cât de aproape e cea mai bună construcție permisă, înmulțit cu glorie
        let mut best = 0.0f32;
        for x in 0..3 * self.sp.n {
            if !self.allowed(p, x) {
                continue;
            }
            let (cost, vp) = self.item(x);
            let total: i32 = cost.iter().sum();
            let mut have = 0.0f32;
            let mut gold = c[5];
            for r in 0..5 {
                let h = c[r].min(cost[r] as f32);
                have += h;
                let miss = cost[r] as f32 - h;
                let g = gold.min(miss);
                have += g;
                gold -= g;
            }
            let f = have / total as f32;
            best = best.max(f * f * (vp as f32 + 0.5));
        }
        best
    }
}

impl Mini for Roll3 {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        if self.side == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            Ph::Roll => vec!["arunc".into()],
            Ph::Keep => {
                let mut v = vec!["gata".to_string()];
                if self.left > 0 {
                    for m in 1..64u32 {
                        v.push(format!(
                            "reia {}",
                            (0..6)
                                .map(|i| if m >> i & 1 == 1 { '1' } else { '0' })
                                .collect::<String>()
                        ));
                    }
                }
                v
            }
            Ph::Build => {
                let me = self.side;
                let c = self.cnt(false);
                let mut v = vec!["fin".to_string()];
                for x in 0..3 * self.sp.n {
                    if self.allowed(me, x) && Self::afford(&c, &self.item(x).0) {
                        v.push(format!("constr {}", x + 1));
                    }
                }
                v
            }
        }
    }
    fn step(&mut self, m: &str) {
        self.plies += 1;
        match self.ph {
            Ph::Roll => {
                for i in 0..6 {
                    if self.re[i] {
                        self.d[i] = self.dice.below(6) as u8;
                    }
                }
                self.re = [false; 6];
                self.left -= 1;
                self.ph = if self.left == 0 { Ph::Build } else { Ph::Keep };
            }
            Ph::Keep => {
                if m == "gata" {
                    self.ph = Ph::Build;
                } else {
                    for (i, ch) in m[5..].chars().enumerate() {
                        self.re[i] = ch == '1';
                    }
                    self.ph = Ph::Roll;
                }
            }
            Ph::Build => {
                if m == "fin" {
                    self.end_turn();
                } else {
                    let x = m[7..].parse::<usize>().unwrap() - 1;
                    let (cost, vp) = self.item(x);
                    self.pay(cost);
                    self.built[self.side][x] = true;
                    self.vp[self.side] += vp;
                    self.did = true;
                }
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.ph != Ph::Roll {
            return None;
        }
        if self.vp[0] >= self.sp.goal || self.vp[1] >= self.sp.goal || self.plies >= self.sp.cap {
            return Some(by_score(self.vp[0], self.vp[1]));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let mut v = 100 * (self.vp[a] - self.vp[1 - a]);
        if self.ph == Ph::Roll && self.left < 3 && self.side == a {
            let mut c = [0f32; 6];
            let mut nre = 0.0;
            for i in 0..6 {
                if self.spent[i] {
                    continue;
                }
                if self.re[i] {
                    nre += 1.0;
                } else {
                    c[self.d[i] as usize] += 1.0;
                }
            }
            for f in 0..6 {
                c[f] += nre / 6.0;
            }
            v += (100.0 * self.next_goal_value(&c, a)) as i32;
        }
        v
    }
    fn show(&self) -> String {
        let mut s = format!("{} — țintă {} glorie\n", self.sp.title, self.sp.goal);
        for p in 0..2 {
            let b: Vec<String> = (0..3 * self.sp.n)
                .filter(|&x| self.built[p][x])
                .map(|x| format!("{}{}", ["D", "A", "O"][x / self.sp.n], x % self.sp.n + 1))
                .collect();
            s.push_str(&format!(
                "{}: glorie {} | construit: {}\n",
                pname(if p == 0 { Player::One } else { Player::Two }),
                self.vp[p],
                if b.is_empty() {
                    "-".into()
                } else {
                    b.join(" ")
                }
            ));
        }
        if self.ph != Ph::Roll {
            let t: Vec<String> = (0..6)
                .map(|i| {
                    format!(
                        "{}{}",
                        RN[self.d[i] as usize],
                        if self.spent[i] { "(cheltuit)" } else { "" }
                    )
                })
                .collect();
            s.push_str(&format!(
                "Zaruri (aruncări rămase {}): {}\n",
                self.left,
                t.join(", ")
            ));
        }
        s.push_str("Construcții: D=drum, A=așezare, O=oraș (numerotate 1..n pe fiecare fel; constr x = indice global)\n");
        s.push_str(&format!(
            "La mutare: {} (arunc | reia m | gata | constr x | fin)\n",
            pname(self.side())
        ));
        s
    }
}
