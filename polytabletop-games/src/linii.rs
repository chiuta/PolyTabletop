//! Linii de acțiune (8×8): cine reușește primul să-și unească toate piesele într-un
//! singur grup (vecinătate pe 8 direcții) câștigă. O piesă se mută în linie dreaptă
//! exact atâtea pătrate câte piese (ale ambilor) sunt pe linia respectivă; poate sări
//! peste piesele proprii, nu peste ale adversarului, și captează aterizând pe o piesă
//! adversă. X (J1) începe; după 300 de mutări jocul e remiză.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: usize = 8;
const AXES: [(i32, i32); 4] = [(0, 1), (1, 0), (1, 1), (1, -1)];
const MAX_PLIES: u16 = 300;

/// Mutare: (de la, la) ca indici 0..64 (rând*8 + coloană).
pub type LoaMove = (u8, u8);

#[derive(Clone)]
pub struct Linii {
    cells: [u8; N * N],
    side: Player,
    plies: u16,
    winner: Option<Player>,
}

impl Default for Linii {
    fn default() -> Self {
        Self::new()
    }
}

fn at(r: i32, c: i32) -> Option<usize> {
    if r >= 0 && r < N as i32 && c >= 0 && c < N as i32 {
        Some(r as usize * N + c as usize)
    } else {
        None
    }
}

impl Linii {
    pub fn new() -> Self {
        let mut cells = [0u8; N * N];
        for k in 1..N - 1 {
            cells[k] = 1;
            cells[(N - 1) * N + k] = 1;
            cells[k * N] = 2;
            cells[k * N + N - 1] = 2;
        }
        Linii {
            cells,
            side: Player::One,
            plies: 0,
            winner: None,
        }
    }

    fn name(i: usize) -> String {
        format!("{}{}", (b'a' + (i % N) as u8) as char, i / N + 1)
    }

    /// Pieces (both colours) on the line through `i` in direction `d`.
    fn line_count(&self, i: usize, d: (i32, i32)) -> i32 {
        let (r, c) = ((i / N) as i32, (i % N) as i32);
        let mut n = 1;
        for sign in [1, -1] {
            let (mut rr, mut cc) = (r + d.0 * sign, c + d.1 * sign);
            while let Some(j) = at(rr, cc) {
                if self.cells[j] != 0 {
                    n += 1;
                }
                rr += d.0 * sign;
                cc += d.1 * sign;
            }
        }
        n
    }

    fn gen(&self, who: u8, first_only: bool) -> Vec<LoaMove> {
        let mut out = Vec::new();
        for i in 0..N * N {
            if self.cells[i] != who {
                continue;
            }
            let (r, c) = ((i / N) as i32, (i % N) as i32);
            for d in AXES {
                let n = self.line_count(i, d);
                for sign in [1, -1] {
                    let Some(to) = at(r + d.0 * sign * n, c + d.1 * sign * n) else {
                        continue;
                    };
                    if self.cells[to] == who {
                        continue;
                    }
                    let blocked = (1..n).any(|k| {
                        at(r + d.0 * sign * k, c + d.1 * sign * k)
                            .is_some_and(|j| self.cells[j] == 3 - who)
                    });
                    if !blocked {
                        out.push((i as u8, to as u8));
                        if first_only {
                            return out;
                        }
                    }
                }
            }
        }
        out
    }

    /// (largest connected group, number of groups) for colour `who`.
    fn groups(&self, who: u8) -> (i32, i32) {
        let mut seen = [false; N * N];
        let (mut best, mut count) = (0, 0);
        for s in 0..N * N {
            if self.cells[s] != who || seen[s] {
                continue;
            }
            count += 1;
            let mut size = 0;
            let mut stack = vec![s];
            seen[s] = true;
            while let Some(i) = stack.pop() {
                size += 1;
                let (r, c) = ((i / N) as i32, (i % N) as i32);
                for dr in -1..=1 {
                    for dc in -1..=1 {
                        if let Some(j) = at(r + dr, c + dc) {
                            if !seen[j] && self.cells[j] == who {
                                seen[j] = true;
                                stack.push(j);
                            }
                        }
                    }
                }
            }
            best = best.max(size);
        }
        (best, count)
    }

    fn is_connected(&self, who: u8) -> bool {
        self.groups(who).1 <= 1
    }

    fn side_score(&self, who: u8) -> i32 {
        const CENTRE: [i32; 4] = [-12, 0, 8, 14];
        let mut s = 0;
        for i in 0..N * N {
            if self.cells[i] == who {
                let (r, c) = (i / N, i % N);
                let ring = r.min(N - 1 - r).min(c).min(N - 1 - c);
                s += CENTRE[ring];
            }
        }
        let (best, count) = self.groups(who);
        s + 8 * best - 25 * (count - 1)
    }
}

impl Game for Linii {
    type Move = LoaMove;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<LoaMove> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        self.gen(self.side.idx() as u8 + 1, false)
    }

    fn apply(&mut self, m: &LoaMove) {
        let who = self.cells[m.0 as usize];
        self.cells[m.0 as usize] = 0;
        self.cells[m.1 as usize] = who;
        self.plies += 1;
        // The mover wins if both sides connect at once.
        if self.is_connected(who) {
            self.winner = Some(self.side);
        } else if self.is_connected(3 - who) {
            self.winner = Some(self.side.other());
        }
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        if let Some(w) = self.winner {
            return Some(Outcome::Win(w));
        }
        if self.plies >= MAX_PLIES {
            return Some(Outcome::Draw);
        }
        if self.gen(self.side.idx() as u8 + 1, true).is_empty() {
            return Some(Outcome::Win(self.side.other()));
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        match self.outcome() {
            Some(Outcome::Win(w)) => return if w == p { 1_000_000 } else { -1_000_000 },
            Some(Outcome::Draw) => return 0,
            None => {}
        }
        let me = p.idx() as u8 + 1;
        self.side_score(me) - self.side_score(3 - me)
    }

    fn render(&self) -> String {
        let hdr = "    a b c d e f g h\n";
        let mut s = String::from(hdr);
        for r in (0..N).rev() {
            s.push_str(&format!("{:>2}  ", r + 1));
            for c in 0..N {
                s.push(match self.cells[r * N + c] {
                    1 => 'X',
                    2 => 'O',
                    _ => '.',
                });
                s.push(' ');
            }
            s.push_str(&format!(" {}\n", r + 1));
        }
        s.push_str(hdr);
        s
    }

    fn format_move(&self, m: &LoaMove) -> String {
        let cap = if self.cells[m.1 as usize] != 0 {
            'x'
        } else {
            '-'
        };
        format!(
            "{}{}{}",
            Self::name(m.0 as usize),
            cap,
            Self::name(m.1 as usize)
        )
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 600,
            max_depth: 6,
        }
    }

    fn playout_cap(&self) -> usize {
        MAX_PLIES as usize
    }

    fn hash_key(&self) -> Option<u64> {
        let mut h = polytabletop_core::hash_mix(1 << 40 | (self.side.idx() as u64) << 32);
        for (i, &v) in self.cells.iter().enumerate() {
            if v != 0 {
                h ^= polytabletop_core::hash_mix((v as u64) << 16 | i as u64);
            }
        }
        Some(h)
    }

    fn move_score(&self, m: &LoaMove) -> i32 {
        let cap = if self.cells[m.1 as usize] != 0 { 20 } else { 0 };
        let (r, c) = (m.1 as usize / N, m.1 as usize % N);
        let ring = r.min(N - 1 - r).min(c).min(N - 1 - c) as i32;
        cap + ring
    }

    fn move_key(&self, m: &LoaMove) -> Option<u32> {
        Some(m.0 as u32 * 64 + m.1 as u32)
    }

    fn is_tactical(&self, m: &LoaMove) -> bool {
        self.cells[m.1 as usize] != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    fn count(g: &Linii, depth: u32) -> u64 {
        if depth == 0 {
            return 1;
        }
        g.legal_moves()
            .iter()
            .map(|m| {
                let mut h = g.clone();
                h.apply(m);
                count(&h, depth - 1)
            })
            .sum()
    }

    #[test]
    fn opening_has_36_moves() {
        let g = Linii::new();
        assert_eq!(g.legal_moves().len(), 36);
        // Regression value of this implementation (blocked lines make it < 36*36).
        assert_eq!(count(&g, 2), 1244);
    }

    #[test]
    fn move_distance_equals_pieces_on_line() {
        let g = Linii::new();
        let names: Vec<String> = g.legal_moves().iter().map(|m| g.format_move(m)).collect();
        // b1: column b holds 2 pieces -> b3; diagonal b1-h7 holds 2 -> d3.
        assert!(names.contains(&"b1-b3".to_string()));
        assert!(names.contains(&"b1-d3".to_string()));
        assert!(!names.contains(&"b1-c2".to_string()));
        assert!(!names.contains(&"b1-b4".to_string()));
    }

    #[test]
    fn cannot_jump_enemy_but_may_jump_own() {
        let mut g = Linii::new();
        g.cells = [0; N * N];
        g.cells[0] = 1; // a1
        g.cells[1] = 1; // b1 (own, jumpable)
        g.cells[2] = 2; // c1 (enemy)
        g.side = Player::One;
        // Row 1 holds 3 pieces: a1 would need to land on d1 passing b1 (own, ok) and c1 (enemy, blocked).
        let ms: Vec<String> = g.legal_moves().iter().map(|m| g.format_move(m)).collect();
        assert!(!ms.contains(&"a1-d1".to_string()));
        // b1 moves 3 to e1 passing c1: blocked too; but b1 jumping left over a1 is off board.
        assert!(!ms.contains(&"b1-e1".to_string()));
    }

    #[test]
    fn capture_and_win_detection() {
        let mut g = Linii::new();
        g.cells = [0; N * N];
        g.cells[0] = 1;
        g.cells[1] = 1;
        g.cells[4 * N + 4] = 1; // e5, far away
        g.cells[7 * N + 7] = 2; // h8
        g.cells[7 * N + 6] = 2; // g8
        g.side = Player::One;
        assert_eq!(g.outcome(), None);
        g.apply(&(36, 9)); // e5 -> b2, touching a1 and b1
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn capture_removes_piece() {
        let mut g = Linii::new();
        g.cells = [0; N * N];
        g.cells[0] = 1; // a1
        g.cells[7] = 1; // h1
        g.cells[N + 1] = 2; // b2
        g.cells[N + 6] = 2; // g2
        g.cells[7 * N + 7] = 2; // h8
        g.side = Player::One;
        let ms: Vec<String> = g.legal_moves().iter().map(|m| g.format_move(m)).collect();
        // Row 1: a1,h1 = 2 pieces -> a1-c1.
        assert!(ms.contains(&"a1-c1".to_string()));
        // Diagonal a1-b2-h8 = 3 pieces; passing the enemy on b2 is forbidden.
        assert!(!ms.iter().any(|m| m.starts_with("a1") && m.ends_with("d4")));
        // h1 anti-diagonal h1-g2: 2 pieces -> lands on f3 (passes enemy g2): illegal.
        assert!(!ms.contains(&"h1-f3".to_string()));
        // Column a: a1 alone... 1 piece -> a2; capturing needs landing on an enemy:
        // h1 up column h: h1,h8 = 2 pieces -> h3.
        assert!(ms.contains(&"h1-h3".to_string()));
        // Landing on an enemy captures: a1 along row? build a direct capture.
        g.cells[2] = 2; // c1: row 1 now has 3 pieces, a1 -> d1 passes c1 (blocked)
        g.cells[N + 1] = 0;
        g.cells[N] = 2; // a2 enemy adjacent; column a holds 2 pieces: a1 -> a3 passes a2: blocked
        let g2 = g.clone();
        let ms2: Vec<String> = g2.legal_moves().iter().map(|m| g2.format_move(m)).collect();
        assert!(!ms2.contains(&"a1-d1".to_string()));
        assert!(!ms2.contains(&"a1-a3".to_string()));
        // Direct capture: h1 -> h4 where an enemy stands.
        let mut g3 = g.clone();
        g3.cells[3 * N + 7] = 2; // h4 (column h then holds 3 pieces)
        let m = *g3
            .legal_moves()
            .iter()
            .find(|m| g3.format_move(m) == "h1xh4")
            .expect("capture on h4");
        let before = g3.cells.iter().filter(|&&v| v == 2).count();
        g3.apply(&m);
        assert_eq!(g3.cells.iter().filter(|&&v| v == 2).count(), before - 1);
    }

    #[test]
    fn random_playouts_terminate() {
        for seed in 1..=8u64 {
            let mut g = Linii::new();
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
                assert!(n <= MAX_PLIES as usize + 1);
            }
        }
    }
}
