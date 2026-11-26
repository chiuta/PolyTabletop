//! Peg solitaire (Franța), jocul clasic de răbdare, în format „duel” pentru 2 jucători.
//!
//! * Tabla engleză cu 33 de găuri (cruce de 7×7), 32 de bile, centrul (d4) liber.
//! * O mutare = o bilă sare ortogonal peste o bilă vecină într-o gaură liberă, iar bila sărită se
//!   scoate. Nu există alte mutări.
//! * Duelul: fiecare jucător joacă **propria tablă** (identică, pornită la fel); jucătorii mută pe
//!   rând. Cine rămâne fără mutări iese din joc, celălalt continuă singur. La final câștigă cine are
//!   **mai puține bile** pe tablă; la egalitate, cine are ultima bilă în centru; altfel remiză.
//!
//! Mutări: `d2-d4` (de la… la… , pătratul de sosire; se sare peste cel dintre ele).

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: usize = 7;

fn valid(r: usize, c: usize) -> bool {
    (2..=4).contains(&r) || (2..=4).contains(&c)
}

#[derive(Clone, PartialEq, Debug)]
pub struct Mv {
    from: u8,
    to: u8,
}

#[derive(Clone)]
pub struct PegSolitaire {
    b: [[bool; 49]; 2],
    side: Player,
}

impl Default for PegSolitaire {
    fn default() -> Self {
        Self::new()
    }
}

fn name(i: u8) -> String {
    format!("{}{}", (b'a' + i % 7) as char, i / 7 + 1)
}

fn moves_of(b: &[bool; 49]) -> Vec<Mv> {
    let mut v = Vec::new();
    for r in 0..N {
        for c in 0..N {
            if !valid(r, c) || !b[r * N + c] {
                continue;
            }
            for (dr, dc) in [(0i32, 2i32), (0, -2), (2, 0), (-2, 0)] {
                let (r2, c2) = (r as i32 + dr, c as i32 + dc);
                let (r1, c1) = (r as i32 + dr / 2, c as i32 + dc / 2);
                if !(0..7).contains(&r2) || !(0..7).contains(&c2) {
                    continue;
                }
                let (r2, c2, r1, c1) = (r2 as usize, c2 as usize, r1 as usize, c1 as usize);
                if valid(r2, c2) && b[r1 * N + c1] && !b[r2 * N + c2] {
                    v.push(Mv {
                        from: (r * N + c) as u8,
                        to: (r2 * N + c2) as u8,
                    });
                }
            }
        }
    }
    v
}

fn pegs(b: &[bool; 49]) -> i32 {
    b.iter().filter(|&&x| x).count() as i32
}

impl PegSolitaire {
    pub fn new() -> Self {
        let mut b = [false; 49];
        for r in 0..N {
            for c in 0..N {
                if valid(r, c) {
                    b[r * N + c] = true;
                }
            }
        }
        b[3 * N + 3] = false;
        PegSolitaire {
            b: [b, b],
            side: Player::One,
        }
    }

    pub fn pegs_left(&self) -> (i32, i32) {
        (pegs(&self.b[0]), pegs(&self.b[1]))
    }

    fn done(&self, p: usize) -> bool {
        moves_of(&self.b[p]).is_empty()
    }

    /// Calitatea unei table (cât de „aproape de rezolvat”), pentru AI.
    fn quality(b: &[bool; 49]) -> i32 {
        let mut s = -pegs(b) * 100;
        for r in 0..N {
            for c in 0..N {
                if !valid(r, c) || !b[r * N + c] {
                    continue;
                }
                // bilă izolată: nicio altă bilă în cruce la distanță ≤ 2
                let mut nb = 0;
                for (dr, dc) in [
                    (0i32, 1i32),
                    (0, -1),
                    (1, 0),
                    (-1, 0),
                    (0, 2),
                    (0, -2),
                    (2, 0),
                    (-2, 0),
                ] {
                    let (rr, cc) = (r as i32 + dr, c as i32 + dc);
                    if (0..7).contains(&rr)
                        && (0..7).contains(&cc)
                        && b[rr as usize * N + cc as usize]
                    {
                        nb += 1;
                    }
                }
                if nb == 0 {
                    s -= 60;
                }
                // preferă bilele spre centru
                s -= ((r as i32 - 3).abs() + (c as i32 - 3).abs()) * 2;
            }
        }
        s + moves_of(b).len() as i32 * 3
    }
}

impl Game for PegSolitaire {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        moves_of(&self.b[self.side.idx()])
    }

    fn apply(&mut self, m: &Mv) {
        let me = self.side.idx();
        let (f, t) = (m.from as usize, m.to as usize);
        let mid = (f + t) / 2;
        let b = &mut self.b[me];
        b[f] = false;
        b[mid] = false;
        b[t] = true;
        let other = 1 - me;
        if !self.done(other) {
            self.side = self.side.other();
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.done(0) || !self.done(1) {
            return None;
        }
        let (a, b) = (pegs(&self.b[0]), pegs(&self.b[1]));
        Some(match a.cmp(&b) {
            std::cmp::Ordering::Less => Outcome::Win(Player::One),
            std::cmp::Ordering::Greater => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => {
                let c = (self.b[0][24], self.b[1][24]);
                match (a == 1 && c.0, b == 1 && c.1) {
                    (true, false) => Outcome::Win(Player::One),
                    (false, true) => Outcome::Win(Player::Two),
                    _ => Outcome::Draw,
                }
            }
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        Self::quality(&self.b[p.idx()]) - Self::quality(&self.b[1 - p.idx()])
    }

    fn render(&self) -> String {
        let mut s = String::from(
            "Peg solitaire (duel): J1 stânga, J2 dreapta; o = bilă, . = gaură goală\n",
        );
        s.push_str("   a b c d e f g        a b c d e f g\n");
        for r in (0..N).rev() {
            for k in 0..2 {
                s.push_str(&format!("{} ", r + 1));
                for c in 0..N {
                    s.push(if !valid(r, c) {
                        ' '
                    } else if self.b[k][r * N + c] {
                        'o'
                    } else {
                        '.'
                    });
                    s.push(' ');
                }
                s.push_str("     ");
            }
            s.push('\n');
        }
        let (a, b) = self.pegs_left();
        s.push_str(&format!("Bile rămase: J1 {a}, J2 {b}\n"));
        match self.outcome() {
            Some(Outcome::Win(p)) => s.push_str(&format!("Câștigă J{}.\n", p.idx() + 1)),
            Some(Outcome::Draw) => s.push_str("Remiză.\n"),
            None => s.push_str(&format!("La mutare: J{}\n", self.side.idx() + 1)),
        }
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        format!("{}-{}", name(m.from), name(m.to))
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
    fn start_has_four_moves_and_32_pegs() {
        let g = PegSolitaire::new();
        assert_eq!(g.pegs_left(), (32, 32));
        assert_eq!(g.legal_moves().len(), 4);
    }

    #[test]
    fn jump_removes_the_jumped_peg_on_own_board_only() {
        let mut g = PegSolitaire::new();
        let m = g.legal_moves()[0].clone();
        g.apply(&m);
        assert_eq!(g.pegs_left(), (31, 32));
        assert_eq!(g.side, Player::Two);
    }

    #[test]
    fn a_player_without_moves_drops_out() {
        let mut g = PegSolitaire::new();
        g.b[1] = [false; 49];
        g.b[1][24] = true; // J2 are o singură bilă: fără mutări
        let m = g.legal_moves()[0].clone();
        g.apply(&m);
        assert_eq!(g.side, Player::One);
    }

    #[test]
    fn random_games_end_and_ai_works() {
        for seed in 1..=8u64 {
            let mut g: Box<dyn polytabletop_core::GameObj> = Box::new(PegSolitaire::new());
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.play(&m[rng.below(m.len())]).unwrap();
                n += 1;
                assert!(n <= 64);
            }
        }
        let mut g: Box<dyn polytabletop_core::GameObj> = Box::new(PegSolitaire::new());
        let mut rng = Rng::new(2);
        while g.outcome().is_none() {
            let m = if g.to_move() == Player::One {
                g.ai_move(rng.next_u64()).unwrap()
            } else {
                let l = g.legal_moves();
                l[rng.below(l.len())].clone()
            };
            g.play(&m).unwrap();
        }
        assert_ne!(g.outcome(), Some(Outcome::Win(Player::Two)));
    }
}
