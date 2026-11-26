//! Motor „bătălie tactică pe grilă” (lista B4): trei tipuri de unități (infanterie, cavalerie,
//! arcași), `cmd` ordine pe tură, loviri deterministe (fără zaruri), flancare +1.
//! Medalii = unități distruse; primul la `medals` câștigă. Cu `sections`, ordinele unei ture sunt
//! limitate la o singură treime de tablă (aleasă de prima acțiune).
use crate::brand_util::*;
use polytabletop_core::{Outcome, Player};

pub struct DSpec {
    pub title: &'static str,
    pub rows: usize,
    pub cols: usize,
    pub cmd: u8,
    pub medals: i32,
    pub sections: bool,
    pub turns: u32,
    pub depth: u32,
}

// tip: 0 infanterie (hp2, atac1, deplasare1, raza1), 1 cavalerie (hp2, atac2, dep 2, raza1),
// 2 arcași (hp1, atac1, dep 1, raza 2)
const HP: [i32; 3] = [2, 2, 1];
const ATK: [i32; 3] = [1, 2, 1];
const MV: [i32; 3] = [1, 2, 1];
const RG: [i32; 3] = [1, 1, 2];
const CH: [char; 3] = ['i', 'c', 'a'];

#[derive(Clone, Copy, PartialEq)]
struct U {
    own: u8,
    t: u8,
    hp: i8,
}

#[derive(Clone)]
pub struct D {
    sp: &'static DSpec,
    b: Vec<Option<U>>,
    acted: Vec<bool>,
    medals: [i32; 2],
    cmd_left: u8,
    section: Option<usize>,
    turn: u32,
    side: Player,
}

impl D {
    pub fn new(sp: &'static DSpec) -> Self {
        let mut b = vec![None; sp.rows * sp.cols];
        let layout = [0u8, 2, 0, 1, 0, 2, 0, 1, 0];
        for c in 0..sp.cols {
            let t = layout[c % layout.len()];
            b[c] = Some(U {
                own: 1,
                t,
                hp: HP[t as usize] as i8,
            });
            b[(sp.rows - 1) * sp.cols + c] = Some(U {
                own: 0,
                t,
                hp: HP[t as usize] as i8,
            });
            if c % 2 == 0 {
                b[sp.cols + c] = Some(U {
                    own: 1,
                    t: 0,
                    hp: 2,
                });
                b[(sp.rows - 2) * sp.cols + c] = Some(U {
                    own: 0,
                    t: 0,
                    hp: 2,
                });
            }
        }
        D {
            sp,
            acted: vec![false; sp.rows * sp.cols],
            b,
            medals: [0; 2],
            cmd_left: sp.cmd,
            section: None,
            turn: 0,
            side: Player::One,
        }
    }
    pub fn with_seed(sp: &'static DSpec, _seed: u64) -> Self {
        Self::new(sp)
    }
    fn rc(&self, i: usize) -> (i32, i32) {
        ((i / self.sp.cols) as i32, (i % self.sp.cols) as i32)
    }
    fn idx(&self, r: i32, c: i32) -> Option<usize> {
        (r >= 0 && c >= 0 && (r as usize) < self.sp.rows && (c as usize) < self.sp.cols)
            .then(|| r as usize * self.sp.cols + c as usize)
    }
    fn sec(&self, i: usize) -> usize {
        self.rc(i).1 as usize * 3 / self.sp.cols
    }
    fn dist(&self, a: usize, b: usize) -> i32 {
        let (ar, ac) = self.rc(a);
        let (br, bc) = self.rc(b);
        (ar - br).abs() + (ac - bc).abs()
    }
    fn reach(&self, from: usize, mv: i32) -> Vec<usize> {
        let mut out = Vec::new();
        let (r, c) = self.rc(from);
        for dr in -mv..=mv {
            for dc in -mv..=mv {
                if dr.abs() + dc.abs() > mv || (dr == 0 && dc == 0) {
                    continue;
                }
                if let Some(j) = self.idx(r + dr, c + dc) {
                    if self.b[j].is_none() {
                        // drum liber pe cale (pentru 2 pași: o celulă intermediară liberă)
                        let ok = if dr.abs() + dc.abs() == 2 {
                            let cands = [(dr.signum(), 0), (0, dc.signum())];
                            cands.iter().any(|&(a, b)| {
                                (a != 0 || b != 0)
                                    && (a.abs() <= dr.abs() && b.abs() <= dc.abs())
                                    && self.idx(r + a, c + b).is_some_and(|k| self.b[k].is_none())
                            })
                        } else {
                            true
                        };
                        if ok {
                            out.push(j);
                        }
                    }
                }
            }
        }
        out
    }
    fn flank(&self, from: usize, tgt: usize) -> i32 {
        // +1 dacă ținta are alt dușman al ei lipit de ea (în afară de atacator)
        let (r, c) = self.rc(tgt);
        let t = self.b[tgt].unwrap();
        for (dr, dc) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            if let Some(j) = self.idx(r + dr, c + dc) {
                if j != from {
                    if let Some(u) = self.b[j] {
                        if u.own != t.own {
                            return 1;
                        }
                    }
                }
            }
        }
        0
    }
}

impl Mini for D {
    fn side(&self) -> Player {
        self.side
    }
    fn depth(&self) -> u32 {
        self.sp.depth
    }
    fn moves(&self) -> Vec<String> {
        let me = pi(self.side) as u8;
        let mut v = vec!["fin".to_string()];
        if self.cmd_left == 0 {
            return v;
        }
        for i in 0..self.b.len() {
            let Some(u) = self.b[i] else { continue };
            if u.own != me || self.acted[i] {
                continue;
            }
            if self.sp.sections {
                if let Some(s) = self.section {
                    if self.sec(i) != s {
                        continue;
                    }
                }
            }
            let (r, c) = self.rc(i);
            for j in self.reach(i, MV[u.t as usize]) {
                let (r2, c2) = self.rc(j);
                v.push(format!("mv {r},{c}>{r2},{c2}"));
            }
            for j in 0..self.b.len() {
                if let Some(t) = self.b[j] {
                    if t.own != me && self.dist(i, j) <= RG[u.t as usize] {
                        let (r2, c2) = self.rc(j);
                        v.push(format!("at {r},{c}>{r2},{c2}"));
                    }
                }
            }
        }
        v
    }
    fn step(&mut self, m: &str) {
        let me = pi(self.side);
        if m == "fin" {
            self.cmd_left = 0;
        } else {
            let (a, b) = m[3..].split_once('>').unwrap();
            let pt = |s: &str| {
                let (r, c) = s.split_once(',').unwrap();
                r.parse::<usize>().unwrap() * self.sp.cols + c.parse::<usize>().unwrap()
            };
            let (a, b) = (pt(a), pt(b));
            if self.section.is_none() {
                self.section = Some(self.sec(a));
            }
            if m.starts_with("mv") {
                self.b[b] = self.b[a].take();
                self.acted[b] = true;
            } else {
                let u = self.b[a].unwrap();
                let dmg = ATK[u.t as usize] as i8 + self.flank(a, b) as i8;
                let mut t = self.b[b].unwrap();
                t.hp -= dmg;
                if t.hp <= 0 {
                    self.b[b] = None;
                    self.medals[me] += 1;
                } else {
                    self.b[b] = Some(t);
                }
                self.acted[a] = true;
            }
            self.cmd_left -= 1;
        }
        if self.cmd_left == 0 {
            self.side = self.side.other();
            self.cmd_left = self.sp.cmd;
            self.section = None;
            for x in self.acted.iter_mut() {
                *x = false;
            }
            if self.side == Player::One {
                self.turn += 1;
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        for p in 0..2 {
            if self.medals[p] >= self.sp.medals {
                return Some(Outcome::Win(if p == 0 { Player::One } else { Player::Two }));
            }
        }
        if self.turn >= self.sp.turns {
            return Some(by_score(self.medals[0], self.medals[1]));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let mut s = 30 * (self.medals[a] - self.medals[1 - a]);
        for i in 0..self.b.len() {
            if let Some(u) = self.b[i] {
                let sign = if u.own as usize == a { 1 } else { -1 };
                s += sign * 3 * u.hp as i32;
                // avans spre inamic
                let (r, _) = self.rc(i);
                let adv = if u.own == 0 {
                    self.sp.rows as i32 - 1 - r
                } else {
                    r
                };
                s += sign * adv / 2;
            }
        }
        s
    }
    fn show(&self) -> String {
        let mut s = format!(
            "{} — tura {}/{} | medalii J1 {} J2 {} (prag {})\n",
            self.sp.title,
            self.turn + 1,
            self.sp.turns,
            self.medals[0],
            self.medals[1],
            self.sp.medals
        );
        for r in 0..self.sp.rows {
            let row: Vec<String> = (0..self.sp.cols)
                .map(|c| match self.b[r * self.sp.cols + c] {
                    None => " . ".to_string(),
                    Some(u) => {
                        let ch = CH[u.t as usize];
                        let ch = if u.own == 0 {
                            ch.to_ascii_uppercase()
                        } else {
                            ch
                        };
                        format!(
                            "{}{}{}",
                            ch,
                            u.hp,
                            if self.acted[r * self.sp.cols + c] {
                                "*"
                            } else {
                                " "
                            }
                        )
                    }
                })
                .collect();
            s.push_str(&format!("{r} {}\n", row.join("")));
        }
        s.push_str(&format!(
            "J1 = MAJUSCULE (jos), J2 = minuscule (sus); i infanterie, c cavalerie, a arcași; cifra = viață.\nLa mutare: {} ({} ordine: mv r,c>r,c | at r,c>r,c | fin)\n",
            pname(self.side),
            self.cmd_left
        ));
        s
    }
}
