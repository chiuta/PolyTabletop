#![allow(
    clippy::needless_range_loop,
    clippy::manual_is_multiple_of,
    clippy::manual_range_contains,
    clippy::too_many_arguments,
    clippy::doc_lazy_continuation,
    clippy::unnecessary_unwrap,
    clippy::unnecessary_cast,
    clippy::type_complexity
)]
//! Court Piece / Rang / Mendikot (India) — joc de levate cu zecile („mendi”), pentru 2 jucători.
//!
//! Reguli folosite (din regulile publice ale jocului în patru, adaptate pentru doi):
//! * 52 de cărți, rang A (cel mai mare) … 2, toate se împart: câte 26 fiecăruia. Se dau mai
//!   întâi 5 cărți fiecăruia; cine nu e dealer (alternativ la fiecare mână) le vede și **alege
//!   atuul** (`rang h`), apoi se împart restul de cărți și el conduce prima levată.
//! * Câștigătorul unei levate conduce următoarea. Răspunzi obligatoriu la culoarea condusă dacă
//!   o ai; altfel poți juca orice, iar atuul bate tot.
//! * Se numără zecile (mendi, 4 în total) din levatele luate. Cine are mai multe zeci câștigă
//!   mâna: 1 punct; dacă ia toate cele 4 zeci (Mendikot) 2 puncte; dacă ia toate cele 26 de
//!   levate (Court) 3 puncte. La 2–2 zeci decide numărul de levate; la egalitate perfectă
//!   (13–13) mâna se anulează și se împarte din nou. Primul la 5 puncte câștigă.
//! * Mutări: `rang h` (atu: c/d/h/s) și cărțile (`10h`, `As`).
//! * Informație ascunsă: vezi doar mâna ta (la alegerea atuului vezi doar primele 5 cărți).
//!   Cărțile adversarului se deduc din cărțile jucate; AI-ul nu le citește (Hard/Expert:
//!   eșantionare Monte-Carlo).

use crate::cards::Viewer;
use crate::cards52::{
    std_deck, std_rank, std_suit, std_symbol, std_text, SUIT_LETTERS, SUIT_SYMBOLS,
};
use crate::p2_util::{
    beats2, follow_cards, mc_samples, mc_value, noise, shuffle_vec, sig_of, sort_std,
    symbols_typed, Mc,
};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const TARGET: u8 = 5;
const TEN: u8 = 8; // rank0 al zecii

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Trump(u8),
    Play(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Trump,
    Play,
    Over,
}

#[derive(Clone)]
pub struct CourtPiece {
    hands: [Vec<u8>; 2],
    five: Vec<u8>,
    played: Vec<u8>,
    trump: Option<u8>,
    dealer: Player,
    side: Player,
    trick: Vec<(Player, u8)>,
    tricks: [u8; 2],
    tens: [u8; 2],
    points: [u8; 2],
    hand_no: u8,
    phase: Phase,
    last: String,
    rng: Rng,
    viewer: Viewer,
}

impl Default for CourtPiece {
    fn default() -> Self {
        Self::new()
    }
}

impl CourtPiece {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let dealer = if rng.below(2) == 0 {
            Player::One
        } else {
            Player::Two
        };
        let mut g = CourtPiece {
            hands: [Vec::new(), Vec::new()],
            five: Vec::new(),
            played: Vec::new(),
            trump: None,
            dealer,
            side: dealer.other(),
            trick: Vec::new(),
            tricks: [0; 2],
            tens: [0; 2],
            points: [0; 2],
            hand_no: 1,
            phase: Phase::Trump,
            last: String::new(),
            rng,
            viewer: Viewer::Fixed(Player::One),
        };
        g.deal();
        g
    }

    /// Cine alege atuul și conduce: nu dealerul.
    pub fn chooser(&self) -> Player {
        self.dealer.other()
    }

    fn deal(&mut self) {
        let mut d = std_deck(&mut self.rng);
        shuffle_vec(&mut d, &mut self.rng);
        let ch = self.chooser().idx();
        self.hands = [Vec::new(), Vec::new()];
        self.five = (0..5).map(|_| d.pop().unwrap()).collect();
        self.hands[ch] = self.five.clone();
        // restul: 21 pentru alegător, 26 pentru celălalt
        for _ in 0..21 {
            self.hands[ch].push(d.pop().unwrap());
        }
        while let Some(c) = d.pop() {
            self.hands[1 - ch].push(c);
        }
        sort_std(&mut self.hands[0]);
        sort_std(&mut self.hands[1]);
        sort_std(&mut self.five);
        self.played.clear();
        self.trump = None;
        self.trick.clear();
        self.tricks = [0; 2];
        self.tens = [0; 2];
        self.phase = Phase::Trump;
        self.side = self.chooser();
    }

    pub fn points(&self) -> (u8, u8) {
        (self.points[0], self.points[1])
    }
    pub fn set_hands_for_test(&mut self, a: Vec<u8>, b: Vec<u8>) {
        self.hands = [a, b];
    }

    /// Cărțile pe care le cunoaște `p` (alegătorul vede doar primele 5 până joacă prima carte;
    /// celălalt nu vede nimic până se alege atuul).
    fn known(&self, p: Player) -> Vec<u8> {
        if self.phase == Phase::Trump {
            return if p == self.chooser() {
                self.five.clone()
            } else {
                Vec::new()
            };
        }
        if p == self.chooser() && self.played.is_empty() && self.trick.is_empty() {
            return self.five.clone();
        }
        self.hands[p.idx()].clone()
    }

    fn end_hand(&mut self) {
        let t = self.tens;
        let tr = self.tricks;
        let w = match t[0].cmp(&t[1]) {
            std::cmp::Ordering::Greater => Some(Player::One),
            std::cmp::Ordering::Less => Some(Player::Two),
            std::cmp::Ordering::Equal => match tr[0].cmp(&tr[1]) {
                std::cmp::Ordering::Greater => Some(Player::One),
                std::cmp::Ordering::Less => Some(Player::Two),
                std::cmp::Ordering::Equal => None,
            },
        };
        match w {
            None => {
                self.last = format!("Mâna {} anulată (zeci și levate egale)", self.hand_no);
            }
            Some(w) => {
                let pts = if tr[w.idx()] == 26 {
                    3
                } else if t[w.idx()] == 4 {
                    2
                } else {
                    1
                };
                self.points[w.idx()] += pts;
                self.last = format!(
                    "Mâna {}: J{} câștigă ({} zeci, {} levate) → {} p.{}",
                    self.hand_no,
                    w.idx() + 1,
                    t[w.idx()],
                    tr[w.idx()],
                    pts,
                    match pts {
                        3 => " Court!",
                        2 => " Mendikot!",
                        _ => "",
                    }
                );
                if self.points[w.idx()] >= TARGET {
                    self.phase = Phase::Over;
                    return;
                }
            }
        }
        self.hand_no += 1;
        self.dealer = self.dealer.other();
        self.deal();
    }

    fn hand_value(&self, h: &[u8], trump: Option<u8>) -> i32 {
        let mut s = 0;
        for &c in h {
            let r = std_rank(c) as i32;
            s += r / 2;
            if Some(std_suit(c)) == trump {
                s += 5 + r;
            } else if r >= 11 {
                s += 4;
            }
            if std_rank(c) == TEN {
                s += 6;
            }
        }
        s
    }
}

impl Mc for CourtPiece {
    fn determinize(&self, viewer: Player, rng: &mut Rng) -> Self {
        let mut g = self.clone();
        let (me, op) = (viewer.idx(), viewer.other().idx());
        let known = self.known(viewer);
        let mut pool: Vec<u8> = (0..52u8)
            .filter(|c| !known.contains(c) && !self.played.contains(c))
            .collect();
        shuffle_vec(&mut pool, rng);
        let mut mine = known.clone();
        let need_me = self.hands[me].len() - mine.len();
        mine.extend(pool.drain(..need_me));
        let n = self.hands[op].len().min(pool.len());
        let theirs: Vec<u8> = pool.drain(..n).collect();
        g.hands[me] = mine;
        g.hands[op] = theirs;
        sort_std(&mut g.hands[0]);
        sort_std(&mut g.hands[1]);
        g.rng = Rng::new(rng.next_u64());
        g
    }
    fn epoch(&self) -> u32 {
        self.hand_no as u32
    }
    fn margin(&self, p: Player) -> i32 {
        10 * (self.points[p.idx()] as i32 - self.points[p.other().idx()] as i32)
    }
}

impl Game for CourtPiece {
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
            Phase::Trump => (0..4u8).map(Move::Trump).collect(),
            Phase::Play => {
                let led = self.trick.first().map(|&(_, c)| c);
                follow_cards(&self.hands[self.side.idx()], led)
                    .into_iter()
                    .map(Move::Play)
                    .collect()
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        match *m {
            Move::Trump(s) => {
                self.trump = Some(s);
                self.phase = Phase::Play;
                self.side = self.chooser();
                self.last = format!("Atu: {}", SUIT_SYMBOLS[s as usize]);
            }
            Move::Play(c) => {
                let p = self.side;
                let h = &mut self.hands[p.idx()];
                let pos = h.iter().position(|&x| x == c).expect("carte în mână");
                h.remove(pos);
                self.played.push(c);
                self.trick.push((p, c));
                if self.trick.len() < 2 {
                    self.side = p.other();
                    return;
                }
                let (lp, lc) = self.trick[0];
                let (rp, rc) = self.trick[1];
                let w = if beats2(lc, rc, self.trump) { rp } else { lp };
                self.tricks[w.idx()] += 1;
                self.tens[w.idx()] +=
                    [lc, rc].iter().filter(|&&x| std_rank(x) == TEN).count() as u8;
                self.trick.clear();
                self.side = w;
                if self.hands[0].is_empty() {
                    self.end_hand();
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        Some(if self.points[0] > self.points[1] {
            Outcome::Win(Player::One)
        } else {
            Outcome::Win(Player::Two)
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let (me, op) = (p.idx(), p.other().idx());
        let mut s = 1000 * (self.points[me] as i32 - self.points[op] as i32);
        if self.phase == Phase::Over {
            return s;
        }
        let known = self.known(p);
        let sig = sig_of(&[
            known
                .iter()
                .fold(0u64, |a, &c| a.wrapping_mul(53).wrapping_add(c as u64 + 1)),
            self.played.len() as u64,
            self.points[0] as u64,
            self.trump.map(|t| t as u64 + 1).unwrap_or(0),
        ]);
        if self.phase == Phase::Trump {
            return s + noise(sig, 60);
        }
        s += 120 * (self.tens[me] as i32 - self.tens[op] as i32);
        s += 15 * (self.tricks[me] as i32 - self.tricks[op] as i32);
        s += self.hand_value(&known, self.trump);
        if let (Some(t), Some(&(w, c))) = (self.trump, self.trick.first()) {
            let mut st = std_rank(c) as i32 * 2 + if std_suit(c) == t { 30 } else { 0 };
            if std_rank(c) == TEN {
                st += 40;
            }
            s += if w == p { st } else { -st };
        }
        s += noise(sig, 60);
        s + mc_value(self, p, mc_samples(), sig)
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Court Piece — mâna {} | puncte: J1 {}, J2 {} (până la {}) | dealer: J{}",
            self.hand_no,
            self.points[0],
            self.points[1],
            TARGET,
            self.dealer.idx() + 1
        );
        if let Some(t) = self.trump {
            s.push_str(&format!(" | atu {}", SUIT_SYMBOLS[t as usize]));
        }
        s.push('\n');
        if !self.last.is_empty() {
            s.push_str(&format!("({})\n", self.last));
        }
        if self.phase == Phase::Over {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        s.push_str(&format!(
            "Levate: J1 {}, J2 {} | zeci luate: J1 {}, J2 {}\n",
            self.tricks[0], self.tricks[1], self.tens[0], self.tens[1]
        ));
        if let Some(&(w, c)) = self.trick.first() {
            s.push_str(&format!("Pe masă: {} (J{})\n", std_symbol(c), w.idx() + 1));
        }
        let v = self.viewer.who(self.side);
        if self.phase == Phase::Trump && v == self.chooser() {
            s.push_str(&format!(
                "Mâna ta (J{}, primele 5 cărți): {}\n",
                v.idx() + 1,
                symbols_typed(&self.five)
            ));
        } else if self.phase == Phase::Trump {
            s.push_str(&format!(
                "Mâna ta (J{}): se împarte după alegerea atuului\n",
                v.idx() + 1
            ));
        } else {
            s.push_str(&format!(
                "Mâna ta (J{}): {}\n",
                v.idx() + 1,
                symbols_typed(&self.hands[v.idx()])
            ));
        }
        s.push_str(&format!(
            "Jucătorul {}: {} cărți\n",
            v.other().idx() + 1,
            self.hands[v.other().idx()].len()
        ));
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Trump(s) => format!("rang {}", SUIT_LETTERS[*s as usize]),
            Move::Play(c) => std_text(*c),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(rank: u8, suit: char) -> u8 {
        let s = SUIT_LETTERS.iter().position(|&x| x == suit).unwrap() as u8;
        s * 13 + (rank - 2)
    }

    #[test]
    fn full_deal_and_hidden_views() {
        let g = CourtPiece::with_seed(1);
        assert_eq!(g.hands[0].len() + g.hands[1].len(), 52);
        let ch = g.chooser();
        let mut v = g.clone();
        v.set_viewer(Some(ch));
        let r = Game::render(&v);
        assert!(r.contains("primele 5 cărți"), "{r}");
        for &card in &g.hands[ch.idx()] {
            if !g.five.contains(&card) {
                assert!(!r.contains(&format!("[{}]", std_text(card))), "{r}");
            }
        }
        let mut w = g.clone();
        w.set_viewer(Some(ch.other()));
        let r = Game::render(&w);
        assert!(r.contains("se împarte"), "{r}");
        for &card in &g.hands[ch.other().idx()] {
            assert!(!r.contains(&format!("[{}]", std_text(card))), "{r}");
        }
    }

    #[test]
    fn tens_are_counted_and_follow_suit() {
        let mut g = CourtPiece::with_seed(2);
        g.apply(&Move::Trump(1));
        g.set_hands_for_test(
            vec![c(10, 'h'), c(3, 'c'), c(9, 'c')],
            vec![c(12, 'h'), c(5, 'd'), c(2, 'c')],
        );
        g.side = Player::One;
        g.apply(&Move::Play(c(10, 'h')));
        assert_eq!(Game::legal_moves(&g), vec![Move::Play(c(12, 'h'))]);
        g.apply(&Move::Play(c(12, 'h')));
        assert_eq!(g.tens, [0, 1]); // Dama ia zecea
        assert_eq!(g.to_move(), Player::Two);
        assert_eq!(g.tricks, [0, 1]);
    }

    #[test]
    fn scoring_tens_mendikot_court_and_void() {
        let mut g = CourtPiece::with_seed(3);
        g.tens = [3, 1];
        g.tricks = [10, 16];
        g.end_hand();
        assert_eq!(g.points(), (1, 0));
        let mut g = CourtPiece::with_seed(3);
        g.tens = [0, 4];
        g.tricks = [10, 16];
        g.end_hand();
        assert_eq!(g.points(), (0, 2));
        let mut g = CourtPiece::with_seed(3);
        g.tens = [4, 0];
        g.tricks = [26, 0];
        g.end_hand();
        assert_eq!(g.points(), (3, 0));
        let mut g = CourtPiece::with_seed(3);
        g.tens = [2, 2];
        g.tricks = [12, 14];
        g.end_hand();
        assert_eq!(g.points(), (0, 1));
        let mut g = CourtPiece::with_seed(3);
        g.tens = [2, 2];
        g.tricks = [13, 13];
        g.end_hand();
        assert_eq!(g.points(), (0, 0));
        assert_eq!(g.hand_no, 2);
    }

    #[test]
    fn determinize_hides_and_conserves() {
        let g = CourtPiece::with_seed(4);
        let ch = g.chooser();
        let mut rng = Rng::new(3);
        let d = g.determinize(ch, &mut rng);
        for card in &g.five {
            assert!(d.hands[ch.idx()].contains(card));
        }
        let mut all: Vec<u8> = d.hands[0].iter().chain(&d.hands[1]).copied().collect();
        all.sort();
        all.dedup();
        assert_eq!(all.len(), 52);
        let mut r2 = Rng::new(99);
        let b = g.determinize(ch, &mut r2);
        assert_eq!(
            polytabletop_core::ai::best_move(&g, 1),
            polytabletop_core::ai::best_move(&b, 1)
        );
    }

    #[test]
    fn ai_finishes_match() {
        let mut g = CourtPiece::with_seed(5);
        let mut n = 0;
        while g.outcome().is_none() && n < 20000 {
            let m = polytabletop_core::ai::best_move(&g, n).unwrap();
            g.apply(&m);
            n += 1;
        }
        assert!(g.outcome().is_some());
    }
}
