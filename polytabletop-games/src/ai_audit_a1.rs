//! Ajutoare de test pentru auditul „AI-ul nu vede informația ascunsă a adversarului” (lotul A1).
//! Doar în teste: rulează o verificare la toate nivelurile de dificultate (Easy/Normal/Hard/Expert).
#![cfg(test)]

use polytabletop_core::ai::{self, Level};
use polytabletop_core::rng::Rng;
use polytabletop_core::Game;
use std::cell::Cell;
use std::sync::Mutex;

static LEVELS: Mutex<()> = Mutex::new(());

thread_local! {
    static DELEGATED: Cell<bool> = const { Cell::new(false) };
}

/// Nivelul AI-ului e global (un atomic pe tot procesul): schimbarea lui într-un test ar strica
/// testele vecine care rulează în paralel și compară două apeluri ale AI-ului. De aceea, când testul
/// rulează pe propriul fir (harness paralel), el se re-lansează o singură dată într-un proces copil
/// cu un singur test (`--exact`, un fir); doar acolo se schimbă nivelul. Întoarce `true` dacă
/// verificarea a fost deja făcută de procesul copil.
fn delegated_to_child() -> bool {
    if std::env::var_os("A1_AUDIT_CHILD").is_some() {
        return false;
    }
    let name = match std::thread::current().name() {
        Some(n) if n != "main" => n.to_owned(),
        _ => return false,
    };
    if DELEGATED.with(|d| d.replace(true)) {
        return true;
    }
    let Ok(exe) = std::env::current_exe() else {
        DELEGATED.with(|d| d.set(false));
        return false;
    };
    let out = std::process::Command::new(exe)
        .args([name.as_str(), "--exact", "--test-threads=1", "--nocapture"])
        .env("A1_AUDIT_CHILD", "1")
        .output()
        .expect("nu s-a putut lansa procesul copil al testului");
    let text = format!(
        "{}\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        out.status.success() && text.contains("1 passed"),
        "testul {name} a eșuat în procesul copil:\n{text}"
    );
    true
}

/// Rulează `f` pentru fiecare nivel (nivelul e global: vezi `delegated_to_child`), apoi revine la
/// Normal.
pub fn all_levels(mut f: impl FnMut(Level)) {
    if delegated_to_child() {
        return;
    }
    let _g = LEVELS.lock().unwrap_or_else(|e| e.into_inner());
    for lv in [Level::Easy, Level::Normal, Level::Hard, Level::Expert] {
        ai::set_level(lv);
        f(lv);
    }
    ai::set_level(Level::Normal);
}

/// Amestecă determinist o felie.
pub fn shuffle<T>(v: &mut [T], seed: u64) {
    let mut r = Rng::new(seed ^ 0xA1A1_5EED);
    for i in (1..v.len()).rev() {
        v.swap(i, r.below(i + 1));
    }
}

/// Alege aceeași mutare `g` și `h` (stări care diferă doar prin informația ascunsă), la toate
/// nivelurile și pentru mai multe semințe ale AI-ului.
pub fn assert_same_choice<G: Game>(name: &str, g: &G, h: &G, ai_seeds: &[u64]) {
    all_levels(|lv| {
        for &seed in ai_seeds {
            let a = ai::best_move(g, seed).map(|m| g.format_move(&m));
            let b = ai::best_move(h, seed).map(|m| h.format_move(&m));
            assert_eq!(
                a, b,
                "{name}: AI-ul depinde de informația ascunsă (nivel {lv:?}, sămânță {seed})"
            );
        }
    });
}

/// Joacă mutări aleatoare din `mk(seed)`; la fiecare `step` plies (până la `max`) construiește cu
/// `scramble` o stare alternativă și compară alegerile AI-ului.
pub fn audit<G: Game>(
    name: &str,
    mk: impl Fn(u64) -> G,
    seeds: std::ops::RangeInclusive<u64>,
    step: usize,
    max: usize,
    scramble: impl Fn(&mut G, u64),
) {
    audit_seeds(name, mk, seeds, step, max, scramble, &[7, 12345]);
}

/// Ca `audit`, cu semințele AI-ului alese de apelant (pentru jocurile cu o evaluare costisitoare).
pub fn audit_seeds<G: Game>(
    name: &str,
    mk: impl Fn(u64) -> G,
    seeds: std::ops::RangeInclusive<u64>,
    step: usize,
    max: usize,
    scramble: impl Fn(&mut G, u64),
    ai_seeds: &[u64],
) {
    let mut checked = 0;
    for seed in seeds {
        let mut g = mk(seed);
        let mut rng = Rng::new(seed.wrapping_mul(977) ^ 0x5151);
        for ply in 0..max {
            if g.outcome().is_some() {
                break;
            }
            if ply % step == 0 {
                let mut h = g.clone();
                scramble(&mut h, seed * 1000 + ply as u64);
                assert_same_choice(name, &g, &h, ai_seeds);
                checked += 1;
            }
            let m = g.legal_moves();
            let m = m[rng.below(m.len())].clone();
            g.apply(&m);
        }
    }
    assert!(checked > 0, "{name}: nicio stare verificată");
}

/// Amestecă `hand` cu `stock` și le redistribuie păstrând mărimile (informație necunoscută jucătorului).
pub fn scramble_pool<T: Clone>(hand: &mut Vec<T>, stock: &mut Vec<T>, salt: u64) {
    let n = hand.len();
    let mut pool: Vec<T> = std::mem::take(hand);
    pool.append(stock);
    shuffle(&mut pool, salt);
    *stock = pool.split_off(n);
    *hand = pool;
}

/// Ca `audit`, pentru jocurile cu `Honest`: starea alternativă e o determinizare din punctul de
/// vedere al celui la mutare (alt pachet / altă mână adversă compatibile cu ce vede).
pub fn audit_honest<G: crate::p3_comun::Honest>(
    name: &str,
    mk: impl Fn(u64) -> G,
    seeds: std::ops::RangeInclusive<u64>,
    step: usize,
    max: usize,
) {
    audit_seeds(
        name,
        mk,
        seeds,
        step,
        max,
        |g, salt| {
            let me = Game::to_move(g);
            g.forget_fresh();
            *g = g.determinize(me, &mut Rng::new(salt));
        },
        &[7],
    );
}

/// `n` cărți „necunoscute”: alese la întâmplare din `all` fără cele din `exclude`.
pub fn fake_cards<T: Clone + PartialEq>(all: Vec<T>, exclude: &[T], n: usize, salt: u64) -> Vec<T> {
    let mut pool: Vec<T> = all.into_iter().filter(|c| !exclude.contains(c)).collect();
    shuffle(&mut pool, salt);
    pool.truncate(n);
    pool
}
