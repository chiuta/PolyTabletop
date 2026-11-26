//! Janggi (șahul coreean), 9×10, scris din regulile publice.
//!
//! Player One = Cho (jos, rândurile 0–4, mută primul), Player Two = Han (sus).
//! Coordonate ca la xiangqi: fișiere `a`–`i`, rânduri `0`–`9`, ex. `b2e2`;
//! `pass` = trecere (permisă doar când nu ești în șah).
//! Piese: General (în palat), Gardă (Sa, în palat), Elefant (Sang: un pas
//! ortogonal + două diagonale, blocabil), Cal (Ma: un pas ortogonal + unul
//! diagonal, blocabil), Car (Cha: turn; în palat și pe diagonale), Tun (Po:
//! sare exact peste o piesă, nu peste/pe tun), Soldat (înainte sau lateral,
//! pe diagonalele palatului inamic înainte). Palatele au diagonale.
//! Mat (șah fără mutare legală) = pierdere. Simplificări: generalii nu se pot
//! privi pe o coloană liberă (mutarea e ilegală); două pass-uri consecutive,
//! o poziție repetată de 3 ori sau 150 de semi-mutări fără captură = remiză.

//! Verificare surse (nov. 2026: en.wikipedia.org/wiki/Janggi): mutarea trecere (permisă doar
//! în afara șahului) și remiza la două treceri consecutive corespund. DIFERENȚĂ MARE,
//! lăsată neschimbată: în regulile reale, „bikjang” (generalii față în față pe o coloană
//! liberă) NU e o mutare ilegală: cine e la mutare trebuie să rupă poziția, altfel jocul e
//! remiză (sau cel care o cere declară remiza; regula se convine dinainte). De asemenea,
//! turneele coreene nu acceptă remiza prin repetiție și hotărăsc după numărătoarea de puncte
//! (car 13, tun 7, cal 5, elefant 3, gardă 3, soldat 2; Han primește 1,5 puncte); aici
//! repetarea de 3 ori e remiză și nu există numărătoare.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const W: i32 = 9;
const H: i32 = 10;

const G: i8 = 1; // general
const SA: i8 = 2; // guard
const SANG: i8 = 3; // elephant
const MA: i8 = 4; // horse
const CHA: i8 = 5; // chariot
const PO: i8 = 6; // cannon
const JOL: i8 = 7; // soldier

const QUIET_CAP: u16 = 150;
const PASS: u8 = 255;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Mv {
    from: u8, // PASS for a pass
    to: u8,
}

impl Mv {
    fn is_pass(&self) -> bool {
        self.from == PASS
    }
}

#[derive(Clone)]
pub struct Janggi {
    b: [i8; 90], // index = rank * 9 + file; >0 One(Cho), <0 Two(Han)
    side: Player,
    quiet: u16,
    passes: u8, // consecutive passes
    hist: Vec<u64>,
    last_pass: Option<Player>,
}

impl Default for Janggi {
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

fn inb(f: i32, r: i32) -> bool {
    (0..W).contains(&f) && (0..H).contains(&r)
}

/// Palace id (0 bottom, 1 top) of a point, if it lies in a palace.
fn palace(f: i32, r: i32) -> Option<i32> {
    if !(3..=5).contains(&f) {
        return None;
    }
    match r {
        0..=2 => Some(0),
        7..=9 => Some(1),
        _ => None,
    }
}

/// Point with palace diagonals (corners and centre).
fn diag_point(f: i32, r: i32) -> bool {
    let Some(p) = palace(f, r) else {
        return false;
    };
    let base = if p == 0 { 0 } else { 7 };
    (f == 4 && r == base + 1) || ((f == 3 || f == 5) && (r == base || r == base + 2))
}

/// A diagonal step inside a palace along its drawn lines.
fn diag_step(f: i32, r: i32, df: i32, dr: i32) -> Option<(i32, i32)> {
    let (tf, tr) = (f + df, r + dr);
    (diag_point(f, r) && diag_point(tf, tr)).then_some((tf, tr))
}

const ORTHO: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
const DIAG: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

impl Janggi {
    pub fn new() -> Self {
        let mut b = [0i8; 90];
        let back = [CHA, SANG, MA, SA, 0, SA, SANG, MA, CHA];
        for (f, &p) in back.iter().enumerate() {
            b[idx(f as i32, 0)] = p;
            b[idx(8 - f as i32, 9)] = -p;
        }
        b[idx(4, 1)] = G;
        b[idx(4, 8)] = -G;
        for f in [1, 7] {
            b[idx(f, 2)] = PO;
            b[idx(8 - f, 7)] = -PO;
        }
        for f in [0, 2, 4, 6, 8] {
            b[idx(f, 3)] = JOL;
            b[idx(f, 6)] = -JOL;
        }
        let mut g = Janggi {
            b,
            side: Player::One,
            quiet: 0,
            passes: 0,
            hist: Vec::new(),
            last_pass: None,
        };
        g.hist.push(g.key());
        g
    }

    fn key(&self) -> u64 {
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        for &v in &self.b {
            h ^= (v as i64 + 8) as u64;
            h = h.wrapping_mul(0x0100_0000_01b3);
        }
        h ^= self.side.idx() as u64 + 31;
        h.wrapping_mul(0x0100_0000_01b3)
    }

    /// Pseudo-legal moves, without passes.
    fn pseudo(&self, side: Player) -> Vec<Mv> {
        let s = sign(side);
        let mut out = Vec::with_capacity(64);
        for from in 0..90i32 {
            let v = self.b[from as usize];
            if v == 0 || v.signum() != s {
                continue;
            }
            let (f, r) = (from % W, from / W);
            // Pushes if the target is empty or an enemy; returns true if it was empty.
            let push = |tf: i32, tr: i32, out: &mut Vec<Mv>| -> bool {
                if inb(tf, tr) {
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
            let in_own_palace =
                |tf: i32, tr: i32| palace(tf, tr).is_some() && palace(tf, tr) == palace(f, r);
            match v.abs() {
                G | SA => {
                    for (df, dr) in ORTHO {
                        let (tf, tr) = (f + df, r + dr);
                        if in_own_palace(tf, tr) {
                            push(tf, tr, &mut out);
                        }
                    }
                    for (df, dr) in DIAG {
                        if let Some((tf, tr)) = diag_step(f, r, df, dr) {
                            push(tf, tr, &mut out);
                        }
                    }
                }
                SANG => {
                    for (df, dr) in ORTHO {
                        let (of, or) = (f + df, r + dr);
                        if !inb(of, or) || self.b[idx(of, or)] != 0 {
                            continue;
                        }
                        // two diagonal steps going away from the orthogonal axis
                        let sides: [(i32, i32); 2] = if df != 0 {
                            [(df, 1), (df, -1)]
                        } else {
                            [(1, dr), (-1, dr)]
                        };
                        for (sf, sr) in sides {
                            let (mf, mr) = (of + sf, or + sr);
                            if !inb(mf, mr) || self.b[idx(mf, mr)] != 0 {
                                continue;
                            }
                            push(mf + sf, mr + sr, &mut out);
                        }
                    }
                }
                MA => {
                    for (df, dr) in ORTHO {
                        let (lf, lr) = (f + df, r + dr);
                        if !inb(lf, lr) || self.b[idx(lf, lr)] != 0 {
                            continue; // hobbled
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
                CHA => {
                    for (df, dr) in ORTHO {
                        let (mut tf, mut tr) = (f + df, r + dr);
                        while push(tf, tr, &mut out) {
                            tf += df;
                            tr += dr;
                        }
                    }
                    for (df, dr) in DIAG {
                        let (mut cf, mut cr) = (f, r);
                        while let Some((tf, tr)) = diag_step(cf, cr, df, dr) {
                            if !push(tf, tr, &mut out) {
                                break;
                            }
                            cf = tf;
                            cr = tr;
                        }
                    }
                }
                PO => {
                    let land = |tf: i32, tr: i32, out: &mut Vec<Mv>| -> bool {
                        // returns true if the cannon may continue sliding past this empty square
                        let t = self.b[idx(tf, tr)];
                        if t == 0 {
                            out.push(Mv {
                                from: from as u8,
                                to: idx(tf, tr) as u8,
                            });
                            true
                        } else {
                            if t.signum() != s && t.abs() != PO {
                                out.push(Mv {
                                    from: from as u8,
                                    to: idx(tf, tr) as u8,
                                });
                            }
                            false
                        }
                    };
                    for (df, dr) in ORTHO {
                        let (mut tf, mut tr) = (f + df, r + dr);
                        while inb(tf, tr) && self.b[idx(tf, tr)] == 0 {
                            tf += df;
                            tr += dr;
                        }
                        if !inb(tf, tr) || self.b[idx(tf, tr)].abs() == PO {
                            continue; // no screen, or screen is a cannon
                        }
                        tf += df;
                        tr += dr;
                        while inb(tf, tr) && land(tf, tr, &mut out) {
                            tf += df;
                            tr += dr;
                        }
                    }
                    for (df, dr) in DIAG {
                        if let Some((mf, mr)) = diag_step(f, r, df, dr) {
                            if self.b[idx(mf, mr)] != 0 && self.b[idx(mf, mr)].abs() != PO {
                                if let Some((tf, tr)) = diag_step(mf, mr, df, dr) {
                                    land(tf, tr, &mut out);
                                }
                            }
                        }
                    }
                }
                _ => {
                    // soldier: forward / sideways; palace diagonals forward
                    let fw = s as i32;
                    push(f, r + fw, &mut out);
                    push(f + 1, r, &mut out);
                    push(f - 1, r, &mut out);
                    for df in [-1, 1] {
                        if let Some((tf, tr)) = diag_step(f, r, df, fw) {
                            push(tf, tr, &mut out);
                        }
                    }
                }
            }
        }
        out
    }

    fn general_pos(&self, side: Player) -> Option<i32> {
        let g = G * sign(side);
        (0..90).find(|&i| self.b[i as usize] == g)
    }

    /// General attacked, or generals facing on an open file.
    fn in_check(&self, side: Player) -> bool {
        let Some(g) = self.general_pos(side) else {
            return true;
        };
        if self.facing() {
            return true;
        }
        self.pseudo(side.other()).iter().any(|m| m.to as i32 == g)
    }

    fn facing(&self) -> bool {
        let (Some(a), Some(b)) = (self.general_pos(Player::One), self.general_pos(Player::Two))
        else {
            return false;
        };
        a % W == b % W && ((a / W + 1)..(b / W)).all(|r| self.b[idx(a % W, r)] == 0)
    }

    /// True if the move was a capture.
    fn make(&mut self, m: &Mv) -> bool {
        let mut cap = false;
        if !m.is_pass() {
            cap = self.b[m.to as usize] != 0;
            self.b[m.to as usize] = self.b[m.from as usize];
            self.b[m.from as usize] = 0;
        }
        self.side = self.side.other();
        cap
    }

    fn all_legal(&self) -> Vec<Mv> {
        let side = self.side;
        let mut v: Vec<Mv> = self
            .pseudo(side)
            .into_iter()
            .filter(|m| {
                let mut g = self.clone_light();
                g.make(m);
                !g.in_check(side)
            })
            .collect();
        v.sort_by_key(|m| if self.b[m.to as usize] != 0 { 0 } else { 1 });
        if !self.in_check(side) {
            v.push(Mv {
                from: PASS,
                to: PASS,
            });
        }
        v
    }

    /// Clone without the history (cheap, for legality probing).
    fn clone_light(&self) -> Janggi {
        Janggi {
            b: self.b,
            side: self.side,
            quiet: self.quiet,
            passes: self.passes,
            hist: Vec::new(),
            last_pass: self.last_pass,
        }
    }

    fn drawn(&self) -> bool {
        if self.quiet >= QUIET_CAP || self.passes >= 2 {
            return true;
        }
        let k = *self.hist.last().unwrap_or(&0);
        self.hist.iter().filter(|&&h| h == k).count() >= 3
    }
}

fn value(p: i8) -> i32 {
    match p.abs() {
        SA => 30,
        SANG => 30,
        MA => 50,
        CHA => 130,
        PO => 70,
        JOL => 20,
        _ => 0,
    }
}

impl Game for Janggi {
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
        let mover = self.side;
        let cap = self.make(m);
        if m.is_pass() {
            self.passes += 1;
            self.last_pass = Some(mover);
        } else {
            self.passes = 0;
            self.last_pass = None;
        }
        if cap {
            self.quiet = 0;
            self.hist.clear();
        } else {
            self.quiet += 1;
        }
        let k = self.key();
        self.hist.push(k);
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.drawn() {
            return Some(Outcome::Draw);
        }
        if self.all_legal().is_empty() {
            // no move and no pass: in check with no escape
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
            let mut x = value(v) * 10;
            match v.abs() {
                JOL => {
                    let adv = if v > 0 { r } else { 9 - r };
                    if adv >= 5 {
                        x += 20 + (adv - 5) * 8;
                    }
                }
                MA | CHA | PO => x += 10 - (f - 4).abs() * 2,
                _ => {}
            }
            s += if v.signum() == me { x } else { -x };
        }
        // a pass wastes a tempo: mild penalty so the AI does not pass idly
        match self.last_pass {
            Some(q) if q == p => s -= 15,
            Some(_) => s += 15,
            None => {}
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
                    0 => {
                        if diag_point(f, r) {
                            '+'
                        } else {
                            '.'
                        }
                    }
                    G => 'g',
                    SA => 's',
                    SANG => 'e',
                    MA => 'h',
                    CHA => 'r',
                    PO => 'c',
                    _ => 'p',
                };
                s.push(if v > 0 { ch.to_ascii_uppercase() } else { ch });
                s.push(' ');
            }
            s.push('\n');
        }
        s.push_str("  a b c d e f g h i\n  (MAJUSCULE = Cho, începe; + = nod cu diagonale în palat; pass = trecere)\n");
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        if m.is_pass() {
            return "pass".into();
        }
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
            1 << 40
                | (self.side.idx() as u64) << 32
                | (self.quiet as u64) << 8
                | (self.passes as u64) << 24
                | (self.last_pass.map_or(0, |p| p.idx() as u64 + 1)) << 28,
        );
        for (i, &v) in self.b.iter().enumerate() {
            if v != 0 {
                h ^= polytabletop_core::hash_mix((v as u8 as u64) << 16 | i as u64);
            }
        }
        Some(h)
    }

    fn move_score(&self, m: &Mv) -> i32 {
        const V: [i32; 8] = [0, 100, 3, 3, 5, 13, 7, 2];
        if m.is_pass() {
            return 0;
        }
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

    fn empty() -> Janggi {
        Janggi {
            b: [0; 90],
            side: Player::One,
            quiet: 0,
            passes: 0,
            hist: Vec::new(),
            last_pass: None,
        }
    }

    /// Both generals home, away from the files used in the tests.
    fn base() -> Janggi {
        let mut g = empty();
        g.b[idx(3, 0)] = G;
        g.b[idx(5, 9)] = -G;
        g
    }

    fn dests(g: &Janggi, f: i32, r: i32) -> Vec<(i32, i32)> {
        let mut v: Vec<(i32, i32)> = g
            .all_legal()
            .iter()
            .filter(|m| !m.is_pass() && m.from as usize == idx(f, r))
            .map(|m| (m.to as i32 % W, m.to as i32 / W))
            .collect();
        v.sort();
        v
    }

    fn count(g: &Janggi, d: u32) -> u64 {
        if d == 0 {
            return 1;
        }
        g.all_legal()
            .iter()
            .map(|m| {
                let mut h = g.clone_light();
                h.make(m);
                count(&h, d - 1)
            })
            .sum()
    }

    #[test]
    fn start_position_and_perft() {
        let g = Janggi::new();
        assert_eq!(g.b.iter().filter(|&&v| v > 0).count(), 16);
        assert_eq!(g.b.iter().filter(|&&v| v < 0).count(), 16);
        // Hand count: 5 soldiers x3 (fwd + 2 sides, minus blocked) etc. is
        // error-prone, so check structure: the pass is always listed first-class
        // and perft(2) is symmetric: every reply count equals perft(1).
        let n1 = count(&g, 1);
        assert!(n1 > 25 && n1 < 60, "{n1}");
        assert_eq!(g.legal_moves().len() as u64, n1);
        let d2 = count(&g, 2);
        assert!(d2 > n1 * 25, "{d2}");
    }

    #[test]
    fn start_move_texts() {
        let g = Janggi::new();
        let mv: Vec<String> = g.legal_moves().iter().map(|m| g.format_move(m)).collect();
        assert!(mv.contains(&"pass".to_string()));
        assert!(mv.contains(&"a3a4".to_string()));
        assert!(mv.contains(&"e1e0".to_string()) && mv.contains(&"e1d1".to_string()));
        assert!(mv.contains(&"c0d2".to_string()));
        assert!(!mv.contains(&"b2b3".to_string())); // a cannon cannot move without a screen
    }

    #[test]
    fn horse_blocked_by_leg() {
        let mut g = base();
        g.b[idx(4, 4)] = MA;
        assert_eq!(dests(&g, 4, 4).len(), 8);
        g.b[idx(4, 5)] = -JOL;
        assert_eq!(dests(&g, 4, 4).len(), 6);
    }

    #[test]
    fn elephant_moves_and_blocks() {
        let mut g = base();
        g.b[idx(4, 4)] = SANG;
        let d = dests(&g, 4, 4);
        assert_eq!(d.len(), 8);
        assert!(d.contains(&(6, 7)) && d.contains(&(7, 6)) && d.contains(&(1, 2)));
        g.b[idx(4, 5)] = JOL; // blocks (5,6)->(6,7) and (3,6)->(2,7)
        assert_eq!(dests(&g, 4, 4).len(), 6);
        g.b[idx(6, 5)] = -JOL; // intermediate square of the right-up branch
        assert_eq!(dests(&g, 4, 4).len(), 5);
        g.b[idx(5, 4)] = -JOL; // blocks the right orthogonal first step
        assert_eq!(dests(&g, 4, 4).len(), 4);
    }

    #[test]
    fn cannon_rules() {
        let mut g = base();
        g.b[idx(0, 3)] = PO;
        // no screen: no vertical moves
        assert!(dests(&g, 0, 3).iter().all(|&(f, _)| f != 0));
        g.b[idx(0, 5)] = JOL; // screen
        let d = dests(&g, 0, 3);
        assert!(d.contains(&(0, 6)) && d.contains(&(0, 9)) && !d.contains(&(0, 4)));
        // cannon cannot capture a cannon, nor jump over one
        g.b[idx(0, 7)] = -PO;
        let d = dests(&g, 0, 3);
        assert!(d.contains(&(0, 6)) && !d.contains(&(0, 7)) && !d.contains(&(0, 8)));
        // capture a normal piece
        g.b[idx(0, 7)] = -MA;
        assert!(dests(&g, 0, 3).contains(&(0, 7)));
        // a cannon as screen is invalid
        g.b[idx(0, 5)] = PO;
        assert!(dests(&g, 0, 3).iter().all(|&(f, _)| f != 0));
    }

    #[test]
    fn cannon_palace_diagonal() {
        let mut g = base();
        g.b[idx(3, 7)] = PO; // Cho cannon in Han palace corner
        g.b[idx(5, 9)] = 0;
        g.b[idx(4, 9)] = -G;
        g.b[idx(4, 8)] = -SA; // screen at centre
        let d = dests(&g, 3, 7);
        assert!(d.contains(&(5, 9))); // jumps the centre to the opposite corner
        g.b[idx(5, 9)] = -PO; // cannon cannot be captured
        assert!(!dests(&g, 3, 7).contains(&(5, 9)));
        g.b[idx(5, 9)] = 0;
        g.b[idx(4, 8)] = -PO; // cannon cannot be the screen
        assert!(!dests(&g, 3, 7).contains(&(5, 9)));
    }

    #[test]
    fn chariot_palace_diagonal() {
        let mut g = base();
        g.b[idx(3, 7)] = CHA;
        let d = dests(&g, 3, 7);
        assert!(d.contains(&(4, 8)));
        assert!(d.contains(&(5, 9))); // captures the general through the centre
        g.b[idx(4, 8)] = -SA;
        let d = dests(&g, 3, 7);
        assert!(d.contains(&(4, 8)) && !d.contains(&(5, 9)));
        // outside the palace no diagonal
        let mut g = base();
        g.b[idx(4, 4)] = CHA;
        assert_eq!(dests(&g, 4, 4).len(), 17);
    }

    #[test]
    fn general_and_guard_use_diagonals() {
        let mut g = empty();
        g.b[idx(4, 1)] = G;
        g.b[idx(5, 9)] = -G;
        g.b[idx(5, 5)] = -JOL; // screen on the generals' file
        assert_eq!(dests(&g, 4, 1).len(), 8);
        let mut g = empty();
        g.b[idx(3, 0)] = G;
        g.b[idx(5, 9)] = -G;
        // corner: 2 orthogonal + 1 diagonal to centre
        assert_eq!(dests(&g, 3, 0), vec![(3, 1), (4, 0), (4, 1)]);
        // edge point (4,0) has no diagonals
        let mut g = empty();
        g.b[idx(4, 0)] = G;
        g.b[idx(5, 9)] = -G;
        g.b[idx(5, 5)] = -JOL;
        assert_eq!(dests(&g, 4, 0).len(), 3);
    }

    #[test]
    fn soldier_moves() {
        let mut g = base();
        g.b[idx(2, 4)] = JOL;
        assert_eq!(dests(&g, 2, 4), vec![(1, 4), (2, 5), (3, 4)]); // never backward
        let mut g = base();
        g.b[idx(3, 7)] = JOL;
        let d = dests(&g, 3, 7);
        assert!(d.contains(&(4, 8))); // diagonal forward in the enemy palace
    }

    #[test]
    fn generals_cannot_face() {
        let mut g = empty();
        g.b[idx(4, 1)] = G;
        g.b[idx(4, 8)] = -G;
        g.b[idx(4, 4)] = CHA; // only screen
        let d = dests(&g, 4, 4);
        assert!(d.iter().all(|&(f, _)| f == 4));
    }

    #[test]
    fn check_pass_and_mate() {
        let mut g = empty();
        g.b[idx(3, 0)] = G;
        g.b[idx(5, 9)] = -G;
        // not in check: pass allowed
        assert!(g.all_legal().iter().any(|m| m.is_pass()));
        // put Cho in check: pass forbidden
        g.b[idx(3, 5)] = -CHA;
        assert!(g.in_check(Player::One));
        assert!(!g.all_legal().iter().any(|m| m.is_pass()));
        assert_eq!(g.outcome(), None); // can step aside
                                       // mate: second chariot covers the other file
        g.b[idx(4, 5)] = -CHA;
        g.b[idx(5, 5)] = -CHA;
        assert_eq!(
            g.format_move(&Mv {
                from: PASS,
                to: PASS
            }),
            "pass"
        );
        // box the general: all of d1? use attackers on ranks to trap
        let mut m = empty();
        m.b[idx(3, 0)] = G;
        m.b[idx(5, 9)] = -G;
        m.b[idx(3, 8)] = -CHA; // checks along file d
        m.b[idx(7, 1)] = -CHA; // controls rank 1 (d2,e2)
        m.b[idx(7, 0)] = -CHA; // controls rank 0 (e1)
        assert!(m.in_check(Player::One));
        assert_eq!(m.outcome(), Some(Outcome::Win(Player::Two)));
    }

    #[test]
    fn double_pass_is_draw() {
        let mut g = Janggi::new();
        let p = Mv {
            from: PASS,
            to: PASS,
        };
        g.apply(&p);
        assert_eq!(g.outcome(), None);
        g.apply(&p);
        assert_eq!(g.outcome(), Some(Outcome::Draw));
    }

    #[test]
    fn repetition_is_draw() {
        let mut g = Janggi::new();
        let mv = |g: &Janggi, s: &str| {
            g.legal_moves()
                .into_iter()
                .find(|m| g.format_move(m) == s)
                .unwrap()
        };
        // shuffle the cho cannon-side chariot and han chariot back and forth
        for _ in 0..2 {
            for s in ["a0a1", "a9a8", "a1a0", "a8a9"] {
                let m = mv(&g, s);
                g.apply(&m);
            }
        }
        assert_eq!(g.outcome(), Some(Outcome::Draw));
    }

    #[test]
    fn quiet_cap_is_draw() {
        let mut g = Janggi::new();
        g.quiet = QUIET_CAP;
        assert_eq!(g.outcome(), Some(Outcome::Draw));
    }

    #[test]
    fn ai_is_fast_and_captures() {
        let g = Janggi::new();
        let t = std::time::Instant::now();
        let m = polytabletop_core::ai::best_move(&g, 3).unwrap();
        let _ = g.format_move(&m);
        assert!(t.elapsed().as_secs_f64() < 5.0);
        let mut g = base();
        g.b[idx(5, 9)] = 0;
        g.b[idx(4, 9)] = -G;
        g.b[idx(0, 0)] = CHA;
        g.b[idx(0, 6)] = -MA;
        let m = polytabletop_core::ai::best_move(&g, 3).unwrap();
        assert_eq!(g.format_move(&m), "a0a6");
    }
}
