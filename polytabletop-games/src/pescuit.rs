//! Pescuit (jocul „Go Fish”), 2 jucători, pachet de 52 de cărți.
//!
//! * 7 cărți fiecăruia, restul în stoc. Cine e la rând cere o valoare pe care o are în mână
//!   (`cer 7`). Adversarul trebuie să răspundă printr-o mutare forțată: `da` (dă toate cărțile
//!   acelei valori) sau `pescuieste` (nu are: cel care a cerut trage o carte din stoc).
//! * După ce primești cărți, sau pescuiești exact valoarea cerută, mai ceri o dată; altfel
//!   rândul trece. Patru cărți de aceeași valoare formează un „set” și se pun jos.
//! * Mână goală: tragi o carte din stoc (dacă mai e). Jocul se termină când toate cele 13 seturi
//!   sunt jos; câștigă cine are mai multe seturi.
//! * Răspunsul e o mutare separată ca AI-ul să nu poată vedea mâna adversarului prin simulare:
//!   AI-ul cere o valoare după cărțile lui și după ce a cerut adversarul (informație publică).
//!
//! Reguli: Go Fish, joc tradițional (domeniu public).

use crate::cards::{full_deck, handover_banner, join_cards, pl, shuffle, Card, Viewer};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Ask(u8),
    Give,
    GoFish,
}

fn label(rank: u8) -> &'static str {
    Card { rank, suit: 0 }.rank_label()
}

#[derive(Clone)]
pub struct Pescuit {
    stock: Vec<Card>,
    hands: [Vec<Card>; 2],
    books: [u8; 2],
    side: Player,
    /// `Some((cine a cerut, valoare))`: răspunsul se așteaptă de la `side`.
    pending: Option<(Player, u8)>,
    /// Valorile pe care adversarul le-a cerut și (după cunoștințele publice) le mai are.
    asked: [[bool; 15]; 2],
    log: String,
    viewer: Viewer,
    handover: bool,
}

impl Default for Pescuit {
    fn default() -> Self {
        Self::new()
    }
}

impl Pescuit {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut stock = full_deck(2);
        shuffle(&mut stock, &mut rng);
        let mut g = Pescuit {
            stock,
            hands: [Vec::new(), Vec::new()],
            books: [0, 0],
            side: Player::One,
            pending: None,
            asked: [[false; 15]; 2],
            log: String::new(),
            viewer: Viewer::Fixed(Player::One),
            handover: false,
        };
        for _ in 0..7 {
            for p in 0..2 {
                let c = g.stock.pop().unwrap();
                g.hands[p].push(c);
            }
        }
        for p in 0..2 {
            g.lay_books(p);
        }
        g.settle();
        g
    }

    pub fn books(&self) -> (u8, u8) {
        (self.books[0], self.books[1])
    }

    fn lay_books(&mut self, p: usize) {
        for r in 2..=14u8 {
            if self.hands[p].iter().filter(|c| c.rank == r).count() == 4 {
                self.hands[p].retain(|c| c.rank != r);
                self.books[p] += 1;
                self.asked[1 - p][r as usize] = false;
                self.asked[p][r as usize] = false;
            }
        }
    }

    fn over(&self) -> bool {
        self.books[0] + self.books[1] == 13
    }

    /// Mână goală la rând: trage din stoc, altfel trece rândul.
    fn settle(&mut self) {
        for _ in 0..2 {
            if self.over() {
                return;
            }
            let s = self.side.idx();
            if self.hands[s].is_empty() {
                if let Some(c) = self.stock.pop() {
                    self.hands[s].push(c);
                } else {
                    self.side = self.side.other();
                    continue;
                }
            }
            return;
        }
    }
}

impl Game for Pescuit {
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
        if self.over() {
            return Vec::new();
        }
        if let Some((_, r)) = self.pending {
            return if self.hands[self.side.idx()].iter().any(|c| c.rank == r) {
                vec![Move::Give]
            } else {
                vec![Move::GoFish]
            };
        }
        let mut ranks: Vec<u8> = self.hands[self.side.idx()].iter().map(|c| c.rank).collect();
        ranks.sort_unstable();
        ranks.dedup();
        ranks.into_iter().map(Move::Ask).collect()
    }

    fn apply(&mut self, m: &Move) {
        self.handover = false;
        match *m {
            Move::Ask(r) => {
                let a = self.side;
                self.pending = Some((a, r));
                self.asked[a.idx()][r as usize] = true;
                self.log = format!("J{} cere: ai {}?", pl(a), label(r));
                self.side = a.other();
                self.handover = true;
            }
            Move::Give => {
                let (a, r) = self.pending.take().expect("pending");
                let b = a.other();
                let (ai, bi) = (a.idx(), b.idx());
                let mut got = 0;
                let mut keep = Vec::new();
                for c in std::mem::take(&mut self.hands[bi]) {
                    if c.rank == r {
                        self.hands[ai].push(c);
                        got += 1;
                    } else {
                        keep.push(c);
                    }
                }
                self.hands[bi] = keep;
                self.asked[ai][r as usize] = false;
                self.lay_books(ai);
                self.log = format!("J{} dă {} cărți de {}.", pl(b), got, label(r));
                self.side = a;
                self.handover = true;
                self.settle();
            }
            Move::GoFish => {
                let (a, r) = self.pending.take().expect("pending");
                let ai = a.idx();
                self.asked[ai][r as usize] = false;
                self.asked[1 - ai][r as usize] = false;
                self.log = format!("J{}: pescuiește!", pl(a));
                if let Some(c) = self.stock.pop() {
                    self.hands[ai].push(c);
                    self.lay_books(ai);
                    if c.rank == r {
                        self.log =
                            format!("J{} pescuiește exact {}: mai cere o dată.", pl(a), label(r));
                        self.side = a;
                    } else {
                        self.side = a.other();
                    }
                } else {
                    self.side = a.other();
                }
                self.handover = true;
                self.settle();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over() {
            return None;
        }
        Some(match self.books[0].cmp(&self.books[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            _ => Outcome::Win(Player::Two),
        })
    }

    /// Doar seturile, mâna lui `p` și ce a cerut adversarul (public).
    fn evaluate(&self, p: Player) -> i32 {
        let q = p.other();
        let mut s = 100 * (self.books[p.idx()] as i32 - self.books[q.idx()] as i32);
        if let Some((a, r)) = self.pending {
            if a == p {
                let mine = self.hands[p.idx()].iter().filter(|c| c.rank == r).count() as i32;
                s += 3 * mine;
                if self.asked[q.idx()][r as usize] {
                    s += 12;
                }
            }
        }
        s
    }

    fn render(&self) -> String {
        let mut s = String::new();
        if self.handover && !self.over() {
            if let Viewer::HotSeat = self.viewer {
                s.push_str(&handover_banner(self.side));
            }
        }
        s.push_str(&format!(
            "Pescuit — stoc: {} cărți | seturi: J1 {}, J2 {}\n",
            self.stock.len(),
            self.books[0],
            self.books[1]
        ));
        if !self.log.is_empty() {
            s.push_str(&self.log);
            s.push('\n');
        }
        if self.over() {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        let v = self.viewer.who(self.side);
        s.push_str(&format!(
            "Mâna Jucătorului {}: {}\n",
            pl(v),
            join_cards(&self.hands[v.idx()])
        ));
        s.push_str(&format!(
            "Jucătorul {}: {} cărți\n",
            pl(v.other()),
            self.hands[v.other().idx()].len()
        ));
        if self.pending.is_some() {
            s.push_str("Răspunde (mutare forțată).\n");
        }
        s.push_str(&format!("La mutare: Jucătorul {}\n", pl(self.side)));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Ask(r) => format!("cer {}", label(*r)),
            Move::Give => "da".into(),
            Move::GoFish => "pescuieste".into(),
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

    fn setup(h1: Vec<Card>, h2: Vec<Card>, stock: Vec<Card>) -> Pescuit {
        let mut g = Pescuit::with_seed(1);
        g.hands = [h1, h2];
        g.stock = stock;
        g.books = [0, 0];
        g.side = Player::One;
        g.pending = None;
        g.asked = [[false; 15]; 2];
        g
    }

    #[test]
    fn deal_conserves_52_cards() {
        for seed in 1..20 {
            let g = Pescuit::with_seed(seed);
            let n = g.stock.len()
                + g.hands[0].len()
                + g.hands[1].len()
                + 4 * (g.books[0] + g.books[1]) as usize;
            assert_eq!(n, 52);
        }
    }

    #[test]
    fn ask_give_and_book() {
        let mut g = setup(
            vec![c(5, 0), c(5, 1), c(9, 0)],
            vec![c(5, 2), c(5, 3), c(2, 0)],
            vec![c(3, 0)],
        );
        assert_eq!(g.legal_moves(), vec![Move::Ask(5), Move::Ask(9)]);
        g.apply(&Move::Ask(5));
        assert_eq!(g.to_move(), Player::Two);
        assert_eq!(g.legal_moves(), vec![Move::Give]);
        g.apply(&Move::Give);
        assert_eq!(g.books(), (1, 0));
        assert_eq!(g.to_move(), Player::One); // asks again
        assert_eq!(g.hands[0], vec![c(9, 0)]);
    }

    #[test]
    fn go_fish_turn_passes_unless_lucky() {
        let mut g = setup(
            vec![c(5, 0), c(9, 0)],
            vec![c(2, 0)],
            vec![c(8, 0), c(5, 3)],
        );
        g.apply(&Move::Ask(9)); // J2 has no 9
        assert_eq!(g.legal_moves(), vec![Move::GoFish]);
        g.apply(&Move::GoFish); // draws 5d: not 9
        assert_eq!(g.to_move(), Player::Two);
        let mut g2 = setup(
            vec![c(5, 0), c(9, 0)],
            vec![c(2, 0)],
            vec![c(9, 3), c(5, 3)],
        );
        g2.apply(&Move::Ask(5));
        g2.apply(&Move::GoFish); // draws 5d: exactly the asked rank
        assert_eq!(g2.to_move(), Player::One);
    }

    #[test]
    fn empty_hand_draws_and_game_ends() {
        let mut g = setup(
            vec![c(5, 0)],
            vec![c(5, 1), c(5, 2), c(5, 3)],
            vec![c(2, 0)],
        );
        g.apply(&Move::Ask(5));
        g.apply(&Move::Give);
        assert_eq!(g.books(), (1, 0));
        assert_eq!(g.hands[0], vec![c(2, 0)]); // empty hand drew from stock
    }

    #[test]
    fn render_hides_opponent_hand() {
        let g = Pescuit::with_seed(3);
        let r = Game::render(&g);
        for x in &g.hands[1] {
            assert!(!r.split_whitespace().any(|t| t == x.text()), "{r}");
        }
        let mut h = g.clone();
        Game::set_viewer(&mut h, None);
        let m = h.legal_moves()[0];
        h.apply(&m);
        let r = Game::render(&h);
        for x in &h.hands[0] {
            assert!(!r.split_whitespace().any(|t| t == x.text()), "{r}");
        }
    }

    #[test]
    fn ai_ignores_hidden_cards() {
        for seed in 1..40u64 {
            let mut g = Pescuit::with_seed(seed);
            let mut rng = Rng::new(seed);
            for _ in 0..(seed as usize % 15) {
                if g.outcome().is_some() {
                    break;
                }
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
            }
            if g.outcome().is_some() || g.pending.is_some() {
                continue;
            }
            let me = g.side.idx();
            let mut h = g.clone();
            let mut pool = h.stock.clone();
            pool.extend(&h.hands[1 - me]);
            shuffle(&mut pool, &mut rng);
            let n = h.hands[1 - me].len();
            h.hands[1 - me] = pool.drain(..n).collect();
            h.stock = pool;
            assert_eq!(
                polytabletop_core::ai::best_move(&g, 3),
                polytabletop_core::ai::best_move(&h, 3),
                "seed {seed}"
            );
        }
    }

    #[test]
    fn random_games_finish_with_13_books() {
        for seed in 1..60u64 {
            let mut g = Pescuit::with_seed(seed);
            let mut rng = Rng::new(seed ^ 9);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                assert!(!mv.is_empty(), "seed {seed}");
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
                assert!(n < 3000, "seed {seed}");
            }
            assert_eq!(g.books[0] + g.books[1], 13);
        }
    }
}
