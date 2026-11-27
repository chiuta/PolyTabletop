//! Intrigi de Palat — influență, bluf și acuzații (variantă originală, 2 jucători).
//!
//! Patru roluri, câte trei exemplare (12 cărți): 0 Vistier, 1 Călău, 2 Hoț, 3 Paznic. Fiecare jucător are 2 cărți
//! ascunse (influență) și 2 monede. Pe rând alegi o acțiune: `venit` (+1 monedă); `taxa` (anunți Vistier: +3);
//! `calau` (anunți Călău, plătești 3: adversarul pierde o influență); `hot` (anunți Hoț: furi 2 monede);
//! `lovitura` (7 monede, fără anunț: adversarul pierde o influență; obligatorie de la 10 monede).
//! Adversarul poate `accept`, `provoc` (acuză că anunțul e fals) sau `bloc R` (anunță Paznic împotriva Călăului sau
//! Paznic/Hoț împotriva furtului). Cartea unei acuzații se verifică la mutarea forțată `arata`: cine a mințit pierde
//! o influență (`pierd i` alege cartea); cine a acuzat greșit pierde una, iar cartea dovedită se schimbă cu pachetul.
//! Pierde cine rămâne fără influență. Informație ascunsă: cărțile; AI-ul nu o citește pe a adversarului.
use crate::b6_util::pl;
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const NAMES: [&str; 4] = ["Vistier", "Călău", "Hoț", "Paznic"];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Act {
    Taxa,
    Calau,
    Hot,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ph {
    Act,
    Resp(Act),
    RevealAct(Act),
    BlockResp(Act, usize),
    RevealBlock(Act, usize),
    /// Jucătorul trebuie să piardă o influență; după aceea: continuare.
    Lose(usize, Cont),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Cont {
    Next,
    Do(Act),
}

fn claim_role(a: Act) -> usize {
    match a {
        Act::Taxa => 0,
        Act::Calau => 1,
        Act::Hot => 2,
    }
}

#[derive(Clone)]
pub struct Palat {
    viewer: Option<Player>,
    alive: [Vec<u8>; 2],
    dead: [Vec<u8>; 2],
    coins: [i32; 2],
    deck: Vec<u8>,
    mover: usize,
    ph: Ph,
    plies: u32,
    rng: Rng,
}

impl Palat {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck: Vec<u8> = (0..12).map(|i| (i % 4) as u8).collect();
        shuf(&mut deck, &mut rng);
        let a = vec![deck.pop().unwrap(), deck.pop().unwrap()];
        let b = vec![deck.pop().unwrap(), deck.pop().unwrap()];
        Palat {
            viewer: None,
            alive: [a, b],
            dead: [Vec::new(), Vec::new()],
            coins: [2, 2],
            deck,
            mover: 0,
            ph: Ph::Act,
            plies: 0,
            rng,
        }
    }
    /// Probabilitatea (sutimi) ca adversarul lui `me` să aibă rolul `r`, din punctul de vedere al lui `me`.
    fn p_opp(&self, me: usize, r: usize) -> i32 {
        let o = 1 - me;
        let seen_r = self.alive[me].iter().filter(|&&c| c as usize == r).count()
            + self.dead[0]
                .iter()
                .chain(self.dead[1].iter())
                .filter(|&&c| c as usize == r)
                .count();
        let unseen_r = 3 - seen_r as i32;
        let unseen =
            12 - self.alive[me].len() as i32 - (self.dead[0].len() + self.dead[1].len()) as i32;
        if unseen_r <= 0 || unseen <= 0 {
            return 0;
        }
        (100 * unseen_r * self.alive[o].len() as i32 / unseen).min(90)
    }
    /// Cât de probabil e ca un anunț al lui `claimer` să fie adevărat (sutimi), din partea lui `me`.
    fn p_true(&self, me: usize, r: usize) -> i32 {
        let ph = self.p_opp(me, r);
        (30 + 70 * ph / 100).min(95)
    }
    fn lose_idx(&mut self, p: usize, i: usize) {
        let c = self.alive[p].remove(i);
        self.dead[p].push(c);
    }
    fn next_turn(&mut self) {
        self.mover = 1 - self.mover;
        self.ph = Ph::Act;
        self.plies += 1;
    }
    fn do_act(&mut self, a: Act) -> Option<usize> {
        let o = 1 - self.mover;
        match a {
            Act::Taxa => {
                self.coins[self.mover] += 3;
                None
            }
            Act::Hot => {
                let t = self.coins[o].min(2);
                self.coins[o] -= t;
                self.coins[self.mover] += t;
                None
            }
            Act::Calau => Some(o),
        }
    }
    fn after_lose(&mut self, cont: Cont) {
        if self.alive[0].is_empty() || self.alive[1].is_empty() {
            return;
        }
        match cont {
            Cont::Next => self.next_turn(),
            Cont::Do(a) => {
                if let Some(t) = self.do_act(a) {
                    self.ph = Ph::Lose(t, Cont::Next);
                } else {
                    self.next_turn();
                }
            }
        }
    }
    fn swap_proven(&mut self, p: usize, r: usize) {
        if let Some(i) = self.alive[p].iter().position(|&c| c as usize == r) {
            let c = self.alive[p].remove(i);
            self.deck.push(c);
            shuf(&mut self.deck, &mut self.rng);
            let n = self.deck.pop().unwrap();
            self.alive[p].push(n);
        }
    }
}

impl Default for Palat {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Palat {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        let o = 1 - self.mover;
        pl(match self.ph {
            Ph::Act => self.mover,
            Ph::Resp(_) => o,
            Ph::RevealAct(_) => self.mover,
            Ph::BlockResp(..) => self.mover,
            Ph::RevealBlock(_, _) => o,
            Ph::Lose(p, _) => p,
        })
    }
    fn moves(&self) -> Vec<String> {
        let me = self.mover;
        match self.ph {
            Ph::Act => {
                if self.coins[me] >= 10 {
                    return vec!["lovitura".into()];
                }
                let mut v = vec!["venit".to_string(), "taxa".into(), "hot".into()];
                if self.coins[me] >= 3 {
                    v.push("calau".into());
                }
                if self.coins[me] >= 7 {
                    v.push("lovitura".into());
                }
                v
            }
            Ph::Resp(a) => {
                let mut v = vec!["accept".to_string(), "provoc".into()];
                match a {
                    Act::Calau => v.push("bloc 3".into()),
                    Act::Hot => {
                        v.push("bloc 3".into());
                        v.push("bloc 2".into());
                    }
                    Act::Taxa => {}
                }
                v
            }
            Ph::BlockResp(..) => vec!["accept".into(), "provoc".into()],
            Ph::RevealAct(_) | Ph::RevealBlock(..) => vec!["arata".into()],
            Ph::Lose(p, _) => (0..self.alive[p].len())
                .map(|i| format!("pierd {}", i + 1))
                .collect(),
        }
    }
    fn step(&mut self, m: &str) {
        let me = self.mover;
        let o = 1 - me;
        match self.ph {
            Ph::Act => match m {
                "venit" => {
                    self.coins[me] += 1;
                    self.next_turn();
                }
                "taxa" => self.ph = Ph::Resp(Act::Taxa),
                "hot" => self.ph = Ph::Resp(Act::Hot),
                "calau" => {
                    self.coins[me] -= 3;
                    self.ph = Ph::Resp(Act::Calau);
                }
                _ => {
                    self.coins[me] -= 7;
                    self.ph = Ph::Lose(o, Cont::Next);
                }
            },
            Ph::Resp(a) => {
                if m == "accept" {
                    if let Some(t) = self.do_act(a) {
                        self.ph = Ph::Lose(t, Cont::Next);
                    } else {
                        self.next_turn();
                    }
                } else if m == "provoc" {
                    self.ph = Ph::RevealAct(a);
                } else {
                    self.ph = Ph::BlockResp(a, m[5..].parse().unwrap());
                }
            }
            Ph::RevealAct(a) => {
                let r = claim_role(a);
                if self.alive[me].iter().any(|&c| c as usize == r) {
                    self.swap_proven(me, r);
                    self.ph = Ph::Lose(o, Cont::Do(a));
                } else {
                    self.ph = Ph::Lose(me, Cont::Next);
                }
            }
            Ph::BlockResp(a, r) => {
                if m == "accept" {
                    self.next_turn();
                } else {
                    self.ph = Ph::RevealBlock(a, r);
                }
            }
            Ph::RevealBlock(a, r) => {
                // cel care a blocat e `o`
                if self.alive[o].iter().any(|&c| c as usize == r) {
                    self.swap_proven(o, r);
                    self.ph = Ph::Lose(me, Cont::Next);
                } else {
                    self.ph = Ph::Lose(o, Cont::Do(a));
                }
            }
            Ph::Lose(p, cont) => {
                let i = m[6..].parse::<usize>().unwrap() - 1;
                self.lose_idx(p, i);
                self.after_lose(cont);
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.alive[0].is_empty() {
            return Some(Outcome::Win(Player::Two));
        }
        if self.alive[1].is_empty() {
            return Some(Outcome::Win(Player::One));
        }
        if self.plies >= 80 {
            let s = |p: usize| 1000 * self.alive[p].len() as i32 + self.coins[p];
            return Some(by_score(s(0), s(1)));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let o = 1 - a;
        let infl = |q: usize| 700 * self.alive[q].len() as i32;
        let base = infl(a) - infl(o) + 100 * (self.coins[a] - self.coins[o]);
        let holds = |r: usize| self.alive[a].iter().any(|&c| c as usize == r);
        match self.ph {
            // am anunțat o acțiune: adversarul acuză cu ~35% (dacă am rolul nu pierd nimic, altfel pierd o influență)
            Ph::Resp(act) if a == self.mover => {
                let r = claim_role(act);
                let gain = match act {
                    Act::Taxa => 300,
                    Act::Calau => 700 + 300, // costul a fost deja scăzut din monede
                    Act::Hot => 400,
                };
                let bluff = if holds(r) { 0 } else { 35 * 700 / 100 };
                base + gain * 65 / 100 - bluff
            }
            // am acuzat anunțul lui r: dacă e fals câștig o influență
            Ph::RevealAct(act) if a != self.mover => {
                let pt = self.p_true(a, claim_role(act));
                base + ((100 - pt) * 700 - pt * 700) / 100
            }
            // am blocat cu rolul r
            Ph::BlockResp(act, r) if a != self.mover => {
                let bluff = if holds(r) { 0 } else { 40 * 700 / 100 };
                base + match act {
                    Act::Calau => 700,
                    _ => 300,
                } * 60
                    / 100
                    - bluff
            }
            // am acuzat blocajul lui r
            Ph::RevealBlock(_, r) if a == self.mover => {
                let pt = self.p_true(a, r);
                base + ((100 - pt) * 700 - pt * 700) / 100
            }
            _ => base,
        }
    }
    fn show(&self) -> String {
        let viewer = self.viewer.map(pi).unwrap_or(pi(self.side()));
        let names = |v: &Vec<u8>| {
            if v.is_empty() {
                "-".to_string()
            } else {
                v.iter()
                    .map(|&c| NAMES[c as usize])
                    .collect::<Vec<_>>()
                    .join(", ")
            }
        };
        format!(
            "Intrigi de Palat | monede J1 {}, J2 {} | influență J1 {}, J2 {}\nCărți pierdute (la vedere): J1 [{}], J2 [{}]\nCărțile tale (J{}): {}\nFaza: {:?} | la mutare: {} (venit | taxa | calau | hot | lovitura | accept | provoc | bloc R | arata | pierd i)\n",
            self.coins[0],
            self.coins[1],
            self.alive[0].len(),
            self.alive[1].len(),
            names(&self.dead[0]),
            names(&self.dead[1]),
            viewer + 1,
            names(&self.alive[viewer]),
            self.ph,
            pname(self.side())
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
        crate::brand_util::check(Palat::with_seed, 400);
    }
    #[test]
    fn ai_ignores_the_opponents_cards() {
        let g = Palat::with_seed(5);
        let mut h = g.clone();
        h.alive[1] = vec![3, 3];
        let mut g2 = g.clone();
        g2.step("taxa");
        let mut h2 = g2.clone();
        h2.alive[0] = vec![3, 3];
        for seed in 1..6 {
            assert_eq!(ai_of(&g, seed), ai_of(&h, seed));
            assert_eq!(ai_of(&g2, seed), ai_of(&h2, seed));
        }
        let mut v = g.clone();
        v.view(Some(Player::Two));
        let s = v.show();
        assert!(s.contains("Cărțile tale (J2)") && !s.contains("Cărțile tale (J1)"));
    }
}

#[cfg(test)]
mod a2_audit {
    use super::*;

    #[test]
    fn ai_nu_vede_cartile_adversarului_nici_pachetul_la_orice_nivel() {
        crate::p3_comun::ai_blind_check(
            "palat_intrigi",
            |s| B(Palat::with_seed(s)),
            |g, me, seed| {
                let p = &mut g.0;
                let (m, o) = (me.idx(), 1 - me.idx());
                let mut pool: Vec<u8> = Vec::new();
                for r in 0..4u8 {
                    let seen = p.alive[m].iter().filter(|&&c| c == r).count()
                        + p.dead[0]
                            .iter()
                            .chain(p.dead[1].iter())
                            .filter(|&&c| c == r)
                            .count();
                    for _ in 0..3usize.saturating_sub(seen) {
                        pool.push(r);
                    }
                }
                crate::p3_comun::shuffle_with(&mut pool, seed);
                let n = p.alive[o].len();
                p.alive[o] = pool.drain(..n).collect();
                p.deck = pool;
                p.rng = Rng::new(seed ^ 0x99);
            },
            12,
        );
    }
}
