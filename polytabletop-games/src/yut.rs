//! Yut (Yunnori, Coreea) — joc de întrecere cu patru bețe-zar, 2 jucători.
//!
//! Reguli folosite (tradiționale, simplificate):
//! * Patru bețe, fiecare cade cu fața plată sau rotundă (50%). Plate: 1 = Do (1),
//!   2 = Gae (2), 3 = Geol (3), 4 = Yut (4, aruncă din nou), 0 = Mo (5, aruncă din nou).
//!   Nu există Back-do (pas înapoi) în această variantă.
//! * Fiecare jucător are 4 piese („mal”). Rezultatele aruncate se păstrează și se
//!   pot folosi în orice ordine, fiecare mută o piesă (sau un grup).
//! * Tabla: cerc de 19 noduri (1–19, nodul 0 = start/final, colțuri 5, 10, 15),
//!   scurtătura 20–24 pornind din colțul 5 (centrul = 22, apoi 23, 24, colțul 15) și
//!   scurtătura 25–28 din colțul 10 (…26, centrul 22, 27, 28, final). Scurtătura se
//!   ia doar când piesa **începe** mutarea dintr-un colț (5 sau 10); din centru se
//!   merge direct spre final. Piesa care trece de ultimul nod a ieșit (acasă).
//! * Piesele proprii de pe același nod formează un grup care se mută împreună.
//!   Aterizarea pe piese adverse le trimite la start și dă o aruncare în plus.
//! * Câștigă cine scoate primul toate cele 4 piese.
//!
//! Zarul este aruncat în stare (Rng propriu); mutarea forțată `arunc` e separată,
//! deci AI-ul (AlphaBeta(1)) nu vede aruncările viitoare.

//!
//! Verificare surse (nov. 2026: en.wikipedia.org/wiki/Yut): valorile Do 1, Gae 2, Geol 3,
//! Yut 4, Mo 5, aruncare în plus la Yut/Mo și la captură, grupuri, scurtături din colțuri și
//! centru, ieșirea prin depășirea punctului de start și 4 piese/echipă corespund. Lipsește
//! Back-do (mutarea înapoi cu un pas), omis intenționat și documentat mai sus.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const START: u8 = 255;
const FIN: u8 = 100;
const NAMES: [&str; 6] = ["", "do", "gae", "geol", "yut", "mo"];

/// Nodurile pe care le parcurge o piesă aflată pe `n`, ultimul fiind `FIN`.
pub fn route(n: u8) -> Vec<u8> {
    let tail_ring = |from: u8| -> Vec<u8> { (from..=19).chain(std::iter::once(FIN)).collect() };
    match n {
        START => tail_ring(1),
        5 => {
            let mut v = vec![20, 21, 22, 23, 24];
            v.extend(tail_ring(15));
            v
        }
        10 => vec![25, 26, 22, 27, 28, FIN],
        22 => vec![27, 28, FIN],
        20 => {
            let mut v = vec![21, 22, 23, 24];
            v.extend(tail_ring(15));
            v
        }
        21 => {
            let mut v = vec![22, 23, 24];
            v.extend(tail_ring(15));
            v
        }
        23 => {
            let mut v = vec![24];
            v.extend(tail_ring(15));
            v
        }
        24 => tail_ring(15),
        25 => vec![26, 22, 27, 28, FIN],
        26 => vec![22, 27, 28, FIN],
        27 => vec![28, FIN],
        28 => vec![FIN],
        x => tail_ring(x + 1),
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Move {
    /// 1..=5
    pub result: u8,
    /// nodul sursă (`START` = piesă la start) sau `0` pentru „arunc”
    pub from: u8,
    pub throw: bool,
}

#[derive(Clone)]
pub struct Yut {
    pieces: [[u8; 4]; 2],
    side: Player,
    pending: Vec<u8>,
    throws_left: u8,
    rng: Rng,
    last_throw: Option<u8>,
}

impl Default for Yut {
    fn default() -> Self {
        Self::new()
    }
}

impl Yut {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        Yut {
            pieces: [[START; 4]; 2],
            side: Player::One,
            pending: Vec::new(),
            throws_left: 1,
            rng: Rng::new(seed),
            last_throw: None,
        }
    }
    fn remaining(&self, p: Player) -> i32 {
        self.pieces[p.idx()]
            .iter()
            .map(|&n| if n == FIN { 0 } else { route(n).len() as i32 })
            .sum()
    }
    fn target(from: u8, k: u8) -> u8 {
        let r = route(from);
        r.get(k as usize - 1).copied().unwrap_or(FIN)
    }
    fn node_text(n: u8) -> String {
        if n == START {
            "start".into()
        } else {
            n.to_string()
        }
    }
}

impl Game for Yut {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        if self.throws_left > 0 {
            return vec![Move {
                result: 0,
                from: 0,
                throw: true,
            }];
        }
        let mut v = Vec::new();
        let mut res: Vec<u8> = self.pending.clone();
        res.sort_unstable();
        res.dedup();
        let mut froms: Vec<u8> = self.pieces[self.side.idx()]
            .iter()
            .copied()
            .filter(|&n| n != FIN)
            .collect();
        froms.sort_unstable();
        froms.dedup();
        for &r in &res {
            for &f in &froms {
                v.push(Move {
                    result: r,
                    from: f,
                    throw: false,
                });
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        if m.throw {
            let f = self.rng.below(16).count_ones() as u8;
            let r = if f == 0 { 5 } else { f };
            self.last_throw = Some(r);
            self.pending.push(r);
            self.throws_left -= 1;
            if r >= 4 {
                self.throws_left += 1;
            }
            return;
        }
        let me = self.side.idx();
        let pos = self.pending.iter().position(|&r| r == m.result).unwrap();
        self.pending.remove(pos);
        let to = Self::target(m.from, m.result);
        // mută grupul (sau o singură piesă de la start)
        let mut moved = 0;
        for p in self.pieces[me].iter_mut() {
            if *p == m.from && (m.from != START || moved == 0) {
                *p = to;
                moved += 1;
            }
        }
        if to != FIN {
            let mut captured = false;
            for p in self.pieces[1 - me].iter_mut() {
                if *p == to {
                    *p = START;
                    captured = true;
                }
            }
            if captured {
                self.throws_left += 1;
            }
        }
        if self.outcome().is_some() {
            return;
        }
        if self.throws_left == 0 && self.pending.is_empty() {
            self.side = self.side.other();
            self.throws_left = 1;
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        for p in [Player::One, Player::Two] {
            if self.pieces[p.idx()].iter().all(|&n| n == FIN) {
                return Some(Outcome::Win(p));
            }
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let mut s = 10 * (self.remaining(p.other()) - self.remaining(p));
        // piesele izolate ale adversarului aflate în fața mea sunt ținte; ale mele, riscuri
        let me = &self.pieces[p.idx()];
        let op = &self.pieces[p.other().idx()];
        for &a in me.iter().filter(|&&n| n != START && n != FIN) {
            for &b in op.iter().filter(|&&n| n != START && n != FIN) {
                for k in 1..=5u8 {
                    if Self::target(b, k) == a {
                        s -= 2;
                    }
                    if Self::target(a, k) == b {
                        s += 2;
                    }
                }
            }
        }
        s
    }

    fn render(&self) -> String {
        let mut s = String::from(
            "Yut — cerc 1–19 (colțuri 5, 10, 15), scurtături 20–24 (din 5) și 25–28 (din 10), centru 22\n",
        );
        for p in [Player::One, Player::Two] {
            let pc = &self.pieces[p.idx()];
            let start = pc.iter().filter(|&&n| n == START).count();
            let home = pc.iter().filter(|&&n| n == FIN).count();
            let mut on: Vec<u8> = pc
                .iter()
                .copied()
                .filter(|&n| n != START && n != FIN)
                .collect();
            on.sort_unstable();
            s.push_str(&format!(
                "J{}: la start {}, pe tablă {:?}, acasă {}\n",
                p.idx() + 1,
                start,
                on,
                home
            ));
        }
        if let Some(r) = self.last_throw {
            s.push_str(&format!("Ultima aruncare: {} ({})\n", NAMES[r as usize], r));
        }
        let pend: Vec<&str> = self.pending.iter().map(|&r| NAMES[r as usize]).collect();
        s.push_str(&format!("Rezultate de folosit: {}\n", pend.join(", ")));
        if self.outcome().is_none() {
            s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        if m.throw {
            "arunc".into()
        } else {
            format!("{}:{}", NAMES[m.result as usize], Self::node_text(m.from))
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fresh() -> Yut {
        Yut::with_seed(3)
    }

    #[test]
    fn routes_end_in_fin_and_lengths() {
        assert_eq!(route(START).len(), 20);
        assert_eq!(route(5).len(), 11);
        assert_eq!(route(10).len(), 6);
        assert_eq!(route(22).len(), 3);
        assert_eq!(route(19), vec![FIN]);
        for n in (1..=19).chain(20..=28) {
            assert_eq!(*route(n).last().unwrap(), FIN);
        }
    }

    #[test]
    fn shortcut_only_when_starting_on_corner() {
        assert_eq!(Yut::target(5, 1), 20);
        assert_eq!(Yut::target(3, 2), 5); // oprește pe colț
        assert_eq!(Yut::target(3, 3), 6); // trece peste colț fără scurtătură
        assert_eq!(Yut::target(10, 3), 22);
        assert_eq!(Yut::target(22, 1), 27);
        assert_eq!(Yut::target(21, 3), 24);
    }

    #[test]
    fn throw_distribution_and_extra() {
        let mut g = fresh();
        let mut counts = [0u32; 6];
        for _ in 0..16000 {
            let mut c = g.clone();
            c.apply(&Move {
                result: 0,
                from: 0,
                throw: true,
            });
            counts[c.last_throw.unwrap() as usize] += 1;
            if c.last_throw.unwrap() >= 4 {
                assert_eq!(c.throws_left, 1);
            } else {
                assert_eq!(c.throws_left, 0);
            }
            g.rng.next_u64();
        }
        let n = counts.iter().sum::<u32>() as f64;
        assert!((counts[2] as f64 / n - 0.375).abs() < 0.03);
        assert!((counts[1] as f64 / n - 0.25).abs() < 0.03);
        assert!((counts[5] as f64 / n - 0.0625).abs() < 0.02);
    }

    #[test]
    fn capture_sends_home_and_gives_extra_throw() {
        let mut g = fresh();
        g.throws_left = 0;
        g.pending = vec![2];
        g.pieces[0] = [3, START, START, START];
        g.pieces[1] = [5, 5, START, START];
        g.apply(&Move {
            result: 2,
            from: 3,
            throw: false,
        });
        assert_eq!(g.pieces[1], [START, START, START, START]);
        assert_eq!(g.throws_left, 1);
        assert_eq!(g.side, Player::One);
    }

    #[test]
    fn stack_moves_together() {
        let mut g = fresh();
        g.throws_left = 0;
        g.pending = vec![3];
        g.pieces[0] = [7, 7, START, START];
        g.apply(&Move {
            result: 3,
            from: 7,
            throw: false,
        });
        assert_eq!(g.pieces[0], [10, 10, START, START]);
        assert_eq!(g.side, Player::Two);
    }

    #[test]
    fn start_moves_a_single_piece() {
        let mut g = fresh();
        g.throws_left = 0;
        g.pending = vec![4];
        g.apply(&Move {
            result: 4,
            from: START,
            throw: false,
        });
        assert_eq!(g.pieces[0].iter().filter(|&&n| n == 4).count(), 1);
        assert_eq!(g.pieces[0].iter().filter(|&&n| n == START).count(), 3);
    }

    #[test]
    fn finishing_all_wins() {
        let mut g = fresh();
        g.throws_left = 0;
        g.pending = vec![5];
        g.pieces[0] = [FIN, FIN, FIN, 19];
        g.apply(&Move {
            result: 5,
            from: 19,
            throw: false,
        });
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn pieces_conserved_and_random_games_end() {
        for seed in 1..40u64 {
            let mut g = Yut::with_seed(seed);
            let mut rng = Rng::new(seed + 100);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
                assert!(n < 5000);
                assert_eq!(g.pieces[0].len() + g.pieces[1].len(), 8);
            }
        }
    }
}
