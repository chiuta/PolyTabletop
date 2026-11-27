//! Codul din Indicii — mesaje cifrate și interceptare (variantă originală, 2 jucători).
//!
//! Fiecare jucător are patru cuvinte-cheie secrete (numerotate 1..4) din 16 cuvinte publice, fiecare
//! cu patru însușiri 0..3 (distanța dintre cuvinte = suma diferențelor). 8 runde, codificatorul
//! alternează. Mutarea forțată `trag` dă codificatorului un cod secret (trei cifre distincte) și șase
//! cuvinte-indiciu. Codificatorul dă trei indicii, câte unul pentru fiecare cifră: `indicii a,b,c`.
//! Partenerul său (un decodor automat care cunoaște cheile) alege pentru fiecare indiciu cheia cea mai
//! apropiată; dacă decodarea diferă de cod: o neînțelegere (a doua te face să pierzi). Adversarul
//! interceptează: `ghic x,y,z`; dacă ghicește codul, primește o interceptare (a doua îl face să câștige).
//! Mutarea forțată `verifica` dezvăluie codul și indiciile (istoric public). La final: interceptări minus
//! neînțelegeri.
#![allow(
    clippy::needless_range_loop,
    clippy::manual_is_multiple_of,
    clippy::manual_memcpy,
    clippy::collapsible_match
)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const WORDS: [&str; 16] = [
    "ancoră",
    "brad",
    "clepsidră",
    "dantelă",
    "eclipsă",
    "farfurie",
    "gong",
    "hamac",
    "iglu",
    "jungla",
    "kilt",
    "lalea",
    "moară",
    "nufăr",
    "oglindă",
    "pârghie",
];
const ROUNDS: u32 = 8;

fn feat(w: usize, f: usize) -> i32 {
    ((w * 7 + f * 11 + (w * w) / 3 + f * w) % 4) as i32
}
fn dist(a: usize, b: usize) -> i32 {
    (0..4).map(|f| (feat(a, f) - feat(b, f)).abs()).sum()
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Ph {
    Deal,
    Encode,
    Intercept,
    Resolve,
}

#[derive(Clone)]
pub struct Cod {
    viewer: Option<Player>,
    kw: [[usize; 4]; 2],
    code: [usize; 3],
    hand: Vec<usize>,
    clues: [usize; 3],
    guess: [usize; 3],
    hist: Vec<(usize, [usize; 3], [usize; 3])>,
    icept: [i32; 2],
    miss: [i32; 2],
    round: u32,
    ph: Ph,
    rng: Rng,
}

pub fn new() -> Cod {
    Cod::with_seed(seed_now())
}

fn perms3of4() -> Vec<[usize; 3]> {
    let mut v = Vec::new();
    for a in 0..4 {
        for b in 0..4 {
            for c in 0..4 {
                if a != b && b != c && a != c {
                    v.push([a, b, c]);
                }
            }
        }
    }
    v
}

impl Cod {
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut kw = [[0; 4]; 2];
        let mut pool: Vec<usize> = (0..16).collect();
        shuf(&mut pool, &mut rng);
        for p in 0..2 {
            for k in 0..4 {
                kw[p][k] = pool[p * 4 + k];
            }
        }
        Cod {
            viewer: None,
            kw,
            code: [0, 1, 2],
            hand: Vec::new(),
            clues: [0; 3],
            guess: [0; 3],
            hist: Vec::new(),
            icept: [0; 2],
            miss: [0; 2],
            round: 0,
            ph: Ph::Deal,
            rng,
        }
    }
    fn enc(&self) -> usize {
        (self.round % 2) as usize
    }
    fn decode(&self, e: usize, w: usize) -> usize {
        (0..4).min_by_key(|&k| (dist(w, self.kw[e][k]), k)).unwrap()
    }
    fn decoded_ok(&self, e: usize, code: &[usize; 3], clues: &[usize; 3]) -> bool {
        (0..3).all(|i| self.decode(e, clues[i]) == code[i])
    }
    /// Cât de plauzibil e `g` ca cod, după indiciile din istoricul public ale codificatorului `e`.
    fn plaus(&self, e: usize, clues: &[usize; 3], g: &[usize; 3]) -> i32 {
        let mut s = 0;
        for i in 0..3 {
            let past: Vec<usize> = self
                .hist
                .iter()
                .filter(|h| h.0 == e)
                .filter_map(|h| h.1.iter().position(|&d| d == g[i]).map(|pos| h.2[pos]))
                .collect();
            s += match past.iter().map(|&w| dist(w, clues[i])).min() {
                Some(d) => -d,
                None => -6,
            };
        }
        s
    }
    fn best_intercept(&self, e: usize, clues: &[usize; 3]) -> [usize; 3] {
        perms3of4()
            .into_iter()
            .max_by_key(|g| (self.plaus(e, clues, g), std::cmp::Reverse(*g)))
            .unwrap()
    }
}

impl Mini for Cod {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        let t = match self.ph {
            Ph::Deal | Ph::Resolve => 0,
            Ph::Encode => self.enc(),
            Ph::Intercept => 1 - self.enc(),
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
            Ph::Encode => {
                let mut v = Vec::new();
                for &a in &self.hand {
                    for &b in &self.hand {
                        for &c in &self.hand {
                            if a != b && b != c && a != c {
                                v.push(format!("indicii {a},{b},{c}"));
                            }
                        }
                    }
                }
                v
            }
            Ph::Intercept => perms3of4()
                .iter()
                .map(|g| format!("ghic {},{},{}", g[0] + 1, g[1] + 1, g[2] + 1))
                .collect(),
        }
    }
    fn step(&mut self, m: &str) {
        match m {
            "trag" => {
                let mut d: Vec<usize> = (0..4).collect();
                shuf(&mut d, &mut self.rng);
                self.code = [d[0], d[1], d[2]];
                let mut pool: Vec<usize> = (0..16).collect();
                shuf(&mut pool, &mut self.rng);
                self.hand = pool[..6].to_vec();
                self.ph = Ph::Encode;
            }
            "verifica" => {
                let e = self.enc();
                if !self.decoded_ok(e, &self.code, &self.clues) {
                    self.miss[e] += 1;
                }
                if self.guess == self.code && self.round > 0 {
                    self.icept[1 - e] += 1;
                }
                self.hist.push((e, self.code, self.clues));
                self.round += 1;
                self.ph = Ph::Deal;
            }
            _ => {
                let rest = m.split(' ').nth(1).unwrap();
                let n: Vec<usize> = rest.split(',').map(|x| x.parse().unwrap()).collect();
                if m.starts_with("indicii") {
                    self.clues = [n[0], n[1], n[2]];
                    self.ph = Ph::Intercept;
                } else {
                    self.guess = [n[0] - 1, n[1] - 1, n[2] - 1];
                    self.ph = Ph::Resolve;
                }
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        for p in 0..2 {
            if self.icept[p] >= 2 {
                return Some(Outcome::Win(if p == 0 { Player::One } else { Player::Two }));
            }
            if self.miss[p] >= 2 {
                return Some(Outcome::Win(if p == 0 { Player::Two } else { Player::One }));
            }
        }
        if self.round >= ROUNDS {
            Some(by_score(
                self.icept[0] - self.miss[0],
                self.icept[1] - self.miss[1],
            ))
        } else {
            None
        }
    }
    fn eval(&self, p: Player) -> i32 {
        let i = pi(p);
        let o = 1 - i;
        let mut e = 60 * ((self.icept[i] - self.miss[i]) - (self.icept[o] - self.miss[o]));
        match self.ph {
            Ph::Intercept if i == self.enc() => {
                // codificatorul își cunoaște cheile și codul
                e += if self.decoded_ok(i, &self.code, &self.clues) {
                    30
                } else {
                    -60
                };
                let g = self.best_intercept(i, &self.clues);
                if g == self.code && self.round > 0 {
                    e -= 40;
                }
            }
            Ph::Resolve if i != self.enc() => {
                e += 4 * self.plaus(self.enc(), &self.clues, &self.guess);
            }
            _ => {}
        }
        e
    }
    fn show(&self) -> String {
        let me = self.viewer.map(pi).unwrap_or(pi(self.side()));
        let mut s = format!(
            "Codul din Indicii — runda {}/{} | interceptări J1 {} / J2 {}, neînțelegeri J1 {} / J2 {}\n",
            (self.round + 1).min(ROUNDS),
            ROUNDS,
            self.icept[0],
            self.icept[1],
            self.miss[0],
            self.miss[1]
        );
        s.push_str("Cuvinte [4 însușiri]: ");
        let w: Vec<String> = (0..16)
            .map(|i| {
                format!(
                    "{i}:{} {:?}",
                    WORDS[i],
                    (0..4).map(|f| feat(i, f)).collect::<Vec<_>>()
                )
            })
            .collect();
        s.push_str(&w.join("; "));
        s.push('\n');
        let k: Vec<String> = (0..4)
            .map(|i| format!("{}={}", i + 1, WORDS[self.kw[me][i]]))
            .collect();
        s.push_str(&format!("Cheile tale (J{}): {}\n", me + 1, k.join(", ")));
        if self.ph != Ph::Deal {
            s.push_str(&format!("Codificator J{}\n", self.enc() + 1));
            if me == self.enc() {
                s.push_str(&format!(
                    "Codul tău: {:?}\n",
                    self.code.iter().map(|d| d + 1).collect::<Vec<_>>()
                ));
            }
            if self.ph == Ph::Encode {
                s.push_str(&format!("Indicii disponibile: {:?}\n", self.hand));
            } else {
                let c: Vec<&str> = self.clues.iter().map(|&w| WORDS[w]).collect();
                s.push_str(&format!("Indicii date: {c:?}\n"));
            }
        }
        for h in self.hist.iter().rev().take(4) {
            let c: Vec<String> = (0..3)
                .map(|i| format!("{}->{}", h.1[i] + 1, WORDS[h.2[i]]))
                .collect();
            s.push_str(&format!("  istoric J{}: {}\n", h.0 + 1, c.join(", ")));
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
    fn cod_ok() {
        check(Cod::with_seed, 300);
    }

    #[test]
    fn interceptorul_nu_citeste_cheile_sau_codul() {
        polytabletop_core::ai::set_time_budget_override_ms(Some(6));
        let mut g = Cod::with_seed(2);
        g.step("trag");
        let m = g.moves()[10].clone();
        g.step(&m);
        assert!(g.ph == Ph::Intercept);
        let a = ai_of(&g, 9);
        let mut h = g.clone();
        h.code = [3, 2, 1];
        h.kw[g.enc()] = [1, 2, 3, 4];
        assert_eq!(a, ai_of(&h, 9));
    }
}

#[cfg(test)]
mod a1_blind {
    use super::*;
    use crate::ai_audit_a1::{audit, shuffle};
    use crate::brand_util::B;

    #[test]
    fn ai_ignores_opponent_keys_code_and_clue_hand_at_all_levels() {
        audit(
            "cod_indicii",
            |s| B(Cod::with_seed(s)),
            1..=4,
            1,
            80,
            |g, salt| {
                let me = pi(Mini::side(&g.0));
                let foe = 1 - me;
                let enc = g.0.enc();
                let mut free: Vec<usize> = (0..16).filter(|w| !g.0.kw[me].contains(w)).collect();
                shuffle(&mut free, salt);
                for i in 0..4 {
                    g.0.kw[foe][i] = free[i];
                }
                if g.0.ph == Ph::Intercept && me != enc {
                    let mut d: Vec<usize> = (0..4).collect();
                    shuffle(&mut d, salt + 1);
                    g.0.code = [d[0], d[1], d[2]];
                    let mut pool: Vec<usize> = (0..16).collect();
                    shuffle(&mut pool, salt + 2);
                    g.0.hand = pool[..6].to_vec();
                }
                g.0.rng = Rng::new(salt ^ 5);
            },
        );
    }
}
