//! Burako (Buraco; Argentina / Uruguay / Brazilia), 2 jucători, cărți de joc.
//!
//! Variantă documentată (reguli publice ale Burako argentinian, adaptate la cărți):
//! * 106 cărți: două pachete de 52 + 2 jokeri. Fiecare primește 11 cărți și un **muerto**
//!   (mână moartă) de 11 cărți; restul = stoc; talonul (pozo) începe gol. Începe jucătorul care nu
//!   a împărțit.
//! * Rând: tragi o carte din stoc (`mazo`) **sau** iei tot pozo (`pozo`), eventual lași jos combinații
//!   sau adaugi la ale tale, apoi arunci o carte (`descarta 5h`) pe pozo.
//! * Combinații: **scară** (≥ 3 cărți consecutive din aceeași culoare; asul e jos sau sus: A-2-3 sau
//!   Q-K-A, fără ocol) și **pierna** (≥ 3 cărți de același rang, orice culori). Comodini: jokerii și
//!   cărțile de 2 (un 2 de culoarea scării pus pe locul lui, sub 3, e carte naturală). **Cel mult un
//!   comodin într-o combinație.** O combinație cu cel puțin 7 cărți e **canastă**: pură (fără comodin)
//!   200, impură 100.
//! * **Muerto**: cine rămâne fără cărți în mână (după lăsare jos sau după ce aruncă ultima
//!   carte) ia muertoul și joacă mai departe cu el (la aruncarea ultimei cărți îl ia la începutul
//!   rândului următor). Pentru a **închide** (a rămâne fără cărți a doua oară) trebuie să ai muertoul
//!   luat și cel puțin o canastă: +100 de puncte.
//! * Valori: A 15, 2 20, 3–7 5, 8–K 10, joker 50. Scor pe mână = cărțile de pe masă − cărțile rămase în
//!   mână + canaste (200/100) + 100 pentru închidere − 100 dacă muertoul nu a fost luat. Dacă stocul se
//!   epuizează (după aruncarea care îl golește) mâna se numără fără bonus de închidere; dacă nimeni nu
//!   are canastă, mâna nu se punctează.
//! * Meci: primul care ajunge la **1500**; ca înlocuitor al limitei de timp din turnee, dacă nimeni
//!   n-a ajuns, după **4 mâini** câștigă cel cu scorul mai mare (egal = remiză).
//!
//! Neimplementat (documentat): schimbul comodinului cu cartea naturală, mutarea comodinului în
//! combinație, scările „reale” (bonus 500/1000), alegerea de către împărțitor a primei cărți.
//! Pozo se poate lua oricând (în regulile argentiniene nu există minim de cărți).
//!
//! Mutări: `mazo`, `pozo`, `esc:4h+5h+Jk=6+7h` (scară nouă; comodinul poartă `=poziția`, asul sus = 14),
//! `set:7h+7c+Jk` (pierna), `+2:8h` (adaugă la combinația nr. 2 a ta; `+2:Jk=9` cu comodin;
//! asul la capete: `+1:Ah@14`), `descarta 5h`, `muerto`, `urmatoarea`.
//!
//! Informație ascunsă: mâna adversarului, muertoul, stocul. `render` arată doar mâna jucătorului
//! vizualizat; `evaluate` citește doar mâna proprie (fără cartea tocmai trasă din stoc), mesele,
//! pozo și numărul cărților adversarului.

use crate::cards::Viewer;
use crate::cards52::{shuffle, SUIT_LETTERS};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const JOKER: u8 = 52;
const TARGET: i32 = 1500;
const MAX_HANDS: u8 = 4;
const TURN_CAP: u16 = 400;
const RANKS: [&str; 13] = [
    "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
];

fn rank(t: u8) -> u8 {
    t % 13 + 1
}
fn suit(t: u8) -> u8 {
    t / 13
}
fn cval(t: u8) -> i32 {
    if t == JOKER {
        return 50;
    }
    match rank(t) {
        1 => 15,
        2 => 20,
        3..=7 => 5,
        _ => 10,
    }
}
pub fn text(t: u8) -> String {
    if t == JOKER {
        "Jk".into()
    } else {
        format!(
            "{}{}",
            RANKS[rank(t) as usize - 1],
            SUIT_LETTERS[suit(t) as usize]
        )
    }
}
fn join(cards: &[u8]) -> String {
    if cards.is_empty() {
        return "—".into();
    }
    cards.iter().map(|&c| text(c)).collect::<Vec<_>>().join(" ")
}

/// Comodin în acest loc? (`set`: cheia = rang; scară: cheia = culoare)
fn wild_at(set: bool, key: u8, pos: u8, c: u8) -> bool {
    if c == JOKER {
        return true;
    }
    if rank(c) == 2 {
        if set {
            key != 2
        } else {
            !(suit(c) == key && pos == 2)
        }
    } else {
        false
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Meld {
    set: bool,
    key: u8,
    /// (poziție, carte), sortat după poziție; la piernă poziția = rang.
    slots: Vec<(u8, u8)>,
}

impl Meld {
    fn wilds(&self) -> usize {
        self.slots
            .iter()
            .filter(|&&(p, c)| wild_at(self.set, self.key, p, c))
            .count()
    }
    fn canasta(&self) -> bool {
        self.slots.len() >= 7
    }
    fn clean(&self) -> bool {
        self.wilds() == 0
    }
    fn value(&self) -> i32 {
        self.slots.iter().map(|&(_, c)| cval(c)).sum()
    }
    fn bonus(&self) -> i32 {
        if !self.canasta() {
            0
        } else if self.clean() {
            200
        } else {
            100
        }
    }
    fn lo(&self) -> u8 {
        self.slots.first().map(|s| s.0).unwrap_or(0)
    }
    fn hi(&self) -> u8 {
        self.slots.last().map(|s| s.0).unwrap_or(0)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Move {
    DrawStock,
    TakePile,
    Lay {
        set: bool,
        slots: Vec<(u8, u8)>,
        key: u8,
    },
    Add {
        idx: usize,
        pos: u8,
        card: u8,
    },
    Discard(u8),
    TakeMuerto,
    Next,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Draw,
    Act,
    /// `true` = în mijlocul rândului (cumpărare directă), `false` = la începutul rândului următor.
    Muerto(bool),
    RoundOver,
}

#[derive(Clone)]
pub struct Burako {
    rng: Rng,
    viewer: Viewer,
    stock: Vec<u8>,
    pile: Vec<u8>,
    hands: [Vec<u8>; 2],
    muerto: [Vec<u8>; 2],
    taken: [bool; 2],
    pending: [bool; 2],
    melds: [Vec<Meld>; 2],
    side: Player,
    dealer: Player,
    phase: Phase,
    fresh: Option<u8>,
    closer: Option<Player>,
    score: [i32; 2],
    prev: [i32; 2],
    hand_no: u8,
    turns: u16,
    log: String,
    done: bool,
}

impl Default for Burako {
    fn default() -> Self {
        Self::new()
    }
}

fn counts(h: &[u8]) -> [u8; 53] {
    let mut c = [0u8; 53];
    for &x in h {
        c[x as usize] += 1;
    }
    c
}

impl Burako {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Burako {
            rng: Rng::new(seed),
            viewer: Viewer::Fixed(Player::One),
            stock: Vec::new(),
            pile: Vec::new(),
            hands: [Vec::new(), Vec::new()],
            muerto: [Vec::new(), Vec::new()],
            taken: [false; 2],
            pending: [false; 2],
            melds: [Vec::new(), Vec::new()],
            side: Player::One,
            dealer: Player::Two,
            phase: Phase::Draw,
            fresh: None,
            closer: None,
            score: [0; 2],
            prev: [0; 2],
            hand_no: 1,
            turns: 0,
            log: String::new(),
            done: false,
        };
        g.deal();
        g
    }

    pub fn scores(&self) -> (i32, i32) {
        (self.score[0], self.score[1])
    }

    fn deal(&mut self) {
        let mut d: Vec<u8> = (0..2).flat_map(|_| 0..52u8).collect();
        d.push(JOKER);
        d.push(JOKER);
        shuffle(&mut d, &mut self.rng);
        for p in 0..2 {
            self.hands[p] = d.drain(..11).collect();
            self.muerto[p] = d.drain(..11).collect();
            self.hands[p].sort_unstable();
        }
        self.stock = d;
        self.pile.clear();
        self.taken = [false; 2];
        self.pending = [false; 2];
        self.melds = [Vec::new(), Vec::new()];
        self.side = self.dealer.other();
        self.phase = Phase::Draw;
        self.fresh = None;
        self.closer = None;
        self.turns = 0;
        self.prev = self.score;
    }

    fn has_canasta(&self, p: usize) -> bool {
        self.melds[p].iter().any(|m| m.canasta())
    }

    fn table_value(&self, p: usize) -> i32 {
        self.melds[p].iter().map(|m| m.value()).sum()
    }

    fn hand_value(&self, p: usize) -> i32 {
        self.hands[p].iter().map(|&c| cval(c)).sum()
    }

    fn round_score(&self, p: usize) -> i32 {
        let mut s = self.table_value(p) - self.hand_value(p);
        s += self.melds[p].iter().map(|m| m.bonus()).sum::<i32>();
        if self.closer == Some(if p == 0 { Player::One } else { Player::Two }) {
            s += 100;
        }
        if !self.taken[p] {
            s -= 100;
        }
        s
    }

    fn end_round(&mut self) {
        let any_canasta = self.has_canasta(0) || self.has_canasta(1);
        let mut r = [0; 2];
        if self.closer.is_some() || any_canasta {
            r = [self.round_score(0), self.round_score(1)];
        }
        self.score[0] += r[0];
        self.score[1] += r[1];
        self.log = format!(
            "Mâna {} încheiată{}: J1 {:+}, J2 {:+}.",
            self.hand_no,
            match self.closer {
                Some(p) => format!(" (închide J{})", p.idx() + 1),
                None => " (stoc epuizat)".into(),
            },
            r[0],
            r[1]
        );
        self.phase = Phase::RoundOver;
        let reached = self.score[0] >= TARGET || self.score[1] >= TARGET;
        if (reached && self.score[0] != self.score[1]) || self.hand_no >= MAX_HANDS {
            self.done = true;
        }
    }

    fn after_empty(&mut self, me: usize, direct: bool) {
        // mâna a rămas goală după lăsare jos
        if !self.taken[me] {
            self.phase = Phase::Muerto(direct);
        } else {
            self.closer = Some(self.side);
            self.end_round();
        }
    }

    fn run_options(&self, h: &[u8], out: &mut Vec<Move>) {
        let cnt = counts(h);
        let wild_types: Vec<u8> = {
            let mut w = Vec::new();
            if cnt[JOKER as usize] > 0 {
                w.push(JOKER);
            }
            for s in 0..4u8 {
                if cnt[(s * 13 + 1) as usize] > 0 {
                    w.push(s * 13 + 1);
                }
            }
            w
        };
        for s in 0..4u8 {
            for lo in 1..=12u8 {
                for hi in (lo + 2)..=14u8 {
                    // naturale disponibile pe fiecare poziție (asul poate fi pe 1 și pe 14)
                    let mut need = [0u8; 53];
                    let mut missing: Vec<u8> = Vec::new();
                    for pos in lo..=hi {
                        let t = s * 13 + if pos == 14 { 0 } else { pos - 1 };
                        if need[t as usize] < cnt[t as usize] {
                            need[t as usize] += 1;
                        } else {
                            missing.push(pos);
                        }
                    }
                    if missing.len() > 1 {
                        continue;
                    }
                    let build = |wild: Option<(u8, u8)>| -> Vec<(u8, u8)> {
                        let mut v = Vec::new();
                        for pos in lo..=hi {
                            if let Some((wp, wc)) = wild {
                                if wp == pos {
                                    v.push((pos, wc));
                                    continue;
                                }
                            }
                            let t = s * 13 + if pos == 14 { 0 } else { pos - 1 };
                            v.push((pos, t));
                        }
                        v
                    };
                    if missing.is_empty() {
                        out.push(Move::Lay {
                            set: false,
                            key: s,
                            slots: build(None),
                        });
                    } else {
                        let wp = missing[0];
                        for &w in &wild_types {
                            // aceeași carte nu poate fi folosită și ca naturală și ca comodin peste stoc
                            let mut used = need;
                            used[w as usize] += 1;
                            if used[w as usize] > cnt[w as usize] {
                                continue;
                            }
                            out.push(Move::Lay {
                                set: false,
                                key: s,
                                slots: build(Some((wp, w))),
                            });
                        }
                    }
                }
            }
        }
    }

    fn set_options(&self, h: &[u8], out: &mut Vec<Move>) {
        let cnt = counts(h);
        let mut wilds: Vec<Option<u8>> = vec![None];
        if cnt[JOKER as usize] > 0 {
            wilds.push(Some(JOKER));
        }
        for r in 1..=13u8 {
            let avail: Vec<u8> = (0..4u8).map(|s| cnt[(s * 13 + r - 1) as usize]).collect();
            let mut w = wilds.clone();
            if r != 2 {
                for s in 0..4u8 {
                    if cnt[(s * 13 + 1) as usize] > 0 {
                        w.push(Some(s * 13 + 1));
                    }
                }
            }
            for a0 in 0..=avail[0] {
                for a1 in 0..=avail[1] {
                    for a2 in 0..=avail[2] {
                        for a3 in 0..=avail[3] {
                            let n = (a0 + a1 + a2 + a3) as usize;
                            for wc in &w {
                                let total = n + usize::from(wc.is_some());
                                if total < 3 || n < 2 {
                                    continue;
                                }
                                let mut slots: Vec<(u8, u8)> = Vec::new();
                                for (s, a) in [a0, a1, a2, a3].into_iter().enumerate() {
                                    for _ in 0..a {
                                        slots.push((r, s as u8 * 13 + r - 1));
                                    }
                                }
                                if let Some(x) = wc {
                                    // verifică disponibilitatea comodinului (alt exemplar decât naturalele)
                                    let taken = slots.iter().filter(|s| s.1 == *x).count() as u8;
                                    if taken >= cnt[*x as usize] {
                                        continue;
                                    }
                                    slots.push((r, *x));
                                }
                                out.push(Move::Lay {
                                    set: true,
                                    key: r,
                                    slots,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    fn add_options(&self, me: usize, h: &[u8], out: &mut Vec<Move>) {
        let cnt = counts(h);
        for (idx, m) in self.melds[me].iter().enumerate() {
            let has_wild = m.wilds() > 0;
            if m.set {
                let r = m.key;
                for s in 0..4u8 {
                    let t = s * 13 + r - 1;
                    if cnt[t as usize] > 0 && m.slots.len() < 20 {
                        out.push(Move::Add {
                            idx,
                            pos: r,
                            card: t,
                        });
                    }
                }
                if !has_wild {
                    if cnt[JOKER as usize] > 0 {
                        out.push(Move::Add {
                            idx,
                            pos: r,
                            card: JOKER,
                        });
                    }
                    if r != 2 {
                        for s in 0..4u8 {
                            if cnt[(s * 13 + 1) as usize] > 0 {
                                out.push(Move::Add {
                                    idx,
                                    pos: r,
                                    card: s * 13 + 1,
                                });
                            }
                        }
                    }
                }
            } else {
                let s = m.key;
                let mut ends = Vec::new();
                if m.lo() > 1 {
                    ends.push(m.lo() - 1);
                }
                if m.hi() < 14 {
                    ends.push(m.hi() + 1);
                }
                for pos in ends {
                    let t = s * 13 + if pos == 14 { 0 } else { pos - 1 };
                    if cnt[t as usize] > 0 {
                        out.push(Move::Add { idx, pos, card: t });
                    }
                    if !has_wild {
                        if cnt[JOKER as usize] > 0 {
                            out.push(Move::Add {
                                idx,
                                pos,
                                card: JOKER,
                            });
                        }
                        for s2 in 0..4u8 {
                            let two = s2 * 13 + 1;
                            if cnt[two as usize] > 0 && !(two == t) {
                                out.push(Move::Add {
                                    idx,
                                    pos,
                                    card: two,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    /// După mutarea care lasă `left` cărți în mână, e legală?
    fn leaves_ok(&self, me: usize, left: usize, canasta_after: bool) -> bool {
        left >= 2 || !self.taken[me] || canasta_after
    }
}

impl Game for Burako {
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
        if self.done && self.phase == Phase::RoundOver {
            return Vec::new();
        }
        let me = self.side.idx();
        match self.phase {
            Phase::RoundOver => vec![Move::Next],
            Phase::Muerto(_) => vec![Move::TakeMuerto],
            Phase::Draw => {
                let mut v = Vec::new();
                if !self.stock.is_empty() {
                    v.push(Move::DrawStock);
                }
                if !self.pile.is_empty() {
                    v.push(Move::TakePile);
                }
                v
            }
            Phase::Act => {
                let h = &self.hands[me];
                let mut v = Vec::new();
                let mut lays = Vec::new();
                self.set_options(h, &mut lays);
                self.run_options(h, &mut lays);
                for m in lays {
                    if let Move::Lay { slots, .. } = &m {
                        let left = h.len() - slots.len();
                        let will = self.has_canasta(me) || slots.len() >= 7;
                        if self.leaves_ok(me, left, will) {
                            v.push(m);
                        }
                    }
                }
                let mut adds = Vec::new();
                self.add_options(me, h, &mut adds);
                for m in adds {
                    if let Move::Add { idx, .. } = &m {
                        let left = h.len() - 1;
                        let will =
                            self.has_canasta(me) || self.melds[me][*idx].slots.len() + 1 >= 7;
                        if self.leaves_ok(me, left, will) {
                            v.push(m);
                        }
                    }
                }
                let mut seen = [false; 53];
                for &c in h {
                    if seen[c as usize] {
                        continue;
                    }
                    seen[c as usize] = true;
                    // la aruncare mâna poate rămâne cu 1 carte (altfel s-ar bloca jocul)
                    if self.leaves_ok(me, h.len(), self.has_canasta(me)) {
                        v.push(Move::Discard(c));
                    }
                }
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        let me = self.side.idx();
        match m {
            Move::Next => {
                self.hand_no += 1;
                self.dealer = self.dealer.other();
                self.log.clear();
                self.deal();
            }
            Move::DrawStock => {
                let c = self.stock.pop().expect("stock");
                self.hands[me].push(c);
                self.fresh = Some(c);
                self.phase = Phase::Act;
            }
            Move::TakePile => {
                let mut p = std::mem::take(&mut self.pile);
                self.hands[me].append(&mut p);
                self.fresh = None;
                self.phase = Phase::Act;
            }
            Move::TakeMuerto => {
                let direct = matches!(self.phase, Phase::Muerto(true));
                let mut mu = std::mem::take(&mut self.muerto[me]);
                self.hands[me].append(&mut mu);
                self.hands[me].sort_unstable();
                self.taken[me] = true;
                self.pending[me] = false;
                self.phase = if direct { Phase::Act } else { Phase::Draw };
            }
            Move::Lay { set, key, slots } => {
                for &(_, c) in slots {
                    let i = self.hands[me].iter().position(|&x| x == c).expect("card");
                    self.hands[me].remove(i);
                }
                self.melds[me].push(Meld {
                    set: *set,
                    key: *key,
                    slots: slots.clone(),
                });
                self.fresh = None;
                if self.hands[me].is_empty() {
                    self.after_empty(me, true);
                }
            }
            Move::Add { idx, pos, card } => {
                let i = self.hands[me].iter().position(|x| x == card).expect("card");
                self.hands[me].remove(i);
                let meld = &mut self.melds[me][*idx];
                meld.slots.push((*pos, *card));
                meld.slots.sort_by_key(|s| s.0);
                self.fresh = None;
                if self.hands[me].is_empty() {
                    self.after_empty(me, true);
                }
            }
            Move::Discard(c) => {
                let i = self.hands[me].iter().position(|x| x == c).expect("card");
                self.hands[me].remove(i);
                self.pile.push(*c);
                self.fresh = None;
                self.turns += 1;
                if self.hands[me].is_empty() {
                    if self.taken[me] {
                        self.closer = Some(self.side);
                        self.end_round();
                        return;
                    }
                    self.pending[me] = true;
                }
                if self.stock.is_empty() || self.turns >= TURN_CAP {
                    self.end_round();
                    return;
                }
                self.side = self.side.other();
                self.phase = if self.pending[self.side.idx()] {
                    Phase::Muerto(false)
                } else {
                    Phase::Draw
                };
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.done || self.phase != Phase::RoundOver {
            return None;
        }
        Some(match self.score[0].cmp(&self.score[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let o = 1 - me;
        let mut known: Vec<u8> = self.hands[me].clone();
        if self.side == p {
            if let Some(f) = self.fresh {
                if let Some(i) = known.iter().position(|&x| x == f) {
                    known.remove(i);
                }
            }
        }
        let mine = |hand: &[u8], who: usize| -> f64 {
            let mut v = self.table_value(who) as f64;
            v += self.melds[who].iter().map(|m| m.bonus()).sum::<i32>() as f64;
            v += if self.taken[who] { 100.0 } else { 0.0 };
            v -= 0.3 * hand.iter().map(|&c| cval(c)).sum::<i32>() as f64;
            // progres spre canastă: combinațiile lungi valorează mai mult
            v += self.melds[who]
                .iter()
                .map(|m| {
                    let n = m.slots.len().min(7) as f64;
                    0.8 * n * n
                })
                .sum::<f64>();
            // potențial de combinații în mâna cunoscută
            let cnt = counts(hand);
            let mut pot = 0.0;
            for s in 0..4usize {
                for r in 0..13usize {
                    let c = cnt[s * 13 + r];
                    if c >= 2 {
                        pot += 8.0;
                    }
                    if r + 1 < 13 && c > 0 && cnt[s * 13 + r + 1] > 0 {
                        pot += 6.0;
                    }
                    if r + 2 < 13 && c > 0 && cnt[s * 13 + r + 2] > 0 {
                        pot += 3.0;
                    }
                }
            }
            for r in 0..13usize {
                let n: u8 = (0..4).map(|s| cnt[s * 13 + r]).sum();
                if n >= 2 {
                    pot += 2.0;
                }
            }
            v + pot
        };
        let opp_hand_est = self.hands[o].len() as f64 * 8.0;
        let mut v = mine(&known, me);
        let mut w = mine(&[], o) - 0.6 * opp_hand_est;
        if self.phase == Phase::RoundOver {
            v = 0.0;
            w = 0.0;
            if let Some(c) = self.closer {
                if c == p {
                    v += 300.0;
                } else {
                    w += 300.0;
                }
            }
        }
        // pozo: cărți publice; luarea ei apare deja în mâna cunoscută
        let diff = (self.prev[me] - self.prev[o]) as f64 * 3.0;
        (v - w + diff).round() as i32
    }

    fn render(&self) -> String {
        let v = self.viewer.who(self.side);
        let reveal = self.phase == Phase::RoundOver;
        let mut s = format!(
            "Burako — mâna {}/{MAX_HANDS} (până la {TARGET}): J1 {} – J2 {}\n",
            self.hand_no, self.score[0], self.score[1]
        );
        s.push_str(&format!(
            "Stoc: {} cărți; pozo ({}): {}\n",
            self.stock.len(),
            self.pile.len(),
            if self.pile.is_empty() {
                "—".into()
            } else {
                let k = self.pile.len().min(8);
                format!(
                    "{}{}",
                    if self.pile.len() > k { "… " } else { "" },
                    join(&self.pile[self.pile.len() - k..])
                )
            }
        ));
        for p in 0..2 {
            let ms: Vec<String> = self.melds[p]
                .iter()
                .enumerate()
                .map(|(i, m)| {
                    format!(
                        "{}) {}{} [{}]",
                        i + 1,
                        if m.set { "pierna " } else { "scară " },
                        m.slots
                            .iter()
                            .map(|&(pos, c)| if wild_at(m.set, m.key, pos, c) {
                                format!("{}={}", text(c), pos)
                            } else {
                                text(c)
                            })
                            .collect::<Vec<_>>()
                            .join(" "),
                        if m.canasta() {
                            if m.clean() {
                                "canastă pură"
                            } else {
                                "canastă impură"
                            }
                        } else {
                            "—"
                        }
                    )
                })
                .collect();
            s.push_str(&format!(
                "Masa J{}: {} (muerto {}; {} cărți în mână)\n",
                p + 1,
                if ms.is_empty() {
                    "—".into()
                } else {
                    ms.join("; ")
                },
                if self.taken[p] { "luat" } else { "neluat" },
                self.hands[p].len()
            ));
        }
        if reveal {
            for p in 0..2 {
                s.push_str(&format!("Mâna J{}: {}\n", p + 1, join(&self.hands[p])));
            }
        } else {
            s.push_str(&format!(
                "Mâna ta (Jucător {}): {}\n",
                v.idx() + 1,
                join(&self.hands[v.idx()])
            ));
        }
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        match (self.outcome(), self.phase) {
            (Some(Outcome::Win(p)), _) => s.push_str(&format!(
                "Meci încheiat: Jucător {} câștigă.\n",
                p.idx() + 1
            )),
            (Some(Outcome::Draw), _) => s.push_str("Meci încheiat: remiză.\n"),
            (_, Phase::RoundOver) => s.push_str("Mâna s-a încheiat. Scrie «urmatoarea».\n"),
            (_, Phase::Muerto(_)) => s.push_str("Mâna ta e goală: scrie «muerto».\n"),
            (_, Phase::Draw) => s.push_str(&format!(
                "La mutare: Jucător {} — mazo / pozo\n",
                self.side.idx() + 1
            )),
            (_, Phase::Act) => s.push_str(&format!(
                "La mutare: Jucător {} — lasă combinații / adaugă / descarta\n",
                self.side.idx() + 1
            )),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::DrawStock => "mazo".into(),
            Move::TakePile => "pozo".into(),
            Move::TakeMuerto => "muerto".into(),
            Move::Next => "urmatoarea".into(),
            Move::Discard(c) => format!("descarta {}", text(*c)),
            Move::Lay { set, key, slots } => {
                let toks: Vec<String> = slots
                    .iter()
                    .map(|&(pos, c)| {
                        if !*set && wild_at(false, *key, pos, c) {
                            format!("{}={}", text(c), pos)
                        } else {
                            text(c)
                        }
                    })
                    .collect();
                format!("{}:{}", if *set { "set" } else { "esc" }, toks.join("+"))
            }
            Move::Add { idx, pos, card } => {
                let meld = &self.melds[self.side.idx()][*idx];
                let tok = if !meld.set && wild_at(false, meld.key, *pos, *card) {
                    format!("{}={}", text(*card), pos)
                } else if !meld.set && rank(*card) == 1 && *card != JOKER {
                    format!("{}@{}", text(*card), pos)
                } else {
                    text(*card)
                };
                format!("+{}:{}", idx + 1, tok)
            }
        }
    }

    fn ai_style(&self) -> AiStyle {
        // Adâncimea 0 = o singură mutare la TOATE nivelurile: la Hard/Expert adâncimea 1 ar extinde
        // răspunsurile adversarului din mâna lui reală (informație ascunsă).
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // c = 0..13 (trefle A..K), d = 13.., h = 26.., s = 39..
    fn c(s: u8, r: u8) -> u8 {
        s * 13 + r - 1
    }

    fn base() -> Burako {
        let mut g = Burako::with_seed(1);
        g.hands = [Vec::new(), Vec::new()];
        g.stock = vec![c(0, 9); 5];
        g.pile.clear();
        g.melds = [Vec::new(), Vec::new()];
        g.taken = [true; 2];
        g.phase = Phase::Act;
        g.side = Player::One;
        g.fresh = None;
        g
    }

    fn texts(g: &Burako) -> Vec<String> {
        g.legal_moves().iter().map(|m| g.format_move(m)).collect()
    }

    #[test]
    fn deal_sizes_and_card_count() {
        let g = Burako::with_seed(3);
        assert_eq!(g.hands[0].len(), 11);
        assert_eq!(g.muerto[1].len(), 11);
        assert_eq!(g.stock.len(), 106 - 44);
        assert_eq!(g.to_move(), Player::One);
    }

    #[test]
    fn runs_wilds_and_limits() {
        let mut g = base();
        g.hands[0] = vec![
            c(2, 4),
            c(2, 5),
            c(2, 6),
            c(2, 8),
            JOKER,
            c(0, 9),
            c(0, 9),
            c(1, 12),
            c(3, 13),
        ];
        let t = texts(&g);
        assert!(t.contains(&"esc:4h+5h+6h".to_string()));
        assert!(t.contains(&"esc:4h+5h+6h+Jk=7+8h".to_string()));
        assert!(!t
            .iter()
            .any(|x| x.starts_with("esc:") && x.matches("Jk").count() > 1));
        // un singur comodin: 4-5-_-_-8 nu e posibil
        assert!(!t.iter().any(|x| x == "esc:4h+5h+Jk=6+Jk=7+8h"));
    }

    #[test]
    fn two_of_same_suit_is_natural_below_the_three() {
        let mut g = base();
        g.hands[0] = vec![c(2, 2), c(2, 3), c(2, 4), JOKER, c(3, 8), c(3, 9)];
        let t = texts(&g);
        assert!(t.contains(&"esc:2h+3h+4h".to_string()));
        // și cu un comodin în plus peste al doilea: A (joker) 2 3 4
        assert!(t.contains(&"esc:Jk=1+2h+3h+4h".to_string()));
        // un 2 de altă culoare e comodin
        g.hands[0] = vec![c(1, 2), c(2, 3), c(2, 4), c(2, 5), c(3, 8), c(3, 9)];
        let t2 = texts(&g);
        assert!(
            t2.contains(&"esc:2d=2+3h+4h+5h".to_string())
                || t2.contains(&"esc:3h+4h+5h".to_string())
        );
        assert!(t2.contains(&"esc:2d=2+3h+4h+5h".to_string()));
    }

    #[test]
    fn ace_low_and_high_but_no_wraparound() {
        let mut g = base();
        g.hands[0] = vec![c(2, 12), c(2, 13), c(2, 1), c(2, 2), c(2, 3), c(3, 7)];
        let t = texts(&g);
        assert!(t.contains(&"esc:Qh+Kh+Ah".to_string()));
        assert!(t.contains(&"esc:Ah+2h+3h".to_string()));
        assert!(!t.iter().any(|x| x.contains("Kh+Ah+2h")));
    }

    #[test]
    fn sets_with_one_wild_and_extension() {
        let mut g = base();
        g.hands[0] = vec![c(0, 7), c(1, 7), JOKER, c(2, 7), c(3, 9)];
        let t = texts(&g);
        assert!(t.contains(&"set:7c+7d+7h".to_string()));
        assert!(t.contains(&"set:7c+7d+Jk".to_string()));
        g.melds[0].push(Meld {
            set: true,
            key: 7,
            slots: vec![(7, c(0, 7)), (7, c(1, 7)), (7, c(2, 7))],
        });
        g.hands[0] = vec![c(3, 7), c(3, 9), c(2, 5)];
        let t = texts(&g);
        assert!(t.contains(&"+1:7s".to_string()));
    }

    #[test]
    fn canasta_scoring_clean_and_dirty() {
        let clean = Meld {
            set: false,
            key: 2,
            slots: (3..=9).map(|p| (p, 2 * 13 + p - 1)).collect(),
        };
        assert!(clean.canasta() && clean.clean());
        assert_eq!(clean.bonus(), 200);
        let mut dirty = clean.clone();
        dirty.slots[2] = (5, JOKER);
        assert!(!dirty.clean());
        assert_eq!(dirty.bonus(), 100);
    }

    #[test]
    fn muerto_direct_closing_and_penalty() {
        // lasă jos tot cu muertoul netras -> primește muertoul
        let mut g = base();
        g.taken = [false, true];
        g.hands[0] = vec![c(2, 4), c(2, 5), c(2, 6)];
        g.apply(&Move::Lay {
            set: false,
            key: 2,
            slots: vec![(4, c(2, 4)), (5, c(2, 5)), (6, c(2, 6))],
        });
        assert_eq!(g.phase, Phase::Muerto(true));
        g.apply(&Move::TakeMuerto);
        assert_eq!(g.hands[0].len(), 11);
        assert_eq!(g.phase, Phase::Act);
        // penalizare −100 dacă muertoul nu a fost luat
        let mut h = base();
        h.taken = [false, true];
        h.melds[1].push(Meld {
            set: true,
            key: 5,
            slots: vec![(5, c(0, 5)), (5, c(1, 5)), (5, c(2, 5))],
        });
        h.closer = Some(Player::Two);
        h.end_round();
        assert_eq!(h.score[0], -100);
        assert_eq!(h.score[1], 115);
    }

    #[test]
    fn cannot_empty_the_hand_without_a_canasta_after_the_muerto() {
        let mut g = base();
        g.hands[0] = vec![c(2, 4), c(2, 5), c(2, 6)];
        let t = texts(&g);
        assert!(!t.contains(&"esc:4h+5h+6h".to_string())); // ar rămâne cu 0 cărți fără canastă
        assert!(t.iter().any(|x| x.starts_with("descarta")));
        // doar două cărți: nu poți arunca una (rămâi cu 1) ... dar poți, e ≥ 1; ultima nu
        g.hands[0] = vec![c(2, 4)];
        assert!(g.legal_moves().is_empty() || !texts(&g).iter().any(|x| x.starts_with("descarta")));
    }

    #[test]
    fn closing_with_a_canasta_ends_the_hand() {
        let mut g = base();
        g.melds[0].push(Meld {
            set: false,
            key: 2,
            slots: (3..=9).map(|p| (p, 2 * 13 + p - 1)).collect(),
        });
        g.hands[0] = vec![c(0, 12)];
        g.apply(&Move::Discard(c(0, 12)));
        assert_eq!(g.phase, Phase::RoundOver);
        assert_eq!(g.closer, Some(Player::One));
        // 3..9 de cupă: 5*5 + 10+10 ... valoare: 3,4,5,6,7 = 25; 8,9 = 20 -> 45; +200 +100
        assert_eq!(g.score[0], 45 + 200 + 100);
        assert_eq!(g.score[1], 0);
    }

    #[test]
    fn random_matches_finish() {
        for seed in 1..=4u64 {
            let mut g = Burako::with_seed(seed);
            let mut rng = Rng::new(seed * 13);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty(), "stuck\n{}", Game::render(&g));
                let mut seen = std::collections::HashSet::new();
                for x in &m {
                    assert!(
                        seen.insert(g.format_move(x)),
                        "dup move text {}",
                        g.format_move(x)
                    );
                }
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 6000, "too long");
            }
        }
    }

    #[test]
    fn evaluate_ignores_fresh_stock_card() {
        let mut g = base();
        g.phase = Phase::Draw;
        g.hands[0] = vec![c(0, 4), c(1, 9), c(2, 12)];
        g.hands[1] = vec![c(3, 4), c(1, 1), c(2, 2)];
        g.stock = vec![c(0, 7), c(3, 3)];
        let mut a = g.clone();
        a.apply(&Move::DrawStock);
        let mut b = g.clone();
        b.stock = vec![c(3, 3), c(0, 7)];
        b.apply(&Move::DrawStock);
        assert_eq!(a.evaluate(Player::One), b.evaluate(Player::One));
    }

    #[test]
    fn ai_beats_random_in_points() {
        use polytabletop_core::GameObj;
        let mut wins = 0;
        let n = 6;
        for seed in 1..=n {
            let mut g: Box<dyn GameObj> = Box::new(Burako::with_seed(seed));
            let mut rng = Rng::new(seed + 3);
            let mut plies = 0;
            while g.outcome().is_none() && plies < 8000 {
                let mv = if g.to_move() == Player::One {
                    g.ai_move(rng.next_u64()).unwrap()
                } else {
                    let m = g.legal_moves();
                    m[rng.below(m.len())].clone()
                };
                g.play(&mv).unwrap();
                plies += 1;
            }
            if g.outcome() == Some(Outcome::Win(Player::One)) {
                wins += 1;
            }
        }
        assert!(wins * 3 >= n * 2, "AI won only {wins}/{n}");
    }
}

#[cfg(test)]
mod a3_audit {
    use super::*;

    #[test]
    fn ai_ignores_hidden_cards_at_every_level() {
        crate::p3_comun::ai_blind_check(
            "burako",
            Burako::with_seed,
            |g, me, s| {
                let h = 1 - me.idx();
                let (nh, nmh, nmm) = (
                    g.hands[h].len(),
                    g.muerto[h].len(),
                    g.muerto[me.idx()].len(),
                );
                let mut pool: Vec<u8> = g.hands[h].clone();
                pool.extend(g.muerto[h].iter().copied());
                pool.extend(g.muerto[me.idx()].iter().copied());
                pool.extend(g.stock.iter().copied());
                crate::p3_comun::shuffle_with(&mut pool, s);
                g.hands[h] = pool.drain(..nh).collect();
                g.hands[h].sort_unstable();
                g.muerto[h] = pool.drain(..nmh).collect();
                g.muerto[me.idx()] = pool.drain(..nmm).collect();
                g.stock = pool;
            },
            3,
        );
    }
}
