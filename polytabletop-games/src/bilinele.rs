//! Bile și inele (variantă proprie de bile colorate pe inele care dispar): tablă hexagonală cu
//! 37 de inele (rază 3). Rezerva comună are 5 bile albe, 7 gri și 9 negre. O tură: dacă o
//! bilă a ta poate sări peste o bilă vecină pe un inel gol din spatele ei, săritura e
//! obligatorie (iei bila sărită; continuă cu aceeași bilă cât mai poți). Altfel pui o bilă la
//! alegere din rezervă (sau din capturile tale, când rezerva e goală) pe un inel gol și scoți
//! un inel gol „liber” (cel cu două laturi vecine consecutive lipsă sau de margine). Dacă
//! scoaterea izolează o zonă în care toate inelele rămase au bile, o iei cu bilele ei.
//! Câștigi cu 3 bile din fiecare culoare, sau 4 albe, 5 gri ori 6 negre (nu poți ține bile ale
//! adversarului). Cine nu mai poate muta pierde. Remiză după 200 de ture.
//! Celule etichetate rând a–g (a sus) + poziție 1–7. Mutări: `w d4 x a1` (bilă albă pe d4, scoate
//! inelul a1; culori: w, g, b), `d4xd6` sau `d4xd6xd8` (salt/lanț de sărituri).
#![allow(clippy::all)]

use crate::b1_util::{Hex, HEXD};
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const G: Hex = Hex { rad: 3 };

#[derive(Clone, PartialEq, Debug)]
pub enum Mv {
    Place(u8, u8, u8, u8), // culoare 0..2, celula, inel scos (255 = niciunul), -
    Jump(Vec<u8>),         // lanț de celule
}

#[derive(Clone)]
pub struct Bilinele {
    ring: [bool; 49],
    marble: [u8; 49], // 0 gol, 1..3 culoare+1
    pool: [u8; 3],
    cap: [[u8; 3]; 2],
    side: Player,
    ply: u32,
    stuck: Option<Player>,
}

impl Default for Bilinele {
    fn default() -> Self {
        Self::new()
    }
}

impl Bilinele {
    pub fn new() -> Self {
        let mut ring = [false; 49];
        for i in G.cells() {
            ring[i] = true;
        }
        Bilinele {
            ring,
            marble: [0; 49],
            pool: [5, 7, 9],
            cap: [[0; 3]; 2],
            side: Player::One,
            ply: 0,
            stuck: None,
        }
    }
    fn free_ring(&self, i: usize) -> bool {
        if !self.ring[i] || self.marble[i] != 0 {
            return false;
        }
        let gone = |d: usize| G.step(i, d % 6).is_none_or(|n| !self.ring[n]);
        (0..6).any(|d| gone(d) && gone(d + 1))
    }
    fn jumps_from(&self, i: usize) -> Vec<usize> {
        let mut v = vec![];
        for d in 0..6 {
            if let Some(a) = G.step(i, d) {
                if self.marble[a] != 0 {
                    if let Some(b) = G.step(a, d) {
                        if self.ring[b] && self.marble[b] == 0 {
                            v.push(b);
                        }
                    }
                }
            }
        }
        v
    }
    fn any_jump(&self) -> bool {
        G.cells()
            .into_iter()
            .any(|i| self.marble[i] != 0 && !self.jumps_from(i).is_empty())
    }
    fn chains(&self, from: usize, path: &mut Vec<u8>, out: &mut Vec<Vec<u8>>) {
        let js = self.jumps_from(from);
        if js.is_empty() {
            out.push(path.clone());
            return;
        }
        for b in js {
            // marble jumped is captured in a copy
            let mut c = self.clone();
            let d = (0..6)
                .find(|&d| G.step(from, d).and_then(|a| G.step(a, d)) == Some(b))
                .unwrap();
            let a = G.step(from, d).unwrap();
            c.marble[b] = c.marble[from];
            c.marble[from] = 0;
            c.marble[a] = 0;
            path.push(b as u8);
            c.chains(b, path, out);
            path.pop();
        }
    }
    fn isolate(&mut self, who: usize) {
        // componente conexe de inele; dacă toate inelele componentei au bile (și nu e toată tabla), le ia
        let mut seen = [false; 49];
        let cells = G.cells();
        let comps: Vec<Vec<usize>> = cells
            .iter()
            .filter(|&&i| self.ring[i])
            .copied()
            .collect::<Vec<_>>()
            .into_iter()
            .filter_map(|i| {
                if seen[i] {
                    return None;
                }
                let mut comp = vec![i];
                seen[i] = true;
                let mut k = 0;
                while k < comp.len() {
                    let c = comp[k];
                    k += 1;
                    for d in 0..6 {
                        if let Some(n) = G.step(c, d) {
                            if self.ring[n] && !seen[n] {
                                seen[n] = true;
                                comp.push(n);
                            }
                        }
                    }
                }
                Some(comp)
            })
            .collect();
        if comps.len() < 2 {
            return;
        }
        for comp in comps {
            if comp.iter().all(|&c| self.marble[c] != 0) {
                for &c in &comp {
                    self.cap[who][self.marble[c] as usize - 1] += 1;
                    self.marble[c] = 0;
                    self.ring[c] = false;
                }
            }
        }
    }
    fn won(&self, p: usize) -> bool {
        let c = self.cap[p];
        c.iter().all(|&n| n >= 3) || c[0] >= 4 || c[1] >= 5 || c[2] >= 6
    }
}

impl Game for Bilinele {
    type Move = Mv;
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<Mv> {
        if self.won(0) || self.won(1) || self.ply >= 200 || self.stuck.is_some() {
            return vec![];
        }
        let p = self.side.idx();
        let mut v = vec![];
        if self.any_jump() {
            for i in G.cells() {
                if self.marble[i] == 0 || self.jumps_from(i).is_empty() {
                    continue;
                }
                let mut out = vec![];
                self.chains(i, &mut vec![i as u8], &mut out);
                for o in out {
                    v.push(Mv::Jump(o));
                }
            }
            return v;
        }
        let pool_empty = self.pool.iter().all(|&n| n == 0);
        for col in 0..3u8 {
            let avail = if pool_empty {
                self.cap[p][col as usize] > 0
            } else {
                self.pool[col as usize] > 0
            };
            if !avail {
                continue;
            }
            for i in G.cells() {
                if !self.ring[i] || self.marble[i] != 0 {
                    continue;
                }
                // inel de scos: orice inel gol liber (după punere, i nu mai e liber)
                let mut any = false;
                for r in G.cells() {
                    if r != i && self.free_ring(r) {
                        v.push(Mv::Place(col, i as u8, r as u8, 0));
                        any = true;
                    }
                }
                if !any {
                    v.push(Mv::Place(col, i as u8, 255, 0));
                }
            }
        }
        v
    }
    fn apply(&mut self, m: &Mv) {
        let p = self.side.idx();
        match m {
            Mv::Place(col, i, r, _) => {
                if self.pool.iter().all(|&n| n == 0) {
                    self.cap[p][*col as usize] -= 1;
                } else {
                    self.pool[*col as usize] -= 1;
                }
                self.marble[*i as usize] = col + 1;
                if *r != 255 {
                    self.ring[*r as usize] = false;
                    self.isolate(p);
                }
            }
            Mv::Jump(path) => {
                let mut cur = path[0] as usize;
                for &n in &path[1..] {
                    let n = n as usize;
                    let d = (0..6)
                        .find(|&d| G.step(cur, d).and_then(|a| G.step(a, d)) == Some(n))
                        .unwrap();
                    let a = G.step(cur, d).unwrap();
                    self.cap[p][self.marble[a] as usize - 1] += 1;
                    self.marble[n] = self.marble[cur];
                    self.marble[cur] = 0;
                    self.marble[a] = 0;
                    cur = n;
                }
            }
        }
        self.ply += 1;
        self.side = self.side.other();
        if !self.won(0) && !self.won(1) && self.legal_moves().is_empty() {
            self.stuck = Some(self.side);
        }
    }
    fn outcome(&self) -> Option<Outcome> {
        if self.won(0) {
            return Some(Outcome::Win(Player::One));
        }
        if self.won(1) {
            return Some(Outcome::Win(Player::Two));
        }
        if let Some(s) = self.stuck {
            return Some(Outcome::Win(s.other()));
        }
        if self.ply >= 200 {
            return Some(Outcome::Draw);
        }
        None
    }
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let score = |q: usize| -> i32 {
            let c = self.cap[q];
            let sets = c.iter().map(|&n| n.min(3) as i32).sum::<i32>();
            sets * 30 + c[0] as i32 * 20 + c[1] as i32 * 12 + c[2] as i32 * 8
        };
        score(me) - score(1 - me)
    }
    fn render(&self) -> String {
        let mut s = G.draw(|i| {
            if !self.ring[i] {
                ' '
            } else {
                ['o', 'w', 'g', 'b'][self.marble[i] as usize]
            }
        });
        s.push_str("o = inel gol, w/g/b = bilă albă/gri/neagră; inelele scoase nu se mai văd.\n");
        s.push_str(&format!(
            "Rezervă: w{} g{} b{}\n",
            self.pool[0], self.pool[1], self.pool[2]
        ));
        for p in 0..2 {
            s.push_str(&format!(
                "Capturi J{}: w{} g{} b{}\n",
                p + 1,
                self.cap[p][0],
                self.cap[p][1],
                self.cap[p][2]
            ));
        }
        s
    }
    fn format_move(&self, m: &Mv) -> String {
        match m {
            Mv::Place(c, i, r, _) => {
                let col = ['w', 'g', 'b'][*c as usize];
                if *r == 255 {
                    format!("{} {}", col, G.label(*i as usize))
                } else {
                    format!(
                        "{} {} x {}",
                        col,
                        G.label(*i as usize),
                        G.label(*r as usize)
                    )
                }
            }
            Mv::Jump(path) => path
                .iter()
                .map(|&c| G.label(c as usize))
                .collect::<Vec<_>>()
                .join("x"),
        }
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 400,
            max_depth: 3,
        }
    }
}

const _: [(i32, i32); 6] = HEXD;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::b1_util::testing::check;
    #[test]
    fn free_rings_and_jump() {
        let g = Bilinele::new();
        // doar inelele de margine sunt libere la început (colțurile și laturile)
        assert!(g.free_ring(G.idx(-3, 0)));
        assert!(!g.free_ring(G.idx(0, 0)));
        let mut h = Bilinele::new();
        h.marble[G.idx(0, 0)] = 1;
        h.marble[G.idx(1, 0)] = 2;
        assert!(h.any_jump());
        let mv = h.legal_moves();
        assert!(mv.iter().all(|m| matches!(m, Mv::Jump(_))));
        h.apply(&mv[0]);
        assert_eq!(h.marble.iter().filter(|&&v| v != 0).count(), 1);
    }
    #[test]
    fn random_and_ai() {
        check(|_| Bilinele::new(), 300);
    }
}
