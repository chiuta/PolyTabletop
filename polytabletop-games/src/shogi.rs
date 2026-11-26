//! Shogi (șahul japonez), 9×9, scris doar din regulile publice.
//!
//! Player One = sente (negru, jos, rândurile g–i), Player Two = gote (sus).
//! Coordonate tradiționale: fișiere `1`–`9` (de la dreapta la stânga, văzute de
//! sente), rânduri `a`–`i` (de sus în jos). Mutări: `7g7f`, `2b8h+` (cu
//! promovare), drop `P*5e`.
//!
//! Reguli: piesele capturate trec în mâna capturantului și pot fi puse înapoi
//! (drop). Drop-uri interzise: pe pătrat ocupat; pion/lance pe ultimul rând și
//! cal pe ultimele două (n-ar mai avea mutări); doi pioni nepromovați pe
//! același fișier (nifu); pion care dă mat imediat (uchifuzume).
//! Promovarea (opțională în zona adversă, obligatorie când piesa n-ar mai avea
//! mutări) schimbă piesa; capturată, piesa revine la forma ei de bază.
//! Mat = pierdere. Simplificări: repetiția (sennichite) = remiză după a patra
//! apariție a aceleiași poziții (șahul perpetuu nu se pedepsește), iar partida
//! se încheie remiză după [`MAX_PLIES`] semi-mutări.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

/// Plafon de semi-mutări după care partida este remiză.
pub const MAX_PLIES: u16 = 320;

const P: i8 = 1;
const L: i8 = 2;
const N: i8 = 3;
const S: i8 = 4;
const G: i8 = 5;
const B: i8 = 6;
const R: i8 = 7;
const K: i8 = 8;
/// Promovatele: bază + 8 (tokin 9, lance 10, cal 11, argint 12, cal-dragon 14, dragon 15).
const PROMO: i8 = 8;

const NOWHERE: u8 = 81;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Mv {
    /// Pătrat sursă (`rând*9+coloană`) sau 81 pentru drop.
    from: u8,
    to: u8,
    promo: bool,
    /// Piesa pusă (doar la drop), 1..=7.
    drop: i8,
}

#[derive(Clone)]
pub struct Shogi {
    /// Index = rând*9 + coloană; rând 0 = `a` (sus), coloană 0 = fișierul 9.
    b: [i8; 81],
    /// hands[jucător][piesă-1] pentru P L N S G B R.
    hands: [[u8; 7]; 2],
    side: Player,
    plies: u16,
    hist: Vec<u64>,
}

impl Default for Shogi {
    fn default() -> Self {
        Self::new()
    }
}

fn sign(p: Player) -> i8 {
    match p {
        Player::One => 1,
        Player::Two => -1,
    }
}

type Vecs = &'static [(i32, i32)];

const GOLD: Vecs = &[(0, 1), (-1, 1), (1, 1), (-1, 0), (1, 0), (0, -1)];
const ORTH: Vecs = &[(0, 1), (0, -1), (1, 0), (-1, 0)];
const DIAG: Vecs = &[(1, 1), (1, -1), (-1, 1), (-1, -1)];
const KING: Vecs = &[
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

/// (pași, glisări) în coordonate (coloană, „înainte”).
fn pattern(kind: i8) -> (Vecs, Vecs) {
    match kind {
        P => (&[(0, 1)], &[]),
        L => (&[], &[(0, 1)]),
        N => (&[(-1, 2), (1, 2)], &[]),
        S => (&[(0, 1), (-1, 1), (1, 1), (-1, -1), (1, -1)], &[]),
        G => (GOLD, &[]),
        B => (&[], DIAG),
        R => (&[], ORTH),
        K => (KING, &[]),
        9..=12 => (GOLD, &[]),
        14 => (ORTH, DIAG),
        15 => (DIAG, ORTH),
        _ => (&[], &[]),
    }
}

fn in_zone(r: i32, s: i8) -> bool {
    if s > 0 {
        r <= 2
    } else {
        r >= 6
    }
}

/// Rândul „înainte” relativ (0 = ultimul rând din fața piesei).
fn rows_left(r: i32, s: i8) -> i32 {
    if s > 0 {
        r
    } else {
        8 - r
    }
}

fn promotable(kind: i8) -> bool {
    matches!(kind, P | L | N | S | B | R)
}

/// Este `sq` atacat de piesele cu semnul `by`?
fn attacked(b: &[i8; 81], sq: i32, by: i8) -> bool {
    let (tc, tr) = (sq % 9, sq / 9);
    for i in 0..81i32 {
        let v = b[i as usize];
        if v == 0 || v.signum() != by {
            continue;
        }
        let (c, r) = (i % 9, i / 9);
        let (steps, slides) = pattern(v.abs());
        let dir = -(by as i32); // „înainte” pe axa rândurilor
        for &(dc, df) in steps {
            if c + dc == tc && r + df * dir == tr {
                return true;
            }
        }
        for &(dc, df) in slides {
            let (mut x, mut y) = (c + dc, r + df * dir);
            while (0..9).contains(&x) && (0..9).contains(&y) {
                if x == tc && y == tr {
                    return true;
                }
                if b[(y * 9 + x) as usize] != 0 {
                    break;
                }
                x += dc;
                y += df * dir;
            }
        }
    }
    false
}

fn king_in_check(b: &[i8; 81], side: i8) -> bool {
    match (0..81).find(|&i| b[i as usize] == K * side) {
        Some(k) => attacked(b, k, -side),
        None => true,
    }
}

impl Shogi {
    pub fn new() -> Self {
        let mut b = [0i8; 81];
        let back = [L, N, S, G, K, G, S, N, L];
        for (c, &p) in back.iter().enumerate() {
            b[c] = -p;
            b[72 + c] = p;
        }
        b[9 + 1] = -R;
        b[9 + 7] = -B;
        b[63 + 1] = B;
        b[63 + 7] = R;
        for c in 0..9 {
            b[18 + c] = -P;
            b[54 + c] = P;
        }
        let mut g = Shogi {
            b,
            hands: [[0; 7]; 2],
            side: Player::One,
            plies: 0,
            hist: Vec::new(),
        };
        g.hist.push(g.hash());
        g
    }

    fn hash(&self) -> u64 {
        let mut h: u64 = 0xcbf29ce484222325;
        let mut mix = |x: u64| {
            h ^= x;
            h = h.wrapping_mul(0x100000001b3);
        };
        for &v in &self.b {
            mix((v + 20) as u64);
        }
        for p in 0..2 {
            for &n in &self.hands[p] {
                mix(n as u64 + 100);
            }
        }
        mix(self.side.idx() as u64 + 200);
        h
    }

    fn drawn(&self) -> bool {
        if self.plies >= MAX_PLIES {
            return true;
        }
        let last = *self.hist.last().unwrap();
        self.hist.iter().filter(|&&h| h == last).count() >= 4
    }

    /// Mutări pseudo-legale (regele propriu poate rămâne în șah), fără testul uchifuzume.
    fn pseudo(&self, side: Player) -> Vec<Mv> {
        let s = sign(side);
        let dir = -(s as i32);
        let mut out = Vec::with_capacity(128);
        for from in 0..81i32 {
            let v = self.b[from as usize];
            if v == 0 || v.signum() != s {
                continue;
            }
            let kind = v.abs();
            let (c, r) = (from % 9, from / 9);
            let (steps, slides) = pattern(kind);
            let add = |x: i32, y: i32, out: &mut Vec<Mv>| {
                let t = self.b[(y * 9 + x) as usize];
                if t.signum() == s {
                    return false;
                }
                let to = (y * 9 + x) as u8;
                let left = rows_left(y, s);
                let forced = match kind {
                    P | L => left == 0,
                    N => left <= 1,
                    _ => false,
                };
                let can = promotable(kind) && (in_zone(r, s) || in_zone(y, s));
                if !forced {
                    out.push(Mv {
                        from: from as u8,
                        to,
                        promo: false,
                        drop: 0,
                    });
                }
                if can {
                    out.push(Mv {
                        from: from as u8,
                        to,
                        promo: true,
                        drop: 0,
                    });
                }
                t == 0
            };
            for &(dc, df) in steps {
                let (x, y) = (c + dc, r + df * dir);
                if (0..9).contains(&x) && (0..9).contains(&y) {
                    add(x, y, &mut out);
                }
            }
            for &(dc, df) in slides {
                let (mut x, mut y) = (c + dc, r + df * dir);
                while (0..9).contains(&x) && (0..9).contains(&y) {
                    if !add(x, y, &mut out) {
                        break;
                    }
                    x += dc;
                    y += df * dir;
                }
            }
        }
        // drop-uri
        let hand = &self.hands[side.idx()];
        let mut pawn_file = [false; 9];
        for i in 0..81usize {
            if self.b[i] == P * s {
                pawn_file[i % 9] = true;
            }
        }
        for kind in 1..=7i8 {
            if hand[(kind - 1) as usize] == 0 {
                continue;
            }
            for to in 0..81i32 {
                if self.b[to as usize] != 0 {
                    continue;
                }
                let left = rows_left(to / 9, s);
                let ok = match kind {
                    P => left > 0 && !pawn_file[(to % 9) as usize],
                    L => left > 0,
                    N => left > 1,
                    _ => true,
                };
                if ok {
                    out.push(Mv {
                        from: NOWHERE,
                        to: to as u8,
                        promo: false,
                        drop: kind,
                    });
                }
            }
        }
        out
    }

    /// Plasă tabla după mutare (fără mâini), pentru testul de șah.
    fn board_after(&self, m: &Mv) -> [i8; 81] {
        let mut b = self.b;
        let s = sign(self.side);
        if m.from == NOWHERE {
            b[m.to as usize] = m.drop * s;
        } else {
            let v = b[m.from as usize];
            b[m.to as usize] = if m.promo { v + PROMO * s } else { v };
            b[m.from as usize] = 0;
        }
        b
    }

    fn make(&mut self, m: &Mv) -> bool {
        let s = sign(self.side);
        let me = self.side.idx();
        let mut cap = false;
        if m.from == NOWHERE {
            self.hands[me][(m.drop - 1) as usize] -= 1;
            self.b[m.to as usize] = m.drop * s;
        } else {
            let t = self.b[m.to as usize];
            if t != 0 {
                cap = true;
                let base = if t.abs() > K {
                    t.abs() - PROMO
                } else {
                    t.abs()
                };
                self.hands[me][(base - 1) as usize] += 1;
            }
            let v = self.b[m.from as usize];
            self.b[m.to as usize] = if m.promo { v + PROMO * s } else { v };
            self.b[m.from as usize] = 0;
        }
        self.side = self.side.other();
        cap
    }

    fn all_legal(&self) -> Vec<Mv> {
        let side = self.side;
        let s = sign(side);
        let mut v: Vec<Mv> = Vec::with_capacity(96);
        for m in self.pseudo(side) {
            let nb = self.board_after(&m);
            if king_in_check(&nb, s) {
                continue;
            }
            if m.from == NOWHERE && m.drop == P && self.pawn_drop_mates(&m, nb) {
                continue;
            }
            v.push(m);
        }
        v.sort_by_key(|m| {
            let cap = self.b[m.to as usize] != 0;
            match (cap, m.promo) {
                (true, _) => 0,
                (_, true) => 1,
                _ => 2,
            }
        });
        v
    }

    /// Uchifuzume: pionul pus dă mat adversarului?
    fn pawn_drop_mates(&self, m: &Mv, nb: [i8; 81]) -> bool {
        let opp = -sign(self.side);
        if !king_in_check(&nb, opp) {
            return false;
        }
        let mut g = self.clone();
        g.make(m);
        g.all_legal().is_empty()
    }
}

fn value(kind: i8) -> i32 {
    match kind {
        P => 100,
        L => 300,
        N => 350,
        S => 450,
        G => 500,
        B => 800,
        R => 1000,
        9 => 520,
        10..=12 => 500,
        14 => 1000,
        15 => 1200,
        _ => 0,
    }
}

fn glyph(kind: i8) -> &'static str {
    match kind {
        P => "P",
        L => "L",
        N => "N",
        S => "S",
        G => "G",
        B => "B",
        R => "R",
        K => "K",
        9 => "+P",
        10 => "+L",
        11 => "+N",
        12 => "+S",
        14 => "+B",
        15 => "+R",
        _ => "?",
    }
}

fn sq_name(i: u8) -> String {
    format!("{}{}", 9 - (i % 9), (b'a' + i / 9) as char)
}

impl Game for Shogi {
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
        self.make(m);
        self.plies += 1;
        let h = self.hash();
        self.hist.push(h);
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.drawn() {
            return Some(Outcome::Draw);
        }
        if self.all_legal().is_empty() {
            return Some(Outcome::Win(self.side.other()));
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = sign(p);
        let mut s = 0;
        for i in 0..81i32 {
            let v = self.b[i as usize];
            if v == 0 {
                continue;
            }
            let kind = v.abs();
            let mut x = value(kind);
            let adv = if v > 0 { 8 - i / 9 } else { i / 9 };
            if matches!(kind, P | L | N | S | G) {
                x += adv * 3;
            }
            if matches!(kind, B | R) {
                x += 3 - ((i % 9) - 4).abs();
            }
            s += if v.signum() == me { x } else { -x };
        }
        for (pi, hand) in self.hands.iter().enumerate() {
            let mine = pi == p.idx();
            for (k, &n) in hand.iter().enumerate() {
                let x = (value(k as i8 + 1) + value(k as i8 + 1) / 8) * n as i32;
                s += if mine { x } else { -x };
            }
        }
        s
    }

    fn render(&self) -> String {
        let hand_line = |p: Player| {
            let mut t = String::new();
            for k in (0..7).rev() {
                let n = self.hands[p.idx()][k];
                if n > 0 {
                    t.push_str(&format!(
                        " {}{}",
                        glyph(k as i8 + 1),
                        if n > 1 { n.to_string() } else { String::new() }
                    ));
                }
            }
            if t.is_empty() {
                t.push_str(" -");
            }
            t
        };
        let mut s = format!("gote (minuscule) în mână:{}\n", hand_line(Player::Two));
        s.push_str("   9  8  7  6  5  4  3  2  1\n");
        for r in 0..9 {
            s.push_str(&format!(" {}", (b'a' + r as u8) as char));
            for c in 0..9 {
                let v = self.b[r * 9 + c];
                if v == 0 {
                    s.push_str(" . ");
                } else {
                    let g = glyph(v.abs());
                    let g = if v < 0 {
                        g.to_lowercase()
                    } else {
                        g.to_string()
                    };
                    if g.len() == 1 {
                        s.push_str(&format!(" {g} "));
                    } else {
                        s.push_str(&format!("{g} "));
                    }
                }
            }
            s.push('\n');
        }
        s.push_str(&format!(
            "sente (MAJUSCULE, începe) în mână:{}\n",
            hand_line(Player::One)
        ));
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        if m.from == NOWHERE {
            format!("{}*{}", glyph(m.drop), sq_name(m.to))
        } else {
            format!(
                "{}{}{}",
                sq_name(m.from),
                sq_name(m.to),
                if m.promo { "+" } else { "" }
            )
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 500,
            max_depth: 10,
        }
    }

    fn hash_key(&self) -> Option<u64> {
        // Zobrist-style key computed on demand: pieces on the board and in hand.
        let mut h = polytabletop_core::hash_mix(1 << 40 | (self.side.idx() as u64) << 32);
        for (i, &v) in self.b.iter().enumerate() {
            if v != 0 {
                h ^= polytabletop_core::hash_mix((v as u8 as u64) << 16 | i as u64);
            }
        }
        for p in 0..2 {
            for (k, &n) in self.hands[p].iter().enumerate() {
                if n != 0 {
                    h ^= polytabletop_core::hash_mix(
                        2 << 40 | (p as u64) << 32 | (k as u64) << 24 | n as u64,
                    );
                }
            }
        }
        Some(h)
    }

    fn move_score(&self, m: &Mv) -> i32 {
        const V: [i32; 16] = [0, 1, 3, 3, 5, 6, 8, 10, 100, 6, 6, 6, 6, 0, 9, 12];
        let mut s = if m.promo { 40 } else { 0 };
        if m.from != NOWHERE {
            let victim = self.b[m.to as usize].unsigned_abs() as usize;
            if victim != 0 {
                let piece = self.b[m.from as usize].unsigned_abs() as usize;
                s += 100 + V[victim] * 10 - V[piece];
            }
        }
        s
    }

    fn move_key(&self, m: &Mv) -> Option<u32> {
        Some(m.from as u32 * 128 + m.to as u32 + if m.drop != 0 { m.drop as u32 * 8192 } else { 0 })
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

    fn count(g: &Shogi, d: u32) -> u64 {
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

    /// Tablă goală cu regii: sente 5i, gote 5a.
    fn empty() -> Shogi {
        let mut g = Shogi::new();
        g.b = [0; 81];
        g.hands = [[0; 7]; 2];
        put(&mut g, "5i", K);
        put(&mut g, "5a", -K);
        g.hist = vec![g.hash()];
        g
    }

    fn put(g: &mut Shogi, sq: &str, v: i8) {
        let f = sq.as_bytes()[0] - b'0';
        let r = sq.as_bytes()[1] - b'a';
        g.b[r as usize * 9 + (9 - f) as usize] = v;
    }

    fn texts(g: &Shogi) -> Vec<String> {
        g.legal_moves().iter().map(|m| g.format_move(m)).collect()
    }

    fn play(g: &mut Shogi, t: &str) {
        let m = g
            .legal_moves()
            .into_iter()
            .find(|m| g.format_move(m) == t)
            .unwrap_or_else(|| panic!("mutare ilegală {t}\n{}", g.render()));
        g.apply(&m);
    }

    #[test]
    fn perft_start() {
        let g = Shogi::new();
        assert_eq!(count(&g, 1), 30);
        assert_eq!(count(&g, 2), 900);
        assert_eq!(count(&g, 3), 25470);
        assert_eq!(count(&g, 4), 719731);
    }

    #[test]
    fn nifu_forbidden() {
        let mut g = empty();
        put(&mut g, "5g", P);
        g.hands[0][0] = 1;
        let t = texts(&g);
        assert!(!t.iter().any(|m| m.starts_with("P*5")));
        assert!(t.contains(&"P*4e".to_string()));
        // un pion promovat pe fișier nu blochează drop-ul
        put(&mut g, "5g", P + PROMO);
        assert!(texts(&g).contains(&"P*5e".to_string()));
    }

    #[test]
    fn dead_end_drops_forbidden() {
        let mut g = empty();
        g.hands[0] = [1, 1, 1, 0, 0, 0, 0];
        let t = texts(&g);
        assert!(!t.contains(&"P*1a".to_string()));
        assert!(!t.contains(&"L*1a".to_string()));
        assert!(!t.contains(&"N*1a".to_string()));
        assert!(!t.contains(&"N*1b".to_string()));
        assert!(t.contains(&"N*1c".to_string()));
        assert!(t.contains(&"L*1b".to_string()));
        // gote: rândurile h, i sunt ultimele
        let mut g = empty();
        g.side = Player::Two;
        g.hands[1] = [1, 1, 1, 0, 0, 0, 0];
        let t = texts(&g);
        assert!(!t.contains(&"P*1i".to_string()));
        assert!(!t.contains(&"N*1h".to_string()));
        assert!(t.contains(&"N*1g".to_string()));
    }

    #[test]
    fn pawn_drop_mate_forbidden() {
        let mut g = empty();
        put(&mut g, "5a", 0);
        put(&mut g, "9a", -K);
        put(&mut g, "7b", G);
        put(&mut g, "9i", L);
        g.hands[0][0] = 1;
        assert!(!texts(&g).contains(&"P*9b".to_string()));
        // fără lance, regele poate lua pionul: drop legal
        put(&mut g, "9i", 0);
        assert!(texts(&g).contains(&"P*9b".to_string()));
    }

    #[test]
    fn gold_drop_mate_wins() {
        let mut g = empty();
        put(&mut g, "5a", 0);
        put(&mut g, "9a", -K);
        put(&mut g, "7b", G);
        put(&mut g, "9i", L);
        g.hands[0][4] = 1;
        play(&mut g, "G*9b");
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
        assert!(g.legal_moves().is_empty());
    }

    #[test]
    fn promotion_rules() {
        let mut g = empty();
        put(&mut g, "1b", P);
        put(&mut g, "3c", N);
        put(&mut g, "7d", S);
        put(&mut g, "8e", G);
        let t = texts(&g);
        assert!(t.contains(&"1b1a+".to_string()));
        assert!(!t.contains(&"1b1a".to_string()));
        assert!(t.contains(&"3c2a+".to_string()) && !t.contains(&"3c2a".to_string()));
        assert!(t.contains(&"7d7c".to_string()) && t.contains(&"7d7c+".to_string()));
        assert!(t.contains(&"7d6e".to_string()) && !t.contains(&"7d6e+".to_string()));
        assert!(
            t.contains(&"8e8d".to_string())
                && !t.iter().any(|m| m.starts_with("8e") && m.ends_with('+'))
        );
        // calul care ajunge pe rândul c (a treia linie) promovează opțional
        let mut g = empty();
        put(&mut g, "3e", N);
        let t = texts(&g);
        assert!(t.contains(&"3e2c".to_string()) && t.contains(&"3e2c+".to_string()));
    }

    #[test]
    fn capture_goes_to_hand_unpromoted() {
        let mut g = empty();
        put(&mut g, "2h", R);
        put(&mut g, "2e", -(P + PROMO));
        play(&mut g, "2h2e");
        assert_eq!(g.hands[0][0], 1);
        assert_eq!(g.hands[0][6], 0);
        assert_eq!(g.side, Player::Two);
        // captura unui dragon dă tură
        let mut g = empty();
        put(&mut g, "2h", R);
        put(&mut g, "2e", -(R + PROMO));
        play(&mut g, "2h2e");
        assert_eq!(g.hands[0][6], 1);
    }

    #[test]
    fn start_example_moves_exist() {
        let mut g = Shogi::new();
        let t = texts(&g);
        assert!(t.contains(&"7g7f".to_string()));
        play(&mut g, "7g7f");
        play(&mut g, "3c3d");
        play(&mut g, "8h2b+"); // schimb de nebuni
        assert_eq!(g.hands[0][5], 1);
    }

    #[test]
    fn pinned_piece_cannot_move() {
        let mut g = empty();
        put(&mut g, "5h", G);
        put(&mut g, "5c", -R);
        let t = texts(&g);
        assert!(t
            .iter()
            .filter(|m| m.starts_with("5h"))
            .all(|m| m.as_bytes()[2] == b'5'));
    }

    #[test]
    fn fourfold_repetition_is_draw() {
        let mut g = Shogi::new();
        for _ in 0..3 {
            for m in ["2h3h", "8b7b", "3h2h", "7b8b"] {
                if g.outcome().is_some() {
                    break;
                }
                play(&mut g, m);
            }
        }
        assert_eq!(g.outcome(), Some(Outcome::Draw));
        assert!(g.legal_moves().is_empty());
    }

    #[test]
    fn ai_is_fast_enough() {
        let mut g = Shogi::new();
        for m in ["7g7f", "3c3d", "8h2b+", "3a2b", "B*5e"] {
            play(&mut g, m);
        }
        let t = std::time::Instant::now();
        assert!(polytabletop_core::ai::best_move(&g, 1).is_some());
        let e = t.elapsed().as_secs_f64();
        eprintln!("ai: {e:.2}s");
        assert!(e < 5.0);
    }
}
