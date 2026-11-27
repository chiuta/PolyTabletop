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
//! Hokm (Iran) — joc de levate cu atu ales de „hakem”, variantă pentru 2 jucători.
//!
//! Reguli folosite (din regulile publice ale jocului în patru, adaptate pentru doi):
//! * 52 de cărți, rang A (cel mai mare) … 2. Hakem (conducătorul) primește mai întâi 5 cărți,
//!   le vede și **alege atuul** (hokm); apoi fiecare are 13 cărți. Celelalte 26 rămân nefolosite
//!   și ascunse (în jocul în patru s-ar împărți toate). Primul hakem e tras la sorți.
//! * Hakem conduce prima levată; câștigătorul unei levate conduce următoarea. Răspunzi
//!   obligatoriu la culoarea condusă dacă o ai; altfel poți juca orice, iar atuul bate tot.
//! * O mână se câștigă cu cel puțin 7 din cele 13 levate: 1 punct; 13–0 („kot”) = 2 puncte;
//!   dacă hakem nu ia nicio levată (hakem-kot) adversarul primește 3. Dacă hakem câștigă, rămâne
//!   hakem; altfel funcția trece la celălalt. Primul la 7 puncte câștigă meciul.
//! * Mutări: `hokm h` (atu: c/d/h/s) și cărțile (`10h`, `As`).
//! * Informație ascunsă: vezi doar mâna ta (la alegerea atuului hakem vede doar primele 5 cărți).
//!   AI-ul alege atuul doar din cele 5 cărți și joacă după mâna proprie, levatele publice și
//!   (Hard/Expert) eșantionare Monte-Carlo a cărților necunoscute.

use crate::cards::Viewer;
use crate::cards52::{std_deck, std_rank, std_suit, std_text, SUIT_LETTERS, SUIT_SYMBOLS};
use crate::p2_util::{
    beats2, follow_cards, mc_samples, mc_value, noise, shuffle_vec, sig_of, sort_std,
    symbols_typed, Mc,
};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const TARGET: u8 = 7;

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
pub struct Hokm {
    hands: [Vec<u8>; 2],
    five: Vec<u8>,
    played: Vec<u8>,
    trump: Option<u8>,
    hakem: Player,
    side: Player,
    trick: Vec<(Player, u8)>,
    tricks: [u8; 2],
    points: [u8; 2],
    hand_no: u8,
    phase: Phase,
    last: String,
    rng: Rng,
    viewer: Viewer,
}

impl Default for Hokm {
    fn default() -> Self {
        Self::new()
    }
}

impl Hokm {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let hakem = if rng.below(2) == 0 {
            Player::One
        } else {
            Player::Two
        };
        let mut g = Hokm {
            hands: [Vec::new(), Vec::new()],
            five: Vec::new(),
            played: Vec::new(),
            trump: None,
            hakem,
            side: hakem,
            trick: Vec::new(),
            tricks: [0; 2],
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

    fn deal(&mut self) {
        let mut d = std_deck(&mut self.rng);
        shuffle_vec(&mut d, &mut self.rng);
        let h = self.hakem.idx();
        let o = 1 - h;
        self.hands = [Vec::new(), Vec::new()];
        self.five = (0..5).map(|_| d.pop().unwrap()).collect();
        self.hands[h] = self.five.clone();
        for _ in 0..8 {
            self.hands[h].push(d.pop().unwrap());
        }
        for _ in 0..13 {
            self.hands[o].push(d.pop().unwrap());
        }
        sort_std(&mut self.hands[0]);
        sort_std(&mut self.hands[1]);
        sort_std(&mut self.five);
        self.played.clear();
        self.trump = None;
        self.trick.clear();
        self.tricks = [0; 2];
        self.phase = Phase::Trump;
        self.side = self.hakem;
    }

    pub fn points(&self) -> (u8, u8) {
        (self.points[0], self.points[1])
    }
    pub fn trump(&self) -> Option<u8> {
        self.trump
    }
    pub fn hakem(&self) -> Player {
        self.hakem
    }
    pub fn set_hands_for_test(&mut self, a: Vec<u8>, b: Vec<u8>) {
        self.hands = [a, b];
    }

    /// Mai are `p` doar cele 5 cărți inițiale (alegerea atuului)?
    fn sees_five_only(&self, p: Player) -> bool {
        p == self.hakem
            && self.played.is_empty()
            && self.trick.is_empty()
            && self.phase != Phase::Over
    }

    /// Cărțile pe care le cunoaște `p`.
    fn known(&self, p: Player) -> Vec<u8> {
        if self.sees_five_only(p) {
            self.five.clone()
        } else {
            self.hands[p.idx()].clone()
        }
    }

    fn end_hand(&mut self) {
        let h = self.hakem;
        let w = if self.tricks[0] >= TARGET {
            Player::One
        } else {
            Player::Two
        };
        let l = w.other();
        let pts = if self.tricks[l.idx()] == 0 {
            if l == h {
                3
            } else {
                2
            }
        } else {
            1
        };
        self.points[w.idx()] += pts;
        self.last = format!(
            "Mâna {}: J{} ia {} levate și {} p. (hakem: J{})",
            self.hand_no,
            w.idx() + 1,
            self.tricks[w.idx()],
            pts,
            h.idx() + 1
        );
        if self.points[w.idx()] >= TARGET {
            self.phase = Phase::Over;
            return;
        }
        if w != h {
            self.hakem = w;
        }
        self.hand_no += 1;
        self.deal();
    }

    fn hand_value(&self, h: &[u8], trump: Option<u8>) -> i32 {
        let mut s = 0;
        for &c in h {
            let r = std_rank(c) as i32;
            s += r / 2;
            if Some(std_suit(c)) == trump {
                s += 6 + r;
            } else if r >= 11 {
                s += 4;
            }
        }
        s
    }
}

impl Mc for Hokm {
    fn determinize(&self, viewer: Player, rng: &mut Rng) -> Self {
        let mut g = self.clone();
        let (me, op) = (viewer.idx(), viewer.other().idx());
        let known = self.known(viewer);
        let mut pool: Vec<u8> = (0..52u8)
            .filter(|c| !known.contains(c) && !self.played.contains(c))
            .collect();
        // cărțile jucate în levata curentă sunt deja în `played`
        shuffle_vec(&mut pool, rng);
        let mut mine = known.clone();
        let need_me = self.hands[me].len() - mine.len();
        mine.extend(pool.drain(..need_me));
        let n = self.hands[op].len();
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

impl Game for Hokm {
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
                self.side = self.hakem;
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
        if self.phase == Phase::Trump && p == self.hakem {
            return s + noise(sig, 60);
        }
        s += 100 * (self.tricks[me] as i32 - self.tricks[op] as i32);
        s += self.hand_value(&known, self.trump);
        if let (Some(t), Some(&(w, c))) = (self.trump, self.trick.first()) {
            let st = std_rank(c) as i32 * 2 + if std_suit(c) == t { 30 } else { 0 };
            s += if w == p { st } else { -st };
        }
        s += noise(sig, 60);
        s + mc_value(self, p, mc_samples(), sig)
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Hokm — mâna {} | puncte: J1 {}, J2 {} (până la {}) | hakem: J{}",
            self.hand_no,
            self.points[0],
            self.points[1],
            TARGET,
            self.hakem.idx() + 1
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
            "Levate: J1 {}, J2 {}\n",
            self.tricks[0], self.tricks[1]
        ));
        if let Some(&(w, c)) = self.trick.first() {
            s.push_str(&format!(
                "Pe masă: {} (J{})\n",
                crate::cards52::std_symbol(c),
                w.idx() + 1
            ));
        }
        let v = self.viewer.who(self.side);
        if self.phase == Phase::Trump && v == self.hakem {
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
            if self.phase == Phase::Trump {
                13
            } else {
                self.hands[v.other().idx()].len()
            }
        ));
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Trump(s) => format!("hokm {}", SUIT_LETTERS[*s as usize]),
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
    fn hakem_sees_five_then_full_hand() {
        let g = Hokm::with_seed(1);
        let hk = g.hakem();
        let r = Game::render(&{
            let mut x = g.clone();
            x.set_viewer(Some(hk));
            x
        });
        assert!(r.contains("primele 5 cărți"), "{r}");
        for &card in &g.hands[hk.idx()] {
            if !g.five.contains(&card) {
                assert!(!r.contains(&format!("[{}]", std_text(card))), "{r}");
            }
        }
        let mut g2 = g.clone();
        g2.apply(&Move::Trump(2));
        let mut v = g2.clone();
        v.set_viewer(Some(hk));
        let r2 = Game::render(&v);
        for &card in &g2.hands[hk.idx()] {
            assert!(r2.contains(&format!("[{}]", std_text(card))), "{r2}");
        }
        // celălalt jucător nu-și vede mâna înainte de atu
        let other = hk.other();
        let mut w = g.clone();
        w.set_viewer(Some(other));
        assert!(Game::render(&w).contains("se împarte"));
    }

    #[test]
    fn follow_suit_and_trump_wins() {
        let mut g = Hokm::with_seed(2);
        g.hakem = Player::One;
        g.apply(&Move::Trump(1)); // atu = ♦
        g.set_hands_for_test(vec![c(10, 'h'), c(3, 'c')], vec![c(2, 'h'), c(5, 'd')]);
        g.side = Player::One;
        g.apply(&Move::Play(c(10, 'h')));
        let mv = Game::legal_moves(&g);
        assert_eq!(mv, vec![Move::Play(c(2, 'h'))]); // obligat să răspundă la culoare
        g.apply(&Move::Play(c(2, 'h')));
        assert_eq!(g.tricks, [1, 0]);
        // J1 conduce ♣3, J2 nu are ♣ → taie cu atu ♦5
        g.apply(&Move::Play(c(3, 'c')));
        assert_eq!(Game::legal_moves(&g), vec![Move::Play(c(5, 'd'))]);
        g.apply(&Move::Play(c(5, 'd')));
        assert_eq!(g.points(), (0, 1)); // 1–1 la levate: J2 (non-hakem... cu < 7 pentru J1)
    }

    #[test]
    fn scoring_normal_kot_and_hakem_kot() {
        let mut g = Hokm::with_seed(3);
        g.hakem = Player::One;
        g.tricks = [7, 6];
        g.end_hand();
        assert_eq!(g.points(), (1, 0));
        assert_eq!(g.hakem(), Player::One);
        let mut g = Hokm::with_seed(3);
        g.hakem = Player::One;
        g.tricks = [13, 0];
        g.end_hand();
        assert_eq!(g.points(), (2, 0));
        let mut g = Hokm::with_seed(3);
        g.hakem = Player::One;
        g.tricks = [0, 13];
        g.end_hand();
        assert_eq!(g.points(), (0, 3));
        assert_eq!(g.hakem(), Player::Two); // funcția trece
    }

    #[test]
    fn determinize_keeps_known_and_ai_does_not_peek_unseen_eight() {
        let g = Hokm::with_seed(4);
        let hk = g.hakem();
        let mut rng = Rng::new(3);
        let d = g.determinize(hk, &mut rng);
        for card in &g.five {
            assert!(d.hands[hk.idx()].contains(card));
        }
        assert_eq!(d.hands[hk.idx()].len(), 13);
        assert_eq!(d.hands[hk.other().idx()].len(), 13);
        let mut all: Vec<u8> = d.hands[0].iter().chain(&d.hands[1]).copied().collect();
        all.sort();
        all.dedup();
        assert_eq!(all.len(), 26);
        // alegerea atuului nu depinde de cele 8 cărți nevăzute
        let mut r2 = Rng::new(77);
        let b = g.determinize(hk, &mut r2);
        let ma = polytabletop_core::ai::best_move(&g, 1);
        let mb = polytabletop_core::ai::best_move(&b, 1);
        assert_eq!(ma, mb);
    }

    #[test]
    fn ai_finishes_match() {
        for seed in 1..3u64 {
            let mut g = Hokm::with_seed(seed);
            let mut n = 0;
            while g.outcome().is_none() && n < 5000 {
                let m = polytabletop_core::ai::best_move(&g, n).unwrap();
                g.apply(&m);
                n += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed}");
        }
    }
}

#[cfg(test)]
mod a2_audit {
    use super::*;

    #[test]
    fn ai_ignores_hidden_information_at_every_level() {
        crate::p3_comun::ai_blind_check_with(
            "hokm",
            Hokm::with_seed,
            |g, me, seed| {
                let mut rng = Rng::new(seed);
                let mut d = crate::p2_util::Mc::determinize(g, me, &mut rng);
                if g.phase != Phase::Trump {
                    // după alegerea atuului hakem-ul își vede toată mâna: doar mâna adversarului se amestecă
                    let own = g.hands[me.idx()].clone();
                    let n = g.hands[me.other().idx()].len();
                    let mut opp: Vec<u8> = (0..52u8)
                        .filter(|c| !own.contains(c) && !g.played.contains(c))
                        .collect();
                    crate::p3_comun::shuffle_with(&mut opp, seed);
                    opp.truncate(n);
                    sort_std(&mut opp);
                    d.hands[me.idx()] = own;
                    d.hands[me.other().idx()] = opp;
                }
                *g = d;
            },
            2,
            4,
            crate::p3_comun::Term::Check,
        );
    }
}
