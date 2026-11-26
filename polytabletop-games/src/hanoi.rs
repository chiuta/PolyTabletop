//! Turnurile din Hanoi — cursă între doi jucători, fiecare pe turnul lui (5 discuri).
//!
//! * Trei tije (a, b, c); un disc mai mare nu se așază peste unul mai mic. Fiecare jucător are
//!   propriul turn cu 5 discuri pe tija a și trebuie să-l mute pe tija c (minimum 31 de mutări).
//!   Mutare: `a-c` (discul de sus de pe tija a trece pe c).
//! * Jucătorii mută pe rând, fiecare doar în turnul lui. Când unul își termină turnul, celălalt
//!   mai primește mutarea rămasă din tura egală; câștigă cel cu mai puține mutări (egal = remiză).
//!   Un AI optim împotriva unui om optim = remiză. După 200 de mutări fără finalizare câștigă cine
//!   mai are de mutat mai puțin.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: usize = 5;
/// După atâtea mutări pe jucător, fără turn gata, câștigă cine are de mutat mai puțin.
const CAP: u32 = 200;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Move(pub u8, pub u8);

#[derive(Clone)]
pub struct Hanoi {
    /// peg[i] = tija discului i (0 = cel mai mic)
    peg: [[u8; N]; 2],
    moves: [u32; 2],
    side: Player,
}

impl Default for Hanoi {
    fn default() -> Self {
        Self::new()
    }
}

/// Număr minim de mutări ca toate discurile 0..=k să ajungă pe `target`.
pub fn dist(peg: &[u8; N], k: usize, target: u8) -> u32 {
    if k == usize::MAX {
        return 0;
    }
    let kk = k as isize;
    if peg[k] == target {
        if k == 0 {
            0
        } else {
            dist(peg, k - 1, target)
        }
    } else {
        let other = 3 - peg[k] - target;
        let below = if k == 0 { 0 } else { dist(peg, k - 1, other) };
        below + 1 + ((1u32 << kk) - 1)
    }
}

impl Hanoi {
    pub fn new() -> Self {
        Hanoi {
            peg: [[0; N]; 2],
            moves: [0, 0],
            side: Player::One,
        }
    }

    fn top(&self, p: usize, t: u8) -> Option<usize> {
        (0..N).find(|&d| self.peg[p][d] == t)
    }

    fn solved(&self, p: usize) -> bool {
        self.peg[p].iter().all(|&x| x == 2)
    }

    pub fn remaining(&self, p: Player) -> u32 {
        dist(&self.peg[p.idx()], N - 1, 2)
    }
}

impl Game for Hanoi {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        let p = self.side.idx();
        let mut v = Vec::new();
        for a in 0..3u8 {
            for b in 0..3u8 {
                if a == b {
                    continue;
                }
                if let Some(d) = self.top(p, a) {
                    if self.top(p, b).is_none_or(|e| e > d) {
                        v.push(Move(a, b));
                    }
                }
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        let p = self.side.idx();
        let d = self.top(p, m.0).unwrap();
        self.peg[p][d] = m.1;
        self.moves[p] += 1;
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.side != Player::One {
            return None;
        }
        let (a, b) = (self.solved(0), self.solved(1));
        if a || b {
            return Some(match (a, b) {
                (true, true) => Outcome::Draw,
                (true, false) => Outcome::Win(Player::One),
                _ => Outcome::Win(Player::Two),
            });
        }
        if self.moves[0] >= CAP {
            let (x, y) = (self.remaining(Player::One), self.remaining(Player::Two));
            return Some(match x.cmp(&y) {
                std::cmp::Ordering::Less => Outcome::Win(Player::One),
                std::cmp::Ordering::Greater => Outcome::Win(Player::Two),
                std::cmp::Ordering::Equal => Outcome::Draw,
            });
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let (me, op) = (p, p.other());
        (self.remaining(op) as i32 - self.remaining(me) as i32) * 10
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Turnurile din Hanoi — mutări: J1 {}, J2 {} (minim {})\n",
            self.moves[0],
            self.moves[1],
            (1u32 << N) - 1
        );
        for p in 0..2 {
            s.push_str(&format!("Turnul lui J{}:\n", p + 1));
            for t in 0..3u8 {
                let mut discs: Vec<usize> = (0..N).filter(|&d| self.peg[p][d] == t).collect();
                discs.sort_unstable_by(|a, b| b.cmp(a));
                s.push_str(&format!(
                    "  {}: {}\n",
                    (b'a' + t) as char,
                    discs
                        .iter()
                        .map(|d| (d + 1).to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                ));
            }
        }
        if self.outcome().is_none() {
            s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        format!("{}-{}", (b'a' + m.0) as char, (b'a' + m.1) as char)
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::ai;
    use polytabletop_core::rng::Rng;

    #[test]
    fn dist_start_and_goal() {
        let g = Hanoi::new();
        assert_eq!(g.remaining(Player::One), 31);
        let mut h = g.clone();
        h.peg[0] = [2; N];
        assert_eq!(h.remaining(Player::One), 0);
    }

    #[test]
    fn optimal_play_is_31_and_draw() {
        let mut g = Hanoi::new();
        let mut n = 0;
        while g.outcome().is_none() && n < 200 {
            let m = ai::best_move(&g, n as u64).unwrap();
            let before = g.remaining(g.side);
            g.apply(&m);
            assert_eq!(g.remaining(g.side.other()), before - 1);
            n += 1;
        }
        assert_eq!(g.outcome(), Some(Outcome::Draw));
        assert_eq!(g.moves, [31, 31]);
    }

    #[test]
    fn random_games_end_or_cap() {
        let mut g = Hanoi::new();
        let mut rng = Rng::new(3);
        for _ in 0..4000 {
            if g.outcome().is_some() {
                break;
            }
            let mv = g.legal_moves();
            assert!(!mv.is_empty());
            g.apply(&mv[rng.below(mv.len())]);
        }
    }
}
