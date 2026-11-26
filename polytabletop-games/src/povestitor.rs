//! POVESTITORUL — variantă originală de joc de povestit cu cărți-simbol (două persoane).
//!
//! Reguli (mecanica genului, cărțile sunt ale noastre — vezi `povestitor_date`):
//! * Fiecare are 6 cărți-simbol (descrieri). Povestitorul (alternativ J1/J2) alege o
//!   carte și unul dintre indiciile ei predefinite: `spune 3a`.
//! * Celălalt jucător pune o **momeală** din mâna lui (`momeala 2`), apoi pasul forțat
//!   `amesteca` adaugă 3 cărți-fantomă din pachet și amestecă cele 5 cărți.
//! * Ghicitorul votează (`vot N`) cartea pe care crede că a spus-o povestitorul (nu pe a lui).
//!   Un al doilea votant, **Publicul**, votează după potrivirea cuvintelor-cheie cu
//!   indiciul (cu o doză de întâmplare), în pasul forțat `dezvaluie`.
//! * Punctaj: dacă exact unul dintre cei doi votanți găsește cartea povestitorului,
//!   povestitorul ia 3 și ghicitorul (dacă a găsit-o) 3; dacă au găsit-o amândoi sau
//!   niciunul, povestitorul ia 0 și ghicitorul 2. Ghicitorul mai ia 1 dacă Publicul
//!   a votat momeala lui. Se joacă până la 12 puncte sau 14 runde.
//!
//! AI-ul vede doar indiciul public, cărțile de pe masă și propria mână.

use crate::cards::{handover_banner, shuffle, Viewer};
use crate::povestitor_date::CARDS;
use crate::w4_comun::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::fmt::Write;
use std::sync::OnceLock;

pub struct Card {
    pub desc: String,
    /// Rădăcinile asociate cărții (etichete + cuvintele descrierii).
    pub assoc: Vec<String>,
    pub clues: Vec<(String, Vec<String>)>,
}

pub fn cards() -> &'static [Card] {
    static CELL: OnceLock<Vec<Card>> = OnceLock::new();
    CELL.get_or_init(|| {
        CARDS
            .iter()
            .map(|&(desc, tags, clues)| {
                let mut assoc = tokens(tags);
                for t in tokens(desc) {
                    if !assoc.contains(&t) {
                        assoc.push(t);
                    }
                }
                Card {
                    desc: desc.to_string(),
                    assoc,
                    clues: clues
                        .split(';')
                        .map(|c| (c.to_string(), tokens(c)))
                        .collect(),
                }
            })
            .collect()
    })
}

fn overlap(clue: &[String], card: &Card) -> i32 {
    clue.iter().filter(|t| card.assoc.contains(t)).count() as i32
}

const WIN_SCORE: i32 = 12;
const MAX_ROUNDS: usize = 14;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Tell,
    Decoy,
    Shuffle,
    Vote,
    Reveal,
    Over,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Tell(usize, usize),
    Decoy(usize),
    Shuffle,
    Vote(usize),
    Reveal,
}

#[derive(Clone)]
pub struct Povestitor {
    rng: Rng,
    deck: Vec<usize>,
    hands: [Vec<usize>; 2],
    teller: Player,
    phase: Phase,
    told: Option<(usize, usize)>,
    decoy: Option<usize>,
    /// (carte, 0 povestitor / 1 ghicitor / 2 fantomă)
    table: Vec<(usize, u8)>,
    vote: usize,
    pts: [i32; 2],
    round: usize,
    log: Vec<String>,
    heur: Heur,
    viewer: Viewer,
    last: Player,
}

impl Default for Povestitor {
    fn default() -> Self {
        Self::new()
    }
}

impl Povestitor {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck: Vec<usize> = (0..cards().len()).collect();
        shuffle(&mut deck, &mut rng);
        let mut g = Povestitor {
            rng,
            deck,
            hands: [Vec::new(), Vec::new()],
            teller: Player::One,
            phase: Phase::Tell,
            told: None,
            decoy: None,
            table: Vec::new(),
            vote: 0,
            pts: [0, 0],
            round: 0,
            log: Vec::new(),
            heur: Heur::default(),
            viewer: Viewer::Fixed(Player::One),
            last: Player::One,
        };
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

    fn guesser(&self) -> Player {
        self.teller.other()
    }

    fn refill(&mut self) {
        for p in 0..2 {
            while self.hands[p].len() < 6 {
                match self.deck.pop() {
                    Some(c) => self.hands[p].push(c),
                    None => break,
                }
            }
        }
    }

    fn clue_tokens(&self) -> Vec<String> {
        match self.told {
            Some((c, j)) => cards()[c].clues[j].1.clone(),
            None => Vec::new(),
        }
    }

    fn clue_text(&self) -> String {
        match self.told {
            Some((c, j)) => cards()[c].clues[j].0.clone(),
            None => String::new(),
        }
    }

    fn finish_round(&mut self) {
        self.round += 1;
        self.teller = self.teller.other();
        self.told = None;
        self.decoy = None;
        self.table.clear();
        self.refill();
        if self.pts.iter().any(|&p| p >= WIN_SCORE) || self.round >= MAX_ROUNDS {
            self.phase = Phase::Over;
        } else {
            self.phase = Phase::Tell;
        }
    }
}

impl Game for Povestitor {
    type Move = Move;

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = match who {
            Some(p) => Viewer::Fixed(p),
            None => Viewer::HotSeat,
        };
    }

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Tell => self.teller,
            Phase::Decoy | Phase::Vote => self.guesser(),
            _ => RESOLVER,
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        match self.phase {
            Phase::Tell => {
                let mut v = Vec::new();
                for (h, &c) in self.hands[self.teller.idx()].iter().enumerate() {
                    for j in 0..cards()[c].clues.len() {
                        v.push(Move::Tell(h, j));
                    }
                }
                v
            }
            Phase::Decoy => (0..self.hands[self.guesser().idx()].len())
                .map(Move::Decoy)
                .collect(),
            Phase::Shuffle => vec![Move::Shuffle],
            Phase::Vote => (0..self.table.len())
                .filter(|&i| self.table[i].1 != 1)
                .map(Move::Vote)
                .collect(),
            Phase::Reveal => vec![Move::Reveal],
            Phase::Over => Vec::new(),
        }
    }

    fn apply(&mut self, m: &Move) {
        let mover = self.to_move();
        if !matches!(m, Move::Shuffle | Move::Reveal) {
            self.last = mover;
        }
        self.heur.clear();
        match *m {
            Move::Tell(h, j) => {
                let t = self.teller.idx();
                let card = self.hands[t][h];
                let clue = &cards()[card].clues[j].1;
                let own = overlap(clue, &cards()[card]);
                let amb = self.hands[t]
                    .iter()
                    .filter(|&&c| c != card && overlap(clue, &cards()[c]) > 0)
                    .count() as i32;
                let v = if own == 0 {
                    0
                } else {
                    100 * own.min(2) - 40 * (amb - 1).abs() + 10 * j as i32
                };
                self.heur.set(mover, v);
                self.hands[t].remove(h);
                self.told = Some((card, j));
                self.log.push(format!(
                    "Povestitorul (J{}): «{}»",
                    t + 1,
                    cards()[card].clues[j].0
                ));
                self.phase = Phase::Decoy;
            }
            Move::Decoy(h) => {
                let g = self.guesser().idx();
                let card = self.hands[g][h];
                let ct = self.clue_tokens();
                self.heur.set(mover, 100 * overlap(&ct, &cards()[card]));
                self.hands[g].remove(h);
                self.decoy = Some(card);
                self.phase = Phase::Shuffle;
            }
            Move::Shuffle => {
                let (tc, _) = self.told.expect("indiciu");
                self.table = vec![(tc, 0), (self.decoy.expect("momeală"), 1)];
                for _ in 0..3 {
                    if let Some(c) = self.deck.pop() {
                        self.table.push((c, 2));
                    }
                }
                shuffle(&mut self.table, &mut self.rng);
                self.phase = Phase::Vote;
            }
            Move::Vote(i) => {
                let ct = self.clue_tokens();
                let v = 100 * overlap(&ct, &cards()[self.table[i].0]);
                self.heur.set(mover, v);
                self.vote = i;
                self.phase = Phase::Reveal;
            }
            Move::Reveal => {
                let ct = self.clue_tokens();
                let t = self.teller;
                let g = self.guesser();
                let spos = self.table.iter().position(|c| c.1 == 0).unwrap();
                let dpos = self.table.iter().position(|c| c.1 == 1).unwrap();
                // votul Publicului: ponderat după potrivire, cu întâmplare
                let w: Vec<usize> = self
                    .table
                    .iter()
                    .map(|&(c, _)| 40 + 120 * overlap(&ct, &cards()[c]) as usize)
                    .collect();
                let total: usize = w.iter().sum();
                let mut r = self.rng.below(total);
                let mut pv = 0;
                for (i, &x) in w.iter().enumerate() {
                    if r < x {
                        pv = i;
                        break;
                    }
                    r -= x;
                }
                let g_ok = self.vote == spos;
                let p_ok = pv == spos;
                let found = g_ok as u8 + p_ok as u8;
                let (mut ts, mut gs) = (0, 0);
                if found == 1 {
                    ts = 3;
                    if g_ok {
                        gs = 3;
                    }
                } else {
                    gs = 2;
                }
                if pv == dpos {
                    gs += 1;
                }
                self.pts[t.idx()] += ts;
                self.pts[g.idx()] += gs;
                self.log.push(format!(
                    "Cartea povestitorului: {}) «{}». Ghicitorul a votat {}, Publicul {}. Puncte: J{} +{}, J{} +{}.",
                    spos + 1,
                    cards()[self.table[spos].0].desc,
                    self.vote + 1,
                    pv + 1,
                    t.idx() + 1,
                    ts,
                    g.idx() + 1,
                    gs
                ));
                self.finish_round();
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
                Phase::Shuffle | Phase::Reveal | Phase::Over => self.last,
                _ => self.to_move(),
            },
        };
        if let Viewer::HotSeat = self.viewer {
            if matches!(self.phase, Phase::Tell | Phase::Decoy | Phase::Vote) && who != self.last {
                s.push_str(&handover_banner(who));
            }
        }
        let _ = writeln!(
            s,
            "Povestitorul — runda {}/{} | puncte: J1 {}, J2 {} (până la {})",
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
            "Povestitor: Jucătorul {} | ghicitor: Jucătorul {} | pachet: {} cărți",
            self.teller.idx() + 1,
            self.guesser().idx() + 1,
            self.deck.len()
        );
        if self.told.is_some() {
            let _ = writeln!(s, "Indiciul povestitorului: «{}»", self.clue_text());
        }
        match self.phase {
            Phase::Tell if who == self.teller => {
                s.push_str("Mâna ta (alege carte + indiciu: spune 3a):\n");
                for (h, &c) in self.hands[who.idx()].iter().enumerate() {
                    let _ = writeln!(s, "  {}) {}", h + 1, cards()[c].desc);
                    for (j, (txt, _)) in cards()[c].clues.iter().enumerate() {
                        let _ = writeln!(s, "       {}) «{}»", (b'a' + j as u8) as char, txt);
                    }
                }
            }
            Phase::Decoy if who == self.guesser() => {
                s.push_str("Mâna ta (alege momeala: momeala 2):\n");
                for (h, &c) in self.hands[who.idx()].iter().enumerate() {
                    let _ = writeln!(s, "  {}) {}", h + 1, cards()[c].desc);
                }
            }
            Phase::Vote => {
                s.push_str("Pe masă (nu votezi cartea marcată ca fiind a ta):\n");
                for (i, &(c, o)) in self.table.iter().enumerate() {
                    let mine = o == 1 && who == self.guesser();
                    let _ = writeln!(
                        s,
                        "  {}) {}{}",
                        i + 1,
                        cards()[c].desc,
                        if mine { "   (momeala ta)" } else { "" }
                    );
                }
            }
            _ => {}
        }
        s.push_str(&log_tail(&self.log, 2));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Tell(h, j) => format!("spune {}{}", h + 1, (b'a' + *j as u8) as char),
            Move::Decoy(h) => format!("momeala {}", h + 1),
            Move::Shuffle => "amesteca".into(),
            Move::Vote(i) => format!("vot {}", i + 1),
            Move::Reveal => "dezvaluie".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::GameObj;

    #[test]
    fn bank_is_big_and_clues_well_formed() {
        let cs = cards();
        assert!(cs.len() >= 200, "{}", cs.len());
        for c in cs {
            assert!(!c.clues.is_empty() && c.clues.len() <= 3);
            assert!(c.assoc.len() >= 3);
        }
        let mut d: Vec<&str> = cs.iter().map(|c| c.desc.as_str()).collect();
        d.sort();
        d.dedup();
        assert_eq!(d.len(), cs.len(), "descrieri duplicate");
        // indiciul direct (a) se potrivește cu propria carte
        let ok = cs.iter().filter(|c| overlap(&c.clues[0].1, c) > 0).count();
        assert!(ok * 100 >= cs.len() * 95);
    }

    #[test]
    fn deal_and_conservation() {
        let g = Povestitor::with_seed(3);
        assert_eq!(g.hands[0].len(), 6);
        assert_eq!(g.hands[1].len(), 6);
        assert_eq!(g.deck.len() + 12, cards().len());
    }

    #[test]
    fn round_flow_and_scoring_bounds() {
        let mut g = Povestitor::with_seed(5);
        let before = g.deck.len();
        g.apply(&Move::Tell(0, 0));
        g.apply(&Move::Decoy(0));
        g.apply(&Move::Shuffle);
        assert_eq!(g.table.len(), 5);
        assert!(Game::legal_moves(&g).iter().all(|m| match m {
            Move::Vote(i) => g.table[*i].1 != 1,
            _ => false,
        }));
        g.apply(&Move::Vote(Game::legal_moves(&g).len().min(1)));
        g.apply(&Move::Reveal);
        assert_eq!(g.hands[0].len(), 6);
        assert_eq!(g.hands[1].len(), 6);
        assert_eq!(g.deck.len(), before - 5);
        assert!(g.pts[0] + g.pts[1] <= 7);
        assert_eq!(g.teller, Player::Two);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..15u64 {
            let mut g: Box<dyn GameObj> = Box::new(Povestitor::with_seed(seed));
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() && n < 3000 {
                let m = g.legal_moves();
                g.play(&m[rng.below(m.len())]).unwrap();
                n += 1;
            }
            assert!(g.outcome().is_some());
        }
    }

    #[test]
    fn ai_vote_ignores_owner_labels() {
        // evaluarea unui vot nu depinde de cine a pus cartea (informație ascunsă)
        let mut g = Povestitor::with_seed(9);
        g.apply(&Move::Tell(0, 0));
        g.apply(&Move::Decoy(0));
        g.apply(&Move::Shuffle);
        let mut h = g.clone();
        for t in h.table.iter_mut() {
            t.1 = (t.1 + 1) % 3; // alte etichete de proprietar
        }
        let p = Game::to_move(&g);
        for i in 0..5 {
            let (mut a, mut b) = (g.clone(), h.clone());
            a.apply(&Move::Vote(i));
            b.apply(&Move::Vote(i));
            assert_eq!(Game::evaluate(&a, p), Game::evaluate(&b, p));
        }
    }

    #[test]
    fn ai_teller_prefers_matching_clue() {
        let g = Povestitor::with_seed(11);
        let mv = GameObj::ai_move(&g, 3).unwrap();
        assert!(mv.starts_with("spune"));
    }
}
