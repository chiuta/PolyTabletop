//! Motor comun „zaruri plasate pe locuri” (lista C6): nouă variante originale, 2 jucători.
//!
//! La începutul rundei zarurile se aruncă (mutarea forțată `tira`), apoi jucătorii pun pe rând
//! zaruri pe locuri (`pon v s`) sau spun `pas`. Moduri de rezolvare: `Majority` (cine are mai multe
//! zaruri pe loc; egalitate = nimeni), `First` (efect imediat, loc exclusiv), `Sum` (suma cea mai mare pe loc),
//! `Fill(k)` (primul cu `k` zaruri valide pe loc îl închide). După `ronda` se împart recompensele.
//! Zarurile și recompensele aleatoare se extrag doar în `tira`; `eval` nu citește generatorul.
#![allow(clippy::needless_range_loop, clippy::manual_is_multiple_of)]
use crate::zr_util::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Rule {
    Any,
    Exact(u8),
    Min(u8),
    Max(u8),
    Even,
    Odd,
}

impl Rule {
    pub fn ok(self, v: u8) -> bool {
        // v în 1..=6
        match self {
            Rule::Any => true,
            Rule::Exact(x) => v == x,
            Rule::Min(x) => v >= x,
            Rule::Max(x) => v <= x,
            Rule::Even => v % 2 == 0,
            Rule::Odd => v % 2 == 1,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    Majority,
    First,
    Sum,
    Fill(u8),
}

pub struct Site {
    pub name: &'static str,
    pub rule: Rule,
    pub cap: u8,
    pub give: [i8; 3],
    pub vp: i8,
    pub cost: [i8; 3],
}

pub const fn st(
    name: &'static str,
    rule: Rule,
    cap: u8,
    give: [i8; 3],
    vp: i8,
    cost: [i8; 3],
) -> Site {
    Site {
        name,
        rule,
        cap,
        give,
        vp,
        cost,
    }
}

pub struct Cfg {
    pub title: &'static str,
    pub rn: [&'static str; 3],
    pub sites: &'static [Site],
    pub mode: Mode,
    pub dice: usize,
    pub rounds: u32,
    /// pune toate zarurile cu aceeași valoare; se aruncă din nou la fiecare tură (stil „cazinou”)
    pub group: bool,
    /// recompense aleatoare 1..=5 × `give[0]` la fiecare rundă
    pub rand_reward: bool,
    /// conversie finală resursă -> puncte (0 = nu contează)
    pub div: [i8; 3],
    pub income: [i8; 3],
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ph {
    Roll,
    Play,
    End,
}

#[derive(Clone)]
pub struct Loc {
    cfg: &'static Cfg,
    rng: u64,
    round: u32,
    ph: Ph,
    side: usize,
    first: usize,
    hand: [Vec<u8>; 2],
    left: [usize; 2],
    done: [bool; 2],
    /// pe fiecare loc: zarurile puse (jucător, valoare 1..=6)
    occ: Vec<Vec<(u8, u8)>>,
    closed: Vec<bool>,
    bonus: Vec<i32>,
    res: [[i32; 3]; 2],
    vp: [i32; 2],
}

impl Loc {
    pub fn new(cfg: &'static Cfg) -> Self {
        Self::with_seed(cfg, seed_now())
    }
    pub fn with_seed(cfg: &'static Cfg, seed: u64) -> Self {
        let n = cfg.sites.len();
        let mut rng = seed_state(seed);
        let bonus = (0..n)
            .map(|_| {
                1 + if cfg.rand_reward {
                    die(&mut rng, 5) as i32
                } else {
                    0
                }
            })
            .collect();
        Loc {
            cfg,
            rng,
            round: 0,
            ph: Ph::Roll,
            side: 0,
            first: 0,
            hand: [Vec::new(), Vec::new()],
            left: [cfg.dice; 2],
            done: [false; 2],
            occ: vec![Vec::new(); n],
            closed: vec![false; n],
            bonus,
            res: [[0; 3]; 2],
            vp: [0; 2],
        }
    }
    fn mine(&self, s: usize, p: usize) -> usize {
        self.occ[s].iter().filter(|x| x.0 as usize == p).count()
    }
    fn can_pay(&self, p: usize, c: &[i8; 3]) -> bool {
        (0..3).all(|k| self.res[p][k] >= c[k] as i32)
    }
    fn reward(&mut self, p: usize, s: usize) {
        let site = &self.cfg.sites[s];
        let mult = if self.cfg.rand_reward {
            self.bonus[s]
        } else {
            1
        };
        for k in 0..3 {
            self.res[p][k] += site.give[k] as i32 * mult;
        }
        self.vp[p] += site.vp as i32;
    }
    fn site_value(&self, s: usize) -> i32 {
        let site = &self.cfg.sites[s];
        let mult = if self.cfg.rand_reward {
            self.bonus[s]
        } else {
            1
        };
        let mut v = 1000 * site.vp as i32;
        for k in 0..3 {
            let d = self.cfg.div[k] as i32;
            if d > 0 {
                v += 1000 * site.give[k] as i32 * mult / d;
            } else {
                v += 150 * site.give[k] as i32 * mult;
            }
        }
        v
    }
    fn score_x(&self, p: usize) -> i32 {
        let mut e = 1000 * self.vp[p];
        for k in 0..3 {
            let d = self.cfg.div[k] as i32;
            e += if d > 0 {
                1000 * self.res[p][k] / d
            } else {
                150 * self.res[p][k]
            };
        }
        e
    }
    fn final_score(&self, p: usize) -> i32 {
        let mut s = self.vp[p];
        for k in 0..3 {
            let d = self.cfg.div[k] as i32;
            if d > 0 {
                s += self.res[p][k] / d;
            }
        }
        s
    }
    fn legal_site(&self, p: usize, v: u8, s: usize) -> bool {
        let site = &self.cfg.sites[s];
        if self.closed[s] || !site.rule.ok(v) {
            return false;
        }
        match self.cfg.mode {
            Mode::First => self.occ[s].len() < site.cap as usize && self.can_pay(p, &site.cost),
            Mode::Fill(_) => self.mine(s, p) < site.cap as usize,
            _ => self.mine(s, p) < site.cap as usize && self.can_pay(p, &site.cost),
        }
    }
    fn advance_side(&mut self) {
        let o = 1 - self.side;
        if !self.done[o] {
            self.side = o;
        }
        if self.cfg.group {
            self.ph = Ph::Roll;
        }
        if self.done[0] && self.done[1] {
            self.ph = Ph::End;
        }
    }
    fn standing(&self, s: usize) -> i32 {
        // în miimi de punct, din perspectiva jucătorului 0
        let site = &self.cfg.sites[s];
        let val = self.site_value(s);
        let sum = |p: usize| -> i32 {
            self.occ[s]
                .iter()
                .filter(|x| x.0 as usize == p)
                .map(|x| x.1 as i32)
                .sum()
        };
        match self.cfg.mode {
            Mode::First => 0,
            Mode::Majority => {
                let (a, b) = (self.mine(s, 0), self.mine(s, 1));
                if self.closed[s] || a == b {
                    0
                } else if a > b {
                    val
                } else {
                    -val
                }
            }
            Mode::Sum => {
                let (a, b) = (sum(0), sum(1));
                if a == b {
                    0
                } else if a > b {
                    val
                } else {
                    -val
                }
            }
            Mode::Fill(k) => {
                if self.closed[s] {
                    return 0;
                }
                let (a, b) = (self.mine(s, 0) as i32, self.mine(s, 1) as i32);
                let _ = site;
                val * (a - b) / (k as i32 + 1)
            }
        }
    }
}

impl Mini for Loc {
    fn side(&self) -> Player {
        pl(self.side)
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            Ph::Roll => vec!["tira".into()],
            Ph::End => vec!["ronda".into()],
            Ph::Play => {
                let p = self.side;
                let mut v = vec!["pas".to_string()];
                let mut vals: Vec<u8> = self.hand[p].clone();
                vals.sort_unstable();
                vals.dedup();
                for &val in &vals {
                    for s in 0..self.cfg.sites.len() {
                        if self.legal_site(p, val + 1, s) {
                            v.push(format!("pon {} {}", val + 1, s + 1));
                        }
                    }
                }
                v
            }
        }
    }
    fn step(&mut self, m: &str) {
        let c = self.cfg;
        match self.ph {
            Ph::Roll => {
                if c.group {
                    let p = self.side;
                    self.hand[p] = (0..self.left[p]).map(|_| die(&mut self.rng, 6)).collect();
                    if self.hand[p].is_empty() {
                        self.done[p] = true;
                    }
                    self.ph = Ph::Play;
                    if self.done[p] {
                        self.advance_side();
                    }
                } else {
                    for p in 0..2 {
                        self.hand[p] = (0..c.dice).map(|_| die(&mut self.rng, 6)).collect();
                    }
                    self.ph = Ph::Play;
                }
            }
            Ph::End => {
                // rezolvare
                for s in 0..c.sites.len() {
                    let sum = |p: usize| -> i32 {
                        self.occ[s]
                            .iter()
                            .filter(|x| x.0 as usize == p)
                            .map(|x| x.1 as i32)
                            .sum()
                    };
                    match c.mode {
                        Mode::First | Mode::Fill(_) => {}
                        Mode::Majority => {
                            let (a, b) = (self.mine(s, 0), self.mine(s, 1));
                            if a != b {
                                self.reward(if a > b { 0 } else { 1 }, s);
                            }
                        }
                        Mode::Sum => {
                            let (a, b) = (sum(0), sum(1));
                            if a != b {
                                self.reward(if a > b { 0 } else { 1 }, s);
                            }
                        }
                    }
                }
                self.round += 1;
                for p in 0..2 {
                    for k in 0..3 {
                        self.res[p][k] += c.income[k] as i32;
                    }
                }
                for s in 0..c.sites.len() {
                    self.occ[s].clear();
                    self.closed[s] = false;
                }
                self.left = [c.dice; 2];
                self.done = [false; 2];
                self.first = 1 - self.first;
                self.side = self.first;
                self.ph = Ph::Roll;
                if c.rand_reward {
                    for s in 0..c.sites.len() {
                        self.bonus[s] = 1 + die(&mut self.rng, 5) as i32;
                    }
                }
            }
            Ph::Play => {
                let p = self.side;
                if m == "pas" {
                    self.done[p] = true;
                    self.hand[p].clear();
                    self.advance_side();
                    return;
                }
                let mut it = m.split(' ').skip(1);
                let val: u8 = it.next().unwrap().parse().unwrap();
                let s: usize = it.next().unwrap().parse::<usize>().unwrap() - 1;
                let site = &c.sites[s];
                let n = if c.group {
                    self.hand[p].iter().filter(|&&d| d == val - 1).count()
                } else {
                    1
                };
                for _ in 0..n {
                    let k = self.hand[p].iter().position(|&d| d == val - 1).unwrap();
                    self.hand[p].remove(k);
                    self.occ[s].push((p as u8, val));
                }
                self.left[p] -= n;
                for k in 0..3 {
                    self.res[p][k] -= site.cost[k] as i32 * n as i32;
                }
                match c.mode {
                    Mode::First => {
                        self.reward(p, s);
                    }
                    Mode::Fill(k) if self.mine(s, p) >= k as usize => {
                        self.reward(p, s);
                        self.closed[s] = true;
                        self.occ[s].clear();
                    }
                    _ => {}
                }
                if self.left[p] == 0 {
                    self.done[p] = true;
                }
                self.advance_side();
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.round >= self.cfg.rounds {
            Some(by_score(self.final_score(0), self.final_score(1)))
        } else {
            None
        }
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let mut e = self.score_x(a) - self.score_x(1 - a);
        let mut st = 0;
        for s in 0..self.cfg.sites.len() {
            st += self.standing(s);
        }
        e += if a == 0 { st } else { -st };
        e
    }
    fn show(&self) -> String {
        let c = self.cfg;
        let mut s = format!(
            "{} — runda {}/{} | la mutare {}\nPuncte: J1 {} (scor {}), J2 {} (scor {}) | {}: J1 {:?}, J2 {:?}\n",
            c.title,
            (self.round + 1).min(c.rounds),
            c.rounds,
            pname(pl(self.side)),
            self.vp[0],
            self.final_score(0),
            self.vp[1],
            self.final_score(1),
            c.rn.join("/"),
            self.res[0],
            self.res[1]
        );
        for (i, site) in c.sites.iter().enumerate() {
            let o: Vec<String> = self.occ[i]
                .iter()
                .map(|x| format!("J{}:{}", x.0 + 1, x.1))
                .collect();
            s.push_str(&format!(
                "  loc {}: {} ({:?}, dă {:?}{}, +{}p){} [{}]\n",
                i + 1,
                site.name,
                site.rule,
                site.give,
                if c.rand_reward {
                    format!(" ×{}", self.bonus[i])
                } else {
                    String::new()
                },
                site.vp,
                if self.closed[i] { " ÎNCHIS" } else { "" },
                o.join(" ")
            ));
        }
        s.push_str(&format!(
            "Zaruri în mână: J1 [{}], J2 [{}]\n",
            dstr(&self.hand[0]),
            dstr(&self.hand[1])
        ));
        s.push_str("Mutări: tira | pon v loc | pas | ronda\n");
        s
    }
}

macro_rules! cfgs {
    ($($name:ident => $e:expr;)*) => { $(pub static $name: Cfg = $e;)* };
}

use Mode::*;
use Rule::*;

cfgs! {
    // Majoritate pe valori, cu aruncare la fiecare tură: „cazinou”
    BANCNOTE => Cfg { title: "Bancnote și zaruri", rn: ["mii", "-", "-"], mode: Majority, dice: 8, rounds: 4, group: true, rand_reward: true,
        div: [1, 0, 0], income: [0, 0, 0],
        sites: &[st("Masa 1", Exact(1), 8, [10, 0, 0], 0, [0; 3]), st("Masa 2", Exact(2), 8, [10, 0, 0], 0, [0; 3]), st("Masa 3", Exact(3), 8, [10, 0, 0], 0, [0; 3]),
                 st("Masa 4", Exact(4), 8, [10, 0, 0], 0, [0; 3]), st("Masa 5", Exact(5), 8, [10, 0, 0], 0, [0; 3]), st("Masa 6", Exact(6), 8, [10, 0, 0], 0, [0; 3])] };
    // Sume pe clădiri
    RASCRUCE => Cfg { title: "Satul de la răscruce", rn: ["aur", "grâu", "-"], mode: Sum, dice: 5, rounds: 6, group: false, rand_reward: false,
        div: [3, 2, 0], income: [0, 0, 0],
        sites: &[st("Banca", Min(4), 3, [3, 0, 0], 0, [0; 3]), st("Moara", Odd, 3, [0, 3, 0], 0, [0; 3]), st("Hanul", Even, 3, [1, 1, 0], 1, [0; 3]),
                 st("Primăria", Any, 3, [0, 0, 0], 3, [0; 3]), st("Fântâna", Max(3), 3, [2, 1, 0], 0, [0; 3])] };
    // Completare de cerințe
    SOARTA => Cfg { title: "Soarta e aruncată", rn: ["piatră", "lemn", "-"], mode: Fill(2), dice: 6, rounds: 6, group: false, rand_reward: false,
        div: [2, 2, 0], income: [0, 0, 0],
        sites: &[st("Forum", Exact(1), 2, [0, 0, 0], 2, [0; 3]), st("Terme", Exact(2), 2, [2, 0, 0], 1, [0; 3]), st("Apeduct", Exact(3), 2, [0, 2, 0], 1, [0; 3]),
                 st("Arenă", Exact(4), 2, [0, 0, 0], 3, [0; 3]), st("Templu", Exact(5), 2, [2, 2, 0], 0, [0; 3]), st("Drum", Exact(6), 2, [4, 0, 0], 0, [0; 3])] };
    // Piețe cu loc exclusiv, efect imediat
    BAZAR => Cfg { title: "Bazarul persan", rn: ["monede", "mărfuri", "mirodenii"], mode: First, dice: 6, rounds: 6, group: false, rand_reward: false,
        div: [4, 2, 1], income: [0, 0, 0],
        sites: &[st("Taraba 1", Exact(1), 1, [3, 0, 0], 0, [0; 3]), st("Taraba 2", Exact(2), 1, [0, 1, 0], 0, [0; 3]), st("Taraba 3", Exact(3), 1, [0, 0, 1], 0, [0; 3]),
                 st("Taraba 4", Exact(4), 1, [2, 1, 0], 0, [0; 3]), st("Taraba 5", Exact(5), 1, [0, 2, 0], 1, [0, 0, 0]), st("Taraba 6", Exact(6), 1, [0, 0, 2], 0, [1, 0, 0]),
                 st("Caravana", Any, 1, [0, 0, 0], 2, [0, 1, 0]), st("Hanul de sus", Min(5), 2, [1, 1, 1], 0, [1, 0, 0])] };
    // Sume cu cost și venit
    BRESLE => Cfg { title: "Cetatea breslelor", rn: ["galbeni", "ucenici", "-"], mode: Sum, dice: 6, rounds: 5, group: false, rand_reward: false,
        div: [3, 1, 0], income: [3, 0, 0],
        sites: &[st("Breasla fierarilor", Min(3), 3, [0, 0, 0], 3, [1, 0, 0]), st("Breasla pânzarilor", Odd, 3, [0, 3, 0], 1, [1, 0, 0]), st("Breasla zidarilor", Even, 3, [0, 0, 0], 4, [2, 0, 0]),
                 st("Piața", Any, 3, [6, 0, 0], 0, [0, 0, 0]), st("Catedrala", Min(5), 3, [0, 0, 0], 6, [2, 0, 0])] };
    // Doc-uri cu resurse; colonizare costisitoare
    DOCURI => Cfg { title: "Șantierul orbital", rn: ["minereu", "combustibil", "colonii"], mode: First, dice: 4, rounds: 8, group: false, rand_reward: false,
        div: [0, 0, 1], income: [1, 0, 0],
        sites: &[st("Doc de minereu", Max(3), 2, [2, 0, 0], 0, [0; 3]), st("Doc de combustibil", Min(4), 2, [0, 2, 0], 0, [0; 3]), st("Uzină", Even, 1, [0, 0, 0], 1, [2, 1, 0]),
                 st("Colonizare", Exact(6), 1, [0, 0, 1], 2, [3, 2, 0]), st("Stație", Odd, 1, [1, 1, 0], 0, [0; 3])] };
    // Scopuri cu tipare diverse
    PLANURI => Cfg { title: "Schițe de construcție", rn: ["piatră", "lemn", "-"], mode: Fill(3), dice: 7, rounds: 5, group: false, rand_reward: false,
        div: [2, 2, 0], income: [0, 0, 0],
        sites: &[st("Turn", Min(5), 3, [0, 0, 0], 3, [0; 3]), st("Pod", Max(2), 3, [2, 0, 0], 1, [0; 3]), st("Fântână", Even, 3, [0, 2, 0], 1, [0; 3]), st("Casă", Odd, 3, [0, 0, 0], 2, [0; 3])] };
    TINTA => Cfg { title: "Ținta fixată", rn: ["-", "-", "-"], mode: Fill(2), dice: 6, rounds: 8, group: false, rand_reward: false,
        div: [0, 0, 0], income: [0, 0, 0],
        sites: &[st("Cartea 1: două de 1", Exact(1), 2, [0; 3], 5, [0; 3]), st("Cartea 2: două de 2", Exact(2), 2, [0; 3], 4, [0; 3]), st("Cartea 3: două de 3", Exact(3), 2, [0; 3], 3, [0; 3]),
                 st("Cartea 4: două de 4", Exact(4), 2, [0; 3], 3, [0; 3]), st("Cartea 5: două de 5", Exact(5), 2, [0; 3], 2, [0; 3]), st("Cartea 6: două de 6", Exact(6), 2, [0; 3], 1, [0; 3])] };
    // Popoare: construcție + armată cu sume
    POPOARE => Cfg { title: "Popoarele și epocile", rn: ["hrană", "oțel", "glorie"], mode: Sum, dice: 5, rounds: 6, group: false, rand_reward: false,
        div: [3, 2, 1], income: [1, 0, 0],
        sites: &[st("Ogoare", Odd, 3, [3, 0, 0], 0, [0; 3]), st("Fierărie", Even, 3, [0, 2, 0], 0, [0; 3]), st("Armată", Min(3), 3, [0, 0, 2], 0, [1, 0, 0]),
                 st("Monument", Any, 3, [0, 0, 0], 3, [0, 1, 0])] };
}

#[cfg(test)]
mod t {
    use super::*;

    fn run(cfg: &'static Cfg, plies: usize) {
        let mk: fn(u64) -> Loc = match cfg.title {
            "Bancnote și zaruri" => |s| Loc::with_seed(&BANCNOTE, s),
            "Satul de la răscruce" => |s| Loc::with_seed(&RASCRUCE, s),
            "Soarta e aruncată" => |s| Loc::with_seed(&SOARTA, s),
            "Bazarul persan" => |s| Loc::with_seed(&BAZAR, s),
            "Cetatea breslelor" => |s| Loc::with_seed(&BRESLE, s),
            "Șantierul orbital" => |s| Loc::with_seed(&DOCURI, s),
            "Schițe de construcție" => |s| Loc::with_seed(&PLANURI, s),
            "Ținta fixată" => |s| Loc::with_seed(&TINTA, s),
            _ => |s| Loc::with_seed(&POPOARE, s),
        };
        check(mk, plies);
        let mut g = mk(4);
        advance(&mut g, 60, |m| m.ph == Ph::Play);
        no_peek(&g, |m| m.rng ^= 0xFEDCBA987);
    }
    #[test]
    fn bancnote() {
        run(&BANCNOTE, 1500);
    }
    #[test]
    fn rascruce() {
        run(&RASCRUCE, 1500);
    }
    #[test]
    fn soarta() {
        run(&SOARTA, 1500);
    }
    #[test]
    fn bazar() {
        run(&BAZAR, 1500);
    }
    #[test]
    fn bresle() {
        run(&BRESLE, 1500);
    }
    #[test]
    fn docuri() {
        run(&DOCURI, 1500);
    }
    #[test]
    fn planuri() {
        run(&PLANURI, 1500);
    }
    #[test]
    fn tinta() {
        run(&TINTA, 1500);
    }
    #[test]
    fn popoare() {
        run(&POPOARE, 1500);
    }
}
