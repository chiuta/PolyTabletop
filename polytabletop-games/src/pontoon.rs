//! Pontoon (Australia / Marea Britanie), banca împotriva unui jucător; cei doi jucători
//! schimbă rolurile la fiecare mână.
//!
//! Reguli folosite (publice, varianta standard): un pachet de 52 (amestecat la fiecare
//! mână); as = 1 sau 11, figurile și 10 = 10. Ierarhia: **Pontoon** (as + carte de 10
//! puncte din primele două) > **Five Card Trick** (cinci cărți, total ≤ 21) > 21 din
//! trei/patru cărți > total ≤ 21 cu cât mai aproape de 21 > pierdut (peste 21). La
//! egalitate **câștigă banca**. Fiecare primește două cărți **cu fața în jos**.
//!
//! * Un Pontoon al băncii încasează dublul mizei; un Pontoon al jucătorului (banca nu are)
//!   se plătește dublu și nu se mai joacă mâna.
//! * Jucătorul: `carte` (twist: carte cu fața în sus, miza neschimbată), `cumpar` (buy: carte
//!   cu fața în jos, miza crește cu miza inițială — permis doar înainte de orice `carte`,
//!   max. 3 cumpărări), `stai` (stick: doar cu total ≥ 15). Peste 21 = pierde miza,
//!   banca nu mai joacă. Cinci cărți ≤ 21 = Five Card Trick.
//! * Banca își întoarce cărțile și poate trage `carte` până alege să rămână (`stai`),
//!   dar **poate rămâne doar de la 17 în sus** (regula obișnuită). Banca peste 21: plătește
//!   miza jucătorilor rămași (Pontoon/Five Card Trick dublu). Banca la ≤21 cu ≤4 cărți: plătește
//!   cine are mai mult, restul pierd miza; Pontoon/Five Card Trick ale jucătorului se plătesc
//!   dublu. Five Card Trick al băncii: pierd dublul mizei toți în afară de Pontoon.
//!
//! Alegeri ale noastre: meci de 20 de mâini (miza 10, start 100 de jetoane fiecare); rolul
//! de bancă alternează (în regulile clasice trece la cine face Pontoon), J2 e banca în
//! prima mână; fără `split` și fără `shoot`; cumpărarea costă mereu miza inițială
//! (o valoare permisă între 1× și 2×). Câștigă cine are mai multe jetoane după 20 de mâini.
//!
//! Informație ascunsă: cărțile cu fața în jos (cele două inițiale și cele cumpărate) sunt
//! private; cărțile trase cu `carte` sunt publice; banca își dezvăluie cărțile la începutul
//! turei ei. `evaluate` citește doar propriile cărți și ce e vizibil; cartea tocmai trasă
//! de jucătorul evaluat e înlocuită cu speranța ei (nu se „trișează” pe pachet).

use crate::blackjack::{hand_value, is_blackjack};
use crate::cards::Viewer;
use crate::cards52::{join_symbols, std_deck, std_symbol};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const HANDS: u8 = 20;
pub const BASE: i32 = 10;
pub const START: i32 = 100;
const MAX_BUYS: u8 = 3;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Punter,
    Banker,
    Settle,
    Over,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Last {
    None,
    Twist,
    Buy,
    Stick,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Twist,
    Buy,
    Stick,
    Settle,
}

#[derive(Clone)]
pub struct Pontoon {
    rng: Rng,
    viewer: Viewer,
    deck: Vec<u8>,
    chips: [i32; 2],
    hand_no: u8,
    banker: Player,
    cards: [Vec<u8>; 2],
    up: [Vec<bool>; 2],
    stake: i32,
    buys: u8,
    twisted: bool,
    phase: Phase,
    last: Last,
    last_actor: Player,
    last_result: String,
}

impl Default for Pontoon {
    fn default() -> Self {
        Self::new()
    }
}

fn total(c: &[u8]) -> u8 {
    hand_value(c).0
}

/// Valoarea de clasament: (nivel, total). Nivel: 4 pontoon, 3 five-card, 2 = 21 din 3–4 cărți
/// sau mai puțin (compară totalul), 0 = pierdut.
fn rank(c: &[u8]) -> (u8, u8) {
    let t = total(c);
    if t > 21 {
        (0, 0)
    } else if is_blackjack(c) {
        (4, 21)
    } else if c.len() >= 5 {
        (3, t)
    } else {
        (2, t)
    }
}

/// Câștigul net al jucătorului (punter) în jetoane.
pub fn punter_net(punter: &[u8], banker: &[u8], stake: i32) -> i32 {
    let (pl, pt) = rank(punter);
    let (bl, bt) = rank(banker);
    if bl == 4 {
        return -2 * stake;
    }
    if pl == 4 {
        return 2 * stake;
    }
    if pl == 0 {
        return -stake;
    }
    if bl == 0 {
        return if pl == 3 { 2 * stake } else { stake };
    }
    if bl == 3 {
        return -2 * stake;
    }
    if pl == 3 {
        return 2 * stake;
    }
    if pt > bt {
        stake
    } else {
        -stake
    }
}

impl Pontoon {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Pontoon {
            rng: Rng::new(seed),
            viewer: Viewer::Fixed(Player::One),
            deck: Vec::new(),
            chips: [START; 2],
            hand_no: 1,
            banker: Player::Two,
            cards: [Vec::new(), Vec::new()],
            up: [Vec::new(), Vec::new()],
            stake: BASE,
            buys: 0,
            twisted: false,
            phase: Phase::Punter,
            last: Last::None,
            last_actor: Player::One,
            last_result: String::new(),
        };
        g.deal();
        g
    }

    pub fn chips(&self) -> [i32; 2] {
        self.chips
    }

    fn punter(&self) -> Player {
        self.banker.other()
    }

    fn deal(&mut self) {
        self.deck = std_deck(&mut self.rng);
        let p = self.punter().idx();
        let b = self.banker.idx();
        self.cards = [Vec::new(), Vec::new()];
        self.up = [Vec::new(), Vec::new()];
        for _ in 0..2 {
            for who in [p, b] {
                let c = self.deck.pop().unwrap();
                self.cards[who].push(c);
                self.up[who].push(false);
            }
        }
        self.stake = BASE;
        self.buys = 0;
        self.twisted = false;
        self.last = Last::None;
        self.phase = if is_blackjack(&self.cards[p]) || is_blackjack(&self.cards[b]) {
            Phase::Settle
        } else {
            Phase::Punter
        };
    }

    fn draw(&mut self, who: usize, face_up: bool) {
        let c = self.deck.pop().expect("deck");
        self.cards[who].push(c);
        self.up[who].push(face_up);
    }

    fn settle(&mut self) {
        let p = self.punter().idx();
        let b = self.banker.idx();
        let net = punter_net(&self.cards[p], &self.cards[b], self.stake);
        self.chips[p] += net;
        self.chips[b] -= net;
        self.last_result = format!(
            "Mâna {}: jucător J{} {} = {}, bancă J{} {} = {} → J{} {}{} jetoane.",
            self.hand_no,
            p + 1,
            join_symbols(&self.cards[p], std_symbol),
            total(&self.cards[p]),
            b + 1,
            join_symbols(&self.cards[b], std_symbol),
            total(&self.cards[b]),
            p + 1,
            if net > 0 { "+" } else { "" },
            net
        );
        if self.hand_no >= HANDS {
            self.phase = Phase::Over;
        } else {
            self.hand_no += 1;
            self.banker = self.banker.other();
            self.deal();
        }
    }

    fn punter_done(&mut self) {
        // pasul după ce jucătorul a terminat (stai / cinci cărți)
        self.phase = Phase::Banker;
    }

    fn all_public(&self) -> bool {
        matches!(self.phase, Phase::Settle | Phase::Over)
    }
}

// ---- model cu pachet infinit pentru evaluate ----------------------------------

fn pv(v: u8) -> f64 {
    if v == 10 {
        4.0 / 13.0
    } else {
        1.0 / 13.0
    }
}
fn tot(h: u8, ace: bool) -> u8 {
    if ace && h + 10 <= 21 {
        h + 10
    } else {
        h
    }
}

/// Distribuția totalului final al unei bănci care trage până la 17 (17..21, bust), medie pe prima carte.
fn banker_dist() -> [f64; 6] {
    let mut out = [0.0; 6];
    for up in 1..=10u8 {
        let d = crate::blackjack::dealer_distribution(if up == 1 { 11 } else { up });
        for (o, x) in out.iter_mut().zip(d.iter()) {
            *o += pv(up) * x;
        }
    }
    out
}

/// Distribuția totalului unui jucător care trage până la ≥ 15 (15..21 și bust), ca 8 valori.
fn punter_dist() -> [f64; 8] {
    fn rec(
        h: u8,
        ace: bool,
        memo: &mut std::collections::HashMap<(u8, bool), [f64; 8]>,
    ) -> [f64; 8] {
        let t = tot(h, ace);
        let mut out = [0.0; 8];
        if t > 21 {
            out[7] = 1.0;
            return out;
        }
        if t >= 15 {
            out[(t - 15) as usize] = 1.0;
            return out;
        }
        if let Some(v) = memo.get(&(h, ace)) {
            return *v;
        }
        for v in 1..=10u8 {
            let s = rec(h + v, ace || v == 1, memo);
            for (o, x) in out.iter_mut().zip(s.iter()) {
                *o += pv(v) * x;
            }
        }
        memo.insert((h, ace), out);
        out
    }
    let mut memo = std::collections::HashMap::new();
    let mut out = [0.0; 8];
    for first in 1..=10u8 {
        let s = rec(first, first == 1, &mut memo);
        for (o, x) in out.iter_mut().zip(s.iter()) {
            *o += pv(first) * x;
        }
    }
    out
}

/// Speranța (în mize) a jucătorului care rămâne cu totalul `t` (egalitatea o pierde).
fn punter_stand(t: u8, bd: &[f64; 6]) -> f64 {
    if t > 21 {
        return -1.0;
    }
    let mut ev = bd[5]; // banca peste 21
    for (i, &p) in bd.iter().enumerate().take(5) {
        let b = 17 + i as u8;
        if b < t {
            ev += p;
        } else {
            ev -= p;
        }
    }
    ev
}

fn punter_opt(
    h: u8,
    ace: bool,
    bd: &[f64; 6],
    memo: &mut std::collections::HashMap<(u8, bool), f64>,
) -> f64 {
    let t = tot(h, ace);
    if t > 21 {
        return -1.0;
    }
    if let Some(v) = memo.get(&(h, ace)) {
        return *v;
    }
    let stand = punter_stand(t, bd);
    let r = if t >= 21 {
        stand
    } else if t < 15 {
        // obligatoriu carte
        (1..=10u8)
            .map(|v| pv(v) * punter_opt(h + v, ace || v == 1, bd, memo))
            .sum()
    } else {
        let twist: f64 = (1..=10u8)
            .map(|v| pv(v) * punter_opt(h + v, ace || v == 1, bd, memo))
            .sum();
        stand.max(twist)
    };
    memo.insert((h, ace), r);
    r
}

/// Speranța băncii care rămâne la `t` contra jucătorilor care au rămas (egalitate = banca).
fn banker_stand(t: u8, pd: &[f64; 8]) -> f64 {
    if t > 21 {
        return -1.0;
    }
    let live: f64 = pd[..7].iter().sum();
    if live <= 0.0 {
        return 0.0;
    }
    let mut ev = 0.0;
    for (i, &p) in pd.iter().enumerate().take(7) {
        let u = 15 + i as u8;
        ev += if t >= u { p } else { -p };
    }
    ev / live
}

fn banker_opt(
    h: u8,
    ace: bool,
    pd: &[f64; 8],
    memo: &mut std::collections::HashMap<(u8, bool), f64>,
) -> f64 {
    let t = tot(h, ace);
    if t > 21 {
        return -1.0;
    }
    if let Some(v) = memo.get(&(h, ace)) {
        return *v;
    }
    let twist: f64 = (1..=10u8)
        .map(|v| pv(v) * banker_opt(h + v, ace || v == 1, pd, memo))
        .sum();
    let r = if t >= 17 {
        banker_stand(t, pd).max(twist)
    } else {
        twist
    };
    memo.insert((h, ace), r);
    r
}

fn hard(cards: &[u8]) -> (u8, bool) {
    let mut h = 0u32;
    let mut ace = false;
    for &c in cards {
        match c % 13 {
            12 => {
                h += 1;
                ace = true;
            }
            r @ 0..=8 => h += r as u32 + 2,
            _ => h += 10,
        }
    }
    (h.min(40) as u8, ace)
}

impl Game for Pontoon {
    type Move = Move;

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = match who {
            Some(p) => Viewer::Fixed(p),
            None => Viewer::HotSeat,
        };
    }

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Punter => self.punter(),
            _ => self.banker,
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        match self.phase {
            Phase::Over => Vec::new(),
            Phase::Settle => vec![Move::Settle],
            Phase::Punter => {
                let c = &self.cards[self.punter().idx()];
                let t = total(c);
                let mut v = Vec::new();
                if t < 21 && c.len() < 5 {
                    v.push(Move::Twist);
                    if !self.twisted && self.buys < MAX_BUYS {
                        v.push(Move::Buy);
                    }
                }
                if t >= 15 {
                    v.push(Move::Stick);
                }
                v
            }
            Phase::Banker => {
                let c = &self.cards[self.banker.idx()];
                let t = total(c);
                let mut v = Vec::new();
                if t < 21 && c.len() < 5 {
                    v.push(Move::Twist);
                }
                if t >= 17 {
                    v.push(Move::Stick);
                }
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        let p = self.punter().idx();
        let b = self.banker.idx();
        if *m != Move::Settle {
            self.last_actor = self.to_move();
        }
        match (self.phase, m) {
            (_, Move::Settle) => self.settle(),
            (Phase::Punter, Move::Twist) => {
                self.draw(p, true);
                self.twisted = true;
                self.last = Last::Twist;
                self.after_punter_draw();
            }
            (Phase::Punter, Move::Buy) => {
                self.draw(p, false);
                self.stake += BASE;
                self.buys += 1;
                self.last = Last::Buy;
                self.after_punter_draw();
            }
            (Phase::Punter, Move::Stick) => {
                self.last = Last::Stick;
                self.punter_done();
            }
            (Phase::Banker, Move::Twist) => {
                self.draw(b, true);
                self.last = Last::Twist;
                let c = &self.cards[b];
                if total(c) > 21 || c.len() >= 5 {
                    self.phase = Phase::Settle;
                }
            }
            (Phase::Banker, Move::Stick) => {
                self.last = Last::Stick;
                self.phase = Phase::Settle;
            }
            _ => {}
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        Some(match self.chips[0].cmp(&self.chips[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut v = (self.chips[me] - self.chips[1 - me]) * 100;
        if self.phase != Phase::Over {
            v += (self.own_ev(p) * self.stake as f64 * 100.0).round() as i32;
        }
        v
    }

    fn render(&self) -> String {
        let who = self.viewer.who(self.to_move());
        let mut s = format!(
            "Pontoon — mâna {}/{HANDS}, miza {} (inițială {BASE}). Jetoane: J1 {}, J2 {} (start {START}).\n",
            self.hand_no.min(HANDS),
            self.stake,
            self.chips[0],
            self.chips[1]
        );
        s.push_str(&format!(
            "Banca: Jucător {}; jucătorul: Jucător {}.\n",
            self.banker.idx() + 1,
            self.punter().idx() + 1
        ));
        for (i, role) in [
            (self.punter().idx(), "Jucător"),
            (self.banker.idx(), "Bancă"),
        ] {
            let c = &self.cards[i];
            let reveal = self.all_public()
                || i == who.idx()
                || (i == self.banker.idx() && self.phase == Phase::Banker);
            if reveal {
                s.push_str(&format!(
                    "{role} J{}: {} = {}\n",
                    i + 1,
                    join_symbols(c, std_symbol),
                    total(c)
                ));
            } else {
                let shown: Vec<String> = c
                    .iter()
                    .zip(&self.up[i])
                    .map(|(&x, &u)| if u { std_symbol(x) } else { "[??]".to_string() })
                    .collect();
                s.push_str(&format!("{role} J{}: {}\n", i + 1, shown.join(" ")));
            }
        }
        if !self.last_result.is_empty() {
            s.push_str(&format!("Mâna anterioară — {}\n", self.last_result));
        }
        match self.phase {
            Phase::Punter => s.push_str(&format!(
                "La mutare: jucătorul J{} — carte / cumpar / stai (stai doar de la 15)\n",
                self.punter().idx() + 1
            )),
            Phase::Banker => s.push_str(&format!(
                "La mutare: banca J{} — carte / stai (stai doar de la 17)\n",
                self.banker.idx() + 1
            )),
            Phase::Settle => s.push_str("Mâna s-a încheiat: decontare\n"),
            Phase::Over => match self.outcome() {
                Some(Outcome::Win(p)) => s.push_str(&format!(
                    "Meci încheiat: Jucător {} câștigă.\n",
                    p.idx() + 1
                )),
                _ => s.push_str("Meci încheiat: remiză.\n"),
            },
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Twist => "carte",
            Move::Buy => "cumpar",
            Move::Stick => "stai",
            Move::Settle => "decontare",
        }
        .into()
    }

    /// Adâncime 0 (o singură mutare): `AlphaBeta(1)` ar fi extins la Hard/Expert cărțile reale din pachet și
    /// răspunsul adversarului; `evaluate` citește doar cărțile proprii și un model cu pachet infinit.
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

impl Pontoon {
    /// Speranța (în mize) a lui `p` pentru mâna curentă, din ce știe `p`: cărțile proprii,
    /// fără cartea tocmai trasă (înlocuită de speranța ei) și un model cu pachet infinit.
    fn own_ev(&self, p: Player) -> f64 {
        use std::collections::HashMap;
        let bd = banker_dist();
        let pd = punter_dist();
        let drew = self.last_actor == p && matches!(self.last, Last::Twist | Last::Buy);
        let cards = &self.cards[p.idx()];
        let known: &[u8] = if drew {
            &cards[..cards.len() - 1]
        } else {
            cards
        };
        let (h, ace) = hard(known);
        let mut pm: HashMap<(u8, bool), f64> = HashMap::new();
        if p == self.punter() {
            if drew {
                (1..=10u8)
                    .map(|x| pv(x) * punter_opt(h + x, ace || x == 1, &bd, &mut pm))
                    .sum()
            } else if self.phase == Phase::Punter {
                punter_opt(h, ace, &bd, &mut pm)
            } else {
                punter_stand(tot(h, ace), &bd)
            }
        } else if drew {
            (1..=10u8)
                .map(|x| pv(x) * banker_opt(h + x, ace || x == 1, &pd, &mut pm))
                .sum()
        } else if self.phase == Phase::Settle || self.last == Last::Stick {
            banker_stand(tot(h, ace), &pd)
        } else {
            banker_opt(h, ace, &pd, &mut pm)
        }
    }

    fn after_punter_draw(&mut self) {
        let p = self.punter().idx();
        let c = &self.cards[p];
        if total(c) > 21 {
            self.phase = Phase::Settle;
        } else if c.len() >= 5 {
            self.punter_done();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ACE: u8 = 12;
    const KING: u8 = 11;
    const TEN: u8 = 8;
    const NINE: u8 = 7;

    #[test]
    fn ranking_and_payouts() {
        // Pontoon al jucătorului (banca fără): +2x
        assert_eq!(punter_net(&[ACE, KING], &[TEN, NINE], 10), 20);
        // Pontoon al băncii: -2x, chiar și contra Pontoon
        assert_eq!(punter_net(&[TEN, 9], &[ACE, TEN], 10), -20);
        assert_eq!(punter_net(&[ACE, TEN], &[ACE, KING], 10), -20);
        // egalitate: banca
        assert_eq!(punter_net(&[TEN, NINE], &[KING, NINE], 10), -10);
        // jucător mai mare
        assert_eq!(punter_net(&[TEN, KING], &[TEN, NINE], 10), 10);
        // jucător peste 21
        assert_eq!(punter_net(&[TEN, KING, KING], &[TEN, 0], 10), -10);
        // banca peste 21
        assert_eq!(punter_net(&[TEN, 5], &[TEN, KING, KING], 10), 10);
        // Five Card Trick al jucătorului (total 18) bate 21 din trei cărți: dublu
        assert_eq!(punter_net(&[0, 0, 0, 0, 10], &[TEN, 5, 3], 10), 20); // 2+2+2+2+Q = 18
                                                                         // Five Card Trick al băncii: pierde dublu
        assert_eq!(punter_net(&[TEN, KING], &[0, 0, 0, 0, 10], 10), -20);
    }

    fn fixed(punter: &[u8], banker: &[u8], deck: &[u8]) -> Pontoon {
        let mut g = Pontoon::with_seed(1);
        g.banker = Player::Two;
        g.cards = [punter.to_vec(), banker.to_vec()];
        g.up = [vec![false; punter.len()], vec![false; banker.len()]];
        g.deck = deck.to_vec();
        g.phase = Phase::Punter;
        g.stake = BASE;
        g.buys = 0;
        g.twisted = false;
        g.last = Last::None;
        g.last_actor = Player::One;
        g.chips = [START; 2];
        g.hand_no = 1;
        g
    }

    #[test]
    fn stick_needs_15_and_banker_17() {
        let g = fixed(&[TEN, 4], &[TEN, 5], &[]); // 10 + 6 = 16
        assert!(g.legal_moves().contains(&Move::Stick));
        let g = fixed(&[TEN, 3], &[TEN, 5], &[]); // 10+5 = 15
        assert!(g.legal_moves().contains(&Move::Stick));
        let g = fixed(&[TEN, 2], &[TEN, 5], &[]); // 13
        assert!(!g.legal_moves().contains(&Move::Stick));
        let mut g = fixed(&[TEN, NINE], &[TEN, 4], &[]); // banca 16
        g.apply(&Move::Stick);
        assert_eq!(g.to_move(), Player::Two);
        assert_eq!(g.legal_moves(), vec![Move::Twist]);
        let mut g = fixed(&[TEN, NINE], &[TEN, 5], &[]); // banca 17
        g.apply(&Move::Stick);
        assert!(g.legal_moves().contains(&Move::Stick));
    }

    #[test]
    fn buy_raises_stake_only_before_twist() {
        let mut g = fixed(&[TEN, 3], &[TEN, 5], &[0, 1, 2]);
        assert!(g.legal_moves().contains(&Move::Buy));
        g.apply(&Move::Buy);
        assert_eq!(g.stake, 2 * BASE);
        assert!(!g.up[0][2]); // cumpărată cu fața în jos
        g.apply(&Move::Twist);
        assert!(g.up[0][3]);
        assert!(!g.legal_moves().contains(&Move::Buy));
    }

    #[test]
    fn bust_settles_without_the_bank_playing_and_five_card_trick_ends_draws() {
        let mut g = fixed(&[TEN, 5], &[TEN, 5], &[TEN]); // 10+7+... pune K (11)
        g.deck = vec![KING];
        g.apply(&Move::Twist); // 10 + 7 + 10 = 27
        assert_eq!(g.phase, Phase::Settle);
        g.apply(&Move::Settle);
        assert_eq!(g.chips, [90, 110]);
        let mut h = fixed(&[0, 0], &[TEN, 5], &[0, 0, 1]); // 2+2 + ...
        h.apply(&Move::Twist);
        h.apply(&Move::Twist);
        h.apply(&Move::Twist);
        assert_eq!(h.cards[0].len(), 5);
        assert_eq!(h.phase, Phase::Banker);
    }

    #[test]
    fn render_hides_facedown_cards_of_the_other_side() {
        let mut g = fixed(&[TEN, 3], &[KING, 5], &[]);
        g.set_viewer(Some(Player::One));
        let r = Game::render(&g);
        assert!(r.contains("[??]"));
        assert!(!r.contains(&std_symbol(KING)));
        g.set_viewer(Some(Player::Two));
        let r2 = Game::render(&g);
        assert!(r2.contains(&std_symbol(KING)));
    }

    #[test]
    fn evaluate_ignores_the_card_just_drawn() {
        let g = fixed(&[TEN, 2], &[TEN, 5], &[0, KING]);
        let mut a = g.clone();
        a.apply(&Move::Twist); // trage KING? pop = KING (ultimul)
        let mut b = g.clone();
        b.deck = vec![KING, 0];
        b.apply(&Move::Twist); // trage 2
        assert_eq!(a.evaluate(Player::One), b.evaluate(Player::One));
    }

    #[test]
    fn random_matches_finish_and_chips_conserved() {
        for seed in 1..=30u64 {
            let mut g = Pontoon::with_seed(seed);
            let mut rng = Rng::new(seed * 3);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                assert_eq!(g.chips[0] + g.chips[1], 2 * START);
                n += 1;
                assert!(n < 2000);
            }
        }
    }

    #[test]
    fn ai_beats_random_on_average() {
        use polytabletop_core::GameObj;
        let (mut ai, mut rn) = (0, 0);
        for seed in 1..=20u64 {
            let mut g: Box<dyn GameObj> = Box::new(Pontoon::with_seed(seed));
            let mut rng = Rng::new(seed);
            while g.outcome().is_none() {
                let mv = if g.to_move() == Player::One {
                    g.ai_move(rng.next_u64()).unwrap()
                } else {
                    let m = g.legal_moves();
                    m[rng.below(m.len())].clone()
                };
                g.play(&mv).unwrap();
            }
            match g.outcome() {
                Some(Outcome::Win(Player::One)) => ai += 1,
                Some(Outcome::Win(Player::Two)) => rn += 1,
                _ => {}
            }
            let _ = &mut ai;
        }
        assert!(ai > rn, "ai {ai} vs random {rn}");
    }
}

#[cfg(test)]
mod a2_audit {
    use super::*;

    #[test]
    fn ai_nu_vede_cartile_cu_fata_in_jos_nici_pachetul_la_orice_nivel() {
        crate::p3_comun::ai_blind_check(
            "pontoon",
            Pontoon::with_seed,
            |g, me, seed| {
                let o = me.other().idx();
                let public = matches!(g.phase, Phase::Settle | Phase::Over)
                    || (o == g.banker.idx() && g.phase == Phase::Banker);
                let hidden: Vec<usize> = if public {
                    Vec::new()
                } else {
                    (0..g.cards[o].len()).filter(|&i| !g.up[o][i]).collect()
                };
                let mut pool: Vec<u8> = g.deck.clone();
                pool.extend(hidden.iter().map(|&i| g.cards[o][i]));
                crate::p3_comun::shuffle_with(&mut pool, seed);
                for &i in &hidden {
                    g.cards[o][i] = pool.pop().unwrap();
                }
                g.deck = pool;
            },
            12,
        );
    }
}
