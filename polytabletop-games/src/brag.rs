#![allow(clippy::needless_range_loop, clippy::manual_is_multiple_of)]
//! Three Card Brag (Anglia) — varianta cu **2 jucători**, pariuri cu fise.
//!
//! Pachet de 52 de cărți (A cea mai mare; A-2-3 este secvență joasă dar a doua ca forță după A-K-Q).
//! Fiecare primește 3 cărți ascunse și pune ante 1. Ierarhia mâinilor (de sus în jos): trio (A-A-A cel mai
//! mare, 3-3-3 cel mai mic; la unii „trei de 3” bate totul — aici este varianta clasică cu A-A-A sus),
//! culoare-secvență, secvență (A-K-Q cea mai mare, 3-2-A cea mai mică), culoare, pereche, carte mare.
//! La mutare: `ridic` (ridici miza cu 1 fisă, până la 5), `vad` (egalezi și se arată cărțile — câștigă mâna
//! mai tare; la egalitate pierde cel care a cerut arătarea) sau `fold` (renunți, pierzi ce ai pus).
//! Meciul are 10 mâini cu 30 de fise de fiecare; câștigă cine are mai multe fise la sfârșit.
//! Fiecare mână se termină cu mutarea forțată `urmatoarea` (cărțile noi se împart acolo).
//! Informație ascunsă: AI-ul cinstit vede doar cărțile lui și determinizează adversarul.

use crate::cards::{full_deck, handover_banner, join_cards, shuffle, Card, Viewer};
use crate::p3_comun::{honest_eval, Honest};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const HANDS: u8 = 10;
const START_CHIPS: i32 = 30;
const MAX_BET: i32 = 5;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Raise,
    See,
    Fold,
    /// mutare forțată: arătarea cărților (rezultatul nu e vizibil înainte de decizie)
    Show,
    Next,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Bet,
    Showdown,
    HandOver,
    Over,
}

/// Forța unei mâini de 3 cărți (mai mare = mai bună).
pub fn strength(h: &[Card]) -> u32 {
    let mut r: Vec<u32> = h.iter().map(|c| c.rank as u32).collect();
    r.sort_unstable_by(|a, b| b.cmp(a));
    let flush = h.iter().all(|c| c.suit == h[0].suit);
    let (a, b, c) = (r[0], r[1], r[2]);
    let seq_high = a == b + 1 && b == c + 1;
    let seq_low = a == 14 && b == 3 && c == 2;
    let seq = seq_high || seq_low;
    // valoare de secvență: A-K-Q cel mai mare, 3-2-A al doilea de jos în secvențe; A-2-3 numit cea mai mică
    let seqv = if seq_low { 3 } else { a };
    let hi = a * 225 + b * 15 + c;
    if a == b && b == c {
        6_000_000 + a * 100
    } else if seq && flush {
        5_000_000 + seqv
    } else if seq {
        4_000_000 + seqv
    } else if flush {
        3_000_000 + hi
    } else if a == b || b == c {
        let (p, k) = if a == b { (a, c) } else { (b, a) };
        2_000_000 + p * 15 + k
    } else {
        1_000_000 + hi
    }
}

fn name(h: &[Card]) -> &'static str {
    let s = strength(h);
    match s / 1_000_000 {
        6 => "trio",
        5 => "culoare-secvență",
        4 => "secvență",
        3 => "culoare",
        2 => "pereche",
        _ => "carte mare",
    }
}

#[derive(Clone)]
pub struct Brag {
    rng: Rng,
    hands: [Vec<Card>; 2],
    chips: [i32; 2],
    put: [i32; 2],
    level: i32,
    side: Player,
    prev: Player,
    dealer: Player,
    hand_no: u8,
    phase: Phase,
    last_raiser: Option<Player>,
    gain: [i32; 2],
    shown: bool,
    log: String,
    viewer: Viewer,
}

impl Default for Brag {
    fn default() -> Self {
        Self::new()
    }
}

impl Brag {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Brag {
            rng: Rng::new(seed),
            hands: [Vec::new(), Vec::new()],
            chips: [START_CHIPS; 2],
            put: [0; 2],
            level: 1,
            side: Player::Two,
            prev: Player::Two,
            dealer: Player::One,
            hand_no: 1,
            phase: Phase::Bet,
            last_raiser: None,
            gain: [0; 2],
            shown: false,
            log: String::new(),
            viewer: Viewer::Fixed(Player::One),
        };
        g.deal();
        g
    }

    pub fn chips(&self) -> (i32, i32) {
        (self.chips[0], self.chips[1])
    }

    fn deal(&mut self) {
        let mut d = full_deck(2);
        shuffle(&mut d, &mut self.rng);
        for p in 0..2 {
            self.hands[p] = (0..3).map(|_| d.pop().unwrap()).collect();
            self.hands[p].sort_by_key(|c| std::cmp::Reverse(c.rank));
            let ante = self.chips[p].min(1);
            self.chips[p] -= ante;
            self.put[p] = ante;
        }
        self.level = 1;
        self.last_raiser = None;
        self.gain = [0; 2];
        self.shown = false;
        self.phase = Phase::Bet;
        self.side = self.dealer.other();
        self.prev = self.side;
    }

    fn settle(&mut self, winner: Player) {
        let pot = self.put[0] + self.put[1];
        self.chips[winner.idx()] += pot;
        self.gain[winner.idx()] = pot - self.put[winner.idx()];
        self.gain[winner.other().idx()] = -self.put[winner.other().idx()];
        let broke = self.chips.iter().any(|&c| c <= 0);
        self.phase = if self.hand_no >= HANDS || broke {
            Phase::Over
        } else {
            Phase::HandOver
        };
    }

    fn cost_to_match(&self) -> i32 {
        (self.put[self.side.other().idx()] - self.put[self.side.idx()]).max(0)
    }
}

impl Game for Brag {
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
            Phase::Showdown => vec![Move::Show],
            Phase::Bet => {
                let mut v = vec![Move::See, Move::Fold];
                let me = self.side.idx();
                let need = self.cost_to_match();
                if self.level < MAX_BET && self.chips[me] > need {
                    v.insert(0, Move::Raise);
                }
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        self.prev = self.side;
        let me = self.side;
        match m {
            Move::Next => {
                self.hand_no += 1;
                self.dealer = self.dealer.other();
                self.deal();
            }
            Move::Raise => {
                let need = self.cost_to_match();
                let pay = (need + 1).min(self.chips[me.idx()]);
                self.chips[me.idx()] -= pay;
                self.put[me.idx()] += pay;
                self.level = self.put[me.idx()].max(self.level + 1).min(MAX_BET);
                self.last_raiser = Some(me);
                self.side = me.other();
                // adversarul trebuie să egaleze: dacă nu are destule fise, „vad” plătește cât are
            }
            Move::Fold => {
                self.log = format!("J{} renunță", me.idx() + 1);
                self.settle(me.other());
            }
            Move::See => {
                let need = self.cost_to_match().min(self.chips[me.idx()]);
                self.chips[me.idx()] -= need;
                self.put[me.idx()] += need;
                self.phase = Phase::Showdown;
                // `side` rămâne la cel care a cerut arătarea
            }
            Move::Show => {
                self.shown = true;
                let (a, b) = (strength(&self.hands[0]), strength(&self.hands[1]));
                let w = if a == b {
                    // egalitate: pierde cel care a cerut arătarea
                    me.other()
                } else if a > b {
                    Player::One
                } else {
                    Player::Two
                };
                self.log = format!(
                    "Arătare: J1 {} ({}) vs J2 {} ({}) — câștigă J{}",
                    join_cards(&self.hands[0]),
                    name(&self.hands[0]),
                    join_cards(&self.hands[1]),
                    name(&self.hands[1]),
                    w.idx() + 1
                );
                self.settle(w);
            }
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
        if self.phase == Phase::Over {
            return 0;
        }
        honest_eval(self, p, 40)
    }

    fn render(&self) -> String {
        let mut s = String::new();
        if self.prev != self.side && self.phase == Phase::Bet {
            if let Viewer::HotSeat = self.viewer {
                s.push_str(&handover_banner(self.side));
            }
        }
        s.push_str(&format!(
            "Three Card Brag — mâna {}/{} | fise: J1 {}, J2 {}\n",
            self.hand_no, HANDS, self.chips[0], self.chips[1]
        ));
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        match self.phase {
            Phase::Over => {
                s.push_str("Meci încheiat.\n");
                return s;
            }
            Phase::HandOver => {
                s.push_str("Scrie «urmatoarea» pentru mâna următoare.\n");
                return s;
            }
            Phase::Showdown => {
                s.push_str("Cărțile se arată: scrie «arata».\n");
                return s;
            }
            Phase::Bet => {}
        }
        s.push_str(&format!(
            "Pot: {} (J1 {}, J2 {}) | miza curentă: {}\n",
            self.put[0] + self.put[1],
            self.put[0],
            self.put[1],
            self.level
        ));
        let v = self.viewer.who(self.side);
        s.push_str(&format!(
            "Mâna ta (J{}): {} — {}\n",
            v.idx() + 1,
            join_cards(&self.hands[v.idx()]),
            name(&self.hands[v.idx()])
        ));
        s.push_str(&format!("J{} are 3 cărți ascunse\n", v.other().idx() + 1));
        s.push_str(&format!(
            "La mutare: Jucătorul {} (de egalat: {})\n",
            self.side.idx() + 1,
            self.cost_to_match()
        ));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Raise => "ridic",
            Move::See => "vad",
            Move::Fold => "fold",
            Move::Show => "arata",
            Move::Next => "urmatoarea",
        }
        .into()
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

impl Honest for Brag {
    fn determinize(&self, me: Player, rng: &mut Rng) -> Self {
        let mut g = self.clone();
        let mi = me.idx();
        let mut pool = full_deck(2);
        pool.retain(|c| !self.hands[mi].contains(c));
        shuffle(&mut pool, rng);
        g.hands[1 - mi] = (0..3).map(|_| pool.pop().unwrap()).collect();
        g
    }

    fn rollout_done(&self) -> bool {
        matches!(self.phase, Phase::HandOver | Phase::Over)
    }

    fn rollout_value(&self, p: Player) -> f64 {
        (self.gain[p.idx()]) as f64
    }

    fn policy(&self, rng: &mut Rng) -> Move {
        let mv = self.legal_moves();
        if self.phase == Phase::Showdown {
            return Move::Show;
        }
        let st = strength(&self.hands[self.side.idx()]) / 1_000_000;
        let r = rng.below(100);
        let pick = match st {
            1 => {
                if r < 25 {
                    Move::Raise
                } else if r < 55 {
                    Move::See
                } else {
                    Move::Fold
                }
            }
            2 => {
                if r < 35 {
                    Move::Raise
                } else if r < 90 {
                    Move::See
                } else {
                    Move::Fold
                }
            }
            _ => {
                if r < 60 {
                    Move::Raise
                } else {
                    Move::See
                }
            }
        };
        if mv.contains(&pick) {
            pick
        } else {
            Move::See
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
    fn ranking() {
        let trio = [c(3, 0), c(3, 1), c(3, 2)];
        let sf = [c(14, 0), c(13, 0), c(12, 0)];
        let run = [c(14, 0), c(13, 1), c(12, 0)];
        let fl = [c(14, 0), c(9, 0), c(2, 0)];
        let pair = [c(14, 0), c(14, 1), c(2, 0)];
        let hi = [c(14, 0), c(9, 1), c(2, 2)];
        let low_run = [c(14, 0), c(3, 1), c(2, 0)];
        for w in [hi, pair, fl, run, sf, trio].windows(2) {
            assert!(strength(&w[0]) < strength(&w[1]));
        }
        assert!(strength(&low_run) < strength(&run));
        assert!(strength(&low_run) > strength(&fl));
    }

    #[test]
    fn fold_and_see() {
        let mut g = Brag::with_seed(5);
        let first = g.to_move();
        g.apply(&Move::Raise);
        assert_eq!(g.to_move(), first.other());
        assert_eq!(g.cost_to_match(), 1);
        g.apply(&Move::Fold);
        assert_eq!(g.phase, Phase::HandOver);
        assert_eq!(g.chips[first.idx()], START_CHIPS - 1 + 3 - 1);
        g.apply(&Move::Next);
        assert_eq!(g.hand_no, 2);
    }

    #[test]
    fn showdown_is_a_separate_forced_move() {
        let mut g = Brag::with_seed(9);
        g.apply(&Move::See);
        assert_eq!(g.legal_moves(), vec![Move::Show]);
        assert!(g.outcome().is_none());
        g.apply(&Move::Show);
        assert!(matches!(g.phase, Phase::HandOver | Phase::Over));
        assert_eq!(g.chips[0] + g.chips[1], 2 * START_CHIPS);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=6u64 {
            let mut g = Brag::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut k = 0;
            while g.outcome().is_none() && k < 4000 {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
                k += 1;
            }
            assert!(g.outcome().is_some());
            assert_eq!(g.chips[0] + g.chips[1], 2 * START_CHIPS);
        }
    }
}
