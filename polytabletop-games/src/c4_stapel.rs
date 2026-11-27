//! Familia „turn de piese” (variante originale C4, 2 jucători): pui pe rând piese pe un turn comun;
//! fiecare joc are propria funcție de risc (publică), iar căderea se hotărăște la mutarea forțată
//! `verific` (zar re-însămânțat la `clone()`: AI-ul nu o poate prevedea). Cine face turnul să cadă
//! ia înapoi piesele căzute. Câștigă primul care rămâne fără piese. Informație complet publică.
//! Modul `Seesaw` (balanță) e determinist: pui pe talerul `s` sau `d`; dacă diferența depășește limita,
//! ia înapoi piesele talerului mai greu.
use crate::c4_util::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Risk {
    /// risc% = base + slope·înălțime + step·max(0, w − vârf)
    Heap {
        base: i32,
        slope: i32,
        step: i32,
    },
    /// limita ascunsă de încărcare e uniformă în [lo, hi]; riscul e probabilitatea condiționată
    Load {
        lo: i32,
        hi: i32,
    },
    Seesaw {
        limit: i32,
    },
}

pub struct Spec {
    pub id: &'static str,
    pub title: &'static str,
    pub risk: Risk,
    /// Piesele (greutăți 1..) din mâna fiecărui jucător.
    pub pieces: &'static [u8],
    /// Zar: (de pus, de dat adversarului) pentru fiecare față; o singură față = fără zar.
    pub die: &'static [(u8, u8)],
    pub cap: u32,
    pub names: &'static [&'static str],
}

#[derive(Clone)]
pub struct Stapel {
    spec: &'static Spec,
    dice: Dice,
    hand: [Vec<u8>; 2],
    tower: Vec<u8>,
    pan: [Vec<u8>; 2],
    turn: usize,
    place: u8,
    give: u8,
    rolled: bool,
    pend: Option<(usize, u8, i32)>,
    plies: u32,
}

impl Stapel {
    pub fn new(spec: &'static Spec) -> Self {
        Self::with_seed(spec, seed_now())
    }
    pub fn with_seed(spec: &'static Spec, seed: u64) -> Self {
        let mut s = Stapel {
            spec,
            dice: Dice::new(seed),
            hand: [spec.pieces.to_vec(), spec.pieces.to_vec()],
            tower: Vec::new(),
            pan: [Vec::new(), Vec::new()],
            turn: 0,
            place: 1,
            give: 0,
            rolled: spec.die.len() < 2,
            pend: None,
            plies: 0,
        };
        s.begin();
        s
    }
    fn begin(&mut self) {
        if self.spec.die.len() < 2 {
            self.place = self.spec.die[0].0;
            self.give = self.spec.die[0].1;
            self.rolled = true;
        } else {
            self.rolled = false;
        }
    }
    fn name(&self, w: u8) -> String {
        self.spec.names[(w as usize - 1).min(self.spec.names.len() - 1)].to_string()
    }
    fn risk(&self, w: u8) -> i32 {
        match self.spec.risk {
            Risk::Heap { base, slope, step } => {
                let top = *self.tower.last().unwrap_or(&0) as i32;
                (base + slope * self.tower.len() as i32 + step * (w as i32 - top).max(0))
                    .clamp(0, 95)
            }
            Risk::Load { lo, hi } => {
                let load: i32 = self.tower.iter().map(|&x| x as i32).sum();
                let over = |l: i32| (l - lo).max(0);
                let d = (hi - lo + 1 - over(load)).max(1);
                ((over(load + w as i32) - over(load)) * 100 / d).clamp(0, 95)
            }
            Risk::Seesaw { .. } => 0,
        }
    }
    fn fall(&self) -> usize {
        (2 + self.tower.len() / 3).min(self.tower.len())
    }
    fn next_turn(&mut self) {
        self.turn = 1 - self.turn;
        self.begin();
        self.plies += 1;
    }
    fn settle(&mut self) {
        if self.pend.is_some() {
            return;
        }
        let h = &self.hand[self.turn];
        if (self.place == 0 || h.is_empty()) && (self.give == 0 || h.is_empty()) {
            self.next_turn();
        }
    }
    fn distinct(&self) -> Vec<u8> {
        let mut v = self.hand[self.turn].clone();
        v.sort();
        v.dedup();
        v
    }
    fn seesaw(&self) -> bool {
        matches!(self.spec.risk, Risk::Seesaw { .. })
    }
    fn weight(&self, i: usize) -> i32 {
        self.pan[i].iter().map(|&x| x as i32).sum()
    }
}

impl Mini for Stapel {
    fn depth(&self) -> u32 {
        if self.seesaw() {
            3
        } else {
            0
        }
    }
    fn side(&self) -> Player {
        pl(self.turn)
    }
    fn moves(&self) -> Vec<String> {
        if self.pend.is_some() {
            return vec!["verific".into()];
        }
        if !self.rolled {
            return vec!["zar".into()];
        }
        let mut v = Vec::new();
        for w in self.distinct() {
            if self.place > 0 {
                if self.seesaw() {
                    v.push(format!("pun {w} s"));
                    v.push(format!("pun {w} d"));
                } else {
                    v.push(format!("pun {w}"));
                }
            } else if self.give > 0 {
                v.push(format!("ofer {w}"));
            }
        }
        v
    }
    fn step(&mut self, m: &str) {
        let me = self.turn;
        if m == "zar" {
            let k = self.dice.below(self.spec.die.len());
            self.place = self.spec.die[k].0;
            self.give = self.spec.die[k].1;
            self.rolled = true;
            self.settle();
            return;
        }
        if m == "verific" {
            let (who, _w, r) = self.pend.take().unwrap();
            if chance(&mut self.dice, r) {
                let k = self.fall();
                for _ in 0..k {
                    let x = self.tower.pop().unwrap();
                    self.hand[who].push(x);
                }
                // piesa pusă tocmai (w) a căzut și ea
                self.place = 0;
                self.give = 0;
            }
            self.settle();
            return;
        }
        let mut it = m.split(' ');
        let verb = it.next().unwrap();
        let w: u8 = it.next().unwrap().parse().unwrap();
        let pos = self.hand[me].iter().position(|&x| x == w).unwrap();
        self.hand[me].remove(pos);
        if verb == "ofer" {
            self.hand[1 - me].push(w);
            self.give -= 1;
            self.settle();
            return;
        }
        self.place -= 1;
        if let Risk::Seesaw { limit } = self.spec.risk {
            let side = if it.next() == Some("s") { 0 } else { 1 };
            self.pan[side].push(w);
            let (a, b) = (self.weight(0), self.weight(1));
            if (a - b).abs() > limit {
                let heavy = if a > b { 0 } else { 1 };
                let back = std::mem::take(&mut self.pan[heavy]);
                self.hand[me].extend(back);
                self.place = 0;
                self.give = 0;
            }
            self.settle();
            return;
        }
        let r = self.risk(w);
        self.tower.push(w);
        self.pend = Some((me, w, r));
    }
    fn result(&self) -> Option<Outcome> {
        if self.pend.is_some() {
            return None;
        }
        if self.hand[0].is_empty() {
            return Some(Outcome::Win(Player::One));
        }
        if self.hand[1].is_empty() {
            return Some(Outcome::Win(Player::Two));
        }
        if self.plies >= self.spec.cap {
            return Some(match self.hand[0].len().cmp(&self.hand[1].len()) {
                std::cmp::Ordering::Less => Outcome::Win(Player::One),
                std::cmp::Ordering::Greater => Outcome::Win(Player::Two),
                std::cmp::Ordering::Equal => Outcome::Draw,
            });
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let mut v = 100 * (self.hand[1 - a].len() as i32 - self.hand[a].len() as i32);
        if self.seesaw() {
            v += 3 * (self.weight(0) - self.weight(1)).abs() * if self.turn == a { -1 } else { 1 };
        }
        if let Some((who, _w, r)) = self.pend {
            let k = self.fall() as i32 + 1;
            v += if who == a { -r * k } else { r * k };
        }
        v
    }
    fn show(&self) -> String {
        let mut s = format!(
            "{} — mutarea {}/{}\n",
            self.spec.title,
            self.plies + 1,
            self.spec.cap
        );
        let names = |h: &Vec<u8>| {
            let mut h = h.clone();
            h.sort();
            h.iter()
                .map(|&w| format!("{}={}", w, self.name(w)))
                .collect::<Vec<_>>()
                .join(", ")
        };
        if self.seesaw() {
            s.push_str(&format!(
                "Taler stâng ({}): {}\nTaler drept ({}): {}\n",
                self.weight(0),
                names(&self.pan[0]),
                self.weight(1),
                names(&self.pan[1])
            ));
        } else {
            let top = self
                .tower
                .last()
                .map(|&w| self.name(w))
                .unwrap_or("-".into());
            s.push_str(&format!("Turn: {} piese, vârf {}\n", self.tower.len(), top));
        }
        for p in 0..2 {
            s.push_str(&format!(
                "J{} ({} piese): {}\n",
                p + 1,
                self.hand[p].len(),
                names(&self.hand[p])
            ));
        }
        s.push_str(&format!(
            "La mutare: {} — de pus {}, de dat {}\nMutări: zar | pun w | ofer w | verific\n",
            pname(self.side()),
            self.place,
            self.give
        ));
        s
    }
}

pub static SPECS: &[Spec] = &[
    Spec {
        id: "turnul_fermei",
        title: "Turnul Animalelor",
        risk: Risk::Heap {
            base: 4,
            slope: 5,
            step: 9,
        },
        pieces: &[1, 1, 2, 2, 3, 3, 4],
        die: &[(1, 0), (1, 0), (2, 0), (1, 1), (0, 1), (1, 0)],
        cap: 160,
        names: &["pui", "oaie", "vacă", "elefant"],
    },
    Spec {
        id: "turnul_rinocerului",
        title: "Zgârie-norul Cartonat",
        risk: Risk::Heap {
            base: 2,
            slope: 7,
            step: 6,
        },
        pieces: &[1, 1, 2, 2, 3, 3],
        die: &[(1, 0)],
        cap: 140,
        names: &["perete jos", "perete mediu", "perete înalt"],
    },
    Spec {
        id: "balansul_cerdacului",
        title: "Balanța Cerdacului",
        risk: Risk::Seesaw { limit: 5 },
        pieces: &[1, 1, 2, 2, 3, 3, 4],
        die: &[(1, 0)],
        cap: 120,
        names: &["bilă", "con", "cub", "bară"],
    },
    Spec {
        id: "catarul_incarcat",
        title: "Măgarul Încărcat",
        risk: Risk::Load { lo: 10, hi: 22 },
        pieces: &[1, 1, 2, 2, 3, 3],
        die: &[(1, 0)],
        cap: 140,
        names: &["lopată", "găleată", "sac"],
    },
    Spec {
        id: "tort_pe_tort",
        title: "Tortul cu Straturi",
        risk: Risk::Heap {
            base: 0,
            slope: 6,
            step: 12,
        },
        pieces: &[1, 2, 2, 3, 3, 4],
        die: &[(1, 0), (1, 0), (2, 0), (1, 0), (1, 1), (1, 0)],
        cap: 150,
        names: &["cireașă", "blat mic", "blat mijlociu", "blat mare"],
    },
    Spec {
        id: "piramida_bolovani",
        title: "Piramida de Bolovani",
        risk: Risk::Load { lo: 12, hi: 26 },
        pieces: &[2, 2, 3, 3, 4, 4],
        die: &[(1, 0), (2, 0), (1, 0), (1, 1), (1, 0), (1, 0)],
        cap: 150,
        names: &["", "pietricică", "bolovan", "stâncă"],
    },
];

pub fn make(id: &str) -> Box<dyn polytabletop_core::GameObj> {
    Box::new(B(Stapel::new(
        SPECS.iter().find(|s| s.id == id).expect("spec"),
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plays() {
        for s in SPECS {
            check_c(&|sd| Stapel::with_seed(s, sd), 600);
        }
    }
}
