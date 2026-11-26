//! Ombre (L'Hombre; Spania / Europa, sec. XVII), adaptat pentru 2 jucători: ombre contra apărătorului.
//!
//! * 40 de cărți (pachetul francez fără 8, 9, 10), câte 9 fiecăruia, 22 în talon. Culori negre ♠ ♣ (s, c) și roșii ♥ ♦ (h, d).
//! * **Licitație**: J-ul din stânga dealerului zice `entrada` (devine ombre) sau `pas`; dacă pasează, dealerul
//!   poate zice `entrada`; dacă pasează amândoi se împarte din nou. Ombre alege atuul (`atu s`), apoi schimbă
//!   cărți (`scap 7s` pentru fiecare carte aruncată, apoi `gata` trage tot atâtea din talon); apoi apărătorul poate
//!   schimba la fel din ce a rămas.
//! * **Atuurile** (de la mare la mic): Spadille (A♠), Manille (2 din atu negru / 7 din atu roșu), Basto (A♣), Punto
//!   (A din atu roșu), apoi K Q J și restul culorii-atu: la atu negru 7 6 5 4 3, la atu roșu 2 3 4 5 6. Asul ♠ și asul ♣
//!   sunt mereu atuuri. Culori fără atu: negre K Q J 7 6 5 4 3 2; roșii K Q J A 2 3 4 5 6 7.
//! * Se respectă culoarea condusă (atuul e o „culoare”); nu există scutirea matadorilor (simplificare).
//!   Mâna: 9 levate, non-dealerul conduce. Ombre câștigă dacă ia cel puțin 5 levate: +2 puncte (toate 9 = +5);
//!   altfel apărătorul primește 3 (codille). Primul la 8 puncte câștigă.
//!
//! Mutări: `entrada`, `pas`, `atu h`, `scap Kd`, `gata`, `7s`, `imparte`.

use crate::cards::{full_deck, join_cards, shuffle, Card, Viewer};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const TARGET: i32 = 8;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Deal,
    Entrada,
    Pass,
    Trump(u8),
    Discard(Card),
    Done,
    Play(Card),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Deal,
    Bid1,
    Bid2,
    Trump,
    ExOmbre,
    ExDef,
    Play,
    Over,
}

#[derive(Clone)]
pub struct Ombre {
    rng: Rng,
    hands: [Vec<Card>; 2],
    stock: Vec<Card>,
    dealer: Player,
    ombre: Option<Player>,
    trump: u8,
    phase: Phase,
    side: Player,
    trick: Vec<(Player, Card)>,
    tricks: [u8; 2],
    discarded: u8,
    score: [i32; 2],
    viewer: Viewer,
    log: String,
}

impl Default for Ombre {
    fn default() -> Self {
        Self::new()
    }
}

fn is_black(suit: u8) -> bool {
    suit >= 2
}

impl Ombre {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Ombre {
            rng: Rng::new(seed),
            hands: [Vec::new(), Vec::new()],
            stock: Vec::new(),
            dealer: Player::Two,
            ombre: None,
            trump: 0,
            phase: Phase::Deal,
            side: Player::One,
            trick: Vec::new(),
            tricks: [0; 2],
            discarded: 0,
            score: [0; 2],
            viewer: Viewer::Fixed(Player::One),
            log: String::new(),
        };
        g.deal();
        g
    }

    pub fn score(&self) -> (i32, i32) {
        (self.score[0], self.score[1])
    }

    pub fn hand_of(&self, p: Player) -> &[Card] {
        &self.hands[p.idx()]
    }

    fn deck() -> Vec<Card> {
        full_deck(2)
            .into_iter()
            .filter(|c| !(8..=10).contains(&c.rank))
            .collect()
    }

    fn deal(&mut self) {
        let mut d = Self::deck();
        shuffle(&mut d, &mut self.rng);
        self.hands = [d.drain(..9).collect(), d.drain(..9).collect()];
        self.stock = d;
        self.sort_hands();
        self.trick.clear();
        self.tricks = [0; 2];
        self.ombre = None;
        self.discarded = 0;
        self.phase = Phase::Bid1;
        self.side = self.dealer.other();
    }

    fn sort_hands(&mut self) {
        for h in &mut self.hands {
            h.sort_by_key(|c| (c.suit, std::cmp::Reverse(c.rank)));
        }
    }

    /// Valoare de forță a cărții ca atu (None dacă nu e atu) pentru atuul `t`.
    fn trump_power(c: Card, t: u8) -> Option<i32> {
        if c == (Card { rank: 14, suit: 2 }) {
            return Some(100);
        }
        let manille = Card {
            rank: if is_black(t) { 2 } else { 7 },
            suit: t,
        };
        if c == manille {
            return Some(99);
        }
        if c == (Card { rank: 14, suit: 3 }) {
            return Some(98);
        }
        if c.suit != t {
            return None;
        }
        if c.rank == 14 {
            return Some(97); // punto (atu roșu)
        }
        Some(match (is_black(t), c.rank) {
            (_, 13) => 96,
            (_, 12) => 95,
            (_, 11) => 94,
            (true, r) => 82 + r as i32,
            (false, r) => 95 - r as i32,
        })
    }

    fn plain_power(c: Card) -> i32 {
        if is_black(c.suit) {
            match c.rank {
                13 => 9,
                12 => 8,
                11 => 7,
                r => r as i32 - 1,
            }
        } else {
            match c.rank {
                13 => 10,
                12 => 9,
                11 => 8,
                14 => 7,
                r => 8 - r as i32,
            }
        }
    }

    fn group(&self, c: Card) -> u8 {
        if Self::trump_power(c, self.trump).is_some() {
            4
        } else {
            c.suit
        }
    }

    fn power(&self, c: Card) -> i32 {
        Self::trump_power(c, self.trump).unwrap_or_else(|| Self::plain_power(c))
    }

    fn second_beats(&self, a: Card, b: Card) -> bool {
        let (ga, gb) = (self.group(a), self.group(b));
        if ga == gb {
            self.power(b) > self.power(a)
        } else {
            gb == 4
        }
    }

    fn playable(&self, p: Player) -> Vec<Card> {
        let h = &self.hands[p.idx()];
        if let Some(&(_, f)) = self.trick.first() {
            let g = self.group(f);
            let same: Vec<Card> = h.iter().copied().filter(|&c| self.group(c) == g).collect();
            if !same.is_empty() {
                return same;
            }
        }
        h.clone()
    }

    /// Valoarea unei cărți pentru AI (atu = mare).
    fn card_value(&self, c: Card) -> i32 {
        match Self::trump_power(c, self.trump) {
            Some(p) => 6 + (p - 82) / 2,
            None => Self::plain_power(c) / 3,
        }
    }

    fn best_trump(hand: &[Card]) -> u8 {
        (0..4u8)
            .max_by_key(|&t| {
                hand.iter()
                    .filter_map(|&c| Self::trump_power(c, t))
                    .map(|p| p - 70)
                    .sum::<i32>()
            })
            .unwrap_or(0)
    }

    fn trump_strength(hand: &[Card], t: u8) -> i32 {
        hand.iter()
            .filter_map(|&c| Self::trump_power(c, t))
            .map(|p| p - 70)
            .sum()
    }

    fn finish_hand(&mut self) {
        let o = self.ombre.unwrap();
        let ot = self.tricks[o.idx()];
        if ot >= 5 {
            let pts = if ot == 9 { 5 } else { 2 };
            self.score[o.idx()] += pts;
            self.log = format!("Ombre (J{}) ia {ot} levate: +{pts}.", o.idx() + 1);
        } else {
            self.score[o.other().idx()] += 3;
            self.log = format!(
                "Ombre (J{}) ia doar {ot} levate: codille, J{} +3.",
                o.idx() + 1,
                o.other().idx() + 1
            );
        }
        if self.score.iter().any(|&s| s >= TARGET) {
            self.phase = Phase::Over;
        } else {
            self.dealer = self.dealer.other();
            self.phase = Phase::Deal;
            self.side = self.dealer.other();
        }
    }
}

impl Game for Ombre {
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
            Phase::Bid1 | Phase::Bid2 => vec![Move::Entrada, Move::Pass],
            Phase::Trump => (0..4).map(Move::Trump).collect(),
            Phase::ExOmbre | Phase::ExDef => {
                let mut v: Vec<Move> = Vec::new();
                let room = self.stock.len() as u8 > self.discarded;
                if room && self.discarded < 8 {
                    for &c in &self.hands[self.side.idx()] {
                        v.push(Move::Discard(c));
                    }
                }
                v.push(Move::Done);
                v
            }
            Phase::Play => self
                .playable(self.side)
                .into_iter()
                .map(Move::Play)
                .collect(),
        }
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        let p = self.side;
        match *m {
            Move::Deal => {
                self.deal();
            }
            Move::Entrada => {
                self.ombre = Some(p);
                self.phase = Phase::Trump;
            }
            Move::Pass => {
                if self.phase == Phase::Bid1 {
                    self.phase = Phase::Bid2;
                    self.side = p.other();
                } else {
                    self.log = "Amândoi au pasat: se împarte din nou.".into();
                    self.dealer = self.dealer.other();
                    self.deal();
                }
            }
            Move::Trump(t) => {
                self.trump = t;
                self.phase = Phase::ExOmbre;
                self.discarded = 0;
                self.log = format!("Atu: {}.", crate::cardsb_util::suit_name(t));
            }
            Move::Discard(c) => {
                let h = &mut self.hands[p.idx()];
                let pos = h.iter().position(|&x| x == c).unwrap();
                h.remove(pos);
                self.discarded += 1;
            }
            Move::Done => {
                for _ in 0..self.discarded {
                    if let Some(c) = self.stock.pop() {
                        self.hands[p.idx()].push(c);
                    }
                }
                self.sort_hands();
                self.discarded = 0;
                if self.phase == Phase::ExOmbre {
                    self.phase = Phase::ExDef;
                    self.side = p.other();
                } else {
                    self.phase = Phase::Play;
                    self.side = self.dealer.other();
                }
            }
            Move::Play(c) => {
                let h = &mut self.hands[p.idx()];
                let pos = h.iter().position(|&x| x == c).unwrap();
                h.remove(pos);
                self.trick.push((p, c));
                if self.trick.len() < 2 {
                    self.side = p.other();
                    return;
                }
                let (lp, lc) = self.trick[0];
                let (rp, rc) = self.trick[1];
                let w = if self.second_beats(lc, rc) { rp } else { lp };
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
        Some(if self.score[0] > self.score[1] {
            Outcome::Win(Player::One)
        } else {
            Outcome::Win(Player::Two)
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let (me, q) = (p.idx(), 1 - p.idx());
        let mut v = (self.score[me] - self.score[q]) * 1000;
        let hand = &self.hands[me];
        match self.phase {
            Phase::Bid1 | Phase::Bid2 | Phase::Deal | Phase::Over => {
                // după `entrada` ombre își asumă 5 levate: se compară forța cu un prag
                if self.ombre == Some(p) {
                    v += Self::trump_strength(hand, Self::best_trump(hand)) * 4 - 60;
                } else if self.ombre == Some(p.other()) {
                    v -= 0;
                }
            }
            Phase::Trump => {
                v += Self::trump_strength(hand, Self::best_trump(hand));
            }
            Phase::ExOmbre | Phase::ExDef => {
                if self.side == p {
                    let s: i32 = hand.iter().map(|&c| self.card_value(c)).sum();
                    v += s * 10 + 40 * self.discarded as i32;
                }
            }
            Phase::Play => {
                let o = self.ombre.unwrap_or(p);
                let sign = if o == p { 1 } else { -1 };
                v += (self.tricks[me] as i32 - self.tricks[q] as i32) * 100;
                v += hand.iter().map(|&c| self.power(c)).sum::<i32>() / 4 * sign.max(1);
                if let Some(&(lp, c)) = self.trick.first() {
                    let k = self.power(c);
                    v += if lp == p { k } else { -k };
                }
            }
        }
        v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Ombre — puncte: J1 {}, J2 {} (la {TARGET}) | dealer: J{}\n",
            self.score[0],
            self.score[1],
            self.dealer.idx() + 1
        );
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        if self.phase == Phase::Over {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        if let Some(o) = self.ombre {
            s.push_str(&format!(
                "Ombre: J{} | atu: {} | levate: J1 {}, J2 {}\n",
                o.idx() + 1,
                if self.phase == Phase::Trump {
                    "de ales".to_string()
                } else {
                    crate::cardsb_util::suit_name(self.trump).to_string()
                },
                self.tricks[0],
                self.tricks[1]
            ));
        }
        match self.phase {
            Phase::Bid1 | Phase::Bid2 => s.push_str("Licitație: entrada sau pas\n"),
            Phase::ExOmbre | Phase::ExDef => s.push_str(&format!(
                "Schimb de cărți (talon: {} cărți; aruncate acum: {}): scap <carte> … apoi gata\n",
                self.stock.len(),
                self.discarded
            )),
            _ => {}
        }
        if !self.trick.is_empty() {
            let t: Vec<Card> = self.trick.iter().map(|&(_, c)| c).collect();
            s.push_str(&format!("Pe masă: {}\n", join_cards(&t)));
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
        s.push_str(&format!("La mutare: J{}\n", self.side.idx() + 1));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Deal => "imparte".into(),
            Move::Entrada => "entrada".into(),
            Move::Pass => "pas".into(),
            Move::Trump(t) => format!("atu {}", crate::cards::SUIT_LETTERS[*t as usize]),
            Move::Discard(c) => format!("scap {}", c.text()),
            Move::Done => "gata".into(),
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
    fn trump_order_black_and_red() {
        // atu pică (negru): A♠ > 2♠ > A♣ > K♠ > Q♠ > J♠ > 7♠ > … > 3♠
        let t = 2;
        let order = [
            c(14, 2),
            c(2, 2),
            c(14, 3),
            c(13, 2),
            c(12, 2),
            c(11, 2),
            c(7, 2),
            c(6, 2),
            c(3, 2),
        ];
        for w in order.windows(2) {
            assert!(
                Ombre::trump_power(w[0], t) > Ombre::trump_power(w[1], t),
                "{:?}",
                w
            );
        }
        // atu cupă (roșu): A♠ > 7♥ > A♣ > A♥ > K > Q > J > 2 > 3 … 6
        let t = 0;
        let order = [
            c(14, 2),
            c(7, 0),
            c(14, 3),
            c(14, 0),
            c(13, 0),
            c(12, 0),
            c(11, 0),
            c(2, 0),
            c(3, 0),
            c(6, 0),
        ];
        for w in order.windows(2) {
            assert!(
                Ombre::trump_power(w[0], t) > Ombre::trump_power(w[1], t),
                "{:?}",
                w
            );
        }
        assert_eq!(Ombre::trump_power(c(5, 1), 0), None);
        // culori simple: roșu 7 cel mai mic, negru K cel mai mare
        assert!(Ombre::plain_power(c(13, 1)) > Ombre::plain_power(c(14, 1)));
        assert!(Ombre::plain_power(c(14, 1)) > Ombre::plain_power(c(2, 1)));
        assert!(Ombre::plain_power(c(2, 1)) > Ombre::plain_power(c(7, 1)));
        assert!(Ombre::plain_power(c(7, 2)) > Ombre::plain_power(c(2, 2)));
        assert_eq!(Ombre::deck().len(), 40);
    }

    #[test]
    fn auction_exchange_and_play_flow() {
        let mut g = Ombre::with_seed(11);
        assert_eq!(g.hands[0].len(), 9);
        assert_eq!(g.stock.len(), 22);
        g.apply(&Move::Entrada);
        g.apply(&Move::Trump(2));
        let h = g.hands[g.side.idx()].clone();
        g.apply(&Move::Discard(h[0]));
        g.apply(&Move::Discard(h[1]));
        g.apply(&Move::Done);
        assert_eq!(g.hands[g.ombre.unwrap().idx()].len(), 9);
        assert_eq!(g.stock.len(), 20);
        g.apply(&Move::Done);
        assert_eq!(g.phase, Phase::Play);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=8u64 {
            let mut g = Ombre::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())].clone());
                n += 1;
                assert!(n < 20000, "{}", g.render());
            }
        }
    }

    #[test]
    fn ai_ignores_hidden_information() {
        let a = Ombre::with_seed(2);
        let mut b = a.clone();
        b.hands[1].reverse();
        b.stock.reverse();
        assert_eq!(
            polytabletop_core::GameObj::ai_move(&a, 1),
            polytabletop_core::GameObj::ai_move(&b, 1)
        );
    }
}
