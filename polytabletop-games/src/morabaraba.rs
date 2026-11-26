//! Morabaraba (Africa de Sud): 24 de puncte ca la Moară, plus diagonalele
//! prin colțuri (a1-b2-c3, g1-f2-e3, a7-b6-c5, g7-f6-e5): 20 de „vaci”
//! (linii de trei). Fiecare jucător are 12 piese.
//! Faza 1: plasare, o piesă pe rând; o vacă formată scoate o piesă adversă
//! (nu din vacă, decât dacă toate sunt în vaci). Faza 2: mutare pe linii la
//! un punct vecin liber (inclusiv diagonale). Cu exact 3 piese poți „zbura”.
//! Pierde cine rămâne cu 2 piese sau nu mai poate muta. Dacă tabla se umple
//! complet la sfârșitul plasării, remiză; remiză și după 100 de mutări fără
//! captură.

//! Verificare surse (nov. 2026: en.wikipedia.org/wiki/Morabaraba): 12 „vaci” pe jucător,
//! plasare apoi mutare, vaci protejate în afară de cazul în care toate sunt în vaci,
//! zbor la 3 piese, pierdere la 2 piese sau fără mutări corespund. Diferențe: sursa dă
//! remiză când un jucător are 3 vaci și nimeni nu capturează în 10 mutări (aici 100 de
//! mutări fără captură) și, la regulile Mind Sports South Africa, o vacă desfăcută nu
//! se poate reface chiar la mutarea următoare (neimplementat).

use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::sync::OnceLock;

const NAMES: [&str; 24] = [
    "a1", "d1", "g1", "b2", "d2", "f2", "c3", "d3", "e3", "a4", "b4", "c4", "e4", "f4", "g4", "c5",
    "d5", "e5", "b6", "d6", "f6", "a7", "d7", "g7",
];

const MILL_NAMES: [[&str; 3]; 20] = [
    ["a1", "d1", "g1"],
    ["b2", "d2", "f2"],
    ["c3", "d3", "e3"],
    ["a4", "b4", "c4"],
    ["e4", "f4", "g4"],
    ["c5", "d5", "e5"],
    ["b6", "d6", "f6"],
    ["a7", "d7", "g7"],
    ["a1", "a4", "a7"],
    ["b2", "b4", "b6"],
    ["c3", "c4", "c5"],
    ["d1", "d2", "d3"],
    ["d5", "d6", "d7"],
    ["e3", "e4", "e5"],
    ["f2", "f4", "f6"],
    ["g1", "g4", "g7"],
    ["a1", "b2", "c3"],
    ["g1", "f2", "e3"],
    ["a7", "b6", "c5"],
    ["g7", "f6", "e5"],
];

struct Tables {
    mills: Vec<[usize; 3]>,
    adj: Vec<Vec<usize>>,
    mills_of: Vec<Vec<usize>>,
}

fn idx(name: &str) -> usize {
    NAMES.iter().position(|&n| n == name).expect("point name")
}

fn tables() -> &'static Tables {
    static T: OnceLock<Tables> = OnceLock::new();
    T.get_or_init(|| {
        let mills: Vec<[usize; 3]> = MILL_NAMES
            .iter()
            .map(|m| [idx(m[0]), idx(m[1]), idx(m[2])])
            .collect();
        let mut adj = vec![Vec::new(); 24];
        let mut mills_of = vec![Vec::new(); 24];
        for (k, m) in mills.iter().enumerate() {
            for &p in m {
                mills_of[p].push(k);
            }
            for w in [(m[0], m[1]), (m[1], m[2])] {
                if !adj[w.0].contains(&w.1) {
                    adj[w.0].push(w.1);
                }
                if !adj[w.1].contains(&w.0) {
                    adj[w.1].push(w.0);
                }
            }
        }
        Tables {
            mills,
            adj,
            mills_of,
        }
    })
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Mv {
    pub from: Option<u8>,
    pub to: u8,
    pub take: Option<u8>,
}

#[derive(Clone)]
pub struct Morabaraba {
    b: [u8; 24],
    hand: [u8; 2],
    side: Player,
    quiet: u16,
}

impl Default for Morabaraba {
    fn default() -> Self {
        Self::new()
    }
}

impl Morabaraba {
    pub fn new() -> Self {
        Morabaraba {
            b: [0; 24],
            hand: [12, 12],
            side: Player::One,
            quiet: 0,
        }
    }

    fn on_board(&self, color: u8) -> usize {
        self.b.iter().filter(|&&c| c == color).count()
    }

    fn total(&self, p: Player) -> usize {
        self.on_board(p.idx() as u8 + 1) + self.hand[p.idx()] as usize
    }

    fn in_cow(b: &[u8; 24], i: usize) -> bool {
        let c = b[i];
        let t = tables();
        c != 0
            && t.mills_of[i]
                .iter()
                .any(|&k| t.mills[k].iter().all(|&q| b[q] == c))
    }

    fn removable(b: &[u8; 24], victim: u8) -> Vec<u8> {
        let all: Vec<usize> = (0..24).filter(|&i| b[i] == victim).collect();
        let free: Vec<usize> = all
            .iter()
            .copied()
            .filter(|&i| !Self::in_cow(b, i))
            .collect();
        let pick = if free.is_empty() { all } else { free };
        pick.into_iter().map(|i| i as u8).collect()
    }

    fn placing(&self) -> bool {
        self.hand[self.side.idx()] > 0
    }

    fn gen(&self) -> Vec<Mv> {
        let me = self.side.idx() as u8 + 1;
        let opp = 3 - me;
        let t = tables();
        let mut base: Vec<(Option<u8>, u8)> = Vec::new();
        if self.placing() {
            for i in 0..24 {
                if self.b[i] == 0 {
                    base.push((None, i as u8));
                }
            }
        } else {
            let fly = self.on_board(me) == 3;
            for from in 0..24 {
                if self.b[from] != me {
                    continue;
                }
                if fly {
                    for to in 0..24 {
                        if self.b[to] == 0 {
                            base.push((Some(from as u8), to as u8));
                        }
                    }
                } else {
                    for &to in &t.adj[from] {
                        if self.b[to] == 0 {
                            base.push((Some(from as u8), to as u8));
                        }
                    }
                }
            }
        }
        let mut out = Vec::new();
        for (from, to) in base {
            let mut nb = self.b;
            if let Some(f) = from {
                nb[f as usize] = 0;
            }
            nb[to as usize] = me;
            if Self::in_cow(&nb, to as usize) {
                for take in Self::removable(&nb, opp) {
                    out.push(Mv {
                        from,
                        to,
                        take: Some(take),
                    });
                }
            } else {
                out.push(Mv {
                    from,
                    to,
                    take: None,
                });
            }
        }
        out
    }

    fn mobility(&self, color: u8) -> i32 {
        let t = tables();
        (0..24)
            .filter(|&i| self.b[i] == color && t.adj[i].iter().any(|&j| self.b[j] == 0))
            .count() as i32
    }

    fn cows(&self, color: u8) -> i32 {
        tables()
            .mills
            .iter()
            .filter(|m| m.iter().all(|&k| self.b[k] == color))
            .count() as i32
    }

    fn open_pairs(&self, color: u8) -> i32 {
        tables()
            .mills
            .iter()
            .filter(|m| {
                m.iter().filter(|&&k| self.b[k] == color).count() == 2
                    && m.iter().any(|&k| self.b[k] == 0)
            })
            .count() as i32
    }

    fn terminal(&self) -> Option<Outcome> {
        for p in [Player::One, Player::Two] {
            if self.total(p) < 3 {
                return Some(Outcome::Win(p.other()));
            }
        }
        if self.hand == [0, 0] && (self.quiet >= 100 || self.b.iter().all(|&c| c != 0)) {
            return Some(Outcome::Draw);
        }
        if !self.placing() {
            let me = self.side.idx() as u8 + 1;
            let fly = self.on_board(me) == 3;
            let t = tables();
            let can = (0..24).any(|i| {
                self.b[i] == me
                    && if fly {
                        self.b.contains(&0)
                    } else {
                        t.adj[i].iter().any(|&j| self.b[j] == 0)
                    }
            });
            if !can {
                return Some(Outcome::Win(self.side.other()));
            }
        }
        None
    }
}

impl Game for Morabaraba {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if self.terminal().is_some() {
            return Vec::new();
        }
        self.gen()
    }

    fn apply(&mut self, m: &Mv) {
        let me = self.side.idx() as u8 + 1;
        match m.from {
            Some(f) => self.b[f as usize] = 0,
            None => self.hand[self.side.idx()] -= 1,
        }
        self.b[m.to as usize] = me;
        if let Some(t) = m.take {
            self.b[t as usize] = 0;
            self.quiet = 0;
        } else {
            self.quiet += 1;
        }
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        self.terminal()
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx() as u8 + 1;
        let o = 3 - me;
        let pieces = self.total(p) as i32 - self.total(p.other()) as i32;
        let mob = self.mobility(me) - self.mobility(o);
        let cows = self.cows(me) - self.cows(o);
        let pairs = self.open_pairs(me) - self.open_pairs(o);
        100 * pieces + 8 * mob + 6 * cows + 12 * pairs
    }

    fn render(&self) -> String {
        let (w, h) = (37usize, 13usize);
        let mut g = vec![vec![' '; w]; h];
        let pos = |name: &str| -> (usize, usize) {
            let f = (name.as_bytes()[0] - b'a') as usize;
            let r = (name.as_bytes()[1] - b'0') as usize;
            (6 * f, 2 * (7 - r))
        };
        for m in MILL_NAMES.iter() {
            for pair in [(m[0], m[1]), (m[1], m[2])] {
                let (x1, y1) = pos(pair.0);
                let (x2, y2) = pos(pair.1);
                if y1 == y2 {
                    for cell in &mut g[y1][x1.min(x2) + 1..x1.max(x2)] {
                        *cell = '-';
                    }
                } else if x1 == x2 {
                    for row in &mut g[y1.min(y2) + 1..y1.max(y2)] {
                        row[x1] = '|';
                    }
                } else {
                    let ch = if (x2 > x1) == (y2 > y1) { '\\' } else { '/' };
                    g[(y1 + y2) / 2][(x1 + x2) / 2] = ch;
                }
            }
        }
        for (i, n) in NAMES.iter().enumerate() {
            let (x, y) = pos(n);
            g[y][x] = match self.b[i] {
                1 => 'X',
                2 => 'O',
                _ => '+',
            };
        }
        let mut s = String::from("    a     b     c     d     e     f     g\n");
        for (y, row) in g.iter().enumerate() {
            let label = if y % 2 == 0 {
                format!("{} ", 7 - y / 2)
            } else {
                "  ".to_string()
            };
            s.push_str(&label);
            s.push_str(&row.iter().collect::<String>());
            s.push('\n');
        }
        s.push_str(&format!(
            "  În mână: X {}  O {}   Pe tablă: X {}  O {}\n",
            self.hand[0],
            self.hand[1],
            self.on_board(1),
            self.on_board(2)
        ));
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        let mut s = match m.from {
            Some(f) => format!("{}-{}", NAMES[f as usize], NAMES[m.to as usize]),
            None => NAMES[m.to as usize].to_string(),
        };
        if let Some(t) = m.take {
            s.push('x');
            s.push_str(NAMES[t as usize]);
        }
        s
    }

    fn ai_style(&self) -> AiStyle {
        if self.hand[0] + self.hand[1] > 0 {
            AiStyle::AlphaBeta(4)
        } else {
            AiStyle::AlphaBeta(5)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    fn blank(hand: [u8; 2]) -> Morabaraba {
        Morabaraba {
            b: [0; 24],
            hand,
            side: Player::One,
            quiet: 0,
        }
    }

    fn put(g: &mut Morabaraba, color: u8, pts: &[&str]) {
        for p in pts {
            g.b[idx(p)] = color;
        }
    }

    fn texts(g: &Morabaraba) -> Vec<String> {
        g.legal_moves().iter().map(|m| g.format_move(m)).collect()
    }

    #[test]
    fn graph_has_diagonals() {
        let t = tables();
        assert_eq!(t.mills.len(), 20);
        let edges: usize = t.adj.iter().map(|a| a.len()).sum::<usize>() / 2;
        assert_eq!(edges, 40);
        assert_eq!(t.adj[idx("b2")].len(), 4);
        assert_eq!(t.adj[idx("d2")].len(), 4);
        assert!(t.adj[idx("a1")].contains(&idx("b2")));
        assert_eq!(Morabaraba::new().legal_moves().len(), 24);
    }

    #[test]
    fn diagonal_cow_captures_in_placement() {
        let mut g = blank([10, 10]);
        put(&mut g, 1, &["a1", "b2"]);
        put(&mut g, 2, &["a4", "d7"]);
        let t = texts(&g);
        assert!(t.contains(&"c3xa4".to_string()), "{t:?}");
        assert!(t.contains(&"c3xd7".to_string()));
        assert!(!t.contains(&"c3".to_string()));
    }

    #[test]
    fn cow_pieces_protected_unless_all_in_cows() {
        let mut g = blank([5, 5]);
        put(&mut g, 1, &["a1", "d1"]);
        put(&mut g, 2, &["a4", "b4", "c4", "g7"]);
        let t = texts(&g);
        assert!(t.contains(&"g1xg7".to_string()));
        assert!(!t.contains(&"g1xa4".to_string()));
        let mut h = blank([5, 5]);
        put(&mut h, 1, &["a1", "d1"]);
        put(&mut h, 2, &["a4", "b4", "c4"]);
        assert!(texts(&h).contains(&"g1xa4".to_string()));
    }

    #[test]
    fn moving_along_diagonal_and_flying() {
        let mut g = blank([0, 0]);
        put(&mut g, 1, &["a1", "d7", "g7", "a7"]);
        put(&mut g, 2, &["c5", "e3", "g4", "d2"]);
        let t = texts(&g);
        assert!(t.contains(&"a1-b2".to_string()));
        assert!(!t.contains(&"a1-c3".to_string()));
        // Cu 3 piese se zboară.
        let mut f = blank([0, 0]);
        put(&mut f, 1, &["a1", "d7", "g7"]);
        put(&mut f, 2, &["c5", "e3", "g4", "d2"]);
        let t = texts(&f);
        assert!(t.contains(&"a1-d5".to_string()), "{t:?}");
    }

    #[test]
    fn two_pieces_lose_and_full_board_draws() {
        let mut g = blank([0, 0]);
        put(&mut g, 1, &["a1", "d7", "g7"]);
        put(&mut g, 2, &["c5", "e3"]);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
        let mut d = blank([0, 0]);
        for i in 0..24 {
            d.b[i] = if i % 2 == 0 { 1 } else { 2 };
        }
        assert_eq!(d.outcome(), Some(Outcome::Draw));
    }

    #[test]
    fn random_games_terminate() {
        for seed in 1..=5u64 {
            let mut g = Morabaraba::new();
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() && n < 3000 {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                assert!(g.total(Player::One) <= 12 && g.total(Player::Two) <= 12);
                n += 1;
            }
            assert!(g.outcome().is_some());
        }
    }

    #[test]
    fn render_shows_diagonals() {
        let r = Morabaraba::new().render();
        assert!(r.contains('/') && r.contains('\\'));
    }

    #[test]
    fn ai_is_fast() {
        let mut g = Morabaraba::new();
        let mut rng = Rng::new(4);
        for _ in 0..50 {
            if g.outcome().is_some() {
                break;
            }
            let t = std::time::Instant::now();
            let m = polytabletop_core::GameObj::ai_move(&g, rng.next_u64()).unwrap();
            if !cfg!(debug_assertions) {
                assert!(
                    t.elapsed().as_secs_f64() < 1.5,
                    "slow AI: {:?}",
                    t.elapsed()
                );
            }
            polytabletop_core::GameObj::play(&mut g, &m).unwrap();
        }
    }
}
