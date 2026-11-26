//! Five Hundred (500), varianta pentru **doi jucători** (Australia / SUA), din regulile publice.
//!
//! * Pachet de 43 de cărți: culorile roșii (♦ ♥) A K Q J 10 9 8 7 6 5 4, cele negre (♠ ♣) A…5, plus jokerul.
//!   Culori: `s` pică, `c` treflă, `d` caro, `h` cupă; jokerul `Jk`. Fiecare jucător primește 10 cărți
//!   în mână și **cinci grămezi de două cărți** pe masă (una cu fața în jos, una peste ea cu fața în sus); 3 cărți
//!   formează *kitty*-ul (43 = 20 + 20 + 3). Cărțile cu fața în sus sunt publice.
//! * **Licitație**: începe jucătorul care nu a împărțit; un jucător licitează un număr de levate (6–10) și un
//!   atu (`licit 7h`, `licit 8nt` fără atu) sau trece (`pas`). Fiecare ofertă trebuie să fie mai mare decât
//!   precedenta; ordinea culorilor: ♠ < ♣ < ♦ < ♥ < fără atu. Cine a trecut nu mai licitează. Dacă amândoi trec,
//!   se redă mâna. **Misere** nu se joacă (în varianta pentru doi jucători e rar și unele reguli îl exclud;
//!   nici open misere). Valorile contractelor: 6♠ 40, 6♣ 60, 6♦ 80, 6♥ 100, 6 NT 120; apoi +100 pe levată în plus
//!   (7♠ 140 … 10 NT 520).
//! * Licitatorul (contractant) ia kitty-ul și **aruncă trei cărți** (`descarta As`) din cele 13.
//! * **Levată** din patru cărți: câte o carte din mâna fiecăruia și câte una dintre cărțile cu fața în sus. Contractantul
//!   conduce prima levată din mână; adversarul răspunde din mână; apoi contractantul joacă o carte cu fața în sus
//!   și adversarul una cu fața în sus. Câștigătorul unei levate conduce următoarea: dacă a câștigat cu o carte cu fața
//!   în sus, conduce cu o carte cu fața în sus (și răspunde tot cu fața în sus), apoi fiecare joacă din mână; dacă a
//!   câștigat cu o carte din mână, conduce din mână și apoi fiecare din cărțile cu fața în sus. La sfârșitul levatei
//!   cărțile cu fața în jos de sub cele jucate se întorc. Obligatoriu se răspunde la culoare (din cărțile disponibile
//!   în acel pas); câștigă cel mai mare atu, altfel cea mai mare carte din culoarea cerută.
//! * **Atu**: jokerul e cel mai mare atu, urmat de valetul de atu (right bower), de valetul de aceeași
//!   culoare (left bower, face parte din atu) și apoi A K Q 10 9 … din atu. **Fără atu**: valeții sunt la rangul lor;
//!   jokerul nu are culoare și e cea mai mare carte: se joacă doar când nu ai culoarea cerută, sau la deschidere
//!   numind o culoare (`Jk:h`) care nu a mai fost cerută în mâna asta (la ultima levată se poate deschide oricând).
//! * **Scor**: contractul reușit (cel puțin atâtea levate) valorează valoarea lui; dacă a luat toate cele 10 levate
//!   (slem) și valoarea e sub 250, primește 250; contractul eșuat scade valoarea. Adversarul primește **10 puncte
//!   pe levată** luată, oricum. Câștigi la **500** doar printr-un contract reușit; pierzi la −500.
//!
//! Simplificări documentate: fără misere/open misere, fără alegerea suitei de către contractant pentru joker înainte
//! de prima carte (se folosește regula „joker fără culoare”). Informație ascunsă: mâna adversarului, cărțile cu fața
//! în jos, kitty; `evaluate` citește doar mâna proprie, cărțile cu fața în sus și levata în curs.

use crate::cards::Viewer;
use crate::cards52::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const JOKER: u8 = 0xFF;
const SUIT_CH: [char; 4] = ['s', 'c', 'd', 'h'];
const NT: u8 = 4;

fn mk(suit: u8, rank: u8) -> u8 {
    (suit << 4) | rank
}
fn suit_of(c: u8) -> u8 {
    c >> 4
}
fn rank_of(c: u8) -> u8 {
    c & 15
}
fn rank_label(r: u8) -> String {
    match r {
        14 => "A".into(),
        13 => "K".into(),
        12 => "Q".into(),
        11 => "J".into(),
        n => n.to_string(),
    }
}
pub fn text(c: u8) -> String {
    if c == JOKER {
        "Jk".into()
    } else {
        format!("{}{}", rank_label(rank_of(c)), SUIT_CH[suit_of(c) as usize])
    }
}
fn join(cards: &[u8]) -> String {
    if cards.is_empty() {
        return "—".into();
    }
    cards.iter().map(|&c| text(c)).collect::<Vec<_>>().join(" ")
}

fn full_deck() -> Vec<u8> {
    let mut d = Vec::new();
    for s in 0..4u8 {
        let lo = if s < 2 { 5 } else { 4 };
        for r in lo..=14u8 {
            d.push(mk(s, r));
        }
    }
    d.push(JOKER);
    d
}

/// Culoarea efectivă a unei cărți (valetul „left” aparține atuului; jokerul e atu; în NT jokerul nu are culoare).
fn eff_suit(c: u8, trump: u8) -> Option<u8> {
    if c == JOKER {
        return if trump < 4 { Some(trump) } else { None };
    }
    if trump < 4 && rank_of(c) == 11 && suit_of(c) == (trump ^ 1) {
        return Some(trump);
    }
    Some(suit_of(c))
}

/// Puterea în cadrul culorii efective.
fn power(c: u8, trump: u8) -> u8 {
    if c == JOKER {
        return 100;
    }
    if trump < 4 && rank_of(c) == 11 {
        if suit_of(c) == trump {
            return 99;
        }
        if suit_of(c) == (trump ^ 1) {
            return 98;
        }
    }
    rank_of(c)
}

pub fn bid_value(tricks: u8, suit: u8) -> i32 {
    (tricks as i32 - 6) * 100 + 40 + 20 * suit as i32
}

#[derive(Clone, PartialEq, Debug)]
pub enum Move {
    Bid(u8, u8),
    Pass,
    Discard(u8),
    /// Carte jucată; `Option<u8>` = culoarea numită la deschiderea cu joker în NT.
    Play(u8, Option<u8>),
    Next,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Bid,
    Kitty,
    Play,
    HandOver,
}

#[derive(Clone)]
pub struct FiveHundred {
    rng: Rng,
    viewer: Viewer,
    hands: [Vec<u8>; 2],
    up: [[Option<u8>; 5]; 2],
    down: [[Option<u8>; 5]; 2],
    kitty: Vec<u8>,
    dealer: Player,
    side: Player,
    phase: Phase,
    bid: Option<(u8, u8, Player)>,
    passed: [bool; 2],
    contractor: Player,
    trump: u8,
    tricks: [u8; 2],
    trick: Vec<(Player, u8, bool)>,
    leader: Player,
    lead_up: bool,
    nominated: Option<u8>,
    led: [bool; 4],
    score: [i32; 2],
    last_card: Option<(Player, u8)>,
    log: String,
    winner: Option<Player>,
}

impl Default for FiveHundred {
    fn default() -> Self {
        Self::new()
    }
}

impl FiveHundred {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = FiveHundred {
            rng: Rng::new(seed),
            viewer: Viewer::Fixed(Player::One),
            hands: [Vec::new(), Vec::new()],
            up: [[None; 5]; 2],
            down: [[None; 5]; 2],
            kitty: Vec::new(),
            dealer: Player::Two,
            side: Player::One,
            phase: Phase::Bid,
            bid: None,
            passed: [false; 2],
            contractor: Player::One,
            trump: NT,
            tricks: [0; 2],
            trick: Vec::new(),
            leader: Player::One,
            lead_up: false,
            nominated: None,
            led: [false; 4],
            score: [0; 2],
            last_card: None,
            log: String::new(),
            winner: None,
        };
        g.deal();
        g
    }

    pub fn scores(&self) -> (i32, i32) {
        (self.score[0], self.score[1])
    }

    fn deal(&mut self) {
        let mut d = full_deck();
        shuffle(&mut d, &mut self.rng);
        for p in 0..2 {
            self.hands[p] = d.drain(..10).collect();
            for k in 0..5 {
                self.up[p][k] = d.pop();
                self.down[p][k] = d.pop();
            }
        }
        self.kitty = d;
        self.sort_hands();
        self.side = self.dealer.other();
        self.phase = Phase::Bid;
        self.bid = None;
        self.passed = [false; 2];
        self.tricks = [0; 2];
        self.trick.clear();
        self.nominated = None;
        self.led = [false; 4];
        self.last_card = None;
    }

    fn sort_hands(&mut self) {
        for h in self.hands.iter_mut() {
            h.sort_unstable();
        }
    }

    fn pool(&self, p: Player, up: bool) -> Vec<u8> {
        if up {
            self.up[p.idx()].iter().flatten().copied().collect()
        } else {
            self.hands[p.idx()].clone()
        }
    }

    fn step_is_up(&self) -> bool {
        // pașii 0 și 1: sursa de început; pașii 2 și 3: cealaltă sursă
        if self.trick.len() < 2 {
            self.lead_up
        } else {
            !self.lead_up
        }
    }

    fn led_suit(&self) -> Option<u8> {
        if let Some(n) = self.nominated {
            return Some(n);
        }
        let (_, c, _) = self.trick.first()?;
        eff_suit(*c, self.trump)
    }

    fn trick_winner(&self) -> (usize, Player) {
        let led = self.led_suit();
        let mut best = 0usize;
        let beats = |a: u8, b: u8| -> bool {
            // `a` bate `b`?
            let (sa, sb) = (eff_suit(a, self.trump), eff_suit(b, self.trump));
            if self.trump == NT {
                if a == JOKER {
                    return b != JOKER;
                }
                if b == JOKER {
                    return false;
                }
            }
            let ta = sa == Some(self.trump) && self.trump < 4;
            let tb = sb == Some(self.trump) && self.trump < 4;
            if ta != tb {
                return ta;
            }
            if sa == sb {
                return power(a, self.trump) > power(b, self.trump);
            }
            // culori diferite: doar cea cerută contează
            sa == led && sb != led
        };
        for i in 1..self.trick.len() {
            if beats(self.trick[i].1, self.trick[best].1) {
                best = i;
            }
        }
        (best, self.trick[best].0)
    }

    fn plays_for(&self, p: Player) -> Vec<Move> {
        let pool = self.pool(p, self.step_is_up());
        let mut out = Vec::new();
        if self.trick.is_empty() {
            let last_trick = self.tricks[0] + self.tricks[1] == 9;
            for &c in &pool {
                if c == JOKER && self.trump == NT {
                    for s in 0..4u8 {
                        if last_trick || !self.led[s as usize] {
                            out.push(Move::Play(c, Some(s)));
                        }
                    }
                    if last_trick && out.is_empty() {
                        out.push(Move::Play(c, Some(0)));
                    }
                    // nu se poate deschide cu jokerul dacă au fost cerute toate culorile (în afara ultimei levate)
                } else {
                    out.push(Move::Play(c, None));
                }
            }
            if out.is_empty() {
                // cazul în care jokerul e singura carte și nu se poate deschide: îl jucăm totuși
                for &c in &pool {
                    out.push(Move::Play(
                        c,
                        if c == JOKER && self.trump == NT {
                            Some(0)
                        } else {
                            None
                        },
                    ));
                }
            }
        } else {
            let led = self.led_suit();
            let follow: Vec<u8> = pool
                .iter()
                .copied()
                .filter(|&c| led.is_some() && eff_suit(c, self.trump) == led)
                .collect();
            let src = if follow.is_empty() { pool } else { follow };
            for c in src {
                out.push(Move::Play(c, None));
            }
        }
        out
    }

    fn start_play(&mut self) {
        self.phase = Phase::Play;
        self.leader = self.contractor;
        self.lead_up = false;
        self.side = self.contractor;
        self.trick.clear();
        self.nominated = None;
    }

    fn finish_hand(&mut self) {
        let c = self.contractor.idx();
        let o = 1 - c;
        let (n, s, _) = self.bid.expect("bid");
        let value = bid_value(n, s);
        let made = self.tricks[c] >= n;
        let gain = if made {
            if self.tricks[c] == 10 && value < 250 {
                250
            } else {
                value
            }
        } else {
            -value
        };
        self.score[c] += gain;
        self.score[o] += 10 * self.tricks[o] as i32;
        self.log = format!(
            "Contract J{} {}{} ({}): {} levate → {:+}; J{} {} levate → {:+}.",
            c + 1,
            n,
            if s == NT {
                "NT".to_string()
            } else {
                SUIT_CH[s as usize].to_string()
            },
            if made { "reușit" } else { "eșuat" },
            self.tricks[c],
            gain,
            o + 1,
            self.tricks[o],
            10 * self.tricks[o] as i32
        );
        if made && self.score[c] >= 500 {
            self.winner = Some(self.contractor);
        } else if self.score[c] <= -500 {
            self.winner = Some(self.contractor.other());
        } else if self.score[o] <= -500 {
            self.winner = Some(self.contractor);
        }
        self.phase = Phase::HandOver;
    }

    /// Puterea estimată a cărților cunoscute (mână + cărți cu fața în sus) pentru atuul `trump`,
    /// exprimată în levate așteptate.
    fn estimate(&self, p: Player, trump: u8) -> f64 {
        let mut cards = self.hands[p.idx()].clone();
        cards.extend(self.up[p.idx()].iter().flatten().copied());
        let mut v = 0.0;
        for &c in &cards {
            let s = eff_suit(c, trump);
            if trump < 4 && s == Some(trump) {
                v += match power(c, trump) {
                    100 => 1.0,
                    99 => 0.95,
                    98 => 0.9,
                    14 => 0.8,
                    13 => 0.6,
                    12 => 0.5,
                    _ => 0.35,
                };
            } else if c == JOKER {
                v += 0.9;
            } else {
                v += match rank_of(c) {
                    14 => 0.6,
                    13 => 0.25,
                    _ => 0.0,
                };
            }
        }
        // 15 cărți cunoscute din 20 ale jucătorului: scalăm la 10 levate
        v * 0.5 * 20.0 / cards.len().max(1) as f64 * (cards.len() as f64 / 15.0).min(1.3) + 0.5
    }
}

impl Game for FiveHundred {
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
        if self.winner.is_some() {
            return Vec::new();
        }
        match self.phase {
            Phase::HandOver => vec![Move::Next],
            Phase::Bid => {
                let mut v = vec![Move::Pass];
                let cur = self.bid.map(|(n, s, _)| bid_value(n, s)).unwrap_or(0);
                for n in 6..=10u8 {
                    for s in 0..=4u8 {
                        if bid_value(n, s) > cur {
                            v.push(Move::Bid(n, s));
                        }
                    }
                }
                v
            }
            Phase::Kitty => {
                let mut seen = Vec::new();
                for &c in &self.hands[self.side.idx()] {
                    if !seen.contains(&c) {
                        seen.push(c);
                    }
                }
                seen.into_iter().map(Move::Discard).collect()
            }
            Phase::Play => self.plays_for(self.side),
        }
    }

    fn apply(&mut self, m: &Move) {
        let me = self.side;
        match m {
            Move::Next => {
                self.dealer = self.dealer.other();
                self.log.clear();
                self.deal();
            }
            Move::Pass => {
                self.passed[me.idx()] = true;
                if let Some((_, _, b)) = self.bid {
                    self.contractor = b;
                    self.trump = self.bid.unwrap().1;
                    self.hands[b.idx()].extend(std::mem::take(&mut self.kitty));
                    self.sort_hands();
                    self.side = b;
                    self.phase = Phase::Kitty;
                } else if self.passed[0] && self.passed[1] {
                    self.log = "Toți au trecut: se redă.".into();
                    self.dealer = self.dealer.other();
                    self.deal();
                } else {
                    self.side = me.other();
                }
            }
            Move::Bid(n, s) => {
                self.bid = Some((*n, *s, me));
                if self.passed[me.other().idx()] {
                    self.contractor = me;
                    self.trump = *s;
                    self.hands[me.idx()].extend(std::mem::take(&mut self.kitty));
                    self.sort_hands();
                    self.side = me;
                    self.phase = Phase::Kitty;
                } else {
                    self.side = me.other();
                }
            }
            Move::Discard(c) => {
                let i = self.hands[me.idx()]
                    .iter()
                    .position(|x| x == c)
                    .expect("card");
                self.hands[me.idx()].remove(i);
                if self.hands[me.idx()].len() == 10 {
                    self.start_play();
                }
            }
            Move::Play(c, name) => {
                let from_up = self.step_is_up();
                if self.trick.is_empty() {
                    if let Some(s) = name {
                        self.nominated = Some(*s);
                    }
                    if let Some(s) = eff_suit(*c, self.trump) {
                        self.led[s as usize] = true;
                    }
                    if let Some(s) = name {
                        self.led[*s as usize] = true;
                    }
                }
                if from_up {
                    let k = self.up[me.idx()]
                        .iter()
                        .position(|x| *x == Some(*c))
                        .expect("face-up card");
                    self.up[me.idx()][k] = None;
                    // cartea de dedesubt se întoarce la sfârșitul levatei: o ținem în `down` până atunci
                } else {
                    let i = self.hands[me.idx()]
                        .iter()
                        .position(|x| x == c)
                        .expect("card");
                    self.hands[me.idx()].remove(i);
                }
                self.trick.push((me, *c, from_up));
                self.last_card = Some((me, *c));
                if self.trick.len() < 4 {
                    // următorul: conducătorul și celălalt alternează
                    self.side = if self.trick.len().is_multiple_of(2) {
                        self.leader
                    } else {
                        self.leader.other()
                    };
                } else {
                    let (wi, w) = self.trick_winner();
                    let won_up = self.trick[wi].2;
                    // întoarce cărțile de sub cele jucate cu fața în sus
                    for p in 0..2 {
                        for k in 0..5 {
                            if self.up[p][k].is_none() && self.down[p][k].is_some() {
                                self.up[p][k] = self.down[p][k].take();
                            }
                        }
                    }
                    self.tricks[w.idx()] += 1;
                    self.trick.clear();
                    self.nominated = None;
                    self.leader = w;
                    self.side = w;
                    self.lead_up = won_up;
                    if self.tricks[0] + self.tricks[1] == 10 {
                        self.finish_hand();
                    }
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.winner.map(Outcome::Win)
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let o = 1 - me;
        let mut v = (self.score[me] - self.score[o]) * 2;
        match self.phase {
            Phase::Bid => {
                // evaluare a ofertei făcute: valoare așteptată a contractului
                if let Some((n, s, b)) = self.bid {
                    let val = bid_value(n, s) as f64;
                    if b == p {
                        let est = self.estimate(p, s);
                        let pm = (0.5 + (est - n as f64) * 0.28).clamp(0.05, 0.95);
                        v += (val * (2.0 * pm - 1.0)) as i32;
                    } else {
                        v -= (val * 0.2) as i32;
                    }
                }
            }
            Phase::Kitty => {
                let (_, s, _) = self.bid.unwrap_or((6, NT, p));
                v += (self.estimate(p, s) * 40.0) as i32;
            }
            Phase::Play | Phase::HandOver => {
                v += (self.tricks[me] as i32 - self.tricks[o] as i32) * 100;
                if !self.trick.is_empty() {
                    let (_, w) = self.trick_winner();
                    v += if w == p { 60 } else { -60 };
                }
                if let Some((who, c)) = self.last_card {
                    if who == p {
                        let pw = if eff_suit(c, self.trump) == Some(self.trump) && self.trump < 4 {
                            power(c, self.trump) as i32 + 20
                        } else {
                            power(c, self.trump) as i32
                        };
                        if self.trick.len() == 1 {
                            v += pw / 3; // conducere cu o carte tare
                        } else {
                            v -= pw / 3; // consumă cât mai puțin
                        }
                    }
                }
            }
        }
        v
    }

    fn render(&self) -> String {
        let who = self.viewer.who(self.side);
        let mut s = format!(
            "Five Hundred (2 jucători) — J1 {} – J2 {} (câștigi la 500 cu contract reușit; pierzi la −500)\n",
            self.score[0], self.score[1]
        );
        match self.bid {
            Some((n, st, b)) => s.push_str(&format!(
                "Licitație: {}{} (J{}){}\n",
                n,
                if st == NT {
                    "NT".to_string()
                } else {
                    SUIT_CH[st as usize].to_string()
                },
                b.idx() + 1,
                if self.phase == Phase::Bid {
                    ""
                } else {
                    " — contract"
                }
            )),
            None => s.push_str("Licitație: —\n"),
        }
        if self.phase != Phase::Bid {
            s.push_str(&format!(
                "Atu: {} · levate: J1 {} – J2 {}\n",
                if self.trump == NT {
                    "fără atu".to_string()
                } else {
                    SUIT_CH[self.trump as usize].to_string()
                },
                self.tricks[0],
                self.tricks[1]
            ));
        }
        for p in 0..2 {
            let up: Vec<String> = self.up[p]
                .iter()
                .map(|c| c.map(text).unwrap_or_else(|| "·".into()))
                .collect();
            let downs = self.down[p].iter().filter(|c| c.is_some()).count();
            s.push_str(&format!(
                "J{}: cu fața în sus [{}], cu fața în jos: {}, în mână: {}\n",
                p + 1,
                up.join(" "),
                downs,
                self.hands[p].len()
            ));
        }
        if !self.trick.is_empty() {
            s.push_str(&format!(
                "Levata: {}\n",
                self.trick
                    .iter()
                    .map(|(p, c, up)| format!(
                        "J{}:{}{}",
                        p.idx() + 1,
                        text(*c),
                        if *up { "↑" } else { "" }
                    ))
                    .collect::<Vec<_>>()
                    .join(" ")
            ));
        }
        let v = who;
        s.push_str(&format!(
            "Mâna ta (Jucător {}): {}\n",
            v.idx() + 1,
            join(&self.hands[v.idx()])
        ));
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        match (self.winner, self.phase) {
            (Some(p), _) => {
                s.push_str(&format!("Joc încheiat: Jucător {} câștigă.\n", p.idx() + 1))
            }
            (_, Phase::HandOver) => s.push_str("Mâna s-a încheiat. Scrie «urmatoarea».\n"),
            (_, Phase::Bid) => s.push_str(&format!(
                "La mutare: Jucător {} — licit 7h / pas\n",
                self.side.idx() + 1
            )),
            (_, Phase::Kitty) => s.push_str(&format!(
                "La mutare: Jucător {} — aruncă 3 cărți din 13 (descarta As); au mai rămas {}\n",
                self.side.idx() + 1,
                self.hands[self.side.idx()].len() - 10
            )),
            (_, Phase::Play) => s.push_str(&format!(
                "La mutare: Jucător {} — joacă o carte {}\n",
                self.side.idx() + 1,
                if self.step_is_up() {
                    "cu fața în sus"
                } else {
                    "din mână"
                }
            )),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Pass => "pas".into(),
            Move::Next => "urmatoarea".into(),
            Move::Bid(n, s) => format!(
                "licit {}{}",
                n,
                if *s == NT {
                    "nt".to_string()
                } else {
                    SUIT_CH[*s as usize].to_string()
                }
            ),
            Move::Discard(c) => format!("descarta {}", text(*c)),
            Move::Play(c, None) => text(*c),
            Move::Play(c, Some(s)) => format!("{}:{}", text(*c), SUIT_CH[*s as usize]),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const H: u8 = 3;
    const D: u8 = 2;
    const S: u8 = 0;
    const C: u8 = 1;

    fn blank() -> FiveHundred {
        let mut g = FiveHundred::with_seed(1);
        g.hands = [Vec::new(), Vec::new()];
        g.up = [[None; 5]; 2];
        g.down = [[None; 5]; 2];
        g.kitty.clear();
        g.phase = Phase::Play;
        g.contractor = Player::One;
        g.leader = Player::One;
        g.side = Player::One;
        g.lead_up = false;
        g.trick.clear();
        g.tricks = [0; 2];
        g.bid = Some((6, H, Player::One));
        g.trump = H;
        g.led = [false; 4];
        g
    }

    #[test]
    fn deck_and_deal() {
        assert_eq!(full_deck().len(), 43);
        let g = FiveHundred::with_seed(2);
        assert_eq!(g.hands[0].len(), 10);
        assert_eq!(g.kitty.len(), 3);
        assert_eq!(g.up[1].iter().flatten().count(), 5);
        assert_eq!(g.down[0].iter().flatten().count(), 5);
        assert_eq!(g.to_move(), Player::One); // J2 a împărțit
    }

    #[test]
    fn bid_values_match_the_table() {
        assert_eq!(bid_value(6, S), 40);
        assert_eq!(bid_value(6, 4), 120);
        assert_eq!(bid_value(7, S), 140);
        assert_eq!(bid_value(8, C), 260);
        assert_eq!(bid_value(10, H), 500);
        assert_eq!(bid_value(10, 4), 520);
    }

    #[test]
    fn bidding_ends_when_one_player_is_left() {
        let mut g = FiveHundred::with_seed(3);
        g.apply(&Move::Bid(6, S));
        assert_eq!(g.to_move(), Player::Two);
        // J2 poate licita doar peste 6♠
        let mv = g.legal_moves();
        assert!(!mv.contains(&Move::Bid(6, S)));
        assert!(mv.contains(&Move::Bid(6, C)));
        g.apply(&Move::Pass);
        assert_eq!(g.phase, Phase::Kitty);
        assert_eq!(g.contractor, Player::One);
        assert_eq!(g.hands[0].len(), 13);
        for _ in 0..3 {
            let m = g.legal_moves()[0].clone();
            g.apply(&m);
        }
        assert_eq!(g.phase, Phase::Play);
        assert_eq!(g.hands[0].len(), 10);
        assert_eq!(g.to_move(), Player::One);
    }

    #[test]
    fn all_pass_redeals() {
        let mut g = FiveHundred::with_seed(4);
        let d = g.dealer;
        g.apply(&Move::Pass);
        g.apply(&Move::Pass);
        assert_eq!(g.phase, Phase::Bid);
        assert_ne!(g.dealer, d);
    }

    #[test]
    fn bowers_and_joker_rank_in_trump_contracts() {
        let right = mk(H, 11);
        let left = mk(D, 11);
        assert_eq!(eff_suit(left, H), Some(H));
        assert!(power(JOKER, H) > power(right, H));
        assert!(power(right, H) > power(left, H));
        assert!(power(left, H) > power(mk(H, 14), H));
        // fără atu: valetul rămâne la rangul lui
        assert_eq!(eff_suit(left, NT), Some(D));
        assert_eq!(power(left, NT), 11);
        // valetul de aceeași culoare nu e left bower pentru culoarea opusă
        assert_eq!(eff_suit(mk(S, 11), H), Some(S));
    }

    #[test]
    fn trick_follow_suit_and_winner() {
        let mut g = blank();
        g.hands[0] = vec![mk(S, 14), mk(H, 5)];
        g.hands[1] = vec![mk(S, 5), mk(D, 14)];
        g.up[0][0] = Some(mk(C, 14));
        g.up[1][0] = Some(mk(C, 5));
        g.apply(&Move::Play(mk(S, 14), None));
        // J2 are pică: obligat
        let m = g.legal_moves();
        assert_eq!(m, vec![Move::Play(mk(S, 5), None)]);
        g.apply(&m[0].clone());
        // acum cărțile cu fața în sus, conduce J1: aceeași culoare cerută (pică) — J1 n-are pică în sus, joacă orice
        assert!(g.step_is_up());
        let m = g.legal_moves();
        assert_eq!(m.len(), 1);
        g.apply(&m[0].clone());
        let m = g.legal_moves();
        g.apply(&m[0].clone());
        // J1 a câștigat cu As de pică (din mână): conduce din mână
        assert_eq!(g.tricks, [1, 0]);
        assert_eq!(g.leader, Player::One);
        assert!(!g.lead_up);
    }

    #[test]
    fn winning_with_a_face_up_card_leads_face_up_next_and_flips_the_card_below() {
        let mut g = blank();
        g.hands[0] = vec![mk(S, 6), mk(S, 7)];
        g.hands[1] = vec![mk(S, 5), mk(D, 4)];
        g.up[0][0] = Some(mk(C, 14));
        g.down[0][0] = Some(mk(C, 6));
        g.up[1][0] = Some(mk(C, 5));
        g.down[1][0] = Some(mk(D, 12));
        g.apply(&Move::Play(mk(S, 6), None));
        g.apply(&Move::Play(mk(S, 5), None)); // J2 răspunde pică din mână; J1 conduce cu 6 > 5
                                              // pasul 3: J1 joacă carte cu fața în sus (nu are pică în sus: orice)
        g.apply(&Move::Play(mk(C, 14), None));
        // J2 cu fața în sus
        g.apply(&Move::Play(mk(C, 5), None));
        // 6♠ a câștigat levata (din mână) -> J1 conduce din mână; ambele cărți de sub cele de sus s-au întors
        assert_eq!(g.up[0][0], Some(mk(C, 6)));
        assert_eq!(g.up[1][0], Some(mk(D, 12)));
        assert_eq!(g.tricks, [1, 0]);
        assert!(!g.lead_up);
        // dacă o carte cu fața în sus câștigă, următoarea levată începe cu fața în sus
        let mut h = blank();
        h.hands[0] = vec![mk(S, 6)];
        h.hands[1] = vec![mk(S, 7)];
        h.up[0][0] = Some(mk(H, 5)); // atu
        h.up[1][0] = Some(mk(D, 5));
        h.apply(&Move::Play(mk(S, 6), None));
        h.apply(&Move::Play(mk(S, 7), None)); // J2 conduce? nu: bate J2 (7>6) la hand, dar urmează up
                                              // pasul 3: J1 (conducător) cu fața în sus: nu are pică în sus, joacă atutul 5♥; J2 nu poate bate
        h.apply(&Move::Play(mk(H, 5), None));
        h.apply(&Move::Play(mk(D, 5), None));
        assert_eq!(h.leader, Player::One);
        assert!(h.lead_up);
    }

    #[test]
    fn nt_joker_rules() {
        let mut g = blank();
        g.trump = NT;
        g.hands[0] = vec![JOKER, mk(S, 5)];
        let t: Vec<String> = g.legal_moves().iter().map(|m| g.format_move(m)).collect();
        assert!(t.contains(&"Jk:h".to_string()));
        assert!(t.contains(&"5s".to_string()));
        g.led = [true; 4];
        let t: Vec<String> = g.legal_moves().iter().map(|m| g.format_move(m)).collect();
        assert!(!t.iter().any(|x| x.starts_with("Jk")));
        // la ultima levată se poate deschide cu joker
        g.tricks = [5, 4];
        let t: Vec<String> = g.legal_moves().iter().map(|m| g.format_move(m)).collect();
        assert!(t.iter().any(|x| x.starts_with("Jk")));
    }

    #[test]
    fn scoring_made_slam_and_failed_contracts() {
        let mut g = blank();
        g.bid = Some((7, H, Player::One));
        g.tricks = [7, 3];
        g.finish_hand();
        assert_eq!(g.score, [200, 30]);
        let mut h = blank();
        h.bid = Some((6, S, Player::One));
        h.tricks = [10, 0];
        h.finish_hand();
        assert_eq!(h.score, [250, 0]); // slem: 250 în loc de 40
        let mut k = blank();
        k.bid = Some((8, H, Player::One));
        k.tricks = [6, 4];
        k.finish_hand();
        assert_eq!(k.score, [-300, 40]);
        // câștig la 500 doar cu contract reușit
        let mut w = blank();
        w.score = [400, 0];
        w.bid = Some((7, H, Player::One));
        w.tricks = [7, 3];
        w.finish_hand();
        assert_eq!(w.outcome(), Some(Outcome::Win(Player::One)));
        // −500 pierde
        let mut l = blank();
        l.score = [-300, 0];
        l.bid = Some((8, H, Player::One));
        l.tricks = [6, 4];
        l.finish_hand();
        assert_eq!(l.outcome(), Some(Outcome::Win(Player::Two)));
    }

    #[test]
    fn defenders_cannot_win_by_tricks_alone() {
        let mut g = blank();
        g.score = [0, 490];
        g.bid = Some((6, S, Player::One));
        g.tricks = [4, 6]; // contract eșuat
        g.finish_hand();
        assert_eq!(g.score[1], 550);
        assert_eq!(g.outcome(), None);
    }

    #[test]
    fn random_matches_finish_and_cards_conserved() {
        for seed in 1..=10u64 {
            let mut g = FiveHundred::with_seed(seed);
            let mut rng = Rng::new(seed * 5);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty(), "stuck\n{}", Game::render(&g));
                let mut seen = std::collections::HashSet::new();
                for x in &m {
                    assert!(seen.insert(g.format_move(x)));
                }
                g.apply(&m[rng.below(m.len())]);
                if g.phase == Phase::Play {
                    let total = g.hands[0].len()
                        + g.hands[1].len()
                        + g.trick.len()
                        + g.up.iter().flatten().flatten().count()
                        + g.down.iter().flatten().flatten().count()
                        + (g.tricks[0] + g.tricks[1]) as usize * 4;
                    assert!(total <= 43);
                }
                n += 1;
                assert!(n < 5000, "too long");
            }
        }
    }

    #[test]
    fn evaluate_does_not_read_hidden_cards() {
        let g = FiveHundred::with_seed(7);
        let mut h = g.clone();
        // schimbă o carte cu fața în jos a adversarului cu una din kitty
        let a = h.down[1][0];
        h.down[1][0] = h.kitty.pop();
        h.kitty.push(a.unwrap());
        let mut hh = h.clone();
        hh.hands[1].swap(0, 1);
        for p in [Player::One, Player::Two] {
            assert_eq!(g.evaluate(p), h.evaluate(p));
            assert_eq!(g.evaluate(p), hh.evaluate(p));
        }
    }

    #[test]
    fn render_hides_opponent_hand_and_facedown() {
        let g = FiveHundred::with_seed(9);
        let r = Game::render(&g);
        for c in &g.hands[1] {
            // nu apar în „Mâna ta”
            let line = r.lines().find(|l| l.starts_with("Mâna ta")).unwrap();
            if !g.hands[0].contains(c) {
                assert!(!line.split_whitespace().skip(3).any(|t| t == text(*c)));
            }
        }
        assert!(r.contains("cu fața în jos: 5"));
    }

    #[test]
    fn ai_beats_random() {
        use polytabletop_core::GameObj;
        let mut wins = 0;
        let n = 8;
        for seed in 1..=n {
            let mut g: Box<dyn GameObj> = Box::new(FiveHundred::with_seed(seed));
            let mut rng = Rng::new(seed + 40);
            let mut plies = 0;
            while g.outcome().is_none() && plies < 6000 {
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
        assert!(wins * 10 >= n * 6, "AI won only {wins}/{n}");
    }
}
