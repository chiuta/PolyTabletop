//! Table chinezească (Sternhalma / Chinese Checkers, Germania), 2 jucători, din regulile publice.
//!
//! * Tabla în formă de stea cu 121 de găuri (17 rânduri: 1-2-3-4-13-12-11-10-9-10-11-12-13-4-3-2-1).
//!   Fiecare jucător are 10 piese în triunghiul său (J1 sus, J2 jos) și trebuie să ocupe triunghiul
//!   opus.
//! * O mutare: un pas pe o gaură vecină liberă (6 direcții) SAU o serie de salturi peste piese (ale
//!   oricui) într-o gaură liberă imediat dincolo, în aceeași direcție, cu salturi înlănțuite.
//!   Mutarea se scrie ca `de-la`, fără traseul (două trasee spre aceeași gaură = aceeași mutare).
//! * Câștigă primul care umple triunghiul opus (toate cele 10 găuri ocupate, cel puțin una a ta; un
//!   adversar care își lasă piese acolo nu te poate bloca la infinit pentru că se aplică și limita).
//! * Limita noastră de joc: după 300 de mutări se compară progresul (suma rândurilor parcurse);
//!   egalitate = remiză.
//!
//! Găurile se numesc cu litera rândului (a–q, de sus în jos) și numărul din rând (de la stânga):
//! `a1`, `e13`. Mutări: `d4-e5`.
#![allow(clippy::int_plus_one, clippy::needless_range_loop)]

use polytabletop_core::{hash_mix, AiStyle, Game, Outcome, Player};

const ROWS: usize = 17;
const COUNTS: [usize; ROWS] = [1, 2, 3, 4, 13, 12, 11, 10, 9, 10, 11, 12, 13, 4, 3, 2, 1];
const MAX_PLIES: u16 = 300;
const OFF: i32 = 12;
const WIDTH: usize = 25;

/// Tabelele statice ale tablei.
#[derive(Clone)]
struct Geo {
    /// (rând, x) -> id
    id: Vec<i16>,
    /// id -> (rând, x)
    pos: Vec<(i32, i32)>,
}

fn geo() -> Geo {
    let mut id = vec![-1i16; ROWS * WIDTH];
    let mut pos = Vec::new();
    for r in 0..ROWS {
        let c = COUNTS[r] as i32;
        let mut x = -(c - 1);
        while x <= c - 1 {
            id[r * WIDTH + (x + OFF) as usize] = pos.len() as i16;
            pos.push((r as i32, x));
            x += 2;
        }
    }
    Geo { id, pos }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Move {
    pub from: u8,
    pub to: u8,
}

#[derive(Clone)]
pub struct ChineseCheckers {
    geo: std::sync::Arc<Geo>,
    cell: Vec<u8>,
    side: Player,
    plies: u16,
    winner: Option<Outcome>,
}

impl Default for ChineseCheckers {
    fn default() -> Self {
        Self::new()
    }
}

impl ChineseCheckers {
    pub fn new() -> Self {
        let g = geo();
        let mut cell = vec![0u8; g.pos.len()];
        for (i, &(r, _)) in g.pos.iter().enumerate() {
            if r <= 3 {
                cell[i] = 1;
            } else if r >= 13 {
                cell[i] = 2;
            }
        }
        ChineseCheckers {
            geo: std::sync::Arc::new(g),
            cell,
            side: Player::One,
            plies: 0,
            winner: None,
        }
    }

    pub fn with_seed(_seed: u64) -> Self {
        Self::new()
    }

    fn at(&self, r: i32, x: i32) -> Option<usize> {
        if r < 0 || r >= ROWS as i32 || x + OFF < 0 || x + OFF >= WIDTH as i32 {
            return None;
        }
        let v = self.geo.id[r as usize * WIDTH + (x + OFF) as usize];
        if v < 0 {
            None
        } else {
            Some(v as usize)
        }
    }

    pub fn name(&self, i: usize) -> String {
        let (r, x) = self.geo.pos[i];
        let c = COUNTS[r as usize] as i32;
        format!("{}{}", (b'a' + r as u8) as char, (x + c - 1) / 2 + 1)
    }

    fn dests(&self, from: usize) -> Vec<usize> {
        let mut out = Vec::new();
        let (r, x) = self.geo.pos[from];
        const D: [(i32, i32); 6] = [(0, 2), (0, -2), (1, 1), (1, -1), (-1, 1), (-1, -1)];
        for (dr, dx) in D {
            if let Some(j) = self.at(r + dr, x + dx) {
                if self.cell[j] == 0 {
                    out.push(j);
                }
            }
        }
        // salturi (BFS), piesa mutată e scoasă din poziția inițială
        let mut seen = vec![false; self.cell.len()];
        seen[from] = true;
        let mut st = vec![from];
        while let Some(c) = st.pop() {
            let (cr, cx) = self.geo.pos[c];
            for (dr, dx) in D {
                if let (Some(m), Some(l)) =
                    (self.at(cr + dr, cx + dx), self.at(cr + 2 * dr, cx + 2 * dx))
                {
                    if self.cell[m] != 0 && m != from && self.cell[l] == 0 && !seen[l] {
                        seen[l] = true;
                        out.push(l);
                        st.push(l);
                    }
                }
            }
        }
        out.sort_unstable();
        out.dedup();
        out
    }

    fn progress(&self, p: Player) -> i32 {
        let t = p.idx() as u8 + 1;
        let mut s = 0;
        for (i, &(r, _)) in self.geo.pos.iter().enumerate() {
            if self.cell[i] == t {
                s += if p == Player::One { r } else { 16 - r };
            }
        }
        s
    }

    fn won(&self, p: Player) -> bool {
        let t = p.idx() as u8 + 1;
        let mut mine = 0;
        for (i, &(r, _)) in self.geo.pos.iter().enumerate() {
            let target = if p == Player::One { r >= 13 } else { r <= 3 };
            if target {
                if self.cell[i] == 0 {
                    return false;
                }
                if self.cell[i] == t {
                    mine += 1;
                }
            }
        }
        mine > 0
    }
}

impl Game for ChineseCheckers {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.winner.is_some() {
            return Vec::new();
        }
        let t = self.side.idx() as u8 + 1;
        let mut v = Vec::new();
        for i in 0..self.cell.len() {
            if self.cell[i] == t {
                for j in self.dests(i) {
                    v.push(Move {
                        from: i as u8,
                        to: j as u8,
                    });
                }
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        let t = self.side.idx() as u8 + 1;
        self.cell[m.from as usize] = 0;
        self.cell[m.to as usize] = t;
        self.plies += 1;
        if self.won(self.side) {
            self.winner = Some(Outcome::Win(self.side));
        } else if self.plies >= MAX_PLIES {
            let (a, b) = (self.progress(Player::One), self.progress(Player::Two));
            self.winner = Some(match a.cmp(&b) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                std::cmp::Ordering::Equal => Outcome::Draw,
            });
        }
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        self.winner
    }

    fn evaluate(&self, p: Player) -> i32 {
        // progres + compactitate (pe coloana centrală) pentru ambii
        let score = |q: Player| {
            let t = q.idx() as u8 + 1;
            let mut s = self.progress(q) * 10;
            for (i, &(_, x)) in self.geo.pos.iter().enumerate() {
                if self.cell[i] == t {
                    s -= x.abs();
                }
            }
            s
        };
        score(p) - score(p.other())
    }

    fn render(&self) -> String {
        let mut s = String::from("Table chinezească — X = J1 (sus → jos), O = J2 (jos → sus)\n");
        for r in 0..ROWS {
            let c = COUNTS[r];
            let pad = (WIDTH as i32 - (2 * c as i32 - 1)) / 2;
            s.push_str(&format!("{} ", (b'a' + r as u8) as char));
            s.push_str(&" ".repeat(pad as usize));
            for k in 0..c {
                let x = -(c as i32 - 1) + 2 * k as i32;
                let i = self.at(r as i32, x).unwrap();
                s.push(match self.cell[i] {
                    1 => 'X',
                    2 => 'O',
                    _ => '.',
                });
                s.push(' ');
            }
            s.push('\n');
        }
        s.push_str("(numărul din rând se numără de la stânga: a1, d4, e13 …)\n");
        match self.winner {
            Some(Outcome::Win(p)) => s.push_str(&format!("Jucătorul {} câștigă.\n", p.idx() + 1)),
            Some(Outcome::Draw) => s.push_str("Remiză.\n"),
            None => s.push_str(&format!(
                "La mutare: Jucătorul {} (mutarea {}/{MAX_PLIES})\n",
                self.side.idx() + 1,
                self.plies + 1
            )),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        format!(
            "{}-{}",
            self.name(m.from as usize),
            self.name(m.to as usize)
        )
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 500,
            max_depth: 3,
        }
    }

    fn hash_key(&self) -> Option<u64> {
        let mut h = self.side.idx() as u64;
        for (i, &c) in self.cell.iter().enumerate() {
            if c != 0 {
                h ^= hash_mix(((i as u64) << 3) | c as u64);
            }
        }
        Some(h)
    }

    fn move_score(&self, m: &Move) -> i32 {
        let (a, b) = (
            self.geo.pos[m.from as usize].0,
            self.geo.pos[m.to as usize].0,
        );
        let d = if self.side == Player::One {
            b - a
        } else {
            a - b
        };
        d * 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    #[test]
    fn board_size_and_start() {
        let g = ChineseCheckers::new();
        assert_eq!(g.cell.len(), 121);
        assert_eq!(g.cell.iter().filter(|&&c| c == 1).count(), 10);
        assert_eq!(g.cell.iter().filter(|&&c| c == 2).count(), 10);
    }

    #[test]
    fn hop_chain() {
        let mut g = ChineseCheckers::new();
        g.cell = vec![0; 121];
        // piesă în a1 (rândul 0), două piese în linie de salturi
        let a1 = g.at(0, 0).unwrap();
        let b = g.at(1, 1).unwrap();
        let c = g.at(2, 2).unwrap();
        g.cell[a1] = 1;
        g.cell[b] = 2;
        // salt peste b spre (2,2)
        let d = g.dests(a1);
        assert!(d.contains(&c));
        // alt salt: peste piesă din (3,3) spre (4,4)
        let e = g.at(3, 3).unwrap();
        let f = g.at(4, 4).unwrap();
        g.cell[e] = 1;
        assert!(g.dests(a1).contains(&f));
    }

    #[test]
    fn names_are_unique() {
        let g = ChineseCheckers::new();
        let mut v: Vec<String> = (0..121).map(|i| g.name(i)).collect();
        v.sort();
        v.dedup();
        assert_eq!(v.len(), 121);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=3u64 {
            let mut g = ChineseCheckers::new();
            let mut rng = Rng::new(seed);
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
            }
        }
    }
}
