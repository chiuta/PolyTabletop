//! Motor comun pentru jocurile de zaruri cu foaie de scor (Balut, Crag), 2 jucători.
//!
//! Fluxul unei ture (ca la „Cinci Zaruri”): `arunc` (mutare separată, ca AI-ul să nu vadă zarurile
//! viitoare), apoi `tine 3 3 5` (ții zarurile spuse și le rearunci pe celelalte; `tine -` = niciunul)
//! sau `scor <categorie>` (scrii în foaie; o categorie care nu se potrivește primește 0 = „tăiere”).
//! După ultima aruncare permisă trebuie să scrii. AI: speranța exactă a celei mai bune categorii
//! libere după rearuncare, față de o valoare „de bază” a fiecărei categorii.

use crate::q5_util::{by_score, verdict};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::marker::PhantomData;

pub trait SheetSpec: Clone + 'static {
    const TITLE: &'static str;
    const NDICE: usize;
    const MAX_ROLLS: u8;
    const CATS: &'static [&'static str];
    /// Valoarea medie „de așteptat” a fiecărei categorii (euristică AI).
    const PAR: &'static [f32];
    fn score(cat: usize, dice: &[u8]) -> i32;
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Roll,
    Keep(Vec<u8>),
    Score(usize),
}

#[derive(Clone)]
pub struct Sheet<S: SheetSpec> {
    sheet: [Vec<Option<i32>>; 2],
    side: Player,
    kept: Vec<u8>,
    dice: Vec<u8>,
    rolls_used: u8,
    rng: Rng,
    log: String,
    _m: PhantomData<S>,
}

impl<S: SheetSpec> Default for Sheet<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: SheetSpec> Sheet<S> {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let n = S::CATS.len();
        Sheet {
            sheet: [vec![None; n], vec![None; n]],
            side: Player::One,
            kept: Vec::new(),
            dice: Vec::new(),
            rolls_used: 0,
            rng: Rng::new(seed),
            log: String::new(),
            _m: PhantomData,
        }
    }

    pub fn total(&self, p: Player) -> i32 {
        self.sheet[p.idx()].iter().flatten().sum()
    }

    pub fn dice(&self) -> &[u8] {
        &self.dice
    }

    pub fn set_dice_for_test(&mut self, d: &[u8], rolls_used: u8) {
        self.dice = d.to_vec();
        self.rolls_used = rolls_used;
    }

    fn rolling(&self) -> bool {
        self.dice.is_empty()
    }

    fn hold_value(&self, p: Player, kept: &[u8]) -> f32 {
        let free = S::NDICE - kept.len();
        let open: Vec<usize> = (0..S::CATS.len())
            .filter(|&c| self.sheet[p.idx()][c].is_none())
            .collect();
        if open.is_empty() {
            return 0.0;
        }
        let total = 6usize.pow(free as u32);
        let mut acc = 0.0f32;
        let mut d = vec![0u8; S::NDICE];
        d[..kept.len()].copy_from_slice(kept);
        for code in 0..total {
            let mut x = code;
            for slot in d.iter_mut().skip(kept.len()) {
                *slot = (x % 6) as u8 + 1;
                x /= 6;
            }
            let mut best = f32::MIN;
            for &c in &open {
                best = best.max(S::score(c, &d) as f32 - S::PAR[c]);
            }
            acc += best;
        }
        acc / total as f32
    }

    fn sheet_value(&self, p: Player) -> f32 {
        let mut v = 0.0;
        for c in 0..S::CATS.len() {
            if let Some(s) = self.sheet[p.idx()][c] {
                v += s as f32 - S::PAR[c];
            }
        }
        v
    }
}

fn subsets(dice: &[u8]) -> Vec<Vec<u8>> {
    let n = dice.len();
    let mut out: Vec<Vec<u8>> = Vec::new();
    for mask in 0u32..(1 << n) {
        if mask == (1 << n) - 1 {
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

impl<S: SheetSpec> Game for Sheet<S> {
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
        if self.rolls_used < S::MAX_ROLLS {
            for k in subsets(&self.dice) {
                v.push(Move::Keep(k));
            }
        }
        for c in 0..S::CATS.len() {
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
                for _ in 0..(S::NDICE - self.kept.len()) {
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
                let s = S::score(*c, &self.dice);
                self.sheet[i][*c] = Some(s);
                self.log = format!(
                    "Jucătorul {}: {:?} → {} = {}.",
                    i + 1,
                    self.dice,
                    S::CATS[*c],
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
            return Some(by_score(self.total(Player::One), self.total(Player::Two)));
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let mut mine = self.sheet_value(p);
        let theirs = self.sheet_value(p.other());
        if self.side == p && self.rolling() && self.rolls_used > 0 && self.rolls_used < S::MAX_ROLLS
        {
            mine += self.hold_value(p, &self.kept);
        }
        ((mine - theirs) * 10.0) as i32
    }

    fn render(&self) -> String {
        let mut s = format!("{}\ncategorie          J1(X)  J2(O)\n", S::TITLE);
        for (c, name) in S::CATS.iter().enumerate() {
            let cell = |p: usize| match self.sheet[p][c] {
                Some(v) => format!("{v:>5}"),
                None => "    .".to_string(),
            };
            s.push_str(&format!("{:<17} {}  {}\n", name, cell(0), cell(1)));
        }
        s.push_str(&format!(
            "Total: J1 {} – J2 {}\n",
            self.total(Player::One),
            self.total(Player::Two)
        ));
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        if !self.kept.is_empty() {
            s.push_str(&format!("Zaruri ținute: {:?}\n", self.kept));
        }
        let v = verdict(self.outcome());
        if v.is_empty() {
            if self.rolling() {
                s.push_str(&format!(
                    "La mutare: Jucător {} — arunc\n",
                    self.side.idx() + 1
                ));
            } else {
                s.push_str(&format!(
                    "La mutare: Jucător {} — zaruri {:?} (aruncarea {}/{}): tine ... / scor <categorie>\n",
                    self.side.idx() + 1,
                    self.dice,
                    self.rolls_used,
                    S::MAX_ROLLS
                ));
            }
        } else {
            s.push_str(&v);
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Roll => "arunc".into(),
            Move::Keep(k) if k.is_empty() => "tine -".into(),
            Move::Keep(k) => format!(
                "tine {}",
                k.iter()
                    .map(|d| d.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            Move::Score(c) => format!("scor {}", S::CATS[*c]),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

pub fn counts(dice: &[u8]) -> [u8; 7] {
    let mut c = [0u8; 7];
    for &d in dice {
        c[d as usize] += 1;
    }
    c
}

#[cfg(test)]
pub mod tests_support {
    use super::*;

    pub fn evaluate_ignores_future_dice<S: SheetSpec>() {
        let mut a = Sheet::<S>::with_seed(1);
        let mut b = Sheet::<S>::with_seed(2);
        let mut d = vec![1u8; S::NDICE];
        d[0] = 2;
        a.set_dice_for_test(&d, 1);
        b.set_dice_for_test(&d, 1);
        a.apply(&Move::Keep(vec![2]));
        b.apply(&Move::Keep(vec![2]));
        assert_eq!(a.evaluate(Player::One), b.evaluate(Player::One));
    }
}
