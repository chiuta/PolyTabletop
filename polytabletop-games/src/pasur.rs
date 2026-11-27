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
//! Pasur (Iran) — joc de capturi cu sume de 11, 2 jucători.
//!
//! Reguli folosite (varianta de bază din regulile publice):
//! * 52 de cărți. Pe masă se pun 4 cărți cu fața în sus (fără valeți; dacă apare un valet se
//!   reamestecă); fiecare primește 4 cărți, iar când mâinile se golesc se dau încă 4 (6 runde
//!   de câte 8 cărți). Cine nu e dealer începe; dealerul alternează între împărțiri.
//! * La mutare joci o carte. **Cărți de la A la 10** (A = 1): capturează orice grupuri de cărți
//!   de pe masă (A–10) a căror sumă împreună cu cartea ta e 11 (poți lua mai multe grupuri
//!   deodată, fiecare cu suma 11 − cartea ta). **Valetul (J)** capturează toate cărțile de pe
//!   masă în afară de Dame și Regi. **Dama** capturează o Damă, **Regele** un Rege de pe masă.
//!   Cartea jucată merge și ea în grămada ta. Dacă nu poți / nu vrei să capturezi, cartea
//!   rămâne pe masă (în această variantă capturarea nu e obligatorie).
//! * „Sur”: dacă o captură (nu cu valet) golește masa, primești 5 puncte la sfârșit (nu și la
//!   ultima carte a împărțirii). La sfârșitul împărțirii cărțile rămase pe masă merg la ultimul
//!   care a capturat.
//! * Puncte pe împărțire: cele mai multe cărți 3 (la egalitate nimeni); cel puțin 7 trefle 7;
//!   10♦ 3; 2♣ 2; fiecare As 1; fiecare Valet 1; fiecare sur 5. Se joacă împărțiri până când
//!   cineva ajunge la 62 de puncte (mai mare câștigă; la egalitate se mai joacă o împărțire).
//! * Mutări: `7h` (doar o pui pe masă) sau `7h x 4c` / `5s x 2c 4d 6h` (capturezi cărțile
//!   listate); `imparte` după ce se golesc mâinile.
//! * Informație ascunsă: vezi doar mâna ta; masa și grămezile capturate sunt publice. AI-ul
//!   folosește doar mâna proprie, masa, grămezile și (Hard/Expert) eșantionare Monte-Carlo.

use crate::cards::Viewer;
use crate::cards52::{std_deck, std_rank, std_suit, std_symbol, std_text};
use crate::p2_util::{
    mc_samples, mc_value, noise, shuffle_vec, sig_of, sort_std, symbols_typed, Mc,
};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const TARGET: i32 = 62;

const R_J: u8 = 9;
const R_Q: u8 = 10;
const R_K: u8 = 11;

/// Valoarea numerică (A = 1, 2..10) sau `None` pentru J/Q/K.
pub fn num_value(c: u8) -> Option<u8> {
    match std_rank(c) {
        12 => Some(1),
        r if r <= 8 => Some(r + 2),
        _ => None,
    }
}

/// Toate mulțimile de cărți de pe masă pe care le poate captura `card` (nevide).
pub fn captures(card: u8, table: &[u8]) -> Vec<Vec<u8>> {
    let r = std_rank(card);
    match r {
        R_J => {
            let all: Vec<u8> = table
                .iter()
                .copied()
                .filter(|&c| std_rank(c) != R_Q && std_rank(c) != R_K)
                .collect();
            if all.is_empty() {
                vec![]
            } else {
                vec![all]
            }
        }
        R_Q | R_K => table
            .iter()
            .copied()
            .filter(|&c| std_rank(c) == r)
            .map(|c| vec![c])
            .collect(),
        _ => {
            let target = 11 - num_value(card).unwrap() as i32;
            let nums: Vec<u8> = table
                .iter()
                .copied()
                .filter(|&c| num_value(c).is_some())
                .collect();
            let n = nums.len().min(16);
            // grupuri cu suma exactă
            let mut groups: Vec<u32> = Vec::new();
            for mask in 1u32..(1 << n) {
                let s: i32 = (0..n)
                    .filter(|&i| mask >> i & 1 == 1)
                    .map(|i| num_value(nums[i]).unwrap() as i32)
                    .sum();
                if s == target {
                    groups.push(mask);
                }
            }
            // uniuni de grupuri disjuncte
            let mut sets: Vec<u32> = Vec::new();
            fn rec(groups: &[u32], from: usize, cur: u32, sets: &mut Vec<u32>) {
                for i in from..groups.len() {
                    if cur & groups[i] == 0 {
                        let nm = cur | groups[i];
                        if !sets.contains(&nm) {
                            sets.push(nm);
                        }
                        rec(groups, i + 1, nm, sets);
                    }
                }
            }
            if groups.len() <= 40 {
                rec(&groups, 0, 0, &mut sets);
            } else {
                sets = groups;
            }
            sets.sort_unstable();
            sets.into_iter()
                .map(|m| {
                    (0..n)
                        .filter(|&i| m >> i & 1 == 1)
                        .map(|i| nums[i])
                        .collect()
                })
                .collect()
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Play(u8, Vec<u8>),
    Deal,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Play,
    Deal,
    Over,
}

#[derive(Clone)]
pub struct Pasur {
    stock: Vec<u8>,
    table: Vec<u8>,
    hands: [Vec<u8>; 2],
    cap: [Vec<u8>; 2],
    sur: [u8; 2],
    score: [i32; 2],
    last_cap: Option<Player>,
    dealer: Player,
    side: Player,
    deal_no: u8,
    phase: Phase,
    last: String,
    rng: Rng,
    viewer: Viewer,
}

impl Default for Pasur {
    fn default() -> Self {
        Self::new()
    }
}

/// Puncte de grămadă (fără majoritatea cărților/trefle): valoarea cărților luate.
fn card_points(c: u8) -> i32 {
    match (std_rank(c), std_suit(c)) {
        (12, _) => 1,
        (R_J, _) => 1,
        (0, 0) => 2, // 2♣
        (8, 1) => 3, // 10♦
        _ => 0,
    }
}

impl Pasur {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Pasur {
            stock: Vec::new(),
            table: Vec::new(),
            hands: [Vec::new(), Vec::new()],
            cap: [Vec::new(), Vec::new()],
            sur: [0; 2],
            score: [0; 2],
            last_cap: None,
            dealer: Player::Two,
            side: Player::One,
            deal_no: 1,
            phase: Phase::Play,
            last: String::new(),
            rng: Rng::new(seed),
            viewer: Viewer::Fixed(Player::One),
        };
        g.new_deal();
        g
    }

    fn new_deal(&mut self) {
        loop {
            let mut d = std_deck(&mut self.rng);
            shuffle_vec(&mut d, &mut self.rng);
            let table: Vec<u8> = (0..4).map(|_| d.pop().unwrap()).collect();
            if table.iter().any(|&c| std_rank(c) == R_J) {
                continue;
            }
            self.table = table;
            self.stock = d;
            break;
        }
        self.cap = [Vec::new(), Vec::new()];
        self.sur = [0; 2];
        self.last_cap = None;
        self.side = self.dealer.other();
        self.deal_hands();
        self.phase = Phase::Play;
    }

    fn deal_hands(&mut self) {
        for p in 0..2 {
            for _ in 0..4 {
                if let Some(c) = self.stock.pop() {
                    self.hands[p].push(c);
                }
            }
            sort_std(&mut self.hands[p]);
        }
    }

    pub fn score(&self) -> (i32, i32) {
        (self.score[0], self.score[1])
    }
    pub fn set_state_for_test(&mut self, table: Vec<u8>, a: Vec<u8>, b: Vec<u8>) {
        self.table = table;
        self.hands = [a, b];
    }

    /// Punctele unei grămezi în raport cu cealaltă, la sfârșitul împărțirii.
    pub fn deal_points(&self) -> [i32; 2] {
        let mut pts = [0i32; 2];
        for p in 0..2 {
            pts[p] += self.cap[p].iter().map(|&c| card_points(c)).sum::<i32>();
            pts[p] += 5 * self.sur[p] as i32;
            let clubs = self.cap[p].iter().filter(|&&c| std_suit(c) == 0).count();
            if clubs >= 7 {
                pts[p] += 7;
            }
        }
        match self.cap[0].len().cmp(&self.cap[1].len()) {
            std::cmp::Ordering::Greater => pts[0] += 3,
            std::cmp::Ordering::Less => pts[1] += 3,
            std::cmp::Ordering::Equal => {}
        }
        pts
    }

    fn finish_deal(&mut self) {
        if let Some(p) = self.last_cap {
            let rest = std::mem::take(&mut self.table);
            self.cap[p.idx()].extend(rest);
        } else {
            self.table.clear();
        }
        let pts = self.deal_points();
        self.score[0] += pts[0];
        self.score[1] += pts[1];
        self.last = format!(
            "Împărțirea {}: J1 +{}, J2 +{}",
            self.deal_no, pts[0], pts[1]
        );
        let (a, b) = (self.score[0], self.score[1]);
        if (a >= TARGET || b >= TARGET) && a != b {
            self.phase = Phase::Over;
            return;
        }
        self.deal_no += 1;
        self.dealer = self.dealer.other();
        self.new_deal();
    }

    /// Valoarea (pentru AI) a unei grămezi.
    fn pile_value(&self, p: usize) -> i32 {
        let mut v = 0;
        for &c in &self.cap[p] {
            v += card_points(c) * 10;
            v += 2;
            if std_suit(c) == 0 {
                v += 3;
            }
        }
        v + 50 * self.sur[p] as i32
    }
}

impl Mc for Pasur {
    fn determinize(&self, viewer: Player, rng: &mut Rng) -> Self {
        let mut g = self.clone();
        let (me, op) = (viewer.idx(), viewer.other().idx());
        let mut pool: Vec<u8> = (0..52u8)
            .filter(|c| {
                !self.hands[me].contains(c)
                    && !self.table.contains(c)
                    && !self.cap[0].contains(c)
                    && !self.cap[1].contains(c)
            })
            .collect();
        shuffle_vec(&mut pool, rng);
        let n = self.hands[op].len().min(pool.len());
        g.hands[op] = pool.drain(..n).collect();
        sort_std(&mut g.hands[op]);
        g.stock = pool;
        g.rng = Rng::new(rng.next_u64());
        g
    }
    fn epoch(&self) -> u32 {
        self.deal_no as u32
    }
    fn margin(&self, p: Player) -> i32 {
        self.score[p.idx()] - self.score[p.other().idx()]
    }
}

impl Game for Pasur {
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
            Phase::Deal => vec![Move::Deal],
            Phase::Play => {
                let mut v = Vec::new();
                for &c in &self.hands[self.side.idx()] {
                    v.push(Move::Play(c, vec![]));
                    for set in captures(c, &self.table) {
                        v.push(Move::Play(c, set));
                    }
                }
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Deal => {
                self.deal_hands();
                self.phase = Phase::Play;
            }
            Move::Play(c, set) => {
                let p = self.side;
                let pos = self.hands[p.idx()]
                    .iter()
                    .position(|x| x == c)
                    .expect("carte în mână");
                self.hands[p.idx()].remove(pos);
                if set.is_empty() {
                    self.table.push(*c);
                    self.last = format!("J{} pune {}", p.idx() + 1, std_text(*c));
                } else {
                    self.table.retain(|x| !set.contains(x));
                    self.cap[p.idx()].push(*c);
                    self.cap[p.idx()].extend(set.iter().copied());
                    self.last_cap = Some(p);
                    let last_card = self.stock.is_empty()
                        && self.hands[0].is_empty()
                        && self.hands[1].is_empty();
                    let mut note = String::new();
                    if self.table.is_empty() && std_rank(*c) != R_J && !last_card {
                        self.sur[p.idx()] += 1;
                        note = " — SUR!".into();
                    }
                    self.last = format!(
                        "J{} capturează {} cu {}{}",
                        p.idx() + 1,
                        set.iter()
                            .map(|&x| std_text(x))
                            .collect::<Vec<_>>()
                            .join(" "),
                        std_text(*c),
                        note
                    );
                }
                self.side = p.other();
                if self.hands[0].is_empty() && self.hands[1].is_empty() {
                    if self.stock.is_empty() {
                        self.finish_deal();
                    } else {
                        self.phase = Phase::Deal;
                        self.side = self.dealer.other();
                    }
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        Some(if self.score[0] > self.score[1] {
            Outcome::Win(Player::One)
        } else {
            Outcome::Win(Player::Two)
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let (me, op) = (p.idx(), p.other().idx());
        let mut s = 1000 * (self.score[me] - self.score[op]);
        if self.phase == Phase::Over {
            return s;
        }
        s += 3 * (self.pile_value(me) - self.pile_value(op));
        // cărți valoroase rămase pe masă
        let table_val: i32 = self.table.iter().map(|&c| card_points(c) * 10 + 4).sum();
        // după mutarea mea, adversarul poate captura masa (aprox.): penalizez masa bogată
        if self.side != p && self.phase == Phase::Play {
            s -= table_val / 4;
            if self.table.is_empty() {
                s += 15;
            }
        }
        // cărți bune ținute în mână
        for &c in &self.hands[me] {
            s += card_points(c) * 4;
        }
        let sig = sig_of(&[
            self.hands[me]
                .iter()
                .fold(0u64, |a, &c| a.wrapping_mul(53).wrapping_add(c as u64 + 1)),
            self.table
                .iter()
                .fold(0u64, |a, &c| a.wrapping_mul(53).wrapping_add(c as u64 + 1)),
            self.cap[0].len() as u64,
        ]);
        s += noise(sig, 50);
        s + mc_value(self, p, mc_samples(), sig)
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Pasur — împărțirea {} | puncte: J1 {}, J2 {} (până la {}) | stoc {}\n",
            self.deal_no,
            self.score[0],
            self.score[1],
            TARGET,
            self.stock.len()
        );
        if !self.last.is_empty() {
            s.push_str(&format!("({})\n", self.last));
        }
        if self.phase == Phase::Over {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        let mut t = self.table.clone();
        sort_std(&mut t);
        s.push_str(&format!(
            "Masa: {}\n",
            if t.is_empty() {
                "—".to_string()
            } else {
                t.iter()
                    .map(|&c| format!("{}[{}]", std_symbol(c), std_text(c)))
                    .collect::<Vec<_>>()
                    .join(" ")
            }
        ));
        s.push_str(&format!(
            "Grămezi: J1 {} cărți ({} sur), J2 {} cărți ({} sur)\n",
            self.cap[0].len(),
            self.sur[0],
            self.cap[1].len(),
            self.sur[1]
        ));
        let v = self.viewer.who(self.side);
        s.push_str(&format!(
            "Mâna ta (J{}): {}\n",
            v.idx() + 1,
            symbols_typed(&self.hands[v.idx()])
        ));
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
            Move::Deal => "imparte".into(),
            Move::Play(c, set) => {
                if set.is_empty() {
                    std_text(*c)
                } else {
                    format!(
                        "{} x {}",
                        std_text(*c),
                        set.iter()
                            .map(|&x| std_text(x))
                            .collect::<Vec<_>>()
                            .join(" ")
                    )
                }
            }
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards52::SUIT_LETTERS;

    fn c(rank: u8, suit: char) -> u8 {
        // rank: 2..10, 11 = J, 12 = Q, 13 = K, 14 = A
        let s = SUIT_LETTERS.iter().position(|&x| x == suit).unwrap() as u8;
        s * 13 + (rank - 2)
    }

    #[test]
    fn number_cards_capture_sum_eleven() {
        let table = vec![c(6, 'h'), c(2, 'c'), c(4, 'd'), c(9, 's')];
        let caps = captures(c(5, 'h'), &table);
        // 5 + 6 = 11; 5 + 2 + 4 = 11; ambele grupuri disjuncte împreună
        assert!(caps.iter().any(|s| s.len() == 1 && s[0] == c(6, 'h')));
        assert!(caps
            .iter()
            .any(|s| s.len() == 2 && s.contains(&c(2, 'c')) && s.contains(&c(4, 'd'))));
        assert!(caps.iter().any(|s| s.len() == 3));
        assert!(!caps.iter().any(|s| s.contains(&c(9, 's'))));
        // As (1) are nevoie de suma 10: 6+4
        let caps = captures(c(14, 'h'), &table);
        assert!(caps
            .iter()
            .any(|s| s.len() == 2 && s.contains(&c(6, 'h')) && s.contains(&c(4, 'd'))));
    }

    #[test]
    fn jack_queen_king_rules() {
        let table = vec![c(6, 'h'), c(12, 'c'), c(13, 'd'), c(11, 's'), c(14, 'c')];
        let j = captures(c(11, 'h'), &table);
        assert_eq!(j.len(), 1);
        let mut s = j[0].clone();
        s.sort();
        let mut want = vec![c(6, 'h'), c(11, 's'), c(14, 'c')];
        want.sort();
        assert_eq!(s, want); // fără Damă și Rege
        assert_eq!(captures(c(12, 'h'), &table), vec![vec![c(12, 'c')]]);
        assert_eq!(captures(c(13, 'h'), &table), vec![vec![c(13, 'd')]]);
        // valet pe masă cu doar K/Q: nu capturează nimic
        assert!(captures(c(11, 'h'), &[c(12, 'c'), c(13, 'd')]).is_empty());
        // 10 captureaza doar Ași
        let caps = captures(c(10, 'h'), &table);
        assert_eq!(caps, vec![vec![c(14, 'c')]]);
    }

    #[test]
    fn sur_rules() {
        let mut g = Pasur::with_seed(1);
        g.set_state_for_test(
            vec![c(6, 'h')],
            vec![c(5, 'c'), c(3, 'd')],
            vec![c(2, 'c'), c(3, 'c')],
        );
        g.side = Player::One;
        g.cap = [Vec::new(), Vec::new()];
        g.apply(&Move::Play(c(5, 'c'), vec![c(6, 'h')]));
        assert_eq!(g.sur[0], 1);
        // sur cu valet nu contează
        let mut g = Pasur::with_seed(1);
        g.set_state_for_test(
            vec![c(6, 'h')],
            vec![c(11, 'c'), c(3, 'd')],
            vec![c(2, 'c'), c(3, 'c')],
        );
        g.side = Player::One;
        g.apply(&Move::Play(c(11, 'c'), vec![c(6, 'h')]));
        assert_eq!(g.sur[0], 0);
    }

    #[test]
    fn deal_points_and_end_sweep() {
        let mut g = Pasur::with_seed(2);
        g.cap = [
            vec![c(10, 'd'), c(2, 'c'), c(14, 'h'), c(11, 's')],
            vec![c(14, 'c'), c(4, 'd')],
        ];
        g.sur = [1, 0];
        // J1: 10♦ 3 + 2♣ 2 + A 1 + J 1 + sur 5 + cele mai multe cărți 3 = 15 ; J2: A♣ 1
        assert_eq!(g.deal_points(), [15, 1]);
        // 7 trefle
        let mut g = Pasur::with_seed(2);
        g.cap[1] = (2..=8).map(|r| c(r, 'c')).collect();
        let p = g.deal_points();
        assert_eq!(p[1], 7 + 2 + 3); // 7 trefle + 2♣ + majoritatea cărților
    }

    #[test]
    fn full_deal_scores_52_cards_and_ai_finishes() {
        for seed in 1..4u64 {
            let mut g = Pasur::with_seed(seed);
            let mut n = 0;
            while g.outcome().is_none() && n < 5000 {
                let m = polytabletop_core::ai::best_move(&g, n).unwrap();
                g.apply(&m);
                n += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed}");
        }
        // conservarea cărților pe parcursul unei împărțiri cu mutări aleatoare
        let mut g = Pasur::with_seed(9);
        let mut rng = Rng::new(3);
        for _ in 0..40 {
            let mv = Game::legal_moves(&g);
            g.apply(&mv[rng.below(mv.len())]);
            let total = g.stock.len()
                + g.table.len()
                + g.hands[0].len()
                + g.hands[1].len()
                + g.cap[0].len()
                + g.cap[1].len();
            assert_eq!(total, 52, "{}", g.last);
        }
    }

    #[test]
    fn render_hides_opponent() {
        let g = Pasur::with_seed(4);
        let r = Game::render(&g);
        for &x in &g.hands[1] {
            assert!(
                !r.contains(&format!("[{}]", std_text(x))) || g.table.contains(&x),
                "{r}"
            );
        }
    }
}

#[cfg(test)]
mod a1_blind {
    use super::*;
    use crate::ai_audit_a1::{audit_seeds, scramble_pool};

    #[test]
    fn ai_ignores_opponent_hand_and_stock_at_all_levels() {
        audit_seeds(
            "pasur",
            Pasur::with_seed,
            1..=3,
            5,
            80,
            |g, salt| {
                let foe = g.side.other().idx();
                scramble_pool(&mut g.hands[foe], &mut g.stock, salt);
                g.rng = Rng::new(salt ^ 3);
            },
            &[7],
        );
    }
}
