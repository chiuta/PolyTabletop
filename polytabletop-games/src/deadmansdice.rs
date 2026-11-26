//! Dead Man's Dice (Marea Britanie) — joc de zaruri pentru doi „pirați”, după rezumatul public
//! din Wikipedia („Dead Man's Dice”), adaptat digital.
//!
//! Fiecare jucător are în rundă 2 zaruri obișnuite și zarul „Jolly Roger” (JR; fețe 1, 2, 3, 4, 5 și
//! craniul = 7). Zarurile aruncate se așază într-o *grămadă* proprie.
//! * Un zar aruncat se așază dacă grămada e goală, dacă are aceeași valoare ca zarul de sus, cu 1 mai mult,
//!   sau dacă împreună dau 7. Zarul JR se așază oricând. Dacă un zar nu se potrivește, tura se termină și zarul
//!   rămâne în mână.
//! * După ce ai aruncat în tura asta, poți **jefui** (`jefuiesc`) zarul de sus al adversarului dacă s-ar
//!   potrivi pe grămada ta (JR nu se jefuiește).
//! * Runda se termină când un jucător rămâne fără zaruri obișnuite. Fiecare numără suma grămezii;
//!   dacă JR e deasupra, suma se dublează; JR rămas în mână costă 7 puncte.
//! * Primul la 100 de puncte câștigă. (O rundă are cel mult 60 de mutări.)
//!
//! Simplificări față de rezumat: fără răsturnarea fizică a turnurilor și fără „fericirea” regulilor
//! detaliate ale producătorului (sursa nu le descrie complet). Mutări: `arunc`, `arunc-jr`, `jefuiesc`.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const TARGET: i32 = 100;
const JR_FACES: [u8; 6] = [1, 2, 3, 4, 5, 7];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mv {
    Roll,
    RollJr,
    Plunder,
}

#[derive(Clone)]
pub struct DeadMansDice {
    pts: [i32; 2],
    hand: [u8; 2],
    jr: [bool; 2],
    pile: [Vec<(u8, bool)>; 2],
    side: usize,
    rolled: bool,
    round_moves: u16,
    round: u16,
    rng: Rng,
    log: String,
}

impl Default for DeadMansDice {
    fn default() -> Self {
        Self::new()
    }
}

fn fits(top: Option<u8>, v: u8) -> bool {
    match top {
        None => true,
        Some(t) => v == t || v == t + 1 || v + t == 7,
    }
}

impl DeadMansDice {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        DeadMansDice {
            pts: [0; 2],
            hand: [2; 2],
            jr: [true; 2],
            pile: [Vec::new(), Vec::new()],
            side: 0,
            rolled: false,
            round_moves: 0,
            round: 1,
            rng: Rng::new(seed),
            log: String::new(),
        }
    }
    pub fn points(&self) -> (i32, i32) {
        (self.pts[0], self.pts[1])
    }
    fn top(&self, p: usize) -> Option<u8> {
        self.pile[p].last().map(|x| x.0)
    }
    fn pile_value(&self, p: usize) -> i32 {
        let s: i32 = self.pile[p].iter().map(|x| x.0 as i32).sum();
        if self.pile[p].last().is_some_and(|x| x.1) {
            2 * s
        } else {
            s
        }
    }
    fn round_score(&self, p: usize) -> i32 {
        self.pile_value(p) - if self.jr[p] { 7 } else { 0 }
    }
    fn end_round(&mut self) {
        for p in 0..2 {
            self.pts[p] += self.round_score(p);
        }
        self.log.push_str(&format!(
            " Sfârșit de rundă: J1 {} – J2 {}.",
            self.pts[0], self.pts[1]
        ));
        self.round += 1;
        self.hand = [2; 2];
        self.jr = [true; 2];
        self.pile = [Vec::new(), Vec::new()];
        self.side = ((self.round - 1) % 2) as usize;
        self.rolled = false;
        self.round_moves = 0;
    }
    fn fail(&mut self) {
        self.side = 1 - self.side;
        self.rolled = false;
    }
}

impl Game for DeadMansDice {
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
        let p = self.side;
        let mut v = Vec::new();
        if self.hand[p] > 0 {
            v.push(Mv::Roll);
        }
        if self.jr[p] {
            v.push(Mv::RollJr);
        }
        if self.rolled {
            if let Some((t, false)) = self.pile[1 - p].last().copied() {
                if fits(self.top(p), t) {
                    v.push(Mv::Plunder);
                }
            }
        }
        v
    }
    fn apply(&mut self, m: &Mv) {
        let p = self.side;
        self.round_moves += 1;
        match m {
            Mv::Roll | Mv::RollJr => {
                let jr = *m == Mv::RollJr;
                let v = if jr {
                    JR_FACES[self.rng.below(6)]
                } else {
                    self.rng.d6()
                };
                self.rolled = true;
                if jr || fits(self.top(p), v) {
                    self.pile[p].push((v, jr));
                    if jr {
                        self.jr[p] = false;
                    } else {
                        self.hand[p] -= 1;
                    }
                    self.log = format!(
                        "J{} aruncă {}{} și îl așază.",
                        p + 1,
                        v,
                        if jr { " (JR)" } else { "" }
                    );
                } else {
                    self.log = format!("J{} aruncă {v}: nu se potrivește, tura trece.", p + 1);
                    self.fail();
                }
            }
            Mv::Plunder => {
                let d = self.pile[1 - p].pop().unwrap();
                self.pile[p].push(d);
                self.log = format!("J{} jefuiește un {}.", p + 1, d.0);
            }
        }
        if self.hand[0] == 0 || self.hand[1] == 0 || self.round_moves >= 60 {
            self.end_round();
        }
    }
    fn outcome(&self) -> Option<Outcome> {
        if self.pts[0] >= TARGET || self.pts[1] >= TARGET {
            return Some(match self.pts[0].cmp(&self.pts[1]) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                _ => Outcome::Draw,
            });
        }
        None
    }
    fn evaluate(&self, pl: Player) -> i32 {
        let me = pl.idx();
        let v = 20 * (self.pts[me] - self.pts[1 - me])
            + 15 * (self.round_score(me) - self.round_score(1 - me));
        v + 3 * (self.hand[1 - me] as i32 - self.hand[me] as i32)
    }
    fn render(&self) -> String {
        let show = |p: usize| {
            let items: Vec<String> = self.pile[p]
                .iter()
                .map(|&(v, jr)| if jr { format!("{v}JR") } else { v.to_string() })
                .collect();
            format!("[{}]", items.join(" "))
        };
        format!(
            "Runda {}. Puncte: J1 {} – {} J2 (țintă {TARGET}).\nGrămada J1 {} (valoare {}), în mână: {} zaruri{}.\nGrămada J2 {} (valoare {}), în mână: {} zaruri{}.\n{}\nLa mutare: Jucătorul {}.",
            self.round,
            self.pts[0],
            self.pts[1],
            show(0),
            self.pile_value(0),
            self.hand[0],
            if self.jr[0] { " + JR" } else { "" },
            show(1),
            self.pile_value(1),
            self.hand[1],
            if self.jr[1] { " + JR" } else { "" },
            self.log.trim(),
            self.side + 1
        )
    }
    fn format_move(&self, m: &Mv) -> String {
        match m {
            Mv::Roll => "arunc",
            Mv::RollJr => "arunc-jr",
            Mv::Plunder => "jefuiesc",
        }
        .into()
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
    fn fit_rule() {
        assert!(fits(None, 3));
        assert!(fits(Some(3), 3));
        assert!(fits(Some(3), 4));
        assert!(fits(Some(3), 4));
        assert!(fits(Some(2), 5));
        assert!(!fits(Some(3), 5));
        assert!(!fits(Some(6), 4));
    }

    #[test]
    fn plunder_takes_the_top_die() {
        let mut g = DeadMansDice::with_seed(1);
        g.pile[1].push((4, false));
        g.pile[0].push((3, false));
        g.rolled = true;
        g.hand = [2, 2];
        assert!(GameObj::legal_moves(&g).contains(&"jefuiesc".to_string()));
        GameObj::play(&mut g, "jefuiesc").unwrap();
        assert_eq!(g.pile[1].len(), 0);
        assert_eq!(g.top(0), Some(4));
    }

    #[test]
    fn random_and_ai_games_finish() {
        random_finishes(|s| Box::new(DeadMansDice::with_seed(s)), 6, 4000);
        ai_vs_random(
            |s| Box::new(DeadMansDice::with_seed(s)),
            2,
            Player::One,
            4000,
        );
        ai_vs_random(
            |s| Box::new(DeadMansDice::with_seed(s)),
            3,
            Player::Two,
            4000,
        );
    }
}
