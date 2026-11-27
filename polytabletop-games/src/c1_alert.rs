#![allow(
    clippy::needless_range_loop,
    clippy::manual_is_multiple_of,
    clippy::collapsible_if
)]
//! Motor cooperativ „sala de comandă” (lista C1, 2 jucători, variantă originală, fără ceas real).
//!
//! O navă cu 3 zone. Amenințările (`zona`, distanță, rezistență, lovitură) apar prin mutarea forțată
//! `ev` și înaintează spre navă. Fiecare jucător are 2 acțiuni pe rundă: `tras z` (cea mai apropiată
//! amenințare din zona z pierde 1 rezistență), `scut z` (zona z absoarbe lovitura rundei), `repar`
//! (+1 integritate, cel mult o dată pe rundă) sau `pas`. La `ev` amenințările aflate la distanța 1
//! lovesc (scutul le oprește), restul se apropie, apar noi amenințări (`spawn`/100 în medie). Integritatea 0 =
//! eșec (REMIZĂ); după `rounds` runde echipa câștigă. Totul e public în afară de viitoarele apariții.
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

pub struct ASpec {
    pub title: &'static str,
    pub rounds: u32,
    pub hull: i32,
    pub spawn: usize,
    pub start_dist: i32,
}

#[derive(Clone)]
pub struct Alert {
    sp: &'static ASpec,
    /// (zonă, distanță, rezistență, lovitură)
    thr: Vec<(usize, i32, i32, i32)>,
    shield: [bool; 3],
    hull: i32,
    round: u32,
    side: Player,
    acts: u8,
    repaired: bool,
    pend: bool,
    end: Option<Outcome>,
    rng: Rng,
}

impl Alert {
    pub fn new(sp: &'static ASpec) -> Self {
        Self::with_seed(sp, seed_now())
    }
    pub fn with_seed(sp: &'static ASpec, seed: u64) -> Self {
        let mut rng = Rng::new(seed ^ 0x165a_b2d1);
        let z = rng.below(3);
        Alert {
            sp,
            thr: vec![(z, sp.start_dist, 1, 1)],
            shield: [false; 3],
            hull: sp.hull,
            round: 0,
            side: Player::One,
            acts: 2,
            repaired: false,
            pend: false,
            end: None,
            rng,
        }
    }
    fn nearest(&self, z: usize) -> Option<usize> {
        (0..self.thr.len())
            .filter(|&i| self.thr[i].0 == z)
            .min_by_key(|&i| self.thr[i].1)
    }
    fn imminent(&self) -> i32 {
        self.thr
            .iter()
            .filter(|t| t.1 <= 1 && !self.shield[t.0])
            .map(|t| t.3)
            .sum()
    }
}

impl Mini for Alert {
    fn side(&self) -> Player {
        self.side
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        if self.pend {
            return vec!["ev".into()];
        }
        let mut v = vec!["pas".to_string()];
        for z in 0..3 {
            if self.nearest(z).is_some() {
                v.push(format!("tras {z}"));
            }
            if !self.shield[z] {
                v.push(format!("scut {z}"));
            }
        }
        if !self.repaired && self.hull < self.sp.hull {
            v.push("repar".into());
        }
        v
    }
    fn step(&mut self, m: &str) {
        if m == "ev" {
            let mut keep = Vec::new();
            let thr = std::mem::take(&mut self.thr);
            for mut t in thr {
                if t.1 <= 1 {
                    if !self.shield[t.0] {
                        self.hull -= t.3;
                    }
                } else {
                    t.1 -= 1;
                    keep.push(t);
                }
            }
            self.thr = keep;
            let mut k = self.sp.spawn / 100;
            if self.rng.below(100) < self.sp.spawn % 100 {
                k += 1;
            }
            for _ in 0..k {
                let z = self.rng.below(3);
                let hp = 1 + self.rng.below(2) as i32;
                let dm = 1 + self.rng.below(2) as i32;
                self.thr.push((z, self.sp.start_dist, hp, dm));
            }
            self.shield = [false; 3];
            self.repaired = false;
            self.round += 1;
            self.side = Player::One;
            self.acts = 2;
            self.pend = false;
            if self.hull <= 0 {
                self.end = Some(Outcome::Draw);
            } else if self.round >= self.sp.rounds {
                self.end = Some(Outcome::Win(Player::One));
            }
            return;
        }
        if let Some(z) = m.strip_prefix("tras ") {
            let z: usize = z.parse().unwrap();
            if let Some(i) = self.nearest(z) {
                self.thr[i].2 -= 1;
                if self.thr[i].2 <= 0 {
                    self.thr.remove(i);
                }
            }
        } else if let Some(z) = m.strip_prefix("scut ") {
            self.shield[z.parse::<usize>().unwrap()] = true;
        } else if m == "repar" {
            self.hull += 1;
            self.repaired = true;
        }
        self.acts -= if m == "pas" { self.acts } else { 1 };
        if self.acts == 0 {
            if self.side == Player::One {
                self.side = Player::Two;
                self.acts = 2;
            } else {
                self.pend = true;
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        self.end
    }
    fn eval(&self, _p: Player) -> i32 {
        let mut s = 200 * self.hull - 600 * self.imminent();
        for t in &self.thr {
            s -= 30 * t.2 * t.3 * (self.sp.start_dist + 1 - t.1);
        }
        // scutul inutil costă ceva
        for z in 0..3 {
            if self.shield[z] {
                s -= 10;
            }
        }
        s
    }
    fn show(&self) -> String {
        let mut s = format!(
            "{} — runda {}/{} | integritate {}/{}\n",
            self.sp.title,
            self.round + 1,
            self.sp.rounds,
            self.hull,
            self.sp.hull
        );
        for z in 0..3 {
            let t: Vec<String> = self
                .thr
                .iter()
                .filter(|t| t.0 == z)
                .map(|t| format!("(dist {}, rez {}, lov {})", t.1, t.2, t.3))
                .collect();
            s.push_str(&format!(
                "  zona {z}{}: {}\n",
                if self.shield[z] { " [scut]" } else { "" },
                t.join(" ")
            ));
        }
        s.push_str(&format!(
            "La mutare: {} ({} acțiuni: tras z | scut z | repar | pas; ev = sfârșit de rundă)\n",
            pname(self.side),
            self.acts
        ));
        s
    }
}
