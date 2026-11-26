//! Misterul Locației — cine nu știe unde e? (variantă originală, 2 jucători).
//!
//! 4 runde. Mutarea forțată `trag` alege o locație secretă din 10 și spionii: doar J1, doar J2 sau
//! amândoi (a treia parte din cazuri). Cine nu e spion știe locația; spionul nu o știe și nu știe dacă
//! celălalt e spion. Fiecare locație are șase însușiri da/nu (publice). Șase schimburi de întrebări
//! (alternează): `intreb a` (însușirea a), răspuns `da` sau `nu`. Apoi, în secret, fiecare alege:
//! `acuz` (adversarul e spion: +2 dacă e, altfel adversarul ia +2), `loc k` (spion care ghicește
//! locația: +3 dacă e corectă, altfel −1; un nespion care ghicește pierde 1) sau `pas` (spion neacuzat: +1).
//! Mutarea forțată `verifica` dezvăluie alegerile. Câștigă scorul mai mare.
#![allow(
    clippy::needless_range_loop,
    clippy::manual_is_multiple_of,
    clippy::manual_memcpy,
    clippy::collapsible_match
)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const LOC: [&str; 10] = [
    "plajă",
    "spital",
    "circ",
    "bibliotecă",
    "gară",
    "stație polară",
    "cazino",
    "școală",
    "cinema",
    "far",
];
const ATTR: [&str; 6] = [
    "în interior",
    "cald",
    "aglomerat",
    "zgomotos",
    "lângă apă",
    "se vizitează noaptea",
];
const ROUNDS: u32 = 4;

fn bit(l: usize, a: usize) -> bool {
    (l * 5 + a * 3 + (l * a) / 2 + l / 3) % 2 == 0
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Act {
    Pas,
    Acuz,
    Loc(usize),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Ph {
    Deal,
    Ask(usize),
    Answer(usize),
    Accuse(usize),
    Resolve,
}

#[derive(Clone)]
pub struct Misterul {
    viewer: Option<Player>,
    spy: [bool; 2],
    loc: usize,
    qa: Vec<(usize, usize, bool)>,
    pending_q: usize,
    k: usize,
    acts: [Option<Act>; 2],
    score: [i32; 2],
    round: u32,
    ph: Ph,
    rng: Rng,
}

pub fn new() -> Misterul {
    Misterul::with_seed(seed_now())
}

impl Misterul {
    pub fn with_seed(seed: u64) -> Self {
        Misterul {
            viewer: None,
            spy: [false; 2],
            loc: 0,
            qa: Vec::new(),
            pending_q: 0,
            k: 0,
            acts: [None; 2],
            score: [0; 2],
            round: 0,
            ph: Ph::Deal,
            rng: Rng::new(seed),
        }
    }
    fn first(&self) -> usize {
        (self.round % 2) as usize
    }
    /// Locațiile compatibile cu răspunsurile primite de `p` (le tratează ca adevărate).
    fn cands(&self, p: usize) -> Vec<usize> {
        (0..10)
            .filter(|&l| {
                self.qa
                    .iter()
                    .filter(|q| q.0 != p)
                    .all(|&(_, a, ans)| bit(l, a) == ans)
            })
            .collect()
    }
    /// Probabilitate (×100) ca adversarul lui `p` să fie spion, din punctul de vedere al lui p.
    fn p_opp_spy(&self, p: usize) -> i32 {
        if self.spy[p] {
            return 50;
        }
        let bad = self
            .qa
            .iter()
            .filter(|q| q.0 == p)
            .any(|&(_, a, ans)| bit(self.loc, a) != ans);
        if bad {
            return 100;
        }
        let n = self.qa.iter().filter(|q| q.0 == p).count() as i32;
        // un spion răspunde corect cu probabilitate ~0,6 pe întrebare
        let w = 0.6f64.powi(n);
        (100.0 * w / (w + 1.0)) as i32
    }
}

impl Mini for Misterul {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        let t = match self.ph {
            Ph::Ask(p) | Ph::Accuse(p) => p,
            Ph::Answer(p) => 1 - p,
            _ => 0,
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
            Ph::Ask(_) => (0..6).map(|a| format!("intreb {a}")).collect(),
            Ph::Answer(_) => vec!["da".into(), "nu".into()],
            Ph::Accuse(_) => {
                let mut v = vec!["pas".to_string(), "acuz".to_string()];
                for k in 0..10 {
                    v.push(format!("loc {k}"));
                }
                v
            }
        }
    }
    fn step(&mut self, m: &str) {
        match m {
            "trag" => {
                self.spy = match self.rng.below(3) {
                    0 => [true, false],
                    1 => [false, true],
                    _ => [true, true],
                };
                self.loc = self.rng.below(10);
                self.qa.clear();
                self.k = 0;
                self.acts = [None; 2];
                self.ph = Ph::Ask(self.first());
            }
            "verifica" => {
                let a = [self.acts[0].unwrap(), self.acts[1].unwrap()];
                for p in 0..2 {
                    let o = 1 - p;
                    match a[p] {
                        Act::Acuz => {
                            if self.spy[o] {
                                self.score[p] += 2;
                            } else {
                                self.score[o] += 2;
                            }
                        }
                        Act::Loc(k) => {
                            if self.spy[p] && k == self.loc {
                                self.score[p] += 3;
                            } else {
                                self.score[p] -= 1;
                            }
                        }
                        Act::Pas => {
                            if self.spy[p] && a[o] != Act::Acuz {
                                self.score[p] += 1;
                            }
                        }
                    }
                }
                self.round += 1;
                self.ph = Ph::Deal;
            }
            "da" | "nu" => {
                if let Ph::Answer(asker) = self.ph {
                    self.qa.push((asker, self.pending_q, m == "da"));
                    self.k += 1;
                    self.ph = if self.k >= 6 {
                        Ph::Accuse(0)
                    } else {
                        Ph::Ask((self.first() + self.k) % 2)
                    };
                }
            }
            "pas" | "acuz" => {
                if let Ph::Accuse(p) = self.ph {
                    self.acts[p] = Some(if m == "pas" { Act::Pas } else { Act::Acuz });
                    self.ph = if p == 0 { Ph::Accuse(1) } else { Ph::Resolve };
                }
            }
            _ if m.starts_with("loc") => {
                if let Ph::Accuse(p) = self.ph {
                    self.acts[p] = Some(Act::Loc(m[4..].parse().unwrap()));
                    self.ph = if p == 0 { Ph::Accuse(1) } else { Ph::Resolve };
                }
            }
            _ => {
                if let Ph::Ask(p) = self.ph {
                    self.pending_q = m[7..].parse().unwrap();
                    self.ph = Ph::Answer(p);
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
        match self.ph {
            Ph::Answer(asker) if asker == i => {
                // am întrebat: preferăm întrebările care împart echilibrat posibilitățile (celui ce nu știe îi e greu să ghicească)
                let c = if self.spy[i] {
                    self.cands(i)
                } else {
                    (0..10).collect()
                };
                let yes = c.iter().filter(|&&l| bit(l, self.pending_q)).count() as i32;
                let no = c.len() as i32 - yes;
                e += 6 * yes.min(no);
                let repeated = self.qa.iter().filter(|q| q.1 == self.pending_q).count() as i32;
                e -= 10 * repeated;
            }
            Ph::Accuse(1) | Ph::Resolve => {
                if let Some(act) = self.acts[i] {
                    let ps = self.p_opp_spy(i);
                    e += match act {
                        Act::Acuz => (ps * 2 - (100 - ps) * 2) / 2,
                        Act::Pas => {
                            if self.spy[i] {
                                50 - 30
                            } else {
                                0
                            }
                        }
                        Act::Loc(k) => {
                            if self.spy[i] {
                                let c = self.cands(i);
                                if c.contains(&k) {
                                    let pr = 100 / c.len() as i32;
                                    (3 * pr - (100 - pr)) / 2
                                } else {
                                    -50
                                }
                            } else {
                                -50
                            }
                        }
                    };
                }
            }
            Ph::Ask(_) | Ph::Accuse(0) if self.k > 0 => {
                // am răspuns la ultima întrebare (starea e după răspunsul meu), dacă ultimul răspuns e al meu
                if let Some(&(asker, a, ans)) = self.qa.last() {
                    if asker != i {
                        if !self.spy[i] {
                            e += if bit(self.loc, a) == ans { 40 } else { -40 };
                        } else {
                            let c = self.cands(i);
                            if !c.is_empty() {
                                let share = c.iter().filter(|&&l| bit(l, a) == ans).count() as i32;
                                e += 40 * share / c.len() as i32;
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        e
    }
    fn show(&self) -> String {
        let me = self.viewer.map(pi).unwrap_or(pi(self.side()));
        let mut s = format!(
            "Misterul Locației — runda {}/{} | J1 {} / J2 {}\n",
            (self.round + 1).min(ROUNDS),
            ROUNDS,
            self.score[0],
            self.score[1]
        );
        s.push_str("Locuri: ");
        let l: Vec<String> = (0..10)
            .map(|i| {
                format!(
                    "{i}:{} [{}]",
                    LOC[i],
                    (0..6)
                        .map(|a| if bit(i, a) { 'D' } else { 'n' })
                        .collect::<String>()
                )
            })
            .collect();
        s.push_str(&l.join(", "));
        s.push('\n');
        s.push_str(&format!("Însușiri: {:?}\n", ATTR));
        if self.ph != Ph::Deal {
            if self.spy[me] {
                s.push_str(&format!("J{}: ești spion — nu știi locația\n", me + 1));
            } else {
                s.push_str(&format!("J{}: locația este {}\n", me + 1, LOC[self.loc]));
            }
            for &(a, q, ans) in &self.qa {
                s.push_str(&format!(
                    "  J{} întreabă „{}”: {}\n",
                    a + 1,
                    ATTR[q],
                    if ans { "da" } else { "nu" }
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
    fn misterul_ok() {
        check(Misterul::with_seed, 300);
    }
}
