//! Teen Patti (India) — poker indian cu 3 cărți, heads-up, 12 mâini.
//!
//! Reguli folosite (variantă simplificată, documentată):
//! * Pachet de 52, 3 cărți fiecare. Ierarhia (de la cea mai mare): trio (trail), scară de
//!   culoare (pure sequence), scară (sequence), culoare (color), pereche, carte mare.
//!   La scări, A-K-Q e cea mai mare, apoi A-2-3, apoi K-Q-J … 4-3-2. Culorile nu
//!   departajează; la egalitate perfectă pierde cel care a cerut `show`.
//! * Fiecare jucător pornește cu 500 de jetoane; la fiecare mână se plătește „boot” = 10,
//!   iar miza (chaal) pornește de la 10. Toți joacă „văzut” (fără joc „orb”).
//! * La mutare: `pack` (abandon), `chaal` (plătești miza curentă), `raise` (dublezi miza și
//!   plătești noua miză; cel mult 80), `show` (plătești miza și se comparăm mâinile; după cel puțin
//!   o acțiune). Cel mult 6 acțiuni pe mână: după aceea doar `pack` sau `show`. Un raise
//!   e permis doar dacă amândoi pot plăti de până la 3 ori noua miză.
//! * Mâna se încheie cu pack sau show. `arata` și `urmatoarea` sunt mutări forțate,
//!   ca AI-ul să nu vadă cărțile adversarului sau pachetul. Meciul: 12 mâini sau până cineva
//!   rămâne cu mai puțin de 40 de jetoane; câștigă cine are mai multe jetoane.
//! * AI: AlphaBeta(1) peste o valoare așteptată calculată doar din propria mână (percentila
//!   ei între toate cele 22 100 de mâini) și din pariuri; nu bluffează.

use crate::cards::Viewer;
use crate::cards52::{shuffle, std_deck, std_rank, std_suit, std_symbol, std_text};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::sync::OnceLock;

pub const HANDS: u8 = 12;
pub const START_CHIPS: i32 = 500;
pub const BOOT: i32 = 10;
const MAX_STAKE: i32 = 80;
const MAX_ACTS: u8 = 6;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Pack,
    Chaal,
    Raise,
    Show,
    Reveal,
    Next,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Bet,
    Showdown,
    HandOver,
}

/// Cheia unei mâini de 3 cărți (mai mare = mai bună).
pub fn hand_key(cards: &[u8; 3]) -> u32 {
    let mut r: Vec<u32> = cards.iter().map(|&c| std_rank(c) as u32).collect();
    r.sort_unstable_by(|a, b| b.cmp(a));
    let flush = cards.iter().all(|&c| std_suit(c) == std_suit(cards[0]));
    let seq_key = if r[0] == 12 && r[1] == 1 && r[2] == 0 {
        Some(99u32) // A-2-3
    } else if r[0] == r[1] + 1 && r[1] == r[2] + 1 {
        Some(if r[0] == 12 { 100 } else { r[0] })
    } else {
        None
    };
    if r[0] == r[1] && r[1] == r[2] {
        (6 << 20) | r[0]
    } else if let (true, Some(k)) = (flush, seq_key) {
        (5 << 20) | k
    } else if let Some(k) = seq_key {
        (4 << 20) | k
    } else if flush {
        (3 << 20) | (r[0] << 8) | (r[1] << 4) | r[2]
    } else if r[0] == r[1] || r[1] == r[2] {
        let (p, k) = if r[0] == r[1] {
            (r[0], r[2])
        } else {
            (r[1], r[0])
        };
        (2 << 20) | (p << 4) | k
    } else {
        (1 << 20) | (r[0] << 8) | (r[1] << 4) | r[2]
    }
}

pub fn category_name(key: u32) -> &'static str {
    match key >> 20 {
        6 => "trio",
        5 => "scară de culoare",
        4 => "scară",
        3 => "culoare",
        2 => "pereche",
        _ => "carte mare",
    }
}

fn table() -> &'static Vec<u32> {
    static T: OnceLock<Vec<u32>> = OnceLock::new();
    T.get_or_init(|| {
        let mut v = Vec::with_capacity(22100);
        for a in 0..52u8 {
            for b in (a + 1)..52 {
                for c in (b + 1)..52 {
                    v.push(hand_key(&[a, b, c]));
                }
            }
        }
        v.sort_unstable();
        v
    })
}

/// Fracția mâinilor pe care o bate (egalitățile cu jumătate).
pub fn percentile(key: u32) -> f64 {
    let t = table();
    let lo = t.partition_point(|&x| x < key);
    let hi = t.partition_point(|&x| x <= key);
    (lo as f64 + (hi - lo) as f64 / 2.0) / t.len() as f64
}

#[derive(Clone)]
pub struct TeenPatti {
    deck: Vec<u8>,
    hands: [[u8; 3]; 2],
    chips: [i32; 2],
    pot: i32,
    stake: i32,
    acts: u8,
    raises: [u8; 2],
    side: Player,
    hand_no: u8,
    phase: Phase,
    caller: Player,
    done: bool,
    last: String,
    rng: Rng,
    viewer: Viewer,
}

impl Default for TeenPatti {
    fn default() -> Self {
        Self::new()
    }
}

impl TeenPatti {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = TeenPatti {
            deck: Vec::new(),
            hands: [[0; 3]; 2],
            chips: [START_CHIPS; 2],
            pot: 0,
            stake: BOOT,
            acts: 0,
            raises: [0; 2],
            side: Player::One,
            hand_no: 0,
            phase: Phase::Bet,
            caller: Player::One,
            done: false,
            last: String::new(),
            rng: Rng::new(seed),
            viewer: Viewer::Fixed(Player::One),
        };
        g.deal();
        g
    }

    fn deal(&mut self) {
        let mut d = std_deck(&mut self.rng);
        shuffle(&mut d, &mut self.rng);
        for p in 0..2 {
            for i in 0..3 {
                self.hands[p][i] = d.pop().unwrap();
            }
        }
        self.deck = d;
        self.chips[0] -= BOOT;
        self.chips[1] -= BOOT;
        self.pot = 2 * BOOT;
        self.stake = BOOT;
        self.acts = 0;
        self.raises = [0; 2];
        self.side = if self.hand_no.is_multiple_of(2) {
            Player::One
        } else {
            Player::Two
        };
        self.phase = Phase::Bet;
    }

    fn pay(&mut self, p: Player, amt: i32) {
        self.chips[p.idx()] -= amt;
        self.pot += amt;
    }

    fn finish_hand(&mut self, winner: Player, why: &str) {
        self.chips[winner.idx()] += self.pot;
        self.last = format!(
            "Mâna {}: câștigă J{} ({}), pot {}",
            self.hand_no + 1,
            winner.idx() + 1,
            why,
            self.pot
        );
        self.pot = 0;
        self.phase = Phase::HandOver;
        if self.hand_no + 1 >= HANDS || self.chips.iter().any(|&c| c < 4 * BOOT) {
            self.done = true;
        }
    }

    pub fn chips(&self) -> (i32, i32) {
        (self.chips[0], self.chips[1])
    }
}

impl Game for TeenPatti {
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
        if self.done {
            return Vec::new();
        }
        match self.phase {
            Phase::Showdown => vec![Move::Reveal],
            Phase::HandOver => vec![Move::Next],
            Phase::Bet => {
                let mut v = vec![Move::Pack];
                if self.acts < MAX_ACTS {
                    v.push(Move::Chaal);
                    let ns = self.stake * 2;
                    let minc = self.chips[0].min(self.chips[1]);
                    if ns <= MAX_STAKE && 3 * ns <= minc {
                        v.push(Move::Raise);
                    }
                }
                if self.acts >= 1 {
                    v.push(Move::Show);
                }
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        let me = self.side;
        match *m {
            Move::Pack => self.finish_hand(me.other(), "abandon"),
            Move::Chaal => {
                self.pay(me, self.stake);
                self.acts += 1;
                self.side = me.other();
            }
            Move::Raise => {
                self.stake *= 2;
                self.pay(me, self.stake);
                self.acts += 1;
                self.raises[me.idx()] += 1;
                self.side = me.other();
            }
            Move::Show => {
                self.pay(me, self.stake);
                self.caller = me;
                self.phase = Phase::Showdown;
            }
            Move::Reveal => {
                let k0 = hand_key(&self.hands[0]);
                let k1 = hand_key(&self.hands[1]);
                let (w, why) = if k0 == k1 {
                    (self.caller.other(), "egalitate, pierde cel care cere show")
                } else if k0 > k1 {
                    (Player::One, category_name(k0))
                } else {
                    (Player::Two, category_name(k1))
                };
                self.finish_hand(w, why);
                self.last.push_str(&format!(
                    " [J1: {} ({}) / J2: {} ({})]",
                    fmt3(&self.hands[0]),
                    category_name(k0),
                    fmt3(&self.hands[1]),
                    category_name(k1)
                ));
            }
            Move::Next => {
                self.hand_no += 1;
                self.deal();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.done || self.phase != Phase::HandOver {
            return None;
        }
        Some(match self.chips[0].cmp(&self.chips[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    /// Valoare așteptată (în jetoane) din punctul de vedere al lui `p`: folosește doar
    /// mâna proprie, potul public și numărul de raise-uri ale adversarului.
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let o = 1 - me;
        let base = (self.chips[me] - self.chips[o]) as f64;
        if self.phase == Phase::HandOver || self.pot == 0 {
            return base as i32;
        }
        let pct = percentile(hand_key(&self.hands[me]));
        let q = pct.powf(1.0 + 0.35 * self.raises[o] as f64);
        // dacă se compară acum, câștig q din pot; chips-urile sunt deja fără pot
        let mut v = base + (2.0 * q - 1.0) * self.pot as f64;
        // a continua (nu a cere show) păstrează opțiunea; a ridica cu mână bună extrage valoare
        if self.phase == Phase::Bet && self.raises[me] > 0 && q > 0.6 {
            v += (q - 0.6) * self.stake as f64;
        }
        v as i32
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Teen Patti — mâna {}/{} | jetoane: J1 {}, J2 {} | pot {} | miza {}\n",
            (self.hand_no + 1).min(HANDS),
            HANDS,
            self.chips[0],
            self.chips[1],
            self.pot,
            self.stake
        );
        if !self.last.is_empty() {
            s.push_str(&format!("{}\n", self.last));
        }
        if self.done && self.phase == Phase::HandOver {
            s.push_str("Meci încheiat.\n");
            return s;
        }
        if self.phase != Phase::HandOver {
            let v = self.viewer.who(self.side);
            s.push_str(&format!(
                "Mâna ta (J{}): {}\n",
                v.idx() + 1,
                fmt3(&self.hands[v.idx()])
            ));
            s.push_str(&format!("J{}: [?? ?? ??]\n", v.other().idx() + 1));
            s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Pack => "pack",
            Move::Chaal => "chaal",
            Move::Raise => "raise",
            Move::Show => "show",
            Move::Reveal => "arata",
            Move::Next => "urmatoarea",
        }
        .into()
    }

    /// Adâncime 0 (o singură mutare): `AlphaBeta(1)` ar fi extins la Hard/Expert mutarea forțată `arata`
    /// (cărțile reale ale adversarului) și răspunsurile lui; evaluarea citește doar mâna proprie și pariurile.
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

fn fmt3(h: &[u8; 3]) -> String {
    let mut v = h.to_vec();
    v.sort_by_key(|&c| (std::cmp::Reverse(std_rank(c)), std_suit(c)));
    v.iter()
        .map(|&c| format!("{}[{}]", std_symbol(c), std_text(c)))
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(rank: u8, suit: u8) -> u8 {
        suit * 13 + (rank - 2)
    }
    fn key(a: (u8, u8), b: (u8, u8), d: (u8, u8)) -> u32 {
        hand_key(&[c(a.0, a.1), c(b.0, b.1), c(d.0, d.1)])
    }

    #[test]
    fn category_order() {
        let trail = key((5, 0), (5, 1), (5, 2));
        let pure = key((9, 0), (10, 0), (11, 0));
        let seq = key((9, 0), (10, 1), (11, 0));
        let color = key((2, 0), (9, 0), (13, 0));
        let pair = key((14, 0), (14, 1), (3, 2));
        let high = key((14, 0), (13, 1), (11, 2));
        assert!(trail > pure && pure > seq && seq > color && color > pair && pair > high);
        assert_eq!(category_name(trail), "trio");
        assert_eq!(category_name(color), "culoare");
    }

    #[test]
    fn sequence_order_aqk_a23() {
        let akq = key((14, 0), (13, 1), (12, 2));
        let a23 = key((14, 0), (2, 1), (3, 2));
        let kqj = key((13, 0), (12, 1), (11, 2));
        let s432 = key((4, 0), (3, 1), (2, 2));
        assert!(akq > a23 && a23 > kqj && kqj > s432);
        // A-K-2 nu e scară
        assert_eq!(key((14, 0), (13, 1), (2, 2)) >> 20, 1);
    }

    #[test]
    fn percentile_distribution() {
        assert_eq!(table().len(), 22100);
        let trail = percentile(key((2, 0), (2, 1), (2, 2)));
        assert!(trail > 0.99);
        let worst = percentile(key((5, 0), (3, 1), (2, 2)));
        assert!(worst < 0.01);
        // 52 de trio-uri în total
        let n = table().iter().filter(|&&k| k >> 20 == 6).count();
        assert_eq!(n, 52);
        assert_eq!(table().iter().filter(|&&k| k >> 20 == 5).count(), 48);
        assert_eq!(table().iter().filter(|&&k| k >> 20 == 4).count(), 720);
    }

    #[test]
    fn pack_awards_pot_and_chips_conserved() {
        let mut g = TeenPatti::with_seed(1);
        assert_eq!(g.pot, 20);
        g.apply(&Move::Chaal);
        g.apply(&Move::Pack);
        assert_eq!(g.chips().0 + g.chips().1, 1000);
        assert_eq!(g.legal_moves(), vec![Move::Next]);
        g.apply(&Move::Next);
        assert_eq!(g.hand_no, 1);
        assert_eq!(g.side, Player::Two);
        assert_eq!(g.chips().0 + g.chips().1 + g.pot, 1000);
    }

    #[test]
    fn show_not_first_and_forced_reveal() {
        let mut g = TeenPatti::with_seed(2);
        assert!(!g.legal_moves().contains(&Move::Show));
        g.apply(&Move::Chaal);
        assert!(g.legal_moves().contains(&Move::Show));
        g.apply(&Move::Show);
        assert_eq!(g.legal_moves(), vec![Move::Reveal]);
        g.apply(&Move::Reveal);
        assert_eq!(g.chips().0 + g.chips().1, 1000);
        assert_eq!(g.legal_moves(), vec![Move::Next]);
    }

    #[test]
    fn tie_loses_for_caller() {
        let mut g = TeenPatti::with_seed(3);
        g.hands[0] = [c(9, 0), c(10, 1), c(12, 2)];
        g.hands[1] = [c(9, 1), c(10, 2), c(12, 3)];
        g.apply(&Move::Chaal);
        g.apply(&Move::Show); // J2 cere
        g.apply(&Move::Reveal);
        assert!(g.chips().0 > g.chips().1);
    }

    #[test]
    fn raise_doubles_stake_with_cap() {
        let mut g = TeenPatti::with_seed(4);
        g.apply(&Move::Raise);
        assert_eq!(g.stake, 20);
        g.apply(&Move::Raise);
        assert_eq!(g.stake, 40);
        g.apply(&Move::Raise);
        assert_eq!(g.stake, 80);
        assert!(!g.legal_moves().contains(&Move::Raise));
    }

    #[test]
    fn render_hides_opponent_and_ai_blind() {
        let g = TeenPatti::with_seed(5);
        let r = Game::render(&g);
        assert!(r.contains("J2: [?? ?? ??]"));
        for &x in &g.hands[1] {
            assert!(!r.contains(&format!("[{}]", std_text(x))));
        }
        for seed in 1..30u64 {
            let mut a = TeenPatti::with_seed(seed);
            let mut rng = Rng::new(seed);
            for _ in 0..(seed % 4) {
                let mv = Game::legal_moves(&a);
                if a.outcome().is_some() {
                    break;
                }
                a.apply(&mv[rng.below(mv.len())]);
            }
            if a.phase != Phase::Bet {
                continue;
            }
            let me = a.side.idx();
            let mut b = a.clone();
            // schimbă mâna adversarului și pachetul
            let mut pool: Vec<u8> = b.deck.clone();
            pool.extend(b.hands[1 - me]);
            shuffle(&mut pool, &mut rng);
            for i in 0..3 {
                b.hands[1 - me][i] = pool.pop().unwrap();
            }
            b.deck = pool;
            assert_eq!(
                polytabletop_core::ai::best_move(&a, 5),
                polytabletop_core::ai::best_move(&b, 5)
            );
        }
    }

    #[test]
    fn random_matches_end_with_conserved_chips() {
        for seed in 1..40u64 {
            let mut g = TeenPatti::with_seed(seed);
            let mut rng = Rng::new(seed ^ 9);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
                assert!(n < 3000);
                assert_eq!(g.chips[0] + g.chips[1] + g.pot, 1000);
            }
        }
    }
}

#[cfg(test)]
mod a2_audit {
    use super::*;

    #[test]
    fn ai_nu_vede_cartile_adversarului_nici_pachetul_la_orice_nivel() {
        crate::p3_comun::ai_blind_check(
            "teenpatti",
            TeenPatti::with_seed,
            |g, me, seed| {
                let o = me.other().idx();
                let mut pool: Vec<u8> = g.deck.clone();
                pool.extend(g.hands[o].iter().copied());
                crate::p3_comun::shuffle_with(&mut pool, seed);
                for i in 0..3 {
                    g.hands[o][i] = pool.pop().unwrap();
                }
                g.deck = pool;
                g.rng = Rng::new(seed ^ 0x7E);
            },
            12,
        );
    }
}
