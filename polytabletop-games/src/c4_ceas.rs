//! Familia „împreună contra clepsidrei” (variante originale C4, 2 jucători): jucătorii lucrează în echipă
//! la sarcini comune cât timp o amenințare înaintează (+1 la fiecare rundă, +1 la fața 1 a zarului).
//! Zarul se aruncă la mutarea forțată `zar` (re-însămânțat la `clone()`). Dacă amenințarea ajunge la maxim,
//! pierdeți amândoi (remiză); dacă toate sarcinile sunt gata, câștigă cine a contribuit mai mult (egal = remiză).
//! Moduri: `Direct` (`lucr t`: zarul se adaugă sarcinii) și `Craft` (`culeg r` adună resurse, `constr t` le cheltuiește).
use crate::c4_util::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Direct,
    Craft,
}

pub struct Spec {
    pub id: &'static str,
    pub title: &'static str,
    pub mode: Mode,
    /// Direct: necesarul fiecărei sarcini. Craft: (resursă 0..2, cantitate).
    pub tasks: &'static [(u8, u8)],
    pub threat_max: i32,
    pub names: &'static [&'static str],
    pub res: &'static [&'static str],
}

#[derive(Clone)]
pub struct Ceas {
    spec: &'static Spec,
    dice: Dice,
    prog: Vec<u8>,
    stock: [u8; 3],
    contrib: [i32; 2],
    threat: i32,
    turn: usize,
    d: Option<u8>,
    plies: u32,
}

impl Ceas {
    pub fn new(spec: &'static Spec) -> Self {
        Self::with_seed(spec, seed_now())
    }
    pub fn with_seed(spec: &'static Spec, seed: u64) -> Self {
        Ceas {
            spec,
            dice: Dice::new(seed),
            prog: vec![0; spec.tasks.len()],
            stock: [0; 3],
            contrib: [0; 2],
            threat: 0,
            turn: 0,
            d: None,
            plies: 0,
        }
    }
    fn need(&self, t: usize) -> u8 {
        match self.spec.mode {
            Mode::Direct => self.spec.tasks[t].1,
            Mode::Craft => 1,
        }
    }
    fn done_task(&self, t: usize) -> bool {
        self.prog[t] >= self.need(t)
    }
    fn all_done(&self) -> bool {
        (0..self.prog.len()).all(|t| self.done_task(t))
    }
    fn total_left(&self) -> i32 {
        (0..self.prog.len())
            .map(|t| (self.need(t) as i32 - self.prog[t] as i32).max(0))
            .sum()
    }
}

impl Mini for Ceas {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        pl(self.turn)
    }
    fn moves(&self) -> Vec<String> {
        if self.d.is_none() {
            return vec!["zar".into()];
        }
        let mut v = Vec::new();
        match self.spec.mode {
            Mode::Direct => {
                for t in 0..self.prog.len() {
                    if !self.done_task(t) {
                        v.push(format!("lucr {}", t + 1));
                    }
                }
            }
            Mode::Craft => {
                for r in 0..3 {
                    v.push(format!("culeg {}", r + 1));
                }
                for t in 0..self.prog.len() {
                    let (r, q) = self.spec.tasks[t];
                    if !self.done_task(t) && self.stock[r as usize] >= q {
                        v.push(format!("constr {}", t + 1));
                    }
                }
            }
        }
        v
    }
    fn step(&mut self, m: &str) {
        let me = self.turn;
        if m == "zar" {
            let d = self.dice.d6();
            if d == 1 {
                self.threat += 1;
            }
            self.d = Some(d);
            return;
        }
        let d = self.d.take().unwrap();
        let (verb, arg) = m.split_once(' ').unwrap();
        let x: usize = arg.parse::<usize>().unwrap() - 1;
        match verb {
            "lucr" => {
                let add = d.min(self.need(x) - self.prog[x]);
                self.prog[x] += add;
                self.contrib[me] += add as i32;
            }
            "culeg" => {
                self.stock[x] = (self.stock[x] + d).min(9);
                self.contrib[me] += 1;
            }
            _ => {
                let (r, q) = self.spec.tasks[x];
                self.stock[r as usize] -= q;
                self.prog[x] = 1;
                self.contrib[me] += 2 * q as i32;
            }
        }
        if me == 1 {
            self.threat += 1;
        }
        self.turn = 1 - me;
        self.plies += 1;
    }
    fn result(&self) -> Option<Outcome> {
        if self.all_done() {
            return Some(pts_outcome(self.contrib));
        }
        if self.threat >= self.spec.threat_max || self.plies > 300 {
            return Some(Outcome::Draw);
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        100 * (self.contrib[a] - self.contrib[1 - a]) - 20 * self.total_left() - 10 * self.threat
    }
    fn show(&self) -> String {
        let mut s = format!(
            "{} — amenințare {}/{}\n",
            self.spec.title, self.threat, self.spec.threat_max
        );
        for t in 0..self.prog.len() {
            match self.spec.mode {
                Mode::Direct => s.push_str(&format!(
                    "{}. {}: {}/{}\n",
                    t + 1,
                    self.spec.names[t],
                    self.prog[t],
                    self.spec.tasks[t].1
                )),
                Mode::Craft => s.push_str(&format!(
                    "{}. {} (costă {} {}): {}\n",
                    t + 1,
                    self.spec.names[t],
                    self.spec.tasks[t].1,
                    self.spec.res[self.spec.tasks[t].0 as usize],
                    if self.prog[t] > 0 {
                        "gata"
                    } else {
                        "de făcut"
                    }
                )),
            }
        }
        if self.spec.mode == Mode::Craft {
            s.push_str(&format!(
                "Stoc: {} {}, {} {}, {} {}\n",
                self.stock[0],
                self.spec.res[0],
                self.stock[1],
                self.spec.res[1],
                self.stock[2],
                self.spec.res[2]
            ));
        }
        s.push_str(&format!(
            "Contribuție: J1 {}, J2 {}\n",
            self.contrib[0], self.contrib[1]
        ));
        if let Some(d) = self.d {
            s.push_str(&format!("Zar: {d}\n"));
        }
        s.push_str(&format!(
            "La mutare: {} (zar | lucr t | culeg r | constr t)\n",
            pname(self.side())
        ));
        s
    }
}

pub static SPECS: &[Spec] = &[
    Spec {
        id: "copii_contra_stafiilor",
        title: "Copiii Apără Căsuța",
        mode: Mode::Direct,
        tasks: &[(0, 7), (0, 8), (0, 9)],
        threat_max: 9,
        names: &[
            "Monstrul din dulap",
            "Monstrul din pivniță",
            "Monstrul de sub pat",
        ],
        res: &[],
    },
    Spec {
        id: "satul_din_piatra",
        title: "Satul din Piatră și Lemn",
        mode: Mode::Craft,
        tasks: &[(0, 4), (1, 5), (2, 4), (0, 6)],
        threat_max: 11,
        names: &["Coliba", "Focul", "Câmpul", "Podul"],
        res: &["lemn", "piatră", "grâu"],
    },
    Spec {
        id: "corabii_din_vale",
        title: "Corăbiile din Vale",
        mode: Mode::Direct,
        tasks: &[(0, 10), (0, 12), (0, 14)],
        threat_max: 11,
        names: &["Barca roșie", "Barca albastră", "Barca verde"],
        res: &[],
    },
];

pub fn make(id: &str) -> Box<dyn polytabletop_core::GameObj> {
    Box::new(B(Ceas::new(
        SPECS.iter().find(|s| s.id == id).expect("spec"),
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plays() {
        for s in SPECS {
            check_c(&|sd| Ceas::with_seed(s, sd), 800);
        }
    }
}
