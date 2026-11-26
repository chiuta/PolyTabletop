//! Dame / English draughts (checkers): 8×8, men move and capture forward,
//! kings in all four diagonals, capture is mandatory, multi-jumps continue,
//! a man that reaches the far row becomes a king and the move ends.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

// Cell codes: 0 empty, 1 man One, 2 king One, 3 man Two, 4 king Two.

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Mv {
    pub path: Vec<u8>,
}

#[derive(Clone)]
pub struct Draughts {
    b: [u8; 64],
    side: Player,
    quiet: u16,
}

impl Default for Draughts {
    fn default() -> Self {
        Self::new()
    }
}

fn owner(c: u8) -> Option<Player> {
    match c {
        1 | 2 => Some(Player::One),
        3 | 4 => Some(Player::Two),
        _ => None,
    }
}

fn is_king(c: u8) -> bool {
    c == 2 || c == 4
}

fn sq(i: usize) -> String {
    format!("{}{}", (b'a' + (i % 8) as u8) as char, i / 8 + 1)
}

impl Draughts {
    pub fn new() -> Self {
        let mut b = [0u8; 64];
        for r in 0..8usize {
            for f in 0..8usize {
                if (f + r) % 2 == 0 {
                    if r < 3 {
                        b[r * 8 + f] = 1;
                    } else if r > 4 {
                        b[r * 8 + f] = 3;
                    }
                }
            }
        }
        Draughts {
            b,
            side: Player::One,
            quiet: 0,
        }
    }

    fn dirs(piece: u8) -> &'static [(i32, i32)] {
        match piece {
            1 => &[(1, -1), (1, 1)],
            3 => &[(-1, -1), (-1, 1)],
            _ => &[(1, -1), (1, 1), (-1, -1), (-1, 1)],
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn jumps(
        &self,
        board: &[u8; 64],
        at: usize,
        piece: u8,
        me: Player,
        path: &mut Vec<u8>,
        captured: &mut Vec<usize>,
        out: &mut Vec<Mv>,
    ) {
        let mut extended = false;
        for &(dr, df) in Self::dirs(piece) {
            let (r, f) = ((at / 8) as i32, (at % 8) as i32);
            let (mr, mf, lr, lf) = (r + dr, f + df, r + 2 * dr, f + 2 * df);
            if !(0..=7).contains(&lr) || !(0..=7).contains(&lf) {
                continue;
            }
            let mid = (mr * 8 + mf) as usize;
            let land = (lr * 8 + lf) as usize;
            let mc = board[mid];
            if owner(mc) == Some(me.other()) && !captured.contains(&mid) && board[land] == 0 {
                extended = true;
                path.push(land as u8);
                captured.push(mid);
                let promotes = !is_king(piece)
                    && ((me == Player::One && lr == 7) || (me == Player::Two && lr == 0));
                if promotes {
                    out.push(Mv { path: path.clone() });
                } else {
                    self.jumps(board, land, piece, me, path, captured, out);
                }
                captured.pop();
                path.pop();
            }
        }
        if !extended && path.len() > 1 {
            out.push(Mv { path: path.clone() });
        }
    }

    fn gen(&self) -> Vec<Mv> {
        let me = self.side;
        let mut jumps = Vec::new();
        for i in 0..64 {
            let p = self.b[i];
            if owner(p) != Some(me) {
                continue;
            }
            let mut board = self.b;
            board[i] = 0; // the moving piece leaves its square
            let mut path = vec![i as u8];
            let mut cap = Vec::new();
            self.jumps(&board, i, p, me, &mut path, &mut cap, &mut jumps);
        }
        if !jumps.is_empty() {
            return jumps;
        }
        let mut steps = Vec::new();
        for i in 0..64 {
            let p = self.b[i];
            if owner(p) != Some(me) {
                continue;
            }
            for &(dr, df) in Self::dirs(p) {
                let (r, f) = ((i / 8) as i32 + dr, (i % 8) as i32 + df);
                if !(0..=7).contains(&r) || !(0..=7).contains(&f) {
                    continue;
                }
                let to = (r * 8 + f) as usize;
                if self.b[to] == 0 {
                    steps.push(Mv {
                        path: vec![i as u8, to as u8],
                    });
                }
            }
        }
        steps
    }

    pub fn counts(&self) -> (usize, usize) {
        let one = self
            .b
            .iter()
            .filter(|&&c| owner(c) == Some(Player::One))
            .count();
        let two = self
            .b
            .iter()
            .filter(|&&c| owner(c) == Some(Player::Two))
            .count();
        (one, two)
    }
}

impl Game for Draughts {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if self.quiet >= 80 {
            return Vec::new();
        }
        self.gen()
    }

    fn apply(&mut self, m: &Mv) {
        let start = m.path[0] as usize;
        let mut piece = self.b[start];
        let was_man = !is_king(piece);
        self.b[start] = 0;
        let mut captured_any = false;
        for w in m.path.windows(2) {
            let (a, b) = (w[0] as usize, w[1] as usize);
            let (ar, br) = (a / 8, b / 8);
            if ar.abs_diff(br) == 2 {
                let mid = ((ar + br) / 2) * 8 + (a % 8 + b % 8) / 2;
                self.b[mid] = 0;
                captured_any = true;
            }
        }
        let end = *m.path.last().unwrap() as usize;
        if was_man
            && ((self.side == Player::One && end / 8 == 7)
                || (self.side == Player::Two && end / 8 == 0))
        {
            piece += 1; // man -> king (1->2, 3->4)
        }
        self.b[end] = piece;
        if captured_any || was_man {
            self.quiet = 0;
        } else {
            self.quiet += 1;
        }
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.quiet >= 80 {
            return Some(Outcome::Draw);
        }
        if self.gen().is_empty() {
            return Some(Outcome::Win(self.side.other()));
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let mut s = 0;
        for i in 0..64 {
            let c = self.b[i];
            let Some(o) = owner(c) else { continue };
            let rank = i / 8;
            let progress = if o == Player::One { rank } else { 7 - rank } as i32;
            let v = if is_king(c) { 170 } else { 100 + 4 * progress };
            if o == p {
                s += v;
            } else {
                s -= v;
            }
        }
        s
    }

    fn render(&self) -> String {
        let mut s = String::from("    a b c d e f g h\n");
        for r in (0..8).rev() {
            s.push_str(&format!(" {}  ", r + 1));
            for f in 0..8 {
                s.push(match self.b[r * 8 + f] {
                    1 => 'x',
                    2 => 'X',
                    3 => 'o',
                    4 => 'O',
                    _ => {
                        if (r + f) % 2 == 0 {
                            '.'
                        } else {
                            ' '
                        }
                    }
                });
                s.push(' ');
            }
            s.push_str(&format!(" {}\n", r + 1));
        }
        s.push_str("    a b c d e f g h\n  x/X = Jucător 1 (om/rege)   o/O = Jucător 2\n");
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        let jump =
            m.path.len() > 2 || (m.path[0] as usize / 8).abs_diff(m.path[1] as usize / 8) == 2;
        let sep = if jump { "x" } else { "-" };
        m.path
            .iter()
            .map(|&i| sq(i as usize))
            .collect::<Vec<_>>()
            .join(sep)
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 500,
            max_depth: 16,
        }
    }

    fn hash_key(&self) -> Option<u64> {
        // Zobrist-style key computed on demand: XOR of one mixed value per piece.
        let mut h = polytabletop_core::hash_mix(
            1 << 40 | (self.side.idx() as u64) << 32 | self.quiet as u64,
        );
        for (i, &v) in self.b.iter().enumerate() {
            if v != 0 {
                h ^= polytabletop_core::hash_mix((v as u64) << 16 | i as u64);
            }
        }
        Some(h)
    }

    fn move_score(&self, m: &Mv) -> i32 {
        if self.is_tactical(m) {
            100 + m.path.len() as i32
        } else {
            0
        }
    }

    fn move_key(&self, m: &Mv) -> Option<u32> {
        Some(m.path[0] as u32 * 64 + m.path[1] as u32)
    }

    fn quiescence(&self) -> bool {
        true
    }

    fn is_tactical(&self, m: &Mv) -> bool {
        (m.path[0] as usize / 8).abs_diff(m.path[1] as usize / 8) == 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_has_twelve_each_and_seven_moves() {
        let g = Draughts::new();
        assert_eq!(g.counts(), (12, 12));
        assert_eq!(g.legal_moves().len(), 7);
    }

    #[test]
    fn capture_is_mandatory_and_chains() {
        let mut g = Draughts::new();
        g.b = [0; 64];
        g.b[2 * 8 + 2] = 1; // c3
        g.b[3 * 8 + 3] = 3; // d4
        g.b[5 * 8 + 5] = 3; // f6
        g.b[7 * 8 + 7] = 3; // h8, far from the action
        let mv = g.legal_moves();
        assert_eq!(mv.len(), 1);
        assert_eq!(g.format_move(&mv[0]), "c3xe5xg7");
        g.apply(&mv[0]);
        assert_eq!(g.b[6 * 8 + 6], 1); // man stands on g7 (not yet a king)
        assert_eq!(g.b[3 * 8 + 3], 0);
        assert_eq!(g.b[5 * 8 + 5], 0);
    }

    #[test]
    fn man_promotes_on_last_row() {
        let mut g = Draughts::new();
        g.b = [0; 64];
        g.b[6 * 8] = 1; // a7
        g.b[6] = 3; // g1
        let m = g.legal_moves();
        assert_eq!(g.format_move(&m[0]), "a7-b8");
        g.apply(&m[0]);
        assert_eq!(g.b[7 * 8 + 1], 2);
    }
}
