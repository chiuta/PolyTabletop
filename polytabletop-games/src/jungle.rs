//! Jungle (Dou Shou Qi, China) — „șahul animalelor”, tablă 7×9, 8 piese pe jucător.
//!
//! Ierarhie: 1 șobolan, 2 pisică, 3 câine, 4 lup, 5 leopard, 6 tigru, 7 leu, 8 elefant.
//! * O piesă mută un pas ortogonal. Mută J1 (jos) întâi.
//! * Captură: rang egal sau mai mare; excepții: șobolanul poate captura elefantul (nu din apă),
//!   elefantul nu poate captura șobolanul. Șobolanul poate intra în lacuri (b–c și e–f, rândurile 4–6);
//!   alte piese nu. Un șobolan în apă nu capturează o piesă de pe uscat și invers.
//! * Leul și tigrul sar peste lac (3 pătrate de apă) pe orizontală/verticală, dacă nu e un șobolan în
//!   drum (al oricui); pot captura la aterizare.
//! * Capcanele (c1, e1, d2 pentru J1; c9, e9, d8 pentru J2): o piesă care intră într-o capcană a
//!   adversarului are rang 0 și poate fi capturată de orice piesă.
//! * Nu intri în propriul bârlog (d1 / d9). Câștigi intrând în bârlogul adversarului, capturându-i
//!   toate piesele sau blocându-l. Remiză după 200 de mutări fără captură.
//!
//! Sursa: regulile publice (Wikipedia „Jungle (board game)”).

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const W: i32 = 7;
const H: i32 = 9;
const NAMES: [&str; 8] = [
    "șobolan", "pisică", "câine", "lup", "leopard", "tigru", "leu", "elefant",
];
const LETTERS: [char; 8] = ['r', 'c', 'd', 'w', 'p', 't', 'l', 'e'];

fn idx(x: i32, y: i32) -> usize {
    (y * W + x) as usize
}
fn name(i: usize) -> String {
    format!(
        "{}{}",
        (b'a' + (i as i32 % W) as u8) as char,
        i as i32 / W + 1
    )
}
fn water(x: i32, y: i32) -> bool {
    (3..=5).contains(&y) && matches!(x, 1 | 2 | 4 | 5)
}
fn den(p: usize) -> (i32, i32) {
    (3, if p == 0 { 0 } else { 8 })
}
/// Capcanele proprii ale lui `p` (în care adversarul e slăbit).
fn is_trap_of(p: usize, x: i32, y: i32) -> bool {
    let (dx, dy) = den(p);
    let dir = if p == 0 { 1 } else { -1 };
    (x == dx - 1 && y == dy) || (x == dx + 1 && y == dy) || (x == dx && y == dy + dir)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Mv {
    pub from: u8,
    pub to: u8,
}

/// Celulă: 0 gol; 1..=8 = J1 rang; 9..=16 = J2 rang (rang+8).
#[derive(Clone)]
pub struct Jungle {
    b: [u8; 63],
    side: Player,
    quiet: u16,
    won: Option<Player>,
}

impl Default for Jungle {
    fn default() -> Self {
        Self::new()
    }
}

fn owner(c: u8) -> Option<usize> {
    match c {
        0 => None,
        1..=8 => Some(0),
        _ => Some(1),
    }
}
fn rank(c: u8) -> u8 {
    if c > 8 {
        c - 8
    } else {
        c
    }
}

impl Jungle {
    pub fn new() -> Self {
        let mut b = [0u8; 63];
        // J1 (jos): leu a1, tigru g1, câine b2, pisică f2, șobolan a3, leopard c3, lup e3, elefant g3
        for (x, y, r) in [
            (0, 0, 7),
            (6, 0, 6),
            (1, 1, 3),
            (5, 1, 2),
            (0, 2, 1),
            (2, 2, 5),
            (4, 2, 4),
            (6, 2, 8),
        ] {
            b[idx(x, y)] = r;
            b[idx(6 - x, 8 - y)] = r + 8;
        }
        Jungle {
            b,
            side: Player::One,
            quiet: 0,
            won: None,
        }
    }

    pub fn count(&self, p: Player) -> usize {
        self.b
            .iter()
            .filter(|&&c| owner(c) == Some(p.idx()))
            .count()
    }

    fn eff_rank(&self, c: u8, x: i32, y: i32) -> u8 {
        let o = owner(c).unwrap();
        if is_trap_of(1 - o, x, y) {
            0
        } else {
            rank(c)
        }
    }

    fn can_capture(&self, a: u8, ax: i32, ay: i32, d: u8, dx: i32, dy: i32) -> bool {
        let (ra, rd) = (rank(a), rank(d));
        // apă / uscat
        if water(ax, ay) != water(dx, dy) {
            return false;
        }
        let ea = self.eff_rank(a, ax, ay);
        let ed = self.eff_rank(d, dx, dy);
        if ea == 1 && rd == 8 && ed != 0 {
            return !water(ax, ay);
        }
        if ra == 8 && rd == 1 && ed != 0 {
            return false;
        }
        ea >= ed
    }

    fn moves_for(&self, side: Player) -> Vec<Mv> {
        let p = side.idx();
        let mut v = Vec::new();
        for i in 0..63usize {
            let c = self.b[i];
            if owner(c) != Some(p) {
                continue;
            }
            let (x, y) = (i as i32 % W, i as i32 / W);
            let r = rank(c);
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let (mut tx, mut ty) = (x + dx, y + dy);
                if !(0..W).contains(&tx) || !(0..H).contains(&ty) {
                    continue;
                }
                // salt peste lac
                if (r == 6 || r == 7) && water(tx, ty) {
                    let mut blocked = false;
                    while water(tx, ty) {
                        if self.b[idx(tx, ty)] != 0 {
                            blocked = true;
                        }
                        tx += dx;
                        ty += dy;
                    }
                    if blocked || !(0..W).contains(&tx) || !(0..H).contains(&ty) {
                        continue;
                    }
                } else if water(tx, ty) && r != 1 {
                    continue;
                }
                if (tx, ty) == den(p) {
                    continue;
                }
                let t = self.b[idx(tx, ty)];
                if t == 0 || (owner(t) != Some(p) && self.can_capture(c, x, y, t, tx, ty)) {
                    v.push(Mv {
                        from: i as u8,
                        to: idx(tx, ty) as u8,
                    });
                }
            }
        }
        v
    }
}

impl Game for Jungle {
    type Move = Mv;
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<Mv> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        self.moves_for(self.side)
    }
    fn apply(&mut self, m: &Mv) {
        let c = self.b[m.from as usize];
        let cap = self.b[m.to as usize] != 0;
        self.b[m.from as usize] = 0;
        self.b[m.to as usize] = c;
        let (x, y) = (m.to as i32 % W, m.to as i32 / W);
        if (x, y) == den(1 - self.side.idx()) {
            self.won = Some(self.side);
        }
        self.quiet = if cap { 0 } else { self.quiet + 1 };
        self.side = self.side.other();
    }
    fn outcome(&self) -> Option<Outcome> {
        if let Some(p) = self.won {
            return Some(Outcome::Win(p));
        }
        for p in [Player::One, Player::Two] {
            if self.count(p) == 0 {
                return Some(Outcome::Win(p.other()));
            }
        }
        if self.quiet >= 200 {
            return Some(Outcome::Draw);
        }
        if self.moves_for(self.side).is_empty() {
            return Some(Outcome::Win(self.side.other()));
        }
        None
    }
    fn evaluate(&self, p: Player) -> i32 {
        let mut s = 0i32;
        for i in 0..63usize {
            let c = self.b[i];
            let Some(o) = owner(c) else { continue };
            let r = rank(c) as i32;
            let (x, y) = (i as i32 % W, i as i32 / W);
            let val = 100 + r * 40;
            let (dx, dy) = den(1 - o);
            let dist = (x - dx).abs() + (y - dy).abs();
            let v = val + (16 - dist).max(0) * 3;
            if o == p.idx() {
                s += v;
            } else {
                s -= v;
            }
        }
        s
    }
    fn move_score(&self, m: &Mv) -> i32 {
        let t = self.b[m.to as usize];
        if t == 0 {
            0
        } else {
            100 + rank(t) as i32
        }
    }
    fn render(&self) -> String {
        let mut s = String::new();
        for y in (0..H).rev() {
            s.push_str(&format!("{}  ", y + 1));
            for x in 0..W {
                let c = self.b[idx(x, y)];
                let cell = match owner(c) {
                    Some(o) => {
                        let l = LETTERS[rank(c) as usize - 1];
                        if o == 0 {
                            l.to_ascii_uppercase()
                        } else {
                            l
                        }
                    }
                    None => {
                        if water(x, y) {
                            '~'
                        } else if (x, y) == den(0) || (x, y) == den(1) {
                            '#'
                        } else if is_trap_of(0, x, y) || is_trap_of(1, x, y) {
                            '+'
                        } else {
                            '.'
                        }
                    }
                };
                s.push(cell);
                s.push(' ');
            }
            s.push('\n');
        }
        s.push_str("   a b c d e f g\n");
        s.push_str("MAJUSCULE = J1, minuscule = J2; r șobolan, c pisică, d câine, w lup, p leopard, t tigru, l leu, e elefant; ~ apă, # bârlog, + capcană.\n");
        s.push_str(&format!(
            "La mutare: Jucătorul {}. ({} / {} piese)",
            self.side.idx() + 1,
            self.count(Player::One),
            self.count(Player::Two)
        ));
        let _ = NAMES;
        s
    }
    fn format_move(&self, m: &Mv) -> String {
        let x = if self.b[m.to as usize] != 0 { 'x' } else { '-' };
        format!("{}{}{}", name(m.from as usize), x, name(m.to as usize))
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 300,
            max_depth: 7,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q4_util::*;
    use polytabletop_core::{GameObj, Player};

    #[test]
    fn setup_counts() {
        let g = Jungle::new();
        assert_eq!(g.count(Player::One), 8);
        assert_eq!(g.count(Player::Two), 8);
        assert_eq!(g.b[idx(0, 0)], 7); // leu a1
    }

    #[test]
    fn rat_beats_elephant_on_land_not_reverse() {
        let mut g = Jungle::new();
        g.b = [0; 63];
        g.b[idx(0, 2)] = 1; // șobolan J1
        g.b[idx(0, 3)] = 8 + 8; // elefant J2
        let m = GameObj::legal_moves(&g);
        assert!(m.contains(&"a3xa4".to_string()), "{m:?}");
        let mut h = g.clone();
        h.side = Player::Two;
        let m2 = GameObj::legal_moves(&h);
        assert!(!m2.contains(&"a4xa3".to_string()), "{m2:?}");
    }

    #[test]
    fn lion_jumps_the_lake_unless_rat_blocks() {
        let mut g = Jungle::new();
        g.b = [0; 63];
        g.b[idx(1, 2)] = 7;
        g.b[idx(6, 8)] = 9;
        assert!(GameObj::legal_moves(&g).contains(&"b3-b7".to_string()));
        g.b[idx(1, 4)] = 16; // șobolan adversar în apă
        assert!(!GameObj::legal_moves(&g).contains(&"b3-b7".to_string()));
    }

    #[test]
    fn random_and_ai_games_finish() {
        random_finishes(|_| Box::new(Jungle::new()), 5, 3000);
        ai_vs_random(|_| Box::new(Jungle::new()), 2, Player::One, 3000);
        ai_vs_random(|_| Box::new(Jungle::new()), 3, Player::Two, 3000);
    }
}
