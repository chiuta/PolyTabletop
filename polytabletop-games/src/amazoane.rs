//! Amazoane (10×10): fiecare jucător are patru amazoane care se mișcă ca o regină
//! de șah, apoi trag o săgeată (tot ca o regină) care blochează definitiv
//! pătratul ales. Pierde cel care nu mai poate muta. J1 (X) începe.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: usize = 10;
const DIRS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

/// Mutare: (de la, la, săgeată), toate ca indici 0..100 (rând*10 + coloană).
pub type AmazonMove = (u8, u8, u8);

#[derive(Clone)]
pub struct Amazoane {
    /// 0 gol, 1 amazoană J1, 2 amazoană J2, 3 săgeată.
    cells: [u8; N * N],
    side: Player,
    plies: u16,
}

impl Default for Amazoane {
    fn default() -> Self {
        Self::new()
    }
}

fn step(i: usize, d: (i32, i32)) -> Option<usize> {
    let (r, c) = ((i / N) as i32 + d.0, (i % N) as i32 + d.1);
    if r >= 0 && r < N as i32 && c >= 0 && c < N as i32 {
        Some(r as usize * N + c as usize)
    } else {
        None
    }
}

impl Amazoane {
    pub fn new() -> Self {
        let mut cells = [0u8; N * N];
        // Rând 0 = „1”. J1: a4 d1 g1 j4; J2: a7 d10 g10 j7.
        for (r, c) in [(3, 0), (0, 3), (0, 6), (3, 9)] {
            cells[r * N + c] = 1;
        }
        for (r, c) in [(6, 0), (9, 3), (9, 6), (6, 9)] {
            cells[r * N + c] = 2;
        }
        Amazoane {
            cells,
            side: Player::One,
            plies: 0,
        }
    }

    fn name(i: usize) -> String {
        format!("{}{}", (b'a' + (i % N) as u8) as char, i / N + 1)
    }

    fn has_move_for(&self, who: u8) -> bool {
        (0..N * N).any(|i| {
            self.cells[i] == who
                && DIRS
                    .iter()
                    .any(|&d| step(i, d).is_some_and(|j| self.cells[j] == 0))
        })
    }

    /// Slides from `from` in each direction over empty squares; `vacated` counts as empty.
    fn slides(&self, from: usize, vacated: usize, mut f: impl FnMut(usize)) {
        for &d in &DIRS {
            let mut cur = from;
            while let Some(j) = step(cur, d) {
                if self.cells[j] != 0 && j != vacated {
                    break;
                }
                f(j);
                cur = j;
            }
        }
    }

    /// Queen-move distance (in moves) from the amazons of `who` to every square.
    fn queen_dist(&self, who: u8) -> [u8; N * N] {
        const FAR: u8 = 255;
        let mut dist = [FAR; N * N];
        let mut frontier: Vec<usize> = (0..N * N).filter(|&i| self.cells[i] == who).collect();
        for &i in &frontier {
            dist[i] = 0;
        }
        let mut level = 0u8;
        while !frontier.is_empty() {
            level += 1;
            let mut next = Vec::new();
            for &i in &frontier {
                for &d in &DIRS {
                    let mut cur = i;
                    while let Some(j) = step(cur, d) {
                        if self.cells[j] != 0 {
                            break;
                        }
                        if dist[j] == FAR {
                            dist[j] = level;
                            next.push(j);
                        }
                        cur = j;
                    }
                }
            }
            frontier = next;
        }
        dist
    }
}

impl Game for Amazoane {
    type Move = AmazonMove;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<AmazonMove> {
        let who = self.side.idx() as u8 + 1;
        let mut out = Vec::new();
        for from in 0..N * N {
            if self.cells[from] != who {
                continue;
            }
            let mut dests = Vec::new();
            self.slides(from, from, |j| dests.push(j));
            for to in dests {
                self.slides(to, from, |a| out.push((from as u8, to as u8, a as u8)));
            }
        }
        out
    }

    fn apply(&mut self, m: &AmazonMove) {
        let who = self.cells[m.0 as usize];
        self.cells[m.0 as usize] = 0;
        self.cells[m.1 as usize] = who;
        self.cells[m.2 as usize] = 3;
        self.plies += 1;
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        let who = self.side.idx() as u8 + 1;
        if self.has_move_for(who) {
            None
        } else {
            Some(Outcome::Win(self.side.other()))
        }
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx() as u8 + 1;
        let you = 3 - me;
        if !self.has_move_for(self.side.idx() as u8 + 1) {
            return if self.side == p {
                -1_000_000
            } else {
                1_000_000
            };
        }
        let (dm, dy) = (self.queen_dist(me), self.queen_dist(you));
        let mut terr = 0i32;
        let mut near = 0i32;
        for i in 0..N * N {
            if self.cells[i] != 0 {
                continue;
            }
            let (a, b) = (dm[i], dy[i]);
            if a < b {
                terr += 1;
            } else if b < a {
                terr -= 1;
            }
            if a == 1 {
                near += 1;
            }
            if b == 1 {
                near -= 1;
            }
        }
        let tempo = if self.side == p { 3 } else { -3 };
        terr * 12 + near * 2 + tempo
    }

    fn render(&self) -> String {
        let hdr = "    a b c d e f g h i j\n";
        let mut s = String::from(hdr);
        for r in (0..N).rev() {
            s.push_str(&format!("{:>2}  ", r + 1));
            for c in 0..N {
                s.push(match self.cells[r * N + c] {
                    1 => 'X',
                    2 => 'O',
                    3 => '#',
                    _ => '.',
                });
                s.push(' ');
            }
            s.push_str(&format!(" {}\n", r + 1));
        }
        s.push_str(hdr);
        s.push_str("X = J1, O = J2, # = săgeată\n");
        s
    }

    fn format_move(&self, m: &AmazonMove) -> String {
        format!(
            "{}-{}/{}",
            Self::name(m.0 as usize),
            Self::name(m.1 as usize),
            Self::name(m.2 as usize)
        )
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 800,
            max_depth: 3,
        }
    }

    fn playout_cap(&self) -> usize {
        100
    }

    fn hash_key(&self) -> Option<u64> {
        let mut h = polytabletop_core::hash_mix(1 << 40 | (self.side.idx() as u64) << 32);
        for (i, &v) in self.cells.iter().enumerate() {
            if v != 0 {
                h ^= polytabletop_core::hash_mix((v as u64) << 16 | i as u64);
            }
        }
        Some(h)
    }

    fn move_score(&self, m: &AmazonMove) -> i32 {
        // Prefer shooting next to enemy amazons and moving toward open space.
        let who = self.side.idx() as u8 + 1;
        let enemy = 3 - who;
        let mut s = 0;
        for &d in &DIRS {
            if let Some(j) = step(m.2 as usize, d) {
                if self.cells[j] == enemy {
                    s += 3;
                }
            }
            if let Some(j) = step(m.1 as usize, d) {
                if self.cells[j] == 0 && j != m.0 as usize && j != m.2 as usize {
                    s += 1;
                }
            }
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    #[test]
    fn opening_move_count_is_2176() {
        let g = Amazoane::new();
        assert_eq!(g.legal_moves().len(), 2176);
    }

    #[test]
    fn moves_are_unique_and_formatted() {
        let g = Amazoane::new();
        let mut names: Vec<String> = g.legal_moves().iter().map(|m| g.format_move(m)).collect();
        names.sort();
        let n = names.len();
        names.dedup();
        assert_eq!(names.len(), n);
        assert!(names.contains(&"d1-d5/d8".to_string()));
    }

    #[test]
    fn arrow_may_land_on_vacated_square() {
        let g = Amazoane::new();
        let mv = g.legal_moves();
        // d1 -> d5, arrow back to d1.
        assert!(mv.iter().any(|m| g.format_move(m) == "d1-d5/d1"));
    }

    #[test]
    fn cannot_jump_over_arrows_or_amazons() {
        let mut g = Amazoane::new();
        g.apply(&(3, 13, 23)); // d1-d2/d3 (arrow on d3)
        g.side = Player::One;
        let mv = g.legal_moves();
        // d2 cannot slide up through the arrow on d3.
        assert!(!mv.iter().any(|m| m.0 == 13 && m.1 >= 33 && m.1 % 10 == 3));
    }

    #[test]
    fn stuck_player_loses() {
        let mut g = Amazoane::new();
        g.cells = [3; N * N];
        g.cells[0] = 1; // a1, boxed in by arrows
        g.cells[99] = 2; // j10 has a free neighbour
        g.cells[98] = 0;
        g.side = Player::One;
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
        g.side = Player::Two;
        assert_eq!(g.outcome(), None);
    }

    #[test]
    fn random_playouts_end_and_match_outcome() {
        for seed in 1..=10u64 {
            let mut g = Amazoane::new();
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
                assert!(n <= 92, "too long");
            }
            assert!(g.legal_moves().is_empty());
        }
    }
}
