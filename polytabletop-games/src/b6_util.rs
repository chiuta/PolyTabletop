//! Ajutoare comune pentru jocurile B6.
use polytabletop_core::Player;

pub fn pl(i: usize) -> Player {
    if i == 0 {
        Player::One
    } else {
        Player::Two
    }
}
