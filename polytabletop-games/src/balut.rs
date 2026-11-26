//! Balut (Filipine): 5 zaruri, până la 3 aruncări pe tură, 7 combinații care se completează o dată fiecare.
//!
//! * `patru`, `cinci`, `sase`: 4/5/6 puncte pentru fiecare zar cu valoarea respectivă.
//! * `scara`: 1-2-3-4-5 sau 2-3-4-5-6 = 50. `casa`: trei la fel + o pereche = suma celor cinci zaruri.
//! * `sansa`: suma celor cinci zaruri. `balut`: cinci la fel = 30.
//! * Dacă nu se potrivește nicio combinație liberă, trebuie să „tai” una (o scrii cu 0).
//! * Joc de 7 ture pe jucător; câștigă cine are mai multe puncte. Fără pariul pe numărul de balut-uri.
//!   Mutări: `arunc`, `tine 6 6`, `tine -`, `scor sase`.

use crate::q5_dice::{counts, Sheet, SheetSpec};

#[derive(Clone)]
pub struct BalutSpec;

impl SheetSpec for BalutSpec {
    const TITLE: &'static str = "Balut";
    const NDICE: usize = 5;
    const MAX_ROLLS: u8 = 3;
    const CATS: &'static [&'static str] =
        &["patru", "cinci", "sase", "scara", "casa", "sansa", "balut"];
    const PAR: &'static [f32] = &[8.0, 10.0, 12.0, 8.0, 7.5, 21.0, 2.5];

    fn score(cat: usize, dice: &[u8]) -> i32 {
        let c = counts(dice);
        let sum: i32 = dice.iter().map(|&d| d as i32).sum();
        let has = |a: usize, n: usize| (a..a + n).all(|v| c[v] > 0);
        match cat {
            0 => c[4] as i32 * 4,
            1 => c[5] as i32 * 5,
            2 => c[6] as i32 * 6,
            3 => {
                if has(1, 5) || has(2, 5) {
                    50
                } else {
                    0
                }
            }
            4 => {
                if c[1..].contains(&3) && c[1..].contains(&2) {
                    sum
                } else {
                    0
                }
            }
            5 => sum,
            _ => {
                if c[1..].contains(&5) {
                    30
                } else {
                    0
                }
            }
        }
    }
}

pub type Balut = Sheet<BalutSpec>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q5_util::testkit::check_game;

    #[test]
    fn scoring() {
        assert_eq!(BalutSpec::score(0, &[4, 4, 1, 2, 4]), 12);
        assert_eq!(BalutSpec::score(3, &[2, 3, 4, 5, 6]), 50);
        assert_eq!(BalutSpec::score(3, &[1, 2, 3, 4, 6]), 0);
        assert_eq!(BalutSpec::score(4, &[2, 2, 5, 5, 5]), 19);
        assert_eq!(BalutSpec::score(4, &[5, 5, 5, 5, 5]), 0);
        assert_eq!(BalutSpec::score(5, &[1, 2, 3, 4, 6]), 16);
        assert_eq!(BalutSpec::score(6, &[3, 3, 3, 3, 3]), 30);
    }

    #[test]
    fn evaluate_does_not_depend_on_future_dice() {
        crate::q5_dice::tests_support::evaluate_ignores_future_dice::<BalutSpec>();
    }

    #[test]
    fn ai_takes_a_balut() {
        let mut g = Balut::with_seed(1);
        g.set_dice_for_test(&[6, 6, 6, 6, 6], 1);
        let best = polytabletop_core::ai::best_move(&g, 3).unwrap();
        assert_eq!(best, crate::q5_dice::Move::Score(6));
    }

    #[test]
    fn random_and_ai_games() {
        check_game(Balut::with_seed, 6, 500);
    }
}
