#![allow(
    clippy::needless_range_loop,
    clippy::manual_is_multiple_of,
    clippy::collapsible_if,
    clippy::too_many_arguments,
    clippy::type_complexity
)]
//! Motor cooperativ „echipa contra adversarului” (lista C1, 2 jucători, variante originale).
//!
//! Fiecare jucător are un pachet propriu și o mână deschisă (jocul e cooperativ: mâinile sunt vizibile
//! echipei; doar ordinea pachetelor și a pachetului adversarului e necunoscută). La tura ta joci cel
//! mult `plays` cărți (`joc i`) sau `fin`; apoi tragi până la mărimea mâinii. După ambii jucători,
//! mutarea forțată `ev` joacă o carte a adversarului (atac sau complot).
//!
//! Cărți: `a` atac (lovește adversarul; în modul cu progres reduce destinul), `d` apărare (reduce
//! atacul rundei), `h` vindecare (jucătorul cu viața mai mică), `p` progres, `s` amețire (adversarul
//! nu atacă), `c` încărcare (mana +v). Steaguri: `e` = epuizare (pachetul nu se amestecă singur; `odih`
//! amestecă aruncările contra 1 viață; un jucător fără cărți e epuizat; ambii epuizați = eșec),
//! `m` = atacul cere 1 mana și lovește cu +2.
//! Victorie: adversarul la 0 (sau progresul atins, dacă `prog` > 0). Eșec (REMIZĂ): viață 0, destin la
//! maxim sau rundele epuizate. Verdictul se dă doar la mutarea forțată `ev`.
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

pub struct DSpec {
    pub title: &'static str,
    pub deck1: &'static str,
    pub deck2: &'static str,
    pub hand: usize,
    pub plays: u8,
    pub boss_hp: i32,
    pub atk: &'static [i32],
    pub scheme: &'static [i32],
    pub prog: i32,
    pub doom_max: i32,
    pub hp: i32,
    pub flags: &'static str,
    pub rounds: u32,
}

type Card = (char, i32);

fn parse(s: &str) -> Vec<Card> {
    s.split(',')
        .map(|t| {
            let mut c = t.chars();
            let k = c.next().unwrap();
            (k, c.as_str().parse().unwrap())
        })
        .collect()
}

#[derive(Clone)]
pub struct Duel {
    sp: &'static DSpec,
    boss: i32,
    prog: i32,
    doom: i32,
    hp: [i32; 2],
    hand: [Vec<Card>; 2],
    deck: [Vec<Card>; 2],
    disc: [Vec<Card>; 2],
    bdeck: Vec<(char, i32)>,
    mana: i32,
    shield: i32,
    stun: bool,
    round: u32,
    side: Player,
    left: u8,
    pend: bool,
    end: Option<Outcome>,
    rng: Rng,
}

impl Duel {
    pub fn new(sp: &'static DSpec) -> Self {
        Self::with_seed(sp, seed_now())
    }
    pub fn with_seed(sp: &'static DSpec, seed: u64) -> Self {
        let mut rng = Rng::new(seed ^ 0x51ed_270b);
        let mut deck = [parse(sp.deck1), parse(sp.deck2)];
        for p in 0..2 {
            shuf(&mut deck[p], &mut rng);
        }
        let mut bdeck = Vec::new();
        for i in 0..(sp.rounds as usize + 2) {
            if i % 3 == 2 && !sp.scheme.is_empty() {
                bdeck.push(('s', sp.scheme[rng.below(sp.scheme.len())]));
            } else {
                bdeck.push(('a', sp.atk[rng.below(sp.atk.len())]));
            }
        }
        let mut g = Duel {
            sp,
            boss: sp.boss_hp,
            prog: 0,
            doom: 0,
            hp: [sp.hp; 2],
            hand: [Vec::new(), Vec::new()],
            deck,
            disc: [Vec::new(), Vec::new()],
            bdeck,
            mana: 0,
            shield: 0,
            stun: false,
            round: 0,
            side: Player::One,
            left: sp.plays,
            pend: false,
            end: None,
            rng,
        };
        for p in 0..2 {
            g.draw(p);
        }
        g
    }
    fn ex(&self) -> bool {
        self.sp.flags.contains('e')
    }
    fn mana_mode(&self) -> bool {
        self.sp.flags.contains('m')
    }
    fn draw(&mut self, p: usize) {
        while self.hand[p].len() < self.sp.hand {
            if self.deck[p].is_empty() {
                if self.ex() || self.disc[p].is_empty() {
                    break;
                }
                let mut d = std::mem::take(&mut self.disc[p]);
                shuf(&mut d, &mut self.rng);
                self.deck[p] = d;
            }
            let c = self.deck[p].pop().unwrap();
            self.hand[p].push(c);
        }
    }
    fn exhausted(&self, p: usize) -> bool {
        self.ex() && self.hand[p].is_empty() && self.deck[p].is_empty() && self.disc[p].is_empty()
    }
    fn playable(&self, c: Card) -> bool {
        !(self.mana_mode() && c.0 == 'a' && self.mana < 1)
    }
    fn end_turn(&mut self) {
        let me = pi(self.side);
        self.draw(me);
        self.left = self.sp.plays;
        if self.side == Player::One {
            self.side = Player::Two;
        } else {
            self.pend = true;
        }
    }
    fn boss_act(&mut self) {
        let (k, v) = self.bdeck[self.round as usize % self.bdeck.len()];
        if k == 'a' {
            if self.stun {
                self.stun = false;
            } else {
                let dmg = (v - self.shield).max(0);
                let t = (self.round as usize + v as usize) % 2;
                self.hp[t] -= dmg;
            }
        } else {
            self.doom += v;
        }
        self.shield = 0;
        self.round += 1;
        self.side = Player::One;
        self.left = self.sp.plays;
        self.pend = false;
        let won = if self.sp.prog > 0 {
            self.prog >= self.sp.prog
        } else {
            self.boss <= 0
        };
        if won {
            self.end = Some(Outcome::Win(Player::One));
        } else if self.hp.iter().any(|&h| h <= 0)
            || self.doom >= self.sp.doom_max
            || self.round >= self.sp.rounds
            || (self.exhausted(0) && self.exhausted(1))
        {
            self.end = Some(Outcome::Draw);
        }
    }
}

impl Mini for Duel {
    fn side(&self) -> Player {
        self.side
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        if self.pend {
            return vec!["ev".into()];
        }
        let me = pi(self.side);
        let mut v = vec!["fin".to_string()];
        let mut seen: Vec<Card> = Vec::new();
        for (i, &c) in self.hand[me].iter().enumerate() {
            if self.playable(c) && !seen.contains(&c) {
                seen.push(c);
                v.push(format!("joc {}", i + 1));
            }
        }
        if self.ex() && !self.disc[me].is_empty() {
            v.push("odih".into());
        }
        v
    }
    fn step(&mut self, m: &str) {
        if m == "ev" {
            self.boss_act();
            return;
        }
        let me = pi(self.side);
        if m == "fin" {
            self.end_turn();
            return;
        }
        if m == "odih" {
            let mut d = std::mem::take(&mut self.disc[me]);
            self.deck[me].append(&mut d);
            let mut dk = std::mem::take(&mut self.deck[me]);
            shuf(&mut dk, &mut self.rng);
            self.deck[me] = dk;
            self.hp[me] -= 1;
            self.end_turn();
            return;
        }
        let i: usize = m[4..].parse::<usize>().unwrap() - 1;
        let c = self.hand[me].remove(i);
        self.disc[me].push(c);
        let (k, v) = c;
        match k {
            'a' => {
                let mut dmg = v;
                if self.mana_mode() {
                    self.mana -= 1;
                    dmg += 2;
                }
                if self.sp.prog > 0 {
                    self.doom = (self.doom - (dmg + 1) / 2).max(0);
                } else {
                    self.boss -= dmg;
                }
            }
            'd' => self.shield += v,
            'h' => {
                let t = if self.hp[0] <= self.hp[1] { 0 } else { 1 };
                self.hp[t] = (self.hp[t] + v).min(self.sp.hp);
            }
            'p' => self.prog += v,
            's' => self.stun = true,
            'c' => self.mana += v,
            _ => {}
        }
        self.left -= 1;
        if self.left == 0 || self.hand[me].is_empty() {
            self.end_turn();
        }
    }
    fn result(&self) -> Option<Outcome> {
        self.end
    }
    fn eval(&self, _p: Player) -> i32 {
        let mut s = 12 * (self.hp[0] + self.hp[1]) - 40 * self.doom;
        if self.sp.prog > 0 {
            s += 70 * self.prog;
        } else {
            s += 45 * (self.sp.boss_hp - self.boss);
        }
        s += 10 * self.shield + if self.stun { 25 } else { 0 } + 6 * self.mana;
        // pe linia vieții: nu lăsa un jucător să cadă
        for p in 0..2 {
            if self.hp[p] <= 3 {
                s -= 80;
            }
        }
        s
    }
    fn show(&self) -> String {
        let mut s = format!(
            "{} — runda {}/{} | viață J1 {} J2 {} | destin {}/{}",
            self.sp.title,
            self.round + 1,
            self.sp.rounds,
            self.hp[0],
            self.hp[1],
            self.doom,
            self.sp.doom_max
        );
        if self.sp.prog > 0 {
            s.push_str(&format!(" | progres {}/{}", self.prog, self.sp.prog));
        } else {
            s.push_str(&format!(" | adversar {}/{}", self.boss, self.sp.boss_hp));
        }
        s.push_str(&format!(
            " | scut {} | mana {}{}\n",
            self.shield,
            self.mana,
            if self.stun { " | adversar amețit" } else { "" }
        ));
        for p in 0..2 {
            let h: Vec<String> = self.hand[p]
                .iter()
                .enumerate()
                .map(|(i, c)| format!("{}:{}{}", i + 1, c.0, c.1))
                .collect();
            s.push_str(&format!(
                "mâna J{} (deschisă): {} | pachet {} aruncate {}\n",
                p + 1,
                h.join(" "),
                self.deck[p].len(),
                self.disc[p].len()
            ));
        }
        s.push_str(&format!(
            "La mutare: {} ({} cărți: joc i | fin{}; ev = adversarul)\n",
            pname(self.side),
            self.left,
            if self.ex() { " | odih" } else { "" }
        ));
        s
    }
}
