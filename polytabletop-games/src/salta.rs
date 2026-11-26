//! Salta (Konrad Büttgenbach, 1899): 10×10, doar pătratele întunecate, câte 15 piese
//! pe primele trei rânduri ale fiecărui jucător. Nu se capturează nimic: scopul este să
//! muți toate cele 15 piese în cele 15 pătrate de start ale adversarului.
//!
//! * Mutare simplă: o piesă merge un pas pe diagonală, înainte sau înapoi, pe un pătrat gol.
//! * Săritura: peste o piesă **adversă** alăturată, pe diagonală **înainte**, pe pătratul
//!   gol de dincolo; piesa sărită rămâne pe loc. Săritura este **obligatorie** dacă există
//!   (orice săritură). Simplificare: o singură săritură pe mutare.
//! * Câștigă cine ocupă primul toate cele 15 pătrate-țintă. După 240 de semi-mutări (120
//!   pe parte) câștigă cine mai are de parcurs mai puțin (suma rândurilor lipsă până în zona
//!   țintă; dacă J1 a terminat primul, J2 primește 1 punct de bonus). Egal = remiză.
//! * Fără regula „ordinii pieselor” din regulamentul original.
//!
//! Mutări: `a1-b2` (pas), `c3xe5` (săritură).

use crate::q5_util::{by_score, sq_name, verdict};
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: usize = 10;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Mv {
    pub from: u8,
    pub to: u8,
    pub jump: bool,
}

#[derive(Clone)]
pub struct Salta {
    c: [u8; N * N],
    side: Player,
    plies: u16,
}

impl Default for Salta {
    fn default() -> Self {
        Self::new()
    }
}

fn dark(r: usize, f: usize) -> bool {
    (r + f).is_multiple_of(2)
}

impl Salta {
    pub fn new() -> Self {
        let mut c = [0u8; N * N];
        for r in 0..3 {
            for f in 0..N {
                if dark(r, f) {
                    c[r * N + f] = 1;
                }
                if dark(N - 1 - r, f) {
                    c[(N - 1 - r) * N + f] = 2;
                }
            }
        }
        Salta {
            c,
            side: Player::One,
            plies: 0,
        }
    }

    fn goal_rows(p: Player) -> std::ops::Range<usize> {
        if p == Player::One {
            7..10
        } else {
            0..3
        }
    }

    pub fn home(&self, p: Player) -> usize {
        let mut n = 0;
        for r in Self::goal_rows(p) {
            for f in 0..N {
                if dark(r, f) && self.c[r * N + f] == p.idx() as u8 + 1 {
                    n += 1;
                }
            }
        }
        n
    }

    /// Cât mai au de parcurs piesele lui `p` până în zona țintă (rânduri).
    pub fn remaining(&self, p: Player) -> i32 {
        let me = p.idx() as u8 + 1;
        let mut s = 0;
        for i in 0..N * N {
            if self.c[i] == me {
                let r = (i / N) as i32;
                s += if p == Player::One {
                    (7 - r).max(0)
                } else {
                    (r - 2).max(0)
                };
            }
        }
        s
    }

    fn done(&self, p: Player) -> bool {
        self.home(p) == 15
    }
}

impl Game for Salta {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if self.done(Player::One) || self.done(Player::Two) || self.plies >= 240 {
            return Vec::new();
        }
        let me = self.side.idx() as u8 + 1;
        let fwd = if self.side == Player::One { 1i32 } else { -1 };
        let mut jumps = Vec::new();
        let mut steps = Vec::new();
        for i in 0..N * N {
            if self.c[i] != me {
                continue;
            }
            let (r, f) = ((i / N) as i32, (i % N) as i32);
            for df in [-1i32, 1] {
                // săritură înainte
                let (mr, mf, lr, lf) = (r + fwd, f + df, r + 2 * fwd, f + 2 * df);
                if (0..N as i32).contains(&lr) && (0..N as i32).contains(&lf) {
                    let mid = self.c[(mr * N as i32 + mf) as usize];
                    let land = (lr * N as i32 + lf) as usize;
                    if mid == 3 - me && self.c[land] == 0 {
                        jumps.push(Mv {
                            from: i as u8,
                            to: land as u8,
                            jump: true,
                        });
                    }
                }
                for dr in [fwd, -fwd] {
                    let (nr, nf) = (r + dr, f + df);
                    if (0..N as i32).contains(&nr)
                        && (0..N as i32).contains(&nf)
                        && self.c[(nr * N as i32 + nf) as usize] == 0
                    {
                        steps.push(Mv {
                            from: i as u8,
                            to: (nr * N as i32 + nf) as u8,
                            jump: false,
                        });
                    }
                }
            }
        }
        if jumps.is_empty() {
            steps
        } else {
            jumps
        }
    }

    fn apply(&mut self, m: &Mv) {
        self.c[m.to as usize] = self.c[m.from as usize];
        self.c[m.from as usize] = 0;
        self.plies += 1;
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        let (a, b) = (self.done(Player::One), self.done(Player::Two));
        if a && !b {
            return Some(Outcome::Win(Player::One));
        }
        if b && !a {
            return Some(Outcome::Win(Player::Two));
        }
        if a && b {
            return Some(Outcome::Draw);
        }
        if self.plies >= 240 {
            return Some(by_score(
                self.remaining(Player::Two),
                self.remaining(Player::One),
            ));
        }
        if self.legal_moves().is_empty() {
            // cine nu poate muta pierde (regula „nu bloca adversarul” e aplicată prin pierdere)
            return Some(Outcome::Win(self.side.other()));
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let o = p.other();
        (self.remaining(o) - self.remaining(p)) * 20
            + (self.home(p) as i32 - self.home(o) as i32) * 30
    }

    fn render(&self) -> String {
        let mut s = String::from("Salta — o = J1 (pleacă jos, ținta sus), x = J2 (pleacă sus, ținta jos)\n   a b c d e f g h i j\n");
        for r in (0..N).rev() {
            s.push_str(&format!("{:>2} ", r + 1));
            for f in 0..N {
                s.push(match self.c[r * N + f] {
                    1 => 'o',
                    2 => 'x',
                    _ if dark(r, f) => '.',
                    _ => ' ',
                });
                s.push(' ');
            }
            s.push_str(&format!("{}\n", r + 1));
        }
        s.push_str(&format!(
            "În țintă: J1 {}/15, J2 {}/15. Semi-mutări {}/240. Mai au de parcurs: J1 {}, J2 {}.\n",
            self.home(Player::One),
            self.home(Player::Two),
            self.plies,
            self.remaining(Player::One),
            self.remaining(Player::Two)
        ));
        let v = verdict(self.outcome());
        if v.is_empty() {
            s.push_str(&format!("La mutare: Jucător {}\n", self.side.idx() + 1));
        } else {
            s.push_str(&v);
        }
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        let a = sq_name(m.from as usize % N, m.from as usize / N);
        let b = sq_name(m.to as usize % N, m.to as usize / N);
        format!("{a}{}{b}", if m.jump { "x" } else { "-" })
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q5_util::testkit::check_game;

    #[test]
    fn setup_and_forced_jump() {
        let g = Salta::new();
        assert_eq!(g.c.iter().filter(|&&x| x == 1).count(), 15);
        assert_eq!(g.c.iter().filter(|&&x| x == 2).count(), 15);
        let mut h = Salta::new();
        h.c = [0; N * N];
        h.c[4 * N + 4] = 1; // e5
        h.c[5 * N + 5] = 2; // f6
        h.c[N] = 1;
        let mv = h.legal_moves();
        assert_eq!(mv.len(), 1);
        assert_eq!(h.format_move(&mv[0]), "e5xg7");
        h.apply(&mv[0]);
        assert_eq!(h.c[5 * N + 5], 2, "piesa sărită rămâne");
    }

    #[test]
    fn random_and_ai_games() {
        check_game(|_| Salta::new(), 3, 3000);
    }
}
