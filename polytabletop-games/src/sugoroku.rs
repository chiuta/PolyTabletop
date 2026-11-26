//! Sugoroku (Japonia) — „e-sugoroku”: cursă pe o tablă cu casete speciale, 2 jucători.
//!
//! Reguli folosite (variantă proprie a jocului tradițional de zaruri pe drum):
//! * Drum de 40 de pași (0 = start, 40 = „agari”, finalul). Se aruncă **două zaruri**
//!   (mutare forțată `arunc`), iar jucătorul alege ce folosește: un zar, celălalt sau suma
//!   (`avansez N`). Aceasta este singura decizie, deci AI-ul ia o decizie reală.
//! * Se câștigă ajungând exact pe 40; dacă treci peste, te întorci pe același număr de pași
//!   (de la 40 înapoi).
//! * Casete speciale (se aplică o singură dată, fără lanț): 4 → +3, 9 → −3, 12 = pierzi
//!   următoarea tură, 15 = mai arunci o dată, 18 → +5, 21 = schimbi locurile cu adversarul,
//!   25 → −6, 28 = pierzi tura, 30 → +4, 33 → −8, 36 = mai arunci o dată, 38 → −10.
//! * Zarurile se rețin în stare cu Rng propriu; AI-ul nu vede aruncările viitoare.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const GOAL: i32 = 40;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Effect {
    Fwd(i32),
    Back(i32),
    Skip,
    Again,
    Swap,
}

pub fn effect(pos: i32) -> Option<Effect> {
    Some(match pos {
        4 => Effect::Fwd(3),
        9 => Effect::Back(3),
        12 | 28 => Effect::Skip,
        15 | 36 => Effect::Again,
        18 => Effect::Fwd(5),
        21 => Effect::Swap,
        25 => Effect::Back(6),
        30 => Effect::Fwd(4),
        33 => Effect::Back(8),
        38 => Effect::Back(10),
        _ => return None,
    })
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Throw,
    Advance(i32),
}

#[derive(Clone)]
pub struct Sugoroku {
    pos: [i32; 2],
    skip: [bool; 2],
    side: Player,
    dice: Option<(u8, u8)>,
    rng: Rng,
}

impl Default for Sugoroku {
    fn default() -> Self {
        Self::new()
    }
}

impl Sugoroku {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        Sugoroku {
            pos: [0, 0],
            skip: [false, false],
            side: Player::One,
            dice: None,
            rng: Rng::new(seed),
        }
    }

    /// Poziția după ce aplici `n` pași (cu întoarcere) și efectul casetei.
    /// Returnează (poziție proprie, poziție adversar, sare, din nou).
    fn land(&self, me: usize, n: i32) -> (i32, i32, bool, bool) {
        let mut p = self.pos[me] + n;
        if p > GOAL {
            p = 2 * GOAL - p;
        }
        let mut other = self.pos[1 - me];
        let (mut skip, mut again) = (false, false);
        if p != GOAL {
            match effect(p) {
                Some(Effect::Fwd(k)) => p = (p + k).min(GOAL),
                Some(Effect::Back(k)) => p = (p - k).max(0),
                Some(Effect::Skip) => skip = true,
                Some(Effect::Again) => again = true,
                Some(Effect::Swap) => std::mem::swap(&mut p, &mut other),
                None => {}
            }
        }
        (p, other, skip, again)
    }
}

impl Game for Sugoroku {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        match self.dice {
            None => vec![Move::Throw],
            Some((a, b)) => {
                let mut v: Vec<i32> = vec![a as i32, b as i32, (a + b) as i32];
                v.sort_unstable();
                v.dedup();
                v.into_iter().map(Move::Advance).collect()
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        match *m {
            Move::Throw => {
                self.dice = Some((self.rng.d6(), self.rng.d6()));
            }
            Move::Advance(n) => {
                let me = self.side.idx();
                let (p, o, skip, again) = self.land(me, n);
                self.pos[me] = p;
                self.pos[1 - me] = o;
                self.dice = None;
                if p == GOAL || o == GOAL {
                    return;
                }
                if skip {
                    self.skip[me] = true;
                }
                if !again {
                    self.side = self.side.other();
                    if self.skip[self.side.idx()] {
                        self.skip[self.side.idx()] = false;
                        self.side = self.side.other();
                        if self.skip[self.side.idx()] {
                            self.skip[self.side.idx()] = false;
                            self.side = self.side.other();
                        }
                    }
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.pos[0] == GOAL {
            Some(Outcome::Win(Player::One))
        } else if self.pos[1] == GOAL {
            Some(Outcome::Win(Player::Two))
        } else {
            None
        }
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut s = 10 * (self.pos[me] - self.pos[1 - me]);
        if self.skip[me] {
            s -= 25;
        }
        if self.skip[1 - me] {
            s += 25;
        }
        s
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Sugoroku — drum 0..{GOAL}\nJ1: {}{} | J2: {}{}\n",
            self.pos[0],
            if self.skip[0] { " (pierde tura)" } else { "" },
            self.pos[1],
            if self.skip[1] { " (pierde tura)" } else { "" }
        );
        if let Some((a, b)) = self.dice {
            s.push_str(&format!(
                "Zaruri: {a} și {b} — alege un zar sau suma ({}).\n",
                a + b
            ));
        }
        if self.outcome().is_none() {
            s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Throw => "arunc".into(),
            Move::Advance(n) => format!("avansez {n}"),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(a: i32, b: i32) -> Sugoroku {
        let mut g = Sugoroku::with_seed(1);
        g.pos = [a, b];
        g
    }

    #[test]
    fn exact_finish_and_bounce() {
        let g = at(37, 0);
        assert_eq!(g.land(0, 3).0, GOAL);
        // 39 + 3 = 42 -> 38 -> efect Back(10) -> 28? (efectul se aplică la aterizare)
        let g = at(39, 0);
        assert_eq!(g.land(0, 3).0, 28);
        let g = at(39, 0);
        assert_eq!(g.land(0, 2).0, 39);
    }

    #[test]
    fn effects_apply_once() {
        let g = at(1, 0);
        assert_eq!(g.land(0, 3).0, 7); // 4 -> +3
        let g = at(6, 0);
        assert_eq!(g.land(0, 3).0, 6); // 9 -> -3
        let g = at(10, 0);
        assert!(g.land(0, 2).2); // 12 = sare
        let g = at(13, 0);
        assert!(g.land(0, 2).3); // 15 = din nou
    }

    #[test]
    fn swap_exchanges_positions() {
        let g = at(19, 30);
        let (p, o, _, _) = g.land(0, 2);
        assert_eq!((p, o), (30, 21));
    }

    #[test]
    fn skip_turn_works() {
        let mut g = at(10, 0);
        g.dice = Some((2, 1));
        g.apply(&Move::Advance(2)); // 12: pierde tura următoare
        assert_eq!(g.side, Player::Two);
        g.dice = Some((1, 1));
        g.apply(&Move::Advance(1));
        assert_eq!(g.side, Player::Two); // J1 a pierdut tura
        assert!(!g.skip[0]);
    }

    #[test]
    fn choice_is_between_distinct_values() {
        let mut g = at(0, 0);
        g.dice = Some((3, 3));
        assert_eq!(g.legal_moves().len(), 2);
        g.dice = Some((2, 5));
        assert_eq!(g.legal_moves().len(), 3);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..60u64 {
            let mut g = Sugoroku::with_seed(seed);
            let mut rng = Rng::new(seed ^ 77);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
                assert!(n < 20000);
            }
        }
    }
}
