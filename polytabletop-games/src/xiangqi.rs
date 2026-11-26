//! Xiangqi (șahul chinezesc), 9×10, written from the public rules.
//!
//! Player One = roșu (jos, rândurile 0–4), Player Two = negru (sus).
//! Moves use UCCI-style coordinates: files `a`–`i`, ranks `0`–`9`, e.g. `h2e2`.
//! Rules: general and advisors stay in the palace, elephants cannot cross the
//! river and are blocked at the "eye", horses are blocked at the "leg", cannons
//! capture by jumping exactly one piece, soldiers gain sideways steps after
//! the river, and the two generals may never face each other on an open file.
//! No move may leave one's own general attacked. No legal move = loss.
//! Draw after 120 plies without a capture (a simplification of repetition rules).

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const W: i32 = 9;
const H: i32 = 10;

const K: i8 = 1;
const A: i8 = 2;
const E: i8 = 3;
const HO: i8 = 4;
const R: i8 = 5;
const C: i8 = 6;
const P: i8 = 7;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Mv {
    from: u8,
    to: u8,
}

#[derive(Clone)]
pub struct Xiangqi {
    b: [i8; 90], // index = rank * 9 + file; >0 One(red), <0 Two(black)
    side: Player,
    quiet: u16,
}

impl Default for Xiangqi {
    fn default() -> Self {
        Self::new()
    }
}

fn idx(f: i32, r: i32) -> usize {
    (r * W + f) as usize
}

fn sign(p: Player) -> i8 {
    match p {
        Player::One => 1,
        Player::Two => -1,
    }
}

impl Xiangqi {
    pub fn new() -> Self {
        let mut b = [0i8; 90];
        let back = [R, HO, E, A, K, A, E, HO, R];
        for (f, &p) in back.iter().enumerate() {
            b[idx(f as i32, 0)] = p;
            b[idx(f as i32, 9)] = -p;
        }
        for f in [1, 7] {
            b[idx(f, 2)] = C;
            b[idx(f, 7)] = -C;
        }
        for f in [0, 2, 4, 6, 8] {
            b[idx(f, 3)] = P;
            b[idx(f, 6)] = -P;
        }
        Xiangqi {
            b,
            side: Player::One,
            quiet: 0,
        }
    }

    fn in_palace(f: i32, r: i32, s: i8) -> bool {
        (3..=5).contains(&f) && if s > 0 { r <= 2 } else { r >= 7 }
    }

    fn own_half(r: i32, s: i8) -> bool {
        if s > 0 {
            r <= 4
        } else {
            r >= 5
        }
    }

    /// Pseudo-legal moves (own general may remain attacked).
    fn pseudo(&self, side: Player) -> Vec<Mv> {
        let s = sign(side);
        let mut out = Vec::with_capacity(48);
        for from in 0..90i32 {
            let v = self.b[from as usize];
            if v == 0 || v.signum() != s {
                continue;
            }
            let (f, r) = (from % W, from / W);
            let push = |tf: i32, tr: i32, out: &mut Vec<Mv>| {
                if (0..W).contains(&tf) && (0..H).contains(&tr) {
                    let t = self.b[idx(tf, tr)];
                    if t.signum() != s {
                        out.push(Mv {
                            from: from as u8,
                            to: idx(tf, tr) as u8,
                        });
                    }
                }
            };
            match v.abs() {
                K => {
                    for (df, dr) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                        let (tf, tr) = (f + df, r + dr);
                        if Self::in_palace(tf, tr, s) {
                            push(tf, tr, &mut out);
                        }
                    }
                }
                A => {
                    for (df, dr) in [(1, 1), (1, -1), (-1, 1), (-1, -1)] {
                        let (tf, tr) = (f + df, r + dr);
                        if Self::in_palace(tf, tr, s) {
                            push(tf, tr, &mut out);
                        }
                    }
                }
                E => {
                    for (df, dr) in [(1, 1), (1, -1), (-1, 1), (-1, -1)] {
                        let (tf, tr) = (f + 2 * df, r + 2 * dr);
                        if (0..W).contains(&tf)
                            && (0..H).contains(&tr)
                            && Self::own_half(tr, s)
                            && self.b[idx(f + df, r + dr)] == 0
                        {
                            push(tf, tr, &mut out);
                        }
                    }
                }
                HO => {
                    for (df, dr) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                        let (lf, lr) = (f + df, r + dr);
                        if !(0..W).contains(&lf) || !(0..H).contains(&lr) {
                            continue;
                        }
                        if self.b[idx(lf, lr)] != 0 {
                            continue; // hobbled leg
                        }
                        let (xf, xr) = (f + 2 * df, r + 2 * dr);
                        if df != 0 {
                            push(xf, xr + 1, &mut out);
                            push(xf, xr - 1, &mut out);
                        } else {
                            push(xf + 1, xr, &mut out);
                            push(xf - 1, xr, &mut out);
                        }
                    }
                }
                R => {
                    for (df, dr) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                        let (mut tf, mut tr) = (f + df, r + dr);
                        while (0..W).contains(&tf) && (0..H).contains(&tr) {
                            let t = self.b[idx(tf, tr)];
                            if t == 0 {
                                push(tf, tr, &mut out);
                            } else {
                                push(tf, tr, &mut out);
                                break;
                            }
                            tf += df;
                            tr += dr;
                        }
                    }
                }
                C => {
                    for (df, dr) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                        let (mut tf, mut tr) = (f + df, r + dr);
                        let mut screen = false;
                        while (0..W).contains(&tf) && (0..H).contains(&tr) {
                            let t = self.b[idx(tf, tr)];
                            if !screen {
                                if t == 0 {
                                    push(tf, tr, &mut out);
                                } else {
                                    screen = true;
                                }
                            } else if t != 0 {
                                if t.signum() != s {
                                    push(tf, tr, &mut out);
                                }
                                break;
                            }
                            tf += df;
                            tr += dr;
                        }
                    }
                }
                _ => {
                    // soldier
                    push(f, r + s as i32, &mut out);
                    if !Self::own_half(r, s) {
                        push(f + 1, r, &mut out);
                        push(f - 1, r, &mut out);
                    }
                }
            }
        }
        out
    }

    fn general_pos(&self, side: Player) -> Option<i32> {
        let g = K * sign(side);
        (0..90).find(|&i| self.b[i as usize] == g)
    }

    /// True if `side`'s general is attacked (including the flying-general rule).
    fn in_check(&self, side: Player) -> bool {
        let Some(g) = self.general_pos(side) else {
            return true;
        };
        if let Some(og) = self.general_pos(side.other()) {
            if og % W == g % W {
                let (lo, hi) = (g.min(og) / W, g.max(og) / W);
                if ((lo + 1)..hi).all(|r| self.b[idx(g % W, r)] == 0) {
                    return true;
                }
            }
        }
        self.pseudo(side.other()).iter().any(|m| m.to as i32 == g)
    }

    fn make(&mut self, m: &Mv) -> bool {
        let cap = self.b[m.to as usize] != 0;
        self.b[m.to as usize] = self.b[m.from as usize];
        self.b[m.from as usize] = 0;
        self.side = self.side.other();
        cap
    }

    fn all_legal(&self) -> Vec<Mv> {
        let side = self.side;
        let mut v: Vec<Mv> = self
            .pseudo(side)
            .into_iter()
            .filter(|m| {
                let mut g = self.clone();
                g.make(m);
                !g.in_check(side)
            })
            .collect();
        // captures first helps alpha-beta
        v.sort_by_key(|m| if self.b[m.to as usize] != 0 { 0 } else { 1 });
        v
    }
}

fn value(p: i8) -> i32 {
    match p.abs() {
        K => 0,
        A => 20,
        E => 20,
        HO => 40,
        R => 90,
        C => 45,
        _ => 10,
    }
}

impl Game for Xiangqi {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if self.quiet >= 120 {
            return Vec::new();
        }
        self.all_legal()
    }

    fn apply(&mut self, m: &Mv) {
        if self.make(m) {
            self.quiet = 0;
        } else {
            self.quiet += 1;
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.quiet >= 120 {
            return Some(Outcome::Draw);
        }
        if self.all_legal().is_empty() {
            // checkmate and stalemate both lose in xiangqi
            return Some(Outcome::Win(self.side.other()));
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = sign(p);
        let mut s = 0;
        for i in 0..90i32 {
            let v = self.b[i as usize];
            if v == 0 {
                continue;
            }
            let (f, r) = (i % W, i / W);
            let mut x = value(v);
            if v.abs() == P {
                let adv = if v > 0 { r } else { 9 - r };
                if adv >= 5 {
                    x += 10 + (adv - 5) * 3;
                }
            }
            if matches!(v.abs(), HO | C | R) {
                x += 4 - (f - 4).abs(); // centre files
            }
            s += if v.signum() == me { x } else { -x };
        }
        s
    }

    fn render(&self) -> String {
        let mut s = String::new();
        for r in (0..H).rev() {
            s.push_str(&format!("{} ", r));
            for f in 0..W {
                let v = self.b[idx(f, r)];
                let ch = match v.abs() {
                    0 => '.',
                    K => 'k',
                    A => 'a',
                    E => 'e',
                    HO => 'h',
                    R => 'r',
                    C => 'c',
                    _ => 'p',
                };
                s.push(if v > 0 { ch.to_ascii_uppercase() } else { ch });
                s.push(' ');
            }
            s.push('\n');
            if r == 5 {
                s.push_str("  ~ ~ ~ râul ~ ~ ~ ~\n");
            }
        }
        s.push_str("  a b c d e f g h i\n  (MAJUSCULE = roșu, începe)\n");
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        let n = |i: u8| format!("{}{}", (b'a' + (i as i32 % W) as u8) as char, i as i32 / W);
        format!("{}{}", n(m.from), n(m.to))
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 500,
            max_depth: 12,
        }
    }

    fn hash_key(&self) -> Option<u64> {
        // Zobrist-style key computed on demand: XOR of one mixed value per piece.
        let mut h = polytabletop_core::hash_mix(
            1 << 40 | (self.side.idx() as u64) << 32 | self.quiet as u64,
        );
        for (i, &v) in self.b.iter().enumerate() {
            if v != 0 {
                h ^= polytabletop_core::hash_mix((v as u8 as u64) << 16 | i as u64);
            }
        }
        Some(h)
    }

    fn move_score(&self, m: &Mv) -> i32 {
        const V: [i32; 8] = [0, 100, 2, 2, 4, 9, 4, 1];
        let victim = self.b[m.to as usize].unsigned_abs() as usize;
        if victim == 0 {
            return 0;
        }
        let piece = self.b[m.from as usize].unsigned_abs() as usize;
        100 + V[victim] * 10 - V[piece]
    }

    fn move_key(&self, m: &Mv) -> Option<u32> {
        Some(m.from as u32 * 128 + m.to as u32)
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

    fn count(g: &Xiangqi, d: u32) -> u64 {
        if d == 0 {
            return 1;
        }
        g.all_legal()
            .iter()
            .map(|m| {
                let mut h = g.clone();
                h.make(m);
                count(&h, d - 1)
            })
            .sum()
    }

    #[test]
    fn perft_start() {
        // Well-known xiangqi perft values: 44, 1920, 79666.
        let g = Xiangqi::new();
        assert_eq!(count(&g, 1), 44);
        assert_eq!(count(&g, 2), 1920);
        assert_eq!(count(&g, 3), 79666);
    }

    #[test]
    fn no_flying_general() {
        let mut g = Xiangqi::new();
        g.b = [0; 90];
        g.b[idx(4, 0)] = K;
        g.b[idx(4, 9)] = -K;
        g.b[idx(4, 4)] = R; // red rook is the only screen
        let moves = g.all_legal();
        // the rook may not leave the e-file (would expose the generals)
        assert!(moves
            .iter()
            .filter(|m| m.from as i32 == idx(4, 4) as i32)
            .all(|m| m.to as i32 % W == 4));
    }

    #[test]
    fn cannon_needs_screen() {
        let mut g = Xiangqi::new();
        g.b = [0; 90];
        g.b[idx(3, 0)] = K;
        g.b[idx(5, 9)] = -K;
        g.b[idx(0, 0)] = C;
        g.b[idx(0, 5)] = -P;
        let caps = g
            .all_legal()
            .into_iter()
            .filter(|m| m.to as i32 == idx(0, 5) as i32)
            .count();
        assert_eq!(caps, 0);
        g.b[idx(0, 2)] = P; // screen
        let caps = g
            .all_legal()
            .into_iter()
            .filter(|m| m.to as i32 == idx(0, 5) as i32)
            .count();
        assert_eq!(caps, 1);
    }

    #[test]
    fn horse_leg_blocks() {
        let mut g = Xiangqi::new();
        g.b = [0; 90];
        g.b[idx(3, 0)] = K;
        g.b[idx(5, 9)] = -K;
        g.b[idx(4, 4)] = HO;
        let free = g
            .all_legal()
            .iter()
            .filter(|m| m.from as i32 == idx(4, 4) as i32)
            .count();
        assert_eq!(free, 8);
        g.b[idx(4, 5)] = -P; // blocks the two forward jumps
        let blocked = g
            .all_legal()
            .iter()
            .filter(|m| m.from as i32 == idx(4, 4) as i32)
            .count();
        assert_eq!(blocked, 6);
    }

    #[test]
    fn mate_is_loss() {
        let mut g = Xiangqi::new();
        g.b = [0; 90];
        g.b[idx(4, 9)] = -K;
        g.b[idx(3, 0)] = K;
        g.b[idx(3, 8)] = R;
        g.b[idx(5, 8)] = R;
        g.b[idx(4, 7)] = R; // black general boxed in: rooks guard all exits
        g.side = Player::Two;
        // black is in check from r on e8? (4,7) with (4,8) empty -> check on file
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }
}
