#![allow(clippy::doc_lazy_continuation)]
//! Petteia (Grecia antică, „pessoi”), reconstituire pentru doi jucători pe tablă 8×8.
//!
//! Regulile exacte ale jocului antic nu s-au păstrat; folosim varianta de reconstituire cea mai
//! răspândită: câte 16 piese pe primele două rânduri ale fiecărui jucător; orice piesă se mută ca un
//! turn (orice număr de pătrate libere, orizontal sau vertical, fără sărituri); o piesă este
//! capturată când două piese adverse o încadrează pe aceeași linie sau coloană (capturare „custodială”):
//! mutarea ta pune piesa pe una din laturi, iar pe cealaltă latură e deja o piesă a ta. Poți intra
//! singur între două piese adverse fără să fii capturat. O mutare poate captura mai multe piese.
//! * Pierde cine rămâne cu o singură piesă sau fără mutări. Dacă 60 de mutări la rând nu captu-
//!   rează nimic, câștigă cine are mai multe piese (egal = remiză) — regulă proprie a implementării.
//! Mutări: `a2a4` (de la, la).

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const S: usize = 8;
const N: usize = S * S;
const QUIET: u16 = 60;
const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn name(i: usize) -> String {
    polytabletop_core::square_name(i % S, i / S)
}

fn step(i: usize, d: usize, k: i32) -> Option<usize> {
    let (r, c) = (
        (i / S) as i32 + DIRS[d].0 * k,
        (i % S) as i32 + DIRS[d].1 * k,
    );
    if r < 0 || c < 0 || r >= S as i32 || c >= S as i32 {
        None
    } else {
        Some(r as usize * S + c as usize)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Mv {
    pub from: u8,
    pub to: u8,
}

#[derive(Clone)]
pub struct Petteia {
    b: [u8; N],
    side: Player,
    quiet: u16,
}

impl Default for Petteia {
    fn default() -> Self {
        Self::new()
    }
}

impl Petteia {
    pub fn new() -> Self {
        let mut b = [0u8; N];
        for c in 0..S {
            b[c] = 1;
            b[S + c] = 1;
            b[6 * S + c] = 2;
            b[7 * S + c] = 2;
        }
        Petteia {
            b,
            side: Player::One,
            quiet: 0,
        }
    }

    fn count(&self, c: u8) -> usize {
        self.b.iter().filter(|&&x| x == c).count()
    }

    fn captured_by(b: &[u8; N], to: usize, me: u8) -> Vec<usize> {
        let mut v = Vec::new();
        for d in 0..4 {
            if let (Some(n1), Some(n2)) = (step(to, d, 1), step(to, d, 2)) {
                if b[n1] != 0 && b[n1] != me && b[n2] == me {
                    v.push(n1);
                }
            }
        }
        v
    }

    fn gen(&self) -> Vec<Mv> {
        let me = self.side.idx() as u8 + 1;
        let mut v = Vec::new();
        for from in 0..N {
            if self.b[from] != me {
                continue;
            }
            for d in 0..4 {
                let mut k = 1;
                while let Some(to) = step(from, d, k) {
                    if self.b[to] != 0 {
                        break;
                    }
                    v.push(Mv {
                        from: from as u8,
                        to: to as u8,
                    });
                    k += 1;
                }
            }
        }
        v
    }

    fn terminal(&self) -> Option<Outcome> {
        let (a, b) = (self.count(1), self.count(2));
        if a <= 1 {
            return Some(Outcome::Win(Player::Two));
        }
        if b <= 1 {
            return Some(Outcome::Win(Player::One));
        }
        if self.quiet >= QUIET {
            return Some(match a.cmp(&b) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                std::cmp::Ordering::Equal => Outcome::Draw,
            });
        }
        if self.gen().is_empty() {
            return Some(Outcome::Win(self.side.other()));
        }
        None
    }
}

impl Game for Petteia {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if self.terminal().is_some() {
            return Vec::new();
        }
        self.gen()
    }

    fn apply(&mut self, m: &Mv) {
        let me = self.side.idx() as u8 + 1;
        self.b[m.from as usize] = 0;
        self.b[m.to as usize] = me;
        let caps = Self::captured_by(&self.b, m.to as usize, me);
        if caps.is_empty() {
            self.quiet += 1;
        } else {
            self.quiet = 0;
            for c in caps {
                self.b[c] = 0;
            }
        }
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        self.terminal()
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx() as u8 + 1;
        let mut v = 0;
        for i in 0..N {
            let c = self.b[i];
            if c == 0 {
                continue;
            }
            let adv = if c == 1 { i / S } else { 7 - i / S } as i32;
            let x = 100 + adv * 2;
            v += if c == me { x } else { -x };
        }
        v
    }

    fn render(&self) -> String {
        let mut s = String::from("   a b c d e f g h\n");
        for r in (0..S).rev() {
            s.push_str(&format!("{}  ", r + 1));
            for c in 0..S {
                s.push(match self.b[r * S + c] {
                    1 => 'X',
                    2 => 'O',
                    _ => '.',
                });
                s.push(' ');
            }
            s.push('\n');
        }
        s.push_str(&format!(
            "Piese: X {}  O {}. La mutare: {}.\n",
            self.count(1),
            self.count(2),
            if self.side == Player::One { "X" } else { "O" }
        ));
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        format!("{}{}", name(m.from as usize), name(m.to as usize))
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    #[test]
    fn custodial_capture_and_safe_entry() {
        let mut g = Petteia::new();
        g.b = [0; N];
        g.b[3 * S + 3] = 2; // d4 negru
        g.b[3 * S + 4] = 1; // e4 alb
        g.b[2] = 1; // c1 alb
        g.b[7 * S + 7] = 2;
        g.b[7 * S + 6] = 2;
        g.side = Player::One;
        let mv = g
            .legal_moves()
            .into_iter()
            .find(|m| g.format_move(m) == "c1c4")
            .unwrap();
        g.apply(&mv);
        assert_eq!(g.b[3 * S + 3], 0, "d4 încadrat între c4 și e4");
        // intrarea voluntară între două piese adverse nu captează
        let mut h = Petteia::new();
        h.b = [0; N];
        h.b[3 * S + 2] = 2;
        h.b[3 * S + 4] = 2;
        h.b[3] = 1; // d1
        h.b[7 * S] = 2;
        h.b[S] = 1;
        h.side = Player::One;
        let mv = h
            .legal_moves()
            .into_iter()
            .find(|m| h.format_move(m) == "d1d4")
            .unwrap();
        h.apply(&mv);
        assert_eq!(h.b[3 * S + 3], 1);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=10u64 {
            let mut g = Petteia::new();
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
                assert!(n < 4000);
            }
        }
    }
}
