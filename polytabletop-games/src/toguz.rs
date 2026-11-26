//! Toguz Kumalak (Kazahstan / Kârgâzstan), 2 jucători, din regulile publice.
//!
//! * Două rânduri de 9 gropi (otau), câte 9 „kumalak” (162 în total) și câte un depozit (kazan).
//! * Muți toate bilele dintr-o groapă proprie: dacă sunt mai multe de una, prima rămâne în aceeași
//!   groapă, restul se pun câte una în gropile următoare (în sens trigonometric: gropile tale 1–9,
//!   apoi gropile adversarului 1–9, apoi iar ale tale); o singură bilă trece în groapa următoare.
//! * Dacă ultima bilă cade într-o groapă a adversarului și o face **pară**, iei toate bilele din ea.
//! * Dacă ultima bilă o face **3**, groapa devine **tuzdyk** (o singură dată per jucător, nu groapa
//!   a 9-a, nu groapa cu același număr ca tuzdyk-ul adversarului): cele 3 bile merg la tine, iar de
//!   acum orice bilă semănată acolo ajunge în kazanul tău.
//! * Câștigă primul care ajunge la **82** de bile în kazan. Dacă la rândul cuiva toate gropile
//!   lui sunt goale, jocul se termină și adversarul își adaugă bilele rămase pe partea lui.
//!
//! Mutări: `1`…`9` (numărul gropii tale, de la stânga la dreapta).

use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone)]
pub struct Toguz {
    /// 0..9 = gropile J1 (1–9), 9..18 = gropile J2 (1–9).
    h: [u8; 18],
    kazan: [u8; 2],
    /// Indicele gropii (0..9) din partea adversarului care e tuzdyk al jucătorului.
    tuz: [Option<usize>; 2],
    side: Player,
    over: bool,
}

impl Default for Toguz {
    fn default() -> Self {
        Self::new()
    }
}

impl Toguz {
    pub fn new() -> Self {
        Toguz {
            h: [9; 18],
            kazan: [0; 2],
            tuz: [None; 2],
            side: Player::One,
            over: false,
        }
    }

    pub fn kazans(&self) -> (u8, u8) {
        (self.kazan[0], self.kazan[1])
    }

    fn own(&self, p: Player) -> std::ops::Range<usize> {
        p.idx() * 9..p.idx() * 9 + 9
    }

    /// Dacă groapa `i` e tuzdyk, returnează jucătorul care o deține.
    fn tuz_owner(&self, i: usize) -> Option<usize> {
        // tuz[p] e în partea adversarului lui p
        for p in 0..2 {
            if let Some(t) = self.tuz[p] {
                if (1 - p) * 9 + t == i {
                    return Some(p);
                }
            }
        }
        None
    }

    fn finish_check(&mut self) {
        if self.kazan[0] >= 82 || self.kazan[1] >= 82 {
            self.over = true;
        }
    }
}

impl Game for Toguz {
    type Move = usize;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<usize> {
        if self.over {
            return Vec::new();
        }
        self.own(self.side)
            .filter(|&i| self.h[i] > 0)
            .map(|i| i - self.side.idx() * 9 + 1)
            .collect()
    }

    fn apply(&mut self, m: &usize) {
        let me = self.side.idx();
        let start = me * 9 + (m - 1);
        let n = self.h[start] as usize;
        self.h[start] = 0;
        let mut pos = start;
        let put = |g: &mut Toguz, pos: usize| {
            if let Some(o) = g.tuz_owner(pos) {
                g.kazan[o] += 1;
            } else {
                g.h[pos] += 1;
            }
        };
        if n == 1 {
            pos = (start + 1) % 18;
            put(self, pos);
        } else {
            self.h[start] = 1;
            for _ in 1..n {
                pos = (pos + 1) % 18;
                put(self, pos);
            }
        }
        // captură
        let opp_side = (1 - me) * 9..(1 - me) * 9 + 9;
        if opp_side.contains(&pos) && self.tuz_owner(pos).is_none() {
            let local = pos - (1 - me) * 9;
            if self.h[pos].is_multiple_of(2) {
                self.kazan[me] += self.h[pos];
                self.h[pos] = 0;
            } else if self.h[pos] == 3
                && local != 8
                && self.tuz[me].is_none()
                && self.tuz[1 - me] != Some(local)
            {
                self.kazan[me] += 3;
                self.h[pos] = 0;
                self.tuz[me] = Some(local);
            }
        }
        self.side = self.side.other();
        self.finish_check();
        if !self.over && self.own(self.side).all(|i| self.h[i] == 0) {
            let o = self.side.other().idx();
            let rest: u8 = self.own(self.side.other()).map(|i| self.h[i]).sum();
            self.kazan[o] += rest;
            for i in self.own(self.side.other()) {
                self.h[i] = 0;
            }
            self.over = true;
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            return None;
        }
        Some(match self.kazan[0].cmp(&self.kazan[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let (a, b) = (p.idx(), 1 - p.idx());
        let mut v = (self.kazan[a] as i32 - self.kazan[b] as i32) * 20;
        let sa: i32 = self.own(p).map(|i| self.h[i] as i32).sum();
        let sb: i32 = self.own(p.other()).map(|i| self.h[i] as i32).sum();
        v += (sa - sb) * 2;
        if let Some(t) = self.tuz[a] {
            v += 40 + t as i32 * 3;
        }
        if let Some(t) = self.tuz[b] {
            v -= 40 + t as i32 * 3;
        }
        v
    }

    fn render(&self) -> String {
        let row = |p: usize| {
            (0..9)
                .map(|k| {
                    let i = p * 9 + k;
                    let t = if self.tuz_owner(i).is_some() { "*" } else { "" };
                    format!("{:>3}{t}", self.h[i])
                })
                .collect::<Vec<_>>()
                .join(" ")
        };
        let mut s = String::from("Toguz Kumalak (9 gropi pe rând; * = tuzdyk)\n");
        s.push_str(&format!("J2: {}   kazan {}\n", row(1), self.kazan[1]));
        s.push_str(&format!("J1: {}   kazan {}\n", row(0), self.kazan[0]));
        s.push_str("     1   2   3   4   5   6   7   8   9  (numerele gropilor tale)\n");
        match self.outcome() {
            Some(Outcome::Win(p)) => s.push_str(&format!("Câștigă J{}.\n", p.idx() + 1)),
            Some(Outcome::Draw) => s.push_str("Remiză.\n"),
            None => s.push_str(&format!("La mutare: J{}\n", self.side.idx() + 1)),
        }
        s
    }

    fn format_move(&self, m: &usize) -> String {
        m.to_string()
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(6)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    #[test]
    fn first_move_sows_and_conserves_stones() {
        let mut g = Toguz::new();
        g.apply(&1);
        assert_eq!(g.h[0], 1);
        assert_eq!(g.h.iter().map(|&x| x as u32).sum::<u32>(), 162);
        // 8 bile: 1,...: ultima în groapa 9 a lui J1 (index 8)
        assert_eq!(g.h[8], 10);
    }

    #[test]
    fn even_capture_and_tuzdyk() {
        let mut g = Toguz::new();
        g.h = [0; 18];
        g.h[0] = 2; // J1 groapa 1: 1 rămâne, 1 în groapa 2
        g.h[1] = 0;
        g.h[2] = 1;
        g.h[8] = 1; // J1 groapa 9 -> merge în J2 groapa 1 (index 9)
        g.h[9] = 1; // devine 2 = pară -> capturat
        g.h[17] = 5;
        g.apply(&9);
        assert_eq!(g.kazan[0], 2);
        // tuzdyk: 2 -> 3
        let mut t = Toguz::new();
        t.h = [0; 18];
        t.h[8] = 1;
        t.h[2] = 4;
        t.h[9 + 3] = 0;
        t.h[9] = 2;
        t.h[17] = 4;
        t.apply(&9);
        assert_eq!(t.tuz[0], Some(0));
        assert_eq!(t.kazan[0], 3);
        // semănat în tuzdyk merge în kazan
        assert_eq!(t.tuz_owner(9), Some(0));
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=10u64 {
            let mut g: Box<dyn polytabletop_core::GameObj> = Box::new(Toguz::new());
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.play(&m[rng.below(m.len())]).unwrap();
                n += 1;
                assert!(n < 3000);
            }
        }
    }

    #[test]
    fn ai_is_legal_and_deterministic() {
        let mut g: Box<dyn polytabletop_core::GameObj> = Box::new(Toguz::new());
        for _ in 0..10 {
            let a = g.ai_move(7).unwrap();
            assert_eq!(a, g.ai_move(7).unwrap());
            g.play(&a).unwrap();
        }
    }
}
