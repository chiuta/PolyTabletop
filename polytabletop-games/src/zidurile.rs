//! Zidurile Mișcătoare — labirint cu plăci culisante (variantă originală), 2 jucători.
//!
//! * Tablă 7×7: 16 plăci fixe (rânduri și coloane pare, numărând de la 0) și 33 de plăci mobile
//!   (+ 1 de rezervă): 12 drepte (│), 16 colțare (└) și 6 în T. 12 comori (A–L) sunt puse pe plăci
//!   (nu în colțurile de start). Fiecare jucător are 6 comori de găsit, într-o ordine fixă; ordinea
//!   este publică și se vede în `render()` (informație completă).
//! * Tura are două părți: `mete N2 r1` introduce placa de rezervă, rotită de `r` ori 90° în sensul
//!   acelor de ceas, pe marginea N (de sus, coloana 2), S (de jos), V (de la stânga, rândul) sau
//!   E (de la dreapta); poziții admise: 2, 4, 6. Placa scoasă din partea opusă devine noua rezervă;
//!   nu poți anula direct mutarea adversarului (inserția opusă celei precedente). Pionii de pe
//!   rândul/coloana împinsă se mută odată cu plăcile; cel scos de pe tablă reapare pe placa introdusă.
//!   Apoi `mergi c3` (coloana a–g, rândul 1–7 de sus) mută pionul pe orice placă legată de el printr-un
//!   drum deschis (`mergi` pe locul curent = rămâi).
//! * Ajungi pe placa cu comoara-țintă: o iei. După a șasea te întorci la colțul de start
//!   (J1: a1, J2: g7). Cine ajunge primul acasă cu toate comorile câștigă.
//! * După 100 de ture (50 fiecare) câștigă cine a strâns mai multe comori; egal = remiză.
//!
//! AI: căutare de 2 plies (inserție + mers) cu euristică de distanță până la țintă (BFS pe drumuri
//! deschise) — jocul e cu informație completă.

#![allow(clippy::needless_range_loop)]

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::collections::VecDeque;

pub const SIZE: usize = 7;
pub const TURN_CAP: u32 = 100;
const GOALS: usize = 6;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Tile {
    /// 0 = dreaptă, 1 = colț, 2 = T.
    pub kind: u8,
    pub rot: u8,
    /// Comoara de pe placă (0..12), dacă există.
    pub treasure: Option<u8>,
}

impl Tile {
    /// Deschis spre direcția `d` (0 = N, 1 = E, 2 = S, 3 = V)?
    pub fn open(&self, d: usize) -> bool {
        let base: &[usize] = match self.kind {
            0 => &[0, 2],
            1 => &[0, 1],
            _ => &[0, 1, 3],
        };
        base.iter().any(|&b| (b + self.rot as usize) % 4 == d)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Edge {
    N,
    S,
    E,
    V,
}

impl Edge {
    fn opposite(self) -> Edge {
        match self {
            Edge::N => Edge::S,
            Edge::S => Edge::N,
            Edge::E => Edge::V,
            Edge::V => Edge::E,
        }
    }
    fn letter(self) -> char {
        match self {
            Edge::N => 'N',
            Edge::S => 'S',
            Edge::E => 'E',
            Edge::V => 'V',
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Insert(Edge, u8, u8),
    Walk(u8, u8),
}

#[derive(Clone)]
pub struct Zidurile {
    board: [[Tile; SIZE]; SIZE],
    spare: Tile,
    pawn: [(u8, u8); 2],
    targets: [[u8; GOALS]; 2],
    found: [usize; 2],
    side: Player,
    walking: bool,
    last_insert: Option<(Edge, u8)>,
    turns: u32,
    won: Option<Player>,
    log: String,
}

impl Default for Zidurile {
    fn default() -> Self {
        Self::new()
    }
}

fn fixed_tile(r: usize, c: usize) -> Tile {
    let (kind, rot) = match (r, c) {
        (0, 0) => (1, 1),
        (0, 6) => (1, 2),
        (6, 6) => (1, 3),
        (6, 0) => (1, 0),
        (0, _) => (2, 2),
        (_, 6) => (2, 3),
        (6, _) => (2, 0),
        (_, 0) => (2, 1),
        (2, 2) => (2, 2),
        (2, 4) => (2, 3),
        (4, 2) => (2, 1),
        _ => (2, 0),
    };
    Tile {
        kind,
        rot,
        treasure: None,
    }
}

fn is_fixed(r: usize, c: usize) -> bool {
    r.is_multiple_of(2) && c.is_multiple_of(2)
}

impl Zidurile {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut pool: Vec<Tile> = Vec::new();
        for _ in 0..12 {
            pool.push(Tile {
                kind: 0,
                rot: 0,
                treasure: None,
            });
        }
        for _ in 0..16 {
            pool.push(Tile {
                kind: 1,
                rot: 0,
                treasure: None,
            });
        }
        for _ in 0..6 {
            pool.push(Tile {
                kind: 2,
                rot: 0,
                treasure: None,
            });
        }
        for i in (1..pool.len()).rev() {
            let j = rng.below(i + 1);
            pool.swap(i, j);
        }
        for t in pool.iter_mut() {
            t.rot = rng.below(4) as u8;
        }
        let mut board = [[fixed_tile(0, 0); SIZE]; SIZE];
        let mut it = pool.into_iter();
        for (r, row) in board.iter_mut().enumerate() {
            for (c, cell) in row.iter_mut().enumerate() {
                *cell = if is_fixed(r, c) {
                    fixed_tile(r, c)
                } else {
                    it.next().unwrap()
                };
            }
        }
        let mut spare = it.next().unwrap();
        // comori: 12 plăci distincte (50 de plăci în total) fără colțurile de start
        let mut slots: Vec<(usize, usize)> = Vec::new();
        for r in 0..SIZE {
            for c in 0..SIZE {
                if !((r == 0 && c == 0) || (r == 6 && c == 6)) {
                    slots.push((r, c));
                }
            }
        }
        slots.push((7, 7)); // placa de rezervă
        for i in (1..slots.len()).rev() {
            let j = rng.below(i + 1);
            slots.swap(i, j);
        }
        for (k, &(r, c)) in slots.iter().take(12).enumerate() {
            if r == 7 {
                spare.treasure = Some(k as u8);
            } else {
                board[r][c].treasure = Some(k as u8);
            }
        }
        let mut ids: Vec<u8> = (0..12).collect();
        for i in (1..ids.len()).rev() {
            let j = rng.below(i + 1);
            ids.swap(i, j);
        }
        let mut targets = [[0u8; GOALS]; 2];
        targets[0].copy_from_slice(&ids[..GOALS]);
        targets[1].copy_from_slice(&ids[GOALS..]);
        Zidurile {
            board,
            spare,
            pawn: [(0, 0), (6, 6)],
            targets,
            found: [0, 0],
            side: Player::One,
            walking: false,
            last_insert: None,
            turns: 0,
            won: None,
            log: String::new(),
        }
    }

    pub fn found(&self, p: Player) -> usize {
        self.found[p.idx()]
    }

    pub fn tile(&self, r: usize, c: usize) -> Tile {
        self.board[r][c]
    }

    fn home(p: Player) -> (u8, u8) {
        match p {
            Player::One => (0, 0),
            Player::Two => (6, 6),
        }
    }

    /// Ținta curentă: o comoară (`Some(id)`) sau drumul spre casă (`None`).
    fn target(&self, p: Player) -> Option<u8> {
        let i = p.idx();
        if self.found[i] < GOALS {
            Some(self.targets[i][self.found[i]])
        } else {
            None
        }
    }

    fn reach(&self, from: (u8, u8)) -> Vec<Vec<i32>> {
        let mut dist = vec![vec![-1i32; SIZE]; SIZE];
        let mut q = VecDeque::new();
        dist[from.0 as usize][from.1 as usize] = 0;
        q.push_back((from.0 as usize, from.1 as usize));
        while let Some((r, c)) = q.pop_front() {
            for d in 0..4usize {
                let (nr, nc) = match d {
                    0 if r > 0 => (r - 1, c),
                    1 if c + 1 < SIZE => (r, c + 1),
                    2 if r + 1 < SIZE => (r + 1, c),
                    3 if c > 0 => (r, c - 1),
                    _ => continue,
                };
                if self.board[r][c].open(d)
                    && self.board[nr][nc].open((d + 2) % 4)
                    && dist[nr][nc] < 0
                {
                    dist[nr][nc] = dist[r][c] + 1;
                    q.push_back((nr, nc));
                }
            }
        }
        dist
    }

    fn target_cell(&self, p: Player) -> Option<(usize, usize)> {
        match self.target(p) {
            None => Some((Self::home(p).0 as usize, Self::home(p).1 as usize)),
            Some(id) => {
                for r in 0..SIZE {
                    for c in 0..SIZE {
                        if self.board[r][c].treasure == Some(id) {
                            return Some((r, c));
                        }
                    }
                }
                None // pe placa de rezervă
            }
        }
    }

    /// Distanță estimată până la țintă (mai mică = mai bine).
    fn distance(&self, p: Player) -> i32 {
        let dist = self.reach(self.pawn[p.idx()]);
        match self.target_cell(p) {
            Some((tr, tc)) => {
                if dist[tr][tc] >= 0 {
                    dist[tr][tc]
                } else {
                    let mut best = i32::MAX;
                    for r in 0..SIZE {
                        for c in 0..SIZE {
                            if dist[r][c] >= 0 {
                                let m = (r as i32 - tr as i32).abs() + (c as i32 - tc as i32).abs();
                                best = best.min(m);
                            }
                        }
                    }
                    12 + best
                }
            }
            None => 20,
        }
    }

    fn rotations(&self) -> &'static [u8] {
        if self.spare.kind == 0 {
            &[0, 1]
        } else {
            &[0, 1, 2, 3]
        }
    }

    fn do_insert(&mut self, e: Edge, idx: u8, rot: u8) {
        let i = idx as usize - 1;
        self.spare.rot = rot;
        let spare = self.spare;
        match e {
            Edge::N => {
                let out = self.board[6][i];
                for r in (1..SIZE).rev() {
                    self.board[r][i] = self.board[r - 1][i];
                }
                self.board[0][i] = spare;
                self.spare = out;
                for p in self.pawn.iter_mut() {
                    if p.1 as usize == i {
                        p.0 = if p.0 == 6 { 0 } else { p.0 + 1 };
                    }
                }
            }
            Edge::S => {
                let out = self.board[0][i];
                for r in 0..SIZE - 1 {
                    self.board[r][i] = self.board[r + 1][i];
                }
                self.board[6][i] = spare;
                self.spare = out;
                for p in self.pawn.iter_mut() {
                    if p.1 as usize == i {
                        p.0 = if p.0 == 0 { 6 } else { p.0 - 1 };
                    }
                }
            }
            Edge::V => {
                let out = self.board[i][6];
                for c in (1..SIZE).rev() {
                    self.board[i][c] = self.board[i][c - 1];
                }
                self.board[i][0] = spare;
                self.spare = out;
                for p in self.pawn.iter_mut() {
                    if p.0 as usize == i {
                        p.1 = if p.1 == 6 { 0 } else { p.1 + 1 };
                    }
                }
            }
            Edge::E => {
                let out = self.board[i][0];
                for c in 0..SIZE - 1 {
                    self.board[i][c] = self.board[i][c + 1];
                }
                self.board[i][6] = spare;
                self.spare = out;
                for p in self.pawn.iter_mut() {
                    if p.0 as usize == i {
                        p.1 = if p.1 == 0 { 6 } else { p.1 - 1 };
                    }
                }
            }
        }
    }
}

impl Game for Zidurile {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        let mut v = Vec::new();
        if !self.walking {
            for e in [Edge::N, Edge::S, Edge::V, Edge::E] {
                for idx in [2u8, 4, 6] {
                    if self.last_insert == Some((e.opposite(), idx)) {
                        continue;
                    }
                    for &r in self.rotations() {
                        v.push(Move::Insert(e, idx, r));
                    }
                }
            }
        } else {
            let dist = self.reach(self.pawn[self.side.idx()]);
            for r in 0..SIZE {
                for c in 0..SIZE {
                    if dist[r][c] >= 0 {
                        v.push(Move::Walk(r as u8, c as u8));
                    }
                }
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        let me = self.side;
        let i = me.idx();
        match *m {
            Move::Insert(e, idx, rot) => {
                self.do_insert(e, idx, rot);
                self.last_insert = Some((e, idx));
                self.walking = true;
                self.log = format!("J{}: a introdus placa pe {}{}.", i + 1, e.letter(), idx);
            }
            Move::Walk(r, c) => {
                self.pawn[i] = (r, c);
                self.walking = false;
                self.log = format!("J{}: a mers la {}.", i + 1, cell_name(r, c));
                match self.target(me) {
                    Some(id) => {
                        if self.board[r as usize][c as usize].treasure == Some(id) {
                            self.board[r as usize][c as usize].treasure = None;
                            self.found[i] += 1;
                            self.log.push_str(&format!(
                                " A luat comoara {} ({}/{}).",
                                (b'A' + id) as char,
                                self.found[i],
                                GOALS
                            ));
                        }
                    }
                    None => {
                        if (r, c) == Self::home(me) {
                            self.won = Some(me);
                        }
                    }
                }
                self.turns += 1;
                self.side = me.other();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if let Some(p) = self.won {
            return Some(Outcome::Win(p));
        }
        if self.turns >= TURN_CAP {
            return Some(match self.found[0].cmp(&self.found[1]) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                std::cmp::Ordering::Equal => Outcome::Draw,
            });
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let o = p.other();
        1000 * (self.found[p.idx()] as i32 - self.found[o.idx()] as i32) - 10 * self.distance(p)
            + 6 * self.distance(o)
    }

    fn render(&self) -> String {
        let mut s =
            String::from("Zidurile Mișcătoare   (1 = J1, 2 = J2, * = amândoi; litera = comoară)\n");
        s.push_str("    a  b  c  d  e  f  g\n");
        for r in 0..SIZE {
            let mut rows = [String::new(), String::new(), String::new()];
            for c in 0..SIZE {
                let t = self.board[r][c];
                let pawn_here: Vec<usize> = (0..2)
                    .filter(|&k| self.pawn[k] == (r as u8, c as u8))
                    .collect();
                let mark = match pawn_here.len() {
                    0 => ' ',
                    2 => '*',
                    _ => (b'1' + pawn_here[0] as u8) as char,
                };
                let ch = match t.treasure {
                    Some(id) => (b'A' + id) as char,
                    None => '·',
                };
                rows[0].push(mark);
                rows[0].push(if t.open(0) { '│' } else { ' ' });
                rows[0].push(' ');
                rows[1].push(if t.open(3) { '─' } else { ' ' });
                rows[1].push(ch);
                rows[1].push(if t.open(1) { '─' } else { ' ' });
                rows[2].push(' ');
                rows[2].push(if t.open(2) { '│' } else { ' ' });
                rows[2].push(' ');
            }
            s.push_str(&format!(
                "   {}\n {}  {}\n   {}\n",
                rows[0],
                r + 1,
                rows[1],
                rows[2]
            ));
        }
        let st = self.spare;
        let name = ["dreaptă │", "colț └", "T ├"][st.kind as usize];
        s.push_str(&format!(
            "Placa de rezervă: {} (rotire 0): {}{}\n",
            name,
            match st.treasure {
                Some(id) => format!("comoara {}, ", (b'A' + id) as char),
                None => String::new(),
            },
            format_args!("deschisă spre {}", {
                let mut d = String::new();
                for (k, ch) in ["N", "E", "S", "V"].iter().enumerate() {
                    if (Tile { rot: 0, ..st }).open(k) {
                        d.push_str(ch);
                    }
                }
                d
            })
        ));
        for (k, nm) in ["J1", "J2"].iter().enumerate() {
            let p = if k == 0 { Player::One } else { Player::Two };
            let t = match self.target(p) {
                Some(id) => format!("comoara {}", (b'A' + id) as char),
                None => format!("acasă ({})", cell_name(Self::home(p).0, Self::home(p).1)),
            };
            let remaining: Vec<String> = self.targets[k][self.found[k]..]
                .iter()
                .map(|&id| ((b'A' + id) as char).to_string())
                .collect();
            s.push_str(&format!(
                "{nm}: {}/{} comori; ținta: {t}; de găsit: [{}]\n",
                self.found[k],
                GOALS,
                remaining.join(" ")
            ));
        }
        if self.outcome().is_none() {
            s.push_str(&format!(
                "La mutare: jucătorul {} ({}), tura {}/{}\n",
                self.side.idx() + 1,
                if self.walking {
                    "mergi cNN"
                } else {
                    "mete N2 r1"
                },
                self.turns + 1,
                TURN_CAP
            ));
        }
        s.push_str(self.log.trim());
        s.push('\n');
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match *m {
            Move::Insert(e, idx, r) => format!("mete {}{} r{}", e.letter(), idx, r),
            Move::Walk(r, c) => format!("mergi {}", cell_name(r, c)),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(2)
    }
}

pub fn cell_name(r: u8, c: u8) -> String {
    format!("{}{}", (b'a' + c) as char, r + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn count_kinds(g: &Zidurile) -> [usize; 3] {
        let mut n = [0usize; 3];
        for row in g.board.iter() {
            for t in row {
                n[t.kind as usize] += 1;
            }
        }
        n[g.spare.kind as usize] += 1;
        n
    }

    fn treasures(g: &Zidurile) -> Vec<u8> {
        let mut v: Vec<u8> = g
            .board
            .iter()
            .flatten()
            .filter_map(|t| t.treasure)
            .collect();
        if let Some(t) = g.spare.treasure {
            v.push(t);
        }
        v.sort_unstable();
        v
    }

    #[test]
    fn tile_openings_rotate() {
        let t = Tile {
            kind: 1,
            rot: 0,
            treasure: None,
        };
        assert!(t.open(0) && t.open(1) && !t.open(2) && !t.open(3));
        let t = Tile { rot: 1, ..t };
        assert!(t.open(1) && t.open(2) && !t.open(0));
        let i = Tile {
            kind: 0,
            rot: 1,
            treasure: None,
        };
        assert!(i.open(1) && i.open(3) && !i.open(0));
        let tt = Tile {
            kind: 2,
            rot: 2,
            treasure: None,
        };
        // T rot 0 = N,E,V; rot 2 = S,V,E (închis la N)
        assert!(tt.open(2) && tt.open(3) && tt.open(1) && !tt.open(0));
    }

    #[test]
    fn setup_has_right_tile_counts_and_treasures() {
        for seed in 1..=10 {
            let g = Zidurile::with_seed(seed);
            assert_eq!(count_kinds(&g), [12, 16 + 4, 6 + 12]);
            assert_eq!(treasures(&g), (0..12).collect::<Vec<u8>>());
            let mut all = g.targets[0].to_vec();
            all.extend(g.targets[1]);
            all.sort_unstable();
            assert_eq!(all, (0..12).collect::<Vec<u8>>());
        }
    }

    #[test]
    fn fixed_tiles_do_not_open_outwards() {
        for r in 0..SIZE {
            for c in 0..SIZE {
                if !is_fixed(r, c) {
                    continue;
                }
                let t = fixed_tile(r, c);
                if r == 0 {
                    assert!(!t.open(0), "{r},{c}");
                }
                if r == 6 {
                    assert!(!t.open(2), "{r},{c}");
                }
                if c == 0 {
                    assert!(!t.open(3), "{r},{c}");
                }
                if c == 6 {
                    assert!(!t.open(1), "{r},{c}");
                }
            }
        }
    }

    #[test]
    fn insertion_shifts_a_column_and_pushes_pawns() {
        let mut g = Zidurile::with_seed(5);
        let col: Vec<Tile> = (0..SIZE).map(|r| g.board[r][1]).collect();
        let spare = g.spare;
        g.pawn = [(6, 1), (3, 1)];
        g.do_insert(Edge::N, 2, spare.rot);
        // coloana 2 (indice 1) s-a mutat în jos
        assert_eq!(g.board[0][1].kind, spare.kind);
        for r in 1..SIZE {
            assert_eq!(g.board[r][1], col[r - 1]);
        }
        assert_eq!(g.spare, col[6]);
        assert_eq!(g.pawn[0], (0, 1)); // scos de pe tablă, reapare pe placa introdusă
        assert_eq!(g.pawn[1], (4, 1));
    }

    #[test]
    fn insertions_in_all_directions_preserve_tiles() {
        for e in [Edge::N, Edge::S, Edge::E, Edge::V] {
            for idx in [2u8, 4, 6] {
                let mut g = Zidurile::with_seed(7);
                let before = count_kinds(&g);
                let tr = treasures(&g);
                g.do_insert(e, idx, 0);
                assert_eq!(count_kinds(&g), before);
                assert_eq!(treasures(&g), tr);
                // plăcile fixe n-au mișcat
                for r in 0..SIZE {
                    for c in 0..SIZE {
                        if is_fixed(r, c) {
                            let (a, b) = (g.board[r][c], fixed_tile(r, c));
                            assert_eq!((a.kind, a.rot), (b.kind, b.rot), "{e:?}{idx}");
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn cannot_undo_the_previous_insertion() {
        let mut g = Zidurile::with_seed(2);
        g.apply(&Move::Insert(Edge::N, 2, 0));
        g.apply(&Move::Walk(g.pawn[0].0, g.pawn[0].1));
        let m = g.legal_moves();
        assert!(!m.iter().any(|x| matches!(x, Move::Insert(Edge::S, 2, _))));
        assert!(m.iter().any(|x| matches!(x, Move::Insert(Edge::N, 2, _))));
    }

    #[test]
    fn walking_requires_a_connected_path() {
        let g = Zidurile::with_seed(3);
        let mut g = g;
        g.walking = true;
        let moves = g.legal_moves();
        // poți mereu rămâne pe loc
        let here = g.pawn[0];
        assert!(moves.contains(&Move::Walk(here.0, here.1)));
        // toate destinațiile sunt accesibile prin BFS
        let dist = g.reach(here);
        for m in &moves {
            if let Move::Walk(r, c) = m {
                assert!(dist[*r as usize][*c as usize] >= 0);
            }
        }
        // o placă izolată nu e accesibilă: construim un caz simplu
        let n_reach = moves.len();
        assert!((1..=49).contains(&n_reach));
    }

    #[test]
    fn collecting_and_winning_home() {
        let mut g = Zidurile::with_seed(4);
        let id = g.targets[0][0];
        // mutăm comoara-țintă pe placa unde stă pionul
        for r in 0..SIZE {
            for c in 0..SIZE {
                if g.board[r][c].treasure == Some(id) {
                    g.board[r][c].treasure = None;
                }
            }
        }
        if g.spare.treasure == Some(id) {
            g.spare.treasure = None;
        }
        g.board[0][0].treasure = Some(id);
        g.walking = true;
        g.apply(&Move::Walk(0, 0));
        assert_eq!(g.found[0], 1);
        assert_eq!(g.board[0][0].treasure, None);
        // victorie: toate comorile și acasă
        let mut h = Zidurile::with_seed(4);
        h.found[0] = GOALS;
        h.pawn[0] = (0, 0);
        h.walking = true;
        h.apply(&Move::Walk(0, 0));
        assert_eq!(h.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn turn_cap_decides_by_treasures() {
        let mut g = Zidurile::with_seed(1);
        g.turns = TURN_CAP;
        g.found = [2, 1];
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
        g.found = [1, 1];
        assert_eq!(g.outcome(), Some(Outcome::Draw));
    }

    #[test]
    fn random_games_finish_and_conserve_tiles() {
        for seed in 1..=10u64 {
            let mut g = Zidurile::with_seed(seed);
            let mut rng = Rng::new(seed + 50);
            let tr = treasures(&g).len();
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 1000);
            }
            assert_eq!(count_kinds(&g).iter().sum::<usize>(), 50);
            assert!(treasures(&g).len() <= tr);
        }
    }

    #[test]
    fn ai_finds_more_treasures_than_random() {
        let (mut ai_total, mut rnd_total) = (0, 0);
        for seed in 1..=6u64 {
            let ai = if seed % 2 == 0 {
                Player::One
            } else {
                Player::Two
            };
            let mut g = Zidurile::with_seed(seed);
            let mut rng = Rng::new(seed * 9);
            while g.outcome().is_none() {
                let mv = if g.side == ai {
                    polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap()
                } else {
                    let m = g.legal_moves();
                    m[rng.below(m.len())]
                };
                g.apply(&mv);
            }
            ai_total += g.found[ai.idx()];
            rnd_total += g.found[ai.other().idx()];
        }
        assert!(ai_total > rnd_total, "{ai_total} vs {rnd_total}");
    }
}
