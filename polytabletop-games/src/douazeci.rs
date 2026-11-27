//! Douăzeci de întrebări (Twenty Questions) — doi jucători, două runde cu roluri inversate.
//!
//! Cine „gândește” alege în secret un obiect din lista jocului (`obiect caine`). Celălalt pune întrebări da/nu din
//! lista numerotată (`intreb 7`; răspunsul vine automat din trăsăturile obiectului) sau încearcă un răspuns
//! (`este caine`). Întrebările și încercările greșite împreună nu pot depăși 20; la 20 fără răspuns runda e pierdută.
//! Cost pe rundă = întrebări + încercări (inclusiv cea corectă); pierdere = 21. Câștigă cine are costul total mai mic
//! după cele două runde (egal = remiză).
//!
//! Informație ascunsă: obiectul ales. Verdictul unei întrebări apare când urmează mutarea următoare; AI-ul nu citește
//! obiectul: alege întrebarea care împarte cel mai bine obiectele încă posibile și încearcă un răspuns când rămân puține.

use crate::q4_secret::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const QUESTIONS: [&str; 18] = [
    "E o ființă vie?",
    "E un animal?",
    "E o plantă sau un fruct?",
    "E făcut de om?",
    "Îl găsești de obicei în casă?",
    "E mai mare decât un om?",
    "Se mănâncă?",
    "Are roți?",
    "Folosește electricitate?",
    "E făcut în mare parte din metal?",
    "Zboară?",
    "Trăiește pe lângă apă sau se folosește pe apă?",
    "E rotund (în general)?",
    "Îl folosești aproape zilnic?",
    "Are blană sau păr?",
    "E mai mic decât o pâine?",
    "Are patru picioare?",
    "Face zgomot?",
];
/// (nume, trăsături: bitul i = răspunsul „da” la întrebarea i)
pub const ITEMS: [(&str, u32); 43] = [
    ("caine", 0x34013),
    ("pisica", 0x14013),
    ("cal", 0x34023),
    ("elefant", 0x30823),
    ("peste", 0x803),
    ("vrabie", 0x28403),
    ("vultur", 0x20403),
    ("rata", 0x20C03),
    ("vaca", 0x14023),
    ("albina", 0x8403),
    ("mar", 0x9045),
    ("banana", 0x8045),
    ("pepene", 0x1045),
    ("morcov", 0x45),
    ("copac", 0x25),
    ("floare", 0x8005),
    ("iepure", 0x1C003),
    ("broasca", 0x8803),
    ("porc", 0x14043),
    ("oaie", 0x34003),
    ("paine", 0x2058),
    ("ciocolata", 0x8058),
    ("televizor", 0x22118),
    ("frigider", 0x2318),
    ("telefon", 0x2A118),
    ("ceas", 0xB018),
    ("masina", 0x202A8),
    ("bicicleta", 0x2288),
    ("tren", 0x203A8),
    ("avion", 0x20628),
    ("barca", 0x828),
    ("vapor", 0x20A28),
    ("minge", 0x1008),
    ("scaun", 0x12018),
    ("pat", 0x12038),
    ("lampa", 0x118),
    ("spalat", 0x20318),
    ("cheie", 0xA208),
    ("pahar", 0xA018),
    ("carte", 0x2018),
    ("bec", 0x9118),
    ("soare", 0x1020),
    ("luna", 0x1000),
];

const MAX_USED: u8 = 20;
const FAIL_COST: i32 = 21;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mv {
    Pick(u8),
    Ask(u8),
    Guess(u8),
    /// Pas forțat după ultima mutare a jocului: abia acum se dezvăluie dacă ghicirea a fost corectă
    /// (altfel o încercare reușită ar încheia jocul și AI-ul ar vedea obiectul secret).
    Reveal,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Pick,
    Ask,
    Over,
}

#[derive(Clone)]
pub struct Douazeci {
    viewer: Option<Player>,
    round: u8,
    setter: usize,
    phase: Phase,
    secret: usize,
    answers: Vec<(u8, bool)>,
    eliminated: Vec<u8>,
    used: u8,
    pending: Option<Mv>,
    cost: [i32; 2],
    noise: u64,
    log: String,
}

impl Default for Douazeci {
    fn default() -> Self {
        Self::new()
    }
}

impl Douazeci {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        Douazeci {
            viewer: None,
            round: 1,
            setter: 0,
            phase: Phase::Pick,
            secret: 0,
            answers: Vec::new(),
            eliminated: Vec::new(),
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

    fn resolved(&self) -> Douazeci {
        let mut r = self.clone();
        r.pending = None;
        let Some(m) = self.pending else { return r };
        let mut solved = false;
        match m {
            Mv::Ask(q) => {
                let a = ITEMS[self.secret].1 >> q & 1 == 1;
                r.answers.push((q, a));
                r.used += 1;
                r.log = format!(
                    "«{}» — {}.",
                    QUESTIONS[q as usize],
                    if a { "da" } else { "nu" }
                );
            }
            Mv::Guess(i) => {
                if i as usize == self.secret {
                    solved = true;
                    r.used += 1;
                } else {
                    r.used += 1;
                    r.eliminated.push(i);
                    r.log = format!("«{}» nu e obiectul.", ITEMS[i as usize].0);
                }
            }
            Mv::Pick(_) | Mv::Reveal => {}
        }
        if solved || r.used >= MAX_USED {
            r.cost[r.guesser()] += if solved { r.used as i32 } else { FAIL_COST };
            r.log = format!(
                "Runda {} încheiată: obiectul era «{}»{}.",
                r.round,
                ITEMS[r.secret].0,
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
                r.answers.clear();
                r.eliminated.clear();
                r.used = 0;
            }
        }
        r
    }

    fn candidates(&self) -> Vec<u8> {
        (0..ITEMS.len() as u8)
            .filter(|&i| {
                !self.eliminated.contains(&i)
                    && self
                        .answers
                        .iter()
                        .all(|&(q, a)| (ITEMS[i as usize].1 >> q & 1 == 1) == a)
            })
            .collect()
    }
}

impl Game for Douazeci {
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
            Phase::Over if self.pending.is_some() => vec![Mv::Reveal],
            Phase::Over => Vec::new(),
            Phase::Pick => (0..ITEMS.len() as u8).map(Mv::Pick).collect(),
            Phase::Ask => {
                let mut v: Vec<Mv> = (0..QUESTIONS.len() as u8)
                    .filter(|q| !r.answers.iter().any(|a| a.0 == *q))
                    .map(Mv::Ask)
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
                self.phase = Phase::Ask;
                self.log = format!(
                    "Runda {}: J{} s-a gândit la un obiect.",
                    self.round,
                    self.setter + 1
                );
            }
            Mv::Reveal => {}
            g => self.pending = Some(g),
        }
    }
    fn outcome(&self) -> Option<Outcome> {
        // cât timp ultima mutare e „în așteptare”, rezultatul nu se citește (nu depinde de secret)
        if self.pending.is_some() {
            return None;
        }
        (self.phase == Phase::Over).then(|| by_points(self.cost[1], self.cost[0]))
    }
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        // cost mai mic = mai bine
        let mut v = 1000 * (self.cost[1 - me] - self.cost[me]);
        if let Some(m) = self.pending {
            let c = self.candidates();
            let n = c.len().max(1) as f32;
            let val = match m {
                Mv::Ask(q) => {
                    let yes = c
                        .iter()
                        .filter(|&&i| ITEMS[i as usize].1 >> q & 1 == 1)
                        .count() as f32
                        / n;
                    let info = 1.0 - (2.0 * yes - 1.0).abs();
                    if c.len() > 3 {
                        info
                    } else {
                        info * 0.3
                    }
                }
                Mv::Guess(i) => {
                    if c.contains(&i) {
                        1.0 / n
                    } else {
                        0.0
                    }
                }
                Mv::Pick(_) | Mv::Reveal => 0.0,
            };
            let x = (val * 400.0) as i32;
            v += if self.guesser() == me { x } else { -x };
        } else if self.phase == Phase::Ask && self.used == 0 && self.setter == me {
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
            let names: Vec<&str> = ITEMS.iter().map(|x| x.0).collect();
            s.push_str(&format!("Obiecte posibile: {}\n", names.join(", ")));
        }
        if r.phase == Phase::Ask {
            s.push_str(&format!("Întrebări folosite: {}/{MAX_USED}.\n", r.used));
            for (i, q) in QUESTIONS.iter().enumerate() {
                let a = r
                    .answers
                    .iter()
                    .find(|x| x.0 as usize == i)
                    .map(|x| if x.1 { "DA" } else { "NU" })
                    .unwrap_or(" ");
                s.push_str(&format!("{:>2}. {q} {a}\n", i));
            }
            s.push_str(&format!(
                "Încă posibile: {} obiecte.\n",
                r.candidates().len()
            ));
            if view == r.setter {
                s.push_str(&format!(
                    "Secretul tău (J{}): {}\n",
                    view + 1,
                    ITEMS[r.secret].0
                ));
            }
        }
        s.push_str(r.log.trim());
        if r.phase != Phase::Over {
            s.push_str(&format!(
                "\nLa mutare: Jucătorul {}.",
                self.to_move().idx() + 1
            ));
        } else if self.pending.is_some() {
            s.push_str("\nScrie «dezvaluie».");
        }
        s
    }
    fn format_move(&self, m: &Mv) -> String {
        match *m {
            Mv::Pick(i) => format!("obiect {}", ITEMS[i as usize].0),
            Mv::Ask(q) => format!("intreb {q}"),
            Mv::Guess(i) => format!("este {}", ITEMS[i as usize].0),
            Mv::Reveal => "dezvaluie".into(),
        }
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
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
    fn items_have_distinct_profiles() {
        let mut v: Vec<u32> = ITEMS.iter().map(|x| x.1).collect();
        v.sort_unstable();
        v.dedup();
        assert_eq!(v.len(), ITEMS.len());
    }

    #[test]
    fn answering_and_guessing() {
        let mut g = Douazeci::with_seed(1);
        GameObj::play(&mut g, "obiect caine").unwrap();
        let mut guesser = g.clone_box();
        guesser.set_viewer(Some(Player::Two));
        assert!(!guesser.render().contains("caine"));
        GameObj::play(&mut g, "intreb 1").unwrap(); // E un animal?
        assert!(
            GameObj::render(&g).contains("DA"),
            "{}",
            GameObj::render(&g)
        );
        GameObj::play(&mut g, "este caine").unwrap();
        assert_eq!(g.costs().1, 2);
    }

    #[test]
    fn random_and_ai_games_finish() {
        random_finishes(|s| Box::new(Douazeci::with_seed(s)), 6, 400);
        let r = ai_vs_random(|s| Box::new(Douazeci::with_seed(s)), 2, Player::One, 400);
        assert!(r.is_some());
        ai_vs_random(|s| Box::new(Douazeci::with_seed(s)), 3, Player::Two, 400);
    }
}

#[cfg(test)]
mod a1_blind {
    use super::*;
    use crate::ai_audit_a1::audit;

    /// Un obiect secret alternativ, la fel de compatibil cu tot ce a aflat ghicitorul.
    fn same_view_secret(g: &Douazeci, salt: u64) -> usize {
        let n = ITEMS.len();
        let ans = |i: usize, q: u8| ITEMS[i].1 >> q & 1;
        for k in 0..n {
            let c = (salt as usize + 1 + k) % n;
            if c == g.secret {
                continue;
            }
            let mut qs: Vec<u8> = g.answers.iter().map(|a| a.0).collect();
            if let Some(Mv::Ask(q)) = g.pending {
                qs.push(q);
            }
            let same = qs.iter().all(|&q| ans(c, q) == ans(g.secret, q));
            let wrong_guess = match g.pending {
                Some(Mv::Guess(i)) => i as usize != c,
                _ => true,
            };
            if same && wrong_guess && !g.eliminated.contains(&(c as u8)) {
                return c;
            }
        }
        g.secret
    }

    #[test]
    fn ai_ignores_the_secret_object_at_all_levels() {
        audit("douazeci", Douazeci::with_seed, 1..=4, 1, 60, |g, salt| {
            // doar ghicitorul nu cunoaște obiectul
            let r = g.resolved();
            if r.phase == Phase::Ask && g.to_move().idx() == r.guesser() {
                g.secret = same_view_secret(g, salt);
            }
        });
    }

    #[test]
    fn last_guess_is_not_decided_before_the_forced_reveal() {
        // runda 2, ultima ghicire: rezultatul apare abia după `dezvaluie`
        let mut g = Douazeci::with_seed(3);
        g.apply(&Mv::Pick(0));
        g.apply(&Mv::Guess(0));
        g.apply(&Mv::Pick(1));
        assert_eq!(g.outcome(), None);
        g.apply(&Mv::Guess(1));
        assert_eq!(g.outcome(), None);
        assert_eq!(g.legal_moves(), vec![Mv::Reveal]);
        g.apply(&Mv::Reveal);
        assert!(g.outcome().is_some());
        assert!(g.legal_moves().is_empty());
    }
}
