//! „Cele șase ținte” — jocul de zaruri tradițional american cunoscut ca Bunco, pentru 2 jucători.
//!
//! * Șase runde; în runda `r` ținta este fața `r` (runda 1 = ași/unu, …, runda 6 = șase).
//! * Pe rând aruncați trei zaruri (mutarea unică `arunc`). Fiecare zar egal cu ținta valorează
//!   1 punct; trei zaruri egale cu ținta („Bunco”) valorează 21 de puncte și încheie runda; trei
//!   zaruri egale, dar nu cu ținta, valorează 5 puncte. Dacă aruncarea a punctat, aruncați din nou;
//!   la prima aruncare fără puncte tura trece la adversar.
//! * Runda se încheie când un jucător ajunge la 21 de puncte în runda respectivă (câștigă runda).
//! * După șase runde câștigă cine a luat mai multe runde; la egalitate decide totalul de puncte
//!   din toate rundele; apoi remiză.
//!
//! Nu există decizii: ca în jocul tradițional, totul e șansă. Zarurile se aruncă la mutare, cu
//! un `Rng` propriu (determinist pentru o sămânță).

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const ROUNDS: u8 = 6;
const TARGET: u32 = 21;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Roll,
}

#[derive(Clone)]
pub struct Bunco {
    rng: Rng,
    round: u8,
    round_pts: [u32; 2],
    total: [u32; 2],
    wins: [u8; 2],
    side: Player,
    first: Player,
    last: Option<[u8; 3]>,
    log: String,
}

impl Default for Bunco {
    fn default() -> Self {
        Self::new()
    }
}

/// Punctele unei aruncări (zaruri, țintă) și dacă e „Bunco”.
pub fn score_roll(d: [u8; 3], target: u8) -> (u32, bool) {
    if d[0] == d[1] && d[1] == d[2] {
        if d[0] == target {
            return (TARGET, true);
        }
        return (5, false);
    }
    (d.iter().filter(|&&x| x == target).count() as u32, false)
}

impl Bunco {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Bunco {
            rng: Rng::new(seed),
            round: 1,
            round_pts: [0, 0],
            total: [0, 0],
            wins: [0, 0],
            side: Player::One,
            first: Player::One,
            last: None,
            log: String::new(),
        }
    }

    pub fn round(&self) -> u8 {
        self.round
    }

    pub fn wins(&self) -> (u8, u8) {
        (self.wins[0], self.wins[1])
    }

    fn over(&self) -> bool {
        self.round > ROUNDS
    }
}

impl Game for Bunco {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over() {
            Vec::new()
        } else {
            vec![Move::Roll]
        }
    }

    fn apply(&mut self, _m: &Move) {
        let d = [self.rng.d6(), self.rng.d6(), self.rng.d6()];
        self.last = Some(d);
        let p = self.side;
        let (pts, bunco) = score_roll(d, self.round);
        self.round_pts[p.idx()] += pts;
        self.total[p.idx()] += pts;
        self.log = format!(
            "J{} aruncă {} {} {}: {}{}",
            p.idx() + 1,
            d[0],
            d[1],
            d[2],
            pts,
            if bunco { " (BUNCO!)" } else { "" }
        );
        if self.round_pts[p.idx()] >= TARGET {
            self.wins[p.idx()] += 1;
            self.log
                .push_str(&format!(" — J{} câștigă runda {}", p.idx() + 1, self.round));
            self.round += 1;
            self.round_pts = [0, 0];
            self.first = self.first.other();
            self.side = self.first;
        } else if pts == 0 {
            self.side = p.other();
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over() {
            return None;
        }
        let (a, b) = (self.wins[0], self.wins[1]);
        let (ta, tb) = (self.total[0], self.total[1]);
        Some(if (a, ta) > (b, tb) {
            Outcome::Win(Player::One)
        } else if (a, ta) < (b, tb) {
            Outcome::Win(Player::Two)
        } else {
            Outcome::Draw
        })
    }

    fn render(&self) -> String {
        let mut s = String::new();
        if self.over() {
            s.push_str("Cele șase ținte (Bunco) — joc încheiat\n");
        } else {
            s.push_str(&format!(
                "Cele șase ținte (Bunco) — runda {}/{} (ținta: {}) | puncte în rundă: J1 {}, J2 {} (până la {})\n",
                self.round, ROUNDS, self.round, self.round_pts[0], self.round_pts[1], TARGET
            ));
        }
        s.push_str(&format!(
            "Runde câștigate: J1 {}, J2 {} | puncte totale: J1 {}, J2 {}\n",
            self.wins[0], self.wins[1], self.total[0], self.total[1]
        ));
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        if !self.over() {
            s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        }
        s
    }

    fn format_move(&self, _m: &Move) -> String {
        "arunc".into()
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scoring() {
        assert_eq!(score_roll([3, 3, 3], 3), (21, true));
        assert_eq!(score_roll([2, 2, 2], 3), (5, false));
        assert_eq!(score_roll([3, 3, 1], 3), (2, false));
        assert_eq!(score_roll([1, 2, 4], 3), (0, false));
        assert_eq!(score_roll([6, 3, 2], 3), (1, false));
    }

    #[test]
    fn no_score_passes_turn() {
        let mut g = Bunco::with_seed(1);
        for _ in 0..200 {
            if g.over() {
                break;
            }
            let before = g.side;
            let r = g.round;
            g.apply(&Move::Roll);
            if g.round == r {
                let d = g.last.unwrap();
                let (pts, _) = score_roll(d, r);
                if pts == 0 {
                    assert_eq!(g.side, before.other());
                } else {
                    assert_eq!(g.side, before);
                }
            }
        }
    }

    #[test]
    fn six_rounds_then_over() {
        for seed in 1..=8u64 {
            let mut g = Bunco::with_seed(seed);
            let mut n = 0;
            while g.outcome().is_none() && n < 100_000 {
                g.apply(&Move::Roll);
                n += 1;
            }
            assert!(g.outcome().is_some());
            assert_eq!(g.round, ROUNDS + 1);
            assert_eq!(g.wins[0] + g.wins[1], ROUNDS);
        }
    }

    #[test]
    fn deterministic() {
        let mut a = Bunco::with_seed(9);
        let mut b = Bunco::with_seed(9);
        for _ in 0..50 {
            if a.over() {
                break;
            }
            a.apply(&Move::Roll);
            b.apply(&Move::Roll);
        }
        assert_eq!(a.render(), b.render());
    }
}
