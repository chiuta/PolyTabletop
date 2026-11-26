//! Lanțuri Hoteliere — fuziuni și acțiuni pe o tablă mică (variantă originală, 2 jucători).
//!
//! Tablă 5×6 (coloane a–f, rânduri 1–5). Trei lanțuri: A, B, C. Piața are 4 plăcuțe vizibile
//! (celule) din pachetul amestecat; pe tură iei una: `pune b3`. Fără vecini: rămâne izolată. Lângă
//! plăcuțe izolate formează un lanț nou: `fond A` (primești o acțiune gratuită; doar dacă lanțul nu
//! există încă). Lângă un lanț: îl mărește. Între două lanțuri: fuziune — cel mai mare (egal: A<B<C)
//! rămâne, celălalt dispare; lanțurile de cel puțin 7 celule nu pot fi absorbite. La fuziune
//! acționarii vând automat acțiunile absorbite la prețul curent (3 × mărimea) și cel cu cele mai
//! multe acțiuni primește bonus 4 × mărimea (egal: se împarte). Apoi cumperi până la 3 acțiuni:
//! `compro A` ... `gata`. Se termină când un lanț are 10 celule, nu mai sunt mutări sau după 60 de
//! mutări: se vând toate acțiunile și se dau bonusuri (major 4×mărimea, minor 2×). Câștigă banii mai mulți.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const R: usize = 5;
const C: usize = 6;
const NAMES: [char; 3] = ['A', 'B', 'C'];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ph {
    Place,
    Found(usize),
    Buy(u8),
}

#[derive(Clone)]
pub struct Lanturi {
    cell: [[i8; C]; R], // -1 gol, 3 = plăcuță izolată, 0..2 = lanț
    market: Vec<(usize, usize)>,
    deck: Vec<(usize, usize)>,
    shares: [[i32; 3]; 2],
    cash: [i32; 2],
    ph: Ph,
    pending: Option<(usize, usize)>,
    side: Player,
    ply: u32,
    over: bool,
}

fn nm(r: usize, c: usize) -> String {
    format!("{}{}", (b'a' + c as u8) as char, r + 1)
}
fn pm(s: &str) -> (usize, usize) {
    let b = s.as_bytes();
    (
        (b[1] - b'1') as usize,
        (b[0].to_ascii_lowercase() - b'a') as usize,
    )
}

impl Lanturi {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck: Vec<(usize, usize)> =
            (0..R).flat_map(|r| (0..C).map(move |c| (r, c))).collect();
        shuf(&mut deck, &mut rng);
        let market = (0..4).filter_map(|_| deck.pop()).collect();
        Lanturi {
            cell: [[-1; C]; R],
            market,
            deck,
            shares: [[0; 3]; 2],
            cash: [60; 2],
            ph: Ph::Place,
            pending: None,
            side: Player::One,
            ply: 0,
            over: false,
        }
    }
    fn size(&self, ch: usize) -> i32 {
        self.cell
            .iter()
            .flatten()
            .filter(|&&x| x == ch as i8)
            .count() as i32
    }
    fn price(&self, ch: usize) -> i32 {
        3 * self.size(ch)
    }
    fn nbrs(r: usize, c: usize) -> Vec<(usize, usize)> {
        let mut v = Vec::new();
        for (dr, dc) in [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
            let (a, b) = (r as i32 + dr, c as i32 + dc);
            if (0..R as i32).contains(&a) && (0..C as i32).contains(&b) {
                v.push((a as usize, b as usize));
            }
        }
        v
    }
    /// Analiza unei plăcuțe: (lanțuri vecine distincte, există vecini izolați?)
    fn around(&self, r: usize, c: usize) -> (Vec<usize>, bool) {
        let mut ch = Vec::new();
        let mut iso = false;
        for (a, b) in Self::nbrs(r, c) {
            match self.cell[a][b] {
                -1 => {}
                3 => iso = true,
                x => {
                    if !ch.contains(&(x as usize)) {
                        ch.push(x as usize);
                    }
                }
            }
        }
        ch.sort();
        (ch, iso)
    }
    fn active(&self, ch: usize) -> bool {
        self.size(ch) > 0
    }
    fn allowed(&self, r: usize, c: usize) -> bool {
        let (ch, iso) = self.around(r, c);
        if ch.len() >= 2 {
            return ch.iter().filter(|&&x| self.size(x) >= 7).count() < 2;
        }
        if ch.is_empty() && iso {
            return (0..3).any(|x| !self.active(x));
        }
        true
    }
    fn flood(&mut self, r: usize, c: usize, to: i8) {
        // transformă plăcuțele izolate conectate în lanțul `to`
        let mut stack = vec![(r, c)];
        self.cell[r][c] = to;
        while let Some((a, b)) = stack.pop() {
            for (x, y) in Self::nbrs(a, b) {
                if self.cell[x][y] == 3 {
                    self.cell[x][y] = to;
                    stack.push((x, y));
                }
            }
        }
    }
    fn bonus_pay(&mut self, ch: usize, major: i32, minor: i32, size: i32) {
        let (a, b) = (self.shares[0][ch], self.shares[1][ch]);
        if a == 0 && b == 0 {
            return;
        }
        if a > b {
            self.cash[0] += major * size;
            self.cash[1] += minor * size;
        } else if b > a {
            self.cash[1] += major * size;
            self.cash[0] += minor * size;
        } else {
            let half = (major + minor) * size / 2;
            self.cash[0] += half;
            self.cash[1] += half;
        }
    }
    fn finish(&mut self) {
        for ch in 0..3 {
            let size = self.size(ch);
            if size > 0 {
                self.bonus_pay(ch, 4, 2, size);
                for p in 0..2 {
                    self.cash[p] += self.shares[p][ch] * self.price(ch);
                    self.shares[p][ch] = 0;
                }
            }
        }
        self.over = true;
    }
    fn worth(&self, p: usize) -> i32 {
        self.cash[p]
            + (0..3)
                .map(|ch| self.shares[p][ch] * self.price(ch))
                .sum::<i32>()
    }
}

impl Default for Lanturi {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Lanturi {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        self.side
    }
    fn moves(&self) -> Vec<String> {
        let me = pi(self.side);
        match self.ph {
            Ph::Place => {
                let v: Vec<String> = self
                    .market
                    .iter()
                    .filter(|&&(r, c)| self.allowed(r, c))
                    .map(|&(r, c)| format!("pune {}", nm(r, c)))
                    .collect();
                if v.is_empty() {
                    vec!["pas".into()]
                } else {
                    v
                }
            }
            Ph::Found(_) => (0..3)
                .filter(|&x| !self.active(x))
                .map(|x| format!("fond {}", NAMES[x]))
                .collect(),
            Ph::Buy(left) => {
                let mut v = vec!["gata".to_string()];
                if left > 0 {
                    for ch in 0..3 {
                        if self.active(ch) && self.cash[me] >= self.price(ch) {
                            v.push(format!("compro {}", NAMES[ch]));
                        }
                    }
                }
                v
            }
        }
    }
    fn step(&mut self, m: &str) {
        let me = pi(self.side);
        self.ply += 1;
        match self.ph {
            Ph::Place => {
                if m == "pas" {
                    self.finish();
                    return;
                }
                let (r, c) = pm(&m[5..]);
                let i = self.market.iter().position(|&x| x == (r, c)).unwrap();
                self.market.remove(i);
                if let Some(n) = self.deck.pop() {
                    self.market.push(n);
                }
                let (ch, iso) = self.around(r, c);
                if ch.len() >= 2 {
                    // fuziune: cel mai mare rămâne
                    let keep = *ch
                        .iter()
                        .max_by_key(|&&x| (self.size(x), -(x as i32)))
                        .unwrap();
                    for &x in &ch {
                        if x != keep {
                            let size = self.size(x);
                            self.bonus_pay(x, 4, 0, size);
                            for p in 0..2 {
                                self.cash[p] += self.shares[p][x] * self.price(x);
                                self.shares[p][x] = 0;
                            }
                            for row in self.cell.iter_mut() {
                                for cl in row.iter_mut() {
                                    if *cl == x as i8 {
                                        *cl = keep as i8;
                                    }
                                }
                            }
                        }
                    }
                    self.flood(r, c, keep as i8);
                    self.ph = Ph::Buy(3);
                } else if ch.len() == 1 {
                    self.flood(r, c, ch[0] as i8);
                    self.ph = Ph::Buy(3);
                } else if iso {
                    self.cell[r][c] = 3;
                    self.pending = Some((r, c));
                    self.ph = Ph::Found(0);
                } else {
                    self.cell[r][c] = 3;
                    self.ph = Ph::Buy(3);
                }
            }
            Ph::Found(_) => {
                let ch = NAMES.iter().position(|&x| x.to_string() == m[5..]).unwrap();
                let (r, c) = self.pending.take().unwrap();
                self.flood(r, c, ch as i8);
                self.shares[me][ch] += 1;
                self.ph = Ph::Buy(3);
            }
            Ph::Buy(left) => {
                if m == "gata" {
                    self.ph = Ph::Place;
                    self.side = self.side.other();
                    if (0..3).any(|x| self.size(x) >= 10) || self.ply > 120 {
                        self.finish();
                    }
                } else {
                    let ch = NAMES.iter().position(|&x| x.to_string() == m[7..]).unwrap();
                    self.cash[me] -= self.price(ch);
                    self.shares[me][ch] += 1;
                    self.ph = Ph::Buy(left - 1);
                }
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.over {
            return Some(by_score(self.cash[0], self.cash[1]));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        self.worth(a) - self.worth(1 - a)
    }
    fn show(&self) -> String {
        let mut s = String::from(
            "Lanțuri Hoteliere — . gol, + plăcuță izolată, A B C lanțuri\n   a b c d e f\n",
        );
        for r in 0..R {
            s.push_str(&format!("{}  ", r + 1));
            for c in 0..C {
                s.push(match self.cell[r][c] {
                    -1 => '.',
                    3 => '+',
                    x => NAMES[x as usize],
                });
                s.push(' ');
            }
            s.push('\n');
        }
        let mk: Vec<String> = self.market.iter().map(|&(r, c)| nm(r, c)).collect();
        s.push_str(&format!(
            "Piață: {} | pachet {}\n",
            mk.join(" "),
            self.deck.len()
        ));
        for ch in 0..3 {
            s.push_str(&format!(
                "{}: mărime {} preț {} | ",
                NAMES[ch],
                self.size(ch),
                self.price(ch)
            ));
        }
        s.push('\n');
        for p in 0..2 {
            s.push_str(&format!(
                "J{}: bani {} | acțiuni {:?} | valoare {}\n",
                p + 1,
                self.cash[p],
                self.shares[p],
                self.worth(p)
            ));
        }
        s.push_str(&format!(
            "La mutare: {} (faza {:?}: pune b3 | fond A | compro A | gata)\n",
            pname(self.side),
            self.ph
        ));
        s
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        crate::brand_util::check(Lanturi::with_seed, 400);
    }
    #[test]
    fn merger_keeps_larger_chain() {
        let mut g = Lanturi::with_seed(2);
        g.cell = [[-1; C]; R];
        for c in 0..3 {
            g.cell[0][c] = 0;
        }
        g.cell[0][4] = 1;
        g.cell[0][5] = 1;
        g.market = vec![(0, 3)];
        g.shares[0][1] = 2;
        g.step("pune d1");
        assert_eq!(g.size(0), 6);
        assert_eq!(g.size(1), 0);
        assert!(g.cash[0] > 60);
    }
}
