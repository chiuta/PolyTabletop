//! Familia „grilă”: fiecare jucător își umple propria tablă cu piese luate pe rând dintr-o
//! ofertă comună (draft); scorul vine din reguli de aranjare proprii fiecărui joc.
//! Variante originale, inspirate doar de mecanica generală „așază și punctează”.
//! Zarul/sacul sunt re-însămânțate la `clone()` (vezi `w7_util::Dice`): AI-ul nu poate
//! „vedea” piesele viitoare.
#![allow(clippy::needless_range_loop, clippy::manual_is_multiple_of)]

use crate::w7_util::Dice;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone, Copy, Debug)]
pub enum Term {
    /// grupuri conexe (4 vecini) de aceeași piesă, cu cel puțin `min` piese: `per` puncte/piesă
    Groups { min: u8, per: i32 },
    /// rând sau coloană complet dintr-o singură piesă
    Lines(i32),
    /// rând sau coloană care conține toate felurile de piese
    Variety(i32),
    /// pereche de vecini diferiți
    Contrast(i32),
    /// pereche de vecini la fel (orizontal/vertical)
    Pairs(i32),
    /// pereche în diagonală la fel
    Diag(i32),
    /// seturi complete (câte un exemplar din fiecare fel) pe întreaga tablă
    Sets(i32),
    /// cele patru colțuri la fel
    Corners(i32),
    /// bonus pentru fiecare fel în care ai mai multe piese decât adversarul
    Majority(i32),
    /// piesele de felul 0 valorează atâta, în plus
    Prize(i32),
    /// piesele de pe margine, de același fel cu cele din centru
    Frame(i32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Place {
    Free,
    /// dacă ai un loc liber lângă o piesă proprie, trebuie să așezi acolo
    Adjacent,
    /// se zidește din margine spre centru: lângă marginea tablei sau lângă o piesă proprie
    Frame,
}

pub struct Spec {
    pub id: &'static str,
    pub w: usize,
    pub h: usize,
    pub kinds: u8,
    pub offer: usize,
    pub place: Place,
    pub terms: &'static [Term],
}

use Term::*;

pub const SPECS: &[Spec] = &[
    Spec {
        id: "plapuma_cal",
        w: 4,
        h: 4,
        kinds: 4,
        offer: 3,
        place: Place::Free,
        terms: &[Groups { min: 3, per: 2 }, Contrast(1)],
    },
    Spec {
        id: "poteci_munte",
        w: 4,
        h: 4,
        kinds: 4,
        offer: 3,
        place: Place::Frame,
        terms: &[Variety(4), Pairs(1)],
    },
    Spec {
        id: "poiana_ursului",
        w: 5,
        h: 4,
        kinds: 3,
        offer: 3,
        place: Place::Adjacent,
        terms: &[Groups { min: 2, per: 1 }, Lines(5)],
    },
    Spec {
        id: "strada_case",
        w: 5,
        h: 4,
        kinds: 5,
        offer: 4,
        place: Place::Free,
        terms: &[Sets(6), Diag(1)],
    },
    Spec {
        id: "exploratori",
        w: 4,
        h: 4,
        kinds: 4,
        offer: 3,
        place: Place::Frame,
        terms: &[Corners(8), Pairs(1), Majority(3)],
    },
    Spec {
        id: "orasul_comun",
        w: 4,
        h: 4,
        kinds: 3,
        offer: 4,
        place: Place::Free,
        terms: &[Majority(4), Groups { min: 2, per: 1 }],
    },
    Spec {
        id: "atlas_pierdut",
        w: 5,
        h: 5,
        kinds: 4,
        offer: 3,
        place: Place::Free,
        terms: &[Lines(6), Sets(3), Contrast(1)],
    },
    Spec {
        id: "parc_dino",
        w: 4,
        h: 3,
        kinds: 6,
        offer: 4,
        place: Place::Free,
        terms: &[Variety(5), Majority(2), Pairs(1)],
    },
    Spec {
        id: "asezari_noi",
        w: 5,
        h: 5,
        kinds: 4,
        offer: 3,
        place: Place::Adjacent,
        terms: &[Groups { min: 4, per: 2 }, Frame(1)],
    },
    Spec {
        id: "curtea_reginei",
        w: 5,
        h: 5,
        kinds: 5,
        offer: 4,
        place: Place::Adjacent,
        terms: &[Groups { min: 2, per: 1 }, Prize(2), Corners(5)],
    },
    Spec {
        id: "sat_mic",
        w: 4,
        h: 4,
        kinds: 5,
        offer: 5,
        place: Place::Free,
        terms: &[Groups { min: 3, per: 3 }, Sets(4)],
    },
    Spec {
        id: "turnul_cifrelor",
        w: 4,
        h: 5,
        kinds: 5,
        offer: 3,
        place: Place::Frame,
        terms: &[Diag(2), Lines(6), Contrast(1)],
    },
    Spec {
        id: "pavilion_vesel",
        w: 5,
        h: 4,
        kinds: 5,
        offer: 5,
        place: Place::Free,
        terms: &[Variety(3), Pairs(1), Majority(2)],
    },
    Spec {
        id: "oras_straturi",
        w: 4,
        h: 4,
        kinds: 4,
        offer: 4,
        place: Place::Free,
        terms: &[Groups { min: 2, per: 1 }, Variety(3), Majority(2)],
    },
    Spec {
        id: "padurea_spiridusilor",
        w: 4,
        h: 5,
        kinds: 4,
        offer: 3,
        place: Place::Adjacent,
        terms: &[Contrast(2), Diag(1)],
    },
    Spec {
        id: "pictura_manastire",
        w: 5,
        h: 4,
        kinds: 4,
        offer: 4,
        place: Place::Frame,
        terms: &[Sets(5), Lines(4), Prize(1)],
    },
    Spec {
        id: "vanatorii_campie",
        w: 5,
        h: 4,
        kinds: 4,
        offer: 4,
        place: Place::Adjacent,
        terms: &[Groups { min: 3, per: 2 }, Majority(3), Prize(1)],
    },
    Spec {
        id: "gradina_corali",
        w: 4,
        h: 4,
        kinds: 3,
        offer: 3,
        place: Place::Free,
        terms: &[Groups { min: 3, per: 3 }, Pairs(1), Frame(1)],
    },
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GMove {
    pub offer: u8,
    pub cell: u8,
}

#[derive(Clone)]
pub struct GridGame {
    spec: &'static Spec,
    boards: [Vec<u8>; 2],
    offer: Vec<u8>,
    side: Player,
    dice: Dice,
}

const EMPTY: u8 = 255;

impl GridGame {
    pub fn new(spec: &'static Spec) -> Self {
        Self::with_seed(spec, Rng::from_clock().next_u64())
    }

    pub fn with_seed(spec: &'static Spec, seed: u64) -> Self {
        let mut dice = Dice::new(seed);
        let n = spec.w * spec.h;
        let offer = (0..spec.offer)
            .map(|_| dice.below(spec.kinds as usize) as u8)
            .collect();
        GridGame {
            spec,
            boards: [vec![EMPTY; n], vec![EMPTY; n]],
            offer,
            side: Player::One,
            dice,
        }
    }

    fn nbrs(&self, c: usize) -> Vec<usize> {
        let (w, h) = (self.spec.w, self.spec.h);
        let (x, y) = (c % w, c / w);
        let mut v = Vec::with_capacity(4);
        if x > 0 {
            v.push(c - 1);
        }
        if x + 1 < w {
            v.push(c + 1);
        }
        if y > 0 {
            v.push(c - w);
        }
        if y + 1 < h {
            v.push(c + w);
        }
        v
    }

    fn cells_for(&self, p: Player) -> Vec<usize> {
        let b = &self.boards[p.idx()];
        let (w, h) = (self.spec.w, self.spec.h);
        let empty: Vec<usize> = (0..b.len()).filter(|&c| b[c] == EMPTY).collect();
        match self.spec.place {
            Place::Free => empty,
            Place::Adjacent => {
                let adj: Vec<usize> = empty
                    .iter()
                    .copied()
                    .filter(|&c| self.nbrs(c).iter().any(|&n| b[n] != EMPTY))
                    .collect();
                if adj.is_empty() {
                    empty
                } else {
                    adj
                }
            }
            Place::Frame => {
                let ok: Vec<usize> = empty
                    .iter()
                    .copied()
                    .filter(|&c| {
                        let (x, y) = (c % w, c / w);
                        x == 0
                            || y == 0
                            || x + 1 == w
                            || y + 1 == h
                            || self.nbrs(c).iter().any(|&n| b[n] != EMPTY)
                    })
                    .collect();
                if ok.is_empty() {
                    empty
                } else {
                    ok
                }
            }
        }
    }

    fn counts(&self, b: &[u8]) -> Vec<i32> {
        let mut c = vec![0; self.spec.kinds as usize];
        for &x in b {
            if x != EMPTY {
                c[x as usize] += 1;
            }
        }
        c
    }

    pub fn score(&self, p: Player) -> i32 {
        let b = &self.boards[p.idx()];
        let o = &self.boards[p.other().idx()];
        let (w, h, k) = (self.spec.w, self.spec.h, self.spec.kinds as usize);
        let mut total = 0;
        for t in self.spec.terms {
            total += match *t {
                Groups { min, per } => {
                    let mut seen = vec![false; b.len()];
                    let mut s = 0;
                    for st in 0..b.len() {
                        if seen[st] || b[st] == EMPTY {
                            continue;
                        }
                        let kind = b[st];
                        let mut stack = vec![st];
                        seen[st] = true;
                        let mut size = 0;
                        while let Some(c) = stack.pop() {
                            size += 1;
                            for n in self.nbrs(c) {
                                if !seen[n] && b[n] == kind {
                                    seen[n] = true;
                                    stack.push(n);
                                }
                            }
                        }
                        if size >= min as i32 {
                            s += per * size;
                        }
                    }
                    s
                }
                Lines(pts) => {
                    let mut s = 0;
                    for y in 0..h {
                        let v = b[y * w];
                        if v != EMPTY && (0..w).all(|x| b[y * w + x] == v) {
                            s += pts;
                        }
                    }
                    for x in 0..w {
                        let v = b[x];
                        if v != EMPTY && (0..h).all(|y| b[y * w + x] == v) {
                            s += pts;
                        }
                    }
                    s
                }
                Variety(pts) => {
                    let mut s = 0;
                    for y in 0..h {
                        let mut seen = vec![false; k];
                        for x in 0..w {
                            if b[y * w + x] != EMPTY {
                                seen[b[y * w + x] as usize] = true;
                            }
                        }
                        if seen.iter().all(|&z| z) {
                            s += pts;
                        }
                    }
                    for x in 0..w {
                        let mut seen = vec![false; k];
                        for y in 0..h {
                            if b[y * w + x] != EMPTY {
                                seen[b[y * w + x] as usize] = true;
                            }
                        }
                        if seen.iter().all(|&z| z) {
                            s += pts;
                        }
                    }
                    s
                }
                Contrast(pts) | Pairs(pts) => {
                    let same = matches!(t, Pairs(_));
                    let mut s = 0;
                    for c in 0..b.len() {
                        if b[c] == EMPTY {
                            continue;
                        }
                        let (x, y) = (c % w, c / w);
                        let mut chk = |n: usize| {
                            if b[n] != EMPTY && (b[n] == b[c]) == same {
                                s += pts;
                            }
                        };
                        if x + 1 < w {
                            chk(c + 1);
                        }
                        if y + 1 < h {
                            chk(c + w);
                        }
                    }
                    s
                }
                Diag(pts) => {
                    let mut s = 0;
                    for y in 0..h.saturating_sub(1) {
                        for x in 0..w {
                            let c = y * w + x;
                            if b[c] == EMPTY {
                                continue;
                            }
                            if x + 1 < w && b[c + w + 1] == b[c] {
                                s += pts;
                            }
                            if x > 0 && b[c + w - 1] == b[c] {
                                s += pts;
                            }
                        }
                    }
                    s
                }
                Sets(pts) => self.counts(b).into_iter().min().unwrap_or(0) * pts,
                Corners(pts) => {
                    let cs = [b[0], b[w - 1], b[(h - 1) * w], b[h * w - 1]];
                    if cs[0] != EMPTY && cs.iter().all(|&x| x == cs[0]) {
                        pts
                    } else {
                        0
                    }
                }
                Majority(pts) => {
                    let (a, c) = (self.counts(b), self.counts(o));
                    (0..k).filter(|&i| a[i] > c[i]).count() as i32 * pts
                }
                Prize(pts) => self.counts(b)[0] * pts,
                Frame(pts) => {
                    let mut s = 0;
                    for c in 0..b.len() {
                        let (x, y) = (c % w, c / w);
                        if b[c] != EMPTY
                            && (x == 0 || y == 0 || x + 1 == w || y + 1 == h)
                            && b[c] == 0
                        {
                            s += pts;
                        }
                    }
                    s
                }
            };
        }
        total
    }

    fn full(&self, p: Player) -> bool {
        self.boards[p.idx()].iter().all(|&x| x != EMPTY)
    }
}

fn glyph(v: u8) -> char {
    if v == EMPTY {
        '.'
    } else {
        (b'A' + v) as char
    }
}

impl Game for GridGame {
    type Move = GMove;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<GMove> {
        if self.outcome().is_some() {
            return vec![];
        }
        let cells = self.cells_for(self.side);
        let mut out = Vec::new();
        let mut seen_kind = Vec::new();
        for (i, &k) in self.offer.iter().enumerate() {
            if seen_kind.contains(&k) {
                continue;
            }
            seen_kind.push(k);
            for &c in &cells {
                out.push(GMove {
                    offer: i as u8,
                    cell: c as u8,
                });
            }
        }
        out
    }

    fn apply(&mut self, m: &GMove) {
        let k = self.offer[m.offer as usize];
        self.boards[self.side.idx()][m.cell as usize] = k;
        self.offer[m.offer as usize] = self.dice.below(self.spec.kinds as usize) as u8;
        self.side = self.side.other();
        if self.full(self.side) && !self.full(self.side.other()) {
            self.side = self.side.other();
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !(self.full(Player::One) && self.full(Player::Two)) {
            return None;
        }
        let (a, b) = (self.score(Player::One), self.score(Player::Two));
        Some(match a.cmp(&b) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            _ => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        self.score(p) - self.score(p.other())
    }

    fn render(&self) -> String {
        let (w, h) = (self.spec.w, self.spec.h);
        let mut s = String::new();
        for p in [Player::One, Player::Two] {
            s.push_str(&format!(
                "Jucătorul {} — scor {}{}\n",
                p.idx() + 1,
                self.score(p),
                if p == self.side { "  <- la rând" } else { "" }
            ));
            for y in (0..h).rev() {
                s.push_str(&format!("{} ", y + 1));
                for x in 0..w {
                    s.push(glyph(self.boards[p.idx()][y * w + x]));
                    s.push(' ');
                }
                s.push('\n');
            }
            s.push_str("  ");
            for x in 0..w {
                s.push((b'a' + x as u8) as char);
                s.push(' ');
            }
            s.push('\n');
        }
        s.push_str("Oferta: ");
        for (i, &k) in self.offer.iter().enumerate() {
            s.push_str(&format!("{}={} ", i + 1, glyph(k)));
        }
        s.push_str("\nMutare: număr_ofertă + celulă, ex. «1 b3» (în tabla ta).\n");
        s
    }

    fn format_move(&self, m: &GMove) -> String {
        let (w, c) = (self.spec.w, m.cell as usize);
        format!(
            "{} {}{}",
            m.offer + 1,
            (b'a' + (c % w) as u8) as char,
            c / w + 1
        )
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(2)
    }
}

pub fn make(id: &str) -> Box<dyn polytabletop_core::GameObj> {
    let spec = SPECS.iter().find(|s| s.id == id).expect("spec");
    Box::new(GridGame::new(spec))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_games_end_and_moves_are_legal() {
        for spec in SPECS {
            for seed in 1..=8u64 {
                let mut g = GridGame::with_seed(spec, seed);
                let mut rng = Rng::new(seed);
                let mut n = 0;
                while g.outcome().is_none() {
                    let mv = g.legal_moves();
                    assert!(!mv.is_empty(), "{}", spec.id);
                    let m = mv[rng.below(mv.len())];
                    let t = g.format_move(&m);
                    let before = g.boards.clone();
                    g.apply(&m);
                    assert_ne!(before, g.boards, "{t}");
                    n += 1;
                    assert!(n <= 2 * spec.w * spec.h);
                }
            }
        }
    }

    #[test]
    fn ai_uses_only_legal_moves_and_beats_random_often() {
        polytabletop_core::ai::set_time_budget_override_ms(Some(5));
        let mut wins = 0;
        for spec in SPECS {
            let mut g = GridGame::with_seed(spec, 3);
            let mut rng = Rng::new(9);
            while g.outcome().is_none() {
                let m = if g.side == Player::One {
                    let t = polytabletop_core::GameObj::ai_move(&g, rng.next_u64()).unwrap();
                    assert!(polytabletop_core::GameObj::legal_moves(&g).contains(&t));
                    polytabletop_core::GameObj::play(&mut g, &t).unwrap();
                    continue;
                } else {
                    let mv = g.legal_moves();
                    mv[rng.below(mv.len())]
                };
                g.apply(&m);
            }
            if g.outcome() == Some(Outcome::Win(Player::One)) {
                wins += 1;
            }
        }
        assert!(wins >= SPECS.len() * 2 / 3, "{wins}");
    }

    #[test]
    fn every_spec_has_unique_id() {
        let mut ids: Vec<_> = SPECS.iter().map(|s| s.id).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), SPECS.len());
    }
}
