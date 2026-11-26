//! Piese comune pentru jocurile de levate cu doi jucători (Inimi, Pică, Prinde, Whist).
//!
//! Pachet de 52 de cărți (2..A). Informație ascunsă: mâna adversarului și cărțile
//! nedistribuite; AI-ul (alpha-beta cu adâncime 1) citește doar propria mână, informația
//! publică (levate, scoruri, licitații, atuul întors) și NUMĂRUL cărților adversarului.

use crate::cards::{full_deck, shuffle, Card};
use polytabletop_core::rng::Rng;
use polytabletop_core::Player;

/// Amestecă un pachet de 52 și dă `n` cărți fiecăruia; restul = cărți nedistribuite (ascunse).
pub fn deal(rng: &mut Rng, n: usize) -> ([Vec<Card>; 2], Vec<Card>) {
    let mut d = full_deck(2);
    shuffle(&mut d, rng);
    let mut h = [Vec::new(), Vec::new()];
    for _ in 0..n {
        h[0].push(d.pop().unwrap());
        h[1].push(d.pop().unwrap());
    }
    sort(&mut h[0]);
    sort(&mut h[1]);
    (h, d)
}

pub fn sort(h: &mut [Card]) {
    h.sort_by_key(|c| (c.suit, std::cmp::Reverse(c.rank)));
}

/// Cărțile jucabile: dacă s-a condus o culoare și o ai, e obligatoriu să răspunzi la ea.
pub fn follow(hand: &[Card], trick: &[(Player, Card)]) -> Vec<Card> {
    if let Some(&(_, l)) = trick.first() {
        if hand.iter().any(|c| c.suit == l.suit) {
            return hand.iter().copied().filter(|c| c.suit == l.suit).collect();
        }
    }
    hand.to_vec()
}

/// Cartea de răspuns bate cartea condusă?
pub fn beats(lead: Card, resp: Card, trump: Option<u8>) -> bool {
    if resp.suit == lead.suit {
        resp.rank > lead.rank
    } else {
        Some(resp.suit) == trump
    }
}

/// Probabilitate grosieră ca `c` să ia o levată (doar din rang și atu).
pub fn p_win(c: Card, trump: Option<u8>) -> f32 {
    let r = (c.rank as f32 - 2.0) / 12.0;
    if Some(c.suit) == trump {
        0.35 + 0.65 * r
    } else {
        0.6 * r * r
    }
}

pub fn hand_power(h: &[Card], trump: Option<u8>) -> f32 {
    h.iter().map(|&c| p_win(c, trump)).sum()
}

/// Puncte de figuri (A4 K3 Q2 J1).
pub fn hcp(h: &[Card]) -> i32 {
    h.iter()
        .map(|c| match c.rank {
            14 => 4,
            13 => 3,
            12 => 2,
            11 => 1,
            _ => 0,
        })
        .sum()
}

/// Pentru teste: reamestecă tot ce `viewer` nu poate vedea (mâna adversarului și cărțile
/// nedistribuite), păstrând cărțile publice (`public`) la locul lor.
pub fn scramble_hidden(
    hands: &mut [Vec<Card>; 2],
    stock: &mut Vec<Card>,
    viewer: Player,
    public: &[Card],
    seed: u64,
) {
    let opp = viewer.other().idx();
    let (mut pool, mut kept_opp, mut kept_stock) = (Vec::new(), Vec::new(), Vec::new());
    for c in hands[opp].drain(..) {
        if public.contains(&c) {
            kept_opp.push(c)
        } else {
            pool.push(c)
        }
    }
    let n_free = pool.len();
    for c in stock.drain(..) {
        if public.contains(&c) {
            kept_stock.push(c)
        } else {
            pool.push(c)
        }
    }
    shuffle(&mut pool, &mut Rng::new(seed));
    hands[opp] = kept_opp;
    hands[opp].extend(pool.drain(..n_free));
    sort(&mut hands[opp]);
    *stock = kept_stock;
    stock.extend(pool);
}
