//! PolyTabletop core.
//!
//! Every game implements [`Game`] (typed, perfect-information, two players).
//! A blanket implementation turns each `Game` into an object-safe [`GameObj`]
//! so that front-ends (CLI, GUI) can handle all games uniformly through text
//! moves.

pub mod ai;
pub mod i18n;
pub use i18n::APP_NAME;
pub mod rng;

use rng::Rng;
use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Player {
    One,
    Two,
}

impl Player {
    pub fn other(self) -> Player {
        match self {
            Player::One => Player::Two,
            Player::Two => Player::One,
        }
    }
    pub fn idx(self) -> usize {
        match self {
            Player::One => 0,
            Player::Two => 1,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Outcome {
    Win(Player),
    Draw,
}

#[derive(Clone, Copy, Debug)]
pub enum AiStyle {
    /// Fixed-depth alpha-beta using `Game::evaluate`.
    AlphaBeta(u32),
    /// UCT Monte-Carlo tree search with this many iterations.
    Mcts(u32),
    /// Iterative-deepening alpha-beta with a time budget (`ms`, overridable with
    /// `ai::set_time_budget_override_ms` or the `POLYTABLETOP_AI_MS` environment variable),
    /// capped at `max_depth` plies. Uses move ordering (previous best, killers,
    /// history, `Game::move_score`), a transposition table when `Game::hash_key`
    /// is provided and quiescence when `Game::quiescence` is true.
    AlphaBetaTime { ms: u32, max_depth: u32 },
}

/// SplitMix64 finaliser: a cheap well-mixed 64-bit hash, for Zobrist-style keys
/// computed on demand (`hash_mix(piece << 16 | square)` XOR-ed over the board).
pub fn hash_mix(x: u64) -> u64 {
    let mut z = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

pub trait Game: Clone {
    type Move: Clone + PartialEq + Debug;

    fn to_move(&self) -> Player;
    /// All legal moves for the side to move. Empty only when the game is over.
    fn legal_moves(&self) -> Vec<Self::Move>;
    fn apply(&mut self, m: &Self::Move);
    fn outcome(&self) -> Option<Outcome>;
    /// Heuristic value of the position from `p`'s point of view.
    fn evaluate(&self, _p: Player) -> i32 {
        0
    }
    fn render(&self) -> String;
    /// Canonical text of a move. Players type exactly this (case-insensitive).
    fn format_move(&self, m: &Self::Move) -> String;
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(4)
    }
    /// Cheap random move for MCTS playouts. Default: uniform over legal moves.
    fn playout_move(&self, rng: &mut Rng) -> Option<Self::Move> {
        let mv = self.legal_moves();
        if mv.is_empty() {
            None
        } else {
            Some(mv[rng.below(mv.len())].clone())
        }
    }
    fn playout_cap(&self) -> usize {
        300
    }
    /// Zobrist-style hash of everything that influences the future of the game
    /// (board, side to move, rights, move counters). `None` (default) disables the
    /// transposition table for this game.
    fn hash_key(&self) -> Option<u64> {
        None
    }
    /// Move-ordering score (higher = search first); captures/promotions etc. Default 0.
    fn move_score(&self, _m: &Self::Move) -> i32 {
        0
    }
    /// Small stable id of a move (< 65536) for the history heuristic. `None` disables it.
    fn move_key(&self, _m: &Self::Move) -> Option<u32> {
        None
    }
    /// True if the game wants quiescence search at the horizon.
    fn quiescence(&self) -> bool {
        false
    }
    /// Moves explored by quiescence search (captures...). Default none.
    fn is_tactical(&self, _m: &Self::Move) -> bool {
        false
    }
    /// Whose private information `render` shows: `Some(p)` = always `p`'s,
    /// `None` = hot-seat (the mover's). Games without hidden hands ignore it.
    fn set_viewer(&mut self, _who: Option<Player>) {}
}

/// Object-safe view of a game, with moves as text.
pub trait GameObj {
    fn render(&self) -> String;
    fn to_move(&self) -> Player;
    fn legal_moves(&self) -> Vec<String>;
    /// Plays a move typed by a person. Returns the canonical move text.
    fn play(&mut self, mv: &str) -> Result<String, String>;
    fn outcome(&self) -> Option<Outcome>;
    /// Chooses (without playing) the AI's move.
    fn ai_move(&self, seed: u64) -> Option<String>;
    fn clone_box(&self) -> Box<dyn GameObj>;
    /// See [`Game::set_viewer`].
    fn set_viewer(&mut self, who: Option<Player>);
}

fn norm(s: &str) -> String {
    s.chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .to_lowercase()
}

impl<G: Game + 'static> GameObj for G {
    fn render(&self) -> String {
        Game::render(self)
    }
    fn to_move(&self) -> Player {
        Game::to_move(self)
    }
    fn legal_moves(&self) -> Vec<String> {
        Game::legal_moves(self)
            .iter()
            .map(|m| self.format_move(m))
            .collect()
    }
    fn play(&mut self, mv: &str) -> Result<String, String> {
        if Game::outcome(self).is_some() {
            return Err("jocul s-a terminat".into());
        }
        let want = norm(mv);
        for m in Game::legal_moves(self) {
            let text = self.format_move(&m);
            if norm(&text) == want {
                self.apply(&m);
                return Ok(text);
            }
        }
        Err(format!("mutare ilegală sau necunoscută: «{}»", mv.trim()))
    }
    fn outcome(&self) -> Option<Outcome> {
        Game::outcome(self)
    }
    fn ai_move(&self, seed: u64) -> Option<String> {
        ai::best_move(self, seed).map(|m| self.format_move(&m))
    }
    fn set_viewer(&mut self, who: Option<Player>) {
        Game::set_viewer(self, who)
    }
    fn clone_box(&self) -> Box<dyn GameObj> {
        Box::new(self.clone())
    }
}

/// Helper shared by board games: "a1" style square names.
pub fn square_name(file: usize, rank: usize) -> String {
    format!("{}{}", (b'a' + file as u8) as char, rank + 1)
}
