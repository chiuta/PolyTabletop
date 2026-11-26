//! Fotbal de masă pe ture (Table football, Europa) — adaptare cu mutări alternative, fără îndemânare.
//!
//! Terenul are 8 bare (de la poarta lui J1 la a lui J2): portar J1, apărare J1, atac J2, mijloc J1,
//! mijloc J2, atac J1, apărare J2, portar J2. Fiecare bară acoperă 2 culoare alăturate din 5
//! (poziția 1–4 = culoarele `p` și `p+1`). Poarta e deschisă pe culoarele 2–4.
//!
//! * O tură = ori muți o bară proprie într-o altă poziție (`atac 3`), ori — dacă ai mingea —
//!   pasezi/șutezi pe un culoar acoperit de bara cu mingea (`trimit 2`).
//! * Mingea merge spre poarta adversă și se oprește la prima bară (a oricui) care acoperă
//!   culoarul: a ta = pasă reușită (acum mingea e a ta), a adversarului = interceptare. Dacă nicio
//!   bară nu acoperă culoarul până la portarul advers: gol dacă culoarul e 2–4, altfel afară
//!   (portarul advers ia mingea).
//! * Primul la 5 goluri câștigă; după 300 de mutări câștigă cine conduce (egal = remiză).
//!
//! Regulile reale ale „fotbalului de masă” depind de îndemânare; aceasta e o adaptare originală
//! cu informație perfectă.

#![allow(clippy::needless_range_loop)]

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const TEAM: [usize; 8] = [0, 0, 1, 0, 1, 0, 1, 1];
const ROLE: [&str; 8] = [
    "portar", "aparare", "atac", "mijloc", "mijloc", "atac", "aparare", "portar",
];
const GOALS_TO_WIN: u8 = 5;
const PLY_CAP: u16 = 300;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mv {
    Rod { col: u8, pos: u8 },
    Send(u8),
}

#[derive(Clone)]
pub struct FotbalMasa {
    pos: [u8; 8],
    ball: u8,
    side: Player,
    score: [u8; 2],
    ply: u16,
    log: String,
}

impl Default for FotbalMasa {
    fn default() -> Self {
        Self::new()
    }
}

fn covers(pos: u8, lane: u8) -> bool {
    lane == pos || lane == pos + 1
}

impl FotbalMasa {
    pub fn new() -> Self {
        FotbalMasa {
            pos: [2, 2, 2, 2, 2, 2, 2, 2],
            ball: 1,
            side: Player::One,
            score: [0; 2],
            ply: 0,
            log: String::new(),
        }
    }

    pub fn score(&self) -> (u8, u8) {
        (self.score[0], self.score[1])
    }

    fn holder(&self) -> usize {
        TEAM[self.ball as usize]
    }

    fn rods_of(&self, p: usize) -> Vec<usize> {
        (0..8).filter(|&c| TEAM[c] == p).collect()
    }

    /// Numele rodului `c` în text, cu indicele între cele cu același rol (`mijloc`, `atac` apar o singură dată pe echipă).
    fn rod_name(c: usize) -> &'static str {
        ROLE[c]
    }

    fn trace(&self, shooter: usize, lane: u8) -> (Option<usize>, bool) {
        // (bara care oprește, gol?)
        let cols: Vec<usize> = if shooter == 0 {
            ((self.ball as usize + 1)..8).collect()
        } else {
            (0..self.ball as usize).rev().collect()
        };
        for c in cols {
            if covers(self.pos[c], lane) {
                return (Some(c), false);
            }
        }
        (None, (2..=4).contains(&lane))
    }
}

impl Game for FotbalMasa {
    type Move = Mv;
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<Mv> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        let p = self.side.idx();
        let mut v = Vec::new();
        for c in self.rods_of(p) {
            for pos in 1..=4u8 {
                if self.pos[c] != pos {
                    v.push(Mv::Rod { col: c as u8, pos });
                }
            }
        }
        if self.holder() == p {
            let b = self.ball as usize;
            for lane in 1..=5u8 {
                if covers(self.pos[b], lane) {
                    v.push(Mv::Send(lane));
                }
            }
        }
        v
    }
    fn apply(&mut self, m: &Mv) {
        let p = self.side.idx();
        self.ply += 1;
        match *m {
            Mv::Rod { col, pos } => {
                self.pos[col as usize] = pos;
                self.log = format!(
                    "J{} mută bara {} în poziția {}.",
                    p + 1,
                    Self::rod_name(col as usize),
                    pos
                );
            }
            Mv::Send(lane) => {
                let (stop, goal) = self.trace(p, lane);
                match stop {
                    Some(c) => {
                        self.ball = c as u8;
                        self.log = if TEAM[c] == p {
                            format!("Pasă reușită pe culoarul {lane}.")
                        } else {
                            format!("Interceptat pe culoarul {lane}!")
                        };
                    }
                    None => {
                        let gk = if p == 0 { 7 } else { 0 };
                        self.ball = gk;
                        if goal {
                            self.score[p] += 1;
                            self.log = format!(
                                "GOL! J{} marchează ({}–{}).",
                                p + 1,
                                self.score[0],
                                self.score[1]
                            );
                        } else {
                            self.log = "Șut afară; portarul ia mingea.".into();
                        }
                        self.side = self.side.other();
                        return;
                    }
                }
            }
        }
        self.side = self.side.other();
    }
    fn outcome(&self) -> Option<Outcome> {
        for p in 0..2 {
            if self.score[p] >= GOALS_TO_WIN {
                return Some(Outcome::Win(if p == 0 { Player::One } else { Player::Two }));
            }
        }
        if self.ply >= PLY_CAP {
            return Some(match self.score[0].cmp(&self.score[1]) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                _ => Outcome::Draw,
            });
        }
        None
    }
    fn evaluate(&self, pl: Player) -> i32 {
        let p = pl.idx();
        let mut v = 1000 * (self.score[p] as i32 - self.score[1 - p] as i32);
        let b = self.ball as i32;
        let adv = if self.holder() == 0 { b } else { 7 - b };
        let sign = if self.holder() == p { 1 } else { -1 };
        v += sign * (30 + 10 * adv);
        // amenințări: culoare de gol libere pentru deținătorul mingii
        let h = self.holder();
        let mut threats = 0;
        for lane in 1..=5u8 {
            if covers(self.pos[b as usize], lane) {
                let (stop, goal) = self.trace(h, lane);
                if stop.is_none() && goal {
                    threats += 1;
                }
            }
        }
        v += sign * 150 * threats;
        v
    }
    fn render(&self) -> String {
        let mut s = String::new();
        s.push_str("poarta J1 |");
        for c in 0..8 {
            s.push_str(&format!(
                " {}{}",
                if TEAM[c] == 0 { "J1:" } else { "J2:" },
                ROLE[c]
            ));
            if c < 7 {
                s.push_str(" |");
            }
        }
        s.push_str(" | poarta J2\n");
        for lane in (1..=5u8).rev() {
            s.push_str(&format!("culoar {lane} "));
            for c in 0..8 {
                let mark = if covers(self.pos[c], lane) {
                    if TEAM[c] == 0 {
                        'X'
                    } else {
                        'O'
                    }
                } else {
                    '.'
                };
                let ball = if self.ball as usize == c && covers(self.pos[c], lane) {
                    '*'
                } else {
                    ' '
                };
                s.push_str(&format!("   {mark}{ball}   "));
            }
            s.push('\n');
        }
        s.push_str(&format!(
            "Scor: J1 {} – {} J2 (primul la {GOALS_TO_WIN}). Mingea e la bara {} a lui J{}. {}\n",
            self.score[0],
            self.score[1],
            ROLE[self.ball as usize],
            self.holder() + 1,
            self.log
        ));
        s.push_str(&format!("La mutare: Jucătorul {}.", self.side.idx() + 1));
        s
    }
    fn format_move(&self, m: &Mv) -> String {
        match *m {
            Mv::Rod { col, pos } => {
                // o echipă are bare cu același rol (mijloc, atac) o singură dată
                format!("{} {}", ROLE[col as usize], pos)
            }
            Mv::Send(l) => format!("trimit {l}"),
        }
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 250,
            max_depth: 6,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q4_util::*;
    use polytabletop_core::{GameObj, Player};

    #[test]
    fn goal_when_lane_open_and_keeper_misses() {
        let mut g = FotbalMasa::new();
        g.pos = [1, 1, 1, 1, 1, 4, 1, 1]; // toate acoperă culoarele 1–2 în afară de J1 atac (4–5)
        g.ball = 5; // atac J1 (col 5) deține mingea
                    // pe culoarul 4: bara 6 (apărare J2) acoperă 1–2, portar J2 (col 7) 1–2 -> gol
        GameObj::play(&mut g, "trimit 4").unwrap();
        assert_eq!(g.score(), (1, 0));
        assert_eq!(g.ball, 7, "portarul J2 reia");
    }

    #[test]
    fn interception_changes_holder() {
        let mut g = FotbalMasa::new();
        g.pos = [2, 2, 2, 2, 2, 2, 2, 2];
        g.ball = 5;
        GameObj::play(&mut g, "trimit 2").unwrap(); // bara 6 (J2) acoperă 2–3
        assert_eq!(g.holder(), 1);
    }

    #[test]
    fn random_and_ai_games_finish() {
        random_finishes(|_| Box::new(FotbalMasa::new()), 6, 400);
        ai_vs_random(|_| Box::new(FotbalMasa::new()), 2, Player::One, 400);
        ai_vs_random(|_| Box::new(FotbalMasa::new()), 3, Player::Two, 400);
    }
}
