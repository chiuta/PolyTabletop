//! Spânzurătoarea (Hangman, global) — doi jucători, două runde cu roluri inversate.
//!
//! Într-o rundă, cine „pune” alege în secret un cuvânt din lista jocului (`cuvant casa`); celălalt ghicește litere
//! (`litera a`) sau întreg cuvântul (`ghicesc casa`, doar dintre cuvintele încă posibile).
//! Fiecare greșeală (literă lipsă sau cuvânt greșit) adaugă o piesă la spânzurătoare; la 7 greșeli rundă pierdută.
//! Punctaj pentru cine ghicește: dacă reușește, 1 + (7 − greșeli), altfel 0. Câștigă cine adună mai multe puncte în
//! cele două runde (egal = remiză).
//!
//! Informație ascunsă: cuvântul ales. Verdictul unei ghiciri apare când urmează mutarea următoare; AI-ul nu citește
//! cuvântul: pentru literă folosește probabilitatea de apariție în cuvintele rămase posibile din listă.

use crate::q4_secret::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const WORDS: [&str; 60] = [
    "casa",
    "masa",
    "carte",
    "scoala",
    "pisica",
    "caine",
    "soare",
    "luna",
    "stea",
    "munte",
    "padure",
    "floare",
    "copac",
    "fluture",
    "elefant",
    "girafa",
    "pantof",
    "palarie",
    "ceasornic",
    "bicicleta",
    "avion",
    "vapor",
    "tren",
    "masina",
    "ciocolata",
    "inghetata",
    "portocala",
    "banana",
    "strugure",
    "cires",
    "telefon",
    "calculator",
    "fereastra",
    "usa",
    "gradina",
    "pod",
    "rau",
    "lac",
    "mare",
    "vant",
    "ploaie",
    "zapada",
    "iarna",
    "vara",
    "toamna",
    "primavara",
    "chitara",
    "pian",
    "vioara",
    "tobe",
    "muzica",
    "poveste",
    "dragon",
    "castel",
    "printesa",
    "cavaler",
    "comoara",
    "pirat",
    "balon",
    "papusa",
];
const MAX_WRONG: u8 = 7;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mv {
    Pick(u8),
    Letter(u8),
    Word(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Pick,
    Guess,
    Over,
}

#[derive(Clone)]
pub struct Hangman {
    viewer: Option<Player>,
    round: u8,
    setter: usize,
    phase: Phase,
    secret: usize,
    tried: u32,
    revealed: Vec<Option<u8>>,
    bad_words: Vec<u8>,
    wrong: u8,
    pending: Option<Mv>,
    pts: [i32; 2],
    noise: u64,
    log: String,
}

impl Default for Hangman {
    fn default() -> Self {
        Self::new()
    }
}

impl Hangman {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        Hangman {
            viewer: None,
            round: 1,
            setter: 0,
            phase: Phase::Pick,
            secret: 0,
            tried: 0,
            revealed: Vec::new(),
            bad_words: Vec::new(),
            wrong: 0,
            pending: None,
            pts: [0; 2],
            noise: rng.next_u64(),
            log: String::new(),
        }
    }
    pub fn points(&self) -> (i32, i32) {
        let r = self.resolved();
        (r.pts[0], r.pts[1])
    }

    fn resolved(&self) -> Hangman {
        let mut r = self.clone();
        r.pending = None;
        let Some(m) = self.pending else { return r };
        let w = WORDS[self.secret].as_bytes();
        let mut solved = false;
        match m {
            Mv::Letter(c) => {
                r.tried |= 1 << c;
                if w.contains(&(b'a' + c)) {
                    for (i, &ch) in w.iter().enumerate() {
                        if ch == b'a' + c {
                            r.revealed[i] = Some(ch);
                        }
                    }
                    r.log = format!("Litera «{}» e în cuvânt.", (b'a' + c) as char);
                } else {
                    r.wrong += 1;
                    r.log = format!("Litera «{}» nu e în cuvânt.", (b'a' + c) as char);
                }
                solved = r.revealed.iter().all(|x| x.is_some());
            }
            Mv::Word(i) => {
                if i as usize == self.secret {
                    solved = true;
                    for (k, &ch) in w.iter().enumerate() {
                        r.revealed[k] = Some(ch);
                    }
                } else {
                    r.wrong += 1;
                    r.bad_words.push(i);
                    r.log = format!("«{}» nu e cuvântul.", WORDS[i as usize]);
                }
            }
            Mv::Pick(_) => {}
        }
        if solved || r.wrong >= MAX_WRONG {
            let guesser = 1 - r.setter;
            if solved {
                r.pts[guesser] += 1 + (MAX_WRONG - r.wrong) as i32;
            }
            r.log = format!(
                "Runda {} s-a încheiat: cuvântul era «{}»{}.",
                r.round,
                WORDS[r.secret],
                if solved { ", ghicit" } else { ", neghicit" }
            );
            r.round += 1;
            if r.round > 2 {
                r.phase = Phase::Over;
            } else {
                r.setter = 1 - r.setter;
                r.phase = Phase::Pick;
                r.tried = 0;
                r.wrong = 0;
                r.bad_words.clear();
                r.revealed.clear();
            }
        }
        r
    }

    /// Cuvinte posibile în starea publică.
    fn candidates(&self) -> Vec<u8> {
        let mut v = Vec::new();
        for (i, w) in WORDS.iter().enumerate() {
            let b = w.as_bytes();
            if b.len() != self.revealed.len() || self.bad_words.contains(&(i as u8)) {
                continue;
            }
            let ok = (0..26u8).filter(|c| self.tried & (1 << c) != 0).all(|c| {
                let ch = b'a' + c;
                (0..b.len()).all(|k| (b[k] == ch) == (self.revealed[k] == Some(ch)))
            });
            if ok {
                v.push(i as u8);
            }
        }
        v
    }

    fn guesser(&self) -> usize {
        1 - self.setter
    }
}

impl Game for Hangman {
    type Move = Mv;
    fn to_move(&self) -> Player {
        let r = self.resolved();
        pl(if r.phase == Phase::Pick {
            r.setter
        } else {
            r.guesser()
        })
    }
    fn legal_moves(&self) -> Vec<Mv> {
        let r = self.resolved();
        match r.phase {
            Phase::Over => Vec::new(),
            Phase::Pick => (0..WORDS.len() as u8).map(Mv::Pick).collect(),
            Phase::Guess => {
                let mut v: Vec<Mv> = (0..26u8)
                    .filter(|c| r.tried & (1 << c) == 0)
                    .map(Mv::Letter)
                    .collect();
                v.extend(r.candidates().into_iter().map(Mv::Word));
                v
            }
        }
    }
    fn apply(&mut self, m: &Mv) {
        *self = self.resolved();
        match *m {
            Mv::Pick(i) => {
                self.secret = i as usize;
                self.revealed = vec![None; WORDS[self.secret].len()];
                self.phase = Phase::Guess;
                self.log = format!(
                    "Runda {}: J{} a ales cuvântul ({} litere).",
                    self.round,
                    self.setter + 1,
                    WORDS[self.secret].len()
                );
            }
            g => self.pending = Some(g),
        }
    }
    fn outcome(&self) -> Option<Outcome> {
        let r = self.resolved();
        (r.phase == Phase::Over).then(|| by_points(r.pts[0], r.pts[1]))
    }
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut v = 1000 * (self.pts[me] - self.pts[1 - me]);
        if let Some(m) = self.pending {
            let c = self.candidates();
            let n = c.len().max(1) as f32;
            let val = match m {
                Mv::Letter(l) => {
                    c.iter()
                        .filter(|&&i| WORDS[i as usize].as_bytes().contains(&(b'a' + l)))
                        .count() as f32
                        / n
                }
                Mv::Word(i) => {
                    if c.contains(&i) {
                        (1.0 / n) + if c.len() <= 2 { 0.02 } else { 0.0 }
                    } else {
                        0.0
                    }
                }
                Mv::Pick(_) => 0.0,
            };
            let x = (val * 400.0) as i32;
            v += if self.guesser() == me { x } else { -x };
        } else if self.phase == Phase::Guess
            && self.tried == 0
            && self.wrong == 0
            && self.setter == me
        {
            v += noise(self.noise, self.secret as u64, 50);
        }
        v
    }
    fn render(&self) -> String {
        let r = self.resolved();
        let view = viewer_of(self.viewer, self.to_move());
        let mut s = format!(
            "Runda {}/2. Puncte: J1 {} – {} J2.\n",
            r.round.min(2),
            r.pts[0],
            r.pts[1]
        );
        if r.phase == Phase::Guess {
            let pat: Vec<String> = r
                .revealed
                .iter()
                .map(|x| x.map_or("_".to_string(), |c| (c as char).to_string()))
                .collect();
            let tried: String = (0..26u8)
                .filter(|c| r.tried & (1 << c) != 0)
                .map(|c| (b'a' + c) as char)
                .collect();
            s.push_str(&format!(
                "Cuvânt: {}   Greșeli: {}/{MAX_WRONG}   Încercate: {}\n",
                pat.join(" "),
                r.wrong,
                tried
            ));
            if view == r.setter {
                s.push_str(&format!(
                    "Secretul tău (J{}): {}\n",
                    view + 1,
                    WORDS[r.secret]
                ));
            }
        }
        s.push_str(r.log.trim());
        if r.phase != Phase::Over {
            s.push_str(&format!(
                "\nLa mutare: Jucătorul {}{}.",
                self.to_move().idx() + 1,
                if r.phase == Phase::Pick {
                    " (alege cuvântul)"
                } else {
                    ""
                }
            ));
        }
        s
    }
    fn format_move(&self, m: &Mv) -> String {
        match *m {
            Mv::Pick(i) => format!("cuvant {}", WORDS[i as usize]),
            Mv::Letter(c) => format!("litera {}", (b'a' + c) as char),
            Mv::Word(i) => format!("ghicesc {}", WORDS[i as usize]),
        }
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q4_util::*;
    use polytabletop_core::{GameObj, Player};

    #[test]
    fn words_are_lowercase_ascii_and_unique() {
        for w in WORDS {
            assert!(w.bytes().all(|b| b.is_ascii_lowercase()), "{w}");
        }
        let mut v = WORDS.to_vec();
        v.sort();
        v.dedup();
        assert_eq!(v.len(), WORDS.len());
    }

    #[test]
    fn guessing_reveals_letters_and_secret_is_hidden_from_guesser() {
        let mut g = Hangman::with_seed(1);
        GameObj::play(&mut g, "cuvant casa").unwrap();
        let mut as_guesser = g.clone_box();
        as_guesser.set_viewer(Some(Player::Two));
        assert!(!as_guesser.render().contains("casa"));
        let mut as_setter = g.clone_box();
        as_setter.set_viewer(Some(Player::One));
        assert!(as_setter.render().contains("Secretul tău (J1): casa"));
        GameObj::play(&mut g, "litera a").unwrap();
        assert!(
            GameObj::render(&g).contains("_ a _ a"),
            "{}",
            GameObj::render(&g)
        );
        GameObj::play(&mut g, "ghicesc casa").unwrap();
        assert_eq!(g.points().1, 1 + 7);
        // runda 2: acum J2 alege
        assert_eq!(Game::to_move(&g), Player::Two);
    }

    #[test]
    fn random_and_ai_games_finish() {
        random_finishes(|s| Box::new(Hangman::with_seed(s)), 6, 400);
        let r = ai_vs_random(|s| Box::new(Hangman::with_seed(s)), 2, Player::One, 400);
        assert!(r.is_some());
        ai_vs_random(|s| Box::new(Hangman::with_seed(s)), 3, Player::Two, 400);
    }
}
