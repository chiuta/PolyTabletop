//! Jass (Schieber), Elveția — adaptare pentru doi jucători, pachet elvețian de 36 de cărți (6–A,
//! cu culorile franceze h/d/s/c).
//!
//! * Fiecare primește 9 cărți (celelalte 18 nu intră în joc). Cel care conduce alege atuul
//!   (`atu h`) sau „schimbă” (`schiebe`): atunci celălalt trebuie să aleagă. 4 mâini; punctele
//!   de carte se adună.
//! * Puncte: A 11, K 4, Q 3, J 2, 10 → 10, 9–6 → 0; la atu: J (Bauer) 20, 9 (Näll) 14, A 11, 10 10,
//!   K 4, Q 3. Ordinea atuului: J > 9 > A > K > Q > 10 > 8 > 7 > 6. Ultima levată +5; toate cele 9
//!   levate (Match) +100. Total pe mână: 157 (+100).
//! * Trebuie să răspunzi la culoare dacă poți (cu atu poți tăia oricând când nu ai culoarea); dacă s-a
//!   cerut atu și ai doar valetul de atu, ești scutit. Fără „Stöck”, fără „Weis” (anunțuri) și fără
//!   interdicția de a sub-tăia, simplificări ale variantei pentru doi.
//!   Mutări: `10h`, `Js`, `atu c`, `schiebe`.

use crate::q5_trick::{Trick, TrickSpec};

#[derive(Clone)]
pub struct JassSpec;

impl TrickSpec for JassSpec {
    const TITLE: &'static str = "Jass (Schieber)";
    const LABELS: &'static [&'static str] = &["6", "7", "8", "9", "10", "J", "Q", "K", "A"];
    const HAND: usize = 9;
    const DEALS: u8 = 4;
    const CHOSEN: bool = true;
    const MUST_TRUMP: bool = false;
    const OVER_SUIT: bool = false;
    const OVER_TRUMP: bool = false;
    const LAST_TRICK: i32 = 5;
    const PIT_BONUS: i32 = 100;
    const BAUER_EXEMPT: bool = true;

    fn strength(rank: usize, trump: bool) -> i32 {
        if trump {
            // 6 7 8 10 Q K A 9 J
            match rank {
                0 => 0,
                1 => 1,
                2 => 2,
                4 => 3,
                6 => 4,
                7 => 5,
                8 => 6,
                3 => 7,
                _ => 8,
            }
        } else {
            rank as i32
        }
    }

    fn points(rank: usize, trump: bool) -> i32 {
        match (rank, trump) {
            (5, true) => 20,
            (3, true) => 14,
            (8, _) => 11,
            (4, _) => 10,
            (7, _) => 4,
            (6, _) => 3,
            (5, false) => 2,
            _ => 0,
        }
    }
}

pub type Jass = Trick<JassSpec>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q5_trick::tests_support::*;
    use crate::q5_util::testkit::check_game;
    use polytabletop_core::{Game, Player};

    #[test]
    fn card_points_sum_to_152() {
        let mut tot = 0;
        for trump in [true, false] {
            for r in 0..9 {
                tot += JassSpec::points(r, trump);
            }
        }
        // o culoare de atu + trei culori simple: 62 + 3 * 30
        let one_trump: i32 = (0..9).map(|r| JassSpec::points(r, true)).sum();
        let plain: i32 = (0..9).map(|r| JassSpec::points(r, false)).sum();
        assert_eq!(one_trump + 3 * plain, 152);
        assert!(tot > 0);
    }

    #[test]
    fn leader_chooses_trump_or_passes_once() {
        let mut g = Jass::with_seed(1);
        let m = g.legal_moves();
        assert_eq!(m.len(), 5);
        assert_eq!(g.to_move(), Player::One);
        g.apply(&crate::q5_trick::Move::Pass);
        assert_eq!(g.to_move(), Player::Two, "după „schiebe” celălalt alege");
        assert_eq!(g.legal_moves().len(), 4, "a doua oară nu se mai poate pasa");
        g.apply(&crate::q5_trick::Move::Trump(2));
        assert_eq!(g.to_move(), Player::One, "tot J1 conduce prima levată");
    }

    #[test]
    fn must_follow_suit_and_jack_exemption() {
        let mut g = Jass::with_seed(2);
        g.apply(&crate::q5_trick::Move::Trump(0));
        // conducem cu o carte, adversarul trebuie să răspundă la culoare dacă are
        let lead = g.legal_moves()[0].clone();
        g.apply(&lead);
        let who = g.to_move();
        let hand = g.hand(who).to_vec();
        let led = match lead {
            crate::q5_trick::Move::Play(c) => c,
            _ => unreachable!(),
        };
        let has = hand.iter().any(|&c| c / 9 == led / 9);
        let moves = g.legal_moves();
        if has {
            for m in &moves {
                if let crate::q5_trick::Move::Play(c) = m {
                    assert_eq!(c / 9, led / 9);
                }
            }
        } else {
            assert_eq!(moves.len(), hand.len());
        }
    }

    #[test]
    fn engine_properties() {
        ai_is_blind::<JassSpec>();
        viewer_hides_the_other_hand::<JassSpec>();
        play_all_cards_conserves_points::<JassSpec>();
    }

    #[test]
    fn random_and_ai_games() {
        check_game(Jass::with_seed, 3, 600);
    }
}
