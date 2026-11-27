//! Gin Rummy, 2 jucători, din regulile publice standard.
//!
//! * 52 de cărți, 10 fiecăruia; următoarea carte se întoarce (talon), restul formează stocul.
//!   Valori: A=1, 2–10 cât scrie, J/Q/K=10. Combinații: seturi (3–4 cărți de aceeași valoare) și secvențe
//!   (3+ cărți consecutive din aceeași culoare; A doar jos: A-2-3, fără K-A-2). Cărțile care nu intră
//!   în combinații sunt „deadwood”; aranjarea mâinii e automată (deadwood minim).
//! * Prima tură: non-dealerul poate lua cartea întoarsă (`talon`) sau `pas`; apoi dealerul la fel; dacă
//!   ambii dau pas, non-dealerul trage din stoc. Apoi, la rând: tragi (`stoc` sau `talon`), apoi arunci
//!   (`dis 7h`) — nu poți arunca chiar cartea luată din talon în acea tură.
//! * Poți **bate** (`bat 7h`: arunci cartea și închizi) dacă deadwood-ul rămas e cel mult 10, sau
//!   **gin** (`gin 7h`) dacă e 0. După bătaie (nu și după gin) adversarul poate lipi cărți pe
//!   combinațiile bătătorului (`adauga 5h`), apoi `gata`.
//! * Punctaj: bătătorul cu deadwood mai mic ia diferența; dacă adversarul are deadwood egal sau mai mic
//!   („undercut”) el ia diferența + 25; gin = 25 + deadwood-ul adversarului. Se joacă până la 100.
//! * Când în stoc rămân două cărți și cel care a tras ultima carte aruncă fără să bată, mâna e remiză.
//!
//! Omise: „big gin” (11 cărți) și bonusurile finale de joc/cutie (câștigă primul la 100; cel mult 20 de mâini, apoi scorul mai mare).
//! Informație ascunsă: vezi `p3_comun`; AI-ul știe doar ce a luat adversarul din talon și ce s-a aruncat.
#![allow(
    clippy::needless_range_loop,
    clippy::collapsible_if,
    clippy::collapsible_match,
    clippy::manual_is_multiple_of,
    clippy::manual_range_patterns,
    clippy::type_complexity
)]

use crate::cards::{full_deck, handover_banner, join_cards, shuffle, Card, Viewer};
use crate::p3_comun::{honest_eval, Honest};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const TARGET: i32 = 100;
const MAX_HANDS: u32 = 20;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    FirstNon,
    FirstDealer,
    Draw,
    Discard,
    Layoff,
    Gin,
    HandOver,
    Over,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    TakeUp,
    Pass,
    Stock,
    Discard(Card),
    Knock(Card),
    Gin(Card),
    Layoff(Card, u8),
    Done,
    Show,
    Next,
}

#[derive(Clone)]
pub struct GinRummy {
    rng: Rng,
    stock: Vec<Card>,
    discard: Vec<Card>,
    hands: [Vec<Card>; 2],
    known: [Vec<Card>; 2],
    dealer: Player,
    side: Player,
    knocker: Player,
    kmelds: Vec<Vec<Card>>,
    kheld: Vec<Card>,
    kdead: i32,
    took: Option<Card>,
    no_take: bool,
    phase: Phase,
    scores: [i32; 2],
    hands_played: u32,
    delta: [i32; 2],
    fresh: [u8; 2],
    prev: Player,
    log: String,
    viewer: Viewer,
}

impl Default for GinRummy {
    fn default() -> Self {
        Self::new()
    }
}

fn gr(c: Card) -> u8 {
    if c.rank == 14 {
        1
    } else {
        c.rank
    }
}

pub fn val(c: Card) -> i32 {
    match c.rank {
        14 => 1,
        11..=13 => 10,
        r => r as i32,
    }
}

fn candidate_melds(h: &[Card]) -> Vec<u16> {
    let n = h.len();
    let mut out = Vec::new();
    // seturi
    for r in 2..=14u8 {
        let idx: Vec<usize> = (0..n).filter(|&i| h[i].rank == r).collect();
        if idx.len() >= 3 {
            let mut m3 = Vec::new();
            for a in 0..idx.len() {
                for b in a + 1..idx.len() {
                    for c in b + 1..idx.len() {
                        m3.push((1u16 << idx[a]) | (1 << idx[b]) | (1 << idx[c]));
                    }
                }
            }
            out.extend(m3);
            if idx.len() == 4 {
                out.push(idx.iter().fold(0u16, |m, &i| m | (1 << i)));
            }
        }
    }
    // secvențe
    for s in 0..4u8 {
        let mut by_rank: [Option<usize>; 15] = [None; 15];
        for i in 0..n {
            if h[i].suit == s {
                by_rank[gr(h[i]) as usize] = Some(i);
            }
        }
        for lo in 1..=13usize {
            let mut mask = 0u16;
            let mut len = 0;
            for r in lo..=13usize {
                match by_rank[r] {
                    Some(i) => {
                        mask |= 1 << i;
                        len += 1;
                        if len >= 3 {
                            out.push(mask);
                        }
                    }
                    None => break,
                }
            }
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

fn solve(mask: u16, n: usize, vals: &[i32], melds: &[u16], memo: &mut [i32]) -> i32 {
    let full = ((1u32 << n) - 1) as u16;
    if mask == full {
        return 0;
    }
    if memo[mask as usize] >= 0 {
        return memo[mask as usize];
    }
    let i = (!mask).trailing_zeros() as usize;
    let mut best = vals[i] + solve(mask | (1 << i), n, vals, melds, memo);
    for &m in melds {
        if m & (1 << i) != 0 && m & mask == 0 {
            best = best.min(solve(mask | m, n, vals, melds, memo));
        }
    }
    memo[mask as usize] = best;
    best
}

/// Deadwood minim și combinațiile alese.
pub fn best_partition(h: &[Card]) -> (i32, Vec<Vec<Card>>) {
    let n = h.len();
    if n == 0 {
        return (0, Vec::new());
    }
    let vals: Vec<i32> = h.iter().map(|&c| val(c)).collect();
    let melds = candidate_melds(h);
    let mut memo = vec![-1i32; 1 << n];
    let dw = solve(0, n, &vals, &melds, &mut memo);
    // reconstruiește
    let mut chosen: Vec<Vec<Card>> = Vec::new();
    let mut mask = 0u16;
    let full = ((1u32 << n) - 1) as u16;
    while mask != full {
        let i = (!mask).trailing_zeros() as usize;
        let cur = solve(mask, n, &vals, &melds, &mut memo);
        if cur == vals[i] + solve(mask | (1 << i), n, &vals, &melds, &mut memo) {
            mask |= 1 << i;
            continue;
        }
        let mut found = false;
        for &m in &melds {
            if m & (1 << i) != 0
                && m & mask == 0
                && cur == solve(mask | m, n, &vals, &melds, &mut memo)
            {
                chosen.push(
                    (0..n)
                        .filter(|&j| m & (1 << j) != 0)
                        .map(|j| h[j])
                        .collect(),
                );
                mask |= m;
                found = true;
                break;
            }
        }
        if !found {
            mask |= 1 << i;
        }
    }
    (dw, chosen)
}

pub fn deadwood(h: &[Card]) -> i32 {
    best_partition(h).0
}

fn extends(meld: &[Card], c: Card) -> bool {
    if meld.iter().all(|m| m.rank == meld[0].rank) {
        return c.rank == meld[0].rank && meld.len() < 4 && !meld.contains(&c);
    }
    if c.suit != meld[0].suit {
        return false;
    }
    let lo = meld.iter().map(|&m| gr(m)).min().unwrap();
    let hi = meld.iter().map(|&m| gr(m)).max().unwrap();
    (gr(c) + 1 == lo && lo > 1) || (gr(c) == hi + 1 && hi < 13)
}

/// Cartea de deadwood cu valoarea cea mai mare din partiția optimă.
fn worst_dead(h: &[Card]) -> (Card, i32) {
    let (dw, melds) = best_partition(h);
    let in_meld: Vec<Card> = melds.iter().flatten().copied().collect();
    let mut cand: Vec<Card> = h.iter().copied().filter(|c| !in_meld.contains(c)).collect();
    if cand.is_empty() {
        cand = h.to_vec();
    }
    let c = cand.into_iter().max_by_key(|&c| (val(c), c.suit)).unwrap();
    (c, dw)
}

impl GinRummy {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = GinRummy {
            rng: Rng::new(seed),
            stock: Vec::new(),
            discard: Vec::new(),
            hands: [Vec::new(), Vec::new()],
            known: [Vec::new(), Vec::new()],
            dealer: Player::Two,
            side: Player::One,
            knocker: Player::One,
            kmelds: Vec::new(),
            kheld: Vec::new(),
            kdead: 0,
            took: None,
            no_take: false,
            phase: Phase::FirstNon,
            scores: [0, 0],
            hands_played: 0,
            delta: [0, 0],
            fresh: [0, 0],
            prev: Player::One,
            log: String::new(),
            viewer: Viewer::Fixed(Player::One),
        };
        g.deal();
        g
    }

    pub fn scores(&self) -> (i32, i32) {
        (self.scores[0], self.scores[1])
    }

    fn deal(&mut self) {
        let mut d = full_deck(2);
        shuffle(&mut d, &mut self.rng);
        self.hands = [Vec::new(), Vec::new()];
        for _ in 0..10 {
            for p in 0..2 {
                self.hands[p].push(d.pop().unwrap());
            }
        }
        self.discard = vec![d.pop().unwrap()];
        self.stock = d;
        self.known = [Vec::new(), Vec::new()];
        self.kmelds.clear();
        self.took = None;
        self.no_take = false;
        self.delta = [0, 0];
        self.fresh = [0, 0];
        self.sort_hands();
        self.phase = Phase::FirstNon;
        self.side = self.dealer.other();
    }

    fn sort_hands(&mut self) {
        for p in 0..2 {
            let nf = (self.fresh[p] as usize).min(self.hands[p].len());
            let k = self.hands[p].len() - nf;
            self.hands[p][..k].sort_by_key(|c| (c.suit, c.rank));
        }
    }

    fn top(&self) -> Card {
        *self.discard.last().unwrap()
    }

    fn take_top(&mut self, p: Player) {
        let c = self.discard.pop().unwrap();
        self.hands[p.idx()].push(c);
        self.known[p.idx()].push(c);
        self.took = Some(c);
        self.phase = Phase::Discard;
        self.side = p;
    }

    fn settle(&mut self, kn: Player, other_dead: i32, gin: bool) {
        let o = kn.other();
        let (who, pts, why) = if gin {
            (kn, 25 + other_dead, "GIN")
        } else if self.kdead < other_dead {
            (kn, other_dead - self.kdead, "bătaie")
        } else {
            (o, self.kdead - other_dead + 25, "undercut")
        };
        self.scores[who.idx()] += pts;
        self.delta[who.idx()] += pts;
        self.log = format!(
            "{}: J{} ia {} puncte (deadwood bătător {}, celălalt {})",
            why,
            who.idx() + 1,
            pts,
            self.kdead,
            other_dead
        );
        self.hand_done();
    }

    fn hand_done(&mut self) {
        self.hands_played += 1;
        self.phase = if self.scores.iter().any(|&s| s >= TARGET) || self.hands_played >= MAX_HANDS {
            Phase::Over
        } else {
            Phase::HandOver
        };
    }
}

impl Game for GinRummy {
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
            Phase::HandOver => vec![Move::Next],
            Phase::Gin => vec![Move::Show],
            Phase::FirstNon | Phase::FirstDealer => vec![Move::TakeUp, Move::Pass],
            Phase::Draw => {
                let mut v = vec![Move::Stock];
                if !self.no_take {
                    v.push(Move::TakeUp);
                }
                v
            }
            Phase::Discard => {
                let h = &self.hands[self.side.idx()];
                let mut v = Vec::new();
                for (i, &c) in h.iter().enumerate() {
                    if Some(c) == self.took || h[..i].contains(&c) {
                        continue;
                    }
                    v.push(Move::Discard(c));
                    let rest: Vec<Card> = h.iter().copied().filter(|&x| x != c).collect();
                    let dw = deadwood(&rest);
                    if dw == 0 {
                        v.push(Move::Gin(c));
                    } else if dw <= 10 {
                        v.push(Move::Knock(c));
                    }
                }
                v
            }
            Phase::Layoff => {
                let h = &self.hands[self.side.idx()];
                let mut v = Vec::new();
                for &c in h {
                    for (i, m) in self.kmelds.iter().enumerate() {
                        let mv = Move::Layoff(c, i as u8 + 1);
                        if extends(m, c) && !v.contains(&mv) {
                            v.push(mv);
                        }
                    }
                }
                v.push(Move::Done);
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        self.fresh = [0, 0];
        self.prev = self.side;
        let p = self.side;
        match *m {
            Move::Next => {
                self.dealer = self.dealer.other();
                self.deal();
            }
            Move::TakeUp => {
                self.take_top(p);
                self.no_take = false;
            }
            Move::Pass => {
                if self.phase == Phase::FirstNon {
                    self.phase = Phase::FirstDealer;
                    self.side = self.dealer;
                } else {
                    self.phase = Phase::Draw;
                    self.no_take = true;
                    self.side = self.dealer.other();
                }
            }
            Move::Stock => {
                let c = self.stock.pop().unwrap();
                self.hands[p.idx()].push(c);
                self.fresh[p.idx()] = 1;
                self.took = None;
                self.no_take = false;
                self.phase = Phase::Discard;
            }
            Move::Discard(c) | Move::Knock(c) | Move::Gin(c) => {
                let pos = self.hands[p.idx()].iter().position(|&x| x == c).unwrap();
                self.hands[p.idx()].remove(pos);
                if let Some(i) = self.known[p.idx()].iter().position(|&x| x == c) {
                    self.known[p.idx()].remove(i);
                }
                self.discard.push(c);
                self.took = None;
                self.sort_hands();
                match *m {
                    Move::Knock(_) | Move::Gin(_) => {
                        let (dw, melds) = best_partition(&self.hands[p.idx()]);
                        self.knocker = p;
                        self.kdead = dw;
                        self.kheld = melds.iter().flatten().copied().collect();
                        self.kmelds = melds;
                        if matches!(m, Move::Gin(_)) {
                            self.phase = Phase::Gin;
                        } else {
                            self.phase = Phase::Layoff;
                            self.side = p.other();
                        }
                    }
                    _ => {
                        if self.stock.len() <= 2 {
                            self.log = "Stocul s-a golit: mână remiză.".into();
                            self.hand_done();
                        } else {
                            self.side = p.other();
                            self.phase = Phase::Draw;
                        }
                    }
                }
            }
            Move::Layoff(c, k) => {
                let pos = self.hands[p.idx()].iter().position(|&x| x == c).unwrap();
                self.hands[p.idx()].remove(pos);
                self.kmelds[k as usize - 1].push(c);
            }
            Move::Done => {
                let dw = deadwood(&self.hands[p.idx()]);
                let kn = self.knocker;
                self.settle(kn, dw, false);
            }
            Move::Show => {
                let kn = self.knocker;
                let dw = deadwood(&self.hands[kn.other().idx()]);
                self.settle(kn, dw, true);
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        Some(match self.scores[0].cmp(&self.scores[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        if self.phase == Phase::Over {
            return 0;
        }
        honest_eval(self, p, 10)
    }

    fn render(&self) -> String {
        let mut s = String::new();
        if self.prev != self.side && !matches!(self.phase, Phase::Over | Phase::HandOver) {
            if let Viewer::HotSeat = self.viewer {
                s.push_str(&handover_banner(self.side));
            }
        }
        s.push_str(&format!(
            "Gin Rummy — scor: J1 {}, J2 {} (până la {}) | dealer: J{}\n",
            self.scores[0],
            self.scores[1],
            TARGET,
            self.dealer.idx() + 1
        ));
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        match self.phase {
            Phase::Over => {
                s.push_str("Joc încheiat.\n");
                return s;
            }
            Phase::HandOver => {
                s.push_str(&format!(
                    "Mână încheiată. Mâini: J1 {} | J2 {}\nScrie «urmatoarea».\n",
                    join_cards(&self.hands[0]),
                    join_cards(&self.hands[1])
                ));
                return s;
            }
            _ => {}
        }
        s.push_str(&format!(
            "Talon: {} | stoc: {} cărți\n",
            self.discard
                .last()
                .map(|c| c.text())
                .unwrap_or_else(|| "—".into()),
            self.stock.len()
        ));
        if matches!(self.phase, Phase::Layoff | Phase::Gin) {
            let ms: Vec<String> = self
                .kmelds
                .iter()
                .map(|m| format!("[{}]", join_cards(m)))
                .collect();
            s.push_str(&format!(
                "J{} a bătut: combinații {} | deadwood {}\n",
                self.knocker.idx() + 1,
                ms.join(" "),
                self.kdead
            ));
            if self.phase == Phase::Gin {
                s.push_str("GIN! Scrie «arata».\n");
            }
        }
        let v = self.viewer.who(self.side);
        let mine = &self.hands[v.idx()];
        let (dw, melds) = best_partition(mine);
        let ms: Vec<String> = melds
            .iter()
            .map(|m| format!("[{}]", join_cards(m)))
            .collect();
        s.push_str(&format!(
            "Mâna ta (J{}): {} (deadwood {}{})\n",
            v.idx() + 1,
            join_cards(mine),
            dw,
            if ms.is_empty() {
                String::new()
            } else {
                format!("; combinații {}", ms.join(" "))
            }
        ));
        s.push_str(&format!(
            "J{} are {} cărți\n",
            v.other().idx() + 1,
            self.hands[v.other().idx()].len()
        ));
        if !self.known[v.other().idx()].is_empty() {
            s.push_str(&format!(
                "J{} a luat din talon: {}\n",
                v.other().idx() + 1,
                join_cards(&self.known[v.other().idx()])
            ));
        }
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::TakeUp => "talon".into(),
            Move::Pass => "pas".into(),
            Move::Stock => "stoc".into(),
            Move::Discard(c) => format!("dis {}", c.text()),
            Move::Knock(c) => format!("bat {}", c.text()),
            Move::Gin(c) => format!("gin {}", c.text()),
            Move::Layoff(c, k) => format!("adauga {} la {}", c.text(), k),
            Move::Done => "gata".into(),
            Move::Show => "arata".into(),
            Move::Next => "urmatoarea".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

impl Honest for GinRummy {
    fn determinize(&self, me: Player, rng: &mut Rng) -> Self {
        let mut g = self.clone();
        let (mi, oi) = (me.idx(), 1 - me.idx());
        let nf = (self.fresh[mi] as usize).min(self.hands[mi].len());
        let known_mine: Vec<Card> = self.hands[mi][..self.hands[mi].len() - nf].to_vec();
        let mut pool = full_deck(2);
        pool.retain(|c| {
            !known_mine.contains(c)
                && !self.discard.contains(c)
                && !self.known[oi].contains(c)
                && !self.kmelds.iter().any(|m| m.contains(c))
        });
        shuffle(&mut pool, rng);
        let mut mine = known_mine;
        for _ in 0..nf {
            mine.push(pool.pop().unwrap());
        }
        let n_opp = self.hands[oi].len();
        let mut opp: Vec<Card> = self.known[oi].clone();
        // combinațiile bătătorului sunt pe masă (publice); deadwood-ul lui rămâne necunoscut
        if matches!(self.phase, Phase::Layoff | Phase::Gin) && self.knocker.idx() == oi {
            opp = self.kheld.clone();
        }
        while opp.len() < n_opp {
            opp.push(pool.pop().unwrap());
        }
        g.hands[mi] = mine;
        g.hands[oi] = opp;
        g.stock = pool;
        g.fresh = [0, 0];
        g.rng = Rng::new(rng.next_u64());
        g
    }

    fn seed_state(&self, me: Player) -> Self {
        let mut c = self.clone();
        let nf = (c.fresh[me.idx()] as usize).min(c.hands[me.idx()].len());
        let k = c.hands[me.idx()].len() - nf;
        c.hands[me.idx()].truncate(k);
        c.fresh = [0, 0];
        c
    }

    fn forget_fresh(&mut self) {
        self.fresh = [0, 0];
    }

    fn rollout_done(&self) -> bool {
        matches!(self.phase, Phase::HandOver | Phase::Over)
    }

    fn rollout_value(&self, p: Player) -> f64 {
        (self.delta[p.idx()] - self.delta[p.other().idx()]) as f64 / 5.0
    }

    fn policy(&self, rng: &mut Rng) -> Move {
        let mv = self.legal_moves();
        if mv.len() == 1 {
            return mv[0];
        }
        let h = &self.hands[self.side.idx()];
        match self.phase {
            Phase::FirstNon | Phase::FirstDealer | Phase::Draw => {
                if !mv.contains(&Move::TakeUp) {
                    return Move::Stock;
                }
                let (_, dw_now) = worst_dead(h);
                let mut with = h.clone();
                with.push(self.top());
                let (wc, dw_with) = worst_dead(&with);
                if dw_with - val(wc) < dw_now {
                    Move::TakeUp
                } else if self.phase == Phase::Draw {
                    Move::Stock
                } else {
                    Move::Pass
                }
            }
            Phase::Discard => {
                if rng.below(100) < 6 {
                    return mv[rng.below(mv.len())];
                }
                let (c, _) = worst_dead(h);
                let d = if mv.contains(&Move::Discard(c)) {
                    c
                } else {
                    mv.iter()
                        .filter_map(|m| {
                            if let Move::Discard(x) = m {
                                Some(*x)
                            } else {
                                None
                            }
                        })
                        .max_by_key(|&x| val(x))
                        .unwrap()
                };
                let rest: Vec<Card> = h.iter().copied().filter(|&x| x != d).collect();
                let real = deadwood(&rest);
                if real == 0 && mv.contains(&Move::Gin(d)) {
                    return Move::Gin(d);
                }
                if real <= 7 && mv.contains(&Move::Knock(d)) {
                    return Move::Knock(d);
                }
                Move::Discard(d)
            }
            Phase::Layoff => {
                for &m in &mv {
                    if let Move::Layoff(..) = m {
                        return m;
                    }
                }
                Move::Done
            }
            _ => mv[0],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(rank: u8, suit: u8) -> Card {
        Card { rank, suit }
    }

    #[test]
    fn deadwood_basic() {
        // set de 7 + secvență 4-5-6 de cupe + restul
        let h = vec![
            c(7, 0),
            c(7, 1),
            c(7, 2),
            c(4, 3),
            c(5, 3),
            c(6, 3),
            c(13, 0),
            c(2, 1),
            c(14, 2),
            c(9, 1),
        ];
        assert_eq!(deadwood(&h), 10 + 2 + 1 + 9);
        // gin
        let g = vec![
            c(7, 0),
            c(7, 1),
            c(7, 2),
            c(4, 3),
            c(5, 3),
            c(6, 3),
            c(10, 0),
            c(11, 0),
            c(12, 0),
            c(13, 0),
        ];
        assert_eq!(deadwood(&g), 0);
        // As jos dar nu peste Rege
        let a = vec![c(14, 0), c(2, 0), c(3, 0)];
        assert_eq!(deadwood(&a), 0);
        let b = vec![c(12, 0), c(13, 0), c(14, 0)];
        assert_eq!(deadwood(&b), 21);
    }

    #[test]
    fn deadwood_picks_best_split() {
        // 5-6-7 de cupe poate fi secvență sau 7 intră în set cu 7,7: set 7 + 5,6 deadwood = 11, secvență = 7+7 =14
        let h = vec![c(5, 0), c(6, 0), c(7, 0), c(7, 1), c(7, 2)];
        assert_eq!(deadwood(&h), 11);
    }

    #[test]
    fn layoff_rules() {
        let run = vec![c(5, 0), c(6, 0), c(7, 0)];
        assert!(extends(&run, c(4, 0)) && extends(&run, c(8, 0)));
        assert!(!extends(&run, c(9, 0)) && !extends(&run, c(4, 1)));
        let set = vec![c(9, 0), c(9, 1), c(9, 2)];
        assert!(extends(&set, c(9, 3)) && !extends(&set, c(8, 3)));
        let ace = vec![c(14, 1), c(2, 1), c(3, 1)];
        assert!(extends(&ace, c(4, 1)) && !extends(&ace, c(13, 1)));
        let king = vec![c(11, 1), c(12, 1), c(13, 1)];
        assert!(extends(&king, c(10, 1)) && !extends(&king, c(14, 1)));
    }

    #[test]
    fn knock_scoring_and_undercut() {
        let mut g = GinRummy::with_seed(1);
        g.kdead = 4;
        g.knocker = Player::One;
        g.settle(Player::One, 9, false);
        assert_eq!(g.scores, [5, 0]);
        let mut h = GinRummy::with_seed(2);
        h.kdead = 6;
        h.settle(Player::One, 3, false);
        assert_eq!(h.scores, [0, 3 + 25]);
        let mut k = GinRummy::with_seed(3);
        k.settle(Player::Two, 12, true);
        assert_eq!(k.scores, [0, 37]);
    }

    #[test]
    fn first_turn_flow_and_no_take_after_double_pass() {
        let mut g = GinRummy::with_seed(4);
        assert_eq!(g.side, g.dealer.other());
        g.apply(&Move::Pass);
        assert_eq!(g.side, g.dealer);
        g.apply(&Move::Pass);
        assert_eq!(g.phase, Phase::Draw);
        assert_eq!(g.side, g.dealer.other());
        assert_eq!(g.legal_moves(), vec![Move::Stock]);
    }

    #[test]
    fn cannot_discard_card_just_taken() {
        let mut g = GinRummy::with_seed(5);
        let top = g.top();
        g.apply(&Move::TakeUp);
        assert_eq!(g.phase, Phase::Discard);
        assert!(!g.legal_moves().contains(&Move::Discard(top)));
        assert_eq!(g.hands[g.side.idx()].len(), 11);
    }

    #[test]
    fn knock_flow_with_layoff() {
        let mut g = GinRummy::with_seed(6);
        g.phase = Phase::Discard;
        g.side = Player::One;
        g.took = None;
        g.hands[0] = vec![
            c(7, 0),
            c(7, 1),
            c(7, 2),
            c(4, 3),
            c(5, 3),
            c(6, 3),
            c(10, 0),
            c(11, 0),
            c(12, 0),
            c(2, 1),
            c(3, 2),
        ];
        g.hands[1] = vec![
            c(7, 3),
            c(8, 3),
            c(9, 1),
            c(9, 2),
            c(13, 0),
            c(13, 1),
            c(14, 3),
            c(2, 0),
            c(3, 0),
            c(5, 2),
        ];
        let mv = g.legal_moves();
        assert!(mv.contains(&Move::Knock(c(2, 1))) || mv.contains(&Move::Knock(c(3, 2))));
        g.apply(&Move::Knock(c(3, 2)));
        assert_eq!(g.phase, Phase::Layoff);
        assert_eq!(g.side, Player::Two);
        assert_eq!(g.kdead, 2);
        // 7♣ se poate lipi pe setul de 7, 8♣? nu: secvența 4-5-6 trefla se extinde cu 7♣
        let run = g.kmelds.iter().position(|m| m.contains(&c(4, 3))).unwrap() as u8 + 1;
        assert!(g.legal_moves().contains(&Move::Layoff(c(7, 3), run)));
        g.apply(&Move::Layoff(c(7, 3), run));
        g.apply(&Move::Layoff(c(8, 3), run));
        g.apply(&Move::Done);
        assert!(matches!(g.phase, Phase::HandOver | Phase::Over));
        assert!(g.scores[0] + g.scores[1] > 0);
    }

    #[test]
    fn determinize_hides_opponent_keeps_taken_card() {
        let mut g = GinRummy::with_seed(7);
        g.apply(&Move::Pass);
        let top = g.top();
        g.apply(&Move::TakeUp);
        assert!(g.known[g.side.idx()].contains(&top));
        let opp = g.side;
        let me = opp.other();
        let mut rng = Rng::new(5);
        let d = g.determinize(me, &mut rng);
        assert!(d.hands[opp.idx()].contains(&top));
        assert_eq!(d.hands[opp.idx()].len(), 11);
        assert_eq!(d.hands[me.idx()], g.hands[me.idx()]);
        assert_eq!(d.stock.len(), g.stock.len());
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=6u64 {
            let mut g = GinRummy::with_seed(seed);
            let mut rng = Rng::new(seed + 3);
            let mut n = 0;
            while g.outcome().is_none() && n < 20_000 {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed}");
        }
    }
}

#[cfg(test)]
mod a1_blind {
    use super::*;

    #[test]
    fn ai_ignores_hidden_information_at_all_levels() {
        crate::ai_audit_a1::audit_honest("gin_rummy", GinRummy::with_seed, 1..=2, 11, 60);
    }
}
