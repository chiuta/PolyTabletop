//! Zip Zap Zop (joc de numărat), 2 jucători.
//!
//! * Se numără pe rând de la 1 până la 30. În loc de multiplii de 3 se spune „zip”, în loc de multiplii de 5
//!   „zap”, în loc de multiplii de 15 „zop”. Cine spune un cuvânt greșit pierde pe loc.
//! * Fiecare jucător are 2 `pas`-uri pe joc: cu `pas` nu spui nimic, iar numărul rămâne adversarului.
//! * Câștigă cine spune corect „30” (zap, fiindcă 30 e multiplu de 15 → zop).
//!
//! Mutări: `<număr>`, `zip`, `zap`, `zop`, `pas`. Informație completă; AI-ul nu greșește niciodată cuvântul.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const LAST: u8 = 30;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Num,
    Zip,
    Zap,
    Zop,
    Pass,
}

#[derive(Clone)]
pub struct ZipZapZop {
    n: u8,
    passes: [u8; 2],
    side: Player,
    winner: Option<Player>,
}

impl Default for ZipZapZop {
    fn default() -> Self {
        Self::new()
    }
}

impl ZipZapZop {
    pub fn new() -> Self {
        ZipZapZop {
            n: 1,
            passes: [2, 2],
            side: Player::One,
            winner: None,
        }
    }
    fn right(&self) -> Move {
        match (self.n.is_multiple_of(3), self.n.is_multiple_of(5)) {
            (true, true) => Move::Zop,
            (true, false) => Move::Zip,
            (false, true) => Move::Zap,
            _ => Move::Num,
        }
    }
}

impl Game for ZipZapZop {
    type Move = Move;
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<Move> {
        if self.winner.is_some() {
            return Vec::new();
        }
        let mut v = vec![Move::Num, Move::Zip, Move::Zap, Move::Zop];
        if self.passes[self.side.idx()] > 0 {
            v.push(Move::Pass);
        }
        v
    }
    fn apply(&mut self, m: &Move) {
        match m {
            Move::Pass => {
                self.passes[self.side.idx()] -= 1;
            }
            w if *w == self.right() => {
                if self.n == LAST {
                    self.winner = Some(self.side);
                }
                self.n += 1;
            }
            _ => {
                self.winner = Some(self.side.other());
            }
        }
        self.side = self.side.other();
    }
    fn outcome(&self) -> Option<Outcome> {
        self.winner.map(Outcome::Win)
    }
    fn evaluate(&self, p: Player) -> i32 {
        // paritate: cine ajunge la 30 cu pași alternați
        let mut v = (self.passes[p.idx()] as i32 - self.passes[1 - p.idx()] as i32) * 5;
        let remaining = (LAST - self.n.min(LAST)) as i32 + 1;
        if (remaining % 2 == 1) == (self.side == p) {
            v += 20;
        } else {
            v -= 20;
        }
        v
    }
    fn render(&self) -> String {
        let mut s = format!(
            "Zip Zap Zop — numărul următor: {} (zip = ×3, zap = ×5, zop = ×15); ținta: spui corect {LAST}\n",
            self.n.min(LAST)
        );
        s.push_str(&format!(
            "Pas-uri rămase: J1 {}, J2 {}\n",
            self.passes[0], self.passes[1]
        ));
        match self.winner {
            Some(w) => s.push_str(&format!("Câștigă J{}.\n", w.idx() + 1)),
            None => s.push_str(&format!(
                "La mutare: J{} (spune: {})\n",
                self.side.idx() + 1,
                self.n
            )),
        }
        s
    }
    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Num => self.n.to_string(),
            Move::Zip => "zip".into(),
            Move::Zap => "zap".into(),
            Move::Zop => "zop".into(),
            Move::Pass => "pas".into(),
        }
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn right_words_and_wrong_word_loses() {
        let mut g = ZipZapZop::new();
        assert_eq!(g.right(), Move::Num);
        g.n = 15;
        assert_eq!(g.right(), Move::Zop);
        g.apply(&Move::Zip);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
    }

    #[test]
    fn ai_never_errs() {
        let mut g: Box<dyn polytabletop_core::GameObj> = Box::new(ZipZapZop::new());
        let mut k = 0;
        while g.outcome().is_none() {
            let m = g.ai_move(k).unwrap();
            g.play(&m).unwrap();
            k += 1;
            assert!(k < 80);
        }
        // nimeni nu a greșit: câștigă cine a spus 30 corect
        assert!(k <= 34);
    }
}
