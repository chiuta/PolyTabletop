//! "Izolare": 7×7 board, one pawn each. A turn = move your pawn like a chess
//! queen (any distance in a straight or diagonal line, over free squares only),
//! then remove one free square from the board for good. A player who cannot
//! move at the start of their turn loses. No draws.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: usize = 7;
const DIRS: [(i32, i32); 8] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Iz {
    pub to: u8,
    pub rem: u8,
}

#[derive(Clone)]
pub struct Izolare {
    pos: [u8; 2],
    removed: u64,
    side: Player,
}

impl Default for Izolare {
    fn default() -> Self {
        Self::new()
    }
}

impl Izolare {
    pub fn new() -> Self {
        Izolare {
            pos: [3, (N * N - 1 - 3) as u8],
            removed: 0,
            side: Player::One,
        }
    }

    fn free(&self, i: usize) -> bool {
        self.removed >> i & 1 == 0 && self.pos[0] as usize != i && self.pos[1] as usize != i
    }

    /// Squares reachable by a queen move from `from`, with `from` itself treated as vacated
    /// only for the purpose of other squares (never re-entered).
    fn dests(&self, from: usize) -> Vec<u8> {
        let mut v = Vec::new();
        let (r, c) = ((from / N) as i32, (from % N) as i32);
        for (dr, dc) in DIRS {
            let (mut rr, mut cc) = (r + dr, c + dc);
            while (0..N as i32).contains(&rr) && (0..N as i32).contains(&cc) {
                let i = rr as usize * N + cc as usize;
                if !self.free(i) {
                    break;
                }
                v.push(i as u8);
                rr += dr;
                cc += dc;
            }
        }
        v
    }

    fn mobility(&self, p: Player) -> i32 {
        self.dests(self.pos[p.idx()] as usize).len() as i32
    }

    fn sq(i: u8) -> String {
        polytabletop_core::square_name(i as usize % N, i as usize / N)
    }

    pub fn pos(&self, p: Player) -> u8 {
        self.pos[p.idx()]
    }
}

impl Game for Izolare {
    type Move = Iz;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Iz> {
        let me = self.pos[self.side.idx()] as usize;
        let opp = self.pos[self.side.other().idx()] as usize;
        let mut out = Vec::new();
        for to in self.dests(me) {
            for rem in 0..(N * N) {
                if self.removed >> rem & 1 == 1 || rem == opp || rem == to as usize {
                    continue;
                }
                out.push(Iz { to, rem: rem as u8 });
            }
        }
        out.sort_by_key(|m| -self.move_score(m));
        out
    }

    fn apply(&mut self, m: &Iz) {
        self.pos[self.side.idx()] = m.to;
        self.removed |= 1 << m.rem;
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.dests(self.pos[self.side.idx()] as usize).is_empty() {
            Some(Outcome::Win(self.side.other()))
        } else {
            None
        }
    }

    fn evaluate(&self, p: Player) -> i32 {
        10 * (self.mobility(p) - 2 * self.mobility(p.other()))
    }

    fn render(&self) -> String {
        let mut s = String::new();
        for r in (0..N).rev() {
            s.push_str(&format!("{} ", r + 1));
            for c in 0..N {
                let i = r * N + c;
                s.push(if self.pos[0] as usize == i {
                    'X'
                } else if self.pos[1] as usize == i {
                    'O'
                } else if self.removed >> i & 1 == 1 {
                    '#'
                } else {
                    '.'
                });
                s.push(' ');
            }
            s.push('\n');
        }
        s.push_str(
            "  a b c d e f g\n# = pătrat scos. Mutare: d1-d3/e5 (muți ca regina, apoi scoți e5).\n",
        );
        s
    }

    fn format_move(&self, m: &Iz) -> String {
        format!(
            "{}-{}/{}",
            Self::sq(self.pos[self.side.idx()]),
            Self::sq(m.to),
            Self::sq(m.rem)
        )
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 700,
            max_depth: 10,
        }
    }

    fn hash_key(&self) -> Option<u64> {
        let mut h = polytabletop_core::hash_mix(self.removed);
        h ^= polytabletop_core::hash_mix(1 << 40 | (self.pos[0] as u64) << 8 | self.pos[1] as u64);
        h ^= polytabletop_core::hash_mix(1 << 41 | (self.side.idx() as u64));
        Some(h)
    }

    fn move_score(&self, m: &Iz) -> i32 {
        // Prefer removing squares near the opponent, and moving to the centre.
        let opp = self.pos[self.side.other().idx()] as i32;
        let (or, oc) = (opp / N as i32, opp % N as i32);
        let (rr, rc) = (m.rem as i32 / N as i32, m.rem as i32 % N as i32);
        let d = (or - rr).abs().max((oc - rc).abs());
        let (tr, tc) = (m.to as i32 / N as i32, m.to as i32 % N as i32);
        let centre = 6 - ((tr - 3).abs() + (tc - 3).abs());
        (7 - d) * 3 + centre
    }

    fn move_key(&self, m: &Iz) -> Option<u32> {
        Some(m.to as u32 * 64 + m.rem as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    #[test]
    fn start_position() {
        let g = Izolare::new();
        assert_eq!(Izolare::sq(g.pos[0]), "d1");
        assert_eq!(Izolare::sq(g.pos[1]), "d7");
        let l = g.legal_moves();
        assert!(!l.is_empty());
        // d1: up d2..d6 (5), left c1..a1 (3), right e1..g1 (3), diagonals 3 + 3 = 17 (incl. 5 up the file) -> 17 dests
        assert_eq!(g.dests(3).len(), 17);
    }

    #[test]
    fn removed_squares_block_and_cannot_be_removed_twice() {
        let mut g = Izolare::new();
        g.removed |= 1 << (N + 3); // d2 removed
        let d = g.dests(3);
        assert!(!d.iter().any(|&i| i as usize % N == 3));
        for m in g.legal_moves() {
            assert_ne!(g.removed >> m.rem & 1, 1);
            assert_ne!(m.rem, g.pos[1]);
            assert_ne!(m.rem, m.to);
        }
    }

    #[test]
    fn trapped_player_loses() {
        let mut g = Izolare::new();
        // Surround One at d1 (index 3): c1, e1, c2, d2, e2 removed.
        for i in [2usize, 4, 9, 10, 11] {
            g.removed |= 1 << i;
        }
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
        assert!(g.legal_moves().is_empty());
    }

    #[test]
    fn move_text_roundtrip() {
        let g = Izolare::new();
        let t: Vec<String> = g.legal_moves().iter().map(|m| g.format_move(m)).collect();
        let mut u = t.clone();
        u.sort();
        u.dedup();
        assert_eq!(u.len(), t.len());
        assert!(t.contains(&"d1-d3/e5".to_string()));
    }

    #[test]
    fn random_playouts_end() {
        for seed in 1..=30 {
            let mut g = Izolare::new();
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let l = g.legal_moves();
                g.apply(&l[rng.below(l.len())]);
                n += 1;
                assert!(n <= 50);
            }
        }
    }
}
