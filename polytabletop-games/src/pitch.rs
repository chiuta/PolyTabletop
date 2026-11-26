//! Pitch (Setback), 2 jucători, din regulile publice ale variantei de bază (fără jokeri).
//!
//! * 52 de cărți, 6 fiecăruia (A cea mai mare). Licitația: non-dealerul spune `pas` sau `licit 2/3/4`
//!   (numărul de puncte pe care se angajează să le ia în mână), apoi dealerul poate spune `pas` sau poate
//!   licita cel puțin la fel (la egalitate îl „ia”); dacă non-dealerul a trecut, dealerul e obligat să licite.
//! * Licitatorul conduce prima levată, iar culoarea cărții conduse devine **atu**.
//! * Se răspunde la culoare **sau** se poate tăia oricând cu atu; cine nu are culoarea cerută poate pune
//!   orice. Dacă se conduce atu, se răspunde cu atu dacă ai. Levata o ia cel mai mare atu, altfel cea mai
//!   mare carte din culoarea cerută.
//! * Patru puncte pe mână: **High** (cel mai mare atu aflat în joc) → cel care ia levata cu el; **Low**
//!   (cel mai mic atu aflat în joc) → cel care l-a jucat (primit la împărțire); **Jack** (valetul de atu,
//!   dacă e în joc) → cel care ia levata cu el; **Game** (A=4, K=3, Q=2, J=1, 10=10 în levatele luate;
//!   mai mult ia punctul, egalitate = nimeni).
//! * Licitatorul care nu ia cel puțin cât a licitat pierde licitația din scor (poate coborî sub 0); celălalt
//!   își ia punctele oricum. Se joacă până la 11; dacă ambii ajung la 11 în aceeași mână, câștigă licitatorul.
//!
//! Cel mult 30 de mâini (apoi scorul mai mare). Mutări: `pas`, `licit 3`, `Ah` (carte), `urmatoarea`. Informație ascunsă: vezi `p3_comun` (AI cinstit).
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

const TARGET: i32 = 11;
const MAX_HANDS: u32 = 30;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    BidNon,
    BidDealer,
    Play,
    HandOver,
    Over,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Pass,
    Bid(u8),
    Play(Card),
    Next,
}

#[derive(Clone)]
pub struct Pitch {
    rng: Rng,
    hands: [Vec<Card>; 2],
    dealer: Player,
    side: Player,
    phase: Phase,
    first_bid: u8,
    bid: u8,
    bidder: Player,
    trump: Option<u8>,
    trick: Vec<(Player, Card)>,
    won: [Vec<Card>; 2],
    high: Option<(u8, Player)>,
    low: Option<(u8, Player)>,
    jack: Option<Player>,
    scores: [i32; 2],
    hands_played: u32,
    delta: [i32; 2],
    prev: Player,
    log: String,
    viewer: Viewer,
}

impl Default for Pitch {
    fn default() -> Self {
        Self::new()
    }
}

pub fn game_pts(c: Card) -> i32 {
    match c.rank {
        14 => 4,
        13 => 3,
        12 => 2,
        11 => 1,
        10 => 10,
        _ => 0,
    }
}

/// Mutările legale la răspuns (reguli comune Pitch / All Fours): culoarea cerută sau atu.
pub fn follow_moves(hand: &[Card], led: Option<u8>, trump: Option<u8>) -> Vec<Card> {
    let mut v: Vec<Card> = Vec::new();
    let allowed = |c: Card| match led {
        None => true,
        Some(l) => {
            if hand.iter().any(|x| x.suit == l) {
                c.suit == l || Some(c.suit) == trump
            } else {
                true
            }
        }
    };
    for &c in hand {
        if allowed(c) && !v.contains(&c) {
            v.push(c);
        }
    }
    v
}

/// Indexul câștigătorului unei levate.
pub fn trick_win(cards: &[Card], trump: Option<u8>) -> usize {
    let led = cards[0].suit;
    let mut best = 0;
    for i in 1..cards.len() {
        let (b, c) = (cards[best], cards[i]);
        let beats = if Some(c.suit) == trump && Some(b.suit) != trump {
            true
        } else if c.suit == b.suit {
            c.rank > b.rank
        } else {
            false
        };
        if beats && (c.suit == led || Some(c.suit) == trump) {
            best = i;
        }
    }
    best
}

impl Pitch {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Pitch {
            rng: Rng::new(seed),
            hands: [Vec::new(), Vec::new()],
            dealer: Player::Two,
            side: Player::One,
            phase: Phase::BidNon,
            first_bid: 0,
            bid: 0,
            bidder: Player::One,
            trump: None,
            trick: Vec::new(),
            won: [Vec::new(), Vec::new()],
            high: None,
            low: None,
            jack: None,
            scores: [0, 0],
            hands_played: 0,
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

    fn deal(&mut self) {
        let mut d = full_deck(2);
        shuffle(&mut d, &mut self.rng);
        self.hands = [Vec::new(), Vec::new()];
        for _ in 0..6 {
            for p in 0..2 {
                self.hands[p].push(d.pop().unwrap());
            }
        }
        for h in &mut self.hands {
            h.sort_by_key(|c| (c.suit, std::cmp::Reverse(c.rank)));
        }
        self.trump = None;
        self.trick.clear();
        self.won = [Vec::new(), Vec::new()];
        self.high = None;
        self.low = None;
        self.jack = None;
        self.delta = [0, 0];
        self.first_bid = 0;
        self.bid = 0;
        self.phase = Phase::BidNon;
        self.side = self.dealer.other();
    }

    fn end_hand(&mut self) {
        let mut pts = [0i32; 2];
        if let Some((_, p)) = self.high {
            pts[p.idx()] += 1;
        }
        if let Some((_, p)) = self.low {
            pts[p.idx()] += 1;
        }
        if let Some(p) = self.jack {
            pts[p.idx()] += 1;
        }
        let g: Vec<i32> = (0..2)
            .map(|i| self.won[i].iter().map(|&c| game_pts(c)).sum())
            .collect();
        if g[0] > g[1] {
            pts[0] += 1;
        } else if g[1] > g[0] {
            pts[1] += 1;
        }
        let b = self.bidder.idx();
        let mut change = pts;
        let made = pts[b] >= self.bid as i32;
        if !made {
            change[b] = -(self.bid as i32);
        }
        for i in 0..2 {
            self.scores[i] += change[i];
            self.delta[i] += change[i];
        }
        self.log = format!(
            "Mână încheiată: J{} licitase {} și {}; puncte: J1 {:+}, J2 {:+}",
            b + 1,
            self.bid,
            if made { "a reușit" } else { "e «set»" },
            change[0],
            change[1]
        );
        let o = 1 - b;
        self.hands_played += 1;
        self.phase = if (made && self.scores[b] >= TARGET)
            || self.scores[o] >= TARGET
            || self.scores[b] >= TARGET
            || self.hands_played >= MAX_HANDS
        {
            Phase::Over
        } else {
            Phase::HandOver
        };
    }
}

impl Game for Pitch {
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
            Phase::BidNon => vec![Move::Pass, Move::Bid(2), Move::Bid(3), Move::Bid(4)],
            Phase::BidDealer => {
                let mut v = Vec::new();
                if self.first_bid > 0 {
                    v.push(Move::Pass);
                }
                for b in self.first_bid.max(2)..=4 {
                    v.push(Move::Bid(b));
                }
                v
            }
            Phase::Play => {
                let h = &self.hands[self.side.idx()];
                let led = self.trick.first().map(|&(_, c)| c.suit);
                follow_moves(h, led, self.trump)
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
            Move::Pass => {
                if self.phase == Phase::BidNon {
                    self.phase = Phase::BidDealer;
                    self.side = self.dealer;
                } else {
                    self.bidder = self.dealer.other();
                    self.bid = self.first_bid;
                    self.phase = Phase::Play;
                    self.side = self.bidder;
                }
            }
            Move::Bid(b) => {
                if self.phase == Phase::BidNon {
                    self.first_bid = b;
                    self.phase = Phase::BidDealer;
                    self.side = self.dealer;
                } else {
                    self.bidder = self.dealer;
                    self.bid = b;
                    self.phase = Phase::Play;
                    self.side = self.bidder;
                }
            }
            Move::Play(c) => {
                let p = self.side;
                let pos = self.hands[p.idx()].iter().position(|&x| x == c).unwrap();
                self.hands[p.idx()].remove(pos);
                if self.trump.is_none() {
                    self.trump = Some(c.suit);
                }
                self.trick.push((p, c));
                if self.trick.len() < 2 {
                    self.side = p.other();
                    return;
                }
                let cards: Vec<Card> = self.trick.iter().map(|&(_, c)| c).collect();
                let w = self.trick[trick_win(&cards, self.trump)].0;
                let t = self.trump.unwrap();
                for &(pl, c) in &self.trick {
                    if c.suit == t {
                        if self.high.is_none_or(|(r, _)| c.rank > r) {
                            self.high = Some((c.rank, w));
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
        let b = self.bidder.idx();
        Some(if self.scores[b] >= TARGET {
            Outcome::Win(self.bidder)
        } else {
            match self.scores[0].cmp(&self.scores[1]) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                std::cmp::Ordering::Equal => Outcome::Draw,
            }
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
        if self.prev != self.side
            && matches!(self.phase, Phase::BidNon | Phase::BidDealer | Phase::Play)
        {
            if let Viewer::HotSeat = self.viewer {
                s.push_str(&handover_banner(self.side));
            }
        }
        s.push_str(&format!(
            "Pitch (Setback) — scor: J1 {}, J2 {} (până la {}) | dealer: J{}\n",
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
            Phase::BidNon => s.push_str("Licitație: J non-dealer.\n"),
            Phase::BidDealer => s.push_str(&format!(
                "Licitație: dealerul răspunde (non-dealerul a spus {}).\n",
                if self.first_bid == 0 {
                    "pas".into()
                } else {
                    self.first_bid.to_string()
                }
            )),
            Phase::Play => {
                s.push_str(&format!(
                    "Licitator: J{} ({} puncte) | atu: {}\n",
                    self.bidder.idx() + 1,
                    self.bid,
                    self.trump
                        .map(|t| ['h', 'd', 's', 'c'][t as usize].to_string())
                        .unwrap_or_else(|| "încă nestabilit (prima carte conduce)".into())
                ));
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
            Move::Pass => "pas".into(),
            Move::Bid(b) => format!("licit {b}"),
            Move::Play(c) => c.text(),
            Move::Next => "urmatoarea".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

pub fn suit_strength(h: &[Card], s: u8) -> f64 {
    let mut v = 0.0;
    let mut n = 0;
    for c in h.iter().filter(|c| c.suit == s) {
        n += 1;
        v += match c.rank {
            14 => 1.0,
            13 => 0.5,
            12 => 0.2,
            11 => 0.7,
            10 => 0.3,
            2 | 3 => 0.45,
            _ => 0.0,
        };
    }
    v + if n > 1 { 0.3 * (n - 1) as f64 } else { 0.0 }
}

impl Honest for Pitch {
    fn determinize(&self, me: Player, rng: &mut Rng) -> Self {
        let mut g = self.clone();
        let oi = 1 - me.idx();
        let mut pool = full_deck(2);
        pool.retain(|c| {
            !self.hands[me.idx()].contains(c)
                && !self.won[0].contains(c)
                && !self.won[1].contains(c)
                && !self.trick.iter().any(|&(_, x)| x == *c)
        });
        shuffle(&mut pool, rng);
        let n = self.hands[oi].len();
        g.hands[oi] = pool.into_iter().take(n).collect();
        g.hands[oi].sort_by_key(|c| (c.suit, std::cmp::Reverse(c.rank)));
        g.rng = Rng::new(rng.next_u64());
        g
    }

    fn rollout_done(&self) -> bool {
        matches!(self.phase, Phase::HandOver | Phase::Over)
    }

    fn rollout_value(&self, p: Player) -> f64 {
        let q = p.other();
        (self.delta[p.idx()] - self.delta[q.idx()]) as f64
    }

    fn policy(&self, rng: &mut Rng) -> Move {
        let mv = self.legal_moves();
        if mv.len() == 1 {
            return mv[0];
        }
        let h = &self.hands[self.side.idx()];
        match self.phase {
            Phase::BidNon | Phase::BidDealer => {
                let best = (0..4u8).map(|s| suit_strength(h, s)).fold(0.0, f64::max);
                let want = if best >= 3.4 {
                    4
                } else if best >= 2.7 {
                    3
                } else if best >= 2.0 {
                    2
                } else {
                    0
                };
                for &m in mv.iter().rev() {
                    match m {
                        Move::Bid(b) if b <= want => return m,
                        Move::Pass if want == 0 => return m,
                        _ => {}
                    }
                }
                mv[0]
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
                let trump = self.trump;
                if let Some(&(_, lead)) = self.trick.first() {
                    // cea mai mică carte care câștigă, altfel cea mai mică
                    let mut win: Option<Card> = None;
                    for &c in &cards {
                        if trick_win(&[lead, c], trump) == 1
                            && win.is_none_or(|w| rank_key(c, trump) < rank_key(w, trump))
                        {
                            win = Some(c);
                        }
                    }
                    let pts = game_pts(lead) >= 3 || Some(lead.suit) == trump;
                    if let Some(w) = win {
                        if pts || game_pts(w) <= 3 {
                            return Move::Play(w);
                        }
                    }
                    let low = cards
                        .iter()
                        .copied()
                        .min_by_key(|&c| rank_key(c, trump))
                        .unwrap();
                    Move::Play(low)
                } else if trump.is_none() {
                    // conducerea stabilește atu: cea mai lungă / puternică culoare, cartea cea mai mare
                    let s = (0..4u8)
                        .max_by(|&a, &b| {
                            suit_strength(h, a)
                                .partial_cmp(&suit_strength(h, b))
                                .unwrap()
                        })
                        .unwrap();
                    let c = cards
                        .iter()
                        .copied()
                        .filter(|c| c.suit == s)
                        .max_by_key(|c| c.rank)
                        .unwrap_or(cards[0]);
                    Move::Play(c)
                } else {
                    let c = cards
                        .iter()
                        .copied()
                        .max_by_key(|&c| rank_key(c, trump))
                        .unwrap();
                    Move::Play(c)
                }
            }
        }
    }
}

pub fn rank_key(c: Card, trump: Option<u8>) -> i32 {
    c.rank as i32 + if Some(c.suit) == trump { 20 } else { 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(rank: u8, suit: u8) -> Card {
        Card { rank, suit }
    }

    #[test]
    fn follow_or_trump_rule() {
        let h = vec![c(5, 0), c(9, 1), c(13, 2)];
        // s-a condus cupă (0), atu = pică (2): poți cupă sau pică, nu caro
        let m = follow_moves(&h, Some(0), Some(2));
        assert!(m.contains(&c(5, 0)) && m.contains(&c(13, 2)) && !m.contains(&c(9, 1)));
        // fără culoarea cerută: orice
        let m = follow_moves(&h, Some(3), Some(2));
        assert_eq!(m.len(), 3);
    }

    #[test]
    fn first_lead_sets_trump_and_bidder_leads() {
        let mut g = Pitch::with_seed(1);
        g.apply(&Move::Bid(2));
        g.apply(&Move::Pass);
        assert_eq!(g.phase, Phase::Play);
        assert_eq!(g.bidder, g.dealer.other());
        assert_eq!(g.side, g.bidder);
        let lead = g.legal_moves()[0];
        g.apply(&lead);
        if let Move::Play(card) = lead {
            assert_eq!(g.trump, Some(card.suit));
        }
    }

    #[test]
    fn dealer_must_bid_after_pass() {
        let mut g = Pitch::with_seed(2);
        g.apply(&Move::Pass);
        assert!(!g.legal_moves().contains(&Move::Pass));
        assert!(g.legal_moves().contains(&Move::Bid(2)));
        let mut h = Pitch::with_seed(3);
        h.apply(&Move::Bid(3));
        let mv = h.legal_moves();
        assert!(
            mv.contains(&Move::Pass) && mv.contains(&Move::Bid(3)) && !mv.contains(&Move::Bid(2))
        );
    }

    #[test]
    fn scoring_set_and_make() {
        let mut g = Pitch::with_seed(4);
        g.bidder = Player::One;
        g.bid = 3;
        g.trump = Some(0);
        g.high = Some((14, Player::One));
        g.low = Some((2, Player::Two));
        g.jack = Some(Player::Two);
        g.won = [vec![c(14, 1), c(10, 2)], vec![c(13, 1)]];
        g.end_hand();
        // J1: high + game = 2 < 3 -> set (-3); J2: low + jack = 2
        assert_eq!(g.scores, [-3, 2]);
        let mut h = Pitch::with_seed(5);
        h.bidder = Player::Two;
        h.bid = 2;
        h.trump = Some(0);
        h.scores = [10, 9];
        h.high = Some((13, Player::Two));
        h.low = Some((4, Player::Two));
        h.won = [vec![c(13, 1)], vec![c(10, 2)]];
        h.end_hand();
        assert_eq!(h.scores, [10, 12]);
        assert_eq!(h.outcome(), Some(Outcome::Win(Player::Two)));
    }

    #[test]
    fn bidder_checked_first_when_both_reach_target() {
        let mut g = Pitch::with_seed(6);
        g.bidder = Player::One;
        g.bid = 2;
        g.trump = Some(0);
        g.scores = [10, 10];
        g.high = Some((14, Player::One));
        g.low = Some((2, Player::One));
        g.jack = Some(Player::Two);
        g.won = [vec![c(10, 1)], vec![]];
        g.end_hand();
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn determinize_keeps_own_hand_and_played_cards() {
        let mut g = Pitch::with_seed(7);
        g.apply(&Move::Bid(2));
        g.apply(&Move::Pass);
        let lead = g.legal_moves()[0];
        g.apply(&lead);
        let mut rng = Rng::new(3);
        let d = g.determinize(Player::Two, &mut rng);
        assert_eq!(d.hands[1], g.hands[1]);
        assert_eq!(d.hands[0].len(), g.hands[0].len());
        if let Move::Play(card) = lead {
            assert!(!d.hands[0].contains(&card));
        }
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=8u64 {
            let mut g = Pitch::with_seed(seed);
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
