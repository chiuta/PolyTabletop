//! Val 17 / Q6: verificări pentru jocurile adăugate de Q6 (jocuri aleatorii, AI legal și determinist).

use polytabletop_core::rng::Rng;
use polytabletop_core::GameObj;
use polytabletop_games::find;

const IDS: &[&str] = &[
    "hnefatafl",
    "brandubh",
    "pente",
    "havannah",
    "chinese_checkers",
    "goose",
    "keno",
    "sichuan_mahjong",
    "long_nard",
    "shove_ha_penny",
    "rithmomachia",
    "pong_hau_kiki",
    "patolli",
    "duodecim_scripta",
    "jigsaw",
    "guobiao",
    "boggle",
    "kings",
    "celebrity",
    "pin_the_tail",
    "war",
    "tressette",
    "piquet",
    "batak",
    "yaniv",
    "ascunselea",
    "de_a_prinselea",
    "elasticul",
    "oina",
    "loto_imagini",
    "bau_cua",
    "yacht",
    "chicago",
    "zara",
    "daldos",
];

/// Jocuri cu căutare limitată de ceas (rezultatul poate diferi între rulări).
const TIMED: &[&str] = &[
    "pente",
    "hnefatafl",
    "brandubh",
    "chinese_checkers",
    "rithmomachia",
];

fn random_to_end(mut g: Box<dyn GameObj>, seed: u64) -> usize {
    let mut rng = Rng::new(seed);
    let mut n = 0;
    while g.outcome().is_none() {
        let moves = g.legal_moves();
        assert!(!moves.is_empty(), "fără mutări legale\n{}", g.render());
        let mut s = moves.clone();
        s.sort();
        s.dedup();
        assert_eq!(s.len(), moves.len(), "mutări duplicate\n{}", g.render());
        let pick = moves[rng.below(moves.len())].clone();
        g.play(&pick).expect("mutare legală");
        n += 1;
        assert!(n < 4000, "nu se termină");
    }
    n
}

#[test]
fn q6_random_games_end() {
    for id in IDS {
        let p = find(id).unwrap_or_else(|| panic!("{id} neînregistrat"));
        for seed in 1..=3u64 {
            random_to_end((p.make)(), seed);
        }
    }
}

#[test]
fn q6_garbage_rejected_and_render_nonempty() {
    for id in IDS {
        let p = find(id).unwrap();
        let mut g = (p.make)();
        assert!(g.play("zzzz").is_err(), "{id}");
        assert!(g.play("").is_err(), "{id}");
        assert!(!g.render().is_empty(), "{id}");
    }
}

fn midgame(id: &str, seed: u64, plies: usize) -> Box<dyn GameObj> {
    let mut g = (find(id).unwrap().make)();
    let mut rng = Rng::new(seed);
    for _ in 0..plies {
        if g.outcome().is_some() {
            break;
        }
        let m = g.legal_moves();
        let pick = m[rng.below(m.len())].clone();
        g.play(&pick).unwrap();
    }
    g
}

#[test]
fn q6_ai_moves_are_legal_and_deterministic() {
    for id in IDS {
        for (k, plies) in [0usize, 7, 25].into_iter().enumerate() {
            let g = midgame(id, 11 + k as u64, plies);
            if g.outcome().is_some() {
                continue;
            }
            let legal = g.legal_moves();
            let a = g.clone_box().ai_move(3);
            assert!(
                a.as_ref().is_some_and(|m| legal.contains(m)),
                "{id}: AI ilegal {a:?}"
            );
            if !TIMED.contains(id) {
                let b = g.clone_box().ai_move(3);
                assert_eq!(a, b, "{id}: AI nedeterminist");
            }
        }
    }
}
