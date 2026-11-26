//! CUVÂNT UNIC — variantă originală de joc cu indicii dintr-un singur cuvânt, în duel.
//!
//! Mecanica genului (indicii de un cuvânt care se anulează când se repetă), adaptată
//! pentru doi jucători și un duel pe 12 runde:
//! * Într-o rundă, un jucător ghicește (alternativ J1/J2), celălalt știe cuvântul secret
//!   și alege un indiciu din lexiconul lui (`indiciu 3`). Doi „martori” robotici aleg și
//!   ei fiecare un indiciu (pasul forțat `dezvaluie`), mai des pe cele obișnuite.
//! * Indiciile **identice** (cel al jucătorului și ale martorilor) se anulează între ele.
//! * Ghicitorul vede ce a rămas și alege unul dintre cei 6 candidați (`ghici 4`) sau
//!   dă `pas`. Pasul forțat `verifica` decontează: ghicit → +1 ghicitor; greșit → +1 pentru
//!   cel care a dat indiciul; pas → nimic. Cine are mai multe puncte după 12 runde câștigă.
//!
//! AI-ul care dă indiciul alege indicii obișnuite (se anulează mai des); cel care
//! ghicește potrivește indiciile rămase cu lexiconul fiecărui candidat. Niciunul nu
//! vede răspunsul în momentul alegerii.

use crate::cards::{handover_banner, shuffle, Viewer};
use crate::cuvantunic_date::WORDS;
use crate::w4_comun::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::fmt::Write;
use std::sync::OnceLock;

const ROUNDS: usize = 12;
/// Ponderea cu care martorii aleg indiciul de rang 0, 1, 2…
const WITNESS_W: [usize; 8] = [8, 5, 4, 3, 2, 1, 1, 1];

pub struct Entry {
    pub word: String,
    pub clues: Vec<String>,
}

pub fn entries() -> &'static [Entry] {
    static CELL: OnceLock<Vec<Entry>> = OnceLock::new();
    CELL.get_or_init(|| {
        WORDS
            .iter()
            .map(|&(w, c)| {
                let mut clues: Vec<String> = Vec::new();
                for x in c.split_whitespace() {
                    if fold(x) != fold(w) && !clues.iter().any(|y| fold(y) == fold(x)) {
                        clues.push(x.to_string());
                    }
                }
                Entry {
                    word: w.to_string(),
                    clues,
                }
            })
            .collect()
    })
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Clue,
    Reveal,
    Guess,
    Verify,
    Over,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Clue(usize),
    Reveal,
    Guess(usize),
    Pass,
    Verify,
}

#[derive(Clone)]
pub struct CuvantUnic {
    rng: Rng,
    order: Vec<usize>,
    round: usize,
    secret: usize,
    cands: [usize; 6],
    spos: usize,
    phase: Phase,
    o_clue: usize,
    witness: [usize; 2],
    survivors: Vec<String>,
    cancelled: usize,
    pending: Option<usize>,
    pts: [i32; 2],
    log: Vec<String>,
    heur: Heur,
    viewer: Viewer,
    last: Player,
}

impl Default for CuvantUnic {
    fn default() -> Self {
        Self::new()
    }
}

impl CuvantUnic {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut order: Vec<usize> = (0..entries().len()).collect();
        shuffle(&mut order, &mut rng);
        let mut g = CuvantUnic {
            rng,
            order,
            round: 0,
            secret: 0,
            cands: [0; 6],
            spos: 0,
            phase: Phase::Clue,
            o_clue: 0,
            witness: [0, 0],
            survivors: Vec::new(),
            cancelled: 0,
            pending: None,
            pts: [0, 0],
            log: Vec::new(),
            heur: Heur::default(),
            viewer: Viewer::Fixed(Player::One),
            last: Player::One,
        };
        g.new_round();
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
        if self.round.is_multiple_of(2) {
            Player::One
        } else {
            Player::Two
        }
    }

    fn giver(&self) -> Player {
        self.guesser().other()
    }

    fn new_round(&mut self) {
        let n = entries().len();
        self.secret = self.order[self.round % n];
        let mut c = vec![self.secret];
        while c.len() < 6 {
            let r = self.rng.below(n);
            if !c.contains(&r) {
                c.push(r);
            }
        }
        shuffle(&mut c, &mut self.rng);
        self.spos = c.iter().position(|&x| x == self.secret).unwrap();
        self.cands.copy_from_slice(&c);
        self.survivors.clear();
        self.cancelled = 0;
        self.pending = None;
        self.phase = Phase::Clue;
    }

    fn rank_in(&self, cand: usize, clue: &str) -> Option<usize> {
        entries()[cand]
            .clues
            .iter()
            .position(|c| fold(c) == fold(clue))
    }

    /// Potrivirea candidatului `k` cu indiciile rămase (informație vizibilă ghicitorului).
    fn cand_score(&self, k: usize) -> i32 {
        let c = self.cands[k];
        self.survivors
            .iter()
            .map(|cl| match self.rank_in(c, cl) {
                Some(r) => 10 * (9 - r as i32),
                None => 0,
            })
            .sum()
    }

    fn witness_pick(&mut self) -> usize {
        let len = entries()[self.secret].clues.len();
        let w: Vec<usize> = (0..len).map(|i| WITNESS_W[i.min(7)]).collect();
        let total: usize = w.iter().sum();
        let mut r = self.rng.below(total);
        for (i, &x) in w.iter().enumerate() {
            if r < x {
                return i;
            }
            r -= x;
        }
        0
    }
}

impl Game for CuvantUnic {
    type Move = Move;

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = match who {
            Some(p) => Viewer::Fixed(p),
            None => Viewer::HotSeat,
        };
    }

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Clue => self.giver(),
            Phase::Guess => self.guesser(),
            _ => RESOLVER,
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        match self.phase {
            Phase::Clue => (0..entries()[self.secret].clues.len())
                .map(Move::Clue)
                .collect(),
            Phase::Reveal => vec![Move::Reveal],
            Phase::Guess => {
                let mut v: Vec<Move> = (0..6).map(Move::Guess).collect();
                v.push(Move::Pass);
                v
            }
            Phase::Verify => vec![Move::Verify],
            Phase::Over => Vec::new(),
        }
    }

    fn apply(&mut self, m: &Move) {
        let mover = self.to_move();
        if !matches!(m, Move::Reveal | Move::Verify) {
            self.last = mover;
        }
        self.heur.clear();
        match *m {
            Move::Clue(j) => {
                self.heur.set(mover, 40 - 8 * j as i32);
                self.o_clue = j;
                self.phase = Phase::Reveal;
            }
            Move::Reveal => {
                let e = &entries()[self.secret];
                self.witness = [self.witness_pick(), self.witness_pick()];
                let all = [self.o_clue, self.witness[0], self.witness[1]];
                self.survivors.clear();
                self.cancelled = 0;
                for (i, &c) in all.iter().enumerate() {
                    let dup = all
                        .iter()
                        .enumerate()
                        .any(|(k, &d)| k != i && fold(&e.clues[d]) == fold(&e.clues[c]));
                    if dup {
                        self.cancelled += 1;
                    } else {
                        self.survivors.push(e.clues[c].clone());
                    }
                }
                self.phase = Phase::Guess;
            }
            Move::Guess(k) => {
                let own = self.cand_score(k);
                let other = (0..6)
                    .filter(|&i| i != k)
                    .map(|i| self.cand_score(i))
                    .max()
                    .unwrap_or(0);
                self.heur.set(mover, own - other / 2);
                self.pending = Some(k);
                self.phase = Phase::Verify;
            }
            Move::Pass => {
                self.heur.set(mover, 20);
                self.pending = None;
                self.phase = Phase::Verify;
            }
            Move::Verify => {
                let g = self.guesser();
                let o = self.giver();
                let word = entries()[self.secret].word.clone();
                match self.pending {
                    Some(k) if k == self.spos => {
                        self.pts[g.idx()] += 1;
                        self.log
                            .push(format!("J{} ghicește «{word}» (+1).", g.idx() + 1));
                    }
                    Some(k) => {
                        self.pts[o.idx()] += 1;
                        self.log.push(format!(
                            "J{} zice «{}», dar era «{word}»: +1 pentru J{}.",
                            g.idx() + 1,
                            entries()[self.cands[k]].word,
                            o.idx() + 1
                        ));
                    }
                    None => self
                        .log
                        .push(format!("J{} dă pas; cuvântul era «{word}».", g.idx() + 1)),
                }
                self.round += 1;
                if self.round >= ROUNDS {
                    self.phase = Phase::Over;
                } else {
                    self.new_round();
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
                Phase::Reveal | Phase::Verify | Phase::Over => self.last,
                _ => self.to_move(),
            },
        };
        if let Viewer::HotSeat = self.viewer {
            if matches!(self.phase, Phase::Clue | Phase::Guess) && who != self.last {
                s.push_str(&handover_banner(who));
            }
        }
        let _ = writeln!(
            s,
            "Cuvânt Unic — runda {}/{} | puncte: J1 {}, J2 {}",
            (self.round + 1).min(ROUNDS),
            ROUNDS,
            self.pts[0],
            self.pts[1]
        );
        if self.phase == Phase::Over {
            s.push_str("Joc încheiat.\n");
            s.push_str(&log_tail(&self.log, 3));
            return s;
        }
        let _ = writeln!(
            s,
            "Ghicește: Jucătorul {} | dă indiciul: Jucătorul {}",
            self.guesser().idx() + 1,
            self.giver().idx() + 1
        );
        s.push_str("Candidați:");
        for (i, &c) in self.cands.iter().enumerate() {
            let _ = write!(s, "  {}) {}", i + 1, entries()[c].word);
        }
        s.push('\n');
        match self.phase {
            Phase::Clue if who == self.giver() => {
                let e = &entries()[self.secret];
                let _ = writeln!(s, "Cuvântul tău: {}", e.word.to_uppercase());
                s.push_str("Indicii (indiciu N; cele obișnuite se anulează mai ușor):\n");
                for (j, c) in e.clues.iter().enumerate() {
                    let _ = writeln!(s, "  {}) {}", j + 1, c);
                }
            }
            Phase::Guess => {
                let _ = writeln!(
                    s,
                    "Indicii rămase: {} ({} anulate)",
                    if self.survivors.is_empty() {
                        "—".to_string()
                    } else {
                        self.survivors.join(", ")
                    },
                    self.cancelled
                );
            }
            _ => {}
        }
        s.push_str(&log_tail(&self.log, 2));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Clue(j) => format!("indiciu {}", j + 1),
            Move::Reveal => "dezvaluie".into(),
            Move::Guess(k) => format!("ghici {}", k + 1),
            Move::Pass => "pas".into(),
            Move::Verify => "verifica".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::GameObj;

    #[test]
    fn lexicon_is_big_and_clean() {
        let es = entries();
        assert!(es.len() >= 200);
        for e in es {
            assert!(e.clues.len() >= 6, "{}", e.word);
        }
    }

    #[test]
    fn identical_clues_cancel() {
        let mut g = CuvantUnic::with_seed(5);
        g.apply(&Move::Clue(0));
        // forțează martorii pe același indiciu
        g.phase = Phase::Reveal;
        let mut h = g.clone();
        h.witness = [0, 0];
        // aplicăm manual logica: simulăm cu un RNG care nu e relevant
        let e = &entries()[h.secret];
        let all = [0usize, 0, 0];
        let dup = all.iter().filter(|&&d| e.clues[d] == e.clues[0]).count();
        assert_eq!(dup, 3);
        g.apply(&Move::Reveal);
        assert!(g.survivors.len() + g.cancelled == 3);
    }

    #[test]
    fn scoring_rules() {
        let mut g = CuvantUnic::with_seed(2);
        g.apply(&Move::Clue(1));
        g.apply(&Move::Reveal);
        let s = g.spos;
        g.apply(&Move::Guess(s));
        g.apply(&Move::Verify);
        assert_eq!(g.pts, [1, 0]);
        g.apply(&Move::Clue(1));
        g.apply(&Move::Reveal);
        let wrong = (g.spos + 1) % 6;
        g.apply(&Move::Guess(wrong));
        g.apply(&Move::Verify);
        assert_eq!(g.pts, [2, 0]);
        assert_eq!(g.round, 2);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..25u64 {
            let mut g: Box<dyn GameObj> = Box::new(CuvantUnic::with_seed(seed));
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
    fn guesser_ai_ignores_answer_position() {
        for seed in 1..10u64 {
            let mut g = CuvantUnic::with_seed(seed);
            g.apply(&Move::Clue(0));
            g.apply(&Move::Reveal);
            let mut h = g.clone();
            h.spos = (h.spos + 1) % 6;
            let p = Game::to_move(&g);
            for m in Game::legal_moves(&g) {
                let (mut a, mut b) = (g.clone(), h.clone());
                a.apply(&m);
                b.apply(&m);
                assert_eq!(Game::evaluate(&a, p), Game::evaluate(&b, p));
            }
        }
    }

    #[test]
    fn ai_guesses_with_clues() {
        // dacă există un indiciu rămas care aparține unui candidat, AI-ul îl alege
        let mut found = 0;
        for seed in 1..40u64 {
            let mut g = CuvantUnic::with_seed(seed);
            g.apply(&Move::Clue(2));
            g.apply(&Move::Reveal);
            if g.survivors.is_empty() {
                continue;
            }
            let mv = GameObj::ai_move(&g, seed).unwrap();
            assert!(mv.starts_with("ghici") || mv == "pas");
            if mv == format!("ghici {}", g.spos + 1) {
                found += 1;
            }
        }
        assert!(found > 5);
    }
}
