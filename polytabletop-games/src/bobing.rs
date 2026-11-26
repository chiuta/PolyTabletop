//! Bo Bing (Pua Tiong Chiu), jocul de Festival al Lunii din Xiamen (China / Taiwan): șase zaruri
//! aruncate într-un castron; premiile (63 de prăjituri de lună) se împart după combinație.
//!
//! * Ierarhie (de la cea mai bună): **Zhuangyuan** (patru sau mai mulți de 4, ori cinci/șase la fel)
//!   = 1 premiu, **Duitang** (scara 1-2-3-4-5-6 sau două triple) = 2, **Sanhong** (exact trei de 4) = 4,
//!   **Sijin** (patru la fel, nu de 4) = 8, **Erju** (exact doi de 4) = 16, **Yixiu** (exact un 4) = 32.
//!   Fără niciun 4 și fără altă combinație nu se câștigă nimic.
//! * Fiecare jucător aruncă pe rând (`arunc`); un premiu din categoria nimerită se ia din rezerva ei
//!   (dacă rezerva e goală, aruncarea nu mai câștigă). Punctele de clasament sunt 32/16/8/4/2/1 pentru
//!   Zhuangyuan … Yixiu. 12 aruncări pe jucător sau până se golește rezerva; câștigă cine are mai multe
//!   puncte. Variantă simplificată: fără „urmărire” (追饼) și fără premii mari pe sub-combinații.
//!   Mutare: `arunc`.

use crate::q5_util::{by_score, verdict};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const THROWS: u8 = 12;
const TIERS: [&str; 6] = ["Zhuangyuan", "Duitang", "Sanhong", "Sijin", "Erju", "Yixiu"];
const POOL: [u8; 6] = [1, 2, 4, 8, 16, 32];
const POINTS: [i32; 6] = [32, 16, 8, 4, 2, 1];

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Move;

#[derive(Clone)]
pub struct Bobing {
    pool: [u8; 6],
    won: [[u8; 6]; 2],
    thrown: [u8; 2],
    side: Player,
    rng: Rng,
    last: String,
}

impl Default for Bobing {
    fn default() -> Self {
        Self::new()
    }
}

/// Categoria (0 = Zhuangyuan … 5 = Yixiu) a unei aruncări de 6 zaruri, dacă există.
pub fn tier(d: &[u8; 6]) -> Option<usize> {
    let mut c = [0u8; 7];
    for &x in d {
        c[x as usize] += 1;
    }
    let max = *c[1..].iter().max().unwrap();
    if c[4] >= 4 || max >= 5 {
        return Some(0);
    }
    let straight = c[1..].iter().all(|&x| x == 1);
    let triples = c[1..].iter().filter(|&&x| x == 3).count() == 2;
    if straight || triples {
        return Some(1);
    }
    if c[4] == 3 {
        return Some(2);
    }
    if max == 4 {
        return Some(3);
    }
    match c[4] {
        2 => Some(4),
        1 => Some(5),
        _ => None,
    }
}

impl Bobing {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Bobing {
            pool: POOL,
            won: [[0; 6]; 2],
            thrown: [0; 2],
            side: Player::One,
            rng: Rng::new(seed),
            last: String::new(),
        }
    }

    pub fn points(&self, p: usize) -> i32 {
        (0..6).map(|t| self.won[p][t] as i32 * POINTS[t]).sum()
    }
}

impl Game for Bobing {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            Vec::new()
        } else {
            vec![Move]
        }
    }

    fn apply(&mut self, _m: &Move) {
        let i = self.side.idx();
        let mut d = [0u8; 6];
        for x in d.iter_mut() {
            *x = self.rng.d6();
        }
        d.sort_unstable();
        self.last = match tier(&d) {
            Some(t) if self.pool[t] > 0 => {
                self.pool[t] -= 1;
                self.won[i][t] += 1;
                format!("J{} aruncă {:?}: {} — ia un premiu.", i + 1, d, TIERS[t])
            }
            Some(t) => format!("J{} aruncă {:?}: {} — rezerva e goală.", i + 1, d, TIERS[t]),
            None => format!("J{} aruncă {:?}: nimic.", i + 1, d),
        };
        self.thrown[i] += 1;
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        if (self.thrown[0] >= THROWS && self.thrown[1] >= THROWS)
            || self.pool.iter().all(|&x| x == 0)
        {
            return Some(by_score(self.points(0), self.points(1)));
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        self.points(p.idx()) - self.points(p.other().idx())
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Bo Bing — aruncări: J1 {}/{THROWS}, J2 {}/{THROWS}. Puncte: J1 {}, J2 {}.\n",
            self.thrown[0],
            self.thrown[1],
            self.points(0),
            self.points(1)
        );
        s.push_str("Rezerva de premii: ");
        for (t, name) in TIERS.iter().enumerate() {
            s.push_str(&format!("{} {} · ", name, self.pool[t]));
        }
        s.push('\n');
        for i in 0..2 {
            s.push_str(&format!("Premii J{}: {:?}\n", i + 1, self.won[i]));
        }
        if !self.last.is_empty() {
            s.push_str(&format!("{}\n", self.last));
        }
        let v = verdict(self.outcome());
        if v.is_empty() {
            s.push_str(&format!(
                "La mutare: Jucător {} — arunc\n",
                self.side.idx() + 1
            ));
        } else {
            s.push_str(&v);
        }
        s
    }

    fn format_move(&self, _m: &Move) -> String {
        "arunc".into()
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
    fn tiers() {
        assert_eq!(tier(&[4, 4, 4, 4, 1, 2]), Some(0));
        assert_eq!(tier(&[2, 2, 2, 2, 2, 5]), Some(0));
        assert_eq!(tier(&[1, 2, 3, 4, 5, 6]), Some(1));
        assert_eq!(tier(&[2, 2, 2, 5, 5, 5]), Some(1));
        assert_eq!(tier(&[4, 4, 4, 1, 2, 3]), Some(2));
        assert_eq!(tier(&[3, 3, 3, 3, 1, 4]), Some(3));
        assert_eq!(tier(&[4, 4, 1, 2, 3, 6]), Some(4));
        assert_eq!(tier(&[4, 1, 1, 2, 3, 6]), Some(5));
        assert_eq!(tier(&[1, 1, 2, 3, 5, 6]), None);
    }

    #[test]
    fn random_and_ai_games() {
        check_game(Bobing::with_seed, 6, 500);
    }
}
