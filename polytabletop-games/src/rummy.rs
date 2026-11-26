//! Remi / Rummy cu 13 cărți, 2 jucători, un pachet de 52 de cărți (fără
//! jokeri), scris doar din regulile publice.
//!
//! * Fiecare primește 13 cărți; una se întoarce pe talon (grămada de
//!   aruncări), restul formează stocul (25 de cărți).
//! * La rând: tragi o carte din stoc (`stoc`) sau iei cartea de deasupra
//!   talonului (`talon`), apoi arunci o carte (`Ah`, `10c`, … — rang + culoare
//!   c/d/h/s). Cartea luată din talon nu poate fi aruncată în aceeași tură.
//! * Închidere (`inchide-Ah`): arunci o carte, iar cele 13 rămase trebuie să se
//!   împartă în combinații: **secvențe** (≥ 3 cărți consecutive de aceeași
//!   culoare; asul e mic sau mare, fără ocolire K-A-2) și **seturi** (3–4 cărți
//!   de același rang, culori diferite), cu cel puțin o secvență. Jocul
//!   găsește singur aranjarea (căutare exactă pe măști de biți); mutarea
//!   `inchide-…` apare doar dacă o aranjare există.
//! * Punctaj: cărțile rămase în afara combinațiilor (după cea mai bună
//!   aranjare) valorează 2–10 = cifra, J/Q/K/A = 10, maxim 80. Cine închide
//!   primește 0, adversarul își adaugă punctele. Dacă stocul se golește fără
//!   închidere, mâna se termină și ambii își adaugă punctele proprii.
//! * Meciul se joacă până când un jucător ajunge la 101 puncte de penalizare;
//!   câștigă cine are mai puține (egal = remiză). Punctele se adaugă la mutarea
//!   forțată `urmatoarea` care urmează după fiecare mână.
//!
//! Informație ascunsă: starea ține stocul și ambele mâini. `render()` arată
//! mâna lui Jucător 1 (cu aranjarea automată) și doar *numărul* de cărți ale
//! adversarului; la sfârșitul mâinii se dezvăluie ambele mâini. AI-ul este
//! `AlphaBeta(1)` peste `evaluate()`, care folosește doar mâna proprie,
//! talonul public și scorurile; cartea trasă din stoc este ignorată și
//! înlocuită cu o valoare medie, iar punctajul mâinii se aplică abia la
//! mutarea `urmatoarea`. Limitări: nu ține minte ce a luat adversarul din
//! talon și nu estimează ce combinații caută el; joacă „cea mai mică
//! penalizare” (puncte libere, perechi și apropieri în aceeași culoare).

use crate::cards::Viewer;
use crate::cards52::{join_symbols, shuffle, std_deck, std_rank, std_suit, std_symbol, std_text};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const TARGET: u16 = 101;
const CAP: u16 = 80;

pub fn card_pts(c: u8) -> u16 {
    let r = std_rank(c);
    if r <= 8 {
        r as u16 + 2
    } else {
        10
    }
}

/// All melds possible inside `cards` as (mask over indices, is_run).
pub fn melds(cards: &[u8]) -> Vec<(u16, bool)> {
    let n = cards.len();
    let mut out = Vec::new();
    // sets
    for r in 0..13u8 {
        let idx: Vec<usize> = (0..n).filter(|&i| std_rank(cards[i]) == r).collect();
        let k = idx.len();
        for a in 0..k {
            for b in a + 1..k {
                for c in b + 1..k {
                    out.push(((1u16 << idx[a]) | (1 << idx[b]) | (1 << idx[c]), false));
                    for d in c + 1..k {
                        out.push((
                            (1u16 << idx[a]) | (1 << idx[b]) | (1 << idx[c]) | (1 << idx[d]),
                            false,
                        ));
                    }
                }
            }
        }
    }
    // runs: positions 0 = low ace, 1..=12 = 2..K, 13 = high ace
    for s in 0..4u8 {
        let mut pos: [Option<usize>; 14] = [None; 14];
        for (i, &card) in cards.iter().enumerate() {
            if std_suit(card) != s {
                continue;
            }
            let r = std_rank(card) as usize;
            if r == 12 {
                pos[0] = Some(i);
                pos[13] = Some(i);
            } else {
                pos[r + 1] = Some(i);
            }
        }
        for start in 0..14usize {
            let mut mask = 0u16;
            for (len, p) in (start..14).enumerate() {
                match pos[p] {
                    Some(i) => mask |= 1 << i,
                    None => break,
                }
                if len + 1 >= 3 && !(start == 0 && p == 13) {
                    out.push((mask, true));
                }
            }
        }
    }
    out
}

fn dw_rec(mask: u16, cards: &[u8], ms: &[(u16, bool)], memo: &mut [i16]) -> u16 {
    if mask == 0 {
        return 0;
    }
    if memo[mask as usize] >= 0 {
        return memo[mask as usize] as u16;
    }
    let low = mask.trailing_zeros() as usize;
    let rest = mask & !(1 << low);
    let mut best = card_pts(cards[low]) + dw_rec(rest, cards, ms, memo);
    for &(m, _) in ms {
        if m & (1 << low) != 0 && m & mask == m {
            best = best.min(dw_rec(mask & !m, cards, ms, memo));
        }
    }
    memo[mask as usize] = best as i16;
    best
}

/// Minimum number of points left outside melds over all arrangements.
pub fn min_deadwood(cards: &[u8]) -> u16 {
    assert!(cards.len() <= 14);
    let ms = melds(cards);
    let mut memo = vec![-1i16; 1 << cards.len()];
    let full = ((1u32 << cards.len()) - 1) as u16;
    dw_rec(full, cards, &ms, &mut memo)
}

fn close_rec(mask: u16, run: bool, ms: &[(u16, bool)], memo: &mut [u8]) -> bool {
    if mask == 0 {
        return run;
    }
    let key = (mask as usize) * 2 + run as usize;
    if memo[key] != 0 {
        return memo[key] == 2;
    }
    let low = mask.trailing_zeros() as usize;
    let mut ok = false;
    for &(m, is_run) in ms {
        if m & (1 << low) != 0 && m & mask == m && close_rec(mask & !m, run || is_run, ms, memo) {
            ok = true;
            break;
        }
    }
    memo[key] = if ok { 2 } else { 1 };
    ok
}

/// Can all the cards be split into melds with at least one run?
pub fn can_close(cards: &[u8]) -> bool {
    if cards.is_empty() || cards.len() > 14 {
        return false;
    }
    let ms = melds(cards);
    let mut memo = vec![0u8; (1 << cards.len()) * 2];
    let full = ((1u32 << cards.len()) - 1) as u16;
    close_rec(full, false, &ms, &mut memo)
}

/// A best arrangement: the melds (as card lists) and the loose cards.
pub fn arrange(cards: &[u8]) -> (Vec<Vec<u8>>, Vec<u8>) {
    let ms = melds(cards);
    let mut memo = vec![-1i16; 1 << cards.len()];
    let full = ((1u32 << cards.len()) - 1) as u16;
    let mut mask = full;
    let mut groups = Vec::new();
    let mut loose = Vec::new();
    while mask != 0 {
        let cur = dw_rec(mask, cards, &ms, &mut memo);
        let low = mask.trailing_zeros() as usize;
        let rest = mask & !(1 << low);
        if cur == card_pts(cards[low]) + dw_rec(rest, cards, &ms, &mut memo) {
            loose.push(cards[low]);
            mask = rest;
            continue;
        }
        let m = ms
            .iter()
            .find(|&&(m, _)| {
                m & (1 << low) != 0
                    && m & mask == m
                    && dw_rec(mask & !m, cards, &ms, &mut memo) == cur
            })
            .expect("a meld reaches the optimum")
            .0;
        groups.push(
            (0..cards.len())
                .filter(|&i| m >> i & 1 == 1)
                .map(|i| cards[i])
                .collect(),
        );
        mask &= !m;
    }
    (groups, loose)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Draw,
    Discard,
    HandOver,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum HandEnd {
    Closed(Player),
    StockOut,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Stock,
    Pile,
    Discard(u8),
    Close(u8),
    Next,
}

#[derive(Clone)]
pub struct Rummy {
    rng: Rng,
    hands: [Vec<u8>; 2],
    stock: Vec<u8>,
    pile: Vec<u8>,
    side: Player,
    phase: Phase,
    taken: Option<u8>,
    drawn_stock: Option<u8>,
    pen: [u16; 2],
    end: Option<HandEnd>,
    dealer: Player,
    over: bool,
    hand_no: u16,
    viewer: Viewer,
}

impl Default for Rummy {
    fn default() -> Self {
        Self::new()
    }
}

impl Rummy {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Rummy {
            rng: Rng::new(seed),
            hands: [Vec::new(), Vec::new()],
            stock: Vec::new(),
            pile: Vec::new(),
            side: Player::One,
            phase: Phase::Draw,
            taken: None,
            drawn_stock: None,
            pen: [0; 2],
            end: None,
            dealer: Player::Two,
            over: false,
            hand_no: 0,
            viewer: Viewer::Fixed(Player::One),
        };
        g.deal();
        g
    }

    pub fn penalties(&self) -> (u16, u16) {
        (self.pen[0], self.pen[1])
    }

    fn deal(&mut self) {
        let mut deck = std_deck(&mut self.rng);
        shuffle(&mut deck, &mut self.rng);
        self.hands = [deck[0..13].to_vec(), deck[13..26].to_vec()];
        self.pile = vec![deck[26]];
        self.stock = deck[27..].to_vec();
        self.phase = Phase::Draw;
        self.taken = None;
        self.drawn_stock = None;
        self.end = None;
        self.hand_no += 1;
        self.side = self.dealer.other();
    }
}

/// Heuristic potential of a (13-card) hand: higher is better.
fn potential(cards: &[u8]) -> i32 {
    let (_, loose) = arrange(cards);
    let dead: i32 = loose.iter().map(|&c| card_pts(c) as i32).sum();
    let mut partial = 0;
    for (i, &a) in loose.iter().enumerate() {
        let linked = loose.iter().enumerate().any(|(j, &b)| {
            if i == j {
                return false;
            }
            let same_rank = std_rank(a) == std_rank(b);
            let dr = (std_rank(a) as i32 - std_rank(b) as i32).abs();
            same_rank || (std_suit(a) == std_suit(b) && (dr == 1 || dr == 2))
        });
        if linked {
            partial += 1;
        }
    }
    -2 * dead + 5 * partial
}

impl Game for Rummy {
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
        match self.phase {
            Phase::Draw => {
                let mut v = Vec::new();
                if !self.stock.is_empty() {
                    v.push(Move::Stock);
                }
                if !self.pile.is_empty() {
                    v.push(Move::Pile);
                }
                v
            }
            Phase::Discard => {
                let hand = &self.hands[self.side.idx()];
                let mut v = Vec::new();
                for (i, &c) in hand.iter().enumerate() {
                    if Some(c) == self.taken {
                        continue;
                    }
                    v.push(Move::Discard(c));
                    let rest: Vec<u8> = hand
                        .iter()
                        .enumerate()
                        .filter(|&(j, _)| j != i)
                        .map(|(_, &x)| x)
                        .collect();
                    if can_close(&rest) {
                        v.push(Move::Close(c));
                    }
                }
                v
            }
            Phase::HandOver => vec![Move::Next],
        }
    }

    fn apply(&mut self, m: &Move) {
        let me = self.side;
        match *m {
            Move::Stock => {
                let c = self.stock.pop().expect("stock");
                self.hands[me.idx()].push(c);
                self.drawn_stock = Some(c);
                self.taken = None;
                self.phase = Phase::Discard;
            }
            Move::Pile => {
                let c = self.pile.pop().expect("pile");
                self.hands[me.idx()].push(c);
                self.taken = Some(c);
                self.drawn_stock = None;
                self.phase = Phase::Discard;
            }
            Move::Discard(c) | Move::Close(c) => {
                let h = &mut self.hands[me.idx()];
                let pos = h.iter().position(|&x| x == c).expect("card in hand");
                h.remove(pos);
                self.pile.push(c);
                self.drawn_stock = None;
                self.taken = None;
                if matches!(m, Move::Close(_)) {
                    self.end = Some(HandEnd::Closed(me));
                    self.phase = Phase::HandOver;
                } else if self.stock.is_empty() {
                    self.end = Some(HandEnd::StockOut);
                    self.phase = Phase::HandOver;
                } else {
                    self.side = me.other();
                    self.phase = Phase::Draw;
                }
            }
            Move::Next => {
                let dw = |g: &Rummy, p: Player| min_deadwood(&g.hands[p.idx()]).min(CAP);
                match self.end {
                    Some(HandEnd::Closed(w)) => {
                        let l = w.other();
                        self.pen[l.idx()] += dw(self, l);
                    }
                    Some(HandEnd::StockOut) => {
                        for p in [Player::One, Player::Two] {
                            self.pen[p.idx()] += dw(self, p);
                        }
                    }
                    None => {}
                }
                if self.pen[0] >= TARGET || self.pen[1] >= TARGET {
                    self.over = true;
                } else {
                    self.dealer = self.dealer.other();
                    self.deal();
                }
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

    /// From `p`'s own hand and public data only (the card just drawn from the
    /// stock is replaced by an average gain).
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let base = (self.pen[1 - me] as i32 - self.pen[me] as i32) * 5;
        if let Some(e) = self.end {
            return base
                + match e {
                    HandEnd::Closed(w) if w == p => 3000,
                    HandEnd::Closed(_) => -3000,
                    HandEnd::StockOut => 0,
                };
        }
        let mut hand = self.hands[me].clone();
        let mut bonus = 0;
        if let Some(d) = self.drawn_stock {
            if self.side == p {
                if let Some(pos) = hand.iter().position(|&c| c == d) {
                    hand.remove(pos);
                    bonus = 15;
                }
            }
        }
        if hand.len() == 14 {
            // best discard (the card taken from the pile cannot go back)
            let mut best = i32::MIN;
            for i in 0..14 {
                if Some(hand[i]) == self.taken {
                    continue;
                }
                let rest: Vec<u8> = hand
                    .iter()
                    .enumerate()
                    .filter(|&(j, _)| j != i)
                    .map(|(_, &x)| x)
                    .collect();
                if can_close(&rest) {
                    return base + 3000;
                }
                best = best.max(potential(&rest));
            }
            base + best
        } else {
            base + potential(&hand) + bonus
        }
    }

    fn render(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            "Remi (13 cărți) — mâna {}, penalizări (până la {TARGET}): J1 {} – J2 {}\n",
            self.hand_no, self.pen[0], self.pen[1]
        ));
        let v = self.viewer.who(self.side);
        s.push_str(&format!(
            "Stoc: {} cărți | Talon (deasupra): {} | Jucător {} (ascuns) are {} cărți\n",
            self.stock.len(),
            self.pile
                .last()
                .map(|&c| std_symbol(c))
                .unwrap_or("—".into()),
            v.other().idx() + 1,
            self.hands[v.other().idx()].len()
        ));
        let mut h = self.hands[v.idx()].clone();
        h.sort_by_key(|&c| (std_suit(c), std_rank(c)));
        let (groups, loose) = arrange(&h);
        s.push_str(&format!(
            "Mâna ta (Jucător {}): {}\n",
            v.idx() + 1,
            join_symbols(&h, std_symbol)
        ));
        let g: Vec<String> = groups
            .iter()
            .map(|g| format!("[{}]", join_symbols(g, std_symbol)))
            .collect();
        s.push_str(&format!(
            "Aranjare automată: {} | libere: {} ({} p)\n",
            if g.is_empty() {
                "—".into()
            } else {
                g.join(" ")
            },
            join_symbols(&loose, std_symbol),
            loose.iter().map(|&c| card_pts(c)).sum::<u16>()
        ));
        match (self.phase, self.over) {
            (Phase::HandOver, over) => {
                s.push_str(&format!(
                    "Mâna s-a încheiat ({}). Jucător {} avea: {} (puncte libere: {})\n",
                    match self.end {
                        Some(HandEnd::Closed(w)) => format!("a închis Jucător {}", w.idx() + 1),
                        _ => "stocul s-a golit".into(),
                    },
                    v.other().idx() + 1,
                    join_symbols(&self.hands[v.other().idx()], std_symbol),
                    min_deadwood(&self.hands[v.other().idx()]).min(CAP)
                ));
                if over {
                    s.push_str("Meci încheiat.\n");
                } else {
                    s.push_str(
                        "Scrie «urmatoarea» pentru a adăuga punctele și a împărți din nou.\n",
                    );
                }
            }
            (Phase::Draw, _) => s.push_str(&format!(
                "La mutare: Jucător {} — trage din «stoc» sau ia de pe «talon»\n",
                self.side.idx() + 1
            )),
            (Phase::Discard, _) => {
                s.push_str(&format!(
                "La mutare: Jucător {} — aruncă o carte (ex. «{}») sau închide cu «inchide-…»\n",
                self.side.idx() + 1,
                self.hands[self.side.idx()].first().map(|&c| std_text(c)).unwrap_or_default()
            ))
            }
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Stock => "stoc".into(),
            Move::Pile => "talon".into(),
            Move::Discard(c) => std_text(*c),
            Move::Close(c) => format!("inchide-{}", std_text(*c)),
            Move::Next => "urmatoarea".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cs(s: &str) -> Vec<u8> {
        s.split_whitespace()
            .map(|t| {
                let (r, su) = t.split_at(t.len() - 1);
                let r = [
                    "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
                ]
                .iter()
                .position(|&x| x == r)
                .unwrap() as u8;
                "cdhs".find(su).unwrap() as u8 * 13 + r
            })
            .collect()
    }

    #[test]
    fn card_points_scale() {
        assert_eq!(card_pts(cs("2c")[0]), 2);
        assert_eq!(card_pts(cs("9c")[0]), 9);
        assert_eq!(card_pts(cs("10c")[0]), 10);
        assert_eq!(card_pts(cs("Kd")[0]), 10);
        assert_eq!(card_pts(cs("Ah")[0]), 10);
    }

    #[test]
    fn runs_and_sets_are_found() {
        let h = cs("3h 4h 5h 9c 9d 9s");
        let ms = melds(&h);
        assert!(ms.iter().any(|&(m, r)| m == 0b000111 && r));
        assert!(ms.iter().any(|&(m, r)| m == 0b111000 && !r));
        assert_eq!(min_deadwood(&h), 0);
        assert!(can_close(&h));
    }

    #[test]
    fn ace_is_low_or_high_but_never_wraps() {
        assert!(can_close(&cs("Ah 2h 3h")));
        assert!(can_close(&cs("Qh Kh Ah")));
        assert!(!can_close(&cs("Kh Ah 2h")));
        assert!(!can_close(&cs("Qh Kh Ah 2h"))); // 4 cards: no wrap, 2h stays loose
        assert_eq!(min_deadwood(&cs("Qh Kh Ah 2h")), 2);
        assert_eq!(min_deadwood(&cs("Kh Ah 2h")), 10 + 10 + 2);
    }

    #[test]
    fn needs_at_least_one_run_to_close() {
        // thirteen cards in sets only (3+3+3+4): not closable, deadwood 0
        let h = cs("2c 2d 2h 5c 5d 5h 9c 9d 9h Kc Kd Kh Ks");
        assert_eq!(min_deadwood(&h), 0);
        assert!(!can_close(&h));
        // 3 sets + one run
        let h = cs("2c 2d 2h 5c 5d 5h 9c 9d 9h 3s 4s 5s 6s");
        assert!(can_close(&h));
    }

    #[test]
    fn optimal_arrangement_prefers_the_cheapest_loose_cards() {
        // 7h could join the run 5h6h7h or the set 7h7c7d: both fine; but with 8h
        // extra, the best split leaves less.
        let h = cs("5h 6h 7h 8h 7c 7d 2s");
        // runs 5-6-7-8h + ... set 7c 7d needs a third 7: so best = run 5-8h, loose 7c 7d 2s=16
        // alternative: set 7h7c7d + loose 5h 6h 8h 2s = 21 -> run wins
        assert_eq!(min_deadwood(&h), 7 + 7 + 2);
        let (groups, loose) = arrange(&h);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].len(), 4);
        assert_eq!(loose.len(), 3);
        // a card can only be in one meld: run 6-7-8h leaves 7c 7d, set 7h7c7d leaves 6h 8h
        let h = cs("6h 7h 8h 7c 7d");
        assert_eq!(min_deadwood(&h), 14);
    }

    #[test]
    fn deadwood_example_from_scores() {
        let h = cs("3h 4h 5h 9c 9d 9s Kc 2d");
        assert_eq!(min_deadwood(&h), 12);
        let (groups, loose) = arrange(&h);
        assert_eq!(groups.len(), 2);
        assert_eq!(loose.len(), 2);
    }

    #[test]
    fn closing_move_is_offered_only_when_valid() {
        let mut g = Rummy::with_seed(1);
        g.hands[0] = cs("3h 4h 5h 6h 9c 9d 9s 2c 2d 2h Jc Jd Jh 7s");
        g.phase = Phase::Discard;
        g.side = Player::One;
        let mv = g.legal_moves();
        // throwing the 7s closes (3-4-5-6 run, 9s set, 2s set, J set)
        assert!(mv.contains(&Move::Close(cs("7s")[0])));
        assert!(mv.contains(&Move::Discard(cs("7s")[0])));
        assert!(!mv.contains(&Move::Close(cs("2c")[0])));
        let mut c = g.clone();
        c.apply(&Move::Close(cs("7s")[0]));
        assert_eq!(c.legal_moves(), vec![Move::Next]);
        // the opponent's loose cards are scored on `urmatoarea`
        c.hands[1] = cs("Ac 2d 5s 9h Kd 3c 7d 8d 10h Qs 4c 6d 2s");
        let before = c.pen;
        assert_eq!(before, [0, 0]);
        c.apply(&Move::Next);
        assert_eq!(c.pen[0], 0);
        assert!(c.pen[1] > 0 && c.pen[1] <= CAP);
    }

    #[test]
    fn taken_pile_card_cannot_be_discarded_at_once() {
        let mut g = Rummy::with_seed(2);
        let top = *g.pile.last().unwrap();
        g.side = Player::One;
        g.apply(&Move::Pile);
        let mv = g.legal_moves();
        assert!(!mv.contains(&Move::Discard(top)));
        assert_eq!(g.hands[0].len(), 14);
    }

    #[test]
    fn stock_out_ends_the_hand_and_penalises_both() {
        let mut g = Rummy::with_seed(3);
        g.stock.truncate(1);
        g.apply(&Move::Stock);
        let c = g.hands[0][0];
        g.apply(&Move::Discard(c));
        assert_eq!(g.legal_moves(), vec![Move::Next]);
        g.apply(&Move::Next);
        assert!(g.pen[0] > 0 && g.pen[1] > 0);
    }

    #[test]
    fn deal_is_a_partition() {
        let g = Rummy::with_seed(5);
        let mut all: Vec<u8> = g.hands[0]
            .iter()
            .chain(&g.hands[1])
            .chain(&g.stock)
            .chain(&g.pile)
            .copied()
            .collect();
        all.sort_unstable();
        assert_eq!(all, (0..52).collect::<Vec<u8>>());
        assert_eq!(g.stock.len(), 25);
    }

    #[test]
    fn render_hides_the_opponent() {
        let g = Rummy::with_seed(8);
        let r = Game::render(&g);
        assert!(r.contains("Jucător 2 (ascuns) are 13 cărți"));
        assert!(r.contains("Aranjare automată"));
    }

    #[test]
    fn viewer_controls_whose_hand_is_shown() {
        let shown =
            |g: &Rummy, r: &str, p: usize| g.hands[p].iter().all(|&c| r.contains(&std_symbol(c)));
        let hidden =
            |g: &Rummy, r: &str, p: usize| g.hands[p].iter().all(|&c| !r.contains(&std_symbol(c)));
        let mut g = Rummy::with_seed(8);
        Game::set_viewer(&mut g, None);
        let mut movers = [false; 2];
        for _ in 0..2 {
            let tm = g.to_move().idx();
            movers[tm] = true;
            let r = Game::render(&g);
            assert!(shown(&g, &r, tm) && hidden(&g, &r, 1 - tm), "{r}");
            g.apply(&Move::Stock);
            let m = Game::legal_moves(&g)
                .into_iter()
                .find(|m| matches!(m, Move::Discard(_)))
                .unwrap();
            g.apply(&m);
        }
        assert!(movers[0] && movers[1]);
        Game::set_viewer(&mut g, Some(Player::Two));
        let r = Game::render(&g);
        assert!(shown(&g, &r, 1) && hidden(&g, &r, 0), "{r}");
        Game::set_viewer(&mut g, Some(Player::One));
        let r = Game::render(&g);
        assert!(shown(&g, &r, 0) && hidden(&g, &r, 1), "{r}");
    }

    #[test]
    fn evaluate_ignores_hidden_cards() {
        let g = Rummy::with_seed(13);
        let mut h = g.clone();
        std::mem::swap(&mut h.hands[1][0], &mut h.stock[3]);
        assert_eq!(g.evaluate(Player::One), h.evaluate(Player::One));
        // the unknown card drawn from the stock does not influence it either
        let mut a = g.clone();
        let mut b = g.clone();
        b.stock.reverse();
        a.apply(&Move::Stock);
        b.apply(&Move::Stock);
        assert_eq!(a.evaluate(Player::One), b.evaluate(Player::One));
    }

    #[test]
    fn random_matches_finish() {
        for seed in 1..=20u64 {
            let mut g = Rummy::with_seed(seed);
            let mut rng = Rng::new(seed * 3 + 1);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty(), "{}", Game::render(&g));
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 20000);
                let all = g.hands[0].len() + g.hands[1].len() + g.stock.len() + g.pile.len();
                assert!(all == 52, "{all}");
            }
        }
    }

    #[test]
    fn ai_beats_a_random_mover_mostly() {
        use polytabletop_core::GameObj;
        let n = 6;
        let mut wins = 0;
        for seed in 1..=n {
            let mut g: Box<dyn GameObj> = Box::new(Rummy::with_seed(seed));
            let mut rng = Rng::new(seed + 9);
            while g.outcome().is_none() {
                let mv = if g.to_move() == Player::One {
                    g.ai_move(rng.next_u64()).unwrap()
                } else {
                    let m = g.legal_moves();
                    m[rng.below(m.len())].clone()
                };
                g.play(&mv).unwrap();
            }
            if g.outcome() == Some(Outcome::Win(Player::One)) {
                wins += 1;
            }
        }
        assert!(wins * 10 >= n * 7, "AI won only {wins}/{n}");
    }
}
