//! `polytabletop replay FILE [--step]`: deterministic replay of a transcript.

use crate::record::{outcome_str, Record};
use polytabletop_core::rng::Rng;
use polytabletop_core::{GameObj, Player};
use std::io::{self, BufRead};

pub type StepFn<'a> = dyn FnMut(usize, Option<&str>, &dyn GameObj) + 'a;

pub struct Report {
    #[allow(dead_code)] // used by tests
    pub final_render: String,
    pub result: &'static str,
}

/// Rebuilds the game exactly as `polytabletop play` created it (global seed, the two
/// throw-away `make()` calls of the hot-seat check, viewer), then plays every
/// move. `on_step(ply, move, game)` is called for the initial position
/// (`ply = 0`, no move) and after each move. Fails on an illegal move or when
/// the final outcome differs from the recorded one.
pub fn run_with(rec: &Record, on_step: &mut StepFn) -> Result<Report, String> {
    let p = polytabletop_games::find(&rec.game)
        .ok_or_else(|| format!("joc necunoscut: {}", rec.game))?;
    Rng::set_global_seed(Some(rec.seed));
    let mut game = (p.make)();
    game.set_viewer(match rec.you.as_str() {
        "both" => None,
        "two" => Some(Player::Two),
        _ => Some(Player::One),
    });
    if matches!(rec.you.as_str(), "both" | "two")
        && !polytabletop_games::HOTSEAT_SAFE.contains(&p.id)
    {
        // `polytabletop play` builds two games here to detect hidden information; the
        // global seed counter advances, so replay must do the same.
        let _ = (p.make)();
        let _ = (p.make)();
    }
    on_step(0, None, game.as_ref());
    for (i, m) in rec.moves.iter().enumerate() {
        game.play(m)
            .map_err(|e| format!("mutarea {} («{m}»): {e}", i + 1))?;
        on_step(i + 1, Some(m), game.as_ref());
    }
    let result = outcome_str(game.outcome());
    if result != rec.result {
        return Err(format!(
            "rezultat diferit: înregistrat «{}», reluat «{result}»",
            rec.result
        ));
    }
    Ok(Report {
        final_render: game.render(),
        result,
    })
}

pub fn cmd_replay(args: &[String]) {
    let Some(file) = args.first() else {
        eprintln!("Folosire: polytabletop replay FILE [--step]");
        std::process::exit(2);
    };
    let step = args.iter().any(|a| a == "--step");
    let text = std::fs::read_to_string(file).unwrap_or_else(|e| {
        eprintln!("Nu pot citi {file}: {e}");
        std::process::exit(2)
    });
    let rec = Record::parse(&text).unwrap_or_else(|e| {
        eprintln!("{file}: {e}");
        std::process::exit(2)
    });
    println!(
        "Reluare: {} (seed {}, {} mutări, rezultat înregistrat: {})\n",
        rec.game,
        rec.seed,
        rec.moves.len(),
        rec.result
    );
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n = rec.moves.len();
    let res = run_with(&rec, &mut |ply, mv, g| {
        match mv {
            Some(m) => println!("--- mutarea {ply}/{n}: {m}"),
            None => println!("--- poziția inițială"),
        }
        println!("{}", g.render());
        if step && ply < n {
            println!("[Enter = următoarea mutare]");
            let _ = lines.next();
        }
    });
    match res {
        Ok(r) => println!(
            "Verificat: rezultatul reluat ({}) coincide cu cel înregistrat.",
            r.result
        ),
        Err(e) => {
            eprintln!("EROARE la reluare: {e}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::SEED_LOCK;

    fn play_random(id: &str, seed: u64, cap: usize) -> Record {
        let p = polytabletop_games::find(id).unwrap();
        Rng::set_global_seed(Some(seed));
        let mut g = (p.make)();
        g.set_viewer(Some(Player::One));
        let mut rec = Record::new(id, seed, "one");
        let mut rng = Rng::new(seed ^ 0xABCD);
        while rec.moves.len() < cap && g.outcome().is_none() {
            let lm = g.legal_moves();
            if lm.is_empty() {
                break;
            }
            let m = lm[rng.below(lm.len())].clone();
            let t = g.play(&m).unwrap();
            rec.moves.push(t);
        }
        rec.result = outcome_str(g.outcome()).to_string();
        rec
    }

    #[test]
    fn record_replay_round_trip_30_games() {
        let _l = SEED_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let ids: Vec<&str> = polytabletop_games::playable()
            .iter()
            .map(|p| p.id)
            .collect();
        let step = (ids.len() / 30).max(1);
        let picked: Vec<&str> = ids.iter().step_by(step).take(30).copied().collect();
        assert_eq!(picked.len(), 30, "need 30 games, have {}", ids.len());
        for (i, id) in picked.iter().enumerate() {
            let seed = 1000 + i as u64 * 7919;
            let rec = play_random(id, seed, 120);
            // Reference final render, rebuilt independently of the replayer.
            let parsed = Record::parse(&rec.to_text()).unwrap();
            assert_eq!(parsed, rec, "{id}");
            let mut last = String::new();
            let rep = run_with(&parsed, &mut |_, _, g| last = g.render())
                .unwrap_or_else(|e| panic!("{id}: {e}"));
            // Second, independent replay must be identical as well.
            let rep2 = run_with(&parsed, &mut |_, _, _| {}).unwrap();
            assert_eq!(rep.final_render, rep2.final_render, "{id}");
            assert_eq!(rep.final_render, last, "{id}");
            // Reference: play the recorded moves by hand on a fresh game.
            let p = polytabletop_games::find(id).unwrap();
            Rng::set_global_seed(Some(seed));
            let mut g = (p.make)();
            g.set_viewer(Some(Player::One));
            for m in &rec.moves {
                g.play(m).unwrap();
            }
            assert_eq!(rep.final_render, g.render(), "{id} seed {seed}");
            assert_eq!(rep.result, rec.result, "{id}");
        }
    }

    #[test]
    fn detects_tampered_result() {
        let _l = SEED_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let mut rec = play_random("chess", 5, 4);
        rec.result = "win1".into();
        assert!(run_with(&rec, &mut |_, _, _| {}).is_err());
    }
}
