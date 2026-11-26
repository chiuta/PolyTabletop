#![allow(
    clippy::needless_range_loop,
    clippy::collapsible_if,
    clippy::manual_is_multiple_of,
    clippy::type_complexity,
    clippy::too_many_arguments
)]
//! Schafkopf, Preferans și Ulti — jocuri de levate cu 32 de cărți, adaptate pentru **2 jucători**
//! (ca la Skat în PolyTabletop): un jucător singur (declarant) contra celuilalt.
//!
//! Motor comun, trei seturi de reguli:
//!
//! * **Schafkopf** (Bavaria): 8 cărți fiecare, restul de 16 rămân ascunse și nu se joacă. Contracte: `solo-c`,
//!   `solo-s`, `solo-h`, `solo-d` (atu = Oberii, Unterii și culoarea) sau `wenz` (atu = doar Unterii =
//!   valeții). Ordinea atuurilor: Ober (♣ ♠ ♥ ♦), Unter (♣ ♠ ♥ ♦), apoi A 10 K 9 8 7 din culoare.
//!   Puncte: A 11, 10 10, K 4, Ober (Q) 3, Unter (J) 2 — 120 în total; declarantul are nevoie de 61.
//!   Valoarea: 3 (+1 schneider ≥90 sau ≤30, +1 schwarz = toate / nicio levată).
//! * **Preferans** (Rusia): 10 cărți fiecare, talon de 2, restul 10 ascunse. Licitezi `licit 6s` … `licit 10nt` (nivel = numărul
//!   de levate promis, culoarea de atu s, c, d, h sau `nt` fără atu); declarantul ia talonul și pune 2 cărți. Ordinea
//!   culorilor: s < c < d < h < nt. Valoarea contractului 2·(nivel − 5); reușit = +valoarea, ratat = −valoarea
//!   pentru fiecare levată lipsă. Obligatoriu răspunzi la culoare, iar fără culoare tai cu atu dacă ai.
//! * **Ulti** (Ungaria): 10 cărți fiecare, talon de 2; ordinea cărților A 10 K Q J 9 8 7; A și 10 valorează 10, ultima levată
//!   10 (90 în total). Contracte `parti-h` (atu cupă) … sau `ulti-h` (parti + ultimul levat cu 7 de atu). Parti: ≥ 46 de
//!   puncte = +1, altfel −1; ulti anunțat: +4 dacă iei ultima levată cu 7 de atu, altfel −4.
//!
//! Licitație (pentru doi): primul (cel care nu a dat) licitează sau dă `pas`; al doilea trebuie să
//! licitează mai sus sau să dea pas; dacă ambii dau pas se împarte din nou. Declarantul conduce
//! prima levată. Meciul are 6 mâini; câștigă cine are mai multe puncte.
//!
//! Mutări: `licit solo-c`, `licit 7h`, `pas`, `talon`, `pune 7h 8c`, `Ah`, `urmatoarea`.
//! Informație ascunsă: vezi `p3_comun` (AI cinstit prin determinizare).

use crate::cards::{full_deck, handover_banner, join_cards, shuffle, Card, Viewer};
use crate::p3_comun::{honest_eval, Honest};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const HANDS: u8 = 6;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Schafkopf,
    Preferans,
    Ulti,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Contract {
    /// poziția în scara de licitație (mai mare = mai tare)
    pub rank: u8,
    /// culoarea de atu (0 h, 1 d, 2 s, 3 c) sau None
    pub trump: Option<u8>,
    pub wenz: bool,
    pub level: u8,
    pub ulti: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Bid(Contract),
    Pass,
    Take,
    Discard(Card, Card),
    Play(Card),
    Next,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Bid,
    Take,
    Discard,
    Play,
    HandOver,
    Over,
}

const SUIT_CH: [char; 4] = ['h', 'd', 's', 'c'];
/// ordinea culorilor la Schafkopf (♣ cel mai tare): h=1, d=0, s=2, c=3
const SK_SUIT: [u8; 4] = [1, 0, 2, 3];

/// Toate contractele unui joc, în ordinea scării.
pub fn contracts(kind: Kind) -> Vec<Contract> {
    let mut v = Vec::new();
    match kind {
        Kind::Schafkopf => {
            v.push(Contract {
                rank: 1,
                trump: None,
                wenz: true,
                level: 0,
                ulti: false,
            });
            for (i, s) in [1u8, 0, 2, 3].iter().enumerate() {
                v.push(Contract {
                    rank: 2 + i as u8,
                    trump: Some(*s),
                    wenz: false,
                    level: 0,
                    ulti: false,
                });
            }
        }
        Kind::Preferans => {
            // s < c < d < h < nt
            let order: [Option<u8>; 5] = [Some(2), Some(3), Some(1), Some(0), None];
            for level in 6..=10u8 {
                for (i, t) in order.iter().enumerate() {
                    v.push(Contract {
                        rank: (level - 6) * 5 + i as u8 + 1,
                        trump: *t,
                        wenz: false,
                        level,
                        ulti: false,
                    });
                }
            }
        }
        Kind::Ulti => {
            for (k, ulti) in [false, true].iter().enumerate() {
                for (i, s) in [1u8, 0, 2, 3].iter().enumerate() {
                    v.push(Contract {
                        rank: (k * 4 + i) as u8 + 1,
                        trump: Some(*s),
                        wenz: false,
                        level: 0,
                        ulti: *ulti,
                    });
                }
            }
        }
    }
    v
}

fn contract_text(kind: Kind, c: &Contract) -> String {
    match kind {
        Kind::Schafkopf => {
            if c.wenz {
                "wenz".into()
            } else {
                format!("solo-{}", SUIT_CH[c.trump.unwrap() as usize])
            }
        }
        Kind::Preferans => format!(
            "{}{}",
            c.level,
            c.trump
                .map(|t| SUIT_CH[t as usize].to_string())
                .unwrap_or_else(|| "nt".into())
        ),
        Kind::Ulti => format!(
            "{}-{}",
            if c.ulti { "ulti" } else { "parti" },
            SUIT_CH[c.trump.unwrap() as usize]
        ),
    }
}

#[derive(Clone)]
pub struct Trick32 {
    kind: Kind,
    rng: Rng,
    hands: [Vec<Card>; 2],
    talon: Vec<Card>,
    discard: Vec<Card>,
    contract: Option<Contract>,
    declarer: Player,
    bid_turn: u8,
    first_bidder: Player,
    standing: Option<(Player, Contract)>,
    trick: Vec<(Player, Card)>,
    taken: [Vec<Card>; 2],
    tricks_won: [u8; 2],
    last_winner: Option<Player>,
    last_trump_seven: bool,
    side: Player,
    dealer: Player,
    scores: [i32; 2],
    hand_no: u8,
    gain: [i32; 2],
    redeal: bool,
    phase: Phase,
    prev: Player,
    log: String,
    viewer: Viewer,
}

impl Trick32 {
    pub fn schafkopf() -> Self {
        Self::with_seed(Kind::Schafkopf, Rng::from_clock().next_u64())
    }
    pub fn preferans() -> Self {
        Self::with_seed(Kind::Preferans, Rng::from_clock().next_u64())
    }
    pub fn ulti() -> Self {
        Self::with_seed(Kind::Ulti, Rng::from_clock().next_u64())
    }

    pub fn with_seed(kind: Kind, seed: u64) -> Self {
        let mut g = Trick32 {
            kind,
            rng: Rng::new(seed),
            hands: [Vec::new(), Vec::new()],
            talon: Vec::new(),
            discard: Vec::new(),
            contract: None,
            declarer: Player::One,
            bid_turn: 0,
            first_bidder: Player::Two,
            standing: None,
            trick: Vec::new(),
            taken: [Vec::new(), Vec::new()],
            tricks_won: [0, 0],
            last_winner: None,
            last_trump_seven: false,
            side: Player::Two,
            dealer: Player::One,
            scores: [0, 0],
            hand_no: 1,
            gain: [0, 0],
            redeal: false,
            phase: Phase::Bid,
            prev: Player::Two,
            log: String::new(),
            viewer: Viewer::Fixed(Player::One),
        };
        g.deal();
        g
    }

    pub fn kind(&self) -> Kind {
        self.kind
    }

    pub fn scores(&self) -> (i32, i32) {
        (self.scores[0], self.scores[1])
    }

    fn per_hand(&self) -> usize {
        match self.kind {
            Kind::Schafkopf => 8,
            _ => 10,
        }
    }

    fn deal(&mut self) {
        let mut d = full_deck(7);
        shuffle(&mut d, &mut self.rng);
        let n = self.per_hand();
        self.hands = [Vec::new(), Vec::new()];
        for _ in 0..n {
            for p in 0..2 {
                self.hands[p].push(d.pop().unwrap());
            }
        }
        self.talon = if self.kind == Kind::Schafkopf {
            Vec::new()
        } else {
            vec![d.pop().unwrap(), d.pop().unwrap()]
        };
        self.sort_hands();
        self.discard.clear();
        self.contract = None;
        self.bid_turn = 0;
        self.standing = None;
        self.trick.clear();
        self.taken = [Vec::new(), Vec::new()];
        self.tricks_won = [0, 0];
        self.last_winner = None;
        self.last_trump_seven = false;
        self.gain = [0, 0];
        self.phase = Phase::Bid;
        self.first_bidder = self.dealer.other();
        self.side = self.first_bidder;
        self.prev = self.side;
    }

    fn sort_hands(&mut self) {
        for h in &mut self.hands {
            h.sort_by_key(|c| (c.suit, std::cmp::Reverse(c.rank)));
        }
    }

    // ---- reguli de carte ----
    fn is_trump(&self, c: Card) -> bool {
        let Some(k) = self.contract else { return false };
        match self.kind {
            Kind::Schafkopf => {
                if k.wenz {
                    c.rank == 11
                } else {
                    c.rank == 12 || c.rank == 11 || Some(c.suit) == k.trump
                }
            }
            _ => Some(c.suit) == k.trump,
        }
    }

    /// Forța unei cărți în interiorul categoriei ei (atu / culoare).
    fn power(&self, c: Card) -> u32 {
        let wenz = self.contract.map(|k| k.wenz).unwrap_or(false);
        match self.kind {
            Kind::Schafkopf => {
                if self.is_trump(c) {
                    if c.rank == 12 && !wenz {
                        return 100 + SK_SUIT[c.suit as usize] as u32;
                    }
                    if c.rank == 11 {
                        return 80 + SK_SUIT[c.suit as usize] as u32;
                    }
                }
                match c.rank {
                    14 => 6,
                    10 => 5,
                    13 => 4,
                    12 => 3.max(if wenz { 3 } else { 0 }),
                    9 => 2,
                    8 => 1,
                    _ => 0,
                }
            }
            Kind::Preferans => c.rank as u32,
            Kind::Ulti => match c.rank {
                14 => 8,
                10 => 7,
                13 => 6,
                12 => 5,
                11 => 4,
                9 => 3,
                8 => 2,
                _ => 1,
            },
        }
    }

    fn points(&self, c: Card) -> i32 {
        match self.kind {
            Kind::Schafkopf => match c.rank {
                14 => 11,
                10 => 10,
                13 => 4,
                12 => 3,
                11 => 2,
                _ => 0,
            },
            Kind::Preferans => 0,
            Kind::Ulti => {
                if c.rank == 14 || c.rank == 10 {
                    10
                } else {
                    0
                }
            }
        }
    }

    fn category(&self, c: Card) -> i8 {
        if self.is_trump(c) {
            -1
        } else {
            c.suit as i8
        }
    }

    fn playable_cards(&self) -> Vec<Card> {
        let h = &self.hands[self.side.idx()];
        let Some(&(_, lead)) = self.trick.first() else {
            return h.clone();
        };
        let cat = self.category(lead);
        let follow: Vec<Card> = h
            .iter()
            .copied()
            .filter(|&c| self.category(c) == cat)
            .collect();
        if !follow.is_empty() {
            return follow;
        }
        if self.kind != Kind::Schafkopf {
            let tr: Vec<Card> = h.iter().copied().filter(|&c| self.is_trump(c)).collect();
            if !tr.is_empty() {
                return tr;
            }
        }
        h.clone()
    }

    fn trick_winner(&self) -> Player {
        let lead = self.trick[0].1;
        let lc = self.category(lead);
        let mut best = (self.trick[0].0, 0u32);
        for &(p, c) in &self.trick {
            let cc = self.category(c);
            let v = if cc == -1 {
                1000 + self.power(c)
            } else if cc == lc {
                self.power(c)
            } else {
                continue;
            };
            if v > best.1 || (best.1 == 0 && p == self.trick[0].0 && v == 0 && false) {
                best = (p, v);
            }
        }
        // primul jucat cu v>0; dacă toate valorile sunt 0 (de ex. toate cărțile cele mai slabe) câștigă cel care a condus
        let mut winner = self.trick[0].0;
        let mut bv = -1i64;
        for &(p, c) in &self.trick {
            let cc = self.category(c);
            let v: i64 = if cc == -1 {
                1000 + self.power(c) as i64
            } else if cc == lc {
                self.power(c) as i64
            } else {
                -2
            };
            if v > bv {
                bv = v;
                winner = p;
            }
        }
        let _ = best;
        winner
    }

    fn card_points_of(&self, p: Player) -> i32 {
        self.taken[p.idx()].iter().map(|&c| self.points(c)).sum()
    }

    fn finish_hand(&mut self) {
        let d = self.declarer;
        let o = d.other();
        let k = self.contract.unwrap();
        let (value, msg) = match self.kind {
            Kind::Schafkopf => {
                let pts = self.card_points_of(d);
                let won = pts >= 61;
                let mut v = 3;
                let schneider = pts >= 90 || pts <= 30;
                let schwarz = self.tricks_won[d.idx()] == 0 || self.tricks_won[o.idx()] == 0;
                if schneider {
                    v += 1;
                }
                if schwarz {
                    v += 1;
                }
                (if won { v } else { -v }, format!("declarant {pts} puncte"))
            }
            Kind::Preferans => {
                let need = k.level;
                let got = self.tricks_won[d.idx()];
                let val = 2 * (need as i32 - 5);
                if got >= need {
                    (val, format!("declarant {got} levate din {need}"))
                } else {
                    (
                        -val * (need - got) as i32,
                        format!("declarant {got} levate din {need}"),
                    )
                }
            }
            Kind::Ulti => {
                let mut pts = self.card_points_of(d);
                if self.last_winner == Some(d) {
                    pts += 10;
                }
                let mut v = if pts >= 46 { 1 } else { -1 };
                if k.ulti {
                    v += if self.last_winner == Some(d) && self.last_trump_seven {
                        4
                    } else {
                        -4
                    };
                }
                (v, format!("declarant {pts} puncte"))
            }
        };
        self.scores[d.idx()] += value;
        self.scores[o.idx()] -= value;
        self.gain[d.idx()] += value;
        self.gain[o.idx()] -= value;
        self.log = format!("{msg}: J{} {:+}", d.idx() + 1, value);
        self.phase = if self.hand_no >= HANDS {
            Phase::Over
        } else {
            Phase::HandOver
        };
    }
}

impl Game for Trick32 {
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
            Phase::Take => vec![Move::Take],
            Phase::Bid => {
                let floor = self.standing.map(|(_, c)| c.rank).unwrap_or(0);
                let mut v: Vec<Move> = contracts(self.kind)
                    .into_iter()
                    .filter(|c| c.rank > floor)
                    .map(Move::Bid)
                    .collect();
                v.push(Move::Pass);
                v
            }
            Phase::Discard => {
                let h = &self.hands[self.side.idx()];
                let mut v = Vec::new();
                for i in 0..h.len() {
                    for j in i + 1..h.len() {
                        v.push(Move::Discard(h[i], h[j]));
                    }
                }
                v
            }
            Phase::Play => {
                let mut v: Vec<Move> = self.playable_cards().into_iter().map(Move::Play).collect();
                v.dedup();
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        self.prev = self.side;
        match *m {
            Move::Next => {
                if !self.redeal {
                    self.hand_no += 1;
                }
                self.redeal = false;
                self.dealer = self.dealer.other();
                self.deal();
            }
            Move::Bid(c) => {
                self.standing = Some((self.side, c));
                self.advance_bid();
            }
            Move::Pass => self.advance_bid(),
            Move::Take => {
                let d = self.declarer.idx();
                let t = std::mem::take(&mut self.talon);
                self.hands[d].extend(t);
                self.sort_hands();
                self.phase = Phase::Discard;
            }
            Move::Discard(a, b) => {
                let d = self.declarer.idx();
                self.hands[d].retain(|&c| c != a && c != b);
                self.discard = vec![a, b];
                self.start_play();
            }
            Move::Play(c) => {
                let p = self.side;
                let pos = self.hands[p.idx()].iter().position(|&x| x == c).unwrap();
                self.hands[p.idx()].remove(pos);
                self.trick.push((p, c));
                if self.trick.len() == 1 {
                    self.side = p.other();
                } else {
                    let w = self.trick_winner();
                    let cards: Vec<Card> = self.trick.iter().map(|x| x.1).collect();
                    let last = self.hands[0].is_empty() && self.hands[1].is_empty();
                    if last {
                        let k = self.contract.unwrap();
                        self.last_trump_seven = self
                            .trick
                            .iter()
                            .any(|&(q, x)| q == w && x.rank == 7 && Some(x.suit) == k.trump);
                    }
                    self.taken[w.idx()].extend(cards);
                    self.tricks_won[w.idx()] += 1;
                    self.last_winner = Some(w);
                    self.trick.clear();
                    self.side = w;
                    if last {
                        self.finish_hand();
                    }
                }
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
        if self.prev != self.side
            && matches!(
                self.phase,
                Phase::Bid | Phase::Play | Phase::Discard | Phase::Take
            )
        {
            if let Viewer::HotSeat = self.viewer {
                s.push_str(&handover_banner(self.side));
            }
        }
        let name = match self.kind {
            Kind::Schafkopf => "Schafkopf (2 jucători)",
            Kind::Preferans => "Preferans (2 jucători)",
            Kind::Ulti => "Ulti (2 jucători)",
        };
        s.push_str(&format!(
            "{name} — mâna {}/{} | scor: J1 {}, J2 {}\n",
            self.hand_no, HANDS, self.scores[0], self.scores[1]
        ));
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        if let Some(k) = self.contract {
            s.push_str(&format!(
                "Contract: {} (declarant J{}) | levate: J1 {}, J2 {}\n",
                contract_text(self.kind, &k),
                self.declarer.idx() + 1,
                self.tricks_won[0],
                self.tricks_won[1]
            ));
        } else if let Some((p, c)) = self.standing {
            s.push_str(&format!(
                "Licitație: J{} a licitat {}\n",
                p.idx() + 1,
                contract_text(self.kind, &c)
            ));
        }
        if self.phase == Phase::Over {
            s.push_str("Meci încheiat.\n");
            return s;
        }
        if self.phase == Phase::HandOver {
            s.push_str("Scrie «urmatoarea» pentru următoarea împărțire.\n");
            return s;
        }
        if !self.trick.is_empty() {
            s.push_str(&format!(
                "Pe masă: {}\n",
                self.trick
                    .iter()
                    .map(|(p, c)| format!("J{}:{}", p.idx() + 1, c.text()))
                    .collect::<Vec<_>>()
                    .join(" ")
            ));
        }
        let v = self.viewer.who(self.side);
        s.push_str(&format!(
            "Mâna ta (J{}): {}\n",
            v.idx() + 1,
            join_cards(&self.hands[v.idx()])
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
            Move::Bid(c) => format!("licit {}", contract_text(self.kind, c)),
            Move::Pass => "pas".into(),
            Move::Take => "talon".into(),
            Move::Discard(a, b) => format!("pune {} {}", a.text(), b.text()),
            Move::Play(c) => c.text(),
            Move::Next => "urmatoarea".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

impl Trick32 {
    fn advance_bid(&mut self) {
        self.bid_turn += 1;
        if self.bid_turn >= 2 {
            match self.standing {
                None => {
                    self.log = "Ambii au dat pas: se împarte din nou".into();
                    self.redeal = true;
                    self.phase = Phase::HandOver;
                }
                Some((p, c)) => {
                    self.declarer = p;
                    self.contract = Some(c);
                    self.log = format!("J{} joacă {}", p.idx() + 1, contract_text(self.kind, &c));
                    if self.kind == Kind::Schafkopf {
                        self.start_play();
                    } else {
                        self.side = p;
                        self.phase = Phase::Take;
                    }
                }
            }
        } else {
            self.side = self.side.other();
        }
    }

    fn start_play(&mut self) {
        self.phase = Phase::Play;
        self.side = self.declarer;
        self.prev = self.side;
        self.sort_hands();
    }

    /// Heuristică pentru politica din simulări.
    fn pick_play(&self, rng: &mut Rng, cards: &[Card]) -> Card {
        if cards.len() == 1 || rng.below(100) < 10 {
            return cards[rng.below(cards.len())];
        }
        let val = |c: Card| -> i64 {
            let t = if self.is_trump(c) { 1000 } else { 0 };
            t + self.power(c) as i64 * 10 + self.points(c) as i64
        };
        if let Some(&(_, lead)) = self.trick.first() {
            let lc = self.category(lead);
            let cur_best = self
                .trick
                .iter()
                .map(|&(_, c)| {
                    if self.category(c) == -1 {
                        1000 + self.power(c) as i64
                    } else if self.category(c) == lc {
                        self.power(c) as i64
                    } else {
                        -1
                    }
                })
                .max()
                .unwrap();
            let beats = |c: Card| -> bool {
                let v = if self.category(c) == -1 {
                    1000 + self.power(c) as i64
                } else if self.category(c) == lc {
                    self.power(c) as i64
                } else {
                    -2
                };
                v > cur_best
            };
            let mut winners: Vec<Card> = cards.iter().copied().filter(|&c| beats(c)).collect();
            if !winners.is_empty() {
                winners.sort_by_key(|&c| val(c));
                return winners[0];
            }
            let mut v = cards.to_vec();
            v.sort_by_key(|&c| self.points(c) as i64 * 100 + val(c));
            v[0]
        } else {
            let mut v = cards.to_vec();
            if self.side == self.declarer {
                v.sort_by_key(|&c| std::cmp::Reverse(val(c)));
            } else {
                v.sort_by_key(|&c| val(c));
            }
            v[0]
        }
    }
}

impl Honest for Trick32 {
    fn determinize(&self, me: Player, rng: &mut Rng) -> Self {
        let mut g = self.clone();
        let mi = me.idx();
        let oi = 1 - mi;
        let mut known: Vec<Card> = self.hands[mi].clone();
        known.extend(self.taken[0].iter().copied());
        known.extend(self.taken[1].iter().copied());
        known.extend(self.trick.iter().map(|x| x.1));
        if self.declarer == me && self.contract.is_some() {
            known.extend(self.discard.iter().copied());
        }
        let talon_known = self.contract.is_some() && self.declarer == me && self.talon.is_empty();
        let _ = talon_known;
        let mut pool = full_deck(7);
        pool.retain(|c| !known.contains(c));
        shuffle(&mut pool, rng);
        let n_opp = self.hands[oi].len();
        g.hands[oi] = (0..n_opp).map(|_| pool.pop().unwrap()).collect();
        if !self.talon.is_empty() {
            let n = self.talon.len();
            g.talon = (0..n).map(|_| pool.pop().unwrap()).collect();
        }
        if self.contract.is_some() && self.declarer != me && !self.discard.is_empty() {
            g.discard = (0..2).map(|_| pool.pop().unwrap()).collect();
            // cărțile aruncate de adversar contează la puncte (Ulti): se adaugă la „taken” doar prin pts în finish — le păstrăm în discard
        }
        g.sort_hands();
        g.rng = Rng::new(rng.next_u64());
        g
    }

    fn rollout_done(&self) -> bool {
        matches!(self.phase, Phase::HandOver | Phase::Over)
    }

    fn rollout_value(&self, p: Player) -> f64 {
        (self.gain[p.idx()] - self.gain[p.other().idx()]) as f64
    }

    fn policy(&self, rng: &mut Rng) -> Move {
        let mv = self.legal_moves();
        match self.phase {
            Phase::Play => {
                let cards = self.playable_cards();
                Move::Play(self.pick_play(rng, &cards))
            }
            Phase::Discard => {
                // aruncă cele două cărți cele mai slabe (nu atu, puține puncte)
                let h = &self.hands[self.side.idx()];
                let mut sc: Vec<(i64, Card)> = h
                    .iter()
                    .map(|&c| {
                        (
                            (if self.contract.is_some() && self.is_trump(c) {
                                1000
                            } else {
                                0
                            }) + self.power(c) as i64 * 5
                                + self.points(c) as i64,
                            c,
                        )
                    })
                    .collect();
                sc.sort_by_key(|x| x.0);
                Move::Discard(sc[0].1, sc[1].1)
            }
            Phase::Bid => {
                if rng.below(100) < 55 {
                    Move::Pass
                } else {
                    mv[rng.below(mv.len())]
                }
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

    fn mk(kind: Kind) -> Trick32 {
        Trick32::with_seed(kind, 1)
    }

    #[test]
    fn deal_sizes() {
        let g = mk(Kind::Schafkopf);
        assert_eq!(
            (g.hands[0].len(), g.hands[1].len(), g.talon.len()),
            (8, 8, 0)
        );
        let g = mk(Kind::Preferans);
        assert_eq!(
            (g.hands[0].len(), g.hands[1].len(), g.talon.len()),
            (10, 10, 2)
        );
        let g = mk(Kind::Ulti);
        assert_eq!(
            (g.hands[0].len(), g.hands[1].len(), g.talon.len()),
            (10, 10, 2)
        );
    }

    #[test]
    fn schafkopf_trump_order() {
        let mut g = mk(Kind::Schafkopf);
        g.contract = Some(contracts(Kind::Schafkopf)[1]); // solo-d
        assert_eq!(g.contract.unwrap().trump, Some(1));
        // Ober ♣ bate Ober ♠, Ober bate Unter, Unter bate Aș de atu, atu bate Aș de altă culoare
        let ober_c = c(12, 3);
        let ober_s = c(12, 2);
        let unter_d = c(11, 1);
        let ace_d = c(14, 1);
        let ace_h = c(14, 0);
        assert!(g.power(ober_c) > g.power(ober_s));
        assert!(g.power(ober_s) > g.power(unter_d));
        assert!(g.power(unter_d) > g.power(ace_d));
        assert!(g.is_trump(ace_d) && !g.is_trump(ace_h));
        g.trick = vec![(Player::One, ace_h), (Player::Two, c(7, 1))];
        assert_eq!(g.trick_winner(), Player::Two); // 7 de atu bate Aș de altă culoare
        g.trick = vec![(Player::One, ace_h), (Player::Two, c(10, 0))];
        assert_eq!(g.trick_winner(), Player::One);
        // wenz: doar Unterii sunt atu
        g.contract = Some(contracts(Kind::Schafkopf)[0]);
        assert!(!g.is_trump(ober_c) && g.is_trump(unter_d));
    }

    #[test]
    fn preferans_ladder_and_follow() {
        let cs = contracts(Kind::Preferans);
        assert_eq!(cs.len(), 25);
        assert!(cs.windows(2).all(|w| w[0].rank < w[1].rank));
        let mut g = mk(Kind::Preferans);
        g.contract = Some(cs[3]); // 6h
        g.phase = Phase::Play;
        g.side = Player::Two;
        g.hands[1] = vec![c(9, 0), c(10, 1), c(12, 2)];
        g.trick = vec![(Player::One, c(14, 2))];
        // trebuie să răspundă la pică
        assert_eq!(g.playable_cards(), vec![c(12, 2)]);
        g.trick = vec![(Player::One, c(14, 3))];
        // fără treflă: trebuie atu (cupă=0 dacă 6h)
        assert_eq!(g.playable_cards(), vec![c(9, 0)]);
    }

    #[test]
    fn bidding_flow_and_redeal() {
        let mut g = mk(Kind::Ulti);
        assert_eq!(g.side, Player::Two); // cel care nu a dat licitează primul
        g.apply(&Move::Pass);
        g.apply(&Move::Pass);
        assert_eq!(g.phase, Phase::HandOver);
        g.apply(&Move::Next);
        assert_eq!(g.phase, Phase::Bid); // împărțit din nou
        assert_eq!(g.hand_no, 1);
        let first = g.side;
        let cs = contracts(Kind::Ulti);
        g.apply(&Move::Bid(cs[0]));
        let mv = g.legal_moves();
        assert!(!mv.contains(&Move::Bid(cs[0])));
        assert!(mv.contains(&Move::Pass));
        g.apply(&Move::Pass);
        assert_eq!(g.phase, Phase::Take);
        assert_eq!(g.declarer, first);
        g.apply(&Move::Take);
        assert_eq!(g.hands[first.idx()].len(), 12);
        let m = g.legal_moves();
        assert_eq!(m.len(), 66);
        g.apply(&m[0]);
        assert_eq!(g.hands[first.idx()].len(), 10);
        assert_eq!(g.phase, Phase::Play);
        assert_eq!(g.side, first);
    }

    #[test]
    fn schafkopf_scoring() {
        let mut g = mk(Kind::Schafkopf);
        g.contract = Some(contracts(Kind::Schafkopf)[2]);
        g.declarer = Player::One;
        g.taken[0] = vec![
            c(14, 1),
            c(10, 1),
            c(14, 0),
            c(10, 0),
            c(14, 2),
            c(10, 2),
            c(12, 3),
            c(12, 2),
        ];
        g.taken[1] = vec![c(9, 3)];
        g.tricks_won = [4, 1];
        g.finish_hand();
        // 11+10+11+10+11+10+3+3 = 69 puncte: reușit, fără schneider (<90) ⇒ +3
        assert_eq!(g.gain, [3, -3]);
    }

    #[test]
    fn random_games_end() {
        for (kind, n) in [(Kind::Schafkopf, 0), (Kind::Preferans, 1), (Kind::Ulti, 2)] {
            for seed in 1..=6u64 {
                let mut g = Trick32::with_seed(kind, seed);
                let mut rng = Rng::new(seed + n);
                let mut k = 0;
                while g.outcome().is_none() && k < 6000 {
                    let mv = g.legal_moves();
                    assert!(!mv.is_empty());
                    g.apply(&mv[rng.below(mv.len())]);
                    k += 1;
                }
                assert!(g.outcome().is_some(), "{kind:?} seed {seed}");
            }
        }
    }

    #[test]
    fn determinize_keeps_own_hand() {
        let g = mk(Kind::Preferans);
        let mut rng = Rng::new(3);
        let d = g.determinize(Player::One, &mut rng);
        assert_eq!(d.hands[0], g.hands[0]);
        assert_eq!(d.hands[1].len(), 10);
        assert_ne!(d.hands[1], g.hands[1]);
    }
}
