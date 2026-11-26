//! Izvoare (variantă proprie de stive conectate pe tablă 7×7): trei „izvoare” neutre ocupă
//! celulele b6, d4 și f2. Pe rând, fiecare jucător pune câte o piesă (23 fiecare) pe celule
//! libere. Apoi mutarea: o stivă condusă de tine (piesa de sus e a ta) se mută pe linie
//! dreaptă (rând, coloană sau diagonală) exact atâtea celule câte piese are, trecând peste
//! orice, și trebuie să aterizeze pe o altă stivă (a oricui); stivele se contopesc și piesa
//! de sus a celei mutate conduce. După fiecare mutare, stivele care nu sunt legate (prin vecini
//! pe 8 direcții) de o stivă cu izvor sunt scoase de pe tablă. Când nimeni nu mai poate muta,
//! câștigă cine conduce stive cu mai multe piese proprii (izvoarele nu contează); egal = remiză.
//! Mutări: în faza de pus `c3`; apoi `c3-e5`; `pas` când nu ai nicio mutare.
#![allow(clippy::all)]

use crate::b1_util::sq;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: i32 = 7;

#[derive(Clone)]
pub struct Izvoare {
    s: Vec<Vec<u8>>, // 1 / 2 jucători, 3 izvor
    left: [u8; 2],
    side: Player,
    passes: u8,
    ply: u32,
}

impl Default for Izvoare {
    fn default() -> Self {
        Self::new()
    }
}

impl Izvoare {
    pub fn new() -> Self {
        let mut s = vec![vec![]; 49];
        for (x, y) in [(1, 5), (3, 3), (5, 1)] {
            s[y * 7 + x] = vec![3];
        }
        Izvoare {
            s,
            left: [23, 23],
            side: Player::One,
            passes: 0,
            ply: 0,
        }
    }
    fn placing(&self) -> bool {
        self.left[0] + self.left[1] > 0
    }
    fn gen(&self, p: usize) -> Vec<(u8, u8)> {
        let me = p as u8 + 1;
        let mut v = vec![];
        for i in 0..49 {
            let st = &self.s[i];
            if st.is_empty() || *st.last().unwrap() != me {
                continue;
            }
            let h = st.len() as i32;
            let (x, y) = ((i % 7) as i32, (i / 7) as i32);
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let (nx, ny) = (x + dx * h, y + dy * h);
                    if nx < 0 || ny < 0 || nx >= N || ny >= N {
                        continue;
                    }
                    let t = (ny * N + nx) as usize;
                    if !self.s[t].is_empty() {
                        v.push((i as u8, t as u8));
                    }
                }
            }
        }
        v
    }
    fn prune(&mut self) {
        let mut ok = [false; 49];
        let mut st: Vec<usize> = (0..49).filter(|&i| self.s[i].contains(&3)).collect();
        for &i in &st {
            ok[i] = true;
        }
        while let Some(i) = st.pop() {
            let (x, y) = ((i % 7) as i32, (i / 7) as i32);
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let (nx, ny) = (x + dx, y + dy);
                    if (dx, dy) == (0, 0) || nx < 0 || ny < 0 || nx >= N || ny >= N {
                        continue;
                    }
                    let j = (ny * N + nx) as usize;
                    if !ok[j] && !self.s[j].is_empty() {
                        ok[j] = true;
                        st.push(j);
                    }
                }
            }
        }
        for i in 0..49 {
            if !ok[i] {
                self.s[i].clear();
            }
        }
    }
    fn score(&self, p: usize) -> i32 {
        let me = p as u8 + 1;
        self.s
            .iter()
            .filter(|st| st.last() == Some(&me))
            .map(|st| st.iter().filter(|&&c| c == me).count() as i32)
            .sum()
    }
}

impl Game for Izvoare {
    type Move = (u8, u8);
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<(u8, u8)> {
        if self.passes >= 2 {
            return vec![];
        }
        if self.placing() {
            return (0..49)
                .filter(|&i| self.s[i].is_empty())
                .map(|i| (i as u8, 255))
                .collect();
        }
        let v = self.gen(self.side.idx());
        if v.is_empty() {
            vec![(255, 255)]
        } else {
            v
        }
    }
    fn apply(&mut self, m: &(u8, u8)) {
        let me = self.side.idx() as u8 + 1;
        if m.1 == 255 && m.0 != 255 {
            self.s[m.0 as usize].push(me);
            self.left[self.side.idx()] -= 1;
        } else if m.0 == 255 {
            self.passes += 1;
        } else {
            self.passes = 0;
            let mut moving = std::mem::take(&mut self.s[m.0 as usize]);
            self.s[m.1 as usize].append(&mut moving);
            self.prune();
        }
        self.ply += 1;
        self.side = self.side.other();
    }
    fn outcome(&self) -> Option<Outcome> {
        if self.passes < 2 {
            return None;
        }
        Some(match self.score(0).cmp(&self.score(1)) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            _ => Outcome::Draw,
        })
    }
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        (self.score(me) - self.score(1 - me)) * 10
            + if self.placing() {
                0
            } else {
                (self.gen(me).len() as i32 - self.gen(1 - me).len() as i32) * 2
            }
    }
    fn render(&self) -> String {
        let mut s = String::new();
        for y in (0..7).rev() {
            s.push_str(&format!("{:>2} ", y + 1));
            for x in 0..7 {
                let st = &self.s[y * 7 + x];
                if st.is_empty() {
                    s.push_str(" .  ");
                } else {
                    let c = match st.last().unwrap() {
                        1 => 'X',
                        2 => 'O',
                        _ => '*',
                    };
                    s.push_str(&format!("{}{:<2} ", c, st.len()));
                }
            }
            s.push('\n');
        }
        s.push_str("    a   b   c   d   e   f   g\n");
        s.push_str("* = izvor la vârf; număr = înălțime. Neconectate de un izvor dispar.\n");
        s.push_str(&format!(
            "De pus: J1 {}, J2 {}; scor piese conduse: J1 {}, J2 {}\n",
            self.left[0],
            self.left[1],
            self.score(0),
            self.score(1)
        ));
        s
    }
    fn format_move(&self, m: &(u8, u8)) -> String {
        if m.0 == 255 {
            "pas".into()
        } else if m.1 == 255 {
            sq(m.0 as usize % 7, m.0 as usize / 7)
        } else {
            format!(
                "{}-{}",
                sq(m.0 as usize % 7, m.0 as usize / 7),
                sq(m.1 as usize % 7, m.1 as usize / 7)
            )
        }
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 400,
            max_depth: 4,
        }
    }
    fn hash_key(&self) -> Option<u64> {
        let mut h = polytabletop_core::hash_mix(
            23 << 40 | (self.side.idx() as u64) << 32 | self.passes as u64,
        );
        for (i, st) in self.s.iter().enumerate() {
            for (k, &c) in st.iter().enumerate() {
                h ^= polytabletop_core::hash_mix((c as u64) << 24 | (k as u64) << 8 | i as u64);
            }
        }
        Some(h ^ polytabletop_core::hash_mix(7000 + self.left[0] as u64 * 30 + self.left[1] as u64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::b1_util::testing::check;
    #[test]
    fn placing_then_moving() {
        let g = Izvoare::new();
        assert_eq!(g.legal_moves().len(), 46);
        assert!(g.placing());
        let mut h = Izvoare::new();
        h.left = [0, 0];
        h.s = vec![vec![]; 49];
        h.s[24] = vec![3, 1]; // d4: izvor + piesă J1 deasupra
        h.s[25] = vec![2]; // e4
        h.s[26] = vec![1]; // f4
        h.s[9] = vec![2]; // b2: nelegat (b2 nu e vecin cu nimic)
        h.prune();
        assert!(h.s[9].is_empty());
        // stiva d4 are înălțime 2: sare 2 celule spre est -> f4
        let m = h.legal_moves();
        assert!(m.contains(&(24, 26)));
        h.apply(&(24, 26));
        assert_eq!(h.s[26].len(), 3);
        assert_eq!(h.s[26].last(), Some(&1));
    }
    #[test]
    fn random_and_ai() {
        check(|_| Izvoare::new(), 400);
    }
}
