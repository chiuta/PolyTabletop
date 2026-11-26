//! Pass the Parcel (Treci pachetul) — joc de petrecere englezesc, adaptat pentru doi.
//!
//! * Pachetul are 8 straturi; fiecare ascunde un premiu (de la −3 la 5 puncte) care se vede doar când
//!   stratul e „la rând” (cel de sub el rămâne necunoscut).
//! * Cât cântă muzica, cel care ține pachetul alege: `pasez` (îl dă mai departe, gratuit) sau `pastrez`
//!   (îl ține, plătind 1 punct). Apoi urmează `muzica` (mutare separată): muzica se poate opri, cu șansa
//!   de 15% + 10% pentru fiecare acțiune din strat (cel mult 75%). Dacă s-a oprit, cel care ține pachetul
//!   desface stratul și primește premiul; începe stratul următor.
//! * După 8 straturi, câștigă cine are mai multe puncte.
//! * AI-ul compară valoarea așteptată a păstrării față de pasare din premiul stratului curent (public) și
//!   șansa de oprire (publică); straturile următoare nu le citește.
//!   Mutări: `pasez`, `pastrez`, `muzica`.

use crate::q5_util::{by_score, verdict};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const LAYERS: usize = 8;
const PRIZES: [i32; LAYERS] = [5, 4, 3, 3, 2, 1, -2, -3];

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Pass,
    Keep,
    Music,
}

#[derive(Clone)]
pub struct Parcel {
    prizes: Vec<i32>,
    layer: usize,
    actions: u8,
    holder: Player,
    check: bool,
    score: [i32; 2],
    rng: Rng,
    last: String,
}

impl Default for Parcel {
    fn default() -> Self {
        Self::new()
    }
}

impl Parcel {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut prizes = PRIZES.to_vec();
        for i in (1..prizes.len()).rev() {
            let j = rng.below(i + 1);
            prizes.swap(i, j);
        }
        Parcel {
            prizes,
            layer: 0,
            actions: 0,
            holder: Player::One,
            check: false,
            score: [0; 2],
            rng,
            last: String::new(),
        }
    }

    pub fn score(&self) -> [i32; 2] {
        self.score
    }

    /// Șansa de oprire în procente, după `actions` acțiuni în strat.
    fn stop_pct(&self) -> i32 {
        (15 + 10 * self.actions as i32).min(75)
    }

    /// Premiul stratului curent (public: „se vede doar stratul la rând”).
    fn current_prize(&self) -> i32 {
        self.prizes[self.layer]
    }
}

impl Game for Parcel {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.holder
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        if self.check {
            vec![Move::Music]
        } else {
            vec![Move::Pass, Move::Keep]
        }
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Pass => {
                self.holder = self.holder.other();
                self.actions += 1;
                self.check = true;
            }
            Move::Keep => {
                self.score[self.holder.idx()] -= 1;
                self.actions += 1;
                self.check = true;
            }
            Move::Music => {
                self.check = false;
                if (self.rng.below(100) as i32) < self.stop_pct() {
                    let pr = self.current_prize();
                    self.score[self.holder.idx()] += pr;
                    self.last = format!(
                        "Muzica s-a oprit: J{} desface stratul și ia {pr}.",
                        self.holder.idx() + 1
                    );
                    self.layer += 1;
                    self.actions = 0;
                } else {
                    self.last = "Muzica merge mai departe.".into();
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.layer >= LAYERS {
            return Some(by_score(self.score[0], self.score[1]));
        }
        None
    }

    /// Doar scorul, stratul curent și șansa publică de oprire; straturile viitoare nu sunt citite.
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut v = (self.score[me] - self.score[1 - me]) * 100;
        if self.check && self.layer < LAYERS {
            let e = self.current_prize() * self.stop_pct();
            v += if self.holder == p { e } else { -e };
        }
        v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Pass the Parcel — stratul {}/{LAYERS}. Puncte: J1 {}, J2 {}.\n",
            (self.layer + 1).min(LAYERS),
            self.score[0],
            self.score[1]
        );
        if self.layer < LAYERS {
            s.push_str(&format!(
                "Strat curent: premiu {} (straturile următoare sunt ascunse). Șansa ca muzica să se oprească: {}%.\n",
                self.current_prize(),
                self.stop_pct()
            ));
        }
        if !self.last.is_empty() {
            s.push_str(&format!("{}\n", self.last));
        }
        let v = verdict(self.outcome());
        if v.is_empty() {
            s.push_str(&format!(
                "Pachetul e la J{}{}\n",
                self.holder.idx() + 1,
                if self.check { " — muzica" } else { "" }
            ));
        } else {
            s.push_str(&v);
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Pass => "pasez".into(),
            Move::Keep => "pastrez".into(),
            Move::Music => "muzica".into(),
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
    fn keep_costs_a_point_and_pass_moves_parcel() {
        let mut g = Parcel::with_seed(1);
        g.apply(&Move::Keep);
        assert_eq!(g.score[0], -1);
        assert_eq!(g.holder, Player::One);
        g.apply(&Move::Music);
        let mut h = Parcel::with_seed(1);
        h.apply(&Move::Pass);
        assert_eq!(h.holder, Player::Two);
    }

    #[test]
    fn ai_keeps_a_big_prize_and_passes_a_penalty() {
        let mut g = Parcel::with_seed(1);
        g.prizes[0] = 5;
        g.actions = 5;
        assert_eq!(polytabletop_core::ai::best_move(&g, 1), Some(Move::Keep));
        g.prizes[0] = -3;
        assert_eq!(polytabletop_core::ai::best_move(&g, 1), Some(Move::Pass));
    }

    #[test]
    fn ai_ignores_future_layers() {
        let mut a = Parcel::with_seed(2);
        let mut b = a.clone();
        a.prizes[0] = 2;
        b.prizes[0] = 2;
        a.prizes[5] = 5;
        b.prizes[5] = -3;
        for s in 1..8 {
            assert_eq!(
                polytabletop_core::ai::best_move(&a, s),
                polytabletop_core::ai::best_move(&b, s)
            );
        }
    }

    #[test]
    fn random_and_ai_games() {
        check_game(Parcel::with_seed, 6, 1500);
    }
}
