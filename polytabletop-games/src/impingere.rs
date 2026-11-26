//! Împingere pe pătrate (variantă proprie): tablă 7×7, fiecare jucător cu 10 bile pe două
//! rânduri. O tură: muți un grup de 1–3 bile proprii aliniate (rând, coloană sau diagonală)
//! cu un pas. „Pe lungime”, grupul poate împinge 1 sau 2 bile adverse aliniate dacă are
//! strict mai multe bile (2 contra 1, 3 contra 1 sau 2) și locul de dincolo e liber sau în
//! afara tablei; bilele împinse în afară sunt capturate. „Lateral” (alt sens decât linia
//! grupului), toate celulele de destinație trebuie să fie libere. Câștigă cine scoate 5 bile
//! adverse; remiză după 100 de semi-mutări fără captură.
//! Mutări: `a1-c3>1` = grupul de la a1 la c3 se mută spre direcția 1 (n, ne, e, se, s, sv, v, nv).
#![allow(clippy::all)]

use crate::b1_util::{grid, hkey, sq};
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: i32 = 7;
const D: [(i32, i32); 8] = [
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];
const DN: [&str; 8] = ["n", "ne", "e", "se", "s", "sv", "v", "nv"];

#[derive(Clone)]
pub struct Impingere {
    b: [u8; 49],
    side: Player,
    out: [u8; 2], // bile pierdute de fiecare
    quiet: u32,
}

impl Default for Impingere {
    fn default() -> Self {
        Self::new()
    }
}

fn inb(x: i32, y: i32) -> bool {
    x >= 0 && y >= 0 && x < N && y < N
}

impl Impingere {
    pub fn new() -> Self {
        let mut b = [0u8; 49];
        for x in 0..7 {
            b[x] = 1;
            b[42 + x] = 2;
        }
        for x in 1..6 {
            b[7 + x] = 1;
            b[35 + x] = 2;
        }
        Impingere {
            b,
            side: Player::One,
            out: [0; 2],
            quiet: 0,
        }
    }
    fn get(&self, x: i32, y: i32) -> u8 {
        self.b[(y * N + x) as usize]
    }
}

impl Game for Impingere {
    // (x0,y0,len,dir_grup,dir_mutare) ; grup = len bile de la (x0,y0) în direcția dir_grup (0..3 pentru dir_grup canonic)
    type Move = (u8, u8, u8, u8, u8);
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<(u8, u8, u8, u8, u8)> {
        if self.out[0] >= 5 || self.out[1] >= 5 || self.quiet >= 100 {
            return vec![];
        }
        let me = self.side.idx() as u8 + 1;
        let mut v = vec![];
        for y in 0..N {
            for x in 0..N {
                if self.get(x, y) != me {
                    continue;
                }
                for len in 1..=3i32 {
                    // direcții canonice de grup: n, ne, e, se (indici 0..3); len 1: doar dir 0
                    let gdirs: &[usize] = if len == 1 { &[0] } else { &[0, 1, 2, 3] };
                    for &gd in gdirs {
                        let (gx, gy) = D[gd];
                        let cells: Vec<(i32, i32)> =
                            (0..len).map(|k| (x + gx * k, y + gy * k)).collect();
                        if !cells
                            .iter()
                            .all(|&(a, c)| inb(a, c) && self.get(a, c) == me)
                        {
                            continue;
                        }
                        for md in 0..8 {
                            if self.try_move(&cells, gd, md, me, len).is_some() {
                                v.push((x as u8, y as u8, len as u8, gd as u8, md as u8));
                            }
                        }
                    }
                }
            }
        }
        v
    }
    fn apply(&mut self, m: &(u8, u8, u8, u8, u8)) {
        let me = self.side.idx() as u8 + 1;
        let (x, y, len, gd, md) = (
            m.0 as i32,
            m.1 as i32,
            m.2 as i32,
            m.3 as usize,
            m.4 as usize,
        );
        let (gx, gy) = D[gd];
        let cells: Vec<(i32, i32)> = (0..len).map(|k| (x + gx * k, y + gy * k)).collect();
        let res = self.try_move(&cells, gd, md, me, len).unwrap();
        let (dx, dy) = D[md];
        let pushed = res;
        // eliminăm grupul, apoi punem la loc deplasat; bilele împinse se mută/ies
        let inline = len > 1 && (D[md] == D[gd] || D[md] == (-D[gd].0, -D[gd].1));
        if inline {
            // capul în direcția mutării
            let mut front: Vec<(i32, i32)> = cells.clone();
            if D[md] != D[gd] {
                front.reverse();
            }
            let head = front[front.len() - 1];
            // bile adverse de împins
            let mut chain = vec![];
            let (mut cx, mut cy) = (head.0 + dx, head.1 + dy);
            while inb(cx, cy) && self.get(cx, cy) == 3 - me {
                chain.push((cx, cy));
                cx += dx;
                cy += dy;
            }
            for &(a, c) in chain.iter().rev() {
                self.b[(c * N + a) as usize] = 0;
                let (na, nc) = (a + dx, c + dy);
                if inb(na, nc) {
                    self.b[(nc * N + na) as usize] = 3 - me;
                } else {
                    self.out[2 - me as usize] += 1;
                }
            }
        }
        for &(a, c) in &cells {
            self.b[(c * N + a) as usize] = 0;
        }
        for &(a, c) in &cells {
            self.b[((c + dy) * N + a + dx) as usize] = me;
        }
        if pushed > 0 {
            self.quiet = 0;
        } else {
            self.quiet += 1;
        }
        self.side = self.side.other();
    }
    fn outcome(&self) -> Option<Outcome> {
        if self.out[1] >= 5 {
            Some(Outcome::Win(Player::One))
        } else if self.out[0] >= 5 {
            Some(Outcome::Win(Player::Two))
        } else if self.quiet >= 100 {
            Some(Outcome::Draw)
        } else {
            None
        }
    }
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx() as u8 + 1;
        let mut s = (self.out[2 - me as usize] as i32 - self.out[me as usize - 1] as i32) * 1000;
        for y in 0..N {
            for x in 0..N {
                let v = self.get(x, y);
                if v == 0 {
                    continue;
                }
                let d = (x - 3).abs().max((y - 3).abs());
                let w = 30 - d * 6;
                s += if v == me { w } else { -w };
            }
        }
        s
    }
    fn render(&self) -> String {
        let mut s = grid(7, 7, |x, y| match self.b[y * 7 + x] {
            1 => 'X',
            2 => 'O',
            _ => '.',
        });
        s.push_str(&format!(
            "Bile scoase: J1 a pierdut {}, J2 a pierdut {}.\n",
            self.out[0], self.out[1]
        ));
        s
    }
    fn format_move(&self, m: &(u8, u8, u8, u8, u8)) -> String {
        let (x, y, len, gd) = (m.0 as i32, m.1 as i32, m.2 as i32, m.3 as usize);
        let (ex, ey) = (x + D[gd].0 * (len - 1), y + D[gd].1 * (len - 1));
        if len == 1 {
            format!("{}>{}", sq(x as usize, y as usize), DN[m.4 as usize])
        } else {
            format!(
                "{}-{}>{}",
                sq(x as usize, y as usize),
                sq(ex as usize, ey as usize),
                DN[m.4 as usize]
            )
        }
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 400,
            max_depth: 5,
        }
    }
    fn hash_key(&self) -> Option<u64> {
        Some(hkey(
            18,
            self.side,
            self.b.iter().enumerate().map(|(i, &v)| (i, v as u64)),
        ))
    }
    fn move_score(&self, m: &(u8, u8, u8, u8, u8)) -> i32 {
        m.2 as i32
    }
}

impl Impingere {
    /// Verifică mutarea; întoarce numărul de bile adverse împinse.
    fn try_move(
        &self,
        cells: &[(i32, i32)],
        gd: usize,
        md: usize,
        me: u8,
        len: i32,
    ) -> Option<usize> {
        let (dx, dy) = D[md];
        let inline = len > 1 && (D[md] == D[gd] || D[md] == (-D[gd].0, -D[gd].1));
        if inline {
            let head = if D[md] == D[gd] {
                cells[cells.len() - 1]
            } else {
                cells[0]
            };
            let mut chain = 0;
            let (mut cx, mut cy) = (head.0 + dx, head.1 + dy);
            while inb(cx, cy) && self.get(cx, cy) == 3 - me {
                chain += 1;
                cx += dx;
                cy += dy;
            }
            if chain >= len as usize {
                return None;
            }
            if !inb(head.0 + dx, head.1 + dy) {
                return None;
            }
            if chain == 0 {
                return if self.get(head.0 + dx, head.1 + dy) == 0 {
                    Some(0)
                } else {
                    None
                };
            }
            if inb(cx, cy) && self.get(cx, cy) != 0 {
                return None;
            }
            Some(chain)
        } else {
            for &(a, c) in cells {
                let (na, nc) = (a + dx, c + dy);
                if !inb(na, nc) || self.get(na, nc) != 0 {
                    return None;
                }
            }
            Some(0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::b1_util::testing::check;
    #[test]
    fn push_two_against_one() {
        let mut g = Impingere::new();
        g.b = [0; 49];
        g.b[0] = 1;
        g.b[1] = 1; // a1,b1 negre
        g.b[2] = 2; // c1 alb
        g.side = Player::One;
        let m = (0u8, 0u8, 2u8, 2u8, 2u8); // grup a1-b1, direcție est
        assert!(g.legal_moves().contains(&m));
        g.apply(&m);
        assert_eq!(g.b[2], 1);
        assert_eq!(g.b[3], 2);
        // 1 contra 1: ilegal
        let mut h = Impingere::new();
        h.b = [0; 49];
        h.b[0] = 1;
        h.b[1] = 2;
        assert!(!h.legal_moves().contains(&(0, 0, 1, 0, 2)));
    }
    #[test]
    fn push_off_board_scores() {
        let mut g = Impingere::new();
        g.b = [0; 49];
        g.b[4] = 1;
        g.b[5] = 1;
        g.b[6] = 2;
        g.apply(&(4, 0, 2, 2, 2));
        assert_eq!(g.out[1], 1);
        assert_eq!(g.b[6], 1);
    }
    #[test]
    fn random_and_ai() {
        check(|_| Impingere::new(), 600);
    }
}
