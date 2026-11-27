//! Spoons (lingurile), joc de petrecere cu cărți — adaptare pentru doi jucători.
//!
//! * Pachet de 52 de cărți, contează doar rangul. Fiecare primește 4 cărți; în mijloc e **o singură
//!   lingură**. Scopul: să strângi patru cărți de același rang și să iei lingura.
//! * Dealer-ul (alternează în fiecare rundă) trage din pachet, celălalt trage ce a aruncat dealer-ul.
//!   Tura: `iau` (tragi cartea), apoi `dau 7` (arunci o carte de rang 7; rangurile: A 2…10 J Q K) sau, dacă
//!   ai patru la fel, `lingura`. Cine ia lingura câștigă runda; celălalt primește o literă din
//!   „SPOONS” (6 litere). Cine strânge toate literele pierde; după 12 runde câștigă cine are mai puține.
//! * Simplificări față de jocul în timp real: nu există „reacția” celorlalți (cu o singură lingură în
//!   doi, cine are patru la fel o ia imediat) și nu se poate lua lingura fără patru la fel.
//!   Dacă pachetul se termină, runda se anulează (fără literă).
//!   Informație ascunsă: mâna adversarului, pachetul și cărțile aruncate încă neluate.
//!   Mutări: `iau`, `dau K`, `lingura`, `mai-departe` (începe runda următoare).

use crate::cards::Viewer;
use crate::cards52::shuffle;
use crate::q5_util::{by_score, verdict};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::collections::VecDeque;

const LABELS: [&str; 13] = [
    "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
];
const ROUNDS: u8 = 12;
const MAX_LETTERS: u8 = 6;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Take,
    Give(u8),
    Spoon,
    Next,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Take,
    Give,
    Between,
}

#[derive(Clone)]
pub struct Spoons {
    hands: [Vec<u8>; 2], // ranguri 0..13
    stock: Vec<u8>,
    queue: VecDeque<u8>, // aruncate de dealer, încă neluate de celălalt
    letters: [u8; 2],
    dealer: Player,
    side: Player,
    phase: Phase,
    round: u8,
    rng: Rng,
    viewer: Viewer,
    log: String,
}

impl Default for Spoons {
    fn default() -> Self {
        Self::new()
    }
}

fn quad(h: &[u8]) -> bool {
    (0..13u8).any(|r| h.iter().filter(|&&x| x == r).count() >= 4)
}

impl Spoons {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Spoons {
            hands: [Vec::new(), Vec::new()],
            stock: Vec::new(),
            queue: VecDeque::new(),
            letters: [0; 2],
            dealer: Player::Two,
            side: Player::One,
            phase: Phase::Take,
            round: 0,
            rng: Rng::new(seed),
            viewer: Viewer::Fixed(Player::One),
            log: String::new(),
        };
        g.deal();
        g
    }

    fn deal(&mut self) {
        let mut deck: Vec<u8> = (0..52u8).map(|c| c % 13).collect();
        shuffle(&mut deck, &mut self.rng);
        self.hands[0] = deck.drain(..4).collect();
        self.hands[1] = deck.drain(..4).collect();
        self.hands[0].sort_unstable();
        self.hands[1].sort_unstable();
        self.stock = deck;
        self.queue.clear();
        self.side = self.dealer;
        self.phase = Phase::Take;
    }

    pub fn letters(&self) -> [u8; 2] {
        self.letters
    }

    fn end_round(&mut self, loser: Option<Player>) {
        if let Some(l) = loser {
            self.letters[l.idx()] += 1;
        }
        self.round += 1;
        self.phase = Phase::Between;
        self.side = self.dealer;
    }

    fn count_quality(h: &[u8]) -> i32 {
        let mut s = 0;
        for r in 0..13u8 {
            let c = h.iter().filter(|&&x| x == r).count() as i32;
            s += c * c * c;
        }
        s
    }
}

impl Game for Spoons {
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
        match self.phase {
            Phase::Between => vec![Move::Next],
            Phase::Take => vec![Move::Take],
            Phase::Give => {
                let h = &self.hands[self.side.idx()];
                let mut v = Vec::new();
                if quad(h) {
                    v.push(Move::Spoon);
                }
                let mut seen = [false; 13];
                for &r in h {
                    if !seen[r as usize] {
                        seen[r as usize] = true;
                        v.push(Move::Give(r));
                    }
                }
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        let me = self.side;
        match m {
            Move::Next => {
                self.dealer = self.dealer.other();
                self.deal();
                self.log.clear();
            }
            Move::Take => {
                let c = if me == self.dealer {
                    self.stock.pop()
                } else {
                    self.queue.pop_front()
                };
                match c {
                    Some(c) => {
                        self.hands[me.idx()].push(c);
                        self.phase = Phase::Give;
                    }
                    None => {
                        self.log = "Pachetul s-a terminat: runda se anulează.".into();
                        self.end_round(None);
                    }
                }
            }
            Move::Give(r) => {
                let h = &mut self.hands[me.idx()];
                let pos = h.iter().position(|x| x == r).unwrap();
                h.remove(pos);
                h.sort_unstable();
                if me == self.dealer {
                    self.queue.push_back(*r);
                }
                self.side = me.other();
                self.phase = Phase::Take;
            }
            Move::Spoon => {
                self.log = format!(
                    "J{} ia lingura! J{} primește o literă.",
                    me.idx() + 1,
                    2 - me.idx()
                );
                self.end_round(Some(me.other()));
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.letters[0] >= MAX_LETTERS
            || self.letters[1] >= MAX_LETTERS
            || (self.round >= ROUNDS && self.phase == Phase::Between)
        {
            return Some(by_score(
                -(self.letters[0] as i32),
                -(self.letters[1] as i32),
            ));
        }
        None
    }

    /// Doar propria mână (după ce a tras) și literele; fără pachet, fără mâna adversarului.
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut v = (self.letters[1 - me] as i32 - self.letters[me] as i32) * 1000;
        if self.phase != Phase::Between {
            v += Self::count_quality(&self.hands[me]);
        }
        v
    }

    fn render(&self) -> String {
        let word = "SPOONS";
        let show = |n: u8| -> String { word.chars().take(n as usize).collect() };
        let mut s = format!(
            "Spoons — runda {}/{ROUNDS}. Litere: J1 „{}”, J2 „{}”. Dealer: J{}.\n",
            (self.round + 1).min(ROUNDS),
            show(self.letters[0]),
            show(self.letters[1]),
            self.dealer.idx() + 1
        );
        s.push_str(&format!(
            "Pachet: {} cărți; aruncate în drum: {}.\n",
            self.stock.len(),
            self.queue.len()
        ));
        let v = self.viewer.who(self.side);
        let o = v.other();
        s.push_str(&format!(
            "Jucător {} (ascuns) ține {} cărți.\n",
            o.idx() + 1,
            self.hands[o.idx()].len()
        ));
        let h: Vec<&str> = self.hands[v.idx()]
            .iter()
            .map(|&r| LABELS[r as usize])
            .collect();
        s.push_str(&format!(
            "Mâna ta (Jucător {}): {}\n",
            v.idx() + 1,
            h.join(" ")
        ));
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        let fin = verdict(self.outcome());
        if fin.is_empty() {
            let what = match self.phase {
                Phase::Take => "iau",
                Phase::Give => "dau <rang> sau lingura",
                Phase::Between => "mai-departe",
            };
            s.push_str(&format!(
                "La mutare: Jucător {} — {what}\n",
                self.side.idx() + 1
            ));
        } else {
            s.push_str(&fin);
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Take => "iau".into(),
            Move::Give(r) => format!("dau {}", LABELS[*r as usize]),
            Move::Spoon => "lingura".into(),
            Move::Next => "mai-departe".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q5_util::testkit::check_game;

    #[test]
    fn four_of_a_kind_takes_the_spoon() {
        let mut g = Spoons::with_seed(3);
        g.hands[0] = vec![4, 4, 4, 4];
        g.dealer = Player::One;
        g.side = Player::One;
        g.stock = vec![9, 2];
        g.apply(&Move::Take);
        let m = g.legal_moves();
        assert!(m.contains(&Move::Spoon));
        g.apply(&Move::Spoon);
        assert_eq!(g.letters(), [0, 1]);
        assert_eq!(g.legal_moves(), vec![Move::Next]);
        g.apply(&Move::Next);
        assert_eq!(g.dealer, Player::Two);
    }

    #[test]
    fn spoon_not_offered_without_four() {
        let g = Spoons::with_seed(5);
        let mut h = g.clone();
        h.hands[0] = vec![1, 2, 3, 4];
        h.dealer = Player::One;
        h.side = Player::One;
        h.stock = vec![5];
        h.apply(&Move::Take);
        assert!(!h.legal_moves().contains(&Move::Spoon));
    }

    #[test]
    fn ai_does_not_see_the_other_hand() {
        let mut a = Spoons::with_seed(6);
        a.apply(&Move::Take);
        let mut b = a.clone();
        let opp = a.side.other().idx();
        b.hands[opp] = vec![12, 12, 12, 11];
        b.stock = vec![0; b.stock.len()];
        for seed in 1..10 {
            assert_eq!(
                polytabletop_core::GameObj::ai_move(&a, seed),
                polytabletop_core::GameObj::ai_move(&b, seed)
            );
        }
        let mut one = a.clone();
        one.set_viewer(Some(Player::One));
        let mut two = a.clone();
        two.set_viewer(Some(Player::Two));
        assert_ne!(one.render(), two.render());
    }

    #[test]
    fn random_and_ai_games() {
        check_game(Spoons::with_seed, 3, 3500);
    }
}

#[cfg(test)]
mod a1_blind {
    use super::*;
    use crate::ai_audit_a1::{audit, shuffle};

    #[test]
    fn ai_ignores_opponent_hand_stock_and_thrown_cards_at_all_levels() {
        audit("spoons", Spoons::with_seed, 1..=4, 2, 60, |g, salt| {
            let foe = g.side.other().idx();
            let n = g.hands[foe].len();
            let q = if g.dealer.idx() == foe {
                g.queue.len()
            } else {
                0
            };
            let mut pool: Vec<u8> = std::mem::take(&mut g.hands[foe]);
            pool.extend(g.stock.iter());
            if q > 0 {
                pool.extend(g.queue.iter());
            }
            shuffle(&mut pool, salt);
            g.hands[foe] = pool.drain(..n).collect();
            if q > 0 {
                g.queue = pool.drain(..q).collect();
            }
            g.stock = pool;
            g.rng = Rng::new(salt ^ 7);
        });
    }
}
