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
