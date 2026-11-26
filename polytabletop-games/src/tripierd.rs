//! Trei-pierde: joc de aliniere pe tablă hexagonală (hexagon cu latura 5, 61 de
//! celule). Pe rând, fiecare pune o piesă pe o celulă goală. Patru (sau mai
//! multe) în linie dreaptă câștigă; exact trei în linie (fără să fie și patru)
//! pierde. Tabla plină fără învingător = remiză. Reguli publice, implementare
//! originală.
//! Mutări: celula, de ex. `e5` (litera = rândul de sus în jos a–i, cifra = poziția în rând).

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const R: i32 = 4; // raza
const SIDE: usize = 9;
const AX: [(i32, i32); 3] = [(1, 0), (0, 1), (1, -1)];

/// Celulele (q, r) ale tablei, rând cu rând.
fn cells() -> &'static [(i32, i32)] {
    static C: std::sync::OnceLock<Vec<(i32, i32)>> = std::sync::OnceLock::new();
    C.get_or_init(build_cells)
}

fn build_cells() -> Vec<(i32, i32)> {
    let mut v = Vec::new();
    for r in -R..=R {
        for q in -R..=R {
            if (q + r).abs() <= R {
                v.push((q, r));
            }
        }
    }
    v
}

const NC: usize = 61;

fn index_of(q: i32, r: i32) -> Option<usize> {
    if q.abs() > R || r.abs() > R || (q + r).abs() > R {
        return None;
    }
    // position in cells() order
    let mut n = 0usize;
    for rr in -R..r {
        n += (SIDE as i32 - rr.abs()) as usize;
    }
    let qmin = (-R).max(-R - r);
    Some(n + (q - qmin) as usize)
}

fn name(i: usize) -> String {
    let (q, r) = cells()[i];
    let qmin = (-R).max(-R - r);
    format!("{}{}", (b'a' + (r + R) as u8) as char, q - qmin + 1)
}

#[derive(Clone)]
pub struct TriPierd {
    b: [u8; NC],
    side: Player,
    count: u8,
    result: Option<Outcome>,
    last: Option<u8>,
}

impl Default for TriPierd {
    fn default() -> Self {
        Self::new()
    }
}

impl TriPierd {
    pub fn new() -> Self {
        TriPierd {
            b: [0; NC],
            side: Player::One,
            count: 0,
            result: None,
            last: None,
        }
    }

    fn cell(&self, q: i32, r: i32) -> u8 {
        match index_of(q, r) {
            Some(i) => self.b[i],
            None => 0xFF,
        }
    }

    /// Length of the run of colour `c` through (q, r) along `d`.
    fn run(&self, q: i32, r: i32, d: (i32, i32), c: u8) -> i32 {
        let mut n = 1;
        for s in [1, -1] {
            let (mut x, mut y) = (q + d.0 * s, r + d.1 * s);
            while self.cell(x, y) == c {
                n += 1;
                x += d.0 * s;
                y += d.1 * s;
            }
        }
        n
    }
}

impl Game for TriPierd {
    type Move = u8;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<u8> {
        if self.result.is_some() {
            return Vec::new();
        }
        // Centre-first ordering.
        let mut v: Vec<u8> = (0..NC as u8).filter(|&i| self.b[i as usize] == 0).collect();
        let cs = cells();
        v.sort_by_key(|&i| {
            let (q, r) = cs[i as usize];
            (q.abs() + r.abs() + (q + r).abs()) / 2
        });
        v
    }

    fn apply(&mut self, m: &u8) {
        let (q, r) = cells()[*m as usize];
        let c = self.side.idx() as u8 + 1;
        self.b[*m as usize] = c;
        self.count += 1;
        self.last = Some(*m);
        let mut four = false;
        let mut three = false;
        for d in AX {
            let n = self.run(q, r, d, c);
            if n >= 4 {
                four = true;
            } else if n == 3 {
                three = true;
            }
        }
        if four {
            self.result = Some(Outcome::Win(self.side));
        } else if three {
            self.result = Some(Outcome::Win(self.side.other()));
        } else if self.count as usize == NC {
            self.result = Some(Outcome::Draw);
        }
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        self.result
    }

    fn evaluate(&self, p: Player) -> i32 {
        // Pairs that can still become a four, minus pairs that would force an
        // awkward three; plus centrality.
        let me = p.idx() as u8 + 1;
        let you = 3 - me;
        let mut score = 0;
        for (q, r) in cells() {
            for d in AX {
                let (mut a, mut b, mut ok) = (0, 0, true);
                for k in 0..4 {
                    match self.cell(q + d.0 * k, r + d.1 * k) {
                        0 => {}
                        x if x == me => a += 1,
                        x if x == you => b += 1,
                        _ => {
                            ok = false;
                            break;
                        }
                    }
                }
                if !ok {
                    continue;
                }
                if b == 0 {
                    score += [0, 1, 6, 40][a.min(3)];
                } else if a == 0 {
                    score -= [0, 1, 6, 40][b.min(3)];
                }
            }
        }
        score
    }

    fn render(&self) -> String {
        let mut s = String::new();
        for r in -R..=R {
            let n = SIDE as i32 - r.abs();
            s.push_str(&format!("{} ", (b'a' + (r + R) as u8) as char));
            s.push_str(&" ".repeat(r.unsigned_abs() as usize));
            let qmin = (-R).max(-R - r);
            for k in 0..n {
                let i = index_of(qmin + k, r).unwrap();
                s.push(match self.b[i] {
                    1 => 'X',
                    2 => 'O',
                    _ => '.',
                });
                s.push(' ');
            }
            s.push('\n');
        }
        s.push_str("Patru în linie câștigă, exact trei în linie pierde. Celule: a1..i5.\n");
        s
    }

    fn format_move(&self, m: &u8) -> String {
        name(*m as usize)
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 500,
            max_depth: 10,
        }
    }

    fn hash_key(&self) -> Option<u64> {
        let mut h = polytabletop_core::hash_mix(3 << 40 | (self.side.idx() as u64) << 32);
        for (i, &v) in self.b.iter().enumerate() {
            if v != 0 {
                h ^= polytabletop_core::hash_mix((v as u64) << 16 | i as u64);
            }
        }
        Some(h)
    }

    fn move_key(&self, m: &u8) -> Option<u32> {
        Some(*m as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    fn idx(s: &str) -> u8 {
        (0..NC).find(|&i| name(i) == s).unwrap() as u8
    }

    #[test]
    fn geometry() {
        assert_eq!(cells().len(), NC);
        for (i, &(q, r)) in cells().iter().enumerate() {
            assert_eq!(index_of(q, r), Some(i));
        }
        assert_eq!(name(0), "a1");
        assert_eq!(name(NC - 1), "i5");
        assert_eq!(TriPierd::new().legal_moves().len(), 61);
    }

    #[test]
    fn four_wins_three_loses() {
        // Row e (r = 0): e1..e9. One builds e1 e2 e3: third stone loses.
        let mut g = TriPierd::new();
        for (a, b) in [("e1", "a1"), ("e2", "a2")] {
            g.apply(&idx(a));
            g.apply(&idx(b));
        }
        g.apply(&idx("e3"));
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
        // Four in a row wins.
        let mut g = TriPierd::new();
        for (a, b) in [("e1", "a1"), ("e2", "a2"), ("e4", "a4")] {
            g.apply(&idx(a));
            g.apply(&idx(b));
        }
        g.apply(&idx("e3"));
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn four_beats_simultaneous_three_and_five_wins() {
        // e1 e2 _ e4 e5 e6 ... filling e3 makes 5 in a row = win (not a loss).
        let mut g = TriPierd::new();
        for (a, b) in [("e1", "a1"), ("e2", "a2"), ("e4", "a4"), ("e5", "b1")] {
            g.apply(&idx(a));
            g.apply(&idx(b));
        }
        // e1 e2 _ e4 e5: still no run of 3. Placing e3 gives 5.
        g.apply(&idx("e3"));
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn diagonal_lines_exist() {
        // Direction (0,1): fixed q, increasing r. Cells (0,-2)..(0,1).
        let mut g = TriPierd::new();
        let line = [(0, -2), (0, -1), (0, 1), (0, 0)];
        let others = [(-4, 0), (-2, -2), (3, -3)];
        for k in 0..3 {
            g.apply(&(index_of(line[k].0, line[k].1).unwrap() as u8));
            g.apply(&(index_of(others[k].0, others[k].1).unwrap() as u8));
        }
        g.apply(&(index_of(0, 0).unwrap() as u8));
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=50u64 {
            let mut g = TriPierd::new();
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n <= NC);
            }
        }
    }
}
