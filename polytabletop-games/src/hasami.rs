//! Hasami Shogi (Japonia): tablă 9×9, câte 9 piese pe jucător, pe rândul din spate.
//! Orice piesă se mută ca un turn (orice număr de pătrate libere, pe orizontală sau
//! verticală, fără sărituri). Captură prin încadrare: dacă mutarea ta pune piesa într-o
//! parte a unui șir (unul sau mai multe) de piese adverse consecutive, pe un rând sau o
//! coloană, iar la celălalt capăt e o piesă a ta, tot șirul se scoate. Ai voie să
//! intri singur între două piese adverse (nu ești capturat). Câștigă cine lasă
//! adversarul cu cel mult o piesă; cine nu mai poate muta pierde; remiză după 100
//! de semi-mutări fără captură (regulă proprie).
//!
//! Verificare surse: en.wikipedia.org/wiki/Hasami_shogi (varianta cu 9 piese, captura
//! custodială, victoria prin reducerea adversarului la o piesă). Captura piesei din colț
//! prin două vecini nu este implementată. Implementare proprie.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const S: usize = 9;
const N: usize = S * S;
const QUIET_LIMIT: u16 = 100;

const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn name(i: usize) -> String {
    polytabletop_core::square_name(i % S, i / S)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Mv {
    pub from: u8,
    pub to: u8,
}

#[derive(Clone)]
pub struct Hasami {
    b: [u8; N],
    side: Player,
    quiet: u16,
}

impl Default for Hasami {
    fn default() -> Self {
        Self::new()
    }
}

fn walk(i: usize, d: usize) -> Option<usize> {
    let (r, c) = ((i / S) as i32 + DIRS[d].0, (i % S) as i32 + DIRS[d].1);
    if r < 0 || c < 0 || r >= S as i32 || c >= S as i32 {
        None
    } else {
        Some(r as usize * S + c as usize)
    }
}

impl Hasami {
    pub fn new() -> Self {
        let mut b = [0u8; N];
        for c in 0..S {
            b[c] = 1;
            b[(S - 1) * S + c] = 2;
        }
        Hasami {
            b,
            side: Player::One,
            quiet: 0,
        }
    }

    fn count(&self, color: u8) -> usize {
        self.b.iter().filter(|&&c| c == color).count()
    }

    /// Squares captured if `color` lands on `to` (board already holds the piece there).
    fn captured_by(b: &[u8; N], to: usize, color: u8) -> Vec<usize> {
        let opp = 3 - color;
        let mut out = Vec::new();
        for d in 0..4 {
            let mut run = Vec::new();
            let mut cur = walk(to, d);
            while let Some(s) = cur {
                if b[s] == opp {
                    run.push(s);
                    cur = walk(s, d);
                } else {
                    if b[s] == color && !run.is_empty() {
                        out.extend(run.iter().copied());
                    }
                    break;
                }
            }
        }
        out
    }

    fn gen_for(&self, color: u8) -> Vec<Mv> {
        let mut out = Vec::new();
        for from in 0..N {
            if self.b[from] != color {
                continue;
            }
            for d in 0..4 {
                let mut cur = walk(from, d);
                while let Some(t) = cur {
                    if self.b[t] != 0 {
                        break;
                    }
                    out.push(Mv {
                        from: from as u8,
                        to: t as u8,
                    });
                    cur = walk(t, d);
                }
            }
        }
        out
    }

    fn mover_has_move(&self, color: u8) -> bool {
        (0..N).any(|i| {
            self.b[i] == color && (0..4).any(|d| walk(i, d).is_some_and(|t| self.b[t] == 0))
        })
    }

    fn captures_of(&self, m: &Mv) -> usize {
        let me = self.side.idx() as u8 + 1;
        let mut nb = self.b;
        nb[m.from as usize] = 0;
        nb[m.to as usize] = me;
        Self::captured_by(&nb, m.to as usize, me).len()
    }
}

impl Game for Hasami {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        self.gen_for(self.side.idx() as u8 + 1)
    }

    fn apply(&mut self, m: &Mv) {
        let me = self.side.idx() as u8 + 1;
        self.b[m.from as usize] = 0;
        self.b[m.to as usize] = me;
        let cap = Self::captured_by(&self.b, m.to as usize, me);
        for &s in &cap {
            self.b[s] = 0;
        }
        self.quiet = if cap.is_empty() { self.quiet + 1 } else { 0 };
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.count(2) <= 1 {
            return Some(Outcome::Win(Player::One));
        }
        if self.count(1) <= 1 {
            return Some(Outcome::Win(Player::Two));
        }
        if self.quiet >= QUIET_LIMIT {
            return Some(Outcome::Draw);
        }
        if !self.mover_has_move(self.side.idx() as u8 + 1) {
            return Some(Outcome::Win(self.side.other()));
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx() as u8 + 1;
        let mat = self.count(me) as i32 - self.count(3 - me) as i32;
        // Threat count: moves of the side that would capture something.
        let threats = |c: u8| -> i32 {
            let mut g = self.clone();
            g.side = if c == 1 { Player::One } else { Player::Two };
            g.gen_for(c).iter().filter(|m| g.captures_of(m) > 0).count() as i32
        };
        let t = threats(me) - threats(3 - me);
        // Mobility-ish: pieces off the back rank are more active.
        100 * mat + 8 * t
    }

    fn render(&self) -> String {
        let mut s = String::from("   a b c d e f g h i\n");
        for r in (0..S).rev() {
            s.push_str(&format!("{}  ", r + 1));
            for c in 0..S {
                s.push(match self.b[r * S + c] {
                    1 => 'X',
                    2 => 'O',
                    _ => '.',
                });
                s.push(' ');
            }
            s.push('\n');
        }
        s.push_str(&format!(
            "  X: {}  O: {}   (captură prin încadrare; câștigi lăsând adversarul cu o singură piesă)\n",
            self.count(1),
            self.count(2)
        ));
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        format!("{}-{}", name(m.from as usize), name(m.to as usize))
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 500,
            max_depth: 6,
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
        10 * self.captures_of(m) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    fn at(s: &str) -> usize {
        let b = s.as_bytes();
        (b[1] - b'1') as usize * S + (b[0] - b'a') as usize
    }

    fn empty() -> Hasami {
        Hasami {
            b: [0; N],
            side: Player::One,
            quiet: 0,
        }
    }

    fn play(g: &mut Hasami, s: &str) {
        let m = g
            .legal_moves()
            .into_iter()
            .find(|m| g.format_move(m) == s)
            .unwrap_or_else(|| panic!("illegal {s}"));
        g.apply(&m);
    }

    #[test]
    fn start_moves() {
        let g = Hasami::new();
        // Each of 9 pieces moves up 1..7 squares.
        assert_eq!(g.legal_moves().len(), 9 * 7);
        assert_eq!(g.count(1), 9);
        assert_eq!(g.count(2), 9);
    }

    #[test]
    fn rook_moves_do_not_jump() {
        let mut g = empty();
        g.b[at("a1")] = 1;
        g.b[at("a4")] = 2;
        g.b[at("i9")] = 2;
        g.b[at("i8")] = 2;
        g.b[at("h1")] = 1;
        let t: Vec<String> = g.legal_moves().iter().map(|m| g.format_move(m)).collect();
        assert!(t.contains(&"a1-a3".to_string()));
        assert!(!t.contains(&"a1-a4".to_string()));
        assert!(!t.contains(&"a1-a5".to_string()));
    }

    #[test]
    fn sandwich_captures_single_and_multiple() {
        let mut g = empty();
        g.b[at("a5")] = 1;
        g.b[at("b5")] = 2;
        g.b[at("c5")] = 2;
        g.b[at("e5")] = 1; // mover
        g.b[at("i9")] = 2;
        g.b[at("h9")] = 2;
        play(&mut g, "e5-d5");
        assert_eq!(g.b[at("b5")], 0);
        assert_eq!(g.b[at("c5")], 0);
        assert_eq!(g.count(2), 2);
        assert_eq!(g.quiet, 0);
    }

    #[test]
    fn vertical_capture() {
        let mut g = empty();
        g.b[at("c1")] = 1;
        g.b[at("c2")] = 2;
        g.b[at("a3")] = 1;
        g.b[at("h9")] = 2;
        g.b[at("i9")] = 2;
        play(&mut g, "a3-c3");
        assert_eq!(g.b[at("c2")], 0);
    }

    #[test]
    fn moving_between_two_enemies_is_safe() {
        let mut g = empty();
        g.b[at("c3")] = 2;
        g.b[at("e3")] = 2;
        g.b[at("d1")] = 1;
        g.b[at("h9")] = 1;
        g.b[at("i9")] = 2;
        g.b[at("a9")] = 2;
        play(&mut g, "d1-d3");
        assert_eq!(g.b[at("d3")], 1);
        assert_eq!(g.count(1), 2);
        // Enemy's pieces are not captured by merely standing next to them either.
        assert_eq!(g.count(2), 4);
    }

    #[test]
    fn no_capture_against_the_edge() {
        let mut g = empty();
        g.b[at("a5")] = 2;
        g.b[at("b5")] = 1;
        g.b[at("h9")] = 1;
        g.b[at("i9")] = 2;
        g.b[at("i8")] = 2;
        play(&mut g, "b5-b6");
        assert_eq!(g.b[at("a5")], 2);
    }

    #[test]
    fn win_when_opponent_has_one_piece_left() {
        let mut g = empty();
        g.b[at("a5")] = 1;
        g.b[at("b5")] = 2;
        g.b[at("e5")] = 1;
        g.b[at("i9")] = 2;
        assert_eq!(g.outcome(), None);
        play(&mut g, "e5-c5");
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn random_games_terminate() {
        for seed in 1..=8u64 {
            let mut g = Hasami::new();
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() && n < 4000 {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                n += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed}");
        }
    }
}
