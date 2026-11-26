//! Ganjifa (India/Persia) — variantă cu levate pentru 2 jucători.
//!
//! Ganjifa e jocul tradițional de cărți rotunde al curților mogule. Aici folosim o variantă
//! documentată, în spiritul lui Hukm/„Mughal ganjifa” pentru doi:
//! * 48 de cărți: 4 culori (Taj = coroane `T`, Shamsher = săbii `S`, Surkh = monede `R`,
//!   Chang = harpe `C`), fiecare cu Mir (rege, `M`), Vazir (`V`) și numerele 1–10.
//!   Ca în ganjifa mogulă, ordinea numerelor depinde de culoare: la `T` și `S` cresc (10 e cel mai
//!   mare), la `R` și `C` sunt **inversate** (1 e cel mai mare). Mir și Vazir bat numerele.
//! * Fiecare jucător primește 12 cărți; celelalte 24 rămân în stoc, ascunse, dar cartea de
//!   deasupra se întoarce și arată **culoarea de atu (hukm)**.
//! * 12 levate. Conduce J1 în prima; câștigătorul unei levate conduce următoarea. Trebuie să
//!   dai din culoarea cerută dacă ai; altfel poți juca orice. Levata o ia cel mai mare atu,
//!   iar dacă nu există atu, cea mai mare carte din culoarea cerută.
//! * Câștigă cine are mai multe levate (7 din 12 sunt suficiente).
//! * Informație ascunsă: vezi doar mâna ta; AI-ul judecă după propria mână și cărțile jucate.

use crate::cards::Viewer;
use crate::cards52::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const SUITS: [char; 4] = ['T', 'S', 'R', 'C'];
pub const SUIT_NAMES: [&str; 4] = ["Taj", "Shamsher", "Surkh", "Chang"];

/// carte = culoare * 12 + rang0 (rang0 0..=9 → numerele 1..=10, 10 = Vazir, 11 = Mir)
pub fn suit(c: u8) -> u8 {
    c / 12
}
pub fn rk(c: u8) -> u8 {
    c % 12
}

pub fn text(c: u8) -> String {
    let r = match rk(c) {
        10 => "V".to_string(),
        11 => "M".to_string(),
        n => (n + 1).to_string(),
    };
    format!("{}{}", SUITS[suit(c) as usize], r)
}

/// Forța unei cărți în culoarea ei (mai mare = mai tare).
pub fn strength(c: u8) -> i32 {
    match rk(c) {
        11 => 100,
        10 => 90,
        n => {
            if suit(c) >= 2 {
                // culori inversate: 1 e cel mai mare
                50 - n as i32
            } else {
                n as i32
            }
        }
    }
}

#[derive(Clone)]
pub struct Ganjifa {
    hands: [Vec<u8>; 2],
    stock: Vec<u8>,
    trump: u8,
    led: Option<u8>,
    side: Player,
    tricks: [u8; 2],
    played: Vec<u8>,
    last: String,
    viewer: Viewer,
}

impl Default for Ganjifa {
    fn default() -> Self {
        Self::new()
    }
}

impl Ganjifa {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut d: Vec<u8> = (0..48).collect();
        shuffle(&mut d, &mut rng);
        let mut h0: Vec<u8> = d.drain(..12).collect();
        let mut h1: Vec<u8> = d.drain(..12).collect();
        h0.sort_unstable();
        h1.sort_unstable();
        let trump = suit(*d.last().unwrap());
        Ganjifa {
            hands: [h0, h1],
            stock: d,
            trump,
            led: None,
            side: Player::One,
            tricks: [0; 2],
            played: Vec::new(),
            last: String::new(),
            viewer: Viewer::Fixed(Player::One),
        }
    }

    pub fn tricks(&self) -> (u8, u8) {
        (self.tricks[0], self.tricks[1])
    }

    fn beats(&self, lead: u8, reply: u8) -> bool {
        // `true` dacă răspunsul bate cartea de deschidere
        let (sl, sr) = (suit(lead), suit(reply));
        if sr == sl {
            strength(reply) > strength(lead)
        } else {
            sr == self.trump
        }
    }

    fn unseen_higher(&self, c: u8, me: usize) -> i32 {
        // câte cărți din aceeași culoare, mai tari, nu sunt nici în mâna mea, nici jucate
        (0..48u8)
            .filter(|&x| {
                suit(x) == suit(c)
                    && strength(x) > strength(c)
                    && !self.hands[me].contains(&x)
                    && !self.played.contains(&x)
            })
            .count() as i32
    }
}

impl Game for Ganjifa {
    type Move = u8;

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = match who {
            Some(p) => Viewer::Fixed(p),
            None => Viewer::HotSeat,
        };
    }

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<u8> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        let hand = &self.hands[self.side.idx()];
        match self.led {
            None => hand.clone(),
            Some(l) => {
                let follow: Vec<u8> = hand
                    .iter()
                    .copied()
                    .filter(|&c| suit(c) == suit(l))
                    .collect();
                if follow.is_empty() {
                    hand.clone()
                } else {
                    follow
                }
            }
        }
    }

    fn apply(&mut self, &c: &u8) {
        let me = self.side;
        self.hands[me.idx()].retain(|&x| x != c);
        self.played.push(c);
        match self.led {
            None => {
                self.led = Some(c);
                self.side = me.other();
            }
            Some(l) => {
                // `me` a răspuns; cel care a condus e `me.other()`
                let winner = if self.beats(l, c) { me } else { me.other() };
                self.tricks[winner.idx()] += 1;
                self.last = format!("Levata: {} / {} → J{}", text(l), text(c), winner.idx() + 1);
                self.led = None;
                self.side = winner;
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.tricks[0] + self.tricks[1] < 12 {
            return None;
        }
        Some(if self.tricks[0] > self.tricks[1] {
            Outcome::Win(Player::One)
        } else {
            Outcome::Win(Player::Two)
        })
    }

    /// Doar mâna proprie, cărțile jucate (publice) și levatele.
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut s = 200 * (self.tricks[me] as i32 - self.tricks[1 - me] as i32);
        for &c in &self.hands[me] {
            let higher = self.unseen_higher(c, me);
            let mut v = 40 - 12 * higher.min(3);
            if suit(c) == self.trump {
                v += 10;
            }
            s += v;
        }
        // o carte deschisă de adversar: valoare dacă o pot bate
        if let Some(l) = self.led {
            if self.side == p {
                let can = self.hands[me].iter().any(|&c| self.beats(l, c));
                if can {
                    s += 5;
                }
            }
        }
        s
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Ganjifa — levate: J1 {}, J2 {} | atu (hukm): {} ({})\n",
            self.tricks[0],
            self.tricks[1],
            SUITS[self.trump as usize],
            SUIT_NAMES[self.trump as usize]
        );
        if !self.last.is_empty() {
            s.push_str(&format!("{}\n", self.last));
        }
        if self.outcome().is_some() {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        if let Some(l) = self.led {
            s.push_str(&format!(
                "Pe masă: {} (deschidere J{})\n",
                text(l),
                self.side.other().idx() + 1
            ));
        }
        let v = self.viewer.who(self.side);
        let hand: Vec<String> = self.hands[v.idx()].iter().map(|&c| text(c)).collect();
        s.push_str(&format!("Mâna ta (J{}): {}\n", v.idx() + 1, hand.join(" ")));
        s.push_str(&format!(
            "J{}: {} cărți | stoc ascuns nefolosit: {}\n",
            v.other().idx() + 1,
            self.hands[v.other().idx()].len(),
            self.stock.len()
        ));
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        s
    }

    fn format_move(&self, m: &u8) -> String {
        text(*m)
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn card(s: u8, r: u8) -> u8 {
        s * 12 + r
    }

    #[test]
    fn strength_order_normal_and_reversed() {
        // Taj (normal): 10 > 1; Surkh (inversat): 1 > 10
        assert!(strength(card(0, 9)) > strength(card(0, 0)));
        assert!(strength(card(2, 0)) > strength(card(2, 9)));
        assert!(strength(card(2, 10)) > strength(card(2, 0)));
        assert!(strength(card(0, 11)) > strength(card(0, 10)));
    }

    #[test]
    fn texts_unique_and_deal_conserves() {
        let mut t: Vec<String> = (0..48).map(text).collect();
        t.sort();
        t.dedup();
        assert_eq!(t.len(), 48);
        let g = Ganjifa::with_seed(5);
        let mut all = g.stock.clone();
        all.extend(&g.hands[0]);
        all.extend(&g.hands[1]);
        all.sort_unstable();
        assert_eq!(all, (0..48).collect::<Vec<u8>>());
        assert_eq!(g.hands[0].len(), 12);
        assert_eq!(suit(*g.stock.last().unwrap()), g.trump);
    }

    #[test]
    fn follow_suit_and_trump_wins() {
        let mut g = Ganjifa::with_seed(1);
        g.trump = 1;
        g.hands[0] = vec![card(0, 3), card(1, 0)];
        g.hands[1] = vec![card(0, 9), card(1, 2), card(2, 5)];
        g.apply(&card(0, 3));
        // J2 trebuie să dea din Taj
        assert_eq!(g.legal_moves(), vec![card(0, 9)]);
        g.apply(&card(0, 9));
        assert_eq!(g.tricks(), (0, 1));
        assert_eq!(g.side, Player::Two);
        // acum J2 conduce Taj? nu mai are; conduce Shamsher (atu) și J1 răspunde cu atu mai mic
        g.hands[1] = vec![card(1, 2), card(2, 5)];
        g.apply(&card(2, 5)); // Surkh
                              // J1 nu are Surkh: poate juca atu -> câștigă
        assert_eq!(g.legal_moves(), vec![card(1, 0)]);
        g.apply(&card(1, 0));
        assert_eq!(g.tricks(), (1, 1));
    }

    #[test]
    fn higher_of_led_suit_wins_without_trump() {
        let mut g = Ganjifa::with_seed(1);
        g.trump = 3;
        g.hands[0] = vec![card(2, 0)];
        g.hands[1] = vec![card(2, 9)];
        g.apply(&card(2, 0)); // Surkh 1 (cel mai mare, inversat)
        g.apply(&card(2, 9)); // Surkh 10
        assert_eq!(g.tricks(), (1, 0));
    }

    #[test]
    fn render_hides_opponent_and_ai_blind() {
        let g = Ganjifa::with_seed(2);
        let r = Game::render(&g);
        assert!(r.contains("J2: 12 cărți"));
        for seed in 1..30u64 {
            let mut a = Ganjifa::with_seed(seed);
            let mut rng = Rng::new(seed);
            for _ in 0..(seed % 10) {
                let mv = a.legal_moves();
                if mv.is_empty() {
                    break;
                }
                a.apply(&mv[rng.below(mv.len())]);
            }
            if a.outcome().is_some() {
                continue;
            }
            let me = a.side.idx();
            let mut b = a.clone();
            let mut pool = b.stock.clone();
            pool.extend(&b.hands[1 - me]);
            // păstrează cartea de atu deasupra stocului
            let top = pool.remove(b.stock.len() - 1);
            shuffle(&mut pool, &mut rng);
            let n = b.hands[1 - me].len();
            b.hands[1 - me] = pool.drain(..n).collect();
            pool.push(top);
            b.stock = pool;
            assert_eq!(
                polytabletop_core::ai::best_move(&a, 5),
                polytabletop_core::ai::best_move(&b, 5)
            );
        }
    }

    #[test]
    fn random_games_end_with_12_tricks() {
        for seed in 1..60u64 {
            let mut g = Ganjifa::with_seed(seed);
            let mut rng = Rng::new(seed ^ 1);
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
            }
            assert_eq!(g.tricks[0] + g.tricks[1], 12);
        }
    }
}
