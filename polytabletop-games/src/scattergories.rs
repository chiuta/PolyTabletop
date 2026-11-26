//! Scattergories (Categorii) — joc de cuvinte cu categorii și literă, pentru doi, adaptat la alegeri dintr-un dicționar.
//!
//! * 3 runde; în fiecare se trage o literă (a, b, c, m, p, s, t, în ordine amestecată) și se joacă 4 categorii:
//!   animal, mâncare, țară, obiect. Pentru fiecare categorie ambii jucători aleg **în secret** un cuvânt din
//!   lista categoriei care începe cu litera rundei (sau `pas`), apoi J1 spune `arat` (mutare separată).
//! * Puncte: cuvânt identic la amândoi = 0 pentru ambii; cuvânt unic = punctele lui (1 la cuvintele obișnuite,
//!   până la 3 la cele rare — lista e ordonată de la comun la rar); `pas` = 0.
//! * Câștigă cine are mai multe puncte după 12 categorii.
//! * AI-ul nu vede alegerea adversarului: estimează șansa de potrivire din popularitatea publică a
//!   cuvintelor (primele din listă sunt cele mai alese) și maximizează punctele așteptate.
//!
//! Mutări: cuvântul ales (de ex. `caine`), `pas`, `arat`.

use crate::cards::Viewer;
use crate::q5_util::{by_score, verdict};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const CATS: [&str; 4] = ["animal", "mancare", "tara", "obiect"];
const LETTERS: [char; 7] = ['a', 'b', 'c', 'm', 'p', 's', 't'];
/// WORDS[categorie][literă] = cuvinte de la comun la rar.
const WORDS: [[&[&str]; 7]; 4] = [
    [
        &["albina", "arici", "antilopa"],
        &["broasca", "bursuc", "bizon"],
        &["caine", "cal", "cocor"],
        &["maimuta", "mistret", "marmota"],
        &["pisica", "porc", "pinguin"],
        &["soarece", "sarpe", "salmon"],
        &["tigru", "taur", "tapir"],
    ],
    [
        &["ananas", "ardei", "aluat"],
        &["banana", "branza", "biscuit"],
        &["cartofi", "ciocolata", "cascaval"],
        &["mar", "mamaliga", "migdale"],
        &["paine", "pizza", "paste"],
        &["supa", "salata", "sarmale"],
        &["tort", "tocana", "tarta"],
    ],
    [
        &["anglia", "austria", "argentina"],
        &["belgia", "brazilia", "bulgaria"],
        &["canada", "china", "cehia"],
        &["mexic", "moldova", "maroc"],
        &["polonia", "portugalia", "peru"],
        &["spania", "suedia", "serbia"],
        &["turcia", "tunisia", "thailanda"],
    ],
    [
        &["afis", "agenda", "aspirator"],
        &["bilet", "balon", "bicicleta"],
        &["carte", "creion", "ceas"],
        &["masa", "mingea", "microfon"],
        &["pahar", "pix", "perna"],
        &["scaun", "sticla", "soba"],
        &["telefon", "televizor", "tigaie"],
    ],
];
const ROUNDS: usize = 3;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Word(u8),
    Skip,
    Reveal,
}

#[derive(Clone)]
pub struct Scattergories {
    letters: [usize; ROUNDS],
    step: usize, // 0..ROUNDS*4
    picks: [Option<Option<u8>>; 2],
    score: [i32; 2],
    viewer: Viewer,
    last: String,
}

impl Default for Scattergories {
    fn default() -> Self {
        Self::new()
    }
}

impl Scattergories {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut l: Vec<usize> = (0..LETTERS.len()).collect();
        for i in (1..l.len()).rev() {
            let j = rng.below(i + 1);
            l.swap(i, j);
        }
        Scattergories {
            letters: [l[0], l[1], l[2]],
            step: 0,
            picks: [None; 2],
            score: [0; 2],
            viewer: Viewer::Fixed(Player::One),
            last: String::new(),
        }
    }

    pub fn score(&self) -> [i32; 2] {
        self.score
    }

    fn cat(&self) -> usize {
        self.step % 4
    }

    fn letter(&self) -> usize {
        self.letters[(self.step / 4).min(ROUNDS - 1)]
    }

    fn list(&self) -> &'static [&'static str] {
        WORDS[self.cat()][self.letter()]
    }

    /// Punctele unui cuvânt unic: 1 la cele comune, 3 la cel mai rar.
    fn value(idx: usize) -> i32 {
        idx as i32 + 1
    }

    /// Șansa (în procente) ca adversarul să aleagă cuvântul `idx`: popularitate publică descrescătoare.
    fn popularity(idx: usize) -> i32 {
        [55, 30, 10][idx.min(2)]
    }
}

impl Game for Scattergories {
    type Move = Move;

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = match who {
            Some(p) => Viewer::Fixed(p),
            None => Viewer::HotSeat,
        };
    }

    fn to_move(&self) -> Player {
        match (self.picks[0], self.picks[1]) {
            (None, _) => Player::One,
            (Some(_), None) => Player::Two,
            _ => Player::One,
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        if self.picks[0].is_some() && self.picks[1].is_some() {
            return vec![Move::Reveal];
        }
        let mut v: Vec<Move> = (0..self.list().len() as u8).map(Move::Word).collect();
        v.push(Move::Skip);
        v
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Word(i) => {
                let k = self.to_move().idx();
                self.picks[k] = Some(Some(*i));
            }
            Move::Skip => {
                let k = self.to_move().idx();
                self.picks[k] = Some(None);
            }
            Move::Reveal => {
                let (a, b) = (self.picks[0].take().unwrap(), self.picks[1].take().unwrap());
                let list = self.list();
                let txt =
                    |x: Option<u8>| x.map_or("pas".to_string(), |i| list[i as usize].to_string());
                let mut note = format!("{}: J1 {} — J2 {}", CATS[self.cat()], txt(a), txt(b));
                if a != b {
                    if let Some(i) = a {
                        self.score[0] += Self::value(i as usize);
                    }
                    if let Some(i) = b {
                        self.score[1] += Self::value(i as usize);
                    }
                } else if a.is_some() {
                    note.push_str(" (identice: 0)");
                }
                self.last = note;
                self.step += 1;
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.step >= ROUNDS * 4 {
            return Some(by_score(self.score[0], self.score[1]));
        }
        None
    }

    /// Doar scorul public și popularitatea publică a cuvintelor; alegerea adversarului nu e citită.
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut v = (self.score[me] - self.score[1 - me]) * 1000;
        if self.step < ROUNDS * 4 {
            if let Some(Some(i)) = self.picks[me] {
                let i = i as usize;
                let nomatch = 100 - Self::popularity(i);
                v += Self::value(i) * nomatch * 10;
            }
        }
        v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Scattergories — categoria {}/{}. Puncte: J1 {}, J2 {}.\n",
            (self.step + 1).min(ROUNDS * 4),
            ROUNDS * 4,
            self.score[0],
            self.score[1]
        );
        if self.step < ROUNDS * 4 {
            s.push_str(&format!(
                "Litera «{}», categoria «{}». Cuvinte ({}): {}\n",
                LETTERS[self.letter()].to_ascii_uppercase(),
                CATS[self.cat()],
                "de la comun la rar",
                self.list().join(", ")
            ));
        }
        let v = self.viewer.who(self.to_move());
        for i in 0..2 {
            if let Some(c) = self.picks[i] {
                if v.idx() == i {
                    let t = c.map_or("pas".to_string(), |k| self.list()[k as usize].to_string());
                    s.push_str(&format!("Alegerea ta (J{}): {t} — secretă.\n", i + 1));
                } else {
                    s.push_str(&format!("J{} a ales (ascuns).\n", i + 1));
                }
            }
        }
        if !self.last.is_empty() {
            s.push_str(&format!("{}\n", self.last));
        }
        let f = verdict(self.outcome());
        if f.is_empty() {
            s.push_str(&format!(
                "La mutare: Jucător {}\n",
                self.to_move().idx() + 1
            ));
        } else {
            s.push_str(&f);
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Word(i) => self.list()[*i as usize].into(),
            Move::Skip => "pas".into(),
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
    fn every_list_starts_with_its_letter() {
        for (c, cat) in WORDS.iter().enumerate() {
            for (l, ch) in LETTERS.iter().enumerate() {
                assert_eq!(cat[l].len(), 3);
                assert!(
                    cat[l].iter().all(|w| w.starts_with(*ch) && w.is_ascii()),
                    "{c} {ch}"
                );
            }
        }
    }

    #[test]
    fn identical_words_cancel_and_unique_score() {
        let mut g = Scattergories::with_seed(1);
        g.apply(&Move::Word(0));
        g.apply(&Move::Word(0));
        g.apply(&Move::Reveal);
        assert_eq!(g.score(), [0, 0]);
        g.apply(&Move::Word(2));
        g.apply(&Move::Word(0));
        g.apply(&Move::Reveal);
        assert_eq!(g.score(), [3, 1]);
    }

    #[test]
    fn hidden_pick_and_blind_ai() {
        let mut a = Scattergories::with_seed(4);
        let mut b = a.clone();
        a.apply(&Move::Word(0));
        b.apply(&Move::Word(1));
        // AI = J2 nu citește alegerea lui J1
        for s in 1..8 {
            assert_eq!(
                polytabletop_core::ai::best_move(&a, s),
                polytabletop_core::ai::best_move(&b, s)
            );
        }
        let mut v = a.clone();
        v.set_viewer(Some(Player::Two));
        assert!(!v.render().contains("Alegerea ta"));
    }

    #[test]
    fn random_and_ai_games() {
        check_game(Scattergories::with_seed, 6, 400);
    }
}
