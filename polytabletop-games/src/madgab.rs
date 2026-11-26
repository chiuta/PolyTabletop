#![allow(clippy::doc_lazy_continuation)]
//! Mad Gab (SUA, joc de cuvinte fonetic) — variantă proprie în română, pentru doi jucători.
//!
//! * Se arată un șir de silabe fără sens („Pisi cadoar me pe pat”) care, citit repede cu voce tare,
//!   sună ca o expresie obișnuită. Jucătorul la rând alege (`ghici 1`…`ghici 4`) expresia corectă
//!   dintre 4 variante; poate cere întâi `ajutor` (se vede primul cuvânt; punctajul scade).
//! * Răspuns corect: 2 puncte (1 dacă a cerut ajutor). Răspuns greșit: celălalt jucător primește
//!   o singură șansă, dintre variantele rămase, pentru 1 punct. 8 runde, începe J1, rolul alternează.
//! * Cele 30 de ghicitori sunt scrise pentru acest joc. Robotul compară sunetele (literele, fără
//!   spații) ale șirului cu fiecare variantă; la nivelurile mici e amestecat cu zgomot. Nu citește
//!   indexul răspunsului corect.
//! Mutări: `ghici N`, `ajutor`, `verific`.

use crate::cards::shuffle;
use crate::w4_comun::fold;
use polytabletop_core::ai::{level, Level};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const ROUNDS: u8 = 8;

/// (șir fonetic, expresie corectă)
pub const PUZZLES: &[(&str, &str)] = &[
    ("Că rție dăbu cate", "Cartea de bucate"),
    ("Zăpa dape mun te", "Zăpadă pe munte"),
    ("Soa re lea ră să rit", "Soarele a răsărit"),
    ("Pisi cadoar me pe pat", "Pisica doarme pe pat"),
    ("Mer glapi a țămâ ine", "Merg la piață mâine"),
    ("A parece deiz vor", "Apă rece de izvor"),
    ("Lu nas tră lu ceș tea noap tea", "Luna strălucește noaptea"),
    ("Cân tăpă să ri leîn pă du re", "Cântă păsările în pădure"),
    ("Ploa iaba teîn geam", "Ploaia bate în geam"),
    (
        "Mă mi caseu ită la tele vi zor",
        "Mămica se uită la televizor",
    ),
    ("Copi iise joa căîn par c", "Copiii se joacă în parc"),
    ("Un tre nspre bu cu rești", "Un tren spre București"),
    ("Ma și nam er ge re pe de", "Mașina merge repede"),
    ("Bu nica fa ce plăcin tă", "Bunica face plăcintă"),
    ("Ia rna vi ne cuză pa dă", "Iarna vine cu zăpadă"),
    ("Mar ia ci te ște ocar te", "Maria citește o carte"),
    ("Să pta mâ na vi i toa re", "Săptămâna viitoare"),
    ("O să mer gem la mun te", "O să mergem la munte"),
    ("Cai ul gal o peaz ape câm p", "Calul galopează pe câmp"),
    ("Fra tele me u ci tește", "Fratele meu citește"),
    ("Fe ti țace un de sen", "Fetița face un desen"),
    ("Su pa de le gu me", "Supă de legume"),
    ("Cio co la ta cal dă", "Ciocolată caldă"),
    ("Pă sa rea zbo a ră", "Pasărea zboară"),
    ("Ur sul do ar me ia rna", "Ursul doarme iarna"),
    ("Ce as ul a ră tat o ra", "Ceasul a arătat ora"),
    ("Vân tul su fla pe de a l", "Vântul suflă pe deal"),
    ("Mă ru lu i cres cpe cră ci", "Mărul crește pe crăci"),
    ("Mă mi ca ga te ște ma sa", "Mămica gătește masa"),
    ("Pe ste le no ată în râ u", "Peștele înoată în râu"),
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Guess,
    Verify(u8),
    Steal,
    VerifySteal(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Guess(u8),
    Help,
    Verify,
}

#[derive(Clone)]
pub struct MadGab {
    order: Vec<usize>,
    round: u8,
    cands: [usize; 4],
    corr: usize,
    starter: Player,
    phase: Phase,
    helped: bool,
    wrong: Option<u8>,
    pts: [i32; 2],
    rng: Rng,
    log: String,
}

fn lev(a: &[char], b: &[char]) -> usize {
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    for i in 1..=a.len() {
        let mut cur = vec![i];
        for j in 1..=b.len() {
            let c = usize::from(a[i - 1] != b[j - 1]);
            cur.push((prev[j] + 1).min(cur[j - 1] + 1).min(prev[j - 1] + c));
        }
        prev = cur;
    }
    prev[b.len()]
}

fn letters(s: &str) -> Vec<char> {
    fold(s).chars().filter(|c| c.is_alphanumeric()).collect()
}

fn sim(garbled: &str, cand: &str) -> f64 {
    let (a, b) = (letters(garbled), letters(cand));
    1.0 - lev(&a, &b) as f64 / a.len().max(b.len()).max(1) as f64
}

impl Default for MadGab {
    fn default() -> Self {
        Self::new()
    }
}

impl MadGab {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut order: Vec<usize> = (0..PUZZLES.len()).collect();
        shuffle(&mut order, &mut rng);
        let mut g = MadGab {
            order,
            round: 0,
            cands: [0; 4],
            corr: 0,
            starter: Player::One,
            phase: Phase::Guess,
            helped: false,
            wrong: None,
            pts: [0; 2],
            rng,
            log: String::new(),
        };
        g.deal();
        g
    }

    fn cur(&self) -> usize {
        self.order[self.round as usize]
    }

    fn deal(&mut self) {
        let cur = self.cur();
        let mut pool: Vec<usize> = (0..PUZZLES.len()).filter(|&i| i != cur).collect();
        shuffle(&mut pool, &mut self.rng);
        let mut c = [cur, pool[0], pool[1], pool[2]];
        shuffle(&mut c, &mut self.rng);
        self.corr = c.iter().position(|&x| x == cur).unwrap();
        self.cands = c;
        self.helped = false;
        self.wrong = None;
        self.phase = Phase::Guess;
    }

    fn guesser(&self) -> Player {
        if self.round.is_multiple_of(2) {
            self.starter
        } else {
            self.starter.other()
        }
    }

    fn noise(&self, k: u8) -> f64 {
        let amp = match level() {
            Level::Easy => 1.2,
            Level::Normal => 0.5,
            Level::Hard => 0.25,
            Level::Expert => 0.08,
        };
        let h = polytabletop_core::hash_mix(((self.cur() as u64) << 8) | k as u64 | 0x9000);
        (h % 1000) as f64 / 1000.0 * amp
    }
}

impl Game for MadGab {
    type Move = Move;

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Guess => self.guesser(),
            Phase::Steal => self.guesser().other(),
            _ => Player::Two,
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.round >= ROUNDS {
            return Vec::new();
        }
        match self.phase {
            Phase::Guess => {
                let mut v: Vec<Move> = (0..4).map(Move::Guess).collect();
                if !self.helped {
                    v.push(Move::Help);
                }
                v
            }
            Phase::Steal => (0..4)
                .filter(|&k| Some(k) != self.wrong)
                .map(Move::Guess)
                .collect(),
            _ => vec![Move::Verify],
        }
    }

    fn apply(&mut self, m: &Move) {
        match (*m, self.phase) {
            (Move::Help, _) => self.helped = true,
            (Move::Guess(k), Phase::Guess) => self.phase = Phase::Verify(k),
            (Move::Guess(k), Phase::Steal) => self.phase = Phase::VerifySteal(k),
            (Move::Verify, Phase::Verify(k)) => {
                let g = self.guesser().idx();
                if k as usize == self.corr {
                    let gain = if self.helped { 1 } else { 2 };
                    self.pts[g] += gain;
                    self.log = format!(
                        "Corect: J{} +{gain}. Era «{}».",
                        g + 1,
                        PUZZLES[self.cur()].1
                    );
                    self.next_round();
                } else {
                    self.wrong = Some(k);
                    self.phase = Phase::Steal;
                    self.log = format!("Greșit (J{}). J{} poate încerca.", g + 1, 2 - g);
                }
            }
            (Move::Verify, Phase::VerifySteal(k)) => {
                let s = self.guesser().other().idx();
                if k as usize == self.corr {
                    self.pts[s] += 1;
                    self.log = format!("J{} ghicește: +1. Era «{}».", s + 1, PUZZLES[self.cur()].1);
                } else {
                    self.log = format!("Greșit și J{}. Era «{}».", s + 1, PUZZLES[self.cur()].1);
                }
                self.next_round();
            }
            _ => {}
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.round < ROUNDS {
            return None;
        }
        Some(match self.pts[0].cmp(&self.pts[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    /// Cinstit: robotul compară sunetele șirului cu textul fiecărei variante (plus zgomot după nivel);
    /// nu citește ce variantă e cea corectă.
    fn evaluate(&self, p: Player) -> i32 {
        let base = (self.pts[p.idx()] - self.pts[p.other().idx()]) * 1000;
        let g = PUZZLES[self.cur()].0;
        let chosen = match self.phase {
            Phase::Verify(k) | Phase::VerifySteal(k) => Some(k),
            _ => None,
        };
        match chosen {
            Some(k) => {
                let who = if matches!(self.phase, Phase::Verify(_)) {
                    self.guesser()
                } else {
                    self.guesser().other()
                };
                let v =
                    ((sim(g, PUZZLES[self.cands[k as usize]].1) + self.noise(k)) * 100.0) as i32;
                base + if who == p { v } else { -v }
            }
            _ => {
                if self.helped && self.phase == Phase::Guess {
                    base + 55 * if self.guesser() == p { 1 } else { -1 }
                } else {
                    base
                }
            }
        }
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Mad Gab — runda {}/{ROUNDS}. Scor: J1 {} – J2 {}.\n",
            (self.round + 1).min(ROUNDS),
            self.pts[0],
            self.pts[1]
        );
        if self.round < ROUNDS {
            s.push_str(&format!("Citește repede: «{}»\n", PUZZLES[self.cur()].0));
            if self.helped {
                let first = PUZZLES[self.cur()]
                    .1
                    .split_whitespace()
                    .next()
                    .unwrap_or("");
                s.push_str(&format!("Ajutor: începe cu «{first}».\n"));
            }
            for k in 0..4 {
                s.push_str(&format!("  {}) {}\n", k + 1, PUZZLES[self.cands[k]].1));
            }
            s.push_str(&format!("La mutare: J{}.\n", self.to_move().idx() + 1));
        }
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Guess(k) => format!("ghici {}", k + 1),
            Move::Help => "ajutor".into(),
            Move::Verify => "verific".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

impl MadGab {
    fn next_round(&mut self) {
        self.round += 1;
        if self.round < ROUNDS {
            self.deal();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzles_are_phonetically_close_to_their_answers() {
        assert!(PUZZLES.len() >= 30);
        for (g, a) in PUZZLES {
            assert!(sim(g, a) > 0.55, "{g} / {a}: {}", sim(g, a));
        }
    }

    #[test]
    fn right_wrong_and_steal_scoring() {
        let mut g = MadGab::with_seed(1);
        let c = g.corr as u8;
        g.apply(&Move::Guess(c));
        g.apply(&Move::Verify);
        assert_eq!(g.pts, [2, 0]);
        let c = g.corr as u8;
        let wrong = (c + 1) % 4;
        g.apply(&Move::Guess(wrong));
        g.apply(&Move::Verify);
        assert!(!g.legal_moves().contains(&Move::Guess(wrong)));
        g.apply(&Move::Guess(c));
        g.apply(&Move::Verify);
        assert_eq!(g.pts, [3, 0]);
    }

    #[test]
    fn random_games_end_and_ai_usually_solves() {
        for seed in 1..=10u64 {
            let mut g = MadGab::with_seed(seed);
            let mut rng = Rng::new(seed);
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
            }
        }
        let mut good = 0;
        for seed in 1..=20u64 {
            let g = MadGab::with_seed(seed);
            let m = polytabletop_core::GameObj::ai_move(&g, seed).unwrap();
            if m == format!("ghici {}", g.corr + 1) {
                good += 1;
            }
        }
        assert!(good >= 12, "{good}");
    }
}
