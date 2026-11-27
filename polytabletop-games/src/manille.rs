//! Manille (Franța) — adaptare pentru doi jucători, pachet de 32 de cărți (7–A, culorile h/d/s/c).
//!
//! * Fiecare primește 8 cărți; următoarea carte din pachet se întoarce și dă atuul. 4 mâini.
//! * Ordinea în fiecare culoare: 10 (manille) > A (manillon) > K > Q > J > 9 > 8 > 7. Puncte: 10 = 5,
//!   A = 4, K = 3, Q = 2, J = 1, restul 0 (60 pe mână) + 10 pentru ultima levată („dix de der”).
//! * Obligații: răspunzi la culoare și, dacă poți, **mai sus** decât cartea cerută; dacă nu ai
//!   culoarea, trebuie să tai cu atu dacă ai; altfel arunci orice.
//! * „Manille muette” (fără anunțuri); fără bonusuri de licitație.
//!   Mutări: `10h`, `As`, `7c`.

use crate::q5_trick::{Trick, TrickSpec};

#[derive(Clone)]
pub struct ManilleSpec;

impl TrickSpec for ManilleSpec {
    const TITLE: &'static str = "Manille";
    // ordinea identității = ordinea puterii: 7 8 9 J Q K A 10
    const LABELS: &'static [&'static str] = &["7", "8", "9", "J", "Q", "K", "A", "10"];
    const HAND: usize = 8;
    const DEALS: u8 = 4;
    const CHOSEN: bool = false;
    const MUST_TRUMP: bool = true;
    const OVER_SUIT: bool = true;
    const OVER_TRUMP: bool = true;
    const LAST_TRICK: i32 = 10;
    const PIT_BONUS: i32 = 0;
    const BAUER_EXEMPT: bool = false;

    fn strength(rank: usize, _trump: bool) -> i32 {
        rank as i32
    }

    fn points(rank: usize, _trump: bool) -> i32 {
        match rank {
            7 => 5,
            6 => 4,
            5 => 3,
            4 => 2,
            3 => 1,
            _ => 0,
        }
    }
}

pub type Manille = Trick<ManilleSpec>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q5_trick::tests_support::*;
    use crate::q5_util::testkit::check_game;

    #[test]
    fn points_per_suit_are_15() {
        let s: i32 = (0..8).map(|r| ManilleSpec::points(r, false)).sum();
        assert_eq!(s, 15);
    }

    #[test]
    fn engine_properties() {
        ai_is_blind::<ManilleSpec>();
        viewer_hides_the_other_hand::<ManilleSpec>();
        play_all_cards_conserves_points::<ManilleSpec>();
    }

    #[test]
    fn random_and_ai_games() {
        check_game(Manille::with_seed, 3, 600);
    }
}

#[cfg(test)]
mod a3_audit {
    use super::*;

    #[test]
    fn ai_ignores_hidden_cards_at_every_level() {
        crate::p3_comun::ai_blind_check(
            "manille",
            Manille::with_seed,
            crate::q5_trick::scramble_hidden,
            10,
        );
    }
}
