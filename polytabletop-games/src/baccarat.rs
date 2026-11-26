//! Baccarat — Punto Banco (Franța / Italia), în doi, cu pariuri la masă.
//!
//! Cărțile (pachet de 52, A = 1, 2–9 valoarea lor, 10/J/Q/K = 0) se împart automat după regulile fixe
//! ale lui Punto Banco; nu există alegeri la tras. Jucătorii pariază pe rând (J1, apoi J2, care vede pariul lui J1),
//! apoi se împarte (`impart`):
//! * `punto N` / `banco N` / `egal N`, N = 1–3 unități.
//! * Punto plătește 1:1, Banco 1:1 minus 5% comision (aici 19:20), Egal 8:1. La egalitate pariurile Punto/Banco se returnează.
//! * Regula: totalul = suma mod 10; dacă cineva are 8 sau 9 („natural”), nu se mai trage. Punto trage la 0–5 și stă la 6–7;
//!   Banco după tabelul standard (total 0–2 trage; 3 trage dacă a treia carte a lui Punto ≠ 8; 4: dacă 2–7; 5: dacă 4–7; 6: dacă 6–7; 7 stă).
//! * 20 de coupe; câștigă cine are mai multe jetoane (unitatea = 20 de jetoane, deci comisionul e exact).
//!
//! Surse: regulile publice ale Punto Banco.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const UNIT: i32 = 20;
const START: i32 = 20 * UNIT;
const COUPS: u8 = 20;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Side {
    Punto,
    Banco,
    Egal,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mv {
    Bet(Side, u8),
    Deal,
}

fn val(c: u8) -> u8 {
    let r = c % 13 + 1;
    if r >= 10 {
        0
    } else {
        r
    }
}

/// (carți Punto, cărți Banco)
pub fn play_coup(deck: &mut Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    let mut p = vec![deck.pop().unwrap(), deck.pop().unwrap()];
    let mut b = vec![deck.pop().unwrap(), deck.pop().unwrap()];
    let sum = |h: &[u8]| h.iter().map(|&c| val(c)).sum::<u8>() % 10;
    let (pt, bt) = (sum(&p), sum(&b));
    if pt >= 8 || bt >= 8 {
        return (p, b);
    }
    let mut p3 = None;
    if pt <= 5 {
        let c = deck.pop().unwrap();
        p3 = Some(val(c));
        p.push(c);
    }
    let draw = match p3 {
        None => bt <= 5,
        Some(x) => match bt {
            0..=2 => true,
            3 => x != 8,
            4 => (2..=7).contains(&x),
            5 => (4..=7).contains(&x),
            6 => (6..=7).contains(&x),
            _ => false,
        },
    };
    if draw {
        b.push(deck.pop().unwrap());
    }
    (p, b)
}

#[derive(Clone)]
pub struct Baccarat {
    chips: [i32; 2],
    bets: [Option<(Side, u8)>; 2],
    coup: u8,
    rng: Rng,
    log: String,
}

impl Default for Baccarat {
    fn default() -> Self {
        Self::new()
    }
}

impl Baccarat {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        Baccarat {
            chips: [START; 2],
            bets: [None; 2],
            coup: 1,
            rng: Rng::new(seed),
            log: String::new(),
        }
    }
    pub fn chips(&self) -> (i32, i32) {
        (self.chips[0], self.chips[1])
    }
    fn bettor(&self) -> Option<usize> {
        if self.bets[0].is_none() {
            Some(0)
        } else if self.bets[1].is_none() {
            Some(1)
        } else {
            None
        }
    }
    fn payout(side: Side, units: u8, res: Side) -> i32 {
        let stake = units as i32 * UNIT;
        match (side, res) {
            (Side::Punto, Side::Punto) => stake,
            (Side::Banco, Side::Banco) => stake * 19 / 20,
            (Side::Egal, Side::Egal) => 8 * stake,
            (Side::Egal, _) => -stake,
            (_, Side::Egal) => 0,
            _ => -stake,
        }
    }
}

impl Game for Baccarat {
    type Move = Mv;
    fn to_move(&self) -> Player {
        match self.bettor() {
            Some(0) | None => Player::One,
            _ => Player::Two,
        }
    }
    fn legal_moves(&self) -> Vec<Mv> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        match self.bettor() {
            None => vec![Mv::Deal],
            Some(p) => {
                let maxu = (self.chips[p] / UNIT).clamp(0, 3) as u8;
                let mut v = Vec::new();
                for n in 1..=maxu {
                    v.push(Mv::Bet(Side::Punto, n));
                    v.push(Mv::Bet(Side::Banco, n));
                    v.push(Mv::Bet(Side::Egal, n));
                }
                v
            }
        }
    }
    fn apply(&mut self, m: &Mv) {
        match *m {
            Mv::Bet(s, n) => {
                let p = self.bettor().unwrap();
                self.bets[p] = Some((s, n));
            }
            Mv::Deal => {
                let mut deck: Vec<u8> = (0..52).collect();
                for i in (1..52).rev() {
                    let j = self.rng.below(i + 1);
                    deck.swap(i, j);
                }
                let (p, b) = play_coup(&mut deck);
                let sum = |h: &[u8]| h.iter().map(|&c| val(c)).sum::<u8>() % 10;
                let (pt, bt) = (sum(&p), sum(&b));
                let res = match pt.cmp(&bt) {
                    std::cmp::Ordering::Greater => Side::Punto,
                    std::cmp::Ordering::Less => Side::Banco,
                    _ => Side::Egal,
                };
                for q in 0..2 {
                    let (s, n) = self.bets[q].unwrap();
                    self.chips[q] += Self::payout(s, n, res);
                }
                self.log = format!(
                    "Coup {}: Punto {pt} ({} cărți), Banco {bt} ({} cărți) → {:?}.",
                    self.coup,
                    p.len(),
                    b.len(),
                    res
                );
                self.bets = [None; 2];
                self.coup += 1;
            }
        }
    }
    fn outcome(&self) -> Option<Outcome> {
        let broke = |p: usize| self.chips[p] < UNIT;
        if self.coup > COUPS || broke(0) || broke(1) {
            return Some(match self.chips[0].cmp(&self.chips[1]) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                _ => Outcome::Draw,
            });
        }
        None
    }
    fn evaluate(&self, pl: Player) -> i32 {
        let me = pl.idx();
        let mut v = 100 * (self.chips[me] - self.chips[1 - me]);
        // speranța matematică (în 1/10000 din miză) a pariurilor deja făcute
        let ev = |s: Side, n: u8| -> f32 {
            let stake = n as f32 * UNIT as f32;
            match s {
                Side::Punto => stake * (0.446247 - 0.458597),
                Side::Banco => stake * (0.458597 * 0.95 - 0.446247),
                Side::Egal => stake * (0.095156 * 8.0 - (1.0 - 0.095156)),
            }
        };
        for q in 0..2 {
            if let Some((s, n)) = self.bets[q] {
                let e = (ev(s, n) * 100.0) as i32;
                v += if q == me { e } else { -e };
            }
        }
        v
    }
    fn render(&self) -> String {
        let bet = |p: usize| self.bets[p].map_or("—".to_string(), |(s, n)| format!("{s:?} {n}"));
        format!(
            "Coup {}/{COUPS}. Jetoane: J1 {} – {} J2 (unitate = {UNIT}).\nPariuri: J1 {}, J2 {}.\n{}\nLa mutare: Jucătorul {}.",
            self.coup.min(COUPS),
            self.chips[0],
            self.chips[1],
            bet(0),
            bet(1),
            self.log,
            self.to_move().idx() + 1
        )
    }
    fn format_move(&self, m: &Mv) -> String {
        match m {
            Mv::Bet(Side::Punto, n) => format!("punto {n}"),
            Mv::Bet(Side::Banco, n) => format!("banco {n}"),
            Mv::Bet(Side::Egal, n) => format!("egal {n}"),
            Mv::Deal => "impart".into(),
        }
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
    fn natural_stops_drawing_and_tableau_works() {
        // ordinea extragerii: punto1, punto2, banco1, banco2, punto3, banco3 (deck.pop())
        let mut d: Vec<u8> = vec![2, 1, 4, 3]; // punto: 4+5 = 9 (natural), banco: 2+3 = 5
        let (p, b) = play_coup(&mut d);
        assert_eq!((p.len(), b.len()), (2, 2));
        // punto 1+4 = 5 trage; banco 0 (9+9) trage; a treia a lui punto = 6 (indice 5)
        let mut d2: Vec<u8> = vec![1, 5, 9, 9, 3, 0];
        let (p2, b2) = play_coup(&mut d2);
        assert_eq!(p2.len(), 3);
        assert_eq!(b2.len(), 3);
        // punto stă la 6–7: A+5 = 6; banco 5 trage la 0–5
        let mut d3: Vec<u8> = vec![7, 2, 1, 0, 4];
        let (p3, b3) = play_coup(&mut d3);
        assert_eq!(p3.len(), 2);
        assert_eq!(b3.len(), 3);
    }

    #[test]
    fn payouts() {
        assert_eq!(Baccarat::payout(Side::Banco, 1, Side::Banco), 19);
        assert_eq!(Baccarat::payout(Side::Punto, 2, Side::Punto), 40);
        assert_eq!(Baccarat::payout(Side::Egal, 1, Side::Egal), 160);
        assert_eq!(Baccarat::payout(Side::Punto, 1, Side::Egal), 0);
        assert_eq!(Baccarat::payout(Side::Banco, 3, Side::Punto), -60);
    }

    #[test]
    fn random_and_ai_games_finish() {
        random_finishes(|s| Box::new(Baccarat::with_seed(s)), 8, 400);
        ai_vs_random(|s| Box::new(Baccarat::with_seed(s)), 2, Player::One, 400);
        ai_vs_random(|s| Box::new(Baccarat::with_seed(s)), 3, Player::Two, 400);
        let g = Baccarat::with_seed(1);
        assert_eq!(GameObj::legal_moves(&g).len(), 9);
    }
}
