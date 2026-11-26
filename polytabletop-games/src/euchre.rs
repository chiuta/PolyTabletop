//! Euchre (variantă pentru 2 jucători), din regulile publice.
//!
//! * Pachet de 24 de cărți (9, 10, J, Q, K, A), 5 cărți fiecăruia; o carte se întoarce
//!   („upcard”), restul stau nefolosite. Nu se joacă cu „dummy”: variantă simplă pentru doi.
//! * Runda 1: non-dealerul, apoi dealerul spun `pas` sau `up` (atu = culoarea întoarsă; dealerul
//!   ia cartea și dă una înapoi). Runda 2: culoarea întoarsă e interzisă; non-dealerul poate da
//!   `pas`, dealerul e obligat să numească o culoare („stick the dealer”).
//! * Atu: Valetul de atu (bower dreapta) e cel mai mare, apoi Valetul de aceeași culoare-soră
//!   (bower stânga, care ESTE atu), apoi A, K, Q, 10, 9. Se răspunde la culoare; atu bate tot.
//! * Cine a numit atu („maker”) trebuie să ia 3 din 5 levate: 3–4 levate = 1 punct, 5 = 2 puncte;
//!   altfel („euchre”) adversarul ia 2 puncte. Se joacă până la 10 puncte.
//!
//! Mutări: `imparte`, `pas`, `up`, `nume h` (h/d/s/c), `dis 9h` (dealerul dă o carte), `Jh`.
//! Informație ascunsă: AI-ul citește doar propria mână + cartea întoarsă + scorul public.

use crate::cards::{full_deck, join_cards, shuffle, Card, Viewer};
use crate::cardsb_util::{has_suit, sort_hand, suit_name, trick_winner};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const TARGET: u8 = 10;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Deal,
    Bid1,
    Bid2,
    Discard,
    Play,
    Over,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Deal,
    Pass,
    Up,
    Name(u8),
    Discard(Card),
    Play(Card),
}

#[derive(Clone)]
pub struct Euchre {
    rng: Rng,
    hands: [Vec<Card>; 2],
    upcard: Card,
    dealer: Player,
    phase: Phase,
    side: Player,
    trump: Option<u8>,
    maker: Option<Player>,
    tricks: [u8; 2],
    trick: Vec<(Player, Card)>,
    leader: Player,
    scores: [u8; 2],
    log: String,
    viewer: Viewer,
}

impl Default for Euchre {
    fn default() -> Self {
        Self::new()
    }
}

fn eff_suit(c: Card, trump: Option<u8>) -> u8 {
    match trump {
        Some(t) if c.rank == 11 && c.suit == (t ^ 1) => t,
        _ => c.suit,
    }
}

fn power(c: Card, trump: Option<u8>) -> u32 {
    if let Some(t) = trump {
        if c.rank == 11 && c.suit == t {
            return 100;
        }
        if c.rank == 11 && c.suit == (t ^ 1) {
            return 99;
        }
    }
    c.rank as u32
}

/// Estimare grosieră a levatelor pe care le poate lua o mână cu atu `t` (doar cărțile date).
fn est_tricks(hand: &[Card], t: u8) -> f32 {
    let mut s = 0.0;
    for &c in hand {
        let e = eff_suit(c, Some(t));
        if e == t {
            s += match power(c, Some(t)) {
                100 => 1.0,
                99 => 0.9,
                14 => 0.8,
                13 => 0.6,
                12 => 0.45,
                10 => 0.35,
                _ => 0.3,
            };
        } else if c.rank == 14 {
            s += 0.6;
        } else if c.rank == 13 {
            s += 0.15;
        }
    }
    s
}

impl Euchre {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Euchre {
            rng: Rng::new(seed),
            hands: [Vec::new(), Vec::new()],
            upcard: Card { rank: 9, suit: 0 },
            dealer: Player::Two,
            phase: Phase::Deal,
            side: Player::One,
            trump: None,
            maker: None,
            tricks: [0, 0],
            trick: Vec::new(),
            leader: Player::One,
            scores: [0, 0],
            log: String::new(),
            viewer: Viewer::Fixed(Player::One),
        };
        g.deal();
        g
    }

    pub fn score(&self) -> (u8, u8) {
        (self.scores[0], self.scores[1])
    }

    fn deal(&mut self) {
        let mut d = full_deck(9);
        shuffle(&mut d, &mut self.rng);
        let pone = self.dealer.other();
        self.hands = [Vec::new(), Vec::new()];
        for _ in 0..5 {
            self.hands[pone.idx()].push(d.pop().unwrap());
        }
        for _ in 0..5 {
            self.hands[self.dealer.idx()].push(d.pop().unwrap());
        }
        sort_hand(&mut self.hands[0]);
        sort_hand(&mut self.hands[1]);
        self.upcard = d.pop().unwrap();
        self.trump = None;
        self.maker = None;
        self.tricks = [0, 0];
        self.trick.clear();
        self.phase = Phase::Bid1;
        self.side = pone;
    }

    fn start_play(&mut self) {
        self.phase = Phase::Play;
        self.leader = self.dealer.other();
        self.side = self.leader;
        sort_hand(&mut self.hands[0]);
        sort_hand(&mut self.hands[1]);
    }

    fn eff(&self, c: Card) -> u8 {
        eff_suit(c, self.trump)
    }
}

impl Game for Euchre {
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
            Phase::Bid1 => vec![Move::Pass, Move::Up],
            Phase::Bid2 => {
                let mut v = Vec::new();
                if self.side != self.dealer {
                    v.push(Move::Pass);
                }
                for s in 0..4u8 {
                    if s != self.upcard.suit {
                        v.push(Move::Name(s));
                    }
                }
                v
            }
            Phase::Discard => self.hands[self.side.idx()]
                .iter()
                .map(|&c| Move::Discard(c))
                .collect(),
            Phase::Play => {
                let h = &self.hands[self.side.idx()];
                let eff = |c: Card| self.eff(c);
                let led = self.trick.first().map(|&(_, c)| eff(c));
                let must = led.filter(|&s| has_suit(h, s, &eff));
                let mut v: Vec<Move> = Vec::new();
                for &c in h {
                    if must.is_none_or(|s| eff(c) == s) {
                        v.push(Move::Play(c));
                    }
                }
                v
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
                    }
                    self.side = self.dealer.other();
                } else {
                    self.side = self.dealer;
                }
            }
            Move::Up => {
                self.trump = Some(self.upcard.suit);
                self.maker = Some(self.side);
                let d = self.dealer;
                self.hands[d.idx()].push(self.upcard);
                self.phase = Phase::Discard;
                self.side = d;
            }
            Move::Name(s) => {
                self.trump = Some(s);
                self.maker = Some(self.side);
                self.start_play();
            }
            Move::Discard(c) => {
                let h = &mut self.hands[self.side.idx()];
                let pos = h.iter().position(|&x| x == c).unwrap();
                h.remove(pos);
                self.start_play();
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
                let t = self.trump;
                let w = trick_winner(&cards, t, &|c| eff_suit(c, t), &|c| power(c, t));
                let winner = self.trick[w].0;
                self.tricks[winner.idx()] += 1;
                self.trick.clear();
                self.leader = winner;
                self.side = winner;
                if self.hands[0].is_empty() {
                    let maker = self.maker.unwrap();
                    let mt = self.tricks[maker.idx()];
                    let (who, pts) = if mt >= 3 {
                        (maker, if mt == 5 { 2 } else { 1 })
                    } else {
                        (maker.other(), 2)
                    };
                    self.scores[who.idx()] += pts;
                    self.log = format!(
                        "Mână încheiată: J{} ia {} punct(e){}",
                        who.idx() + 1,
                        pts,
                        if mt < 3 { " (euchre)" } else { "" }
                    );
                    if self.scores[who.idx()] >= TARGET {
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
        Some(if self.scores[0] >= TARGET {
            Outcome::Win(Player::One)
        } else {
            Outcome::Win(Player::Two)
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let q = p.other();
        let mut s = 1000 * (self.scores[p.idx()] as i32 - self.scores[q.idx()] as i32);
        let mine = &self.hands[p.idx()];
        match (self.phase, self.trump) {
            (Phase::Discard, Some(t)) | (Phase::Play, Some(t)) => {
                let mut est = est_tricks(mine, t);
                // atenție: cărțile deja jucate sunt scoase din mână, deci împreună cu levatele
                est += self.tricks[p.idx()] as f32;
                if self.phase == Phase::Discard && self.side != p {
                    return s;
                }
                if self.maker == Some(p) {
                    s += ((est - 2.8) * 40.0) as i32;
                } else {
                    s += ((est - 2.2) * 40.0) as i32;
                }
                if self.phase == Phase::Play {
                    s += 100 * (self.tricks[p.idx()] as i32 - self.tricks[q.idx()] as i32);
                    if let Some(&(w, _)) = self.trick.first() {
                        // un singur card pe masă: cine îl pune (după `apply`) e `w`
                        let c = self.trick[0].1;
                        let strength = power(c, Some(t)) as i32
                            + if eff_suit(c, Some(t)) == t { 40 } else { 0 };
                        s += if w == p { 2 * strength } else { -strength };
                    }
                }
            }
            _ => {}
        }
        s
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Euchre — scor: J1 {}, J2 {} (până la {}) | dealer: J{}\n",
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
        }
        if let Some(t) = self.trump {
            s.push_str(&format!(
                "Atu: {} (numit de J{}) | levate: J1 {}, J2 {}\n",
                suit_name(t),
                self.maker.map(|m| m.idx() + 1).unwrap_or(0),
                self.tricks[0],
                self.tricks[1]
            ));
        }
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
            Move::Pass => "pas".into(),
            Move::Up => "up".into(),
            Move::Name(s) => format!("nume {}", ['h', 'd', 's', 'c'][*s as usize]),
            Move::Discard(c) => format!("dis {}", c.text()),
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
    fn viewer_controls_whose_hand_is_shown() {
        let mut g = Euchre::with_seed(6);
        g.hands[0] = vec![c(9, 0), c(10, 1), c(12, 2)];
        g.hands[1] = vec![c(13, 3), c(14, 0), c(11, 1)];
        let own = |r: &str| {
            r.lines()
                .find(|l| l.starts_with("Mâna ta"))
                .unwrap()
                .to_string()
        };
        Game::set_viewer(&mut g, None);
        for side in [Player::One, Player::Two] {
            g.side = side;
            let r = Game::render(&g);
            assert!(own(&r).ends_with(&join_cards(&g.hands[side.idx()])), "{r}");
            assert!(
                !r.contains(&join_cards(&g.hands[side.other().idx()])),
                "{r}"
            );
        }
        g.side = Player::One;
        Game::set_viewer(&mut g, Some(Player::Two));
        let r = Game::render(&g);
        assert!(own(&r).ends_with(&join_cards(&g.hands[1])), "{r}");
        assert!(!r.contains(&join_cards(&g.hands[0])), "{r}");
    }

    #[test]
    fn bowers_rank_and_suit() {
        let t = Some(0u8); // cupe, sora = caro (1)
        assert_eq!(eff_suit(c(11, 1), t), 0);
        assert_eq!(eff_suit(c(11, 2), t), 2);
        assert!(power(c(11, 0), t) > power(c(11, 1), t));
        assert!(power(c(11, 1), t) > power(c(14, 0), t));
    }

    #[test]
    fn deal_sizes() {
        let g = Euchre::with_seed(5);
        assert_eq!(g.hands[0].len(), 5);
        assert_eq!(g.hands[1].len(), 5);
        assert_ne!(g.hands[0], g.hands[1]);
    }

    #[test]
    fn dealer_cannot_pass_in_round_two() {
        let mut g = Euchre::with_seed(1);
        g.apply(&Move::Pass);
        g.apply(&Move::Pass);
        assert_eq!(g.phase, Phase::Bid2);
        g.apply(&Move::Pass);
        assert_eq!(g.side, g.dealer);
        assert!(!g.legal_moves().contains(&Move::Pass));
        assert_eq!(g.legal_moves().len(), 3);
    }

    #[test]
    fn order_up_dealer_discards() {
        let mut g = Euchre::with_seed(2);
        g.apply(&Move::Up);
        assert_eq!(g.phase, Phase::Discard);
        assert_eq!(g.hands[g.dealer.idx()].len(), 6);
        let m = g.legal_moves()[0];
        g.apply(&m);
        assert_eq!(g.phase, Phase::Play);
        assert_eq!(g.hands[g.dealer.idx()].len(), 5);
    }

    #[test]
    fn must_follow_left_bower_as_trump() {
        let mut g = Euchre::with_seed(1);
        g.phase = Phase::Play;
        g.trump = Some(0);
        g.hands = [vec![c(11, 1), c(9, 2)], vec![c(12, 0), c(9, 3)]];
        g.trick = vec![(Player::Two, c(10, 0))];
        g.side = Player::One;
        let mv = g.legal_moves();
        assert_eq!(mv, vec![Move::Play(c(11, 1))]); // J caro = atu
    }

    #[test]
    fn scoring_maker_euchre() {
        let mut g = Euchre::with_seed(1);
        g.phase = Phase::Play;
        g.trump = Some(0);
        g.maker = Some(Player::One);
        g.tricks = [1, 3];
        g.hands = [vec![], vec![c(9, 3)]];
        g.trick = vec![(Player::One, c(10, 2))];
        g.side = Player::Two;
        g.apply(&Move::Play(c(9, 3)));
        // J1 conduce 10♠, J2 pune 9♣ (altă culoare, nu atu): J1 ia levata -> 2-3 pentru maker: euchre
        assert_eq!(g.scores, [0, 2]);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=10u64 {
            let mut g = Euchre::with_seed(seed);
            let mut rng = Rng::new(seed + 7);
            let mut n = 0;
            while g.outcome().is_none() && n < 5000 {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
            }
            assert!(g.outcome().is_some());
        }
    }
}
