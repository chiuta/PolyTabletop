//! Ayo (Ayoayo, Yoruba): 2 × 6 gropi, 4 semințe în fiecare (48 în total).
//!
//! Reguli folosite (descrierile publice ale jocului):
//! * Semănat în sens trigonometric, câte o sămânță în fiecare groapă; groapa
//!   din care semeni se sare (dacă semănatul face un tur complet, ea rămâne goală).
//! * Semănat în lanț: dacă ultima sămânță cade într-o groapă (de oricare parte)
//!   care avea deja semințe, le ridici pe toate și semeni mai departe, până
//!   când ultima sămânță cade într-o groapă goală.
//! * Captură: dacă ultima sămânță cade într-o groapă goală de pe partea ta și
//!   groapa opusă are semințe, le iei pe acelea (nu și sămânța ultimă, care
//!   rămâne). Pe partea adversă nu se capturează.
//! * Hrănire: dacă adversarul nu mai are semințe, ești obligat să muți astfel
//!   încât să-i dai semințe; dacă nu poți, jocul se termină și îți păstrezi
//!   semințele de pe partea ta.
//! * Dacă la mutare nu mai ai semințe, jocul se termină și adversarul ia ce a
//!   rămas pe partea lui. Câștigă cine are mai multe semințe capturate.
//!
//! Simplificări (documentate): penalizarea pentru „nehrănire” nu există (mutarea
//! de hrănire e pur și simplu obligatorie); lanțul de semănat e limitat la 2000
//! de semințe puse; după 500 de mutări fiecare ia semințele de pe partea lui.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const CAP: u32 = 2000;
const MAX_PLIES: u32 = 500;

#[derive(Clone)]
pub struct Ayo {
    pits: [u8; 12], // 0..=5 One (stânga→dreapta), 6..=11 Two (dreapta→stânga)
    store: [u8; 2],
    side: Player,
    plies: u32,
    over: bool,
}

impl Default for Ayo {
    fn default() -> Self {
        Self::new()
    }
}

impl Ayo {
    pub fn new() -> Self {
        Ayo {
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

    /// Semănat + relay + captură, fără schimbarea părții la mutare.
    fn sow(&mut self, m: u8) {
        let me = self.side;
        let mut origin = Self::range(me).start + (m as usize - 1);
        let mut seeds = self.pits[origin];
        self.pits[origin] = 0;
        let mut pos = origin;
        let mut sown = 0u32;
        loop {
            while seeds > 0 {
                pos = (pos + 1) % 12;
                if pos == origin {
                    continue;
                }
                self.pits[pos] += 1;
                seeds -= 1;
                sown += 1;
            }
            if self.pits[pos] > 1 {
                if sown > CAP {
                    return;
                }
                seeds = self.pits[pos];
                self.pits[pos] = 0;
                origin = pos;
                continue;
            }
            if Self::range(me).contains(&pos) {
                let opp = 11 - pos;
                self.store[me.idx()] += self.pits[opp];
                self.pits[opp] = 0;
            }
            return;
        }
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

impl Game for Ayo {
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
        if self.plies >= MAX_PLIES
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
        10 * (self.store[p.idx()] as i32 - self.store[p.other().idx()] as i32)
            + (sd(p) - sd(p.other()))
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
        s.push_str("  [..] = semințe capturate\n");
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

    fn custom(pits: [u8; 12], side: Player) -> Ayo {
        Ayo {
            pits,
            store: [0, 0],
            side,
            plies: 0,
            over: false,
        }
    }

    #[test]
    fn capture_excludes_last_seed() {
        let mut p = [0u8; 12];
        p[0] = 1; // cade în groapa 1 (goală); opusa e 10
        p[10] = 3;
        p[7] = 2;
        let mut g = custom(p, Player::One);
        g.apply(&1);
        assert_eq!(g.stores().0, 3);
        assert_eq!(g.pits[1], 1);
        assert_eq!(g.pits[10], 0);
    }

    #[test]
    fn no_capture_on_opponent_side() {
        let mut p = [0u8; 12];
        p[5] = 1; // cade în groapa 6 (Doi), goală
        p[0] = 1;
        p[5 + 1 + 4] = 3;
        let mut g = custom(p, Player::One);
        g.apply(&6);
        assert_eq!(g.stores().0, 0);
        assert_eq!(g.pits[6], 1);
    }

    #[test]
    fn relay_continues_on_occupied_pit() {
        let mut p = [0u8; 12];
        p[0] = 1;
        p[1] = 1; // devine 2 → ridicat, semănat în 2 și 3 (3 goală)
        p[9] = 2;
        let mut g = custom(p, Player::One);
        g.apply(&1);
        assert_eq!(g.pits[1], 0);
        assert_eq!(g.pits[2], 1);
        // ultima în groapa 3 goală, opusa 8 goală → fără captură
        assert_eq!(g.pits[3], 1);
    }

    #[test]
    fn full_lap_conserves_seeds() {
        let mut p = [0u8; 12];
        p[0] = 12;
        let mut g = custom(p, Player::One);
        g.sow(1);
        assert_eq!(g.total_seeds(), 12);
    }

    #[test]
    fn must_feed_an_empty_opponent() {
        let mut p = [0u8; 12];
        p[0] = 1; // nu hrănește
        p[5] = 1; // hrănește (cade în groapa 6)
        let g = custom(p, Player::One);
        assert_eq!(Game::legal_moves(&g), vec![6]);
    }

    #[test]
    fn game_ends_when_no_feeding_move_exists() {
        let mut p = [0u8; 12];
        p[0] = 1;
        p[11] = 1;
        let mut g = custom(p, Player::Two);
        g.apply(&6);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
        assert_eq!(g.total_seeds(), 2);
        assert_eq!(g.stores().0, 2);
    }

    #[test]
    fn seeds_conserved_and_random_games_finish() {
        for seed in 1..=40u64 {
            let mut g = Ayo::new();
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
