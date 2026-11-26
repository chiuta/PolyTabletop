//! Tarock / Tarok (Austria, Slovenia, Ungaria), adaptat pentru 2 jucători: levate fără licitație.
//!
//! * 54 de cărți: 22 de **tarocuri** (I = Pagat … XXI = Mond, plus **Škis**) și 4 culori × 8 (negre ♠♣:
//!   7 8 9 10 J C Q K; roșii ♦♥: 4 3 2 1 J C Q K, de la mică la mare). Fiecare primește 18 cărți, 18 rămân
//!   nefolosite și ascunse.
//! * Respecți culoarea condusă; dacă n-o ai, ești obligat să joci un tarok dacă ai. Un tarok bate orice
//!   culoare; între tarocuri câștigă numărul mai mare.
//! * **Škis** (`skis`) se poate juca oricând, nu câștigă niciodată și rămâne în punctele celui care l-a jucat.
//! * Puncte: K 5, Q 4, C 3, J 2, restul culorilor 1; tarocurile I, XXI și Škis 5, celelalte tarocuri 1.
//!   **Pagat ultimo**: cine câștigă ultima levată cu tarocul I primește 10 puncte în plus.
//! * Cine are mai multe puncte câștigă mâna; meciul: primul la 2 mâini (maximum 5 mâini).
//!
//! Mutări: `t7` (tarocul VII), `skis`, `Kh`, `10s`, `1d`; `imparte` între mâini.

use crate::q2_trick::TrickRules;
use polytabletop_core::Player;

#[derive(Clone)]
pub struct TarockRules;

const SUITS: [char; 4] = ['d', 'h', 's', 'c'];
const BLACK: [&str; 8] = ["7", "8", "9", "10", "J", "C", "Q", "K"];
const RED: [&str; 8] = ["4", "3", "2", "1", "J", "C", "Q", "K"];
const SKIS: u8 = 21;

fn suit_rank(id: u8) -> (usize, usize) {
    let i = (id - 22) as usize;
    (i / 8, i % 8)
}

impl TrickRules for TarockRules {
    const NAME: &'static str = "Tarock (2 jucători)";
    const HAND: usize = 18;
    const HAND_WINS: i32 = 2;

    fn deck() -> Vec<u8> {
        (0..54).collect()
    }

    fn label(id: u8) -> String {
        if id == SKIS {
            "skis".into()
        } else if id < 21 {
            format!("t{}", id + 1)
        } else {
            let (s, r) = suit_rank(id);
            let names = if s >= 2 { &BLACK } else { &RED };
            format!("{}{}", names[r], SUITS[s])
        }
    }

    fn group(id: u8) -> u8 {
        if id <= SKIS {
            4
        } else {
            suit_rank(id).0 as u8
        }
    }

    fn trump_group() -> u8 {
        4
    }

    fn rank(id: u8) -> i32 {
        if id <= SKIS {
            id as i32 + 1
        } else {
            suit_rank(id).1 as i32
        }
    }

    fn points(id: u8) -> i32 {
        if id == 0 || id == 20 || id == SKIS {
            return 5;
        }
        if id < 21 {
            return 1;
        }
        match suit_rank(id).1 {
            7 => 5,
            6 => 4,
            5 => 3,
            4 => 2,
            _ => 1,
        }
    }

    fn power(id: u8) -> i32 {
        if id == SKIS {
            return 0;
        }
        if id < 21 {
            return 2 + (id as i32) / 3;
        }
        (suit_rank(id).1 as i32) / 2
    }

    fn free_play(id: u8) -> bool {
        id == SKIS
    }

    fn never_wins(id: u8) -> bool {
        id == SKIS
    }

    fn must_trump() -> bool {
        true
    }

    fn keeps_owner(id: u8) -> bool {
        id == SKIS
    }

    fn hand_winner(pts: [i32; 2], last: Option<(Player, u8)>) -> Option<Player> {
        let mut p = pts;
        if let Some((w, 0)) = last {
            p[w.idx()] += 10;
        }
        match p[0].cmp(&p[1]) {
            std::cmp::Ordering::Greater => Some(Player::One),
            std::cmp::Ordering::Less => Some(Player::Two),
            std::cmp::Ordering::Equal => None,
        }
    }
}

crate::trick_game!(Tarock, TarockRules);

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;
    use polytabletop_core::Game;

    #[test]
    fn deck_and_labels_are_unique_and_points_total() {
        let mut labels: Vec<String> = TarockRules::deck()
            .iter()
            .map(|&c| TarockRules::label(c))
            .collect();
        labels.sort();
        labels.dedup();
        assert_eq!(labels.len(), 54);
        let total: i32 = TarockRules::deck()
            .iter()
            .map(|&c| TarockRules::points(c))
            .sum();
        // 3 × 5 + 19 × 1 (tarocuri) + 4 × (5+4+3+2+4) = 15 + 19 + 72
        assert_eq!(total, 106);
    }

    #[test]
    fn skis_never_wins_and_stays_with_its_player() {
        let mut g = Tarock::with_seed(2);
        g.0 = {
            let mut t = g.0.clone();
            t.hands_set_for_test([vec![SKIS], vec![22]], [vec![], vec![]]);
            t
        };
        g.apply(&crate::q2_trick::TMove::Play(SKIS));
        g.apply(&crate::q2_trick::TMove::Play(22));
        assert_eq!(g.0.eyes().0, 5);
        assert_eq!(g.0.eyes().1, 1);
    }

    #[test]
    fn random_games_finish() {
        for seed in 1..=3u64 {
            let mut g: Box<dyn polytabletop_core::GameObj> = Box::new(Tarock::with_seed(seed));
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.play(&m[rng.below(m.len())]).unwrap();
                n += 1;
                assert!(n < 300);
            }
        }
    }
}
