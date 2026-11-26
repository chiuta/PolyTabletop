//! Sittuyin (șahul birman), 2 jucători, din regulile publice.
//!
//! * Tablă 8×8. Cei 8 pioni (nè) ai fiecărui jucător sunt deja pe tablă: J1 pe a3 b3 c3 d4 e4 f3 g3 h3,
//!   J2 pe a6 b6 c6 d5 e5 f6 g6 h6.
//! * **Faza de plasare** (J1 și J2 pe rând, J1 primul): fiecare își așază celelalte 8 piese —
//!   rege (K), general (G), 2 elefanți (E), 2 cai (H), 2 bărci/care (R) — pe orice pătrat liber
//!   din **jumătatea proprie** (rândurile 1–4, respectiv 5–8). Apoi începe J1.
//! * Mutări: rege un pas oricum; general un pas în diagonală; elefant un pas în diagonală sau
//!   înainte; cal ca în șah; barcă pe linii și coloane; pion un pas înainte, capturează un pas în
//!   diagonală înainte, niciodată înapoi.
//! * **Promovare**: un pion aflat pe o diagonală mare în jumătatea adversarului (J1: a8 b7 c6 d5
//!   e5 f6 g7 h8; J2: a1 b2 c3 d4 e4 f3 g2 h1) poate, *în loc de o mutare*, deveni general, dar
//!   numai dacă jucătorul nu mai are general pe tablă.
//! * Mat câștigă. Pat nu e permis (o mutare care lasă adversarul fără mutări fără a da mat e ilegală).
//!   Simplificare: remiză după 300 de plie sau când rămân doar regii.
//!
//! Mutări: `K@e1` (plasare), `e3-e4`, `e4xd5`, `d5=G` (promovare).

use polytabletop_core::{AiStyle, Game, Outcome, Player};

// tip: 1 K, 2 G, 3 E, 4 H, 5 R, 6 P ; +8 pentru J2
const LET: [char; 7] = ['?', 'K', 'G', 'E', 'H', 'R', 'P'];
const VAL: [i32; 7] = [0, 0, 200, 250, 300, 500, 100];

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Mv {
    Place(u8, u8),
    Step(u8, u8),
    Promote(u8),
}

#[derive(Clone)]
pub struct Sittuyin {
    b: [u8; 64],
    /// piese de plasat: [K, G, E, H, R] per jucător
    hand: [[u8; 5]; 2],
    side: Player,
    plies: u16,
}

impl Default for Sittuyin {
    fn default() -> Self {
        Self::new()
    }
}

fn sqn(i: u8) -> String {
    format!("{}{}", (b'a' + i % 8) as char, i / 8 + 1)
}

fn col(c: u8) -> usize {
    (c >> 3) as usize
}

fn typ(c: u8) -> usize {
    (c & 7) as usize
}

fn is_promo_sq(side: Player, i: usize) -> bool {
    let (r, f) = (i / 8, i % 8);
    let diag = r == f || r + f == 7;
    diag && if side == Player::One { r >= 4 } else { r <= 3 }
}

fn off(i: usize, dr: i32, df: i32) -> Option<usize> {
    let (r, f) = ((i / 8) as i32 + dr, (i % 8) as i32 + df);
    if (0..8).contains(&r) && (0..8).contains(&f) {
        Some((r * 8 + f) as usize)
    } else {
        None
    }
}

impl Sittuyin {
    pub fn new() -> Self {
        let mut b = [0u8; 64];
        for s in ["a3", "b3", "c3", "d4", "e4", "f3", "g3", "h3"] {
            b[idx(s)] = 6;
        }
        for s in ["a6", "b6", "c6", "d5", "e5", "f6", "g6", "h6"] {
            b[idx(s)] = 6 + 8;
        }
        Sittuyin {
            b,
            hand: [[1, 1, 2, 2, 2]; 2],
            side: Player::One,
            plies: 0,
        }
    }

    fn placing(&self) -> bool {
        self.hand.iter().any(|h| h.iter().any(|&x| x > 0))
    }

    /// Pătratele atacate de piesa de pe `i` (fără a ține seama de regele propriu).
    fn attacks(&self, i: usize) -> Vec<usize> {
        let c = self.b[i];
        let me = col(c);
        let fwd = if me == 0 { 1 } else { -1 };
        let mut v = Vec::new();
        let mut add = |d: Option<usize>| {
            if let Some(d) = d {
                v.push(d);
            }
        };
        match typ(c) {
            1 => {
                for dr in -1..=1 {
                    for df in -1..=1 {
                        if (dr, df) != (0, 0) {
                            add(off(i, dr, df));
                        }
                    }
                }
            }
            2 => {
                for dr in [-1, 1] {
                    for df in [-1, 1] {
                        add(off(i, dr, df));
                    }
                }
            }
            3 => {
                for dr in [-1, 1] {
                    for df in [-1, 1] {
                        add(off(i, dr, df));
                    }
                }
                add(off(i, fwd, 0));
            }
            4 => {
                for (dr, df) in [
                    (1, 2),
                    (2, 1),
                    (-1, 2),
                    (-2, 1),
                    (1, -2),
                    (2, -1),
                    (-1, -2),
                    (-2, -1),
                ] {
                    add(off(i, dr, df));
                }
            }
            5 => {
                for (dr, df) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let mut cur = i;
                    while let Some(n) = off(cur, dr, df) {
                        v.push(n);
                        if self.b[n] != 0 {
                            break;
                        }
                        cur = n;
                    }
                }
            }
            6 => {
                add(off(i, fwd, 1));
                add(off(i, fwd, -1));
            }
            _ => {}
        }
        v
    }

    fn attacked(&self, sq: usize, by: usize) -> bool {
        (0..64).any(|i| self.b[i] != 0 && col(self.b[i]) == by && self.attacks(i).contains(&sq))
    }

    fn king_sq(&self, p: usize) -> Option<usize> {
        (0..64).find(|&i| self.b[i] != 0 && col(self.b[i]) == p && typ(self.b[i]) == 1)
    }

    fn in_check(&self, p: usize) -> bool {
        match self.king_sq(p) {
            Some(k) => self.attacked(k, 1 - p),
            None => false,
        }
    }

    fn has_general(&self, p: usize) -> bool {
        self.b.iter().any(|&c| c != 0 && col(c) == p && typ(c) == 2)
    }

    fn pseudo(&self) -> Vec<Mv> {
        let me = self.side.idx();
        let fwd = if me == 0 { 1 } else { -1 };
        let mut v = Vec::new();
        for i in 0..64 {
            let c = self.b[i];
            if c == 0 || col(c) != me {
                continue;
            }
            if typ(c) == 6 {
                if let Some(n) = off(i, fwd, 0) {
                    if self.b[n] == 0 {
                        v.push(Mv::Step(i as u8, n as u8));
                    }
                }
                for df in [-1, 1] {
                    if let Some(n) = off(i, fwd, df) {
                        if self.b[n] != 0 && col(self.b[n]) != me {
                            v.push(Mv::Step(i as u8, n as u8));
                        }
                    }
                }
                if is_promo_sq(self.side, i) && !self.has_general(me) {
                    v.push(Mv::Promote(i as u8));
                }
                continue;
            }
            for n in self.attacks(i) {
                if self.b[n] == 0 || col(self.b[n]) != me {
                    v.push(Mv::Step(i as u8, n as u8));
                }
            }
        }
        v
    }

    fn make(&self, m: &Mv) -> Sittuyin {
        let mut g = self.clone();
        g.do_move(m);
        g
    }

    fn do_move(&mut self, m: &Mv) {
        let me = self.side.idx();
        match *m {
            Mv::Place(t, s) => {
                self.b[s as usize] = t + 8 * me as u8;
                self.hand[me][t as usize - 1] -= 1;
            }
            Mv::Step(a, z) => {
                self.b[z as usize] = self.b[a as usize];
                self.b[a as usize] = 0;
            }
            Mv::Promote(a) => {
                self.b[a as usize] = 2 + 8 * me as u8;
            }
        }
        self.plies += 1;
        self.side = self.side.other();
    }

    fn legal_inner(&self, check_stalemate: bool) -> Vec<Mv> {
        if self.placing() {
            let me = self.side.idx();
            let rows = if me == 0 { 0..4 } else { 4..8 };
            let mut v = Vec::new();
            for t in 1..=5u8 {
                if self.hand[me][t as usize - 1] == 0 {
                    continue;
                }
                for s in 0..64u8 {
                    if self.b[s as usize] == 0 && rows.contains(&(s as usize / 8)) {
                        v.push(Mv::Place(t, s));
                    }
                }
            }
            // dacă jumătatea e plină (nu se poate întâmpla: 32 pătrate, 16 piese) – sigur nevid
            return v;
        }
        let me = self.side.idx();
        let mut v = Vec::new();
        for m in self.pseudo() {
            let g = self.make(&m);
            if g.in_check(me) {
                continue;
            }
            if check_stalemate {
                // pat interzis
                let opp = 1 - me;
                if !g.in_check(opp) && g.legal_inner(false).is_empty() {
                    continue;
                }
            }
            v.push(m);
        }
        v
    }

    fn only_kings(&self) -> bool {
        self.b.iter().all(|&c| c == 0 || typ(c) == 1)
    }
}

fn idx(s: &str) -> usize {
    let b = s.as_bytes();
    (b[1] - b'1') as usize * 8 + (b[0] - b'a') as usize
}

impl Game for Sittuyin {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        self.legal_inner(true)
    }

    fn apply(&mut self, m: &Mv) {
        self.do_move(m);
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.placing() {
            return None;
        }
        if self.plies >= 300 || self.only_kings() {
            return Some(Outcome::Draw);
        }
        if self.legal_inner(false).is_empty() {
            // mat (sau pat forțat): câștigă cine a mutat
            return Some(if self.in_check(self.side.idx()) {
                Outcome::Win(self.side.other())
            } else {
                Outcome::Draw
            });
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let mut s = 0;
        for i in 0..64 {
            let c = self.b[i];
            if c == 0 {
                continue;
            }
            let o = col(c);
            let t = typ(c);
            let r = i / 8;
            let adv = if o == 0 { r } else { 7 - r } as i32;
            let mut v = VAL[t];
            if t == 6 {
                v += adv * 8;
            } else if t != 1 {
                let cf = (i % 8) as i32;
                v += 6 - (2 * cf - 7).abs() - (2 * r as i32 - 7).abs() / 2;
            }
            if o == p.idx() {
                s += v;
            } else {
                s -= v;
            }
        }
        if !self.placing() {
            if self.in_check(1 - p.idx()) {
                s += 30;
            }
            if self.in_check(p.idx()) {
                s -= 30;
            }
        }
        s
    }

    fn render(&self) -> String {
        let mut s = String::from("Sittuyin (J1 = majuscule K G E H R P, J2 = minuscule)\n\n");
        for r in (0..8).rev() {
            s.push_str(&format!("{} ", r + 1));
            for f in 0..8 {
                let c = self.b[r * 8 + f];
                s.push(if c == 0 {
                    '.'
                } else if col(c) == 0 {
                    LET[typ(c)]
                } else {
                    LET[typ(c)].to_ascii_lowercase()
                });
                s.push(' ');
            }
            s.push('\n');
        }
        s.push_str("  a b c d e f g h\n");
        if self.placing() {
            let l = |p: usize| {
                let n = ["K", "G", "E", "H", "R"];
                (0..5)
                    .filter(|&k| self.hand[p][k] > 0)
                    .map(|k| format!("{}×{}", n[k], self.hand[p][k]))
                    .collect::<Vec<_>>()
                    .join(" ")
            };
            s.push_str(&format!(
                "Plasare — de așezat: J1 [{}], J2 [{}]\n",
                l(0),
                l(1)
            ));
        } else if self.in_check(self.side.idx()) {
            s.push_str("Șah!\n");
        }
        match self.outcome() {
            Some(Outcome::Win(p)) => s.push_str(&format!("Mat. Câștigă J{}.\n", p.idx() + 1)),
            Some(Outcome::Draw) => s.push_str("Remiză.\n"),
            None => s.push_str(&format!("La mutare: J{}\n", self.side.idx() + 1)),
        }
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        match *m {
            Mv::Place(t, s) => format!("{}@{}", LET[t as usize], sqn(s)),
            Mv::Step(a, z) => format!(
                "{}{}{}",
                sqn(a),
                if self.b[z as usize] != 0 { 'x' } else { '-' },
                sqn(z)
            ),
            Mv::Promote(a) => format!("{}=G", sqn(a)),
        }
    }

    fn ai_style(&self) -> AiStyle {
        if self.placing() {
            AiStyle::AlphaBeta(1)
        } else {
            AiStyle::AlphaBetaTime {
                ms: 300,
                max_depth: 6,
            }
        }
    }

    fn move_score(&self, m: &Mv) -> i32 {
        match *m {
            Mv::Step(_, z) if self.b[z as usize] != 0 => 100 + VAL[typ(self.b[z as usize])] / 10,
            Mv::Promote(_) => 90,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    #[test]
    fn placement_then_play() {
        let mut g = Sittuyin::new();
        assert!(g.placing());
        let mut n = 0;
        while g.placing() {
            let m = g.legal_moves();
            assert!(!m.is_empty());
            // J1 doar pe rândurile 1–4
            if let Mv::Place(_, s) = m[0] {
                let r = s as usize / 8;
                assert_eq!(r < 4, g.side == Player::One);
            }
            g.apply(&m[n % m.len()]);
            n += 1;
        }
        assert_eq!(n, 16);
        assert_eq!(g.side, Player::One);
        assert!(!g.legal_moves().is_empty());
    }

    #[test]
    fn promotion_only_without_general() {
        let mut g = Sittuyin::new();
        g.b = [0; 64];
        g.hand = [[0; 5]; 2];
        g.b[idx("a1")] = 1;
        g.b[idx("h8")] = 1 + 8;
        g.b[idx("d5")] = 6;
        g.b[idx("c1")] = 2;
        assert!(!g.legal_moves().iter().any(|m| matches!(m, Mv::Promote(_))));
        g.b[idx("c1")] = 0;
        assert!(g.legal_moves().iter().any(|m| matches!(m, Mv::Promote(_))));
    }

    #[test]
    fn random_games_terminate_and_ai_works() {
        for seed in 1..=3u64 {
            let mut g: Box<dyn polytabletop_core::GameObj> = Box::new(Sittuyin::new());
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty(), "{}", g.render());
                g.play(&m[rng.below(m.len())]).unwrap();
                n += 1;
                assert!(n < 400);
            }
        }
        polytabletop_core::ai::set_time_budget_override_ms(Some(10));
        let mut g: Box<dyn polytabletop_core::GameObj> = Box::new(Sittuyin::new());
        for _ in 0..24 {
            let m = g.ai_move(5).unwrap();
            g.play(&m).unwrap();
        }
    }
}
