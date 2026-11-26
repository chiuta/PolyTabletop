//! Seega (Egipt): tablă 5 × 5, 12 piese pe jucător, pătratul central rămâne gol la plasare.
//!
//! Reguli folosite (descrierile publice ale jocului, varianta cea mai răspândită):
//! * **Plasare**: jucătorii pun alternativ câte două piese (primul doi, apoi al doilea
//!   doi etc.) pe pătrate libere, fără centrul (c3), până se așază toate cele 24 de piese.
//! * **Mutare**: începe cine a plasat ultima piesă. O piesă se mută un pas ortogonal pe un
//!   pătrat liber. Prima mutare intră obligatoriu în centru.
//! * **Captură** (custodială): piesa mutată prinde între ea și o altă piesă proprie, pe
//!   aceeași linie, un șir continuu de piese adverse; toate sunt luate. Cine a capturat
//!   mută din nou (cu orice piesă).
//! * **Blocaj**: dacă jucătorul la mutare nu poate muta, adversarul scoate una dintre
//!   piesele sale (mutarea `scoate-<pătrat>`) și blocatul încearcă din nou; dacă nici
//!   adversarul nu poate muta, jocul se oprește.
//! * Câștigă cine rămâne cu piese când adversarul nu mai are; după 100 de mutări fără
//!   captură câștigă cine are mai multe piese (egal = remiză).
//!
//! Notație: plasare `b2`; mutare `b2-b3`; scoatere `scoate-b2`.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const W: usize = 5;
const N: usize = 25;
const CENTER: usize = 12;
const QUIET_LIMIT: u32 = 100;

fn name(i: usize) -> String {
    format!("{}{}", (b'a' + (i % W) as u8) as char, i / W + 1)
}

fn neigh(i: usize) -> Vec<usize> {
    let (r, c) = (i / W, i % W);
    let mut v = Vec::new();
    if r > 0 {
        v.push(i - W);
    }
    if r + 1 < W {
        v.push(i + W);
    }
    if c > 0 {
        v.push(i - 1);
    }
    if c + 1 < W {
        v.push(i + 1);
    }
    v
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Place(u8),
    Step(u8, u8),
    Free(u8),
}

#[derive(Clone)]
pub struct Seega {
    b: [u8; N],
    hand: [u8; 2],
    side: Player,
    placed_turn: u8,
    /// jucătorul blocat care așteaptă ca adversarul să scoată o piesă
    blocked: Option<Player>,
    quiet: u32,
    done: bool,
}

impl Default for Seega {
    fn default() -> Self {
        Self::new()
    }
}

impl Seega {
    pub fn new() -> Self {
        Seega {
            b: [0; N],
            hand: [12, 12],
            side: Player::One,
            placed_turn: 0,
            blocked: None,
            quiet: 0,
            done: false,
        }
    }

    fn placing(&self) -> bool {
        self.hand[0] + self.hand[1] > 0
    }

    fn count(&self, p: Player) -> usize {
        self.b.iter().filter(|&&c| c == p.idx() as u8 + 1).count()
    }

    pub fn pieces(&self, p: Player) -> usize {
        self.count(p) + self.hand[p.idx()] as usize
    }

    fn steps_of(&self, p: Player) -> Vec<Move> {
        let me = p.idx() as u8 + 1;
        let mut v = Vec::new();
        for i in 0..N {
            if self.b[i] == me {
                for j in neigh(i) {
                    if self.b[j] == 0 {
                        v.push(Move::Step(i as u8, j as u8));
                    }
                }
            }
        }
        v
    }

    /// Piesele adverse capturate de mutarea unei piese a lui `me` pe `to`.
    fn captures(&self, me: u8, to: usize) -> Vec<usize> {
        let mut out = Vec::new();
        let (r, c) = ((to / W) as i32, (to % W) as i32);
        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let mut run = Vec::new();
            let (mut rr, mut cc) = (r + dr, c + dc);
            loop {
                if !(0..W as i32).contains(&rr) || !(0..W as i32).contains(&cc) {
                    run.clear();
                    break;
                }
                let k = rr as usize * W + cc as usize;
                let v = self.b[k];
                if v == 0 {
                    run.clear();
                    break;
                }
                if v == me {
                    break;
                }
                run.push(k);
                rr += dr;
                cc += dc;
            }
            out.extend(run);
        }
        out
    }

    fn end_check(&mut self) {
        if self.placing() {
            return;
        }
        for p in [Player::One, Player::Two] {
            if self.count(p) == 0 {
                self.done = true;
                return;
            }
        }
        if self.quiet >= QUIET_LIMIT {
            self.done = true;
            return;
        }
        if self.blocked.is_some() {
            return;
        }
        if self.steps_of(self.side).is_empty() {
            let other = self.side.other();
            if self.steps_of(other).is_empty() {
                self.done = true;
            } else {
                self.blocked = Some(self.side);
                self.side = other;
            }
        }
    }
}

impl Game for Seega {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.done {
            return Vec::new();
        }
        if self.placing() {
            return (0..N)
                .filter(|&i| i != CENTER && self.b[i] == 0)
                .map(|i| Move::Place(i as u8))
                .collect();
        }
        if let Some(bp) = self.blocked {
            let me = bp.other().idx() as u8 + 1;
            return (0..N)
                .filter(|&i| self.b[i] == me)
                .map(|i| Move::Free(i as u8))
                .collect();
        }
        self.steps_of(self.side)
    }

    fn apply(&mut self, m: &Move) {
        let me = self.side;
        let tag = me.idx() as u8 + 1;
        match *m {
            Move::Place(i) => {
                self.b[i as usize] = tag;
                self.hand[me.idx()] -= 1;
                self.placed_turn += 1;
                if self.placed_turn == 2 {
                    self.placed_turn = 0;
                    self.side = me.other();
                }
                if !self.placing() {
                    // începe cine a plasat ultima piesă: `me` (când ultima pereche e a lui)
                    self.side = me;
                    self.end_check();
                }
            }
            Move::Step(a, b) => {
                self.b[a as usize] = 0;
                self.b[b as usize] = tag;
                let caps = self.captures(tag, b as usize);
                if caps.is_empty() {
                    self.quiet += 1;
                    self.side = me.other();
                } else {
                    self.quiet = 0;
                    for k in caps {
                        self.b[k] = 0;
                    }
                }
                self.end_check();
            }
            Move::Free(i) => {
                self.b[i as usize] = 0;
                if let Some(bp) = self.blocked.take() {
                    self.side = bp;
                }
                self.end_check();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.done {
            return None;
        }
        let (a, b) = (self.count(Player::One), self.count(Player::Two));
        Some(match a.cmp(&b) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            _ => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let o = p.other();
        let mat = self.count(p) as i32 - self.count(o) as i32;
        let mob = if self.placing() {
            0
        } else {
            self.steps_of(p).len() as i32 - self.steps_of(o).len() as i32
        };
        let mut pos = 0;
        if self.placing() {
            // în plasare, marginile și colțurile sunt mai greu de prins
            for i in 0..N {
                let (r, c) = (i / W, i % W);
                let edge = r == 0 || r == W - 1 || c == 0 || c == W - 1;
                let v = if edge { 1 } else { 0 };
                if self.b[i] == p.idx() as u8 + 1 {
                    pos += v;
                } else if self.b[i] == o.idx() as u8 + 1 {
                    pos -= v;
                }
            }
        }
        100 * mat + 3 * mob + pos
    }

    fn render(&self) -> String {
        let mut s = String::new();
        for r in (0..W).rev() {
            s.push_str(&format!("{} ", r + 1));
            for c in 0..W {
                let i = r * W + c;
                let ch = match self.b[i] {
                    1 => 'X',
                    2 => 'O',
                    _ if i == CENTER => '+',
                    _ => '.',
                };
                s.push(' ');
                s.push(ch);
            }
            s.push('\n');
        }
        s.push_str("   a b c d e\n");
        s.push_str(&format!(
            "În mână: J1 {}, J2 {}; pe tablă: X {}, O {}\n",
            self.hand[0],
            self.hand[1],
            self.count(Player::One),
            self.count(Player::Two)
        ));
        if self.placing() {
            s.push_str("Faza de plasare (două piese pe tură, fără centru).\n");
        } else if self.blocked.is_some() {
            s.push_str("Blocaj: scoate una dintre piesele tale (scoate-<pătrat>).\n");
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match *m {
            Move::Place(i) => name(i as usize),
            Move::Step(a, b) => format!("{}-{}", name(a as usize), name(b as usize)),
            Move::Free(i) => format!("scoate-{}", name(i as usize)),
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

    fn moving() -> Seega {
        Seega {
            b: [0; N],
            hand: [0, 0],
            side: Player::One,
            placed_turn: 0,
            blocked: None,
            quiet: 0,
            done: false,
        }
    }

    #[test]
    fn placement_two_by_two_and_center_stays_empty() {
        let mut g = Seega::new();
        let mv = Game::legal_moves(&g);
        assert_eq!(mv.len(), 24);
        assert!(!mv.contains(&Move::Place(CENTER as u8)));
        g.apply(&Move::Place(0));
        assert_eq!(g.to_move(), Player::One);
        g.apply(&Move::Place(1));
        assert_eq!(g.to_move(), Player::Two);
    }

    #[test]
    fn last_placer_moves_first_and_must_enter_center() {
        let mut g = Seega::new();
        let mut cells: Vec<usize> = (0..N).filter(|&i| i != CENTER).collect();
        while g.placing() {
            let c = cells.remove(0);
            g.apply(&Move::Place(c as u8));
        }
        assert_eq!(g.to_move(), Player::Two);
        for m in Game::legal_moves(&g) {
            match m {
                Move::Step(_, to) => assert_eq!(to as usize, CENTER),
                _ => panic!("doar mutări în centru"),
            }
        }
    }

    #[test]
    fn custodial_capture_of_a_run() {
        let mut g = moving();
        // rând 1: X . O O X  → X mută în gol și prinde OO? X la a1, mută b? construim: a1 X, b1 O, c1 O, d1 gol, e1 X
        g.b[0] = 1;
        g.b[1] = 2;
        g.b[2] = 2;
        g.b[4] = 1;
        g.b[8] = 1; // d2, va coborî în d1
        g.b[24] = 2;
        g.apply(&Move::Step(8, 3));
        assert_eq!(g.b[1], 0);
        assert_eq!(g.b[2], 0);
        assert_eq!(g.to_move(), Player::One); // mută din nou
    }

    #[test]
    fn no_capture_against_the_edge() {
        let mut g = moving();
        g.b[0] = 2;
        g.b[6] = 1; // b2 → mută la b1?
        g.b[24] = 2;
        g.apply(&Move::Step(6, 1));
        assert_eq!(g.b[0], 2);
        assert_eq!(g.to_move(), Player::Two);
    }

    #[test]
    fn blocked_player_makes_opponent_free_a_piece() {
        let mut g = moving();
        // O în colțul a1 înconjurat de X pe a2 și b1, blocați de X-uri în spate
        g.b[0] = 2;
        g.b[1] = 1;
        g.b[5] = 1;
        g.b[2] = 1; // c1 ocupat de X, a3 ocupat de X → ambele X blocate pe b-uri? nu neapărat
        g.b[10] = 1;
        for i in [
            6, 7, 11, 12, 13, 3, 4, 8, 9, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        ] {
            g.b[i] = 1;
        }
        // tabla plină cu excepția unui singur pătrat (idx 24 devine gol): X mută, apoi O blocat?
        g.b[24] = 0;
        g.side = Player::One;
        // X mută 23 → 24: eliberează 23; O (a1) rămâne blocat
        g.apply(&Move::Step(23, 24));
        assert!(g.blocked.is_some());
        assert_eq!(g.to_move(), Player::One);
        let free = Game::legal_moves(&g);
        assert!(free.iter().all(|m| matches!(m, Move::Free(_))));
        g.apply(&free[0]);
        assert_eq!(g.to_move(), Player::Two);
    }

    #[test]
    fn random_games_terminate() {
        for seed in 1..=25u64 {
            let mut g = Seega::new();
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = Game::legal_moves(&g);
                assert!(!m.is_empty(), "{}", g.render());
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 5000);
            }
        }
    }
}
