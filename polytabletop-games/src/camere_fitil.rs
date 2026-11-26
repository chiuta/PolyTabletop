//! Două Camere și un Fitil — separă sau adună (variantă originală, 2 jucători, informație ascunsă).
//!
//! Șase personaje (1–6) sunt împărțite în două camere. J1 știe în secret care e Președintele, J2 știe în secret
//! care e Artificierul (personaje diferite). Pe rând fiecare mută de trei ori: `muta c` trece personajul c
//! în camera cealaltă. După cele șase mutări, `verific` arată: dacă Președintele și Artificierul sunt în
//! aceeași cameră, câștigă J2; altfel câștigă J1. Mutările sunt publice, deci fiecare încearcă să
//! deducă și să încurce secretul celuilalt. AI-ul nu citește secretul adversarului.
use crate::b6_util::pl;
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

#[derive(Clone)]
pub struct Camere {
    viewer: Option<Player>,
    room: [u8; 6],
    pres: usize,
    bomb: usize,
    ply: u32,
    done: bool,
}

impl Camere {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut room = [0u8, 0, 0, 1, 1, 1];
        shuf(&mut room, &mut rng);
        let pres = rng.below(6);
        let bomb = (pres + 1 + rng.below(5)) % 6;
        Camere {
            viewer: None,
            room,
            pres,
            bomb,
            ply: 0,
            done: false,
        }
    }
    fn mover(&self) -> usize {
        (self.ply % 2) as usize
    }
}

impl Default for Camere {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Camere {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        if self.ply >= 6 {
            Player::One
        } else {
            pl(self.mover())
        }
    }
    fn moves(&self) -> Vec<String> {
        if self.ply >= 6 {
            return vec!["verific".into()];
        }
        (1..=6).map(|c| format!("muta {c}")).collect()
    }
    fn step(&mut self, m: &str) {
        if m == "verific" {
            self.done = true;
            return;
        }
        let c = m[5..].parse::<usize>().unwrap() - 1;
        self.room[c] ^= 1;
        self.ply += 1;
    }
    fn result(&self) -> Option<Outcome> {
        if self.done {
            return Some(Outcome::Win(
                if self.room[self.pres] == self.room[self.bomb] {
                    Player::Two
                } else {
                    Player::One
                },
            ));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        // probabilitatea (în miimi) ca secretul necunoscut să fie în camera potrivită
        match p {
            Player::One => {
                let r = self.room[self.pres];
                let sep = (0..6)
                    .filter(|&c| c != self.pres && self.room[c] != r)
                    .count() as i32;
                200 * sep
            }
            Player::Two => {
                let r = self.room[self.bomb];
                let same = (0..6)
                    .filter(|&c| c != self.bomb && self.room[c] == r)
                    .count() as i32;
                200 * same
            }
        }
    }
    fn show(&self) -> String {
        let viewer = self.viewer.map(pi).unwrap_or(pi(self.side()));
        let list = |r: u8| {
            (0..6)
                .filter(|&c| self.room[c] == r)
                .map(|c| (c + 1).to_string())
                .collect::<Vec<_>>()
                .join(" ")
        };
        format!(
            "Două Camere și un Fitil — mutarea {}/6\nCamera A: {}\nCamera B: {}\nSecretul tău (J{}): {} este {}\nLa mutare: {} (muta c | verific)\n",
            (self.ply + 1).min(6),
            list(0),
            list(1),
            viewer + 1,
            if viewer == 0 { self.pres + 1 } else { self.bomb + 1 },
            if viewer == 0 { "Președintele" } else { "Artificierul" },
            pname(self.side())
        )
    }
    fn view(&mut self, w: Option<Player>) {
        self.viewer = w;
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        crate::brand_util::check(Camere::with_seed, 20);
    }
    #[test]
    fn secrets_are_private_and_ai_ignores_the_other() {
        let g = Camere::with_seed(4);
        // J1 (mutare) nu își schimbă alegerea după artificierul real
        let mut h = g.clone();
        h.bomb = (0..6).find(|&c| c != g.pres && c != g.bomb).unwrap();
        for seed in 1..6 {
            assert_eq!(ai_of(&g, seed), ai_of(&h, seed));
        }
        let mut v = g.clone();
        v.view(Some(Player::One));
        assert!(v
            .show()
            .contains(&format!("{} este Președintele", g.pres + 1)));
        assert!(!v.show().contains("Artificierul"));
    }
}
