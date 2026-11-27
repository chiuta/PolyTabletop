//! Tarneeb (Orientul Mijlociu), variantă pentru 2 jucători, din regulile publice.
//!
//! * 52 de cărți; fiecare primește 13, celelalte 26 rămân nefolosite și ascunse (simplificare
//!   față de jocul în patru). Ordinea cărților: A (cea mai mare) … 2.
//! * Licitația: se licitează numărul de levate (7–13), urcând cu 1 sau 2 la fiecare tură; după un
//!   `pas` licitația se termină; dacă primul spune pas, dealerul e obligat să liciteze 7.
//!   Câștigătorul licitației numește atuul și conduce prima levată.
//! * Se răspunde la culoare; atu bate tot. La final: dacă licitatorul a luat cel puțin cât a
//!   licitat, primește numărul de levate luate (16 dacă a licitat 13 și l-a făcut); altfel pierde
//!   cât a licitat. Adversarul primește levatele luate (+3 bonus dacă licitatorul a căzut). Prima persoană la 31 de puncte câștigă.
//!
//! Mutări: `imparte`, `pas`, `7`…`13` (licitație), `atu h` (h/d/s/c), `Ah` (carte).
//! Informația ascunsă: AI-ul citește doar propria mână.

use crate::cards::Viewer;
use crate::cards::{full_deck, join_cards, shuffle, Card};
use crate::cardsb_util::{has_suit, sort_hand, suit_name, trick_winner};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const TARGET: i32 = 31;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Deal,
    Bid,
    Trump,
    Play,
    Over,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Deal,
    Pass,
    Bid(u8),
    Trump(u8),
    Play(Card),
}

#[derive(Clone)]
pub struct Tarneeb {
    rng: Rng,
    viewer: Viewer,
    hands: [Vec<Card>; 2],
    dealer: Player,
    phase: Phase,
    side: Player,
    high: u8,
    bidder: Option<Player>,
    trump: Option<u8>,
    tricks: [u8; 2],
    trick: Vec<(Player, Card)>,
    scores: [i32; 2],
    log: String,
}

impl Default for Tarneeb {
    fn default() -> Self {
        Self::new()
    }
}

/// Estimare a levatelor cu atu `t` pentru o mână de 13 cărți.
fn est_tricks(hand: &[Card], t: u8) -> f32 {
    let hcp: i32 = hand
        .iter()
        .map(|c| match c.rank {
            14 => 4,
            13 => 3,
            12 => 2,
            11 => 1,
            _ => 0,
        })
        .sum();
    let nt = hand.iter().filter(|c| c.suit == t).count() as f32;
    6.5 + 0.35 * (hcp as f32 - 10.0) + 0.6 * (nt - 3.25)
}

fn best_trump(hand: &[Card]) -> (u8, f32) {
    let mut best = (0u8, f32::MIN);
    for t in 0..4u8 {
        let e = est_tricks(hand, t);
        if e > best.1 {
            best = (t, e);
        }
    }
    best
}

impl Tarneeb {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Tarneeb {
            rng: Rng::new(seed),
            viewer: Viewer::Fixed(Player::One),
            hands: [Vec::new(), Vec::new()],
            dealer: Player::Two,
            phase: Phase::Deal,
            side: Player::One,
            high: 0,
            bidder: None,
            trump: None,
            tricks: [0, 0],
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
        let mut d = full_deck(2);
        shuffle(&mut d, &mut self.rng);
        for p in 0..2 {
            self.hands[p] = d.split_off(d.len() - 13);
            sort_hand(&mut self.hands[p]);
        }
        self.high = 0;
        self.bidder = None;
        self.trump = None;
        self.tricks = [0, 0];
        self.trick.clear();
        self.phase = Phase::Bid;
        self.side = self.dealer.other();
    }
}

impl Game for Tarneeb {
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
            Phase::Bid => {
                let mut v = Vec::new();
                // dealerul nu poate da pas dacă nu s-a licitat încă
                if self.bidder.is_some() {
                    v.push(Move::Pass);
                } else if self.side == self.dealer {
                    // obligat să liciteze
                } else {
                    v.push(Move::Pass);
                }
                for b in (self.high.max(6) + 1)..=(self.high.max(6) + 2).min(13) {
                    v.push(Move::Bid(b));
                }
                v
            }
            Phase::Trump => (0..4u8).map(Move::Trump).collect(),
            Phase::Play => {
                let h = &self.hands[self.side.idx()];
                let eff = |c: Card| c.suit;
                let must = self
                    .trick
                    .first()
                    .map(|&(_, c)| c.suit)
                    .filter(|&s| has_suit(h, s, &eff));
                h.iter()
                    .filter(|c| must.is_none_or(|s| c.suit == s))
                    .map(|&c| Move::Play(c))
                    .collect()
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        match *m {
            Move::Deal => self.deal(),
            Move::Pass => {
                if let Some(b) = self.bidder {
                    // licitația s-a terminat
                    self.phase = Phase::Trump;
                    self.side = b;
                } else {
                    // primul a dat pas; urmează dealerul (obligat)
                    self.side = self.side.other();
                }
            }
            Move::Bid(n) => {
                self.high = n;
                self.bidder = Some(self.side);
                self.side = self.side.other();
            }
            Move::Trump(s) => {
                self.trump = Some(s);
                self.phase = Phase::Play;
                self.side = self.bidder.unwrap();
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
                let cards: Vec<Card> = self.trick.iter().map(|&(_, c)| c).collect();
                let w = trick_winner(&cards, self.trump, &|c| c.suit, &|c| c.rank as u32);
                let winner = self.trick[w].0;
                self.tricks[winner.idx()] += 1;
                self.trick.clear();
                self.side = winner;
                if self.hands[0].is_empty() {
                    let b = self.bidder.unwrap();
                    let bt = self.tricks[b.idx()];
                    let delta = if bt >= self.high {
                        if self.high == 13 {
                            16
                        } else {
                            bt as i32
                        }
                    } else {
                        -(self.high as i32)
                    };
                    self.scores[b.idx()] += delta;
                    let set_bonus = if bt >= self.high { 0 } else { 3 };
                    self.scores[b.other().idx()] += self.tricks[b.other().idx()] as i32 + set_bonus;
                    self.log = format!(
                        "Mână încheiată: J{} a licitat {} și a luat {} → {:+}",
                        b.idx() + 1,
                        self.high,
                        bt,
                        delta
                    );
                    let (a, c2) = (self.scores[0], self.scores[1]);
                    if (a >= TARGET || c2 >= TARGET) && a != c2 {
                        self.phase = Phase::Over;
                    } else {
                        self.dealer = self.dealer.other();
                        self.phase = Phase::Deal;
                        self.side = self.dealer.other();
                    }
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
        let mut s = 50 * (self.scores[p.idx()] - self.scores[q.idx()]);
        let mine = &self.hands[p.idx()];
        match self.phase {
            Phase::Trump if self.bidder == Some(p) => {
                let (_, e) = best_trump(mine);
                s += ((e - self.high as f32) * 30.0) as i32 + self.high as i32 * 5;
            }
            Phase::Trump => {
                // adversarul a câștigat licitația: cu cât licitează mai sus, cu atât e mai bine pentru mine
                s += (self.high as i32 - 7) * 30;
            }
            Phase::Bid if self.bidder == Some(q) => {
                // am dat/ voi da răspuns la licitația adversarului (după pas, faza devine Trump)
                s += (self.high as i32 - 7) * 30;
            }
            Phase::Bid if self.bidder == Some(p) => {
                let (_, e) = best_trump(mine);
                s += ((e - self.high as f32) * 30.0) as i32 + self.high as i32 * 5;
            }
            Phase::Play => {
                if let Some(t) = self.trump {
                    s += 100 * (self.tricks[p.idx()] as i32 - self.tricks[q.idx()] as i32);
                    // valoare de mână: atu și cărți mari contează
                    for c in mine {
                        s += c.rank as i32 / 2 + if c.suit == t { c.rank as i32 / 2 } else { 0 };
                    }
                    if let Some(&(w, c)) = self.trick.first() {
                        let st = c.rank as i32 * 2 + if c.suit == t { 30 } else { 0 };
                        s += if w == p { st } else { -st };
                    }
                }
            }
            _ => {}
        }
        s
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Tarneeb — scor: J1 {}, J2 {} (până la {}) | dealer: J{}\n",
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
        if let Some(b) = self.bidder {
            s.push_str(&format!(
                "Licitație curentă: J{} cu {}\n",
                b.idx() + 1,
                self.high
            ));
        }
        if let Some(t) = self.trump {
            s.push_str(&format!(
                "Atu: {} | levate: J1 {}, J2 {}\n",
                suit_name(t),
                self.tricks[0],
                self.tricks[1]
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
            Move::Bid(n) => n.to_string(),
            Move::Trump(s) => format!("atu {}", ['h', 'd', 's', 'c'][*s as usize]),
            Move::Play(c) => c.text(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(rank: u8, suit: u8) -> Card {
        Card { rank, suit }
    }

    #[test]
    fn deal_13_each_distinct() {
        let g = Tarneeb::with_seed(3);
        assert_eq!(g.hands[0].len(), 13);
        assert_eq!(g.hands[1].len(), 13);
        let mut all = g.hands[0].clone();
        all.extend(&g.hands[1]);
        all.sort_by_key(|c| (c.suit, c.rank));
        all.dedup();
        assert_eq!(all.len(), 26);
    }

    #[test]
    fn dealer_must_bid_after_pass() {
        let mut g = Tarneeb::with_seed(1);
        let first = g.side;
        assert!(g.legal_moves().contains(&Move::Pass));
        g.apply(&Move::Pass);
        assert_ne!(g.side, first);
        assert!(!g.legal_moves().contains(&Move::Pass));
        assert_eq!(g.legal_moves()[0], Move::Bid(7));
    }

    #[test]
    fn bid_then_pass_ends_auction() {
        let mut g = Tarneeb::with_seed(1);
        g.apply(&Move::Bid(8));
        assert!(!g.legal_moves().contains(&Move::Bid(8)));
        assert!(g.legal_moves().contains(&Move::Bid(10)));
        g.apply(&Move::Pass);
        assert_eq!(g.phase, Phase::Trump);
        assert_eq!(g.side, g.bidder.unwrap());
    }

    #[test]
    fn scoring_made_and_failed() {
        let mut g = Tarneeb::with_seed(1);
        g.phase = Phase::Play;
        g.bidder = Some(Player::One);
        g.high = 8;
        g.trump = Some(0);
        g.tricks = [8, 4];
        g.hands = [vec![], vec![c(9, 1)]];
        g.trick = vec![(Player::One, c(14, 1))];
        g.side = Player::Two;
        g.apply(&Move::Play(c(9, 1)));
        // J1 a luat și ultima: 9 levate ≥ 8 → +9; J2 4
        assert_eq!(g.scores, [9, 4]);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=10u64 {
            let mut g = Tarneeb::with_seed(seed);
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
mod a1_blind {
    use super::*;
    use crate::ai_audit_a1::{audit, fake_cards};

    #[test]
    fn ai_ignores_opponent_hand_at_all_levels() {
        audit("tarneeb", Tarneeb::with_seed, 1..=4, 3, 70, |g, salt| {
            let foe = g.side.other().idx();
            let mut ex: Vec<Card> = g.hands[g.side.idx()].clone();
            ex.extend(g.trick.iter().map(|t| t.1));
            let fake = fake_cards(full_deck(2), &ex, g.hands[foe].len(), salt);
            if fake.len() == g.hands[foe].len() {
                g.hands[foe] = fake;
                sort_hand(&mut g.hands[foe]);
            }
            g.rng = Rng::new(salt ^ 3);
        });
    }
}
