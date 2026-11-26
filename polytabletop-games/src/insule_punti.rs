//! Insulele Punților: duel pe 12 insule legate prin 16 rute. Ai 3 cărți deschise (fiecare = o insulă).
//! Joci o carte: ori construiești un pod pe o rută care atinge insula cărții («pun a b»), ori
//! tai un pod al adversarului pe o rută care atinge insula cărții («tai a b»), ori schimbi cartea
//! («descarc a»). După 12, 24 și 36 de mutări are loc o numărătoare: insula e a celui cu mai multe
//! poduri la ea (marcajul se păstrează la egalitate); fiecare insulă marcată aduce 1 punct; apoi
//! podurile se retrag. După a treia numărătoare câștigă cine are mai multe puncte.
//! Variantă originală (hartă, număr de runde și puncte proprii).
#![allow(clippy::needless_range_loop, clippy::manual_is_multiple_of)]

use crate::w7_util::Dice;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: usize = 12;
const LINKS: [(u8, u8); 16] = [
    (0, 1),
    (1, 2),
    (2, 3),
    (3, 4),
    (4, 5),
    (5, 6),
    (6, 7),
    (7, 8),
    (8, 9),
    (9, 10),
    (10, 11),
    (11, 0),
    (0, 6),
    (3, 9),
    (2, 8),
    (5, 11),
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mv {
    Build(u8),
    Cut(u8),
    Swap(u8),
}

#[derive(Clone)]
pub struct Punti {
    bridge: [Option<Player>; 16],
    hand: [[u8; 3]; 2],
    marker: [Option<Player>; N],
    pts: [i32; 2],
    side: Player,
    plies: u32,
    dice: Dice,
}

impl Default for Punti {
    fn default() -> Self {
        Self::new()
    }
}

impl Punti {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut dice = Dice::new(seed);
        let mut hand = [[0u8; 3]; 2];
        for h in hand.iter_mut() {
            for c in h.iter_mut() {
                *c = dice.below(N) as u8;
            }
        }
        Punti {
            bridge: [None; 16],
            hand,
            marker: [None; N],
            pts: [0; 2],
            side: Player::One,
            plies: 0,
            dice,
        }
    }

    fn score_round(&mut self) {
        for i in 0..N {
            let mut n = [0; 2];
            for (l, &(a, b)) in LINKS.iter().enumerate() {
                if let Some(p) = self.bridge[l] {
                    if a as usize == i || b as usize == i {
                        n[p.idx()] += 1;
                    }
                }
            }
            if n[0] > n[1] {
                self.marker[i] = Some(Player::One);
            } else if n[1] > n[0] {
                self.marker[i] = Some(Player::Two);
            }
        }
        for p in 0..2 {
            let me = if p == 0 { Player::One } else { Player::Two };
            self.pts[p] += self.marker.iter().filter(|&&m| m == Some(me)).count() as i32;
        }
        self.bridge = [None; 16];
    }
}

impl Game for Punti {
    type Move = Mv;
    // Mutarea codifică ruta/cartea ca u8: Build(link) e într-un singur octet pentru ruta aleasă;
    // cartea folosită se alege automat (prima care atinge ruta).

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if self.outcome().is_some() {
            return vec![];
        }
        let h = self.hand[self.side.idx()];
        let touches = |l: usize| {
            let (a, b) = LINKS[l];
            h.iter().any(|&c| c == a || c == b)
        };
        let mut v = vec![];
        for l in 0..16 {
            if touches(l) {
                match self.bridge[l] {
                    None => v.push(Mv::Build(l as u8)),
                    Some(p) if p != self.side => v.push(Mv::Cut(l as u8)),
                    _ => {}
                }
            }
        }
        let mut seen = vec![];
        for &c in &h {
            if !seen.contains(&c) {
                seen.push(c);
                v.push(Mv::Swap(c));
            }
        }
        v
    }

    fn apply(&mut self, m: &Mv) {
        let p = self.side.idx();
        let used = match *m {
            Mv::Build(l) => {
                self.bridge[l as usize] = Some(self.side);
                let (a, b) = LINKS[l as usize];
                self.hand[p]
                    .iter()
                    .position(|&c| c == a)
                    .or(self.hand[p].iter().position(|&c| c == b))
            }
            Mv::Cut(l) => {
                self.bridge[l as usize] = None;
                let (a, b) = LINKS[l as usize];
                self.hand[p]
                    .iter()
                    .position(|&c| c == a)
                    .or(self.hand[p].iter().position(|&c| c == b))
            }
            Mv::Swap(c) => self.hand[p].iter().position(|&x| x == c),
        };
        if let Some(i) = used {
            self.hand[p][i] = self.dice.below(N) as u8;
        }
        self.plies += 1;
        if self.plies % 12 == 0 {
            self.score_round();
        }
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.plies < 36 {
            return None;
        }
        Some(match self.pts[0].cmp(&self.pts[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            _ => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let mut v = (self.pts[p.idx()] - self.pts[p.other().idx()]) * 20;
        for i in 0..N {
            let mut n = [0i32; 2];
            for (l, &(a, b)) in LINKS.iter().enumerate() {
                if let Some(q) = self.bridge[l] {
                    if a as usize == i || b as usize == i {
                        n[q.idx()] += 1;
                    }
                }
            }
            v += (n[p.idx()] - n[p.other().idx()]).signum() * 4 + n[p.idx()] - n[p.other().idx()];
        }
        v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Mutarea {}/36 | puncte J1 {} - J2 {} | la rând J{}\n",
            self.plies,
            self.pts[0],
            self.pts[1],
            self.side.idx() + 1
        );
        for (l, &(a, b)) in LINKS.iter().enumerate() {
            s.push_str(&format!(
                "{}-{}:{} ",
                a + 1,
                b + 1,
                match self.bridge[l] {
                    Some(Player::One) => "J1",
                    Some(Player::Two) => "J2",
                    None => "..",
                }
            ));
        }
        s.push_str("\nMarcaje: ");
        for i in 0..N {
            s.push_str(&format!(
                "{}{} ",
                i + 1,
                match self.marker[i] {
                    Some(Player::One) => "(1)",
                    Some(Player::Two) => "(2)",
                    None => "",
                }
            ));
        }
        for q in 0..2 {
            s.push_str(&format!("\nCărțile lui J{}: ", q + 1));
            for &c in &self.hand[q] {
                s.push_str(&format!("{} ", c + 1));
            }
        }
        s.push_str("\nMutări: «pun a b», «tai a b», «descarc a».\n");
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        match *m {
            Mv::Build(l) => format!(
                "pun {} {}",
                LINKS[l as usize].0 + 1,
                LINKS[l as usize].1 + 1
            ),
            Mv::Cut(l) => format!(
                "tai {} {}",
                LINKS[l as usize].0 + 1,
                LINKS[l as usize].1 + 1
            ),
            Mv::Swap(c) => format!("descarc {}", c + 1),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_games_end_in_36_plies() {
        for seed in 1..=30u64 {
            let mut g = Punti::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
            }
            assert_eq!(n, 36);
        }
    }

    #[test]
    fn ai_legal_and_not_losing_to_random() {
        polytabletop_core::ai::set_time_budget_override_ms(Some(5));
        let mut wins = 0;
        for seed in 1..=8u64 {
            let mut g = Punti::with_seed(seed);
            let mut rng = Rng::new(seed);
            while g.outcome().is_none() {
                if g.side == Player::One {
                    let t = polytabletop_core::GameObj::ai_move(&g, rng.next_u64()).unwrap();
                    assert!(polytabletop_core::GameObj::legal_moves(&g).contains(&t));
                    polytabletop_core::GameObj::play(&mut g, &t).unwrap();
                } else {
                    let mv = g.legal_moves();
                    g.apply(&mv[rng.below(mv.len())]);
                }
            }
            if g.outcome() == Some(Outcome::Win(Player::One)) {
                wins += 1;
            }
        }
        assert!(wins >= 4, "{wins}");
    }
}
