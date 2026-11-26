#![allow(clippy::needless_range_loop, clippy::manual_range_contains)]
//! Shatranj (Persia) — strămoșul șahului, 8×8.
//!
//! * Piese: Rege (un pas în orice direcție), Ferz (un pas pe diagonală), Alfil („elefant”: sare
//!   exact două pătrate pe diagonală, peste orice piesă), Cal, Turn și Pion (un pas înainte, captură
//!   pe diagonală, fără pas dublu, se promovează **obligatoriu în Ferz** pe ultimul rând). Fără
//!   rocadă și fără en passant.
//! * Câștigi prin **mat**, prin **pat** (în șahul persan cel pătat pierde), sau lăsând adversarul
//!   cu **regele gol** (fără alte piese), exceptând cazul în care el poate captura imediat ultima ta
//!   piesă (atunci remiză). După 300 de semi-mutări remiza e declarată.
//! * Mutări: `e2e3`, `b1c3` (fără sufix de promovare).

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const CAP: u32 = 300;

// Cod: 0 gol; 1 Pion, 2 Ferz, 3 Alfil, 4 Cal, 5 Turn, 6 Rege; +8 pentru J2.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Move {
    pub from: u8,
    pub to: u8,
}

#[derive(Clone)]
pub struct Shatranj {
    b: [u8; 64],
    side: Player,
    ply: u32,
}

impl Default for Shatranj {
    fn default() -> Self {
        Self::new()
    }
}

fn owner(c: u8) -> Option<Player> {
    match c {
        1..=6 => Some(Player::One),
        9..=14 => Some(Player::Two),
        _ => None,
    }
}

fn kind(c: u8) -> u8 {
    c & 7
}

fn sq(i: usize) -> String {
    format!("{}{}", (b'a' + (i % 8) as u8) as char, i / 8 + 1)
}

const VAL: [i32; 7] = [0, 100, 200, 200, 400, 600, 0];
const KNIGHT: [(i32, i32); 8] = [
    (1, 2),
    (2, 1),
    (-1, 2),
    (-2, 1),
    (1, -2),
    (2, -1),
    (-1, -2),
    (-2, -1),
];
const DIAG: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
const ORTH: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

impl Shatranj {
    pub fn new() -> Self {
        let mut b = [0u8; 64];
        let back = [5u8, 4, 3, 2, 6, 3, 4, 5];
        for f in 0..8 {
            b[f] = back[f];
            b[8 + f] = 1;
            b[48 + f] = 9;
            b[56 + f] = back[f] + 8;
        }
        Shatranj {
            b,
            side: Player::One,
            ply: 0,
        }
    }

    fn at(r: i32, f: i32) -> Option<usize> {
        if r < 0 || f < 0 || r > 7 || f > 7 {
            None
        } else {
            Some((r * 8 + f) as usize)
        }
    }

    /// Mutări pseudo-legale ale lui `p`.
    fn pseudo(&self, p: Player) -> Vec<Move> {
        let mut v = Vec::new();
        for i in 0..64 {
            let c = self.b[i];
            if owner(c) != Some(p) {
                continue;
            }
            let (r, f) = ((i / 8) as i32, (i % 8) as i32);
            let push = |to: usize, v: &mut Vec<Move>, b: &[u8; 64]| {
                if owner(b[to]) != Some(p) {
                    v.push(Move {
                        from: i as u8,
                        to: to as u8,
                    });
                }
            };
            match kind(c) {
                1 => {
                    let dr = if p == Player::One { 1 } else { -1 };
                    if let Some(t) = Self::at(r + dr, f) {
                        if self.b[t] == 0 {
                            v.push(Move {
                                from: i as u8,
                                to: t as u8,
                            });
                        }
                    }
                    for df in [-1, 1] {
                        if let Some(t) = Self::at(r + dr, f + df) {
                            if owner(self.b[t]) == Some(p.other()) {
                                v.push(Move {
                                    from: i as u8,
                                    to: t as u8,
                                });
                            }
                        }
                    }
                }
                2 | 6 => {
                    let dirs: Vec<(i32, i32)> = if kind(c) == 2 {
                        DIAG.to_vec()
                    } else {
                        DIAG.iter().chain(ORTH.iter()).copied().collect()
                    };
                    for (dr, df) in dirs {
                        if let Some(t) = Self::at(r + dr, f + df) {
                            push(t, &mut v, &self.b);
                        }
                    }
                }
                3 => {
                    for (dr, df) in DIAG {
                        if let Some(t) = Self::at(r + 2 * dr, f + 2 * df) {
                            push(t, &mut v, &self.b);
                        }
                    }
                }
                4 => {
                    for (dr, df) in KNIGHT {
                        if let Some(t) = Self::at(r + dr, f + df) {
                            push(t, &mut v, &self.b);
                        }
                    }
                }
                5 => {
                    for (dr, df) in ORTH {
                        let (mut rr, mut ff) = (r + dr, f + df);
                        while let Some(t) = Self::at(rr, ff) {
                            push(t, &mut v, &self.b);
                            if self.b[t] != 0 {
                                break;
                            }
                            rr += dr;
                            ff += df;
                        }
                    }
                }
                _ => {}
            }
        }
        v
    }

    fn king_sq(&self, p: Player) -> Option<usize> {
        let k = if p == Player::One { 6 } else { 14 };
        self.b.iter().position(|&c| c == k)
    }

    fn in_check(&self, p: Player) -> bool {
        match self.king_sq(p) {
            None => true,
            Some(k) => self.pseudo(p.other()).iter().any(|m| m.to as usize == k),
        }
    }

    fn make(&self, m: &Move) -> Shatranj {
        let mut g = self.clone();
        let mut c = g.b[m.from as usize];
        g.b[m.from as usize] = 0;
        if kind(c) == 1 {
            let last = if self.side == Player::One { 7 } else { 0 };
            if m.to as usize / 8 == last {
                c = 2 + if self.side == Player::One { 0 } else { 8 };
            }
        }
        g.b[m.to as usize] = c;
        g.side = self.side.other();
        g.ply += 1;
        g
    }

    fn legal_for(&self, p: Player) -> Vec<Move> {
        let mut me = self.clone();
        me.side = p;
        self.pseudo(p)
            .into_iter()
            .filter(|m| !me.make(m).in_check(p))
            .collect()
    }

    fn pieces(&self, p: Player) -> usize {
        self.b.iter().filter(|&&c| owner(c) == Some(p)).count()
    }
}

impl Game for Shatranj {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        self.legal_for(self.side)
    }

    fn apply(&mut self, m: &Move) {
        *self = self.make(m);
    }

    fn outcome(&self) -> Option<Outcome> {
        let me = self.side;
        let op = me.other();
        // rege gol
        if self.pieces(me) == 1 && self.pieces(op) > 1 {
            // poate captura ultima piesă care nu e regele adversarului? remiză dacă îl lasă gol
            let saves = self.legal_for(me).iter().any(|m| {
                let n = self.make(m);
                n.pieces(op) == 1
            });
            return Some(if saves {
                Outcome::Draw
            } else {
                Outcome::Win(op)
            });
        }
        if self.pieces(me) == 1 && self.pieces(op) == 1 {
            return Some(Outcome::Draw);
        }
        if self.legal_for(me).is_empty() {
            // mat sau pat: în ambele cazuri pierde cel la mutare
            return Some(Outcome::Win(op));
        }
        if self.ply >= CAP {
            return Some(Outcome::Draw);
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let mut s = 0;
        for i in 0..64 {
            let c = self.b[i];
            let Some(o) = owner(c) else { continue };
            let k = kind(c) as usize;
            let mut v = VAL[k];
            if k == 1 {
                let adv = if o == Player::One { i / 8 } else { 7 - i / 8 } as i32;
                v += adv * 12;
            }
            let center =
                3 - ((i % 8) as i32 * 2 - 7).abs() / 2 + 3 - ((i / 8) as i32 * 2 - 7).abs() / 2;
            v += center * 2;
            if o == p {
                s += v;
            } else {
                s -= v;
            }
        }
        s
    }

    fn render(&self) -> String {
        let sym = [".", "P", "F", "A", "N", "R", "K"];
        let mut s = String::from("    a b c d e f g h\n");
        for r in (0..8).rev() {
            s.push_str(&format!(" {}  ", r + 1));
            for f in 0..8 {
                let c = self.b[r * 8 + f];
                let t = sym[kind(c) as usize];
                s.push_str(&if owner(c) == Some(Player::Two) {
                    t.to_lowercase()
                } else {
                    t.to_string()
                });
                s.push(' ');
            }
            s.push_str(&format!(" {}\n", r + 1));
        }
        s.push_str("    a b c d e f g h\n  MAJUSCULE = J1 (jos), minuscule = J2; P pion, F ferz, A alfil, N cal, R turn, K rege\n");
        if self.in_check(self.side) && self.outcome().is_none() {
            s.push_str("Șah!\n");
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        format!("{}{}", sq(m.from as usize), sq(m.to as usize))
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(3)
    }

    fn move_score(&self, m: &Move) -> i32 {
        let c = self.b[m.to as usize];
        if c != 0 {
            VAL[kind(c) as usize] - VAL[kind(self.b[m.from as usize]) as usize] / 10 + 1000
        } else {
            0
        }
    }

    fn is_tactical(&self, m: &Move) -> bool {
        self.b[m.to as usize] != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    #[test]
    fn start_moves() {
        let g = Shatranj::new();
        // 8 pioni + 4 mutări de cal + 4 sărituri de alfil (peste pioni, pe rândul 3 liber)
        assert_eq!(g.legal_moves().len(), 16);
    }

    #[test]
    fn alfil_jumps_two_diagonally() {
        let mut g = Shatranj::new();
        g.b = [0; 64];
        g.b[4] = 6; // Re1
        g.b[60] = 14; // Re8
        g.b[2 * 8 + 2] = 3; // alfil c3
        g.b[3 * 8 + 3] = 9; // pion negru pe d4 între
        let mv = g.legal_moves();
        let to: Vec<usize> = mv
            .iter()
            .filter(|m| m.from == 18)
            .map(|m| m.to as usize)
            .collect();
        assert!(to.contains(&(4 * 8 + 4))); // e5
        assert!(to.contains(&0)); // a1
        assert!(!to.contains(&4)); // e1 e propriul rege
    }

    #[test]
    fn pawn_promotes_to_ferz_and_stalemate_loses() {
        let mut g = Shatranj::new();
        g.b = [0; 64];
        g.b[0] = 6; // Ra1
        g.b[63] = 14; // Rh8
        g.b[6 * 8 + 3] = 1; // pion d7
        g.apply(&Move { from: 51, to: 59 });
        assert_eq!(g.b[59], 2);
        // pat: Rege negru h8 blocat -> pierde cel la mutare
        let mut h = Shatranj::new();
        h.b = [0; 64];
        h.b[63] = 14; // Rh8
        h.b[5 * 8 + 6] = 2; // ferz g6 (nu dă șah) controlează g7,h7,f7 ...
        h.b[5 * 8 + 5] = 6; // Rf6 controlează g7
        h.b[5 * 8 + 7] = 5; // turn h6 controlează h7
        h.side = Player::Two;
        h.b[7 * 8 + 5] = 0;
        // regele negru pe h8: g8 liber? turn h6 nu controlează g8; ferz g6 nu; => are mutare; adaugă turn g1
        h.b[6] = 5; // turn g1 controlează g8
        assert_eq!(h.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=4u64 {
            let mut g = Shatranj::new();
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() && n < 1000 {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
            }
            assert!(g.outcome().is_some());
        }
    }
}
