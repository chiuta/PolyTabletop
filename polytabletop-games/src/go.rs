//! Go on a 9×9 board. Area (Chinese) scoring, komi 7.5, simple ko, no suicide.
//! The game ends after two consecutive passes (or a hard move cap).
//! The AI is Monte-Carlo tree search.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: usize = 9;
const KOMI: f32 = 7.5;
const MOVE_CAP: u16 = 260;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mv {
    Place(u8),
    Pass,
}

#[derive(Clone)]
pub struct Go {
    b: [u8; N * N],    // 0 empty, 1 black (One), 2 white (Two)
    prev: [u8; N * N], // board before the opponent's last move (for ko)
    side: Player,
    passes: u8,
    moves: u16,
    iterations: u32,
}

impl Default for Go {
    fn default() -> Self {
        Self::new()
    }
}

fn neighbours(i: usize) -> impl Iterator<Item = usize> {
    let (r, c) = (i / N, i % N);
    let mut v = [usize::MAX; 4];
    if r > 0 {
        v[0] = i - N;
    }
    if r + 1 < N {
        v[1] = i + N;
    }
    if c > 0 {
        v[2] = i - 1;
    }
    if c + 1 < N {
        v[3] = i + 1;
    }
    v.into_iter().filter(|&x| x != usize::MAX)
}

/// The group containing `start` and its liberty count.
fn group(b: &[u8; N * N], start: usize) -> (Vec<usize>, usize) {
    let color = b[start];
    let mut seen = [false; N * N];
    let mut stack = vec![start];
    let mut stones = Vec::new();
    let mut libs = [false; N * N];
    seen[start] = true;
    while let Some(i) = stack.pop() {
        stones.push(i);
        for n in neighbours(i) {
            if b[n] == 0 {
                libs[n] = true;
            } else if b[n] == color && !seen[n] {
                seen[n] = true;
                stack.push(n);
            }
        }
    }
    let l = libs.iter().filter(|&&x| x).count();
    (stones, l)
}

impl Go {
    pub fn new() -> Self {
        Go {
            b: [0; N * N],
            prev: [0; N * N],
            side: Player::One,
            passes: 0,
            moves: 0,
            iterations: 1500,
        }
    }

    pub fn with_iterations(mut self, n: u32) -> Self {
        self.iterations = n;
        self
    }

    /// Board after `side` plays at `i`, or `None` if the move is illegal
    /// (occupied, suicide, or ko).
    fn try_place(&self, i: usize) -> Option<[u8; N * N]> {
        if self.b[i] != 0 {
            return None;
        }
        let me = self.side.idx() as u8 + 1;
        let opp = 3 - me;
        let mut nb = self.b;
        nb[i] = me;
        for n in neighbours(i) {
            if nb[n] == opp {
                let (stones, libs) = group(&nb, n);
                if libs == 0 {
                    for s in stones {
                        nb[s] = 0;
                    }
                }
            }
        }
        let (_, libs) = group(&nb, i);
        if libs == 0 {
            return None; // suicide
        }
        if nb == self.prev {
            return None; // ko
        }
        Some(nb)
    }

    fn is_own_eye(&self, i: usize) -> bool {
        let me = self.side.idx() as u8 + 1;
        neighbours(i).all(|n| self.b[n] == me)
    }

    /// Area score: (black, white) = stones + surrounded empty points.
    pub fn score(&self) -> (u32, u32) {
        let (mut black, mut white) = (0u32, 0u32);
        let mut seen = [false; N * N];
        for i in 0..N * N {
            match self.b[i] {
                1 => black += 1,
                2 => white += 1,
                _ => {
                    if seen[i] {
                        continue;
                    }
                    let mut stack = vec![i];
                    seen[i] = true;
                    let mut size = 0u32;
                    let (mut tb, mut tw) = (false, false);
                    while let Some(j) = stack.pop() {
                        size += 1;
                        for n in neighbours(j) {
                            match self.b[n] {
                                0 => {
                                    if !seen[n] {
                                        seen[n] = true;
                                        stack.push(n);
                                    }
                                }
                                1 => tb = true,
                                _ => tw = true,
                            }
                        }
                    }
                    if tb && !tw {
                        black += size;
                    } else if tw && !tb {
                        white += size;
                    }
                }
            }
        }
        (black, white)
    }

    fn name(i: usize) -> String {
        format!("{}{}", b"abcdefghj"[i % N] as char, i / N + 1)
    }
}

impl Game for Go {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        let mut v: Vec<Mv> = (0..N * N)
            .filter(|&i| self.try_place(i).is_some())
            .map(|i| Mv::Place(i as u8))
            .collect();
        v.push(Mv::Pass);
        v
    }

    fn apply(&mut self, m: &Mv) {
        match m {
            Mv::Pass => {
                self.passes += 1;
                self.prev = self.b;
            }
            Mv::Place(i) => {
                if let Some(nb) = self.try_place(*i as usize) {
                    self.prev = self.b;
                    self.b = nb;
                }
                self.passes = 0;
            }
        }
        self.moves += 1;
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.passes >= 2 || self.moves >= MOVE_CAP {
            let (b, w) = self.score();
            Some(if b as f32 > w as f32 + KOMI {
                Outcome::Win(Player::One)
            } else {
                Outcome::Win(Player::Two)
            })
        } else {
            None
        }
    }

    fn render(&self) -> String {
        let cols = "a b c d e f g h j";
        let mut s = format!("    {cols}\n");
        for r in (0..N).rev() {
            s.push_str(&format!(" {}  ", r + 1));
            for c in 0..N {
                s.push(match self.b[r * N + c] {
                    1 => 'X',
                    2 => 'O',
                    _ => {
                        let star = [2usize, 4, 6];
                        if star.contains(&r) && star.contains(&c) && (r == 4) == (c == 4) {
                            '+'
                        } else {
                            '.'
                        }
                    }
                });
                s.push(' ');
            }
            s.push_str(&format!(" {}\n", r + 1));
        }
        let (b, w) = self.score();
        s.push_str(&format!(
            "    {cols}\n  X = Negru, O = Alb (komi {KOMI}). Teritoriu+pietre acum: X {b}  O {w}\n"
        ));
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        match m {
            Mv::Pass => "pass".into(),
            Mv::Place(i) => Self::name(*i as usize),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::Mcts(self.iterations)
    }

    fn playout_cap(&self) -> usize {
        140
    }

    fn playout_move(&self, rng: &mut Rng) -> Option<Mv> {
        for _ in 0..24 {
            let i = rng.below(N * N);
            if self.b[i] == 0 && !self.is_own_eye(i) && self.try_place(i).is_some() {
                return Some(Mv::Place(i as u8));
            }
        }
        Some(Mv::Pass)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn play(g: &mut Go, s: &str) {
        let m = g
            .legal_moves()
            .into_iter()
            .find(|m| g.format_move(m) == s)
            .unwrap_or_else(|| panic!("illegal {s}"));
        g.apply(&m);
    }

    #[test]
    fn corner_capture() {
        let mut g = Go::new();
        // White stone at a1 is captured by black at a2 and b1.
        play(&mut g, "a2");
        play(&mut g, "a1");
        play(&mut g, "b1");
        assert_eq!(g.b[0], 0);
    }

    #[test]
    fn suicide_is_illegal() {
        let mut g = Go::new();
        play(&mut g, "b1");
        play(&mut g, "e5");
        play(&mut g, "a2");
        // White at a1 would have no liberties and captures nothing.
        let legal: Vec<String> = g.legal_moves().iter().map(|m| g.format_move(m)).collect();
        assert!(!legal.contains(&"a1".to_string()));
    }

    #[test]
    fn simple_ko_is_forbidden() {
        let mut g = Go::new();
        for m in ["d6", "e6", "d4", "e4", "c5", "f5", "a9", "d5"] {
            play(&mut g, m);
        }
        // Black captures the stone on d5 by playing e5 ...
        play(&mut g, "e5");
        assert_eq!(g.b[4 * N + 3], 0); // d5 is empty again
                                       // ... and White may not immediately recapture on d5 (ko).
        let legal: Vec<String> = g.legal_moves().iter().map(|m| g.format_move(m)).collect();
        assert!(!legal.contains(&"d5".to_string()));
        // After a move elsewhere by each side, the ban is lifted.
        play(&mut g, "a1");
        play(&mut g, "a8");
        let legal: Vec<String> = g.legal_moves().iter().map(|m| g.format_move(m)).collect();
        assert!(legal.contains(&"d5".to_string()));
    }

    #[test]
    fn two_passes_end_the_game_and_white_wins_empty_board_by_komi() {
        let mut g = Go::new();
        g.apply(&Mv::Pass);
        g.apply(&Mv::Pass);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
    }

    #[test]
    fn mcts_returns_a_legal_move() {
        let g = Go::new().with_iterations(200);
        let m = polytabletop_core::ai::best_move(&g, 3).unwrap();
        assert!(g.legal_moves().contains(&m));
    }
}
