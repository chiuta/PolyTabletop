//! Piese comune pentru jocurile cu baraja spaniolă de 40 de cărți (Escoba, Cuarenta, Chinchón).
//!
//! Cartea e un `u8` din `0..40`: `suit = c / 10`, `pos = c % 10`. Culori: `o` = oros (monede),
//! `c` = copas (cupe), `e` = espadas (săbii), `b` = bastos (bastoane). Pozițiile 0..=6 sunt
//! cărțile 1–7, iar 7, 8, 9 sunt sota (10), caballo (11) și rey (12), în această ordine
//! (8 și 9 lipsesc din pachet). Informația ascunsă (mâini, pachet) rămâne în starea
//! jocului, ca la celelalte jocuri de cărți: vezi `cards.rs`.

use crate::cards52::shuffle;
use polytabletop_core::rng::Rng;

pub const SUIT_LETTERS: [char; 4] = ['o', 'c', 'e', 'b'];
pub const LABELS: [&str; 10] = ["1", "2", "3", "4", "5", "6", "7", "10", "11", "12"];

pub fn suit(c: u8) -> u8 {
    c / 10
}
pub fn pos(c: u8) -> u8 {
    c % 10
}
/// Textul cărții, ex. `7o`, `12b`.
pub fn text(c: u8) -> String {
    format!(
        "{}{}",
        LABELS[pos(c) as usize],
        SUIT_LETTERS[suit(c) as usize]
    )
}
pub fn join(cards: &[u8]) -> String {
    if cards.is_empty() {
        return "—".into();
    }
    cards.iter().map(|&c| text(c)).collect::<Vec<_>>().join(" ")
}
pub fn sorted(mut v: Vec<u8>) -> Vec<u8> {
    v.sort_unstable();
    v
}
/// Pachetul amestecat.
pub fn deck(rng: &mut Rng) -> Vec<u8> {
    let mut d: Vec<u8> = (0..40).collect();
    shuffle(&mut d, rng);
    d
}
