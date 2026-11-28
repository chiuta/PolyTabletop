//! Generic AI: alpha-beta (for games with a good evaluation) and UCT Monte-Carlo
//! tree search (for games like Go). Both work on any `Game`.

use crate::rng::Rng;
use crate::{AiStyle, Game, Outcome, Player};
use std::sync::atomic::{AtomicU64, AtomicU8, Ordering};
use std::sync::OnceLock;
use std::time::Duration;
use web_time::Instant;

const WIN: i32 = 1_000_000;

/// Best move according to the game's preferred AI style.
pub fn best_move<G: Game>(g: &G, seed: u64) -> Option<G::Move> {
    // search a determinised view: the AI must not see the opponent's hidden information
    let view = g.ai_view(g.to_move());
    let g = &view;
    let lv = level();
    match g.ai_style() {
        AiStyle::AlphaBeta(d) => {
            let d = match lv {
                Level::Easy => d.saturating_sub(1).max(1),
                Level::Normal => d,
                Level::Hard if d <= 4 => d + 1,
                Level::Expert if d <= 6 => d + 1,
                _ => d,
            };
            alphabeta_root(g, d, seed)
        }
        AiStyle::Mcts(n) => mcts(g, scale_u32(n, lv), seed),
        AiStyle::AlphaBetaTime { ms, max_depth } => {
            alphabeta_time(g, budget_ms(ms as u64), max_depth, seed)
        }
    }
}

/// Strength level of the AI (global setting, see [`set_level`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Easy,
    Normal,
    Hard,
    Expert,
}

impl Level {
    /// Multiplier (in tenths) applied to time budgets / rollout counts.
    pub fn factor_x10(self) -> u64 {
        match self {
            Level::Easy => 1,
            Level::Normal => 10,
            Level::Hard => 25,
            Level::Expert => 60,
        }
    }
    pub fn parse(s: &str) -> Option<Level> {
        match s.trim().to_ascii_lowercase().as_str() {
            "easy" | "usor" | "ușor" => Some(Level::Easy),
            "normal" => Some(Level::Normal),
            "hard" | "greu" => Some(Level::Hard),
            "expert" => Some(Level::Expert),
            _ => None,
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Level::Easy => "easy",
            Level::Normal => "normal",
            Level::Hard => "hard",
            Level::Expert => "expert",
        }
    }
}

// 255 = unset (then POLYTABLETOP_LEVEL is consulted, default normal).
static LEVEL: AtomicU8 = AtomicU8::new(255);

/// Sets the global AI level (CLI `--level`, GUI).
pub fn set_level(l: Level) {
    LEVEL.store(l as u8, Ordering::Relaxed);
}

/// Current level: explicit `set_level`, else env `POLYTABLETOP_LEVEL`, else normal.
pub fn level() -> Level {
    match LEVEL.load(Ordering::Relaxed) {
        0 => Level::Easy,
        1 => Level::Normal,
        2 => Level::Hard,
        3 => Level::Expert,
        _ => {
            static ENV: OnceLock<Level> = OnceLock::new();
            *ENV.get_or_init(|| {
                std::env::var("POLYTABLETOP_LEVEL")
                    .ok()
                    .and_then(|v| Level::parse(&v))
                    .unwrap_or(Level::Normal)
            })
        }
    }
}

fn scale_u32(n: u32, l: Level) -> u32 {
    ((n as u64 * l.factor_x10() / 10).max(8)).min(u32::MAX as u64) as u32
}

/// For `Easy`: with probability ~25% choose the 2nd/3rd best of `scored`
/// (value, move) instead of the best. `scored` need not be sorted.
fn easy_pick<M: Clone>(mut scored: Vec<(f64, M)>, best: Option<M>, seed: u64) -> Option<M> {
    if level() != Level::Easy || scored.len() < 2 {
        return best;
    }
    let mut rng = Rng::new(seed ^ 0xE45E_7001);
    if rng.below(100) >= 25 {
        return best;
    }
    scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
    let top = scored.len().min(3);
    Some(scored[1 + rng.below(top - 1)].1.clone())
}

static BUDGET_OVERRIDE: AtomicU64 = AtomicU64::new(0);

/// Overrides the time budget of every `AlphaBetaTime` search (e.g. a few ms in tests).
/// `None` removes the override (the `POLYTABLETOP_AI_MS` environment variable still applies).
pub fn set_time_budget_override_ms(ms: Option<u64>) {
    BUDGET_OVERRIDE.store(ms.unwrap_or(0), Ordering::Relaxed);
}

thread_local! {
    static THREAD_BUDGET: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// Per-thread time budget override (wins over the global one); `None` removes it.
/// Lets an arena give each seat its own budget from several threads.
pub fn set_thread_budget_ms(ms: Option<u64>) {
    THREAD_BUDGET.with(|c| c.set(ms.unwrap_or(0)));
}

fn budget_ms(default: u64) -> u64 {
    let t = THREAD_BUDGET.with(|c| c.get());
    if t > 0 {
        return t;
    }
    let f = level().factor_x10();
    let sc = |v: u64| (v * f / 10).max(1);
    let o = BUDGET_OVERRIDE.load(Ordering::Relaxed);
    if o > 0 {
        return sc(o);
    }
    static ENV: OnceLock<u64> = OnceLock::new();
    let e = *ENV.get_or_init(|| {
        std::env::var("POLYTABLETOP_AI_MS")
            .ok()
            .and_then(|v| v.trim().parse().ok())
            .unwrap_or(0)
    });
    sc(if e > 0 { e } else { default })
}

pub fn alphabeta_root<G: Game>(g: &G, depth: u32, seed: u64) -> Option<G::Move> {
    let moves = g.legal_moves();
    if moves.is_empty() {
        return None;
    }
    let root = g.to_move();
    let mut best = i32::MIN;
    let mut best_moves: Vec<usize> = Vec::new();
    let mut alpha = i32::MIN + 1;
    let easy = level() == Level::Easy;
    let mut all: Vec<(f64, usize)> = Vec::new();
    for (i, m) in moves.iter().enumerate() {
        let mut c = g.clone();
        c.apply(m);
        // Window kept slightly open so equal moves are all found (for random tie-break).
        // Easy needs exact scores for every move (second/third best).
        let lo = if easy { i32::MIN + 1 } else { alpha - 1 };
        let v = ab(&c, depth.saturating_sub(1), lo, i32::MAX, root, 1);
        if easy {
            all.push((v as f64, i));
        }
        if v > best {
            best = v;
            best_moves.clear();
            best_moves.push(i);
            alpha = alpha.max(best);
        } else if v == best {
            best_moves.push(i);
        }
    }
    let mut rng = Rng::new(seed);
    let pick = best_moves[rng.below(best_moves.len())];
    if easy {
        let r = easy_pick(all, Some(pick), seed);
        return r.map(|i| moves[i].clone());
    }
    Some(moves[pick].clone())
}

fn ab<G: Game>(g: &G, depth: u32, mut alpha: i32, mut beta: i32, root: Player, ply: i32) -> i32 {
    if let Some(o) = g.outcome() {
        return match o {
            Outcome::Win(p) if p == root => WIN - ply,
            Outcome::Win(_) => -WIN + ply,
            Outcome::Draw => 0,
        };
    }
    if depth == 0 {
        return g.evaluate(root);
    }
    let moves = g.legal_moves();
    if moves.is_empty() {
        return g.evaluate(root);
    }
    if g.to_move() == root {
        let mut best = i32::MIN;
        for m in moves {
            let mut c = g.clone();
            c.apply(&m);
            let v = ab(&c, depth - 1, alpha, beta, root, ply + 1);
            best = best.max(v);
            alpha = alpha.max(best);
            if alpha >= beta {
                break;
            }
        }
        best
    } else {
        let mut best = i32::MAX;
        for m in moves {
            let mut c = g.clone();
            c.apply(&m);
            let v = ab(&c, depth - 1, alpha, beta, root, ply + 1);
            best = best.min(v);
            beta = beta.min(best);
            if alpha >= beta {
                break;
            }
        }
        best
    }
}

struct Node<M> {
    parent: usize,
    mv: Option<M>,
    children: Vec<usize>,
    untried: Vec<M>,
    wins: f64,
    visits: f64,
    /// Player who made the move leading into this node.
    mover: Player,
}

/// UCT Monte-Carlo tree search with random playouts.
pub fn mcts<G: Game>(root: &G, iterations: u32, seed: u64) -> Option<G::Move> {
    let moves = root.legal_moves();
    if moves.is_empty() {
        return None;
    }
    if moves.len() == 1 {
        return Some(moves[0].clone());
    }
    let mut rng = Rng::new(seed);
    let cap = root.playout_cap();
    let mut nodes: Vec<Node<G::Move>> = vec![Node {
        parent: usize::MAX,
        mv: None,
        children: Vec::new(),
        untried: moves,
        wins: 0.0,
        visits: 0.0,
        mover: root.to_move().other(),
    }];

    for _ in 0..iterations {
        let mut state = root.clone();
        let mut cur = 0usize;

        // Selection.
        while nodes[cur].untried.is_empty() && !nodes[cur].children.is_empty() {
            let ln = nodes[cur].visits.max(1.0).ln();
            let mut best = nodes[cur].children[0];
            let mut best_score = f64::MIN;
            for &c in &nodes[cur].children {
                let n = &nodes[c];
                let s = n.wins / n.visits + 1.4 * (ln / n.visits).sqrt();
                if s > best_score {
                    best_score = s;
                    best = c;
                }
            }
            cur = best;
            if let Some(m) = nodes[cur].mv.as_ref() {
                state.apply(m);
            }
        }

        // Expansion.
        if !nodes[cur].untried.is_empty() {
            let i = rng.below(nodes[cur].untried.len());
            let m = nodes[cur].untried.swap_remove(i);
            let mover = state.to_move();
            state.apply(&m);
            let untried = if state.outcome().is_some() {
                Vec::new()
            } else {
                state.legal_moves()
            };
            let id = nodes.len();
            nodes.push(Node {
                parent: cur,
                mv: Some(m),
                children: Vec::new(),
                untried,
                wins: 0.0,
                visits: 0.0,
                mover,
            });
            nodes[cur].children.push(id);
            cur = id;
        }

        // Playout.
        let mut plies = 0usize;
        let result = loop {
            if let Some(o) = state.outcome() {
                break Some(o);
            }
            if plies >= cap {
                break None;
            }
            match state.playout_move(&mut rng) {
                Some(m) => {
                    state.apply(&m);
                    plies += 1;
                }
                None => break None,
            }
        };

        // Back-propagation.
        let mut n = cur;
        loop {
            let mover = nodes[n].mover;
            let r = match result {
                Some(Outcome::Win(p)) => {
                    if p == mover {
                        1.0
                    } else {
                        0.0
                    }
                }
                _ => 0.5,
            };
            nodes[n].visits += 1.0;
            nodes[n].wins += r;
            if nodes[n].parent == usize::MAX {
                break;
            }
            n = nodes[n].parent;
        }
    }

    let mut best = None;
    let mut best_visits = -1.0;
    let mut all: Vec<(f64, G::Move)> = Vec::new();
    for &c in &nodes[0].children {
        if let Some(m) = nodes[c].mv.clone() {
            all.push((nodes[c].visits, m));
        }
        if nodes[c].visits > best_visits {
            best_visits = nodes[c].visits;
            best = nodes[c].mv.clone();
        }
    }
    easy_pick(all, best, seed)
}

// ---------------------------------------------------------------------------
// Iterative-deepening alpha-beta with a time budget (negamax, fail-soft).
// ---------------------------------------------------------------------------

const MATE_BOUND: i32 = WIN - 1000;
const TT_BITS: u32 = 18;
const HIST: usize = 1 << 16;
const MAX_PLY: usize = 128;
const QDEPTH: u32 = 8;

#[derive(Clone, Copy)]
struct Entry {
    key: u64,
    score: i32,
    mv: u16,
    depth: i8,
    flag: u8, // 0 empty, 1 exact, 2 lower bound, 3 upper bound
}

struct Searcher<G: Game> {
    deadline: Instant,
    nodes: u64,
    stop: bool,
    tt: Vec<Entry>,
    use_tt: bool,
    use_q: bool,
    killers: Vec<[Option<G::Move>; 2]>,
    history: Vec<i32>,
}

impl<G: Game> Searcher<G> {
    fn tick(&mut self) -> bool {
        self.nodes += 1;
        if self.nodes & 1023 == 0 && Instant::now() >= self.deadline {
            self.stop = true;
        }
        self.stop
    }

    fn terminal(g: &G, ply: i32) -> Option<i32> {
        g.outcome().map(|o| match o {
            Outcome::Win(p) if p == g.to_move() => WIN - ply,
            Outcome::Win(_) => -WIN + ply,
            Outcome::Draw => 0,
        })
    }

    /// Order move indices: tt move, tactical/scored moves, killers, history.
    fn order(&self, g: &G, moves: &[G::Move], tt_mv: Option<usize>, ply: usize) -> Vec<usize> {
        let side = g.to_move().idx() * HIST;
        // Huge move lists (e.g. ~2000 moves): once the deadline has passed, stop
        // scoring (remaining moves keep their natural order).
        let mut late = false;
        let mut scored: Vec<(i32, usize)> = moves
            .iter()
            .enumerate()
            .map(|(i, m)| {
                if !late && i & 127 == 127 && Instant::now() >= self.deadline {
                    late = true;
                }
                if late {
                    return (0, i);
                }
                let mut s = g.move_score(m) << 10;
                if Some(i) == tt_mv {
                    s += 1 << 28;
                } else {
                    if ply < MAX_PLY {
                        if self.killers[ply][0].as_ref() == Some(m) {
                            s += 50 << 10;
                        } else if self.killers[ply][1].as_ref() == Some(m) {
                            s += 49 << 10;
                        }
                    }
                    if let Some(k) = g.move_key(m) {
                        s += self.history[side + (k as usize & (HIST - 1))].min(1023);
                    }
                }
                (-s, i)
            })
            .collect();
        scored.sort_unstable();
        scored.into_iter().map(|(_, i)| i).collect()
    }

    fn child_value(
        &mut self,
        g: &G,
        m: &G::Move,
        depth: i32,
        alpha: i32,
        beta: i32,
        ply: usize,
    ) -> i32 {
        let mut c = g.clone();
        c.apply(m);
        if c.to_move() == g.to_move() {
            self.search(&c, depth, alpha, beta, ply + 1)
        } else {
            -self.search(&c, depth, -beta, -alpha, ply + 1)
        }
    }

    fn search(&mut self, g: &G, depth: i32, mut alpha: i32, mut beta: i32, ply: usize) -> i32 {
        if self.tick() {
            return 0;
        }
        if let Some(v) = Self::terminal(g, ply as i32) {
            return v;
        }
        if depth <= 0 {
            return if self.use_q {
                self.qsearch(g, alpha, beta, ply, QDEPTH)
            } else {
                g.evaluate(g.to_move())
            };
        }
        let alpha0 = alpha;
        let key = if self.use_tt { g.hash_key() } else { None };
        let mut tt_mv = None;
        if let Some(k) = key {
            let e = self.tt[(k >> (64 - TT_BITS)) as usize];
            if e.flag != 0 && e.key == k {
                tt_mv = Some(e.mv as usize);
                if e.depth as i32 >= depth {
                    let mut s = e.score;
                    if s > MATE_BOUND {
                        s -= ply as i32;
                    } else if s < -MATE_BOUND {
                        s += ply as i32;
                    }
                    match e.flag {
                        1 => return s,
                        2 => alpha = alpha.max(s),
                        _ => beta = beta.min(s),
                    }
                    if alpha >= beta {
                        return s;
                    }
                }
            }
        }
        let moves = g.legal_moves();
        if moves.is_empty() {
            return g.evaluate(g.to_move());
        }
        let order = self.order(g, &moves, tt_mv, ply);
        let mut best = i32::MIN + 1;
        let mut best_i = 0usize;
        for &i in &order {
            let v = self.child_value(g, &moves[i], depth - 1, alpha, beta, ply);
            if self.stop {
                return 0;
            }
            if v > best {
                best = v;
                best_i = i;
            }
            alpha = alpha.max(best);
            if alpha >= beta {
                let m = &moves[i];
                if !g.is_tactical(m) && ply < MAX_PLY {
                    if self.killers[ply][0].as_ref() != Some(m) {
                        self.killers[ply][1] = self.killers[ply][0].take();
                        self.killers[ply][0] = Some(m.clone());
                    }
                    if let Some(k) = g.move_key(m) {
                        let h =
                            &mut self.history[g.to_move().idx() * HIST + (k as usize & (HIST - 1))];
                        *h = (*h + depth * depth).min(1 << 20);
                    }
                }
                break;
            }
        }
        if let Some(k) = key {
            let flag = if best >= beta {
                2
            } else if best > alpha0 {
                1
            } else {
                3
            };
            let mut s = best;
            if s > MATE_BOUND {
                s += ply as i32;
            } else if s < -MATE_BOUND {
                s -= ply as i32;
            }
            self.tt[(k >> (64 - TT_BITS)) as usize] = Entry {
                key: k,
                score: s,
                mv: best_i as u16,
                depth: depth.min(100) as i8,
                flag,
            };
        }
        best
    }

    fn qsearch(&mut self, g: &G, mut alpha: i32, beta: i32, ply: usize, qd: u32) -> i32 {
        if self.tick() {
            return 0;
        }
        if let Some(v) = Self::terminal(g, ply as i32) {
            return v;
        }
        let stand = g.evaluate(g.to_move());
        if qd == 0 || stand >= beta {
            return stand;
        }
        alpha = alpha.max(stand);
        let mut moves: Vec<G::Move> = g
            .legal_moves()
            .into_iter()
            .filter(|m| g.is_tactical(m))
            .collect();
        moves.sort_by_key(|m| -g.move_score(m));
        let mut best = stand;
        for m in &moves {
            let mut c = g.clone();
            c.apply(m);
            let v = if c.to_move() == g.to_move() {
                self.qsearch(&c, alpha, beta, ply + 1, qd - 1)
            } else {
                -self.qsearch(&c, -beta, -alpha, ply + 1, qd - 1)
            };
            if self.stop {
                return 0;
            }
            best = best.max(v);
            alpha = alpha.max(best);
            if alpha >= beta {
                break;
            }
        }
        best
    }
}

/// Iterative-deepening alpha-beta limited by `ms` milliseconds and `max_depth` plies.
/// The result of the last fully searched depth is used; ties at the root are
/// broken at random (seeded).
pub fn alphabeta_time<G: Game>(g: &G, ms: u64, max_depth: u32, seed: u64) -> Option<G::Move> {
    let moves = g.legal_moves();
    if moves.is_empty() {
        return None;
    }
    if moves.len() == 1 {
        return Some(moves[0].clone());
    }
    let start = Instant::now();
    let budget = Duration::from_millis(ms);
    let use_tt = g.hash_key().is_some();
    let mut s = Searcher::<G> {
        deadline: start + budget,
        nodes: 0,
        stop: false,
        tt: if use_tt {
            vec![
                Entry {
                    key: 0,
                    score: 0,
                    mv: 0,
                    depth: 0,
                    flag: 0
                };
                1 << TT_BITS
            ]
        } else {
            Vec::new()
        },
        use_tt,
        use_q: g.quiescence(),
        killers: vec![[None, None]; MAX_PLY],
        history: vec![0; 2 * HIST],
    };
    // The first ordering may use at most the budget (d == 1 below is otherwise unbounded).
    let mut order = s.order(g, &moves, None, 0);
    let easy = level() == Level::Easy;
    let mut last_vals: Vec<(i32, usize)> = Vec::new();
    let mut best_moves: Vec<usize> = vec![order[0]];
    for d in 1..=max_depth.max(1) as i32 {
        if d >= 2 && start.elapsed() * 100 > budget * 55 {
            break;
        }
        let mut vals: Vec<(i32, usize)> = Vec::with_capacity(order.len());
        let mut alpha = i32::MIN + 2;
        let mut best = i32::MIN;
        let mut ties: Vec<usize> = Vec::new();
        for &i in &order {
            // At d == 1 the first move is never interrupted so a legal answer always
            // exists; later moves obey the deadline (huge root move lists stay in budget).
            if d == 1 && vals.is_empty() {
                s.deadline = start + Duration::from_secs(3600);
            } else {
                s.deadline = start + budget;
            }
            if d == 1 && !vals.is_empty() && Instant::now() >= s.deadline {
                s.stop = true;
                break;
            }
            let v = s.child_value(g, &moves[i], d - 1, alpha - 1, i32::MAX, 0);
            if s.stop {
                break;
            }
            vals.push((v, i));
            if v > best {
                best = v;
                ties.clear();
                ties.push(i);
                if !easy {
                    alpha = best;
                }
            } else if v == best {
                ties.push(i);
            }
        }
        if s.stop {
            // Partial depth-1 pass over a huge list: use what was evaluated.
            if d == 1 && !ties.is_empty() {
                best_moves = ties;
                last_vals = vals;
            }
            break;
        }
        best_moves = ties;
        last_vals = vals.clone();
        // Best first (stable on previous order) for the next iteration.
        vals.sort_by_key(|&(v, _)| -v);
        order = vals.into_iter().map(|(_, i)| i).collect();
        if best.abs() > MATE_BOUND {
            break;
        }
    }
    let mut rng = Rng::new(seed);
    let pick = best_moves[rng.below(best_moves.len())];
    if easy {
        let all = last_vals.iter().map(|&(v, i)| (v as f64, i)).collect();
        return easy_pick(all, Some(pick), seed).map(|i| moves[i].clone());
    }
    Some(moves[pick].clone())
}
