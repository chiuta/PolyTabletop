#![allow(
    clippy::needless_range_loop,
    clippy::manual_is_multiple_of,
    clippy::collapsible_if
)]
//! Motor „axa timpului” (lista C1, 2 jucători, variantă originală de joc de petrecere cu cronologie).
//!
//! Fiecare jucător are o cronologie de invenții în ordinea anilor (începe cu una dată). La rândul tău,
//! mutarea forțată `trag` arată numele unei invenții (anul rămâne ascuns); alegi unde o așezi:
//! `pun k` (k = poziția 0..lungime între cărțile tale). Mutarea forțată `verific` arată anul: dacă locul e
//! corect, cartea rămâne în cronologie, altfel se aruncă. Primul care ajunge la `goal` cărți câștigă;
//! după `cap` ture, cine are cronologia mai lungă. Cronologia fiecăruia e publică; singura informație
//! ascunsă e anul cărții trase (a nimănui, deci fără informație privată). AI-ul își folosește propria
//! „memorie” aproximativă a anilor (cu o eroare deterministă), nu anul exact din stare.
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

pub struct TSpec {
    pub title: &'static str,
    pub goal: usize,
    pub cap: u32,
    /// 0 = toate invențiile; 1 = doar după 1800
    pub era: u8,
}

const ITEMS: [(&str, i32); 40] = [
    ("tiparul cu litere mobile", 1450),
    ("telescopul", 1608),
    ("pendulul cu ceas", 1656),
    ("paratrăsnetul", 1752),
    ("balonul cu aer cald", 1783),
    ("bateria electrică", 1800),
    ("locomotiva cu abur", 1804),
    ("fotografia", 1826),
    ("telegraful electric", 1837),
    ("telefonul", 1876),
    ("fonograful", 1877),
    ("becul cu filament", 1879),
    ("automobilul cu benzină", 1886),
    ("cinematograful", 1895),
    ("radioul", 1895),
    ("avionul cu motor", 1903),
    ("televiziunea electronică", 1927),
    ("penicilina", 1928),
    ("calculatorul electronic ENIAC", 1945),
    ("tranzistorul", 1947),
    ("circuitul integrat", 1958),
    ("laserul", 1960),
    ("zborul în spațiu al lui Gagarin", 1961),
    ("aselenizarea", 1969),
    ("rețeaua ARPANET", 1969),
    ("microprocesorul", 1971),
    ("telefonul mobil", 1973),
    ("calculatorul personal", 1975),
    ("discul compact", 1982),
    ("rețeaua mondială web", 1989),
    ("telescopul Hubble", 1990),
    ("clonarea oii Dolly", 1996),
    ("motorul de căutare Google", 1998),
    ("enciclopedia Wikipedia", 2001),
    ("secvențierea genomului uman", 2003),
    ("smartphone-ul modern", 2007),
    ("tableta modernă", 2010),
    ("detectarea undelor gravitaționale", 2015),
    ("prima imagine a unei găuri negre", 2019),
    ("imprimarea 3D de consum", 2009),
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ph {
    Draw,
    Place,
    Verify,
}

#[derive(Clone)]
pub struct Cronol {
    sp: &'static TSpec,
    line: [Vec<usize>; 2],
    deck: Vec<usize>,
    cur: usize,
    slot: usize,
    ph: Ph,
    side: Player,
    turns: u32,
    rng: Rng,
}

fn memory(i: usize) -> i32 {
    // „amintirea” AI-ului: anul real + o eroare deterministă de până la ±12 ani
    ITEMS[i].1 + ((i * 37 + 11) % 25) as i32 - 12
}

impl Cronol {
    pub fn new(sp: &'static TSpec) -> Self {
        Self::with_seed(sp, seed_now())
    }
    pub fn with_seed(sp: &'static TSpec, seed: u64) -> Self {
        let mut rng = Rng::new(seed ^ 0xa54f_f53a);
        let mut deck: Vec<usize> = (0..ITEMS.len())
            .filter(|&i| sp.era == 0 || ITEMS[i].1 >= 1800)
            .collect();
        shuf(&mut deck, &mut rng);
        let a = deck.pop().unwrap();
        let b = deck.pop().unwrap();
        Cronol {
            sp,
            line: [vec![a], vec![b]],
            deck,
            cur: 0,
            slot: 0,
            ph: Ph::Draw,
            side: Player::One,
            turns: 0,
            rng,
        }
    }
}

impl Mini for Cronol {
    fn side(&self) -> Player {
        self.side
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            Ph::Draw => vec!["trag".into()],
            Ph::Verify => vec!["verific".into()],
            Ph::Place => (0..=self.line[pi(self.side)].len())
                .map(|k| format!("pun {k}"))
                .collect(),
        }
    }
    fn step(&mut self, m: &str) {
        let me = pi(self.side);
        match self.ph {
            Ph::Draw => {
                if self.deck.is_empty() {
                    let mut d: Vec<usize> = (0..ITEMS.len())
                        .filter(|&i| !self.line[0].contains(&i) && !self.line[1].contains(&i))
                        .collect();
                    shuf(&mut d, &mut self.rng);
                    self.deck = d;
                }
                self.cur = self.deck.pop().unwrap_or(0);
                self.ph = Ph::Place;
            }
            Ph::Place => {
                self.slot = m[4..].parse().unwrap();
                self.ph = Ph::Verify;
            }
            Ph::Verify => {
                let y = ITEMS[self.cur].1;
                let l = &self.line[me];
                let lo_ok = self.slot == 0 || ITEMS[l[self.slot - 1]].1 <= y;
                let hi_ok = self.slot == l.len() || y <= ITEMS[l[self.slot]].1;
                if lo_ok && hi_ok {
                    self.line[me].insert(self.slot, self.cur);
                }
                if me == 1 {
                    self.turns += 1;
                }
                self.side = self.side.other();
                self.ph = Ph::Draw;
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        let (a, b) = (self.line[0].len(), self.line[1].len());
        if self.side == Player::One && self.ph == Ph::Draw {
            if a >= self.sp.goal || b >= self.sp.goal {
                return Some(by_score(a as i32, b as i32));
            }
            if self.turns >= self.sp.cap {
                return Some(by_score(a as i32, b as i32));
            }
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        if self.ph == Ph::Verify && self.side == p {
            // șansa ca locul ales să fie corect, după „memoria” AI-ului
            let y = memory(self.cur);
            let l = &self.line[a];
            let lo = if self.slot == 0 {
                -9999
            } else {
                ITEMS[l[self.slot - 1]].1
            };
            let hi = if self.slot == l.len() {
                9999
            } else {
                ITEMS[l[self.slot]].1
            };
            let (lo_y, hi_y) = (lo, hi);
            let miss = (lo_y - y).max(0) + (y - hi_y).max(0);
            return 1000 - 40 * miss.min(25) - ((hi_y - lo_y).min(300) / 10);
        }
        0
    }
    fn show(&self) -> String {
        let mut s = format!(
            "{} — tura {}/{} | țintă {} cărți\n",
            self.sp.title,
            self.turns + 1,
            self.sp.cap,
            self.sp.goal
        );
        for p in 0..2 {
            let t: Vec<String> = self.line[p]
                .iter()
                .map(|&i| format!("{} ({})", ITEMS[i].0, ITEMS[i].1))
                .collect();
            s.push_str(&format!("cronologia lui J{}: {}\n", p + 1, t.join(" < ")));
        }
        if self.ph != Ph::Draw {
            s.push_str(&format!(
                "carte trasă: {} (anul ascuns)\n",
                ITEMS[self.cur].0
            ));
        }
        s.push_str(&format!(
            "La mutare: {} (trag | pun k = poziția 0..{} | verific)\n",
            pname(self.side),
            self.line[pi(self.side)].len()
        ));
        s
    }
}
