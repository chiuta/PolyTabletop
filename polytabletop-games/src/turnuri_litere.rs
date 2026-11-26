//! Turnuri de Litere — cuvinte în încrucișare cu litere suprapuse (variantă originală, 2 jucători).
//!
//! Tablă 6×6; fiecare celulă poate avea o stivă de până la 3 litere (contează cea de sus).
//! Fiecare ține 7 litere (mână ascunsă) dintr-un sac de 60. Pe tură pui O literă: `pune L@r,c`
//! (pe celulă goală sau peste o stivă de sub 3 litere, cu altă literă decât cea de sus). Toate
//! șirurile orizontale și verticale de cel puțin 2 celule prin celula nouă trebuie să fie cuvinte
//! din dicționar, iar cel puțin un astfel de șir trebuie să existe (prima literă e liberă).
//! Scor: pentru fiecare cuvânt format, suma înălțimilor stivelor lui. `pas` dacă nu ai mutare. Jocul se
//! termină când sacul e gol și cineva rămâne fără litere, sau după 3 pase consecutive; literele
//! rămase se scad. Informație ascunsă: vezi doar mâna ta.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const N: usize = 6;
const DIST: [u8; 26] = [
    6, 2, 3, 3, 6, 1, 2, 1, 6, 1, 0, 3, 3, 4, 4, 3, 0, 4, 4, 4, 3, 1, 0, 1, 0, 1,
];

#[derive(Clone)]
pub struct TurnuriLitere {
    viewer: Option<Player>,
    top: [[i8; N]; N],
    height: [[u8; N]; N],
    rack: [Vec<u8>; 2],
    bag: Vec<u8>,
    score: [i32; 2],
    passes: u8,
    side: Player,
    ply: u32,
}

fn letter(c: u8) -> char {
    (b'a' + c) as char
}

impl TurnuriLitere {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut bag: Vec<u8> = Vec::new();
        for (l, &n) in DIST.iter().enumerate() {
            for _ in 0..(n as usize * 2) {
                bag.push(l as u8);
            }
        }
        shuf(&mut bag, &mut rng);
        bag.truncate(60);
        let h1: Vec<u8> = (0..7).map(|_| bag.pop().unwrap()).collect();
        let h2: Vec<u8> = (0..7).map(|_| bag.pop().unwrap()).collect();
        TurnuriLitere {
            viewer: None,
            top: [[-1; N]; N],
            height: [[0; N]; N],
            rack: [h1, h2],
            bag,
            score: [0; 2],
            passes: 0,
            side: Player::One,
            ply: 0,
        }
    }
    fn empty_board(&self) -> bool {
        self.height.iter().flatten().all(|&h| h == 0)
    }
    /// Cuvintele formate prin (r,c) după punerea literei `l` (valoare: suma înălțimilor) sau None dacă ilegal.
    fn try_place(&self, l: u8, r: usize, c: usize) -> Option<i32> {
        if self.height[r][c] >= 3 || self.top[r][c] == l as i8 {
            return None;
        }
        let mut t = self.top;
        let mut h = self.height;
        t[r][c] = l as i8;
        h[r][c] += 1;
        let mut total = 0;
        let mut words = 0;
        for horiz in [true, false] {
            let (mut a, mut b) = (r as i32, c as i32);
            let (dr, dc) = if horiz { (0, 1) } else { (1, 0) };
            while a - dr >= 0 && b - dc >= 0 && h[(a - dr) as usize][(b - dc) as usize] > 0 {
                a -= dr;
                b -= dc;
            }
            let mut w = Vec::new();
            let mut sum = 0;
            while (a as usize) < N && (b as usize) < N && h[a as usize][b as usize] > 0 {
                w.push(t[a as usize][b as usize] as u8);
                sum += h[a as usize][b as usize] as i32;
                a += dr;
                b += dc;
            }
            if w.len() >= 2 {
                if !is_word(&w) {
                    return None;
                }
                words += 1;
                total += sum;
            }
        }
        if words == 0 && !self.empty_board() {
            return None;
        }
        Some(total)
    }
    fn left(&self, p: usize) -> i32 {
        self.rack[p].len() as i32
    }
}

impl Default for TurnuriLitere {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for TurnuriLitere {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        self.side
    }
    fn moves(&self) -> Vec<String> {
        let me = pi(self.side);
        let mut v = Vec::new();
        let mut seen = Vec::new();
        for &l in &self.rack[me] {
            if seen.contains(&l) {
                continue;
            }
            seen.push(l);
            for r in 0..N {
                for c in 0..N {
                    if self.try_place(l, r, c).is_some() {
                        v.push(format!("pune {}@{},{}", letter(l), r + 1, c + 1));
                    }
                }
            }
        }
        if v.is_empty() {
            v.push("pas".into());
        }
        v
    }
    fn step(&mut self, m: &str) {
        let me = pi(self.side);
        self.ply += 1;
        if m == "pas" {
            self.passes += 1;
        } else {
            self.passes = 0;
            let (l, rc) = m[5..].split_once('@').unwrap();
            let l = l.as_bytes()[0].to_ascii_lowercase() - b'a';
            let (r, c) = rc.split_once(',').unwrap();
            let (r, c) = (
                r.parse::<usize>().unwrap() - 1,
                c.parse::<usize>().unwrap() - 1,
            );
            let pts = self.try_place(l, r, c).unwrap();
            self.score[me] += pts;
            self.top[r][c] = l as i8;
            self.height[r][c] += 1;
            let i = self.rack[me].iter().position(|&x| x == l).unwrap();
            self.rack[me].remove(i);
            if let Some(n) = self.bag.pop() {
                self.rack[me].push(n);
            }
        }
        self.side = self.side.other();
    }
    fn result(&self) -> Option<Outcome> {
        let out = self.bag.is_empty() && (self.rack[0].is_empty() || self.rack[1].is_empty());
        if out || self.passes >= 3 || self.ply > 300 {
            return Some(by_score(
                self.score[0] - self.left(0),
                self.score[1] - self.left(1),
            ));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        4 * (self.score[a] - self.score[1 - a]) + (self.left(1 - a) - self.left(a))
    }
    fn show(&self) -> String {
        let viewer = self.viewer.map(pi).unwrap_or(pi(self.side));
        let mut s = format!(
            "Turnuri de Litere — sac {} | scor J1 {}, J2 {}\n   1 2 3 4 5 6\n",
            self.bag.len(),
            self.score[0],
            self.score[1]
        );
        for r in 0..N {
            s.push_str(&format!("{}  ", r + 1));
            for c in 0..N {
                if self.height[r][c] == 0 {
                    s.push_str(". ");
                } else {
                    s.push(letter(self.top[r][c] as u8));
                    s.push((b'0' + self.height[r][c]) as char);
                }
            }
            s.push('\n');
        }
        let rk: String = self.rack[viewer]
            .iter()
            .map(|&l| letter(l).to_string())
            .collect::<Vec<_>>()
            .join(" ");
        s.push_str(&format!(
            "Mâna ta (J{}): {} | adversarul are {} litere\nLa mutare: {} (pune L@r,c | pas)\n",
            viewer + 1,
            rk,
            self.rack[1 - viewer].len(),
            pname(self.side)
        ));
        s
    }
    fn view(&mut self, w: Option<Player>) {
        self.viewer = w;
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        crate::brand_util::check(TurnuriLitere::with_seed, 300);
    }
    #[test]
    fn stacking_requires_a_new_letter() {
        let mut g = TurnuriLitere::with_seed(1);
        g.top[2][2] = 0;
        g.height[2][2] = 1;
        assert!(g.try_place(0, 2, 2).is_none());
    }
    #[test]
    fn hand_is_hidden_in_render() {
        let mut g = TurnuriLitere::with_seed(6);
        g.view(Some(Player::One));
        let s1 = g.show();
        g.view(Some(Player::Two));
        let s2 = g.show();
        assert!(s1.contains("Mâna ta (J1)") && s2.contains("Mâna ta (J2)"));
    }
    #[test]
    fn ai_ignores_the_opponent_rack_and_the_bag() {
        let a = TurnuriLitere::with_seed(6);
        let mut b = a.clone();
        b.rack[1].reverse();
        b.rack[1][0] = (b.rack[1][0] + 5) % 26;
        b.bag.reverse();
        for seed in 1..4 {
            assert_eq!(
                crate::brand_util::ai_of(&a, seed),
                crate::brand_util::ai_of(&b, seed)
            );
        }
    }
}
