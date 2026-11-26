//! Pică — levate cu licitație și pică atu (tip „Spades”), variantă pentru 2 jucători.
//!
//! Reguli din descrierile publice ale jocurilor de tip Spades (nume generic; nicio marcă):
//! * 52 de cărți, câte 13 fiecăruia, 26 nefolosite și nevăzute. Atu permanent: pica (s).
//! * Licitație (o singură rundă, nu se vede cartea celuilalt): fiecare spune câte levate ia
//!   (`licit 1`…`licit 13`) sau `licit 0` = „nil” (promite zero levate).
//! * Culoare obligatorie; atu bate tot; nu se conduce pică până nu s-a jucat una (decât dacă
//!   ai doar pică). Câștigătorul levatei conduce.
//! * Scor pe mână: bid ≥ 1 reușit = 10×bid + 1 pe fiecare levată în plus („bag”); picat = −10×bid.
//!   Nil reușit +50, picat −50. La 10 bag-uri adunate: −100. Se joacă până la 150 de puncte
//!   (sau −150, sau maximum 12 mâini; atunci câștigă scorul mai mare).
//!
//! Mutări: `imparte`, `licit N`, carte (`As`, `7h`). AI-ul citește doar propria mână, licitațiile,
//! levatele (publice) și numărul cărților adversarului.

use crate::cards::{join_cards, Card, Viewer};
use crate::trickkit::{beats, deal, follow, hand_power, hcp};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const TARGET: i32 = 150;
const MAX_HANDS: u8 = 12;
const TRUMP: Option<u8> = Some(2);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Deal,
    Bid,
    Play,
    Over,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Deal,
    Bid(u8),
    Play(Card),
}

#[derive(Clone)]
pub struct Pica {
    rng: Rng,
    hands: [Vec<Card>; 2],
    stock: Vec<Card>,
    dealer: Player,
    phase: Phase,
    side: Player,
    bids: [Option<u8>; 2],
    trick: Vec<(Player, Card)>,
    tricks: [u8; 2],
    scores: [i32; 2],
    bags: [i32; 2],
    broken: bool,
    played_hands: u8,
    log: String,
    viewer: Viewer,
}

impl Default for Pica {
    fn default() -> Self {
        Self::new()
    }
}

/// Estimare grosieră a levatelor unei mâini de 13 cărți contra unui singur adversar.
pub fn est_tricks(h: &[Card]) -> f32 {
    let sp = h.iter().filter(|c| c.suit == 2).count() as f32;
    (6.5 + 0.3 * (hcp(h) as f32 - 10.0) + 0.4 * (sp - 3.25)).clamp(0.0, 13.0)
}

fn bid_cost(b: u8, est: f32) -> f32 {
    if b == 0 {
        150.0 * (est - 1.0).max(0.0) - 20.0
    } else {
        80.0 * (b as f32 - est).abs()
    }
}

/// Valoarea (în sutimi de punct) a stării unei mâini pentru un jucător, din informație publică.
fn trick_value(bid: u8, t: u8) -> f32 {
    let (b, t) = (bid as f32, t as f32);
    if bid == 0 {
        if t > 0.0 {
            -1000.0
        } else {
            50.0
        }
    } else {
        100.0 * t.min(b) - 15.0 * (t - b).max(0.0)
    }
}

impl Pica {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Pica {
            rng: Rng::new(seed),
            hands: [Vec::new(), Vec::new()],
            stock: Vec::new(),
            dealer: Player::Two,
            phase: Phase::Deal,
            side: Player::One,
            bids: [None, None],
            trick: Vec::new(),
            tricks: [0, 0],
            scores: [0, 0],
            bags: [0, 0],
            broken: false,
            played_hands: 0,
            log: String::new(),
            viewer: Viewer::Fixed(Player::One),
        };
        g.deal_hand();
        g
    }

    pub fn scores(&self) -> (i32, i32) {
        (self.scores[0], self.scores[1])
    }

    fn deal_hand(&mut self) {
        let (h, s) = deal(&mut self.rng, 13);
        self.hands = h;
        self.stock = s;
        self.bids = [None, None];
        self.tricks = [0, 0];
        self.broken = false;
        self.trick.clear();
        self.phase = Phase::Bid;
        self.side = self.dealer.other();
    }

    /// Mâna unui jucător (pentru teste și interfețe care au voie să o vadă).
    pub fn hand_of(&self, p: Player) -> &[Card] {
        &self.hands[p.idx()]
    }

    /// Pentru teste: reamestecă ce nu vede `viewer`.
    pub fn scramble_hidden(&mut self, viewer: Player, seed: u64) {
        crate::trickkit::scramble_hidden(&mut self.hands, &mut self.stock, viewer, &[], seed);
    }

    fn finish_hand(&mut self) {
        let mut parts = Vec::new();
        for i in 0..2 {
            let (b, t) = (self.bids[i].unwrap() as i32, self.tricks[i] as i32);
            let mut d = if b == 0 {
                if t == 0 {
                    50
                } else {
                    -50
                }
            } else if t >= b {
                self.bags[i] += t - b;
                10 * b + (t - b)
            } else {
                -10 * b
            };
            while self.bags[i] >= 10 {
                self.bags[i] -= 10;
                d -= 100;
            }
            self.scores[i] += d;
            parts.push(format!("J{} {:+}", i + 1, d));
        }
        self.log = format!("Mână încheiată: {}", parts.join(", "));
        self.played_hands += 1;
        let over = self.scores.iter().any(|&s| s >= TARGET || s <= -TARGET)
            || self.played_hands >= MAX_HANDS;
        if (over && self.scores[0] != self.scores[1]) || self.played_hands >= MAX_HANDS {
            self.phase = Phase::Over;
        } else {
            self.dealer = self.dealer.other();
            self.phase = Phase::Deal;
            self.side = self.dealer.other();
        }
    }
}

impl Game for Pica {
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
            Phase::Bid => (0..=13).map(Move::Bid).collect(),
            Phase::Play => {
                let h = &self.hands[self.side.idx()];
                let mut v = follow(h, &self.trick);
                if self.trick.is_empty() && !self.broken && v.iter().any(|c| c.suit != 2) {
                    v.retain(|c| c.suit != 2);
                }
                v.into_iter().map(Move::Play).collect()
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        match *m {
            Move::Deal => self.deal_hand(),
            Move::Bid(b) => {
                self.bids[self.side.idx()] = Some(b);
                if self.bids[self.side.other().idx()].is_some() {
                    self.phase = Phase::Play;
                    self.side = self.dealer.other();
                } else {
                    self.side = self.side.other();
                }
            }
            Move::Play(c) => {
                let p = self.side;
                let pos = self.hands[p.idx()].iter().position(|&x| x == c).unwrap();
                self.hands[p.idx()].remove(pos);
                if c.suit == 2 {
                    self.broken = true;
                }
                self.trick.push((p, c));
                if self.trick.len() < 2 {
                    self.side = p.other();
                    return;
                }
                let (lp, lc) = self.trick[0];
                let (rp, rc) = self.trick[1];
                let w = if beats(lc, rc, TRUMP) { rp } else { lp };
                self.tricks[w.idx()] += 1;
                self.trick.clear();
                self.side = w;
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
        Some(match self.scores[0].cmp(&self.scores[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let (pi, qi) = (p.idx(), p.other().idx());
        let mut s = 10.0 * (self.scores[pi] - self.scores[qi]) as f32;
        match self.phase {
            Phase::Bid => {
                if let Some(b) = self.bids[pi] {
                    s -= bid_cost(b, est_tricks(&self.hands[pi]));
                }
            }
            Phase::Play if self.hands[pi].len() == 13 && self.trick.is_empty() => {
                // licitația tocmai s-a încheiat: contează doar cât de bună e oferta mea
                s -= bid_cost(self.bids[pi].unwrap(), est_tricks(&self.hands[pi]));
            }
            Phase::Play => {
                let (bm, bo) = (self.bids[pi].unwrap(), self.bids[qi].unwrap());
                s += trick_value(bm, self.tricks[pi]) - trick_value(bo, self.tricks[qi]);
                let need = bm > 0 && self.tricks[pi] < bm;
                let pw = hand_power(&self.hands[pi], TRUMP);
                let nil = bm == 0;
                let w = if need {
                    40.0
                } else if nil {
                    -80.0
                } else {
                    -20.0
                };
                s += w * pw;
                if let Some(&(lp, c)) = self.trick.first() {
                    let pwin = crate::trickkit::p_win(c, TRUMP);
                    if lp == p {
                        s += if need { 60.0 * pwin } else { -60.0 * pwin };
                    }
                }
            }
            _ => {}
        }
        s as i32
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Pică — scor: J1 {} (bag {}), J2 {} (bag {}) | până la {} | mâna {}/{} | dealer: J{}\n",
            self.scores[0],
            self.bags[0],
            self.scores[1],
            self.bags[1],
            TARGET,
            self.played_hands + 1,
            MAX_HANDS,
            self.dealer.idx() + 1
        );
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        if self.phase == Phase::Over {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        let bid_txt = |b: Option<u8>| match b {
            None => "—".to_string(),
            Some(0) => "nil".to_string(),
            Some(n) => n.to_string(),
        };
        s.push_str(&format!(
            "Licitații: J1 {}, J2 {} | levate: J1 {}, J2 {} | pică {}\n",
            bid_txt(self.bids[0]),
            bid_txt(self.bids[1]),
            self.tricks[0],
            self.tricks[1],
            if self.broken { "sparte" } else { "nesparte" }
        ));
        if !self.trick.is_empty() {
            let cs: Vec<Card> = self.trick.iter().map(|&(_, c)| c).collect();
            s.push_str(&format!("Pe masă: {}\n", join_cards(&cs)));
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
            Move::Deal => "imparte".into(),
            Move::Bid(b) => format!("licit {b}"),
            Move::Play(c) => c.text(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(rank: u8, suit: u8) -> Card {
        Card { rank, suit }
    }

    #[test]
    fn spades_trump_and_not_led_until_broken() {
        let mut g = Pica::with_seed(1);
        g.phase = Phase::Play;
        g.bids = [Some(2), Some(2)];
        g.hands = [vec![c(3, 2), c(4, 0)], vec![c(14, 0), c(2, 2)]];
        g.side = Player::One;
        assert_eq!(g.legal_moves(), vec![Move::Play(c(4, 0))]);
        g.apply(&Move::Play(c(4, 0)));
        g.apply(&Move::Play(c(14, 0)));
        assert_eq!(g.tricks, [0, 1]);
    }

    #[test]
    fn scoring_made_failed_nil_and_bags() {
        let mut g = Pica::with_seed(3);
        g.bids = [Some(4), Some(0)];
        g.tricks = [6, 0];
        g.finish_hand();
        assert_eq!(g.scores, [42, 50]);
        assert_eq!(g.bags, [2, 0]);
        let mut g = Pica::with_seed(3);
        g.bids = [Some(5), Some(0)];
        g.tricks = [3, 10];
        g.bags = [9, 0];
        g.finish_hand();
        assert_eq!(g.scores, [-50, -50]);
        let mut g = Pica::with_seed(3);
        g.bids = [Some(2), Some(3)];
        g.tricks = [3, 3];
        g.bags = [9, 0];
        g.finish_hand();
        assert_eq!(g.scores, [21 - 100, 30]);
        assert_eq!(g.bags[0], 0);
    }

    #[test]
    fn bid_range() {
        let g = Pica::with_seed(4);
        assert_eq!(g.legal_moves().len(), 14);
    }
}
