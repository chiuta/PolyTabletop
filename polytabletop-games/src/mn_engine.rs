//! Motor „schiță de luptă” (lista T3, miniaturi): tablă pătrățită sau hexagonală, unități cu
//! mișcare, rază, zaruri și armură, trei scopuri (eliminare, controlul punctelor, străpungere).
//! Zarurile se aruncă doar în mutarea forțată `tira`, după ce atacul a fost ales: starea de
//! dinainte nu conține niciun rezultat viitor, iar AI-ul zarurilor folosește adâncime 0.
#![allow(clippy::needless_range_loop)]
use crate::zr_util::*;

pub struct UT {
    pub sym: char,
    pub name: &'static str,
    pub hp: i8,
    pub mv: i8,
    pub rng: i8,
    pub dmg: i8,
    pub arm: i8,
    /// H = nu atacă după mișcare, L = zburător, M = medic, C = încărcare, B = explozie,
    /// K = împinge, F = bonus de flanc
    pub fl: &'static str,
}

pub struct MSpec {
    pub title: &'static str,
    pub w: i8,
    pub h: i8,
    pub hex: bool,
    pub diag: bool,
    /// rânduri separate prin `/`: `.` liber, `#` zid, `~` teren dificil, `*` punct
    pub map: &'static str,
    pub uts: &'static [UT],
    pub army: [&'static str; 2],
    /// 0 = eliminare (puncte pe pierderi), 1 = controlul punctelor, 2 = străpungere
    pub goal: u8,
    pub act: u8,
    pub turns: u32,
    pub target: i32,
    /// d = zaruri, r = linie de vedere, v = acoperire, x = 6 dublu, c = încărcare, f = flanc, p = împingere
    pub fl: &'static str,
}

#[derive(Clone)]
struct U {
    t: u8,
    own: u8,
    x: i8,
    y: i8,
    hp: i8,
    used: u8,
    mvd: i8,
}

#[derive(Clone)]
pub struct Mn {
    sp: &'static MSpec,
    rng: u64,
    terr: Vec<u8>,
    us: Vec<U>,
    side: usize,
    act_left: u8,
    turn: u32,
    pts: [i32; 2],
    pend: Option<(usize, usize)>,
    over: Option<Outcome>,
    last: String,
}

fn cell(x: i8, y: i8) -> String {
    format!("{}{}", (b'a' + x as u8) as char, y + 1)
}

fn parse_cell(s: &str) -> (i8, i8) {
    let b = s.as_bytes();
    ((b[0] - b'a') as i8, s[1..].parse::<i8>().unwrap_or(1) - 1)
}

impl Mn {
    pub fn new(sp: &'static MSpec) -> Self {
        Self::with_seed(sp, seed_now())
    }
    pub fn with_seed(sp: &'static MSpec, seed: u64) -> Self {
        let mut terr = vec![b'.'; (sp.w as usize) * (sp.h as usize)];
        for (y, row) in sp.map.split('/').enumerate() {
            for (x, ch) in row.bytes().enumerate() {
                terr[y * sp.w as usize + x] = ch;
            }
        }
        let mut us = Vec::new();
        for p in 0..2 {
            let n = sp.army[p].len() as i8;
            for (i, ch) in sp.army[p].chars().enumerate() {
                let t = sp
                    .uts
                    .iter()
                    .position(|u| u.sym == ch)
                    .expect("tip necunoscut") as u8;
                let i = i as i8;
                let per = sp.w.min(n);
                let row = i / per;
                let k = i % per;
                let cnt = if (row + 1) * per <= n {
                    per
                } else {
                    n - row * per
                };
                let x = (k * sp.w + sp.w / 2) / cnt.max(1);
                let x = x.min(sp.w - 1);
                let y = if p == 0 { row } else { sp.h - 1 - row };
                us.push(U {
                    t,
                    own: p as u8,
                    x,
                    y,
                    hp: sp.uts[t as usize].hp,
                    used: 0,
                    mvd: 0,
                });
            }
        }
        Mn {
            sp,
            rng: seed_state(seed),
            terr,
            us,
            side: 0,
            act_left: sp.act,
            turn: 0,
            pts: [0; 2],
            pend: None,
            over: None,
            last: String::new(),
        }
    }
    fn dice(&self) -> bool {
        self.sp.fl.contains('d')
    }
    fn ter(&self, x: i8, y: i8) -> u8 {
        self.terr[y as usize * self.sp.w as usize + x as usize]
    }
    fn inb(&self, x: i8, y: i8) -> bool {
        x >= 0 && y >= 0 && x < self.sp.w && y < self.sp.h
    }
    fn at(&self, x: i8, y: i8) -> Option<usize> {
        self.us.iter().position(|u| u.x == x && u.y == y)
    }
    fn ut(&self, i: usize) -> &'static UT {
        &self.sp.uts[self.us[i].t as usize]
    }
    fn has(&self, i: usize, f: char) -> bool {
        self.ut(i).fl.contains(f)
    }
    fn nbrs(&self, x: i8, y: i8) -> Vec<(i8, i8)> {
        let mut v = Vec::new();
        if self.sp.hex {
            let d: [(i8, i8); 6] = if y % 2 == 0 {
                [(1, 0), (-1, 0), (0, -1), (-1, -1), (0, 1), (-1, 1)]
            } else {
                [(1, 0), (-1, 0), (1, -1), (0, -1), (1, 1), (0, 1)]
            };
            for (dx, dy) in d {
                v.push((x + dx, y + dy));
            }
        } else {
            for (dx, dy) in [
                (0, -1),
                (-1, 0),
                (1, 0),
                (0, 1),
                (-1, -1),
                (1, -1),
                (-1, 1),
                (1, 1),
            ] {
                if self.sp.diag || dx == 0 || dy == 0 {
                    v.push((x + dx, y + dy));
                }
            }
        }
        v.retain(|&(a, b)| self.inb(a, b));
        v
    }
    fn dist(&self, a: (i8, i8), b: (i8, i8)) -> i32 {
        if self.sp.hex {
            let cube = |p: (i8, i8)| {
                let q = p.0 as i32 - (p.1 as i32 - (p.1 as i32 & 1)) / 2;
                (q, p.1 as i32)
            };
            let (q1, r1) = cube(a);
            let (q2, r2) = cube(b);
            let (dq, dr) = (q1 - q2, r1 - r2);
            (dq.abs() + dr.abs() + (dq + dr).abs()) / 2
        } else {
            let dx = (a.0 as i32 - b.0 as i32).abs();
            let dy = (a.1 as i32 - b.1 as i32).abs();
            if self.sp.diag {
                dx.max(dy)
            } else {
                dx + dy
            }
        }
    }
    fn los(&self, a: (i8, i8), b: (i8, i8)) -> bool {
        let d = self.dist(a, b);
        if d <= 1 {
            return true;
        }
        for i in 1..d {
            let t = i as f64 / d as f64;
            let (x, y) = if self.sp.hex {
                let cube = |p: (i8, i8)| {
                    let q = p.0 as f64 - (p.1 as f64 - (p.1 & 1) as f64) / 2.0;
                    (q, p.1 as f64)
                };
                let (q1, r1) = cube(a);
                let (q2, r2) = cube(b);
                let (q, r) = (q1 + (q2 - q1) * t + 1e-6, r1 + (r2 - r1) * t + 2e-6);
                let s = -q - r;
                let (mut rq, mut rr, rs) = (q.round(), r.round(), s.round());
                let (dq, dr, ds) = ((rq - q).abs(), (rr - r).abs(), (rs - s).abs());
                if dq > dr && dq > ds {
                    rq = -rr - rs;
                } else if dr > ds {
                    rr = -rq - rs;
                }
                let y = rr as i32;
                ((rq as i32 + (y - (y & 1)) / 2) as i8, y as i8)
            } else {
                (
                    (a.0 as f64 + (b.0 as f64 - a.0 as f64) * t + 1e-6).round() as i8,
                    (a.1 as f64 + (b.1 as f64 - a.1 as f64) * t + 2e-6).round() as i8,
                )
            };
            if self.inb(x, y) && self.ter(x, y) == b'#' {
                return false;
            }
        }
        true
    }
    fn reach(&self, i: usize) -> Vec<(i8, i8)> {
        let u = &self.us[i];
        let ut = self.ut(i);
        let fly = ut.fl.contains('L');
        let n = self.terr.len();
        let mut best = vec![i32::MAX; n];
        let idx = |x: i8, y: i8| y as usize * self.sp.w as usize + x as usize;
        let mut st = vec![(u.x, u.y)];
        best[idx(u.x, u.y)] = 0;
        let mut out = Vec::new();
        while let Some((x, y)) = st.pop() {
            let c = best[idx(x, y)];
            for (a, b) in self.nbrs(x, y) {
                let t = self.ter(a, b);
                if t == b'#' && !fly {
                    continue;
                }
                let occ = self.at(a, b).is_some();
                if occ && !fly {
                    continue;
                }
                let step = if t == b'~' && !fly { 2 } else { 1 };
                let nc = c + step;
                if nc <= ut.mv as i32 && nc < best[idx(a, b)] {
                    best[idx(a, b)] = nc;
                    st.push((a, b));
                }
            }
        }
        for y in 0..self.sp.h {
            for x in 0..self.sp.w {
                if best[idx(x, y)] != i32::MAX
                    && best[idx(x, y)] > 0
                    && self.at(x, y).is_none()
                    && self.ter(x, y) != b'#'
                {
                    out.push((x, y));
                }
            }
        }
        out
    }
    fn can_act(&self, i: usize) -> bool {
        let u = &self.us[i];
        u.own as usize == self.side
            && (u.used == 0 && self.act_left > 0 || u.used != 0 && u.used & 2 == 0)
    }
    fn can_hit(&self, a: usize, b: usize) -> bool {
        let (ua, ub) = (&self.us[a], &self.us[b]);
        if ua.own == ub.own || ua.used & 2 != 0 {
            return false;
        }
        if self.has(a, 'H') && ua.used & 1 != 0 {
            return false;
        }
        let d = self.dist((ua.x, ua.y), (ub.x, ub.y));
        if d < 1 || d > self.ut(a).rng as i32 {
            return false;
        }
        !(self.sp.fl.contains('r') && !self.los((ua.x, ua.y), (ub.x, ub.y)))
    }
    fn nd(&self, a: usize, b: usize) -> i32 {
        let ua = &self.us[a];
        let ub = &self.us[b];
        let mut n = self.ut(a).dmg as i32;
        let melee = self.dist((ua.x, ua.y), (ub.x, ub.y)) == 1;
        if melee && ua.mvd >= 2 && (self.sp.fl.contains('c') || self.has(a, 'C')) {
            n += 1;
        }
        if (self.sp.fl.contains('f') || self.has(a, 'F'))
            && self
                .us
                .iter()
                .enumerate()
                .any(|(k, w)| k != a && w.own == ua.own && self.dist((w.x, w.y), (ub.x, ub.y)) == 1)
        {
            n += 1;
        }
        n
    }
    fn arm(&self, b: usize) -> i32 {
        let ub = &self.us[b];
        let mut a = self.ut(b).arm as i32;
        if self.sp.fl.contains('v') && self.ter(ub.x, ub.y) == b'~' {
            a += 1;
        }
        a
    }
    fn ev(&self, a: usize, b: usize) -> i32 {
        let n = self.nd(a, b);
        let ar = self.arm(b);
        if self.dice() {
            (n * (6 - ar).clamp(0, 6) * 10 / 6).min(self.us[b].hp as i32 * 10)
        } else {
            ((n - ar).max(1) * 10).min(self.us[b].hp as i32 * 10)
        }
    }
    fn hurt(&mut self, b: usize, d: i32, by: usize, dead: &mut Vec<usize>) {
        self.us[b].hp -= d as i8;
        if self.us[b].hp <= 0 && !dead.contains(&b) {
            dead.push(b);
            if self.sp.goal == 0 {
                let t = self.ut(b);
                self.pts[by] += t.hp as i32 + 1;
            }
        }
    }
    fn resolve(&mut self, a: usize, b: usize) {
        let me = self.us[a].own as usize;
        let n = self.nd(a, b);
        let ar = self.arm(b);
        let mut dmg;
        if self.dice() {
            dmg = 0;
            for _ in 0..n {
                let v = die(&mut self.rng, 6) as i32 + 1;
                if v > ar {
                    dmg += if v == 6 && self.sp.fl.contains('x') {
                        2
                    } else {
                        1
                    };
                }
            }
        } else {
            dmg = (n - ar).max(1);
        }
        let mut dead: Vec<usize> = Vec::new();
        let (ax, ay, bx, by) = (self.us[a].x, self.us[a].y, self.us[b].x, self.us[b].y);
        self.last = format!(
            "{} lovește {} din {}: {} pierderi",
            self.ut(a).name,
            self.ut(b).name,
            cell(bx, by),
            dmg
        );
        if dmg > 0 {
            if self.has(a, 'B') {
                let around = self.nbrs(bx, by);
                for (x, y) in around {
                    if let Some(k) = self.at(x, y) {
                        if k != a && k != b {
                            self.hurt(k, 1, me, &mut dead);
                        }
                    }
                }
            }
            if (self.sp.fl.contains('p') || self.has(a, 'K')) && self.dist((ax, ay), (bx, by)) == 1
            {
                let mut bestc: Option<(i8, i8)> = None;
                let mut bd = self.dist((ax, ay), (bx, by));
                for (x, y) in self.nbrs(bx, by) {
                    let d = self.dist((ax, ay), (x, y));
                    if d > bd && self.at(x, y).is_none() && self.ter(x, y) != b'#' {
                        bd = d;
                        bestc = Some((x, y));
                    }
                }
                match bestc {
                    Some((x, y)) => {
                        self.us[b].x = x;
                        self.us[b].y = y;
                    }
                    None => dmg += 1,
                }
            }
            self.hurt(b, dmg, me, &mut dead);
        }
        dead.sort_unstable();
        for k in dead.into_iter().rev() {
            self.us.remove(k);
        }
        self.check();
    }
    fn count(&self, p: usize) -> usize {
        self.us.iter().filter(|u| u.own as usize == p).count()
    }
    fn hpsum(&self, p: usize) -> i32 {
        self.us
            .iter()
            .filter(|u| u.own as usize == p)
            .map(|u| u.hp as i32)
            .sum()
    }
    fn check(&mut self) {
        if self.over.is_some() {
            return;
        }
        let (c0, c1) = (self.count(0), self.count(1));
        if c0 == 0 || c1 == 0 {
            self.over = Some(if self.sp.goal == 2 || c0 == c1 {
                by_score(self.pts[0], self.pts[1])
            } else if c0 == 0 {
                Outcome::Win(Player::Two)
            } else {
                Outcome::Win(Player::One)
            });
            return;
        }
        if self.sp.target > 0 && (self.pts[0] >= self.sp.target || self.pts[1] >= self.sp.target) {
            self.over = Some(by_score(self.pts[0], self.pts[1]));
            return;
        }
        if self.turn >= self.sp.turns {
            let s = by_score(self.pts[0], self.pts[1]);
            self.over = Some(if s == Outcome::Draw {
                by_score(self.hpsum(0), self.hpsum(1))
            } else {
                s
            });
        }
    }
    fn goal_row(&self, p: usize) -> i8 {
        if p == 0 {
            self.sp.h - 1
        } else {
            0
        }
    }
    fn end_turn(&mut self) {
        let me = self.side;
        // vindecare
        let meds: Vec<(i8, i8)> = (0..self.us.len())
            .filter(|&i| self.us[i].own as usize == me && self.has(i, 'M'))
            .map(|i| (self.us[i].x, self.us[i].y))
            .collect();
        for k in 0..self.us.len() {
            if self.us[k].own as usize == me {
                let mx = self.ut(k).hp;
                let p = (self.us[k].x, self.us[k].y);
                if self.us[k].hp < mx && meds.iter().any(|&m| m != p && self.dist(m, p) == 1) {
                    self.us[k].hp += 1;
                }
            }
        }
        if self.sp.goal == 1 {
            let objs: Vec<(i8, i8)> = (0..self.sp.h)
                .flat_map(|y| (0..self.sp.w).map(move |x| (x, y)))
                .filter(|&(x, y)| self.ter(x, y) == b'*')
                .collect();
            for o in objs {
                let (mut f, mut e) = (0, 0);
                for u in &self.us {
                    if self.dist((u.x, u.y), o) <= 1 {
                        if u.own as usize == me {
                            f += 1;
                        } else {
                            e += 1;
                        }
                    }
                }
                if f > e {
                    self.pts[me] += 1;
                }
            }
        }
        for u in self.us.iter_mut() {
            if u.own as usize == me {
                u.used = 0;
                u.mvd = 0;
            }
        }
        self.side = 1 - me;
        self.act_left = self.sp.act;
        if self.side == 0 {
            self.turn += 1;
        }
        self.check();
    }
    fn dprog(&self, p: usize) -> i32 {
        // apropiere de scop (mai mic = mai bine)
        let mut tot = 0;
        for i in 0..self.us.len() {
            let u = &self.us[i];
            if u.own as usize != p {
                continue;
            }
            let pos = (u.x, u.y);
            let d = match self.sp.goal {
                2 => (self.goal_row(p) - u.y).abs() as i32,
                1 => (0..self.sp.h)
                    .flat_map(|y| (0..self.sp.w).map(move |x| (x, y)))
                    .filter(|&(x, y)| self.ter(x, y) == b'*')
                    .map(|c| self.dist(pos, c))
                    .min()
                    .unwrap_or(0),
                _ => self
                    .us
                    .iter()
                    .filter(|w| w.own as usize != p)
                    .map(|w| self.dist(pos, (w.x, w.y)))
                    .min()
                    .unwrap_or(0),
            };
            tot += d.min(12);
        }
        tot
    }
}

impl Mini for Mn {
    fn side(&self) -> Player {
        pl(self.side)
    }
    fn depth(&self) -> u32 {
        if self.dice() {
            0
        } else {
            1
        }
    }
    fn moves(&self) -> Vec<String> {
        if self.pend.is_some() {
            return vec!["tira".into()];
        }
        let mut v = Vec::new();
        for i in 0..self.us.len() {
            if !self.can_act(i) {
                continue;
            }
            let u = &self.us[i];
            if u.used & 1 == 0 {
                for (x, y) in self.reach(i) {
                    v.push(format!("mv {}>{}", cell(u.x, u.y), cell(x, y)));
                }
            }
            for j in 0..self.us.len() {
                if self.can_hit(i, j) {
                    v.push(format!(
                        "atq {}>{}",
                        cell(u.x, u.y),
                        cell(self.us[j].x, self.us[j].y)
                    ));
                }
            }
        }
        v.push("fin".into());
        v
    }
    fn step(&mut self, m: &str) {
        if m == "tira" {
            let (a, b) = self.pend.take().unwrap();
            self.resolve(a, b);
            return;
        }
        if m == "fin" {
            self.end_turn();
            return;
        }
        let (kind, rest) = m.split_once(' ').unwrap();
        let (f, t) = rest.split_once('>').unwrap();
        let (f, t) = (parse_cell(f), parse_cell(t));
        let i = self.at(f.0, f.1).unwrap();
        if self.us[i].used == 0 {
            self.act_left -= 1;
        }
        if kind == "mv" {
            let d = self.dist(f, t) as i8;
            self.us[i].x = t.0;
            self.us[i].y = t.1;
            self.us[i].used |= 1;
            self.us[i].mvd = d;
            if self.sp.goal == 2 && t.1 == self.goal_row(self.side) {
                self.us.remove(i);
                self.pts[self.side] += 2;
                self.check();
            }
        } else {
            let j = self.at(t.0, t.1).unwrap();
            self.us[i].used |= 2;
            if self.dice() {
                self.pend = Some((i, j));
            } else {
                self.resolve(i, j);
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        self.over
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let mat = |q: usize| -> i32 {
            self.us
                .iter()
                .filter(|u| u.own as usize == q)
                .map(|u| u.hp as i32 * 10 + self.sp.uts[u.t as usize].dmg as i32 * 3)
                .sum::<i32>()
                + self.pts[q] * 25
        };
        let mut v = mat(a) - mat(1 - a) - (self.dprog(a) - self.dprog(1 - a));
        if let Some((x, y)) = self.pend {
            let e = self.ev(x, y);
            v += if self.us[x].own as usize == a { e } else { -e };
        }
        v
    }
    fn show(&self) -> String {
        let sp = self.sp;
        let mut s = format!(
            "{} — runda {}/{} | puncte J1 {} / J2 {} | la mutare: {} ({} activări libere)\n",
            sp.title,
            self.turn + 1,
            sp.turns,
            self.pts[0],
            self.pts[1],
            pname(pl(self.side)),
            self.act_left
        );
        s.push_str("    ");
        for x in 0..sp.w {
            s.push_str(&format!(" {} ", (b'a' + x as u8) as char));
        }
        s.push('\n');
        for y in 0..sp.h {
            s.push_str(&format!("{:>2}  ", y + 1));
            if sp.hex && y % 2 == 1 {
                s.push_str("  ");
            }
            for x in 0..sp.w {
                match self.at(x, y) {
                    Some(i) => {
                        let u = &self.us[i];
                        let c = self.ut(i).sym;
                        let c = if u.own == 0 {
                            c.to_ascii_uppercase()
                        } else {
                            c
                        };
                        s.push_str(&format!("{}{} ", c, u.hp));
                        if sp.hex {
                            s.push(' ');
                        }
                    }
                    None => {
                        s.push_str(&format!("{}  ", self.ter(x, y) as char));
                        if sp.hex {
                            s.push(' ');
                        }
                    }
                }
            }
            s.push('\n');
        }
        for p in 0..2 {
            s.push_str(&format!("{}:", pname(pl(p))));
            for (i, u) in self.us.iter().enumerate() {
                if u.own as usize == p {
                    s.push_str(&format!(
                        " {}@{} {}/{}{}",
                        self.ut(i).name,
                        cell(u.x, u.y),
                        u.hp,
                        self.ut(i).hp,
                        if u.used & 2 != 0 { "*" } else { "" }
                    ));
                }
            }
            s.push('\n');
        }
        s.push_str("Tipuri (simbol hp mv raza zaruri/forță armură):");
        for t in sp.uts {
            s.push_str(&format!(
                " {}={} {}/{}/{}/{}/{}{}{};",
                t.sym,
                t.name,
                t.hp,
                t.mv,
                t.rng,
                t.dmg,
                t.arm,
                if t.fl.is_empty() { "" } else { " " },
                t.fl
            ));
        }
        s.push('\n');
        if let Some((a, b)) = self.pend {
            s.push_str(&format!(
                "Atac în curs: {} → {} (urmează tira)\n",
                self.ut(a).name,
                self.ut(b).name
            ));
        }
        if !self.last.is_empty() {
            s.push_str(&format!("Ultima: {}\n", self.last));
        }
        s.push_str("Mutări: mv a1>b2 | atq a1>b2 | tira | fin\n");
        s
    }
}

#[cfg(test)]
pub fn check_spec(sp: &'static MSpec) {
    let w = sp.w as usize;
    let rows: Vec<&str> = sp.map.split('/').collect();
    assert_eq!(rows.len(), sp.h as usize, "{}", sp.title);
    for r in &rows {
        assert_eq!(r.len(), w, "{}: rând greșit {r}", sp.title);
    }
    let m = Mn::with_seed(sp, 1);
    for (i, u) in m.us.iter().enumerate() {
        assert!(m.ter(u.x, u.y) != b'#', "{}: unitate în zid", sp.title);
        assert!(m.at(u.x, u.y) == Some(i), "{}: unități suprapuse", sp.title);
    }
    use polytabletop_core::rng::Rng;
    use polytabletop_core::GameObj;
    for seed in 1..=3u64 {
        let mut g = B(Mn::with_seed(sp, seed));
        let mut rng = Rng::new(seed * 77);
        let mut n = 0;
        while GameObj::outcome(&g).is_none() && n < 4000 {
            let lm = GameObj::legal_moves(&g);
            assert!(!lm.is_empty(), "fără mutări\n{}", g.0.show());
            let m = lm[rng.below(lm.len())].clone();
            GameObj::play(&mut g, &m).unwrap();
            n += 1;
        }
        assert!(
            GameObj::outcome(&g).is_some(),
            "{}: partida nu s-a terminat\n{}",
            sp.title,
            g.0.show()
        );
    }
    // AI legal, partida cu AI se termină
    let mut g = B(Mn::with_seed(sp, 5));
    let mut rng = Rng::new(1005);
    let mut n = 0;
    while GameObj::outcome(&g).is_none() && n < 4000 {
        let lm = GameObj::legal_moves(&g);
        let mv = if GameObj::to_move(&g) == Player::One {
            let a = GameObj::ai_move(&g, rng.next_u64()).expect("AI fără mutare");
            assert!(lm.contains(&a), "AI ilegal {a}");
            a
        } else {
            lm[rng.below(lm.len())].clone()
        };
        GameObj::play(&mut g, &mv).unwrap();
        n += 1;
    }
    assert!(
        GameObj::outcome(&g).is_some(),
        "{}: partida cu AI nu s-a terminat\n{}",
        sp.title,
        g.0.show()
    );
    // determinism
    let run = |seed: u64| {
        let mut g = B(Mn::with_seed(sp, seed));
        let mut rng = Rng::new(5);
        for _ in 0..60 {
            if GameObj::outcome(&g).is_some() {
                break;
            }
            let lm = GameObj::legal_moves(&g);
            let m = lm[rng.below(lm.len())].clone();
            GameObj::play(&mut g, &m).unwrap();
        }
        g.0.show()
    };
    assert_eq!(run(9), run(9));
    // AI-ul nu citește generatorul de zaruri
    let mut a = Mn::with_seed(sp, 3);
    advance(&mut a, 6, |_| false);
    no_peek(&a, |h| h.rng ^= 0xDEAD_BEEF_1234_5679);
}
