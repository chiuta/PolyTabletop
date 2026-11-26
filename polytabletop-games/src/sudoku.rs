//! Sudoku duel (Japonia / SUA) — două persoane (sau om + calculator) rezolvă împreună o grilă cu soluție unică.
//!
//! * Pe rând, fiecare alege o celulă goală și o cifră care nu se repetă pe rând/coloană/pătrat
//!   (`r3c4=5` = rândul 3, coloana 4, cifra 5).
//! * Cifra corectă (cea din soluție) rămâne în grilă: +1 punct, +2 puncte pentru fiecare rând,
//!   coloană sau pătrat 3×3 pe care îl completezi. Cifra greșită nu se pune: −1 punct, iar cifra
//!   e eliminată pentru acea celulă (informație publică). Verdictul unei mutări se afișează
//!   când urmează adversarul.
//! * Se termină când grila e plină; cine are mai multe puncte câștigă.
//!
//! Grila e generată de program (soluție unică, ~36 de indicii). AI-ul nu citește soluția stocată:
//! își verifică alegerile cu un rezolvitor propriu pe grila publică.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const PLY_CAP: u16 = 900;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Mv {
    pub cell: u8,
    pub digit: u8,
}

#[derive(Clone)]
pub struct Sudoku {
    grid: [u8; 81],
    solution: [u8; 81],
    excl: [u16; 81],
    score: [i32; 2],
    pending: Option<(Mv, usize)>,
    side: Player,
    ply: u16,
}

fn units(cell: usize) -> [usize; 3] {
    let (r, c) = (cell / 9, cell % 9);
    [r, 9 + c, 18 + (r / 3) * 3 + c / 3]
}

fn unit_cells(u: usize) -> [usize; 9] {
    let mut v = [0usize; 9];
    for (k, slot) in v.iter_mut().enumerate() {
        *slot = match u {
            0..=8 => u * 9 + k,
            9..=17 => k * 9 + (u - 9),
            _ => {
                let b = u - 18;
                (b / 3 * 3 + k / 3) * 9 + (b % 3) * 3 + k % 3
            }
        };
    }
    v
}

fn candidates(g: &[u8; 81], cell: usize) -> u16 {
    let mut used = 0u16;
    for u in units(cell) {
        for c in unit_cells(u) {
            if g[c] != 0 {
                used |= 1 << g[c];
            }
        }
    }
    !used & 0x3FE
}

/// Numără soluțiile (până la `limit`).
fn count_solutions(g: &mut [u8; 81], limit: usize) -> usize {
    let mut best = None;
    let mut best_n = 10;
    for i in 0..81 {
        if g[i] == 0 {
            let n = candidates(g, i).count_ones();
            if n < best_n {
                best_n = n;
                best = Some(i);
                if n <= 1 {
                    break;
                }
            }
        }
    }
    let Some(i) = best else { return 1 };
    let cand = candidates(g, i);
    let mut total = 0;
    for d in 1..=9u8 {
        if cand & (1 << d) != 0 {
            g[i] = d;
            total += count_solutions(g, limit - total);
            g[i] = 0;
            if total >= limit {
                break;
            }
        }
    }
    total
}

fn fill_random(g: &mut [u8; 81], rng: &mut Rng) -> bool {
    let Some(i) = (0..81).find(|&i| g[i] == 0) else {
        return true;
    };
    let cand = candidates(g, i);
    let mut ds: Vec<u8> = (1..=9).filter(|d| cand & (1 << d) != 0).collect();
    for k in (1..ds.len()).rev() {
        ds.swap(k, rng.below(k + 1));
    }
    for d in ds {
        g[i] = d;
        if fill_random(g, rng) {
            return true;
        }
    }
    g[i] = 0;
    false
}

impl Default for Sudoku {
    fn default() -> Self {
        Self::new()
    }
}

impl Sudoku {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut sol = [0u8; 81];
        fill_random(&mut sol, &mut rng);
        let mut grid = sol;
        let mut order: Vec<usize> = (0..81).collect();
        for k in (1..81).rev() {
            order.swap(k, rng.below(k + 1));
        }
        let mut clues = 81;
        for c in order {
            if clues <= 34 {
                break;
            }
            let keep = grid[c];
            grid[c] = 0;
            let mut t = grid;
            if count_solutions(&mut t, 2) == 1 {
                clues -= 1;
            } else {
                grid[c] = keep;
            }
        }
        Sudoku {
            grid,
            solution: sol,
            excl: [0; 81],
            score: [0; 2],
            pending: None,
            side: Player::One,
            ply: 0,
        }
    }

    pub fn scores(&self) -> (i32, i32) {
        let r = self.resolved();
        (r.score[0], r.score[1])
    }

    pub fn empty_cells(&self) -> usize {
        self.resolved().grid.iter().filter(|&&d| d == 0).count()
    }

    /// Starea cu verdictul mutării în așteptare aplicat.
    fn resolved(&self) -> Sudoku {
        let mut r = self.clone();
        r.pending = None;
        if let Some((m, who)) = self.pending {
            let c = m.cell as usize;
            if self.solution[c] == m.digit {
                r.grid[c] = m.digit;
                r.score[who] += 1;
                for u in units(c) {
                    if unit_cells(u).iter().all(|&x| r.grid[x] != 0) {
                        r.score[who] += 2;
                    }
                }
            } else {
                r.score[who] -= 1;
                r.excl[c] |= 1 << m.digit;
            }
        }
        r
    }
}

impl Game for Sudoku {
    type Move = Mv;
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<Mv> {
        let r = self.resolved();
        if r.grid.iter().all(|&d| d != 0) || r.ply >= PLY_CAP {
            return Vec::new();
        }
        let mut v = Vec::new();
        for i in 0..81 {
            if r.grid[i] == 0 {
                let cand = candidates(&r.grid, i) & !r.excl[i];
                for d in 1..=9u8 {
                    if cand & (1 << d) != 0 {
                        v.push(Mv {
                            cell: i as u8,
                            digit: d,
                        });
                    }
                }
            }
        }
        v
    }
    fn apply(&mut self, m: &Mv) {
        let who = self.side.idx();
        *self = self.resolved();
        self.pending = Some((*m, who));
        self.ply += 1;
        self.side = self.side.other();
    }
    fn outcome(&self) -> Option<Outcome> {
        let r = self.resolved();
        let full = r.grid.iter().all(|&d| d != 0);
        if !full && r.ply < PLY_CAP {
            return None;
        }
        Some(match r.score[0].cmp(&r.score[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            _ => Outcome::Draw,
        })
    }
    fn evaluate(&self, p: Player) -> i32 {
        // doar informație publică: punctajul verificat și, pentru ultima cifră, o verificare cu rezolvitorul
        let me = p.idx();
        let mut v = 100 * (self.score[me] - self.score[1 - me]);
        if let Some((m, who)) = self.pending {
            let mut g = self.grid;
            g[m.cell as usize] = m.digit;
            let ok = count_solutions(&mut g, 1) >= 1;
            let delta = if ok { 100 } else { -100 };
            v += if who == me { delta } else { -delta };
        }
        v
    }
    fn render(&self) -> String {
        let r = self.resolved();
        let mut s = String::from("    1 2 3   4 5 6   7 8 9\n");
        for row in 0..9 {
            if row % 3 == 0 {
                s.push_str("  +-------+-------+-------+\n");
            }
            s.push_str(&format!("{} |", row + 1));
            for col in 0..9 {
                let d = r.grid[row * 9 + col];
                s.push(' ');
                s.push(if d == 0 { '.' } else { (b'0' + d) as char });
                if col % 3 == 2 {
                    s.push_str(" |");
                }
            }
            s.push('\n');
        }
        s.push_str("  +-------+-------+-------+\n");
        s.push_str(&format!(
            "Scor: J1 {} – {} J2. Celule goale: {}. ",
            r.score[0],
            r.score[1],
            r.grid.iter().filter(|&&d| d == 0).count()
        ));
        if let Some((m, who)) = self.pending {
            let ok = self.solution[m.cell as usize] == m.digit;
            s.push_str(&format!(
                "Ultima mutare (J{}): {} — {}. ",
                who + 1,
                self.format_move(&m),
                if ok { "corectă" } else { "greșită" }
            ));
        }
        s.push_str(&format!("La mutare: Jucătorul {}.", self.side.idx() + 1));
        s
    }
    fn format_move(&self, m: &Mv) -> String {
        format!("r{}c{}={}", m.cell / 9 + 1, m.cell % 9 + 1, m.digit)
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q4_util::*;
    use polytabletop_core::{GameObj, Player};

    #[test]
    fn puzzle_is_valid_and_unique() {
        let g = Sudoku::with_seed(3);
        let mut t = g.grid;
        assert_eq!(count_solutions(&mut t, 2), 1);
        assert_eq!(t, g.grid, "restaurat");
        let mut s = g.grid;
        // rezolvarea completă coincide cu soluția stocată
        fn solve(g: &mut [u8; 81]) -> bool {
            let Some(i) = (0..81).find(|&i| g[i] == 0) else {
                return true;
            };
            let c = candidates(g, i);
            for d in 1..=9u8 {
                if c & (1 << d) != 0 {
                    g[i] = d;
                    if solve(g) {
                        return true;
                    }
                }
            }
            g[i] = 0;
            false
        }
        assert!(solve(&mut s));
        assert_eq!(s, g.solution);
    }

    #[test]
    fn correct_digit_scores_wrong_digit_is_excluded() {
        let mut g = Sudoku::with_seed(4);
        let cell = (0..81).find(|&i| g.grid[i] == 0).unwrap();
        let right = g.solution[cell];
        let wrong = GameObj::legal_moves(&g).into_iter().find(|t| {
            t.starts_with(&format!("r{}c{}=", cell / 9 + 1, cell % 9 + 1))
                && !t.ends_with(&right.to_string())
        });
        let ok = Mv {
            cell: cell as u8,
            digit: right,
        };
        g.apply(&ok);
        assert!(g.scores().0 >= 1);
        let _ = wrong;
        let mut h = Sudoku::with_seed(4);
        // alege o cifră greșită, dacă există una fără conflict
        let cand = candidates(&h.grid, cell);
        if let Some(d) = (1..=9u8).find(|&d| d != right && cand & (1 << d) != 0) {
            h.apply(&Mv {
                cell: cell as u8,
                digit: d,
            });
            assert_eq!(h.scores().0, -1);
            assert!(!GameObj::legal_moves(&h)
                .iter()
                .any(|t| *t == format!("r{}c{}={}", cell / 9 + 1, cell % 9 + 1, d)));
        }
    }

    #[test]
    fn random_and_ai_games_finish() {
        random_finishes(|s| Box::new(Sudoku::with_seed(s)), 2, 1200);
        let r = ai_vs_random(|s| Box::new(Sudoku::with_seed(s)), 5, Player::One, 1200);
        assert_eq!(r, Some(Outcome::Win(Player::One)));
    }
}
