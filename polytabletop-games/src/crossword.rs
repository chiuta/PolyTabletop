#![allow(
    clippy::needless_range_loop,
    clippy::manual_is_multiple_of,
    clippy::type_complexity
)]
//! Cuvinte încrucișate (Crossword) — duel de rezolvat o careu generat, pentru 2 jucători.
//!
//! La fiecare partidă se generează (din vocabularul românesc al jocului «Dicționarul Viu») un careu de 11×11 cu 6–9 cuvinte
//! care se încrucișează. Fiecare cuvânt are număr, direcție (`o` orizontal, `v` vertical), lungime, categorie și o definiție.
//! La mutare alegi un cuvânt nerezolvat și propui o soluție: `3o:casa` (numărul, direcția, două puncte, cuvântul;
//! fără diacritice). Propunerea trebuie să aibă lungimea potrivită și să respecte literele deja descoperite la încrucișări.
//! Mutarea forțată `verific` arată dacă ai nimerit: corect = puncte egale cu lungimea cuvântului și mai
//! ai rândul; greșit = −1 punct, cuvântul propus nu mai poate fi încercat la acel loc, iar rândul trece
//! la adversar. Când toate cuvintele sunt rezolvate câștigă scorul mai mare (egal = remiză). AI-ul nu citește soluția: alege după
//! lungime, litere cunoscute și categorie (numărul de candidați posibili).

use crate::dictionar_date::WORDS;
use crate::litere::normalize;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::sync::OnceLock;

const N: usize = 11;

struct PoolWord {
    text: String,
    cat: &'static str,
    clue: &'static str,
}

fn pool() -> &'static Vec<PoolWord> {
    static P: OnceLock<Vec<PoolWord>> = OnceLock::new();
    P.get_or_init(|| {
        let mut v: Vec<PoolWord> = Vec::new();
        for (w, cat, clues) in WORDS {
            if let Some(n) = normalize(w) {
                if (4..=9).contains(&n.len()) {
                    let text: String = n.iter().map(|&b| (b'a' + b) as char).collect();
                    if !v.iter().any(|x| x.text == text) {
                        v.push(PoolWord {
                            text,
                            cat,
                            clue: clues.split(';').next().unwrap_or(""),
                        });
                    }
                }
            }
        }
        v
    })
}

fn cat_name(c: &str) -> &'static str {
    match c {
        "an" => "animale",
        "mn" => "mâncare",
        "ca" => "casă și obiecte",
        "un" => "unelte",
        "me" => "meserii",
        "na" => "natură",
        "cl" => "clădiri și locuri",
        "ab" => "idei și acțiuni",
        "sp" => "sport și joc",
        "ce" => "cosmos",
        "im" => "haine",
        _ => "diverse",
    }
}

#[derive(Clone)]
struct Slot {
    num: u8,
    horiz: bool,
    r: usize,
    c: usize,
    len: usize,
    word: u16,
    cat: &'static str,
    solved: bool,
    wrong: Vec<u16>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Guess(u8, u16),
    Verify,
}

#[derive(Clone)]
pub struct Crossword {
    slots: Vec<Slot>,
    side: Player,
    scores: [i32; 2],
    pending: Option<(u8, u16)>,
    log: String,
}

impl Default for Crossword {
    fn default() -> Self {
        Self::new()
    }
}

impl Crossword {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        loop {
            if let Some(slots) = generate(&mut rng) {
                return Crossword {
                    slots,
                    side: Player::One,
                    scores: [0; 2],
                    pending: None,
                    log: String::new(),
                };
            }
        }
    }

    pub fn scores(&self) -> (i32, i32) {
        (self.scores[0], self.scores[1])
    }

    pub fn slot_count(&self) -> usize {
        self.slots.len()
    }

    fn cells(s: &Slot) -> Vec<(usize, usize)> {
        (0..s.len)
            .map(|i| {
                if s.horiz {
                    (s.r, s.c + i)
                } else {
                    (s.r + i, s.c)
                }
            })
            .collect()
    }

    /// literele cunoscute din cuvintele rezolvate
    fn known(&self) -> [[u8; N]; N] {
        let mut g = [[0u8; N]; N];
        for s in self.slots.iter().filter(|s| s.solved) {
            let w = pool()[s.word as usize].text.as_bytes();
            for (i, (r, c)) in Self::cells(s).into_iter().enumerate() {
                g[r][c] = w[i];
            }
        }
        g
    }

    fn candidates(&self, si: usize, known: &[[u8; N]; N]) -> Vec<u16> {
        let s = &self.slots[si];
        let cells = Self::cells(s);
        pool()
            .iter()
            .enumerate()
            .filter(|(i, w)| {
                w.text.len() == s.len
                    && !s.wrong.contains(&(*i as u16))
                    && cells
                        .iter()
                        .enumerate()
                        .all(|(k, &(r, c))| known[r][c] == 0 || known[r][c] == w.text.as_bytes()[k])
            })
            .map(|(i, _)| i as u16)
            .collect()
    }
}

fn generate(rng: &mut Rng) -> Option<Vec<Slot>> {
    let p = pool();
    let mut grid = [[0u8; N]; N];
    // cuvintele așezate: (pool, r, c, orizontal)
    let mut placed: Vec<(u16, usize, usize, bool)> = Vec::new();
    let first = loop {
        let i = rng.below(p.len());
        if (6..=8).contains(&p[i].text.len()) {
            break i;
        }
    };
    let len = p[first].text.len();
    let c0 = (N - len) / 2;
    for (k, b) in p[first].text.bytes().enumerate() {
        grid[5][c0 + k] = b;
    }
    placed.push((first as u16, 5, c0, true));
    let mut attempts = 0;
    while placed.len() < 9 && attempts < 800 {
        attempts += 1;
        let wi = rng.below(p.len());
        if placed.iter().any(|x| x.0 as usize == wi) {
            continue;
        }
        let w = p[wi].text.as_bytes();
        // alegem o literă comună cu careul
        let base = placed[rng.below(placed.len())];
        let bw = p[base.0 as usize].text.as_bytes();
        let bi = rng.below(bw.len());
        let letter = bw[bi];
        let wi_pos: Vec<usize> = (0..w.len()).filter(|&k| w[k] == letter).collect();
        if wi_pos.is_empty() {
            continue;
        }
        let k = wi_pos[rng.below(wi_pos.len())];
        let horiz = !base.3;
        let (cr, cc) = if base.3 {
            (base.1, base.2 + bi)
        } else {
            (base.1 + bi, base.2)
        };
        let (sr, sc) = if horiz {
            if cc < k {
                continue;
            }
            (cr, cc - k)
        } else {
            if cr < k {
                continue;
            }
            (cr - k, cc)
        };
        if try_place(&mut grid, w, sr, sc, horiz) {
            placed.push((wi as u16, sr, sc, horiz));
        }
    }
    if placed.len() < 6 {
        return None;
    }
    // numerotare după celula de start, în ordine de citire
    let mut starts: Vec<(usize, usize)> = placed.iter().map(|x| (x.1, x.2)).collect();
    starts.sort();
    starts.dedup();
    let mut slots: Vec<Slot> = placed
        .iter()
        .map(|&(wi, r, c, h)| Slot {
            num: starts.iter().position(|&s| s == (r, c)).unwrap() as u8 + 1,
            horiz: h,
            r,
            c,
            len: p[wi as usize].text.len(),
            word: wi,
            cat: p[wi as usize].cat,
            solved: false,
            wrong: Vec::new(),
        })
        .collect();
    slots.sort_by_key(|s| (s.num, !s.horiz));
    Some(slots)
}

fn try_place(grid: &mut [[u8; N]; N], w: &[u8], sr: usize, sc: usize, horiz: bool) -> bool {
    let n = w.len();
    let (er, ec) = if horiz {
        (sr, sc + n - 1)
    } else {
        (sr + n - 1, sc)
    };
    if er >= N || ec >= N {
        return false;
    }
    let at = |g: &[[u8; N]; N], r: isize, c: isize| -> u8 {
        if r < 0 || c < 0 || r >= N as isize || c >= N as isize {
            0
        } else {
            g[r as usize][c as usize]
        }
    };
    let (dr, dc) = if horiz { (0isize, 1isize) } else { (1, 0) };
    // celulele dinaintea și de după cuvânt trebuie să fie goale
    if at(grid, sr as isize - dr, sc as isize - dc) != 0
        || at(grid, er as isize + dr, ec as isize + dc) != 0
    {
        return false;
    }
    let mut crossings = 0;
    for k in 0..n {
        let (r, c) = (sr as isize + dr * k as isize, sc as isize + dc * k as isize);
        let cur = at(grid, r, c);
        if cur != 0 {
            if cur != w[k] {
                return false;
            }
            crossings += 1;
        } else {
            // vecinii laterali trebuie goi (altfel cuvintele s-ar lipi)
            let (pr, pc) = (dc, dr);
            if at(grid, r + pr, c + pc) != 0 || at(grid, r - pr, c - pc) != 0 {
                return false;
            }
        }
    }
    if crossings != 1 {
        return false;
    }
    for k in 0..n {
        let (r, c) = (
            sr + if horiz { 0 } else { k },
            sc + if horiz { k } else { 0 },
        );
        grid[r][c] = w[k];
    }
    true
}

impl Game for Crossword {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        if self.pending.is_some() {
            return vec![Move::Verify];
        }
        let known = self.known();
        let mut v = Vec::new();
        for (si, s) in self.slots.iter().enumerate() {
            if s.solved {
                continue;
            }
            for w in self.candidates(si, &known) {
                v.push(Move::Guess(si as u8, w));
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        match *m {
            Move::Guess(si, w) => self.pending = Some((si, w)),
            Move::Verify => {
                let (si, w) = self.pending.take().unwrap();
                let me = self.side.idx();
                let s = &mut self.slots[si as usize];
                if s.word == w {
                    s.solved = true;
                    self.scores[me] += s.len as i32;
                    self.log = format!(
                        "J{} a nimerit «{}» (+{})",
                        me + 1,
                        pool()[w as usize].text,
                        s.len
                    );
                } else {
                    s.wrong.push(w);
                    self.scores[me] -= 1;
                    self.log = format!(
                        "J{}: «{}» nu este soluția (-1)",
                        me + 1,
                        pool()[w as usize].text
                    );
                    self.side = self.side.other();
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.slots.iter().all(|s| s.solved) {
            return None;
        }
        Some(match self.scores[0].cmp(&self.scores[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let mut v = (self.scores[p.idx()] - self.scores[p.other().idx()]) * 1000;
        if let Some((si, _)) = self.pending {
            // probabilitatea de a fi corect = 1 / numărul candidaților din aceeași categorie (informație publică)
            let known = self.known();
            let s = &self.slots[si as usize];
            let cand = self.candidates(si as usize, &known);
            let mut k = cand
                .iter()
                .filter(|&&w| pool()[w as usize].cat == s.cat)
                .count() as i32;
            if k == 0 {
                k = cand.len().max(1) as i32;
            }
            let ev = (s.len as i32 * 1000 - (k - 1) * 1000) / k;
            v += if self.side == p { ev } else { -ev };
        }
        v
    }

    fn render(&self) -> String {
        let known = self.known();
        let mut used = [[false; N]; N];
        for s in &self.slots {
            for (r, c) in Self::cells(s) {
                used[r][c] = true;
            }
        }
        let mut starts = std::collections::HashMap::new();
        for s in &self.slots {
            starts.entry((s.r, s.c)).or_insert(s.num);
        }
        let mut out = format!(
            "Cuvinte încrucișate — J1 {} : {} J2\n",
            self.scores[0], self.scores[1]
        );
        if !self.log.is_empty() {
            out.push_str(&format!("({})\n", self.log));
        }
        for r in 0..N {
            if !(0..N).any(|c| used[r][c]) {
                continue;
            }
            let mut line = String::new();
            for c in 0..N {
                let ch = if !used[r][c] {
                    "  ".to_string()
                } else if known[r][c] != 0 {
                    format!("{} ", (known[r][c] as char).to_ascii_uppercase())
                } else if let Some(n) = starts.get(&(r, c)) {
                    format!(
                        "{} ",
                        if *n < 10 {
                            n.to_string()
                        } else {
                            "+".to_string()
                        }
                    )
                } else {
                    "_ ".to_string()
                };
                line.push_str(&ch);
            }
            out.push_str(&format!("{}\n", line.trim_end()));
        }
        for s in &self.slots {
            if s.solved {
                continue;
            }
            let w = &pool()[s.word as usize];
            out.push_str(&format!(
                "{}{} ({} litere, {}): {}\n",
                s.num,
                if s.horiz { "o" } else { "v" },
                s.len,
                cat_name(s.cat),
                w.clue
            ));
        }
        if self.outcome().is_none() {
            if self.pending.is_some() {
                out.push_str("Scrie «verific».\n");
            }
            out.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        }
        out
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Guess(si, w) => {
                let s = &self.slots[*si as usize];
                format!(
                    "{}{}:{}",
                    s.num,
                    if s.horiz { "o" } else { "v" },
                    pool()[*w as usize].text
                )
            }
            Move::Verify => "verific".into(),
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
    fn generated_grid_is_consistent() {
        for seed in 1..=10u64 {
            let g = Crossword::with_seed(seed);
            assert!(g.slot_count() >= 6, "seed {seed}");
            // literele din cuvintele care se încrucișează coincid
            let mut grid = [[0u8; N]; N];
            for s in &g.slots {
                let w = pool()[s.word as usize].text.as_bytes();
                for (i, (r, c)) in Crossword::cells(s).into_iter().enumerate() {
                    assert!(grid[r][c] == 0 || grid[r][c] == w[i]);
                    grid[r][c] = w[i];
                }
            }
        }
    }

    #[test]
    fn solving_flow() {
        let mut g = Crossword::with_seed(3);
        let (si, w) = (0u8, g.slots[0].word);
        g.apply(&Move::Guess(si, w));
        assert_eq!(g.legal_moves(), vec![Move::Verify]);
        g.apply(&Move::Verify);
        assert!(g.slots[0].solved);
        assert_eq!(g.to_move(), Player::One);
        assert_eq!(g.scores[0], g.slots[0].len as i32);
        // o propunere greșită costă rândul
        let bad = g
            .legal_moves()
            .into_iter()
            .find(|m| matches!(m, Move::Guess(s, w) if g.slots[*s as usize].word != *w))
            .unwrap();
        g.apply(&bad);
        g.apply(&Move::Verify);
        assert_eq!(g.to_move(), Player::Two);
    }

    #[test]
    fn evaluation_ignores_answer() {
        let mut a = Crossword::with_seed(5);
        let m = a.legal_moves()[0];
        a.apply(&m);
        let mut b = a.clone();
        // schimbăm răspunsul real al altor cuvinte nu are cum să schimbe evaluarea (folosește doar numărul de candidați)
        for s in b.slots.iter_mut().skip(1) {
            s.word = (s.word + 1) % pool().len() as u16;
        }
        assert_eq!(a.evaluate(Player::One), b.evaluate(Player::One));
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=6u64 {
            let mut g = Crossword::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut k = 0;
            while g.outcome().is_none() && k < 4000 {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
                k += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed} k {k}");
        }
    }
}
