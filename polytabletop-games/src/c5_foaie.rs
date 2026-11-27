//! Motor „foaia isteță” (lista C5, 2 jucători): șase zaruri (cinci colorate + unul alb). Cel la rând
//! aruncă forțat (`arunc`), apoi alege până la trei zaruri (`alege d a`: zarul d în zona a; zarul
//! colorat merge doar în zona lui, cel alb în orice zonă); zarurile mai mici decât cel ales și cele
//! rămase la sfârșit trec pe tava de argint, de unde celălalt alege un singur zar (`alege d a`).
//! Cinci tipuri de zone (încrucișare, sumă, urcare, multiplicator, șir) cu parametri proprii fiecărui joc.
//! Zarul e re-însămânțat la `clone()`; informație publică.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use crate::w7_util::Dice;
use polytabletop_core::{Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum K {
    /// o căsuță pentru fiecare valoare 1..6
    Cross,
    /// zar + zarul alb (valoarea lui la aruncare): căsuță pentru fiecare sumă 2..12
    Sum,
    /// de la stânga la dreapta, valoarea trebuie să atingă pragul căsuței
    Climb,
    /// de la stânga la dreapta, scorul = valoare × multiplicatorul căsuței
    Mult,
    /// de la stânga la dreapta, strict crescător; după un 6 se reia de la zero
    Run,
}

pub struct Area {
    pub kind: K,
    pub name: &'static str,
    /// Praguri (Climb) sau multiplicatori (Mult), altfel gol; lungimea zonei = max(len, praguri).
    pub par: &'static [i32],
    pub len: usize,
}

pub struct FoaieSpec {
    pub title: &'static str,
    pub areas: [Area; 5],
    pub turns: u32,
    pub full_bonus: i32,
    pub picks: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Ph {
    Roll,
    Act,
    Pass,
}

#[derive(Clone)]
pub struct Foaie {
    sp: &'static FoaieSpec,
    sheet: [[Vec<u8>; 5]; 2],
    d: [u8; 6],
    avail: [bool; 6],
    taken: [bool; 6],
    picks: usize,
    ph: Ph,
    turn: u32,
    side: usize,
    dice: Dice,
}

impl Foaie {
    pub fn new(sp: &'static FoaieSpec) -> Self {
        Self::with_seed(sp, seed_now())
    }
    pub fn with_seed(sp: &'static FoaieSpec, seed: u64) -> Self {
        let mk = || [vec![], vec![], vec![], vec![], vec![]];
        Foaie {
            sp,
            sheet: [mk(), mk()],
            d: [1; 6],
            avail: [false; 6],
            taken: [false; 6],
            picks: 0,
            ph: Ph::Roll,
            turn: 0,
            side: 0,
            dice: Dice::new(seed),
        }
    }
    fn who(&self) -> usize {
        if self.ph == Ph::Pass {
            1 - self.side
        } else {
            self.side
        }
    }
    /// Încearcă să pună valoarea `v` în zona `a` a lui `p`; întoarce valoarea de stocat.
    fn can(&self, p: usize, a: usize, v: u8, white: bool) -> Option<u8> {
        let ar = &self.sp.areas[a];
        let sh = &self.sheet[p][a];
        match ar.kind {
            K::Cross => (!sh.contains(&v)).then_some(v),
            K::Sum => {
                let s = v + if white { v } else { self.d[5] };
                (!sh.contains(&s)).then_some(s)
            }
            K::Climb => {
                let i = sh.len();
                (i < ar.len && v as i32 >= ar.par[i.min(ar.par.len() - 1)]).then_some(v)
            }
            K::Mult => (sh.len() < ar.len).then_some(v),
            K::Run => {
                (sh.len() < ar.len && (sh.last().is_none_or(|&l| l == 6 || v > l))).then_some(v)
            }
        }
    }
    fn area_score(&self, p: usize, a: usize) -> i32 {
        let ar = &self.sp.areas[a];
        let sh = &self.sheet[p][a];
        let n = sh.len() as i32;
        let base = match ar.kind {
            K::Cross | K::Climb => n * (n + 1) / 2,
            K::Sum => n + 2 * (n / 4),
            K::Mult => sh
                .iter()
                .enumerate()
                .map(|(i, &v)| v as i32 * ar.par[i.min(ar.par.len() - 1)])
                .sum(),
            K::Run => sh.iter().map(|&v| v as i32).sum(),
        };
        let cap = match ar.kind {
            K::Cross => 6,
            K::Sum => 11,
            _ => ar.len,
        };
        base + if sh.len() >= cap {
            self.sp.full_bonus
        } else {
            0
        }
    }
    fn score(&self, p: usize) -> i32 {
        (0..5).map(|a| self.area_score(p, a)).sum()
    }
    fn options(&self) -> Vec<(usize, usize)> {
        let p = self.who();
        let mut v = Vec::new();
        for d in 0..6 {
            let ok = if self.ph == Ph::Act {
                self.avail[d]
            } else {
                !self.taken[d]
            };
            if !ok {
                continue;
            }
            if d < 5 {
                if self.can(p, d, self.d[d], false).is_some() {
                    v.push((d, d));
                }
            } else {
                for a in 0..5 {
                    if self.can(p, a, self.d[5], true).is_some() {
                        v.push((d, a));
                    }
                }
            }
        }
        v
    }
}

impl Mini for Foaie {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        if self.who() == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn moves(&self) -> Vec<String> {
        if self.ph == Ph::Roll {
            return vec!["arunc".into()];
        }
        let mut v: Vec<String> = self
            .options()
            .iter()
            .map(|&(d, a)| format!("alege {} {}", d + 1, a + 1))
            .collect();
        v.push(if self.ph == Ph::Act { "gata" } else { "pas" }.into());
        v
    }
    fn step(&mut self, m: &str) {
        match self.ph {
            Ph::Roll => {
                for i in 0..6 {
                    self.d[i] = self.dice.below(6) as u8 + 1;
                }
                self.avail = [true; 6];
                self.taken = [false; 6];
                self.picks = 0;
                self.ph = Ph::Act;
            }
            _ => {
                if m == "gata" || m == "pas" {
                    if self.ph == Ph::Act {
                        self.ph = Ph::Pass;
                    } else {
                        self.turn += 1;
                        self.side = 1 - self.side;
                        self.ph = Ph::Roll;
                    }
                    return;
                }
                let p = self.who();
                let mut it = m[6..].split(' ');
                let d = it.next().unwrap().parse::<usize>().unwrap() - 1;
                let a = it.next().unwrap().parse::<usize>().unwrap() - 1;
                let v = self.can(p, a, self.d[d], d == 5).unwrap();
                let val = self.d[d];
                self.sheet[p][a].push(v);
                self.avail[d] = false;
                self.taken[d] = true;
                if self.ph == Ph::Act {
                    self.picks += 1;
                    for j in 0..6 {
                        if self.avail[j] && self.d[j] < val {
                            self.avail[j] = false;
                        }
                    }
                    if self.picks >= self.sp.picks || !self.avail.iter().any(|&x| x) {
                        self.ph = Ph::Pass;
                    }
                } else {
                    self.turn += 1;
                    self.side = 1 - self.side;
                    self.ph = Ph::Roll;
                }
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.ph == Ph::Roll && self.turn >= self.sp.turns {
            return Some(by_score(self.score(0), self.score(1)));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        100 * (self.score(a) - self.score(1 - a))
    }
    fn show(&self) -> String {
        let mut s = format!(
            "{} — tura {}/{}\n",
            self.sp.title,
            self.turn + 1,
            self.sp.turns
        );
        for p in 0..2 {
            s.push_str(&format!(
                "{} (scor {}):\n",
                pname(if p == 0 { Player::One } else { Player::Two }),
                self.score(p)
            ));
            for a in 0..5 {
                s.push_str(&format!(
                    "  {}. {}: {:?}\n",
                    a + 1,
                    self.sp.areas[a].name,
                    self.sheet[p][a]
                ));
            }
        }
        if self.ph != Ph::Roll {
            let t: Vec<String> = (0..6)
                .map(|d| {
                    format!(
                        "{}{}={}{}",
                        if d == 5 { "alb" } else { "z" },
                        d + 1,
                        self.d[d],
                        if self.taken[d] {
                            "(luat)"
                        } else if self.avail[d] {
                            ""
                        } else {
                            "(argint)"
                        }
                    )
                })
                .collect();
            s.push_str(&format!("Zaruri: {}\n", t.join(" ")));
        }
        s.push_str(&format!(
            "La mutare: {} (arunc | alege d a | gata | pas)\n",
            pname(self.side())
        ));
        s
    }
}
