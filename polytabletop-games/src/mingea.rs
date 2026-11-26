#![allow(clippy::needless_range_loop, clippy::manual_is_multiple_of)]
//! Mingea (Dodgeball) — joc cu mingea între două echipe de câte 4 jucători, pe ture.
//!
//! Terenul fiecărei echipe are 7 coloane (1–7) și pe fiecare stau cel mult un jucător. La mutare
//! alegi: fie **muți** un jucător propriu într-o coloană liberă vecină (`3>4`), fie **arunci** spre o
//! coloană adversă (`arunc 4`). Mingea nu ajunge exact unde ținteai: mutarea forțată `zbor` stabilește
//! la întâmplare coloana de aterizare (50% ținta, 20% fiecare vecină, 5% la două coloane distanță;
//! în afara terenului = ratat). Jucătorul din coloana de aterizare este scos din joc. Câștigă echipa care
//! rămâne cu jucători după ce cealaltă a pierdut pe toți (după 300 de mutări câștigă echipa cu mai mulți
//! jucători, altfel remiză). AI-ul nu vede în avans unde cade mingea.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const COLS: usize = 7;
const CAP: u16 = 300;
/// procente pentru decalajul −2..=2
const SPREAD: [u32; 5] = [5, 20, 50, 20, 5];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Shift(u8, u8),
    Throw(u8),
    Fly,
}

#[derive(Clone)]
pub struct Mingea {
    rng: Rng,
    /// biți pe coloane pentru fiecare echipă
    mask: [u8; 2],
    side: Player,
    aim: Option<u8>,
    plies: u16,
    log: String,
}

impl Default for Mingea {
    fn default() -> Self {
        Self::new()
    }
}

fn hit_expect(mask: u8, aim: u8) -> f64 {
    let mut e = 0.0;
    for (k, &pct) in SPREAD.iter().enumerate() {
        let c = aim as i32 + k as i32 - 2;
        if (0..COLS as i32).contains(&c) && mask & (1 << c) != 0 {
            e += pct as f64 / 100.0;
        }
    }
    e
}

fn best_aim(mask: u8) -> f64 {
    (0..COLS as u8)
        .map(|c| hit_expect(mask, c))
        .fold(0.0, f64::max)
}

impl Mingea {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let m: u8 = (1 << 1) | (1 << 2) | (1 << 4) | (1 << 5);
        Mingea {
            rng: Rng::new(seed),
            mask: [m, m],
            side: Player::One,
            aim: None,
            plies: 0,
            log: String::new(),
        }
    }

    pub fn players(&self) -> (u32, u32) {
        (self.mask[0].count_ones(), self.mask[1].count_ones())
    }
}

impl Game for Mingea {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        if self.aim.is_some() {
            return vec![Move::Fly];
        }
        let mask = self.mask[self.side.idx()];
        let mut v = Vec::new();
        for c in 0..COLS as u8 {
            if mask & (1 << c) != 0 {
                if c > 0 && mask & (1 << (c - 1)) == 0 {
                    v.push(Move::Shift(c + 1, c));
                }
                if (c as usize) + 1 < COLS && mask & (1 << (c + 1)) == 0 {
                    v.push(Move::Shift(c + 1, c + 2));
                }
            }
        }
        for c in 0..COLS as u8 {
            v.push(Move::Throw(c + 1));
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        self.plies += 1;
        let me = self.side.idx();
        match *m {
            Move::Shift(a, b) => {
                self.mask[me] &= !(1 << (a - 1));
                self.mask[me] |= 1 << (b - 1);
                self.side = self.side.other();
            }
            Move::Throw(c) => self.aim = Some(c - 1),
            Move::Fly => {
                let aim = self.aim.take().unwrap() as i32;
                let r = self.rng.below(100) as u32;
                let mut acc = 0;
                let mut off = 0;
                for (k, &pct) in SPREAD.iter().enumerate() {
                    acc += pct;
                    if r < acc {
                        off = k as i32 - 2;
                        break;
                    }
                }
                let land = aim + off;
                let o = 1 - me;
                if (0..COLS as i32).contains(&land) && self.mask[o] & (1 << land) != 0 {
                    self.mask[o] &= !(1 << land);
                    self.log =
                        format!("J{} lovește coloana {}: un adversar iese", me + 1, land + 1);
                } else {
                    self.log = format!("J{} ratează (mingea cade în coloana {})", me + 1, land + 1);
                }
                self.side = self.side.other();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.mask[0] == 0 {
            return Some(Outcome::Win(Player::Two));
        }
        if self.mask[1] == 0 {
            return Some(Outcome::Win(Player::One));
        }
        if self.plies >= CAP {
            return Some(
                match self.mask[0].count_ones().cmp(&self.mask[1].count_ones()) {
                    std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                    std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                    std::cmp::Ordering::Equal => Outcome::Draw,
                },
            );
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let o = 1 - me;
        let mut v = (self.mask[me].count_ones() as i32 - self.mask[o].count_ones() as i32) * 1000;
        if let Some(a) = self.aim {
            let e = hit_expect(self.mask[1 - self.side.idx()], a);
            let sign = if self.side == p { 1.0 } else { -1.0 };
            v += (sign * e * 1000.0) as i32;
        } else {
            v += (best_aim(self.mask[o]) * 400.0) as i32 - (best_aim(self.mask[me]) * 400.0) as i32;
            // cine are rândul de aruncare are avantaj
            v += if self.side == p { 100 } else { 0 };
        }
        v
    }

    fn render(&self) -> String {
        let row = |m: u8| -> String {
            (0..COLS)
                .map(|c| if m & (1 << c) != 0 { "O" } else { "." })
                .collect::<Vec<_>>()
                .join(" ")
        };
        let mut s = format!(
            "Mingea (Dodgeball) — jucători: J1 {}, J2 {}\n",
            self.mask[0].count_ones(),
            self.mask[1].count_ones()
        );
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        s.push_str("coloane: 1 2 3 4 5 6 7\n");
        s.push_str(&format!("J1:      {}\n", row(self.mask[0])));
        s.push_str(&format!("J2:      {}\n", row(self.mask[1])));
        if self.outcome().is_none() {
            if self.aim.is_some() {
                s.push_str("Mingea zboară... scrie «zbor».\n");
            }
            s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Shift(a, b) => format!("{a}>{b}"),
            Move::Throw(c) => format!("arunc {c}"),
            Move::Fly => "zbor".into(),
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
    fn expectations() {
        let m = (1u8 << 3) | (1 << 4);
        assert!((hit_expect(m, 3) - 0.7).abs() < 1e-9); // 50 + 20
        assert!(best_aim(m) >= 0.7);
    }

    #[test]
    fn shift_and_throw_flow() {
        let mut g = Mingea::with_seed(1);
        assert!(g.legal_moves().contains(&Move::Shift(2, 1)));
        assert!(!g.legal_moves().contains(&Move::Shift(2, 3)) || g.mask[0] & (1 << 2) == 0);
        g.apply(&Move::Throw(4));
        assert_eq!(g.legal_moves(), vec![Move::Fly]);
        g.apply(&Move::Fly);
        assert_eq!(g.to_move(), Player::Two);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=6u64 {
            let mut g = Mingea::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut k = 0;
            while g.outcome().is_none() && k < 4000 {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
                k += 1;
            }
            assert!(g.outcome().is_some());
        }
    }
}
