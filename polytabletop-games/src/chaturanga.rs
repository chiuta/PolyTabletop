//! Chaturanga (India) — strămoșul șahului, în forma reconstruită cea mai răspândită.
//!
//! Piese: raja (rege: un pas în orice direcție), mantri (sfetnic: un pas pe diagonală),
//! gaja (elefant: sare exact două pătrate pe diagonală, peste orice), ashva (cal ca la șah),
//! ratha (car: turn), padati (pion: un pas înainte, captură pe diagonală înainte;
//! ajuns pe ultimul rând devine mantri). Fără rocadă, fără pas dublu, fără en passant.
//!
//! * Mat = pierdere; pat = remiză; „raja gol” (adversarul rămas doar cu raja) = victorie
//!   (dacă și celălalt e gol, remiză). Remiză după 100 de mutări fără captură.
//! * Tabla: J1 jos (rândurile 1–2), J2 sus; același aranjament pe ambele părți
//!   (car, cal, elefant, mantri, raja, elefant, cal, car). Mutări: e2e3, d1d2.
//!
//! Sursa: reguli reconstruite publice (Wikipedia „Chaturanga”, varianta cu mat și raja gol).

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const RAJA: u8 = 1;
const MANTRI: u8 = 2;
const GAJA: u8 = 3;
const ASHVA: u8 = 4;
const RATHA: u8 = 5;
const PADATI: u8 = 6;
const VAL: [i32; 7] = [0, 0, 160, 220, 320, 520, 100];
const LET: [char; 7] = ['.', 'R', 'M', 'G', 'A', 'T', 'P'];

fn name(i: usize) -> String {
    format!("{}{}", (b'a' + (i % 8) as u8) as char, i / 8 + 1)
}

fn own(c: u8) -> Option<usize> {
    match c {
        0 => None,
        1..=6 => Some(0),
        _ => Some(1),
    }
}
fn kind(c: u8) -> u8 {
    if c > 6 {
        c - 6
    } else {
        c
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Mv {
    pub from: u8,
    pub to: u8,
}

#[derive(Clone)]
pub struct Chaturanga {
    b: [u8; 64],
    side: Player,
    quiet: u16,
    plies: u16,
}

impl Default for Chaturanga {
    fn default() -> Self {
        Self::new()
    }
}

impl Chaturanga {
    pub fn new() -> Self {
        let mut b = [0u8; 64];
        let back = [RATHA, ASHVA, GAJA, MANTRI, RAJA, GAJA, ASHVA, RATHA];
        for x in 0..8 {
            b[x] = back[x];
            b[8 + x] = PADATI;
            b[56 + x] = back[x] + 6;
            b[48 + x] = PADATI + 6;
        }
        Chaturanga {
            b,
            side: Player::One,
            quiet: 0,
            plies: 0,
        }
    }

    pub fn piece_count(&self, p: Player) -> usize {
        self.b.iter().filter(|&&c| own(c) == Some(p.idx())).count()
    }

    /// Mutări pseudo-legale ale lui `side`; cu `attacks` doar pătratele atacate (pionul: diagonalele).
    fn gen(&self, side: usize, attacks: bool) -> Vec<Mv> {
        let mut v = Vec::new();
        let push =
            |v: &mut Vec<Mv>, from: usize, x: i32, y: i32, capture_only: bool, move_only: bool| {
                if !(0..8).contains(&x) || !(0..8).contains(&y) {
                    return false;
                }
                let t = (y * 8 + x) as usize;
                let c = self.b[t];
                if c == 0 {
                    if (!capture_only || attacks) && !(attacks && move_only) {
                        v.push(Mv {
                            from: from as u8,
                            to: t as u8,
                        });
                    }
                    true
                } else {
                    if own(c) != Some(side) && !move_only {
                        v.push(Mv {
                            from: from as u8,
                            to: t as u8,
                        });
                    }
                    false
                }
            };
        for i in 0..64usize {
            let c = self.b[i];
            if own(c) != Some(side) {
                continue;
            }
            let (x, y) = (i as i32 % 8, i as i32 / 8);
            match kind(c) {
                RAJA => {
                    for dx in -1..=1 {
                        for dy in -1..=1 {
                            if dx != 0 || dy != 0 {
                                push(&mut v, i, x + dx, y + dy, false, false);
                            }
                        }
                    }
                }
                MANTRI => {
                    for (dx, dy) in [(1, 1), (1, -1), (-1, 1), (-1, -1)] {
                        push(&mut v, i, x + dx, y + dy, false, false);
                    }
                }
                GAJA => {
                    for (dx, dy) in [(2, 2), (2, -2), (-2, 2), (-2, -2)] {
                        push(&mut v, i, x + dx, y + dy, false, false);
                    }
                }
                ASHVA => {
                    for (dx, dy) in [
                        (1, 2),
                        (2, 1),
                        (-1, 2),
                        (-2, 1),
                        (1, -2),
                        (2, -1),
                        (-1, -2),
                        (-2, -1),
                    ] {
                        push(&mut v, i, x + dx, y + dy, false, false);
                    }
                }
                RATHA => {
                    for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                        let (mut tx, mut ty) = (x + dx, y + dy);
                        while push(&mut v, i, tx, ty, false, false) {
                            tx += dx;
                            ty += dy;
                        }
                    }
                }
                _ => {
                    let dy = if side == 0 { 1 } else { -1 };
                    if !attacks {
                        let (tx, ty) = (x, y + dy);
                        if (0..8).contains(&ty) && self.b[(ty * 8 + tx) as usize] == 0 {
                            v.push(Mv {
                                from: i as u8,
                                to: (ty * 8 + tx) as u8,
                            });
                        }
                    }
                    for dx in [-1, 1] {
                        let (tx, ty) = (x + dx, y + dy);
                        if (0..8).contains(&tx) && (0..8).contains(&ty) {
                            let t = (ty * 8 + tx) as usize;
                            if attacks || (self.b[t] != 0 && own(self.b[t]) != Some(side)) {
                                v.push(Mv {
                                    from: i as u8,
                                    to: t as u8,
                                });
                            }
                        }
                    }
                }
            }
        }
        v
    }

    fn raja_square(&self, side: usize) -> Option<usize> {
        self.b
            .iter()
            .position(|&c| own(c) == Some(side) && kind(c) == RAJA)
    }

    pub fn in_check(&self, p: Player) -> bool {
        let s = p.idx();
        match self.raja_square(s) {
            Some(k) => self.gen(1 - s, true).iter().any(|m| m.to as usize == k),
            None => true,
        }
    }

    fn do_move(&mut self, m: &Mv) {
        let mut c = self.b[m.from as usize];
        self.b[m.from as usize] = 0;
        if kind(c) == PADATI {
            let ty = m.to as usize / 8;
            if (own(c) == Some(0) && ty == 7) || (own(c) == Some(1) && ty == 0) {
                c = c - PADATI + MANTRI;
            }
        }
        self.b[m.to as usize] = c;
    }

    fn legal_for(&self, side: Player) -> Vec<Mv> {
        self.gen(side.idx(), false)
            .into_iter()
            .filter(|m| {
                let mut c = self.clone();
                c.do_move(m);
                !c.in_check(side)
            })
            .collect()
    }
}

impl Game for Chaturanga {
    type Move = Mv;
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<Mv> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        self.legal_for(self.side)
    }
    fn apply(&mut self, m: &Mv) {
        let cap = self.b[m.to as usize] != 0;
        let pawn = kind(self.b[m.from as usize]) == PADATI;
        self.do_move(m);
        self.quiet = if cap || pawn { 0 } else { self.quiet + 1 };
        self.plies += 1;
        self.side = self.side.other();
    }
    fn outcome(&self) -> Option<Outcome> {
        let (a, b) = (self.piece_count(Player::One), self.piece_count(Player::Two));
        if a == 1 && b == 1 {
            return Some(Outcome::Draw);
        }
        if a == 1 {
            return Some(Outcome::Win(Player::Two));
        }
        if b == 1 {
            return Some(Outcome::Win(Player::One));
        }
        if self.quiet >= 100 || self.plies >= 500 {
            return Some(Outcome::Draw);
        }
        if self.legal_for(self.side).is_empty() {
            return Some(if self.in_check(self.side) {
                Outcome::Win(self.side.other())
            } else {
                Outcome::Draw
            });
        }
        None
    }
    fn evaluate(&self, p: Player) -> i32 {
        let mut s = 0;
        for i in 0..64usize {
            let c = self.b[i];
            let Some(o) = own(c) else { continue };
            let k = kind(c);
            let mut v = VAL[k as usize];
            if k == PADATI {
                let adv = if o == 0 { i / 8 } else { 7 - i / 8 } as i32;
                v += adv * 8;
            } else if k != RAJA {
                let cx = (i as i32 % 8 * 2 - 7).abs();
                let cy = (i as i32 / 8 * 2 - 7).abs();
                v += 8 - (cx + cy) / 2;
            }
            if o == p.idx() {
                s += v;
            } else {
                s -= v;
            }
        }
        if self.in_check(p.other()) {
            s += 30;
        }
        if self.in_check(p) {
            s -= 30;
        }
        s
    }
    fn move_score(&self, m: &Mv) -> i32 {
        let t = self.b[m.to as usize];
        if t == 0 {
            0
        } else {
            1000 + VAL[kind(t) as usize] - VAL[kind(self.b[m.from as usize]) as usize] / 10
        }
    }
    fn render(&self) -> String {
        let mut s = String::new();
        for y in (0..8).rev() {
            s.push_str(&format!("{}  ", y + 1));
            for x in 0..8 {
                let c = self.b[y * 8 + x];
                let ch = LET[kind(c) as usize];
                s.push(if own(c) == Some(1) {
                    ch.to_ascii_lowercase()
                } else {
                    ch
                });
                s.push(' ');
            }
            s.push('\n');
        }
        s.push_str("   a b c d e f g h\nMAJUSCULE = J1, minuscule = J2: R raja, M mantri, G gaja, A ashva, T ratha, P padati.\n");
        s.push_str(&format!(
            "La mutare: Jucătorul {}{}.",
            self.side.idx() + 1,
            if self.in_check(self.side) {
                " (șah)"
            } else {
                ""
            }
        ));
        s
    }
    fn format_move(&self, m: &Mv) -> String {
        format!("{}{}", name(m.from as usize), name(m.to as usize))
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 400,
            max_depth: 6,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q4_util::*;
    use polytabletop_core::{GameObj, Player};

    #[test]
    fn opening_moves() {
        let g = Chaturanga::new();
        let m = GameObj::legal_moves(&g);
        // 8 pioni + 2 cai × 2 + 2 elefanți: sar la c1->? (blocați de proprii: c3 are pion? nu: c1-a3/e3 libere)
        assert!(m.contains(&"e2e3".to_string()));
        assert!(m.contains(&"b1a3".to_string()));
        assert!(
            m.contains(&"c1e3".to_string()),
            "gaja sare 2 pe diagonală: {m:?}"
        );
        assert!(!m.contains(&"e2e4".to_string()), "fără pas dublu");
    }

    #[test]
    fn pawn_promotes_to_mantri() {
        let mut g = Chaturanga::new();
        g.b = [0; 64];
        g.b[4] = RAJA;
        g.b[60] = RAJA + 6;
        g.b[6 * 8] = PADATI;
        g.b[3 * 8 + 7] = ASHVA + 6;
        GameObj::play(&mut g, "a7a8").unwrap();
        assert_eq!(g.b[56], MANTRI);
    }

    #[test]
    fn bare_raja_loses_and_mate_is_detected() {
        let mut g = Chaturanga::new();
        g.b = [0; 64];
        g.b[4] = RAJA;
        g.b[8 * 7 + 4] = RAJA + 6;
        g.b[8 * 3 + 4] = RATHA;
        assert_eq!(Game::outcome(&g), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn random_and_ai_games_finish() {
        random_finishes(|_| Box::new(Chaturanga::new()), 4, 2000);
        ai_vs_random(|_| Box::new(Chaturanga::new()), 2, Player::One, 2000);
        ai_vs_random(|_| Box::new(Chaturanga::new()), 3, Player::Two, 2000);
    }
}
