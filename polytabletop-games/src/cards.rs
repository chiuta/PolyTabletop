//! Shared helpers for the hidden-information games (Septica, Macao, Domino).
//!
//! # Hidden information, in one place
//!
//! `polytabletop-core` is a perfect-information framework: `Game::render` and
//! `Game::evaluate` get only `&self`. The hidden-information games therefore
//! keep the shuffled stock and *both* hands inside their state (shuffled with
//! their own [`Rng`], seeded at creation) and protect secrecy as follows.
//!
//! * **Rendering** – a [`Viewer`] decides whose hand is printed. The default is
//!   `Viewer::Fixed(Player::One)`: the human (Player 1, the CLI default) sees
//!   only their own hand plus the *number* of the opponent's cards, even when
//!   the AI is to move. `Viewer::HotSeat` (two humans on one screen) prints the
//!   hand of the player to move only, with a hand-over banner whenever the move
//!   changes sides. When the game is over both hands are revealed.
//! * **AI** – `ai_style` is `AlphaBeta(1)`, i.e. a greedy one-ply choice, and
//!   `evaluate(p)` reads only what `p` could see: public tables/scores, `p`'s own
//!   hand, and *counts* of the opponent's cards. It never reads the opponent's
//!   hand or the order/content of the stock. Cards that a simulated move makes
//!   `p` draw (the `fresh` counters) are excluded from composition-based terms,
//!   otherwise the search would "peek" at the top of the stock. Limitation: the
//!   AI cannot reason about what the opponent holds (no determinization/MCTS is
//!   possible through the `Game` trait), so it is a decent but simple player.

use polytabletop_core::rng::Rng;
use polytabletop_core::Player;

/// A playing card. `rank` is 2..=10, 11 = J, 12 = Q, 13 = K, 14 = A.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Card {
    pub rank: u8,
    /// 0 = h (inimă roșie), 1 = d (romb), 2 = s (inimă neagră), 3 = c (treflă).
    pub suit: u8,
}

pub const SUIT_LETTERS: [char; 4] = ['h', 'd', 's', 'c'];
pub const SUIT_SYMBOLS: [&str; 4] = ["♥", "♦", "♠", "♣"];

impl Card {
    pub fn rank_label(self) -> &'static str {
        match self.rank {
            2 => "2",
            3 => "3",
            4 => "4",
            5 => "5",
            6 => "6",
            7 => "7",
            8 => "8",
            9 => "9",
            10 => "10",
            11 => "J",
            12 => "Q",
            13 => "K",
            _ => "A",
        }
    }
    pub fn suit_letter(self) -> char {
        SUIT_LETTERS[self.suit as usize]
    }
    /// Canonical typed text, e.g. `10h` (ten of hearts), `As` (ace of spades).
    pub fn text(self) -> String {
        format!("{}{}", self.rank_label(), self.suit_letter())
    }
}

/// All cards with `rank >= min_rank` (32-card deck: 7; 52-card deck: 2).
pub fn full_deck(min_rank: u8) -> Vec<Card> {
    let mut v = Vec::new();
    for suit in 0..4u8 {
        for rank in min_rank..=14u8 {
            v.push(Card { rank, suit });
        }
    }
    v
}

/// Fisher–Yates shuffle.
pub fn shuffle<T>(v: &mut [T], rng: &mut Rng) {
    for i in (1..v.len()).rev() {
        let j = rng.below(i + 1);
        v.swap(i, j);
    }
}

pub fn join_cards(cards: &[Card]) -> String {
    if cards.is_empty() {
        return "—".into();
    }
    cards.iter().map(|c| c.text()).collect::<Vec<_>>().join(" ")
}

/// Whose hand `render` shows.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Viewer {
    /// Always this player's hand (default: Player One, the human against the AI).
    Fixed(Player),
    /// The hand of the player to move (two humans sharing a screen).
    HotSeat,
}

impl Viewer {
    pub fn who(self, to_move: Player) -> Player {
        match self {
            Viewer::Fixed(p) => p,
            Viewer::HotSeat => to_move,
        }
    }
}

/// Hot-seat banner shown when the move has just changed sides.
pub fn handover_banner(next: Player) -> String {
    format!(
        "════════ Predă ecranul: urmează Jucătorul {} (celălalt să nu se uite) ════════\n",
        next.idx() + 1
    )
}

pub fn pl(p: Player) -> usize {
    p.idx() + 1
}
