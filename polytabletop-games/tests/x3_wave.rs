//! Verificări generice pentru jocurile valului x3 (Ceasornicul și Coroana).

use polytabletop_core::rng::Rng;
use polytabletop_core::GameObj;
use polytabletop_games::find;

const IDS: [&str; 2] = ["ceasornic", "coroana"];

#[test]
fn random_games_end_within_cap_with_unique_moves() {
    for id in IDS {
        let p = find(id).unwrap();
        for seed in 1..=40u64 {
            let mut g = (p.make)();
            let mut rng = Rng::new(seed);
            let mut plies = 0;
            while g.outcome().is_none() {
                let moves = g.legal_moves();
                assert!(!moves.is_empty(), "{id}\n{}", g.render());
                let mut s = moves.clone();
                s.sort();
                s.dedup();
                assert_eq!(s.len(), moves.len(), "{id} duplicate move text");
                g.play(&moves[rng.below(moves.len())]).unwrap();
                plies += 1;
                assert!(plies < 4000, "{id} nu s-a terminat\n{}", g.render());
            }
        }
    }
}

#[test]
fn garbage_rejected_and_case_insensitive() {
    for id in IDS {
        let p = find(id).unwrap();
        let mut g = (p.make)();
        assert!(g.play("zzzz").is_err());
        assert!(g.play("").is_err());
        let first = g.legal_moves()[0].clone();
        assert_eq!(
            g.play(&format!("  {}  ", first.to_uppercase())).unwrap(),
            first
        );
    }
}

#[test]
fn ai_answers_fast() {
    for id in IDS {
        let p = find(id).unwrap();
        let mut g: Box<dyn GameObj> = (p.make)();
        let mut rng = Rng::new(5);
        let mut worst = 0f32;
        let mut n = 0;
        while g.outcome().is_none() && n < 1500 {
            let t = std::time::Instant::now();
            let m = g.ai_move(rng.next_u64()).unwrap();
            worst = worst.max(t.elapsed().as_secs_f32());
            g.play(&m).unwrap();
            n += 1;
        }
        assert!(worst < 1.0, "{id}: {worst}");
    }
}
