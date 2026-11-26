//! Intrigile Cetății — alegere de roluri și construcție de cartiere (variantă originală, 2 jucători).
//!
//! Șase roluri, jucate în ordine în fiecare rundă: 1 Asasin (alege un rol de rând care e scos),
//! 2 Hoț (ia aurul celui care joacă rolul ales), 3 Rege (+1 aur per cartier nobil, primești coroana),
//! 4 Negustor (+1 aur, +1 per cartier comercial), 5 Arhitect (trage 2 cărți, construiește până la 3),
//! 6 Căpitan (+1 aur per cartier militar; poate distruge un cartier al adversarului plătind costul−1).
//! Alegerea: cine are coroana alege primul, apoi `alege r` în ordinea C, A, A, C (fiecare 2 roluri).
//! Cu rolul tău: `tinta r` (Asasin/Hoț), apoi `aur` (+2) sau `carte` (+2 cărți), `construiesc i`
//! (cartea i din mână, plătești costul; 1 cartier pe rol, 3 pentru Arhitect) sau `gata`, iar Căpitanul
//! `distrug i` sau `nu`. Mâinile sunt deschise. Când cineva are 7 cartiere la sfârșitul rundei,
//! jocul se oprește: scor = costurile cartierelor + 3 dacă are toate cele 4 culori + 2 cine a terminat primul.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const ROLES: [&str; 6] = ["Asasin", "Hot", "Rege", "Negustor", "Arhitect", "Capitan"];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ph {
    Draft,
    Target,
    Income,
    Build,
    Destroy,
}

type Card = (u8, u8); // (cost, culoare)

#[derive(Clone)]
pub struct Cetate {
    gold: [i32; 2],
    hand: [Vec<Card>; 2],
    city: [Vec<Card>; 2],
    deck: Vec<Card>,
    owner: [Option<usize>; 6],
    dead: Option<usize>,
    robbed: Option<usize>,
    crown: usize,
    picks: usize,
    cur: usize,
    ph: Ph,
    left: u8,
    round: u32,
    first_done: Option<usize>,
    over: bool,
    ply: u32,
}

impl Cetate {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck: Vec<Card> = Vec::new();
        for col in 0..4u8 {
            for c in [1u8, 1, 2, 2, 3, 3, 4, 4, 5, 5] {
                deck.push((c, col));
            }
        }
        shuf(&mut deck, &mut rng);
        let h1 = (0..3).map(|_| deck.pop().unwrap()).collect();
        let h2 = (0..3).map(|_| deck.pop().unwrap()).collect();
        Cetate {
            gold: [2, 2],
            hand: [h1, h2],
            city: [Vec::new(), Vec::new()],
            deck,
            owner: [None; 6],
            dead: None,
            robbed: None,
            crown: 0,
            picks: 0,
            cur: 0,
            ph: Ph::Draft,
            left: 1,
            round: 0,
            first_done: None,
            over: false,
            ply: 0,
        }
    }
    fn actor(&self) -> usize {
        if self.ph == Ph::Draft {
            let seq = [self.crown, 1 - self.crown, 1 - self.crown, self.crown];
            seq[self.picks]
        } else {
            self.owner[self.cur].unwrap()
        }
    }
    fn score(&self, p: usize) -> i32 {
        let mut s: i32 = self.city[p].iter().map(|c| c.0 as i32).sum();
        if (0..4).all(|col| self.city[p].iter().any(|c| c.1 == col)) {
            s += 3;
        }
        if self.first_done == Some(p) {
            s += 2;
        }
        s
    }
    fn count(&self, p: usize, col: u8) -> i32 {
        self.city[p].iter().filter(|c| c.1 == col).count() as i32
    }
    fn draw(&mut self, p: usize, n: usize) {
        for _ in 0..n {
            if let Some(c) = self.deck.pop() {
                self.hand[p].push(c);
            }
        }
    }
    fn begin_role(&mut self) {
        // pornește rolul `cur` (există proprietar)
        let r = self.cur;
        let p = self.owner[r].unwrap();
        if self.dead == Some(r) {
            self.next_role();
            return;
        }
        if self.robbed == Some(r) && r != 1 {
            if let Some(t) = self.owner[1] {
                if t != p {
                    self.gold[t] += self.gold[p];
                    self.gold[p] = 0;
                }
            }
        }
        self.left = if r == 4 { 3 } else { 1 };
        self.ph = if r < 2 { Ph::Target } else { Ph::Income };
    }
    fn next_role(&mut self) {
        let mut r = self.cur + 1;
        while r < 6 && self.owner[r].is_none() {
            r += 1;
        }
        if r >= 6 {
            self.end_round();
        } else {
            self.cur = r;
            self.begin_role();
        }
    }
    fn end_round(&mut self) {
        self.round += 1;
        for p in 0..2 {
            if self.city[p].len() >= 7 && self.first_done.is_none() {
                self.first_done = Some(p);
            }
        }
        if self.first_done.is_some() || self.round >= 20 {
            self.over = true;
            return;
        }
        if let Some(k) = self.owner[2] {
            if self.dead != Some(2) {
                self.crown = k;
            }
        }
        self.owner = [None; 6];
        self.dead = None;
        self.robbed = None;
        self.picks = 0;
        self.ph = Ph::Draft;
    }
    fn start_acting(&mut self) {
        self.cur = 0;
        while self.cur < 6 && self.owner[self.cur].is_none() {
            self.cur += 1;
        }
        self.begin_role();
    }
}

impl Default for Cetate {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Cetate {
    fn depth(&self) -> u32 {
        1
    }
    fn side(&self) -> Player {
        if self.actor() == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn moves(&self) -> Vec<String> {
        let me = self.actor();
        let mut v = Vec::new();
        match self.ph {
            Ph::Draft => {
                for r in 0..6 {
                    if self.owner[r].is_none() {
                        v.push(format!("alege {}", r + 1));
                    }
                }
            }
            Ph::Target => {
                for r in (self.cur + 1)..6 {
                    if self.cur == 1 && r < 2 {
                        continue;
                    }
                    if self.cur == 1 && self.dead == Some(r) {
                        continue;
                    }
                    v.push(format!("tinta {}", r + 1));
                }
            }
            Ph::Income => {
                v.push("aur".into());
                v.push("carte".into());
            }
            Ph::Build => {
                v.push("gata".into());
                if self.left > 0 {
                    let mut seen: Vec<Card> = Vec::new();
                    for (i, c) in self.hand[me].iter().enumerate() {
                        if c.0 as i32 <= self.gold[me]
                            && !seen.contains(c)
                            && !self.city[me].contains(c)
                        {
                            seen.push(*c);
                            v.push(format!("construiesc {}", i + 1));
                        }
                    }
                }
            }
            Ph::Destroy => {
                v.push("nu".into());
                if self.city[1 - me].len() < 7 {
                    for (i, c) in self.city[1 - me].iter().enumerate() {
                        if (c.0 as i32 - 1) <= self.gold[me] {
                            v.push(format!("distrug {}", i + 1));
                        }
                    }
                }
            }
        }
        v
    }
    fn step(&mut self, m: &str) {
        let me = self.actor();
        self.ply += 1;
        let (cmd, arg) = m.split_once(' ').unwrap_or((m, ""));
        match (self.ph, cmd) {
            (Ph::Draft, _) => {
                let r: usize = arg.parse::<usize>().unwrap() - 1;
                self.owner[r] = Some(me);
                self.picks += 1;
                if self.picks == 4 {
                    self.start_acting();
                }
            }
            (Ph::Target, _) => {
                let r: usize = arg.parse::<usize>().unwrap() - 1;
                if self.cur == 0 {
                    self.dead = Some(r);
                } else {
                    self.robbed = Some(r);
                }
                self.ph = Ph::Income;
            }
            (Ph::Income, c) => {
                if c == "aur" {
                    self.gold[me] += 2;
                } else {
                    self.draw(me, 2);
                }
                match self.cur {
                    2 => self.gold[me] += self.count(me, 0),
                    3 => self.gold[me] += 1 + self.count(me, 2),
                    4 => self.draw(me, 2),
                    5 => self.gold[me] += self.count(me, 3),
                    _ => {}
                }
                self.ph = Ph::Build;
            }
            (Ph::Build, "construiesc") => {
                let i = arg.parse::<usize>().unwrap() - 1;
                let c = self.hand[me].remove(i);
                self.gold[me] -= c.0 as i32;
                self.city[me].push(c);
                self.left -= 1;
            }
            (Ph::Build, _) => {
                if self.cur == 5 && !self.city[1 - me].is_empty() {
                    self.ph = Ph::Destroy;
                } else {
                    self.next_role();
                }
            }
            (Ph::Destroy, "distrug") => {
                let i = arg.parse::<usize>().unwrap() - 1;
                let c = self.city[1 - me].remove(i);
                self.gold[me] -= c.0 as i32 - 1;
                self.next_role();
            }
            (Ph::Destroy, _) => self.next_role(),
        }
        if self.ph == Ph::Build && self.left == 0 && self.cur != 5 && cmd == "construiesc" {
            self.next_role();
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.over || self.ply > 1500 {
            return Some(by_score(self.score(0), self.score(1)));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let f = |i: usize| {
            10 * self.score(i)
                + 3 * self.gold[i]
                + 4 * self.hand[i].len() as i32
                + 2 * self.city[i].len() as i32
        };
        f(a) - f(1 - a)
    }
    fn show(&self) -> String {
        let mut s = format!(
            "Intrigile Cetății — runda {} | coroana: J{}\n",
            self.round + 1,
            self.crown + 1
        );
        s.push_str("Roluri: ");
        for r in 0..6 {
            s.push_str(&format!(
                "{}{}{}{} ",
                r + 1,
                ROLES[r],
                match self.owner[r] {
                    Some(p) => format!("(J{})", p + 1),
                    None => String::new(),
                },
                if self.dead == Some(r) { "†" } else { "" }
            ));
        }
        s.push('\n');
        for p in 0..2 {
            let fmt = |v: &Vec<Card>| {
                v.iter()
                    .map(|c| format!("{}{}", c.0, ['n', 'b', 'c', 'm'][c.1 as usize]))
                    .collect::<Vec<_>>()
                    .join(" ")
            };
            s.push_str(&format!(
                "J{}: aur {} | cartiere [{}] | mână [{}]\n",
                p + 1,
                self.gold[p],
                fmt(&self.city[p]),
                fmt(&self.hand[p])
            ));
        }
        s.push_str(&format!(
            "(culori: n nobil, b biserică, c comerț, m militar) | La mutare: {} (faza {:?}{})\n",
            pname(self.side()),
            self.ph,
            if self.ph != Ph::Draft {
                format!(", rol {}", ROLES[self.cur])
            } else {
                String::new()
            }
        ));
        s
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        crate::brand_util::check(Cetate::with_seed, 1500);
    }
}
