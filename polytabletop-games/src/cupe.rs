//! Cupe încuibate, tablă 4×4. Fiecare jucător are 12 cupe în 3 stive de câte 4 mărimi
//! (1-4), încuibate, din care se poate lua doar cea de sus (cea mai mare rămasă). O tură:
//! fie pui o cupă din rezervă pe o celulă goală sau peste o cupă mai mică (a oricui), fie
//! muți o cupă proprie vizibilă de pe tablă pe o celulă goală sau peste o cupă mai mică.
//! Cupa din rezervă poate acoperi o cupă adversă doar dacă aceasta face parte dintr-un șir
//! de trei piese adverse vizibile pe aceeași linie. Câștigă cine are patru cupe vizibile
//! (cele de sus) pe un rând, o coloană sau o diagonală. Dacă ridicarea unei cupe descoperă
//! șirul de patru al adversarului, acesta câștigă (chiar dacă și tu închei un șir). Cine nu
//! mai poate muta pierde; remiză după 50 de semi-mutări fără punere din rezervă.
//!
//! Verificare surse: regulile publice ale jocurilor cu cupe încuibate (ridicare-descoperire,
//! acoperire condiționată din rezervă). Implementare proprie; nume și text originale.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const QUIET_LIMIT: u16 = 50;

const LINES: [[usize; 4]; 10] = [
    [0, 1, 2, 3],
    [4, 5, 6, 7],
    [8, 9, 10, 11],
    [12, 13, 14, 15],
    [0, 4, 8, 12],
    [1, 5, 9, 13],
    [2, 6, 10, 14],
    [3, 7, 11, 15],
    [0, 5, 10, 15],
    [3, 6, 9, 12],
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Mv {
    /// Celula de plecare (None = din rezervă).
    pub from: Option<u8>,
    pub size: u8,
    pub to: u8,
}

#[derive(Clone)]
pub struct Cupe {
    /// c[celulă][mărime-1] = proprietar (0 = nimeni, 1 = J1, 2 = J2).
    c: [[u8; 4]; 16],
    /// Cupele rămase în fiecare stivă din rezervă (vârful are mărimea = numărul).
    res: [[u8; 3]; 2],
    side: Player,
    quiet: u16,
    winner: Option<Player>,
}

impl Default for Cupe {
    fn default() -> Self {
        Self::new()
    }
}

fn name(i: usize) -> String {
    polytabletop_core::square_name(i % 4, i / 4)
}

fn top(c: &[u8; 4]) -> Option<(u8, u8)> {
    (0..4)
        .rev()
        .find(|&s| c[s] != 0)
        .map(|s| (s as u8 + 1, c[s]))
}

impl Cupe {
    pub fn new() -> Self {
        Cupe {
            c: [[0; 4]; 16],
            res: [[4; 3]; 2],
            side: Player::One,
            quiet: 0,
            winner: None,
        }
    }

    fn owner_at(&self, i: usize) -> u8 {
        top(&self.c[i]).map_or(0, |t| t.1)
    }

    fn has_line(&self, o: u8) -> bool {
        LINES
            .iter()
            .any(|l| l.iter().all(|&i| self.owner_at(i) == o))
    }

    /// Celule adverse acoperibile din rezervă: parte a unui șir de 3 piese vizibile ale lui `o`.
    fn threes(&self, o: u8) -> [bool; 16] {
        let mut t = [false; 16];
        for l in &LINES {
            if l.iter().filter(|&&i| self.owner_at(i) == o).count() >= 3 {
                for &i in l {
                    if self.owner_at(i) == o {
                        t[i] = true;
                    }
                }
            }
        }
        t
    }

    fn gen(&self) -> Vec<Mv> {
        let me = self.side.idx() as u8 + 1;
        let opp = 3 - me;
        let mut out = Vec::new();
        let th = self.threes(opp);
        let mut sizes = [false; 5];
        for &n in &self.res[self.side.idx()] {
            if n > 0 {
                sizes[n as usize] = true;
            }
        }
        for s in 1..=4u8 {
            if !sizes[s as usize] {
                continue;
            }
            for (to, &three) in th.iter().enumerate() {
                if let Some((ts, to_owner)) = top(&self.c[to]) {
                    if ts >= s || (to_owner == opp && !three) {
                        continue;
                    }
                }
                out.push(Mv {
                    from: None,
                    size: s,
                    to: to as u8,
                });
            }
        }
        for from in 0..16usize {
            let Some((s, o)) = top(&self.c[from]) else {
                continue;
            };
            if o != me {
                continue;
            }
            for to in 0..16usize {
                if to == from {
                    continue;
                }
                if let Some((ts, _)) = top(&self.c[to]) {
                    if ts >= s {
                        continue;
                    }
                }
                out.push(Mv {
                    from: Some(from as u8),
                    size: s,
                    to: to as u8,
                });
            }
        }
        out
    }
}

impl Game for Cupe {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if self.winner.is_some() || self.quiet >= QUIET_LIMIT {
            return Vec::new();
        }
        self.gen()
    }

    fn apply(&mut self, m: &Mv) {
        let me = self.side.idx() as u8 + 1;
        match m.from {
            None => {
                let k = self.res[self.side.idx()]
                    .iter()
                    .position(|&n| n == m.size)
                    .unwrap();
                self.res[self.side.idx()][k] -= 1;
                self.quiet = 0;
            }
            Some(f) => {
                self.c[f as usize][m.size as usize - 1] = 0;
                self.quiet += 1;
            }
        }
        self.c[m.to as usize][m.size as usize - 1] = me;
        if self.has_line(3 - me) {
            self.winner = Some(self.side.other());
        } else if self.has_line(me) {
            self.winner = Some(self.side);
        }
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        if let Some(w) = self.winner {
            return Some(Outcome::Win(w));
        }
        if self.quiet >= QUIET_LIMIT {
            return Some(Outcome::Draw);
        }
        if self.gen().is_empty() {
            return Some(Outcome::Win(self.side.other()));
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx() as u8 + 1;
        let mut v = 0;
        for l in &LINES {
            let (mut m, mut t) = (0, 0);
            for &i in l {
                let o = self.owner_at(i);
                if o == me {
                    m += 1;
                } else if o != 0 {
                    t += 1;
                }
            }
            let w = [0, 2, 10, 60, 0];
            if t == 0 {
                v += w[m];
            } else if m == 0 {
                v -= w[t];
            }
        }
        for c in &self.c {
            if let Some((s, o)) = top(c) {
                v += if o == me { s as i32 } else { -(s as i32) };
            }
        }
        // Rezerva: cupe mari în rezervă = flexibilitate.
        for k in 0..3 {
            v += self.res[p.idx()][k] as i32 - self.res[p.other().idx()][k] as i32;
        }
        v
    }

    fn render(&self) -> String {
        let mut s = String::from("     a    b    c    d\n");
        for r in (0..4).rev() {
            s.push_str(&format!("{}   ", r + 1));
            for c in 0..4 {
                let cell = &self.c[r * 4 + c];
                match top(cell) {
                    None => s.push_str(" .   "),
                    Some((sz, o)) => {
                        s.push(if o == 1 { 'X' } else { 'O' });
                        s.push_str(&format!(
                            "{}{} ",
                            sz,
                            if cell.iter().filter(|&&x| x != 0).count() > 1 {
                                '+'
                            } else {
                                ' '
                            }
                        ));
                        s.push(' ');
                    }
                }
            }
            s.push('\n');
        }
        s.push_str(&format!(
            "  rezerve X: {:?}  O: {:?} (mărimea cupei de sus din fiecare stivă)\n",
            self.res[0], self.res[1]
        ));
        s.push_str("  mutare: 3@b2 = pui din rezervă o cupă de mărime 3; b2-c3 = muți cupa vizibilă (+ = are cupe dedesubt)\n");
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        match m.from {
            None => format!("{}@{}", m.size, name(m.to as usize)),
            Some(f) => format!("{}-{}", name(f as usize), name(m.to as usize)),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 500,
            max_depth: 7,
        }
    }

    fn hash_key(&self) -> Option<u64> {
        let mut h = polytabletop_core::hash_mix(
            9 << 40 | (self.side.idx() as u64) << 32 | self.quiet as u64,
        );
        for p in 0..2 {
            for k in 0..3 {
                h ^= polytabletop_core::hash_mix(
                    1 << 48 | (p as u64) << 8 | (self.res[p][k] as u64) << 16 | k as u64,
                );
            }
        }
        for (i, c) in self.c.iter().enumerate() {
            for (s, &o) in c.iter().enumerate() {
                if o != 0 {
                    h ^=
                        polytabletop_core::hash_mix((o as u64) << 24 | (s as u64) << 16 | i as u64);
                }
            }
        }
        Some(h)
    }

    fn move_score(&self, m: &Mv) -> i32 {
        let cover = top(&self.c[m.to as usize]).is_some_and(|t| t.1 != self.side.idx() as u8 + 1);
        m.size as i32 + if cover { 5 } else { 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(s: &str) -> usize {
        let b = s.as_bytes();
        (b[1] - b'1') as usize * 4 + (b[0] - b'a') as usize
    }
    fn texts(g: &Cupe) -> Vec<String> {
        g.legal_moves().iter().map(|m| g.format_move(m)).collect()
    }
    fn play(g: &mut Cupe, s: &str) {
        let m = g
            .legal_moves()
            .into_iter()
            .find(|m| g.format_move(m) == s)
            .unwrap_or_else(|| panic!("illegal {s}"));
        g.apply(&m);
    }

    #[test]
    fn start_moves_are_by_size() {
        let g = Cupe::new();
        // 4 mărimi disponibile la început? doar mărimea 4 e vizibilă în rezervă
        assert_eq!(g.legal_moves().len(), 16);
        assert!(texts(&g).contains(&"4@a1".to_string()));
    }

    #[test]
    fn reserve_cannot_cover_unless_three() {
        let mut g = Cupe::new();
        play(&mut g, "4@a1");
        // J2 nu poate acoperi cupa 4 (nu există mai mare); o cupă mică nu acoperă.
        assert!(!texts(&g).contains(&"4@a1".to_string()));
        // șir de trei adverse + cupa mică adversă acoperibilă din rezervă
        let mut h = Cupe::new();
        h.c[at("a1")][0] = 1;
        h.c[at("b1")][0] = 1;
        h.c[at("c1")][0] = 1;
        h.c[at("a3")][0] = 1;
        h.side = Player::Two;
        let t = texts(&h);
        assert!(t.contains(&"4@a1".to_string()));
        assert!(!t.contains(&"4@a3".to_string()));
    }

    #[test]
    fn lifting_reveals_line_loses() {
        let mut g = Cupe::new();
        // J2 are linia de jos a1..d1 dintre care b1 acoperită de o cupă X mai mare; X o ridică.
        for c in ["a1", "c1", "d1"] {
            g.c[at(c)][1] = 2;
        }
        g.c[at("b1")][1] = 2;
        g.c[at("b1")][2] = 1; // X acoperă
        g.res = [[4, 4, 4], [4, 4, 4]];
        g.side = Player::One;
        play(&mut g, "b1-a2");
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
    }
}
