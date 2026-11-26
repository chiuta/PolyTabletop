//! Yacht (Asia / Marea Britanie), 2 jucători, din regulile publice ale jocului de zaruri.
//!
//! * 12 runde pe jucător; în fiecare rundă: până la 3 aruncări cu 5 zaruri; după fiecare aruncare (primele
//!   două) poți **ține** unele zaruri (`tine 3 3 5`, `tine -` = niciunul) și rearunci restul, sau **scrii**
//!   scorul într-o categorie liberă (`scor casa`). După a treia aruncare trebuie să scrii (o categorie
//!   neîndeplinită primește 0).
//! * Aruncarea este o mutare separată, forțată (`arunc`), ca AI-ul să nu vadă zarurile viitoare.
//! * Categorii (fiecare o singură dată): unu…sase (suma zarurilor cu acea valoare), alegere (suma tuturor),
//!   patru-la-fel (suma celor patru zaruri egale), casa (trei + două, suma tuturor), scara-mica (1-2-3-4-5: 30),
//!   scara-mare (2-3-4-5-6: 30), yacht (cinci egale: 50). Nu există bonus pentru partea de sus.
//!
//! AI: `evaluate` folosește doar scorurile, zarurile tale actuale și zarurile ținute (speranța exactă a
//! celei mai bune categorii după o aruncare a zarurilor rămase).

#![allow(clippy::needless_range_loop)]

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const CATS: usize = 12;
pub const CAT_NAMES: [&str; CATS] = [
    "unu",
    "doi",
    "trei",
    "patru",
    "cinci",
    "sase",
    "alegere",
    "patru-la-fel",
    "casa",
    "scara-mica",
    "scara-mare",
    "yacht",
];
/// Valori medii „de așteptat” per categorie (pentru euristica AI).
const PAR: [f32; CATS] = [
    2.1, 5.3, 8.6, 12.2, 15.7, 19.2, 22.0, 8.0, 10.0, 11.0, 12.0, 10.0,
];

/// Scorul unei categorii pentru 5 zaruri.
pub fn score(cat: usize, dice: &[u8]) -> u8 {
    let mut cnt = [0u8; 7];
    let mut sum = 0u8;
    for &d in dice {
        cnt[d as usize] += 1;
        sum += d;
    }
    let max = *cnt[1..].iter().max().unwrap();
    match cat {
        0..=5 => cnt[cat + 1] * (cat as u8 + 1),
        6 => sum,
        7 => {
            if max >= 4 {
                (1..=6).find(|&v| cnt[v] >= 4).unwrap() as u8 * 4
            } else {
                0
            }
        }
        8 => {
            if cnt[1..].contains(&3) && cnt[1..].contains(&2) {
                sum
            } else {
                0
            }
        }
        9 => {
            if (1..=5).all(|v| cnt[v] == 1) {
                30
            } else {
                0
            }
        }
        10 => {
            if (2..=6).all(|v| cnt[v] == 1) {
                30
            } else {
                0
            }
        }
        _ => {
            if max == 5 {
                50
            } else {
                0
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Roll,
    Keep(Vec<u8>),
    Score(usize),
}

#[derive(Clone)]
pub struct Yacht {
    sheet: [[Option<u8>; CATS]; 2],
    side: Player,
    /// Zaruri ținute (înaintea aruncării următoare), sortate.
    kept: Vec<u8>,
    /// Zarurile curente (după aruncare), sortate. Gol când urmează o aruncare.
    dice: Vec<u8>,
    rolls_used: u8,
    rng: Rng,
    log: String,
}

impl Default for Yacht {
    fn default() -> Self {
        Self::new()
    }
}

impl Yacht {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Yacht {
            sheet: [[None; CATS]; 2],
            side: Player::One,
            kept: Vec::new(),
            dice: Vec::new(),
            rolls_used: 0,
            rng: Rng::new(seed),
            log: String::new(),
        }
    }

    pub fn total(&self, p: Player) -> u32 {
        self.sheet[p.idx()]
            .iter()
            .flatten()
            .map(|&x| x as u32)
            .sum()
    }

    fn rolling(&self) -> bool {
        self.dice.is_empty()
    }

    /// Valoarea (față de PAR) a celei mai bune categorii libere, ca speranță după o aruncare a
    /// `5 - kept.len()` zaruri.
    fn hold_value(&self, p: Player, kept: &[u8]) -> f32 {
        let free = 5 - kept.len();
        let open: Vec<usize> = (0..CATS)
            .filter(|&c| self.sheet[p.idx()][c].is_none())
            .collect();
        if open.is_empty() {
            return 0.0;
        }
        let total = 6usize.pow(free as u32);
        let mut acc = 0.0f32;
        let mut d = [0u8; 5];
        d[..kept.len()].copy_from_slice(kept);
        for code in 0..total {
            let mut x = code;
            for slot in d.iter_mut().skip(kept.len()) {
                *slot = (x % 6) as u8 + 1;
                x /= 6;
            }
            let mut best = f32::MIN;
            for &c in &open {
                best = best.max(score(c, &d) as f32 - PAR[c]);
            }
            acc += best;
        }
        acc / total as f32
    }

    fn sheet_value(&self, p: Player) -> f32 {
        let mut v = 0.0;
        for c in 0..CATS {
            if let Some(s) = self.sheet[p.idx()][c] {
                v += s as f32 - PAR[c];
            }
        }
        v
    }
}

fn subsets(dice: &[u8]) -> Vec<Vec<u8>> {
    let mut out: Vec<Vec<u8>> = Vec::new();
    for mask in 0u8..32 {
        if mask == 31 {
            continue;
        }
        let mut v: Vec<u8> = Vec::new();
        for (i, &d) in dice.iter().enumerate() {
            if mask & (1 << i) != 0 {
                v.push(d);
            }
        }
        v.sort_unstable();
        if !out.contains(&v) {
            out.push(v);
        }
    }
    out
}

impl Game for Yacht {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        if self.rolling() {
            return vec![Move::Roll];
        }
        let mut v = Vec::new();
        if self.rolls_used < 3 {
            for k in subsets(&self.dice) {
                v.push(Move::Keep(k));
            }
        }
        for c in 0..CATS {
            if self.sheet[self.side.idx()][c].is_none() {
                v.push(Move::Score(c));
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        let i = self.side.idx();
        match m {
            Move::Roll => {
                let mut d = self.kept.clone();
                for _ in 0..(5 - self.kept.len()) {
                    d.push(self.rng.d6());
                }
                d.sort_unstable();
                self.rolls_used += 1;
                self.log = format!(
                    "Jucătorul {}: aruncarea {} → {:?}",
                    i + 1,
                    self.rolls_used,
                    d
                );
                self.dice = d;
                self.kept.clear();
            }
            Move::Keep(k) => {
                self.kept = k.clone();
                self.dice.clear();
            }
            Move::Score(c) => {
                let s = score(*c, &self.dice);
                self.sheet[i][*c] = Some(s);
                self.log = format!(
                    "Jucătorul {}: {:?} → {} = {}.",
                    i + 1,
                    self.dice,
                    CAT_NAMES[*c],
                    s
                );
                self.dice.clear();
                self.kept.clear();
                self.rolls_used = 0;
                self.side = self.side.other();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.sheet.iter().all(|s| s.iter().all(|x| x.is_some())) {
            let (a, b) = (self.total(Player::One), self.total(Player::Two));
            Some(match a.cmp(&b) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                std::cmp::Ordering::Equal => Outcome::Draw,
            })
        } else {
            None
        }
    }

    fn evaluate(&self, p: Player) -> i32 {
        let mut mine = self.sheet_value(p);
        let theirs = self.sheet_value(p.other());
        if self.side == p {
            if !self.rolling() {
                // pozițiile după „scor” sunt deja în foaie; aici nu e nimic de adăugat
            } else if self.rolls_used > 0 && self.rolls_used < 3 {
                mine += self.hold_value(p, &self.kept);
            } else if self.rolls_used == 0 && self.kept.is_empty() {
                // început de tură: speranța unei aruncări complete
                mine += 0.0;
            }
        }
        ((mine - theirs) * 10.0) as i32
    }

    fn render(&self) -> String {
        let mut s = String::from("Yacht\n");
        s.push_str("categorie        J1(X)  J2(O)\n");
        for (c, name) in CAT_NAMES.iter().enumerate() {
            let cell = |p: usize| match self.sheet[p][c] {
                Some(v) => format!("{v:>5}"),
                None => "    .".to_string(),
            };
            s.push_str(&format!("{:<15} {}  {}\n", name, cell(0), cell(1)));
        }
        s.push_str(&format!(
            "Total: {} / {}\n",
            self.total(Player::One),
            self.total(Player::Two)
        ));
        if self.outcome().is_none() {
            s.push_str(&format!("La mutare: jucătorul {}\n", self.side.idx() + 1));
            if self.rolling() {
                s.push_str(&format!(
                    "Ținute: {:?}  aruncări folosite: {}  (scrie «arunc»)\n",
                    self.kept, self.rolls_used
                ));
            } else {
                s.push_str(&format!(
                    "Zaruri: {:?}  aruncări folosite: {}/3\n",
                    self.dice, self.rolls_used
                ));
            }
        }
        s.push_str(self.log.trim());
        s.push('\n');
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Roll => "arunc".to_string(),
            Move::Keep(k) => {
                if k.is_empty() {
                    "tine -".to_string()
                } else {
                    format!(
                        "tine {}",
                        k.iter()
                            .map(|d| d.to_string())
                            .collect::<Vec<_>>()
                            .join(" ")
                    )
                }
            }
            Move::Score(c) => format!("scor {}", CAT_NAMES[*c]),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scoring_categories() {
        assert_eq!(score(0, &[1, 1, 3, 4, 1]), 3);
        assert_eq!(score(5, &[6, 6, 2, 2, 1]), 12);
        assert_eq!(score(6, &[3, 3, 2, 5, 1]), 14);
        assert_eq!(score(7, &[4, 4, 4, 4, 1]), 16);
        assert_eq!(score(7, &[4, 4, 4, 2, 1]), 0);
        assert_eq!(score(8, &[2, 2, 5, 5, 5]), 19);
        assert_eq!(score(8, &[5, 5, 5, 5, 2]), 0);
        assert_eq!(score(8, &[5, 5, 5, 5, 5]), 0);
        assert_eq!(score(9, &[1, 2, 3, 4, 5]), 30);
        assert_eq!(score(9, &[2, 3, 4, 5, 6]), 0);
        assert_eq!(score(10, &[2, 3, 4, 5, 6]), 30);
        assert_eq!(score(11, &[6, 6, 6, 6, 6]), 50);
        assert_eq!(score(11, &[6, 6, 6, 6, 5]), 0);
    }

    #[test]
    fn turn_flow_roll_keep_roll_score() {
        let mut g = Yacht::with_seed(5);
        assert_eq!(g.legal_moves(), vec![Move::Roll]);
        g.apply(&Move::Roll);
        assert_eq!(g.dice.len(), 5);
        assert_eq!(g.rolls_used, 1);
        let keep = g
            .legal_moves()
            .into_iter()
            .find(|m| matches!(m, Move::Keep(k) if k.len() == 2))
            .unwrap();
        g.apply(&keep);
        assert_eq!(g.legal_moves(), vec![Move::Roll]);
        g.apply(&Move::Roll);
        g.apply(&Move::Keep(vec![]));
        g.apply(&Move::Roll);
        assert_eq!(g.rolls_used, 3);
        // la a treia aruncare nu mai poți ține
        assert!(g.legal_moves().iter().all(|m| matches!(m, Move::Score(_))));
        assert_eq!(g.legal_moves().len(), CATS);
        g.apply(&Move::Score(6));
        assert_eq!(g.side, Player::Two);
        assert!(g.sheet[0][6].is_some());
        assert_eq!(g.rolls_used, 0);
    }

    #[test]
    fn kept_dice_are_preserved_by_the_roll() {
        let mut g = Yacht::with_seed(9);
        g.apply(&Move::Roll);
        let k = vec![g.dice[0], g.dice[1]];
        g.apply(&Move::Keep(k.clone()));
        g.apply(&Move::Roll);
        for d in &k {
            let need = k.iter().filter(|x| *x == d).count();
            assert!(g.dice.iter().filter(|x| *x == d).count() >= need);
        }
    }

    #[test]
    fn keep_moves_are_distinct_multisets() {
        let mut g = Yacht::with_seed(1);
        g.dice = vec![3, 3, 3, 5, 5];
        g.rolls_used = 1;
        let ks: Vec<Vec<u8>> = g
            .legal_moves()
            .into_iter()
            .filter_map(|m| if let Move::Keep(k) = m { Some(k) } else { None })
            .collect();
        // multiseturi proprii: (3+1)*(2+1) - 1 = 11
        assert_eq!(ks.len(), 11);
        assert!(!ks.contains(&vec![3, 3, 3, 5, 5]));
    }

    #[test]
    fn totals() {
        let mut g = Yacht::with_seed(1);
        g.sheet[0][..3].copy_from_slice(&[Some(3), Some(6), Some(9)]);
        assert_eq!(g.total(Player::One), 18);
    }

    #[test]
    fn full_games_end_with_all_cells_filled() {
        for seed in 1..=20u64 {
            let mut g = Yacht::with_seed(seed);
            let mut rng = Rng::new(seed + 100);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                let pick = m[rng.below(m.len())].clone();
                g.apply(&pick);
                n += 1;
                assert!(n < 2000);
            }
            for p in 0..2 {
                assert!(g.sheet[p].iter().all(|x| x.is_some()));
            }
        }
    }

    #[test]
    fn evaluate_does_not_depend_on_future_dice() {
        // două semințe diferite, aceeași stare vizibilă după „tine”: aceeași evaluare
        let mut a = Yacht::with_seed(1);
        let mut b = Yacht::with_seed(2);
        a.dice = vec![1, 2, 3, 4, 4];
        b.dice = vec![1, 2, 3, 4, 4];
        a.rolls_used = 1;
        b.rolls_used = 1;
        a.apply(&Move::Keep(vec![4, 4]));
        b.apply(&Move::Keep(vec![4, 4]));
        assert_eq!(a.evaluate(Player::One), b.evaluate(Player::One));
    }

    #[test]
    fn ai_scores_obvious_five_of_a_kind() {
        let mut g = Yacht::with_seed(1);
        g.dice = vec![6, 6, 6, 6, 6];
        g.rolls_used = 1;
        let best = polytabletop_core::ai::best_move(&g, 3).unwrap();
        assert!(matches!(best, Move::Score(11)), "{best:?}");
    }
}
