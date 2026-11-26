//! Renju (Japonia): Gomoku pe 15×15 cu restricții pentru Negru (J1, mută primul):
//! **linia lungă** (6 sau mai multe), **dublu-patru** (4-4) și **dublu-trei** (3-3 deschis)
//! sunt interzise lui Negru; el câștigă doar cu exact cinci în linie, Alb câștigă cu cinci
//! sau mai multe. O mutare care face exact cinci e întotdeauna permisă.
//!
//! Simplificări: fără regulile de deschidere (swap, Soosyrv etc.; Negru începe în centru);
//! la verificarea unui „trei deschis” nu se mai verifică recursiv dacă patru-ul rezultat ar
//! fi la rândul lui interzis; punctele interzise nu apar în lista mutărilor lui Negru.
//! Alb poate câștiga și dacă Negru rămâne fără mutări permise.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: usize = 15;
const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (1, 1), (1, -1)];

#[derive(Clone)]
pub struct Renju {
    cells: [u8; N * N],
    side: Player,
    count: u16,
    winner: Option<Player>,
}

impl Default for Renju {
    fn default() -> Self {
        Self::new()
    }
}

impl Renju {
    pub fn new() -> Self {
        Renju {
            cells: [0; N * N],
            side: Player::One,
            count: 0,
            winner: None,
        }
    }

    fn at(&self, r: i32, c: i32) -> u8 {
        if r < 0 || r >= N as i32 || c < 0 || c >= N as i32 {
            255
        } else {
            self.cells[r as usize * N + c as usize]
        }
    }

    /// Lungimea liniei continue de culoare `me` prin (r,c) pe direcția (dr,dc), cu (r,c) tratat ca `me`.
    fn run(&self, r: i32, c: i32, dr: i32, dc: i32, me: u8) -> i32 {
        let mut n = 1;
        for sign in [1i32, -1] {
            let (mut rr, mut cc) = (r + dr * sign, c + dc * sign);
            while self.at(rr, cc) == me {
                n += 1;
                rr += dr * sign;
                cc += dc * sign;
            }
        }
        n
    }

    fn five_at(&self, idx: usize) -> bool {
        let me = self.cells[idx];
        let (row, col) = ((idx / N) as i32, (idx % N) as i32);
        for (dr, dc) in DIRS {
            let n = self.run(row, col, dr, dc, me);
            if (me == 2 && n >= 5) || (me == 1 && n == 5) {
                return true;
            }
        }
        false
    }

    /// Punctele goale care, jucate de Negru, ar face exact cinci pe direcția dată, prin (r,c).
    fn completions(&mut self, r: i32, c: i32, dr: i32, dc: i32) -> Vec<(i32, i32)> {
        let mut v = Vec::new();
        for k in -5..=5i32 {
            let (rr, cc) = (r + dr * k, c + dc * k);
            if self.at(rr, cc) == 0 {
                self.cells[rr as usize * N + cc as usize] = 1;
                if self.run(rr, cc, dr, dc, 1) == 5 {
                    v.push((rr, cc));
                }
                self.cells[rr as usize * N + cc as usize] = 0;
            }
        }
        v
    }

    /// Este (r,c) — în care Negru tocmai a pus o piesă — un punct interzis?
    fn forbidden_black(&mut self, idx: usize) -> bool {
        let (r, c) = ((idx / N) as i32, (idx % N) as i32);
        self.cells[idx] = 1;
        let mut bad = false;
        let (mut fours, mut threes) = (0, 0);
        let mut five = false;
        let mut over = false;
        for (dr, dc) in DIRS {
            let n = self.run(r, c, dr, dc, 1);
            if n == 5 {
                five = true;
            }
            if n >= 6 {
                over = true;
            }
        }
        if !five {
            if over {
                bad = true;
            } else {
                for (dr, dc) in DIRS {
                    let comp = self.completions(r, c, dr, dc);
                    if !comp.is_empty() {
                        // două completări la distanță 5 = un singur patru deschis
                        let one = comp.len() == 2
                            && ((comp[0].0 - comp[1].0)
                                .abs()
                                .max((comp[0].1 - comp[1].1).abs())
                                == 5);
                        fours += if one { 1 } else { comp.len() };
                    } else if self.open_three(r, c, dr, dc) {
                        threes += 1;
                    }
                }
                if fours >= 2 || threes >= 2 {
                    bad = true;
                }
            }
        }
        self.cells[idx] = 0;
        bad
    }

    /// Există un punct care transformă linia în patru deschis (două capete libere, fără linie lungă)?
    fn open_three(&mut self, r: i32, c: i32, dr: i32, dc: i32) -> bool {
        for k in -4..=4i32 {
            let (rr, cc) = (r + dr * k, c + dc * k);
            if self.at(rr, cc) != 0 {
                continue;
            }
            self.cells[rr as usize * N + cc as usize] = 1;
            let mut open_four = false;
            if self.run(rr, cc, dr, dc, 1) == 4 {
                // capetele continuării
                let mut lo = 0;
                let (mut a, mut b) = (rr, cc);
                while self.at(a - dr, b - dc) == 1 {
                    a -= dr;
                    b -= dc;
                    lo += 1;
                }
                let _ = lo;
                let (mut e, mut f) = (rr, cc);
                while self.at(e + dr, f + dc) == 1 {
                    e += dr;
                    f += dc;
                }
                let (l1, l2) = (self.at(a - dr, b - dc), self.at(e + dr, f + dc));
                let (l1b, l2b) = (
                    self.at(a - 2 * dr, b - 2 * dc),
                    self.at(e + 2 * dr, f + 2 * dc),
                );
                open_four = l1 == 0 && l2 == 0 && l1b != 1 && l2b != 1;
            }
            self.cells[rr as usize * N + cc as usize] = 0;
            if open_four && (k != 0) {
                return true;
            }
        }
        false
    }

    fn name(idx: usize) -> String {
        format!("{}{}", (b'a' + (idx % N) as u8) as char, idx / N + 1)
    }
}

impl Game for Renju {
    type Move = u16;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<u16> {
        if self.winner.is_some() {
            return Vec::new();
        }
        if self.count == 0 {
            return vec![(N / 2 * N + N / 2) as u16];
        }
        // Only points within distance 2 of an existing stone: keeps the AI fast
        // and loses nothing of practical value.
        let mut near = [false; N * N];
        for i in 0..N * N {
            if self.cells[i] != 0 {
                let (r, c) = ((i / N) as i32, (i % N) as i32);
                for dr in -2..=2 {
                    for dc in -2..=2 {
                        let (rr, cc) = (r + dr, c + dc);
                        if rr >= 0 && rr < N as i32 && cc >= 0 && cc < N as i32 {
                            near[rr as usize * N + cc as usize] = true;
                        }
                    }
                }
            }
        }
        let all: Vec<u16> = (0..N * N)
            .filter(|&i| near[i] && self.cells[i] == 0)
            .map(|i| i as u16)
            .collect();
        if self.side == Player::Two {
            return all;
        }
        let mut probe = self.clone();
        let ok: Vec<u16> = all
            .iter()
            .copied()
            .filter(|&i| !probe.forbidden_black(i as usize))
            .collect();
        if ok.is_empty() {
            all
        } else {
            ok
        }
    }

    fn apply(&mut self, m: &u16) {
        let i = *m as usize;
        self.cells[i] = self.side.idx() as u8 + 1;
        self.count += 1;
        if self.five_at(i) {
            self.winner = Some(self.side);
        }
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        if let Some(p) = self.winner {
            Some(Outcome::Win(p))
        } else if self.count as usize == N * N {
            Some(Outcome::Draw)
        } else {
            None
        }
    }

    fn evaluate(&self, p: Player) -> i32 {
        // Pattern evaluation over every maximal run of stones: length, number of
        // open ends and (via 5-windows) broken shapes. Threat bonuses reward
        // fours / open threes of the side to move, which is what wins races.
        let me = p.idx() as u8 + 1;
        let mover = self.side.idx() as u8 + 1;
        let run_w = |len: i32, open: i32, own_turn: bool| -> i32 {
            if len >= 5 {
                return 1_000_000;
            }
            match (len, open) {
                (4, 2) => {
                    if own_turn {
                        200_000
                    } else {
                        60_000
                    }
                }
                (4, 1) => {
                    if own_turn {
                        60_000
                    } else {
                        1_500
                    }
                }
                (3, 2) => {
                    if own_turn {
                        8_000
                    } else {
                        2_000
                    }
                }
                (3, 1) => 200,
                (2, 2) => 150,
                (2, 1) => 20,
                (1, 2) => 8,
                _ => 0,
            }
        };
        let mut score = 0i32;
        let n = N as i32;
        for (dr, dc) in [(0i32, 1i32), (1, 0), (1, 1), (1, -1)] {
            for r in 0..n {
                for c in 0..n {
                    let at = |rr: i32, cc: i32| -> u8 {
                        if rr < 0 || rr >= n || cc < 0 || cc >= n {
                            255
                        } else {
                            self.cells[rr as usize * N + cc as usize]
                        }
                    };
                    let v = at(r, c);
                    if v == 0 {
                        continue;
                    }
                    // Only start at the beginning of a run.
                    if at(r - dr, c - dc) == v {
                        continue;
                    }
                    let mut len = 0;
                    while at(r + dr * len, c + dc * len) == v {
                        len += 1;
                    }
                    let mut open = 0;
                    if at(r - dr, c - dc) == 0 {
                        open += 1;
                    }
                    if at(r + dr * len, c + dc * len) == 0 {
                        open += 1;
                    }
                    // Split shapes: X.XX style (gap of one) count as a (len+1) run
                    // with one fewer open end.
                    let mut w = run_w(len, open, v == mover);
                    if open >= 1 && at(r + dr * len, c + dc * len) == 0 {
                        let mut l2 = 0;
                        while at(r + dr * (len + 1 + l2), c + dc * (len + 1 + l2)) == v {
                            l2 += 1;
                        }
                        if l2 > 0 && len + l2 >= 3 {
                            let bonus = run_w((len + l2).min(4), 1, v == mover);
                            w = w.max(bonus);
                        }
                    }
                    if v == me {
                        score += w;
                    } else {
                        score -= w;
                    }
                }
            }
        }
        score
    }

    fn render(&self) -> String {
        let mut s = String::from("    a b c d e f g h i j k l m n o\n");
        for row in (0..N).rev() {
            s.push_str(&format!("{:>2}  ", row + 1));
            for col in 0..N {
                s.push(match self.cells[row * N + col] {
                    1 => 'X',
                    2 => 'O',
                    _ => '.',
                });
                s.push(' ');
            }
            s.push_str(&format!(" {}\n", row + 1));
        }
        s.push_str("    a b c d e f g h i j k l m n o\n");
        s
    }

    fn format_move(&self, m: &u16) -> String {
        Self::name(*m as usize)
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 500,
            max_depth: 8,
        }
    }

    fn hash_key(&self) -> Option<u64> {
        // Zobrist-style key computed on demand: XOR of one mixed value per piece.
        let mut h = polytabletop_core::hash_mix(1 << 40 | (self.side.idx() as u64) << 32);
        for (i, &v) in self.cells.iter().enumerate() {
            if v != 0 {
                h ^= polytabletop_core::hash_mix((v as u64) << 16 | i as u64);
            }
        }
        Some(h)
    }

    fn move_score(&self, m: &u16) -> i32 {
        // Local threat value: what the point builds for us and blocks for them.
        let (r, c) = ((*m as usize / N) as i32, (*m as usize % N) as i32);
        let n = N as i32;
        let me = self.side.idx() as u8 + 1;
        let you = 3 - me;
        let at = |rr: i32, cc: i32| -> u8 {
            if rr < 0 || rr >= n || cc < 0 || cc >= n {
                255
            } else {
                self.cells[rr as usize * N + cc as usize]
            }
        };
        let w = [1, 10, 100, 1_000, 10_000, 100_000];
        let mut s = 0;
        for (dr, dc) in [(0i32, 1i32), (1, 0), (1, 1), (1, -1)] {
            for (who, mul) in [(me, 11), (you, 10)] {
                let mut len = 0usize;
                let mut open = 0;
                for sign in [1i32, -1] {
                    let mut k = 1;
                    while at(r + dr * sign * k, c + dc * sign * k) == who {
                        len += 1;
                        k += 1;
                    }
                    if at(r + dr * sign * k, c + dc * sign * k) == 0 {
                        open += 1;
                    }
                }
                if len > 0 && (open > 0 || len >= 4) {
                    let l = len.min(4);
                    s += w[l] * mul / if open == 1 { 4 } else { 1 };
                }
            }
        }
        let d = (r - 7).abs().max((c - 7).abs());
        s + 7 - d
    }

    fn move_key(&self, m: &u16) -> Option<u32> {
        Some(*m as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn play(g: &mut Renju, s: &str) {
        let m = g
            .legal_moves()
            .into_iter()
            .find(|m| g.format_move(m) == s)
            .unwrap_or_else(|| panic!("illegal {s}"));
        g.apply(&m);
    }

    #[test]
    fn five_in_a_row_wins() {
        let mut g = Renju::new();
        for (a, b) in [
            ("h8", "h9"),
            ("i8", "i9"),
            ("j8", "j10"),
            ("k8", "k10"),
            ("l8", ""),
        ] {
            play(&mut g, a);
            if !b.is_empty() {
                play(&mut g, b);
            }
        }
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn first_move_is_centre() {
        let g = Renju::new();
        assert_eq!(g.legal_moves().len(), 1);
        assert_eq!(g.format_move(&g.legal_moves()[0]), "h8");
    }

    fn at(r: usize, c: usize) -> usize {
        r * N + c
    }

    #[test]
    fn black_overline_and_double_three_are_forbidden() {
        let mut g = Renju::new();
        g.cells[at(7, 7)] = 2; // piesă albă departe
        g.count = 1;
        for c in [1, 2, 3, 5, 6] {
            g.cells[at(4, c)] = 1;
        }
        let m: Vec<String> = g.legal_moves().iter().map(|x| g.format_move(x)).collect();
        assert!(
            !m.contains(&"e5".to_string()),
            "6 în linie e interzis: {m:?}"
        );
        let mut h = Renju::new();
        h.cells[at(12, 12)] = 2;
        h.count = 1;
        for (r, c) in [(7, 5), (7, 6), (8, 7), (9, 7)] {
            h.cells[at(r, c)] = 1;
        }
        let m: Vec<String> = h.legal_moves().iter().map(|x| h.format_move(x)).collect();
        assert!(!m.contains(&"h8".to_string()), "3-3 interzis: {m:?}");
        assert!(m.contains(&"h6".to_string()) || m.contains(&"g8".to_string()));
    }

    #[test]
    fn exact_five_is_allowed_and_white_may_overline() {
        let mut g = Renju::new();
        g.cells[at(12, 12)] = 2;
        g.count = 1;
        for c in [1, 2, 3, 4] {
            g.cells[at(4, c)] = 1;
        }
        let m: Vec<String> = g.legal_moves().iter().map(|x| g.format_move(x)).collect();
        assert!(m.contains(&"f5".to_string()));
        let mut w = Renju::new();
        w.side = Player::Two;
        w.count = 3;
        for c in [0, 1, 2, 4, 5] {
            w.cells[at(3, c)] = 2;
        }
        w.cells[at(10, 10)] = 1;
        let mv = w
            .legal_moves()
            .into_iter()
            .find(|m| w.format_move(m) == "d4")
            .unwrap();
        w.apply(&mv);
        assert_eq!(w.outcome(), Some(Outcome::Win(Player::Two)));
    }
}
