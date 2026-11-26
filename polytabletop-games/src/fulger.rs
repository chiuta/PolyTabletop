//! FULGER — potrivire de simboluri pe cărți (variantă originală de „găsește simbolul comun”).
//!
//! 31 de cărți cu câte 6 simboluri (plan proiectiv de ordin 5): oricare două cărți au
//! **exact un simbol comun**. Fiecare jucător are o carte în față, la mijloc stă o carte;
//! mutarea ta este să **numești simbolul comun** dintre cartea ta și cea din mijloc.
//! Cine îl numește primul (și corect) ia cartea din mijloc: aceasta devine cartea lui
//! și îi aduce un punct. Cronometrul e simulat pe ture: omul și masa (`Player::Two`)
//! primesc fiecare un timp de reacție tras la zar (masa devine puțin mai rapidă pe
//! parcurs). Un simbol greșit dă runda adversarului. Masa greșește uneori.
//! La final (pachetul gol) câștigă cine are mai multe puncte.
//! Informație: masa vede doar cartea ei și pe cea din mijloc, nu cartea ta.

use crate::w5_util::shuffle_vec;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const SYMBOLS: [&str; 31] = [
    "ancora", "bec", "cheie", "clopot", "corb", "dop", "fluture", "frunza", "gheata", "harta",
    "inel", "lacat", "lampa", "luna", "mar", "nor", "ou", "pahar", "pana", "pestera", "piatra",
    "pod", "roata", "scara", "stea", "tigaie", "umbrela", "val", "vas", "zar", "zmeu",
];

/// Cele 31 de cărți (fiecare 6 indici de simbol) pentru planul proiectiv de ordin 5.
pub fn all_cards() -> Vec<[u8; 6]> {
    let q = 5usize;
    // puncte: (x,y) -> x*5+y (0..25), pantă m -> 25+m, infinit -> 30
    let mut cards = Vec::new();
    for m in 0..q {
        for b in 0..q {
            let mut c = [0u8; 6];
            for (x, slot) in c.iter_mut().take(q).enumerate() {
                *slot = (x * q + (m * x + b) % q) as u8;
            }
            c[5] = (25 + m) as u8;
            cards.push(c);
        }
    }
    for cx in 0..q {
        let mut c = [0u8; 6];
        for (y, slot) in c.iter_mut().take(q).enumerate() {
            *slot = (cx * q + y) as u8;
        }
        c[5] = 30;
        cards.push(c);
    }
    cards.push([25, 26, 27, 28, 29, 30]);
    cards
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Name(u8),
    Clock,
}

#[derive(Clone)]
pub struct Fulger {
    deck: Vec<[u8; 6]>,
    centre: [u8; 6],
    own: [[u8; 6]; 2],
    pts: [u32; 2],
    guess: Option<u8>,
    round: u32,
    rng: Rng,
    over: bool,
    log: Vec<String>,
}

impl Default for Fulger {
    fn default() -> Self {
        Self::new()
    }
}

fn common(a: &[u8; 6], b: &[u8; 6]) -> u8 {
    *a.iter().find(|x| b.contains(x)).unwrap()
}

impl Fulger {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck = all_cards();
        shuffle_vec(&mut deck, &mut rng);
        let a = deck.pop().unwrap();
        let b = deck.pop().unwrap();
        let c = deck.pop().unwrap();
        Fulger {
            deck,
            centre: c,
            own: [a, b],
            pts: [0, 0],
            guess: None,
            round: 1,
            rng,
            over: false,
            log: Vec::new(),
        }
    }

    pub fn points(&self) -> (u32, u32) {
        (self.pts[0], self.pts[1])
    }
    pub fn deck_len(&self) -> usize {
        self.deck.len()
    }

    fn names(c: &[u8; 6]) -> String {
        let mut v: Vec<&str> = c.iter().map(|&i| SYMBOLS[i as usize]).collect();
        v.sort();
        v.join(", ")
    }

    fn award(&mut self, w: usize) {
        self.pts[w] += 1;
        self.own[w] = self.centre;
        match self.deck.pop() {
            Some(c) => self.centre = c,
            None => self.over = true,
        }
        self.round += 1;
        self.guess = None;
    }
}

impl Game for Fulger {
    type Move = Move;

    fn to_move(&self) -> Player {
        if self.guess.is_some() {
            Player::Two
        } else {
            Player::One
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over {
            return Vec::new();
        }
        if self.guess.is_some() {
            return vec![Move::Clock];
        }
        let mut v: Vec<Move> = self.own[0].iter().map(|&s| Move::Name(s)).collect();
        v.sort_by_key(|m| match m {
            Move::Name(s) => SYMBOLS[*s as usize],
            _ => "",
        });
        v
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Name(s) => self.guess = Some(*s),
            Move::Clock => {
                let g = self.guess.unwrap();
                let right = common(&self.own[0], &self.centre);
                let my_ok = g == right;
                // timpi de reacție în sutimi de secundă; masa se învioreză cu rundele
                let t_h = 90 + self.rng.below(160) as i32;
                let speed_up = (self.round as i32 * 2).min(40);
                let t_m = 80 + self.rng.below(190) as i32 - speed_up;
                let m_ok = self.rng.below(100) >= 9;
                let line = format!(
                    "Runda {}: ai spus «{}» ({}), {:.2}s; masa {}, {:.2}s.",
                    self.round,
                    SYMBOLS[g as usize],
                    if my_ok { "corect" } else { "greșit" },
                    t_h as f32 / 100.0,
                    if m_ok { "corect" } else { "greșit" },
                    t_m as f32 / 100.0
                );
                self.log.push(line);
                if self.log.len() > 8 {
                    self.log.remove(0);
                }
                let w = match (my_ok, m_ok) {
                    (true, true) => {
                        if t_h <= t_m {
                            0
                        } else {
                            1
                        }
                    }
                    (true, false) => 0,
                    (false, true) => 1,
                    (false, false) => {
                        // nimeni nu ia cartea: se trece peste ea fără punct
                        match self.deck.pop() {
                            Some(c) => self.centre = c,
                            None => self.over = true,
                        }
                        self.round += 1;
                        self.guess = None;
                        return;
                    }
                };
                self.award(w);
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            return None;
        }
        Some(match self.pts[0].cmp(&self.pts[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let (a, b) = (self.pts[0] as i32, self.pts[1] as i32);
        match p {
            Player::One => a - b,
            Player::Two => b - a,
        }
    }

    fn render(&self) -> String {
        let mut s = format!(
            "FULGER — runda {} — pachet: {} — puncte: tu {} / masa {}\n",
            self.round,
            self.deck.len(),
            self.pts[0],
            self.pts[1]
        );
        s.push_str(&format!("Cartea ta:    {}\n", Self::names(&self.own[0])));
        s.push_str(&format!(
            "Cartea din mijloc: {}\n",
            Self::names(&self.centre)
        ));
        s.push_str("Numește simbolul comun: «simbol» = numele lui (ex. luna).\n");
        for l in &self.log {
            s.push_str(l);
            s.push('\n');
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Name(s) => SYMBOLS[*s as usize].to_string(),
            Move::Clock => "cronometru".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plane_properties() {
        let c = all_cards();
        assert_eq!(c.len(), 31);
        for a in 0..31 {
            for b in 0..31 {
                let n = c[a].iter().filter(|x| c[b].contains(x)).count();
                assert_eq!(n, if a == b { 6 } else { 1 });
            }
            let mut d = c[a].to_vec();
            d.sort();
            d.dedup();
            assert_eq!(d.len(), 6);
        }
        for s in 0..31u8 {
            assert_eq!(c.iter().filter(|x| x.contains(&s)).count(), 6);
        }
        assert_eq!(SYMBOLS.len(), 31);
    }

    #[test]
    fn perfect_player_wins_most() {
        let mut won = 0;
        for seed in 1..100u64 {
            let mut g = Fulger::with_seed(seed);
            while g.outcome().is_none() {
                let m = if g.to_move() == Player::One {
                    Move::Name(common(&g.own[0], &g.centre))
                } else {
                    Move::Clock
                };
                g.apply(&m);
            }
            if g.outcome() == Some(Outcome::Win(Player::One)) {
                won += 1;
            }
        }
        assert!(won > 25, "{won}");
    }

    #[test]
    fn exactly_one_correct_symbol_and_game_ends() {
        let mut g = Fulger::with_seed(3);
        let mut n = 0;
        while g.outcome().is_none() {
            let right = common(&g.own[0], &g.centre);
            assert_eq!(g.own[0].iter().filter(|s| g.centre.contains(s)).count(), 1);
            let _ = right;
            let m = g.legal_moves()[0].clone();
            g.apply(&m);
            n += 1;
            assert!(n < 200);
        }
        let (a, b) = g.points();
        assert!(a + b <= 29);
    }
}
