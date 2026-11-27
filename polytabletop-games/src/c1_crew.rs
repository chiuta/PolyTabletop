#![allow(
    clippy::needless_range_loop,
    clippy::manual_is_multiple_of,
    clippy::collapsible_if
)]
//! Motor cooperativ „levate cu misiuni” (lista C1, 2 jucători, variantă originală).
//!
//! Pachet de 40 de cărți: patru culori (a, b, c, d) cu valorile 1–9 și o culoare-atu (t) 1–4.
//! Fiecare jucător primește 10 cărți (restul de 20 rămân nevăzute). `tasks` cărți-țintă din mâinile
//! amândurora (nu atu) sunt atribuite pe rând jucătorilor: ținta trebuie luată într-o levată câștigată
//! de jucătorul ei. Se joacă `joc a5`: obligatoriu culoarea cerută dacă o ai; atuul bate culorile,
//! altfel cea mai mare carte din culoarea cerută; câștigătorul conduce. Mutarea forțată `levez` închide
//! levata și verifică țintele: o țintă luată de celălalt jucător = eșec (REMIZĂ); toate țintele
//! îndeplinite după 10 levate = victoria echipei.
//! Informație ascunsă: mâna fiecăruia (HOTSEAT_SAFE); țintele sunt publice. AI-ul folosește doar mâna
//! proprie, levata în curs și țintele.
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

pub struct CSpec {
    pub title: &'static str,
    pub tasks: usize,
}

const SUITS: [&str; 5] = ["a", "b", "c", "d", "t"];

fn suit(c: u8) -> u8 {
    c / 10
}
fn val(c: u8) -> u8 {
    c % 10
}
fn name(c: u8) -> String {
    format!("{}{}", SUITS[suit(c) as usize], val(c))
}

#[derive(Clone)]
pub struct Crew {
    sp: &'static CSpec,
    hand: [Vec<u8>; 2],
    trick: Vec<(usize, u8)>,
    lead: usize,
    tasks: Vec<(u8, usize, bool)>,
    tricks: u32,
    seen: Vec<u8>,
    judge: bool,
    end: Option<Outcome>,
    viewer: Option<Player>,
}

impl Crew {
    pub fn new(sp: &'static CSpec) -> Self {
        Self::with_seed(sp, seed_now())
    }
    pub fn with_seed(sp: &'static CSpec, seed: u64) -> Self {
        let mut rng = Rng::new(seed ^ 0x85eb_ca6b);
        let mut deck: Vec<u8> = Vec::new();
        for s in 0..4u8 {
            for v in 1..=9u8 {
                deck.push(s * 10 + v);
            }
        }
        for v in 1..=4u8 {
            deck.push(40 + v);
        }
        shuf(&mut deck, &mut rng);
        let mut hand = [deck[..10].to_vec(), deck[10..20].to_vec()];
        for h in hand.iter_mut() {
            h.sort();
        }
        let mut cand: Vec<u8> = deck[..20]
            .iter()
            .copied()
            .filter(|&c| suit(c) < 4)
            .collect();
        shuf(&mut cand, &mut rng);
        let tasks = cand
            .iter()
            .take(sp.tasks)
            .enumerate()
            .map(|(i, &c)| (c, i % 2, false))
            .collect();
        Crew {
            sp,
            hand,
            trick: Vec::new(),
            lead: 0,
            tasks,
            tricks: 0,
            seen: Vec::new(),
            judge: false,
            end: None,
            viewer: None,
        }
    }
    fn turn(&self) -> usize {
        (self.lead + self.trick.len()) % 2
    }
    fn winner(trick: &[(usize, u8)]) -> usize {
        let mut best = trick[0];
        for &(p, c) in &trick[1..] {
            let (bs, cs) = (suit(best.1), suit(c));
            if (cs == 4 && bs != 4) || (cs == bs && val(c) > val(best.1)) {
                best = (p, c);
            }
        }
        best.0
    }
}

impl Mini for Crew {
    fn side(&self) -> Player {
        if self.judge {
            return if self.lead == 0 {
                Player::One
            } else {
                Player::Two
            };
        }
        if self.turn() == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        if self.judge {
            return vec!["levez".into()];
        }
        let me = self.turn();
        let h = &self.hand[me];
        let mut cards: Vec<u8> = h.clone();
        if let Some(&(_, c0)) = self.trick.first() {
            let s = suit(c0);
            if h.iter().any(|&c| suit(c) == s) {
                cards.retain(|&c| suit(c) == s);
            }
        }
        cards.iter().map(|&c| format!("joc {}", name(c))).collect()
    }
    fn step(&mut self, m: &str) {
        if m == "levez" {
            let w = Self::winner(&self.trick);
            for t in self.tasks.iter_mut() {
                if self.trick.iter().any(|&(_, c)| c == t.0) {
                    if t.1 == w {
                        t.2 = true;
                    } else {
                        self.end = Some(Outcome::Draw);
                    }
                }
            }
            self.trick.clear();
            self.lead = w;
            self.tricks += 1;
            self.judge = false;
            if self.end.is_none() && self.tricks >= 10 {
                self.end = Some(if self.tasks.iter().all(|t| t.2) {
                    Outcome::Win(Player::One)
                } else {
                    Outcome::Draw
                });
            }
            return;
        }
        let me = self.turn();
        let s = m[4..5].chars().next().unwrap();
        let sidx = SUITS.iter().position(|x| x.starts_with(s)).unwrap() as u8;
        let v: u8 = m[5..].parse().unwrap();
        let c = sidx * 10 + v;
        let i = self.hand[me].iter().position(|&x| x == c).unwrap();
        self.hand[me].remove(i);
        self.trick.push((me, c));
        self.seen.push(c);
        if self.trick.len() == 2 {
            self.judge = true;
        }
    }
    fn result(&self) -> Option<Outcome> {
        self.end
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        if self.judge {
            let w = Self::winner(&self.trick);
            let mut s = 300;
            let mut touched = false;
            for t in &self.tasks {
                if self.trick.iter().any(|&(_, c)| c == t.0) {
                    touched = true;
                    s += if t.1 == w { 1000 } else { -5000 };
                }
            }
            if !touched {
                // nu irosi atuurile; câștigă levata doar când ajută
                let tr = self
                    .trick
                    .iter()
                    .filter(|&&(q, c)| q == a && suit(c) == 4)
                    .count() as i32;
                s -= 25 * tr;
                if w == a {
                    s += 10;
                }
            }
            return s;
        }
        // am condus: evaluez cartea condusă
        if let Some(&(q, c)) = self.trick.last() {
            if q == a {
                let own = self.tasks.iter().find(|t| t.0 == c && !t.2);
                return match own {
                    Some(t) if t.1 == a => {
                        500 + 15 * val(c) as i32 + if suit(c) == 4 { 200 } else { 0 }
                    }
                    Some(_) => 100,
                    None => 300 - 6 * val(c) as i32 - if suit(c) == 4 { 60 } else { 0 },
                };
            }
        }
        0
    }
    fn show(&self) -> String {
        let me = pi(self.viewer.unwrap_or(self.side()));
        let mut s = format!(
            "{} — levata {}/10\n",
            self.sp.title,
            (self.tricks + 1).min(10)
        );
        let h: Vec<String> = self.hand[me].iter().map(|&c| name(c)).collect();
        s.push_str(&format!(
            "mâna lui J{} (ascunsă celuilalt): {}\n",
            me + 1,
            h.join(" ")
        ));
        s.push_str(&format!("cărți la celălalt: {}\n", self.hand[1 - me].len()));
        for t in &self.tasks {
            s.push_str(&format!(
                "  țintă {}: levata cu această carte trebuie luată de J{} [{}]\n",
                name(t.0),
                t.1 + 1,
                if t.2 { "îndeplinită" } else { "deschisă" }
            ));
        }
        let t: Vec<String> = self
            .trick
            .iter()
            .map(|&(p, c)| format!("J{}:{}", p + 1, name(c)))
            .collect();
        s.push_str(&format!("levata curentă: {}\n", t.join(" ")));
        s.push_str(&format!(
            "La mutare: {} (joc <culoare><valoare>, ex. joc a5{})\n",
            pname(self.side()),
            if self.judge { " | levez" } else { "" }
        ));
        s
    }
    fn view(&mut self, w: Option<Player>) {
        self.viewer = w;
    }
}

#[cfg(test)]
impl Crew {
    /// Pentru test: înlocuiește mâna lui `hide` cu alte cărți nevăzute (aceeași mărime).
    pub fn scramble(&mut self, hide: usize, seed: u64) {
        let mut rng = Rng::new(seed);
        let mut pool: Vec<u8> = (0..40u8)
            .map(|i| {
                if i < 36 {
                    (i / 9) * 10 + i % 9 + 1
                } else {
                    40 + (i - 36) + 1
                }
            })
            .filter(|c| {
                !self.hand[1 - hide].contains(c)
                    && !self.seen.contains(c)
                    && !self.tasks.iter().any(|t| t.0 == *c)
            })
            .collect();
        shuf(&mut pool, &mut rng);
        let n = self.hand[hide].len();
        let keep: Vec<u8> = self.hand[hide]
            .iter()
            .copied()
            .filter(|c| self.tasks.iter().any(|t| t.0 == *c))
            .collect();
        let mut h = keep;
        while h.len() < n {
            h.push(pool.pop().unwrap());
        }
        h.sort();
        self.hand[hide] = h;
    }
}
