//! Insula — joc original de colonizare cu resurse pe hexagoane (2 jucători).
//!
//! Mecanica „hexagoane cu resurse și numere, colonii, orașe, drumuri,
//! tâlhar la 7” este generică; insula, resursele, numele și textele sunt ale
//! noastre. Simplificări: fără cărți de dezvoltare, fără porturi, fără
//! schimburi între jucători (doar schimb 4:1 cu banca), fără limită de resurse
//! în bancă.
//!
//! * Tabla: 19 hexagoane (pădure `l`, argilă `a`, pășune `p`, lan `g`, munte
//!   `m`, un deșert) cu numere 2–12 amestecate la fiecare joc; 54 de vârfuri
//!   `vN`, 72 de muchii `eN`, hexagoane `hN`.
//! * Început: pe rând J1, J2, J2, J1 pui o colonie (`colonie v12`) și un drum
//!   lipit de ea (`drum e7`); a doua colonie a fiecăruia dă câte o resursă din
//!   fiecare hexagon vecin.
//! * Tura: `arunc` (două zaruri; hexagoanele cu suma dau resurse proprietarilor
//!   coloniilor — 1 — și orașelor — 2). La 7: cine are peste 7 cărți aruncă
//!   jumătate (la întâmplare, automat), apoi mutați tâlharul (`bandit h5`) și
//!   furi o carte la întâmplare de la adversar, dacă are construcții acolo.
//!   Hexagonul cu tâlhar nu produce.
//! * Apoi, oricâte: `drum eN` (l a), `colonie vN` (l a p g; pe vârf liber, la
//!   cel puțin două muchii de orice construcție, legat de drumurile tale),
//!   `oras vN` (2 g + 3 m, pe colonia ta), `schimb l g` (4 cărți dintr-o
//!   resursă pentru 1 din alta), apoi `sfarsit`.
//! * Puncte: colonie 1, oraș 2, cel mai lung drum (cel puțin 5, strict mai lung
//!   decât al adversarului) 2. Primul la 10 puncte câștigă la rândul lui.
//! * Piese: 5 colonii, 4 orașe, 15 drumuri. După 1500 de mutări câștigă cine
//!   are mai multe puncte (egal = remiză).
//!
//! Informație ascunsă: `render` arată doar mâna Jucătorului 1; AI-ul nu se uită
//! la cărțile adversarului.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::collections::HashMap;
use std::sync::OnceLock;

const RES_LETTERS: [char; 5] = ['l', 'a', 'p', 'g', 'm'];
const RES_NAMES: [&str; 6] = ["pădure", "argilă", "pășune", "lan", "munte", "deșert"];
const DESERT: u8 = 5;
const PLY_CAP: u32 = 1500;
const WIN_VP: i32 = 10;

struct Board {
    hv: Vec<[usize; 6]>,
    vh: Vec<Vec<usize>>,
    edges: Vec<(usize, usize)>,
    /// pentru fiecare vârf: (vecin, muchie)
    vadj: Vec<Vec<(usize, usize)>>,
}

fn board() -> &'static Board {
    static B: OnceLock<Board> = OnceLock::new();
    B.get_or_init(|| {
        let mut coords: Vec<(i32, i32)> = Vec::new();
        for r in -2..=2i32 {
            for q in -2..=2i32 {
                if (q + r).abs() <= 2 {
                    coords.push((q, r));
                }
            }
        }
        let mut vkey: HashMap<(i32, i32), usize> = HashMap::new();
        let mut hv = Vec::new();
        let sqrt3 = 3f64.sqrt();
        for &(q, r) in &coords {
            let cx = sqrt3 * (q as f64 + r as f64 / 2.0);
            let cy = 1.5 * r as f64;
            let mut vs = [0usize; 6];
            for (k, slot) in vs.iter_mut().enumerate() {
                let a = (60.0 * k as f64 + 30.0).to_radians();
                let x = cx + a.cos();
                let y = cy + a.sin();
                let key = ((x * 100.0).round() as i32, (y * 100.0).round() as i32);
                let n = vkey.len();
                *slot = *vkey.entry(key).or_insert(n);
            }
            hv.push(vs);
        }
        let nv = vkey.len();
        let mut vh = vec![Vec::new(); nv];
        for (h, vs) in hv.iter().enumerate() {
            for &v in vs {
                vh[v].push(h);
            }
        }
        let mut edges: Vec<(usize, usize)> = Vec::new();
        let mut emap: HashMap<(usize, usize), usize> = HashMap::new();
        for vs in &hv {
            for k in 0..6 {
                let (a, b) = (vs[k], vs[(k + 1) % 6]);
                let key = (a.min(b), a.max(b));
                if let std::collections::hash_map::Entry::Vacant(e) = emap.entry(key) {
                    e.insert(edges.len());
                    edges.push(key);
                }
            }
        }
        let mut vadj = vec![Vec::new(); nv];
        for (e, &(a, b)) in edges.iter().enumerate() {
            vadj[a].push((b, e));
            vadj[b].push((a, e));
        }
        Board {
            hv,
            vh,
            edges,
            vadj,
        }
    })
}

pub fn vertex_count() -> usize {
    board().vh.len()
}

pub fn edge_count() -> usize {
    board().edges.len()
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Roll,
    Settle(u8),
    City(u8),
    Road(u8),
    Robber(u8),
    Trade(u8, u8),
    End,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    SetupSettle(u8),
    SetupRoad(u8, u8),
    Roll,
    Robber,
    Main,
}

const SETUP_ORDER: [Player; 4] = [Player::One, Player::Two, Player::Two, Player::One];

#[derive(Clone)]
pub struct Insula {
    /// Whose private info `render` shows (`None` = the mover's, hot-seat).
    viewer: Option<Player>,
    res: [u8; 19],
    num: [u8; 19],
    robber: usize,
    /// (jucător, oraș?)
    build: Vec<Option<(Player, bool)>>,
    road: Vec<Option<Player>>,
    hands: [[u8; 5]; 2],
    side: Player,
    phase: Phase,
    longest: Option<Player>,
    plies: u32,
    winner: Option<Outcome>,
    last: String,
    /// Cartea furată de tâlhar în ultima mutare (jucător, resursă): evaluarea nu o citește (nu se știa).
    stolen: Option<(usize, usize)>,
    rng: Rng,
}

impl Default for Insula {
    fn default() -> Self {
        Self::new()
    }
}

fn pips(n: u8) -> i32 {
    if n == 0 {
        0
    } else {
        6 - (7 - n as i32).abs()
    }
}

impl Insula {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut kinds: Vec<u8> = Vec::new();
        for (r, n) in [(0u8, 4), (1, 3), (2, 4), (3, 4), (4, 3), (DESERT, 1)] {
            kinds.extend(std::iter::repeat_n(r, n));
        }
        let mut nums: Vec<u8> = vec![2, 3, 3, 4, 4, 5, 5, 6, 6, 8, 8, 9, 9, 10, 10, 11, 11, 12];
        for v in [&mut kinds] {
            for i in (1..v.len()).rev() {
                let j = rng.below(i + 1);
                v.swap(i, j);
            }
        }
        for i in (1..nums.len()).rev() {
            let j = rng.below(i + 1);
            nums.swap(i, j);
        }
        let mut res = [0u8; 19];
        let mut num = [0u8; 19];
        let mut robber = 0;
        let mut it = nums.into_iter();
        for h in 0..19 {
            res[h] = kinds[h];
            if kinds[h] == DESERT {
                robber = h;
            } else {
                num[h] = it.next().expect("numere");
            }
        }
        Insula {
            viewer: None,
            res,
            num,
            robber,
            build: vec![None; vertex_count()],
            road: vec![None; edge_count()],
            hands: [[0; 5]; 2],
            side: Player::One,
            phase: Phase::SetupSettle(0),
            longest: None,
            plies: 0,
            winner: None,
            last: String::new(),
            stolen: None,
            rng,
        }
    }

    pub fn hand(&self, p: Player) -> [u8; 5] {
        self.hands[p.idx()]
    }

    pub fn hand_total(&self, p: Player) -> u32 {
        self.hands[p.idx()].iter().map(|&x| x as u32).sum()
    }

    pub fn robber_hex(&self) -> usize {
        self.robber
    }

    pub fn hex_resource(&self, h: usize) -> u8 {
        self.res[h]
    }

    pub fn hex_number(&self, h: usize) -> u8 {
        self.num[h]
    }

    pub fn building(&self, v: usize) -> Option<(Player, bool)> {
        self.build[v]
    }

    pub fn road_owner(&self, e: usize) -> Option<Player> {
        self.road[e]
    }

    pub fn longest_holder(&self) -> Option<Player> {
        self.longest
    }

    /// Puncte de victorie.
    pub fn vp(&self, p: Player) -> i32 {
        let mut v = 0;
        for b in self.build.iter().flatten() {
            if b.0 == p {
                v += if b.1 { 2 } else { 1 };
            }
        }
        if self.longest == Some(p) {
            v += 2;
        }
        v
    }

    fn count(&self, p: Player, city: bool) -> usize {
        self.build
            .iter()
            .flatten()
            .filter(|b| b.0 == p && b.1 == city)
            .count()
    }

    fn roads_of(&self, p: Player) -> usize {
        self.road.iter().filter(|r| **r == Some(p)).count()
    }

    fn dist_ok(&self, v: usize) -> bool {
        self.build[v].is_none()
            && board().vadj[v]
                .iter()
                .all(|&(n, _)| self.build[n].is_none())
    }

    fn own_road_at(&self, p: Player, v: usize) -> bool {
        board().vadj[v]
            .iter()
            .any(|&(_, e)| self.road[e] == Some(p))
    }

    fn can_settle_spot(&self, p: Player, v: usize) -> bool {
        self.dist_ok(v) && self.own_road_at(p, v)
    }

    fn road_connected(&self, p: Player, e: usize) -> bool {
        let (a, b) = board().edges[e];
        for v in [a, b] {
            match self.build[v] {
                Some((o, _)) if o == p => return true,
                Some(_) => continue,
                None => {}
            }
            if board().vadj[v]
                .iter()
                .any(|&(_, e2)| e2 != e && self.road[e2] == Some(p))
            {
                return true;
            }
        }
        false
    }

    fn afford(&self, p: Player, cost: [u8; 5]) -> bool {
        (0..5).all(|r| self.hands[p.idx()][r] >= cost[r])
    }

    fn pay(&mut self, p: Player, cost: [u8; 5]) {
        for (h, c) in self.hands[p.idx()].iter_mut().zip(cost) {
            *h -= c;
        }
    }

    pub fn longest_road_len(&self, p: Player) -> i32 {
        fn dfs(g: &Insula, p: Player, v: usize, used: &mut Vec<bool>) -> i32 {
            let mut best = 0;
            for &(n, e) in &board().vadj[v] {
                if used[e] || g.road[e] != Some(p) {
                    continue;
                }
                used[e] = true;
                let cont = match g.build[n] {
                    Some((o, _)) if o != p => 0,
                    _ => dfs(g, p, n, used),
                };
                best = best.max(1 + cont);
                used[e] = false;
            }
            best
        }
        let mut used = vec![false; edge_count()];
        (0..vertex_count())
            .map(|v| dfs(self, p, v, &mut used))
            .max()
            .unwrap_or(0)
    }

    fn update_longest(&mut self) {
        let l = [
            self.longest_road_len(Player::One),
            self.longest_road_len(Player::Two),
        ];
        match self.longest {
            Some(h) => {
                let o = h.other();
                if l[o.idx()] > l[h.idx()] && l[o.idx()] >= 5 {
                    self.longest = Some(o);
                } else if l[h.idx()] < 5 {
                    self.longest = if l[o.idx()] >= 5 { Some(o) } else { None };
                }
            }
            None => {
                let (a, b) = (l[0], l[1]);
                if a >= 5 && a > b {
                    self.longest = Some(Player::One);
                } else if b >= 5 && b > a {
                    self.longest = Some(Player::Two);
                }
            }
        }
    }

    fn check_win(&mut self) {
        if self.winner.is_some() {
            return;
        }
        for p in [Player::One, Player::Two] {
            if self.vp(p) >= WIN_VP && (p == self.side || self.vp(self.side) < WIN_VP) {
                self.winner = Some(Outcome::Win(p));
                return;
            }
        }
        if self.plies >= PLY_CAP {
            let (a, b) = (self.vp(Player::One), self.vp(Player::Two));
            self.winner = Some(match a.cmp(&b) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                std::cmp::Ordering::Equal => Outcome::Draw,
            });
        }
    }

    fn take_random_card(&mut self, p: Player) -> Option<usize> {
        let total = self.hand_total(p) as usize;
        if total == 0 {
            return None;
        }
        let mut k = self.rng.below(total);
        for r in 0..5 {
            let n = self.hands[p.idx()][r] as usize;
            if k < n {
                self.hands[p.idx()][r] -= 1;
                return Some(r);
            }
            k -= n;
        }
        None
    }

    fn produce(&mut self, sum: u8) {
        for h in 0..19 {
            if self.num[h] != sum || self.robber == h || self.res[h] == DESERT {
                continue;
            }
            for &v in &board().hv[h] {
                if let Some((p, city)) = self.build[v] {
                    self.hands[p.idx()][self.res[h] as usize] += if city { 2 } else { 1 };
                }
            }
        }
    }

    /// Producția așteptată a lui `p` (puncte pe zar), cu tâlharul și orașele.
    pub fn production(&self, p: Player) -> i32 {
        let mut s = 0;
        for h in 0..19 {
            if self.robber == h || self.res[h] == DESERT {
                continue;
            }
            for &v in &board().hv[h] {
                if let Some((o, city)) = self.build[v] {
                    if o == p {
                        s += pips(self.num[h]) * if city { 2 } else { 1 };
                    }
                }
            }
        }
        s
    }

    fn diversity(&self, p: Player) -> i32 {
        let mut seen = [false; 5];
        for h in 0..19 {
            if self.res[h] == DESERT {
                continue;
            }
            for &v in &board().hv[h] {
                if matches!(self.build[v], Some((o, _)) if o == p) {
                    seen[self.res[h] as usize] = true;
                }
            }
        }
        seen.iter().filter(|&&b| b).count() as i32
    }
}

const COST_ROAD: [u8; 5] = [1, 1, 0, 0, 0];
const COST_SETTLE: [u8; 5] = [1, 1, 1, 1, 0];
const COST_CITY: [u8; 5] = [0, 0, 0, 2, 3];

impl Game for Insula {
    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.winner.is_some() {
            return Vec::new();
        }
        let me = self.side;
        let mut v = Vec::new();
        match self.phase {
            Phase::SetupSettle(_) => {
                for x in 0..vertex_count() {
                    if self.dist_ok(x) {
                        v.push(Move::Settle(x as u8));
                    }
                }
            }
            Phase::SetupRoad(_, sv) => {
                for &(_, e) in &board().vadj[sv as usize] {
                    if self.road[e].is_none() {
                        v.push(Move::Road(e as u8));
                    }
                }
            }
            Phase::Roll => v.push(Move::Roll),
            Phase::Robber => {
                for h in 0..19 {
                    if h != self.robber {
                        v.push(Move::Robber(h as u8));
                    }
                }
            }
            Phase::Main => {
                if self.roads_of(me) < 15 && self.afford(me, COST_ROAD) {
                    for e in 0..edge_count() {
                        if self.road[e].is_none() && self.road_connected(me, e) {
                            v.push(Move::Road(e as u8));
                        }
                    }
                }
                if self.count(me, false) < 5 && self.afford(me, COST_SETTLE) {
                    for x in 0..vertex_count() {
                        if self.can_settle_spot(me, x) {
                            v.push(Move::Settle(x as u8));
                        }
                    }
                }
                if self.count(me, true) < 4 && self.afford(me, COST_CITY) {
                    for x in 0..vertex_count() {
                        if self.build[x] == Some((me, false)) {
                            v.push(Move::City(x as u8));
                        }
                    }
                }
                for a in 0..5u8 {
                    if self.hands[me.idx()][a as usize] >= 4 {
                        for b in 0..5u8 {
                            if a != b {
                                v.push(Move::Trade(a, b));
                            }
                        }
                    }
                }
                v.push(Move::End);
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        self.plies += 1;
        let me = self.side;
        self.last.clear();
        self.stolen = None;
        match *m {
            Move::Settle(x) => {
                let x = x as usize;
                self.build[x] = Some((me, false));
                if let Phase::SetupSettle(step) = self.phase {
                    if step >= 2 {
                        for &h in &board().vh[x] {
                            if self.res[h] != DESERT {
                                self.hands[me.idx()][self.res[h] as usize] += 1;
                            }
                        }
                    }
                    self.phase = Phase::SetupRoad(step, x as u8);
                } else {
                    self.pay(me, COST_SETTLE);
                    self.update_longest();
                }
            }
            Move::Road(e) => {
                let e = e as usize;
                self.road[e] = Some(me);
                if let Phase::SetupRoad(step, _) = self.phase {
                    if step == 3 {
                        self.side = Player::One;
                        self.phase = Phase::Roll;
                    } else {
                        self.side = SETUP_ORDER[step as usize + 1];
                        self.phase = Phase::SetupSettle(step + 1);
                    }
                } else {
                    self.pay(me, COST_ROAD);
                    self.update_longest();
                }
            }
            Move::City(x) => {
                self.pay(me, COST_CITY);
                self.build[x as usize] = Some((me, true));
            }
            Move::Trade(a, b) => {
                self.hands[me.idx()][a as usize] -= 4;
                self.hands[me.idx()][b as usize] += 1;
            }
            Move::Roll => {
                let d1 = self.rng.d6();
                let d2 = self.rng.d6();
                let sum = d1 + d2;
                self.last = format!("J{} aruncă {}+{} = {}", me.idx() + 1, d1, d2, sum);
                if sum == 7 {
                    for p in [Player::One, Player::Two] {
                        let total = self.hand_total(p);
                        if total > 7 {
                            for _ in 0..total / 2 {
                                self.take_random_card(p);
                            }
                        }
                    }
                    self.phase = Phase::Robber;
                } else {
                    self.produce(sum);
                    self.phase = Phase::Main;
                }
            }
            Move::Robber(h) => {
                let h = h as usize;
                self.robber = h;
                let opp = me.other();
                let adjacent = board().hv[h]
                    .iter()
                    .any(|&v| matches!(self.build[v], Some((o, _)) if o == opp));
                if adjacent {
                    if let Some(r) = self.take_random_card(opp) {
                        self.hands[me.idx()][r] += 1;
                        self.stolen = Some((me.idx(), r));
                    }
                }
                self.phase = Phase::Main;
            }
            Move::End => {
                self.side = me.other();
                self.phase = Phase::Roll;
            }
        }
        self.check_win();
    }

    fn outcome(&self) -> Option<Outcome> {
        self.winner
    }

    /// Doar informația lui `p`: tabla, mâna lui, numărul de cărți al adversarului.
    fn evaluate(&self, p: Player) -> i32 {
        let side = |x: Player| -> i32 {
            let mut s = 100 * self.vp(x);
            s += 12 * self.production(x);
            s += 25 * self.diversity(x);
            s += 6 * self.longest_road_len(x);
            s += 3 * self.roads_of(x) as i32;
            let spots = (0..vertex_count())
                .filter(|&v| self.can_settle_spot(x, v))
                .count() as i32;
            s += 10 * spots.min(3);
            s
        };
        let mut s = side(p) - side(p.other());
        // cartea furată chiar acum de tâlhar vine din mâna reală a adversarului: nu se numără
        let mut h = self.hands[p.idx()];
        if let Some((who, r)) = self.stolen {
            if who == p.idx() && h[r] > 0 {
                h[r] -= 1;
            }
        }
        let total: i32 = h.iter().map(|&x| x as i32).sum();
        s += 3 * total.min(9);
        if total > 7 {
            s -= 6 * (total - 7);
        }
        let can = |cost: [u8; 5]| (0..5).all(|r| h[r] >= cost[r]);
        if can(COST_SETTLE) {
            s += 40;
        }
        if can(COST_CITY) {
            s += 50;
        }
        // grâu/minereu pentru orașe
        s += 2 * (h[3].min(2) + h[4].min(3)) as i32;
        // cărțile adversarului: doar numărul (risc de tâlhar la 7)
        let opp_total = self.hand_total(p.other()) as i32;
        s -= (opp_total - 7).max(0);
        s
    }

    fn render(&self) -> String {
        let b = board();
        let mut s = String::from("Insula — hexagoane (id: resursă număr)\n");
        let mut line = String::new();
        for h in 0..19 {
            let cell = if self.res[h] == DESERT {
                format!("h{}: deșert", h)
            } else {
                format!(
                    "h{}: {} {}",
                    h, RES_LETTERS[self.res[h] as usize], self.num[h]
                )
            };
            let cell = if self.robber == h {
                format!("{cell} [tâlhar]")
            } else {
                cell
            };
            line.push_str(&format!("{:<24}", cell));
            if h % 4 == 3 {
                s.push_str(line.trim_end());
                s.push('\n');
                line.clear();
            }
        }
        if !line.is_empty() {
            s.push_str(line.trim_end());
            s.push('\n');
        }
        s.push_str(&format!(
            "(l pădure, a argilă, p pășune, g lan, m munte)\nColții hexagoanelor (vârfuri): {}\n",
            (0..19)
                .map(|h| format!(
                    "h{}={}",
                    h,
                    b.hv[h]
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .join("/")
                ))
                .collect::<Vec<_>>()
                .join(" ")
        ));
        for p in [Player::One, Player::Two] {
            let cons: Vec<String> = (0..vertex_count())
                .filter_map(|v| match self.build[v] {
                    Some((o, c)) if o == p => {
                        Some(format!("{}{}", if c { "oraș v" } else { "colonie v" }, v))
                    }
                    _ => None,
                })
                .collect();
            let roads: Vec<String> = (0..edge_count())
                .filter(|&e| self.road[e] == Some(p))
                .map(|e| format!("e{}", e))
                .collect();
            s.push_str(&format!(
                "J{}: {} puncte; construcții: {}; drumuri: {}\n",
                p.idx() + 1,
                self.vp(p),
                if cons.is_empty() {
                    "—".into()
                } else {
                    cons.join(", ")
                },
                if roads.is_empty() {
                    "—".into()
                } else {
                    roads.join(" ")
                }
            ));
        }
        if let Some(h) = self.longest {
            s.push_str(&format!("Cel mai lung drum: J{}\n", h.idx() + 1));
        }
        if !self.last.is_empty() {
            s.push_str(&format!("{}\n", self.last));
        }
        if self.winner.is_some() {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        let v = self.viewer.unwrap_or_else(|| self.to_move());
        let h = &self.hands[v.idx()];
        s.push_str(&format!(
            "Mâna Jucătorului {}: {}\n",
            v.idx() + 1,
            (0..5)
                .map(|r| format!("{}×{}", RES_LETTERS[r], h[r]))
                .collect::<Vec<_>>()
                .join(" ")
        ));
        s.push_str(&format!(
            "Jucătorul {}: {} cărți\n",
            v.other().idx() + 1,
            self.hand_total(v.other())
        ));
        match self.phase {
            Phase::SetupSettle(_) => s.push_str("Începutul: pune o colonie (colonie vN).\n"),
            Phase::SetupRoad(..) => {
                s.push_str("Începutul: pune un drum lângă colonie (drum eN).\n")
            }
            Phase::Roll => s.push_str("Aruncă zarurile (arunc).\n"),
            Phase::Robber => s.push_str("Mută tâlharul (bandit hN).\n"),
            Phase::Main => s.push_str("Construiește / schimbă / sfarsit.\n"),
        }
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        let _ = RES_NAMES;
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Roll => "arunc".into(),
            Move::Settle(v) => format!("colonie v{v}"),
            Move::City(v) => format!("oras v{v}"),
            Move::Road(e) => format!("drum e{e}"),
            Move::Robber(h) => format!("bandit h{h}"),
            Move::Trade(a, b) => format!(
                "schimb {} {}",
                RES_LETTERS[*a as usize], RES_LETTERS[*b as usize]
            ),
            Move::End => "sfarsit".into(),
        }
    }

    /// Adâncime 0 (o singură mutare): `AlphaBeta(1)` ar fi extins la Hard/Expert aruncarea (zaruri viitoare)
    /// și răspunsurile adversarului; evaluarea citește doar tabla, mâna proprie și numărul cărților adverse.
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn geometry_is_a_standard_island() {
        let b = board();
        assert_eq!(b.hv.len(), 19);
        assert_eq!(vertex_count(), 54);
        assert_eq!(edge_count(), 72);
        for v in 0..54 {
            assert!((2..=3).contains(&b.vadj[v].len()));
            assert!((1..=3).contains(&b.vh[v].len()));
        }
        assert_eq!(b.vh.iter().filter(|h| h.len() == 3).count(), 24);
    }

    #[test]
    fn tiles_and_numbers() {
        let g = Insula::with_seed(1);
        assert_eq!(g.res.iter().filter(|&&r| r == DESERT).count(), 1);
        assert_eq!(g.num.iter().filter(|&&n| n != 0).count(), 18);
        assert_eq!(g.num[g.robber], 0);
        assert_eq!(g.res[g.robber], DESERT);
    }

    fn setup_done(seed: u64) -> Insula {
        let mut g = Insula::with_seed(seed);
        let mut rng = Rng::new(seed);
        for _ in 0..8 {
            let m = g.legal_moves();
            g.apply(&m[rng.below(m.len())]);
        }
        assert_eq!(g.phase, Phase::Roll);
        assert_eq!(g.to_move(), Player::One);
        g
    }

    #[test]
    fn setup_snake_order_and_resources() {
        let mut g = Insula::with_seed(2);
        let mut order = Vec::new();
        let mut rng = Rng::new(3);
        while !matches!(g.phase, Phase::Roll) {
            order.push(g.to_move());
            let m = g.legal_moves();
            g.apply(&m[rng.below(m.len())]);
        }
        assert_eq!(
            order,
            vec![
                Player::One,
                Player::One,
                Player::Two,
                Player::Two,
                Player::Two,
                Player::Two,
                Player::One,
                Player::One
            ]
        );
        assert_eq!(g.count(Player::One, false), 2);
        assert_eq!(g.count(Player::Two, false), 2);
        assert_eq!(g.roads_of(Player::One), 2);
        // a doua colonie dă resurse (cel puțin una dacă nu e doar deșert)
        let _ = g.hand_total(Player::One);
    }

    #[test]
    fn distance_rule() {
        let mut g = Insula::with_seed(4);
        g.apply(&Move::Settle(0));
        let nbrs: Vec<usize> = board().vadj[0].iter().map(|&(n, _)| n).collect();
        // trece la drum, apoi J2 nu poate pune lângă
        let m = g.legal_moves();
        g.apply(&m[0]);
        let m2 = g.legal_moves();
        for n in nbrs {
            assert!(!m2.contains(&Move::Settle(n as u8)));
        }
        assert!(!m2.contains(&Move::Settle(0)));
    }

    #[test]
    fn production_pays_settlement_and_city() {
        let mut g = setup_done(5);
        g.hands = [[0; 5]; 2];
        // alegem un vârf liber și un hexagon vecin cu număr
        let v = (0..54)
            .find(|&v| g.build[v].is_none() && g.dist_ok(v))
            .unwrap();
        g.build[v] = Some((Player::One, false));
        let h = *board().vh[v]
            .iter()
            .find(|&&h| g.res[h] != DESERT && h != g.robber)
            .unwrap();
        let n = g.num[h];
        let before: u32 = g.hand_total(Player::One);
        g.robber = if h == 0 { 1 } else { 0 };
        if g.robber == h {
            return;
        }
        g.produce(n);
        assert!(g.hand_total(Player::One) > before);
        g.build[v] = Some((Player::One, true));
        let t0 = g.hand_total(Player::One);
        g.produce(n);
        assert!(g.hand_total(Player::One) >= t0 + 2);
    }

    #[test]
    fn robber_blocks_and_steals() {
        let mut g = setup_done(6);
        g.hands = [[0; 5]; 2];
        g.hands[1] = [2, 0, 0, 0, 0];
        let v2 = (0..54)
            .find(|&v| g.build[v].map(|b| b.0) == Some(Player::Two))
            .unwrap();
        let h = board().vh[v2][0];
        g.phase = Phase::Robber;
        g.side = Player::One;
        if h == g.robber {
            return;
        }
        g.apply(&Move::Robber(h as u8));
        assert_eq!(g.hand_total(Player::One), 1);
        assert_eq!(g.hand_total(Player::Two), 1);
        assert_eq!(g.phase, Phase::Main);
        let before = g.hand_total(Player::Two);
        g.produce(g.num[h]);
        assert_eq!(g.hand_total(Player::Two), before); // tâlharul blochează
    }

    #[test]
    fn seven_discards_half() {
        let mut g = setup_done(7);
        g.hands[0] = [3, 3, 2, 0, 0];
        g.hands[1] = [1, 0, 0, 0, 0];
        // forțăm un 7 prin apel direct al logicii de aruncare
        for _ in 0..200 {
            g.rng.next_u64();
            let mut c = g.clone();
            c.apply(&Move::Roll);
            if c.phase == Phase::Robber {
                assert_eq!(c.hand_total(Player::One), 4);
                assert_eq!(c.hand_total(Player::Two), 1);
                return;
            }
        }
        panic!("n-am obținut 7");
    }

    #[test]
    fn building_costs_and_city_upgrade() {
        let mut g = setup_done(8);
        g.hands[0] = [4, 4, 4, 4, 4];
        g.phase = Phase::Main;
        let m = g.legal_moves();
        assert!(m.iter().any(|x| matches!(x, Move::City(_))));
        let city = m
            .iter()
            .find_map(|x| {
                if let Move::City(v) = x {
                    Some(*v)
                } else {
                    None
                }
            })
            .unwrap();
        let before = g.vp(Player::One);
        g.apply(&Move::City(city));
        assert_eq!(g.vp(Player::One), before + 1);
        assert_eq!(g.hands[0], [4, 4, 4, 2, 1]);
        // schimb 4:1
        g.hands[0] = [4, 0, 0, 0, 0];
        g.apply(&Move::Trade(0, 4));
        assert_eq!(g.hands[0], [0, 0, 0, 0, 1]);
    }

    #[test]
    fn longest_road_scores_two() {
        let mut g = setup_done(9);
        // construim manual un lanț de 5 muchii pentru J1 pornind dintr-un vârf
        let mut chain = Vec::new();
        let mut v = (0..54).find(|&v| board().vadj[v].len() == 2).unwrap();
        let mut used = std::collections::HashSet::new();
        'outer: while chain.len() < 5 {
            for &(n, e) in &board().vadj[v] {
                if !used.contains(&e) && g.road[e].is_none() && g.build[n].is_none() {
                    used.insert(e);
                    chain.push(e);
                    v = n;
                    continue 'outer;
                }
            }
            break;
        }
        for e in &chain {
            g.road[*e] = Some(Player::One);
        }
        g.update_longest();
        if chain.len() >= 5 {
            assert!(g.longest_road_len(Player::One) >= 5);
            assert_eq!(g.longest_holder(), Some(Player::One));
        }
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=10u64 {
            let mut g = Insula::with_seed(seed);
            let mut rng = Rng::new(seed ^ 0xabc);
            let mut plies = 0;
            while Game::outcome(&g).is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                plies += 1;
                assert!(plies < 4000);
            }
        }
    }

    #[test]
    fn render_hides_opponent_hand() {
        let g = setup_done(10);
        let r = Game::render(&g);
        assert!(r.contains("Mâna Jucătorului 1"));
        assert!(!r.contains("Mâna Jucătorului 2"));
    }

    #[test]
    fn ai_beats_random_mostly() {
        let (mut ai, mut rnd) = (0, 0);
        for seed in 1..=8u64 {
            let ai_side = if seed % 2 == 0 {
                Player::One
            } else {
                Player::Two
            };
            let mut g = Insula::with_seed(seed);
            let mut rng = Rng::new(seed * 5);
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
        assert!(ai > rnd, "AI {ai} vs random {rnd}");
    }
}

#[cfg(test)]
mod a2_audit {
    use super::*;

    #[test]
    fn ai_nu_vede_mana_adversarului_nici_zarurile_viitoare_la_orice_nivel() {
        crate::p3_comun::ai_blind_check(
            "insula",
            Insula::with_seed,
            |g, me, seed| {
                // mâna adversarului: aceleași cărți (numărul e public), alte resurse; zarurile viitoare: alt RNG
                let o = me.other().idx();
                let n: u8 = g.hands[o].iter().sum();
                let mut rng = Rng::new(seed);
                g.hands[o] = [0; 5];
                for _ in 0..n {
                    g.hands[o][rng.below(5)] += 1;
                }
                g.rng = Rng::new(seed ^ 0x1515);
            },
            24,
        );
    }
}
