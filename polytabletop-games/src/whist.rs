//! Whist (clasic, adaptat pentru 2 jucători), din descrierile publice ale jocului.
//!
//! * Pachet de 52, câte 13 cărți fiecăruia; 26 de cărți rămân nefolosite și nevăzute.
//!   Ultima carte împărțită dealerului se arată: culoarea ei e atu (cartea rămâne în mâna lui).
//! * Primul conduce non-dealerul; culoare obligatorie, atu bate tot, câștigătorul unei levate
//!   conduce următoarea.
//! * Scor: cine ia mai mult de 6 levate din 13 primește câte un punct pentru fiecare levată peste
//!   a șasea. Jocul se încheie la 5 puncte (fără „onoruri”, simplificare declarată).
//!
//! Mutări: `imparte`, carte (`Ah`, `9c`). AI-ul citește doar propria mână, atuul arătat,
//! levatele luate (publice) și numărul cărților adversarului.

use crate::cards::{join_cards, Card, Viewer};
use crate::trickkit::{beats, deal, follow, hand_power, p_win};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const TARGET: i32 = 5;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Deal,
    Play,
    Over,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Deal,
    Play(Card),
}

#[derive(Clone)]
pub struct Whist {
    rng: Rng,
    hands: [Vec<Card>; 2],
    stock: Vec<Card>,
    trump_card: Card,
    dealer: Player,
    phase: Phase,
    side: Player,
    trick: Vec<(Player, Card)>,
    tricks: [u8; 2],
    scores: [i32; 2],
    log: String,
    viewer: Viewer,
}

impl Default for Whist {
    fn default() -> Self {
        Self::new()
    }
}

impl Whist {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Whist {
            rng: Rng::new(seed),
            hands: [Vec::new(), Vec::new()],
            stock: Vec::new(),
            trump_card: Card { rank: 2, suit: 0 },
            dealer: Player::Two,
            phase: Phase::Deal,
            side: Player::One,
            trick: Vec::new(),
            tricks: [0, 0],
            scores: [0, 0],
            log: String::new(),
            viewer: Viewer::Fixed(Player::One),
        };
        g.deal_hand();
        g
    }

    pub fn scores(&self) -> (i32, i32) {
        (self.scores[0], self.scores[1])
    }

    fn trump(&self) -> Option<u8> {
        Some(self.trump_card.suit)
    }

    fn deal_hand(&mut self) {
        let (h, s) = deal(&mut self.rng, 13);
        self.hands = h;
        self.stock = s;
        let k = self.rng.below(13);
        self.trump_card = self.hands[self.dealer.idx()][k];
        self.tricks = [0, 0];
        self.trick.clear();
        self.phase = Phase::Play;
        self.side = self.dealer.other();
    }

    /// Mâna unui jucător (pentru teste și interfețe care au voie să o vadă).
    pub fn hand_of(&self, p: Player) -> &[Card] {
        &self.hands[p.idx()]
    }

    /// Pentru teste: reamestecă ce nu vede `viewer` (atuul arătat rămâne la locul lui).
    pub fn scramble_hidden(&mut self, viewer: Player, seed: u64) {
        let tc = self.trump_card;
        crate::trickkit::scramble_hidden(&mut self.hands, &mut self.stock, viewer, &[tc], seed);
    }
}

impl Game for Whist {
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
            Phase::Play => follow(&self.hands[self.side.idx()], &self.trick)
                .into_iter()
                .map(Move::Play)
                .collect(),
        }
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        match *m {
            Move::Deal => self.deal_hand(),
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
                    let t = self.tricks[w.idx()].max(self.tricks[w.other().idx()]);
                    let win = if self.tricks[0] > self.tricks[1] {
                        Player::One
                    } else {
                        Player::Two
                    };
                    let pts = t as i32 - 6;
                    self.scores[win.idx()] += pts;
                    self.log = format!(
                        "Mână încheiată: {}–{} levate, J{} ia {} punct(e)",
                        self.tricks[0],
                        self.tricks[1],
                        win.idx() + 1,
                        pts
                    );
                    if self.scores.iter().any(|&s| s >= TARGET) {
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
        Some(match self.scores[0].cmp(&self.scores[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let (pi, qi) = (p.idx(), p.other().idx());
        let mut s = 100.0 * (self.scores[pi] - self.scores[qi]) as f32;
        if self.phase == Phase::Play {
            s += 100.0 * (self.tricks[pi] as f32 - self.tricks[qi] as f32);
            s += 30.0 * hand_power(&self.hands[pi], self.trump());
            if let Some(&(lp, c)) = self.trick.first() {
                if lp == p {
                    s += 60.0 * p_win(c, self.trump());
                }
            }
        }
        s as i32
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Whist — scor: J1 {}, J2 {} (până la {}) | dealer: J{}\n",
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
        s.push_str(&format!(
            "Atu: {} (carte arătată {}) | levate: J1 {}, J2 {}\n",
            crate::cardsb_util::suit_name(self.trump_card.suit),
            self.trump_card.text(),
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
            Move::Play(c) => c.text(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        // Adâncimea 0 = o singură mutare la TOATE nivelurile: la Hard/Expert adâncimea 1 ar extinde
        // răspunsurile adversarului din mâna lui reală (informație ascunsă).
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
    fn trump_shown_belongs_to_dealer_and_scoring() {
        let g = Whist::with_seed(7);
        assert!(g.hand_of(g.dealer).contains(&g.trump_card));
        let mut g = Whist::with_seed(7);
        g.hands = [vec![c(9, 1)], vec![c(3, 0)]];
        g.trump_card = c(2, 0);
        g.tricks = [6, 6];
        g.side = Player::One;
        g.apply(&Move::Play(c(9, 1)));
        g.apply(&Move::Play(c(3, 0))); // atu bate
        assert_eq!(g.scores, [0, 1]);
    }

    #[test]
    fn must_follow() {
        let mut g = Whist::with_seed(8);
        g.hands = [vec![c(9, 1)], vec![c(3, 0), c(8, 1), c(5, 2)]];
        g.trump_card = c(2, 0);
        g.trick = vec![(Player::One, c(9, 1))];
        g.side = Player::Two;
        assert_eq!(g.legal_moves(), vec![Move::Play(c(8, 1))]);
    }
}

#[cfg(test)]
mod a3_audit {
    use super::*;

    #[test]
    fn ai_ignores_hidden_cards_at_every_level() {
        crate::p3_comun::ai_blind_check(
            "whist",
            Whist::with_seed,
            |g, me, s| g.scramble_hidden(me, s),
            10,
        );
    }
}
