//! Simon zice (Simon Says, Marea Britanie) — joc pentru copii, în doi, pe runde.
//!
//! Un jucător e „Simon” și dă o poruncă; celălalt o execută **numai** dacă porunca începe cu „Simon zice”.
//! * Simon alege una din 6 acțiuni, cu prefix (`simon-zice sari`) sau fără (`ordin sari`).
//! * Cel care ascultă răspunde: `execut sari` (alege exact acțiunea) sau `stau`.
//! * Corect = a executat acțiunea cerută după „Simon zice” sau a stat la o poruncă fără prefix.
//!   Dacă cel care ascultă a răspuns corect, primește el punctul; dacă a greșit, punctul merge la Simon.
//! * 12 runde, rolurile alternează; mai multe puncte câștigă.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const ACTIONS: [&str; 6] = [
    "sari",
    "aplauda",
    "atinge-nasul",
    "ghemuieste-te",
    "ridica-mana",
    "invarte-te",
];
const ROUNDS: u8 = 12;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mv {
    Order { action: u8, simon: bool },
    Do(u8),
    Stay,
}

#[derive(Clone)]
pub struct Simon {
    score: [u8; 2],
    round: u8,
    cmd: Option<(u8, bool)>,
    noise: u64,
    rng: Rng,
    log: String,
}

impl Default for Simon {
    fn default() -> Self {
        Self::new()
    }
}

impl Simon {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let noise = rng.next_u64();
        Simon {
            score: [0; 2],
            round: 1,
            cmd: None,
            noise,
            rng,
            log: String::new(),
        }
    }
    pub fn scores(&self) -> (u8, u8) {
        (self.score[0], self.score[1])
    }
    fn leader(&self) -> usize {
        ((self.round - 1) % 2) as usize
    }
    fn actor(&self) -> usize {
        if self.cmd.is_some() {
            1 - self.leader()
        } else {
            self.leader()
        }
    }
}

impl Game for Simon {
    type Move = Mv;
    fn to_move(&self) -> Player {
        if self.actor() == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn legal_moves(&self) -> Vec<Mv> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        if self.cmd.is_none() {
            let mut v = Vec::new();
            for a in 0..6u8 {
                v.push(Mv::Order {
                    action: a,
                    simon: true,
                });
                v.push(Mv::Order {
                    action: a,
                    simon: false,
                });
            }
            v
        } else {
            let mut v: Vec<Mv> = (0..6u8).map(Mv::Do).collect();
            v.push(Mv::Stay);
            v
        }
    }
    fn apply(&mut self, m: &Mv) {
        match *m {
            Mv::Order { action, simon } => {
                self.cmd = Some((action, simon));
            }
            Mv::Do(_) | Mv::Stay => {
                let (action, simon) = self.cmd.take().unwrap();
                let correct = match *m {
                    Mv::Do(a) => simon && a == action,
                    _ => !simon,
                };
                let listener = 1 - self.leader();
                let winner = if correct { listener } else { self.leader() };
                self.score[winner] += 1;
                self.log = format!(
                    "Runda {}: {} {} — J{} a răspuns {}. Punct pentru J{}.",
                    self.round,
                    if simon { "Simon zice" } else { "Fără Simon:" },
                    ACTIONS[action as usize],
                    listener + 1,
                    if correct { "corect" } else { "greșit" },
                    winner + 1
                );
                self.round += 1;
                self.noise = self.rng.next_u64();
            }
        }
    }
    fn outcome(&self) -> Option<Outcome> {
        if self.round > ROUNDS {
            return Some(match self.score[0].cmp(&self.score[1]) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                _ => Outcome::Draw,
            });
        }
        None
    }
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut v = 100 * (self.score[me] as i32 - self.score[1 - me] as i32);
        // alegerea ordinului de către Simon: alegere „cu zgomot” (nu poate fi ghicită din starea publică)
        if let Some((a, s)) = self.cmd {
            let h =
                (self.noise ^ ((a as u64) << 3 | s as u64).wrapping_mul(0x9E3779B97F4A7C15)) >> 40;
            let n = (h % 40) as i32;
            v += if self.leader() == me { n } else { -n };
        }
        v
    }
    fn render(&self) -> String {
        let mut s = format!(
            "Runda {}/{ROUNDS}. Scor: J1 {} – {} J2. Simon = J{}.\n",
            self.round.min(ROUNDS),
            self.score[0],
            self.score[1],
            self.leader() + 1
        );
        if let Some((a, simon)) = self.cmd {
            s.push_str(&format!(
                "Porunca: «{}{}»\n",
                if simon { "Simon zice " } else { "" },
                ACTIONS[a as usize]
            ));
        }
        s.push_str(self.log.trim());
        s.push_str(&format!("\nLa mutare: Jucătorul {}.", self.actor() + 1));
        s
    }
    fn format_move(&self, m: &Mv) -> String {
        match *m {
            Mv::Order { action, simon } => format!(
                "{} {}",
                if simon { "simon-zice" } else { "ordin" },
                ACTIONS[action as usize]
            ),
            Mv::Do(a) => format!("execut {}", ACTIONS[a as usize]),
            Mv::Stay => "stau".into(),
        }
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q4_util::*;
    use polytabletop_core::{GameObj, Player};

    #[test]
    fn only_simon_says_commands_are_obeyed() {
        let mut g = Simon::with_seed(1);
        GameObj::play(&mut g, "ordin sari").unwrap();
        GameObj::play(&mut g, "execut sari").unwrap(); // greșit: nu a zis Simon
        assert_eq!(g.scores(), (1, 0));
        GameObj::play(&mut g, "simon-zice aplauda").unwrap(); // runda 2: J2 e Simon
        GameObj::play(&mut g, "execut aplauda").unwrap(); // corect pentru J1
        assert_eq!(g.scores(), (2, 0));
    }

    #[test]
    fn random_and_ai_games_finish() {
        random_finishes(|s| Box::new(Simon::with_seed(s)), 8, 200);
        let r = ai_vs_random(|s| Box::new(Simon::with_seed(s)), 2, Player::One, 200);
        assert_ne!(r, Some(Outcome::Win(Player::Two)));
        ai_vs_random(|s| Box::new(Simon::with_seed(s)), 3, Player::Two, 200);
    }
}
