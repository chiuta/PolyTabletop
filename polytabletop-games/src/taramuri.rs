//! Tărâmuri — joc original de transformare a terenului și de ritual pe patru
//! trepte (2 jucători).
//!
//! Mecanica „fiecare popor trăiește pe un tip de teren, transformi terenul
//! vecin în terenul tău pe o roată de tipuri, construiești și îmbunătățești
//! clădiri, avansezi pe piste de cult” e generică; harta, tipurile, numerele
//! și textele sunt ale noastre. Informație completă (seed-ul stabilește
//! harta și ordinea bonusurilor de rundă).
//!
//! Simplificări: hartă pătrată 7×6 cu vecinătate pe 4 direcții (nu hexagoane),
//! fără puterea/ciclul de mană, fără facțiuni diferite (doar teren natal
//! diferit), fără fortărețe, fără schimb de puncte pentru surplus.
//!
//! Reguli:
//! * 7 tipuri de teren pe o roată (0–6). J1 e de tip 0, J2 de tip 3. Un teren
//!   la distanță `d` pe roată de tipul tău costă `d` lopeți; o lopată costă 2
//!   muncitori (1 dacă ai Pământ ≥ 5).
//! * Început: pe rând J1, J2, J2, J1 pui câte o locuință pe un teren natal liber
//!   (`inicial c3`). Apoi 6 runde. Resurse de început: 3 muncitori, 5 monede,
//!   1 preot.
//! * Pe rând faci o singură acțiune sau `pas` (după pas nu mai joci în runda
//!   respectivă; primul care dă pas începe runda următoare):
//!   - `colonie c3`: pe o celulă fără clădire, vecină cu o clădire a ta (cu
//!     Aer ≥ 5 și la distanța 2), transformi terenul în terenul tău (lopeți) și
//!     pui o locuință (1 muncitor + 2 monede). Cel mult 8 locuințe.
//!   - `oras c3`: locuință → târg (2 muncitori + 3 monede; cel mult 4);
//!   - `templu c3`: târg → templu (4 muncitori + 6 monede; cel mult 2; +1 preot);
//!   - `preot foc|apa|pamant|aer`: un preot avansează 2 trepte pe pista aleasă
//!     (0–10); la 5 pista dă un privilegiu permanent: Foc +2 muncitori pe
//!     rundă, Apă +2 monede pe rundă, Pământ lopată mai ieftină, Aer rază 2.
//! * Venit la sfârșitul rundei: muncitori = 1 + locuințe (+2 Foc ≥ 5), monede =
//!   2 + 2 × târguri + 4 × temple (+2 Apă ≥ 5).
//! * Bonus de rundă (ordinea depinde de seed): +2 puncte pe locuință nouă, +3
//!   pe târg, +5 pe templu, +2 pe lopată folosită, +1 pe treaptă de cult, +1
//!   pe orice clădire.
//! * Final după runda 6: pe fiecare pistă primul +8, al doilea +4 (egal: 6 și
//!   6); cel mai mare grup de clădiri vecine +10 (egal: 5 și 5); 1 punct la 3
//!   monede.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const W: usize = 7;
pub const H: usize = 6;
const N: usize = W * H;
const ROUNDS: u8 = 6;
const TRACK: [&str; 4] = ["foc", "apa", "pamant", "aer"];
const HOME: [u8; 2] = [0, 3];
const BONUS_NAME: [&str; 6] = [
    "+2 pe locuință",
    "+3 pe târg",
    "+5 pe templu",
    "+2 pe lopată",
    "+1 pe treaptă de cult",
    "+1 pe clădire",
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Start(u8),
    Colony(u8),
    Town(u8),
    Temple(u8),
    Priest(u8),
    Pass,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Setup(u8),
    Play,
    Over,
}

const SETUP_ORDER: [Player; 4] = [Player::One, Player::Two, Player::Two, Player::One];

#[derive(Clone)]
pub struct Taramuri {
    terrain: [u8; N],
    /// 0 = nimic, 1 locuință, 2 târg, 3 templu
    build: [u8; N],
    owner: [Option<Player>; N],
    workers: [i32; 2],
    coins: [i32; 2],
    priests: [i32; 2],
    cult: [[i32; 4]; 2],
    vp: [i32; 2],
    bonus_order: [u8; 6],
    passed: [bool; 2],
    first_passer: Option<Player>,
    starter: Player,
    side: Player,
    phase: Phase,
    round: u8,
    outcome: Option<Outcome>,
    last: String,
}

impl Default for Taramuri {
    fn default() -> Self {
        Self::new()
    }
}

pub fn cell_name(c: usize) -> String {
    format!("{}{}", (b'a' + (c % W) as u8) as char, c / W + 1)
}

#[cfg(test)]
fn parse_cell(s: &str) -> Option<usize> {
    let b = s.as_bytes();
    if b.len() != 2 {
        return None;
    }
    let x = b[0].checked_sub(b'a')? as usize;
    let y = b[1].checked_sub(b'1')? as usize;
    if x < W && y < H {
        Some(y * W + x)
    } else {
        None
    }
}

fn ring_dist(a: u8, b: u8) -> i32 {
    let d = (a as i32 - b as i32).rem_euclid(7);
    d.min(7 - d)
}

fn neighbours(c: usize) -> Vec<usize> {
    let (x, y) = (c % W, c / W);
    let mut v = Vec::new();
    if x > 0 {
        v.push(c - 1);
    }
    if x + 1 < W {
        v.push(c + 1);
    }
    if y > 0 {
        v.push(c - W);
    }
    if y + 1 < H {
        v.push(c + W);
    }
    v
}

fn manhattan(a: usize, b: usize) -> usize {
    (a % W).abs_diff(b % W) + (a / W).abs_diff(b / W)
}

impl Taramuri {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut terr: Vec<u8> = (0..N).map(|i| (i % 7) as u8).collect();
        for i in (1..N).rev() {
            let j = rng.below(i + 1);
            terr.swap(i, j);
        }
        let mut terrain = [0u8; N];
        terrain.copy_from_slice(&terr);
        let mut bo: Vec<u8> = (0..6).collect();
        for i in (1..bo.len()).rev() {
            let j = rng.below(i + 1);
            bo.swap(i, j);
        }
        let mut bonus_order = [0u8; 6];
        bonus_order.copy_from_slice(&bo);
        Taramuri {
            terrain,
            build: [0; N],
            owner: [None; N],
            workers: [3, 3],
            coins: [5, 5],
            priests: [1, 1],
            cult: [[0; 4]; 2],
            vp: [0; 2],
            bonus_order,
            passed: [false; 2],
            first_passer: None,
            starter: Player::One,
            side: Player::One,
            phase: Phase::Setup(0),
            round: 1,
            outcome: None,
            last: String::new(),
        }
    }

    pub fn terrain_at(&self, c: usize) -> u8 {
        self.terrain[c]
    }

    pub fn score(&self, p: Player) -> i32 {
        self.vp[p.idx()]
    }

    fn count(&self, p: Player, kind: u8) -> i32 {
        (0..N)
            .filter(|&c| self.owner[c] == Some(p) && self.build[c] == kind)
            .count() as i32
    }

    fn spade_cost(&self, p: Player) -> i32 {
        if self.cult[p.idx()][2] >= 5 {
            1
        } else {
            2
        }
    }

    fn spades(&self, p: Player, c: usize) -> i32 {
        ring_dist(self.terrain[c], HOME[p.idx()])
    }

    fn reach(&self, p: Player, c: usize) -> bool {
        let wide = self.cult[p.idx()][3] >= 5;
        (0..N).any(|o| {
            self.owner[o] == Some(p)
                && ((!wide && neighbours(c).contains(&o)) || (wide && manhattan(c, o) <= 2))
        })
    }

    fn bonus(&self) -> u8 {
        self.bonus_order[(self.round as usize - 1).min(5)]
    }

    fn largest_group(&self, p: Player) -> i32 {
        let mut seen = [false; N];
        let mut best = 0;
        for s in 0..N {
            if seen[s] || self.owner[s] != Some(p) {
                continue;
            }
            let mut stack = vec![s];
            seen[s] = true;
            let mut n = 0;
            while let Some(c) = stack.pop() {
                n += 1;
                for nb in neighbours(c) {
                    if !seen[nb] && self.owner[nb] == Some(p) {
                        seen[nb] = true;
                        stack.push(nb);
                    }
                }
            }
            best = best.max(n);
        }
        best
    }

    fn end_round(&mut self) {
        for p in [Player::One, Player::Two] {
            let i = p.idx();
            self.workers[i] += 1 + self.count(p, 1) + if self.cult[i][0] >= 5 { 2 } else { 0 };
            self.coins[i] += 2
                + 2 * self.count(p, 2)
                + 4 * self.count(p, 3)
                + if self.cult[i][1] >= 5 { 2 } else { 0 };
        }
        if self.round >= ROUNDS {
            self.finish();
            return;
        }
        self.round += 1;
        self.starter = self.first_passer.unwrap_or(self.starter);
        self.first_passer = None;
        self.passed = [false; 2];
        self.side = self.starter;
    }

    fn finish(&mut self) {
        for t in 0..4 {
            let (a, b) = (self.cult[0][t], self.cult[1][t]);
            if a > b {
                self.vp[0] += 8;
                self.vp[1] += 4;
            } else if b > a {
                self.vp[1] += 8;
                self.vp[0] += 4;
            } else if a > 0 {
                self.vp[0] += 6;
                self.vp[1] += 6;
            }
        }
        let (ga, gb) = (
            self.largest_group(Player::One),
            self.largest_group(Player::Two),
        );
        if ga > gb {
            self.vp[0] += 10;
        } else if gb > ga {
            self.vp[1] += 10;
        } else {
            self.vp[0] += 5;
            self.vp[1] += 5;
        }
        for i in 0..2 {
            self.vp[i] += self.coins[i] / 3;
        }
        self.phase = Phase::Over;
        self.outcome = Some(if self.vp[0] > self.vp[1] {
            Outcome::Win(Player::One)
        } else if self.vp[1] > self.vp[0] {
            Outcome::Win(Player::Two)
        } else {
            Outcome::Draw
        });
    }

    fn next_turn(&mut self, p: Player) {
        let o = p.other();
        if self.passed[o.idx()] {
            self.side = p;
        } else {
            self.side = o;
        }
    }

    fn colony_cost(&self, p: Player, c: usize) -> (i32, i32) {
        (1 + self.spades(p, c) * self.spade_cost(p), 2)
    }
}

impl Game for Taramuri {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        let p = self.side;
        let i = p.idx();
        match self.phase {
            Phase::Over => Vec::new(),
            Phase::Setup(_) => (0..N)
                .filter(|&c| self.terrain[c] == HOME[i] && self.build[c] == 0)
                .map(|c| Move::Start(c as u8))
                .collect(),
            Phase::Play => {
                let mut v = Vec::new();
                if self.count(p, 1) < 8 {
                    for c in 0..N {
                        if self.build[c] != 0 || !self.reach(p, c) {
                            continue;
                        }
                        let (w, m) = self.colony_cost(p, c);
                        if self.workers[i] >= w && self.coins[i] >= m {
                            v.push(Move::Colony(c as u8));
                        }
                    }
                }
                for c in 0..N {
                    if self.owner[c] != Some(p) {
                        continue;
                    }
                    if self.build[c] == 1
                        && self.count(p, 2) < 4
                        && self.workers[i] >= 2
                        && self.coins[i] >= 3
                    {
                        v.push(Move::Town(c as u8));
                    }
                    if self.build[c] == 2
                        && self.count(p, 3) < 2
                        && self.workers[i] >= 4
                        && self.coins[i] >= 6
                    {
                        v.push(Move::Temple(c as u8));
                    }
                }
                if self.priests[i] > 0 {
                    for t in 0..4u8 {
                        if self.cult[i][t as usize] < 10 {
                            v.push(Move::Priest(t));
                        }
                    }
                }
                v.push(Move::Pass);
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        let p = self.side;
        let i = p.idx();
        let bonus = self.bonus();
        match *m {
            Move::Start(c) => {
                let c = c as usize;
                self.build[c] = 1;
                self.owner[c] = Some(p);
                self.last = format!("J{} pune o locuință pe {}", i + 1, cell_name(c));
                if let Phase::Setup(k) = self.phase {
                    if k >= 3 {
                        self.phase = Phase::Play;
                        self.side = self.starter;
                    } else {
                        self.phase = Phase::Setup(k + 1);
                        self.side = SETUP_ORDER[k as usize + 1];
                    }
                }
            }
            Move::Colony(c) => {
                let c = c as usize;
                let spades = self.spades(p, c);
                let (w, mo) = self.colony_cost(p, c);
                self.workers[i] -= w;
                self.coins[i] -= mo;
                self.terrain[c] = HOME[i];
                self.build[c] = 1;
                self.owner[c] = Some(p);
                match bonus {
                    0 => self.vp[i] += 2,
                    3 => self.vp[i] += 2 * spades,
                    5 => self.vp[i] += 1,
                    _ => {}
                }
                self.last = format!(
                    "J{} colonizează {} ({} lopeți)",
                    i + 1,
                    cell_name(c),
                    spades
                );
                self.next_turn(p);
            }
            Move::Town(c) => {
                self.workers[i] -= 2;
                self.coins[i] -= 3;
                self.build[c as usize] = 2;
                match bonus {
                    1 => self.vp[i] += 3,
                    5 => self.vp[i] += 1,
                    _ => {}
                }
                self.last = format!("J{} ridică un târg pe {}", i + 1, cell_name(c as usize));
                self.next_turn(p);
            }
            Move::Temple(c) => {
                self.workers[i] -= 4;
                self.coins[i] -= 6;
                self.build[c as usize] = 3;
                self.priests[i] += 1;
                match bonus {
                    2 => self.vp[i] += 5,
                    5 => self.vp[i] += 1,
                    _ => {}
                }
                self.last = format!("J{} ridică un templu pe {}", i + 1, cell_name(c as usize));
                self.next_turn(p);
            }
            Move::Priest(t) => {
                let t = t as usize;
                let before = self.cult[i][t];
                self.cult[i][t] = (before + 2).min(10);
                let steps = self.cult[i][t] - before;
                self.priests[i] -= 1;
                if bonus == 4 {
                    self.vp[i] += steps;
                }
                self.last = format!("J{} trimite un preot pe {}", i + 1, TRACK[t]);
                self.next_turn(p);
            }
            Move::Pass => {
                self.passed[i] = true;
                if self.first_passer.is_none() {
                    self.first_passer = Some(p);
                }
                self.last = format!("J{} dă pas", i + 1);
                if self.passed[0] && self.passed[1] {
                    self.end_round();
                } else {
                    self.side = p.other();
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.outcome
    }

    fn evaluate(&self, p: Player) -> i32 {
        let side = |q: Player| -> i32 {
            let i = q.idx();
            let mut v = self.vp[i] * 100;
            v += self.workers[i] * 14 + self.coins[i] * 9 + self.priests[i] * 70;
            v += self.count(q, 1) * 55 + self.count(q, 2) * 130 + self.count(q, 3) * 220;
            v += self.largest_group(q) * 18;
            for t in 0..4 {
                let c = self.cult[i][t];
                v += c * 22 + if c >= 5 { 80 } else { 0 };
            }
            v
        };
        let mut e = side(p) - side(p.other());
        // pistele: avans față de adversar (majoritate la final)
        for t in 0..4 {
            let d = self.cult[p.idx()][t] - self.cult[p.other().idx()][t];
            e += d.signum() * 60;
        }
        e
    }

    fn render(&self) -> String {
        let sym = ['0', '1', '2', '3', '4', '5', '6'];
        let mut s = format!(
            "Tărâmuri — runda {}/{} | bonus de rundă: {}\n",
            self.round,
            ROUNDS,
            BONUS_NAME[self.bonus() as usize]
        );
        s.push_str("    ");
        for x in 0..W {
            s.push_str(&format!("{}    ", (b'a' + x as u8) as char));
        }
        s.push('\n');
        for y in 0..H {
            s.push_str(&format!("{}  ", y + 1));
            for x in 0..W {
                let c = y * W + x;
                let b = match (self.owner[c], self.build[c]) {
                    (Some(p), k) => {
                        let ch = ['.', 'L', 'T', 'S'][k as usize];
                        format!("{}{}", ch, p.idx() + 1)
                    }
                    _ => "  ".into(),
                };
                s.push_str(&format!(" {}{} ", sym[self.terrain[c] as usize], b));
            }
            s.push('\n');
        }
        s.push_str(
            "(teren 0–6 pe roată; L locuință, T târg, S templu, cifra după literă = jucătorul)\n",
        );
        for q in [Player::One, Player::Two] {
            let i = q.idx();
            s.push_str(&format!(
                "J{} (teren natal {}): {} puncte | muncitori {} monede {} preoți {} | cult foc {} apă {} pământ {} aer {}{}\n",
                i + 1,
                HOME[i],
                self.vp[i],
                self.workers[i],
                self.coins[i],
                self.priests[i],
                self.cult[i][0],
                self.cult[i][1],
                self.cult[i][2],
                self.cult[i][3],
                if self.passed[i] { " (pas)" } else { "" }
            ));
        }
        if !self.last.is_empty() {
            s.push_str(&format!("Ultima mutare: {}\n", self.last));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match *m {
            Move::Start(c) => format!("inicial {}", cell_name(c as usize)),
            Move::Colony(c) => format!("colonie {}", cell_name(c as usize)),
            Move::Town(c) => format!("oras {}", cell_name(c as usize)),
            Move::Temple(c) => format!("templu {}", cell_name(c as usize)),
            Move::Priest(t) => format!("preot {}", TRACK[t as usize]),
            Move::Pass => "pas".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(6)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mv(g: &mut Taramuri, s: &str) {
        let m = g
            .legal_moves()
            .into_iter()
            .find(|m| g.format_move(m) == s)
            .unwrap_or_else(|| panic!("ilegal: {s}\n{}", g.render()));
        g.apply(&m);
    }

    fn started(seed: u64) -> Taramuri {
        let mut g = Taramuri::with_seed(seed);
        let mut order = Vec::new();
        for _ in 0..4 {
            order.push(g.to_move());
            let m = g.legal_moves()[0];
            g.apply(&m);
        }
        assert_eq!(
            order,
            vec![Player::One, Player::Two, Player::Two, Player::One]
        );
        assert_eq!(g.phase, Phase::Play);
        g
    }

    #[test]
    fn terrain_is_balanced() {
        let g = Taramuri::with_seed(1);
        for t in 0..7u8 {
            assert_eq!(g.terrain.iter().filter(|&&x| x == t).count(), 6);
        }
        let mut b = g.bonus_order.to_vec();
        b.sort();
        assert_eq!(b, vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn ring_distance() {
        assert_eq!(ring_dist(0, 3), 3);
        assert_eq!(ring_dist(0, 4), 3);
        assert_eq!(ring_dist(0, 6), 1);
        assert_eq!(ring_dist(5, 5), 0);
        assert_eq!(parse_cell("c4"), Some(3 * W + 2));
        assert_eq!(cell_name(3 * W + 2), "c4");
    }

    #[test]
    fn setup_places_home_dwellings() {
        let g = started(2);
        assert_eq!(g.count(Player::One, 1), 2);
        assert_eq!(g.count(Player::Two, 1), 2);
        for c in 0..N {
            if let Some(p) = g.owner[c] {
                assert_eq!(g.terrain[c], HOME[p.idx()]);
            }
        }
    }

    #[test]
    fn colony_costs_spades() {
        let mut g = started(3);
        g.side = Player::One;
        g.workers = [10, 10];
        g.coins = [10, 10];
        // găsește o celulă vecină cu teren la distanță 2 de terenul natal 0
        let mut found = None;
        for c in 0..N {
            if g.build[c] == 0 && g.reach(Player::One, c) {
                found = Some(c);
                break;
            }
        }
        let c = found.expect("există o celulă la rază");
        g.terrain[c] = 2;
        let name = cell_name(c);
        let m = g.legal_moves();
        assert!(m.contains(&Move::Colony(c as u8)));
        mv(&mut g, &format!("colonie {name}"));
        assert_eq!(g.workers[0], 10 - 1 - 2 * 2);
        assert_eq!(g.coins[0], 8);
        assert_eq!(g.terrain[c], 0);
        assert_eq!(g.owner[c], Some(Player::One));
        // reducere Pământ ≥ 5
        let mut h = started(3);
        h.side = Player::One;
        h.workers = [10, 10];
        h.coins = [10, 10];
        h.cult[0][2] = 5;
        let c2 = (0..N)
            .find(|&c| h.build[c] == 0 && h.reach(Player::One, c))
            .unwrap();
        h.terrain[c2] = 2;
        let n2 = cell_name(c2);
        mv(&mut h, &format!("colonie {n2}"));
        assert_eq!(h.workers[0], 10 - 1 - 2);
    }

    #[test]
    fn upgrades_and_priests() {
        let mut g = started(4);
        g.side = Player::One;
        g.workers = [10, 10];
        g.coins = [20, 20];
        let c = (0..N).find(|&c| g.owner[c] == Some(Player::One)).unwrap();
        let n = cell_name(c);
        mv(&mut g, &format!("oras {n}"));
        assert_eq!(g.build[c], 2);
        g.side = Player::One;
        mv(&mut g, &format!("templu {n}"));
        assert_eq!(g.build[c], 3);
        assert_eq!(g.priests[0], 2);
        g.side = Player::One;
        mv(&mut g, "preot foc");
        assert_eq!(g.cult[0][0], 2);
        assert_eq!(g.priests[0], 1);
        // nu mai poți urca peste 10
        g.cult[0][0] = 9;
        g.side = Player::One;
        mv(&mut g, "preot foc");
        assert_eq!(g.cult[0][0], 10);
    }

    #[test]
    fn round_cycle_and_income() {
        let mut g = started(5);
        g.workers = [0, 0];
        g.coins = [0, 0];
        let start = g.to_move();
        mv(&mut g, "pas");
        assert_eq!(g.first_passer, Some(start));
        mv(&mut g, "pas");
        assert_eq!(g.round, 2);
        assert_eq!(g.workers, [3, 3]); // 1 + 2 locuințe
        assert_eq!(g.coins, [2, 2]);
        assert_eq!(g.to_move(), start);
    }

    #[test]
    fn final_scoring_cult_and_area() {
        let mut g = started(6);
        g.cult[0] = [10, 4, 0, 0];
        g.cult[1] = [2, 4, 0, 0];
        g.vp = [0, 0];
        g.coins = [0, 0];
        g.finish();
        // foc 8/4, apă 6/6; grupurile se compară pe harta de început
        assert!(g.vp[0] >= 14 && g.vp[1] >= 10);
        assert!(g.outcome.is_some());
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=30u64 {
            let mut g = Taramuri::with_seed(seed);
            let mut rng = Rng::new(seed ^ 0x31);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty(), "{}", g.render());
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 3000);
                for i in 0..2 {
                    assert!(g.workers[i] >= 0 && g.coins[i] >= 0 && g.priests[i] >= 0);
                    assert!(g.cult[i].iter().all(|&x| (0..=10).contains(&x)));
                }
            }
        }
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
            let mut g = Taramuri::with_seed(seed);
            let mut rng = Rng::new(seed * 3);
            while g.outcome().is_none() {
                let m = if g.to_move() == ai_side {
                    polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap()
                } else {
                    let l = g.legal_moves();
                    l[rng.below(l.len())]
                };
                g.apply(&m);
            }
            match g.outcome() {
                Some(Outcome::Win(p)) if p == ai_side => ai += 1,
                Some(Outcome::Win(_)) => rnd += 1,
                _ => {}
            }
        }
        assert!(ai > rnd, "AI {ai} vs random {rnd}");
    }
}
