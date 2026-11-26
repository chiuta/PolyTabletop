//! Blackjack / 21: Player 1 against the dealer (Player 2, who follows the
//! fixed house rule and has no choices).
//!
//! Rules (public): single 52-card deck, freshly shuffled for every hand;
//! number cards count their value, J/Q/K 10, ace 1 or 11. Two cards each,
//! one dealer card face down. Moves for the player: `carte` (hit), `stai`
//! (stand), `dublu` (double down: one card only, stake doubled; only as the
//! first decision). The dealer draws until 17 or more and stands on every 17
//! (soft ones included). A two-card 21 is a *blackjack* (pays 3:2, loses to
//! nothing but a dealer blackjack, which is a push). No splitting, no
//! insurance, no surrender.
//!
//! Match format ("who wins" for a game of one against the house): a set of 21
//! hands with a flat stake of 10 chips from a 100-chip bankroll. After the 21st
//! hand Player 1 wins if the bankroll is above 100, the dealer wins if it is
//! below, otherwise it is a draw.
//!
//! Dealer/settlement steps are forced single moves of Player 2
//! (`dealer-trage`, `dealer-sta`, `decontare`), so that a hit by the AI can
//! never reveal the next card or the hole card to `evaluate()`.
//!
//! Hidden information: the deck and the dealer's hole card are in the state;
//! `render()` shows the dealer's up card and "[??]" until the dealer plays.
//! The AI (`AlphaBeta(1)`) uses `evaluate()`, which computes the expected
//! result of the position with an infinite-deck model from the player's cards
//! and the dealer's up card only (the card drawn by the last hit/double is
//! ignored and replaced by its expectation). Limitation: it does not count
//! cards of the real deck; it plays "basic strategy" to a good approximation.

use crate::cards52::{join_symbols, std_deck, std_symbol};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const HANDS: u8 = 21;
pub const BET: i32 = 10;
pub const START_BANKROLL: i32 = 100;

/// Card value with the ace as 11.
fn raw_value(c: u8) -> u8 {
    match c % 13 {
        r @ 0..=8 => r + 2,
        9..=11 => 10,
        _ => 11,
    }
}

/// Best total of a hand and whether an ace is still counted as 11.
pub fn hand_value(cards: &[u8]) -> (u8, bool) {
    let mut total: u32 = cards.iter().map(|&c| raw_value(c) as u32).sum();
    let mut soft_aces = cards.iter().filter(|&&c| c % 13 == 12).count();
    while total > 21 && soft_aces > 0 {
        total -= 10;
        soft_aces -= 1;
    }
    (total.min(99) as u8, soft_aces > 0)
}

pub fn is_blackjack(cards: &[u8]) -> bool {
    cards.len() == 2 && hand_value(cards).0 == 21
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Player,
    Dealer,
    Settle,
    Over,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Last {
    None,
    Hit,
    Stand,
    Double,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Move {
    Hit,
    Stand,
    Double,
    DealerDraw,
    DealerStand,
    Settle,
}

#[derive(Clone)]
pub struct Blackjack {
    rng: Rng,
    deck: Vec<u8>,
    player: Vec<u8>,
    dealer: Vec<u8>, // [0] up card, [1] hole card
    phase: Phase,
    bankroll: i32,
    hands_played: u8,
    doubled: bool,
    last: Last,
    last_result: String,
}

impl Default for Blackjack {
    fn default() -> Self {
        Self::new()
    }
}

impl Blackjack {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Blackjack {
            rng: Rng::new(seed),
            deck: Vec::new(),
            player: Vec::new(),
            dealer: Vec::new(),
            phase: Phase::Player,
            bankroll: START_BANKROLL,
            hands_played: 0,
            doubled: false,
            last: Last::None,
            last_result: String::new(),
        };
        g.new_hand();
        g
    }

    pub fn bankroll(&self) -> i32 {
        self.bankroll
    }

    fn new_hand(&mut self) {
        self.deck = std_deck(&mut self.rng);
        self.player.clear();
        self.dealer.clear();
        let a = self.deck.pop().unwrap();
        let b = self.deck.pop().unwrap();
        let c = self.deck.pop().unwrap();
        let d = self.deck.pop().unwrap();
        self.player = vec![a, c];
        self.dealer = vec![b, d];
        self.doubled = false;
        self.last = Last::None;
        self.phase = if is_blackjack(&self.player) || is_blackjack(&self.dealer) {
            Phase::Settle
        } else {
            Phase::Player
        };
    }

    /// Chip change of the current hand, from the player's side.
    fn delta(&self) -> i32 {
        let wager = if self.doubled { 2 * BET } else { BET };
        let pbj = is_blackjack(&self.player);
        let dbj = is_blackjack(&self.dealer);
        let p = hand_value(&self.player).0;
        let d = hand_value(&self.dealer).0;
        if pbj && dbj {
            0
        } else if pbj {
            BET * 3 / 2
        } else if dbj || p > 21 {
            -wager
        } else if d > 21 || p > d {
            wager
        } else if p < d {
            -wager
        } else {
            0
        }
    }

    fn settle(&mut self) {
        let d = self.delta();
        self.bankroll += d;
        self.last_result = format!(
            "Mâna {}: tu {} ({}), dealer {} ({}) → {}{} jetoane.",
            self.hands_played + 1,
            join_symbols(&self.player, std_symbol),
            hand_value(&self.player).0,
            join_symbols(&self.dealer, std_symbol),
            hand_value(&self.dealer).0,
            if d > 0 { "+" } else { "" },
            d
        );
        self.hands_played += 1;
        if self.hands_played >= HANDS {
            self.phase = Phase::Over;
        } else {
            self.new_hand();
        }
    }

    fn hole_revealed(&self) -> bool {
        matches!(self.phase, Phase::Dealer | Phase::Settle | Phase::Over)
    }
}

// ---- infinite-deck expectation model (used by evaluate) -------------------

const FINALS: usize = 6; // 17,18,19,20,21,bust

fn card_prob(v: u8) -> f64 {
    if v == 10 {
        4.0 / 13.0
    } else {
        1.0 / 13.0
    }
}

fn total_of(h: u8, ace: bool) -> u8 {
    if ace && h + 10 <= 21 {
        h + 10
    } else {
        h
    }
}

fn dealer_rec(h: u8, ace: bool, memo: &mut [[Option<[f64; FINALS]>; 2]; 32]) -> [f64; FINALS] {
    let t = total_of(h, ace);
    let mut out = [0.0; FINALS];
    if t > 21 {
        out[5] = 1.0;
        return out;
    }
    if t >= 17 {
        out[(t - 17) as usize] = 1.0;
        return out;
    }
    if let Some(v) = memo[h as usize][ace as usize] {
        return v;
    }
    for v in 1..=10u8 {
        let sub = dealer_rec(h + v, ace || v == 1, memo);
        for (o, s) in out.iter_mut().zip(sub.iter()) {
            *o += card_prob(v) * s;
        }
    }
    memo[h as usize][ace as usize] = Some(out);
    out
}

/// Final-total distribution of a dealer showing `up` (ace = 11).
pub fn dealer_distribution(up: u8) -> [f64; FINALS] {
    let mut memo = [[None; 2]; 32];
    let (h, ace) = if up == 11 { (1, true) } else { (up, false) };
    dealer_rec(h, ace, &mut memo)
}

/// Expected result (in stakes) of standing on `t` against the distribution.
fn stand_ev(t: u8, dist: &[f64; FINALS]) -> f64 {
    if t > 21 {
        return -1.0;
    }
    let mut win = dist[5];
    let mut lose = 0.0;
    for (i, &p) in dist.iter().enumerate().take(5) {
        let d = 17 + i as u8;
        if d < t {
            win += p;
        } else if d > t {
            lose += p;
        }
    }
    win - lose
}

fn opt_rec(h: u8, ace: bool, dist: &[f64; FINALS], memo: &mut [[Option<f64>; 2]; 32]) -> f64 {
    let t = total_of(h, ace);
    if t > 21 {
        return -1.0;
    }
    if let Some(v) = memo[h as usize][ace as usize] {
        return v;
    }
    let stand = stand_ev(t, dist);
    let mut hit = 0.0;
    for v in 1..=10u8 {
        hit += card_prob(v) * opt_rec(h + v, ace || v == 1, dist, memo);
    }
    let best = stand.max(hit);
    memo[h as usize][ace as usize] = Some(best);
    best
}

/// Hard total (aces as 1) and whether the hand holds an ace.
fn hard(cards: &[u8]) -> (u8, bool) {
    let h: u32 = cards
        .iter()
        .map(|&c| if c % 13 == 12 { 1 } else { raw_value(c) as u32 })
        .sum();
    (h.min(30) as u8, cards.iter().any(|&c| c % 13 == 12))
}

impl Game for Blackjack {
    type Move = Move;

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Player => Player::One,
            _ => Player::Two,
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        match self.phase {
            Phase::Player => {
                let mut v = vec![Move::Hit, Move::Stand];
                if self.player.len() == 2 && !self.doubled {
                    v.push(Move::Double);
                }
                v
            }
            Phase::Dealer => {
                if hand_value(&self.dealer).0 < 17 {
                    vec![Move::DealerDraw]
                } else {
                    vec![Move::DealerStand]
                }
            }
            Phase::Settle => vec![Move::Settle],
            Phase::Over => Vec::new(),
        }
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Hit => {
                self.player.push(self.deck.pop().expect("deck"));
                self.last = Last::Hit;
                if hand_value(&self.player).0 > 21 {
                    self.phase = Phase::Settle;
                }
            }
            Move::Stand => {
                self.last = Last::Stand;
                self.phase = Phase::Dealer;
            }
            Move::Double => {
                self.player.push(self.deck.pop().expect("deck"));
                self.doubled = true;
                self.last = Last::Double;
                self.phase = if hand_value(&self.player).0 > 21 {
                    Phase::Settle
                } else {
                    Phase::Dealer
                };
            }
            Move::DealerDraw => self.dealer.push(self.deck.pop().expect("deck")),
            Move::DealerStand | Move::Settle => self.settle(),
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        Some(match self.bankroll.cmp(&START_BANKROLL) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    /// Expected chips from the player's side, using the up card and the
    /// player's own cards only (infinite-deck model). The card drawn by the
    /// last hit/double is replaced by its expectation, so the evaluation
    /// never depends on the true next card.
    fn evaluate(&self, p: Player) -> i32 {
        let up = raw_value(self.dealer[0]);
        let dist = dealer_distribution(up);
        let mut memo = [[None; 2]; 32];
        let known: &[u8] = match self.last {
            Last::Hit | Last::Double => &self.player[..self.player.len() - 1],
            _ => &self.player,
        };
        let (h, ace) = hard(known);
        let t = total_of(h, ace);
        let ev = match self.last {
            Last::Stand => stand_ev(t, &dist),
            Last::Hit => (1..=10u8)
                .map(|v| card_prob(v) * opt_rec(h + v, ace || v == 1, &dist, &mut memo))
                .sum(),
            Last::Double => {
                2.0 * (1..=10u8)
                    .map(|v| card_prob(v) * stand_ev(total_of(h + v, ace || v == 1), &dist))
                    .sum::<f64>()
            }
            Last::None => opt_rec(h, ace, &dist, &mut memo),
        };
        let v = (self.bankroll - START_BANKROLL) * 100 + (ev * BET as f64 * 100.0).round() as i32;
        if p == Player::One {
            v
        } else {
            -v
        }
    }

    fn render(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            "Blackjack — mâna {}/{HANDS}, pariu {BET} jetoane. Jetoane: {} (start {START_BANKROLL}). Câștigi dacă termini peste {START_BANKROLL}.\n",
            (self.hands_played + 1).min(HANDS),
            self.bankroll
        ));
        let (pv, soft) = hand_value(&self.player);
        s.push_str(&format!(
            "Mâna ta (Jucător 1): {} = {}{}\n",
            join_symbols(&self.player, std_symbol),
            pv,
            if soft { " (soft)" } else { "" }
        ));
        if self.hole_revealed() {
            s.push_str(&format!(
                "Dealer (Jucător 2): {} = {}\n",
                join_symbols(&self.dealer, std_symbol),
                hand_value(&self.dealer).0
            ));
        } else {
            s.push_str(&format!(
                "Dealer (Jucător 2): {} [??] (1 carte ascunsă)\n",
                std_symbol(self.dealer[0])
            ));
        }
        if !self.last_result.is_empty() {
            s.push_str(&format!("Mâna anterioară — {}\n", self.last_result));
        }
        match self.phase {
            Phase::Player => s.push_str("La mutare: tu — carte / stai / dublu\n"),
            Phase::Dealer => s.push_str("Dealerul joacă (dealer-trage / dealer-sta)\n"),
            Phase::Settle => s.push_str("Mâna s-a încheiat: decontare\n"),
            Phase::Over => s.push_str("Setul s-a încheiat.\n"),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Hit => "carte",
            Move::Stand => "stai",
            Move::Double => "dublu",
            Move::DealerDraw => "dealer-trage",
            Move::DealerStand => "dealer-sta",
            Move::Settle => "decontare",
        }
        .into()
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // card helper: rank label index (0 = "2" … 12 = "A"), suit 0
    fn k(rank: u8) -> u8 {
        rank
    }
    const TWO: u8 = 0;
    const NINE: u8 = 7;
    const TEN: u8 = 8;
    const JACK: u8 = 9;
    const KING: u8 = 11;
    const ACE: u8 = 12;

    #[test]
    fn hand_values() {
        assert_eq!(hand_value(&[k(ACE), k(KING)]), (21, true));
        assert_eq!(hand_value(&[k(ACE), k(ACE)]), (12, true));
        assert_eq!(hand_value(&[k(ACE), k(ACE), k(NINE)]), (21, true));
        assert_eq!(hand_value(&[k(ACE), k(NINE), k(TEN)]), (20, false));
        assert_eq!(hand_value(&[k(KING), k(JACK), k(TWO)]), (22, false));
        assert_eq!(hand_value(&[k(TWO), k(TWO), k(NINE)]), (13, false));
        assert_eq!(hand_value(&[k(ACE), k(5)]), (18, true)); // A + 7
    }

    #[test]
    fn blackjack_needs_two_cards() {
        assert!(is_blackjack(&[ACE, TEN]));
        assert!(!is_blackjack(&[7, 7, 7])); // 9+9+9? no: 3 cards
        assert!(!is_blackjack(&[ACE, ACE, NINE]));
    }

    fn fixed(player: &[u8], dealer: &[u8], deck: &[u8]) -> Blackjack {
        let mut g = Blackjack::with_seed(1);
        g.player = player.to_vec();
        g.dealer = dealer.to_vec();
        g.deck = deck.to_vec(); // popped from the end
        g.phase = Phase::Player;
        g.doubled = false;
        g.last = Last::None;
        g.bankroll = START_BANKROLL;
        g.hands_played = 0;
        g
    }

    #[test]
    fn dealer_draws_to_17_and_stands_on_soft_17() {
        let mut g = fixed(&[TEN, 8], &[TEN, 4], &[2, 1]); // dealer 10+7 = 17? 4 is "6": 16
        g.apply(&Move::Stand);
        assert_eq!(g.legal_moves(), vec![Move::DealerDraw]);
        g.apply(&Move::DealerDraw); // pops 1 ("3") -> 19
        assert_eq!(hand_value(&g.dealer).0, 19);
        assert_eq!(g.legal_moves(), vec![Move::DealerStand]);
        let mut s = fixed(&[TEN, 8], &[ACE, 5], &[]); // soft 17
        s.apply(&Move::Stand);
        assert_eq!(s.legal_moves(), vec![Move::DealerStand]);
    }

    #[test]
    fn payouts() {
        // player 20 beats dealer 19
        let mut g = fixed(&[TEN, TEN], &[TEN, NINE], &[]);
        g.apply(&Move::Stand);
        g.apply(&Move::DealerStand);
        assert_eq!(g.bankroll, 110);
        // bust loses even if the dealer would bust
        let mut g = fixed(&[TEN, 5], &[TEN, 5], &[TEN]);
        g.apply(&Move::Hit);
        assert_eq!(g.to_move(), Player::Two);
        assert_eq!(g.legal_moves(), vec![Move::Settle]);
        g.apply(&Move::Settle);
        assert_eq!(g.bankroll, 90);
        // push
        let mut g = fixed(&[TEN, NINE], &[TEN, NINE], &[]);
        g.apply(&Move::Stand);
        g.apply(&Move::DealerStand);
        assert_eq!(g.bankroll, 100);
        // dealer busts
        let mut g = fixed(&[TEN, 5], &[TEN, 4], &[JACK]);
        g.apply(&Move::Stand);
        g.apply(&Move::DealerDraw);
        g.apply(&Move::DealerStand);
        assert_eq!(g.bankroll, 110);
    }

    #[test]
    fn double_down_doubles_the_stake() {
        // 7+6 = 13, doubling draws a 10: 23, bust, -20
        let mut g = fixed(&[5, 4], &[TEN, 5], &[TEN]);
        assert!(g.legal_moves().contains(&Move::Double));
        g.apply(&Move::Double);
        assert_eq!(g.player.len(), 3);
        assert_eq!(g.legal_moves(), vec![Move::Settle]);
        g.apply(&Move::Settle);
        assert_eq!(g.bankroll, 80);
        // 5+6 = 11, doubling draws a 10: 21 beats the dealer's 17, +20
        let mut g = fixed(&[3, 4], &[TEN, 5], &[TEN]);
        g.apply(&Move::Double);
        assert_eq!(g.legal_moves(), vec![Move::DealerStand]);
        g.apply(&Move::DealerStand);
        assert_eq!(g.bankroll, 120);
        // no second double, no double after a hit
        let mut g = fixed(&[3, 4], &[TEN, 5], &[0, 0]);
        g.apply(&Move::Hit);
        assert!(!g.legal_moves().contains(&Move::Double));
    }

    #[test]
    fn natural_pays_three_to_two_and_push_on_both() {
        let mut g = fixed(&[ACE, KING], &[TEN, 5], &[]);
        g.phase = Phase::Settle;
        g.apply(&Move::Settle);
        assert_eq!(g.bankroll, 115);
        let mut g = fixed(&[ACE, KING], &[ACE, TEN], &[]);
        g.phase = Phase::Settle;
        g.apply(&Move::Settle);
        assert_eq!(g.bankroll, 100);
        let mut g = fixed(&[TEN, 5], &[ACE, TEN], &[]);
        g.phase = Phase::Settle;
        g.apply(&Move::Settle);
        assert_eq!(g.bankroll, 90);
    }

    #[test]
    fn dealer_distribution_sums_to_one() {
        for up in 2..=11u8 {
            let d = dealer_distribution(up);
            assert!((d.iter().sum::<f64>() - 1.0).abs() < 1e-9, "{up}");
        }
        // a dealer showing 6 busts far more often than one showing 10
        assert!(dealer_distribution(6)[5] > 0.38 && dealer_distribution(10)[5] < 0.25);
    }

    #[test]
    fn evaluation_prefers_basic_strategy() {
        // hard 16 vs 10: hit is better than stand; hard 12 vs 6: stand
        let mut stand = fixed(&[TEN, 4], &[TEN, 0], &[]);
        stand.last = Last::Stand;
        let mut hit = fixed(&[TEN, 4, 0], &[TEN, 0], &[]); // last card is ignored
        hit.last = Last::Hit;
        assert!(hit.evaluate(Player::One) > stand.evaluate(Player::One));
        let mut stand = fixed(&[TEN, 0], &[4, 0], &[]); // 12 vs 6
        stand.last = Last::Stand;
        let mut hit = fixed(&[TEN, 0, 0], &[4, 0], &[]);
        hit.last = Last::Hit;
        assert!(stand.evaluate(Player::One) > hit.evaluate(Player::One));
        // doubling 11 vs 6 beats hitting
        let mut dbl = fixed(&[3, 4, 0], &[4, 0], &[]);
        dbl.last = Last::Double;
        let mut hit = fixed(&[3, 4, 0], &[4, 0], &[]);
        hit.last = Last::Hit;
        assert!(dbl.evaluate(Player::One) > hit.evaluate(Player::One));
    }

    #[test]
    fn evaluate_does_not_see_the_next_card() {
        let g = fixed(&[TEN, 4], &[TEN, 0], &[0, 12]);
        let mut a = g.clone();
        a.apply(&Move::Hit); // draws ACE (12): total 22? 10+6+... ace = 1 -> 17
        let mut b = g.clone();
        b.deck = vec![0, KING];
        b.apply(&Move::Hit); // draws a King: bust
        assert_eq!(a.evaluate(Player::One), b.evaluate(Player::One));
    }

    #[test]
    fn render_hides_the_hole_card() {
        let g = Blackjack::with_seed(4);
        if g.phase == Phase::Player {
            let r = Game::render(&g);
            assert!(r.contains("[??]"));
            assert!(
                !r.contains(&std_symbol(g.dealer[1]).to_string()) || g.dealer[1] == g.dealer[0]
            );
        }
    }

    #[test]
    fn random_sets_finish_and_conserve() {
        for seed in 1..=40u64 {
            let mut g = Blackjack::with_seed(seed);
            let mut rng = Rng::new(seed * 7);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 2000);
            }
            assert_eq!(g.hands_played, HANDS);
        }
    }

    #[test]
    fn ai_does_better_than_a_random_player() {
        use polytabletop_core::GameObj;
        let (mut ai_sum, mut rnd_sum) = (0, 0);
        for seed in 1..=30u64 {
            let mut g: Box<dyn GameObj> = Box::new(Blackjack::with_seed(seed));
            let mut rng = Rng::new(seed);
            let mut r: Box<dyn GameObj> = Box::new(Blackjack::with_seed(seed));
            while g.outcome().is_none() {
                let m = g.ai_move(rng.next_u64()).unwrap();
                g.play(&m).unwrap();
            }
            while r.outcome().is_none() {
                let m = r.legal_moves();
                r.play(&m[rng.below(m.len())]).unwrap();
            }
            let score = |b: &dyn GameObj| match b.outcome() {
                Some(Outcome::Win(Player::One)) => 1,
                Some(Outcome::Win(Player::Two)) => -1,
                _ => 0,
            };
            ai_sum += score(g.as_ref());
            rnd_sum += score(r.as_ref());
        }
        assert!(ai_sum > rnd_sum, "ai {ai_sum} vs random {rnd_sum}");
    }
}
