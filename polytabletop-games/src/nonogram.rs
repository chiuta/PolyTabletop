//! Nonogram (Picross, Japonia) — duel pe aceeași grilă 8×8.
//!
//! * Numerele din dreptul fiecărui rând și de deasupra fiecărei coloane arată lungimile blocurilor
//!   de căsuțe pline (în ordine, separate prin cel puțin o căsuță goală). Soluția e generată din
//!   sămânță și nu se vede.
//! * Pe rând, fiecare jucător **revendică** o căsuță necunoscută: `a1` (e plină) sau `a1x` (e goală);
//!   litera = coloana, cifra = rândul de sus în jos. Apoi `verific` (mutare separată) arată adevărul:
//!   revendicare corectă +1 punct, greșită −2; căsuța devine cunoscută pentru amândoi.
//! * Jocul se termină când nu mai rămâne nicio căsuță necunoscută; câștigă cine are mai multe puncte.
//! * AI-ul deduce din indicii și din căsuțele deja cunoscute (probabilitatea fiecărei căsuțe din
//!   toate aranjamentele posibile ale rândului și coloanei); nu citește soluția.

use crate::q5_util::{by_score, verdict};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: usize = 8;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Claim(u8, bool), // celulă, plină?
    Verify,
}

#[derive(Clone)]
pub struct Nonogram {
    sol: [bool; N * N],
    known: [u8; N * N], // 0 necunoscut, 1 plin, 2 gol
    row_clues: Vec<Vec<usize>>,
    col_clues: Vec<Vec<usize>>,
    score: [i32; 2],
    side: Player,
    pending: Option<(u8, bool)>,
}

fn clues_of(line: &[bool]) -> Vec<usize> {
    let mut v = Vec::new();
    let mut run = 0;
    for &b in line {
        if b {
            run += 1;
        } else if run > 0 {
            v.push(run);
            run = 0;
        }
    }
    if run > 0 {
        v.push(run);
    }
    if v.is_empty() {
        v.push(0);
    }
    v
}

/// Toate aranjamentele liniei compatibile cu indiciile și cu căsuțele cunoscute; întoarce
/// numărul de aranjamente și, pentru fiecare căsuță, în câte e plină.
fn arrangements(clues: &[usize], known: &[u8]) -> (u32, [u32; N]) {
    let mut total = 0;
    let mut filled = [0u32; N];
    let blocks: Vec<usize> = if clues == [0] {
        Vec::new()
    } else {
        clues.to_vec()
    };
    let mut line = [false; N];
    fn rec(
        blocks: &[usize],
        pos: usize,
        line: &mut [bool; N],
        known: &[u8],
        total: &mut u32,
        filled: &mut [u32; N],
    ) {
        if blocks.is_empty() {
            if known[pos..N].contains(&1) {
                return;
            }
            *total += 1;
            for i in 0..N {
                if line[i] {
                    filled[i] += 1;
                }
            }
            return;
        }
        let need: usize = blocks.iter().sum::<usize>() + blocks.len() - 1;
        if pos + need > N {
            return;
        }
        for start in pos..=(N - need) {
            // căsuțele dintre pos și start sunt goale
            if (pos..start).any(|i| known[i] == 1) {
                break;
            }
            let end = start + blocks[0];
            if (start..end).any(|i| known[i] == 2) {
                continue;
            }
            if end < N && known[end] == 1 {
                continue;
            }
            for l in line.iter_mut().take(end).skip(start) {
                *l = true;
            }
            rec(&blocks[1..], (end + 1).min(N), line, known, total, filled);
            for l in line.iter_mut().take(end).skip(start) {
                *l = false;
            }
        }
    }
    rec(&blocks, 0, &mut line, known, &mut total, &mut filled);
    (total, filled)
}

impl Default for Nonogram {
    fn default() -> Self {
        Self::new()
    }
}

impl Nonogram {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut sol = [false; N * N];
        for c in sol.iter_mut() {
            *c = rng.below(100) < 55;
        }
        let rows: Vec<Vec<usize>> = (0..N).map(|r| clues_of(&sol[r * N..r * N + N])).collect();
        let cols: Vec<Vec<usize>> = (0..N)
            .map(|c| clues_of(&(0..N).map(|r| sol[r * N + c]).collect::<Vec<_>>()))
            .collect();
        Nonogram {
            sol,
            known: [0; N * N],
            row_clues: rows,
            col_clues: cols,
            score: [0; 2],
            side: Player::One,
            pending: None,
        }
    }

    /// Probabilitatea (din rând și coloană, combinate) ca o căsuță necunoscută să fie plină.
    pub fn p_filled(&self, cell: usize) -> f64 {
        let (r, c) = (cell / N, cell % N);
        let rowk: Vec<u8> = (0..N).map(|k| self.known[r * N + k]).collect();
        let colk: Vec<u8> = (0..N).map(|k| self.known[k * N + c]).collect();
        let (tr, fr) = arrangements(&self.row_clues[r], &rowk);
        let (tc, fc) = arrangements(&self.col_clues[c], &colk);
        let pr = if tr == 0 {
            0.5
        } else {
            fr[c] as f64 / tr as f64
        };
        let pc = if tc == 0 {
            0.5
        } else {
            fc[r] as f64 / tc as f64
        };
        let a = pr * pc;
        let b = (1.0 - pr) * (1.0 - pc);
        if a + b <= 0.0 {
            0.5
        } else {
            a / (a + b)
        }
    }

    pub fn unknown(&self) -> usize {
        self.known.iter().filter(|&&k| k == 0).count()
    }

    pub fn score(&self) -> [i32; 2] {
        self.score
    }

    fn cell_name(i: usize) -> String {
        format!("{}{}", (b'a' + (i % N) as u8) as char, i / N + 1)
    }
}

impl Game for Nonogram {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        if self.pending.is_some() {
            return vec![Move::Verify];
        }
        let mut v = Vec::new();
        for i in 0..N * N {
            if self.known[i] == 0 {
                v.push(Move::Claim(i as u8, true));
                v.push(Move::Claim(i as u8, false));
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Claim(c, f) => self.pending = Some((*c, *f)),
            Move::Verify => {
                let (c, f) = self.pending.take().unwrap();
                let truth = self.sol[c as usize];
                self.known[c as usize] = if truth { 1 } else { 2 };
                self.score[self.side.idx()] += if truth == f { 1 } else { -2 };
                self.side = self.side.other();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.unknown() == 0 && self.pending.is_none() {
            return Some(by_score(self.score[0], self.score[1]));
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let mut d = (self.score[p.idx()] - self.score[p.other().idx()]) as f64;
        if let Some((c, f)) = self.pending {
            let pf = self.p_filled(c as usize);
            let pc = if f { pf } else { 1.0 - pf };
            let ev = pc - 2.0 * (1.0 - pc);
            d += if self.side == p { ev } else { -ev };
        }
        (d * 1000.0) as i32
    }

    fn render(&self) -> String {
        let width = self.row_clues.iter().map(|c| c.len()).max().unwrap_or(1);
        let height = self.col_clues.iter().map(|c| c.len()).max().unwrap_or(1);
        let mut s = format!(
            "Nonogram 8×8 — J1 {} p., J2 {} p.; necunoscute: {}\n",
            self.score[0],
            self.score[1],
            self.unknown()
        );
        for h in 0..height {
            s.push_str(&" ".repeat(width * 3 + 1));
            for c in 0..N {
                let cl = &self.col_clues[c];
                let off = height - cl.len();
                if h >= off {
                    s.push_str(&format!("{:>2} ", cl[h - off]));
                } else {
                    s.push_str("   ");
                }
            }
            s.push('\n');
        }
        s.push_str(&" ".repeat(width * 3 + 1));
        for c in 0..N {
            s.push_str(&format!(" {} ", (b'a' + c as u8) as char));
        }
        s.push('\n');
        for r in 0..N {
            let cl = &self.row_clues[r];
            let mut t = String::new();
            for _ in 0..width - cl.len() {
                t.push_str("   ");
            }
            for x in cl {
                t.push_str(&format!("{x:>2} "));
            }
            s.push_str(&format!("{t} "));
            for c in 0..N {
                s.push_str(match self.known[r * N + c] {
                    1 => " # ",
                    2 => " . ",
                    _ => " _ ",
                });
            }
            s.push_str(&format!(" {}\n", r + 1));
        }
        if let Some((c, f)) = self.pending {
            s.push_str(&format!(
                "Revendicare în așteptare: {} {}.\n",
                Self::cell_name(c as usize),
                if f { "plină" } else { "goală" }
            ));
        }
        let v = verdict(self.outcome());
        if v.is_empty() {
            s.push_str(&format!(
                "La mutare: Jucător {} — {}\n",
                self.side.idx() + 1,
                if self.pending.is_some() {
                    "verific"
                } else {
                    "a1 (plină) / a1x (goală)"
                }
            ));
        } else {
            s.push_str(&v);
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Claim(c, true) => Self::cell_name(*c as usize),
            Move::Claim(c, false) => format!("{}x", Self::cell_name(*c as usize)),
            Move::Verify => "verific".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q5_util::testkit::check_game;

    #[test]
    fn clues_and_arrangements() {
        assert_eq!(
            clues_of(&[true, true, false, true, false, false, false, false]),
            vec![2, 1]
        );
        assert_eq!(clues_of(&[false; 8]), vec![0]);
        let (t, f) = arrangements(&[8], &[0; 8]);
        assert_eq!(t, 1);
        assert!(f.iter().all(|&x| x == 1));
        // [4,3] pe 8 căsuțe: o singură așezare (4 + 1 + 3)
        let (t, _) = arrangements(&[4, 3], &[0; 8]);
        assert_eq!(t, 1);
        // căsuță cunoscută goală exclude aranjamente
        let (t0, _) = arrangements(&[2], &[0; 8]);
        let mut k = [0u8; 8];
        k[0] = 2;
        let (t1, _) = arrangements(&[2], &k);
        assert!(t1 < t0);
        // căsuță cunoscută plină
        let mut k = [0u8; 8];
        k[3] = 1;
        let (t2, f2) = arrangements(&[2], &k);
        assert_eq!(t2, 2);
        assert_eq!(f2[3], 2);
    }

    #[test]
    fn the_true_cell_has_positive_probability_and_scoring_works() {
        let mut g = Nonogram::with_seed(5);
        for i in 0..N * N {
            let p = g.p_filled(i);
            assert!((0.0..=1.0).contains(&p));
            if g.sol[i] {
                assert!(p > 0.0, "celulă plină cu probabilitate 0");
            } else {
                assert!(p < 1.0);
            }
        }
        let truth = g.sol[0];
        g.apply(&Move::Claim(0, truth));
        g.apply(&Move::Verify);
        assert_eq!(g.score[0], 1);
        let truth = g.sol[1];
        g.apply(&Move::Claim(1, !truth));
        g.apply(&Move::Verify);
        assert_eq!(g.score[1], -2);
    }

    #[test]
    fn ai_never_peeks_at_the_solution() {
        // două soluții diferite cu aceleași indicii și aceeași stare vizibilă => aceeași mutare AI
        let a = Nonogram::with_seed(11);
        let mut b = a.clone();
        // alt aranjament nu există în general; verificăm măcar că soluția nu intră în evaluare:
        // inversăm o căsuță în `sol` fără a atinge indiciile
        b.sol[10] = !b.sol[10];
        for seed in 1..6 {
            assert_eq!(
                polytabletop_core::GameObj::ai_move(&a, seed),
                polytabletop_core::GameObj::ai_move(&b, seed)
            );
        }
    }

    #[test]
    fn random_and_ai_games() {
        check_game(Nonogram::with_seed, 3, 500);
    }
}
