#![allow(
    clippy::needless_range_loop,
    clippy::manual_is_multiple_of,
    clippy::collapsible_if
)]
//! Motor cooperativ „rezistența tăcută” (lista C1, 2 jucători, variantă originală).
//!
//! În fiecare rundă mutarea forțată `amenint` pune pe masă o amenințare de un tip (0–3). Apoi, pe rând,
//! fiecare joacă `joc i` (o carte de același tip scoate o amenințare; joker `joc i t` alege tipul) sau
//! `pas`; două pasuri la rând închid runda (mutarea forțată `trag`: ambii trag până la mărimea mâinii).
//! Dacă un tip ajunge la `cap` amenințări pe masă, echipa pierde (REMIZĂ); după `rounds` runde echipa
//! câștigă. Informație ascunsă: mâna fiecăruia (HOTSEAT_SAFE). AI-ul decide după masă și mâna proprie.
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

pub struct RSpec {
    pub title: &'static str,
    pub rounds: u32,
    pub cap: u8,
    pub hand: usize,
    pub wild: usize,
    pub per_type: usize,
}

const TYPES: [&str; 4] = ["frig", "ploaie", "gaz", "obuz"];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ph {
    Threat,
    Act,
    Draw,
}

#[derive(Clone)]
pub struct Resist {
    sp: &'static RSpec,
    hand: [Vec<u8>; 2],
    supply: Vec<u8>,
    disc: Vec<u8>,
    table: [u8; 4],
    round: u32,
    ph: Ph,
    side: Player,
    passes: u8,
    end: Option<Outcome>,
    viewer: Option<Player>,
    rng: Rng,
}

impl Resist {
    pub fn new(sp: &'static RSpec) -> Self {
        Self::with_seed(sp, seed_now())
    }
    pub fn with_seed(sp: &'static RSpec, seed: u64) -> Self {
        let mut rng = Rng::new(seed ^ 0x27d4_eb2f);
        let mut supply = Vec::new();
        for t in 0..4u8 {
            for _ in 0..sp.per_type {
                supply.push(t);
            }
        }
        supply.extend(std::iter::repeat_n(4, sp.wild));
        shuf(&mut supply, &mut rng);
        let mut g = Resist {
            sp,
            hand: [Vec::new(), Vec::new()],
            supply,
            disc: Vec::new(),
            table: [0; 4],
            round: 0,
            ph: Ph::Threat,
            side: Player::One,
            passes: 0,
            end: None,
            viewer: None,
            rng,
        };
        g.fill();
        g
    }
    fn fill(&mut self) {
        for p in 0..2 {
            while self.hand[p].len() < self.sp.hand {
                if self.supply.is_empty() {
                    if self.disc.is_empty() {
                        break;
                    }
                    let mut d = std::mem::take(&mut self.disc);
                    shuf(&mut d, &mut self.rng);
                    self.supply = d;
                }
                let c = self.supply.pop().unwrap();
                self.hand[p].push(c);
            }
            self.hand[p].sort();
        }
    }
}

impl Mini for Resist {
    fn side(&self) -> Player {
        self.side
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            Ph::Threat => vec!["amenint".into()],
            Ph::Draw => vec!["trag".into()],
            Ph::Act => {
                let me = pi(self.side);
                let mut v = vec!["pas".to_string()];
                let mut seen = [false; 5];
                for (i, &c) in self.hand[me].iter().enumerate() {
                    if seen[c as usize] {
                        continue;
                    }
                    seen[c as usize] = true;
                    if c < 4 {
                        if self.table[c as usize] > 0 {
                            v.push(format!("joc {}", i + 1));
                        }
                    } else {
                        for t in 0..4 {
                            if self.table[t] > 0 {
                                v.push(format!("joc {} {}", i + 1, t));
                            }
                        }
                    }
                }
                v
            }
        }
    }
    fn step(&mut self, m: &str) {
        match self.ph {
            Ph::Threat => {
                let t = self.rng.below(4);
                self.table[t] += 1;
                if self.table[t] >= self.sp.cap {
                    self.end = Some(Outcome::Draw);
                    return;
                }
                self.ph = Ph::Act;
                self.passes = 0;
                self.side = if self.round % 2 == 0 {
                    Player::One
                } else {
                    Player::Two
                };
            }
            Ph::Draw => {
                self.fill();
                self.round += 1;
                if self.round >= self.sp.rounds {
                    self.end = Some(Outcome::Win(Player::One));
                } else {
                    self.ph = Ph::Threat;
                }
            }
            Ph::Act => {
                let me = pi(self.side);
                if m == "pas" {
                    self.passes += 1;
                    if self.passes >= 2 {
                        self.ph = Ph::Draw;
                    }
                } else {
                    let mut it = m[4..].split(' ');
                    let i: usize = it.next().unwrap().parse::<usize>().unwrap() - 1;
                    let c = self.hand[me].remove(i);
                    let t = it
                        .next()
                        .map(|x| x.parse::<usize>().unwrap())
                        .unwrap_or(c as usize);
                    self.table[t] -= 1;
                    self.disc.push(c);
                    self.passes = 0;
                }
                self.side = self.side.other();
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        self.end
    }
    fn eval(&self, _p: Player) -> i32 {
        let mut s = 0;
        for t in 0..4 {
            let x = self.table[t] as i32;
            s -= 60 * x * x;
        }
        s
    }
    fn show(&self) -> String {
        let me = pi(self.viewer.unwrap_or(self.side));
        let mut s = format!(
            "{} — runda {}/{} | pragul pierderii: {} amenințări de același tip\n",
            self.sp.title,
            self.round + 1,
            self.sp.rounds,
            self.sp.cap
        );
        let t: Vec<String> = (0..4)
            .map(|i| format!("{} {}", TYPES[i], self.table[i]))
            .collect();
        s.push_str(&format!("masa: {}\n", t.join(" | ")));
        let h: Vec<String> = self.hand[me]
            .iter()
            .enumerate()
            .map(|(i, &c)| {
                format!(
                    "{}:{}",
                    i + 1,
                    if c < 4 { TYPES[c as usize] } else { "joker" }
                )
            })
            .collect();
        s.push_str(&format!(
            "mâna lui J{} (ascunsă celuilalt): {}\n",
            me + 1,
            h.join(" ")
        ));
        s.push_str(&format!(
            "La mutare: {} (joc i | joc i tip(0–3) pentru joker | pas; amenint/trag forțate)\n",
            pname(self.side)
        ));
        s
    }
    fn view(&mut self, w: Option<Player>) {
        self.viewer = w;
    }
}

#[cfg(test)]
impl Resist {
    pub fn scramble(&mut self, hide: usize, seed: u64) {
        let mut rng = Rng::new(seed);
        for c in self.hand[hide].iter_mut() {
            *c = rng.below(5) as u8;
        }
        self.hand[hide].sort();
    }
}
