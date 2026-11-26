//! Moara (Nine Men's Morris / Țintar): 24 points, 9 pieces each.
//! Phase 1: place pieces. Phase 2: slide to an adjacent point.
//! With exactly 3 pieces left a player may "fly" anywhere.
//! Forming a mill (three in a line) removes an opposing piece (not in a mill,
//! unless all of them are). Fewer than 3 pieces, or no legal move, loses.

use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::sync::OnceLock;

const NAMES: [&str; 24] = [
    "a1", "d1", "g1", "b2", "d2", "f2", "c3", "d3", "e3", "a4", "b4", "c4", "e4", "f4", "g4", "c5",
    "d5", "e5", "b6", "d6", "f6", "a7", "d7", "g7",
];

const MILL_NAMES: [[&str; 3]; 16] = [
    ["a1", "d1", "g1"],
    ["b2", "d2", "f2"],
    ["c3", "d3", "e3"],
    ["a4", "b4", "c4"],
    ["e4", "f4", "g4"],
    ["c5", "d5", "e5"],
    ["b6", "d6", "f6"],
    ["a7", "d7", "g7"],
    ["a1", "a4", "a7"],
    ["b2", "b4", "b6"],
    ["c3", "c4", "c5"],
    ["d1", "d2", "d3"],
    ["d5", "d6", "d7"],
    ["e3", "e4", "e5"],
    ["f2", "f4", "f6"],
    ["g1", "g4", "g7"],
];

struct Tables {
    mills: Vec<[usize; 3]>,
    adj: Vec<Vec<usize>>,
}

fn idx(name: &str) -> usize {
    NAMES.iter().position(|&n| n == name).expect("point name")
}

fn tables() -> &'static Tables {
    static T: OnceLock<Tables> = OnceLock::new();
    T.get_or_init(|| {
        let mills: Vec<[usize; 3]> = MILL_NAMES
            .iter()
            .map(|m| [idx(m[0]), idx(m[1]), idx(m[2])])
            .collect();
        let mut adj = vec![Vec::new(); 24];
        for m in &mills {
            for w in [(m[0], m[1]), (m[1], m[2])] {
                if !adj[w.0].contains(&w.1) {
                    adj[w.0].push(w.1);
                }
                if !adj[w.1].contains(&w.0) {
                    adj[w.1].push(w.0);
                }
            }
        }
        Tables { mills, adj }
    })
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Mv {
    pub from: Option<u8>,
    pub to: u8,
    pub take: Option<u8>,
}

#[derive(Clone)]
pub struct Morris {
    b: [u8; 24],
    hand: [u8; 2],
    side: Player,
    quiet: u16,
}

impl Default for Morris {
    fn default() -> Self {
        Self::new()
    }
}

impl Morris {
    pub fn new() -> Self {
        Morris {
            b: [0; 24],
            hand: [9, 9],
            side: Player::One,
            quiet: 0,
        }
    }

    fn on_board(&self, color: u8) -> usize {
        self.b.iter().filter(|&&c| c == color).count()
    }

    fn total(&self, p: Player) -> usize {
        self.on_board(p.idx() as u8 + 1) + self.hand[p.idx()] as usize
    }

    fn in_mill(b: &[u8; 24], i: usize) -> bool {
        let c = b[i];
        c != 0
            && tables()
                .mills
                .iter()
                .any(|m| m.contains(&i) && m.iter().all(|&k| b[k] == c))
    }

    fn removable(&self, b: &[u8; 24], victim: u8) -> Vec<u8> {
        let all: Vec<usize> = (0..24).filter(|&i| b[i] == victim).collect();
        let free: Vec<usize> = all
            .iter()
            .copied()
            .filter(|&i| !Self::in_mill(b, i))
            .collect();
        let pick = if free.is_empty() { all } else { free };
        pick.into_iter().map(|i| i as u8).collect()
    }

    fn gen(&self) -> Vec<Mv> {
        let me = self.side.idx() as u8 + 1;
        let opp = 3 - me;
        let t = tables();
        let mut base: Vec<(Option<u8>, u8)> = Vec::new();
        if self.hand[self.side.idx()] > 0 {
            for i in 0..24 {
                if self.b[i] == 0 {
                    base.push((None, i as u8));
                }
            }
        } else {
            let fly = self.on_board(me) == 3;
            for from in 0..24 {
                if self.b[from] != me {
                    continue;
                }
                if fly {
                    for to in 0..24 {
                        if self.b[to] == 0 {
                            base.push((Some(from as u8), to as u8));
                        }
                    }
                } else {
                    for &to in &t.adj[from] {
                        if self.b[to] == 0 {
                            base.push((Some(from as u8), to as u8));
                        }
                    }
                }
            }
        }
        let mut out = Vec::new();
        for (from, to) in base {
            let mut nb = self.b;
            if let Some(f) = from {
                nb[f as usize] = 0;
            }
            nb[to as usize] = me;
            if Self::in_mill(&nb, to as usize) {
                for take in self.removable(&nb, opp) {
                    out.push(Mv {
                        from,
                        to,
                        take: Some(take),
                    });
                }
            } else {
                out.push(Mv {
                    from,
                    to,
                    take: None,
                });
            }
        }
        out
    }

    fn mobility(&self, color: u8) -> i32 {
        let t = tables();
        (0..24)
            .filter(|&i| self.b[i] == color && t.adj[i].iter().any(|&j| self.b[j] == 0))
            .count() as i32
    }
}

impl Game for Morris {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        self.gen()
    }

    fn apply(&mut self, m: &Mv) {
        let me = self.side.idx() as u8 + 1;
        match m.from {
            Some(f) => self.b[f as usize] = 0,
            None => self.hand[self.side.idx()] -= 1,
        }
        self.b[m.to as usize] = me;
        if let Some(t) = m.take {
            self.b[t as usize] = 0;
            self.quiet = 0;
        } else {
            self.quiet += 1;
        }
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        for p in [Player::One, Player::Two] {
            if self.total(p) < 3 {
                return Some(Outcome::Win(p.other()));
            }
        }
        if self.hand[0] == 0 && self.hand[1] == 0 && self.quiet >= 100 {
            return Some(Outcome::Draw);
        }
        if self.hand[self.side.idx()] == 0 && self.gen().is_empty() {
            return Some(Outcome::Win(self.side.other()));
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx() as u8 + 1;
        let pieces = self.total(p) as i32 - self.total(p.other()) as i32;
        let mob = self.mobility(me) - self.mobility(3 - me);
        100 * pieces + 10 * mob
    }

    fn render(&self) -> String {
        // Grid: file a..g -> x = 6*file, rank 7..1 -> y = 2*(7-rank).
        let (w, h) = (37usize, 13usize);
        let mut g = vec![vec![' '; w]; h];
        let pos = |name: &str| -> (usize, usize) {
            let f = (name.as_bytes()[0] - b'a') as usize;
            let r = (name.as_bytes()[1] - b'0') as usize;
            (6 * f, 2 * (7 - r))
        };
        for m in MILL_NAMES.iter() {
            for pair in [(m[0], m[1]), (m[1], m[2])] {
                let (x1, y1) = pos(pair.0);
                let (x2, y2) = pos(pair.1);
                if y1 == y2 {
                    for cell in &mut g[y1][x1.min(x2) + 1..x1.max(x2)] {
                        *cell = '-';
                    }
                } else {
                    for row in &mut g[y1.min(y2) + 1..y1.max(y2)] {
                        row[x1] = '|';
                    }
                }
            }
        }
        for (i, n) in NAMES.iter().enumerate() {
            let (x, y) = pos(n);
            g[y][x] = match self.b[i] {
                1 => 'X',
                2 => 'O',
                _ => '+',
            };
        }
        let mut s = String::from("    a     b     c     d     e     f     g\n");
        for (y, row) in g.iter().enumerate() {
            let label = if y % 2 == 0 {
                format!("{} ", 7 - y / 2)
            } else {
                "  ".to_string()
            };
            s.push_str(&label);
            s.push_str(&row.iter().collect::<String>());
            s.push('\n');
        }
        s.push_str(&format!(
            "  În mână: X {}  O {}   Pe tablă: X {}  O {}\n",
            self.hand[0],
            self.hand[1],
            self.on_board(1),
            self.on_board(2)
        ));
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        let mut s = match m.from {
            Some(f) => format!("{}-{}", NAMES[f as usize], NAMES[m.to as usize]),
            None => NAMES[m.to as usize].to_string(),
        };
        if let Some(t) = m.take {
            s.push('x');
            s.push_str(NAMES[t as usize]);
        }
        s
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 500,
            max_depth: 10,
        }
    }

    fn hash_key(&self) -> Option<u64> {
        // Zobrist-style key computed on demand: XOR of one mixed value per piece.
        let mut h = polytabletop_core::hash_mix(
            1 << 40
                | (self.side.idx() as u64) << 32
                | (self.quiet as u64)
                | (self.hand[0] as u64) << 8
                | (self.hand[1] as u64) << 16,
        );
        for (i, &v) in self.b.iter().enumerate() {
            if v != 0 {
                h ^= polytabletop_core::hash_mix((v as u64) << 16 | i as u64);
            }
        }
        Some(h)
    }

    fn move_score(&self, m: &Mv) -> i32 {
        if m.take.is_some() {
            100
        } else {
            0
        }
    }

    fn move_key(&self, m: &Mv) -> Option<u32> {
        Some(m.from.map_or(24, |f| f as u32) * 32 + m.to as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn play(g: &mut Morris, s: &str) {
        let m = g
            .legal_moves()
            .into_iter()
            .find(|m| g.format_move(m) == s)
            .unwrap_or_else(|| panic!("illegal {s}"));
        g.apply(&m);
    }

    #[test]
    fn graph_is_standard() {
        let t = tables();
        assert_eq!(t.mills.len(), 16);
        let edges: usize = t.adj.iter().map(|a| a.len()).sum::<usize>() / 2;
        assert_eq!(edges, 32);
        assert_eq!(t.adj[idx("d2")].len(), 4);
        assert_eq!(t.adj[idx("a1")].len(), 2);
    }

    #[test]
    fn mill_removes_a_piece() {
        let mut g = Morris::new();
        for m in ["a1", "b2", "d1", "b4", "g1"] {
            // One completes a1-d1-g1 on the third piece; the move needs a take.
            if m == "g1" {
                let g1: Vec<String> = g
                    .legal_moves()
                    .iter()
                    .map(|x| g.format_move(x))
                    .filter(|t| t.starts_with("g1"))
                    .collect();
                assert!(!g1.is_empty() && g1.iter().all(|t| t.contains('x')));
                play(&mut g, "g1xb2");
            } else {
                play(&mut g, m);
            }
        }
        assert_eq!(g.on_board(2), 1);
        assert_eq!(g.hand[1], 7);
    }
}
