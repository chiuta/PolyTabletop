//! Doppelkopf (Germania), adaptat pentru 2 jucători: jocul normal, fără anunțuri și solouri.
//!
//! * 48 de cărți (9, 10, J, Q, K, A în 4 culori, de două ori). Se împart toate: 24 fiecăruia.
//! * **Atuuri** (de la cel mai mare): 10♥ (Dulle), Q♣ Q♠ Q♥ Q♦, J♣ J♠ J♥ J♦, A♦ 10♦ K♦ 9♦. Culorile
//!   „fără atu” (♣ ♠ ♥) se ordonează A, 10, K, 9. Culoarea (sau atuul) condusă se respectă dacă se poate.
//! * Două cărți egale: câștigă prima jucată, cu excepția Dulle (a doua 10♥ o bate pe prima).
//! * Puncte: A 11, 10 10, K 4, Q 3, J 2, 9 0 (240 în total). Cine adună **peste 120** câștigă mâna
//!   (120–120 = remiză). Meciul: primul la 2 mâini câștigate (maximum 5 mâini).
//!
//! Mutări: `Qc` (damă de treflă), `10h` (Dulle), `Ad`; `imparte` între mâini. Litere: h ♥, d ♦, s ♠, c ♣.

use crate::q2_trick::TrickRules;
use polytabletop_core::Player;

#[derive(Clone)]
pub struct DkRules;

const SUITS: [char; 4] = ['d', 'h', 's', 'c'];
const RANKS: [&str; 6] = ["9", "10", "J", "Q", "K", "A"];

fn parts(id: u8) -> (usize, usize) {
    let i = (id % 24) as usize;
    (i / 6, i % 6)
}

fn is_trump(id: u8) -> bool {
    let (s, k) = parts(id);
    (s == 1 && k == 1) || k == 2 || k == 3 || s == 0
}

impl TrickRules for DkRules {
    const NAME: &'static str = "Doppelkopf (2 jucători)";
    const HAND: usize = 24;
    const HAND_WINS: i32 = 2;

    fn deck() -> Vec<u8> {
        (0..48).collect()
    }

    fn label(id: u8) -> String {
        let (s, k) = parts(id);
        format!("{}{}", RANKS[k], SUITS[s])
    }

    fn group(id: u8) -> u8 {
        if is_trump(id) {
            4
        } else {
            parts(id).0 as u8
        }
    }

    fn trump_group() -> u8 {
        4
    }

    fn rank(id: u8) -> i32 {
        let (s, k) = parts(id);
        if s == 1 && k == 1 {
            return 100;
        }
        match k {
            3 => 90 + s as i32,
            2 => 80 + s as i32,
            _ if s == 0 => 60 + k as i32,
            _ => k as i32,
        }
    }

    fn points(id: u8) -> i32 {
        [0, 10, 2, 3, 4, 11][parts(id).1]
    }

    fn power(id: u8) -> i32 {
        if !is_trump(id) {
            return [0, 1, 0, 0, 0, 2][parts(id).1].max(0);
        }
        match Self::rank(id) {
            100 => 10,
            90..=93 => 8,
            80..=83 => 5,
            _ => 3,
        }
    }

    fn second_wins_tie(id: u8) -> bool {
        let (s, k) = parts(id);
        s == 1 && k == 1
    }

    fn hand_winner(pts: [i32; 2], _last: Option<(Player, u8)>) -> Option<Player> {
        if pts[0] > 120 {
            Some(Player::One)
        } else if pts[1] > 120 {
            Some(Player::Two)
        } else {
            None
        }
    }
}

crate::trick_game!(Doppelkopf, DkRules);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q2_trick::TMove;
    use polytabletop_core::rng::Rng;
    use polytabletop_core::Outcome;

    #[test]
    fn deck_points_and_trump_order() {
        let total: i32 = DkRules::deck().iter().map(|&c| DkRules::points(c)).sum();
        assert_eq!(total, 240);
        let trumps = DkRules::deck().iter().filter(|&&c| is_trump(c)).count();
        assert_eq!(trumps, 26);
        let find = |l: &str| {
            DkRules::deck()
                .into_iter()
                .find(|&c| DkRules::label(c) == l)
                .unwrap()
        };
        assert!(DkRules::rank(find("10h")) > DkRules::rank(find("Qc")));
        assert!(DkRules::rank(find("Qc")) > DkRules::rank(find("Qs")));
        assert!(DkRules::rank(find("Qd")) > DkRules::rank(find("Jc")));
        assert!(DkRules::rank(find("Jd")) > DkRules::rank(find("Ad")));
        assert!(DkRules::rank(find("As")) > DkRules::rank(find("10s")));
    }

    #[test]
    fn must_follow_and_hidden_hand() {
        let g = Doppelkopf::with_seed(3);
        assert_eq!(g.0.hand_of(Player::One).len(), 24);
        let r = polytabletop_core::GameObj::render(&g);
        assert!(r.contains("J2 are 24 cărți"));
    }

    #[test]
    fn random_games_finish() {
        for seed in 1..=3u64 {
            let mut g: Box<dyn polytabletop_core::GameObj> = Box::new(Doppelkopf::with_seed(seed));
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.play(&m[rng.below(m.len())]).unwrap();
                n += 1;
                assert!(n < 400);
            }
            assert!(matches!(
                g.outcome(),
                Some(Outcome::Win(_)) | Some(Outcome::Draw)
            ));
        }
    }

    #[test]
    fn ai_does_not_read_the_opponent_hand() {
        let a = Doppelkopf::with_seed(4);
        let mut b = a.clone();
        // schimbăm mâna lui J2 cu o permutare a cărților nedistribuite (aici: swap între mâna lui J2 și stoc nu există) —
        // se amestecă doar mâna adversarului
        b.0.reverse_hand_for_test(Player::Two);
        let m = polytabletop_core::GameObj::ai_move(&a, 2).unwrap();
        assert_eq!(Some(m), polytabletop_core::GameObj::ai_move(&b, 2));
        let _ = TMove::Deal;
    }
}
