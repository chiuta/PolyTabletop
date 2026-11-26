//! Alquerque (Orientul Mijlociu / Spania) — tabla 5×5 cu diagonale, 12 piese pe jucător.
//!
//! * Puncte a1–e5; linii ortogonale peste tot, diagonale doar prin punctele „pare”
//!   (a1, c1, e1, b2, d2, a3, c3, e3, …, adică culoarea 1 a tablei).
//! * Start: J1 ocupă rândurile 1–2 și a3, b3; J2 rândurile 4–5 și d3, e3; c3 e liber.
//! * O piesă mută un pas pe o linie, în orice direcție, pe un punct liber.
//! * Captura (sărind peste o piesă adversă alăturată pe linie, pe un punct liber) e
//!   obligatorie; salturile continuă cât se poate (se joacă toată secvența), `a3xc3xe3`.
//! * Câștigă cine capturează toate piesele adversarului sau îl blochează.
//!   Remiză după 100 de mutări fără captură.
//!
//! Sursă: regulile publice ale jocului tradițional (Wikipedia „Alquerque”, varianta de 12).

#![allow(clippy::needless_range_loop)]

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: i32 = 5;

fn name(i: usize) -> String {
    format!(
        "{}{}",
        (b'a' + (i as i32 % N) as u8) as char,
        i as i32 / N + 1
    )
}
fn diag_ok(x: i32, y: i32) -> bool {
    (x + y) % 2 == 0
}
fn inb(x: i32, y: i32) -> bool {
    (0..N).contains(&x) && (0..N).contains(&y)
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Mv(pub Vec<u8>);

#[derive(Clone)]
pub struct Alquerque {
    b: [u8; 25],
    side: Player,
    quiet: u16,
}

impl Default for Alquerque {
    fn default() -> Self {
        Self::new()
    }
}

fn dirs(x: i32, y: i32) -> Vec<(i32, i32)> {
    let mut d = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    if diag_ok(x, y) {
        d.extend([(1, 1), (1, -1), (-1, 1), (-1, -1)]);
    }
    d
}

impl Alquerque {
    pub fn new() -> Self {
        let mut b = [0u8; 25];
        for i in 0..25 {
            let (x, y) = (i as i32 % N, i as i32 / N);
            b[i] = match y {
                0 | 1 => 1,
                3 | 4 => 2,
                _ => match x {
                    0 | 1 => 1,
                    3 | 4 => 2,
                    _ => 0,
                },
            };
        }
        Alquerque {
            b,
            side: Player::One,
            quiet: 0,
        }
    }

    pub fn count(&self, p: Player) -> usize {
        self.b.iter().filter(|&&c| c == p.idx() as u8 + 1).count()
    }

    fn chains(b: &[u8; 25], pos: usize, me: u8, path: &mut Vec<u8>, out: &mut Vec<Vec<u8>>) {
        let (x, y) = (pos as i32 % N, pos as i32 / N);
        let mut cont = false;
        for (dx, dy) in dirs(x, y) {
            let (mx, my, tx, ty) = (x + dx, y + dy, x + 2 * dx, y + 2 * dy);
            if !inb(tx, ty) {
                continue;
            }
            let (mi, ti) = ((my * N + mx) as usize, (ty * N + tx) as usize);
            if b[mi] == 3 - me && b[ti] == 0 {
                cont = true;
                let mut nb = *b;
                nb[mi] = 0;
                nb[pos] = 0;
                nb[ti] = me;
                path.push(ti as u8);
                Self::chains(&nb, ti, me, path, out);
                path.pop();
            }
        }
        if !cont && path.len() > 1 {
            out.push(path.clone());
        }
    }

    fn moves_for(&self, side: Player) -> Vec<Mv> {
        let me = side.idx() as u8 + 1;
        let mut caps = Vec::new();
        for i in 0..25 {
            if self.b[i] == me {
                let mut out = Vec::new();
                let mut path = vec![i as u8];
                Self::chains(&self.b, i, me, &mut path, &mut out);
                caps.extend(out.into_iter().map(Mv));
            }
        }
        if !caps.is_empty() {
            return caps;
        }
        let mut v = Vec::new();
        for i in 0..25 {
            if self.b[i] != me {
                continue;
            }
            let (x, y) = (i as i32 % N, i as i32 / N);
            for (dx, dy) in dirs(x, y) {
                let (tx, ty) = (x + dx, y + dy);
                if inb(tx, ty) && self.b[(ty * N + tx) as usize] == 0 {
                    v.push(Mv(vec![i as u8, (ty * N + tx) as u8]));
                }
            }
        }
        v
    }
}

impl Game for Alquerque {
    type Move = Mv;
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<Mv> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        self.moves_for(self.side)
    }
    fn apply(&mut self, m: &Mv) {
        let me = self.side.idx() as u8 + 1;
        let mut cap = false;
        for w in m.0.windows(2) {
            let (a, c) = (w[0] as i32, w[1] as i32);
            self.b[a as usize] = 0;
            if (a % N - c % N).abs() == 2 || (a / N - c / N).abs() == 2 {
                let mid = ((a % N + c % N) / 2) + ((a / N + c / N) / 2) * N;
                self.b[mid as usize] = 0;
                cap = true;
            }
            self.b[c as usize] = me;
        }
        self.quiet = if cap { 0 } else { self.quiet + 1 };
        self.side = self.side.other();
    }
    fn outcome(&self) -> Option<Outcome> {
        for p in [Player::One, Player::Two] {
            if self.count(p) == 0 {
                return Some(Outcome::Win(p.other()));
            }
        }
        if self.quiet >= 100 {
            return Some(Outcome::Draw);
        }
        if self.moves_for(self.side).is_empty() {
            return Some(Outcome::Win(self.side.other()));
        }
        None
    }
    fn evaluate(&self, p: Player) -> i32 {
        let d = self.count(p) as i32 - self.count(p.other()) as i32;
        100 * d
    }
    fn render(&self) -> String {
        let mut s = String::new();
        for y in (0..N).rev() {
            s.push_str(&format!("{}  ", y + 1));
            for x in 0..N {
                s.push(match self.b[(y * N + x) as usize] {
                    1 => 'X',
                    2 => 'O',
                    _ => '.',
                });
                s.push(' ');
            }
            s.push('\n');
        }
        s.push_str("   a b c d e\nX = J1, O = J2. Diagonalele trec doar prin punctele a1, c1, e1, b2, d2, a3, c3, e3, b4, d4, a5, c5, e5.\n");
        s.push_str(&format!("La mutare: Jucătorul {}.", self.side.idx() + 1));
        s
    }
    fn format_move(&self, m: &Mv) -> String {
        let jump = m.0.len() > 2 || {
            let (a, c) = (m.0[0] as i32, m.0[1] as i32);
            (a % N - c % N).abs() == 2 || (a / N - c / N).abs() == 2
        };
        let mut t = name(m.0[0] as usize);
        for &q in &m.0[1..] {
            t.push(if jump { 'x' } else { '-' });
            t.push_str(&name(q as usize));
        }
        t
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 250,
            max_depth: 8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q4_util::*;
    use polytabletop_core::{GameObj, Player};

    #[test]
    fn opening_has_only_c3_moves() {
        let g = Alquerque::new();
        assert_eq!(g.count(Player::One), 12);
        let m = GameObj::legal_moves(&g);
        assert!(m.contains(&"b3-c3".to_string()), "{m:?}");
        assert!(m.contains(&"b2-c3".to_string()), "{m:?}");
        assert!(m.iter().all(|t| t.ends_with("c3")), "{m:?}");
    }

    #[test]
    fn capture_is_mandatory_and_chains() {
        let mut g = Alquerque::new();
        g.b = [0; 25];
        g.b[0] = 1; // a1
        g.b[6] = 2; // b2
        g.b[18] = 2; // d4
        g.b[2] = 1; // c1
        let m = GameObj::legal_moves(&g);
        assert!(m.contains(&"a1xc3xe5".to_string()), "{m:?}");
        assert!(m.iter().all(|t| t.contains('x')));
    }

    #[test]
    fn random_and_ai_games_finish() {
        random_finishes(|_| Box::new(Alquerque::new()), 6, 600);
        ai_vs_random(|_| Box::new(Alquerque::new()), 2, Player::One, 600);
        ai_vs_random(|_| Box::new(Alquerque::new()), 3, Player::Two, 600);
    }
}
