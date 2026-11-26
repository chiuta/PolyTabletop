//! Vulpea și gâștele (Fox and Geese) — tabla cu 33 de găuri în cruce, o vulpe contra 13 gâște.
//!
//! * Jucătorul 1 = vulpea (începe în centru, d4), Jucătorul 2 = gâștele (13, pe rândurile 1–3).
//! * Vulpea mută un pas ortogonal sau sare peste o gâscă alăturată pe un loc liber (captură);
//!   sărituri înlănțuite se scriu `d4xd6xf6`. Captura nu e obligatorie.
//! * Gâștele mută un pas înainte sau lateral (niciodată înapoi), fără salturi.
//! * Gâștele câștigă dacă vulpea nu mai poate muta; vulpea câștigă dacă rămân mai puțin de 6
//!   gâște (prea puține ca s-o încercuiască — prag ales aici) sau dacă gâștele nu mai pot muta.
//!   Remiză după 300 de mutări.
//!
//! Sursa regulilor: tradiție publică (varianta clasică, 13 gâște, fără diagonale).

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const W: i32 = 7;
const MIN_GEESE: u8 = 6;
const PLY_CAP: u16 = 300;

fn valid(x: i32, y: i32) -> bool {
    (0..W).contains(&x) && (0..W).contains(&y) && ((2..=4).contains(&x) || (2..=4).contains(&y))
}
fn idx(x: i32, y: i32) -> usize {
    (y * W + x) as usize
}
/// `y` crește în jos pe ecran; rangul afișat = 7 − y.
fn name(i: usize) -> String {
    let (x, y) = (i as i32 % W, i as i32 / W);
    format!("{}{}", (b'a' + x as u8) as char, 7 - y)
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Mv {
    Fox(Vec<u8>),
    Goose(u8, u8),
}

#[derive(Clone)]
pub struct FoxGeese {
    b: [u8; 49], // 0 gol, 1 vulpe, 2 gâscă
    fox: usize,
    geese: u8,
    side: Player,
    ply: u16,
}

impl Default for FoxGeese {
    fn default() -> Self {
        Self::new()
    }
}

impl FoxGeese {
    pub fn new() -> Self {
        let mut b = [0u8; 49];
        for y in 4..7 {
            for x in 0..W {
                if valid(x, y) {
                    b[idx(x, y)] = 2;
                }
            }
        }
        let fox = idx(3, 3);
        b[fox] = 1;
        FoxGeese {
            b,
            fox,
            geese: 13,
            side: Player::One,
            ply: 0,
        }
    }

    pub fn geese(&self) -> u8 {
        self.geese
    }

    fn jumps_from(&self, pos: usize, board: &[u8; 49], out: &mut Vec<Vec<u8>>, path: &mut Vec<u8>) {
        let (x, y) = (pos as i32 % W, pos as i32 / W);
        let mut any = false;
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let (mx, my, tx, ty) = (x + dx, y + dy, x + 2 * dx, y + 2 * dy);
            if valid(tx, ty) && board[idx(mx, my)] == 2 && board[idx(tx, ty)] == 0 {
                any = true;
                let mut nb = *board;
                nb[idx(mx, my)] = 0;
                path.push(idx(tx, ty) as u8);
                out.push(path.clone());
                self.jumps_from(idx(tx, ty), &nb, out, path);
                path.pop();
            }
        }
        let _ = any;
    }

    fn fox_moves(&self) -> Vec<Mv> {
        let mut v = Vec::new();
        let (x, y) = (self.fox as i32 % W, self.fox as i32 / W);
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            if valid(x + dx, y + dy) && self.b[idx(x + dx, y + dy)] == 0 {
                v.push(Mv::Fox(vec![self.fox as u8, idx(x + dx, y + dy) as u8]));
            }
        }
        let mut board = self.b;
        board[self.fox] = 0;
        let mut out = Vec::new();
        let mut path = vec![self.fox as u8];
        self.jumps_from(self.fox, &board, &mut out, &mut path);
        v.extend(out.into_iter().map(Mv::Fox));
        v
    }

    fn goose_moves(&self) -> Vec<Mv> {
        let mut v = Vec::new();
        for i in 0..49 {
            if self.b[i] != 2 {
                continue;
            }
            let (x, y) = (i as i32 % W, i as i32 / W);
            for (dx, dy) in [(0, -1), (-1, 0), (1, 0)] {
                if valid(x + dx, y + dy) && self.b[idx(x + dx, y + dy)] == 0 {
                    v.push(Mv::Goose(i as u8, idx(x + dx, y + dy) as u8));
                }
            }
        }
        v
    }

    fn fox_mobility(&self) -> i32 {
        self.fox_moves().len() as i32
    }
}

impl Game for FoxGeese {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        if self.side == Player::One {
            self.fox_moves()
        } else {
            self.goose_moves()
        }
    }

    fn apply(&mut self, m: &Mv) {
        match m {
            Mv::Fox(path) => {
                self.b[self.fox] = 0;
                for w in path.windows(2) {
                    let (a, c) = (w[0] as i32, w[1] as i32);
                    if (a % W - c % W).abs() + (a / W - c / W).abs() == 2 {
                        let mid = ((a % W + c % W) / 2, (a / W + c / W) / 2);
                        self.b[idx(mid.0, mid.1)] = 0;
                        self.geese -= 1;
                    }
                }
                self.fox = *path.last().unwrap() as usize;
                self.b[self.fox] = 1;
            }
            Mv::Goose(a, c) => {
                self.b[*a as usize] = 0;
                self.b[*c as usize] = 2;
            }
        }
        self.side = self.side.other();
        self.ply += 1;
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.geese < MIN_GEESE {
            return Some(Outcome::Win(Player::One));
        }
        if self.side == Player::One && self.fox_moves().is_empty() {
            return Some(Outcome::Win(Player::Two));
        }
        if self.side == Player::Two && self.goose_moves().is_empty() {
            return Some(Outcome::Win(Player::One));
        }
        if self.ply >= PLY_CAP {
            return Some(Outcome::Draw);
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        // din punctul de vedere al vulpii: gâște mâncate, mobilitate, apropiere de gâște
        let fox_view = 300 * (13 - self.geese as i32) + 12 * self.fox_mobility();
        // gâștele avansate sunt bune pentru gâște
        let mut adv = 0;
        for i in 0..49 {
            if self.b[i] == 2 {
                adv += 6 - (i as i32 / W);
            }
        }
        let v = fox_view - 4 * adv;
        if p == Player::One {
            v
        } else {
            -v
        }
    }

    fn render(&self) -> String {
        let mut s = String::from("   a b c d e f g\n");
        for y in 0..W {
            s.push_str(&format!("{}  ", 7 - y));
            for x in 0..W {
                s.push(if !valid(x, y) {
                    ' '
                } else {
                    match self.b[idx(x, y)] {
                        1 => 'V',
                        2 => 'g',
                        _ => '.',
                    }
                });
                s.push(' ');
            }
            s.push('\n');
        }
        s.push_str(&format!(
            "V = vulpea (J1), g = gâște (J2), gâște rămase: {}. La mutare: Jucătorul {}.",
            self.geese,
            self.side.idx() + 1
        ));
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        match m {
            Mv::Fox(path) => {
                let mut t = name(path[0] as usize);
                for w in path.windows(2) {
                    let (a, c) = (w[0] as i32, w[1] as i32);
                    let jump = (a % W - c % W).abs() + (a / W - c / W).abs() == 2;
                    t.push(if jump { 'x' } else { '-' });
                    t.push_str(&name(w[1] as usize));
                }
                t
            }
            Mv::Goose(a, c) => format!("{}-{}", name(*a as usize), name(*c as usize)),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 300,
            max_depth: 8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q4_util::*;
    use polytabletop_core::{GameObj, Player};

    #[test]
    fn setup_and_first_moves() {
        let g = FoxGeese::new();
        assert_eq!(g.geese(), 13);
        let m = GameObj::legal_moves(&g);
        // vulpea din d4 se poate muta doar pe d5, c4, e4 (d3 e ocupat de gâscă? nu: rândul 3 e gol)
        assert!(m.contains(&"d4-d5".to_string()), "{m:?}");
    }

    #[test]
    fn fox_captures_by_jumping() {
        let mut g = FoxGeese::new();
        g.b = [0; 49];
        g.b[idx(3, 3)] = 1;
        g.b[idx(3, 4)] = 2;
        g.b[idx(3, 6)] = 2;
        g.fox = idx(3, 3);
        g.geese = 13;
        let m = GameObj::legal_moves(&g);
        assert!(m.contains(&"d4xd2".to_string()), "{m:?}");
    }

    #[test]
    fn random_and_ai_games_finish() {
        random_finishes(|_| Box::new(FoxGeese::new()), 6, 700);
        ai_vs_random(|_| Box::new(FoxGeese::new()), 2, Player::One, 700);
        ai_vs_random(|_| Box::new(FoxGeese::new()), 2, Player::Two, 700);
    }
}
