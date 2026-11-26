//! Pinochle (varianta pentru 2 jucători), din regulile publice.
//!
//! * 48 de cărți: 9, 10, J, Q, K, A în fiecare culoare, câte **două** exemplare. Ordinea: A, 10, K, Q, J, 9.
//!   Fiecare primește 12 cărți; următoarea se întoarce (atu) și rămâne la fundul stocului (24 de cărți).
//!   Dacă cartea întoarsă e un 9 de atu, dealerul ia 10 puncte.
//! * Non-dealerul conduce. **Cât durează stocul** nu e obligatoriu să răspunzi la culoare; levata o ia cea
//!   mai mare carte din culoarea cerută sau un atu. Cine ia levata poate anunța **o combinație** (`anunt …`)
//!   sau spune `fara`; apoi trag din stoc, câștigătorul primul (perdantul primește la final cartea
//!   întoarsă). Cărțile anunțate rămân în mână și pot fi jucate; o combinație nouă trebuie să conțină cel
//!   puțin o carte neanunțată încă.
//! * **După golirea stocului** se răspunde obligatoriu la culoare și trebuie să bați cartea cerută dacă poți;
//!   fără culoare tai cu atu dacă ai. Nu se mai anunță combinații.
//! * Combinații: dix (9 de atu) 10; căsătorie (K+Q aceeași culoare) 20; căsătorie regală (de atu) 40;
//!   secvență (A,10,K,Q,J de atu) 150; pinochle (Q♠ + J♦) 40, dublu pinochle 300; „patru în cerc”: patru ași
//!   (câte unul din fiecare culoare) 100, patru popi 80, patru dame 60, patru valeți 40; de două ori
//!   (8 cărți) 1000 / 800 / 600 / 400.
//! * La final: fiecare As, 10 și K luat în levate valorează 10, ultima levată 10 (în total 250 de puncte de
//!   levate). Se joacă până la 1000 de puncte (cine atinge 1000 prin combinații câștigă pe loc).
//!
//! Omis: schimbul 9-ului de atu cu cartea întoarsă. Mutări: `Ah`, `10s`, `anunt nunta h`, `anunt regala`,
//! `anunt secventa`, `anunt dix`, `anunt pinochle`, `anunt asi`, `fara`, `urmatoarea`.
//! Informație ascunsă: vezi `p3_comun` (AI cinstit prin determinizare).

use crate::cards::{handover_banner, join_cards, shuffle, Card, Viewer};
use crate::p3_comun::{honest_eval, Honest};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const TARGET: i32 = 1000;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Play,
    Meld,
    HandOver,
    Over,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Meld {
    Dix,
    Marriage(u8),
    Royal,
    Run,
    Pinochle,
    DoublePinochle,
    Around(u8, bool),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Play(Card),
    Declare(Meld),
    Skip,
    Next,
}

#[derive(Clone)]
pub struct Pinochle {
    rng: Rng,
    stock: Vec<Card>,
    hands: [Vec<Card>; 2],
    shown: [Vec<Card>; 2],
    known: [Vec<Card>; 2],
    won: [Vec<Card>; 2],
    trick: Vec<(Player, Card)>,
    trump: u8,
    upcard: Card,
    dealer: Player,
    side: Player,
    phase: Phase,
    scores: [i32; 2],
    delta: [i32; 2],
    fresh: [u8; 2],
    prev: Player,
    log: String,
    viewer: Viewer,
}

impl Default for Pinochle {
    fn default() -> Self {
        Self::new()
    }
}

fn power(c: Card) -> u8 {
    match c.rank {
        14 => 5,
        10 => 4,
        13 => 3,
        12 => 2,
        11 => 1,
        _ => 0,
    }
}

fn cnt(v: &[Card], c: Card) -> usize {
    v.iter().filter(|&&x| x == c).count()
}

fn remove_one(v: &mut Vec<Card>, c: Card) -> bool {
    if let Some(i) = v.iter().position(|&x| x == c) {
        v.remove(i);
        true
    } else {
        false
    }
}

fn deck48() -> Vec<Card> {
    let mut d = Vec::new();
    for _ in 0..2 {
        for suit in 0..4u8 {
            for rank in [9u8, 10, 11, 12, 13, 14] {
                d.push(Card { rank, suit });
            }
        }
    }
    d
}

fn is_counter(c: Card) -> bool {
    matches!(c.rank, 14 | 10 | 13)
}

pub fn meld_points(m: Meld) -> i32 {
    match m {
        Meld::Dix => 10,
        Meld::Marriage(_) => 20,
        Meld::Royal => 40,
        Meld::Run => 150,
        Meld::Pinochle => 40,
        Meld::DoublePinochle => 300,
        Meld::Around(r, dbl) => {
            let b = match r {
                14 => 100,
                13 => 80,
                12 => 60,
                _ => 40,
            };
            if dbl {
                b * 10
            } else {
                b
            }
        }
    }
}

/// Cărțile cerute de o combinație (tip, număr de exemplare), pentru atutul `t`.
fn needs(m: Meld, t: u8) -> Vec<(Card, usize)> {
    let c = |rank, suit| Card { rank, suit };
    match m {
        Meld::Dix => vec![(c(9, t), 1)],
        Meld::Marriage(s) => vec![(c(13, s), 1), (c(12, s), 1)],
        Meld::Royal => vec![(c(13, t), 1), (c(12, t), 1)],
        Meld::Run => vec![
            (c(14, t), 1),
            (c(10, t), 1),
            (c(13, t), 1),
            (c(12, t), 1),
            (c(11, t), 1),
        ],
        Meld::Pinochle => vec![(c(12, 2), 1), (c(11, 1), 1)],
        Meld::DoublePinochle => vec![(c(12, 2), 2), (c(11, 1), 2)],
        Meld::Around(r, dbl) => (0..4).map(|s| (c(r, s), if dbl { 2 } else { 1 })).collect(),
    }
}

fn meld_text(m: Meld) -> String {
    match m {
        Meld::Dix => "dix".into(),
        Meld::Marriage(s) => format!("nunta {}", ['h', 'd', 's', 'c'][s as usize]),
        Meld::Royal => "regala".into(),
        Meld::Run => "secventa".into(),
        Meld::Pinochle => "pinochle".into(),
        Meld::DoublePinochle => "dpinochle".into(),
        Meld::Around(r, d) => format!(
            "{}{}",
            if d { "d" } else { "" },
            match r {
                14 => "asi",
                13 => "regi",
                12 => "dame",
                _ => "valeti",
            }
        ),
    }
}

impl Pinochle {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Pinochle {
            rng: Rng::new(seed),
            stock: Vec::new(),
            hands: [Vec::new(), Vec::new()],
            shown: [Vec::new(), Vec::new()],
            known: [Vec::new(), Vec::new()],
            won: [Vec::new(), Vec::new()],
            trick: Vec::new(),
            trump: 0,
            upcard: Card { rank: 9, suit: 0 },
            dealer: Player::Two,
            side: Player::One,
            phase: Phase::Play,
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
        let mut d = deck48();
        shuffle(&mut d, &mut self.rng);
        self.hands = [Vec::new(), Vec::new()];
        for _ in 0..12 {
            for p in 0..2 {
                self.hands[p].push(d.pop().unwrap());
            }
        }
        // întoarcem următoarea carte, o punem la fundul stocului (indexul 0)
        let up = d.pop().unwrap();
        d.insert(0, up);
        self.upcard = up;
        self.trump = up.suit;
        self.stock = d;
        self.shown = [Vec::new(), Vec::new()];
        self.known = [Vec::new(), Vec::new()];
        self.won = [Vec::new(), Vec::new()];
        self.trick.clear();
        self.delta = [0, 0];
        self.fresh = [0, 0];
        self.sort_hands();
        self.phase = Phase::Play;
        self.side = self.dealer.other();
        self.log.clear();
        if up.rank == 9 {
            let dl = self.dealer;
            self.scores[dl.idx()] += 10;
            self.delta[dl.idx()] += 10;
            self.log = format!("Cartea întoarsă e dix: J{} ia 10 puncte.", dl.idx() + 1);
        }
    }

    fn sort_hands(&mut self) {
        let t = self.trump;
        for h in &mut self.hands {
            h.sort_by_key(|c| (c.suit != t, c.suit, std::cmp::Reverse(power(*c))));
        }
    }

    fn endgame(&self) -> bool {
        self.stock.is_empty()
    }

    fn melds_available(&self, p: Player) -> Vec<Meld> {
        let t = self.trump;
        let mut all = vec![
            Meld::Dix,
            Meld::Royal,
            Meld::Run,
            Meld::Pinochle,
            Meld::DoublePinochle,
        ];
        for s in 0..4u8 {
            if s != t {
                all.push(Meld::Marriage(s));
            }
        }
        for r in [14u8, 13, 12, 11] {
            all.push(Meld::Around(r, false));
            all.push(Meld::Around(r, true));
        }
        let (h, sh) = (&self.hands[p.idx()], &self.shown[p.idx()]);
        all.into_iter()
            .filter(|&m| {
                let n = needs(m, t);
                n.iter().all(|&(c, k)| cnt(h, c) >= k) && n.iter().any(|&(c, k)| k > cnt(sh, c))
            })
            .collect()
    }

    fn play_cards(&self) -> Vec<Card> {
        let h = &self.hands[self.side.idx()];
        let mut v: Vec<Card> = Vec::new();
        let mut push = |c: Card| {
            if !v.contains(&c) {
                v.push(c);
            }
        };
        if self.trick.is_empty() || !self.endgame() {
            for &c in h {
                push(c);
            }
            return v;
        }
        let led = self.trick[0].1;
        let same: Vec<Card> = h.iter().copied().filter(|c| c.suit == led.suit).collect();
        if !same.is_empty() {
            let head: Vec<Card> = same
                .iter()
                .copied()
                .filter(|&c| power(c) > power(led))
                .collect();
            let pool = if head.is_empty() { same } else { head };
            for c in pool {
                push(c);
            }
        } else {
            let tr: Vec<Card> = h.iter().copied().filter(|c| c.suit == self.trump).collect();
            let pool = if tr.is_empty() { h.clone() } else { tr };
            for c in pool {
                push(c);
            }
        }
        v
    }

    fn draw_after(&mut self, w: Player) {
        let l = w.other();
        for p in [w, l] {
            if let Some(c) = self.stock.pop() {
                self.hands[p.idx()].push(c);
                self.fresh[p.idx()] += 1;
                if self.stock.is_empty() && p == l {
                    // ultima carte (cea întoarsă) e cunoscută de ambii
                    self.known[p.idx()].push(c);
                }
            }
        }
        self.sort_keep_fresh();
        self.side = w;
        self.phase = Phase::Play;
    }

    /// Sortează mâinile, dar păstrează cărțile proaspăt trase la sfârșit (determinizarea le re-eșantionează).
    fn sort_keep_fresh(&mut self) {
        let t = self.trump;
        for p in 0..2 {
            let nf = (self.fresh[p] as usize).min(self.hands[p].len());
            let split = self.hands[p].len() - nf;
            self.hands[p][..split]
                .sort_by_key(|c| (c.suit != t, c.suit, std::cmp::Reverse(power(*c))));
        }
    }

    fn end_hand(&mut self, last: Player) {
        for p in 0..2 {
            let n = self.won[p].iter().filter(|&&c| is_counter(c)).count() as i32 * 10;
            self.scores[p] += n;
            self.delta[p] += n;
        }
        self.scores[last.idx()] += 10;
        self.delta[last.idx()] += 10;
        self.log = format!(
            "Mână încheiată: J1 {:+}, J2 {:+} (ultima levată: J{})",
            self.delta[0],
            self.delta[1],
            last.idx() + 1
        );
        self.phase = if self.scores[0] >= TARGET || self.scores[1] >= TARGET {
            Phase::Over
        } else {
            Phase::HandOver
        };
    }

    fn after_meld(&mut self, w: Player) {
        if self.scores[w.idx()] >= TARGET {
            self.phase = Phase::Over;
            return;
        }
        self.draw_after(w);
    }
}

impl Game for Pinochle {
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
            Phase::Play => self.play_cards().into_iter().map(Move::Play).collect(),
            Phase::Meld => {
                let mut v = vec![Move::Skip];
                v.extend(
                    self.melds_available(self.side)
                        .into_iter()
                        .map(Move::Declare),
                );
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        self.prev = self.side;
        self.fresh = [0, 0];
        match *m {
            Move::Next => {
                self.dealer = self.dealer.other();
                self.deal();
            }
            Move::Skip => {
                let w = self.side;
                self.after_meld(w);
            }
            Move::Declare(md) => {
                let w = self.side;
                let t = self.trump;
                for (c, k) in needs(md, t) {
                    while cnt(&self.shown[w.idx()], c) < k {
                        self.shown[w.idx()].push(c);
                    }
                }
                let pts = meld_points(md);
                self.scores[w.idx()] += pts;
                self.delta[w.idx()] += pts;
                self.log = format!("J{} anunță {} (+{})", w.idx() + 1, meld_text(md), pts);
                self.after_meld(w);
            }
            Move::Play(c) => {
                let p = self.side;
                remove_one(&mut self.hands[p.idx()], c);
                if cnt(&self.shown[p.idx()], c) > cnt(&self.hands[p.idx()], c) {
                    remove_one(&mut self.shown[p.idx()], c);
                }
                remove_one(&mut self.known[p.idx()], c);
                self.trick.push((p, c));
                if self.trick.len() < 2 {
                    self.side = p.other();
                    return;
                }
                let (lead, reply) = (self.trick[0], self.trick[1]);
                let w = if reply.1.suit == lead.1.suit {
                    if power(reply.1) > power(lead.1) {
                        reply.0
                    } else {
                        lead.0
                    }
                } else if reply.1.suit == self.trump {
                    reply.0
                } else {
                    lead.0
                };
                self.won[w.idx()].push(lead.1);
                self.won[w.idx()].push(reply.1);
                self.trick.clear();
                if self.hands[0].is_empty() && self.stock.is_empty() {
                    self.end_hand(w);
                } else if !self.endgame() {
                    self.phase = Phase::Meld;
                    self.side = w;
                } else {
                    self.side = w;
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
        if self.prev != self.side && matches!(self.phase, Phase::Play | Phase::Meld) {
            if let Viewer::HotSeat = self.viewer {
                s.push_str(&handover_banner(self.side));
            }
        }
        s.push_str(&format!(
            "Pinochle — scor: J1 {}, J2 {} (până la {}) | dealer: J{}\n",
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
            _ => {}
        }
        s.push_str(&format!(
            "Atu: {} | stoc: {} cărți{}\n",
            ['h', 'd', 's', 'c'][self.trump as usize],
            self.stock.len(),
            if self.stock.is_empty() {
                " (joc strâns: culoare și bătaie obligatorii)".to_string()
            } else {
                format!(" (jos: {})", self.upcard.text())
            }
        ));
        for p in 0..2 {
            if !self.shown[p].is_empty() {
                s.push_str(&format!(
                    "Anunțate de J{} (încă în mână): {}\n",
                    p + 1,
                    join_cards(&self.shown[p])
                ));
            }
        }
        if !self.trick.is_empty() {
            let cs: Vec<Card> = self.trick.iter().map(|&(_, c)| c).collect();
            s.push_str(&format!("Pe masă: {}\n", join_cards(&cs)));
        }
        if self.phase == Phase::Meld {
            s.push_str(&format!(
                "J{} a luat levata: anunță o combinație sau «fara».\n",
                self.side.idx() + 1
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
            Move::Play(c) => c.text(),
            Move::Declare(md) => format!("anunt {}", meld_text(*md)),
            Move::Skip => "fara".into(),
            Move::Next => "urmatoarea".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

impl Honest for Pinochle {
    fn determinize(&self, me: Player, rng: &mut Rng) -> Self {
        let mut g = self.clone();
        let (mi, oi) = (me.idx(), 1 - me.idx());
        let nf = (self.fresh[mi] as usize).min(self.hands[mi].len());
        let known_mine: Vec<Card> = self.hands[mi][..self.hands[mi].len() - nf].to_vec();
        let mut pool = deck48();
        for &c in known_mine
            .iter()
            .chain(self.won[0].iter())
            .chain(self.won[1].iter())
            .chain(self.trick.iter().map(|(_, c)| c))
            .chain(self.shown[oi].iter())
        {
            remove_one(&mut pool, c);
        }
        // cartea întoarsă: încă la fundul stocului sau cunoscută în mâna cuiva
        let up_in_stock = !self.stock.is_empty();
        if up_in_stock {
            remove_one(&mut pool, self.upcard);
        } else {
            for &c in &self.known[oi] {
                if !self.shown[oi].contains(&c) {
                    remove_one(&mut pool, c);
                }
            }
        }
        shuffle(&mut pool, rng);
        let mut opp: Vec<Card> = self.shown[oi].clone();
        if !up_in_stock {
            for &c in &self.known[oi] {
                if !opp.contains(&c) {
                    opp.push(c);
                }
            }
        }
        let n_opp = self.hands[oi].len();
        while opp.len() < n_opp {
            opp.push(pool.pop().unwrap());
        }
        let mut mine = known_mine;
        for _ in 0..nf {
            mine.push(pool.pop().unwrap());
        }
        g.hands[mi] = mine;
        g.hands[oi] = opp;
        if up_in_stock {
            // restul stocului: cărți necunoscute, cu cartea întoarsă la fund
            let n = self.stock.len() - 1;
            let mut st: Vec<Card> = vec![self.upcard];
            for _ in 0..n {
                st.push(pool.pop().unwrap());
            }
            g.stock = st;
        }
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
        (self.delta[p.idx()] - self.delta[p.other().idx()]) as f64 / 10.0
    }

    fn policy(&self, rng: &mut Rng) -> Move {
        let mv = self.legal_moves();
        if mv.len() == 1 {
            return mv[0];
        }
        match self.phase {
            Phase::Meld => {
                // cea mai valoroasă combinație
                let mut best = Move::Skip;
                let mut bp = 0;
                for &m in &mv {
                    if let Move::Declare(md) = m {
                        let pts = meld_points(md);
                        if pts > bp {
                            bp = pts;
                            best = m;
                        }
                    }
                }
                best
            }
            _ => {
                if rng.below(100) < 8 {
                    return mv[rng.below(mv.len())];
                }
                let t = self.trump;
                let key = |c: Card| power(c) as i32 + if c.suit == t { 10 } else { 0 };
                let cards: Vec<Card> = mv
                    .iter()
                    .filter_map(|m| {
                        if let Move::Play(c) = m {
                            Some(*c)
                        } else {
                            None
                        }
                    })
                    .collect();
                if let Some(&(_, lead)) = self.trick.first() {
                    let wins = |c: Card| {
                        if c.suit == lead.suit {
                            power(c) > power(lead)
                        } else {
                            c.suit == t
                        }
                    };
                    let rich = is_counter(lead) || lead.suit == t;
                    let mut best_win: Option<Card> = None;
                    for &c in &cards {
                        if wins(c) && best_win.is_none_or(|b| key(c) < key(b)) {
                            best_win = Some(c);
                        }
                    }
                    if let Some(w) = best_win {
                        if rich || !is_counter(w) {
                            return Move::Play(w);
                        }
                    }
                    let low = cards
                        .iter()
                        .copied()
                        .min_by_key(|&c| (is_counter(c), key(c)))
                        .unwrap();
                    Move::Play(low)
                } else {
                    // conducere: o carte mare dintr-o culoare non-atu dacă e As, altfel cea mai mică
                    let ace = cards.iter().copied().find(|c| c.rank == 14 && c.suit != t);
                    match ace {
                        Some(a) => Move::Play(a),
                        None => Move::Play(
                            cards
                                .iter()
                                .copied()
                                .min_by_key(|&c| (is_counter(c), key(c)))
                                .unwrap(),
                        ),
                    }
                }
            }
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
    fn deal_sizes() {
        for seed in 1..10u64 {
            let g = Pinochle::with_seed(seed);
            assert_eq!(g.hands[0].len(), 12);
            assert_eq!(g.hands[1].len(), 12);
            assert_eq!(g.stock.len(), 24);
            assert_eq!(g.stock[0], g.upcard);
            assert_eq!(g.trump, g.upcard.suit);
        }
    }

    #[test]
    fn meld_values_and_availability() {
        let mut g = Pinochle::with_seed(1);
        g.trump = 0;
        g.hands[0] = vec![c(13, 0), c(12, 0), c(12, 2), c(11, 1), c(13, 2), c(9, 0)];
        g.shown[0].clear();
        let av = g.melds_available(Player::One);
        assert!(av.contains(&Meld::Royal));
        assert!(av.contains(&Meld::Marriage(2)));
        assert!(av.contains(&Meld::Pinochle));
        assert!(av.contains(&Meld::Dix));
        assert!(!av.contains(&Meld::Run));
        assert!(!av.contains(&Meld::DoublePinochle));
        assert_eq!(meld_points(Meld::Around(14, true)), 1000);
        assert_eq!(meld_points(Meld::Around(11, false)), 40);
        assert_eq!(meld_points(Meld::Run), 150);
    }

    #[test]
    fn new_meld_needs_a_new_card() {
        let mut g = Pinochle::with_seed(2);
        g.trump = 0;
        g.hands[0] = vec![c(13, 0), c(12, 0)];
        g.shown[0] = vec![c(13, 0), c(12, 0)];
        assert!(!g.melds_available(Player::One).contains(&Meld::Royal));
        // secvența după căsătoria regală: A,10,J nou
        g.hands[0] = vec![c(13, 0), c(12, 0), c(14, 0), c(10, 0), c(11, 0)];
        assert!(g.melds_available(Player::One).contains(&Meld::Run));
        // dublu pinochle cere 2+2
        g.hands[0] = vec![c(12, 2), c(12, 2), c(11, 1), c(11, 1)];
        g.shown[0].clear();
        assert!(g
            .melds_available(Player::One)
            .contains(&Meld::DoublePinochle));
    }

    #[test]
    fn declare_scores_and_marks_cards() {
        let mut g = Pinochle::with_seed(3);
        g.trump = 0;
        g.hands[0] = vec![c(13, 0), c(12, 0), c(9, 3)];
        g.phase = Phase::Meld;
        g.side = Player::One;
        g.scores = [0, 0];
        let before = g.stock.len();
        g.apply(&Move::Declare(Meld::Royal));
        assert_eq!(g.scores[0], 40);
        assert!(g.shown[0].contains(&c(13, 0)));
        assert_eq!(g.stock.len(), before - 2);
        assert_eq!(g.phase, Phase::Play);
        assert_eq!(g.side, Player::One);
    }

    #[test]
    fn endgame_must_follow_and_head() {
        let mut g = Pinochle::with_seed(4);
        g.stock.clear();
        g.trump = 0;
        g.phase = Phase::Play;
        g.side = Player::Two;
        g.trick = vec![(Player::One, c(13, 2))];
        g.hands[1] = vec![c(9, 2), c(14, 2), c(12, 3), c(9, 0)];
        let mv = g.legal_moves();
        assert_eq!(mv, vec![Move::Play(c(14, 2))]); // trebuie să bată popa de pică
        g.hands[1] = vec![c(9, 2), c(11, 2)];
        let mv = g.legal_moves();
        assert_eq!(mv.len(), 2); // nu poate bate: orice carte de pică
        g.hands[1] = vec![c(12, 3), c(9, 0)];
        assert_eq!(g.legal_moves(), vec![Move::Play(c(9, 0))]); // fără pică: trebuie atu
        g.hands[1] = vec![c(12, 3), c(10, 1)];
        assert_eq!(g.legal_moves().len(), 2);
    }

    #[test]
    fn stock_phase_is_free() {
        let mut g = Pinochle::with_seed(5);
        g.trick = vec![(Player::One, c(13, 2))];
        g.side = Player::Two;
        let mut u: Vec<Card> = Vec::new();
        for &c in &g.hands[1] {
            if !u.contains(&c) {
                u.push(c);
            }
        }
        let n = u.len();
        assert_eq!(g.legal_moves().len(), n);
    }

    #[test]
    fn counters_and_last_trick() {
        let mut g = Pinochle::with_seed(6);
        g.scores = [0, 0];
        g.delta = [0, 0];
        g.won = [vec![c(14, 0), c(10, 1), c(9, 2)], vec![c(13, 3), c(12, 1)]];
        g.end_hand(Player::Two);
        assert_eq!(g.scores, [20, 10 + 10]);
    }

    #[test]
    fn determinize_hides_opponent_but_keeps_shown() {
        let mut g = Pinochle::with_seed(7);
        g.shown[1] = vec![g.hands[1][0]];
        let mut rng = Rng::new(9);
        let d = g.determinize(Player::One, &mut rng);
        assert_eq!(d.hands[0], g.hands[0]);
        assert_eq!(d.hands[1].len(), 12);
        assert!(d.hands[1].contains(&g.hands[1][0]));
        assert_eq!(d.stock.len(), g.stock.len());
        assert_eq!(d.stock[0], g.upcard);
        // totalul cărților se conservă
        let mut all: Vec<Card> = d.hands[0]
            .iter()
            .chain(&d.hands[1])
            .chain(&d.stock)
            .copied()
            .collect();
        all.sort_by_key(|c| (c.suit, c.rank));
        let mut want = deck48();
        want.sort_by_key(|c| (c.suit, c.rank));
        assert_eq!(all, want);
    }

    #[test]
    fn full_hand_conserves_cards_and_ends() {
        for seed in 1..=6u64 {
            let mut g = Pinochle::with_seed(seed);
            let mut rng = Rng::new(seed + 1);
            let mut n = 0;
            while g.outcome().is_none() && n < 20_000 {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
                let total = g.hands[0].len()
                    + g.hands[1].len()
                    + g.stock.len()
                    + g.won[0].len()
                    + g.won[1].len()
                    + g.trick.len();
                assert_eq!(total, 48, "seed {seed}");
                n += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed}");
        }
    }
}
