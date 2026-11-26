//! Generala (Argentina / Uruguay), 2 jucători, cinci zaruri, din regulile publice.
//!
//! * 10 ture pe jucător; în fiecare tură: până la 3 aruncări cu 5 zaruri; după fiecare
//!   aruncare (primele două) **ții** unele zaruri (`tine 3 3 5`, `tine -` = niciunul) și
//!   rearunci restul, sau **scrii** scorul într-o categorie liberă (`scor escalera`). După a
//!   treia aruncare trebuie să scrii; o categorie care nu se potrivește se **taie** (0 puncte).
//! * Categorii: `unu`…`sase` (suma zarurilor cu acea valoare), `escalera` (1-2-3-4-5, 2-3-4-5-6
//!   sau 3-4-5-6-1) 20, `full` (trei + doi) 30, `poker` (patru la fel; cinci la fel
//!   merge și ea) 40, `generala` (cinci la fel) 50.
//! * **Servida**: escalera, full sau poker făcute din prima aruncare a turei se plătesc cu
//!   5 puncte în plus (25 / 35 / 45). **Generala servida** (cinci la fel din prima aruncare)
//!   câștigă jocul pe loc, cu excepția cazului în care categoria `generala` a fost deja tăiată.
//! * Varianta „generala dublă” (100) nu se joacă. Câștigă cel mai mare total după 10 ture.
//! * Aruncarea e mutare separată, forțată (`arunc`), ca AI-ul să nu vadă zarurile viitoare.
//!
//! AI: `evaluate` folosește doar scorurile, zarurile tale actuale și zarurile ținute; valoarea unei
//! ținute este speranța exactă a celei mai bune categorii după o aruncare a zarurilor rămase,
//! față de o valoare „de bază” a fiecărei categorii.

#![allow(clippy::needless_range_loop)]

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const CATS: usize = 10;
pub const CAT_NAMES: [&str; CATS] = [
    "unu", "doi", "trei", "patru", "cinci", "sase", "escalera", "full", "poker", "generala",
];
/// Valori medii „de așteptat” per categorie (pentru euristica AI).
const PAR: [f32; CATS] = [2.1, 5.3, 8.6, 12.2, 15.7, 19.2, 12.0, 14.0, 12.0, 6.0];

/// Scorul unei categorii pentru 5 zaruri; `served` = făcut din prima aruncare a turei.
pub fn score(cat: usize, dice: &[u8], served: bool) -> u8 {
    let mut cnt = [0u8; 7];
    for &d in dice {
        cnt[d as usize] += 1;
    }
    let max = *cnt[1..].iter().max().unwrap();
    let bonus = if served { 5 } else { 0 };
    match cat {
        0..=5 => cnt[cat + 1] * (cat as u8 + 1),
        6 => {
            let straight = (cnt[1..=5].iter().all(|&c| c == 1))
                || (cnt[2..=6].iter().all(|&c| c == 1))
                || (cnt[3..=6].iter().all(|&c| c == 1) && cnt[1] == 1);
            if straight {
                20 + bonus
            } else {
                0
            }
        }
        7 => {
            if cnt[1..].contains(&3) && cnt[1..].contains(&2) {
                30 + bonus
            } else {
                0
            }
        }
        8 => {
            if max >= 4 {
                40 + bonus
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
pub struct Generala {
    sheet: [[Option<u8>; CATS]; 2],
    side: Player,
    /// Zaruri ținute (înaintea aruncării următoare), sortate.
    kept: Vec<u8>,
    /// Zarurile curente (după aruncare), sortate. Gol când urmează o aruncare.
    dice: Vec<u8>,
    rolls_used: u8,
    rng: Rng,
    log: String,
    /// Câștigător instantaneu (generala servida).
    instant: Option<Player>,
}

impl Default for Generala {
    fn default() -> Self {
        Self::new()
    }
}

impl Generala {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Generala {
            sheet: [[None; CATS]; 2],
            side: Player::One,
            kept: Vec::new(),
            dice: Vec::new(),
            rolls_used: 0,
            rng: Rng::new(seed),
            log: String::new(),
            instant: None,
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

    /// Speranța (față de PAR) a celei mai bune categorii libere după o aruncare a
    /// `5 - kept.len()` zaruri (a doua sau a treia aruncare: fără servida).
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
                best = best.max(score(c, &d, false) as f32 - PAR[c]);
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

impl Game for Generala {
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
                if self.rolls_used == 1
                    && d.iter().all(|&x| x == d[0])
                    && self.sheet[i][9] != Some(0)
                {
                    self.instant = Some(self.side);
                    self.log.push_str(" — GENERALA SERVIDA, câștigi jocul!");
                }
                self.dice = d;
                self.kept.clear();
            }
            Move::Keep(k) => {
                self.kept = k.clone();
                self.dice.clear();
            }
            Move::Score(c) => {
                let s = score(*c, &self.dice, self.rolls_used == 1);
                self.sheet[i][*c] = Some(s);
                self.log = format!(
                    "Jucătorul {}: {:?} → {} = {}{}.",
                    i + 1,
                    self.dice,
                    CAT_NAMES[*c],
                    s,
                    if s == 0 { " (tăiat)" } else { "" }
                );
                self.dice.clear();
                self.kept.clear();
                self.rolls_used = 0;
                self.side = self.side.other();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if let Some(p) = self.instant {
            return Some(Outcome::Win(p));
        }
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
        if self.side == p && self.rolling() && self.rolls_used > 0 && self.rolls_used < 3 {
            mine += self.hold_value(p, &self.kept);
        }
        ((mine - theirs) * 10.0) as i32
    }

    fn render(&self) -> String {
        let mut s = String::from("Generala\n");
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
        assert_eq!(score(0, &[1, 1, 3, 4, 1], false), 3);
        assert_eq!(score(5, &[6, 6, 2, 2, 1], false), 12);
        // escalera: tre secvențe, cu bonus servida
        assert_eq!(score(6, &[1, 2, 3, 4, 5], false), 20);
        assert_eq!(score(6, &[2, 3, 4, 5, 6], true), 25);
        assert_eq!(score(6, &[1, 3, 4, 5, 6], false), 20);
        assert_eq!(score(6, &[1, 2, 4, 5, 6], false), 0);
        assert_eq!(score(6, &[2, 2, 3, 4, 5], false), 0);
        // full
        assert_eq!(score(7, &[4, 4, 4, 2, 2], false), 30);
        assert_eq!(score(7, &[4, 4, 4, 2, 2], true), 35);
        assert_eq!(score(7, &[4, 4, 4, 4, 2], false), 0);
        assert_eq!(score(7, &[4, 4, 4, 4, 4], false), 0);
        // poker
        assert_eq!(score(8, &[5, 5, 5, 5, 2], false), 40);
        assert_eq!(score(8, &[5, 5, 5, 5, 2], true), 45);
        assert_eq!(score(8, &[5, 5, 5, 5, 5], false), 40);
        assert_eq!(score(8, &[5, 5, 5, 2, 2], false), 0);
        // generala
        assert_eq!(score(9, &[6, 6, 6, 6, 6], false), 50);
        assert_eq!(score(9, &[6, 6, 6, 6, 5], false), 0);
    }

    fn set_dice(g: &mut Generala, d: &[u8], rolls: u8) {
        g.dice = d.to_vec();
        g.rolls_used = rolls;
    }

    #[test]
    fn served_bonus_only_on_first_roll() {
        let mut g = Generala::with_seed(1);
        set_dice(&mut g, &[2, 2, 2, 5, 5], 1);
        g.apply(&Move::Score(7));
        assert_eq!(g.sheet[0][7], Some(35));
        let mut h = Generala::with_seed(1);
        set_dice(&mut h, &[2, 2, 2, 5, 5], 2);
        h.apply(&Move::Score(7));
        assert_eq!(h.sheet[0][7], Some(30));
    }

    #[test]
    fn served_generala_wins_unless_scratched() {
        let mut found = false;
        for seed in 1..20000u64 {
            let mut g = Generala::with_seed(seed);
            g.apply(&Move::Roll);
            if g.instant.is_some() {
                assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
                found = true;
                break;
            }
        }
        assert!(found);
        let mut g = Generala::with_seed(1);
        g.sheet[0][9] = Some(0);
        for seed in 1..20000u64 {
            let mut h = g.clone();
            h.rng = Rng::new(seed);
            h.apply(&Move::Roll);
            assert!(h.instant.is_none());
        }
    }

    #[test]
    fn turn_flow_roll_keep_roll_score() {
        let mut g = Generala::with_seed(3);
        assert_eq!(g.legal_moves(), vec![Move::Roll]);
        g.apply(&Move::Roll);
        if g.instant.is_some() {
            return;
        }
        let first = g.dice.clone();
        let keep = vec![first[0]];
        g.apply(&Move::Keep(keep.clone()));
        assert_eq!(g.legal_moves(), vec![Move::Roll]);
        g.apply(&Move::Roll);
        assert!(g.dice.contains(&keep[0]));
        assert_eq!(g.rolls_used, 2);
        g.apply(&Move::Score(0));
        assert_eq!(g.to_move(), Player::Two);
        assert_eq!(g.rolls_used, 0);
    }

    #[test]
    fn third_roll_forces_scoring() {
        let mut g = Generala::with_seed(2);
        set_dice(&mut g, &[1, 2, 3, 4, 6], 3);
        assert!(g.legal_moves().iter().all(|m| matches!(m, Move::Score(_))));
        assert_eq!(g.legal_moves().len(), CATS);
    }

    #[test]
    fn full_games_end_with_all_cells_filled_or_instant() {
        for seed in 1..=20u64 {
            let mut g = Generala::with_seed(seed);
            let mut rng = Rng::new(seed * 5);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 5000);
            }
            if g.instant.is_none() {
                assert!(g.sheet.iter().all(|s| s.iter().all(|x| x.is_some())));
            }
        }
    }

    #[test]
    fn evaluate_does_not_depend_on_future_dice() {
        let mut g = Generala::with_seed(5);
        set_dice(&mut g, &[1, 2, 3, 3, 6], 1);
        let mut a = g.clone();
        a.apply(&Move::Keep(vec![3, 3]));
        let mut b = a.clone();
        b.rng = Rng::new(999);
        assert_eq!(a.evaluate(Player::One), b.evaluate(Player::One));
    }

    #[test]
    fn ai_scores_obvious_poker() {
        use polytabletop_core::GameObj;
        let mut g = Generala::with_seed(7);
        set_dice(&mut g, &[4, 4, 4, 4, 1], 3);
        let b: Box<dyn GameObj> = Box::new(g);
        assert_eq!(b.ai_move(1).unwrap(), "scor poker");
    }
}
