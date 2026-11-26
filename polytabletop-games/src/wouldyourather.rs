//! Would You Rather (Ai prefera…), joc de petrecere pentru 2 jucători.
//!
//! * 6 dileme. La fiecare, fiecare jucător alege în secret două litere: prima = răspunsul lui (`a`/`b`),
//!   a doua = ce crede că a ales celălalt. Mutări: `aa`, `ab`, `ba`, `bb`, apoi `arat`.
//! * Fiecare presupunere corectă aduce 1 punct. Cel cu mai multe puncte câștigă, egal = remiză.
//! * Informație ascunsă: alegerea ta rămâne secretă până la `arat`; AI-ul nu o citește.

use crate::q2_util::{by_score, end_line};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const BANK: [(&str, &str); 14] = [
    ("să poți zbura", "să poți deveni invizibil"),
    ("să nu mai dormi niciodată", "să nu mai mănânci niciodată"),
    ("o vacanță la munte", "o vacanță la mare"),
    ("să știi toate limbile", "să cânți la toate instrumentele"),
    ("mereu prea cald", "mereu prea frig"),
    ("să trăiești în trecut", "să trăiești în viitor"),
    ("să vorbești cu animalele", "să citești gândurile"),
    ("cartofi prăjiți toată viața", "înghețată toată viața"),
    ("să fii foarte celebru", "să fii foarte bogat"),
    ("fără telefon un an", "fără mașină un an"),
    ("o zi lungă de 48 de ore", "o săptămână de 5 zile"),
    ("să schimbi trecutul", "să vezi viitorul"),
    ("un oraș mare", "un sat liniștit"),
    ("să fii mereu primul", "să fii mereu ultimul dar fericit"),
];
const ROUNDS: usize = 6;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Pick(bool, bool),
    Show,
}

#[derive(Clone)]
pub struct WouldYouRather {
    order: Vec<usize>,
    round: usize,
    picks: [Option<(bool, bool)>; 2],
    score: [i32; 2],
    viewer: Option<Player>,
    last: String,
}

impl Default for WouldYouRather {
    fn default() -> Self {
        Self::new()
    }
}

impl WouldYouRather {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut order: Vec<usize> = (0..BANK.len()).collect();
        crate::cards::shuffle(&mut order, &mut rng);
        order.truncate(ROUNDS);
        WouldYouRather {
            order,
            round: 0,
            picks: [None; 2],
            score: [0; 2],
            viewer: Some(Player::One),
            last: String::new(),
        }
    }
    fn over(&self) -> bool {
        self.round >= ROUNDS
    }
}

fn ab(b: bool) -> char {
    if b {
        'b'
    } else {
        'a'
    }
}

impl Game for WouldYouRather {
    type Move = Move;
    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }
    fn to_move(&self) -> Player {
        if self.picks[0].is_none() {
            Player::One
        } else if self.picks[1].is_none() {
            Player::Two
        } else {
            Player::One
        }
    }
    fn legal_moves(&self) -> Vec<Move> {
        if self.over() {
            return Vec::new();
        }
        if self.picks[0].is_some() && self.picks[1].is_some() {
            return vec![Move::Show];
        }
        let mut v = Vec::new();
        for x in [false, true] {
            for y in [false, true] {
                v.push(Move::Pick(x, y));
            }
        }
        v
    }
    fn apply(&mut self, m: &Move) {
        match *m {
            Move::Pick(x, y) => {
                let i = self.to_move().idx();
                self.picks[i] = Some((x, y));
            }
            Move::Show => {
                let (a, b) = (self.picks[0].unwrap(), self.picks[1].unwrap());
                let g0 = a.1 == b.0;
                let g1 = b.1 == a.0;
                self.score[0] += g0 as i32;
                self.score[1] += g1 as i32;
                self.last = format!(
                    "J1: {} (crede {}), J2: {} (crede {}) → J1 {}, J2 {}",
                    ab(a.0),
                    ab(a.1),
                    ab(b.0),
                    ab(b.1),
                    if g0 { "+1" } else { "0" },
                    if g1 { "+1" } else { "0" }
                );
                self.picks = [None; 2];
                self.round += 1;
            }
        }
    }
    fn outcome(&self) -> Option<Outcome> {
        if self.over() {
            Some(by_score(self.score[0], self.score[1]))
        } else {
            None
        }
    }
    fn evaluate(&self, p: Player) -> i32 {
        (self.score[p.idx()] - self.score[1 - p.idx()]) * 100
    }
    fn render(&self) -> String {
        let mut s = format!(
            "Ai prefera… — dilema {}/{ROUNDS}. Puncte: J1 {}, J2 {}\n",
            (self.round + 1).min(ROUNDS),
            self.score[0],
            self.score[1]
        );
        if !self.last.is_empty() {
            s.push_str(&format!("Runda precedentă: {}\n", self.last));
        }
        if !self.over() {
            let (a, b) = BANK[self.order[self.round]];
            s.push_str(&format!("a) {a}\nb) {b}\n"));
            let me = self.viewer.unwrap_or_else(|| self.to_move());
            match self.picks[me.idx()] {
                Some((x, y)) => s.push_str(&format!(
                    "Alegerea ta (J{}): {}{} (secret)\n",
                    me.idx() + 1,
                    ab(x),
                    ab(y)
                )),
                None => s.push_str(&format!("J{}: nu ai ales încă.\n", me.idx() + 1)),
            }
            s.push_str(&format!(
                "Adversarul a ales: {}\n",
                if self.picks[1 - me.idx()].is_some() {
                    "da (ascuns)"
                } else {
                    "nu încă"
                }
            ));
        }
        s.push_str(&end_line(self.outcome(), self.to_move()));
        s
    }
    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Pick(x, y) => format!("{}{}", ab(*x), ab(*y)),
            Move::Show => "arat".into(),
        }
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guessing_scores_and_secret_is_hidden() {
        let mut g = WouldYouRather::with_seed(2);
        g.apply(&Move::Pick(false, true));
        g.set_viewer(Some(Player::Two));
        assert!(!g.render().contains("Alegerea ta (J2): ab"));
        g.apply(&Move::Pick(true, false));
        g.apply(&Move::Show);
        assert_eq!(g.score, [1, 1]);
    }

    #[test]
    fn ai_ignores_secret_and_game_ends() {
        let mut a = WouldYouRather::with_seed(4);
        a.apply(&Move::Pick(false, false));
        let mut b = a.clone();
        b.picks[0] = Some((true, true));
        assert_eq!(
            polytabletop_core::GameObj::ai_move(&a, 3),
            polytabletop_core::GameObj::ai_move(&b, 3)
        );
        let mut g: Box<dyn polytabletop_core::GameObj> = Box::new(WouldYouRather::with_seed(5));
        let mut k = 0;
        while g.outcome().is_none() {
            let m = g.ai_move(k).unwrap();
            g.play(&m).unwrap();
            k += 1;
            assert!(k < 100);
        }
    }
}
