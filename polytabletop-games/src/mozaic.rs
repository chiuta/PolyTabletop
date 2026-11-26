//! Mozaic — joc original de plasat plăcuțe cu drumuri, cetăți și mănăstiri
//! (2 jucători).
//!
//! Mecanica „tragi o plăcuță, o lipești de cele existente cu muchiile
//! potrivite, poți pune un pion pe ce ai construit și puncte la închiderea
//! unui element” este generică; setul de plăcuțe (65, desenat de noi), numele
//! și textele sunt ale noastre. Simplificare: **fără câmpuri / fermieri**
//! (pionii se pun doar pe drumuri, cetăți și mănăstiri).
//!
//! * Muchiile unei plăcuțe: `F` câmp, `R` drum, `C` cetate, în ordinea N E S V.
//!   Plăcuța trasă se lipește de o plăcuță existentă cu muchiile identice
//!   (`pune(x,y)rK`: rotația K = 0–3 sferturi de tură în sens orar; x spre
//!   est, y spre nord, plăcuța de start e (0,0)). Dacă nu se poate așeza
//!   nicăieri: `arunca`.
//! * Apoi pui un pion pe un element al plăcuței (`drum:N`, `oras:E`,
//!   `manastire`) dacă elementul (cu tot ce e legat de el) n-are pion, sau
//!   `fara`. Fiecare are 7 pioni.
//! * Cetatea terminată: 2 puncte pe plăcuță + 2 pe scut; drumul terminat: 1 pe
//!   plăcuță; mănăstirea înconjurată de 8 plăcuțe: 9. Majoritatea pionilor ia
//!   punctele (la egalitate, ambii); pionii se întorc.
//! * La final (plăcuțe terminate) se numără și elementele neterminate: cetate
//!   1 pe plăcuță + 1 pe scut, drum 1 pe plăcuță, mănăstire 1 + vecinii.
//!
//! Plăcuțele se trag la întâmplare; AI-ul nu cunoaște ordinea pachetului și la
//! evaluare ignoră plăcuța trasă pentru adversar.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::collections::HashMap;

const F: u8 = 0;
const R: u8 = 1;
const C: u8 = 2;

struct TileDef {
    edges: [u8; 4],
    joined: bool,
    monastery: bool,
    shield: bool,
    count: u8,
}

const fn t(edges: [u8; 4], joined: bool, monastery: bool, shield: bool, count: u8) -> TileDef {
    TileDef {
        edges,
        joined,
        monastery,
        shield,
        count,
    }
}

const START_DEF: usize = 10;

const TILES: [TileDef; 21] = [
    t([F, F, F, F], false, true, false, 4),
    t([F, F, R, F], false, true, false, 2),
    t([C, C, C, C], true, false, true, 1),
    t([C, C, F, C], true, false, false, 3),
    t([C, C, F, C], true, false, true, 1),
    t([C, C, R, C], true, false, false, 2),
    t([C, F, F, C], true, false, false, 3),
    t([C, F, F, C], true, false, true, 2),
    t([C, R, R, C], true, false, false, 3),
    t([C, F, F, F], false, false, false, 5),
    t([C, R, F, R], false, false, false, 3),
    t([C, R, R, F], false, false, false, 3),
    t([C, F, R, R], false, false, false, 3),
    t([C, R, R, R], false, false, false, 3),
    t([C, F, C, F], false, false, false, 2),
    t([C, F, C, F], true, false, false, 1),
    t([C, C, F, F], false, false, false, 2),
    t([F, R, F, R], false, false, false, 8),
    t([F, F, R, R], false, false, false, 9),
    t([F, R, R, R], false, false, false, 4),
    t([R, R, R, R], false, false, false, 1),
];

const DX: [i32; 4] = [0, 1, 0, -1];
const DY: [i32; 4] = [1, 0, -1, 0];
const KIND_ROAD: u8 = 1;
const KIND_CITY: u8 = 2;
const KIND_MON: u8 = 3;
const MEEPLES: u8 = 7;

pub fn total_tiles() -> usize {
    TILES.iter().map(|d| d.count as usize).sum()
}

fn rotated(e: [u8; 4], rot: u8) -> [u8; 4] {
    let mut o = [0; 4];
    for (i, slot) in o.iter_mut().enumerate() {
        *slot = e[(i + 4 - rot as usize % 4) % 4];
    }
    o
}

/// Segmentele locale ale unei plăcuțe: (tip, masca muchiilor).
fn local_segments(edges: [u8; 4], joined: bool, monastery: bool) -> Vec<(u8, u8)> {
    let mut v = Vec::new();
    let cities: Vec<usize> = (0..4).filter(|&i| edges[i] == C).collect();
    if !cities.is_empty() {
        if joined {
            v.push((KIND_CITY, cities.iter().fold(0u8, |m, &i| m | (1 << i))));
        } else {
            for &i in &cities {
                v.push((KIND_CITY, 1 << i));
            }
        }
    }
    let roads: Vec<usize> = (0..4).filter(|&i| edges[i] == R).collect();
    if roads.len() == 2 {
        v.push((KIND_ROAD, roads.iter().fold(0u8, |m, &i| m | (1 << i))));
    } else {
        for &i in &roads {
            v.push((KIND_ROAD, 1 << i));
        }
    }
    if monastery {
        v.push((KIND_MON, 0));
    }
    v
}

#[derive(Clone, Debug)]
struct Placed {
    x: i32,
    y: i32,
    edges: [u8; 4],
    monastery: bool,
    /// ids globale ale segmentelor, în ordinea locală
    segs: Vec<usize>,
    edge_seg: [Option<usize>; 4],
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Place(i32, i32, u8),
    Discard,
    /// Pion pe segmentul local cu acest index.
    Meeple(u8),
    NoMeeple,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Place,
    Meeple(usize),
}

#[derive(Clone)]
pub struct Mozaic {
    placed: Vec<Placed>,
    cell: HashMap<(i32, i32), usize>,
    // segmente globale (union-find)
    parent: Vec<usize>,
    kind: Vec<u8>,
    open: Vec<i32>,
    tiles: Vec<Vec<u32>>,
    shields: Vec<i32>,
    meeples: Vec<[u8; 2]>,
    scored: Vec<bool>,
    pool: Vec<usize>,
    current: Option<usize>,
    meeples_left: [u8; 2],
    score: [i32; 2],
    side: Player,
    phase: Phase,
    over: bool,
}

impl Default for Mozaic {
    fn default() -> Self {
        Self::new()
    }
}

impl Mozaic {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut pool: Vec<usize> = Vec::new();
        for (i, d) in TILES.iter().enumerate() {
            let n = d.count as usize - usize::from(i == START_DEF);
            pool.extend(std::iter::repeat_n(i, n));
        }
        for i in (1..pool.len()).rev() {
            let j = rng.below(i + 1);
            pool.swap(i, j);
        }
        let mut g = Mozaic {
            placed: Vec::new(),
            cell: HashMap::new(),
            parent: Vec::new(),
            kind: Vec::new(),
            open: Vec::new(),
            tiles: Vec::new(),
            shields: Vec::new(),
            meeples: Vec::new(),
            scored: Vec::new(),
            pool,
            current: None,
            meeples_left: [MEEPLES; 2],
            score: [0; 2],
            side: Player::One,
            phase: Phase::Place,
            over: false,
        };
        g.put_tile(START_DEF, 0, 0, 0);
        g.current = g.pool.pop();
        g
    }

    pub fn score(&self, p: Player) -> i32 {
        self.score[p.idx()]
    }

    pub fn meeples_left(&self, p: Player) -> u8 {
        self.meeples_left[p.idx()]
    }

    pub fn tiles_on_board(&self) -> usize {
        self.placed.len()
    }

    pub fn tiles_remaining(&self) -> usize {
        self.pool.len() + usize::from(self.current.is_some())
    }

    pub fn current_tile(&self) -> Option<[u8; 4]> {
        self.current.map(|d| TILES[d].edges)
    }

    fn find(&self, mut a: usize) -> usize {
        while self.parent[a] != a {
            a = self.parent[a];
        }
        a
    }

    fn union_ends(&mut self, a: usize, b: usize) {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra == rb {
            self.open[ra] -= 2;
            return;
        }
        self.parent[rb] = ra;
        self.open[ra] = self.open[ra] + self.open[rb] - 2;
        let tb = std::mem::take(&mut self.tiles[rb]);
        for t in tb {
            if !self.tiles[ra].contains(&t) {
                self.tiles[ra].push(t);
            }
        }
        self.shields[ra] += self.shields[rb];
        let mb = self.meeples[rb];
        self.meeples[ra][0] += mb[0];
        self.meeples[ra][1] += mb[1];
        self.meeples[rb] = [0, 0];
    }

    /// Așază fizic plăcuța (fără verificări) și leagă segmentele de vecini.
    fn put_tile(&mut self, def: usize, x: i32, y: i32, rot: u8) -> usize {
        let d = &TILES[def];
        let edges = rotated(d.edges, rot);
        let tid = self.placed.len();
        let segs_local = local_segments(edges, d.joined, d.monastery);
        let mut ids = Vec::new();
        let mut edge_seg = [None; 4];
        for &(k, mask) in &segs_local {
            let id = self.parent.len();
            self.parent.push(id);
            self.kind.push(k);
            self.open.push(mask.count_ones() as i32);
            self.tiles.push(vec![tid as u32]);
            self.shields
                .push(if k == KIND_CITY && d.shield { 1 } else { 0 });
            self.meeples.push([0, 0]);
            self.scored.push(false);
            for (i, slot) in edge_seg.iter_mut().enumerate() {
                if mask & (1 << i) != 0 {
                    *slot = Some(id);
                }
            }
            ids.push(id);
        }
        self.placed.push(Placed {
            x,
            y,
            edges,
            monastery: d.monastery,
            segs: ids,
            edge_seg,
        });
        self.cell.insert((x, y), tid);
        for i in 0..4 {
            if edges[i] == F {
                continue;
            }
            if let Some(&n) = self.cell.get(&(x + DX[i], y + DY[i])) {
                if n == tid {
                    continue;
                }
                let mine = self.placed[tid].edge_seg[i];
                let theirs = self.placed[n].edge_seg[(i + 2) % 4];
                if let (Some(a), Some(b)) = (mine, theirs) {
                    self.union_ends(a, b);
                }
            }
        }
        tid
    }

    fn fits(&self, def: usize, x: i32, y: i32, rot: u8) -> bool {
        if self.cell.contains_key(&(x, y)) {
            return false;
        }
        let e = rotated(TILES[def].edges, rot);
        let mut any = false;
        for i in 0..4 {
            if let Some(&n) = self.cell.get(&(x + DX[i], y + DY[i])) {
                any = true;
                if self.placed[n].edges[(i + 2) % 4] != e[i] {
                    return false;
                }
            }
        }
        any
    }

    fn placements(&self, def: usize) -> Vec<(i32, i32, u8)> {
        let mut spots: Vec<(i32, i32)> = Vec::new();
        for p in &self.placed {
            for i in 0..4 {
                let c = (p.x + DX[i], p.y + DY[i]);
                if !self.cell.contains_key(&c) && !spots.contains(&c) {
                    spots.push(c);
                }
            }
        }
        spots.sort_unstable();
        let mut out = Vec::new();
        for (x, y) in spots {
            let mut seen: Vec<[u8; 4]> = Vec::new();
            for rot in 0..4u8 {
                let e = rotated(TILES[def].edges, rot);
                if seen.contains(&e) {
                    continue;
                }
                if self.fits(def, x, y, rot) {
                    seen.push(e);
                    out.push((x, y, rot));
                }
            }
        }
        out
    }

    fn neighbors8(&self, x: i32, y: i32) -> usize {
        let mut n = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if (dx != 0 || dy != 0) && self.cell.contains_key(&(x + dx, y + dy)) {
                    n += 1;
                }
            }
        }
        n
    }

    fn majority(&self, root: usize) -> Vec<Player> {
        let m = self.meeples[root];
        if m[0] == 0 && m[1] == 0 {
            return Vec::new();
        }
        match m[0].cmp(&m[1]) {
            std::cmp::Ordering::Greater => vec![Player::One],
            std::cmp::Ordering::Less => vec![Player::Two],
            std::cmp::Ordering::Equal => vec![Player::One, Player::Two],
        }
    }

    fn group_points(&self, root: usize, final_count: bool) -> i32 {
        let n = self.tiles[root].len() as i32;
        match self.kind[root] {
            KIND_CITY => {
                if final_count {
                    n + self.shields[root]
                } else {
                    2 * n + 2 * self.shields[root]
                }
            }
            KIND_ROAD => n,
            _ => 0,
        }
    }

    fn score_group(&mut self, root: usize, final_count: bool) {
        if self.scored[root] {
            return;
        }
        let winners = self.majority(root);
        if !winners.is_empty() {
            let pts = self.group_points(root, final_count);
            for w in &winners {
                self.score[w.idx()] += pts;
            }
        }
        // pionii se întorc
        for p in 0..2 {
            self.meeples_left[p] += self.meeples[root][p];
            self.meeples[root][p] = 0;
        }
        self.scored[root] = true;
    }

    fn score_monastery(&mut self, tid: usize, final_count: bool) {
        let t = &self.placed[tid];
        if !t.monastery {
            return;
        }
        let seg = *t.segs.last().unwrap();
        let (x, y) = (t.x, t.y);
        if self.scored[seg] || self.meeples[seg] == [0, 0] {
            return;
        }
        let n = self.neighbors8(x, y);
        if n == 8 || final_count {
            let pts = 1 + n as i32;
            let owner = if self.meeples[seg][0] > 0 {
                Player::One
            } else {
                Player::Two
            };
            self.score[owner.idx()] += pts;
            self.meeples_left[owner.idx()] += 1;
            self.meeples[seg] = [0, 0];
            self.scored[seg] = true;
        }
    }

    fn score_after_move(&mut self, tid: usize) {
        let segs = self.placed[tid].segs.clone();
        for s in segs {
            let r = self.find(s);
            if (self.kind[r] == KIND_CITY || self.kind[r] == KIND_ROAD) && self.open[r] == 0 {
                self.score_group(r, false);
            }
        }
        let (x, y) = (self.placed[tid].x, self.placed[tid].y);
        for dx in -1..=1 {
            for dy in -1..=1 {
                if let Some(&n) = self.cell.get(&(x + dx, y + dy)) {
                    self.score_monastery(n, false);
                }
            }
        }
    }

    fn final_scoring(&mut self) {
        let n = self.parent.len();
        for s in 0..n {
            if self.find(s) == s && (self.kind[s] == KIND_CITY || self.kind[s] == KIND_ROAD) {
                self.score_group(s, true);
            }
        }
        for tid in 0..self.placed.len() {
            self.score_monastery(tid, true);
        }
        self.over = true;
    }

    fn next_tile(&mut self) {
        self.current = self.pool.pop();
        if self.current.is_none() {
            self.final_scoring();
        }
    }

    fn seg_name(&self, tid: usize, local: usize) -> String {
        let seg = self.placed[tid].segs[local];
        match self.kind[seg] {
            KIND_MON => "manastire".to_string(),
            k => {
                let first = (0..4)
                    .find(|&i| self.placed[tid].edge_seg[i] == Some(seg))
                    .unwrap_or(0);
                format!(
                    "{}:{}",
                    if k == KIND_ROAD { "drum" } else { "oras" },
                    ['N', 'E', 'S', 'V'][first]
                )
            }
        }
    }

    /// Valoare estimată a elementelor deschise (pentru AI).
    fn potential(&self, p: Player) -> i32 {
        let mut s = 0;
        for r in 0..self.parent.len() {
            if self.parent[r] != r || self.scored[r] {
                continue;
            }
            let m = self.meeples[r];
            if m == [0, 0] {
                continue;
            }
            let mine = m[p.idx()];
            let theirs = m[p.other().idx()];
            let v = match self.kind[r] {
                KIND_MON => {
                    let seg_tile = self
                        .placed
                        .iter()
                        .find(|t| t.monastery && *t.segs.last().unwrap() == r);
                    match seg_tile {
                        Some(t) => 1 + self.neighbors8(t.x, t.y) as i32,
                        None => 1,
                    }
                }
                _ => {
                    // șanse de închidere mai mici cu multe capete deschise
                    let pts = self.group_points(r, false);
                    (pts * 6 / (self.open[r].max(1) + 2)).max(1)
                }
            };
            if mine > theirs {
                s += v;
            } else if theirs > mine {
                s -= v;
            }
        }
        s
    }
}

impl Game for Mozaic {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over {
            return Vec::new();
        }
        match self.phase {
            Phase::Place => {
                let def = self.current.expect("plăcuță curentă");
                let pl = self.placements(def);
                if pl.is_empty() {
                    vec![Move::Discard]
                } else {
                    pl.into_iter()
                        .map(|(x, y, r)| Move::Place(x, y, r))
                        .collect()
                }
            }
            Phase::Meeple(tid) => {
                let mut v = vec![Move::NoMeeple];
                if self.meeples_left[self.side.idx()] > 0 {
                    for (i, &seg) in self.placed[tid].segs.iter().enumerate() {
                        let r = self.find(seg);
                        if self.meeples[r] == [0, 0] && !self.scored[r] {
                            v.push(Move::Meeple(i as u8));
                        }
                    }
                }
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        match *m {
            Move::Place(x, y, rot) => {
                let def = self.current.take().expect("plăcuță curentă");
                let tid = self.put_tile(def, x, y, rot);
                self.phase = Phase::Meeple(tid);
            }
            Move::Discard => {
                self.side = self.side.other();
                self.next_tile();
            }
            Move::Meeple(i) => {
                if let Phase::Meeple(tid) = self.phase {
                    let seg = self.placed[tid].segs[i as usize];
                    let r = self.find(seg);
                    self.meeples[r][self.side.idx()] += 1;
                    self.meeples_left[self.side.idx()] -= 1;
                    self.score_after_move(tid);
                    self.phase = Phase::Place;
                    self.side = self.side.other();
                    self.next_tile();
                }
            }
            Move::NoMeeple => {
                if let Phase::Meeple(tid) = self.phase {
                    self.score_after_move(tid);
                    self.phase = Phase::Place;
                    self.side = self.side.other();
                    self.next_tile();
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            return None;
        }
        Some(match self.score[0].cmp(&self.score[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    /// Nu citește plăcuța trasă pentru următorul jucător.
    fn evaluate(&self, p: Player) -> i32 {
        let q = p.other();
        let mut s = 10 * (self.score[p.idx()] - self.score[q.idx()]);
        s += 8 * self.potential(p);
        s += 4 * (self.meeples_left[p.idx()] as i32 - self.meeples_left[q.idx()] as i32);
        s
    }

    fn render(&self) -> String {
        let mut s = String::from(
            "Mozaic — muchii N E S V (F câmp, R drum, C cetate), M mănăstire, ①② pion\n",
        );
        let (mut x0, mut x1, mut y0, mut y1) = (i32::MAX, i32::MIN, i32::MAX, i32::MIN);
        for p in &self.placed {
            x0 = x0.min(p.x);
            x1 = x1.max(p.x);
            y0 = y0.min(p.y);
            y1 = y1.max(p.y);
        }
        let letter = |e: u8| ['F', 'R', 'C'][e as usize];
        let mut head = String::from("     ");
        for x in x0..=x1 {
            head.push_str(&format!("{:^8}", x));
        }
        s.push_str(head.trim_end());
        s.push('\n');
        for y in (y0..=y1).rev() {
            let mut line = format!("{:>3}  ", y);
            for x in x0..=x1 {
                match self.cell.get(&(x, y)) {
                    Some(&tid) => {
                        let t = &self.placed[tid];
                        let mut cell: String = t.edges.iter().map(|&e| letter(e)).collect();
                        cell.push(if t.monastery { 'M' } else { '.' });
                        let mut mk = ' ';
                        for &seg in &t.segs {
                            let m = self.meeples[self.find(seg)];
                            if m[0] > 0 {
                                mk = '①';
                            }
                            if m[1] > 0 {
                                mk = '②';
                            }
                        }
                        // marcăm pionul elementului (grupului) din care face parte plăcuța
                        cell.push(mk);
                        line.push_str(&format!("{:<8}", cell));
                    }
                    None => line.push_str("        "),
                }
            }
            s.push_str(line.trim_end());
            s.push('\n');
        }
        s.push_str(&format!(
            "Puncte: J1 {} ({} pioni), J2 {} ({} pioni); plăcuțe rămase: {}\n",
            self.score[0],
            self.meeples_left[0],
            self.score[1],
            self.meeples_left[1],
            self.tiles_remaining()
        ));
        if self.over {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        match self.phase {
            Phase::Place => {
                if let Some(d) = self.current {
                    let e = TILES[d].edges;
                    s.push_str(&format!(
                        "Plăcuța curentă (rotația 0): N={} E={} S={} V={}{}{}\n",
                        letter(e[0]),
                        letter(e[1]),
                        letter(e[2]),
                        letter(e[3]),
                        if TILES[d].monastery {
                            ", mănăstire"
                        } else {
                            ""
                        },
                        if TILES[d].shield { ", scut" } else { "" }
                    ));
                }
            }
            Phase::Meeple(tid) => {
                let names: Vec<String> = (0..self.placed[tid].segs.len())
                    .map(|i| self.seg_name(tid, i))
                    .collect();
                s.push_str(&format!(
                    "Pui un pion? Elemente ale plăcuței: {} (sau fara)\n",
                    names.join(", ")
                ));
            }
        }
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Place(x, y, r) => format!("pune({x},{y})r{r}"),
            Move::Discard => "arunca".into(),
            Move::NoMeeple => "fara".into(),
            Move::Meeple(i) => match self.phase {
                Phase::Meeple(tid) => self.seg_name(tid, *i as usize),
                _ => "pion".into(),
            },
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tile_set_size() {
        assert_eq!(total_tiles(), 65);
        let g = Mozaic::with_seed(1);
        assert_eq!(g.tiles_on_board(), 1);
        assert_eq!(g.tiles_remaining(), 64);
    }

    #[test]
    fn rotation_and_segments() {
        assert_eq!(rotated([C, R, F, F], 1), [F, C, R, F]);
        assert_eq!(rotated([C, R, F, F], 4), [C, R, F, F]);
        // curbă de drum = un singur segment; răscruce = trei
        assert_eq!(local_segments([F, F, R, R], false, false).len(), 1);
        assert_eq!(local_segments([F, R, R, R], false, false).len(), 3);
        assert_eq!(local_segments([C, F, C, F], false, false).len(), 2);
        assert_eq!(local_segments([C, F, C, F], true, false).len(), 1);
        assert_eq!(local_segments([F, F, F, F], false, true).len(), 1);
    }

    #[test]
    fn placement_respects_edges() {
        let mut g = Mozaic::with_seed(2);
        // plăcuța de start: N=C E=R S=F V=R
        g.current = Some(0); // mănăstire FFFF
        let m = g.legal_moves();
        for mv in &m {
            if let Move::Place(x, y, r) = mv {
                assert!(g.fits(0, *x, *y, *r));
            }
        }
        // FFFF nu poate fi lipită pe muchia C (nord) sau R (est/vest)
        assert!(!m.contains(&Move::Place(0, 1, 0)));
        assert!(!m.contains(&Move::Place(1, 0, 0)));
        assert!(!m.contains(&Move::Place(-1, 0, 0)));
        assert!(m.contains(&Move::Place(0, -1, 0)));
    }

    #[test]
    fn city_closed_gives_points_and_meeple_back() {
        let mut g = Mozaic::with_seed(3);
        g.current = Some(9);
        g.apply(&Move::Place(0, 1, 2));
        let city = g
            .legal_moves()
            .into_iter()
            .find(|m| matches!(m, Move::Meeple(_)))
            .unwrap();
        // cetatea e deja închisă (2 plăcuțe, fără muchii deschise): pionul se pune și se ia îndată
        g.apply(&city);
        assert_eq!(g.score(Player::One), 4);
        assert_eq!(g.meeples_left(Player::One), 7);
    }

    #[test]
    fn monastery_scores_nine_when_surrounded() {
        let mut g = Mozaic::with_seed(4);
        // simulăm: mănăstire la (0,-1) cu pion al lui J1, apoi 8 plăcuțe în jur (manual)
        g.current = Some(0);
        g.apply(&Move::Place(0, -1, 0));
        let mon = g
            .legal_moves()
            .into_iter()
            .find(|m| matches!(m, Move::Meeple(_)))
            .unwrap();
        g.apply(&mon);
        assert_eq!(g.meeples_left(Player::One), 6);
        // umplem manual cele 7 celule rămase din jur cu câmpuri-fals (put_tile pe tipul FFFF nu
        // verifică muchiile) și apelăm punctarea
        let ring = [
            (-1, -2),
            (0, -2),
            (1, -2),
            (-1, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
        ];
        let mut last = 0;
        for (x, y) in ring {
            last = g.put_tile(0, x, y, 0);
        }
        // (0,0) e deja ocupat de start; acum sunt 8 vecini
        assert_eq!(g.neighbors8(0, -1), 8);
        let before = g.score(Player::One);
        g.score_after_move(last);
        assert_eq!(g.score(Player::One), before + 9);
        assert_eq!(g.meeples_left(Player::One), 7);
    }

    #[test]
    fn majority_and_ties() {
        let mut g = Mozaic::with_seed(5);
        let r = g.parent.len() - 1;
        g.meeples[r] = [1, 1];
        assert_eq!(g.majority(r).len(), 2);
        g.meeples[r] = [2, 1];
        assert_eq!(g.majority(r), vec![Player::One]);
    }

    #[test]
    fn random_games_finish_with_conservation() {
        for seed in 1..=15u64 {
            let mut g = Mozaic::with_seed(seed);
            let mut rng = Rng::new(seed + 200);
            let mut plies = 0;
            while Game::outcome(&g).is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                plies += 1;
                assert!(plies < 1000);
                // pioni: pe tablă + în rezervă = 7 per jucător
                for p in 0..2 {
                    let on_board: u8 = (0..g.parent.len())
                        .filter(|&r| g.parent[r] == r)
                        .map(|r| g.meeples[r][p])
                        .sum();
                    assert_eq!(on_board + g.meeples_left[p], MEEPLES);
                }
            }
            assert!(g.tiles_on_board() <= 65);
        }
    }

    #[test]
    fn all_tiles_get_used_or_discarded() {
        let mut g = Mozaic::with_seed(9);
        let mut rng = Rng::new(9);
        let mut discards = 0;
        while Game::outcome(&g).is_none() {
            let m = g.legal_moves();
            if m == vec![Move::Discard] {
                discards += 1;
            }
            g.apply(&m[rng.below(m.len())]);
        }
        assert_eq!(g.tiles_on_board() + discards, 65);
    }

    #[test]
    fn ai_not_worse_than_random() {
        let (mut ai, mut rnd) = (0, 0);
        for seed in 1..=8u64 {
            let ai_side = if seed % 2 == 0 {
                Player::One
            } else {
                Player::Two
            };
            let mut g = Mozaic::with_seed(seed);
            let mut rng = Rng::new(seed * 11);
            while Game::outcome(&g).is_none() {
                let m = if g.to_move() == ai_side {
                    polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap()
                } else {
                    let l = g.legal_moves();
                    l[rng.below(l.len())]
                };
                g.apply(&m);
            }
            match Game::outcome(&g) {
                Some(Outcome::Win(p)) if p == ai_side => ai += 1,
                Some(Outcome::Win(_)) => rnd += 1,
                _ => {}
            }
        }
        assert!(ai >= rnd, "AI {ai} vs random {rnd}");
    }
}
