#![allow(
    clippy::needless_range_loop,
    clippy::manual_is_multiple_of,
    clippy::collapsible_if
)]
//! Motor cooperativ „cabina dublă” (lista C1, 2 jucători, variantă originală de zaruri cooperative).
//!
//! În fiecare rundă (mutarea forțată `zar`) fiecare jucător aruncă 2 zaruri, văzute doar de el.
//! J1 apoi J2 plasează zarurile: `pun 1` / `pun 2` pune zarul ales pe AXĂ, celălalt pe MOTOARE
//! (valorile plasate devin publice). Rezolvare (`rezolv`): axa = |axaJ1 − axaJ2| trebuie să fie
//! cel mult `tol`, iar motoarele = suma celor două zaruri de motor trebuie să încapă în fereastra
//! rundei. Fiecare abatere consumă o rezervă; fără rezerve = eșec (REMIZĂ), toate rundele
//! trecute = victoria echipei. `relanc` (de 2 ori în joc, comun) reface zarurile tale înainte de plasare.
//! Informație ascunsă: zarurile neplasate (HOTSEAT_SAFE). AI-ul calculează șansa de reușită doar din
//! zarurile lui și din valorile deja plasate.
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

pub struct SSpec {
    pub title: &'static str,
    pub lo: &'static [i32],
    pub hi: &'static [i32],
    pub tol: i32,
    pub reserve: i32,
    pub rerolls: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ph {
    Roll,
    Place(usize),
    Reroll(usize),
    Resolve,
}

#[derive(Clone)]
pub struct Sky {
    sp: &'static SSpec,
    ph: Ph,
    round: usize,
    dice: [[i32; 2]; 2],
    placed: [Option<(i32, i32)>; 2],
    reserve: i32,
    rerolls: i32,
    end: Option<Outcome>,
    viewer: Option<Player>,
    rng: Rng,
}

fn ok_pair(a: (i32, i32), b: (i32, i32), tol: i32, lo: i32, hi: i32) -> bool {
    (a.0 - b.0).abs() <= tol && (a.1 + b.1) >= lo && (a.1 + b.1) <= hi
}

impl Sky {
    pub fn new(sp: &'static SSpec) -> Self {
        Self::with_seed(sp, seed_now())
    }
    pub fn with_seed(sp: &'static SSpec, seed: u64) -> Self {
        Sky {
            sp,
            ph: Ph::Roll,
            round: 0,
            dice: [[1; 2]; 2],
            placed: [None; 2],
            reserve: sp.reserve,
            rerolls: sp.rerolls,
            end: None,
            viewer: None,
            rng: Rng::new(seed ^ 0x1b87_3593),
        }
    }
    fn roll(&mut self, p: usize) {
        self.dice[p] = [self.rng.d6() as i32, self.rng.d6() as i32];
    }
    fn win(&self) -> (i32, i32) {
        let r = self.round.min(self.sp.lo.len() - 1);
        (self.sp.lo[r], self.sp.hi[r])
    }
    /// Șansa (în mii) ca perechea să reușească, partenerul cooperând optim.
    fn chance(&self, p: usize, mine: (i32, i32)) -> i32 {
        let (lo, hi) = self.win();
        let tol = self.sp.tol;
        if let Some(o) = self.placed[1 - p] {
            return if ok_pair(mine, o, tol, lo, hi) {
                1000
            } else {
                0
            };
        }
        let mut good = 0;
        for a in 1..=6 {
            for b in 1..=6 {
                if ok_pair(mine, (a, b), tol, lo, hi) || ok_pair(mine, (b, a), tol, lo, hi) {
                    good += 1;
                }
            }
        }
        good * 1000 / 36
    }
}

impl Mini for Sky {
    fn side(&self) -> Player {
        match self.ph {
            Ph::Place(p) | Ph::Reroll(p) => {
                if p == 0 {
                    Player::One
                } else {
                    Player::Two
                }
            }
            _ => Player::One,
        }
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            Ph::Roll => vec!["zar".into()],
            Ph::Reroll(_) => vec!["zar".into()],
            Ph::Resolve => vec!["rezolv".into()],
            Ph::Place(_) => {
                let mut v = vec!["pun 1".to_string(), "pun 2".to_string()];
                if self.dice[pi(self.side())][0] == self.dice[pi(self.side())][1] {
                    v.truncate(1);
                }
                if self.rerolls > 0 {
                    v.push("relanc".into());
                }
                v
            }
        }
    }
    fn step(&mut self, m: &str) {
        match self.ph {
            Ph::Roll => {
                self.roll(0);
                self.roll(1);
                self.placed = [None; 2];
                self.ph = Ph::Place(0);
            }
            Ph::Reroll(p) => {
                self.roll(p);
                self.ph = Ph::Place(p);
            }
            Ph::Place(p) => {
                if m == "relanc" {
                    self.rerolls -= 1;
                    self.ph = Ph::Reroll(p);
                    return;
                }
                let k = if m == "pun 1" { 0 } else { 1 };
                self.placed[p] = Some((self.dice[p][k], self.dice[p][1 - k]));
                self.ph = if p == 0 { Ph::Place(1) } else { Ph::Resolve };
            }
            Ph::Resolve => {
                let (lo, hi) = self.win();
                let (a, b) = (self.placed[0].unwrap(), self.placed[1].unwrap());
                if !ok_pair(a, b, self.sp.tol, lo, hi) {
                    self.reserve -= 1;
                }
                self.round += 1;
                if self.reserve < 0 {
                    self.end = Some(Outcome::Draw);
                } else if self.round >= self.sp.lo.len() {
                    self.end = Some(Outcome::Win(Player::One));
                } else {
                    self.ph = Ph::Roll;
                }
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        self.end
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        match self.ph {
            Ph::Reroll(_) => 620 - 80,
            Ph::Place(q) if q == 1 && a == 0 => {
                // J1 a plasat; șansa cu partenerul necunoscut
                self.placed[0].map_or(0, |m| self.chance(0, m))
            }
            Ph::Resolve => {
                let (lo, hi) = self.win();
                match (self.placed[0], self.placed[1]) {
                    (Some(x), Some(y)) if a == 1 => {
                        if ok_pair(x, y, self.sp.tol, lo, hi) {
                            1000
                        } else {
                            0
                        }
                    }
                    (Some(x), _) => self.chance(0, x),
                    _ => 0,
                }
            }
            _ => 0,
        }
    }
    fn show(&self) -> String {
        let (lo, hi) = self.win();
        let mut s = format!(
            "{} — runda {}/{} | rezerve {} | relansări {} | fereastră motoare {}–{} | toleranță axă {}\n",
            self.sp.title,
            (self.round + 1).min(self.sp.lo.len()),
            self.sp.lo.len(),
            self.reserve,
            self.rerolls,
            lo,
            hi,
            self.sp.tol
        );
        let me = self.viewer.map(pi).unwrap_or(pi(self.side()));
        if self.ph != Ph::Roll {
            if self.placed[me].is_none() {
                s.push_str(&format!(
                    "zarurile lui J{} (ascunse celuilalt): {} și {}\n",
                    me + 1,
                    self.dice[me][0],
                    self.dice[me][1]
                ));
            }
            for p in 0..2 {
                if let Some((ax, en)) = self.placed[p] {
                    s.push_str(&format!("J{} a plasat: axă {ax}, motor {en}\n", p + 1));
                }
            }
        }
        s.push_str(&format!(
            "La mutare: {} (pun 1 | pun 2 | relanc; zar / rezolv = forțate)\n",
            pname(self.side())
        ));
        s
    }
    fn view(&mut self, w: Option<Player>) {
        self.viewer = w;
    }
}

#[cfg(test)]
impl Sky {
    pub fn scramble(&mut self, hide: usize, seed: u64) {
        let mut rng = Rng::new(seed);
        if self.placed[hide].is_none() {
            self.dice[hide] = [rng.d6() as i32, rng.d6() as i32];
        }
    }
}
