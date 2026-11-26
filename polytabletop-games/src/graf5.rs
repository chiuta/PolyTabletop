//! Motor comun „control de teritoriu pe graf” (variante originale, 2 jucători, fără șansă la luptă).
//!
//! Fiecare joc are propria hartă, propriile reguli de câștig și de venit (`Cfg`).
//! Mutări: `lev a` (cumperi o trupă în nodul a), `mut a>b n` (muți n trupe; lupta e deterministă:
//! atacatorul cu n trupe câștigă dacă n > apărători + bonusul fortăreței), `cump b` (doar unde
//! mita e permisă), `fin` (închei tura). Informație completă; AI = alpha-beta cu adâncime fixă.
#![allow(
    clippy::needless_range_loop,
    clippy::manual_is_multiple_of,
    clippy::manual_memcpy,
    clippy::collapsible_match
)]
use crate::brand_util::*;
use polytabletop_core::{Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Win {
    /// După `rounds` runde: cea mai mare valoare de teritorii (plus punctele acumulate).
    Points,
    /// Cine ocupă capitala adversarului câștigă imediat.
    Capital,
    /// Cine ține la sfârșitul turei sale cel puțin k noduri speciale câștigă.
    Metro(u8),
}

pub struct Cfg {
    pub title: &'static str,
    /// Harta: 0 grilă, 1 inel cu coarde, 2 două arhipelaguri unite prin punți, 3 butuc cu spițe.
    pub map: u8,
    pub n: usize,
    pub w: usize,
    pub rounds: u32,
    pub ap: u8,
    pub fort: i32,
    pub win: Win,
    pub income_base: i32,
    pub income_div: i32,
    pub lev_cost: i32,
    pub start_troops: i32,
    /// Al doilea jucător: (noduri de start, trupe pe capitală).
    pub p2_start: (usize, i32),
    pub loot: i32,
    pub bribe: bool,
    pub score_each: u32,
    pub unit: &'static str,
}

#[derive(Clone)]
pub struct Graf {
    cfg: &'static Cfg,
    adj: Vec<Vec<usize>>,
    val: Vec<i32>,
    fort: Vec<bool>,
    special: Vec<bool>,
    owner: Vec<u8>,
    troops: Vec<i32>,
    gold: [i32; 2],
    vp: [i32; 2],
    turn: usize,
    ap_left: u8,
    round: u32,
    won: Option<Outcome>,
}

fn build_adj(cfg: &Cfg) -> Vec<Vec<usize>> {
    let n = cfg.n;
    let mut adj = vec![Vec::new(); n];
    let link = |a: usize, b: usize, adj: &mut Vec<Vec<usize>>| {
        if a != b && !adj[a].contains(&b) {
            adj[a].push(b);
            adj[b].push(a);
        }
    };
    match cfg.map {
        0 => {
            let w = cfg.w;
            for i in 0..n {
                if (i + 1) % w != 0 && i + 1 < n {
                    link(i, i + 1, &mut adj);
                }
                if i + w < n {
                    link(i, i + w, &mut adj);
                }
            }
        }
        1 => {
            for i in 0..n {
                link(i, (i + 1) % n, &mut adj);
                if i % 3 == 0 {
                    link(i, (i + n / 2) % n, &mut adj);
                }
            }
        }
        2 => {
            let h = n / 2;
            for i in 0..h - 1 {
                link(i, i + 1, &mut adj);
                link(h + i, h + i + 1, &mut adj);
            }
            for i in 0..h {
                link(i, (i + 2) % h, &mut adj);
                link(h + i, h + (i + 2) % h, &mut adj);
            }
            link(h / 2, h + h / 2, &mut adj);
            link(h - 1, n - 1, &mut adj);
            link(0, h, &mut adj);
        }
        _ => {
            // nod 0 = butuc; spițe: grupuri de câte 3 noduri în lanț, extremitățile legate la vecinul spiței următoare
            let spokes = (n - 1) / 3;
            for s in 0..spokes {
                let b = 1 + s * 3;
                link(0, b, &mut adj);
                link(b, b + 1, &mut adj);
                link(b + 1, b + 2, &mut adj);
                let nb = 1 + ((s + 1) % spokes) * 3;
                link(b + 2, nb + 2, &mut adj);
            }
        }
    }
    adj
}

fn bfs_order(adj: &[Vec<usize>], from: usize) -> Vec<usize> {
    let mut seen = vec![false; adj.len()];
    let mut q = std::collections::VecDeque::new();
    let mut out = Vec::new();
    q.push_back(from);
    seen[from] = true;
    while let Some(x) = q.pop_front() {
        out.push(x);
        for &y in &adj[x] {
            if !seen[y] {
                seen[y] = true;
                q.push_back(y);
            }
        }
    }
    out
}

impl Graf {
    pub fn new(cfg: &'static Cfg) -> Self {
        Self::with_seed(cfg, seed_now())
    }
    pub fn with_seed(cfg: &'static Cfg, seed: u64) -> Self {
        let n = cfg.n;
        let adj = build_adj(cfg);
        let mut rng = polytabletop_core::rng::Rng::new(seed);
        let mut val = vec![1; n];
        let mut fort = vec![false; n];
        let mut special = vec![false; n];
        for i in 0..n {
            val[i] = 1 + (i % 3 == 0) as i32 + (i % 7 == 3) as i32;
            fort[i] = i % 4 == 1;
            special[i] = i % 4 == 2;
        }
        let mut owner = vec![2u8; n];
        let mut troops = vec![0i32; n];
        for i in 0..n {
            troops[i] = 1 + (rng.below(2) as i32);
        }
        let o1 = bfs_order(&adj, 0);
        let o2 = bfs_order(&adj, n - 1);
        for &i in o1.iter().take(3) {
            owner[i] = 0;
            troops[i] = cfg.start_troops;
        }
        let mut k = 0;
        for &i in &o2 {
            if k >= cfg.p2_start.0 {
                break;
            }
            if owner[i] == 2 {
                owner[i] = 1;
                troops[i] = cfg.start_troops;
                k += 1;
            }
        }
        troops[n - 1] = cfg.p2_start.1;
        troops[0] = cfg.start_troops + 1;
        // nodurile speciale de start nu rămân fără valoare: capitalele sunt fortărețe
        fort[0] = true;
        fort[n - 1] = true;
        Graf {
            cfg,
            adj,
            val,
            fort,
            special,
            owner,
            troops,
            gold: [cfg.lev_cost + 1; 2],
            vp: [0; 2],
            turn: 0,
            ap_left: cfg.ap,
            round: 0,
            won: None,
        }
    }
    fn mine(&self, p: usize) -> impl Iterator<Item = usize> + '_ {
        (0..self.cfg.n).filter(move |&i| self.owner[i] as usize == p)
    }
    fn count_val(&self, p: usize) -> i32 {
        self.mine(p).map(|i| self.val[i]).sum()
    }
    fn bonus(&self, i: usize) -> i32 {
        if self.fort[i] {
            self.cfg.fort
        } else {
            0
        }
    }
    fn bribe_cost(&self, b: usize) -> i32 {
        2 * self.troops[b] + 2 + self.val[b]
    }
    fn end_turn(&mut self) {
        let p = self.turn;
        if let Win::Metro(k) = self.cfg.win {
            if self.mine(p).filter(|&i| self.special[i]).count() >= k as usize {
                self.won = Some(Outcome::Win(if p == 0 { Player::One } else { Player::Two }));
                return;
            }
        }
        if p == 1 {
            // sfârșit de rundă
            self.round += 1;
            for q in 0..2 {
                let inc = self.cfg.income_base + self.count_val(q) / self.cfg.income_div.max(1);
                self.gold[q] += inc;
            }
            if self.cfg.score_each > 0 && self.round % self.cfg.score_each == 0 {
                for q in 0..2 {
                    self.vp[q] += self.count_val(q);
                }
            }
            if self.round >= self.cfg.rounds {
                self.won = Some(self.final_result());
                return;
            }
        }
        self.turn = 1 - p;
        self.ap_left = self.cfg.ap;
    }
    fn total(&self, p: usize) -> i32 {
        self.vp[p] + self.count_val(p)
    }
    fn final_result(&self) -> Outcome {
        by_score(self.total(0), self.total(1))
    }
    fn after_action(&mut self) {
        if self.cfg.win == Win::Capital {
            let c1 = self.owner[self.cfg.n - 1] == 0;
            let c0 = self.owner[0] == 1;
            if c1 {
                self.won = Some(Outcome::Win(Player::One));
                return;
            }
            if c0 {
                self.won = Some(Outcome::Win(Player::Two));
                return;
            }
        }
        self.ap_left = self.ap_left.saturating_sub(1);
        if self.ap_left == 0 {
            self.end_turn();
        }
    }
}

impl Mini for Graf {
    fn depth(&self) -> u32 {
        1
    }
    fn side(&self) -> Player {
        if self.turn == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn moves(&self) -> Vec<String> {
        let p = self.turn;
        let mut v = vec!["fin".to_string()];
        for a in self.mine(p) {
            if self.gold[p] >= self.cfg.lev_cost {
                v.push(format!("lev {a}"));
            }
            let t = self.troops[a];
            if t > 1 {
                let mut opts = vec![1, (t - 1) / 2, t - 1];
                opts.retain(|&x| x >= 1);
                opts.sort();
                opts.dedup();
                for &b in &self.adj[a] {
                    for &k in &opts {
                        v.push(format!("mut {a}>{b}:{k}"));
                    }
                }
            }
        }
        if self.cfg.bribe {
            for b in 0..self.cfg.n {
                if self.owner[b] as usize != p
                    && self.adj[b].iter().any(|&a| self.owner[a] as usize == p)
                    && self.gold[p] >= self.bribe_cost(b)
                {
                    v.push(format!("cump {b}"));
                }
            }
        }
        v
    }
    fn step(&mut self, m: &str) {
        let p = self.turn;
        if m == "fin" {
            self.end_turn();
            return;
        }
        let mut it = m.split([' ', '>', ':']);
        match it.next().unwrap() {
            "lev" => {
                let a: usize = it.next().unwrap().parse().unwrap();
                self.gold[p] -= self.cfg.lev_cost;
                self.troops[a] += 1;
            }
            "cump" => {
                let b: usize = it.next().unwrap().parse().unwrap();
                self.gold[p] -= self.bribe_cost(b);
                self.owner[b] = p as u8;
                self.troops[b] = 1;
            }
            _ => {
                let a: usize = it.next().unwrap().parse().unwrap();
                let b: usize = it.next().unwrap().parse().unwrap();
                let n: i32 = it.next().unwrap().parse().unwrap();
                self.troops[a] -= n;
                if self.owner[b] as usize == p {
                    self.troops[b] += n;
                } else {
                    let d = self.troops[b];
                    let need = d + self.bonus(b);
                    if n > need {
                        self.owner[b] = p as u8;
                        self.troops[b] = n - need;
                        self.gold[p] += self.cfg.loot;
                    } else {
                        self.troops[b] = (d - n).max(1);
                    }
                }
            }
        }
        self.after_action();
    }
    fn result(&self) -> Option<Outcome> {
        self.won
    }
    fn eval(&self, p: Player) -> i32 {
        let i = pi(p);
        let o = 1 - i;
        let tr = |q: usize| -> i32 { self.mine(q).map(|x| self.troops[x]).sum() };
        let mut e = 10 * (self.total(i) - self.total(o))
            + 4 * (tr(i) - tr(o))
            + (self.gold[i] - self.gold[o]);
        // amenințări: ținte care pot fi cucerite acum de către cineva
        let thr = |q: usize| -> i32 {
            let mut t = 0;
            for b in 0..self.cfg.n {
                if self.owner[b] as usize == q {
                    continue;
                }
                let need = self.troops[b] + self.bonus(b);
                if self.adj[b]
                    .iter()
                    .any(|&a| self.owner[a] as usize == q && self.troops[a] - 1 > need)
                {
                    t += 5 * self.val[b];
                    if self.cfg.win == Win::Capital && (b == 0 || b == self.cfg.n - 1) {
                        t += 40;
                    }
                }
            }
            t
        };
        e += thr(i) - thr(o);
        if let Win::Metro(_) = self.cfg.win {
            let s = |q: usize| self.mine(q).filter(|&x| self.special[x]).count() as i32;
            e += 25 * (s(i) - s(o));
        }
        if self.cfg.win == Win::Capital {
            // cât de aproape sunt trupele mele de capitala adversă
            let (tgt, home) = if i == 0 {
                (self.cfg.n - 1, 0)
            } else {
                (0, self.cfg.n - 1)
            };
            e += 3 * self.troops[home].min(6) - 3 * self.troops[tgt].min(6);
            let order = bfs_order(&self.adj, tgt);
            for (rank, &x) in order.iter().enumerate() {
                if self.owner[x] as usize == i {
                    e += (10 - rank as i32).max(0);
                } else if self.owner[x] as usize == o {
                    e -= (10 - (self.cfg.n - rank) as i32).max(0) / 2;
                }
            }
        }
        e
    }
    fn show(&self) -> String {
        let c = self.cfg;
        let mut s = format!(
            "{} — runda {}/{}, la mutare J{} ({} acțiuni), aur J1 {} / J2 {}\n",
            c.title,
            (self.round + 1).min(c.rounds),
            c.rounds,
            self.turn + 1,
            self.ap_left,
            self.gold[0],
            self.gold[1]
        );
        s.push_str(&format!(
            "Scor: J1 {} / J2 {} ({} = valoarea teritoriilor + puncte)\n",
            self.total(0),
            self.total(1),
            c.unit
        ));
        for i in 0..c.n {
            let o = match self.owner[i] {
                0 => "J1",
                1 => "J2",
                _ => "--",
            };
            s.push_str(&format!(
                "[{:2}] {} {:2} {} val{}{}{} -> {:?}\n",
                i,
                o,
                self.troops[i],
                c.unit,
                self.val[i],
                if self.fort[i] { " fort" } else { "" },
                if self.special[i] && matches!(c.win, Win::Metro(_)) {
                    " cheie"
                } else {
                    ""
                },
                self.adj[i]
            ));
        }
        s
    }
}
