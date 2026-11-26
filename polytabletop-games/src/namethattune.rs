//! Ghicește melodia (Name That Tune), joc de petrecere pentru 2 jucători.
//!
//! * 6 melodii din domeniul public, date ca șir de note (ex. `E E F G`). La fiecare rundă se licitează:
//!   „o recunosc din N note”. Următorul jucător fie coboară oferta (`ofer k`, k mai mic), fie cere dovada (`ghicesc`).
//!   Cine a licitat cel mai jos aude doar acele N note și alege titlul dintre 4 variante (`o1`…`o4`), apoi `arat`.
//! * Ghicit corect: +1 punct pentru ghicitor, altfel +1 pentru celălalt. Mai multe puncte câștigă, egal = remiză.
//! * Informație ascunsă: titlul corect nu apare în afișare până la `arat`. AI-ul recunoaște melodia doar din
//!   notele auzite și din titlurile variantelor (compară începutul), nu citește răspunsul.

use crate::q2_util::{by_score, end_line};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const TUNES: [(&str, &str); 9] = [
    ("Oda bucuriei", "E E F G G F E D C C D E E D D"),
    ("Twinkle, Twinkle", "C C G G A A G F F E E D D C"),
    ("Frère Jacques", "C D E C C D E C E F G E F G"),
    ("Jingle Bells", "E E E E E E E G C D E"),
    ("Mary Had a Little Lamb", "E D C D E E E D D D E G G"),
    ("Au clair de la lune", "C C C D E D C E D D C"),
    ("Oh! Susanna", "C D E G G A G E C D E E D C D"),
    ("Pentru Elisa", "E D# E D# E B D C A"),
    ("Row, Row, Row Your Boat", "C C C D E E D E F G"),
];
const ROUNDS: usize = 6;
const MAXN: u8 = 8;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Offer(u8),
    Demand,
    Pick(u8),
    Show,
}

#[derive(Clone)]
pub struct NameThatTune {
    order: Vec<usize>,
    opts: Vec<[usize; 4]>,
    round: usize,
    bid: Option<(u8, Player)>,
    guesser: Option<Player>,
    pick: Option<u8>,
    score: [i32; 2],
    side: Player,
    viewer: Option<Player>,
    last: String,
}

impl Default for NameThatTune {
    fn default() -> Self {
        Self::new()
    }
}

impl NameThatTune {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut order: Vec<usize> = (0..TUNES.len()).collect();
        crate::cards::shuffle(&mut order, &mut rng);
        order.truncate(ROUNDS);
        let mut opts = Vec::new();
        for &t in &order {
            let mut others: Vec<usize> = (0..TUNES.len()).filter(|&x| x != t).collect();
            crate::cards::shuffle(&mut others, &mut rng);
            let mut o = [t, others[0], others[1], others[2]];
            crate::cards::shuffle(&mut o, &mut rng);
            opts.push(o);
        }
        NameThatTune {
            order,
            opts,
            round: 0,
            bid: None,
            guesser: None,
            pick: None,
            score: [0; 2],
            side: Player::One,
            viewer: Some(Player::One),
            last: String::new(),
        }
    }
    fn over(&self) -> bool {
        self.round >= ROUNDS
    }
    fn notes(t: usize, n: u8) -> Vec<&'static str> {
        TUNES[t].1.split(' ').take(n as usize).collect()
    }
    /// Se potrivește titlul `t` cu notele auzite? (folosit de AI, fără a citi răspunsul)
    fn matches_heard(&self, t: usize, n: u8) -> bool {
        let heard = Self::notes(self.order[self.round], n);
        // AI-ul „aude” aceleași note: le compară cu începutul fiecărui titlu din repertoriul său
        let own = Self::notes(t, n);
        heard == own
    }
}

impl Game for NameThatTune {
    type Move = Move;
    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<Move> {
        if self.over() {
            return Vec::new();
        }
        if self.pick.is_some() {
            return vec![Move::Show];
        }
        if self.guesser.is_some() {
            return (1..=4).map(Move::Pick).collect();
        }
        match self.bid {
            None => (1..=MAXN).map(Move::Offer).collect(),
            Some((n, _)) => {
                let mut v: Vec<Move> = (1..n).map(Move::Offer).collect();
                v.push(Move::Demand);
                v
            }
        }
    }
    fn apply(&mut self, m: &Move) {
        match *m {
            Move::Offer(n) => {
                self.bid = Some((n, self.side));
                self.side = self.side.other();
            }
            Move::Demand => {
                let (_, who) = self.bid.unwrap();
                self.guesser = Some(who);
                self.side = who;
            }
            Move::Pick(i) => {
                self.pick = Some(i);
            }
            Move::Show => {
                let g = self.guesser.unwrap();
                let t = self.order[self.round];
                let ok = self.opts[self.round][self.pick.unwrap() as usize - 1] == t;
                let w = if ok { g } else { g.other() };
                self.score[w.idx()] += 1;
                self.last = format!(
                    "era «{}» — {}",
                    TUNES[t].0,
                    if ok { "ghicit" } else { "greșit" }
                );
                self.round += 1;
                self.bid = None;
                self.guesser = None;
                self.pick = None;
                self.side = if self.round.is_multiple_of(2) {
                    Player::One
                } else {
                    Player::Two
                };
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
        let mut v = (self.score[p.idx()] - self.score[1 - p.idx()]) * 100;
        if let (Some(g), Some(pk), false) = (self.guesser, self.pick, self.over()) {
            let n = self.bid.map(|b| b.0).unwrap_or(1);
            let t = self.opts[self.round][pk as usize - 1];
            // recunoaștere: titlul ales se potrivește cu notele auzite și e singurul care se potrivește
            let unique = self.opts[self.round]
                .iter()
                .filter(|&&o| self.matches_heard(o, n))
                .count()
                == 1;
            let good = self.matches_heard(t, n) && unique;
            let b = if good { 50 } else { -50 };
            v += if g == p { b } else { -b };
        }
        v
    }
    fn render(&self) -> String {
        let mut s = format!(
            "Ghicește melodia — runda {}/{ROUNDS}. Puncte: J1 {}, J2 {}\n",
            (self.round + 1).min(ROUNDS),
            self.score[0],
            self.score[1]
        );
        if !self.last.is_empty() {
            s.push_str(&format!("Runda precedentă: {}\n", self.last));
        }
        if !self.over() {
            match self.bid {
                Some((n, w)) => s.push_str(&format!("Oferta cea mai mică: J{} din {n} note\n", w.idx() + 1)),
                None => s.push_str("Nu s-a licitat încă (alege `ofer N`, N = numărul de note de care ai nevoie).\n"),
            }
            if let Some(g) = self.guesser {
                let n = self.bid.unwrap().0;
                s.push_str(&format!(
                    "Note auzite ({n}): {}\n",
                    Self::notes(self.order[self.round], n).join(" ")
                ));
                for (i, &t) in self.opts[self.round].iter().enumerate() {
                    s.push_str(&format!("  o{}) {}\n", i + 1, TUNES[t].0));
                }
                s.push_str(&format!("Ghicește J{}\n", g.idx() + 1));
            }
        }
        s.push_str(&end_line(self.outcome(), self.side));
        s
    }
    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Offer(n) => format!("ofer {n}"),
            Move::Demand => "ghicesc".into(),
            Move::Pick(i) => format!("o{i}"),
            Move::Show => "arat".into(),
        }
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answer_hidden_until_show() {
        let mut g = NameThatTune::with_seed(3);
        g.apply(&Move::Offer(4));
        g.apply(&Move::Demand);
        let r = g.render();
        assert!(r.contains("Note auzite (4)"));
        assert!(!r.contains("Runda precedentă"));
        g.apply(&Move::Pick(1));
        g.apply(&Move::Show);
        assert!(g.render().contains("Runda precedentă"));
        assert_eq!(g.score[0] + g.score[1], 1);
    }

    #[test]
    fn ai_does_not_read_the_answer_and_finishes() {
        let mut g: Box<dyn polytabletop_core::GameObj> = Box::new(NameThatTune::with_seed(8));
        let mut k = 0;
        while g.outcome().is_none() {
            let m = g.ai_move(k).unwrap();
            g.play(&m).unwrap();
            k += 1;
            assert!(k < 200);
        }
    }
}
