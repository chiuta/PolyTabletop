//! Loto cu imagini (Picture lotto), 2 jucători — variantă pe cartonașe 3×3.
//!
//! * 24 de imagini (cuvinte); fiecare jucător are un cartonaș cu 9 imagini diferite, public.
//! * La rândul tău tragi din sac o imagine (`trage`). Dacă imaginea e pe cartonașul tău poți să o
//!   acoperi (`acopera`) sau să o lași (`pas`); apoi, dacă ea e și pe cartonașul adversarului, el
//!   poate la fel să o acopere. Cine acoperă primul tot cartonașul (9 imagini) strigă «loto» și câștigă;
//!   dacă sacul se golește, câștigă cine are mai multe imagini acoperite (egal = remiză).
//! * Fiecare jucător alege pe rând cine trage: cel care a tras ultimul cedează tragerea.
//!
//! Mutări: `trage`, `acopera`, `pas`.
#![allow(clippy::needless_range_loop)]

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const NAMES: [&str; 24] = [
    "pisica", "caine", "soare", "luna", "stea", "casa", "copac", "floare", "mar", "para", "barca",
    "tren", "avion", "minge", "carte", "cheie", "ceas", "pește", "pasare", "nor", "umbrela",
    "clopot", "balon", "tobă",
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Draw,
    Cover,
    Pass,
}

#[derive(Clone)]
pub struct LotoImagini {
    rng: Rng,
    bag: Vec<u8>,
    cards: [[u8; 9]; 2],
    covered: [[bool; 9]; 2],
    side: Player,
    drawer: Player,
    current: Option<u8>,
    log: String,
}

impl Default for LotoImagini {
    fn default() -> Self {
        Self::new()
    }
}

impl LotoImagini {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut all: Vec<u8> = (0..24).collect();
        crate::q6_util::shuffle(&mut all, &mut rng);
        let mut cards = [[0u8; 9]; 2];
        for p in 0..2 {
            let mut c: Vec<u8> = (0..24).collect();
            crate::q6_util::shuffle(&mut c, &mut rng);
            cards[p].copy_from_slice(&c[..9]);
            cards[p].sort();
        }
        LotoImagini {
            rng,
            bag: all,
            cards,
            covered: [[false; 9]; 2],
            side: Player::One,
            drawer: Player::One,
            current: None,
            log: String::new(),
        }
    }
    fn slot(&self, p: Player, img: u8) -> Option<usize> {
        self.cards[p.idx()].iter().position(|&x| x == img)
    }
    fn count(&self, p: usize) -> usize {
        self.covered[p].iter().filter(|&&x| x).count()
    }
    fn can_cover(&self, p: Player) -> bool {
        self.current
            .is_some_and(|i| self.slot(p, i).is_some_and(|k| !self.covered[p.idx()][k]))
    }
    /// Următorul jucător care poate reacționa după `after`, sau None dacă nu mai e nimeni.
    fn next_react(&self, after: Option<Player>) -> Option<Player> {
        let order = match after {
            None => [Some(self.drawer), Some(self.drawer.other())],
            Some(p) if p == self.drawer => [Some(self.drawer.other()), None],
            _ => [None, None],
        };
        order.into_iter().flatten().find(|&p| self.can_cover(p))
    }
    fn finish_draw(&mut self, after: Option<Player>) {
        match self.next_react(after) {
            Some(p) => self.side = p,
            None => {
                self.current = None;
                self.drawer = self.drawer.other();
                self.side = self.drawer;
            }
        }
    }
}

impl Game for LotoImagini {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return vec![];
        }
        if self.current.is_none() {
            vec![Move::Draw]
        } else {
            vec![Move::Cover, Move::Pass]
        }
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        match m {
            Move::Draw => {
                let i = self.bag.pop().unwrap();
                self.current = Some(i);
                self.log = format!("S-a tras: {}.", NAMES[i as usize]);
                self.finish_draw(None);
            }
            Move::Cover => {
                let i = self.current.unwrap();
                let k = self.slot(self.side, i).unwrap();
                self.covered[self.side.idx()][k] = true;
                let me = self.side;
                self.finish_draw(Some(me));
            }
            Move::Pass => {
                let me = self.side;
                self.finish_draw(Some(me));
            }
        }
        let _ = &mut self.rng;
    }

    fn outcome(&self) -> Option<Outcome> {
        for p in [Player::One, Player::Two] {
            if self.count(p.idx()) == 9 {
                return Some(Outcome::Win(p));
            }
        }
        if self.bag.is_empty() && self.current.is_none() {
            return Some(match self.count(0).cmp(&self.count(1)) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                std::cmp::Ordering::Equal => Outcome::Draw,
            });
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        (self.count(p.idx()) as i32 - self.count(p.other().idx()) as i32) * 10
    }

    fn render(&self) -> String {
        let mut s = format!("Loto cu imagini — în sac: {} imagini\n", self.bag.len());
        for p in 0..2 {
            s.push_str(&format!("Cartonașul J{} ({}/9):", p + 1, self.count(p)));
            for k in 0..9 {
                if k % 3 == 0 {
                    s.push_str("\n  ");
                }
                s.push_str(&format!(
                    "{}{:<9}",
                    if self.covered[p][k] { "[x]" } else { "[ ]" },
                    NAMES[self.cards[p][k] as usize]
                ));
            }
            s.push('\n');
        }
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        match self.outcome() {
            Some(Outcome::Win(p)) => {
                s.push_str(&format!("Loto! Jucătorul {} câștigă.\n", p.idx() + 1))
            }
            Some(Outcome::Draw) => s.push_str("Remiză.\n"),
            None => {
                if let Some(i) = self.current {
                    s.push_str(&format!(
                        "Imaginea «{}» e pe cartonașul tău: «acopera» sau «pas».\n",
                        NAMES[i as usize]
                    ));
                }
                s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
            }
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Draw => "trage",
            Move::Cover => "acopera",
            Move::Pass => "pas",
        }
        .into()
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_games_end_and_cards_valid() {
        for seed in 0..40u64 {
            let mut g = LotoImagini::with_seed(seed);
            for p in 0..2 {
                let mut c = g.cards[p].to_vec();
                c.dedup();
                assert_eq!(c.len(), 9);
            }
            let mut r = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[r.below(m.len())]);
                n += 1;
                assert!(n < 500);
            }
        }
    }

    #[test]
    fn cover_always_helps_ai_picks_cover() {
        let mut g = LotoImagini::with_seed(2);
        while g.current.is_none() || !g.can_cover(g.side) {
            if g.outcome().is_some() {
                return;
            }
            let m = g.legal_moves()[0];
            g.apply(&m);
        }
        let mut a = g.clone();
        let mut b = g.clone();
        a.apply(&Move::Cover);
        b.apply(&Move::Pass);
        assert!(a.evaluate(g.side) > b.evaluate(g.side));
    }
}
