//! Familia „cursă pe traseu” (variante originale C4, 2 jucători): zarul se aruncă la mutarea forțată `zar`
//! (zar re-însămânțat la `clone()`), apoi alegi pionul/zarul: `mut k f`. Fiecare joc are traseul, capcanele,
//! scurtăturile și fața specială a zarului proprii (urmăritor, pierdere, reluare, întoarcere, schimb).
use crate::c4_util::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sp {
    /// Fața 0: un urmăritor înaintează cu n; pionii rămași în urma lui revin la start.
    Chase(i32),
    /// Fața 0: pierzi tura.
    Skip,
    /// Fața 0: mai arunci o dată.
    Again,
    /// Fața 0: pionul tău cel mai avansat revine cu n pătrate.
    Back(i32),
    /// Fața 0: schimbi locul pionului tău cel mai avansat cu cel mai avansat al adversarului.
    Swap,
}

pub struct Spec {
    pub id: &'static str,
    pub title: &'static str,
    pub len: i32,
    pub pawns: usize,
    pub ndice: usize,
    /// Fețele zarului (0 = față specială).
    pub faces: &'static [i8],
    pub sp: Sp,
    pub traps: &'static [i32],
    pub boosts: &'static [(i32, i32)],
    pub exact: bool,
    pub hit: bool,
}

#[derive(Clone)]
pub struct Cursa {
    spec: &'static Spec,
    dice: Dice,
    pos: [Vec<i32>; 2],
    hunter: i32,
    turn: usize,
    opts: Vec<i8>,
    rolled: bool,
    plies: u32,
    last: String,
}

impl Cursa {
    pub fn new(spec: &'static Spec) -> Self {
        Self::with_seed(spec, seed_now())
    }
    pub fn with_seed(spec: &'static Spec, seed: u64) -> Self {
        Cursa {
            spec,
            dice: Dice::new(seed),
            pos: [vec![0; spec.pawns], vec![0; spec.pawns]],
            hunter: -2,
            turn: 0,
            opts: Vec::new(),
            rolled: false,
            plies: 0,
            last: String::new(),
        }
    }
    fn landing(&self, who: usize, k: usize, f: i8) -> Option<i32> {
        let p = self.pos[who][k];
        if p >= self.spec.len {
            return None;
        }
        let n = p + f as i32;
        if n > self.spec.len {
            if self.spec.exact {
                return None;
            }
            return Some(self.spec.len);
        }
        Some(n)
    }
    fn lead(&self, who: usize) -> usize {
        let mut b = 0;
        for k in 0..self.spec.pawns {
            if self.pos[who][k] < self.spec.len
                && (self.pos[who][b] >= self.spec.len || self.pos[who][k] > self.pos[who][b])
            {
                b = k;
            }
        }
        b
    }
    fn catch(&mut self) {
        let h = self.hunter;
        for w in 0..2 {
            for k in 0..self.spec.pawns {
                let p = self.pos[w][k];
                if p > 0 && p < self.spec.len && p <= h {
                    self.pos[w][k] = 0;
                }
            }
        }
    }
    fn next_turn(&mut self) {
        self.turn = 1 - self.turn;
        self.rolled = false;
        self.opts.clear();
        self.plies += 1;
    }
    fn home(&self, w: usize) -> usize {
        self.pos[w].iter().filter(|&&p| p >= self.spec.len).count()
    }
    fn progress(&self, w: usize) -> i32 {
        self.pos[w].iter().sum()
    }
}

impl Mini for Cursa {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        pl(self.turn)
    }
    fn moves(&self) -> Vec<String> {
        if !self.rolled {
            return vec!["zar".into()];
        }
        let mut v = Vec::new();
        let mut fs = self.opts.clone();
        fs.sort();
        fs.dedup();
        for f in fs {
            let mut seen: Vec<i32> = Vec::new();
            for k in 0..self.spec.pawns {
                // pionii la aceeași poziție sunt echivalenți
                let p = self.pos[self.turn][k];
                if seen.contains(&p) {
                    continue;
                }
                if self.landing(self.turn, k, f).is_some() {
                    seen.push(p);
                    v.push(format!("mut {} {}", k + 1, f));
                }
            }
        }
        if v.is_empty() {
            v.push("pas".into());
        }
        v
    }
    fn step(&mut self, m: &str) {
        let me = self.turn;
        if m == "zar" {
            let mut again = 0;
            loop {
                self.opts.clear();
                let mut skip = false;
                let mut re = false;
                for _ in 0..self.spec.ndice {
                    let f = self.spec.faces[self.dice.below(self.spec.faces.len())];
                    if f > 0 {
                        self.opts.push(f);
                        continue;
                    }
                    match self.spec.sp {
                        Sp::Chase(n) => {
                            self.hunter += n;
                            self.catch();
                            self.last = "urmăritorul înaintează".into();
                        }
                        Sp::Skip => skip = true,
                        Sp::Again => re = true,
                        Sp::Back(n) => {
                            let b = self.lead(me);
                            if self.pos[me][b] < self.spec.len {
                                self.pos[me][b] = (self.pos[me][b] - n).max(0);
                            }
                        }
                        Sp::Swap => {
                            let (a, b) = (self.lead(me), self.lead(1 - me));
                            if self.pos[me][a] < self.spec.len
                                && self.pos[1 - me][b] < self.spec.len
                            {
                                let t = self.pos[me][a];
                                self.pos[me][a] = self.pos[1 - me][b];
                                self.pos[1 - me][b] = t;
                            }
                        }
                    }
                }
                if skip {
                    self.opts.clear();
                }
                if re && again < 3 {
                    again += 1;
                    continue;
                }
                break;
            }
            self.rolled = true;
            return;
        }
        if m == "pas" {
            self.next_turn();
            return;
        }
        let mut it = m.split(' ').skip(1);
        let k: usize = it.next().unwrap().parse::<usize>().unwrap() - 1;
        let f: i8 = it.next().unwrap().parse().unwrap();
        let mut n = self.landing(me, k, f).unwrap();
        if n < self.spec.len {
            if self.spec.traps.contains(&n) {
                n = 0;
            } else if let Some(&(_, b)) = self.spec.boosts.iter().find(|&&(a, _)| a == n) {
                n = b;
            }
        }
        self.pos[me][k] = n;
        if self.spec.hit && n > 0 && n < self.spec.len {
            for j in 0..self.spec.pawns {
                if self.pos[1 - me][j] == n {
                    self.pos[1 - me][j] = 0;
                    break;
                }
            }
        }
        self.next_turn();
    }
    fn result(&self) -> Option<Outcome> {
        for w in 0..2 {
            if self.home(w) == self.spec.pawns {
                return Some(Outcome::Win(pl(w)));
            }
        }
        if self.plies >= 600 {
            return Some(pts_outcome([self.progress(0), self.progress(1)]));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let mut v = 10 * (self.progress(a) - self.progress(1 - a))
            + 400 * (self.home(a) as i32 - self.home(1 - a) as i32);
        if let Sp::Chase(_) = self.spec.sp {
            // pionii prea aproape de urmăritor sunt în pericol
            for w in 0..2 {
                for &x in &self.pos[w] {
                    if x > 0 && x < self.spec.len && x - self.hunter <= 3 {
                        v += if w == a { -15 } else { 15 };
                    }
                }
            }
        }
        v
    }
    fn show(&self) -> String {
        let mut s = format!("{} — traseu 0..{}\n", self.spec.title, self.spec.len);
        for w in 0..2 {
            let ps: Vec<String> = self.pos[w].iter().map(|p| p.to_string()).collect();
            s.push_str(&format!(
                "J{}: {} (acasă {}/{})\n",
                w + 1,
                ps.join(" "),
                self.home(w),
                self.spec.pawns
            ));
        }
        if let Sp::Chase(_) = self.spec.sp {
            s.push_str(&format!("Urmăritorul: {}\n", self.hunter.max(0)));
        }
        if !self.spec.traps.is_empty() {
            s.push_str(&format!("Capcane: {:?}\n", self.spec.traps));
        }
        if !self.spec.boosts.is_empty() {
            s.push_str(&format!("Scurtături: {:?}\n", self.spec.boosts));
        }
        if self.rolled {
            s.push_str(&format!("Zar: {:?}\n", self.opts));
        }
        s.push_str(&format!(
            "La mutare: {} (zar | mut k f | pas)\n",
            pname(self.side())
        ));
        s
    }
}

macro_rules! sp {
    ($id:expr, $t:expr, $len:expr, $pw:expr, $nd:expr, $f:expr, $sp:expr, $tr:expr, $bo:expr, $ex:expr, $hit:expr) => {
        Spec {
            id: $id,
            title: $t,
            len: $len,
            pawns: $pw,
            ndice: $nd,
            faces: $f,
            sp: $sp,
            traps: $tr,
            boosts: $bo,
            exact: $ex,
            hit: $hit,
        }
    };
}

pub static SPECS: &[Spec] = &[
    sp!(
        "iepuras_galop",
        "Iepurașul și Găurile",
        24,
        1,
        1,
        &[1, 1, 2, 2, 3, 3],
        Sp::Skip,
        &[6, 11, 17],
        &[],
        false,
        false
    ),
    sp!(
        "soricei_pe_branza",
        "Șoricei pe Brânză",
        22,
        2,
        1,
        &[1, 2, 2, 3, 3, 0],
        Sp::Chase(3),
        &[],
        &[(7, 12)],
        false,
        false
    ),
    sp!(
        "paianjen_pe_fir",
        "Păianjenul de pe Fir",
        20,
        1,
        1,
        &[1, 2, 2, 3, 3, 0],
        Sp::Back(4),
        &[],
        &[(5, 9), (13, 16)],
        true,
        true
    ),
    sp!(
        "viermi_in_mar",
        "Viermele din Măr",
        18,
        2,
        1,
        &[1, 1, 2, 2, 3, 0],
        Sp::Again,
        &[9],
        &[(4, 8)],
        false,
        true
    ),
    sp!(
        "purcei_cursa_mare",
        "Cursa Purceilor",
        26,
        3,
        1,
        &[1, 2, 3, 3, 4, 0],
        Sp::Swap,
        &[],
        &[],
        false,
        false
    ),
    sp!(
        "stafia_din_castel",
        "Stafia din Castel",
        20,
        2,
        1,
        &[1, 2, 2, 3, 4, 0],
        Sp::Chase(2),
        &[],
        &[],
        false,
        true
    ),
    sp!(
        "zaruri_grabite",
        "Zarurile Grăbite",
        28,
        2,
        2,
        &[1, 2, 3, 4, 5, 6],
        Sp::Skip,
        &[14],
        &[(8, 12)],
        false,
        false
    ),
];

pub fn make(id: &str) -> Box<dyn polytabletop_core::GameObj> {
    Box::new(B(Cursa::new(
        SPECS.iter().find(|s| s.id == id).expect("spec"),
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plays() {
        for s in SPECS {
            check_c(&|sd| Cursa::with_seed(s, sd), 800);
        }
    }
}
