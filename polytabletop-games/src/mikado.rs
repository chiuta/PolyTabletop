#![allow(clippy::manual_is_multiple_of, clippy::needless_range_loop)]
//! Mikado (Pick-up sticks) — adaptare pe ture pentru 2 jucători.
//!
//! Un morman de 21 de bețișoare cade într-o grămadă: bețișoarele sunt așezate în straturi și fiecare stă
//! peste cel mult două dintre cele de sub el. Valori (ca la Mikado): 1 Mikado (20), 3 Mandarini (10), 5 Bonzi (5),
//! 6 Samurai (3), 6 Kuli (2). La mutare: `ia N` ridică bețișorul N. Dacă **nu are niciun bețișor deasupra** reușește
//! sigur și rămâi la rând; dacă este acoperit de `k` bețișoare, riști: mutarea forțată `incerc` hotărăște la
//! întâmplare (reușită 50% pentru k=1, 25% pentru k=2, 12% pentru k=3 …) dacă mormanul stă nemișcat. La
//! eșec se pierde rândul și bețișorul rămâne. `pas` încheie rândul după ce ai ridicat cel puțin un bețișor
//! (păstrezi punctele). Când mormanul e gol, câștigă cine are mai multe puncte. Bețișorul Mikado, odată
//! luat, nu schimbă regulile (simplificare: fără folosirea Mikado-ului ca pârghie).
//! Nu există informație ascunsă; riscul se rezolvă într-o mutare forțată, deci AI-ul nu vede rezultatul în avans.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: usize = 21;
const VALUES: [i32; N] = [
    20, 10, 10, 10, 5, 5, 5, 5, 5, 3, 3, 3, 3, 3, 3, 2, 2, 2, 2, 2, 2,
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Take(u8),
    Try,
    Pass,
}

#[derive(Clone)]
pub struct Mikado {
    rng: Rng,
    /// peste cine stă fiecare bețișor: `covers[i]` = masca bețișoarelor care stau PE i
    on_top: [u32; N],
    left: u32,
    scores: [i32; 2],
    side: Player,
    picked: u8,
    pending: Option<u8>,
    log: String,
}

impl Default for Mikado {
    fn default() -> Self {
        Self::new()
    }
}

impl Mikado {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        // ordine de cădere aleatoare: bețișorul care cade mai târziu stă peste 0–2 din cele căzute înainte
        let mut order: Vec<usize> = (0..N).collect();
        for i in (1..N).rev() {
            let j = rng.below(i + 1);
            order.swap(i, j);
        }
        let mut on_top = [0u32; N];
        for k in 1..N {
            let me = order[k];
            let lies_on = rng.below(3); // 0, 1 sau 2
            let mut chosen = 0;
            let mut tries = 0;
            while chosen < lies_on && tries < 10 {
                let below = order[rng.below(k)];
                if on_top[below] & (1 << me) == 0 {
                    on_top[below] |= 1 << me;
                    chosen += 1;
                }
                tries += 1;
            }
        }
        Mikado {
            rng,
            on_top,
            left: (1u32 << N) - 1,
            scores: [0; 2],
            side: Player::One,
            picked: 0,
            pending: None,
            log: String::new(),
        }
    }

    pub fn scores(&self) -> (i32, i32) {
        (self.scores[0], self.scores[1])
    }

    fn cover_count(&self, i: usize) -> u32 {
        (self.on_top[i] & self.left).count_ones()
    }

    fn free_chance(&self, k: u32) -> usize {
        // procent de reușită: 100, 50, 25, 12, ...
        (100usize >> k).max(5)
    }

    fn remove(&mut self, i: usize) {
        self.left &= !(1 << i);
    }
}

impl Game for Mikado {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.left == 0 {
            return Vec::new();
        }
        if self.pending.is_some() {
            return vec![Move::Try];
        }
        let mut v: Vec<Move> = (0..N)
            .filter(|&i| self.left & (1 << i) != 0)
            .map(|i| Move::Take(i as u8 + 1))
            .collect();
        if self.picked > 0 {
            v.push(Move::Pass);
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        let me = self.side.idx();
        match *m {
            Move::Take(n) => {
                let i = n as usize - 1;
                if self.cover_count(i) == 0 {
                    self.remove(i);
                    self.scores[me] += VALUES[i];
                    self.picked += 1;
                    self.log = format!("J{} ridică {} (+{})", me + 1, n, VALUES[i]);
                } else {
                    self.pending = Some(i as u8);
                }
            }
            Move::Try => {
                let i = self.pending.take().unwrap() as usize;
                let k = self.cover_count(i);
                if self.rng.below(100) < self.free_chance(k) {
                    self.remove(i);
                    self.scores[me] += VALUES[i];
                    self.picked += 1;
                    self.log = format!("J{} ridică riscant {} (+{})", me + 1, i + 1, VALUES[i]);
                } else {
                    self.log = format!("Mormanul s-a mișcat! J{} pierde rândul", me + 1);
                    self.picked = 0;
                    self.side = self.side.other();
                }
            }
            Move::Pass => {
                self.picked = 0;
                self.side = self.side.other();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.left != 0 {
            return None;
        }
        Some(match self.scores[0].cmp(&self.scores[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let mut v = (self.scores[p.idx()] - self.scores[p.other().idx()]) * 100;
        if let Some(i) = self.pending {
            // valoare așteptată a riscului: câștigi bețișorul cu probabilitatea de reușită
            let i = i as usize;
            let ch = self.free_chance(self.cover_count(i)) as i32;
            let sign = if self.side == p { 1 } else { -1 };
            v += sign * VALUES[i] * ch;
        }
        // cine are rândul păstrează inițiativa
        if self.pending.is_none() && self.side == p {
            v += 1;
        }
        v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Mikado — scor: J1 {}, J2 {}\n",
            self.scores[0], self.scores[1]
        );
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        let mut rows = Vec::new();
        for i in 0..N {
            if self.left & (1 << i) != 0 {
                let k = self.cover_count(i);
                rows.push(format!(
                    "{}:{}{}",
                    i + 1,
                    VALUES[i],
                    if k == 0 {
                        "".to_string()
                    } else {
                        format!("(sub {k})")
                    }
                ));
            }
        }
        s.push_str(&format!(
            "Bețișoare (nr:valoare, «sub k» = acoperit de k): {}\n",
            rows.join(" ")
        ));
        if self.left != 0 {
            if self.pending.is_some() {
                s.push_str("Risc! Scrie «incerc».\n");
            }
            s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Take(n) => format!("ia {n}"),
            Move::Try => "incerc".into(),
            Move::Pass => "pas".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_stick_is_free_and_total_value() {
        for seed in 1..20u64 {
            let g = Mikado::with_seed(seed);
            assert!((0..N).any(|i| g.cover_count(i) == 0), "seed {seed}");
            assert_eq!(VALUES.iter().sum::<i32>(), 20 + 30 + 25 + 18 + 12);
        }
    }

    #[test]
    fn free_pick_keeps_turn_and_pass_needs_pick() {
        let mut g = Mikado::with_seed(3);
        assert!(!g.legal_moves().contains(&Move::Pass));
        let i = (0..N).find(|&i| g.cover_count(i) == 0).unwrap();
        g.apply(&Move::Take(i as u8 + 1));
        assert_eq!(g.to_move(), Player::One);
        assert!(g.legal_moves().contains(&Move::Pass));
        assert_eq!(g.scores[0], VALUES[i]);
        g.apply(&Move::Pass);
        assert_eq!(g.to_move(), Player::Two);
    }

    #[test]
    fn covered_pick_is_risky() {
        let mut g = Mikado::with_seed(4);
        let i = (0..N).find(|&i| g.cover_count(i) > 0).unwrap();
        g.apply(&Move::Take(i as u8 + 1));
        assert_eq!(g.legal_moves(), vec![Move::Try]);
        assert_eq!(g.scores[0], 0);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=6u64 {
            let mut g = Mikado::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut k = 0;
            while g.outcome().is_none() && k < 4000 {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
                k += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed}");
            assert_eq!(g.scores[0] + g.scores[1], 105);
        }
    }
}
