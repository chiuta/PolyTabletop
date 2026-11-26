#![allow(clippy::needless_range_loop, clippy::manual_is_multiple_of)]
//! Perquackey — joc de zaruri cu litere (varianta pe ture pentru 2 jucători).
//!
//! Originalul se joacă în timp real (3 minute): 15 zaruri cu litere, fiecare jucător notează cuvinte
//! formate din literele aruncate. Aici fiecare rundă are zarurile aruncate într-o mutare forțată
//! (`arunc`); apoi jucătorii alternă: `cuv <cuvânt>` (cuvânt românesc din dicționarul jocului, de cel puțin 3 litere,
//! format din zaruri, fiecare zar o singură dată; un cuvânt poate fi luat o singură dată pe rundă) sau `pas`.
//! Runda se termină după două `pas` consecutive. Punctaj: lungime − 2 (3 litere = 1 … 7 litere = 5, 8+ = 6+).
//! Sunt 3 runde; câștigă scorul mai mare. Diacriticele se ignoră. Informație ascunsă: nu.

use crate::dictionar_date::WORDS;
use crate::litere::normalize;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::sync::OnceLock;

const ROUNDS: u8 = 3;
const DICE: usize = 15;
/// frecvențe aproximative în română (a..z)
const FREQ: [u32; 26] = [
    10, 1, 5, 4, 11, 1, 1, 1, 10, 0, 0, 5, 3, 7, 6, 4, 0, 8, 7, 7, 4, 2, 0, 0, 0, 0,
];

fn dict() -> &'static Vec<String> {
    static D: OnceLock<Vec<String>> = OnceLock::new();
    D.get_or_init(|| {
        let mut v: Vec<String> = Vec::new();
        let text = include_str!("../data/litere_dict.txt");
        for line in text.lines() {
            if line.trim_start().starts_with('#') {
                continue;
            }
            for tok in line.split_whitespace() {
                if let Some(w) = normalize(tok) {
                    if w.len() >= 3 && w.len() <= DICE {
                        v.push(w.iter().map(|&b| (b'a' + b) as char).collect());
                    }
                }
            }
        }
        for (w, _, _) in WORDS {
            if let Some(n) = normalize(w) {
                if n.len() >= 3 && n.len() <= DICE {
                    v.push(n.iter().map(|&b| (b'a' + b) as char).collect());
                }
            }
        }
        v.sort();
        v.dedup();
        v
    })
}

fn can_form(word: &str, dice: &[u8]) -> bool {
    let mut cnt = [0i32; 26];
    for &d in dice {
        cnt[d as usize] += 1;
    }
    for b in word.bytes() {
        let i = (b - b'a') as usize;
        cnt[i] -= 1;
        if cnt[i] < 0 {
            return false;
        }
    }
    true
}

fn points(len: usize) -> i32 {
    len as i32 - 2
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Roll,
    Word(String),
    Pass,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Roll,
    Play,
    Over,
}

#[derive(Clone)]
pub struct Perquackey {
    rng: Rng,
    dice: Vec<u8>,
    used: Vec<String>,
    scores: [i32; 2],
    round: u8,
    passes: u8,
    side: Player,
    starter: Player,
    phase: Phase,
    log: String,
}

impl Default for Perquackey {
    fn default() -> Self {
        Self::new()
    }
}

impl Perquackey {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Perquackey {
            rng: Rng::new(seed),
            dice: Vec::new(),
            used: Vec::new(),
            scores: [0; 2],
            round: 1,
            passes: 0,
            side: Player::One,
            starter: Player::One,
            phase: Phase::Roll,
            log: String::new(),
        }
    }

    pub fn scores(&self) -> (i32, i32) {
        (self.scores[0], self.scores[1])
    }

    fn roll(&mut self) {
        let total: u32 = FREQ.iter().sum();
        loop {
            let mut d = Vec::new();
            for _ in 0..DICE {
                let mut r = self.rng.below(total as usize) as u32;
                for (i, &f) in FREQ.iter().enumerate() {
                    if r < f {
                        d.push(i as u8);
                        break;
                    }
                    r -= f;
                }
            }
            let vowels = d
                .iter()
                .filter(|&&x| matches!(x, 0 | 4 | 8 | 14 | 20))
                .count();
            if (5..=8).contains(&vowels) {
                d.sort_unstable();
                self.dice = d;
                return;
            }
        }
    }

    fn words(&self) -> Vec<String> {
        dict()
            .iter()
            .filter(|w| can_form(w, &self.dice) && !self.used.contains(w))
            .cloned()
            .collect()
    }

    fn dice_text(&self) -> String {
        self.dice
            .iter()
            .map(|&b| ((b'a' + b) as char).to_ascii_uppercase().to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl Game for Perquackey {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        match self.phase {
            Phase::Over => Vec::new(),
            Phase::Roll => vec![Move::Roll],
            Phase::Play => {
                let mut v: Vec<Move> = self.words().into_iter().map(Move::Word).collect();
                v.push(Move::Pass);
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        match m {
            Move::Roll => {
                self.roll();
                self.used.clear();
                self.passes = 0;
                self.side = self.starter;
                self.phase = Phase::Play;
            }
            Move::Word(w) => {
                self.scores[self.side.idx()] += points(w.len());
                self.log = format!(
                    "J{} a găsit «{}» (+{})",
                    self.side.idx() + 1,
                    w,
                    points(w.len())
                );
                self.used.push(w.clone());
                self.passes = 0;
                self.side = self.side.other();
            }
            Move::Pass => {
                self.passes += 1;
                self.side = self.side.other();
                if self.passes >= 2 {
                    if self.round >= ROUNDS {
                        self.phase = Phase::Over;
                    } else {
                        self.round += 1;
                        self.starter = self.starter.other();
                        self.side = self.starter;
                        self.phase = Phase::Roll;
                    }
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
        (self.scores[p.idx()] - self.scores[p.other().idx()]) * 100
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Perquackey — runda {}/{} | scor: J1 {}, J2 {}\n",
            self.round, ROUNDS, self.scores[0], self.scores[1]
        );
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        match self.phase {
            Phase::Over => s.push_str("Meci încheiat.\n"),
            Phase::Roll => s.push_str("Scrie «arunc» pentru a arunca zarurile.\n"),
            Phase::Play => {
                s.push_str(&format!("Zaruri: {}\n", self.dice_text()));
                if !self.used.is_empty() {
                    s.push_str(&format!("Cuvinte luate: {}\n", self.used.join(", ")));
                }
                s.push_str(&format!(
                    "La mutare: Jucătorul {} (cuv <cuvânt> sau pas)\n",
                    self.side.idx() + 1
                ));
            }
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Roll => "arunc".into(),
            Move::Word(w) => format!("cuv {w}"),
            Move::Pass => "pas".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dictionary_nonempty_and_forms() {
        assert!(dict().len() > 200);
        assert!(can_form("casa", &[2, 0, 18, 0, 5]));
        assert!(!can_form("casa", &[2, 0, 18, 5]));
    }

    #[test]
    fn round_flow() {
        let mut g = Perquackey::with_seed(2);
        g.apply(&Move::Roll);
        assert_eq!(g.dice.len(), DICE);
        let w = g.legal_moves();
        assert!(w.len() > 1 || w == vec![Move::Pass]);
        if let Some(Move::Word(x)) = w.first().cloned() {
            let len = x.len();
            g.apply(&Move::Word(x.clone()));
            assert_eq!(g.scores[0], points(len));
            assert!(!g.legal_moves().contains(&Move::Word(x)));
        }
        g.apply(&Move::Pass);
        g.apply(&Move::Pass);
        assert_eq!(g.round, 2);
        assert_eq!(g.phase, Phase::Roll);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=6u64 {
            let mut g = Perquackey::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut k = 0;
            while g.outcome().is_none() && k < 4000 {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
                k += 1;
            }
            assert!(g.outcome().is_some());
        }
    }
}
