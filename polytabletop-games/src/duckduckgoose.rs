#![allow(clippy::doc_lazy_continuation)]
//! Duck Duck Goose (SUA), joc de copii în cerc, adaptat ca duel de risc pentru doi jucători.
//!
//! * În fiecare rundă un jucător e „Cel care atinge” (J1 la rundele impare, J2 la cele pare) și
//!   celălalt este „Gâsca”. Cel care atinge alege câte capete de „rață” spune înainte să strige
//!   „gâscă” (1–5): cu cât mai multe, cu atât gâsca e mai departe de locul liber (șanse mai mici de
//!   a fi prins), dar scăpând cu bine primești pe atât puncte.
//! * Șansa ca cel care atinge să ajungă pe locul gâștei fără să fie prins: 90% − 14% × rațe
//!   (rațe = 1…5); dacă e prins, gâsca primește 1 punct.
//! * 10 runde; cine are mai multe puncte câștigă (egal = remiză). Cursa se hotărăște la pasul
//!   forțat `fuga`, deci AI-ul nu vede rezultatul; alege rațele cu cea mai bună valoare așteptată.
//! Mutări: `rate 1`…`rate 5`, `fuga`.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const ROUNDS: u8 = 10;

fn p_safe(ducks: u8) -> f64 {
    0.9 - 0.14 * ducks as f64
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Ducks(u8),
    Run,
}

#[derive(Clone)]
pub struct DuckDuckGoose {
    rng: Rng,
    pts: [i32; 2],
    round: u8,
    ducks: Option<u8>,
    last: String,
}

impl Default for DuckDuckGoose {
    fn default() -> Self {
        Self::new()
    }
}

impl DuckDuckGoose {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        DuckDuckGoose {
            rng: Rng::new(seed),
            pts: [0; 2],
            round: 0,
            ducks: None,
            last: String::new(),
        }
    }

    fn toucher(&self) -> Player {
        if self.round.is_multiple_of(2) {
            Player::One
        } else {
            Player::Two
        }
    }
}

impl Game for DuckDuckGoose {
    type Move = Move;

    fn to_move(&self) -> Player {
        if self.ducks.is_some() {
            Player::Two
        } else {
            self.toucher()
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.round >= ROUNDS {
            return Vec::new();
        }
        if self.ducks.is_some() {
            return vec![Move::Run];
        }
        (1..=5).map(Move::Ducks).collect()
    }

    fn apply(&mut self, m: &Move) {
        match *m {
            Move::Ducks(k) => self.ducks = Some(k),
            Move::Run => {
                let k = self.ducks.take().unwrap();
                let t = self.toucher().idx();
                let g = 1 - t;
                if (self.rng.below(1000) as f64) < p_safe(k) * 1000.0 {
                    self.pts[t] += k as i32;
                    self.last = format!("J{} a spus {k} rațe și a ajuns pe loc: +{k}", t + 1);
                } else {
                    self.pts[g] += 1;
                    self.last = format!(
                        "J{} a fost prins de gâscă (J{}): +1 pentru gâscă",
                        t + 1,
                        g + 1
                    );
                }
                self.round += 1;
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.round < ROUNDS {
            return None;
        }
        Some(match self.pts[0].cmp(&self.pts[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let mut v = (self.pts[p.idx()] - self.pts[p.other().idx()]) * 100;
        if let Some(k) = self.ducks {
            let t = self.toucher();
            let ev = p_safe(k) * k as f64 - (1.0 - p_safe(k));
            v += (if t == p { ev } else { -ev } * 100.0) as i32;
        }
        v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Duck Duck Goose — runda {}/{ROUNDS}. Scor: J1 {} – J2 {}.\n",
            (self.round + 1).min(ROUNDS),
            self.pts[0],
            self.pts[1]
        );
        if self.round < ROUNDS {
            s.push_str(&format!(
                "Cel care atinge: J{}. {}\n",
                self.toucher().idx() + 1,
                match self.ducks {
                    Some(k) => format!("A spus {k} rațe — scrie «fuga»."),
                    None => "Alege `rate 1`…`rate 5` (șansă de scăpare 76%…20%).".into(),
                }
            ));
        }
        if !self.last.is_empty() {
            s.push_str(&format!("{}\n", self.last));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Ducks(k) => format!("rate {k}"),
            Move::Run => "fuga".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rounds_alternate_and_games_end() {
        let mut g = DuckDuckGoose::with_seed(1);
        assert_eq!(g.toucher(), Player::One);
        g.apply(&Move::Ducks(2));
        g.apply(&Move::Run);
        assert_eq!(g.toucher(), Player::Two);
        for seed in 1..=20u64 {
            let mut g = DuckDuckGoose::with_seed(seed);
            let mut rng = Rng::new(seed);
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
            }
        }
    }

    #[test]
    fn ai_picks_the_best_expected_value_blindly() {
        let (a, b) = (DuckDuckGoose::with_seed(1), DuckDuckGoose::with_seed(77));
        let m = polytabletop_core::GameObj::ai_move(&a, 1);
        assert_eq!(m, polytabletop_core::GameObj::ai_move(&b, 9));
        assert_eq!(m.as_deref(), Some("rate 3"));
    }
}
