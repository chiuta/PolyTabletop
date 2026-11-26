//! Cuburi glisante (variantă proprie): tablă 5×5 de 25 de cuburi, fiecare cub neutru sau
//! marcat X/O. Pe rând iei un cub de pe marginea tablei (neutru sau al tău), îl marchezi cu
//! semnul tău și îl împingi înapoi pe tablă, de la capătul opus al rândului sau coloanei lui,
//! celelalte cuburi alunecând. Cine face cinci semne ale sale în linie câștigă (dacă o
//! mutare face linie pentru ambii, pierde cel care a mutat). Mutări: a3>e (ia a3, împinge
//! spre est), s/n/e/v = sud/nord/est/vest.
#![allow(clippy::all)]

use crate::b1_util::{hkey, sq};
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: usize = 5;

#[derive(Clone)]
pub struct Cuburi {
    b: [u8; N * N],
    side: Player,
    win: Option<Player>,
    ply: u32,
}

impl Default for Cuburi {
    fn default() -> Self {
        Self::new()
    }
}

fn five(b: &[u8; 25], who: u8) -> bool {
    let lines: [[usize; 5]; 12] = [
        [0, 1, 2, 3, 4],
        [5, 6, 7, 8, 9],
        [10, 11, 12, 13, 14],
        [15, 16, 17, 18, 19],
        [20, 21, 22, 23, 24],
        [0, 5, 10, 15, 20],
        [1, 6, 11, 16, 21],
        [2, 7, 12, 17, 22],
        [3, 8, 13, 18, 23],
        [4, 9, 14, 19, 24],
        [0, 6, 12, 18, 24],
        [4, 8, 12, 16, 20],
    ];
    lines.iter().any(|l| l.iter().all(|&i| b[i] == who))
}

impl Cuburi {
    pub fn new() -> Self {
        Cuburi {
            b: [0; 25],
            side: Player::One,
            win: None,
            ply: 0,
        }
    }
}

const DIRS: [(char, i32, i32); 4] = [('n', 0, 1), ('s', 0, -1), ('e', 1, 0), ('v', -1, 0)];

impl Game for Cuburi {
    type Move = (u8, u8);
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<(u8, u8)> {
        if self.win.is_some() || self.ply >= 120 {
            return vec![];
        }
        let me = self.side.idx() as u8 + 1;
        let mut v = vec![];
        for i in 0..N * N {
            let (x, y) = (i % N, i / N);
            if !(x == 0 || y == 0 || x == N - 1 || y == N - 1)
                || (self.b[i] != 0 && self.b[i] != me)
            {
                continue;
            }
            for (d, &(_, dx, dy)) in DIRS.iter().enumerate() {
                // cubul se împinge de la capătul opus: nu înapoi în afara aceleiași margini
                let ok = (dx == 1 && x != N - 1)
                    || (dx == -1 && x != 0)
                    || (dy == 1 && y != N - 1)
                    || (dy == -1 && y != 0);
                if ok {
                    v.push((i as u8, d as u8));
                }
            }
        }
        v
    }
    fn apply(&mut self, m: &(u8, u8)) {
        let me = self.side.idx() as u8 + 1;
        let i = m.0 as usize;
        let (_, dx, dy) = DIRS[m.1 as usize];
        let (x, y) = ((i % N) as i32, (i / N) as i32);
        // cubul pleacă din (x,y) și ajunge la capătul rândului/coloanei în direcția dx,dy
        let (ex, ey) = if dx != 0 {
            (if dx > 0 { N as i32 - 1 } else { 0 }, y)
        } else {
            (x, if dy > 0 { N as i32 - 1 } else { 0 })
        };
        let (mut cx, mut cy) = (x, y);
        while (cx, cy) != (ex, ey) {
            let (nx, ny) = (cx + dx, cy + dy);
            self.b[cy as usize * N + cx as usize] = self.b[ny as usize * N + nx as usize];
            cx = nx;
            cy = ny;
        }
        self.b[ey as usize * N + ex as usize] = me;
        self.ply += 1;
        let (a, o) = (five(&self.b, me), five(&self.b, 3 - me));
        if o {
            self.win = Some(self.side.other());
        } else if a {
            self.win = Some(self.side);
        }
        self.side = self.side.other();
    }
    fn outcome(&self) -> Option<Outcome> {
        match self.win {
            Some(w) => Some(Outcome::Win(w)),
            None if self.ply >= 120 => Some(Outcome::Draw),
            None => None,
        }
    }
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx() as u8 + 1;
        let mut s = 0;
        let lines: Vec<Vec<usize>> = (0..5)
            .map(|r| (0..5).map(|c| r * 5 + c).collect())
            .chain((0..5).map(|c| (0..5).map(|r| r * 5 + c).collect()))
            .chain([
                (0..5).map(|k| k * 6).collect(),
                (0..5).map(|k| 4 + k * 4).collect(),
            ])
            .collect();
        for l in lines {
            let a = l.iter().filter(|&&i| self.b[i] == me).count() as i32;
            let o = l.iter().filter(|&&i| self.b[i] == 3 - me).count() as i32;
            if o == 0 {
                s += a * a * a;
            }
            if a == 0 {
                s -= o * o * o;
            }
        }
        s
    }
    fn render(&self) -> String {
        crate::b1_util::grid(N, N, |x, y| match self.b[y * N + x] {
            1 => 'X',
            2 => 'O',
            _ => '.',
        })
    }
    fn format_move(&self, m: &(u8, u8)) -> String {
        let i = m.0 as usize;
        format!("{}>{}", sq(i % N, i / N), DIRS[m.1 as usize].0)
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 300,
            max_depth: 6,
        }
    }
    fn hash_key(&self) -> Option<u64> {
        Some(hkey(
            13,
            self.side,
            self.b.iter().enumerate().map(|(i, &v)| (i, v as u64)),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::b1_util::testing::check;
    #[test]
    fn push_rules() {
        let mut g = Cuburi::new();
        // colț: două direcții valide (opusă marginilor)
        let m = g.legal_moves();
        assert!(m.contains(&(0, 2)) && m.contains(&(0, 0)));
        assert!(!m.contains(&(0, 1)) && !m.contains(&(0, 3)));
        g.b[0] = 1;
        g.b[1] = 2;
        g.side = Player::One;
        g.apply(&(0, 2)); // a1>e: a1 ia, împinge spre est
        assert_eq!(g.b[4], 1);
        assert_eq!(g.b[0], 2);
    }
    #[test]
    fn random_and_ai() {
        check(|_| Cuburi::new(), 200);
    }
}
