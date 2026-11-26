//! Motor „cucerire pe hartă” (lista B4): noduri cu valoare, unități, mutări și bătălii deterministe
//! (fără zaruri). `ap` acțiuni pe tură. Capitala inamică luată = victorie imediată.
use crate::brand_util::*;
use polytabletop_core::{Outcome, Player};

pub struct CSpec {
    pub title: &'static str,
    pub edges: &'static str,
    pub vals: &'static [i32],
    /// nodurile-fortăreață (apărătorul primește +1)
    pub forts: &'static [usize],
    pub caps: [usize; 2],
    pub ap: u8,
    pub turns: u32,
    /// t = tehnologie (+1 atac, max 3) cu `tec`
    pub fl: &'static str,
    pub depth: u32,
}

#[derive(Clone)]
pub struct C {
    sp: &'static CSpec,
    adj: Vec<Vec<usize>>,
    own: Vec<i8>,
    u: Vec<i32>,
    tech: [i32; 2],
    ap_left: u8,
    turn: u32,
    side: Player,
}

impl C {
    pub fn new(sp: &'static CSpec) -> Self {
        let n = sp.vals.len();
        let mut adj = vec![Vec::new(); n];
        for e in sp.edges.split(',') {
            let (a, b) = e.split_once('-').unwrap();
            let (a, b): (usize, usize) = (a.parse().unwrap(), b.parse().unwrap());
            adj[a].push(b);
            adj[b].push(a);
        }
        let mut own = vec![-1i8; n];
        let mut u = vec![0; n];
        for p in 0..2 {
            own[sp.caps[p]] = p as i8;
            u[sp.caps[p]] = 3;
            for &j in &adj[sp.caps[p]] {
                if own[j] < 0 {
                    own[j] = p as i8;
                    u[j] = 1;
                }
            }
        }
        C {
            sp,
            adj,
            own,
            u,
            tech: [0; 2],
            ap_left: sp.ap,
            turn: 0,
            side: Player::One,
        }
    }
    pub fn with_seed(sp: &'static CSpec, _seed: u64) -> Self {
        Self::new(sp)
    }
    fn owned(&self, p: usize) -> impl Iterator<Item = usize> + '_ {
        (0..self.own.len()).filter(move |&i| self.own[i] == p as i8)
    }
    fn fort(&self, i: usize) -> i32 {
        self.sp.forts.contains(&i) as i32
    }
    fn mv(&mut self, a: usize, b: usize, k: i32) {
        let me = self.own[a];
        self.u[a] -= k;
        if self.u[a] == 0 {
            self.own[a] = -1;
        }
        if self.own[b] == me || self.own[b] < 0 {
            self.own[b] = me;
            self.u[b] += k;
        } else {
            let atk = k + self.tech[me as usize];
            let def = self.u[b] + self.fort(b);
            if atk > def {
                self.own[b] = me;
                self.u[b] = (atk - def).max(1);
            } else if atk == def {
                self.u[b] = 0;
                self.own[b] = -1;
            } else {
                self.u[b] = (self.u[b] - (atk - self.fort(b)).max(1)).max(1);
            }
        }
    }
    fn pts(&self, p: usize) -> i32 {
        self.owned(p).map(|i| self.sp.vals[i]).sum::<i32>()
            + self
                .u
                .iter()
                .enumerate()
                .filter(|(i, _)| self.own[*i] == p as i8)
                .map(|(_, x)| *x)
                .sum::<i32>()
                / 3
    }
    fn capital_lost(&self, p: usize) -> bool {
        self.own[self.sp.caps[p]] != p as i8
    }
}

impl Mini for C {
    fn side(&self) -> Player {
        self.side
    }
    fn depth(&self) -> u32 {
        self.sp.depth
    }
    fn moves(&self) -> Vec<String> {
        let me = pi(self.side);
        let mut v = vec!["fin".to_string()];
        for a in self.owned(me).collect::<Vec<_>>() {
            v.push(format!("rec {a}"));
            for &b in &self.adj[a] {
                if self.u[a] >= 2 {
                    v.push(format!("mov {a}>{b}"));
                }
                v.push(format!("all {a}>{b}"));
            }
        }
        if self.sp.fl.contains('t') && self.tech[me] < 3 && self.ap_left >= 2 {
            v.push("tec".into());
        }
        v
    }
    fn step(&mut self, m: &str) {
        let me = pi(self.side);
        let mut cost = 1;
        if m == "fin" {
            cost = self.ap_left;
        } else if m == "tec" {
            self.tech[me] += 1;
            cost = 2;
        } else if let Some(a) = m.strip_prefix("rec ") {
            let a: usize = a.parse().unwrap();
            self.u[a] += 1;
        } else {
            let (a, b) = m[4..].split_once('>').unwrap();
            let (a, b): (usize, usize) = (a.parse().unwrap(), b.parse().unwrap());
            let k = if m.starts_with("mov") {
                self.u[a] - 1
            } else {
                self.u[a]
            };
            self.mv(a, b, k);
        }
        self.ap_left = self.ap_left.saturating_sub(cost);
        if self.ap_left == 0 {
            // venit: +1 unitate în capitala proprie dacă încă o ai
            let cp = self.sp.caps[me];
            if self.own[cp] == me as i8 {
                self.u[cp] += 1;
            }
            self.side = self.side.other();
            self.ap_left = self.sp.ap;
            if self.side == Player::One {
                self.turn += 1;
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.capital_lost(0) && !self.capital_lost(1) {
            return Some(Outcome::Win(Player::Two));
        }
        if self.capital_lost(1) && !self.capital_lost(0) {
            return Some(Outcome::Win(Player::One));
        }
        if self.capital_lost(0) && self.capital_lost(1) {
            return Some(Outcome::Draw);
        }
        if self.turn >= self.sp.turns {
            return Some(by_score(self.pts(0), self.pts(1)));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let mat = |q: usize| -> i32 {
            self.u
                .iter()
                .enumerate()
                .filter(|(i, _)| self.own[*i] == q as i8)
                .map(|(_, x)| *x * 2)
                .sum::<i32>()
                + 6 * self.pts(q)
                + 4 * self.tech[q]
        };
        mat(a) - mat(1 - a)
    }
    fn show(&self) -> String {
        let mut s = format!(
            "{} — tura {}/{}\n",
            self.sp.title,
            self.turn + 1,
            self.sp.turns
        );
        for i in 0..self.own.len() {
            let o = match self.own[i] {
                0 => "J1",
                1 => "J2",
                _ => "—",
            };
            s.push_str(&format!(
                "  nod {i:>2} val {} {}{} {o} ×{} -> {:?}\n",
                self.sp.vals[i],
                if self.sp.caps.contains(&i) {
                    "CAP "
                } else {
                    ""
                },
                if self.sp.forts.contains(&i) {
                    "FORT"
                } else {
                    ""
                },
                self.u[i],
                self.adj[i]
            ));
        }
        s.push_str(&format!(
            "tehnologie {:?} | La mutare: {} ({} acțiuni: rec a | mov a>b | all a>b | tec | fin)\n",
            self.tech,
            pname(self.side),
            self.ap_left
        ));
        s
    }
}
