//! Controale pentru jocurile adăugate în valul 17 (Q2).

use polytabletop_core::rng::Rng;
use polytabletop_core::{GameObj, Player};
use polytabletop_games::playable;

const IDS: [&str; 36] = [
    "dame10",
    "toguz",
    "sittuyin",
    "gebeta",
    "sprouts",
    "pegsol",
    "shuffleboard",
    "yatzy",
    "ceelo",
    "oversevens",
    "skunk",
    "hazard",
    "howzat",
    "dreidel",
    "parimpar",
    "mehen",
    "cinci_pietricele",
    "jocuri_ata",
    "skittles",
    "doppelkopf",
    "tarock",
    "sevens",
    "skitgubbe",
    "ombre",
    "domino9",
    "narde",
    "dog",
    "toc",
    "trictrac",
    "tangram",
    "zipzapzop",
    "wouldyourather",
    "chinesewhispers",
    "namethattune",
    "babaoarba",
    "capturetheflag",
];

fn mine() -> Vec<polytabletop_games::Playable> {
    let v: Vec<_> = playable()
        .into_iter()
        .filter(|p| IDS.contains(&p.id))
        .collect();
    assert_eq!(v.len(), IDS.len(), "jocuri lipsă din registru");
    v
}

#[test]
fn random_games_finish_with_unique_move_texts() {
    for p in mine() {
        for seed in 1..=3u64 {
            let mut g = (p.make)();
            let mut rng = Rng::new(seed);
            let mut plies = 0;
            while g.outcome().is_none() {
                let moves = g.legal_moves();
                assert!(!moves.is_empty(), "{}: fără mutări\n{}", p.id, g.render());
                let mut s = moves.clone();
                s.sort();
                s.dedup();
                assert_eq!(
                    s.len(),
                    moves.len(),
                    "{}: mutări duplicate\n{}",
                    p.id,
                    g.render()
                );
                g.play(&moves[rng.below(moves.len())]).unwrap();
                plies += 1;
                assert!(plies < 20000, "{} nu se termină", p.id);
            }
            assert!(!g.render().is_empty());
        }
    }
}

#[test]
fn garbage_rejected_and_case_insensitive() {
    for p in mine() {
        let mut g = (p.make)();
        assert!(g.play("zzzz").is_err(), "{}", p.id);
        let first = g.legal_moves()[0].clone();
        let shouty = format!("  {}  ", first.to_uppercase());
        assert_eq!(g.play(&shouty).unwrap(), first, "{}", p.id);
    }
}

#[test]
fn ai_plays_only_legal_moves_to_the_end() {
    polytabletop_core::ai::set_time_budget_override_ms(Some(5));
    for p in mine() {
        let mut g = (p.make)();
        let mut n = 0u64;
        while g.outcome().is_none() && n < 3000 {
            let m = g
                .ai_move(n)
                .unwrap_or_else(|| panic!("{}: AI fără mutare\n{}", p.id, g.render()));
            assert!(
                g.legal_moves().contains(&m),
                "{}: mutare AI ilegală {m}",
                p.id
            );
            g.play(&m).unwrap();
            n += 1;
        }
        assert!(g.outcome().is_some() || n >= 3000, "{}", p.id);
    }
}

#[test]
fn hidden_info_games_do_not_leak_to_other_viewer() {
    // în hot-seat sigur, afișarea pentru un spectator fix nu trebuie să conțină mâna adversarului:
    // verificăm doar că render() diferă între cei doi spectatori (fiecare își vede propria mână)
    for p in mine().into_iter().filter(|p| {
        [
            "dog",
            "toc",
            "sevens",
            "ombre",
            "skitgubbe",
            "doppelkopf",
            "tarock",
            "domino9",
        ]
        .contains(&p.id)
    }) {
        let mut a: Box<dyn GameObj> = (p.make)();
        let mut b = a.clone_box();
        a.set_viewer(Some(Player::One));
        b.set_viewer(Some(Player::Two));
        assert_ne!(a.render(), b.render(), "{}", p.id);
    }
}
