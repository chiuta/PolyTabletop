//! Bid Whist — adaptare pentru 2 jucători, din regulile publice ale jocului în patru.
//!
//! * 54 de cărți (52 + Joker Mare `JB` + Joker Mic `JL`). Fiecare jucător primește 12 cărți, 6 merg la
//!   „kitty”, iar celelalte 24 (mâinile celorlalți doi jucători din jocul în patru) rămân deoparte,
//!   nefolosite și ascunse. Se joacă 12 levate; „book” = 6 levate.
//! * Licitație: non-dealerul spune `pas` sau `licit N` (N = 3…6 levate peste book), apoi dealerul poate da
//!   `pas` sau licita mai mult decât el. Dacă ambii dau pas, se reîmparte (pasul forțat `reimparte`). Câștigătorul licitației ia
//!   kitty-ul (are 18 cărți), aruncă 6 cărți (`dis 5h`, pe rând) și declară contractul:
//!   `sus h` (Uptown, atu cupă: As cel mai mare), `jos h` (Downtown: 2 cel mai mare, As cel mai mic),
//!   `fara sus` sau `fara jos` (fără atu).
//! * Jokerii sunt cei mai mari doi atu (Mare > Mic) în contractele cu atu. Fără atu, jokerii pot fi jucați
//!   oricând și iau levata (Mare bate Mic); un joker condus lasă ceilalți să pună orice.
//! * Licitatorul conduce; se răspunde obligatoriu la culoare; fără culoare se poate tăia sau arunca.
//! * Scor: dacă licitatorul ia cel puțin 6 + N levate primește levatele peste book; altfel pierde N.
//!   Contractele fără atu valorează dublu. Se joacă până la +7 (sau −7 = pierdere).
//!
//! Adaptarea pentru doi jucători (12 cărți fiecare, 24 deoparte) și limitele 3…6 sunt alegeri ale
//! acestei implementări. Mutări: `pas`, `licit 4`, `dis 5h`, `sus h`, `fara jos`, `Ah`, `JB`, `urmatoarea`.

use crate::cards::{handover_banner, Viewer};
use crate::cards52::{shuffle, std_rank, std_suit, std_text, SUIT_LETTERS};
use crate::p3_comun::{honest_eval, Honest};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const TARGET: i32 = 7;
const BIG: u8 = 52;
const LITTLE: u8 = 53;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Bid1,
    Bid2,
    Redeal,
    Discard,
    Declare,
    Play,
    HandOver,
    Over,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Contract {
    pub up: bool,
    pub trump: Option<u8>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Pass,
    Bid(u8),
    Discard(u8),
    Declare(Contract),
    Play(u8),
    Redeal,
    Next,
}

#[derive(Clone)]
pub struct BidWhist {
    rng: Rng,
    hands: [Vec<u8>; 2],
    kitty: Vec<u8>,
    junk: Vec<u8>,
    dealer: Player,
    side: Player,
    phase: Phase,
    first_bid: u8,
    bid: u8,
    bidder: Player,
    contract: Option<Contract>,
    trick: Vec<(Player, u8)>,
    played: Vec<u8>,
    taken: [u8; 2],
    scores: [i32; 2],
    delta: [i32; 2],
    fresh: [u8; 2],
    prev: Player,
    log: String,
    viewer: Viewer,
}

impl Default for BidWhist {
    fn default() -> Self {
        Self::new()
    }
}

fn ctext(c: u8) -> String {
    match c {
        BIG => "JB".into(),
        LITTLE => "JL".into(),
        _ => std_text(c),
    }
}

fn join(v: &[u8]) -> String {
    if v.is_empty() {
        return "—".into();
    }
    v.iter().map(|&c| ctext(c)).collect::<Vec<_>>().join(" ")
}

fn is_joker(c: u8) -> bool {
    c >= 52
}

fn sort_cards(v: &mut [u8]) {
    v.sort_by_key(|&c| {
        if is_joker(c) {
            (9u8, 0u8)
        } else {
            (std_suit(c), std_rank(c))
        }
    });
}

impl Contract {
    fn text(self) -> String {
        match self.trump {
            Some(t) => format!(
                "{} {}",
                if self.up { "sus" } else { "jos" },
                SUIT_LETTERS[t as usize]
            ),
            None => format!("fara {}", if self.up { "sus" } else { "jos" }),
        }
    }
}

/// Culoarea efectivă a unei cărți în contract (jokerii sunt atu dacă există atu).
fn eff_suit(c: u8, k: Contract) -> Option<u8> {
    if is_joker(c) {
        k.trump
    } else {
        Some(std_suit(c))
    }
}

/// Puterea în cadrul culorii (mai mare = bate).
fn power(c: u8, k: Contract) -> i32 {
    if c == BIG {
        return 200;
    }
    if c == LITTLE {
        return 199;
    }
    let r = std_rank(c) as i32; // 0 = „2” … 12 = „A”
    if k.up {
        r
    } else {
        12 - r
    }
}

/// Mutările legale pentru `hand` cu cartea de deschidere `lead` (dacă există).
fn legal_cards(hand: &[u8], lead: Option<u8>, k: Contract) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::new();
    let mut add = |c: u8| {
        if !v.contains(&c) {
            v.push(c)
        }
    };
    let Some(l) = lead else {
        for &c in hand {
            add(c);
        }
        return v;
    };
    let led = if k.trump.is_none() && is_joker(l) {
        None
    } else {
        eff_suit(l, k)
    };
    let Some(ls) = led else {
        for &c in hand {
            add(c);
        }
        return v;
    };
    let follow: Vec<u8> = hand
        .iter()
        .copied()
        .filter(|&c| eff_suit(c, k) == Some(ls))
        .collect();
    if follow.is_empty() {
        for &c in hand {
            add(c);
        }
    } else {
        for c in follow {
            add(c);
        }
        if k.trump.is_none() {
            for &c in hand {
                if is_joker(c) {
                    add(c);
                }
            }
        }
    }
    v
}

/// Indexul câștigătorului unei levate de 2 cărți.
fn winner(cards: &[u8], k: Contract) -> usize {
    let (a, b) = (cards[0], cards[1]);
    if is_joker(a) || is_joker(b) {
        return if power(b, k) > power(a, k) && is_joker(b) {
            1
        } else if is_joker(a) {
            0
        } else {
            1
        };
    }
    let (sa, sb) = (std_suit(a), std_suit(b));
    if sa == sb {
        return if power(b, k) > power(a, k) { 1 } else { 0 };
    }
    if Some(sb) == k.trump {
        1
    } else {
        0
    }
}

impl BidWhist {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = BidWhist {
            rng: Rng::new(seed),
            hands: [Vec::new(), Vec::new()],
            kitty: Vec::new(),
            junk: Vec::new(),
            dealer: Player::Two,
            side: Player::One,
            phase: Phase::Bid1,
            first_bid: 0,
            bid: 0,
            bidder: Player::One,
            contract: None,
            trick: Vec::new(),
            played: Vec::new(),
            taken: [0, 0],
            scores: [0, 0],
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
        let mut d: Vec<u8> = (0..54).collect();
        shuffle(&mut d, &mut self.rng);
        self.hands = [Vec::new(), Vec::new()];
        for _ in 0..12 {
            for p in 0..2 {
                self.hands[p].push(d.pop().unwrap());
            }
        }
        self.kitty = (0..6).map(|_| d.pop().unwrap()).collect();
        self.junk.clear();
        self.contract = None;
        self.trick.clear();
        self.played.clear();
        self.taken = [0, 0];
        self.delta = [0, 0];
        self.fresh = [0, 0];
        self.first_bid = 0;
        self.bid = 0;
        for h in &mut self.hands {
            sort_cards(h);
        }
        self.phase = Phase::Bid1;
        self.side = self.dealer.other();
    }

    fn take_kitty(&mut self, p: Player) {
        let k = std::mem::take(&mut self.kitty);
        self.fresh[p.idx()] = k.len() as u8;
        self.hands[p.idx()].extend(k);
        self.bidder = p;
        self.phase = Phase::Discard;
        self.side = p;
    }

    fn end_hand(&mut self) {
        let k = self.contract.unwrap();
        let b = self.bidder;
        let t = self.taken[b.idx()] as i32;
        let mult = if k.trump.is_none() { 2 } else { 1 };
        let book = 6;
        let pts = if t >= book + self.bid as i32 {
            (t - book) * mult
        } else {
            -(self.bid as i32) * mult
        };
        self.scores[b.idx()] += pts;
        self.delta[b.idx()] += pts;
        self.log = format!(
            "Mână încheiată: J{} a luat {} levate (a licitat {}): {:+} puncte",
            b.idx() + 1,
            t,
            self.bid,
            pts
        );
        let over = self.scores.iter().any(|&s| s >= TARGET || s <= -TARGET);
        self.phase = if over { Phase::Over } else { Phase::HandOver };
    }
}

impl Game for BidWhist {
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
            Phase::Redeal => vec![Move::Redeal],
            Phase::Bid1 => {
                let mut v = vec![Move::Pass];
                v.extend((3..=6).map(Move::Bid));
                v
            }
            Phase::Bid2 => {
                let mut v = vec![Move::Pass];
                v.extend(((self.first_bid + 1).max(3)..=6).map(Move::Bid));
                v
            }
            Phase::Discard => {
                let mut v = Vec::new();
                for &c in &self.hands[self.side.idx()] {
                    if !v.contains(&Move::Discard(c)) {
                        v.push(Move::Discard(c));
                    }
                }
                v
            }
            Phase::Declare => {
                let mut v = Vec::new();
                for up in [true, false] {
                    for t in 0..4u8 {
                        v.push(Move::Declare(Contract { up, trump: Some(t) }));
                    }
                    v.push(Move::Declare(Contract { up, trump: None }));
                }
                v
            }
            Phase::Play => {
                let k = self.contract.unwrap();
                legal_cards(
                    &self.hands[self.side.idx()],
                    self.trick.first().map(|&(_, c)| c),
                    k,
                )
                .into_iter()
                .map(Move::Play)
                .collect()
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        self.fresh = [0, 0];
        self.prev = self.side;
        match *m {
            Move::Next => {
                self.dealer = self.dealer.other();
                self.deal();
            }
            Move::Redeal => {
                self.deal();
            }
            Move::Pass => {
                if self.phase == Phase::Bid1 {
                    self.phase = Phase::Bid2;
                    self.side = self.dealer;
                } else if self.first_bid == 0 {
                    self.log = "Toți au dat pas: scrie «reimparte».".into();
                    self.phase = Phase::Redeal;
                } else {
                    let p = self.dealer.other();
                    self.bid = self.first_bid;
                    self.take_kitty(p);
                }
            }
            Move::Bid(n) => {
                if self.phase == Phase::Bid1 {
                    self.first_bid = n;
                    self.phase = Phase::Bid2;
                    self.side = self.dealer;
                } else {
                    self.bid = n;
                    let p = self.dealer;
                    self.take_kitty(p);
                }
            }
            Move::Discard(c) => {
                let p = self.side;
                let pos = self.hands[p.idx()].iter().position(|&x| x == c).unwrap();
                self.hands[p.idx()].remove(pos);
                self.junk.push(c);
                if self.hands[p.idx()].len() == 12 {
                    self.phase = Phase::Declare;
                }
            }
            Move::Declare(k) => {
                self.contract = Some(k);
                for h in &mut self.hands {
                    sort_cards(h);
                }
                self.phase = Phase::Play;
                self.side = self.bidder;
            }
            Move::Play(c) => {
                let p = self.side;
                let k = self.contract.unwrap();
                let pos = self.hands[p.idx()].iter().position(|&x| x == c).unwrap();
                self.hands[p.idx()].remove(pos);
                self.trick.push((p, c));
                if self.trick.len() < 2 {
                    self.side = p.other();
                    return;
                }
                let cards = [self.trick[0].1, self.trick[1].1];
                let w = self.trick[winner(&cards, k)].0;
                self.taken[w.idx()] += 1;
                self.played.extend(cards);
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
        Some(if self.scores[0] >= TARGET || self.scores[1] <= -TARGET {
            Outcome::Win(Player::One)
        } else {
            Outcome::Win(Player::Two)
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        if self.phase == Phase::Over {
            return 0;
        }
        honest_eval(self, p, 14)
    }

    fn render(&self) -> String {
        let mut s = String::new();
        if self.prev != self.side && !matches!(self.phase, Phase::Over | Phase::HandOver) {
            if let Viewer::HotSeat = self.viewer {
                s.push_str(&handover_banner(self.side));
            }
        }
        s.push_str(&format!(
            "Bid Whist (2 jucători) — scor: J1 {}, J2 {} (±{}) | dealer: J{}\n",
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
                s.push_str("Scrie «urmatoarea».\n");
                return s;
            }
            Phase::Redeal => s.push_str("Toți au dat pas. Scrie «reimparte».\n"),
            Phase::Bid1 => s.push_str("Licitație: non-dealerul (pas sau 3–6).\n"),
            Phase::Bid2 => s.push_str(&format!(
                "Licitație: dealerul (non-dealerul a spus {}).\n",
                if self.first_bid == 0 {
                    "pas".to_string()
                } else {
                    self.first_bid.to_string()
                }
            )),
            Phase::Discard => s.push_str(&format!(
                "J{} a licitat {} și a luat kitty-ul: aruncă până rămân 12 cărți.\n",
                self.bidder.idx() + 1,
                self.bid
            )),
            Phase::Declare => s.push_str("Declară contractul: sus/jos + atu, sau fara sus/jos.\n"),
            Phase::Play => {
                s.push_str(&format!(
                    "Contract J{}: {} (la {} levate) | levate: J1 {}, J2 {}\n",
                    self.bidder.idx() + 1,
                    self.contract.unwrap().text(),
                    6 + self.bid as i32,
                    self.taken[0],
                    self.taken[1]
                ));
                if !self.trick.is_empty() {
                    let cs: Vec<u8> = self.trick.iter().map(|&(_, c)| c).collect();
                    s.push_str(&format!("Pe masă: {}\n", join(&cs)));
                }
            }
        }
        let v = self.viewer.who(self.side);
        s.push_str(&format!(
            "Mâna ta (J{}): {}\n",
            v.idx() + 1,
            join(&self.hands[v.idx()])
        ));
        s.push_str(&format!(
            "J{} are {} cărți\n",
            v.other().idx() + 1,
            self.hands[v.other().idx()].len()
        ));
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Pass => "pas".into(),
            Move::Bid(n) => format!("licit {n}"),
            Move::Discard(c) => format!("dis {}", ctext(*c)),
            Move::Declare(k) => k.text(),
            Move::Play(c) => ctext(*c),
            Move::Redeal => "reimparte".into(),
            Move::Next => "urmatoarea".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

/// Contractul probabil bun pentru o mână: (contract, putere estimată).
fn best_contract(h: &[u8]) -> (Contract, f64) {
    let jokers = h.iter().filter(|&&c| is_joker(c)).count() as f64;
    let mut best = (
        Contract {
            up: true,
            trump: Some(0),
        },
        -1.0,
    );
    for up in [true, false] {
        for t in 0..4u8 {
            let k = Contract { up, trump: Some(t) };
            let mut s = jokers * 2.0;
            for &c in h.iter().filter(|&&c| !is_joker(c)) {
                let hi = power(c, k) as f64; // 0..12
                if std_suit(c) == t {
                    s += 0.6 + hi / 8.0;
                } else {
                    s += (hi - 8.0).max(0.0) / 6.0;
                }
            }
            if s > best.1 {
                best = (k, s);
            }
        }
    }
    best
}

impl Honest for BidWhist {
    fn determinize(&self, me: Player, rng: &mut Rng) -> Self {
        let mut g = self.clone();
        let (mi, oi) = (me.idx(), 1 - me.idx());
        let nf = (self.fresh[mi] as usize).min(self.hands[mi].len());
        let known_mine: Vec<u8> = self.hands[mi][..self.hands[mi].len() - nf].to_vec();
        let mut pool: Vec<u8> = (0..54u8)
            .filter(|c| {
                !known_mine.contains(c)
                    && !self.played.contains(c)
                    && !self.trick.iter().any(|&(_, x)| x == *c)
                    && !(self.bidder == me && self.junk.contains(c))
            })
            .collect();
        shuffle(&mut pool, rng);
        let mut mine = known_mine;
        for _ in 0..nf {
            mine.push(pool.pop().unwrap());
        }
        let n_opp = self.hands[oi].len();
        g.hands[oi] = (0..n_opp).map(|_| pool.pop().unwrap()).collect();
        if !self.kitty.is_empty() {
            g.kitty = (0..self.kitty.len()).map(|_| pool.pop().unwrap()).collect();
        }
        if self.bidder == me {
            // aruncările mele rămân cunoscute mie
        } else {
            let nj = self.junk.len();
            g.junk = (0..nj).map(|_| pool.pop().unwrap()).collect();
        }
        g.hands[mi] = mine;
        sort_cards(&mut g.hands[oi]);
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
        (self.delta[p.idx()] - self.delta[p.other().idx()]) as f64
    }

    fn policy(&self, rng: &mut Rng) -> Move {
        let mv = self.legal_moves();
        if mv.len() == 1 {
            return mv[0];
        }
        let h = &self.hands[self.side.idx()];
        match self.phase {
            Phase::Bid1 | Phase::Bid2 => {
                let (_, s) = best_contract(h);
                let want = if s >= 12.5 {
                    6
                } else if s >= 11.0 {
                    5
                } else if s >= 9.5 {
                    4
                } else if s >= 8.0 {
                    3
                } else {
                    0
                };
                let mut pick = Move::Pass;
                for &m in &mv {
                    if let Move::Bid(n) = m {
                        if n <= want {
                            pick = m;
                        }
                    }
                }
                pick
            }
            Phase::Discard => {
                let (k, _) = best_contract(h);
                let t = k.trump.unwrap();
                // aruncă cărțile cele mai slabe din culori non-atu (preferând culorile scurte)
                let worst = h
                    .iter()
                    .copied()
                    .filter(|&c| !is_joker(c) && std_suit(c) != t)
                    .min_by_key(|&c| {
                        let len = h
                            .iter()
                            .filter(|&&x| !is_joker(x) && std_suit(x) == std_suit(c))
                            .count();
                        (power(c, k) + 3 * len as i32, c)
                    });
                let c = worst.unwrap_or_else(|| {
                    h.iter()
                        .copied()
                        .min_by_key(|&c| if is_joker(c) { 1000 } else { power(c, k) })
                        .unwrap()
                });
                Move::Discard(c)
            }
            Phase::Declare => {
                let (k, _) = best_contract(h);
                Move::Declare(k)
            }
            _ => {
                if rng.below(100) < 8 {
                    return mv[rng.below(mv.len())];
                }
                let k = self.contract.unwrap();
                let cards: Vec<u8> = mv
                    .iter()
                    .filter_map(|m| {
                        if let Move::Play(c) = m {
                            Some(*c)
                        } else {
                            None
                        }
                    })
                    .collect();
                let key = |c: u8| -> i32 {
                    let tr = eff_suit(c, k) == k.trump && k.trump.is_some();
                    power(c, k) + if tr || is_joker(c) { 100 } else { 0 }
                };
                if let Some(&(_, lead)) = self.trick.first() {
                    let mut win: Option<u8> = None;
                    for &c in &cards {
                        if winner(&[lead, c], k) == 1 && win.is_none_or(|w| key(c) < key(w)) {
                            win = Some(c);
                        }
                    }
                    match win {
                        Some(w) => Move::Play(w),
                        None => Move::Play(cards.iter().copied().min_by_key(|&c| key(c)).unwrap()),
                    }
                } else {
                    Move::Play(cards.iter().copied().max_by_key(|&c| key(c)).unwrap())
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn card(rank_label: &str, suit: char) -> u8 {
        let r = [
            "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
        ]
        .iter()
        .position(|&x| x == rank_label)
        .unwrap() as u8;
        let s = SUIT_LETTERS.iter().position(|&x| x == suit).unwrap() as u8;
        s * 13 + r
    }

    const UP_H: Contract = Contract {
        up: true,
        trump: Some(2),
    };

    #[test]
    fn deal_sizes() {
        let g = BidWhist::with_seed(1);
        assert_eq!(g.hands[0].len(), 12);
        assert_eq!(g.hands[1].len(), 12);
        assert_eq!(g.kitty.len(), 6);
    }

    #[test]
    fn uptown_downtown_order() {
        let (a, k) = (card("A", 'h'), card("K", 'h'));
        assert_eq!(winner(&[k, a], UP_H), 1);
        let down = Contract {
            up: false,
            trump: Some(2),
        };
        assert_eq!(winner(&[k, a], down), 0); // la Downtown As e cel mai mic
        let (two, three) = (card("2", 'd'), card("3", 'd'));
        assert_eq!(winner(&[three, two], down), 1);
    }

    #[test]
    fn jokers_are_top_trumps_and_no_trump_rule() {
        let ah = card("A", 'h');
        assert_eq!(winner(&[ah, LITTLE], UP_H), 1);
        assert_eq!(winner(&[LITTLE, BIG], UP_H), 1);
        assert_eq!(winner(&[BIG, LITTLE], UP_H), 0);
        let nt = Contract {
            up: true,
            trump: None,
        };
        assert_eq!(winner(&[ah, LITTLE], nt), 1);
        // fără atu, jokerul se joacă oricând
        let hand = vec![card("2", 'c'), card("5", 'c'), BIG, card("A", 's')];
        let m = legal_cards(&hand, Some(card("K", 'c')), nt);
        assert!(m.contains(&BIG) && m.contains(&card("2", 'c')) && !m.contains(&card("A", 's')));
    }

    #[test]
    fn must_follow_suit_jokers_count_as_trump() {
        let hand = vec![card("2", 'c'), BIG, card("A", 's')];
        // s-a condus atu (cupă): jokerul e atu, deci poți răspunde cu el; altfel orice nu e permis dacă ai atu
        let m = legal_cards(&hand, Some(card("K", 'h')), UP_H);
        assert!(m.contains(&BIG) && m.len() == 1);
        // ai culoarea cerută (treflă): obligatoriu treflă
        let m = legal_cards(&hand, Some(card("K", 'c')), UP_H);
        assert_eq!(m, vec![card("2", 'c')]);
        // fără culoarea cerută: orice
        let m = legal_cards(&hand, Some(card("K", 'd')), UP_H);
        assert_eq!(m.len(), 3);
    }

    #[test]
    fn auction_flow_and_kitty() {
        let mut g = BidWhist::with_seed(2);
        let kit = g.kitty.clone();
        g.apply(&Move::Bid(4));
        assert_eq!(g.phase, Phase::Bid2);
        assert!(!g.legal_moves().contains(&Move::Bid(4)));
        assert!(g.legal_moves().contains(&Move::Bid(5)));
        g.apply(&Move::Pass);
        assert_eq!(g.bidder, g.dealer.other());
        assert_eq!(g.phase, Phase::Discard);
        assert_eq!(g.hands[g.bidder.idx()].len(), 18);
        for c in kit {
            assert!(g.hands[g.bidder.idx()].contains(&c));
        }
        for _ in 0..6 {
            let m = g.legal_moves()[0];
            g.apply(&m);
        }
        assert_eq!(g.phase, Phase::Declare);
        assert_eq!(g.legal_moves().len(), 10);
        g.apply(&Move::Declare(UP_H));
        assert_eq!(g.phase, Phase::Play);
        assert_eq!(g.side, g.bidder);
    }

    #[test]
    fn everyone_passes_redeals() {
        let mut g = BidWhist::with_seed(3);
        let before = g.hands.clone();
        g.apply(&Move::Pass);
        g.apply(&Move::Pass);
        assert_eq!(g.phase, Phase::Redeal);
        g.apply(&Move::Redeal);
        assert_eq!(g.phase, Phase::Bid1);
        assert_ne!(before, g.hands);
    }

    #[test]
    fn scoring() {
        let mut g = BidWhist::with_seed(4);
        g.bidder = Player::One;
        g.bid = 4;
        g.contract = Some(UP_H);
        g.taken = [10, 2];
        g.end_hand();
        assert_eq!(g.scores, [4, 0]);
        let mut h = BidWhist::with_seed(5);
        h.bidder = Player::Two;
        h.bid = 3;
        h.contract = Some(Contract {
            up: true,
            trump: None,
        });
        h.taken = [5, 7];
        h.end_hand();
        assert_eq!(h.scores, [0, -6]); // set la fără atu: dublu
    }

    #[test]
    fn determinize_conserves_cards() {
        let g = BidWhist::with_seed(6);
        let mut rng = Rng::new(7);
        let d = g.determinize(Player::One, &mut rng);
        assert_eq!(d.hands[0], g.hands[0]);
        assert_eq!(d.hands[1].len(), 12);
        assert_eq!(d.kitty.len(), 6);
        let mut all: Vec<u8> = d.hands[0]
            .iter()
            .chain(&d.hands[1])
            .chain(&d.kitty)
            .copied()
            .collect();
        all.sort_unstable();
        all.dedup();
        assert_eq!(all.len(), 30);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=8u64 {
            let mut g = BidWhist::with_seed(seed);
            let mut rng = Rng::new(seed + 3);
            let mut n = 0;
            while g.outcome().is_none() && n < 20_000 {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed} after {n}");
        }
    }
}

#[cfg(test)]
mod a1_blind {
    use super::*;

    #[test]
    fn ai_ignores_hidden_information_at_all_levels() {
        crate::ai_audit_a1::audit_honest("bid_whist", BidWhist::with_seed, 1..=2, 11, 60);
    }
}
