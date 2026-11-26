//! Chinchirorin (Japonia) — joc de zaruri cu trei zaruri aruncate într-un bol, între „oya” (banca) și „ko”.
//!
//! Aici, 2 jucători cu câte 30 de jetoane; rolurile alternează la fiecare rundă.
//! * Ko pariază 1–3 jetoane (`pariez 2`), apoi oya aruncă (`arunc`), până la 3 încercări pentru o „mână”.
//! * Mâini: **Pinzoro** (1-1-1) plătește 5×, **Arashi** (alt triplet) 3×, **Shigoro** (4-5-6) 2×,
//!   **punct** = valoarea zarului singur dintr-o pereche (de ex. 2-2-5 → 5); **Hifumi** (1-2-3) pierde 2×;
//!   fără mână după 3 aruncări = pierde 1×.
//! * Dacă oya obține Pinzoro/Arashi/Shigoro câștigă imediat; Hifumi/fără mână = pierde imediat.
//!   Cu un punct, aruncă ko, cu aceleași reguli; punctul mai mare câștigă 1×, egal = remiză.
//! * Se termină când cineva rămâne fără jetoane sau după 12 runde (mai multe jetoane câștigă).
//!
//! Sursă: regulile publice ale jocului (Wikipedia „Chinchirorin”); multiplicatorii sunt cei uzuali.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const START: i32 = 30;
const ROUNDS: u8 = 12;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Hand {
    Hifumi,
    Menashi,
    Point(u8),
    Shigoro,
    Arashi(u8),
    Pinzoro,
}

pub fn classify(d: [u8; 3]) -> Hand {
    let mut s = d;
    s.sort_unstable();
    if s[0] == s[2] {
        return if s[0] == 1 {
            Hand::Pinzoro
        } else {
            Hand::Arashi(s[0])
        };
    }
    if s == [4, 5, 6] {
        return Hand::Shigoro;
    }
    if s == [1, 2, 3] {
        return Hand::Hifumi;
    }
    if s[0] == s[1] {
        return Hand::Point(s[2]);
    }
    if s[1] == s[2] {
        return Hand::Point(s[0]);
    }
    Hand::Menashi
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mv {
    Bet(u8),
    Throw,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Bet,
    Oya,
    Ko,
}

#[derive(Clone)]
pub struct Chinchirorin {
    chips: [i32; 2],
    oya: usize,
    bet: i32,
    phase: Phase,
    throws: u8,
    dice: [u8; 3],
    oya_hand: Option<Hand>,
    round: u8,
    rng: Rng,
    log: String,
}

impl Default for Chinchirorin {
    fn default() -> Self {
        Self::new()
    }
}

impl Chinchirorin {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        Chinchirorin {
            chips: [START; 2],
            oya: 0,
            bet: 0,
            phase: Phase::Bet,
            throws: 0,
            dice: [0; 3],
            oya_hand: None,
            round: 1,
            rng: Rng::new(seed),
            log: String::new(),
        }
    }
    pub fn chips(&self) -> (i32, i32) {
        (self.chips[0], self.chips[1])
    }

    fn ko(&self) -> usize {
        1 - self.oya
    }

    /// `winner` câștigă `mult × bet` de la celălalt (limitat de jetoanele lui); apoi runda următoare.
    fn pay(&mut self, winner: Option<usize>, mult: i32) {
        if let Some(w) = winner {
            let amt = (mult * self.bet).min(self.chips[1 - w]);
            self.chips[w] += amt;
            self.chips[1 - w] -= amt;
            self.log.push_str(&format!(" J{} câștigă {amt}.", w + 1));
        } else {
            self.log.push_str(" Egalitate, nimic nu se plătește.");
        }
        self.round += 1;
        self.oya = 1 - self.oya;
        self.phase = Phase::Bet;
        self.throws = 0;
        self.oya_hand = None;
    }

    fn side_now(&self) -> usize {
        match self.phase {
            Phase::Bet | Phase::Ko => self.ko(),
            Phase::Oya => self.oya,
        }
    }
}

impl Game for Chinchirorin {
    type Move = Mv;
    fn to_move(&self) -> Player {
        if self.side_now() == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn legal_moves(&self) -> Vec<Mv> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        match self.phase {
            Phase::Bet => {
                let max = 3.min(self.chips[0]).min(self.chips[1]);
                (1..=max as u8).map(Mv::Bet).collect()
            }
            _ => vec![Mv::Throw],
        }
    }
    fn apply(&mut self, m: &Mv) {
        match (*m, self.phase) {
            (Mv::Bet(n), Phase::Bet) => {
                self.bet = n as i32;
                self.phase = Phase::Oya;
                self.throws = 0;
                self.log = format!("Runda {}: J{} (ko) pariază {n}.", self.round, self.ko() + 1);
            }
            (Mv::Throw, ph) => {
                let d = [self.rng.d6(), self.rng.d6(), self.rng.d6()];
                self.dice = d;
                self.throws += 1;
                let h = classify(d);
                let who = self.side_now();
                self.log.push_str(&format!(
                    " J{} aruncă {}-{}-{} ({}).",
                    who + 1,
                    d[0],
                    d[1],
                    d[2],
                    hand_name(h)
                ));
                if h == Hand::Menashi && self.throws < 3 {
                    return;
                }
                if ph == Phase::Oya {
                    match h {
                        Hand::Pinzoro => self.pay(Some(self.oya), 5),
                        Hand::Arashi(_) => self.pay(Some(self.oya), 3),
                        Hand::Shigoro => self.pay(Some(self.oya), 2),
                        Hand::Hifumi => {
                            let ko = self.ko();
                            self.pay(Some(ko), 2)
                        }
                        Hand::Menashi => {
                            let ko = self.ko();
                            self.pay(Some(ko), 1)
                        }
                        Hand::Point(_) => {
                            self.oya_hand = Some(h);
                            self.phase = Phase::Ko;
                            self.throws = 0;
                        }
                    }
                } else {
                    let ko = self.ko();
                    let oy = self.oya;
                    match h {
                        Hand::Pinzoro => self.pay(Some(ko), 5),
                        Hand::Arashi(_) => self.pay(Some(ko), 3),
                        Hand::Shigoro => self.pay(Some(ko), 2),
                        Hand::Hifumi => self.pay(Some(oy), 2),
                        Hand::Menashi => self.pay(Some(oy), 1),
                        Hand::Point(k) => {
                            let Some(Hand::Point(o)) = self.oya_hand else {
                                unreachable!()
                            };
                            match k.cmp(&o) {
                                std::cmp::Ordering::Greater => self.pay(Some(ko), 1),
                                std::cmp::Ordering::Less => self.pay(Some(oy), 1),
                                _ => self.pay(None, 0),
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
    fn outcome(&self) -> Option<Outcome> {
        for p in 0..2 {
            if self.chips[p] <= 0 {
                return Some(Outcome::Win(if p == 0 { Player::Two } else { Player::One }));
            }
        }
        if self.round > ROUNDS {
            return Some(match self.chips[0].cmp(&self.chips[1]) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                _ => Outcome::Draw,
            });
        }
        None
    }
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut v = 100 * (self.chips[me] - self.chips[1 - me]);
        // ko are un dezavantaj: pariu mic e preferabil pentru ko când e la rând
        if self.phase == Phase::Oya && self.ko() == me {
            v -= 10 * self.bet;
        } else if self.phase == Phase::Oya {
            v += 10 * self.bet;
        }
        v
    }
    fn render(&self) -> String {
        let role = |p: usize| if p == self.oya { "oya (banca)" } else { "ko" };
        let mut s = format!(
            "Runda {}/{ROUNDS}. Jetoane: J1 {} ({}), J2 {} ({}). Pariu: {}.\n",
            self.round.min(ROUNDS),
            self.chips[0],
            role(0),
            self.chips[1],
            role(1),
            self.bet
        );
        if self.throws > 0 || self.oya_hand.is_some() {
            s.push_str(&format!(
                "Zaruri: {}-{}-{}. ",
                self.dice[0], self.dice[1], self.dice[2]
            ));
        }
        s.push_str(self.log.trim());
        s.push_str(&format!("\nLa mutare: Jucătorul {}.", self.side_now() + 1));
        s
    }
    fn format_move(&self, m: &Mv) -> String {
        match m {
            Mv::Bet(n) => format!("pariez {n}"),
            Mv::Throw => "arunc".into(),
        }
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

fn hand_name(h: Hand) -> String {
    match h {
        Hand::Hifumi => "Hifumi 1-2-3, pierde".into(),
        Hand::Menashi => "fără mână".into(),
        Hand::Point(n) => format!("punct {n}"),
        Hand::Shigoro => "Shigoro 4-5-6".into(),
        Hand::Arashi(n) => format!("Arashi {n}-{n}-{n}"),
        Hand::Pinzoro => "Pinzoro".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q4_util::*;
    use polytabletop_core::Player;

    #[test]
    fn hands_are_classified() {
        assert_eq!(classify([1, 1, 1]), Hand::Pinzoro);
        assert_eq!(classify([5, 5, 5]), Hand::Arashi(5));
        assert_eq!(classify([6, 4, 5]), Hand::Shigoro);
        assert_eq!(classify([3, 1, 2]), Hand::Hifumi);
        assert_eq!(classify([2, 5, 2]), Hand::Point(5));
        assert_eq!(classify([1, 3, 6]), Hand::Menashi);
    }

    #[test]
    fn random_and_ai_games_finish() {
        random_finishes(|s| Box::new(Chinchirorin::with_seed(s)), 8, 600);
        ai_vs_random(
            |s| Box::new(Chinchirorin::with_seed(s)),
            2,
            Player::One,
            600,
        );
        ai_vs_random(
            |s| Box::new(Chinchirorin::with_seed(s)),
            3,
            Player::Two,
            600,
        );
    }
}
