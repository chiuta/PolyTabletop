//! All Fours (Trinidad / Old Sledge), 2 jucători, din regulile publice.
//!
//! * 52 de cărți, 6 fiecăruia; următoarea carte se întoarce: culoarea ei e **atu**. Dacă e un Valet
//!   („kick”), dealerul ia 3 puncte.
//! * (Pasul `imparte` e forțat: dă cărțile după `schimb`.)
//! * Non-dealerul poate `cer` (cere schimbarea) sau `joc` (acceptă atuul). La `cer`, dealerul alege
//!   `dau` (non-dealerul ia 1 punct și se joacă cu atuul întors) sau `schimb` (se mai dau 3 cărți
//!   fiecăruia, se întoarce un atu nou — dacă are aceeași culoare se repetă —, apoi fiecare aruncă
//!   3 cărți cu `dis 5h` ca să rămână cu 6).
//! * Non-dealerul conduce. Se răspunde la culoare **sau** se poate tăia cu atu (ca la Pitch); fără
//!   culoarea cerută poți pune orice. Levata o ia cel mai mare atu, altfel cea mai mare carte din
//!   culoarea cerută.
//! * Patru puncte pe mână: **High** (cel mai mare atu aflat în joc) și **Low** (cel mai mic) → cel care
//!   le-a avut în mână; **Jack** (valetul de atu, dacă a fost jucat) → cel care ia levata cu el;
//!   **Game** (A=4, K=3, Q=2, J=1, 10=10 în levatele luate; mai mult ia punctul, egal = nimeni).
//! * Câștigă primul la 14 puncte (punctele se acordă în ordinea kick, High, Low, Jack, Game).
//!
//! Omise: „hang jack” și punctele de kick pentru As și Șase (variante regionale).
//! Mutări: `cer`, `joc`, `dau`, `schimb`, `dis 5h`, `Ah`, `urmatoarea`.

use crate::cards::{full_deck, handover_banner, join_cards, shuffle, Card, Viewer};
use crate::p3_comun::{honest_eval, Honest};
use crate::pitch::{follow_moves, game_pts, rank_key, suit_strength, trick_win};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const TARGET: i32 = 14;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Beg,
    Reply,
    Dealing,
    Discard,
    Play,
    HandOver,
    Over,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Beg,
    Stand,
    Give,
    Run,
    Deal3,
    Discard(Card),
    Play(Card),
    Next,
}

#[derive(Clone)]
pub struct AllFours {
    rng: Rng,
    stock: Vec<Card>,
    hands: [Vec<Card>; 2],
    junk: [Vec<Card>; 2],
    dealer: Player,
    side: Player,
    phase: Phase,
    trump_card: Card,
    shown: Vec<Card>,
    trick: Vec<(Player, Card)>,
    won: [Vec<Card>; 2],
    high: Option<(u8, Player)>,
    low: Option<(u8, Player)>,
    jack: Option<Player>,
    scores: [i32; 2],
    delta: [i32; 2],
    prev: Player,
    log: String,
    viewer: Viewer,
}

impl Default for AllFours {
    fn default() -> Self {
        Self::new()
    }
}

impl AllFours {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = AllFours {
            rng: Rng::new(seed),
            stock: Vec::new(),
            hands: [Vec::new(), Vec::new()],
            junk: [Vec::new(), Vec::new()],
            dealer: Player::Two,
            side: Player::One,
            phase: Phase::Beg,
            trump_card: Card { rank: 2, suit: 0 },
            shown: Vec::new(),
            trick: Vec::new(),
            won: [Vec::new(), Vec::new()],
            high: None,
            low: None,
            jack: None,
            scores: [0, 0],
            delta: [0, 0],
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

    fn trump(&self) -> u8 {
        self.trump_card.suit
    }

    fn sort_hands(&mut self) {
        for h in &mut self.hands {
            h.sort_by_key(|c| (c.suit, std::cmp::Reverse(c.rank)));
        }
    }

    fn award(&mut self, p: Player, n: i32) {
        self.scores[p.idx()] += n;
        self.delta[p.idx()] += n;
    }

    fn reached(&self) -> bool {
        self.scores[0] >= TARGET || self.scores[1] >= TARGET
    }

    fn kick(&mut self) {
        self.trump_card = self.stock.pop().unwrap();
        self.shown.push(self.trump_card);
        if self.trump_card.rank == 11 {
            let d = self.dealer;
            self.award(d, 3);
            self.log
                .push_str(&format!(" Kick Valet: J{} +3.", d.idx() + 1));
        }
    }

    fn deal(&mut self) {
        let mut d = full_deck(2);
        shuffle(&mut d, &mut self.rng);
        self.hands = [Vec::new(), Vec::new()];
        self.junk = [Vec::new(), Vec::new()];
        for _ in 0..6 {
            for p in 0..2 {
                self.hands[p].push(d.pop().unwrap());
            }
        }
        self.stock = d;
        self.shown.clear();
        self.trick.clear();
        self.won = [Vec::new(), Vec::new()];
        self.high = None;
        self.low = None;
        self.jack = None;
        self.delta = [0, 0];
        self.log.clear();
        self.kick();
        self.sort_hands();
        self.side = self.dealer.other();
        self.phase = if self.reached() {
            Phase::Over
        } else {
            Phase::Beg
        };
    }

    fn start_play(&mut self) {
        self.phase = Phase::Play;
        self.side = self.dealer.other();
        self.sort_hands();
    }

    fn end_hand(&mut self) {
        if let Some((_, p)) = self.high {
            self.award(p, 1);
            if self.reached() {
                return self.finish();
            }
        }
        if let Some((_, p)) = self.low {
            self.award(p, 1);
            if self.reached() {
                return self.finish();
            }
        }
        if let Some(p) = self.jack {
            self.award(p, 1);
            if self.reached() {
                return self.finish();
            }
        }
        let g: Vec<i32> = (0..2)
            .map(|i| self.won[i].iter().map(|&c| game_pts(c)).sum())
            .collect();
        if g[0] != g[1] {
            let w = if g[0] > g[1] {
                Player::One
            } else {
                Player::Two
            };
            self.award(w, 1);
        }
        self.finish();
    }

    fn finish(&mut self) {
        self.log = format!(
            "Mână încheiată: J1 {:+}, J2 {:+}",
            self.delta[0], self.delta[1]
        );
        self.phase = if self.reached() {
            Phase::Over
        } else {
            Phase::HandOver
        };
    }
}

impl Game for AllFours {
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
            Phase::Beg => vec![Move::Beg, Move::Stand],
            Phase::Reply => vec![Move::Give, Move::Run],
            Phase::Dealing => vec![Move::Deal3],
            Phase::Discard => {
                let h = &self.hands[self.side.idx()];
                let mut v = Vec::new();
                for &c in h {
                    if !v.contains(&Move::Discard(c)) {
                        v.push(Move::Discard(c));
                    }
                }
                v
            }
            Phase::Play => {
                let h = &self.hands[self.side.idx()];
                let led = self.trick.first().map(|&(_, c)| c.suit);
                follow_moves(h, led, Some(self.trump()))
                    .into_iter()
                    .map(Move::Play)
                    .collect()
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        self.prev = self.side;
        match *m {
            Move::Next => {
                self.dealer = self.dealer.other();
                self.deal();
            }
            Move::Stand => self.start_play(),
            Move::Beg => {
                self.phase = Phase::Reply;
                self.side = self.dealer;
            }
            Move::Give => {
                let b = self.dealer.other();
                self.award(b, 1);
                self.log = format!("Dealerul dă un punct lui J{}.", b.idx() + 1);
                if self.reached() {
                    self.finish();
                } else {
                    self.start_play();
                }
            }
            Move::Run => {
                // cărțile se dau la pasul forțat următor (altfel AI-ul ar vedea cărțile din stoc)
                self.phase = Phase::Dealing;
            }
            Move::Deal3 => {
                // încă 3 cărți fiecăruia, atu nou (se repetă cât timp are aceeași culoare)
                let old = self.trump();
                for _ in 0..3 {
                    for p in 0..2 {
                        if let Some(c) = self.stock.pop() {
                            self.hands[p].push(c);
                        }
                    }
                }
                self.kick();
                let mut guard = 0;
                while self.trump() == old && self.stock.len() > 7 && guard < 4 && !self.reached() {
                    for _ in 0..3 {
                        for p in 0..2 {
                            if let Some(c) = self.stock.pop() {
                                self.hands[p].push(c);
                            }
                        }
                    }
                    self.kick();
                    guard += 1;
                }
                if self.reached() {
                    return self.finish();
                }
                self.phase = Phase::Discard;
                self.side = self.dealer.other();
            }
            Move::Discard(c) => {
                let p = self.side;
                let pos = self.hands[p.idx()].iter().position(|&x| x == c).unwrap();
                self.hands[p.idx()].remove(pos);
                self.junk[p.idx()].push(c);
                if self.hands[p.idx()].len() == 6 {
                    if p == self.dealer {
                        self.start_play();
                    } else {
                        self.side = self.dealer;
                    }
                }
            }
            Move::Play(c) => {
                let p = self.side;
                let pos = self.hands[p.idx()].iter().position(|&x| x == c).unwrap();
                self.hands[p.idx()].remove(pos);
                self.trick.push((p, c));
                if self.trick.len() < 2 {
                    self.side = p.other();
                    return;
                }
                let t = self.trump();
                let cards: Vec<Card> = self.trick.iter().map(|&(_, c)| c).collect();
                let w = self.trick[trick_win(&cards, Some(t))].0;
                for &(pl, c) in &self.trick {
                    if c.suit == t {
                        if self.high.is_none_or(|(r, _)| c.rank > r) {
                            self.high = Some((c.rank, pl));
                        }
                        if self.low.is_none_or(|(r, _)| c.rank < r) {
                            self.low = Some((c.rank, pl));
                        }
                        if c.rank == 11 {
                            self.jack = Some(w);
                        }
                    }
                }
                for &(_, c) in &self.trick {
                    self.won[w.idx()].push(c);
                }
                self.trick.clear();
                self.side = w;
                if self.hands[0].is_empty() {
                    self.end_hand();
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        Some(if self.scores[0] >= TARGET {
            Outcome::Win(Player::One)
        } else {
            Outcome::Win(Player::Two)
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        if self.phase == Phase::Over {
            return 0;
        }
        honest_eval(self, p, 16)
    }

    fn render(&self) -> String {
        let mut s = String::new();
        if self.prev != self.side && !matches!(self.phase, Phase::Over | Phase::HandOver) {
            if let Viewer::HotSeat = self.viewer {
                s.push_str(&handover_banner(self.side));
            }
        }
        s.push_str(&format!(
            "All Fours — scor: J1 {}, J2 {} (până la {}) | dealer: J{}\n",
            self.scores[0],
            self.scores[1],
            TARGET,
            self.dealer.idx() + 1
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
            _ => {}
        }
        s.push_str(&format!(
            "Atu întors: {} (atu {})\n",
            self.trump_card.text(),
            ['h', 'd', 's', 'c'][self.trump() as usize]
        ));
        match self.phase {
            Phase::Beg => s.push_str("Non-dealerul: cer sau joc.\n"),
            Phase::Reply => s.push_str("Dealerul: dau (un punct) sau schimb.\n"),
            Phase::Dealing => s.push_str(
                "Dealerul a ales schimb: scrie «imparte» (încă 3 cărți fiecăruia și atu nou).\n",
            ),
            Phase::Discard => s.push_str("Aruncă cărți (dis X) până rămân 6.\n"),
            _ => {
                if !self.trick.is_empty() {
                    let cs: Vec<Card> = self.trick.iter().map(|&(_, c)| c).collect();
                    s.push_str(&format!("Pe masă: {}\n", join_cards(&cs)));
                }
            }
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
            Move::Beg => "cer".into(),
            Move::Stand => "joc".into(),
            Move::Give => "dau".into(),
            Move::Run => "schimb".into(),
            Move::Deal3 => "imparte".into(),
            Move::Discard(c) => format!("dis {}", c.text()),
            Move::Play(c) => c.text(),
            Move::Next => "urmatoarea".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

impl Honest for AllFours {
    fn determinize(&self, me: Player, rng: &mut Rng) -> Self {
        let mut g = self.clone();
        let (mi, oi) = (me.idx(), 1 - me.idx());
        let mut pool = full_deck(2);
        pool.retain(|c| {
            !self.hands[mi].contains(c)
                && !self.junk[mi].contains(c)
                && !self.shown.contains(c)
                && !self.won[0].contains(c)
                && !self.won[1].contains(c)
                && !self.trick.iter().any(|&(_, x)| x == *c)
        });
        shuffle(&mut pool, rng);
        let n = self.hands[oi].len();
        let nj = self.junk[oi].len();
        g.hands[oi] = pool.drain(..n).collect();
        g.junk[oi] = pool.drain(..nj).collect();
        g.stock = pool;
        g.hands[oi].sort_by_key(|c| (c.suit, std::cmp::Reverse(c.rank)));
        g.rng = Rng::new(rng.next_u64());
        g
    }

    fn rollout_done(&self) -> bool {
        matches!(self.phase, Phase::HandOver | Phase::Over)
    }

    fn rollout_value(&self, p: Player) -> f64 {
        (self.delta[p.idx()] - self.delta[p.other().idx()]) as f64
    }

    fn policy(&self, rng: &mut Rng) -> Move {
        let mv = self.legal_moves();
        if mv.len() == 1 {
            return mv[0];
        }
        let h = &self.hands[self.side.idx()];
        let t = self.trump();
        match self.phase {
            Phase::Beg => {
                if suit_strength(h, t) < 1.4 {
                    Move::Beg
                } else {
                    Move::Stand
                }
            }
            Phase::Reply => {
                if suit_strength(h, t) >= 2.2 {
                    Move::Give
                } else {
                    Move::Run
                }
            }
            Phase::Discard => {
                let best = (0..4u8)
                    .max_by(|&a, &b| {
                        suit_strength(h, a)
                            .partial_cmp(&suit_strength(h, b))
                            .unwrap()
                    })
                    .unwrap();
                let c = h
                    .iter()
                    .copied()
                    .min_by_key(|c| (c.suit == best || c.suit == t, c.rank))
                    .unwrap();
                Move::Discard(c)
            }
            _ => {
                if rng.below(100) < 10 {
                    return mv[rng.below(mv.len())];
                }
                let cards: Vec<Card> = mv
                    .iter()
                    .filter_map(|m| {
                        if let Move::Play(c) = m {
                            Some(*c)
                        } else {
                            None
                        }
                    })
                    .collect();
                let trump = Some(t);
                if let Some(&(_, lead)) = self.trick.first() {
                    let mut win: Option<Card> = None;
                    for &c in &cards {
                        if trick_win(&[lead, c], trump) == 1
                            && win.is_none_or(|w| rank_key(c, trump) < rank_key(w, trump))
                        {
                            win = Some(c);
                        }
                    }
                    let valuable = game_pts(lead) >= 3 || lead.suit == t;
                    if let Some(w) = win {
                        if valuable || game_pts(w) <= 3 {
                            return Move::Play(w);
                        }
                    }
                    Move::Play(
                        cards
                            .iter()
                            .copied()
                            .min_by_key(|&c| rank_key(c, trump))
                            .unwrap(),
                    )
                } else {
                    Move::Play(
                        cards
                            .iter()
                            .copied()
                            .max_by_key(|&c| rank_key(c, trump))
                            .unwrap(),
                    )
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(rank: u8, suit: u8) -> Card {
        Card { rank, suit }
    }

    #[test]
    fn deal_and_kick() {
        for seed in 1..20u64 {
            let g = AllFours::with_seed(seed);
            assert_eq!(g.hands[0].len() + g.hands[1].len() + g.stock.len() + 1, 52);
            assert_eq!(g.hands[0].len(), 6);
            if g.trump_card.rank == 11 {
                assert_eq!(g.scores[g.dealer.idx()], 3);
            } else {
                assert_eq!(g.scores, [0, 0]);
            }
        }
    }

    #[test]
    fn give_one_scores_for_beggar() {
        let mut g = AllFours::with_seed(3);
        g.scores = [0, 0];
        g.apply(&Move::Beg);
        assert_eq!(g.phase, Phase::Reply);
        assert_eq!(g.side, g.dealer);
        let before = g.scores[g.dealer.other().idx()];
        g.apply(&Move::Give);
        assert_eq!(g.scores[g.dealer.other().idx()], before + 1);
        assert_eq!(g.phase, Phase::Play);
        assert_eq!(g.side, g.dealer.other());
    }

    #[test]
    fn run_deals_three_more_and_discard_to_six() {
        let mut g = AllFours::with_seed(4);
        g.apply(&Move::Beg);
        g.apply(&Move::Run);
        assert_eq!(g.phase, Phase::Dealing);
        g.apply(&Move::Deal3);
        if g.phase == Phase::Discard {
            assert_eq!(g.hands[0].len(), 9);
            assert_eq!(g.hands[1].len(), 9);
            for _ in 0..6 {
                let m = g.legal_moves()[0];
                g.apply(&m);
            }
            assert_eq!(g.hands[0].len(), 6);
            assert_eq!(g.hands[1].len(), 6);
            assert_eq!(g.phase, Phase::Play);
        }
    }

    #[test]
    fn high_low_belong_to_holder_jack_to_captor() {
        let mut g = AllFours::with_seed(5);
        g.phase = Phase::Play;
        g.trump_card = c(5, 0);
        g.scores = [0, 0];
        g.delta = [0, 0];
        g.won = [vec![], vec![]];
        g.high = None;
        g.low = None;
        g.jack = None;
        g.hands = [vec![c(14, 0), c(9, 1)], vec![c(11, 0), c(2, 0)]];
        g.side = Player::One;
        g.trick.clear();
        // J1 conduce A♥ (atu), J2 pune Valetul de atu: levata e a lui J1, valetul ajunge la J1
        g.apply(&Move::Play(c(14, 0)));
        g.apply(&Move::Play(c(11, 0)));
        assert_eq!(g.jack, Some(Player::One));
        assert_eq!(g.high, Some((14, Player::One)));
        // ultima levată: J1 conduce 9 caro, J2 taie cu 2 de atu
        g.apply(&Move::Play(c(9, 1)));
        g.apply(&Move::Play(c(2, 0)));
        // High J1, Low J2 (2♥), Jack J1, Game: J1 A+? contra 2: J1 are A(4)+J(1)=5, J2 are 9(0)+2(0)=0 -> J1
        assert_eq!(g.scores, [3, 1]);
        assert_eq!(g.phase, Phase::HandOver);
    }

    #[test]
    fn game_tie_scores_nothing() {
        let mut g = AllFours::with_seed(6);
        g.trump_card = c(5, 0);
        g.scores = [0, 0];
        g.delta = [0, 0];
        g.won = [vec![c(14, 1)], vec![c(14, 2)]];
        g.high = None;
        g.low = None;
        g.jack = None;
        g.end_hand();
        assert_eq!(g.scores, [0, 0]);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=8u64 {
            let mut g = AllFours::with_seed(seed);
            let mut rng = Rng::new(seed + 3);
            let mut n = 0;
            while g.outcome().is_none() && n < 8000 {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed}");
        }
    }
}
