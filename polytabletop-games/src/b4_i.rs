//! Motor „influență pe hartă” (lista B4): puncte de influență pe noduri, control prin diferență
//! ≥ stabilitate, punctaj periodic pe o bară comună (tragere de funie). Informație perfectă.
use crate::brand_util::*;
use polytabletop_core::{Outcome, Player};

pub struct ISpec {
    pub title: &'static str,
    pub edges: &'static str,
    pub vals: &'static [i32],
    pub stab: &'static [i32],
    pub starts: [&'static [usize]; 2],
    pub ap: u8,
    pub rounds: u32,
    /// punctaj la fiecare `every` runde
    pub every: u32,
    pub win: i32,
    pub depth: u32,
}

#[derive(Clone)]
pub struct I {
    sp: &'static ISpec,
    adj: Vec<Vec<usize>>,
    inf: Vec<[i32; 2]>,
    track: i32,
    ap_left: u8,
    round: u32,
    side: Player,
    won: Option<Player>,
}

impl I {
    pub fn new(sp: &'static ISpec) -> Self {
        let n = sp.vals.len();
        let mut adj = vec![Vec::new(); n];
        for e in sp.edges.split(',') {
            let (a, b) = e.split_once('-').unwrap();
            let (a, b): (usize, usize) = (a.parse().unwrap(), b.parse().unwrap());
            adj[a].push(b);
            adj[b].push(a);
        }
        let mut inf = vec![[0; 2]; n];
        for p in 0..2 {
            for &i in sp.starts[p] {
                inf[i][p] += 2;
            }
        }
        I {
            sp,
            adj,
            inf,
            track: 0,
            ap_left: sp.ap,
            round: 0,
            side: Player::One,
            won: None,
        }
    }
    pub fn with_seed(sp: &'static ISpec, _seed: u64) -> Self {
        Self::new(sp)
    }
    fn ctrl(&self, i: usize) -> Option<usize> {
        let d = self.inf[i][0] - self.inf[i][1];
        if d >= self.sp.stab[i] {
            Some(0)
        } else if -d >= self.sp.stab[i] {
            Some(1)
        } else {
            None
        }
    }
    fn near(&self, p: usize, i: usize) -> bool {
        self.inf[i][p] > 0 || self.adj[i].iter().any(|&j| self.inf[j][p] > 0)
    }
    fn scoring(&mut self) {
        for i in 0..self.inf.len() {
            match self.ctrl(i) {
                Some(0) => self.track += self.sp.vals[i],
                Some(1) => self.track -= self.sp.vals[i],
                _ => {}
            }
        }
        if self.track >= self.sp.win {
            self.won = Some(Player::One);
        } else if self.track <= -self.sp.win {
            self.won = Some(Player::Two);
        }
    }
}

impl Mini for I {
    fn side(&self) -> Player {
        self.side
    }
    fn depth(&self) -> u32 {
        self.sp.depth
    }
    fn moves(&self) -> Vec<String> {
        let me = pi(self.side);
        let mut v = vec!["fin".to_string()];
        for i in 0..self.inf.len() {
            let cost = if self.near(me, i) { 1 } else { 2 };
            if self.ap_left >= cost {
                v.push(format!("inf {i}"));
            }
            if self.ap_left >= 2 && self.inf[i][1 - me] > 0 {
                v.push(format!("cup {i}"));
            }
        }
        v
    }
    fn step(&mut self, m: &str) {
        let me = pi(self.side);
        let mut cost = 1;
        if m == "fin" {
            cost = self.ap_left;
        } else if let Some(i) = m.strip_prefix("inf ") {
            let i: usize = i.parse().unwrap();
            cost = if self.near(me, i) { 1 } else { 2 };
            self.inf[i][me] += 1;
        } else if let Some(i) = m.strip_prefix("cup ") {
            let i: usize = i.parse().unwrap();
            cost = 2;
            self.inf[i][1 - me] = (self.inf[i][1 - me] - 2).max(0);
        }
        self.ap_left = self.ap_left.saturating_sub(cost);
        if self.ap_left == 0 {
            self.ap_left = self.sp.ap;
            self.side = self.side.other();
            if self.side == Player::One {
                self.round += 1;
                if self.round % self.sp.every == 0 {
                    self.scoring();
                }
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if let Some(w) = self.won {
            return Some(Outcome::Win(w));
        }
        if self.round >= self.sp.rounds {
            return Some(match self.track.cmp(&0) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                _ => Outcome::Draw,
            });
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let sign = if a == 0 { 1 } else { -1 };
        let mut s = 10 * self.track * sign;
        for i in 0..self.inf.len() {
            let v = self.sp.vals[i];
            match self.ctrl(i) {
                Some(x) if x == a => s += 5 * v,
                Some(_) => s -= 5 * v,
                None => {}
            }
            s += v * (self.inf[i][a] - self.inf[i][1 - a]).clamp(-3, 3);
        }
        s
    }
    fn show(&self) -> String {
        let mut s = format!(
            "{} — runda {}/{} | bara {:+} (J1 pozitiv, J2 negativ, prag {})\n",
            self.sp.title,
            self.round + 1,
            self.sp.rounds,
            self.track,
            self.sp.win
        );
        for i in 0..self.inf.len() {
            let c = match self.ctrl(i) {
                Some(0) => "J1",
                Some(1) => "J2",
                _ => "—",
            };
            s.push_str(&format!(
                "  nod {i:>2} val {} stab {} J1:{} J2:{} control {c} -> {:?}\n",
                self.sp.vals[i], self.sp.stab[i], self.inf[i][0], self.inf[i][1], self.adj[i]
            ));
        }
        s.push_str(&format!(
            "La mutare: {} ({} puncte: inf n | cup n | fin; punctaj la fiecare {} runde)\n",
            pname(self.side),
            self.ap_left,
            self.sp.every
        ));
        s
    }
}
