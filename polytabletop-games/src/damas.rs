//! Dame braziliene (Damas), 8×8 cu regulile dameï internaționale: piesele simple
//! mută înainte și capturează în toate cele patru direcții, damele „zboară”,
//! captura este obligatorie și trebuie să fie **cea mai mare** (regula majorității),
//! piesele capturate se scot abia după întreaga mutare, iar o piesă care doar trece
//! pe ultimul rând (fără să se oprească acolo) nu devine damă.
//! Remiză după 40 de semi-mutări (20 pe parte) fără captură și fără mutare de piesă simplă.
//!
//! Mutări: `c3-d4`; captură cu pătratele de aterizare: `c3xe5xg7`.

use crate::q5_util::{sq_name, verdict};
use polytabletop_core::{AiStyle, Game, Outcome, Player};

// 0 gol, 1 piesă J1, 2 damă J1, 3 piesă J2, 4 damă J2.
const DIRS: [(i32, i32); 4] = [(1, -1), (1, 1), (-1, -1), (-1, 1)];

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Mv {
    pub path: Vec<u8>,
    pub cap: bool,
}

#[derive(Clone)]
pub struct Damas {
    b: [u8; 64],
    side: Player,
    quiet: u16,
}

impl Default for Damas {
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

fn is_king(c: u8) -> bool {
    c == 2 || c == 4
}

fn inside(r: i32, f: i32) -> bool {
    (0..8).contains(&r) && (0..8).contains(&f)
}

impl Damas {
    pub fn new() -> Self {
        let mut b = [0u8; 64];
        for r in 0..8usize {
            for f in 0..8usize {
                if (f + r) % 2 == 0 {
                    if r < 3 {
                        b[r * 8 + f] = 1;
                    } else if r > 4 {
                        b[r * 8 + f] = 3;
                    }
                }
            }
        }
        Damas {
            b,
            side: Player::One,
            quiet: 0,
        }
    }

    #[cfg(test)]
    fn with_cells(cells: &[(usize, u8)], side: Player) -> Self {
        let mut b = [0u8; 64];
        for &(i, c) in cells {
            b[i] = c;
        }
        Damas { b, side, quiet: 0 }
    }

    pub fn piece_count(&self, p: Player) -> usize {
        self.b.iter().filter(|&&c| owner(c) == Some(p)).count()
    }

    #[allow(clippy::too_many_arguments)]
    fn grab(
        &self,
        b: &[u8; 64],
        at: usize,
        piece: u8,
        me: Player,
        taken: &mut Vec<usize>,
        path: &mut Vec<u8>,
        out: &mut Vec<(Vec<u8>, usize)>,
    ) {
        let mut found = false;
        let (r0, f0) = ((at / 8) as i32, (at % 8) as i32);
        for &(dr, df) in &DIRS {
            // primul pătrat ocupat pe direcție (damă) sau cel alăturat (piesă simplă)
            let (mut r, mut f) = (r0 + dr, f0 + df);
            if is_king(piece) {
                while inside(r, f) && b[(r * 8 + f) as usize] == 0 {
                    r += dr;
                    f += df;
                }
            }
            if !inside(r, f) {
                continue;
            }
            let mid = (r * 8 + f) as usize;
            if owner(b[mid]) != Some(me.other()) || taken.contains(&mid) {
                continue;
            }
            let (mut lr, mut lf) = (r + dr, f + df);
            while inside(lr, lf) && b[(lr * 8 + lf) as usize] == 0 {
                found = true;
                let land = (lr * 8 + lf) as usize;
                taken.push(mid);
                path.push(land as u8);
                self.grab(b, land, piece, me, taken, path, out);
                path.pop();
                taken.pop();
                if !is_king(piece) {
                    break;
                }
                lr += dr;
                lf += df;
            }
        }
        if !found && path.len() > 1 {
            out.push((path.clone(), taken.len()));
        }
    }

    fn gen(&self) -> Vec<Mv> {
        let me = self.side;
        let mut caps: Vec<(Vec<u8>, usize)> = Vec::new();
        for i in 0..64 {
            let p = self.b[i];
            if owner(p) != Some(me) {
                continue;
            }
            let mut b = self.b;
            b[i] = 0;
            let mut path = vec![i as u8];
            self.grab(&b, i, p, me, &mut Vec::new(), &mut path, &mut caps);
        }
        if !caps.is_empty() {
            let best = caps.iter().map(|c| c.1).max().unwrap();
            return caps
                .into_iter()
                .filter(|c| c.1 == best)
                .map(|(path, _)| Mv { path, cap: true })
                .collect();
        }
        let mut out = Vec::new();
        for i in 0..64 {
            let p = self.b[i];
            if owner(p) != Some(me) {
                continue;
            }
            let (r0, f0) = ((i / 8) as i32, (i % 8) as i32);
            for &(dr, df) in &DIRS {
                if !is_king(p) && (dr == 1) != (me == Player::One) {
                    continue;
                }
                let (mut r, mut f) = (r0 + dr, f0 + df);
                while inside(r, f) && self.b[(r * 8 + f) as usize] == 0 {
                    out.push(Mv {
                        path: vec![i as u8, (r * 8 + f) as u8],
                        cap: false,
                    });
                    if !is_king(p) {
                        break;
                    }
                    r += dr;
                    f += df;
                }
            }
        }
        out
    }
}

fn sq(i: u8) -> String {
    sq_name(i as usize % 8, i as usize / 8)
}

impl Game for Damas {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if self.quiet >= 40 {
            return Vec::new();
        }
        self.gen()
    }

    fn apply(&mut self, m: &Mv) {
        let from = m.path[0] as usize;
        let to = *m.path.last().unwrap() as usize;
        let piece = self.b[from];
        self.b[from] = 0;
        for w in m.path.windows(2) {
            let (a, c) = (w[0] as i32, w[1] as i32);
            let (dr, df) = (((c / 8) - (a / 8)).signum(), ((c % 8) - (a % 8)).signum());
            let (mut r, mut f) = (a / 8 + dr, a % 8 + df);
            while (r * 8 + f) != c {
                let i = (r * 8 + f) as usize;
                if self.b[i] != 0 {
                    self.b[i] = 0;
                }
                r += dr;
                f += df;
            }
        }
        let last = if self.side == Player::One { 7 } else { 0 };
        let promoted = !is_king(piece) && to / 8 == last;
        self.b[to] = if promoted { piece + 1 } else { piece };
        if m.cap || !is_king(piece) {
            self.quiet = 0;
        } else {
            self.quiet += 1;
        }
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.quiet >= 40 {
            return Some(Outcome::Draw);
        }
        if self.gen().is_empty() {
            return Some(Outcome::Win(self.side.other()));
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let mut s = 0;
        for i in 0..64 {
            let c = self.b[i];
            let Some(o) = owner(c) else { continue };
            let row = (i / 8) as i32;
            let v = if is_king(c) {
                300
            } else {
                let adv = if o == Player::One { row } else { 7 - row };
                100 + adv * 4
            };
            let centre = 3 - ((i % 8) as i32 * 2 - 7).abs() / 2;
            let v = v + centre;
            if o == p {
                s += v;
            } else {
                s -= v;
            }
        }
        s
    }

    fn render(&self) -> String {
        let mut s = String::from(
            "Dame braziliene (Damas) — o/O = J1 (jos), x/X = J2 (sus); majuscula = damă\n",
        );
        s.push_str("   a b c d e f g h\n");
        for r in (0..8).rev() {
            s.push_str(&format!("{}  ", r + 1));
            for f in 0..8 {
                let c = self.b[r * 8 + f];
                s.push(match c {
                    1 => 'o',
                    2 => 'O',
                    3 => 'x',
                    4 => 'X',
                    _ if (f + r) % 2 == 0 => '.',
                    _ => ' ',
                });
                s.push(' ');
            }
            s.push_str(&format!(" {}\n", r + 1));
        }
        s.push_str("   a b c d e f g h\n");
        s.push_str(&format!(
            "Piese: J1 {} – J2 {}. Fără progres: {}/40.\n",
            self.piece_count(Player::One),
            self.piece_count(Player::Two),
            self.quiet
        ));
        let v = verdict(self.outcome());
        if v.is_empty() {
            s.push_str(&format!("La mutare: Jucător {}\n", self.side.idx() + 1));
        } else {
            s.push_str(&v);
        }
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        let sep = if m.cap { "x" } else { "-" };
        m.path.iter().map(|&i| sq(i)).collect::<Vec<_>>().join(sep)
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(5)
    }

    fn is_tactical(&self, m: &Mv) -> bool {
        m.cap
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q5_util::testkit::check_game;

    fn idx(s: &str) -> usize {
        let b = s.as_bytes();
        ((b[1] - b'1') as usize) * 8 + (b[0] - b'a') as usize
    }

    #[test]
    fn opening_has_seven_moves() {
        assert_eq!(Damas::new().legal_moves().len(), 7);
    }

    #[test]
    fn men_capture_backwards_and_majority_is_forced() {
        // piesă J1 pe c3; piese J2 pe d4 și f6 (captură dublă) și b2 (captură înapoi simplă)
        let g = Damas::with_cells(
            &[
                (idx("c3"), 1),
                (idx("d4"), 3),
                (idx("f6"), 3),
                (idx("b2"), 3),
            ],
            Player::One,
        );
        let m = g.legal_moves();
        assert_eq!(
            m.len(),
            1,
            "{:?}",
            m.iter().map(|x| g.format_move(x)).collect::<Vec<_>>()
        );
        assert_eq!(g.format_move(&m[0]), "c3xe5xg7");
    }

    #[test]
    fn flying_king_captures_from_distance() {
        let g = Damas::with_cells(&[(idx("a1"), 2), (idx("d4"), 3)], Player::One);
        let mv: Vec<String> = g.legal_moves().iter().map(|m| g.format_move(m)).collect();
        assert!(
            mv.contains(&"a1xe5".to_string()) && mv.contains(&"a1xh8".to_string()),
            "{mv:?}"
        );
    }

    #[test]
    fn promotion_on_last_row() {
        let mut g = Damas::with_cells(&[(idx("c7"), 1)], Player::One);
        let m = g
            .legal_moves()
            .into_iter()
            .find(|m| g.format_move(m) == "c7-d8")
            .unwrap();
        g.apply(&m);
        assert_eq!(g.b[idx("d8")], 2);
    }

    #[test]
    fn random_and_ai_games() {
        polytabletop_core::ai::set_time_budget_override_ms(Some(10));
        check_game(|_| Damas::new(), 4, 3000);
    }
}
