//! Oware (Ghana / Africa de Vest, varianta Abapa): 2 × 6 gropi, 4 semințe în fiecare.
//!
//! Reguli folosite (regulile publice ale jocului, varianta de turneu):
//! * Semănat în sens trigonometric, câte o sămânță în fiecare groapă; dacă groapa de
//!   start are 12 sau mai multe semințe, ea este sărită.
//! * Captură: dacă ultima sămânță cade într-o groapă ADVERSĂ care ajunge la 2 sau 3
//!   semințe, le iei; apoi te uiți la groapa anterioară (tot a adversarului) și
//!   continui cât timp are 2 sau 3 semințe.
//! * „Grand slam”: o mutare care ar lua toate semințele adversarului este legală, dar
//!   nu capturează nimic.
//! * Hrănire: dacă adversarul nu are semințe, trebuie să muți astfel încât să-i dai;
//!   dacă nu poți, jocul se termină și iei semințele de pe partea ta.
//! * Câștigă primul care capturează 25 de semințe (24–24 = remiză). Dacă la mutare
//!   nu mai ai semințe, adversarul își ia semințele rămase.
//!
//! Spre deosebire de `ayo` (semănat în lanț, captură pe partea proprie), Oware nu
//! are lanț; de aceea e un motor separat. Simplificare: după 400 de mutări (poziție
//! care se repetă practic) fiecare ia semințele de pe partea lui.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const MAX_PLIES: u32 = 400;

#[derive(Clone)]
pub struct Oware {
    pits: [u8; 12], // 0..=5 Unu (stânga→dreapta), 6..=11 Doi (dreapta→stânga)
    store: [u8; 2],
    side: Player,
    plies: u32,
    over: bool,
}

impl Default for Oware {
    fn default() -> Self {
        Self::new()
    }
}

impl Oware {
    pub fn new() -> Self {
        Oware {
            pits: [4; 12],
            store: [0, 0],
            side: Player::One,
            plies: 0,
            over: false,
        }
    }

    fn range(p: Player) -> std::ops::Range<usize> {
        match p {
            Player::One => 0..6,
            Player::Two => 6..12,
        }
    }

    pub fn stores(&self) -> (u8, u8) {
        (self.store[0], self.store[1])
    }

    pub fn total_seeds(&self) -> u32 {
        self.pits.iter().map(|&x| x as u32).sum::<u32>()
            + self.store.iter().map(|&x| x as u32).sum::<u32>()
    }

    fn side_empty(&self, p: Player) -> bool {
        self.pits[Self::range(p)].iter().all(|&x| x == 0)
    }

    /// Semănat + captură, fără schimbarea părții la mutare.
    fn sow(&mut self, m: u8) {
        let me = self.side;
        let origin = Self::range(me).start + (m as usize - 1);
        let mut seeds = self.pits[origin];
        self.pits[origin] = 0;
        let mut pos = origin;
        while seeds > 0 {
            pos = (pos + 1) % 12;
            if pos == origin {
                continue;
            }
            self.pits[pos] += 1;
            seeds -= 1;
        }
        let opp = Self::range(me.other());
        if !opp.contains(&pos) {
            return;
        }
        // grand slam: simulăm captura și o anulăm dacă ar goli partea adversă
        let mut taken = 0u32;
        let mut q = pos;
        let mut snapshot = self.pits;
        while opp.contains(&q) && (self.pits[q] == 2 || self.pits[q] == 3) {
            taken += self.pits[q] as u32;
            snapshot[q] = 0;
            if q == 0 {
                break;
            }
            q -= 1;
        }
        if taken == 0 {
            return;
        }
        if snapshot[opp.clone()].iter().all(|&x| x == 0) {
            return; // grand slam: nicio captură
        }
        self.pits = snapshot;
        self.store[me.idx()] += taken as u8;
    }

    fn feeds(&self, m: u8) -> bool {
        let mut c = self.clone();
        c.sow(m);
        !c.side_empty(self.side.other())
    }

    fn finish(&mut self) {
        for p in [Player::One, Player::Two] {
            let rest: u8 = self.pits[Self::range(p)].iter().sum();
            for i in Self::range(p) {
                self.pits[i] = 0;
            }
            self.store[p.idx()] += rest;
        }
        self.over = true;
    }

    fn movable(&self) -> Vec<u8> {
        let start = Self::range(self.side).start;
        (0..6u8)
            .filter(|&k| self.pits[start + k as usize] > 0)
            .map(|k| k + 1)
            .collect()
    }
}

impl Game for Oware {
    type Move = u8;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<u8> {
        if self.over {
            return Vec::new();
        }
        let all = self.movable();
        if self.side_empty(self.side.other()) {
            all.into_iter().filter(|&m| self.feeds(m)).collect()
        } else {
            all
        }
    }

    fn apply(&mut self, m: &u8) {
        self.sow(*m);
        self.plies += 1;
        self.side = self.side.other();
        if self.store[0] >= 25
            || self.store[1] >= 25
            || self.plies >= MAX_PLIES
            || self.side_empty(self.side)
            || (self.side_empty(self.side.other()) && self.legal_moves().is_empty())
        {
            self.finish();
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            return None;
        }
        let (a, b) = self.stores();
        Some(if a > b {
            Outcome::Win(Player::One)
        } else if b > a {
            Outcome::Win(Player::Two)
        } else {
            Outcome::Draw
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let sd = |q: Player| -> i32 { self.pits[Self::range(q)].iter().map(|&x| x as i32).sum() };
        let vulnerable = |q: Player| -> i32 {
            self.pits[Self::range(q)]
                .iter()
                .filter(|&&x| x == 1 || x == 2)
                .count() as i32
        };
        10 * (self.store[p.idx()] as i32 - self.store[p.other().idx()] as i32)
            + (sd(p) - sd(p.other()))
            - 2 * (vulnerable(p) - vulnerable(p.other()))
    }

    fn render(&self) -> String {
        let p = &self.pits;
        let mut s = String::new();
        s.push_str("  Jucător 2 (O):   6   5   4   3   2   1\n");
        s.push_str(&format!(
            "       [{:>2}]  {:>2}  {:>2}  {:>2}  {:>2}  {:>2}  {:>2}\n",
            self.store[1], p[11], p[10], p[9], p[8], p[7], p[6]
        ));
        s.push_str(&format!(
            "              {:>2}  {:>2}  {:>2}  {:>2}  {:>2}  {:>2}   [{:>2}]\n",
            p[0], p[1], p[2], p[3], p[4], p[5], self.store[0]
        ));
        s.push_str("  Jucător 1 (X):   1   2   3   4   5   6\n");
        s.push_str("  [..] = semințe capturate (25 câștigă)\n");
        s
    }

    fn format_move(&self, m: &u8) -> String {
        m.to_string()
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(7)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    fn custom(pits: [u8; 12], side: Player) -> Oware {
        Oware {
            pits,
            store: [0, 0],
            side,
            plies: 0,
            over: false,
        }
    }

    #[test]
    fn capture_chain_goes_backwards() {
        let mut p = [0u8; 12];
        p[5] = 3; // 6→7,8,9 (indici 6,7,8)
        p[6] = 1;
        p[7] = 1;
        p[8] = 1;
        p[11] = 5; // ca partea adversă să nu rămână goală
        let mut g = custom(p, Player::One);
        g.apply(&6);
        // 6,7,8 devin 2,2,2 → capturate toate trei = 6
        assert_eq!(g.stores().0, 6);
        assert_eq!(g.pits[6], 0);
        assert_eq!(g.pits[8], 0);
    }

    #[test]
    fn capture_stops_at_non_2_3() {
        let mut p = [0u8; 12];
        p[5] = 2;
        p[6] = 1; // devine 2
        p[7] = 4;
        p[11] = 1;
        let mut g = custom(p, Player::One);
        g.apply(&6);
        // ultima în 7 (index 7): 4+1=5 → fără captură
        assert_eq!(g.stores().0, 0);
        let mut p = [0u8; 12];
        p[5] = 2;
        p[6] = 1;
        p[7] = 1; // 2
        p[11] = 1;
        let mut g = custom(p, Player::One);
        g.apply(&6);
        // 6 → 2, 7 → 2: capturează 7 și 6 (lanț înapoi)
        assert_eq!(g.stores().0, 4);
    }

    #[test]
    fn no_capture_on_own_side() {
        let mut p = [0u8; 12];
        p[0] = 1;
        p[1] = 1;
        p[11] = 1;
        let mut g = custom(p, Player::One);
        g.apply(&1);
        assert_eq!(g.stores().0, 0);
    }

    #[test]
    fn grand_slam_captures_nothing() {
        let mut p = [0u8; 12];
        p[5] = 1;
        p[6] = 1; // devine 2 și e singura groapă a adversarului
        let mut g = custom(p, Player::One);
        g.apply(&6);
        // niciuna nu e capturată de J1; jocul se termină (Doi nu poate hrăni), Doi își ia semințele
        assert_eq!(g.stores(), (0, 2));
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
    }

    #[test]
    fn lap_skips_origin() {
        let mut p = [0u8; 12];
        p[0] = 12;
        p[11] = 1;
        let mut g = custom(p, Player::One);
        g.sow(1);
        assert_eq!(g.pits[0], 0);
        assert_eq!(g.total_seeds(), 13);
        assert_eq!(g.pits[1], 2);
    }

    #[test]
    fn must_feed_an_empty_opponent() {
        let mut p = [0u8; 12];
        p[0] = 1;
        p[5] = 1;
        let g = custom(p, Player::One);
        assert_eq!(Game::legal_moves(&g), vec![6]);
    }

    #[test]
    fn winner_at_25() {
        let mut p = [0u8; 12];
        p[5] = 1;
        p[6] = 1;
        p[11] = 3;
        let mut g = custom(p, Player::One);
        g.store[0] = 23;
        g.apply(&6);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn seeds_conserved_and_random_games_finish() {
        for seed in 1..=40u64 {
            let mut g = Oware::new();
            let mut rng = Rng::new(seed);
            while g.outcome().is_none() {
                let m = Game::legal_moves(&g);
                assert!(!m.is_empty(), "{}", g.render());
                g.apply(&m[rng.below(m.len())]);
                assert_eq!(g.total_seeds(), 48);
            }
        }
    }
}
