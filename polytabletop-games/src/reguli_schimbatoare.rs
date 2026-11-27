//! Reguli Mereu Schimbate — carte care își rescrie regulile (variantă originală, 2 jucători).
//!
//! Pachet de 31 de cărți: 12 Obiecte (șase feluri K0–K5, câte două), 7 Scopuri (G01, G23, G45, G03, G14, G25, G05:
//! perechea de obiecte cerută), 12 Reguli (Rd1–Rd3: câte cărți tragi la începutul turei; Rp1–Rp3: câte cărți joci,
//! câte două exemplare fiecare). La început se trage 1 și se joacă 1, fiecare are 3 cărți, nu e niciun scop.
//! La rândul tău tragi cât spune regula de tras, apoi joci `joc X` (obiect → în fața ta; scop → înlocuiește scopul;
//! regulă → înlocuiește regula de același fel) exact cât spune regula de jucat, sau `pas` după ce ai jucat cel puțin una.
//! Dacă cineva are ambele obiecte ale scopului curent, câștigă imediat (chiar și adversarul, dacă tu i-ai dat scopul).
//! După 80 de mutări câștigă cine are mai multe obiecte în față. Informație ascunsă: mâinile; AI-ul nu o citește pe a adversarului.
use crate::b6_util::pl;
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const GOALS: [(usize, usize); 7] = [(0, 1), (2, 3), (4, 5), (0, 3), (1, 4), (2, 5), (0, 5)];
const CAP: u32 = 80;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Card {
    K(u8),
    G(u8),
    D(u8),
    P(u8),
}

fn tok(c: Card) -> String {
    match c {
        Card::K(i) => format!("K{i}"),
        Card::G(i) => format!("G{}{}", GOALS[i as usize].0, GOALS[i as usize].1),
        Card::D(n) => format!("Rd{n}"),
        Card::P(n) => format!("Rp{n}"),
    }
}

#[derive(Clone)]
pub struct Reguli {
    viewer: Option<Player>,
    deck: Vec<Card>,
    discard: Vec<Card>,
    hand: [Vec<Card>; 2],
    keepers: [u8; 2],
    goal: Option<usize>,
    draw: u8,
    play: u8,
    played: u8,
    mover: usize,
    plies: u32,
    over: Option<Outcome>,
    rng: Rng,
}

impl Reguli {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck = Vec::new();
        for i in 0..6 {
            deck.push(Card::K(i));
            deck.push(Card::K(i));
        }
        for i in 0..7 {
            deck.push(Card::G(i));
        }
        for n in 1..=3 {
            deck.push(Card::D(n));
            deck.push(Card::D(n));
            deck.push(Card::P(n));
            deck.push(Card::P(n));
        }
        shuf(&mut deck, &mut rng);
        let mut g = Reguli {
            viewer: None,
            deck,
            discard: Vec::new(),
            hand: [Vec::new(), Vec::new()],
            keepers: [0; 2],
            goal: None,
            draw: 1,
            play: 1,
            played: 0,
            mover: 0,
            plies: 0,
            over: None,
            rng,
        };
        for _ in 0..3 {
            for p in 0..2 {
                let c = g.take();
                g.hand[p].push(c);
            }
        }
        g.begin_turn();
        g
    }
    fn take(&mut self) -> Card {
        if self.deck.is_empty() {
            self.deck = std::mem::take(&mut self.discard);
            shuf(&mut self.deck, &mut self.rng);
        }
        self.deck.pop().unwrap_or(Card::K(0))
    }
    fn begin_turn(&mut self) {
        for _ in 0..self.draw {
            let c = self.take();
            self.hand[self.mover].push(c);
        }
        self.played = 0;
        self.hand[self.mover].sort_by_key(|&c| tok(c));
    }
    fn has(&self, p: usize, k: usize) -> bool {
        self.keepers[p] & (1 << k) != 0
    }
    fn won(&self, p: usize) -> bool {
        self.goal
            .is_some_and(|g| self.has(p, GOALS[g].0) && self.has(p, GOALS[g].1))
    }
    fn prog(&self, p: usize) -> i32 {
        match self.goal {
            None => 0,
            Some(g) => self.has(p, GOALS[g].0) as i32 + self.has(p, GOALS[g].1) as i32,
        }
    }
    fn end_turn(&mut self) {
        self.mover = 1 - self.mover;
        self.begin_turn();
    }
}

impl Default for Reguli {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Reguli {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        pl(self.mover)
    }
    fn moves(&self) -> Vec<String> {
        let mut v: Vec<String> = Vec::new();
        for &c in &self.hand[self.mover] {
            let m = format!("joc {}", tok(c));
            if !v.contains(&m) {
                v.push(m);
            }
        }
        if self.played > 0 || v.is_empty() {
            v.push("pas".into());
        }
        v
    }
    fn step(&mut self, m: &str) {
        self.plies += 1;
        if m != "pas" {
            let t = &m[4..];
            let pos = self.hand[self.mover]
                .iter()
                .position(|&c| tok(c) == t)
                .unwrap();
            let c = self.hand[self.mover].remove(pos);
            match c {
                Card::K(i) => self.keepers[self.mover] |= 1 << i,
                Card::G(i) => {
                    self.goal = Some(i as usize);
                }
                Card::D(n) => self.draw = n,
                Card::P(n) => self.play = n,
            }
            if !matches!(c, Card::K(_)) {
                self.discard.push(c);
            }
            self.played += 1;
            for p in [self.mover, 1 - self.mover] {
                if self.won(p) {
                    self.over = Some(Outcome::Win(pl(p)));
                    return;
                }
            }
        }
        if self.plies >= CAP {
            let k = [self.keepers[0].count_ones(), self.keepers[1].count_ones()];
            self.over = Some(match k[0].cmp(&k[1]) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                std::cmp::Ordering::Equal => Outcome::Draw,
            });
            return;
        }
        if m == "pas" || self.played >= self.play || self.hand[self.mover].is_empty() {
            self.end_turn();
        }
    }
    fn result(&self) -> Option<Outcome> {
        self.over
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let o = 1 - a;
        100 * (self.prog(a) - self.prog(o))
            + 10 * (self.keepers[a].count_ones() as i32 - self.keepers[o].count_ones() as i32)
    }
    fn show(&self) -> String {
        let viewer = self.viewer.map(pi).unwrap_or(pi(self.side()));
        let kp = |p: usize| {
            (0..6)
                .filter(|&k| self.has(p, k))
                .map(|k| format!("K{k}"))
                .collect::<Vec<_>>()
                .join(" ")
        };
        let hand: Vec<String> = self.hand[viewer].iter().map(|&c| tok(c)).collect();
        format!(
            "Reguli Mereu Schimbate — mutarea {}/{} | trage {} | joacă {}\nScop: {}\nObiecte J1: {} | J2: {}\nMâna ta (J{}): {} | J{} are {} cărți\nLa mutare: {} (joc X | pas) — jucate în tura asta: {}\n",
            self.plies + 1,
            CAP,
            self.draw,
            self.play,
            self.goal.map(|g| tok(Card::G(g as u8))).unwrap_or("-".into()),
            kp(0),
            kp(1),
            viewer + 1,
            hand.join(" "),
            2 - viewer,
            self.hand[1 - viewer].len(),
            pname(self.side()),
            self.played
        )
    }
    fn view(&mut self, w: Option<Player>) {
        self.viewer = w;
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        crate::brand_util::check(Reguli::with_seed, 300);
    }
    #[test]
    fn hands_are_private_and_ai_ignores_the_other_hand() {
        let g = Reguli::with_seed(3);
        let mut h = g.clone();
        h.hand[1] = vec![Card::K(5); 4];
        for seed in 1..6 {
            assert_eq!(ai_of(&g, seed), ai_of(&h, seed));
        }
        let mut v = g.clone();
        v.view(Some(Player::One));
        let s = v.show();
        assert!(s.contains("Mâna ta (J1)") && !s.contains("Mâna ta (J2)"));
    }
}

#[cfg(test)]
mod a1_blind {
    use super::*;
    use crate::ai_audit_a1::{audit, scramble_pool};
    use crate::brand_util::B;

    #[test]
    fn ai_ignores_opponent_hand_and_deck_at_all_levels() {
        audit(
            "reguli_schimbatoare",
            |s| B(Reguli::with_seed(s)),
            1..=4,
            3,
            60,
            |g, salt| {
                let foe = 1 - g.0.mover;
                scramble_pool(&mut g.0.hand[foe], &mut g.0.deck, salt);
                g.0.rng = Rng::new(salt ^ 3);
            },
        );
    }
}
