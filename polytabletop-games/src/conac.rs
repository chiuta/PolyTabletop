//! Misterul din Conac — joc original de deducție (2 jucători).
//!
//! O crimă a avut loc în conac: un **suspect**, o **armă** și o **cameră**
//! formează soluția secretă. Mecanica „sugerezi, adversarul infirmă cu o carte,
//! deduci” este generică; personajele, armele, camerele și textele sunt ale
//! noastre. Nu există tablă de mers: oricând poți sugera orice cameră.
//!
//! * 21 de cărți: 6 suspecți (s1–s6), 6 arme (a1–a6), 9 camere (c1–c9).
//!   Una din fiecare categorie e pusă deoparte (soluția); celelalte 18 se
//!   împart: câte 9 fiecare. Nu rămân cărți libere.
//! * La rând: `sugerez s2 a3 c5` — adversarul trebuie să arate (numai ție) o
//!   carte din cele trei pe care le are (`arat s2`); dacă nu are niciuna,
//!   `nu-am`. Dacă ai de ales, alegi tu ce arăți.
//! * Sau `acuz s2 a3 c5`, apoi `verific`: acuzația corectă câștigă, cea
//!   greșită pierde pe loc. (Verificarea e o mutare separată ca AI-ul să nu
//!   „vadă” soluția.)
//! * Limită de siguranță: după 150 de sugestii, remiză.
//!
//! Informație ascunsă: `render` arată doar mâna Jucătorului 1 și ce i s-a
//! arătat lui. AI-ul deduce din propria mână, din cărțile arătate lui și din
//! sugestiile la care adversarul n-a avut niciun răspuns; nu citește mâna
//! adversarului sau soluția.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const SUSPECTS: [&str; 6] = [
    "Doamna Lavandă",
    "Domnul Chihlimbar",
    "Doctorul Salcâm",
    "Profesoara Mușețel",
    "Colonelul Rugină",
    "Contesa Indigo",
];
pub const WEAPONS: [&str; 6] = [
    "Frânghia",
    "Sfeșnicul",
    "Bastonul",
    "Cleștele",
    "Fierul de călcat",
    "Otrava de șoareci",
];
pub const ROOMS: [&str; 9] = [
    "Biblioteca",
    "Sera",
    "Pivnița",
    "Salonul",
    "Bucătăria",
    "Observatorul",
    "Capela",
    "Atelierul",
    "Mansarda",
];

const N_CARDS: usize = 21;
const SUGGESTION_CAP: usize = 150;

fn card_name(c: usize) -> String {
    match c {
        0..=5 => SUSPECTS[c].to_string(),
        6..=11 => WEAPONS[c - 6].to_string(),
        _ => ROOMS[c - 12].to_string(),
    }
}

fn card_code(c: usize) -> String {
    match c {
        0..=5 => format!("s{}", c + 1),
        6..=11 => format!("a{}", c - 5),
        _ => format!("c{}", c - 11),
    }
}

type Triple = (u8, u8, u8);

fn triple_cards(t: Triple) -> [usize; 3] {
    [t.0 as usize, t.1 as usize, t.2 as usize]
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Suggest(Triple),
    Accuse(Triple),
    /// Arată o carte (răspuns la sugestie).
    Show(u8),
    NoCard,
    Verify,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Turn,
    Respond(Triple),
    Verify(Triple),
}

#[derive(Clone, Copy, Debug)]
struct Event {
    who: Player,
    triple: Triple,
    /// `None` = adversarul nu a avut nicio carte; `Some(c)` = a arătat `c`.
    shown: Option<u8>,
    answered: bool,
}

#[derive(Clone)]
pub struct Conac {
    /// Whose private info `render` shows (`None` = the mover's, hot-seat).
    viewer: Option<Player>,
    solution: Triple,
    hands: [Vec<u8>; 2],
    side: Player,
    phase: Phase,
    log: Vec<Event>,
    suggestions: usize,
    result: Option<Outcome>,
}

impl Default for Conac {
    fn default() -> Self {
        Self::new()
    }
}

impl Conac {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let s = rng.below(6) as u8;
        let w = 6 + rng.below(6) as u8;
        let r = 12 + rng.below(9) as u8;
        let mut rest: Vec<u8> = (0..N_CARDS as u8)
            .filter(|&c| c != s && c != w && c != r)
            .collect();
        for i in (1..rest.len()).rev() {
            let j = rng.below(i + 1);
            rest.swap(i, j);
        }
        let h1: Vec<u8> = rest[..9].to_vec();
        let h2: Vec<u8> = rest[9..].to_vec();
        Conac {
            viewer: None,
            solution: (s, w, r),
            hands: [h1, h2],
            side: Player::One,
            phase: Phase::Turn,
            log: Vec::new(),
            suggestions: 0,
            result: None,
        }
    }

    pub fn hand(&self, p: Player) -> &[u8] {
        &self.hands[p.idx()]
    }

    pub fn solution(&self) -> Triple {
        self.solution
    }

    /// Soluțiile încă posibile din punctul de vedere al lui `p` (doar informația lui).
    pub fn candidates(&self, p: Player) -> Vec<Triple> {
        let mine = &self.hands[p.idx()];
        let mut known_opp = [false; N_CARDS];
        for e in &self.log {
            if e.who == p && e.answered {
                if let Some(c) = e.shown {
                    known_opp[c as usize] = true;
                }
            }
        }
        let mut out = Vec::new();
        for s in 0..6u8 {
            for w in 6..12u8 {
                for r in 12..21u8 {
                    let t = (s, w, r);
                    let cs = triple_cards(t);
                    if cs
                        .iter()
                        .any(|&c| mine.contains(&(c as u8)) || known_opp[c])
                    {
                        continue;
                    }
                    // sugestiile mele fără răspuns: toate cele trei cărți sunt la mine sau în soluție
                    let ok = self.log.iter().all(|e| {
                        if e.who != p || !e.answered || e.shown.is_some() {
                            return true;
                        }
                        triple_cards(e.triple)
                            .iter()
                            .all(|&c| mine.contains(&(c as u8)) || cs.contains(&c))
                    });
                    if ok {
                        out.push(t);
                    }
                }
            }
        }
        out
    }

    fn respond_cards(&self, t: Triple) -> Vec<u8> {
        let responder = self.side.other();
        let mut v: Vec<u8> = triple_cards(t)
            .iter()
            .map(|&c| c as u8)
            .filter(|c| self.hands[responder.idx()].contains(c))
            .collect();
        v.sort_unstable();
        v
    }

    /// Cât de „nesigură” e cartea (0 = hotărâtă, 1 = 50/50) față de candidați.
    fn uncertainty(cands: &[Triple], card: usize) -> f64 {
        if cands.is_empty() {
            return 0.0;
        }
        let k = cands
            .iter()
            .filter(|t| triple_cards(**t).contains(&card))
            .count() as f64;
        let f = k / cands.len() as f64;
        1.0 - (2.0 * f - 1.0).abs()
    }
}

impl Game for Conac {
    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }
    type Move = Move;

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Respond(_) => self.side.other(),
            _ => self.side,
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.result.is_some() {
            return Vec::new();
        }
        match self.phase {
            Phase::Turn => {
                let mut v = Vec::with_capacity(648);
                for s in 0..6u8 {
                    for w in 6..12u8 {
                        for r in 12..21u8 {
                            v.push(Move::Suggest((s, w, r)));
                        }
                    }
                }
                for s in 0..6u8 {
                    for w in 6..12u8 {
                        for r in 12..21u8 {
                            v.push(Move::Accuse((s, w, r)));
                        }
                    }
                }
                v
            }
            Phase::Respond(t) => {
                let c = self.respond_cards(t);
                if c.is_empty() {
                    vec![Move::NoCard]
                } else {
                    c.into_iter().map(Move::Show).collect()
                }
            }
            Phase::Verify(_) => vec![Move::Verify],
        }
    }

    fn apply(&mut self, m: &Move) {
        match *m {
            Move::Suggest(t) => {
                self.suggestions += 1;
                self.log.push(Event {
                    who: self.side,
                    triple: t,
                    shown: None,
                    answered: false,
                });
                self.phase = Phase::Respond(t);
            }
            Move::Show(c) => {
                if let Some(e) = self.log.last_mut() {
                    e.shown = Some(c);
                    e.answered = true;
                }
                self.phase = Phase::Turn;
                self.side = self.side.other();
            }
            Move::NoCard => {
                if let Some(e) = self.log.last_mut() {
                    e.shown = None;
                    e.answered = true;
                }
                self.phase = Phase::Turn;
                self.side = self.side.other();
            }
            Move::Accuse(t) => self.phase = Phase::Verify(t),
            Move::Verify => {
                if let Phase::Verify(t) = self.phase {
                    self.result = Some(if t == self.solution {
                        Outcome::Win(self.side)
                    } else {
                        Outcome::Win(self.side.other())
                    });
                }
            }
        }
        if self.result.is_none() && self.suggestions >= SUGGESTION_CAP && self.phase == Phase::Turn
        {
            self.result = Some(Outcome::Draw);
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.result
    }

    /// Doar informația lui `p` (mâna lui, cărțile arătate lui, sugestiile fără răspuns).
    fn evaluate(&self, p: Player) -> i32 {
        let cands = self.candidates(p);
        let n = cands.len().max(1) as f64;
        match self.phase {
            Phase::Verify(t) if self.side == p => {
                if cands.contains(&t) {
                    (200_000.0 / n - 100_000.0) as i32
                } else {
                    -100_000
                }
            }
            Phase::Respond(t) if self.side == p => {
                let mut v = 0.0;
                for c in triple_cards(t) {
                    v += 100.0 * Self::uncertainty(&cands, c);
                }
                let repeated = self.log[..self.log.len() - 1]
                    .iter()
                    .any(|e| e.who == p && e.triple == t);
                if repeated {
                    v -= 500.0;
                }
                v as i32
            }
            _ => {
                // preferă să arăt o carte pe care adversarul a văzut-o deja
                let mut b = 0;
                if let Some(e) = self.log.last() {
                    if e.who != p && e.answered {
                        if let Some(c) = e.shown {
                            let before = &self.log[..self.log.len() - 1];
                            if before.iter().any(|x| x.who != p && x.shown == Some(c)) {
                                b = 5;
                            }
                        }
                    }
                }
                -(cands.len() as i32) + b
            }
        }
    }

    fn render(&self) -> String {
        let me = self.viewer.unwrap_or_else(|| self.to_move());
        let mut s = String::from("Misterul din Conac\n");
        s.push_str("Suspecți: ");
        s.push_str(
            &(0..6)
                .map(|c| format!("{}={}", card_code(c), card_name(c)))
                .collect::<Vec<_>>()
                .join(", "),
        );
        s.push_str("\nArme: ");
        s.push_str(
            &(6..12)
                .map(|c| format!("{}={}", card_code(c), card_name(c)))
                .collect::<Vec<_>>()
                .join(", "),
        );
        s.push_str("\nCamere: ");
        s.push_str(
            &(12..21)
                .map(|c| format!("{}={}", card_code(c), card_name(c)))
                .collect::<Vec<_>>()
                .join(", "),
        );
        s.push('\n');
        if let Some(r) = self.result {
            s.push_str(&format!(
                "Joc încheiat ({:?}). Soluția: {}, {}, {}.\n",
                r,
                card_name(self.solution.0 as usize),
                card_name(self.solution.1 as usize),
                card_name(self.solution.2 as usize)
            ));
            return s;
        }
        let mut h = self.hands[me.idx()].clone();
        h.sort_unstable();
        s.push_str(&format!(
            "Mâna Jucătorului {}: {}\n",
            me.idx() + 1,
            h.iter()
                .map(|&c| card_code(c as usize))
                .collect::<Vec<_>>()
                .join(" ")
        ));
        let shown: Vec<String> = self
            .log
            .iter()
            .filter(|e| e.who == me && e.answered)
            .filter_map(|e| e.shown)
            .map(|c| card_code(c as usize))
            .collect();
        s.push_str(&format!(
            "Cărți arătate ție (la J{}): {}\n",
            me.other().idx() + 1,
            if shown.is_empty() {
                "—".to_string()
            } else {
                shown.join(" ")
            }
        ));
        s.push_str(&format!(
            "Soluții încă posibile pentru tine: {}\n",
            self.candidates(me).len()
        ));
        s.push_str("Ultimele sugestii:\n");
        let start = self.log.len().saturating_sub(6);
        for e in &self.log[start..] {
            let t = format!(
                "{} {} {}",
                card_code(e.triple.0 as usize),
                card_code(e.triple.1 as usize),
                card_code(e.triple.2 as usize)
            );
            let r = if !e.answered {
                "așteaptă răspuns".to_string()
            } else if e.shown.is_none() {
                "nicio carte".to_string()
            } else if e.who == me {
                format!("arătat: {}", card_code(e.shown.unwrap() as usize))
            } else {
                "a arătat o carte".to_string()
            };
            s.push_str(&format!("  J{}: {} — {}\n", e.who.idx() + 1, t, r));
        }
        if let Phase::Respond(t) = self.phase {
            {
                s.push_str(&format!(
                    "J{} sugerează {} {} {}: arată o carte din cele pe care le ai.\n",
                    self.side.idx() + 1,
                    card_code(t.0 as usize),
                    card_code(t.1 as usize),
                    card_code(t.2 as usize)
                ));
            }
        }
        s.push_str(&format!(
            "La mutare: Jucătorul {}\n",
            self.to_move().idx() + 1
        ));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        let t = |t: &Triple| {
            format!(
                "{} {} {}",
                card_code(t.0 as usize),
                card_code(t.1 as usize),
                card_code(t.2 as usize)
            )
        };
        match m {
            Move::Suggest(x) => format!("sugerez {}", t(x)),
            Move::Accuse(x) => format!("acuz {}", t(x)),
            Move::Show(c) => format!("arat {}", card_code(*c as usize)),
            Move::NoCard => "nu-am".into(),
            Move::Verify => "verific".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deal_is_complete_and_disjoint() {
        for seed in 1..20u64 {
            let g = Conac::with_seed(seed);
            let mut all: Vec<u8> = g.hands[0]
                .iter()
                .chain(g.hands[1].iter())
                .copied()
                .collect();
            all.push(g.solution.0);
            all.push(g.solution.1);
            all.push(g.solution.2);
            all.sort_unstable();
            assert_eq!(all, (0..21u8).collect::<Vec<_>>());
            assert_eq!(g.hands[0].len(), 9);
            assert!(g.solution.0 < 6 && (6..12).contains(&g.solution.1) && g.solution.2 >= 12);
        }
    }

    #[test]
    fn suggestion_response_flow() {
        let mut g = Conac::with_seed(1);
        // J1 sugerează cu trei cărți ale lui J2 (dacă are) -> J2 trebuie să arate
        let h2 = g.hands[1].clone();
        let pick = |lo: u8, hi: u8| (lo..hi).find(|c| h2.contains(c));
        let t = (
            pick(0, 6).unwrap_or(0),
            pick(6, 12).unwrap_or(6),
            pick(12, 21).unwrap_or(12),
        );
        g.apply(&Move::Suggest(t));
        assert_eq!(g.to_move(), Player::Two);
        let m = g.legal_moves();
        let expect = triple_cards(t)
            .iter()
            .filter(|&&c| h2.contains(&(c as u8)))
            .count();
        if expect == 0 {
            assert_eq!(m, vec![Move::NoCard]);
        } else {
            assert_eq!(m.len(), expect);
            assert!(m.iter().all(|x| matches!(x, Move::Show(_))));
        }
        g.apply(&m[0]);
        assert_eq!(g.to_move(), Player::Two); // după răspuns, rândul trece
    }

    #[test]
    fn accusation_two_step() {
        let mut g = Conac::with_seed(2);
        let sol = g.solution();
        g.apply(&Move::Accuse(sol));
        assert!(g.outcome().is_none());
        assert_eq!(g.legal_moves(), vec![Move::Verify]);
        g.apply(&Move::Verify);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));

        let mut g = Conac::with_seed(2);
        let mut wrong = g.solution();
        wrong.0 = (wrong.0 + 1) % 6;
        g.apply(&Move::Accuse(wrong));
        g.apply(&Move::Verify);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
    }

    #[test]
    fn candidates_always_contain_the_truth() {
        // joc cu sugestii aleatorii: soluția reală rămâne mereu printre candidați
        for seed in 1..=30u64 {
            let mut g = Conac::with_seed(seed);
            let mut rng = Rng::new(seed * 3);
            for _ in 0..25 {
                let sug: Vec<Move> = g
                    .legal_moves()
                    .into_iter()
                    .filter(|m| matches!(m, Move::Suggest(_)))
                    .collect();
                g.apply(&sug[rng.below(sug.len())]);
                let r = g.legal_moves();
                g.apply(&r[rng.below(r.len())]);
                for p in [Player::One, Player::Two] {
                    assert!(
                        g.candidates(p).contains(&g.solution()),
                        "seed {seed}: candidații {p:?} nu mai conțin soluția"
                    );
                }
            }
        }
    }

    #[test]
    fn no_response_narrows_candidates() {
        let mut g = Conac::with_seed(3);
        let before = g.candidates(Player::One).len();
        // sugestie cu cărți din mâna lui J1 sau a soluției: J2 nu poate infirma doar dacă toate sunt aici
        let sol = g.solution();
        g.apply(&Move::Suggest(sol));
        assert_eq!(g.legal_moves(), vec![Move::NoCard]);
        g.apply(&Move::NoCard);
        let after = g.candidates(Player::One);
        assert!(after.len() <= before);
        assert!(after.contains(&sol));
    }

    #[test]
    fn ai_wins_against_random() {
        let (mut ai, mut rnd) = (0, 0);
        for seed in 1..=20u64 {
            let ai_side = if seed % 2 == 0 {
                Player::One
            } else {
                Player::Two
            };
            let mut g = Conac::with_seed(seed);
            let mut rng = Rng::new(seed * 17);
            let mut plies = 0;
            while Game::outcome(&g).is_none() {
                let m = if g.to_move() == ai_side {
                    polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap()
                } else {
                    let l = g.legal_moves();
                    l[rng.below(l.len())]
                };
                g.apply(&m);
                plies += 1;
                assert!(plies < 2000);
            }
            match Game::outcome(&g) {
                Some(Outcome::Win(p)) if p == ai_side => ai += 1,
                Some(Outcome::Win(_)) => rnd += 1,
                _ => {}
            }
        }
        assert!(ai >= 15 && ai > rnd, "AI {ai} vs random {rnd}");
    }

    #[test]
    fn ai_vs_ai_terminates_with_a_winner() {
        for seed in 1..=6u64 {
            let mut g = Conac::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut plies = 0;
            while Game::outcome(&g).is_none() {
                let m = polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap();
                g.apply(&m);
                plies += 1;
                assert!(plies < 600, "seed {seed} nu se termină");
            }
        }
    }

    #[test]
    fn render_hides_opponent_and_solution() {
        let g = Conac::with_seed(4);
        let r = Game::render(&g);
        assert!(r.contains("Mâna Jucătorului 1"));
        assert!(!r.contains("Mâna Jucătorului 2"));
        assert!(!r.contains(&format!("Soluția: {}", card_name(g.solution.0 as usize))));
    }
}

#[cfg(test)]
mod a1_blind {
    use super::*;
    use crate::ai_audit_a1::{audit_seeds, shuffle};

    #[test]
    fn ai_ignores_opponent_hand_and_solution_at_all_levels() {
        audit_seeds(
            "conac",
            Conac::with_seed,
            1..=2,
            7,
            50,
            |g, salt| {
                // adversarul (cel care nu mută) și soluția sunt necunoscute celui care mută;
                // sugestiile deja arătate lui rămân (sunt istorie publică pentru el)
                let me = g.to_move().idx();
                let foe = 1 - me;
                let mut pool: Vec<u8> = (0..N_CARDS as u8)
                    .filter(|c| !g.hands[me].contains(c))
                    .collect();
                shuffle(&mut pool, salt);
                let mut hand: Vec<u8> = Vec::new();
                let mut sol = [255u8; 3];
                let range = [0u8..6, 6..12, 12..21];
                for (k, r) in range.iter().enumerate() {
                    if let Some(i) = pool.iter().position(|c| r.contains(c)) {
                        sol[k] = pool.remove(i);
                    }
                }
                hand.extend(pool.drain(..9.min(pool.len())));
                if sol.iter().all(|&c| c != 255) {
                    g.hands[foe] = hand;
                    g.solution = (sol[0], sol[1], sol[2]);
                }
            },
            &[7],
        );
    }
}
