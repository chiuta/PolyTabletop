//! Orașul Detectivilor — urmărirea suspecților pe o hartă (variantă originală de deducție, 2 jucători).
//!
//! Orașul e o grilă 6×6, timpul are pași 0–7. Șase suspecți (1–6) au mers fiecare câte un pas pe
//! tură (sau au stat); o crimă a avut loc la pasul 5 într-o celulă X (marcată pe hartă) și doar
//! vinovatul a fost acolo. Pe tură: `arata s,t` (cere să se dezvăluie public unde era suspectul s la pasul t, 0–7)
//! sau `acuz s`. Cine acuză corect câștigă, cine acuză greșit pierde (mutarea forțată `verific`).
//! Eliminarea se face din fapte: un suspect care nu putea ajunge în X la pasul 5 (distanță mai mare decât
//! pașii rămași) sau care era în alt loc la pasul 5 nu e vinovat. După 60 de mutări (cu afișările): remiză.
//! Informație ascunsă: traseele întregi nu se văd; AI-ul folosește doar faptele dezvăluite.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const T: usize = 8;
const TC: usize = 5;

#[derive(Clone)]
pub struct Detectiv {
    path: [[(i8, i8); T]; 6],
    culprit: usize,
    crime: (i8, i8),
    known: [[Option<(i8, i8)>; T]; 6],
    accused: Option<(usize, Player)>,
    pending: Option<(usize, usize)>,
    side: Player,
    ply: u32,
    done: Option<Outcome>,
}

fn dist(a: (i8, i8), b: (i8, i8)) -> i32 {
    (a.0 as i32 - b.0 as i32).abs() + (a.1 as i32 - b.1 as i32).abs()
}

impl Detectiv {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        loop {
            let mut path = [[(0i8, 0i8); T]; 6];
            for s in 0..6 {
                let mut p = (rng.below(6) as i8, rng.below(6) as i8);
                for t in 0..T {
                    path[s][t] = p;
                    let d = rng.below(5);
                    let (dr, dc) = [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)][d];
                    let np = (p.0 + dr, p.1 + dc);
                    if (0..6).contains(&np.0) && (0..6).contains(&np.1) {
                        p = np;
                    }
                }
            }
            let culprit = rng.below(6);
            let crime = path[culprit][TC];
            let n = (0..6).filter(|&s| path[s][TC] == crime).count();
            if n == 1 {
                return Detectiv {
                    path,
                    culprit,
                    crime,
                    known: [[None; T]; 6],
                    accused: None,
                    pending: None,
                    side: Player::One,
                    ply: 0,
                    done: None,
                };
            }
        }
    }
    fn candidate(&self, s: usize) -> bool {
        let facts: Vec<(usize, (i8, i8))> = (0..T)
            .filter_map(|t| self.known[s][t].map(|p| (t, p)))
            .collect();
        for &(t, p) in &facts {
            if t == TC && p != self.crime {
                return false;
            }
            if dist(p, self.crime) > (t as i32 - TC as i32).abs() {
                return false;
            }
        }
        for i in 0..facts.len() {
            for j in i + 1..facts.len() {
                if dist(facts[i].1, facts[j].1) > (facts[i].0 as i32 - facts[j].0 as i32).abs() {
                    return false;
                }
            }
        }
        true
    }
    fn cands(&self) -> Vec<usize> {
        (0..6).filter(|&s| self.candidate(s)).collect()
    }
}

impl Default for Detectiv {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Detectiv {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        self.side
    }
    fn moves(&self) -> Vec<String> {
        if self.accused.is_some() {
            return vec!["verific".into()];
        }
        if self.pending.is_some() {
            return vec!["afisez".into()];
        }
        let mut v = Vec::new();
        for s in 0..6 {
            for t in 0..T {
                if self.known[s][t].is_none() {
                    v.push(format!("arata {},{}", s + 1, t));
                }
            }
            v.push(format!("acuz {}", s + 1));
        }
        v
    }
    fn step(&mut self, m: &str) {
        self.ply += 1;
        if m == "verific" {
            let (s, who) = self.accused.take().unwrap();
            self.done = Some(if s == self.culprit {
                Outcome::Win(who)
            } else {
                Outcome::Win(who.other())
            });
            return;
        }
        if m == "afisez" {
            let (s, t) = self.pending.take().unwrap();
            self.known[s][t] = Some(self.path[s][t]);
            return;
        }
        if let Some(s) = m.strip_prefix("acuz ") {
            self.accused = Some((s.parse::<usize>().unwrap() - 1, self.side));
        } else {
            let (s, t) = m[6..].split_once(',').unwrap();
            let (s, t) = (s.parse::<usize>().unwrap() - 1, t.parse::<usize>().unwrap());
            self.pending = Some((s, t));
        }
        self.side = self.side.other();
    }
    fn result(&self) -> Option<Outcome> {
        if self.done.is_some() {
            return self.done;
        }
        if self.ply >= 60 {
            return Some(Outcome::Draw);
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let c = self.cands();
        if let Some((s, who)) = self.accused {
            // verdict estimat din fapte publice
            let good = if !c.contains(&s) {
                -1000
            } else if c.len() == 1 {
                1000
            } else {
                1000 * 2 / c.len() as i32 - 1000
            };
            return if who == p { good } else { -good };
        }
        if c.len() == 1 {
            return if self.side == p { 1000 } else { -1000 };
        }
        // mic avantaj: număr de candidați rămași pentru cel care urmează
        0
    }
    fn show(&self) -> String {
        let mut s = format!(
            "Orașul Detectivilor — crima la pasul {} în celula ({},{}) [rând,coloană 1..6]\n",
            TC,
            self.crime.0 + 1,
            self.crime.1 + 1
        );
        for r in 0..6 {
            for c in 0..6 {
                s.push(if (r as i8, c as i8) == self.crime {
                    'X'
                } else {
                    '.'
                });
                s.push(' ');
            }
            s.push('\n');
        }
        for st in 0..6 {
            let f: Vec<String> = (0..T)
                .filter_map(|t| {
                    self.known[st][t].map(|p| format!("t{}=({},{})", t, p.0 + 1, p.1 + 1))
                })
                .collect();
            s.push_str(&format!(
                "Suspectul {}: {}{}\n",
                st + 1,
                if f.is_empty() {
                    "-".to_string()
                } else {
                    f.join(" ")
                },
                if self.candidate(st) {
                    ""
                } else {
                    "  (eliminat)"
                }
            ));
        }
        let c: Vec<String> = self.cands().iter().map(|x| (x + 1).to_string()).collect();
        s.push_str(&format!(
            "Rămași suspecți: {}\nLa mutare: {} (arata s,t | acuz s | afisez | verific)\n",
            c.join(" "),
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
        crate::brand_util::check(Detectiv::with_seed, 100);
    }
    #[test]
    fn culprit_is_never_eliminated() {
        let mut g = Detectiv::with_seed(11);
        for s in 0..6 {
            for t in 0..T {
                g.known[s][t] = Some(g.path[s][t]);
            }
        }
        assert_eq!(g.cands(), vec![g.culprit]);
    }
    #[test]
    fn ai_ignores_unrevealed_paths() {
        let a = Detectiv::with_seed(5);
        let mut b = a.clone();
        for s in 0..6 {
            for t in 0..T {
                if t != TC {
                    b.path[s][t] = (0, 0);
                }
            }
        }
        for seed in 1..5 {
            assert_eq!(
                crate::brand_util::ai_of(&a, seed),
                crate::brand_util::ai_of(&b, seed)
            );
        }
    }
}
