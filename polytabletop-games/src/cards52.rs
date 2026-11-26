//! Shared helpers for the card games (hidden-information games).
//!
//! Hidden information in PolyTabletop: the `Game` trait is fully typed and clonable,
//! so a card game keeps the shuffled deck and both hands inside its state
//! (with its own seeded [`Rng`]). Two rules keep the AI honest:
//!
//! * `render()` shows only Player 1's hand and the *number* of the opponent's
//!   cards (everything that is public is shown to both sides);
//! * `evaluate()` reads only what the evaluated player can see, and chance
//!   events (draws, deals) that the AI would trigger by its own move are
//!   either deferred to a separate forced move or ignored by `evaluate()`.

use polytabletop_core::rng::Rng;

pub const SUIT_LETTERS: [char; 4] = ['c', 'd', 'h', 's'];
pub const SUIT_SYMBOLS: [&str; 4] = ["♣", "♦", "♥", "♠"];

/// Standard 52-card deck: card = suit * 13 + rank, rank 0 = "2" … 12 = "A".
pub const RANK_LABELS: [&str; 13] = [
    "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
];

/// In-place Fisher–Yates shuffle.
pub fn shuffle<T>(v: &mut [T], rng: &mut Rng) {
    for i in (1..v.len()).rev() {
        let j = rng.below(i + 1);
        v.swap(i, j);
    }
}

pub fn std_deck(rng: &mut Rng) -> Vec<u8> {
    let mut d: Vec<u8> = (0..52).collect();
    shuffle(&mut d, rng);
    d
}

pub fn std_rank(c: u8) -> u8 {
    c % 13
}
pub fn std_suit(c: u8) -> u8 {
    c / 13
}

/// "Ah", "10c" — the text form used for typed moves.
pub fn std_text(c: u8) -> String {
    format!(
        "{}{}",
        RANK_LABELS[std_rank(c) as usize],
        SUIT_LETTERS[std_suit(c) as usize]
    )
}

/// "A♥" — display form.
pub fn std_symbol(c: u8) -> String {
    format!(
        "{}{}",
        RANK_LABELS[std_rank(c) as usize],
        SUIT_SYMBOLS[std_suit(c) as usize]
    )
}

pub fn join_symbols(cards: &[u8], f: impl Fn(u8) -> String) -> String {
    if cards.is_empty() {
        return "—".into();
    }
    cards.iter().map(|&c| f(c)).collect::<Vec<_>>().join(" ")
}
