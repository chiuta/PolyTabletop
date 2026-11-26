//! Hex 11×11 (fără regula de schimb): doi jucători unesc laturi opuse ale unui
//! romb de hexagoane. Nu există remiză. X (J1) unește sus–jos (rândurile 1 și 11),
//! O (J2) unește stânga–dreapta (coloanele a și k). X începe.

use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::collections::VecDeque;

const N: usize = 11;
const NB: [(i32, i32); 6] = [(-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0)];

#[derive(Clone)]
pub struct Hex {
    cells: [u8; N * N],
    side: Player,
    count: u16,
    winner: Option<Player>,
}

impl Default for Hex {
    fn default() -> Self {
        Self::new()
    }
}

fn neighbours(i: usize) -> impl Iterator<Item = usize> {
    let (r, c) = ((i / N) as i32, (i % N) as i32);
    NB.iter().filter_map(move |&(dr, dc)| {
        let (rr, cc) = (r + dr, c + dc);
        if rr >= 0 && rr < N as i32 && cc >= 0 && cc < N as i32 {
            Some(rr as usize * N + cc as usize)
        } else {
            None
        }
    })
}

impl Hex {
    pub fn new() -> Self {
        Hex {
            cells: [0; N * N],
            side: Player::One,
            count: 0,
            winner: None,
        }
    }

    fn is_start(p: u8, i: usize) -> bool {
        if p == 1 {
            i / N == 0
        } else {
            i.is_multiple_of(N)
        }
    }

    fn is_goal(p: u8, i: usize) -> bool {
        if p == 1 {
            i / N == N - 1
        } else {
            i % N == N - 1
        }
    }

    fn connected(&self, p: u8) -> bool {
        let mut seen = [false; N * N];
        let mut stack: Vec<usize> = Vec::new();
        for (i, flag) in seen.iter_mut().enumerate() {
            if self.cells[i] == p && Self::is_start(p, i) {
                *flag = true;
                stack.push(i);
            }
        }
        while let Some(i) = stack.pop() {
            if Self::is_goal(p, i) {
                return true;
            }
            for j in neighbours(i) {
                if !seen[j] && self.cells[j] == p {
                    seen[j] = true;
                    stack.push(j);
                }
            }
        }
        false
    }

    /// Cheapest number of empty cells still to fill for `p` to connect (0-1 BFS).
    fn distance(&self, p: u8) -> i32 {
        const INF: i32 = 1000;
        let mut dist = [INF; N * N];
        let mut dq: VecDeque<usize> = VecDeque::new();
        let cost = |v: u8| -> i32 {
            if v == p {
                0
            } else if v == 0 {
                1
            } else {
                INF
            }
        };
        #[allow(clippy::needless_range_loop)]
        for i in 0..N * N {
            if Self::is_start(p, i) {
                let c = cost(self.cells[i]);
                if c < INF {
                    dist[i] = c;
                    if c == 0 {
                        dq.push_front(i);
                    } else {
                        dq.push_back(i);
                    }
                }
            }
        }
        while let Some(i) = dq.pop_front() {
            let d = dist[i];
            for j in neighbours(i) {
                let c = cost(self.cells[j]);
                if c >= INF {
                    continue;
                }
                if d + c < dist[j] {
                    dist[j] = d + c;
                    if c == 0 {
                        dq.push_front(j);
                    } else {
                        dq.push_back(j);
                    }
                }
            }
        }
        (0..N * N)
            .filter(|&i| Self::is_goal(p, i))
            .map(|i| dist[i])
            .min()
            .unwrap_or(INF)
    }

    fn name(i: usize) -> String {
        format!("{}{}", (b'a' + (i % N) as u8) as char, i / N + 1)
    }
}

impl Game for Hex {
    type Move = u8;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<u8> {
        if self.winner.is_some() {
            return Vec::new();
        }
        (0..N * N)
            .filter(|&i| self.cells[i] == 0)
            .map(|i| i as u8)
            .collect()
    }

    fn apply(&mut self, m: &u8) {
        let p = self.side.idx() as u8 + 1;
        self.cells[*m as usize] = p;
        self.count += 1;
        if self.count as usize >= 2 * N - 1 && self.connected(p) {
            self.winner = Some(self.side);
        }
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        self.winner.map(Outcome::Win)
    }

    fn evaluate(&self, p: Player) -> i32 {
        if let Some(w) = self.winner {
            return if w == p { 1_000_000 } else { -1_000_000 };
        }
        let me = p.idx() as u8 + 1;
        let you = 3 - me;
        let (dm, dy) = (self.distance(me), self.distance(you));
        let mut s = (dy - dm) * 100;
        // The side to move gains a tempo.
        s += if self.side == p { 30 } else { -30 };
        s
    }

    fn render(&self) -> String {
        let mut s = String::from("   ");
        for c in 0..N {
            s.push(' ');
            s.push((b'a' + c as u8) as char);
        }
        s.push_str("   (O)\n");
        for r in 0..N {
            s.push_str(&format!("{:>2} ", r + 1));
            s.push_str(&" ".repeat(r));
            for c in 0..N {
                s.push(' ');
                s.push(match self.cells[r * N + c] {
                    1 => 'X',
                    2 => 'O',
                    _ => '.',
                });
            }
            s.push('\n');
        }
        s.push_str("X (J1): sus–jos, O (J2): stânga–dreapta\n");
        s
    }

    fn format_move(&self, m: &u8) -> String {
        Self::name(*m as usize)
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 600,
            max_depth: 4,
        }
    }

    fn playout_cap(&self) -> usize {
        N * N
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

    fn move_score(&self, m: &u8) -> i32 {
        let (r, c) = ((*m as usize / N) as i32, (*m as usize % N) as i32);
        let d = (r - 5).abs().max((c - 5).abs());
        10 - d
    }

    fn move_key(&self, m: &u8) -> Option<u32> {
        Some(*m as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    fn play(g: &mut Hex, s: &str) {
        let m = g
            .legal_moves()
            .into_iter()
            .find(|m| g.format_move(m) == s)
            .unwrap_or_else(|| panic!("ilegal {s}"));
        g.apply(&m);
    }

    #[test]
    fn opening_has_121_moves() {
        let g = Hex::new();
        assert_eq!(g.legal_moves().len(), 121);
        assert_eq!(g.format_move(&0), "a1");
        assert_eq!(g.format_move(&120), "k11");
    }

    #[test]
    fn top_bottom_chain_wins_for_x() {
        let mut g = Hex::new();
        // Column a, rows 1..11: a straight chain of neighbours (r,c)-(r+1,c).
        for r in 1..=11 {
            play(&mut g, &format!("a{r}"));
            if r < 11 {
                play(&mut g, &format!("c{r}"));
            }
        }
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn left_right_chain_wins_for_o() {
        let mut g = Hex::new();
        for c in b'a'..=b'k' {
            play(&mut g, &format!("{}3", c as char));
            play(&mut g, &format!("{}7", c as char));
        }
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
    }

    #[test]
    fn adjacency_is_hexagonal() {
        // Interior cell has 6 neighbours, corners 2 or 3.
        assert_eq!(neighbours(5 * N + 5).count(), 6);
        assert_eq!(neighbours(0).count(), 2);
        assert_eq!(neighbours(N - 1).count(), 3);
        assert_eq!(neighbours(N * (N - 1)).count(), 3);
        assert_eq!(neighbours(N * N - 1).count(), 2);
    }

    #[test]
    fn random_playouts_always_end_with_winner() {
        for seed in 1..=30u64 {
            let mut g = Hex::new();
            let mut rng = Rng::new(seed);
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
            }
            assert!(matches!(g.outcome(), Some(Outcome::Win(_))));
        }
    }

    #[test]
    fn evaluate_prefers_shorter_path() {
        let mut g = Hex::new();
        for r in 1..=5 {
            play(&mut g, &format!("f{r}"));
            play(&mut g, &format!("a{r}"));
        }
        assert!(g.evaluate(Player::One) > g.evaluate(Player::Two));
        assert!(g.distance(1) < g.distance(2));
    }
}
