//! Oina (jocul românesc cu bâta și mingea), 2 jucători — adaptare simplificată în reprize.
//!
//! Oina reală se joacă în două echipe de 11 pe un teren cu linie de start, linie de var și tarc; aici
//! o versiune stilizată (variantă aleasă de noi): 3 reprize pentru fiecare, cu rolurile alternând;
//! o repriză durează până la 3 «arși» (ieșiri).
//! * Bătăușul alege lovitura: `usoara` (greșește 10%), `medie` (25%) sau `puternica` (45%).
//! * Apoi apărătorul alege unde stă: `aproape`, `mijloc` sau `departe`.
//! * `arunca` rezolvă: dacă bătăușul a greșit, e ars. Altfel mingea cade în zona dată de forța loviturii
//!   (ușoară: aproape 70% / mijloc 30%; medie: aproape 25% / mijloc 55% / departe 20%; puternică:
//!   mijloc 25% / departe 75%). Dacă apărătorul stă exact în zona mingii o prinde din zbor și bătăușul e ars;
//!   altfel bătăușul aleargă: aproape = 1 punct, mijloc = 2, departe = 4 (bătăușul trece prin tarc și înapoi).
//! * Câștigă cine are mai multe puncte după 6 reprize, egal = remiză.
//!
//! Mutări: `usoara`/`medie`/`puternica`, apoi `aproape`/`mijloc`/`departe`, apoi `arunca`.
#![allow(clippy::manual_is_multiple_of, clippy::needless_range_loop)]

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const INNINGS: u8 = 6;
const MISS: [u32; 3] = [10, 25, 45];
const ZONES: [[u32; 3]; 3] = [[70, 30, 0], [25, 55, 20], [0, 25, 75]];
const RUNS: [i32; 3] = [1, 2, 4];
const ZN: [&str; 3] = ["aproape", "mijloc", "departe"];
const SN: [&str; 3] = ["usoara", "medie", "puternica"];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Swing(u8),
    Stand(u8),
    Throw,
}

#[derive(Clone)]
pub struct Oina {
    rng: Rng,
    inning: u8,
    outs: u8,
    score: [i32; 2],
    swing: Option<u8>,
    stand: Option<u8>,
    log: String,
}

impl Default for Oina {
    fn default() -> Self {
        Self::new()
    }
}

impl Oina {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        Oina {
            rng: Rng::new(seed),
            inning: 0,
            outs: 0,
            score: [0, 0],
            swing: None,
            stand: None,
            log: String::new(),
        }
    }
    fn batter(&self) -> Player {
        if self.inning % 2 == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    /// Valoarea așteptată (în puncte) a unei lovituri contra unei poziții.
    fn ev(swing: u8, stand: u8) -> f64 {
        let hit = 1.0 - MISS[swing as usize] as f64 / 100.0;
        let mut e = 0.0;
        for z in 0..3 {
            if z as u8 != stand {
                e += ZONES[swing as usize][z] as f64 / 100.0 * RUNS[z] as f64;
            }
        }
        hit * e
    }
}

impl Game for Oina {
    type Move = Move;

    fn to_move(&self) -> Player {
        if self.swing.is_none() || self.stand.is_some() {
            self.batter()
        } else {
            self.batter().other()
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.inning >= INNINGS {
            return vec![];
        }
        match (self.swing, self.stand) {
            (None, _) => (0..3).map(Move::Swing).collect(),
            (Some(_), None) => (0..3).map(Move::Stand).collect(),
            (Some(_), Some(_)) => vec![Move::Throw],
        }
    }

    fn apply(&mut self, m: &Move) {
        match *m {
            Move::Swing(s) => self.swing = Some(s),
            Move::Stand(z) => self.stand = Some(z),
            Move::Throw => {
                let (s, st) = (self.swing.take().unwrap(), self.stand.take().unwrap());
                let b = self.batter();
                if (self.rng.below(100) as u32) < MISS[s as usize] {
                    self.outs += 1;
                    self.log = format!("J{} greșește mingea: ars ({}/3).", b.idx() + 1, self.outs);
                } else {
                    let mut r = self.rng.below(100) as u32;
                    let mut zone = 2;
                    for z in 0..3 {
                        if r < ZONES[s as usize][z] {
                            zone = z;
                            break;
                        }
                        r -= ZONES[s as usize][z];
                    }
                    if zone as u8 == st {
                        self.outs += 1;
                        self.log = format!(
                            "Mingea cade {} și e prinsă din zbor: J{} ars ({}/3).",
                            ZN[zone],
                            b.idx() + 1,
                            self.outs
                        );
                    } else {
                        self.score[b.idx()] += RUNS[zone];
                        self.log = format!(
                            "Mingea cade {}: J{} aleargă și marchează {}.",
                            ZN[zone],
                            b.idx() + 1,
                            RUNS[zone]
                        );
                    }
                }
                if self.outs >= 3 {
                    self.outs = 0;
                    self.inning += 1;
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.inning < INNINGS {
            return None;
        }
        Some(match self.score[0].cmp(&self.score[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let mut v = (self.score[p.idx()] - self.score[p.other().idx()]) * 10;
        if let (Some(s), Some(st)) = (self.swing, self.stand) {
            let e = Self::ev(s, st);
            let sign = if self.batter() == p { 1.0 } else { -1.0 };
            v += (sign * e * 10.0) as i32;
        } else if let Some(s) = self.swing {
            // apărătorul alege: cel mai bun răspuns la lovitură e vizibil (mutare secvențială, informație perfectă)
            let _ = s;
        }
        v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Oina — repriza {}/{INNINGS}, arși {}/3 | scor J1 {} – J2 {}\nBate: J{}\n",
            (self.inning + 1).min(INNINGS),
            self.outs,
            self.score[0],
            self.score[1],
            self.batter().idx() + 1
        );
        if let Some(sw) = self.swing {
            s.push_str(&format!("Lovitură aleasă: {}\n", SN[sw as usize]));
        }
        if let Some(st) = self.stand {
            s.push_str(&format!(
                "Apărătorul stă {}: scrie «arunca».\n",
                ZN[st as usize]
            ));
        }
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        match self.outcome() {
            Some(Outcome::Win(p)) => s.push_str(&format!("Jucătorul {} câștigă.\n", p.idx() + 1)),
            Some(Outcome::Draw) => s.push_str("Remiză.\n"),
            None => s.push_str(&format!(
                "La mutare: Jucătorul {}\n",
                self.to_move().idx() + 1
            )),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Swing(i) => SN[*i as usize].into(),
            Move::Stand(i) => ZN[*i as usize].into(),
            Move::Throw => "arunca".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_games_end() {
        for seed in 0..30u64 {
            let mut g = Oina::with_seed(seed);
            let mut r = Rng::new(seed + 7);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[r.below(m.len())]);
                n += 1;
                assert!(n < 2000);
            }
        }
    }

    #[test]
    fn zone_tables_sum_to_100() {
        for z in ZONES {
            assert_eq!(z.iter().sum::<u32>(), 100);
        }
    }
}
