#![allow(clippy::doc_lazy_continuation)]
//! Drop Dead (SUA), joc de zaruri cu cinci zaruri, fără decizii: se joacă din „arunc”.
//!
//! * La tura ta arunci toate zarurile rămase. Dacă nu apare niciun 2 sau 5, adaugi totalul la
//!   scor. Dacă apar 2 sau 5, rularea nu punctează, iar zarurile cu 2 și 5 „cad” (se pun deoparte);
//!   continui cu restul. Tura se termină când toate cele cinci zaruri au căzut („drop dead”).
//! * Punctele câștigate mai devreme în tură se păstrează. Fiecare jucător are 5 ture; cine
//!   are scorul total mai mare câștigă (egalitate = remiză). Numărul de ture e o regulă a
//!   implementării (sursele nu fixează sfârșitul jocului).
//! Mutare: `arunc`.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const TURNS: u8 = 5;

#[derive(Clone)]
pub struct DropDead {
    rng: Rng,
    score: [u32; 2],
    turns: [u8; 2],
    side: Player,
    dice: u8,
    last: String,
}

impl Default for DropDead {
    fn default() -> Self {
        Self::new()
    }
}

impl DropDead {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        DropDead {
            rng: Rng::new(seed),
            score: [0; 2],
            turns: [0; 2],
            side: Player::One,
            dice: 5,
            last: String::new(),
        }
    }

    fn over(&self) -> bool {
        self.turns[0] >= TURNS && self.turns[1] >= TURNS
    }
}

impl Game for DropDead {
    type Move = ();

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<()> {
        if self.over() {
            Vec::new()
        } else {
            vec![()]
        }
    }

    fn apply(&mut self, _: &()) {
        let i = self.side.idx();
        let roll: Vec<u8> = (0..self.dice).map(|_| self.rng.d6()).collect();
        let dropped = roll.iter().filter(|&&d| d == 2 || d == 5).count() as u8;
        let txt: Vec<String> = roll.iter().map(|d| d.to_string()).collect();
        if dropped == 0 {
            let t: u32 = roll.iter().map(|&d| d as u32).sum();
            self.score[i] += t;
            self.last = format!("J{}: {} → +{t}", i + 1, txt.join(" "));
        } else {
            self.dice -= dropped;
            self.last = format!(
                "J{}: {} → cad {dropped} zaruri, 0 puncte",
                i + 1,
                txt.join(" ")
            );
        }
        if self.dice == 0 {
            self.turns[i] += 1;
            self.dice = 5;
            self.side = self.side.other();
            if self.turns[self.side.idx()] >= TURNS {
                self.side = self.side.other();
            }
        }
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

    fn render(&self) -> String {
        format!(
            "Drop Dead — tura {}/{TURNS} (J1) și {}/{TURNS} (J2).\nScor: J1 {} – J2 {}. La mutare: J{} cu {} zaruri.\n{}\n",
            (self.turns[0] + 1).min(TURNS),
            (self.turns[1] + 1).min(TURNS),
            self.score[0],
            self.score[1],
            self.side.idx() + 1,
            self.dice,
            self.last
        )
    }

    fn format_move(&self, _: &()) -> String {
        "arunc".into()
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn games_end_and_dice_only_fall_on_two_or_five() {
        for seed in 1..=20u64 {
            let mut g = DropDead::with_seed(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let before = g.dice;
                g.apply(&());
                assert!(g.dice <= 5 && (g.dice <= before || g.dice == 5));
                n += 1;
                assert!(n < 5000);
            }
            assert_eq!(g.turns, [TURNS, TURNS]);
        }
    }
}
