#![allow(clippy::doc_lazy_continuation)]
//! Halma (SUA / Marea Britanie, 1883), varianta pentru doi jucători pe tabla 16×16.
//!
//! * Fiecare jucător are 19 piese într-un colț (rânduri de 5, 5, 4, 3 și 2 piese): J1 (X) în colțul
//!   din stânga-jos, J2 (O) în colțul din dreapta-sus. Țelul: mută toate cele 19 piese în tabăra
//!   adversă.
//! * Mutarea: un pas pe un pătrat liber vecin (și pe diagonală) **sau** un lanț de sărituri: sari peste
//!   o piesă vecină (a ta sau a adversarului) pe pătratul liber de dincolo și poți continua să sari din
//!   acolo cu aceeași piesă. Nu se captează nimic. Mutarea se scrie doar cu punctul de plecare și cel
//!   de sosire (`b2d4`); lanțul e dedus (același rezultat indiferent de drum).
//! * Câștigă cine aduce primul toate cele 19 piese în tabăra adversă. Pentru a opri blocajele (și jocurile
//!   aleatoare infinite): după 150 de mutări pe jucător câștigă cine are mai multe piese în tabăra adversă
//!   (egal = remiză) — regulă proprie.
//! Mutări: `b2d4` (de la, la).

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const S: usize = 16;
const N: usize = S * S;
const WIDTHS: [usize; 5] = [5, 5, 4, 3, 2];
const MAX_PLIES: u16 = 300;
const DIRS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn name(i: usize) -> String {
    polytabletop_core::square_name(i % S, i / S)
}

fn camp1(i: usize) -> bool {
    let (r, c) = (i / S, i % S);
    r < 5 && c < WIDTHS[r]
}

fn camp2(i: usize) -> bool {
    let (r, c) = (i / S, i % S);
    r >= S - 5 && (S - 1 - c) < WIDTHS[S - 1 - r]
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Mv {
    pub from: u16,
    pub to: u16,
}

#[derive(Clone)]
pub struct Halma {
    b: [u8; N],
    side: Player,
    plies: u16,
}

impl Default for Halma {
    fn default() -> Self {
        Self::new()
    }
}

impl Halma {
    pub fn new() -> Self {
        let mut b = [0u8; N];
        for (i, cell) in b.iter_mut().enumerate() {
            if camp1(i) {
                *cell = 1;
            } else if camp2(i) {
                *cell = 2;
            }
        }
        Halma {
            b,
            side: Player::One,
            plies: 0,
        }
    }

    fn in_target(&self, p: usize, i: usize) -> bool {
        if p == 0 {
            camp2(i)
        } else {
            camp1(i)
        }
    }

    fn at_target(&self, p: usize) -> usize {
        (0..N)
            .filter(|&i| self.b[i] == p as u8 + 1 && self.in_target(p, i))
            .count()
    }

    fn destinations(&self, from: usize) -> Vec<usize> {
        let mut out = Vec::new();
        let (r, c) = ((from / S) as i32, (from % S) as i32);
        for (dr, dc) in DIRS {
            let (nr, nc) = (r + dr, c + dc);
            if nr >= 0 && nc >= 0 && nr < S as i32 && nc < S as i32 {
                let n = nr as usize * S + nc as usize;
                if self.b[n] == 0 {
                    out.push(n);
                }
            }
        }
        // sărituri: BFS
        let mut seen = vec![false; N];
        seen[from] = true;
        let mut stack = vec![from];
        while let Some(x) = stack.pop() {
            let (r, c) = ((x / S) as i32, (x % S) as i32);
            for (dr, dc) in DIRS {
                let (mr, mc, tr, tc) = (r + dr, c + dc, r + 2 * dr, c + 2 * dc);
                if tr < 0 || tc < 0 || tr >= S as i32 || tc >= S as i32 {
                    continue;
                }
                let (m, t) = (mr as usize * S + mc as usize, tr as usize * S + tc as usize);
                if self.b[m] != 0 && self.b[t] == 0 && !seen[t] {
                    seen[t] = true;
                    stack.push(t);
                    if !out.contains(&t) {
                        out.push(t);
                    }
                }
            }
        }
        out.sort_unstable();
        out
    }

    fn adv(&self, p: usize) -> i32 {
        let mut v = 0;
        for i in 0..N {
            if self.b[i] == p as u8 + 1 {
                let s = (i / S + i % S) as i32;
                v += if p == 0 { s } else { 30 - s };
            }
        }
        v
    }

    fn finished(&self, p: usize) -> bool {
        self.at_target(p) == 19
    }
}

impl Game for Halma {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        let me = self.side.idx() as u8 + 1;
        let mut v = Vec::new();
        for from in 0..N {
            if self.b[from] == me {
                for to in self.destinations(from) {
                    v.push(Mv {
                        from: from as u16,
                        to: to as u16,
                    });
                }
            }
        }
        // mutările care înaintează întâi (ordonare pentru căutare)
        let p = self.side.idx();
        let key = |m: &Mv| -> i32 {
            let f = (m.from as usize / S + m.from as usize % S) as i32;
            let t = (m.to as usize / S + m.to as usize % S) as i32;
            if p == 0 {
                f - t
            } else {
                t - f
            }
        };
        v.sort_by_key(key);
        v
    }

    fn apply(&mut self, m: &Mv) {
        let me = self.b[m.from as usize];
        self.b[m.from as usize] = 0;
        self.b[m.to as usize] = me;
        self.side = self.side.other();
        self.plies += 1;
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.finished(0) {
            return Some(Outcome::Win(Player::One));
        }
        if self.finished(1) {
            return Some(Outcome::Win(Player::Two));
        }
        if self.plies >= MAX_PLIES {
            return Some(match self.at_target(0).cmp(&self.at_target(1)) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                std::cmp::Ordering::Equal => Outcome::Draw,
            });
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let (a, b) = (p.idx(), 1 - p.idx());
        (self.adv(a) - self.adv(b)) * 10
            + (self.at_target(a) as i32 - self.at_target(b) as i32) * 15
    }

    fn render(&self) -> String {
        let mut s = String::from("    a b c d e f g h i j k l m n o p\n");
        for r in (0..S).rev() {
            s.push_str(&format!("{:>2}  ", r + 1));
            for c in 0..S {
                s.push(match self.b[r * S + c] {
                    1 => 'X',
                    2 => 'O',
                    _ => '.',
                });
                s.push(' ');
            }
            s.push('\n');
        }
        s.push_str(&format!(
            "În tabăra țintă: X {}/19, O {}/19. La mutare: {} (mutarea {}/{}).\n",
            self.at_target(0),
            self.at_target(1),
            if self.side == Player::One { "X" } else { "O" },
            self.plies / 2 + 1,
            MAX_PLIES / 2
        ));
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        format!("{}{}", name(m.from as usize), name(m.to as usize))
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 400,
            max_depth: 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    #[test]
    fn camps_have_19_cells_and_start_position_is_symmetric() {
        let g = Halma::new();
        assert_eq!(g.b.iter().filter(|&&x| x == 1).count(), 19);
        assert_eq!(g.b.iter().filter(|&&x| x == 2).count(), 19);
        assert!(!g.legal_moves().is_empty());
    }

    #[test]
    fn chained_hops_are_reachable() {
        let mut g = Halma::new();
        g.b = [0; N];
        g.b[0] = 1; // a1
        g.b[S + 1] = 2; // b2
        g.b[3 * S + 3] = 2; // d4
        g.b[S * 5 + 5] = 2; // f6
        let d = g.destinations(0);
        assert!(d.contains(&(2 * S + 2))); // c3
        assert!(d.contains(&(4 * S + 4))); // e5
        assert!(d.contains(&(6 * S + 6))); // g7
    }

    #[test]
    fn random_games_end_and_ai_is_fast() {
        for seed in 1..=3u64 {
            let mut g = Halma::new();
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
                assert!(n < 400);
            }
        }
        let g = Halma::new();
        let m = polytabletop_core::GameObj::ai_move(&g, 1).unwrap();
        assert!(g.legal_moves().iter().any(|x| g.format_move(x) == m));
    }
}
