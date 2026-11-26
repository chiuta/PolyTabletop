#![allow(clippy::needless_range_loop, clippy::manual_range_contains)]
//! Dame rusești (Shashki) — 8×8, regulile rusești.
//!
//! * Piesele simple mută un pas înainte pe diagonală, dar **captură și înapoi**; regele („damka”)
//!   zboară pe diagonală oricât și capturează de la distanță (exact o piesă adversă pe drum, cu
//!   câmpuri libere dincolo de ea).
//! * Captura e obligatorie și salturile continuă cât se poate (nu e obligatoriu numărul maxim de
//!   piese). Piesele capturate se scot la sfârșitul mutării și nu pot fi sărite de două ori. O piesă
//!   simplă care ajunge pe ultimul rând în timpul capturii devine rege și continuă ca rege.
//!   La regele care a capturat, dacă există aterizări de la care captura continuă, se alege una.
//! * Câștigă cine rămâne cu ultima mutare (adversarul fără piese sau blocat). 30 de semi-mutări
//!   doar cu regi, fără captură, = remiză.
//! * Mutări: `c3-d4`, `c3xe5xg7` (pătratele de aterizare).

use polytabletop_core::{AiStyle, Game, Outcome, Player};

// 0 gol, 1 om J1, 2 rege J1, 3 om J2, 4 rege J2.

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Mv {
    pub path: Vec<u8>,
}

#[derive(Clone)]
pub struct Shashki {
    b: [u8; 64],
    side: Player,
    quiet: u16,
}

impl Default for Shashki {
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

const DIRS: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

fn at(r: i32, f: i32) -> Option<usize> {
    if r < 0 || f < 0 || r > 7 || f > 7 {
        None
    } else {
        Some((r * 8 + f) as usize)
    }
}

impl Shashki {
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
        Shashki {
            b,
            side: Player::One,
            quiet: 0,
        }
    }

    pub fn counts(&self) -> (usize, usize) {
        (
            self.b
                .iter()
                .filter(|&&c| owner(c) == Some(Player::One))
                .count(),
            self.b
                .iter()
                .filter(|&&c| owner(c) == Some(Player::Two))
                .count(),
        )
    }

    /// Capturi posibile dintr-un singur salt: (piesa capturată, pătrate de aterizare).
    fn hops(
        &self,
        board: &[u8; 64],
        at_: usize,
        king: bool,
        me: Player,
        caught: &[usize],
    ) -> Vec<(usize, Vec<usize>)> {
        let mut out: Vec<(usize, Vec<usize>)> = Vec::new();
        let (r, f) = ((at_ / 8) as i32, (at_ % 8) as i32);
        for (dr, df) in DIRS {
            if king {
                let (mut rr, mut ff) = (r + dr, f + df);
                let mut victim = None;
                while let Some(t) = at(rr, ff) {
                    if board[t] == 0 {
                        if victim.is_some() {
                            // aterizare
                            if let Some(x) = out.last_mut() {
                                x.1.push(t);
                            }
                        }
                    } else if victim.is_none()
                        && owner(board[t]) == Some(me.other())
                        && !caught.contains(&t)
                    {
                        victim = Some(t);
                        out.push((t, Vec::new()));
                    } else {
                        break;
                    }
                    rr += dr;
                    ff += df;
                }
                if let Some(x) = out.last() {
                    if x.1.is_empty() {
                        out.pop();
                    }
                }
            } else if let (Some(m), Some(l)) = (at(r + dr, f + df), at(r + 2 * dr, f + 2 * df)) {
                if owner(board[m]) == Some(me.other()) && !caught.contains(&m) && board[l] == 0 {
                    out.push((m, vec![l]));
                }
            }
        }
        out
    }

    #[allow(clippy::too_many_arguments)]
    fn chain(
        &self,
        board: &[u8; 64],
        from: usize,
        king: bool,
        me: Player,
        path: &mut Vec<u8>,
        caught: &mut Vec<usize>,
        out: &mut Vec<Mv>,
    ) {
        let hops = self.hops(board, from, king, me, caught);
        if hops.is_empty() {
            if path.len() > 1 {
                out.push(Mv { path: path.clone() });
            }
            return;
        }
        for (victim, lands) in hops {
            caught.push(victim);
            // aterizări cu continuare
            let promo_row = if me == Player::One { 7 } else { 0 };
            let conts: Vec<(usize, bool, bool)> = lands
                .iter()
                .map(|&l| {
                    let k2 = king || l / 8 == promo_row;
                    (l, k2, !self.hops(board, l, k2, me, caught).is_empty())
                })
                .collect();
            let any = conts.iter().any(|c| c.2);
            for (l, k2, more) in conts {
                if any && !more {
                    continue;
                }
                path.push(l as u8);
                self.chain(board, l, k2, me, path, caught, out);
                path.pop();
            }
            caught.pop();
        }
    }

    fn gen(&self) -> Vec<Mv> {
        let me = self.side;
        let mut caps = Vec::new();
        for i in 0..64 {
            let c = self.b[i];
            if owner(c) != Some(me) {
                continue;
            }
            let mut board = self.b;
            board[i] = 0;
            let mut path = vec![i as u8];
            let mut caught = Vec::new();
            self.chain(&board, i, is_king(c), me, &mut path, &mut caught, &mut caps);
        }
        if !caps.is_empty() {
            return caps;
        }
        let mut v = Vec::new();
        for i in 0..64 {
            let c = self.b[i];
            if owner(c) != Some(me) {
                continue;
            }
            let (r, f) = ((i / 8) as i32, (i % 8) as i32);
            for (dr, df) in DIRS {
                if is_king(c) {
                    let (mut rr, mut ff) = (r + dr, f + df);
                    while let Some(t) = at(rr, ff) {
                        if self.b[t] != 0 {
                            break;
                        }
                        v.push(Mv {
                            path: vec![i as u8, t as u8],
                        });
                        rr += dr;
                        ff += df;
                    }
                } else if (me == Player::One && dr == 1) || (me == Player::Two && dr == -1) {
                    if let Some(t) = at(r + dr, f + df) {
                        if self.b[t] == 0 {
                            v.push(Mv {
                                path: vec![i as u8, t as u8],
                            });
                        }
                    }
                }
            }
        }
        v
    }
}

impl Game for Shashki {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if self.quiet >= 30 {
            return Vec::new();
        }
        self.gen()
    }

    fn apply(&mut self, m: &Mv) {
        let start = m.path[0] as usize;
        let mut piece = self.b[start];
        let was_man = !is_king(piece);
        self.b[start] = 0;
        let mut captured = false;
        for w in m.path.windows(2) {
            let (a, b) = (w[0] as i32, w[1] as i32);
            let (ar, af, br, bf) = (a / 8, a % 8, b / 8, b % 8);
            let (dr, df) = ((br - ar).signum(), (bf - af).signum());
            let (mut r, mut f) = (ar + dr, af + df);
            while (r, f) != (br, bf) {
                let t = (r * 8 + f) as usize;
                if self.b[t] != 0 {
                    self.b[t] = 0;
                    captured = true;
                }
                r += dr;
                f += df;
            }
        }
        // promovare: oricând piesa simplă atinge ultimul rând
        let promo_row = if self.side == Player::One { 7 } else { 0 };
        for &sqr in &m.path {
            if (sqr as usize) / 8 == promo_row && !is_king(piece) {
                piece += 1;
            }
        }
        let end = *m.path.last().unwrap() as usize;
        self.b[end] = piece;
        if captured || was_man {
            self.quiet = 0;
        } else {
            self.quiet += 1;
        }
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.quiet >= 30 {
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
            let v = if is_king(c) { 330 } else { 100 + 5 * progress };
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
        s.push_str("    a b c d e f g h\n  x/X = Jucător 1 (piesă/damă)   o/O = Jucător 2\n");
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        let (a, b) = (m.path[0] as i32, m.path[1] as i32);
        // salt: mutarea are cel puțin o piesă capturată pe segment; se recunoaște după faptul că există capturi obligatorii
        let jump = self.is_capture_move(a as usize, b as usize);
        let sep = if jump { "x" } else { "-" };
        m.path
            .iter()
            .map(|&i| sq(i as usize))
            .collect::<Vec<_>>()
            .join(sep)
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(4)
    }

    fn is_tactical(&self, m: &Mv) -> bool {
        self.is_capture_move(m.path[0] as usize, m.path[1] as usize)
    }

    fn move_score(&self, m: &Mv) -> i32 {
        if self.is_tactical(m) {
            100 + m.path.len() as i32
        } else {
            0
        }
    }
}

impl Shashki {
    fn is_capture_move(&self, a: usize, b: usize) -> bool {
        let (ar, af, br, bf) = (
            (a / 8) as i32,
            (a % 8) as i32,
            (b / 8) as i32,
            (b % 8) as i32,
        );
        let (dr, df) = ((br - ar).signum(), (bf - af).signum());
        let (mut r, mut f) = (ar + dr, af + df);
        while (r, f) != (br, bf) {
            if self.b[(r * 8 + f) as usize] != 0 {
                return true;
            }
            r += dr;
            f += df;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    #[test]
    fn start_has_seven_moves() {
        let g = Shashki::new();
        assert_eq!(g.counts(), (12, 12));
        assert_eq!(g.legal_moves().len(), 7);
    }

    #[test]
    fn man_captures_backward() {
        let mut g = Shashki::new();
        g.b = [0; 64];
        g.b[3 * 8 + 3] = 1; // d4
        g.b[2 * 8 + 2] = 3; // c3 înapoi
        g.b[7 * 8 + 7] = 3;
        let mv = g.legal_moves();
        assert_eq!(mv.len(), 1);
        assert_eq!(g.format_move(&mv[0]), "d4xb2");
    }

    #[test]
    fn flying_king_capture_and_choice() {
        let mut g = Shashki::new();
        g.b = [0; 64];
        g.b[0] = 2; // a1 rege
        g.b[3 * 8 + 3] = 3; // d4 negru
        g.b[7 * 8 + 7] = 3; // h8 negru departe
        let mv = g.legal_moves();
        // damă sare d4: poate ateriza e5, f6, g7 (h8 ocupat)
        assert_eq!(mv.len(), 3);
        g.apply(&mv[0]);
        assert_eq!(g.b[3 * 8 + 3], 0);
    }

    #[test]
    fn man_promotes_mid_capture() {
        let mut g = Shashki::new();
        g.b = [0; 64];
        g.b[5 * 8 + 1] = 1; // b6
        g.b[6 * 8 + 2] = 3; // c7 negru
        g.b[6 * 8 + 4] = 3; // e7 negru
        g.b[7] = 3;
        let mv = g.legal_moves();
        // b6xd8 devine rege, apoi poate captura e7 pe diagonala d8-e7-f6..
        assert!(mv.iter().any(|m| m.path.len() == 3), "{:?}", mv);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=6u64 {
            let mut g = Shashki::new();
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() && n < 3000 {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed}");
        }
    }
}
