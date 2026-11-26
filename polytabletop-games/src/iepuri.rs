//! Iepurii și Elefantul (variantă proprie de forță și împingere pe tablă 8×8): fiecare jucător are
//! 16 piese: elefant (5), cămilă (4), 2 cai (3), 2 câini (2), 2 pisici (1), 8 iepuri (0). O tură
//! are până la 4 pași (fiecare pas e o mutare separată; `fin` încheie tura după cel puțin
//! un pas). Pas simplu: o piesă merge pe o celulă vecină ortogonală liberă (iepurii nu merg
//! înapoi). Împingere (2 pași): o piesă proprie mai puternică intră pe celula unei piese
//! adverse mai slabe, care e împinsă pe o altă celulă vecină liberă. Tragere (2 pași): o piesă
//! proprie face un pas pe o celulă liberă și trage după ea o piesă adversă mai slabă aflată
//! lângă locul ei de plecare. O piesă lângă o piesă adversă mai puternică și fără vreun
//! prieten vecin e înghețată (nu poate face pași proprii). O piesă pe una din capcanele c3,
//! f3, c6, f6 fără prieten vecin e scoasă. Câștigi dacă un iepure ajunge pe rândul de capăt
//! al adversarului, dacă adversarul rămâne fără iepuri sau fără mutări. Remiză după 400 de pași.
//! Mutări: `e2n` pas (n, s, e, v), `d4n>e` împingere (piesa inamică merge spre e), `d4n<e4` trage
//! piesa de pe e4, `fin`.
#![allow(clippy::all)]

use crate::b1_util::{grid, sq};
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: i32 = 8;
const D: [(char, i32, i32); 4] = [('n', 0, 1), ('s', 0, -1), ('e', 1, 0), ('v', -1, 0)];
const TRAPS: [usize; 4] = [18, 21, 42, 45];
const SYM: [char; 6] = ['r', 'c', 'd', 'h', 'm', 'e'];

#[derive(Clone, PartialEq, Debug)]
pub enum Mv {
    Step(u8, u8),
    Push(u8, u8, u8),
    Pull(u8, u8, u8), // de la, direcția pasului, celula trasă
    End,
}

#[derive(Clone)]
pub struct Iepuri {
    b: [u8; 64], // 0 gol, owner<<3 | (rang+1)
    side: Player,
    steps: u8,
    total: u32,
    win: Option<Outcome>,
}

impl Default for Iepuri {
    fn default() -> Self {
        Self::new()
    }
}

fn own(p: u8) -> usize {
    (p >> 3) as usize
}
fn rank(p: u8) -> u8 {
    (p & 7) - 1
}

impl Iepuri {
    pub fn new() -> Self {
        let mut b = [0u8; 64];
        let back = [2u8, 3, 4, 6, 5, 4, 3, 2]; // pisică, câine, cal, cămilă, elefant, cal, câine, pisică (rang+1)
        let back2 = [2u8, 3, 4, 5, 6, 4, 3, 2];
        for x in 0..8 {
            b[x] = back[x];
            b[8 + x] = 1;
            b[56 + x] = 8 | back2[x];
            b[48 + x] = 8 | 1;
        }
        Iepuri {
            b,
            side: Player::One,
            steps: 4,
            total: 0,
            win: None,
        }
    }
    fn nb(i: usize, d: usize) -> Option<usize> {
        let (nx, ny) = ((i % 8) as i32 + D[d].1, (i / 8) as i32 + D[d].2);
        (nx >= 0 && ny >= 0 && nx < N && ny < N).then(|| (ny * N + nx) as usize)
    }
    fn frozen(&self, i: usize) -> bool {
        let p = self.b[i];
        let mut stronger = false;
        for d in 0..4 {
            if let Some(n) = Self::nb(i, d) {
                let q = self.b[n];
                if q != 0 {
                    if own(q) == own(p) {
                        return false;
                    }
                    if rank(q) > rank(p) {
                        stronger = true;
                    }
                }
            }
        }
        stronger
    }
    fn settle_traps(&mut self) {
        for &t in &TRAPS {
            let p = self.b[t];
            if p == 0 {
                continue;
            }
            let friend = (0..4).any(|d| {
                Self::nb(t, d).is_some_and(|n| self.b[n] != 0 && own(self.b[n]) == own(p))
            });
            if !friend {
                self.b[t] = 0;
            }
        }
    }
    fn check_end(&mut self) {
        let me = self.side.idx();
        let goal = |who: usize| -> bool {
            let row = if who == 0 { 7 } else { 0 };
            (0..8).any(|x| self.b[row * 8 + x] == ((who as u8) << 3 | 1))
        };
        let rabbits = |who: usize| {
            self.b
                .iter()
                .filter(|&&p| p == ((who as u8) << 3 | 1))
                .count()
        };
        if goal(me) {
            self.win = Some(Outcome::Win(self.side));
        } else if goal(1 - me) {
            self.win = Some(Outcome::Win(self.side.other()));
        } else if rabbits(me) == 0 {
            self.win = Some(Outcome::Win(self.side.other()));
        } else if rabbits(1 - me) == 0 {
            self.win = Some(Outcome::Win(self.side));
        }
    }
}

impl Game for Iepuri {
    type Move = Mv;
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<Mv> {
        if self.win.is_some() {
            return vec![];
        }
        if self.total >= 400 {
            return vec![];
        }
        let me = self.side.idx();
        let mut v = vec![];
        for i in 0..64 {
            let p = self.b[i];
            if p == 0 || own(p) != me {
                continue;
            }
            let fr = self.frozen(i);
            for d in 0..4 {
                let Some(t) = Self::nb(i, d) else { continue };
                let back = rank(p) == 0 && ((me == 0 && D[d].2 < 0) || (me == 1 && D[d].2 > 0));
                if self.b[t] == 0 {
                    if !fr && !back {
                        v.push(Mv::Step(i as u8, d as u8));
                    }
                    // tragere: o piesă adversă mai slabă vecină cu i
                    if !fr && !back && self.steps >= 2 {
                        for e in 0..4 {
                            if let Some(c) = Self::nb(i, e) {
                                let q = self.b[c];
                                if q != 0 && own(q) != me && rank(q) < rank(p) && c != t {
                                    v.push(Mv::Pull(i as u8, d as u8, c as u8));
                                }
                            }
                        }
                    }
                } else if own(self.b[t]) != me
                    && rank(self.b[t]) < rank(p)
                    && !fr
                    && self.steps >= 2
                {
                    // împingere: piesa adversă merge pe o celulă vecină liberă
                    for e in 0..4 {
                        if let Some(u) = Self::nb(t, e) {
                            if self.b[u] == 0 {
                                v.push(Mv::Push(i as u8, d as u8, e as u8));
                            }
                        }
                    }
                }
            }
        }
        if self.steps < 4 {
            v.push(Mv::End);
        }
        if v.is_empty() {
            v.push(Mv::End);
        }
        v
    }
    fn apply(&mut self, m: &Mv) {
        let mut used = 1;
        match m {
            Mv::Step(i, d) => {
                let t = Self::nb(*i as usize, *d as usize).unwrap();
                self.b[t] = self.b[*i as usize];
                self.b[*i as usize] = 0;
            }
            Mv::Push(i, d, e) => {
                let t = Self::nb(*i as usize, *d as usize).unwrap();
                let u = Self::nb(t, *e as usize).unwrap();
                self.b[u] = self.b[t];
                self.b[t] = self.b[*i as usize];
                self.b[*i as usize] = 0;
                used = 2;
            }
            Mv::Pull(i, d, c) => {
                let t = Self::nb(*i as usize, *d as usize).unwrap();
                self.b[t] = self.b[*i as usize];
                self.b[*i as usize] = self.b[*c as usize];
                self.b[*c as usize] = 0;
                used = 2;
            }
            Mv::End => {
                self.side = self.side.other();
                self.steps = 4;
                self.total += 1;
                // mutator: dacă noul jucător nu are mutări, pierde
                if self.win.is_none() && self.total < 400 {
                    let n = self.legal_moves();
                    if n.len() == 1 && n[0] == Mv::End && self.steps == 4 {
                        self.win = Some(Outcome::Win(self.side.other()));
                    }
                }
                return;
            }
        }
        self.total += used as u32;
        self.steps -= used;
        self.settle_traps();
        // câștig imediat: ținta e verificată din punctul de vedere al celui la mutare
        self.check_end();
        if self.win.is_none() && self.steps == 0 {
            self.apply(&Mv::End);
        }
    }
    fn outcome(&self) -> Option<Outcome> {
        if self.win.is_some() {
            return self.win;
        }
        if self.total >= 400 {
            return Some(Outcome::Draw);
        }
        None
    }
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut s = 0;
        for i in 0..64 {
            let q = self.b[i];
            if q == 0 {
                continue;
            }
            let o = own(q);
            let r = rank(q) as i32;
            let mut v = 20 + r * r * 6;
            if r == 0 {
                let adv = if o == 0 {
                    (i / 8) as i32
                } else {
                    7 - (i / 8) as i32
                };
                v += adv * adv * 3;
            }
            s += if o == me { v } else { -v };
        }
        s
    }
    fn render(&self) -> String {
        let mut s = grid(8, 8, |x, y| {
            let i = y * 8 + x;
            let p = self.b[i];
            if p == 0 {
                if TRAPS.contains(&i) {
                    '+'
                } else {
                    '.'
                }
            } else {
                let c = SYM[rank(p) as usize];
                if own(p) == 0 {
                    c.to_ascii_uppercase()
                } else {
                    c
                }
            }
        });
        s.push_str(
            "r iepure, c pisică, d câine, h cal, m cămilă, e elefant; MAJUSCULE = J1; + capcană.\n",
        );
        s.push_str(&format!("Pași rămași în tură: {}\n", self.steps));
        s
    }
    fn format_move(&self, m: &Mv) -> String {
        let nm = |i: u8| sq(i as usize % 8, i as usize / 8);
        match m {
            Mv::Step(i, d) => format!("{}{}", nm(*i), D[*d as usize].0),
            Mv::Push(i, d, e) => format!("{}{}>{}", nm(*i), D[*d as usize].0, D[*e as usize].0),
            Mv::Pull(i, d, c) => format!("{}{}<{}", nm(*i), D[*d as usize].0, nm(*c)),
            Mv::End => "fin".into(),
        }
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 500,
            max_depth: 4,
        }
    }
    fn move_score(&self, m: &Mv) -> i32 {
        match m {
            Mv::Push(..) | Mv::Pull(..) => 3,
            Mv::Step(..) => 1,
            Mv::End => -5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::b1_util::testing::check;
    fn play(g: &mut Iepuri, s: &str) {
        let m = g
            .legal_moves()
            .into_iter()
            .find(|m| g.format_move(m) == s)
            .unwrap_or_else(|| panic!("ilegal {s}"));
        g.apply(&m);
    }
    #[test]
    fn steps_turn_and_freeze() {
        let mut g = Iepuri::new();
        play(&mut g, "a2n");
        assert_eq!(g.to_move(), Player::One);
        play(&mut g, "b2n");
        play(&mut g, "c2n");
        play(&mut g, "d2n");
        assert_eq!(g.to_move(), Player::Two);
        // iepurele nu merge înapoi
        let mut h = Iepuri::new();
        h.b = [0; 64];
        h.b[20] = 1; // iepure J1 pe e? idx 20 = e3
        h.b[63] = 8 | 6;
        h.b[7] = 6;
        assert!(!h.legal_moves().iter().any(|m| h.format_move(m) == "e3s"));
        // pisica adversară înghețată de elefant
        h.b[28] = 8 | 2; // pisică J2 pe e4... rank 1
        h.b[27] = 6; // elefant J1 lângă ea
        assert!(h.frozen(28));
        assert!(!h
            .legal_moves()
            .iter()
            .any(|m| matches!(m, Mv::Step(f, _) if *f == 28)));
    }
    #[test]
    fn push_pull_and_trap() {
        let mut g = Iepuri::new();
        g.b = [0; 64];
        g.b[7] = 6; // elefant J1 h1
        g.b[63] = 8 | 6;
        g.b[0] = 1; // iepure
        g.b[19] = 5; // cămilă J1 d3 lângă capcană c3 (18)
        g.b[20] = 8 | 2; // pisică J2 e3
                         // împinge pisica: d3 -> e3, pisica merge spre n (e4)
        play(&mut g, "d3e>n");
        assert_eq!(g.b[28], 8 | 2);
        assert_eq!(g.b[20], 5);
        // capcană: piesă pe c3 fără prieten dispare
        let mut h = Iepuri::new();
        h.b = [0; 64];
        h.b[63] = 8 | 6;
        h.b[7] = 6;
        h.b[0] = 1;
        h.b[19] = 3; // câine d3
        play(&mut h, "d3v");
        assert_eq!(h.b[18], 0);
    }
    #[test]
    fn random_and_ai() {
        check(|_| Iepuri::new(), 800);
    }
}
