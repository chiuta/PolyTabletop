//! Crima cu ochiade (Wink Murder, Marea Britanie) — joc de deducție pentru doi: ucigașul și detectivul.
//!
//! Zece suspecți stau în cerc (1–10). J1 e ucigașul și își alege în secret un loc (`sunt 4`). În fiecare noapte
//! ucigașul face cu ochiul unui suspect viu care moare (`clipesc 7`); detectivul (J2) primește un *martor*:
//! o mulțime de 4 suspecți vii dintre care unul e ucigașul. Apoi detectivul acuză (`acuz 3`) sau așteaptă (`astept`).
//! Acuzație corectă = detectivul câștigă. Acuzat nevinovat iese din cerc și detectivul primește o greșeală; la 3 greșeli
//! sau când rămân doar 2 suspecți vii câștigă ucigașul. Martorii se intersectează pentru a restrânge bănuiții.
//!
//! Informație ascunsă: locul ucigașului. Martorii și verdictul unei acuzații apar când urmează mutarea următoare;
//! AI-ul detectiv nu citește locul: folosește doar martorii; AI-ul ucigaș alege victimele astfel încât să păstreze
//! mulți bănuiți în viață.

use crate::q4_secret::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const SEATS: u32 = 10;
const FULL: u16 = (1 << SEATS) - 1;
const MAX_STRIKES: u8 = 3;
const WITNESS: usize = 4;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mv {
    Pick(u8),
    Wink(u8),
    Accuse(u8),
    Wait,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Pick,
    Wink,
    Detect,
    Over,
}

#[derive(Clone)]
pub struct Crima {
    viewer: Option<Player>,
    murderer: u8,
    alive: u16,
    witnesses: Vec<u16>,
    witness_seed: Option<u64>,
    accuse: Option<u8>,
    strikes: u8,
    phase: Phase,
    winner: Option<Player>,
    rng: Rng,
    noise: u64,
    log: String,
}

impl Default for Crima {
    fn default() -> Self {
        Self::new()
    }
}

fn count(m: u16) -> u32 {
    m.count_ones()
}

impl Crima {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let noise = rng.next_u64();
        Crima {
            viewer: None,
            murderer: 0,
            alive: FULL,
            witnesses: Vec::new(),
            witness_seed: None,
            accuse: None,
            strikes: 0,
            phase: Phase::Pick,
            winner: None,
            rng,
            noise,
            log: String::new(),
        }
    }
    pub fn alive_count(&self) -> u32 {
        count(self.alive)
    }

    fn resolved(&self) -> Crima {
        let mut r = self.clone();
        if let Some(seed) = self.witness_seed {
            r.witness_seed = None;
            let mut rng = Rng::new(seed);
            let mut others: Vec<u8> = (0..SEATS as u8)
                .filter(|&k| r.alive >> k & 1 == 1 && k != r.murderer)
                .collect();
            for i in (1..others.len()).rev() {
                others.swap(i, rng.below(i + 1));
            }
            let mut mask = 1u16 << r.murderer;
            for &k in others.iter().take(WITNESS - 1) {
                mask |= 1 << k;
            }
            r.witnesses.push(mask);
            r.log
                .push_str(&format!(" Martor: unul dintre {}.", mask_text(mask)));
            r.phase = Phase::Detect;
        }
        if let Some(k) = self.accuse {
            r.accuse = None;
            if k == self.murderer {
                r.winner = Some(Player::Two);
                r.phase = Phase::Over;
                r.log = format!(
                    "Acuzația împotriva lui {} era corectă: detectivul câștigă!",
                    k + 1
                );
            } else {
                r.strikes += 1;
                r.alive &= !(1 << k);
                r.log = format!(
                    "{} e nevinovat și iese din cerc (greșeli: {}/{MAX_STRIKES}).",
                    k + 1,
                    r.strikes
                );
                if r.strikes >= MAX_STRIKES || count(r.alive) <= 2 {
                    r.winner = Some(Player::One);
                    r.phase = Phase::Over;
                    r.log.push_str(" Ucigașul scapă!");
                }
            }
        }
        r
    }

    /// Bănuiții publici: suspecți vii în toate mulțimile-martor.
    fn suspects(&self) -> u16 {
        let mut s = self.alive;
        for &w in &self.witnesses {
            s &= w;
        }
        s
    }
}

fn mask_text(m: u16) -> String {
    (0..SEATS as u8)
        .filter(|&k| m >> k & 1 == 1)
        .map(|k| (k + 1).to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

impl Game for Crima {
    type Move = Mv;
    fn to_move(&self) -> Player {
        let r = self.resolved();
        match r.phase {
            Phase::Pick | Phase::Wink => Player::One,
            _ => Player::Two,
        }
    }
    fn legal_moves(&self) -> Vec<Mv> {
        let r = self.resolved();
        match r.phase {
            Phase::Over => Vec::new(),
            Phase::Pick => (0..SEATS as u8).map(Mv::Pick).collect(),
            Phase::Wink => (0..SEATS as u8)
                .filter(|&k| r.alive >> k & 1 == 1 && k != r.murderer)
                .map(Mv::Wink)
                .collect(),
            Phase::Detect => {
                let mut v: Vec<Mv> = (0..SEATS as u8)
                    .filter(|&k| r.alive >> k & 1 == 1)
                    .map(Mv::Accuse)
                    .collect();
                v.push(Mv::Wait);
                v
            }
        }
    }
    fn apply(&mut self, m: &Mv) {
        *self = self.resolved();
        match *m {
            Mv::Pick(k) => {
                self.murderer = k;
                self.phase = Phase::Wink;
                self.log = "Ucigașul și-a ales locul. Începe noaptea.".into();
            }
            Mv::Wink(k) => {
                self.alive &= !(1 << k);
                self.witness_seed = Some(self.rng.next_u64());
                self.log = format!("Ucigașul face cu ochiul: {} moare.", k + 1);
                if count(self.alive) <= 2 {
                    self.winner = Some(Player::One);
                    self.witness_seed = None;
                    self.phase = Phase::Over;
                    self.log
                        .push_str(" Au rămas doar doi suspecți: ucigașul câștigă!");
                }
            }
            Mv::Accuse(k) => self.accuse = Some(k),
            Mv::Wait => {
                self.phase = Phase::Wink;
                self.log = "Detectivul așteaptă.".into();
            }
        }
    }
    fn outcome(&self) -> Option<Outcome> {
        self.resolved().winner.map(Outcome::Win)
    }
    fn evaluate(&self, p: Player) -> i32 {
        // din punctul de vedere al detectivului (J2); informație publică
        let sus = self.suspects();
        let n = count(sus).max(1) as f32;
        let mut det = 0.0;
        if let Some(k) = self.accuse {
            let p_ok = if sus >> k & 1 == 1 { 1.0 / n } else { 0.0 };
            det = p_ok - (1.0 - p_ok) * (self.strikes as f32 + 1.0) / 6.0;
        } else if self.phase == Phase::Wink && !self.witnesses.is_empty() {
            // detectivul a așteptat
            det = 0.1;
        } else if self.phase == Phase::Detect || self.witness_seed.is_some() {
            det = -(n / 10.0);
        }
        let mut v = (det * 1000.0) as i32;
        if self.phase == Phase::Wink && self.witnesses.is_empty() {
            v = noise(self.noise, self.murderer as u64, 40);
            return if p == Player::Two { v } else { -v };
        }
        if p == Player::Two {
            v
        } else {
            -v
        }
    }
    fn render(&self) -> String {
        let r = self.resolved();
        let view = viewer_of(self.viewer, self.to_move());
        let circle: Vec<String> = (0..SEATS as u8)
            .map(|k| {
                if r.alive >> k & 1 == 1 {
                    format!("{}", k + 1)
                } else {
                    "·".to_string()
                }
            })
            .collect();
        let mut s = format!(
            "Cercul (vii): {}. Greșeli detectiv: {}/{MAX_STRIKES}.\n",
            circle.join(" "),
            r.strikes
        );
        if !r.witnesses.is_empty() {
            for (i, &w) in r.witnesses.iter().enumerate() {
                s.push_str(&format!(
                    "Martor {}: ucigașul e printre {}\n",
                    i + 1,
                    mask_text(w)
                ));
            }
            s.push_str(&format!("Bănuiți acum: {}\n", mask_text(r.suspects())));
        }
        if view == 0 && r.phase != Phase::Pick {
            s.push_str(&format!(
                "Secretul tău (J1): ești suspectul {}\n",
                r.murderer + 1
            ));
        }
        s.push_str(r.log.trim());
        if r.phase != Phase::Over {
            s.push_str(&format!(
                "\nLa mutare: Jucătorul {}{}.",
                self.to_move().idx() + 1,
                if r.phase == Phase::Pick {
                    " (ucigașul își alege locul)"
                } else {
                    ""
                }
            ));
        }
        s
    }
    fn format_move(&self, m: &Mv) -> String {
        match *m {
            Mv::Pick(k) => format!("sunt {}", k + 1),
            Mv::Wink(k) => format!("clipesc {}", k + 1),
            Mv::Accuse(k) => format!("acuz {}", k + 1),
            Mv::Wait => "astept".into(),
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
    fn witness_contains_the_murderer_and_accusation_resolves() {
        let mut g = Crima::with_seed(5);
        GameObj::play(&mut g, "sunt 4").unwrap();
        GameObj::play(&mut g, "clipesc 7").unwrap();
        let r = GameObj::render(&g);
        assert!(r.contains("Martor 1"), "{r}");
        assert!(g.resolved().witnesses[0] >> 3 & 1 == 1);
        // J2 nu vede secretul
        let mut v = g.clone_box();
        v.set_viewer(Some(Player::Two));
        assert!(!v.render().contains("Secretul tău"));
        let mut u = g.clone_box();
        u.set_viewer(Some(Player::One));
        assert!(u.render().contains("ești suspectul 4"));
        GameObj::play(&mut g, "acuz 4").unwrap();
        assert_eq!(Game::outcome(&g), Some(Outcome::Win(Player::Two)));
    }

    #[test]
    fn random_and_ai_games_finish() {
        random_finishes(|s| Box::new(Crima::with_seed(s)), 8, 200);
        ai_vs_random(|s| Box::new(Crima::with_seed(s)), 2, Player::One, 200);
        ai_vs_random(|s| Box::new(Crima::with_seed(s)), 3, Player::Two, 200);
    }
}
