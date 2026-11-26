//! Prinde coada măgarului (Pin the Tail on the Donkey, SUA), 2 jucători — joc de petrecere simulat.
//!
//! * Pe un câmp 7×5 (coloane a–g, rânduri 1–5) coada trebuie prinsă la locul ei, marcat cu `T` și
//!   schimbat în fiecare rundă. Cel cu ochii legați își alege direcția (celula spre care merge), dar
//!   se învârte: pe fiecare axă greșește cu 0 (50%), ±1 (20% fiecare) sau ±2 (5% fiecare).
//! * Puncte: aterizare exactă 5; la 1 pătrat (pe orice axă, diagonala contează) 3; la 2 pătrate 2; la 3 pătrate 1;
//!   mai departe 0. O aterizare în afara câmpului se oprește la marginea lui.
//! * 5 runde pentru fiecare; câștigă cine are mai multe puncte (egal = remiză).
//! * Mutarea se face în doi pași: alegi celula-țintă (`c3`), apoi forțata `invart`, ca AI-ul să nu vadă
//!   rezultatul aleator înainte să aleagă.
//!
//! Mutări: `c3`, apoi `invart`.
#![allow(clippy::manual_is_multiple_of)]

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const W: i32 = 7;
const H: i32 = 5;
const ROUNDS: u8 = 5;

#[derive(Clone, PartialEq, Debug)]
pub enum Move {
    Aim(u8, u8),
    Spin,
}

#[derive(Clone)]
pub struct PinTheTail {
    rng: Rng,
    tail: (i32, i32),
    score: [i32; 2],
    side: Player,
    turn: u8,
    pending: Option<(u8, u8)>,
    log: String,
}

impl Default for PinTheTail {
    fn default() -> Self {
        Self::new()
    }
}

fn noise(rng: &mut Rng) -> i32 {
    match rng.below(100) {
        0..=49 => 0,
        50..=69 => 1,
        70..=89 => -1,
        90..=94 => 2,
        _ => -2,
    }
}

impl PinTheTail {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let tail = (rng.below(W as usize) as i32, rng.below(H as usize) as i32);
        PinTheTail {
            rng,
            tail,
            score: [0, 0],
            side: Player::One,
            turn: 0,
            pending: None,
            log: String::new(),
        }
    }

    fn over(&self) -> bool {
        self.turn >= ROUNDS * 2
    }
}

impl Game for PinTheTail {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over() {
            return Vec::new();
        }
        if self.pending.is_some() {
            return vec![Move::Spin];
        }
        let mut v = Vec::new();
        for y in 0..H {
            for x in 0..W {
                v.push(Move::Aim(x as u8, y as u8));
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Aim(x, y) => self.pending = Some((*x, *y)),
            Move::Spin => {
                let (x, y) = self.pending.take().unwrap();
                let nx = (x as i32 + noise(&mut self.rng)).clamp(0, W - 1);
                let ny = (y as i32 + noise(&mut self.rng)).clamp(0, H - 1);
                let d = (nx - self.tail.0).abs().max((ny - self.tail.1).abs());
                let pts = match d {
                    0 => 5,
                    1 => 3,
                    2 => 2,
                    3 => 1,
                    _ => 0,
                };
                self.score[self.side.idx()] += pts;
                self.log = format!(
                    "J{}: ținta {}{}, a ajuns pe {}{} → +{pts}.",
                    self.side.idx() + 1,
                    (b'a' + x) as char,
                    y + 1,
                    (b'a' + nx as u8) as char,
                    ny + 1
                );
                self.turn += 1;
                self.side = self.side.other();
                // coadă nouă la fiecare rundă (după ce au mers ambii)
                if self.turn % 2 == 0 {
                    self.tail = (
                        self.rng.below(W as usize) as i32,
                        self.rng.below(H as usize) as i32,
                    );
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over() {
            return None;
        }
        Some(match self.score[0].cmp(&self.score[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let mut v = (self.score[p.idx()] - self.score[p.other().idx()]) * 10;
        if let Some((x, y)) = self.pending {
            if self.side == p {
                let d = (x as i32 - self.tail.0)
                    .abs()
                    .max((y as i32 - self.tail.1).abs());
                v -= d * 3;
            }
        }
        v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Prinde coada măgarului — runda {}/{ROUNDS}, scor J1 {} – J2 {}\n   ",
            (self.turn / 2 + 1).min(ROUNDS),
            self.score[0],
            self.score[1]
        );
        for x in 0..W {
            s.push_str(&format!(" {}", (b'a' + x as u8) as char));
        }
        s.push('\n');
        for y in 0..H {
            s.push_str(&format!("{:>2} ", y + 1));
            for x in 0..W {
                s.push_str(if (x, y) == self.tail { " T" } else { " ." });
            }
            s.push('\n');
        }
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        match self.outcome() {
            Some(Outcome::Win(p)) => s.push_str(&format!("Jucătorul {} câștigă.\n", p.idx() + 1)),
            Some(Outcome::Draw) => s.push_str("Remiză.\n"),
            None => {
                if self.pending.is_some() {
                    s.push_str("Ochii legați, te învârți: scrie «invart».\n");
                }
                s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
            }
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Aim(x, y) => format!("{}{}", (b'a' + x) as char, y + 1),
            Move::Spin => "invart".into(),
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
    fn ten_throws() {
        let mut g = PinTheTail::with_seed(3);
        let mut rng = Rng::new(1);
        let mut n = 0;
        while g.outcome().is_none() {
            let m = g.legal_moves();
            g.apply(&m[rng.below(m.len())]);
            n += 1;
        }
        assert_eq!(n, 20);
    }

    #[test]
    fn aiming_at_the_tail_scores_more_on_average() {
        let mut best = 0;
        let mut worst = 0;
        for seed in 0..200u64 {
            let mut g = PinTheTail::with_seed(seed);
            let t = g.tail;
            let mut h = g.clone();
            g.apply(&Move::Aim(t.0 as u8, t.1 as u8));
            g.apply(&Move::Spin);
            best += g.score[0];
            h.apply(&Move::Aim(((t.0 + 3) % W) as u8, ((t.1 + 3) % H) as u8));
            h.apply(&Move::Spin);
            worst += h.score[0];
        }
        assert!(best > worst);
    }
}
