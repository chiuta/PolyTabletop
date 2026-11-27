//! Jocuri pentru copii (lista C3, variante originale, 2 jucători): Mâna Sigură, Hipopotamii Lacomi,
//! Pioni Prinși, Cursa Cașcavalului, Bilele în Tub. Șansa apare doar în mutarea forțată
//! (aruncare/încercare), deci AI-ul alege cu informația publică, fără să vadă zarul.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

fn pl(i: usize) -> Player {
    if i == 0 {
        Player::One
    } else {
        Player::Two
    }
}

// ───────────────────────── Mâna Sigură (operație cu mână fermă) ─────────────────────────
// 10 piese (valoare, dificultate 1..5); `scot i` încearcă piesa; rezultatul vine prin `mana`
// (forțat): reușita are șansa 95-14·dificultate %. Eșec: bâzâit, celălalt primește 1.
const PIESE: [(i32, i32); 10] = [
    (1, 1),
    (1, 1),
    (2, 2),
    (2, 2),
    (3, 3),
    (3, 3),
    (4, 3),
    (5, 4),
    (6, 5),
    (8, 5),
];

#[derive(Clone)]
pub struct Mana {
    left: Vec<bool>,
    score: [i32; 2],
    pend: Option<usize>,
    side: usize,
    rng: Rng,
}

pub fn mana(seed: u64) -> Mana {
    Mana {
        left: vec![true; 10],
        score: [0; 2],
        pend: None,
        side: 0,
        rng: Rng::new(seed),
    }
}

pub fn mana_new() -> Mana {
    mana(seed_now())
}

impl Mini for Mana {
    fn side(&self) -> Player {
        pl(self.side)
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        if self.pend.is_some() {
            return vec!["mana".into()];
        }
        (0..10)
            .filter(|&i| self.left[i])
            .map(|i| format!("scot {}", i + 1))
            .collect()
    }
    fn step(&mut self, m: &str) {
        if let Some(i) = self.pend.take() {
            let p = 95 - 14 * PIESE[i].1;
            if (self.rng.below(100) as i32) < p {
                self.score[self.side] += PIESE[i].0;
                self.left[i] = false;
            } else {
                self.score[1 - self.side] += 1;
            }
            self.side = 1 - self.side;
            return;
        }
        self.pend = Some(m[5..].parse::<usize>().unwrap() - 1);
    }
    fn result(&self) -> Option<Outcome> {
        if self.left.iter().any(|&l| l) {
            None
        } else {
            Some(by_score(self.score[0], self.score[1]))
        }
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let mut e = 100 * (self.score[a] - self.score[1 - a]);
        if let Some(i) = self.pend {
            let pr = 95 - 14 * PIESE[i].1;
            let ev = pr * PIESE[i].0 - (100 - pr);
            e += if self.side == a { ev } else { -ev };
        }
        e
    }
    fn show(&self) -> String {
        let mut s = format!(
            "Mâna Sigură — J1 {} | J2 {}\n",
            self.score[0], self.score[1]
        );
        for i in 0..10 {
            if self.left[i] {
                s.push_str(&format!(
                    "Piesa {}: valoare {}, dificultate {} ({}% reușită)\n",
                    i + 1,
                    PIESE[i].0,
                    PIESE[i].1,
                    95 - 14 * PIESE[i].1
                ));
            }
        }
        s.push_str(&format!("La mutare: {} — scot i\n", pname(self.side())));
        s
    }
}

// ───────────────────────── Hipopotamii Lacomi ─────────────────────────
// 8 runde, 2 bile pe rundă. La începutul rundei jucătorul care deschide (alternează) apoi
// celălalt aleg `tap k` (k=0..3, cel mult cât energie au, 14 pe partidă). Apoi `lansez` (forțat):
// fiecare bilă ajunge la J1 cu probabilitatea e1/(e1+e2) (50% dacă 0/0).
#[derive(Clone)]
pub struct Hipo {
    marbles: [i32; 2],
    energy: [i32; 2],
    tap: [Option<i32>; 2],
    round: u32,
    rng: Rng,
}

pub fn hipo(seed: u64) -> Hipo {
    Hipo {
        marbles: [0; 2],
        energy: [14; 2],
        tap: [None; 2],
        round: 0,
        rng: Rng::new(seed),
    }
}

pub fn hipo_new() -> Hipo {
    hipo(seed_now())
}

impl Hipo {
    fn mover(&self) -> usize {
        let first = (self.round % 2) as usize;
        if self.tap[first].is_none() {
            first
        } else {
            1 - first
        }
    }
}

impl Mini for Hipo {
    fn side(&self) -> Player {
        pl(self.mover())
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        if self.tap[0].is_some() && self.tap[1].is_some() {
            return vec!["lansez".into()];
        }
        let e = self.energy[self.mover()];
        (0..=3)
            .filter(|&k| k <= e)
            .map(|k| format!("tap {k}"))
            .collect()
    }
    fn step(&mut self, m: &str) {
        if m == "lansez" {
            let (a, b) = (self.tap[0].unwrap(), self.tap[1].unwrap());
            for _ in 0..2 {
                let win = if a + b == 0 {
                    self.rng.below(2) as i32
                } else {
                    (self.rng.below((a + b) as usize) as i32 >= a) as i32
                };
                self.marbles[win as usize] += 1;
            }
            self.energy[0] -= a;
            self.energy[1] -= b;
            self.tap = [None; 2];
            self.round += 1;
            return;
        }
        let k = m[4..].parse::<i32>().unwrap();
        let me = self.mover();
        self.tap[me] = Some(k);
    }
    fn result(&self) -> Option<Outcome> {
        if self.round >= 8 {
            Some(by_score(self.marbles[0], self.marbles[1]))
        } else {
            None
        }
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let mut e = 100 * (self.marbles[a] - self.marbles[1 - a])
            + 12 * (self.energy[a] - self.energy[1 - a]);
        if let Some(k) = self.tap[a] {
            e += 40 * k / (k + 2);
            e -= 12 * k;
        }
        e
    }
    fn show(&self) -> String {
        format!(
            "Hipopotamii Lacomi — runda {}/8 | bile J1 {} J2 {} | energie J1 {} J2 {}\nAle rundei: {}\nLa mutare: {} — tap k (0..3)\n",
            (self.round + 1).min(8),
            self.marbles[0],
            self.marbles[1],
            self.energy[0],
            self.energy[1],
            match (self.tap[0], self.tap[1]) {
                (Some(a), Some(b)) => format!("J1 {a}, J2 {b}"),
                (Some(a), None) => format!("J1 {a}"),
                (None, Some(b)) => format!("J2 {b}"),
                _ => "-".into(),
            },
            pname(self.side())
        )
    }
}

// ───────────────────────── Pioni Prinși (ludo cu 3 pioni) ─────────────────────────
// Inel de 24 de câmpuri; fiecare intră cu 6 la câmpul de start (0 și 12), parcurge 24 de pași
// și trebuie să ajungă exact la 24 (acasă). Un pion care aterizează peste adversar îl trimite în
// bază. 6 = încă o aruncare. Câștigă primul cu 3 pioni acasă.
#[derive(Clone)]
pub struct Pioni {
    pg: [[i32; 3]; 2],
    die: Option<i32>,
    side: usize,
    rng: Rng,
}

pub fn pioni(seed: u64) -> Pioni {
    Pioni {
        pg: [[-1; 3]; 2],
        die: None,
        side: 0,
        rng: Rng::new(seed),
    }
}

pub fn pioni_new() -> Pioni {
    pioni(seed_now())
}

impl Pioni {
    fn abs(p: usize, g: i32) -> i32 {
        (g + 12 * p as i32) % 24
    }
    fn dest(&self, p: usize, k: usize, d: i32) -> Option<i32> {
        let g = self.pg[p][k];
        let n = if g < 0 {
            if d == 6 {
                0
            } else {
                return None;
            }
        } else {
            g + d
        };
        if n > 24 || g == 24 {
            return None;
        }
        if n < 24 && (0..3).any(|j| j != k && self.pg[p][j] == n) {
            return None;
        }
        Some(n)
    }
}

impl Mini for Pioni {
    fn side(&self) -> Player {
        pl(self.side)
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        let Some(d) = self.die else {
            return vec!["zar".into()];
        };
        let v: Vec<String> = (0..3)
            .filter(|&k| self.dest(self.side, k, d).is_some())
            .map(|k| format!("mut {}", k + 1))
            .collect();
        if v.is_empty() {
            vec!["pas".into()]
        } else {
            v
        }
    }
    fn step(&mut self, m: &str) {
        let Some(d) = self.die else {
            self.die = Some(1 + self.rng.below(6) as i32);
            return;
        };
        if let Some(k) = m.strip_prefix("mut ") {
            let k = k.parse::<usize>().unwrap() - 1;
            let n = self.dest(self.side, k, d).unwrap();
            self.pg[self.side][k] = n;
            if n < 24 {
                let a = Self::abs(self.side, n);
                let o = 1 - self.side;
                for j in 0..3 {
                    let g = self.pg[o][j];
                    if (0..24).contains(&g) && Self::abs(o, g) == a {
                        self.pg[o][j] = -1;
                    }
                }
            }
        }
        self.die = None;
        if d != 6 {
            self.side = 1 - self.side;
        }
    }
    fn result(&self) -> Option<Outcome> {
        for p in 0..2 {
            if self.pg[p].iter().all(|&g| g == 24) {
                return Some(Outcome::Win(pl(p)));
            }
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let s = |q: usize| -> i32 {
            self.pg[q]
                .iter()
                .map(|&g| {
                    if g < 0 {
                        0
                    } else {
                        g + 1 + if g == 24 { 20 } else { 0 }
                    }
                })
                .sum()
        };
        s(a) - s(1 - a)
    }
    fn show(&self) -> String {
        let mut s = String::from("Pioni Prinși\n");
        for p in 0..2 {
            let v: Vec<String> = self.pg[p]
                .iter()
                .map(|&g| match g {
                    -1 => "bază".to_string(),
                    24 => "acasă".to_string(),
                    g => format!("{g}"),
                })
                .collect();
            s.push_str(&format!("J{}: {}\n", p + 1, v.join(" | ")));
        }
        s.push_str(&format!(
            "Zar: {} — La mutare: {} (mut k | pas)\n",
            self.die.map(|d| d.to_string()).unwrap_or("-".into()),
            pname(self.side())
        ));
        s
    }
}

// ───────────────────────── Cursa Cașcavalului (cursă cu capcană) ─────────────────────────
// Două șoricei pe jucător pe o pistă 0..15 (15 = cașcaval). Aruncarea (`zar`, forțată) apoi
// `mut m`. Căsuțele 3, 6, 9, 12 adaugă o piesă la capcană (4 piese = gata). Cu capcana gata,
// în loc de mutare poți `declansez`: șoriceii adversarului de pe căsuțele >= 10 se întorc la
// start și capcana se golește. Câștigă cine aduce ambii șoricei la cașcaval.
#[derive(Clone)]
pub struct Soareci {
    pos: [[i32; 2]; 2],
    parts: i32,
    die: Option<i32>,
    side: usize,
    rng: Rng,
}

pub fn soareci(seed: u64) -> Soareci {
    Soareci {
        pos: [[0; 2]; 2],
        parts: 0,
        die: None,
        side: 0,
        rng: Rng::new(seed),
    }
}

pub fn soareci_new() -> Soareci {
    soareci(seed_now())
}

impl Mini for Soareci {
    fn side(&self) -> Player {
        pl(self.side)
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        let Some(_d) = self.die else {
            return vec!["zar".into()];
        };
        let mut v: Vec<String> = (0..2)
            .filter(|&k| self.pos[self.side][k] < 15)
            .map(|k| format!("mut {}", k + 1))
            .collect();
        if self.parts >= 4
            && self.pos[1 - self.side]
                .iter()
                .any(|&p| (10..15).contains(&p))
        {
            v.push("declansez".into());
        }
        if v.is_empty() {
            v.push("pas".into());
        }
        v
    }
    fn step(&mut self, m: &str) {
        let Some(d) = self.die else {
            self.die = Some(1 + self.rng.below(6) as i32);
            return;
        };
        if let Some(k) = m.strip_prefix("mut ") {
            let k = k.parse::<usize>().unwrap() - 1;
            let n = (self.pos[self.side][k] + d).min(15);
            self.pos[self.side][k] = n;
            if [3, 6, 9, 12].contains(&n) && self.parts < 4 {
                self.parts += 1;
            }
        } else if m == "declansez" {
            for k in 0..2 {
                if (10..15).contains(&self.pos[1 - self.side][k]) {
                    self.pos[1 - self.side][k] = 0;
                }
            }
            self.parts = 0;
        }
        self.die = None;
        self.side = 1 - self.side;
    }
    fn result(&self) -> Option<Outcome> {
        for p in 0..2 {
            if self.pos[p] == [15, 15] {
                return Some(Outcome::Win(pl(p)));
            }
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let s = |q: usize| self.pos[q][0] + self.pos[q][1];
        s(a) - s(1 - a)
    }
    fn show(&self) -> String {
        format!(
            "Cursa Cașcavalului — capcană {}/4\nJ1: șoriceii la {} și {}\nJ2: șoriceii la {} și {}\nZar: {} — La mutare: {} (mut k | declansez | pas)\n",
            self.parts,
            self.pos[0][0],
            self.pos[0][1],
            self.pos[1][0],
            self.pos[1][1],
            self.die.map(|d| d.to_string()).unwrap_or("-".into()),
            pname(self.side())
        )
    }
}

// ───────────────────────── Bilele în Tub ─────────────────────────
// 12 bețe, 14 bile; fiecare bilă stă pe 3 bețe (după semințe). `trag s` scoate bățul s; bilele
// rămase fără sprijin cad și se pun pe seama celui care a tras. Când nu mai e niciun băț,
// câștigă cine are mai puține bile căzute pe seama sa. Informație perfectă.
#[derive(Clone)]
pub struct Tub {
    sup: Vec<[u8; 3]>,
    fallen: Vec<bool>,
    sticks: [bool; 12],
    got: [i32; 2],
    side: usize,
}

pub fn tub(seed: u64) -> Tub {
    let mut rng = Rng::new(seed);
    let mut sup = Vec::new();
    for _ in 0..14 {
        let mut ids: Vec<u8> = (0..12).collect();
        shuf(&mut ids, &mut rng);
        sup.push([ids[0], ids[1], ids[2]]);
    }
    Tub {
        fallen: vec![false; 14],
        sup,
        sticks: [true; 12],
        got: [0; 2],
        side: 0,
    }
}

pub fn tub_new() -> Tub {
    tub(seed_now())
}

impl Mini for Tub {
    fn side(&self) -> Player {
        pl(self.side)
    }
    fn depth(&self) -> u32 {
        4
    }
    fn moves(&self) -> Vec<String> {
        (0..12)
            .filter(|&i| self.sticks[i])
            .map(|i| format!("trag {}", i + 1))
            .collect()
    }
    fn step(&mut self, m: &str) {
        let s = m[5..].parse::<usize>().unwrap() - 1;
        self.sticks[s] = false;
        for b in 0..self.sup.len() {
            if !self.fallen[b] && self.sup[b].iter().all(|&x| !self.sticks[x as usize]) {
                self.fallen[b] = true;
                self.got[self.side] += 1;
            }
        }
        self.side = 1 - self.side;
    }
    fn result(&self) -> Option<Outcome> {
        if self.sticks.iter().any(|&s| s) {
            None
        } else {
            Some(by_score(self.got[1], self.got[0]))
        }
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        (self.got[1 - a] - self.got[a]) * 10
    }
    fn show(&self) -> String {
        let st: Vec<String> = (0..12)
            .filter(|&i| self.sticks[i])
            .map(|i| (i + 1).to_string())
            .collect();
        let mut s = format!(
            "Bilele în Tub — bețe: {} | căzute pe seama J1 {} J2 {}\n",
            st.join(" "),
            self.got[0],
            self.got[1]
        );
        for (b, sp) in self.sup.iter().enumerate() {
            if !self.fallen[b] {
                s.push_str(&format!(
                    "bila {}: pe bețele {} {} {}\n",
                    b + 1,
                    sp[0] + 1,
                    sp[1] + 1,
                    sp[2] + 1
                ));
            }
        }
        s.push_str(&format!("La mutare: {} — trag s\n", pname(self.side())));
        s
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        check(mana, 400);
        check(hipo, 400);
        check(pioni, 4000);
        check(soareci, 4000);
        check(tub, 100);
    }
}
