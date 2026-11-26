//! I Spy — „Ghicește ce văd” (Marea Britanie), joc pentru copii în doi, două runde cu roluri inversate.
//!
//! Scena e o cameră cu 36 de obiecte. Cine „vede” alege în secret un obiect (`vad masa`) și zice
//! „Văd ceva care începe cu litera M”. Celălalt ghicește (`este masa`, doar obiecte care încep cu acea literă)
//! sau cere un indiciu (`indiciu culoare`, `indiciu loc`, `indiciu marime`). Fiecare indiciu și fiecare ghicire
//! greșită contează ca o încercare; după 6 încercări fără răspuns runda e pierdută.
//! Cost pe rundă = încercări (inclusiv cea corectă), pierdere = 7; costul total mai mic câștigă (egal = remiză).
//!
//! Informație ascunsă: obiectul ales. Verdictul unei mutări apare când urmează mutarea următoare; AI-ul nu citește obiectul:
//! pentru indiciu alege trăsătura care desparte cel mai bine obiectele rămase.

use crate::q4_secret::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

/// (nume, culoare, loc, mărime)
pub const OBJECTS: [(&str, &str, &str, &str); 36] = [
    ("lampa", "galben", "sus", "mic"),
    ("tablou", "albastru", "sus", "mare"),
    ("ceas", "alb", "sus", "mic"),
    ("fereastra", "albastru", "sus", "mare"),
    ("perdea", "rosu", "sus", "mare"),
    ("raft", "maro", "sus", "mare"),
    ("oglinda", "gri", "sus", "mare"),
    ("carte", "rosu", "mijloc", "mic"),
    ("masa", "maro", "mijloc", "mare"),
    ("scaun", "maro", "mijloc", "mare"),
    ("canapea", "verde", "mijloc", "mare"),
    ("televizor", "negru", "mijloc", "mare"),
    ("planta", "verde", "mijloc", "mic"),
    ("vaza", "albastru", "mijloc", "mic"),
    ("calculator", "gri", "mijloc", "mic"),
    ("cana", "alb", "mijloc", "mic"),
    ("floare", "rosu", "mijloc", "mic"),
    ("minge", "galben", "jos", "mic"),
    ("masinuta", "rosu", "jos", "mic"),
    ("covor", "rosu", "jos", "mare"),
    ("pisica", "gri", "jos", "mic"),
    ("papuc", "verde", "jos", "mic"),
    ("cos", "maro", "jos", "mic"),
    ("jucarie", "portocaliu", "jos", "mic"),
    ("cutie", "alb", "jos", "mare"),
    ("ghiozdan", "albastru", "jos", "mic"),
    ("umbrela", "negru", "jos", "mare"),
    ("pian", "negru", "jos", "mare"),
    ("mar", "rosu", "mijloc", "mic"),
    ("pernuta", "verde", "mijloc", "mic"),
    ("cheie", "gri", "mijloc", "mic"),
    ("caiet", "albastru", "mijloc", "mic"),
    ("camasa", "alb", "sus", "mic"),
    ("manusa", "negru", "jos", "mic"),
    ("cana-mare", "verde", "mijloc", "mare"),
    ("pat", "albastru", "jos", "mare"),
];
const MAX_TRIES: u8 = 6;
const FAIL: i32 = 7;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mv {
    Pick(u8),
    Guess(u8),
    /// 0 culoare, 1 loc, 2 mărime
    Hint(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Pick,
    Guess,
    Over,
}

#[derive(Clone)]
pub struct ISpy {
    viewer: Option<Player>,
    round: u8,
    setter: usize,
    phase: Phase,
    secret: usize,
    hints: [Option<&'static str>; 3],
    wrong: Vec<u8>,
    used: u8,
    pending: Option<Mv>,
    cost: [i32; 2],
    noise: u64,
    log: String,
}

impl Default for ISpy {
    fn default() -> Self {
        Self::new()
    }
}

fn letter(i: usize) -> char {
    OBJECTS[i].0.chars().next().unwrap()
}

impl ISpy {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        ISpy {
            viewer: None,
            round: 1,
            setter: 0,
            phase: Phase::Pick,
            secret: 0,
            hints: [None; 3],
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
    fn trait_of(i: usize, h: u8) -> &'static str {
        match h {
            0 => OBJECTS[i].1,
            1 => OBJECTS[i].2,
            _ => OBJECTS[i].3,
        }
    }

    fn resolved(&self) -> ISpy {
        let mut r = self.clone();
        r.pending = None;
        let Some(m) = self.pending else { return r };
        let mut solved = false;
        match m {
            Mv::Hint(h) => {
                r.hints[h as usize] = Some(Self::trait_of(self.secret, h));
                r.used += 1;
                r.log = format!("Indiciu: {}.", Self::trait_of(self.secret, h));
            }
            Mv::Guess(i) => {
                r.used += 1;
                if i as usize == self.secret {
                    solved = true;
                } else {
                    r.wrong.push(i);
                    r.log = format!("«{}» nu e.", OBJECTS[i as usize].0);
                }
            }
            Mv::Pick(_) => {}
        }
        if solved || r.used >= MAX_TRIES {
            r.cost[r.guesser()] += if solved { r.used as i32 } else { FAIL };
            r.log = format!(
                "Runda {}: era «{}»{}.",
                r.round,
                OBJECTS[r.secret].0,
                if solved {
                    format!(", ghicit în {}", r.used)
                } else {
                    ", neghicit".to_string()
                }
            );
            r.round += 1;
            if r.round > 2 {
                r.phase = Phase::Over;
            } else {
                r.setter = 1 - r.setter;
                r.phase = Phase::Pick;
                r.hints = [None; 3];
                r.wrong.clear();
                r.used = 0;
            }
        }
        r
    }

    fn candidates(&self) -> Vec<u8> {
        let l = letter(self.secret);
        (0..OBJECTS.len() as u8)
            .filter(|&i| {
                letter(i as usize) == l
                    && !self.wrong.contains(&i)
                    && (0..3u8).all(|h| {
                        self.hints[h as usize].is_none_or(|v| Self::trait_of(i as usize, h) == v)
                    })
            })
            .collect()
    }
}

impl Game for ISpy {
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
            Phase::Pick => (0..OBJECTS.len() as u8).map(Mv::Pick).collect(),
            Phase::Guess => {
                let mut v: Vec<Mv> = (0..3u8)
                    .filter(|&h| r.hints[h as usize].is_none())
                    .map(Mv::Hint)
                    .collect();
                v.extend(r.candidates().into_iter().map(Mv::Guess));
                v
            }
        }
    }
    fn apply(&mut self, m: &Mv) {
        *self = self.resolved();
        match *m {
            Mv::Pick(i) => {
                self.secret = i as usize;
                self.phase = Phase::Guess;
                self.log = format!(
                    "Runda {}: J{} zice «Văd ceva care începe cu litera {}».",
                    self.round,
                    self.setter + 1,
                    letter(self.secret).to_ascii_uppercase()
                );
            }
            g => self.pending = Some(g),
        }
    }
    fn outcome(&self) -> Option<Outcome> {
        let r = self.resolved();
        (r.phase == Phase::Over).then(|| by_points(r.cost[1], r.cost[0]))
    }
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut v = 1000 * (self.cost[1 - me] - self.cost[me]);
        if let Some(m) = self.pending {
            let c = self.candidates();
            let n = c.len().max(1) as f32;
            let val = match m {
                Mv::Hint(h) => {
                    // cât de bine desparte trăsătura candidații: 1 − (cea mai mare grupă / n)
                    let mut best = 0;
                    for &i in &c {
                        let t = Self::trait_of(i as usize, h);
                        best = best.max(
                            c.iter()
                                .filter(|&&j| Self::trait_of(j as usize, h) == t)
                                .count(),
                        );
                    }
                    let split = 1.0 - best as f32 / n;
                    if c.len() > 2 {
                        split
                    } else {
                        split * 0.2
                    }
                }
                Mv::Guess(i) => {
                    if c.contains(&i) {
                        1.0 / n
                    } else {
                        0.0
                    }
                }
                Mv::Pick(_) => 0.0,
            };
            let x = (val * 400.0) as i32;
            v += if self.guesser() == me { x } else { -x };
        } else if self.phase == Phase::Guess && self.used == 0 && self.setter == me {
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
            let n: Vec<&str> = OBJECTS.iter().map(|o| o.0).collect();
            s.push_str(&format!("În cameră: {}\n", n.join(", ")));
        }
        if r.phase == Phase::Guess {
            s.push_str(&format!("Litera: {}. Încercări folosite: {}/{MAX_TRIES}. Culoare: {}, loc: {}, mărime: {}.\n", letter(r.secret).to_ascii_uppercase(), r.used, r.hints[0].unwrap_or("?"), r.hints[1].unwrap_or("?"), r.hints[2].unwrap_or("?")));
            s.push_str(&format!(
                "Posibile încă: {}\n",
                r.candidates()
                    .iter()
                    .map(|&i| OBJECTS[i as usize].0)
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            if view == r.setter {
                s.push_str(&format!(
                    "Secretul tău (J{}): {}\n",
                    view + 1,
                    OBJECTS[r.secret].0
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
            Mv::Pick(i) => format!("vad {}", OBJECTS[i as usize].0),
            Mv::Guess(i) => format!("este {}", OBJECTS[i as usize].0),
            Mv::Hint(h) => format!("indiciu {}", ["culoare", "loc", "marime"][h as usize]),
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
    fn object_names_are_unique() {
        let mut v: Vec<&str> = OBJECTS.iter().map(|o| o.0).collect();
        v.sort_unstable();
        v.dedup();
        assert_eq!(v.len(), OBJECTS.len());
    }

    #[test]
    fn letter_clue_limits_guesses_and_hint_is_revealed_later() {
        let mut g = ISpy::with_seed(1);
        GameObj::play(&mut g, "vad masa").unwrap();
        let moves = GameObj::legal_moves(&g);
        assert!(moves.contains(&"este masa".to_string()));
        assert!(!moves.contains(&"este lampa".to_string()));
        let mut guesser = g.clone_box();
        guesser.set_viewer(Some(Player::Two));
        assert!(!guesser.render().contains("Secretul tău"));
        GameObj::play(&mut g, "indiciu culoare").unwrap();
        assert!(GameObj::render(&g).contains("Culoare: maro"));
        GameObj::play(&mut g, "este masa").unwrap();
        assert_eq!(g.costs().1, 2);
    }

    #[test]
    fn random_and_ai_games_finish() {
        random_finishes(|s| Box::new(ISpy::with_seed(s)), 6, 300);
        ai_vs_random(|s| Box::new(ISpy::with_seed(s)), 2, Player::One, 300);
        ai_vs_random(|s| Box::new(ISpy::with_seed(s)), 3, Player::Two, 300);
    }
}
