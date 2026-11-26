//! Poieni — plăcuțe hexagonale de habitat și jetoane de animale, variantă originală pentru doi.
//!
//! Fiecare jucător își construiește un ținut din plăcuțe hexagonale. Fiecare muchie a unei
//! plăcuțe are un habitat: Pădure (P), Munte (M), Râu (R), Luncă (L), Mlaștină (B). Plăcuțele
//! au 1 habitat (cheie de boltă: acceptă un singur animal) sau 2 habitate (acceptă 2–3 animale).
//! Animale: Urs (U), Vidră (V), Zimbru (Z), Gaiță (G), Vulpe (F).
//!
//! Tura: iei din piață o pereche plăcuță + jeton (`ia 2`), sau, cu un jeton-natură, orice
//! plăcuță cu orice jeton (`nat 2 3`) ori schimbi cele 4 jetoane (`nat reset`); așezi plăcuța
//! lipită de ținutul tău, în orice rotire 0–5 (`pune 1,-1 r3`); apoi pui jetonul pe o plăcuță
//! liberă a ta care îl acceptă (`animal 0,1`) sau îl arunci (`arunca`). Un jeton pe o cheie de
//! boltă dă un jeton-natură. Trei jetoane la fel în piață se schimbă automat.
//!
//! 20 de ture pe jucător (plus 3 plăcuțe de start fiecare). Scor (regulile noastre):
//! * Urs: grupuri de exact 2 urși: 1 grup 5, 2 → 12, 3 → 20, 4+ → 30.
//! * Vidră: fiecare grup conex de vidre: 1→1, 2→3, 3→6, 4→10, 5+→15.
//! * Zimbru: fiecare turmă (grup conex) de exact 3 zimbri = 9.
//! * Gaiță: gaițe fără altă gaiță alături: 1→2, 2→5, 3→9, 4→14, 5→20, 6→27, 7+→35.
//! * Vulpe: fiecare vulpe = câte alte specii diferite are alături.
//! * Habitate: cel mai mare coridor al fiecărui habitat (plăcuțe legate prin muchii de același
//!   habitat) dă tot atâtea puncte; cine are cel mai mare coridor al unui habitat primește +3
//!   (egalitate: +1 fiecare). Fiecare jeton-natură rămas = 1 punct.
//!
//! Informație deschisă; întâmplarea e doar la pachet și la sacul de jetoane, iar AI-ul nu
//! citește piața (care se reumple la întâmplare).
#![allow(clippy::needless_range_loop, clippy::doc_lazy_continuation)]

use crate::cards::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const HAB: [char; 5] = ['P', 'M', 'R', 'L', 'B'];
pub const ANI: [char; 5] = ['U', 'V', 'Z', 'G', 'F'];
const DIRS: [(i32, i32); 6] = [(1, 0), (1, -1), (0, -1), (-1, 0), (-1, 1), (0, 1)];
const OFF: i32 = 13;
const W: usize = 27;
const TURNS: u32 = 20;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Tile {
    pub edges: [u8; 6],
    pub accept: u8,
}

impl Tile {
    fn mono(&self) -> bool {
        self.edges.iter().all(|e| *e == self.edges[0])
    }
    fn keystone(&self) -> bool {
        self.accept.count_ones() == 1
    }
    fn rotated(&self, rot: u8) -> Tile {
        let mut e = [0u8; 6];
        for i in 0..6 {
            e[(i + rot as usize) % 6] = self.edges[i];
        }
        Tile {
            edges: e,
            accept: self.accept,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Placed {
    q: i32,
    r: i32,
    tile: Tile,
    token: Option<u8>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Take(u8),
    TakeNat(u8, u8),
    Wipe,
    PutTile(i8, i8, u8),
    PutToken(u8),
    Discard,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Pick,
    Tile,
    Token,
    Done,
}

#[derive(Clone)]
pub struct Poieni {
    deck: Vec<Tile>,
    bag: Vec<u8>,
    market: [Option<(Tile, u8)>; 4],
    board: [Vec<Placed>; 2],
    grid: [[[i8; W]; W]; 2],
    nature: [u8; 2],
    side: Player,
    phase: Phase,
    held: Option<(Tile, u8)>,
    last_placed: usize,
    turns: u32,
    rng: Rng,
}

impl Default for Poieni {
    fn default() -> Self {
        Self::new()
    }
}

fn gen_tiles(rng: &mut Rng, n: usize) -> Vec<Tile> {
    let mut v = Vec::new();
    for i in 0..n {
        let h1 = (i % 5) as u8;
        if rng.below(100) < 28 {
            let a = rng.below(5) as u8;
            v.push(Tile {
                edges: [h1; 6],
                accept: 1 << a,
            });
        } else {
            let mut h2 = rng.below(5) as u8;
            if h2 == h1 {
                h2 = (h2 + 1 + rng.below(4) as u8) % 5;
            }
            let k = [2usize, 3, 3, 4][rng.below(4)];
            let mut e = [h1; 6];
            for j in 0..k {
                e[j] = h2;
            }
            // animale: 2 sau 3 distincte
            let na = 2 + rng.below(2);
            let mut acc = 0u8;
            while acc.count_ones() < na as u32 {
                acc |= 1 << rng.below(5);
            }
            v.push(Tile {
                edges: e,
                accept: acc,
            });
        }
    }
    shuffle(&mut v, rng);
    v
}

impl Poieni {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let deck = gen_tiles(&mut rng, 50);
        let mut bag: Vec<u8> = (0..60).map(|i| (i % 5) as u8).collect();
        shuffle(&mut bag, &mut rng);
        let mut g = Poieni {
            deck,
            bag,
            market: [None; 4],
            board: [Vec::new(), Vec::new()],
            grid: [[[-1; W]; W]; 2],
            nature: [0; 2],
            side: Player::One,
            phase: Phase::Pick,
            held: None,
            last_placed: 0,
            turns: 0,
            rng,
        };
        for p in 0..2 {
            for (q, r) in [(0, 0), (1, 0), (0, 1)] {
                let t = g.deck.pop().expect("tile");
                g.put(p, q, r, t);
            }
        }
        for i in 0..4 {
            g.refill(i);
        }
        g.fix_market();
        g
    }

    fn put(&mut self, p: usize, q: i32, r: i32, tile: Tile) -> usize {
        let idx = self.board[p].len();
        self.board[p].push(Placed {
            q,
            r,
            tile,
            token: None,
        });
        self.grid[p][(r + OFF) as usize][(q + OFF) as usize] = idx as i8;
        idx
    }

    fn at(&self, p: usize, q: i32, r: i32) -> Option<usize> {
        let (a, b) = (r + OFF, q + OFF);
        if a < 0 || b < 0 || a >= W as i32 || b >= W as i32 {
            return None;
        }
        let v = self.grid[p][a as usize][b as usize];
        if v < 0 {
            None
        } else {
            Some(v as usize)
        }
    }

    fn refill(&mut self, i: usize) {
        if let Some(t) = self.deck.pop() {
            let tok = self.bag.pop().unwrap_or(0);
            self.market[i] = Some((t, tok));
        } else {
            self.market[i] = None;
        }
    }

    /// Trei jetoane la fel: se schimbă; patru: toate.
    fn fix_market(&mut self) {
        for _ in 0..10 {
            let mut cnt = [0u8; 5];
            for m in self.market.iter().flatten() {
                cnt[m.1 as usize] += 1;
            }
            let Some(a) = (0..5).find(|a| cnt[*a] >= 3) else {
                return;
            };
            for i in 0..4 {
                if let Some(m) = self.market[i] {
                    if m.1 as usize == a {
                        self.bag.push(m.1);
                        let _ = i;
                    }
                }
            }
            shuffle(&mut self.bag, &mut self.rng);
            for i in 0..4 {
                if let Some(m) = self.market[i] {
                    if m.1 as usize == a {
                        let n = self.bag.pop().unwrap_or(0);
                        self.market[i] = Some((m.0, n));
                    }
                }
            }
        }
    }

    pub fn bag_total(&self) -> usize {
        self.bag.len()
            + self.market.iter().flatten().count()
            + self
                .board
                .iter()
                .flatten()
                .filter(|p| p.token.is_some())
                .count()
            + usize::from(self.held.is_some())
    }

    fn neighbors_of(&self, p: usize, idx: usize) -> Vec<usize> {
        let pl = &self.board[p][idx];
        (0..6)
            .filter_map(|d| self.at(p, pl.q + DIRS[d].0, pl.r + DIRS[d].1))
            .collect()
    }

    fn groups(&self, p: usize, animal: u8) -> Vec<usize> {
        let n = self.board[p].len();
        let mut seen = vec![false; n];
        let mut out = Vec::new();
        for s in 0..n {
            if seen[s] || self.board[p][s].token != Some(animal) {
                continue;
            }
            let mut size = 0;
            let mut st = vec![s];
            seen[s] = true;
            while let Some(x) = st.pop() {
                size += 1;
                for y in self.neighbors_of(p, x) {
                    if !seen[y] && self.board[p][y].token == Some(animal) {
                        seen[y] = true;
                        st.push(y);
                    }
                }
            }
            out.push(size);
        }
        out
    }

    pub fn animal_scores(&self, p: usize) -> [i32; 5] {
        let mut s = [0i32; 5];
        // Urs
        let pairs = self.groups(p, 0).iter().filter(|x| **x == 2).count();
        s[0] = [0, 5, 12, 20, 30][pairs.min(4)];
        // Vidră
        s[1] = self
            .groups(p, 1)
            .iter()
            .map(|x| [0, 1, 3, 6, 10, 15][(*x).min(5)])
            .sum();
        // Zimbru
        s[2] = 9 * self.groups(p, 2).iter().filter(|x| **x == 3).count() as i32;
        // Gaiță
        let lone = (0..self.board[p].len())
            .filter(|&i| {
                self.board[p][i].token == Some(3)
                    && !self
                        .neighbors_of(p, i)
                        .iter()
                        .any(|&j| self.board[p][j].token == Some(3))
            })
            .count();
        s[3] = [0, 2, 5, 9, 14, 20, 27, 35][lone.min(7)];
        // Vulpe
        for i in 0..self.board[p].len() {
            if self.board[p][i].token == Some(4) {
                let mut m = 0u8;
                for j in self.neighbors_of(p, i) {
                    if let Some(t) = self.board[p][j].token {
                        if t != 4 {
                            m |= 1 << t;
                        }
                    }
                }
                s[4] += m.count_ones() as i32;
            }
        }
        s
    }

    /// Cel mai mare coridor pentru fiecare habitat.
    pub fn corridors(&self, p: usize) -> [i32; 5] {
        let n = self.board[p].len();
        let mut out = [0i32; 5];
        for h in 0..5u8 {
            let mut seen = vec![false; n];
            for s in 0..n {
                if seen[s] || !self.board[p][s].tile.edges.contains(&h) {
                    continue;
                }
                let mut size = 0;
                let mut st = vec![s];
                seen[s] = true;
                while let Some(x) = st.pop() {
                    size += 1;
                    let a = self.board[p][x];
                    for d in 0..6 {
                        if a.tile.edges[d] != h {
                            continue;
                        }
                        if let Some(y) = self.at(p, a.q + DIRS[d].0, a.r + DIRS[d].1) {
                            if !seen[y] && self.board[p][y].tile.edges[(d + 3) % 6] == h {
                                seen[y] = true;
                                st.push(y);
                            }
                        }
                    }
                }
                out[h as usize] = out[h as usize].max(size);
            }
        }
        out
    }

    pub fn score(&self, p: usize) -> i32 {
        let a: i32 = self.animal_scores(p).iter().sum();
        let mine = self.corridors(p);
        let theirs = self.corridors(1 - p);
        let mut h = 0;
        for k in 0..5 {
            h += mine[k];
            if mine[k] > theirs[k] {
                h += 3;
            } else if mine[k] == theirs[k] {
                h += 1;
            }
        }
        a + h + i32::from(self.nature[p])
    }

    fn finish_turn(&mut self) {
        self.turns += 1;
        self.held = None;
        if self.turns >= TURNS * 2 {
            self.phase = Phase::Done;
            return;
        }
        self.side = self.side.other();
        self.phase = Phase::Pick;
    }

    fn place_options(&self, p: usize) -> Vec<(i32, i32)> {
        let mut seen = vec![false; W * W];
        let mut v = Vec::new();
        for pl in &self.board[p] {
            for d in 0..6 {
                let (q, r) = (pl.q + DIRS[d].0, pl.r + DIRS[d].1);
                if self.at(p, q, r).is_some() || q.abs() >= OFF || r.abs() >= OFF {
                    continue;
                }
                let k = ((r + OFF) as usize) * W + (q + OFF) as usize;
                if !seen[k] {
                    seen[k] = true;
                    v.push((q, r));
                }
            }
        }
        v.sort();
        v
    }

    fn heuristic(&self, p: usize) -> f64 {
        let mut s = f64::from(self.score(p));
        s += 1.5 * f64::from(self.nature[p]);
        // jetoane așezate promit; plăcuțe libere cu animale
        let free: f64 = self.board[p].iter().filter(|t| t.token.is_none()).count() as f64;
        s += 0.2 * free;
        s
    }
}

impl Game for Poieni {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        let p = self.side.idx();
        match self.phase {
            Phase::Done => Vec::new(),
            Phase::Pick => {
                let mut v = Vec::new();
                for i in 0..4u8 {
                    if self.market[i as usize].is_some() {
                        v.push(Move::Take(i));
                    }
                }
                if self.nature[p] > 0 {
                    for i in 0..4u8 {
                        for j in 0..4u8 {
                            if i != j
                                && self.market[i as usize].is_some()
                                && self.market[j as usize].is_some()
                            {
                                v.push(Move::TakeNat(i, j));
                            }
                        }
                    }
                    if self.market.iter().any(|m| m.is_some()) && !self.bag.is_empty() {
                        v.push(Move::Wipe);
                    }
                }
                v
            }
            Phase::Tile => {
                let (tile, _) = self.held.expect("held");
                let mut v = Vec::new();
                for (q, r) in self.place_options(p) {
                    let rots = if tile.mono() { 1 } else { 6 };
                    for rot in 0..rots {
                        v.push(Move::PutTile(q as i8, r as i8, rot));
                    }
                }
                v
            }
            Phase::Token => {
                let (_, tok) = self.held.expect("held");
                let mut v = Vec::new();
                for (i, pl) in self.board[p].iter().enumerate() {
                    if pl.token.is_none() && pl.tile.accept & (1 << tok) != 0 {
                        v.push(Move::PutToken(i as u8));
                    }
                }
                v.push(Move::Discard);
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        let p = self.side.idx();
        match *m {
            Move::Take(i) => {
                self.held = self.market[i as usize];
                self.refill(i as usize);
                self.fix_market();
                self.phase = Phase::Tile;
            }
            Move::TakeNat(i, j) => {
                self.nature[p] -= 1;
                let (t, _) = self.market[i as usize].expect("tile");
                let (_, tok) = self.market[j as usize].expect("tok");
                self.held = Some((t, tok));
                // jetonul perechii i trece la plăcuța perechii j; perechea i se reface
                let ti_tok = self.market[i as usize].map(|x| x.1).expect("t");
                let (tj_tile, _) = self.market[j as usize].expect("j");
                self.market[j as usize] = Some((tj_tile, ti_tok));
                self.refill(i as usize);
                self.fix_market();
                self.phase = Phase::Tile;
            }
            Move::Wipe => {
                self.nature[p] -= 1;
                for i in 0..4 {
                    if let Some(m) = self.market[i] {
                        self.bag.push(m.1);
                    }
                }
                shuffle(&mut self.bag, &mut self.rng);
                for i in 0..4 {
                    if let Some(m) = self.market[i] {
                        let n = self.bag.pop().unwrap_or(0);
                        self.market[i] = Some((m.0, n));
                    }
                }
                self.fix_market();
            }
            Move::PutTile(q, r, rot) => {
                let (tile, tok) = self.held.expect("held");
                let t = tile.rotated(rot);
                let idx = self.put(p, i32::from(q), i32::from(r), t);
                self.last_placed = idx;
                self.held = Some((tile, tok));
                self.phase = Phase::Token;
            }
            Move::PutToken(i) => {
                let (_, tok) = self.held.expect("held");
                let pl = &mut self.board[p][i as usize];
                pl.token = Some(tok);
                if pl.tile.keystone() {
                    self.nature[p] += 1;
                }
                self.finish_turn();
            }
            Move::Discard => {
                if let Some((_, tok)) = self.held {
                    self.bag.insert(0, tok);
                }
                self.finish_turn();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Done {
            return None;
        }
        let (a, b) = (self.score(0), self.score(1));
        Some(if a > b {
            Outcome::Win(Player::One)
        } else if b > a {
            Outcome::Win(Player::Two)
        } else if self.nature[0] != self.nature[1] {
            Outcome::Win(if self.nature[0] > self.nature[1] {
                Player::One
            } else {
                Player::Two
            })
        } else {
            Outcome::Draw
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        (self.heuristic(p.idx()) - self.heuristic(p.other().idx())) as i32
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(3)
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Poieni — tura {} din {} | scor J1 {} · J2 {}\n",
            self.turns / 2 + 1,
            TURNS,
            self.score(0),
            self.score(1)
        );
        s.push_str("Piața: ");
        let mk: Vec<String> = self
            .market
            .iter()
            .enumerate()
            .map(|(i, m)| match m {
                Some((t, tok)) => format!(
                    "{}:[{}|{}→{}]",
                    i + 1,
                    edge_str(&t.edges),
                    acc_str(t.accept),
                    ANI[*tok as usize]
                ),
                None => format!("{}:—", i + 1),
            })
            .collect();
        s.push_str(&mk.join("  "));
        s.push('\n');
        s.push_str(&format!(
            "Pachet {} | jetoane-natură: J1 {}, J2 {}\n",
            self.deck.len(),
            self.nature[0],
            self.nature[1]
        ));
        for p in 0..2 {
            let pr = self.corridors(p);
            let sc = self.animal_scores(p);
            s.push_str(&format!(
                "J{}: coridoare P{} M{} R{} L{} B{} | animale U{} V{} Z{} G{} F{}\n",
                p + 1,
                pr[0],
                pr[1],
                pr[2],
                pr[3],
                pr[4],
                sc[0],
                sc[1],
                sc[2],
                sc[3],
                sc[4]
            ));
            let (mut q0, mut q1, mut r0, mut r1) = (99, -99, 99, -99);
            for pl in &self.board[p] {
                q0 = q0.min(pl.q);
                q1 = q1.max(pl.q);
                r0 = r0.min(pl.r);
                r1 = r1.max(pl.r);
            }
            for r in r0..=r1 {
                s.push_str(&" ".repeat(((r - r0) * 4) as usize));
                for q in q0..=q1 {
                    match self.at(p, q, r) {
                        Some(i) => {
                            let pl = &self.board[p][i];
                            let t = match pl.token {
                                Some(a) => ANI[a as usize].to_string(),
                                None => acc_str(pl.tile.accept).to_lowercase(),
                            };
                            s.push_str(&format!("{}:{:<3} ", edge_str(&pl.tile.edges), t));
                        }
                        None => s.push_str("   ·       "),
                    }
                }
                s.push_str(&format!("   (r={r})\n"));
            }
            s.push_str(&format!("   q de la {q0} la {q1}\n"));
        }
        match self.phase {
            Phase::Done => s.push_str("Joc încheiat.\n"),
            Phase::Pick => s.push_str(&format!(
                "Mută J{}: ia o pereche (ia N).\n",
                self.side.idx() + 1
            )),
            Phase::Tile => {
                let (t, tok) = self.held.expect("held");
                s.push_str(&format!(
                    "Mută J{}: așază plăcuța [{}|{}] (pune q,r rN); jeton {}.\n",
                    self.side.idx() + 1,
                    edge_str(&t.edges),
                    acc_str(t.accept),
                    ANI[tok as usize]
                ));
            }
            Phase::Token => s.push_str(&format!(
                "Mută J{}: pune jetonul (animal q,r) sau arunca.\n",
                self.side.idx() + 1
            )),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        let p = self.side.idx();
        match *m {
            Move::Take(i) => format!("ia {}", i + 1),
            Move::TakeNat(i, j) => format!("nat {} {}", i + 1, j + 1),
            Move::Wipe => "nat reset".into(),
            Move::PutTile(q, r, rot) => format!("pune {q},{r} r{rot}"),
            Move::PutToken(i) => {
                let pl = &self.board[p][i as usize];
                format!("animal {},{}", pl.q, pl.r)
            }
            Move::Discard => "arunca".into(),
        }
    }
}

fn edge_str(e: &[u8; 6]) -> String {
    e.iter().map(|h| HAB[*h as usize]).collect()
}

fn acc_str(a: u8) -> String {
    (0..5)
        .filter(|i| a & (1 << i) != 0)
        .map(|i| ANI[i])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tile(e: [u8; 6], acc: u8) -> Tile {
        Tile {
            edges: e,
            accept: acc,
        }
    }

    fn empty() -> Poieni {
        let mut g = Poieni::with_seed(1);
        g.board = [Vec::new(), Vec::new()];
        g.grid = [[[-1; W]; W]; 2];
        g
    }

    #[test]
    fn corridor_follows_matching_edges() {
        let mut g = empty();
        // a la (0,0) cu muchia 0 (E) = Pădure; b la (1,0) cu muchia 3 (V) = Pădure
        g.put(0, 0, 0, tile([0, 1, 1, 1, 1, 1], 1));
        g.put(0, 1, 0, tile([1, 1, 1, 0, 1, 1], 1));
        g.put(0, 2, 0, tile([1, 1, 1, 1, 1, 1], 1));
        assert_eq!(g.corridors(0)[0], 2);
        assert_eq!(g.corridors(0)[1], 2); // muntele leagă b și c (b muchia 0 = M, c muchia 3 = M)
                                          // mismatch: mută tile c
        let mut g2 = empty();
        g2.put(0, 0, 0, tile([0, 1, 1, 1, 1, 1], 1));
        g2.put(0, 1, 0, tile([1, 1, 1, 1, 1, 1], 1));
        assert_eq!(g2.corridors(0)[0], 1);
    }

    #[test]
    fn animal_scoring_rules() {
        let mut g = empty();
        // rând de 5 plăcuțe; urși la 0,1 (pereche) și 3,4 (pereche) cu un gol între
        for q in 0..5 {
            g.put(0, q, 0, tile([0; 6], 0b11111));
        }
        for q in [0usize, 1, 3, 4] {
            g.board[0][q].token = Some(0);
        }
        assert_eq!(g.animal_scores(0)[0], 12);
        // vidre la 2: grup de 1
        g.board[0][2].token = Some(1);
        assert_eq!(g.animal_scores(0)[1], 1);
        // vulpe între urs și vidră: două specii
        g.board[0][2].token = Some(4);
        assert_eq!(g.animal_scores(0)[4], 1); // vecini: urs (1) și urs (3) = o singură specie
        g.board[0][3].token = Some(1);
        assert_eq!(g.animal_scores(0)[4], 2);
        // zimbri: 3 în grup
        let mut g = empty();
        for q in 0..4 {
            g.put(0, q, 0, tile([0; 6], 0b11111));
            g.board[0][q as usize].token = Some(2);
        }
        assert_eq!(g.animal_scores(0)[2], 0);
        g.board[0][3].token = None;
        assert_eq!(g.animal_scores(0)[2], 9);
        // gaițe singure
        let mut g = empty();
        for q in [0, 2, 4] {
            g.put(0, q, 0, tile([0; 6], 0b11111));
            let n = g.board[0].len() - 1;
            g.board[0][n].token = Some(3);
        }
        assert_eq!(g.animal_scores(0)[3], 9);
    }

    #[test]
    fn keystone_gives_nature_token_and_wipe_costs_one() {
        let mut g = Poieni::with_seed(5);
        g.held = Some((tile([0; 6], 0b1), 0));
        g.board[0][0].tile.accept = 0b1;
        g.board[0][0].token = None;
        g.phase = Phase::Token;
        g.apply(&Move::PutToken(0));
        assert_eq!(g.nature[0], 1);
        assert_eq!(g.to_move(), Player::Two);
        g.nature[1] = 1;
        assert!(g.legal_moves().contains(&Move::Wipe));
    }

    #[test]
    fn market_never_has_three_alike_after_fix() {
        for seed in 1..50 {
            let g = Poieni::with_seed(seed);
            let mut cnt = [0; 5];
            for m in g.market.iter().flatten() {
                cnt[m.1 as usize] += 1;
            }
            assert!(cnt.iter().all(|c| *c < 3), "seed {seed} {cnt:?}");
        }
    }

    #[test]
    fn random_games_finish_and_conserve_tokens() {
        for seed in 1..25u64 {
            let mut g = Poieni::with_seed(seed);
            let total = g.bag_total();
            let mut rng = Rng::new(seed ^ 9);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 1500);
            }
            // jetoane aruncate se întorc în sac: totalul rămâne 60
            assert_eq!(g.bag_total(), total, "seed {seed}");
            assert_eq!(g.board[0].len() + g.board[1].len(), 6 + 40);
        }
    }

    #[test]
    fn ai_beats_random() {
        let mut wins = 0;
        for seed in 1..=12u64 {
            let mut g = Poieni::with_seed(seed);
            let mut rng = Rng::new(seed + 3);
            while g.outcome().is_none() {
                let m = if g.to_move() == Player::One {
                    polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap()
                } else {
                    let l = g.legal_moves();
                    l[rng.below(l.len())]
                };
                g.apply(&m);
            }
            if g.outcome() == Some(Outcome::Win(Player::One)) {
                wins += 1;
            }
        }
        assert!(wins >= 10, "wins {wins}");
    }
}
