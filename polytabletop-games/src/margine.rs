//! De la margine (variantă proprie de împingere pe tablă hexagonală cu 37 de celule): fiecare
//! jucător are 10 piese în rezervă. O tură: introduci o piesă din rezervă pe tablă dintr-un
//! punct de pe margine, într-o direcție; dacă prima celulă e ocupată, tot șirul continuu de
//! piese din acea linie e împins cu o celulă (mutarea e ilegală dacă șirul ar ieși de pe
//! tablă, adică linia e plină). După mutare, orice șir de 4 sau mai multe piese consecutive
//! ale aceleiași culori e scos: piesele lui se întorc la proprietar, iar piesele adverse
//! așezate imediat în prelungirea șirului în aceeași linie sunt luate definitiv. Întâi se
//! verifică șirurile celui care a mutat, apoi ale adversarului (în ordinea tablei). Pierzi
//! dacă la rândul tău nu mai ai piese în rezervă; după 200 de semi-mutări câștigă cine are
//! mai multe piese în rezervă (egal = remiză).
//! Celulele au etichete ca rândurile a–g (a sus), poziția în rând 1–7. Mutări: `a1>se`
//! (intri în celula a1 spre direcția se). Direcții: e, ne, nv, v, sv, se.
#![allow(clippy::all)]

use crate::b1_util::{Hex, HEXD, HEXN};
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const G: Hex = Hex { rad: 3 };

#[derive(Clone)]
pub struct Margine {
    b: [u8; 49],
    res: [u8; 2],
    side: Player,
    ply: u32,
}

impl Default for Margine {
    fn default() -> Self {
        Self::new()
    }
}

impl Margine {
    pub fn new() -> Self {
        Margine {
            b: [0; 49],
            res: [10, 10],
            side: Player::One,
            ply: 0,
        }
    }
    /// Intrări: (celula de margine, direcția) astfel încât celula dinaintea ei e în afara tablei.
    fn entries() -> Vec<(usize, usize)> {
        let mut v = vec![];
        for i in G.cells() {
            for d in 0..6 {
                let (q, r) = G.qr(i);
                if !G.valid(q - HEXD[d].0, r - HEXD[d].1) {
                    v.push((i, d));
                }
            }
        }
        v
    }
    fn chain_ok(&self, i: usize, d: usize) -> bool {
        let mut c = i;
        loop {
            if self.b[c] == 0 {
                return true;
            }
            match G.step(c, d) {
                Some(n) => c = n,
                None => return false,
            }
        }
    }
    fn clear_lines(&mut self, who: u8) {
        loop {
            let mut found = None;
            'outer: for i in G.cells() {
                if self.b[i] != who {
                    continue;
                }
                for d in 0..3 {
                    // începutul șirului: celula dinainte nu e `who`
                    let (q, r) = G.qr(i);
                    let prev = (q - HEXD[d].0, r - HEXD[d].1);
                    if G.valid(prev.0, prev.1) && self.b[G.idx(prev.0, prev.1)] == who {
                        continue;
                    }
                    let mut run = vec![i];
                    let mut c = i;
                    while let Some(n) = G.step(c, d) {
                        if self.b[n] == who {
                            run.push(n);
                            c = n;
                        } else {
                            break;
                        }
                    }
                    if run.len() >= 4 {
                        found = Some((run, d));
                        break 'outer;
                    }
                }
            }
            let Some((run, d)) = found else { return };
            // prelungiri
            let mut taken = vec![];
            let first = run[0];
            let (q, r) = G.qr(first);
            let (mut pq, mut pr) = (q - HEXD[d].0, r - HEXD[d].1);
            while G.valid(pq, pr) && self.b[G.idx(pq, pr)] == 3 - who {
                taken.push(G.idx(pq, pr));
                pq -= HEXD[d].0;
                pr -= HEXD[d].1;
            }
            let last = *run.last().unwrap();
            let mut c = last;
            while let Some(n) = G.step(c, d) {
                if self.b[n] == 3 - who {
                    taken.push(n);
                    c = n;
                } else {
                    break;
                }
            }
            for &c in &run {
                self.b[c] = 0;
            }
            self.res[who as usize - 1] += run.len() as u8;
            for &c in &taken {
                self.b[c] = 0;
            }
        }
    }
}

impl Game for Margine {
    type Move = (u8, u8);
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<(u8, u8)> {
        if self.outcome().is_some() {
            return vec![];
        }
        Self::entries()
            .into_iter()
            .filter(|&(i, d)| self.chain_ok(i, d))
            .map(|(i, d)| (i as u8, d as u8))
            .collect()
    }
    fn apply(&mut self, m: &(u8, u8)) {
        let me = self.side.idx() as u8 + 1;
        let (i, d) = (m.0 as usize, m.1 as usize);
        // deplasăm șirul
        let mut cells = vec![i];
        let mut c = i;
        while self.b[c] != 0 {
            c = G.step(c, d).unwrap();
            cells.push(c);
        }
        for k in (1..cells.len()).rev() {
            self.b[cells[k]] = self.b[cells[k - 1]];
        }
        self.b[i] = me;
        self.res[me as usize - 1] -= 1;
        self.clear_lines(me);
        self.clear_lines(3 - me);
        self.ply += 1;
        self.side = self.side.other();
    }
    fn outcome(&self) -> Option<Outcome> {
        let p = self.side.idx();
        if self.res[p] == 0 && self.ply > 0 {
            return Some(Outcome::Win(self.side.other()));
        }
        if self.ply >= 200 {
            return Some(match self.res[0].cmp(&self.res[1]) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                _ => Outcome::Draw,
            });
        }
        None
    }
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut s = (self.res[me] as i32 - self.res[1 - me] as i32) * 30;
        for i in G.cells() {
            let v = self.b[i];
            if v == 0 {
                continue;
            }
            for d in 0..3 {
                let mut n = 1;
                let mut c = i;
                while let Some(x) = G.step(c, d) {
                    if self.b[x] == v {
                        n += 1;
                        c = x;
                    } else {
                        break;
                    }
                }
                let w = [0, 0, 6, 18, 60][n.min(4)];
                s += if v as usize == me + 1 { w } else { -w };
            }
        }
        s
    }
    fn render(&self) -> String {
        let mut s = G.draw(|i| match self.b[i] {
            1 => 'X',
            2 => 'O',
            _ => '.',
        });
        s.push_str(&format!(
            "Rezervă: J1 {}, J2 {}\n",
            self.res[0], self.res[1]
        ));
        s
    }
    fn format_move(&self, m: &(u8, u8)) -> String {
        format!("{}>{}", G.label(m.0 as usize), HEXN[m.1 as usize])
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 400,
            max_depth: 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::b1_util::testing::check;
    #[test]
    fn entries_and_push() {
        let g = Margine::new();
        assert_eq!(Margine::entries().len(), 42);
        let mut h = g.clone();
        let m = h.legal_moves()[0];
        h.apply(&m);
        assert_eq!(h.res, [9, 10]);
        assert_eq!(h.b.iter().filter(|&&v| v == 1).count(), 1);
    }
    #[test]
    fn line_of_four_returns_pieces() {
        let mut g = Margine::new();
        // patru piese ale lui J1 pe rândul d (r=0), q -3..0 ; a cincea intră din vest
        for q in -3..=0 {
            g.b[G.idx(q, 0)] = 1;
        }
        g.res = [5, 10];
        // piesă adversă în prelungire la q=1
        g.b[G.idx(1, 0)] = 2;
        g.clear_lines(1);
        assert_eq!(g.res[0], 9);
        assert_eq!(g.b[G.idx(1, 0)], 0);
        assert!(g.b.iter().all(|&v| v == 0));
    }
    #[test]
    fn random_and_ai() {
        check(|_| Margine::new(), 300);
    }
}
