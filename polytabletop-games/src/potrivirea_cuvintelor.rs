//! Potrivirea Cuvintelor — cărți-substantiv și un arbitru cu gusturi ascunse (variantă originală, 2 jucători).
//!
//! 12 substantive au (public) o fișă de însușiri 0..9: dulce, puternic, vechi, periculos, amuzant.
//! 10 runde: mutarea forțată `trag` dezvăluie o însușire cerută și completează mâinile (5 cărți,
//! ascunse). Fiecare joacă pe rând, cu fața în jos, o carte: `da i`. Mutarea forțată `judeca`
//! dezvăluie cărțile: arbitrul (motorul) alege cartea cu valoarea cea mai mare la însușirea cerută,
//! plus un „gust” ascuns al lui pentru fiecare substantiv (−2..+2, fix toată partida). Câștigătorul
//! rundei ia un punct; la egalitate nimeni. Istoricul rezultatelor e public: poți deduce gusturile.
#![allow(
    clippy::needless_range_loop,
    clippy::manual_is_multiple_of,
    clippy::manual_memcpy,
    clippy::collapsible_match
)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const NAMES: [&str; 12] = [
    "leu",
    "piatră",
    "zahăr",
    "foc",
    "gheață",
    "pisică",
    "munte",
    "ceas",
    "spadă",
    "clovn",
    "ciocolată",
    "fantomă",
];
const ATTR: [&str; 5] = ["dulce", "puternic", "vechi", "periculos", "amuzant"];
const PROF: [[i32; 5]; 12] = [
    [0, 9, 3, 8, 1],
    [0, 6, 9, 2, 0],
    [9, 0, 2, 0, 2],
    [0, 5, 4, 9, 1],
    [1, 3, 5, 5, 0],
    [3, 2, 2, 3, 6],
    [0, 9, 9, 3, 0],
    [0, 1, 7, 0, 1],
    [0, 5, 8, 9, 0],
    [2, 1, 1, 2, 9],
    [9, 1, 1, 0, 3],
    [0, 2, 9, 6, 4],
];
const ROUNDS: u32 = 10;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Ph {
    Deal,
    Play,
    Judge,
}

#[derive(Clone)]
pub struct Potrivirea {
    viewer: Option<Player>,
    hand: [Vec<u8>; 2],
    played: [Option<u8>; 2],
    crit: usize,
    taste: [i32; 12],
    hist: Vec<(usize, u8, u8)>,
    score: [i32; 2],
    round: u32,
    ph: Ph,
    rng: Rng,
}

pub fn new() -> Potrivirea {
    Potrivirea::with_seed(seed_now())
}

impl Potrivirea {
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut taste = [0; 12];
        for t in taste.iter_mut() {
            *t = rng.below(5) as i32 - 2;
        }
        Potrivirea {
            viewer: None,
            hand: [Vec::new(), Vec::new()],
            played: [None; 2],
            crit: 0,
            taste,
            hist: Vec::new(),
            score: [0; 2],
            round: 0,
            ph: Ph::Deal,
            rng,
        }
    }
    /// Estimarea gustului pe baza istoricului public (câștigătorul +1, pierzătorul −1).
    fn est_taste(&self) -> [i32; 12] {
        let mut t = [0; 12];
        for &(a, w, l) in &self.hist {
            let _ = a;
            t[w as usize] += 1;
            t[l as usize] -= 1;
        }
        t
    }
    fn value(&self, noun: u8, attr: usize, taste: &[i32; 12]) -> i32 {
        PROF[noun as usize][attr] + taste[noun as usize]
    }
}

impl Mini for Potrivirea {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        match self.ph {
            Ph::Play if self.played[0].is_some() => Player::Two,
            _ => Player::One,
        }
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            Ph::Deal => vec!["trag".into()],
            Ph::Judge => vec!["judeca".into()],
            Ph::Play => {
                let p = if self.played[0].is_some() { 1 } else { 0 };
                (0..self.hand[p].len()).map(|i| format!("da {i}")).collect()
            }
        }
    }
    fn step(&mut self, m: &str) {
        match m {
            "trag" => {
                self.crit = self.rng.below(5);
                for p in 0..2 {
                    while self.hand[p].len() < 5 {
                        let n = self.rng.below(12) as u8;
                        self.hand[p].push(n);
                    }
                }
                self.played = [None; 2];
                self.ph = Ph::Play;
            }
            "judeca" => {
                let a = self.played[0].unwrap();
                let b = self.played[1].unwrap();
                let va = self.value(a, self.crit, &self.taste);
                let vb = self.value(b, self.crit, &self.taste);
                if va > vb {
                    self.score[0] += 1;
                    self.hist.push((self.crit, a, b));
                } else if vb > va {
                    self.score[1] += 1;
                    self.hist.push((self.crit, b, a));
                }
                self.round += 1;
                self.ph = Ph::Deal;
            }
            _ => {
                let i: usize = m[3..].parse().unwrap();
                let p = if self.played[0].is_some() { 1 } else { 0 };
                let c = self.hand[p].remove(i);
                self.played[p] = Some(c);
                if p == 1 {
                    self.ph = Ph::Judge;
                }
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
        if self.ph != Ph::Deal {
            if let Some(c) = self.played[i] {
                // valoarea mea estimată față de media estimată a unei cărți necunoscute
                let t = self.est_taste();
                let mine = self.value(c, self.crit, &t);
                let avg: i32 = (0..12)
                    .map(|n| self.value(n as u8, self.crit, &t))
                    .sum::<i32>()
                    / 12;
                e += 3 * (mine - avg);
            }
        }
        e
    }
    fn show(&self) -> String {
        let me = self.viewer.map(pi).unwrap_or(pi(self.side()));
        let mut s = format!(
            "Potrivirea Cuvintelor — runda {}/{} | J1 {} / J2 {}\n",
            (self.round + 1).min(ROUNDS),
            ROUNDS,
            self.score[0],
            self.score[1]
        );
        if self.ph != Ph::Deal {
            s.push_str(&format!("Însușirea cerută: {}\n", ATTR[self.crit]));
            s.push_str(&format!(
                "Cărți jucate: J1 {}, J2 {}\n",
                if self.played[0].is_some() {
                    "(cu fața în jos)"
                } else {
                    "-"
                },
                if self.played[1].is_some() {
                    "(cu fața în jos)"
                } else {
                    "-"
                }
            ));
            let h: Vec<String> = self.hand[me]
                .iter()
                .enumerate()
                .map(|(i, &n)| format!("{i}:{} {:?}", NAMES[n as usize], PROF[n as usize]))
                .collect();
            s.push_str(&format!(
                "Mâna lui J{} (dulce,puternic,vechi,periculos,amuzant): {}\n",
                me + 1,
                h.join(", ")
            ));
        }
        let last: Vec<String> = self
            .hist
            .iter()
            .rev()
            .take(5)
            .map(|&(a, w, l)| {
                format!(
                    "{}: {} bate {}",
                    ATTR[a], NAMES[w as usize], NAMES[l as usize]
                )
            })
            .collect();
        s.push_str(&format!(
            "Ultimele hotărâri ale arbitrului: {}\n",
            last.join("; ")
        ));
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
    fn potrivirea_ok() {
        check(Potrivirea::with_seed, 200);
    }

    #[test]
    fn al_doilea_nu_vede_cartea_primului() {
        polytabletop_core::ai::set_time_budget_override_ms(Some(6));
        let mut g = Potrivirea::with_seed(3);
        g.step("trag");
        g.step("da 0");
        let a = ai_of(&g, 5);
        let mut h = g.clone();
        h.played[0] = Some(((g.played[0].unwrap() as usize + 5) % 12) as u8);
        h.taste = [0; 12];
        assert_eq!(a, ai_of(&h, 5));
    }
}
