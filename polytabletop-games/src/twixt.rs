#![allow(clippy::needless_range_loop)]
#![allow(clippy::doc_lazy_continuation)]
//! TwixT (SUA, 1962 — Alex Randolph), joc de conectare pe tabla 24×24, pentru doi jucători.
//!
//! * J1 (X) leagă marginea de sus de cea de jos (rândurile 24 și 1), J2 (O) leagă marginea din stânga de
//!   cea din dreapta (coloanele a și x). Fiecare plasează pe rând un cui pe o intersecție liberă; J1 nu
//!   poate pune cuie pe coloanele a/x, J2 pe rândurile 1/24 (marginile adversarului), iar colțurile
//!   rămân goale.
//! * **Legături**: două cuie ale aceluiași jucător aflate la „săritura calului” (un pas într-o direcție
//!   și doi în cealaltă) se pot lega cu o punte. Punțile nu se pot încrucișa cu nicio altă punte. Aici
//!   legăturile se fac **automat**: la plasarea unui cui, se leagă cu toate cuiele proprii la distanță de
//!   cal la care nu se încrucișează cu o punte existentă. Nu se pot retrage cuie sau punți.
//! * Câștigă cine închide primul un lanț continuu de punți între cele două margini ale lui. Regula
//!   de schimb (pie rule) nu e folosită.
//! Mutări: `e12` (coloana a–x, rândul 1–24; rândul 24 e sus).
//! AI: evaluare prin distanța celui mai scurt drum (0-1 BFS prin salturi de cal), nu vede nimic ascuns.

use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::collections::VecDeque;

const S: usize = 24;
const N: usize = S * S;
const KN: [(i32, i32); 8] = [
    (1, 2),
    (2, 1),
    (2, -1),
    (1, -2),
    (-1, -2),
    (-2, -1),
    (-2, 1),
    (-1, 2),
];

fn name(i: usize) -> String {
    format!("{}{}", (b'a' + (i % S) as u8) as char, i / S + 1)
}

fn knight(i: usize, k: usize) -> Option<usize> {
    let (r, c) = ((i / S) as i32 + KN[k].0, (i % S) as i32 + KN[k].1);
    if r < 0 || c < 0 || r >= S as i32 || c >= S as i32 {
        None
    } else {
        Some(r as usize * S + c as usize)
    }
}

fn pt(i: usize) -> (i32, i32) {
    ((i % S) as i32, (i / S) as i32)
}

fn orient(a: (i32, i32), b: (i32, i32), c: (i32, i32)) -> i32 {
    ((b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)).signum()
}

fn cross(a: usize, b: usize, c: usize, d: usize) -> bool {
    if a == c || a == d || b == c || b == d {
        return false;
    }
    let (pa, pb, pc, pd) = (pt(a), pt(b), pt(c), pt(d));
    orient(pa, pb, pc) * orient(pa, pb, pd) < 0 && orient(pc, pd, pa) * orient(pc, pd, pb) < 0
}

fn allowed(p: usize, i: usize) -> bool {
    let (r, c) = (i / S, i % S);
    let corner = (r == 0 || r == S - 1) && (c == 0 || c == S - 1);
    if corner {
        return false;
    }
    if p == 0 {
        c != 0 && c != S - 1
    } else {
        r != 0 && r != S - 1
    }
}

#[derive(Clone)]
pub struct Twixt {
    peg: [u8; N],
    /// Pentru fiecare cui, masca direcțiilor de cal cu punte.
    lk: [u8; N],
    links: Vec<(u16, u16)>,
    side: Player,
    won: Option<Player>,
    count: u16,
}

impl Default for Twixt {
    fn default() -> Self {
        Self::new()
    }
}

impl Twixt {
    pub fn new() -> Self {
        Twixt {
            peg: [0; N],
            lk: [0; N],
            links: Vec::new(),
            side: Player::One,
            won: None,
            count: 0,
        }
    }

    fn start_end(p: usize, i: usize) -> (bool, bool) {
        let x = if p == 0 { i / S } else { i % S };
        (x == 0, x == S - 1)
    }

    fn connected(&self, p: usize) -> bool {
        let me = p as u8 + 1;
        let mut seen = [false; N];
        let mut q: Vec<usize> = Vec::new();
        for i in 0..N {
            if self.peg[i] == me && Self::start_end(p, i).0 {
                seen[i] = true;
                q.push(i);
            }
        }
        while let Some(x) = q.pop() {
            if Self::start_end(p, x).1 {
                return true;
            }
            for k in 0..8 {
                if self.lk[x] & (1 << k) != 0 {
                    if let Some(y) = knight(x, k) {
                        if !seen[y] {
                            seen[y] = true;
                            q.push(y);
                        }
                    }
                }
            }
        }
        false
    }

    /// Distanța (nr. de cuie de pus) a celui mai scurt drum al lui `p`, ignorând încrucișările.
    fn dist(&self, p: usize) -> i32 {
        let me = p as u8 + 1;
        let mut d = vec![i32::MAX; N];
        let mut dq: VecDeque<usize> = VecDeque::new();
        for i in 0..N {
            if Self::start_end(p, i).0 && allowed(p, i) && self.peg[i] != 3 - me {
                let w = if self.peg[i] == me { 0 } else { 1 };
                d[i] = w;
                if w == 0 {
                    dq.push_front(i);
                } else {
                    dq.push_back(i);
                }
            }
        }
        let mut best = i32::MAX;
        while let Some(x) = dq.pop_front() {
            if Self::start_end(p, x).1 {
                best = best.min(d[x]);
                continue;
            }
            for k in 0..8 {
                if let Some(y) = knight(x, k) {
                    if !allowed(p, y) || self.peg[y] == 3 - me {
                        continue;
                    }
                    let w = if self.peg[y] == me { 0 } else { 1 };
                    if d[x] + w < d[y] {
                        d[y] = d[x] + w;
                        if w == 0 {
                            dq.push_front(y);
                        } else {
                            dq.push_back(y);
                        }
                    }
                }
            }
        }
        best
    }
}

impl Game for Twixt {
    type Move = u16;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<u16> {
        if self.won.is_some() {
            return Vec::new();
        }
        let p = self.side.idx();
        (0..N)
            .filter(|&i| self.peg[i] == 0 && allowed(p, i))
            .map(|i| i as u16)
            .collect()
    }

    fn apply(&mut self, m: &u16) {
        let i = *m as usize;
        let p = self.side.idx();
        let me = p as u8 + 1;
        self.peg[i] = me;
        for k in 0..8 {
            if let Some(j) = knight(i, k) {
                if self.peg[j] == me
                    && !self
                        .links
                        .iter()
                        .any(|&(a, b)| cross(i, j, a as usize, b as usize))
                {
                    self.links.push((i as u16, j as u16));
                    self.lk[i] |= 1 << k;
                    self.lk[j] |= 1 << ((k + 4) % 8);
                }
            }
        }
        self.count += 1;
        if self.connected(p) {
            self.won = Some(self.side);
        }
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        if let Some(w) = self.won {
            return Some(Outcome::Win(w));
        }
        if self.count > 400 {
            let p = self.side.idx();
            if !(0..N).any(|i| self.peg[i] == 0 && allowed(p, i)) {
                return Some(Outcome::Draw); // tablă plină fără lanț (punțile automate pot bloca)
            }
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let (a, b) = (self.dist(p.idx()), self.dist(p.other().idx()));
        if a == i32::MAX || b == i32::MAX {
            return 0;
        }
        (b - a) * 10
    }

    fn render(&self) -> String {
        let mut s = String::from("    a b c d e f g h i j k l m n o p q r s t u v w x\n");
        for r in (0..S).rev() {
            s.push_str(&format!("{:>2}  ", r + 1));
            for c in 0..S {
                s.push(match self.peg[r * S + c] {
                    1 => 'X',
                    2 => 'O',
                    _ => {
                        let corner = (r == 0 || r == S - 1) && (c == 0 || c == S - 1);
                        if corner {
                            ' '
                        } else if r == 0 || r == S - 1 {
                            '-'
                        } else if c == 0 || c == S - 1 {
                            '|'
                        } else {
                            '.'
                        }
                    }
                });
                s.push(' ');
            }
            s.push('\n');
        }
        s.push_str(&format!(
            "X: sus–jos (rândurile 1 și 24); O: stânga–dreapta (coloanele a și x). Cuie: {}, punți: {}. La mutare: {}.\n",
            self.count,
            self.links.len(),
            if self.side == Player::One { "X" } else { "O" }
        ));
        s
    }

    fn format_move(&self, m: &u16) -> String {
        name(*m as usize)
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 400,
            max_depth: 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    #[test]
    fn crossing_links_are_refused_and_borders_respected() {
        let mut g = Twixt::new();
        g.apply(&((10 * S + 10) as u16)); // X k11
        g.apply(&((11 * S + 10) as u16)); // O
        g.apply(&((12 * S + 11) as u16)); // X: la distanță de cal de (10,10) -> punte
        assert_eq!(g.links.len(), 1);
        // O în (10,11) și (11,12) ar traversa puntea X (10,10)-(12,11)
        let moves = g.legal_moves();
        assert!(moves.iter().all(|&m| allowed(1, m as usize)));
        let mv: Vec<String> = Twixt::new()
            .legal_moves()
            .iter()
            .map(|&m| name(m as usize))
            .collect();
        assert!(!mv.contains(&"a5".to_string()) && mv.contains(&"b1".to_string()));
    }

    #[test]
    fn a_chain_of_links_wins() {
        let mut g = Twixt::new();
        let mut cells: Vec<usize> = (0..12)
            .map(|k| (2 * k) * S + if k % 2 == 0 { 5 } else { 6 })
            .collect();
        cells.push(23 * S + 8);
        for (n, &c) in cells.iter().enumerate() {
            g.side = Player::One;
            g.apply(&(c as u16));
            if n + 1 < cells.len() {
                assert!(g.outcome().is_none());
                g.side = Player::Two;
                g.apply(&(((n % 20 + 2) * S + 1) as u16));
            }
        }
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn random_games_end_and_ai_is_fast() {
        for seed in 1..=3u64 {
            let mut g = Twixt::new();
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
                assert!(n < 700);
            }
        }
        let mut g = Twixt::new();
        g.apply(&((10 * S + 10) as u16));
        let m = polytabletop_core::GameObj::ai_move(&g, 1).unwrap();
        assert!(g.legal_moves().iter().any(|x| g.format_move(x) == m));
    }
}
