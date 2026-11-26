//! Crag (SUA): 3 zaruri, 13 categorii, 13 ture pe jucător, câștigă cine are mai multe puncte.
//!
//! * Pe tură arunci toate zarurile și poți rearunca oricâte (a doua aruncare e cea finală);
//!   apoi alegi o categorie liberă. Dacă zarurile nu o satisfac, primește 0.
//! * `unu`…`sase`: suma zarurilor cu acea valoare. `trei-la-fel` = 25. `treisprezece` (suma 13) = 26.
//!   `scara-mica` (1-2-3), `scara-mare` (4-5-6), `scara-impara` (1-3-5), `scara-para` (2-4-6) = 20 fiecare.
//!   `crag` = pereche plus un al treilea zar cu suma totală 13 = 50.
//!   Mutări: `arunc`, `tine 6 6`, `tine -`, `scor crag`.

use crate::q5_dice::{counts, Sheet, SheetSpec};

#[derive(Clone)]
pub struct CragSpec;

impl SheetSpec for CragSpec {
    const TITLE: &'static str = "Crag";
    const NDICE: usize = 3;
    const MAX_ROLLS: u8 = 2;
    const CATS: &'static [&'static str] = &[
        "unu",
        "doi",
        "trei",
        "patru",
        "cinci",
        "sase",
        "trei-la-fel",
        "treisprezece",
        "scara-mica",
        "scara-mare",
        "scara-impara",
        "scara-para",
        "crag",
    ];
    const PAR: &'static [f32] = &[
        0.9, 1.8, 2.8, 3.7, 4.6, 5.5, 4.0, 7.8, 3.0, 3.0, 3.0, 3.0, 4.0,
    ];

    fn score(cat: usize, dice: &[u8]) -> i32 {
        let c = counts(dice);
        let sum: i32 = dice.iter().map(|&d| d as i32).sum();
        let set = |a: usize, b: usize, d: usize| c[a] == 1 && c[b] == 1 && c[d] == 1;
        match cat {
            0..=5 => c[cat + 1] as i32 * (cat as i32 + 1),
            6 => {
                if c[1..].contains(&3) {
                    25
                } else {
                    0
                }
            }
            7 => {
                if sum == 13 {
                    26
                } else {
                    0
                }
            }
            8 => i32::from(set(1, 2, 3)) * 20,
            9 => i32::from(set(4, 5, 6)) * 20,
            10 => i32::from(set(1, 3, 5)) * 20,
            11 => i32::from(set(2, 4, 6)) * 20,
            _ => {
                if sum == 13 && c[1..].contains(&2) {
                    50
                } else {
                    0
                }
            }
        }
    }
}

pub type Crag = Sheet<CragSpec>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q5_util::testkit::check_game;

    #[test]
    fn scoring() {
        assert_eq!(CragSpec::score(5, &[6, 6, 2]), 12);
        assert_eq!(CragSpec::score(6, &[4, 4, 4]), 25);
        assert_eq!(CragSpec::score(7, &[6, 4, 3]), 26);
        assert_eq!(CragSpec::score(7, &[6, 4, 4]), 0);
        assert_eq!(CragSpec::score(8, &[3, 1, 2]), 20);
        assert_eq!(CragSpec::score(10, &[5, 3, 1]), 20);
        assert_eq!(CragSpec::score(11, &[2, 4, 6]), 20);
        assert_eq!(CragSpec::score(12, &[5, 5, 3]), 50);
        assert_eq!(CragSpec::score(12, &[6, 6, 1]), 50);
        assert_eq!(CragSpec::score(12, &[6, 6, 2]), 0);
        assert_eq!(CragSpec::score(12, &[6, 4, 3]), 0);
    }

    #[test]
    fn evaluate_does_not_depend_on_future_dice() {
        crate::q5_dice::tests_support::evaluate_ignores_future_dice::<CragSpec>();
    }

    #[test]
    fn random_and_ai_games() {
        check_game(Crag::with_seed, 6, 500);
    }
}
