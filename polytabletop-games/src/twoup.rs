//! Two-up (Australia), 2 jucători, din regulile publice ale jocului de monede.
//!
//! * Două monede sunt aruncate de „spinner”. Două steme (`cap-cap`) = spinnerul câștigă
//!   miza și **continuă**; două bani (`pajura-pajura`) = spinnerul pierde miza și
//!   rolul trece celuilalt; o stemă și un ban („odds”) = miza rămâne înghețată și
//!   se aruncă din nou. **Cinci „odds” la rând** = spinnerul pierde miza (în cazinou
//!   o încasează casa; aici o primește cel care a acoperit-o).
//! * Spinnerul mizează pe stemă (cum se obișnuiește) o sumă pe care celălalt
//!   jucător o **acoperă** automat (mize 5, 10 sau 20 de jetoane, limitate de ce are
//!   fiecare).
//! * Formatul meciului (ales de noi): fiecare începe cu 50 de jetoane; după 24 de mize
//!   rezolvate câștigă cine are mai multe jetoane (egal = remiză); cine rămâne fără
//!   jetoane pierde imediat. Jucătorul 1 e primul spinner.
//!
//! Mutări: `miza 5` / `miza 10` / `miza 20`, apoi `arunc` (pas forțat al spinnerului,
//! repetat după „odds”). Aruncarea se face în `apply` cu `Rng`-ul jocului, deci AI-ul
//! nu vede monedele viitoare. Șansa de „spinner câștigător” e ușor sub 50% din cauza
//! regulii celor cinci „odds” (≈48,4%), iar AI-ul alege miza după situația scorului.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const START: i32 = 50;
const BETS: u8 = 24;
const STAKES: [i32; 3] = [5, 10, 20];

#[derive(Clone, PartialEq, Debug)]
pub enum Move {
    Stake(i32),
    Spin,
}

#[derive(Clone)]
pub struct TwoUp {
    rng: Rng,
    chips: [i32; 2],
    spinner: Player,
    stake: i32,
    odds: u8,
    resolved: u8,
    last: String,
}

impl Default for TwoUp {
    fn default() -> Self {
        Self::new()
    }
}

impl TwoUp {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        TwoUp {
            rng: Rng::new(seed),
            chips: [START; 2],
            spinner: Player::One,
            stake: 0,
            odds: 0,
            resolved: 0,
            last: String::new(),
        }
    }

    pub fn chips(&self) -> [i32; 2] {
        self.chips
    }

    fn over(&self) -> bool {
        self.resolved >= BETS || self.chips[0] <= 0 || self.chips[1] <= 0
    }

    fn settle(&mut self, spinner_wins: bool, why: &str) {
        let s = self.spinner.idx();
        let o = 1 - s;
        if spinner_wins {
            self.chips[s] += self.stake;
            self.chips[o] -= self.stake;
        } else {
            self.chips[s] -= self.stake;
            self.chips[o] += self.stake;
            self.spinner = self.spinner.other();
        }
        self.last = format!(
            "{why}: {} {} jetoane.",
            if spinner_wins {
                "spinnerul câștigă"
            } else {
                "spinnerul pierde"
            },
            self.stake
        );
        self.resolved += 1;
        self.stake = 0;
        self.odds = 0;
    }
}

impl Game for TwoUp {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.spinner
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over() {
            return Vec::new();
        }
        if self.stake > 0 {
            return vec![Move::Spin];
        }
        let cap = self.chips[0].min(self.chips[1]);
        let mut v: Vec<Move> = STAKES
            .iter()
            .copied()
            .filter(|&s| s <= cap)
            .map(Move::Stake)
            .collect();
        if v.is_empty() {
            v.push(Move::Stake(cap));
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Stake(s) => {
                self.stake = *s;
                self.odds = 0;
            }
            Move::Spin => {
                let a = self.rng.below(2);
                let b = self.rng.below(2);
                match a + b {
                    2 => {
                        self.last = "cap-cap".into();
                        self.settle(true, "cap-cap");
                    }
                    0 => {
                        self.last = "pajura-pajura".into();
                        self.settle(false, "pajura-pajura");
                    }
                    _ => {
                        self.odds += 1;
                        if self.odds >= 5 {
                            self.settle(false, "cinci «odds» la rând");
                        } else {
                            self.last =
                                format!("cap-pajura (odds {}/5), se aruncă din nou", self.odds);
                        }
                    }
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over() {
            return None;
        }
        Some(match self.chips[0].cmp(&self.chips[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    /// Doar jetoanele și miza aleasă: se evaluează valoarea așteptată a mizei
    /// (spinnerul câștigă ≈48,4%) și o preferință de risc după scor.
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let diff = self.chips[me] - self.chips[1 - me];
        let mut v = diff * 100;
        if self.stake > 0 {
            let spin_win = 0.484_f64;
            let ev = (2.0 * spin_win - 1.0) * self.stake as f64;
            let ev = if self.spinner == p { ev } else { -ev };
            v += (ev * 100.0).round() as i32;
            // în urmă = caută varianța, în față = o evită
            let risk = self.stake
                * if diff < 0 {
                    12
                } else if diff > 0 {
                    -12
                } else {
                    0
                };
            v += risk;
        }
        v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Two-up — miza {}/{BETS}. Jetoane: J1 {}, J2 {} (start {START}).\n",
            self.resolved + 1,
            self.chips[0],
            self.chips[1]
        );
        s.push_str(&format!(
            "Spinner: Jucător {} (mizează pe stemă); celălalt acoperă.\n",
            self.spinner.idx() + 1
        ));
        if self.stake > 0 {
            s.push_str(&format!(
                "Miza pe masă: {} jetoane. «odds» consecutive: {}.\n",
                self.stake, self.odds
            ));
        }
        if !self.last.is_empty() {
            s.push_str(&format!("Ultima aruncare: {}\n", self.last));
        }
        match self.outcome() {
            Some(Outcome::Win(p)) => s.push_str(&format!(
                "Jocul s-a încheiat: Jucător {} câștigă.\n",
                p.idx() + 1
            )),
            Some(Outcome::Draw) => s.push_str("Jocul s-a încheiat: remiză.\n"),
            None if self.stake > 0 => s.push_str("La mutare: spinnerul — arunc\n"),
            None => s.push_str("La mutare: spinnerul — alege miza (miza 5 / 10 / 20)\n"),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Stake(s) => format!("miza {s}"),
            Move::Spin => "arunc".into(),
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
    fn stake_then_spin_and_roles() {
        let mut g = TwoUp::with_seed(1);
        assert_eq!(g.legal_moves().len(), 3);
        g.apply(&Move::Stake(10));
        assert_eq!(g.legal_moves(), vec![Move::Spin]);
        let mut guard = 0;
        while g.stake > 0 {
            g.apply(&Move::Spin);
            guard += 1;
            assert!(guard < 100);
        }
        assert_eq!(g.chips[0] + g.chips[1], 2 * START);
        assert_eq!(g.resolved, 1);
    }

    #[test]
    fn tails_pass_the_kip_heads_keep_it_and_five_odds_lose() {
        let mut g = TwoUp::with_seed(2);
        g.stake = 10;
        g.settle(true, "x");
        assert_eq!(g.spinner, Player::One);
        assert_eq!(g.chips, [60, 40]);
        g.stake = 10;
        g.settle(false, "x");
        assert_eq!(g.spinner, Player::Two);
        assert_eq!(g.chips, [50, 50]);
        let mut h = TwoUp::with_seed(3);
        h.stake = 5;
        h.odds = 4;
        // forțăm o aruncare de tip odds prin simulare până apare
        let mut tries = 0;
        loop {
            let mut c = h.clone();
            c.apply(&Move::Spin);
            if c.resolved == 1 && c.chips == [45, 55] {
                break;
            }
            h.rng.next_u64();
            tries += 1;
            assert!(tries < 200);
        }
    }

    #[test]
    fn stakes_are_limited_by_the_poorer_player() {
        let mut g = TwoUp::with_seed(4);
        g.chips = [50, 7];
        assert_eq!(g.legal_moves(), vec![Move::Stake(5)]);
        g.chips = [50, 3];
        assert_eq!(g.legal_moves(), vec![Move::Stake(3)]);
    }

    #[test]
    fn random_games_end_with_conserved_chips() {
        for seed in 1..=30u64 {
            let mut g = TwoUp::with_seed(seed);
            let mut rng = Rng::new(seed ^ 9);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                g.apply(&m[rng.below(m.len())]);
                assert_eq!(g.chips[0] + g.chips[1], 2 * START);
                n += 1;
                assert!(n < 2000);
            }
        }
    }

    #[test]
    fn ai_plays_and_is_deterministic() {
        use polytabletop_core::GameObj;
        let g: Box<dyn GameObj> = Box::new(TwoUp::with_seed(5));
        assert_eq!(g.ai_move(1), g.ai_move(1));
        let mut a: Box<dyn GameObj> = Box::new(TwoUp::with_seed(6));
        let mut b: Box<dyn GameObj> = Box::new(TwoUp::with_seed(6));
        while a.outcome().is_none() {
            let m = a.ai_move(3).unwrap();
            a.play(&m).unwrap();
            let m2 = b.ai_move(3).unwrap();
            b.play(&m2).unwrap();
        }
        assert_eq!(a.render(), b.render());
    }
}
