//! Match harness: new time-budgeted AI vs old fixed-depth AI.
//! Run: `MATCH_GAMES=20 MATCH_MS=500 cargo test --release --test ai_match -- --ignored --nocapture`
//! (optional `MATCH_ONLY=chess` to run one game). Not part of the normal test run.

use polytabletop_core::rng::Rng;
use polytabletop_core::{ai, AiStyle, Game, Outcome, Player};
use polytabletop_games::{chess::Chess, connect4::Connect4, draughts::Draughts, gomoku::Gomoku};
use polytabletop_games::{janggi::Janggi, makruk::Makruk, morris::Morris, reversi::Reversi};
use polytabletop_games::{shogi::Shogi, xiangqi::Xiangqi};

fn env_u64(k: &str, d: u64) -> u64 {
    std::env::var(k)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(d)
}

/// Plays `games` games (colours alternate, each opening is repeated with colours
/// swapped). Returns (new wins, draws, old wins).
fn run<G: Game + Clone + Send + 'static>(
    mk: fn() -> G,
    old_depth: u32,
    games: u64,
    ms: u64,
) -> (u32, u32, u32) {
    let max_depth = match mk().ai_style() {
        AiStyle::AlphaBetaTime { max_depth, .. } => max_depth,
        _ => panic!("game is not on the timed AI"),
    };
    let mut handles = Vec::new();
    for gi in 0..games {
        handles.push(std::thread::spawn(move || {
            let new_side = if gi % 2 == 0 {
                Player::One
            } else {
                Player::Two
            };
            let pair = gi / 2;
            let mut g = mk();
            let mut open = Rng::new(1000 + pair);
            for _ in 0..2 {
                let mv = g.legal_moves();
                if g.outcome().is_some() || mv.is_empty() {
                    break;
                }
                let m = mv[open.below(mv.len())].clone();
                g.apply(&m);
            }
            let mut rng = Rng::new(77 + gi);
            let mut plies = 0;
            while g.outcome().is_none() && plies < 400 {
                let m = if g.to_move() == new_side {
                    ai::alphabeta_time(&g, ms, max_depth, rng.next_u64())
                } else {
                    ai::alphabeta_root(&g, old_depth, rng.next_u64())
                };
                match m {
                    Some(m) => g.apply(&m),
                    None => break,
                }
                plies += 1;
            }
            match g.outcome() {
                Some(Outcome::Win(p)) if p == new_side => 1,
                Some(Outcome::Win(_)) => -1,
                _ => 0,
            }
        }));
        // Two cores: keep at most two games in flight.
        if handles.len() >= 2 {
            let mut done = Vec::new();
            for h in handles.drain(..) {
                done.push(h.join().unwrap());
            }
            RESULTS.with(|r| r.borrow_mut().extend(done));
        }
    }
    for h in handles {
        let v = h.join().unwrap();
        RESULTS.with(|r| r.borrow_mut().push(v));
    }
    let all: Vec<i32> = RESULTS.with(|r| std::mem::take(&mut *r.borrow_mut()));
    (
        all.iter().filter(|&&x| x == 1).count() as u32,
        all.iter().filter(|&&x| x == 0).count() as u32,
        all.iter().filter(|&&x| x == -1).count() as u32,
    )
}

thread_local! {
    static RESULTS: std::cell::RefCell<Vec<i32>> = const { std::cell::RefCell::new(Vec::new()) };
}

#[test]
#[ignore]
fn new_vs_old() {
    let games = env_u64("MATCH_GAMES", 20);
    let ms = env_u64("MATCH_MS", 500);
    let only = std::env::var("MATCH_ONLY").ok();
    let want = |n: &str| only.as_deref().is_none_or(|o| o == n);
    macro_rules! m {
        ($name:expr, $ty:ty, $old:expr) => {
            if want($name) {
                let t = std::time::Instant::now();
                let (w, d, l) = run::<$ty>(<$ty>::new, $old, games, ms);
                println!(
                    "RESULT {:<10} new {w} - draws {d} - old {l}  (score {:.1}/{games}, old depth {}, {} ms, {:?})",
                    $name,
                    w as f64 + d as f64 / 2.0,
                    $old,
                    ms,
                    t.elapsed()
                );
            }
        };
    }
    m!("chess", Chess, 3);
    m!("draughts", Draughts, 7);
    m!("xiangqi", Xiangqi, 3);
    m!("shogi", Shogi, 3);
    m!("reversi", Reversi, 5);
    m!("connect4", Connect4, 7);
    m!("gomoku", Gomoku, 2);
    m!("morris", Morris, 4);
    m!("makruk", Makruk, 4);
    m!("janggi", Janggi, 3);
}
