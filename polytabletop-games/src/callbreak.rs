#![allow(
    clippy::needless_range_loop,
    clippy::manual_is_multiple_of,
    clippy::manual_range_contains,
    clippy::too_many_arguments,
    clippy::doc_lazy_continuation,
    clippy::unnecessary_unwrap,
    clippy::unnecessary_cast,
    clippy::type_complexity
)]
//! Call Break (Nepal / India) — joc de levate cu licitație exactă și pică ca atu, 2 jucători.
//!
//! Reguli folosite (din regulile publice ale jocului în patru, adaptate pentru doi):
//! * 52 de cărți, rang A (cel mai mare) … 2; **pica (♠) este mereu atu**. Fiecare primește 13
//!   cărți; celelalte 26 rămân nefolosite și ascunse (în patru jucători s-ar împărți toate).
//! * Licitația: fiecare spune (`licit N`, 1–13) câte levate se angajează să ia; cel care nu e
//!   dealer licitează primul și conduce prima levată; câștigătorul unei levate conduce următoarea.
//! * Joc: răspunzi la culoarea condusă și, dacă poți, **trebuie să dai o carte mai mare**;
//!   dacă n-ai culoarea, **trebuie să tai cu pică** dacă ai; altfel dai orice.
//! * Scor pe mână (în zecimi, afișat ca la hârtie): cine ia cel puțin cât a licitat primește
//!   licitația + 0,1 pentru fiecare levată în plus (ex.: 4 licitate, 6 luate = 4,2); cine ia mai
//!   puțin pierde licitația (−4). Meciul are 5 mâini; câștigă cine are scorul total mai mare
//!   (egalitate = remiză).
//! * Mutări: `licit 4`, apoi cărțile (`10h`, `As`).
//! * Informație ascunsă: vezi doar mâna ta și licitația adversarului. AI-ul își estimează
//!   numărul de levate din propria mână și joacă după levate rămase; la Hard/Expert folosește
//!   eșantionare Monte-Carlo a cărților necunoscute.

use crate::cards::Viewer;
use crate::cards52::{std_deck, std_rank, std_suit, std_symbol, std_text};
use crate::p2_util::{
    beats2, mc_samples, mc_value, noise, shuffle_vec, sig_of, sort_std, symbols_typed, Mc,
};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const ROUNDS: u8 = 5;
const SPADE: u8 = 3;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Bid(u8),
    Play(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Bid,
    Play,
    Over,
}

/// Scorul unei mâini în zecimi.
pub fn hand_score(bid: u8, won: u8) -> i32 {
    if won >= bid {
        10 * bid as i32 + (won - bid) as i32
    } else {
        -10 * bid as i32
    }
}

fn fmt_tenths(x: i32) -> String {
    format!(
        "{}{}.{}",
        if x < 0 { "-" } else { "" },
        x.abs() / 10,
        x.abs() % 10
    )
}

/// Cărțile permise la răspuns (obligația de a urca și de a tăia).
pub fn playable(hand: &[u8], led: Option<u8>) -> Vec<u8> {
    let Some(l) = led else {
        return hand.to_vec();
    };
    let follow: Vec<u8> = hand
        .iter()
        .copied()
        .filter(|&c| std_suit(c) == std_suit(l))
        .collect();
    if !follow.is_empty() {
        let higher: Vec<u8> = follow
            .iter()
            .copied()
            .filter(|&c| std_rank(c) > std_rank(l))
            .collect();
        if !higher.is_empty() {
            return higher;
        }
        return follow;
    }
    let spades: Vec<u8> = hand
        .iter()
        .copied()
        .filter(|&c| std_suit(c) == SPADE)
        .collect();
    if !spades.is_empty() {
        return spades;
    }
    hand.to_vec()
}

#[derive(Clone)]
pub struct CallBreak {
    hands: [Vec<u8>; 2],
    stock: Vec<u8>,
    played: Vec<u8>,
    bids: [Option<u8>; 2],
    tricks: [u8; 2],
    trick: Vec<(Player, u8)>,
    score: [i32; 2],
    dealer: Player,
    side: Player,
    round: u8,
    phase: Phase,
    last: String,
    rng: Rng,
    viewer: Viewer,
}

impl Default for CallBreak {
    fn default() -> Self {
        Self::new()
    }
}

/// Estimare grosieră a levatelor pe care le ia o mână (în zecimi de levată).
pub fn estimate_tricks(h: &[u8]) -> i32 {
    let mut s = 0i32;
    let mut spades = 0;
    for &c in h {
        let r = std_rank(c) as i32; // 0..12
        if std_suit(c) == SPADE {
            spades += 1;
            s += match r {
                12 => 10,
                11 => 8,
                10 => 6,
                9 => 4,
                _ => 2,
            };
        } else {
            s += match r {
                12 => 8,
                11 => 4,
                10 => 2,
                _ => 0,
            };
        }
    }
    // culori scurte pot fi tăiate cu pică
    for suit in 0..3u8 {
        let n = h.iter().filter(|&&c| std_suit(c) == suit).count();
        if n <= 1 && spades >= 3 {
            s += 4 * (2 - n as i32).max(0) * spades.min(4) / 4;
        }
    }
    s
}

impl CallBreak {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = CallBreak {
            hands: [Vec::new(), Vec::new()],
            stock: Vec::new(),
            played: Vec::new(),
            bids: [None; 2],
            tricks: [0; 2],
            trick: Vec::new(),
            score: [0; 2],
            dealer: Player::Two,
            side: Player::One,
            round: 1,
            phase: Phase::Bid,
            last: String::new(),
            rng: Rng::new(seed),
            viewer: Viewer::Fixed(Player::One),
        };
        g.deal();
        g
    }

    fn deal(&mut self) {
        let mut d = std_deck(&mut self.rng);
        shuffle_vec(&mut d, &mut self.rng);
        self.hands = [Vec::new(), Vec::new()];
        for _ in 0..13 {
            self.hands[0].push(d.pop().unwrap());
            self.hands[1].push(d.pop().unwrap());
        }
        sort_std(&mut self.hands[0]);
        sort_std(&mut self.hands[1]);
        self.stock = d;
        self.played.clear();
        self.bids = [None; 2];
        self.tricks = [0; 2];
        self.trick.clear();
        self.phase = Phase::Bid;
        self.side = self.dealer.other();
    }

    pub fn score(&self) -> (i32, i32) {
        (self.score[0], self.score[1])
    }
    pub fn set_hands_for_test(&mut self, a: Vec<u8>, b: Vec<u8>) {
        self.hands = [a, b];
    }

    fn end_round(&mut self) {
        let mut parts = Vec::new();
        for p in 0..2 {
            let b = self.bids[p].unwrap_or(1);
            let sc = hand_score(b, self.tricks[p]);
            self.score[p] += sc;
            parts.push(format!(
                "J{}: licitat {}, luat {} → {}",
                p + 1,
                b,
                self.tricks[p],
                fmt_tenths(sc)
            ));
        }
        self.last = format!("Mâna {}: {}", self.round, parts.join(" | "));
        if self.round >= ROUNDS {
            self.phase = Phase::Over;
            return;
        }
        self.round += 1;
        self.dealer = self.dealer.other();
        self.deal();
    }
}

impl Mc for CallBreak {
    fn determinize(&self, viewer: Player, rng: &mut Rng) -> Self {
        let mut g = self.clone();
        let (me, op) = (viewer.idx(), viewer.other().idx());
        let mut pool: Vec<u8> = (0..52u8)
            .filter(|c| !self.hands[me].contains(c) && !self.played.contains(c))
            .collect();
        shuffle_vec(&mut pool, rng);
        let n = self.hands[op].len();
        g.hands[op] = pool.drain(..n).collect();
        sort_std(&mut g.hands[op]);
        g.stock = pool;
        g.rng = Rng::new(rng.next_u64());
        g
    }
    fn epoch(&self) -> u32 {
        self.round as u32
    }
    fn margin(&self, p: Player) -> i32 {
        self.score[p.idx()] - self.score[p.other().idx()]
    }
}

impl Game for CallBreak {
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
            Phase::Bid => (1..=13u8).map(Move::Bid).collect(),
            Phase::Play => {
                let led = self.trick.first().map(|&(_, c)| c);
                playable(&self.hands[self.side.idx()], led)
                    .into_iter()
                    .map(Move::Play)
                    .collect()
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        match *m {
            Move::Bid(n) => {
                self.bids[self.side.idx()] = Some(n);
                if self.bids[self.side.other().idx()].is_some() {
                    self.phase = Phase::Play;
                    self.side = self.dealer.other();
                } else {
                    self.side = self.side.other();
                }
            }
            Move::Play(c) => {
                let p = self.side;
                let h = &mut self.hands[p.idx()];
                let pos = h.iter().position(|&x| x == c).expect("carte în mână");
                h.remove(pos);
                self.played.push(c);
                self.trick.push((p, c));
                if self.trick.len() < 2 {
                    self.side = p.other();
                    return;
                }
                let (lp, lc) = self.trick[0];
                let (rp, rc) = self.trick[1];
                let w = if beats2(lc, rc, Some(SPADE)) { rp } else { lp };
                self.tricks[w.idx()] += 1;
                self.trick.clear();
                self.side = w;
                if self.hands[0].is_empty() {
                    self.end_round();
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        Some(match self.score[0].cmp(&self.score[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let (me, op) = (p.idx(), p.other().idx());
        let mut s = 100 * (self.score[me] - self.score[op]);
        if self.phase == Phase::Over {
            return s;
        }
        let h = &self.hands[me];
        let est = estimate_tricks(h); // zecimi de levată
        let sig = sig_of(&[
            h.iter()
                .fold(0u64, |a, &c| a.wrapping_mul(53).wrapping_add(c as u64 + 1)),
            self.played.len() as u64,
            self.score[0] as u64,
            self.bids[0].unwrap_or(0) as u64 * 20 + self.bids[1].unwrap_or(0) as u64,
        ]);
        match self.phase {
            Phase::Bid => {
                // valoarea licitației mele: probabilitate de reușită × câștig − pierdere
                if let Some(b) = self.bids[me] {
                    let z = (est as f64 / 10.0 - b as f64 + 0.5) / 1.6;
                    let pr = 0.5 + 0.5 * (1.4 * z).tanh();
                    let gain = 10.0 * b as f64 + 1.0;
                    s += (pr * gain - (1.0 - pr) * 10.0 * b as f64) as i32 * 10;
                }
            }
            _ => {
                let played_tricks = (self.tricks[0] + self.tricks[1]) as i32;
                let left = 13 - played_tricks - self.trick.len() as i32 / 2;
                let b_me = self.bids[me].unwrap_or(1);
                let b_op = self.bids[op].unwrap_or(1);
                // levate viitoare estimate pentru mine (din mâna rămasă) și pentru adversar (restul)
                let my_more = (est as i32 / 10).min(left).max(0);
                let op_more = (left - my_more).max(0);
                let mine = hand_score(b_me, (self.tricks[me] as i32 + my_more) as u8);
                let theirs = hand_score(b_op, (self.tricks[op] as i32 + op_more) as u8);
                s += (mine - theirs) * 10;
                // cărți mari în hand și mutare la masă
                s += est;
                if let Some(&(w, c)) = self.trick.first() {
                    let st = std_rank(c) as i32 * 2 + if std_suit(c) == SPADE { 30 } else { 0 };
                    s += if w == p { st } else { -st };
                }
            }
        }
        s += noise(sig, 60);
        s + mc_value(self, p, mc_samples(), sig)
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Call Break — mâna {}/{} | scor: J1 {}, J2 {} | atu ♠\n",
            self.round.min(ROUNDS),
            ROUNDS,
            fmt_tenths(self.score[0]),
            fmt_tenths(self.score[1])
        );
        if !self.last.is_empty() {
            s.push_str(&format!("({})\n", self.last));
        }
        if self.phase == Phase::Over {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        let bid = |p: usize| {
            self.bids[p]
                .map(|b| b.to_string())
                .unwrap_or_else(|| "—".into())
        };
        s.push_str(&format!(
            "Licitații: J1 {}, J2 {} | levate luate: J1 {}, J2 {}\n",
            bid(0),
            bid(1),
            self.tricks[0],
            self.tricks[1]
        ));
        if let Some(&(w, c)) = self.trick.first() {
            s.push_str(&format!("Pe masă: {} (J{})\n", std_symbol(c), w.idx() + 1));
        }
        let v = self.viewer.who(self.side);
        s.push_str(&format!(
            "Mâna ta (J{}): {}\n",
            v.idx() + 1,
            symbols_typed(&self.hands[v.idx()])
        ));
        s.push_str(&format!(
            "Jucătorul {}: {} cărți\n",
            v.other().idx() + 1,
            self.hands[v.other().idx()].len()
        ));
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Bid(n) => format!("licit {n}"),
            Move::Play(c) => std_text(*c),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards52::SUIT_LETTERS;

    fn c(rank: u8, suit: char) -> u8 {
        let s = SUIT_LETTERS.iter().position(|&x| x == suit).unwrap() as u8;
        s * 13 + (rank - 2)
    }

    #[test]
    fn scoring_formula() {
        assert_eq!(hand_score(4, 4), 40);
        assert_eq!(hand_score(4, 6), 42); // 4,2
        assert_eq!(hand_score(5, 3), -50);
        assert_eq!(fmt_tenths(42), "4.2");
        assert_eq!(fmt_tenths(-50), "-5.0");
    }

    #[test]
    fn must_follow_beat_and_trump() {
        // ai culoarea și poți urca → doar cărțile mai mari
        let h = vec![c(3, 'h'), c(9, 'h'), c(12, 'h'), c(5, 's')];
        let mut p = playable(&h, Some(c(8, 'h')));
        p.sort();
        assert_eq!(p, vec![c(9, 'h'), c(12, 'h')]);
        // nu poți urca → orice carte din culoare
        let mut p = playable(&h, Some(c(14, 'h')));
        p.sort();
        assert_eq!(p, vec![c(3, 'h'), c(9, 'h'), c(12, 'h')]);
        // fără culoare, cu pică → trebuie pică
        let h = vec![c(3, 'd'), c(5, 's'), c(9, 's')];
        let mut p = playable(&h, Some(c(8, 'h')));
        p.sort();
        assert_eq!(p, vec![c(5, 's'), c(9, 's')]);
        // fără culoare și fără pică → orice
        let h = vec![c(3, 'd'), c(5, 'c')];
        assert_eq!(playable(&h, Some(c(8, 'h'))).len(), 2);
        // conduci → orice
        assert_eq!(playable(&h, None).len(), 2);
    }

    #[test]
    fn bidding_order_and_play_start() {
        let mut g = CallBreak::with_seed(1);
        assert_eq!(g.to_move(), Player::One); // non-dealer licitează primul
        g.apply(&Move::Bid(3));
        assert_eq!(g.to_move(), Player::Two);
        g.apply(&Move::Bid(4));
        assert_eq!(g.to_move(), Player::One); // conduce non-dealerul
        assert!(Game::legal_moves(&g)
            .iter()
            .all(|m| matches!(m, Move::Play(_))));
    }

    #[test]
    fn spades_trump_and_round_end_scores() {
        let mut g = CallBreak::with_seed(2);
        g.apply(&Move::Bid(1));
        g.apply(&Move::Bid(1));
        g.set_hands_for_test(vec![c(14, 'h')], vec![c(2, 's')]);
        g.side = Player::One;
        g.apply(&Move::Play(c(14, 'h')));
        g.apply(&Move::Play(c(2, 's')));
        // J2 a tăiat cu pică: 1 levată, a licitat 1 → +1,0; J1 0 levate, licitat 1 → −1,0
        assert_eq!(g.score(), (-10, 10));
        assert_eq!(g.round, 2);
    }

    #[test]
    fn ai_bids_reasonably() {
        // mână puternică licitează mai mult decât una slabă
        let mut strong = CallBreak::with_seed(3);
        strong.hands[0] = vec![
            c(14, 's'),
            c(13, 's'),
            c(12, 's'),
            c(11, 's'),
            c(14, 'h'),
            c(14, 'd'),
            c(14, 'c'),
            c(13, 'h'),
            c(13, 'd'),
            c(10, 's'),
            c(9, 's'),
            c(8, 's'),
            c(7, 's'),
        ];
        let mut weak = strong.clone();
        weak.hands[0] = vec![
            c(2, 's'),
            c(3, 'h'),
            c(4, 'd'),
            c(5, 'c'),
            c(6, 'h'),
            c(7, 'd'),
            c(8, 'c'),
            c(9, 'h'),
            c(3, 'd'),
            c(4, 'c'),
            c(5, 'h'),
            c(6, 'd'),
            c(7, 'c'),
        ];
        let bid = |g: &CallBreak| match polytabletop_core::ai::best_move(g, 1).unwrap() {
            Move::Bid(n) => n,
            _ => 0,
        };
        assert!(
            bid(&strong) > bid(&weak),
            "{} vs {}",
            bid(&strong),
            bid(&weak)
        );
    }

    #[test]
    fn render_hides_opponent() {
        let g = CallBreak::with_seed(4);
        let r = Game::render(&g);
        for &x in &g.hands[1] {
            assert!(!r.contains(&format!("[{}]", std_text(x))), "{r}");
        }
    }

    #[test]
    fn ai_finishes_match() {
        let mut g = CallBreak::with_seed(5);
        let mut n = 0;
        while g.outcome().is_none() && n < 5000 {
            let m = polytabletop_core::ai::best_move(&g, n).unwrap();
            g.apply(&m);
            n += 1;
        }
        assert!(g.outcome().is_some());
    }
}

#[cfg(test)]
mod a3_audit {
    use super::*;

    #[test]
    fn ai_ignores_the_opponent_hand_at_every_level() {
        crate::p3_comun::ai_blind_check(
            "callbreak",
            CallBreak::with_seed,
            |g, me, s| {
                *g = g.determinize(me, &mut Rng::new(s));
            },
            4,
        );
    }
}
