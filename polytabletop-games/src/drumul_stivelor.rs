//! Drumul stivelor (variantă proprie de stive pe tablă 5×5): fiecare jucător are 21 de pietre
//! plate și o piatră-coroană. O tură: fie pui o piesă pe o celulă goală (`f c3` piatră plată,
//! `s c3` piatră în picioare = zid, `c c3` coroană), fie muți o stivă pe care o conduci
//! (piesa de sus e a ta): iei până la 5 piese de sus, pornești într-o direcție și lași
//! câte una sau mai multe pe fiecare celulă parcursă. Pe zid nu se poate pune nimic, decât
//! dacă ultima piesă lăsată e coroana singură (ea turtește zidul); pe coroană nu se poate
//! pune nimic. În primele două ture fiecare pune o piatră plată a adversarului.
//! Câștigi cu un drum de pietre plate/coroane proprii (nu ziduri) între două margini
//! opuse; când un jucător rămâne fără piese sau tabla se umple, câștigă cine are mai multe
//! piese plate deasupra stivelor. Mutări: `f c3`, `c3n3:21` (din c3 spre nord, ia 3, lasă 2 apoi 1).
//! Direcții: n, s, e, v.
#![allow(clippy::all)]

use crate::b1_util::sq;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: usize = 5;
const DIR: [(char, i32, i32); 4] = [('n', 0, 1), ('s', 0, -1), ('e', 1, 0), ('v', -1, 0)];

#[derive(Clone, PartialEq, Debug)]
pub enum Mv {
    Place(u8, u8, u8),          // celula, tip 0 plat 1 zid 2 coroană
    Slide(u8, u8, u8, Vec<u8>), // celula, dir, ia, lasă[]
}

#[derive(Clone)]
pub struct Stive {
    s: Vec<Vec<u8>>, // fiecare piesă: owner(0/1)<<2 | tip(0,1,2)
    flats: [u8; 2],
    caps: [u8; 2],
    side: Player,
    ply: u32,
    res: Option<Outcome>,
}

impl Default for Stive {
    fn default() -> Self {
        Self::new()
    }
}

fn owner(p: u8) -> usize {
    (p >> 2) as usize
}
fn kind(p: u8) -> u8 {
    p & 3
}

impl Stive {
    pub fn new() -> Self {
        Stive {
            s: vec![vec![]; 25],
            flats: [21, 21],
            caps: [1, 1],
            side: Player::One,
            ply: 0,
            res: None,
        }
    }
    fn top(&self, i: usize) -> Option<u8> {
        self.s[i].last().copied()
    }
    fn road(&self, who: usize) -> bool {
        let ok = |i: usize| self.top(i).is_some_and(|p| owner(p) == who && kind(p) != 1);
        for vertical in [false, true] {
            let mut seen = [false; 25];
            let mut st = vec![];
            for k in 0..N {
                let i = if vertical { k } else { k * N };
                if ok(i) {
                    seen[i] = true;
                    st.push(i);
                }
            }
            while let Some(i) = st.pop() {
                let (x, y) = (i % N, i / N);
                if (vertical && y == N - 1) || (!vertical && x == N - 1) {
                    return true;
                }
                for (_, dx, dy) in DIR {
                    let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                    if nx < 0 || ny < 0 || nx >= N as i32 || ny >= N as i32 {
                        continue;
                    }
                    let j = ny as usize * N + nx as usize;
                    if !seen[j] && ok(j) {
                        seen[j] = true;
                        st.push(j);
                    }
                }
            }
        }
        false
    }
    fn flat_count(&self, who: usize) -> i32 {
        self.s
            .iter()
            .filter(|c| c.last().is_some_and(|&p| owner(p) == who && kind(p) == 0))
            .count() as i32
    }
    fn drops(&self, from: usize, d: usize, carry: usize, v: &mut Vec<Mv>) {
        let (_, dx, dy) = DIR[d];
        let st = &self.s[from];
        let carried: Vec<u8> = st[st.len() - carry..].to_vec();
        let topcap = kind(*carried.last().unwrap()) == 2;
        // recursie: pas i, rest, secvența
        fn rec(
            g: &Stive,
            from: usize,
            dx: i32,
            dy: i32,
            carry: usize,
            topcap: bool,
            step: i32,
            rest: usize,
            seq: &mut Vec<u8>,
            v: &mut Vec<Mv>,
            d: usize,
        ) {
            let (x, y) = ((from % N) as i32 + dx * step, (from / N) as i32 + dy * step);
            if x < 0 || y < 0 || x >= N as i32 || y >= N as i32 {
                return;
            }
            let t = y as usize * N + x as usize;
            if let Some(p) = g.top(t) {
                match kind(p) {
                    2 => return,
                    1 => {
                        // doar coroana singură, ca ultimă lăsare
                        if !(topcap && rest == 1) {
                            return;
                        }
                    }
                    _ => {}
                }
            }
            for k in 1..=rest {
                if let Some(p) = g.top(t) {
                    if kind(p) == 1 && !(topcap && rest == 1 && k == 1) {
                        continue;
                    }
                }
                seq.push(k as u8);
                if k == rest {
                    v.push(Mv::Slide(from as u8, d as u8, carry as u8, seq.clone()));
                } else {
                    rec(
                        g,
                        from,
                        dx,
                        dy,
                        carry,
                        topcap,
                        step + 1,
                        rest - k,
                        seq,
                        v,
                        d,
                    );
                }
                seq.pop();
            }
        }
        rec(
            self,
            from,
            dx,
            dy,
            carry,
            topcap,
            1,
            carry,
            &mut vec![],
            v,
            d,
        );
    }
    fn finish(&mut self) {
        let me = self.side.idx();
        let (a, b) = (self.road(me), self.road(1 - me));
        if a {
            self.res = Some(Outcome::Win(self.side));
        } else if b {
            self.res = Some(Outcome::Win(self.side.other()));
        } else {
            let full = self.s.iter().all(|c| !c.is_empty());
            let out = (self.flats[0] == 0 && self.caps[0] == 0)
                || (self.flats[1] == 0 && self.caps[1] == 0);
            if full || out || self.ply >= 200 {
                let (f0, f1) = (self.flat_count(0), self.flat_count(1));
                self.res = Some(match f0.cmp(&f1) {
                    std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                    std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                    _ => Outcome::Draw,
                });
            }
        }
    }
}

impl Game for Stive {
    type Move = Mv;
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<Mv> {
        if self.res.is_some() {
            return vec![];
        }
        let p = self.side.idx();
        let mut v = vec![];
        if self.ply < 2 {
            // se pune piatra plată a adversarului
            let q = 1 - p;
            if self.flats[q] > 0 {
                for i in 0..25 {
                    if self.s[i].is_empty() {
                        v.push(Mv::Place(i as u8, 0, 0));
                    }
                }
            }
            return v;
        }
        for i in 0..25 {
            if self.s[i].is_empty() {
                if self.flats[p] > 0 {
                    v.push(Mv::Place(i as u8, 0, 0));
                    v.push(Mv::Place(i as u8, 1, 0));
                }
                if self.caps[p] > 0 {
                    v.push(Mv::Place(i as u8, 2, 0));
                }
            } else if owner(self.top(i).unwrap()) == p {
                let h = self.s[i].len().min(5);
                for d in 0..4 {
                    for carry in 1..=h {
                        self.drops(i, d, carry, &mut v);
                    }
                }
            }
        }
        v
    }
    fn apply(&mut self, m: &Mv) {
        let p = self.side.idx();
        match m {
            Mv::Place(i, k, _) => {
                let o = if self.ply < 2 { 1 - p } else { p };
                self.s[*i as usize].push((o as u8) << 2 | k);
                if *k == 2 {
                    self.caps[o] -= 1;
                } else {
                    self.flats[o] -= 1;
                }
            }
            Mv::Slide(from, d, carry, seq) => {
                let (_, dx, dy) = DIR[*d as usize];
                let l = self.s[*from as usize].len();
                let carried: Vec<u8> = self.s[*from as usize].split_off(l - *carry as usize);
                let mut idx = 0;
                let (mut x, mut y) = ((*from as usize % N) as i32, (*from as usize / N) as i32);
                for &n in seq {
                    x += dx;
                    y += dy;
                    let t = y as usize * N + x as usize;
                    for &piece in &carried[idx..idx + n as usize] {
                        // coroana turtește zidul
                        if let Some(top) = self.s[t].last_mut() {
                            if kind(*top) == 1 {
                                *top = owner(*top) as u8 * 4;
                            }
                        }
                        self.s[t].push(piece);
                    }
                    idx += n as usize;
                }
            }
        }
        self.ply += 1;
        self.finish();
        self.side = self.side.other();
    }
    fn outcome(&self) -> Option<Outcome> {
        self.res
    }
    fn evaluate(&self, p: Player) -> i32 {
        let dist = |who: usize| -> i32 {
            // cel mai scurt drum (0-1 BFS) în celule de completat
            let mut best = 99;
            for vertical in [false, true] {
                let mut d = [99i32; 25];
                let mut dq = std::collections::VecDeque::new();
                for k in 0..N {
                    let i = if vertical { k } else { k * N };
                    if let Some(c) = self.cost(i, who) {
                        d[i] = c;
                        dq.push_back(i);
                    }
                }
                while let Some(i) = dq.pop_front() {
                    for (_, dx, dy) in DIR {
                        let (nx, ny) = ((i % N) as i32 + dx, (i / N) as i32 + dy);
                        if nx < 0 || ny < 0 || nx >= N as i32 || ny >= N as i32 {
                            continue;
                        }
                        let j = ny as usize * N + nx as usize;
                        if let Some(c) = self.cost(j, who) {
                            if d[i] + c < d[j] {
                                d[j] = d[i] + c;
                                dq.push_back(j);
                            }
                        }
                    }
                }
                for k in 0..N {
                    let i = if vertical {
                        (N - 1) * N + k
                    } else {
                        k * N + N - 1
                    };
                    best = best.min(d[i]);
                }
            }
            best
        };
        let me = p.idx();
        (dist(1 - me) - dist(me)) * 40 + (self.flat_count(me) - self.flat_count(1 - me)) * 6
    }
    fn render(&self) -> String {
        let mut s = String::new();
        for y in (0..N).rev() {
            s.push_str(&format!("{} ", y + 1));
            for x in 0..N {
                let st = &self.s[y * N + x];
                if st.is_empty() {
                    s.push_str("  .   ");
                } else {
                    let t = *st.last().unwrap();
                    let ch = match (owner(t), kind(t)) {
                        (0, 0) => 'x',
                        (0, 1) => 'X',
                        (0, _) => '@',
                        (_, 0) => 'o',
                        (_, 1) => 'O',
                        _ => '&',
                    };
                    s.push_str(&format!(
                        " {}{:<4}",
                        ch,
                        if st.len() > 1 {
                            st.len().to_string()
                        } else {
                            String::new()
                        }
                    ));
                }
            }
            s.push('\n');
        }
        s.push_str("     a     b     c     d     e\n");
        s.push_str(
            "x/o plat, X/O zid, @/& coroană (J1 = x, J2 = o); numărul = înălțimea stivei.\n",
        );
        s.push_str(&format!(
            "Rezerve: J1 {} plate + {} coroană, J2 {} plate + {} coroană\n",
            self.flats[0], self.caps[0], self.flats[1], self.caps[1]
        ));
        s
    }
    fn format_move(&self, m: &Mv) -> String {
        match m {
            Mv::Place(i, k, _) => format!(
                "{} {}",
                ['f', 's', 'c'][*k as usize],
                sq(*i as usize % N, *i as usize / N)
            ),
            Mv::Slide(f, d, c, seq) => {
                let sc: String = seq.iter().map(|n| char::from(b'0' + n)).collect();
                format!(
                    "{}{}{}:{}",
                    sq(*f as usize % N, *f as usize / N),
                    DIR[*d as usize].0,
                    c,
                    sc
                )
            }
        }
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 500,
            max_depth: 4,
        }
    }
    fn hash_key(&self) -> Option<u64> {
        let mut h = polytabletop_core::hash_mix(
            21 << 40 | (self.side.idx() as u64) << 32 | self.ply.min(2) as u64,
        );
        for (i, st) in self.s.iter().enumerate() {
            for (k, &p) in st.iter().enumerate() {
                h ^= polytabletop_core::hash_mix((p as u64 + 1) << 24 | (k as u64) << 8 | i as u64);
            }
        }
        Some(h)
    }
}

impl Stive {
    fn cost(&self, i: usize, who: usize) -> Option<i32> {
        match self.top(i) {
            None => Some(1),
            Some(p) if kind(p) == 1 => {
                if owner(p) == who {
                    Some(2)
                } else {
                    None
                }
            }
            Some(p) if owner(p) == who => Some(0),
            Some(p) if kind(p) == 2 => None,
            Some(_) => Some(2),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::b1_util::testing::check;
    fn play(g: &mut Stive, s: &str) {
        let m = g
            .legal_moves()
            .into_iter()
            .find(|m| g.format_move(m) == s)
            .unwrap_or_else(|| panic!("ilegal {s}"));
        g.apply(&m);
    }
    #[test]
    fn opening_gives_enemy_flat() {
        let mut g = Stive::new();
        play(&mut g, "f a1");
        assert_eq!(g.s[0][0] >> 2, 1); // J2 stăpânește piatra pusă de J1
        play(&mut g, "f e5");
        assert_eq!(g.s[24][0] >> 2, 0);
        assert_eq!(g.flats, [20, 20]);
    }
    #[test]
    fn stack_move_and_road() {
        let mut g = Stive::new();
        g.ply = 5;
        for i in 0..4 {
            g.s[i] = vec![0];
        }
        g.s[5] = vec![0];
        play(&mut g, "f e1");
        // rând complet a1..e1 deschide drumul stânga-dreapta pentru J1
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
        // mutare de stivă cu două lăsări
        let mut h = Stive::new();
        h.ply = 5;
        h.s[12] = vec![0, 0, 0];
        let m = h.legal_moves();
        assert!(m.iter().any(|x| h.format_move(x) == "c3n3:21"));
        assert!(m.iter().any(|x| h.format_move(x) == "c3n3:12"));
        assert!(!m.iter().any(|x| h.format_move(x) == "c3n3:111"));
    }
    #[test]
    fn wall_blocks_but_cap_flattens() {
        let mut g = Stive::new();
        g.ply = 5;
        g.s[0] = vec![0];
        g.s[1] = vec![1 | 4];
        assert!(!g.legal_moves().iter().any(|x| g.format_move(x) == "a1e1:1"));
        g.s[0] = vec![2];
        assert!(g.legal_moves().iter().any(|x| g.format_move(x) == "a1e1:1"));
        play(&mut g, "a1e1:1");
        assert_eq!(g.s[1], vec![4, 2]);
    }
    #[test]
    fn random_and_ai() {
        check(|_| Stive::new(), 400);
    }
}
