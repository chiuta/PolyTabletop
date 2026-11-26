#![allow(clippy::needless_range_loop, clippy::manual_range_contains)]
//! Hounds and Jackals (58 de găuri, Egiptul antic) — reconstrucție modernă a cursei cu cinci piese.
//!
//! Regulile originale nu s-au păstrat; folosim reconstrucția cea mai răspândită:
//!
//! * Fiecare jucător (J1 = câinii, J2 = șacalii) are 5 piese și o pistă proprie de 30 de găuri
//!   (cea de sus, 30, e comună scopului). Se aruncă 4 bețe marcate pe o față: numărul de fețe
//!   marcate în sus (1–4), iar patru fețe nemarcate = 5 (`arunc`, forțat).
//! * O piesă intră pe gaura 1 doar cu aruncarea 1 și cine a intrat aruncă încă o dată (`intra`).
//!   Piesele înaintează cu aruncarea (`p12` = piesa de pe gaura 12) peste piesele proprii, dar nu
//!   pot sări peste o piesă proprie de pe găurile 10, 15, 20 sau 25 și nu pot ateriza peste alta.
//! * Scurtături: gaura 6 duce la 20 și 8 la 10; gaurile 20 și 10 aruncă înapoi la 6 și la 8.
//! * Scopul (gaura 30) se atinge exact; o piesă iese de acolo (`iese`) doar cu aruncarea 1 sau 5 și
//!   doar după ce toate cele 5 piese au intrat pe tablă. Primul care scoate toate piesele câștigă.
//! * Fără mutare posibilă: `pas`.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const GOAL: u8 = 30;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Roll,
    Enter,
    Step(u8),
    Exit,
    Pass,
}

#[derive(Clone)]
pub struct Hounds {
    rng: Rng,
    /// poziții 0 = în afara tablei, 1..=30; None = scoasă
    pieces: [[Option<u8>; 5]; 2],
    roll: Option<u8>,
    side: Player,
    log: String,
}

impl Default for Hounds {
    fn default() -> Self {
        Self::new()
    }
}

fn shortcut(p: u8) -> u8 {
    match p {
        6 => 20,
        8 => 10,
        20 => 6,
        10 => 8,
        x => x,
    }
}

impl Hounds {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Hounds {
            rng: Rng::new(seed),
            pieces: [[Some(0); 5]; 2],
            roll: None,
            side: Player::One,
            log: String::new(),
        }
    }

    fn mine(&self) -> &[Option<u8>; 5] {
        &self.pieces[self.side.idx()]
    }

    fn all_entered(&self) -> bool {
        self.mine().iter().all(|x| *x != Some(0))
    }

    fn blocked_between(&self, from: u8, to: u8) -> bool {
        // piesă proprie pe 10/15/20/25 strict între from și to
        for b in [10u8, 15, 20, 25] {
            if b > from && b < to && self.mine().contains(&Some(b)) {
                return true;
            }
        }
        false
    }

    fn step_target(&self, from: u8, r: u8) -> Option<u8> {
        let to = from + r;
        if to > GOAL || self.blocked_between(from, to) {
            return None;
        }
        if to < GOAL && self.mine().contains(&Some(to)) {
            return None;
        }
        let t = shortcut(to);
        if t != to && self.mine().contains(&Some(t)) {
            return None;
        }
        Some(t)
    }

    fn progress(&self, p: usize) -> i32 {
        self.pieces[p]
            .iter()
            .map(|x| match x {
                None => 40,
                Some(0) => 0,
                Some(n) => *n as i32,
            })
            .sum()
    }
}

impl Game for Hounds {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        let Some(r) = self.roll else {
            return vec![Move::Roll];
        };
        let mut v = Vec::new();
        let m = self.mine();
        if r == 1 && m.contains(&Some(0)) && !m.contains(&Some(1)) {
            v.push(Move::Enter);
        }
        let mut seen = Vec::new();
        for x in m.iter().flatten() {
            if *x == 0 || seen.contains(x) {
                continue;
            }
            seen.push(*x);
            if *x == GOAL {
                if (r == 1 || r == 5) && self.all_entered() {
                    v.push(Move::Exit);
                }
            } else if self.step_target(*x, r).is_some() {
                v.push(Move::Step(*x));
            }
        }
        if v.is_empty() {
            v.push(Move::Pass);
        }
        v
    }

    fn apply(&mut self, mv: &Move) {
        self.log.clear();
        let p = self.side.idx();
        match mv {
            Move::Roll => {
                let k = (0..4).filter(|_| self.rng.below(2) == 0).count() as u8;
                let r = if k == 0 { 5 } else { k };
                self.roll = Some(r);
                self.log = format!("J{} aruncă {}", p + 1, r);
            }
            Move::Enter => {
                let i = self.pieces[p].iter().position(|x| *x == Some(0)).unwrap();
                self.pieces[p][i] = Some(1);
                self.roll = None; // aruncă din nou
            }
            Move::Step(from) => {
                let r = self.roll.take().unwrap();
                let t = self.step_target(*from, r).unwrap();
                let i = self.pieces[p]
                    .iter()
                    .position(|x| *x == Some(*from))
                    .unwrap();
                self.pieces[p][i] = Some(t);
                if t != *from + r {
                    self.log = format!("scurtătură {} → {}", *from + r, t);
                }
                self.side = self.side.other();
            }
            Move::Exit => {
                self.roll = None;
                let i = self.pieces[p]
                    .iter()
                    .position(|x| *x == Some(GOAL))
                    .unwrap();
                self.pieces[p][i] = None;
                self.side = self.side.other();
            }
            Move::Pass => {
                self.roll = None;
                self.side = self.side.other();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        for p in 0..2 {
            if self.pieces[p].iter().all(|x| x.is_none()) {
                return Some(Outcome::Win(if p == 0 { Player::One } else { Player::Two }));
            }
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        (self.progress(p.idx()) - self.progress(1 - p.idx())) * 10
    }

    fn render(&self) -> String {
        let names = ["Câini (J1)", "Șacali (J2)"];
        let mut s = String::from("Hounds and Jackals (58 de găuri)\n");
        for p in 0..2 {
            let txt: Vec<String> = self.pieces[p]
                .iter()
                .map(|x| match x {
                    None => "ieșit".to_string(),
                    Some(0) => "—".to_string(),
                    Some(n) => n.to_string(),
                })
                .collect();
            s.push_str(&format!("{}: {}\n", names[p], txt.join(" ")));
        }
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        if let Some(r) = self.roll {
            s.push_str(&format!("Aruncare: {r}\n"));
        }
        if self.outcome().is_none() {
            s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Roll => "arunc".into(),
            Move::Enter => "intra".into(),
            Move::Step(n) => format!("p{n}"),
            Move::Exit => "iese".into(),
            Move::Pass => "pas".into(),
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
    fn shortcuts_and_blocks() {
        let mut g = Hounds::with_seed(1);
        g.pieces[0] = [Some(4), Some(0), Some(0), Some(0), Some(0)];
        assert_eq!(g.step_target(4, 2), Some(20)); // 6 → 20
        g.pieces[0] = [Some(3), Some(15), Some(0), Some(0), Some(0)];
        assert_eq!(g.step_target(3, 5), Some(10)); // 8 → 10, ok
        g.pieces[0] = [Some(8), Some(10), Some(0), Some(0), Some(0)];
        assert_eq!(g.step_target(8, 3), None); // 11 sare peste 10 ocupat
        g.pieces[0] = [Some(28), Some(0), Some(0), Some(0), Some(0)];
        assert_eq!(g.step_target(28, 3), None); // depășește 30
        assert_eq!(g.step_target(28, 2), Some(30));
    }

    #[test]
    fn enter_only_with_one() {
        let mut g = Hounds::with_seed(1);
        g.roll = Some(3);
        assert_eq!(g.legal_moves(), vec![Move::Pass]);
        g.roll = Some(1);
        assert!(g.legal_moves().contains(&Move::Enter));
    }

    #[test]
    fn exit_needs_all_entered() {
        let mut g = Hounds::with_seed(1);
        g.pieces[0] = [Some(30), Some(0), Some(0), Some(0), Some(0)];
        g.roll = Some(5);
        assert!(!g.legal_moves().contains(&Move::Exit));
        g.pieces[0] = [Some(30), Some(1), Some(2), Some(3), Some(4)];
        assert!(g.legal_moves().contains(&Move::Exit));
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=6u64 {
            let mut g = Hounds::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() && n < 20000 {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed} n={n}");
            assert!(n < 4000, "seed {seed} prea lung: {n}");
        }
    }
}
