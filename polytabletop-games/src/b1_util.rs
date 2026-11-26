//! Ajutoare comune pentru jocurile B1 (variante proprii ale jocurilor de marcă):
//! notație de pătrate, desen de grilă, hash și un verificator generic pentru teste.
#![allow(clippy::all)]

use polytabletop_core::Player;

/// Numele unui pătrat: `a1` (coloană literă, rând număr), `x`/`y` de la 0.
pub fn sq(x: usize, y: usize) -> String {
    format!("{}{}", (b'a' + x as u8) as char, y + 1)
}

/// Citește `a1` -> (x, y) dacă încape în `w`×`h`.
pub fn parse_sq(s: &str, w: usize, h: usize) -> Option<(usize, usize)> {
    let b = s.as_bytes();
    if b.len() < 2 || !b[0].is_ascii_lowercase() {
        return None;
    }
    let x = (b[0] - b'a') as usize;
    let y: usize = s[1..].parse().ok()?;
    (x < w && y >= 1 && y <= h).then(|| (x, y - 1))
}

/// Grilă `w`×`h` cu etichete; rândul cel mai de sus afișat = ultimul rând.
pub fn grid(w: usize, h: usize, f: impl Fn(usize, usize) -> char) -> String {
    let mut s = String::new();
    for y in (0..h).rev() {
        s.push_str(&format!("{:>2} ", y + 1));
        for x in 0..w {
            s.push(f(x, y));
            s.push(' ');
        }
        s.push('\n');
    }
    s.push_str("   ");
    for x in 0..w {
        s.push((b'a' + x as u8) as char);
        s.push(' ');
    }
    s.push('\n');
    s
}

/// Hash Zobrist-ish: `tag` + jucătorul la mutare + perechi (cheie, valoare) nenule.
pub fn hkey(tag: u64, side: Player, it: impl Iterator<Item = (usize, u64)>) -> u64 {
    let mut h = polytabletop_core::hash_mix(tag << 40 | (side.idx() as u64) << 32);
    for (i, v) in it {
        if v != 0 {
            h ^= polytabletop_core::hash_mix(v << 20 | i as u64);
        }
    }
    h
}

/// Cel mai lung șir de `who` care trece prin (x, y) pe direcția (dx, dy) (ambele sensuri).
pub fn run_through(
    w: i32,
    h: i32,
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    is: impl Fn(i32, i32) -> bool,
) -> i32 {
    let mut n = 1;
    for s in [1, -1] {
        let (mut cx, mut cy) = (x + dx * s, y + dy * s);
        while cx >= 0 && cy >= 0 && cx < w && cy < h && is(cx, cy) {
            n += 1;
            cx += dx * s;
            cy += dy * s;
        }
    }
    n
}

#[cfg(test)]
pub mod testing {
    use polytabletop_core::rng::Rng;
    use polytabletop_core::{ai, Game, Outcome};

    /// Joacă la întâmplare până la capăt; verifică mutări nevide și text unic.
    pub fn playout<G: Game>(mut g: G, seed: u64, cap: usize) -> (G, usize) {
        let mut rng = Rng::new(seed);
        let mut n = 0;
        while g.outcome().is_none() && n < cap {
            let mv = g.legal_moves();
            assert!(
                !mv.is_empty(),
                "fără mutări dar jocul nu e gata\n{}",
                g.render()
            );
            let mut t: Vec<String> = mv.iter().map(|m| g.format_move(m)).collect();
            t.sort();
            let l = t.len();
            t.dedup();
            assert_eq!(l, t.len(), "text de mutare duplicat\n{}", g.render());
            g.apply(&mv[rng.below(mv.len())]);
            n += 1;
        }
        (g, n)
    }

    /// Verificare generică: partide aleatoare se termină, AI-ul alege mutări legale.
    pub fn check<G: Game>(mk: impl Fn(u64) -> G, cap: usize) {
        ai::set_time_budget_override_ms(Some(5));
        for seed in 1..=6u64 {
            let (g, n) = playout(mk(seed), seed, cap);
            assert!(
                g.outcome().is_some(),
                "nu s-a terminat în {n} pași\n{}",
                g.render()
            );
            assert!(!g.render().is_empty());
        }
        for seed in 1..=3u64 {
            let mut g = mk(seed);
            let mut rng = Rng::new(seed + 77);
            for step in 0..40 {
                if g.outcome().is_some() {
                    break;
                }
                let mv = g.legal_moves();
                if step % 4 == 0 {
                    let a = ai::best_move(&g, seed).expect("AI fără mutare");
                    assert!(mv.contains(&a), "AI a ales o mutare ilegală");
                }
                g.apply(&mv[rng.below(mv.len())]);
            }
        }
    }

    pub fn is_draw_or_win(o: Option<Outcome>) -> bool {
        o.is_some()
    }
}

/// Geometrie pentru o tablă hexagonală de rază `rad` (coordonate axiale q,r), stocată într-un
/// pătrat de `(2·rad+1)²` celule; celulele din afara hexagonului sunt invalide.
pub const HEXD: [(i32, i32); 6] = [(1, 0), (1, -1), (0, -1), (-1, 0), (-1, 1), (0, 1)];
pub const HEXN: [&str; 6] = ["e", "ne", "nv", "v", "sv", "se"];

#[derive(Clone, Copy)]
pub struct Hex {
    pub rad: i32,
}

impl Hex {
    pub fn side(&self) -> usize {
        (2 * self.rad + 1) as usize
    }
    pub fn size(&self) -> usize {
        self.side() * self.side()
    }
    pub fn valid(&self, q: i32, r: i32) -> bool {
        q.abs() <= self.rad && r.abs() <= self.rad && (q + r).abs() <= self.rad
    }
    pub fn idx(&self, q: i32, r: i32) -> usize {
        ((r + self.rad) * (2 * self.rad + 1) + q + self.rad) as usize
    }
    pub fn qr(&self, i: usize) -> (i32, i32) {
        (
            (i % self.side()) as i32 - self.rad,
            (i / self.side()) as i32 - self.rad,
        )
    }
    pub fn cells(&self) -> Vec<usize> {
        (0..self.size())
            .filter(|&i| {
                let (q, r) = self.qr(i);
                self.valid(q, r)
            })
            .collect()
    }
    /// Vecin în direcția `d` (0..6) dacă e pe tablă.
    pub fn step(&self, i: usize, d: usize) -> Option<usize> {
        let (q, r) = self.qr(i);
        let (nq, nr) = (q + HEXD[d].0, r + HEXD[d].1);
        self.valid(nq, nr).then(|| self.idx(nq, nr))
    }
    /// Etichetă: litera rândului (a = sus … ) + poziția în rând, ex. `d4` = centrul pentru rază 3.
    pub fn label(&self, i: usize) -> String {
        let (q, r) = self.qr(i);
        let qmin = (-self.rad).max(-self.rad - r);
        format!("{}{}", (b'a' + (r + self.rad) as u8) as char, q - qmin + 1)
    }
    pub fn parse(&self, s: &str) -> Option<usize> {
        self.cells().into_iter().find(|&i| self.label(i) == s)
    }
    /// Desen: rânduri indentate, `f(i)` dă simbolul celulei.
    pub fn draw(&self, f: impl Fn(usize) -> char) -> String {
        let mut s = String::new();
        for r in -self.rad..=self.rad {
            let qmin = (-self.rad).max(-self.rad - r);
            let qmax = self.rad.min(self.rad - r);
            s.push_str(&" ".repeat((r.unsigned_abs()) as usize));
            s.push_str(&format!("{} ", (b'a' + (r + self.rad) as u8) as char));
            for q in qmin..=qmax {
                s.push(f(self.idx(q, r)));
                s.push(' ');
            }
            s.push('\n');
        }
        s
    }
}
