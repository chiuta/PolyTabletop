//! Ascunselea (Hide-and-seek), 2 jucători — adaptare pe un curte 3×3.
//!
//! * 4 runde; în rundele 1 și 3 ascunde J1, în 2 și 4 ascunde J2. Ascunzătorul alege în secret una
//!   din cele 9 ascunzători (a1–c3) cu `ascunde b2`.
//! * Căutătorul are 3 încercări (`cauta b2`). După o greșeală primește un indiciu public: «cald» dacă
//!   ascunzătoarea e vecină (inclusiv diagonal) cu locul ales, «rece» altfel.
//! * Găsit din prima încercare: 3 puncte pentru căutător, din a doua 2, din a treia 1; dacă nu e găsit,
//!   ascunzătorul primește 2 puncte. Cine are mai multe puncte după 4 runde câștigă.
//! * Informație ascunsă: locul ales; `render` îl arată doar ascunzătorului (sau la final de rundă).
//!   AI-ul nu citește locul ascunzătorului atunci când caută.
#![allow(clippy::manual_is_multiple_of)]

use polytabletop_core::{hash_mix, AiStyle, Game, Outcome, Player};

const ROUNDS: u8 = 4;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Hide(u8),
    Seek(u8),
}

#[derive(Clone)]
pub struct Ascunselea {
    round: u8,
    spot: Option<u8>,
    guesses: Vec<u8>,
    score: [i32; 2],
    log: String,
    viewer: Option<Player>,
    salt: u64,
}

impl Default for Ascunselea {
    fn default() -> Self {
        Self::new()
    }
}

fn name(c: u8) -> String {
    format!("{}{}", (b'a' + c % 3) as char, c / 3 + 1)
}
fn near(a: u8, b: u8) -> bool {
    let (ax, ay, bx, by) = (
        (a % 3) as i32,
        (a / 3) as i32,
        (b % 3) as i32,
        (b / 3) as i32,
    );
    (ax - bx).abs() <= 1 && (ay - by).abs() <= 1
}

impl Ascunselea {
    pub fn new() -> Self {
        Self::with_seed(polytabletop_core::rng::Rng::from_clock().next_u64())
    }
    pub fn with_seed(salt: u64) -> Self {
        Ascunselea {
            round: 0,
            spot: None,
            guesses: vec![],
            score: [0, 0],
            log: String::new(),
            viewer: Some(Player::One),
            salt,
        }
    }
    fn hider(&self) -> Player {
        if self.round % 2 == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
}

impl Game for Ascunselea {
    type Move = Move;

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }

    fn to_move(&self) -> Player {
        if self.spot.is_none() {
            self.hider()
        } else {
            self.hider().other()
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.round >= ROUNDS {
            return vec![];
        }
        if self.spot.is_none() {
            (0..9).map(Move::Hide).collect()
        } else {
            (0..9)
                .filter(|c| !self.guesses.contains(c))
                .map(Move::Seek)
                .collect()
        }
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        match *m {
            Move::Hide(c) => {
                self.spot = Some(c);
                self.guesses.clear();
            }
            Move::Seek(c) => {
                let spot = self.spot.unwrap();
                let seeker = self.hider().other();
                self.guesses.push(c);
                if c == spot {
                    let pts = 4 - self.guesses.len() as i32;
                    self.score[seeker.idx()] += pts;
                    self.log = format!(
                        "J{} a găsit ascunzătoarea {} din {} încercări: +{pts}.",
                        seeker.idx() + 1,
                        name(spot),
                        self.guesses.len()
                    );
                    self.next_round();
                } else if self.guesses.len() == 3 {
                    self.score[self.hider().idx()] += 2;
                    self.log = format!(
                        "Negăsit! Ascunzătoarea era {}: +2 pentru ascunzător.",
                        name(spot)
                    );
                    self.next_round();
                } else {
                    self.log = format!(
                        "{}: {}.",
                        name(c),
                        if near(c, spot) { "cald" } else { "rece" }
                    );
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.round < ROUNDS {
            return None;
        }
        Some(match self.score[0].cmp(&self.score[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        // Nu citește locul ascuns decât pentru ascunzătorul însuși; zgomot determinist pentru varietate.
        let mut v = (self.score[p.idx()] - self.score[p.other().idx()]) * 100;
        let mut h = self.salt ^ (self.round as u64) << 8;
        if p == self.hider() {
            if let Some(s) = self.spot {
                h ^= (s as u64 + 1) << 16;
            }
        }
        for g in &self.guesses {
            h ^= (*g as u64 + 1) << 24;
        }
        v += (hash_mix(h) % 9) as i32;
        v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Ascunselea — runda {}/{ROUNDS}, scor J1 {} – J2 {}\n",
            (self.round + 1).min(ROUNDS),
            self.score[0],
            self.score[1]
        );
        if self.round < ROUNDS {
            s.push_str(&format!(
                "Ascunde: J{} | caută: J{}\n",
                self.hider().idx() + 1,
                self.hider().other().idx() + 1
            ));
            s.push_str("   a b c\n");
            let v = self.viewer.unwrap_or_else(|| self.to_move());
            for y in 0..3u8 {
                s.push_str(&format!("{}  ", y + 1));
                for x in 0..3u8 {
                    let c = y * 3 + x;
                    let ch = if self.guesses.contains(&c) {
                        'x'
                    } else if self.spot == Some(c) && v == self.hider() {
                        'H'
                    } else {
                        '.'
                    };
                    s.push(ch);
                    s.push(' ');
                }
                s.push('\n');
            }
            if self.spot.is_some() {
                s.push_str(&format!("Încercări folosite: {}/3\n", self.guesses.len()));
            }
        }
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        match self.outcome() {
            Some(Outcome::Win(p)) => s.push_str(&format!("Jucătorul {} câștigă.\n", p.idx() + 1)),
            Some(Outcome::Draw) => s.push_str("Remiză.\n"),
            None => s.push_str(&format!(
                "La mutare: Jucătorul {}\n",
                self.to_move().idx() + 1
            )),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Hide(c) => format!("ascunde {}", name(*c)),
            Move::Seek(c) => format!("cauta {}", name(*c)),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

impl Ascunselea {
    fn next_round(&mut self) {
        self.round += 1;
        self.spot = None;
        self.guesses.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    #[test]
    fn random_game_ends() {
        let mut g = Ascunselea::with_seed(1);
        let mut r = Rng::new(2);
        let mut n = 0;
        while g.outcome().is_none() {
            let m = g.legal_moves();
            g.apply(&m[r.below(m.len())]);
            n += 1;
            assert!(n < 100);
        }
    }

    #[test]
    fn seeker_cannot_see_spot() {
        let mut g = Ascunselea::with_seed(1);
        g.apply(&Move::Hide(4));
        g.set_viewer(Some(Player::Two));
        assert!(!g.render().contains('H'));
        g.set_viewer(Some(Player::One));
        assert!(g.render().contains('H'));
    }

    #[test]
    fn ai_choice_ignores_hidden_spot() {
        // evaluate al căutătorului nu depinde de loc
        let mut a = Ascunselea::with_seed(5);
        let mut b = Ascunselea::with_seed(5);
        a.apply(&Move::Hide(0));
        b.apply(&Move::Hide(8));
        for m in [Move::Seek(3), Move::Seek(6)] {
            let (mut x, mut y) = (a.clone(), b.clone());
            x.apply(&m);
            y.apply(&m);
            // atenție: indiciul poate diferi în log, dar evaluarea căutătorului e identică
            assert_eq!(x.evaluate(Player::Two), y.evaluate(Player::Two));
        }
    }
}
