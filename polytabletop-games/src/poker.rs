//! Poker — Texas hold'em, heads-up (2 players), fixed-limit style, 20 hands.
//!
//! Rules: 52 cards, two hole cards each, five community cards (flop 3, turn,
//! river), best five of seven. Each player starts with 1000 chips; blinds are
//! fixed at 10 (small, the button) / 20 (big); the button alternates every
//! hand and starts with Player 1. Heads-up the button acts first before the
//! flop and last after it. Actions: `fold`, `call`/`check`, `raise` — the
//! raise size is *fixed*: 20 before the flop and on the flop, 40 on the turn
//! and river; at most 3 raises per betting round. Short stacks go all-in and
//! uncalled chips are returned. Ties split the pot.
//!
//! Match: 20 hands; the player with more chips wins (equal = draw); the match
//! also ends early if a player is out of chips.
//!
//! Dealing is made of forced single moves (`flop`, `turn`, `river`,
//! `showdown`, `decontare`) so that the AI's own action never reveals a card
//! to `evaluate()`; `showdown`/`decontare` also deal the next hand.
//!
//! Hidden information: deck and both hands live in the state; `render()`
//! shows Player 1's hole cards and "[??] [??]" for Player 2 (revealed in the
//! text of the previous hand's result). The AI is `AlphaBeta(1)` over
//! `evaluate()`: an equity estimate built only from the evaluated player's
//! hole cards and the board (Chen-style score before the flop; made-hand
//! category after it, plus a flush-draw bonus), turned into expected chips
//! assuming outstanding bets are called. Limitations: no opponent
//! modelling, no bluffing, no draws other than flush draws; it plays
//! "tight-aggressive by the numbers" and can be exploited.

use crate::cards::Viewer;
use crate::cards52::{join_symbols, std_deck, std_rank, std_suit, std_symbol};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const HANDS: u8 = 20;
pub const START_STACK: i32 = 1000;
pub const SMALL_BLIND: i32 = 10;
pub const BIG_BLIND: i32 = 20;
const MAX_RAISES: u8 = 3;

// ---------------------------------------------------------------- evaluator

pub const CATEGORY_NAMES: [&str; 9] = [
    "carte mare",
    "pereche",
    "două perechi",
    "trei de un fel",
    "scară",
    "culoare",
    "full house",
    "careu",
    "scară de culoare",
];

/// Score of exactly five cards: `category << 20 | five 4-bit tie-breakers`.
/// Higher is better; equal scores tie.
pub fn eval5(cards: &[u8; 5]) -> u32 {
    let mut counts = [0u8; 13];
    for &c in cards {
        counts[std_rank(c) as usize] += 1;
    }
    let flush = cards.iter().all(|&c| std_suit(c) == std_suit(cards[0]));
    // groups sorted by (count desc, rank desc)
    let mut groups: Vec<(u8, u8)> = (0..13u8)
        .filter(|&r| counts[r as usize] > 0)
        .map(|r| (counts[r as usize], r))
        .collect();
    groups.sort_unstable_by(|a, b| b.cmp(a));
    let straight_high = if groups.len() == 5 {
        let hi = groups[0].1;
        let lo = groups[4].1;
        if hi - lo == 4 {
            Some(hi)
        } else if hi == 12 && groups[1].1 == 3 && lo == 0 {
            Some(3) // wheel A-2-3-4-5, five-high
        } else {
            None
        }
    } else {
        None
    };
    let shape: Vec<u8> = groups.iter().map(|g| g.0).collect();
    let (cat, kick): (u32, Vec<u8>) = match (straight_high, flush) {
        (Some(h), true) => (8, vec![h]),
        _ if shape == [4, 1] => (7, groups.iter().map(|g| g.1).collect()),
        _ if shape == [3, 2] => (6, groups.iter().map(|g| g.1).collect()),
        (_, true) => (5, groups.iter().map(|g| g.1).collect()),
        (Some(h), false) => (4, vec![h]),
        _ if shape == [3, 1, 1] => (3, groups.iter().map(|g| g.1).collect()),
        _ if shape == [2, 2, 1] => (2, groups.iter().map(|g| g.1).collect()),
        _ if shape == [2, 1, 1, 1] => (1, groups.iter().map(|g| g.1).collect()),
        _ => (0, groups.iter().map(|g| g.1).collect()),
    };
    let mut score = cat << 20;
    for (i, &k) in kick.iter().take(5).enumerate() {
        score |= (k as u32) << (16 - 4 * i as u32);
    }
    score
}

/// Best five-card score among 5 to 7 cards.
pub fn best_of(cards: &[u8]) -> u32 {
    let n = cards.len();
    assert!((5..=7).contains(&n));
    let mut best = 0;
    for mask in 0u32..(1 << n) {
        if mask.count_ones() != 5 {
            continue;
        }
        let mut pick = [0u8; 5];
        let mut j = 0;
        for (i, &c) in cards.iter().enumerate() {
            if mask >> i & 1 == 1 {
                pick[j] = c;
                j += 1;
            }
        }
        best = best.max(eval5(&pick));
    }
    best
}

pub fn category(score: u32) -> usize {
    (score >> 20) as usize
}

// ------------------------------------------------------- equity (heuristic)

/// Chen-style preflop score of two hole cards (about -1 … 20).
fn chen(h: [u8; 2]) -> f64 {
    let (a, b) = (
        std_rank(h[0]).max(std_rank(h[1])),
        std_rank(h[0]).min(std_rank(h[1])),
    );
    let base = |r: u8| -> f64 {
        match r {
            12 => 10.0,
            11 => 8.0,
            10 => 7.0,
            9 => 6.0,
            r => (r as f64 + 2.0) / 2.0,
        }
    };
    let mut s = base(a);
    if a == b {
        return (s * 2.0).max(5.0).ceil();
    }
    if std_suit(h[0]) == std_suit(h[1]) {
        s += 2.0;
    }
    let gap = a - b - 1;
    s -= match gap {
        0 => 0.0,
        1 => 1.0,
        2 => 2.0,
        3 => 4.0,
        _ => 5.0,
    };
    if gap <= 1 && a < 10 {
        s += 1.0;
    }
    s.ceil()
}

fn count_category(cards: &[u8]) -> usize {
    let mut counts = [0u8; 13];
    for &c in cards {
        counts[std_rank(c) as usize] += 1;
    }
    let mut v: Vec<u8> = counts.iter().copied().filter(|&c| c > 0).collect();
    v.sort_unstable_by(|a, b| b.cmp(a));
    match v.as_slice() {
        [4, ..] => 7,
        [3, 2, ..] => 6,
        [3, ..] => 3,
        [2, 2, ..] => 2,
        [2, ..] => 1,
        _ => 0,
    }
}

/// Winning chance estimate in 0..1 from the player's own cards only.
pub fn equity(hole: [u8; 2], board: &[u8]) -> f64 {
    if board.is_empty() {
        return 0.32 + 0.5 * ((chen(hole) + 1.0) / 21.0).clamp(0.0, 1.0);
    }
    let mut all = hole.to_vec();
    all.extend_from_slice(board);
    let (score, cat) = if all.len() >= 5 {
        let s = best_of(&all);
        (s, category(s))
    } else {
        (0, count_category(&all))
    };
    let board_cat = if board.len() >= 5 {
        category(best_of(board))
    } else {
        count_category(board)
    };
    let top = std_rank(hole[0]).max(std_rank(hole[1])) as f64 / 12.0;
    let mut e = if cat == board_cat && cat < 4 {
        // the board alone makes this hand: only the high card matters
        0.22 + 0.04 * cat as f64 + 0.18 * top
    } else {
        match cat {
            0 => 0.22 + 0.18 * top,
            1 => 0.50 + 0.15 * ((score >> 16 & 0xF) as f64 / 12.0),
            2 => 0.76,
            3 => 0.85,
            4 => 0.90,
            5 => 0.93,
            6 => 0.96,
            7 => 0.99,
            _ => 0.995,
        }
    };
    if cat < 5 && board.len() < 5 {
        for s in 0..4u8 {
            let n = all.iter().filter(|&&c| std_suit(c) == s).count();
            let mine = hole.iter().any(|&c| std_suit(c) == s);
            if n >= 4 && mine {
                e += 0.07;
            }
        }
    }
    e.clamp(0.02, 0.995)
}

// ------------------------------------------------- equity (Monte Carlo, fair)

fn straight_high(mask: u16) -> Option<u8> {
    for h in (4..=12u8).rev() {
        if (mask >> (h - 4)) & 0x1F == 0x1F {
            return Some(h);
        }
    }
    if mask & 0x100F == 0x100F {
        return Some(3);
    }
    None
}

fn top_ranks(mut mask: u16, n: usize) -> u32 {
    let mut out = 0u32;
    for i in 0..n {
        if mask == 0 {
            break;
        }
        let r = 15 - mask.leading_zeros();
        out |= r << (16 - 4 * i as u32);
        mask &= !(1 << r);
    }
    out
}

/// Fast best-hand score of 5 to 7 cards (same category scale as `best_of`).
fn fast_score(cards: &[u8]) -> u32 {
    let mut rc = [0u8; 13];
    let mut sm = [0u16; 4];
    let mut all = 0u16;
    for &c in cards {
        let (r, s) = (std_rank(c) as usize, std_suit(c) as usize);
        rc[r] += 1;
        sm[s] |= 1 << r;
        all |= 1 << r;
    }
    let flush = (0..4).find(|&s| sm[s].count_ones() >= 5);
    if let Some(f) = flush {
        if let Some(h) = straight_high(sm[f]) {
            return 8 << 20 | (h as u32) << 16;
        }
    }
    let (mut quad, mut trips, mut pairs) = (None, Vec::new(), Vec::new());
    for r in (0..13usize).rev() {
        match rc[r] {
            4 => quad = Some(r as u32),
            3 => trips.push(r as u32),
            2 => pairs.push(r as u32),
            _ => {}
        }
    }
    if let Some(q) = quad {
        let rest = all & !(1 << q);
        return 7 << 20 | q << 16 | top_ranks(rest, 1) >> 4;
    }
    if !trips.is_empty() && (trips.len() > 1 || !pairs.is_empty()) {
        let t = trips[0];
        let pr = if trips.len() > 1 {
            trips[1].max(pairs.first().copied().unwrap_or(0))
        } else {
            pairs[0]
        };
        return 6 << 20 | t << 16 | pr << 12;
    }
    if let Some(f) = flush {
        return 5 << 20 | top_ranks(sm[f], 5);
    }
    if let Some(h) = straight_high(all) {
        return 4 << 20 | (h as u32) << 16;
    }
    if let Some(&t) = trips.first() {
        let rest = all & !(1 << t);
        return 3 << 20 | t << 16 | top_ranks(rest, 2) >> 4;
    }
    if pairs.len() >= 2 {
        let (a, b) = (pairs[0], pairs[1]);
        let rest = all & !(1 << a) & !(1 << b);
        return 2 << 20 | a << 16 | b << 12 | top_ranks(rest, 1) >> 8;
    }
    if let Some(&a) = pairs.first() {
        let rest = all & !(1 << a);
        return 1 << 20 | a << 16 | top_ranks(rest, 3) >> 4;
    }
    top_ranks(all, 5)
}

thread_local! {
    static EQ_CACHE: std::cell::RefCell<std::collections::HashMap<Vec<u8>, f64>> =
        std::cell::RefCell::new(std::collections::HashMap::new());
}

/// Chance of winning (ties count half) against a uniformly random hand, by
/// sampling the opponent's two cards and the missing board cards from the cards
/// the player cannot see. Uses only the player's hole cards and the public board.
pub fn mc_equity(hole: [u8; 2], board: &[u8]) -> f64 {
    let mut key: Vec<u8> = vec![hole[0].min(hole[1]), hole[0].max(hole[1])];
    key.extend_from_slice(board);
    if let Some(v) = EQ_CACHE.with(|c| c.borrow().get(&key).copied()) {
        return v;
    }
    let mut unknown: Vec<u8> = (0..52u8)
        .filter(|c| !hole.contains(c) && !board.contains(c))
        .collect();
    let mut seed = 0xC0FFEEu64;
    for &k in &key {
        seed = seed.rotate_left(6) ^ (k as u64 + 1);
    }
    let mut rng = Rng::new(seed);
    let need = 5 - board.len();
    let n = 500usize;
    let mut total = 0.0;
    let mut mine: Vec<u8> = hole.to_vec();
    mine.extend_from_slice(board);
    let mut theirs: Vec<u8> = Vec::with_capacity(7);
    for _ in 0..n {
        let k = 2 + need;
        for i in 0..k {
            let j = i + rng.below(unknown.len() - i);
            unknown.swap(i, j);
        }
        mine.truncate(2 + board.len());
        mine.extend_from_slice(&unknown[2..k]);
        theirs.clear();
        theirs.extend_from_slice(&unknown[..2]);
        theirs.extend_from_slice(board);
        theirs.extend_from_slice(&unknown[2..k]);
        let (a, b) = (fast_score(&mine), fast_score(&theirs));
        total += if a > b {
            1.0
        } else if a == b {
            0.5
        } else {
            0.0
        };
    }
    let v = (total / n as f64).clamp(0.01, 0.99);
    EQ_CACHE.with(|c| {
        let mut c = c.borrow_mut();
        if c.len() > 20000 {
            c.clear();
        }
        c.insert(key, v);
    });
    v
}

// ---------------------------------------------------------------- the game

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Betting,
    Deal,
    Showdown,
    FoldEnd(Player),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Fold,
    Call,
    Raise,
    Deal,
    Showdown,
    Settle,
}

#[derive(Clone)]
pub struct Poker {
    rng: Rng,
    deck: Vec<u8>,
    hole: [[u8; 2]; 2],
    board: Vec<u8>,
    stacks: [i32; 2], // chips behind
    commit: [i32; 2], // chips put in this hand
    street_commit: [i32; 2],
    street: u8,
    button: Player,
    side: Player,
    phase: Phase,
    raises: u8,
    acted: [bool; 2],
    hand_no: u8,
    over: bool,
    last_result: String,
    viewer: Viewer,
}

impl Default for Poker {
    fn default() -> Self {
        Self::new()
    }
}

impl Poker {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Poker {
            rng: Rng::new(seed),
            deck: Vec::new(),
            hole: [[0; 2]; 2],
            board: Vec::new(),
            stacks: [START_STACK; 2],
            commit: [0; 2],
            street_commit: [0; 2],
            street: 0,
            button: Player::One,
            side: Player::One,
            phase: Phase::Betting,
            raises: 0,
            acted: [false; 2],
            hand_no: 0,
            over: false,
            last_result: String::new(),
            viewer: Viewer::Fixed(Player::One),
        };
        g.start_hand();
        g
    }

    pub fn stacks(&self) -> (i32, i32) {
        (self.stacks[0], self.stacks[1])
    }

    fn post(&mut self, p: Player, amount: i32) {
        let a = amount.min(self.stacks[p.idx()]);
        self.stacks[p.idx()] -= a;
        self.commit[p.idx()] += a;
        self.street_commit[p.idx()] += a;
    }

    fn start_hand(&mut self) {
        self.hand_no += 1;
        self.deck = std_deck(&mut self.rng);
        for p in 0..2 {
            self.hole[p] = [self.deck.pop().unwrap(), self.deck.pop().unwrap()];
        }
        self.board.clear();
        self.commit = [0; 2];
        self.street_commit = [0; 2];
        self.street = 0;
        self.raises = 0;
        self.acted = [false; 2];
        let bb = self.button.other();
        self.post(self.button, SMALL_BLIND);
        self.post(bb, BIG_BLIND);
        self.side = self.button;
        self.phase = if self.stacks[0] == 0 || self.stacks[1] == 0 {
            Phase::Showdown
        } else {
            Phase::Betting
        };
    }

    fn to_call(&self) -> i32 {
        let me = self.side.idx();
        (self.street_commit[1 - me] - self.street_commit[me]).max(0)
    }

    fn raise_size(&self) -> i32 {
        if self.street < 2 {
            20
        } else {
            40
        }
    }

    fn can_raise(&self) -> bool {
        let me = self.side.idx();
        self.raises < MAX_RAISES && self.stacks[me] > self.to_call() && self.stacks[1 - me] > 0
    }

    fn end_round(&mut self) {
        if self.stacks[0] == 0 || self.stacks[1] == 0 || self.street == 3 {
            self.phase = Phase::Showdown;
        } else {
            self.phase = Phase::Deal;
        }
    }

    /// Splits the pot, records the result text, deals the next hand or ends.
    fn settle(&mut self, folded_winner: Option<Player>) {
        while self.board.len() < 5 && folded_winner.is_none() {
            self.board.push(self.deck.pop().unwrap());
        }
        let matched = self.commit[0].min(self.commit[1]);
        for i in 0..2 {
            self.stacks[i] += self.commit[i] - matched;
        }
        let pot = 2 * matched;
        let text;
        match folded_winner {
            Some(w) => {
                self.stacks[w.idx()] += pot;
                text = format!(
                    "Mâna {}: Jucător {} câștigă {} (adversarul a renunțat).",
                    self.hand_no,
                    w.idx() + 1,
                    pot
                );
            }
            None => {
                let mut sc = [0u32; 2];
                for (p, s) in sc.iter_mut().enumerate() {
                    let mut cards = self.hole[p].to_vec();
                    cards.extend_from_slice(&self.board);
                    *s = best_of(&cards);
                }
                let shown = format!(
                    "J1 {} ({}), J2 {} ({}), masă {}",
                    join_symbols(&self.hole[0], std_symbol),
                    CATEGORY_NAMES[category(sc[0])],
                    join_symbols(&self.hole[1], std_symbol),
                    CATEGORY_NAMES[category(sc[1])],
                    join_symbols(&self.board, std_symbol)
                );
                if sc[0] == sc[1] {
                    self.stacks[0] += matched;
                    self.stacks[1] += matched;
                    text = format!(
                        "Mâna {}: egalitate, potul se împarte. {shown}",
                        self.hand_no
                    );
                } else {
                    let w = if sc[0] > sc[1] { 0 } else { 1 };
                    self.stacks[w] += pot;
                    text = format!(
                        "Mâna {}: Jucător {} câștigă {}. {shown}",
                        self.hand_no,
                        w + 1,
                        pot
                    );
                }
            }
        }
        self.last_result = text;
        self.commit = [0; 2];
        self.street_commit = [0; 2];
        if self.hand_no >= HANDS || self.stacks[0] == 0 || self.stacks[1] == 0 {
            self.over = true;
        } else {
            self.button = self.button.other();
            self.start_hand();
        }
    }
}

impl Game for Poker {
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
            Phase::Betting => {
                let mut v = Vec::new();
                if self.to_call() > 0 {
                    v.push(Move::Fold);
                }
                v.push(Move::Call);
                if self.can_raise() {
                    v.push(Move::Raise);
                }
                v
            }
            Phase::Deal => vec![Move::Deal],
            Phase::Showdown => vec![Move::Showdown],
            Phase::FoldEnd(_) => vec![Move::Settle],
        }
    }

    fn apply(&mut self, m: &Move) {
        let me = self.side;
        let o = me.other();
        match m {
            Move::Fold => {
                self.phase = Phase::FoldEnd(o);
            }
            Move::Call | Move::Raise => {
                let tc = self.to_call();
                if *m == Move::Raise {
                    let r = self.raise_size().min(self.stacks[me.idx()] - tc);
                    self.post(me, tc + r);
                    self.raises += 1;
                    self.acted[me.idx()] = true;
                    self.acted[o.idx()] = false;
                } else {
                    self.post(me, tc);
                    self.acted[me.idx()] = true;
                }
                let (sm, so) = (self.street_commit[me.idx()], self.street_commit[o.idx()]);
                let closed = self.acted[me.idx()]
                    && ((self.acted[o.idx()] && (sm == so || self.stacks[me.idx()] == 0))
                        || (self.stacks[o.idx()] == 0 && so <= sm));
                if closed {
                    self.end_round();
                } else {
                    self.side = o;
                }
            }
            Move::Deal => {
                let n = if self.street == 0 { 3 } else { 1 };
                for _ in 0..n {
                    self.board.push(self.deck.pop().unwrap());
                }
                self.street += 1;
                self.street_commit = [0; 2];
                self.raises = 0;
                self.acted = [false; 2];
                self.side = self.button.other();
                self.phase = Phase::Betting;
            }
            Move::Showdown => self.settle(None),
            Move::Settle => {
                if let Phase::FoldEnd(w) = self.phase {
                    self.settle(Some(w));
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            return None;
        }
        Some(match self.stacks[0].cmp(&self.stacks[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    /// Expected final chips for `p`, from `p`'s hole cards and the public
    /// board only; outstanding bets are assumed to be called.
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let o = 1 - me;
        if self.over {
            return self.stacks[me];
        }
        let matched_now = self.commit[0].min(self.commit[1]);
        if let Phase::FoldEnd(w) = self.phase {
            let refund = self.commit[me] - matched_now;
            return self.stacks[me] + refund + if w == p { 2 * matched_now } else { 0 };
        }
        let e = mc_equity(self.hole[me], &self.board);
        let mx = self.commit[0].max(self.commit[1]);
        let matched = mx
            .min(self.commit[me] + self.stacks[me])
            .min(self.commit[o] + self.stacks[o]);
        self.stacks[me] + (self.commit[me] - matched) + (e * 2.0 * matched as f64).round() as i32
    }

    fn render(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            "Poker Texas hold'em heads-up — mâna {}/{HANDS}, blinduri {SMALL_BLIND}/{BIG_BLIND}, buton: Jucător {}\n",
            self.hand_no.min(HANDS),
            self.button.idx() + 1
        ));
        let pot = self.commit[0] + self.commit[1];
        s.push_str(&format!(
            "Jetoane: J1 {} – J2 {} | pot {}\n",
            self.stacks[0], self.stacks[1], pot
        ));
        s.push_str(&format!(
            "Masă: {}\n",
            join_symbols(&self.board, std_symbol)
        ));
        let v = self.viewer.who(self.side);
        if self.over {
            s.push_str(&format!(
                "Cărțile tale (J{}): {}\n",
                v.idx() + 1,
                join_symbols(&self.hole[v.idx()], std_symbol)
            ));
        } else {
            s.push_str(&format!(
                "Cărțile tale (Jucător {}): {} — Jucător {}: [??] [??] (2 cărți ascunse)\n",
                v.idx() + 1,
                join_symbols(&self.hole[v.idx()], std_symbol),
                v.other().idx() + 1
            ));
        }
        if !self.last_result.is_empty() {
            s.push_str(&format!("{}\n", self.last_result));
        }
        if !self.over {
            match self.phase {
                Phase::Betting => s.push_str(&format!(
                    "La mutare: Jucător {} (de plată: {}; raise = {})\n",
                    self.side.idx() + 1,
                    self.to_call(),
                    self.raise_size()
                )),
                Phase::Deal => s.push_str(
                    "Runda de pariuri s-a încheiat: se împart cărțile (flop/turn/river)\n",
                ),
                Phase::Showdown => s.push_str("Showdown: scrie «showdown»\n"),
                Phase::FoldEnd(_) => {
                    s.push_str("Mâna s-a încheiat prin renunțare: scrie «decontare»\n")
                }
            }
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Fold => "fold".into(),
            Move::Call => if self.to_call() > 0 { "call" } else { "check" }.into(),
            Move::Raise => "raise".into(),
            Move::Deal => match self.street {
                0 => "flop",
                1 => "turn",
                _ => "river",
            }
            .into(),
            Move::Showdown => "showdown".into(),
            Move::Settle => "decontare".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// "As", "Kh", "Td", "9c", "2s" style helper (T = 10).
    fn cards(s: &str) -> Vec<u8> {
        s.split_whitespace()
            .map(|t| {
                let b: Vec<char> = t.chars().collect();
                let r = "23456789TJQKA".find(b[0]).unwrap() as u8;
                let su = "cdhs".find(b[1]).unwrap() as u8;
                su * 13 + r
            })
            .collect()
    }
    fn score(s: &str) -> u32 {
        best_of(&cards(s))
    }

    #[test]
    fn categories_are_recognised() {
        assert_eq!(category(score("As Ks Qs Js Ts")), 8); // royal flush
        assert_eq!(category(score("9h 8h 7h 6h 5h")), 8);
        assert_eq!(category(score("9c 9d 9h 9s 2c")), 7);
        assert_eq!(category(score("Kc Kd Kh 4s 4c")), 6);
        assert_eq!(category(score("2h 9h Jh 4h Ah")), 5);
        assert_eq!(category(score("5c 6d 7h 8s 9c")), 4);
        assert_eq!(category(score("Qc Qd Qh 4s 9c")), 3);
        assert_eq!(category(score("Qc Qd 4h 4s 9c")), 2);
        assert_eq!(category(score("Qc Qd 3h 4s 9c")), 1);
        assert_eq!(category(score("Qc 2d 3h 4s 9c")), 0);
    }

    #[test]
    fn category_order_is_correct() {
        let ladder = [
            "Qc 2d 3h 4s 9c",
            "Qc Qd 3h 4s 9c",
            "Qc Qd 4h 4s 9c",
            "Qc Qd Qh 4s 9c",
            "5c 6d 7h 8s 9c",
            "2h 9h Jh 4h Ah",
            "Kc Kd Kh 4s 4c",
            "9c 9d 9h 9s 2c",
            "9h 8h 7h 6h 5h",
            "As Ks Qs Js Ts",
        ];
        for w in ladder.windows(2) {
            assert!(score(w[0]) < score(w[1]), "{} < {}", w[0], w[1]);
        }
    }

    #[test]
    fn wheel_is_the_lowest_straight() {
        let wheel = score("Ac 2d 3h 4s 5c");
        assert_eq!(category(wheel), 4);
        assert!(wheel < score("2c 3d 4h 5s 6c"));
        assert!(wheel > score("Ac Ad Ah 4s 5c")); // trips < straight
                                                  // broadway beats everything below
        assert!(score("Tc Jd Qh Ks Ac") > score("9c Td Jh Qs Kc"));
        // no wrap-around: Q K A 2 3 is not a straight
        assert_eq!(category(score("Qc Kd Ah 2s 3c")), 0);
    }

    #[test]
    fn kickers_decide() {
        // pair of aces: kicker K beats kicker Q
        assert!(score("Ac Ad Kh 4s 9c") > score("Ah As Qh 4d 9d"));
        // identical hands tie
        assert_eq!(score("Ac Ad Kh 4s 9c"), score("Ah As Kd 4d 9d"));
        // two pair: higher top pair, then second pair, then kicker
        assert!(score("Kc Kd 3h 3s 9c") > score("Qc Qd Jh Js Ac"));
        assert!(score("Kc Kd 5h 5s 9c") > score("Kh Ks 4h 4d Ac"));
        assert!(score("Kc Kd 5h 5s Tc") > score("Kh Ks 5d 5c 9d"));
        // full house: trips rank first
        assert!(score("3c 3d 3h 2s 2c") > score("2h 2d 2s Ac Ad"));
        // flush: highest card first, then the next
        assert!(score("Ah 9h 7h 4h 2h") > score("Kh Qh Jh 9h 7h"));
        assert!(score("Ah Kh 7h 4h 2h") > score("Ah Qh Jh 9h 7h"));
        // quads: kicker
        assert!(score("8c 8d 8h 8s Ac") > score("8c 8d 8h 8s Kc"));
        // straight flush beats quads
        assert!(score("5h 6h 7h 8h 9h") > score("Ac Ad Ah As Kc"));
        // trips: kickers
        assert!(score("7c 7d 7h Ks 2c") > score("7c 7d 7h Qs Jc"));
    }

    #[test]
    fn best_of_seven_picks_the_best_five() {
        // board has a pair; hole cards make a flush
        let s = score("2h 7h Kh 9h 9c 3d Ad");
        assert_eq!(category(s), 1);
        let s = score("2h 7h Kh 9h 4h 3d Ad");
        assert_eq!(category(s), 5);
        // straight hidden in seven cards
        let s = score("2c 3d 4h 5s 6c Kd Kh");
        assert_eq!(category(s), 4);
        // full house from two trips: best trips + best pair
        let s = score("Kc Kd Kh 4s 4c 4d 9h");
        assert_eq!(category(s), 6);
        assert_eq!(s, score("Kc Kd Kh 4s 4c"));
        // royal on the board: split (both play the board)
        let board = "As Ks Qs Js Ts";
        assert_eq!(
            score(&format!("{board} 2c 3d")),
            score(&format!("{board} 9h 9d"))
        );
        // seven cards, exactly five is also fine
        assert_eq!(best_of(&cards("As Ks Qs Js Ts")), score("As Ks Qs Js Ts"));
        // higher straight within seven cards
        assert_eq!(score("3c 4d 5h 6s 7c 8d 9h"), score("5h 6s 7c 8d 9h"));
    }

    #[test]
    fn category_counts_over_all_five_card_hands() {
        // number of hands per category among the 2,598,960 five-card hands
        let expected = [
            1_302_540u32,
            1_098_240,
            123_552,
            54_912,
            10_200,
            5_108,
            3_744,
            624,
            40,
        ];
        let mut got = [0u32; 9];
        for a in 0..48u8 {
            for b in a + 1..49 {
                for c in b + 1..50 {
                    for d in c + 1..51 {
                        for e in d + 1..52 {
                            got[category(eval5(&[a, b, c, d, e]))] += 1;
                        }
                    }
                }
            }
        }
        assert_eq!(got, expected);
    }

    #[test]
    fn chips_are_conserved_in_random_matches() {
        for seed in 1..=60u64 {
            let mut g = Poker::with_seed(seed);
            let mut rng = Rng::new(seed * 31);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty(), "stuck\n{}", Game::render(&g));
                g.apply(&m[rng.below(m.len())]);
                let total = g.stacks[0] + g.stacks[1] + g.commit[0] + g.commit[1];
                assert_eq!(total, 2 * START_STACK);
                n += 1;
                assert!(n < 5000);
            }
            assert!(g.hand_no <= HANDS);
        }
    }

    /// Chips of each player including what is already in the pot.
    fn tot(g: &Poker) -> (i32, i32) {
        (g.stacks[0] + g.commit[0], g.stacks[1] + g.commit[1])
    }

    fn fixed_hand(h1: &str, h2: &str, board: &str) -> Poker {
        let mut g = Poker::with_seed(2);
        let a = cards(h1);
        let b = cards(h2);
        g.hole = [[a[0], a[1]], [b[0], b[1]]];
        let mut deck = cards(board);
        deck.reverse(); // popped from the end
        g.deck = deck;
        g
    }

    #[test]
    fn heads_up_betting_flow() {
        let mut g = fixed_hand("As Ad", "7c 2d", "Kh Qd 5s 9c 3h");
        // blinds posted: button (J1) is small blind
        assert_eq!(g.street_commit, [10, 20]);
        assert_eq!(g.to_move(), Player::One);
        assert_eq!(g.legal_moves(), vec![Move::Fold, Move::Call, Move::Raise]);
        g.apply(&Move::Call); // J1 completes the blind
        assert_eq!(g.to_move(), Player::Two); // big blind has the option
        assert_eq!(g.legal_moves(), vec![Move::Call, Move::Raise]); // check or raise
        assert_eq!(g.format_move(&Move::Call), "check");
        g.apply(&Move::Call);
        assert_eq!(g.legal_moves(), vec![Move::Deal]);
        g.apply(&Move::Deal);
        assert_eq!(g.board.len(), 3);
        assert_eq!(g.to_move(), Player::Two); // out of position first after the flop
        g.apply(&Move::Raise);
        assert_eq!(g.to_call(), 20);
        g.apply(&Move::Call);
        g.apply(&Move::Deal); // turn
        g.apply(&Move::Call);
        g.apply(&Move::Call);
        g.apply(&Move::Deal); // river
        assert_eq!(g.board.len(), 5);
        g.apply(&Move::Call);
        g.apply(&Move::Call);
        assert_eq!(g.legal_moves(), vec![Move::Showdown]);
        g.apply(&Move::Showdown);
        // pocket aces win 40 chips from the other player (20 + 20 raise)
        assert_eq!(tot(&g), (1040, 960));
        assert!(g.last_result.contains("Jucător 1 câștigă"));
        assert_eq!(g.hand_no, 2);
        assert_eq!(g.button, Player::Two);
    }

    #[test]
    fn fold_gives_the_pot_and_returns_uncalled_chips() {
        let mut g = fixed_hand("As Ad", "7c 2d", "Kh Qd 5s 9c 3h");
        g.apply(&Move::Raise); // J1 raises to 30 (20 more)
        assert_eq!(g.street_commit, [40, 20]); // call 10 + raise 20
        g.apply(&Move::Fold);
        assert_eq!(g.legal_moves(), vec![Move::Settle]);
        g.apply(&Move::Settle);
        assert_eq!(tot(&g), (1020, 980));
    }

    #[test]
    fn raise_cap_and_sizes() {
        let mut g = fixed_hand("As Ad", "7c 2d", "Kh Qd 5s 9c 3h");
        for _ in 0..3 {
            assert!(g.legal_moves().contains(&Move::Raise));
            g.apply(&Move::Raise);
        }
        assert!(!g.legal_moves().contains(&Move::Raise)); // cap reached
                                                          // blinds 10/20, then raises of 20: 40, 60, 80 -> the last raiser is at 80
        assert_eq!(g.street_commit, [80, 60]);
        assert_eq!(g.to_move(), Player::Two);
        assert_eq!(g.legal_moves(), vec![Move::Fold, Move::Call]);
    }

    #[test]
    fn all_in_runs_out_the_board_and_returns_excess() {
        let mut g = fixed_hand("As Ad", "7c 2d", "Kh Qd 5s 9c 3h");
        g.stacks = [30, 1000 - 20];
        g.commit = [10, 20];
        g.street_commit = [10, 20];
        let total0 = 30 + 980 + 10 + 20;
        // J1 (30 behind) calls 10 and has 20 left, then raises all-in, J2 calls
        g.apply(&Move::Call);
        g.apply(&Move::Raise);
        assert!(g.stacks[0] == 0 || g.legal_moves().contains(&Move::Call));
        // simply play it out
        let mut guard = 0;
        while g.hand_no == 1 {
            let m = g.legal_moves();
            let pick = if m.contains(&Move::Call) {
                Move::Call
            } else {
                m[0]
            };
            g.apply(&pick);
            guard += 1;
            assert!(guard < 50);
        }
        assert_eq!(
            g.stacks[0] + g.stacks[1] + g.commit[0] + g.commit[1],
            total0
        );
        // J1 can lose at most the 40 chips it had in front of it
        assert!(g.stacks[0] + g.commit[0] == 0 || g.stacks[0] + g.commit[0] >= 40 || g.over);
    }

    #[test]
    fn split_pot_on_equal_hands() {
        let mut g = fixed_hand("2c 3d", "2h 3s", "As Ks Qs Js Ts");
        let mut guard = 0;
        while g.hand_no == 1 {
            let m = g.legal_moves();
            let pick = if m.contains(&Move::Call) {
                Move::Call
            } else {
                m[0]
            };
            g.apply(&pick);
            guard += 1;
            assert!(guard < 50);
        }
        assert_eq!(tot(&g), (1000, 1000));
        assert!(g.last_result.contains("egalitate"));
    }

    #[test]
    fn render_hides_player_two() {
        let g = Poker::with_seed(11);
        let r = Game::render(&g);
        assert!(r.contains("[??] [??]"));
        for &c in &g.hole[1] {
            assert!(!r.contains(&std_symbol(c)) || g.hole[0].contains(&c));
        }
    }

    #[test]
    fn viewer_controls_whose_hole_cards_are_shown() {
        let mut g = Poker::with_seed(11);
        Game::set_viewer(&mut g, None);
        let mut movers = [false; 2];
        for _ in 0..2 {
            let tm = g.to_move().idx();
            movers[tm] = true;
            let r = Game::render(&g);
            for &c in &g.hole[tm] {
                assert!(r.contains(&std_symbol(c)), "{r}");
            }
            for &c in &g.hole[1 - tm] {
                assert!(!r.contains(&std_symbol(c)), "{r}");
            }
            g.apply(&Move::Call);
        }
        assert!(movers[0] && movers[1]);
        let mut g = Poker::with_seed(11);
        for (p, o) in [(Player::One, 1), (Player::Two, 0)] {
            Game::set_viewer(&mut g, Some(p));
            let r = Game::render(&g);
            for &c in &g.hole[p.idx()] {
                assert!(r.contains(&std_symbol(c)), "{r}");
            }
            for &c in &g.hole[o] {
                assert!(!r.contains(&std_symbol(c)), "{r}");
            }
        }
    }

    #[test]
    fn evaluate_ignores_hidden_cards() {
        let g = Poker::with_seed(21);
        let mut h = g.clone();
        // change the opponent's hole cards and the undealt deck
        h.hole[1] = [h.deck[0], h.deck[1]];
        h.deck.reverse();
        assert_eq!(g.evaluate(Player::One), h.evaluate(Player::One));
        // AI's raise does not reveal future cards: evaluate depends only on own cards
        let mut a = g.clone();
        let mut b = h.clone();
        a.apply(&Move::Raise);
        b.apply(&Move::Raise);
        assert_eq!(a.evaluate(Player::One), b.evaluate(Player::One));
    }

    #[test]
    fn equity_is_monotone_in_hand_strength() {
        let aa = equity([cards("As")[0], cards("Ad")[0]], &[]);
        let ak = equity([cards("As")[0], cards("Kd")[0]], &[]);
        let s72 = equity([cards("7s")[0], cards("2d")[0]], &[]);
        assert!(aa > ak && ak > s72);
        let board = cards("Kh 7d 2s");
        let trips = equity([cards("Kd")[0], cards("Kc")[0]], &board);
        let pair = equity([cards("Kd")[0], cards("3c")[0]], &board);
        let nothing = equity([cards("Jd")[0], cards("3c")[0]], &board);
        assert!(trips > pair && pair > nothing);
    }

    #[test]
    fn ai_beats_a_random_mover_mostly() {
        use polytabletop_core::GameObj;
        let n = 12;
        let mut wins = 0;
        for seed in 1..=n {
            let mut g: Box<dyn GameObj> = Box::new(Poker::with_seed(seed));
            let mut rng = Rng::new(seed + 5);
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

    #[test]
    fn fast_score_agrees_with_best_of() {
        let mut rng = Rng::new(5);
        for _ in 0..3000 {
            let mut d = std_deck(&mut rng);
            let a: Vec<u8> = d.drain(..7).collect();
            let b: Vec<u8> = d.drain(..7).collect();
            let (x, y) = (best_of(&a), best_of(&b));
            let (fx, fy) = (fast_score(&a), fast_score(&b));
            assert_eq!(category(x), category(fx), "{a:?}");
            assert_eq!(x.cmp(&y), fx.cmp(&fy), "{a:?} vs {b:?}");
        }
    }

    #[test]
    fn mc_equity_orders_hands_and_ignores_the_opponent() {
        let aa = mc_equity([cards("As")[0], cards("Ad")[0]], &[]);
        let s72 = mc_equity([cards("7s")[0], cards("2d")[0]], &[]);
        assert!(aa > 0.75 && s72 < 0.45 && aa > s72, "{aa} {s72}");
        let g = Poker::with_seed(11);
        let mut h = g.clone();
        h.hole[1] = [h.hole[1][1], h.hole[1][0]];
        h.deck.reverse();
        assert_eq!(g.evaluate(Player::One), h.evaluate(Player::One));
    }
}
