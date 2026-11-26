#![allow(
    clippy::needless_range_loop,
    clippy::manual_range_contains,
    clippy::type_complexity
)]
//! Chu Shogi (Japonia) — șahul japonez mijlociu, 12×12, 46 de piese pe parte.
//!
//! Pozițiile de start, mersul pieselor și promovările sunt cele din regulile publice ale jocului
//! (Regele, Prințul, Leul, Regina, Dragon King/Horse, Kirin, Phoenix, Tigrul orb, Leopardul, Elefantul
//! beat, Mutătorii, Căruța inversă, Lancea, Aurul, Argintul, Cuprul, Pionul, Mijlocașul ș.a.).
//!
//! * Mutări: `e4-e5`; cu `+` la sfârșit promovezi (`e8-e9+`); Leul poate muta în două pași,
//!   scris cu pătratul intermediar: `g6-g7-g8`; captura fără mișcare (igui) = `g6-g7-g6`.
//! * Promovarea e opțională în zona din ultimele 4 rânduri: la intrarea în zonă sau la o captură
//!   pornită din zonă.
//! * Câștigă cine capturează Regele adversarului **și** Prințul lui (dacă are); nu există obligația de a
//!   ieși din șah. După 800 de mutări remiză.
//! * Simplificări documentate: Leul nu are restricțiile de schimb de lei; Vulturul zburător și
//!   Șoimul cu coarne au mutarea de „leu” pe direcțiile lor ca o săritură de 1–2 pătrate (fără
//!   captură dublă); fără regula „regele gol” și fără „jitto” (trecerea Leului).

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: usize = 12;
const CAP: u32 = 800;
const NONE: u8 = 255;

// Tipuri de piese (1..=29), +32 pentru J2.
const CODES: [&str; 30] = [
    ".", "P", "GB", "C", "S", "G", "FL", "BT", "DE", "K", "Pr", "L", "RC", "SM", "VM", "R", "B",
    "DK", "DH", "Q", "Ph", "Ki", "Ln", "SE", "HF", "FS", "WH", "Wl", "FB", "FO",
];
const VALUE: [i32; 30] = [
    0, 100, 130, 250, 300, 350, 280, 330, 420, 6000, 3000, 300, 450, 400, 450, 650, 600, 900, 800,
    1100, 360, 360, 1500, 1300, 1200, 500, 650, 650, 650, 650,
];

fn promo(k: u8) -> u8 {
    match k {
        1 => 5,
        2 => 8,
        3 => 13,
        4 => 14,
        5 => 15,
        6 => 16,
        7 => 25,
        8 => 10,
        11 => 26,
        12 => 27,
        13 => 28,
        14 => 29,
        15 => 17,
        16 => 18,
        17 => 23,
        18 => 24,
        21 => 22,
        20 => 19,
        _ => 0,
    }
}

const NN: (i8, i8) = (1, 0);
const SS: (i8, i8) = (-1, 0);
const EE: (i8, i8) = (0, 1);
const WW: (i8, i8) = (0, -1);
const NE: (i8, i8) = (1, 1);
const NW: (i8, i8) = (1, -1);
const SE: (i8, i8) = (-1, 1);
const SW: (i8, i8) = (-1, -1);

struct Rule {
    steps: &'static [(i8, i8)],
    ranges: &'static [(i8, i8)],
    jumps: &'static [(i8, i8)],
    leap2: &'static [(i8, i8)],
}

const ORTH: &[(i8, i8)] = &[NN, SS, EE, WW];
const DIAG: &[(i8, i8)] = &[NE, NW, SE, SW];
const ALL: &[(i8, i8)] = &[NN, SS, EE, WW, NE, NW, SE, SW];

fn rule(k: u8) -> Rule {
    let r = |steps, ranges, jumps, leap2| Rule {
        steps,
        ranges,
        jumps,
        leap2,
    };
    match k {
        9 | 10 => r(ALL, &[], &[], &[]),
        8 => r(&[NN, EE, WW, NE, NW, SE, SW], &[], &[], &[]),
        5 => r(&[NN, SS, EE, WW, NE, NW], &[], &[], &[]),
        4 => r(&[NN, NE, NW, SE, SW], &[], &[], &[]),
        3 => r(&[NN, NE, NW, SS], &[], &[], &[]),
        6 => r(&[NN, NE, NW, SS, SE, SW], &[], &[], &[]),
        7 => r(&[SS, EE, WW, NE, NW, SE, SW], &[], &[], &[]),
        2 => r(&[NN, SS], &[], &[], &[]),
        1 => r(&[NN], &[], &[], &[]),
        11 => r(&[], &[NN], &[], &[]),
        12 => r(&[], &[NN, SS], &[], &[]),
        13 => r(&[NN, SS], &[EE, WW], &[], &[]),
        14 => r(&[EE, WW], &[NN, SS], &[], &[]),
        15 => r(&[], ORTH, &[], &[]),
        16 => r(&[], DIAG, &[], &[]),
        17 => r(DIAG, ORTH, &[], &[]),
        18 => r(ORTH, DIAG, &[], &[]),
        19 => r(&[], ALL, &[], &[]),
        20 => r(ORTH, &[], DIAG, &[]),
        21 => r(DIAG, &[], ORTH, &[]),
        23 => r(&[], &[NN, SS, EE, WW, SE, SW], &[], &[NE, NW]),
        24 => r(&[], &[SS, EE, WW, NE, NW, SE, SW], &[], &[NN]),
        25 => r(&[EE, WW, NE, NW, SE, SW], &[NN, SS], &[], &[]),
        26 => r(&[], &[NN, SS, NE, NW], &[], &[]),
        27 => r(&[], &[NN, SS, SE, SW], &[], &[]),
        28 => r(&[], &[EE, WW, NE, NW, SE, SW], &[], &[]),
        29 => r(&[], &[NN, SS, NE, NW, SE, SW], &[], &[]),
        _ => r(&[], &[], &[], &[]),
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Mv {
    pub from: u8,
    pub via: u8,
    pub to: u16,
    pub promote: bool,
}

#[derive(Clone)]
pub struct ChuShogi {
    b: [u8; N * N],
    side: Player,
    ply: u32,
}

impl Default for ChuShogi {
    fn default() -> Self {
        Self::new()
    }
}

fn owner(c: u8) -> Option<Player> {
    match c {
        1..=29 => Some(Player::One),
        33..=61 => Some(Player::Two),
        _ => None,
    }
}

fn kind(c: u8) -> u8 {
    c & 31
}

fn sq(i: usize) -> String {
    format!("{}{}", (b'a' + (i % N) as u8) as char, i / N + 1)
}

impl ChuShogi {
    pub fn new() -> Self {
        let mut b = [0u8; N * N];
        let back: [u8; 12] = [11, 6, 3, 4, 5, 8, 9, 5, 4, 3, 6, 11];
        let second: [u8; 12] = [12, 0, 16, 0, 7, 20, 21, 7, 0, 16, 0, 12];
        let third: [u8; 12] = [13, 14, 15, 18, 17, 19, 22, 17, 18, 15, 14, 13];
        for f in 0..N {
            b[f] = back[f];
            b[N + f] = second[f];
            b[2 * N + f] = third[f];
            b[3 * N + f] = 1;
            // J2: rotit cu 180°
            let g = N - 1 - f;
            b[(N - 1) * N + g] = if back[f] > 0 { back[f] + 32 } else { 0 };
            b[(N - 2) * N + g] = if second[f] > 0 { second[f] + 32 } else { 0 };
            b[(N - 3) * N + g] = third[f] + 32;
            b[(N - 4) * N + g] = 1 + 32;
        }
        // mijlocașii (go-between) pe rândul 5, coloanele d și i (4 și 9)
        b[4 * N + 3] = 2;
        b[4 * N + 8] = 2;
        b[(N - 5) * N + (N - 1 - 3)] = 2 + 32;
        b[(N - 5) * N + (N - 1 - 8)] = 2 + 32;
        ChuShogi {
            b,
            side: Player::One,
            ply: 0,
        }
    }

    pub fn piece_count(&self, p: Player) -> usize {
        self.b.iter().filter(|&&c| owner(c) == Some(p)).count()
    }

    fn zone(&self, p: Player, i: usize) -> bool {
        let r = i / N;
        if p == Player::One {
            r >= N - 4
        } else {
            r < 4
        }
    }

    fn gen(&self, p: Player) -> Vec<Mv> {
        let sgn: i8 = if p == Player::One { 1 } else { -1 };
        let mut out = Vec::new();
        for i in 0..N * N {
            let c = self.b[i];
            if owner(c) != Some(p) {
                continue;
            }
            let k = kind(c);
            let (r, f) = ((i / N) as i32, (i % N) as i32);
            let at = |dr: i32, df: i32| -> Option<usize> {
                let (nr, nf) = (r + dr, f + df);
                if nr < 0 || nf < 0 || nr >= N as i32 || nf >= N as i32 {
                    None
                } else {
                    Some(nr as usize * N + nf as usize)
                }
            };
            let free_or_enemy = |t: usize, s: &Self| owner(s.b[t]) != Some(p);
            if k == 22 {
                // Leul: unul sau doi pași
                for &(dr, df) in ALL {
                    let Some(s1) = at(dr as i32, df as i32) else {
                        continue;
                    };
                    if !free_or_enemy(s1, self) {
                        continue;
                    }
                    out.push(Mv {
                        from: i as u8,
                        via: NONE,
                        to: s1 as u16,
                        promote: false,
                    });
                    let (r1, f1) = ((s1 / N) as i32, (s1 % N) as i32);
                    for &(dr2, df2) in ALL {
                        let (nr, nf) = (r1 + dr2 as i32, f1 + df2 as i32);
                        if nr < 0 || nf < 0 || nr >= N as i32 || nf >= N as i32 {
                            continue;
                        }
                        let s2 = nr as usize * N + nf as usize;
                        let enemy_at_s1 = owner(self.b[s1]).is_some();
                        if s2 == i {
                            if enemy_at_s1 {
                                out.push(Mv {
                                    from: i as u8,
                                    via: s1 as u8,
                                    to: s2 as u16,
                                    promote: false,
                                });
                            }
                            continue;
                        }
                        if owner(self.b[s2]) == Some(p) {
                            continue;
                        }
                        out.push(Mv {
                            from: i as u8,
                            via: s1 as u8,
                            to: s2 as u16,
                            promote: false,
                        });
                    }
                }
                continue;
            }
            let rl = rule(k);
            let mut targets: Vec<usize> = Vec::new();
            for &(dr, df) in rl.steps {
                if let Some(t) = at((dr * sgn) as i32, df as i32) {
                    if free_or_enemy(t, self) {
                        targets.push(t);
                    }
                }
            }
            for &(dr, df) in rl.ranges {
                let (dr, df) = ((dr * sgn) as i32, df as i32);
                let (mut rr, mut ff) = (dr, df);
                while let Some(t) = at(rr, ff) {
                    if owner(self.b[t]) == Some(p) {
                        break;
                    }
                    targets.push(t);
                    if self.b[t] != 0 {
                        break;
                    }
                    rr += dr;
                    ff += df;
                }
            }
            for &(dr, df) in rl.jumps {
                if let Some(t) = at((dr * sgn) as i32 * 2, df as i32 * 2) {
                    if free_or_enemy(t, self) {
                        targets.push(t);
                    }
                }
            }
            for &(dr, df) in rl.leap2 {
                for m in 1..=2 {
                    if let Some(t) = at((dr * sgn) as i32 * m, df as i32 * m) {
                        if free_or_enemy(t, self) {
                            targets.push(t);
                        }
                    }
                }
            }
            let pk = promo(k);
            for t in targets {
                out.push(Mv {
                    from: i as u8,
                    via: NONE,
                    to: t as u16,
                    promote: false,
                });
                if pk != 0 && self.zone(p, t) && (!self.zone(p, i) || self.b[t] != 0) {
                    out.push(Mv {
                        from: i as u8,
                        via: NONE,
                        to: t as u16,
                        promote: true,
                    });
                }
            }
        }
        out
    }

    fn royal_left(&self, p: Player) -> bool {
        self.b
            .iter()
            .any(|&c| owner(c) == Some(p) && (kind(c) == 9 || kind(c) == 10))
    }

    fn text(&self, m: &Mv) -> String {
        let mut s = sq(m.from as usize);
        if m.via != NONE {
            s.push('-');
            s.push_str(&sq(m.via as usize));
        }
        s.push('-');
        s.push_str(&sq(m.to as usize));
        if m.promote {
            s.push('+');
        }
        s
    }
}

impl Game for ChuShogi {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        self.gen(self.side)
    }

    fn apply(&mut self, m: &Mv) {
        let from = m.from as usize;
        let mut c = self.b[from];
        self.b[from] = 0;
        if m.via != NONE {
            self.b[m.via as usize] = 0;
        }
        if m.promote {
            c = promo(kind(c)) + if self.side == Player::Two { 32 } else { 0 };
        }
        self.b[m.to as usize] = c;
        self.side = self.side.other();
        self.ply += 1;
    }

    fn outcome(&self) -> Option<Outcome> {
        let (a, b) = (self.royal_left(Player::One), self.royal_left(Player::Two));
        if !a {
            return Some(Outcome::Win(Player::Two));
        }
        if !b {
            return Some(Outcome::Win(Player::One));
        }
        if self.ply >= CAP {
            return Some(Outcome::Draw);
        }
        if self.gen(self.side).is_empty() {
            return Some(Outcome::Win(self.side.other()));
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let mut s = 0;
        for i in 0..N * N {
            let c = self.b[i];
            let Some(o) = owner(c) else { continue };
            let k = kind(c) as usize;
            let mut v = VALUE[k];
            let adv = if o == Player::One {
                i / N
            } else {
                N - 1 - i / N
            } as i32;
            if k <= 8 {
                v += adv * 4;
            }
            if o == p {
                s += v;
            } else {
                s -= v;
            }
        }
        s
    }

    fn render(&self) -> String {
        let mut s = String::from("     a   b   c   d   e   f   g   h   i   j   k   l\n");
        for r in (0..N).rev() {
            s.push_str(&format!(" {:>2}  ", r + 1));
            for f in 0..N {
                let c = self.b[r * N + f];
                let t = CODES[kind(c) as usize];
                let t = if owner(c) == Some(Player::Two) {
                    t.to_lowercase()
                } else {
                    t.to_string()
                };
                s.push_str(&format!("{:<4}", t));
            }
            s.push_str(&format!("{}\n", r + 1));
        }
        s.push_str("     a   b   c   d   e   f   g   h   i   j   k   l\n");
        s.push_str("MAJUSCULE = J1 (jos), minuscule = J2. P pion, GB mijlocaș, C cupru, S argint, G aur, FL leopard, BT tigru orb, DE elefant beat, K rege, Pr prinț,\nL lance, RC căruță inversă, SM/VM mutători, R turn, B nebun, DK/DH dragon rege/cal, Q regină, Ph phoenix, Ki kirin, Ln leu, SE/HF vultur/șoim,\nFS cerb, WH cal alb, Wl balenă, FB mistreț, FO bou zburător.\n");
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        self.text(m)
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(2)
    }

    fn move_score(&self, m: &Mv) -> i32 {
        let t = self.b[m.to as usize];
        let mut v = 0;
        if t != 0 {
            v +=
                1000 + VALUE[kind(t) as usize] - VALUE[kind(self.b[m.from as usize]) as usize] / 10;
        }
        if m.promote {
            v += 300;
        }
        v
    }

    fn is_tactical(&self, m: &Mv) -> bool {
        self.b[m.to as usize] != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    #[test]
    fn setup_has_46_pieces_each() {
        let g = ChuShogi::new();
        assert_eq!(g.piece_count(Player::One), 46);
        assert_eq!(g.piece_count(Player::Two), 46);
        // regele și leul pe pozițiile corecte (J1 jos, rândul 1: rege pe g1; leu pe rândul 3)
        assert_eq!(kind(g.b[6]), 9);
        assert_eq!(kind(g.b[2 * N + 6]), 22);
        // J2 e imaginea rotită
        assert_eq!(g.b[(N - 1) * N + (N - 1 - 6)], 9 + 32);
        assert!(!g.legal_moves().is_empty());
    }

    #[test]
    fn promotion_rules() {
        let mut g = ChuShogi::new();
        g.b = [0; N * N];
        g.b[6] = 9; // rege J1
        g.b[N * 11 + 6] = 9 + 32; // rege J2
        g.b[7 * N] = 1; // pion a8 (rândul 8, înainte de zonă)
        let mv = g.legal_moves();
        let t = |m: &Mv| g.format_move(m);
        let list: Vec<String> = mv.iter().map(t).collect();
        assert!(list.contains(&"a8-a9".to_string()));
        assert!(list.contains(&"a8-a9+".to_string()));
    }

    #[test]
    fn lion_double_step_and_igui() {
        let mut g = ChuShogi::new();
        g.b = [0; N * N];
        g.b[6] = 9;
        g.b[N * 11 + 6] = 9 + 32;
        g.b[5 * N + 5] = 22; // leu f6
        g.b[5 * N + 6] = 1 + 32; // pion J2 g6 adiacent
        let list: Vec<String> = g.legal_moves().iter().map(|m| g.format_move(m)).collect();
        assert!(list.contains(&"f6-g6-f6".to_string()), "igui lipsește");
        assert!(list.contains(&"f6-g6-h6".to_string()));
        assert!(list.contains(&"f6-f7-f8".to_string()));
    }

    #[test]
    fn kirin_jumps_and_royal_capture_ends() {
        let mut g = ChuShogi::new();
        g.b = [0; N * N];
        g.b[6] = 9;
        g.b[N * 11 + 6] = 9 + 32;
        g.b[5 * N + 5] = 21; // kirin f6
        g.b[7 * N + 5] = 3 + 32; // piesă la f8 (2 în față): kirin poate sări
        let list: Vec<String> = g.legal_moves().iter().map(|m| g.format_move(m)).collect();
        assert!(list.contains(&"f6-f8".to_string()));
        assert!(list.contains(&"f6-g7".to_string()));
        // captura regelui
        let mut h = ChuShogi::new();
        h.b = [0; N * N];
        h.b[0] = 9;
        h.b[N * 11 + 6] = 9 + 32;
        h.b[N * 10 + 6] = 15; // turn J1 pe g11
        let m = h
            .legal_moves()
            .into_iter()
            .find(|m| m.to as usize == N * 11 + 6)
            .unwrap();
        h.apply(&m);
        assert_eq!(h.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=2u64 {
            let mut g = ChuShogi::new();
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() && n < 2000 {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
            }
            assert!(g.outcome().is_some());
        }
    }
}
