//! Regii (Kings / Ring of Fire, SUA), 2 jucători — versiune fără alcool, cu jetoane de penalizare.
//!
//! Jocul de petrecere cu pachet de 52 de cărți în care fiecare rang are o regulă. Aici „înghițiturile” devin
//! **jetoane de penalizare**; câștigă cine are mai puține la sfârșit. Se trage pe rând o carte (`trag`):
//! * **A cascadă**: ambii +1. **2 tu**: adversarul +2. **3 eu**: tu +2. **4 podea**: unul dintre voi (la sorți) +1.
//! * **5 echipă**: adversarul +1. **6 echipă**: tu +1. **8 partener**: unul dintre voi (la sorți) +1.
//! * **9 rimă**: adversarul +1. **10 categorie**: tu +1. **Q întrebări**: unul dintre voi (la sorți) +2.
//! * **7 cer** și **J regulă**: alegi (`scut` = următoarele 2 jetoane care ți se dau nu se pun; `atac` =
//!   adversarul +1; la J, atacul dă +2).
//! * **K rege**: primii trei regi adaugă 2 jetoane în pahar și tu +1; al patrulea rege te face să iei **tot
//!   paharul** și încheie jocul. Dacă pachetul se termină, jocul se oprește.
//! * Cartea se trage cu generatorul jocului la mutarea `trag`, deci AI-ul nu o poate prevedea.
//!
//! Mutări: `trag`, `scut`, `atac`.

use crate::q6_util::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const RN: [&str; 13] = [
    "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
];
const SN: [char; 4] = ['c', 'd', 'h', 's'];

#[derive(Clone, PartialEq, Debug)]
pub enum Move {
    Draw,
    Shield,
    Attack,
}

#[derive(Clone)]
pub struct Kings {
    rng: Rng,
    deck: Vec<u8>,
    tokens: [i32; 2],
    shield: [i32; 2],
    cup: i32,
    kings: u8,
    side: Player,
    pending: Option<u8>,
    last: Option<u8>,
    log: String,
    over: bool,
}

impl Default for Kings {
    fn default() -> Self {
        Self::new()
    }
}

impl Kings {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck: Vec<u8> = (0..52).collect();
        shuffle(&mut deck, &mut rng);
        Kings {
            rng,
            deck,
            tokens: [0, 0],
            shield: [0, 0],
            cup: 0,
            kings: 0,
            side: Player::One,
            pending: None,
            last: None,
            log: String::new(),
            over: false,
        }
    }

    pub fn tokens(&self) -> [i32; 2] {
        self.tokens
    }

    fn give(&mut self, p: usize, n: i32) {
        let mut n = n;
        let s = self.shield[p].min(n);
        self.shield[p] -= s;
        n -= s;
        self.tokens[p] += n;
    }
}

impl Game for Kings {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over {
            return Vec::new();
        }
        if self.pending.is_some() {
            vec![Move::Shield, Move::Attack]
        } else {
            vec![Move::Draw]
        }
    }

    fn apply(&mut self, m: &Move) {
        let me = self.side.idx();
        let you = 1 - me;
        match m {
            Move::Draw => {
                let c = self.deck.pop().unwrap();
                self.last = Some(c);
                let r = (c % 13) as usize;
                let name = format!("{}{}", RN[r], SN[(c / 13) as usize]);
                let mut next = true;
                let eff: String = match r {
                    0 => {
                        self.give(me, 1);
                        self.give(you, 1);
                        "cascadă: ambii +1".into()
                    }
                    1 => {
                        self.give(you, 2);
                        "tu: adversarul +2".into()
                    }
                    2 => {
                        self.give(me, 2);
                        "eu: +2 pentru tine".into()
                    }
                    3 | 7 => {
                        let who = self.rng.below(2);
                        self.give(who, 1);
                        format!("reflex: J{} +1", who + 1)
                    }
                    4 | 8 => {
                        self.give(you, 1);
                        "adversarul +1".into()
                    }
                    5 | 9 => {
                        self.give(me, 1);
                        "tu +1".into()
                    }
                    6 | 10 => {
                        self.pending = Some(r as u8);
                        next = false;
                        "alegi: scut sau atac".into()
                    }
                    11 => {
                        let who = self.rng.below(2);
                        self.give(who, 2);
                        format!("întrebări: J{} +2", who + 1)
                    }
                    _ => {
                        self.kings += 1;
                        if self.kings >= 4 {
                            let cup = self.cup;
                            self.give(me, cup);
                            self.over = true;
                            format!("al patrulea rege: iei paharul ({cup})")
                        } else {
                            self.cup += 2;
                            self.give(me, 1);
                            format!("rege {}: +2 în pahar, tu +1", self.kings)
                        }
                    }
                };
                self.log = format!("J{} trage {name} — {eff}", me + 1);
                if self.deck.is_empty() {
                    self.over = true;
                }
                if next && !self.over {
                    self.side = self.side.other();
                }
            }
            Move::Shield => {
                self.pending = None;
                self.shield[me] += 2;
                self.log = format!("J{} își ia un scut (2 jetoane).", me + 1);
                self.side = self.side.other();
            }
            Move::Attack => {
                let r = self.pending.take().unwrap();
                let n = if r == 10 { 2 } else { 1 };
                self.give(you, n);
                self.log = format!("J{} atacă: J{} +{n}.", me + 1, you + 1);
                self.side = self.side.other();
            }
        }
        if self.deck.is_empty() && self.pending.is_none() {
            self.over = true;
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            return None;
        }
        Some(match self.tokens[0].cmp(&self.tokens[1]) {
            std::cmp::Ordering::Less => Outcome::Win(Player::One),
            std::cmp::Ordering::Greater => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        (self.tokens[1 - me] - self.tokens[me]) * 10 + (self.shield[me] - self.shield[1 - me]) * 4
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Regii — jetoane de penalizare: J1 {} (scut {}), J2 {} (scut {}); în pahar: {}; regi trași: {}/4; în pachet: {}\n",
            self.tokens[0],
            self.shield[0],
            self.tokens[1],
            self.shield[1],
            self.cup,
            self.kings,
            self.deck.len()
        );
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        match self.outcome() {
            Some(Outcome::Win(p)) => s.push_str(&format!(
                "Jucătorul {} câștigă (mai puține jetoane).\n",
                p.idx() + 1
            )),
            Some(Outcome::Draw) => s.push_str("Remiză.\n"),
            None => {
                if self.pending.is_some() {
                    s.push_str("Alege: «scut» sau «atac».\n");
                }
                s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
            }
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Draw => "trag".into(),
            Move::Shield => "scut".into(),
            Move::Attack => "atac".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn games_end_with_fourth_king_or_deck() {
        for seed in 1..=20u64 {
            let mut g = Kings::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 200);
            }
        }
    }

    #[test]
    fn shield_absorbs() {
        let mut g = Kings::with_seed(1);
        g.shield[0] = 2;
        g.give(0, 3);
        assert_eq!(g.tokens[0], 1);
        assert_eq!(g.shield[0], 0);
    }

    #[test]
    fn fourth_king_takes_the_cup() {
        let mut g = Kings::with_seed(1);
        g.kings = 3;
        g.cup = 6;
        g.deck.push(12); // K♣
        g.apply(&Move::Draw);
        assert!(g.over);
        assert_eq!(g.tokens[0], 6);
    }
}
