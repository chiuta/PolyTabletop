//! Belote (Franța) pentru 2 jucători — variantă „belote clasică” adaptată, din regulile publice.
//!
//! Pachet de 32 de cărți (7 8 9 10 J Q K A, culori c d h s).
//!
//! * **Împărțire**: 5 cărți fiecăruia, o carte se întoarce (propunerea de atu).
//!   Licitație în două tururi: în turul 1 fiecare spune `prends` (atu = culoarea
//!   cărții întoarse) sau `pas`; în turul 2 `prends-c|d|h|s` (altă culoare) sau `pas`.
//!   Dacă toți dau pas, se re-împarte. Cine ia primește cartea întoarsă; apoi se
//!   împarte (mutare forțată `imparte`) până la 8 cărți fiecare. **Simplificare
//!   pentru doi**: cele 16 cărți rămase (mai puțin 5 date) nu se joacă niciodată
//!   (rămân ascunse), deci nu se poate număra tot pachetul.
//! * **Levate**: se răspunde la culoare; dacă nu ai culoarea trebuie să tai cu atu
//!   (dacă ai); la atu se urcă peste atuul de pe masă, dacă se poate. Nu există partener.
//! * **Ordinea și punctele**: atu J 20, 9 14, A 11, 10 10, K 4, Q 3, 8 și 7 zero
//!   (ordine J 9 A 10 K Q 8 7); alte culori A 11, 10 10, K 4, Q 3, J 2, restul zero
//!   (ordine A 10 K Q J 9 8 7).
//! * **Bonusuri**: *belote-rebelote* (K și Q de atu în aceeași mână) 20, *dix de der*
//!   (ultima levată) 10, *capot* (toate cele 8 levate) 90.
//! * **Contractul**: cine a luat trebuie să ajungă cel puțin la punctele adversarului;
//!   dacă are mai puține, „cade” și adversarul scrie toate punctele din mână
//!   (belote rămâne cui îl are). Meci până la 301 de puncte.
//!
//! Informație ascunsă: vezi `cards52.rs`; AI-ul (`AlphaBeta(1)`) judecă licitația
//! doar după cele 5 cărți ale lui și cartea întoarsă.

use crate::cards::Viewer;
use crate::cards52::{shuffle, SUIT_LETTERS, SUIT_SYMBOLS};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const RANK_LABEL: [&str; 8] = ["7", "8", "9", "10", "J", "Q", "K", "A"];
const TARGET: u16 = 301;
const THRESHOLD: i32 = 30;

pub fn suit(c: u8) -> u8 {
    c / 8
}
pub fn rank(c: u8) -> u8 {
    c % 8
}
pub fn card_text(c: u8) -> String {
    format!(
        "{}{}",
        RANK_LABEL[rank(c) as usize],
        SUIT_LETTERS[suit(c) as usize]
    )
}
fn card_symbol(c: u8) -> String {
    format!(
        "{}{}",
        RANK_LABEL[rank(c) as usize],
        SUIT_SYMBOLS[suit(c) as usize]
    )
}
const K_: u8 = 6;
const Q_: u8 = 5;
const J_: u8 = 4;
const N9: u8 = 2;
const A_: u8 = 7;
const T10: u8 = 3;

/// Puterea unei cărți (mai mare bate) în interiorul culorii ei.
pub fn power(c: u8, trump: u8) -> u8 {
    let r = rank(c);
    if suit(c) == trump {
        match r {
            0 => 0,
            1 => 1,
            Q_ => 2,
            K_ => 3,
            T10 => 4,
            A_ => 5,
            N9 => 6,
            _ => 7, // J
        }
    } else {
        match r {
            T10 => 6,
            J_ => 3,
            Q_ => 4,
            K_ => 5,
            _ => r, // 7 8 9 sunt 0 1 2, A este 7
        }
    }
}

pub fn card_points(c: u8, trump: u8) -> u16 {
    let r = rank(c);
    if suit(c) == trump {
        match r {
            J_ => 20,
            N9 => 14,
            A_ => 11,
            T10 => 10,
            K_ => 4,
            Q_ => 3,
            _ => 0,
        }
    } else {
        match r {
            A_ => 11,
            T10 => 10,
            K_ => 4,
            Q_ => 3,
            J_ => 2,
            _ => 0,
        }
    }
}

/// Cartea `resp` (a doua) bate cartea condusă `led`?
pub fn second_wins(led: u8, resp: u8, trump: u8) -> bool {
    if suit(resp) == suit(led) {
        power(resp, trump) > power(led, trump)
    } else {
        suit(resp) == trump
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Move {
    Take,
    TakeSuit(u8),
    Pass,
    Deal,
    Redeal,
    Play(u8),
    Next,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Bid1,
    Bid2,
    NoTaker,
    Deal,
    Play,
    HandOver,
}

#[derive(Clone)]
pub struct Belote {
    rng: Rng,
    hands: [Vec<u8>; 2],
    stock: Vec<u8>,
    upcard: u8,
    trump: u8,
    taker: Option<Player>,
    phase: Phase,
    side: Player,
    dealer: Player,
    led: Option<u8>,
    pts: [u16; 2],
    tricks: [u8; 2],
    trick_no: u8,
    belote: [bool; 2],
    last_trick_winner: Option<Player>,
    score: [u16; 2],
    /// Rezultatul ultimei mâini: (totaluri, a căzut contractul?).
    last_hand: Option<([u16; 2], bool)>,
    /// Cărțile jucate în mâna curentă (public; masca pe biți a cărților 0..32).
    seen: u32,
    viewer: Viewer,
}

impl Default for Belote {
    fn default() -> Self {
        Self::new()
    }
}

impl Belote {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Belote {
            rng: Rng::new(seed),
            hands: [Vec::new(), Vec::new()],
            stock: Vec::new(),
            upcard: 0,
            trump: 0,
            taker: None,
            phase: Phase::Bid1,
            side: Player::One,
            dealer: Player::Two,
            led: None,
            pts: [0; 2],
            tricks: [0; 2],
            trick_no: 0,
            belote: [false; 2],
            last_trick_winner: None,
            score: [0; 2],
            last_hand: None,
            seen: 0,
            viewer: Viewer::Fixed(Player::One),
        };
        g.deal_hand();
        g
    }

    fn deal_hand(&mut self) {
        let mut deck: Vec<u8> = (0..32).collect();
        shuffle(&mut deck, &mut self.rng);
        self.hands = [deck[0..5].to_vec(), deck[5..10].to_vec()];
        self.upcard = deck[10];
        self.stock = deck[11..].to_vec();
        self.trump = suit(self.upcard);
        self.taker = None;
        self.phase = Phase::Bid1;
        self.side = self.dealer.other();
        self.led = None;
        self.pts = [0; 2];
        self.tricks = [0; 2];
        self.trick_no = 0;
        self.belote = [false; 2];
        self.last_trick_winner = None;
        self.seen = 0;
    }

    pub fn score(&self) -> (u16, u16) {
        (self.score[0], self.score[1])
    }
    pub fn hand_of(&self, p: Player) -> &[u8] {
        &self.hands[p.idx()]
    }
    pub fn trump_suit(&self) -> Option<u8> {
        self.taker.map(|_| self.trump)
    }

    fn match_over(&self) -> bool {
        (self.score[0] >= TARGET || self.score[1] >= TARGET) && self.score[0] != self.score[1]
    }

    /// Cărțile pe care le poate juca `me` ca răspuns la `led` (sau oricare, dacă conduce).
    fn playable(&self, me: Player) -> Vec<u8> {
        let hand = &self.hands[me.idx()];
        let Some(l) = self.led else {
            return hand.clone();
        };
        let t = self.trump;
        let same: Vec<u8> = hand
            .iter()
            .copied()
            .filter(|&c| suit(c) == suit(l))
            .collect();
        if suit(l) == t {
            if same.is_empty() {
                return hand.clone();
            }
            let higher: Vec<u8> = same
                .iter()
                .copied()
                .filter(|&c| power(c, t) > power(l, t))
                .collect();
            return if higher.is_empty() { same } else { higher };
        }
        if !same.is_empty() {
            return same;
        }
        let trumps: Vec<u8> = hand.iter().copied().filter(|&c| suit(c) == t).collect();
        if trumps.is_empty() {
            hand.clone()
        } else {
            trumps
        }
    }

    /// Estimare (x2) a forței unei mâini pentru atu `t`.
    fn strength(hand: &[u8], t: u8) -> i32 {
        let mut s = 0;
        let mut trumps = 0;
        for &c in hand {
            if suit(c) == t {
                trumps += 1;
                s += match rank(c) {
                    J_ => 16,
                    N9 => 12,
                    A_ => 6,
                    T10 => 5,
                    K_ | Q_ => 3,
                    _ => 2,
                };
            } else if rank(c) == A_ {
                s += 5;
            } else if rank(c) == T10 {
                s += 2;
            }
        }
        if hand.contains(&(t * 8 + K_)) && hand.contains(&(t * 8 + Q_)) {
            s += 5;
        }
        s + trumps * 2
    }

    fn score_hand(&mut self) {
        let taker = self.taker.expect("taker");
        let mut tot = self.pts;
        if let Some(w) = self.last_trick_winner {
            tot[w.idx()] += 10;
        }
        for (t, n) in tot.iter_mut().zip(self.tricks) {
            if n == 8 {
                *t += 90;
            }
        }
        let bel = [
            if self.belote[0] { 20 } else { 0 },
            if self.belote[1] { 20 } else { 0 },
        ];
        let (t, d) = (taker.idx(), taker.other().idx());
        let fell = tot[t] + bel[t] < tot[d] + bel[d];
        let fin = if fell {
            let mut f = [0u16; 2];
            f[d] = tot[t] + tot[d] + bel[d];
            f[t] = bel[t];
            f
        } else {
            [tot[0] + bel[0], tot[1] + bel[1]]
        };
        self.score[0] += fin[0];
        self.score[1] += fin[1];
        self.last_hand = Some((fin, fell));
        self.phase = Phase::HandOver;
    }
}

const ROLLOUTS: usize = 250;
const ROLLOUTS_BID: usize = 300;

impl Belote {
    fn is_master(&self, c: u8, hand: &[u8]) -> bool {
        let t = self.trump;
        (0..32u8).all(|d| {
            suit(d) != suit(c)
                || power(d, t) <= power(c, t)
                || self.seen & (1 << d) != 0
                || hand.contains(&d)
        })
    }

    fn bid_pick(&self) -> Move {
        let hand = &self.hands[self.side.idx()];
        if self.phase == Phase::Bid1 {
            let mut h = hand.clone();
            h.push(self.upcard);
            if Self::strength(&h, suit(self.upcard)) >= THRESHOLD {
                Move::Take
            } else {
                Move::Pass
            }
        } else {
            let mut best = (THRESHOLD - 1, None);
            for s in (0..4u8).filter(|&s| s != suit(self.upcard)) {
                let v = Self::strength(hand, s);
                if v > best.0 {
                    best = (v, Some(s));
                }
            }
            match best.1 {
                Some(s) => Move::TakeSuit(s),
                None => Move::Pass,
            }
        }
    }

    fn play_pick(&self, rng: &mut Rng) -> u8 {
        let t = self.trump;
        let hand = &self.hands[self.side.idx()];
        let mv = self.playable(self.side);
        let mut best = (i32::MIN, mv[0]);
        for &c in &mv {
            let cost = card_points(c, t) as i32 * 3 + power(c, t) as i32;
            let sc = match self.led {
                None => {
                    let master = self.is_master(c, hand);
                    if suit(c) == t {
                        if master {
                            70 + card_points(c, t) as i32
                        } else {
                            -cost - 20
                        }
                    } else if master {
                        60 + card_points(c, t) as i32
                    } else {
                        -cost
                    }
                }
                Some(l) => {
                    if second_wins(l, c, t) {
                        let gain = card_points(l, t) as i32 + 4;
                        if suit(c) != t || cost / 3 <= gain + 6 {
                            100 - cost
                        } else {
                            -cost
                        }
                    } else {
                        -cost
                    }
                }
            };
            let sc = sc * 4 + rng.below(4) as i32;
            if sc > best.0 {
                best = (sc, c);
            }
        }
        best.1
    }

    /// Joacă mâna până la capăt; întoarce diferența de scor (p − adversar) în puncte.
    fn rollout(&mut self, p: Player, rng: &mut Rng) -> f64 {
        let mut guard = 0;
        while !matches!(self.phase, Phase::HandOver | Phase::NoTaker) && guard < 200 {
            guard += 1;
            match self.phase {
                Phase::Bid1 | Phase::Bid2 => {
                    let m = self.bid_pick();
                    self.apply(&m);
                }
                Phase::Deal => self.apply(&Move::Deal),
                Phase::Play => {
                    let c = self.play_pick(rng);
                    self.apply(&Move::Play(c));
                }
                _ => break,
            }
        }
        let d = self.score[p.idx()] as f64 - self.score[p.other().idx()] as f64;
        if self.match_over() {
            d + if d > 0.0 { 300.0 } else { -300.0 }
        } else {
            d
        }
    }
}

impl Game for Belote {
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
        if self.match_over() {
            return Vec::new();
        }
        match self.phase {
            Phase::Bid1 => vec![Move::Take, Move::Pass],
            Phase::Bid2 => {
                let mut v: Vec<Move> = (0..4u8)
                    .filter(|&s| s != suit(self.upcard))
                    .map(Move::TakeSuit)
                    .collect();
                v.push(Move::Pass);
                v
            }
            Phase::NoTaker => vec![Move::Redeal],
            Phase::Deal => vec![Move::Deal],
            Phase::HandOver => vec![Move::Next],
            Phase::Play => self
                .playable(self.side)
                .into_iter()
                .map(Move::Play)
                .collect(),
        }
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Take | Move::TakeSuit(_) => {
                let me = self.side;
                if let Move::TakeSuit(s) = m {
                    self.trump = *s;
                }
                self.taker = Some(me);
                self.hands[me.idx()].push(self.upcard);
                self.phase = Phase::Deal;
            }
            Move::Pass => {
                if self.side == self.dealer {
                    if self.phase == Phase::Bid1 {
                        self.phase = Phase::Bid2;
                        self.side = self.dealer.other();
                    } else {
                        self.phase = Phase::NoTaker;
                    }
                } else {
                    self.side = self.dealer;
                }
            }
            Move::Redeal | Move::Next => {
                self.dealer = self.dealer.other();
                self.deal_hand();
            }
            Move::Deal => {
                let taker = self.taker.expect("taker");
                for (p, n) in [(taker, 2), (taker.other(), 3)] {
                    for _ in 0..n {
                        let c = self.stock.pop().expect("stock");
                        self.hands[p.idx()].push(c);
                    }
                }
                self.phase = Phase::Play;
                self.side = self.dealer.other();
            }
            Move::Play(c) => {
                let me = self.side;
                let t = self.trump;
                let h = &mut self.hands[me.idx()];
                let pos = h.iter().position(|x| x == c).expect("card in hand");
                h.remove(pos);
                self.seen |= 1 << *c;
                if suit(*c) == t && (rank(*c) == K_ || rank(*c) == Q_) {
                    let other = t * 8 + if rank(*c) == K_ { Q_ } else { K_ };
                    if self.hands[me.idx()].contains(&other) {
                        self.belote[me.idx()] = true;
                    }
                }
                match self.led {
                    None => {
                        self.led = Some(*c);
                        self.side = me.other();
                    }
                    Some(l) => {
                        let winner = if second_wins(l, *c, t) {
                            me
                        } else {
                            me.other()
                        };
                        self.pts[winner.idx()] += card_points(l, t) + card_points(*c, t);
                        self.tricks[winner.idx()] += 1;
                        self.trick_no += 1;
                        self.led = None;
                        self.side = winner;
                        if self.trick_no == 8 {
                            self.last_trick_winner = Some(winner);
                            self.score_hand();
                        }
                    }
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.match_over() {
            Some(Outcome::Win(if self.score[0] > self.score[1] {
                Player::One
            } else {
                Player::Two
            }))
        } else {
            None
        }
    }

    /// Evaluare cinstită prin determinizare. Citește doar informația vizibilă lui `p`: mâna
    /// lui `p`, cartea întoarsă / atuul, cărțile deja jucate, scorurile și mărimea mâinii
    /// adversarului. Mâna adversarului (și stocul) se eșantionează din cărțile necunoscute,
    /// apoi mâna se joacă până la capăt cu o politică euristică de licitație și joc.
    fn evaluate(&self, p: Player) -> i32 {
        let q = p.other();
        let me = p.idx();
        if matches!(self.phase, Phase::HandOver | Phase::NoTaker) {
            return (self.score[me] as i32 - self.score[q.idx()] as i32) * 10;
        }
        let in_play = self.phase == Phase::Play;
        let mine = self.hands[me].clone();
        let mut known_opp: Vec<u8> = Vec::new();
        let opp_n = self.hands[q.idx()].len();
        if self.taker == Some(q) && self.seen & (1 << self.upcard) == 0 {
            known_opp.push(self.upcard); // cartea întoarsă luată de adversar e publică
        }
        let mut pool: Vec<u8> = (0..32u8)
            .filter(|&c| {
                !mine.contains(&c)
                    && self.seen & (1 << c) == 0
                    && !known_opp.contains(&c)
                    && self.led != Some(c)
                    && !(self.taker.is_none() && c == self.upcard)
                    && !(self.taker.is_some() && c == self.upcard && !in_play)
            })
            .collect();
        if pool.len() + known_opp.len() < opp_n {
            return 0;
        }
        let mut seed: u64 = 0xBE10_7E00 ^ (self.seen as u64) << 8 ^ opp_n as u64;
        seed ^= (self.score[0] as u64) << 40 ^ (self.score[1] as u64) << 52 ^ self.phase as u64;
        for &c in mine.iter().chain(self.led.iter()) {
            seed = seed.rotate_left(11) ^ (c as u64 + 1);
        }
        let mut rng = Rng::new(seed);
        let mut total = 0.0f64;
        let n = if in_play { ROLLOUTS } else { ROLLOUTS_BID };
        for _ in 0..n {
            shuffle(&mut pool, &mut rng);
            let mut g = self.clone();
            let need = opp_n - known_opp.len();
            let mut oh = known_opp.clone();
            oh.extend_from_slice(&pool[..need]);
            g.hands[q.idx()] = oh;
            let rest: Vec<u8> = pool[need..].to_vec();
            if !in_play {
                g.stock = rest;
            }
            g.rng = Rng::new(rng.next_u64());
            total += g.rollout(p, &mut rng);
        }
        (10.0 * total / n as f64) as i32
    }

    fn render(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            "Belote — meci până la {TARGET}: J1 {} – J2 {}\n",
            self.score[0], self.score[1]
        ));
        match self.taker {
            None => s.push_str(&format!(
                "Carte întoarsă: {}. Licitație.\n",
                card_symbol(self.upcard)
            )),
            Some(t) => s.push_str(&format!(
                "Atu: {}. A luat Jucător {}. Levate: J1 {}, J2 {}; puncte în mână: J1 {}, J2 {}\n",
                SUIT_SYMBOLS[self.trump as usize],
                t.idx() + 1,
                self.tricks[0],
                self.tricks[1],
                self.pts[0],
                self.pts[1]
            )),
        }
        let v = self.viewer.who(self.side);
        s.push_str(&format!(
            "Jucător {} (ascuns) are {} cărți.\n",
            v.other().idx() + 1,
            self.hands[v.other().idx()].len()
        ));
        if let Some(l) = self.led {
            s.push_str(&format!(
                "Pe masă: Jucător {} a jucat {}\n",
                self.side.other().idx() + 1,
                card_symbol(l)
            ));
        }
        let mut h = self.hands[v.idx()].clone();
        h.sort_unstable();
        s.push_str(&format!(
            "Mâna ta (Jucător {}): {}\n",
            v.idx() + 1,
            h.iter()
                .map(|&c| card_text(c))
                .collect::<Vec<_>>()
                .join(" ")
        ));
        if let Some((fin, fell)) = self.last_hand {
            if self.phase == Phase::HandOver {
                s.push_str(&format!(
                    "Mână încheiată: J1 +{}, J2 +{}{}. Scrie «urmatoarea».\n",
                    fin[0],
                    fin[1],
                    if fell { " (contractul a căzut)" } else { "" }
                ));
            }
        }
        match self.outcome() {
            Some(Outcome::Win(p)) => s.push_str(&format!(
                "Meci încheiat: Jucător {} câștigă.\n",
                p.idx() + 1
            )),
            _ => match self.phase {
                Phase::Deal => s.push_str("Scrie «imparte».\n"),
                Phase::NoTaker => s.push_str("Nimeni n-a luat: scrie «reimparte».\n"),
                Phase::HandOver => {}
                _ => s.push_str(&format!("La mutare: Jucător {}\n", self.side.idx() + 1)),
            },
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Take => "prends".into(),
            Move::TakeSuit(s) => format!("prends-{}", SUIT_LETTERS[*s as usize]),
            Move::Pass => "pas".into(),
            Move::Deal => "imparte".into(),
            Move::Redeal => "reimparte".into(),
            Move::Play(c) => card_text(*c),
            Move::Next => "urmatoarea".into(),
        }
    }

    /// Adâncime 0 (o singură mutare): `AlphaBeta(1)` ar fi extins la Hard/Expert răspunsul adversarului din
    /// mâna lui reală (și împărțirea reală din stoc); `evaluate` determinizează tot ce nu se vede.
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // culoare*8 + rang (0=7,1=8,2=9,3=10,4=J,5=Q,6=K,7=A)
    fn c(s: u8, r: u8) -> u8 {
        s * 8 + r
    }

    fn base() -> Belote {
        let mut g = Belote::with_seed(1);
        g.hands = [Vec::new(), Vec::new()];
        g.stock = (0..32).collect();
        g.trump = 2;
        g.taker = Some(Player::One);
        g.phase = Phase::Play;
        g.side = Player::One;
        g.led = None;
        g.pts = [0; 2];
        g.tricks = [0; 2];
        g.trick_no = 0;
        g.belote = [false; 2];
        g
    }

    #[test]
    fn points_total_152_per_full_suit_set() {
        for t in 0..4u8 {
            let total: u16 = (0..32).map(|x| card_points(x, t)).sum();
            assert_eq!(total, 152);
        }
        assert_eq!(card_points(c(2, J_), 2), 20);
        assert_eq!(card_points(c(2, N9), 2), 14);
        assert_eq!(card_points(c(1, J_), 2), 2);
    }

    #[test]
    fn trump_order_is_j_9_a_10_k_q_8_7() {
        let t = 0;
        let order = [0u8, 1, Q_, K_, T10, A_, N9, J_];
        for w in order.windows(2) {
            assert!(power(c(t, w[1]), t) > power(c(t, w[0]), t));
        }
        assert!(second_wins(c(0, N9), c(0, J_), t));
        assert!(!second_wins(c(0, J_), c(0, N9), t));
    }

    #[test]
    fn non_trump_order_is_a_10_k_q_j_9_8_7() {
        let t = 0;
        let order = [0u8, 1, N9, J_, Q_, K_, T10, A_];
        for w in order.windows(2) {
            assert!(power(c(1, w[1]), t) > power(c(1, w[0]), t));
        }
        assert!(second_wins(c(1, K_), c(1, T10), t)); // 10 bate K
        assert!(!second_wins(c(1, A_), c(1, T10), t));
        assert!(second_wins(c(1, A_), c(0, 0), t)); // 7 de atu taie asul
    }

    #[test]
    fn follow_rules() {
        let mut g = base(); // atu = 2
        g.side = Player::Two;
        g.led = Some(c(0, 5)); // Q de culoarea 0
        g.hands[1] = vec![c(0, 0), c(0, 6), c(1, 7), c(2, 0)];
        let mut p = g.playable(Player::Two);
        p.sort_unstable();
        assert_eq!(p, vec![c(0, 0), c(0, 6)]); // trebuie culoarea
        g.hands[1] = vec![c(1, 7), c(2, 0), c(2, 1)];
        let mut p = g.playable(Player::Two);
        p.sort_unstable();
        assert_eq!(p, vec![c(2, 0), c(2, 1)]); // fără culoare: taie
        g.hands[1] = vec![c(1, 7), c(3, 0)];
        assert_eq!(g.playable(Player::Two).len(), 2); // fără atu: orice
                                                      // atu condus: urci dacă poți
        g.led = Some(c(2, 6)); // K atu
        g.hands[1] = vec![c(2, 0), c(2, 4), c(1, 7)];
        assert_eq!(g.playable(Player::Two), vec![c(2, 4)]); // doar J bate K
        g.hands[1] = vec![c(2, 0), c(2, 1), c(1, 7)];
        let mut p = g.playable(Player::Two);
        p.sort_unstable();
        assert_eq!(p, vec![c(2, 0), c(2, 1)]); // nu poți urca, dar dai atu
    }

    #[test]
    fn belote_dix_de_der_and_capot() {
        let mut g = base();
        g.trick_no = 7;
        g.tricks = [7, 0];
        g.pts = [100, 0];
        g.hands[0] = vec![c(2, K_), c(2, Q_)];
        // J1 are K și Q de atu: le joacă pe rând; mai întâi testăm belote
        g.hands[0] = vec![c(2, K_), c(2, Q_)];
        g.hands[1] = vec![c(1, 0), c(1, 1)];
        // două levate rămase
        g.trick_no = 6;
        g.tricks = [6, 0];
        g.apply(&Move::Play(c(2, K_)));
        assert!(g.belote[0]);
        g.apply(&Move::Play(c(1, 0)));
        g.apply(&Move::Play(c(2, Q_)));
        g.apply(&Move::Play(c(1, 1)));
        assert_eq!(g.phase, Phase::HandOver);
        let (fin, fell) = g.last_hand.unwrap();
        assert!(!fell);
        // 100 + 4+3 (K,Q de atu) + 10 de der + 90 capot + 20 belote
        assert_eq!(fin[0], 100 + 4 + 3 + 10 + 90 + 20);
        assert_eq!(fin[1], 0);
    }

    #[test]
    fn failed_contract_gives_all_points_to_defender() {
        let mut g = base();
        g.taker = Some(Player::One);
        g.pts = [30, 60];
        g.tricks = [3, 5];
        g.trick_no = 7;
        g.hands[0] = vec![c(0, 0)];
        g.hands[1] = vec![c(0, 1)];
        g.side = Player::One;
        g.belote = [true, false];
        g.apply(&Move::Play(c(0, 0)));
        g.apply(&Move::Play(c(0, 1))); // 8 > 7: J2 ia ultima levată
        let (fin, fell) = g.last_hand.unwrap();
        assert!(fell);
        // totaluri: J1 30, J2 60 + 10 de der; belote 20 rămâne la J1
        assert_eq!(fin, [20, 30 + 70]);
    }

    #[test]
    fn bidding_flow() {
        let mut g = Belote::with_seed(7);
        assert_eq!(g.to_move(), Player::One); // dealer = Two
        g.apply(&Move::Pass);
        assert_eq!(g.to_move(), Player::Two);
        g.apply(&Move::Pass);
        assert_eq!(g.phase, Phase::Bid2);
        assert_eq!(g.to_move(), Player::One);
        assert_eq!(g.legal_moves().len(), 4); // 3 culori + pas
        g.apply(&Move::Pass);
        g.apply(&Move::Pass);
        assert_eq!(g.legal_moves(), vec![Move::Redeal]);
        g.apply(&Move::Redeal);
        assert_eq!(g.dealer, Player::One);
        assert_eq!(g.to_move(), Player::Two);
        // luarea în turul 1
        g.apply(&Move::Take);
        assert_eq!(g.hands[1].len(), 6);
        assert_eq!(g.legal_moves(), vec![Move::Deal]);
        g.apply(&Move::Deal);
        assert_eq!(g.hands[1].len(), 8);
        assert_eq!(g.hands[0].len(), 8);
        assert_eq!(g.phase, Phase::Play);
    }

    #[test]
    fn deal_conserves_cards() {
        let mut g = Belote::with_seed(3);
        g.apply(&Move::Take);
        g.apply(&Move::Deal);
        let mut all: Vec<u8> = g.hands[0]
            .iter()
            .chain(&g.hands[1])
            .chain(&g.stock)
            .copied()
            .collect();
        all.sort_unstable();
        assert_eq!(all, (0..32).collect::<Vec<u8>>());
    }

    #[test]
    fn random_matches_finish() {
        for seed in 1..=20u64 {
            let mut g = Belote::with_seed(seed);
            let mut rng = Rng::new(seed ^ 0x31);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 20000);
            }
        }
    }

    #[test]
    fn render_hides_the_opponent() {
        let g = Belote::with_seed(5);
        assert!(Game::render(&g).contains("Jucător 2 (ascuns) are 5 cărți"));
    }

    #[test]
    fn viewer_controls_whose_hand_is_shown() {
        let hand_txt = |g: &Belote, p: usize| {
            let mut h = g.hands[p].clone();
            h.sort_unstable();
            h.iter()
                .map(|&c| card_text(c))
                .collect::<Vec<_>>()
                .join(" ")
        };
        let mut g = Belote::with_seed(5);
        Game::set_viewer(&mut g, None);
        g.apply(&Move::Pass);
        assert_eq!(g.to_move(), Player::Two);
        let r = Game::render(&g);
        assert!(r.contains(&hand_txt(&g, 1)), "{r}");
        assert!(!r.contains(&hand_txt(&g, 0)), "{r}");
        Game::set_viewer(&mut g, Some(Player::One));
        let r = Game::render(&g);
        assert!(
            r.contains(&hand_txt(&g, 0)) && !r.contains(&hand_txt(&g, 1)),
            "{r}"
        );
        Game::set_viewer(&mut g, Some(Player::Two));
        let r = Game::render(&g);
        assert!(
            r.contains(&hand_txt(&g, 1)) && !r.contains(&hand_txt(&g, 0)),
            "{r}"
        );
    }

    #[test]
    fn evaluate_ignores_the_opponents_cards() {
        let mut g = Belote::with_seed(9);
        g.apply(&Move::Take);
        g.apply(&Move::Deal);
        let mut h = g.clone();
        let (a, b) = (h.hands[1][0], h.stock[2]);
        h.hands[1][0] = b;
        h.stock[2] = a;
        assert_eq!(g.evaluate(Player::One), h.evaluate(Player::One));
    }

    #[test]
    fn ai_beats_a_random_mover_mostly() {
        use polytabletop_core::GameObj;
        let mut wins = 0;
        let n = 20;
        for seed in 1..=n {
            let mut g: Box<dyn GameObj> = Box::new(Belote::with_seed(seed));
            let mut rng = Rng::new(seed + 100);
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

#[cfg(test)]
mod a2_audit {
    use super::*;

    #[test]
    fn ai_nu_vede_mana_adversarului_nici_stocul_la_orice_nivel() {
        crate::p3_comun::ai_blind_check_with(
            "belote",
            Belote::with_seed,
            |g, me, seed| {
                let o = me.other().idx();
                let n = g.hands[o].len();
                let mut pool: Vec<u8> = g.hands[o].clone();
                pool.extend(g.stock.iter().copied());
                crate::p3_comun::shuffle_with(&mut pool, seed);
                g.hands[o] = pool.drain(..n).collect();
                g.stock = pool;
                g.rng = Rng::new(seed ^ 0xBE);
            },
            2,
            4,
            crate::p3_comun::Term::Check,
        );
    }
}
