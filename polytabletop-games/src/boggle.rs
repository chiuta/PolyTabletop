//! Boggle (SUA), 2 jucători, adaptare pe ture cu dicționarul românesc mic al jocului „Litere”.
//!
//! * Grilă 4×4 de litere (generată din seed, cu cel puțin 20 de cuvinte valide). Un cuvânt se formează din
//!   litere vecine (inclusiv pe diagonală), fără a refolosi aceeași căsuță în același cuvânt, de cel puțin
//!   3 litere, fără diacritice (ca în `Litere`).
//! * Pe rând, fiecare jucător revendică **un cuvânt** găsit în grilă (fiecare cuvânt se poate revendica o
//!   singură dată). Puncte: lungimea − 2 (3 litere = 1, 4 = 2, 5 = 3 …). Câte 10 revendicări maxim pentru
//!   fiecare; `pas` îți cedează rândul, iar două pasuri la rând încheie jocul.
//! * Cuvintele valide pot fi listate cu `?` (lista mutărilor legale) — pentru un joc fără ajutor nu folosi `?`.
//!
//! Mutări: `casa` (cuvântul scris fără diacritice), `pas`.

use crate::litere::{normalize, Dictionary};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::collections::HashSet;
use std::sync::OnceLock;

const N: usize = 4;
const MAX_CLAIMS: usize = 10;
const FREQ: &str =
    "aaaaaaaeeeeeeiiiiiiioooouuuuaăîâcccdddllllmmmnnnnnnrrrrrrsssstttttppbbffgghhvvzzjxk";

struct Lex {
    dict: Dictionary,
    prefixes: HashSet<Vec<u8>>,
}

fn lex() -> &'static Lex {
    static L: OnceLock<Lex> = OnceLock::new();
    L.get_or_init(|| {
        let text = include_str!("../data/litere_dict.txt");
        let dict = Dictionary::from_text(text);
        let mut prefixes = HashSet::new();
        for line in text.lines() {
            if line.starts_with('#') {
                continue;
            }
            for w in line.split_whitespace() {
                if let Some(v) = normalize(w) {
                    for k in 1..=v.len() {
                        prefixes.insert(v[..k].to_vec());
                    }
                }
            }
        }
        Lex { dict, prefixes }
    })
}

#[derive(Clone, PartialEq, Debug)]
pub enum Move {
    Word(Vec<u8>),
    Pass,
}

fn text(w: &[u8]) -> String {
    w.iter().map(|&c| (b'a' + c) as char).collect()
}

#[derive(Clone)]
pub struct Boggle {
    grid: [u8; N * N],
    found: Vec<Vec<u8>>,
    taken: Vec<(Vec<u8>, usize)>,
    side: Player,
    passes: u8,
    score: [i32; 2],
}

impl Default for Boggle {
    fn default() -> Self {
        Self::new()
    }
}

fn all_words(grid: &[u8; N * N]) -> Vec<Vec<u8>> {
    let l = lex();
    let mut out: HashSet<Vec<u8>> = HashSet::new();
    fn dfs(
        grid: &[u8; N * N],
        l: &Lex,
        pos: usize,
        used: &mut [bool; N * N],
        cur: &mut Vec<u8>,
        out: &mut HashSet<Vec<u8>>,
    ) {
        cur.push(grid[pos]);
        used[pos] = true;
        if l.prefixes.contains(cur.as_slice()) {
            if cur.len() >= 3 && l.dict.contains(cur) {
                out.insert(cur.clone());
            }
            let (r, c) = ((pos / N) as i32, (pos % N) as i32);
            for dr in -1..=1 {
                for dc in -1..=1 {
                    let (rr, cc) = (r + dr, c + dc);
                    if (dr, dc) != (0, 0) && rr >= 0 && rr < N as i32 && cc >= 0 && cc < N as i32 {
                        let q = rr as usize * N + cc as usize;
                        if !used[q] {
                            dfs(grid, l, q, used, cur, out);
                        }
                    }
                }
            }
        }
        used[pos] = false;
        cur.pop();
    }
    for p in 0..N * N {
        let mut used = [false; N * N];
        let mut cur = Vec::new();
        dfs(grid, l, p, &mut used, &mut cur, &mut out);
    }
    let mut v: Vec<Vec<u8>> = out.into_iter().collect();
    v.sort();
    v
}

impl Boggle {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let bag: Vec<u8> = FREQ
            .chars()
            .map(|c| match c {
                'ă' | 'â' => 0,
                'î' => 8,
                x => x as u8 - b'a',
            })
            .collect();
        loop {
            let mut grid = [0u8; N * N];
            for g in grid.iter_mut() {
                *g = bag[rng.below(bag.len())];
            }
            let found = all_words(&grid);
            if found.len() >= 20 {
                return Boggle {
                    grid,
                    found,
                    taken: Vec::new(),
                    side: Player::One,
                    passes: 0,
                    score: [0, 0],
                };
            }
        }
    }

    pub fn words_left(&self) -> usize {
        self.found.len() - self.taken.len()
    }

    fn claims(&self, p: usize) -> usize {
        self.taken.iter().filter(|t| t.1 == p).count()
    }

    fn over(&self) -> bool {
        self.passes >= 2
            || self.words_left() == 0
            || (self.claims(0) >= MAX_CLAIMS && self.claims(1) >= MAX_CLAIMS)
    }
}

impl Game for Boggle {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over() {
            return Vec::new();
        }
        let mut v = vec![Move::Pass];
        if self.claims(self.side.idx()) < MAX_CLAIMS {
            for w in &self.found {
                if !self.taken.iter().any(|t| &t.0 == w) {
                    v.push(Move::Word(w.clone()));
                }
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Pass => self.passes += 1,
            Move::Word(w) => {
                self.passes = 0;
                self.score[self.side.idx()] += w.len() as i32 - 2;
                self.taken.push((w.clone(), self.side.idx()));
            }
        }
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over() {
            return None;
        }
        Some(match self.score[0].cmp(&self.score[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        (self.score[p.idx()] - self.score[p.other().idx()]) * 10
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Boggle — scor J1 {} – J2 {}; cuvinte rămase de găsit: {}\n",
            self.score[0],
            self.score[1],
            self.words_left()
        );
        for r in 0..N {
            s.push_str("  ");
            for c in 0..N {
                s.push_str(&format!(
                    " {}",
                    ((b'a' + self.grid[r * N + c]) as char).to_ascii_uppercase()
                ));
            }
            s.push('\n');
        }
        for p in 0..2 {
            let w: Vec<String> = self
                .taken
                .iter()
                .filter(|t| t.1 == p)
                .map(|t| text(&t.0))
                .collect();
            s.push_str(&format!("J{}: {}\n", p + 1, w.join(", ")));
        }
        match self.outcome() {
            Some(Outcome::Win(p)) => s.push_str(&format!("Jucătorul {} câștigă.\n", p.idx() + 1)),
            Some(Outcome::Draw) => s.push_str("Remiză.\n"),
            None => s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1)),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Pass => "pas".into(),
            Move::Word(w) => text(w),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }

    fn move_score(&self, m: &Move) -> i32 {
        match m {
            Move::Word(w) => w.len() as i32,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_has_words_and_they_are_on_the_grid() {
        let g = Boggle::with_seed(9);
        assert!(g.found.len() >= 20);
        let again = all_words(&g.grid);
        assert_eq!(again, g.found);
    }

    #[test]
    fn random_games_end_and_scores_match() {
        for seed in 1..=5u64 {
            let mut g = Boggle::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 500);
            }
            let sum: i32 = g.taken.iter().map(|t| t.0.len() as i32 - 2).sum();
            assert_eq!(sum, g.score[0] + g.score[1]);
        }
    }
}
