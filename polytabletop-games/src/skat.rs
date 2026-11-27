//! Skat pentru 2 jucători — variantă adaptată, din regulile publice.
//!
//! Pachet de 32 de cărți (7 8 9 10 J Q K A; culori c ♣, s ♠, h ♥, d ♦).
//!
//! * **Împărțire**: 10 cărți fiecăruia, 2 în *skat*; celelalte 10 rămân ascunse și
//!   nu se joacă (simplificare pentru doi). Cine nu a împărțit conduce prima levată.
//! * **Licitație**: cine a împărțit „spune” valori din scara 18 20 22 23 24 27 30 33 35
//!   36 40 44 45 46 48 50 54 55 59 60; celălalt `tin` (ține) sau `pas`. Cine rămâne
//!   în licitație e jucătorul singur (declarant). Dacă licitantul dă pas din
//!   start, celălalt poate declara `licit 18` sau da pas (re-împărțire, numără ca mână).
//! * **Skat**: declarantul ia skat-ul (`skat`) și pune jos 2 cărți (`pune 10h Jc`),
//!   sau joacă „din mână” (`hand`, jocul valorează mai mult).
//! * **Jocuri**: `trefla` (bază 12), `pica` (11), `cupa` (10), `caro` (9) — atu sunt cei
//!   4 juniori (J) și culoarea; `grand` (24) — atu doar juniorii; `null` (23, din mână 35)
//!   — fără atu, ordinea A K Q J 10 9 8 7, declarantul nu are voie să ia nicio levată.
//!   Trebuie să răspunzi la culoare (juniorii aparțin atuului).
//!   Punctele: A 11, 10 10, K 4, Q 3, J 2; cu cele din skat declarantul are nevoie de 61 din 120.
//! * **Valoarea jocului** = bază × (matadori + 1 + mână + schneider + schwarz); matadori =
//!   „cu / fără” juniorii (♣♠♥♦) și atuul consecutiv, numărați în cele 12 cărți ale
//!   declarantului. Schneider = 90+ sau cel mult 30 de puncte; schwarz = toate / nicio levată.
//!   Câștig: + valoarea; pierdere: −2 × valoarea; dacă valoarea e mai mică decât licitația
//!   („supralicitat”) jocul e pierdut (−2 × cel mai mic multiplu al bazei ≥ licitație).
//! * Meciul are 6 mâini; câștigă cine are mai multe puncte la final (egal = remiză).
//!   Nu există ouvert, Ramsch, Kontra, Re, schneider/schwarz anunțate.
//!
//! Informație ascunsă: vezi `cards52.rs`. `evaluate` citește doar mâna proprie,
//! licitația și punctele din levate; cărțile din skat luate nu intră în evaluarea mutării
//! `skat` (nu se „peticește” viitorul).

use crate::cards::Viewer;
use crate::cards52::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const RANK_LABEL: [&str; 8] = ["7", "8", "9", "10", "J", "Q", "K", "A"];
const SUIT_LETTERS: [char; 4] = ['c', 's', 'h', 'd'];
const SUIT_SYMBOLS: [&str; 4] = ["♣", "♠", "♥", "♦"];
const BASE: [i32; 4] = [12, 11, 10, 9];
const LADDER: [u16; 20] = [
    18, 20, 22, 23, 24, 27, 30, 33, 35, 36, 40, 44, 45, 46, 48, 50, 54, 55, 59, 60,
];
const HANDS: u8 = 6;
const JACK: u8 = 4;

pub fn suit(c: u8) -> u8 {
    c / 8
}
pub fn rank(c: u8) -> u8 {
    c % 8
}
pub fn make_card(s: u8, r: u8) -> u8 {
    s * 8 + r
}
pub fn card_text(c: u8) -> String {
    format!(
        "{}{}",
        RANK_LABEL[rank(c) as usize],
        SUIT_LETTERS[suit(c) as usize]
    )
}
pub fn card_points(c: u8) -> u16 {
    match rank(c) {
        7 => 11,
        3 => 10,
        6 => 4,
        5 => 3,
        4 => 2,
        _ => 0,
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Suit(u8),
    Grand,
    Null,
}

impl Kind {
    fn name(self) -> &'static str {
        match self {
            Kind::Suit(0) => "trefla",
            Kind::Suit(1) => "pica",
            Kind::Suit(2) => "cupa",
            Kind::Suit(_) => "caro",
            Kind::Grand => "grand",
            Kind::Null => "null",
        }
    }
    fn base(self, hand: bool) -> i32 {
        match self {
            Kind::Suit(s) => BASE[s as usize],
            Kind::Grand => 24,
            Kind::Null => {
                if hand {
                    35
                } else {
                    23
                }
            }
        }
    }
}

const KINDS: [Kind; 6] = [
    Kind::Suit(0),
    Kind::Suit(1),
    Kind::Suit(2),
    Kind::Suit(3),
    Kind::Grand,
    Kind::Null,
];

/// Grupul de răspuns: 4 = atu, altfel culoarea.
fn group(c: u8, k: Kind) -> u8 {
    match k {
        Kind::Null => suit(c),
        Kind::Grand => {
            if rank(c) == JACK {
                4
            } else {
                suit(c)
            }
        }
        Kind::Suit(t) => {
            if rank(c) == JACK || suit(c) == t {
                4
            } else {
                suit(c)
            }
        }
    }
}

/// Puterea în interiorul grupului (A 10 K Q 9 8 7; la null A K Q J 10 9 8 7).
fn power(c: u8, k: Kind) -> u8 {
    if k == Kind::Null {
        return rank(c);
    }
    if rank(c) == JACK {
        return 100 + (3 - suit(c));
    }
    // 7 8 9 Q K 10 A
    match rank(c) {
        0 => 0,
        1 => 1,
        2 => 2,
        5 => 3,
        6 => 4,
        3 => 5,
        _ => 6,
    }
}

/// Cartea `resp` (a doua) bate cartea condusă `led`?
fn second_wins(led: u8, resp: u8, k: Kind) -> bool {
    let (gl, gr) = (group(led, k), group(resp, k));
    if gl == gr {
        power(resp, k) > power(led, k)
    } else {
        gr == 4
    }
}

/// Matadori (cu/fără) în `cards` pentru jocul `k`.
pub fn matadors(cards: &[u8], k: Kind) -> u16 {
    let mut seq: Vec<u8> = (0..4u8).map(|s| make_card(s, JACK)).collect();
    if let Kind::Suit(t) = k {
        for r in [7u8, 3, 6, 5, 2, 1, 0] {
            seq.push(make_card(t, r));
        }
    }
    let with = cards.contains(&seq[0]);
    let mut n = 0;
    for c in &seq {
        if cards.contains(c) == with {
            n += 1;
        } else {
            break;
        }
    }
    n
}

#[derive(Clone, PartialEq, Debug)]
pub enum Move {
    Say(u16),
    Hold,
    Pass,
    TakeSkat,
    Hand,
    Put(u8, u8),
    Declare(Kind),
    Play(u8),
    Redeal,
    Next,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    BidSay,
    BidHold,
    BidSolo,
    NoDeal,
    Pickup,
    Discard,
    Declare,
    Play,
    HandOver,
}

#[derive(Clone)]
pub struct Skat {
    rng: Rng,
    hands: [Vec<u8>; 2],
    skat: [u8; 2],
    dealer: Player,
    side: Player,
    phase: Phase,
    bidder: Player,
    cur_bid: u16,
    declarer: Player,
    bid: u16,
    hand_game: bool,
    kind: Kind,
    /// Cărțile din skat ale declarantului (netrase sau puse jos): punctele lor îi revin.
    pile: Vec<u8>,
    /// Cele 12 cărți ale declarantului (pentru matadori).
    decl_cards: Vec<u8>,
    led: Option<u8>,
    tricks: [u8; 2],
    pts: [u16; 2],
    trick_no: u8,
    hands_done: u8,
    total: [i32; 2],
    last_hand: Option<String>,
    viewer: Viewer,
}

impl Default for Skat {
    fn default() -> Self {
        Self::new()
    }
}

impl Skat {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Skat {
            rng: Rng::new(seed),
            hands: [Vec::new(), Vec::new()],
            skat: [0; 2],
            dealer: Player::Two,
            side: Player::Two,
            phase: Phase::BidSay,
            bidder: Player::Two,
            cur_bid: 0,
            declarer: Player::One,
            bid: 0,
            hand_game: false,
            kind: Kind::Grand,
            pile: Vec::new(),
            decl_cards: Vec::new(),
            led: None,
            tricks: [0; 2],
            pts: [0; 2],
            trick_no: 0,
            hands_done: 0,
            total: [0; 2],
            last_hand: None,
            viewer: Viewer::Fixed(Player::One),
        };
        g.deal_hand();
        g
    }

    fn deal_hand(&mut self) {
        let mut deck: Vec<u8> = (0..32).collect();
        shuffle(&mut deck, &mut self.rng);
        self.hands = [deck[0..10].to_vec(), deck[10..20].to_vec()];
        self.skat = [deck[20], deck[21]];
        self.phase = Phase::BidSay;
        self.bidder = self.dealer;
        self.side = self.dealer;
        self.cur_bid = 0;
        self.bid = 0;
        self.hand_game = false;
        self.pile.clear();
        self.decl_cards.clear();
        self.led = None;
        self.tricks = [0; 2];
        self.pts = [0; 2];
        self.trick_no = 0;
    }

    pub fn totals(&self) -> (i32, i32) {
        (self.total[0], self.total[1])
    }
    pub fn hand_of(&self, p: Player) -> &[u8] {
        &self.hands[p.idx()]
    }
    pub fn hands_done(&self) -> u8 {
        self.hands_done
    }

    fn over(&self) -> bool {
        self.hands_done >= HANDS
    }

    fn responder(&self) -> Player {
        self.bidder.other()
    }

    fn next_bid(&self) -> Option<u16> {
        LADDER.iter().copied().find(|&b| b > self.cur_bid)
    }

    fn set_declarer(&mut self, d: Player, bid: u16) {
        self.declarer = d;
        self.bid = bid;
        self.phase = Phase::Pickup;
        self.side = d;
    }

    /// Estimarea valorii unui joc pentru o mână de 10 cărți (0 dacă nu pare de jucat).
    fn est(hand: &[u8], k: Kind, hand_game: bool) -> i32 {
        let jacks = hand.iter().filter(|&&c| rank(c) == JACK).count() as i32;
        let hand_bonus = i32::from(hand_game);
        match k {
            Kind::Suit(t) => {
                let tc = hand.iter().filter(|&&c| group(c, k) == 4).count() as i32;
                let aces = hand
                    .iter()
                    .filter(|&&c| rank(c) == 7 && suit(c) != t)
                    .count() as i32;
                if tc >= 7 || (tc == 6 && aces >= 1 && jacks >= 2) {
                    BASE[t as usize] * (matadors(hand, k) as i32 + 1 + hand_bonus)
                } else {
                    0
                }
            }
            Kind::Grand => {
                let aces = hand.iter().filter(|&&c| rank(c) == 7).count() as i32;
                if (jacks >= 3 && aces >= 2) || jacks == 4 {
                    24 * (matadors(hand, k) as i32 + 1 + hand_bonus)
                } else {
                    0
                }
            }
            Kind::Null => {
                let high = hand.iter().filter(|&&c| rank(c) >= JACK).count();
                let tens = hand.iter().filter(|&&c| rank(c) == 3).count();
                if high + tens == 0 {
                    k.base(hand_game)
                } else {
                    0
                }
            }
        }
    }

    fn cap(hand: &[u8], hand_game: bool) -> i32 {
        KINDS
            .iter()
            .map(|&k| Self::est(hand, k, hand_game))
            .max()
            .unwrap_or(0)
    }

    fn hold(&self, c: u8) -> i32 {
        if self.kind == Kind::Null {
            return -(rank(c) as i32) * 3;
        }
        if group(c, self.kind) == 4 {
            20 + (power(c, self.kind) as i32 % 100) * 4 + if rank(c) == JACK { 30 } else { 0 }
        } else {
            card_points(c) as i32 * 4 + power(c, self.kind) as i32 * 2
        }
    }

    fn playable(&self, me: Player) -> Vec<u8> {
        let hand = &self.hands[me.idx()];
        let Some(l) = self.led else {
            return hand.clone();
        };
        let g = group(l, self.kind);
        let same: Vec<u8> = hand
            .iter()
            .copied()
            .filter(|&c| group(c, self.kind) == g)
            .collect();
        if same.is_empty() {
            hand.clone()
        } else {
            same
        }
    }

    fn finish_hand(&mut self) {
        let d = self.declarer.idx();
        let pile_pts: u16 = self.pile.iter().map(|&c| card_points(c)).sum();
        let dp = self.pts[d] + pile_pts;
        let k = self.kind;
        let base = k.base(self.hand_game);
        let (won_play, mult_value) = match k {
            Kind::Null => (self.tricks[d] == 0, base),
            _ => {
                let schneider = i32::from(dp >= 90 || dp <= 30);
                let schwarz = i32::from(self.tricks[d] == 10 || self.tricks[d] == 0);
                let mult = matadors(&self.decl_cards, k) as i32
                    + 1
                    + i32::from(self.hand_game)
                    + schneider
                    + schwarz;
                (dp >= 61, base * mult)
            }
        };
        let bid = self.bid as i32;
        let (delta, note) = if bid > mult_value {
            let ceil = (bid + base - 1) / base * base;
            (-2 * ceil, "supralicitat")
        } else if won_play {
            (mult_value, "câștigat")
        } else {
            (-2 * mult_value, "pierdut")
        };
        self.total[d] += delta;
        self.hands_done += 1;
        self.phase = Phase::HandOver;
        self.last_hand = Some(format!(
            "J{} a jucat {} (licitat {}, {} puncte): {} {:+}",
            d + 1,
            k.name(),
            self.bid,
            if k == Kind::Null {
                format!("{} levate", self.tricks[d])
            } else {
                dp.to_string()
            },
            note,
            delta
        ));
    }
}

impl Game for Skat {
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
        if self.over() {
            return Vec::new();
        }
        match self.phase {
            Phase::BidSay => {
                let mut v = Vec::new();
                if let Some(b) = self.next_bid() {
                    v.push(Move::Say(b));
                }
                v.push(Move::Pass);
                v
            }
            Phase::BidHold => vec![Move::Hold, Move::Pass],
            Phase::BidSolo => vec![Move::Say(18), Move::Pass],
            Phase::NoDeal => vec![Move::Redeal],
            Phase::Pickup => vec![Move::TakeSkat, Move::Hand],
            Phase::Discard => {
                let mut h = self.hands[self.declarer.idx()].clone();
                h.sort_unstable();
                let mut v = Vec::new();
                for i in 0..h.len() {
                    for j in i + 1..h.len() {
                        v.push(Move::Put(h[i], h[j]));
                    }
                }
                v
            }
            Phase::Declare => KINDS.iter().map(|&k| Move::Declare(k)).collect(),
            Phase::Play => self
                .playable(self.side)
                .into_iter()
                .map(Move::Play)
                .collect(),
            Phase::HandOver => vec![Move::Next],
        }
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Say(b) => {
                if self.phase == Phase::BidSolo {
                    self.set_declarer(self.side, *b);
                } else {
                    self.cur_bid = *b;
                    self.phase = Phase::BidHold;
                    self.side = self.responder();
                }
            }
            Move::Hold => {
                self.phase = Phase::BidSay;
                self.side = self.bidder;
            }
            Move::Pass => match self.phase {
                Phase::BidSay => {
                    if self.cur_bid == 0 {
                        self.phase = Phase::BidSolo;
                        self.side = self.responder();
                    } else {
                        let d = self.responder();
                        self.set_declarer(d, self.cur_bid);
                    }
                }
                Phase::BidHold => {
                    let d = self.bidder;
                    self.set_declarer(d, self.cur_bid);
                }
                _ => {
                    // BidSolo
                    self.phase = Phase::NoDeal;
                }
            },
            Move::Redeal => {
                self.hands_done += 1;
                self.dealer = self.dealer.other();
                if !self.over() {
                    self.deal_hand();
                }
            }
            Move::Next => {
                self.dealer = self.dealer.other();
                if !self.over() {
                    self.deal_hand();
                }
            }
            Move::TakeSkat => {
                let d = self.declarer.idx();
                self.hands[d].extend(self.skat);
                self.phase = Phase::Discard;
            }
            Move::Hand => {
                self.hand_game = true;
                self.pile = self.skat.to_vec();
                self.phase = Phase::Declare;
            }
            Move::Put(a, b) => {
                let d = self.declarer.idx();
                self.hands[d].retain(|c| c != a && c != b);
                self.pile = vec![*a, *b];
                self.phase = Phase::Declare;
            }
            Move::Declare(k) => {
                self.kind = *k;
                let d = self.declarer.idx();
                self.decl_cards = self.hands[d].iter().chain(&self.pile).copied().collect();
                self.phase = Phase::Play;
                self.side = self.dealer.other();
                self.led = None;
            }
            Move::Play(c) => {
                let me = self.side;
                let h = &mut self.hands[me.idx()];
                let pos = h.iter().position(|x| x == c).expect("card in hand");
                h.remove(pos);
                match self.led {
                    None => {
                        self.led = Some(*c);
                        self.side = me.other();
                    }
                    Some(l) => {
                        let winner = if second_wins(l, *c, self.kind) {
                            me
                        } else {
                            me.other()
                        };
                        self.pts[winner.idx()] += card_points(l) + card_points(*c);
                        self.tricks[winner.idx()] += 1;
                        self.trick_no += 1;
                        self.led = None;
                        self.side = winner;
                        if self.trick_no == 10 {
                            self.finish_hand();
                        }
                    }
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over() {
            return None;
        }
        Some(match self.total[0].cmp(&self.total[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    /// Citește doar: mâna lui `p` (la `skat`, fără cărțile din skat), licitația,
    /// jocul declarat și punctele din levate (punctele din skat doar pentru declarant).
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut s = (self.total[me] - self.total[1 - me]) * 20;
        let bid_value = |cap: i32, bid: u16| {
            if cap >= bid as i32 {
                10 + cap - bid as i32
            } else {
                -30
            }
        };
        match self.phase {
            Phase::HandOver | Phase::NoDeal | Phase::BidSolo => s,
            Phase::BidHold => {
                if self.bidder == p {
                    s + bid_value(Self::cap(&self.hands[me], false), self.cur_bid)
                } else {
                    s
                }
            }
            Phase::BidSay => {
                if self.cur_bid > 0 && self.responder() == p {
                    s + bid_value(Self::cap(&self.hands[me], false), self.cur_bid)
                } else {
                    s
                }
            }
            Phase::Pickup => {
                if self.declarer == p {
                    s + bid_value(Self::cap(&self.hands[me], false), self.bid)
                } else {
                    s
                }
            }
            Phase::Discard => {
                if self.declarer == p {
                    let own: Vec<u8> = self.hands[me]
                        .iter()
                        .copied()
                        .filter(|c| !self.skat.contains(c))
                        .collect();
                    s + bid_value(Self::cap(&own, false) + 8, self.bid)
                } else {
                    s
                }
            }
            Phase::Declare => {
                if self.declarer == p {
                    let cap = Self::cap(&self.hands[me], self.hand_game);
                    let cap = if self.hand_game { cap * 12 / 10 } else { cap };
                    let pile: i32 = self.pile.iter().map(|&c| card_points(c) as i32).sum();
                    s + bid_value(cap, self.bid) + if self.hand_game { 0 } else { pile / 2 }
                } else {
                    s
                }
            }
            Phase::Play => {
                let d = self.declarer;
                if self.trick_no == 0 && self.led.is_none() && self.declarer == p {
                    let q = Self::est(&self.hands[me], self.kind, self.hand_game);
                    s += bid_value(q, self.bid);
                }
                if self.kind == Kind::Null {
                    let t = self.tricks[d.idx()] as i32 * 300;
                    s += if p == d { -t } else { t };
                } else {
                    let dp = self.pts[d.idx()] as i32;
                    let op = self.pts[d.other().idx()] as i32;
                    if p == d {
                        // skat-ul nevăzut (joc din mână) nu se citește; cărțile puse jos le știe declarantul
                        let pile: i32 = if self.hand_game {
                            0
                        } else {
                            self.pile.iter().map(|&c| card_points(c) as i32).sum()
                        };
                        s += (dp + pile - op) * 10;
                    } else {
                        s += (op - dp) * 10;
                    }
                }
                for &c in &self.hands[me] {
                    s += self.hold(c);
                }
                // cartea mea condusă, așteptând răspunsul: se socotește aproape ca rămasă în mână
                if let Some(l) = self.led {
                    if self.side != p && self.kind != Kind::Null {
                        s += self.hold(l) / 2;
                    }
                }
                s
            }
        }
    }

    fn render(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            "Skat (2 jucători) — mâna {} din {HANDS}: J1 {} – J2 {}\n",
            (self.hands_done + 1).min(HANDS),
            self.total[0],
            self.total[1]
        ));
        let show_kind = matches!(self.phase, Phase::Play | Phase::HandOver);
        match self.phase {
            Phase::BidSay | Phase::BidHold | Phase::BidSolo => s.push_str(&format!(
                "Licitație: valoarea curentă {}; licitează Jucător {}\n",
                self.cur_bid,
                self.bidder.idx() + 1
            )),
            Phase::NoDeal => s.push_str("Nimeni n-a licitat. Scrie «reimparte».\n"),
            _ => s.push_str(&format!(
                "Declarant: Jucător {} (licitat {}){}\n",
                self.declarer.idx() + 1,
                self.bid,
                if show_kind {
                    format!(
                        ", joc: {}{}",
                        self.kind.name(),
                        if self.hand_game { " (din mână)" } else { "" }
                    )
                } else {
                    String::new()
                }
            )),
        }
        if self.phase == Phase::Play || self.phase == Phase::HandOver {
            s.push_str(&format!(
                "Levate: J1 {} ({} p.), J2 {} ({} p.)\n",
                self.tricks[0], self.pts[0], self.tricks[1], self.pts[1]
            ));
        }
        let v = self.viewer.who(self.side);
        s.push_str(&format!(
            "Jucător {} (ascuns) are {} cărți.\n",
            v.other().idx() + 1,
            self.hands[v.other().idx()].len()
        ));
        if let Some(l) = self.led {
            s.push_str(&format!(
                "Pe masă: Jucător {} a jucat {} {}\n",
                self.side.other().idx() + 1,
                card_text(l),
                SUIT_SYMBOLS[suit(l) as usize]
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
        if self.phase == Phase::HandOver {
            if let Some(t) = &self.last_hand {
                s.push_str(&format!("{t}. Scrie «urmatoarea».\n"));
            }
        }
        match self.outcome() {
            Some(Outcome::Win(p)) => s.push_str(&format!(
                "Meci încheiat: Jucător {} câștigă ({}–{}).\n",
                p.idx() + 1,
                self.total[0],
                self.total[1]
            )),
            Some(Outcome::Draw) => s.push_str("Meci încheiat: remiză.\n"),
            None => {
                if !matches!(self.phase, Phase::NoDeal | Phase::HandOver) {
                    s.push_str(&format!("La mutare: Jucător {}\n", self.side.idx() + 1));
                }
            }
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Say(b) => format!("licit {b}"),
            Move::Hold => "tin".into(),
            Move::Pass => "pas".into(),
            Move::TakeSkat => "skat".into(),
            Move::Hand => "hand".into(),
            Move::Put(a, b) => format!("pune {} {}", card_text(*a), card_text(*b)),
            Move::Declare(k) => k.name().into(),
            Move::Play(c) => card_text(*c),
            Move::Redeal => "reimparte".into(),
            Move::Next => "urmatoarea".into(),
        }
    }

    /// Adâncime 0 (o singură mutare): `AlphaBeta(1)` ar fi extins la Hard/Expert răspunsul adversarului din
    /// mâna lui reală; evaluarea citește doar mâna proprie, scorurile publice și (nu) skat-ul.
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn viewer_controls_whose_hand_is_shown() {
        let hand_txt = |g: &Skat, p: usize| {
            let mut h = g.hands[p].clone();
            h.sort_unstable();
            h.iter()
                .map(|&c| card_text(c))
                .collect::<Vec<_>>()
                .join(" ")
        };
        let mut g = Skat::with_seed(5);
        Game::set_viewer(&mut g, None);
        let mut movers = [false; 2];
        for _ in 0..6 {
            let tm = g.to_move().idx();
            movers[tm] = true;
            let r = Game::render(&g);
            assert!(r.contains(&hand_txt(&g, tm)), "{r}");
            assert!(!r.contains(&hand_txt(&g, 1 - tm)), "{r}");
            let m = Game::legal_moves(&g)[0].clone();
            g.apply(&m);
        }
        assert!(movers[0] && movers[1]);
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

    // culoare: 0 c, 1 s, 2 h, 3 d; rang 0=7 1=8 2=9 3=10 4=J 5=Q 6=K 7=A
    fn c(s: u8, r: u8) -> u8 {
        make_card(s, r)
    }

    #[test]
    fn deck_points_120() {
        let t: u16 = (0..32).map(card_points).sum();
        assert_eq!(t, 120);
    }

    #[test]
    fn trump_groups_and_order() {
        let k = Kind::Suit(2); // cupa
        assert_eq!(group(c(0, JACK), k), 4);
        assert_eq!(group(c(2, 0), k), 4);
        assert_eq!(group(c(1, 7), k), 1);
        // J♣ > J♠ > J♥ > J♦ > A♥
        assert!(second_wins(c(1, JACK), c(0, JACK), k));
        assert!(second_wins(c(3, JACK), c(1, JACK), k));
        assert!(!second_wins(c(1, JACK), c(3, JACK), k));
        assert!(second_wins(c(2, 7), c(3, JACK), k));
        // zecele bate regele; atu mic bate asul altei culori
        assert!(second_wins(c(1, 6), c(1, 3), k));
        assert!(second_wins(c(1, 7), c(2, 0), k));
        // grand: doar juniorii sunt atu
        assert_eq!(group(c(2, 0), Kind::Grand), 2);
        assert_eq!(group(c(2, JACK), Kind::Grand), 4);
        // null: A K Q J 10 9 8 7
        assert!(second_wins(c(1, 3), c(1, JACK), Kind::Null));
        assert!(second_wins(c(1, JACK), c(1, 5), Kind::Null));
        assert!(!second_wins(c(1, 5), c(1, 3), Kind::Null));
    }

    #[test]
    fn matadors_with_and_without() {
        let k = Kind::Suit(2);
        // cu 2: J♣ J♠, fără J♥
        let h = vec![c(0, JACK), c(1, JACK), c(2, 7), c(2, 3)];
        assert_eq!(matadors(&h, k), 2);
        // fără 1: nu are J♣, are J♠
        let h = vec![c(1, JACK), c(2, 7)];
        assert_eq!(matadors(&h, k), 1);
        // cu 4 + A + 10 la atu cupa: J♣ J♠ J♥ J♦ A♥ 10♥ ... = 6 (următorul K lipsește)
        let h = vec![
            c(0, JACK),
            c(1, JACK),
            c(2, JACK),
            c(3, JACK),
            c(2, 7),
            c(2, 3),
        ];
        assert_eq!(matadors(&h, k), 6);
        assert_eq!(matadors(&h, Kind::Grand), 4);
    }

    fn base() -> Skat {
        let mut g = Skat::with_seed(1);
        g.hands = [Vec::new(), Vec::new()];
        g.declarer = Player::Two;
        g.phase = Phase::Play;
        g.side = Player::One;
        g.led = None;
        g.tricks = [0; 2];
        g.pts = [0; 2];
        g.trick_no = 9;
        g.pile.clear();
        g.hand_game = false;
        g
    }

    #[test]
    fn follow_suit_with_jacks_as_trumps() {
        let mut g = base();
        g.kind = Kind::Suit(2);
        g.led = Some(c(1, 7)); // A♠
        g.side = Player::One;
        g.hands[0] = vec![c(1, JACK), c(1, 0), c(3, 7)];
        // J♠ e atu: trebuie doar 7♠
        assert_eq!(g.playable(Player::One), vec![c(1, 0)]);
        g.hands[0] = vec![c(1, JACK), c(3, 7)];
        assert_eq!(g.playable(Player::One).len(), 2); // fără pică: orice
        g.led = Some(c(0, JACK));
        g.hands[0] = vec![c(2, 0), c(3, JACK), c(1, 7)];
        let mut p = g.playable(Player::One);
        p.sort_unstable();
        assert_eq!(p, vec![c(2, 0), c(3, JACK)]); // atu condus: atu
    }

    #[test]
    fn scoring_win_loss_overbid_and_null() {
        // declarantul (J2) câștigă cu 70 de puncte, cu 1 matador, trefla
        let mut g = base();
        g.kind = Kind::Suit(0);
        g.decl_cards = vec![c(0, JACK), c(0, 7), c(1, 0)];
        g.bid = 18;
        g.pts = [30, 70];
        g.tricks = [4, 6];
        g.finish_hand();
        // cu 1 (J♣, apoi J♠ lipsește) => 1 + 1 = 2 × 12 = 24
        assert_eq!(g.total[1], 24);
        // pierdere: 50 de puncte: -2 * 24
        let mut g = base();
        g.kind = Kind::Suit(0);
        g.decl_cards = vec![c(0, JACK), c(0, 7), c(1, 0)];
        g.bid = 18;
        g.pts = [70, 50];
        g.tricks = [5, 5];
        g.finish_hand();
        assert_eq!(g.total[1], -48);
        // supralicitat: valoare 24 (2×12), licitat 30 -> pierdut: ceil(30/12)=3 -> 36 -> -72
        let mut g = base();
        g.kind = Kind::Suit(0);
        g.decl_cards = vec![c(0, JACK), c(0, 7), c(1, 0)];
        g.bid = 30;
        g.pts = [30, 90];
        g.tricks = [3, 7];
        g.finish_hand();
        // 90 puncte => schneider (+1): mult 3 => 36 >= 30 : câștigat 36
        assert_eq!(g.total[1], 36);
        let mut g = base();
        g.kind = Kind::Suit(0);
        g.decl_cards = vec![c(0, JACK), c(0, 7), c(1, 0)];
        g.bid = 36;
        g.pts = [40, 80];
        g.tricks = [4, 6];
        g.finish_hand();
        assert_eq!(g.total[1], -72); // valoare 24 < 36: supralicitat, ceil=36
                                     // null din mână: 35, fără levate
        let mut g = base();
        g.kind = Kind::Null;
        g.hand_game = true;
        g.bid = 23;
        g.tricks = [10, 0];
        g.finish_hand();
        assert_eq!(g.total[1], 35);
        let mut g = base();
        g.kind = Kind::Null;
        g.bid = 23;
        g.tricks = [9, 1];
        g.finish_hand();
        assert_eq!(g.total[1], -46);
    }

    #[test]
    fn schwarz_counts_both_ways() {
        let mut g = base();
        g.kind = Kind::Grand;
        g.decl_cards = vec![c(0, JACK), c(1, JACK), c(2, 7)];
        g.bid = 18;
        g.pts = [0, 120];
        g.tricks = [0, 10];
        g.finish_hand();
        // cu 2 + 1 + schneider + schwarz = 5 × 24 = 120
        assert_eq!(g.total[1], 120);
    }

    #[test]
    fn bidding_flow_and_forced_steps() {
        let mut g = Skat::with_seed(5);
        assert_eq!(g.to_move(), Player::Two); // dealer licitează
        assert_eq!(g.legal_moves(), vec![Move::Say(18), Move::Pass]);
        g.apply(&Move::Say(18));
        assert_eq!(g.to_move(), Player::One);
        g.apply(&Move::Hold);
        assert_eq!(g.to_move(), Player::Two);
        assert_eq!(g.legal_moves()[0], Move::Say(20));
        g.apply(&Move::Say(20));
        g.apply(&Move::Pass); // J1 renunță: J2 e declarant cu 20
        assert_eq!(g.declarer, Player::Two);
        assert_eq!(g.bid, 20);
        assert_eq!(g.phase, Phase::Pickup);
        g.apply(&Move::TakeSkat);
        assert_eq!(g.hands[1].len(), 12);
        assert_eq!(g.legal_moves().len(), 66);
        let mv = g.legal_moves()[0].clone();
        g.apply(&mv);
        assert_eq!(g.hands[1].len(), 10);
        assert_eq!(g.pile.len(), 2);
        assert_eq!(g.legal_moves().len(), 6);
        g.apply(&Move::Declare(Kind::Grand));
        assert_eq!(g.phase, Phase::Play);
        assert_eq!(g.to_move(), Player::One); // cine n-a împărțit conduce
        assert_eq!(g.decl_cards.len(), 12);
    }

    #[test]
    fn first_pass_gives_the_other_a_chance_and_redeal_counts() {
        let mut g = Skat::with_seed(6);
        g.apply(&Move::Pass);
        assert_eq!(g.phase, Phase::BidSolo);
        assert_eq!(g.to_move(), Player::One);
        g.apply(&Move::Pass);
        assert_eq!(g.legal_moves(), vec![Move::Redeal]);
        g.apply(&Move::Redeal);
        assert_eq!(g.hands_done, 1);
        assert_eq!(g.phase, Phase::BidSay);
        assert_eq!(g.dealer, Player::One);
        // dacă licitantul trece, celălalt poate declara 18
        g.apply(&Move::Pass);
        g.apply(&Move::Say(18));
        assert_eq!(g.declarer, Player::Two);
        assert_eq!(g.bid, 18);
    }

    #[test]
    fn hand_game_keeps_the_skat_for_the_declarer() {
        let mut g = Skat::with_seed(7);
        g.apply(&Move::Say(18));
        g.apply(&Move::Pass);
        assert_eq!(g.declarer, Player::Two);
        let sk = g.skat;
        g.apply(&Move::Hand);
        assert!(g.hand_game);
        assert_eq!(g.pile, sk.to_vec());
        assert_eq!(g.hands[1].len(), 10);
    }

    #[test]
    fn deal_conserves_cards() {
        let g = Skat::with_seed(3);
        let mut all: Vec<u8> = g.hands[0]
            .iter()
            .chain(&g.hands[1])
            .chain(&g.skat)
            .copied()
            .collect();
        all.sort_unstable();
        all.dedup();
        assert_eq!(all.len(), 22);
    }

    #[test]
    fn random_matches_finish() {
        for seed in 1..=30u64 {
            let mut g = Skat::with_seed(seed);
            let mut rng = Rng::new(seed ^ 0x13);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 20000);
            }
            assert_eq!(g.hands_done, HANDS);
        }
    }

    #[test]
    fn render_hides_the_opponent() {
        let g = Skat::with_seed(5);
        assert!(Game::render(&g).contains("Jucător 2 (ascuns) are 10 cărți"));
    }

    #[test]
    fn evaluate_ignores_the_opponents_cards_and_the_skat() {
        let g = Skat::with_seed(9);
        let mut h = g.clone();
        // schimbă o carte a adversarului cu una din skat
        let (a, b) = (h.hands[1][0], h.skat[0]);
        h.hands[1][0] = b;
        h.skat[0] = a;
        assert_eq!(g.evaluate(Player::One), h.evaluate(Player::One));
        // după licitație, J2 declarant: `skat` nu trebuie evaluat după conținutul skat-ului
        let mut x = Skat::with_seed(11);
        x.apply(&Move::Say(18));
        x.apply(&Move::Pass);
        let mut y = x.clone();
        y.skat = [c(0, JACK), c(1, JACK)];
        let mut x2 = x.clone();
        x2.apply(&Move::TakeSkat);
        y.apply(&Move::TakeSkat);
        // dacă skat-ul diferă dar mâinile proprii sunt la fel, evaluarea e aceeași
        let own_x: Vec<u8> = x2.hands[1][..10].to_vec();
        assert_eq!(own_x, y.hands[1][..10].to_vec());
        assert_eq!(x2.evaluate(Player::Two), y.evaluate(Player::Two));
    }

    #[test]
    fn ai_beats_a_random_mover_mostly() {
        use polytabletop_core::GameObj;
        let mut wins = 0;
        let n = 20;
        for seed in 1..=n {
            let mut g: Box<dyn GameObj> = Box::new(Skat::with_seed(seed));
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
    fn ai_nu_vede_mana_adversarului_nici_skat_ul_la_orice_nivel() {
        crate::p3_comun::ai_blind_check(
            "skat",
            Skat::with_seed,
            |g, me, seed| {
                let (m, o) = (me.idx(), 1 - me.idx());
                let mut pool: Vec<u8> = (0..32u8)
                    .filter(|c| {
                        !g.hands[m].contains(c)
                            && g.led != Some(*c)
                            && !(g.declarer == me && g.pile.contains(c) && !g.hand_game)
                    })
                    .collect();
                crate::p3_comun::shuffle_with(&mut pool, seed);
                let n = g.hands[o].len();
                g.hands[o] = pool.drain(..n).collect();
                // skat-ul rămâne necunoscut jucătorului până nu îl ia în mână
                let skat_unseen = matches!(
                    g.phase,
                    Phase::BidSay | Phase::BidHold | Phase::BidSolo | Phase::NoDeal | Phase::Pickup
                ) || g.hand_game
                    || g.declarer != me;
                if skat_unseen {
                    g.skat = [pool.pop().unwrap(), pool.pop().unwrap()];
                    if g.hand_game {
                        g.pile = g.skat.to_vec();
                    }
                }
                if g.declarer == me.other() && !g.hand_game && !g.pile.is_empty() {
                    let k = g.pile.len();
                    g.pile = pool.drain(..k).collect();
                }
                g.rng = Rng::new(seed ^ 0x5CA7);
            },
            12,
        );
    }
}
