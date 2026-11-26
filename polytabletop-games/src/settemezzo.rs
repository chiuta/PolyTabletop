//! Sette e mezzo (Italia / Spania) — joc de cărți cu pachet italian de 40 de cărți, în doi.
//!
//! Valori: A–7 = valoarea lor, 8 (fante), 9 (cavallo) și 10 (re) = ½ punct. Scopul: să te apropii cât mai mult de
//! **7½** fără să treci peste (peste 7½ = „sballato”, pierzi).
//! * Rolurile: banca și ko alternează la fiecare rundă; 20 de jetoane fiecare, 12 runde.
//! * Ko pariază 1–3 (`pariez 2`); fiecare primește o carte, **prima** cu fața în jos (adversarul n-o vede).
//! * Ko joacă primul: `carta` (încă o carte, cu fața în sus) sau `stau`. Dacă trece de 7½, pierde miza imediat.
//!   Apoi joacă banca la fel. Banca câștigă la egalitate. Cine câștigă cu 7½ „natural” (două cărți: 7 + figură)
//!   încasează dublu când e ko.
//! * Câștigă la final cine are mai multe jetoane (egal = remiză), sau cine rămâne cu jetoane când celălalt are 0.
//!
//! Informație ascunsă: prima carte a fiecăruia. AI-ul nu o citește pe a adversarului: decide din propriile cărți și din
//! cele vizibile, cu speranța matematică calculată pe cărțile nevăzute (inclusiv cartea ascunsă a adversarului).

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const START: i32 = 20;
const ROUNDS: u8 = 12;
const SUITS: [&str; 4] = ["monede", "cupe", "spade", "bastoane"];

/// valoarea în jumătăți de punct
fn half(card: u8) -> i32 {
    let r = card % 10;
    if r < 7 {
        2 * (r as i32 + 1)
    } else {
        1
    }
}
fn card_text(c: u8) -> String {
    let r = c % 10;
    let rank = match r {
        0..=6 => (r + 1).to_string(),
        7 => "fante".to_string(),
        8 => "cavallo".to_string(),
        _ => "re".to_string(),
    };
    format!("{}-{}", rank, &SUITS[(c / 10) as usize][..2])
}
fn total(h: &[u8]) -> i32 {
    h.iter().map(|&c| half(c)).sum()
}
fn fmt_total(t: i32) -> String {
    if t % 2 == 0 {
        format!("{}", t / 2)
    } else {
        format!("{}½", t / 2)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mv {
    Bet(u8),
    Draw,
    Stand,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Bet,
    Ko,
    Bank,
}

#[derive(Clone)]
pub struct SetteMezzo {
    viewer: Option<Player>,
    chips: [i32; 2],
    bank: usize,
    bet: i32,
    phase: Phase,
    hands: [Vec<u8>; 2],
    deck: Vec<u8>,
    round: u8,
    /// (cine, a tras?, totalul dinainte)
    last: Option<(usize, bool, i32)>,
    rng: Rng,
    log: String,
}

impl Default for SetteMezzo {
    fn default() -> Self {
        Self::new()
    }
}

impl SetteMezzo {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        SetteMezzo {
            viewer: None,
            chips: [START; 2],
            bank: 0,
            bet: 0,
            phase: Phase::Bet,
            hands: [Vec::new(), Vec::new()],
            deck: Vec::new(),
            round: 1,
            last: None,
            rng: Rng::new(seed),
            log: String::new(),
        }
    }
    pub fn chips(&self) -> (i32, i32) {
        (self.chips[0], self.chips[1])
    }
    fn ko(&self) -> usize {
        1 - self.bank
    }
    fn actor(&self) -> usize {
        match self.phase {
            Phase::Bet | Phase::Ko => self.ko(),
            Phase::Bank => self.bank,
        }
    }
    fn shuffle(&mut self) {
        let mut d: Vec<u8> = (0..40).collect();
        for i in (1..40).rev() {
            let j = self.rng.below(i + 1);
            d.swap(i, j);
        }
        self.deck = d;
    }
    fn settle(&mut self, winner: usize, mult: i32, why: &str) {
        let amt = (self.bet * mult).min(self.chips[1 - winner]);
        self.chips[winner] += amt;
        self.chips[1 - winner] -= amt;
        self.log = format!(
            "{why} J{} câștigă {amt}. (J1: {}, J2: {})",
            winner + 1,
            fmt_total(total(&self.hands[0])),
            fmt_total(total(&self.hands[1]))
        );
        self.round += 1;
        self.bank = 1 - self.bank;
        self.phase = Phase::Bet;
        self.last = None;
    }
    /// Valoarea de rămânere (≈ probabilitate de câștig) pentru totalul `t` (în jumătăți).
    fn stand_value(t: i32) -> f32 {
        if t > 15 {
            0.0
        } else {
            (t as f32 / 15.0).powi(2)
        }
    }
    /// Cărțile nevăzute de `who`: nu e în mâna lui și nu e vizibilă (cărți deschise ale adversarului).
    fn unseen(&self, who: usize) -> Vec<u8> {
        let mut seen = [false; 40];
        for &c in &self.hands[who] {
            seen[c as usize] = true;
        }
        for &c in self.hands[1 - who].iter().skip(1) {
            seen[c as usize] = true;
        }
        (0..40u8).filter(|&c| !seen[c as usize]).collect()
    }
}

impl Game for SetteMezzo {
    type Move = Mv;
    fn to_move(&self) -> Player {
        if self.actor() == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn legal_moves(&self) -> Vec<Mv> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        match self.phase {
            Phase::Bet => {
                let max = 3.min(self.chips[0]).min(self.chips[1]);
                (1..=max as u8).map(Mv::Bet).collect()
            }
            _ => vec![Mv::Draw, Mv::Stand],
        }
    }
    fn apply(&mut self, m: &Mv) {
        let who = self.actor();
        match *m {
            Mv::Bet(n) => {
                self.bet = n as i32;
                self.shuffle();
                for p in 0..2 {
                    self.hands[p] = vec![self.deck.pop().unwrap()];
                }
                self.phase = Phase::Ko;
                self.last = None;
                self.log = format!(
                    "Runda {}: J{} (ko) pariază {n}; fiecare primește o carte ascunsă.",
                    self.round,
                    self.ko() + 1
                );
            }
            Mv::Draw => {
                let before = total(&self.hands[who]);
                let c = self.deck.pop().unwrap();
                self.hands[who].push(c);
                self.last = Some((who, true, before));
                if total(&self.hands[who]) > 15 {
                    let w = 1 - who;
                    self.settle(w, 1, &format!("J{} depășește 7½ („sballato”)!", who + 1));
                }
            }
            Mv::Stand => {
                let before = total(&self.hands[who]);
                self.last = Some((who, false, before));
                if self.phase == Phase::Ko {
                    self.phase = Phase::Bank;
                } else {
                    let (k, b) = (total(&self.hands[self.ko()]), total(&self.hands[self.bank]));
                    if k > b {
                        let natural = self.hands[self.ko()].len() == 2 && k == 15;
                        let ko = self.ko();
                        self.settle(ko, if natural { 2 } else { 1 }, "Ko bate banca.");
                    } else {
                        let bank = self.bank;
                        self.settle(bank, 1, "Banca câștigă (la egalitate câștigă banca).");
                    }
                }
            }
        }
    }
    fn outcome(&self) -> Option<Outcome> {
        for p in 0..2 {
            if self.chips[p] <= 0 {
                return Some(Outcome::Win(if p == 0 { Player::Two } else { Player::One }));
            }
        }
        if self.round > ROUNDS {
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
        let mut v = 1000 * (self.chips[me] - self.chips[1 - me]);
        if self.phase == Phase::Bet {
            // pariu: banca preferă mize mari, ko mici
            if self.ko() == me {
                v -= 10 * self.bet;
            }
            return v;
        }
        if let Some((who, drew, before)) = self.last {
            // valoarea deciziei, calculată din ce știa decidentul (fără cartea trasă efectiv)
            let val = if drew {
                let un = self.unseen(who);
                let mut s = 0.0;
                for &c in &un {
                    s += Self::stand_value(before + half(c));
                }
                s / un.len().max(1) as f32
            } else {
                Self::stand_value(before)
            };
            let x = (val * 500.0) as i32;
            v += if who == me { x } else { -x };
        }
        v
    }
    fn render(&self) -> String {
        let view = self.viewer.unwrap_or(self.to_move()).idx();
        let mut s = format!(
            "Runda {}/{ROUNDS}. Jetoane: J1 {}, J2 {}. Banca: J{}. Pariu: {}.\n",
            self.round.min(ROUNDS),
            self.chips[0],
            self.chips[1],
            self.bank + 1,
            self.bet
        );
        if self.phase != Phase::Bet {
            let own: Vec<String> = self.hands[view].iter().map(|&c| card_text(c)).collect();
            s.push_str(&format!(
                "Mâna ta (J{}): {} = {}\n",
                view + 1,
                own.join(", "),
                fmt_total(total(&self.hands[view]))
            ));
            let opp = 1 - view;
            let shown: Vec<String> = self.hands[opp]
                .iter()
                .enumerate()
                .map(|(i, &c)| {
                    if i == 0 {
                        "?".to_string()
                    } else {
                        card_text(c)
                    }
                })
                .collect();
            s.push_str(&format!(
                "Adversarul (J{}): {} (vizibil: {})\n",
                opp + 1,
                shown.join(", "),
                fmt_total(total(&self.hands[opp][1.min(self.hands[opp].len())..]))
            ));
        }
        s.push_str(self.log.trim());
        s.push_str(&format!("\nLa mutare: Jucătorul {}.", self.actor() + 1));
        s
    }
    fn format_move(&self, m: &Mv) -> String {
        match m {
            Mv::Bet(n) => format!("pariez {n}"),
            Mv::Draw => "carta".into(),
            Mv::Stand => "stau".into(),
        }
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q4_util::*;
    use polytabletop_core::{GameObj, Player};

    #[test]
    fn card_values() {
        assert_eq!(half(0), 2); // as = 1
        assert_eq!(half(6), 14); // 7
        assert_eq!(half(7), 1);
        assert_eq!(half(9), 1);
        assert_eq!(total(&[6, 9]), 15);
    }

    #[test]
    fn opponent_first_card_is_hidden() {
        let mut g = SetteMezzo::with_seed(2);
        GameObj::play(&mut g, "pariez 1").unwrap();
        let mut a = g.clone_box();
        a.set_viewer(Some(Player::One));
        let r = a.render();
        assert!(r.contains("Mâna ta (J1)"), "{r}");
        assert!(r.contains("Adversarul (J2): ?"), "{r}");
        let mut b = g.clone_box();
        b.set_viewer(Some(Player::Two));
        assert_ne!(a.render(), b.render());
    }

    #[test]
    fn random_and_ai_games_finish() {
        random_finishes(|s| Box::new(SetteMezzo::with_seed(s)), 8, 800);
        ai_vs_random(|s| Box::new(SetteMezzo::with_seed(s)), 2, Player::One, 800);
        ai_vs_random(|s| Box::new(SetteMezzo::with_seed(s)), 3, Player::Two, 800);
    }
}
