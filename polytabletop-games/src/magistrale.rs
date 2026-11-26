//! Magistrale — joc original de conectat orașe cu cărți de vagon (2 jucători).
//!
//! Harta (19 orașe, 38 de rute), pachetul, biletele de destinație și textele
//! sunt ale noastre; mecanica „aduni cărți de aceeași culoare și ocupi rute” e
//! generică.
//!
//! Reguli:
//! * Cărți de vagon în 5 culori (r roșu, a albastru, v verde, g galben, m mov),
//!   câte 12, plus 10 locomotive `j` (joker). 5 cărți vizibile; dacă trei sunt
//!   locomotive, se înlocuiesc toate cinci.
//! * 40 de vagoane fiecare. Pornești cu 4 cărți și 3 bilete de destinație din
//!   care păstrezi cel puțin 2 (`pastrez 1,3`). Biletele sunt secrete.
//! * La rând alegi exact una dintre:
//!   - tragi 2 cărți: `trage` (din pachet) sau `trage r` (vizibilă); o
//!     locomotivă vizibilă (`trage j`) se ia singură, ca întreaga mutare;
//!   - `ocupa 12 r`: ocupi ruta 12 cu cărți din culoarea r (ruta cere culoarea
//!     ei; rutele gri merg cu orice culoare), locomotivele țin loc de orice;
//!     scorul rutei: 1, 2, 4, 7, 10, 15 puncte pentru lungimea 1–6;
//!   - `bilete`: tragi 3 bilete și păstrezi cel puțin unul.
//! * În doi, la rutele duble doar una dintre ele poate fi ocupată.
//! * Când cuiva îi rămân cel mult 2 vagoane, adversarul mai joacă o ultimă
//!   mutare. La final: bilet îndeplinit +valoarea, neîndeplinit −valoarea;
//!   cel mai lung drum continuu +10 (la egalitate, ambii).
//! * Valoarea unui bilet = lungimea celui mai scurt drum dintre orașe.
//! * Dacă nu ai nicio altă mutare, `pas`. Limită de siguranță: 1200 de mutări.
//!
//! Informație ascunsă: `render` arată doar cărțile și biletele Jucătorului 1.
//! AI-ul nu se uită la mâna adversarului sau la ordinea pachetelor; la
//! evaluare ignoră cărțile abia trase „pe orb”.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const CITIES: [&str; 19] = [
    "Timișoara",
    "Arad",
    "Oradea",
    "Baia Mare",
    "Cluj",
    "Deva",
    "Sibiu",
    "Târgu Mureș",
    "Suceava",
    "Iași",
    "Bacău",
    "Brașov",
    "Ploiești",
    "București",
    "Pitești",
    "Craiova",
    "Constanța",
    "Galați",
    "Râmnicu Vâlcea",
];

const COLOR_LETTERS: [char; 6] = ['r', 'a', 'v', 'g', 'm', 'j'];
const GRAY: u8 = 5;
const WILD: usize = 5;

/// (oraș A, oraș B, lungime, culoare 0..=4 sau 5 = gri)
pub const ROUTES: [(u8, u8, u8, u8); 38] = [
    (0, 1, 1, 5),
    (1, 2, 3, 5),
    (1, 2, 3, 1),
    (1, 5, 3, 0),
    (0, 5, 3, 1),
    (0, 15, 5, 2),
    (2, 4, 2, 0),
    (2, 4, 2, 1),
    (2, 3, 3, 3),
    (3, 4, 2, 2),
    (3, 8, 5, 4),
    (4, 5, 3, 3),
    (4, 6, 3, 1),
    (4, 7, 2, 5),
    (5, 6, 2, 0),
    (5, 15, 4, 4),
    (5, 18, 4, 2),
    (6, 7, 2, 2),
    (6, 11, 2, 3),
    (6, 18, 2, 5),
    (7, 8, 4, 1),
    (7, 11, 3, 0),
    (7, 10, 4, 2),
    (8, 9, 3, 5),
    (8, 10, 2, 3),
    (9, 10, 2, 0),
    (9, 17, 3, 2),
    (10, 17, 3, 1),
    (10, 11, 4, 5),
    (11, 12, 2, 4),
    (11, 14, 3, 3),
    (12, 13, 1, 5),
    (12, 13, 1, 5),
    (12, 17, 4, 0),
    (13, 17, 4, 3),
    (13, 16, 3, 1),
    (16, 17, 4, 4),
    (13, 14, 3, 2),
];
// Rutele următoare completează harta (separat ca să rămână tabloul de sus lizibil).
const ROUTES_EXTRA: [(u8, u8, u8, u8); 4] = [
    (13, 15, 4, 0),
    (14, 15, 3, 3),
    (14, 18, 2, 1),
    (15, 18, 3, 4),
];

const TICKET_PAIRS: [(u8, u8); 20] = [
    (0, 13),
    (2, 16),
    (3, 13),
    (8, 13),
    (0, 17),
    (9, 15),
    (4, 12),
    (1, 16),
    (3, 16),
    (2, 9),
    (0, 8),
    (6, 17),
    (5, 16),
    (4, 17),
    (7, 15),
    (2, 13),
    (9, 14),
    (8, 15),
    (1, 10),
    (3, 15),
];

const ROUTE_POINTS: [i32; 7] = [0, 1, 2, 4, 7, 10, 15];
const START_TRAINS: i32 = 40;
const PLY_CAP: u32 = 1200;
const INF: i32 = 10_000;

fn route(i: usize) -> (u8, u8, u8, u8) {
    if i < ROUTES.len() {
        ROUTES[i]
    } else {
        ROUTES_EXTRA[i - ROUTES.len()]
    }
}

pub fn route_count() -> usize {
    ROUTES.len() + ROUTES_EXTRA.len()
}

fn floyd(blocked: &dyn Fn(usize) -> Option<i32>) -> Vec<Vec<i32>> {
    let n = CITIES.len();
    let mut d = vec![vec![INF; n]; n];
    for (i, row) in d.iter_mut().enumerate() {
        row[i] = 0;
    }
    for i in 0..route_count() {
        let (a, b, _, _) = route(i);
        if let Some(w) = blocked(i) {
            let (a, b) = (a as usize, b as usize);
            if w < d[a][b] {
                d[a][b] = w;
                d[b][a] = w;
            }
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let v = d[i][k] + d[k][j];
                if v < d[i][j] {
                    d[i][j] = v;
                }
            }
        }
    }
    d
}

/// Valoarea biletului `t` = cel mai scurt drum pe harta goală.
pub fn ticket_value(t: usize) -> i32 {
    let (a, b) = TICKET_PAIRS[t];
    floyd(&|i| Some(route(i).2 as i32))[a as usize][b as usize]
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    /// Trage din pachet.
    Draw,
    /// Trage o carte vizibilă (culoarea 0..=5, 5 = locomotivă).
    Face(u8),
    /// Ocupă ruta cu culoarea de cărți (0..=5; 5 = numai locomotive).
    Claim(u8, u8),
    DrawTickets,
    /// Păstrează biletele din ofertă indicate de mască (bit i = biletul i).
    Keep(u8),
    Pass,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Keep { min: u8, setup: bool },
    Turn,
    Second,
}

#[derive(Clone)]
pub struct Magistrale {
    /// Whose private info `render` shows (`None` = the mover's, hot-seat).
    viewer: Option<Player>,
    deck: Vec<u8>,
    discard: Vec<u8>,
    face: Vec<u8>,
    hands: [[u8; 6]; 2],
    trains: [i32; 2],
    score: [i32; 2],
    owner: Vec<Option<Player>>,
    ticket_deck: Vec<usize>,
    tickets: [Vec<usize>; 2],
    offer: Vec<usize>,
    side: Player,
    phase: Phase,
    end_after: Option<Player>,
    /// cărți trase pe orb în ultima mutare (pentru AI)
    blind: [[u8; 6]; 2],
    plies: u32,
    result: Option<Outcome>,
    final_scores: Option<[i32; 2]>,
    rng: Rng,
}

impl Default for Magistrale {
    fn default() -> Self {
        Self::new()
    }
}

fn shuffle<T>(v: &mut [T], rng: &mut Rng) {
    for i in (1..v.len()).rev() {
        let j = rng.below(i + 1);
        v.swap(i, j);
    }
}

impl Magistrale {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck: Vec<u8> = Vec::new();
        for c in 0..5u8 {
            deck.extend(std::iter::repeat_n(c, 12));
        }
        deck.extend(std::iter::repeat_n(5u8, 10));
        shuffle(&mut deck, &mut rng);
        let mut tdeck: Vec<usize> = (0..TICKET_PAIRS.len()).collect();
        shuffle(&mut tdeck, &mut rng);
        let mut g = Magistrale {
            viewer: None,
            deck,
            discard: Vec::new(),
            face: Vec::new(),
            hands: [[0; 6]; 2],
            trains: [START_TRAINS; 2],
            score: [0; 2],
            owner: vec![None; route_count()],
            ticket_deck: tdeck,
            tickets: [Vec::new(), Vec::new()],
            offer: Vec::new(),
            side: Player::One,
            phase: Phase::Keep {
                min: 2,
                setup: true,
            },
            end_after: None,
            blind: [[0; 6]; 2],
            plies: 0,
            result: None,
            final_scores: None,
            rng,
        };
        for p in 0..2 {
            for _ in 0..4 {
                let c = g.take().expect("pachet");
                g.hands[p][c as usize] += 1;
            }
        }
        for _ in 0..5 {
            let c = g.take().expect("pachet");
            g.face.push(c);
        }
        g.fix_face();
        g.deal_offer(3);
        g
    }

    fn take(&mut self) -> Option<u8> {
        if self.deck.is_empty() && !self.discard.is_empty() {
            let mut d = std::mem::take(&mut self.discard);
            shuffle(&mut d, &mut self.rng);
            self.deck = d;
        }
        self.deck.pop()
    }

    /// Dacă 3+ locomotive sunt vizibile, înlocuiește cele cinci cărți.
    fn fix_face(&mut self) {
        for _ in 0..10 {
            if self.face.iter().filter(|&&c| c == 5).count() < 3 {
                return;
            }
            let old = std::mem::take(&mut self.face);
            self.discard.extend(old);
            for _ in 0..5 {
                if let Some(c) = self.take() {
                    self.face.push(c);
                }
            }
        }
    }

    fn refill_face(&mut self) {
        while self.face.len() < 5 {
            match self.take() {
                Some(c) => self.face.push(c),
                None => break,
            }
        }
        self.fix_face();
    }

    fn deal_offer(&mut self, n: usize) {
        self.offer.clear();
        for _ in 0..n {
            if let Some(t) = self.ticket_deck.pop() {
                self.offer.push(t);
            }
        }
    }

    pub fn trains(&self, p: Player) -> i32 {
        self.trains[p.idx()]
    }

    pub fn score(&self, p: Player) -> i32 {
        self.score[p.idx()]
    }

    pub fn owner_of(&self, route: usize) -> Option<Player> {
        self.owner[route]
    }

    pub fn hand_total(&self, p: Player) -> u32 {
        self.hands[p.idx()].iter().map(|&x| x as u32).sum()
    }

    /// Toate cărțile (pachet, aruncate, vizibile, mâini) — trebuie să fie 70.
    pub fn card_total(&self) -> usize {
        self.deck.len()
            + self.discard.len()
            + self.face.len()
            + self
                .hands
                .iter()
                .flatten()
                .map(|&x| x as usize)
                .sum::<usize>()
    }

    /// Biletele (deck + ofertă + mâini) — trebuie să fie 20.
    pub fn ticket_total(&self) -> usize {
        self.ticket_deck.len() + self.offer.len() + self.tickets[0].len() + self.tickets[1].len()
    }

    fn twin_taken(&self, i: usize) -> bool {
        let (a, b, _, _) = route(i);
        (0..route_count()).any(|j| {
            j != i && self.owner[j].is_some() && {
                let (c, d, _, _) = route(j);
                (a, b) == (c, d)
            }
        })
    }

    /// Cărțile folosite pentru a ocupa ruta `i` cu culoarea `c`, dacă se poate.
    fn payment(&self, p: Player, i: usize, c: u8) -> Option<[u8; 6]> {
        let (_, _, len, col) = route(i);
        let h = &self.hands[p.idx()];
        let mut pay = [0u8; 6];
        if c == 5 {
            if h[WILD] >= len {
                pay[WILD] = len;
                return Some(pay);
            }
            return None;
        }
        if col != GRAY && col != c {
            return None;
        }
        let k = h[c as usize].min(len);
        let w = len - k;
        if h[WILD] < w || k == 0 {
            return None;
        }
        pay[c as usize] = k;
        pay[WILD] = w;
        Some(pay)
    }

    fn can_claim(&self, p: Player, i: usize) -> bool {
        self.owner[i].is_none() && !self.twin_taken(i) && self.trains[p.idx()] >= route(i).2 as i32
    }

    fn finish_turn(&mut self) {
        self.phase = Phase::Turn;
        let me = self.side;
        if let Some(last) = self.end_after {
            if last == me {
                self.finish_game();
                return;
            }
        } else if self.trains[me.idx()] <= 2 {
            self.end_after = Some(me.other());
        }
        self.side = me.other();
    }

    fn longest(&self, p: Player) -> i32 {
        let edges: Vec<(usize, usize, i32)> = (0..route_count())
            .filter(|&i| self.owner[i] == Some(p))
            .map(|i| {
                let (a, b, l, _) = route(i);
                (a as usize, b as usize, l as i32)
            })
            .collect();
        fn dfs(node: usize, edges: &[(usize, usize, i32)], used: &mut Vec<bool>) -> i32 {
            let mut best = 0;
            for k in 0..edges.len() {
                if used[k] {
                    continue;
                }
                let (a, b, l) = edges[k];
                let next = if a == node {
                    b
                } else if b == node {
                    a
                } else {
                    continue;
                };
                used[k] = true;
                best = best.max(l + dfs(next, edges, used));
                used[k] = false;
            }
            best
        }
        let mut best = 0;
        let mut used = vec![false; edges.len()];
        for start in 0..CITIES.len() {
            best = best.max(dfs(start, &edges, &mut used));
        }
        best
    }

    fn ticket_done(&self, p: Player, t: usize) -> bool {
        let (a, b) = TICKET_PAIRS[t];
        let d = floyd(&|i| {
            if self.owner[i] == Some(p) {
                Some(0)
            } else {
                None
            }
        });
        d[a as usize][b as usize] == 0
    }

    fn finish_game(&mut self) {
        let mut sc = self.score;
        for p in [Player::One, Player::Two] {
            for &t in &self.tickets[p.idx()] {
                let v = ticket_value(t);
                if self.ticket_done(p, t) {
                    sc[p.idx()] += v;
                } else {
                    sc[p.idx()] -= v;
                }
            }
        }
        let l0 = self.longest(Player::One);
        let l1 = self.longest(Player::Two);
        if l0 >= l1 {
            sc[0] += 10;
        }
        if l1 >= l0 {
            sc[1] += 10;
        }
        self.final_scores = Some(sc);
        self.result = Some(match sc[0].cmp(&sc[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        });
    }

    pub fn final_scores(&self) -> Option<[i32; 2]> {
        self.final_scores
    }

    fn est_tickets(&self, p: Player) -> i32 {
        let q = p.other();
        let d = floyd(&|i| match self.owner[i] {
            Some(o) if o == p => Some(0),
            Some(_) => None,
            None => {
                if self.twin_taken(i) {
                    None
                } else {
                    let _ = q;
                    Some(route(i).2 as i32)
                }
            }
        });
        let mut s = 0;
        for &t in &self.tickets[p.idx()] {
            let (a, b) = TICKET_PAIRS[t];
            let v = ticket_value(t);
            let rem = d[a as usize][b as usize];
            if rem >= INF {
                s -= 10 * v;
            } else {
                s += 10 * v * (v - 2 * rem.min(v)) / v.max(1);
            }
        }
        s
    }
}

impl Game for Magistrale {
    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.result.is_some() {
            return Vec::new();
        }
        let p = self.side;
        let mut v = Vec::new();
        match self.phase {
            Phase::Keep { min, .. } => {
                let n = self.offer.len();
                for mask in 1u8..(1 << n) {
                    if mask.count_ones() as u8 >= min.min(n as u8) {
                        v.push(Move::Keep(mask));
                    }
                }
            }
            Phase::Turn | Phase::Second => {
                if !self.deck.is_empty() || !self.discard.is_empty() {
                    v.push(Move::Draw);
                }
                let mut seen = [false; 6];
                for &c in &self.face {
                    let c = c as usize;
                    if seen[c] {
                        continue;
                    }
                    if c == WILD && self.phase == Phase::Second {
                        continue;
                    }
                    seen[c] = true;
                    v.push(Move::Face(c as u8));
                }
                if self.phase == Phase::Turn {
                    for i in 0..route_count() {
                        if !self.can_claim(p, i) {
                            continue;
                        }
                        for c in 0..6u8 {
                            if self.payment(p, i, c).is_some() {
                                v.push(Move::Claim(i as u8, c));
                            }
                        }
                    }
                    if !self.ticket_deck.is_empty() {
                        v.push(Move::DrawTickets);
                    }
                }
                if v.is_empty() {
                    v.push(Move::Pass);
                }
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        self.plies += 1;
        self.blind = [[0; 6]; 2];
        let me = self.side;
        match *m {
            Move::Keep(mask) => {
                let offer = std::mem::take(&mut self.offer);
                let mut back = Vec::new();
                for (i, &t) in offer.iter().enumerate() {
                    if mask & (1 << i) != 0 {
                        self.tickets[me.idx()].push(t);
                    } else {
                        back.push(t);
                    }
                }
                // cele refuzate merg la baza teancului
                for t in back {
                    self.ticket_deck.insert(0, t);
                }
                let setup = matches!(self.phase, Phase::Keep { setup: true, .. });
                if setup && me == Player::One {
                    self.side = Player::Two;
                    self.deal_offer(3);
                    self.phase = Phase::Keep {
                        min: 2,
                        setup: true,
                    };
                } else if setup {
                    self.side = Player::One;
                    self.phase = Phase::Turn;
                } else {
                    self.finish_turn();
                }
            }
            Move::DrawTickets => {
                self.deal_offer(3);
                self.phase = Phase::Keep {
                    min: 1,
                    setup: false,
                };
            }
            Move::Draw => {
                if let Some(c) = self.take() {
                    self.hands[me.idx()][c as usize] += 1;
                    self.blind[me.idx()][c as usize] += 1;
                }
                if self.phase == Phase::Turn {
                    self.phase = Phase::Second;
                } else {
                    self.finish_turn();
                }
            }
            Move::Face(c) => {
                if let Some(pos) = self.face.iter().position(|&x| x == c) {
                    self.face.remove(pos);
                    self.hands[me.idx()][c as usize] += 1;
                    self.refill_face();
                }
                if c == 5 || self.phase == Phase::Second {
                    self.finish_turn();
                } else {
                    self.phase = Phase::Second;
                }
            }
            Move::Claim(i, c) => {
                let i = i as usize;
                let pay = self.payment(me, i, c).expect("plată");
                for (col, &n) in pay.iter().enumerate() {
                    self.hands[me.idx()][col] -= n;
                    for _ in 0..n {
                        self.discard.push(col as u8);
                    }
                }
                let len = route(i).2 as usize;
                self.owner[i] = Some(me);
                self.trains[me.idx()] -= len as i32;
                self.score[me.idx()] += ROUTE_POINTS[len];
                self.finish_turn();
            }
            Move::Pass => {
                if self.phase == Phase::Second || self.phase == Phase::Turn {
                    self.finish_turn();
                }
            }
        }
        if self.result.is_none() && self.plies >= PLY_CAP {
            self.finish_game();
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.result
    }

    fn evaluate(&self, p: Player) -> i32 {
        let q = p.other();
        let mut s = 10 * (self.score[p.idx()] - self.score[q.idx()]);
        // cărți: doar cele care nu au fost trase pe orb în această mutare
        let mut cards = 0;
        for c in 0..6 {
            let n = self.hands[p.idx()][c] as i32 - self.blind[p.idx()][c] as i32;
            cards += n;
            s += n * n;
            if c == WILD {
                s += 3 * n;
            }
        }
        s += 2 * cards;
        s += 2
            * (self.hand_total(p) as i32 - self.hand_total(q) as i32)
                .signum()
                .min(0);
        s += self.est_tickets(p);
        if let Phase::Keep { setup: false, .. } = self.phase {
            if self.side == p {
                // oferta nu se citește (nu o cunosc „în avans”); bonus fix mic dacă am puține bilete
                if self.tickets[p.idx()].len() <= 4 {
                    s += 25;
                }
            }
        }
        s
    }

    fn render(&self) -> String {
        let mut s = String::from("Magistrale — rute (id: orașe, lungime, culoare, ocupant)\n");
        let n = route_count();
        let half = n.div_ceil(2);
        for r in 0..half {
            let mut line = String::new();
            for i in [r, r + half] {
                if i >= n {
                    continue;
                }
                let (a, b, l, c) = route(i);
                let col = if c == GRAY {
                    '*'
                } else {
                    COLOR_LETTERS[c as usize]
                };
                let own = match self.owner[i] {
                    Some(o) => format!(" [J{}]", o.idx() + 1),
                    None => String::new(),
                };
                let cell = format!(
                    "{:>2}: {}–{} {}{}{}",
                    i, CITIES[a as usize], CITIES[b as usize], l, col, own
                );
                let pad = 46usize.saturating_sub(cell.chars().count());
                line.push_str(&cell);
                line.push_str(&" ".repeat(pad));
            }
            s.push_str(line.trim_end());
            s.push('\n');
        }
        s.push_str("(* = gri, merge cu orice culoare)\n");
        let face: Vec<String> = self
            .face
            .iter()
            .map(|&c| COLOR_LETTERS[c as usize].to_string())
            .collect();
        s.push_str(&format!(
            "Cărți vizibile: {} | pachet: {} | aruncate: {} | bilete rămase: {}\n",
            face.join(" "),
            self.deck.len(),
            self.discard.len(),
            self.ticket_deck.len()
        ));
        s.push_str(&format!(
            "Puncte: J1 {} ({} vagoane), J2 {} ({} vagoane)\n",
            self.score[0], self.trains[0], self.score[1], self.trains[1]
        ));
        if let Some(f) = self.final_scores {
            s.push_str(&format!(
                "Joc încheiat. Scor final (cu bilete și drum lung): J1 {}, J2 {}\n",
                f[0], f[1]
            ));
            return s;
        }
        let v = self.viewer.unwrap_or_else(|| self.to_move());
        let hand = &self.hands[v.idx()];
        let h: Vec<String> = (0..6)
            .filter(|&c| hand[c] > 0)
            .map(|c| format!("{}×{}", COLOR_LETTERS[c], hand[c]))
            .collect();
        s.push_str(&format!(
            "Mâna Jucătorului {}: {}\n",
            v.idx() + 1,
            if h.is_empty() {
                "—".into()
            } else {
                h.join(" ")
            }
        ));
        let tl: Vec<String> = self.tickets[0]
            .iter()
            .map(|&t| {
                let (a, b) = TICKET_PAIRS[t];
                format!(
                    "{}–{} ({}{})",
                    CITIES[a as usize],
                    CITIES[b as usize],
                    ticket_value(t),
                    if self.ticket_done(Player::One, t) {
                        ", făcut"
                    } else {
                        ""
                    }
                )
            })
            .collect();
        s.push_str(&format!("Biletele Jucătorului 1: {}\n", tl.join("; ")));
        s.push_str(&format!(
            "Jucătorul 2: {} cărți, {} bilete\n",
            self.hand_total(Player::Two),
            self.tickets[1].len()
        ));
        if let Phase::Keep { min, .. } = self.phase {
            let o: Vec<String> = self
                .offer
                .iter()
                .enumerate()
                .map(|(i, &t)| {
                    let (a, b) = TICKET_PAIRS[t];
                    format!(
                        "{}) {}–{} ({})",
                        i + 1,
                        CITIES[a as usize],
                        CITIES[b as usize],
                        ticket_value(t)
                    )
                })
                .collect();
            if self.side == Player::One {
                s.push_str(&format!(
                    "Ofertă de bilete (păstrezi cel puțin {min}): {}\n",
                    o.join("; ")
                ));
            } else {
                s.push_str("Jucătorul 2 alege biletele.\n");
            }
        }
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Draw => "trage".into(),
            Move::Face(c) => format!("trage {}", COLOR_LETTERS[*c as usize]),
            Move::Claim(i, c) => format!("ocupa {} {}", i, COLOR_LETTERS[*c as usize]),
            Move::DrawTickets => "bilete".into(),
            Move::Keep(mask) => {
                let v: Vec<String> = (0..self.offer.len())
                    .filter(|i| mask & (1 << i) != 0)
                    .map(|i| (i + 1).to_string())
                    .collect();
                format!("pastrez {}", v.join(","))
            }
            Move::Pass => "pas".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_is_valid() {
        assert_eq!(route_count(), 42);
        let d = floyd(&|i| Some(route(i).2 as i32));
        for a in 0..CITIES.len() {
            for b in 0..CITIES.len() {
                assert!(d[a][b] < INF, "{} – {} neconectate", CITIES[a], CITIES[b]);
            }
        }
        for t in 0..TICKET_PAIRS.len() {
            assert!(ticket_value(t) >= 4, "bilet {t}");
        }
        for i in 0..route_count() {
            let (a, b, l, c) = route(i);
            assert!(a != b && (1..=6).contains(&l) && c <= 5);
        }
    }

    #[test]
    fn setup_deals_conserve() {
        let g = Magistrale::with_seed(3);
        assert_eq!(g.card_total(), 70);
        assert_eq!(g.ticket_total(), 20);
        assert_eq!(g.hand_total(Player::One), 4);
        assert_eq!(g.face.len(), 5);
        assert!(g.face.iter().filter(|&&c| c == 5).count() < 3);
        assert_eq!(g.legal_moves().len(), 4); // 3 oferte: 3 perechi + 1 triplet
    }

    fn started(seed: u64) -> Magistrale {
        let mut g = Magistrale::with_seed(seed);
        g.apply(&Move::Keep(0b011));
        g.apply(&Move::Keep(0b111));
        assert_eq!(g.to_move(), Player::One);
        assert_eq!(g.tickets[0].len(), 2);
        assert_eq!(g.tickets[1].len(), 3);
        g
    }

    #[test]
    fn draw_two_cards_flow() {
        let mut g = started(4);
        g.apply(&Move::Draw);
        assert_eq!(g.to_move(), Player::One);
        assert!(!g.legal_moves().iter().any(|m| matches!(m, Move::Claim(..))));
        g.apply(&Move::Draw);
        assert_eq!(g.to_move(), Player::Two);
        assert_eq!(g.hand_total(Player::One), 6);
        assert_eq!(g.card_total(), 70);
    }

    #[test]
    fn face_wild_takes_whole_turn() {
        let mut g = started(5);
        g.face = vec![5, 0, 1, 2, 3];
        g.apply(&Move::Face(5));
        assert_eq!(g.to_move(), Player::Two);
        let mut g = started(5);
        g.face = vec![5, 0, 1, 2, 3];
        g.apply(&Move::Face(0));
        assert!(!g.legal_moves().iter().any(|m| matches!(m, Move::Face(5))));
    }

    #[test]
    fn claiming_a_route_scores_and_pays() {
        let mut g = started(6);
        g.hands[0] = [0, 0, 0, 0, 0, 0];
        g.hands[0][0] = 2;
        g.hands[0][5] = 1;
        // ruta 3: Arad–Deva, 3, roșu
        assert!(g.legal_moves().contains(&Move::Claim(3, 0)));
        // gri: ruta 0 (lungime 1) cu orice culoare
        assert!(g.legal_moves().contains(&Move::Claim(0, 0)));
        let total = g.card_total();
        g.apply(&Move::Claim(3, 0));
        assert_eq!(g.owner_of(3), Some(Player::One));
        assert_eq!(g.score(Player::One), 4);
        assert_eq!(g.trains(Player::One), 37);
        assert_eq!(g.hand_total(Player::One), 0);
        assert_eq!(g.card_total(), total);
    }

    #[test]
    fn double_route_blocks_twin() {
        let mut g = started(7);
        g.hands[0][0] = 6;
        g.apply(&Move::Claim(6, 0)); // Oradea–Cluj roșu
        g.side = Player::Two;
        g.hands[1][1] = 6;
        let m = g.legal_moves();
        assert!(!m.contains(&Move::Claim(7, 1)), "gemena e blocată");
        assert!(m.contains(&Move::Claim(9, 2)) || g.hands[1][2] == 0);
    }

    #[test]
    fn wrong_color_rejected() {
        let mut g = started(8);
        g.hands[0] = [0; 6];
        g.hands[0][1] = 5;
        let m = g.legal_moves();
        assert!(!m.contains(&Move::Claim(3, 1)), "ruta 3 e roșie");
        assert!(m.contains(&Move::Claim(4, 1)), "ruta 4 e albastră");
    }

    #[test]
    fn longest_path_and_tickets() {
        let mut g = started(9);
        // J1: Timișoara–Arad(0) și Arad–Deva(3)
        g.owner[0] = Some(Player::One);
        g.owner[3] = Some(Player::One);
        assert_eq!(g.longest(Player::One), 4);
        let t = TICKET_PAIRS.iter().position(|&x| x == (0, 5));
        assert!(t.is_none());
        g.tickets[0] = vec![TICKET_PAIRS.iter().position(|&x| x == (0, 13)).unwrap()];
        assert!(!g.ticket_done(Player::One, g.tickets[0][0]));
    }

    #[test]
    fn end_game_triggered_by_low_trains() {
        let mut g = started(10);
        g.trains[0] = 1;
        g.apply(&Move::Draw);
        g.apply(&Move::Draw);
        assert!(g.end_after.is_some());
        assert!(g.outcome().is_none());
        g.apply(&Move::Draw);
        g.apply(&Move::Draw); // ultima mutare a lui J2
        assert!(g.outcome().is_some());
        assert!(g.final_scores().is_some());
    }

    #[test]
    fn random_games_end_conserving_everything() {
        for seed in 1..=12u64 {
            let mut g = Magistrale::with_seed(seed);
            let mut rng = Rng::new(seed + 99);
            let mut plies = 0;
            while Game::outcome(&g).is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                assert_eq!(g.card_total(), 70);
                assert_eq!(g.ticket_total(), 20);
                plies += 1;
                assert!(plies < 4000);
            }
        }
    }

    #[test]
    fn render_hides_opponent() {
        let g = started(11);
        let r = Game::render(&g);
        assert!(r.contains("Mâna Jucătorului 1"));
        assert!(!r.contains("Mâna Jucătorului 2"));
        assert!(!r.contains("Biletele Jucătorului 2"));
    }

    #[test]
    fn ai_not_worse_than_random() {
        let (mut ai, mut rnd) = (0, 0);
        for seed in 1..=10u64 {
            let ai_side = if seed % 2 == 0 {
                Player::One
            } else {
                Player::Two
            };
            let mut g = Magistrale::with_seed(seed);
            let mut rng = Rng::new(seed * 7);
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
