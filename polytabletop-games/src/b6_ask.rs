//! Motor comun „întrebări da/nu” pentru jocurile de petrecere B6 (variante originale, 2 jucători).
//!
//! Un obiect ascuns dintr-un set cu atribute (A, B, C…). Moduri:
//! * `Mayor` — un jucător (primarul) știe ținta, celălalt pune întrebări `intreb A=2` (răspuns automat,
//!   adevărat) și apoi ghicește `ghic k`;
//! * `Shared` — ținta nu e știută de nimeni; ambii pun întrebări pe rând (răspunsurile sunt publice);
//! * `Own` — fiecare are propriul obiect, pe care îl vede doar adversarul (ca pe o bentiță pe frunte);
//! * `Ladder` — indiciile apar pe rând (`mai mult`), fiecare poate ghici oricând.
//!
//! Răspunsul la o întrebare și rezultatul unei ghiciri se cunosc abia la mutarea forțată
//! (`raspuns`, `arata`, `verific`), deci AI-ul nu poate „ghici” din rezultatul ascuns.
#![allow(clippy::needless_range_loop)]
use crate::b6_util::pl;
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AMode {
    Mayor,
    Shared,
    Own,
    Ladder,
}

pub struct ASpec {
    pub title: &'static str,
    pub vals: &'static [usize],
    pub mode: AMode,
    pub rounds: u32,
    /// Întrebări permise (Mayor: total; altfel: pe jucător) sau indicii (Ladder).
    pub qmax: u32,
    pub attr_names: &'static [&'static str],
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ph {
    Act,
    PendAsk(usize, usize),
    PendGuess(usize),
    PendHint,
}

#[derive(Clone)]
pub struct Ask {
    spec: &'static ASpec,
    viewer: Option<Player>,
    target: [usize; 2],
    facts: [Vec<(usize, usize, bool)>; 2],
    ph: Ph,
    mover: usize,
    asked: u32,
    round: u32,
    pts: [i32; 2],
    rng: Rng,
}

fn n_items(s: &ASpec) -> usize {
    s.vals.iter().product()
}

fn av(s: &ASpec, item: usize, a: usize) -> usize {
    let stride: usize = s.vals[..a].iter().product();
    (item / stride) % s.vals[a]
}

impl Ask {
    pub fn new(spec: &'static ASpec) -> Self {
        Self::with_seed(spec, seed_now())
    }
    pub fn with_seed(spec: &'static ASpec, seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let n = n_items(spec);
        let target = [rng.below(n), rng.below(n)];
        Ask {
            spec,
            viewer: None,
            target,
            facts: [Vec::new(), Vec::new()],
            ph: Ph::Act,
            mover: 0,
            asked: 0,
            round: 0,
            pts: [0; 2],
            rng,
        }
    }
    fn mayor(&self) -> usize {
        (self.round % 2) as usize
    }
    fn fidx(&self, who: usize) -> usize {
        if self.spec.mode == AMode::Own {
            who
        } else {
            0
        }
    }
    fn actor(&self) -> usize {
        if self.spec.mode == AMode::Mayor {
            1 - self.mayor()
        } else {
            self.mover
        }
    }
    fn tgt(&self, who: usize) -> usize {
        if self.spec.mode == AMode::Own {
            self.target[who]
        } else {
            self.target[0]
        }
    }
    fn cands(&self, who: usize) -> Vec<usize> {
        let f = &self.facts[self.fidx(who)];
        (0..n_items(self.spec))
            .filter(|&k| f.iter().all(|&(a, v, y)| (av(self.spec, k, a) == v) == y))
            .collect()
    }
    fn used(&self, who: usize) -> u32 {
        if self.spec.mode == AMode::Mayor || self.spec.mode == AMode::Ladder {
            self.asked
        } else {
            self.facts[self.fidx(who)].len() as u32
        }
    }
    fn can_ask(&self, who: usize) -> bool {
        self.used(who) < self.spec.qmax
    }
    fn end_round(&mut self) {
        self.round += 1;
        let n = n_items(self.spec);
        self.target = [self.rng.below(n), self.rng.below(n)];
        self.facts = [Vec::new(), Vec::new()];
        self.asked = 0;
        self.ph = Ph::Act;
        self.mover = (self.round % 2) as usize;
    }
    fn item_str(&self, k: usize) -> String {
        let mut s = format!("{:>2}:", k + 1);
        for a in 0..self.spec.vals.len() {
            s.push_str(&format!(
                " {}{}",
                (b'A' + a as u8) as char,
                av(self.spec, k, a) + 1
            ));
        }
        s
    }
    fn fact_str(&self, who: usize) -> String {
        let f = &self.facts[self.fidx(who)];
        if f.is_empty() {
            return "-".into();
        }
        f.iter()
            .map(|&(a, v, y)| {
                format!(
                    "{}{}{}",
                    (b'A' + a as u8) as char,
                    if y { "=" } else { "≠" },
                    v + 1
                )
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl Mini for Ask {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        pl(self.actor())
    }
    fn moves(&self) -> Vec<String> {
        let who = self.actor();
        match self.ph {
            Ph::Act => {
                let mut v = Vec::new();
                let c = self.cands(who);
                let ladder = self.spec.mode == AMode::Ladder;
                if self.can_ask(who) && c.len() > 1 {
                    if ladder {
                        v.push("mai mult".to_string());
                    } else {
                        for a in 0..self.spec.vals.len() {
                            for val in 0..self.spec.vals[a] {
                                let y = c.iter().filter(|&&k| av(self.spec, k, a) == val).count();
                                if y > 0 && y < c.len() {
                                    v.push(format!(
                                        "intreb {}={}",
                                        (b'A' + a as u8) as char,
                                        val + 1
                                    ));
                                }
                            }
                        }
                    }
                }
                for k in 0..n_items(self.spec) {
                    v.push(format!("ghic {}", k + 1));
                }
                v
            }
            Ph::PendAsk(..) => vec!["raspuns".into()],
            Ph::PendGuess(_) => vec!["verific".into()],
            Ph::PendHint => vec!["arata".into()],
        }
    }
    fn step(&mut self, m: &str) {
        let who = self.actor();
        match self.ph {
            Ph::Act => {
                if m == "mai mult" {
                    self.ph = Ph::PendHint;
                } else if let Some(r) = m.strip_prefix("intreb ") {
                    let a = (r.as_bytes()[0].to_ascii_uppercase() - b'A') as usize;
                    let v = r[2..].parse::<usize>().unwrap() - 1;
                    self.ph = Ph::PendAsk(a, v);
                } else {
                    self.ph = Ph::PendGuess(m[5..].parse::<usize>().unwrap() - 1);
                }
            }
            Ph::PendAsk(a, v) => {
                let yes = av(self.spec, self.tgt(who), a) == v;
                let fi = self.fidx(who);
                self.facts[fi].push((a, v, yes));
                self.asked += 1;
                self.ph = Ph::Act;
                if self.spec.mode != AMode::Mayor {
                    self.mover = 1 - self.mover;
                }
            }
            Ph::PendHint => {
                let t = self.target[0];
                let free: Vec<usize> = (0..self.spec.vals.len())
                    .filter(|&a| !self.facts[0].iter().any(|f| f.0 == a))
                    .collect();
                if !free.is_empty() {
                    let a = free[self.rng.below(free.len())];
                    self.facts[0].push((a, av(self.spec, t, a), true));
                }
                self.asked += 1;
                self.ph = Ph::Act;
                self.mover = 1 - self.mover;
            }
            Ph::PendGuess(k) => {
                let ok = k == self.tgt(who);
                match self.spec.mode {
                    AMode::Mayor => {
                        if ok {
                            self.pts[who] += 2;
                            self.pts[1 - who] += 1;
                        } else {
                            self.pts[1 - who] += 2;
                        }
                    }
                    AMode::Ladder => {
                        if ok {
                            self.pts[who] += 1 + (self.spec.qmax as i32 - self.asked as i32).max(0);
                        } else {
                            self.pts[1 - who] += 1;
                        }
                    }
                    _ => {
                        if ok {
                            self.pts[who] += 2;
                        } else {
                            self.pts[1 - who] += 1;
                        }
                    }
                }
                self.end_round();
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.round >= self.spec.rounds {
            return Some(by_score(self.pts[0], self.pts[1]));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let base = 1000 * (self.pts[a] - self.pts[1 - a]);
        if a != self.actor() {
            return base;
        }
        let c = self.cands(a);
        let cn = c.len().max(1) as i32;
        let guess_val = |c: i32| -> i32 {
            if self.spec.mode == AMode::Mayor {
                100 * (3 - 2 * c) / c
            } else {
                100 * (2 - c) / c
            }
        };
        match self.ph {
            Ph::PendAsk(at, v) => {
                let y = c.iter().filter(|&&k| av(self.spec, k, at) == v).count() as i32;
                let n = cn - y;
                let c2 = 100 * (y * y + n * n) / cn;
                base + 700 - (c2 - 100) * 10 / 4
            }
            Ph::PendHint => base + 700 - 250 * (((cn + 1) / 2) - 1),
            Ph::PendGuess(k) => {
                if c.contains(&k) {
                    base + 10 * guess_val(cn)
                } else {
                    base - 1500
                }
            }
            Ph::Act => base,
        }
    }
    fn show(&self) -> String {
        let viewer = self.viewer.map(pi).unwrap_or(pi(self.side()));
        let mut s = format!(
            "{} — runda {}/{} | puncte J1 {}, J2 {}\n",
            self.spec.title,
            (self.round + 1).min(self.spec.rounds),
            self.spec.rounds,
            self.pts[0],
            self.pts[1]
        );
        let names: Vec<String> = self
            .spec
            .attr_names
            .iter()
            .enumerate()
            .map(|(i, n)| format!("{}={}", (b'A' + i as u8) as char, n))
            .collect();
        s.push_str(&format!("Atribute: {}\n", names.join(", ")));
        for k in 0..n_items(self.spec) {
            let mark = match self.spec.mode {
                AMode::Mayor if viewer == self.mayor() && k == self.target[0] => "  <- ținta ta",
                AMode::Own if k == self.target[1 - viewer] => "  <- obiectul adversarului",
                _ => "",
            };
            s.push_str(&format!("{}{}\n", self.item_str(k), mark));
        }
        match self.spec.mode {
            AMode::Own => {
                s.push_str(&format!("Faptele tale: {}\n", self.fact_str(viewer)));
                s.push_str(&format!(
                    "Faptele adversarului: {}\n",
                    self.fact_str(1 - viewer)
                ));
            }
            _ => s.push_str(&format!("Fapte publice: {}\n", self.fact_str(0))),
        }
        s.push_str(&format!(
            "Candidați rămași pentru {}: {}\nLa mutare: {} (faza {:?}: intreb X=v | mai mult | ghic k | raspuns | arata | verific)\n",
            pname(pl(self.actor())),
            self.cands(self.actor()).len(),
            pname(self.side()),
            self.ph
        ));
        s
    }
    fn view(&mut self, w: Option<Player>) {
        self.viewer = w;
    }
}

/// Teste comune: ținta ascunsă celui care ghicește; AI-ul nu citește informația ascunsă.
#[cfg(test)]
pub fn audit(spec: &'static ASpec) {
    let mut g = Ask::with_seed(spec, 8);
    let who = g.actor();
    // starea de ghicit: informația ascunsă a celorlalți nu apare în randare
    let mut v = g.clone();
    v.view(Some(pl(who)));
    let s = v.show();
    match spec.mode {
        AMode::Mayor => assert!(!s.contains("ținta ta")),
        AMode::Own => {
            assert!(s.contains("obiectul adversarului"));
            assert_eq!(s.matches("obiectul adversarului").count(), 1);
        }
        _ => assert!(!s.contains("<-")),
    }
    // AI-ul alege la fel indiferent de obiectul ascuns
    g.ph = Ph::Act;
    let mut h = g.clone();
    let n = n_items(spec);
    match spec.mode {
        AMode::Own => h.target[who] = (g.target[who] + 1) % n,
        _ => h.target[0] = (g.target[0] + 1) % n,
    }
    for seed in 1..6 {
        assert_eq!(ai_of(&g, seed), ai_of(&h, seed));
    }
}
