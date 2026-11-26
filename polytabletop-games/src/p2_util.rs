#![allow(
    clippy::needless_range_loop,
    clippy::manual_is_multiple_of,
    clippy::manual_range_contains,
    clippy::too_many_arguments,
    clippy::doc_lazy_continuation,
    clippy::unnecessary_unwrap,
    clippy::unnecessary_cast,
    clippy::type_complexity
)]
//! Piese comune pentru jocurile tradiționale din lotul P2 (cărți, zaruri).
//!
//! * `mc_value`: evaluare Monte-Carlo cu *determinizare* — completările de joc se trag doar din
//!   cărțile pe care jucătorul evaluat nu le poate vedea (mâna adversarului + cărțile nefolosite
//!   se reamestecă), deci AI-ul nu trișează. Nivelul (Easy/Normal/Hard/Expert) alege numărul de
//!   eșantioane.
//! * `noise`: zgomot determinist pentru nivelul Easy.

use polytabletop_core::ai::{level, Level};
use polytabletop_core::rng::Rng;
use polytabletop_core::{hash_mix, Game, Outcome, Player};

/// Câte eșantioane Monte-Carlo face `evaluate` la nivelul curent.
pub fn mc_samples() -> u32 {
    match level() {
        Level::Easy | Level::Normal => 0,
        Level::Hard => 4,
        Level::Expert => 12,
    }
}

/// Zgomot determinist pentru nivelul Easy (0 la celelalte niveluri), în `-amp..=amp`.
pub fn noise(sig: u64, amp: i32) -> i32 {
    if level() != Level::Easy || amp <= 0 {
        return 0;
    }
    (hash_mix(sig ^ 0xEA51) % (2 * amp as u64 + 1)) as i32 - amp
}

/// Semnătură rapidă a unei liste de numere (pentru semințe deterministe).
pub fn sig_of(parts: &[u64]) -> u64 {
    let mut h = 0x1234_5678_9ABC_DEF0u64;
    for &p in parts {
        h = hash_mix(h ^ p.wrapping_mul(0x9E37_79B9_7F4A_7C15));
    }
    h
}

/// Joc cu informație ascunsă care știe să-și „determinizeze” starea din punctul de vedere al
/// unui jucător.
pub trait Mc: Game {
    /// O stare completă compatibilă cu ce vede `viewer` (restul se reamestecă).
    fn determinize(&self, viewer: Player, rng: &mut Rng) -> Self;
    /// Crește când se termină o mână / o împărțire (playout-ul se oprește atunci).
    fn epoch(&self) -> u32;
    /// Diferența cumulată de scor (`p` minus adversarul).
    fn margin(&self, p: Player) -> i32;
}

/// Media (în unități de `margin`, ×100) a câștigului lui `p` până la sfârșitul mâinii curente,
/// cu mutări aleatoare și informație determinizată.
pub fn mc_value<G: Mc>(g: &G, p: Player, samples: u32, seed: u64) -> i32 {
    if samples == 0 {
        return 0;
    }
    let mut rng = Rng::new(seed);
    let mut total = 0i64;
    let base = g.margin(p);
    let ep = g.epoch();
    for _ in 0..samples {
        let mut d = g.determinize(p, &mut rng);
        let mut n = 0;
        while d.outcome().is_none() && d.epoch() == ep && n < 600 {
            let Some(m) = d.playout_move(&mut rng) else {
                break;
            };
            d.apply(&m);
            n += 1;
        }
        let v = match d.outcome() {
            Some(Outcome::Win(w)) if w == p => 1000,
            Some(Outcome::Win(_)) => -1000,
            Some(Outcome::Draw) => 0,
            None => 0,
        };
        total += (d.margin(p) - base) as i64 * 100 + v;
    }
    (total / samples as i64) as i32
}

/// Amestecă un `Vec` în loc (Fisher–Yates).
pub fn shuffle_vec<T>(v: &mut [T], rng: &mut Rng) {
    for i in (1..v.len()).rev() {
        let j = rng.below(i + 1);
        v.swap(i, j);
    }
}

/// Ordinea de afișare a unei mâini de cărți standard (culoare, apoi rang descrescător).
pub fn sort_std(h: &mut [u8]) {
    h.sort_by_key(|&c| (c / 13, std::cmp::Reverse(c % 13)));
}

/// În doi jucători: răspunsul `resp` bate cartea condusă `lead`?
pub fn beats2(lead: u8, resp: u8, trump: Option<u8>) -> bool {
    let (ls, rs) = (lead / 13, resp / 13);
    if ls == rs {
        resp % 13 > lead % 13
    } else {
        Some(rs) == trump
    }
}

/// Cărțile permise: răspunzi la culoarea condusă dacă o ai.
pub fn follow_cards(hand: &[u8], led: Option<u8>) -> Vec<u8> {
    if let Some(l) = led {
        let f: Vec<u8> = hand.iter().copied().filter(|&c| c / 13 == l / 13).collect();
        if !f.is_empty() {
            return f;
        }
    }
    hand.to_vec()
}

/// "7♥ K♠" cu simboluri.
pub fn symbols(cards: &[u8]) -> String {
    crate::cards52::join_symbols(cards, crate::cards52::std_symbol)
}

/// "7♥[7h] K♠[Ks]" — simbol + text de tastat.
pub fn symbols_typed(cards: &[u8]) -> String {
    crate::cards52::join_symbols(cards, |c| {
        format!(
            "{}[{}]",
            crate::cards52::std_symbol(c),
            crate::cards52::std_text(c)
        )
    })
}
