//! Chess with the full rules: castling, en passant, promotion, check,
//! checkmate, stalemate, fifty-move rule, threefold repetition and
//! insufficient material. White is `Player::One`.
//! Moves are typed in coordinate form: e2e4, e7e8q.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const P: i8 = 1;
const N: i8 = 2;
const B: i8 = 3;
const R: i8 = 4;
const Q: i8 = 5;
const K: i8 = 6;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Mv {
    pub from: u8,
    pub to: u8,
    pub promo: i8,
}

#[derive(Clone)]
pub struct Chess {
    b: [i8; 64], // index = rank * 8 + file, a1 = 0; white > 0, black < 0
    side: Player,
    castle: [bool; 4], // white king-side, white queen-side, black king-side, black queen-side
    ep: Option<u8>,
    half: u32,
    full: u32,
    hist: Vec<u64>,
    depth: u32,
    /// True: time-budgeted iterative deepening (default); false: fixed `depth`.
    timed: bool,
}

impl Default for Chess {
    fn default() -> Self {
        Self::new()
    }
}

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
const KING: [(i32, i32); 8] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];
const ROOK_D: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
const BISHOP_D: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

fn on(f: i32, r: i32) -> bool {
    (0..8).contains(&f) && (0..8).contains(&r)
}

fn attacked(b: &[i8; 64], sq: usize, by_white: bool) -> bool {
    let s: i8 = if by_white { 1 } else { -1 };
    let (f, r) = ((sq % 8) as i32, (sq / 8) as i32);
    let pr = if by_white { r - 1 } else { r + 1 };
    for df in [-1, 1] {
        if on(f + df, pr) && b[(pr * 8 + f + df) as usize] == s * P {
            return true;
        }
    }
    for (df, dr) in KNIGHT {
        if on(f + df, r + dr) && b[((r + dr) * 8 + f + df) as usize] == s * N {
            return true;
        }
    }
    for (df, dr) in KING {
        if on(f + df, r + dr) && b[((r + dr) * 8 + f + df) as usize] == s * K {
            return true;
        }
    }
    for (dirs, a, c) in [(&ROOK_D[..], R, Q), (&BISHOP_D[..], B, Q)] {
        for &(df, dr) in dirs {
            let (mut x, mut y) = (f + df, r + dr);
            while on(x, y) {
                let v = b[(y * 8 + x) as usize];
                if v != 0 {
                    if v == s * a || v == s * c {
                        return true;
                    }
                    break;
                }
                x += df;
                y += dr;
            }
        }
    }
    false
}

fn king_square(b: &[i8; 64], white: bool) -> usize {
    let k = if white { K } else { -K };
    b.iter().position(|&x| x == k).unwrap_or(0)
}

fn fnv(data: impl Iterator<Item = u8>) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in data {
        h ^= byte as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

impl Chess {
    pub fn new() -> Self {
        let mut b = [0i8; 64];
        let back = [R, N, B, Q, K, B, N, R];
        for f in 0..8 {
            b[f] = back[f];
            b[8 + f] = P;
            b[48 + f] = -P;
            b[56 + f] = -back[f];
        }
        let mut c = Chess {
            b,
            side: Player::One,
            castle: [true; 4],
            ep: None,
            half: 0,
            full: 1,
            hist: Vec::new(),
            depth: 3,
            timed: true,
        };
        c.hist.push(c.key());
        c
    }

    pub fn with_depth(mut self, d: u32) -> Self {
        self.depth = d;
        self.timed = false;
        self
    }

    /// En-passant square that really matters for repetition: FIDE counts two
    /// positions as equal only if the same en-passant captures are legal, so a
    /// double step that no enemy pawn can answer legally leaves no trace.
    fn effective_ep(&self) -> Option<u8> {
        let e = self.ep? as usize;
        let white = self.white();
        let pawn = if white { P } else { -P };
        let from_sqs: [usize; 2] = if white {
            [e.wrapping_sub(9), e.wrapping_sub(7)]
        } else {
            [e + 7, e + 9]
        };
        for from in from_sqs {
            if from >= 64 || (from % 8).abs_diff(e % 8) != 1 || self.b[from] != pawn {
                continue;
            }
            let nb = self.board_after(&Mv {
                from: from as u8,
                to: e as u8,
                promo: 0,
            });
            if !attacked(&nb, king_square(&nb, white), !white) {
                return Some(e as u8);
            }
        }
        None
    }

    fn key(&self) -> u64 {
        let bytes = self
            .b
            .iter()
            .map(|&x| x as u8)
            .chain(self.castle.iter().map(|&x| x as u8))
            .chain([self.side.idx() as u8, self.effective_ep().unwrap_or(255)]);
        fnv(bytes)
    }

    fn white(&self) -> bool {
        self.side == Player::One
    }

    /// Board after `m` (piece movement only: capture, en passant, castling
    /// rook, promotion). Does not touch the rest of the state.
    fn board_after(&self, m: &Mv) -> [i8; 64] {
        let mut nb = self.b;
        let (from, to) = (m.from as usize, m.to as usize);
        let piece = nb[from];
        nb[from] = 0;
        if piece.abs() == P && self.ep == Some(m.to) && (from % 8) != (to % 8) && nb[to] == 0 {
            // En passant: the captured pawn stands beside the origin square.
            nb[from / 8 * 8 + to % 8] = 0;
        }
        if piece.abs() == K && (from % 8).abs_diff(to % 8) == 2 {
            let (rf, rt) = if to % 8 == 6 {
                (from / 8 * 8 + 7, from / 8 * 8 + 5)
            } else {
                (from / 8 * 8, from / 8 * 8 + 3)
            };
            nb[rt] = nb[rf];
            nb[rf] = 0;
        }
        nb[to] = if m.promo != 0 {
            m.promo * piece.signum()
        } else {
            piece
        };
        nb
    }

    fn pseudo(&self) -> Vec<Mv> {
        let white = self.white();
        let s: i8 = if white { 1 } else { -1 };
        let mut out = Vec::with_capacity(48);
        for from in 0..64usize {
            let p = self.b[from];
            if p == 0 || p.signum() != s {
                continue;
            }
            let (f, r) = ((from % 8) as i32, (from / 8) as i32);
            let mut push = |to: usize, promo: i8| {
                out.push(Mv {
                    from: from as u8,
                    to: to as u8,
                    promo,
                })
            };
            match p.abs() {
                P => {
                    let dir = if white { 1 } else { -1 };
                    let last = if white { 7 } else { 0 };
                    let start = if white { 1 } else { 6 };
                    let r1 = r + dir;
                    if on(f, r1) && self.b[(r1 * 8 + f) as usize] == 0 {
                        let to = (r1 * 8 + f) as usize;
                        if r1 == last {
                            for pr in [Q, R, B, N] {
                                push(to, pr);
                            }
                        } else {
                            push(to, 0);
                            let r2 = r + 2 * dir;
                            if r == start && self.b[(r2 * 8 + f) as usize] == 0 {
                                push((r2 * 8 + f) as usize, 0);
                            }
                        }
                    }
                    for df in [-1, 1] {
                        if !on(f + df, r1) {
                            continue;
                        }
                        let to = (r1 * 8 + f + df) as usize;
                        let target = self.b[to];
                        let is_ep = self.ep == Some(to as u8) && target == 0;
                        if (target != 0 && target.signum() == -s) || is_ep {
                            if r1 == last {
                                for pr in [Q, R, B, N] {
                                    push(to, pr);
                                }
                            } else {
                                push(to, 0);
                            }
                        }
                    }
                }
                N | K => {
                    let deltas: &[(i32, i32)] = if p.abs() == N { &KNIGHT } else { &KING };
                    for &(df, dr) in deltas {
                        if on(f + df, r + dr) {
                            let to = ((r + dr) * 8 + f + df) as usize;
                            if self.b[to].signum() != s {
                                push(to, 0);
                            }
                        }
                    }
                }
                _ => {
                    let dirs: Vec<(i32, i32)> = match p.abs() {
                        R => ROOK_D.to_vec(),
                        B => BISHOP_D.to_vec(),
                        _ => ROOK_D.iter().chain(BISHOP_D.iter()).copied().collect(),
                    };
                    for (df, dr) in dirs {
                        let (mut x, mut y) = (f + df, r + dr);
                        while on(x, y) {
                            let to = (y * 8 + x) as usize;
                            let t = self.b[to];
                            if t == 0 {
                                push(to, 0);
                            } else {
                                if t.signum() != s {
                                    push(to, 0);
                                }
                                break;
                            }
                            x += df;
                            y += dr;
                        }
                    }
                }
            }
        }
        // Castling.
        let (base, ks, qs) = if white {
            (0usize, 0, 1)
        } else {
            (56usize, 2, 3)
        };
        if self.b[base + 4] == s * K && !attacked(&self.b, base + 4, !white) {
            if self.castle[ks]
                && self.b[base + 5] == 0
                && self.b[base + 6] == 0
                && self.b[base + 7] == s * R
                && !attacked(&self.b, base + 5, !white)
            {
                out.push(Mv {
                    from: (base + 4) as u8,
                    to: (base + 6) as u8,
                    promo: 0,
                });
            }
            if self.castle[qs]
                && self.b[base + 1] == 0
                && self.b[base + 2] == 0
                && self.b[base + 3] == 0
                && self.b[base] == s * R
                && !attacked(&self.b, base + 3, !white)
            {
                out.push(Mv {
                    from: (base + 4) as u8,
                    to: (base + 2) as u8,
                    promo: 0,
                });
            }
        }
        out
    }

    fn gen(&self) -> Vec<Mv> {
        let white = self.white();
        let mut v: Vec<Mv> = self
            .pseudo()
            .into_iter()
            .filter(|m| {
                let nb = self.board_after(m);
                !attacked(&nb, king_square(&nb, white), !white)
            })
            .collect();
        // Captures first: better alpha-beta pruning.
        v.sort_by_key(|m| {
            let victim = self.b[m.to as usize].abs();
            -(victim as i32 * 10 - self.b[m.from as usize].abs() as i32)
        });
        v
    }

    pub fn in_check(&self) -> bool {
        let white = self.white();
        attacked(&self.b, king_square(&self.b, white), !white)
    }

    fn insufficient(&self) -> bool {
        let pieces: Vec<i8> = self
            .b
            .iter()
            .filter(|&&x| x != 0 && x.abs() != K)
            .map(|x| x.abs())
            .collect();
        match pieces.len() {
            0 => true,
            1 => pieces[0] == N || pieces[0] == B,
            _ => {
                // Only bishops, all on squares of one colour: no mate is possible.
                let mut colours = self
                    .b
                    .iter()
                    .enumerate()
                    .filter(|&(_, &x)| x.abs() == B)
                    .map(|(i, _)| (i / 8 + i % 8) % 2);
                let all_bishops = pieces.iter().all(|&x| x == B);
                match colours.next() {
                    Some(c0) if all_bishops => colours.all(|c| c == c0),
                    _ => false,
                }
            }
        }
    }

    pub fn fen_board(&self) -> String {
        let mut s = String::new();
        for r in (0..8).rev() {
            let mut empty = 0;
            for f in 0..8 {
                let v = self.b[r * 8 + f];
                if v == 0 {
                    empty += 1;
                } else {
                    if empty > 0 {
                        s.push_str(&empty.to_string());
                        empty = 0;
                    }
                    s.push(piece_char(v));
                }
            }
            if empty > 0 {
                s.push_str(&empty.to_string());
            }
            if r > 0 {
                s.push('/');
            }
        }
        s
    }
}

fn piece_char(v: i8) -> char {
    let c = match v.abs() {
        P => 'p',
        N => 'n',
        B => 'b',
        R => 'r',
        Q => 'q',
        _ => 'k',
    };
    if v > 0 {
        c.to_ascii_uppercase()
    } else {
        c
    }
}

fn promo_char(p: i8) -> &'static str {
    match p {
        Q => "q",
        R => "r",
        B => "b",
        N => "n",
        _ => "",
    }
}

fn sq(i: u8) -> String {
    format!("{}{}", (b'a' + i % 8) as char, i / 8 + 1)
}

fn value(p: i8) -> i32 {
    match p.abs() {
        P => 100,
        N => 320,
        B => 330,
        R => 500,
        Q => 900,
        _ => 0,
    }
}

impl Game for Chess {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        let mv = self.gen();
        if mv.is_empty() || self.half >= 100 || self.insufficient() {
            return Vec::new();
        }
        let k = *self.hist.last().unwrap_or(&0);
        if self.hist.iter().filter(|&&h| h == k).count() >= 3 {
            return Vec::new();
        }
        mv
    }

    fn apply(&mut self, m: &Mv) {
        let (from, to) = (m.from as usize, m.to as usize);
        let piece = self.b[from];
        let capture =
            self.b[to] != 0 || (piece.abs() == P && self.ep == Some(m.to) && from % 8 != to % 8);
        let nb = self.board_after(m);
        self.b = nb;
        self.ep = if piece.abs() == P && from.abs_diff(to) == 16 {
            Some(((from + to) / 2) as u8)
        } else {
            None
        };
        // Castling rights.
        if piece.abs() == K {
            if piece > 0 {
                self.castle[0] = false;
                self.castle[1] = false;
            } else {
                self.castle[2] = false;
                self.castle[3] = false;
            }
        }
        for s in [from, to] {
            match s {
                0 => self.castle[1] = false,
                7 => self.castle[0] = false,
                56 => self.castle[3] = false,
                63 => self.castle[2] = false,
                _ => {}
            }
        }
        if piece.abs() == P || capture {
            self.half = 0;
        } else {
            self.half += 1;
        }
        if self.side == Player::Two {
            self.full += 1;
        }
        self.side = self.side.other();
        let k = self.key();
        self.hist.push(k);
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.gen().is_empty() {
            return Some(if self.in_check() {
                Outcome::Win(self.side.other())
            } else {
                Outcome::Draw
            });
        }
        if self.half >= 100 || self.insufficient() {
            return Some(Outcome::Draw);
        }
        let k = *self.hist.last().unwrap_or(&0);
        if self.hist.iter().filter(|&&h| h == k).count() >= 3 {
            return Some(Outcome::Draw);
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let mut s = 0;
        for i in 0..64 {
            let v = self.b[i];
            if v == 0 {
                continue;
            }
            let white = v > 0;
            let (f, r) = ((i % 8) as i32, (i / 8) as i32);
            let rel_rank = if white { r } else { 7 - r };
            let centre =
                3 - (f - 3).abs().min((f - 4).abs()) + 3 - (r - 3).abs().min((r - 4).abs());
            let mut x = value(v);
            match v.abs() {
                P => x += 6 * rel_rank + 2 * centre,
                N | B => x += 4 * centre,
                _ => {}
            }
            if white == (p == Player::One) {
                s += x;
            } else {
                s -= x;
            }
        }
        s
    }

    fn render(&self) -> String {
        let mut s = String::from("    a b c d e f g h\n");
        for r in (0..8).rev() {
            s.push_str(&format!(" {}  ", r + 1));
            for f in 0..8 {
                let v = self.b[r * 8 + f];
                s.push(if v == 0 { '.' } else { piece_char(v) });
                s.push(' ');
            }
            s.push_str(&format!(" {}\n", r + 1));
        }
        s.push_str("    a b c d e f g h\n");
        s.push_str(&format!(
            "  Mută {} (mutarea {}){}\n  MAJUSCULE = alb, minuscule = negru\n",
            if self.white() { "albul" } else { "negrul" },
            self.full,
            if self.in_check() { "  — ȘAH!" } else { "" }
        ));
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        format!("{}{}{}", sq(m.from), sq(m.to), promo_char(m.promo))
    }

    fn ai_style(&self) -> AiStyle {
        if self.timed {
            AiStyle::AlphaBetaTime {
                ms: 500,
                max_depth: 14,
            }
        } else {
            AiStyle::AlphaBeta(self.depth)
        }
    }

    fn hash_key(&self) -> Option<u64> {
        // `key()` covers board, side, castling rights and en-passant square.
        let k = self.hist.last().copied().unwrap_or_else(|| self.key());
        Some(polytabletop_core::hash_mix(k))
    }

    fn move_score(&self, m: &Mv) -> i32 {
        const V: [i32; 7] = [0, 1, 3, 3, 5, 9, 0];
        let piece = self.b[m.from as usize].unsigned_abs() as usize;
        let mut victim = self.b[m.to as usize].unsigned_abs() as usize;
        if victim == 0 && piece == P as usize && m.from % 8 != m.to % 8 {
            victim = P as usize; // en passant
        }
        let mut s = 0;
        if victim != 0 {
            s += 100 + V[victim] * 10 - V[piece];
        }
        if m.promo != 0 {
            s += 80 + V[m.promo.unsigned_abs() as usize];
        }
        s
    }

    fn move_key(&self, m: &Mv) -> Option<u32> {
        Some(m.from as u32 * 64 + m.to as u32)
    }

    fn quiescence(&self) -> bool {
        true
    }

    fn is_tactical(&self, m: &Mv) -> bool {
        self.move_score(m) > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn count_nodes(g: &Chess, depth: u32) -> u64 {
        if depth == 0 {
            return 1;
        }
        let mut n = 0;
        for m in g.gen() {
            let mut c = g.clone();
            c.apply(&m);
            n += count_nodes(&c, depth - 1);
        }
        n
    }

    fn play(g: &mut Chess, s: &str) {
        let m = g
            .legal_moves()
            .into_iter()
            .find(|m| g.format_move(m) == s)
            .unwrap_or_else(|| panic!("illegal {s}"));
        g.apply(&m);
    }

    #[test]
    fn perft_start_position() {
        let g = Chess::new();
        assert_eq!(count_nodes(&g, 1), 20);
        assert_eq!(count_nodes(&g, 2), 400);
        assert_eq!(count_nodes(&g, 3), 8_902);
        assert_eq!(count_nodes(&g, 4), 197_281);
    }

    #[test]
    fn fools_mate() {
        let mut g = Chess::new();
        for m in ["f2f3", "e7e5", "g2g4", "d8h4"] {
            play(&mut g, m);
        }
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
    }

    #[test]
    fn castling_en_passant_promotion() {
        let mut g = Chess::new();
        for m in ["e2e4", "a7a6", "e4e5", "d7d5"] {
            play(&mut g, m);
        }
        // En passant must be offered.
        assert!(g.legal_moves().iter().any(|m| g.format_move(m) == "e5d6"));
        play(&mut g, "e5d6");
        assert_eq!(g.b[4 * 8 + 3], 0); // captured pawn on d5 is gone

        let mut c = Chess::new();
        for m in ["g1f3", "g8f6", "e2e3", "e7e6", "f1e2", "f8e7"] {
            play(&mut c, m);
        }
        assert!(c.legal_moves().iter().any(|m| c.format_move(m) == "e1g1"));
        play(&mut c, "e1g1");
        assert_eq!(c.b[6], K);
        assert_eq!(c.b[5], R);

        let mut p = Chess::new();
        p.b = [0; 64];
        p.b[4] = K;
        p.b[60] = -K;
        p.b[6 * 8] = P; // a7
        p.hist = vec![p.key()];
        let moves: Vec<String> = p.legal_moves().iter().map(|m| p.format_move(m)).collect();
        assert!(moves.contains(&"a7a8q".to_string()));
        assert!(moves.contains(&"a7a8n".to_string()));
    }

    #[test]
    fn stalemate_is_a_draw() {
        let mut g = Chess::new();
        g.b = [0; 64];
        g.b[7 * 8] = -K; // a8
        g.b[5 * 8 + 1] = K; // b6
        g.b[6 * 8 + 2] = Q; // c7
        g.side = Player::Two;
        g.hist = vec![g.key()];
        assert_eq!(g.outcome(), Some(Outcome::Draw));
    }

    #[test]
    fn cannot_castle_through_check() {
        let mut g = Chess::new();
        g.b = [0; 64];
        g.b[4] = K;
        g.b[7] = R;
        g.b[60] = -K;
        g.b[5 + 56] = -R; // black rook on f8 attacks f1
        g.hist = vec![g.key()];
        assert!(!g.legal_moves().iter().any(|m| g.format_move(m) == "e1g1"));
    }
}
