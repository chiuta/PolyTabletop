#![allow(clippy::needless_range_loop, clippy::manual_range_contains)]
//! Uckers (Marea Britanie, Royal Navy) — Ludo marinăresc, 2 jucători, 4 piese fiecare, un zar.
//!
//! * Pista comună are 48 de pătrate; J1 pornește de la pătratul 0, J2 de la 24. Progres: 0 = în bază,
//!   1..=48 = pe pistă, 49..=53 = coloana de acasă (5 pătrate), 54 = acasă (se atinge exact).
//! * Cu un **6** scoți o piesă din bază pe pătratul de start. Un 6 dă încă o aruncare; la fel o
//!   **captură**. Fără piese pe tablă ai trei încercări să arunci 6.
//! * Aterizarea pe o piesă singură a adversarului pe pistă o trimite în bază. **Piggyback**: două
//!   (sau mai multe) piese proprii pe același pătrat formează un bloc — nu poate fi capturat, nici
//!   aterizat, nici trecut de adversar. Piesele proprii se pot suprapune liber.
//! * Mutarea se scrie prin progresul piesei (`12`) sau `intră` pentru o piesă din bază. Dacă nu ai
//!   nicio mutare, tura trece. Primul cu toate cele 4 piese acasă câștigă.
//! * Zarul se aruncă în starea jocului; AI-ul nu ține cont de aruncările viitoare.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const TRACK: u8 = 48;
const GOAL: u8 = 54;

#[derive(Clone)]
pub struct Uckers {
    prog: [[u8; 4]; 2],
    side: Player,
    roll: u8,
    rng: Rng,
    over: bool,
    log: String,
}

impl Default for Uckers {
    fn default() -> Self {
        Self::new()
    }
}

fn start_off(p: Player) -> u8 {
    match p {
        Player::One => 0,
        Player::Two => 24,
    }
}

fn square(p: Player, progress: u8) -> u8 {
    (start_off(p) + progress - 1) % TRACK
}

impl Uckers {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Uckers {
            prog: [[0; 4]; 2],
            side: Player::One,
            roll: 0,
            rng: Rng::new(seed),
            over: false,
            log: String::new(),
        };
        g.begin_turn();
        g
    }

    pub fn pieces(&self, p: Player) -> [u8; 4] {
        self.prog[p.idx()]
    }

    /// Câte piese ale lui `q` sunt pe pătratul de pistă `sq`.
    fn on_square(&self, q: Player, sq: u8) -> usize {
        self.prog[q.idx()]
            .iter()
            .filter(|&&p| (1..=TRACK).contains(&p) && square(q, p) == sq)
            .count()
    }

    fn blocked(&self, from: u8, to: u8) -> bool {
        let o = self.side.other();
        for step in from + 1..=to.min(TRACK) {
            if step == 0 {
                continue;
            }
            if self.on_square(o, square(self.side, step)) >= 2 {
                return true;
            }
        }
        false
    }

    fn moves(&self) -> Vec<u8> {
        let mine = &self.prog[self.side.idx()];
        let mut out: Vec<u8> = Vec::new();
        for &pr in mine {
            if out.contains(&pr) || pr == GOAL {
                continue;
            }
            let target = if pr == 0 {
                if self.roll != 6 {
                    continue;
                }
                1
            } else {
                pr + self.roll
            };
            if target > GOAL {
                continue;
            }
            let from = if pr == 0 { 0 } else { pr };
            if self.blocked(from, target) {
                continue;
            }
            out.push(pr);
        }
        out.sort_unstable();
        out
    }

    fn begin_turn(&mut self) {
        loop {
            let on_board = self.prog[self.side.idx()]
                .iter()
                .any(|&p| (1..GOAL).contains(&p));
            let tries = if on_board { 1 } else { 3 };
            for _ in 0..tries {
                self.roll = self.rng.d6();
                if !self.moves().is_empty() {
                    return;
                }
            }
            self.log.push_str(&format!(
                " J{} aruncă {} și nu poate muta.",
                self.side.idx() + 1,
                self.roll
            ));
            self.side = self.side.other();
        }
    }
}

impl Game for Uckers {
    type Move = u8;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<u8> {
        if self.over {
            Vec::new()
        } else {
            self.moves()
        }
    }

    fn apply(&mut self, m: &u8) {
        let me = self.side;
        let mi = me.idx();
        let target = if *m == 0 { 1 } else { *m + self.roll };
        let k = self.prog[mi].iter().position(|&p| p == *m).expect("piesă");
        self.prog[mi][k] = target;
        self.log = format!("J{}: {} cu {}.", mi + 1, self.format_move(m), self.roll);
        let mut extra = self.roll == 6;
        if target <= TRACK {
            let sq = square(me, target);
            let o = me.other();
            for p in self.prog[o.idx()].iter_mut() {
                if (1..=TRACK).contains(p) && square(o, *p) == sq {
                    *p = 0;
                    self.log.push_str(" Captură!");
                    extra = true;
                }
            }
        }
        if self.prog[mi].iter().all(|&p| p == GOAL) {
            self.over = true;
            return;
        }
        if !extra {
            self.side = me.other();
        } else {
            self.log.push_str(" Aruncă din nou.");
        }
        self.begin_turn();
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            return None;
        }
        let one = self.prog[0].iter().all(|&p| p == GOAL);
        Some(Outcome::Win(if one { Player::One } else { Player::Two }))
    }

    fn evaluate(&self, p: Player) -> i32 {
        let s = |q: Player| -> i32 {
            let mut v: i32 = self.prog[q.idx()]
                .iter()
                .map(|&x| x as i32 + if x == GOAL { 10 } else { 0 })
                .sum();
            // bonus pentru piggyback
            for sq in 0..TRACK {
                if self.on_square(q, sq) >= 2 {
                    v += 6;
                }
            }
            v
        };
        s(p) - s(p.other())
    }

    fn render(&self) -> String {
        let mut ring = vec!['.'; TRACK as usize];
        for (pl, ch) in [(Player::One, 'X'), (Player::Two, 'O')] {
            for &p in &self.prog[pl.idx()] {
                if (1..=TRACK).contains(&p) {
                    let sq = square(pl, p) as usize;
                    ring[sq] = if ring[sq] == ch {
                        ch.to_ascii_lowercase()
                    } else {
                        ch
                    };
                }
            }
        }
        let row = |a: usize, b: usize| ring[a..b].iter().collect::<String>();
        let mut s = String::new();
        s.push_str("Pista (48 pătrate; X pornește la 0, O la 24; literă mică = piggyback):\n");
        s.push_str(&format!(
            "   0-23 : {}\n  24-47 : {}\n",
            row(0, 24),
            row(24, 48)
        ));
        for (pl, ch) in [(Player::One, 'X'), (Player::Two, 'O')] {
            let v = self.prog[pl.idx()];
            s.push_str(&format!(
                "Jucător {} ({}): bază {}, acasă {}, progres piese {:?}\n",
                pl.idx() + 1,
                ch,
                v.iter().filter(|&&p| p == 0).count(),
                v.iter().filter(|&&p| p == GOAL).count(),
                v.iter().filter(|&&p| p != 0).collect::<Vec<_>>()
            ));
        }
        if !self.over {
            s.push_str(&format!(
                "La mutare: jucătorul {}   Zar: {}\n",
                self.side.idx() + 1,
                self.roll
            ));
        }
        s.push_str(self.log.trim());
        s.push('\n');
        s
    }

    fn format_move(&self, m: &u8) -> String {
        if *m == 0 {
            "intră".to_string()
        } else {
            m.to_string()
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set(g: &mut Uckers, side: Player, roll: u8, one: [u8; 4], two: [u8; 4]) {
        g.prog = [one, two];
        g.side = side;
        g.roll = roll;
    }

    #[test]
    fn six_enters_and_gives_extra_roll() {
        let mut g = Uckers::with_seed(1);
        set(&mut g, Player::One, 6, [0, 0, 0, 0], [0, 0, 0, 0]);
        assert_eq!(g.moves(), vec![0]);
        set(&mut g, Player::One, 3, [0, 0, 0, 0], [0, 0, 0, 0]);
        assert!(g.moves().is_empty());
    }

    #[test]
    fn single_piece_is_captured_block_is_not() {
        let mut g = Uckers::with_seed(1);
        // J2 progres 1 stă pe pătratul 24; J1 progres 25 → pătratul 24
        set(&mut g, Player::One, 2, [23, 0, 0, 0], [1, 0, 0, 0]);
        g.apply(&23);
        assert_eq!(g.prog[1][0], 0, "captură");
        // bloc: două piese J2 pe 24 → J1 nu poate ateriza/trece
        let mut h = Uckers::with_seed(1);
        set(&mut h, Player::One, 2, [23, 0, 0, 0], [1, 1, 0, 0]);
        assert!(h.moves().is_empty());
        set(&mut h, Player::One, 4, [22, 0, 0, 0], [1, 1, 0, 0]);
        assert!(h.moves().is_empty(), "nu se trece peste bloc");
    }

    #[test]
    fn exact_goal() {
        let mut g = Uckers::with_seed(1);
        set(&mut g, Player::One, 5, [50, 54, 54, 54], [0, 0, 0, 0]);
        assert!(g.moves().is_empty());
        set(&mut g, Player::One, 4, [50, 54, 54, 54], [0, 0, 0, 0]);
        assert_eq!(g.moves(), vec![50]);
        g.apply(&50);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=6u64 {
            let mut g = Uckers::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() && n < 20000 {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed}");
            assert!(n < 3500, "seed {seed}: {n}");
        }
    }
}
