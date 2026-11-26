//! Batak (Turcia), 2 jucători — adaptare a jocului de levate cu licitație (în original, 4 jucători).
//!
//! * Pachet de 52, câte 13 cărți fiecăruia; ceilalte 26 rămân nefolosite și nevăzute (ca la celelalte jocuri
//!   de levate pe doi ale colecției).
//! * **Licitația**: cel care nu e dealer spune un număr de levate între 5 și 13 sau `pas`; dealerul poate licita
//!   mai mult sau da pas (dacă primul a dat pas, dealerul trebuie să liciteze). Cine licitează cel mai mult
//!   alege **atuul** (oricare culoare) și conduce.
//! * **Jocul**: se răspunde obligatoriu la culoare și, dacă poți, cu o carte mai mare decât cea care ia
//!   momentan levata; fără culoare trebuie să tai cu atu (și să suprataie dacă poți); fără atu, orice. Ia cea mai
//!   mare carte din culoarea condusă sau cel mai mare atu; câștigătorul conduce.
//! * **Scor**: licitatorul primește +contract dacă ia cel puțin atâtea levate, altfel −contract; celălalt
//!   primește câte un punct pe levată luată. Câștigă primul care ajunge la 41 de puncte (după cel mult 30 de
//!   mâini câștigă cine are mai multe).
//!
//! Mutări: `6` (licitație), `pas`, `atu h` (culoarea atuului: h d s c), carte (`Ah`, `7c`), `urmatoarea`.
#![allow(clippy::if_same_then_else)]

use crate::cards::{full_deck, handover_banner, join_cards, shuffle, Card, Viewer};
use crate::p3_comun::{honest_eval, Honest};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const TARGET: i32 = 41;
const MAX_HANDS: u32 = 30;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Bid1,
    Bid2,
    Trump,
    Play,
    HandOver,
    Over,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Bid(u8),
    Pass,
    Trump(u8),
    Play(Card),
    Next,
}

#[derive(Clone)]
pub struct Batak {
    rng: Rng,
    hands: [Vec<Card>; 2],
    dealer: Player,
    phase: Phase,
    side: Player,
    bid: u8,
    bidder: Player,
    trump: u8,
    trick: Vec<(Player, Card)>,
    won: [u8; 2],
    scores: [i32; 2],
    gain: [i32; 2],
    seen: Vec<Card>,
    hands_played: u32,
    prev: Player,
    log: String,
    viewer: Viewer,
}

impl Default for Batak {
    fn default() -> Self {
        Self::new()
    }
}

fn sort_hand(h: &mut [Card]) {
    h.sort_by_key(|c| (c.suit, std::cmp::Reverse(c.rank)));
}

impl Batak {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Batak {
            rng: Rng::new(seed),
            hands: [Vec::new(), Vec::new()],
            dealer: Player::Two,
            phase: Phase::Bid1,
            side: Player::One,
            bid: 0,
            bidder: Player::One,
            trump: 0,
            trick: Vec::new(),
            won: [0, 0],
            scores: [0, 0],
            gain: [0, 0],
            seen: Vec::new(),
            hands_played: 0,
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
        for p in 0..2 {
            self.hands[p] = d.drain(..13).collect();
            sort_hand(&mut self.hands[p]);
        }
        self.phase = Phase::Bid1;
        self.side = self.dealer.other();
        self.bid = 0;
        self.trick.clear();
        self.won = [0, 0];
        self.gain = [0, 0];
        self.seen.clear();
    }

    /// Cartea `c` bate cartea `best` (care ia momentan levata) în contextul culorii conduse?
    fn beats(&self, best: Card, c: Card, led: u8) -> bool {
        if c.suit == best.suit {
            c.rank > best.rank
        } else {
            c.suit == self.trump && best.suit != self.trump && led != u8::MAX
        }
    }

    fn winning(&self) -> Card {
        let led = self.trick[0].1.suit;
        let mut best = self.trick[0].1;
        for &(_, c) in &self.trick[1..] {
            if self.beats(best, c, led) {
                best = c;
            }
        }
        best
    }

    fn allowed(&self) -> Vec<Card> {
        let h = &self.hands[self.side.idx()];
        if self.trick.is_empty() {
            return h.clone();
        }
        let led = self.trick[0].1.suit;
        let best = self.winning();
        let of_led: Vec<Card> = h.iter().copied().filter(|c| c.suit == led).collect();
        if !of_led.is_empty() {
            if best.suit == led {
                let higher: Vec<Card> = of_led
                    .iter()
                    .copied()
                    .filter(|c| c.rank > best.rank)
                    .collect();
                if !higher.is_empty() {
                    return higher;
                }
            }
            return of_led;
        }
        let trumps: Vec<Card> = h.iter().copied().filter(|c| c.suit == self.trump).collect();
        if !trumps.is_empty() {
            if best.suit == self.trump {
                let higher: Vec<Card> = trumps
                    .iter()
                    .copied()
                    .filter(|c| c.rank > best.rank)
                    .collect();
                if !higher.is_empty() {
                    return higher;
                }
            }
            return trumps;
        }
        h.clone()
    }

    fn finish(&mut self) {
        let b = self.bidder.idx();
        let o = 1 - b;
        if self.won[b] >= self.bid {
            self.gain[b] = self.bid as i32;
        } else {
            self.gain[b] = -(self.bid as i32);
        }
        self.gain[o] = self.won[o] as i32;
        for p in 0..2 {
            self.scores[p] += self.gain[p];
        }
        self.hands_played += 1;
        self.log = format!(
            "Contract {} al lui J{}: {} levate luate → {:+} / {:+}",
            self.bid,
            b + 1,
            self.won[b],
            self.gain[b],
            self.gain[o]
        );
        self.phase = if self.scores[0] >= TARGET
            || self.scores[1] >= TARGET
            || self.hands_played >= MAX_HANDS
        {
            Phase::Over
        } else {
            Phase::HandOver
        };
    }
}

impl Game for Batak {
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
            Phase::Bid1 => {
                let mut v: Vec<Move> = (5..=13).map(Move::Bid).collect();
                v.push(Move::Pass);
                v
            }
            Phase::Bid2 => {
                let mut v: Vec<Move> = (self.bid.max(4) + 1..=13).map(Move::Bid).collect();
                if self.bid > 0 {
                    v.push(Move::Pass);
                } else if v.is_empty() {
                    v.push(Move::Pass);
                }
                v
            }
            Phase::Trump => (0..4).map(Move::Trump).collect(),
            Phase::Play => self.allowed().into_iter().map(Move::Play).collect(),
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
            Move::Bid(n) => {
                self.bid = n;
                self.bidder = self.side;
                if self.phase == Phase::Bid1 {
                    self.phase = Phase::Bid2;
                    self.side = self.side.other();
                } else {
                    self.phase = Phase::Trump;
                    self.side = self.bidder;
                }
            }
            Move::Pass => {
                if self.phase == Phase::Bid1 {
                    self.phase = Phase::Bid2;
                    self.side = self.side.other();
                } else {
                    // dealerul a dat pas: licitatorul rămâne primul
                    self.phase = Phase::Trump;
                    self.side = self.bidder;
                }
            }
            Move::Trump(s) => {
                self.trump = s;
                self.phase = Phase::Play;
                self.side = self.bidder;
            }
            Move::Play(c) => {
                let p = self.side;
                let pos = self.hands[p.idx()].iter().position(|&x| x == c).unwrap();
                self.hands[p.idx()].remove(pos);
                self.seen.push(c);
                self.trick.push((p, c));
                if self.trick.len() == 2 {
                    let best = self.winning();
                    let winner = self.trick.iter().find(|t| t.1 == best).unwrap().0;
                    self.won[winner.idx()] += 1;
                    self.log = format!("J{} ia levata", winner.idx() + 1);
                    self.trick.clear();
                    self.side = winner;
                    if self.hands[0].is_empty() {
                        self.finish();
                    }
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
        Some(match self.scores[0].cmp(&self.scores[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
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
        if self.prev != self.side && !matches!(self.phase, Phase::Over | Phase::HandOver) {
            if let Viewer::HotSeat = self.viewer {
                s.push_str(&handover_banner(self.side));
            }
        }
        s.push_str(&format!(
            "Batak — scor J1 {}, J2 {} (țintă {TARGET})\n",
            self.scores[0], self.scores[1]
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
                s.push_str("Mâna s-a încheiat. Scrie «urmatoarea».\n");
                return s;
            }
            _ => {}
        }
        if self.phase == Phase::Play || self.phase == Phase::Trump {
            s.push_str(&format!(
                "Contract: J{} cu {}{} | levate: J1 {}, J2 {}\n",
                self.bidder.idx() + 1,
                self.bid,
                if self.phase == Phase::Play {
                    format!(", atu {}", ['h', 'd', 's', 'c'][self.trump as usize])
                } else {
                    String::new()
                },
                self.won[0],
                self.won[1]
            ));
        }
        if !self.trick.is_empty() {
            s.push_str(&format!(
                "Pe masă: {}\n",
                self.trick
                    .iter()
                    .map(|(p, c)| format!("J{}:{}", p.idx() + 1, c.text()))
                    .collect::<Vec<_>>()
                    .join(" ")
            ));
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
            Move::Next => "urmatoarea".into(),
            Move::Pass => "pas".into(),
            Move::Bid(n) => n.to_string(),
            Move::Trump(s) => format!("atu {}", ['h', 'd', 's', 'c'][*s as usize]),
            Move::Play(c) => c.text(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

impl Honest for Batak {
    fn determinize(&self, me: Player, rng: &mut Rng) -> Self {
        let mut g = self.clone();
        let mi = me.idx();
        let oi = 1 - mi;
        let known: Vec<Card> = self.hands[mi].clone();
        let mut pool = full_deck(2);
        pool.retain(|c| !known.contains(c) && !self.seen.contains(c));
        shuffle(&mut pool, rng);
        let n_opp = self.hands[oi].len();
        g.hands[oi] = (0..n_opp).map(|_| pool.pop().unwrap()).collect();
        sort_hand(&mut g.hands[oi]);
        g.rng = Rng::new(rng.next_u64());
        g
    }

    fn rollout_done(&self) -> bool {
        matches!(self.phase, Phase::HandOver | Phase::Over)
    }

    fn rollout_value(&self, p: Player) -> f64 {
        let q = p.other();
        if self.rollout_done() {
            return (self.gain[p.idx()] - self.gain[q.idx()]) as f64;
        }
        (self.won[p.idx()] as f64 - self.won[q.idx()] as f64) * 0.5
    }

    fn policy(&self, rng: &mut Rng) -> Move {
        let mv = self.legal_moves();
        if mv.len() == 1 || rng.below(100) < 15 {
            return mv[rng.below(mv.len())];
        }
        match self.phase {
            Phase::Bid1 | Phase::Bid2 => {
                // licitează după puterea mâinii
                let h = &self.hands[self.side.idx()];
                let strength: i32 = h
                    .iter()
                    .map(|c| {
                        if c.rank >= 13 {
                            2
                        } else if c.rank == 12 {
                            1
                        } else {
                            0
                        }
                    })
                    .sum::<i32>()
                    / 2
                    + 4;
                let want = (strength as u8).clamp(5, 13);
                for &m in &mv {
                    if m == Move::Bid(want) {
                        return m;
                    }
                }
                if mv.contains(&Move::Pass) {
                    return Move::Pass;
                }
                mv[0]
            }
            Phase::Trump => {
                let h = &self.hands[self.side.idx()];
                let mut cnt = [0; 4];
                for c in h {
                    cnt[c.suit as usize] += 1 + (c.rank >= 12) as i32;
                }
                let best = (0..4).max_by_key(|&s| cnt[s]).unwrap();
                Move::Trump(best as u8)
            }
            _ => {
                let mut best = mv[0];
                let mut bs = i32::MIN;
                for &m in &mv {
                    if let Move::Play(c) = m {
                        let mut sc = if self.trick.is_empty() {
                            c.rank as i32 + if c.suit == self.trump { 12 } else { 0 }
                        } else {
                            let b = self.winning();
                            let led = self.trick[0].1.suit;
                            if self.beats(b, c, led) {
                                30 - c.rank as i32
                            } else {
                                -(c.rank as i32)
                            }
                        };
                        if c.suit == self.trump && !self.trick.is_empty() {
                            sc -= 1;
                        }
                        if sc > bs {
                            bs = sc;
                            best = m;
                        }
                    }
                }
                best
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn must_follow_and_head_the_trick() {
        let mut g = Batak::with_seed(2);
        g.phase = Phase::Play;
        g.trump = 2; // spade
        let c = |rank, suit| Card { rank, suit };
        g.hands[1] = vec![c(5, 0), c(12, 0), c(3, 1), c(9, 2)];
        g.trick = vec![(Player::One, c(9, 0))];
        g.side = Player::Two;
        // trebuie Q♥ (mai mare decât 9♥)
        assert_eq!(g.allowed(), vec![c(12, 0)]);
        // fără culoare: atu obligatoriu
        g.hands[1] = vec![c(3, 1), c(9, 2), c(4, 2)];
        assert_eq!(g.allowed().len(), 2);
        // fără culoare și fără atu: orice
        g.hands[1] = vec![c(3, 1), c(4, 3)];
        assert_eq!(g.allowed().len(), 2);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=4u64 {
            let mut g = Batak::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty(), "{}", g.render());
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 5000);
            }
        }
    }
}
