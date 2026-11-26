//! „CrazyEightsi” — Crazy Eights, 2 jucători, din regulile publice.
//!
//! * 52 de cărți, 7 fiecăruia, o carte de start pe masă (dacă e un 8, se reamestecă și se întoarce
//!   alta); restul formează stocul.
//! * Pui o carte de aceeași culoare sau aceeași valoare cu cea de sus. Optarul (8) se pune oricând și
//!   alege culoarea următoare (`8h:s` = optar de cupă, culoarea nouă pică).
//! * Fără carte potrivită tragi din stoc (`trage`, câte una) până poți juca; dacă ai o carte potrivită
//!   nu poți trage; cu stocul gol și fără carte potrivită spui `pas`. Fără reamestecarea aruncatelor.
//! * Mâna se termină când cineva rămâne fără cărți (ia punctele din mâna adversarului: 8 = 50,
//!   K Q J 10 = 10, A = 1, restul valoarea scrisă) sau când ambii dau pas la rând (blocaj: ia diferența
//!   cel cu mâna mai mică). Se joacă până la 100 de puncte.
//!
//! Mutări: `7h`, `8s:d`, `trage`, `pas`, `urmatoarea` (mâna următoare). Informație ascunsă: vezi `p3_comun`.
#![allow(
    clippy::needless_range_loop,
    clippy::collapsible_if,
    clippy::collapsible_match,
    clippy::manual_is_multiple_of,
    clippy::manual_range_patterns,
    clippy::type_complexity
)]

use crate::cards::{full_deck, handover_banner, join_cards, shuffle, Card, Viewer};
use crate::p3_comun::{honest_eval, Honest};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const TARGET: i32 = 100;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Play,
    HandOver,
    Over,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Play(Card, Option<u8>),
    Draw,
    Pass,
    Next,
}

#[derive(Clone)]
pub struct CrazyEights {
    rng: Rng,
    stock: Vec<Card>,
    discard: Vec<Card>,
    hands: [Vec<Card>; 2],
    suit: u8,
    side: Player,
    first: Player,
    scores: [i32; 2],
    gain: [i32; 2],
    passes: u8,
    phase: Phase,
    fresh: [u8; 2],
    prev: Player,
    log: String,
    viewer: Viewer,
}

impl Default for CrazyEights {
    fn default() -> Self {
        Self::new()
    }
}

fn card_value(c: Card) -> i32 {
    match c.rank {
        8 => 50,
        14 => 1,
        11..=13 | 10 => 10,
        r => r as i32,
    }
}

fn hand_value(h: &[Card]) -> i32 {
    h.iter().map(|&c| card_value(c)).sum()
}

impl CrazyEights {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = CrazyEights {
            rng: Rng::new(seed),
            stock: Vec::new(),
            discard: Vec::new(),
            hands: [Vec::new(), Vec::new()],
            suit: 0,
            side: Player::One,
            first: Player::One,
            scores: [0, 0],
            gain: [0, 0],
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

    pub fn scores(&self) -> (i32, i32) {
        (self.scores[0], self.scores[1])
    }

    fn deal(&mut self) {
        loop {
            let mut d = full_deck(2);
            shuffle(&mut d, &mut self.rng);
            self.hands = [Vec::new(), Vec::new()];
            for _ in 0..7 {
                for p in 0..2 {
                    self.hands[p].push(d.pop().unwrap());
                }
            }
            let up = d.pop().unwrap();
            if up.rank == 8 {
                continue;
            }
            self.discard = vec![up];
            self.suit = up.suit;
            self.stock = d;
            break;
        }
        for h in &mut self.hands {
            h.sort_by_key(|c| (c.suit, c.rank));
        }
        self.passes = 0;
        self.gain = [0, 0];
        self.fresh = [0, 0];
        self.phase = Phase::Play;
        self.side = self.first;
    }

    fn top(&self) -> Card {
        *self.discard.last().unwrap()
    }

    fn playable(&self, c: Card) -> bool {
        c.rank == 8 || c.suit == self.suit || c.rank == self.top().rank
    }

    fn finish(&mut self, winner: Player, pts: i32) {
        self.scores[winner.idx()] += pts;
        self.gain[winner.idx()] += pts;
        self.log = format!("J{} câștigă mâna: +{} puncte", winner.idx() + 1, pts);
        self.phase = if self.scores[winner.idx()] >= TARGET {
            Phase::Over
        } else {
            Phase::HandOver
        };
    }
}

impl Game for CrazyEights {
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
                let h = &self.hands[self.side.idx()];
                let mut v = Vec::new();
                for (i, &c) in h.iter().enumerate() {
                    if h[..i].contains(&c) || !self.playable(c) {
                        continue;
                    }
                    if c.rank == 8 {
                        for s in 0..4u8 {
                            v.push(Move::Play(c, Some(s)));
                        }
                    } else {
                        v.push(Move::Play(c, None));
                    }
                }
                if v.is_empty() {
                    if self.stock.is_empty() {
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
                self.first = self.first.other();
                self.deal();
            }
            Move::Draw => {
                let c = self.stock.pop().unwrap();
                self.hands[p.idx()].push(c);
                self.fresh[p.idx()] = 1;
                self.passes = 0;
                // dacă nu se poate juca nici cartea trasă și stocul s-a golit, urmează pas
            }
            Move::Pass => {
                self.passes += 1;
                if self.passes >= 2 {
                    let (a, b) = (hand_value(&self.hands[0]), hand_value(&self.hands[1]));
                    match a.cmp(&b) {
                        std::cmp::Ordering::Less => self.finish(Player::One, b - a),
                        std::cmp::Ordering::Greater => self.finish(Player::Two, a - b),
                        std::cmp::Ordering::Equal => {
                            self.log = "Blocaj egal: nimeni nu punctează".into();
                            self.phase = Phase::HandOver;
                        }
                    }
                    return;
                }
                self.side = p.other();
            }
            Move::Play(c, ns) => {
                let pos = self.hands[p.idx()].iter().position(|&x| x == c).unwrap();
                self.hands[p.idx()].remove(pos);
                self.discard.push(c);
                self.suit = ns.unwrap_or(c.suit);
                self.passes = 0;
                if self.hands[p.idx()].is_empty() {
                    let pts = hand_value(&self.hands[p.other().idx()]);
                    self.finish(p, pts);
                    return;
                }
                self.side = p.other();
            }
        }
        // după tragere cu stocul gol și fără carte jucabilă, rămâne mutarea Pass pentru același jucător
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        Some(if self.scores[0] >= self.scores[1] {
            Outcome::Win(Player::One)
        } else {
            Outcome::Win(Player::Two)
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        if self.phase == Phase::Over {
            return 0;
        }
        honest_eval(self, p, 14)
    }

    fn render(&self) -> String {
        let mut s = String::new();
        if self.prev != self.side && self.phase == Phase::Play {
            if let Viewer::HotSeat = self.viewer {
                s.push_str(&handover_banner(self.side));
            }
        }
        s.push_str(&format!(
            "CrazyEightsi (Crazy Eights) — scor: J1 {}, J2 {} (până la {})\n",
            self.scores[0], self.scores[1], TARGET
        ));
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        s.push_str(&format!(
            "Pe masă: {} | culoarea cerută: {} | stoc: {} cărți\n",
            self.top().text(),
            ['h', 'd', 's', 'c'][self.suit as usize],
            self.stock.len()
        ));
        if self.phase == Phase::Over {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        if self.phase == Phase::HandOver {
            s.push_str(&format!(
                "Mâna s-a încheiat. Mâinile rămase: J1 {} | J2 {}\n",
                join_cards(&self.hands[0]),
                join_cards(&self.hands[1])
            ));
            s.push_str("Scrie «urmatoarea».\n");
            return s;
        }
        let v = self.viewer.who(self.side);
        s.push_str(&format!(
            "Mâna ta (J{}): {}\n",
            v.idx() + 1,
            join_cards(&self.hands[v.idx()])
        ));
        s.push_str(&format!(
            "J{} are {} cărți\n",
            v.other().idx() + 1,
            self.hands[v.other().idx()].len()
        ));
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Play(c, None) => c.text(),
            Move::Play(c, Some(s)) => format!("{}:{}", c.text(), ['h', 'd', 's', 'c'][*s as usize]),
            Move::Draw => "trage".into(),
            Move::Pass => "pas".into(),
            Move::Next => "urmatoarea".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

impl Honest for CrazyEights {
    fn determinize(&self, me: Player, rng: &mut Rng) -> Self {
        let mut g = self.clone();
        let mi = me.idx();
        let oi = 1 - mi;
        let nf = (self.fresh[mi] as usize).min(self.hands[mi].len());
        let known: Vec<Card> = self.hands[mi][..self.hands[mi].len() - nf].to_vec();
        let mut pool = full_deck(2);
        pool.retain(|c| !known.contains(c) && !self.discard.contains(c));
        shuffle(&mut pool, rng);
        let n_opp = self.hands[oi].len();
        let mut mine = known;
        for _ in 0..nf {
            mine.push(pool.pop().unwrap());
        }
        g.hands[mi] = mine;
        g.hands[oi] = (0..n_opp).map(|_| pool.pop().unwrap()).collect();
        g.stock = pool;
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
            return (self.hands[q.idx()].len() as f64 - self.hands[p.idx()].len() as f64) * 3.0;
        }
        (self.gain[p.idx()] - self.gain[q.idx()]) as f64
    }

    fn policy(&self, rng: &mut Rng) -> Move {
        let mv = self.legal_moves();
        if mv.len() == 1 || rng.below(100) < 15 {
            return mv[rng.below(mv.len())];
        }
        let h = &self.hands[self.side.idx()];
        let mut best = mv[0];
        let mut bs = i32::MIN;
        for &m in &mv {
            let sc = match m {
                Move::Play(c, ns) => {
                    let mut sc = card_value(c);
                    if c.rank == 8 {
                        sc = -100;
                        if let Some(s) = ns {
                            sc +=
                                h.iter().filter(|x| x.suit == s && x.rank != 8).count() as i32 * 5;
                        }
                        if h.len() == 1 {
                            sc = 1000;
                        }
                    } else {
                        sc += h.iter().filter(|x| x.suit == c.suit).count() as i32;
                    }
                    sc
                }
                _ => 0,
            };
            if sc > bs {
                bs = sc;
                best = m;
            }
        }
        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(rank: u8, suit: u8) -> Card {
        Card { rank, suit }
    }

    #[test]
    fn deal_sizes_and_start_not_eight() {
        for seed in 1..30u64 {
            let g = CrazyEights::with_seed(seed);
            assert_eq!(g.hands[0].len(), 7);
            assert_eq!(g.hands[1].len(), 7);
            assert_eq!(g.stock.len(), 52 - 14 - 1);
            assert_ne!(g.top().rank, 8);
        }
    }

    #[test]
    fn play_rules_and_wild() {
        let mut g = CrazyEights::with_seed(1);
        g.discard = vec![c(9, 0)];
        g.suit = 0;
        g.hands[0] = vec![c(5, 0), c(9, 2), c(8, 3), c(7, 1)];
        let mv = g.legal_moves();
        assert!(mv.contains(&Move::Play(c(5, 0), None)));
        assert!(mv.contains(&Move::Play(c(9, 2), None)));
        assert!(!mv.contains(&Move::Play(c(7, 1), None)));
        assert!(mv.contains(&Move::Play(c(8, 3), Some(2))));
        assert!(!mv.contains(&Move::Draw));
        g.apply(&Move::Play(c(8, 3), Some(2)));
        assert_eq!(g.suit, 2);
        assert_eq!(g.side, Player::Two);
    }

    #[test]
    fn draw_only_when_stuck_and_pass_when_stock_empty() {
        let mut g = CrazyEights::with_seed(2);
        g.discard = vec![c(9, 0)];
        g.suit = 0;
        g.hands[0] = vec![c(5, 1), c(7, 2)];
        assert_eq!(g.legal_moves(), vec![Move::Draw]);
        g.stock.clear();
        assert_eq!(g.legal_moves(), vec![Move::Pass]);
    }

    #[test]
    fn going_out_scores_opponent_hand() {
        let mut g = CrazyEights::with_seed(3);
        g.discard = vec![c(9, 0)];
        g.suit = 0;
        g.hands[0] = vec![c(5, 0)];
        g.hands[1] = vec![c(8, 1), c(13, 2), c(14, 3), c(4, 0)];
        g.apply(&Move::Play(c(5, 0), None));
        assert_eq!(g.scores, [50 + 10 + 1 + 4, 0]);
        assert_eq!(g.phase, Phase::HandOver);
        assert_eq!(g.legal_moves(), vec![Move::Next]);
    }

    #[test]
    fn block_gives_difference() {
        let mut g = CrazyEights::with_seed(4);
        g.stock.clear();
        g.discard = vec![c(9, 0)];
        g.suit = 0;
        g.hands[0] = vec![c(5, 1)];
        g.hands[1] = vec![c(13, 2), c(6, 1)];
        g.apply(&Move::Pass);
        g.apply(&Move::Pass);
        assert_eq!(g.scores, [11, 0]);
    }

    #[test]
    fn determinize_hides_opponent() {
        let g = CrazyEights::with_seed(5);
        let mut rng = Rng::new(1);
        let d = g.determinize(Player::One, &mut rng);
        assert_eq!(d.hands[0], g.hands[0]);
        assert_eq!(d.hands[1].len(), 7);
        assert_ne!(d.hands[1], g.hands[1]);
        assert_eq!(d.stock.len(), g.stock.len());
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=8u64 {
            let mut g = CrazyEights::with_seed(seed);
            let mut rng = Rng::new(seed + 3);
            let mut n = 0;
            while g.outcome().is_none() && n < 5000 {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed}");
        }
    }
}
