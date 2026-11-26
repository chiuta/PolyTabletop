//! Mariáš (Cehia / Slovacia) — variantă pentru doi jucători, din regulile publice ale mariášului.
//!
//! Pachet de 32 de cărți (7 8 9 U O K 10 A; U = spodek, O = svršek; culori g ghinde, f frunze, i inimi, c clopote).
//! Ordinea (de la cea mai mare): A, 10, K, O, U, 9, 8, 7. Valorează câte 10 puncte ași și zecile („honzy”); ultima levată +10.
//! * Împărțire: 10 cărți fiecăruia și 2 în talon (adaptare pentru doi; cele 10 cărți rămase nu se joacă). Forhont (cel care
//!   nu a împărțit) ia talonul, lasă două cărți (`las 7g`; punctele lor sunt ale lui) și alege atuul (`atu inimi`).
//! * Joc: 10 levate, forhontul conduce. Răspunzi la culoare și **bați** dacă poți; fără culoare joci **atu** dacă ai (și
//!   bați dacă poți); altfel orice.
//! * Hlášky: conducând K sau O cu perechea în mână (`Kg hlaska`) încasezi 20, iar în atu 40.
//! * Mâna: forhontul câștigă dacă are mai multe puncte (honzy + hlášky + ultima levată + cărți lăsate) decât adversarul;
//!   câștigătorul ia 1 punct. 6 mâini; mai multe puncte câștigă (egal = remiză).
//!
//! Omise: betl, durch, sedma, flekuri. Informație ascunsă: mâinile și talonul.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const RANKS: [&str; 8] = ["7", "8", "9", "U", "O", "K", "10", "A"];
const SUITS: [char; 4] = ['g', 'f', 'i', 'c'];
const SUIT_NAMES: [&str; 4] = ["ghinde", "frunze", "inimi", "clopote"];
const HANDS: u8 = 6;

fn suit(c: u8) -> u8 {
    c / 8
}
fn rank(c: u8) -> u8 {
    c % 8
}
pub fn points(c: u8) -> i32 {
    if rank(c) >= 6 {
        10
    } else {
        0
    }
}
pub fn text(c: u8) -> String {
    format!("{}{}", RANKS[rank(c) as usize], SUITS[suit(c) as usize])
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mv {
    Discard(u8),
    Trump(u8),
    Play(u8),
    Marry(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Discard,
    Declare,
    Play,
    Over,
}

#[derive(Clone)]
pub struct Mariash {
    viewer: Option<Player>,
    hands: [Vec<u8>; 2],
    talon: Vec<u8>,
    discards: Vec<u8>,
    trump: u8,
    phase: Phase,
    trick: Vec<(usize, u8)>,
    leader: usize,
    taken: [i32; 2],
    marriage: [i32; 2],
    forhont: usize,
    hand_no: u8,
    scores: [i32; 2],
    rng: Rng,
    log: String,
}

impl Default for Mariash {
    fn default() -> Self {
        Self::new()
    }
}

impl Mariash {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut g = Mariash {
            viewer: None,
            hands: [Vec::new(), Vec::new()],
            talon: Vec::new(),
            discards: Vec::new(),
            trump: 0,
            phase: Phase::Over,
            trick: Vec::new(),
            leader: 0,
            taken: [0; 2],
            marriage: [0; 2],
            forhont: 1,
            hand_no: 0,
            scores: [0; 2],
            rng: Rng::new(seed),
            log: String::new(),
        };
        g.deal();
        g
    }
    pub fn scores(&self) -> (i32, i32) {
        (self.scores[0], self.scores[1])
    }
    fn deal(&mut self) {
        self.hand_no += 1;
        self.forhont = 1 - self.forhont;
        let mut d: Vec<u8> = (0..32).collect();
        for i in (1..32).rev() {
            let j = self.rng.below(i + 1);
            d.swap(i, j);
        }
        let f = self.forhont;
        let mut hands = [Vec::new(), Vec::new()];
        hands[f] = d[0..12].to_vec();
        hands[1 - f] = d[12..22].to_vec();
        self.hands = hands;
        self.talon.clear();
        self.discards.clear();
        self.phase = Phase::Discard;
        self.trick.clear();
        self.taken = [0; 2];
        self.marriage = [0; 2];
        self.leader = f;
        self.log = format!(
            "Mâna {}/{HANDS}: forhont e J{} (12 cărți, lasă două).",
            self.hand_no,
            f + 1
        );
    }
    fn actor(&self) -> usize {
        match self.phase {
            Phase::Play => (self.leader + self.trick.len()) % 2,
            _ => self.forhont,
        }
    }
    fn legal_cards(&self, p: usize) -> Vec<u8> {
        let h = &self.hands[p];
        let Some(&(_, c0)) = self.trick.first() else {
            return h.clone();
        };
        let best = self.trick_best();
        let same: Vec<u8> = h.iter().copied().filter(|&c| suit(c) == suit(c0)).collect();
        if !same.is_empty() {
            // trebuie să bați dacă poți
            let over: Vec<u8> = same
                .iter()
                .copied()
                .filter(|&c| self.beats(c, best))
                .collect();
            return if over.is_empty() { same } else { over };
        }
        let tr: Vec<u8> = h
            .iter()
            .copied()
            .filter(|&c| suit(c) == self.trump)
            .collect();
        if !tr.is_empty() {
            let over: Vec<u8> = tr
                .iter()
                .copied()
                .filter(|&c| self.beats(c, best))
                .collect();
            return if over.is_empty() { tr } else { over };
        }
        h.clone()
    }
    fn trick_best(&self) -> u8 {
        let mut best = self.trick[0].1;
        for &(_, c) in &self.trick[1..] {
            if self.beats(c, best) {
                best = c;
            }
        }
        best
    }
    fn beats(&self, a: u8, b: u8) -> bool {
        let (ta, tb) = (suit(a) == self.trump, suit(b) == self.trump);
        if ta != tb {
            return ta;
        }
        suit(a) == suit(b) && rank(a) > rank(b)
    }
    fn can_marry(&self, p: usize, c: u8) -> bool {
        self.trick.is_empty()
            && (rank(c) == 4 || rank(c) == 5)
            && self.hands[p].contains(&(suit(c) * 8 + if rank(c) == 4 { 5 } else { 4 }))
    }
    fn potential(&self, p: usize) -> i32 {
        self.hands[p]
            .iter()
            .map(|&c| rank(c) as i32 + if suit(c) == self.trump { 6 } else { 0 })
            .sum()
    }
    fn finish_hand(&mut self, last_winner: usize) {
        self.taken[last_winner] += 10;
        let f = self.forhont;
        let disc: i32 = self.discards.iter().map(|&c| points(c)).sum();
        let tf = self.taken[f] + self.marriage[f] + disc;
        let to = self.taken[1 - f] + self.marriage[1 - f];
        let w = if tf > to { f } else { 1 - f };
        self.scores[w] += 1;
        self.log = format!(
            "Forhont J{} are {tf}, adversarul {to}: punct pentru J{}.",
            f + 1,
            w + 1
        );
        if self.hand_no >= HANDS {
            self.phase = Phase::Over;
        } else {
            let keep = self.log.clone();
            self.deal();
            self.log = format!("{keep} {}", self.log);
        }
    }
}

impl Game for Mariash {
    type Move = Mv;
    fn to_move(&self) -> Player {
        if self.actor() == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn legal_moves(&self) -> Vec<Mv> {
        let p = self.actor();
        match self.phase {
            Phase::Over => Vec::new(),
            Phase::Discard => self.hands[p].iter().map(|&c| Mv::Discard(c)).collect(),
            Phase::Declare => (0..4u8).map(Mv::Trump).collect(),
            Phase::Play => {
                let mut v = Vec::new();
                for c in self.legal_cards(p) {
                    v.push(Mv::Play(c));
                    if self.can_marry(p, c) {
                        v.push(Mv::Marry(c));
                    }
                }
                v
            }
        }
    }
    fn apply(&mut self, m: &Mv) {
        let p = self.actor();
        match (*m, self.phase) {
            (Mv::Discard(c), Phase::Discard) => {
                let h = &mut self.hands[p];
                let i = h.iter().position(|&x| x == c).unwrap();
                h.remove(i);
                self.discards.push(c);
                if self.discards.len() == 2 {
                    self.phase = Phase::Declare;
                }
            }
            (Mv::Trump(t), Phase::Declare) => {
                self.trump = t;
                self.phase = Phase::Play;
                self.leader = p;
                self.log = format!("Atu: {}.", SUIT_NAMES[t as usize]);
            }
            (Mv::Play(c), Phase::Play) | (Mv::Marry(c), Phase::Play) => {
                if matches!(m, Mv::Marry(_)) {
                    let v = if suit(c) == self.trump { 40 } else { 20 };
                    self.marriage[p] += v;
                    self.log = format!(
                        "J{} anunță hlaska {}: +{v}.",
                        p + 1,
                        SUIT_NAMES[suit(c) as usize]
                    );
                }
                let h = &mut self.hands[p];
                let i = h.iter().position(|&x| x == c).unwrap();
                h.remove(i);
                self.trick.push((p, c));
                if self.trick.len() == 2 {
                    let (a, b) = (self.trick[0], self.trick[1]);
                    let winner = if self.beats(b.1, a.1) { b.0 } else { a.0 };
                    self.taken[winner] += points(a.1) + points(b.1);
                    self.trick.clear();
                    self.leader = winner;
                    if self.hands[0].is_empty() {
                        self.finish_hand(winner);
                    }
                }
            }
            _ => {}
        }
    }
    fn outcome(&self) -> Option<Outcome> {
        if self.phase == Phase::Over {
            Some(match self.scores[0].cmp(&self.scores[1]) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                _ => Outcome::Draw,
            })
        } else {
            None
        }
    }
    fn evaluate(&self, pl: Player) -> i32 {
        let me = pl.idx();
        let mut v = 500 * (self.scores[me] - self.scores[1 - me]);
        match self.phase {
            Phase::Declare | Phase::Discard => {
                if me == self.forhont {
                    // alegerea atuului: număr și valoare de cărți în culoarea aleasă
                    let t = self.trump;
                    let own: i32 = self.hands[me]
                        .iter()
                        .filter(|&&c| suit(c) == t)
                        .map(|&c| 3 + rank(c) as i32)
                        .sum();
                    v += own;
                    v += self.hands[me].iter().map(|&c| rank(c) as i32).sum::<i32>();
                    v += 3 * self.discards.iter().map(|&c| points(c)).sum::<i32>();
                }
            }
            _ => {
                v += 5
                    * (self.taken[me] + self.marriage[me]
                        - self.taken[1 - me]
                        - self.marriage[1 - me]);
                v += self.potential(me);
            }
        }
        v
    }
    fn render(&self) -> String {
        let view = self.viewer.unwrap_or(self.to_move()).idx();
        let mut s = format!(
            "Mâna {}/{HANDS}. Scor: J1 {} – {} J2. Atu: {}.\n",
            self.hand_no.min(HANDS),
            self.scores[0],
            self.scores[1],
            if matches!(self.phase, Phase::Play) {
                SUIT_NAMES[self.trump as usize]
            } else {
                "—"
            }
        );
        if self.phase != Phase::Over {
            let mut h = self.hands[view].clone();
            h.sort_by_key(|&c| (suit(c), std::cmp::Reverse(rank(c))));
            s.push_str(&format!(
                "Mâna ta (J{}): {}\n",
                view + 1,
                h.iter().map(|&c| text(c)).collect::<Vec<_>>().join(" ")
            ));
            s.push_str(&format!(
                "Adversarul are {} cărți.\n",
                self.hands[1 - view].len()
            ));
            if self.phase == Phase::Play {
                s.push_str(&format!(
                    "Puncte: J1 {} (+{} hlášky), J2 {} (+{}).\n",
                    self.taken[0], self.marriage[0], self.taken[1], self.marriage[1]
                ));
            }
            if !self.trick.is_empty() {
                s.push_str(&format!(
                    "Pe masă: {}\n",
                    self.trick
                        .iter()
                        .map(|&(p, c)| format!("J{}:{}", p + 1, text(c)))
                        .collect::<Vec<_>>()
                        .join(" ")
                ));
            }
        }
        s.push_str(self.log.trim());
        if self.phase != Phase::Over {
            s.push_str(&format!("\nLa mutare: Jucătorul {}.", self.actor() + 1));
        }
        s
    }
    fn format_move(&self, m: &Mv) -> String {
        match *m {
            Mv::Discard(c) => format!("las {}", text(c)),
            Mv::Trump(t) => format!("atu {}", SUIT_NAMES[t as usize]),
            Mv::Play(c) => text(c),
            Mv::Marry(c) => format!("{} hlaska", text(c)),
        }
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q4_util::*;
    use polytabletop_core::{GameObj, Player};

    #[test]
    fn honors_marriage_and_must_beat() {
        assert_eq!((0..32u8).map(points).sum::<i32>(), 80);
        let mut g = Mariash::with_seed(2);
        g.phase = Phase::Play;
        g.trump = 2; // inimi
        g.forhont = 0;
        g.leader = 0;
        g.hands = [vec![5, 13], vec![4, 6, 16]]; // J1: K♦g? ; J2: O,10 ghinde, 7 inimi
        g.trick = vec![(0, 5)]; // K ghinde condus
        let m = GameObj::legal_moves(&g);
        assert_eq!(m, vec!["10g"], "{m:?}"); // trebuie să bată regele cu zecele
        let mut h = Mariash::with_seed(2);
        h.phase = Phase::Play;
        h.forhont = 0;
        h.leader = 0;
        h.trump = 0;
        h.hands = [vec![4, 5, 9], vec![0, 1, 2]];
        h.trick.clear();
        let m = GameObj::legal_moves(&h);
        assert!(
            m.contains(&"Kg hlaska".to_string()) && m.contains(&"Og hlaska".to_string()),
            "{m:?}"
        );
        GameObj::play(&mut h, "Kg hlaska").unwrap();
        assert_eq!(h.marriage[0], 40);
    }

    #[test]
    fn viewer_hides_hand() {
        let g = Mariash::with_seed(3);
        let mut a = g.clone_box();
        a.set_viewer(Some(Player::One));
        let mut b = g.clone_box();
        b.set_viewer(Some(Player::Two));
        assert_ne!(a.render(), b.render());
    }

    #[test]
    fn random_and_ai_games_finish() {
        random_finishes(|s| Box::new(Mariash::with_seed(s)), 4, 1500);
        ai_vs_random(|s| Box::new(Mariash::with_seed(s)), 2, Player::One, 1500);
        ai_vs_random(|s| Box::new(Mariash::with_seed(s)), 3, Player::Two, 1500);
    }
}
