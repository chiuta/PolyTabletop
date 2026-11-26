//! Motor „plăci pe tablă” (lista B4): plăci colorate din mână (publică), grupuri de aceeași culoare.
//! `shared` = o tablă comună; altfel câte o tablă personală. `minscore` = scorul final e cea mai
//! slabă dintre cele 4 culori (plus un bonus mic din totalul culorilor).
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

pub struct GSpec {
    pub title: &'static str,
    pub n: usize,
    pub shared: bool,
    pub minscore: bool,
    pub tiles: usize,
    pub depth: u32,
}

#[derive(Clone)]
pub struct G {
    sp: &'static GSpec,
    b: [Vec<i8>; 2],
    sc: [[i32; 4]; 2],
    hand: [Vec<i8>; 2],
    queue: Vec<i8>,
    side: Player,
}

impl G {
    pub fn new(sp: &'static GSpec) -> Self {
        Self::with_seed(sp, seed_now())
    }
    pub fn with_seed(sp: &'static GSpec, seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut queue: Vec<i8> = (0..sp.tiles).map(|i| (i % 4) as i8).collect();
        shuf(&mut queue, &mut rng);
        let cells = sp.n * sp.n;
        let mut g = G {
            sp,
            b: [vec![-1; cells], vec![-1; cells]],
            sc: [[0; 4]; 2],
            hand: [Vec::new(), Vec::new()],
            queue,
            side: Player::One,
        };
        for p in 0..2 {
            g.draw(p);
        }
        g
    }
    fn draw(&mut self, p: usize) {
        while self.hand[p].len() < 3 && !self.queue.is_empty() {
            let t = self.queue.remove(0);
            self.hand[p].push(t);
        }
    }
    fn bi(&self, p: usize) -> usize {
        if self.sp.shared {
            0
        } else {
            p
        }
    }
    fn nbrs(&self, i: usize) -> Vec<usize> {
        let n = self.sp.n;
        let (r, c) = (i / n, i % n);
        let mut v = Vec::new();
        if r > 0 {
            v.push(i - n);
        }
        if r + 1 < n {
            v.push(i + n);
        }
        if c > 0 {
            v.push(i - 1);
        }
        if c + 1 < n {
            v.push(i + 1);
        }
        v
    }
    fn legal_cells(&self, p: usize) -> Vec<usize> {
        let b = &self.b[self.bi(p)];
        let cells = self.sp.n * self.sp.n;
        let any = b.iter().any(|&x| x >= 0);
        let mut v = Vec::new();
        for i in 0..cells {
            if b[i] >= 0 {
                continue;
            }
            if !any || self.nbrs(i).iter().any(|&j| b[j] >= 0) {
                v.push(i);
            }
        }
        v
    }
    fn group(&self, bi: usize, i: usize) -> i32 {
        let col = self.b[bi][i];
        let mut seen = vec![false; self.b[bi].len()];
        let mut st = vec![i];
        seen[i] = true;
        let mut n = 0;
        while let Some(x) = st.pop() {
            n += 1;
            for j in self.nbrs(x) {
                if !seen[j] && self.b[bi][j] == col {
                    seen[j] = true;
                    st.push(j);
                }
            }
        }
        n
    }
    fn total(&self, p: usize) -> i32 {
        if self.sp.minscore {
            self.sc[p].iter().copied().min().unwrap() * 4 + self.sc[p].iter().sum::<i32>() / 4
        } else {
            self.sc[p].iter().sum()
        }
    }
}

impl Mini for G {
    fn side(&self) -> Player {
        self.side
    }
    fn depth(&self) -> u32 {
        self.sp.depth
    }
    fn moves(&self) -> Vec<String> {
        let me = pi(self.side);
        let n = self.sp.n;
        let cells = self.legal_cells(me);
        let mut v = Vec::new();
        if cells.is_empty() {
            return vec!["pas".into()];
        }
        let mut seen = [false; 4];
        for (k, &t) in self.hand[me].iter().enumerate() {
            if seen[t as usize] {
                continue;
            }
            seen[t as usize] = true;
            for &i in &cells {
                v.push(format!("pune {},{} {}", i / n, i % n, k));
            }
        }
        if v.is_empty() {
            v.push("pas".into());
        }
        v
    }
    fn step(&mut self, m: &str) {
        let me = pi(self.side);
        if m != "pas" {
            let mut it = m[5..].split(' ');
            let rc = it.next().unwrap();
            let k: usize = it.next().unwrap().parse().unwrap();
            let (r, c) = rc.split_once(',').unwrap();
            let i = r.parse::<usize>().unwrap() * self.sp.n + c.parse::<usize>().unwrap();
            let t = self.hand[me].remove(k);
            let bi = self.bi(me);
            self.b[bi][i] = t;
            self.sc[me][t as usize] += self.group(bi, i);
            self.draw(me);
        }
        self.side = self.side.other();
    }
    fn result(&self) -> Option<Outcome> {
        let stuck = |p: usize| self.hand[p].is_empty() || self.legal_cells(p).is_empty();
        if stuck(0) && stuck(1) {
            return Some(by_score(self.total(0), self.total(1)));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        4 * (self.total(a) - self.total(1 - a))
            + (self.sc[a].iter().sum::<i32>() - self.sc[1 - a].iter().sum::<i32>())
    }
    fn show(&self) -> String {
        let n = self.sp.n;
        let mut s = format!("{} — tabla {}×{}\n", self.sp.title, n, n);
        for bi in 0..(if self.sp.shared { 1 } else { 2 }) {
            if !self.sp.shared {
                s.push_str(&format!("Tabla J{}:\n", bi + 1));
            }
            for r in 0..n {
                let row: Vec<String> = (0..n)
                    .map(|c| match self.b[bi][r * n + c] {
                        -1 => ".".to_string(),
                        t => ((b'A' + t as u8) as char).to_string(),
                    })
                    .collect();
                s.push_str(&format!("  {}\n", row.join(" ")));
            }
        }
        for p in 0..2 {
            let h: Vec<String> = self.hand[p]
                .iter()
                .map(|&t| ((b'A' + t as u8) as char).to_string())
                .collect();
            s.push_str(&format!(
                "J{}: mână {} | culori {:?} | scor {}\n",
                p + 1,
                h.join(""),
                self.sc[p],
                self.total(p)
            ));
        }
        s.push_str(&format!(
            "La mutare: {} (pune r,c k — plăcuța k din mână, de la 0 | pas)\n",
            pname(self.side)
        ));
        s
    }
}
