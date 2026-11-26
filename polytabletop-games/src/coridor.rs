//! Coridor: cursă de pioni pe o tablă 9×9 cu ziduri. Reguli publice, implementare
//! originală.
//!
//! * J1 (X) pornește din e1 și trebuie să ajungă pe rândul 9; J2 (O) pornește din
//!   e9 și trebuie să ajungă pe rândul 1.
//! * Pe rând: muți pionul un pas ortogonal (liber de ziduri) SAU pui un zid (10
//!   ziduri de jucător), lung cât două pătrate.
//! * Pionul adversar vecin se sare în linie dreaptă; dacă în spatele lui e zid,
//!   margine sau nimic, se poate sări în diagonală (pe lângă el).
//! * Zidurile nu se pot suprapune, încrucișa în cruce sau lăsa un jucător fără
//!   nicio cale către rândul său țintă.
//! * Remiză la 200 de semi-mutări (protecție împotriva buclelor).
//!
//! Mutări: `e2` (pion), `d4h` (zid orizontal peste d4–e4, între rândurile 4 și 5),
//! `d4v` (zid vertical între coloanele d și e, rândurile 4–5).

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: usize = 9;
const WALLS: u8 = 10;
const PLY_CAP: u16 = 200;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mv {
    Step(u8, u8),
    WallH(u8, u8),
    WallV(u8, u8),
}

#[derive(Clone)]
pub struct Coridor {
    pawn: [(i8, i8); 2],
    left: [u8; 2],
    /// h[x][y]: wall above squares (x,y),(x+1,y)
    h: [[bool; 8]; 8],
    /// v[x][y]: wall right of squares (x,y),(x,y+1)
    v: [[bool; 8]; 8],
    side: Player,
    ply: u16,
}

impl Default for Coridor {
    fn default() -> Self {
        Self::new()
    }
}

fn sqn(x: i32, y: i32) -> String {
    format!("{}{}", (b'a' + x as u8) as char, y + 1)
}

impl Coridor {
    pub fn new() -> Self {
        Coridor {
            pawn: [(4, 0), (4, 8)],
            left: [WALLS; 2],
            h: [[false; 8]; 8],
            v: [[false; 8]; 8],
            side: Player::One,
            ply: 0,
        }
    }

    /// Is the step between (x,y) and (x+dx,y+dy) (orthogonal, on board) free of walls?
    fn open(&self, x: i32, y: i32, dx: i32, dy: i32) -> bool {
        let (nx, ny) = (x + dx, y + dy);
        if nx < 0 || ny < 0 || nx >= N as i32 || ny >= N as i32 {
            return false;
        }
        let hw = |a: i32, b: i32| {
            (0..8).contains(&a) && (0..8).contains(&b) && self.h[a as usize][b as usize]
        };
        let vw = |a: i32, b: i32| {
            (0..8).contains(&a) && (0..8).contains(&b) && self.v[a as usize][b as usize]
        };
        if dy != 0 {
            let lo = y.min(ny);
            !(hw(x, lo) || hw(x - 1, lo))
        } else {
            let lo = x.min(nx);
            !(vw(lo, y) || vw(lo, y - 1))
        }
    }

    fn goal_row(p: usize) -> i32 {
        if p == 0 {
            8
        } else {
            0
        }
    }

    /// Shortest path length (in steps, ignoring pawns) from `from` to the goal row.
    fn dist(&self, p: usize) -> Option<i32> {
        let g = Self::goal_row(p);
        let mut d = [[-1i32; N]; N];
        let mut q = [(0i8, 0i8); N * N];
        let (mut head, mut tail) = (0, 0);
        let s = self.pawn[p];
        d[s.0 as usize][s.1 as usize] = 0;
        q[tail] = s;
        tail += 1;
        while head < tail {
            let (x, y) = q[head];
            head += 1;
            let cd = d[x as usize][y as usize];
            if y as i32 == g {
                return Some(cd);
            }
            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                if self.open(x as i32, y as i32, dx, dy) {
                    let (nx, ny) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
                    if d[nx][ny] < 0 {
                        d[nx][ny] = cd + 1;
                        q[tail] = (nx as i8, ny as i8);
                        tail += 1;
                    }
                }
            }
        }
        None
    }

    fn pawn_steps(&self, out: &mut Vec<Mv>) {
        let me = self.pawn[self.side.idx()];
        let op = self.pawn[self.side.other().idx()];
        let (x, y) = (me.0 as i32, me.1 as i32);
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if !self.open(x, y, dx, dy) {
                continue;
            }
            let (nx, ny) = (x + dx, y + dy);
            if (nx, ny) != (op.0 as i32, op.1 as i32) {
                out.push(Mv::Step(nx as u8, ny as u8));
                continue;
            }
            // Adjacent opponent: straight jump, else diagonals.
            if self.open(nx, ny, dx, dy) {
                out.push(Mv::Step((nx + dx) as u8, (ny + dy) as u8));
            } else {
                let sides: [(i32, i32); 2] = if dx == 0 {
                    [(1, 0), (-1, 0)]
                } else {
                    [(0, 1), (0, -1)]
                };
                for (sx, sy) in sides {
                    if self.open(nx, ny, sx, sy) {
                        out.push(Mv::Step((nx + sx) as u8, (ny + sy) as u8));
                    }
                }
            }
        }
    }

    fn wall_ok(&mut self, m: Mv) -> bool {
        match m {
            Mv::WallH(x, y) => {
                let (xi, yi) = (x as usize, y as usize);
                if self.h[xi][yi]
                    || self.v[xi][yi]
                    || (xi > 0 && self.h[xi - 1][yi])
                    || (xi < 7 && self.h[xi + 1][yi])
                {
                    return false;
                }
                self.h[xi][yi] = true;
                let ok = self.dist(0).is_some() && self.dist(1).is_some();
                self.h[xi][yi] = false;
                ok
            }
            Mv::WallV(x, y) => {
                let (xi, yi) = (x as usize, y as usize);
                if self.v[xi][yi]
                    || self.h[xi][yi]
                    || (yi > 0 && self.v[xi][yi - 1])
                    || (yi < 7 && self.v[xi][yi + 1])
                {
                    return false;
                }
                self.v[xi][yi] = true;
                let ok = self.dist(0).is_some() && self.dist(1).is_some();
                self.v[xi][yi] = false;
                ok
            }
            Mv::Step(..) => true,
        }
    }
}

impl Game for Coridor {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        let mut v = Vec::new();
        self.pawn_steps(&mut v);
        if self.left[self.side.idx()] > 0 {
            let mut c = self.clone();
            for x in 0..8u8 {
                for y in 0..8u8 {
                    for m in [Mv::WallH(x, y), Mv::WallV(x, y)] {
                        if c.wall_ok(m) {
                            v.push(m);
                        }
                    }
                }
            }
        }
        v
    }

    fn apply(&mut self, m: &Mv) {
        let i = self.side.idx();
        match *m {
            Mv::Step(x, y) => self.pawn[i] = (x as i8, y as i8),
            Mv::WallH(x, y) => {
                self.h[x as usize][y as usize] = true;
                self.left[i] -= 1;
            }
            Mv::WallV(x, y) => {
                self.v[x as usize][y as usize] = true;
                self.left[i] -= 1;
            }
        }
        self.ply += 1;
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.pawn[0].1 == 8 {
            Some(Outcome::Win(Player::One))
        } else if self.pawn[1].1 == 0 {
            Some(Outcome::Win(Player::Two))
        } else if self.ply >= PLY_CAP {
            Some(Outcome::Draw)
        } else {
            None
        }
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let a = self.dist(me).unwrap_or(99);
        let b = self.dist(1 - me).unwrap_or(99);
        (b - a) * 100 + (self.left[me] as i32 - self.left[1 - me] as i32) * 12
    }

    fn render(&self) -> String {
        let mut s = String::new();
        for y in (0..N as i32).rev() {
            s.push_str(&format!("{} ", y + 1));
            for x in 0..N as i32 {
                let c = if self.pawn[0] == (x as i8, y as i8) {
                    'X'
                } else if self.pawn[1] == (x as i8, y as i8) {
                    'O'
                } else {
                    '.'
                };
                s.push(c);
                if x < 8 {
                    let w = (y < 8 && self.v[x as usize][y as usize])
                        || (y > 0 && self.v[x as usize][(y - 1) as usize]);
                    s.push(if w { '|' } else { ' ' });
                }
            }
            s.push('\n');
            if y > 0 {
                s.push_str("  ");
                for x in 0..N as i32 {
                    let w = (x < 8 && self.h[x as usize][(y - 1) as usize])
                        || (x > 0 && self.h[(x - 1) as usize][(y - 1) as usize]);
                    s.push(if w { '-' } else { ' ' });
                    s.push(' ');
                }
                s.push('\n');
            }
        }
        s.push_str("  a b c d e f g h i\n");
        s.push_str(&format!(
            "Ziduri rămase: X {}, O {}\n",
            self.left[0], self.left[1]
        ));
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        match *m {
            Mv::Step(x, y) => sqn(x as i32, y as i32),
            Mv::WallH(x, y) => format!("{}h", sqn(x as i32, y as i32)),
            Mv::WallV(x, y) => format!("{}v", sqn(x as i32, y as i32)),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 600,
            max_depth: 6,
        }
    }

    fn hash_key(&self) -> Option<u64> {
        let mut k = polytabletop_core::hash_mix(
            (self.side.idx() as u64) << 40 | (self.ply.min(PLY_CAP) as u64) << 20,
        );
        for i in 0..2 {
            k ^= polytabletop_core::hash_mix(
                1 << 50 | (i as u64) << 24 | (self.pawn[i].0 as u64) << 8 | self.pawn[i].1 as u64,
            );
            k ^= polytabletop_core::hash_mix(2 << 50 | (i as u64) << 8 | self.left[i] as u64);
        }
        for x in 0..8 {
            for y in 0..8 {
                if self.h[x][y] {
                    k ^= polytabletop_core::hash_mix(3 << 50 | (x * 8 + y) as u64);
                }
                if self.v[x][y] {
                    k ^= polytabletop_core::hash_mix(4 << 50 | (x * 8 + y) as u64);
                }
            }
        }
        Some(k)
    }

    fn move_score(&self, m: &Mv) -> i32 {
        match m {
            Mv::Step(_, y) => {
                let g = Self::goal_row(self.side.idx());
                10 - (g - *y as i32).abs()
            }
            _ => 0,
        }
    }

    fn playout_cap(&self) -> usize {
        PLY_CAP as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    fn mv(g: &Coridor, s: &str) -> Mv {
        g.legal_moves()
            .into_iter()
            .find(|m| g.format_move(m) == s)
            .unwrap_or_else(|| panic!("ilegal {s}"))
    }
    fn has(g: &Coridor, s: &str) -> bool {
        g.legal_moves().iter().any(|m| g.format_move(m) == s)
    }

    #[test]
    fn start_moves() {
        let g = Coridor::new();
        // 3 pawn steps + 128 walls.
        assert_eq!(g.legal_moves().len(), 3 + 128);
        assert_eq!(g.dist(0), Some(8));
    }

    #[test]
    fn wall_blocks_step_and_no_overlap() {
        let mut g = Coridor::new();
        g.apply(&mv(&g, "e1h")); // J1 wall above e1,f1 ... wait: anchored at e1
        assert!(!g.open(4, 0, 0, 1));
        assert!(!g.open(5, 0, 0, 1));
        assert!(g.open(3, 0, 0, 1));
        // J2: overlapping or crossing walls are illegal.
        assert!(!has(&g, "e1h"));
        assert!(!has(&g, "d1h"));
        assert!(!has(&g, "f1h"));
        assert!(!has(&g, "e1v"));
        assert!(has(&g, "g1h"));
        assert!(has(&g, "e2v"));
    }

    #[test]
    fn jump_straight_and_diagonal() {
        let mut g = Coridor::new();
        g.pawn = [(4, 3), (4, 4)];
        assert!(has(&g, "e6"));
        assert!(!has(&g, "e5"));
        g.h[4][4] = true; // wall above e5 (between rows 5 and 6) blocks straight jump
        assert!(!has(&g, "e6"));
        assert!(has(&g, "d5"));
        assert!(has(&g, "f5"));
    }

    #[test]
    fn cannot_wall_in_a_player() {
        let mut g = Coridor::new();
        g.pawn = [(0, 0), (8, 8)];
        for x in [0usize, 2, 4, 7] {
            g.h[x][0] = true;
        }
        // Only gap on row 1 is g1-g2; a vertical wall right of f1 would seal J1 in.
        assert!(g.dist(0).is_some());
        assert!(!has(&g, "f1v"));
        // Walls elsewhere that keep a path are fine.
        assert!(has(&g, "a5h"));
    }

    #[test]
    fn reaching_goal_wins() {
        let mut g = Coridor::new();
        g.pawn = [(2, 7), (6, 5)];
        let m = mv(&g, "c9");
        g.apply(&m);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
        let mut g = Coridor::new();
        g.side = Player::Two;
        g.pawn = [(2, 5), (6, 1)];
        let m = mv(&g, "g1");
        g.apply(&m);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
    }

    #[test]
    fn ten_walls_max() {
        let mut g = Coridor::new();
        g.left[0] = 0;
        assert_eq!(g.legal_moves().len(), 3);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=6u64 {
            let mut g = Coridor::new();
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n <= 200);
                assert!(g.dist(0).is_some() && g.dist(1).is_some());
            }
        }
    }
}
