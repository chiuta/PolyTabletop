//! Septica (jocul românesc de cărți), 2 jucători, pachet de 32 de cărți (7..A).
//!
//! Reguli folosite (tradiționale, din regulile publice):
//! * Fiecare primește 4 cărți; restul rămâne în stoc.
//! * Cine conduce pune o carte pe masă. Adversarul poate **prelua** masa cu o
//!   carte de aceeași valoare ca prima carte de pe masă, sau cu un **7** ori un
//!   **8** (acestea preiau orice). Apoi primul poate **tăia** la rândul lui cu
//!   aceleași cărți, și tot așa. Cine nu poate / nu vrea să taie spune „pas”:
//!   ultimul care a pus o carte ia toată masa.
//! * Punctele sunt cărțile de 10 și A (1 punct fiecare, 8 în total). Câștigătorul
//!   mesei trage primul până la 4 cărți și conduce următoarea masă.
//! * La sfârșit (stoc și mâini goale) câștigă cine are peste 4 puncte; 4–4 = remiză.
//!
//! Informația ascunsă și AI-ul: vezi `cards.rs`.

//! Verificare surse (nov. 2026): en.wikipedia.org/wiki/Sedma (secțiunea Șeptică: 10 și A
//! valorează 1 punct, fără bonus pentru ultima levată, deci 4–4 e posibil), github.com/
//! claudiu-persoiu/septica și bzi.ro: cartea de aceeași valoare și 7 taie orice; punctele
//! (10 și A) = 8 în total corespund. DUBIU: majoritatea surselor dau doar 7-urile ca
//! cărți universale; 8-ul taie doar în variante (3 jucători, „Sedma osmová”). Codul
//! păstrează 8 ca joker (convenție locală), nemodificat din cauza surselor contradictorii.
//!
use crate::cards::{full_deck, handover_banner, join_cards, pl, shuffle, Card, Viewer};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Play(Card),
    Pass,
}

#[derive(Clone)]
pub struct Septica {
    deck: Vec<Card>, // se trage de la sfârșit
    hands: [Vec<Card>; 2],
    table: Vec<Card>,
    /// Cine a pus ultima carte pe masă (ia masa dacă adversarul dă pas).
    holder: Player,
    side: Player,
    pts: [u8; 2],
    captured: [u8; 2], // număr de cărți câștigate (pentru verificări)
    /// Câte cărți din coada mâinii au fost trase în ultima mutare (nu se folosesc la evaluare).
    fresh: [u8; 2],
    over: bool,
    viewer: Viewer,
    handover: bool,
}

impl Default for Septica {
    fn default() -> Self {
        Self::new()
    }
}

fn points_of(c: Card) -> u8 {
    u8::from(c.rank == 10 || c.rank == 14)
}

impl Septica {
    /// Joc nou, amestecat cu seed din ceas.
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck = full_deck(7);
        shuffle(&mut deck, &mut rng);
        let mut g = Septica {
            deck,
            hands: [Vec::new(), Vec::new()],
            table: Vec::new(),
            holder: Player::One,
            side: Player::One,
            pts: [0, 0],
            captured: [0, 0],
            fresh: [0, 0],
            over: false,
            viewer: Viewer::Fixed(Player::One),
            handover: false,
        };
        g.refill(Player::One);
        g.fresh = [0, 0];
        g
    }

    /// Alege al cui punct de vedere îl arată `render` (implicit Jucătorul 1).
    pub fn with_viewer(mut self, v: Viewer) -> Self {
        self.viewer = v;
        self
    }

    /// Două persoane la același ecran: se arată mâna celui la mutare.
    pub fn hot_seat(self) -> Self {
        self.with_viewer(Viewer::HotSeat)
    }

    pub fn points(&self) -> (u8, u8) {
        (self.pts[0], self.pts[1])
    }

    /// Completează mâinile la 4 cărți, pe rând, începând cu `first`.
    fn refill(&mut self, first: Player) {
        self.fresh = [0, 0];
        let mut p = first;
        loop {
            let need0 = self.hands[0].len() < 4;
            let need1 = self.hands[1].len() < 4;
            if self.deck.is_empty() || (!need0 && !need1) {
                break;
            }
            if self.hands[p.idx()].len() < 4 {
                if let Some(c) = self.deck.pop() {
                    self.hands[p.idx()].push(c);
                    self.fresh[p.idx()] += 1;
                }
            }
            p = p.other();
        }
    }

    fn lead_rank(&self) -> Option<u8> {
        self.table.first().map(|c| c.rank)
    }

    fn can_take(&self, c: Card) -> bool {
        match self.lead_rank() {
            None => true,
            Some(r) => c.rank == r || c.rank == 7 || c.rank == 8,
        }
    }
}

impl Game for Septica {
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
        if self.over {
            return Vec::new();
        }
        let hand = &self.hands[self.side.idx()];
        let mut v: Vec<Move> = Vec::new();
        for &c in hand {
            if self.can_take(c) && !v.contains(&Move::Play(c)) {
                v.push(Move::Play(c));
            }
        }
        if !self.table.is_empty() {
            v.push(Move::Pass);
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        self.fresh = [0, 0];
        self.handover = false;
        match *m {
            Move::Play(c) => {
                let h = &mut self.hands[self.side.idx()];
                let pos = h.iter().position(|&x| x == c).expect("card in hand");
                h.remove(pos);
                self.table.push(c);
                self.holder = self.side;
                self.side = self.side.other();
                self.handover = true;
            }
            Move::Pass => {
                let w = self.holder;
                let gained: u8 = self.table.iter().map(|&c| points_of(c)).sum();
                self.pts[w.idx()] += gained;
                self.captured[w.idx()] += self.table.len() as u8;
                self.table.clear();
                self.refill(w);
                if self.hands[0].is_empty() && self.hands[1].is_empty() {
                    self.over = true;
                } else {
                    self.side = if !self.hands[w.idx()].is_empty() {
                        w
                    } else {
                        w.other()
                    };
                }
                self.handover = true;
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            return None;
        }
        Some(match self.pts[0].cmp(&self.pts[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    /// Doar informație vizibilă lui `p`: punctele, masa, mâna lui `p` (fără cărțile
    /// abia trase în simulare) și numărul de cărți al adversarului.
    fn evaluate(&self, p: Player) -> i32 {
        let q = p.other();
        let mut s = 100 * (self.pts[p.idx()] as i32 - self.pts[q.idx()] as i32);
        let hand = &self.hands[p.idx()];
        let known = hand.len().saturating_sub(self.fresh[p.idx()] as usize);
        for c in &hand[..known] {
            s += match c.rank {
                7 => 9,
                8 => 7,
                10 | 14 => 5,
                _ => 0,
            };
        }
        let tp: i32 = self.table.iter().map(|&c| points_of(c) as i32).sum();
        let t = 4 * tp + 2;
        if self.holder == p {
            s += t;
        } else {
            s -= t;
        }
        s
    }

    fn render(&self) -> String {
        let mut s = String::new();
        if self.handover && !self.over {
            if let Viewer::HotSeat = self.viewer {
                s.push_str(&handover_banner(self.side));
            }
        }
        s.push_str(&format!(
            "Septica — stoc: {} cărți | puncte: J1 {}, J2 {}\n",
            self.deck.len(),
            self.pts[0],
            self.pts[1]
        ));
        s.push_str(&format!("Masa: {}\n", join_cards(&self.table)));
        if self.over {
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
        s.push_str(&format!("La mutare: Jucătorul {}\n", pl(self.side)));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Play(c) => c.text(),
            Move::Pass => "pas".into(),
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

    fn setup(h1: Vec<Card>, h2: Vec<Card>) -> Septica {
        let mut g = Septica::with_seed(1);
        g.hands = [h1, h2];
        g.deck.clear();
        g.table.clear();
        g.pts = [0, 0];
        g
    }

    #[test]
    fn deal_is_four_each_and_cards_conserved() {
        let g = Septica::with_seed(3);
        assert_eq!(g.hands[0].len(), 4);
        assert_eq!(g.hands[1].len(), 4);
        assert_eq!(g.deck.len(), 24);
        let mut all: Vec<Card> = g.deck.clone();
        all.extend(&g.hands[0]);
        all.extend(&g.hands[1]);
        all.sort_by_key(|c| (c.suit, c.rank));
        all.dedup();
        assert_eq!(all.len(), 32);
    }

    #[test]
    fn take_with_same_rank_seven_or_eight() {
        let mut g = setup(vec![c(9, 0)], vec![c(9, 1), c(7, 1), c(8, 2), c(12, 3)]);
        g.apply(&Move::Play(c(9, 0)));
        let m = g.legal_moves();
        assert!(m.contains(&Move::Play(c(9, 1))));
        assert!(m.contains(&Move::Play(c(7, 1))));
        assert!(m.contains(&Move::Play(c(8, 2))));
        assert!(!m.contains(&Move::Play(c(12, 3))));
        assert!(m.contains(&Move::Pass));
    }

    #[test]
    fn leader_has_no_pass_and_can_cut_back() {
        let mut g = setup(vec![c(10, 0), c(10, 1)], vec![c(7, 0)]);
        assert!(!g.legal_moves().contains(&Move::Pass));
        g.apply(&Move::Play(c(10, 0)));
        g.apply(&Move::Play(c(7, 0)));
        // leader cuts again with the same rank as the first card
        assert!(g.legal_moves().contains(&Move::Play(c(10, 1))));
    }

    #[test]
    fn pass_gives_table_to_holder_with_points() {
        let mut g = setup(vec![c(10, 0), c(14, 1)], vec![c(10, 1), c(9, 2)]);
        g.apply(&Move::Play(c(10, 0))); // J1 leads a ten
        g.apply(&Move::Play(c(10, 1))); // J2 takes
        g.apply(&Move::Pass); // J1 gives up: J2 holds the table
        assert_eq!(g.points(), (0, 2));
        assert_eq!(g.to_move(), Player::Two); // the winner leads
        assert!(g.table.is_empty());
    }

    #[test]
    fn winner_draws_first_and_hands_refill() {
        let mut g = setup(vec![c(9, 0), c(9, 1)], vec![c(9, 2)]);
        g.deck = vec![c(13, 3), c(12, 3), c(11, 3)]; // pops: J, Q, K
        g.apply(&Move::Play(c(9, 0)));
        g.apply(&Move::Pass); // J2 passes: J1 wins the table
                              // stoc: se trage J, Q, K pe rând, câștigătorul (J1) primul
        assert_eq!(g.hands[0].len(), 1 + 2);
        assert_eq!(g.hands[1].len(), 1 + 1);
        assert_eq!(g.hands[0].last().copied(), Some(c(13, 3)));
        assert_eq!(g.fresh, [2, 1]);
    }

    #[test]
    fn game_ends_with_points_and_draw() {
        let mut g = setup(vec![c(10, 0)], vec![]);
        g.apply(&Move::Play(c(10, 0)));
        g.apply(&Move::Pass);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
        let mut g = setup(vec![c(9, 0)], vec![c(9, 1)]);
        g.pts = [4, 4];
        g.apply(&Move::Play(c(9, 0)));
        g.apply(&Move::Play(c(9, 1)));
        g.apply(&Move::Pass);
        assert_eq!(g.outcome(), Some(Outcome::Draw));
    }

    #[test]
    fn empty_handed_player_is_forced_to_pass() {
        let mut g = setup(vec![c(9, 0)], vec![]);
        g.apply(&Move::Play(c(9, 0)));
        assert_eq!(g.legal_moves(), vec![Move::Pass]);
    }

    #[test]
    fn render_hides_opponent_hand() {
        let g = Septica::with_seed(5);
        let r = Game::render(&g);
        for c in &g.hands[1] {
            assert!(!r.split_whitespace().any(|t| t == c.text()), "{r}");
        }
        for c in &g.hands[0] {
            assert!(r.split_whitespace().any(|t| t == c.text()), "{r}");
        }
        // even when the AI (Player Two) is to move, the fixed viewer sees only its own hand
        let mut g2 = g.clone();
        let m = Game::legal_moves(&g2)[0];
        g2.apply(&m);
        let r2 = Game::render(&g2);
        for c in &g2.hands[1] {
            assert!(!r2.split_whitespace().any(|t| t == c.text()), "{r2}");
        }
    }

    #[test]
    fn hot_seat_shows_mover_only_with_banner() {
        let mut g = Septica::with_seed(5).hot_seat();
        let m = Game::legal_moves(&g)[0];
        g.apply(&m);
        let r = Game::render(&g);
        assert!(r.contains("Predă ecranul"));
        for c in &g.hands[0] {
            assert!(!r.split_whitespace().any(|t| t == c.text()), "{r}");
        }
        for c in &g.hands[1] {
            assert!(r.split_whitespace().any(|t| t == c.text()), "{r}");
        }
    }

    #[test]
    fn set_viewer_switches_hand_shown() {
        // CLI `--you two` / `--you both` use Game::set_viewer.
        let mut g = Septica::with_seed(5);
        Game::set_viewer(&mut g, Some(Player::Two));
        let r = Game::render(&g);
        for c in &g.hands[1] {
            assert!(r.split_whitespace().any(|t| t == c.text()), "{r}");
        }
        for c in &g.hands[0] {
            assert!(!r.split_whitespace().any(|t| t == c.text()), "{r}");
        }
        Game::set_viewer(&mut g, None);
        let m = Game::legal_moves(&g)[0];
        g.apply(&m);
        assert!(Game::render(&g).contains("Predă ecranul"));
    }

    #[test]
    fn ai_is_blind_to_hidden_information() {
        for seed in 1..40u64 {
            let mut g = Septica::with_seed(seed);
            let mut rng = Rng::new(seed);
            for _ in 0..(seed as usize % 25) {
                if g.outcome().is_some() {
                    break;
                }
                let mv = Game::legal_moves(&g);
                g.apply(&mv[rng.below(mv.len())]);
            }
            if g.outcome().is_some() {
                continue;
            }
            // scramble everything the side to move cannot see
            let me = g.side.idx();
            let mut h = g.clone();
            let mut pool: Vec<Card> = h.deck.clone();
            pool.extend(&h.hands[1 - me]);
            shuffle(&mut pool, &mut rng);
            let n = h.hands[1 - me].len();
            h.hands[1 - me] = pool.drain(..n).collect();
            h.deck = pool;
            for p in [Player::One, Player::Two] {
                if p.idx() == me {
                    assert_eq!(Game::evaluate(&g, p), Game::evaluate(&h, p));
                }
            }
            assert_eq!(
                polytabletop_core::ai::best_move(&g, 7),
                polytabletop_core::ai::best_move(&h, 7),
                "AI decision depends on hidden cards (seed {seed})"
            );
        }
    }

    #[test]
    fn random_games_finish_and_total_8_points() {
        for seed in 1..60u64 {
            let mut g = Septica::with_seed(seed);
            let mut rng = Rng::new(seed ^ 0xABCD);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = Game::legal_moves(&g);
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
                assert!(n < 500);
            }
            assert_eq!(g.pts[0] + g.pts[1], 8);
            assert_eq!(g.captured[0] + g.captured[1], 32);
        }
    }
}
