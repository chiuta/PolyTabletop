//! Sueca (Portugalia / Brazilia) — adaptare pentru doi jucători, pachet de 40 de cărți (fără 8, 9, 10).
//!
//! * Fiecare primește 10 cărți; următoarea carte se întoarce și dă atuul (celelalte 19 nu intră în joc).
//!   4 mâini; punctele de carte se adună și câștigă cine are mai multe.
//! * Ordinea: A > 7 > K > J > Q > 6 > 5 > 4 > 3 > 2. Puncte: A 11, 7 10, K 4, J 3, Q 2 (120 pe pachet
//!   complet de patru culori; într-o mână pentru doi se joacă doar 20 de cărți).
//! * Trebuie să răspunzi la culoare dacă poți; nu există obligația de a tăia sau de a bate. Dacă nu ai
//!   culoarea, arunci orice („renúncia” e doar la carte necesară).
//!   Mutări: `Ah`, `7s`, `Qc`.

use crate::q5_trick::{Trick, TrickSpec};

#[derive(Clone)]
pub struct SuecaSpec;

impl TrickSpec for SuecaSpec {
    const TITLE: &'static str = "Sueca";
    // ordinea identității = ordinea puterii: 2 3 4 5 6 Q J K 7 A
    const LABELS: &'static [&'static str] = &["2", "3", "4", "5", "6", "Q", "J", "K", "7", "A"];
    const HAND: usize = 10;
    const DEALS: u8 = 4;
    const CHOSEN: bool = false;
    const MUST_TRUMP: bool = false;
    const OVER_SUIT: bool = false;
    const OVER_TRUMP: bool = false;
    const LAST_TRICK: i32 = 0;
    const PIT_BONUS: i32 = 0;
    const BAUER_EXEMPT: bool = false;

    fn strength(rank: usize, _trump: bool) -> i32 {
        rank as i32
    }

    fn points(rank: usize, _trump: bool) -> i32 {
        match rank {
            9 => 11,
            8 => 10,
            7 => 4,
            6 => 3,
            5 => 2,
            _ => 0,
        }
    }
}

pub type Sueca = Trick<SuecaSpec>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q5_trick::tests_support::*;
    use crate::q5_util::testkit::check_game;

    #[test]
    fn points_per_suit_are_30() {
        let s: i32 = (0..10).map(|r| SuecaSpec::points(r, false)).sum();
        assert_eq!(s, 30);
    }

    #[test]
    fn engine_properties() {
        ai_is_blind::<SuecaSpec>();
        viewer_hides_the_other_hand::<SuecaSpec>();
        play_all_cards_conserves_points::<SuecaSpec>();
    }

    #[test]
    fn random_and_ai_games() {
        check_game(Sueca::with_seed, 3, 600);
    }
}
