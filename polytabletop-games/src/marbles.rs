//! Bile (Marbles) — jocul de ghicit „par sau impar”, cunoscut din Antichitate (Grecia, Roma) până azi.
//!
//! * Fiecare jucător începe cu 10 bile; 12 runde, rolurile se schimbă în fiecare rundă.
//! * Cel care ține **ascunde** în pumn 1–5 bile (`ascund N`, N ≤ ce are); celălalt **ghicește**
//!   (`par` sau `impar`). Apoi cel care ține spune `arat` (dezvăluirea e o mutare separată).
//! * Ghicești bine: iei din stocul lui atâtea bile cât ascunsese; ghicești rău: îi dai tu atâtea
//!   (cel mult câte ai). Cine rămâne fără bile pierde; altfel, după 12 runde câștigă cine are mai multe.
//! * Informație ascunsă: numărul de bile din pumn se vede doar de cel care ține.
//!
//! AI-ul care ghicește nu citește numărul ascuns (rezolvarea e după `arat`); cel care ascunde alege la
//! întâmplare (după sămânță).

use crate::cards::Viewer;
use crate::q5_util::{by_score, verdict};
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const ROUNDS: u8 = 12;
const MAX_HIDE: i32 = 5;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Hide(u8),
    Guess(bool), // true = par
    Reveal,
}

#[derive(Clone)]
pub struct Marbles {
    stock: [i32; 2],
    holder: Player,
    hidden: Option<u8>,
    guess: Option<bool>,
    round: u8,
    last: String,
    viewer: Viewer,
}

impl Default for Marbles {
    fn default() -> Self {
        Self::new()
    }
}

impl Marbles {
    pub fn new() -> Self {
        Marbles {
            stock: [10; 2],
            holder: Player::One,
            hidden: None,
            guess: None,
            round: 0,
            last: String::new(),
            viewer: Viewer::Fixed(Player::One),
        }
    }

    pub fn stock(&self) -> [i32; 2] {
        self.stock
    }

    pub fn hidden_for_test(&self) -> Option<u8> {
        self.hidden
    }
}

impl Game for Marbles {
    type Move = Move;

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = match who {
            Some(p) => Viewer::Fixed(p),
            None => Viewer::HotSeat,
        };
    }

    fn to_move(&self) -> Player {
        match (self.hidden, self.guess) {
            (None, _) => self.holder,
            (Some(_), None) => self.holder.other(),
            (Some(_), Some(_)) => self.holder,
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        match (self.hidden, self.guess) {
            (None, _) => (1..=self.stock[self.holder.idx()].min(MAX_HIDE) as u8)
                .map(Move::Hide)
                .collect(),
            (Some(_), None) => vec![Move::Guess(true), Move::Guess(false)],
            (Some(_), Some(_)) => vec![Move::Reveal],
        }
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Hide(n) => self.hidden = Some(*n),
            Move::Guess(g) => self.guess = Some(*g),
            Move::Reveal => {
                let n = self.hidden.take().unwrap() as i32;
                let g = self.guess.take().unwrap();
                let h = self.holder.idx();
                let right = (n % 2 == 0) == g;
                if right {
                    self.stock[h] -= n;
                    self.stock[1 - h] += n;
                    self.last = format!(
                        "Rundă {}: J{} ținea {n} — ghicit ({}), J{} ia {n}.",
                        self.round + 1,
                        h + 1,
                        if g { "par" } else { "impar" },
                        2 - h
                    );
                } else {
                    let pay = n.min(self.stock[1 - h]);
                    self.stock[1 - h] -= pay;
                    self.stock[h] += pay;
                    self.last = format!(
                        "Rundă {}: J{} ținea {n} — ghicitul „{}” a fost greșit, J{} primește {pay}.",
                        self.round + 1,
                        h + 1,
                        if g { "par" } else { "impar" },
                        h + 1
                    );
                }
                self.round += 1;
                self.holder = self.holder.other();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.stock[0] <= 0 || self.stock[1] <= 0 || self.round >= ROUNDS {
            return Some(by_score(self.stock[0], self.stock[1]));
        }
        None
    }

    /// Doar stocurile; rezolvarea unei runde e în `Reveal`, deci evaluarea nu depinde de pumn.
    fn evaluate(&self, p: Player) -> i32 {
        (self.stock[p.idx()] - self.stock[p.other().idx()]) * 10
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Bile (par/impar) — runda {}/{ROUNDS}. Bile: J1 {}, J2 {}.\n",
            (self.round + 1).min(ROUNDS),
            self.stock[0],
            self.stock[1]
        );
        s.push_str(&format!(
            "Ține în pumn: Jucător {}.\n",
            self.holder.idx() + 1
        ));
        let v = self.viewer.who(self.to_move());
        if let (Some(n), g) = (self.hidden, self.guess) {
            if v == self.holder {
                s.push_str(&format!("Pumnul tău ascunde {n} bile.\n"));
            } else {
                s.push_str("Pumnul adversarului ascunde un număr de bile.\n");
            }
            if let Some(g) = g {
                s.push_str(&format!("Ghicit: {}.\n", if g { "par" } else { "impar" }));
            }
        }
        if !self.last.is_empty() {
            s.push_str(&format!("{}\n", self.last));
        }
        let fin = verdict(self.outcome());
        if fin.is_empty() {
            let what = match (self.hidden, self.guess) {
                (None, _) => "ascunde bilele (ascund N)",
                (Some(_), None) => "ghicește (par / impar)",
                (Some(_), Some(_)) => "arată pumnul (arat)",
            };
            s.push_str(&format!(
                "La mutare: Jucător {} — {what}\n",
                self.to_move().idx() + 1
            ));
        } else {
            s.push_str(&fin);
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Hide(n) => format!("ascund {n}"),
            Move::Guess(true) => "par".into(),
            Move::Guess(false) => "impar".into(),
            Move::Reveal => "arat".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q5_util::testkit::check_game;

    #[test]
    fn round_flow_and_transfer() {
        let mut g = Marbles::new();
        g.apply(&Move::Hide(3));
        assert_eq!(g.to_move(), Player::Two);
        g.apply(&Move::Guess(false)); // impar: corect
        assert_eq!(g.to_move(), Player::One);
        g.apply(&Move::Reveal);
        assert_eq!(g.stock(), [7, 13]);
        assert_eq!(g.holder, Player::Two);
    }

    #[test]
    fn the_hidden_count_is_only_shown_to_the_holder() {
        let mut g = Marbles::new();
        g.apply(&Move::Hide(4));
        let mut one = g.clone();
        one.set_viewer(Some(Player::One));
        let mut two = g.clone();
        two.set_viewer(Some(Player::Two));
        assert!(one.render().contains("ascunde 4 bile"));
        assert!(!two.render().contains("ascunde 4"));
        let mut hot = g.clone();
        hot.set_viewer(None);
        assert!(!hot.render().contains("ascunde 4"));
    }

    #[test]
    fn guesser_ai_does_not_see_the_hidden_number() {
        // aceeași stare vizibilă, pumn diferit => aceeași mutare AI (aceeași sămânță)
        let mut a = Marbles::new();
        let mut b = Marbles::new();
        a.apply(&Move::Hide(2));
        b.apply(&Move::Hide(5));
        for seed in 1..20 {
            assert_eq!(
                polytabletop_core::GameObj::ai_move(&a, seed),
                polytabletop_core::GameObj::ai_move(&b, seed)
            );
        }
    }

    #[test]
    fn random_and_ai_games() {
        check_game(|_| Marbles::new(), 5, 500);
    }
}
