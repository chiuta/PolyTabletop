#![allow(
    clippy::needless_range_loop,
    clippy::manual_is_multiple_of,
    clippy::collapsible_if
)]
//! Motor cooperativ „sincron fără cuvinte” (lista C1, 2 jucători, variantă originală).
//!
//! Fiecare nivel dă fiecărui jucător câte `L` cărți (numere unice 1..`top`), ascunse adversarului.
//! Fără a vorbi, cărțile trebuie jucate în ordine crescătoare, de oricare jucător. Pe rând:
//! `joc` (joci cea mai mică carte a ta) sau `astept` (lași ceasul comun să avanseze cu 1). Dacă
//! cineva joacă o carte mai mare decât cea mai mică a celuilalt: o viață se pierde și toate cărțile
//! mai mici (din ambele mâini) se aruncă la vedere. Mână goală pentru ambii = nivel trecut; la
//! ultimul nivel, victorie; fără vieți: eșec (REMIZĂ). Verdictul vine doar prin mutarea forțată `nivel`.
//! Informație ascunsă: fiecare vede doar propria mână (HOTSEAT_SAFE). AI-ul joacă după ceasul comun
//! și propria carte (cu o mică neliniște deterministă), fără a citi mâna celuilalt.
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

pub struct MSpec {
    pub title: &'static str,
    pub levels: usize,
    pub lives: i32,
    pub top: usize,
}

#[derive(Clone)]
pub struct Mind {
    sp: &'static MSpec,
    hand: [Vec<usize>; 2],
    tick: usize,
    lives: i32,
    level: usize,
    side: Player,
    judge: bool,
    played: Vec<usize>,
    burned: Vec<usize>,
    last: Option<(usize, usize)>,
    end: Option<Outcome>,
    viewer: Option<Player>,
    rng: Rng,
}

impl Mind {
    pub fn new(sp: &'static MSpec) -> Self {
        Self::with_seed(sp, seed_now())
    }
    pub fn with_seed(sp: &'static MSpec, seed: u64) -> Self {
        let mut g = Mind {
            sp,
            hand: [Vec::new(), Vec::new()],
            tick: 0,
            lives: sp.lives,
            level: 1,
            side: Player::One,
            judge: false,
            played: Vec::new(),
            burned: Vec::new(),
            last: None,
            end: None,
            viewer: None,
            rng: Rng::new(seed ^ 0x7f4a_7c15),
        };
        g.deal();
        g
    }
    fn deal(&mut self) {
        let mut pool: Vec<usize> = (1..=self.sp.top).collect();
        shuf(&mut pool, &mut self.rng);
        for p in 0..2 {
            let mut h: Vec<usize> = pool.drain(..self.level).collect();
            h.sort();
            self.hand[p] = h;
        }
        self.tick = 0;
        self.played.clear();
        self.burned.clear();
        self.last = None;
        self.side = Player::One;
        self.fix_side();
    }
    fn fix_side(&mut self) {
        if self.hand[pi(self.side)].is_empty() && !self.hand[1 - pi(self.side)].is_empty() {
            self.side = self.side.other();
        }
    }
}

impl Mini for Mind {
    fn side(&self) -> Player {
        self.side
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        if self.judge {
            return vec!["nivel".into()];
        }
        let mut v = vec!["joc".to_string()];
        if self.tick < self.sp.top {
            v.push("astept".into());
        }
        v
    }
    fn step(&mut self, m: &str) {
        let me = pi(self.side);
        if m == "nivel" {
            self.judge = false;
            if self.lives <= 0 {
                self.end = Some(Outcome::Draw);
            } else if self.hand[0].is_empty() && self.hand[1].is_empty() {
                if self.level >= self.sp.levels {
                    self.end = Some(Outcome::Win(Player::One));
                } else {
                    self.level += 1;
                    self.deal();
                }
            }
            return;
        }
        if m == "astept" {
            self.tick += 1;
            self.last = None;
            self.side = self.side.other();
            self.fix_side();
            return;
        }
        let c = self.hand[me].remove(0);
        self.played.push(c);
        self.last = Some((me, c));
        let mut bad = false;
        for p in 0..2 {
            while !self.hand[p].is_empty() && self.hand[p][0] < c {
                let b = self.hand[p].remove(0);
                self.burned.push(b);
                bad = true;
            }
        }
        if bad {
            self.lives -= 1;
        }
        if self.lives <= 0 || (self.hand[0].is_empty() && self.hand[1].is_empty()) {
            self.judge = true;
        } else {
            self.side = self.side.other();
            self.fix_side();
        }
    }
    fn result(&self) -> Option<Outcome> {
        self.end
    }
    fn eval(&self, p: Player) -> i32 {
        // Doar propria mână + ceasul comun. După `joc` cartea tocmai jucată e în `last`.
        let a = pi(p);
        let mut s = 1000 * self.level as i32 + 300 * self.lives;
        if let Some((who, c)) = self.last {
            if who == a {
                let nerv = ((c * 7 + self.level * 3 + self.lives as usize) % 5) as i32 - 2;
                let gap = c as i32 - self.tick as i32 - nerv;
                s += 60 - 12 * gap.clamp(-3, 20);
                return s;
            }
        }
        s
    }
    fn show(&self) -> String {
        let me = pi(self.viewer.unwrap_or(self.side));
        let mut s = format!(
            "{} — nivelul {}/{} | vieți {} | ceas {}/{}\n",
            self.sp.title, self.level, self.sp.levels, self.lives, self.tick, self.sp.top
        );
        let h: Vec<String> = self.hand[me].iter().map(|c| c.to_string()).collect();
        s.push_str(&format!(
            "mâna lui J{} (ascunsă adversarului): {}\n",
            me + 1,
            if h.is_empty() {
                "—".into()
            } else {
                h.join(" ")
            }
        ));
        s.push_str(&format!(
            "cărți rămase la celălalt: {}\n",
            self.hand[1 - me].len()
        ));
        s.push_str(&format!(
            "jucate: {:?} | aruncate la vedere: {:?}\n",
            self.played, self.burned
        ));
        s.push_str(&format!(
            "La mutare: {} (joc | astept{})\n",
            pname(self.side),
            if self.judge { " | nivel" } else { "" }
        ));
        s
    }
    fn view(&mut self, w: Option<Player>) {
        self.viewer = w;
    }
}

#[cfg(test)]
impl Mind {
    pub fn scramble(&mut self, hide: usize, seed: u64) {
        let mut rng = Rng::new(seed);
        let n = self.hand[hide].len();
        let mut pool: Vec<usize> = (1..=self.sp.top)
            .filter(|c| {
                !self.hand[1 - hide].contains(c)
                    && !self.played.contains(c)
                    && !self.burned.contains(c)
            })
            .collect();
        shuf(&mut pool, &mut rng);
        let mut h: Vec<usize> = pool.into_iter().take(n).collect();
        h.sort();
        self.hand[hide] = h;
    }
}
