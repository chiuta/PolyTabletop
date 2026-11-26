//! Makruk (șahul thailandez), 8×8, scris din regulile publice.
//!
//! Player One = alb (jos, rândurile 0–2), Player Two = negru (sus).
//! Piese: Khun (rege), Met (regină: un pas diagonal), Khon (nebun: un pas
//! diagonal sau înainte), Ma (cal), Rua (turn), pion. Fără rocadă, fără
//! pas dublu, fără en passant. Pionul se promovează automat în Met când
//! ajunge pe al șaselea rând (din partea lui). Mat = victorie; pat = remiză.
//! Numărare simplificată: când nu mai există pioni, se numără mutările
//! (limită după materialul părții mai puternice: 2 ture 8, 1 turn 16,
//! 2 nebuni 22, 2 cai 32, 1 nebun 44, altfel 64 de mutări; ambele
//! părți cu piese = 64). În plus, 200 de semi-mutări fără captură sau mutare
//! de pion = remiză. Mutări: e3e4 (de la, la).

//! Verificare surse (nov. 2026: en.wikipedia.org/wiki/Makruk): pionul promovat pe rândul 6
//! în Met, limitele 8/16/22/32/44/64 și „board honour” 64 de mutări corespund. Diferențe
//! documentate (numărarea simplificată): în joc real numărarea o pornește partea mai slabă
//! (poate s-o oprească sau reia), începe de la numărul pieselor de pe tablă, iar
//! „piece honour” se aplică doar când partea slabă a rămas cu regele gol; aici numărul
//! rulează automat pentru toate pozițiile fără pioni.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const KH: i8 = 1; // Khun
const MET: i8 = 2;
const KHON: i8 = 3;
const MA: i8 = 4;
const RUA: i8 = 5;
const P: i8 = 6;

const QUIET_CAP: u16 = 200;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Mv {
    from: u8,
    to: u8,
}

#[derive(Clone)]
pub struct Makruk {
    b: [i8; 64], // index = rank * 8 + file; >0 One(alb), <0 Two(negru)
    side: Player,
    quiet: u16,
    count: u16,
}

impl Default for Makruk {
    fn default() -> Self {
        Self::new()
    }
}

fn idx(f: i32, r: i32) -> usize {
    (r * 8 + f) as usize
}

fn sign(p: Player) -> i8 {
    match p {
        Player::One => 1,
        Player::Two => -1,
    }
}

const ORTHO: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
const DIAG: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
const KNIGHT: [(i32, i32); 8] = [
    (1, 2),
    (2, 1),
    (2, -1),
    (1, -2),
    (-1, -2),
    (-2, -1),
    (-2, 1),
    (-1, 2),
];

impl Makruk {
    pub fn new() -> Self {
        let mut b = [0i8; 64];
        let white = [RUA, MA, KHON, MET, KH, KHON, MA, RUA];
        let black = [RUA, MA, KHON, KH, MET, KHON, MA, RUA];
        for f in 0..8 {
            b[idx(f as i32, 0)] = white[f];
            b[idx(f as i32, 7)] = -black[f];
            b[idx(f as i32, 2)] = P;
            b[idx(f as i32, 5)] = -P;
        }
        Makruk {
            b,
            side: Player::One,
            quiet: 0,
            count: 0,
        }
    }

    fn pseudo(&self, side: Player) -> Vec<Mv> {
        let s = sign(side);
        let mut out = Vec::with_capacity(40);
        for from in 0..64i32 {
            let v = self.b[from as usize];
            if v == 0 || v.signum() != s {
                continue;
            }
            let (f, r) = (from % 8, from / 8);
            let push = |tf: i32, tr: i32, out: &mut Vec<Mv>| -> bool {
                // returns true if the square was empty
                if (0..8).contains(&tf) && (0..8).contains(&tr) {
                    let t = self.b[idx(tf, tr)];
                    if t.signum() != s {
                        out.push(Mv {
                            from: from as u8,
                            to: idx(tf, tr) as u8,
                        });
                    }
                    return t == 0;
                }
                false
            };
            match v.abs() {
                KH => {
                    for (df, dr) in ORTHO.iter().chain(DIAG.iter()) {
                        push(f + df, r + dr, &mut out);
                    }
                }
                MET => {
                    for (df, dr) in DIAG {
                        push(f + df, r + dr, &mut out);
                    }
                }
                KHON => {
                    for (df, dr) in DIAG {
                        push(f + df, r + dr, &mut out);
                    }
                    push(f, r + s as i32, &mut out);
                }
                MA => {
                    for (df, dr) in KNIGHT {
                        push(f + df, r + dr, &mut out);
                    }
                }
                RUA => {
                    for (df, dr) in ORTHO {
                        let (mut tf, mut tr) = (f + df, r + dr);
                        while push(tf, tr, &mut out) {
                            tf += df;
                            tr += dr;
                        }
                    }
                }
                _ => {
                    let tr = r + s as i32;
                    if (0..8).contains(&tr) {
                        if self.b[idx(f, tr)] == 0 {
                            out.push(Mv {
                                from: from as u8,
                                to: idx(f, tr) as u8,
                            });
                        }
                        for df in [-1, 1] {
                            let tf = f + df;
                            if (0..8).contains(&tf) && self.b[idx(tf, tr)].signum() == -s {
                                out.push(Mv {
                                    from: from as u8,
                                    to: idx(tf, tr) as u8,
                                });
                            }
                        }
                    }
                }
            }
        }
        out
    }

    fn king_pos(&self, side: Player) -> Option<usize> {
        let k = KH * sign(side);
        (0..64).find(|&i| self.b[i] == k)
    }

    fn in_check(&self, side: Player) -> bool {
        let Some(k) = self.king_pos(side) else {
            return true;
        };
        self.pseudo(side.other()).iter().any(|m| m.to as usize == k)
    }

    /// Plays the move; returns true if it was a capture or a pawn move.
    fn make(&mut self, m: &Mv) -> bool {
        let v = self.b[m.from as usize];
        let cap = self.b[m.to as usize] != 0;
        let tr = m.to as i32 / 8;
        let promo = v.abs() == P && ((v > 0 && tr == 5) || (v < 0 && tr == 2));
        self.b[m.to as usize] = if promo { MET * v.signum() } else { v };
        self.b[m.from as usize] = 0;
        self.side = self.side.other();
        cap || v.abs() == P
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
        v.sort_by_key(|m| if self.b[m.to as usize] != 0 { 0 } else { 1 });
        v
    }

    fn has_pawns(&self) -> bool {
        self.b.iter().any(|v| v.abs() == P)
    }

    /// Counting limit (in plies) for the current material, once no pawns remain.
    fn count_limit(&self) -> u16 {
        let tally = |s: i8| {
            let mut c = [0u8; 8];
            for &v in &self.b {
                if v != 0 && v.signum() == s {
                    c[v.unsigned_abs() as usize] += 1;
                }
            }
            c
        };
        let (w, k) = (tally(1), tally(-1));
        let bare = |c: &[u8; 8]| c.iter().sum::<u8>() == 1;
        if bare(&w) && bare(&k) {
            return 0;
        }
        let strong = if bare(&w) {
            Some(k)
        } else if bare(&k) {
            Some(w)
        } else {
            None
        };
        let moves = match strong {
            None => 64,
            Some(c) => {
                if c[RUA as usize] >= 2 {
                    8
                } else if c[RUA as usize] == 1 {
                    16
                } else if c[KHON as usize] >= 2 {
                    22
                } else if c[MA as usize] >= 2 {
                    32
                } else if c[KHON as usize] == 1 {
                    44
                } else {
                    64
                }
            }
        };
        moves * 2
    }

    fn drawn(&self) -> bool {
        self.quiet >= QUIET_CAP
            || (self.count > 0 && self.count >= self.count_limit())
            || (!self.has_pawns() && self.count_limit() == 0)
    }
}

fn value(p: i8) -> i32 {
    match p.abs() {
        MET => 130,
        KHON => 200,
        MA => 250,
        RUA => 500,
        P => 100,
        _ => 0,
    }
}

impl Game for Makruk {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if self.drawn() {
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
        if self.has_pawns() {
            self.count = 0;
        } else {
            self.count += 1;
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.drawn() {
            return Some(Outcome::Draw);
        }
        if self.all_legal().is_empty() {
            return Some(if self.in_check(self.side) {
                Outcome::Win(self.side.other())
            } else {
                Outcome::Draw
            });
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = sign(p);
        let mut s = 0;
        let mut mat = 0;
        let mut kings = [(0i32, 0i32); 2];
        for i in 0..64i32 {
            let v = self.b[i as usize];
            if v == 0 {
                continue;
            }
            let (f, r) = (i % 8, i / 8);
            let mut x = value(v);
            if v.abs() == KH {
                kings[usize::from(v.signum() != me)] = (f, r);
            }
            if v.abs() == P {
                let adv = if v > 0 { r - 2 } else { 5 - r };
                x += adv * 8;
            } else if v.abs() != KH {
                x += 3 - ((2 * f - 7).abs() + (2 * r - 7).abs()) / 4; // centralisation
            }
            if v.signum() == me {
                s += x;
                mat += value(v);
            } else {
                s -= x;
                mat -= value(v);
            }
        }
        // With a clear material lead, drive the enemy king to the edge.
        if mat.abs() >= 200 && !self.has_pawns() {
            let (own, foe) = if mat > 0 {
                (kings[0], kings[1])
            } else {
                (kings[1], kings[0])
            };
            let edge = (2 * foe.0 - 7).abs() + (2 * foe.1 - 7).abs();
            let dist = (own.0 - foe.0).abs() + (own.1 - foe.1).abs();
            let bonus = edge * 4 + (14 - dist) * 2;
            s += if mat > 0 { bonus } else { -bonus };
        }
        s
    }

    fn render(&self) -> String {
        let mut s = String::new();
        for r in (0..8).rev() {
            s.push_str(&format!("{} ", r + 1));
            for f in 0..8 {
                let v = self.b[idx(f, r)];
                let ch = match v.abs() {
                    0 => '.',
                    KH => 'k',
                    MET => 'm',
                    KHON => 'b',
                    MA => 'n',
                    RUA => 'r',
                    _ => 'p',
                };
                s.push(if v > 0 { ch.to_ascii_uppercase() } else { ch });
                s.push(' ');
            }
            s.push('\n');
        }
        s.push_str("  a b c d e f g h\n  (MAJUSCULE = alb, începe; k=Khun m=Met b=Khon n=Ma r=Rua p=pion)\n");
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        let n = |i: u8| polytabletop_core::square_name(i as usize % 8, i as usize / 8);
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
            1 << 40
                | (self.side.idx() as u64) << 32
                | (self.quiet as u64) << 8 ^ (self.count as u64) << 16,
        );
        for (i, &v) in self.b.iter().enumerate() {
            if v != 0 {
                h ^= polytabletop_core::hash_mix((v as u8 as u64) << 16 | i as u64);
            }
        }
        Some(h)
    }

    fn move_score(&self, m: &Mv) -> i32 {
        const V: [i32; 7] = [0, 100, 2, 3, 3, 5, 1];
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

    fn count(g: &Makruk, d: u32) -> u64 {
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

    fn empty() -> Makruk {
        Makruk {
            b: [0; 64],
            side: Player::One,
            quiet: 0,
            count: 0,
        }
    }

    fn targets(g: &Makruk, f: i32, r: i32) -> usize {
        g.all_legal()
            .iter()
            .filter(|m| m.from as usize == idx(f, r))
            .count()
    }

    #[test]
    fn perft_start() {
        // Derived by hand: 8 pawns + Ma 1+1 + Khon 3+3 + Met 2 + Khun 3 + Rua 1+1 = 23.
        let g = Makruk::new();
        assert_eq!(count(&g, 1), 23);
        // No interaction between the sides in two plies.
        assert_eq!(count(&g, 2), 23 * 23);
    }

    #[test]
    fn setup_is_rotationally_symmetric() {
        let g = Makruk::new();
        for i in 0..64 {
            assert_eq!(g.b[i], -g.b[63 - i]);
        }
    }

    #[test]
    fn piece_moves() {
        let mut g = empty();
        g.b[idx(0, 0)] = KH;
        g.b[idx(7, 7)] = -KH;
        g.b[idx(4, 3)] = MET;
        assert_eq!(targets(&g, 4, 3), 4);
        g.b[idx(4, 3)] = KHON;
        assert_eq!(targets(&g, 4, 3), 5); // 4 diagonals + forward
        g.b[idx(4, 3)] = MA;
        assert_eq!(targets(&g, 4, 3), 8);
        g.b[idx(4, 3)] = RUA;
        assert_eq!(targets(&g, 4, 3), 14);
        g.b[idx(4, 6)] = -P; // enemy blocks the file: capture allowed, nothing beyond
        assert_eq!(targets(&g, 4, 3), 13);
    }

    #[test]
    fn pawn_moves_and_capture() {
        let mut g = empty();
        g.b[idx(0, 0)] = KH;
        g.b[idx(7, 7)] = -KH;
        g.b[idx(3, 2)] = P;
        assert_eq!(targets(&g, 3, 2), 1); // no double step
        g.b[idx(4, 3)] = -P;
        g.b[idx(2, 3)] = P; // own piece: not capturable
        assert_eq!(targets(&g, 3, 2), 2); // forward + capture e4
        g.b[idx(3, 3)] = -P; // blocked ahead
        assert_eq!(targets(&g, 3, 2), 1);
    }

    #[test]
    fn promotion_on_sixth_rank() {
        let mut g = empty();
        g.b[idx(0, 0)] = KH;
        g.b[idx(7, 7)] = -KH;
        g.b[idx(3, 4)] = P;
        g.b[idx(5, 3)] = -P;
        let m = Mv {
            from: idx(3, 4) as u8,
            to: idx(3, 5) as u8,
        };
        g.apply(&m);
        assert_eq!(g.b[idx(3, 5)], MET);
        // black pawn promotes on its sixth rank (rank 3)
        let m = Mv {
            from: idx(5, 3) as u8,
            to: idx(5, 2) as u8,
        };
        g.apply(&m);
        assert_eq!(g.b[idx(5, 2)], -MET);
        assert_eq!(g.format_move(&m), "f4f3");
    }

    #[test]
    fn check_and_mate() {
        let mut g = empty();
        g.b[idx(7, 7)] = -KH;
        g.b[idx(0, 0)] = KH;
        g.b[idx(0, 7)] = RUA;
        g.b[idx(0, 6)] = RUA;
        g.side = Player::Two;
        assert!(g.in_check(Player::Two));
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
        // the same without the second rook: the king can step away
        g.b[idx(0, 6)] = 0;
        assert_eq!(g.outcome(), None);
        assert!(g.all_legal().iter().all(|m| m.from as usize == idx(7, 7)));
    }

    #[test]
    fn stalemate_is_draw() {
        let mut g = empty();
        g.b[idx(7, 7)] = -KH;
        g.b[idx(0, 0)] = KH;
        g.b[idx(0, 6)] = RUA;
        g.b[idx(6, 0)] = RUA;
        g.side = Player::Two;
        assert!(!g.in_check(Player::Two));
        assert_eq!(g.outcome(), Some(Outcome::Draw));
    }

    #[test]
    fn cannot_move_into_check() {
        let mut g = empty();
        g.b[idx(4, 0)] = KH;
        g.b[idx(4, 7)] = -KH;
        g.b[idx(0, 1)] = -RUA;
        let moves = g.all_legal();
        assert!(moves.iter().all(|m| m.to as i32 / 8 != 1));
    }

    #[test]
    fn counting_draw() {
        let mut g = empty();
        g.b[idx(0, 0)] = KH;
        g.b[idx(7, 7)] = -KH;
        g.b[idx(3, 3)] = RUA;
        g.b[idx(4, 4)] = RUA;
        assert_eq!(g.count_limit(), 16); // two rooks: 8 moves
        g.b[idx(4, 4)] = KHON;
        assert_eq!(g.count_limit(), 32); // one rook: 16 moves
        g.b[idx(3, 3)] = KHON;
        assert_eq!(g.count_limit(), 44);
        g.b[idx(3, 3)] = MET;
        g.b[idx(4, 4)] = MET;
        assert_eq!(g.count_limit(), 128);
        g.count = 127;
        assert_eq!(g.outcome(), None);
        g.count = 128;
        assert_eq!(g.outcome(), Some(Outcome::Draw));
    }

    #[test]
    fn bare_kings_draw_and_quiet_cap() {
        let mut g = empty();
        g.b[idx(0, 0)] = KH;
        g.b[idx(7, 7)] = -KH;
        assert_eq!(g.outcome(), Some(Outcome::Draw));
        let mut g = Makruk::new();
        g.quiet = QUIET_CAP;
        assert_eq!(g.outcome(), Some(Outcome::Draw));
        assert!(g.legal_moves().is_empty());
    }

    #[test]
    fn ai_answers_fast_and_takes_free_rook() {
        let mut g = empty();
        g.b[idx(0, 0)] = KH;
        g.b[idx(7, 7)] = -KH;
        g.b[idx(3, 3)] = RUA;
        g.b[idx(3, 6)] = -RUA;
        let t = std::time::Instant::now();
        let mv = polytabletop_core::ai::best_move(&g, 1).unwrap();
        assert_eq!(g.format_move(&mv), "d4d7");
        let g = Makruk::new();
        polytabletop_core::ai::best_move(&g, 1).unwrap();
        assert!(t.elapsed().as_secs_f64() < 5.0);
    }
}
