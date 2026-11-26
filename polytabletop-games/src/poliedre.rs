//! Poliedre (duo): tablă 14×14, doi jucători, fiecare cu cele 21 de piese poliomino
//! (de la 1 la 5 pătrate: 1+1+2+5+12 piese). Prima piesă a fiecărui jucător acoperă
//! pătratul lui de start (e5 pentru J1, j10 pentru J2). Fiecare piesă următoare trebuie să
//! atingă o piesă proprie prin colț și nu are voie să atingă nicio piesă proprie pe latură
//! (piesele adverse pot fi atinse liber). Cine nu mai poate pune trece; partida se termină
//! când ambii trec la rând. Câștigă cine are mai puține pătrate rămase în piesele nefolosite;
//! egalitate = remiză.
//!
//! Piesele se notează cu litere a-u (în ordinea mărimii), orientările cu 1-8: 1-4 rotirile
//! cu câte 90° în sens orar ale formei desenate, 5-8 aceleași rotiri ale formei oglindite
//! (orientările identice cu una anterioară se omit). Celula mutării = colțul stânga-sus al
//! dreptunghiului în care încape piesa. Mutare: k3@e5, trecere: pas.
//!
//! Verificare surse: regulile publice ale jocurilor de plasare a poliominourilor în doi.
//! Piesele sunt generate (poliominourile libere). Implementare proprie.

use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::collections::BTreeSet;
use std::sync::OnceLock;

const S: usize = 14;
const N: usize = S * S;
const PASS: u8 = 255;
const START: [usize; 2] = [4 * S + 4, 9 * S + 9];

type Shape = Vec<(i8, i8)>;

struct Piece {
    size: u8,
    orients: Vec<Shape>,
    distinct: [bool; 8],
}

fn normalize(mut v: Shape) -> Shape {
    let (mr, mc) = (
        v.iter().map(|p| p.0).min().unwrap(),
        v.iter().map(|p| p.1).min().unwrap(),
    );
    for p in &mut v {
        p.0 -= mr;
        p.1 -= mc;
    }
    v.sort();
    v
}

fn transform(s: &Shape, o: usize) -> Shape {
    let mut v: Shape = s.clone();
    if o >= 4 {
        for p in &mut v {
            p.1 = -p.1;
        }
    }
    for _ in 0..(o % 4) {
        for p in &mut v {
            *p = (p.1, -p.0);
        }
    }
    normalize(v)
}

fn pieces() -> &'static Vec<Piece> {
    static P: OnceLock<Vec<Piece>> = OnceLock::new();
    P.get_or_init(|| {
        let canon = |s: &Shape| -> Shape { (0..8).map(|o| transform(s, o)).min().unwrap() };
        let mut levels: Vec<BTreeSet<Shape>> = vec![BTreeSet::new(); 6];
        levels[1].insert(vec![(0, 0)]);
        for n in 2..=5 {
            let prev: Vec<Shape> = levels[n - 1].iter().cloned().collect();
            for s in prev {
                for &(r, c) in &s {
                    for (dr, dc) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                        let q = (r + dr, c + dc);
                        if !s.contains(&q) {
                            let mut t = s.clone();
                            t.push(q);
                            levels[n].insert(canon(&normalize(t)));
                        }
                    }
                }
            }
        }
        let mut out = Vec::new();
        for lv in levels.iter().skip(1) {
            for s in lv {
                let orients: Vec<Shape> = (0..8).map(|o| transform(s, o)).collect();
                let mut distinct = [false; 8];
                for o in 0..8 {
                    distinct[o] = !orients[..o].contains(&orients[o]);
                }
                out.push(Piece {
                    size: s.len() as u8,
                    orients,
                    distinct,
                });
            }
        }
        out
    })
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Mv {
    /// Litera piesei (0..21), sau 255 = trecere.
    pub piece: u8,
    pub orient: u8,
    pub r: u8,
    pub c: u8,
}

#[derive(Clone)]
pub struct Poliedre {
    b: [u8; N],
    used: [u32; 2],
    side: Player,
    passes: u8,
}

impl Default for Poliedre {
    fn default() -> Self {
        Self::new()
    }
}

fn cname(i: usize) -> String {
    polytabletop_core::square_name(i % S, i / S)
}

impl Poliedre {
    pub fn new() -> Self {
        assert_eq!(pieces().len(), 21);
        Poliedre {
            b: [0; N],
            used: [0, 0],
            side: Player::One,
            passes: 0,
        }
    }

    fn left(&self, p: usize) -> i32 {
        pieces()
            .iter()
            .enumerate()
            .filter(|(i, _)| self.used[p] >> i & 1 == 0)
            .map(|(_, pc)| pc.size as i32)
            .sum()
    }

    fn edge_free(&self, r: i32, c: i32, who: u8) -> bool {
        for (dr, dc) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let (rr, cc) = (r + dr, c + dc);
            if rr >= 0
                && cc >= 0
                && rr < S as i32
                && cc < S as i32
                && self.b[rr as usize * S + cc as usize] == who
            {
                return false;
            }
        }
        true
    }

    fn fits(&self, shape: &Shape, ar: i32, ac: i32, who: u8) -> bool {
        shape.iter().all(|&(dr, dc)| {
            let (r, c) = (ar + dr as i32, ac + dc as i32);
            r >= 0
                && c >= 0
                && r < S as i32
                && c < S as i32
                && self.b[r as usize * S + c as usize] == 0
                && self.edge_free(r, c, who)
        })
    }

    /// Celule libere atinse în colț de o piesă proprie și nu pe latură.
    fn corners(&self, who: u8) -> Vec<usize> {
        let mut v = Vec::new();
        for i in 0..N {
            if self.b[i] != 0 {
                continue;
            }
            let (r, c) = ((i / S) as i32, (i % S) as i32);
            if !self.edge_free(r, c, who) {
                continue;
            }
            let diag = [(1, 1), (1, -1), (-1, 1), (-1, -1)]
                .iter()
                .any(|&(dr, dc)| {
                    let (rr, cc) = (r + dr, c + dc);
                    rr >= 0
                        && cc >= 0
                        && rr < S as i32
                        && cc < S as i32
                        && self.b[rr as usize * S + cc as usize] == who
                });
            if diag {
                v.push(i);
            }
        }
        v
    }

    fn gen_placements(&self, p: usize) -> Vec<Mv> {
        let who = p as u8 + 1;
        let cands: Vec<usize> = if self.used[p] == 0 {
            vec![START[p]]
        } else {
            self.corners(who)
        };
        let mut out = Vec::new();
        for &cell in &cands {
            let (cr, cc) = ((cell / S) as i32, (cell % S) as i32);
            for (pi, pc) in pieces().iter().enumerate() {
                if self.used[p] >> pi & 1 == 1 {
                    continue;
                }
                for o in 0..8 {
                    if !pc.distinct[o] {
                        continue;
                    }
                    let shape = &pc.orients[o];
                    for &(kr, kc) in shape {
                        let (ar, ac) = (cr - kr as i32, cc - kc as i32);
                        if self.fits(shape, ar, ac, who) {
                            out.push(Mv {
                                piece: pi as u8,
                                orient: o as u8,
                                r: ar as u8,
                                c: ac as u8,
                            });
                        }
                    }
                }
            }
        }
        out.sort_by_key(|m| (m.piece, m.orient, m.r, m.c));
        out.dedup();
        out
    }
}

impl Game for Poliedre {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if self.passes >= 2 {
            return Vec::new();
        }
        let v = self.gen_placements(self.side.idx());
        if v.is_empty() {
            vec![Mv {
                piece: PASS,
                orient: 0,
                r: 0,
                c: 0,
            }]
        } else {
            v
        }
    }

    fn apply(&mut self, m: &Mv) {
        if m.piece == PASS {
            self.passes += 1;
        } else {
            let who = self.side.idx() as u8 + 1;
            let shape = &pieces()[m.piece as usize].orients[m.orient as usize];
            for &(dr, dc) in shape {
                let (r, c) = (m.r as i32 + dr as i32, m.c as i32 + dc as i32);
                self.b[r as usize * S + c as usize] = who;
            }
            self.used[self.side.idx()] |= 1 << m.piece;
            self.passes = 0;
        }
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.passes < 2 {
            return None;
        }
        let (a, b) = (self.left(0), self.left(1));
        Some(match a.cmp(&b) {
            std::cmp::Ordering::Less => Outcome::Win(Player::One),
            std::cmp::Ordering::Greater => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let sq = self.left(1 - me) - self.left(me);
        let mob = |q: usize| -> i32 {
            if self.used[q] == 0 {
                1
            } else {
                self.corners(q as u8 + 1).len() as i32
            }
        };
        10 * sq + 2 * (mob(me) - mob(1 - me))
    }

    fn render(&self) -> String {
        let mut s = String::from("    a b c d e f g h i j k l m n\n");
        for r in (0..S).rev() {
            s.push_str(&format!("{:>2}  ", r + 1));
            for c in 0..S {
                let i = r * S + c;
                s.push(match self.b[i] {
                    1 => 'X',
                    2 => 'O',
                    _ if i == START[0] || i == START[1] => '+',
                    _ => '.',
                });
                s.push(' ');
            }
            s.push('\n');
        }
        let who = self.side.idx();
        s.push_str(&format!(
            "  pătrate rămase: X {}  O {}   (câștigă cine are mai puține)\n",
            self.left(0),
            self.left(1)
        ));
        s.push_str(&format!(
            "  piese rămase pentru {}:\n",
            if who == 0 { "X (J1)" } else { "O (J2)" }
        ));
        let rest: Vec<usize> = (0..21).filter(|&i| self.used[who] >> i & 1 == 0).collect();
        for chunk in rest.chunks(7) {
            for row in 0..3 {
                s.push_str("   ");
                for &pi in chunk {
                    let sh = &pieces()[pi].orients[0];
                    for col in 0..5 {
                        let on = sh.contains(&(row as i8, col as i8));
                        s.push(if on { '#' } else { ' ' });
                    }
                    s.push_str("  ");
                }
                s.push('\n');
            }
            s.push_str("   ");
            for &pi in chunk {
                s.push_str(&format!("{:<7}", (b'a' + pi as u8) as char));
            }
            s.push('\n');
        }
        s.push_str("  mutare: k3@e5 (piesa k, orientarea 3, colțul stânga-sus la e5) sau pas\n");
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        if m.piece == PASS {
            "pas".to_string()
        } else {
            format!(
                "{}{}@{}",
                (b'a' + m.piece) as char,
                m.orient + 1,
                cname(m.r as usize * S + m.c as usize)
            )
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 800,
            max_depth: 3,
        }
    }

    fn hash_key(&self) -> Option<u64> {
        let mut h = polytabletop_core::hash_mix(
            11 << 40
                | (self.side.idx() as u64) << 32
                | (self.passes as u64) << 28
                | self.used[0] as u64,
        );
        h ^= polytabletop_core::hash_mix(12 << 40 | self.used[1] as u64);
        for (i, &v) in self.b.iter().enumerate() {
            if v != 0 {
                h ^= polytabletop_core::hash_mix((v as u64) << 16 | i as u64);
            }
        }
        Some(h)
    }

    fn move_score(&self, m: &Mv) -> i32 {
        if m.piece == PASS {
            0
        } else {
            pieces()[m.piece as usize].size as i32
        }
    }

    fn playout_cap(&self) -> usize {
        200
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn twenty_one_pieces() {
        let p = pieces();
        assert_eq!(p.len(), 21);
        let by = |n: u8| p.iter().filter(|x| x.size == n).count();
        assert_eq!((by(1), by(2), by(3), by(4), by(5)), (1, 1, 2, 5, 12));
        assert_eq!(p.iter().map(|x| x.size as u32).sum::<u32>(), 89);
    }

    #[test]
    fn first_move_covers_start_square() {
        let g = Poliedre::new();
        let mv = g.legal_moves();
        assert!(!mv.is_empty());
        for m in &mv {
            let sh = &pieces()[m.piece as usize].orients[m.orient as usize];
            assert!(sh.iter().any(|&(dr, dc)| (m.r as usize + dr as usize) * S
                + m.c as usize
                + dc as usize
                == START[0]));
        }
    }

    #[test]
    fn corner_touch_only() {
        let mut g = Poliedre::new();
        // pune monomino pe e5 (piesa 0)
        let m = *g
            .legal_moves()
            .iter()
            .find(|m| m.piece == 0)
            .expect("monomino");
        g.apply(&m);
        g.side = Player::One; // forțăm tot J1 pentru verificare
        let mv = g.legal_moves();
        for m in &mv {
            let sh = &pieces()[m.piece as usize].orients[m.orient as usize];
            let cells: Vec<usize> = sh
                .iter()
                .map(|&(dr, dc)| (m.r as usize + dr as usize) * S + m.c as usize + dc as usize)
                .collect();
            assert!(!cells.contains(&(START[0] + 1)) && !cells.contains(&(START[0] + S)));
            assert!(
                cells.contains(&(START[0] + S + 1))
                    || cells.contains(&(START[0] + S - 1))
                    || cells.contains(&(START[0] - S + 1))
                    || cells.contains(&(START[0] - S - 1))
            );
        }
    }

    #[test]
    fn double_pass_ends() {
        let mut g = Poliedre::new();
        g.used = [(1 << 21) - 1, (1 << 21) - 1];
        assert_eq!(g.legal_moves().len(), 1);
        let p = g.legal_moves()[0];
        g.apply(&p);
        g.apply(&p);
        assert_eq!(g.outcome(), Some(Outcome::Draw));
    }
}
