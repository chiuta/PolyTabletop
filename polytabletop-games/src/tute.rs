#![allow(clippy::doc_lazy_continuation)]
//! Tute (Spania), joc de levate cu baraja de 40 de cărți, pentru doi jucători (o singură mână).
//!
//! * Ordinea în culoare, de la cea mai mare: as, 3, rege (12), cal (11), valet (10), 7, 6, 5, 4, 2. Puncte:
//!   as 11, trei 10, rege 4, cal 3, valet 2 (120 de puncte în pachet). Fiecare primește 6 cărți; următoarea
//!   carte a pachetului se întoarce și dă **atuul** (e luată ultima din pachet).
//! * Atâta timp cât mai e pachet, poți juca orice carte (nu ești obligat să răspunzi la culoare). Câștigă
//!   levata cea mai mare din culoarea cerută sau cel mai mare atu; cine câștigă ia cele două cărți, trage primul și
//!   conduce. După epuizarea pachetului (arrastre) trebuie să răspunzi la culoare, să bați dacă poți și,
//!   fără culoare, să tăiești cu atu dacă ai.
//! * După ce ai câștigat o levată și când conduci poți: `canto <culoare>` (rege + cal dintr-o culoare:
//!   20 de puncte, 40 la culoarea atuului, o singură dată pe culoare), `cambio` (schimbi 7 de atu cu cartea
//!   atu răsturnată, cât mai e pachet) sau `tute` (patru regi sau patru cai: câștigi pe loc).
//! * Ultima levată dă 10 puncte („diez de últimas”). La final câștigă cine are mai multe puncte (levate + cantes).
//! * Informație ascunsă: AI-ul citește doar mâna lui, cărțile jucate și atuul; trage cărțile la pasul `trag`.
//! Mutări: `3o`, `12b` (număr + culoare o/c/e/b), `canto e`, `cambio`, `tute`, `trag`.

use crate::baraja_es::{deck, join, pos, suit, text};
use crate::cards::{handover_banner, Viewer};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const POWER: [u8; 10] = [10, 1, 9, 2, 3, 4, 5, 6, 7, 8];
const PTS: [i32; 10] = [11, 0, 10, 0, 0, 0, 0, 2, 3, 4];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Play(u8),
    Sing(u8),
    Swap,
    Tute,
    Draw,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Lead,
    Follow,
    Draw,
    Over,
}

#[derive(Clone)]
pub struct Tute {
    hands: [Vec<u8>; 2],
    stock: Vec<u8>,
    trump: u8,
    pts: [i32; 2],
    sung: [[bool; 4]; 2],
    leader: Player,
    table: Vec<u8>,
    phase: Phase,
    tricks: [u8; 2],
    winner: Option<Player>,
    viewer: Viewer,
    log: String,
}

fn power(c: u8) -> u8 {
    POWER[pos(c) as usize]
}

fn points(c: u8) -> i32 {
    PTS[pos(c) as usize]
}

fn is_king(c: u8) -> bool {
    pos(c) == 9
}
fn is_horse(c: u8) -> bool {
    pos(c) == 8
}

impl Default for Tute {
    fn default() -> Self {
        Self::new()
    }
}

impl Tute {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut d = deck(&mut rng);
        let mut hands = [Vec::new(), Vec::new()];
        for _ in 0..6 {
            hands[0].push(d.pop().unwrap());
            hands[1].push(d.pop().unwrap());
        }
        let up = d.pop().unwrap();
        let mut stock = vec![up];
        stock.extend(d);
        Tute {
            hands,
            trump: suit(up),
            stock,
            pts: [0; 2],
            sung: [[false; 4]; 2],
            leader: Player::One,
            table: Vec::new(),
            phase: Phase::Lead,
            tricks: [0; 2],
            winner: None,
            viewer: Viewer::Fixed(Player::One),
            log: String::new(),
        }
    }

    fn follower(&self) -> Player {
        self.leader.other()
    }

    fn follow_legal(&self, hand: &[u8], led: u8) -> Vec<u8> {
        if !self.stock.is_empty() {
            return hand.to_vec();
        }
        let same: Vec<u8> = hand
            .iter()
            .copied()
            .filter(|&c| suit(c) == suit(led))
            .collect();
        if !same.is_empty() {
            let better: Vec<u8> = same
                .iter()
                .copied()
                .filter(|&c| power(c) > power(led))
                .collect();
            return if better.is_empty() { same } else { better };
        }
        let trumps: Vec<u8> = hand
            .iter()
            .copied()
            .filter(|&c| suit(c) == self.trump)
            .collect();
        if !trumps.is_empty() {
            return trumps;
        }
        hand.to_vec()
    }

    fn beats(&self, led: u8, reply: u8) -> bool {
        if suit(reply) == suit(led) {
            power(reply) > power(led)
        } else {
            suit(reply) == self.trump
        }
    }

    fn hand_value(&self, p: Player) -> i32 {
        self.hands[p.idx()]
            .iter()
            .map(|&c| points(c) + if suit(c) == self.trump { 3 } else { 0 })
            .sum()
    }

    fn finish(&mut self) {
        self.phase = Phase::Over;
    }
}

impl Game for Tute {
    type Move = Move;

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Lead => self.leader,
            Phase::Follow => self.follower(),
            _ => Player::Two,
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        match self.phase {
            Phase::Over => Vec::new(),
            Phase::Draw => vec![Move::Draw],
            Phase::Lead => {
                let me = self.leader.idx();
                let h = &self.hands[me];
                let mut v: Vec<Move> = h.iter().map(|&c| Move::Play(c)).collect();
                if self.tricks[me] > 0 {
                    for s in 0..4u8 {
                        if !self.sung[me][s as usize]
                            && h.iter().any(|&c| suit(c) == s && is_king(c))
                            && h.iter().any(|&c| suit(c) == s && is_horse(c))
                        {
                            v.push(Move::Sing(s));
                        }
                    }
                    let seven = self.trump * 10 + 6;
                    if self.stock.len() > 1 && h.contains(&seven) {
                        v.push(Move::Swap);
                    }
                    if h.iter().filter(|&&c| is_king(c)).count() == 4
                        || h.iter().filter(|&&c| is_horse(c)).count() == 4
                    {
                        v.push(Move::Tute);
                    }
                }
                v.sort_by_key(|m| match m {
                    Move::Play(c) => *c as i32,
                    _ => 100,
                });
                v
            }
            Phase::Follow => {
                let f = self.follower().idx();
                self.follow_legal(&self.hands[f], self.table[0])
                    .into_iter()
                    .map(Move::Play)
                    .collect()
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        match *m {
            Move::Sing(s) => {
                let me = self.leader.idx();
                self.sung[me][s as usize] = true;
                let v = if s == self.trump { 40 } else { 20 };
                self.pts[me] += v;
                self.log = format!(
                    "J{} cântă rege și cal de {} ({} puncte).",
                    me + 1,
                    crate::baraja_es::SUIT_LETTERS[s as usize],
                    v
                );
            }
            Move::Swap => {
                let me = self.leader.idx();
                let seven = self.trump * 10 + 6;
                let k = self.hands[me].iter().position(|&c| c == seven).unwrap();
                let up = self.stock[0];
                self.hands[me][k] = up;
                self.stock[0] = seven;
                self.log = format!("J{} schimbă 7 de atu cu {}.", me + 1, text(up));
            }
            Move::Tute => {
                self.winner = Some(self.leader);
                self.log = format!("J{} declară tute!", self.leader.idx() + 1);
                self.finish();
            }
            Move::Play(c) => match self.phase {
                Phase::Lead => {
                    let me = self.leader.idx();
                    self.hands[me].retain(|&x| x != c);
                    self.table = vec![c];
                    self.phase = Phase::Follow;
                }
                Phase::Follow => {
                    let f = self.follower().idx();
                    self.hands[f].retain(|&x| x != c);
                    let led = self.table[0];
                    let win = if self.beats(led, c) {
                        self.follower()
                    } else {
                        self.leader
                    };
                    let w = win.idx();
                    self.pts[w] += points(led) + points(c);
                    self.tricks[w] += 1;
                    self.log = format!(
                        "J{}: {} contra {} — levata e a lui J{}.",
                        self.leader.idx() + 1,
                        text(led),
                        text(c),
                        w + 1
                    );
                    self.leader = win;
                    self.table.clear();
                    if self.hands[0].is_empty() && self.hands[1].is_empty() {
                        self.pts[w] += 10;
                        self.log.push_str(" Ultima levată: +10.");
                        self.finish();
                    } else if !self.stock.is_empty() {
                        self.phase = Phase::Draw;
                    } else {
                        self.phase = Phase::Lead;
                    }
                }
                _ => {}
            },
            Move::Draw => {
                let w = self.leader;
                self.hands[w.idx()].push(self.stock.pop().unwrap());
                self.hands[w.other().idx()].push(self.stock.pop().unwrap());
                self.phase = Phase::Lead;
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        if let Some(w) = self.winner {
            return Some(Outcome::Win(w));
        }
        Some(match self.pts[0].cmp(&self.pts[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    /// Doar informația lui `p`: punctele, mâna proprie și cartea de pe masă; nu mâna adversarului și
    /// nici pachetul.
    fn evaluate(&self, p: Player) -> i32 {
        let q = p.other();
        let mut v = (self.pts[p.idx()] - self.pts[q.idx()]) * 10;
        v += self.hand_value(p) * 2;
        if let (Phase::Follow, Some(&led)) = (self.phase, self.table.first()) {
            // p a condus: cartea expusă poate fi luată de adversar
            if self.leader == p {
                v -= (points(led) + 2) * 3 + if suit(led) == self.trump { 15 } else { 0 };
            }
        }
        v
    }

    fn render(&self) -> String {
        let me = self.viewer.who(self.to_move());
        let mut s = format!(
            "Tute — atu: {} (cartea răsturnată: {}). Pachet: {} cărți.\nPuncte: J1 {}, J2 {} (levate {}–{}).\n",
            ["oros", "copas", "espadas", "bastos"][self.trump as usize],
            if self.stock.is_empty() { "luată".to_string() } else { text(self.stock[0]) },
            self.stock.len(),
            self.pts[0],
            self.pts[1],
            self.tricks[0],
            self.tricks[1]
        );
        if let Viewer::HotSeat = self.viewer {
            if self.phase == Phase::Follow {
                s.push_str(&handover_banner(self.follower()));
            }
        }
        if !self.table.is_empty() {
            s.push_str(&format!(
                "Pe masă (condus de J{}): {}\n",
                self.leader.idx() + 1,
                join(&self.table)
            ));
        }
        for p in 0..2 {
            if p == me.idx() {
                let mut h = self.hands[p].clone();
                h.sort_unstable();
                s.push_str(&format!("J{} (mâna ta): {}\n", p + 1, join(&h)));
            } else {
                s.push_str(&format!("J{}: {} cărți\n", p + 1, self.hands[p].len()));
            }
        }
        match self.phase {
            Phase::Lead => s.push_str(&format!("Conduce J{}.\n", self.leader.idx() + 1)),
            Phase::Follow => s.push_str(&format!("Răspunde J{}.\n", self.follower().idx() + 1)),
            Phase::Draw => s.push_str("Se trag cărți: scrie «trag».\n"),
            Phase::Over => s.push_str("Mâna s-a terminat.\n"),
        }
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Play(c) => text(*c),
            Move::Sing(s) => format!("canto {}", crate::baraja_es::SUIT_LETTERS[*s as usize]),
            Move::Swap => "cambio".into(),
            Move::Tute => "tute".into(),
            Move::Draw => "trag".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = match who {
            Some(p) => Viewer::Fixed(p),
            None => Viewer::HotSeat,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deal_and_card_values() {
        let g = Tute::with_seed(1);
        assert_eq!(g.hands[0].len(), 6);
        assert_eq!(g.stock.len(), 40 - 12);
        let all: i32 = (0..40u8).map(points).sum();
        assert_eq!(all, 120);
        assert!(power(0) > power(2) && power(2) > power(9) && power(9) > power(8));
    }

    #[test]
    fn random_games_end_with_consistent_points() {
        for seed in 1..=40u64 {
            let mut g = Tute::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                assert!(!mv.is_empty(), "{}", g.render());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
                assert!(n < 400);
            }
            if g.winner.is_none() {
                let cantes: i32 = (0..2)
                    .map(|p| {
                        (0..4)
                            .filter(|&s| g.sung[p][s])
                            .map(|s| if s as u8 == g.trump { 40 } else { 20 })
                            .sum::<i32>()
                    })
                    .sum();
                assert_eq!(g.pts[0] + g.pts[1], 120 + 10 + cantes);
            }
        }
    }

    #[test]
    fn must_follow_in_arrastre_and_view_hidden() {
        let mut g = Tute::with_seed(2);
        g.stock.clear();
        g.trump = 0;
        g.hands = [vec![12, 13], vec![15, 21, 2]];
        g.leader = Player::One;
        g.phase = Phase::Follow;
        g.table = vec![10];
        // J2 are copas (suit 1): 15, 12? răspunde la culoare (copas) și bate dacă poate
        let mv = g.legal_moves();
        assert!(mv
            .iter()
            .all(|m| matches!(m, Move::Play(c) if suit(*c) == 1)));
        g.set_viewer(Some(Player::Two));
        let a = g.render();
        g.set_viewer(Some(Player::One));
        assert_ne!(a, g.render());
    }
}
