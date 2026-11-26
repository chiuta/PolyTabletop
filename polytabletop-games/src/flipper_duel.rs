//! Flipper (variantă proprie de pinball pe ture, duel de scor): masa are 7 coloane (0–6) și 12
//! rânduri; are bumpere (b) și ținte (T, o singură lovitură pe bilă). Fiecare jucător are 3
//! bile, alternativ. O bilă: lansezi (coloana de lansare 0–6 și puterea 1–3: `3/2`); bila urcă
//! drept până la înălțimea dată de putere, apoi coboară în diagonală, ricoșând de pereți
//! și de bumpere (100 de puncte fiecare lovitură; ținta valorează 300). Când bila ajunge jos,
//! o poți relansa cu paleta stângă (`st`, bila trebuie să fie în coloanele 0–3) sau dreaptă
//! (`dr`, coloanele 3–6), maxim de 4 ori; `nu` o lasă să cadă (bila se termină). Relansarea
//! trimite bila în sus, înclinată spre cealaltă parte. Câștigă cine are scorul mai mare.
//! Mutări: lansare `3/2`, relansare `st` / `dr`, `nu`.
#![allow(clippy::all)]

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const W: i32 = 7;
const H: i32 = 12;
const BUMPERS: [(i32, i32); 6] = [(2, 5), (4, 5), (3, 7), (1, 8), (5, 8), (3, 3)];
const TARGETS: [(i32, i32); 3] = [(1, 11), (3, 11), (5, 11)];

#[derive(Clone, PartialEq, Debug)]
pub enum Mv {
    Launch(u8, u8),
    Left,
    Right,
    Drain,
}

#[derive(Clone)]
pub struct FlipperDuel {
    score: [i32; 2],
    balls: [u8; 2], // bile rămase de lansat (inclusiv cea curentă)
    side: Player,
    in_play: bool, // bila e jos, așteaptă paleta
    ball_x: i32,
    flips: u8,
    hit_targets: u8,
    ply: u32,
}

impl Default for FlipperDuel {
    fn default() -> Self {
        Self::new()
    }
}

impl FlipperDuel {
    pub fn new() -> Self {
        FlipperDuel {
            score: [0; 2],
            balls: [3, 3],
            side: Player::One,
            in_play: false,
            ball_x: 0,
            flips: 0,
            hit_targets: 0,
            ply: 0,
        }
    }
    /// Simulează urcarea până la `apex` și coborârea; întoarce coloana de aterizare.
    fn run(&mut self, x0: i32, dx0: i32, apex: i32) -> i32 {
        let p = self.side.idx();
        let (mut x, mut y, mut dx, mut dy) = (x0, 0, dx0, 1);
        let mut steps = 0;
        while steps < 60 {
            steps += 1;
            let (mut nx, mut ny) = (x + dx, y + dy);
            if nx < 0 || nx >= W {
                dx = -dx;
                nx = (x + dx).clamp(0, W - 1);
            }
            if ny >= H {
                dy = -1;
                ny = y;
            } else if dy > 0 && ny > apex {
                dy = -1;
                ny = y;
            }
            if ny <= 0 && dy < 0 {
                x = nx;
                break;
            }
            if BUMPERS.contains(&(nx, ny)) {
                self.score[p] += 100;
                // ricoșează: schimbă sensul vertical și lateral
                dy = -dy;
                dx = if (nx + ny) % 2 == 0 { 1 } else { -1 };
                continue;
            }
            if let Some(k) = TARGETS.iter().position(|&t| t == (nx, ny)) {
                if self.hit_targets >> k & 1 == 0 {
                    self.hit_targets |= 1 << k;
                    self.score[p] += 300;
                }
                dy = -dy;
                continue;
            }
            x = nx;
            y = ny;
        }
        x.clamp(0, W - 1)
    }
    fn end_ball(&mut self) {
        let p = self.side.idx();
        self.balls[p] -= 1;
        self.in_play = false;
        self.flips = 0;
        self.hit_targets = 0;
        self.side = self.side.other();
    }
}

impl Game for FlipperDuel {
    type Move = Mv;
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<Mv> {
        if self.outcome().is_some() {
            return vec![];
        }
        if !self.in_play {
            let mut v = vec![];
            for x in 0..W as u8 {
                for pw in 1..=3 {
                    v.push(Mv::Launch(x, pw));
                }
            }
            return v;
        }
        let mut v = vec![Mv::Drain];
        if self.flips < 4 {
            if self.ball_x <= 3 {
                v.push(Mv::Left);
            }
            if self.ball_x >= 3 {
                v.push(Mv::Right);
            }
        }
        v
    }
    fn apply(&mut self, m: &Mv) {
        self.ply += 1;
        match m {
            Mv::Launch(x, pw) => {
                self.ball_x = self.run(*x as i32, 0, 2 + *pw as i32 * 3);
                self.in_play = true;
                self.flips = 0;
                self.hit_targets = 0;
            }
            Mv::Left => {
                self.flips += 1;
                self.ball_x = self.run(self.ball_x, 1, 9);
            }
            Mv::Right => {
                self.flips += 1;
                self.ball_x = self.run(self.ball_x, -1, 9);
            }
            Mv::Drain => self.end_ball(),
        }
    }
    fn outcome(&self) -> Option<Outcome> {
        if self.balls[0] == 0 && self.balls[1] == 0 {
            return Some(match self.score[0].cmp(&self.score[1]) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                _ => Outcome::Draw,
            });
        }
        None
    }
    fn evaluate(&self, p: Player) -> i32 {
        self.score[p.idx()] - self.score[1 - p.idx()]
    }
    fn render(&self) -> String {
        let mut s = String::new();
        for y in (0..H).rev() {
            s.push_str(&format!("{:>2} ", y));
            for x in 0..W {
                let c = if self.in_play && y == 0 && x == self.ball_x {
                    '@'
                } else if BUMPERS.contains(&(x, y)) {
                    'b'
                } else if let Some(k) = TARGETS.iter().position(|&t| t == (x, y)) {
                    if self.hit_targets >> k & 1 == 1 {
                        'x'
                    } else {
                        'T'
                    }
                } else {
                    '.'
                };
                s.push(c);
                s.push(' ');
            }
            s.push('\n');
        }
        s.push_str("   0 1 2 3 4 5 6\n");
        s.push_str(&format!(
            "Scor: J1 {}, J2 {}; bile rămase: J1 {}, J2 {}\n",
            self.score[0], self.score[1], self.balls[0], self.balls[1]
        ));
        if self.in_play {
            s.push_str(&format!(
                "Bila e jos, în coloana {} (relansări folosite: {}/4).\n",
                self.ball_x, self.flips
            ));
        } else {
            s.push_str("Lansează bila: coloană/putere (ex. 3/2).\n");
        }
        s
    }
    fn format_move(&self, m: &Mv) -> String {
        match m {
            Mv::Launch(x, p) => format!("{x}/{p}"),
            Mv::Left => "st".into(),
            Mv::Right => "dr".into(),
            Mv::Drain => "nu".into(),
        }
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::b1_util::testing::check;
    #[test]
    fn launch_flip_drain_cycle() {
        let mut g = FlipperDuel::new();
        assert_eq!(g.legal_moves().len(), 21);
        g.apply(&Mv::Launch(3, 3));
        assert!(g.in_play);
        let m = g.legal_moves();
        assert!(m.contains(&Mv::Drain));
        g.apply(&Mv::Drain);
        assert_eq!(g.balls, [2, 3]);
        assert_eq!(g.to_move(), Player::Two);
    }
    #[test]
    fn bumper_scores() {
        let mut g = FlipperDuel::new();
        // urcă pe coloana 3 până la apex 11: lovește bumperul (3,3) și apoi altele
        g.apply(&Mv::Launch(3, 3));
        assert!(g.score[0] >= 100);
    }
    #[test]
    fn random_and_ai() {
        check(|_| FlipperDuel::new(), 400);
    }
}
