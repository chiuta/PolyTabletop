//! Ajutor comun pentru mancalele cu tablă 4 × 8 (Bao, Hus): fiecare jucător deține
//! două rânduri (interior = „față”, exterior = „spate”) unite într-un inel de 16 gropi.
//!
//! Indexul de inel `r`: `0..=7` = rândul interior de la stânga la dreapta (din punctul
//! de vedere al proprietarului); `8..=15` = rândul exterior de la dreapta la stânga
//! (coloana exterioară = `15 - r`). Groapa interioară `c` a unui jucător se află față
//! în față cu groapa interioară `7 - c` a adversarului.

use polytabletop_core::Player;

pub const RING: usize = 16;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Kete {
    pub ring: [[u8; RING]; 2],
}

/// Coloana (1..=8) și rândul unei gropi de inel.
pub fn ring_name(r: usize) -> String {
    if r < 8 {
        format!("{}", r + 1)
    } else {
        format!("b{}", 15 - r + 1)
    }
}

/// Inversa lui [`ring_name`] (pentru teste).
pub fn parse_ring(s: &str) -> Option<usize> {
    if let Some(rest) = s.strip_prefix('b') {
        let c: usize = rest.parse().ok()?;
        (1..=8).contains(&c).then(|| 15 - (c - 1))
    } else {
        let c: usize = s.parse().ok()?;
        (1..=8).contains(&c).then(|| c - 1)
    }
}

pub fn step(r: usize, dir: i32) -> usize {
    ((r as i32 + dir).rem_euclid(RING as i32)) as usize
}

impl Kete {
    pub fn empty() -> Self {
        Kete {
            ring: [[0; RING]; 2],
        }
    }

    pub fn total(&self, p: Player) -> u32 {
        self.ring[p.idx()].iter().map(|&x| x as u32).sum()
    }

    pub fn inner_empty(&self, p: Player) -> bool {
        self.ring[p.idx()][..8].iter().all(|&x| x == 0)
    }

    /// Cele 4 rânduri pentru afișare: (Doi exterior, Doi interior, Unu interior, Unu exterior);
    /// rândurile lui Doi sunt oglindite ca să se potrivească coloanele față în față.
    pub fn render_rows(&self) -> [[u8; 8]; 4] {
        let one = &self.ring[0];
        let two = &self.ring[1];
        let mut o = [[0u8; 8]; 4];
        for s in 0..8 {
            // pe ecran, coloana s a lui Doi este coloana (7 - s) din punctul lui de vedere
            let c2 = 7 - s;
            o[0][s] = two[15 - c2];
            o[1][s] = two[c2];
            o[2][s] = one[s];
            o[3][s] = one[15 - s];
        }
        o
    }

    pub fn render(&self) -> String {
        let r = self.render_rows();
        let line = |row: &[u8; 8]| {
            row.iter()
                .map(|x| format!("{:>2}", x))
                .collect::<Vec<_>>()
                .join(" ")
        };
        let mut s = String::new();
        s.push_str(&format!("  J2 spate    {}\n", line(&r[0])));
        s.push_str(&format!("  J2 față     {}\n", line(&r[1])));
        s.push_str(&format!("  J1 față     {}\n", line(&r[2])));
        s.push_str(&format!("  J1 spate    {}\n", line(&r[3])));
        s.push_str("  coloane J1:  1  2  3  4  5  6  7  8\n");
        s
    }
}
