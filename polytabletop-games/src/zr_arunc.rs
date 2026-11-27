//! Două jocuri „de îndemânare simulată” (lista C6, variante originale): `zar_peste_masa` și
//! `coridorul_incercarilor`. Îndemânarea e înlocuită cu probabilități publice; rezultatul se
//! află doar în mutarea forțată `tira`, deci AI-ul (adâncime 0) nu poate trișa.
#![allow(clippy::needless_range_loop, clippy::manual_is_multiple_of)]
use crate::zr_util::*;

// ---------------------------------------------------------------- zar_peste_masa
const BP: [i32; 3] = [70, 50, 30];
const BR: [i32; 3] = [1, 2, 4];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum BPh {
    Aim,
    Guard,
    Roll,
}

#[derive(Clone)]
pub struct ZarMasa {
    rng: u64,
    side: usize,
    ph: BPh,
    aim: usize,
    guarded: bool,
    charges: [u8; 2],
    pts: [i32; 2],
    turns: u32,
    last: String,
}

fn beer_ev(aim: usize, guarded: bool) -> i32 {
    let p = (BP[aim] - if guarded { 20 } else { 0 }).max(5);
    let pen = if aim == 2 { 1 } else { 0 };
    // în miimi de punct
    p * 10 * BR[aim] - (100 - p) * 10 * pen
}

impl ZarMasa {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        ZarMasa {
            rng: seed_state(seed),
            side: 0,
            ph: BPh::Aim,
            aim: 0,
            guarded: false,
            charges: [2, 2],
            pts: [0; 2],
            turns: 0,
            last: String::new(),
        }
    }
}

impl Default for ZarMasa {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for ZarMasa {
    fn side(&self) -> Player {
        pl(match self.ph {
            BPh::Guard => 1 - self.side,
            _ => self.side,
        })
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            BPh::Aim => vec!["usor".into(), "tintit".into(), "risc".into()],
            BPh::Guard => {
                let mut v = vec!["liber".to_string()];
                if self.charges[1 - self.side] > 0 {
                    v.push("garda".into());
                }
                v
            }
            BPh::Roll => vec!["tira".into()],
        }
    }
    fn step(&mut self, m: &str) {
        match self.ph {
            BPh::Aim => {
                self.aim = match m {
                    "usor" => 0,
                    "tintit" => 1,
                    _ => 2,
                };
                self.ph = BPh::Guard;
            }
            BPh::Guard => {
                self.guarded = m == "garda";
                if self.guarded {
                    self.charges[1 - self.side] -= 1;
                }
                self.ph = BPh::Roll;
            }
            BPh::Roll => {
                let p = (BP[self.aim] - if self.guarded { 20 } else { 0 }).max(5);
                let hit = (die(&mut self.rng, 100) as i32) < p;
                if hit {
                    self.pts[self.side] += BR[self.aim];
                    self.last = "lovit".into();
                } else {
                    if self.aim == 2 {
                        self.pts[1 - self.side] += 1;
                    }
                    self.last = "ratat".into();
                }
                self.ph = BPh::Aim;
                self.side = 1 - self.side;
                self.turns += 1;
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.pts[0] >= 12 || self.pts[1] >= 12 || self.turns >= 60 {
            return Some(by_score(self.pts[0], self.pts[1]));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let mut e = 1000 * (self.pts[a] - self.pts[1 - a])
            + 300 * (self.charges[a] as i32 - self.charges[1 - a] as i32);
        let sgn = if self.side == a { 1 } else { -1 };
        match self.ph {
            BPh::Guard => {
                let ug = beer_ev(self.aim, false);
                let d = 1 - self.side;
                let v = if self.charges[d] > 0 {
                    ug.min(beer_ev(self.aim, true) + 300)
                } else {
                    ug
                };
                e += sgn * v;
            }
            BPh::Roll => e += sgn * beer_ev(self.aim, self.guarded),
            BPh::Aim => {}
        }
        e
    }
    fn show(&self) -> String {
        format!(
            "Zar peste masă — puncte J1 {}, J2 {} (la 12 câștigi)\nGărzi rămase: J1 {}, J2 {}\nAruncă: {} | faza {:?} | ținta {} | ultima: {}\nMutări: usor (70%, +1) | tintit (50%, +2) | risc (30%, +4, ratarea dă 1 adversarului) | garda/liber | tira\n",
            self.pts[0],
            self.pts[1],
            self.charges[0],
            self.charges[1],
            pname(pl(self.side)),
            self.ph,
            self.aim + 1,
            self.last
        )
    }
}

// ---------------------------------------------------------------- coridorul_incercarilor
const MON: [i32; 5] = [3, 4, 5, 4, 6];
const AP: [i32; 3] = [85, 60, 35];
const AD: [i32; 3] = [1, 2, 3];

#[derive(Clone)]
pub struct Coridor {
    rng: u64,
    side: usize,
    idx: usize,
    hp: i32,
    att: Option<usize>,
    pts: [i32; 2],
    last: String,
}

impl Coridor {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        Coridor {
            rng: seed_state(seed),
            side: 0,
            idx: 0,
            hp: MON[0],
            att: None,
            pts: [0; 2],
            last: String::new(),
        }
    }
    fn reward(&self) -> i32 {
        MON[self.idx.min(4)]
    }
    /// valoarea (în miimi) pentru cel care atacă, după efectul așteptat al atacului `a`
    fn ev(&self, a: usize) -> i32 {
        let p = AP[a];
        let d = AD[a];
        let succ = if d >= self.hp {
            self.reward() * 1000
        } else {
            let h = self.hp - d;
            // adversarul poate încerca imediat să dea lovitura finală
            let opp = (0..3)
                .filter(|&b| AD[b] >= h)
                .map(|b| AP[b] * self.reward() * 10)
                .max()
                .unwrap_or(0);
            -opp
        };
        let fail = if a == 2 { -1000 } else { 0 };
        (p * succ + (100 - p) * fail) / 100
    }
}

impl Default for Coridor {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Coridor {
    fn side(&self) -> Player {
        pl(self.side)
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        if self.att.is_none() {
            vec!["usor".into(), "mediu".into(), "greu".into()]
        } else {
            vec!["tira".into()]
        }
    }
    fn step(&mut self, m: &str) {
        let Some(a) = self.att else {
            self.att = Some(match m {
                "usor" => 0,
                "mediu" => 1,
                _ => 2,
            });
            return;
        };
        self.att = None;
        let hit = (die(&mut self.rng, 100) as i32) < AP[a];
        if hit {
            self.hp -= AD[a];
            self.last = format!("lovit ({})", AD[a]);
            if self.hp <= 0 {
                self.pts[self.side] += self.reward();
                self.idx += 1;
                if self.idx < MON.len() {
                    self.hp = MON[self.idx];
                }
            }
        } else {
            self.last = "ratat".into();
            if a == 2 {
                self.pts[self.side] -= 1;
            }
        }
        self.side = 1 - self.side;
    }
    fn result(&self) -> Option<Outcome> {
        if self.idx >= MON.len() {
            return Some(by_score(self.pts[0], self.pts[1]));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let mut e = 1000 * (self.pts[a] - self.pts[1 - a]);
        if let Some(k) = self.att {
            let v = self.ev(k);
            e += if self.side == a { v } else { -v };
        }
        e
    }
    fn show(&self) -> String {
        format!(
            "Coridorul încercărilor — monstrul {}/{} (viață {}, recompensă {})\nPuncte: J1 {}, J2 {} | la mutare {} | ultima: {}\nMutări: usor (85%, 1) | mediu (60%, 2) | greu (35%, 3; ratarea costă 1) | tira\n",
            (self.idx + 1).min(MON.len()),
            MON.len(),
            self.hp.max(0),
            self.reward(),
            self.pts[0],
            self.pts[1],
            pname(pl(self.side)),
            self.last
        )
    }
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn zar_masa() {
        check(ZarMasa::with_seed, 600);
        let mut g = ZarMasa::with_seed(3);
        advance(&mut g, 30, |m| m.ph == BPh::Roll);
        no_peek(&g, |m| m.rng ^= 0xFEDCBA987);
    }
    #[test]
    fn coridor() {
        check(Coridor::with_seed, 600);
        let mut g = Coridor::with_seed(3);
        advance(&mut g, 30, |m| m.att.is_some());
        no_peek(&g, |m| m.rng ^= 0xFEDCBA987);
    }
}
