//! Ambuteiaj — duel de mașini pe o tablă 6×6 (variantă originală, mecanică generală de „blocaj”).
//!
//! Două mașini-țintă: A (J1) pornește pe rândul 3 din stânga și trebuie să ajungă la marginea
//! dreaptă; B (J2) pornește pe rândul 4 din dreapta și trebuie să ajungă la marginea stângă.
//! La fiecare tură muți o singură mașină (oricare, inclusiv a adversarului) cu oricâte
//! pătrate pe axa ei: `A+2`, `C-1`. Mașinile de umplutură sunt verticale pe rândurile țintelor,
//! orice orientare în rest. Câștigă primul care își eliberează ținta; după 120 de mutări: remiză.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const N: i8 = 6;
const CAP: u32 = 120;

#[derive(Clone)]
struct Car {
    id: u8,
    r: i8,
    c: i8,
    len: i8,
    h: bool,
}

#[derive(Clone)]
pub struct Ambuteiaj {
    cars: Vec<Car>,
    side: Player,
    ply: u32,
}

fn cells(c: &Car) -> Vec<(i8, i8)> {
    (0..c.len)
        .map(|i| if c.h { (c.r, c.c + i) } else { (c.r + i, c.c) })
        .collect()
}

impl Ambuteiaj {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut cars = vec![
            Car {
                id: b'A',
                r: 2,
                c: 0,
                len: 2,
                h: true,
            },
            Car {
                id: b'B',
                r: 3,
                c: 4,
                len: 2,
                h: true,
            },
        ];
        let mut id = b'C';
        let mut tries = 0;
        while cars.len() < 11 && tries < 400 {
            tries += 1;
            let len = 2 + rng.below(2) as i8;
            let h = rng.below(2) == 0;
            let (r, c) = (rng.below(6) as i8, rng.below(6) as i8);
            let car = Car { id, r, c, len, h };
            let cs = cells(&car);
            if cs.iter().any(|&(r, c)| r >= N || c >= N) {
                continue;
            }
            if h && (r == 2 || r == 3) {
                continue;
            }
            if cs.iter().any(|x| cars.iter().any(|o| cells(o).contains(x))) {
                continue;
            }
            cars.push(car);
            id += 1;
        }
        Ambuteiaj {
            cars,
            side: Player::One,
            ply: 0,
        }
    }
    fn grid(&self) -> [[u8; 6]; 6] {
        let mut g = [[b'.'; 6]; 6];
        for c in &self.cars {
            for (r, k) in cells(c) {
                g[r as usize][k as usize] = c.id;
            }
        }
        g
    }
    fn cost(&self, who: usize) -> i32 {
        let g = self.grid();
        let car = &self.cars[who];
        if who == 0 {
            let mut k = 0;
            for c in car.c + 2..N {
                if g[car.r as usize][c as usize] != b'.' {
                    k += 3;
                }
            }
            k + (N - 2 - car.c) as i32
        } else {
            let mut k = 0;
            for c in 0..car.c {
                if g[car.r as usize][c as usize] != b'.' {
                    k += 3;
                }
            }
            k + car.c as i32
        }
    }
}

impl Default for Ambuteiaj {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Ambuteiaj {
    fn depth(&self) -> u32 {
        2
    }
    fn side(&self) -> Player {
        self.side
    }
    fn moves(&self) -> Vec<String> {
        let g = self.grid();
        let mut v = Vec::new();
        for car in &self.cars {
            for dir in [-1i8, 1] {
                for k in 1..N {
                    let d = dir * k;
                    let ok = if dir > 0 {
                        let (r, c) = if car.h {
                            (car.r, car.c + car.len - 1 + k)
                        } else {
                            (car.r + car.len - 1 + k, car.c)
                        };
                        r < N && c < N && g[r as usize][c as usize] == b'.'
                    } else {
                        let (r, c) = if car.h {
                            (car.r, car.c - k)
                        } else {
                            (car.r - k, car.c)
                        };
                        r >= 0 && c >= 0 && g[r as usize][c as usize] == b'.'
                    };
                    if !ok {
                        break;
                    }
                    v.push(format!("{}{:+}", car.id as char, d));
                }
            }
        }
        v
    }
    fn step(&mut self, m: &str) {
        let id = m.as_bytes()[0].to_ascii_uppercase();
        let d: i8 = m[1..].parse().unwrap();
        let car = self.cars.iter_mut().find(|c| c.id == id).unwrap();
        if car.h {
            car.c += d
        } else {
            car.r += d
        }
        self.ply += 1;
        self.side = self.side.other();
    }
    fn result(&self) -> Option<Outcome> {
        if self.cars[0].c + 2 == N {
            return Some(Outcome::Win(Player::One));
        }
        if self.cars[1].c == 0 {
            return Some(Outcome::Win(Player::Two));
        }
        if self.ply >= CAP {
            return Some(Outcome::Draw);
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let (a, b) = (self.cost(0), self.cost(1));
        let d = b - a;
        10 * if p == Player::One { d } else { -d }
    }
    fn show(&self) -> String {
        let g = self.grid();
        let mut s = format!("Ambuteiaj — A (J1) ieșire la dreapta pe rândul 3, B (J2) ieșire la stânga pe rândul 4 | tura {}\n", self.ply);
        for r in 0..6 {
            s.push_str(&format!("{} ", r + 1));
            for c in 0..6 {
                s.push(g[r][c] as char);
                s.push(' ');
            }
            s.push_str(if r == 2 {
                "→ ieșire J1"
            } else if r == 3 {
                "← ieșire J2"
            } else {
                ""
            });
            s.push('\n');
        }
        s.push_str(&format!(
            "La mutare: {}. Mutări: literă+deplasare, ex. A+1\n",
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
        crate::brand_util::check(Ambuteiaj::with_seed, 400);
    }
    #[test]
    fn exit_wins() {
        let mut g = Ambuteiaj::with_seed(1);
        g.cars.retain(|c| c.id == b'A' || c.id == b'B');
        g.step("A+4");
        assert_eq!(g.result(), Some(Outcome::Win(Player::One)));
    }
}
