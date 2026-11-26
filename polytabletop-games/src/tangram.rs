//! Tangram (China), duel de plasare pentru 2 jucători — variantă simplificată pe grilă.
//!
//! Nu sunt siluete și unghiuri de 45°: cele șapte piese devin poliomino cu arii apropiate, ca să se poată juca pe
//! o tablă 6×6 de pătrățele. Fiecare jucător are propriul set de 7 piese: 2 triunghiuri mici (1 pătrățel), triunghi mediu
//! (domino), pătrat (2×2), paralelogram (S de 4 pătrățele), 2 triunghiuri mari (L de 3 pătrățele).
//! Jucătorii pun alternativ câte o piesă (orice rotire/oglindire) pe tablă comună, fără suprapuneri.
//! Cine nu mai poate pune nicio piesă pierde (joc normal).
//!
//! Mutări: `<piesă><orientare>@<colț>`, ex. `q1@c3`, `g2@a1` (colțul din stânga-jos al piesei, orientările sunt numerotate de la 1).

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: i32 = 6;
const IDS: [char; 7] = ['a', 'b', 'd', 'q', 'p', 'g', 'h'];

type Shape = Vec<(i32, i32)>;

fn base(id: usize) -> Shape {
    match id {
        0 | 1 => vec![(0, 0)],
        2 => vec![(0, 0), (1, 0)],
        3 => vec![(0, 0), (1, 0), (0, 1), (1, 1)],
        4 => vec![(0, 0), (1, 0), (1, 1), (2, 1)],
        _ => vec![(0, 0), (1, 0), (0, 1)],
    }
}

fn orientations(id: usize) -> Vec<Shape> {
    let mut out: Vec<Shape> = Vec::new();
    let b = base(id);
    for t in 0..8 {
        let mut s: Shape = b
            .iter()
            .map(|&(x, y)| {
                let (mut x, mut y) = (x, y);
                if t & 4 != 0 {
                    x = -x;
                }
                for _ in 0..(t & 3) {
                    let (nx, ny) = (-y, x);
                    x = nx;
                    y = ny;
                }
                (x, y)
            })
            .collect();
        let (mx, my) = (
            s.iter().map(|c| c.0).min().unwrap(),
            s.iter().map(|c| c.1).min().unwrap(),
        );
        for c in s.iter_mut() {
            c.0 -= mx;
            c.1 -= my;
        }
        s.sort();
        if !out.contains(&s) {
            out.push(s);
        }
    }
    out
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Move {
    piece: u8,
    orient: u8,
    x: u8,
    y: u8,
}

#[derive(Clone)]
pub struct Tangram {
    grid: [u8; 36],
    left: [[bool; 7]; 2],
    side: Player,
    orients: Vec<Vec<Shape>>,
}

impl Default for Tangram {
    fn default() -> Self {
        Self::new()
    }
}

impl Tangram {
    pub fn new() -> Self {
        Tangram {
            grid: [0; 36],
            left: [[true; 7]; 2],
            side: Player::One,
            orients: (0..7).map(orientations).collect(),
        }
    }
    fn moves_for(&self, p: usize) -> Vec<Move> {
        let mut v = Vec::new();
        for id in 0..7 {
            if !self.left[p][id] {
                continue;
            }
            // piesele identice (a,b) nu se dublează: se folosește doar prima disponibilă
            if id == 1 && self.left[p][0] {
                continue;
            }
            for (o, sh) in self.orients[id].iter().enumerate() {
                let (w, h) = (
                    sh.iter().map(|c| c.0).max().unwrap() + 1,
                    sh.iter().map(|c| c.1).max().unwrap() + 1,
                );
                for y in 0..=(N - h) {
                    for x in 0..=(N - w) {
                        if sh
                            .iter()
                            .all(|&(dx, dy)| self.grid[((y + dy) * N + x + dx) as usize] == 0)
                        {
                            v.push(Move {
                                piece: id as u8,
                                orient: o as u8,
                                x: x as u8,
                                y: y as u8,
                            });
                        }
                    }
                }
            }
        }
        v
    }
}

impl Game for Tangram {
    type Move = Move;
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<Move> {
        self.moves_for(self.side.idx())
    }
    fn apply(&mut self, m: &Move) {
        let p = self.side.idx();
        // identice: dacă s-a cerut `a` dar e deja folosită, nu se ajunge aici; `b` doar dacă `a` e folosită
        let id = m.piece as usize;
        self.left[p][id] = false;
        for &(dx, dy) in &self.orients[id][m.orient as usize] {
            self.grid[((m.y as i32 + dy) * N + m.x as i32 + dx) as usize] = p as u8 + 1;
        }
        self.side = self.side.other();
    }
    fn outcome(&self) -> Option<Outcome> {
        if self.moves_for(self.side.idx()).is_empty() {
            Some(Outcome::Win(self.side.other()))
        } else {
            None
        }
    }
    fn evaluate(&self, p: Player) -> i32 {
        let a = self.moves_for(p.idx()).len() as i32;
        let b = self.moves_for(1 - p.idx()).len() as i32;
        (a - b).clamp(-500, 500)
    }
    fn render(&self) -> String {
        let mut s = String::from("Tangram (variantă pe grilă) — J1 = X, J2 = O\n");
        for y in (0..N).rev() {
            s.push_str(&format!("{} ", y + 1));
            for x in 0..N {
                s.push(match self.grid[(y * N + x) as usize] {
                    1 => 'X',
                    2 => 'O',
                    _ => '.',
                });
                s.push(' ');
            }
            s.push('\n');
        }
        s.push_str("  a b c d e f\n");
        for p in 0..2 {
            let r: Vec<String> = (0..7)
                .filter(|&i| self.left[p][i])
                .map(|i| IDS[i].to_string())
                .collect();
            s.push_str(&format!("Piese J{}: {}\n", p + 1, r.join(" ")));
        }
        match self.outcome() {
            Some(Outcome::Win(w)) => s.push_str(&format!(
                "J{} nu mai poate pune nimic. Câștigă J{}.\n",
                w.other().idx() + 1,
                w.idx() + 1
            )),
            _ => s.push_str(&format!("La mutare: J{}\n", self.side.idx() + 1)),
        }
        s
    }
    fn format_move(&self, m: &Move) -> String {
        format!(
            "{}{}@{}{}",
            IDS[m.piece as usize],
            m.orient + 1,
            (b'a' + m.x) as char,
            m.y + 1
        )
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

    #[test]
    fn orientation_counts() {
        assert_eq!(orientations(3).len(), 1);
        assert_eq!(orientations(2).len(), 2);
        assert_eq!(orientations(4).len(), 4);
        assert_eq!(orientations(5).len(), 4);
    }

    #[test]
    fn no_overlap_and_player_without_moves_loses() {
        let mut g = Tangram::new();
        let m = Move {
            piece: 3,
            orient: 0,
            x: 0,
            y: 0,
        };
        g.apply(&m);
        assert!(!g
            .legal_moves()
            .iter()
            .any(|x| x.piece == 3 && x.x == 0 && x.y == 0));
        g.grid = [1; 36];
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn move_texts_unique_and_games_finish() {
        let g = Tangram::new();
        let mut t: Vec<String> = g.legal_moves().iter().map(|m| g.format_move(m)).collect();
        let n = t.len();
        t.sort();
        t.dedup();
        assert_eq!(n, t.len());
        let mut g: Box<dyn polytabletop_core::GameObj> = Box::new(Tangram::new());
        let mut k = 0;
        while g.outcome().is_none() {
            let m = g.ai_move(k).unwrap();
            g.play(&m).unwrap();
            k += 1;
            assert!(k < 20);
        }
    }
}
