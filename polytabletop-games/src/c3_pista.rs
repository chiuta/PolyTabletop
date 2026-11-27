//! Motor cooperativ „cursă împotriva unui pericol” (lista C3, variante originale, 2 jucători în echipă).
//!
//! O pistă de `len` spații colorate (culori 0..C). `k` pioni pornesc de la start (poziția 0 =
//! în afara pistei) și trebuie să ajungă la sfârșit înainte ca pericolul (soarele / căpcăunul)
//! să parcurgă `danger` pași. Un pion mută la următorul spațiu din fața lui cu culoarea cerută.
//! * Mod cărți (mâini deschise de 3): `joc i p` joacă cartea i cu pionul p, `arunc i` o aruncă;
//!   tragerea (`pesc`) e o mutare forțată; cărțile „soare” trase avansează pericolul.
//! * Mod zar: aruncarea (`zar`) e forțată; apoi `mut p` mută pionul p la culoarea căzută
//!   (fața „căpcăun” avansează pericolul; fața „joker” mută la spațiul următor oricare ar fi).
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

pub struct PSpec {
    pub title: &'static str,
    pub len: usize,
    pub colors: usize,
    pub pawns: usize,
    pub danger: u32,
    pub dice: bool,
    pub suns: usize,
    pub per_color: usize,
}

pub static SPECS: [PSpec; 2] = [
    PSpec {
        title: "Zborul Bufnițelor",
        len: 24,
        colors: 6,
        pawns: 4,
        danger: 13,
        dice: false,
        suns: 10,
        per_color: 8,
    },
    PSpec {
        title: "Ogrul pe Urme",
        len: 20,
        colors: 4,
        pawns: 4,
        danger: 9,
        dice: true,
        suns: 0,
        per_color: 0,
    },
];

#[derive(Clone)]
pub struct Pista {
    sp: &'static PSpec,
    track: Vec<u8>,
    pos: Vec<usize>,
    danger: u32,
    deck: Vec<u8>,
    hands: [Vec<u8>; 2],
    face: Option<u8>,
    refill: bool,
    side: usize,
    rng: Rng,
}

pub fn new(i: usize) -> Pista {
    with_seed(i, seed_now())
}

pub fn with_seed(i: usize, seed: u64) -> Pista {
    let sp = &SPECS[i];
    let mut rng = Rng::new(seed);
    let track: Vec<u8> = (0..sp.len)
        .map(|k| ((k * 5 + rng.below(3)) % sp.colors) as u8)
        .collect();
    let mut deck = Vec::new();
    for c in 0..sp.colors {
        for _ in 0..sp.per_color {
            deck.push(c as u8);
        }
    }
    deck.extend(std::iter::repeat_n(255, sp.suns));
    shuf(&mut deck, &mut rng);
    let mut g = Pista {
        sp,
        track,
        pos: vec![0; sp.pawns],
        danger: 0,
        deck,
        hands: [Vec::new(), Vec::new()],
        face: None,
        refill: false,
        side: 0,
        rng,
    };
    if !sp.dice {
        for h in 0..2 {
            for _ in 0..3 {
                let c = g.draw_card();
                g.hands[h].push(c);
            }
        }
    }
    g
}

impl Pista {
    fn draw_card(&mut self) -> u8 {
        loop {
            if self.deck.is_empty() {
                for c in 0..self.sp.colors {
                    for _ in 0..self.sp.per_color {
                        self.deck.push(c as u8);
                    }
                }
                for _ in 0..self.sp.suns {
                    self.deck.push(255);
                }
                shuf(&mut self.deck, &mut self.rng);
            }
            let c = self.deck.pop().unwrap();
            if c == 255 {
                self.danger += 1;
            } else {
                return c;
            }
        }
    }
    /// Locul în care ajunge pionul `p` cu culoarea `c` (pozițiile 1..=len; len+1 = ajuns).
    fn dest(&self, p: usize, c: Option<u8>) -> Option<usize> {
        let cur = self.pos[p];
        if cur > self.sp.len {
            return None;
        }
        let mut k = cur + 1;
        while k <= self.sp.len {
            if c.is_none() || self.track[k - 1] == c.unwrap() {
                return Some(k);
            }
            k += 1;
        }
        Some(self.sp.len + 1)
    }
    fn done(&self) -> bool {
        self.pos.iter().all(|&p| p > self.sp.len)
    }
}

impl Mini for Pista {
    fn side(&self) -> Player {
        if self.side == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        if self.sp.dice {
            match self.face {
                None => return vec!["zar".into()],
                Some(f) => {
                    let mut v: Vec<String> = (0..self.sp.pawns)
                        .filter(|&p| self.pos[p] <= self.sp.len && f as usize != self.sp.colors)
                        .map(|p| format!("mut {}", p + 1))
                        .collect();
                    if v.is_empty() {
                        v.push("pas".into());
                    }
                    return v;
                }
            }
        }
        if self.refill {
            return vec!["pesc".into()];
        }
        let mut v = Vec::new();
        for i in 0..self.hands[self.side].len() {
            for p in 0..self.sp.pawns {
                if self.pos[p] <= self.sp.len {
                    v.push(format!("joc {} {}", i + 1, p + 1));
                }
            }
            v.push(format!("arunc {}", i + 1));
        }
        v.sort();
        v.dedup();
        v
    }
    fn step(&mut self, m: &str) {
        if self.sp.dice {
            match self.face {
                None => {
                    self.face = Some(self.rng.below(self.sp.colors + 2) as u8);
                    return;
                }
                Some(f) => {
                    let fi = f as usize;
                    if fi == self.sp.colors {
                        self.danger += 1;
                    } else if let Some(p) = m.strip_prefix("mut ") {
                        let p = p.parse::<usize>().unwrap() - 1;
                        let col = if fi < self.sp.colors { Some(f) } else { None };
                        self.pos[p] = self.dest(p, col).unwrap();
                    }
                    self.face = None;
                    self.side = 1 - self.side;
                    return;
                }
            }
        }
        if self.refill {
            let c = self.draw_card();
            self.hands[self.side].push(c);
            self.refill = false;
            self.side = 1 - self.side;
            return;
        }
        let (kind, rest) = m.split_once(' ').unwrap();
        let mut it = rest.split(' ');
        let i = it.next().unwrap().parse::<usize>().unwrap() - 1;
        let c = self.hands[self.side].remove(i);
        if kind == "joc" {
            let p = it.next().unwrap().parse::<usize>().unwrap() - 1;
            self.pos[p] = self.dest(p, Some(c)).unwrap();
        }
        self.refill = true;
    }
    fn result(&self) -> Option<Outcome> {
        if self.done() {
            Some(Outcome::Win(Player::One))
        } else if self.danger >= self.sp.danger {
            Some(Outcome::Draw)
        } else {
            None
        }
    }
    fn eval(&self, _p: Player) -> i32 {
        let adv: i32 = self
            .pos
            .iter()
            .map(|&p| p.min(self.sp.len + 1) as i32)
            .sum();
        let fin = self.pos.iter().filter(|&&p| p > self.sp.len).count() as i32;
        adv * 5 + fin * 40 - self.danger as i32 * 20
    }
    fn show(&self) -> String {
        let mut s = format!(
            "{} — pericol {}/{} | pioni la țintă {}/{}\n",
            self.sp.title,
            self.danger,
            self.sp.danger,
            self.pos.iter().filter(|&&p| p > self.sp.len).count(),
            self.sp.pawns
        );
        let t: Vec<String> = self.track.iter().map(|c| format!("{}", c + 1)).collect();
        s.push_str(&format!("Pista (culori): {}\n", t.join(" ")));
        for p in 0..self.sp.pawns {
            if self.pos[p] > self.sp.len {
                s.push_str(&format!("Pionul {}: ajuns\n", p + 1));
            } else {
                s.push_str(&format!("Pionul {}: spațiul {}\n", p + 1, self.pos[p]));
            }
        }
        if self.sp.dice {
            match self.face {
                None => s.push_str("Urmează aruncarea zarului.\n"),
                Some(f) => {
                    let n = if (f as usize) < self.sp.colors {
                        format!("culoarea {}", f + 1)
                    } else if f as usize == self.sp.colors {
                        "căpcăun".to_string()
                    } else {
                        "joker".to_string()
                    };
                    s.push_str(&format!("Zar: {n}\n"));
                }
            }
        } else {
            for h in 0..2 {
                let c: Vec<String> = self.hands[h].iter().map(|c| (c + 1).to_string()).collect();
                s.push_str(&format!("Mâna J{} (deschisă): {}\n", h + 1, c.join(" ")));
            }
        }
        s.push_str(&format!("La mutare: {}\n", pname(self.side())));
        s
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        check(|s| with_seed(0, s), 3000);
        check(|s| with_seed(1, s), 3000);
    }
}
