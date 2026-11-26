//! Faianța — alegerea de plăcuțe pe modele, variantă originală pentru doi.
//!
//! Cinci culori de plăcuțe (T turcoaz, O ocru, G grena, I indigo, A alb), câte 20, într-o pungă.
//! Fiecare rundă: 5 manufacturi (fabrici) primesc câte 4 plăcuțe din pungă. Pe rând, un jucător ia
//! TOATE plăcuțele de o culoare dintr-o manufactură (restul trec în centru) sau din centru.
//! Cine ia primul din centru ia și jetonul de start (cade pe podea, −1) și începe runda următoare.
//! Plăcuțele se pun pe una din cele 5 linii de model (capacități 1..5; o linie ține o singură
//! culoare; nu poți pune o culoare pe linie dacă rândul de perete corespunzător o are deja) sau pe
//! podea. Ce nu încape cade pe podea. Podeaua penalizează: −1, −1, −2, −2, −2, −3, −3.
//!
//! La sfârșitul rundei (mutarea forțată `decontare`) fiecare linie plină trimite o plăcuță pe peretele
//! 5×5 (culoarea c a rândului r merge pe coloana (c+r) mod 5) și punctează: 1 dacă e izolată,
//! altfel lungimea șirului orizontal și/sau vertical care o conține. Jocul se termină când cineva
//! completează un rând al peretelui. Bonus final: +2 pe rând complet, +7 pe coloană completă,
//! +10 pe culoare completă (5 plăcuțe).
//!
//! Mutări: `f3 T 2` (din manufactura 3, culoarea T, pe linia 2), `c O p` (din centru, pe podea:
//! `p`). Informație: toate vizibile; singura întâmplare e extragerea din pungă.
#![allow(clippy::needless_range_loop, clippy::doc_lazy_continuation)]

use crate::cards::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const COLORS: [char; 5] = ['T', 'O', 'G', 'I', 'A'];
const FLOOR_PEN: [i32; 7] = [1, 1, 2, 2, 2, 3, 3];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    /// `src` 0..5 = manufactură, 5 = centru; `line` 0..5 = linie, 5 = podea.
    Take {
        src: u8,
        color: u8,
        line: u8,
    },
    Settle,
}

#[derive(Clone)]
pub struct Faianta {
    bag: Vec<u8>,
    lid: Vec<u8>,
    fact: [[u8; 5]; 5],
    center: [u8; 5],
    marker_in_center: bool,
    wall: [[[bool; 5]; 5]; 2],
    lines: [[(i8, u8); 5]; 2],
    /// Podea: Some(culoare) sau None = jetonul de start.
    floor: [Vec<Option<u8>>; 2],
    score: [i32; 2],
    side: Player,
    next_first: Player,
    settling: bool,
    over: bool,
    rng: Rng,
    rounds: u32,
}

impl Default for Faianta {
    fn default() -> Self {
        Self::new()
    }
}

fn col_of(row: usize, color: usize) -> usize {
    (color + row) % 5
}

impl Faianta {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut bag = Vec::new();
        for c in 0..5u8 {
            for _ in 0..20 {
                bag.push(c);
            }
        }
        shuffle(&mut bag, &mut rng);
        let mut g = Faianta {
            bag,
            lid: Vec::new(),
            fact: [[0; 5]; 5],
            center: [0; 5],
            marker_in_center: true,
            wall: [[[false; 5]; 5]; 2],
            lines: [[(-1, 0); 5]; 2],
            floor: [Vec::new(), Vec::new()],
            score: [0; 2],
            side: Player::One,
            next_first: Player::One,
            settling: false,
            over: false,
            rng,
            rounds: 1,
        };
        g.refill();
        g
    }

    pub fn scores(&self) -> (i32, i32) {
        (self.score[0], self.score[1])
    }

    fn draw_tile(&mut self) -> Option<u8> {
        if self.bag.is_empty() && !self.lid.is_empty() {
            let mut l = std::mem::take(&mut self.lid);
            shuffle(&mut l, &mut self.rng);
            self.bag = l;
        }
        self.bag.pop()
    }

    fn refill(&mut self) {
        for f in 0..5 {
            self.fact[f] = [0; 5];
            for _ in 0..4 {
                if let Some(t) = self.draw_tile() {
                    self.fact[f][t as usize] += 1;
                }
            }
        }
        self.center = [0; 5];
        self.marker_in_center = true;
    }

    pub fn total_tiles(&self) -> usize {
        let mut n = self.bag.len() + self.lid.len();
        for f in &self.fact {
            n += f.iter().map(|x| *x as usize).sum::<usize>();
        }
        n += self.center.iter().map(|x| *x as usize).sum::<usize>();
        for p in 0..2 {
            n += self.wall[p].iter().flatten().filter(|x| **x).count();
            n += self.lines[p].iter().map(|l| l.1 as usize).sum::<usize>();
            n += self.floor[p].iter().filter(|x| x.is_some()).count();
        }
        n
    }

    fn tiles_on_table(&self) -> usize {
        self.fact
            .iter()
            .flatten()
            .map(|x| *x as usize)
            .sum::<usize>()
            + self.center.iter().map(|x| *x as usize).sum::<usize>()
    }

    fn can_place(&self, p: usize, color: u8, line: usize) -> bool {
        let (c, n) = self.lines[p][line];
        if n as usize > line {
            return false;
        }
        if c >= 0 && c != color as i8 {
            return false;
        }
        !self.wall[p][line][col_of(line, color as usize)]
    }

    fn floor_penalty(f: &[Option<u8>]) -> i32 {
        f.iter()
            .take(7)
            .enumerate()
            .map(|(i, _)| FLOOR_PEN[i])
            .sum()
    }

    fn place_score(&self, p: usize, r: usize, c: usize) -> i32 {
        let w = &self.wall[p];
        let (mut h, mut v) = (1, 1);
        let mut x = c;
        while x > 0 && w[r][x - 1] {
            h += 1;
            x -= 1;
        }
        x = c;
        while x < 4 && w[r][x + 1] {
            h += 1;
            x += 1;
        }
        let mut y = r;
        while y > 0 && w[y - 1][c] {
            v += 1;
            y -= 1;
        }
        y = r;
        while y < 4 && w[y + 1][c] {
            v += 1;
            y += 1;
        }
        if h == 1 && v == 1 {
            1
        } else {
            (if h > 1 { h } else { 0 }) + (if v > 1 { v } else { 0 })
        }
    }

    fn settle(&mut self) {
        for p in 0..2 {
            for r in 0..5 {
                let (c, n) = self.lines[p][r];
                if n as usize == r + 1 {
                    let col = col_of(r, c as usize);
                    self.wall[p][r][col] = true;
                    self.score[p] += self.place_score(p, r, col);
                    for _ in 0..r {
                        self.lid.push(c as u8);
                    }
                    self.lines[p][r] = (-1, 0);
                }
            }
            let pen = Self::floor_penalty(&self.floor[p]);
            self.score[p] = (self.score[p] - pen).max(0);
            for t in std::mem::take(&mut self.floor[p]).into_iter().flatten() {
                self.lid.push(t);
            }
        }
        let ended = (0..2).any(|p| (0..5).any(|r| self.wall[p][r].iter().all(|x| *x)));
        if ended {
            for p in 0..2 {
                self.score[p] += self.final_bonus(p);
            }
            self.over = true;
        } else {
            self.refill();
            self.side = self.next_first;
            self.settling = false;
            self.rounds += 1;
        }
    }

    fn final_bonus(&self, p: usize) -> i32 {
        let w = &self.wall[p];
        let mut b = 0;
        for r in 0..5 {
            if w[r].iter().all(|x| *x) {
                b += 2;
            }
        }
        for c in 0..5 {
            if (0..5).all(|r| w[r][c]) {
                b += 7;
            }
        }
        for color in 0..5 {
            if (0..5).all(|r| w[r][col_of(r, color)]) {
                b += 10;
            }
        }
        b
    }

    fn rows_done(&self, p: usize) -> usize {
        (0..5)
            .filter(|r| self.wall[p][*r].iter().all(|x| *x))
            .count()
    }

    /// Valoarea așteptată a stării unui jucător (fără a citi punga).
    fn heuristic(&self, p: usize) -> f64 {
        let mut s = f64::from(self.score[p]);
        // Linii pline vor puncta la decontare.
        let mut tmp = self.clone();
        let mut gain = 0;
        for r in 0..5 {
            let (c, n) = self.lines[p][r];
            if n as usize == r + 1 {
                let col = col_of(r, c as usize);
                gain += tmp.place_score(p, r, col);
                tmp.wall[p][r][col] = true;
            } else if n > 0 {
                // Progres parțial, cu potențial de adiacență.
                let frac = f64::from(n) / (r + 1) as f64;
                let col = col_of(r, c as usize);
                let adj = tmp.place_score(p, r, col);
                s += frac * frac * f64::from(adj) * 0.8;
            }
        }
        s += f64::from(gain);
        s -= f64::from(Self::floor_penalty(&self.floor[p]));
        // Bonusuri potențiale.
        let w = &tmp.wall[p];
        for r in 0..5 {
            let k = w[r].iter().filter(|x| **x).count() as f64;
            s += 0.35 * k * k / 5.0;
        }
        for c in 0..5 {
            let k = (0..5).filter(|r| w[*r][c]).count() as f64;
            s += 0.9 * k * k / 5.0;
        }
        for color in 0..5 {
            let k = (0..5).filter(|r| w[*r][col_of(*r, color)]).count() as f64;
            s += 1.2 * k * k / 5.0;
        }
        s
    }
}

impl Game for Faianta {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over {
            return Vec::new();
        }
        if self.settling {
            return vec![Move::Settle];
        }
        let p = self.side.idx();
        let mut v = Vec::new();
        for src in 0..6u8 {
            let tiles = if src == 5 {
                &self.center
            } else {
                &self.fact[src as usize]
            };
            for color in 0..5u8 {
                if tiles[color as usize] == 0 {
                    continue;
                }
                for line in 0..5usize {
                    if self.can_place(p, color, line) {
                        v.push(Move::Take {
                            src,
                            color,
                            line: line as u8,
                        });
                    }
                }
                v.push(Move::Take {
                    src,
                    color,
                    line: 5,
                });
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        let p = self.side.idx();
        match *m {
            Move::Settle => self.settle(),
            Move::Take { src, color, line } => {
                let n;
                if src == 5 {
                    n = self.center[color as usize];
                    self.center[color as usize] = 0;
                    if self.marker_in_center {
                        self.marker_in_center = false;
                        self.floor[p].push(None);
                        self.next_first = self.side;
                    }
                } else {
                    let f = src as usize;
                    n = self.fact[f][color as usize];
                    for c in 0..5 {
                        if c != color as usize {
                            self.center[c] += self.fact[f][c];
                        }
                    }
                    self.fact[f] = [0; 5];
                }
                let mut left = n;
                if line < 5 {
                    let l = line as usize;
                    let room = (l as u8 + 1) - self.lines[p][l].1;
                    let put = room.min(n);
                    self.lines[p][l].0 = color as i8;
                    self.lines[p][l].1 += put;
                    left -= put;
                }
                for _ in 0..left {
                    if self.floor[p].len() < 7 {
                        self.floor[p].push(Some(color));
                    } else {
                        self.lid.push(color);
                    }
                }
                if self.tiles_on_table() == 0 {
                    self.settling = true;
                }
                self.side = self.side.other();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            return None;
        }
        Some(match self.score[0].cmp(&self.score[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => match self.rows_done(0).cmp(&self.rows_done(1)) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                std::cmp::Ordering::Equal => Outcome::Draw,
            },
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        ((self.heuristic(p.idx()) - self.heuristic(p.other().idx())) * 10.0) as i32
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(2)
    }

    fn render(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            "Faianța — runda {} | scor: J1 {}, J2 {} | pungă {}\n",
            self.rounds,
            self.score[0],
            self.score[1],
            self.bag.len()
        ));
        let show = |t: &[u8; 5]| {
            let mut out = String::new();
            for c in 0..5 {
                for _ in 0..t[c] {
                    out.push(COLORS[c]);
                }
            }
            if out.is_empty() {
                "—".to_string()
            } else {
                out
            }
        };
        for f in 0..5 {
            s.push_str(&format!("manufactura {}: {}\n", f + 1, show(&self.fact[f])));
        }
        s.push_str(&format!(
            "centru: {}{}\n",
            show(&self.center),
            if self.marker_in_center {
                "  (+ jetonul de start)"
            } else {
                ""
            }
        ));
        for p in 0..2 {
            s.push_str(&format!("— J{} —\n", p + 1));
            for r in 0..5 {
                let (c, n) = self.lines[p][r];
                let mut line = String::new();
                for i in 0..5 {
                    // capacitate r+1, aliniat la dreapta
                    if i + r + 1 >= 5 {
                        let k = i + r + 1 - 5;
                        if (k as u8) < n {
                            line.push(COLORS[c as usize]);
                        } else {
                            line.push('.');
                        }
                    } else {
                        line.push(' ');
                    }
                }
                let mut wall = String::new();
                for col in 0..5 {
                    // culoarea acestui loc: (col − r) mod 5
                    let color = (col + 5 - r) % 5;
                    if self.wall[p][r][col] {
                        wall.push(COLORS[color]);
                    } else {
                        wall.push(COLORS[color].to_ascii_lowercase());
                    }
                }
                s.push_str(&format!("  {} {} | {}\n", r + 1, line, wall));
            }
            let fl: Vec<String> = self.floor[p]
                .iter()
                .map(|t| match t {
                    Some(c) => COLORS[*c as usize].to_string(),
                    None => "1".to_string(),
                })
                .collect();
            s.push_str(&format!(
                "  podea: {}\n",
                if fl.is_empty() {
                    "—".into()
                } else {
                    fl.join(" ")
                }
            ));
        }
        if self.over {
            s.push_str("Joc încheiat.\n");
        } else if self.settling {
            s.push_str("Runda s-a terminat: decontare.\n");
        } else {
            s.push_str(&format!(
                "Mută J{}: f<n> <culoare> <linie 1-5|p>  sau  c <culoare> <linie|p>.\n",
                self.side.idx() + 1
            ));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match *m {
            Move::Settle => "decontare".into(),
            Move::Take { src, color, line } => {
                let from = if src == 5 {
                    "c".to_string()
                } else {
                    format!("f{}", src + 1)
                };
                let to = if line == 5 {
                    "p".to_string()
                } else {
                    (line + 1).to_string()
                };
                format!("{from} {} {to}", COLORS[color as usize])
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setup_conserves() {
        let g = Faianta::with_seed(1);
        assert_eq!(g.total_tiles(), 100);
        assert_eq!(g.tiles_on_table(), 20);
    }

    #[test]
    fn take_moves_rest_to_center_and_marker() {
        let mut g = Faianta::with_seed(2);
        g.fact = [[0; 5]; 5];
        g.fact[0] = [2, 1, 1, 0, 0];
        g.center = [0, 0, 0, 3, 0];
        g.bag.truncate(10);
        g.apply(&Move::Take {
            src: 0,
            color: 0,
            line: 1,
        });
        assert_eq!(g.lines[0][1], (0, 2));
        assert_eq!(g.center, [0, 1, 1, 3, 0]);
        // Din centru: ia jetonul și 3 plăcuțe, linia 2 (capacitate 3).
        g.apply(&Move::Take {
            src: 5,
            color: 3,
            line: 2,
        });
        assert_eq!(g.floor[1], vec![None]);
        assert_eq!(g.next_first, Player::Two);
        assert_eq!(g.lines[1][2], (3, 3));
    }

    #[test]
    fn overflow_goes_to_floor_and_color_rules() {
        let mut g = Faianta::with_seed(3);
        g.fact = [[0; 5]; 5];
        g.fact[1] = [4, 0, 0, 0, 0];
        g.apply(&Move::Take {
            src: 1,
            color: 0,
            line: 0,
        });
        assert_eq!(g.lines[0][0], (0, 1));
        assert_eq!(g.floor[0].len(), 3);
        // Linia 1 are turcoaz: nu se mai pune altă culoare și nici peste plin.
        g.fact[0] = [1, 1, 0, 0, 0];
        g.settling = false;
        g.side = Player::One;
        let l = g.legal_moves();
        assert!(!l.contains(&Move::Take {
            src: 0,
            color: 1,
            line: 0
        }));
        assert!(!l.contains(&Move::Take {
            src: 0,
            color: 0,
            line: 0
        }));
        assert!(l.contains(&Move::Take {
            src: 0,
            color: 0,
            line: 4
        }));
        // Perete: dacă rândul 2 are deja culoarea, nu se poate.
        g.wall[0][1][col_of(1, 1)] = true;
        assert!(!g.legal_moves().contains(&Move::Take {
            src: 0,
            color: 1,
            line: 1
        }));
    }

    #[test]
    fn wall_scoring() {
        let mut g = Faianta::with_seed(4);
        g.wall[0][2][1] = true;
        g.wall[0][2][2] = true;
        g.wall[0][1][3] = true;
        // plăcuță la (2,3): orizontal 3 (1,2,3), vertical cu (1,3): 2 → 5
        g.wall[0][2][3] = true;
        assert_eq!(g.place_score(0, 2, 3), 3 + 2);
        let h = Faianta::with_seed(5);
        assert_eq!(h.place_score(0, 0, 0), 1);
    }

    #[test]
    fn settle_scores_and_penalizes() {
        let mut g = Faianta::with_seed(6);
        g.fact = [[0; 5]; 5];
        g.center = [0; 5];
        g.lines[0][0] = (2, 1);
        g.lines[0][1] = (0, 2);
        g.floor[0] = vec![None, Some(1), Some(1)];
        g.settling = true;
        g.apply(&Move::Settle);
        assert!(g.wall[0][0][col_of(0, 2)]);
        assert!(g.wall[0][1][col_of(1, 0)]);
        // 1 + 1 (izolate) − (1+1+2)=4 → max(0, 2−4) = 0
        assert_eq!(g.score[0], 0);
        assert_eq!(g.tiles_on_table(), 20);
        assert_eq!(g.to_move(), Player::One);
    }

    #[test]
    fn random_games_end_and_conserve() {
        for seed in 1..15 {
            let mut g = Faianta::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            let mut check_after_settle = false;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                let was_settle = m == vec![Move::Settle];
                g.apply(&m[rng.below(m.len())]);
                if was_settle {
                    check_after_settle = true;
                }
                // Mereu: plăcuțe totale ≤ 100 și (după refill) = 100.
                assert!(g.total_tiles() <= 100);
                if check_after_settle && !g.over {
                    assert_eq!(g.total_tiles(), 100);
                    check_after_settle = false;
                }
                n += 1;
                assert!(n < 5000);
            }
        }
    }

    #[test]
    fn ai_beats_random() {
        let mut wins = 0;
        for seed in 1..=10u64 {
            let mut g = Faianta::with_seed(seed);
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
        assert!(wins >= 9, "wins {wins}");
    }
}
