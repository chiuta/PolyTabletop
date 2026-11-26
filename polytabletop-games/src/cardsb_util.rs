//! Helperi comuni pentru jocurile de cărți din valul B (Bridge, Euchre, Tarneeb, …).

use crate::cards::Card;

/// Indexul cărții câștigătoare dintr-o levată (`cards` în ordinea jucată).
/// `eff` dă culoarea efectivă, `power` dă puterea în cadrul culorii (atu bate tot).
pub fn trick_winner(
    cards: &[Card],
    trump: Option<u8>,
    eff: &dyn Fn(Card) -> u8,
    power: &dyn Fn(Card) -> u32,
) -> usize {
    let led = eff(cards[0]);
    let mut best = 0usize;
    for i in 1..cards.len() {
        let (bs, cs) = (eff(cards[best]), eff(cards[i]));
        let beats = if Some(cs) == trump && Some(bs) != trump {
            true
        } else if cs == bs {
            power(cards[i]) > power(cards[best])
        } else {
            false
        };
        if beats && (cs == led || Some(cs) == trump) {
            best = i;
        }
    }
    best
}

/// Sortează o mână: pe culori, apoi valoare descrescătoare.
pub fn sort_hand(h: &mut [Card]) {
    h.sort_by_key(|c| (c.suit, std::cmp::Reverse(c.rank)));
}

pub fn has_suit(h: &[Card], suit: u8, eff: &dyn Fn(Card) -> u8) -> bool {
    h.iter().any(|&c| eff(c) == suit)
}

/// Numele culorii în română, cu litera de tastat.
pub fn suit_name(s: u8) -> &'static str {
    ["cupă♥", "caro♦", "pică♠", "treflă♣"][s as usize]
}

pub fn suit_from_letter(c: char) -> Option<u8> {
    match c {
        'h' => Some(0),
        'd' => Some(1),
        's' => Some(2),
        'c' => Some(3),
        _ => None,
    }
}
