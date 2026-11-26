//! AI-vs-AI arena: `polytabletop arena`, `polytabletop bench`, `polytabletop ladder`.
//!
//!   polytabletop arena ID [--games N] [--ms A:B[:C..]] [--level L1:L2] [--threads T]
//!                 [--seed S] [--max-plies P]
//!   polytabletop bench [--threads T] [--seed S]
//!   polytabletop ladder ID [--games N] [--threads T] [--seed S]
//!
//! Budgets are applied per seat through a per-thread override of the time
//! budget of the iterative-deepening search (`polytabletop_core::ai::set_thread_budget_ms`).
//! Games whose AI has a fixed depth / iteration count ignore the budget.

use polytabletop_core::ai::set_thread_budget_ms;
use polytabletop_core::rng::Rng;
use polytabletop_core::{GameObj, Outcome, Player};
use polytabletop_games::{find, playable, Playable};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Condvar, Mutex};
use std::time::Instant;

const LADDER: [u64; 6] = [10, 20, 40, 80, 160, 320];

pub fn dispatch(cmd: &str, args: &[String]) {
    let r = match cmd {
        "arena" => cmd_arena(args),
        "bench" => cmd_bench(args),
        _ => cmd_ladder(args),
    };
    if let Err(e) = r {
        eprintln!("{e}");
        std::process::exit(2);
    }
}

fn opt<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.windows(2)
        .find(|w| w[0] == name)
        .map(|w| w[1].as_str())
}

fn opt_num(args: &[String], name: &str, default: u64) -> Result<u64, String> {
    match opt(args, name) {
        None => Ok(default),
        Some(v) => v
            .parse::<u64>()
            .map_err(|_| format!("{name} așteaptă un număr întreg fără semn.")),
    }
}

fn default_threads() -> u64 {
    std::thread::available_parallelism()
        .map(|n| n.get() as u64)
        .unwrap_or(1)
        .min(4)
}

fn level_ms(s: &str) -> Result<u64, String> {
    match s {
        "easy" | "usor" => Ok(10),
        "normal" => Ok(40),
        "hard" | "greu" => Ok(160),
        "max" => Ok(640),
        n => n
            .parse::<u64>()
            .ok()
            .filter(|&v| v > 0)
            .ok_or_else(|| format!("nivel necunoscut: «{n}» (easy|normal|hard|max sau ms)")),
    }
}

/// Budgets per seat (ms) from `--ms A:B:C` or `--level easy:hard`.
fn budgets(args: &[String]) -> Result<Vec<u64>, String> {
    let (raw, lv) = match (opt(args, "--ms"), opt(args, "--level")) {
        (Some(m), _) => (m, false),
        (None, Some(l)) => (l, true),
        _ => return Ok(vec![50, 50]),
    };
    let mut v = Vec::new();
    for part in raw.split(':') {
        v.push(if lv {
            level_ms(part)?
        } else {
            level_ms(part).and_then(|x| {
                if part.parse::<u64>().is_ok() {
                    Ok(x)
                } else {
                    Err(format!("--ms așteaptă numere: «{part}»"))
                }
            })?
        });
    }
    if v.len() == 1 {
        v.push(v[0]);
    }
    Ok(v)
}

#[derive(Default, Clone)]
struct SeatTime {
    sum_ms: f64,
    n: u64,
    worst: f64,
}

#[derive(Default, Clone)]
struct Stats {
    a_win: u32,
    b_win: u32,
    draw: u32,
    truncated: u32,
    plies: u64,
    games: u32,
    seats: Vec<SeatTime>,
}

impl Stats {
    fn merge(&mut self, o: &Stats) {
        self.a_win += o.a_win;
        self.b_win += o.b_win;
        self.draw += o.draw;
        self.truncated += o.truncated;
        self.plies += o.plies;
        self.games += o.games;
        if self.seats.len() < o.seats.len() {
            self.seats.resize(o.seats.len(), SeatTime::default());
        }
        for (s, t) in self.seats.iter_mut().zip(&o.seats) {
            s.sum_ms += t.sum_ms;
            s.n += t.n;
            s.worst = s.worst.max(t.worst);
        }
    }
    fn all_time(&self) -> (f64, u64, f64) {
        let mut s = 0.0;
        let mut n = 0;
        let mut w: f64 = 0.0;
        for t in &self.seats {
            s += t.sum_ms;
            n += t.n;
            w = w.max(t.worst);
        }
        (s, n, w)
    }
}

/// One match. Team A = seat 0; team B = seats 1.. (their moves rotate among the
/// seats). Even games: A plays `Player::One`; odd games: A plays `Player::Two`.
fn play_match(mut g: Box<dyn GameObj>, idx: usize, b: &[u64], seed: u64, max_plies: u64) -> Stats {
    let a_side = if idx.is_multiple_of(2) {
        Player::One
    } else {
        Player::Two
    };
    let mut rng = Rng::new(seed ^ (idx as u64 + 1).wrapping_mul(0x9E37_79B9_7F4A_7C15));
    let mut st = Stats {
        games: 1,
        seats: vec![SeatTime::default(); b.len()],
        ..Default::default()
    };
    let mut b_turn = 0usize;
    let mut plies = 0u64;
    let mut over = None;
    while plies < max_plies {
        if let Some(o) = g.outcome() {
            over = Some(o);
            break;
        }
        let side = g.to_move();
        let seat = if side == a_side {
            0
        } else {
            let s = 1 + b_turn % (b.len() - 1);
            b_turn += 1;
            s
        };
        set_thread_budget_ms(Some(b[seat]));
        let t0 = Instant::now();
        let mv = g.ai_move(rng.next_u64());
        let ms = t0.elapsed().as_secs_f64() * 1000.0;
        let t = &mut st.seats[seat];
        t.sum_ms += ms;
        t.n += 1;
        t.worst = t.worst.max(ms);
        let Some(mv) = mv else { break };
        if g.play(&mv).is_err() {
            break;
        }
        plies += 1;
    }
    set_thread_budget_ms(None);
    st.plies = plies;
    match over.or_else(|| g.outcome()) {
        Some(Outcome::Win(p)) if p == a_side => st.a_win = 1,
        Some(Outcome::Win(_)) => st.b_win = 1,
        Some(Outcome::Draw) => st.draw = 1,
        None => {
            st.draw = 1;
            st.truncated = 1;
        }
    }
    st
}

/// Plays `games` matches on `threads` threads. Games are created in index
/// order (a ticket) so that the seeded deals are the same whatever the
/// thread count.
fn run_series(p: &Playable, b: &[u64], games: usize, threads: usize, seed: u64, cap: u64) -> Stats {
    Rng::set_global_seed(Some(seed));
    let next = AtomicUsize::new(0);
    let ticket = (Mutex::new(0usize), Condvar::new());
    let total = Mutex::new(Stats::default());
    std::thread::scope(|sc| {
        for _ in 0..threads.clamp(1, games.max(1)) {
            sc.spawn(|| loop {
                let idx = next.fetch_add(1, Ordering::SeqCst);
                if idx >= games {
                    break;
                }
                let g = {
                    let (m, cv) = &ticket;
                    let mut t = m.lock().unwrap();
                    while *t != idx {
                        t = cv.wait(t).unwrap();
                    }
                    let g = (p.make)();
                    *t += 1;
                    cv.notify_all();
                    g
                };
                let s = play_match(g, idx, b, seed, cap);
                total.lock().unwrap().merge(&s);
            });
        }
    });
    total.into_inner().unwrap()
}

fn lookup(args: &[String]) -> Result<Playable, String> {
    let id = args
        .first()
        .filter(|a| !a.starts_with("--"))
        .ok_or("Lipsește ID-ul jocului. Încearcă `polytabletop list`.")?;
    find(id).ok_or_else(|| format!("Joc necunoscut: {id}. Încearcă `polytabletop list`."))
}

fn cmd_arena(args: &[String]) -> Result<(), String> {
    let p = lookup(args)?;
    let b = budgets(args)?;
    let games = opt_num(args, "--games", 10)? as usize;
    let threads = opt_num(args, "--threads", default_threads())? as usize;
    let seed = opt_num(args, "--seed", 1)?;
    let cap = opt_num(args, "--max-plies", 600)?;
    let t0 = Instant::now();
    let s = run_series(&p, &b, games, threads, seed, cap);
    let (sum, n, worst) = s.all_time();
    let budget_txt: Vec<String> = b.iter().map(|x| format!("{x} ms")).collect();
    println!(
        "{} ({}) — {} partide, bugete pe locuri [{}], seed {seed}, {} fire",
        p.title,
        p.id,
        s.games,
        budget_txt.join(" | "),
        threads.max(1)
    );
    println!(
        "Locul 1 (A, {} ms): {} victorii | remize: {} | Locul 2{} (B): {} victorii",
        b[0],
        s.a_win,
        s.draw,
        if b.len() > 2 {
            "..".to_string() + &b.len().to_string()
        } else {
            String::new()
        },
        s.b_win
    );
    if s.truncated > 0 {
        println!(
            "  din care {} oprite la limita de {cap} mutări (numărate ca remiză)",
            s.truncated
        );
    }
    println!(
        "Lungime medie: {:.1} mutări | timp mediu/mutare: {:.1} ms | cea mai lentă mutare: {:.0} ms | total {:.1} s",
        s.plies as f64 / s.games.max(1) as f64,
        sum / n.max(1) as f64,
        worst,
        t0.elapsed().as_secs_f64()
    );
    if b.len() > 2 {
        for (i, t) in s.seats.iter().enumerate() {
            println!(
                "  loc {} ({} ms): {:.1} ms/mutare, maxim {:.0} ms",
                i + 1,
                b[i],
                t.sum_ms / t.n.max(1) as f64,
                t.worst
            );
        }
    }
    Ok(())
}

fn cmd_bench(args: &[String]) -> Result<(), String> {
    let threads = opt_num(args, "--threads", 1)? as usize;
    let seed = opt_num(args, "--seed", 1)?;
    let only = opt(args, "--only");
    let list: Vec<Playable> = playable()
        .into_iter()
        .filter(|p| only.is_none_or(|o| o.split(',').any(|x| x == p.id)))
        .collect();
    let rows = Mutex::new(Vec::new());
    let next = AtomicUsize::new(0);
    let ticket = (Mutex::new(0usize), Condvar::new());
    Rng::set_global_seed(Some(seed));
    std::thread::scope(|sc| {
        for _ in 0..threads.clamp(1, list.len().max(1)) {
            sc.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::SeqCst);
                if i >= list.len() {
                    break;
                }
                let mut g = {
                    let (m, cv) = &ticket;
                    let mut t = m.lock().unwrap();
                    while *t != i {
                        t = cv.wait(t).unwrap();
                    }
                    let g = (list[i].make)();
                    *t += 1;
                    cv.notify_all();
                    g
                };
                let mut rng = Rng::new(seed ^ ((i as u64 + 1) * 0x9E37_79B9));
                // Reach a mid-game position with random moves.
                for _ in 0..8 {
                    let mv = g.legal_moves();
                    if g.outcome().is_some() || mv.is_empty() {
                        break;
                    }
                    let _ = g.play(&mv[rng.below(mv.len())]);
                }
                set_thread_budget_ms(Some(20));
                let (mut sum, mut worst, mut n) = (0.0f64, 0.0f64, 0u32);
                for _ in 0..3 {
                    if g.outcome().is_some() {
                        break;
                    }
                    let t0 = Instant::now();
                    let mv = g.ai_move(rng.next_u64());
                    let ms = t0.elapsed().as_secs_f64() * 1000.0;
                    sum += ms;
                    worst = worst.max(ms);
                    n += 1;
                    match mv {
                        Some(m) => {
                            let _ = g.play(&m);
                        }
                        None => break,
                    }
                }
                set_thread_budget_ms(None);
                rows.lock()
                    .unwrap()
                    .push((i, list[i].id, list[i].category, n, sum, worst));
            });
        }
    });
    let mut rows = rows.into_inner().unwrap();
    rows.sort_by_key(|r| r.0);
    println!(
        "{:<14} {:<10} {:>6} {:>10} {:>10}",
        "joc", "categorie", "mutări", "med ms", "max ms"
    );
    let (mut tsum, mut tn, mut tworst) = (0.0, 0u32, 0.0f64);
    for (_, id, cat, n, sum, worst) in &rows {
        let avg = if *n > 0 { sum / *n as f64 } else { 0.0 };
        println!("{id:<14} {cat:<10} {n:>6} {avg:>10.1} {worst:>10.0}");
        tsum += sum;
        tn += n;
        tworst = tworst.max(*worst);
    }
    println!(
        "\n{} jocuri, {} mutări cronometrate (buget AI 20 ms), medie {:.1} ms, maxim {:.0} ms",
        rows.len(),
        tn,
        tsum / tn.max(1) as f64,
        tworst
    );
    Ok(())
}

/// Elo-like fit (anchored at the weakest budget = 0) from a score matrix.
fn fit_elo(score: &[Vec<f64>], played: &[Vec<f64>]) -> Vec<f64> {
    let n = score.len();
    let mut r = vec![0.0f64; n];
    for _ in 0..2000 {
        let mut g = vec![0.0f64; n];
        for i in 0..n {
            for j in 0..n {
                if i == j || played[i][j] == 0.0 {
                    continue;
                }
                // Half-game prior towards 50% keeps the fit finite for sweeps.
                let s = (score[i][j] + 0.5) / (played[i][j] + 1.0);
                let e = 1.0 / (1.0 + 10f64.powf((r[j] - r[i]) / 400.0));
                g[i] += (s - e) * played[i][j];
            }
        }
        for i in 0..n {
            r[i] += 20.0 * g[i];
        }
    }
    let base = r[0];
    r.iter().map(|x| x - base).collect()
}

fn cmd_ladder(args: &[String]) -> Result<(), String> {
    let p = lookup(args)?;
    let games = opt_num(args, "--games", 6)? as usize;
    let threads = opt_num(args, "--threads", default_threads())? as usize;
    let seed = opt_num(args, "--seed", 1)?;
    let cap = opt_num(args, "--max-plies", 600)?;
    let n = LADDER.len();
    let mut score = vec![vec![0.0; n]; n];
    let mut played = vec![vec![0.0; n]; n];
    let t0 = Instant::now();
    for i in 0..n {
        for j in i + 1..n {
            let s = run_series(&p, &[LADDER[i], LADDER[j]], games, threads, seed, cap);
            let gi = s.a_win as f64 + 0.5 * s.draw as f64;
            let gj = s.b_win as f64 + 0.5 * s.draw as f64;
            score[i][j] = gi;
            score[j][i] = gj;
            played[i][j] = s.games as f64;
            played[j][i] = s.games as f64;
        }
    }
    let elo = fit_elo(&score, &played);
    println!(
        "{} ({}) — scară de bugete, {games} partide/pereche, seed {seed}",
        p.title, p.id
    );
    println!(
        "{:>8} {:>8} {:>8} {:>8}",
        "buget", "puncte", "partide", "Elo"
    );
    for i in 0..n {
        let pts: f64 = score[i].iter().sum();
        let pl: f64 = played[i].iter().sum();
        println!("{:>5} ms {pts:>8.1} {pl:>8.0} {:>8.0}", LADDER[i], elo[i]);
    }
    println!(
        "(total {:.1} s; Elo relativ la 10 ms = 0)",
        t0.elapsed().as_secs_f64()
    );
    Ok(())
}
