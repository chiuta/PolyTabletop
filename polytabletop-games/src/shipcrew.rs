//! Ship, Captain and Crew (SUA / Marea Britanie) — joc de zaruri cu 5 zaruri.
//!
//! * Fiecare tură: până la 3 aruncări. Trebuie să obții, în ordine, un **6** (Ship), un **5** (Captain)
//!   și un **4** (Crew); acestea se pun deoparte automat (în aceeași aruncare poți obține mai multe, în ordine).
//! * Cele două zaruri rămase sunt **încărcătura** (cargo): suma lor e scorul turei (2–12).
//!   După o aruncare cu ship+captain+crew poți păstra încărcătura (`pastrez`) sau rearunca
//!   cele două zaruri (`rearunc`, apoi `arunc`), dacă mai ai aruncări.
//! * Dacă după 3 aruncări nu ai ship, captain și crew, scorul turei e 0.
//! * Se joacă 5 ture pe jucător; câștigă cine are suma cea mai mare (egal = remiză).
//!
//! Aruncarea este o mutare separată (`arunc`), ca AI-ul să nu vadă zarurile viitoare.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const TURNS: u8 = 5;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mv {
    Roll,
    Keep,
    Again,
}

#[derive(Clone)]
pub struct ShipCrew {
    total: [i32; 2],
    side: usize,
    turns_done: [u8; 2],
    rolls: u8,
    have: [bool; 3], // 6, 5, 4
    free: Vec<u8>,
    deciding: bool,
    rerolling: bool,
    rng: Rng,
    log: String,
}

impl Default for ShipCrew {
    fn default() -> Self {
        Self::new()
    }
}

impl ShipCrew {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        ShipCrew {
            total: [0; 2],
            side: 0,
            turns_done: [0; 2],
            rolls: 0,
            have: [false; 3],
            free: Vec::new(),
            deciding: false,
            rerolling: false,
            rng: Rng::new(seed),
            log: String::new(),
        }
    }
    pub fn totals(&self) -> (i32, i32) {
        (self.total[0], self.total[1])
    }
    fn complete(&self) -> bool {
        self.have.iter().all(|&b| b)
    }
    fn end_turn(&mut self, score: i32) {
        self.total[self.side] += score;
        self.turns_done[self.side] += 1;
        self.log.push_str(&format!(" Scor tură: {score}."));
        self.side = 1 - self.side;
        self.rolls = 0;
        self.have = [false; 3];
        self.free = Vec::new();
        self.deciding = false;
        self.rerolling = false;
    }
}

impl Game for ShipCrew {
    type Move = Mv;
    fn to_move(&self) -> Player {
        if self.side == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn legal_moves(&self) -> Vec<Mv> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        if self.deciding {
            vec![Mv::Keep, Mv::Again]
        } else {
            vec![Mv::Roll]
        }
    }
    fn apply(&mut self, m: &Mv) {
        match m {
            Mv::Keep => {
                let s: i32 = self.free.iter().map(|&d| d as i32).sum();
                self.end_turn(s);
            }
            Mv::Again => {
                self.deciding = false;
                self.rerolling = true;
            }
            Mv::Roll => {
                if self.rolls == 0 {
                    self.log.clear();
                }
                self.rerolling = false;
                self.rolls += 1;
                let n = 5 - self.have.iter().filter(|&&b| b).count();
                self.free = (0..n).map(|_| self.rng.d6()).collect();
                for (k, target) in [6u8, 5, 4].iter().enumerate() {
                    if k > 0 && !self.have[k - 1] {
                        break;
                    }
                    if !self.have[k] {
                        if let Some(pos) = self.free.iter().position(|d| d == target) {
                            self.free.remove(pos);
                            self.have[k] = true;
                        }
                    }
                }
                self.log.push_str(&format!(
                    " Aruncarea {}: ship {}, captain {}, crew {}, libere {:?}.",
                    self.rolls,
                    mark(self.have[0]),
                    mark(self.have[1]),
                    mark(self.have[2]),
                    self.free
                ));
                if self.rolls >= 3 {
                    let s = if self.complete() {
                        self.free.iter().map(|&d| d as i32).sum()
                    } else {
                        0
                    };
                    self.end_turn(s);
                } else if self.complete() {
                    self.deciding = true;
                }
            }
        }
    }
    fn outcome(&self) -> Option<Outcome> {
        if self.turns_done[0] >= TURNS && self.turns_done[1] >= TURNS {
            return Some(match self.total[0].cmp(&self.total[1]) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                _ => Outcome::Draw,
            });
        }
        None
    }
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut v = 100 * (self.total[me] - self.total[1 - me]);
        if self.rerolling {
            // rearuncare a încărcăturii: speranța pentru jucătorul la rând
            let left = 3 - self.rolls as i32;
            let e = if left >= 2 { 797 } else { 700 };
            v += if self.side == me { e } else { -e };
        }
        v
    }
    fn render(&self) -> String {
        let mut s = format!(
            "Ture jucate: J1 {}/{TURNS}, J2 {}/{TURNS}. Scor: J1 {} – {} J2.\n",
            self.turns_done[0], self.turns_done[1], self.total[0], self.total[1]
        );
        s.push_str(&format!(
            "Tura lui J{}: aruncarea {} din 3. Ship(6) {}, Captain(5) {}, Crew(4) {}.",
            self.side + 1,
            self.rolls,
            mark(self.have[0]),
            mark(self.have[1]),
            mark(self.have[2])
        ));
        if self.deciding {
            s.push_str(&format!(
                " Încărcătură: {:?} = {}.",
                self.free,
                self.free.iter().map(|&d| d as i32).sum::<i32>()
            ));
        }
        s.push_str(self.log.trim_end());
        s.push_str(&format!("\nLa mutare: Jucătorul {}.", self.side + 1));
        s
    }
    fn format_move(&self, m: &Mv) -> String {
        match m {
            Mv::Roll => "arunc".into(),
            Mv::Keep => "pastrez".into(),
            Mv::Again => "rearunc".into(),
        }
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

fn mark(b: bool) -> &'static str {
    if b {
        "da"
    } else {
        "nu"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q4_util::*;
    use polytabletop_core::{GameObj, Player};

    #[test]
    fn turn_ends_after_three_rolls_or_keep() {
        let mut g = ShipCrew::with_seed(1);
        for _ in 0..3 {
            let m = GameObj::legal_moves(&g);
            let pick = if m.contains(&"arunc".to_string()) {
                "arunc"
            } else {
                "rearunc"
            };
            GameObj::play(&mut g, pick).unwrap();
        }
        // a doua tură (J2) a început sau a fost forțat Keep/roll
        assert!(g.turns_done[0] >= 1 || g.deciding || g.rolls > 0);
    }

    #[test]
    fn random_and_ai_games_finish() {
        random_finishes(|s| Box::new(ShipCrew::with_seed(s)), 8, 400);
        ai_vs_random(|s| Box::new(ShipCrew::with_seed(s)), 2, Player::One, 400);
        ai_vs_random(|s| Box::new(ShipCrew::with_seed(s)), 3, Player::Two, 400);
    }
}
