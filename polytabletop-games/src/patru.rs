//! Patru atribute (variantă de selecție comună): 16 piese distincte, fiecare cu patru
//! însușiri binare (culoare, înălțime, formă, vârf), pe o tablă 4×4. O tură = pui pe tablă
//! piesa pe care ți-a dat-o adversarul, apoi alegi dintre piesele rămase una pe care i-o dai
//! lui (prima tură are doar „dă o piesă"). Cine așază o piesă care completează o linie,
//! coloană sau diagonală de patru piese cu o însușire comună câștigă (nu mai dă piesă).
//! Dacă tabla se umple fără câștigător, remiză.
//!
//! Piesele au numele hexazecimal 0-9, a-f; bitul 0 = culoare (alb/negru), bitul 1 = înălțime
//! (jos/înalt), bitul 2 = formă (rotund/pătrat), bitul 3 = vârf (plin/gol).
//! Verificare surse: regula publică a jocurilor cu piesă oferită de adversar. Implementare proprie.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const NONE: u8 = 255;

const LINES: [[usize; 4]; 10] = [
    [0, 1, 2, 3],
    [4, 5, 6, 7],
    [8, 9, 10, 11],
    [12, 13, 14, 15],
    [0, 4, 8, 12],
    [1, 5, 9, 13],
    [2, 6, 10, 14],
    [3, 7, 11, 15],
    [0, 5, 10, 15],
    [3, 6, 9, 12],
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Mv {
    /// Celula în care se pune piesa primită (None la prima mutare).
    pub cell: Option<u8>,
    /// Piesa dată adversarului (None dacă partida se încheie prin această mutare).
    pub give: Option<u8>,
}

#[derive(Clone)]
pub struct Patru {
    b: [u8; 16],
    avail: u16,
    given: Option<u8>,
    side: Player,
    winner: Option<Player>,
}

impl Default for Patru {
    fn default() -> Self {
        Self::new()
    }
}

fn cell_name(i: usize) -> String {
    polytabletop_core::square_name(i % 4, i / 4)
}

fn hex(p: u8) -> char {
    std::char::from_digit(p as u32, 16).unwrap()
}

fn line_wins(b: &[u8; 16], l: &[usize; 4]) -> bool {
    let (mut a, mut o) = (0xFu8, 0xFu8);
    for &i in l {
        if b[i] == NONE {
            return false;
        }
        a &= b[i];
        o &= !b[i];
    }
    (a | o) & 0xF != 0
}

impl Patru {
    pub fn new() -> Self {
        Patru {
            b: [NONE; 16],
            avail: 0xFFFF,
            given: None,
            side: Player::One,
            winner: None,
        }
    }

    fn wins_at(&self, cell: usize, piece: u8) -> bool {
        let mut b = self.b;
        b[cell] = piece;
        LINES.iter().any(|l| l.contains(&cell) && line_wins(&b, l))
    }

    fn filled(&self) -> usize {
        self.b.iter().filter(|&&x| x != NONE).count()
    }

    fn piece_wins_somewhere(&self, piece: u8) -> bool {
        (0..16).any(|c| self.b[c] == NONE && self.wins_at(c, piece))
    }
}

impl Game for Patru {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if self.winner.is_some() {
            return Vec::new();
        }
        let Some(g) = self.given else {
            if self.filled() == 0 {
                return (0..16u8)
                    .map(|p| Mv {
                        cell: None,
                        give: Some(p),
                    })
                    .collect();
            }
            return Vec::new();
        };
        let mut out = Vec::new();
        let full_after = self.filled() == 15;
        for c in 0..16usize {
            if self.b[c] != NONE {
                continue;
            }
            if full_after || self.wins_at(c, g) {
                out.push(Mv {
                    cell: Some(c as u8),
                    give: None,
                });
            } else {
                for p in 0..16u8 {
                    if self.avail >> p & 1 == 1 {
                        out.push(Mv {
                            cell: Some(c as u8),
                            give: Some(p),
                        });
                    }
                }
            }
        }
        out
    }

    fn apply(&mut self, m: &Mv) {
        if let Some(c) = m.cell {
            let g = self.given.take().unwrap();
            self.b[c as usize] = g;
            if self.wins_at(c as usize, g) {
                self.winner = Some(self.side);
            }
        }
        if let Some(p) = m.give {
            self.given = Some(p);
            self.avail &= !(1 << p);
        }
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        if let Some(w) = self.winner {
            return Some(Outcome::Win(w));
        }
        if self.filled() == 16 {
            return Some(Outcome::Draw);
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        // Cine pune acum are piesa dată; dacă ea câștigă pe loc, mutarea va fi câștigătoare.
        let Some(g) = self.given else { return 0 };
        let mut v = 0;
        if self.piece_wins_somewhere(g) {
            v += 5000;
        } else {
            // Câte piese rămase sunt „sigure" de dat (nu oferă victorie adversarului)?
            let safe = (0..16u8)
                .filter(|&q| self.avail >> q & 1 == 1 && !self.piece_wins_somewhere(q))
                .count() as i32;
            // Pe tablă, paritatea pieselor sigure decide cine rămâne fără: puțin avantaj la impar.
            v += if safe % 2 == 1 { 20 } else { -20 };
        }
        if self.side == p {
            v
        } else {
            -v
        }
    }

    fn render(&self) -> String {
        let mut s = String::from("    a b c d\n");
        for r in (0..4).rev() {
            s.push_str(&format!("{}   ", r + 1));
            for c in 0..4 {
                let p = self.b[r * 4 + c];
                s.push(if p == NONE { '.' } else { hex(p) });
                s.push(' ');
            }
            s.push('\n');
        }
        s.push_str("  piese rămase (de dat): ");
        for p in 0..16u8 {
            if self.avail >> p & 1 == 1 {
                s.push(hex(p));
                s.push(' ');
            }
        }
        s.push('\n');
        match self.given {
            Some(g) => s.push_str(&format!(
                "  de pus acum: {}  (culoare {}, {}, {}, {})\n",
                hex(g),
                ["albă", "neagră"][(g & 1) as usize],
                ["joasă", "înaltă"][(g >> 1 & 1) as usize],
                ["rotundă", "pătrată"][(g >> 2 & 1) as usize],
                ["plină", "goală"][(g >> 3 & 1) as usize],
            )),
            None => s.push_str("  prima mutare: dai o piesă (ex. >5)\n"),
        }
        s.push_str("  mutare: c3>7 = pui piesa primită pe c3 și îi dai adversarului piesa 7; c3 = câștigi/umpli tabla\n");
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        let mut t = String::new();
        if let Some(c) = m.cell {
            t.push_str(&cell_name(c as usize));
        }
        if let Some(p) = m.give {
            t.push('>');
            t.push(hex(p));
        }
        t
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 500,
            max_depth: 6,
        }
    }

    fn hash_key(&self) -> Option<u64> {
        let mut h = polytabletop_core::hash_mix(
            5 << 40
                | (self.side.idx() as u64) << 32
                | (self.given.map_or(31, |g| g as u64)) << 16
                | self.avail as u64,
        );
        for (i, &v) in self.b.iter().enumerate() {
            if v != NONE {
                h ^= polytabletop_core::hash_mix((v as u64 + 1) << 16 | i as u64);
            }
        }
        Some(h)
    }

    fn move_score(&self, m: &Mv) -> i32 {
        if m.give.is_none() && m.cell.is_some() {
            1000
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_move_gives_a_piece() {
        let g = Patru::new();
        let mv = g.legal_moves();
        assert_eq!(mv.len(), 16);
        assert_eq!(g.format_move(&mv[5]), ">5");
    }

    #[test]
    fn shared_attribute_line_wins() {
        // 0,2,4 au bitul 0 = 0 (toate "albe"); a patra piesă albă completează linia.
        let mut g = Patru::new();
        g.b[0] = 0;
        g.b[1] = 2;
        g.b[2] = 4;
        g.given = Some(6);
        g.avail = !(1u16 | 1 << 2 | 1 << 4 | 1 << 6);
        let mv = g.legal_moves();
        let win = mv
            .iter()
            .filter(|m| m.cell == Some(3) && m.give.is_none())
            .count();
        assert_eq!(win, 1);
        // celula 3 nu are alte mutări cu dat piesă
        assert!(mv.iter().all(|m| m.cell != Some(3) || m.give.is_none()));
        let w = *mv.iter().find(|m| m.cell == Some(3)).unwrap();
        g.apply(&w);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn opposite_pieces_do_not_win() {
        let mut g = Patru::new();
        g.b[0] = 0b0000;
        g.b[1] = 0b0001;
        g.b[2] = 0b0010;
        g.b[3] = 0b0011; // bitul 2 și 3 = 0 la toate patru -> câștigă
        assert!(line_wins(&g.b, &LINES[0]));
        g.b[3] = 0b1101;
        g.b[2] = 0b1110;
        g.b[1] = 0b0001;
        g.b[0] = 0b0010;
        assert!(!line_wins(&g.b, &LINES[0]));
    }
}
