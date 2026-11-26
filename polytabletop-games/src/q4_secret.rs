//! Ajutoare comune pentru jocurile de tip „secret și ghicit” (Q4): zgomot determinist pentru alegerile AI,
//! rezultat din puncte și ce jucător vede secretul.

use polytabletop_core::{Outcome, Player};

/// Zgomot determinist (0..m) din semințele de stare `noise` și alegerea `x` (nu depinde de informații ascunse).
pub fn noise(noise: u64, x: u64, m: u64) -> i32 {
    let h = (noise ^ x.wrapping_mul(0x9E37_79B9_7F4A_7C15)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    ((h >> 33) % m) as i32
}

pub fn by_points(a: i32, b: i32) -> Outcome {
    match a.cmp(&b) {
        std::cmp::Ordering::Greater => Outcome::Win(Player::One),
        std::cmp::Ordering::Less => Outcome::Win(Player::Two),
        _ => Outcome::Draw,
    }
}

pub fn pl(i: usize) -> Player {
    if i == 0 {
        Player::One
    } else {
        Player::Two
    }
}

/// Jucătorul al cărui secret se arată: `viewer` sau, în hot-seat, cel la mutare.
pub fn viewer_of(viewer: Option<Player>, to_move: Player) -> usize {
    viewer.unwrap_or(to_move).idx()
}
