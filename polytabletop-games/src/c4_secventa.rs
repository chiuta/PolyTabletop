//! „Memoria Luminilor” — variantă originală C4 de memorie secvențială (2 jucători, informație publică).
//! Cine „adaugă” pune o culoare la coada secvenței (`adaug c`, culorile r v a g); cealaltă persoană o repetă
//! din memorie, culoare cu culoare (`rep c`). O greșeală dă o penalizare celui care repetă (3 = pierde),
//! secvența se reia și începe cel care a greșit. Secvența completă de 10 repetată corect încheie runda fără penalizare.
//! Randarea arată secvența doar în faza de adăugare; AI-ul își amintește cel mult 7 semnale (peste, alege la întâmplare).
use crate::c4_util::*;

const COL: [&str; 4] = ["r", "v", "a", "g"];
const MEM: usize = 7;
const FULL: usize = 10;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ph {
    Add,
    Rep(usize),
}

#[derive(Clone)]
pub struct Secventa {
    seq: Vec<u8>,
    ph: Ph,
    turn: usize,
    strikes: [i32; 2],
    plies: u32,
    big: bool,
}

impl Secventa {
    pub fn new() -> Self {
        Secventa {
            seq: Vec::new(),
            ph: Ph::Add,
            turn: 0,
            strikes: [0; 2],
            plies: 0,
            big: false,
        }
    }
}

impl Default for Secventa {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Secventa {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        pl(self.turn)
    }
    fn moves(&self) -> Vec<String> {
        let verb = if self.ph == Ph::Add { "adaug" } else { "rep" };
        COL.iter().map(|c| format!("{verb} {c}")).collect()
    }
    fn step(&mut self, m: &str) {
        let (verb, c) = m.split_once(' ').unwrap();
        let c = COL.iter().position(|&x| x == c).unwrap() as u8;
        self.plies += 1;
        self.big = verb == "rep" && self.seq.len() > MEM;
        if verb == "adaug" {
            self.seq.push(c);
            self.ph = Ph::Rep(0);
            self.turn = 1 - self.turn;
            return;
        }
        let Ph::Rep(i) = self.ph else { return };
        if self.seq[i] != c {
            self.strikes[self.turn] += 1;
            self.seq.clear();
            self.ph = Ph::Add;
            return;
        }
        if i + 1 == self.seq.len() {
            if self.seq.len() >= FULL {
                self.seq.clear();
            }
            self.ph = Ph::Add;
        } else {
            self.ph = Ph::Rep(i + 1);
        }
    }
    fn result(&self) -> Option<Outcome> {
        for w in 0..2 {
            if self.strikes[w] >= 3 {
                return Some(Outcome::Win(pl(1 - w)));
            }
        }
        if self.plies > 600 {
            return Some(by_score(-self.strikes[0], -self.strikes[1]));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let mut v = 100 * (self.strikes[1 - a] - self.strikes[a]);
        // memoria AI-ului e limitată: dincolo de MEM, corectitudinea nu se vede
        if self.big {
            v = 0;
        }
        v
    }
    fn show(&self) -> String {
        let mut s = format!(
            "Memoria Luminilor — penalizări J1 {}/3, J2 {}/3\n",
            self.strikes[0], self.strikes[1]
        );
        match self.ph {
            Ph::Add => {
                let q: Vec<&str> = self.seq.iter().map(|&c| COL[c as usize]).collect();
                s.push_str(&format!("Secvența ({}): {}\n", self.seq.len(), q.join(" ")));
            }
            Ph::Rep(i) => s.push_str(&format!(
                "Repetă din memorie: {}/{} culori (secvența e ascunsă)\n",
                i,
                self.seq.len()
            )),
        }
        s.push_str(&format!(
            "La mutare: {} (adaug c | rep c; c = r v a g)\n",
            pname(self.side())
        ));
        s
    }
}

pub fn make() -> Box<dyn polytabletop_core::GameObj> {
    Box::new(B(Secventa::new()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plays() {
        check(|_| Secventa::new(), 1500);
    }
}
