//! Cupa Cunoașterii — întrebări cu 4 variante de răspuns și șase „felii” de colectat (variantă
//! originală; banca de 126 de întrebări din `cunoastere_bank.rs` este scrisă de noi).
//!
//! * Șase categorii: geografie, istorie, stiinta, natura, arta, general. Câștigă primul care are
//!   câte o felie din fiecare.
//! * La mutare alegi o categorie pentru care încă nu ai felia (`alege istorie`), apare o
//!   întrebare cu variantele a–d (ordinea e amestecată la fiecare întrebare), răspunzi
//!   (`raspund b`) și apoi mutarea forțată `verific` arată rezultatul. Răspuns corect: primești
//!   felia și continui; greșit: tura trece la celălalt.
//! * Fără tablă și fără zar (simplificare față de jocurile cu tablă): alegerea categoriei ține loc de mers.
//! * Întrebările folosite nu se repetă până se epuizează categoria, apoi se amestecă din nou.
//!
//! # AI și informația ascunsă
//! Varianta corectă și restul întrebărilor stau în starea jocului, dar `evaluate` nu le citește:
//! după `raspund x` rezultatul nu e încă rezolvat, iar evaluarea ignoră întrebarea. Prin urmare AI-ul
//! **nu are cunoștințe**: ghicește (aprox. 25% șanse) și își alege categoria la întâmplare.
//! Pentru un meci echilibrat jucați în doi oameni pe același ecran (`render()` arată întrebarea oricui
//! e la rând, dar nu și răspunsul corect) sau socotiți AI-ul un adversar slab.

use crate::cunoastere_bank::{BANK, CATEGORIES, CATEGORY_TITLES};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const ALL: u8 = 0b11_1111;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Choose(usize),
    Answer(usize),
    Verify,
}

#[derive(Clone)]
struct Asked {
    idx: usize,
    /// Ordinea variantelor afișate: `order[k]` = indicele din banca (0 = corect).
    order: [usize; 4],
    chosen: Option<usize>,
}

#[derive(Clone)]
pub struct Cunoastere {
    wedges: [u8; 2],
    side: Player,
    asked: Option<Asked>,
    /// Pentru fiecare categorie, coada întrebărilor încă nefolosite.
    queues: Vec<Vec<usize>>,
    rng: Rng,
    log: String,
    right: [u32; 2],
    wrong: [u32; 2],
}

impl Default for Cunoastere {
    fn default() -> Self {
        Self::new()
    }
}

impl Cunoastere {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Cunoastere {
            wedges: [0; 2],
            side: Player::One,
            asked: None,
            queues: vec![Vec::new(); 6],
            rng: Rng::new(seed),
            log: String::new(),
            right: [0; 2],
            wrong: [0; 2],
        };
        for c in 0..6 {
            g.refill(c);
        }
        g
    }

    fn refill(&mut self, cat: usize) {
        let mut v: Vec<usize> = BANK
            .iter()
            .enumerate()
            .filter(|(_, q)| q.0 as usize == cat)
            .map(|(i, _)| i)
            .collect();
        for i in (1..v.len()).rev() {
            let j = self.rng.below(i + 1);
            v.swap(i, j);
        }
        self.queues[cat] = v;
    }

    pub fn wedges(&self, p: Player) -> u8 {
        self.wedges[p.idx()]
    }

    pub fn answered(&self, p: Player) -> (u32, u32) {
        (self.right[p.idx()], self.wrong[p.idx()])
    }

    fn missing(&self, p: Player) -> Vec<usize> {
        (0..6)
            .filter(|&c| self.wedges[p.idx()] & (1 << c) == 0)
            .collect()
    }
}

impl Game for Cunoastere {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        match &self.asked {
            None => self
                .missing(self.side)
                .into_iter()
                .map(Move::Choose)
                .collect(),
            Some(a) if a.chosen.is_none() => (0..4).map(Move::Answer).collect(),
            Some(_) => vec![Move::Verify],
        }
    }

    fn apply(&mut self, m: &Move) {
        let i = self.side.idx();
        match *m {
            Move::Choose(c) => {
                if self.queues[c].is_empty() {
                    self.refill(c);
                }
                let idx = self.queues[c].pop().unwrap();
                let mut order = [0usize, 1, 2, 3];
                for k in (1..4).rev() {
                    let j = self.rng.below(k + 1);
                    order.swap(k, j);
                }
                self.asked = Some(Asked {
                    idx,
                    order,
                    chosen: None,
                });
            }
            Move::Answer(k) => {
                if let Some(a) = self.asked.as_mut() {
                    a.chosen = Some(k);
                }
            }
            Move::Verify => {
                let a = self.asked.take().unwrap();
                let q = &BANK[a.idx];
                let k = a.chosen.unwrap();
                let correct = a.order[k] == 0;
                if correct {
                    self.wedges[i] |= 1 << q.0;
                    self.right[i] += 1;
                    self.log = format!(
                        "J{}: «{}» — corect! Felia «{}» câștigată.",
                        i + 1,
                        q.2[0],
                        CATEGORY_TITLES[q.0 as usize]
                    );
                } else {
                    self.wrong[i] += 1;
                    self.log = format!(
                        "J{}: «{}» — greșit. Răspunsul corect era «{}».",
                        i + 1,
                        q.2[a.order[k]],
                        q.2[0]
                    );
                    self.side = self.side.other();
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.wedges[0] == ALL {
            Some(Outcome::Win(Player::One))
        } else if self.wedges[1] == ALL {
            Some(Outcome::Win(Player::Two))
        } else {
            None
        }
    }

    fn evaluate(&self, p: Player) -> i32 {
        // doar feliile (public); întrebarea în curs și rezultatul ei nu intră în calcul
        self.wedges[p.idx()].count_ones() as i32 - self.wedges[p.other().idx()].count_ones() as i32
    }

    fn render(&self) -> String {
        let mut s = String::from("Cupa Cunoașterii\n");
        for (k, name) in ["J1 (X)", "J2 (O)"].iter().enumerate() {
            let w: Vec<&str> = (0..6)
                .filter(|&c| self.wedges[k] & (1 << c) != 0)
                .map(|c| CATEGORY_TITLES[c])
                .collect();
            s.push_str(&format!(
                "{name}: {}/6 felii [{}]  (corecte {}, greșite {})\n",
                w.len(),
                w.join(", "),
                self.right[k],
                self.wrong[k]
            ));
        }
        if self.outcome().is_none() {
            s.push_str(&format!("La mutare: jucătorul {}\n", self.side.idx() + 1));
            match &self.asked {
                None => {
                    let m: Vec<&str> = self
                        .missing(self.side)
                        .iter()
                        .map(|&c| CATEGORIES[c])
                        .collect();
                    s.push_str(&format!("Alege o categorie: {}\n", m.join(", ")));
                }
                Some(a) => {
                    let q = &BANK[a.idx];
                    s.push_str(&format!("[{}] {}\n", CATEGORY_TITLES[q.0 as usize], q.1));
                    for k in 0..4 {
                        s.push_str(&format!(
                            "  {}) {}{}\n",
                            (b'a' + k as u8) as char,
                            q.2[a.order[k]],
                            if a.chosen == Some(k) {
                                "   ← ales"
                            } else {
                                ""
                            }
                        ));
                    }
                    if a.chosen.is_some() {
                        s.push_str("Scrie «verific».\n");
                    }
                }
            }
        }
        s.push_str(self.log.trim());
        s.push('\n');
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match *m {
            Move::Choose(c) => format!("alege {}", CATEGORIES[c]),
            Move::Answer(k) => format!("raspund {}", (b'a' + k as u8) as char),
            Move::Verify => "verific".to_string(),
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
    fn bank_is_large_balanced_and_well_formed() {
        assert!(BANK.len() >= 120);
        for c in 0..6u8 {
            assert!(
                BANK.iter().filter(|q| q.0 == c).count() >= 20,
                "categoria {c}"
            );
        }
        for (i, q) in BANK.iter().enumerate() {
            assert!(q.0 < 6);
            assert!(q.1.ends_with('?'), "{i}: {}", q.1);
            // variante distincte
            for a in 0..4 {
                assert!(!q.2[a].is_empty());
                for b in (a + 1)..4 {
                    assert_ne!(q.2[a], q.2[b], "întrebarea {i}");
                }
            }
        }
        // fără întrebări duplicate
        let mut qs: Vec<&str> = BANK.iter().map(|q| q.1).collect();
        qs.sort();
        let n = qs.len();
        qs.dedup();
        assert_eq!(qs.len(), n, "întrebări duplicate");
    }

    #[test]
    fn correct_answer_gives_wedge_and_keeps_turn() {
        let mut g = Cunoastere::with_seed(1);
        g.apply(&Move::Choose(1));
        let a = g.asked.clone().unwrap();
        let right = (0..4).find(|&k| a.order[k] == 0).unwrap();
        g.apply(&Move::Answer(right));
        assert_eq!(g.legal_moves(), vec![Move::Verify]);
        g.apply(&Move::Verify);
        assert_eq!(g.wedges[0], 1 << 1);
        assert_eq!(g.side, Player::One);
        // categoria cu felie nu mai e disponibilă
        assert!(!g.legal_moves().contains(&Move::Choose(1)));
        assert_eq!(g.legal_moves().len(), 5);
    }

    #[test]
    fn wrong_answer_passes_turn() {
        let mut g = Cunoastere::with_seed(2);
        g.apply(&Move::Choose(0));
        let a = g.asked.clone().unwrap();
        let wrong = (0..4).find(|&k| a.order[k] != 0).unwrap();
        g.apply(&Move::Answer(wrong));
        g.apply(&Move::Verify);
        assert_eq!(g.wedges[0], 0);
        assert_eq!(g.side, Player::Two);
        assert_eq!(g.wrong[0], 1);
    }

    #[test]
    fn six_wedges_win() {
        let mut g = Cunoastere::with_seed(3);
        g.wedges[0] = ALL & !(1 << 4);
        g.apply(&Move::Choose(4));
        let a = g.asked.clone().unwrap();
        let right = (0..4).find(|&k| a.order[k] == 0).unwrap();
        g.apply(&Move::Answer(right));
        g.apply(&Move::Verify);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
        assert!(g.legal_moves().is_empty());
    }

    #[test]
    fn no_question_repeats_before_the_category_is_exhausted() {
        let mut g = Cunoastere::with_seed(4);
        let mut seen = std::collections::HashSet::new();
        for _ in 0..21 {
            if g.queues[2].is_empty() {
                break;
            }
            let idx = g.queues[2].pop().unwrap();
            assert!(seen.insert(idx));
            assert_eq!(BANK[idx].0, 2);
        }
        assert_eq!(seen.len(), 21);
        // coada goală se reface la alegere
        g.apply(&Move::Choose(2));
        assert!(g.asked.is_some());
        assert_eq!(g.queues[2].len(), 20);
    }

    #[test]
    fn options_are_shuffled_across_games() {
        let mut positions = [0u32; 4];
        for seed in 1..=200u64 {
            let mut g = Cunoastere::with_seed(seed);
            g.apply(&Move::Choose(0));
            let a = g.asked.unwrap();
            positions[(0..4).find(|&k| a.order[k] == 0).unwrap()] += 1;
        }
        assert!(positions.iter().all(|&p| p > 20), "{positions:?}");
    }

    #[test]
    fn render_never_shows_the_right_answer_marker_before_verification() {
        let mut g = Cunoastere::with_seed(5);
        g.apply(&Move::Choose(0));
        let r = Game::render(&g);
        assert!(r.contains("a) ") && r.contains("d) "));
        assert!(!r.contains("corect!") && !r.contains("Răspunsul corect"));
    }

    #[test]
    fn evaluate_ignores_the_hidden_answer() {
        let mut a = Cunoastere::with_seed(7);
        a.apply(&Move::Choose(0));
        let mut vals = Vec::new();
        for k in 0..4 {
            let mut b = a.clone();
            b.apply(&Move::Answer(k));
            vals.push(b.evaluate(Player::One));
        }
        assert!(vals.iter().all(|&v| v == vals[0]));
    }

    #[test]
    fn random_games_terminate() {
        for seed in 1..=10u64 {
            let mut g = Cunoastere::with_seed(seed);
            let mut rng = Rng::new(seed * 3);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 3000, "seed {seed}");
            }
        }
    }
}
