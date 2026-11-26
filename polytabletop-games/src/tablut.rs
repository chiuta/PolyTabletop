//! Tablut (Laponia, descris de Linnaeus în 1732): 9×9, 16 atacatori (J1, mută primul),
//! 8 apărători și regele (J2) în castelul central.
//!
//! * Toate piesele mută ca turnul (orizontal/vertical, oricât, fără salturi). Doar regele
//!   se poate opri pe tron (e5); orice piesă poate trece peste tronul gol.
//! * Captura este „în clește”: o piesă adversă între două piese ale tale (regele apărătorilor
//!   contează ca piesă proprie) sau între o piesă a ta și tronul **gol**. Intrarea voluntară
//!   între două piese adverse este sigură. Regele participă la capturi.
//! * Regele e capturat când e înconjurat de atacatori pe patru laturi (pe tron sau lângă el
//!   tronul ține loc de latură; în rest tot patru laturi).
//! * Apărătorii câștigă dacă regele ajunge pe **orice** pătrat de pe margine; atacatorii
//!   câștigă capturând regele. Cine nu mai are mutări pierde; remiză după 300 de semi-mutări.
//!
//! Mutări: `e1-e3`.

use crate::q5_util::{sq_name, verdict};
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: usize = 9;
const THRONE: usize = 40;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Mv {
    pub from: u8,
    pub to: u8,
}

#[derive(Clone)]
pub struct Tablut {
    c: [u8; N * N], // 0 gol, 1 atacator, 2 apărător, 3 rege
    side: Player,
    plies: u16,
    winner: Option<Player>,
}

impl Default for Tablut {
    fn default() -> Self {
        Self::new()
    }
}

fn idx(s: &str) -> usize {
    let b = s.as_bytes();
    ((b[1] - b'1') as usize) * N + (b[0] - b'a') as usize
}

impl Tablut {
    pub fn new() -> Self {
        let mut c = [0u8; N * N];
        for s in [
            "d9", "e9", "f9", "e8", "d1", "e1", "f1", "e2", "a4", "a5", "a6", "b5", "i4", "i5",
            "i6", "h5",
        ] {
            c[idx(s)] = 1;
        }
        for s in ["e3", "e4", "e6", "e7", "c5", "d5", "f5", "g5"] {
            c[idx(s)] = 2;
        }
        c[THRONE] = 3;
        Tablut {
            c,
            side: Player::One,
            plies: 0,
            winner: None,
        }
    }

    fn mine(&self, cell: u8, p: Player) -> bool {
        match p {
            Player::One => cell == 1,
            Player::Two => cell == 2 || cell == 3,
        }
    }

    fn king_pos(&self) -> Option<usize> {
        self.c.iter().position(|&x| x == 3)
    }

    fn ngb(i: usize) -> Vec<(i32, usize)> {
        let (r, f) = ((i / N) as i32, (i % N) as i32);
        let mut v = Vec::new();
        for (k, (dr, df)) in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter().enumerate() {
            let (rr, ff) = (r + dr, f + df);
            if (0..N as i32).contains(&rr) && (0..N as i32).contains(&ff) {
                v.push((k as i32, (rr * N as i32 + ff) as usize));
            }
        }
        v
    }

    pub fn king_open_lines(&self) -> usize {
        let Some(k) = self.king_pos() else { return 0 };
        let (r, f) = ((k / N) as i32, (k % N) as i32);
        let mut n = 0;
        for (dr, df) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let (mut rr, mut ff) = (r + dr, f + df);
            let mut ok = true;
            loop {
                if !(0..N as i32).contains(&rr) || !(0..N as i32).contains(&ff) {
                    break;
                }
                if self.c[(rr * N as i32 + ff) as usize] != 0 {
                    ok = false;
                    break;
                }
                rr += dr;
                ff += df;
            }
            if ok {
                n += 1;
            }
        }
        n
    }
}

impl Game for Tablut {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if self.winner.is_some() || self.plies >= 300 {
            return Vec::new();
        }
        let mut out = Vec::new();
        for i in 0..N * N {
            if !self.mine(self.c[i], self.side) {
                continue;
            }
            let (r0, f0) = ((i / N) as i32, (i % N) as i32);
            for (dr, df) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let (mut r, mut f) = (r0 + dr, f0 + df);
                while (0..N as i32).contains(&r) && (0..N as i32).contains(&f) {
                    let j = (r * N as i32 + f) as usize;
                    if self.c[j] != 0 {
                        break;
                    }
                    if j != THRONE || self.c[i] == 3 {
                        out.push(Mv {
                            from: i as u8,
                            to: j as u8,
                        });
                    }
                    r += dr;
                    f += df;
                }
            }
        }
        out
    }

    fn apply(&mut self, m: &Mv) {
        let (from, to) = (m.from as usize, m.to as usize);
        let me = self.side;
        self.c[to] = self.c[from];
        self.c[from] = 0;
        self.plies += 1;
        let (r, f) = ((to / N) as i32, (to % N) as i32);
        for (dr, df) in [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)] {
            let (nr, nf, or, of) = (r + dr, f + df, r + 2 * dr, f + 2 * df);
            if !(0..N as i32).contains(&nr) || !(0..N as i32).contains(&nf) {
                continue;
            }
            let n = (nr * N as i32 + nf) as usize;
            let victim = self.c[n];
            if victim == 0 || self.mine(victim, me) || victim == 3 {
                continue;
            }
            if !(0..N as i32).contains(&or) || !(0..N as i32).contains(&of) {
                continue;
            }
            let o = (or * N as i32 + of) as usize;
            if self.mine(self.c[o], me) || (o == THRONE && self.c[o] == 0) {
                self.c[n] = 0;
            }
        }
        if me == Player::One {
            // regele capturat?
            if let Some(k) = self.king_pos() {
                let nb = Self::ngb(k);
                let hostile = nb
                    .iter()
                    .filter(|&&(_, j)| self.c[j] == 1 || (j == THRONE && k != THRONE))
                    .count();
                if nb.len() == 4 && hostile == 4 {
                    self.c[k] = 0;
                    self.winner = Some(Player::One);
                }
            }
        } else if self.c[to] == 3 && (r == 0 || f == 0 || r == N as i32 - 1 || f == N as i32 - 1) {
            self.winner = Some(Player::Two);
        }
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        if let Some(w) = self.winner {
            return Some(Outcome::Win(w));
        }
        if self.plies >= 300 {
            return Some(Outcome::Draw);
        }
        if self.legal_moves().is_empty() {
            return Some(Outcome::Win(self.side.other()));
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let att = self.c.iter().filter(|&&x| x == 1).count() as i32;
        let def = self.c.iter().filter(|&&x| x == 2).count() as i32;
        let mut d = def * 160 - att * 100;
        let lines = self.king_open_lines() as i32;
        if let Some(k) = self.king_pos() {
            let (r, f) = ((k / N) as i32, (k % N) as i32);
            let edge = r.min(N as i32 - 1 - r).min(f).min(N as i32 - 1 - f);
            d += (4 - edge) * 12;
            let adj = Self::ngb(k)
                .iter()
                .filter(|&&(_, j)| self.c[j] == 1)
                .count() as i32;
            d -= adj * 25;
        }
        if lines >= 1 && self.side == Player::Two {
            d += 5000;
        } else if lines >= 2 {
            d += 4000;
        } else {
            d += lines * 150;
        }
        if p == Player::Two {
            d
        } else {
            -d
        }
    }

    fn render(&self) -> String {
        let mut s =
            String::from("Tablut — a = atacator (J1), d = apărător (J2), K = rege, + = tron gol\n");
        s.push_str("   a b c d e f g h i\n");
        for r in (0..N).rev() {
            s.push_str(&format!("{}  ", r + 1));
            for f in 0..N {
                let ch = match self.c[r * N + f] {
                    1 => 'a',
                    2 => 'd',
                    3 => 'K',
                    _ if r * N + f == THRONE => '+',
                    _ => '.',
                };
                s.push(ch);
                s.push(' ');
            }
            s.push_str(&format!(" {}\n", r + 1));
        }
        s.push_str("   a b c d e f g h i\n");
        s.push_str(&format!(
            "Atacatori {}, apărători {}, linii libere ale regelui spre margine: {}.\n",
            self.c.iter().filter(|&&x| x == 1).count(),
            self.c.iter().filter(|&&x| x == 2).count(),
            self.king_open_lines()
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

    fn empty() -> Tablut {
        let mut g = Tablut::new();
        g.c = [0; N * N];
        g
    }

    #[test]
    fn setup_counts() {
        let g = Tablut::new();
        assert_eq!(g.c.iter().filter(|&&x| x == 1).count(), 16);
        assert_eq!(g.c.iter().filter(|&&x| x == 2).count(), 8);
        assert_eq!(g.c[THRONE], 3);
    }

    #[test]
    fn custodial_capture_and_throne_rules() {
        let mut g = empty();
        g.c[idx("b2")] = 1;
        g.c[idx("c3")] = 2;
        g.c[idx("e5")] = 3;
        g.c[idx("d9")] = 1;
        g.apply(&Mv {
            from: idx("d9") as u8,
            to: idx("d3") as u8,
        });
        assert_eq!(g.c[idx("c3")], 2); // un singur atacator lângă: nu e captură
        let mut h = empty();
        h.c[idx("a3")] = 1;
        h.c[idx("b3")] = 2;
        h.c[idx("e9")] = 1;
        h.c[idx("i1")] = 3;
        h.c[idx("c8")] = 1;
        h.apply(&Mv {
            from: idx("c8") as u8,
            to: idx("c3") as u8,
        });
        assert_eq!(h.c[idx("b3")], 0);
        // piesa de lângă tronul gol e capturată cu un singur atacator
        let mut t = empty();
        t.c[idx("e4")] = 2;
        t.c[idx("h3")] = 1;
        t.c[idx("a9")] = 3;
        t.side = Player::One;
        t.apply(&Mv {
            from: idx("h3") as u8,
            to: idx("e3") as u8,
        });
        assert_eq!(t.c[idx("e4")], 0);
        // doar regele se oprește pe tron
        let g2 = Tablut::new();
        assert!(g2.legal_moves().iter().all(|m| m.to as usize != THRONE));
    }

    #[test]
    fn king_escape_and_capture() {
        let mut g = empty();
        g.c[idx("c5")] = 3;
        g.c[idx("a1")] = 1;
        g.side = Player::Two;
        g.apply(&Mv {
            from: idx("c5") as u8,
            to: idx("a5") as u8,
        });
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
        let mut h = empty();
        h.c[idx("c5")] = 3;
        h.c[idx("c6")] = 1;
        h.c[idx("c4")] = 1;
        h.c[idx("b5")] = 1;
        h.c[idx("i5")] = 1;
        h.apply(&Mv {
            from: idx("i5") as u8,
            to: idx("d5") as u8,
        });
        assert_eq!(h.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn random_and_ai_games() {
        check_game(|_| Tablut::new(), 3, 3000);
    }
}
