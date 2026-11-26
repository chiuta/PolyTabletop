//! Șase-zeci și șase (Santase / Sixty-six), 2 players, 24 cards, written from
//! the publicly known rules.
//!
//! * Cards 9, J, Q, K, 10, A in four suits (power order 9 < J < Q < K < 10 < A,
//!   points 0, 2, 3, 4, 10, 11 — 120 in all). Six cards each; the next card is
//!   turned up and fixes the trump suit (it stays at the bottom of the stock).
//! * While the stock is open there is no obligation to follow suit; the trick
//!   winner draws first. After the stock is exhausted or *closed* you must
//!   follow suit, beat the led card if you can, and trump if you are void.
//! * Marriage: leading Q or K while holding both gives 20 points (40 in
//!   trump) — only after you have won a trick. Move text `Qh+`.
//! * Exchange: when leading with the stock open (≥ 3 cards left) the 9 of
//!   trump may be swapped for the face-up trump card (`schimb`).
//! * Closing (`inchide`): when leading with ≥ 3 stock cards left; the
//!   stock is no longer drawn from.
//! * 66 or more points ends the hand at once (the engine claims it for you).
//!   Game points: loser without a trick 3, loser under 33 points 2, else 1.
//!   If the closer fails to reach 66, the opponent gets 2 (3 if the opponent
//!   had no trick when the game was closed). If nobody reaches 66 and the
//!   stock was not closed, the winner of the last trick gets 1.
//! * The match goes to 7 game points (variant chosen for brevity).
//!
//! Moves: `9h`, `Jd`, `10s`, `Ac` (rank + suit letter c/d/h/s), `Qh+`
//! (announce marriage), `schimb`, `inchide`, `urmatoarea` (deal the next
//! hand, after a hand ended).
//!
//! Hidden information: the state owns the shuffled stock and both hands.
//! `render()` shows Player 1's hand and only the *number* of Player 2's
//! cards. The AI is `AlphaBeta(1)` over `evaluate()`, which reads only the
//! evaluated player's own hand, the face-up trump card and public points; the
//! cards drawn after the trick just completed are ignored (they would be
//! information from the future). It therefore plays a heuristic, not a
//! card-counting or look-ahead game: it never "cheats", but it is weak in
//! the end-game where card counting matters.

use crate::cards::Viewer;
use crate::cards52::{shuffle, SUIT_LETTERS, SUIT_SYMBOLS};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const RANK_LABEL: [&str; 6] = ["9", "J", "Q", "K", "10", "A"];
const RANK_PTS: [u16; 6] = [0, 2, 3, 4, 10, 11];
const TARGET: u8 = 7;

pub fn suit(c: u8) -> u8 {
    c / 6
}
pub fn rank(c: u8) -> u8 {
    c % 6
}
pub fn make_card(suit: u8, rank: u8) -> u8 {
    suit * 6 + rank
}
pub fn card_points(c: u8) -> u16 {
    RANK_PTS[rank(c) as usize]
}
pub fn card_text(c: u8) -> String {
    format!(
        "{}{}",
        RANK_LABEL[rank(c) as usize],
        SUIT_LETTERS[suit(c) as usize]
    )
}
fn card_symbol(c: u8) -> String {
    format!(
        "{}{}",
        RANK_LABEL[rank(c) as usize],
        SUIT_SYMBOLS[suit(c) as usize]
    )
}

/// Does `resp` (played second) win against `led`?
pub fn second_wins(led: u8, resp: u8, trump: u8) -> bool {
    if suit(resp) == suit(led) {
        rank(resp) > rank(led)
    } else {
        suit(resp) == trump
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Move {
    Play(u8, bool),
    Exchange,
    Close,
    Next,
}

#[derive(Clone)]
pub struct SixtySix {
    rng: Rng,
    viewer: Viewer,
    hands: [Vec<u8>; 2],
    /// Index 0 is the face-up trump card (bottom); cards are drawn with `pop`.
    stock: Vec<u8>,
    trump: u8,
    pts: [u16; 2],
    tricks: [u8; 2],
    led: Option<u8>,
    side: Player,
    closed: Option<Player>,
    opp_tricks_at_close: u8,
    game: [u8; 2],
    hand_result: Option<(Player, u8)>,
    dealer: Player,
    /// Cards drawn when the last trick was completed (ignored by evaluate).
    drawn: [Option<u8>; 2],
}

impl Default for SixtySix {
    fn default() -> Self {
        Self::new()
    }
}

impl SixtySix {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = SixtySix {
            rng: Rng::new(seed),
            viewer: Viewer::Fixed(Player::One),
            hands: [Vec::new(), Vec::new()],
            stock: Vec::new(),
            trump: 0,
            pts: [0; 2],
            tricks: [0; 2],
            led: None,
            side: Player::One,
            closed: None,
            opp_tricks_at_close: 0,
            game: [0; 2],
            hand_result: None,
            dealer: Player::Two,
            drawn: [None; 2],
        };
        g.deal();
        g
    }

    fn deal(&mut self) {
        let mut deck: Vec<u8> = (0..24).collect();
        shuffle(&mut deck, &mut self.rng);
        self.hands = [deck[0..6].to_vec(), deck[6..12].to_vec()];
        self.stock = deck[12..24].to_vec();
        self.trump = suit(self.stock[0]);
        self.pts = [0; 2];
        self.tricks = [0; 2];
        self.led = None;
        self.closed = None;
        self.opp_tricks_at_close = 0;
        self.hand_result = None;
        self.drawn = [None; 2];
        self.side = self.dealer.other();
    }

    pub fn trump_suit(&self) -> u8 {
        self.trump
    }
    pub fn points(&self) -> (u16, u16) {
        (self.pts[0], self.pts[1])
    }
    pub fn game_points(&self) -> (u8, u8) {
        (self.game[0], self.game[1])
    }
    pub fn hand_of(&self, p: Player) -> &[u8] {
        &self.hands[p.idx()]
    }
    pub fn stock_len(&self) -> usize {
        self.stock.len()
    }

    fn is_over(&self) -> bool {
        self.game[0] >= TARGET || self.game[1] >= TARGET
    }

    fn open_stock(&self) -> bool {
        self.closed.is_none() && !self.stock.is_empty()
    }

    fn can_marry(&self, me: Player, c: u8) -> bool {
        let r = rank(c);
        if !(r == 2 || r == 3) || self.tricks[me.idx()] == 0 {
            return false;
        }
        let partner = make_card(suit(c), 5 - r);
        self.hands[me.idx()].contains(&partner)
    }

    fn follow_cards(&self, me: Player) -> Vec<u8> {
        let hand = &self.hands[me.idx()];
        if self.open_stock() {
            return hand.clone();
        }
        let l = self.led.expect("a card was led");
        let beat: Vec<u8> = hand
            .iter()
            .copied()
            .filter(|&c| suit(c) == suit(l) && rank(c) > rank(l))
            .collect();
        if !beat.is_empty() {
            return beat;
        }
        let same: Vec<u8> = hand
            .iter()
            .copied()
            .filter(|&c| suit(c) == suit(l))
            .collect();
        if !same.is_empty() {
            return same;
        }
        let trumps: Vec<u8> = hand
            .iter()
            .copied()
            .filter(|&c| suit(c) == self.trump)
            .collect();
        if !trumps.is_empty() {
            return trumps;
        }
        hand.clone()
    }

    fn game_points_for(&self, winner: Player) -> u8 {
        let loser = winner.other();
        if let Some(c) = self.closed {
            if c != winner {
                return if self.opp_tricks_at_close == 0 { 3 } else { 2 };
            }
        }
        if self.tricks[loser.idx()] == 0 {
            3
        } else if self.pts[loser.idx()] < 33 {
            2
        } else {
            1
        }
    }

    fn end_hand(&mut self, winner: Player, gp: u8) {
        self.game[winner.idx()] += gp;
        self.hand_result = Some((winner, gp));
        self.led = None;
    }

    fn hand_view(&self, p: Player) -> Vec<u8> {
        let drawn = self.drawn[p.idx()];
        self.hands[p.idx()]
            .iter()
            .copied()
            .filter(|&c| Some(c) != drawn)
            .collect()
    }
}

impl Game for SixtySix {
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
        if self.is_over() {
            return Vec::new();
        }
        if self.hand_result.is_some() {
            return vec![Move::Next];
        }
        let me = self.side;
        let mut v = Vec::new();
        if self.led.is_none() {
            for &c in &self.hands[me.idx()] {
                v.push(Move::Play(c, false));
                if self.can_marry(me, c) {
                    v.push(Move::Play(c, true));
                }
            }
            if self.open_stock() && self.stock.len() >= 3 {
                let nine = make_card(self.trump, 0);
                if self.hands[me.idx()].contains(&nine) {
                    v.push(Move::Exchange);
                }
                v.push(Move::Close);
            }
        } else {
            for c in self.follow_cards(me) {
                v.push(Move::Play(c, false));
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        let me = self.side;
        match *m {
            Move::Next => {
                self.dealer = self.dealer.other();
                self.deal();
            }
            Move::Exchange => {
                let nine = make_card(self.trump, 0);
                let h = &mut self.hands[me.idx()];
                let pos = h.iter().position(|&c| c == nine).expect("holds the 9");
                h[pos] = self.stock[0];
                self.stock[0] = nine;
            }
            Move::Close => {
                self.closed = Some(me);
                self.opp_tricks_at_close = self.tricks[me.other().idx()];
            }
            Move::Play(c, announce) => {
                let h = &mut self.hands[me.idx()];
                let pos = h.iter().position(|&x| x == c).expect("card in hand");
                h.remove(pos);
                self.drawn = [None; 2];
                if announce {
                    self.pts[me.idx()] += if suit(c) == self.trump { 40 } else { 20 };
                    if self.pts[me.idx()] >= 66 {
                        // the played card is not resolved: the hand is over
                        let gp = self.game_points_for(me);
                        self.end_hand(me, gp);
                        return;
                    }
                }
                match self.led {
                    None => {
                        self.led = Some(c);
                        self.side = me.other();
                    }
                    Some(l) => {
                        let leader = me.other();
                        let winner = if second_wins(l, c, self.trump) {
                            me
                        } else {
                            leader
                        };
                        let loser = winner.other();
                        self.led = None;
                        self.pts[winner.idx()] += card_points(l) + card_points(c);
                        self.tricks[winner.idx()] += 1;
                        if self.open_stock() {
                            if let Some(x) = self.stock.pop() {
                                self.hands[winner.idx()].push(x);
                                self.drawn[winner.idx()] = Some(x);
                            }
                            if let Some(x) = self.stock.pop() {
                                self.hands[loser.idx()].push(x);
                                self.drawn[loser.idx()] = Some(x);
                            }
                        }
                        if self.pts[winner.idx()] >= 66 {
                            let gp = self.game_points_for(winner);
                            self.end_hand(winner, gp);
                        } else if self.hands[0].is_empty() {
                            match self.closed {
                                Some(c) => {
                                    let w = c.other();
                                    let gp = self.game_points_for(w);
                                    self.end_hand(w, gp);
                                }
                                None => self.end_hand(winner, 1),
                            }
                        } else {
                            self.side = winner;
                        }
                    }
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.game[0] >= TARGET {
            Some(Outcome::Win(Player::One))
        } else if self.game[1] >= TARGET {
            Some(Outcome::Win(Player::Two))
        } else {
            None
        }
    }

    /// Reads only: the evaluated player's own hand (minus the cards just
    /// drawn), the public trump card, announced/won points and game points.
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let opp = p.other().idx();
        let mut s = (self.game[me] as i32 - self.game[opp] as i32) * 5000;
        if let Some((w, gp)) = self.hand_result {
            let bonus = 1000 * gp as i32;
            return s + if w == p { bonus } else { -bonus };
        }
        s += (self.pts[me] as i32 - self.pts[opp] as i32) * 12;
        let hand = self.hand_view(p);
        for &c in &hand {
            let r = rank(c) as i32;
            s += if suit(c) == self.trump {
                20 + 3 * r
            } else {
                match rank(c) {
                    5 => 12,
                    4 => 8,
                    3 => 5,
                    2 => 3,
                    1 => 2,
                    _ => 0,
                }
            };
            if rank(c) == 3 && hand.contains(&make_card(suit(c), 2)) {
                s += if suit(c) == self.trump { 30 } else { 14 };
            }
        }
        if let Some(l) = self.led {
            if self.side != p {
                // my card is on the table, waiting for the answer
                if suit(l) != self.trump && rank(l) < 5 {
                    s -= card_points(l) as i32 * 4;
                }
            }
        }
        if let Some(c) = self.closed {
            let bonus = (self.pts[me] as i32 - 25) * 8;
            s += if c == p { bonus } else { -bonus / 2 };
        }
        s
    }

    fn render(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            "Șase-zeci și șase — meci până la {TARGET} puncte de joc: J1 {} – J2 {}\n",
            self.game[0], self.game[1]
        ));
        s.push_str(&format!(
            "Mâna curentă: J1 {} puncte, J2 {} puncte. Atu: {}",
            self.pts[0], self.pts[1], SUIT_SYMBOLS[self.trump as usize]
        ));
        if let Some(&t) = self.stock.first() {
            s.push_str(&format!(" (cartea de atu din pachet: {})", card_symbol(t)));
        }
        s.push('\n');
        s.push_str(&format!(
            "Pachet: {} cărți{}. Jucător {} (ascuns) are {} cărți.\n",
            self.stock.len(),
            match self.closed {
                Some(c) => format!(" — închis de Jucător {}", c.idx() + 1),
                None => String::new(),
            },
            self.viewer.who(self.side).other().idx() + 1,
            self.hands[self.viewer.who(self.side).other().idx()].len()
        ));
        if let Some(l) = self.led {
            s.push_str(&format!(
                "Pe masă: Jucător {} a jucat {}\n",
                self.side.other().idx() + 1,
                card_symbol(l)
            ));
        }
        let v = self.viewer.who(self.side);
        let mut h = self.hands[v.idx()].clone();
        h.sort_unstable();
        s.push_str(&format!(
            "Mâna ta (Jucător {}): {}\n",
            v.idx() + 1,
            h.iter()
                .map(|&c| card_symbol(c))
                .collect::<Vec<_>>()
                .join(" ")
        ));
        match self.hand_result {
            Some((w, gp)) if !self.is_over() => s.push_str(&format!(
                "Mâna s-a încheiat: Jucător {} câștigă {} punct(e) de joc. Scrie «urmatoarea».\n",
                w.idx() + 1,
                gp
            )),
            Some((w, gp)) => s.push_str(&format!(
                "Meci încheiat: Jucător {} câștigă ({} p. în ultima mână).\n",
                w.idx() + 1,
                gp
            )),
            None => s.push_str(&format!("La mutare: Jucător {}\n", self.side.idx() + 1)),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Play(c, false) => card_text(*c),
            Move::Play(c, true) => format!("{}+", card_text(*c)),
            Move::Exchange => "schimb".into(),
            Move::Close => "inchide".into(),
            Move::Next => "urmatoarea".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(s: u8, r: u8) -> u8 {
        make_card(s, r)
    }

    #[test]
    fn deck_is_worth_120_points() {
        let total: u16 = (0..24).map(card_points).sum();
        assert_eq!(total, 120);
        assert_eq!(card_points(c(0, 5)), 11); // ace
        assert_eq!(card_points(c(0, 4)), 10); // ten
        assert_eq!(card_points(c(0, 0)), 0); // nine
    }

    #[test]
    fn deal_is_consistent() {
        let g = SixtySix::with_seed(3);
        assert_eq!(g.hands[0].len(), 6);
        assert_eq!(g.hands[1].len(), 6);
        assert_eq!(g.stock.len(), 12);
        let mut all: Vec<u8> = g.hands[0]
            .iter()
            .chain(&g.hands[1])
            .chain(&g.stock)
            .copied()
            .collect();
        all.sort_unstable();
        assert_eq!(all, (0..24).collect::<Vec<u8>>());
        assert_eq!(g.trump, suit(g.stock[0]));
    }

    #[test]
    fn trick_winner_rules() {
        let trump = 2;
        assert!(second_wins(c(0, 1), c(0, 4), trump)); // higher of same suit
        assert!(!second_wins(c(0, 4), c(0, 1), trump));
        assert!(!second_wins(c(0, 1), c(1, 5), trump)); // off-suit loses
        assert!(second_wins(c(0, 5), c(2, 0), trump)); // trump beats ace
        assert!(!second_wins(c(2, 0), c(0, 5), trump));
        assert!(second_wins(c(2, 0), c(2, 1), trump));
    }

    fn base() -> SixtySix {
        let mut g = SixtySix::with_seed(1);
        g.hands = [Vec::new(), Vec::new()];
        g.stock.clear();
        g.trump = 2;
        g.pts = [0; 2];
        g.tricks = [0; 2];
        g.led = None;
        g.side = Player::One;
        g.closed = None;
        g.hand_result = None;
        g
    }

    #[test]
    fn must_follow_and_beat_when_stock_closed() {
        let mut g = base();
        g.closed = Some(Player::One);
        g.led = Some(c(0, 2)); // Q of suit 0 led
        g.side = Player::Two;
        g.hands[1] = vec![c(0, 0), c(0, 3), c(1, 5), c(2, 0)];
        let mv: Vec<u8> = g.follow_cards(Player::Two);
        assert_eq!(mv, vec![c(0, 3)]); // only the K beats the Q
        g.hands[1] = vec![c(0, 0), c(1, 5), c(2, 0)];
        assert_eq!(g.follow_cards(Player::Two), vec![c(0, 0)]); // follow without beating
        g.hands[1] = vec![c(1, 5), c(2, 0), c(2, 1)];
        assert_eq!(g.follow_cards(Player::Two), vec![c(2, 0), c(2, 1)]); // must trump
        g.hands[1] = vec![c(1, 5), c(3, 0)];
        assert_eq!(g.follow_cards(Player::Two).len(), 2); // anything
    }

    #[test]
    fn open_stock_allows_any_card() {
        let mut g = base();
        g.stock = vec![c(2, 1), c(1, 0), c(1, 1)];
        g.led = Some(c(0, 2));
        g.side = Player::Two;
        g.hands[1] = vec![c(0, 3), c(1, 5)];
        assert_eq!(g.follow_cards(Player::Two).len(), 2);
    }

    #[test]
    fn marriage_gives_20_or_40_and_needs_a_trick() {
        let mut g = base();
        g.stock = vec![c(2, 1), c(1, 0), c(1, 1)];
        g.hands[0] = vec![c(0, 2), c(0, 3), c(2, 2), c(2, 3), c(1, 0)];
        g.hands[1] = vec![c(3, 0)];
        // no trick yet: no announcement
        assert!(!g.legal_moves().contains(&Move::Play(c(0, 2), true)));
        g.tricks[0] = 1;
        assert!(g.legal_moves().contains(&Move::Play(c(0, 2), true)));
        assert!(!g.legal_moves().contains(&Move::Play(c(1, 0), true)));
        let mut a = g.clone();
        a.apply(&Move::Play(c(0, 2), true));
        assert_eq!(a.pts[0], 20);
        let mut b = g.clone();
        b.apply(&Move::Play(c(2, 3), true)); // trump marriage
        assert_eq!(b.pts[0], 40);
        assert_eq!(b.led, Some(c(2, 3)));
    }

    #[test]
    fn announcement_to_66_ends_the_hand() {
        let mut g = base();
        g.stock = vec![c(2, 1), c(1, 0), c(1, 1)];
        g.hands[0] = vec![c(2, 2), c(2, 3), c(1, 0)];
        g.hands[1] = vec![c(3, 0), c(3, 1)];
        g.tricks = [2, 3];
        g.pts = [30, 20];
        g.apply(&Move::Play(c(2, 3), true)); // +40 = 70
        assert_eq!(g.hand_result, Some((Player::One, 2))); // loser has 20 < 33 points
    }

    #[test]
    fn game_point_scale() {
        let mut g = base();
        g.tricks = [3, 2];
        g.pts = [66, 40];
        assert_eq!(g.game_points_for(Player::One), 1);
        g.pts = [66, 20];
        assert_eq!(g.game_points_for(Player::One), 2);
        g.tricks = [3, 0];
        g.pts = [66, 0];
        assert_eq!(g.game_points_for(Player::One), 3);
    }

    #[test]
    fn closer_failing_gives_opponent_two_or_three() {
        let mut g = base();
        g.closed = Some(Player::One);
        g.opp_tricks_at_close = 1;
        g.tricks = [2, 2];
        g.pts = [50, 66];
        assert_eq!(g.game_points_for(Player::Two), 2);
        g.opp_tricks_at_close = 0;
        assert_eq!(g.game_points_for(Player::Two), 3);
        // closer who makes 66 is scored normally
        g.pts = [66, 10];
        assert_eq!(g.game_points_for(Player::One), 2);
    }

    #[test]
    fn exchange_swaps_the_nine_of_trump() {
        let mut g = base();
        g.stock = vec![c(2, 5), c(1, 0), c(1, 1)];
        g.hands[0] = vec![c(2, 0), c(0, 0)];
        g.hands[1] = vec![c(3, 0)];
        assert!(g.legal_moves().contains(&Move::Exchange));
        g.apply(&Move::Exchange);
        assert!(g.hands[0].contains(&c(2, 5)));
        assert_eq!(g.stock[0], c(2, 0));
        assert!(!g.legal_moves().contains(&Move::Exchange));
        assert_eq!(g.to_move(), Player::One);
    }

    #[test]
    fn closing_stops_drawing() {
        let mut g = base();
        g.stock = vec![c(2, 5), c(1, 0), c(1, 1)];
        g.hands[0] = vec![c(0, 0), c(0, 1)];
        g.hands[1] = vec![c(0, 2), c(1, 2)];
        g.apply(&Move::Close);
        assert_eq!(g.closed, Some(Player::One));
        g.apply(&Move::Play(c(0, 0), false));
        g.apply(&Move::Play(c(0, 2), false)); // Q beats 9
        assert_eq!(g.stock.len(), 3);
        assert_eq!(g.hands[1].len(), 1);
        assert_eq!(g.pts[1], 3);
        assert_eq!(g.to_move(), Player::Two);
    }

    #[test]
    fn last_trick_wins_if_nobody_has_66() {
        let mut g = base();
        g.hands[0] = vec![c(0, 1)];
        g.hands[1] = vec![c(0, 3)];
        g.tricks = [3, 3];
        g.pts = [30, 40];
        g.apply(&Move::Play(c(0, 1), false));
        g.apply(&Move::Play(c(0, 3), false));
        assert_eq!(g.hand_result, Some((Player::Two, 1)));
        assert_eq!(g.legal_moves(), vec![Move::Next]);
    }

    #[test]
    fn render_hides_the_opponent() {
        let g = SixtySix::with_seed(5);
        let r = Game::render(&g);
        assert!(r.contains("Jucător 2 (ascuns) are 6 cărți"));
        for &x in &g.hands[1] {
            assert!(!r.contains(&card_symbol(x)) || g.hands[0].contains(&x) || g.stock[0] == x);
        }
    }

    #[test]
    fn random_matches_finish() {
        for seed in 1..=30u64 {
            let mut g = SixtySix::with_seed(seed);
            let mut rng = Rng::new(seed ^ 0xabc);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 20000);
            }
        }
    }

    #[test]
    fn evaluate_ignores_the_opponents_cards() {
        let g = SixtySix::with_seed(9);
        let mut h = g.clone();
        // swap what the opponent holds with stock cards: evaluate(One) is unchanged
        let (a, b) = (h.hands[1][0], h.stock[3]);
        h.hands[1][0] = b;
        h.stock[3] = a;
        assert_eq!(g.evaluate(Player::One), h.evaluate(Player::One));
    }

    #[test]
    fn ai_beats_a_random_mover_mostly() {
        use polytabletop_core::GameObj;
        let mut wins = 0;
        let n = 12;
        for seed in 1..=n {
            let mut g: Box<dyn GameObj> = Box::new(SixtySix::with_seed(seed));
            let mut rng = Rng::new(seed + 100);
            while g.outcome().is_none() {
                let mv = if g.to_move() == Player::One {
                    g.ai_move(rng.next_u64()).unwrap()
                } else {
                    let m = g.legal_moves();
                    m[rng.below(m.len())].clone()
                };
                g.play(&mv).unwrap();
            }
            if g.outcome() == Some(Outcome::Win(Player::One)) {
                wins += 1;
            }
        }
        assert!(wins * 10 >= n * 7, "AI won only {wins}/{n}");
    }
}
