//! Familia „capcană ascunsă” (variante originale C4, 2 jucători): norocul se hotărăște lazy, la
//! mutarea forțată `arata`, cu zar re-însămânțat la `clone()`; AI-ul nu vede capcana, doar riscul public.
//! * `Slots` — alegi un loc dintre cele rămase; o capcană ascunsă (uniformă) pierde runda; primul la `target`.
//! * `Bank` — împingi norocul: fiecare acțiune costă ticuri și dă câștig; limita ascunsă e uniformă în [lo, hi];
//!   `stop` bagă potul în scor, depășirea îl pierde.
//! * `Bag` — tragi dintr-un sac cu compoziție publică (bune cu valori, rele); la `lives` rele potul se pierde.
use crate::c4_util::*;

#[derive(Clone, Copy, Debug)]
pub enum Kind {
    Slots {
        n: u8,
        target: i32,
    },
    Bank {
        lo: i32,
        hi: i32,
        acts: &'static [(i32, i32)],
    },
    Bag {
        good: &'static [u8],
        bad: u8,
        lives: u8,
    },
}

pub struct Spec {
    pub id: &'static str,
    pub title: &'static str,
    pub kind: Kind,
    /// Numărul total de ture (Bank/Bag).
    pub turns: u32,
    pub names: &'static [&'static str],
}

#[derive(Clone)]
pub struct Capcana {
    spec: &'static Spec,
    dice: Dice,
    pts: [i32; 2],
    turn: usize,
    pot: i32,
    ticks: i32,
    used: u32,
    bag: Vec<u8>,
    lost: u8,
    pend: Option<usize>,
    done: u32,
    last: String,
}

impl Capcana {
    pub fn new(spec: &'static Spec) -> Self {
        Self::with_seed(spec, seed_now())
    }
    pub fn with_seed(spec: &'static Spec, seed: u64) -> Self {
        let mut s = Capcana {
            spec,
            dice: Dice::new(seed),
            pts: [0; 2],
            turn: 0,
            pot: 0,
            ticks: 0,
            used: 0,
            bag: Vec::new(),
            lost: 0,
            pend: None,
            done: 0,
            last: String::new(),
        };
        s.refill();
        s
    }
    fn refill(&mut self) {
        self.pot = 0;
        self.ticks = 0;
        self.used = 0;
        self.lost = 0;
        if let Kind::Bag { good, bad, .. } = self.spec.kind {
            self.bag = good.to_vec();
            self.bag.push(bad);
        }
    }
    fn over(l: i32, lo: i32) -> i32 {
        (l - lo).max(0)
    }
    /// Probabilitatea (în %) ca mutarea `k` să prăbușească tura, calculată doar din date publice.
    fn hazard(&self, k: usize) -> i32 {
        match self.spec.kind {
            Kind::Slots { n, .. } => 100 / (n as i32 - self.used.count_ones() as i32).max(1),
            Kind::Bank { lo, hi, acts } => {
                let c = acts[k].0;
                let d = (hi - lo + 1 - Self::over(self.ticks, lo)).max(1);
                ((Self::over(self.ticks + c, lo) - Self::over(self.ticks, lo)) * 100 / d)
                    .clamp(0, 100)
            }
            Kind::Bag { lives, .. } => {
                let tot: i32 = self.bag.iter().map(|&x| x as i32).sum();
                let bad = *self.bag.last().unwrap() as i32;
                if self.lost + 1 >= lives && tot > 0 {
                    bad * 100 / tot
                } else {
                    0
                }
            }
        }
    }
    fn gain(&self, k: usize) -> i32 {
        match self.spec.kind {
            Kind::Bank { acts, .. } => acts[k].1,
            Kind::Bag { good, .. } => {
                let tot: i32 = self.bag.iter().map(|&x| x as i32).sum::<i32>().max(1);
                let s: i32 = good
                    .iter()
                    .enumerate()
                    .map(|(i, _)| self.bag[i] as i32 * (i as i32 + 1))
                    .sum();
                s / tot
            }
            _ => 0,
        }
    }
    fn end_turn(&mut self) {
        self.done += 1;
        self.turn = 1 - self.turn;
        self.refill();
    }
}

impl Mini for Capcana {
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
        match self.spec.kind {
            Kind::Slots { n, .. } => (0..n)
                .filter(|i| self.used >> i & 1 == 0)
                .map(|i| format!("taie {}", i + 1))
                .collect(),
            Kind::Bank { acts, .. } => {
                let mut v: Vec<String> =
                    (0..acts.len()).map(|k| format!("act {}", k + 1)).collect();
                if self.pot > 0 {
                    v.push("stop".into());
                }
                v
            }
            Kind::Bag { .. } => {
                let mut v = Vec::new();
                if self.bag.iter().map(|&x| x as u32).sum::<u32>() > 0 {
                    v.push("tragi".into());
                }
                if self.pot > 0 || v.is_empty() {
                    v.push("stop".into());
                }
                v
            }
        }
    }
    fn step(&mut self, m: &str) {
        match m {
            "stop" => {
                self.pts[self.turn] += self.pot;
                self.last = format!("{} a băgat {} puncte", pname(pl(self.turn)), self.pot);
                self.end_turn();
            }
            "arata" => {
                let k = self.pend.take().unwrap();
                match self.spec.kind {
                    Kind::Slots { n, target } => {
                        let rem = n as usize - self.used.count_ones() as usize;
                        // `used` include deja locul ales
                        let hit = chance(&mut self.dice, 100 / (rem as i32 + 1));
                        if hit {
                            self.pts[1 - self.turn] += 1;
                            self.last = format!("{} a atins capcana", pname(pl(self.turn)));
                            self.used = 0;
                            self.turn = 1 - self.turn;
                            let _ = target;
                        } else {
                            self.turn = 1 - self.turn;
                        }
                        self.done += 1;
                    }
                    Kind::Bank { acts, .. } => {
                        let h = self.hazard_at(k);
                        self.ticks += acts[k].0;
                        if chance(&mut self.dice, h) {
                            self.last = format!("{} a depășit limita", pname(pl(self.turn)));
                            self.end_turn();
                        } else {
                            self.pot += acts[k].1;
                        }
                    }
                    Kind::Bag { good, lives, .. } => {
                        let tot: usize = self.bag.iter().map(|&x| x as usize).sum();
                        let mut r = self.dice.below(tot);
                        let mut which = 0;
                        for (i, &c) in self.bag.iter().enumerate() {
                            if r < c as usize {
                                which = i;
                                break;
                            }
                            r -= c as usize;
                        }
                        self.bag[which] -= 1;
                        if which < good.len() {
                            self.pot += which as i32 + 1;
                        } else {
                            self.lost += 1;
                            if self.lost >= lives {
                                self.last = format!("{} a pierdut potul", pname(pl(self.turn)));
                                self.end_turn();
                            }
                        }
                    }
                }
            }
            _ => {
                if let Some(x) = m.strip_prefix("taie ") {
                    let i: u32 = x.parse().unwrap();
                    self.used |= 1 << (i - 1);
                    self.pend = Some(0);
                } else {
                    let k: usize = m[4..].parse::<usize>().unwrap_or(1) - 1;
                    self.pend = Some(k);
                    if m == "tragi" {
                        self.pend = Some(0);
                    }
                }
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.pend.is_some() {
            return None;
        }
        match self.spec.kind {
            Kind::Slots { target, .. } => {
                if self.pts[0] >= target || self.pts[1] >= target || self.done > 400 {
                    Some(pts_outcome(self.pts))
                } else {
                    None
                }
            }
            _ => {
                if self.done >= self.spec.turns {
                    Some(pts_outcome(self.pts))
                } else {
                    None
                }
            }
        }
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let base = 100 * (self.pts[a] - self.pts[1 - a]);
        if let Some(k) = self.pend {
            // riscul public al mutării în așteptare (fără a citi capcana)
            let h = self.hazard_at(k);
            let sign = if self.turn == a { 1 } else { -1 };
            return match self.spec.kind {
                Kind::Slots { .. } => base - sign * h,
                _ => base + sign * ((100 - h) * (self.pot + self.gain(k))),
            };
        }
        if self.turn == a {
            base + 100 * self.pot
        } else {
            base - 100 * self.pot
        }
    }
    fn show(&self) -> String {
        let mut s = format!(
            "{} — scor J1 {} / J2 {}\n",
            self.spec.title, self.pts[0], self.pts[1]
        );
        match self.spec.kind {
            Kind::Slots { n, target } => {
                let free: Vec<String> = (0..n)
                    .filter(|i| self.used >> i & 1 == 0)
                    .map(|i| (i + 1).to_string())
                    .collect();
                s.push_str(&format!(
                    "Primul la {target}. Locuri rămase: {}\n",
                    free.join(" ")
                ));
            }
            Kind::Bank { lo, hi, .. } => {
                s.push_str(&format!(
                    "Tura {}/{}. Pot: {}, ticuri: {} (limita ascunsă e între {} și {})\n",
                    self.done + 1,
                    self.spec.turns,
                    self.pot,
                    self.ticks,
                    lo,
                    hi
                ));
            }
            Kind::Bag { good, lives, .. } => {
                let mut c = String::new();
                for (i, _) in good.iter().enumerate() {
                    c.push_str(&format!("valoare {}: {} | ", i + 1, self.bag[i]));
                }
                s.push_str(&format!(
                    "Tura {}/{}. Pot: {}. În sac — {}rele: {} (pierzi la {} rele, ai {})\n",
                    self.done + 1,
                    self.spec.turns,
                    self.pot,
                    c,
                    self.bag.last().unwrap(),
                    lives,
                    self.lost
                ));
            }
        }
        if !self.last.is_empty() {
            s.push_str(&format!("Ultima: {}\n", self.last));
        }
        s.push_str(&format!(
            "La mutare: {} (taie i | act k | tragi | stop | arata)\n",
            pname(self.side())
        ));
        s
    }
}

impl Capcana {
    fn hazard_at(&self, k: usize) -> i32 {
        match self.spec.kind {
            // locul ales e deja marcat în `used`: riscul era 1/(rămase+1)
            Kind::Slots { .. } => 100 / (self.spec_remaining() + 1),
            Kind::Bank { lo, hi, acts } => {
                // `ticks` nu include încă acțiunea în așteptare
                let c = acts[k].0;
                let d = (hi - lo + 1 - Self::over(self.ticks, lo)).max(1);
                ((Self::over(self.ticks + c, lo) - Self::over(self.ticks, lo)) * 100 / d)
                    .clamp(0, 100)
            }
            Kind::Bag { .. } => self.hazard(0),
        }
    }
    fn spec_remaining(&self) -> i32 {
        match self.spec.kind {
            Kind::Slots { n, .. } => n as i32 - self.used.count_ones() as i32,
            _ => 1,
        }
    }
}

pub static SPECS: &[Spec] = &[
    Spec {
        id: "butoiasul_corsar",
        title: "Butoiașul Corsarului",
        kind: Kind::Slots { n: 10, target: 3 },
        turns: 0,
        names: &[],
    },
    Spec {
        id: "camara_adormita",
        title: "Camera Somnoroasă",
        kind: Kind::Bank {
            lo: 4,
            hi: 10,
            acts: &[(1, 1), (2, 3)],
        },
        turns: 8,
        names: &[],
    },
    Spec {
        id: "piesa_la_timp",
        title: "Piesa din Clepsidră",
        kind: Kind::Bank {
            lo: 5,
            hi: 12,
            acts: &[(1, 1), (2, 2), (3, 4)],
        },
        turns: 8,
        names: &[],
    },
    Spec {
        id: "oua_din_cuib",
        title: "Ouăle din Cuib",
        kind: Kind::Bag {
            good: &[5, 4, 2],
            bad: 5,
            lives: 2,
        },
        turns: 8,
        names: &[],
    },
    Spec {
        id: "pietre_sclipitoare",
        title: "Pietrele Sclipitoare",
        kind: Kind::Bag {
            good: &[4, 4, 3, 2],
            bad: 3,
            lives: 1,
        },
        turns: 8,
        names: &[],
    },
];

pub fn make(id: &str) -> Box<dyn polytabletop_core::GameObj> {
    Box::new(B(Capcana::new(
        SPECS.iter().find(|s| s.id == id).expect("spec"),
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plays() {
        for s in SPECS {
            check_c(&|sd| Capcana::with_seed(s, sd), 800);
        }
    }
}
