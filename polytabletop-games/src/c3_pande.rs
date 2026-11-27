//! Motor cooperativ „focare pe hartă” (lista C3, variante originale, 2 jucători în echipă).
//!
//! Pe un graf de locuri cresc pericole (0..3). Fiecare jucător are `ap` acțiuni pe tură: `mers v`
//! (la un vecin), `cur` (scade pericolul din locul curent), `rit` (într-un loc cu laborator și
//! pericol 0: un ritual împlinit), `pas`. După tura fiecărui jucător urmează faza forțată `trag`:
//! se trag `rate` locuri la întâmplare, iar fiecare primește +1 pericol; peste 3 izbucnește
//! (vecinii primesc +1, cel mult o dată pe lanț). Echipa câștigă cu `goal` ritualuri, pierde la
//! `maxout` izbucniri sau când se termină rundele (remiză, nu există adversar).
//! Zarul/tragerea nu apar decât în mutarea forțată, deci AI-ul nu „vede” viitorul.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

pub struct PSpec {
    pub title: &'static str,
    pub place: &'static str,
    pub edges: &'static str,
    pub n: usize,
    pub labs: &'static [usize],
    pub goal: u32,
    pub rate: u32,
    pub maxout: u32,
    pub rounds: u32,
    pub ap: u8,
    pub start: [usize; 2],
    pub init: &'static [usize],
}

pub static SPECS: [PSpec; 4] = [
    PSpec {
        title: "Focar Zero",
        place: "oraș",
        edges: "0-1,1-2,2-3,3-4,4-5,5-0,0-6,3-6,1-7,4-7",
        n: 8,
        labs: &[2, 5, 6],
        goal: 5,
        rate: 2,
        maxout: 4,
        rounds: 28,
        ap: 3,
        start: [6, 6],
        init: &[1, 4, 7],
    },
    PSpec {
        title: "Hotarul Căzut",
        place: "provincie",
        edges: "0-1,1-2,2-3,3-4,4-5,5-6,6-7,7-0,1-8,5-8,3-9,7-9,8-9",
        n: 10,
        labs: &[8, 9],
        goal: 5,
        rate: 2,
        maxout: 5,
        rounds: 30,
        ap: 3,
        start: [8, 9],
        init: &[0, 2, 4, 6],
    },
    PSpec {
        title: "Garnizoana din Cuib",
        place: "sector",
        edges: "0-1,1-2,0-3,1-4,2-5,3-4,4-5,3-6,4-7,5-8,6-7,7-8",
        n: 9,
        labs: &[4, 6],
        goal: 4,
        rate: 2,
        maxout: 3,
        rounds: 26,
        ap: 3,
        start: [4, 4],
        init: &[0, 2, 8],
    },
    PSpec {
        title: "Galeria Prăbușită",
        place: "galerie",
        edges: "0-1,1-2,2-3,3-4,1-5,5-6,3-6,6-7,7-8,0-8",
        n: 9,
        labs: &[0, 4, 7],
        goal: 5,
        rate: 3,
        maxout: 5,
        rounds: 32,
        ap: 4,
        start: [0, 4],
        init: &[2, 5, 6],
    },
];

#[derive(Clone)]
pub struct Pande {
    sp: &'static PSpec,
    adj: Vec<Vec<usize>>,
    thr: Vec<u8>,
    pos: [usize; 2],
    done: u32,
    out: u32,
    ap_left: u8,
    round: u32,
    side: usize,
    draw: bool,
    rng: Rng,
}

pub fn new(i: usize) -> Pande {
    with_seed(i, seed_now())
}

pub fn with_seed(i: usize, seed: u64) -> Pande {
    let sp = &SPECS[i];
    let mut adj = vec![Vec::new(); sp.n];
    for e in sp.edges.split(',') {
        let (a, b) = e.split_once('-').unwrap();
        let (a, b): (usize, usize) = (a.parse().unwrap(), b.parse().unwrap());
        adj[a].push(b);
        adj[b].push(a);
    }
    let mut thr = vec![0u8; sp.n];
    for &k in sp.init {
        thr[k] = 1;
    }
    Pande {
        sp,
        adj,
        thr,
        pos: sp.start,
        done: 0,
        out: 0,
        ap_left: sp.ap,
        round: 0,
        side: 0,
        draw: false,
        rng: Rng::new(seed),
    }
}

impl Pande {
    fn infect(&mut self, v: usize) {
        let mut seen = vec![false; self.sp.n];
        let mut st = vec![v];
        while let Some(x) = st.pop() {
            if self.thr[x] < 3 {
                self.thr[x] += 1;
            } else if !seen[x] {
                seen[x] = true;
                self.out += 1;
                for &y in &self.adj[x] {
                    if !seen[y] {
                        st.push(y);
                    }
                }
            }
        }
    }
    fn dist_from(&self, s: usize) -> Vec<u32> {
        let mut d = vec![u32::MAX; self.sp.n];
        d[s] = 0;
        let mut q = std::collections::VecDeque::from([s]);
        while let Some(x) = q.pop_front() {
            for &y in &self.adj[x] {
                if d[y] == u32::MAX {
                    d[y] = d[x] + 1;
                    q.push_back(y);
                }
            }
        }
        d
    }
}

impl Mini for Pande {
    fn side(&self) -> Player {
        if self.side == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        if self.draw {
            return vec!["trag".into()];
        }
        let p = self.pos[self.side];
        let mut v = Vec::new();
        for &y in &self.adj[p] {
            v.push(format!("mers {}", y + 1));
        }
        if self.thr[p] > 0 {
            v.push("cur".into());
        }
        if self.sp.labs.contains(&p) && self.thr[p] == 0 {
            v.push("rit".into());
        }
        v.push("pas".into());
        v
    }
    fn step(&mut self, m: &str) {
        if self.draw {
            for _ in 0..self.sp.rate {
                let v = self.rng.below(self.sp.n);
                self.infect(v);
            }
            self.draw = false;
            self.side = 1 - self.side;
            self.ap_left = self.sp.ap;
            self.round += 1;
            return;
        }
        let p = self.pos[self.side];
        if let Some(t) = m.strip_prefix("mers ") {
            self.pos[self.side] = t.parse::<usize>().unwrap() - 1;
        } else if m == "cur" {
            self.thr[p] -= 1;
        } else if m == "rit" {
            self.done += 1;
        } else {
            self.ap_left = 1;
        }
        self.ap_left -= 1;
        if self.ap_left == 0 {
            self.draw = true;
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.done >= self.sp.goal {
            Some(Outcome::Win(Player::One))
        } else if self.out >= self.sp.maxout || self.round >= self.sp.rounds {
            Some(Outcome::Draw)
        } else {
            None
        }
    }
    fn eval(&self, _p: Player) -> i32 {
        let total: i32 = self.thr.iter().map(|&t| t as i32).sum();
        let mut e = self.done as i32 * 300 - self.out as i32 * 120 - total * 6;
        if self.done < self.sp.goal {
            let d = self.dist_from(self.pos[self.side]);
            let near = self
                .sp
                .labs
                .iter()
                .map(|&l| d[l] as i32 * 3 + self.thr[l] as i32 * 8)
                .min()
                .unwrap_or(0);
            e -= near;
            // locul curent cu pericol: bonus pentru a-l curăța
            e -= self.thr[self.pos[self.side]] as i32 * 4;
        }
        e
    }
    fn show(&self) -> String {
        let mut s = format!(
            "{} — runda {}/{} | ritualuri {}/{} | izbucniri {}/{}\n",
            self.sp.title,
            self.round + 1,
            self.sp.rounds,
            self.done,
            self.sp.goal,
            self.out,
            self.sp.maxout
        );
        for i in 0..self.sp.n {
            let mut t = String::new();
            if self.pos[0] == i {
                t.push_str(" [J1]");
            }
            if self.pos[1] == i {
                t.push_str(" [J2]");
            }
            if self.sp.labs.contains(&i) {
                t.push_str(" (laborator)");
            }
            let nb: Vec<String> = self.adj[i].iter().map(|x| (x + 1).to_string()).collect();
            s.push_str(&format!(
                "{} {}: pericol {} — vecini {}{}\n",
                self.sp.place,
                i + 1,
                self.thr[i],
                nb.join(","),
                t
            ));
        }
        s.push_str(&format!(
            "La mutare: {} ({} acțiuni rămase) — mers v | cur | rit | pas\n",
            pname(self.side()),
            self.ap_left
        ));
        s
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        for i in 0..4 {
            let f: fn(u64) -> Pande = match i {
                0 => |s| with_seed(0, s),
                1 => |s| with_seed(1, s),
                2 => |s| with_seed(2, s),
                _ => |s| with_seed(3, s),
            };
            check(f, 3000);
        }
    }
}
