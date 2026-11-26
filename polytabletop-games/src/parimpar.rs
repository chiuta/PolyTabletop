//! Pare sau impare (Odds and evens), joc global cu degetele, 2 jucători.
//!
//! * J1 e „impar”, J2 e „par”. La fiecare rundă fiecare alege în secret un număr de degete de la 1 la 5
//!   (`n1`…`n5`), apoi se arată (`arat`). Dacă suma e impară câștigă J1 runda, dacă e pară câștigă J2.
//! * Primul la 5 runde câștigate câștigă jocul.
//! * Informație ascunsă: alegerea ta rămâne secretă până la `arat`; AI-ul joacă un amestec uniform
//!   (strategia optimă) și nu citește alegerea ta.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const TARGET: u8 = 5;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Pick(u8),
    Show,
}

#[derive(Clone)]
pub struct ParImpar {
    picks: [Option<u8>; 2],
    wins: [u8; 2],
    rounds: u8,
    viewer: Option<Player>,
    /// alegerile (publice) ale ultimei runde
    last: Option<(u8, u8)>,
    rng: Rng,
}

impl Default for ParImpar {
    fn default() -> Self {
        Self::new()
    }
}

impl ParImpar {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        ParImpar {
            picks: [None; 2],
            wins: [0; 2],
            rounds: 0,
            viewer: Some(Player::One),
            last: None,
            rng: Rng::new(seed),
        }
    }
    pub fn wins(&self) -> (u8, u8) {
        (self.wins[0], self.wins[1])
    }
    fn over(&self) -> bool {
        self.wins.iter().any(|&w| w >= TARGET)
    }
}

impl Game for ParImpar {
    type Move = Move;

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }

    fn to_move(&self) -> Player {
        match (self.picks[0], self.picks[1]) {
            (None, _) => Player::One,
            (Some(_), None) => Player::Two,
            _ => Player::One,
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over() {
            return Vec::new();
        }
        if self.picks[0].is_some() && self.picks[1].is_some() {
            return vec![Move::Show];
        }
        (1..=5).map(Move::Pick).collect()
    }

    fn apply(&mut self, m: &Move) {
        match *m {
            Move::Pick(n) => {
                let i = self.to_move().idx();
                self.picks[i] = Some(n);
            }
            Move::Show => {
                let (a, b) = (self.picks[0].unwrap(), self.picks[1].unwrap());
                let _ = self.rng.next_u64();
                if (a + b) % 2 == 1 {
                    self.wins[0] += 1;
                } else {
                    self.wins[1] += 1;
                }
                self.last = Some((a, b));
                self.picks = [None; 2];
                self.rounds += 1;
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over() {
            return None;
        }
        Some(if self.wins[0] > self.wins[1] {
            Outcome::Win(Player::One)
        } else {
            Outcome::Win(Player::Two)
        })
    }

    /// Doar scorul public: alegerile ascunse nu se citesc.
    fn evaluate(&self, p: Player) -> i32 {
        (self.wins[p.idx()] as i32 - self.wins[1 - p.idx()] as i32) * 100
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Pare sau impare — J1 = impar, J2 = par. Runde câștigate: J1 {}, J2 {} (la {TARGET})\n",
            self.wins[0], self.wins[1]
        );
        if let Some((a, b)) = self.last {
            s.push_str(&format!(
                "Runda precedentă: J1 {a}, J2 {b}, suma {} ({})\n",
                a + b,
                if (a + b) % 2 == 1 { "impar" } else { "par" }
            ));
        }
        let me = self.viewer.unwrap_or_else(|| self.to_move());
        let mine = self.picks[me.idx()];
        match mine {
            Some(n) => s.push_str(&format!("Alegerea ta (J{}): {n} (secret)\n", me.idx() + 1)),
            None => s.push_str(&format!("J{}: nu ai ales încă.\n", me.idx() + 1)),
        }
        let o = self.picks[1 - me.idx()].is_some();
        s.push_str(&format!(
            "Adversarul a ales: {}\n",
            if o { "da (ascuns)" } else { "nu încă" }
        ));
        match self.outcome() {
            Some(Outcome::Win(p)) => s.push_str(&format!("Câștigă J{}.\n", p.idx() + 1)),
            _ => s.push_str(&format!("La mutare: J{}\n", self.to_move().idx() + 1)),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Pick(n) => format!("n{n}"),
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
    fn parity_decides_the_round() {
        let mut g = ParImpar::with_seed(1);
        g.apply(&Move::Pick(2));
        g.apply(&Move::Pick(3));
        assert_eq!(g.legal_moves(), vec![Move::Show]);
        g.apply(&Move::Show);
        assert_eq!(g.wins(), (1, 0));
    }

    #[test]
    fn hidden_pick_is_not_rendered_for_the_other_player() {
        let mut g = ParImpar::with_seed(1);
        g.apply(&Move::Pick(4));
        g.set_viewer(Some(Player::Two));
        let r = g.render();
        assert!(!r.contains("Alegerea ta (J2): 4"), "{r}");
        assert!(r.contains("ascuns"));
        g.set_viewer(Some(Player::One));
        assert!(g.render().contains("Alegerea ta (J1): 4"));
    }

    #[test]
    fn games_end_and_ai_ignores_secret() {
        let mut a = ParImpar::with_seed(1);
        a.apply(&Move::Pick(1));
        let mut b = a.clone();
        b.picks[0] = Some(2);
        assert_eq!(
            polytabletop_core::GameObj::ai_move(&a, 6),
            polytabletop_core::GameObj::ai_move(&b, 6)
        );
        let mut g: Box<dyn polytabletop_core::GameObj> = Box::new(ParImpar::with_seed(3));
        let mut n = 0;
        while g.outcome().is_none() {
            let m = g.ai_move(n).unwrap();
            g.play(&m).unwrap();
            n += 1;
            assert!(n < 100);
        }
    }
}
