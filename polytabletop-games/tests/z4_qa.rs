#![allow(unused_imports)]
//! z4: cross-cutting QA layer over every playable game.
//! (1) move round-trip / clone / render invariants, (2) hot-seat viewer,
//! (3) AI legality, (4) AI determinism.

use polytabletop_core::rng::Rng;
use polytabletop_core::{ai, GameObj, Player};
use polytabletop_games::{playable, HOTSEAT_SAFE};
use std::collections::BTreeMap;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{Arc, Mutex};

type Make = fn() -> Box<dyn GameObj>;
const SEEDS: u64 = 20;
const PLY_CAP: usize = 120;
/// Max legal moves round-tripped through `play` per state (all if fewer).
const RT_MOVES: usize = 8;
/// Per-game wall-clock budget of the round-trip test; slow games (huge move
/// lists) get fewer seeds/plies instead of blowing the suite runtime.
const RT_BUDGET_MS: u128 = 1500;

fn set_ai_budget() {
    ai::set_time_budget_override_ms(Some(20));
}

/// Runs `f` for every playable game on a few threads; collects the failures.
fn for_all<F>(f: F) -> Vec<String>
where
    F: Fn(&'static str, Make) -> Vec<String> + Send + Sync + 'static,
{
    set_ai_budget();
    let games: Vec<(&'static str, Make)> = playable().into_iter().map(|p| (p.id, p.make)).collect();
    let queue = Arc::new(Mutex::new(games));
    let out = Arc::new(Mutex::new(Vec::<String>::new()));
    let f = Arc::new(f);
    let n = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
        .min(16);
    let hs: Vec<_> = (0..n)
        .map(|_| {
            let (q, o, f) = (queue.clone(), out.clone(), f.clone());
            std::thread::Builder::new()
                .stack_size(64 << 20)
                .spawn(move || loop {
                    let Some((id, make)) = q.lock().unwrap().pop() else {
                        break;
                    };
                    let t0 = std::time::Instant::now();
                    let r = catch_unwind(AssertUnwindSafe(|| f(id, make)));
                    if std::env::var("Z4_TIMES").is_ok() {
                        eprintln!("TIME {id} {:?}", t0.elapsed());
                    }
                    let mut v = match r {
                        Ok(v) => v,
                        Err(_) => vec![format!("{id}: PANIC in harness")],
                    };
                    o.lock().unwrap().append(&mut v);
                })
                .unwrap()
        })
        .collect();
    for h in hs {
        h.join().unwrap();
    }
    let mut v = out.lock().unwrap().clone();
    v.sort();
    v
}

fn report(fails: Vec<String>) {
    if !fails.is_empty() {
        // keep at most 3 lines per game
        let mut per: BTreeMap<String, usize> = BTreeMap::new();
        let mut shown = Vec::new();
        for f in &fails {
            let id = f.split(':').next().unwrap_or("").to_string();
            let c = per.entry(id).or_insert(0);
            *c += 1;
            if *c <= 3 {
                shown.push(f.clone());
            }
        }
        panic!("{} failures:\n{}", fails.len(), shown.join("\n"));
    }
}

fn pick_sample(rng: &mut Rng, n: usize, k: usize) -> Vec<usize> {
    if n <= k {
        return (0..n).collect();
    }
    let mut v: Vec<usize> = (0..n).collect();
    for i in 0..k {
        let j = i + rng.below(n - i);
        v.swap(i, j);
    }
    v.truncate(k);
    v
}

fn snap(g: &dyn GameObj) -> (String, Vec<String>, Player, bool) {
    (
        g.render(),
        g.legal_moves(),
        g.to_move(),
        g.outcome().is_some(),
    )
}

// ---------------------------------------------------------------- (1)
fn check_roundtrip(id: &'static str, make: fn() -> Box<dyn GameObj>) -> Vec<String> {
    let mut fails = Vec::new();
    let t0 = std::time::Instant::now();
    for seed in 1..=SEEDS {
        let mut rng = Rng::new(seed * 7919 + 13);
        let mut g = make();
        for ply in 0..PLY_CAP {
            if g.outcome().is_some() {
                break;
            }
            if t0.elapsed().as_millis() > RT_BUDGET_MS && seed > 1 {
                return fails;
            }
            let r = g.render();
            if r.trim().is_empty() {
                fails.push(format!("{id}: seed {seed} ply {ply}: empty render"));
                return fails;
            }
            let moves = g.legal_moves();
            if moves.is_empty() {
                fails.push(format!("{id}: seed {seed} ply {ply}: no moves, not over"));
                return fails;
            }
            // clone equivalence
            let c = g.clone_box();
            if c.render() != r || c.legal_moves() != moves || c.to_move() != g.to_move() {
                fails.push(format!("{id}: seed {seed} ply {ply}: clone_box differs"));
                return fails;
            }
            // illegal texts: rejected, no panic, no mutation
            let before = snap(g.as_ref());
            let mut junk: Vec<String> = vec![
                "".into(),
                "   ".into(),
                "zzzz".into(),
                "\u{1F600}".into(),
                "-1".into(),
                "99999".into(),
                "x".repeat(300),
            ];
            let m0 = &moves[rng.below(moves.len())];
            junk.push(format!("{m0}~~"));
            junk.push(format!("~{m0}"));
            for j in &junk {
                let mut t = g.clone_box();
                let res = catch_unwind(AssertUnwindSafe(|| t.play(j)));
                match res {
                    Err(_) => {
                        fails.push(format!("{id}: seed {seed} ply {ply}: play({j:?}) panicked"));
                        return fails;
                    }
                    Ok(Ok(_)) => {
                        // accepted junk is only OK if it normalises to a legal move
                        fails.push(format!("{id}: seed {seed} ply {ply}: junk {j:?} accepted"));
                        return fails;
                    }
                    Ok(Err(_)) => {}
                }
                if snap(t.as_ref()) != before {
                    fails.push(format!("{id}: seed {seed} ply {ply}: junk {j:?} mutated"));
                    return fails;
                }
            }
            // legal moves: accepted by play(), canonical text returned
            for i in pick_sample(&mut rng, moves.len(), RT_MOVES) {
                let mut t = g.clone_box();
                match t.play(&moves[i]) {
                    Ok(txt) => {
                        let a: String = txt.split_whitespace().collect::<String>().to_lowercase();
                        let b: String = moves[i]
                            .split_whitespace()
                            .collect::<String>()
                            .to_lowercase();
                        if a != b {
                            fails.push(format!(
                                "{id}: seed {seed} ply {ply}: play returned {txt:?} for {:?}",
                                moves[i]
                            ));
                            return fails;
                        }
                    }
                    Err(e) => {
                        fails.push(format!(
                            "{id}: seed {seed} ply {ply}: legal {:?} rejected: {e}",
                            moves[i]
                        ));
                        return fails;
                    }
                }
                if t.render().trim().is_empty() {
                    fails.push(format!(
                        "{id}: seed {seed} ply {ply}: empty render after move"
                    ));
                    return fails;
                }
            }
            // untouched original
            if snap(g.as_ref()) != before {
                fails.push(format!(
                    "{id}: seed {seed} ply {ply}: state changed w/o play"
                ));
                return fails;
            }
            let pick = moves[rng.below(moves.len())].clone();
            if g.play(&pick).is_err() {
                fails.push(format!("{id}: seed {seed} ply {ply}: playout play failed"));
                return fails;
            }
        }
    }
    fails
}

#[test]
fn z4_roundtrip_clone_render() {
    report(for_all(check_roundtrip));
}

// ---------------------------------------------------------------- (2)
/// Games in HOTSEAT_SAFE whose render has no per-player private info, so
/// Some(One) == Some(Two) is expected everywhere (justification in comments).
fn no_private_hand(id: &str) -> bool {
    matches!(
        id,
        // single player vs. dealer / chance: nothing hidden from the other seat;
        // barbut = open dice duel; karuta = the hidden reading order is not per-seat.
        "blackjack" | "bingo" | "loteria" | "barbut" | "karuta" | "vinovat_tainuit"
    )
}

fn check_viewer(id: &'static str, make: fn() -> Box<dyn GameObj>) -> Vec<String> {
    let mut fails = Vec::new();
    if !HOTSEAT_SAFE.contains(&id) {
        return fails;
    }
    let mut differ = false;
    for seed in 1..=SEEDS {
        let mut rng = Rng::new(seed * 104729 + 5);
        let mut g = make();
        g.set_viewer(None);
        for ply in 0..PLY_CAP {
            if g.outcome().is_some() {
                break;
            }
            // The hot-seat hand-over banner ("Predă ecranul…") is deliberate extra
            // text for the pass-the-screen flow; everything else must be equal.
            let hot: String = g
                .render()
                .lines()
                .filter(|l| !l.contains("Predă ecranul"))
                .map(|l| format!("{l}\n"))
                .collect();
            let mut m = g.clone_box();
            m.set_viewer(Some(g.to_move()));
            let mr: String = m.render().lines().map(|l| format!("{l}\n")).collect();
            // impostor: in its Tally/Verify/Over phases hot-seat deliberately shows
            // the view of the player who acted last (the screen is not yet passed).
            let mut o = g.clone_box();
            o.set_viewer(Some(g.to_move().other()));
            let or: String = o.render().lines().map(|l| format!("{l}\n")).collect();
            if mr != hot && !(id == "impostor" && or == hot) {
                fails.push(format!(
                    "{id}: seed {seed} ply {ply}: hot-seat render != Some(mover)"
                ));
                return fails;
            }
            let mut a = g.clone_box();
            a.set_viewer(Some(Player::One));
            let mut b = g.clone_box();
            b.set_viewer(Some(Player::Two));
            if a.render() != b.render() {
                differ = true;
            }
            let moves = g.legal_moves();
            if moves.is_empty() {
                break;
            }
            let pick = moves[rng.below(moves.len())].clone();
            g.play(&pick).unwrap();
        }
    }
    if !differ && !no_private_hand(id) {
        fails.push(format!(
            "{id}: Some(One) and Some(Two) renders never differ"
        ));
    }
    fails
}

#[test]
fn z4_hotseat_viewer() {
    report(for_all(check_viewer));
}

// ---------------------------------------------------------------- (3)(4)
fn midgame_states(make: fn() -> Box<dyn GameObj>, seed: u64) -> Vec<Box<dyn GameObj>> {
    let mut rng = Rng::new(seed);
    let mut g = make();
    let mut out = Vec::new();
    let marks = [3usize, 10, 24];
    let mut ply = 0;
    while g.outcome().is_none() && ply < 40 {
        if marks.contains(&ply) {
            out.push(g.clone_box());
        }
        let moves = g.legal_moves();
        let pick = moves[rng.below(moves.len())].clone();
        g.play(&pick).unwrap();
        ply += 1;
    }
    if out.is_empty() {
        out.push(make());
    }
    out
}

fn check_ai_legal(id: &'static str, make: fn() -> Box<dyn GameObj>) -> Vec<String> {
    let mut fails = Vec::new();
    for (k, s) in midgame_states(make, 99).into_iter().enumerate() {
        let moves = s.legal_moves();
        match s.ai_move(k as u64 + 1) {
            Some(m) if moves.contains(&m) => {
                // and playable
                let mut t = s.clone_box();
                if t.play(&m).is_err() {
                    fails.push(format!("{id}: state {k}: ai move {m:?} not playable"));
                }
            }
            Some(m) => fails.push(format!("{id}: state {k}: ai move {m:?} not legal")),
            None => fails.push(format!("{id}: state {k}: ai_move returned None")),
        }
    }
    fails
}

#[test]
fn z4_ai_move_legal() {
    report(for_all(check_ai_legal));
}

/// Same state + same seed must give the same AI move. Cloned states are used
/// because `make()` seeds shuffles from the clock. Searches that are bounded by
/// wall-clock time (`AlphaBetaTime`) can legitimately differ between runs.
fn check_determinism(id: &'static str, make: fn() -> Box<dyn GameObj>) -> Vec<String> {
    let mut fails = Vec::new();
    // Wall-clock bounded iterative deepening: results depend on machine speed.
    if matches!(
        id,
        "chess"
            | "crazyhouse"
            | "halma"
            | "twixt"
            | "connect4"
            | "draughts"
            | "gomoku"
            | "renju"
            | "janggi"
            | "makruk"
            | "morris"
            | "reversi"
            | "shogi"
            | "xiangqi"
            | "fanorona"
            | "tigri"
            | "surakarta"
            | "hasami"
            | "hex"
            | "amazoane"
            | "linii"
            | "coridor"
            | "drum"
            | "tripierd"
            | "konane"
            | "puncte"
            | "strapungere"
            | "izolare"
            | "coloane"
            | "bile"
            | "patru"
            | "cupe"
            | "poliedre"
            | "rotire"
            | "armonia"
            | "asezari_noi"
            | "atlas_pierdut"
            | "bilinele"
            | "carti_cascada"
            | "cuburi"
            | "curtea_reginei"
            | "dame10"
            | "drumul_stivelor"
            | "duel_giuvaeruri"
            | "exploratori"
            | "gradina_corali"
            | "morpiuri9"
            | "oras_straturi"
            | "orasul_comun"
            | "parc_dino"
            | "pavilion_vesel"
            | "perechi_duel"
            | "pictura_manastire"
            | "piramida"
            | "poiana_ursului"
            | "poteci_munte"
            | "sat_mic"
            | "strada_case"
            | "terase"
            | "turnul_cifrelor"
            | "vanatorii_campie"
            | "hochei_masa"
            | "padurea_spiridusilor"
            | "peloton"
            | "plapuma_cal"
            | "caravana_aromelor"
            | "cursa_motoare"
            | "atelier_mecanisme"
            | "golemii_atelier"
            | "lampioane_lac"
            | "rivalii_colonizarii"
            | "ferma_animalelor"
            | "linii_urbane"
            | "cai_continentale"
            | "fronturi_mari"
            | "tratat_fragil"
            | "darurile_serii"
            | "insule_punti"
            | "pasari_sarma"
            | "debarcader"
            | "reteaua_luminii"
            | "poteca_turmelor"
            | "planete_verzi"
            | "targul_marii"
            | "colturi"
            | "sah960"
            | "impingere"
            | "roiul"
            | "cartimut"
            | "sferturi"
            | "inele"
            | "sase"
            | "margine"
            | "izvoare"
            | "trei_tipuri"
            | "animale"
            | "iepuri"
            | "asalt"
            | "dar4"
            | "culori_pas"
            | "trenuri"
            | "marmure"
            | "blocaje"
            | "zbor"
            | "discuri"
            | "cai_de_curse"
            | "cub2"
            | "flipper_duel"
            | "scuze"
            | "forme6"
            | "palate"
            | "sir5"
            | "kilometri"
            | "camile"
            | "tren_jaf"
            | "arbori_poiana"
            | "alquerque"
            | "camelot"
            | "chaturanga"
            | "foxgeese"
            | "latrunculi"
            | "jungle"
            | "fotbalmasa"
            | "tiddlywinks"
            | "pente"
            | "hnefatafl"
            | "brandubh"
            | "chinese_checkers"
            | "rithmomachia"
    ) {
        return fails;
    }
    for (k, s) in midgame_states(make, 7).into_iter().enumerate() {
        for seed in [1u64, 2] {
            let a = s.clone_box().ai_move(seed);
            let b = s.clone_box().ai_move(seed);
            if a != b {
                fails.push(format!("{id}: state {k} seed {seed}: {a:?} vs {b:?}"));
                break;
            }
        }
    }
    fails
}

#[test]
fn z4_ai_determinism() {
    report(for_all(check_determinism));
}
