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
//! Big Two / Pusoy Dos (China, Filipine) — joc de eliminare a cărților, 2 jucători.
//!
//! Reguli folosite (varianta internațională cea mai răspândită, adaptată pentru doi):
//! * 52 de cărți, 13 fiecăruia; celelalte 26 rămân nefolosite și ascunse (în patru jucători s-ar
//!   împărți toate). Rangul: 3 < 4 < … < K < A < 2 (2 e cea mai mare); culoarea: ♦ < ♣ < ♥ < ♠.
//! * Combinații: carte singură, pereche, treime și mâini de cinci cărți: scară (5 ranguri
//!   consecutive în ordinea 3…2, fără întoarcere), culoare (5 de aceeași culoare), full (treime
//!   + pereche), careu + 1 carte, scară de culoare. Ordinea mâinilor de 5: scară < culoare <
//!   full < careu < scară de culoare. Scările și scările de culoare se compară după cea mai
//!   mare carte (rang, apoi culoare), culorile după culoare apoi cărțile, full-ul după treime,
//!   careul după careu.
//! * Cine conduce joacă orice combinație; celălalt trebuie să dea o combinație **de același
//!   număr de cărți și mai mare** (aceeași formă: pereche peste pereche, mână de 5 peste mână de
//!   5 mai mică), ori `pas`. Dacă adversarul dă pas, conduci din nou. Prima mână: cine are cea
//!   mai mică carte (3♦ de obicei) conduce și trebuie să o includă.
//! * Mâna se termină când cineva rămâne fără cărți; perdantul primește ca penalizare numărul de
//!   cărți rămase (dublu de la 10 cărți în sus). Meciul se termină când un jucător ajunge la 30 de
//!   puncte de penalizare (pierde) sau după 8 mâini (câștigă cine are mai puține).
//! * Omise: scările A-2-3-4-5 / 2-3-4-5-6, ordinea alternativă a culorilor (Pusoy Dos clasic),
//!   regula „2 de la capăt”, 3–4 jucători.
//! * Mutări: cărți separate prin spațiu, ex. `3d`, `9h 9s`, `3d 4c 5h 6s 7d`, `pas`.
//! * Informație ascunsă: vezi doar mâna ta; AI-ul citește doar mâna proprie, cărțile jucate și
//!   numărul cărților adversarului (Hard/Expert: eșantionare Monte-Carlo cu completări ale
//!   cărților necunoscute).

use crate::cards::Viewer;
use crate::cards52::{std_deck, std_rank, std_suit, std_symbol, std_text};
use crate::p2_util::{mc_samples, mc_value, noise, shuffle_vec, sig_of, Mc};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const LIMIT: i32 = 30;
pub const MAX_HANDS: u8 = 8;

/// Rangul în ordinea Big Two: 3 = 0 … A = 11, 2 = 12.
pub fn strength(c: u8) -> u8 {
    (std_rank(c) + 12) % 13
}
/// Culoarea în ordinea ♦ < ♣ < ♥ < ♠.
pub fn suit_ord(c: u8) -> u8 {
    [1u8, 0, 2, 3][std_suit(c) as usize]
}
pub fn card_key(c: u8) -> u32 {
    strength(c) as u32 * 4 + suit_ord(c) as u32
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Combo {
    pub cards: Vec<u8>,
    pub class: u8,
    pub key: u64,
}

impl Combo {
    pub fn beats(&self, o: &Combo) -> bool {
        self.cards.len() == o.cards.len() && (self.class, self.key) > (o.class, o.key)
    }
    pub fn text(&self) -> String {
        self.cards
            .iter()
            .map(|&c| std_text(c))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

fn classify(cards: &[u8]) -> Option<Combo> {
    let mut v = cards.to_vec();
    v.sort_by_key(|&c| card_key(c));
    let n = v.len();
    let top = card_key(v[n - 1]) as u64;
    match n {
        1 => Some(Combo {
            cards: v,
            class: 1,
            key: top,
        }),
        2 => (strength(v[0]) == strength(v[1])).then_some(Combo {
            cards: v,
            class: 2,
            key: top,
        }),
        3 => {
            let k = strength(v[0]) as u64;
            (strength(v[0]) == strength(v[2])).then_some(Combo {
                cards: v,
                class: 3,
                key: k,
            })
        }
        5 => {
            let r: Vec<u8> = v.iter().map(|&c| strength(c)).collect();
            let flush = v.iter().all(|&c| suit_ord(c) == suit_ord(v[0]));
            let straight = (0..4).all(|i| r[i + 1] == r[i] + 1);
            let mut cnt = [0u8; 13];
            for &x in &r {
                cnt[x as usize] += 1;
            }
            let quad = (0..13).find(|&i| cnt[i] == 4);
            let trip = (0..13).find(|&i| cnt[i] == 3);
            let pair = (0..13).any(|i| cnt[i] == 2);
            let class_key = if straight && flush {
                (14, top)
            } else if let Some(q) = quad {
                (13, q as u64)
            } else if trip.is_some() && pair {
                (12, trip.unwrap() as u64)
            } else if flush {
                let mut k = (suit_ord(v[0]) as u64) << 24;
                for (i, &x) in r.iter().rev().enumerate() {
                    k |= (x as u64) << (4 * (4 - i));
                }
                (11, k)
            } else if straight {
                (10, top)
            } else {
                return None;
            };
            Some(Combo {
                cards: v,
                class: class_key.0,
                key: class_key.1,
            })
        }
        _ => None,
    }
}

fn subsets(n: usize, k: usize) -> Vec<Vec<usize>> {
    fn rec(n: usize, k: usize, start: usize, cur: &mut Vec<usize>, out: &mut Vec<Vec<usize>>) {
        if cur.len() == k {
            out.push(cur.clone());
            return;
        }
        for i in start..n {
            cur.push(i);
            rec(n, k, i + 1, cur, out);
            cur.pop();
        }
    }
    let mut out = Vec::new();
    rec(n, k, 0, &mut Vec::new(), &mut out);
    out
}

pub fn gen_combos(hand: &[u8]) -> Vec<Combo> {
    let mut out = Vec::new();
    for k in [1usize, 2, 3, 5] {
        if hand.len() < k {
            continue;
        }
        for s in subsets(hand.len(), k) {
            let cs: Vec<u8> = s.iter().map(|&i| hand[i]).collect();
            if let Some(c) = classify(&cs) {
                out.push(c);
            }
        }
    }
    out
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Play(Combo),
    Pass,
}

#[derive(Clone)]
pub struct BigTwo {
    hands: [Vec<u8>; 2],
    stock: Vec<u8>,
    played: Vec<u8>,
    field: Option<Combo>,
    holder: Player,
    side: Player,
    must: Option<u8>,
    pen: [i32; 2],
    hand_no: u8,
    over: bool,
    last: String,
    rng: Rng,
    viewer: Viewer,
}

impl Default for BigTwo {
    fn default() -> Self {
        Self::new()
    }
}

impl BigTwo {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = BigTwo {
            hands: [Vec::new(), Vec::new()],
            stock: Vec::new(),
            played: Vec::new(),
            field: None,
            holder: Player::One,
            side: Player::One,
            must: None,
            pen: [0; 2],
            hand_no: 1,
            over: false,
            last: String::new(),
            rng: Rng::new(seed),
            viewer: Viewer::Fixed(Player::One),
        };
        g.deal(None);
        g
    }

    fn deal(&mut self, leader: Option<Player>) {
        let mut d = std_deck(&mut self.rng);
        shuffle_vec(&mut d, &mut self.rng);
        self.hands = [Vec::new(), Vec::new()];
        for _ in 0..13 {
            self.hands[0].push(d.pop().unwrap());
            self.hands[1].push(d.pop().unwrap());
        }
        for h in &mut self.hands {
            h.sort_by_key(|&c| card_key(c));
        }
        self.stock = d;
        self.played.clear();
        self.field = None;
        match leader {
            Some(p) => {
                self.side = p;
                self.must = None;
            }
            None => {
                let l0 = card_key(self.hands[0][0]);
                let l1 = card_key(self.hands[1][0]);
                let (p, c) = if l0 < l1 {
                    (Player::One, self.hands[0][0])
                } else {
                    (Player::Two, self.hands[1][0])
                };
                self.side = p;
                self.must = Some(c);
            }
        }
        self.holder = self.side;
    }

    pub fn penalties(&self) -> (i32, i32) {
        (self.pen[0], self.pen[1])
    }
    pub fn set_hands_for_test(&mut self, a: Vec<u8>, b: Vec<u8>) {
        self.hands = [a, b];
        for h in &mut self.hands {
            h.sort_by_key(|&c| card_key(c));
        }
        self.must = None;
    }

    fn turns_estimate(h: &[u8]) -> i32 {
        let mut seen = [false; 13];
        for &c in h {
            seen[strength(c) as usize] = true;
        }
        seen.iter().filter(|&&x| x).count() as i32
    }
}

impl Mc for BigTwo {
    fn determinize(&self, viewer: Player, rng: &mut Rng) -> Self {
        let mut g = self.clone();
        let (me, op) = (viewer.idx(), viewer.other().idx());
        let mut pool: Vec<u8> = (0..52u8)
            .filter(|c| !self.hands[me].contains(c) && !self.played.contains(c))
            .collect();
        shuffle_vec(&mut pool, rng);
        let n = self.hands[op].len();
        g.hands[op] = pool.drain(..n).collect();
        g.hands[op].sort_by_key(|&c| card_key(c));
        g.stock = pool;
        g.rng = Rng::new(rng.next_u64());
        g
    }
    fn epoch(&self) -> u32 {
        self.hand_no as u32
    }
    fn margin(&self, p: Player) -> i32 {
        self.pen[p.other().idx()] - self.pen[p.idx()]
    }
}

impl Game for BigTwo {
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
        let combos = gen_combos(&self.hands[self.side.idx()]);
        let mut v: Vec<Move> = Vec::new();
        for c in combos {
            if let Some(m) = self.must {
                if !c.cards.contains(&m) {
                    continue;
                }
            }
            if let Some(f) = &self.field {
                if !c.beats(f) {
                    continue;
                }
            }
            v.push(Move::Play(c));
        }
        if self.field.is_some() {
            v.push(Move::Pass);
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        let me = self.side;
        match m {
            Move::Pass => {
                self.field = None;
                self.side = self.holder;
                self.last = format!("J{} dă pas", me.idx() + 1);
            }
            Move::Play(c) => {
                let h = &mut self.hands[me.idx()];
                h.retain(|x| !c.cards.contains(x));
                self.played.extend(c.cards.iter().copied());
                self.field = Some(c.clone());
                self.holder = me;
                self.must = None;
                self.last = format!("J{} joacă {}", me.idx() + 1, c.text());
                if self.hands[me.idx()].is_empty() {
                    let o = me.other().idx();
                    let left = self.hands[o].len() as i32;
                    let pen = if left >= 10 { left * 2 } else { left };
                    self.pen[o] += pen;
                    self.last.push_str(&format!(
                        " — mâna se termină: J{} primește {} penalizare",
                        o + 1,
                        pen
                    ));
                    if self.pen[o] >= LIMIT || self.hand_no >= MAX_HANDS {
                        self.over = true;
                    } else {
                        self.hand_no += 1;
                        self.deal(Some(me));
                    }
                    return;
                }
                self.side = me.other();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            return None;
        }
        Some(match self.pen[0].cmp(&self.pen[1]) {
            std::cmp::Ordering::Less => Outcome::Win(Player::One),
            std::cmp::Ordering::Greater => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let (me, op) = (p.idx(), p.other().idx());
        let mut s = 200 * (self.pen[op] - self.pen[me]);
        if self.over {
            return s;
        }
        let h = &self.hands[me];
        let ctrl = h.iter().filter(|&&c| strength(c) >= 11).count() as i32;
        s += 30 * (self.hands[op].len() as i32 - h.len() as i32);
        s -= 35 * Self::turns_estimate(h);
        s += 22 * ctrl;
        if self.field.is_none() && self.side == p {
            s += 25;
        }
        let sig = sig_of(&[
            h.iter()
                .fold(0u64, |a, &c| a.wrapping_mul(53).wrapping_add(c as u64 + 1)),
            self.played.len() as u64,
            self.pen[0] as u64,
        ]);
        s += noise(sig, 80);
        s + mc_value(self, p, mc_samples(), sig)
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Big Two — mâna {}/{} | penalizări: J1 {}, J2 {} (limita {})\n",
            self.hand_no.min(MAX_HANDS),
            MAX_HANDS,
            self.pen[0],
            self.pen[1],
            LIMIT
        );
        if !self.last.is_empty() {
            s.push_str(&format!("({})\n", self.last));
        }
        if self.over {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        match &self.field {
            Some(c) => s.push_str(&format!(
                "Câmp: {}\n",
                c.cards
                    .iter()
                    .map(|&x| std_symbol(x))
                    .collect::<Vec<_>>()
                    .join(" ")
            )),
            None => s.push_str("Câmp: gol (conduci)\n"),
        }
        let v = self.viewer.who(self.side);
        s.push_str(&format!(
            "Mâna ta (J{}): {}\n",
            v.idx() + 1,
            self.hands[v.idx()]
                .iter()
                .map(|&c| format!("{}[{}]", std_symbol(c), std_text(c)))
                .collect::<Vec<_>>()
                .join(" ")
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
            Move::Pass => "pas".into(),
            Move::Play(c) => c.text(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// rang 3..=15 (11=J, 12=Q, 13=K, 14=A, 15=2), culoare 'c','d','h','s'.
    fn c(rank: u8, suit: char) -> u8 {
        let r0 = if rank == 15 { 0 } else { rank - 2 };
        let s = match suit {
            'c' => 0,
            'd' => 1,
            'h' => 2,
            _ => 3,
        };
        s * 13 + r0
    }
    fn combo(v: &[(u8, char)]) -> Combo {
        classify(&v.iter().map(|&(r, s)| c(r, s)).collect::<Vec<_>>()).expect("combinație")
    }

    #[test]
    fn rank_and_suit_order() {
        assert!(card_key(c(15, 'd')) > card_key(c(14, 's')));
        assert!(card_key(c(3, 's')) > card_key(c(3, 'h')));
        assert!(card_key(c(3, 'h')) > card_key(c(3, 'c')));
        assert!(card_key(c(3, 'c')) > card_key(c(3, 'd')));
        assert!(card_key(c(3, 'd')) < card_key(c(4, 'd')));
    }

    #[test]
    fn five_card_hierarchy() {
        let straight = combo(&[(3, 'd'), (4, 'c'), (5, 'h'), (6, 's'), (7, 'd')]);
        let flush = combo(&[(3, 'h'), (5, 'h'), (7, 'h'), (9, 'h'), (11, 'h')]);
        let full = combo(&[(3, 'h'), (3, 'd'), (3, 'c'), (4, 'h'), (4, 's')]);
        let quad = combo(&[(5, 'h'), (5, 'd'), (5, 'c'), (5, 's'), (4, 's')]);
        let sf = combo(&[(3, 'd'), (4, 'd'), (5, 'd'), (6, 'd'), (7, 'd')]);
        assert!(flush.beats(&straight));
        assert!(full.beats(&flush));
        assert!(quad.beats(&full));
        assert!(sf.beats(&quad));
        // scara mai mare după cartea cea mai mare
        let s2 = combo(&[(4, 'd'), (5, 'c'), (6, 'h'), (7, 's'), (8, 'd')]);
        assert!(s2.beats(&straight));
        // culoare: întâi culoarea (♠ bate ♥)
        let fs = combo(&[(3, 's'), (5, 's'), (7, 's'), (9, 's'), (11, 's')]);
        assert!(fs.beats(&flush));
        // 5 cărți nu bat o pereche
        let pair = combo(&[(9, 'h'), (9, 's')]);
        assert!(!straight.beats(&pair));
        // 2 înalt: J Q K A 2 e scară
        assert!(classify(&[c(11, 'd'), c(12, 'd'), c(13, 'c'), c(14, 'h'), c(15, 's')]).is_some());
        // fără întoarcere: Q K A 2 3
        assert!(classify(&[c(12, 'd'), c(13, 'd'), c(14, 'c'), c(15, 'h'), c(3, 's')]).is_none());
    }

    #[test]
    fn pairs_and_triples_compare() {
        let p1 = combo(&[(9, 'h'), (9, 'c')]);
        let p2 = combo(&[(9, 'd'), (9, 's')]);
        let p3 = combo(&[(10, 'd'), (10, 'c')]);
        assert!(p2.beats(&p1) && p3.beats(&p2) && !p1.beats(&p2));
        let t1 = combo(&[(9, 'h'), (9, 'c'), (9, 'd')]);
        let t2 = combo(&[(10, 'h'), (10, 'c'), (10, 'd')]);
        assert!(t2.beats(&t1));
        assert!(!t2.beats(&p1));
    }

    #[test]
    fn first_move_must_include_lowest_card() {
        for seed in 1..10 {
            let g = BigTwo::with_seed(seed);
            let low = g.must.unwrap();
            for m in Game::legal_moves(&g) {
                match m {
                    Move::Play(cb) => assert!(cb.cards.contains(&low)),
                    Move::Pass => panic!("nu se poate pasa la început"),
                }
            }
            // cartea cea mai mică din cele două mâini
            let m0 = card_key(g.hands[0][0]);
            let m1 = card_key(g.hands[1][0]);
            assert_eq!(card_key(low), m0.min(m1));
        }
    }

    #[test]
    fn follow_must_beat_same_size_and_pass_gives_lead() {
        let mut g = BigTwo::with_seed(2);
        g.set_hands_for_test(
            vec![c(9, 'h'), c(9, 's'), c(5, 'c'), c(15, 'd')],
            vec![c(10, 'h'), c(10, 'c'), c(6, 'd'), c(4, 'd')],
        );
        g.side = Player::One;
        g.holder = Player::One;
        g.apply(&Move::Play(combo(&[(9, 'h'), (9, 's')])));
        let mv = Game::legal_moves(&g);
        assert!(mv.contains(&Move::Pass));
        assert_eq!(mv.len(), 2); // 10-10 sau pas
        g.apply(&Move::Pass);
        assert_eq!(g.to_move(), Player::One);
        assert!(g.field.is_none());
        assert!(!Game::legal_moves(&g).contains(&Move::Pass));
    }

    #[test]
    fn hand_end_penalty_and_redeal() {
        let mut g = BigTwo::with_seed(3);
        g.set_hands_for_test(vec![c(9, 'h')], (3..13).map(|r| c(r, 'c')).collect());
        g.side = Player::One;
        g.apply(&Move::Play(combo(&[(9, 'h')])));
        // J2 avea 10 cărți → penalizare dublă
        assert_eq!(g.penalties(), (0, 20));
        assert_eq!(g.hand_no, 2);
        assert_eq!(g.to_move(), Player::One);
        assert_eq!(g.hands[0].len(), 13);
        // 30 de puncte → sfârșit
        g.pen[1] = 20;
        g.set_hands_for_test(vec![c(9, 'h')], (3..13).map(|r| c(r, 'c')).collect());
        g.side = Player::One;
        g.apply(&Move::Play(combo(&[(9, 'h')])));
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn render_hides_opponent_and_viewers() {
        let g = BigTwo::with_seed(4);
        let r = Game::render(&g);
        for &x in &g.hands[1] {
            assert!(!r.contains(&format!("[{}]", std_text(x))), "{r}");
        }
        assert!(r.contains("Mâna ta (J1)"));
    }

    #[test]
    fn determinize_keeps_own_hand_and_known_cards() {
        let g = BigTwo::with_seed(5);
        let mut rng = Rng::new(9);
        let d = g.determinize(Player::One, &mut rng);
        assert_eq!(d.hands[0], g.hands[0]);
        assert_eq!(d.hands[1].len(), 13);
        let mut all: Vec<u8> = d.hands[0]
            .iter()
            .chain(&d.hands[1])
            .chain(&d.stock)
            .copied()
            .collect();
        all.sort();
        all.dedup();
        assert_eq!(all.len(), 52);
    }

    #[test]
    fn ai_finishes_match() {
        let mut g = BigTwo::with_seed(6);
        let mut n = 0;
        while g.outcome().is_none() && n < 4000 {
            let m = polytabletop_core::ai::best_move(&g, n).unwrap();
            g.apply(&m);
            n += 1;
        }
        assert!(g.outcome().is_some());
    }
}

#[cfg(test)]
mod a2_audit {
    use super::*;

    #[test]
    fn ai_ignores_hidden_information_at_every_level() {
        crate::p3_comun::ai_blind_check_with(
            "bigtwo",
            BigTwo::with_seed,
            |g, me, seed| {
                let mut rng = Rng::new(seed);
                *g = crate::p2_util::Mc::determinize(g, me, &mut rng);
            },
            2,
            4,
            crate::p3_comun::Term::Check,
        );
    }
}
