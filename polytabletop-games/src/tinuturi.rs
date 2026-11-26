//! Ținuturi — domino de teritorii cu coroane, variantă originală pentru doi.
//!
//! Fiecare jucător are un regat pe o grilă 9×9 cu castelul în `e5`; regatul final trebuie să
//! încapă într-un pătrat 7×7. Șase teritorii: Câmp (C), Pădure (P), Apă (A), Pășune (S),
//! Mlaștină (M), Stâncă (R); unele pătrate poartă coroane (cifra de lângă literă).
//! Setul nostru are 48 de piese-domino numerotate (generate de noi); într-un joc se folosesc
//! 24, trase la întâmplare. Rânduri de câte 4 piese, în ordinea numerelor.
//!
//! * Runda 0: se aleg cele 4 piese ale primului rând în ordinea J1, J2, J2, J1 (`alege 3`).
//! * Apoi se dezvăluie rândul următor (mutare forțată `revelare`). Piesele rândului curent se
//!   joacă în ordinea numerelor: proprietarul o așază (`pune e6-e7`: prima jumătate pe e6, a
//!   doua pe e7) sau, doar dacă nu are nicio așezare legală, o aruncă (`descarta`), apoi își
//!   alege o piesă din rândul următor. Cine are piesa cu numărul mai mic joacă mai devreme.
//! * Așezare legală: ambele pătrate libere, regatul rămâne în 7×7 și cel puțin un pătrat se
//!   lipește pe lat de castel sau de un pătrat de același teritoriu.
//! * Scor: fiecare regiune conexă de același teritoriu = (număr de pătrate) × (coroane).
//!   +5 Armonie (nicio piesă aruncată), +10 Regat central (castelul e în centrul regatului).
//!   Egalitate: regiunea cea mai mare, apoi cele mai multe coroane.
//!
//! Informația e deschisă; singura întâmplare e ordinea pachetului (`revelare`). AI-ul nu
//! citește pachetul: dezvăluirea e mutare separată.
#![allow(clippy::needless_range_loop, clippy::doc_lazy_continuation)]

use crate::cards::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const N: usize = 9;
const CASTLE: u8 = 6;
const EMPTY: u8 = 255;
pub const LETTERS: [char; 6] = ['C', 'P', 'A', 'S', 'M', 'R'];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Tile {
    pub num: u8,
    pub t: [u8; 2],
    pub c: [u8; 2],
}

/// Cele 48 de piese, generate determinist (propriile noastre).
pub fn all_tiles() -> Vec<Tile> {
    let counts = [22usize, 20, 18, 14, 12, 10];
    let mut sq: Vec<u8> = Vec::new();
    for (t, n) in counts.iter().enumerate() {
        for _ in 0..*n {
            sq.push(t as u8);
        }
    }
    let mut rng = Rng::new(0x7131_C0DE);
    shuffle(&mut sq, &mut rng);
    let mut v = Vec::new();
    for i in 0..48usize {
        let k = match i {
            0..=11 => 0,
            12..=29 => 1,
            30..=41 => 2,
            _ => 3,
        };
        let mut c = [0u8; 2];
        match k {
            1 => c[rng.below(2)] = 1,
            2 => {
                if rng.below(2) == 0 {
                    c = [1, 1];
                } else {
                    c[rng.below(2)] = 2;
                }
            }
            3 => {
                let a = rng.below(2);
                c[a] = 2;
                c[1 - a] = 1;
            }
            _ => {}
        }
        v.push(Tile {
            num: i as u8 + 1,
            t: [sq[2 * i], sq[2 * i + 1]],
            c,
        });
    }
    v
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    /// Alege piesa `i` din rândul curent (faza de deschidere) sau din cel următor.
    Pick(u8),
    /// Prima jumătate la (r,c), a doua la (r2,c2).
    Place(u8, u8, u8, u8),
    Discard,
    Reveal,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Open(u8),
    Reveal,
    Place,
    Pick,
    Done,
}

#[derive(Clone)]
pub struct Tinuturi {
    deck: Vec<Tile>,
    cur: Vec<(Tile, Option<Player>)>,
    next: Vec<(Tile, Option<Player>)>,
    grid: [[[u8; N]; N]; 2],
    crown: [[[u8; N]; N]; 2],
    discards: [u8; 2],
    placed: [u8; 2],
    idx: usize,
    phase: Phase,
    rng: Rng,
}

impl Default for Tinuturi {
    fn default() -> Self {
        Self::new()
    }
}

fn fmt_cell(r: usize, c: usize) -> String {
    format!("{}{}", (b'a' + c as u8) as char, r + 1)
}

impl Tinuturi {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut all = all_tiles();
        shuffle(&mut all, &mut rng);
        all.truncate(24);
        let mut first: Vec<Tile> = all.drain(..4).collect();
        first.sort_by_key(|t| t.num);
        let mut grid = [[[EMPTY; N]; N]; 2];
        for p in 0..2 {
            grid[p][4][4] = CASTLE;
        }
        Tinuturi {
            deck: all,
            cur: first.into_iter().map(|t| (t, None)).collect(),
            next: Vec::new(),
            grid,
            crown: [[[0; N]; N]; 2],
            discards: [0; 2],
            placed: [0; 2],
            idx: 0,
            phase: Phase::Open(0),
            rng,
        }
    }

    fn open_order(k: u8) -> Player {
        if k == 0 || k == 3 {
            Player::One
        } else {
            Player::Two
        }
    }

    fn bbox(&self, p: usize) -> (usize, usize, usize, usize) {
        let (mut r0, mut r1, mut c0, mut c1) = (N, 0, N, 0);
        for r in 0..N {
            for c in 0..N {
                if self.grid[p][r][c] != EMPTY {
                    r0 = r0.min(r);
                    r1 = r1.max(r);
                    c0 = c0.min(c);
                    c1 = c1.max(c);
                }
            }
        }
        (r0, r1, c0, c1)
    }

    fn neighbors(r: usize, c: usize) -> Vec<(usize, usize)> {
        let mut v = Vec::new();
        if r > 0 {
            v.push((r - 1, c));
        }
        if r + 1 < N {
            v.push((r + 1, c));
        }
        if c > 0 {
            v.push((r, c - 1));
        }
        if c + 1 < N {
            v.push((r, c + 1));
        }
        v
    }

    fn placements(&self, p: usize, tile: &Tile) -> Vec<Move> {
        let (r0, r1, c0, c1) = self.bbox(p);
        let mut v = Vec::new();
        for r in 0..N {
            for c in 0..N {
                if self.grid[p][r][c] != EMPTY {
                    continue;
                }
                for (r2, c2) in Self::neighbors(r, c) {
                    if self.grid[p][r2][c2] != EMPTY {
                        continue;
                    }
                    let nr0 = r0.min(r).min(r2);
                    let nr1 = r1.max(r).max(r2);
                    let nc0 = c0.min(c).min(c2);
                    let nc1 = c1.max(c).max(c2);
                    if nr1 - nr0 >= 7 || nc1 - nc0 >= 7 {
                        continue;
                    }
                    let ok = |rr: usize, cc: usize, t: u8| {
                        Self::neighbors(rr, cc).iter().any(|&(a, b)| {
                            let g = self.grid[p][a][b];
                            g == CASTLE || g == t
                        })
                    };
                    if ok(r, c, tile.t[0]) || ok(r2, c2, tile.t[1]) {
                        v.push(Move::Place(r as u8, c as u8, r2 as u8, c2 as u8));
                    }
                }
            }
        }
        v
    }

    /// Regiuni: (teritoriu, pătrate, coroane).
    fn regions(&self, p: usize) -> Vec<(u8, u32, u32)> {
        let mut seen = [[false; N]; N];
        let mut out = Vec::new();
        for r in 0..N {
            for c in 0..N {
                let t = self.grid[p][r][c];
                if t >= 6 || seen[r][c] {
                    continue;
                }
                let (mut size, mut cr) = (0u32, 0u32);
                let mut st = vec![(r, c)];
                seen[r][c] = true;
                while let Some((a, b)) = st.pop() {
                    size += 1;
                    cr += u32::from(self.crown[p][a][b]);
                    for (x, y) in Self::neighbors(a, b) {
                        if !seen[x][y] && self.grid[p][x][y] == t {
                            seen[x][y] = true;
                            st.push((x, y));
                        }
                    }
                }
                out.push((t, size, cr));
            }
        }
        out
    }

    pub fn base_score(&self, p: usize) -> i32 {
        self.regions(p)
            .iter()
            .map(|&(_, s, c)| (s * c) as i32)
            .sum()
    }

    fn bonuses(&self, p: usize) -> i32 {
        let mut b = 0;
        if self.discards[p] == 0 {
            b += 5;
        }
        let (r0, r1, c0, c1) = self.bbox(p);
        if r0 + r1 == 8 && c0 + c1 == 8 {
            b += 10;
        }
        b
    }

    pub fn score(&self, p: usize) -> i32 {
        self.base_score(p) + self.bonuses(p)
    }

    fn tiebreak(&self, p: usize) -> (u32, u32) {
        let reg = self.regions(p);
        (
            reg.iter().map(|x| x.1).max().unwrap_or(0),
            reg.iter().map(|x| x.2).sum(),
        )
    }

    fn cur_owner(&self) -> Player {
        self.cur[self.idx].1.expect("owner")
    }

    fn advance_after_turn(&mut self) {
        self.idx += 1;
        if self.idx >= self.cur.len() {
            if self.next.is_empty() {
                self.phase = Phase::Done;
            } else {
                self.cur = std::mem::take(&mut self.next);
                self.idx = 0;
                self.phase = Phase::Reveal;
            }
        } else {
            self.phase = Phase::Place;
        }
    }

    fn heuristic(&self, p: usize) -> f64 {
        let mut s = f64::from(self.base_score(p)) + f64::from(self.bonuses(p)) * 0.6;
        // Piese revendicate încă neașezate: coroane și potrivire cu regatul.
        let me = if p == 0 { Player::One } else { Player::Two };
        for (t, o) in self.cur.iter().skip(self.idx).chain(self.next.iter()) {
            if *o == Some(me) {
                let crowns: f64 = f64::from(t.c[0] + t.c[1]);
                s += 2.2 * crowns - 0.05 * f64::from(t.num);
                for k in 0..2 {
                    if self.regions(p).iter().any(|r| r.0 == t.t[k]) {
                        s += 0.8;
                    }
                }
            }
        }
        s
    }
}

impl Game for Tinuturi {
    type Move = Move;

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Open(k) => Self::open_order(k),
            Phase::Place | Phase::Pick => self.cur_owner(),
            Phase::Reveal => {
                // revelarea o face cel care urmează
                if self.idx < self.cur.len() {
                    self.cur[0].1.unwrap_or(Player::One)
                } else {
                    Player::One
                }
            }
            Phase::Done => Player::One,
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        match self.phase {
            Phase::Open(_) => (0..self.cur.len())
                .filter(|&i| self.cur[i].1.is_none())
                .map(|i| Move::Pick(i as u8))
                .collect(),
            Phase::Reveal => vec![Move::Reveal],
            Phase::Place => {
                let p = self.cur_owner().idx();
                let v = self.placements(p, &self.cur[self.idx].0);
                if v.is_empty() {
                    vec![Move::Discard]
                } else {
                    v
                }
            }
            Phase::Pick => (0..self.next.len())
                .filter(|&i| self.next[i].1.is_none())
                .map(|i| Move::Pick(i as u8))
                .collect(),
            Phase::Done => Vec::new(),
        }
    }

    fn apply(&mut self, m: &Move) {
        match (self.phase, *m) {
            (Phase::Open(k), Move::Pick(i)) => {
                self.cur[i as usize].1 = Some(Self::open_order(k));
                self.phase = if k == 3 {
                    Phase::Reveal
                } else {
                    Phase::Open(k + 1)
                };
            }
            (Phase::Reveal, Move::Reveal) => {
                let n = self.deck.len().min(4);
                let mut row: Vec<Tile> = Vec::new();
                for _ in 0..n {
                    let k = self.rng.below(self.deck.len());
                    row.push(self.deck.swap_remove(k));
                }
                row.sort_by_key(|t| t.num);
                self.next = row.into_iter().map(|t| (t, None)).collect();
                self.idx = 0;
                self.phase = Phase::Place;
            }
            (Phase::Place, Move::Place(r, c, r2, c2)) => {
                let p = self.cur_owner().idx();
                let t = self.cur[self.idx].0;
                self.grid[p][r as usize][c as usize] = t.t[0];
                self.crown[p][r as usize][c as usize] = t.c[0];
                self.grid[p][r2 as usize][c2 as usize] = t.t[1];
                self.crown[p][r2 as usize][c2 as usize] = t.c[1];
                self.placed[p] += 1;
                if self.next.is_empty() {
                    self.advance_after_turn();
                } else {
                    self.phase = Phase::Pick;
                }
            }
            (Phase::Place, Move::Discard) => {
                let p = self.cur_owner().idx();
                self.discards[p] += 1;
                if self.next.is_empty() {
                    self.advance_after_turn();
                } else {
                    self.phase = Phase::Pick;
                }
            }
            (Phase::Pick, Move::Pick(i)) => {
                let me = self.cur_owner();
                self.next[i as usize].1 = Some(me);
                self.advance_after_turn();
            }
            _ => {}
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Done {
            return None;
        }
        let (a, b) = (self.score(0), self.score(1));
        if a != b {
            return Some(Outcome::Win(if a > b { Player::One } else { Player::Two }));
        }
        let (ta, tb) = (self.tiebreak(0), self.tiebreak(1));
        if ta != tb {
            return Some(Outcome::Win(if ta > tb {
                Player::One
            } else {
                Player::Two
            }));
        }
        Some(Outcome::Draw)
    }

    fn evaluate(&self, p: Player) -> i32 {
        (self.heuristic(p.idx()) - self.heuristic(p.other().idx())) as i32
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(2)
    }

    fn render(&self) -> String {
        let mut s = String::from("Ținuturi\n");
        let tile_txt = |t: &Tile| {
            let h = |k: usize| {
                if t.c[k] > 0 {
                    format!("{}{}", LETTERS[t.t[k] as usize], t.c[k])
                } else {
                    format!("{}", LETTERS[t.t[k] as usize])
                }
            };
            format!("#{}[{}|{}]", t.num, h(0), h(1))
        };
        let own = |o: &Option<Player>| match o {
            Some(Player::One) => " (J1)",
            Some(Player::Two) => " (J2)",
            None => "",
        };
        s.push_str("Rând curent: ");
        s.push_str(
            &self
                .cur
                .iter()
                .enumerate()
                .map(|(i, (t, o))| {
                    let mark = if matches!(self.phase, Phase::Place | Phase::Pick) && i == self.idx
                    {
                        "▶"
                    } else {
                        ""
                    };
                    format!("{}{}:{}{}", mark, i + 1, tile_txt(t), own(o))
                })
                .collect::<Vec<_>>()
                .join("  "),
        );
        s.push('\n');
        if !self.next.is_empty() {
            s.push_str("Rând următor: ");
            s.push_str(
                &self
                    .next
                    .iter()
                    .enumerate()
                    .map(|(i, (t, o))| format!("{}:{}{}", i + 1, tile_txt(t), own(o)))
                    .collect::<Vec<_>>()
                    .join("  "),
            );
            s.push('\n');
        }
        s.push_str(&format!("În pachet: {}\n", self.deck.len()));
        for p in 0..2 {
            s.push_str(&format!(
                "J{}: scor {} (bază {}) | așezate {} aruncate {}\n",
                p + 1,
                self.score(p),
                self.base_score(p),
                self.placed[p],
                self.discards[p]
            ));
            let (r0, r1, c0, c1) = self.bbox(p);
            let (r0, r1) = (r0.saturating_sub(1), (r1 + 1).min(N - 1));
            let (c0, c1) = (c0.saturating_sub(1), (c1 + 1).min(N - 1));
            s.push_str("   ");
            for c in c0..=c1 {
                s.push_str(&format!(" {} ", (b'a' + c as u8) as char));
            }
            s.push('\n');
            for r in r0..=r1 {
                s.push_str(&format!("{:>2} ", r + 1));
                for c in c0..=c1 {
                    let g = self.grid[p][r][c];
                    let cell = if g == CASTLE {
                        "## ".to_string()
                    } else if g == EMPTY {
                        "·· ".to_string()
                    } else {
                        let k = self.crown[p][r][c];
                        format!(
                            "{}{} ",
                            LETTERS[g as usize],
                            if k > 0 { k.to_string() } else { "-".into() }
                        )
                    };
                    s.push_str(&cell);
                }
                s.push('\n');
            }
        }
        match self.phase {
            Phase::Done => s.push_str("Joc încheiat.\n"),
            Phase::Reveal => s.push_str("Se dezvăluie rândul următor (revelare).\n"),
            Phase::Open(_) => s.push_str(&format!(
                "Mută J{}: alege o piesă din rândul curent (alege N).\n",
                self.to_move().idx() + 1
            )),
            Phase::Place => s.push_str(&format!(
                "Mută J{}: așază piesa ▶ (pune e6-e7).\n",
                self.to_move().idx() + 1
            )),
            Phase::Pick => s.push_str(&format!(
                "Mută J{}: alege din rândul următor (alege N).\n",
                self.to_move().idx() + 1
            )),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match *m {
            Move::Pick(i) => format!("alege {}", i + 1),
            Move::Place(r, c, r2, c2) => format!(
                "pune {}-{}",
                fmt_cell(r as usize, c as usize),
                fmt_cell(r2 as usize, c2 as usize)
            ),
            Move::Discard => "descarta".into(),
            Move::Reveal => "revelare".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn play_random(seed: u64) -> Tinuturi {
        let mut g = Tinuturi::with_seed(seed);
        let mut rng = Rng::new(seed ^ 77);
        let mut n = 0;
        while g.outcome().is_none() {
            let m = g.legal_moves();
            assert!(!m.is_empty());
            g.apply(&m[rng.below(m.len())]);
            n += 1;
            assert!(n < 500);
        }
        g
    }

    #[test]
    fn tile_set_is_sane() {
        let t = all_tiles();
        assert_eq!(t.len(), 48);
        let mut cnt = [0; 6];
        for x in &t {
            cnt[x.t[0] as usize] += 1;
            cnt[x.t[1] as usize] += 1;
        }
        assert_eq!(cnt, [22, 20, 18, 14, 12, 10]);
        assert!(t.iter().map(|x| x.c[0] + x.c[1]).sum::<u8>() > 30);
    }

    #[test]
    fn region_scoring() {
        let mut g = Tinuturi::with_seed(1);
        // 3 pătrate de Câmp (2 coroane), 1 de Pădure (1 coroană)
        let cells = [(4, 5, 0, 1), (4, 6, 0, 1), (5, 5, 0, 0), (3, 4, 1, 1)];
        for (r, c, t, k) in cells {
            g.grid[0][r][c] = t;
            g.crown[0][r][c] = k;
        }
        assert_eq!(g.base_score(0), 3 * 2 + 1);
    }

    #[test]
    fn placement_rules() {
        let mut g = Tinuturi::with_seed(2);
        g.apply(&Move::Pick(0));
        g.apply(&Move::Pick(0));
        g.apply(&Move::Pick(0));
        g.apply(&Move::Pick(0));
        g.apply(&Move::Reveal);
        let t = g.cur[0].0;
        let p = g.cur_owner().idx();
        for m in g.legal_moves() {
            if let Move::Place(r, c, r2, c2) = m {
                // vecin de castel sau de același teritoriu: aici, doar castelul există
                let near = |rr: u8, cc: u8| (rr as i32 - 4).abs() + (cc as i32 - 4).abs() == 1;
                assert!(near(r, c) || near(r2, c2));
                assert!(g.grid[p][r as usize][c as usize] == EMPTY);
            }
        }
        assert!(!g.legal_moves().is_empty());
        let _ = t;
    }

    #[test]
    fn bounding_box_seven() {
        let mut g = Tinuturi::with_seed(3);
        // un rând de teritorii pe rândul 5 de la a5 la g5 (castelul e e5): lățime 7
        for c in 0..7 {
            if c != 4 {
                g.grid[0][4][c] = 0;
            }
        }
        let tile = Tile {
            num: 1,
            t: [0, 0],
            c: [0, 0],
        };
        for m in g.placements(0, &tile) {
            if let Move::Place(_, c, _, c2) = m {
                assert!((0..7).contains(&(c as usize)) && (0..7).contains(&(c2 as usize)));
            }
        }
    }

    #[test]
    fn every_tile_is_accounted_for() {
        for seed in 1..30 {
            let g = play_random(seed);
            for p in 0..2 {
                assert_eq!(g.placed[p] + g.discards[p], 12, "seed {seed}");
            }
            assert!(g.deck.is_empty());
        }
    }

    #[test]
    fn center_bonus() {
        let mut g = Tinuturi::with_seed(4);
        assert_eq!(g.bonuses(0), 15);
        g.discards[0] = 1;
        assert_eq!(g.bonuses(0), 10);
        g.grid[0][4][5] = 0;
        assert_eq!(g.bonuses(0), 0);
    }

    #[test]
    fn ai_beats_random() {
        let mut wins = 0;
        for seed in 1..=20u64 {
            let mut g = Tinuturi::with_seed(seed);
            let mut rng = Rng::new(seed + 3);
            while g.outcome().is_none() {
                let m = if g.to_move() == Player::One {
                    polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap()
                } else {
                    let l = g.legal_moves();
                    l[rng.below(l.len())]
                };
                g.apply(&m);
            }
            if g.outcome() == Some(Outcome::Win(Player::One)) {
                wins += 1;
            }
        }
        assert!(wins >= 17, "wins {wins}");
    }
}
