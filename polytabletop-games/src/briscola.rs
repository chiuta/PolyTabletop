//! Briscola (Italia), 2 jucători, pachet italian de 40 de cărți, din regulile publice.
//!
//! * Patru culori: d = denari, c = coppe, s = spade, b = bastoni. Cărțile, de la
//!   cea mai mică la cea mai mare: 2 4 5 6 7 F(ante) C(avallo) R(e) 3 A(sso).
//!   Puncte: A 11, 3 10, R 4, C 3, F 2, restul 0 (120 în total).
//! * Fiecare primește 3 cărți; următoarea carte se întoarce și arată **briscola**
//!   (atuul) și rămâne jos, ca ultima carte a pachetului.
//! * Se joacă pe ture de câte două cărți. Nu e obligatoriu să răspunzi la culoare.
//!   Ia levata cartea mai mare de aceeași culoare sau, în lipsă, un atu. Cine ia
//!   trage primul o carte, apoi celălalt, și conduce următoarea levată.
//! * La sfârșit câștigă cine are peste 60 de puncte; 60–60 = remiză.
//!
//! Informație ascunsă: vezi `cards52.rs`. `render` arată doar mâna Jucătorului 1;
//! cărțile trase după ultima levată nu intră în `evaluate` (ar fi informație din viitor).
//! AI-ul (`AlphaBeta(1)`) citește doar mâna proprie, atuul și punctele câștigate.

use crate::cards::Viewer;
use crate::cards52::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const RANK_LABEL: [&str; 10] = ["2", "4", "5", "6", "7", "F", "C", "R", "3", "A"];
const RANK_PTS: [u16; 10] = [0, 0, 0, 0, 0, 2, 3, 4, 10, 11];
const SUIT_LETTERS: [char; 4] = ['d', 'c', 's', 'b'];
const SUIT_NAMES: [&str; 4] = ["denari", "coppe", "spade", "bastoni"];

/// Cartea = culoare * 10 + rang (0..=9, în ordinea puterii).
pub fn suit(c: u8) -> u8 {
    c / 10
}
pub fn rank(c: u8) -> u8 {
    c % 10
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

/// Cartea `resp` (jucată a doua) bate `led`?
pub fn second_wins(led: u8, resp: u8, trump: u8) -> bool {
    if suit(resp) == suit(led) {
        rank(resp) > rank(led)
    } else {
        suit(resp) == trump
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Move(pub u8);

#[derive(Clone)]
pub struct Briscola {
    hands: [Vec<u8>; 2],
    /// Indicele 0 este atuul întors (ultima carte); se trage cu `pop`.
    stock: Vec<u8>,
    trump: u8,
    led: Option<u8>,
    side: Player,
    pts: [u16; 2],
    drawn: [Option<u8>; 2],
    viewer: Viewer,
}

impl Default for Briscola {
    fn default() -> Self {
        Self::new()
    }
}

impl Briscola {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck: Vec<u8> = (0..40).collect();
        shuffle(&mut deck, &mut rng);
        let hands = [deck[0..3].to_vec(), deck[3..6].to_vec()];
        let stock = deck[6..40].to_vec();
        let trump = suit(stock[0]);
        Briscola {
            hands,
            stock,
            trump,
            led: None,
            side: Player::One,
            pts: [0; 2],
            drawn: [None; 2],
            viewer: Viewer::Fixed(Player::One),
        }
    }

    pub fn points(&self) -> (u16, u16) {
        (self.pts[0], self.pts[1])
    }
    pub fn trump_suit(&self) -> u8 {
        self.trump
    }
    pub fn stock_len(&self) -> usize {
        self.stock.len()
    }
    pub fn hand_of(&self, p: Player) -> &[u8] {
        &self.hands[p.idx()]
    }

    fn hold_value(&self, c: u8) -> i32 {
        if suit(c) == self.trump {
            30 + 6 * rank(c) as i32
        } else {
            card_points(c) as i32 * 8
        }
    }
}

impl Game for Briscola {
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
        if self.outcome().is_some() {
            return Vec::new();
        }
        self.hands[self.side.idx()]
            .iter()
            .map(|&c| Move(c))
            .collect()
    }

    fn apply(&mut self, m: &Move) {
        let me = self.side;
        let h = &mut self.hands[me.idx()];
        let pos = h.iter().position(|&x| x == m.0).expect("card in hand");
        h.remove(pos);
        self.drawn = [None; 2];
        match self.led {
            None => {
                self.led = Some(m.0);
                self.side = me.other();
            }
            Some(l) => {
                let leader = me.other();
                let winner = if second_wins(l, m.0, self.trump) {
                    me
                } else {
                    leader
                };
                self.led = None;
                self.pts[winner.idx()] += card_points(l) + card_points(m.0);
                for p in [winner, winner.other()] {
                    if let Some(x) = self.stock.pop() {
                        self.hands[p.idx()].push(x);
                        self.drawn[p.idx()] = Some(x);
                    }
                }
                self.side = winner;
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.hands[0].is_empty() || !self.hands[1].is_empty() || self.led.is_some() {
            return None;
        }
        Some(match self.pts[0].cmp(&self.pts[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    /// Citește doar: mâna lui `p` (fără cartea abia trasă), atuul și punctele câștigate.
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut s = (self.pts[me] as i32 - self.pts[1 - me] as i32) * 10;
        for &c in &self.hands[me] {
            if Some(c) != self.drawn[me] {
                s += self.hold_value(c);
            }
        }
        s
    }

    fn render(&self) -> String {
        let mut s = String::new();
        s.push_str(
            "Briscola — culori: d denari, c coppe, s spade, b bastoni; rang: 2 4 5 6 7 F C R 3 A\n",
        );
        let trump_card = self.stock.first().copied();
        s.push_str(&format!(
            "Briscola (atu): {}{}. Pachet: {} cărți. Puncte: J1 {} – J2 {}\n",
            SUIT_NAMES[self.trump as usize],
            match trump_card {
                Some(t) => format!(" (jos: {})", card_text(t)),
                None => " (cartea de jos a fost trasă)".into(),
            },
            self.stock.len(),
            self.pts[0],
            self.pts[1]
        ));
        let v = self.viewer.who(self.side);
        let o = v.other();
        s.push_str(&format!(
            "Jucător {} (ascuns) are {} cărți.\n",
            o.idx() + 1,
            self.hands[o.idx()].len()
        ));
        if let Some(l) = self.led {
            s.push_str(&format!(
                "Pe masă: Jucător {} a jucat {}\n",
                self.side.other().idx() + 1,
                card_text(l)
            ));
        }
        let mut h = self.hands[v.idx()].clone();
        h.sort_unstable();
        s.push_str(&format!(
            "Mâna ta (Jucător {}): {}\n",
            v.idx() + 1,
            h.iter()
                .map(|&c| card_text(c))
                .collect::<Vec<_>>()
                .join(" ")
        ));
        match self.outcome() {
            Some(Outcome::Win(p)) => s.push_str(&format!(
                "Final: Jucător {} câștigă ({}–{}).\n",
                p.idx() + 1,
                self.pts[0],
                self.pts[1]
            )),
            Some(Outcome::Draw) => s.push_str("Final: remiză 60–60.\n"),
            None => s.push_str(&format!("La mutare: Jucător {}\n", self.side.idx() + 1)),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        card_text(m.0)
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

    fn c(s: u8, r: u8) -> u8 {
        s * 10 + r
    }

    #[test]
    fn deck_is_worth_120() {
        let t: u16 = (0..40).map(card_points).sum();
        assert_eq!(t, 120);
    }

    #[test]
    fn deal_is_consistent() {
        let g = Briscola::with_seed(4);
        assert_eq!(g.hands[0].len(), 3);
        assert_eq!(g.hands[1].len(), 3);
        assert_eq!(g.stock.len(), 34);
        let mut all: Vec<u8> = g.hands[0]
            .iter()
            .chain(&g.hands[1])
            .chain(&g.stock)
            .copied()
            .collect();
        all.sort_unstable();
        assert_eq!(all, (0..40).collect::<Vec<u8>>());
        assert_eq!(g.trump, suit(g.stock[0]));
    }

    #[test]
    fn trick_rules() {
        let trump = 3;
        assert!(second_wins(c(0, 1), c(0, 5), trump));
        assert!(!second_wins(c(0, 5), c(0, 1), trump));
        assert!(!second_wins(c(0, 1), c(1, 9), trump)); // alta culoare pierde
        assert!(second_wins(c(0, 9), c(3, 0), trump)); // atu bate asul
        assert!(!second_wins(c(3, 0), c(0, 9), trump));
        assert!(second_wins(c(3, 0), c(3, 1), trump));
        // ordinea: 3 bate Re, Asso bate 3
        assert!(second_wins(c(0, 7), c(0, 8), trump));
        assert!(second_wins(c(0, 8), c(0, 9), trump));
    }

    #[test]
    fn winner_draws_first_and_leads() {
        let mut g = Briscola::with_seed(1);
        g.trump = 3;
        g.hands = [
            vec![c(0, 9), c(1, 0), c(1, 1)],
            vec![c(0, 8), c(1, 2), c(2, 2)],
        ];
        g.stock = vec![c(3, 0), c(2, 0), c(2, 1), c(2, 4), c(2, 5)];
        g.apply(&Move(c(0, 9)));
        assert_eq!(g.to_move(), Player::Two);
        g.apply(&Move(c(0, 8)));
        assert_eq!(g.pts, [21, 0]); // asul ia treiul
        assert_eq!(g.to_move(), Player::One);
        assert_eq!(g.hands[0].len(), 3);
        assert_eq!(g.hands[0][2], c(2, 5)); // câștigătorul a tras primul
        assert_eq!(g.hands[1][2], c(2, 4));
        assert_eq!(g.stock.len(), 3);
    }

    #[test]
    fn last_stock_card_is_the_trump_card() {
        let mut g = Briscola::with_seed(2);
        let t = g.stock[0];
        let mut rng = Rng::new(3);
        while g.stock.len() > 2 {
            let m = g.legal_moves();
            g.apply(&m[rng.below(m.len())]);
        }
        assert_eq!(g.stock[0], t);
        assert_eq!(g.stock.len(), 2);
    }

    #[test]
    fn cards_are_conserved_and_game_ends() {
        for seed in 1..=40u64 {
            let mut g = Briscola::with_seed(seed);
            let mut rng = Rng::new(seed ^ 0x77);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n <= 40);
            }
            assert_eq!(g.pts[0] + g.pts[1], 120);
            assert_eq!(n, 40);
            assert!(g.stock.is_empty());
        }
    }

    #[test]
    fn draw_at_60() {
        let mut g = Briscola::with_seed(1);
        g.hands = [vec![], vec![]];
        g.stock.clear();
        g.pts = [60, 60];
        assert_eq!(g.outcome(), Some(Outcome::Draw));
        g.pts = [61, 59];
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn render_hides_the_opponent() {
        let g = Briscola::with_seed(5);
        let r = Game::render(&g);
        assert!(r.contains("Jucător 2 (ascuns) are 3 cărți"));
        let hand: Vec<String> = g.hands[0].iter().map(|&c| card_text(c)).collect();
        for x in &g.hands[1] {
            let t = card_text(*x);
            if !hand.contains(&t) && Some(*x) != g.stock.first().copied() {
                assert!(!r.split("Mâna ta").nth(1).unwrap().contains(&t));
            }
        }
    }

    #[test]
    fn evaluate_ignores_the_opponents_cards() {
        let g = Briscola::with_seed(9);
        let mut h = g.clone();
        let (a, b) = (h.hands[1][0], h.stock[5]);
        h.hands[1][0] = b;
        h.stock[5] = a;
        assert_eq!(g.evaluate(Player::One), h.evaluate(Player::One));
    }

    #[test]
    fn ai_beats_a_random_mover_mostly() {
        use polytabletop_core::GameObj;
        let mut wins = 0;
        let n = 30;
        for seed in 1..=n {
            let mut g: Box<dyn GameObj> = Box::new(Briscola::with_seed(seed));
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

#[cfg(test)]
mod a3_audit {
    use super::*;

    #[test]
    fn ai_ignores_the_opponent_hand_and_stock_at_every_level() {
        crate::p3_comun::ai_blind_check(
            "briscola",
            Briscola::with_seed,
            |g, me, s| {
                let h = 1 - me.idx();
                // stock[0] = atuul întors (public)
                crate::p3_comun::mix_unseen(&mut g.hands[h], &mut g.stock, 1, s);
            },
            10,
        );
    }
}
