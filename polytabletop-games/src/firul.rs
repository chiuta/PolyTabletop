//! Firul Luminii — jucători care urmează drumuri pe plăci (variantă originală pe o tablă 6×6).
//!
//! Fiecare placă are 8 capete (câte 2 pe fiecare latură, numerotate 0–7 în sens orar de la
//! stânga-sus) unite două câte două. Ficare jucător are un jeton care intră pe o muchie a tablei;
//! la tura ta pui una din cele 3 plăci ale tale (mâinile sunt deschise) în celula în care urmează
//! să intre jetonul tău, rotită 0–3 sferturi: `pune t,r`. Apoi toate jetoanele aleargă pe
//! drumuri până la prima celulă goală. Cine iese de pe tablă e eliminat; ciocnirea în același
//! punct elimină ambii jucători. Câștigă ultimul rămas (ambii eliminați: remiză; plăci epuizate: remiză).
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const MAPPING: [usize; 8] = [5, 4, 7, 6, 1, 0, 3, 2];
const DR: [i32; 4] = [-1, 0, 1, 0];
const DC: [i32; 4] = [0, 1, 0, -1];

type Tile = [u8; 8];

fn rotate(t: &Tile, k: usize) -> Tile {
    let mut o = [0u8; 8];
    for e in 0..8 {
        let ne = (e + 2 * k) % 8;
        o[ne] = ((t[e] as usize + 2 * k) % 8) as u8;
    }
    o
}

#[derive(Clone)]
pub struct Firul {
    board: [[Option<Tile>; 6]; 6],
    tok: [(i32, i32, usize); 2],
    alive: [bool; 2],
    hand: [Vec<Tile>; 2],
    deck: Vec<Tile>,
    side: Player,
    over: bool,
}

impl Firul {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck: Vec<Tile> = Vec::new();
        for _ in 0..35 {
            let mut pts: Vec<u8> = (0..8).collect();
            shuf(&mut pts, &mut rng);
            let mut t = [0u8; 8];
            for k in 0..4 {
                let (a, b) = (pts[2 * k] as usize, pts[2 * k + 1]);
                t[a] = b;
                t[b as usize] = a as u8;
            }
            deck.push(t);
        }
        let h1: Vec<Tile> = (0..3).map(|_| deck.pop().unwrap()).collect();
        let h2: Vec<Tile> = (0..3).map(|_| deck.pop().unwrap()).collect();
        Firul {
            board: [[None; 6]; 6],
            tok: [(0, 1, 0), (5, 4, 4)],
            alive: [true; 2],
            hand: [h1, h2],
            deck,
            side: Player::One,
            over: false,
        }
    }
    /// Avansează un jeton cât timp celula țintă e ocupată.
    fn advance(&mut self, p: usize) {
        for _ in 0..80 {
            if !self.alive[p] {
                return;
            }
            let (r, c, e) = self.tok[p];
            let Some(t) = self.board[r as usize][c as usize] else {
                return;
            };
            let f = t[e] as usize;
            let d = f / 2;
            let (nr, nc) = (r + DR[d], c + DC[d]);
            if !(0..6).contains(&nr) || !(0..6).contains(&nc) {
                self.alive[p] = false;
                return;
            }
            self.tok[p] = (nr, nc, MAPPING[f]);
        }
        self.alive[p] = false;
    }
    fn safe_count(&self, p: usize) -> i32 {
        if !self.alive[p] {
            return -50;
        }
        let (r, c, e) = self.tok[p];
        if self.board[r as usize][c as usize].is_some() {
            return 0;
        }
        let mut n = 0;
        for t in &self.hand[p] {
            for k in 0..4 {
                let rt = rotate(t, k);
                let f = rt[e] as usize;
                let (nr, nc) = (r + DR[f / 2], c + DC[f / 2]);
                if (0..6).contains(&nr) && (0..6).contains(&nc) {
                    n += 1;
                }
            }
        }
        n
    }
}

impl Default for Firul {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Firul {
    fn depth(&self) -> u32 {
        1
    }
    fn side(&self) -> Player {
        self.side
    }
    fn moves(&self) -> Vec<String> {
        let p = pi(self.side);
        let mut v = Vec::new();
        for (i, t) in self.hand[p].iter().enumerate() {
            let mut seen: Vec<Tile> = Vec::new();
            for k in 0..4 {
                let rt = rotate(t, k);
                if !seen.contains(&rt) {
                    seen.push(rt);
                    v.push(format!("pune {},{}", i + 1, k));
                }
            }
        }
        if v.is_empty() {
            v.push("pas".into());
        }
        v
    }
    fn step(&mut self, m: &str) {
        let p = pi(self.side);
        if m == "pas" {
            self.over = true;
            return;
        }
        let r = m.strip_prefix("pune ").unwrap();
        let (i, k) = r.split_once(',').unwrap();
        let t = self.hand[p].remove(i.parse::<usize>().unwrap() - 1);
        let (r0, c0, _) = self.tok[p];
        self.board[r0 as usize][c0 as usize] = Some(rotate(&t, k.parse().unwrap()));
        if let Some(n) = self.deck.pop() {
            self.hand[p].push(n);
        }
        self.advance(0);
        self.advance(1);
        if self.alive[0] && self.alive[1] && self.tok[0] == self.tok[1] {
            self.alive = [false, false];
        }
        if !self.alive[0] || !self.alive[1] {
            self.over = true;
        }
        self.side = self.side.other();
    }
    fn result(&self) -> Option<Outcome> {
        if !self.over {
            return None;
        }
        Some(match (self.alive[0], self.alive[1]) {
            (true, false) => Outcome::Win(Player::One),
            (false, true) => Outcome::Win(Player::Two),
            _ => Outcome::Draw,
        })
    }
    fn eval(&self, p: Player) -> i32 {
        self.safe_count(pi(p)) - self.safe_count(1 - pi(p))
    }
    fn show(&self) -> String {
        let mut s = String::from("Firul Luminii — celule cu plăci (r,c); jeton J1=1, J2=2\n");
        for r in 0..6 {
            for c in 0..6 {
                let mut ch = ".";
                if self.board[r][c].is_some() {
                    ch = "#";
                }
                for p in 0..2 {
                    if self.alive[p] && self.tok[p].0 == r as i32 && self.tok[p].1 == c as i32 {
                        ch = if p == 0 { "1" } else { "2" };
                    }
                }
                s.push_str(ch);
                s.push(' ');
            }
            s.push('\n');
        }
        for p in 0..2 {
            let (r, c, e) = self.tok[p];
            s.push_str(&format!(
                "J{}: jeton la celula ({},{}) capătul {}{}\n",
                p + 1,
                r + 1,
                c + 1,
                e,
                if self.alive[p] { "" } else { " (eliminat)" }
            ));
            for (i, t) in self.hand[p].iter().enumerate() {
                let mut pr = Vec::new();
                for a in 0..8 {
                    if (t[a] as usize) > a {
                        pr.push(format!("{}-{}", a, t[a]));
                    }
                }
                s.push_str(&format!("   placa {}: {}\n", i + 1, pr.join(" ")));
            }
        }
        s.push_str(&format!(
            "La mutare: {} (pune t,r — placa t, rotită r sferturi) | pachet: {}\n",
            pname(self.side),
            self.deck.len()
        ));
        s
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        crate::brand_util::check(Firul::with_seed, 100);
    }
    #[test]
    fn rotation_is_involution_after_four() {
        let g = Firul::with_seed(2);
        let t = g.hand[0][0];
        assert_eq!(rotate(&t, 4), t);
        let r = rotate(&t, 1);
        for a in 0..8 {
            assert_eq!(r[r[a] as usize] as usize, a);
        }
    }
}
