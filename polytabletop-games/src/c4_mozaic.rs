//! Familia „așază plăci” (variante originale C4, 2 jucători). Piața are 3 plăci publice; mutarea alege și așază
//! într-un singur pas (`j r c o` sau `j r c` pentru plăci de o celulă, `j r c f` cu însoțitor); noua placă se trage
//! din pachet lazy (zar re-însămânțat la `clone()`), deci AI-ul nu cunoaște plăcile viitoare. Informație publică.
//! * `Dom`: domino cu două jumătăți (teren 0..3, ouă 0..2); o placă nouă trebuie să atingă un teren la fel; scor = ouăle.
//! * `Pair`: perechi de animale în parc propriu; scor = 2 puncte pentru fiecare vecinătate ortogonală de același animal.
//! * `Shared`: tablă comună, plăci de o celulă; la așezare iei 1 + vecinii de același fel; însoțitorii (3 pe jucător)
//!   iau la final mărimea grupului lor.
use crate::c4_util::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Dom,
    Pair,
    Shared,
}

pub struct Spec {
    pub id: &'static str,
    pub title: &'static str,
    pub mode: Mode,
    pub w: i32,
    pub h: i32,
    /// Plăci pe jucător.
    pub turns: u32,
    pub names: &'static [&'static str],
}

type Tile = (u8, u8, u8, u8);

#[derive(Clone)]
pub struct Mozaic {
    spec: &'static Spec,
    dice: Dice,
    board: [Vec<Option<(u8, u8)>>; 2],
    market: Vec<Tile>,
    followers: [Vec<(i32, i32)>; 2],
    fleft: [u8; 2],
    placed: [u32; 2],
    turn: usize,
    plies: u32,
    pts: [i32; 2],
}

impl Mozaic {
    pub fn new(spec: &'static Spec) -> Self {
        Self::with_seed(spec, seed_now())
    }
    pub fn with_seed(spec: &'static Spec, seed: u64) -> Self {
        let n = (spec.w * spec.h) as usize;
        let mut s = Mozaic {
            spec,
            dice: Dice::new(seed),
            board: [vec![None; n], vec![None; n]],
            market: Vec::new(),
            followers: [Vec::new(), Vec::new()],
            fleft: [3; 2],
            placed: [0; 2],
            turn: 0,
            plies: 0,
            pts: [0; 2],
        };
        for _ in 0..3 {
            let t = s.draw();
            s.market.push(t);
        }
        s
    }
    fn draw(&mut self) -> Tile {
        let k = |d: &mut Dice| d.below(4) as u8;
        let a = k(&mut self.dice);
        let b = k(&mut self.dice);
        let (ea, eb) = (self.dice.below(3) as u8 % 2, self.dice.below(3) as u8 % 2);
        match self.spec.mode {
            Mode::Dom => (a, ea, b, eb),
            _ => (a, 0, b, 0),
        }
    }
    fn bi(&self, who: usize) -> usize {
        if self.spec.mode == Mode::Shared {
            0
        } else {
            who
        }
    }
    fn at(&self, who: usize, r: i32, c: i32) -> Option<(u8, u8)> {
        if r < 0 || c < 0 || r >= self.spec.h || c >= self.spec.w {
            return None;
        }
        self.board[self.bi(who)][(r * self.spec.w + c) as usize]
    }
    fn free(&self, who: usize, r: i32, c: i32) -> bool {
        r >= 0 && c >= 0 && r < self.spec.h && c < self.spec.w && self.at(who, r, c).is_none()
    }
    fn cells(&self, r: i32, c: i32, o: u8) -> [(i32, i32); 2] {
        match o {
            0 | 2 => [(r, c), (r, c + 1)],
            _ => [(r, c), (r + 1, c)],
        }
    }
    fn nbrs(r: i32, c: i32) -> [(i32, i32); 4] {
        [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)]
    }
    fn touches(&self, who: usize, cells: &[(i32, i32)]) -> bool {
        cells.iter().any(|&(r, c)| {
            Self::nbrs(r, c)
                .iter()
                .any(|&(a, b)| self.at(who, a, b).is_some())
        })
    }
    fn legal(&self, who: usize, t: Tile, r: i32, c: i32, o: u8) -> bool {
        let cs = self.cells(r, c, o);
        if !cs.iter().all(|&(a, b)| self.free(who, a, b)) {
            return false;
        }
        let empty = self.board[self.bi(who)].iter().all(|x| x.is_none());
        match self.spec.mode {
            Mode::Dom => {
                if empty {
                    return true;
                }
                let (t1, t2) = if o < 2 { (t.0, t.2) } else { (t.2, t.0) };
                let ts = [t1, t2];
                (0..2).any(|i| {
                    Self::nbrs(cs[i].0, cs[i].1).iter().any(|&(a, b)| {
                        !cs.contains(&(a, b)) && self.at(who, a, b).map(|x| x.0) == Some(ts[i])
                    })
                })
            }
            Mode::Pair => true,
            Mode::Shared => empty || self.touches(who, &cs[..1]),
        }
    }
    fn group(&self, r: i32, c: i32) -> i32 {
        let t = self.at(0, r, c).map(|x| x.0);
        let mut seen = vec![(r, c)];
        let mut i = 0;
        while i < seen.len() {
            let (a, b) = seen[i];
            for (x, y) in Self::nbrs(a, b) {
                if self.at(0, x, y).map(|v| v.0) == t && !seen.contains(&(x, y)) {
                    seen.push((x, y));
                }
            }
            i += 1;
        }
        seen.len() as i32
    }
    fn score(&self, who: usize) -> i32 {
        match self.spec.mode {
            Mode::Dom => self.board[who].iter().flatten().map(|x| x.1 as i32).sum(),
            Mode::Pair => {
                let mut s = 0;
                for r in 0..self.spec.h {
                    for c in 0..self.spec.w {
                        if let Some(x) = self.at(who, r, c) {
                            if self.at(who, r, c + 1).map(|y| y.0) == Some(x.0) {
                                s += 2;
                            }
                            if self.at(who, r + 1, c).map(|y| y.0) == Some(x.0) {
                                s += 2;
                            }
                        }
                    }
                }
                s
            }
            Mode::Shared => {
                self.pts[who]
                    + self.followers[who]
                        .iter()
                        .map(|&(r, c)| self.group(r, c))
                        .sum::<i32>()
            }
        }
    }
    fn tile_moves(&self, who: usize, j: usize) -> Vec<String> {
        let t = self.market[j];
        let mut v = Vec::new();
        for r in 0..self.spec.h {
            for c in 0..self.spec.w {
                if self.spec.mode == Mode::Shared {
                    if self.legal(who, t, r, c, 0) {
                        v.push(format!("{} {} {}", j + 1, r + 1, c + 1));
                        if self.fleft[who] > 0 {
                            v.push(format!("{} {} {} f", j + 1, r + 1, c + 1));
                        }
                    }
                } else {
                    for o in 0..4u8 {
                        // orientările 2 și 3 contează doar dacă jumătățile diferă
                        if o >= 2 && t.0 == t.2 && t.1 == t.3 {
                            continue;
                        }
                        if self.legal(who, t, r, c, o) {
                            v.push(format!("{} {} {} {}", j + 1, r + 1, c + 1, o));
                        }
                    }
                }
            }
        }
        v
    }
}

impl Mini for Mozaic {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        pl(self.turn)
    }
    fn moves(&self) -> Vec<String> {
        let mut v = Vec::new();
        for j in 0..self.market.len() {
            v.extend(self.tile_moves(self.turn, j));
        }
        if v.is_empty() {
            v.push("pas".into());
        }
        v
    }
    fn step(&mut self, m: &str) {
        let me = self.turn;
        if m != "pas" {
            let p: Vec<&str> = m.split(' ').collect();
            let j: usize = p[0].parse::<usize>().unwrap() - 1;
            let r: i32 = p[1].parse::<i32>().unwrap() - 1;
            let c: i32 = p[2].parse::<i32>().unwrap() - 1;
            let t = self.market.remove(j);
            let bi = self.bi(me);
            let w = self.spec.w;
            if self.spec.mode == Mode::Shared {
                let same = Self::nbrs(r, c)
                    .iter()
                    .filter(|&&(a, b)| self.at(0, a, b).map(|x| x.0) == Some(t.0))
                    .count();
                self.board[0][(r * w + c) as usize] = Some((t.0, 0));
                self.pts[me] += 1 + same as i32;
                if p.len() > 3 {
                    self.followers[me].push((r, c));
                    self.fleft[me] -= 1;
                }
            } else {
                let o: u8 = p[3].parse().unwrap();
                let cs = self.cells(r, c, o);
                let (a, b) = if o < 2 {
                    ((t.0, t.1), (t.2, t.3))
                } else {
                    ((t.2, t.3), (t.0, t.1))
                };
                self.board[bi][(cs[0].0 * w + cs[0].1) as usize] = Some(a);
                self.board[bi][(cs[1].0 * w + cs[1].1) as usize] = Some(b);
            }
            self.placed[me] += 1;
            let n = self.draw();
            self.market.push(n);
        } else {
            // nimic de așezat: se aruncă prima placă
            self.market.remove(0);
            let n = self.draw();
            self.market.push(n);
            self.placed[me] += 1;
        }
        self.turn = 1 - me;
        self.plies += 1;
    }
    fn result(&self) -> Option<Outcome> {
        if self.placed[0] >= self.spec.turns && self.placed[1] >= self.spec.turns {
            return Some(pts_outcome([self.score(0), self.score(1)]));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        100 * (self.score(a) - self.score(1 - a))
    }
    fn show(&self) -> String {
        let mut s = format!(
            "{} — plăci puse J1 {}/{}, J2 {}/{}\n",
            self.spec.title, self.placed[0], self.spec.turns, self.placed[1], self.spec.turns
        );
        let nm = |t: u8| self.spec.names[t as usize % self.spec.names.len()];
        let boards = if self.spec.mode == Mode::Shared { 1 } else { 2 };
        for b in 0..boards {
            if boards == 2 {
                s.push_str(&format!("Tabla J{} (scor {}):\n", b + 1, self.score(b)));
            } else {
                s.push_str(&format!(
                    "Tabla comună (J1 {}, J2 {}):\n",
                    self.score(0),
                    self.score(1)
                ));
            }
            for r in 0..self.spec.h {
                let row: Vec<String> = (0..self.spec.w)
                    .map(|c| match self.at(b, r, c) {
                        Some((t, e)) if e > 0 => format!("{}{}", &nm(t)[..1], e),
                        Some((t, _)) => format!("{} ", &nm(t)[..1]),
                        None => ". ".into(),
                    })
                    .collect();
                s.push_str(&format!("  {}\n", row.join(" ")));
            }
        }
        let ms: Vec<String> = self
            .market
            .iter()
            .enumerate()
            .map(|(i, t)| match self.spec.mode {
                Mode::Shared => format!("{}={}", i + 1, nm(t.0)),
                _ => format!(
                    "{}={}{}+{}{}",
                    i + 1,
                    nm(t.0),
                    if t.1 > 0 {
                        "*".repeat(t.1 as usize)
                    } else {
                        String::new()
                    },
                    nm(t.2),
                    if t.3 > 0 {
                        "*".repeat(t.3 as usize)
                    } else {
                        String::new()
                    }
                ),
            })
            .collect();
        s.push_str(&format!("Piață: {}\n", ms.join(" | ")));
        s.push_str(&format!(
            "La mutare: {} (j r c o | j r c [f] | pas)\n",
            pname(self.side())
        ));
        s
    }
}

pub static SPECS: &[Spec] = &[
    Spec {
        id: "insula_dragonilor",
        title: "Insula Puilor de Dragon",
        mode: Mode::Dom,
        w: 5,
        h: 5,
        turns: 9,
        names: &["Pădure", "Apă", "Câmp", "Munte"],
    },
    Spec {
        id: "parcul_perechilor",
        title: "Parcul cu Perechi",
        mode: Mode::Pair,
        w: 4,
        h: 3,
        turns: 6,
        names: &["Urs", "Girafă", "Zebră", "Leu"],
    },
    Spec {
        id: "cetatea_si_drumul",
        title: "Drumul spre Cetate",
        mode: Mode::Shared,
        w: 5,
        h: 5,
        turns: 10,
        names: &["Drum", "Cetate", "Câmp", "Pădure"],
    },
];

pub fn make(id: &str) -> Box<dyn polytabletop_core::GameObj> {
    Box::new(B(Mozaic::new(
        SPECS.iter().find(|s| s.id == id).expect("spec"),
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plays() {
        for s in SPECS {
            check_c(&|sd| Mozaic::with_seed(s, sd), 800);
        }
    }
}
