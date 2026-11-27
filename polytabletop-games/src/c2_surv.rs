//! Motor cooperativ „supraviețuire în colonie” (lista C2): echipa (J1, doi supraviețuitori pe rând)
//! adună resurse, întreține oamenii și duce un proiect la capăt; J2 este Jocul (forțat `eveniment`:
//! hrănire și criza anunțată public). Câștigă J1 = proiect terminat; câștigă J2 = colonia cade.
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

pub struct VSpec {
    pub title: &'static str,
    pub heroes: [&'static str; 2],
    pub res: [&'static str; 3],
    pub locs: [(&'static str, usize, i32); 4],
    pub pop: i32,
    pub goal: i32,
    pub rounds: u32,
    pub morale: i32,
    pub crises: &'static [(usize, i32, i32)],
    pub project: &'static str,
}

#[derive(Clone)]
pub struct V {
    sp: &'static VSpec,
    stock: [i32; 3],
    morale: i32,
    pop: i32,
    prog: i32,
    round: u32,
    used: [bool; 4],
    next: usize,
    hero: usize,
    ap: u8,
    side: Player,
    rng: Rng,
}

impl V {
    pub fn new(sp: &'static VSpec) -> Self {
        Self::with_seed(sp, seed_now())
    }
    pub fn with_seed(sp: &'static VSpec, seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let next = rng.below(sp.crises.len());
        V {
            sp,
            stock: [4, 2, 2],
            morale: sp.morale,
            pop: sp.pop,
            prog: 0,
            round: 1,
            used: [false; 4],
            next,
            hero: 0,
            ap: 2,
            side: Player::One,
            rng,
        }
    }
    fn next_hero(&mut self) {
        self.ap = 2;
        if self.hero == 0 {
            self.hero = 1;
        } else {
            self.hero = 0;
            self.side = Player::Two;
        }
    }
    fn spend(&mut self) {
        self.ap -= 1;
        if self.ap == 0 {
            self.next_hero();
        }
    }
}

impl Mini for V {
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
        let mut v = Vec::new();
        for i in 0..4 {
            if !self.used[i] {
                v.push(format!("cauta {}", i));
            }
        }
        if self.stock[1] >= 2 {
            v.push("lucru".into());
        }
        if self.stock[0] >= 1 && self.morale < self.sp.morale {
            v.push("ingrij".into());
        }
        v.push("pas".into());
        v
    }
    fn step(&mut self, m: &str) {
        match m {
            "eveniment" => {
                let need = (self.pop + 1) / 2;
                if self.stock[0] >= need {
                    self.stock[0] -= need;
                } else {
                    self.stock[0] = 0;
                    self.pop -= 1;
                    self.morale -= 1;
                }
                let (r, n, pen) = self.sp.crises[self.next];
                if self.stock[r] >= n {
                    self.stock[r] -= n;
                } else {
                    self.morale -= pen;
                    if pen >= 2 {
                        self.pop -= 1;
                    }
                }
                self.next = self.rng.below(self.sp.crises.len());
                self.used = [false; 4];
                self.round += 1;
                self.hero = 0;
                self.ap = 2;
                self.side = Player::One;
            }
            "pas" => self.next_hero(),
            "lucru" => {
                self.stock[1] -= 2;
                self.prog += 1;
                self.spend();
            }
            "ingrij" => {
                self.stock[0] -= 1;
                self.morale += 1;
                self.spend();
            }
            _ => {
                let i: usize = m[6..].parse().unwrap();
                self.used[i] = true;
                let (_, r, y) = self.sp.locs[i];
                self.stock[r] += y;
                self.spend();
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.prog >= self.sp.goal {
            Some(Outcome::Win(Player::One))
        } else if self.morale <= 0 || self.pop <= 0 || self.round > self.sp.rounds {
            Some(Outcome::Win(Player::Two))
        } else {
            None
        }
    }
    fn eval(&self, p: Player) -> i32 {
        let (r, n, pen) = self.sp.crises[self.next];
        let mut v = self.prog * 14 + self.morale * 4 + self.pop * 5;
        v += self.stock[0].min(self.pop) * 2 + self.stock[1].min(4) + self.stock[2].min(3);
        if self.stock[r] >= n {
            v += pen * 2;
        }
        if p == Player::One {
            v
        } else {
            -v
        }
    }
    fn show(&self) -> String {
        let (r, n, pen) = self.sp.crises[self.next];
        let mut s = format!(
            "{} — runda {}/{} | oameni {} | moral {}\n",
            self.sp.title, self.round, self.sp.rounds, self.pop, self.morale
        );
        s.push_str(&format!(
            "Resurse: {} {} | {} {} | {} {}\n{}: {}/{}\n",
            self.sp.res[0],
            self.stock[0],
            self.sp.res[1],
            self.stock[1],
            self.sp.res[2],
            self.stock[2],
            self.sp.project,
            self.prog,
            self.sp.goal
        ));
        s.push_str(&format!(
            "Criza anunțată: cere {} {} (altfel moral -{})\n",
            n, self.sp.res[r], pen
        ));
        for i in 0..4 {
            let (nm, r, y) = self.sp.locs[i];
            s.push_str(&format!(
                "  [{}] {} (+{} {}){}\n",
                i,
                nm,
                y,
                self.sp.res[r],
                if self.used[i] { " — folosit" } else { "" }
            ));
        }
        s.push_str(&format!(
            "La rând: {} ({} acțiuni) — cauta i | lucru (2 {} → +1 proiect) | ingrij (1 {} → +1 moral) | pas\n",
            self.sp.heroes[self.hero], self.ap, self.sp.res[1], self.sp.res[0]
        ));
        s
    }
}
