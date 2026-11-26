//! Senet (Egiptul antic): cursă pe o tablă de 30 de pătrate parcursă în S, 5 piese pe jucător,
//! în reconstituirea lui Kendall (cea mai răspândită).
//!
//! * Pătratele 1–30 se parcurg în ordine crescătoare. La început piesele alternează pe
//!   pătratele 1–10: Jucătorul 1 pe 1, 3, 5, 7, 9; Jucătorul 2 pe 2, 4, 6, 8, 10.
//! * Aruncare: 4 bețe cu două fețe (50% fiecare): numărul de fețe marcate în sus = 1–4,
//!   zero fețe = 5. **1, 4 și 5** dau o nouă aruncare; 2 și 3 încheie tura.
//! * O piesă înaintează exact cât arată aruncarea. Nu poți ateriza pe o piesă proprie.
//!   Aterizarea pe o piesă adversă o schimbă cu a ta (ea ajunge unde erai), dacă nu e
//!   protejată: protejate sunt piesele vecine cu o altă piesă a aceluiași adversar și cele
//!   de pe pătratele 15, 26, 28, 29. Nu poți trece peste trei piese adverse consecutive.
//! * Pătratul 26 (Casa Fericirii) e obligatoriu: nicio piesă nu trece de el fără să se
//!   oprească acolo. Din 26 se iese cu 5; pătratul 27 (apa) trimite piesa înapoi pe 15
//!   (sau pe cel mai apropiat pătrat liber înainte de 15). Din 28 se iese cu exact 3,
//!   din 29 cu exact 2, din 30 cu exact 1 (se poate și avansa spre 29/30 cu aruncări mai
//!   mici).
//! * Dacă nicio mutare nu e posibilă, treci. Câștigă primul care scoate toate cele 5 piese.
//!
//! Limită: nu există reguli originale atestate; se folosește reconstituirea modernă.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const PIECES: usize = 5;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Throw,
    /// de la pătratul dat (ieșirea = spre pătratul 31)
    Step(u8),
    Pass,
}

#[derive(Clone)]
pub struct Senet {
    b: [u8; 32], // 1..=30
    off: [u8; 2],
    side: Player,
    val: Option<u8>,
    rng: Rng,
}

impl Default for Senet {
    fn default() -> Self {
        Self::new()
    }
}

fn safe_sq(s: usize) -> bool {
    matches!(s, 15 | 26 | 28 | 29)
}

impl Senet {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut b = [0u8; 32];
        for (s, v) in b.iter_mut().enumerate().take(11).skip(1) {
            *v = if s % 2 == 1 { 1 } else { 2 };
        }
        Senet {
            b,
            off: [0, 0],
            side: Player::One,
            val: None,
            rng: Rng::new(seed),
        }
    }

    pub fn off(&self, p: Player) -> u8 {
        self.off[p.idx()]
    }

    /// Destinația mutării de la `s` cu aruncarea `v`: `Some(31)` = ieșire, `None` = ilegal.
    fn target(&self, s: usize, v: u8, me: u8) -> Option<usize> {
        let en = 3 - me;
        let v = v as usize;
        let t = match s {
            26 => {
                if v == 5 {
                    return Some(31);
                }
                26 + v
            }
            28 => {
                return match v {
                    3 => Some(31),
                    1 => Some(29),
                    2 => Some(30),
                    _ => None,
                }
                .filter(|&t| t == 31 || self.b[t] != me);
            }
            29 => {
                return match v {
                    2 => Some(31),
                    1 => Some(30),
                    _ => None,
                }
                .filter(|&t| t == 31 || self.b[t] != me);
            }
            30 => {
                return if v == 1 { Some(31) } else { None };
            }
            _ => s + v,
        };
        if s < 26 && t > 26 {
            return None; // trebuie să se oprească pe 26
        }
        if t > 30 {
            return None;
        }
        if self.b[t] == me {
            return None;
        }
        if self.b[t] == en {
            if safe_sq(t) {
                return None;
            }
            let left = t > 1 && self.b[t - 1] == en;
            let right = t < 30 && self.b[t + 1] == en;
            if left || right {
                return None;
            }
        }
        // blocadă de trei piese adverse consecutive între s și t
        for x in (s + 1)..t {
            if x + 2 < t && self.b[x] == en && self.b[x + 1] == en && self.b[x + 2] == en {
                return None;
            }
        }
        Some(t)
    }

    fn progress(&self, p: usize) -> i32 {
        let tag = p as u8 + 1;
        let on: i32 = (1..=30)
            .filter(|&s| self.b[s] == tag)
            .map(|s| s as i32)
            .sum();
        on + 40 * self.off[p] as i32
    }
}

impl Game for Senet {
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
        let me = self.side.idx() as u8 + 1;
        let mut out: Vec<Move> = (1..=30usize)
            .filter(|&s| self.b[s] == me && self.target(s, v, me).is_some())
            .map(|s| Move::Step(s as u8))
            .collect();
        if out.is_empty() {
            out.push(Move::Pass);
        }
        out
    }

    fn apply(&mut self, m: &Move) {
        if *m == Move::Throw {
            let up = self.rng.below(16).count_ones() as u8;
            self.val = Some(if up == 0 { 5 } else { up });
            return;
        }
        let v = self.val.take().unwrap_or(0);
        let me = self.side.idx() as u8 + 1;
        if let Move::Step(s) = *m {
            let s = s as usize;
            let t = self.target(s, v, me).expect("legal");
            self.b[s] = 0;
            if t == 31 {
                self.off[me as usize - 1] += 1;
            } else if t == 27 {
                // apa: înapoi pe 15 sau pe primul pătrat liber înainte de 15
                let mut d = 15;
                while d > 1 && self.b[d] != 0 {
                    d -= 1;
                }
                self.b[d] = me;
            } else {
                let prev = self.b[t];
                self.b[t] = me;
                if prev != 0 {
                    self.b[s] = prev; // schimb
                }
            }
        }
        if !matches!(v, 1 | 4 | 5) {
            self.side = self.side.other();
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        for p in [Player::One, Player::Two] {
            if self.off[p.idx()] as usize == PIECES {
                return Some(Outcome::Win(p));
            }
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        self.progress(p.idx()) - self.progress(p.other().idx())
    }

    fn render(&self) -> String {
        let mut s = String::new();
        // trei rânduri: 1–10, 20–11, 21–30
        let cell = |sq: usize| -> String {
            let ch = match self.b[sq] {
                1 => 'X',
                2 => 'O',
                _ => '.',
            };
            format!("{:>2}{}", sq, ch)
        };
        for range in [
            (1..=10usize).collect::<Vec<_>>(),
            (11..=20usize).rev().collect::<Vec<_>>(),
            (21..=30usize).collect::<Vec<_>>(),
        ] {
            let line: Vec<String> = range.into_iter().map(cell).collect();
            s.push_str(&line.join(" "));
            s.push('\n');
        }
        s.push_str(&format!(
            "X = J1, O = J2. Ieșite: J1 {}, J2 {}. Speciale: 15 renaștere, 26 fericire, 27 apă, 28/29/30 ieșire cu 3/2/1.\n",
            self.off[0], self.off[1]
        ));
        if let Some(v) = self.val {
            s.push_str(&format!("Aruncare: {v}\n"));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match *m {
            Move::Throw => "arunca".into(),
            Move::Pass => "pas".into(),
            Move::Step(s) => {
                let me = self.side.idx() as u8 + 1;
                let t = self
                    .target(s as usize, self.val.unwrap_or(0), me)
                    .unwrap_or(31);
                if t == 31 {
                    format!("{s}-iese")
                } else {
                    format!("{s}-{t}")
                }
            }
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty() -> Senet {
        let mut g = Senet::with_seed(3);
        g.b = [0; 32];
        g
    }

    #[test]
    fn start_position_alternates() {
        let g = Senet::with_seed(1);
        assert_eq!(g.b[1], 1);
        assert_eq!(g.b[2], 2);
        assert_eq!(g.b[9], 1);
        assert_eq!(g.b[10], 2);
        assert_eq!(Game::legal_moves(&g), vec![Move::Throw]);
    }

    #[test]
    fn swap_with_an_unprotected_enemy() {
        let mut g = empty();
        g.b[3] = 1;
        g.b[5] = 2;
        g.val = Some(2);
        g.apply(&Move::Step(3));
        assert_eq!(g.b[5], 1);
        assert_eq!(g.b[3], 2);
    }

    #[test]
    fn adjacent_enemies_are_protected() {
        let mut g = empty();
        g.b[3] = 1;
        g.b[5] = 2;
        g.b[6] = 2;
        g.val = Some(2);
        assert_eq!(Game::legal_moves(&g), vec![Move::Pass]);
    }

    #[test]
    fn three_in_a_row_blocks_passage() {
        let mut g = empty();
        g.b[1] = 1;
        g.b[3] = 2;
        g.b[4] = 2;
        g.b[5] = 2;
        g.val = Some(5); // 1 → 6 trece peste blocadă
        assert_eq!(Game::legal_moves(&g), vec![Move::Pass]);
    }

    #[test]
    fn must_stop_on_26_and_water_returns_to_15() {
        let mut g = empty();
        g.b[24] = 1;
        g.val = Some(4); // 28 > 26
        assert_eq!(Game::legal_moves(&g), vec![Move::Pass]);
        g.val = Some(2); // 26
        assert_eq!(Game::legal_moves(&g), vec![Move::Step(24)]);
        let mut h = empty();
        h.b[26] = 1;
        h.val = Some(1); // 27 → 15
        h.apply(&Move::Step(26));
        assert_eq!(h.b[15], 1);
        assert_eq!(h.b[27], 0);
    }

    #[test]
    fn exits_need_exact_throws() {
        let mut g = empty();
        g.b[26] = 1;
        g.b[28] = 1;
        g.b[29] = 1;
        g.b[30] = 1;
        g.side = Player::One;
        g.val = Some(5);
        assert_eq!(Game::legal_moves(&g), vec![Move::Step(26)]);
        g.val = Some(3);
        assert!(Game::legal_moves(&g).contains(&Move::Step(28)));
        g.val = Some(2);
        assert!(Game::legal_moves(&g).contains(&Move::Step(29)));
        g.val = Some(1);
        assert!(Game::legal_moves(&g).contains(&Move::Step(30)));
        g.apply(&Move::Step(30));
        assert_eq!(g.off(Player::One), 1);
    }

    #[test]
    fn extra_throw_on_1_4_5() {
        let mut g = empty();
        g.b[1] = 1;
        g.val = Some(1);
        g.apply(&Move::Step(1));
        assert_eq!(g.to_move(), Player::One);
        let mut h = empty();
        h.b[1] = 1;
        h.val = Some(2);
        h.apply(&Move::Step(1));
        assert_eq!(h.to_move(), Player::Two);
    }

    #[test]
    fn random_games_terminate() {
        for seed in 1..=10u64 {
            let mut g = Senet::with_seed(seed);
            let mut rng = Rng::new(seed + 9);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = Game::legal_moves(&g);
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 200_000, "{}", g.render());
            }
        }
    }
}
