//! Jocul regal din Ur (Mesopotamia, c. 2600 î.Hr.), cu regulile reconstituite de Irving Finkel.
//!
//! * Fiecare jucător are 7 piese și parcurge un drum de 14 pătrate: 4 private (1–4), 8 comune
//!   (5–12, cele din mijlocul tablei) și 2 private (13–14). Poziția 0 = piese neintrate,
//!   15 = piesă ieșită. Rozetele sunt pe pozițiile 4, 8 (comună) și 14.
//! * Aruncarea: patru zaruri tetraedrice (0 sau 1 fiecare) = 0–4. Cu 0 pierzi mutarea.
//! * O piesă înaintează exact atâtea pătrate cât ai aruncat; ieșirea cere număr exact (poziția
//!   15). Nu poți ateriza pe propria piesă. Pe un pătrat comun capturezi piesa adversă
//!   (care se întoarce la start), cu excepția rozetei centrale (8), care e sigură.
//!   Cine aterizează pe o rozetă aruncă din nou.
//! * Câștigă primul care scoate toate cele 7 piese.
//!
//! Aruncarea e o mutare separată (`arunc`), ca AI-ul să nu vadă aruncările viitoare.
//! Mutări: `arunc`, `de-la-spre` (ex. `0-3`, `12-15`), `pas` (când nu ai mutare).

use crate::q5_util::verdict;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const PIECES: u8 = 7;
const ROSETTES: [usize; 3] = [4, 8, 14];

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Roll,
    Go(u8, u8),
    Pass,
}

#[derive(Clone)]
pub struct Ur {
    /// ocupare[jucător][poziție 1..=14]
    occ: [[bool; 16]; 2],
    home: [u8; 2],
    off: [u8; 2],
    side: Player,
    roll: Option<u8>,
    rng: Rng,
    log: String,
}

impl Default for Ur {
    fn default() -> Self {
        Self::new()
    }
}

fn shared(pos: usize) -> bool {
    (5..=12).contains(&pos)
}

impl Ur {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Ur {
            occ: [[false; 16]; 2],
            home: [PIECES; 2],
            off: [0; 2],
            side: Player::One,
            roll: None,
            rng: Rng::new(seed),
            log: String::new(),
        }
    }

    fn targets(&self, r: u8) -> Vec<(u8, u8)> {
        let me = self.side.idx();
        let mut v = Vec::new();
        if r == 0 {
            return v;
        }
        for from in 0..=14usize {
            let present = if from == 0 {
                self.home[me] > 0
            } else {
                self.occ[me][from]
            };
            if !present {
                continue;
            }
            let to = from + r as usize;
            if to > 15 {
                continue;
            }
            if to < 15 {
                if self.occ[me][to] {
                    continue;
                }
                if to == 8 && self.occ[1 - me][8] {
                    continue;
                }
            }
            v.push((from as u8, to as u8));
        }
        v
    }

    pub fn progress(&self, p: Player) -> i32 {
        let i = p.idx();
        let mut s = self.off[i] as i32 * 15;
        for pos in 1..=14 {
            if self.occ[i][pos] {
                s += pos as i32;
            }
        }
        s
    }

    /// Probabilitatea (în 16-imi) să cadă exact `d` (0..=4).
    fn p16(d: usize) -> i32 {
        [1, 4, 6, 4, 1][d]
    }

    /// Risc: suma, pe piesele lui `p` din zona comună, (pierdere × probabilitatea ca adversarul să lovească).
    fn danger(&self, p: Player) -> i32 {
        let i = p.idx();
        let o = 1 - i;
        let mut risk = 0;
        for pos in 5..=12usize {
            if !self.occ[i][pos] || pos == 8 {
                continue;
            }
            let mut hit = 0;
            for d in 1..=4usize {
                if pos >= d {
                    let from = pos - d;
                    let present = if from == 0 {
                        self.home[o] > 0
                    } else {
                        self.occ[o][from]
                    };
                    if present {
                        hit += Self::p16(d);
                    }
                }
            }
            risk += pos as i32 * hit.min(16);
        }
        risk
    }
}

impl Game for Ur {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        let Some(r) = self.roll else {
            return vec![Move::Roll];
        };
        let t = self.targets(r);
        if t.is_empty() {
            return vec![Move::Pass];
        }
        t.into_iter().map(|(a, b)| Move::Go(a, b)).collect()
    }

    fn apply(&mut self, m: &Move) {
        let me = self.side.idx();
        match m {
            Move::Roll => {
                let r: u8 = (0..4).map(|_| self.rng.below(2) as u8).sum();
                self.roll = Some(r);
                self.log = format!("Jucător {}: aruncă {r}.", me + 1);
            }
            Move::Pass => {
                self.log = format!("Jucător {}: nicio mutare posibilă.", me + 1);
                self.roll = None;
                self.side = self.side.other();
            }
            Move::Go(from, to) => {
                let (from, to) = (*from as usize, *to as usize);
                if from == 0 {
                    self.home[me] -= 1;
                } else {
                    self.occ[me][from] = false;
                }
                let mut text = format!("Jucător {}: {from}→{to}", me + 1);
                if to == 15 {
                    self.off[me] += 1;
                } else {
                    self.occ[me][to] = true;
                    if shared(to) && self.occ[1 - me][to] {
                        self.occ[1 - me][to] = false;
                        self.home[1 - me] += 1;
                        text.push_str(" (captură)");
                    }
                }
                self.log = text;
                self.roll = None;
                if !ROSETTES.contains(&to) || to == 15 {
                    self.side = self.side.other();
                } else {
                    self.log.push_str(", rozetă: aruncă din nou");
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        for p in [Player::One, Player::Two] {
            if self.off[p.idx()] == PIECES {
                return Some(Outcome::Win(p));
            }
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let o = p.other();
        let mut s = (self.progress(p) - self.progress(o)) * 10;
        s += (self.off[p.idx()] as i32 - self.off[o.idx()] as i32) * 40;
        s -= self.danger(p) / 2;
        s += self.danger(o) / 3;
        if self.occ[p.idx()][8] {
            s += 40;
        }
        if self.occ[o.idx()][8] {
            s -= 40;
        }
        s
    }

    fn render(&self) -> String {
        let mut s = String::from("Jocul regal din Ur\n");
        for p in [Player::One, Player::Two] {
            let i = p.idx();
            let on: Vec<String> = (1..=14)
                .filter(|&k| self.occ[i][k])
                .map(|k| k.to_string())
                .collect();
            s.push_str(&format!(
                "J{}: la start {}, pe drum [{}], ieșite {}/{}\n",
                i + 1,
                self.home[i],
                on.join(" "),
                self.off[i],
                PIECES
            ));
        }
        // drumul: 1-4 privat, 5-12 comun, 13-14 privat
        let cell = |pos: usize| -> String {
            let a = self.occ[0][pos];
            let b = self.occ[1][pos];
            let rose = if ROSETTES.contains(&pos) { "*" } else { " " };
            let ch = if a && b {
                '?'
            } else if a {
                'o'
            } else if b {
                'x'
            } else {
                '.'
            };
            format!("{pos:>2}{rose}{ch}")
        };
        let row: Vec<String> = (1..=14).map(cell).collect();
        s.push_str(&format!(
            "Drum (pozițiile 5–12 sunt comune; * = rozetă; o = J1, x = J2):\n{}\n",
            row.join(" ")
        ));
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        let v = verdict(self.outcome());
        if v.is_empty() {
            match self.roll {
                None => s.push_str(&format!(
                    "La mutare: Jucător {} — arunc\n",
                    self.side.idx() + 1
                )),
                Some(r) => s.push_str(&format!(
                    "La mutare: Jucător {} (aruncare {r})\n",
                    self.side.idx() + 1
                )),
            }
        } else {
            s.push_str(&v);
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Roll => "arunc".into(),
            Move::Pass => "pas".into(),
            Move::Go(a, b) => format!("{a}-{b}"),
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
    fn rosette_gives_extra_turn_and_capture_on_shared_only() {
        let mut g = Ur::with_seed(1);
        g.roll = Some(4);
        g.apply(&Move::Go(0, 4));
        assert_eq!(g.side, Player::One, "rozeta 4 dă aruncare în plus");
        g.roll = Some(1);
        g.occ[0][4] = false;
        g.occ[0][7] = true;
        g.occ[1][8] = true; // rozeta centrală ocupată: nu poți intra
        assert!(g.targets(1).iter().all(|&(_, t)| t != 8));
        g.occ[1][8] = false;
        g.occ[1][9] = true;
        g.occ[0][8] = true;
        g.occ[0][7] = false;
        g.apply(&Move::Go(8, 9));
        assert!(!g.occ[1][9]);
        assert_eq!(g.home[1], 8);
    }

    #[test]
    fn exact_roll_needed_to_bear_off() {
        let mut g = Ur::with_seed(2);
        g.home[0] = 0;
        g.occ[0][13] = true;
        g.roll = Some(3);
        assert!(g.targets(3).is_empty());
        assert_eq!(g.targets(2), vec![(13, 15)]);
    }

    #[test]
    fn random_and_ai_games() {
        check_game(Ur::with_seed, 6, 3000);
    }
}
