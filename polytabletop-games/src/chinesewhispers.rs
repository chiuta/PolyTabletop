//! Chinese Whispers (Telefonul fără fir), duel asimetric pentru 2 jucători.
//!
//! * Mesajul = 4 cifre. Într-o rundă un jucător e „trimițătorul” (vede mesajul), celălalt „receptorul”.
//!   Trimițătorul poate lăsa mesajul neschimbat (`las`) sau poate strica o cifră cu ±1 (`strica 2+`, `strica 3-`…).
//!   Receptorul vede mesajul ajuns și suma de control (suma cifrelor originale mod 10, spusă în clar), apoi
//!   „restaurează”: arată ce cifră crede că a fost stricată (`restaur 1`…`restaur 4`) sau zice `restaur 0` (nimic stricat).
//! * Receptorul câștigă punctul dacă mesajul restaurat e identic cu originalul, altfel punctul merge la trimițător.
//!   4 runde, rolurile alternează. Mai multe puncte câștigă, egal = remiză.
//! * Informație ascunsă: originalul și stricarea rămân ascunse receptorului până la final de rundă.

use crate::q2_util::{by_score, end_line};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const ROUNDS: usize = 6;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Keep,
    Alter(u8, i8),
    Restore(u8),
    Next,
}

#[derive(Clone)]
pub struct ChineseWhispers {
    rng: Rng,
    orig: [u8; 4],
    cur: [u8; 4],
    altered: bool,
    round: usize,
    score: [i32; 2],
    viewer: Option<Player>,
    last: String,
    fresh: bool,
    pending: Option<u8>,
}

impl Default for ChineseWhispers {
    fn default() -> Self {
        Self::new()
    }
}

impl ChineseWhispers {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut g = ChineseWhispers {
            rng: Rng::new(seed),
            orig: [0; 4],
            cur: [0; 4],
            altered: false,
            round: 0,
            score: [0; 2],
            viewer: Some(Player::One),
            last: String::new(),
            fresh: true,
            pending: None,
        };
        g.new_message();
        g
    }
    fn new_message(&mut self) {
        for i in 0..4 {
            self.orig[i] = self.rng.below(10) as u8;
        }
        self.cur = self.orig;
        self.altered = false;
        self.fresh = true;
        self.pending = None;
    }
    fn sender(&self) -> Player {
        if self.round.is_multiple_of(2) {
            Player::One
        } else {
            Player::Two
        }
    }
    fn over(&self) -> bool {
        self.round >= ROUNDS
    }
    fn checksum(&self) -> u8 {
        self.orig.iter().sum::<u8>() % 10
    }
}

fn digits(d: &[u8; 4]) -> String {
    d.iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

impl Game for ChineseWhispers {
    type Move = Move;
    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }
    fn to_move(&self) -> Player {
        if self.pending.is_some() {
            return self.sender();
        }
        if self.fresh {
            self.sender()
        } else {
            self.sender().other()
        }
    }
    fn legal_moves(&self) -> Vec<Move> {
        if self.over() {
            return Vec::new();
        }
        if self.pending.is_some() {
            return vec![Move::Next];
        }
        if self.fresh {
            let mut v = vec![Move::Keep];
            for i in 1..=4u8 {
                for d in [1i8, -1] {
                    let x = self.orig[i as usize - 1] as i8 + d;
                    if (0..=9).contains(&x) {
                        v.push(Move::Alter(i, d));
                    }
                }
            }
            v
        } else {
            (0..=4u8).map(Move::Restore).collect()
        }
    }
    fn apply(&mut self, m: &Move) {
        match *m {
            Move::Keep => self.fresh = false,
            Move::Alter(i, d) => {
                let c = &mut self.cur[i as usize - 1];
                *c = (*c as i8 + d) as u8;
                self.altered = true;
                self.fresh = false;
            }
            Move::Restore(i) => {
                let mut fixed = self.cur;
                if i > 0 {
                    // corectează cifra astfel încât suma de control să se potrivească
                    let sum: i32 = self.cur.iter().map(|&x| x as i32).sum();
                    let want = self.checksum() as i32;
                    let diff = (want - sum).rem_euclid(10);
                    let adj = if diff == 1 {
                        1
                    } else if diff == 9 {
                        -1
                    } else {
                        0
                    };
                    // fără nepotrivire nu există ce corecta: „restaurarea” strică o cifră intactă
                    let adj = if adj == 0 { 1 } else { adj };
                    let x = (fixed[i as usize - 1] as i32 + adj).rem_euclid(10);
                    fixed[i as usize - 1] = x as u8;
                }
                let ok = fixed == self.orig;
                let r = self.sender().other().idx();
                let s = self.sender().idx();
                if ok {
                    self.score[r] += 1;
                } else {
                    self.score[s] += 1;
                }
                self.last = format!(
                    "original {} → ajuns {} → restaurat {} ({})",
                    digits(&self.orig),
                    digits(&self.cur),
                    digits(&fixed),
                    if ok {
                        "receptorul a nimerit"
                    } else {
                        "receptorul a greșit"
                    }
                );
                self.pending = Some(i);
            }
            Move::Next => {
                self.round += 1;
                if !self.over() {
                    self.new_message();
                }
            }
        }
    }
    fn outcome(&self) -> Option<Outcome> {
        if self.over() {
            Some(by_score(self.score[0], self.score[1]))
        } else {
            None
        }
    }
    fn evaluate(&self, p: Player) -> i32 {
        (self.score[p.idx()] - self.score[1 - p.idx()]) * 100
    }
    fn render(&self) -> String {
        let mut s = format!(
            "Telefonul fără fir — runda {}/{ROUNDS}; trimițător: J{}. Puncte: J1 {}, J2 {}\n",
            (self.round + 1).min(ROUNDS),
            self.sender().idx() + 1,
            self.score[0],
            self.score[1]
        );
        if !self.last.is_empty() {
            s.push_str(&format!("Runda precedentă: {}\n", self.last));
        }
        if !self.over() {
            let me = self.viewer.unwrap_or_else(|| self.to_move());
            if me == self.sender() {
                s.push_str(&format!("Mesajul tău original: {}\n", digits(&self.orig)));
                if !self.fresh {
                    s.push_str("Ai trimis mesajul; receptorul alege.\n");
                }
            } else if self.fresh {
                s.push_str("Trimițătorul pregătește mesajul.\n");
            } else {
                s.push_str(&format!(
                    "Mesaj ajuns: {} | suma de control a originalului (mod 10): {}\n",
                    digits(&self.cur),
                    self.checksum()
                ));
            }
        }
        s.push_str(&end_line(self.outcome(), self.to_move()));
        s
    }
    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Keep => "las".into(),
            Move::Alter(i, d) => format!("strica {}{}", i, if *d > 0 { '+' } else { '-' }),
            Move::Restore(i) => format!("restaur {i}"),
            Move::Next => "urmeaza".into(),
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
    fn restoring_the_right_digit_wins_the_point() {
        let mut g = ChineseWhispers::with_seed(9);
        g.orig = [1, 2, 3, 4];
        g.cur = g.orig;
        g.apply(&Move::Alter(2, 1));
        assert_eq!(g.cur, [1, 3, 3, 4]);
        g.apply(&Move::Restore(2));
        assert_eq!(g.score, [0, 1]);
    }

    #[test]
    fn wrong_guess_gives_sender_the_point() {
        let mut g = ChineseWhispers::with_seed(9);
        g.orig = [1, 2, 3, 4];
        g.cur = g.orig;
        g.apply(&Move::Alter(2, 1));
        g.apply(&Move::Restore(3));
        assert_eq!(g.score, [1, 0]);
    }

    #[test]
    fn original_hidden_from_receiver() {
        let mut g = ChineseWhispers::with_seed(1);
        g.orig = [7, 7, 7, 7];
        g.cur = g.orig;
        g.apply(&Move::Keep);
        g.set_viewer(Some(Player::Two));
        assert!(!g.render().contains("Mesajul tău original"));
        g.set_viewer(Some(Player::One));
        assert!(g.render().contains("Mesajul tău original: 7 7 7 7"));
    }

    #[test]
    fn ai_plays_through() {
        let mut g: Box<dyn polytabletop_core::GameObj> = Box::new(ChineseWhispers::with_seed(5));
        let mut k = 0;
        while g.outcome().is_none() {
            let m = g.ai_move(k).unwrap();
            g.play(&m).unwrap();
            k += 1;
            assert!(k < 100);
        }
    }
}
