//! Camelot (SUA / Marea Britanie) — tablă 12×14 cu colțurile tăiate și câte un castel (2 pătrate)
//! la capete; 10 pioni („men”) și 4 cavaleri pe jucător.
//!
//! * Mutare simplă: un pas în oricare din 8 direcții.
//! * Canter: sari peste o piesă proprie alăturată pe pătratul liber de după ea (optional, înlănțuit).
//! * Salt (jump): sari peste o piesă adversă alăturată, pe pătratul liber de după, o capturezi;
//!   înlănțuit. Un singur tip de mișcare pe tură; **cavalerul** poate face și „încărcare”:
//!   unul sau mai multe canter-uri urmate de unul sau mai multe salturi.
//! * Captura (cel puțin un salt) e obligatorie când există.
//! * Nu intri în propriul castel decât sărind. Câștigi cu două piese în castelul adversarului,
//!   capturând toate piesele adversarului (rămânând cu cel puțin două) sau blocându-l (cu ≥2 piese);
//!   remiză dacă ambii au ≤1 piesă. Remiză și după 100 de mutări fără captură.
//!
//! Sursa: Wikipedia „Camelot (board game)” (reguli publice). Text mutare: `d6-d7`, `d6-d8` (canter),
//! `e7xe9xg11` (salturi).

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const W: i32 = 12;
const H: i32 = 16;
const N: usize = (W * H) as usize;

fn idx(x: i32, y: i32) -> usize {
    (y * W + x) as usize
}
fn valid(x: i32, y: i32) -> bool {
    if !(0..W).contains(&x) || !(0..H).contains(&y) {
        return false;
    }
    if y == 0 || y == H - 1 {
        return x == 5 || x == 6;
    }
    // colțuri tăiate (3 pătrate)
    let cuts = [
        (0, 1),
        (1, 1),
        (0, 2),
        (11, 1),
        (10, 1),
        (11, 2),
        (0, 14),
        (1, 14),
        (0, 13),
        (11, 14),
        (10, 14),
        (11, 13),
    ];
    !cuts.contains(&(x, y))
}
fn name(i: usize) -> String {
    format!(
        "{}{}",
        (b'a' + (i as i32 % W) as u8) as char,
        i as i32 / W + 1
    )
}
fn castle(owner: usize, x: i32, y: i32) -> bool {
    (x == 5 || x == 6) && y == if owner == 0 { 0 } else { H - 1 }
}
/// 0 gol; 1 pion J1; 2 cavaler J1; 3 pion J2; 4 cavaler J2.
fn owner(c: u8) -> Option<usize> {
    match c {
        0 => None,
        1 | 2 => Some(0),
        _ => Some(1),
    }
}
fn is_knight(c: u8) -> bool {
    c == 2 || c == 4
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Mv {
    pub path: Vec<u8>,
    pub capture: bool,
}

#[derive(Clone)]
pub struct Camelot {
    b: [u8; N],
    side: Player,
    quiet: u16,
}

impl Default for Camelot {
    fn default() -> Self {
        Self::new()
    }
}

const DIRS: [(i32, i32); 8] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

impl Camelot {
    pub fn new() -> Self {
        let mut b = [0u8; N];
        let wk = [(2, 5), (3, 6), (8, 6), (9, 5)];
        let wm = [
            (3, 5),
            (4, 5),
            (4, 6),
            (5, 5),
            (5, 6),
            (6, 5),
            (6, 6),
            (7, 5),
            (7, 6),
            (8, 5),
        ];
        let bk = [(2, 10), (3, 9), (8, 9), (9, 10)];
        let bm = [
            (3, 10),
            (4, 9),
            (4, 10),
            (5, 9),
            (5, 10),
            (6, 9),
            (6, 10),
            (7, 9),
            (7, 10),
            (8, 10),
        ];
        for (x, y) in wk {
            b[idx(x, y)] = 2;
        }
        for (x, y) in wm {
            b[idx(x, y)] = 1;
        }
        for (x, y) in bk {
            b[idx(x, y)] = 4;
        }
        for (x, y) in bm {
            b[idx(x, y)] = 3;
        }
        Camelot {
            b,
            side: Player::One,
            quiet: 0,
        }
    }

    pub fn count(&self, p: Player) -> usize {
        self.b
            .iter()
            .filter(|&&c| owner(c) == Some(p.idx()))
            .count()
    }

    fn in_castle(&self, p: usize) -> usize {
        // piese ale lui p în castelul adversarului
        let y = if p == 0 { H - 1 } else { 0 };
        [5, 6]
            .iter()
            .filter(|&&x| owner(self.b[idx(x, y)]) == Some(p))
            .count()
    }

    fn jumps(bd: &[u8; N], pos: usize, side: usize, path: &mut Vec<u8>, out: &mut Vec<Mv>) {
        let (x, y) = (pos as i32 % W, pos as i32 / W);
        for (dx, dy) in DIRS {
            let (mx, my, tx, ty) = (x + dx, y + dy, x + 2 * dx, y + 2 * dy);
            if !valid(tx, ty) || !valid(mx, my) {
                continue;
            }
            let m = bd[idx(mx, my)];
            if owner(m) == Some(1 - side) && bd[idx(tx, ty)] == 0 {
                let mut nb = *bd;
                nb[idx(mx, my)] = 0;
                nb[idx(tx, ty)] = nb[pos];
                nb[pos] = 0;
                path.push(idx(tx, ty) as u8);
                out.push(Mv {
                    path: path.clone(),
                    capture: true,
                });
                Self::jumps(&nb, idx(tx, ty), side, path, out);
                path.pop();
            }
        }
    }

    fn canters(
        bd: &[u8; N],
        pos: usize,
        side: usize,
        knight: bool,
        path: &mut Vec<u8>,
        out: &mut Vec<Mv>,
    ) {
        let (x, y) = (pos as i32 % W, pos as i32 / W);
        for (dx, dy) in DIRS {
            let (mx, my, tx, ty) = (x + dx, y + dy, x + 2 * dx, y + 2 * dy);
            if !valid(tx, ty) || !valid(mx, my) {
                continue;
            }
            let t = idx(tx, ty);
            if owner(bd[idx(mx, my)]) == Some(side)
                && bd[t] == 0
                && !castle(side, tx, ty)
                && !path.contains(&(t as u8))
            {
                let mut nb = *bd;
                nb[t] = nb[pos];
                nb[pos] = 0;
                path.push(t as u8);
                out.push(Mv {
                    path: path.clone(),
                    capture: false,
                });
                if knight {
                    Self::jumps(&nb, t, side, path, out);
                }
                Self::canters(&nb, t, side, knight, path, out);
                path.pop();
            }
        }
    }

    fn moves_for(&self, side: usize) -> Vec<Mv> {
        let mut all = Vec::new();
        for i in 0..N {
            let c = self.b[i];
            if owner(c) != Some(side) {
                continue;
            }
            let (x, y) = (i as i32 % W, i as i32 / W);
            for (dx, dy) in DIRS {
                let (tx, ty) = (x + dx, y + dy);
                if valid(tx, ty) && self.b[idx(tx, ty)] == 0 && !castle(side, tx, ty) {
                    all.push(Mv {
                        path: vec![i as u8, idx(tx, ty) as u8],
                        capture: false,
                    });
                }
            }
            let mut path = vec![i as u8];
            Self::canters(&self.b, i, side, is_knight(c), &mut path, &mut all);
            let mut path = vec![i as u8];
            Self::jumps(&self.b, i, side, &mut path, &mut all);
        }
        if all.iter().any(|m| m.capture) {
            all.retain(|m| m.capture);
        }
        // eliminăm duplicatele de text identice (aceeași cale)
        all.sort_by(|a, b| a.path.cmp(&b.path));
        all.dedup_by(|a, b| a.path == b.path);
        all
    }
}

impl Game for Camelot {
    type Move = Mv;
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<Mv> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        self.moves_for(self.side.idx())
    }
    fn apply(&mut self, m: &Mv) {
        let mut cap = false;
        for w in m.path.windows(2) {
            let (a, c) = (w[0] as i32, w[1] as i32);
            let (ax, ay, cx, cy) = (a % W, a / W, c % W, c / W);
            if (ax - cx).abs() == 2 || (ay - cy).abs() == 2 {
                let mid = idx((ax + cx) / 2, (ay + cy) / 2);
                if owner(self.b[mid]) != Some(self.side.idx()) {
                    self.b[mid] = 0;
                    cap = true;
                }
            }
            self.b[c as usize] = self.b[a as usize];
            self.b[a as usize] = 0;
        }
        self.quiet = if cap { 0 } else { self.quiet + 1 };
        self.side = self.side.other();
    }
    fn outcome(&self) -> Option<Outcome> {
        for p in [Player::One, Player::Two] {
            if self.in_castle(p.idx()) >= 2 {
                return Some(Outcome::Win(p));
            }
        }
        let (a, b) = (self.count(Player::One), self.count(Player::Two));
        if a <= 1 && b <= 1 {
            return Some(Outcome::Draw);
        }
        if b <= 1 && a >= 2 {
            return Some(Outcome::Win(Player::One));
        }
        if a <= 1 && b >= 2 {
            return Some(Outcome::Win(Player::Two));
        }
        if self.quiet >= 100 {
            return Some(Outcome::Draw);
        }
        if self.moves_for(self.side.idx()).is_empty() {
            return Some(Outcome::Win(self.side.other()));
        }
        None
    }
    fn evaluate(&self, p: Player) -> i32 {
        let mut s = 0;
        for i in 0..N {
            let c = self.b[i];
            let Some(o) = owner(c) else { continue };
            let y = i as i32 / W;
            let adv = if o == 0 { y } else { H - 1 - y };
            let v = if is_knight(c) { 300 } else { 100 } + adv * 4;
            if o == p.idx() {
                s += v;
            } else {
                s -= v;
            }
        }
        s += 200 * (self.in_castle(p.idx()) as i32 - self.in_castle(p.other().idx()) as i32);
        s
    }
    fn move_score(&self, m: &Mv) -> i32 {
        if m.capture {
            100 + m.path.len() as i32
        } else {
            0
        }
    }
    fn render(&self) -> String {
        let mut s = String::new();
        for y in (0..H).rev() {
            s.push_str(&format!("{:>2} ", y + 1));
            for x in 0..W {
                s.push(if !valid(x, y) {
                    ' '
                } else {
                    match self.b[idx(x, y)] {
                        1 => 'X',
                        2 => 'K',
                        3 => 'o',
                        4 => 'k',
                        _ => {
                            if y == 0 || y == H - 1 {
                                '#'
                            } else {
                                '.'
                            }
                        }
                    }
                });
                s.push(' ');
            }
            s.push('\n');
        }
        s.push_str("   a b c d e f g h i j k l\nX/K = pion/cavaler J1, o/k = pion/cavaler J2, # = castel.\n");
        s.push_str(&format!("La mutare: Jucătorul {}.", self.side.idx() + 1));
        s
    }
    fn format_move(&self, m: &Mv) -> String {
        let mut t = name(m.path[0] as usize);
        for w in m.path.windows(2) {
            let (a, c) = (w[0] as i32, w[1] as i32);
            let far = (a % W - c % W).abs() == 2 || (a / W - c / W).abs() == 2;
            let jump = far && {
                let mid = idx(((a % W) + (c % W)) / 2, ((a / W) + (c / W)) / 2);
                owner(self.b[mid]) == Some(1 - self.side.idx())
            };
            t.push(if jump { 'x' } else { '-' });
            t.push_str(&name(w[1] as usize));
        }
        t
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 300,
            max_depth: 5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q4_util::*;
    use polytabletop_core::{GameObj, Player};

    #[test]
    fn board_has_160_squares_and_setup() {
        let n = (0..H)
            .flat_map(|y| (0..W).map(move |x| (x, y)))
            .filter(|&(x, y)| valid(x, y))
            .count();
        assert_eq!(n, 160);
        let g = Camelot::new();
        assert_eq!(g.count(Player::One), 14);
        assert_eq!(g.count(Player::Two), 14);
    }

    #[test]
    fn jump_is_mandatory_and_knight_can_charge() {
        let mut g = Camelot::new();
        g.b = [0; N];
        g.b[idx(5, 5)] = 2; // cavaler J1 f6
        g.b[idx(5, 6)] = 1; // pion propriu f7
        g.b[idx(5, 8)] = 3; // pion J2 f9 (după canter f6->f8, sare peste f9 pe f10)
        g.b[idx(8, 8)] = 1;
        g.b[idx(10, 12)] = 3;
        let m = GameObj::legal_moves(&g);
        assert!(m.iter().all(|t| t.contains('x')), "{m:?}");
        assert!(m.contains(&"f6-f8xf10".to_string()), "{m:?}");
    }

    #[test]
    fn two_pieces_in_enemy_castle_win() {
        let mut g = Camelot::new();
        g.b = [0; N];
        g.b[idx(5, H - 1)] = 1;
        g.b[idx(6, H - 1)] = 1;
        g.b[idx(2, 5)] = 3;
        g.b[idx(3, 5)] = 3;
        assert_eq!(Game::outcome(&g), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn random_and_ai_games_finish() {
        random_finishes(|_| Box::new(Camelot::new()), 4, 3000);
        ai_vs_random(|_| Box::new(Camelot::new()), 2, Player::One, 3000);
        ai_vs_random(|_| Box::new(Camelot::new()), 3, Player::Two, 3000);
    }
}
