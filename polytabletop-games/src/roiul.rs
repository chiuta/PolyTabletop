//! Roiul (variantă proprie de piese-insecte pe tablă hexagonală liberă, fără tablă fixă):
//! fiecare jucător are 9 piese: 1 regină (R), 2 gândaci (G), 2 păianjeni (P), 2 lăcuste (L)
//! și 2 furnici (F).
//! Prima piesă se pune oricum, a doua lângă ea; apoi o piesă nouă trebuie să atingă doar
//! piese proprii (privind piesa de deasupra). Regina trebuie pusă cel târziu la a patra tură
//! a ta; până nu e pusă, nu poți muta piese. Mișcările nu pot rupe roiul în două bucăți și
//! piesele alunecă pe margine (nu se strecoară printre două piese care se ating).
//! Regina: 1 pas; gândacul: 1 pas, poate urca pe alte piese; păianjenul: exact 3 pași;
//! lăcusta: sare în linie dreaptă peste una sau mai multe piese pe prima celulă goală;
//! furnica: oricâți pași pe margine. Câștigă cine înconjoară regina adversarului pe toate
//! cele șase laturi (ambele deodată = remiză). Remiză după 200 de semi-mutări.
//! Coordonate axiale `q,r`: vecinii sunt (+1,0) (+1,-1) (0,-1) (-1,0) (-1,+1) (0,+1).
//! Mutări: `+R@0,0` (pune), `0,0>1,-1` (mută), `pas`.
#![allow(clippy::all)]

use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

type Pos = (i8, i8);
const NB: [Pos; 6] = [(1, 0), (1, -1), (0, -1), (-1, 0), (-1, 1), (0, 1)];
const TY: [char; 5] = ['R', 'G', 'P', 'L', 'F'];
const START: [u8; 5] = [1, 2, 2, 2, 2];

#[derive(Clone, PartialEq, Debug)]
pub enum Mv {
    Place(u8, Pos),
    Move(Pos, Pos),
    Pass,
}

#[derive(Clone)]
pub struct Roiul {
    b: BTreeMap<Pos, Vec<u8>>, // piesă = owner<<3 | tip
    hand: [[u8; 5]; 2],
    turns: [u8; 2],
    side: Player,
    ply: u32,
}

impl Default for Roiul {
    fn default() -> Self {
        Self::new()
    }
}

fn add(a: Pos, d: Pos) -> Pos {
    (a.0 + d.0, a.1 + d.1)
}
fn own(p: u8) -> usize {
    (p >> 3) as usize
}
fn ty(p: u8) -> usize {
    (p & 7) as usize
}

impl Roiul {
    pub fn new() -> Self {
        Roiul {
            b: BTreeMap::new(),
            hand: [START; 2],
            turns: [0; 2],
            side: Player::One,
            ply: 0,
        }
    }
    fn occ(b: &BTreeMap<Pos, Vec<u8>>, p: Pos) -> bool {
        b.contains_key(&p)
    }
    fn connected(b: &BTreeMap<Pos, Vec<u8>>) -> bool {
        let Some(&start) = b.keys().next() else {
            return true;
        };
        let mut seen = BTreeSet::new();
        let mut q = VecDeque::new();
        seen.insert(start);
        q.push_back(start);
        while let Some(c) = q.pop_front() {
            for d in NB {
                let n = add(c, d);
                if b.contains_key(&n) && seen.insert(n) {
                    q.push_back(n);
                }
            }
        }
        seen.len() == b.len()
    }
    fn can_slide(b: &BTreeMap<Pos, Vec<u8>>, a: Pos, to: Pos) -> bool {
        // a și to sunt vecine; to e goală
        let di = NB.iter().position(|&d| add(a, d) == to).unwrap();
        let c1 = add(a, NB[(di + 5) % 6]);
        let c2 = add(a, NB[(di + 1) % 6]);
        let (o1, o2) = (Self::occ(b, c1), Self::occ(b, c2));
        o1 != o2
    }
    fn dests(&self, from: Pos) -> Vec<Pos> {
        let piece = *self.b[&from].last().unwrap();
        let mut b = self.b.clone();
        {
            let st = b.get_mut(&from).unwrap();
            st.pop();
            if st.is_empty() {
                b.remove(&from);
            }
        }
        let stack_left = b.contains_key(&from);
        if !stack_left && !Self::connected(&b) {
            return vec![];
        }
        let mut out = BTreeSet::new();
        match ty(piece) {
            0 => {
                for d in NB {
                    let t = add(from, d);
                    if !Self::occ(&b, t) && Self::can_slide(&b, from, t) {
                        out.insert(t);
                    }
                }
            }
            1 => {
                for d in NB {
                    let t = add(from, d);
                    if Self::occ(&b, t) || stack_left {
                        out.insert(t);
                    } else if Self::can_slide(&b, from, t) {
                        out.insert(t);
                    }
                }
            }
            2 => {
                fn dfs(
                    b: &BTreeMap<Pos, Vec<u8>>,
                    from: Pos,
                    cur: Pos,
                    path: &mut Vec<Pos>,
                    out: &mut BTreeSet<Pos>,
                ) {
                    if path.len() == 3 {
                        out.insert(cur);
                        return;
                    }
                    for d in NB {
                        let t = add(cur, d);
                        if b.contains_key(&t) || t == from || path.contains(&t) {
                            continue;
                        }
                        // aluneca: păstrăm atingerea cu roiul (fără piesa de pe `from`)
                        if Roiul::can_slide(b, cur, t) {
                            path.push(t);
                            dfs(b, from, t, path, out);
                            path.pop();
                        }
                    }
                }
                let mut path = vec![];
                dfs(&b, from, from, &mut path, &mut out);
            }
            3 => {
                for d in NB {
                    let mut t = add(from, d);
                    if !Self::occ(&b, t) {
                        continue;
                    }
                    while Self::occ(&b, t) {
                        t = add(t, d);
                    }
                    out.insert(t);
                }
            }
            _ => {
                let mut seen = BTreeSet::new();
                let mut q = VecDeque::new();
                seen.insert(from);
                q.push_back(from);
                while let Some(c) = q.pop_front() {
                    for d in NB {
                        let t = add(c, d);
                        if !Self::occ(&b, t) && !seen.contains(&t) && Self::can_slide(&b, c, t) {
                            seen.insert(t);
                            out.insert(t);
                            q.push_back(t);
                        }
                    }
                }
            }
        }
        out.remove(&from);
        out.into_iter().collect()
    }
    fn queen_pos(&self, who: usize) -> Option<Pos> {
        self.b
            .iter()
            .find(|(_, st)| st.contains(&((who as u8) << 3)))
            .map(|(p, _)| *p)
    }
    fn surrounded(&self, who: usize) -> bool {
        self.queen_pos(who)
            .is_some_and(|q| NB.iter().all(|&d| self.b.contains_key(&add(q, d))))
    }
    fn free_neighbors(&self, who: usize) -> i32 {
        self.queen_pos(who).map_or(6, |q| {
            NB.iter()
                .filter(|&&d| !self.b.contains_key(&add(q, d)))
                .count() as i32
        })
    }
}

impl Game for Roiul {
    type Move = Mv;
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<Mv> {
        if self.surrounded(0) || self.surrounded(1) || self.ply >= 200 {
            return vec![];
        }
        let p = self.side.idx();
        let mut v = vec![];
        // puneri
        let mut cells: BTreeSet<Pos> = BTreeSet::new();
        if self.b.is_empty() {
            cells.insert((0, 0));
        } else if self.turns[p] == 0 {
            for pos in self.b.keys() {
                for d in NB {
                    let t = add(*pos, d);
                    if !self.b.contains_key(&t) {
                        cells.insert(t);
                    }
                }
            }
        } else {
            for pos in self.b.keys() {
                for d in NB {
                    let t = add(*pos, d);
                    if self.b.contains_key(&t) || cells.contains(&t) {
                        continue;
                    }
                    let ok = NB.iter().all(|&e| {
                        self.b
                            .get(&add(t, e))
                            .is_none_or(|st| own(*st.last().unwrap()) == p)
                    });
                    if ok {
                        cells.insert(t);
                    }
                }
            }
        }
        let queen_placed = self.hand[p][0] == 0;
        let must_queen = !queen_placed && self.turns[p] >= 3;
        for t in 0..5 {
            if self.hand[p][t] == 0 || (must_queen && t != 0) {
                continue;
            }
            for &c in &cells {
                v.push(Mv::Place(t as u8, c));
            }
        }
        if queen_placed {
            let mine: Vec<Pos> = self
                .b
                .iter()
                .filter(|(_, st)| own(*st.last().unwrap()) == p)
                .map(|(k, _)| *k)
                .collect();
            for f in mine {
                for t in self.dests(f) {
                    v.push(Mv::Move(f, t));
                }
            }
        }
        if v.is_empty() {
            v.push(Mv::Pass);
        }
        v
    }
    fn apply(&mut self, m: &Mv) {
        let p = self.side.idx();
        match m {
            Mv::Place(t, c) => {
                self.b.entry(*c).or_default().push((p as u8) << 3 | t);
                self.hand[p][*t as usize] -= 1;
                self.turns[p] += 1;
            }
            Mv::Move(f, t) => {
                let st = self.b.get_mut(f).unwrap();
                let piece = st.pop().unwrap();
                if st.is_empty() {
                    self.b.remove(f);
                }
                self.b.entry(*t).or_default().push(piece);
                self.turns[p] += 1;
            }
            Mv::Pass => {
                self.turns[p] += 1;
            }
        }
        self.ply += 1;
        self.side = self.side.other();
    }
    fn outcome(&self) -> Option<Outcome> {
        let (a, b) = (self.surrounded(0), self.surrounded(1));
        match (a, b) {
            (true, true) => Some(Outcome::Draw),
            (true, false) => Some(Outcome::Win(Player::Two)),
            (false, true) => Some(Outcome::Win(Player::One)),
            _ if self.ply >= 200 => Some(Outcome::Draw),
            _ => None,
        }
    }
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut s = (6 - self.free_neighbors(1 - me)) * 40 - (6 - self.free_neighbors(me)) * 45;
        // piese pe masă și libertate (piese mobile)
        for st in self.b.values() {
            let t = *st.last().unwrap();
            let v = match ty(t) {
                4 => 12,
                3 => 6,
                2 => 5,
                1 => 8,
                _ => 0,
            };
            s += if own(t) == me { v } else { -v };
        }
        s
    }
    fn render(&self) -> String {
        let mut s = String::new();
        if self.b.is_empty() {
            return "Roiul: masă goală; pune prima piesă pe 0,0 (ex. +R@0,0).\n".into();
        }
        let (qmin, qmax) = (
            self.b.keys().map(|k| k.0).min().unwrap() - 1,
            self.b.keys().map(|k| k.0).max().unwrap() + 1,
        );
        let (rmin, rmax) = (
            self.b.keys().map(|k| k.1).min().unwrap() - 1,
            self.b.keys().map(|k| k.1).max().unwrap() + 1,
        );
        s.push_str(&format!(
            "q de la {qmin} la {qmax}; fiecare rând r e deplasat cu o jumătate de celulă\n"
        ));
        for r in rmin..=rmax {
            s.push_str(&format!("r={r:>3} "));
            for _ in 0..(r - rmin) {
                s.push(' ');
            }
            for q in qmin..=qmax {
                match self.b.get(&(q, r)) {
                    Some(st) => {
                        let t = *st.last().unwrap();
                        let c = TY[ty(t)];
                        s.push(if own(t) == 0 {
                            c
                        } else {
                            c.to_ascii_lowercase()
                        });
                        s.push(if st.len() > 1 {
                            char::from(b'0' + st.len() as u8)
                        } else {
                            ' '
                        });
                    }
                    None => s.push_str(". "),
                }
            }
            s.push('\n');
        }
        s.push_str("Majuscule = J1, minuscule = J2; cifra = înălțimea stivei (gândaci).\n");
        for p in 0..2 {
            s.push_str(&format!("În mână J{}: ", p + 1));
            for t in 0..5 {
                if self.hand[p][t] > 0 {
                    s.push_str(&format!("{}×{} ", TY[t], self.hand[p][t]));
                }
            }
            s.push('\n');
        }
        s
    }
    fn format_move(&self, m: &Mv) -> String {
        match m {
            Mv::Place(t, c) => format!("+{}@{},{}", TY[*t as usize], c.0, c.1),
            Mv::Move(f, t) => format!("{},{}>{},{}", f.0, f.1, t.0, t.1),
            Mv::Pass => "pas".into(),
        }
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
    fn play(g: &mut Roiul, s: &str) {
        let m = g
            .legal_moves()
            .into_iter()
            .find(|m| g.format_move(m) == s)
            .unwrap_or_else(|| panic!("ilegal {s}"));
        g.apply(&m);
    }
    #[test]
    fn placement_rules() {
        let mut g = Roiul::new();
        assert_eq!(g.legal_moves().len(), 5);
        play(&mut g, "+R@0,0");
        assert!(g.legal_moves().iter().all(|m| matches!(m, Mv::Place(_, _))));
        play(&mut g, "+R@1,0");
        // J1: nu poate pune lângă piesa lui J2 (1,0) decât dacă nu o atinge
        let ms: Vec<String> = g.legal_moves().iter().map(|m| g.format_move(m)).collect();
        assert!(ms.contains(&"+G@-1,0".to_string()));
        assert!(!ms.contains(&"+G@2,0".to_string()));
    }
    #[test]
    fn grasshopper_and_one_hive() {
        let mut g = Roiul::new();
        g.b.insert((0, 0), vec![3]); // lăcustă J1
        g.b.insert((1, 0), vec![8 | 4]); // furnică J2
        g.b.insert((2, 0), vec![8 | 4]);
        g.hand[0][0] = 0;
        let d = g.dests((0, 0));
        assert_eq!(d, vec![(3, 0)]);
        // regina înconjurată câștigă
        let mut h = Roiul::new();
        h.b.insert((0, 0), vec![8]); // regina J2
        for (i, d) in NB.iter().enumerate() {
            h.b.insert(*d, vec![if i == 0 { 1 } else { 2 }]);
        }
        assert_eq!(h.outcome(), Some(Outcome::Win(Player::One)));
    }
    #[test]
    fn random_and_ai() {
        check(|_| Roiul::new(), 300);
    }
}
