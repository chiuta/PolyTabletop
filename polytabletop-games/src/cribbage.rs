//! Cribbage (2 jucători, 6 cărți), cu tablă de colac (121 de găuri), din regulile publice.
//!
//! * Se dau 6 cărți fiecăruia; fiecare dă 2 în „crib” (al dealerului). Non-dealerul taie
//!   cartea de start; un Valet de start dă dealerului 2 puncte („his heels”).
//! * Pegging: cărțile se joacă pe rând, suma curentă nu depășește 31. Se punctează 15 (2),
//!   31 (2), perechi (2/6/12), serii de 3+ (oricare ordine), „go”/ultima carte (1).
//! * Numărarea: 15-uri (2 fiecare), perechi, serii, culoare (4 în mână, 5 cu start; cribul
//!   doar 5), valetul de aceeași culoare cu startul (1). Non-dealer, dealer, crib.
//! * Primul la 121 câștigă pe loc, chiar în mijlocul numărării.
//!
//! Mutări: `d 5h 6s` (dai în crib), `taie`, `5h` (joci), `go`, `numara`, `imparte`.
//! Informația ascunsă: render() arată mâna Jucătorului 1; cribul și mâna adversarului apar
//! doar când se numără. AI-ul (AlphaBeta(0)) citește doar propria mână și informațiile publice.

use crate::cards::{full_deck, join_cards, shuffle, Card, Viewer};
use crate::cardsb_util::sort_hand;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Deal,
    Discard,
    Cut,
    Peg,
    ShowPone,
    ShowDealer,
    ShowCrib,
    Over,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Deal,
    Discard(Card, Card),
    Cut,
    Play(Card),
    Go,
    Count,
}

#[derive(Clone)]
pub struct Cribbage {
    rng: Rng,
    deck: Vec<Card>,
    hands: [Vec<Card>; 2],
    kept: [Vec<Card>; 2],
    crib: Vec<Card>,
    starter: Option<Card>,
    dealer: Player,
    phase: Phase,
    side: Player,
    pegs: [u32; 2],
    back: [u32; 2],
    // pegging
    pile: Vec<Card>,
    count: u8,
    last: Option<Player>,
    go_by: Option<Player>,
    log: String,
    viewer: Viewer,
}

impl Default for Cribbage {
    fn default() -> Self {
        Self::new()
    }
}

fn val(c: Card) -> u8 {
    match c.rank {
        11..=13 => 10,
        14 => 1,
        r => r,
    }
}
fn run_rank(c: Card) -> u8 {
    if c.rank == 14 {
        1
    } else {
        c.rank
    }
}

/// Puncte de pegging pentru ultima carte pusă (`pile` conține deja cartea).
fn peg_score(pile: &[Card], count: u8) -> u32 {
    let mut s = 0;
    if count == 15 || count == 31 {
        s += 2;
    }
    let n = pile.len();
    let mut same = 1;
    while same < n && pile[n - 1 - same].rank == pile[n - 1].rank {
        same += 1;
    }
    s += match same {
        2 => 2,
        3 => 6,
        4 => 12,
        _ => 0,
    };
    let mut best = 0;
    for len in (3..=n).rev() {
        let mut rs: Vec<u8> = pile[n - len..].iter().map(|&c| run_rank(c)).collect();
        rs.sort_unstable();
        if rs.windows(2).all(|w| w[1] == w[0] + 1) {
            best = len as u32;
            break;
        }
    }
    s + best
}

/// Valoarea unei mâini (4 cărți + start). `crib` = reguli de culoare ale cribului.
pub fn hand_score(hand: &[Card], starter: Option<Card>, crib: bool) -> u32 {
    let mut all: Vec<Card> = hand.to_vec();
    if let Some(s) = starter {
        all.push(s);
    }
    let n = all.len();
    let mut pts = 0;
    // 15-uri
    for mask in 1u32..(1 << n) {
        let mut sum = 0;
        for (i, &c) in all.iter().enumerate() {
            if mask & (1 << i) != 0 {
                sum += val(c) as u32;
            }
        }
        if sum == 15 {
            pts += 2;
        }
    }
    // perechi
    for i in 0..n {
        for j in i + 1..n {
            if all[i].rank == all[j].rank {
                pts += 2;
            }
        }
    }
    // serii (doar lungimea maximă)
    for len in (3..=n).rev() {
        let mut found = 0;
        for mask in 1u32..(1 << n) {
            if mask.count_ones() as usize != len {
                continue;
            }
            let mut rs: Vec<u8> = all
                .iter()
                .enumerate()
                .filter(|(i, _)| mask & (1 << i) != 0)
                .map(|(_, &c)| run_rank(c))
                .collect();
            rs.sort_unstable();
            if rs.windows(2).all(|w| w[1] == w[0] + 1) {
                found += 1;
            }
        }
        if found > 0 {
            pts += found * len as u32;
            break;
        }
    }
    // culoare
    if hand.len() == 4 && hand.iter().all(|c| c.suit == hand[0].suit) {
        match starter {
            Some(s) if s.suit == hand[0].suit => pts += 5,
            _ => {
                if !crib {
                    pts += 4;
                }
            }
        }
    }
    // nobs
    if let Some(s) = starter {
        if hand.iter().any(|c| c.rank == 11 && c.suit == s.suit) {
            pts += 1;
        }
    }
    pts
}

impl Cribbage {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Cribbage {
            rng: Rng::new(seed),
            deck: Vec::new(),
            hands: [Vec::new(), Vec::new()],
            kept: [Vec::new(), Vec::new()],
            crib: Vec::new(),
            starter: None,
            dealer: Player::Two,
            phase: Phase::Deal,
            side: Player::One,
            pegs: [0, 0],
            back: [0, 0],
            pile: Vec::new(),
            count: 0,
            last: None,
            go_by: None,
            log: String::new(),
            viewer: Viewer::Fixed(Player::One),
        };
        // prima împărțire e deja făcută (nu e nimic de „prevăzut”)
        g.deal();
        g
    }

    pub fn scores(&self) -> (u32, u32) {
        (self.pegs[0], self.pegs[1])
    }

    fn deal(&mut self) {
        let mut d = full_deck(2);
        shuffle(&mut d, &mut self.rng);
        let pone = self.dealer.other();
        self.hands = [Vec::new(), Vec::new()];
        for _ in 0..6 {
            self.hands[pone.idx()].push(d.pop().unwrap());
            self.hands[self.dealer.idx()].push(d.pop().unwrap());
        }
        sort_hand(&mut self.hands[0]);
        sort_hand(&mut self.hands[1]);
        self.deck = d;
        self.kept = [Vec::new(), Vec::new()];
        self.crib.clear();
        self.starter = None;
        self.pile.clear();
        self.count = 0;
        self.last = None;
        self.go_by = None;
        self.phase = Phase::Discard;
        self.side = pone;
    }

    fn add(&mut self, p: Player, pts: u32) {
        if pts == 0 || self.phase == Phase::Over {
            return;
        }
        self.back[p.idx()] = self.pegs[p.idx()];
        self.pegs[p.idx()] = (self.pegs[p.idx()] + pts).min(121);
        if self.pegs[p.idx()] >= 121 {
            self.phase = Phase::Over;
        }
    }

    fn can_play(&self, p: Player) -> bool {
        self.hands[p.idx()]
            .iter()
            .any(|&c| self.count + val(c) <= 31)
    }

    fn pegging_over(&self) -> bool {
        self.hands[0].is_empty() && self.hands[1].is_empty()
    }

    fn finish_pegging(&mut self) {
        // cărțile jucate revin în „kept” pentru numărare (le-am păstrat separat)
        self.phase = Phase::ShowPone;
        self.side = self.dealer.other();
    }

    /// Terminarea unei numărători (go sau 31): resetează și alege cine conduce.
    fn end_count(&mut self, award_last: bool) {
        if award_last {
            if let Some(l) = self.last {
                self.add(l, 1);
            }
        }
        self.pile.clear();
        self.count = 0;
        self.go_by = None;
        if self.phase == Phase::Over {
            return;
        }
        if self.pegging_over() {
            self.finish_pegging();
            return;
        }
        let l = self.last.unwrap_or(self.dealer.other());
        let next = l.other();
        self.side = if !self.hands[next.idx()].is_empty() {
            next
        } else {
            l
        };
    }
}

impl Game for Cribbage {
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
            Phase::Deal => vec![Move::Deal],
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
            Phase::Cut => vec![Move::Cut],
            Phase::Peg => {
                let h = &self.hands[self.side.idx()];
                let mut v: Vec<Move> = h
                    .iter()
                    .filter(|&&c| self.count + val(c) <= 31)
                    .map(|&c| Move::Play(c))
                    .collect();
                if v.is_empty() {
                    v.push(Move::Go);
                }
                v
            }
            Phase::ShowPone | Phase::ShowDealer | Phase::ShowCrib => vec![Move::Count],
        }
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        match *m {
            Move::Deal => self.deal(),
            Move::Discard(a, b) => {
                let p = self.side;
                for c in [a, b] {
                    let pos = self.hands[p.idx()].iter().position(|&x| x == c).unwrap();
                    self.hands[p.idx()].remove(pos);
                    self.crib.push(c);
                }
                if p == self.dealer {
                    self.kept = [self.hands[0].clone(), self.hands[1].clone()];
                    self.phase = Phase::Cut;
                    self.side = self.dealer.other();
                } else {
                    self.side = self.dealer;
                }
            }
            Move::Cut => {
                let s = self.deck.pop().unwrap();
                self.starter = Some(s);
                self.log = format!("Start: {}", s.text());
                if s.rank == 11 {
                    let d = self.dealer;
                    self.add(d, 2);
                    self.log.push_str(" (valet: 2 puncte pentru dealer)");
                }
                if self.phase != Phase::Over {
                    self.phase = Phase::Peg;
                    self.side = self.dealer.other();
                }
            }
            Move::Play(c) => {
                let p = self.side;
                let pos = self.hands[p.idx()].iter().position(|&x| x == c).unwrap();
                self.hands[p.idx()].remove(pos);
                self.pile.push(c);
                self.count += val(c);
                self.last = Some(p);
                let pts = peg_score(&self.pile, self.count);
                self.add(p, pts);
                if pts > 0 {
                    self.log = format!("{} la {}: +{}", c.text(), self.count, pts);
                }
                if self.phase == Phase::Over {
                    return;
                }
                if self.count == 31 {
                    self.end_count(false);
                    return;
                }
                if self.pegging_over() {
                    self.end_count(true);
                    return;
                }
                let o = p.other();
                match self.go_by {
                    Some(_) => {
                        if self.can_play(p) {
                            self.side = p;
                        } else {
                            self.end_count(true);
                        }
                    }
                    None => {
                        self.side = if !self.hands[o.idx()].is_empty() {
                            o
                        } else {
                            p
                        };
                    }
                }
            }
            Move::Go => {
                let q = self.side;
                let p = q.other();
                if self.go_by.is_none() && self.can_play(p) {
                    self.go_by = Some(q);
                    self.side = p;
                } else {
                    self.end_count(true);
                }
            }
            Move::Count => {
                let s = self.starter;
                match self.phase {
                    Phase::ShowPone => {
                        let p = self.dealer.other();
                        let pts = hand_score(&self.kept[p.idx()], s, false);
                        self.log = format!("Mâna J{}: {} puncte", p.idx() + 1, pts);
                        self.add(p, pts);
                        if self.phase != Phase::Over {
                            self.phase = Phase::ShowDealer;
                            self.side = self.dealer;
                        }
                    }
                    Phase::ShowDealer => {
                        let p = self.dealer;
                        let pts = hand_score(&self.kept[p.idx()], s, false);
                        self.log = format!("Mâna J{}: {} puncte", p.idx() + 1, pts);
                        self.add(p, pts);
                        if self.phase != Phase::Over {
                            self.phase = Phase::ShowCrib;
                        }
                    }
                    _ => {
                        let p = self.dealer;
                        let pts = hand_score(&self.crib, s, true);
                        self.log = format!("Cribul J{}: {} puncte", p.idx() + 1, pts);
                        self.add(p, pts);
                        if self.phase != Phase::Over {
                            self.dealer = self.dealer.other();
                            self.phase = Phase::Deal;
                            self.side = self.dealer.other();
                        }
                    }
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        Some(if self.pegs[0] >= 121 {
            Outcome::Win(Player::One)
        } else {
            Outcome::Win(Player::Two)
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let q = p.other();
        let mut s = 10 * (self.pegs[p.idx()] as i32 - self.pegs[q.idx()] as i32);
        match self.phase {
            Phase::Cut => {
                // mâna păstrată de p (4 cărți): valoare estimată
                s += hand_score(&self.kept[p.idx()], None, false) as i32 * 10 + 20;
                if self.dealer == p {
                    let n = self.crib.len();
                    s += crib_estimate(self.crib[n - 2], self.crib[n - 1]) * 8;
                }
            }
            Phase::Peg => {
                if self.count == 5 || self.count == 21 {
                    // cine a jucat ultimul lasă 5 / 21: o carte de 10 îi dă adversarului 15 / 31
                    if self.last == Some(p) {
                        s -= 15;
                    } else if self.last == Some(q) {
                        s += 15;
                    }
                }
            }
            Phase::Discard => {
                let h = &self.hands[p.idx()];
                if h.len() == 4 && self.crib.len() >= 2 {
                    // p tocmai a dat: apreciem mâna păstrată și cele 2 cărți din crib (ultimele date)
                    let sc = hand_score(h, None, false) as i32 * 10 + 20;
                    let n = self.crib.len();
                    let (a, b) = (self.crib[n - 2], self.crib[n - 1]);
                    let cv = crib_estimate(a, b);
                    let sign = if self.dealer == p { 1 } else { -1 };
                    s += sc + sign * cv * 8;
                }
            }
            _ => {}
        }
        s
    }

    fn render(&self) -> String {
        let mut out =
            String::from("Cribbage — tabla de colac (● = țăruș din față, ○ = din spate)\n");
        for p in [Player::One, Player::Two] {
            out.push_str(&format!(
                "J{} ({}): {}\n",
                p.idx() + 1,
                self.pegs[p.idx()],
                board_line(self.pegs[p.idx()], self.back[p.idx()])
            ));
        }
        if self.phase == Phase::Over {
            out.push_str("Joc încheiat.\n");
            return out;
        }
        out.push_str(&format!(
            "Dealer: J{}{}\n",
            self.dealer.idx() + 1,
            match self.starter {
                Some(s) => format!(" | Start: {}", s.text()),
                None => String::new(),
            }
        ));
        if !self.log.is_empty() {
            out.push_str(&format!("({})\n", self.log));
        }
        match self.phase {
            Phase::Peg => {
                out.push_str(&format!(
                    "Pegging — suma: {} | jucate: {}\n",
                    self.count,
                    join_cards(&self.pile)
                ));
                out.push_str(&format!(
                    "Cărți rămase: J1 {}, J2 {}\n",
                    self.hands[0].len(),
                    self.hands[1].len()
                ));
            }
            Phase::ShowPone | Phase::ShowDealer | Phase::ShowCrib => {
                let pone = self.dealer.other();
                out.push_str(&format!(
                    "Mâna J{}: {}\n",
                    pone.idx() + 1,
                    join_cards(&self.kept[pone.idx()])
                ));
                if self.phase != Phase::ShowPone {
                    out.push_str(&format!(
                        "Mâna J{}: {}\n",
                        self.dealer.idx() + 1,
                        join_cards(&self.kept[self.dealer.idx()])
                    ));
                }
                if self.phase == Phase::ShowCrib {
                    out.push_str(&format!("Crib: {}\n", join_cards(&self.crib)));
                }
            }
            _ => {}
        }
        if matches!(self.phase, Phase::Discard | Phase::Cut | Phase::Peg) {
            let v = self.viewer.who(self.side).idx();
            out.push_str(&format!(
                "Mâna ta (J{}): {}\n",
                v + 1,
                join_cards(&self.hands[v])
            ));
            if self.phase != Phase::Peg {
                out.push_str(&format!(
                    "Cărțile tale păstrate: {}\n",
                    join_cards(&self.kept[v])
                ));
            }
        }
        out.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        out
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Deal => "imparte".into(),
            Move::Discard(a, b) => format!("d {} {}", a.text(), b.text()),
            Move::Cut => "taie".into(),
            Move::Play(c) => c.text(),
            Move::Go => "go".into(),
            Move::Count => "numara".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

fn crib_estimate(a: Card, b: Card) -> i32 {
    let mut cv = 0;
    if a.rank == b.rank {
        cv += 2;
    }
    if val(a) + val(b) == 15 {
        cv += 2;
    }
    if a.rank == 5 || b.rank == 5 {
        cv += 1;
    }
    cv
}

fn board_line(front: u32, back: u32) -> String {
    let mut s = String::new();
    if front == 0 {
        s.push_str("[start] ");
    }
    for i in 1..=121u32 {
        s.push(if i == front {
            '●'
        } else if i == back && back > 0 {
            '○'
        } else {
            '·'
        });
        if i % 5 == 0 && i != 121 {
            s.push(' ');
        }
        if i == 40 || i == 80 {
            s.push_str("\n      ");
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(rank: u8, suit: u8) -> Card {
        Card { rank, suit }
    }

    #[test]
    fn viewer_controls_whose_hand_is_shown() {
        let mut g = Cribbage::with_seed(4);
        g.phase = Phase::Peg;
        g.hands[0] = vec![c(2, 0), c(3, 1), c(4, 2)];
        g.hands[1] = vec![c(9, 3), c(10, 0), c(12, 1)];
        g.kept = [g.hands[0].clone(), g.hands[1].clone()];
        g.pile.clear();
        let own = |r: &str| {
            r.lines()
                .find(|l| l.starts_with("Mâna ta"))
                .unwrap()
                .to_string()
        };
        Game::set_viewer(&mut g, None);
        for side in [Player::One, Player::Two] {
            g.side = side;
            let r = Game::render(&g);
            assert!(own(&r).ends_with(&join_cards(&g.hands[side.idx()])), "{r}");
            assert!(
                !r.contains(&join_cards(&g.hands[side.other().idx()])),
                "{r}"
            );
        }
        g.side = Player::One;
        Game::set_viewer(&mut g, Some(Player::Two));
        let r = Game::render(&g);
        assert!(own(&r).ends_with(&join_cards(&g.hands[1])), "{r}");
        assert!(!r.contains(&join_cards(&g.hands[0])), "{r}");
    }

    #[test]
    fn perfect_hand_29() {
        let h = [c(5, 0), c(5, 1), c(5, 3), c(11, 2)];
        assert_eq!(hand_score(&h, Some(c(5, 2)), false), 29);
    }

    #[test]
    fn hand_examples() {
        // 5 5 5 J + start 5 -> 29; 4-4-5-6 + 7 mixed
        let h = [c(4, 0), c(4, 1), c(5, 2), c(6, 3)];
        // 15s: 4+5+6 (x2)=4, plus 5+... 4+4+... no; pairs 2; runs 4-5-6 x2 = 6 -> 12 with start 9
        assert_eq!(hand_score(&h, Some(c(9, 0)), false), 4 + 2 + 6 + 2);
        // flush
        let f = [c(2, 0), c(4, 0), c(6, 0), c(8, 0)];
        assert_eq!(hand_score(&f, Some(c(13, 1)), false), 4);
        assert_eq!(hand_score(&f, Some(c(13, 0)), false), 5);
        assert_eq!(hand_score(&f, Some(c(13, 1)), true), 0);
    }

    #[test]
    fn pegging_scores() {
        // 15
        assert_eq!(peg_score(&[c(10, 0), c(5, 1)], 15), 2);
        // pereche
        assert_eq!(peg_score(&[c(7, 0), c(7, 1)], 14), 2);
        assert_eq!(peg_score(&[c(7, 0), c(7, 1), c(7, 2)], 21), 6);
        // serie 3 în ordine amestecată
        assert_eq!(peg_score(&[c(4, 0), c(6, 1), c(5, 2)], 15), 3 + 2);
    }

    #[test]
    fn deal_conserves_cards() {
        let g = Cribbage::with_seed(4);
        assert_eq!(g.hands[0].len(), 6);
        assert_eq!(g.hands[1].len(), 6);
        assert_eq!(g.deck.len(), 40);
    }

    #[test]
    fn discard_then_cut_then_peg() {
        let mut g = Cribbage::with_seed(2);
        for _ in 0..2 {
            let m = g.legal_moves()[0];
            g.apply(&m);
        }
        assert_eq!(g.crib.len(), 4);
        assert_eq!(g.phase, Phase::Cut);
        g.apply(&Move::Cut);
        assert!(g.starter.is_some() || g.phase == Phase::Over);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=8u64 {
            let mut g = Cribbage::with_seed(seed);
            let mut rng = Rng::new(seed + 100);
            let mut n = 0;
            while g.outcome().is_none() && n < 20000 {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                let m = mv[rng.below(mv.len())];
                g.apply(&m);
                n += 1;
                assert!(g.pegs[0] <= 121 && g.pegs[1] <= 121);
            }
            assert!(g.outcome().is_some());
        }
    }

    #[test]
    fn go_logic_last_card_point() {
        let mut g = Cribbage::with_seed(1);
        g.phase = Phase::Peg;
        g.hands = [vec![c(10, 0)], vec![c(10, 1)]];
        g.side = Player::One;
        g.count = 0;
        g.pile.clear();
        g.pegs = [0, 0];
        g.apply(&Move::Play(c(10, 0)));
        assert_eq!(g.side, Player::Two);
        g.apply(&Move::Play(c(10, 1)));
        // ambele mâini goale -> ultima carte (1) + pereche (2) pentru J2
        assert_eq!(g.pegs[1], 3);
        assert_eq!(g.phase, Phase::ShowPone);
    }
}

#[cfg(test)]
mod a1_blind {
    use super::*;
    use crate::ai_audit_a1::{audit, shuffle};

    #[test]
    fn ai_ignores_opponent_hand_deck_and_cut_at_all_levels() {
        audit("cribbage", Cribbage::with_seed, 1..=4, 3, 70, |g, salt| {
            let foe = g.side.other().idx();
            let mut pool: Vec<Card> = g.hands[foe].clone();
            pool.extend(g.deck.iter());
            shuffle(&mut pool, salt);
            let nh = g.hands[foe].len();
            g.hands[foe] = pool.drain(..nh).collect();
            g.deck = pool;
            if g.phase == Phase::Peg || g.phase == Phase::Cut {
                // cărțile păstrate de adversar (numărate abia la sfârșit) sunt ascunse
                let k = g.kept[foe].len();
                let mut pk: Vec<Card> = g.kept[foe].clone();
                shuffle(&mut pk, salt ^ 5);
                g.kept[foe] = pk.drain(..k).collect();
            }
            g.rng = Rng::new(salt ^ 3);
        });
    }
}
