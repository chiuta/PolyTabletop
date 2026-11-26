//! Motor comun „publicul decide” (variante originale B6, 2 jucători).
//!
//! Fiecare jucător are o mână de 5 cărți, fiecare cu trei valori 0–4 (de ex. „absurd”, „dulce”, „tăios”).
//! În fiecare rundă publicul are gusturi ascunse `w` (trei valori 0–3), din care una e arătată public.
//! Pe rând jucătorii aleg în secret `joc i` (sau `joc i,j` la două cărți); mutarea forțată `voteaza`
//! dezvăluie totul: scorul unei alegeri = Σ cărți·(gusturi + scena publică); cel mai mare ia rundă (+1).
//! Informație ascunsă: mâna adversarului și alegerea primului jucător până la vot; AI-ul nu le citește.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

type V3 = [i32; 3];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Scene {
    /// Fără scenă.
    None,
    /// O scenă nouă în fiecare rundă (imaginea).
    Random,
    /// Scena e cartea câștigătoare din runda precedentă (banda continuă).
    Prev,
}

pub struct CSpec {
    pub title: &'static str,
    pub pick: usize,
    pub scene: Scene,
    pub rounds: u32,
    pub dims: [&'static str; 3],
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ph {
    First,
    Second,
    Resolve,
}

#[derive(Clone)]
pub struct Crowd {
    spec: &'static CSpec,
    viewer: Option<Player>,
    hands: [Vec<V3>; 2],
    sel: [Vec<usize>; 2],
    taste: V3,
    hint: (usize, i32),
    scene: V3,
    ph: Ph,
    round: u32,
    pts: [i32; 2],
    rng: Rng,
}

fn rand_card(rng: &mut Rng) -> V3 {
    [
        rng.below(5) as i32,
        rng.below(5) as i32,
        rng.below(5) as i32,
    ]
}

impl Crowd {
    pub fn new(spec: &'static CSpec) -> Self {
        Self::with_seed(spec, seed_now())
    }
    pub fn with_seed(spec: &'static CSpec, seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let h0: Vec<V3> = (0..5).map(|_| rand_card(&mut rng)).collect();
        let h1: Vec<V3> = (0..5).map(|_| rand_card(&mut rng)).collect();
        let mut g = Crowd {
            spec,
            viewer: None,
            hands: [h0, h1],
            sel: [Vec::new(), Vec::new()],
            taste: [0; 3],
            hint: (0, 0),
            scene: [0; 3],
            ph: Ph::First,
            round: 0,
            pts: [0; 2],
            rng,
        };
        g.new_round();
        g
    }
    fn new_round(&mut self) {
        for j in 0..3 {
            self.taste[j] = self.rng.below(4) as i32;
        }
        let d = self.rng.below(3);
        self.hint = (d, self.taste[d]);
        if self.spec.scene == Scene::Random || (self.spec.scene == Scene::Prev && self.round == 0) {
            self.scene = [
                self.rng.below(4) as i32,
                self.rng.below(4) as i32,
                self.rng.below(4) as i32,
            ];
        }
    }
    fn vec_of(&self, p: usize) -> V3 {
        let mut v = [0; 3];
        for &i in &self.sel[p] {
            for j in 0..3 {
                v[j] += self.hands[p][i][j];
            }
        }
        v
    }
    fn score(&self, v: V3) -> i32 {
        (0..3).map(|j| v[j] * (self.taste[j] + self.scene[j])).sum()
    }
    /// Scorul așteptat (×2), din informația publică.
    fn expect2(&self, v: V3) -> i32 {
        (0..3)
            .map(|j| {
                let w2 = if j == self.hint.0 { 2 * self.hint.1 } else { 3 };
                v[j] * (w2 + 2 * self.scene[j])
            })
            .sum()
    }
}

impl Mini for Crowd {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        match self.ph {
            Ph::First | Ph::Resolve => Player::One,
            Ph::Second => Player::Two,
        }
    }
    fn moves(&self) -> Vec<String> {
        if self.ph == Ph::Resolve {
            return vec!["voteaza".into()];
        }
        let p = pi(self.side());
        let n = self.hands[p].len();
        let mut v = Vec::new();
        if self.spec.pick == 1 {
            for i in 0..n {
                v.push(format!("joc {}", i + 1));
            }
        } else {
            for i in 0..n {
                for j in i + 1..n {
                    v.push(format!("joc {},{}", i + 1, j + 1));
                }
            }
        }
        v
    }
    fn step(&mut self, m: &str) {
        match self.ph {
            Ph::First | Ph::Second => {
                let p = pi(self.side());
                self.sel[p] = m[4..]
                    .split(',')
                    .map(|x| x.parse::<usize>().unwrap() - 1)
                    .collect();
                self.ph = if self.ph == Ph::First {
                    Ph::Second
                } else {
                    Ph::Resolve
                };
            }
            Ph::Resolve => {
                let v = [self.vec_of(0), self.vec_of(1)];
                let sc = [self.score(v[0]), self.score(v[1])];
                if sc[0] != sc[1] {
                    let w = if sc[0] > sc[1] { 0 } else { 1 };
                    self.pts[w] += 1;
                    if self.spec.scene == Scene::Prev {
                        self.scene = v[w];
                        for j in 0..3 {
                            self.scene[j] = self.scene[j] * 3 / (4 * self.spec.pick as i32);
                        }
                    }
                }
                for p in 0..2 {
                    let mut idx = self.sel[p].clone();
                    idx.sort_unstable_by(|a, b| b.cmp(a));
                    for i in idx {
                        self.hands[p].remove(i);
                    }
                    while self.hands[p].len() < 5 {
                        let c = rand_card(&mut self.rng);
                        self.hands[p].push(c);
                    }
                    self.sel[p].clear();
                }
                self.round += 1;
                self.ph = Ph::First;
                if self.round < self.spec.rounds {
                    self.new_round();
                }
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
        if self.sel[a].is_empty() {
            return base;
        }
        base + 10 * self.expect2(self.vec_of(a))
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
        s.push_str(&format!(
            "Dimensiuni: {} / {} / {}\nPublicul iubește {} = {}",
            self.spec.dims[0],
            self.spec.dims[1],
            self.spec.dims[2],
            self.spec.dims[self.hint.0],
            self.hint.1
        ));
        if self.spec.scene != Scene::None {
            s.push_str(&format!("\nScena: {:?}", self.scene));
        }
        s.push('\n');
        let hand: Vec<String> = self.hands[viewer]
            .iter()
            .enumerate()
            .map(|(i, c)| format!("{}:({},{},{})", i + 1, c[0], c[1], c[2]))
            .collect();
        s.push_str(&format!("Mâna ta (J{}): {}\n", viewer + 1, hand.join(" ")));
        if !self.sel[viewer].is_empty() && self.ph != Ph::Resolve {
            s.push_str(&format!(
                "Alegerea ta: {:?}\n",
                self.sel[viewer].iter().map(|i| i + 1).collect::<Vec<_>>()
            ));
        }
        s.push_str(&format!(
            "La mutare: {} (joc i | voteaza)\n",
            pname(self.side())
        ));
        s
    }
    fn view(&mut self, w: Option<Player>) {
        self.viewer = w;
    }
}

/// Teste: AI-ul celui de-al doilea nu citește mâna/alegerea primului; randarea le ascunde.
#[cfg(test)]
pub fn audit(spec: &'static CSpec) {
    let mut g = Crowd::with_seed(spec, 6);
    let first = g.moves()[0].clone();
    g.step(&first);
    let mut h = g.clone();
    h.hands[0] = vec![[4, 4, 4]; 5];
    h.sel[0] = vec![0];
    if spec.pick == 2 {
        h.sel[0] = vec![0, 1];
    }
    for seed in 1..6 {
        assert_eq!(ai_of(&g, seed), ai_of(&h, seed));
    }
    let mut v = g.clone();
    v.view(Some(Player::Two));
    let s = v.show();
    assert!(s.contains("Mâna ta (J2)"));
    assert!(!s.contains("Mâna ta (J1)"));
}
