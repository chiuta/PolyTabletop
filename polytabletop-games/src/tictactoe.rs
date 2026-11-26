//! X și 0 (Tic-tac-toe): tablă 3×3, X (Jucătorul 1) începe, trei în linie câștigă,
//! tabla plină fără linie = remiză. Mutări: `a1`…`c3` (coloana a–c, rândul 1–3).
//! Joc rezolvat (remiză cu joc corect): AI-ul caută tot arborele și nu trișează.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const LINES: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

#[derive(Clone)]
pub struct TicTacToe {
    b: [u8; 9],
    side: Player,
}

impl Default for TicTacToe {
    fn default() -> Self {
        Self::new()
    }
}

impl TicTacToe {
    pub fn new() -> Self {
        TicTacToe {
            b: [0; 9],
            side: Player::One,
        }
    }

    fn winner(&self) -> Option<u8> {
        LINES
            .iter()
            .find(|l| {
                self.b[l[0]] != 0 && self.b[l[0]] == self.b[l[1]] && self.b[l[1]] == self.b[l[2]]
            })
            .map(|l| self.b[l[0]])
    }
}

fn name(i: usize) -> String {
    format!("{}{}", (b'a' + (i % 3) as u8) as char, i / 3 + 1)
}

impl Game for TicTacToe {
    type Move = u8;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<u8> {
        if self.winner().is_some() {
            return Vec::new();
        }
        (0..9u8).filter(|&i| self.b[i as usize] == 0).collect()
    }

    fn apply(&mut self, m: &u8) {
        self.b[*m as usize] = self.side.idx() as u8 + 1;
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        match self.winner() {
            Some(1) => Some(Outcome::Win(Player::One)),
            Some(_) => Some(Outcome::Win(Player::Two)),
            None if self.b.iter().all(|&c| c != 0) => Some(Outcome::Draw),
            None => None,
        }
    }

    fn evaluate(&self, _p: Player) -> i32 {
        0
    }

    fn render(&self) -> String {
        let mut s = String::from("   a b c\n");
        for r in 0..3 {
            s.push_str(&format!("{}  ", r + 1));
            for c in 0..3 {
                s.push(match self.b[r * 3 + c] {
                    1 => 'X',
                    2 => '0',
                    _ => '.',
                });
                s.push(' ');
            }
            s.push('\n');
        }
        s
    }

    fn format_move(&self, m: &u8) -> String {
        name(*m as usize)
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(9)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    #[test]
    fn nine_opening_moves_and_win_detection() {
        let mut g = TicTacToe::new();
        assert_eq!(g.legal_moves().len(), 9);
        for s in ["a1", "a2", "b1", "b2", "c1"] {
            polytabletop_core::GameObj::play(&mut g, s).unwrap();
        }
        assert_eq!(Game::outcome(&g), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn random_games_end_and_ai_never_loses_to_random() {
        for seed in 1..=20u64 {
            let mut g = TicTacToe::new();
            let mut rng = Rng::new(seed);
            while Game::outcome(&g).is_none() {
                if g.side == Player::Two {
                    let m = polytabletop_core::GameObj::ai_move(&g, rng.next_u64()).unwrap();
                    polytabletop_core::GameObj::play(&mut g, &m).unwrap();
                } else {
                    let mv = g.legal_moves();
                    g.apply(&mv[rng.below(mv.len())]);
                }
            }
            assert_ne!(Game::outcome(&g), Some(Outcome::Win(Player::One)));
        }
    }
}
