//! Motor cooperativ „asediu pe culoare” (lista C2): echipa (J1, două personaje jucate pe rând)
//! apără o bază împotriva unor amenințări care înaintează pe culoare; J2 este Jocul (mutare
//! forțată `eveniment`: înaintare, lovituri, apariții). Câștigă J1 = echipa; câștigă J2 = Jocul.
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

pub struct SSpec {
    pub title: &'static str,
    pub lanes: &'static [&'static str],
    pub heroes: [&'static str; 2],
    pub start: i32,
    pub base: i32,
    pub rounds: u32,
    pub boss_hp: i32,
    pub kill_goal: u32,
    pub spawn: usize,
    pub hand: usize,
    pub bonus: [i32; 2],
    pub wall: bool,
    pub heal: bool,
    pub freeze: bool,
    pub speed: i32,
}

#[derive(Clone)]
struct T {
    hp: i32,
    d: i32,
    boss: bool,
}

#[derive(Clone)]
pub struct S {
    sp: &'static SSpec,
    lanes: Vec<Vec<T>>,
    wall: Vec<i32>,
    frozen: Vec<bool>,
    hands: [Vec<i32>; 2],
    deck: Vec<i32>,
    base: i32,
    round: u32,
    kills: u32,
    boss_dead: bool,
    hero: usize,
    ap: u8,
    side: Player,
    rng: Rng,
}

impl S {
    pub fn new(sp: &'static SSpec) -> Self {
        Self::with_seed(sp, seed_now())
    }
    pub fn with_seed(sp: &'static SSpec, seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck: Vec<i32> = (0..30).map(|i| (i % 5) + 1).collect();
        shuf(&mut deck, &mut rng);
        let n = sp.lanes.len();
        let mut s = S {
            sp,
            lanes: vec![Vec::new(); n],
            wall: vec![0; n],
            frozen: vec![false; n],
            hands: [Vec::new(), Vec::new()],
            deck,
            base: sp.base,
            round: 1,
            kills: 0,
            boss_dead: sp.boss_hp == 0,
            hero: 0,
            ap: 2,
            side: Player::One,
            rng,
        };
        s.refill();
        let l0 = s.rng.below(n);
        s.lanes[l0].push(T {
            hp: 2,
            d: sp.start - 1,
            boss: false,
        });
        s
    }
    fn draw(&mut self) -> i32 {
        if self.deck.is_empty() {
            self.deck = (0..30).map(|i| (i % 5) + 1).collect();
            let mut r = self.rng.clone();
            shuf(&mut self.deck, &mut r);
            self.rng = r;
        }
        self.deck.pop().unwrap()
    }
    fn refill(&mut self) {
        for h in 0..2 {
            while self.hands[h].len() < self.sp.hand {
                let c = self.draw();
                self.hands[h].push(c);
            }
        }
    }
    fn won(&self) -> bool {
        if self.sp.kill_goal > 0 {
            self.kills >= self.sp.kill_goal
        } else {
            self.round > self.sp.rounds && self.boss_dead
        }
    }
    fn lost(&self) -> bool {
        self.base <= 0 || self.round > self.sp.rounds + 8
    }
    fn front(&self, l: usize) -> Option<usize> {
        (0..self.lanes[l].len()).min_by_key(|&i| self.lanes[l][i].d)
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
    fn event(&mut self) {
        let sp = self.sp;
        for l in 0..self.lanes.len() {
            if self.frozen[l] {
                self.frozen[l] = false;
            } else {
                for t in self.lanes[l].iter_mut() {
                    t.d -= sp.speed;
                }
            }
            let mut i = 0;
            while i < self.lanes[l].len() {
                if self.lanes[l][i].d <= 0 {
                    let t = self.lanes[l].remove(i);
                    let mut dmg = (t.hp + 1) / 2;
                    if t.boss {
                        dmg += 2;
                    }
                    let ab = dmg.min(self.wall[l]);
                    self.wall[l] -= ab;
                    self.base -= dmg - ab;
                } else {
                    i += 1;
                }
            }
        }
        for l in 0..self.lanes.len() {
            for _ in 0..2 {
                if self.lanes[l].len() >= 6 || self.rng.below(100) >= sp.spawn {
                    continue;
                }
                let hp = 2 + self.round as i32 + self.rng.below(3) as i32;
                self.lanes[l].push(T {
                    hp,
                    d: sp.start,
                    boss: false,
                });
            }
        }
        if sp.boss_hp > 0 && self.round == (sp.rounds / 2).max(1) {
            let l = self.rng.below(self.lanes.len());
            self.lanes[l].push(T {
                hp: sp.boss_hp,
                d: sp.start,
                boss: true,
            });
        }
        self.round += 1;
        self.refill();
        self.hero = 0;
        self.ap = 2;
        self.side = Player::One;
    }
}

impl Mini for S {
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
        let h = &self.hands[self.hero];
        for (k, &c) in h.iter().enumerate() {
            if h[..k].contains(&c) {
                continue;
            }
            for l in 0..self.lanes.len() {
                if !self.lanes[l].is_empty() {
                    v.push(format!("atac {} {}", l, k));
                    if self.sp.freeze && !self.frozen[l] {
                        v.push(format!("ingh {} {}", l, k));
                    }
                }
                if self.sp.wall {
                    v.push(format!("zid {} {}", l, k));
                }
            }
            if self.sp.heal && self.base < self.sp.base {
                v.push(format!("cur {}", k));
            }
        }
        v.push("pas".into());
        v
    }
    fn step(&mut self, m: &str) {
        if m == "eveniment" {
            self.event();
            return;
        }
        let mut it = m.split(' ');
        let cmd = it.next().unwrap();
        let a: Vec<usize> = it.map(|x| x.parse().unwrap()).collect();
        match cmd {
            "pas" => {
                self.next_hero();
            }
            "atac" => {
                let (l, k) = (a[0], a[1]);
                let v = self.hands[self.hero].remove(k) + self.sp.bonus[self.hero];
                if let Some(i) = self.front(l) {
                    self.lanes[l][i].hp -= v;
                    if self.lanes[l][i].hp <= 0 {
                        let t = self.lanes[l].remove(i);
                        self.kills += 1;
                        if t.boss {
                            self.boss_dead = true;
                        }
                    }
                }
                self.spend();
            }
            "ingh" => {
                self.hands[self.hero].remove(a[1]);
                self.frozen[a[0]] = true;
                self.spend();
            }
            "zid" => {
                let v = self.hands[self.hero].remove(a[1]);
                self.wall[a[0]] += v;
                self.spend();
            }
            _ => {
                self.hands[self.hero].remove(a[0]);
                self.base = (self.base + 2).min(self.sp.base);
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
        let mut v = self.base * 6 + self.kills as i32 * 5;
        v += self.wall.iter().sum::<i32>();
        for l in &self.lanes {
            for t in l {
                v -= t.hp * (self.sp.start + 2 - t.d);
            }
        }
        for (l, f) in self.frozen.iter().enumerate() {
            if *f {
                v += self.lanes[l].len() as i32 * 2;
            }
        }
        if p == Player::One {
            v
        } else {
            -v
        }
    }
    fn show(&self) -> String {
        let mut s = format!(
            "{} — runda {}/{} | bază {} | eliminate {}\n",
            self.sp.title, self.round, self.sp.rounds, self.base, self.kills
        );
        for (l, name) in self.sp.lanes.iter().enumerate() {
            let ts: Vec<String> = self.lanes[l]
                .iter()
                .map(|t| format!("{}{}@{}", if t.boss { "!" } else { "" }, t.hp, t.d))
                .collect();
            s.push_str(&format!(
                "  [{}] {} — zid {}{} | {}\n",
                l,
                name,
                self.wall[l],
                if self.frozen[l] { " | înghețat" } else { "" },
                if ts.is_empty() {
                    "-".into()
                } else {
                    ts.join(" ")
                }
            ));
        }
        for h in 0..2 {
            let c: Vec<String> = self.hands[h].iter().map(|x| x.to_string()).collect();
            s.push_str(&format!("{}: cărți {}\n", self.sp.heroes[h], c.join(" ")));
        }
        s.push_str(&format!(
            "La rând: {} ({} acțiuni) — atac L k | zid L k | ingh L k | cur k | pas\n",
            self.sp.heroes[self.hero], self.ap
        ));
        s
    }
}
