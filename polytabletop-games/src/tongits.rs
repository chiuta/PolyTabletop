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
//! Tong-its / Tongits (Filipine) — rummy cu apel „draw”, 2 jucători.
//!
//! Reguli folosite (varianta cea mai răspândită, adaptată pentru doi):
//! * 52 de cărți. Dealerul primește 13 cărți, celălalt 12; dealerul începe (poate pune combinații,
//!   apoi aruncă o carte, fără să tragă). Apoi fiecare tură: tragi din stoc (`trage`) sau iei cartea
//!   de sus din aruncate dacă o folosești imediat într-o combinație (`ia 5d 5h 5c`) sau o
//!   adaugi la una existentă (`ia sapaw N`); apoi poți pune combinații (`pune 3h 4h 5h`), poți
//!   adăuga cărți la orice combinație de pe masă, și ale adversarului (`sapaw 6h N`), și
//!   arunci o carte (`arunca Kd`).
//! * Combinații: set de 3–4 cărți de același rang (culori diferite) sau scară de cel puțin 3 cărți
//!   consecutive de aceeași culoare (As doar jos: A-2-3, nu Q-K-A). Valori: A 1, 2–10 cât scrie,
//!   J/Q/K 10.
//! * Câștigi mâna: (a) rămâi fără cărți („Tongits”): 3 puncte; (b) apelezi `cheama` (draw) la
//!   începutul turei, dacă ai cel puțin o combinație pusă: adversarul spune `renunt` (tu iei 1
//!   punct) sau `lupta`: se compară cărțile rămase în mână (suma valorilor) — cine are mai puțin
//!   câștigă 1 punct; la egalitate câștigă cel care a luptat; dacă cel care a chemat pierde,
//!   adversarul primește 2 puncte; (c) stocul se golește: după aruncarea celui care a tras
//!   ultima carte, cine are mai puține puncte în mână câștigă 1 (egalitate = fără punct).
//! * Meciul: primul la 6 puncte. Dealerul alternează.
//! * Omise: combinații ascunse („secret melds”), regula „burn”, 3 jucători, miza de jetoane;
//!   punctajul e o adaptare simplă a plăților.
//! * Informație ascunsă: vezi doar mâna ta; combinațiile și aruncatele sunt publice. AI-ul
//!   minimizează cărțile rămase în mână și evită să arunce ce adversarul poate folosi.

use crate::cards::Viewer;
use crate::cards52::{std_deck, std_symbol, std_text};
use crate::p2_util::{noise, shuffle_vec, sig_of, symbols_typed};
use polytabletop_core::ai::{level, Level};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::collections::HashMap;

pub const TARGET: i32 = 6;

/// Rangul cu As jos: A = 0, 2 = 1 … K = 12.
pub fn tr(c: u8) -> u8 {
    (c % 13 + 1) % 13
}
pub fn suit(c: u8) -> u8 {
    c / 13
}
pub fn val(c: u8) -> i32 {
    (tr(c) as i32 + 1).min(10)
}

fn sort_cards(v: &mut [u8]) {
    v.sort_by_key(|&c| (suit(c), tr(c)));
}

pub fn is_set(cs: &[u8]) -> bool {
    if cs.len() < 3 || cs.len() > 4 {
        return false;
    }
    let r = tr(cs[0]);
    if !cs.iter().all(|&c| tr(c) == r) {
        return false;
    }
    let mut s: Vec<u8> = cs.iter().map(|&c| suit(c)).collect();
    s.sort();
    s.dedup();
    s.len() == cs.len()
}

pub fn is_run(cs: &[u8]) -> bool {
    if cs.len() < 3 {
        return false;
    }
    let s = suit(cs[0]);
    if !cs.iter().all(|&c| suit(c) == s) {
        return false;
    }
    let mut r: Vec<u8> = cs.iter().map(|&c| tr(c)).collect();
    r.sort();
    r.windows(2).all(|w| w[1] == w[0] + 1)
}

pub fn valid_meld(cs: &[u8]) -> bool {
    is_set(cs) || is_run(cs)
}

/// Toate combinațiile posibile cu cărțile din `hand`.
pub fn all_melds(hand: &[u8]) -> Vec<Vec<u8>> {
    let mut out: Vec<Vec<u8>> = Vec::new();
    // seturi
    for r in 0..13u8 {
        let g: Vec<u8> = hand.iter().copied().filter(|&c| tr(c) == r).collect();
        let n = g.len();
        if n >= 3 {
            for skip in 0..n {
                let m: Vec<u8> = (0..n).filter(|&i| i != skip).map(|i| g[i]).collect();
                if n == 3 {
                    if skip == 0 {
                        out.push(g.clone());
                    }
                    break;
                }
                out.push(m);
            }
            if n == 4 {
                out.push(g.clone());
            }
        }
    }
    // scări
    for s in 0..4u8 {
        let mut ranks: Vec<u8> = hand
            .iter()
            .copied()
            .filter(|&c| suit(c) == s)
            .map(tr)
            .collect();
        ranks.sort();
        let n = ranks.len();
        for i in 0..n {
            let mut j = i;
            while j + 1 < n && ranks[j + 1] == ranks[j] + 1 {
                j += 1;
                if j - i + 1 >= 3 {
                    out.push(
                        ranks[i..=j]
                            .iter()
                            .map(|&r| s * 13 + (r + 12) % 13)
                            .collect(),
                    );
                }
            }
        }
    }
    for m in &mut out {
        sort_cards(m);
    }
    out.sort();
    out.dedup();
    out
}

/// Suma minimă a cărților rămase în mână după cea mai bună acoperire cu combinații.
pub fn min_deadwood(hand: &[u8]) -> i32 {
    let n = hand.len();
    if n == 0 {
        return 0;
    }
    let melds: Vec<u32> = all_melds(hand)
        .into_iter()
        .map(|m| {
            m.iter()
                .map(|c| 1u32 << hand.iter().position(|x| x == c).unwrap())
                .sum()
        })
        .collect();
    fn rec(mask: u32, hand: &[u8], melds: &[u32], memo: &mut HashMap<u32, i32>) -> i32 {
        if mask == 0 {
            return 0;
        }
        if let Some(&v) = memo.get(&mask) {
            return v;
        }
        let i = mask.trailing_zeros() as usize;
        let rest = mask & (mask - 1);
        let mut best = val(hand[i]) + rec(rest, hand, melds, memo);
        for &m in melds {
            if m & (1 << i) != 0 && m & mask == m {
                best = best.min(rec(mask & !m, hand, melds, memo));
            }
        }
        memo.insert(mask, best);
        best
    }
    let full = if n >= 32 { u32::MAX } else { (1u32 << n) - 1 };
    rec(full, hand, &melds, &mut HashMap::new())
}

/// Potențial: perechi / goluri într-o scară.
fn near_count(hand: &[u8]) -> i32 {
    let mut k = 0;
    for i in 0..hand.len() {
        for j in (i + 1)..hand.len() {
            let (a, b) = (hand[i], hand[j]);
            if tr(a) == tr(b) {
                k += 2;
            } else if suit(a) == suit(b) {
                let d = (tr(a) as i32 - tr(b) as i32).abs();
                if d == 1 {
                    k += 2;
                } else if d == 2 {
                    k += 1;
                }
            }
        }
    }
    k
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Meld {
    pub cards: Vec<u8>,
    pub owner: Player,
}

impl Meld {
    pub fn is_run(&self) -> bool {
        is_run(&self.cards)
    }
    pub fn can_lay(&self, c: u8) -> bool {
        let mut t = self.cards.clone();
        t.push(c);
        valid_meld(&t)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Draw,
    Take(Vec<u8>),
    TakeLay(usize),
    Call,
    Fight,
    Fold,
    Reveal,
    Meld(Vec<u8>),
    Lay(u8, usize),
    Discard(u8),
    Next,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Turn,
    Meld,
    Respond,
    Showdown,
    HandOver,
    Over,
}

#[derive(Clone)]
pub struct Tongits {
    hands: [Vec<u8>; 2],
    stock: Vec<u8>,
    discard: Vec<u8>,
    melds: Vec<Meld>,
    pts: [i32; 2],
    dealer: Player,
    side: Player,
    phase: Phase,
    /// Cartea trasă din stoc în tura curentă (nu o „vede” evaluarea).
    fresh: Option<u8>,
    caller: Player,
    first_turn: bool,
    hand_no: u8,
    /// Cărțile pe care fiecare le-a luat din aruncate (publice).
    took: [Vec<u8>; 2],
    last: String,
    rng: Rng,
    viewer: Viewer,
}

impl Default for Tongits {
    fn default() -> Self {
        Self::new()
    }
}

impl Tongits {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Tongits {
            hands: [Vec::new(), Vec::new()],
            stock: Vec::new(),
            discard: Vec::new(),
            melds: Vec::new(),
            pts: [0; 2],
            dealer: Player::One,
            side: Player::One,
            phase: Phase::Meld,
            fresh: None,
            caller: Player::One,
            first_turn: true,
            hand_no: 1,
            took: [Vec::new(), Vec::new()],
            last: String::new(),
            rng: Rng::new(seed),
            viewer: Viewer::Fixed(Player::One),
        };
        g.deal();
        g
    }

    fn deal(&mut self) {
        let mut d = std_deck(&mut self.rng);
        shuffle_vec(&mut d, &mut self.rng);
        self.hands = [Vec::new(), Vec::new()];
        let dl = self.dealer.idx();
        for _ in 0..12 {
            self.hands[0].push(d.pop().unwrap());
            self.hands[1].push(d.pop().unwrap());
        }
        self.hands[dl].push(d.pop().unwrap());
        for h in &mut self.hands {
            sort_cards(h);
        }
        self.stock = d;
        self.discard.clear();
        self.melds.clear();
        self.took = [Vec::new(), Vec::new()];
        self.fresh = None;
        self.first_turn = true;
        self.side = self.dealer;
        self.phase = Phase::Meld;
    }

    pub fn points(&self) -> (i32, i32) {
        (self.pts[0], self.pts[1])
    }
    pub fn set_hands_for_test(&mut self, a: Vec<u8>, b: Vec<u8>) {
        self.hands = [a, b];
    }

    fn finish(&mut self, winner: Option<Player>, pts: i32, why: &str) {
        if let Some(w) = winner {
            self.pts[w.idx()] += pts;
            self.last = format!(
                "Mâna {}: J{} câștigă {} p. ({})",
                self.hand_no,
                w.idx() + 1,
                pts,
                why
            );
        } else {
            self.last = format!("Mâna {}: egalitate ({})", self.hand_no, why);
        }
        self.phase = if self.pts.iter().any(|&p| p >= TARGET) {
            Phase::Over
        } else {
            Phase::HandOver
        };
    }

    fn deadwood(&self, p: usize) -> i32 {
        self.hands[p].iter().map(|&c| val(c)).sum()
    }

    fn check_empty(&mut self, p: Player) -> bool {
        if self.hands[p.idx()].is_empty() {
            self.finish(Some(p), 3, "Tongits: mâna goală");
            return true;
        }
        false
    }

    fn can_take(&self) -> Vec<Move> {
        let mut v = Vec::new();
        let Some(&d) = self.discard.last() else {
            return v;
        };
        let h = &self.hands[self.side.idx()];
        let mut pool = h.clone();
        pool.push(d);
        for m in all_melds(&pool) {
            if m.contains(&d) {
                v.push(Move::Take(m));
            }
        }
        for (i, m) in self.melds.iter().enumerate() {
            if m.can_lay(d) {
                v.push(Move::TakeLay(i));
            }
        }
        v
    }

    /// Cartea ar putea fi folosită de adversar (extinde o combinație sau are rang luat).
    fn danger(&self, c: u8, me: usize) -> i32 {
        let mut d = 0;
        for m in &self.melds {
            if m.can_lay(c) {
                d += 25;
            }
        }
        if self.took[1 - me].iter().any(|&t| tr(t) == tr(c)) {
            d += 12;
        }
        if self.took[1 - me]
            .iter()
            .any(|&t| suit(t) == suit(c) && (tr(t) as i32 - tr(c) as i32).abs() <= 2)
        {
            d += 8;
        }
        d
    }

    /// Valoarea unei mâini (după ce urmează o aruncare optimă dacă `pending_discard`).
    fn hand_value(&self, me: usize, hand: &[u8], pending_discard: bool) -> i32 {
        let base = |h: &[u8]| -> i32 { -12 * min_deadwood(h) + 3 * near_count(h) };
        if !pending_discard || hand.is_empty() {
            return base(hand);
        }
        let mut best = i32::MIN;
        for i in 0..hand.len() {
            let mut h = hand.to_vec();
            let c = h.remove(i);
            let v = base(&h) - self.danger(c, me) * if level() >= Level::Hard { 2 } else { 1 };
            best = best.max(v);
        }
        best
    }
}

impl Game for Tongits {
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
        let me = self.side.idx();
        match self.phase {
            Phase::Over => Vec::new(),
            Phase::HandOver => vec![Move::Next],
            Phase::Showdown => vec![Move::Reveal],
            Phase::Respond => vec![Move::Fight, Move::Fold],
            Phase::Turn => {
                let mut v = Vec::new();
                if !self.stock.is_empty() {
                    v.push(Move::Draw);
                }
                v.extend(self.can_take());
                if self.melds.iter().any(|m| m.owner == self.side) {
                    v.push(Move::Call);
                }
                v
            }
            Phase::Meld => {
                let h = &self.hands[me];
                let mut v = Vec::new();
                for m in all_melds(h) {
                    v.push(Move::Meld(m));
                }
                for &c in h {
                    for (i, m) in self.melds.iter().enumerate() {
                        if m.can_lay(c) {
                            v.push(Move::Lay(c, i));
                        }
                    }
                }
                for &c in h {
                    v.push(Move::Discard(c));
                }
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        let me = self.side;
        let i = me.idx();
        match m {
            Move::Next => {
                self.hand_no += 1;
                self.dealer = self.dealer.other();
                self.deal();
            }
            Move::Draw => {
                let c = self.stock.pop().expect("stoc");
                self.hands[i].push(c);
                sort_cards(&mut self.hands[i]);
                self.fresh = Some(c);
                self.phase = Phase::Meld;
                self.first_turn = false;
            }
            Move::Take(cards) => {
                let d = self.discard.pop().expect("aruncate");
                self.took[i].push(d);
                for c in cards {
                    if *c != d {
                        let p = self.hands[i].iter().position(|x| x == c).expect("în mână");
                        self.hands[i].remove(p);
                    }
                }
                self.melds.push(Meld {
                    cards: cards.clone(),
                    owner: me,
                });
                self.last = format!(
                    "J{} ia {} și pune {}",
                    i + 1,
                    std_text(d),
                    cards_text(cards)
                );
                self.phase = Phase::Meld;
                self.first_turn = false;
                self.fresh = None;
                self.check_empty(me);
            }
            Move::TakeLay(k) => {
                let d = self.discard.pop().expect("aruncate");
                self.took[i].push(d);
                self.melds[*k].cards.push(d);
                sort_cards(&mut self.melds[*k].cards);
                self.last = format!(
                    "J{} ia {} și o adaugă la combinația {}",
                    i + 1,
                    std_text(d),
                    k + 1
                );
                self.phase = Phase::Meld;
                self.first_turn = false;
                self.fresh = None;
            }
            Move::Call => {
                self.caller = me;
                self.phase = Phase::Respond;
                self.side = me.other();
                self.last = format!("J{} cheamă (draw)", i + 1);
            }
            Move::Fold => {
                let caller = self.caller;
                self.finish(Some(caller), 1, "adversarul renunță");
            }
            Move::Fight => {
                self.phase = Phase::Showdown;
                self.last = format!("J{} acceptă lupta", i + 1);
            }
            Move::Reveal => {
                let caller = self.caller;
                let (dc, dr) = (
                    self.deadwood(caller.idx()),
                    self.deadwood(caller.other().idx()),
                );
                let why = format!(
                    "J{} {} vs J{} {}",
                    caller.idx() + 1,
                    dc,
                    caller.other().idx() + 1,
                    dr
                );
                if dc < dr {
                    self.finish(Some(caller), 1, &why);
                } else {
                    self.finish(Some(caller.other()), 2, &why);
                }
            }
            Move::Meld(cards) => {
                for c in cards {
                    let p = self.hands[i].iter().position(|x| x == c).expect("în mână");
                    self.hands[i].remove(p);
                }
                self.melds.push(Meld {
                    cards: cards.clone(),
                    owner: me,
                });
                self.last = format!("J{} pune {}", i + 1, cards_text(cards));
                self.check_empty(me);
            }
            Move::Lay(c, k) => {
                let p = self.hands[i].iter().position(|x| x == c).expect("în mână");
                self.hands[i].remove(p);
                self.melds[*k].cards.push(*c);
                sort_cards(&mut self.melds[*k].cards);
                self.last = format!("J{} adaugă {} la combinația {}", i + 1, std_text(*c), k + 1);
                self.check_empty(me);
            }
            Move::Discard(c) => {
                let p = self.hands[i].iter().position(|x| x == c).expect("în mână");
                self.hands[i].remove(p);
                self.discard.push(*c);
                self.last = format!("J{} aruncă {}", i + 1, std_text(*c));
                self.fresh = None;
                self.first_turn = false;
                if self.check_empty(me) {
                    return;
                }
                if self.stock.is_empty() {
                    let (a, b) = (self.deadwood(0), self.deadwood(1));
                    let why = format!("stoc epuizat: J1 {a}, J2 {b}");
                    match a.cmp(&b) {
                        std::cmp::Ordering::Less => self.finish(Some(Player::One), 1, &why),
                        std::cmp::Ordering::Greater => self.finish(Some(Player::Two), 1, &why),
                        std::cmp::Ordering::Equal => self.finish(None, 0, &why),
                    }
                    return;
                }
                self.phase = Phase::Turn;
                self.side = me.other();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        Some(match self.pts[0].cmp(&self.pts[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let (me, op) = (p.idx(), p.other().idx());
        let mut s = 1000 * (self.pts[me] - self.pts[op]);
        if matches!(self.phase, Phase::Over | Phase::HandOver) {
            return s;
        }
        // mâna proprie fără cartea trasă din stoc (nevăzută la alegere)
        let mut mine = self.hands[me].clone();
        if self.side == p {
            if let Some(f) = self.fresh {
                if let Some(pos) = mine.iter().position(|&x| x == f) {
                    mine.remove(pos);
                }
            }
        }
        let pending = self.phase == Phase::Meld && self.side == p;
        s += self.hand_value(me, &mine, pending);
        if self.side == p && self.fresh.is_some() {
            s -= 36; // o carte necunoscută: în medie adaugă puncte moarte
        }
        // puncte în mână ale adversarului: estimare după numărul de cărți
        s -= 4 * (self.hands[op].len() as i32) * 5 / 2;
        s += 6 * self
            .melds
            .iter()
            .filter(|m| m.owner == p)
            .map(|m| m.cards.len() as i32)
            .sum::<i32>();
        s -= 4 * self
            .melds
            .iter()
            .filter(|m| m.owner != p)
            .map(|m| m.cards.len() as i32)
            .sum::<i32>();
        match self.phase {
            Phase::Respond | Phase::Showdown => {
                // calea apelului: câștig dacă am mai puține puncte decât estimarea adversarului
                let dead = min_deadwood(&self.hands[me]) as f64;
                let est = 4.2 * self.hands[op].len() as f64;
                let adv = (est - dead) * 25.0;
                s += if self.caller == p {
                    adv as i32
                } else {
                    -(adv as i32)
                };
            }
            _ => {}
        }
        let sig = sig_of(&[
            mine.iter()
                .fold(0u64, |a, &c| a.wrapping_mul(53).wrapping_add(c as u64 + 1)),
            self.discard.len() as u64,
            self.pts[0] as u64,
        ]);
        s + noise(sig, 60)
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Tong-its — mâna {} | puncte: J1 {}, J2 {} (până la {}) | dealer: J{} | stoc {}\n",
            self.hand_no,
            self.pts[0],
            self.pts[1],
            TARGET,
            self.dealer.idx() + 1,
            self.stock.len()
        );
        if !self.last.is_empty() {
            s.push_str(&format!("({})\n", self.last));
        }
        if self.phase == Phase::Over {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        for (k, m) in self.melds.iter().enumerate() {
            s.push_str(&format!(
                "Combinația {} (J{}): {}\n",
                k + 1,
                m.owner.idx() + 1,
                m.cards
                    .iter()
                    .map(|&c| std_symbol(c))
                    .collect::<Vec<_>>()
                    .join(" ")
            ));
        }
        let top: Vec<String> = self
            .discard
            .iter()
            .rev()
            .take(4)
            .map(|&c| std_symbol(c))
            .collect();
        s.push_str(&format!(
            "Aruncate (sus): {}\n",
            if top.is_empty() {
                "—".into()
            } else {
                top.join(" ")
            }
        ));
        if self.phase == Phase::HandOver {
            s.push_str("Mâna s-a terminat.\n");
        }
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
            Move::Draw => "trage".into(),
            Move::Take(c) => format!("ia {}", cards_text(c)),
            Move::TakeLay(k) => format!("ia sapaw {}", k + 1),
            Move::Call => "cheama".into(),
            Move::Fight => "lupta".into(),
            Move::Fold => "renunt".into(),
            Move::Reveal => "arata".into(),
            Move::Meld(c) => format!("pune {}", cards_text(c)),
            Move::Lay(c, k) => format!("sapaw {} {}", std_text(*c), k + 1),
            Move::Discard(c) => format!("arunca {}", std_text(*c)),
            Move::Next => "urmatoarea".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

fn cards_text(c: &[u8]) -> String {
    c.iter().map(|&x| std_text(x)).collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards52::SUIT_LETTERS;

    fn c(rank: u8, suit: char) -> u8 {
        // rank 1 = As, 2..10, 11 J, 12 Q, 13 K
        let s = SUIT_LETTERS.iter().position(|&x| x == suit).unwrap() as u8;
        let r0 = if rank == 1 { 12 } else { rank - 2 };
        s * 13 + r0
    }

    #[test]
    fn meld_validation() {
        assert!(valid_meld(&[c(5, 'h'), c(5, 'd'), c(5, 'c')]));
        assert!(valid_meld(&[c(5, 'h'), c(5, 'd'), c(5, 'c'), c(5, 's')]));
        assert!(!valid_meld(&[c(5, 'h'), c(5, 'h'), c(5, 'c')]));
        assert!(valid_meld(&[c(1, 'h'), c(2, 'h'), c(3, 'h')]));
        assert!(!valid_meld(&[c(12, 'h'), c(13, 'h'), c(1, 'h')])); // As doar jos
        assert!(valid_meld(&[c(4, 's'), c(5, 's'), c(6, 's'), c(7, 's')]));
        assert!(!valid_meld(&[c(4, 's'), c(5, 'h'), c(6, 's')]));
        assert!(!valid_meld(&[c(4, 's'), c(5, 's')]));
    }

    #[test]
    fn deadwood_and_values() {
        assert_eq!(val(c(1, 'h')), 1);
        assert_eq!(val(c(13, 'h')), 10);
        assert_eq!(val(c(9, 'h')), 9);
        let h = vec![
            c(5, 'h'),
            c(5, 'd'),
            c(5, 'c'),
            c(7, 's'),
            c(13, 'h'),
            c(2, 'd'),
        ];
        assert_eq!(min_deadwood(&h), 7 + 10 + 2);
        let h = vec![
            c(3, 'h'),
            c(4, 'h'),
            c(5, 'h'),
            c(6, 'h'),
            c(6, 'd'),
            c(6, 'c'),
        ];
        assert_eq!(min_deadwood(&h), 0); // 3-4-5h + 6h6d6c
        assert!(all_melds(&h).len() >= 4);
    }

    #[test]
    fn dealer_starts_with_13_and_discards_first() {
        let g = Tongits::with_seed(1);
        assert_eq!(g.hands[0].len(), 13);
        assert_eq!(g.hands[1].len(), 12);
        assert_eq!(g.stock.len(), 27);
        let mv = Game::legal_moves(&g);
        assert!(mv
            .iter()
            .all(|m| matches!(m, Move::Meld(_) | Move::Lay(..) | Move::Discard(_))));
        assert!(mv.iter().any(|m| matches!(m, Move::Discard(_))));
    }

    #[test]
    fn take_discard_requires_meld_and_lay_off_on_opponent_meld() {
        let mut g = Tongits::with_seed(2);
        g.set_hands_for_test(
            vec![c(5, 'h'), c(5, 'd'), c(9, 's'), c(2, 's')],
            vec![c(8, 'h'), c(9, 'h'), c(13, 's'), c(3, 'c')],
        );
        g.discard = vec![c(5, 'c')];
        g.melds = vec![Meld {
            cards: vec![c(6, 's'), c(7, 's'), c(8, 's')],
            owner: Player::Two,
        }];
        g.phase = Phase::Turn;
        g.side = Player::One;
        g.first_turn = false;
        let mv = Game::legal_moves(&g);
        assert!(mv.contains(&Move::Draw));
        assert!(mv
            .iter()
            .any(|m| matches!(m, Move::Take(t) if t.len() == 3)));
        assert!(!mv.iter().any(|m| matches!(m, Move::TakeLay(_)))); // 5c nu intră în 6-7-8♠
        g.apply(&Move::Draw);
        // 9♠ se poate adăuga la scara adversarului 6-7-8♠
        let mv = Game::legal_moves(&g);
        assert!(mv.contains(&Move::Lay(c(9, 's'), 0)));
    }

    #[test]
    fn lay_off_extends_runs_and_sets() {
        let m = Meld {
            cards: vec![c(6, 's'), c(7, 's'), c(8, 's')],
            owner: Player::One,
        };
        assert!(m.can_lay(c(5, 's')));
        assert!(m.can_lay(c(9, 's')));
        assert!(!m.can_lay(c(10, 's')));
        assert!(!m.can_lay(c(5, 'h')));
        let set = Meld {
            cards: vec![c(9, 's'), c(9, 'h'), c(9, 'd')],
            owner: Player::One,
        };
        assert!(set.can_lay(c(9, 'c')));
        assert!(!set.can_lay(c(9, 's')));
        assert!(!set.can_lay(c(8, 'c')));
    }

    #[test]
    fn empty_hand_is_tongits_and_call_fight_scoring() {
        let mut g = Tongits::with_seed(3);
        g.set_hands_for_test(
            vec![c(5, 'h'), c(5, 'd'), c(5, 'c')],
            vec![c(8, 'h'), c(9, 'h')],
        );
        g.side = Player::One;
        g.phase = Phase::Meld;
        g.apply(&Move::Meld(vec![c(5, 'h'), c(5, 'd'), c(5, 'c')]));
        assert_eq!(g.points(), (3, 0));
        assert_eq!(Game::legal_moves(&g), vec![Move::Next]);
        // apel: caller are mai puține → 1; pierde → adversarul 2
        let mut g = Tongits::with_seed(4);
        g.set_hands_for_test(vec![c(2, 'h')], vec![c(9, 'h'), c(9, 'd')]);
        g.melds = vec![Meld {
            cards: vec![c(5, 'h'), c(5, 'd'), c(5, 'c')],
            owner: Player::One,
        }];
        g.phase = Phase::Turn;
        g.side = Player::One;
        g.first_turn = false;
        assert!(Game::legal_moves(&g).contains(&Move::Call));
        g.apply(&Move::Call);
        assert_eq!(Game::legal_moves(&g), vec![Move::Fight, Move::Fold]);
        g.apply(&Move::Fight);
        assert_eq!(Game::legal_moves(&g), vec![Move::Reveal]);
        g.apply(&Move::Reveal);
        assert_eq!(g.points(), (1, 0));
        // egalitate → câștigă cel care a luptat (2 p.)
        let mut g = Tongits::with_seed(4);
        g.set_hands_for_test(vec![c(9, 'h')], vec![c(9, 'd')]);
        g.melds = vec![Meld {
            cards: vec![c(5, 'h'), c(5, 'd'), c(5, 'c')],
            owner: Player::One,
        }];
        g.phase = Phase::Turn;
        g.side = Player::One;
        g.apply(&Move::Call);
        g.apply(&Move::Fight);
        g.apply(&Move::Reveal);
        assert_eq!(g.points(), (0, 2));
        // renunț → 1 p. pentru cel care a chemat
        let mut g = Tongits::with_seed(4);
        g.set_hands_for_test(vec![c(9, 'h')], vec![c(9, 'd')]);
        g.melds = vec![Meld {
            cards: vec![c(5, 'h'), c(5, 'd'), c(5, 'c')],
            owner: Player::One,
        }];
        g.phase = Phase::Turn;
        g.side = Player::One;
        g.apply(&Move::Call);
        g.apply(&Move::Fold);
        assert_eq!(g.points(), (1, 0));
    }

    #[test]
    fn call_requires_own_exposed_meld() {
        let mut g = Tongits::with_seed(5);
        g.phase = Phase::Turn;
        g.side = Player::One;
        assert!(!Game::legal_moves(&g).contains(&Move::Call));
    }

    #[test]
    fn cards_are_conserved_in_random_play() {
        let mut g = Tongits::with_seed(6);
        let mut rng = Rng::new(5);
        for _ in 0..400 {
            let mv = Game::legal_moves(&g);
            if mv.is_empty() {
                break;
            }
            g.apply(&mv[rng.below(mv.len())]);
            if g.phase == Phase::HandOver || g.phase == Phase::Over {
                continue;
            }
            let total = g.stock.len()
                + g.discard.len()
                + g.hands[0].len()
                + g.hands[1].len()
                + g.melds.iter().map(|m| m.cards.len()).sum::<usize>();
            assert_eq!(total, 52, "{}", g.last);
        }
    }

    #[test]
    fn render_hides_opponent_and_ai_finishes() {
        let g = Tongits::with_seed(7);
        let r = Game::render(&g);
        for &x in &g.hands[1] {
            assert!(!r.contains(&format!("[{}]", std_text(x))), "{r}");
        }
        for seed in 1..3u64 {
            let mut g = Tongits::with_seed(seed);
            let mut n = 0;
            while g.outcome().is_none() && n < 20000 {
                let m = polytabletop_core::ai::best_move(&g, n).unwrap();
                g.apply(&m);
                n += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed}");
        }
    }
}

#[cfg(test)]
mod a3_audit {
    use super::*;

    #[test]
    fn ai_ignores_the_opponent_hand_and_stock_at_every_level() {
        crate::p3_comun::ai_blind_check(
            "tongits",
            Tongits::with_seed,
            |g, me, s| {
                let h = 1 - me.idx();
                crate::p3_comun::mix_unseen(&mut g.hands[h], &mut g.stock, 0, s);
                sort_cards(&mut g.hands[h]);
            },
            6,
        );
    }
}
