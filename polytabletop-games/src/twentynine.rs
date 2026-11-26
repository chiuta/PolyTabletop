#![allow(clippy::doc_lazy_continuation)]
//! Twenty-Nine (29) (India / Pakistan / Asia de Sud), joc de levate cu licitație, în varianta pentru doi
//! jucători, după regulile publice (jocul original e pentru patru, în parteneriate).
//!
//! * Pachet de 32 de cărți (7–A). Ordinea în culoare: valet, 9, as, 10, rege, damă, 8, 7. Puncte:
//!   valet 3, 9 două, as 1, 10 unu (28 de puncte în tot pachetul, câte 7 pe culoare).
//! * **Adaptare pentru doi**: fiecare primește 8 cărți; cele 16 rămase nu se joacă și rămân ascunse.
//!   Contractul se exprimă tot în puncte, dar se numără doar punctele din cele 16 cărți jucate, așa că licitația
//!   merge de la 10 la 20.
//! * Licitația: primul jucător (alternativ la mâini) trebuie să liciteze (`licitez 10`…); celălalt urcă (`licitez N`) sau
//!   dă `pas`. Cine câștigă licitația alege atuul (`atu h|d|s|c`) și celălalt conduce prima levată.
//! * Răspunzi la culoare; dacă n-ai, poți tăia cu atu sau arunca altceva (fără obligația de a tăia).
//!   Câștigă levata cel mai mare atu sau cea mai mare carte din culoarea cerută.
//! * După 8 levate, dacă cel ce a licitat a adunat cel puțin cât a licitat, ia 1 punct de meci; altfel îl ia
//!   adversarul. Primul la 5 puncte de meci câștigă (cel mult 10 mâini, apoi cine are mai multe).
//! * Nu sunt implementate: atuul ascuns, perechea rege+damă („marriage”) și dublarea.
//! Informație ascunsă: AI-ul citește doar mâna lui și levatele jucate.
//! Mutări: `licitez 12`, `pas`, `atu s`, `Jh` (carte).

use crate::cards::{handover_banner, join_cards, shuffle, Card, Viewer, SUIT_LETTERS};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const MIN_BID: u8 = 10;
const MAX_BID: u8 = 20;
const TARGET: i32 = 5;
const MAX_HANDS: u8 = 10;

fn power(c: Card) -> u8 {
    match c.rank {
        11 => 8,
        9 => 7,
        14 => 6,
        10 => 5,
        13 => 4,
        12 => 3,
        8 => 2,
        _ => 1,
    }
}

fn pts(c: Card) -> i32 {
    match c.rank {
        11 => 3,
        9 => 2,
        14 | 10 => 1,
        _ => 0,
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Bid(u8),
    Pass,
    Trump(u8),
    Play(Card),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Bid,
    Trump,
    Play,
    Over,
}

#[derive(Clone)]
pub struct TwentyNine {
    hands: [Vec<Card>; 2],
    phase: Phase,
    side: Player,
    bidder: Player,
    bid: u8,
    first_bid_done: bool,
    trump: u8,
    table: Vec<Card>,
    leader: Player,
    pts: [i32; 2],
    game_pts: [i32; 2],
    hand_no: u8,
    rng: Rng,
    viewer: Viewer,
    log: String,
}

impl Default for TwentyNine {
    fn default() -> Self {
        Self::new()
    }
}

fn est(hand: &[Card], trump: Option<u8>) -> f64 {
    let mut e = 0.0;
    for &c in hand {
        let t = Some(c.suit) == trump;
        let w = match (t, c.rank) {
            (true, 11) | (true, 9) => 0.95,
            (true, _) => 0.55,
            (false, 11) => 0.55,
            (false, 9) | (false, 14) => 0.4,
            _ => 0.3,
        };
        e += pts(c) as f64 * w + if t { 0.6 } else { 0.0 };
    }
    e
}

impl TwentyNine {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = TwentyNine {
            hands: [Vec::new(), Vec::new()],
            phase: Phase::Bid,
            side: Player::One,
            bidder: Player::One,
            bid: 0,
            first_bid_done: false,
            trump: 0,
            table: Vec::new(),
            leader: Player::One,
            pts: [0; 2],
            game_pts: [0; 2],
            hand_no: 0,
            rng: Rng::new(seed),
            viewer: Viewer::Fixed(Player::One),
            log: String::new(),
        };
        g.deal();
        g
    }

    fn deal(&mut self) {
        let mut d = crate::cards::full_deck(7);
        shuffle(&mut d, &mut self.rng);
        self.hands = [d[0..8].to_vec(), d[8..16].to_vec()];
        for h in &mut self.hands {
            h.sort_by_key(|c| (c.suit, std::cmp::Reverse(power(*c))));
        }
        self.phase = Phase::Bid;
        self.side = if self.hand_no.is_multiple_of(2) {
            Player::One
        } else {
            Player::Two
        };
        self.bid = 0;
        self.first_bid_done = false;
        self.pts = [0; 2];
        self.table.clear();
    }

    fn legal_cards(&self, p: Player) -> Vec<Card> {
        let h = &self.hands[p.idx()];
        if let Some(&led) = self.table.first() {
            let same: Vec<Card> = h.iter().copied().filter(|c| c.suit == led.suit).collect();
            if !same.is_empty() {
                return same;
            }
        }
        h.clone()
    }

    fn trick_winner(&self) -> usize {
        let (a, b) = (self.table[0], self.table[1]);
        let beats = if b.suit == a.suit {
            power(b) > power(a)
        } else {
            b.suit == self.trump
        };
        usize::from(beats)
    }

    fn end_hand(&mut self) {
        let b = self.bidder.idx();
        let made = self.pts[b] >= self.bid as i32;
        let winner = if made { b } else { 1 - b };
        self.game_pts[winner] += 1;
        self.log = format!(
            "J{} a licitat {} și a adunat {}: {}.",
            b + 1,
            self.bid,
            self.pts[b],
            if made {
                "contract făcut (+1)"
            } else {
                "contract căzut (+1 pentru adversar)"
            }
        );
        self.hand_no += 1;
        if self.game_pts.iter().any(|&g| g >= TARGET) || self.hand_no >= MAX_HANDS {
            self.phase = Phase::Over;
        } else {
            self.deal();
        }
    }
}

impl Game for TwentyNine {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        match self.phase {
            Phase::Over => Vec::new(),
            Phase::Bid => {
                let lo = if self.bid == 0 { MIN_BID } else { self.bid + 1 };
                let mut v: Vec<Move> = (lo..=MAX_BID).map(Move::Bid).collect();
                if self.first_bid_done || self.bid > 0 {
                    v.push(Move::Pass);
                }
                if v.is_empty() {
                    v.push(Move::Pass);
                }
                v
            }
            Phase::Trump => (0..4).map(Move::Trump).collect(),
            Phase::Play => self
                .legal_cards(self.side)
                .into_iter()
                .map(Move::Play)
                .collect(),
        }
    }

    fn apply(&mut self, m: &Move) {
        match *m {
            Move::Bid(n) => {
                self.bid = n;
                self.bidder = self.side;
                self.first_bid_done = true;
                self.side = self.side.other();
            }
            Move::Pass => {
                self.side = self.bidder;
                self.phase = Phase::Trump;
            }
            Move::Trump(s) => {
                self.trump = s;
                self.leader = self.bidder.other();
                self.side = self.leader;
                self.phase = Phase::Play;
            }
            Move::Play(c) => {
                let p = self.side.idx();
                let k = self.hands[p].iter().position(|&x| x == c).unwrap();
                self.hands[p].remove(k);
                self.table.push(c);
                if self.table.len() == 1 {
                    self.side = self.side.other();
                } else {
                    let w = self.trick_winner();
                    let winner = if w == 0 {
                        self.leader
                    } else {
                        self.leader.other()
                    };
                    self.pts[winner.idx()] += pts(self.table[0]) + pts(self.table[1]);
                    self.table.clear();
                    self.leader = winner;
                    self.side = winner;
                    if self.hands[0].is_empty() {
                        self.end_hand();
                    }
                }
            }
        }
        if self.phase == Phase::Bid && self.bid == MAX_BID {
            // oferta maximă: adversarul poate doar da pas
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        Some(match self.game_pts[0].cmp(&self.game_pts[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let q = p.other();
        let base = (self.game_pts[p.idx()] - self.game_pts[q.idx()]) * 1000;
        match self.phase {
            Phase::Bid => {
                let e = est(&self.hands[p.idx()], None) + 4.0;
                // `p` tocmai a acționat: dacă a licitat, ultima ofertă e a lui; dacă a dat pas, oferta e a adversarului
                if self.bidder == p && self.first_bid_done {
                    base + ((e - self.bid as f64) * 40.0) as i32
                } else {
                    base - (20.0 * (e - self.bid as f64).max(0.0)) as i32
                }
            }
            Phase::Trump => {
                let best = (0..4u8)
                    .map(|s| est(&self.hands[p.idx()], Some(s)))
                    .fold(0.0, f64::max);
                base + if self.bidder == p {
                    (best * 10.0) as i32
                } else {
                    0
                }
            }
            _ => {
                let mut v = base + (self.pts[p.idx()] - self.pts[q.idx()]) * 30;
                v += (est(&self.hands[p.idx()], Some(self.trump)) * 5.0) as i32;
                if let (Some(&led), true) = (self.table.first(), self.leader == p) {
                    v -= pts(led) * 15
                        + if led.suit == self.trump { 20 } else { 0 }
                        + power(led) as i32 * 3;
                }
                v
            }
        }
    }

    fn render(&self) -> String {
        let me = self.viewer.who(self.to_move());
        let mut s = format!(
            "Twenty-Nine — mâna {}/{MAX_HANDS}. Puncte de meci: J1 {}, J2 {} (primul la {TARGET}).\n",
            (self.hand_no + 1).min(MAX_HANDS),
            self.game_pts[0],
            self.game_pts[1]
        );
        if let Viewer::HotSeat = self.viewer {
            s.push_str(&handover_banner(self.side));
        }
        for p in 0..2 {
            if p == me.idx() {
                s.push_str(&format!(
                    "J{} (mâna ta): {}\n",
                    p + 1,
                    join_cards(&self.hands[p])
                ));
            } else {
                s.push_str(&format!("J{}: {} cărți\n", p + 1, self.hands[p].len()));
            }
        }
        match self.phase {
            Phase::Bid => s.push_str(&format!(
                "Licitație: ultima ofertă {} (J{}). La rând J{}: `licitez N` ({MIN_BID}–{MAX_BID}) sau `pas`.\n",
                if self.bid == 0 { "—".into() } else { self.bid.to_string() },
                self.bidder.idx() + 1,
                self.side.idx() + 1
            )),
            Phase::Trump => s.push_str(&format!("J{} alege atuul: `atu h|d|s|c`.\n", self.side.idx() + 1)),
            Phase::Play | Phase::Over => {
                s.push_str(&format!(
                    "Contract: J{} cu {} (atu {}). Puncte în levate: J1 {}, J2 {}.\n",
                    self.bidder.idx() + 1,
                    self.bid,
                    SUIT_LETTERS[self.trump as usize],
                    self.pts[0],
                    self.pts[1]
                ));
                if !self.table.is_empty() {
                    s.push_str(&format!("Pe masă: {}\n", join_cards(&self.table)));
                }
            }
        }
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Bid(n) => format!("licitez {n}"),
            Move::Pass => "pas".into(),
            Move::Trump(s) => format!("atu {}", SUIT_LETTERS[*s as usize]),
            Move::Play(c) => c.text(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = match who {
            Some(p) => Viewer::Fixed(p),
            None => Viewer::HotSeat,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deck_points_and_bidding_flow() {
        let all: i32 = crate::cards::full_deck(7).iter().map(|&c| pts(c)).sum();
        assert_eq!(all, 28);
        let mut g = TwentyNine::with_seed(1);
        assert!(
            !g.legal_moves().contains(&Move::Pass),
            "primul trebuie să liciteze"
        );
        g.apply(&Move::Bid(12));
        assert!(g.legal_moves().contains(&Move::Pass));
        assert!(!g.legal_moves().contains(&Move::Bid(12)));
        g.apply(&Move::Pass);
        assert_eq!(g.phase, Phase::Trump);
        assert_eq!(g.side, Player::One);
    }

    #[test]
    fn must_follow_suit_but_may_discard_otherwise() {
        let mut g = TwentyNine::with_seed(2);
        g.hands[0] = vec![Card { rank: 11, suit: 0 }, Card { rank: 7, suit: 1 }];
        g.table = vec![Card { rank: 9, suit: 0 }];
        g.side = Player::One;
        g.phase = Phase::Play;
        assert_eq!(g.legal_moves().len(), 1);
        g.hands[0] = vec![Card { rank: 7, suit: 1 }, Card { rank: 8, suit: 2 }];
        assert_eq!(g.legal_moves().len(), 2);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=15u64 {
            let mut g = TwentyNine::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                assert!(!mv.is_empty(), "{}", g.render());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
                assert!(n < 2000);
            }
        }
    }

    #[test]
    fn hidden_hand_and_ai_legal() {
        let mut g = TwentyNine::with_seed(5);
        g.set_viewer(Some(Player::One));
        let a = g.render();
        g.set_viewer(Some(Player::Two));
        assert_ne!(a, g.render());
        let m = polytabletop_core::GameObj::ai_move(&g, 2).unwrap();
        assert!(g.legal_moves().iter().any(|x| g.format_move(x) == m));
    }
}
