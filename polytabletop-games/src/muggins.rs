#![allow(clippy::needless_range_loop, clippy::manual_range_contains)]
//! Muggins / All Fives (SUA, Marea Britanie) — domino cu punctaj pe capete, 2 jucători, set dublu-6.
//!
//! * 7 piese fiecare, restul stoc. Prima rundă începe cine are cel mai mare dublu; apoi începe
//!   câștigătorul rundei precedente cu orice piesă. Așezi piese la stânga (`S`) sau la dreapta (`D`),
//!   cu numărul potrivit; fără piesă potrivită pescuiești (`pesc`) până poți juca; cu stocul gol: `pas`.
//! * **Punctaj**: după fiecare piesă, dacă suma celor două capete ale lanțului e multiplu de 5, iei
//!   suma / 5 puncte (10 → 2 puncte). O piesă dublă la un capăt se numără cu ambele jumătăți; o
//!   singură dublă în lanț valorează suma jumătăților. Fără „spinner” (dublă care se ramifică).
//! * Runda se termină când cineva rămâne fără piese (ia pipurile adversarului, rotunjite la cel mai
//!   apropiat multiplu de 5, împărțite la 5) sau la blocaj (cine are mai puține pipuri ia diferența, la
//!   fel rotunjită). Primul la 61 de puncte câștigă; următoarea rundă: `urmatoarea`.
//!
//! Mutări: `3-4:S`, `3-4:D` (prima piesă a rundei: `6-6` sau orice piesă, fără sufix), `pesc`, `pas`,
//! `urmatoarea`. Informația ascunsă: vezi `cards.rs`; AI-ul citește doar mâna lui, lanțul și numărul
//! pieselor adversarului.

use crate::cards::{handover_banner, pl, shuffle, Viewer};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub type Tile = (u8, u8);

const TARGET: i32 = 61;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum End {
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Place(Tile, End),
    Draw,
    Pass,
    Next,
}

fn tile_text(t: Tile) -> String {
    format!("{}-{}", t.0, t.1)
}

fn join_tiles(v: &[Tile]) -> String {
    if v.is_empty() {
        return "—".into();
    }
    v.iter()
        .map(|&t| tile_text(t))
        .collect::<Vec<_>>()
        .join(" ")
}

fn pips(t: Tile) -> i32 {
    (t.0 + t.1) as i32
}

/// Valoarea de numărat a capetelor lanțului (suma, cu dublele la capăt numărate dublu).
pub fn ends_total(chain: &[Tile]) -> i32 {
    match chain.len() {
        0 => 0,
        1 => pips(chain[0]),
        _ => {
            let l = chain[0];
            let r = chain[chain.len() - 1];
            let lv = if l.0 == l.1 {
                2 * l.0 as i32
            } else {
                l.0 as i32
            };
            let rv = if r.0 == r.1 {
                2 * r.1 as i32
            } else {
                r.1 as i32
            };
            lv + rv
        }
    }
}

/// Punctele (în „cinciuri”) pentru o sumă de capete.
pub fn five_points(total: i32) -> i32 {
    if total > 0 && total % 5 == 0 {
        total / 5
    } else {
        0
    }
}

fn round5(x: i32) -> i32 {
    ((x + 2) / 5) * 5
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Play,
    RoundOver,
    Over,
}

#[derive(Clone)]
pub struct Muggins {
    rng: Rng,
    stock: Vec<Tile>,
    hands: [Vec<Tile>; 2],
    chain: Vec<Tile>,
    side: Player,
    first: Player,
    first_round: bool,
    start: Tile,
    passes: u8,
    fresh: [u8; 2],
    scores: [i32; 2],
    phase: Phase,
    log: String,
    viewer: Viewer,
    handover: bool,
}

impl Default for Muggins {
    fn default() -> Self {
        Self::new()
    }
}

impl Muggins {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Muggins {
            rng: Rng::new(seed),
            stock: Vec::new(),
            hands: [Vec::new(), Vec::new()],
            chain: Vec::new(),
            side: Player::One,
            first: Player::One,
            first_round: true,
            start: (0, 0),
            passes: 0,
            fresh: [0, 0],
            scores: [0, 0],
            phase: Phase::Play,
            log: String::new(),
            viewer: Viewer::Fixed(Player::One),
            handover: false,
        };
        g.deal();
        g
    }

    pub fn scores(&self) -> (i32, i32) {
        (self.scores[0], self.scores[1])
    }

    fn deal(&mut self) {
        let mut all: Vec<Tile> = Vec::new();
        for a in 0..=6u8 {
            for b in a..=6u8 {
                all.push((a, b));
            }
        }
        shuffle(&mut all, &mut self.rng);
        let h1: Vec<Tile> = all.drain(..7).collect();
        let h2: Vec<Tile> = all.drain(..7).collect();
        self.stock = all;
        self.hands = [h1, h2];
        self.chain.clear();
        self.passes = 0;
        self.fresh = [0, 0];
        self.phase = Phase::Play;
        if self.first_round {
            let key = |t: Tile| (t.0 == t.1, pips(t), t.1);
            let b1 = *self.hands[0].iter().max_by_key(|&&t| key(t)).unwrap();
            let b2 = *self.hands[1].iter().max_by_key(|&&t| key(t)).unwrap();
            if key(b1) >= key(b2) {
                self.side = Player::One;
                self.start = b1;
            } else {
                self.side = Player::Two;
                self.start = b2;
            }
        } else {
            self.side = self.first;
        }
    }

    fn ends(&self) -> Option<(u8, u8)> {
        Some((self.chain.first()?.0, self.chain.last()?.1))
    }

    fn hand_pips(&self, p: Player) -> i32 {
        self.hands[p.idx()].iter().map(|&t| pips(t)).sum()
    }

    fn end_round(&mut self, winner: Option<Player>, pts: i32) {
        if let Some(w) = winner {
            self.scores[w.idx()] += pts;
            self.log = format!("{} J{} ia {} puncte", self.log_prefix(), w.idx() + 1, pts);
            self.first = w;
        } else {
            self.log = "Blocaj egal: nimeni nu punctează".into();
        }
        self.first_round = false;
        self.phase = if self.scores.iter().any(|&s| s >= TARGET) {
            Phase::Over
        } else {
            Phase::RoundOver
        };
    }

    fn log_prefix(&self) -> &'static str {
        if self.passes >= 2 {
            "Blocaj:"
        } else {
            "Domino!"
        }
    }
}

impl Game for Muggins {
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
            Phase::Over => return Vec::new(),
            Phase::RoundOver => return vec![Move::Next],
            Phase::Play => {}
        }
        let hand = &self.hands[self.side.idx()];
        let Some((l, r)) = self.ends() else {
            if self.first_round {
                return vec![Move::Place(self.start, End::Left)];
            }
            let mut v: Vec<Move> = hand.iter().map(|&t| Move::Place(t, End::Left)).collect();
            v.dedup();
            return v;
        };
        let mut v = Vec::new();
        for &t in hand {
            if t.0 == l || t.1 == l {
                v.push(Move::Place(t, End::Left));
            }
            if t.0 == r || t.1 == r {
                v.push(Move::Place(t, End::Right));
            }
        }
        if v.is_empty() {
            v.push(if self.stock.is_empty() {
                Move::Pass
            } else {
                Move::Draw
            });
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        self.fresh = [0, 0];
        self.handover = false;
        self.log.clear();
        let me = self.side;
        match *m {
            Move::Next => {
                self.deal();
            }
            Move::Place(t, end) => {
                let h = &mut self.hands[me.idx()];
                let pos = h.iter().position(|&x| x == t).expect("tile in hand");
                h.remove(pos);
                match (self.ends(), end) {
                    (None, _) => self.chain.push(t),
                    (Some((l, _)), End::Left) => {
                        let o = if t.1 == l { t } else { (t.1, t.0) };
                        self.chain.insert(0, o);
                    }
                    (Some((_, r)), End::Right) => {
                        let o = if t.0 == r { t } else { (t.1, t.0) };
                        self.chain.push(o);
                    }
                }
                self.passes = 0;
                let pts = five_points(ends_total(&self.chain));
                if pts > 0 {
                    self.scores[me.idx()] += pts;
                    self.log = format!(
                        "J{} ia {} pct (capete {})",
                        me.idx() + 1,
                        pts,
                        ends_total(&self.chain)
                    );
                }
                if self.scores[me.idx()] >= TARGET {
                    self.phase = Phase::Over;
                    return;
                }
                if self.hands[me.idx()].is_empty() {
                    let pts = round5(self.hand_pips(me.other())) / 5;
                    let l = self.log.clone();
                    self.end_round(Some(me), pts);
                    if !l.is_empty() {
                        self.log = format!("{l}; {}", self.log);
                    }
                } else {
                    self.side = me.other();
                    self.handover = true;
                }
            }
            Move::Draw => {
                if let Some(t) = self.stock.pop() {
                    self.hands[me.idx()].push(t);
                    self.fresh[me.idx()] = 1;
                }
            }
            Move::Pass => {
                self.passes += 1;
                if self.passes >= 2 {
                    let (a, b) = (self.hand_pips(Player::One), self.hand_pips(Player::Two));
                    match a.cmp(&b) {
                        std::cmp::Ordering::Less => {
                            self.end_round(Some(Player::One), round5(b - a) / 5)
                        }
                        std::cmp::Ordering::Greater => {
                            self.end_round(Some(Player::Two), round5(a - b) / 5)
                        }
                        std::cmp::Ordering::Equal => self.end_round(None, 0),
                    }
                } else {
                    self.side = me.other();
                    self.handover = true;
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
        let q = p.other();
        let hand = &self.hands[p.idx()];
        let known = &hand[..hand.len().saturating_sub(self.fresh[p.idx()] as usize)];
        let mut s = (self.scores[p.idx()] - self.scores[q.idx()]) * 25;
        s += 4 * (self.hands[q.idx()].len() as i32 - hand.len() as i32);
        s -= known.iter().map(|&t| pips(t)).sum::<i32>() / 2;
        // capete care dau multiplu de 5 pentru mine (ușor) sau pentru adversar (periculos): se judecă prin scorul de pe masă
        let t = ends_total(&self.chain);
        let pt = five_points(t) * 10;
        if self.side == q {
            s += pt; // tocmai am mutat eu: capetele valorează pentru mine
        }
        s
    }

    fn render(&self) -> String {
        let mut s = String::new();
        if self.handover && self.phase == Phase::Play {
            if let Viewer::HotSeat = self.viewer {
                s.push_str(&handover_banner(self.side));
            }
        }
        s.push_str(&format!(
            "Muggins (All Fives) — puncte: J1 {}, J2 {} (până la {}) | stoc: {} piese\n",
            self.scores[0],
            self.scores[1],
            TARGET,
            self.stock.len()
        ));
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        let chain: Vec<String> = self
            .chain
            .iter()
            .map(|&(a, b)| format!("{a}|{b}"))
            .collect();
        s.push_str(&format!(
            "Lanț: {}   (capete: {})\n",
            if chain.is_empty() {
                "—".to_string()
            } else {
                chain.join(" ")
            },
            ends_total(&self.chain)
        ));
        match self.phase {
            Phase::Over => {
                s.push_str("Joc încheiat.\n");
                return s;
            }
            Phase::RoundOver => {
                s.push_str(&format!(
                    "Mâna J1: {} | Mâna J2: {}\nScrie «urmatoarea».\n",
                    join_tiles(&self.hands[0]),
                    join_tiles(&self.hands[1])
                ));
                return s;
            }
            Phase::Play => {}
        }
        let v = self.viewer.who(self.side);
        s.push_str(&format!(
            "Mâna Jucătorului {}: {}\n",
            pl(v),
            join_tiles(&self.hands[v.idx()])
        ));
        s.push_str(&format!(
            "Jucătorul {}: {} piese\n",
            pl(v.other()),
            self.hands[v.other().idx()].len()
        ));
        s.push_str(&format!("La mutare: Jucătorul {}\n", pl(self.side)));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Place(t, e) => {
                if self.chain.is_empty() {
                    tile_text(*t)
                } else {
                    format!(
                        "{}:{}",
                        tile_text(*t),
                        if *e == End::Left { 'S' } else { 'D' }
                    )
                }
            }
            Move::Draw => "pesc".into(),
            Move::Pass => "pas".into(),
            Move::Next => "urmatoarea".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup(h1: Vec<Tile>, h2: Vec<Tile>, chain: Vec<Tile>) -> Muggins {
        let mut g = Muggins::with_seed(1);
        g.hands = [h1, h2];
        g.chain = chain;
        g.stock.clear();
        g.side = Player::One;
        g.passes = 0;
        g.first_round = false;
        g
    }

    #[test]
    fn end_totals() {
        assert_eq!(ends_total(&[(3, 3)]), 6);
        assert_eq!(ends_total(&[(2, 3), (3, 4)]), 6);
        assert_eq!(ends_total(&[(5, 5), (5, 3)]), 13);
        assert_eq!(ends_total(&[(6, 4), (4, 4), (4, 5)]), 11);
        assert_eq!(ends_total(&[(5, 5), (5, 0), (0, 5)]), 15);
        assert_eq!(five_points(10), 2);
        assert_eq!(five_points(15), 3);
        assert_eq!(five_points(7), 0);
    }

    #[test]
    fn scoring_on_placement() {
        let mut g = setup(vec![(3, 4), (1, 1)], vec![(6, 6), (0, 0)], vec![(2, 3)]);
        g.apply(&Move::Place((3, 4), End::Right)); // lanț 2|3 3|4: capete 2+4 = 6 → nimic?
        assert_eq!(g.scores, [0, 0]);
        let mut h = setup(vec![(2, 3), (1, 1)], vec![(6, 6), (0, 0)], vec![(5, 5)]);
        h.apply(&Move::Place((2, 3), End::Left)); // lanț 2|3? (5,5) capete: (3,2)? piesa orientată: t.1 == l(5)? nu -> (3,2) apoi 5|5 -> capete 3 + 10 = 13
        assert_eq!(h.scores, [0, 0]);
        let mut k = setup(vec![(5, 0), (1, 1)], vec![(6, 6), (0, 0)], vec![(5, 5)]);
        k.apply(&Move::Place((5, 0), End::Left)); // 0|5 5|5 → capete 0 + 10 = 10 → 2 puncte
        assert_eq!(k.scores, [2, 0]);
    }

    #[test]
    fn going_out_scores_rounded_pips() {
        let mut g = setup(vec![(5, 0)], vec![(6, 6), (3, 4)], vec![(5, 5)]);
        g.apply(&Move::Place((5, 0), End::Left));
        // capete 10 → 2 puncte + mâna adversarului 12+7 = 19 → 20 → 4
        assert_eq!(g.scores[0], 2 + 4);
        assert_eq!(g.legal_moves(), vec![Move::Next]);
        g.apply(&Move::Next);
        assert_eq!(g.side, Player::One);
        assert_eq!(g.hands[0].len(), 7);
        assert!(
            g.legal_moves().len() == 7
                || g.legal_moves()
                    .iter()
                    .all(|m| matches!(m, Move::Place(_, End::Left)))
        );
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=8u64 {
            let mut g = Muggins::with_seed(seed);
            let mut rng = Rng::new(seed + 3);
            let mut n = 0;
            while g.outcome().is_none() && n < 8000 {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed}");
        }
    }
}
