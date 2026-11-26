//! Lasca (Emanuel Lasker, 1911): 7×7, 25 pătrate întunecate, câte 11 soldați.
//!
//! * Soldații mută o căsuță pe diagonală înainte; ofițerii (soldații ajunși pe ultimul
//!   rând) mută și capturează în toate cele patru direcții, câte o căsuță.
//! * Captura: sari peste o coloană adversă adiacentă pe un pătrat gol; **doar piesa de
//!   deasupra** e luată și trece la **baza coloanei tale**; restul coloanei sărite rămâne
//!   pe loc. Captura e obligatorie (orice captură), săriturile se înlănțuie, dar aceeași
//!   coloană nu se sare de două ori într-o mutare. Soldatul care ajunge pe ultimul rând
//!   devine ofițer și mutarea se termină.
//! * Coloana aparține celui a cărui piesă e deasupra. Pierde cine nu mai are mutări.
//! * Remiză după 60 de semi-mutări fără captură și fără mutare de soldat.
//!
//! Mutări: `a1-b2`, `a1xc3xe5` (pătratele de aterizare).

use crate::q5_util::{sq_name, verdict};
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: usize = 7;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Mv {
    pub path: Vec<u8>,
    pub cap: bool,
}

#[derive(Clone)]
pub struct Lasca {
    st: [Vec<u8>; N * N],
    side: Player,
    quiet: u16,
}

impl Default for Lasca {
    fn default() -> Self {
        Self::new()
    }
}

// cod: 0 soldat J1, 1 ofițer J1, 2 soldat J2, 3 ofițer J2
fn own(c: u8) -> Player {
    if c < 2 {
        Player::One
    } else {
        Player::Two
    }
}
fn officer(c: u8) -> bool {
    c % 2 == 1
}
fn inside(r: i32, f: i32) -> bool {
    (0..N as i32).contains(&r) && (0..N as i32).contains(&f)
}

impl Lasca {
    pub fn new() -> Self {
        let mut st: [Vec<u8>; N * N] = std::array::from_fn(|_| Vec::new());
        for r in 0..N {
            for f in 0..N {
                if (f + r) % 2 == 0 {
                    if r < 3 {
                        st[r * N + f].push(0);
                    } else if r > 3 {
                        st[r * N + f].push(2);
                    }
                }
            }
        }
        Lasca {
            st,
            side: Player::One,
            quiet: 0,
        }
    }

    fn top(&self, i: usize) -> Option<u8> {
        self.st[i].last().copied()
    }

    fn dirs(c: u8) -> Vec<(i32, i32)> {
        if officer(c) {
            vec![(1, -1), (1, 1), (-1, -1), (-1, 1)]
        } else if own(c) == Player::One {
            vec![(1, -1), (1, 1)]
        } else {
            vec![(-1, -1), (-1, 1)]
        }
    }

    fn far_row(p: Player) -> i32 {
        if p == Player::One {
            (N - 1) as i32
        } else {
            0
        }
    }

    fn grab(
        &self,
        at: usize,
        c: u8,
        from: usize,
        taken: &mut Vec<usize>,
        path: &mut Vec<u8>,
        out: &mut Vec<Vec<u8>>,
    ) {
        let mut found = false;
        let (r0, f0) = ((at / N) as i32, (at % N) as i32);
        for (dr, df) in Self::dirs(c) {
            let (mr, mf, lr, lf) = (r0 + dr, f0 + df, r0 + 2 * dr, f0 + 2 * df);
            if !inside(lr, lf) {
                continue;
            }
            let mid = (mr * N as i32 + mf) as usize;
            let land = (lr * N as i32 + lf) as usize;
            let Some(t) = self.top(mid) else { continue };
            if own(t) == own(c) || taken.contains(&mid) {
                continue;
            }
            if !(self.st[land].is_empty() || land == from) {
                continue;
            }
            found = true;
            taken.push(mid);
            path.push(land as u8);
            if !officer(c) && lr == Self::far_row(own(c)) {
                out.push(path.clone());
            } else {
                self.grab(land, c, from, taken, path, out);
            }
            path.pop();
            taken.pop();
        }
        if !found && path.len() > 1 {
            out.push(path.clone());
        }
    }

    fn gen(&self) -> Vec<Mv> {
        let me = self.side;
        let mut caps = Vec::new();
        for i in 0..N * N {
            if let Some(c) = self.top(i) {
                if own(c) == me {
                    let mut path = vec![i as u8];
                    self.grab(i, c, i, &mut Vec::new(), &mut path, &mut caps);
                }
            }
        }
        if !caps.is_empty() {
            return caps
                .into_iter()
                .map(|path| Mv { path, cap: true })
                .collect();
        }
        let mut out = Vec::new();
        for i in 0..N * N {
            if let Some(c) = self.top(i) {
                if own(c) != me {
                    continue;
                }
                let (r0, f0) = ((i / N) as i32, (i % N) as i32);
                for (dr, df) in Self::dirs(c) {
                    let (r, f) = (r0 + dr, f0 + df);
                    if inside(r, f) && self.st[(r * N as i32 + f) as usize].is_empty() {
                        out.push(Mv {
                            path: vec![i as u8, (r * N as i32 + f) as u8],
                            cap: false,
                        });
                    }
                }
            }
        }
        out
    }
}

fn sq(i: u8) -> String {
    sq_name(i as usize % N, i as usize / N)
}

impl Game for Lasca {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if self.quiet >= 60 {
            return Vec::new();
        }
        self.gen()
    }

    fn apply(&mut self, m: &Mv) {
        let from = m.path[0] as usize;
        let to = *m.path.last().unwrap() as usize;
        let mut col = std::mem::take(&mut self.st[from]);
        let was_soldier = !officer(*col.last().unwrap());
        for w in m.path.windows(2) {
            if m.cap {
                let mid = ((w[0] as usize / N + w[1] as usize / N) / 2) * N
                    + (w[0] as usize % N + w[1] as usize % N) / 2;
                let t = self.st[mid].pop().expect("coloană sărită");
                col.insert(0, t);
            }
        }
        if was_soldier && (to / N) as i32 == Self::far_row(self.side) {
            let t = col.last_mut().unwrap();
            *t |= 1;
            self.quiet = 0;
        }
        self.st[to] = col;
        if m.cap || was_soldier {
            self.quiet = 0;
        } else {
            self.quiet += 1;
        }
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.quiet >= 60 {
            return Some(Outcome::Draw);
        }
        if self.gen().is_empty() {
            return Some(Outcome::Win(self.side.other()));
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let mut s = 0;
        for i in 0..N * N {
            let col = &self.st[i];
            let Some(&t) = col.last() else { continue };
            let sign = if own(t) == p { 1 } else { -1 };
            let row = (i / N) as i32;
            let adv = if own(t) == Player::One {
                row
            } else {
                N as i32 - 1 - row
            };
            let mut v = if officer(t) { 230 } else { 100 + adv * 5 };
            for &c in col.iter().take(col.len() - 1) {
                v += if own(c) == own(t) { 25 } else { -25 };
            }
            s += sign * v;
            for &c in col.iter().take(col.len() - 1) {
                if own(c) != own(t) {
                    // prizonier al celuilalt: valoare pentru stăpânul lui
                    s += if own(c) == p { 20 } else { -20 };
                }
            }
        }
        s
    }

    fn render(&self) -> String {
        let mut s = String::from("Lasca — o/O = J1 (jos), x/X = J2 (sus), majuscula = ofițer; cifra = înălțimea coloanei\n");
        s.push_str("    a   b   c   d   e   f   g\n");
        for r in (0..N).rev() {
            s.push_str(&format!("{}  ", r + 1));
            for f in 0..N {
                let cell = match self.top(r * N + f) {
                    Some(c) => {
                        let ch = match c {
                            0 => 'o',
                            1 => 'O',
                            2 => 'x',
                            _ => 'X',
                        };
                        let h = self.st[r * N + f].len();
                        if h > 1 {
                            format!("{ch}{h} ")
                        } else {
                            format!("{ch}  ")
                        }
                    }
                    None if (f + r) % 2 == 0 => ".  ".to_string(),
                    None => "   ".to_string(),
                };
                s.push_str(&cell);
                s.push(' ');
            }
            s.push_str(&format!("{}\n", r + 1));
        }
        let mut cols = Vec::new();
        for i in 0..N * N {
            if self.st[i].len() > 1 {
                let txt: String = self.st[i]
                    .iter()
                    .map(|&c| ['o', 'O', 'x', 'X'][c as usize])
                    .collect();
                cols.push(format!("{}:{}", sq(i as u8), txt));
            }
        }
        if !cols.is_empty() {
            s.push_str(&format!("Coloane (de jos în sus): {}\n", cols.join(" ")));
        }
        let v = verdict(self.outcome());
        if v.is_empty() {
            s.push_str(&format!("La mutare: Jucător {}\n", self.side.idx() + 1));
        } else {
            s.push_str(&v);
        }
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        let sep = if m.cap { "x" } else { "-" };
        m.path.iter().map(|&i| sq(i)).collect::<Vec<_>>().join(sep)
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q5_util::testkit::check_game;

    #[test]
    fn opening_moves_and_setup() {
        let g = Lasca::new();
        assert_eq!(g.st.iter().filter(|c| !c.is_empty()).count(), 22);
        assert_eq!(g.legal_moves().len(), 6);
    }

    #[test]
    fn capture_takes_only_the_top_piece_to_the_bottom() {
        let mut g = Lasca::new();
        g.st = std::array::from_fn(|_| Vec::new());
        g.st[2 * N + 2] = vec![0]; // c3
        g.st[3 * N + 3] = vec![0, 2]; // d4: soldat J1 acoperit de soldat J2
        let m = g.legal_moves();
        assert_eq!(m.len(), 1);
        assert_eq!(g.format_move(&m[0]), "c3xe5");
        g.apply(&m[0]);
        assert_eq!(g.st[3 * N + 3], vec![0]); // rămâne soldatul J1
        assert_eq!(g.st[4 * N + 4], vec![2, 0]); // coloană: baza = piesa capturată
        assert_eq!(g.top(4 * N + 4), Some(0));
    }

    #[test]
    fn promotion_ends_the_move() {
        let mut g = Lasca::new();
        g.st = std::array::from_fn(|_| Vec::new());
        g.st[5 * N + 1] = vec![0]; // b6
        g.apply(&Mv {
            path: vec![(5 * N + 1) as u8, (6 * N + 2) as u8],
            cap: false,
        });
        assert_eq!(g.st[6 * N + 2], vec![1]);
    }

    #[test]
    fn random_and_ai_games() {
        check_game(|_| Lasca::new(), 4, 3000);
    }
}
