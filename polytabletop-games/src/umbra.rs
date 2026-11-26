//! Umbra Urbei — urmărire pe hartă cu bilete și informație ascunsă (variantă originală), 2 jucători.
//!
//! * Jucătorul 1 conduce patru **detectivi** (d1…d4), Jucătorul 2 este **Umbra** (fugarul).
//!   Harta „Orașul Cenușiu” are 30 de stații numerotate 1–30 (grila 6×5), legate pe trei rețele:
//!   **jos** (străzi), **tram** (diagonale lungi) și **metro** (5 stații: 2, 9, 16, 21, 29).
//! * Fiecare detectiv are 10 bilete jos, 8 tram, 4 metro. Umbra are 12 jos, 8 tram, 4 metro și
//!   3 bilete **negre** (ascund tipul de transport; valabile pe orice legătură).
//! * Tura: J1 mută pe rând fiecare detectiv (`d2 17 tram`; un detectiv fără mutare e sărit), apoi Umbra
//!   (`x 12 jos`, `x 12 negru`). Începe J1; Umbra pornește dintr-una din 6 stații centrale, cunoscute
//!   public (9, 10, 15, 16, 21, 22). Detectivii nu pot sta pe aceeași stație.
//! * Umbra se **arată** după mutările 4, 9, 14 și 19 (poziția apare în `render()`); altfel se vede doar
//!   biletul folosit. Detectivii câștigă dacă ajung pe stația Umbrei sau o blochează; Umbra câștigă dacă
//!   rezistă celor 22 de mutări (și răspunsului final) sau dacă toți detectivii rămân fără mutări.
//! * Simplificare: fără bilet dublu; biletele detectivilor nu merg la Umbră.
//!
//! # Informație ascunsă
//! Poziția Umbrei e ținută în stare, dar `render()` o arată doar la reveniri și la final. Pentru a juca
//! onest ca detectivi, `render()` afișează și **mulțimea stațiilor posibile** (calculată din biletele
//! folosite și din revelări) — exact ce ar putea deduce un jucător atent. AI-ul detectiv folosește doar
//! această mulțime; AI-ul Umbră își cunoaște poziția (e a lui) și vede detectivii (public).

#![allow(clippy::needless_range_loop)]

use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::collections::VecDeque;
use std::sync::OnceLock;

pub const NODES: usize = 30;
pub const ROUNDS: u8 = 22;
pub const REVEAL: [u8; 4] = [4, 9, 14, 19];
pub const DET_START: [u8; 4] = [1, 6, 25, 30];
pub const X_STARTS: [u8; 6] = [9, 10, 15, 16, 21, 22];
const MODE_NAMES: [&str; 4] = ["jos", "tram", "metro", "negru"];
const TRAM: [(u8, u8); 12] = [
    (1, 8),
    (3, 10),
    (5, 12),
    (7, 14),
    (9, 16),
    (11, 18),
    (13, 20),
    (15, 22),
    (17, 24),
    (19, 26),
    (21, 28),
    (23, 30),
];
const METRO: [(u8, u8); 5] = [(2, 9), (9, 16), (16, 21), (16, 29), (2, 21)];

/// Legături (a, b, mod) cu stațiile numerotate de la 1.
pub fn edges() -> &'static Vec<(u8, u8, usize)> {
    static E: OnceLock<Vec<(u8, u8, usize)>> = OnceLock::new();
    E.get_or_init(|| {
        let mut v = Vec::new();
        for r in 0..5u8 {
            for c in 0..5u8 {
                let n = r * 6 + c + 1;
                v.push((n, n + 1, 0));
            }
        }
        for r in 0..4u8 {
            for c in [0u8, 2, 5] {
                let n = r * 6 + c + 1;
                v.push((n, n + 6, 0));
            }
        }
        v.push((4, 10, 0));
        v.push((16, 22, 0));
        for &(a, b) in TRAM.iter() {
            v.push((a, b, 1));
        }
        for &(a, b) in METRO.iter() {
            v.push((a, b, 2));
        }
        v
    })
}

/// Vecini (index 0-based, mod) pentru fiecare nod 0-based.
fn adj() -> &'static Vec<Vec<(u8, usize)>> {
    static A: OnceLock<Vec<Vec<(u8, usize)>>> = OnceLock::new();
    A.get_or_init(|| {
        let mut a = vec![Vec::new(); NODES];
        for &(x, y, m) in edges() {
            a[x as usize - 1].push((y - 1, m));
            a[y as usize - 1].push((x - 1, m));
        }
        a
    })
}

/// Distanțe în pași (orice rețea).
fn dist() -> &'static Vec<Vec<u8>> {
    static D: OnceLock<Vec<Vec<u8>>> = OnceLock::new();
    D.get_or_init(|| {
        let a = adj();
        let mut all = vec![vec![u8::MAX; NODES]; NODES];
        for (s, row) in all.iter_mut().enumerate() {
            row[s] = 0;
            let mut q = VecDeque::new();
            q.push_back(s);
            while let Some(u) = q.pop_front() {
                for &(w, _) in &a[u] {
                    if row[w as usize] == u8::MAX {
                        row[w as usize] = row[u] + 1;
                        q.push_back(w as usize);
                    }
                }
            }
        }
        all
    })
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    /// Detectivul `i` merge la nodul (0-based) cu biletul `mod`.
    Det(u8, u8, u8),
    /// Umbra merge la nod cu bilet (3 = negru).
    X(u8, u8),
}

#[derive(Clone)]
pub struct Umbra {
    det: [u8; 4],
    det_tickets: [[u8; 3]; 4],
    x: u8,
    x_tickets: [u8; 4],
    side: Player,
    idx: usize,
    /// Mutări făcute de Umbră.
    moves_done: u8,
    belief: u32,
    seen: Option<(u8, u8)>,
    history: Vec<(u8, Option<u8>)>,
    winner: Option<Player>,
    log: String,
    /// `Some(Two)` = the Shadow's view (shows its true station); otherwise the detectives'.
    /// `None` = hot-seat (the mover's).
    viewer: Option<Player>,
}

impl Default for Umbra {
    fn default() -> Self {
        Self::new()
    }
}

impl Umbra {
    pub fn new() -> Self {
        Self::with_seed(polytabletop_core::rng::Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = polytabletop_core::rng::Rng::new(seed);
        let x = X_STARTS[rng.below(X_STARTS.len())] - 1;
        let mut belief = 0u32;
        for &s in X_STARTS.iter() {
            belief |= 1 << (s - 1);
        }
        let mut g = Umbra {
            det: DET_START.map(|d| d - 1),
            det_tickets: [[10, 8, 4]; 4],
            x,
            x_tickets: [12, 8, 4, 3],
            side: Player::One,
            idx: 0,
            moves_done: 0,
            belief,
            seen: None,
            history: Vec::new(),
            winner: None,
            log: String::new(),
            viewer: Some(Player::One),
        };
        g.skip_stuck();
        g
    }

    pub fn detectives(&self) -> [u8; 4] {
        self.det.map(|d| d + 1)
    }

    pub fn x_position(&self) -> u8 {
        self.x + 1
    }

    pub fn possible(&self) -> Vec<u8> {
        (0..NODES as u8)
            .filter(|&n| self.belief & (1 << n) != 0)
            .map(|n| n + 1)
            .collect()
    }

    fn occupied_by_det(&self, n: u8) -> bool {
        self.det.contains(&n)
    }

    fn det_moves(&self, i: usize) -> Vec<(u8, u8)> {
        let mut v = Vec::new();
        for &(nb, m) in &adj()[self.det[i] as usize] {
            if self.det_tickets[i][m] > 0
                && !self.det.iter().enumerate().any(|(j, &d)| j != i && d == nb)
                && !v.contains(&(nb, m as u8))
            {
                v.push((nb, m as u8));
            }
        }
        v
    }

    fn x_moves(&self) -> Vec<(u8, u8)> {
        let mut v: Vec<(u8, u8)> = Vec::new();
        for &(nb, m) in &adj()[self.x as usize] {
            if self.occupied_by_det(nb) {
                continue;
            }
            if self.x_tickets[m] > 0 && !v.contains(&(nb, m as u8)) {
                v.push((nb, m as u8));
            }
        }
        if self.x_tickets[3] > 0 {
            for &(nb, _) in &adj()[self.x as usize] {
                if !self.occupied_by_det(nb) && !v.contains(&(nb, 3)) {
                    v.push((nb, 3));
                }
            }
        }
        v
    }

    /// Sare peste detectivii fără mutări; dacă nu mai e niciunul, trece la Umbră (sau Umbra câștigă).
    fn skip_stuck(&mut self) {
        while self.idx < 4 && self.det_moves(self.idx).is_empty() {
            self.idx += 1;
        }
        if self.idx >= 4 {
            self.finish_det_phase();
        }
    }

    fn finish_det_phase(&mut self) {
        // a rămas vreo mutare posibilă la începutul fazei?
        if self.moves_done >= ROUNDS {
            self.winner = Some(Player::Two);
            return;
        }
        self.side = Player::Two;
        self.idx = 0;
    }

    fn begin_det_phase(&mut self) {
        self.side = Player::One;
        self.idx = 0;
        if (0..4).all(|i| self.det_moves(i).is_empty()) {
            self.winner = Some(Player::Two);
            self.log.push_str(" Detectivii nu mai au mutări.");
            return;
        }
        self.skip_stuck();
    }

    fn belief_after(&self, ticket: u8, belief: u32) -> u32 {
        let mut nb = 0u32;
        for n in 0..NODES {
            if belief & (1 << n) == 0 {
                continue;
            }
            for &(w, m) in &adj()[n] {
                if (ticket == 3 || ticket as usize == m) && !self.occupied_by_det(w) {
                    nb |= 1 << w;
                }
            }
        }
        nb
    }

    fn dets_threat(&self, n: u8) -> bool {
        // un detectiv poate ajunge pe `n` dintr-o mutare?
        (0..4).any(|i| {
            adj()[self.det[i] as usize]
                .iter()
                .any(|&(w, m)| w == n && self.det_tickets[i][m] > 0)
        })
    }
}

impl Game for Umbra {
    type Move = Move;

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        if self.side == Player::One {
            self.det_moves(self.idx)
                .into_iter()
                .map(|(n, m)| Move::Det(self.idx as u8, n, m))
                .collect()
        } else {
            self.x_moves()
                .into_iter()
                .map(|(n, t)| Move::X(n, t))
                .collect()
        }
    }

    fn apply(&mut self, m: &Move) {
        match *m {
            Move::Det(i, to, mode) => {
                let i = i as usize;
                self.det_tickets[i][mode as usize] -= 1;
                self.det[i] = to;
                self.log = format!("d{} → {} ({}).", i + 1, to + 1, MODE_NAMES[mode as usize]);
                if to == self.x {
                    self.winner = Some(Player::One);
                    self.log.push_str(" Umbra a fost prinsă!");
                    return;
                }
                // Umbra nu e pe nicio stație ocupată de detectivi
                for &d in &self.det {
                    self.belief &= !(1 << d);
                }
                self.idx += 1;
                self.skip_stuck();
            }
            Move::X(to, t) => {
                self.x = to;
                self.x_tickets[t as usize] -= 1;
                self.moves_done += 1;
                let nb = self.belief_after(t, self.belief);
                self.belief = nb;
                let reveal = REVEAL.contains(&self.moves_done);
                if reveal {
                    self.seen = Some((self.moves_done, to));
                    self.belief = 1 << to;
                }
                self.history.push((t, if reveal { Some(to) } else { None }));
                self.log = format!(
                    "Umbra: mutarea {} cu bilet {}{}.",
                    self.moves_done,
                    MODE_NAMES[t as usize],
                    if reveal {
                        format!(" — apare la stația {}", to + 1)
                    } else {
                        String::new()
                    }
                );
                self.begin_det_phase();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if let Some(w) = self.winner {
            return Some(Outcome::Win(w));
        }
        if self.side == Player::Two && self.x_moves().is_empty() {
            return Some(Outcome::Win(Player::One));
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let d = dist();
        if p == Player::Two {
            // punctul de vedere al Umbrei: își știe poziția
            let x = self.x as usize;
            let mut v = 0i32;
            let mut mind = u8::MAX;
            for &dd in &self.det {
                let k = d[dd as usize][x];
                mind = mind.min(k);
                v += k.min(6) as i32 * 4;
            }
            v += mind.min(5) as i32 * 30;
            if self.dets_threat(self.x) {
                v -= 400;
            }
            v -= self
                .det
                .iter()
                .filter(|&&dd| d[dd as usize][x] <= 2)
                .count() as i32
                * 25;
            // ieșiri: vecini liberi, mai ales cei pe care niciun detectiv nu-i atinge dintr-o mutare
            let mut exits = 0;
            for &(w, m) in &adj()[x] {
                if !self.occupied_by_det(w) && (self.x_tickets[m] > 0 || self.x_tickets[3] > 0) {
                    exits += 1;
                    v += if self.dets_threat(w) { 4 } else { 14 };
                }
            }
            if exits == 0 {
                v -= 2000;
            }
            // bilete rămase: cele de jos sunt cele mai ușor de epuizat; negrele sunt rezerva de urgență
            let moves_left = (ROUNDS - self.moves_done) as i32;
            v += (self.x_tickets[0] as i32).min(moves_left) * 5
                + self.x_tickets[1] as i32 * 3
                + self.x_tickets[2] as i32 * 3
                + self.x_tickets[3] as i32 * 14;
            v
        } else {
            // punctul de vedere al detectivilor: doar mulțimea stațiilor posibile
            let b: Vec<usize> = (0..NODES)
                .filter(|&n| self.belief & (1 << n) != 0)
                .collect();
            if b.is_empty() {
                return 0;
            }
            let mut total = 0i32;
            let mut covered = 0i32;
            for &n in &b {
                let k = self.det.iter().map(|&dd| d[dd as usize][n]).min().unwrap() as i32;
                total += k;
                if self.dets_threat(n as u8) {
                    covered += 1;
                }
            }
            -(total * 40) / b.len() as i32 - b.len() as i32 * 3 + covered * 20 / b.len() as i32
        }
    }

    fn render(&self) -> String {
        let mut s =
            String::from("Umbra Urbei — Orașul Cenușiu (J1 = detectivi d1–d4, J2 = Umbra)\n");
        s.push_str("      1    2    3    4    5    6   (stații numerotate pe rânduri de câte 6)\n");
        let possible = self.possible();
        let shadow = self.viewer.unwrap_or(self.side) == Player::Two;
        for r in 0..5 {
            let mut line = String::new();
            for c in 0..6 {
                let n = r * 6 + c;
                let cell = if let Some(i) = self.det.iter().position(|&d| d as usize == n) {
                    format!("d{}", i + 1)
                } else if self.winner.is_some() && self.x as usize == n {
                    "U!".to_string()
                } else if shadow && self.x as usize == n {
                    "U".to_string()
                } else if possible.contains(&(n as u8 + 1)) && self.winner.is_none() {
                    "?".to_string()
                } else {
                    String::new()
                };
                let metro = if [2u8, 9, 16, 21, 29].contains(&(n as u8 + 1)) {
                    "m"
                } else {
                    " "
                };
                line.push_str(&format!("{:>2}{}{:<2} ", n + 1, metro, cell));
            }
            s.push_str(&format!("  {line}\n"));
        }
        s.push_str("(m = stație de metro; d = detectiv; ? = poziții posibile ale Umbrei)\n");
        s.push_str("Jos: ");
        for (a, b, m) in edges() {
            if *m == 0 {
                s.push_str(&format!("{a}-{b} "));
            }
        }
        s.push_str("\nTram: ");
        for (a, b) in TRAM {
            s.push_str(&format!("{a}-{b} "));
        }
        s.push_str("\nMetro: ");
        for (a, b) in METRO {
            s.push_str(&format!("{a}-{b} "));
        }
        s.push('\n');
        for i in 0..4 {
            s.push_str(&format!(
                "d{}: stația {}  bilete jos {} tram {} metro {}\n",
                i + 1,
                self.det[i] + 1,
                self.det_tickets[i][0],
                self.det_tickets[i][1],
                self.det_tickets[i][2]
            ));
        }
        s.push_str(&format!(
            "Umbra: {} mutări din {}; bilete jos {} tram {} metro {} negre {}\n",
            self.moves_done,
            ROUNDS,
            self.x_tickets[0],
            self.x_tickets[1],
            self.x_tickets[2],
            self.x_tickets[3]
        ));
        let hist: Vec<String> = self
            .history
            .iter()
            .enumerate()
            .map(|(i, (t, p))| match p {
                Some(p) => format!("{}:{}@{}", i + 1, MODE_NAMES[*t as usize], p + 1),
                None => format!("{}:{}", i + 1, MODE_NAMES[*t as usize]),
            })
            .collect();
        s.push_str(&format!("Istoric Umbră: {}\n", hist.join(" ")));
        let poss: Vec<String> = possible.iter().map(|n| n.to_string()).collect();
        if self.winner.is_none() {
            s.push_str(&format!(
                "Stații posibile ({}): {}\n",
                poss.len(),
                poss.join(" ")
            ));
            if shadow {
                s.push_str(&format!("Umbra (tu) te afli la stația {}.\n", self.x + 1));
            }
            if self.side == Player::One {
                s.push_str(&format!(
                    "La mutare: detectivul d{} (ex. «d{} 17 tram»)\n",
                    self.idx + 1,
                    self.idx + 1
                ));
            } else {
                s.push_str("La mutare: Umbra (ex. «x 12 jos»)\n");
            }
        } else {
            s.push_str(&format!("Umbra se afla la stația {}.\n", self.x + 1));
        }
        s.push_str(self.log.trim());
        s.push('\n');
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match *m {
            Move::Det(i, to, mode) => {
                format!("d{} {} {}", i + 1, to + 1, MODE_NAMES[mode as usize])
            }
            Move::X(to, t) => format!("x {} {}", to + 1, MODE_NAMES[t as usize]),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    #[test]
    fn map_is_symmetric_connected_and_sane() {
        let a = adj();
        for (n, row) in a.iter().enumerate() {
            assert!(!row.is_empty());
            for &(w, m) in row {
                assert_ne!(w as usize, n);
                assert!(a[w as usize].contains(&(n as u8, m)), "asimetrie {n} {w}");
            }
        }
        let d = dist();
        for i in 0..NODES {
            for j in 0..NODES {
                assert!(d[i][j] != u8::MAX, "{i} {j}");
            }
        }
        // metro are exact 5 stații
        let mut st: Vec<u8> = METRO.iter().flat_map(|&(a, b)| [a, b]).collect();
        st.sort();
        st.dedup();
        assert_eq!(st, vec![2, 9, 16, 21, 29]);
        // fără duplicate în listă
        let mut es: Vec<(u8, u8, usize)> = edges()
            .iter()
            .map(|&(a, b, m)| (a.min(b), a.max(b), m))
            .collect();
        let n = es.len();
        es.sort();
        es.dedup();
        assert_eq!(es.len(), n);
    }

    #[test]
    fn start_state_is_public_set_and_hidden_position() {
        let g = Umbra::with_seed(3);
        assert_eq!(g.possible(), X_STARTS.to_vec());
        assert!(X_STARTS.contains(&g.x_position()));
        let r = Game::render(&g);
        assert!(!r.contains("Umbra se afla"));
        assert!(!r.contains("U!"));
        assert_eq!(g.to_move(), Player::One);
    }

    #[test]
    fn detective_phase_goes_d1_to_d4_then_umbra() {
        let mut g = Umbra::with_seed(3);
        for i in 0..4usize {
            let m = g.legal_moves();
            assert!(m
                .iter()
                .all(|x| matches!(x, Move::Det(k, _, _) if *k as usize == i)));
            g.apply(&m[0]);
            if g.winner.is_some() {
                return;
            }
        }
        assert_eq!(g.side, Player::Two);
        assert!(g.legal_moves().iter().all(|x| matches!(x, Move::X(_, _))));
    }

    #[test]
    fn tickets_decrease_and_run_out() {
        let mut g = Umbra::with_seed(1);
        g.det_tickets[0] = [0, 0, 0];
        g.skip_stuck();
        assert_eq!(g.idx, 1);
        let m = g.legal_moves()[0];
        let before = g.det_tickets[1];
        g.apply(&m);
        if let Move::Det(i, _, mode) = m {
            assert_eq!(
                g.det_tickets[i as usize][mode as usize],
                before[mode as usize] - 1
            );
        }
    }

    #[test]
    fn detectives_cannot_share_a_station() {
        let mut g = Umbra::with_seed(1);
        g.det = [0, 1, 20, 29];
        for m in g.det_moves(0) {
            assert_ne!(m.0, 1);
        }
    }

    #[test]
    fn capture_by_landing_on_umbra() {
        let mut g = Umbra::with_seed(1);
        g.x = 7;
        g.det = [6, 0, 24, 29];
        let m = g
            .legal_moves()
            .into_iter()
            .find(|m| matches!(m, Move::Det(0, 7, 0)))
            .unwrap();
        g.apply(&m);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn umbra_cannot_step_on_detectives_and_black_hides_ticket() {
        let mut g = Umbra::with_seed(1);
        g.side = Player::Two;
        g.x = 8; // stația 9
        g.det = [9, 14, 7, 29]; // în jurul lui 9: 8, 10, 3, 15...
        for (n, _) in g.x_moves() {
            assert!(!g.det.contains(&n));
        }
        let before = g.x_tickets[3];
        let m = g
            .legal_moves()
            .into_iter()
            .find(|m| matches!(m, Move::X(_, 3)))
            .unwrap();
        g.apply(&m);
        assert_eq!(g.x_tickets[3], before - 1);
        assert_eq!(g.history.last().unwrap().0, 3);
    }

    #[test]
    fn belief_always_contains_the_true_position() {
        for seed in 1..=60u64 {
            let mut g = Umbra::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                g.apply(&m[rng.below(m.len())]);
                if g.winner != Some(Player::One) {
                    assert!(
                        g.belief & (1 << g.x) != 0,
                        "seed {seed} plies {n}: {:?}",
                        g.possible()
                    );
                }
                n += 1;
                assert!(n < 400);
            }
        }
    }

    #[test]
    fn reveal_rounds_collapse_belief() {
        let mut g = Umbra::with_seed(2);
        let mut rng = Rng::new(9);
        while g.moves_done < 4 && g.outcome().is_none() {
            let m = g.legal_moves();
            g.apply(&m[rng.below(m.len())]);
        }
        if g.outcome().is_none() || g.winner == Some(Player::Two) {
            assert!(g.seen.is_some());
        }
    }

    #[test]
    fn x_wins_after_22_moves_if_not_caught() {
        let mut g = Umbra::with_seed(1);
        g.moves_done = ROUNDS - 1;
        g.side = Player::Two;
        g.x = 14;
        g.det = [0, 5, 24, 29];
        let m = g.legal_moves()[0];
        g.apply(&m);
        assert_eq!(g.moves_done, ROUNDS);
        // detectivii mai au o mutare
        assert!(g.outcome().is_none());
        for _ in 0..4 {
            if g.outcome().is_some() {
                break;
            }
            let m = g.legal_moves();
            if let Some(&mv) = m
                .iter()
                .find(|m| matches!(m, Move::Det(_, n, _) if *n != g.x))
            {
                g.apply(&mv);
            } else {
                break;
            }
        }
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
    }

    #[test]
    fn ai_as_detectives_uses_only_the_belief() {
        // aceeași poziție publică, Umbră diferită: aceeași evaluare pentru J1
        let mut a = Umbra::with_seed(1);
        let mut b = a.clone();
        a.x = 9;
        b.x = 15;
        assert_eq!(a.evaluate(Player::One), b.evaluate(Player::One));
        assert_eq!(
            polytabletop_core::ai::best_move(&a, 5),
            polytabletop_core::ai::best_move(&b, 5)
        );
    }

    #[test]
    fn ai_vs_random_games_finish() {
        for seed in 1..=20u64 {
            let ai = if seed % 2 == 0 {
                Player::One
            } else {
                Player::Two
            };
            let mut g = Umbra::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = if g.side == ai {
                    polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap()
                } else {
                    let m = g.legal_moves();
                    m[rng.below(m.len())]
                };
                g.apply(&mv);
                n += 1;
                assert!(n < 500);
            }
        }
    }

    #[test]
    fn umbra_ai_escapes_random_detectives_most_of_the_time() {
        let mut escaped = 0;
        for seed in 1..=20u64 {
            let mut g = Umbra::with_seed(seed);
            let mut rng = Rng::new(seed + 5);
            while g.outcome().is_none() {
                let mv = if g.side == Player::Two {
                    polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap()
                } else {
                    let m = g.legal_moves();
                    m[rng.below(m.len())]
                };
                g.apply(&mv);
            }
            if g.outcome() == Some(Outcome::Win(Player::Two)) {
                escaped += 1;
            }
        }
        assert!(escaped >= 14, "{escaped}/20");
    }
}

#[cfg(test)]
mod viewer_tests {
    use super::*;

    #[test]
    fn only_the_shadow_sees_its_station() {
        let mut g = Umbra::with_seed(4);
        let line = format!("Umbra (tu) te afli la stația {}.", g.x + 1);
        Game::set_viewer(&mut g, Some(Player::One));
        let r = Game::render(&g);
        assert!(!r.contains("Umbra (tu)") && !r.contains(" U "), "{r}");
        Game::set_viewer(&mut g, Some(Player::Two));
        let r2 = Game::render(&g);
        assert!(r2.contains(&line), "{r2}");
        Game::set_viewer(&mut g, None); // mută detectivii
        assert_eq!(r, Game::render(&g));
        g.side = Player::Two;
        assert!(Game::render(&g).contains(&line));
    }
}
