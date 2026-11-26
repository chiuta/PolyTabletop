//! Portretul Misterios — joc de deducție cu 24 de personaje inventate (variantă originală).
//!
//! * Fiecare jucător are un personaj secret, tras la sorți din cele 24. Pe rând, la mutare poți
//!   **întreba** o trăsătură (`intreb ochelari`) — adversarul răspunde corect, automat —
//!   sau **ghici** (`ghicesc Matei`): ghicit corect câștigi, greșit pierzi pe loc.
//! * Răspunsul este o mutare separată, forțată (`raspund da` / `raspund nu`) a celui întrebat, singura
//!   permisă; astfel AI-ul nu vede răspunsul înainte să pună întrebarea (evaluarea nu citește
//!   personajul secret al nimănui). Se pot pune doar întrebări care încă împart lista ta de candidați.
//! * `render()` arată personajul tău secret (J1), candidații rămași pentru personajul adversarului și
//!   ultimele întrebări. Cealaltă parte (personajul AI) nu apare decât la final.
//! * AI: alege trăsătura care împarte cel mai echilibrat candidații lui; ghicește când rămâne unul.

#![allow(clippy::needless_range_loop)]

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const TRAITS: [&str; 14] = [
    "ochelari",
    "palarie",
    "barba",
    "mustata",
    "par-negru",
    "par-blond",
    "par-roscat",
    "par-alb",
    "barbat",
    "cercei",
    "pistrui",
    "chel",
    "esarfa",
    "zambeste",
];

const fn mask(gender: u8, hair: u8, flags: &str) -> u16 {
    let mut m: u16 = 0;
    if gender == b'M' {
        m |= 1 << 8;
    }
    match hair {
        b'N' => m |= 1 << 4,
        b'B' => m |= 1 << 5,
        b'R' => m |= 1 << 6,
        b'A' => m |= 1 << 7,
        _ => m |= 1 << 11, // chel
    }
    let f = flags.as_bytes();
    let mut i = 0;
    while i < f.len() {
        m |= match f[i] {
            b'o' => 1 << 0,
            b'p' => 1 << 1,
            b'b' => 1 << 2,
            b'm' => 1 << 3,
            b'e' => 1 << 9,
            b'f' => 1 << 10,
            b's' => 1 << 12,
            _ => 1 << 13, // z
        };
        i += 1;
    }
    m
}

/// (nume, mască de trăsături în ordinea `TRAITS`, bit 0 = ochelari). Personaje inventate.
pub const CHARS: [(&str, u16); 24] = [
    ("Ilinca", mask(b'F', b'N', "oz")),
    ("Matei", mask(b'M', b'B', "bm")),
    ("Sorana", mask(b'F', b'R', "ef")),
    ("Vlad", mask(b'M', b'N', "opb")),
    ("Dorina", mask(b'F', b'A', "sz")),
    ("Costin", mask(b'M', b'C', "m")),
    ("Anemona", mask(b'F', b'B', "eoz")),
    ("Teodor", mask(b'M', b'A', "bpo")),
    ("Maia", mask(b'F', b'N', "fs")),
    ("Radu", mask(b'M', b'R', "mz")),
    ("Zâna", mask(b'F', b'C', "e")),
    ("Bogdan", mask(b'M', b'B', "pz")),
    ("Lavinia", mask(b'F', b'R', "os")),
    ("Emil", mask(b'M', b'N', "m")),
    ("Octavia", mask(b'F', b'A', "ep")),
    ("Nicu", mask(b'M', b'C', "bo")),
    ("Florentina", mask(b'F', b'B', "fz")),
    ("Gelu", mask(b'M', b'R', "bp")),
    ("Daria", mask(b'F', b'N', "ps")),
    ("Silviu", mask(b'M', b'A', "mz")),
    ("Mirela", mask(b'F', b'R', "ez")),
    ("Petru", mask(b'M', b'N', "ofz")),
    ("Ruxandra", mask(b'F', b'A', "fo")),
    ("Tudor", mask(b'M', b'B', "eso")),
];

pub fn has_trait(ch: usize, t: usize) -> bool {
    CHARS[ch].1 & (1 << t) != 0
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Ask(usize),
    Guess(usize),
    Answer(bool),
    /// Verdictul forțat al celui ghicit (corect / greșit).
    Verdict(bool),
}

#[derive(Clone)]
pub struct Portret {
    secret: [usize; 2],
    /// Candidații lui `cand[p]` = personajele care mai pot fi secretul adversarului lui `p`.
    cand: [Vec<usize>; 2],
    side: Player,
    /// Întrebarea în așteptarea răspunsului (pusă de `side.other()`).
    pending: Option<usize>,
    /// Ghicirea în așteptarea verdictului (făcută de `side.other()`).
    pending_guess: Option<usize>,
    winner: Option<Player>,
    history: Vec<String>,
    /// `Some(p)` = render shows `p`'s secret character; `None` = hot-seat (the mover's).
    viewer: Option<Player>,
}

impl Default for Portret {
    fn default() -> Self {
        Self::new()
    }
}

impl Portret {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let a = rng.below(CHARS.len());
        let mut b = rng.below(CHARS.len());
        while b == a {
            b = rng.below(CHARS.len());
        }
        let all: Vec<usize> = (0..CHARS.len()).collect();
        Portret {
            secret: [a, b],
            cand: [all.clone(), all],
            side: Player::One,
            pending: None,
            pending_guess: None,
            winner: None,
            history: Vec::new(),
            viewer: Some(Player::One),
        }
    }

    pub fn secret(&self, p: Player) -> usize {
        self.secret[p.idx()]
    }

    pub fn candidates(&self, p: Player) -> &[usize] {
        &self.cand[p.idx()]
    }

    /// Dezechilibrul împărțirii candidaților lui `p` de către trăsătura `t` (0 = perfect).
    fn imbalance(&self, p: Player, t: usize) -> i32 {
        let c = &self.cand[p.idx()];
        let yes = c.iter().filter(|&&x| has_trait(x, t)).count() as i32;
        (2 * yes - c.len() as i32).abs()
    }
}

impl Game for Portret {
    type Move = Move;

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.winner.is_some() {
            return Vec::new();
        }
        if let Some(x) = self.pending_guess {
            return vec![Move::Verdict(self.secret[self.side.idx()] == x)];
        }
        if let Some(t) = self.pending {
            // `side` este cel întrebat; adevărul se calculează din secretul lui.
            return vec![Move::Answer(has_trait(self.secret[self.side.idx()], t))];
        }
        let c = &self.cand[self.side.idx()];
        let mut v = Vec::new();
        for t in 0..TRAITS.len() {
            let yes = c.iter().filter(|&&x| has_trait(x, t)).count();
            if yes > 0 && yes < c.len() {
                v.push(Move::Ask(t));
            }
        }
        for &x in c {
            v.push(Move::Guess(x));
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        let me = self.side;
        match m {
            Move::Ask(t) => {
                self.pending = Some(*t);
                self.history.push(format!(
                    "J{}: are personajul tău «{}»?",
                    me.idx() + 1,
                    TRAITS[*t]
                ));
                self.side = me.other();
            }
            Move::Answer(ans) => {
                let t = self.pending.take().unwrap();
                let asker = me.other();
                self.cand[asker.idx()].retain(|&x| has_trait(x, t) == *ans);
                self.history.push(format!(
                    "J{}: {}.",
                    me.idx() + 1,
                    if *ans { "da" } else { "nu" }
                ));
                // după răspuns, la mutare e cel care a întrebat? Nu: tura trece la cel întrebat.
                self.side = me;
            }
            Move::Guess(x) => {
                self.pending_guess = Some(*x);
                self.history
                    .push(format!("J{}: ghicește «{}».", me.idx() + 1, CHARS[*x].0));
                self.side = me.other();
            }
            Move::Verdict(right) => {
                let guesser = me.other();
                self.pending_guess = None;
                self.history.push(format!(
                    "Verdict: {}.",
                    if *right { "corect" } else { "greșit" }
                ));
                self.winner = Some(if *right { guesser } else { me });
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.winner.map(Outcome::Win)
    }

    fn evaluate(&self, p: Player) -> i32 {
        // doar informație publică: mărimea listelor de candidați și echilibrul întrebării în curs
        let mine = self.cand[p.idx()].len() as i32;
        let theirs = self.cand[p.other().idx()].len() as i32;
        let mut v = (theirs - mine) * 100;
        if let Some(x) = self.pending_guess {
            // J-ghicitorul nu știe încă verdictul: speranța din candidații lui
            let guesser = self.side.other();
            let c = self.cand[guesser.idx()].len() as i32;
            let known = self.cand[guesser.idx()].contains(&x) as i32;
            let q = known * 20000 / c.max(1) - 10000;
            return v + if guesser == p { q } else { -q };
        }
        if let Some(t) = self.pending {
            // `side` răspunde; întrebarea a fost pusă de `side.other()`
            let asker = self.side.other();
            let q = 50 - self.imbalance(asker, t) * 4;
            if asker == p {
                v += q;
            } else {
                v -= q;
            }
        }
        if self.pending.is_none() {
            let me = self.cand[self.side.idx()].len();
            if me == 1 {
                v += if self.side == p { 5000 } else { -5000 };
            }
        }
        v
    }

    fn render(&self) -> String {
        let mut s = String::from("Portretul Misterios\n");
        let v = self.viewer.unwrap_or(self.side);
        let (vi, oi) = (v.idx(), v.other().idx());
        s.push_str(&format!(
            "Personajul tău secret (J{}): {}\n",
            vi + 1,
            CHARS[self.secret[vi]].0
        ));
        if self.winner.is_some() {
            s.push_str(&format!(
                "Personajul secret al lui J{}: {}\n",
                oi + 1,
                CHARS[self.secret[oi]].0
            ));
        }
        let c: Vec<String> = self.cand[vi]
            .iter()
            .map(|&x| CHARS[x].0.to_string())
            .collect();
        s.push_str(&format!(
            "Candidați pentru personajul lui J{} ({}): {}\n",
            oi + 1,
            c.len(),
            c.join(", ")
        ));
        s.push_str(&format!(
            "J{} mai are {} candidați pentru al tău.\n",
            oi + 1,
            self.cand[oi].len()
        ));
        s.push_str("Trăsături: ");
        s.push_str(&TRAITS.join(", "));
        s.push('\n');
        if let Some(t) = self.pending {
            s.push_str(&format!(
                "Întrebare: «{}»? Răspunde jucătorul {} (scrie «raspund da» sau «raspund nu», corect).\n",
                TRAITS[t],
                self.side.idx() + 1
            ));
        } else if let Some(x) = self.pending_guess {
            s.push_str(&format!(
                "Ghicire: «{}». Jucătorul {} dă verdictul (scrie «verdict corect» sau «verdict gresit», adevărat).\n",
                CHARS[x].0,
                self.side.idx() + 1
            ));
        } else if self.winner.is_none() {
            s.push_str(&format!("La mutare: jucătorul {}\n", self.side.idx() + 1));
        }
        for l in self.history.iter().rev().take(6).rev() {
            s.push_str(l);
            s.push('\n');
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Ask(t) => format!("intreb {}", TRAITS[*t]),
            Move::Guess(x) => format!("ghicesc {}", ascii(CHARS[*x].0)),
            Move::Answer(a) => format!("raspund {}", if *a { "da" } else { "nu" }),
            Move::Verdict(r) => format!("verdict {}", if *r { "corect" } else { "gresit" }),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

fn ascii(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'â' => 'a',
            'ă' => 'a',
            'î' => 'i',
            'ș' => 's',
            'ț' => 't',
            'Â' => 'A',
            other => other,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_characters_are_distinguishable() {
        for i in 0..CHARS.len() {
            for j in (i + 1)..CHARS.len() {
                assert_ne!(CHARS[i].1, CHARS[j].1, "{} și {}", CHARS[i].0, CHARS[j].0);
            }
            assert!(CHARS[i].1 < (1 << TRAITS.len()));
        }
        let mut names: Vec<&str> = CHARS.iter().map(|c| c.0).collect();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), 24);
    }

    #[test]
    fn hair_colour_is_consistent() {
        // cel mult o culoare de păr (negru/blond/roșcat/alb); chel = fără păr colorat
        for (n, m) in CHARS {
            let hair = (m >> 4) & 0b1111;
            assert!(hair.count_ones() <= 1, "{n}");
            let bald = m & (1 << 11) != 0;
            assert!(!(bald && hair != 0), "{n} chel cu păr");
            assert!(bald || hair != 0, "{n} fără păr și nu e chel");
        }
    }

    #[test]
    fn traits_split_the_set_usefully() {
        for t in 0..TRAITS.len() {
            let yes = (0..CHARS.len()).filter(|&c| has_trait(c, t)).count();
            assert!((3..=21).contains(&yes), "{} {}", TRAITS[t], yes);
        }
    }

    #[test]
    fn question_answer_cycle_filters_candidates() {
        let mut g = Portret::with_seed(4);
        let t = 0; // ochelari
        g.apply(&Move::Ask(t));
        assert_eq!(g.side, Player::Two);
        let m = g.legal_moves();
        assert_eq!(m.len(), 1);
        let expect = has_trait(g.secret[1], t);
        assert_eq!(m[0], Move::Answer(expect));
        g.apply(&m[0]);
        assert!(g.cand[0].iter().all(|&x| has_trait(x, t) == expect));
        assert!(g.cand[0].contains(&g.secret[1]));
        // acum tura e a celui care a răspuns
        assert_eq!(g.side, Player::Two);
        assert!(g.pending.is_none());
    }

    #[test]
    fn guess_decides_the_game() {
        let mut g = Portret::with_seed(4);
        let right = g.secret[1];
        g.apply(&Move::Guess(right));
        assert!(g.outcome().is_none());
        assert_eq!(g.legal_moves(), vec![Move::Verdict(true)]);
        g.apply(&Move::Verdict(true));
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
        let mut g = Portret::with_seed(4);
        let wrong = (g.secret[1] + 1) % 24;
        g.apply(&Move::Guess(wrong));
        assert_eq!(g.legal_moves(), vec![Move::Verdict(false)]);
        g.apply(&Move::Verdict(false));
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
    }

    #[test]
    fn answers_are_always_truthful_so_secret_stays_a_candidate() {
        for seed in 1..=40u64 {
            let mut g = Portret::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                let pick = m[rng.below(m.len())].clone();
                g.apply(&pick);
                if g.outcome().is_none() {
                    assert!(g.cand[0].contains(&g.secret[1]));
                    assert!(g.cand[1].contains(&g.secret[0]));
                }
                n += 1;
                assert!(n < 500);
            }
        }
    }

    #[test]
    fn render_hides_opponent_secret_until_the_end() {
        let g = Portret::with_seed(3);
        let r = Game::render(&g);
        assert!(r.contains(CHARS[g.secret[0]].0));
        assert!(!r.contains("secret al lui J2"));
        let mut g = g;
        g.apply(&Move::Guess(0));
        let v = g.legal_moves()[0].clone();
        g.apply(&v);
        assert!(Game::render(&g).contains("secret al lui J2"));
    }

    #[test]
    fn ai_vs_ai_converges_and_wins_by_guessing() {
        for seed in 1..=15u64 {
            let mut g = Portret::with_seed(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = polytabletop_core::ai::best_move(&g, seed + n).unwrap();
                g.apply(&m);
                n += 1;
                assert!(n < 100, "seed {seed}");
            }
            // niciun AI nu ghicește greșit: câștigătorul e cel care a ghicit corect
            let last = g.history.last().unwrap();
            assert!(last.contains("corect"), "{last}");
        }
    }

    #[test]
    fn ai_asks_balanced_questions() {
        let g = Portret::with_seed(8);
        // J1 e la mutare cu toți candidații; cea mai bună trăsătură e cea mai echilibrată
        let best = polytabletop_core::ai::best_move(&g, 1).unwrap();
        if let Move::Ask(t) = best {
            let min = (0..TRAITS.len())
                .filter(|&t| {
                    let y = (0..24).filter(|&c| has_trait(c, t)).count();
                    y > 0 && y < 24
                })
                .map(|t| g.imbalance(Player::One, t))
                .min()
                .unwrap();
            assert_eq!(g.imbalance(Player::One, t), min);
        } else {
            panic!("AI-ul ar trebui să întrebe");
        }
    }
}

#[cfg(test)]
mod viewer_tests {
    use super::*;

    #[test]
    fn viewer_sees_only_own_secret() {
        let mut g = Portret::with_seed(6);
        let (a, b) = (CHARS[g.secret[0]].0, CHARS[g.secret[1]].0);
        Game::set_viewer(&mut g, Some(Player::One));
        let r = Game::render(&g);
        assert!(
            r.contains(&format!("Personajul tău secret (J1): {a}")),
            "{r}"
        );
        assert!(!r.contains("(J2):") && !r.contains("secret al lui"), "{r}");
        Game::set_viewer(&mut g, Some(Player::Two));
        let r = Game::render(&g);
        assert!(
            r.contains(&format!("Personajul tău secret (J2): {b}")),
            "{r}"
        );
        assert!(!r.contains("(J1):"), "{r}");
        Game::set_viewer(&mut g, None);
        assert!(Game::render(&g).contains("secret (J1)"));
    }
}
