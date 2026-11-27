//! Familia „urmărire” (variante originale C4, 2 jucători).
//! * `gheata_alunecoasa`: pe rând, un pește fuge spre ieșire și un pisoi îl urmărește pe un coridor de gheață;
//!   `desli k` (1–3): cu cât distanța e mai mare, cu atât alunecarea poate ieși mai scurtă (rezultatul se află la
//!   mutarea forțată `arata`, zar re-însămânțat la `clone()`). Peștele ieșit ia 2, prins dă pisoiului 3; rolurile alternează, 4 runde.
//! * `fantoma_fugara`: un fantomă se plimbă pe un inel de 12 camere; fiecare își mută urmăritorul (`mut k` înainte,
//!   `inv k` înapoi, 1–3 camere); cine aterizează pe fantomă o prinde (+1, primul la 3); fantoma fuge la mutarea forțată `fuge`.
use crate::c4_util::*;

pub struct Spec {
    pub id: &'static str,
    pub title: &'static str,
}

#[derive(Clone)]
pub struct Urm {
    spec: &'static Spec,
    dice: Dice,
    ice: bool,
    // gheață: poziții
    cp: i32,
    fp: i32,
    // fantomă
    pos: [i32; 2],
    ghost: i32,
    round: u32,
    turn: usize,
    pts: [i32; 2],
    pend: Option<i32>,
    mover_done: bool,
    plies: u32,
    last: String,
}

const EXIT: i32 = 10;
const RING: i32 = 12;

impl Urm {
    pub fn new(spec: &'static Spec) -> Self {
        Self::with_seed(spec, seed_now())
    }
    pub fn with_seed(spec: &'static Spec, seed: u64) -> Self {
        let ice = spec.id == "gheata_alunecoasa";
        let mut u = Urm {
            spec,
            dice: Dice::new(seed),
            ice,
            cp: 0,
            fp: 3,
            pos: [0, 6],
            ghost: 3,
            round: 0,
            turn: 0,
            pts: [0; 2],
            pend: None,
            mover_done: false,
            plies: 0,
            last: String::new(),
        };
        u.begin_round();
        u
    }
    fn fish(&self) -> usize {
        // peștele: J1 în rundele pare, J2 în cele impare; el mută primul
        (self.round % 2) as usize
    }
    fn begin_round(&mut self) {
        self.cp = 0;
        self.fp = 3;
        self.turn = self.fish();
    }
    fn p_ok(k: i32) -> i32 {
        match k {
            1 => 90,
            2 => 70,
            _ => 50,
        }
    }
    fn dist(a: i32, b: i32) -> i32 {
        let d = (a - b).rem_euclid(RING);
        d.min(RING - d)
    }
}

impl Mini for Urm {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        pl(self.turn)
    }
    fn moves(&self) -> Vec<String> {
        if self.pend.is_some() {
            return vec!["arata".into()];
        }
        if self.ice {
            return (1..=3).map(|k| format!("desli {k}")).collect();
        }
        if self.mover_done {
            return vec!["fuge".into()];
        }
        let mut v = Vec::new();
        for k in 1..=3 {
            v.push(format!("mut {k}"));
            v.push(format!("inv {k}"));
        }
        v
    }
    fn step(&mut self, m: &str) {
        let me = self.turn;
        if self.ice {
            if let Some(k) = m.strip_prefix("desli ") {
                self.pend = Some(k.parse().unwrap());
                return;
            }
            let k = self.pend.take().unwrap();
            let d = if chance(&mut self.dice, Self::p_ok(k)) {
                k
            } else {
                k - 1
            };
            let fish = me == self.fish();
            if fish {
                self.fp += d;
                if self.fp >= EXIT {
                    self.pts[me] += 2;
                    self.last = "peștele a ieșit".into();
                    self.round += 1;
                    self.begin_round();
                    return;
                }
            } else {
                self.cp += d;
                if self.cp >= self.fp {
                    self.pts[me] += 3;
                    self.last = "pisoiul a prins peștele".into();
                    self.round += 1;
                    self.begin_round();
                    return;
                }
            }
            self.turn = 1 - me;
            self.plies += 1;
            return;
        }
        if m == "fuge" {
            let step = self.dice.below(4) as i32 - 1;
            // fantoma se îndepărtează de cel mai apropiat urmăritor cu probabilitate, altfel la întâmplare
            let near = if Self::dist(self.ghost, self.pos[0]) <= Self::dist(self.ghost, self.pos[1])
            {
                0
            } else {
                1
            };
            let away = if (self.ghost - self.pos[near]).rem_euclid(RING) < RING / 2 {
                1
            } else {
                -1
            };
            let dirn = if self.dice.below(3) == 0 { -away } else { away };
            self.ghost = (self.ghost + dirn * (step.abs() + 1)).rem_euclid(RING);
            self.mover_done = false;
            self.turn = 1 - me;
            self.plies += 1;
            return;
        }
        let (v, k) = m.split_once(' ').unwrap();
        let k: i32 = k.parse().unwrap();
        let sgn = if v == "mut" { 1 } else { -1 };
        self.pos[me] = (self.pos[me] + sgn * k).rem_euclid(RING);
        if self.pos[me] == self.ghost {
            self.pts[me] += 1;
            self.ghost = (self.pos[me] + 6).rem_euclid(RING);
            self.last = format!("J{} a prins fantoma", me + 1);
        }
        self.mover_done = true;
    }
    fn result(&self) -> Option<Outcome> {
        if self.pend.is_some() {
            return None;
        }
        if self.ice {
            if self.round >= 4 || self.plies > 300 {
                return Some(pts_outcome(self.pts));
            }
        } else if self.pts[0] >= 3 || self.pts[1] >= 3 || self.plies > 400 {
            return Some(pts_outcome(self.pts));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let mut v = 300 * (self.pts[a] - self.pts[1 - a]);
        if self.ice {
            let fish_a = self.fish() == a;
            let gap = self.fp - self.cp;
            let prog = self.fp;
            v += if fish_a {
                10 * prog + 12 * gap
            } else {
                -10 * prog - 12 * gap
            };
            if let Some(k) = self.pend {
                let sign = if self.turn == a { 1 } else { -1 };
                v += sign * Self::p_ok(k) / 4;
            }
        } else {
            v += 20
                * (Self::dist(self.ghost, self.pos[1 - a]) - Self::dist(self.ghost, self.pos[a]));
        }
        v
    }
    fn show(&self) -> String {
        let mut s = format!("{}\n", self.spec.title);
        if self.ice {
            s.push_str(&format!(
                "Runda {}/4. Pește: J{}. Pește la {} (ieșire {EXIT}), pisoi la {}. Puncte J1 {}, J2 {}\n",
                self.round + 1,
                self.fish() + 1,
                self.fp,
                self.cp,
                self.pts[0],
                self.pts[1]
            ));
        } else {
            s.push_str(&format!(
                "Inel de {RING} camere (0..{}). Fantoma: {}. J1 la {}, J2 la {}. Puncte J1 {}, J2 {} (primul la 3)\n",
                RING - 1,
                self.ghost,
                self.pos[0],
                self.pos[1],
                self.pts[0],
                self.pts[1]
            ));
        }
        if !self.last.is_empty() {
            s.push_str(&format!("Ultima: {}\n", self.last));
        }
        s.push_str(&format!(
            "La mutare: {} (desli k | mut k | inv k | arata | fuge)\n",
            pname(self.side())
        ));
        s
    }
}

pub static SPECS: &[Spec] = &[
    Spec {
        id: "gheata_alunecoasa",
        title: "Pisoiul pe Gheață",
    },
    Spec {
        id: "fantoma_fugara",
        title: "Fantoma Fugară",
    },
];

pub fn make(id: &str) -> Box<dyn polytabletop_core::GameObj> {
    Box::new(B(Urm::new(
        SPECS.iter().find(|s| s.id == id).expect("spec"),
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plays() {
        for s in SPECS {
            check_c(&|sd| Urm::with_seed(s, sd), 800);
        }
    }
}
