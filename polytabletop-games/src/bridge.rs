//! Bridge duplicat pentru 2 jucători (cu mâna moartă), din regulile publice.
//!
//! **Cum devine bridge-ul un joc cinstit în doi.** În bridge-ul real cei patru jucători sunt
//! parteneri doi câte doi. Aici fiecare jucător conduce două mâini (un „cuplu” care își vede
//! toate cele 26 de cărți): J1 conduce Nord–Sud, J2 conduce Est–Vest. Aceeași donă (mâinile) se
//! joacă de DOUĂ ORI, ca în bridge-ul duplicat: la tabla 1 J1 are N–S, la tabla 2 rolurile se
//! schimbă (J1 conduce E–V, J2 conduce N–S, aceleași cărți). Câștigă cine are scorul combinat mai
//! bun (scor N–S tabla 1 − scor N–S tabla 2, văzut de J1). Cine joacă a doua oară același set
//! de cărți își poate aminti de la prima tablă — e natura duplicatului cu o singură donă.
//!
//! * Licitație: cele două părți licitează pe rând (Nord–Sud începe): `1c`…`7n` (c, d, h, s, n =
//!   fără atu), `pass`, `x` (contra), `xx` (recontra). Licitația se termină după un `pass`
//!   care urmează unei licitații/contre; dacă primele două chemări sunt pass, tabla e „aruncată” (0).
//! * Declarant: Sud (la N–S) sau Vest (la E–V); mâna moartă (Nord / Est) se arată după prima carte
//!   jucată, iar declarantul o joacă. Se conduce la stânga declarantului. Se răspunde la culoare.
//! * Scor (nimeni vulnerabil): puncte pe levată (minor 20, major 30, fără atu 40+30…, ×2 contrat,
//!   ×4 recontrat), bonus manșă (300 dacă punctele de contract ≥ 100, altfel 50), șlem mic 500,
//!   șlem mare 1000, levate peste contract, bonus de „insultă” (50/100), căderi (50 / 100-200-300…).
//!
//! Informația ascunsă: render() arată doar mâinile deținute de J1 și mâna moartă, când e vizibilă;
//! AI-ul citește doar mâinile lui și mâna moartă.
//!
//! Mutări: `pass`, `x`, `xx`, `1c`…`7n` (licitație); `Ah`, `10s` (carte, din mâna al cărei rând e).

use crate::cards::{full_deck, join_cards, shuffle, Card, Viewer};
use crate::cardsb_util::{has_suit, sort_hand, trick_winner};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const SEAT: [char; 4] = ['N', 'E', 'S', 'W'];
const STRAIN: [char; 5] = ['c', 'd', 'h', 's', 'n'];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Auction,
    Play,
    Over,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Pass,
    Double,
    Redouble,
    Bid(u8, u8),
    Play(Card),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Contract {
    level: u8,
    strain: u8,
    dbl: u8,
    /// 0 = N–S, 1 = E–V
    side: u8,
}

#[derive(Clone)]
pub struct Bridge {
    deal: [Vec<Card>; 4],
    hands: [Vec<Card>; 4],
    board: u8,
    phase: Phase,
    calls: Vec<Move>,
    contract: Option<Contract>,
    turn: usize,
    trick: Vec<(usize, Card)>,
    tricks: [u8; 2],
    dummy_open: bool,
    results: [Option<i32>; 2],
    log: String,
    viewer: Viewer,
}

impl Default for Bridge {
    fn default() -> Self {
        Self::new()
    }
}

fn strain_suit(st: u8) -> Option<u8> {
    match st {
        0 => Some(3),
        1 => Some(1),
        2 => Some(0),
        3 => Some(2),
        _ => None,
    }
}

fn hcp(h: &[Card]) -> i32 {
    h.iter()
        .map(|c| match c.rank {
            14 => 4,
            13 => 3,
            12 => 2,
            11 => 1,
            _ => 0,
        })
        .sum()
}

fn trick_value(strain: u8, n: u8) -> i32 {
    match strain {
        0 | 1 => 20,
        2 | 3 => 30,
        _ => {
            if n == 1 {
                40
            } else {
                30
            }
        }
    }
}

/// Scorul declarantului (nimeni vulnerabil). `tricks` = levate luate de declarant.
pub fn contract_score(level: u8, strain: u8, dbl: u8, tricks: u8) -> i32 {
    let need = level + 6;
    let mult = [1, 2, 4][dbl as usize];
    if tricks >= need {
        let mut base = 0;
        for n in 1..=level {
            base += trick_value(strain, n);
        }
        base *= mult;
        let mut s = base;
        s += if base >= 100 { 300 } else { 50 };
        if level == 6 {
            s += 500;
        }
        if level == 7 {
            s += 1000;
        }
        let over = (tricks - need) as i32;
        s += match dbl {
            0 => over * trick_value(strain, 2),
            1 => over * 100,
            _ => over * 200,
        };
        s += match dbl {
            0 => 0,
            1 => 50,
            _ => 100,
        };
        s
    } else {
        let down = (need - tricks) as i32;
        let mut s = 0;
        for k in 1..=down {
            s += match dbl {
                0 => 50,
                _ => {
                    let v = match k {
                        1 => 100,
                        2 | 3 => 200,
                        _ => 300,
                    };
                    v * (dbl as i32)
                }
            };
        }
        -s
    }
}

impl Bridge {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut d = full_deck(2);
        shuffle(&mut d, &mut rng);
        let mut deal: [Vec<Card>; 4] = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        for (i, h) in deal.iter_mut().enumerate() {
            *h = d[i * 13..(i + 1) * 13].to_vec();
            sort_hand(h);
        }
        let mut g = Bridge {
            hands: deal.clone(),
            deal,
            board: 0,
            phase: Phase::Auction,
            calls: Vec::new(),
            contract: None,
            turn: 0,
            trick: Vec::new(),
            tricks: [0, 0],
            dummy_open: false,
            results: [None, None],
            log: String::new(),
            viewer: Viewer::Fixed(Player::One),
        };
        g.start_board(0);
        g
    }

    /// Rezultatul (N–S) al fiecărei table, dacă s-a încheiat.
    pub fn board_results(&self) -> (Option<i32>, Option<i32>) {
        (self.results[0], self.results[1])
    }

    fn start_board(&mut self, b: u8) {
        self.board = b;
        self.hands = self.deal.clone();
        self.phase = Phase::Auction;
        self.calls.clear();
        self.contract = None;
        self.trick.clear();
        self.tricks = [0, 0];
        self.dummy_open = false;
        self.turn = 0;
    }

    /// Proprietarul N–S la tabla curentă.
    fn ns_owner(&self) -> Player {
        if self.board == 0 {
            Player::One
        } else {
            Player::Two
        }
    }

    fn owner_of_seat(&self, seat: usize) -> Player {
        if seat.is_multiple_of(2) {
            self.ns_owner()
        } else {
            self.ns_owner().other()
        }
    }

    fn auction_side(&self) -> u8 {
        (self.calls.len() % 2) as u8
    }

    fn last_bid(&self) -> Option<(u8, u8, u8)> {
        let mut best = None;
        for (i, c) in self.calls.iter().enumerate() {
            if let Move::Bid(l, s) = c {
                best = Some((*l, *s, (i % 2) as u8));
            }
        }
        best
    }

    fn dbl_state(&self) -> u8 {
        let mut d = 0;
        for c in &self.calls {
            match c {
                Move::Bid(..) => d = 0,
                Move::Double => d = 1,
                Move::Redouble => d = 2,
                _ => {}
            }
        }
        d
    }

    fn finish_auction(&mut self) {
        match self.last_bid() {
            None => {
                self.finish_board(0);
            }
            Some((level, strain, side)) => {
                let dbl = self.dbl_state();
                self.contract = Some(Contract {
                    level,
                    strain,
                    dbl,
                    side,
                });
                let decl = if side == 0 { 2 } else { 3 };
                self.turn = (decl + 1) % 4;
                self.phase = Phase::Play;
            }
        }
    }

    fn finish_board(&mut self, ns_score: i32) {
        self.results[self.board as usize] = Some(ns_score);
        self.log = format!("Tabla {} încheiată: N–S {:+}", self.board + 1, ns_score);
        if self.board == 0 {
            self.start_board(1);
        } else {
            self.phase = Phase::Over;
        }
    }

    fn declarer(&self) -> usize {
        match self.contract {
            Some(c) if c.side == 0 => 2,
            _ => 3,
        }
    }

    fn dummy(&self) -> usize {
        (self.declarer() + 2) % 4
    }

    fn trump(&self) -> Option<u8> {
        self.contract.and_then(|c| strain_suit(c.strain))
    }

    fn my_seats(&self, p: Player) -> [usize; 2] {
        if self.ns_owner() == p {
            [0, 2]
        } else {
            [1, 3]
        }
    }

    /// Estimare a levatelor pentru partea lui `p` cu atuul `strain`.
    fn est(&self, p: Player, strain: u8) -> f32 {
        let s = self.my_seats(p);
        let mut all: Vec<Card> = self.hands[s[0]].clone();
        all.extend(&self.hands[s[1]]);
        let hc = hcp(&all) as f32;
        let mut e = 6.0 + (hc - 20.0) * 0.3;
        match strain_suit(strain) {
            Some(t) => {
                let n = all.iter().filter(|c| c.suit == t).count() as f32;
                e += (n - 8.0) * 0.8;
            }
            None => {
                let mut longest = 0;
                for su in 0..4u8 {
                    longest = longest.max(all.iter().filter(|c| c.suit == su).count());
                }
                e += 0.4 * (longest as f32 - 6.0).max(0.0);
            }
        }
        e
    }
}

fn prob(margin: f32) -> f32 {
    1.0 / (1.0 + (-1.3 * margin).exp())
}

impl Game for Bridge {
    type Move = Move;

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = match who {
            Some(p) => Viewer::Fixed(p),
            None => Viewer::HotSeat,
        };
    }

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Auction => {
                if self.auction_side() == 0 {
                    self.ns_owner()
                } else {
                    self.ns_owner().other()
                }
            }
            _ => self.owner_of_seat(self.turn),
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        match self.phase {
            Phase::Over => Vec::new(),
            Phase::Auction => {
                let mut v = vec![Move::Pass];
                let lb = self.last_bid();
                let side = self.auction_side();
                let floor = lb.map(|(l, s, _)| l as i32 * 5 + s as i32).unwrap_or(-1);
                for l in 1..=7u8 {
                    for s in 0..5u8 {
                        if l as i32 * 5 + s as i32 > floor {
                            v.push(Move::Bid(l, s));
                        }
                    }
                }
                if let Some((_, _, bs)) = lb {
                    let d = self.dbl_state();
                    if bs != side && d == 0 {
                        v.push(Move::Double);
                    }
                    if bs == side && d == 1 {
                        v.push(Move::Redouble);
                    }
                }
                v
            }
            Phase::Play => {
                let h = &self.hands[self.turn];
                let eff = |c: Card| c.suit;
                let must = self
                    .trick
                    .first()
                    .map(|&(_, c)| c.suit)
                    .filter(|&s| has_suit(h, s, &eff));
                h.iter()
                    .filter(|c| must.is_none_or(|s| c.suit == s))
                    .map(|&c| Move::Play(c))
                    .collect()
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        match *m {
            Move::Play(c) => {
                let seat = self.turn;
                let pos = self.hands[seat].iter().position(|&x| x == c).unwrap();
                self.hands[seat].remove(pos);
                self.trick.push((seat, c));
                if self.trick.len() == 1 && !self.dummy_open {
                    self.dummy_open = true;
                }
                if self.trick.len() < 4 {
                    self.turn = (seat + 1) % 4;
                    return;
                }
                let cards: Vec<Card> = self.trick.iter().map(|&(_, c)| c).collect();
                let tr = self.trump();
                let w = trick_winner(&cards, tr, &|c| c.suit, &|c| c.rank as u32);
                let ws = self.trick[w].0;
                self.tricks[ws % 2] += 1;
                self.trick.clear();
                self.turn = ws;
                if self.hands[0].is_empty() {
                    let c = self.contract.unwrap();
                    let dt = self.tricks[c.side as usize];
                    let sc = contract_score(c.level, c.strain, c.dbl, dt);
                    let ns = if c.side == 0 { sc } else { -sc };
                    self.finish_board(ns);
                }
            }
            call => {
                self.calls.push(call);
                let n = self.calls.len();
                let ended = if self.last_bid().is_none() {
                    n >= 2
                        && matches!(self.calls[n - 1], Move::Pass)
                        && matches!(self.calls[n - 2], Move::Pass)
                } else {
                    matches!(self.calls[n - 1], Move::Pass)
                };
                if ended {
                    self.finish_auction();
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        let m = self.results[0].unwrap_or(0) - self.results[1].unwrap_or(0);
        Some(match m.cmp(&0) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        // scorul deja încheiat (din punctul de vedere al lui p)
        let mut s = 0i32;
        let sgn_ns = |board: usize| -> i32 {
            // proprietarul N–S la tabla `board`
            let o = if board == 0 { Player::One } else { Player::Two };
            if o == p {
                1
            } else {
                -1
            }
        };
        for b in 0..2usize {
            if let Some(r) = self.results[b] {
                s += sgn_ns(b) * r;
            }
        }
        match self.phase {
            Phase::Auction => {
                let myside = if self.ns_owner() == p { 0u8 } else { 1u8 };
                let evalc = |level: u8, strain: u8, side: u8, dbl: u8| -> f32 {
                    let need = level as f32 + 6.0;
                    let all_hc: i32 = {
                        let st = self.my_seats(p);
                        hcp(&self.hands[st[0]]) + hcp(&self.hands[st[1]])
                    };
                    if side == myside {
                        let m = self.est(p, strain) - need;
                        let pr = prob(m);
                        let made = contract_score(level, strain, dbl, level + 6) as f32;
                        let down = -contract_score(level, strain, dbl, level + 4) as f32;
                        pr * made - (1.0 - pr) * (down * 0.75 + 0.0)
                    } else {
                        let opp_hc = (40 - all_hc) as f32;
                        let e = 6.0 + (opp_hc - 20.0) * 0.3;
                        let pr = prob(e - need);
                        let made = contract_score(level, strain, dbl, level + 6) as f32;
                        let down = -contract_score(level, strain, dbl, level + 4) as f32;
                        -(pr * made - (1.0 - pr) * (down * 0.75))
                    }
                };
                if let Some((l, st, side)) = self.last_bid() {
                    s += evalc(l, st, side, self.dbl_state()) as i32;
                }
            }
            Phase::Play => {
                let myside = if self.ns_owner() == p { 0u8 } else { 1u8 };
                let diff =
                    self.tricks[myside as usize] as i32 - self.tricks[1 - myside as usize] as i32;
                s += 100 * diff;
                let seats = self.my_seats(p);
                for &st in &seats {
                    for cc in &self.hands[st] {
                        s += (cc.rank as i32 - 6) / 2;
                        if Some(cc.suit) == self.trump() {
                            s += 3;
                        }
                    }
                }
                if let Some(&(seat, cc)) = self.trick.first() {
                    let mut best = (seat, cc);
                    let tr = self.trump();
                    for &(sx, cx) in &self.trick[1..] {
                        let beats = (Some(cx.suit) == tr && Some(best.1.suit) != tr)
                            || (cx.suit == best.1.suit && cx.rank > best.1.rank);
                        if beats {
                            best = (sx, cx);
                        }
                    }
                    let mine = best.0 % 2 == if myside == 0 { 0 } else { 1 };
                    let v = 30 + best.1.rank as i32;
                    s += if mine { v } else { -v };
                }
            }
            Phase::Over => {}
        }
        s
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Bridge duplicat — tabla {}/2 (J1 conduce {})\n",
            self.board.min(1) + 1,
            if self.ns_owner() == Player::One {
                "N–S"
            } else {
                "E–V"
            }
        );
        if let (Some(a), None) = (self.results[0], self.results[1]) {
            s.push_str(&format!("Tabla 1: N–S {:+}\n", a));
        }
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        if self.phase == Phase::Over {
            s.push_str(&format!(
                "Tabla 1: N–S {:+} | Tabla 2: N–S {:+} | marja J1: {:+}\nJoc încheiat.\n",
                self.results[0].unwrap_or(0),
                self.results[1].unwrap_or(0),
                self.results[0].unwrap_or(0) - self.results[1].unwrap_or(0)
            ));
            return s;
        }
        if !self.calls.is_empty() || self.phase == Phase::Auction {
            let names: Vec<String> = self
                .calls
                .iter()
                .enumerate()
                .map(|(i, c)| {
                    format!(
                        "{}:{}",
                        if i % 2 == 0 { "N–S" } else { "E–V" },
                        self.format_move(c)
                    )
                })
                .collect();
            s.push_str(&format!(
                "Licitație: {}\n",
                if names.is_empty() {
                    "—".into()
                } else {
                    names.join(" ")
                }
            ));
        }
        if let Some(c) = self.contract {
            s.push_str(&format!(
                "Contract: {}{}{} ({}), declarant {} | levate: N–S {}, E–V {}\n",
                c.level,
                STRAIN[c.strain as usize],
                ["", " contrat", " recontrat"][c.dbl as usize],
                if c.side == 0 { "N–S" } else { "E–V" },
                SEAT[self.declarer()],
                self.tricks[0],
                self.tricks[1]
            ));
        }
        let me = self.viewer.who(self.to_move());
        let mine = self.my_seats(me);
        for &st in &mine {
            s.push_str(&format!(
                "{} (tu): {}\n",
                SEAT[st],
                join_cards(&self.hands[st])
            ));
        }
        if self.dummy_open && self.owner_of_seat(self.dummy()) != me {
            s.push_str(&format!(
                "{} (mâna moartă): {}\n",
                SEAT[self.dummy()],
                join_cards(&self.hands[self.dummy()])
            ));
        }
        if !self.trick.is_empty() {
            let t: Vec<String> = self
                .trick
                .iter()
                .map(|&(st, c)| format!("{}:{}", SEAT[st], c.text()))
                .collect();
            s.push_str(&format!("Pe masă: {}\n", t.join(" ")));
        }
        let tm = self.to_move();
        if self.phase == Phase::Play {
            s.push_str(&format!(
                "La rând: {} (J{})\n",
                SEAT[self.turn],
                tm.idx() + 1
            ));
        } else {
            s.push_str(&format!("La mutare: Jucătorul {}\n", tm.idx() + 1));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Pass => "pass".into(),
            Move::Double => "x".into(),
            Move::Redouble => "xx".into(),
            Move::Bid(l, s) => format!("{}{}", l, STRAIN[*s as usize]),
            Move::Play(c) => c.text(),
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

    #[test]
    fn viewer_controls_whose_hands_are_shown() {
        let mut g = Bridge::with_seed(3);
        Game::set_viewer(&mut g, None);
        // the mover changes during the auction; the render follows it
        for _ in 0..3 {
            let tm = g.to_move();
            let r = Game::render(&g);
            let mine = g.my_seats(tm);
            let theirs = g.my_seats(tm.other());
            for st in mine {
                assert!(r.contains(&join_cards(&g.hands[st])), "{r}");
            }
            for st in theirs {
                assert!(!r.contains(&join_cards(&g.hands[st])), "{r}");
            }
            g.apply(&Move::Pass);
        }
        for p in [Player::One, Player::Two] {
            Game::set_viewer(&mut g, Some(p));
            let r = Game::render(&g);
            for st in g.my_seats(p) {
                assert!(r.contains(&join_cards(&g.hands[st])), "{r}");
            }
            for st in g.my_seats(p.other()) {
                assert!(!r.contains(&join_cards(&g.hands[st])), "{r}");
            }
        }
    }

    #[test]
    fn scoring_table() {
        // 3NT făcut cu 9 levate: 100 + 300 = 400
        assert_eq!(contract_score(3, 4, 0, 9), 400);
        // 4♠ cu 10: 120 + 300
        assert_eq!(contract_score(4, 3, 0, 10), 420);
        // 2♥ cu 8: 60 + 50
        assert_eq!(contract_score(2, 2, 0, 8), 110);
        // 1♣ cu 8 (o supralevată): 20 + 50 + 20
        assert_eq!(contract_score(1, 0, 0, 8), 90);
        // 6♠ cu 12: 180 + 300 + 500
        assert_eq!(contract_score(6, 3, 0, 12), 980);
        // 7NT: 40+30*6 = 220 + 300 + 1000
        assert_eq!(contract_score(7, 4, 0, 13), 1520);
        // 4♠ contrat, făcut: 120*2=240 + 300 + 50
        assert_eq!(contract_score(4, 3, 1, 10), 590);
        // căderi: 3NT -2 fără contra = -100; contrat: -100 -200 = -300; recontrat: -600
        assert_eq!(contract_score(3, 4, 0, 7), -100);
        assert_eq!(contract_score(3, 4, 1, 7), -300);
        assert_eq!(contract_score(3, 4, 2, 7), -600);
        // contrat cădere 4: 100+200+200+300
        assert_eq!(contract_score(4, 3, 1, 6), -800);
    }

    #[test]
    fn deal_is_a_partition() {
        let g = Bridge::with_seed(3);
        let mut all: Vec<Card> = g.deal.iter().flatten().copied().collect();
        assert_eq!(all.len(), 52);
        all.sort_by_key(|c| (c.suit, c.rank));
        all.dedup();
        assert_eq!(all.len(), 52);
    }

    #[test]
    fn auction_rules() {
        let mut g = Bridge::with_seed(1);
        assert_eq!(g.to_move(), Player::One);
        assert!(!g.legal_moves().contains(&Move::Double));
        g.apply(&Move::Bid(1, 3));
        assert_eq!(g.to_move(), Player::Two);
        let l = g.legal_moves();
        assert!(l.contains(&Move::Double));
        assert!(!l.contains(&Move::Bid(1, 2)));
        assert!(l.contains(&Move::Bid(1, 4)));
        g.apply(&Move::Double);
        assert_eq!(g.to_move(), Player::One);
        assert!(g.legal_moves().contains(&Move::Redouble));
        g.apply(&Move::Pass);
        assert_eq!(g.phase, Phase::Play);
        let c = g.contract.unwrap();
        assert_eq!((c.level, c.strain, c.dbl, c.side), (1, 3, 1, 0));
        // declarant Sud, conduce Vest (J2 la tabla 1)
        assert_eq!(g.turn, 3);
        assert_eq!(g.to_move(), Player::Two);
    }

    #[test]
    fn passed_out_scores_zero_and_next_board_swaps_roles() {
        let mut g = Bridge::with_seed(1);
        g.apply(&Move::Pass);
        g.apply(&Move::Pass);
        assert_eq!(g.results[0], Some(0));
        assert_eq!(g.board, 1);
        assert_eq!(g.to_move(), Player::Two); // N–S aparține lui J2 la tabla 2
        assert_eq!(g.hands, g.deal);
    }

    #[test]
    fn full_board_play_all_tricks() {
        let mut g = Bridge::with_seed(5);
        g.apply(&Move::Bid(1, 4));
        g.apply(&Move::Pass);
        let mut n = 0;
        while g.board == 0 && g.phase == Phase::Play {
            let l = g.legal_moves();
            assert!(!l.is_empty());
            g.apply(&l[0]);
            n += 1;
            assert!(n <= 52);
        }
        assert_eq!(n, 52);
        assert!(g.results[0].is_some());
    }

    #[test]
    fn dummy_visible_only_after_lead() {
        let mut g = Bridge::with_seed(5);
        // J2 declară 1♣ la tabla 1 (E–V): licitează J1 pass, J2 1c, J1 pass
        g.apply(&Move::Pass);
        g.apply(&Move::Bid(1, 0));
        g.apply(&Move::Pass);
        assert_eq!(g.phase, Phase::Play);
        assert!(!g.render().contains("mâna moartă"));
        let m = g.legal_moves()[0];
        g.apply(&m);
        assert!(g.render().contains("mâna moartă"));
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=10u64 {
            let mut g = Bridge::with_seed(seed);
            let mut rng = Rng::new(seed + 21);
            let mut n = 0;
            while g.outcome().is_none() && n < 5000 {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed}");
        }
    }
}

#[cfg(test)]
mod a3_audit {
    use super::*;

    #[test]
    fn ai_ignores_the_opponent_hands_at_every_level() {
        crate::p3_comun::ai_blind_check(
            "bridge",
            Bridge::with_seed,
            |g, me, s| {
                let seats = g.my_seats(me.other());
                let dummy = match (g.dummy_open, g.contract) {
                    (true, Some(c)) => Some(if c.side == 0 { 0usize } else { 1 }),
                    _ => None,
                };
                let hidden: Vec<usize> = seats
                    .iter()
                    .copied()
                    .filter(|&x| Some(x) != dummy)
                    .collect();
                let sizes: Vec<usize> = hidden.iter().map(|&x| g.hands[x].len()).collect();
                let mut pool: Vec<Card> = hidden.iter().flat_map(|&x| g.hands[x].clone()).collect();
                crate::p3_comun::shuffle_with(&mut pool, s);
                for (k, &x) in hidden.iter().enumerate() {
                    g.hands[x] = pool.drain(..sizes[k]).collect();
                    sort_hand(&mut g.hands[x]);
                }
            },
            6,
        );
    }
}
