//! Never Have I Ever (Eu niciodată) — joc de petrecere, adaptat pentru doi, cu „degete” în loc de băutură.
//!
//! * Fiecare jucător are 5 degete ridicate și un profil ascuns: pentru fiecare dintre cele 16 afirmații se
//!   trage (la începutul jocului) dacă a făcut sau nu acel lucru, după o probabilitate publică înscrisă lângă ea.
//! * La rând, cel care vorbește alege o afirmație pe care **nu a făcut-o el** (le vede doar pe acestea) și spune
//!   „Eu niciodată…” (`am-nu-N`, N = numărul afirmației). Apoi `raspund` (mutare separată): celălalt răspunde
//!   sincer după profilul lui — dacă a făcut-o, coboară un deget. Afirmația se scoate din joc.
//! * Pierde cine rămâne primul fără degete; dacă afirmațiile se termină, câștigă cine are mai multe degete.
//! * AI-ul alege afirmația cu cea mai mare probabilitate publică de a fi fost făcută de adversar, dintre cele
//!   pe care nu le-a făcut el; nu citește profilul adversarului.
//!
//! Mutări: `eu-niciodata N` (cu N = numărul din listă, scris `eu-niciodata-N`) și `raspund`.

use crate::cards::Viewer;
use crate::q5_util::{by_score, verdict};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

/// (text, probabilitate % ca un om oarecare să fi făcut-o)
const STATEMENTS: [(&str, u8); 16] = [
    ("mancat pizza la micul dejun", 70),
    ("adormit la cinema", 65),
    ("uitat cheile in casa", 60),
    ("calatorit cu avionul", 55),
    ("cantat pe scena", 40),
    ("pierdut un telefon", 45),
    ("inotat noaptea", 50),
    ("sarit cu parasuta", 8),
    ("mancat insecte", 15),
    ("scris o poezie de dragoste", 35),
    ("vazut o aurora boreala", 10),
    ("fost la un concert in aer liber", 62),
    ("gatit pentru douazeci de oameni", 25),
    ("pierdut un tren", 48),
    ("cantat la karaoke", 52),
    ("dormit intr-un cort", 58),
];
const FINGERS: i32 = 5;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Say(u8),
    Answer,
}

#[derive(Clone)]
pub struct Nhie {
    done: [[bool; 16]; 2], // profil ascuns
    used: [bool; 16],
    fingers: [i32; 2],
    pending: Option<u8>,
    speaker: Player,
    viewer: Viewer,
    last: String,
}

impl Default for Nhie {
    fn default() -> Self {
        Self::new()
    }
}

impl Nhie {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut done = [[false; 16]; 2];
        for row in done.iter_mut() {
            for (i, (_, pct)) in STATEMENTS.iter().enumerate() {
                row[i] = (rng.below(100) as u8) < *pct;
            }
        }
        Nhie {
            done,
            used: [false; 16],
            fingers: [FINGERS; 2],
            pending: None,
            speaker: Player::One,
            viewer: Viewer::Fixed(Player::One),
            last: String::new(),
        }
    }

    pub fn fingers(&self) -> [i32; 2] {
        self.fingers
    }

    fn answerer(&self) -> Player {
        self.speaker.other()
    }
}

impl Game for Nhie {
    type Move = Move;

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = match who {
            Some(p) => Viewer::Fixed(p),
            None => Viewer::HotSeat,
        };
    }

    fn to_move(&self) -> Player {
        if self.pending.is_some() {
            self.answerer()
        } else {
            self.speaker
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        if self.pending.is_some() {
            return vec![Move::Answer];
        }
        let me = self.speaker.idx();
        let v: Vec<Move> = (0..16u8)
            .filter(|i| !self.used[*i as usize] && !self.done[me][*i as usize])
            .map(Move::Say)
            .collect();
        if v.is_empty() {
            // nicio afirmație „curată”: își poate spune oricare nefolosită (a mințit-o, pierde un deget)
            (0..16u8)
                .filter(|i| !self.used[*i as usize])
                .map(Move::Say)
                .collect()
        } else {
            v
        }
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Say(i) => {
                let me = self.speaker.idx();
                if self.done[me][*i as usize] {
                    // a recunoscut o minciună: pierde degetul chiar el
                    self.fingers[me] -= 1;
                }
                self.pending = Some(*i);
            }
            Move::Answer => {
                let i = self.pending.take().unwrap() as usize;
                let a = self.answerer().idx();
                self.used[i] = true;
                if self.done[a][i] {
                    self.fingers[a] -= 1;
                    self.last = format!(
                        "«Eu niciodată n-am {}» — J{} a făcut-o și coboară un deget.",
                        STATEMENTS[i].0,
                        a + 1
                    );
                } else {
                    self.last = format!(
                        "«Eu niciodată n-am {}» — J{} nu a făcut-o.",
                        STATEMENTS[i].0,
                        a + 1
                    );
                }
                self.speaker = self.speaker.other();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.fingers[0] <= 0 || self.fingers[1] <= 0 || self.used.iter().all(|u| *u) {
            return Some(by_score(self.fingers[0], self.fingers[1]));
        }
        None
    }

    /// Citește doar degetele, probabilitățile publice și propriul profil — nu profilul adversarului.
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut v = (self.fingers[me] - self.fingers[1 - me]) * 1000;
        if let Some(i) = self.pending {
            let e = STATEMENTS[i as usize].1 as i32 * 10;
            v += if self.speaker == p { e } else { -e };
        }
        v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Eu niciodată — degete: J1 {}, J2 {}. Rămase: {} afirmații.\n",
            self.fingers[0],
            self.fingers[1],
            self.used.iter().filter(|u| !**u).count()
        );
        let v = self.viewer.who(self.to_move());
        if self.pending.is_none() && self.outcome().is_none() && v == self.speaker {
            s.push_str("Poți spune (nu le-ai făcut tu): ");
            let list: Vec<String> = self
                .legal_moves()
                .iter()
                .map(|m| match m {
                    Move::Say(i) => {
                        format!("{} ({}%)", self.format_move(m), STATEMENTS[*i as usize].1)
                    }
                    Move::Answer => String::new(),
                })
                .collect();
            s.push_str(&list.join(", "));
            s.push('\n');
            for m in self.legal_moves() {
                if let Move::Say(i) = m {
                    s.push_str(&format!("  {} = {}\n", i + 1, STATEMENTS[i as usize].0));
                }
            }
        }
        if let Some(i) = self.pending {
            s.push_str(&format!(
                "«Eu niciodată n-am {}» — J{} răspunde.\n",
                STATEMENTS[i as usize].0,
                self.answerer().idx() + 1
            ));
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
            Move::Say(i) => format!("eu-niciodata-{}", i + 1),
            Move::Answer => "raspund".into(),
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
    fn speaker_only_sees_statements_they_have_not_done() {
        let g = Nhie::with_seed(3);
        for m in g.legal_moves() {
            if let Move::Say(i) = m {
                assert!(!g.done[0][i as usize]);
            }
        }
    }

    #[test]
    fn answer_costs_a_finger_when_true() {
        let mut g = Nhie::with_seed(1);
        g.done[0] = [false; 16];
        g.done[1] = [true; 16];
        g.apply(&Move::Say(0));
        g.apply(&Move::Answer);
        assert_eq!(g.fingers, [5, 4]);
        assert_eq!(g.speaker, Player::Two);
    }

    #[test]
    fn ai_picks_likely_statement_and_ignores_opponent_profile() {
        let mut a = Nhie::with_seed(9);
        a.speaker = Player::Two;
        a.done[1] = [false; 16];
        let mut b = a.clone();
        a.done[0] = [true; 16];
        b.done[0] = [false; 16];
        for s in 1..6 {
            let x = polytabletop_core::ai::best_move(&a, s);
            assert_eq!(x, polytabletop_core::ai::best_move(&b, s));
            assert_eq!(x, Some(Move::Say(0))); // cea mai probabilă: 70%
        }
    }

    #[test]
    fn render_hides_opponent_statements() {
        let mut g = Nhie::with_seed(2);
        g.set_viewer(Some(Player::Two));
        assert!(!g.render().contains("Poți spune")); // J1 vorbește; J2 nu vede lista lui J1
        g.set_viewer(Some(Player::One));
        assert!(g.render().contains("Poți spune"));
    }

    #[test]
    fn random_and_ai_games() {
        check_game(Nhie::with_seed, 6, 600);
    }
}
