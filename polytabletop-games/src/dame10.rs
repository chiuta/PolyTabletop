//! Dame internaționale (10×10), 2 jucători, din regulile publice (FMJD).
//!
//! * Tablă 10×10, 20 de piese pe jucător pe pătratele întunecate (a1 e întunecat); Jucătorul 1 (jos)
//!   începe. Piesele simple mută un pas în diagonală înainte; **capturează înainte și înapoi**.
//! * Damele (piesele promovate) „zboară”: mută și capturează pe orice distanță în diagonală.
//! * Captura e obligatorie și trebuie aleasă **secvența cu cele mai multe piese**; piesele
//!   capturate se scot la sfârșitul secvenței și nu pot fi sărite de două ori.
//! * O piesă simplă devine damă doar dacă își **încheie** mutarea pe ultimul rând.
//! * Remiză: 50 de plie consecutive doar cu dame fără captură sau 400 de plie în total
//!   (simplificare declarată; regulile FMJD au și repetiția).
//!
//! Mutări: `c4-d5`, captură `c4xe6xg8` (pătratele de oprire).

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: usize = 10;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Mv {
    pub path: Vec<u8>,
    pub capture: bool,
}

#[derive(Clone)]
pub struct Dame10 {
    /// 0 gol, 1/2 = om/damă J1, 3/4 = om/damă J2.
    b: [u8; 100],
    side: Player,
    quiet: u16,
    plies: u16,
}

impl Default for Dame10 {
    fn default() -> Self {
        Self::new()
    }
}

fn owner(c: u8) -> Option<Player> {
    match c {
        1 | 2 => Some(Player::One),
        3 | 4 => Some(Player::Two),
        _ => None,
    }
}

fn sq(i: u8) -> String {
    format!("{}{}", (b'a' + i % 10) as char, i / 10 + 1)
}

const DIRS: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

fn step(i: usize, d: (i32, i32)) -> Option<usize> {
    let (r, f) = ((i / N) as i32 + d.0, (i % N) as i32 + d.1);
    if (0..N as i32).contains(&r) && (0..N as i32).contains(&f) {
        Some(r as usize * N + f as usize)
    } else {
        None
    }
}

impl Dame10 {
    pub fn new() -> Self {
        let mut b = [0u8; 100];
        for r in 0..N {
            for f in 0..N {
                if (f + r) % 2 == 0 {
                    if r < 4 {
                        b[r * N + f] = 1;
                    } else if r > 5 {
                        b[r * N + f] = 3;
                    }
                }
            }
        }
        Dame10 {
            b,
            side: Player::One,
            quiet: 0,
            plies: 0,
        }
    }

    pub fn counts(&self) -> (usize, usize) {
        let c = |p| self.b.iter().filter(|&&x| owner(x) == Some(p)).count();
        (c(Player::One), c(Player::Two))
    }

    #[allow(clippy::too_many_arguments)]
    fn jumps(
        &self,
        pos: usize,
        piece: u8,
        me: Player,
        cap: &mut [bool; 100],
        path: &mut Vec<u8>,
        out: &mut Vec<(Vec<u8>, usize)>,
    ) {
        let king = piece == 2 || piece == 4;
        let mut any = false;
        for d in DIRS {
            let mut cur = pos;
            // zboară până la prima piesă
            let mut victim = None;
            while let Some(n) = step(cur, d) {
                // originea e liberă (piesa a plecat); piesele deja capturate blochează
                if self.b[n] == 0 && !cap[n] || n == path[0] as usize {
                    if king {
                        cur = n;
                        continue;
                    }
                    break;
                }
                if cap[n] {
                    break;
                }
                if owner(self.b[n]) == Some(me) {
                    break;
                }
                victim = Some(n);
                break;
            }
            let Some(v) = victim else { continue };
            // atinsă o piesă adversă; ne oprim după ea
            if !king && cur != pos {
                continue;
            }
            let mut land = step(v, d);
            let mut landings = Vec::new();
            while let Some(l) = land {
                if (self.b[l] == 0 && !cap[l]) || l == path[0] as usize {
                    landings.push(l);
                    if !king {
                        break;
                    }
                    land = step(l, d);
                } else {
                    break;
                }
            }
            for l in landings {
                any = true;
                cap[v] = true;
                path.push(l as u8);
                self.jumps(l, piece, me, cap, path, out);
                path.pop();
                cap[v] = false;
            }
        }
        if !any && path.len() > 1 {
            let n = cap.iter().filter(|&&x| x).count();
            out.push((path.clone(), n));
        }
    }

    fn gen(&self) -> Vec<Mv> {
        let me = self.side;
        let mut caps: Vec<(Vec<u8>, usize)> = Vec::new();
        for i in 0..100 {
            let c = self.b[i];
            if owner(c) != Some(me) {
                continue;
            }
            let mut cap = [false; 100];
            let mut path = vec![i as u8];
            self.jumps(i, c, me, &mut cap, &mut path, &mut caps);
        }
        if !caps.is_empty() {
            let best = caps.iter().map(|c| c.1).max().unwrap();
            let mut v: Vec<Mv> = Vec::new();
            for (p, n) in caps {
                if n == best && !v.iter().any(|m| m.path == p) {
                    v.push(Mv {
                        path: p,
                        capture: true,
                    });
                }
            }
            return v;
        }
        let mut v = Vec::new();
        let fwd = if me == Player::One { 1 } else { -1 };
        for i in 0..100 {
            let c = self.b[i];
            if owner(c) != Some(me) {
                continue;
            }
            let king = c == 2 || c == 4;
            for d in DIRS {
                if !king && d.0 != fwd {
                    continue;
                }
                let mut cur = i;
                while let Some(n) = step(cur, d) {
                    if self.b[n] != 0 {
                        break;
                    }
                    v.push(Mv {
                        path: vec![i as u8, n as u8],
                        capture: false,
                    });
                    if !king {
                        break;
                    }
                    cur = n;
                }
            }
        }
        v
    }
}

impl Game for Dame10 {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        self.gen()
    }

    fn apply(&mut self, m: &Mv) {
        let from = m.path[0] as usize;
        let to = *m.path.last().unwrap() as usize;
        let piece = self.b[from];
        let was_king = piece == 2 || piece == 4;
        self.b[from] = 0;
        if m.capture {
            for w in m.path.windows(2) {
                let (a, z) = (w[0] as usize, w[1] as usize);
                let (dr, df) = (
                    ((z / N) as i32 - (a / N) as i32).signum(),
                    ((z % N) as i32 - (a % N) as i32).signum(),
                );
                let mut cur = a;
                while let Some(n) = step(cur, (dr, df)) {
                    if n == z {
                        break;
                    }
                    if self.b[n] != 0 {
                        self.b[n] = 0;
                    }
                    cur = n;
                }
            }
        }
        let last_row = if self.side == Player::One { N - 1 } else { 0 };
        self.b[to] = if !was_king && to / N == last_row {
            piece + 1
        } else {
            piece
        };
        if m.capture || !was_king {
            self.quiet = 0;
        } else {
            self.quiet += 1;
        }
        self.plies += 1;
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.quiet >= 50 || self.plies >= 400 {
            return Some(Outcome::Draw);
        }
        if self.gen().is_empty() {
            return Some(Outcome::Win(self.side.other()));
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let mut s = 0;
        for i in 0..100 {
            let c = self.b[i];
            let Some(o) = owner(c) else { continue };
            let king = c == 2 || c == 4;
            let r = (i / N) as i32;
            let adv = if o == Player::One { r } else { 9 - r };
            let (rr, ff) = (r - 5, (i % N) as i32 - 5);
            let center = 4 - ((2 * rr + 1).abs().max((2 * ff + 1).abs()) / 2);
            let v = if king { 330 } else { 100 + adv * 4 } + center * 3;
            s += if o == p { v } else { -v };
        }
        s
    }

    fn render(&self) -> String {
        let mut s = String::from("Dame internaționale 10×10 (x/X = J1 om/damă, o/O = J2)\n\n");
        for r in (0..N).rev() {
            s.push_str(&format!("{:>2} ", r + 1));
            for f in 0..N {
                let ch = match self.b[r * N + f] {
                    1 => 'x',
                    2 => 'X',
                    3 => 'o',
                    4 => 'O',
                    _ if (f + r) % 2 == 0 => '.',
                    _ => ' ',
                };
                s.push(ch);
                s.push(' ');
            }
            s.push('\n');
        }
        s.push_str("   a b c d e f g h i j\n");
        let (a, b) = self.counts();
        s.push_str(&format!("Piese: J1 {a}, J2 {b}. "));
        match self.outcome() {
            Some(Outcome::Win(p)) => s.push_str(&format!("Câștigă J{}.\n", p.idx() + 1)),
            Some(Outcome::Draw) => s.push_str("Remiză.\n"),
            None => s.push_str(&format!("La mutare: J{}\n", self.side.idx() + 1)),
        }
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        let sep = if m.capture { 'x' } else { '-' };
        m.path
            .iter()
            .map(|&i| sq(i))
            .collect::<Vec<_>>()
            .join(&sep.to_string())
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 400,
            max_depth: 9,
        }
    }

    fn move_score(&self, m: &Mv) -> i32 {
        if m.capture {
            100 + m.path.len() as i32
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    fn empty() -> Dame10 {
        Dame10 {
            b: [0; 100],
            side: Player::One,
            quiet: 0,
            plies: 0,
        }
    }

    #[test]
    fn start_has_twenty_each_and_nine_moves() {
        let g = Dame10::new();
        assert_eq!(g.counts(), (20, 20));
        assert_eq!(g.legal_moves().len(), 9);
    }

    #[test]
    fn men_capture_backwards_and_max_capture_is_forced() {
        let mut g = empty();
        g.b[3 * 10 + 3] = 1; // d4
        g.b[4 * 10 + 4] = 3; // e5
        g.b[2 * 10 + 2] = 3; // c3 (înapoi)
        g.b[11] = 0;
        g.b[7 * 10 + 7] = 3;
        let m = g.legal_moves();
        assert!(m.iter().all(|x| x.capture));
        assert_eq!(m.len(), 2);
        // lanț de 2: d4 x f6 x ... (adăugăm victimă)
        g.b[6 * 10 + 6] = 0;
        g.b[5 * 10 + 5] = 0;
        g.b[6 * 10 + 4] = 3; // e7
        let m = g.legal_moves();
        assert!(m.iter().all(|x| x.path.len() == 3), "{m:?}");
    }

    #[test]
    fn flying_king_and_promotion_at_end_only() {
        let mut g = empty();
        g.b[0] = 2;
        g.b[5 * 10 + 5] = 3;
        let m = g.legal_moves();
        // dama de la a1 sare f6 și aterizează pe g7..j10
        assert_eq!(m.len(), 4);
        let mut h = empty();
        h.b[8 * 10 + 8] = 1; // i9
        h.apply(&Mv {
            path: vec![88, 99],
            capture: false,
        });
        assert_eq!(h.b[99], 2);
    }

    #[test]
    fn random_games_terminate() {
        for seed in 1..=4u64 {
            let mut g: Box<dyn polytabletop_core::GameObj> = Box::new(Dame10::new());
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.play(&m[rng.below(m.len())]).unwrap();
                n += 1;
                assert!(n <= 400);
            }
        }
    }

    #[test]
    fn ai_plays_legally() {
        polytabletop_core::ai::set_time_budget_override_ms(Some(20));
        let mut g: Box<dyn polytabletop_core::GameObj> = Box::new(Dame10::new());
        for _ in 0..20 {
            let m = g.ai_move(3).unwrap();
            g.play(&m).unwrap();
        }
    }
}
