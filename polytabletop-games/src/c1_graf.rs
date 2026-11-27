#![allow(
    clippy::needless_range_loop,
    clippy::manual_is_multiple_of,
    clippy::collapsible_if,
    clippy::too_many_arguments,
    clippy::type_complexity
)]
//! Motor cooperativ „harta amenințărilor” (lista C1, 2 jucători, variante originale).
//!
//! Cei doi jucători formează o echipă împotriva jocului. Pe o hartă (graf) apar amenințări
//! (`hz`) prin mutarea forțată `ev` (zar ascuns, sfârșit de rundă). Echipa câștigă dacă livrează
//! `need` obiecte la ieșire (sau, în modul `a`, elimină `need` amenințări). Orice eșec = REMIZĂ
//! (convenția jocurilor cooperative din colecție: nu există adversar).
//!
//! Moduri de răspândire (`spread`):
//! * `s` — peste prag, nodul se scufundă (pierdut definitiv, obiectul de pe el dispare);
//! * `o` — peste prag, amenințarea „izbucnește”: +1 destin, vecinii primesc +1;
//! * `f` — la prag, focul se întinde la vecini; jucătorul de pe un nod la prag pierde 1 viață;
//! * `a` — amenințările înaintează câte un pas spre ieșire; ajunse acolo, cresc destinul;
//! * `w` — nisip: nodurile la prag blochează trecerea; în plus destinul crește în fiecare rundă.
//!
//! Mutări: `mov n`, `fix n` (curăță o amenințare din nodul curent sau vecin; în modul `a` o ucide;
//! lângă urmăritor îl alungă), `iau`, `dau`, `fin`; mutarea forțată `ev`.
//! Informație: totul e public, în afară de viitoarele extrageri (AI-ul decide fără ele).
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

pub struct GSpec {
    pub title: &'static str,
    pub n: usize,
    pub edges: &'static str,
    pub start: [usize; 2],
    pub exit: usize,
    pub items: &'static [usize],
    pub need: u8,
    pub cap: u8,
    pub per: u8,
    pub spread: char,
    pub doom_max: u8,
    pub rounds: u32,
    /// nodul de start al urmăritorului (n = fără urmăritor)
    pub pursuer: usize,
    /// doar jucătorul rundei (par/impar) poate lua obiecte
    pub rst: bool,
    pub hp: i8,
}

#[derive(Clone)]
pub struct G {
    sp: &'static GSpec,
    adj: Vec<Vec<usize>>,
    hz: Vec<u8>,
    lost: Vec<bool>,
    item: Vec<bool>,
    gone: u8,
    pos: [usize; 2],
    carry: [u8; 2],
    done: u8,
    doom: u8,
    hp: i8,
    pur: usize,
    round: u32,
    side: Player,
    ap: u8,
    pend: bool,
    end: Option<Outcome>,
    rng: Rng,
}

const AP: u8 = 4;

impl G {
    pub fn new(sp: &'static GSpec) -> Self {
        Self::with_seed(sp, seed_now())
    }
    pub fn with_seed(sp: &'static GSpec, seed: u64) -> Self {
        let mut adj = vec![Vec::new(); sp.n];
        for e in sp.edges.split(',') {
            let (a, b) = e.split_once('-').unwrap();
            let (a, b): (usize, usize) = (a.parse().unwrap(), b.parse().unwrap());
            adj[a].push(b);
            adj[b].push(a);
        }
        let mut item = vec![false; sp.n];
        for &i in sp.items {
            item[i] = true;
        }
        G {
            sp,
            adj,
            hz: vec![0; sp.n],
            lost: vec![false; sp.n],
            item,
            gone: 0,
            pos: sp.start,
            carry: [0; 2],
            done: 0,
            doom: 0,
            hp: sp.hp,
            pur: sp.pursuer,
            round: 0,
            side: Player::One,
            ap: AP,
            pend: false,
            end: None,
            rng: Rng::new(seed ^ 0x9e37_79b9),
        }
    }
    fn blocked(&self, v: usize) -> bool {
        self.lost[v] || (self.sp.spread == 'w' && self.hz[v] >= self.sp.cap)
    }
    fn dists(&self, from: usize) -> Vec<i32> {
        let mut d = vec![99; self.sp.n];
        let mut q = std::collections::VecDeque::new();
        d[from] = 0;
        q.push_back(from);
        while let Some(v) = q.pop_front() {
            for &w in &self.adj[v] {
                if d[w] == 99 && !self.lost[w] {
                    d[w] = d[v] + 1;
                    q.push_back(w);
                }
            }
        }
        d
    }
    fn items_left(&self) -> u8 {
        self.item.iter().filter(|x| **x).count() as u8 + self.carry[0] + self.carry[1]
    }
    fn failed(&self) -> bool {
        if self.doom >= self.sp.doom_max || self.hp <= 0 || self.round >= self.sp.rounds {
            return true;
        }
        self.sp.spread != 'a' && self.done + self.items_left() < self.sp.need
    }
    fn place(&mut self, v: usize) {
        let cap = self.sp.cap;
        if self.hz[v] < cap {
            self.hz[v] += 1;
            return;
        }
        match self.sp.spread {
            's' => {
                self.lost[v] = true;
                self.doom += 1;
                if self.item[v] {
                    self.item[v] = false;
                    self.gone += 1;
                }
                for p in 0..2 {
                    if self.pos[p] == v {
                        let opts: Vec<usize> = self.adj[v]
                            .iter()
                            .copied()
                            .filter(|&w| !self.lost[w])
                            .collect();
                        if opts.is_empty() {
                            self.hp = 0;
                        } else {
                            self.pos[p] = opts[self.rng.below(opts.len())];
                        }
                    }
                }
            }
            'o' => {
                self.doom += 1;
                for w in self.adj[v].clone() {
                    if self.hz[w] < cap {
                        self.hz[w] += 1;
                    }
                }
            }
            'f' => {
                for w in self.adj[v].clone() {
                    if self.hz[w] < cap {
                        self.hz[w] += 1;
                    }
                }
            }
            'a' => {
                self.doom += 1;
            }
            _ => {}
        }
    }
    fn event(&mut self) {
        let n = self.sp.n;
        if self.sp.spread == 'a' {
            // înaintare spre ieșire
            let d = self.dists(self.sp.exit);
            let mut nh = vec![0u8; n];
            for v in 0..n {
                if self.hz[v] == 0 {
                    continue;
                }
                if v == self.sp.exit {
                    self.doom = self.doom.saturating_add(self.hz[v]);
                    continue;
                }
                let to = self.adj[v]
                    .iter()
                    .copied()
                    .filter(|&w| d[w] < d[v])
                    .min()
                    .unwrap_or(v);
                nh[to] = nh[to].saturating_add(self.hz[v]);
            }
            self.hz = nh;
        }
        for _ in 0..self.sp.per {
            let cand: Vec<usize> = if self.sp.spread == 'a' {
                let d = self.dists(self.sp.exit);
                let mx = d.iter().copied().filter(|&x| x < 99).max().unwrap_or(0);
                (0..n).filter(|&v| d[v] == mx).collect()
            } else {
                (0..n)
                    .filter(|&v| !self.lost[v] && v != self.sp.exit)
                    .collect()
            };
            if cand.is_empty() {
                break;
            }
            let v = cand[self.rng.below(cand.len())];
            self.place(v);
        }
        if self.sp.spread == 'w' {
            self.doom += 1;
        }
        if self.sp.spread == 'f' {
            for p in 0..2 {
                if self.hz[self.pos[p]] >= self.sp.cap {
                    self.hp -= 1;
                }
            }
        }
        if self.pur < self.sp.n {
            let d0 = self.dists(self.pos[0]);
            let d1 = self.dists(self.pos[1]);
            let tgt = if d0[self.pur] <= d1[self.pur] {
                &d0
            } else {
                &d1
            };
            if let Some(&w) = self.adj[self.pur]
                .iter()
                .filter(|&&w| !self.lost[w])
                .min_by_key(|&&w| (tgt[w], w))
            {
                self.pur = w;
            }
            if self.pos.contains(&self.pur) {
                self.hp -= 1;
                self.pur = self.sp.pursuer;
            }
        }
        self.round += 1;
        self.side = Player::One;
        self.ap = AP;
        self.pend = false;
        // verdictul se dă doar la mutarea forțată (AI-ul nu e atras de starea finală)
        if self.done >= self.sp.need {
            self.end = Some(Outcome::Win(Player::One));
        } else if self.failed() {
            self.end = Some(Outcome::Draw);
        }
    }
    fn can_take(&self, me: usize) -> bool {
        !self.sp.rst || (self.round as usize) % 2 == me
    }
}

impl Mini for G {
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
        let p = self.pos[me];
        let mut v = vec!["fin".to_string()];
        for &w in &self.adj[p] {
            if !self.blocked(w) {
                v.push(format!("mov {w}"));
            }
        }
        let mut near = vec![p];
        near.extend(self.adj[p].iter().copied());
        for w in near {
            if !self.lost[w] && (self.hz[w] > 0 || w == self.pur) {
                v.push(format!("fix {w}"));
            }
        }
        if self.item[p] && self.carry[me] < 2 && self.can_take(me) {
            v.push("iau".into());
        }
        if p == self.sp.exit && self.carry[me] > 0 {
            v.push("dau".into());
        }
        v
    }
    fn step(&mut self, m: &str) {
        if m == "ev" {
            self.event();
            return;
        }
        let me = pi(self.side);
        let mut cost = 1;
        if m == "fin" {
            cost = self.ap;
        } else if let Some(a) = m.strip_prefix("mov ") {
            self.pos[me] = a.parse().unwrap();
        } else if let Some(a) = m.strip_prefix("fix ") {
            let a: usize = a.parse().unwrap();
            if a == self.pur && self.pur < self.sp.n {
                self.pur = self.sp.pursuer;
            } else {
                self.hz[a] -= 1;
                if self.sp.spread == 'a' {
                    self.done += 1;
                }
            }
        } else if m == "iau" {
            let p = self.pos[me];
            self.item[p] = false;
            self.carry[me] += 1;
        } else if m == "dau" {
            self.done += self.carry[me];
            self.carry[me] = 0;
        }
        self.ap = self.ap.saturating_sub(cost);
        if self.ap == 0 {
            if self.side == Player::One {
                self.side = Player::Two;
                self.ap = AP;
            } else {
                self.pend = true;
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        self.end
    }
    fn eval(&self, _p: Player) -> i32 {
        let mut s =
            1000 * self.done as i32 - 450 * self.doom as i32 - 300 * (self.sp.hp - self.hp) as i32;
        for v in 0..self.sp.n {
            let h = self.hz[v] as i32;
            s -= 18 * h + 8 * h * h + if self.item[v] { 25 * h } else { 0 };
        }
        if self.sp.spread == 'a' {
            // aproape de ieșire amenințările valorează mai mult
            let d = self.dists(self.sp.exit);
            for v in 0..self.sp.n {
                s -= self.hz[v] as i32 * (6 - d[v].min(6)) * 4;
            }
        }
        for me in 0..2 {
            s += 160 * self.carry[me] as i32;
            let d = self.dists(self.pos[me]);
            let tgt = if self.sp.spread == 'a' {
                (0..self.sp.n)
                    .filter(|&v| self.hz[v] > 0)
                    .map(|v| d[v])
                    .min()
                    .unwrap_or(0)
            } else if self.carry[me] > 0 {
                d[self.sp.exit]
            } else {
                (0..self.sp.n)
                    .filter(|&v| self.item[v])
                    .map(|v| d[v])
                    .min()
                    .unwrap_or(d[self.sp.exit])
            };
            s -= 14 * tgt.min(20);
            if self.pur < self.sp.n {
                let dp = d[self.pur].min(5);
                s += 12 * dp;
            }
            if self.sp.spread == 'f' && self.hz[self.pos[me]] >= self.sp.cap {
                s -= 200;
            }
        }
        s
    }
    fn show(&self) -> String {
        let mut s = format!(
            "{} — runda {}/{} | destin {}/{} | viață {} | {} {}/{}\n",
            self.sp.title,
            self.round + 1,
            self.sp.rounds,
            self.doom,
            self.sp.doom_max,
            self.hp,
            if self.sp.spread == 'a' {
                "eliminări"
            } else {
                "livrate"
            },
            self.done,
            self.sp.need
        );
        for v in 0..self.sp.n {
            let mut tags = String::new();
            if v == self.sp.exit {
                tags.push_str(" IEȘIRE");
            }
            if self.item[v] {
                tags.push_str(" [obiect]");
            }
            if self.lost[v] {
                tags.push_str(" PIERDUT");
            }
            if v == self.pur {
                tags.push_str(" {urmăritor}");
            }
            for p in 0..2 {
                if self.pos[p] == v {
                    tags.push_str(&format!(" <J{}>", p + 1));
                }
            }
            s.push_str(&format!(
                "  nod {v:>2} amenințare {}/{}{tags} -> {:?}\n",
                self.hz[v], self.sp.cap, self.adj[v]
            ));
        }
        s.push_str(&format!(
            "poartă J1 {} / J2 {} | La mutare: {} ({} acțiuni: mov n | fix n | iau | dau | fin; ev = sfârșit de rundă)\n",
            self.carry[0],
            self.carry[1],
            pname(self.side),
            self.ap
        ));
        s
    }
}
