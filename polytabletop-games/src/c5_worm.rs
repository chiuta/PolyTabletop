//! Motor „viermișori pe grătar” (lista C5, 2 jucători): opt zaruri cu fețele 1–5 și o față „vierme”
//! (valorează 5). Aruncarea forțată `arunc`; alegi o față nefolosită încă în tură (`alege f`) și
//! pui deoparte toate zarurile cu ea; apoi `continu` (rămase zaruri) sau `opresc`. Trebuie `lo`
//! puncte și cel puțin un vierme. Iei de pe grătar (public) piesa exactă, sau pe cea mai mare mai
//! mică; poți fura piesa de sus a adversarului dacă are exact suma ta. Altfel (bust) îți pierzi piesa
//! de sus (revine pe grătar) și cea mai mare piesă a grătarului se scoate. Câștigă cine are mai mulți viermi.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use crate::w7_util::Dice;
use polytabletop_core::{Outcome, Player};

pub struct WormSpec {
    pub title: &'static str,
    pub lo: i32,
    pub hi: i32,
    pub dice: usize,
    /// Numărul de piese pe „valoare vierme” (1..=4): piesa t are 1 + (t − lo) * 4 / (hi − lo + 1) viermi.
    pub cap: u32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Ph {
    Roll,
    Pick,
    Decide,
}

#[derive(Clone)]
pub struct Worm {
    sp: &'static WormSpec,
    row: Vec<i32>,
    stack: [Vec<i32>; 2],
    kept: [u8; 6],
    used: [bool; 6],
    cur: Vec<u8>,
    ph: Ph,
    side: usize,
    dice: Dice,
    plies: u32,
}

fn fv(f: usize) -> i32 {
    if f == 5 {
        5
    } else {
        f as i32 + 1
    }
}

impl Worm {
    pub fn new(sp: &'static WormSpec) -> Self {
        Self::with_seed(sp, seed_now())
    }
    pub fn with_seed(sp: &'static WormSpec, seed: u64) -> Self {
        Worm {
            sp,
            row: (sp.lo..=sp.hi).collect(),
            stack: [Vec::new(), Vec::new()],
            kept: [0; 6],
            used: [false; 6],
            cur: Vec::new(),
            ph: Ph::Roll,
            side: 0,
            dice: Dice::new(seed),
            plies: 0,
        }
    }
    fn worms(&self, t: i32) -> i32 {
        1 + (t - self.sp.lo) * 4 / (self.sp.hi - self.sp.lo + 1)
    }
    fn sum(&self) -> i32 {
        (0..6).map(|f| self.kept[f] as i32 * fv(f)).sum()
    }
    fn left(&self) -> usize {
        self.sp.dice - self.kept.iter().map(|&x| x as usize).sum::<usize>()
    }
    fn score(&self, s: usize) -> i32 {
        self.stack[s].iter().map(|&t| self.worms(t)).sum()
    }
    /// Piesa pe care ai lua-o cu suma curentă (dacă e validă).
    fn grab(&self) -> Option<(i32, bool)> {
        let t = self.sum();
        if t < self.sp.lo || self.kept[5] == 0 {
            return None;
        }
        if self.row.contains(&t) {
            return Some((t, false));
        }
        if self.stack[1 - self.side].last() == Some(&t) {
            return Some((t, true));
        }
        self.row
            .iter()
            .filter(|&&x| x <= t)
            .max()
            .map(|&x| (x, false))
    }
    fn bust(&mut self) {
        let me = self.side;
        if let Some(t) = self.stack[me].pop() {
            self.row.push(t);
            self.row.sort();
        }
        self.row.pop();
        self.finish_turn();
    }
    fn finish_turn(&mut self) {
        self.kept = [0; 6];
        self.used = [false; 6];
        self.cur.clear();
        self.ph = Ph::Roll;
        self.side = 1 - self.side;
    }
}

impl Mini for Worm {
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
            Ph::Pick => {
                let mut v = Vec::new();
                for f in 0..6 {
                    if !self.used[f] && self.cur.contains(&(f as u8)) {
                        v.push(format!(
                            "alege {}",
                            if f == 5 {
                                "v".to_string()
                            } else {
                                (f + 1).to_string()
                            }
                        ));
                    }
                }
                if v.is_empty() {
                    v.push("bust".into());
                }
                v
            }
            Ph::Decide => {
                let mut v = Vec::new();
                if self.left() > 0 {
                    v.push("continu".to_string());
                }
                if self.grab().is_some() {
                    v.push("opresc".into());
                }
                if v.is_empty() {
                    v.push("bust".into());
                }
                v
            }
        }
    }
    fn step(&mut self, m: &str) {
        self.plies += 1;
        match self.ph {
            Ph::Roll => {
                let n = self.left();
                self.cur = (0..n).map(|_| self.dice.below(6) as u8).collect();
                self.ph = Ph::Pick;
            }
            Ph::Pick => {
                if m == "bust" {
                    self.bust();
                    return;
                }
                let f = match &m[6..] {
                    "v" => 5,
                    x => x.parse::<usize>().unwrap() - 1,
                };
                let c = self.cur.iter().filter(|&&x| x as usize == f).count();
                self.kept[f] += c as u8;
                self.used[f] = true;
                self.cur.clear();
                self.ph = Ph::Decide;
            }
            Ph::Decide => match m {
                "continu" => self.ph = Ph::Roll,
                "opresc" => {
                    let (t, steal) = self.grab().unwrap();
                    let me = self.side;
                    if steal {
                        self.stack[1 - me].pop();
                    } else {
                        self.row.retain(|&x| x != t);
                    }
                    self.stack[me].push(t);
                    self.finish_turn();
                }
                _ => self.bust(),
            },
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.row.is_empty() || self.plies >= self.sp.cap {
            let (a, b) = (self.score(0), self.score(1));
            return Some(by_score(a, b));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let base = 1000 * (self.score(a) - self.score(1 - a));
        let sgn = if self.side == a { 1 } else { -1 };
        let left = self.left().min(8);
        let (valid, w_now) = match self.grab() {
            Some((t, steal)) => (true, self.worms(t) * if steal { 2 } else { 1 }),
            None => (false, 0),
        };
        let stop_v = if valid { 1000 * w_now } else { -1000 };
        let cont_v = if left == 0 {
            stop_v
        } else if valid {
            let pv = [0, 10, 30, 50, 65, 78, 88, 94, 97][left];
            (pv * (w_now * 1000 + 500) - (100 - pv) * 1000) / 100
        } else {
            let po = [0, 5, 15, 30, 50, 65, 80, 90, 95][left];
            (po * 2000 - (100 - po) * 1000) / 100
        };
        match self.ph {
            Ph::Decide => base + sgn * stop_v.max(cont_v),
            Ph::Roll if self.sum() > 0 => base + sgn * cont_v,
            _ => base,
        }
    }
    fn show(&self) -> String {
        let mut s = format!("{} — grătarul: ", self.sp.title);
        s.push_str(
            &self
                .row
                .iter()
                .map(|t| format!("{t}({})", self.worms(*t)))
                .collect::<Vec<_>>()
                .join(" "),
        );
        s.push('\n');
        for p in 0..2 {
            s.push_str(&format!(
                "{}: viermi {} | piese {:?}\n",
                pname(if p == 0 { Player::One } else { Player::Two }),
                self.score(p),
                self.stack[p]
            ));
        }
        let kept: Vec<String> = (0..6)
            .filter(|&f| self.kept[f] > 0)
            .map(|f| {
                format!(
                    "{}×{}",
                    self.kept[f],
                    if f == 5 {
                        "v".to_string()
                    } else {
                        (f + 1).to_string()
                    }
                )
            })
            .collect();
        s.push_str(&format!(
            "Puse deoparte: {} (suma {})\n",
            kept.join(" "),
            self.sum()
        ));
        if !self.cur.is_empty() {
            let c: Vec<String> = self
                .cur
                .iter()
                .map(|&f| {
                    if f == 5 {
                        "v".to_string()
                    } else {
                        (f + 1).to_string()
                    }
                })
                .collect();
            s.push_str(&format!("Aruncate: {}\n", c.join(" ")));
        }
        s.push_str(&format!(
            "La mutare: {} (arunc | alege f | continu | opresc | bust)\n",
            pname(self.side())
        ));
        s
    }
}
