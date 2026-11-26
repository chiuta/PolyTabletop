//! Itinerar: nivelurile IA (global per proces) — Hard bate Easy, partidele se termină.
use polytabletop_core::ai::{self, Level};
use polytabletop_core::rng::Rng;
use polytabletop_core::{Game, Outcome, Player};
use polytabletop_games::itinerar::Itinerar;

fn ai_game(seed: u64, lv: [Level; 2]) -> Option<Player> {
    let mut g = Itinerar::with_seed(seed);
    let mut rng = Rng::new(seed ^ 7);
    let mut n = 0;
    while g.outcome().is_none() {
        ai::set_level(lv[g.to_move().idx()]);
        let m = ai::best_move(&g, rng.next_u64()).unwrap();
        g.apply(&m);
        n += 1;
        assert!(n < 20_000);
    }
    match g.outcome() {
        Some(Outcome::Win(p)) => Some(p),
        _ => None,
    }
}

#[test]
fn ai_levels_finish_and_hard_beats_easy() {
    let mut hard = 0;
    let mut easy = 0;
    for seed in 1..=30u64 {
        let hard_first = seed % 2 == 0;
        let lv = if hard_first {
            [Level::Hard, Level::Easy]
        } else {
            [Level::Easy, Level::Hard]
        };
        let hp = if hard_first { Player::One } else { Player::Two };
        match ai_game(seed, lv) {
            Some(w) if w == hp => hard += 1,
            Some(_) => easy += 1,
            None => {}
        }
    }
    eprintln!("itinerar: Hard {hard} - Easy {easy} din 30");
    assert!(hard > easy, "{hard} vs {easy}");
    // Normal și Expert termină partide
    for lv in [Level::Normal, Level::Expert] {
        for seed in 1..=4u64 {
            ai_game(seed, [lv, lv]);
        }
    }
    // partidele Normal contra Normal nu se blochează și au câștigător sau remiză valide
    let mut decisive = 0;
    for seed in 100..=115u64 {
        if ai_game(seed, [Level::Normal, Level::Normal]).is_some() {
            decisive += 1;
        }
    }
    eprintln!("itinerar: Normal vs Normal decisive {decisive}/16");
    assert!(decisive >= 12);
}
