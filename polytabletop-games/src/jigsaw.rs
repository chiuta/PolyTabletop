//! Puzzle cu piese (Jigsaw, Marea Britanie), 2 jucători în întrecere — adaptare pe ture a puzzle-ului.
//!
//! * Puzzle 4×4 = 16 piese. Fiecare piesă are patru margini: `0` = margine plată (marginea cadrului), iar o
//!   margine interioară are un cod unic (+k pe o piesă, −k pe vecina cu care se îmbină), amestecat la start.
//! * Pe rând, fiecare jucător așază **o piesă** într-un loc liber, dacă: (1) toate marginile ei se potrivesc cu
//!   vecinele deja așezate (suma codurilor = 0), (2) marginile plate ale piesei cad exact pe marginea
//!   cadrului, (3) locul e în **colț** sau lipit de o piesă deja așezată (puzzle-ul crește din cadru).
//! * Puncte: 1 + numărul de vecini deja așezați cu care se îmbină piesa; cine așază ultima piesă primește +3.
//!   La final câștigă cine are mai multe puncte (egal = remiză).
//!
//! Mutări: `p7r2c3` (piesa 7 la rândul 2, coloana 3, numărând de la 1).

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: usize = 4;

#[derive(Clone, PartialEq, Debug)]
pub struct Move {
    pub piece: u8,
    pub slot: u8,
}

#[derive(Clone)]
pub struct Jigsaw {
    /// [sus, dreapta, jos, stânga] pentru piesa k (k = poziția ei corectă)
    edges: [[i8; 4]; N * N],
    placed: [Option<u8>; N * N],
    used: [bool; N * N],
    score: [i32; 2],
    side: Player,
    left: usize,
}

impl Default for Jigsaw {
    fn default() -> Self {
        Self::new()
    }
}

impl Jigsaw {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let total = 2 * N * (N - 1);
        let mut codes: Vec<i8> = (1..=total as i8).collect();
        crate::q6_util::shuffle(&mut codes, &mut rng);
        let mut edges = [[0i8; 4]; N * N];
        let mut k = 0;
        for r in 0..N {
            for c in 0..N {
                let i = r * N + c;
                if c + 1 < N {
                    let e = codes[k];
                    k += 1;
                    edges[i][1] = e;
                    edges[i + 1][3] = -e;
                }
                if r + 1 < N {
                    let e = codes[k];
                    k += 1;
                    edges[i][2] = e;
                    edges[i + N][0] = -e;
                }
            }
        }
        Jigsaw {
            edges,
            placed: [None; N * N],
            used: [false; N * N],
            score: [0, 0],
            side: Player::One,
            left: N * N,
        }
    }

    fn neighbours(slot: usize) -> [(usize, Option<usize>); 4] {
        let (r, c) = (slot / N, slot % N);
        [
            (0, if r > 0 { Some(slot - N) } else { None }),
            (1, if c + 1 < N { Some(slot + 1) } else { None }),
            (2, if r + 1 < N { Some(slot + N) } else { None }),
            (3, if c > 0 { Some(slot - 1) } else { None }),
        ]
    }

    /// Numărul de vecini potriviți dacă piesa `p` poate fi pusă în `slot`.
    fn fit(&self, p: usize, slot: usize) -> Option<i32> {
        let mut touching = 0;
        for (side, nb) in Self::neighbours(slot) {
            let e = self.edges[p][side];
            match nb {
                None => {
                    if e != 0 {
                        return None;
                    }
                }
                Some(n) => {
                    if e == 0 {
                        return None;
                    }
                    if let Some(q) = self.placed[n] {
                        let f = self.edges[q as usize][(side + 2) % 4];
                        if e + f != 0 {
                            return None;
                        }
                        touching += 1;
                    }
                }
            }
        }
        let corner = {
            let (r, c) = (slot / N, slot % N);
            (r == 0 || r == N - 1) && (c == 0 || c == N - 1)
        };
        if touching == 0 && !corner {
            return None;
        }
        Some(touching)
    }
}

impl Game for Jigsaw {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        let mut v = Vec::new();
        for p in 0..N * N {
            if self.used[p] {
                continue;
            }
            for s in 0..N * N {
                if self.placed[s].is_none() && self.fit(p, s).is_some() {
                    v.push(Move {
                        piece: p as u8,
                        slot: s as u8,
                    });
                }
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        let t = self.fit(m.piece as usize, m.slot as usize).unwrap_or(0);
        self.placed[m.slot as usize] = Some(m.piece);
        self.used[m.piece as usize] = true;
        self.left -= 1;
        self.score[self.side.idx()] += 1 + t + if self.left == 0 { 3 } else { 0 };
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.left > 0 {
            return None;
        }
        Some(match self.score[0].cmp(&self.score[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        (self.score[p.idx()] - self.score[p.other().idx()]) * 10
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Puzzle 4×4 — scor J1 {} – J2 {}\nCadrul (piesa la locul ei):\n",
            self.score[0], self.score[1]
        );
        for r in 0..N {
            for c in 0..N {
                match self.placed[r * N + c] {
                    Some(p) => s.push_str(&format!(" [{:>2}]", p)),
                    None => s.push_str(" [..]"),
                }
            }
            s.push('\n');
        }
        s.push_str("Piese rămase (sus, dreapta, jos, stânga):\n");
        for p in 0..N * N {
            if !self.used[p] {
                let e = self.edges[p];
                s.push_str(&format!(
                    "  {:>2}: {:>3} {:>3} {:>3} {:>3}\n",
                    p, e[0], e[1], e[2], e[3]
                ));
            }
        }
        match self.outcome() {
            Some(Outcome::Win(p)) => s.push_str(&format!("Jucătorul {} câștigă.\n", p.idx() + 1)),
            Some(Outcome::Draw) => s.push_str("Remiză.\n"),
            None => s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1)),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        format!(
            "p{}r{}c{}",
            m.piece,
            m.slot as usize / N + 1,
            m.slot as usize % N + 1
        )
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(6)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_moves_are_corners_only() {
        let g = Jigsaw::with_seed(4);
        let m = g.legal_moves();
        assert_eq!(m.len(), 4);
        for x in m {
            assert_eq!(x.piece, x.slot);
        }
    }

    #[test]
    fn random_games_finish_with_all_pieces() {
        for seed in 1..=10u64 {
            let mut g = Jigsaw::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                n += 1;
            }
            assert_eq!(n, 16);
            // fiecare piesă e la locul ei
            for s in 0..16 {
                assert_eq!(g.placed[s], Some(s as u8));
            }
        }
    }
}
