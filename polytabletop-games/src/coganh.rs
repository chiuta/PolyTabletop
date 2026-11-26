#![allow(clippy::doc_lazy_continuation)]
//! Cờ gánh (Vietnam), joc de capturi pe tabla 5×5 cu diagonale, pentru doi jucători.
//!
//! * Tablă 5×5 (25 de puncte); liniile ortogonale unesc toate punctele vecine, iar diagonalele
//!   există doar prin punctele cu (rând + coloană) par (ca la Alquerque). Fiecare jucător are 8 piese:
//!   J1 (X) tot rândul 1, plus a2, e2 și e3; J2 (O) tot rândul 5, plus a4, e4 și a3.
//! * O mutare = un pas pe o linie spre un punct liber.
//! * Capturile (piesele capturate **își schimbă culoarea**, nu dispar):
//!   - **Gánh** („cărat”): piesa mutată încadrează o piesă adversă pe aceeași linie (între ea și o
//!     piesă a ta), sau aterizează între două piese adverse de pe aceeași linie: ambele se transformă.
//!   - **Vây** („încercuire”): orice grup de piese adverse conectate care nu mai are niciun punct liber
//!     în jur se transformă.
//!   Chém (capturarea obligatorie prin ocuparea golului lăsat de adversar) nu e implementată.
//! * Câștigă cine rămâne cu toate piesele sau cine rămâne cu mutări când adversarul nu mai are;
//!   remiză după 100 de mutări fără nicio captură, cu număr egal de piese (altfel câștigă cine are mai multe).
//! Mutări: `a1b2` (de la, la).

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const S: usize = 5;
const N: usize = 25;
const QUIET: u16 = 100;
const DIRS: [(i32, i32); 8] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
];

fn name(i: usize) -> String {
    polytabletop_core::square_name(i % S, i / S)
}

/// Vecinul lui `i` pe direcția `d`, dacă există linie.
fn nb(i: usize, d: usize) -> Option<usize> {
    let (r, c) = ((i / S) as i32, (i % S) as i32);
    let (dr, dc) = DIRS[d];
    if d >= 4 && (r + c) % 2 != 0 {
        return None;
    }
    let (nr, nc) = (r + dr, c + dc);
    if nr < 0 || nc < 0 || nr >= S as i32 || nc >= S as i32 {
        return None;
    }
    Some(nr as usize * S + nc as usize)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Mv {
    pub from: u8,
    pub to: u8,
}

#[derive(Clone)]
pub struct CoGanh {
    b: [u8; N],
    side: Player,
    quiet: u16,
}

impl Default for CoGanh {
    fn default() -> Self {
        Self::new()
    }
}

impl CoGanh {
    pub fn new() -> Self {
        let mut b = [0u8; N];
        for c in 0..S {
            b[c] = 1;
            b[4 * S + c] = 2;
        }
        // simetrie la 180°: X ia a2, e2, e3; O ia a4, e4, a3
        for (r, c, v) in [
            (1, 0, 1),
            (1, 4, 1),
            (2, 4, 1),
            (3, 4, 2),
            (3, 0, 2),
            (2, 0, 2),
        ] {
            b[r * S + c] = v;
        }
        CoGanh {
            b,
            side: Player::One,
            quiet: 0,
        }
    }

    fn count(&self, c: u8) -> usize {
        self.b.iter().filter(|&&x| x == c).count()
    }

    /// Mută piesa și aplică capturile pe `b`; întoarce numărul de piese transformate.
    fn play(b: &mut [u8; N], from: usize, to: usize, me: u8) -> usize {
        let opp = 3 - me;
        b[from] = 0;
        b[to] = me;
        let mut flips = 0;
        // gánh: piesa mutată încadrează / e încadrată
        for d in 0..8 {
            let rev = match d {
                0 => 1,
                1 => 0,
                2 => 3,
                3 => 2,
                4 => 7,
                5 => 6,
                6 => 5,
                _ => 4,
            };
            if d > rev {
                continue;
            }
            // aterizare între două piese adverse pe linia d / rev
            if let (Some(a), Some(c)) = (nb(to, d), nb(to, rev)) {
                if b[a] == opp && b[c] == opp {
                    b[a] = me;
                    b[c] = me;
                    flips += 2;
                }
            }
        }
        for d in 0..8 {
            if let Some(a) = nb(to, d) {
                if b[a] == opp {
                    if let Some(c) = nb(a, d) {
                        if b[c] == me {
                            b[a] = me;
                            flips += 1;
                        }
                    }
                }
            }
        }
        // vây: grupuri adverse fără puncte libere
        let mut seen = [false; N];
        for s in 0..N {
            if b[s] != opp || seen[s] {
                continue;
            }
            let mut group = vec![s];
            seen[s] = true;
            let mut free = false;
            let mut k = 0;
            while k < group.len() {
                let x = group[k];
                k += 1;
                for d in 0..8 {
                    if let Some(y) = nb(x, d) {
                        if b[y] == 0 {
                            free = true;
                        } else if b[y] == opp && !seen[y] {
                            seen[y] = true;
                            group.push(y);
                        }
                    }
                }
            }
            if !free {
                for &x in &group {
                    b[x] = me;
                }
                flips += group.len();
            }
        }
        flips
    }

    fn gen(&self) -> Vec<Mv> {
        let me = self.side.idx() as u8 + 1;
        let mut v = Vec::new();
        for from in 0..N {
            if self.b[from] != me {
                continue;
            }
            for d in 0..8 {
                if let Some(to) = nb(from, d) {
                    if self.b[to] == 0 {
                        v.push(Mv {
                            from: from as u8,
                            to: to as u8,
                        });
                    }
                }
            }
        }
        v
    }

    fn terminal(&self) -> Option<Outcome> {
        let (a, b) = (self.count(1), self.count(2));
        if b == 0 {
            return Some(Outcome::Win(Player::One));
        }
        if a == 0 {
            return Some(Outcome::Win(Player::Two));
        }
        if self.quiet >= QUIET {
            return Some(match a.cmp(&b) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                std::cmp::Ordering::Equal => Outcome::Draw,
            });
        }
        if self.gen().is_empty() {
            return Some(Outcome::Win(self.side.other()));
        }
        None
    }
}

impl Game for CoGanh {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if self.terminal().is_some() {
            return Vec::new();
        }
        self.gen()
    }

    fn apply(&mut self, m: &Mv) {
        let me = self.side.idx() as u8 + 1;
        let f = Self::play(&mut self.b, m.from as usize, m.to as usize, me);
        self.quiet = if f > 0 { 0 } else { self.quiet + 1 };
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        self.terminal()
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx() as u8 + 1;
        let (a, b) = (self.count(me) as i32, self.count(3 - me) as i32);
        let mob = |c: u8| -> i32 {
            (0..N)
                .filter(|&i| self.b[i] == c)
                .map(|i| {
                    (0..8)
                        .filter(|&d| nb(i, d).is_some_and(|j| self.b[j] == 0))
                        .count() as i32
                })
                .sum()
        };
        100 * (a - b) + 2 * (mob(me) - mob(3 - me))
    }

    fn render(&self) -> String {
        let mut s = String::from("   a b c d e\n");
        for r in (0..S).rev() {
            s.push_str(&format!("{}  ", r + 1));
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
            "Piese: X {}  O {}. La mutare: {}. (diagonale doar prin punctele a1, c1, b2, …)\n",
            self.count(1),
            self.count(2),
            if self.side == Player::One { "X" } else { "O" }
        ));
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        format!("{}{}", name(m.from as usize), name(m.to as usize))
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    #[test]
    fn start_has_eight_pieces_each() {
        let g = CoGanh::new();
        assert_eq!(g.count(1), 8);
        assert_eq!(g.count(2), 8);
        assert!(!g.legal_moves().is_empty());
    }

    #[test]
    fn ganh_flips_a_flanked_piece_and_lands_between_two() {
        let mut b = [0u8; N];
        b[2 * S + 1] = 2; // b3 negru
        b[2 * S + 3] = 1; // d3 alb
        b[S + 2] = 1; // c2 alb va muta pe c3? c2->c3 încadrează b3 între c3 și ... a3 nu e a lui
        let f = CoGanh::play(&mut b, S + 2, 2 * S + 2, 1);
        // c3 are vecin b3 (negru) și dincolo a3 (gol) => nu; dar c3 e între b3 și d3? d3 e al lui: nu captură
        assert_eq!(f, 0);
        let mut b = [0u8; N];
        b[2 * S + 1] = 2;
        b[2 * S + 3] = 2;
        b[S + 2] = 1;
        let f = CoGanh::play(&mut b, S + 2, 2 * S + 2, 1);
        assert_eq!(f, 2);
        assert_eq!(b[2 * S + 1], 1);
        assert_eq!(b[2 * S + 3], 1);
    }

    #[test]
    fn vay_flips_a_blocked_group() {
        let mut b = [0u8; N];
        // O la a1 înconjurat: b1, a2 și diagonala b2 ocupate de X
        b[0] = 2;
        b[1] = 1;
        b[S] = 1;
        b[S + 1] = 1;
        b[3 * S + 3] = 1; // piesă care mută
        b[3 * S + 4] = 0;
        let f = CoGanh::play(&mut b, 3 * S + 3, 3 * S + 4, 1);
        assert_eq!(f, 1);
        assert_eq!(b[0], 1);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=15u64 {
            let mut g = CoGanh::new();
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
                assert!(n < 3000);
            }
        }
    }
}
