#![allow(
    clippy::needless_range_loop,
    clippy::manual_is_multiple_of,
    clippy::collapsible_if,
    clippy::too_many_arguments,
    clippy::type_complexity
)]
//! Motor cooperativ „enigma” (lista C1, 2 jucători, variante originale, informație complet publică
//! în afară de soluția ascunsă — secretul nu aparține niciunui jucător, deci nu e informație privată).
//!
//! * `l` (încuietori): o serie de `a` încuietori, fiecare cu un cod de `b` cifre în baza `c`.
//!   Pe rând propuneți `cod 012`; mutarea forțată `verific` anunță (la locul exact, la alt loc).
//!   `ajutor` (costă `hint` timp) dezvăluie o cifră prin mutarea forțată `arata`. Fiecare propunere
//!   costă 1 timp; epuizarea timpului înainte de ultima încuietoare = eșec.
//! * `k` (caz): `a` suspecți cu 3 însușiri (`c` valori); `viz L` vizitează un loc (din `b`), iar
//!   mutarea forțată `indiciu` dă un indiciu adevărat despre vinovat (locurile pare: „are valoarea x”,
//!   impare: „nu are valoarea x”); `acuz s` acuză (verific): corect = victorie, greșit = suspectul
//!   iese și se pierde o șansă (2 șanse). Fiecare vizită/acuzare costă 1 timp.
//!
//! Succesul = victoria echipei (Jucătorul 1 în raport), eșecul = REMIZĂ; verdictul se dă doar la
//! mutări forțate.
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

pub struct ESpec {
    pub title: &'static str,
    pub mode: char,
    pub a: usize,
    pub b: usize,
    pub c: usize,
    pub time: u32,
    pub hint: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ph {
    Act,
    Check,
    Reveal,
    Clue,
    Accuse,
}

#[derive(Clone)]
pub struct Enigma {
    sp: &'static ESpec,
    ph: Ph,
    side: Player,
    time: u32,
    end: Option<Outcome>,
    rng: Rng,
    // încuietori
    codes: Vec<Vec<u8>>,
    cur: usize,
    hist: Vec<(Vec<u8>, (usize, usize))>,
    rev: Vec<Option<u8>>,
    pend: Vec<u8>,
    // caz
    sus: Vec<[u8; 3]>,
    culprit: usize,
    out: Vec<bool>,
    facts: Vec<(usize, u8, bool)>,
    visited: Vec<bool>,
    arg: usize,
    chances: u8,
}

fn feedback(code: &[u8], g: &[u8]) -> (usize, usize) {
    let exact = code.iter().zip(g).filter(|(a, b)| a == b).count();
    let mut cc = [0usize; 10];
    let mut gc = [0usize; 10];
    for (&a, &b) in code.iter().zip(g) {
        cc[a as usize] += 1;
        gc[b as usize] += 1;
    }
    let common: usize = (0..10).map(|i| cc[i].min(gc[i])).sum();
    (exact, common - exact)
}

impl Enigma {
    pub fn new(sp: &'static ESpec) -> Self {
        Self::with_seed(sp, seed_now())
    }
    pub fn with_seed(sp: &'static ESpec, seed: u64) -> Self {
        let mut rng = Rng::new(seed ^ 0x2545_f491);
        let mut codes = Vec::new();
        let mut sus = Vec::new();
        let mut culprit = 0;
        if sp.mode == 'l' {
            for _ in 0..sp.a {
                codes.push((0..sp.b).map(|_| rng.below(sp.c) as u8).collect());
            }
        } else {
            for _ in 0..sp.a {
                sus.push([
                    rng.below(sp.c) as u8,
                    rng.below(sp.c) as u8,
                    rng.below(sp.c) as u8,
                ]);
            }
            culprit = rng.below(sp.a);
        }
        Enigma {
            sp,
            ph: Ph::Act,
            side: Player::One,
            time: 0,
            end: None,
            rng,
            codes,
            cur: 0,
            hist: Vec::new(),
            rev: vec![None; sp.b],
            pend: Vec::new(),
            sus,
            culprit,
            out: vec![false; sp.a],
            facts: Vec::new(),
            visited: vec![false; sp.b],
            arg: 0,
            chances: 2,
        }
    }
    fn all_codes(&self) -> Vec<Vec<u8>> {
        let mut v: Vec<Vec<u8>> = vec![Vec::new()];
        for _ in 0..self.sp.b {
            let mut n = Vec::new();
            for c in &v {
                for d in 0..self.sp.c {
                    let mut x = c.clone();
                    x.push(d as u8);
                    n.push(x);
                }
            }
            v = n;
        }
        v
    }
    fn code_ok(&self, cand: &[u8]) -> bool {
        self.hist.iter().all(|(g, fb)| feedback(cand, g) == *fb)
            && self
                .rev
                .iter()
                .enumerate()
                .all(|(i, r)| r.is_none_or(|d| cand[i] == d))
    }
    fn n_codes(&self) -> usize {
        self.all_codes().iter().filter(|c| self.code_ok(c)).count()
    }
    fn sus_ok(&self, s: usize) -> bool {
        !self.out[s]
            && self
                .facts
                .iter()
                .all(|&(j, v, eq)| (self.sus[s][j] == v) == eq)
    }
    fn n_sus(&self) -> usize {
        (0..self.sp.a).filter(|&s| self.sus_ok(s)).count()
    }
    fn judge(&mut self, solved: bool) {
        if solved {
            self.end = Some(Outcome::Win(Player::One));
        } else if self.time >= self.sp.time {
            self.end = Some(Outcome::Draw);
        }
    }
}

impl Mini for Enigma {
    fn side(&self) -> Player {
        self.side
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            Ph::Check => vec!["verific".into()],
            Ph::Reveal => vec!["arata".into()],
            Ph::Clue => vec!["indiciu".into()],
            Ph::Accuse => vec!["verific".into()],
            Ph::Act => {
                let mut v = Vec::new();
                if self.sp.mode == 'l' {
                    for c in self.all_codes() {
                        let s: String = c.iter().map(|d| (b'0' + d) as char).collect();
                        v.push(format!("cod {s}"));
                    }
                    if self.sp.hint > 0 && self.rev.iter().any(|r| r.is_none()) {
                        v.push("ajutor".into());
                    }
                } else {
                    for l in 0..self.sp.b {
                        if !self.visited[l] {
                            v.push(format!("viz {l}"));
                        }
                    }
                    for s in 0..self.sp.a {
                        if !self.out[s] {
                            v.push(format!("acuz {s}"));
                        }
                    }
                }
                v
            }
        }
    }
    fn step(&mut self, m: &str) {
        match self.ph {
            Ph::Act => {
                if let Some(c) = m.strip_prefix("cod ") {
                    self.pend = c.bytes().map(|b| b - b'0').collect();
                    self.time += 1;
                    self.ph = Ph::Check;
                } else if m == "ajutor" {
                    self.time += self.sp.hint;
                    self.ph = Ph::Reveal;
                } else if let Some(l) = m.strip_prefix("viz ") {
                    let l: usize = l.parse().unwrap();
                    self.visited[l] = true;
                    self.arg = l;
                    self.time += 1;
                    self.ph = Ph::Clue;
                } else if let Some(s) = m.strip_prefix("acuz ") {
                    self.arg = s.parse().unwrap();
                    self.time += 1;
                    self.ph = Ph::Accuse;
                }
            }
            Ph::Check => {
                let code = self.codes[self.cur].clone();
                let fb = feedback(&code, &self.pend);
                self.hist.push((self.pend.clone(), fb));
                let mut solved = false;
                if fb.0 == self.sp.b {
                    self.cur += 1;
                    self.hist.clear();
                    self.rev = vec![None; self.sp.b];
                    solved = self.cur == self.sp.a;
                }
                self.ph = Ph::Act;
                self.side = self.side.other();
                self.judge(solved);
            }
            Ph::Reveal => {
                let code = self.codes[self.cur].clone();
                if let Some(i) = (0..self.sp.b).find(|&i| self.rev[i].is_none()) {
                    self.rev[i] = Some(code[i]);
                }
                self.ph = Ph::Act;
                self.side = self.side.other();
                self.judge(false);
            }
            Ph::Clue => {
                let j = self.arg % 3;
                let real = self.sus[self.culprit][j];
                if self.arg % 2 == 0 {
                    self.facts.push((j, real, true));
                } else {
                    let mut w = self.rng.below(self.sp.c) as u8;
                    if w == real {
                        w = (w + 1) % self.sp.c as u8;
                    }
                    self.facts.push((j, w, false));
                }
                self.ph = Ph::Act;
                self.side = self.side.other();
                self.judge(false);
            }
            Ph::Accuse => {
                if self.arg == self.culprit {
                    self.judge(true);
                } else {
                    self.out[self.arg] = true;
                    self.chances -= 1;
                    if self.chances == 0 {
                        self.time = self.sp.time;
                    }
                    self.ph = Ph::Act;
                    self.side = self.side.other();
                    self.judge(false);
                }
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        self.end
    }
    fn eval(&self, _p: Player) -> i32 {
        let left = self.sp.time.saturating_sub(self.time) as i32;
        if self.sp.mode == 'l' {
            match self.ph {
                Ph::Check => {
                    let n = self.n_codes_before_pending();
                    if n.1 {
                        1000 - n.0 as i32 / 4
                    } else {
                        100
                    }
                }
                Ph::Reveal if self.n_codes() > 10 => 800,
                _ => 0,
            }
        } else {
            let n = self.n_sus() as i32;
            match self.ph {
                Ph::Clue => {
                    if n > 1 && left > 1 {
                        600
                    } else {
                        0
                    }
                }
                Ph::Accuse => {
                    let ok = self.sus_ok(self.arg);
                    if ok && (n == 1 || left <= 1) {
                        1000
                    } else if ok && self.visited.iter().all(|v| *v) {
                        900
                    } else if ok {
                        200
                    } else {
                        -500
                    }
                }
                _ => 0,
            }
        }
    }
    fn show(&self) -> String {
        let mut s = format!("{} — timp {}/{}\n", self.sp.title, self.time, self.sp.time);
        if self.sp.mode == 'l' {
            s.push_str(&format!(
                "încuietoarea {}/{} (cod de {} cifre, 0–{})\n",
                (self.cur + 1).min(self.sp.a),
                self.sp.a,
                self.sp.b,
                self.sp.c - 1
            ));
            let r: Vec<String> = self
                .rev
                .iter()
                .map(|x| x.map_or("_".to_string(), |d| d.to_string()))
                .collect();
            s.push_str(&format!("cifre aflate: {}\n", r.join(" ")));
            for (g, fb) in &self.hist {
                let t: String = g.iter().map(|d| (b'0' + d) as char).collect();
                s.push_str(&format!("  {t} -> exact {} / aproape {}\n", fb.0, fb.1));
            }
            if self.ph == Ph::Check {
                let t: String = self.pend.iter().map(|d| (b'0' + d) as char).collect();
                s.push_str(&format!("propunere în așteptare: {t}\n"));
            }
        } else {
            s.push_str(&format!(
                "{} suspecți (însușiri a,b,c cu valori 0–{}), {} locuri, șanse {}\n",
                self.sp.a,
                self.sp.c - 1,
                self.sp.b,
                self.chances
            ));
            for i in 0..self.sp.a {
                s.push_str(&format!(
                    "  suspect {i}: {:?}{}\n",
                    self.sus[i],
                    if self.out[i] { " (scos)" } else { "" }
                ));
            }
            for &(j, v, eq) in &self.facts {
                s.push_str(&format!(
                    "  indiciu: însușirea {} {} {v}\n",
                    ["a", "b", "c"][j],
                    if eq { "=" } else { "≠" }
                ));
            }
            let vis: Vec<String> = (0..self.sp.b)
                .filter(|&l| !self.visited[l])
                .map(|l| l.to_string())
                .collect();
            s.push_str(&format!("locuri nevizitate: {}\n", vis.join(" ")));
        }
        s.push_str(&format!(
            "La mutare: {} ({:?}; {})\n",
            pname(self.side),
            self.ph,
            if self.sp.mode == 'l' {
                "cod xyz | ajutor"
            } else {
                "viz L | acuz s"
            }
        ));
        s
    }
}

impl Enigma {
    /// (numărul de coduri încă posibile înainte de propunere, propunerea e consistentă)
    fn n_codes_before_pending(&self) -> (usize, bool) {
        let ok = self.code_ok(&self.pend);
        (self.n_codes(), ok)
    }
}
