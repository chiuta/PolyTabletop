//! Vânătoarea de comori (Treasure hunt, Marea Britanie) — doi jucători, două runde cu roluri inversate.
//!
//! Cine ascunde alege în secret o celulă a unei grile 8×8 (`ascund d5`). Celălalt sapă (`sap c3`) și primește
//! un indiciu de distanță (distanța Manhattan până la comoară): 0 = comoara!, 1–2 foarte cald, 3–4 cald, 5–7 rece,
//! 8+ înghețat. Are cel mult 12 săpături; costul rundei = săpături folosite, eșec = 13. Costul total mai mic câștigă.
//!
//! Informație ascunsă: celula comorii. Verdictul unei săpături apare când urmează mutarea următoare; AI-ul nu citește
//! celula: alege, dintre celulele încă posibile după indiciile primite, una care desparte bine posibilitățile.

use crate::q4_secret::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: i32 = 8;
const MAX_DIGS: u8 = 12;
const FAIL: i32 = 13;

fn cell_name(i: u8) -> String {
    format!(
        "{}{}",
        (b'a' + (i as i32 % N) as u8) as char,
        i as i32 / N + 1
    )
}
fn dist(a: u8, b: u8) -> i32 {
    ((a as i32 % N) - (b as i32 % N)).abs() + ((a as i32 / N) - (b as i32 / N)).abs()
}
fn bucket(d: i32) -> u8 {
    match d {
        0 => 0,
        1..=2 => 1,
        3..=4 => 2,
        5..=7 => 3,
        _ => 4,
    }
}
const BUCKET_NAME: [&str; 5] = ["COMOARA!", "foarte cald", "cald", "rece", "înghețat"];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mv {
    Hide(u8),
    Dig(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Hide,
    Dig,
    Over,
}

#[derive(Clone)]
pub struct Comori {
    viewer: Option<Player>,
    round: u8,
    setter: usize,
    phase: Phase,
    secret: u8,
    digs: Vec<(u8, u8)>,
    pending: Option<u8>,
    cost: [i32; 2],
    noise: u64,
    log: String,
}

impl Default for Comori {
    fn default() -> Self {
        Self::new()
    }
}

impl Comori {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        Comori {
            viewer: None,
            round: 1,
            setter: 0,
            phase: Phase::Hide,
            secret: 0,
            digs: Vec::new(),
            pending: None,
            cost: [0; 2],
            noise: rng.next_u64(),
            log: String::new(),
        }
    }
    pub fn costs(&self) -> (i32, i32) {
        let r = self.resolved();
        (r.cost[0], r.cost[1])
    }
    fn guesser(&self) -> usize {
        1 - self.setter
    }

    fn resolved(&self) -> Comori {
        let mut r = self.clone();
        r.pending = None;
        let Some(c) = self.pending else { return r };
        let b = bucket(dist(c, self.secret));
        r.digs.push((c, b));
        r.log = format!("Sapi la {}: {}.", cell_name(c), BUCKET_NAME[b as usize]);
        let found = b == 0;
        if found || r.digs.len() as u8 >= MAX_DIGS {
            r.cost[r.guesser()] += if found { r.digs.len() as i32 } else { FAIL };
            r.log = format!(
                "Runda {}: comoara era la {}{}.",
                r.round,
                cell_name(r.secret),
                if found {
                    format!(", găsită în {} săpături", r.digs.len())
                } else {
                    ", negăsită".to_string()
                }
            );
            r.round += 1;
            if r.round > 2 {
                r.phase = Phase::Over;
            } else {
                r.setter = 1 - r.setter;
                r.phase = Phase::Hide;
                r.digs.clear();
            }
        }
        r
    }

    fn candidates(&self) -> Vec<u8> {
        (0..(N * N) as u8)
            .filter(|&s| self.digs.iter().all(|&(c, b)| bucket(dist(c, s)) == b))
            .collect()
    }
}

impl Game for Comori {
    type Move = Mv;
    fn to_move(&self) -> Player {
        let r = self.resolved();
        pl(if r.phase == Phase::Hide {
            r.setter
        } else {
            r.guesser()
        })
    }
    fn legal_moves(&self) -> Vec<Mv> {
        let r = self.resolved();
        match r.phase {
            Phase::Over => Vec::new(),
            Phase::Hide => (0..(N * N) as u8).map(Mv::Hide).collect(),
            Phase::Dig => (0..(N * N) as u8)
                .filter(|c| !r.digs.iter().any(|d| d.0 == *c))
                .map(Mv::Dig)
                .collect(),
        }
    }
    fn apply(&mut self, m: &Mv) {
        *self = self.resolved();
        match *m {
            Mv::Hide(c) => {
                self.secret = c;
                self.phase = Phase::Dig;
                self.log = format!(
                    "Runda {}: J{} a ascuns comoara.",
                    self.round,
                    self.setter + 1
                );
            }
            Mv::Dig(c) => self.pending = Some(c),
        }
    }
    fn outcome(&self) -> Option<Outcome> {
        let r = self.resolved();
        (r.phase == Phase::Over).then(|| by_points(r.cost[1], r.cost[0]))
    }
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut v = 1000 * (self.cost[1 - me] - self.cost[me]);
        if let Some(c) = self.pending {
            let cands = self.candidates();
            let n = cands.len().max(1) as f32;
            // valoare: șansa de a găsi acum + cât de bine împarte posibilitățile (după bucket)
            let hit = if cands.contains(&c) { 1.0 / n } else { 0.0 };
            let mut counts = [0usize; 5];
            for &s in &cands {
                counts[bucket(dist(c, s)) as usize] += 1;
            }
            let worst = *counts.iter().max().unwrap() as f32 / n;
            let val = 0.5 * hit + 0.5 * (1.0 - worst);
            let x = (val * 400.0) as i32;
            v += if self.guesser() == me { x } else { -x };
        } else if self.phase == Phase::Dig && self.digs.is_empty() && self.setter == me {
            v += noise(self.noise, self.secret as u64, 50);
        }
        v
    }
    fn render(&self) -> String {
        let r = self.resolved();
        let view = viewer_of(self.viewer, self.to_move());
        let mut s = format!(
            "Runda {}/2. Cost total (mai mic = mai bine): J1 {} – {} J2.\n",
            r.round.min(2),
            r.cost[0],
            r.cost[1]
        );
        if r.phase == Phase::Dig {
            s.push_str("   a b c d e f g h\n");
            for y in (0..N).rev() {
                s.push_str(&format!("{}  ", y + 1));
                for x in 0..N {
                    let c = (y * N + x) as u8;
                    let ch = match r.digs.iter().find(|d| d.0 == c) {
                        Some(&(_, b)) => ['X', '4', '3', '2', '1'][b as usize],
                        None => '.',
                    };
                    s.push(ch);
                    s.push(' ');
                }
                s.push('\n');
            }
            s.push_str(&format!(
                "Săpături: {}/{MAX_DIGS} (4 = foarte cald, 3 = cald, 2 = rece, 1 = înghețat).\n",
                r.digs.len()
            ));
            if view == r.setter {
                s.push_str(&format!(
                    "Secretul tău (J{}): comoara la {}\n",
                    view + 1,
                    cell_name(r.secret)
                ));
            }
        }
        s.push_str(r.log.trim());
        if r.phase != Phase::Over {
            s.push_str(&format!(
                "\nLa mutare: Jucătorul {}.",
                self.to_move().idx() + 1
            ));
        }
        s
    }
    fn format_move(&self, m: &Mv) -> String {
        match *m {
            Mv::Hide(c) => format!("ascund {}", cell_name(c)),
            Mv::Dig(c) => format!("sap {}", cell_name(c)),
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
    fn dig_feedback_and_find() {
        let mut g = Comori::with_seed(1);
        GameObj::play(&mut g, "ascund d5").unwrap();
        let mut s = g.clone_box();
        s.set_viewer(Some(Player::Two));
        assert!(!s.render().contains("Secretul tău"));
        GameObj::play(&mut g, "sap a1").unwrap();
        assert!(
            GameObj::render(&g).contains("înghețat"),
            "{}",
            GameObj::render(&g)
        );
        GameObj::play(&mut g, "sap d5").unwrap();
        assert_eq!(g.costs().1, 2);
    }

    #[test]
    fn random_and_ai_games_finish() {
        random_finishes(|s| Box::new(Comori::with_seed(s)), 6, 300);
        let r = ai_vs_random(|s| Box::new(Comori::with_seed(s)), 2, Player::One, 300);
        assert!(r.is_some());
        ai_vs_random(|s| Box::new(Comori::with_seed(s)), 3, Player::Two, 300);
    }
}
