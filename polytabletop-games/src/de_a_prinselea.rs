//! De-a prinselea (Tag), 2 jucători — urmărire pe un câmp 7×7.
//!
//! * J1 pornește ca «prinzător» (în colțul a1), J2 e fugarul (g7). Fiecare mutare: un pas ortogonal sau
//!   pe diagonală (`n`, `s`, `e`, `v`, `ne`, `nv`, `se`, `sv`) sau `stai`.
//! * Dacă prinzătorul termină mutarea adiacent (inclusiv diagonal) de celălalt, acesta e «prins»:
//!   rolurile se schimbă, iar noul prinzător stă pe loc o mutare («numărătoare»).
//! * După fiecare rundă (ambii au mutat) cel care nu e prinzător primește 1 punct. 15 runde; mai multe
//!   puncte câștigă, egal = remiză.
//!
//! Mutări: `n`, `s`, `e`, `v`, `ne`, `nv`, `se`, `sv`, `stai`.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: i32 = 7;
const ROUNDS: u8 = 15;
const DIRS: [(&str, i32, i32); 8] = [
    ("n", 0, -1),
    ("s", 0, 1),
    ("e", 1, 0),
    ("v", -1, 0),
    ("ne", 1, -1),
    ("nv", -1, -1),
    ("se", 1, 1),
    ("sv", -1, 1),
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Step(u8),
    Stay,
}

#[derive(Clone)]
pub struct Prinselea {
    pos: [(i32, i32); 2],
    it: Player,
    frozen: bool,
    side: Player,
    round: u8,
    score: [i32; 2],
    log: String,
}

impl Default for Prinselea {
    fn default() -> Self {
        Self::new()
    }
}

impl Prinselea {
    pub fn new() -> Self {
        Prinselea {
            pos: [(0, 0), (N - 1, N - 1)],
            it: Player::One,
            frozen: false,
            side: Player::One,
            round: 0,
            score: [0, 0],
            log: String::new(),
        }
    }
    fn dist(&self) -> i32 {
        let (a, b) = (self.pos[0], self.pos[1]);
        (a.0 - b.0).abs().max((a.1 - b.1).abs())
    }
}

impl Game for Prinselea {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.round >= ROUNDS {
            return vec![];
        }
        if self.frozen && self.side == self.it {
            return vec![Move::Stay];
        }
        let p = self.pos[self.side.idx()];
        let o = self.pos[self.side.other().idx()];
        let mut v = vec![Move::Stay];
        for (i, d) in DIRS.iter().enumerate() {
            let q = (p.0 + d.1, p.1 + d.2);
            if (0..N).contains(&q.0) && (0..N).contains(&q.1) && q != o {
                v.push(Move::Step(i as u8));
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        let me = self.side;
        if let Move::Step(i) = m {
            let d = DIRS[*i as usize];
            let p = &mut self.pos[me.idx()];
            *p = (p.0 + d.1, p.1 + d.2);
        }
        if self.frozen && me == self.it {
            self.frozen = false;
        } else if me == self.it && self.dist() <= 1 {
            self.it = me.other();
            self.frozen = true;
            self.log = format!(
                "J{} a fost prins! Acum prinde J{} (stă o mutare).",
                me.other().idx() + 1,
                me.other().idx() + 1
            );
        }
        if me == Player::Two {
            self.score[self.it.other().idx()] += 1;
            self.round += 1;
        }
        self.side = me.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.round < ROUNDS {
            return None;
        }
        Some(match self.score[0].cmp(&self.score[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let mut v = (self.score[p.idx()] - self.score[p.other().idx()]) * 100;
        let d = self.dist();
        if self.it == p {
            v -= d * 8 + if self.frozen { 20 } else { 0 };
        } else {
            v += d * 8;
        }
        v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "De-a prinselea — runda {}/{ROUNDS}, scor J1 {} – J2 {}\n",
            (self.round + 1).min(ROUNDS),
            self.score[0],
            self.score[1]
        );
        for y in 0..N {
            for x in 0..N {
                let c = if self.pos[0] == (x, y) {
                    if self.it == Player::One {
                        '1'
                    } else {
                        'a'
                    }
                } else if self.pos[1] == (x, y) {
                    if self.it == Player::Two {
                        '2'
                    } else {
                        'b'
                    }
                } else {
                    '.'
                };
                s.push(c);
                s.push(' ');
            }
            s.push('\n');
        }
        s.push_str("(cifră = prinzătorul, literă = fugarul: a = J1, b = J2)\n");
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        match self.outcome() {
            Some(Outcome::Win(p)) => s.push_str(&format!("Jucătorul {} câștigă.\n", p.idx() + 1)),
            Some(Outcome::Draw) => s.push_str("Remiză.\n"),
            None => s.push_str(&format!(
                "Prinzător: J{} | la mutare: Jucătorul {}\n",
                self.it.idx() + 1,
                self.side.idx() + 1
            )),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Step(i) => DIRS[*i as usize].0.into(),
            Move::Stay => "stai".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    #[test]
    fn random_game_ends() {
        let mut g = Prinselea::new();
        let mut r = Rng::new(3);
        let mut n = 0;
        while g.outcome().is_none() {
            let m = g.legal_moves();
            g.apply(&m[r.below(m.len())]);
            n += 1;
        }
        assert_eq!(n, ROUNDS as usize * 2);
    }

    #[test]
    fn tagging_swaps_roles_and_freezes() {
        let mut g = Prinselea::new();
        g.pos = [(2, 2), (4, 2)];
        g.apply(&Move::Step(2)); // J1 e -> (3,2), adiacent
        assert_eq!(g.it, Player::Two);
        assert!(g.frozen);
        // J2 (nou prinzător) e înghețat: o singură mutare
        assert_eq!(g.legal_moves(), vec![Move::Stay]);
    }
}
