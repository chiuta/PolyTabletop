//! Cassino (joc italian de capturi, 52 de cărți), 2 jucători, în varianta fără „construiri”.
//!
//! * 4 cărți pe masă, 4 în mână fiecăruia; când mâinile se golesc se împart încă 4 (6 ture pe
//!   împărțire, 52 de cărți). Valori: A = 1, 2–10 cât scriu; J, Q, K = 11, 12, 13.
//! * La rând pui o carte din mână: o **lași** pe masă (`5h`) sau **capturezi** (`5h x 2c 3d`):
//!   cartea ta ia orice grupuri de cărți de pe masă, fiecare grup fiind o carte de aceeași
//!   valoare ori un set cu suma egală cu valoarea cărții tale (doar pentru A–10; J, Q, K iau doar
//!   cărți identice). Cartea jucată merge și ea în grămada ta.
//! * Măturare (masa rămâne goală după captură) = 1 punct. La finalul împărțirii, cine a capturat
//!   ultimul ia ce a rămas pe masă (fără punct de măturare).
//! * Puncte pe împărțire: cele mai multe cărți 3 (egal: 0), cele mai multe pici 1, 10♦ „Cassino
//!   mare” 2, 2♠ „Cassino mic” 1, fiecare as 1, fiecare măturare 1. Se joacă împărțiri până cineva
//!   are cel puțin 11 puncte (cel mai mare scor câștigă; egal = încă o împărțire; maxim 8 împărțiri).
//! * Informație ascunsă: AI-ul evaluează doar grămezile capturate, masa și scorurile (public);
//!   nu citește mâna adversarului și nici stocul.
//!
//! Reguli: Cassino (domeniu public). Simplificare: fără construiri (builds) și fără dubla valoare
//! a asului.

#![allow(clippy::needless_range_loop)]

use crate::cards::{full_deck, handover_banner, join_cards, pl, shuffle, Card, Viewer};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const TARGET: u8 = 11;
pub const MAX_DEALS: u8 = 8;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Move {
    pub card: Card,
    /// Indici în masă (crescători); gol = lași cartea pe masă.
    pub take: Vec<usize>,
}

fn value(c: Card) -> i32 {
    if c.rank == 14 {
        1
    } else {
        c.rank as i32
    }
}

fn is_numeric(c: Card) -> bool {
    c.rank <= 10 || c.rank == 14
}

/// Poate fi împărțit `vals` în grupuri, fiecare cu suma exact `t`?
fn partitions(vals: &[i32], t: i32) -> bool {
    if vals.is_empty() {
        return true;
    }
    let x = vals[0];
    if x > t {
        return false;
    }
    let rest = &vals[1..];
    let n = rest.len();
    for mask in 0u32..(1u32 << n) {
        let mut sum = x;
        for (i, &v) in rest.iter().enumerate() {
            if mask & (1 << i) != 0 {
                sum += v;
            }
        }
        if sum != t {
            continue;
        }
        let left: Vec<i32> = rest
            .iter()
            .enumerate()
            .filter(|(i, _)| mask & (1 << i) == 0)
            .map(|(_, &v)| v)
            .collect();
        if partitions(&left, t) {
            return true;
        }
    }
    false
}

fn can_capture_set(card: Card, set: &[Card]) -> bool {
    if set.is_empty() {
        return false;
    }
    if !is_numeric(card) {
        return set.iter().all(|c| c.rank == card.rank);
    }
    if !set.iter().all(|&c| is_numeric(c)) {
        return false;
    }
    let t = value(card);
    let mut vals: Vec<i32> = set.iter().map(|&c| value(c)).collect();
    if vals.iter().sum::<i32>() % t != 0 {
        return false;
    }
    vals.sort_unstable_by(|a, b| b.cmp(a));
    partitions(&vals, t)
}

#[derive(Clone)]
pub struct Casino {
    deck: Vec<Card>,
    hands: [Vec<Card>; 2],
    table: Vec<Card>,
    pile: [Vec<Card>; 2],
    sweeps: [u8; 2],
    score: [u8; 2],
    last_capture: Option<Player>,
    side: Player,
    first: Player,
    deals: u8,
    rng: Rng,
    over: bool,
    log: String,
    viewer: Viewer,
    handover: bool,
}

impl Default for Casino {
    fn default() -> Self {
        Self::new()
    }
}

fn card_points(c: Card) -> i32 {
    let mut p = 0;
    if c.rank == 10 && c.suit == 1 {
        p += 8;
    }
    if c.rank == 2 && c.suit == 2 {
        p += 4;
    }
    if c.rank == 14 {
        p += 4;
    }
    if c.suit == 2 {
        p += 2;
    }
    p + 1
}

impl Casino {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Casino {
            deck: Vec::new(),
            hands: [Vec::new(), Vec::new()],
            table: Vec::new(),
            pile: [Vec::new(), Vec::new()],
            sweeps: [0, 0],
            score: [0, 0],
            last_capture: None,
            side: Player::Two,
            first: Player::One,
            deals: 0,
            rng: Rng::new(seed),
            over: false,
            log: String::new(),
            viewer: Viewer::Fixed(Player::One),
            handover: false,
        };
        g.new_deal();
        g
    }

    pub fn scores(&self) -> (u8, u8) {
        (self.score[0], self.score[1])
    }

    fn new_deal(&mut self) {
        let mut d = full_deck(2);
        shuffle(&mut d, &mut self.rng);
        self.deck = d;
        self.hands = [Vec::new(), Vec::new()];
        self.pile = [Vec::new(), Vec::new()];
        self.sweeps = [0, 0];
        self.last_capture = None;
        self.table = (0..4).map(|_| self.deck.pop().unwrap()).collect();
        self.deal_hands();
        self.side = self.first;
    }

    fn deal_hands(&mut self) {
        for _ in 0..4 {
            for p in 0..2 {
                if let Some(c) = self.deck.pop() {
                    self.hands[p].push(c);
                }
            }
        }
    }

    /// Puncte din grămezi pentru fiecare jucător (fără egalități la cărți/pici).
    pub fn deal_points(&self) -> [u8; 2] {
        let mut pts = [0u8; 2];
        let (a, b) = (self.pile[0].len(), self.pile[1].len());
        if a > b {
            pts[0] += 3;
        } else if b > a {
            pts[1] += 3;
        }
        let sp = |p: usize| self.pile[p].iter().filter(|c| c.suit == 2).count();
        if sp(0) > sp(1) {
            pts[0] += 1;
        } else if sp(1) > sp(0) {
            pts[1] += 1;
        }
        for p in 0..2 {
            for c in &self.pile[p] {
                if c.rank == 10 && c.suit == 1 {
                    pts[p] += 2;
                }
                if c.rank == 2 && c.suit == 2 {
                    pts[p] += 1;
                }
                if c.rank == 14 {
                    pts[p] += 1;
                }
            }
            pts[p] += self.sweeps[p];
        }
        pts
    }

    fn end_deal(&mut self) {
        if let Some(w) = self.last_capture {
            let t = std::mem::take(&mut self.table);
            self.pile[w.idx()].extend(t);
        } else {
            self.table.clear();
        }
        let pts = self.deal_points();
        for p in 0..2 {
            self.score[p] += pts[p];
        }
        self.deals += 1;
        self.log = format!(
            "Împărțirea {}: J1 +{}, J2 +{} (cărți {}–{}).",
            self.deals,
            pts[0],
            pts[1],
            self.pile[0].len(),
            self.pile[1].len()
        );
        let done = (self.score.iter().any(|&s| s >= TARGET) && self.score[0] != self.score[1])
            || self.deals >= MAX_DEALS;
        if done {
            self.over = true;
        } else {
            self.first = self.first.other();
            self.new_deal();
        }
    }
}

impl Game for Casino {
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
        let n = self.table.len().min(12);
        let mut out: Vec<Move> = Vec::new();
        let mut seen: Vec<Card> = Vec::new();
        for &card in &self.hands[self.side.idx()] {
            if seen.contains(&card) {
                continue;
            }
            seen.push(card);
            out.push(Move {
                card,
                take: Vec::new(),
            });
            for mask in 1u32..(1u32 << n) {
                let idx: Vec<usize> = (0..n).filter(|i| mask & (1 << i) != 0).collect();
                let set: Vec<Card> = idx.iter().map(|&i| self.table[i]).collect();
                if can_capture_set(card, &set) {
                    out.push(Move { card, take: idx });
                }
            }
        }
        out
    }

    fn apply(&mut self, m: &Move) {
        self.handover = false;
        let me = self.side;
        let mi = me.idx();
        let pos = self.hands[mi]
            .iter()
            .position(|&c| c == m.card)
            .expect("card in hand");
        self.hands[mi].remove(pos);
        if m.take.is_empty() {
            self.table.push(m.card);
            self.log = format!("J{} lasă {}.", mi + 1, m.card.text());
        } else {
            let mut taken = Vec::new();
            for &i in m.take.iter().rev() {
                taken.push(self.table.remove(i));
            }
            self.pile[mi].extend(taken);
            self.pile[mi].push(m.card);
            self.last_capture = Some(me);
            if self.table.is_empty() {
                self.sweeps[mi] += 1;
                self.log = format!("J{} ia cu {} și mătură masa!", mi + 1, m.card.text());
            } else {
                self.log = format!("J{} ia cu {}.", mi + 1, m.card.text());
            }
        }
        self.side = me.other();
        self.handover = true;
        if self.hands[0].is_empty() && self.hands[1].is_empty() {
            if self.deck.is_empty() {
                let log = std::mem::take(&mut self.log);
                self.end_deal();
                self.log = format!("{log} {}", self.log);
            } else {
                self.deal_hands();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            return None;
        }
        Some(match self.score[0].cmp(&self.score[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    /// Doar informație publică: scoruri, grămezi, mături și masa.
    fn evaluate(&self, p: Player) -> i32 {
        let q = p.other();
        let (pi, qi) = (p.idx(), q.idx());
        let mut s = 100 * (self.score[pi] as i32 - self.score[qi] as i32);
        for c in &self.pile[pi] {
            s += card_points(*c);
        }
        for c in &self.pile[qi] {
            s -= card_points(*c);
        }
        s += 6 * (self.sweeps[pi] as i32 - self.sweeps[qi] as i32);
        if self.side != p {
            // cărțile rămase pe masă sunt expuse adversarului
            for c in &self.table {
                s -= card_points(*c) / 3;
            }
        }
        s
    }

    fn render(&self) -> String {
        let mut s = String::new();
        if self.handover && !self.over {
            if let Viewer::HotSeat = self.viewer {
                s.push_str(&handover_banner(self.side));
            }
        }
        s.push_str(&format!(
            "Cassino — împărțirea {} | puncte: J1 {}, J2 {} (la {})\n",
            (self.deals + 1).min(MAX_DEALS),
            self.score[0],
            self.score[1],
            TARGET
        ));
        if !self.log.is_empty() {
            s.push_str(self.log.trim());
            s.push('\n');
        }
        if self.over {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        let tbl: Vec<String> = self
            .table
            .iter()
            .enumerate()
            .map(|(i, c)| format!("{}:{}", i + 1, c.text()))
            .collect();
        s.push_str(&format!(
            "Masa: {}\n",
            if tbl.is_empty() {
                "—".to_string()
            } else {
                tbl.join(" ")
            }
        ));
        s.push_str(&format!(
            "Capturate: J1 {} cărți ({} mături), J2 {} cărți ({} mături) | stoc {}\n",
            self.pile[0].len(),
            self.sweeps[0],
            self.pile[1].len(),
            self.sweeps[1],
            self.deck.len()
        ));
        let v = self.viewer.who(self.side);
        s.push_str(&format!(
            "Mâna Jucătorului {}: {}\n",
            pl(v),
            join_cards(&self.hands[v.idx()])
        ));
        s.push_str(&format!(
            "Jucătorul {}: {} cărți\n",
            pl(v.other()),
            self.hands[v.other().idx()].len()
        ));
        s.push_str(&format!("La mutare: Jucătorul {}\n", pl(self.side)));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        if m.take.is_empty() {
            m.card.text()
        } else {
            format!(
                "{} x {}",
                m.card.text(),
                m.take
                    .iter()
                    .map(|&i| self.table[i].text())
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(rank: u8, suit: u8) -> Card {
        Card { rank, suit }
    }

    fn setup(hand: Vec<Card>, table: Vec<Card>) -> Casino {
        let mut g = Casino::with_seed(1);
        g.hands = [hand, vec![c(13, 0)]];
        g.table = table;
        g.deck.clear();
        g.pile = [Vec::new(), Vec::new()];
        g.sweeps = [0, 0];
        g.side = Player::One;
        g.last_capture = None;
        g
    }

    #[test]
    fn capture_by_match_and_sum() {
        assert!(can_capture_set(c(7, 0), &[c(7, 1)]));
        assert!(can_capture_set(c(7, 0), &[c(3, 1), c(4, 2)]));
        assert!(can_capture_set(c(7, 0), &[c(3, 1), c(4, 2), c(7, 3)]));
        assert!(can_capture_set(
            c(5, 0),
            &[c(2, 1), c(3, 2), c(4, 0), c(14, 2)]
        ));
        assert!(!can_capture_set(c(7, 0), &[c(3, 1), c(5, 2)]));
        assert!(!can_capture_set(c(7, 0), &[c(3, 1), c(4, 2), c(2, 3)]));
        assert!(can_capture_set(c(12, 0), &[c(12, 1)]));
        assert!(!can_capture_set(c(12, 0), &[c(6, 1), c(6, 2)]));
        assert!(!can_capture_set(c(10, 0), &[c(13, 1)]));
        assert!(can_capture_set(c(14, 0), &[c(14, 1)]));
        assert!(!can_capture_set(c(14, 0), &[c(14, 1), c(2, 1)]));
    }

    #[test]
    fn partition_requires_exact_groups() {
        // 8 = 3+5 and 2+6 -> {3,5,2,6} captured by an 8
        assert!(can_capture_set(
            c(8, 0),
            &[c(3, 0), c(5, 1), c(2, 2), c(6, 3)]
        ));
        // 8: {3,3,2} sums 8 but {3,3,2,6,5}? 19 not multiple
        assert!(!can_capture_set(c(8, 0), &[c(4, 0), c(4, 1), c(4, 2)]));
        assert!(can_capture_set(
            c(6, 0),
            &[c(2, 0), c(2, 1), c(2, 2), c(6, 3)]
        ));
    }

    #[test]
    fn sweep_and_last_capture_cards() {
        let mut g = setup(vec![c(7, 0), c(9, 1)], vec![c(3, 1), c(4, 2)]);
        let m = g
            .legal_moves()
            .into_iter()
            .find(|m| m.card == c(7, 0) && m.take == vec![0, 1])
            .expect("capture");
        g.apply(&m);
        assert_eq!(g.sweeps[0], 1);
        assert_eq!(g.pile[0].len(), 3);
        assert!(g.table.is_empty());
    }

    #[test]
    fn leftover_goes_to_last_capturer_and_scoring() {
        let mut g = setup(vec![c(9, 1)], vec![c(2, 1), c(5, 3)]);
        g.hands = [vec![c(9, 1)], vec![c(13, 0)]];
        g.last_capture = Some(Player::Two);
        g.apply(&Move {
            card: c(9, 1),
            take: vec![],
        });
        // J2 plays the king, hands and deck empty -> deal ends, table to J2
        g.apply(&Move {
            card: c(13, 0),
            take: vec![],
        });
        assert_eq!(g.deals, 1);
        // J2 got all 4 cards = 3 points for cards
        assert_eq!(g.score[1], 3);
        assert_eq!(g.score[0], 0);
    }

    #[test]
    fn deal_points_total() {
        let mut g = Casino::with_seed(2);
        g.pile[0] = vec![c(10, 1), c(2, 2), c(14, 0)];
        g.pile[1] = vec![c(14, 1), c(5, 2)];
        g.sweeps = [1, 0];
        // J1: cards 3 + spades(2♠:1 vs 5♠:1 tie 0) + big 2 + little 1 + ace 1 + sweep 1
        assert_eq!(g.deal_points(), [8, 1]);
    }

    #[test]
    fn move_text_round_trips() {
        let mut g = Casino::with_seed(5);
        let mv = g.legal_moves();
        for m in &mv {
            let t = Game::format_move(&g, m);
            let mut h = g.clone();
            assert!(polytabletop_core::GameObj::play(&mut h, &t).is_ok(), "{t}");
        }
        // texts are unique
        let mut texts: Vec<String> = mv.iter().map(|m| Game::format_move(&g, m)).collect();
        texts.sort();
        let n = texts.len();
        texts.dedup();
        assert_eq!(n, texts.len());
        let _ = &mut g;
    }

    #[test]
    fn render_hides_opponent_hand() {
        let g = Casino::with_seed(7);
        let r = Game::render(&g);
        for x in &g.hands[1] {
            // the table could contain... a card can't be both
            assert!(!r.split_whitespace().any(|t| t == x.text()), "{r}");
        }
        let mut h = g.clone();
        Game::set_viewer(&mut h, None);
        let m = h.legal_moves()[0].clone();
        h.apply(&m);
        let r = Game::render(&h);
        assert!(r.contains("Predă ecranul"));
        for x in &h.hands[0] {
            assert!(!r.split_whitespace().any(|t| t == x.text()), "{r}");
        }
    }

    #[test]
    fn ai_ignores_hidden_cards() {
        for seed in 1..40u64 {
            let mut g = Casino::with_seed(seed);
            let mut rng = Rng::new(seed);
            for _ in 0..(seed as usize % 10) {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
            }
            if g.over {
                continue;
            }
            let me = g.side.idx();
            let mut h = g.clone();
            let mut pool = h.deck.clone();
            pool.extend(&h.hands[1 - me]);
            shuffle(&mut pool, &mut rng);
            let n = h.hands[1 - me].len();
            h.hands[1 - me] = pool.drain(..n).collect();
            h.deck = pool;
            assert_eq!(
                polytabletop_core::ai::best_move(&g, 5),
                polytabletop_core::ai::best_move(&h, 5),
                "seed {seed}"
            );
        }
    }

    #[test]
    fn random_games_finish_and_conserve_cards() {
        for seed in 1..40u64 {
            let mut g = Casino::with_seed(seed);
            let mut rng = Rng::new(seed ^ 11);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                let total = g.deck.len()
                    + g.table.len()
                    + g.hands[0].len()
                    + g.hands[1].len()
                    + g.pile[0].len()
                    + g.pile[1].len();
                assert_eq!(total, 52);
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
                assert!(n < 2000, "seed {seed}");
            }
        }
    }
}
