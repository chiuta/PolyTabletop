//! Dara (Nigeria): 5 rânduri × 6 coloane, 12 piese pe jucător.
//! Faza 1: plasare alternativă (fără capturi). Faza 2: mutări ortogonale cu
//! un pas; o moară de EXACT trei piese (rând sau coloană; patru în linie nu
//! contează) formată prin mutare scoate o piesă adversă oarecare.
//! Pierde cine rămâne cu mai puțin de 3 piese sau nu mai poate muta.
//! Remiză după 100 de mutări consecutive fără captură.

//! Verificare surse (nov. 2026: en.wikipedia.org/wiki/Dara_(game), jpneto.github.io/
//! world_abstract_games/dara.htm, bead.game, whatdowedoallday.com): tabla 5×6, 12 piese,
//! plasare fără capturi, mutări ortogonale, doar linii de EXACT trei, pierde cine rămâne
//! cu <3 piese corespund. DUBIU: Wikipedia și alte surse interzic capturarea unei piese
//! aflate într-o linie de trei a adversarului, bead.game permite orice piesă; codul permite
//! orice piesă (varianta bead.game). Draw după 100 de mutări fără captură e o regulă proprie.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const W: usize = 6;
const H: usize = 5;
const N: usize = W * H;
const PIECES: u8 = 12;

fn name(i: usize) -> String {
    format!("{}{}", (b'a' + (i % W) as u8) as char, i / W + 1)
}

fn neighbours(i: usize) -> impl Iterator<Item = usize> {
    let (r, c) = (i / W, i % W);
    let mut v = [usize::MAX; 4];
    if r > 0 {
        v[0] = i - W;
    }
    if r + 1 < H {
        v[1] = i + W;
    }
    if c > 0 {
        v[2] = i - 1;
    }
    if c + 1 < W {
        v[3] = i + 1;
    }
    v.into_iter().filter(|&x| x != usize::MAX)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Mv {
    pub from: Option<u8>,
    pub to: u8,
    pub take: Option<u8>,
}

#[derive(Clone)]
pub struct Dara {
    b: [u8; N],
    hand: [u8; 2],
    side: Player,
    quiet: u16,
}

impl Default for Dara {
    fn default() -> Self {
        Self::new()
    }
}

impl Dara {
    pub fn new() -> Self {
        Dara {
            b: [0; N],
            hand: [PIECES, PIECES],
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

    /// True if the piece at `i` is in a line of exactly three (row or column).
    fn exact_mill(b: &[u8; N], i: usize) -> bool {
        let c = b[i];
        if c == 0 {
            return false;
        }
        let (r, col) = (i / W, i % W);
        let mut run = 1;
        let mut k = col;
        while k > 0 && b[r * W + k - 1] == c {
            k -= 1;
            run += 1;
        }
        k = col;
        while k + 1 < W && b[r * W + k + 1] == c {
            k += 1;
            run += 1;
        }
        if run == 3 {
            return true;
        }
        run = 1;
        let mut j = r;
        while j > 0 && b[(j - 1) * W + col] == c {
            j -= 1;
            run += 1;
        }
        j = r;
        while j + 1 < H && b[(j + 1) * W + col] == c {
            j += 1;
            run += 1;
        }
        run == 3
    }

    fn placing(&self) -> bool {
        self.hand[self.side.idx()] > 0
    }

    fn gen(&self) -> Vec<Mv> {
        let me = self.side.idx() as u8 + 1;
        let opp = 3 - me;
        let mut out = Vec::new();
        if self.placing() {
            for i in 0..N {
                if self.b[i] == 0 {
                    out.push(Mv {
                        from: None,
                        to: i as u8,
                        take: None,
                    });
                }
            }
            return out;
        }
        for from in 0..N {
            if self.b[from] != me {
                continue;
            }
            for to in neighbours(from) {
                if self.b[to] != 0 {
                    continue;
                }
                let mut nb = self.b;
                nb[from] = 0;
                nb[to] = me;
                if Self::exact_mill(&nb, to) {
                    for t in 0..N {
                        if self.b[t] == opp {
                            out.push(Mv {
                                from: Some(from as u8),
                                to: to as u8,
                                take: Some(t as u8),
                            });
                        }
                    }
                } else {
                    out.push(Mv {
                        from: Some(from as u8),
                        to: to as u8,
                        take: None,
                    });
                }
            }
        }
        out
    }

    fn can_move(&self, color: u8) -> bool {
        (0..N).any(|i| self.b[i] == color && neighbours(i).any(|j| self.b[j] == 0))
    }

    fn mobility(&self, color: u8) -> i32 {
        (0..N)
            .filter(|&i| self.b[i] == color)
            .map(|i| neighbours(i).filter(|&j| self.b[j] == 0).count() as i32)
            .sum()
    }

    fn terminal(&self) -> Option<Outcome> {
        for p in [Player::One, Player::Two] {
            if self.total(p) < 3 {
                return Some(Outcome::Win(p.other()));
            }
        }
        if self.hand == [0, 0] && self.quiet >= 100 {
            return Some(Outcome::Draw);
        }
        if !self.placing() && !self.can_move(self.side.idx() as u8 + 1) {
            return Some(Outcome::Win(self.side.other()));
        }
        None
    }
}

impl Game for Dara {
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
        let pieces = self.total(p) as i32 - self.total(p.other()) as i32;
        let mob = self.mobility(me) - self.mobility(3 - me);
        100 * pieces + 3 * mob
    }

    fn render(&self) -> String {
        let mut s = String::from("   a b c d e f\n");
        for r in (0..H).rev() {
            s.push_str(&format!("{}  ", r + 1));
            for c in 0..W {
                s.push(match self.b[r * W + c] {
                    1 => 'X',
                    2 => 'O',
                    _ => '.',
                });
                s.push(' ');
            }
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
            Some(f) => format!("{}-{}", name(f as usize), name(m.to as usize)),
            None => name(m.to as usize),
        };
        if let Some(t) = m.take {
            s.push('x');
            s.push_str(&name(t as usize));
        }
        s
    }

    fn ai_style(&self) -> AiStyle {
        if self.hand[0] + self.hand[1] > 0 {
            AiStyle::AlphaBeta(3)
        } else {
            AiStyle::AlphaBeta(4)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    fn empty_moving() -> Dara {
        Dara {
            b: [0; N],
            hand: [0, 0],
            side: Player::One,
            quiet: 0,
        }
    }

    fn at(s: &str) -> usize {
        let b = s.as_bytes();
        (b[1] - b'1') as usize * W + (b[0] - b'a') as usize
    }

    #[test]
    fn start_has_30_placements() {
        let g = Dara::new();
        assert_eq!(g.legal_moves().len(), 30);
        assert_eq!(g.format_move(&g.legal_moves()[0]), "a1");
    }

    #[test]
    fn placement_never_captures() {
        let mut g = Dara::new();
        for s in ["a1", "a5", "b1", "b5", "c1"] {
            assert!(polytabletop_core::GameObj::play(&mut g, s).is_ok());
        }
        // a1 b1 c1 form three in a row during placement: nothing removed.
        assert_eq!(g.on_board(1), 3);
        assert_eq!(g.on_board(2), 2);
    }

    #[test]
    fn exact_three_is_a_mill_four_is_not() {
        let mut g = empty_moving();
        for s in ["a1", "b1", "c2", "f5"] {
            g.b[at(s)] = 1;
        }
        g.b[at("e5")] = 2;
        g.b[at("e1")] = 2;
        g.b[at("f1")] = 2;
        let txt: Vec<String> = g.legal_moves().iter().map(|m| g.format_move(m)).collect();
        assert!(txt.contains(&"c2-c1xe5".to_string()), "{txt:?}");
        assert!(!txt.contains(&"c2-c1".to_string()));
        // Four in a row (a1 b1 c1 d1) is not a mill.
        let mut h = empty_moving();
        for s in ["a1", "b1", "d1", "c2", "f5"] {
            h.b[at(s)] = 1;
        }
        h.b[at("e5")] = 2;
        h.b[at("e1")] = 2;
        h.b[at("f1")] = 2;
        let txt: Vec<String> = h.legal_moves().iter().map(|m| h.format_move(m)).collect();
        assert!(txt.contains(&"c2-c1".to_string()), "{txt:?}");
    }

    #[test]
    fn vertical_mill_and_capture_applies() {
        let mut g = empty_moving();
        for s in ["a1", "a2", "b3", "e5"] {
            g.b[at(s)] = 1;
        }
        for s in ["f1", "f2", "f3"] {
            g.b[at(s)] = 2;
        }
        let mv = g
            .legal_moves()
            .into_iter()
            .find(|m| g.format_move(m) == "b3-a3xf1")
            .unwrap();
        g.apply(&mv);
        assert_eq!(g.on_board(2), 2);
        // Opponent reduced to 2 pieces? Not yet: 3 -> 2 means loss.
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn blocked_player_loses() {
        let mut g = empty_moving();
        g.b[at("a1")] = 1;
        g.b[at("b1")] = 2;
        g.b[at("a2")] = 2;
        g.b[at("f5")] = 1;
        g.b[at("f4")] = 1;
        g.b[at("e5")] = 2;
        g.b[at("e4")] = 2;
        g.b[at("d4")] = 2;
        // One's a1 is blocked but f5/f4 can still move: no loss yet.
        assert!(g.outcome().is_none());
        g.b[at("f3")] = 2;
        g.b[at("e3")] = 2;
        g.b[at("c3")] = 2;
        // f5: e5 (O), f4 (X). f4: e4 (O), f3 (O), f5 (X): all blocked.
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
    }

    #[test]
    fn random_games_terminate_and_conserve() {
        for seed in 1..=5u64 {
            let mut g = Dara::new();
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
    fn ai_is_fast() {
        let mut g = Dara::new();
        let mut rng = Rng::new(3);
        for _ in 0..40 {
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
