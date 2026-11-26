//! Gândaci Bluffați — bluf cu cărți-gâză (variantă originală, 2 jucători).
//!
//! 36 de cărți: 6 feluri de gâze (0–5), câte 6 din fiecare. Fiecare jucător primește 12 în mână (restul de 12 rămân în
//! cutie, necunoscute). Cel la rând joacă `pas t c`: cartea de tipul t, cu fața în jos, anunțând că e de tipul c. Adversarul
//! spune `adevar` (crede anunțul) sau `minciuna`; mutarea forțată `arata` întoarce cartea: dacă adversarul a avut
//! dreptate, cartea o ia cel care a jucat-o, altfel o ia cel care a ghicit. Cartea ajunge, cu fața în sus, în fața
//! celui care o ia, și acela joacă următorul. Cine strânge 4 gâze de același fel pierde; dacă cel la rând nu mai
//! are cărți, câștigă cine are mai puține cărți în față.
//! Informație ascunsă: mâna și cartea jucată; AI-ul adversar nu le citește (doar numără ce a văzut).
use crate::b6_util::pl;
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const NAMES: [&str; 6] = [
    "gândac",
    "furnică",
    "albină",
    "păianjen",
    "fluture",
    "greiere",
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ph {
    Play,
    Call(usize, usize),
    Resolve(usize, usize, bool),
}

#[derive(Clone)]
pub struct Gandaci {
    viewer: Option<Player>,
    hand: [[i32; 6]; 2],
    table: [[i32; 6]; 2],
    active: usize,
    ph: Ph,
    over: Option<Outcome>,
}

impl Gandaci {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck: Vec<usize> = (0..36).map(|i| i % 6).collect();
        shuf(&mut deck, &mut rng);
        let mut hand = [[0; 6]; 2];
        for i in 0..24 {
            hand[i % 2][deck[i]] += 1;
        }
        let mut g = Gandaci {
            viewer: None,
            hand,
            table: [[0; 6]; 2],
            active: 0,
            ph: Ph::Play,
            over: None,
        };
        g.check_start();
        g
    }
    fn check_start(&mut self) {
        if self.over.is_none()
            && self.ph == Ph::Play
            && self.hand[self.active].iter().sum::<i32>() == 0
        {
            let t: Vec<i32> = (0..2).map(|p| self.table[p].iter().sum()).collect();
            self.over = Some(match t[0].cmp(&t[1]) {
                std::cmp::Ordering::Less => Outcome::Win(Player::One),
                std::cmp::Ordering::Greater => Outcome::Win(Player::Two),
                std::cmp::Ordering::Equal => Outcome::Draw,
            });
        }
    }
    /// Probabilitatea (în sutimi) ca adversarul lui `me` să aibă un tip `c`, din ce se vede.
    fn p_has(&self, me: usize, c: usize) -> i32 {
        let unseen_c = 6 - self.hand[me][c] - self.table[0][c] - self.table[1][c];
        let unseen: i32 = 36
            - self.hand[me].iter().sum::<i32>()
            - self.table[0].iter().sum::<i32>()
            - self.table[1].iter().sum::<i32>();
        if unseen_c <= 0 || unseen <= 0 {
            return 0;
        }
        (200 * unseen_c / unseen).min(90)
    }
    fn danger(&self, p: usize, t: usize) -> i32 {
        let c = self.table[p][t];
        match c {
            0 => 10,
            1 => 30,
            2 => 80,
            _ => 400,
        }
    }
}

impl Default for Gandaci {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Gandaci {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        match self.ph {
            Ph::Play => pl(self.active),
            Ph::Call(..) => pl(1 - self.active),
            Ph::Resolve(..) => pl(1 - self.active),
        }
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            Ph::Play => {
                let mut v = Vec::new();
                for t in 0..6 {
                    if self.hand[self.active][t] > 0 {
                        for c in 0..6 {
                            v.push(format!("pas {t} {c}"));
                        }
                    }
                }
                v
            }
            Ph::Call(..) => vec!["adevar".into(), "minciuna".into()],
            Ph::Resolve(..) => vec!["arata".into()],
        }
    }
    fn step(&mut self, m: &str) {
        match self.ph {
            Ph::Play => {
                let mut it = m[4..].split(' ');
                let t: usize = it.next().unwrap().parse().unwrap();
                let c: usize = it.next().unwrap().parse().unwrap();
                self.hand[self.active][t] -= 1;
                self.ph = Ph::Call(t, c);
            }
            Ph::Call(t, c) => {
                self.ph = Ph::Resolve(t, c, m == "adevar");
            }
            Ph::Resolve(t, c, believes) => {
                let truth = t == c;
                let caller_right = believes == truth;
                let taker = if caller_right {
                    self.active
                } else {
                    1 - self.active
                };
                self.table[taker][t] += 1;
                if self.table[taker][t] >= 4 {
                    self.over = Some(Outcome::Win(pl(1 - taker)));
                }
                self.active = taker;
                self.ph = Ph::Play;
                self.check_start();
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        self.over
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let pen = |q: usize| -> i32 { (0..6).map(|t| self.danger(q, t)).sum() };
        let base = pen(1 - a) - pen(a);
        let o = 1 - a;
        match self.ph {
            Ph::Call(t, c) if a == self.active => {
                // cunosc cartea mea (t); adversarul crede „adevar” dacă anunțul e verosimil
                let believes = self.p_has(a, c) >= 50;
                let caller_right = believes == (t == c);
                if caller_right {
                    base - 2 * self.danger(a, t)
                } else {
                    base + 2 * self.danger(o, t)
                }
            }
            Ph::Resolve(_, c, believes) if a != self.active => {
                // nu știu t: anunțul e adevărat cu probabilitatea p
                let p_true = self.p_has(a, c).max(10);
                let right = if believes { p_true } else { 100 - p_true };
                // dacă am dreptate o ia cel care a jucat, altfel eu (tip necunoscut: pericol mediu pentru tipul anunțat)
                base + right * self.danger(self.active, c) / 50
                    - (100 - right) * self.danger(a, c) / 50
            }
            _ => base,
        }
    }
    fn show(&self) -> String {
        let viewer = self.viewer.map(pi).unwrap_or(pi(self.side()));
        let row = |v: &[i32; 6]| {
            (0..6)
                .map(|t| format!("{}={}×{}", t, NAMES[t], v[t]))
                .collect::<Vec<_>>()
                .join(" ")
        };
        let mut s = format!(
            "Gândaci Bluffați | în față J1: {} | în față J2: {}\nMâna ta (J{}): {} | J{} are {} cărți\n",
            row(&self.table[0]),
            row(&self.table[1]),
            viewer + 1,
            row(&self.hand[viewer]),
            2 - viewer,
            self.hand[1 - viewer].iter().sum::<i32>()
        );
        match self.ph {
            Ph::Call(_, c) | Ph::Resolve(_, c, _) => s.push_str(&format!(
                "Anunț de la J{}: „e un {}” ({})\n",
                self.active + 1,
                NAMES[c],
                c
            )),
            _ => {}
        }
        s.push_str(&format!(
            "La mutare: {} (pas t c | adevar | minciuna | arata)\n",
            pname(self.side())
        ));
        s
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
        crate::brand_util::check(Gandaci::with_seed, 200);
    }
    #[test]
    fn caller_ai_ignores_the_played_card_and_hands() {
        let mut g = Gandaci::with_seed(6);
        let t = (0..6).find(|&t| g.hand[0][t] > 0).unwrap();
        g.step(&format!("pas {t} 2"));
        let mut h = g.clone();
        h.ph = Ph::Call((t + 1) % 6, 2);
        h.hand[0] = [0, 0, 0, 0, 0, 0];
        h.hand[0][0] = 5;
        let mut g2 = g.clone();
        g2.hand[0] = h.hand[0];
        for seed in 1..6 {
            assert_eq!(ai_of(&g, seed), ai_of(&h, seed));
            assert_eq!(ai_of(&g, seed), ai_of(&g2, seed));
        }
        let mut v = g.clone();
        v.view(Some(Player::Two));
        assert!(v.show().contains("Mâna ta (J2)"));
        let _ = Rng::new(1);
    }
}
