//! Tâb (Egipt / Orientul Mijlociu): joc de cursă cu 4 bețe-zaruri, variantă simplificată
//! documentată pe tablă 4 × 7 (fără un consens asupra dimensiunii și a detaliilor).
//!
//! * Fiecare jucător are 7 piese, la început „în rezervă”. Se aruncă 4 bețe cu două fețe
//!   (50% fiecare): numărul de fețe plate în sus = 1, 2, 3, 4; nicio față plată = 6.
//!   **1, 4 și 6** dau o nouă aruncare.
//! * O piesă intră în joc doar cu un 1 („tâb”), pe primul pătrat al drumului ei.
//! * Drumul unui jucător parcurge rândurile în zigzag: rândul lui (stânga → dreapta), al
//!   doilea (dreapta → stânga), al treilea (stânga → dreapta), al patrulea (dreapta →
//!   stânga); apoi piesa iese. Jucătorul 2 pornește din rândul opus, rotit cu 180°.
//! * Aterizarea pe o piesă adversă o captură (piesa se întoarce în rezervă), cu excepția
//!   pieselor aflate în rândul de plecare al proprietarului lor (sigure). Nu poți ateriza
//!   pe o piesă proprie. Dacă nu ai nicio mutare, treci.
//! * Câștigă primul care scoate toate cele 7 piese (ieșirea se face cu orice aruncare care
//!   depășește sfârșitul drumului).
//!
//! Limită: regulile istorice variază mult; aceasta este o simplificare (fără rânduri
//! suplimentare și fără regula „ultimului rând”).

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const PIECES: u8 = 7;
const LEN: i8 = 28;
const OFF: i8 = LEN;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Throw,
    Enter,
    /// piesa de la indicele de drum dat
    Step(i8),
    Pass,
}

#[derive(Clone)]
pub struct Tab {
    /// progres: -1 în rezervă, 0..27 pe drum, 28 ieșită
    p: [[i8; PIECES as usize]; 2],
    side: Player,
    val: Option<u8>,
    rng: Rng,
}

impl Default for Tab {
    fn default() -> Self {
        Self::new()
    }
}

/// Pătratul de pe tablă (rând * 7 + coloană) pentru progresul `i` al jucătorului `p`.
fn square(p: usize, i: i8) -> usize {
    let i = i as usize;
    let row = i / 7;
    let col = if row.is_multiple_of(2) {
        i % 7
    } else {
        6 - i % 7
    };
    if p == 0 {
        row * 7 + col
    } else {
        (3 - row) * 7 + (6 - col)
    }
}

fn sq_name(s: usize) -> String {
    format!("{}{}", (b'a' + (s % 7) as u8) as char, s / 7 + 1)
}

impl Tab {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Tab {
            p: [[-1; PIECES as usize]; 2],
            side: Player::One,
            val: None,
            rng: Rng::new(seed),
        }
    }

    pub fn exited(&self, p: Player) -> usize {
        self.p[p.idx()].iter().filter(|&&x| x == OFF).count()
    }

    /// Piesa adversă (indice) de pe pătratul `sq`, dacă există.
    fn enemy_at(&self, me: usize, sq: usize) -> Option<usize> {
        let o = 1 - me;
        (0..PIECES as usize).find(|&k| {
            let x = self.p[o][k];
            (0..LEN).contains(&x) && square(o, x) == sq
        })
    }

    fn own_at(&self, me: usize, sq: usize) -> bool {
        self.p[me]
            .iter()
            .any(|&x| (0..LEN).contains(&x) && square(me, x) == sq)
    }

    fn can_land(&self, me: usize, to: i8) -> bool {
        let sq = square(me, to);
        !self.own_at(me, sq)
    }

    fn progress(&self, p: usize) -> i32 {
        self.p[p]
            .iter()
            .map(|&x| match x {
                -1 => 0,
                OFF => 40,
                v => v as i32 + 1,
            })
            .sum()
    }
}

impl Game for Tab {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        let Some(v) = self.val else {
            return vec![Move::Throw];
        };
        let me = self.side.idx();
        let mut out = Vec::new();
        if v == 1 && self.p[me].contains(&-1) && self.can_land(me, 0) {
            out.push(Move::Enter);
        }
        let mut seen = Vec::new();
        for &x in &self.p[me] {
            if !(0..LEN).contains(&x) || seen.contains(&x) {
                continue;
            }
            seen.push(x);
            let to = x + v as i8;
            if to >= OFF || self.can_land(me, to) {
                out.push(Move::Step(x));
            }
        }
        if out.is_empty() {
            out.push(Move::Pass);
        }
        out
    }

    fn apply(&mut self, m: &Move) {
        let me = self.side.idx();
        let o = 1 - me;
        if *m == Move::Throw {
            let up = self.rng.below(16).count_ones() as u8;
            self.val = Some(if up == 0 { 6 } else { up });
            return;
        }
        let v = self.val.take().unwrap_or(0);
        match *m {
            Move::Enter | Move::Step(_) => {
                let from = if let Move::Step(x) = *m { x } else { -1 };
                let to = if from == -1 { 0 } else { from + v as i8 };
                let k = self.p[me].iter().position(|&x| x == from).unwrap();
                if to >= OFF {
                    self.p[me][k] = OFF;
                } else {
                    self.p[me][k] = to;
                    let sq = square(me, to);
                    if let Some(e) = self.enemy_at(me, sq) {
                        let ei = self.p[o][e];
                        // pătrate sigure: rândul de plecare al proprietarului (primele 7)
                        if ei >= 7 {
                            self.p[o][e] = -1;
                        }
                    }
                }
            }
            _ => {}
        }
        if !matches!(v, 1 | 4 | 6) {
            self.side = self.side.other();
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        for p in [Player::One, Player::Two] {
            if self.exited(p) == PIECES as usize {
                return Some(Outcome::Win(p));
            }
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        self.progress(p.idx()) - self.progress(p.other().idx())
    }

    fn render(&self) -> String {
        let mut cells = [' '; 28];
        for (pl, ch) in [(0usize, 'X'), (1usize, 'O')] {
            for &x in &self.p[pl] {
                if (0..LEN).contains(&x) {
                    cells[square(pl, x)] = ch;
                }
            }
        }
        let mut s = String::new();
        for r in (0..4).rev() {
            s.push_str(&format!("{} ", r + 1));
            for c in 0..7 {
                let ch = cells[r * 7 + c];
                s.push(' ');
                s.push(if ch == ' ' { '.' } else { ch });
            }
            s.push('\n');
        }
        s.push_str("   a b c d e f g\n");
        let stock = |p: usize| self.p[p].iter().filter(|&&x| x == -1).count();
        s.push_str(&format!(
            "Rezervă: J1 {}, J2 {}; ieșite: J1 {}, J2 {}. Rândul 1 = plecarea lui J1, rândul 4 = a lui J2 (sigure).\n",
            stock(0),
            stock(1),
            self.exited(Player::One),
            self.exited(Player::Two)
        ));
        if let Some(v) = self.val {
            s.push_str(&format!("Aruncare: {v}\n"));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        let me = self.side.idx();
        match *m {
            Move::Throw => "arunca".into(),
            Move::Enter => format!("intra-{}", sq_name(square(me, 0))),
            Move::Step(x) => {
                let to = x + self.val.unwrap_or(0) as i8;
                if to >= OFF {
                    format!("{}-iese", sq_name(square(me, x)))
                } else {
                    format!("{}-{}", sq_name(square(me, x)), sq_name(square(me, to)))
                }
            }
            Move::Pass => "pas".into(),
        }
    }

    /// Aruncarea următoare e deja decisă doar după ce a fost cerută; căutarea de adâncime 1
    /// nu aruncă niciodată în interiorul simulării.
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fresh() -> Tab {
        Tab::with_seed(7)
    }

    #[test]
    fn starts_with_a_throw_and_needs_a_one_to_enter() {
        let mut g = fresh();
        assert_eq!(Game::legal_moves(&g), vec![Move::Throw]);
        g.val = Some(3);
        assert_eq!(Game::legal_moves(&g), vec![Move::Pass]);
        g.val = Some(1);
        assert_eq!(Game::legal_moves(&g), vec![Move::Enter]);
    }

    #[test]
    fn zigzag_path_and_rotation() {
        assert_eq!(square(0, 0), 0);
        assert_eq!(square(0, 6), 6);
        assert_eq!(square(0, 7), 7 + 6);
        assert_eq!(square(0, 14), 14);
        assert_eq!(square(1, 0), 3 * 7 + 6);
        assert_eq!(square(1, 7), 2 * 7);
    }

    #[test]
    fn capture_outside_the_home_row_only() {
        let mut g = fresh();
        g.p[0][0] = 9; // J1 în rândul 2 (indice 9 → rând 1, col 4)
        g.p[1][0] = 0; // J2 pe plecarea lui; pătratul (3,6) nu e al lui J1
                       // J2 pe un pătrat din rândul 3 al drumului lui (indice 15 → rând 2)
        let sq = square(0, 9);
        // găsim progresul lui J2 care ajunge pe același pătrat
        let ei = (0..LEN).find(|&i| square(1, i) == sq).unwrap();
        g.p[1][1] = ei;
        g.side = Player::One;
        g.val = Some(1);
        // J1 mută 8 → 9? îl punem la 8 și sărim peste
        g.p[0][0] = 8;
        g.apply(&Move::Step(8));
        if ei >= 7 {
            assert_eq!(g.p[1][1], -1);
        } else {
            assert_eq!(g.p[1][1], ei);
        }
    }

    #[test]
    fn extra_throw_on_1_4_6() {
        let mut g = fresh();
        g.val = Some(3);
        g.apply(&Move::Pass);
        assert_eq!(g.to_move(), Player::Two);
        let mut h = fresh();
        h.val = Some(4);
        h.apply(&Move::Pass);
        assert_eq!(h.to_move(), Player::One);
    }

    #[test]
    fn exit_ends_the_game_when_all_out() {
        let mut g = fresh();
        g.p[0] = [OFF; 7];
        g.p[0][0] = 26;
        g.side = Player::One;
        g.val = Some(3);
        g.apply(&Move::Step(26));
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn random_games_terminate() {
        use polytabletop_core::GameObj;
        for seed in 1..=10u64 {
            let mut g = Tab::with_seed(seed);
            let mut rng = Rng::new(seed + 50);
            let mut n = 0;
            while GameObj::outcome(&g).is_none() {
                let m = Game::legal_moves(&g);
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 100_000);
            }
        }
    }
}
