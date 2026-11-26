//! Capra (Leapfrog, „sărit capra”) — cursă de broscuțe/copii pe un șir de 10 căsuțe.
//!
//! * Fiecare jucător are 3 „săritori”: J1 pe căsuțele 1–3 (merge spre 10), J2 pe 8–10 (merge spre 1).
//! * La mutare: un săritor face un pas înainte pe o căsuță liberă (`3-4`) sau **sare capra**: peste un
//!   săritor vecin (oricare) pe căsuța liberă de după el (`3-5`). Nu se merge înapoi.
//! * Câștigă primul care aduce toți cei 3 săritori pe ultimele 3 căsuțe din partea opusă. Dacă nu poți muta,
//!   spui `pas`; dacă nimeni nu mai poate muta, câștigă cine a înaintat mai mult (egal = remiză).
//!   Mutări: `de-la-la` cu numere de căsuțe, de ex. `3-5`, sau `pas`.

use crate::q5_util::{by_score, verdict};
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: usize = 10;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Hop(u8, u8),
    Pass,
}

#[derive(Clone)]
pub struct Capra {
    cell: [u8; N], // 0 liber, 1 = J1, 2 = J2
    turn: Player,
}

impl Default for Capra {
    fn default() -> Self {
        Self::new()
    }
}

impl Capra {
    pub fn new() -> Self {
        let mut cell = [0u8; N];
        for i in 0..3 {
            cell[i] = 1;
            cell[N - 1 - i] = 2;
        }
        Capra {
            cell,
            turn: Player::One,
        }
    }

    fn dir(p: Player) -> i32 {
        if p == Player::One {
            1
        } else {
            -1
        }
    }

    fn moves_of(&self, p: Player) -> Vec<Move> {
        let me = p.idx() as u8 + 1;
        let d = Self::dir(p);
        let mut v = Vec::new();
        for i in 0..N {
            if self.cell[i] != me {
                continue;
            }
            let a = i as i32 + d;
            if a < 0 || a >= N as i32 {
                continue;
            }
            if self.cell[a as usize] == 0 {
                v.push(Move::Hop(i as u8 + 1, a as u8 + 1));
            } else {
                let b = i as i32 + 2 * d;
                if b >= 0 && b < N as i32 && self.cell[b as usize] == 0 {
                    v.push(Move::Hop(i as u8 + 1, b as u8 + 1));
                }
            }
        }
        v
    }

    /// Progresul total al lui `p` (suma distanțelor parcurse de la start).
    pub fn progress(&self, p: Player) -> i32 {
        let me = p.idx() as u8 + 1;
        let mut s = 0;
        let mut k = 0;
        for i in 0..N {
            if self.cell[i] == me {
                let pos = if p == Player::One {
                    i as i32
                } else {
                    (N - 1 - i) as i32
                };
                s += pos - k; // k = indicele săritorului în ordinea de pornire (0,1,2)
                k += 1;
            }
        }
        s
    }

    fn home(&self, p: Player) -> bool {
        let me = p.idx() as u8 + 1;
        (0..3).all(|k| {
            let i = if p == Player::One { N - 1 - k } else { k };
            self.cell[i] == me
        })
    }
}

impl Game for Capra {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.turn
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        let v = self.moves_of(self.turn);
        if v.is_empty() {
            vec![Move::Pass]
        } else {
            v
        }
    }

    fn apply(&mut self, m: &Move) {
        if let Move::Hop(a, b) = m {
            let (a, b) = (*a as usize - 1, *b as usize - 1);
            self.cell[b] = self.cell[a];
            self.cell[a] = 0;
        }
        self.turn = self.turn.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.home(Player::One) {
            return Some(Outcome::Win(Player::One));
        }
        if self.home(Player::Two) {
            return Some(Outcome::Win(Player::Two));
        }
        if self.moves_of(Player::One).is_empty() && self.moves_of(Player::Two).is_empty() {
            return Some(by_score(
                self.progress(Player::One),
                self.progress(Player::Two),
            ));
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        (self.progress(p) - self.progress(p.other())) * 10 + self.moves_of(p).len() as i32
            - self.moves_of(p.other()).len() as i32
    }

    fn render(&self) -> String {
        let mut s = String::from("Capra — J1 (A) spre dreapta, J2 (B) spre stânga\n ");
        for i in 0..N {
            s.push_str(&format!("{:>3}", i + 1));
        }
        s.push_str("\n ");
        for i in 0..N {
            s.push_str(match self.cell[i] {
                1 => "  A",
                2 => "  B",
                _ => "  .",
            });
        }
        s.push('\n');
        let v = verdict(self.outcome());
        if v.is_empty() {
            s.push_str(&format!("La mutare: Jucător {}\n", self.turn.idx() + 1));
        } else {
            s.push_str(&v);
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Hop(a, b) => format!("{a}-{b}"),
            Move::Pass => "pas".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q5_util::testkit::check_game;

    #[test]
    fn opening_moves_and_leap() {
        let g = Capra::new();
        // pas 3-4 sau săritura lui 2 peste 3 pe 4
        assert_eq!(g.legal_moves(), vec![Move::Hop(2, 4), Move::Hop(3, 4)]);
        let mut h = g.clone();
        h.cell = [1, 1, 0, 1, 2, 0, 2, 2, 0, 0];
        // J1 pe 4 sare peste 5 (J2) pe 6
        assert!(h.moves_of(Player::One).contains(&Move::Hop(4, 6)));
    }

    #[test]
    fn win_and_progress() {
        let mut g = Capra::new();
        g.cell = [0, 0, 0, 0, 0, 0, 0, 1, 1, 1];
        assert!(g.home(Player::One));
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn random_and_ai_games() {
        check_game(|_| Capra::new(), 6, 400);
    }
}
