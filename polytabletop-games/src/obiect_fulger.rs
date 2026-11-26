//! Obiectul Fulger — joc de reflex original (2 jucători, alegeri simultane).
//!
//! Cinci obiecte (1 lampă, 2 pălărie, 3 cheie, 4 pantof, 5 cană), fiecare cu culoarea ei „naturală”
//! (roșu, verde, albastru, galben, negru, în aceeași ordine). Pe fiecare carte apar două obiecte, în culori
//! oarecare. Regula noastră: obiectele candidate sunt cele a căror formă NU apare pe carte, dar a căror culoare
//! naturală apare; dacă e un singur candidat, îl prinzi pe el; dacă sunt doi, îl prinzi pe cel a cărui culoare
//! e pe prima poziție; dacă nu sunt candidați, prinzi primul obiect (în ordinea 1–5) a cărui formă și
//! a cărui culoare naturală lipsesc de pe carte. Fiecare alege în secret `prind k`; mutarea forțată
//! `verific` dezvăluie: corect +1, greșit −1; dacă ambii au nimerit, mai rapidul (șansa crește cu seria) ia cartea.
//! Informație ascunsă: alegerea primului jucător până la verificare; AI-ul nu o citește.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const OBJ: [&str; 5] = ["lampă", "pălărie", "cheie", "pantof", "cană"];
const COL: [&str; 5] = ["roșu", "verde", "albastru", "galben", "negru"];
const CARDS: u32 = 16;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ph {
    First,
    Second,
    Resolve,
}

#[derive(Clone)]
pub struct ObiectFulger {
    viewer: Option<Player>,
    card: [(usize, usize); 2],
    sel: [Option<usize>; 2],
    ph: Ph,
    n: u32,
    pts: [i32; 2],
    streak: [i32; 2],
    rng: Rng,
}

fn right(card: &[(usize, usize); 2]) -> usize {
    let shapes = [card[0].0, card[1].0];
    let cols = [card[0].1, card[1].1];
    let cand: Vec<usize> = (0..5)
        .filter(|o| !shapes.contains(o) && cols.contains(o))
        .collect();
    match cand.len() {
        1 => cand[0],
        2 => {
            if cand.contains(&card[0].1) {
                card[0].1
            } else {
                cand[0]
            }
        }
        _ => (0..5)
            .find(|o| !shapes.contains(o) && !cols.contains(o))
            .unwrap_or(0),
    }
}

fn draw(rng: &mut Rng) -> [(usize, usize); 2] {
    let a = rng.below(5);
    let b = (a + 1 + rng.below(4)) % 5;
    [(a, rng.below(5)), (b, rng.below(5))]
}

impl ObiectFulger {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let card = draw(&mut rng);
        ObiectFulger {
            viewer: None,
            card,
            sel: [None; 2],
            ph: Ph::First,
            n: 0,
            pts: [0; 2],
            streak: [0; 2],
            rng,
        }
    }
}

impl Default for ObiectFulger {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for ObiectFulger {
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
        match self.ph {
            Ph::Resolve => vec!["verific".into()],
            _ => (1..=5).map(|k| format!("prind {k}")).collect(),
        }
    }
    fn step(&mut self, m: &str) {
        match self.ph {
            Ph::First => {
                self.sel[0] = Some(m[6..].parse::<usize>().unwrap() - 1);
                self.ph = Ph::Second;
            }
            Ph::Second => {
                self.sel[1] = Some(m[6..].parse::<usize>().unwrap() - 1);
                self.ph = Ph::Resolve;
            }
            Ph::Resolve => {
                let r = right(&self.card);
                let ok = [self.sel[0] == Some(r), self.sel[1] == Some(r)];
                for p in 0..2 {
                    if !ok[p] {
                        self.pts[p] -= 1;
                        self.streak[p] = 0;
                    }
                }
                let winner = match ok {
                    [true, true] => {
                        let p1 = (50 + 8 * (self.streak[0] - self.streak[1])).clamp(20, 80);
                        Some(if (self.rng.below(100) as i32) < p1 {
                            0
                        } else {
                            1
                        })
                    }
                    [true, false] => Some(0),
                    [false, true] => Some(1),
                    _ => None,
                };
                if let Some(w) = winner {
                    self.pts[w] += 1;
                    self.streak[w] += 1;
                    self.streak[1 - w] = 0;
                }
                self.n += 1;
                self.sel = [None; 2];
                self.card = draw(&mut self.rng);
                self.ph = Ph::First;
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.n >= CARDS {
            return Some(by_score(self.pts[0], self.pts[1]));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let base = 100 * (self.pts[a] - self.pts[1 - a]);
        let r = right(&self.card);
        match self.sel[a] {
            Some(k) if k == r => base + 50,
            Some(_) => base - 100,
            None => base,
        }
    }
    fn show(&self) -> String {
        let viewer = self.viewer.map(pi).unwrap_or(pi(self.side()));
        let mut s = format!(
            "Obiectul Fulger — cartea {}/{} | puncte J1 {}, J2 {}\n",
            (self.n + 1).min(CARDS),
            CARDS,
            self.pts[0],
            self.pts[1]
        );
        s.push_str(&format!(
            "Cartea: {} {} + {} {}\n",
            OBJ[self.card[0].0], COL[self.card[0].1], OBJ[self.card[1].0], COL[self.card[1].1]
        ));
        s.push_str(&format!(
            "Obiecte: {}\n",
            (0..5)
                .map(|o| format!("{}={} ({})", o + 1, OBJ[o], COL[o]))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        if let Some(k) = self.sel[viewer] {
            s.push_str(&format!("Alegerea ta: {}\n", k + 1));
        }
        s.push_str(&format!(
            "La mutare: {} (prind k | verific)\n",
            pname(self.side())
        ));
        s
    }
    fn view(&mut self, w: Option<Player>) {
        self.viewer = w;
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        crate::brand_util::check(ObiectFulger::with_seed, 100);
    }
    #[test]
    fn rule_gives_a_valid_object_and_choice_is_hidden() {
        let mut g = ObiectFulger::with_seed(5);
        for _ in 0..50 {
            let c = draw(&mut g.rng);
            assert!(right(&c) < 5);
        }
        g.step("prind 3");
        g.view(Some(Player::Two));
        assert!(!g.show().contains("Alegerea ta"));
        g.view(Some(Player::One));
        assert!(g.show().contains("Alegerea ta: 3"));
    }
    #[test]
    fn second_player_ai_ignores_first_choice() {
        let mut a = ObiectFulger::with_seed(5);
        a.step("prind 1");
        let mut b = a.clone();
        b.sel[0] = Some(4);
        for seed in 1..6 {
            assert_eq!(ai_of(&a, seed), ai_of(&b, seed));
        }
        assert_eq!(ai_of(&a, 1), Some(format!("prind {}", right(&a.card) + 1)));
    }
}
