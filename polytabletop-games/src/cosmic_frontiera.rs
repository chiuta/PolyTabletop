//! Frontiera Stelară — duel de ambasadori (variantă originală, inspirată de genul „întâlniri
//! cu negociere”). Fiecare are 8 cărți numerotate (0,1,2,3,5,8,13,21) și 2 aliați (+3, o dată).
//! Rolurile alternează: provocatorul joacă `ata k [+]` cu fața în sus, apărătorul răspunde `ras k [+]`
//! (cu aliatul sau nu). Cel mai mare total câștigă o colonie de la celălalt; egal: nimic.
//! Primul care ajunge la 7 colonii (pornind de la 4) sau lasă adversarul cu 1, câștigă.
use crate::brand_util::*;
use polytabletop_core::{Outcome, Player};

const CARDS: [i32; 8] = [0, 1, 2, 3, 5, 8, 13, 21];

#[derive(Clone)]
pub struct Cosmic {
    col: [i32; 2],
    used: [u8; 2],
    ally: [i32; 2],
    chal: usize,
    pend: Option<(i32, bool)>,
    enc: u32,
    side: Player,
}

impl Cosmic {
    pub fn new() -> Self {
        Cosmic {
            col: [4, 4],
            used: [0; 2],
            ally: [2, 2],
            chal: 0,
            pend: None,
            enc: 0,
            side: Player::One,
        }
    }
    pub fn with_seed(_s: u64) -> Self {
        Self::new()
    }
}

impl Default for Cosmic {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Cosmic {
    fn side(&self) -> Player {
        self.side
    }
    fn depth(&self) -> u32 {
        2
    }
    fn moves(&self) -> Vec<String> {
        let me = pi(self.side);
        let verb = if self.pend.is_none() { "ata" } else { "ras" };
        let mut v = Vec::new();
        for k in 0..8 {
            if self.used[me] & (1 << k) == 0 {
                v.push(format!("{verb} {k}"));
                if self.ally[me] > 0 {
                    v.push(format!("{verb} {k} +"));
                }
            }
        }
        v
    }
    fn step(&mut self, m: &str) {
        let me = pi(self.side);
        let t: Vec<&str> = m.split(' ').collect();
        let k: usize = t[1].parse().unwrap();
        let ally = t.len() > 2;
        self.used[me] |= 1 << k;
        if ally {
            self.ally[me] -= 1;
        }
        let tot = CARDS[k] + if ally { 3 } else { 0 };
        match self.pend.take() {
            None => {
                self.pend = Some((tot, ally));
                self.side = self.side.other();
            }
            Some((ct, _)) => {
                let ch = self.chal;
                let df = 1 - ch;
                if ct > tot {
                    self.col[ch] += 1;
                    self.col[df] -= 1;
                } else if tot > ct {
                    self.col[df] += 1;
                    self.col[ch] -= 1;
                }
                if self.used[0] == 255 {
                    self.used = [0; 2];
                }
                self.enc += 1;
                self.chal = 1 - self.chal;
                self.side = if self.chal == 0 {
                    Player::One
                } else {
                    Player::Two
                };
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.col[0] >= 7 || self.col[1] <= 1 {
            return Some(Outcome::Win(Player::One));
        }
        if self.col[1] >= 7 || self.col[0] <= 1 {
            return Some(Outcome::Win(Player::Two));
        }
        if self.enc >= 40 {
            return Some(by_score(self.col[0], self.col[1]));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let me_cards: i32 = (0..8)
            .filter(|k| self.used[a] & (1 << k) == 0)
            .map(|k| CARDS[k])
            .sum();
        let op_cards: i32 = (0..8)
            .filter(|k| self.used[1 - a] & (1 << k) == 0)
            .map(|k| CARDS[k])
            .sum();
        20 * (self.col[a] - self.col[1 - a])
            + (me_cards - op_cards) / 4
            + 2 * (self.ally[a] - self.ally[1 - a])
    }
    fn show(&self) -> String {
        let left = |p: usize| -> Vec<i32> {
            (0..8)
                .filter(|k| self.used[p] & (1 << k) == 0)
                .map(|k| CARDS[k])
                .collect()
        };
        format!(
            "Frontiera Stelară — întâlnirea {}/40 | colonii J1 {} J2 {} | aliați {:?}\nCărți (indice 0..7 = 0,1,2,3,5,8,13,21): J1 {:?} J2 {:?}\n{}La mutare: {} ({}: k [+])\n",
            self.enc + 1,
            self.col[0],
            self.col[1],
            self.ally,
            left(0),
            left(1),
            match self.pend {
                Some((t, a)) => format!("Provocatorul a jucat total {t}{}\n", if a { " (cu aliat)" } else { "" }),
                None => String::new(),
            },
            pname(self.side),
            if self.pend.is_none() { "ata" } else { "ras" }
        )
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        crate::brand_util::check(Cosmic::with_seed, 200);
    }
}
