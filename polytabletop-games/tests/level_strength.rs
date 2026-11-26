//! AI difficulty levels (global `polytabletop_core::ai::set_level`). Single #[test] per
//! binary section because the level is process-global.
use polytabletop_core::ai::{set_level, set_time_budget_override_ms, Level};
use polytabletop_core::{GameObj, Outcome, Player};
use polytabletop_games::find;
use std::time::Instant;

/// Plays `games` games Expert vs Easy (seats alternate); returns (expert wins, draws).
fn duel(id: &str, games: u64, ms: u64, max_plies: usize) -> (u64, u64) {
    set_time_budget_override_ms(Some(ms));
    let p = find(id).unwrap();
    let (mut w, mut d) = (0, 0);
    for k in 0..games {
        let mut g: Box<dyn GameObj> = (p.make)();
        let expert = if k % 2 == 0 { Player::One } else { Player::Two };
        let mut n = 0;
        while g.outcome().is_none() && n < max_plies {
            let lv = if g.to_move() == expert {
                Level::Expert
            } else {
                Level::Easy
            };
            set_level(lv);
            let Some(m) = g.ai_move(k * 1000 + n as u64) else {
                break;
            };
            g.play(&m).unwrap();
            n += 1;
        }
        match g.outcome() {
            Some(Outcome::Win(p)) if p == expert => w += 1,
            Some(Outcome::Draw) | None => d += 1,
            _ => {}
        }
    }
    set_level(Level::Normal);
    set_time_budget_override_ms(None);
    (w, d)
}

#[test]
fn levels_connect4() {
    let (w, d) = duel("connect4", 10, 5, 60);
    eprintln!("connect4 expert-vs-easy: {w}/10 wins, {d} draws");
    assert!(w >= 8, "expert won only {w}/10");
}

#[test]
#[ignore]
fn levels_hex() {
    let (w, d) = duel("hex", 10, 20, 130);
    eprintln!("hex expert-vs-easy: {w}/10 wins, {d} draws");
    assert!(w >= 8, "expert won only {w}/10");
}

#[test]
fn amazoane_first_ply_in_budget() {
    set_time_budget_override_ms(Some(50));
    set_level(Level::Normal);
    let g = (find("amazoane").unwrap().make)();
    let t = Instant::now();
    assert!(g.ai_move(1).is_some());
    let ms = t.elapsed().as_millis();
    eprintln!("amazoane first ply, budget 50ms: {ms} ms");
    set_time_budget_override_ms(None);
    assert!(ms < 400, "took {ms} ms");
}
