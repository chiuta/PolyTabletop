//! Ludus latrunculorum (Roma antică), reconstituire modernă pe 8×8.
//!
//! Regulile originale nu s-au păstrat integral; folosim reconstituirea cea mai răspândită:
//! * 8×8, câte 16 piese pe primele două rânduri (J1 jos, J2 sus).
//! * Orice piesă mută ca turnul (orizontal/vertical, oricât, fără sărituri).
//! * Captura „în clește”: mutarea ta închide o piesă adversă între două piese ale tale pe
//!   o linie sau coloană (doar piesa mutată captureză). A te băga singur între două piese
//!   adverse este sigur. O piesă dintr-un colț e capturată când ambele ei vecine sunt ale
//!   adversarului.
//! * Câștigă cine lasă adversarul cu o singură piesă sau fără mutări. Fără dux și fără
//!   „blocarea” piesei izolate. După 240 de semi-mutări câștigă cine are mai multe piese.
//!
//! Mutări: `a2-a5`.

use crate::q5_util::{by_score, sq_name, verdict};
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: usize = 8;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Mv {
    pub from: u8,
    pub to: u8,
}

#[derive(Clone)]
pub struct Latrunculi {
    c: [u8; N * N],
    side: Player,
    plies: u16,
}

impl Default for Latrunculi {
    fn default() -> Self {
        Self::new()
    }
}

impl Latrunculi {
    pub fn new() -> Self {
        let mut c = [0u8; N * N];
        for f in 0..N {
            c[f] = 1;
            c[N + f] = 1;
            c[6 * N + f] = 2;
            c[7 * N + f] = 2;
        }
        Latrunculi {
            c,
            side: Player::One,
            plies: 0,
        }
    }

    pub fn count(&self, p: Player) -> i32 {
        self.c.iter().filter(|&&x| x == p.idx() as u8 + 1).count() as i32
    }
}

impl Game for Latrunculi {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if self.count(Player::One) <= 1 || self.count(Player::Two) <= 1 || self.plies >= 240 {
            return Vec::new();
        }
        let me = self.side.idx() as u8 + 1;
        let mut out = Vec::new();
        for i in 0..N * N {
            if self.c[i] != me {
                continue;
            }
            let (r0, f0) = ((i / N) as i32, (i % N) as i32);
            for (dr, df) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let (mut r, mut f) = (r0 + dr, f0 + df);
                while (0..N as i32).contains(&r)
                    && (0..N as i32).contains(&f)
                    && self.c[(r * N as i32 + f) as usize] == 0
                {
                    out.push(Mv {
                        from: i as u8,
                        to: (r * N as i32 + f) as u8,
                    });
                    r += dr;
                    f += df;
                }
            }
        }
        out
    }

    fn apply(&mut self, m: &Mv) {
        let me = self.side.idx() as u8 + 1;
        let you = 3 - me;
        let (from, to) = (m.from as usize, m.to as usize);
        self.c[to] = me;
        self.c[from] = 0;
        self.plies += 1;
        let (r, f) = ((to / N) as i32, (to % N) as i32);
        for (dr, df) in [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)] {
            let (nr, nf, or, of) = (r + dr, f + df, r + 2 * dr, f + 2 * df);
            if !(0..N as i32).contains(&nr) || !(0..N as i32).contains(&nf) {
                continue;
            }
            let n = (nr * N as i32 + nf) as usize;
            if self.c[n] != you {
                continue;
            }
            if (0..N as i32).contains(&or)
                && (0..N as i32).contains(&of)
                && self.c[(or * N as i32 + of) as usize] == me
            {
                self.c[n] = 0;
            }
        }
        // colțuri: piesa adversă din colț cu ambii vecini ai mutatorului
        for (corner, a, b) in [
            (0, 1, N),
            (N - 1, N - 2, 2 * N - 1),
            ((N - 1) * N, (N - 2) * N, (N - 1) * N + 1),
            (N * N - 1, N * N - 2, N * N - 1 - N),
        ] {
            if self.c[corner] == you && self.c[a] == me && self.c[b] == me {
                self.c[corner] = 0;
            }
        }
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        let (a, b) = (self.count(Player::One), self.count(Player::Two));
        if b <= 1 && a > 1 {
            return Some(Outcome::Win(Player::One));
        }
        if a <= 1 && b > 1 {
            return Some(Outcome::Win(Player::Two));
        }
        if a <= 1 && b <= 1 {
            return Some(Outcome::Draw);
        }
        if self.plies >= 240 {
            return Some(by_score(a, b));
        }
        if self.legal_moves().is_empty() {
            return Some(Outcome::Win(self.side.other()));
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let mut s = 0;
        for i in 0..N * N {
            let c = self.c[i];
            if c == 0 {
                continue;
            }
            let (r, f) = ((i / N) as i32, (i % N) as i32);
            let centre = 6 - ((2 * r - 7).abs() + (2 * f - 7).abs()) / 2;
            let v = 100 + centre;
            if c == p.idx() as u8 + 1 {
                s += v;
            } else {
                s -= v;
            }
        }
        s
    }

    fn render(&self) -> String {
        let mut s =
            String::from("Ludus latrunculorum — o = J1 (jos), x = J2 (sus)\n   a b c d e f g h\n");
        for r in (0..N).rev() {
            s.push_str(&format!("{}  ", r + 1));
            for f in 0..N {
                s.push(match self.c[r * N + f] {
                    1 => 'o',
                    2 => 'x',
                    _ => '.',
                });
                s.push(' ');
            }
            s.push_str(&format!(" {}\n", r + 1));
        }
        s.push_str(&format!(
            "Piese: J1 {} – J2 {}. Semi-mutări: {}/240.\n",
            self.count(Player::One),
            self.count(Player::Two),
            self.plies
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
        format!(
            "{}-{}",
            sq_name(m.from as usize % N, m.from as usize / N),
            sq_name(m.to as usize % N, m.to as usize / N)
        )
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q5_util::testkit::check_game;

    #[test]
    fn custodial_capture_works() {
        let mut g = Latrunculi::new();
        g.c = [0; N * N];
        g.c[3 * N] = 1; // a4
        g.c[3 * N + 1] = 2; // b4
        g.c[7 * N + 7] = 2;
        g.c[5 * N + 2] = 1; // c6
        g.c[N + 2] = 1; // c2
        g.c[6 * N + 6] = 2;
        // c6 -> c4 cu b4 între a4 și c4? c4 e (3,2): b4 între a4 și c4
        g.apply(&Mv {
            from: (5 * N + 2) as u8,
            to: (3 * N + 2) as u8,
        });
        assert_eq!(g.c[3 * N + 1], 0);
        // intrarea între două piese este sigură
        let mut h = Latrunculi::new();
        h.c = [0; N * N];
        h.c[3 * N] = 2;
        h.c[3 * N + 2] = 2;
        h.c[N + 1] = 1;
        h.c[7 * N] = 1;
        h.c[7 * N + 7] = 2;
        h.apply(&Mv {
            from: (N + 1) as u8,
            to: (3 * N + 1) as u8,
        });
        assert_eq!(h.c[3 * N + 1], 1);
    }

    #[test]
    fn random_and_ai_games() {
        check_game(|_| Latrunculi::new(), 3, 3000);
    }
}
