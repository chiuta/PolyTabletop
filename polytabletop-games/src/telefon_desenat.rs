//! Telefonul Desenat — schițe parțiale și încredere (variantă originală, 2 jucători).
//!
//! 16 obiecte au fișe cu patru însușiri 0..3 (mărime, viteză, moliciune, zgomot), publice. În fiecare
//! rundă (8, desenatorul alternează) mutarea forțată `trag` alege un obiect secret și patru candidați
//! (inclusiv el). Desenatorul „schițează” două însușiri: `schita a,b` (sinceră: arată valorile
//! obiectului secret) sau `momeala a,b` (arată valorile unui candidat fals care diferă; fiecare jucător
//! are doar 2 momeli pe partidă, numărul rămas e public). Ghicitorul vede doar cele două valori și alege
//! `ghic k`. Mutarea forțată `verifica`: sincer și ghicit: ghicitor +3, desenator +1; momeală și ghicitorul
//! alege secretul: ghicitorul +3; momeală și ghicitorul alege altceva: desenatorul +3.
//! Alegerile ghicitorilor (după sau împotriva schiței) devin publice; câștigă scorul mai mare.
#![allow(
    clippy::needless_range_loop,
    clippy::manual_is_multiple_of,
    clippy::manual_memcpy,
    clippy::collapsible_match
)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const NAMES: [&str; 16] = [
    "barcă",
    "umbrelă",
    "pernă",
    "tobă",
    "balon",
    "ciocan",
    "pisică",
    "avion",
    "ceapă",
    "clopot",
    "pătură",
    "tren",
    "bucșă",
    "vulpe",
    "ceasornic",
    "tunet",
];
const ROUNDS: u32 = 8;
const PAIRS: [(usize, usize); 6] = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];

fn feat(i: usize, f: usize) -> i32 {
    ((i * 5 + f * 3 + (i * f) / 2 + i / 4) % 4) as i32
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Ph {
    Deal,
    Draw,
    Guess,
    Resolve,
}

#[derive(Clone)]
pub struct Telefon {
    viewer: Option<Player>,
    cand: [usize; 4],
    secret: usize,
    sketch: Option<(usize, usize, bool)>,
    guess: usize,
    score: [i32; 2],
    round: u32,
    decept: u32,
    tokens: [u32; 2],
    anti: [u32; 2],
    guesses: [u32; 2],
    ph: Ph,
    rng: Rng,
}

pub fn new() -> Telefon {
    Telefon::with_seed(seed_now())
}

impl Telefon {
    pub fn with_seed(seed: u64) -> Self {
        Telefon {
            viewer: None,
            cand: [0; 4],
            secret: 0,
            sketch: None,
            guess: 0,
            score: [0; 2],
            round: 0,
            decept: 0,
            tokens: [2, 2],
            anti: [0; 2],
            guesses: [0; 2],
            ph: Ph::Deal,
            rng: Rng::new(seed),
        }
    }
    fn drawer(&self) -> usize {
        (self.round % 2) as usize
    }
    /// Candidatul afișat în schiță (secretul sau momeala).
    fn shown(&self, a: usize, b: usize, deceptive: bool) -> usize {
        if !deceptive {
            return self.secret;
        }
        for d in 1..4 {
            let k = (self.secret + d) % 4;
            let (x, y) = (self.cand[k], self.cand[self.secret]);
            if feat(x, a) != feat(y, a) || feat(x, b) != feat(y, b) {
                return k;
            }
        }
        self.secret
    }
    fn can_deceive(&self, a: usize, b: usize) -> bool {
        self.shown(a, b, true) != self.secret
    }
    fn matches(&self, k: usize, a: usize, b: usize, shown: usize) -> bool {
        feat(self.cand[k], a) == feat(self.cand[shown], a)
            && feat(self.cand[k], b) == feat(self.cand[shown], b)
    }
    /// Valori vizibile ghicitorului: (a, b, va, vb)
    fn visible(&self) -> Option<(usize, usize, i32, i32)> {
        self.sketch.map(|(a, b, d)| {
            let s = self.shown(a, b, d);
            (a, b, feat(self.cand[s], a), feat(self.cand[s], b))
        })
    }
    fn vis_match(&self, k: usize) -> bool {
        match self.visible() {
            Some((a, b, va, vb)) => feat(self.cand[k], a) == va && feat(self.cand[k], b) == vb,
            None => false,
        }
    }
    /// Rundele rămase (inclusiv cea curentă) în care `p` desenează.
    fn draws_left(&self, p: usize) -> u32 {
        (self.round..ROUNDS)
            .filter(|r| (r % 2) as usize == p)
            .count() as u32
    }
    /// Procentul estimat de momeală al desenatorului curent (public: momelile rămase / rundele de desen rămase).
    fn dec_pct(&self) -> i32 {
        let d = self.drawer();
        (100 * self.tokens[d] as i32 / self.draws_left(d).max(1) as i32).min(100)
    }
}

impl Mini for Telefon {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        let t = match self.ph {
            Ph::Deal | Ph::Draw => self.drawer(),
            Ph::Guess => 1 - self.drawer(),
            Ph::Resolve => 0,
        };
        if t == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            Ph::Deal => vec!["trag".into()],
            Ph::Resolve => vec!["verifica".into()],
            Ph::Draw => {
                let mut v = Vec::new();
                for &(a, b) in &PAIRS {
                    v.push(format!("schita {a},{b}"));
                    if self.tokens[self.drawer()] > 0 && self.can_deceive(a, b) {
                        v.push(format!("momeala {a},{b}"));
                    }
                }
                v
            }
            Ph::Guess => (0..4).map(|k| format!("ghic {k}")).collect(),
        }
    }
    fn step(&mut self, m: &str) {
        match m {
            "trag" => {
                let mut pool: Vec<usize> = (0..16).collect();
                shuf(&mut pool, &mut self.rng);
                for k in 0..4 {
                    self.cand[k] = pool[k];
                }
                self.secret = self.rng.below(4);
                self.sketch = None;
                self.ph = Ph::Draw;
            }
            "verifica" => {
                let (a, b, d) = self.sketch.unwrap();
                let _ = (a, b);
                let dr = self.drawer();
                let gs = 1 - dr;
                let right = self.guess == self.secret;
                let (a0, b0, _) = self.sketch.unwrap();
                let s0 = self.shown(a0, b0, d);
                self.guesses[gs] += 1;
                if !self.matches(self.guess, a0, b0, s0) {
                    self.anti[gs] += 1;
                }
                if !d {
                    if right {
                        self.score[gs] += 3;
                        self.score[dr] += 1;
                    }
                } else {
                    self.decept += 1;
                    self.tokens[dr] -= 1;
                    if right {
                        self.score[gs] += 3;
                    } else {
                        self.score[dr] += 3;
                    }
                }
                self.round += 1;
                self.ph = Ph::Deal;
            }
            _ if m.starts_with("ghic") => {
                self.guess = m[5..].parse().unwrap();
                self.ph = Ph::Resolve;
            }
            _ => {
                let d = m.starts_with("momeala");
                let rest = m.split(' ').nth(1).unwrap();
                let mut it = rest.split(',');
                let a: usize = it.next().unwrap().parse().unwrap();
                let b: usize = it.next().unwrap().parse().unwrap();
                self.sketch = Some((a, b, d));
                self.ph = Ph::Guess;
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.round >= ROUNDS {
            Some(by_score(self.score[0], self.score[1]))
        } else {
            None
        }
    }
    fn eval(&self, p: Player) -> i32 {
        let i = pi(p);
        let o = 1 - i;
        let mut e = 20 * (self.score[i] - self.score[o]);
        match self.ph {
            Ph::Guess => {
                // desenatorul tocmai a ales schița (se evaluează din perspectiva desenatorului)
                if let (true, Some((a, b, d))) = (i == self.drawer(), self.sketch) {
                    let sh = self.shown(a, b, d);
                    let k = (0..4).filter(|&k| self.matches(k, a, b, sh)).count() as i32;
                    let clarity = (4 - k) * 2;
                    let g = 1 - i;
                    let pt = 100 - 100 * (self.anti[g] as i32 + 1) / (self.guesses[g] as i32 + 2);
                    if d {
                        let rem = self.draws_left(i) as i32;
                        let cost = 12 * (rem - self.tokens[i] as i32 + 1).max(0);
                        e += (20 * (3 * pt + (100 - pt))) / 100 - cost + clarity;
                    } else {
                        e += -(20 * 14 * pt) / 1000 + clarity;
                    }
                }
            }
            Ph::Resolve => {
                if i == 1 - self.drawer() {
                    let m = self.vis_match(self.guess);
                    e += if self.dec_pct() < 41 {
                        if m {
                            40
                        } else {
                            -10
                        }
                    } else if m {
                        -15
                    } else {
                        10
                    };
                }
            }
            _ => {}
        }
        e
    }
    fn show(&self) -> String {
        let me = self.viewer.map(pi).unwrap_or(pi(self.side()));
        let mut s = format!(
            "Telefonul Desenat — runda {}/{} | J1 {} / J2 {} | momeli folosite: {} (rămase J1 {}, J2 {})\n",
            (self.round + 1).min(ROUNDS),
            ROUNDS,
            self.score[0],
            self.score[1],
            self.decept,
            self.tokens[0],
            self.tokens[1]
        );
        if self.ph != Ph::Deal {
            let c: Vec<String> = self
                .cand
                .iter()
                .enumerate()
                .map(|(k, &i)| format!("{k}:{}", NAMES[i]))
                .collect();
            s.push_str(&format!(
                "Desenator J{}; candidați: {}\n",
                self.drawer() + 1,
                c.join(", ")
            ));
            if me == self.drawer() {
                s.push_str(&format!("Obiectul tău secret: {}\n", self.secret));
            }
            for k in 0..4 {
                s.push_str(&format!(
                    "  {} [mărime,viteză,moliciune,zgomot] = {:?}\n",
                    NAMES[self.cand[k]],
                    (0..4).map(|f| feat(self.cand[k], f)).collect::<Vec<_>>()
                ));
            }
            if let Some((a, b, va, vb)) = self.visible() {
                s.push_str(&format!(
                    "Schiță: însușirea {a} = {va}, însușirea {b} = {vb}\n"
                ));
            }
        }
        s
    }
    fn view(&mut self, w: Option<Player>) {
        self.viewer = w;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn telefon_ok() {
        check(Telefon::with_seed, 200);
    }

    #[test]
    fn ghicitorul_nu_citeste_secretul() {
        polytabletop_core::ai::set_time_budget_override_ms(Some(6));
        let mut g = Telefon::with_seed(8);
        g.step("trag");
        g.step("schita 0,1");
        assert!(g.ph == Ph::Guess);
        let a = ai_of(&g, 2);
        // același cuvânt vizibil, alt candidat secret (schița arată valorile secretului inițial, deci păstrăm ce se vede)
        let mut h = g.clone();
        let sh = g.shown(0, 1, false);
        let other = (0..4)
            .find(|&k| k != sh && !g.matches(k, 0, 1, sh))
            .unwrap_or(sh);
        h.secret = other;
        h.sketch = Some((0, 1, true));
        // dacă schița falsă arată aceleași valori, alegerea trebuie să fie aceeași
        if h.visible() == g.visible() {
            assert_eq!(a, ai_of(&h, 2));
        }
    }
}
