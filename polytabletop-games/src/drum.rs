//! Drum: construcție de drumuri cu stive pe o tablă 5×5. Reguli publice (joc de
//! tip „drum de pietre”), implementare originală.
//!
//! * Fiecare jucător are 21 de pietre plate și o piatră-căpetenie (capstone).
//! * Prima mutare a fiecăruia: pune o piatră plată a ADVERSARULUI pe un pătrat gol.
//! * Apoi, pe rând: (a) pui o piatră din rezervă pe un pătrat gol — plată (`c3`),
//!   în picioare / zid (`sc3`) sau căpetenia (`cc3`); sau (b) muți o stivă care
//!   îți are la vârf piatra: iei de sus 1–5 pietre și le lași pe o linie dreaptă
//!   (`+` sus, `-` jos, `<` stânga, `>` dreapta), cel puțin una pe fiecare pătrat
//!   parcurs, în ordinea de jos în sus. Notație: `[nr]pătrat dir [lăsate]`, ex.
//!   `c3>` (o piatră), `3c3>21` (3 pietre: 2 pe d3, 1 pe e3).
//! * Zidurile și căpetenia blochează mutarea stivelor; doar o căpetenie lăsată
//!   SINGURĂ ca ultimă piatră peste un zid îl turtește (devine plat). Căpetenia
//!   nu poate fi acoperită.
//! * Drum = lanț ortogonal de pietre proprii plate sau căpetenii (la vârful
//!   stivelor) între două margini opuse; cine îl închide câștigă (dacă o mutare
//!   face drum ambilor, câștigă cel care a mutat).
//! * Dacă tabla se umple, cineva își epuizează rezerva sau se ating 150 de
//!   semi-mutări, câștigă cine are mai multe pietre plate la vârf (egal = remiză).

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: usize = 5;
const CARRY: usize = 5;
const PLY_CAP: u16 = 150;
const COL0: u32 = 0b00001_00001_00001_00001_00001;
const COL4: u32 = COL0 << 4;
const ROW0: u32 = 0b11111;
const ROW4: u32 = ROW0 << 20;
const DIRS: [(i32, i32); 4] = [(0, 1), (0, -1), (-1, 0), (1, 0)];
const DCH: [char; 4] = ['+', '-', '<', '>'];

const FLAT: u8 = 0;
const STAND: u8 = 1;
const CAP: u8 = 2;

#[derive(Clone, Copy)]
struct Stack {
    h: u8,
    p: [u8; 44],
}

impl Stack {
    const EMPTY: Stack = Stack { h: 0, p: [0; 44] };
    fn top(&self) -> Option<u8> {
        (self.h > 0).then(|| self.p[self.h as usize - 1])
    }
}

fn color(p: u8) -> usize {
    (p & 1) as usize
}
fn kind(p: u8) -> u8 {
    p >> 1
}

#[derive(Clone, PartialEq, Debug)]
pub enum Mv {
    Place(u8, u8),
    /// from, direction, number taken, drops, number of drop squares
    Slide(u8, u8, u8, [u8; CARRY], u8),
}

#[derive(Clone)]
pub struct Drum {
    s: [Stack; N * N],
    flats: [u8; 2],
    caps: [u8; 2],
    side: Player,
    ply: u16,
    result: Option<Outcome>,
}

impl Default for Drum {
    fn default() -> Self {
        Self::new()
    }
}

fn sqn(i: usize) -> String {
    format!("{}{}", (b'a' + (i % N) as u8) as char, i / N + 1)
}

fn grow(cur: u32, mask: u32) -> u32 {
    let mut c = cur;
    loop {
        let n = (c | ((c << 1) & !COL0) | ((c >> 1) & !COL4) | (c << 5) | (c >> 5)) & mask;
        if n == c {
            return c;
        }
        c = n;
    }
}

fn has_road(mask: u32) -> bool {
    grow(mask & COL0, mask) & COL4 != 0 || grow(mask & ROW0, mask) & ROW4 != 0
}

impl Drum {
    pub fn new() -> Self {
        Drum {
            s: [Stack::EMPTY; N * N],
            flats: [21, 21],
            caps: [1, 1],
            side: Player::One,
            ply: 0,
            result: None,
        }
    }

    fn road_mask(&self, c: usize) -> u32 {
        let mut m = 0;
        for i in 0..N * N {
            if let Some(t) = self.s[i].top() {
                if color(t) == c && kind(t) != STAND {
                    m |= 1 << i;
                }
            }
        }
        m
    }

    fn flat_count(&self, c: usize) -> i32 {
        (0..N * N)
            .filter(|&i| matches!(self.s[i].top(), Some(t) if color(t) == c && kind(t) == FLAT))
            .count() as i32
    }

    fn check_end(&mut self, mover: usize) {
        let (a, b) = (
            has_road(self.road_mask(mover)),
            has_road(self.road_mask(1 - mover)),
        );
        if a {
            self.result = Some(Outcome::Win(if mover == 0 {
                Player::One
            } else {
                Player::Two
            }));
            return;
        }
        if b {
            self.result = Some(Outcome::Win(if mover == 0 {
                Player::Two
            } else {
                Player::One
            }));
            return;
        }
        let full = (0..N * N).all(|i| self.s[i].h > 0);
        let empty_reserve = (0..2).any(|c| self.flats[c] + self.caps[c] == 0);
        if full || empty_reserve || self.ply >= PLY_CAP {
            let (x, y) = (self.flat_count(0), self.flat_count(1));
            self.result = Some(match x.cmp(&y) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                std::cmp::Ordering::Equal => Outcome::Draw,
            });
        }
    }

    fn gen_slides(&self, from: usize, out: &mut Vec<Mv>) {
        let st = &self.s[from];
        let (x, y) = ((from % N) as i32, (from / N) as i32);
        for (d, (dx, dy)) in DIRS.iter().enumerate() {
            let max_steps = (0..N as i32)
                .take_while(|k| {
                    let (nx, ny) = (x + dx * (k + 1), y + dy * (k + 1));
                    nx >= 0 && ny >= 0 && nx < N as i32 && ny < N as i32
                })
                .count();
            if max_steps == 0 {
                continue;
            }
            for take in 1..=(st.h as usize).min(CARRY) {
                let top_is_cap = kind(st.p[st.h as usize - 1]) == CAP;
                let mut drops = [0u8; CARRY];
                self.rec(
                    from, d, take, take, 0, max_steps, top_is_cap, &mut drops, out,
                );
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn rec(
        &self,
        from: usize,
        d: usize,
        take: usize,
        remaining: usize,
        step: usize,
        max_steps: usize,
        top_is_cap: bool,
        drops: &mut [u8; CARRY],
        out: &mut Vec<Mv>,
    ) {
        if step >= max_steps {
            return;
        }
        let (dx, dy) = DIRS[d];
        let (x, y) = ((from % N) as i32, (from / N) as i32);
        let t = ((y + dy * (step as i32 + 1)) * N as i32 + x + dx * (step as i32 + 1)) as usize;
        let tt = self.s[t].top();
        match tt.map(kind) {
            Some(CAP) => return,
            Some(STAND) if !(top_is_cap && remaining == 1) => return,
            _ => {}
        }
        for dr in 1..=remaining {
            drops[step] = dr as u8;
            if dr == remaining {
                out.push(Mv::Slide(
                    from as u8,
                    d as u8,
                    take as u8,
                    *drops,
                    step as u8 + 1,
                ));
            } else if tt.map(kind) != Some(STAND) {
                self.rec(
                    from,
                    d,
                    take,
                    remaining - dr,
                    step + 1,
                    max_steps,
                    top_is_cap,
                    drops,
                    out,
                );
            }
        }
        drops[step] = 0;
    }

    /// Cheapest number of extra stones for `c` to complete a road (heuristic).
    fn road_cost(&self, c: usize) -> i32 {
        let cost = |i: usize| -> i32 {
            match self.s[i].top() {
                None => 1,
                Some(t) => {
                    if color(t) == c {
                        if kind(t) == STAND {
                            3
                        } else {
                            0
                        }
                    } else if kind(t) == FLAT {
                        3
                    } else {
                        5
                    }
                }
            }
        };
        let mut best = 99;
        for vertical in [false, true] {
            let mut d = [99i32; N * N];
            for k in 0..N {
                let i = if vertical { k } else { k * N };
                d[i] = cost(i);
            }
            loop {
                let mut ch = false;
                for i in 0..N * N {
                    if d[i] >= 99 {
                        continue;
                    }
                    let (x, y) = (i % N, i / N);
                    for (dx, dy) in DIRS {
                        let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                        if nx < 0 || ny < 0 || nx >= N as i32 || ny >= N as i32 {
                            continue;
                        }
                        let j = ny as usize * N + nx as usize;
                        let nd = d[i] + cost(j);
                        if nd < d[j] {
                            d[j] = nd;
                            ch = true;
                        }
                    }
                }
                if !ch {
                    break;
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
    }
}

impl Game for Drum {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if self.result.is_some() {
            return Vec::new();
        }
        let me = self.side.idx();
        let mut v = Vec::new();
        if self.ply < 2 {
            for i in 0..N * N {
                if self.s[i].h == 0 {
                    v.push(Mv::Place(i as u8, FLAT));
                }
            }
            return v;
        }
        for i in 0..N * N {
            if self.s[i].h == 0 {
                if self.flats[me] > 0 {
                    v.push(Mv::Place(i as u8, FLAT));
                    v.push(Mv::Place(i as u8, STAND));
                }
                if self.caps[me] > 0 {
                    v.push(Mv::Place(i as u8, CAP));
                }
            } else if matches!(self.s[i].top(), Some(t) if color(t) == me) {
                self.gen_slides(i, &mut v);
            }
        }
        v
    }

    fn apply(&mut self, m: &Mv) {
        let me = self.side.idx();
        match m {
            Mv::Place(i, k) => {
                let c = if self.ply < 2 { 1 - me } else { me };
                if *k == CAP {
                    self.caps[c] -= 1;
                } else {
                    self.flats[c] -= 1;
                }
                let st = &mut self.s[*i as usize];
                st.p[0] = c as u8 | k << 1;
                st.h = 1;
            }
            Mv::Slide(from, d, take, drops, n) => {
                let from = *from as usize;
                let (dx, dy) = DIRS[*d as usize];
                let (x, y) = ((from % N) as i32, (from / N) as i32);
                let h = self.s[from].h as usize;
                let take = *take as usize;
                let mut carried = [0u8; CARRY];
                carried[..take].copy_from_slice(&self.s[from].p[h - take..h]);
                self.s[from].h = (h - take) as u8;
                let mut idx = 0;
                #[allow(clippy::needless_range_loop)]
                for step in 0..*n as usize {
                    let t = ((y + dy * (step as i32 + 1)) * N as i32 + x + dx * (step as i32 + 1))
                        as usize;
                    let st = &mut self.s[t];
                    if st.h > 0 && kind(st.p[st.h as usize - 1]) == STAND {
                        st.p[st.h as usize - 1] &= 1; // flatten
                    }
                    for _ in 0..drops[step] {
                        st.p[st.h as usize] = carried[idx];
                        st.h += 1;
                        idx += 1;
                    }
                }
            }
        }
        self.ply += 1;
        self.check_end(me);
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        self.result
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let dm = self.road_cost(me);
        let dy = self.road_cost(1 - me);
        let mut score = (dy - dm) * 40 + (self.flat_count(me) - self.flat_count(1 - me)) * 20;
        for i in 0..N * N {
            if let Some(t) = self.s[i].top() {
                let sgn = if color(t) == me { 1 } else { -1 };
                if kind(t) == CAP {
                    score += sgn * 15;
                }
                // Control of tall stacks.
                score += sgn * (self.s[i].h as i32 - 1).min(4) * 3;
            }
        }
        score
    }

    fn render(&self) -> String {
        let mut s = String::new();
        for y in (0..N).rev() {
            s.push_str(&format!("{} ", y + 1));
            for x in 0..N {
                let st = &self.s[y * N + x];
                let mut cell = String::new();
                if st.h == 0 {
                    cell.push('.');
                } else {
                    for k in 0..st.h as usize {
                        let p = st.p[k];
                        let ch = if color(p) == 0 { 'x' } else { 'o' };
                        let ch = match kind(p) {
                            FLAT => ch,
                            STAND => format!("{}", ch).to_uppercase().chars().next().unwrap(),
                            _ => {
                                if ch == 'x' {
                                    '@'
                                } else {
                                    '#'
                                }
                            }
                        };
                        cell.push(ch);
                    }
                }
                s.push_str(&format!("{:<8}", cell));
            }
            s.push('\n');
        }
        s.push_str("  a       b       c       d       e\n");
        s.push_str("(jos→sus; x/o plate, X/O zid, @/# căpetenie; ultima = vârful)\n");
        s.push_str(&format!(
            "Rezervă: X {} plate + {} căp., O {} plate + {} căp.\n",
            self.flats[0], self.caps[0], self.flats[1], self.caps[1]
        ));
        if self.ply < 2 {
            s.push_str("Prima mutare: pui o piatră plată a adversarului.\n");
        }
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        match m {
            Mv::Place(i, k) => {
                let pre = match *k {
                    FLAT => "",
                    STAND => "s",
                    _ => "c",
                };
                format!("{}{}", pre, sqn(*i as usize))
            }
            Mv::Slide(from, d, take, drops, n) => {
                let mut s = String::new();
                if *take > 1 {
                    s.push_str(&take.to_string());
                }
                s.push_str(&sqn(*from as usize));
                s.push(DCH[*d as usize]);
                if *take > 1 {
                    for d in drops.iter().take(*n as usize) {
                        s.push_str(&d.to_string());
                    }
                }
                s
            }
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 600,
            max_depth: 8,
        }
    }

    fn hash_key(&self) -> Option<u64> {
        let mut h = polytabletop_core::hash_mix(
            5 << 40 | (self.side.idx() as u64) << 32 | self.ply.min(2) as u64,
        );
        for i in 0..N * N {
            for k in 0..self.s[i].h as usize {
                h ^= polytabletop_core::hash_mix(
                    (self.s[i].p[k] as u64 + 1) << 24 | (k as u64) << 8 | i as u64,
                );
            }
        }
        Some(h)
    }

    fn move_score(&self, m: &Mv) -> i32 {
        match m {
            Mv::Place(_, CAP) => 3,
            Mv::Place(_, FLAT) => 2,
            Mv::Slide(..) => 1,
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

    fn sq(s: &str) -> usize {
        let b = s.as_bytes();
        (b[1] - b'1') as usize * N + (b[0] - b'a') as usize
    }
    fn play(g: &mut Drum, s: &str) {
        let m = g
            .legal_moves()
            .into_iter()
            .find(|m| g.format_move(m) == s)
            .unwrap_or_else(|| {
                panic!(
                    "ilegal {s}: {:?}",
                    g.legal_moves()
                        .iter()
                        .map(|m| g.format_move(m))
                        .collect::<Vec<_>>()
                )
            });
        g.apply(&m);
    }
    fn has(g: &Drum, s: &str) -> bool {
        g.legal_moves().iter().any(|m| g.format_move(m) == s)
    }
    fn put(g: &mut Drum, s: &str, pieces: &[u8]) {
        let i = sq(s);
        g.s[i].h = pieces.len() as u8;
        g.s[i].p[..pieces.len()].copy_from_slice(pieces);
    }
    // piece codes: colour | kind << 1
    const XF: u8 = 0;
    const OF: u8 = 1;
    const XS: u8 = 2;
    const XC: u8 = 4;
    const OC: u8 = 5;

    fn open_game() -> Drum {
        let mut g = Drum::new();
        g.ply = 2;
        g
    }

    #[test]
    fn opening_places_opponents_stone() {
        let mut g = Drum::new();
        assert_eq!(g.legal_moves().len(), 25);
        play(&mut g, "a1");
        assert_eq!(g.s[0].top(), Some(OF));
        assert_eq!(g.flats, [21, 20]);
        play(&mut g, "e5");
        assert_eq!(g.s[24].top(), Some(XF));
        assert_eq!(g.flats, [20, 20]);
        // Now normal placements: flat, wall, cap on each of the 23 empty squares.
        assert!(has(&g, "sb2") && has(&g, "cb2") && has(&g, "b2"));
        // a1 holds O: X cannot slide it.
        assert!(!has(&g, "a1+"));
    }

    #[test]
    fn road_wins_and_walls_do_not_count() {
        let mut g = open_game();
        for f in ["a3", "b3", "c3", "d3"] {
            put(&mut g, f, &[XF]);
        }
        put(&mut g, "e2", &[OF]);
        assert!(has(&g, "e3"));
        play(&mut g, "e3");
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
        // A wall breaks the road.
        let mut g = open_game();
        for f in ["a3", "b3", "d3"] {
            put(&mut g, f, &[XF]);
        }
        put(&mut g, "c3", &[XS]);
        put(&mut g, "e2", &[OF]);
        play(&mut g, "e3");
        assert_eq!(g.outcome(), None);
        // A capstone counts.
        let mut g = open_game();
        for f in ["a3", "b3", "d3", "e3"] {
            put(&mut g, f, &[XF]);
        }
        g.flats[0] = 5;
        play(&mut g, "cc3");
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn stack_movement_rules() {
        let mut g = open_game();
        put(&mut g, "c3", &[OF, XF, XF, OF, XF, XF]); // height 6, X on top
        put(&mut g, "d3", &[OF]);
        put(&mut g, "b3", &[OF]);
        // Carry limit is 5.
        assert!(has(&g, "5c3>41") || has(&g, "5c3>5") || has(&g, "5c3>32"));
        assert!(!g
            .legal_moves()
            .iter()
            .any(|m| matches!(m, Mv::Slide(_, _, 6, ..))));
        play(&mut g, "3c3>12");
        // Pieces taken (top 3: OF? indices 3..6 = OF, XF, XF): bottom one dropped first.
        assert_eq!(g.s[sq("c3")].h, 3);
        assert_eq!(g.s[sq("d3")].h, 2);
        assert_eq!(g.s[sq("d3")].p[1], OF);
        assert_eq!(g.s[sq("e3")].h, 2);
        assert_eq!(g.s[sq("e3")].top(), Some(XF));
    }

    #[test]
    fn walls_block_and_capstone_flattens() {
        let mut g = open_game();
        put(&mut g, "c3", &[XF, XF]);
        put(&mut g, "d3", &[OF]);
        // Replace d3 with an O wall.
        put(&mut g, "d3", &[OF | 2]);
        // Cannot slide onto a wall with a flat on top.
        assert!(!has(&g, "c3>"));
        assert!(!has(&g, "2c3>11"));
        // Capstone alone flattens it.
        put(&mut g, "c3", &[XF, XC]);
        assert!(has(&g, "c3>"));
        // A capstone with a stone beneath it: dropping 2 on the wall is not allowed...
        assert!(!has(&g, "2c3>2"));
        play(&mut g, "c3>");
        assert_eq!(g.s[sq("d3")].h, 2);
        assert_eq!(kind(g.s[sq("d3")].p[0]), FLAT);
        assert_eq!(g.s[sq("d3")].top(), Some(XC));
        // A capstone cannot be covered or passed over.
        let mut g = open_game();
        put(&mut g, "c3", &[XF, XF]);
        put(&mut g, "d3", &[OC]);
        assert!(!has(&g, "c3>"));
        assert!(!has(&g, "2c3>11"));
        // Cap carried with another stone: the wall must be the final square and receive exactly the cap.
        let mut g = open_game();
        put(&mut g, "a1", &[XF, XC]);
        put(&mut g, "b1", &[OF | 2]);
        assert!(!has(&g, "2a1>2"));
        assert!(
            !has(&g, "2a1>11"),
            "second drop would be onto wall as a final single cap"
        );
        // Cap travelling with one more stone: flat on b1, cap flattens the wall on c1.
        let mut g = open_game();
        put(&mut g, "a1", &[XF, XC]);
        put(&mut g, "c1", &[OF | 2]);
        play(&mut g, "2a1>11");
        assert_eq!(g.s[sq("c1")].top(), Some(XC));
        assert_eq!(kind(g.s[sq("c1")].p[0]), FLAT);
    }

    #[test]
    fn flat_win_when_board_full() {
        let mut g = open_game();
        // Fill all squares but one; no road possible: alternate pattern with walls.
        for i in 0..N * N - 1 {
            let c = if i % 2 == 0 { XS } else { OF | 2 };
            g.s[i].h = 1;
            g.s[i].p[0] = c;
        }
        g.s[1].p[0] = XF;
        g.s[3].p[0] = OF;
        g.flats[0] = 10;
        g.flats[1] = 10;
        play(&mut g, "e5");
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=30u64 {
            let mut g = Drum::new();
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty(), "no moves at ply {n}");
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n <= 150);
                let total: u32 = (0..N * N).map(|i| g.s[i].h as u32).sum();
                assert_eq!(
                    total,
                    44 - (g.flats[0] + g.flats[1] + g.caps[0] + g.caps[1]) as u32
                );
            }
        }
    }
}
