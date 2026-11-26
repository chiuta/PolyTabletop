//! Inele și semne (variantă proprie de inele care alunecă pe o tablă hexagonală de 61 de
//! puncte, rază 4): fiecare jucător are 3 inele. Faza 1: pe rând, fiecare pune cele 3 inele pe
//! puncte goale (`r d5`). Faza 2: o tură = alegi un inel propriu, lași în locul lui un semn de
//! culoarea ta (X sau O) și muți inelul pe o linie dreaptă pe un punct gol: poate trece peste
//! puncte goale; când întâlnește semne, trebuie să sară peste tot șirul continuu de semne și
//! să se oprească pe primul punct gol de după el (șirul de semne sărit își schimbă culoarea).
//! Nu poate sări peste alt inel. După mutare, un rând de 4 semne consecutive ale unui jucător
//! (inclusiv ale adversarului, formate prin întoarceri) se scoate; proprietarul rândului scoate
//! apoi unul dintre inelele lui (`x d5`). Cine scoate 2 inele câștigă. Dacă semnele din rezervă
//! (24 pe jucător) se termină, câștigă cine a scos mai multe inele (egal = remiză).
//! Mutări: `r d5`, `d5>d8`, `x d5`.
#![allow(clippy::all)]

use crate::b1_util::{Hex, HEXD};
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const G: Hex = Hex { rad: 4 };

#[derive(Clone, PartialEq, Debug)]
pub enum Mv {
    Ring(u8),
    Slide(u8, u8),
    Remove(u8),
}

#[derive(Clone)]
pub struct Inele {
    // 0 gol, 1/2 inel J1/J2, 3/4 semn J1/J2 (valori ale celulei: nu pot coexista inel și semn în afara mutării)
    c: [u8; 81],
    rings_left: [u8; 2],
    removed: [u8; 2],
    markers: [u8; 2],
    side: Player,
    pending: Option<Player>,
    orig: Player,
    ply: u32,
}

impl Default for Inele {
    fn default() -> Self {
        Self::new()
    }
}

impl Inele {
    pub fn new() -> Self {
        Inele {
            c: [0; 81],
            rings_left: [3, 3],
            removed: [0; 2],
            markers: [24, 24],
            side: Player::One,
            pending: None,
            orig: Player::One,
            ply: 0,
        }
    }
    fn placing(&self) -> bool {
        self.rings_left[0] + self.rings_left[1] > 0
    }
    fn slides(&self, i: usize) -> Vec<usize> {
        let mut v = vec![];
        for d in 0..6 {
            let mut cur = i;
            let mut jumped = false;
            while let Some(n) = G.step(cur, d) {
                let k = self.c[n];
                if k == 1 || k == 2 {
                    break;
                }
                if k == 3 || k == 4 {
                    jumped = true;
                    cur = n;
                    continue;
                }
                // gol
                v.push(n);
                if jumped {
                    break;
                }
                cur = n;
            }
        }
        v
    }
    fn find_row(&self, who: u8) -> Option<Vec<usize>> {
        let mk = who + 2; // 3 sau 4 pentru who=1/2
        for i in G.cells() {
            if self.c[i] != mk {
                continue;
            }
            for d in 0..3 {
                let (q, r) = G.qr(i);
                if G.valid(q - HEXD[d].0, r - HEXD[d].1)
                    && self.c[G.idx(q - HEXD[d].0, r - HEXD[d].1)] == mk
                {
                    continue;
                }
                let mut run = vec![i];
                let mut cur = i;
                while let Some(n) = G.step(cur, d) {
                    if self.c[n] == mk {
                        run.push(n);
                        cur = n;
                    } else {
                        break;
                    }
                }
                if run.len() >= 4 {
                    run.truncate(4);
                    return Some(run);
                }
            }
        }
        None
    }
    /// Caută rânduri: întâi ale lui `orig`, apoi ale celuilalt; dacă găsește, le scoate și cere scoaterea unui inel.
    fn settle(&mut self) {
        for who in [self.orig, self.orig.other()] {
            if self.removed[who.idx()] >= 2 {
                continue;
            }
            if let Some(run) = self.find_row(who.idx() as u8 + 1) {
                for c in run {
                    self.c[c] = 0;
                }
                self.pending = Some(who);
                self.side = who;
                return;
            }
        }
        self.pending = None;
        self.side = self.orig.other();
    }
}

impl Game for Inele {
    type Move = Mv;
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<Mv> {
        if self.outcome().is_some() {
            return vec![];
        }
        let p = self.side.idx() as u8 + 1;
        if let Some(w) = self.pending {
            let wi = w.idx() as u8 + 1;
            return G
                .cells()
                .into_iter()
                .filter(|&i| self.c[i] == wi)
                .map(|i| Mv::Remove(i as u8))
                .collect();
        }
        if self.placing() {
            return G
                .cells()
                .into_iter()
                .filter(|&i| self.c[i] == 0)
                .map(|i| Mv::Ring(i as u8))
                .collect();
        }
        let mut v = vec![];
        for i in G.cells() {
            if self.c[i] == p {
                for t in self.slides(i) {
                    v.push(Mv::Slide(i as u8, t as u8));
                }
            }
        }
        v
    }
    fn apply(&mut self, m: &Mv) {
        let p = self.side.idx();
        match m {
            Mv::Ring(i) => {
                self.c[*i as usize] = p as u8 + 1;
                self.rings_left[p] -= 1;
                self.orig = self.side;
                self.side = self.side.other();
                self.ply += 1;
                return;
            }
            Mv::Slide(f, t) => {
                let (f, t) = (*f as usize, *t as usize);
                self.c[f] = p as u8 + 3;
                self.markers[p] = self.markers[p].saturating_sub(1);
                // întoarce semnele de pe drum
                let (fq, fr) = G.qr(f);
                let (tq, tr) = G.qr(t);
                let d = (0..6).find(|&d| {
                    let (dq, dr) = HEXD[d];
                    let n = ((tq - fq)
                        .abs()
                        .max((tr - fr).abs())
                        .max((tq - fq + tr - fr).abs())) as i32;
                    n > 0 && fq + dq * n == tq && fr + dr * n == tr
                });
                let d = d.unwrap();
                let mut cur = f;
                while let Some(n) = G.step(cur, d) {
                    if n == t {
                        break;
                    }
                    let k = self.c[n];
                    if k == 3 || k == 4 {
                        self.c[n] = 7 - k;
                    }
                    cur = n;
                }
                self.c[t] = p as u8 + 1;
                self.orig = self.side;
            }
            Mv::Remove(i) => {
                self.c[*i as usize] = 0;
                let w = self.pending.unwrap();
                self.removed[w.idx()] += 1;
            }
        }
        self.ply += 1;
        if self.removed[0] >= 2 || self.removed[1] >= 2 {
            self.pending = None;
            return;
        }
        self.settle();
    }
    fn outcome(&self) -> Option<Outcome> {
        if self.removed[0] >= 2 {
            return Some(Outcome::Win(Player::One));
        }
        if self.removed[1] >= 2 {
            return Some(Outcome::Win(Player::Two));
        }
        let out_of_markers =
            !self.placing() && self.pending.is_none() && self.markers[self.side.idx()] == 0;
        let stuck = !self.placing() && self.pending.is_none() && {
            let p = self.side.idx() as u8 + 1;
            !G.cells()
                .into_iter()
                .any(|i| self.c[i] == p && !self.slides(i).is_empty())
        };
        if out_of_markers || stuck || self.ply >= 300 {
            return Some(match self.removed[0].cmp(&self.removed[1]) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                _ => Outcome::Draw,
            });
        }
        None
    }
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut s = (self.removed[me] as i32 - self.removed[1 - me] as i32) * 500;
        let mk = [3u8, 4];
        for i in G.cells() {
            let k = self.c[i];
            if k == 3 || k == 4 {
                for d in 0..3 {
                    let mut n = 1;
                    let mut cur = i;
                    while let Some(x) = G.step(cur, d) {
                        if self.c[x] == k {
                            n += 1;
                            cur = x;
                        } else {
                            break;
                        }
                    }
                    let w = [0, 1, 6, 30, 200][n.min(4)];
                    s += if k == mk[me] { w } else { -w };
                }
            }
        }
        s
    }
    fn render(&self) -> String {
        let mut s = G.draw(|i| match self.c[i] {
            1 => 'A',
            2 => 'B',
            3 => 'x',
            4 => 'o',
            _ => '.',
        });
        s.push_str("A/B = inelele J1/J2, x/o = semnele J1/J2.\n");
        s.push_str(&format!(
            "Inele scoase: J1 {}, J2 {}; semne rămase: J1 {}, J2 {}\n",
            self.removed[0], self.removed[1], self.markers[0], self.markers[1]
        ));
        if self.pending.is_some() {
            s.push_str("Rând format: scoate un inel propriu (x <celulă>).\n");
        }
        s
    }
    fn format_move(&self, m: &Mv) -> String {
        match m {
            Mv::Ring(i) => format!("r {}", G.label(*i as usize)),
            Mv::Slide(f, t) => format!("{}>{}", G.label(*f as usize), G.label(*t as usize)),
            Mv::Remove(i) => format!("x {}", G.label(*i as usize)),
        }
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
    use crate::b1_util::testing::check;
    #[test]
    fn placement_then_slide_flips() {
        let mut g = Inele::new();
        for _ in 0..6 {
            let m = g.legal_moves()[0].clone();
            g.apply(&m);
        }
        assert!(!g.placing());
        let mut h = Inele::new();
        h.rings_left = [0, 0];
        h.c[G.idx(0, 0)] = 1;
        h.c[G.idx(1, 0)] = 4; // semn adversar
        h.c[G.idx(2, 0)] = 0;
        h.c[G.idx(-4, 4)] = 2;
        h.apply(&Mv::Slide(G.idx(0, 0) as u8, G.idx(2, 0) as u8));
        assert_eq!(h.c[G.idx(1, 0)], 3); // întors
        assert_eq!(h.c[G.idx(0, 0)], 3); // semn lăsat
        assert_eq!(h.c[G.idx(2, 0)], 1);
    }
    #[test]
    fn row_of_four_removes_and_asks_ring() {
        let mut h = Inele::new();
        h.rings_left = [0, 0];
        for q in -3..=0 {
            h.c[G.idx(q, 0)] = 3;
        }
        h.c[G.idx(2, 2)] = 1;
        h.c[G.idx(4, -4)] = 2;
        h.orig = Player::One;
        h.settle();
        assert_eq!(h.pending, Some(Player::One));
        let mv = h.legal_moves();
        assert_eq!(mv, vec![Mv::Remove(G.idx(2, 2) as u8)]);
        h.apply(&mv[0]);
        assert_eq!(h.removed[0], 1);
        assert_eq!(h.to_move(), Player::Two);
    }
    #[test]
    fn random_and_ai() {
        check(|_| Inele::new(), 300);
    }
}
