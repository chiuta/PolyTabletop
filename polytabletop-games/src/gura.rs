//! GURA SATULUI — variantă originală de joc de completat fraze cu cărți-răspuns (doi jucători).
//!
//! Mecanica genului (o frază cu gol, răspunsuri jucate, un judecător care alege cel mai
//! bun răspuns); frazele și răspunsurile sunt ale noastre (`gura_date`), fără glume
//! cu ținte reale.
//! * Fiecare are 7 răspunsuri. Judecătorul alternează (J1, J2, J1…). Celălalt joacă un
//!   răspuns (`joaca N`). Pasul forțat `amesteca` adaugă 2 răspunsuri „ale casei” din
//!   pachet și amestecă cele 3 cărți fără să arate cine ce a pus.
//! * Judecătorul alege cea mai nostimă completare (`alege N`). Dacă a ales cartea
//!   jucătorului, acesta ia un punct; dacă a ales una a casei, ia punctul judecătorul
//!   (pasul forțat `numara`). Primul la 6 puncte sau cel mai bun după 12 runde.
//!
//! AI-ul potrivește etichetele frazei cu cele ale răspunsului (și „hazul” lui).
//! Nu vede ce carte a pus cine (alege doar după potrivire).

use crate::cards::{handover_banner, shuffle, Viewer};
use crate::gura_date::{ANSWERS, PROMPTS};
use crate::w4_comun::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::fmt::Write;

const HAND: usize = 7;
const WIN_SCORE: i32 = 6;
const MAX_ROUNDS: usize = 12;

fn common(a: &str, b: &str) -> i32 {
    a.chars().filter(|c| b.contains(*c)).count() as i32
}

/// Cât de bine se potrivește răspunsul `a` cu fraza `p` (după etichete și haz).
fn fit(p: usize, a: usize) -> i32 {
    20 * common(PROMPTS[p].1, ANSWERS[a].1) + 10 * ANSWERS[a].2 as i32
}

pub fn fill(p: usize, a: usize) -> String {
    PROMPTS[p].0.replacen("___", ANSWERS[a].0, 1)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Play,
    Shuffle,
    Judge,
    Tally,
    Over,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Play(usize),
    Shuffle,
    Pick(usize),
    Tally,
}

#[derive(Clone)]
pub struct Gura {
    rng: Rng,
    prompts: Vec<usize>,
    deck: Vec<usize>,
    hands: [Vec<usize>; 2],
    round: usize,
    prompt: usize,
    played: Option<usize>,
    /// (răspuns, 0 = al jucătorului, 1 = al casei)
    table: Vec<(usize, u8)>,
    pick: usize,
    pts: [i32; 2],
    phase: Phase,
    log: Vec<String>,
    heur: Heur,
    viewer: Viewer,
    last: Player,
}

impl Default for Gura {
    fn default() -> Self {
        Self::new()
    }
}

impl Gura {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut prompts: Vec<usize> = (0..PROMPTS.len()).collect();
        shuffle(&mut prompts, &mut rng);
        let mut deck: Vec<usize> = (0..ANSWERS.len()).collect();
        shuffle(&mut deck, &mut rng);
        let mut g = Gura {
            rng,
            prompts,
            deck,
            hands: [Vec::new(), Vec::new()],
            round: 0,
            prompt: 0,
            played: None,
            table: Vec::new(),
            pick: 0,
            pts: [0, 0],
            phase: Phase::Play,
            log: Vec::new(),
            heur: Heur::default(),
            viewer: Viewer::Fixed(Player::One),
            last: Player::One,
        };
        g.prompt = g.prompts[0];
        g.refill();
        g
    }

    pub fn with_viewer(mut self, v: Viewer) -> Self {
        self.viewer = v;
        self
    }

    pub fn hot_seat(self) -> Self {
        self.with_viewer(Viewer::HotSeat)
    }

    pub fn scores(&self) -> (i32, i32) {
        (self.pts[0], self.pts[1])
    }

    fn judge(&self) -> Player {
        if self.round.is_multiple_of(2) {
            Player::One
        } else {
            Player::Two
        }
    }

    fn player(&self) -> Player {
        self.judge().other()
    }

    fn refill(&mut self) {
        for p in 0..2 {
            while self.hands[p].len() < HAND {
                match self.deck.pop() {
                    Some(c) => self.hands[p].push(c),
                    None => break,
                }
            }
        }
    }
}

impl Game for Gura {
    type Move = Move;

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = match who {
            Some(p) => Viewer::Fixed(p),
            None => Viewer::HotSeat,
        };
    }

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Play => self.player(),
            Phase::Judge => self.judge(),
            _ => RESOLVER,
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        match self.phase {
            Phase::Play => (0..self.hands[self.player().idx()].len())
                .map(Move::Play)
                .collect(),
            Phase::Shuffle => vec![Move::Shuffle],
            Phase::Judge => (0..self.table.len()).map(Move::Pick).collect(),
            Phase::Tally => vec![Move::Tally],
            Phase::Over => Vec::new(),
        }
    }

    fn apply(&mut self, m: &Move) {
        let mover = self.to_move();
        if !matches!(m, Move::Shuffle | Move::Tally) {
            self.last = mover;
        }
        self.heur.clear();
        match *m {
            Move::Play(h) => {
                let pl = self.player().idx();
                let a = self.hands[pl].remove(h);
                self.heur.set(mover, fit(self.prompt, a));
                self.played = Some(a);
                self.phase = Phase::Shuffle;
            }
            Move::Shuffle => {
                let a = self.played.expect("răspuns jucat");
                self.table = vec![(a, 0)];
                for _ in 0..2 {
                    if let Some(c) = self.deck.pop() {
                        self.table.push((c, 1));
                    }
                }
                shuffle(&mut self.table, &mut self.rng);
                self.phase = Phase::Judge;
            }
            Move::Pick(i) => {
                let v = fit(self.prompt, self.table[i].0);
                self.heur.set(mover, v);
                self.pick = i;
                self.phase = Phase::Tally;
            }
            Move::Tally => {
                let (a, own) = self.table[self.pick];
                let who = if own == 0 {
                    self.player()
                } else {
                    self.judge()
                };
                self.pts[who.idx()] += 1;
                self.log.push(format!(
                    "Judecătorul (J{}) a ales «{}» — punct pentru J{} ({}).",
                    self.judge().idx() + 1,
                    fill(self.prompt, a),
                    who.idx() + 1,
                    if own == 0 {
                        "carte jucată"
                    } else {
                        "cartea casei"
                    }
                ));
                self.round += 1;
                self.played = None;
                self.table.clear();
                self.refill();
                if self.pts.iter().any(|&p| p >= WIN_SCORE) || self.round >= MAX_ROUNDS {
                    self.phase = Phase::Over;
                } else {
                    self.prompt = self.prompts[self.round % self.prompts.len()];
                    self.phase = Phase::Play;
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase == Phase::Over {
            Some(result_of(&self.pts))
        } else {
            None
        }
    }

    fn evaluate(&self, p: Player) -> i32 {
        100 * diff(&self.pts, p) + self.heur.for_player(p)
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }

    fn render(&self) -> String {
        let mut s = String::new();
        let who = match self.viewer {
            Viewer::Fixed(p) => p,
            Viewer::HotSeat => match self.phase {
                Phase::Shuffle | Phase::Tally | Phase::Over => self.last,
                _ => self.to_move(),
            },
        };
        if let Viewer::HotSeat = self.viewer {
            if matches!(self.phase, Phase::Play | Phase::Judge) && who != self.last {
                s.push_str(&handover_banner(who));
            }
        }
        let _ = writeln!(
            s,
            "Gura Satului — runda {}/{} | puncte: J1 {}, J2 {} (până la {})",
            (self.round + 1).min(MAX_ROUNDS),
            MAX_ROUNDS,
            self.pts[0],
            self.pts[1],
            WIN_SCORE
        );
        if self.phase == Phase::Over {
            s.push_str("Joc încheiat.\n");
            s.push_str(&log_tail(&self.log, 3));
            return s;
        }
        let _ = writeln!(
            s,
            "Judecător: Jucătorul {} | joacă: Jucătorul {}",
            self.judge().idx() + 1,
            self.player().idx() + 1
        );
        let _ = writeln!(s, "Fraza: {}", PROMPTS[self.prompt].0);
        match self.phase {
            Phase::Play if who == self.player() => {
                s.push_str("Răspunsurile tale (joaca N):\n");
                for (i, &a) in self.hands[who.idx()].iter().enumerate() {
                    let _ = writeln!(s, "  {}) {}", i + 1, ANSWERS[a].0);
                }
            }
            Phase::Judge | Phase::Tally => {
                s.push_str("Completări (alege N cea mai nostimă):\n");
                for (i, &(a, _)) in self.table.iter().enumerate() {
                    let _ = writeln!(s, "  {}) {}", i + 1, fill(self.prompt, a));
                }
            }
            _ => {}
        }
        s.push_str(&log_tail(&self.log, 2));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Play(h) => format!("joaca {}", h + 1),
            Move::Shuffle => "amesteca".into(),
            Move::Pick(i) => format!("alege {}", i + 1),
            Move::Tally => "numara".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::GameObj;

    #[test]
    fn banks_are_big_and_clean() {
        assert!(PROMPTS.len() >= 200 && ANSWERS.len() >= 200);
        for (p, t) in PROMPTS {
            assert_eq!(p.matches("___").count(), 1, "{p}");
            assert!(!t.is_empty());
        }
        let mut a: Vec<&str> = ANSWERS.iter().map(|x| x.0).collect();
        a.sort();
        a.dedup();
        assert_eq!(a.len(), ANSWERS.len());
        assert!(ANSWERS.iter().all(|x| (1..=3).contains(&x.2)));
    }

    #[test]
    fn deal_conserves_cards() {
        let g = Gura::with_seed(2);
        assert_eq!(g.hands[0].len(), HAND);
        assert_eq!(g.hands[1].len(), HAND);
        assert_eq!(g.deck.len() + 2 * HAND, ANSWERS.len());
    }

    #[test]
    fn round_scores_exactly_one_point() {
        let mut g = Gura::with_seed(3);
        g.apply(&Move::Play(0));
        g.apply(&Move::Shuffle);
        assert_eq!(g.table.len(), 3);
        assert_eq!(g.table.iter().filter(|t| t.1 == 0).count(), 1);
        g.apply(&Move::Pick(1));
        g.apply(&Move::Tally);
        assert_eq!(g.pts[0] + g.pts[1], 1);
        assert_eq!(g.hands[0].len(), HAND);
        assert_eq!(g.judge(), Player::Two);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..20u64 {
            let mut g: Box<dyn GameObj> = Box::new(Gura::with_seed(seed));
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() && n < 1000 {
                let m = g.legal_moves();
                g.play(&m[rng.below(m.len())]).unwrap();
                n += 1;
            }
            assert!(g.outcome().is_some());
        }
    }

    #[test]
    fn judge_ai_does_not_see_owners() {
        let mut g = Gura::with_seed(6);
        g.apply(&Move::Play(0));
        g.apply(&Move::Shuffle);
        let mut h = g.clone();
        for t in h.table.iter_mut() {
            t.1 = 1 - t.1;
        }
        let p = Game::to_move(&g);
        for i in 0..3 {
            let (mut a, mut b) = (g.clone(), h.clone());
            a.apply(&Move::Pick(i));
            b.apply(&Move::Pick(i));
            assert_eq!(Game::evaluate(&a, p), Game::evaluate(&b, p));
        }
    }

    #[test]
    fn ai_plays_best_fitting_answer() {
        let g = Gura::with_seed(8);
        let best = (0..HAND)
            .map(|h| fit(g.prompt, g.hands[1][h]))
            .max()
            .unwrap();
        let mv = GameObj::ai_move(&g, 1).unwrap();
        let h: usize = mv.trim_start_matches("joaca ").parse::<usize>().unwrap() - 1;
        assert_eq!(fit(g.prompt, g.hands[1][h]), best);
    }
}
