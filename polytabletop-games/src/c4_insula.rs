//! „Pirații din Golf” — variantă originală C4 de economie pe o insulă (2 jucători, informație publică).
//! Inel de 6 câmpuri numerotate 1..6, fiecare cu o resursă (lemn, aur, lână). 6 locuri de ancorare între câmpuri vecine.
//! Mutarea forțată `zar` produce resurse (fiecare ancoră produce câte 1 din câmpurile ei vecine cu acel număr).
//! Apoi: `ancor s` (construiești o ancoră liberă, 1 lemn + 1 aur + 1 lână), `schimb a b` (2 din a pentru 1 din b), `pas`.
//! Primul cu 4 ancore câștigă. Zarul e re-însămânțat la `clone()`: AI-ul nu-l poate prevedea.
use crate::c4_util::*;

const RES: [&str; 3] = ["lemn", "aur", "lână"];
/// Resursa fiecărui câmp (numărul câmpului = indexul + 1).
const FIELD: [usize; 6] = [0, 1, 2, 1, 2, 0];

#[derive(Clone)]
pub struct Insula {
    dice: Dice,
    owner: [Option<usize>; 6],
    res: [[u8; 3]; 2],
    turn: usize,
    d: Option<u8>,
    plies: u32,
}

impl Insula {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut owner = [None; 6];
        owner[0] = Some(0);
        owner[3] = Some(1);
        Insula {
            dice: Dice::new(seed),
            owner,
            res: [[1; 3]; 2],
            turn: 0,
            d: None,
            plies: 0,
        }
    }
    fn lairs(&self, w: usize) -> i32 {
        self.owner.iter().filter(|&&o| o == Some(w)).count() as i32
    }
    fn stock(&self, w: usize) -> i32 {
        self.res[w].iter().map(|&x| x as i32).sum()
    }
}

impl Default for Insula {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Insula {
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
        let r = self.res[self.turn];
        let mut v = vec!["pas".to_string()];
        if r.iter().all(|&x| x >= 1) {
            for s in 0..6 {
                if self.owner[s].is_none() {
                    v.push(format!("ancor {}", s + 1));
                }
            }
        }
        for (a, &have) in r.iter().enumerate() {
            for b in 0..3 {
                if a != b && have >= 2 {
                    v.push(format!("schimb {} {}", a + 1, b + 1));
                }
            }
        }
        v
    }
    fn step(&mut self, m: &str) {
        let me = self.turn;
        if m == "zar" {
            let d = self.dice.d6();
            self.d = Some(d);
            // câmpul d: ancorele de pe locurile s (câmpurile s și s+1) produc
            for s in 0..6 {
                if let Some(w) = self.owner[s] {
                    for f in [s, (s + 1) % 6] {
                        if f + 1 == d as usize {
                            self.res[w][FIELD[f]] = (self.res[w][FIELD[f]] + 1).min(9);
                        }
                    }
                }
            }
            return;
        }
        let p: Vec<&str> = m.split(' ').collect();
        match p[0] {
            "ancor" => {
                let s = p[1].parse::<usize>().unwrap() - 1;
                for x in self.res[me].iter_mut() {
                    *x -= 1;
                }
                self.owner[s] = Some(me);
            }
            "schimb" => {
                let a = p[1].parse::<usize>().unwrap() - 1;
                let b = p[2].parse::<usize>().unwrap() - 1;
                self.res[me][a] -= 2;
                self.res[me][b] = (self.res[me][b] + 1).min(9);
                return; // poți continua cu alte acțiuni
            }
            _ => {}
        }
        self.d = None;
        self.turn = 1 - me;
        self.plies += 1;
    }
    fn result(&self) -> Option<Outcome> {
        for w in 0..2 {
            if self.lairs(w) >= 4 {
                return Some(Outcome::Win(pl(w)));
            }
        }
        if self.plies > 300 {
            return Some(pts_outcome([
                self.lairs(0) * 10 + self.stock(0),
                self.lairs(1) * 10 + self.stock(1),
            ]));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let o = 1 - a;
        let mut v = 1000 * (self.lairs(a) - self.lairs(o)) + 30 * (self.stock(a) - self.stock(o));
        // varietatea de resurse contează (ancora cere câte una din fiecare)
        let var = |w: usize| self.res[w].iter().filter(|&&x| x > 0).count() as i32;
        v += 50 * (var(a) - var(o));
        v
    }
    fn show(&self) -> String {
        let mut s = String::from("Pirații din Golf\n");
        let fields: Vec<String> = (0..6)
            .map(|i| format!("{}={}", i + 1, RES[FIELD[i]]))
            .collect();
        s.push_str(&format!("Câmpuri: {}\n", fields.join(", ")));
        let sites: Vec<String> = (0..6)
            .map(|i| {
                format!(
                    "{}:{}",
                    i + 1,
                    self.owner[i]
                        .map(|w| format!("J{}", w + 1))
                        .unwrap_or("-".into())
                )
            })
            .collect();
        s.push_str(&format!(
            "Ancore (loc s între câmpurile s și s+1): {}\n",
            sites.join(" ")
        ));
        for w in 0..2 {
            s.push_str(&format!(
                "J{}: {} ancore; {} lemn, {} aur, {} lână\n",
                w + 1,
                self.lairs(w),
                self.res[w][0],
                self.res[w][1],
                self.res[w][2]
            ));
        }
        if let Some(d) = self.d {
            s.push_str(&format!("Zar: {d}\n"));
        }
        s.push_str(&format!(
            "La mutare: {} (zar | ancor s | schimb a b | pas)\n",
            pname(self.side())
        ));
        s
    }
}

pub fn make() -> Box<dyn polytabletop_core::GameObj> {
    Box::new(B(Insula::new()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plays() {
        check(Insula::with_seed, 1200);
    }
}
