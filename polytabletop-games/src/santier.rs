//! Șantierul Piramidei — pietre pe corăbii spre cinci șantiere (variantă originală, 2 jucători).
//!
//! 5 runde. În fiecare rundă sunt 4 corăbii (capacități 1, 2, 3, 3; pornesc cu minim 1, 1, 2, 2
//! pietre) și 4 șantiere: 1 Piramida, 2 Obeliscul, 3 Templul, 4 Piața. Pe tură: `iau` (+2 pietre în
//! rezerva ta, max 6), `incarc s` (o piatră din rezervă pe corabia s), `navighez s,k` (corabia s,
//! cu destul încărcătură, pleacă la șantierul k liber în runda asta; pietrele se descarcă în ordinea
//! locurilor), sau `pas` când nu ai altceva. Piramida: fiecare piatră descărcată valorează 1, 2, 3, ...
//! (se numără pe tot jocul); Obeliscul: la final n×n/2 puncte pentru n pietre; Templul: 2 puncte
//! pe piatră, dar numai pentru perechile (n/2×2); Piața: 1 punct și 1 piatră în rezervă pe piatră.
//! Runda se încheie când toate corăbiile au plecat sau ambii pasați. Câștigă scorul mai mare.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::{Outcome, Player};

const CAP: [usize; 4] = [1, 2, 3, 3];
const MIN: [usize; 4] = [1, 1, 2, 2];

#[derive(Clone)]
pub struct Santier {
    ship: [Vec<u8>; 4],
    sailed: [bool; 4],
    site: [bool; 4],
    store: [i32; 2],
    pts: [i32; 2],
    pyramid: i32,
    obelisk: [i32; 2],
    temple: [i32; 2],
    round: u32,
    passes: u8,
    side: Player,
    ply: u32,
}

impl Santier {
    pub fn new() -> Self {
        Self::with_seed(0)
    }
    pub fn with_seed(_s: u64) -> Self {
        Santier {
            ship: Default::default(),
            sailed: [false; 4],
            site: [false; 4],
            store: [2, 3],
            pts: [0; 2],
            pyramid: 0,
            obelisk: [0; 2],
            temple: [0; 2],
            round: 0,
            passes: 0,
            side: Player::One,
            ply: 0,
        }
    }
    fn final_pts(&self, p: usize) -> i32 {
        self.pts[p] + self.obelisk[p] * self.obelisk[p] / 2 + (self.temple[p] / 2) * 4
    }
    fn end_round_if_needed(&mut self) {
        if self.sailed.iter().all(|&x| x) || self.passes >= 2 {
            self.round += 1;
            self.ship = Default::default();
            self.sailed = [false; 4];
            self.site = [false; 4];
            self.passes = 0;
            self.store[0] = self.store[0].max(2);
            self.store[1] = self.store[1].max(2);
        }
    }
}

impl Default for Santier {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Santier {
    fn depth(&self) -> u32 {
        3
    }
    fn side(&self) -> Player {
        self.side
    }
    fn moves(&self) -> Vec<String> {
        let me = pi(self.side);
        let mut v = Vec::new();
        if self.store[me] < 6 {
            v.push("iau".into());
        }
        for s in 0..4 {
            if self.sailed[s] {
                continue;
            }
            if self.store[me] > 0 && self.ship[s].len() < CAP[s] {
                v.push(format!("incarc {}", s + 1));
            }
            if self.ship[s].len() >= MIN[s] {
                for k in 0..4 {
                    if !self.site[k] {
                        v.push(format!("navighez {},{}", s + 1, k + 1));
                    }
                }
            }
        }
        v.push("pas".into());
        v
    }
    fn step(&mut self, m: &str) {
        let me = pi(self.side);
        self.ply += 1;
        if m == "pas" {
            self.passes += 1;
        } else {
            self.passes = 0;
            if m == "iau" {
                self.store[me] += 2;
            } else if let Some(s) = m.strip_prefix("incarc ") {
                let s = s.parse::<usize>().unwrap() - 1;
                self.ship[s].push(me as u8);
                self.store[me] -= 1;
            } else if let Some(r) = m.strip_prefix("navighez ") {
                let (s, k) = r.split_once(',').unwrap();
                let (s, k) = (
                    s.parse::<usize>().unwrap() - 1,
                    k.parse::<usize>().unwrap() - 1,
                );
                self.sailed[s] = true;
                self.site[k] = true;
                for &o in &self.ship[s] {
                    let o = o as usize;
                    match k {
                        0 => {
                            self.pyramid += 1;
                            self.pts[o] += self.pyramid;
                        }
                        1 => self.obelisk[o] += 1,
                        2 => self.temple[o] += 1,
                        _ => {
                            self.pts[o] += 1;
                            self.store[o] = (self.store[o] + 1).min(6);
                        }
                    }
                }
            }
        }
        self.side = self.side.other();
        self.end_round_if_needed();
    }
    fn result(&self) -> Option<Outcome> {
        if self.round >= 5 || self.ply > 400 {
            return Some(by_score(self.final_pts(0), self.final_pts(1)));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let on_ships = |i: usize| -> i32 {
            self.ship
                .iter()
                .enumerate()
                .filter(|(s, _)| !self.sailed[*s])
                .map(|(_, v)| v.iter().filter(|&&o| o as usize == i).count() as i32)
                .sum()
        };
        10 * (self.final_pts(a) - self.final_pts(1 - a))
            + 2 * (self.store[a] - self.store[1 - a])
            + 3 * (on_ships(a) - on_ships(1 - a))
    }
    fn show(&self) -> String {
        let mut s = format!(
            "Șantierul Piramidei — runda {}/5 | piramida {} pietre\n",
            (self.round + 1).min(5),
            self.pyramid
        );
        for k in 0..4 {
            let c: Vec<String> = self.ship[k]
                .iter()
                .map(|&o| format!("J{}", o + 1))
                .collect();
            s.push_str(&format!(
                "Corabia {} (loc {}, min {}): [{}]{}\n",
                k + 1,
                CAP[k],
                MIN[k],
                c.join(" "),
                if self.sailed[k] { " a plecat" } else { "" }
            ));
        }
        for p in 0..2 {
            s.push_str(&format!(
                "J{}: rezervă {} | puncte {} | obelisc {} | templu {}\n",
                p + 1,
                self.store[p],
                self.pts[p],
                self.obelisk[p],
                self.temple[p]
            ));
        }
        s.push_str(&format!(
            "La mutare: {} (iau | incarc s | navighez s,k | pas)\n",
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
        crate::brand_util::check(Santier::with_seed, 600);
    }
}
