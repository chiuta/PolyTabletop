//! Pente (SUA), 2 jucători, din regulile publice.
//!
//! * Tablă 19×19 (intersecții). Se pun pietre pe rând; J1 (X) începe, cu prima piatră obligatoriu în
//!   centru (k10). Regula de turneu: a doua piatră a lui J1 trebuie să fie la cel puțin trei
//!   intersecții de centru.
//! * Captură: două pietre adverse consecutive (exact două) pe o linie, încadrate la capete de pietrele
//!   tale, sunt scoase. Intrarea voluntară într-o poziție încadrată nu e captură.
//! * Câștigă cine face cinci sau mai multe pietre în linie sau cine capturează cinci perechi
//!   (zece pietre). Tabla plină sau 600 de mutări = remiză.
//! * Mutările legale sunt limitate la intersecțiile aflate la cel mult două pași de o piatră existentă
//!   (la fel ca la Gomoku): nu pierde nimic practic și ține AI-ul rapid.
//!
//! Mutări: `k10` (coloane a–s fără i, rânduri 1–19).
#![allow(clippy::needless_range_loop)]

use polytabletop_core::{hash_mix, AiStyle, Game, Outcome, Player};

const N: usize = 19;
const COLS: &[u8; 19] = b"abcdefghjklmnopqrst";
const MAX_PLIES: u16 = 600;

#[derive(Clone)]
pub struct Pente {
    cells: [u8; N * N],
    side: Player,
    plies: u16,
    caps: [u8; 2],
    winner: Option<Outcome>,
    stones: u16,
}

impl Default for Pente {
    fn default() -> Self {
        Self::new()
    }
}

fn name(i: usize) -> String {
    format!("{}{}", COLS[i % N] as char, i / N + 1)
}

impl Pente {
    pub fn new() -> Self {
        Pente {
            cells: [0; N * N],
            side: Player::One,
            plies: 0,
            caps: [0, 0],
            winner: None,
            stones: 0,
        }
    }

    pub fn with_seed(_seed: u64) -> Self {
        Self::new()
    }

    pub fn captures(&self) -> [u8; 2] {
        self.caps
    }

    fn five_at(&self, idx: usize) -> bool {
        let me = self.cells[idx];
        let (row, col) = ((idx / N) as i32, (idx % N) as i32);
        for (dr, dc) in [(0i32, 1i32), (1, 0), (1, 1), (1, -1)] {
            let mut n = 1;
            for sign in [1i32, -1] {
                let (mut r, mut c) = (row + dr * sign, col + dc * sign);
                while r >= 0 && r < N as i32 && c >= 0 && c < N as i32 {
                    if self.cells[r as usize * N + c as usize] != me {
                        break;
                    }
                    n += 1;
                    r += dr * sign;
                    c += dc * sign;
                }
            }
            if n >= 5 {
                return true;
            }
        }
        false
    }

    fn at(&self, r: i32, c: i32) -> u8 {
        if r < 0 || r >= N as i32 || c < 0 || c >= N as i32 {
            9
        } else {
            self.cells[r as usize * N + c as usize]
        }
    }

    fn score_windows(&self, me: u8) -> i32 {
        let mut total = 0;
        let you = 3 - me;
        for r in 0..N as i32 {
            for c in 0..N as i32 {
                for (dr, dc) in [(0i32, 1i32), (1, 0), (1, 1), (1, -1)] {
                    let (er, ec) = (r + 4 * dr, c + 4 * dc);
                    if er < 0 || er >= N as i32 || ec < 0 || ec >= N as i32 {
                        continue;
                    }
                    let (mut a, mut b) = (0, 0);
                    for k in 0..5 {
                        match self.at(r + k * dr, c + k * dc) {
                            x if x == me => a += 1,
                            x if x == you => b += 1,
                            _ => {}
                        }
                    }
                    if b == 0 && a > 0 {
                        total += [0, 1, 8, 60, 500, 100000][a];
                    }
                }
            }
        }
        total
    }
}

impl Game for Pente {
    type Move = u16;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<u16> {
        if self.winner.is_some() {
            return Vec::new();
        }
        let centre = N / 2 * N + N / 2;
        if self.stones == 0 {
            return vec![centre as u16];
        }
        let mut near = [false; N * N];
        for i in 0..N * N {
            if self.cells[i] != 0 {
                let (r, c) = ((i / N) as i32, (i % N) as i32);
                for dr in -2..=2 {
                    for dc in -2..=2 {
                        let (rr, cc) = (r + dr, c + dc);
                        if rr >= 0 && rr < N as i32 && cc >= 0 && cc < N as i32 {
                            near[rr as usize * N + cc as usize] = true;
                        }
                    }
                }
            }
        }
        let tournament = self.plies == 2;
        (0..N * N)
            .filter(|&i| near[i] && self.cells[i] == 0)
            .filter(|&i| {
                if !tournament {
                    return true;
                }
                let (r, c) = ((i / N) as i32 - 9, (i % N) as i32 - 9);
                r.abs().max(c.abs()) >= 3
            })
            .map(|i| i as u16)
            .collect()
    }

    fn apply(&mut self, m: &u16) {
        let i = *m as usize;
        let me = self.side.idx() as u8 + 1;
        let you = 3 - me;
        self.cells[i] = me;
        self.stones += 1;
        self.plies += 1;
        let (r, c) = ((i / N) as i32, (i % N) as i32);
        for dr in -1i32..=1 {
            for dc in -1i32..=1 {
                if dr == 0 && dc == 0 {
                    continue;
                }
                if self.at(r + dr, c + dc) == you
                    && self.at(r + 2 * dr, c + 2 * dc) == you
                    && self.at(r + 3 * dr, c + 3 * dc) == me
                {
                    self.cells[(r + dr) as usize * N + (c + dc) as usize] = 0;
                    self.cells[(r + 2 * dr) as usize * N + (c + 2 * dc) as usize] = 0;
                    self.caps[self.side.idx()] += 1;
                    self.stones -= 2;
                }
            }
        }
        if self.caps[self.side.idx()] >= 5 || self.five_at(i) {
            self.winner = Some(Outcome::Win(self.side));
        } else if self.stones as usize >= N * N || self.plies >= MAX_PLIES {
            self.winner = Some(Outcome::Draw);
        }
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        self.winner
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx() as u8 + 1;
        let you = 3 - me;
        let cap = self.caps[p.idx()] as i32 - self.caps[p.other().idx()] as i32;
        (self.score_windows(me) - self.score_windows(you)).clamp(-50_000, 50_000) + cap * 400
    }

    fn render(&self) -> String {
        let mut s = String::from("Pente — X = J1, O = J2\n   ");
        for c in 0..N {
            s.push_str(&format!(" {}", COLS[c] as char));
        }
        s.push('\n');
        for r in (0..N).rev() {
            s.push_str(&format!("{:>2} ", r + 1));
            for c in 0..N {
                s.push_str(match self.cells[r * N + c] {
                    1 => " X",
                    2 => " O",
                    _ => " .",
                });
            }
            s.push('\n');
        }
        s.push_str(&format!(
            "Capturi (perechi): J1 {}, J2 {}\n",
            self.caps[0], self.caps[1]
        ));
        match self.winner {
            Some(Outcome::Win(p)) => s.push_str(&format!("Jucătorul {} câștigă.\n", p.idx() + 1)),
            Some(Outcome::Draw) => s.push_str("Remiză.\n"),
            None => s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1)),
        }
        s
    }

    fn format_move(&self, m: &u16) -> String {
        name(*m as usize)
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 600,
            max_depth: 4,
        }
    }

    fn hash_key(&self) -> Option<u64> {
        let mut h =
            self.side.idx() as u64 ^ ((self.caps[0] as u64) << 8) ^ ((self.caps[1] as u64) << 12);
        for i in 0..N * N {
            if self.cells[i] != 0 {
                h ^= hash_mix(((i as u64) << 3) | self.cells[i] as u64);
            }
        }
        Some(h ^ (self.plies.min(3) as u64) << 20)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    #[test]
    fn first_move_centre_and_second_restriction() {
        let mut g = Pente::new();
        assert_eq!(g.legal_moves(), vec![(9 * N + 9) as u16]);
        g.apply(&((9 * N + 9) as u16));
        let m = g.legal_moves();
        g.apply(&m[0]);
        let m2 = g.legal_moves();
        assert!(!m2.is_empty());
        for x in m2 {
            let (r, c) = ((x as usize / N) as i32 - 9, (x as usize % N) as i32 - 9);
            assert!(r.abs().max(c.abs()) >= 3);
        }
    }

    #[test]
    fn pair_capture() {
        let mut g = Pente::new();
        g.stones = 3;
        g.plies = 5;
        g.cells[9 * N + 5] = 1; // X
        g.cells[9 * N + 6] = 2;
        g.cells[9 * N + 7] = 2;
        g.side = Player::One;
        g.apply(&((9 * N + 8) as u16));
        assert_eq!(g.cells[9 * N + 6], 0);
        assert_eq!(g.cells[9 * N + 7], 0);
        assert_eq!(g.captures()[0], 1);
    }

    #[test]
    fn moving_into_a_flank_is_safe() {
        let mut g = Pente::new();
        g.plies = 5;
        g.stones = 2;
        g.cells[9 * N + 5] = 1;
        g.cells[9 * N + 8] = 1;
        g.cells[9 * N + 6] = 2;
        g.side = Player::Two;
        g.apply(&((9 * N + 7) as u16));
        assert_eq!(g.cells[9 * N + 6], 2);
        assert_eq!(g.cells[9 * N + 7], 2);
    }

    #[test]
    fn five_in_row_wins() {
        let mut g = Pente::new();
        g.plies = 8;
        g.stones = 4;
        for c in 3..7 {
            g.cells[9 * N + c] = 1;
        }
        g.side = Player::One;
        g.apply(&((9 * N + 7) as u16));
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn random_game_ends() {
        for seed in 1..=3u64 {
            let mut g = Pente::new();
            let mut rng = Rng::new(seed);
            while g.outcome().is_none() {
                let m = g.legal_moves();
                g.apply(&m[rng.below(m.len())]);
            }
        }
    }
}
