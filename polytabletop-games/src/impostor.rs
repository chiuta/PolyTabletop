//! IMPOSTORUL DIN GRILĂ — variantă originală de joc cu impostor (un „cameleon”), pe
//! patru locuri: Jucătorul 1, Jucătorul 2 și doi roboți de masă (Robot A și Robot B).
//!
//! * Pe masă e o grilă de 16 cuvinte dintr-un subiect (`impostor_date`). Un cuvânt
//!   este secret și toți îl știu, **mai puțin impostorul** (ales la întâmplare dintre
//!   cei patru; tu îți vezi rolul în `render`).
//! * Pe rând, fiecare spune un indiciu dintr-un singur cuvânt. Cine știe cuvântul
//!   alege dintre indiciile lui (`indiciu lup`); impostorul alege, la nimereală, un
//!   indiciu „de trecut” din grilă, ascultând ce s-a spus. Un indiciu nu se repetă.
//! * Apoi J1 și J2 votează (`vot N`: 1 = J1, 2 = J2, 3 = Robot A, 4 = Robot B); roboții
//!   votează în pasul forțat `numara`. Cine are cele mai multe voturi (fără egalitate)
//!   e acuzat. Dacă impostorul e prins, are o ultimă șansă să ghicească cuvântul
//!   (`ghici cuvant`, apoi `verifica`).
//! * Puncte (doar J1 și J2): fiecare om care a votat impostorul +1; impostor neprins +2;
//!   impostor prins care nu ghicește: fiecare om nevinovat +2; prins care ghicește: +1
//!   pentru impostor. 8 runde.
//!
//! AI-ul folosește numai ce știe rolul lui: cine știe cuvântul se uită la indicii, iar
//! impostorul deduce cuvântul din indiciile spuse (nu citește cuvântul secret).

use crate::cards::{handover_banner, shuffle, Viewer};
use crate::impostor_date::TOPICS;
use crate::w4_comun::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::fmt::Write;

const ROUNDS: usize = 8;
const SEATS: [usize; 4] = [0, 2, 1, 3];
const NAMES: [&str; 4] = ["J1", "J2", "Robot A", "Robot B"];

fn clues_of(topic: usize, word: usize) -> Vec<&'static str> {
    TOPICS[topic].1[word].1.split_whitespace().collect()
}

fn word_of(topic: usize, word: usize) -> &'static str {
    TOPICS[topic].1[word].0
}

fn is_human(p: usize) -> bool {
    p < 2
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Clue,
    VoteOne,
    VoteTwo,
    Tally,
    ChamGuess,
    Verify,
    Over,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Clue(usize),
    Vote(usize),
    Tally,
    Guess(usize),
    Verify,
}

#[derive(Clone)]
pub struct Impostor {
    rng: Rng,
    topics: Vec<usize>,
    round: usize,
    topic: usize,
    /// ordinea cuvintelor în grilă (indici în subiect)
    grid: [usize; 16],
    /// poziția în grilă a cuvântului secret
    secret: usize,
    cham: usize,
    start: usize,
    spoken: Vec<(usize, String)>,
    votes: [Option<usize>; 4],
    phase: Phase,
    pending: usize,
    pts: [i32; 2],
    log: Vec<String>,
    heur: Heur,
    viewer: Viewer,
    last: Player,
}

impl Default for Impostor {
    fn default() -> Self {
        Self::new()
    }
}

impl Impostor {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut topics: Vec<usize> = (0..TOPICS.len()).collect();
        shuffle(&mut topics, &mut rng);
        let mut g = Impostor {
            rng,
            topics,
            round: 0,
            topic: 0,
            grid: [0; 16],
            secret: 0,
            cham: 0,
            start: 0,
            spoken: Vec::new(),
            votes: [None; 4],
            phase: Phase::Clue,
            pending: 0,
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

    fn new_round(&mut self) {
        self.topic = self.topics[self.round % self.topics.len()];
        let mut g: Vec<usize> = (0..16).collect();
        shuffle(&mut g, &mut self.rng);
        self.grid.copy_from_slice(&g);
        self.secret = self.rng.below(16);
        self.cham = self.rng.below(4);
        self.start = self.rng.below(4);
        self.spoken.clear();
        self.votes = [None; 4];
        self.advance();
    }

    fn seat(&self, k: usize) -> usize {
        SEATS[(self.start + k) % 4]
    }

    /// Roboții vorbesc până la următorul om sau până se termină runda de indicii.
    fn advance(&mut self) {
        loop {
            if self.spoken.len() >= 4 {
                self.phase = Phase::VoteOne;
                return;
            }
            let who = self.seat(self.spoken.len());
            if is_human(who) {
                self.phase = Phase::Clue;
                return;
            }
            let c = self.bot_clue(who);
            self.spoken.push((who, c));
        }
    }

    fn used(&self, c: &str) -> bool {
        self.spoken.iter().any(|(_, s)| fold(s) == fold(c))
    }

    fn word_has(&self, w: usize, clue: &str) -> bool {
        clues_of(self.topic, w)
            .iter()
            .any(|c| fold(c) == fold(clue))
    }

    /// Cât de bine explică indiciile spuse cuvântul din grilă `pos` (informație publică).
    fn explain(&self, pos: usize) -> i32 {
        let w = self.grid[pos];
        self.spoken
            .iter()
            .filter(|(_, c)| self.word_has(w, c))
            .count() as i32
    }

    fn bot_clue(&mut self, who: usize) -> String {
        if who != self.cham {
            let all = clues_of(self.topic, self.grid[self.secret]);
            let avail: Vec<&str> = all.into_iter().filter(|c| !self.used(c)).collect();
            let k = self.rng.below(avail.len().clamp(1, 3));
            avail[k.min(avail.len() - 1)].to_string()
        } else {
            // impostor-robot: cuvântul care explică cel mai bine indiciile de până acum
            let mut best = (i32::MIN, 0usize);
            for pos in 0..16 {
                let v = 10 * self.explain(pos) + self.rng.below(5) as i32;
                if v > best.0 {
                    best = (v, pos);
                }
            }
            let w = self.grid[best.1];
            let mut cl: Vec<&str> = clues_of(self.topic, w)
                .into_iter()
                .filter(|c| !self.used(c))
                .collect();
            if cl.is_empty() {
                cl.push("nimic");
            }
            cl[0].to_string()
        }
    }

    /// Opțiunile de indiciu ale unui om (impostorul nu știe cuvântul).
    fn clue_options(&self, who: usize) -> Vec<String> {
        let mut v: Vec<String> = Vec::new();
        if who == self.cham {
            for pos in 0..16 {
                let c = clues_of(self.topic, self.grid[pos])[0];
                if !self.used(c) && !v.iter().any(|x| fold(x) == fold(c)) {
                    v.push(c.to_string());
                }
            }
        } else {
            for c in clues_of(self.topic, self.grid[self.secret]) {
                if !self.used(c) && !v.iter().any(|x| fold(x) == fold(c)) {
                    v.push(c.to_string());
                }
            }
        }
        v
    }

    fn speaker(&self) -> usize {
        self.seat(self.spoken.len().min(3))
    }

    fn clue_of(&self, who: usize) -> &str {
        self.spoken
            .iter()
            .find(|(p, _)| *p == who)
            .map(|(_, c)| c.as_str())
            .unwrap_or("")
    }

    /// Suspiciunea lui `who` față de `target`, din ce știe `who`.
    fn suspicion(&self, who: usize, target: usize) -> i32 {
        let c = self.clue_of(target);
        if who != self.cham {
            if self.word_has(self.grid[self.secret], c) {
                0
            } else {
                100
            }
        } else {
            let best = (0..16).max_by_key(|&p| self.explain(p)).unwrap_or(0);
            if self.word_has(self.grid[best], c) {
                0
            } else {
                50
            }
        }
    }

    fn end_round(&mut self) {
        self.round += 1;
        if self.round >= ROUNDS {
            self.phase = Phase::Over;
        } else {
            self.new_round();
        }
    }

    fn finish_scoring(&mut self, caught: bool, guessed: bool) {
        for h in 0..2 {
            if self.votes[h] == Some(self.cham) {
                self.pts[h] += 1;
            }
        }
        if !caught {
            if is_human(self.cham) {
                self.pts[self.cham] += 2;
            }
        } else if guessed {
            if is_human(self.cham) {
                self.pts[self.cham] += 1;
            }
        } else {
            for h in 0..2 {
                if h != self.cham {
                    self.pts[h] += 2;
                }
            }
        }
    }
}

impl Game for Impostor {
    type Move = Move;

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = match who {
            Some(p) => Viewer::Fixed(p),
            None => Viewer::HotSeat,
        };
    }

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Clue => {
                if self.speaker() == 0 {
                    Player::One
                } else {
                    Player::Two
                }
            }
            Phase::VoteOne => Player::One,
            Phase::VoteTwo => Player::Two,
            Phase::ChamGuess => {
                if self.cham == 0 {
                    Player::One
                } else {
                    Player::Two
                }
            }
            _ => RESOLVER,
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        match self.phase {
            Phase::Clue => (0..self.clue_options(self.speaker()).len())
                .map(Move::Clue)
                .collect(),
            Phase::VoteOne => (1..4).map(Move::Vote).collect(),
            Phase::VoteTwo => [0usize, 2, 3].iter().map(|&t| Move::Vote(t)).collect(),
            Phase::Tally => vec![Move::Tally],
            Phase::ChamGuess => (0..16).map(Move::Guess).collect(),
            Phase::Verify => vec![Move::Verify],
            Phase::Over => Vec::new(),
        }
    }

    fn apply(&mut self, m: &Move) {
        let mover = self.to_move();
        if !matches!(m, Move::Tally | Move::Verify) {
            self.last = mover;
        }
        self.heur.clear();
        let me = mover.idx();
        match *m {
            Move::Clue(k) => {
                let opts = self.clue_options(me);
                let clue = opts[k].clone();
                let v = if me == self.cham {
                    // impostor: indiciul „al cuvântului” care explică mai bine ce s-a spus
                    let mut best = 0;
                    for pos in 0..16 {
                        if clues_of(self.topic, self.grid[pos])[0] == clue {
                            best = 30 * self.explain(pos);
                        }
                    }
                    best
                } else {
                    let r = clues_of(self.topic, self.grid[self.secret])
                        .iter()
                        .position(|c| fold(c) == fold(&clue))
                        .unwrap_or(0) as i32;
                    60 - 15 * (r - 1).abs()
                };
                self.heur.set(mover, v);
                self.spoken.push((me, clue));
                self.advance();
            }
            Move::Vote(t) => {
                self.heur.set(mover, self.suspicion(me, t));
                self.votes[me] = Some(t);
                self.phase = if me == 0 {
                    Phase::VoteTwo
                } else {
                    Phase::Tally
                };
            }
            Move::Tally => {
                // roboții votează
                for b in 2..4usize {
                    let mut best = (i32::MIN, 0usize);
                    for t in 0..4usize {
                        if t == b {
                            continue;
                        }
                        let v = self.suspicion(b, t) + self.rng.below(40) as i32;
                        if v > best.0 {
                            best = (v, t);
                        }
                    }
                    self.votes[b] = Some(best.1);
                }
                let mut cnt = [0usize; 4];
                for v in self.votes.iter().flatten() {
                    cnt[*v] += 1;
                }
                let mx = *cnt.iter().max().unwrap();
                let top: Vec<usize> = (0..4).filter(|&i| cnt[i] == mx).collect();
                let caught = top.len() == 1 && top[0] == self.cham;
                let word = word_of(self.topic, self.grid[self.secret]);
                self.log.push(format!(
                    "Voturi: J1→{}, J2→{}, A→{}, B→{}. Impostorul era {}; cuvântul: «{}».",
                    NAMES[self.votes[0].unwrap()],
                    NAMES[self.votes[1].unwrap()],
                    NAMES[self.votes[2].unwrap()],
                    NAMES[self.votes[3].unwrap()],
                    NAMES[self.cham],
                    word
                ));
                if caught && is_human(self.cham) {
                    self.phase = Phase::ChamGuess;
                } else if caught {
                    // robotul-impostor ghicește după indiciile spuse
                    let mut best = (i32::MIN, 0usize);
                    for pos in 0..16 {
                        let v = 100 * self.explain(pos) + self.rng.below(60) as i32;
                        if v > best.0 {
                            best = (v, pos);
                        }
                    }
                    let ok = best.1 == self.secret;
                    self.finish_scoring(true, ok);
                    self.end_round();
                } else {
                    self.finish_scoring(false, false);
                    self.end_round();
                }
            }
            Move::Guess(pos) => {
                self.heur.set(mover, 100 * self.explain(pos));
                self.pending = pos;
                self.phase = Phase::Verify;
            }
            Move::Verify => {
                let ok = self.pending == self.secret;
                self.log.push(format!(
                    "Impostorul ghicește «{}»: {}.",
                    word_of(self.topic, self.grid[self.pending]),
                    if ok { "corect" } else { "greșit" }
                ));
                self.finish_scoring(true, ok);
                self.end_round();
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
                Phase::Tally | Phase::Verify | Phase::Over => self.last,
                _ => self.to_move(),
            },
        };
        if let Viewer::HotSeat = self.viewer {
            if matches!(
                self.phase,
                Phase::Clue | Phase::VoteOne | Phase::VoteTwo | Phase::ChamGuess
            ) && who != self.last
            {
                s.push_str(&handover_banner(who));
            }
        }
        let _ = writeln!(
            s,
            "Impostorul din Grilă — runda {}/{} | puncte: J1 {}, J2 {}",
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
        let _ = writeln!(s, "Subiect: {}", TOPICS[self.topic].0);
        for r in 0..4 {
            for c in 0..4 {
                let pos = r * 4 + c;
                let _ = write!(
                    s,
                    "{:<17}",
                    format!("{}) {}", pos + 1, word_of(self.topic, self.grid[pos]))
                );
            }
            s.push('\n');
        }
        let me = who.idx();
        if me == self.cham {
            s.push_str("Rolul tău: IMPOSTOR — nu știi cuvântul secret; ascultă indiciile.\n");
        } else {
            let _ = writeln!(
                s,
                "Cuvântul secret: {} (poziția {})",
                word_of(self.topic, self.grid[self.secret]).to_uppercase(),
                self.secret + 1
            );
        }
        if !self.spoken.is_empty() {
            s.push_str("Indicii:");
            for (p, c) in &self.spoken {
                let _ = write!(s, "  {}: {}", NAMES[*p], c);
            }
            s.push('\n');
        }
        match self.phase {
            Phase::Clue if self.speaker() == who.idx() => {
                let opts = self.clue_options(self.speaker());
                let _ = writeln!(
                    s,
                    "Indiciul tău (indiciu cuvânt): {}",
                    opts.iter().map(|c| fold(c)).collect::<Vec<_>>().join(", ")
                );
            }
            Phase::VoteOne | Phase::VoteTwo => {
                s.push_str(
                    "Votează impostorul: vot 1..4 (1 = J1, 2 = J2, 3 = Robot A, 4 = Robot B)\n",
                );
            }
            Phase::ChamGuess => {
                s.push_str("Ai fost prins! Ghicește cuvântul secret: ghici <cuvânt>\n")
            }
            _ => {}
        }
        s.push_str(&log_tail(&self.log, 2));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Clue(k) => format!(
                "indiciu {}",
                fold(&self.clue_options(self.to_move().idx())[*k])
            ),
            Move::Vote(t) => format!("vot {}", t + 1),
            Move::Tally => "numara".into(),
            Move::Guess(p) => format!("ghici {}", fold(word_of(self.topic, self.grid[*p]))),
            Move::Verify => "verifica".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::GameObj;

    #[test]
    fn clue_lists_have_no_folded_duplicates() {
        for (t, (_, words)) in TOPICS.iter().enumerate() {
            for w in 0..words.len() {
                let c = clues_of(t, w);
                for i in 0..c.len() {
                    for j in 0..i {
                        assert_ne!(fold(c[i]), fold(c[j]), "{} / {}", words[w].0, c[i]);
                    }
                }
            }
        }
    }

    #[test]
    fn data_is_big_and_clean() {
        let n: usize = TOPICS.iter().map(|t| t.1.len()).sum();
        assert!(n >= 200, "{n}");
        for (name, words) in TOPICS {
            assert_eq!(words.len(), 16, "{name}");
            let mut ws: Vec<String> = words.iter().map(|w| fold(w.0)).collect();
            ws.sort();
            ws.dedup();
            assert_eq!(ws.len(), 16, "{name}: cuvinte duplicate");
            let mut firsts: Vec<String> = words
                .iter()
                .map(|w| fold(w.1.split_whitespace().next().unwrap()))
                .collect();
            firsts.sort();
            firsts.dedup();
            assert_eq!(firsts.len(), 16, "{name}: primele indicii se repetă");
            for (w, c) in words.iter() {
                assert!(c.split_whitespace().count() >= 5, "{w}");
                assert!(!c.split_whitespace().any(|x| fold(x) == fold(w)), "{w}");
            }
        }
    }

    #[test]
    fn setup_roles_and_flow() {
        for seed in 1..30u64 {
            let g = Impostor::with_seed(seed);
            assert!(g.cham < 4 && g.secret < 16);
            let mut gg: Vec<usize> = g.grid.to_vec();
            gg.sort();
            assert_eq!(gg, (0..16).collect::<Vec<_>>());
            // până la primul om, roboții au vorbit deja
            assert!(g.spoken.len() <= 3);
            assert!(!Game::legal_moves(&g).is_empty());
        }
    }

    #[test]
    fn random_games_end_and_scores_bounded() {
        for seed in 1..25u64 {
            let mut g: Box<dyn GameObj> = Box::new(Impostor::with_seed(seed));
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() && n < 2000 {
                let m = g.legal_moves();
                g.play(&m[rng.below(m.len())]).unwrap();
                n += 1;
            }
            assert!(g.outcome().is_some());
        }
    }

    #[test]
    fn impostor_ai_does_not_read_secret() {
        let mut checked = 0;
        for seed in 1..80u64 {
            let mut g = Impostor::with_seed(seed);
            // vrem un joc în care Jucătorul 2 e impostor
            g.cham = 1;
            g.spoken.clear();
            g.advance();
            let mut h = g.clone();
            h.secret = (h.secret + 5) % 16;
            let p = Game::to_move(&g);
            if p != Player::Two {
                continue;
            }
            for m in Game::legal_moves(&g) {
                let (mut a, mut b) = (g.clone(), h.clone());
                a.apply(&m);
                b.apply(&m);
                assert_eq!(Game::evaluate(&a, p), Game::evaluate(&b, p));
            }
            checked += 1;
        }
        assert!(checked > 5);
    }

    #[test]
    fn caught_impostor_scoring() {
        let mut g = Impostor::with_seed(4);
        g.cham = 3; // robot
        g.votes = [Some(3), Some(3), None, None];
        g.finish_scoring(true, false);
        // fiecare om a votat impostorul (+1) și e nevinovat (+2)
        assert_eq!(g.pts, [3, 3]);
        let mut h = Impostor::with_seed(4);
        h.cham = 0;
        h.votes = [Some(1), Some(0), None, None];
        h.finish_scoring(false, false);
        assert_eq!(h.pts, [2, 1]);
    }
}
