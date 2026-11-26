//! Zaruri colorate (Kismet, SUA) — joc de zaruri cu categorii, înrudit cu Yahtzee, 2 jucători.
//!
//! 15 ture pe jucător; în fiecare tură, până la 3 aruncări cu 5 zaruri (`arunc`), după care alegi
//! o categorie liberă (`scor kismet`). După primele două aruncări poți ține zaruri (`tine 3 3 5`).
//!
//! **Culori**: fiecare zar are fețe roșii și albastre; aici fețele impare sunt roșii, cele pare albastre
//! (convenție proprie, fiindcă aranjamentul real al culorilor nu e esențial pentru reguli).
//!
//! Categorii și scor (după Wikipedia „Kismet (dice game)”):
//! * secțiunea de bază `unu`…`sase`: suma zarurilor cu acea valoare; bonus 35 la total ≥ 63, 55 la ≥ 71, 75 la ≥ 78;
//! * `2perechi` = două perechi de aceeași culoare (acoperă și patru la fel, casa de aceeași culoare, kismet): suma;
//! * `3la-fel`: suma; `scara` (5 consecutive): 30; `culoare` (toate zarurile de aceeași culoare): 35;
//! * `casa` (3+2): suma + 15; `casa-culoare` (3+2 de aceeași culoare): suma + 20; `4la-fel`: suma + 25;
//! * `sansa` (Yarborough): suma; `kismet` (cinci la fel): suma + 50.
//!
//! Simplificări: fără regula „joker” după un kismet și fără penalizarea adversarilor la al doilea kismet.

#![allow(clippy::needless_range_loop)]

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const CATS: usize = 15;
pub const NAMES: [&str; CATS] = [
    "unu",
    "doi",
    "trei",
    "patru",
    "cinci",
    "sase",
    "2perechi",
    "3la-fel",
    "scara",
    "culoare",
    "casa",
    "casa-culoare",
    "4la-fel",
    "sansa",
    "kismet",
];
const PAR: [f32; CATS] = [
    2.1, 5.3, 8.6, 12.2, 15.7, 19.2, 18.0, 17.0, 15.0, 11.0, 22.0, 15.0, 28.0, 22.0, 20.0,
];

fn color(v: u8) -> u8 {
    v % 2
}

/// Scorul unei categorii pentru 5 zaruri (0 dacă nu se potrivește).
pub fn score(cat: usize, dice: &[u8]) -> i32 {
    let mut cnt = [0u8; 7];
    let mut sum = 0i32;
    for &d in dice {
        cnt[d as usize] += 1;
        sum += d as i32;
    }
    let max = *cnt[1..].iter().max().unwrap();
    let pairs: Vec<u8> = (1..=6u8).filter(|&v| cnt[v as usize] >= 2).collect();
    let flush = dice.iter().all(|&d| color(d) == color(dice[0]));
    let full = cnt[1..].contains(&3) && cnt[1..].contains(&2);
    let full_color = full && {
        let t = (1..=6u8).find(|&v| cnt[v as usize] == 3).unwrap();
        let w = (1..=6u8).find(|&v| cnt[v as usize] == 2).unwrap();
        color(t) == color(w)
    };
    match cat {
        0..=5 => cnt[cat + 1] as i32 * (cat as i32 + 1),
        6 => {
            let ok = (1..=6u8).any(|v| cnt[v as usize] >= 4)
                || pairs
                    .iter()
                    .any(|&a| pairs.iter().any(|&b| a < b && color(a) == color(b)));
            if ok {
                sum
            } else {
                0
            }
        }
        7 => {
            if max >= 3 {
                sum
            } else {
                0
            }
        }
        8 => {
            if (1..=2).any(|a| (a..a + 5).all(|v| cnt[v as usize] > 0)) {
                30
            } else {
                0
            }
        }
        9 => {
            if flush {
                35
            } else {
                0
            }
        }
        10 => {
            if full {
                sum + 15
            } else {
                0
            }
        }
        11 => {
            if full_color {
                sum + 20
            } else {
                0
            }
        }
        12 => {
            if max >= 4 {
                sum + 25
            } else {
                0
            }
        }
        13 => sum,
        _ => {
            if max == 5 {
                sum + 50
            } else {
                0
            }
        }
    }
}

fn bonus(basic_total: i32) -> i32 {
    if basic_total >= 78 {
        75
    } else if basic_total >= 71 {
        55
    } else if basic_total >= 63 {
        35
    } else {
        0
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Mv {
    Roll,
    Hold(Vec<u8>),
    Score(u8),
}

#[derive(Clone)]
pub struct Kismet {
    card: [[Option<i32>; CATS]; 2],
    side: usize,
    dice: Vec<u8>,
    held: Vec<u8>,
    rolls: u8,
    rng: Rng,
}

impl Default for Kismet {
    fn default() -> Self {
        Self::new()
    }
}

impl Kismet {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        Kismet {
            card: [[None; CATS]; 2],
            side: 0,
            dice: Vec::new(),
            held: Vec::new(),
            rolls: 0,
            rng: Rng::new(seed),
        }
    }
    pub fn total(&self, p: usize) -> i32 {
        let t: i32 = self.card[p].iter().flatten().sum();
        let basic: i32 = self.card[p][..6].iter().flatten().sum();
        t + bonus(basic)
    }
    fn surplus(&self, p: usize) -> f32 {
        let mut s = 0.0;
        for c in 0..CATS {
            if let Some(v) = self.card[p][c] {
                s += v as f32 - PAR[c];
            }
        }
        let basic: i32 = self.card[p][..6].iter().flatten().sum();
        s + bonus(basic) as f32
    }
    fn full(&self) -> Vec<u8> {
        let mut v = self.held.clone();
        v.extend(&self.dice);
        v
    }
    fn needs_roll(&self) -> bool {
        self.dice.is_empty()
    }
    /// Valoarea așteptată (peste „par”) a celei mai bune categorii libere după rearuncarea a `k` zaruri.
    fn exp_best(&self, held: &[u8], k: usize) -> f32 {
        let open: Vec<usize> = (0..CATS)
            .filter(|&c| self.card[self.side][c].is_none())
            .collect();
        let mut h: u64 = 0xcbf29ce484222325;
        for &d in held {
            h = (h ^ d as u64).wrapping_mul(0x100000001b3);
        }
        h ^= (k as u64) << 40 ^ self.card[self.side].iter().filter(|c| c.is_some()).count() as u64;
        let mut rng = Rng::new(h);
        let n = if k == 0 { 1 } else { 90 };
        let mut tot = 0.0;
        for _ in 0..n {
            let mut d = held.to_vec();
            for _ in 0..k {
                d.push(rng.d6());
            }
            let best = open
                .iter()
                .map(|&c| score(c, &d) as f32 - PAR[c])
                .fold(f32::MIN, f32::max);
            tot += best;
        }
        tot / n as f32
    }
}

impl Game for Kismet {
    type Move = Mv;
    fn to_move(&self) -> Player {
        if self.side == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn legal_moves(&self) -> Vec<Mv> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        if self.needs_roll() {
            return vec![Mv::Roll];
        }
        let mut v = Vec::new();
        if self.rolls < 3 {
            // submulțimi distincte (după valoare) ale celor 5 zaruri, fără „toate”
            let all = {
                let mut a = self.full();
                a.sort_unstable();
                a
            };
            let mut seen: Vec<Vec<u8>> = Vec::new();
            for mask in 0u32..31 {
                let mut sub: Vec<u8> = (0..5)
                    .filter(|i| mask & (1 << i) != 0)
                    .map(|i| all[i])
                    .collect();
                sub.sort_unstable();
                if !seen.contains(&sub) {
                    seen.push(sub.clone());
                    v.push(Mv::Hold(sub));
                }
            }
        }
        for c in 0..CATS {
            if self.card[self.side][c].is_none() {
                v.push(Mv::Score(c as u8));
            }
        }
        v
    }
    fn apply(&mut self, m: &Mv) {
        match m {
            Mv::Roll => {
                let k = 5 - self.held.len();
                self.dice = (0..k).map(|_| self.rng.d6()).collect();
                self.rolls += 1;
            }
            Mv::Hold(h) => {
                self.held = h.clone();
                self.dice.clear();
            }
            Mv::Score(c) => {
                let d = self.full();
                self.card[self.side][*c as usize] = Some(score(*c as usize, &d));
                self.side = 1 - self.side;
                self.dice.clear();
                self.held.clear();
                self.rolls = 0;
            }
        }
    }
    fn outcome(&self) -> Option<Outcome> {
        if self.card.iter().all(|c| c.iter().all(|x| x.is_some())) {
            let (a, b) = (self.total(0), self.total(1));
            return Some(match a.cmp(&b) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                _ => Outcome::Draw,
            });
        }
        None
    }
    fn evaluate(&self, pl: Player) -> i32 {
        let me = pl.idx();
        let mut v = self.surplus(me) - self.surplus(1 - me);
        // mutarea „tine”: zarurile rămase se vor rearuncă (nu sunt încă aruncate)
        if self.needs_roll() && self.rolls > 0 {
            let e = self.exp_best(&self.held, 5 - self.held.len());
            v += if self.side == me { e } else { -e };
        }
        (v * 10.0) as i32
    }
    fn render(&self) -> String {
        let mut s = String::from("Categorie       J1    J2\n");
        for c in 0..CATS {
            let f = |p: usize| self.card[p][c].map_or("  .".to_string(), |v| format!("{v:>3}"));
            s.push_str(&format!("{:<14} {}   {}\n", NAMES[c], f(0), f(1)));
        }
        s.push_str(&format!(
            "Total (cu bonus): J1 {} – {} J2\n",
            self.total(0),
            self.total(1)
        ));
        if !self.held.is_empty() || !self.dice.is_empty() {
            let col = |v: u8| {
                if color(v) == 1 {
                    format!("{v}r")
                } else {
                    format!("{v}a")
                }
            };
            s.push_str(&format!(
                "Ținute: {:?}; aruncate: {}. Aruncarea {} din 3 (r = roșu, a = albastru).\n",
                self.held,
                self.dice
                    .iter()
                    .map(|&d| col(d))
                    .collect::<Vec<_>>()
                    .join(" "),
                self.rolls
            ));
        }
        s.push_str(&format!("La mutare: Jucătorul {}.", self.side + 1));
        s
    }
    fn format_move(&self, m: &Mv) -> String {
        match m {
            Mv::Roll => "arunc".into(),
            Mv::Hold(h) if h.is_empty() => "tine -".into(),
            Mv::Hold(h) => format!(
                "tine {}",
                h.iter()
                    .map(|d| d.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            Mv::Score(c) => format!("scor {}", NAMES[*c as usize]),
        }
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q4_util::*;
    use polytabletop_core::Player;

    #[test]
    fn scoring_table() {
        assert_eq!(score(14, &[3, 3, 3, 3, 3]), 15 + 50);
        assert_eq!(score(8, &[2, 3, 4, 5, 6]), 30);
        assert_eq!(score(9, &[1, 3, 5, 5, 1]), 35);
        assert_eq!(score(10, &[2, 2, 5, 5, 5]), 19 + 15);
        assert_eq!(score(11, &[2, 2, 4, 4, 4]), 16 + 20);
        assert_eq!(score(11, &[2, 2, 5, 5, 5]), 0);
        assert_eq!(score(6, &[1, 1, 3, 3, 6]), 14); // 1 și 3 sunt roșii
        assert_eq!(score(6, &[1, 1, 2, 2, 6]), 0);
        assert_eq!(score(12, &[4, 4, 4, 4, 1]), 17 + 25);
        assert_eq!(bonus(63), 35);
        assert_eq!(bonus(71), 55);
        assert_eq!(bonus(78), 75);
    }

    #[test]
    fn random_and_ai_games_finish() {
        random_finishes(|s| Box::new(Kismet::with_seed(s)), 5, 1500);
        let r = ai_vs_random(|s| Box::new(Kismet::with_seed(s)), 2, Player::One, 1500);
        assert!(r.is_some());
        ai_vs_random(|s| Box::new(Kismet::with_seed(s)), 3, Player::Two, 1500);
    }
}
