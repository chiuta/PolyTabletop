//! Baloot (Arabia Saudită), variantă pentru 2 jucători, din regulile publice.
//!
//! * Pachet de 32 de cărți (7–A); fiecare primește 8, o carte se întoarce (stabilește culoarea
//!   propusă în runda 1) iar restul de 15 rămân nefolosite — variantă simplă pentru doi.
//! * Licitația: runda 1 — `hokm` (atu = culoarea întoarsă), `sun` (fără atu) sau `pas`; runda 2 —
//!   `hokm h|d|s|c` (altă culoare decât cea întoarsă), `sun` sau `pas`. Dacă ambii dau pas în
//!   runda 2, se dă din nou. Cumpărătorul trebuie să adune MAI MULTE puncte decât adversarul.
//! * Valori: Sun — A 11, 10 10, K 4, Q 3, J 2, restul 0. Hokm — în atu J 20, 9 14, A 11, 10 10,
//!   K 4, Q 3; celelalte culori ca la Sun. Ultima levată +10. Ordinea în atu: J, 9, A, 10, K, Q, 8, 7.
//! * Obligații la Hokm: răspunzi la culoare; dacă n-ai culoarea și ai atu, ești obligat să tai
//!   cu atu; la atu cerut, dacă poți, urci peste atu-ul adversarului.
//! * Puncte de joc: Hokm = puncte/10 (rotunjit), Sun = puncte/5 (dublu). Cumpărătorul care pierde
//!   dă tot adversarului. „Baloot” (K+Q de atu jucate de același jucător) = +2. Primul la 50 câștigă.
//!   Proiectele (sira, 50, 100, 400) și dublarea nu sunt incluse.
//!
//! Mutări: `imparte`, `pas`, `hokm`, `hokm h`, `sun`, `Jh` (carte). AI-ul citește doar propria mână.

use crate::cards::Viewer;
use crate::cards::{full_deck, join_cards, shuffle, Card};
use crate::cardsb_util::{has_suit, sort_hand, suit_name, trick_winner};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const TARGET: i32 = 50;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Deal,
    Bid1,
    Bid2,
    Play,
    Over,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Deal,
    Pass,
    Hokm(u8),
    Sun,
    Play(Card),
}

#[derive(Clone)]
pub struct Baloot {
    rng: Rng,
    viewer: Viewer,
    hands: [Vec<Card>; 2],
    upcard: Card,
    dealer: Player,
    phase: Phase,
    side: Player,
    trump: Option<u8>,
    buyer: Option<Player>,
    sun: bool,
    pts: [i32; 2],
    bonus: [i32; 2],
    king_q: [u8; 2],
    trick: Vec<(Player, Card)>,
    scores: [i32; 2],
    log: String,
}

impl Default for Baloot {
    fn default() -> Self {
        Self::new()
    }
}

fn card_value(c: Card, trump: Option<u8>) -> i32 {
    let is_t = Some(c.suit) == trump;
    match (c.rank, is_t) {
        (11, true) => 20,
        (9, true) => 14,
        (14, _) => 11,
        (10, _) => 10,
        (13, _) => 4,
        (12, _) => 3,
        (11, false) => 2,
        _ => 0,
    }
}

fn card_power(c: Card, trump: Option<u8>) -> u32 {
    let is_t = Some(c.suit) == trump;
    if is_t {
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
    } else {
        match c.rank {
            14 => 8,
            10 => 7,
            13 => 6,
            12 => 5,
            11 => 4,
            9 => 3,
            8 => 2,
            _ => 1,
        }
    }
}

fn hand_value(h: &[Card], trump: Option<u8>) -> i32 {
    let mut v: i32 = h.iter().map(|&c| card_value(c, trump)).sum();
    if let Some(t) = trump {
        v += 4 * h.iter().filter(|c| c.suit == t).count() as i32;
    }
    v
}

impl Baloot {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Baloot {
            rng: Rng::new(seed),
            viewer: Viewer::Fixed(Player::One),
            hands: [Vec::new(), Vec::new()],
            upcard: Card { rank: 7, suit: 0 },
            dealer: Player::Two,
            phase: Phase::Deal,
            side: Player::One,
            trump: None,
            buyer: None,
            sun: false,
            pts: [0, 0],
            bonus: [0, 0],
            king_q: [0, 0],
            trick: Vec::new(),
            scores: [0, 0],
            log: String::new(),
        };
        g.deal();
        g
    }

    pub fn score(&self) -> (i32, i32) {
        (self.scores[0], self.scores[1])
    }

    fn deal(&mut self) {
        let mut d = full_deck(7);
        shuffle(&mut d, &mut self.rng);
        for p in 0..2 {
            self.hands[p] = d.split_off(d.len() - 8);
            sort_hand(&mut self.hands[p]);
        }
        self.upcard = d.pop().unwrap();
        self.trump = None;
        self.buyer = None;
        self.sun = false;
        self.pts = [0, 0];
        self.bonus = [0, 0];
        self.king_q = [0, 0];
        self.trick.clear();
        self.phase = Phase::Bid1;
        self.side = self.dealer.other();
    }

    fn start_play(&mut self, buyer: Player) {
        self.buyer = Some(buyer);
        self.phase = Phase::Play;
        self.side = self.dealer.other();
    }
}

impl Game for Baloot {
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
            Phase::Deal => vec![Move::Deal],
            Phase::Bid1 => vec![Move::Pass, Move::Hokm(self.upcard.suit), Move::Sun],
            Phase::Bid2 => {
                let mut v = vec![Move::Pass, Move::Sun];
                for s in 0..4u8 {
                    if s != self.upcard.suit {
                        v.push(Move::Hokm(s));
                    }
                }
                v
            }
            Phase::Play => {
                let h = &self.hands[self.side.idx()];
                let eff = |c: Card| c.suit;
                let all = || h.iter().map(|&c| Move::Play(c)).collect::<Vec<_>>();
                let Some(&(_, lead)) = self.trick.first() else {
                    return all();
                };
                let ls = lead.suit;
                if has_suit(h, ls, &eff) {
                    let follow: Vec<Card> = h.iter().copied().filter(|c| c.suit == ls).collect();
                    if Some(ls) == self.trump {
                        let lp = card_power(lead, self.trump);
                        let higher: Vec<Card> = follow
                            .iter()
                            .copied()
                            .filter(|&c| card_power(c, self.trump) > lp)
                            .collect();
                        if !higher.is_empty() {
                            return higher.into_iter().map(Move::Play).collect();
                        }
                    }
                    return follow.into_iter().map(Move::Play).collect();
                }
                if let Some(t) = self.trump {
                    if has_suit(h, t, &eff) {
                        return h
                            .iter()
                            .filter(|c| c.suit == t)
                            .map(|&c| Move::Play(c))
                            .collect();
                    }
                }
                all()
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        match *m {
            Move::Deal => self.deal(),
            Move::Pass => {
                if self.side == self.dealer {
                    if self.phase == Phase::Bid1 {
                        self.phase = Phase::Bid2;
                        self.side = self.dealer.other();
                    } else {
                        // nimeni nu cumpără: se dă din nou, cu celălalt dealer
                        self.dealer = self.dealer.other();
                        self.phase = Phase::Deal;
                        self.side = self.dealer.other();
                    }
                } else {
                    self.side = self.dealer;
                }
            }
            Move::Hokm(s) => {
                self.trump = Some(s);
                let b = self.side;
                self.start_play(b);
            }
            Move::Sun => {
                self.sun = true;
                let b = self.side;
                self.start_play(b);
            }
            Move::Play(c) => {
                let p = self.side;
                let pos = self.hands[p.idx()].iter().position(|&x| x == c).unwrap();
                self.hands[p.idx()].remove(pos);
                if Some(c.suit) == self.trump && (c.rank == 12 || c.rank == 13) {
                    self.king_q[p.idx()] += 1;
                    if self.king_q[p.idx()] == 2 {
                        self.bonus[p.idx()] += 2;
                        self.log = format!("Baloot! J{} +2", p.idx() + 1);
                    }
                }
                self.trick.push((p, c));
                if self.trick.len() < 2 {
                    self.side = p.other();
                    return;
                }
                let cards: Vec<Card> = self.trick.iter().map(|&(_, c)| c).collect();
                let t = self.trump;
                let w = trick_winner(&cards, t, &|c| c.suit, &|c| card_power(c, t));
                let winner = self.trick[w].0;
                let mut gain: i32 = cards.iter().map(|&c| card_value(c, t)).sum();
                if self.hands[0].is_empty() {
                    gain += 10;
                }
                self.pts[winner.idx()] += gain;
                self.trick.clear();
                self.side = winner;
                if self.hands[0].is_empty() {
                    self.finish_hand();
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        Some(if self.scores[0] > self.scores[1] {
            Outcome::Win(Player::One)
        } else {
            Outcome::Win(Player::Two)
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let q = p.other();
        let mut s = 100 * (self.scores[p.idx()] - self.scores[q.idx()]);
        let mine = &self.hands[p.idx()];
        if self.phase == Phase::Play {
            {
                let t = self.trump;
                let avg = if self.sun { 30 } else { 38 };
                if self.trick.is_empty() && mine.len() == 8 {
                    // început de mână: cine a cumpărat și cât valorează mâna lui p
                    let hv = hand_value(mine, t);
                    s += (hv - avg) * if self.buyer == Some(p) { 3 } else { 2 };
                } else {
                    s += 3 * (self.pts[p.idx()] - self.pts[q.idx()]);
                    for &c in mine {
                        s += card_value(c, t) / 2 + card_power(c, t) as i32;
                    }
                    if let Some(&(w, c)) = self.trick.first() {
                        let st = card_value(c, t) + 2 * card_power(c, t) as i32;
                        s += if w == p { st } else { -st };
                    }
                }
            }
        }
        s
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Baloot — scor: J1 {}, J2 {} (până la {}) | dealer: J{}\n",
            self.scores[0],
            self.scores[1],
            TARGET,
            self.dealer.idx() + 1
        );
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        if self.phase == Phase::Over {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        if matches!(self.phase, Phase::Bid1 | Phase::Bid2) {
            s.push_str(&format!("Cartea întoarsă: {}\n", self.upcard.text()));
        } else if let Some(b) = self.buyer {
            s.push_str(&format!(
                "Contract: {} cumpărat de J{} | puncte de carte: J1 {}, J2 {}\n",
                if self.sun {
                    "SUN".to_string()
                } else {
                    format!("HOKM {}", suit_name(self.trump.unwrap()))
                },
                b.idx() + 1,
                self.pts[0],
                self.pts[1]
            ));
        }
        if !self.trick.is_empty() {
            let cs: Vec<Card> = self.trick.iter().map(|&(_, c)| c).collect();
            s.push_str(&format!("Pe masă: {}\n", join_cards(&cs)));
        }
        let v = self.viewer.who(self.side);
        let o = v.other();
        s.push_str(&format!(
            "Mâna ta (J{}): {}\n",
            v.idx() + 1,
            join_cards(&self.hands[v.idx()])
        ));
        s.push_str(&format!(
            "J{} are {} cărți\n",
            o.idx() + 1,
            self.hands[o.idx()].len()
        ));
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Deal => "imparte".into(),
            Move::Pass => "pas".into(),
            Move::Hokm(s) => {
                if self.phase == Phase::Bid1 {
                    "hokm".into()
                } else {
                    format!("hokm {}", ['h', 'd', 's', 'c'][*s as usize])
                }
            }
            Move::Sun => "sun".into(),
            Move::Play(c) => c.text(),
        }
    }

    /// Adâncime 0 (o singură mutare): `AlphaBeta(1)` ar fi extins la Hard/Expert răspunsul adversarului din
    /// mâna lui reală (și următoarea împărțire); evaluarea citește doar mâna proprie și datele publice.
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

impl Baloot {
    fn finish_hand(&mut self) {
        let b = self.buyer.unwrap();
        let o = b.other();
        let total = self.pts[0] + self.pts[1];
        let conv = |x: i32| -> i32 {
            if self.sun {
                (2 * x + 5) / 10
            } else {
                (x + 5) / 10
            }
        };
        let made = self.pts[b.idx()] > self.pts[o.idx()];
        if made {
            self.scores[b.idx()] += conv(self.pts[b.idx()]);
            self.scores[o.idx()] += conv(self.pts[o.idx()]);
        } else {
            self.scores[o.idx()] += conv(total);
        }
        self.scores[0] += self.bonus[0];
        self.scores[1] += self.bonus[1];
        self.log = format!(
            "Mână încheiată: cumpărător J{} {} ({} – {})",
            b.idx() + 1,
            if made {
                "a reușit"
            } else {
                "a pierdut (khasara)"
            },
            self.pts[0],
            self.pts[1]
        );
        if (self.scores[0] >= TARGET || self.scores[1] >= TARGET)
            && self.scores[0] != self.scores[1]
        {
            self.phase = Phase::Over;
        } else {
            self.dealer = self.dealer.other();
            self.phase = Phase::Deal;
            self.side = self.dealer.other();
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
    fn values() {
        let t = Some(0u8);
        assert_eq!(card_value(c(11, 0), t), 20);
        assert_eq!(card_value(c(9, 0), t), 14);
        assert_eq!(card_value(c(11, 1), t), 2);
        assert_eq!(card_value(c(9, 1), t), 0);
        assert!(card_power(c(11, 0), t) > card_power(c(9, 0), t));
        assert!(card_power(c(9, 0), t) > card_power(c(14, 0), t));
        // total Hokm = 152 (+10 ultima)
        let mut tot = 0;
        for s in 0..4 {
            for r in 7..=14 {
                tot += card_value(c(r, s), t);
            }
        }
        assert_eq!(tot, 152);
        assert_eq!(
            (0..4)
                .flat_map(|s| (7..=14).map(move |r| card_value(c(r, s), None)))
                .sum::<i32>(),
            120
        );
    }

    #[test]
    fn deal_and_bidding_rounds() {
        let mut g = Baloot::with_seed(2);
        assert_eq!(g.hands[0].len(), 8);
        g.apply(&Move::Pass);
        g.apply(&Move::Pass);
        assert_eq!(g.phase, Phase::Bid2);
        assert_eq!(g.legal_moves().len(), 2 + 3);
        g.apply(&Move::Pass);
        g.apply(&Move::Pass);
        assert_eq!(g.phase, Phase::Deal);
        g.apply(&Move::Deal);
        assert_eq!(g.phase, Phase::Bid1);
    }

    #[test]
    fn must_trump_when_void() {
        let mut g = Baloot::with_seed(1);
        g.phase = Phase::Play;
        g.trump = Some(0);
        g.hands = [vec![c(8, 0), c(10, 1)], vec![c(9, 2)]];
        g.trick = vec![(Player::Two, c(9, 2))];
        g.side = Player::One;
        assert_eq!(g.legal_moves(), vec![Move::Play(c(8, 0))]);
    }

    #[test]
    fn must_overtrump() {
        let mut g = Baloot::with_seed(1);
        g.phase = Phase::Play;
        g.trump = Some(0);
        g.hands = [vec![c(7, 0), c(11, 0), c(13, 0)], vec![]];
        g.trick = vec![(Player::Two, c(10, 0))];
        g.side = Player::One;
        let l = g.legal_moves();
        assert!(l.contains(&Move::Play(c(11, 0))));
        assert!(
            !l.contains(&Move::Play(c(13, 0)))
                || card_power(c(13, 0), Some(0)) > card_power(c(10, 0), Some(0))
        );
        assert!(!l.contains(&Move::Play(c(7, 0))));
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=10u64 {
            let mut g = Baloot::with_seed(seed);
            let mut rng = Rng::new(seed + 3);
            let mut n = 0;
            while g.outcome().is_none() && n < 20000 {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed}");
        }
    }
}

#[cfg(test)]
mod a2_audit {
    use super::*;

    #[test]
    fn ai_nu_vede_mana_adversarului_la_orice_nivel() {
        crate::p3_comun::ai_blind_check(
            "baloot",
            Baloot::with_seed,
            |g, me, seed| {
                let o = me.other().idx();
                let mut pool: Vec<Card> = full_deck(7)
                    .into_iter()
                    .filter(|c| {
                        !g.hands[me.idx()].contains(c) && !g.trick.iter().any(|&(_, x)| x == *c)
                    })
                    .collect();
                crate::p3_comun::shuffle_with(&mut pool, seed);
                let n = g.hands[o].len();
                g.hands[o] = pool.into_iter().take(n).collect();
                sort_hand(&mut g.hands[o]);
                g.rng = Rng::new(seed ^ 0xBA);
            },
            12,
        );
    }
}
