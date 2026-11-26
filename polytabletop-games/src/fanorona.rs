//! Fanorona (Madagascar): 5 rânduri × 9 coloane de puncte, 22 piese pe jucător.
//! Liniile ortogonale leagă toate punctele; diagonalele leagă punctele „pare”
//! (rând + coloană par). O piesă se mută pe un punct vecin liber. Dacă mutarea
//! continuă în aceeași direcție spre o piesă adversă, o capturează pe ea și toată
//! linia continuă de piese adverse (ABORDARE); dacă piesa se îndepărtează de o piesă
//! adversă aflată în spatele ei, o capturează la fel (RETRAGERE). Captura e
//! obligatorie dacă există; după o captură aceeași piesă poate continua (opțional)
//! cu alte capturi, fără să revină pe un punct deja ocupat în tura respectivă și fără
//! să repete direcția pasului precedent. Câștigă cine scoate toate piesele adverse.
//! Remiză după 40 de semi-mutări fără captură (regulă proprie, simplificată).
//!
//! Verificare surse: descrierea publică a regulilor (en.wikipedia.org/wiki/Fanorona);
//! tabla de pornire (rândul de mijloc X O X O . X O X O) e cea uzuală. Implementare
//! proprie, fără cod preluat.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const W: usize = 9;
const H: usize = 5;
const N: usize = W * H;
const QUIET_LIMIT: u16 = 40;

const DIRS: [(i32, i32); 8] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
];

fn name(i: usize) -> String {
    format!("{}{}", (b'a' + (i % W) as u8) as char, i / W + 1)
}

/// Neighbour of `i` in direction `d` (index into DIRS), if the line exists.
fn step(i: usize, d: usize) -> Option<usize> {
    let (r, c) = ((i / W) as i32, (i % W) as i32);
    let (dr, dc) = DIRS[d];
    if dr != 0 && dc != 0 && (r + c) % 2 != 0 {
        return None;
    }
    let (nr, nc) = (r + dr, c + dc);
    if nr < 0 || nc < 0 || nr >= H as i32 || nc >= W as i32 {
        None
    } else {
        Some(nr as usize * W + nc as usize)
    }
}

fn opposite(d: usize) -> usize {
    d ^ 1
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    /// Mutare simplă, fără captură.
    Plain,
    Approach,
    Withdraw,
}

/// O tură întreagă: piesa pornește din `from` și face unul sau mai mulți pași.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Mv {
    pub from: u8,
    pub steps: Vec<(u8, Kind)>,
}

#[derive(Clone)]
pub struct Fanorona {
    b: [u8; N],
    side: Player,
    quiet: u16,
}

impl Default for Fanorona {
    fn default() -> Self {
        Self::new()
    }
}

impl Fanorona {
    pub fn new() -> Self {
        let mut b = [0u8; N];
        for c in 0..W {
            b[c] = 1;
            b[W + c] = 1;
            b[3 * W + c] = 2;
            b[4 * W + c] = 2;
        }
        let mid = [1u8, 2, 1, 2, 0, 1, 2, 1, 2];
        b[2 * W..3 * W].copy_from_slice(&mid);
        Fanorona {
            b,
            side: Player::One,
            quiet: 0,
        }
    }

    fn count(&self, color: u8) -> usize {
        self.b.iter().filter(|&&c| c == color).count()
    }

    /// Capture step (kind, direction-opposite chain) of piece `from` moving in `d`
    /// on board `b`. Returns the squares that would be captured.
    fn captured(b: &[u8; N], from: usize, d: usize, kind: Kind, me: u8) -> Vec<usize> {
        let opp = 3 - me;
        let to = match step(from, d) {
            Some(t) => t,
            None => return vec![],
        };
        let (mut cur, dir) = match kind {
            Kind::Approach => (step(to, d), d),
            Kind::Withdraw => (step(from, opposite(d)), opposite(d)),
            Kind::Plain => return vec![],
        };
        let mut v = Vec::new();
        while let Some(s) = cur {
            if b[s] != opp {
                break;
            }
            v.push(s);
            cur = step(s, dir);
        }
        v
    }

    #[allow(clippy::too_many_arguments)]
    fn chains(
        b: &[u8; N],
        pos: usize,
        me: u8,
        visited: u64,
        last: Option<usize>,
        path: &mut Vec<(u8, Kind)>,
        start: usize,
        out: &mut Vec<Mv>,
    ) {
        for d in 0..8 {
            if Some(d) == last {
                continue;
            }
            let to = match step(pos, d) {
                Some(t) if b[t] == 0 && visited >> t & 1 == 0 => t,
                _ => continue,
            };
            for kind in [Kind::Approach, Kind::Withdraw] {
                let cap = Self::captured(b, pos, d, kind, me);
                if cap.is_empty() {
                    continue;
                }
                let mut nb = *b;
                for &s in &cap {
                    nb[s] = 0;
                }
                nb[pos] = 0;
                nb[to] = me;
                path.push((to as u8, kind));
                out.push(Mv {
                    from: start as u8,
                    steps: path.clone(),
                });
                Self::chains(&nb, to, me, visited | 1 << to, Some(d), path, start, out);
                path.pop();
            }
        }
    }

    fn gen(&self) -> Vec<Mv> {
        let me = self.side.idx() as u8 + 1;
        let mut caps = Vec::new();
        for from in 0..N {
            if self.b[from] == me {
                let mut path = Vec::new();
                Self::chains(
                    &self.b,
                    from,
                    me,
                    1 << from,
                    None,
                    &mut path,
                    from,
                    &mut caps,
                );
            }
        }
        if !caps.is_empty() {
            return caps;
        }
        let mut out = Vec::new();
        for from in 0..N {
            if self.b[from] != me {
                continue;
            }
            for d in 0..8 {
                if let Some(t) = step(from, d) {
                    if self.b[t] == 0 {
                        out.push(Mv {
                            from: from as u8,
                            steps: vec![(t as u8, Kind::Plain)],
                        });
                    }
                }
            }
        }
        out
    }

    fn dir_between(a: usize, b: usize) -> usize {
        let dr = (b / W) as i32 - (a / W) as i32;
        let dc = (b % W) as i32 - (a % W) as i32;
        DIRS.iter().position(|&x| x == (dr, dc)).unwrap()
    }

    fn can_move(&self, color: u8) -> bool {
        (0..N).any(|i| {
            self.b[i] == color && (0..8).any(|d| step(i, d).is_some_and(|t| self.b[t] == 0))
        })
    }
}

impl Game for Fanorona {
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
        let me = self.side.idx() as u8 + 1;
        let mut pos = m.from as usize;
        let mut captured_any = false;
        for &(to, kind) in &m.steps {
            let to = to as usize;
            let d = Self::dir_between(pos, to);
            for s in Self::captured(&self.b, pos, d, kind, me) {
                self.b[s] = 0;
                captured_any = true;
            }
            self.b[pos] = 0;
            self.b[to] = me;
            pos = to;
        }
        self.quiet = if captured_any { 0 } else { self.quiet + 1 };
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        let (a, b) = (self.count(1), self.count(2));
        if a == 0 {
            return Some(Outcome::Win(Player::Two));
        }
        if b == 0 {
            return Some(Outcome::Win(Player::One));
        }
        if self.quiet >= QUIET_LIMIT {
            return Some(Outcome::Draw);
        }
        if !self.can_move(self.side.idx() as u8 + 1) {
            return Some(Outcome::Win(self.side.other()));
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx() as u8 + 1;
        let (a, b) = (self.count(me) as i32, self.count(3 - me) as i32);
        // Pieces on even points (with diagonals) are slightly more useful.
        let pos: i32 = (0..N)
            .filter(|&i| (i / W + i % W).is_multiple_of(2))
            .map(|i| match self.b[i] {
                c if c == me => 1,
                0 => 0,
                _ => -1,
            })
            .sum();
        100 * (a - b) + 2 * pos
    }

    fn render(&self) -> String {
        let mut s = String::from("   a   b   c   d   e   f   g   h   i\n");
        for r in (0..H).rev() {
            s.push_str(&format!("{}  ", r + 1));
            for c in 0..W {
                s.push(match self.b[r * W + c] {
                    1 => 'X',
                    2 => 'O',
                    _ => '+',
                });
                if c + 1 < W {
                    s.push_str(" - ");
                }
            }
            s.push('\n');
            if r > 0 {
                s.push_str("   ");
                for c in 0..W {
                    s.push('|');
                    if c + 1 < W {
                        // Cell between rows r-1 and r, columns c and c+1.
                        s.push(' ');
                        s.push(if (r - 1 + c) % 2 == 0 { '/' } else { '\\' });
                        s.push(' ');
                    }
                }
                s.push('\n');
            }
        }
        s.push_str(&format!(
            "  X: {}  O: {}   (X începe; a = abordare, w = retragere)\n",
            self.count(1),
            self.count(2)
        ));
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        let mut s = name(m.from as usize);
        for &(to, kind) in &m.steps {
            s.push('-');
            s.push_str(&name(to as usize));
            match kind {
                Kind::Plain => {}
                Kind::Approach => s.push('a'),
                Kind::Withdraw => s.push('w'),
            }
        }
        s
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 500,
            max_depth: 8,
        }
    }

    fn hash_key(&self) -> Option<u64> {
        let mut h = polytabletop_core::hash_mix(
            1 << 40 | (self.side.idx() as u64) << 32 | self.quiet as u64,
        );
        for (i, &v) in self.b.iter().enumerate() {
            if v != 0 {
                h ^= polytabletop_core::hash_mix((v as u64) << 16 | i as u64);
            }
        }
        Some(h)
    }

    fn move_score(&self, m: &Mv) -> i32 {
        m.steps.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    fn at(s: &str) -> usize {
        let b = s.as_bytes();
        (b[1] - b'1') as usize * W + (b[0] - b'a') as usize
    }

    fn empty() -> Fanorona {
        Fanorona {
            b: [0; N],
            side: Player::One,
            quiet: 0,
        }
    }

    fn texts(g: &Fanorona) -> Vec<String> {
        g.legal_moves().iter().map(|m| g.format_move(m)).collect()
    }

    fn play(g: &mut Fanorona, s: &str) {
        let m = g
            .legal_moves()
            .into_iter()
            .find(|m| g.format_move(m) == s)
            .unwrap_or_else(|| panic!("illegal {s}: {:?}", texts(g)));
        g.apply(&m);
    }

    #[test]
    fn start_position() {
        let g = Fanorona::new();
        assert_eq!(g.count(1), 22);
        assert_eq!(g.count(2), 22);
        assert_eq!(g.b[at("e3")], 0);
        // Must capture at once; all opening moves enter the centre point.
        for t in texts(&g) {
            assert!(t.contains("-e3"), "{t}");
        }
        assert!(!texts(&g).is_empty());
    }

    #[test]
    fn approach_captures_whole_line() {
        let mut g = empty();
        g.b[at("a1")] = 1;
        g.b[at("c1")] = 2;
        g.b[at("d1")] = 2;
        g.b[at("e1")] = 2;
        g.b[at("f1")] = 1; // stops the line
        g.b[at("h5")] = 2;
        assert!(texts(&g).contains(&"a1-b1a".to_string()));
        play(&mut g, "a1-b1a");
        assert_eq!(g.b[at("b1")], 1);
        assert_eq!(g.b[at("c1")], 0);
        assert_eq!(g.b[at("d1")], 0);
        assert_eq!(g.b[at("e1")], 0);
        assert_eq!(g.b[at("h5")], 2);
        assert_eq!(g.count(2), 1);
    }

    #[test]
    fn withdrawal_captures_behind() {
        let mut g = empty();
        g.b[at("d3")] = 1;
        g.b[at("c3")] = 2;
        g.b[at("b3")] = 2;
        g.b[at("a3")] = 0;
        g.b[at("i5")] = 2;
        let t = texts(&g);
        assert!(t.contains(&"d3-e3w".to_string()), "{t:?}");
        play(&mut g, "d3-e3w");
        assert_eq!(g.b[at("c3")], 0);
        assert_eq!(g.b[at("b3")], 0);
        assert_eq!(g.count(2), 1);
    }

    #[test]
    fn capture_is_compulsory_and_plain_moves_disappear() {
        let mut g = empty();
        g.b[at("a1")] = 1;
        g.b[at("c1")] = 2;
        g.b[at("i5")] = 1;
        let t = texts(&g);
        assert!(
            t.iter().all(|s| s.contains('a') || s.contains('w')),
            "{t:?}"
        );
        // Without any capture available, plain moves are listed.
        let mut h = empty();
        h.b[at("a1")] = 1;
        h.b[at("i5")] = 2;
        assert!(texts(&h).contains(&"a1-b1".to_string()));
    }

    #[test]
    fn diagonals_exist_only_on_even_points() {
        // b1 (odd point: col 1,row 0 -> sum 1) has no diagonals.
        let mut g = empty();
        g.b[at("b1")] = 1;
        g.b[at("i5")] = 2;
        let t = texts(&g);
        assert!(!t.contains(&"b1-c2".to_string()));
        assert!(!t.contains(&"b1-a2".to_string()));
        assert!(t.contains(&"b1-b2".to_string()));
        // a1 (sum 0) has a diagonal.
        let mut h = empty();
        h.b[at("a1")] = 1;
        h.b[at("i5")] = 2;
        assert!(texts(&h).contains(&"a1-b2".to_string()));
    }

    #[test]
    fn chain_continues_but_not_same_direction_or_revisit() {
        let mut g = empty();
        // X a1 approaches c1 via b1, then from b1 can go b2 withdrawing? Build:
        g.b[at("a1")] = 1;
        g.b[at("c1")] = 2;
        // After a1-b1 (capture c1), from b1 moving up to b2 approaches b3.
        g.b[at("b3")] = 2;
        g.b[at("i5")] = 2;
        let t = texts(&g);
        assert!(t.contains(&"a1-b1a".to_string()));
        assert!(t.contains(&"a1-b1a-b2a".to_string()), "{t:?}");
        // Same direction as previous step is forbidden: b1 -> c1 east again.
        assert!(!t.iter().any(|s| s.starts_with("a1-b1a-c1")));
        // Going back to a1 (visited) is forbidden.
        assert!(!t.iter().any(|s| s.starts_with("a1-b1a-a1")));
        // Chain is optional: the shorter prefix is itself a legal move.
        play(&mut g, "a1-b1a-b2a");
        assert_eq!(g.count(2), 1);
        assert_eq!(g.side, Player::Two);
    }

    #[test]
    fn approach_and_withdrawal_are_distinct_choices() {
        let mut g = empty();
        // d3 can approach f3 (moving east) or withdraw from b3.
        g.b[at("d3")] = 1;
        g.b[at("f3")] = 2;
        g.b[at("c3")] = 2;
        g.b[at("i5")] = 2;
        let t = texts(&g);
        assert!(t.contains(&"d3-e3a".to_string()), "{t:?}");
        assert!(t.contains(&"d3-e3w".to_string()));
    }

    #[test]
    fn win_by_elimination() {
        let mut g = empty();
        g.b[at("a1")] = 1;
        g.b[at("c1")] = 2;
        assert_eq!(g.outcome(), None);
        play(&mut g, "a1-b1a");
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
        assert!(g.legal_moves().is_empty());
    }

    #[test]
    fn random_games_terminate() {
        for seed in 1..=6u64 {
            let mut g = Fanorona::new();
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() && n < 2000 {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                n += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed}");
        }
    }
}
