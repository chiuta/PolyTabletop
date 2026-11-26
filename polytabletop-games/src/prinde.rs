//! Prinde — levate cu licitație exactă (tip „Oh Hell”), variantă pentru 2 jucători.
//!
//! Reguli din descrierile publice ale jocurilor de tip Oh Hell / Contract Whist (nume generic):
//! * 13 runde cu mâini de 1, 2, … 7, … 2, 1 cărți (pachet de 52). După împărțire se întoarce o
//!   carte din restul pachetului: culoarea ei e atu în runda respectivă (carte publică).
//! * Fiecare spune câte levate va lua (`licit N`); cine licitează ultimul (dealerul) nu are voie
//!   să aducă suma licitațiilor la numărul de levate al rundei („cineva trebuie să piardă”).
//! * Culoare obligatorie; atu bate tot. Primul conduce non-dealerul; cine ia levata conduce.
//! * Scor: exact cât ai licitat = 10 + licitația; altfel 0. Câștigă cine are mai multe puncte
//!   după a 13-a rundă (egal = remiză).
//!
//! Mutări: `imparte`, `licit N`, carte (`Kd`, `4s`). AI-ul citește doar propria mână, atuul
//! întors, licitațiile, levatele luate și numărul cărților adversarului.

use crate::cards::{join_cards, Card, Viewer};
use crate::trickkit::{beats, deal, follow, p_win};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const SIZES: [usize; 13] = [1, 2, 3, 4, 5, 6, 7, 6, 5, 4, 3, 2, 1];

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
pub struct Prinde {
    rng: Rng,
    hands: [Vec<Card>; 2],
    stock: Vec<Card>,
    trump_card: Card,
    round: usize,
    dealer: Player,
    phase: Phase,
    side: Player,
    bids: [Option<u8>; 2],
    trick: Vec<(Player, Card)>,
    tricks: [u8; 2],
    scores: [i32; 2],
    log: String,
    viewer: Viewer,
}

impl Default for Prinde {
    fn default() -> Self {
        Self::new()
    }
}

fn est_wins(h: &[Card], trump: Option<u8>) -> f32 {
    let n = h.len() as f32;
    let p: f32 = h.iter().map(|&c| p_win(c, trump)).sum();
    (0.5 * n + (p - 0.3 * n)).clamp(0.0, n)
}

impl Prinde {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Prinde {
            rng: Rng::new(seed),
            hands: [Vec::new(), Vec::new()],
            stock: Vec::new(),
            trump_card: Card { rank: 2, suit: 0 },
            round: 0,
            dealer: Player::Two,
            phase: Phase::Deal,
            side: Player::One,
            bids: [None, None],
            trick: Vec::new(),
            tricks: [0, 0],
            scores: [0, 0],
            log: String::new(),
            viewer: Viewer::Fixed(Player::One),
        };
        g.deal_round();
        g
    }

    pub fn scores(&self) -> (i32, i32) {
        (self.scores[0], self.scores[1])
    }

    fn trump(&self) -> Option<u8> {
        Some(self.trump_card.suit)
    }

    fn deal_round(&mut self) {
        let (h, mut s) = deal(&mut self.rng, SIZES[self.round]);
        self.hands = h;
        self.trump_card = s.pop().unwrap();
        s.push(self.trump_card);
        self.stock = s;
        self.bids = [None, None];
        self.tricks = [0, 0];
        self.trick.clear();
        self.phase = Phase::Bid;
        self.side = self.dealer.other();
    }

    /// Mâna unui jucător (pentru teste și interfețe care au voie să o vadă).
    pub fn hand_of(&self, p: Player) -> &[Card] {
        &self.hands[p.idx()]
    }

    /// Pentru teste: reamestecă ce nu vede `viewer` (atuul întors rămâne la locul lui).
    pub fn scramble_hidden(&mut self, viewer: Player, seed: u64) {
        let tc = self.trump_card;
        crate::trickkit::scramble_hidden(&mut self.hands, &mut self.stock, viewer, &[tc], seed);
    }

    fn finish_round(&mut self) {
        let mut parts = Vec::new();
        for i in 0..2 {
            let b = self.bids[i].unwrap();
            let d = if self.tricks[i] == b {
                10 + b as i32
            } else {
                0
            };
            self.scores[i] += d;
            parts.push(format!("J{} {}/{} → +{}", i + 1, self.tricks[i], b, d));
        }
        self.log = format!("Runda încheiată: {}", parts.join(", "));
        self.round += 1;
        if self.round >= SIZES.len() {
            self.phase = Phase::Over;
        } else {
            self.dealer = self.dealer.other();
            self.phase = Phase::Deal;
            self.side = self.dealer.other();
        }
    }
}

impl Game for Prinde {
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
                let n = SIZES[self.round] as u8;
                let ban = self.bids[self.side.other().idx()]
                    .filter(|&b| b <= n)
                    .map(|b| n - b);
                (0..=n).filter(|&b| Some(b) != ban).map(Move::Bid).collect()
            }
            Phase::Play => follow(&self.hands[self.side.idx()], &self.trick)
                .into_iter()
                .map(Move::Play)
                .collect(),
        }
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        match *m {
            Move::Deal => self.deal_round(),
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
                self.trick.push((p, c));
                if self.trick.len() < 2 {
                    self.side = p.other();
                    return;
                }
                let (lp, lc) = self.trick[0];
                let (rp, rc) = self.trick[1];
                let w = if beats(lc, rc, self.trump()) { rp } else { lp };
                self.tricks[w.idx()] += 1;
                self.trick.clear();
                self.side = w;
                if self.hands[0].is_empty() {
                    self.finish_round();
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
                    s -= 80.0 * (b as f32 - est_wins(&self.hands[pi], self.trump())).abs();
                }
            }
            Phase::Play if self.tricks == [0, 0] && self.hands[pi].len() == SIZES[self.round] => {
                let b = self.bids[pi].unwrap();
                s -= 80.0 * (b as f32 - est_wins(&self.hands[pi], self.trump())).abs();
            }
            Phase::Play => {
                let (bm, bo) = (self.bids[pi].unwrap(), self.bids[qi].unwrap());
                let mut er_me = est_wins(&self.hands[pi], self.trump());
                let mut er_op = 0.5 * self.hands[qi].len() as f32;
                if let Some(&(lp, c)) = self.trick.first() {
                    let pw = p_win(c, self.trump());
                    if lp == p {
                        er_me += pw;
                        er_op += 1.0 - pw;
                    }
                }
                let val = |b: u8, t: u8, er: f32| {
                    let proj = t as f32 + er;
                    10.0 * (10.0 + b as f32) * (1.0 - (proj - b as f32).abs()).max(0.0)
                };
                s += val(bm, self.tricks[pi], er_me) - val(bo, self.tricks[qi], er_op);
            }
            _ => {}
        }
        s as i32
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Prinde — scor: J1 {}, J2 {} | runda {}/{} ({} cărți) | dealer: J{}\n",
            self.scores[0],
            self.scores[1],
            (self.round + 1).min(SIZES.len()),
            SIZES.len(),
            SIZES[self.round.min(SIZES.len() - 1)],
            self.dealer.idx() + 1
        );
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        if self.phase == Phase::Over {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        let bt = |b: Option<u8>| b.map(|x| x.to_string()).unwrap_or_else(|| "—".into());
        s.push_str(&format!(
            "Atu: {} (carte întoarsă {}) | licitații: J1 {}, J2 {} | levate: J1 {}, J2 {}\n",
            crate::cardsb_util::suit_name(self.trump_card.suit),
            self.trump_card.text(),
            bt(self.bids[0]),
            bt(self.bids[1]),
            self.tricks[0],
            self.tricks[1]
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
    fn dealer_cannot_make_bids_sum_to_hand_size() {
        let mut g = Prinde::with_seed(5);
        g.round = 3; // 4 cărți
        g.deal_round();
        g.apply(&Move::Bid(1));
        let ms = g.legal_moves();
        assert!(!ms.contains(&Move::Bid(3)));
        assert!(
            ms.contains(&Move::Bid(2)) && ms.contains(&Move::Bid(0)) && ms.contains(&Move::Bid(4))
        );
    }

    #[test]
    fn exact_bid_scores_ten_plus() {
        let mut g = Prinde::with_seed(5);
        g.bids = [Some(2), Some(1)];
        g.tricks = [2, 2];
        g.finish_round();
        assert_eq!(g.scores, [12, 0]);
    }

    #[test]
    fn trump_card_is_public_and_wins() {
        let mut g = Prinde::with_seed(6);
        g.trump_card = c(2, 0);
        g.phase = Phase::Play;
        g.round = 5;
        g.hands = [vec![c(10, 3), c(2, 3)], vec![c(3, 0), c(4, 1)]];
        g.side = Player::One;
        g.apply(&Move::Play(c(10, 3)));
        g.apply(&Move::Play(c(3, 0)));
        assert_eq!(g.tricks, [0, 1]);
    }
}
