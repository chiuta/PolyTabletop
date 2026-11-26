//! Motor comun „cărți de reflex” (variante originale B6, 2 jucători).
//!
//! Fiecare jucător are o mână de cărți-simbol (vezi doar a ta; ai la vedere doar numărul cărților celuilalt) și
//! o stivă pe masă. Pe rând joci un simbol `joc s` pe stiva ta. Dacă se produce o potrivire (după mod),
//! urmează un duel de reflexe, rezolvat prin mutarea forțată `prind`: șansa de a fi mai rapid crește cu
//! seria de dueluri câștigate (50% + 8% pe diferență de serie, între 20% și 80%). Pierzătorul duelului
//! ia în mână toate cărțile de pe masă. Câștigă primul care rămâne fără cărți în mână; după plafonul de
//! mutări câștigă cine are mai puține cărți în mână.
#![allow(clippy::needless_range_loop)]
use crate::b6_util::pl;
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SMode {
    /// Potrivire cu simbolul de deasupra stivei adversarului.
    Top,
    /// Simbolul jucat egal cu cuvântul numărat în ritm (0, 1, 2… ciclic).
    Call,
}

pub struct SSpec {
    pub title: &'static str,
    pub syms: usize,
    pub hand: usize,
    pub mode: SMode,
    pub cap: u32,
    /// Numele simbolurilor (pentru afișare).
    pub names: &'static [&'static str],
}

#[derive(Clone)]
pub struct Slap {
    spec: &'static SSpec,
    viewer: Option<Player>,
    hands: [Vec<u8>; 2],
    table: [Vec<u8>; 2],
    calls: usize,
    speed: [i32; 2],
    pend: bool,
    mover: usize,
    plies: u32,
    rng: Rng,
}

impl Slap {
    pub fn new(spec: &'static SSpec) -> Self {
        Self::with_seed(spec, seed_now())
    }
    pub fn with_seed(spec: &'static SSpec, seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck: Vec<u8> = (0..spec.hand * 2).map(|i| (i % spec.syms) as u8).collect();
        shuf(&mut deck, &mut rng);
        let b = deck.split_off(spec.hand);
        Slap {
            spec,
            viewer: None,
            hands: [deck, b],
            table: [Vec::new(), Vec::new()],
            calls: 0,
            speed: [0; 2],
            pend: false,
            mover: 0,
            plies: 0,
            rng,
        }
    }
    fn p_mover(&self) -> i32 {
        (50 + 8 * (self.speed[self.mover] - self.speed[1 - self.mover])).clamp(20, 80)
    }
    fn name(&self, s: u8) -> String {
        self.spec.names[s as usize].to_string()
    }
    fn tsize(&self) -> usize {
        self.table[0].len() + self.table[1].len()
    }
}

impl Mini for Slap {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        pl(self.mover)
    }
    fn moves(&self) -> Vec<String> {
        if self.pend {
            return vec!["prind".into()];
        }
        let mut v = Vec::new();
        for s in 0..self.spec.syms {
            if self.hands[self.mover].contains(&(s as u8)) {
                v.push(format!("joc {}", s + 1));
            }
        }
        v
    }
    fn step(&mut self, m: &str) {
        let me = self.mover;
        if self.pend {
            let win = (self.rng.below(100) as i32) < self.p_mover();
            let (w, l) = if win { (me, 1 - me) } else { (1 - me, me) };
            let mut t = std::mem::take(&mut self.table[0]);
            t.append(&mut self.table[1]);
            shuf(&mut t, &mut self.rng);
            self.hands[l].append(&mut t);
            self.speed[w] += 1;
            self.speed[l] = 0;
            self.pend = false;
            self.mover = 1 - me;
            self.plies += 1;
            return;
        }
        let s = (m[4..].parse::<usize>().unwrap() - 1) as u8;
        let pos = self.hands[me].iter().position(|&c| c == s).unwrap();
        self.hands[me].remove(pos);
        let hit = match self.spec.mode {
            SMode::Top => self.table[1 - me].last() == Some(&s),
            SMode::Call => {
                let w = self.calls % self.spec.syms;
                self.calls += 1;
                w == s as usize
            }
        };
        self.table[me].push(s);
        self.plies += 1;
        if hit {
            self.pend = true;
        } else {
            self.mover = 1 - me;
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.pend {
            return None;
        }
        let (a, b) = (self.hands[0].len(), self.hands[1].len());
        if a == 0 && b == 0 {
            return Some(Outcome::Draw);
        }
        if a == 0 {
            return Some(Outcome::Win(Player::One));
        }
        if b == 0 {
            return Some(Outcome::Win(Player::Two));
        }
        if self.plies >= self.spec.cap {
            return Some(match a.cmp(&b) {
                std::cmp::Ordering::Less => Outcome::Win(Player::One),
                std::cmp::Ordering::Greater => Outcome::Win(Player::Two),
                std::cmp::Ordering::Equal => Outcome::Draw,
            });
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let (ha, ho) = (self.hands[a].len() as i32, self.hands[1 - a].len() as i32);
        if !self.pend {
            return 100 * (ho - ha);
        }
        let t = self.tsize() as i32;
        let pm = if self.mover == a {
            self.p_mover()
        } else {
            100 - self.p_mover()
        };
        // câștig: adversarul ia masa; pierdere: o iau eu
        pm * (100 * (ho + t - ha)) / 100 + (100 - pm) * (100 * (ho - ha - t)) / 100
    }
    fn show(&self) -> String {
        let viewer = self.viewer.map(pi).unwrap_or(pi(self.side()));
        let mut s = format!(
            "{} — mutarea {}/{}\n",
            self.spec.title,
            self.plies + 1,
            self.spec.cap
        );
        for p in 0..2 {
            let top: Vec<String> = self.table[p]
                .iter()
                .rev()
                .take(6)
                .map(|&c| self.name(c))
                .collect();
            s.push_str(&format!(
                "J{}: {} cărți în mână, serie {}, masă ({}): {}\n",
                p + 1,
                self.hands[p].len(),
                self.speed[p],
                self.table[p].len(),
                if top.is_empty() {
                    "-".into()
                } else {
                    top.join(" < ")
                }
            ));
        }
        let mut hand = self.hands[viewer].clone();
        hand.sort();
        s.push_str(&format!(
            "Mâna ta (J{}): {}\n",
            viewer + 1,
            hand.iter()
                .map(|&c| format!("{}={}", c + 1, self.name(c)))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        if self.spec.mode == SMode::Call {
            s.push_str(&format!(
                "Cuvântul numărat acum: {}\n",
                self.name((self.calls % self.spec.syms) as u8)
            ));
        }
        s.push_str(&format!(
            "La mutare: {} (joc s | prind)\n",
            pname(self.side())
        ));
        s
    }
    fn view(&mut self, w: Option<Player>) {
        self.viewer = w;
    }
}

/// Teste: AI-ul nu citește mâna adversarului; randarea o ascunde.
#[cfg(test)]
pub fn audit(spec: &'static SSpec) {
    let g = Slap::with_seed(spec, 3);
    let mut h = g.clone();
    for c in h.hands[1].iter_mut() {
        *c = (*c + 1) % spec.syms as u8;
    }
    for seed in 1..6 {
        assert_eq!(ai_of(&g, seed), ai_of(&h, seed));
    }
    let mut v = g.clone();
    v.view(Some(Player::One));
    let s = v.show();
    assert!(s.contains("Mâna ta (J1)"));
    assert!(!s.contains("Mâna ta (J2)"));
}
