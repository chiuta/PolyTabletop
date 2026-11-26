//! "Puncte și căsuțe": 6×6 dots (5×5 boxes). Players alternately draw one edge;
//! whoever closes the fourth side of a box takes it and moves again.
//! Most boxes wins (25 boxes, so no draws). Public-domain pencil-and-paper game.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const B: usize = 5; // boxes per side
const D: usize = B + 1; // dots per side
const NH: usize = D * B; // horizontal edges: dot row r (0..6), column c (0..5)
const NE: usize = NH * 2;

#[derive(Clone)]
pub struct Puncte {
    edges: u64, // bit i set = edge drawn
    owner: [u8; B * B],
    score: [u8; 2],
    side: Player,
}

impl Default for Puncte {
    fn default() -> Self {
        Self::new()
    }
}

fn h(r: usize, c: usize) -> usize {
    r * B + c
}
fn v(r: usize, c: usize) -> usize {
    NH + r * D + c
}

impl Puncte {
    pub fn new() -> Self {
        Puncte {
            edges: 0,
            owner: [0; B * B],
            score: [0; 2],
            side: Player::One,
        }
    }

    fn has(&self, e: usize) -> bool {
        self.edges >> e & 1 == 1
    }

    fn box_edges(r: usize, c: usize) -> [usize; 4] {
        [h(r, c), h(r + 1, c), v(r, c), v(r, c + 1)]
    }

    fn sides(&self, r: usize, c: usize) -> usize {
        Self::box_edges(r, c)
            .iter()
            .filter(|&&e| self.has(e))
            .count()
    }

    /// Boxes touching edge `e`.
    fn adjacent(e: usize) -> Vec<(usize, usize)> {
        let mut out = Vec::with_capacity(2);
        if e < NH {
            let (r, c) = (e / B, e % B);
            if r > 0 {
                out.push((r - 1, c));
            }
            if r < B {
                out.push((r, c));
            }
        } else {
            let (r, c) = ((e - NH) / D, (e - NH) % D);
            if c > 0 {
                out.push((r, c - 1));
            }
            if c < B {
                out.push((r, c));
            }
        }
        out
    }

    fn third_side_count(&self) -> i32 {
        let mut n = 0;
        for r in 0..B {
            for c in 0..B {
                if self.owner[r * B + c] == 0 && self.sides(r, c) == 3 {
                    n += 1;
                }
            }
        }
        n
    }

    pub fn score(&self, p: Player) -> u8 {
        self.score[p.idx()]
    }
}

impl Game for Puncte {
    type Move = u8;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<u8> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        let mut mv: Vec<u8> = (0..NE as u8).filter(|&e| !self.has(e as usize)).collect();
        mv.sort_by_key(|m| -self.move_score(m));
        mv
    }

    fn apply(&mut self, m: &u8) {
        let e = *m as usize;
        self.edges |= 1 << e;
        let mut got = 0;
        for (r, c) in Self::adjacent(e) {
            if self.owner[r * B + c] == 0 && self.sides(r, c) == 4 {
                self.owner[r * B + c] = self.side.idx() as u8 + 1;
                self.score[self.side.idx()] += 1;
                got += 1;
            }
        }
        if got == 0 {
            self.side = self.side.other();
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.score[0] + self.score[1] < (B * B) as u8 {
            return None;
        }
        Some(match self.score[0].cmp(&self.score[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let diff = self.score[p.idx()] as i32 - self.score[p.other().idx()] as i32;
        let open = self.third_side_count() * 40;
        100 * diff + if self.side == p { open } else { -open }
    }

    fn render(&self) -> String {
        let mut s = String::from("    a   b   c   d   e   f\n");
        for r in (0..D).rev() {
            s.push_str(&format!("{}   ", r + 1));
            for c in 0..D {
                s.push('+');
                if c < B {
                    s.push_str(if self.has(h(r, c)) { "---" } else { "   " });
                }
            }
            s.push('\n');
            if r > 0 {
                s.push_str("    ");
                for c in 0..D {
                    s.push(if self.has(v(r - 1, c)) { '|' } else { ' ' });
                    if c < B {
                        let o = self.owner[(r - 1) * B + c];
                        s.push_str(match o {
                            1 => " X ",
                            2 => " O ",
                            _ => "   ",
                        });
                    }
                }
                s.push('\n');
            }
        }
        s.push_str(&format!(
            "Căsuțe: X (J1) {}  –  O (J2) {}\n",
            self.score[0], self.score[1]
        ));
        s
    }

    fn format_move(&self, m: &u8) -> String {
        let e = *m as usize;
        if e < NH {
            format!("o{}{}", (b'a' + (e % B) as u8) as char, e / B + 1)
        } else {
            let k = e - NH;
            format!("v{}{}", (b'a' + (k % D) as u8) as char, k / D + 1)
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 600,
            max_depth: 24,
        }
    }

    fn hash_key(&self) -> Option<u64> {
        let a = polytabletop_core::hash_mix(self.edges);
        let b = polytabletop_core::hash_mix(
            1 << 50
                | (self.side.idx() as u64) << 20
                | (self.score[0] as u64) << 8
                | self.score[1] as u64,
        );
        Some(a ^ b.rotate_left(17))
    }

    fn move_score(&self, m: &u8) -> i32 {
        let mut s = 0;
        for (r, c) in Self::adjacent(*m as usize) {
            match self.sides(r, c) {
                3 => s += 100,
                2 => s -= 50,
                _ => {}
            }
        }
        s
    }

    fn move_key(&self, m: &u8) -> Option<u32> {
        Some(*m as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    fn mv(g: &Puncte, t: &str) -> u8 {
        *g.legal_moves()
            .iter()
            .find(|m| g.format_move(m) == t)
            .unwrap_or_else(|| panic!("no move {t}"))
    }

    #[test]
    fn box_capture_keeps_turn_and_scores() {
        let mut g = Puncte::new();
        for t in ["oa1", "ob6", "oa2", "ob5", "va1"] {
            let m = mv(&g, t);
            g.apply(&m);
        }
        assert_eq!(g.to_move(), Player::Two);
        let m = mv(&g, "vb1");
        g.apply(&m);
        assert_eq!(g.score(Player::Two), 1);
        assert_eq!(g.to_move(), Player::Two);
        assert!(g.render().contains(" O "));
    }

    #[test]
    fn one_edge_can_close_two_boxes() {
        let mut g = Puncte::new();
        // boxes a1 and b1 share edge vb1. Draw all their other edges.
        let seq = ["oa1", "oa2", "va1", "ob1", "ob2", "vc1"];
        let mut who = vec![];
        for t in seq {
            let m = mv(&g, t);
            who.push(g.to_move());
            g.apply(&m);
        }
        let before = g.to_move();
        let m = mv(&g, "vb1");
        g.apply(&m);
        assert_eq!(g.score(before), 2);
        assert_eq!(g.to_move(), before);
    }

    #[test]
    fn random_playouts_end_with_25_boxes() {
        for seed in 1..=20 {
            let mut g = Puncte::new();
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let l = g.legal_moves();
                assert!(!l.is_empty());
                g.apply(&l[rng.below(l.len())]);
                n += 1;
                assert!(n <= NE);
            }
            assert_eq!(n, NE);
            assert_eq!(g.score[0] + g.score[1], 25);
            assert!(matches!(g.outcome(), Some(Outcome::Win(_))));
        }
    }

    #[test]
    fn move_texts_unique() {
        let g = Puncte::new();
        let mut t: Vec<String> = g.legal_moves().iter().map(|m| g.format_move(m)).collect();
        assert_eq!(t.len(), 60);
        t.sort();
        t.dedup();
        assert_eq!(t.len(), 60);
    }
}
