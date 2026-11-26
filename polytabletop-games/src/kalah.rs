//! Kalah (SUA, 6 gropi × 4 semințe): același motor ca `mancala` (Kalah 6/4), expus sub propriul id.
//!
//! Reguli: semănat în sens antiorar, se sare peste magazia adversarului; ultima sămânță în
//! magazia proprie = încă o mutare; ultima într-o groapă proprie goală cu groapa opusă
//! nevidă capturează opusa + sămânța; când o parte rămâne goală, cealaltă își bagă restul
//! în magazie. Câștigă cine are mai multe semințe în magazie.

pub use crate::mancala::Mancala as Kalah;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q4_util::*;
    use polytabletop_core::{GameObj, Player};

    #[test]
    fn kalah_random_games_finish() {
        random_finishes(|_| Box::new(Kalah::new()), 6, 500);
    }

    #[test]
    fn kalah_has_six_opening_moves_and_ai_is_legal() {
        let g = Kalah::new();
        assert_eq!(GameObj::legal_moves(&g).len(), 6);
        ai_vs_random(|_| Box::new(Kalah::new()), 3, Player::One, 500);
        ai_vs_random(|_| Box::new(Kalah::new()), 4, Player::Two, 500);
    }
}
