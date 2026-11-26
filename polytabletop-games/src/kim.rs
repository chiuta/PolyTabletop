//! Jocul lui Kim (Kim's Game, Kipling: „Kim”) — joc de memorie, 5 runde.
//!
//! * Pe tavă sunt obiecte (6, 8, 10, 12, 14 în cele 5 runde). În faza „privești” toate obiectele se văd;
//!   cine e la rând spune `acopera`. Apoi un obiect este scos în secret (`scot`, mutare separată) și
//!   rămân vizibile doar celelalte — ghicitorii trebuie să-și amintească tava inițială.
//! * Alternativ, cei doi ghicesc ce obiect lipsește (numele obiectului): corect = 1 punct și runda se
//!   încheie; greșit = obiectul se elimină din variante și rândul trece la celălalt. `verific` e mutarea
//!   separată care arată dacă ghicirea e corectă.
//! * Rundele încep alternativ. Câștigă cine are mai multe puncte după 5 runde.
//! * AI-ul nu vede obiectul scos: deduce din tava inițială (informație publică) pe care o „ține minte”
//!   doar parțial — memorează doar primele 8 obiecte din tavă.
//!
//! Mutări: `acopera`, `scot`, `verific`, sau numele unui obiect (de ex. `cheie`).

use crate::q5_util::{by_score, verdict};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const POOL: [&str; 30] = [
    "cheie",
    "ceas",
    "lingura",
    "moneda",
    "creion",
    "nasture",
    "carte",
    "minge",
    "pahar",
    "cheita",
    "foarfeca",
    "cana",
    "pieptene",
    "lumanare",
    "periuta",
    "sapun",
    "mar",
    "pana",
    "inel",
    "cana2",
    "busola",
    "clestisor",
    "ghem",
    "figurina",
    "farfurie",
    "caiet",
    "ochelari",
    "umbrela",
    "telefon",
    "sticla",
];
const SIZES: [usize; 5] = [6, 8, 10, 12, 14];
/// Câte obiecte din tavă „ține minte” AI-ul.
const MEMORY: usize = 8;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Cover,
    Remove,
    Guess(u8),
    Verify,
}

#[derive(Clone)]
pub struct Kim {
    tray: Vec<u8>,      // ordinea inițială a obiectelor (public, văzută în faza „privești”)
    gone: Option<u8>,   // obiectul scos (ascuns)
    phase: u8,          // 0 privești, 1 scoatere, 2 ghicitori, 3 verificare
    ruled_out: Vec<u8>, // variante deja greșite în runda curentă
    pending: Option<u8>, // ghicirea în așteptarea verificării
    round: usize,
    score: [i32; 2],
    starter: Player,
    turn: Player,
    rng: Rng,
    last: String,
}

impl Default for Kim {
    fn default() -> Self {
        Self::new()
    }
}

fn name(i: u8) -> &'static str {
    POOL[i as usize].trim_end_matches('2')
}

impl Kim {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Kim {
            tray: Vec::new(),
            gone: None,
            phase: 0,
            ruled_out: Vec::new(),
            pending: None,
            round: 0,
            score: [0; 2],
            starter: Player::One,
            turn: Player::One,
            rng: Rng::new(seed),
            last: String::new(),
        };
        g.new_tray();
        g
    }

    fn new_tray(&mut self) {
        let mut ids: Vec<u8> = (0..POOL.len() as u8).collect();
        for i in (1..ids.len()).rev() {
            let j = self.rng.below(i + 1);
            ids.swap(i, j);
        }
        ids.truncate(SIZES[self.round]);
        self.tray = ids;
        self.gone = None;
        self.phase = 0;
        self.ruled_out.clear();
        self.pending = None;
        self.turn = self.starter;
    }

    /// Obiectele încă vizibile pe tavă după scoatere (publice).
    fn visible(&self) -> Vec<u8> {
        self.tray
            .iter()
            .copied()
            .filter(|t| Some(*t) != self.gone)
            .collect()
    }

    pub fn score(&self) -> [i32; 2] {
        self.score
    }
}

impl Game for Kim {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.turn
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        match self.phase {
            0 => vec![Move::Cover],
            1 => vec![Move::Remove],
            3 => vec![Move::Verify],
            _ => self
                .tray
                .iter()
                .filter(|t| !self.ruled_out.contains(t))
                .map(|t| Move::Guess(*t))
                .collect(),
        }
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Cover => {
                self.phase = 1;
                self.last.clear();
            }
            Move::Remove => {
                let k = self.rng.below(self.tray.len());
                self.gone = Some(self.tray[k]);
                self.phase = 2;
            }
            Move::Guess(t) => {
                self.pending = Some(*t);
                self.phase = 3;
            }
            Move::Verify => {
                let g = self.pending.take().unwrap();
                if Some(g) == self.gone {
                    self.score[self.turn.idx()] += 1;
                    self.last = format!("J{} a ghicit: lipsea «{}».", self.turn.idx() + 1, name(g));
                    self.round += 1;
                    self.starter = self.starter.other();
                    if self.round < SIZES.len() {
                        self.new_tray();
                    }
                } else {
                    self.last = format!(
                        "J{} a greșit: «{}» nu lipsește.",
                        self.turn.idx() + 1,
                        name(g)
                    );
                    self.ruled_out.push(g);
                    self.turn = self.turn.other();
                    self.phase = 2;
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.round >= SIZES.len() {
            return Some(by_score(self.score[0], self.score[1]));
        }
        None
    }

    /// Citește doar tava inițială, obiectele rămase și variantele eliminate — niciodată `gone`.
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut v = (self.score[me] - self.score[1 - me]) * 1000;
        if let Some(g) = self.pending {
            let vis = self.visible_public();
            let remembered: Vec<u8> = self.tray.iter().take(MEMORY).copied().collect();
            // candidați dedusi: obiecte ținute minte care nu mai sunt vizibile
            let missing_known: Vec<u8> = remembered
                .iter()
                .copied()
                .filter(|t| !vis.contains(t))
                .collect();
            let forgotten: Vec<u8> = self
                .tray
                .iter()
                .skip(MEMORY)
                .copied()
                .filter(|t| !self.ruled_out.contains(t))
                .collect();
            let chance = if !missing_known.is_empty() {
                if missing_known.contains(&g) {
                    1000
                } else {
                    0
                }
            } else if forgotten.contains(&g) {
                1000 / forgotten.len().max(1) as i32
            } else {
                0
            };
            v += if self.turn == p { chance } else { -chance };
        }
        v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Jocul lui Kim — runda {}/{}. Puncte: J1 {}, J2 {}.\n",
            (self.round + 1).min(SIZES.len()),
            SIZES.len(),
            self.score[0],
            self.score[1]
        );
        match self.phase {
            0 => {
                s.push_str("Pe tavă: ");
                s.push_str(
                    &self
                        .tray
                        .iter()
                        .map(|t| name(*t))
                        .collect::<Vec<_>>()
                        .join(", "),
                );
                s.push_str("\nMemorează, apoi spune «acopera».\n");
            }
            1 => s.push_str("Tava e acoperită; urmează «scot».\n"),
            _ => {
                s.push_str("A rămas pe tavă: ");
                s.push_str(
                    &self
                        .visible_public()
                        .iter()
                        .map(|t| name(*t))
                        .collect::<Vec<_>>()
                        .join(", "),
                );
                s.push_str("\nCe obiect lipsește? Ghicește după numele din tava inițială.\n");
                if !self.ruled_out.is_empty() {
                    s.push_str(&format!(
                        "Deja greșite: {}\n",
                        self.ruled_out
                            .iter()
                            .map(|t| name(*t))
                            .collect::<Vec<_>>()
                            .join(", ")
                    ));
                }
            }
        }
        if !self.last.is_empty() {
            s.push_str(&format!("{}\n", self.last));
        }
        let v = verdict(self.outcome());
        if v.is_empty() {
            s.push_str(&format!("La mutare: Jucător {}\n", self.turn.idx() + 1));
        } else {
            s.push_str(&v);
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Cover => "acopera".into(),
            Move::Remove => "scot".into(),
            Move::Guess(t) => POOL[*t as usize].into(),
            Move::Verify => "verific".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

impl Kim {
    /// Tava vizibilă fără a citi `gone` direct decât pentru fază: după scoatere, obiectele publice.
    fn visible_public(&self) -> Vec<u8> {
        if self.phase >= 2 {
            self.visible()
        } else {
            self.tray.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q5_util::testkit::check_game;

    #[test]
    fn names_are_unique_and_ascii() {
        let mut v: Vec<&str> = POOL.to_vec();
        v.sort();
        v.dedup();
        assert_eq!(v.len(), POOL.len());
        assert!(POOL.iter().all(|s| s.is_ascii()));
    }

    #[test]
    fn round_flow_and_scoring() {
        let mut g = Kim::with_seed(3);
        assert_eq!(g.tray.len(), 6);
        g.apply(&Move::Cover);
        g.apply(&Move::Remove);
        let gone = g.gone.unwrap();
        // ghicește corect
        g.apply(&Move::Guess(gone));
        g.apply(&Move::Verify);
        assert_eq!(g.score[0], 1);
        assert_eq!(g.round, 1);
        assert_eq!(g.tray.len(), 8);
        assert_eq!(g.turn, Player::Two); // începe celălalt
    }

    #[test]
    fn wrong_guess_passes_turn_and_is_ruled_out() {
        let mut g = Kim::with_seed(5);
        g.apply(&Move::Cover);
        g.apply(&Move::Remove);
        let gone = g.gone.unwrap();
        let wrong = *g.tray.iter().find(|t| **t != gone).unwrap();
        g.apply(&Move::Guess(wrong));
        g.apply(&Move::Verify);
        assert_eq!(g.to_move(), Player::Two);
        assert!(!g.legal_moves().contains(&Move::Guess(wrong)));
    }

    #[test]
    fn ai_does_not_peek_at_the_hidden_item() {
        // două stări identice public, dar cu obiect scos diferit în zona „uitată”: aceeași decizie
        let mut a = Kim::with_seed(11);
        a.round = 4;
        a.new_tray();
        a.apply(&Move::Cover);
        let mut b = a.clone();
        a.gone = Some(a.tray[12]);
        b.gone = Some(b.tray[13]);
        a.phase = 2;
        b.phase = 2;
        for seed in 1..8 {
            // vizibilele diferă, dar ambele lipsuri sunt în zona uitată: AI ghicește din candidații uitați
            let (x, y) = (
                polytabletop_core::ai::best_move(&a, seed),
                polytabletop_core::ai::best_move(&b, seed),
            );
            assert!(matches!(x, Some(Move::Guess(t)) if a.tray[8..].contains(&t)));
            assert!(matches!(y, Some(Move::Guess(t)) if b.tray[8..].contains(&t)));
        }
    }

    #[test]
    fn render_never_names_the_missing_item() {
        let mut g = Kim::with_seed(7);
        g.apply(&Move::Cover);
        g.apply(&Move::Remove);
        let gone = name(g.gone.unwrap());
        let r = g.render();
        let after = r
            .split("A rămas pe tavă: ")
            .nth(1)
            .unwrap()
            .lines()
            .next()
            .unwrap()
            .to_string();
        assert!(!after.split(", ").any(|x| x == gone), "{r}");
    }

    #[test]
    fn random_and_ai_games() {
        check_game(Kim::with_seed, 6, 1500);
    }
}
