//! Helperi comuni pentru jocurile din valul P3 (Pinochle, Gin Rummy, Crazy Eights, Pitch, Bid Whist,
//! All Fours, Conquián, Chicken Foot, Bunco, Farkle, Liar's Dice, Crokinole).
//!
//! # AI cinstit pentru jocurile cu informație ascunsă
//!
//! Toate jocurile cu mâini ascunse folosesc `AiStyle::AlphaBeta(0)`: regula din `ai.rs` face din
//! adâncimea 0 o căutare de un singur nivel la *toate* nivelurile (Easy/Normal/Hard/Expert), deci AI-ul
//! nu extinde niciodată mutările adversarului din mâna lui reală. Valoarea unei mutări vine din
//! [`honest_eval`]: starea rezultată e **determinizată** (tot ce nu vede jucătorul `p` e amestecat din
//! cărțile nevăzute), apoi se joacă până la capătul mâinii cu o politică euristică. Nivelul scalează
//! numărul de simulări (`ai::level()`): Easy foarte puține, Expert multe. Mutările care trag din stoc
//! marchează cărțile trase în `fresh`, iar determinizarea le re-eșantionează (AI-ul nu vede vârful stocului).

use polytabletop_core::ai::level;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Game, Player};

/// Hash FNV-1a pe 64 de biți (sămânță derivată doar din informația vizibilă).
pub fn fnv(s: &str) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in s.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

/// Joc cu informație ascunsă, evaluabil prin determinizare.
pub trait Honest: Game {
    /// Copie în care tot ce `me` nu poate vedea este re-eșantionat.
    fn determinize(&self, me: Player, rng: &mut Rng) -> Self;
    /// Uită marcajul „cărți proaspăt trase” (stare reală, nu simulată): folosit de teste ca să
    /// construiască o lume alternativă fără a re-eșantiona mâna proprie.
    fn forget_fresh(&mut self) {}
    /// Starea „așa cum o vede `me`”, fără cărțile proaspăt trase în mutarea simulată (din ea se
    /// derivă sămânța simulărilor, ca să nu depindă de vârful stocului).
    fn seed_state(&self, _me: Player) -> Self {
        self.clone()
    }
    /// Adevărat când simulările trebuie oprite (mână terminată sau joc terminat).
    fn rollout_done(&self) -> bool;
    /// Valoarea stării (din punctul de vedere al lui `p`) la capătul simulării.
    fn rollout_value(&self, p: Player) -> f64;
    /// Politica din simulări (implicit: aleator).
    fn policy(&self, rng: &mut Rng) -> Self::Move {
        let m = self.legal_moves();
        m[rng.below(m.len())].clone()
    }
}

/// Sămânță derivată doar din ce vede `p`: randarea cu privitorul fixat pe `p`.
pub fn view_seed<G: Game>(g: &G, p: Player) -> u64 {
    let mut c = g.clone();
    c.set_viewer(Some(p));
    fnv(&c.render())
}

/// Evaluare prin determinizare: media valorilor simulărilor, în sutimi.
/// `base` simulări la nivelul Normal (scalate cu nivelul AI).
pub fn honest_eval<G: Honest>(g: &G, p: Player, base: u64) -> i32 {
    let n = (base * level().factor_x10() / 10).clamp(1, 600) as usize;
    let mut rng = Rng::new(view_seed(&g.seed_state(p), p));
    let mut total = 0.0f64;
    for _ in 0..n {
        let mut s = g.determinize(p, &mut rng);
        let mut guard = 0;
        while !s.rollout_done() && guard < 1500 {
            let m = s.policy(&mut rng);
            s.apply(&m);
            guard += 1;
        }
        total += s.rollout_value(p);
    }
    (total / n as f64 * 100.0) as i32
}

/// Nivelul curent ca multiplicator în zecimi (1 = Easy … 60 = Expert).
pub fn lvl() -> u64 {
    level().factor_x10()
}

/// Test de onestitate (audit AI): pentru mai multe semințe ale AI-ului, mutarea AI-ului trebuie să rămână
/// identică după ce `scramble` re-amestecă informația ascunsă a adversarului (mână, stoc, rol, țintă
/// secretă, zaruri viitoare). `scramble(stare, la_mutare, sare)`. Se verifică `ai_move` la nivelul curent
/// și căutarea cu adâncimile pe care le folosesc nivelurile Easy/Normal/Hard/Expert (fără a schimba
/// nivelul global, ca testele care rulează în paralel să nu se influențeze): un `AlphaBeta(1)` ar
/// extinde la Hard/Expert răspunsurile adversarului din mâna lui reală și ar pica aici.
/// `checks` = câte poziții se verifică per partidă; partidele se joacă cu mutări aleatoare.
#[cfg(test)]
pub fn ai_blind_check<G: Game + 'static>(
    name: &str,
    mk: impl Fn(u64) -> G,
    scramble: impl Fn(&mut G, Player, u64),
    checks: usize,
) {
    ai_blind_check_with(name, mk, scramble, checks, 3, Term::Check);
}

/// Adâncimile căutării `AlphaBeta(d)` la Easy, Normal, Hard, Expert (oglinda lui `ai::best_move`).
#[cfg(test)]
fn level_depths(d: u32) -> [u32; 4] {
    [
        d.saturating_sub(1).max(1),
        d,
        if d <= 4 { d + 1 } else { d },
        if d <= 6 { d + 1 } else { d },
    ]
}

/// Cum se tratează mutările care încheie partida în `ai_blind_check_with`.
#[cfg(test)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Term {
    /// Sfârșitul partidei trebuie să fie același în lumea amestecată.
    Check,
    /// Se sar pozițiile din care o mutare poate încheia partida (rezultatul real e vizibil căutării doar
    /// prin `Game::outcome`).
    Skip,
    /// Sfârșitul partidei poate depinde de informația ascunsă, dar nu schimbă decizia (ex.: «strig» e
    /// mereu cea mai bună mutare când ai linie); nu se compară.
    Ignore,
}

/// Ca [`ai_blind_check`], cu `stride` (se verifică pozițiile cu `pas % stride == 1`, sau toate dacă
/// `stride <= 1`) și tratamentul mutărilor terminale (`term`).
#[cfg(test)]
pub fn ai_blind_check_with<G: Game + 'static>(
    name: &str,
    mk: impl Fn(u64) -> G,
    scramble: impl Fn(&mut G, Player, u64),
    checks: usize,
    stride: usize,
    term: Term,
) {
    use polytabletop_core::ai::alphabeta_root;
    use polytabletop_core::{AiStyle, GameObj};
    for seed in 1..=3u64 {
        let mut g = mk(seed);
        let depths: Vec<u32> = match Game::ai_style(&g) {
            AiStyle::AlphaBeta(d) => level_depths(d).to_vec(),
            _ => Vec::new(),
        };
        let mut rng = Rng::new(seed * 77 + 5);
        let mut done = 0;
        for step in 0..600usize {
            if Game::outcome(&g).is_some() || done >= checks {
                break;
            }
            let legal = Game::legal_moves(&g);
            if legal.is_empty() {
                break;
            }
            let due = stride <= 1 || step % stride == 1;
            let ends = term == Term::Skip
                && legal.iter().any(|m| {
                    let mut c = g.clone();
                    Game::apply(&mut c, m);
                    Game::outcome(&c).is_some()
                });
            if legal.len() > 1 && due && !ends {
                let me = Game::to_move(&g);
                // valoarea fiecărei mutări (copilul evaluat la rădăcină) nu trebuie să depindă de ce nu se vede:
                // prinde și scurgerile care nu schimbă (încă) cea mai bună mutare, inclusiv ordinea din Easy
                let mut w = g.clone();
                scramble(&mut w, me, step as u64 * 7 + 11);
                for m in &legal {
                    let (mut cg, mut cw) = (g.clone(), w.clone());
                    Game::apply(&mut cg, m);
                    Game::apply(&mut cw, m);
                    let (og, ow) = (Game::outcome(&cg).is_some(), Game::outcome(&cw).is_some());
                    if term != Term::Ignore {
                        assert_eq!(
                            og, ow,
                            "{name} (seed {seed}, pas {step}): sfârșitul partidei după {m:?} depinde de informația ascunsă"
                        );
                    }
                    if !og && !ow {
                        assert_eq!(
                            Game::evaluate(&cg, me),
                            Game::evaluate(&cw, me),
                            "{name} (seed {seed}, pas {step}): evaluarea după {m:?} depinde de informația ascunsă"
                        );
                    }
                }
                for ai_seed in [3u64, 99, 4242] {
                    let mut h = g.clone();
                    scramble(&mut h, me, step as u64 * 13 + ai_seed + 1);
                    assert_eq!(
                        GameObj::ai_move(&g, ai_seed),
                        GameObj::ai_move(&h, ai_seed),
                        "{name} (seed {seed}, pas {step}): AI-ul depinde de informația ascunsă a adversarului"
                    );
                    for &d in &depths {
                        let (a, b) = (
                            alphabeta_root(&g, d, ai_seed),
                            alphabeta_root(&h, d, ai_seed),
                        );
                        assert!(
                            a == b,
                            "{name} (seed {seed}, pas {step}): AI-ul (adâncime {d}) depinde de informația ascunsă: {a:?} != {b:?}"
                        );
                    }
                }
                done += 1;
            }
            let m = legal[rng.below(legal.len())].clone();
            Game::apply(&mut g, &m);
        }
        assert!(done > 0, "{name}: nicio poziție verificată");
    }
}

/// Test de onestitate pentru `Game::ai_view`: două lumi pe care jucătorul la mutare nu le poate deosebi
/// (`scramble` re-amestecă informația ascunsă) trebuie să dea aceeași vedere pentru AI, deci aceleași
/// rezultate terminale, aceeași evaluare după fiecare mutare și aceeași mutare aleasă, *inclusiv* la
/// mutările care încheie partida (spre deosebire de `ai_blind_check`, care citește starea reală).
#[cfg(test)]
pub fn ai_view_check<G: Game + 'static>(
    name: &str,
    mk: impl Fn(u64) -> G,
    scramble: impl Fn(&mut G, Player, u64),
    checks: usize,
) {
    use polytabletop_core::ai::alphabeta_root;
    use polytabletop_core::{AiStyle, GameObj};
    for seed in 1..=3u64 {
        let mut g = mk(seed);
        let depths: Vec<u32> = match Game::ai_style(&g) {
            AiStyle::AlphaBeta(d) => level_depths(d).to_vec(),
            _ => Vec::new(),
        };
        let mut rng = Rng::new(seed * 131 + 7);
        let mut done = 0;
        for step in 0..800usize {
            if Game::outcome(&g).is_some() || done >= checks {
                break;
            }
            let legal = Game::legal_moves(&g);
            if legal.is_empty() {
                break;
            }
            if legal.len() > 1 {
                let me = Game::to_move(&g);
                let mut h = g.clone();
                scramble(&mut h, me, step as u64 * 7 + 11);
                let (vg, vh) = (Game::ai_view(&g, me), Game::ai_view(&h, me));
                assert_eq!(
                    Game::legal_moves(&vg).len(),
                    legal.len(),
                    "{name} (seed {seed}, pas {step}): vederea schimbă mutările legale"
                );
                for m in &legal {
                    let (mut a, mut b) = (vg.clone(), vh.clone());
                    Game::apply(&mut a, m);
                    Game::apply(&mut b, m);
                    assert_eq!(
                        Game::outcome(&a),
                        Game::outcome(&b),
                        "{name} (seed {seed}, pas {step}): rezultatul după {m:?} depinde de informația ascunsă"
                    );
                    if Game::outcome(&a).is_none() {
                        assert_eq!(
                            Game::evaluate(&a, me),
                            Game::evaluate(&b, me),
                            "{name} (seed {seed}, pas {step}): evaluarea după {m:?} depinde de informația ascunsă"
                        );
                    }
                }
                for ai_seed in [3u64, 99] {
                    assert_eq!(
                        GameObj::ai_move(&g, ai_seed),
                        GameObj::ai_move(&h, ai_seed),
                        "{name} (seed {seed}, pas {step}): AI-ul depinde de informația ascunsă"
                    );
                    for &d in &depths {
                        let (x, y) = (
                            alphabeta_root(&vg, d, ai_seed),
                            alphabeta_root(&vh, d, ai_seed),
                        );
                        assert!(
                            x == y,
                            "{name} (seed {seed}, pas {step}): adâncimea {d}: {x:?} != {y:?}"
                        );
                    }
                }
                done += 1;
            }
            let m = legal[rng.below(legal.len())].clone();
            Game::apply(&mut g, &m);
        }
        assert!(done > 0, "{name}: nicio poziție verificată");
    }
}

/// Amestecă (determinist, din `seed`) o listă: ajutor pentru funcțiile `scramble` din teste.
#[cfg(test)]
pub fn shuffle_with<T>(v: &mut [T], seed: u64) {
    let mut rng = Rng::new(seed ^ 0xA3A3_5EED);
    for i in (1..v.len()).rev() {
        let j = rng.below(i + 1);
        v.swap(i, j);
    }
}

/// Scramble generic pentru jocurile `Honest`: lume alternativă compatibilă cu ce vede jucătorul `me`.
#[cfg(test)]
pub fn honest_scramble<G: Honest>(g: &mut G, me: Player, seed: u64) {
    g.forget_fresh();
    let mut rng = Rng::new(seed);
    *g = g.determinize(me, &mut rng);
}

/// Amestecă (pentru teste) mâna ascunsă și stocul: primele `keep_bottom` cărți din stoc (ex. atuul arătat)
/// rămân la locul lor; restul și mâna se redistribuie la întâmplare, păstrând mărimile.
#[cfg(test)]
pub fn mix_unseen<T: Clone>(hand: &mut Vec<T>, stock: &mut Vec<T>, keep_bottom: usize, seed: u64) {
    let n = hand.len();
    let keep = keep_bottom.min(stock.len());
    let bottom: Vec<T> = stock[..keep].to_vec();
    let mut pool: Vec<T> = hand.clone();
    pool.extend(stock[keep..].iter().cloned());
    shuffle_with(&mut pool, seed);
    *hand = pool.drain(..n).collect();
    *stock = bottom;
    stock.extend(pool);
}
