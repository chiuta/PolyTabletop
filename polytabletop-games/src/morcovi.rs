//! Cursa Morcovilor — cursă cu resursă de energie (variantă originală).
//!
//! Poteca are 41 de câmpuri (0 = start, 40 = sosire). Fiecare animal pornește cu 60 de morcovi.
//! Mutarea înainte cu k câmpuri costă k(k+1)/2 morcovi (`avans k`); nu poți ateriza pe câmp
//! ocupat. Câmp de morcovi (c): aterizând alegi `avans k:iau` (+10) sau `avans k:dau` (−10).
//! Câmp de salată (s, la 12, 24, 36): nu se trece fără oprire; mănânci (+20 morcovi) și sari
//! următoarea tură. `inapoi` te duce pe cel mai apropiat câmp de morcovi liber din urmă și
//! primești 10 morcovi pentru fiecare câmp parcurs. `odihna` (+10) dacă nu ai altă mutare.
//! Câștigă primul care ajunge la 40 având după mutare cel mult 20 de morcovi; 400 de mutări: remiză.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const FIN: usize = 40;

#[derive(Clone)]
pub struct Morcovi {
    kind: [u8; 41], // 0 normal, 1 morcov, 2 salată
    pos: [usize; 2],
    car: [i32; 2],
    skip: [bool; 2],
    side: Player,
    ply: u32,
    winner: Option<Player>,
}

impl Morcovi {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut kind = [0u8; 41];
        for i in 1..40 {
            if i % 12 == 0 {
                kind[i] = 2;
            } else if rng.below(100) < 28 {
                kind[i] = 1;
            }
        }
        Morcovi {
            kind,
            pos: [0, 0],
            car: [60, 60],
            skip: [false; 2],
            side: Player::One,
            ply: 0,
            winner: None,
        }
    }
    fn next_lettuce(&self, p: usize) -> usize {
        (p + 1..=FIN)
            .find(|&i| i == FIN || self.kind[i] == 2)
            .unwrap()
    }
}

impl Default for Morcovi {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Morcovi {
    fn depth(&self) -> u32 {
        3
    }
    fn side(&self) -> Player {
        self.side
    }
    fn moves(&self) -> Vec<String> {
        let me = pi(self.side);
        if self.skip[me] {
            return vec!["treci".into()];
        }
        let mut v = Vec::new();
        let lim = self.next_lettuce(self.pos[me]);
        for k in 1..=(lim - self.pos[me]) {
            let cost = (k * (k + 1) / 2) as i32;
            let t = self.pos[me] + k;
            if cost > self.car[me] || (t != FIN && self.pos[1 - me] == t) {
                continue;
            }
            if t == FIN {
                if self.car[me] - cost <= 20 {
                    v.push(format!("avans {}", k));
                }
            } else if self.kind[t] == 1 {
                v.push(format!("avans {}:iau", k));
                if self.car[me] - cost >= 10 {
                    v.push(format!("avans {}:dau", k));
                }
            } else {
                v.push(format!("avans {}", k));
            }
        }
        if let Some(b) = (1..self.pos[me])
            .rev()
            .find(|&i| self.kind[i] == 1 && self.pos[1 - me] != i)
        {
            let _ = b;
            v.push("inapoi".into());
        }
        if v.is_empty() {
            v.push("odihna".into());
        }
        v
    }
    fn step(&mut self, m: &str) {
        let me = pi(self.side);
        self.ply += 1;
        if m == "treci" {
            self.skip[me] = false;
        } else if m == "odihna" {
            self.car[me] += 10;
        } else if m == "inapoi" {
            let b = (1..self.pos[me])
                .rev()
                .find(|&i| self.kind[i] == 1 && self.pos[1 - me] != i)
                .unwrap();
            self.car[me] += 10 * (self.pos[me] - b) as i32;
            self.pos[me] = b;
        } else {
            let body = &m[6..];
            let (ks, act) = body.split_once(':').unwrap_or((body, ""));
            let k: usize = ks.parse().unwrap();
            self.car[me] -= (k * (k + 1) / 2) as i32;
            self.pos[me] += k;
            match act {
                "iau" => self.car[me] += 10,
                "dau" => self.car[me] -= 10,
                _ => {}
            }
            if self.pos[me] < FIN && self.kind[self.pos[me]] == 2 {
                self.car[me] += 20;
                self.skip[me] = true;
            }
            if self.pos[me] == FIN {
                self.winner = Some(self.side);
            }
        }
        self.side = self.side.other();
    }
    fn result(&self) -> Option<Outcome> {
        if let Some(w) = self.winner {
            return Some(Outcome::Win(w));
        }
        if self.ply >= 400 {
            return Some(match self.pos[0].cmp(&self.pos[1]) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                _ => Outcome::Draw,
            });
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let f = |i: usize| 4 * self.pos[i] as i32 + self.car[i] / 3 - (self.car[i] - 40).max(0) / 2;
        f(a) - f(1 - a)
    }
    fn show(&self) -> String {
        let mut s =
            String::from("Cursa Morcovilor — câmpuri: . normal, c morcovi, S salată, F sosire\n");
        for i in 0..=FIN {
            let mut ch = match self.kind[i] {
                1 => 'c',
                2 => 'S',
                _ => '.',
            };
            if i == FIN {
                ch = 'F';
            }
            let mut t = ch.to_string();
            for p in 0..2 {
                if self.pos[p] == i {
                    t = format!("{}", p + 1);
                }
            }
            s.push_str(&t);
        }
        s.push('\n');
        for p in 0..2 {
            s.push_str(&format!(
                "J{}: câmp {} | morcovi {}{}\n",
                p + 1,
                self.pos[p],
                self.car[p],
                if self.skip[p] { " | sare tura" } else { "" }
            ));
        }
        s.push_str(&format!(
            "La mutare: {} (avans k[:iau|:dau] | inapoi | odihna | treci)\n",
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
        crate::brand_util::check(Morcovi::with_seed, 500);
    }
    #[test]
    fn cost_is_triangular() {
        let mut g = Morcovi::with_seed(1);
        g.kind = [0; 41];
        g.step("avans 4");
        assert_eq!(g.car[0], 50);
    }
}
