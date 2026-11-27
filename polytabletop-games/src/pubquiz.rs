#![allow(clippy::doc_lazy_continuation)]
//! Pub quiz (Marea Britanie), serată de întrebări la cârciumă, pentru două echipe (J1 și J2).
//!
//! * 4 runde de câte 5 întrebări, fiecare rundă dintr-o altă categorie (din banca de întrebări
//!   a jocului „Cupa Cunoașterii”). La fiecare întrebare cu 4 variante răspund ambele echipe;
//!   un răspuns corect valorează 1 punct.
//! * Înaintea fiecărei runde fiecare echipă poate juca o dată pe meci **jokerul** (`joker`): toate
//!   punctele echipei din runda aceea se dublează. Se poate refuza (`fara joker`).
//! * Răspunsurile sunt secrete până la pasul `verific` (arbitru J2). Dacă după 4 runde scorul e egal,
//!   meciul e remiză.
//! * Robotul nu vede răspunsul corect: la întrebări alege la întâmplare (nu există o „cunoaștere”
//!   simulată), iar la joker decide aleatoriu.
//! Mutări: `joker`, `fara joker`, `raspund a`…`raspund d`, `verific`.

use crate::cards::{handover_banner, shuffle, Viewer};
use crate::cunoastere_bank::{BANK, CATEGORY_TITLES};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const ROUNDS: usize = 4;
const PER: usize = 5;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Joker(u8),
    Answer(u8),
    Verify,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Joker(bool),
    Answer(u8),
    Verify,
}

#[derive(Clone)]
pub struct PubQuiz {
    qs: Vec<Vec<usize>>,
    order: [usize; 4],
    round: usize,
    q: usize,
    phase: Phase,
    answers: [Option<u8>; 2],
    joker_used: [bool; 2],
    joker_now: [bool; 2],
    pts: [i32; 2],
    round_pts: [i32; 2],
    viewer: Viewer,
    log: String,
}

impl Default for PubQuiz {
    fn default() -> Self {
        Self::new()
    }
}

impl PubQuiz {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut cats: Vec<usize> = (0..6).collect();
        shuffle(&mut cats, &mut rng);
        let mut qs = Vec::new();
        for &c in cats.iter().take(ROUNDS) {
            let mut v: Vec<usize> = BANK
                .iter()
                .enumerate()
                .filter(|(_, q)| q.0 as usize == c)
                .map(|(i, _)| i)
                .collect();
            shuffle(&mut v, &mut rng);
            v.truncate(PER);
            qs.push(v);
        }
        PubQuiz {
            qs,
            order: [cats[0], cats[1], cats[2], cats[3]],
            round: 0,
            q: 0,
            phase: Phase::Joker(0),
            answers: [None; 2],
            joker_used: [false; 2],
            joker_now: [false; 2],
            pts: [0; 2],
            round_pts: [0; 2],
            viewer: Viewer::Fixed(Player::One),
            log: String::new(),
        }
    }

    fn perm(&self) -> [usize; 4] {
        // permutare dependentă de întrebare, deterministă
        let mut r = Rng::new(0xC0FFEE ^ (self.cur() as u64) ^ ((self.order[0] as u64) << 20));
        let mut p = [0usize, 1, 2, 3];
        shuffle(&mut p, &mut r);
        p
    }

    fn cur(&self) -> usize {
        self.qs[self.round][self.q]
    }

    fn over(&self) -> bool {
        self.round >= ROUNDS
    }
}

impl Game for PubQuiz {
    type Move = Move;

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Joker(p) | Phase::Answer(p) => {
                if p == 0 {
                    Player::One
                } else {
                    Player::Two
                }
            }
            Phase::Verify => Player::Two,
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over() {
            return Vec::new();
        }
        match self.phase {
            Phase::Joker(p) => {
                if self.joker_used[p as usize] {
                    vec![Move::Joker(false)]
                } else {
                    vec![Move::Joker(true), Move::Joker(false)]
                }
            }
            Phase::Answer(_) => (0..4).map(Move::Answer).collect(),
            Phase::Verify => vec![Move::Verify],
        }
    }

    fn apply(&mut self, m: &Move) {
        match (*m, self.phase) {
            (Move::Joker(j), Phase::Joker(p)) => {
                self.joker_now[p as usize] = j;
                if j {
                    self.joker_used[p as usize] = true;
                }
                self.phase = if p == 0 {
                    Phase::Joker(1)
                } else {
                    Phase::Answer(0)
                };
            }
            (Move::Answer(k), Phase::Answer(p)) => {
                self.answers[p as usize] = Some(k);
                self.phase = if p == 0 {
                    Phase::Answer(1)
                } else {
                    Phase::Verify
                };
            }
            (Move::Verify, Phase::Verify) => {
                let perm = self.perm();
                let mut msg = String::new();
                for i in 0..2 {
                    let k = self.answers[i].unwrap() as usize;
                    if perm[k] == 0 {
                        self.round_pts[i] += 1;
                        msg.push_str(&format!("J{} corect. ", i + 1));
                    } else {
                        msg.push_str(&format!("J{} greșit. ", i + 1));
                    }
                }
                self.log = format!("«{}» — {msg}", BANK[self.cur()].2[0]);
                self.answers = [None; 2];
                self.q += 1;
                if self.q >= PER {
                    for i in 0..2 {
                        self.pts[i] += self.round_pts[i] * if self.joker_now[i] { 2 } else { 1 };
                    }
                    self.round_pts = [0; 2];
                    self.joker_now = [false; 2];
                    self.round += 1;
                    self.q = 0;
                    self.phase = Phase::Joker(0);
                } else {
                    self.phase = Phase::Answer(0);
                }
            }
            _ => {}
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over() {
            return None;
        }
        Some(match self.pts[0].cmp(&self.pts[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    /// Fără acces la răspunsul corect: robotul răspunde uniform.
    fn evaluate(&self, _p: Player) -> i32 {
        0
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Pub quiz — runda {}/{ROUNDS}. Scor: J1 {} – J2 {} (joker folosit: J1 {}, J2 {}).\n",
            (self.round + 1).min(ROUNDS),
            self.pts[0],
            self.pts[1],
            if self.joker_used[0] { "da" } else { "nu" },
            if self.joker_used[1] { "da" } else { "nu" }
        );
        if self.over() {
            return s;
        }
        s.push_str(&format!(
            "Categoria: {}.\n",
            CATEGORY_TITLES[self.order[self.round]]
        ));
        match self.phase {
            Phase::Joker(p) => s.push_str(&format!(
                "J{}: `joker` (puncte duble în runda asta) sau `fara joker`.\n",
                p + 1
            )),
            _ => {
                let q = &BANK[self.cur()];
                let perm = self.perm();
                s.push_str(&format!("Întrebarea {}/{PER}: {}\n", self.q + 1, q.1));
                for (k, &pk) in perm.iter().enumerate().take(4) {
                    s.push_str(&format!("  {}) {}\n", (b'a' + k as u8) as char, q.2[pk]));
                }
                let me = self.viewer.who(self.to_move());
                if let Phase::Answer(1) = self.phase {
                    if let Viewer::HotSeat = self.viewer {
                        s.push_str(&handover_banner(Player::Two));
                    }
                    if me == Player::One {
                        s.push_str(&format!(
                            "J1 a răspuns {} (ascuns pentru J2).\n",
                            (b'a' + self.answers[0].unwrap()) as char
                        ));
                    } else {
                        s.push_str("J1 a răspuns în ascuns.\n");
                    }
                } else if self.phase == Phase::Verify {
                    s.push_str("Ambele echipe au răspuns. Scrie «verific».\n");
                }
            }
        }
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Joker(true) => "joker".into(),
            Move::Joker(false) => "fara joker".into(),
            Move::Answer(k) => format!("raspund {}", (b'a' + k) as char),
            Move::Verify => "verific".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = match who {
            Some(p) => Viewer::Fixed(p),
            None => Viewer::HotSeat,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_game_scores_and_joker_doubles() {
        let mut g = PubQuiz::with_seed(7);
        g.apply(&Move::Joker(true));
        g.apply(&Move::Joker(false));
        // J1 răspunde corect la toate cele 5, J2 niciodată
        for _ in 0..PER {
            let perm = g.perm();
            let good = perm.iter().position(|&x| x == 0).unwrap() as u8;
            let bad = (good + 1) % 4;
            g.apply(&Move::Answer(good));
            g.apply(&Move::Answer(bad));
            g.apply(&Move::Verify);
        }
        assert_eq!(g.pts, [10, 0]);
        assert_eq!(g.round, 1);
        assert_eq!(g.legal_moves(), vec![Move::Joker(false)]);
    }

    #[test]
    fn ai_ignores_the_other_teams_secret_answer_at_every_level() {
        crate::p3_comun::ai_blind_check(
            "pubquiz",
            PubQuiz::with_seed,
            |g, me, s| {
                let h = 1 - me.idx();
                if g.answers[h].is_some() {
                    g.answers[h] = Some((s % 4) as u8);
                }
            },
            12,
        );
    }

    #[test]
    fn random_games_end_and_second_team_is_blind() {
        for seed in 1..=10u64 {
            let mut g = PubQuiz::with_seed(seed);
            let mut rng = Rng::new(seed);
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
            }
        }
        let mut g = PubQuiz::with_seed(3);
        g.apply(&Move::Joker(false));
        g.apply(&Move::Joker(false));
        g.apply(&Move::Answer(2));
        g.set_viewer(Some(Player::Two));
        assert!(!g.render().contains("a răspuns c"));
        g.set_viewer(Some(Player::One));
        assert!(g.render().contains("a răspuns c"));
    }
}
