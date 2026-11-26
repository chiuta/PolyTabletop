//! Klaverjas (Olanda), varianta Amsterdam — adaptare pentru doi jucători, pachet de 32 de cărți (7–A).
//!
//! * Fiecare primește 8 cărți; următoarea carte se întoarce și dă atuul. 4 mâini; punctele se adună.
//! * Ordine în culorile simple: A > 10 > K > Q > J > 9 > 8 > 7 (A 11, 10 10, K 4, Q 3, J 2); la atu:
//!   J (20) > 9 (14) > A (11) > 10 (10) > K (4) > Q (3) > 8 > 7. Ultima levată +10; toate levatele
//!   („pit”) +100. Total 152 + 10 pe mână.
//! * Obligații: răspunzi la culoare; dacă s-a cerut atu, trebuie să bați atuul dacă poți; fără culoare
//!   trebuie să tai cu atu dacă ai, altfel arunci orice.
//! * Roem (anunțurile de secvențe și „stuk”) lipsește: cere trei sau mai multe cărți pe levată, iar aici
//!   levatele au două. Jucătorii de la 4 pot juca cu regulile complete.
//!   Mutări: `Jh`, `10s`, `Ac`.

use crate::q5_trick::{Trick, TrickSpec};

#[derive(Clone)]
pub struct KlaverjasSpec;

impl TrickSpec for KlaverjasSpec {
    const TITLE: &'static str = "Klaverjas";
    const LABELS: &'static [&'static str] = &["7", "8", "9", "10", "J", "Q", "K", "A"];
    const HAND: usize = 8;
    const DEALS: u8 = 4;
    const CHOSEN: bool = false;
    const MUST_TRUMP: bool = true;
    const OVER_SUIT: bool = false;
    const OVER_TRUMP: bool = true;
    const LAST_TRICK: i32 = 10;
    const PIT_BONUS: i32 = 100;
    const BAUER_EXEMPT: bool = false;

    fn strength(rank: usize, trump: bool) -> i32 {
        if trump {
            // 7 8 Q K 10 A 9 J
            match rank {
                0 => 0,
                1 => 1,
                5 => 2,
                6 => 3,
                3 => 4,
                7 => 5,
                2 => 6,
                _ => 7,
            }
        } else {
            // 7 8 9 J Q K 10 A
            match rank {
                0 => 0,
                1 => 1,
                2 => 2,
                4 => 3,
                5 => 4,
                6 => 5,
                3 => 6,
                _ => 7,
            }
        }
    }

    fn points(rank: usize, trump: bool) -> i32 {
        match (rank, trump) {
            (4, true) => 20,
            (2, true) => 14,
            (7, _) => 11,
            (3, _) => 10,
            (6, _) => 4,
            (5, _) => 3,
            (4, false) => 2,
            _ => 0,
        }
    }
}

pub type Klaverjas = Trick<KlaverjasSpec>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q5_trick::tests_support::*;
    use crate::q5_util::testkit::check_game;

    #[test]
    fn points_sum_to_152() {
        let t: i32 = (0..8).map(|r| KlaverjasSpec::points(r, true)).sum();
        let p: i32 = (0..8).map(|r| KlaverjasSpec::points(r, false)).sum();
        assert_eq!(t + 3 * p, 152);
    }

    #[test]
    fn engine_properties() {
        ai_is_blind::<KlaverjasSpec>();
        viewer_hides_the_other_hand::<KlaverjasSpec>();
        play_all_cards_conserves_points::<KlaverjasSpec>();
    }

    #[test]
    fn random_and_ai_games() {
        check_game(Klaverjas::with_seed, 3, 600);
    }
}
