//! Konane (dame hawaiiană): 8×8, piese pe tot tabla în șah, capturi prin salt
//! ortogonal. Reguli publice, implementare originală.
//!
//! * Negrul (J1) ocupă pătratele cu (fișier+rând) par (a1, c1, …), albul (J2) pe cele impare.
//! * Deschidere: J1 ridică una dintre piesele sale d4, e5, a1 sau h8; J2 ridică
//!   apoi o piesă proprie vecină (ortogonal) cu locul rămas gol.
//! * Mutare: o piesă sare ortogonal peste o piesă adversă vecină într-un pătrat
//!   gol (scoțând-o); poate continua în aceeași direcție, peste 1–n piese.
//! * Cine nu mai poate muta pierde (fără remize).
//!
//! Mutări: `remove d4` (deschidere), `c3-e3` (un salt), `c3-g3` (salt dublu).

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: usize = 8;
const DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

#[derive(Clone, PartialEq, Debug)]
pub enum Mv {
    Remove(u8),
    /// from, direction index, number of jumps (>= 1)
    Jump(u8, u8, u8),
}

#[derive(Clone)]
pub struct Konane {
    b: [u8; N * N], // 0 empty, 1 black, 2 white; index = rank*8 + file
    side: Player,
    ply: u32,
}

impl Default for Konane {
    fn default() -> Self {
        Self::new()
    }
}

fn sq(i: usize) -> String {
    format!("{}{}", (b'a' + (i % N) as u8) as char, i / N + 1)
}

impl Konane {
    pub fn new() -> Self {
        let mut b = [0u8; N * N];
        for (i, c) in b.iter_mut().enumerate() {
            *c = if (i % N + i / N).is_multiple_of(2) {
                1
            } else {
                2
            };
        }
        Konane {
            b,
            side: Player::One,
            ply: 0,
        }
    }

    fn color(p: Player) -> u8 {
        p.idx() as u8 + 1
    }

    fn jumps_from(&self, i: usize, color: u8, out: &mut Vec<Mv>) {
        let (x, y) = ((i % N) as i32, (i / N) as i32);
        for (d, (dx, dy)) in DIRS.iter().enumerate() {
            let mut n = 0u8;
            loop {
                let (mx, my) = (x + dx * (2 * n as i32 + 1), y + dy * (2 * n as i32 + 1));
                let (tx, ty) = (mx + dx, my + dy);
                if tx < 0 || ty < 0 || tx >= N as i32 || ty >= N as i32 {
                    break;
                }
                let over = self.b[my as usize * N + mx as usize];
                let land = self.b[ty as usize * N + tx as usize];
                if over == 0 || over == color || land != 0 {
                    break;
                }
                n += 1;
                out.push(Mv::Jump(i as u8, d as u8, n));
            }
        }
    }

    fn gen(&self, p: Player) -> Vec<Mv> {
        let color = Self::color(p);
        let mut v = Vec::new();
        for i in 0..N * N {
            if self.b[i] == color {
                self.jumps_from(i, color, &mut v);
            }
        }
        v
    }

    fn mobility(&self, p: Player) -> i32 {
        self.gen(p).len() as i32
    }
}

impl Game for Konane {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        match self.ply {
            0 => [27usize, 36, 0, 63]
                .iter()
                .map(|&i| Mv::Remove(i as u8))
                .collect(),
            1 => {
                let gap = (0..N * N).find(|&i| self.b[i] == 0).unwrap();
                let (x, y) = ((gap % N) as i32, (gap / N) as i32);
                DIRS.iter()
                    .filter_map(|(dx, dy)| {
                        let (nx, ny) = (x + dx, y + dy);
                        (nx >= 0 && ny >= 0 && nx < N as i32 && ny < N as i32)
                            .then(|| Mv::Remove((ny as usize * N + nx as usize) as u8))
                    })
                    .collect()
            }
            _ => self.gen(self.side),
        }
    }

    fn apply(&mut self, m: &Mv) {
        match m {
            Mv::Remove(i) => self.b[*i as usize] = 0,
            Mv::Jump(f, d, n) => {
                let (dx, dy) = DIRS[*d as usize];
                let c = self.b[*f as usize];
                let (mut x, mut y) = ((*f as usize % N) as i32, (*f as usize / N) as i32);
                self.b[*f as usize] = 0;
                for _ in 0..*n {
                    self.b[((y + dy) as usize) * N + (x + dx) as usize] = 0;
                    x += 2 * dx;
                    y += 2 * dy;
                }
                self.b[y as usize * N + x as usize] = c;
            }
        }
        self.ply += 1;
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.ply >= 2 && self.gen(self.side).is_empty() {
            Some(Outcome::Win(self.side.other()))
        } else {
            None
        }
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = self.mobility(p);
        let you = self.mobility(p.other());
        (me - you) * 10
    }

    fn render(&self) -> String {
        let mut s = String::new();
        for y in (0..N).rev() {
            s.push_str(&format!("{} ", y + 1));
            for x in 0..N {
                s.push(match self.b[y * N + x] {
                    1 => 'X',
                    2 => 'O',
                    _ => '.',
                });
                s.push(' ');
            }
            s.push('\n');
        }
        s.push_str("  a b c d e f g h\n");
        if self.ply < 2 {
            s.push_str("Deschidere: ridică o piesă proprie (remove <pătrat>).\n");
        }
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        match m {
            Mv::Remove(i) => format!("remove {}", sq(*i as usize)),
            Mv::Jump(f, d, n) => {
                let (dx, dy) = DIRS[*d as usize];
                let (x, y) = ((*f as usize % N) as i32, (*f as usize / N) as i32);
                let to =
                    ((y + dy * 2 * *n as i32) as usize) * N + (x + dx * 2 * *n as i32) as usize;
                format!("{}-{}", sq(*f as usize), sq(to))
            }
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 500,
            max_depth: 12,
        }
    }

    fn hash_key(&self) -> Option<u64> {
        let mut h = polytabletop_core::hash_mix(
            7 << 40 | (self.side.idx() as u64) << 32 | self.ply.min(2) as u64,
        );
        for (i, &v) in self.b.iter().enumerate() {
            if v != 0 {
                h ^= polytabletop_core::hash_mix((v as u64) << 16 | i as u64);
            }
        }
        Some(h)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    fn fmt(g: &Konane) -> Vec<String> {
        g.legal_moves().iter().map(|m| g.format_move(m)).collect()
    }

    fn play(g: &mut Konane, s: &str) {
        let m = g
            .legal_moves()
            .into_iter()
            .find(|m| g.format_move(m) == s)
            .unwrap_or_else(|| panic!("ilegal {s}"));
        g.apply(&m);
    }

    #[test]
    fn opening_rules() {
        let mut g = Konane::new();
        assert_eq!(
            fmt(&g),
            ["remove d4", "remove e5", "remove a1", "remove h8"]
        );
        play(&mut g, "remove a1");
        let mut r = fmt(&g);
        r.sort();
        assert_eq!(r, ["remove a2", "remove b1"]);
        play(&mut g, "remove b1");
        // Black must now jump into a gap; only a1-gap / b1-gap landings are possible.
        let m = fmt(&g);
        assert!(m.contains(&"c1-a1".to_string()) || m.contains(&"a3-a1".to_string()));
        assert!(m.iter().all(|s| !s.starts_with("remove")));
    }

    #[test]
    fn multi_jump_and_capture() {
        let mut g = Konane::new();
        g.b = [0; 64];
        g.ply = 2;
        g.b[0] = 1; // a1 black
        g.b[1] = 2; // b1 white
        g.b[3] = 2; // d1 white
        let m = fmt(&g);
        assert!(m.contains(&"a1-c1".to_string()));
        assert!(m.contains(&"a1-e1".to_string()));
        play(&mut g, "a1-e1");
        assert_eq!(g.b[1], 0);
        assert_eq!(g.b[3], 0);
        assert_eq!(g.b[4], 1);
        // White has no pieces left on board: White to move loses.
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn cannot_jump_own_or_straddle_gap() {
        let mut g = Konane::new();
        g.b = [0; 64];
        g.ply = 2;
        g.b[0] = 1;
        g.b[1] = 1; // own piece adjacent
        g.b[2] = 0;
        g.b[8] = 2; // a2 white, landing a3 empty
        g.b[24] = 2; // a4 white: after landing a3, a4 white but a5 empty -> double
        let m = fmt(&g);
        assert!(m.contains(&"a1-a3".to_string()));
        assert!(m.contains(&"a1-a5".to_string()));
        assert!(!m.iter().any(|s| s.starts_with("a1-c1")));
        // Landing square occupied: no jump.
        g.b[16] = 2;
        let m = fmt(&g);
        assert!(!m.contains(&"a1-a3".to_string()));
    }

    #[test]
    fn random_games_terminate() {
        for seed in 1..=20u64 {
            let mut g = Konane::new();
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 200);
            }
        }
    }
}
