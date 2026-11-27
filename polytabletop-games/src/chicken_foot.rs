//! Chicken Foot (domino „laba de găină”), 2 jucători, din regulile publice.
//!
//! * Set dublu-12 (91 de piese). Se joacă 13 runde; runda `r` pornește cu dublul `12-r` ca „motor”
//!   (12-12, 11-11, …, 0-0), pus în mijloc. Fiecare jucător ia 15 piese, restul e stocul.
//! * Se așază o piesă cu o jumătate egală cu numărul unui capăt liber (`5-7@2` = piesa 5-7 la capătul 2;
//!   capetele sunt numerotate în afișare). Cine nu are piesă potrivită trage din stoc (`trage`, una câte una,
//!   până poate juca); cu stocul gol și fără piesă spune `pas`.
//! * **Dublurile** (și motorul) trebuie „satisfăcute”: după ce a fost așezat un dublu, următoarele trei
//!   piese trebuie puse pe cele trei laturi libere ale lui (laba) — de oricare jucător — înainte ca vreo
//!   piesă să se poată pune în altă parte. Motorul are patru laturi libere și cere trei piese; a patra
//!   rămâne apoi capăt liber obișnuit.
//! * Runda se termină când un jucător rămâne fără piese sau când ambii dau pas la rând (blocaj). Fiecare
//!   adaugă la scorul său punctele rămase în mână (câștigătorul rundei adaugă 0). După 13 runde câștigă cel cu
//!   scorul total **mai mic** (egal = remiză).
//!
//! Mutări: `5-7@2`, `trage`, `pas`, `urmatoarea`. Informație ascunsă: vezi `p3_comun`.
#![allow(
    clippy::needless_range_loop,
    clippy::collapsible_if,
    clippy::collapsible_match,
    clippy::manual_is_multiple_of,
    clippy::manual_range_patterns,
    clippy::type_complexity
)]

use crate::cards::{handover_banner, shuffle, Viewer};
use crate::p3_comun::{honest_eval, Honest};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

type Tile = (u8, u8);
const ROUNDS: u8 = 13;
const HAND: usize = 15;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Play,
    HandOver,
    Over,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Place(Tile, u8),
    Draw,
    Pass,
    Next,
}

#[derive(Clone)]
pub struct ChickenFoot {
    rng: Rng,
    boneyard: Vec<Tile>,
    hands: [Vec<Tile>; 2],
    ends: Vec<(u8, u32)>,
    pending: Vec<(u32, u8)>,
    next_group: u32,
    placed: Vec<Tile>,
    round: u8,
    totals: [i32; 2],
    add: [i32; 2],
    side: Player,
    passes: u8,
    phase: Phase,
    fresh: [u8; 2],
    prev: Player,
    log: String,
    viewer: Viewer,
}

impl Default for ChickenFoot {
    fn default() -> Self {
        Self::new()
    }
}

fn pips(t: Tile) -> i32 {
    (t.0 + t.1) as i32
}

fn tt(t: Tile) -> String {
    format!("{}-{}", t.0, t.1)
}

fn join(v: &[Tile]) -> String {
    if v.is_empty() {
        return "—".into();
    }
    v.iter().map(|&t| tt(t)).collect::<Vec<_>>().join(" ")
}

fn all_tiles() -> Vec<Tile> {
    let mut v = Vec::new();
    for a in 0..=12u8 {
        for b in a..=12u8 {
            v.push((a, b));
        }
    }
    v
}

impl ChickenFoot {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = ChickenFoot {
            rng: Rng::new(seed),
            boneyard: Vec::new(),
            hands: [Vec::new(), Vec::new()],
            ends: Vec::new(),
            pending: Vec::new(),
            next_group: 1,
            placed: Vec::new(),
            round: 0,
            totals: [0, 0],
            add: [0, 0],
            side: Player::One,
            passes: 0,
            phase: Phase::Play,
            fresh: [0, 0],
            prev: Player::One,
            log: String::new(),
            viewer: Viewer::Fixed(Player::One),
        };
        g.deal();
        g
    }

    pub fn totals(&self) -> (i32, i32) {
        (self.totals[0], self.totals[1])
    }

    pub fn round(&self) -> u8 {
        self.round
    }

    fn engine(&self) -> u8 {
        12 - self.round
    }

    fn deal(&mut self) {
        let d = self.engine();
        let mut set = all_tiles();
        set.retain(|&t| t != (d, d));
        shuffle(&mut set, &mut self.rng);
        self.hands = [Vec::new(), Vec::new()];
        for _ in 0..HAND {
            for p in 0..2 {
                self.hands[p].push(set.pop().unwrap());
            }
        }
        self.boneyard = set;
        self.placed = vec![(d, d)];
        self.next_group = 2;
        self.ends = vec![(d, 1); 4];
        self.pending = vec![(1, 3)];
        self.add = [0, 0];
        self.passes = 0;
        self.fresh = [0, 0];
        for h in &mut self.hands {
            h.sort_unstable();
        }
        self.side = if self.round % 2 == 0 {
            Player::One
        } else {
            Player::Two
        };
        self.phase = Phase::Play;
    }

    fn allowed_ends(&self) -> Vec<usize> {
        let g = self.pending.last().map(|&(g, _)| g).unwrap_or(0);
        (0..self.ends.len())
            .filter(|&i| self.ends[i].1 == g)
            .collect()
    }

    fn end_round(&mut self, why: &str) {
        for p in 0..2 {
            self.totals[p] += self.add[p];
        }
        self.log = format!(
            "{}: puncte adăugate J1 +{}, J2 +{}",
            why, self.add[0], self.add[1]
        );
        self.phase = if self.round + 1 >= ROUNDS {
            Phase::Over
        } else {
            Phase::HandOver
        };
    }
}

impl Game for ChickenFoot {
    type Move = Move;

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = match who {
            Some(p) => Viewer::Fixed(p),
            None => Viewer::HotSeat,
        };
    }

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        match self.phase {
            Phase::Over => Vec::new(),
            Phase::HandOver => vec![Move::Next],
            Phase::Play => {
                let mut v = Vec::new();
                let allowed = self.allowed_ends();
                for &t in &self.hands[self.side.idx()] {
                    for &e in &allowed {
                        let val = self.ends[e].0;
                        if t.0 == val || t.1 == val {
                            v.push(Move::Place(t, e as u8 + 1));
                        }
                    }
                }
                if v.is_empty() {
                    if self.boneyard.is_empty() {
                        v.push(Move::Pass);
                    } else {
                        v.push(Move::Draw);
                    }
                }
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        self.fresh = [0, 0];
        self.prev = self.side;
        let p = self.side;
        match *m {
            Move::Next => {
                self.round += 1;
                self.deal();
            }
            Move::Draw => {
                let t = self.boneyard.pop().unwrap();
                self.hands[p.idx()].push(t);
                self.fresh[p.idx()] = 1;
                self.passes = 0;
            }
            Move::Pass => {
                self.passes += 1;
                if self.passes >= 2 {
                    for q in 0..2 {
                        self.add[q] = self.hands[q].iter().map(|&t| pips(t)).sum();
                    }
                    self.end_round("Blocaj");
                } else {
                    self.side = p.other();
                }
            }
            Move::Place(t, e) => {
                let pos = self.hands[p.idx()].iter().position(|&x| x == t).unwrap();
                self.hands[p.idx()].remove(pos);
                self.placed.push(t);
                self.passes = 0;
                let e = e as usize - 1;
                let (v, g) = self.ends[e];
                if t.0 == t.1 {
                    self.ends.remove(e);
                } else {
                    let other = if t.0 == v { t.1 } else { t.0 };
                    self.ends[e] = (other, 0);
                }
                if g > 0 {
                    if let Some(top) = self.pending.last_mut() {
                        top.1 -= 1;
                        if top.1 == 0 {
                            let gid = top.0;
                            self.pending.pop();
                            for en in &mut self.ends {
                                if en.1 == gid {
                                    en.1 = 0;
                                }
                            }
                        }
                    }
                }
                if t.0 == t.1 {
                    let gid = self.next_group;
                    self.next_group += 1;
                    for _ in 0..3 {
                        self.ends.push((v, gid));
                    }
                    self.pending.push((gid, 3));
                }
                if self.hands[p.idx()].is_empty() {
                    self.add[p.other().idx()] =
                        self.hands[p.other().idx()].iter().map(|&t| pips(t)).sum();
                    self.end_round(&format!("J{} a rămas fără piese", p.idx() + 1));
                } else {
                    self.side = p.other();
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        Some(match self.totals[0].cmp(&self.totals[1]) {
            std::cmp::Ordering::Less => Outcome::Win(Player::One),
            std::cmp::Ordering::Greater => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        if self.phase == Phase::Over {
            return 0;
        }
        honest_eval(self, p, 12)
    }

    fn render(&self) -> String {
        let mut s = String::new();
        if self.prev != self.side && self.phase == Phase::Play {
            if let Viewer::HotSeat = self.viewer {
                s.push_str(&handover_banner(self.side));
            }
        }
        s.push_str(&format!(
            "Chicken Foot — runda {}/{} (motor {}-{}) | scor total (mai mic = mai bine): J1 {}, J2 {}\n",
            self.round + 1,
            ROUNDS,
            self.engine(),
            self.engine(),
            self.totals[0],
            self.totals[1]
        ));
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        match self.phase {
            Phase::Over => {
                s.push_str("Joc încheiat.\n");
                return s;
            }
            Phase::HandOver => {
                s.push_str("Scrie «urmatoarea».\n");
                return s;
            }
            Phase::Play => {}
        }
        let allowed = self.allowed_ends();
        let list: Vec<String> = self
            .ends
            .iter()
            .enumerate()
            .map(|(i, &(v, g))| {
                format!(
                    "{}:{}{}{}",
                    i + 1,
                    v,
                    if g > 0 { "*" } else { "" },
                    if allowed.contains(&i) { "" } else { "(blocat)" }
                )
            })
            .collect();
        s.push_str(&format!("Capete: {}\n", list.join("  ")));
        if let Some(&(_, r)) = self.pending.last() {
            s.push_str(&format!(
                "Dublu de satisfăcut: mai trebuie {} piese pe latura lui (*).\n",
                r
            ));
        }
        s.push_str(&format!("Stoc: {} piese\n", self.boneyard.len()));
        let v = self.viewer.who(self.side);
        s.push_str(&format!(
            "Mâna ta (J{}): {}\n",
            v.idx() + 1,
            join(&self.hands[v.idx()])
        ));
        s.push_str(&format!(
            "J{} are {} piese\n",
            v.other().idx() + 1,
            self.hands[v.other().idx()].len()
        ));
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Place(t, e) => format!("{}@{}", tt(*t), e),
            Move::Draw => "trage".into(),
            Move::Pass => "pas".into(),
            Move::Next => "urmatoarea".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

impl Honest for ChickenFoot {
    fn determinize(&self, me: Player, rng: &mut Rng) -> Self {
        let mut g = self.clone();
        let (mi, oi) = (me.idx(), 1 - me.idx());
        let nf = (self.fresh[mi] as usize).min(self.hands[mi].len());
        let known_mine: Vec<Tile> = self.hands[mi][..self.hands[mi].len() - nf].to_vec();
        let mut pool = all_tiles();
        pool.retain(|t| !known_mine.contains(t) && !self.placed.contains(t));
        shuffle(&mut pool, rng);
        let mut mine = known_mine;
        for _ in 0..nf {
            mine.push(pool.pop().unwrap());
        }
        let n_opp = self.hands[oi].len();
        g.hands[mi] = mine;
        g.hands[oi] = (0..n_opp).map(|_| pool.pop().unwrap()).collect();
        g.boneyard = pool;
        g.fresh = [0, 0];
        g.rng = Rng::new(rng.next_u64());
        g
    }

    fn seed_state(&self, me: Player) -> Self {
        let mut c = self.clone();
        let nf = (c.fresh[me.idx()] as usize).min(c.hands[me.idx()].len());
        let k = c.hands[me.idx()].len() - nf;
        c.hands[me.idx()].truncate(k);
        c.fresh = [0, 0];
        c
    }

    fn forget_fresh(&mut self) {
        self.fresh = [0, 0];
    }

    fn rollout_done(&self) -> bool {
        self.phase != Phase::Play
    }

    fn rollout_value(&self, p: Player) -> f64 {
        let q = p.other();
        if self.phase == Phase::Play {
            let sp: i32 = self.hands[p.idx()].iter().map(|&t| pips(t)).sum();
            let sq: i32 = self.hands[q.idx()].iter().map(|&t| pips(t)).sum();
            return (sq - sp) as f64 / 10.0;
        }
        (self.add[q.idx()] - self.add[p.idx()]) as f64 / 10.0
    }

    fn policy(&self, rng: &mut Rng) -> Move {
        let mv = self.legal_moves();
        if mv.len() == 1 || rng.below(100) < 10 {
            return mv[rng.below(mv.len())];
        }
        let mut best = mv[0];
        let mut bs = i32::MIN;
        for &m in &mv {
            if let Move::Place(t, _) = m {
                let sc = pips(t) + if t.0 == t.1 { 4 } else { 0 };
                if sc > bs {
                    bs = sc;
                    best = m;
                }
            }
        }
        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deal_and_engine() {
        let g = ChickenFoot::with_seed(1);
        assert_eq!(g.hands[0].len(), 15);
        assert_eq!(g.hands[1].len(), 15);
        assert_eq!(g.boneyard.len(), 91 - 1 - 30);
        assert_eq!(g.ends.len(), 4);
        assert!(g.ends.iter().all(|&(v, _)| v == 12));
        assert_eq!(g.pending, vec![(1, 3)]);
        assert!(!g.hands[0].contains(&(12, 12)) && !g.boneyard.contains(&(12, 12)));
    }

    #[test]
    fn engine_needs_three_then_fourth_frees() {
        let mut g = ChickenFoot::with_seed(2);
        g.hands[0] = vec![(1, 12), (2, 12), (3, 12), (4, 12), (5, 5)];
        g.hands[1] = vec![(6, 12), (0, 1)];
        g.side = Player::One;
        g.apply(&Move::Place((1, 12), 1));
        g.apply(&Move::Place((6, 12), 2));
        assert_eq!(g.pending, vec![(1, 1)]);
        // blocat: capetele libere (valoarea 1 și 6) nu se pot folosi încă
        g.side = Player::One;
        let moves = g.legal_moves();
        assert!(moves
            .iter()
            .all(|m| matches!(m, Move::Place((_, b), e) if *b == 12 && (*e == 3 || *e == 4))));
        g.apply(&Move::Place((2, 12), 3));
        assert!(g.pending.is_empty());
        // al patrulea capăt al motorului e acum liber
        assert!(g.ends.iter().any(|&(v, gr)| v == 12 && gr == 0));
    }

    #[test]
    fn double_creates_three_toes() {
        let mut g = ChickenFoot::with_seed(3);
        g.pending.clear();
        g.ends = vec![(5, 0), (9, 0)];
        g.hands[0] = vec![(5, 5), (5, 7)];
        g.hands[1] = vec![(0, 1)];
        g.side = Player::One;
        g.apply(&Move::Place((5, 5), 1));
        assert_eq!(g.ends.len(), 4);
        assert_eq!(g.pending.last().unwrap().1, 3);
        g.side = Player::Two;
        // J2 nu poate pune în altă parte decât pe laba dublului
        let m = g.legal_moves();
        assert!(m == vec![Move::Draw] || m == vec![Move::Pass]);
    }

    #[test]
    fn going_out_scores_other_hand_and_lower_total_wins() {
        let mut g = ChickenFoot::with_seed(4);
        g.pending.clear();
        g.ends = vec![(5, 0)];
        g.hands[0] = vec![(5, 6)];
        g.hands[1] = vec![(3, 4), (0, 12)];
        g.side = Player::One;
        g.apply(&Move::Place((5, 6), 1));
        assert_eq!(g.totals, [0, 7 + 12]);
        assert_eq!(g.phase, Phase::HandOver);
        g.round = ROUNDS - 1;
        g.phase = Phase::Play;
        g.totals = [10, 30];
        g.ends = vec![(1, 0)];
        g.pending.clear();
        g.hands = [vec![(1, 2)], vec![(0, 0)]];
        g.side = Player::One;
        g.apply(&Move::Place((1, 2), 1));
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn block_after_two_passes() {
        let mut g = ChickenFoot::with_seed(5);
        g.boneyard.clear();
        g.pending.clear();
        g.ends = vec![(0, 0)];
        g.hands = [vec![(5, 6)], vec![(7, 8)]];
        g.side = Player::One;
        assert_eq!(g.legal_moves(), vec![Move::Pass]);
        g.apply(&Move::Pass);
        g.apply(&Move::Pass);
        assert_eq!(g.totals, [11, 15]);
    }

    #[test]
    fn determinize_conserves_tiles() {
        let g = ChickenFoot::with_seed(6);
        let mut rng = Rng::new(1);
        let d = g.determinize(Player::One, &mut rng);
        assert_eq!(d.hands[0], g.hands[0]);
        assert_eq!(d.hands[1].len(), 15);
        assert_eq!(d.boneyard.len(), g.boneyard.len());
        let mut all: Vec<Tile> = d.hands[0]
            .iter()
            .chain(&d.hands[1])
            .chain(&d.boneyard)
            .chain(&d.placed)
            .copied()
            .collect();
        all.sort_unstable();
        all.dedup();
        assert_eq!(all.len(), 91);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=4u64 {
            let mut g = ChickenFoot::with_seed(seed);
            let mut rng = Rng::new(seed + 3);
            let mut n = 0;
            while g.outcome().is_none() && n < 30_000 {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed} after {n}");
        }
    }
}

#[cfg(test)]
mod a1_blind {
    use super::*;

    #[test]
    fn ai_ignores_hidden_information_at_all_levels() {
        crate::ai_audit_a1::audit_honest("chicken_foot", ChickenFoot::with_seed, 1..=2, 11, 60);
    }
}
