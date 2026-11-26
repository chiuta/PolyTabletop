//! Cub 2×2: duel (variantă proprie, joc de doi jucători cu un cub-puzzle 2×2×2 amestecat din 7
//! mutări la întâmplare): pe rând, fiecare jucător face o rotire a unei fețe (U, R sau F, cu
//! 90° în sens orar, 180° sau 90° antiorar; colțul din spate-stânga-jos rămâne fix). Câștigă
//! cine face cubul rezolvat (fiecare față o singură culoare). Remiză după 40 de rotiri.
//! Mutări: U, U2, U' (apostrof pentru antiorar), R, R2, R', F, F2, F'.
#![allow(clippy::all)]

use polytabletop_core::{AiStyle, Game, Outcome, Player};

// un autocolant: poziția cubului (±1), normala și culoarea
type St = ([i8; 3], [i8; 3], u8);

const FACE: [(usize, &str); 3] = [(1, "U"), (0, "R"), (2, "F")];

fn rot(v: [i8; 3], ax: usize) -> [i8; 3] {
    let (a, b) = ((ax + 1) % 3, (ax + 2) % 3);
    let mut o = v;
    o[a] = -v[b];
    o[b] = v[a];
    o
}

#[derive(Clone)]
pub struct Cub2 {
    s: Vec<St>,
    side: Player,
    ply: u32,
    done: bool,
}

impl Default for Cub2 {
    fn default() -> Self {
        Self::new()
    }
}

impl Cub2 {
    pub fn solved() -> Self {
        let mut s = vec![];
        for x in [-1i8, 1] {
            for y in [-1i8, 1] {
                for z in [-1i8, 1] {
                    for ax in 0..3 {
                        let mut n = [0i8; 3];
                        let pos = [x, y, z];
                        n[ax] = pos[ax];
                        let col = ax as u8 * 2 + if pos[ax] > 0 { 1 } else { 0 };
                        s.push(([x, y, z], n, col));
                    }
                }
            }
        }
        Cub2 {
            s,
            side: Player::One,
            ply: 0,
            done: false,
        }
    }
    pub fn new() -> Self {
        Self::with_seed(polytabletop_core::rng::Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = polytabletop_core::rng::Rng::new(seed);
        loop {
            let mut c = Self::solved();
            let mut last = 9;
            for _ in 0..7 {
                let mut f = rng.below(3);
                while f == last {
                    f = rng.below(3);
                }
                last = f;
                c.turn(f, 1 + rng.below(3));
            }
            if !c.is_solved() {
                return c;
            }
        }
    }
    fn turn(&mut self, face: usize, times: usize) {
        let (ax, _) = FACE[face];
        for st in self.s.iter_mut() {
            if st.0[ax] == 1 {
                for _ in 0..times {
                    st.0 = rot(st.0, ax);
                    st.1 = rot(st.1, ax);
                }
            }
        }
    }
    fn is_solved(&self) -> bool {
        for f in 0..6 {
            let (ax, sg) = (f / 2, if f % 2 == 1 { 1 } else { -1 });
            let cols: Vec<u8> = self
                .s
                .iter()
                .filter(|st| st.1[ax] == sg)
                .map(|st| st.2)
                .collect();
            if cols.iter().any(|&c| c != cols[0]) {
                return false;
            }
        }
        true
    }
    fn faces_ok(&self) -> i32 {
        let mut n = 0;
        for f in 0..6 {
            let (ax, sg) = (f / 2, if f % 2 == 1 { 1 } else { -1 });
            let cols: Vec<u8> = self
                .s
                .iter()
                .filter(|st| st.1[ax] == sg)
                .map(|st| st.2)
                .collect();
            let mut best = 0;
            for c in 0..6 {
                best = best.max(cols.iter().filter(|&&x| x == c).count());
            }
            n += best as i32;
        }
        n
    }
}

impl Game for Cub2 {
    type Move = u8;
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<u8> {
        if self.outcome().is_some() {
            return vec![];
        }
        (0..9).collect()
    }
    fn apply(&mut self, m: &u8) {
        let (f, k) = (*m as usize / 3, *m as usize % 3 + 1);
        self.turn(f, k);
        self.ply += 1;
        if self.is_solved() {
            self.done = true;
        } else {
            self.side = self.side.other();
        }
    }
    fn outcome(&self) -> Option<Outcome> {
        if self.done {
            Some(Outcome::Win(self.side))
        } else if self.ply >= 40 {
            Some(Outcome::Draw)
        } else {
            None
        }
    }
    fn evaluate(&self, _p: Player) -> i32 {
        0
    }
    fn render(&self) -> String {
        let letters = ['W', 'Y', 'R', 'O', 'B', 'G']; // -x +x ... culori
        let face = |f: usize| -> String {
            let (ax, sg) = (f / 2, if f % 2 == 1 { 1i8 } else { -1 });
            let (a, b) = ((ax + 1) % 3, (ax + 2) % 3);
            let mut g = [['?'; 2]; 2];
            for st in self.s.iter().filter(|st| st.1[ax] == sg) {
                let (i, j) = (
                    if st.0[a] > 0 { 1 } else { 0 },
                    if st.0[b] > 0 { 1 } else { 0 },
                );
                g[1 - j][i] = letters[st.2 as usize];
            }
            format!("{}{}/{}{}", g[0][0], g[0][1], g[1][0], g[1][1])
        };
        let names = ["-x", "+x(R)", "-y", "+y(U)", "-z", "+z(F)"];
        let mut s = String::new();
        for f in 0..6 {
            s.push_str(&format!("{}: {}   ", names[f], face(f)));
        }
        s.push_str(&format!(
            "\nFețe aproape rezolvate: {}/24. Cine rezolvă cubul câștigă.\n",
            self.faces_ok()
        ));
        s
    }
    fn format_move(&self, m: &u8) -> String {
        let (f, k) = (*m as usize / 3, *m as usize % 3);
        format!("{}{}", FACE[f].1, ["", "2", "'"][k])
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 300,
            max_depth: 6,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::b1_util::testing::check;
    #[test]
    fn turns_are_reversible() {
        let mut c = Cub2::solved();
        assert!(c.is_solved());
        c.turn(0, 1);
        assert!(!c.is_solved());
        c.turn(0, 3);
        assert!(c.is_solved());
        c.turn(1, 2);
        c.turn(1, 2);
        assert!(c.is_solved());
        // R U R' U' nu e identitate dar 6x da
        let mut d = Cub2::solved();
        for _ in 0..6 {
            d.turn(1, 1);
            d.turn(0, 1);
            d.turn(1, 3);
            d.turn(0, 3);
        }
        assert!(d.is_solved());
    }
    #[test]
    fn scramble_not_solved_and_solve_wins() {
        let g = Cub2::with_seed(5);
        assert!(!g.is_solved());
        let mut h = Cub2::solved();
        h.turn(0, 1);
        h.apply(&(0 * 3 + 2)); // R' readuce cubul
        assert_eq!(h.outcome(), Some(Outcome::Win(Player::One)));
    }
    #[test]
    fn random_and_ai() {
        check(Cub2::with_seed, 60);
    }
}
