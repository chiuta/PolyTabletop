//! Motor cooperativ „drum cu întâlniri” (lista C2): echipa (J1, doi eroi pe rând) străbate un șir
//! de întâlniri cu cărți de trei feluri; J2 este Jocul (forțat `eveniment`: întâlnirea lovește).
//! Întâlnirea următoare se dezvăluie printr-o mutare forțată `avans` (AI-ul nu o vede dinainte).
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

pub struct QSpec {
    pub title: &'static str,
    pub heroes: [&'static str; 2],
    pub kinds: [&'static str; 3],
    pub steps: usize,
    pub hp: i32,
    pub hand: usize,
    pub pow_base: i32,
    pub boss_pow: i32,
    pub rest: bool,
    pub loot: bool,
    pub clock: u32,
}

#[derive(Clone)]
struct Enc {
    kind: usize,
    power: i32,
    prog: i32,
}

#[derive(Clone)]
pub struct Q {
    sp: &'static QSpec,
    pos: usize,
    enc: Option<Enc>,
    hands: [Vec<(usize, i32)>; 2],
    hp: [i32; 2],
    clock: u32,
    hero: usize,
    ap: u8,
    side: Player,
    rng: Rng,
}

impl Q {
    pub fn new(sp: &'static QSpec) -> Self {
        Self::with_seed(sp, seed_now())
    }
    pub fn with_seed(sp: &'static QSpec, seed: u64) -> Self {
        let mut q = Q {
            sp,
            pos: 0,
            enc: None,
            hands: [Vec::new(), Vec::new()],
            hp: [sp.hp; 2],
            clock: 0,
            hero: 0,
            ap: 2,
            side: Player::One,
            rng: Rng::new(seed),
        };
        q.refill();
        q.reveal();
        q
    }
    fn card(&mut self) -> (usize, i32) {
        (self.rng.below(3), self.rng.below(4) as i32 + 1)
    }
    fn refill(&mut self) {
        for h in 0..2 {
            while self.hands[h].len() < self.sp.hand {
                let c = self.card();
                self.hands[h].push(c);
            }
        }
    }
    fn reveal(&mut self) {
        let kind = self.rng.below(3);
        let power = if self.pos >= self.sp.steps {
            self.sp.boss_pow
        } else {
            self.sp.pow_base + self.pos as i32 + self.rng.below(3) as i32
        };
        self.enc = Some(Enc {
            kind,
            power,
            prog: 0,
        });
    }
    fn alive(&self, h: usize) -> bool {
        self.hp[h] > 0
    }
    fn won(&self) -> bool {
        self.pos > self.sp.steps
    }
    fn lost(&self) -> bool {
        (!self.alive(0) && !self.alive(1))
            || self.clock > if self.sp.clock > 0 { self.sp.clock } else { 40 }
    }
    fn next_hero(&mut self) {
        self.ap = 2;
        if self.hero == 0 {
            self.hero = 1;
        } else {
            self.hero = 0;
            self.side = Player::Two;
        }
        if self.side == Player::One && !self.alive(self.hero) {
            self.next_hero();
        }
    }
    fn spend(&mut self) {
        self.ap -= 1;
        if self.ap == 0 {
            self.next_hero();
        }
    }
}

impl Mini for Q {
    fn side(&self) -> Player {
        self.side
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        if self.side == Player::Two {
            return vec!["eveniment".into()];
        }
        if self.enc.is_none() {
            return vec!["avans".into()];
        }
        let mut v = Vec::new();
        let h = &self.hands[self.hero];
        for (k, c) in h.iter().enumerate() {
            if !h[..k].contains(c) {
                v.push(format!("joc {}", k));
            }
        }
        if self.sp.rest && self.hp[self.hero] < self.sp.hp {
            v.push("odihna".into());
        }
        v.push("pas".into());
        v
    }
    fn step(&mut self, m: &str) {
        if m == "eveniment" {
            let dmg = self.enc.as_ref().map_or(0, |e| 1 + e.power / 4);
            let tgt = if self.alive(0) && self.alive(1) {
                (self.clock % 2) as usize
            } else if self.alive(0) {
                0
            } else {
                1
            };
            self.hp[tgt] -= dmg;
            self.clock += 1;
            self.refill();
            self.hero = 0;
            self.ap = 2;
            self.side = Player::One;
            if !self.alive(0) {
                self.hero = 1;
            }
            return;
        }
        match m {
            "avans" => {
                if self.pos <= self.sp.steps {
                    self.reveal();
                }
            }
            "pas" => self.next_hero(),
            "odihna" => {
                self.hp[self.hero] += 1;
                let c = self.card();
                self.hands[self.hero].push(c);
                self.spend();
            }
            _ => {
                let k: usize = m[4..].parse().unwrap();
                let (kind, val) = self.hands[self.hero].remove(k);
                let e = self.enc.as_mut().unwrap();
                e.prog += if kind == e.kind { val } else { (val + 1) / 2 };
                if e.prog >= e.power {
                    self.enc = None;
                    self.pos += 1;
                    if self.sp.loot {
                        for h in 0..2 {
                            let c = self.card();
                            self.hands[h].push(c);
                        }
                    } else {
                        for h in 0..2 {
                            if self.alive(h) {
                                self.hp[h] = (self.hp[h] + 1).min(self.sp.hp);
                            }
                        }
                    }
                }
                self.spend();
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.won() {
            Some(Outcome::Win(Player::One))
        } else if self.lost() {
            Some(Outcome::Win(Player::Two))
        } else {
            None
        }
    }
    fn eval(&self, p: Player) -> i32 {
        let mut v = self.pos as i32 * 60 + self.hp[0].max(0) * 2 + self.hp[1].max(0) * 2;
        if let Some(e) = &self.enc {
            v += e.prog * 5 - e.power;
        }
        if p == Player::One {
            v
        } else {
            -v
        }
    }
    fn show(&self) -> String {
        let mut s = format!(
            "{} — etapa {}/{} | ceas {}\n",
            self.sp.title,
            (self.pos + 1).min(self.sp.steps + 1),
            self.sp.steps + 1,
            self.clock
        );
        match &self.enc {
            Some(e) => s.push_str(&format!(
                "Întâlnire: {} putere {} (progres {})\n",
                self.sp.kinds[e.kind], e.power, e.prog
            )),
            None => s.push_str("Întâlnirea a fost depășită; urmează `avans`.\n"),
        }
        for h in 0..2 {
            let c: Vec<String> = self.hands[h]
                .iter()
                .map(|&(k, v)| format!("{}{}", self.sp.kinds[k].chars().next().unwrap(), v))
                .collect();
            s.push_str(&format!(
                "{}: viață {} | cărți {}\n",
                self.sp.heroes[h],
                self.hp[h].max(0),
                c.join(" ")
            ));
        }
        s.push_str(&format!(
            "La rând: {} ({} acțiuni) — joc k | odihna | pas\n",
            self.sp.heroes[self.hero], self.ap
        ));
        s
    }
}
