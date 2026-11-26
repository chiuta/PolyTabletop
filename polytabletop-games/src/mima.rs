//! Mima (Charades, Franța) — adaptare pentru doi jucători fără mimă fizică, două runde cu roluri inversate.
//!
//! Cine „mimează” alege în secret un cuvânt din lista jocului (`mimez delfin`) și apoi dă indicii (`gest categorie`,
//! `gest silabe`, `gest prima-litera`, `gest ultima-litera`, `gest lungime`), unul câte unul. După fiecare indiciu
//! celălalt ghicește (`ghicesc delfin`, doar cuvinte încă posibile) sau cere încă un indiciu (`mai-multe`).
//! Indiciile și ghicirile greșite contează în cost; la 7 fără răspuns runda e pierdută (cost 8).
//! Costul total mai mic după două runde câștigă (egal = remiză).
//!
//! Informație ascunsă: cuvântul ales (indiciile sunt publice). AI-ul nu citește cuvântul: pentru ghicit
//! folosește cuvintele rămase posibile, iar pentru indicii (ca mimator) alege un indiciu prin zgomot determinist.

use crate::q4_secret::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

/// (cuvânt, categorie, silabe)
pub const WORDS: [(&str, &str, u8); 32] = [
    ("elefant", "animal", 3),
    ("girafa", "animal", 3),
    ("pisica", "animal", 3),
    ("tigru", "animal", 2),
    ("maimuta", "animal", 3),
    ("delfin", "animal", 2),
    ("fluture", "animal", 3),
    ("vulpe", "animal", 2),
    ("dragon", "personaj", 2),
    ("printesa", "personaj", 3),
    ("pirat", "personaj", 2),
    ("vrajitor", "personaj", 3),
    ("cavaler", "personaj", 3),
    ("zana", "personaj", 2),
    ("medic", "profesie", 2),
    ("pompier", "profesie", 2),
    ("bucatar", "profesie", 3),
    ("pilot", "profesie", 2),
    ("profesor", "profesie", 3),
    ("pictor", "profesie", 2),
    ("fotbal", "sport", 2),
    ("inot", "sport", 2),
    ("tenis", "sport", 2),
    ("schi", "sport", 1),
    ("baschet", "sport", 2),
    ("box", "sport", 1),
    ("dansat", "activitate", 2),
    ("cantat", "activitate", 2),
    ("citit", "activitate", 2),
    ("gatit", "activitate", 2),
    ("alergat", "activitate", 3),
    ("desenat", "activitate", 3),
];
const HINTS: [&str; 5] = [
    "categorie",
    "silabe",
    "prima-litera",
    "ultima-litera",
    "lungime",
];
const MAX_COST: u8 = 7;
const FAIL: i32 = 8;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mv {
    Pick(u8),
    Hint(u8),
    Guess(u8),
    More,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Pick,
    Hint,
    Guess,
    Over,
}

fn hint_value(i: usize, h: u8) -> String {
    let w = WORDS[i];
    match h {
        0 => w.1.to_string(),
        1 => w.2.to_string(),
        2 => w.0[..1].to_string(),
        3 => w.0[w.0.len() - 1..].to_string(),
        _ => w.0.len().to_string(),
    }
}

#[derive(Clone)]
pub struct Mima {
    viewer: Option<Player>,
    round: u8,
    setter: usize,
    phase: Phase,
    secret: usize,
    shown: [Option<String>; 5],
    wrong: Vec<u8>,
    used: u8,
    pending: Option<u8>,
    cost: [i32; 2],
    noise: u64,
    log: String,
}

impl Default for Mima {
    fn default() -> Self {
        Self::new()
    }
}

impl Mima {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        Mima {
            viewer: None,
            round: 1,
            setter: 0,
            phase: Phase::Pick,
            secret: 0,
            shown: Default::default(),
            wrong: Vec::new(),
            used: 0,
            pending: None,
            cost: [0; 2],
            noise: rng.next_u64(),
            log: String::new(),
        }
    }
    pub fn costs(&self) -> (i32, i32) {
        let r = self.resolved();
        (r.cost[0], r.cost[1])
    }
    fn guesser(&self) -> usize {
        1 - self.setter
    }

    fn resolved(&self) -> Mima {
        let mut r = self.clone();
        r.pending = None;
        let Some(g) = self.pending else { return r };
        let solved = g as usize == self.secret;
        if !solved {
            r.used += 1;
            r.wrong.push(g);
            r.log = format!("«{}» nu e cuvântul.", WORDS[g as usize].0);
        }
        if solved || r.used >= MAX_COST {
            r.cost[r.guesser()] += if solved { r.used as i32 } else { FAIL };
            r.log = format!(
                "Runda {}: cuvântul era «{}»{}.",
                r.round,
                WORDS[r.secret].0,
                if solved { ", ghicit" } else { ", neghicit" }
            );
            r.round += 1;
            if r.round > 2 {
                r.phase = Phase::Over;
            } else {
                r.setter = 1 - r.setter;
                r.phase = Phase::Pick;
                r.shown = Default::default();
                r.wrong.clear();
                r.used = 0;
            }
        }
        r
    }

    fn candidates(&self) -> Vec<u8> {
        (0..WORDS.len() as u8)
            .filter(|&i| {
                !self.wrong.contains(&i)
                    && (0..5u8).all(|h| {
                        self.shown[h as usize]
                            .as_ref()
                            .is_none_or(|v| hint_value(i as usize, h) == *v)
                    })
            })
            .collect()
    }
}

impl Game for Mima {
    type Move = Mv;
    fn to_move(&self) -> Player {
        let r = self.resolved();
        pl(match r.phase {
            Phase::Pick | Phase::Hint => r.setter,
            _ => r.guesser(),
        })
    }
    fn legal_moves(&self) -> Vec<Mv> {
        let r = self.resolved();
        match r.phase {
            Phase::Over => Vec::new(),
            Phase::Pick => (0..WORDS.len() as u8).map(Mv::Pick).collect(),
            Phase::Hint => (0..5u8)
                .filter(|&h| r.shown[h as usize].is_none())
                .map(Mv::Hint)
                .collect(),
            Phase::Guess => {
                let mut v: Vec<Mv> = r.candidates().into_iter().map(Mv::Guess).collect();
                if r.shown.iter().any(|s| s.is_none()) {
                    v.push(Mv::More);
                }
                v
            }
        }
    }
    fn apply(&mut self, m: &Mv) {
        *self = self.resolved();
        match *m {
            Mv::Pick(i) => {
                self.secret = i as usize;
                self.phase = Phase::Hint;
                self.log = format!(
                    "Runda {}: J{} mimează un cuvânt.",
                    self.round,
                    self.setter + 1
                );
            }
            Mv::Hint(h) => {
                self.shown[h as usize] = Some(hint_value(self.secret, h));
                self.used += 1;
                self.phase = Phase::Guess;
                self.log = format!(
                    "Indiciu «{}»: {}.",
                    HINTS[h as usize],
                    hint_value(self.secret, h)
                );
            }
            Mv::More => self.phase = Phase::Hint,
            Mv::Guess(i) => self.pending = Some(i),
        }
    }
    fn outcome(&self) -> Option<Outcome> {
        let r = self.resolved();
        (r.phase == Phase::Over).then(|| by_points(r.cost[1], r.cost[0]))
    }
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut v = 1000 * (self.cost[1 - me] - self.cost[me]);
        let c = self.candidates();
        let n = c.len().max(1) as f32;
        let val = if let Some(g) = self.pending {
            if c.contains(&g) {
                1.0 / n
            } else {
                0.0
            }
        } else if self.phase == Phase::Hint && self.used > 0 {
            // a ales „mai multe”: merită când rămân multe posibilități
            if c.len() > 3 {
                0.45
            } else {
                0.1
            }
        } else {
            f32::NAN
        };
        if !val.is_nan() {
            let x = (val * 400.0) as i32;
            v += if self.guesser() == me { x } else { -x };
        } else if self.phase == Phase::Hint && self.used == 0 && self.setter == me {
            v += noise(self.noise, self.secret as u64, 50);
        }
        v
    }
    fn render(&self) -> String {
        let r = self.resolved();
        let view = viewer_of(self.viewer, self.to_move());
        let mut s = format!(
            "Runda {}/2. Cost total (mai mic = mai bine): J1 {} – {} J2.\n",
            r.round.min(2),
            r.cost[0],
            r.cost[1]
        );
        if r.phase == Phase::Pick {
            let n: Vec<&str> = WORDS.iter().map(|w| w.0).collect();
            s.push_str(&format!("Cuvinte: {}\n", n.join(", ")));
        }
        if matches!(r.phase, Phase::Hint | Phase::Guess) {
            let sh = |h: usize| r.shown[h].clone().unwrap_or_else(|| "?".into());
            s.push_str(&format!("Indicii: categorie {}, silabe {}, prima literă {}, ultima literă {}, lungime {}. Cost până acum: {}/{MAX_COST}.\n", sh(0), sh(1), sh(2), sh(3), sh(4), r.used));
            s.push_str(&format!(
                "Posibile: {}\n",
                r.candidates()
                    .iter()
                    .map(|&i| WORDS[i as usize].0)
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            if view == r.setter {
                s.push_str(&format!(
                    "Secretul tău (J{}): {}\n",
                    view + 1,
                    WORDS[r.secret].0
                ));
            }
        }
        s.push_str(r.log.trim());
        if r.phase != Phase::Over {
            s.push_str(&format!(
                "\nLa mutare: Jucătorul {}.",
                self.to_move().idx() + 1
            ));
        }
        s
    }
    fn format_move(&self, m: &Mv) -> String {
        match *m {
            Mv::Pick(i) => format!("mimez {}", WORDS[i as usize].0),
            Mv::Hint(h) => format!("gest {}", HINTS[h as usize]),
            Mv::Guess(i) => format!("ghicesc {}", WORDS[i as usize].0),
            Mv::More => "mai-multe".into(),
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
    fn hints_narrow_candidates() {
        let mut g = Mima::with_seed(1);
        GameObj::play(&mut g, "mimez delfin").unwrap();
        GameObj::play(&mut g, "gest categorie").unwrap();
        let m = GameObj::legal_moves(&g);
        assert!(m.contains(&"ghicesc delfin".to_string()));
        assert!(!m.contains(&"ghicesc medic".to_string()));
        GameObj::play(&mut g, "ghicesc delfin").unwrap();
        assert_eq!(g.costs().1, 1);
    }

    #[test]
    fn random_and_ai_games_finish() {
        random_finishes(|s| Box::new(Mima::with_seed(s)), 6, 400);
        ai_vs_random(|s| Box::new(Mima::with_seed(s)), 2, Player::One, 400);
        ai_vs_random(|s| Box::new(Mima::with_seed(s)), 3, Player::Two, 400);
    }
}
