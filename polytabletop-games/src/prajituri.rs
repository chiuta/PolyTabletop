//! Drumul Prăjiturilor — cursă de culori pe un drum de 58 de pătrate (variantă originală,
//! mecanica „trage o carte de culoare și mergi la următorul pătrat de acea culoare”).
//!
//! Reguli folosite (ale noastre):
//! * Drumul are pătrate numerotate 0 (start) .. 57 (Palatul de zahăr). Culoarea pătratului `i`
//!   este `(i + i/6) % 6` (roșu, portocaliu, galben, verde, albastru, mov).
//! * Pachetul de 39 de cărți: câte 4 cărți simple și 2 duble pentru fiecare dintre cele 6 culori,
//!   plus 3 cărți „loc” (Peștera de ciocolată → 18, Pădurea de bomboane → 33, Castelul de caramel → 46).
//!   Cartea se trage automat și se vede în `render()`; singura mutare este `trag` (ca la
//!   „Șerpi și scări”, nu există alegeri reale, deci AI-ul nu poate „vedea” nimic ascuns).
//! * Carte simplă: avansezi la prima culoare identică de după poziția curentă; dublă: la a doua.
//!   Dacă nu mai există, ajungi în Palat și câștigi.
//! * Scurtături (început → sfârșit): 8→22, 25→41, 36→49. Pătrate lipicioase: 14, 31, 44
//!   (pierzi următoarea tură). Cartea „loc” poate trimite și înapoi.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const LAST: u8 = 57;
pub const SHORTCUTS: [(u8, u8); 3] = [(8, 22), (25, 41), (36, 49)];
pub const STICKY: [u8; 3] = [14, 31, 44];
pub const LANDMARKS: [(&str, u8); 3] = [
    ("Peștera de ciocolată", 18),
    ("Pădurea de bomboane", 33),
    ("Castelul de caramel", 46),
];
const COLOR_NAMES: [&str; 6] = ["roșu", "portocaliu", "galben", "verde", "albastru", "mov"];

pub fn color_of(i: u8) -> u8 {
    ((i as usize + i as usize / 6) % 6) as u8
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Card {
    Single(u8),
    Double(u8),
    Landmark(u8),
}

impl Card {
    fn text(self) -> String {
        match self {
            Card::Single(c) => format!("{} simplu", COLOR_NAMES[c as usize]),
            Card::Double(c) => format!("{} dublu", COLOR_NAMES[c as usize]),
            Card::Landmark(i) => LANDMARKS[i as usize].0.to_string(),
        }
    }
}

fn full_deck() -> Vec<Card> {
    let mut v = Vec::new();
    for c in 0..6u8 {
        for _ in 0..4 {
            v.push(Card::Single(c));
        }
        for _ in 0..2 {
            v.push(Card::Double(c));
        }
    }
    for i in 0..3u8 {
        v.push(Card::Landmark(i));
    }
    v
}

#[derive(Clone)]
pub struct Prajituri {
    pos: [u8; 2],
    skip: [bool; 2],
    side: Player,
    deck: Vec<Card>,
    card: Option<Card>,
    rng: Rng,
    log: String,
}

impl Default for Prajituri {
    fn default() -> Self {
        Self::new()
    }
}

impl Prajituri {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Prajituri {
            pos: [0; 2],
            skip: [false; 2],
            side: Player::One,
            deck: Vec::new(),
            card: None,
            rng: Rng::new(seed),
            log: String::new(),
        };
        g.draw();
        g
    }

    pub fn positions(&self) -> (u8, u8) {
        (self.pos[0], self.pos[1])
    }

    fn draw(&mut self) {
        if self.deck.is_empty() {
            self.deck = full_deck();
            for i in (1..self.deck.len()).rev() {
                let j = self.rng.below(i + 1);
                self.deck.swap(i, j);
            }
        }
        self.card = self.deck.pop();
    }

    fn next_of_color(from: u8, color: u8, nth: u8) -> u8 {
        let mut left = nth;
        for i in (from + 1)..=LAST {
            if color_of(i) == color && i != LAST {
                left -= 1;
                if left == 0 {
                    return i;
                }
            }
        }
        LAST
    }
}

impl Game for Prajituri {
    type Move = ();

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<()> {
        if self.outcome().is_some() {
            Vec::new()
        } else {
            vec![()]
        }
    }

    fn apply(&mut self, _m: &()) {
        let i = self.side.idx();
        if self.skip[i] {
            self.skip[i] = false;
            self.log = format!("Jucătorul {}: e lipit de caramel și pierde tura.", i + 1);
        } else if let Some(card) = self.card {
            let from = self.pos[i];
            let mut to = match card {
                Card::Single(c) => Self::next_of_color(from, c, 1),
                Card::Double(c) => Self::next_of_color(from, c, 2),
                Card::Landmark(k) => LANDMARKS[k as usize].1,
            };
            self.log = format!(
                "Jucătorul {}: carte «{}», {} → {}",
                i + 1,
                card.text(),
                from,
                to
            );
            if let Some(&(_, t)) = SHORTCUTS.iter().find(|s| s.0 == to) {
                self.log.push_str(&format!(", scurtătură spre {t}"));
                to = t;
            } else if STICKY.contains(&to) {
                self.log
                    .push_str(", pătrat lipicios (pierde tura următoare)");
                self.skip[i] = true;
            }
            self.log.push('.');
            self.pos[i] = to;
            if to == LAST {
                self.card = None;
                return;
            }
        }
        self.side = self.side.other();
        if self.skip[self.side.idx()] {
            self.card = None;
        } else {
            self.draw();
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.pos[0] == LAST {
            Some(Outcome::Win(Player::One))
        } else if self.pos[1] == LAST {
            Some(Outcome::Win(Player::Two))
        } else {
            None
        }
    }

    fn evaluate(&self, p: Player) -> i32 {
        self.pos[p.idx()] as i32 - self.pos[p.other().idx()] as i32
    }

    fn render(&self) -> String {
        let mut s = String::from("Drumul Prăjiturilor (0 start … 57 Palatul de zahăr)\n");
        s.push_str("Scurtături: ");
        for (a, b) in SHORTCUTS {
            s.push_str(&format!("{a}→{b} "));
        }
        s.push_str("\nLipicioase: ");
        for a in STICKY {
            s.push_str(&format!("{a} "));
        }
        s.push_str("\nLocuri: ");
        for (n, p) in LANDMARKS {
            s.push_str(&format!("{n}={p} "));
        }
        s.push_str("\nCulori de la 1: ");
        for i in 1..=12u8 {
            s.push_str(&format!(
                "{}:{} ",
                i,
                &COLOR_NAMES[color_of(i) as usize][..1]
            ));
        }
        s.push_str("… (culoarea lui i = (i + i/6) mod 6)\n");
        s.push_str(&format!(
            "Jucător 1 (X): {}   Jucător 2 (O): {}\n",
            self.pos[0], self.pos[1]
        ));
        if self.outcome().is_none() {
            match self.card {
                Some(c) => s.push_str(&format!(
                    "La mutare: jucătorul {}   Carte: {}   (scrie «trag»)\n",
                    self.side.idx() + 1,
                    c.text()
                )),
                None => s.push_str(&format!(
                    "La mutare: jucătorul {} (lipit: pierde tura; scrie «trag»)\n",
                    self.side.idx() + 1
                )),
            }
        }
        s.push_str(self.log.trim());
        s.push('\n');
        s
    }

    fn format_move(&self, _m: &()) -> String {
        "trag".to_string()
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_cycle_and_every_color_appears() {
        for c in 0..6u8 {
            assert!((1..LAST).filter(|&i| color_of(i) == c).count() > 5);
        }
    }

    #[test]
    fn deck_has_39_cards() {
        assert_eq!(full_deck().len(), 39);
    }

    #[test]
    fn shortcuts_and_sticky_are_consistent() {
        for (a, b) in SHORTCUTS {
            assert!(b > a && b < LAST);
            assert!(!STICKY.contains(&a) && !STICKY.contains(&b));
            assert!(!SHORTCUTS.iter().any(|s| s.0 == b));
        }
        for (_, p) in LANDMARKS {
            assert!(p < LAST && !SHORTCUTS.iter().any(|s| s.0 == p) && !STICKY.contains(&p));
        }
    }

    #[test]
    fn single_and_double_move_to_next_color() {
        let mut g = Prajituri::with_seed(1);
        g.pos[0] = 0;
        g.card = Some(Card::Single(color_of(2)));
        g.apply(&());
        let first = (1..=LAST).find(|&i| color_of(i) == color_of(2)).unwrap();
        assert_eq!(g.pos[0], first);
        let mut g = Prajituri::with_seed(1);
        g.card = Some(Card::Double(color_of(2)));
        g.apply(&());
        let second = (1..LAST)
            .filter(|&i| color_of(i) == color_of(2))
            .nth(1)
            .unwrap();
        assert_eq!(g.pos[0], second);
    }

    #[test]
    fn shortcut_and_sticky_work() {
        let mut g = Prajituri::with_seed(1);
        g.pos[0] = 0;
        g.card = Some(Card::Single(color_of(8)));
        // primul pătrat cu culoarea lui 8 poate fi înaintea lui 8: forțăm poziția
        g.pos[0] = 7;
        g.apply(&());
        assert!(g.pos[0] == 22 || color_of(g.pos[0]) == color_of(8));
        let mut g = Prajituri::with_seed(1);
        g.pos[0] = 12;
        g.card = Some(Card::Landmark(0));
        g.apply(&());
        assert_eq!(g.pos[0], 18);
        let mut g = Prajituri::with_seed(2);
        g.pos[0] = 13;
        g.card = Some(Card::Single(color_of(14)));
        g.apply(&());
        if g.pos[0] == 14 {
            assert!(g.skip[0]);
            assert!(g.card.is_some());
            g.apply(&()); // jucătorul 2
            assert_eq!(g.side, Player::One);
            assert!(g.card.is_none());
            g.apply(&()); // 1 pierde tura
            assert!(!g.skip[0]);
            assert_eq!(g.pos[0], 14);
        }
    }

    #[test]
    fn games_finish_and_are_deterministic() {
        for seed in 1..=30u64 {
            let run = || {
                let mut g = Prajituri::with_seed(seed);
                let mut n = 0;
                while g.outcome().is_none() && n < 5000 {
                    g.apply(&());
                    n += 1;
                }
                assert!(g.outcome().is_some(), "seed {seed}");
                (g.outcome(), g.positions())
            };
            assert_eq!(run(), run());
        }
    }

    #[test]
    fn reaching_end_wins() {
        let mut g = Prajituri::with_seed(1);
        g.pos[1] = 55;
        g.side = Player::Two;
        g.card = Some(Card::Double(color_of(56)));
        g.apply(&());
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
    }
}
