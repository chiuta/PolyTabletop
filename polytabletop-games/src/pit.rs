//! Pit — joc de negociere cu cărți (Parker Brothers, 1904), adaptare pentru doi jucători pe ture.
//!
//! 3 mărfuri (grâu, ovăz, cafea) × 6 cărți = 18 cărți, 9 pentru fiecare. Scop: **corner** — să ții toate cele 6 cărți
//! ale unei mărfi. Corner-ul valorează 100 (grâu), 75 (ovăz) sau 60 (cafea) puncte; joc până la 200 sau 8 runde.
//! * Tura: cel care propune oferă `n` (1–3) cărți ale unei mărfi, cu fața în jos (`ofer grau 2`).
//! * Celălalt răspunde cu `n` cărți dintr-o marfă din care are cel puțin `n` (`dau ovaz`), tot fără să vadă oferta.
//! * Apoi schimbul se face (`schimb`) și rolurile se inversează. Cine prinde corner-ul oprește runda.
//! * Dacă după 24 de schimburi nimeni nu are corner, runda se anulează.
//!
//! Informație ascunsă: mâinile și cărțile din schimb.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const NAMES: [&str; 3] = ["grau", "ovaz", "cafea"];
const VALUE: [i32; 3] = [100, 75, 60];
const GOAL: i32 = 200;
const ROUNDS: u8 = 8;
const MAX_SWAPS: u8 = 24;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mv {
    Offer(u8, u8),
    Reply(u8),
    Swap,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Offer,
    Reply,
    Swap,
    Over,
}

#[derive(Clone)]
pub struct Pit {
    viewer: Option<Player>,
    hands: [[u8; 3]; 2],
    offer: Option<(u8, u8)>,
    reply: Option<u8>,
    phase: Phase,
    proposer: usize,
    swaps: u8,
    round: u8,
    scores: [i32; 2],
    rng: Rng,
    noise: u64,
    log: String,
}

impl Default for Pit {
    fn default() -> Self {
        Self::new()
    }
}

impl Pit {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut g = Pit {
            viewer: None,
            hands: [[0; 3]; 2],
            offer: None,
            reply: None,
            phase: Phase::Offer,
            proposer: 1,
            swaps: 0,
            round: 0,
            scores: [0; 2],
            rng: Rng::new(seed),
            noise: seed,
            log: String::new(),
        };
        g.deal();
        g
    }
    pub fn scores(&self) -> (i32, i32) {
        (self.scores[0], self.scores[1])
    }
    fn deal(&mut self) {
        self.round += 1;
        self.proposer = 1 - self.proposer;
        loop {
            let mut d: Vec<u8> = (0..18).map(|i| i / 6).collect();
            for i in (1..18).rev() {
                let j = self.rng.below(i + 1);
                d.swap(i, j);
            }
            let mut h = [[0u8; 3]; 2];
            for (i, &c) in d.iter().enumerate() {
                h[i / 9][c as usize] += 1;
            }
            if h.iter().flatten().all(|&n| n < 6) {
                self.hands = h;
                break;
            }
        }
        self.noise = self.rng.next_u64();
        self.offer = None;
        self.reply = None;
        self.phase = Phase::Offer;
        self.swaps = 0;
    }
    fn actor(&self) -> usize {
        match self.phase {
            Phase::Reply => 1 - self.proposer,
            _ => self.proposer,
        }
    }
    fn corner_of(&self, p: usize) -> Option<usize> {
        (0..3).find(|&c| self.hands[p][c] == 6)
    }
    fn end_round(&mut self) {
        self.offer = None;
        self.reply = None;
        if self.scores.iter().any(|&s| s >= GOAL) || self.round >= ROUNDS {
            self.phase = Phase::Over;
        } else {
            let keep = std::mem::take(&mut self.log);
            self.deal();
            self.log = keep;
        }
    }
}

impl Game for Pit {
    type Move = Mv;
    fn to_move(&self) -> Player {
        crate::q4_secret::pl(self.actor())
    }
    fn legal_moves(&self) -> Vec<Mv> {
        let p = self.actor();
        match self.phase {
            Phase::Over => Vec::new(),
            Phase::Offer => {
                let mut v = Vec::new();
                for c in 0..3u8 {
                    for n in 1..=3u8.min(self.hands[p][c as usize]) {
                        v.push(Mv::Offer(c, n));
                    }
                }
                v
            }
            Phase::Reply => {
                let n = self.offer.map(|o| o.1).unwrap_or(1);
                (0..3u8)
                    .filter(|&c| self.hands[p][c as usize] >= n)
                    .map(Mv::Reply)
                    .collect()
            }
            Phase::Swap => vec![Mv::Swap],
        }
    }
    fn apply(&mut self, m: &Mv) {
        let p = self.actor();
        match (*m, self.phase) {
            (Mv::Offer(c, n), Phase::Offer) => {
                self.hands[p][c as usize] -= n;
                self.offer = Some((c, n));
                self.phase = Phase::Reply;
                self.log = format!("J{} oferă {n} cărți (ascunse).", p + 1);
            }
            (Mv::Reply(c), Phase::Reply) => {
                let n = self.offer.unwrap().1;
                self.hands[p][c as usize] -= n;
                self.reply = Some(c);
                self.phase = Phase::Swap;
            }
            (Mv::Swap, Phase::Swap) => {
                let (oc, n) = self.offer.take().unwrap();
                let rc = self.reply.take().unwrap();
                let q = 1 - p;
                self.hands[p][rc as usize] += n;
                self.hands[q][oc as usize] += n;
                self.swaps += 1;
                self.log = format!("Schimb de câte {n} cărți.");
                let w = if let Some(c) = self.corner_of(p) {
                    Some((p, c))
                } else {
                    self.corner_of(q).map(|c| (q, c))
                };
                if let Some((w, c)) = w {
                    self.scores[w] += VALUE[c];
                    self.log = format!("J{} face corner la {} (+{}).", w + 1, NAMES[c], VALUE[c]);
                    self.end_round();
                } else if self.swaps >= MAX_SWAPS {
                    self.log = "Runda s-a anulat fără corner.".into();
                    self.end_round();
                } else {
                    self.proposer = 1 - self.proposer;
                    self.phase = Phase::Offer;
                }
            }
            _ => {}
        }
    }
    fn outcome(&self) -> Option<Outcome> {
        if self.phase == Phase::Over {
            Some(crate::q4_secret::by_points(self.scores[0], self.scores[1]))
        } else {
            None
        }
    }
    fn evaluate(&self, pl: Player) -> i32 {
        let me = pl.idx();
        let mut v = 100 * (self.scores[me] - self.scores[1 - me]);
        // Doar mâna proprie (fără cărțile aflate în schimb și fără mâna adversarului).
        let h = self.hands[me];
        let incoming = if self.phase == Phase::Reply && self.actor() == me {
            self.offer.map(|o| o.1 as i32).unwrap_or(0)
        } else {
            0
        };
        let best = (0..3).map(|c| h[c] as i32).max().unwrap_or(0);
        let held: i32 = h.iter().map(|&n| (n as i32) * (n as i32)).sum();
        v += 30 * best + 3 * held + incoming;
        v + crate::q4_secret::noise(
            self.noise,
            (h[0] as u64) * 7 + h[1] as u64 + self.swaps as u64 * 131,
            7,
        )
    }
    fn render(&self) -> String {
        let view = crate::q4_secret::viewer_of(self.viewer, self.to_move());
        let mut s = format!(
            "Runda {}/{ROUNDS}. Scor: J1 {} – {} J2 (până la {GOAL}).\n",
            self.round.min(ROUNDS),
            self.scores[0],
            self.scores[1]
        );
        if self.phase != Phase::Over {
            let h = self.hands[view];
            s.push_str(&format!(
                "Mâna ta (J{}): {}\n",
                view + 1,
                (0..3)
                    .map(|c| format!("{} ×{}", NAMES[c], h[c]))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            s.push_str(&format!(
                "Adversarul ține {} cărți.\n",
                self.hands[1 - view].iter().map(|&n| n as u32).sum::<u32>()
            ));
            if let Some((c, n)) = self.offer {
                if view == self.proposer {
                    s.push_str(&format!("Ai oferit {n} × {}.\n", NAMES[c as usize]));
                } else {
                    s.push_str(&format!("Adversarul îți oferă {n} cărți (necunoscute).\n"));
                }
            }
            if self.phase == Phase::Swap {
                s.push_str("Ambii au pus cărțile; urmează schimbul.\n");
            }
        }
        s.push_str(self.log.trim());
        if self.phase != Phase::Over {
            s.push_str(&format!("\nLa mutare: Jucătorul {}.", self.actor() + 1));
        }
        s
    }
    fn format_move(&self, m: &Mv) -> String {
        match *m {
            Mv::Offer(c, n) => format!("ofer {} {n}", NAMES[c as usize]),
            Mv::Reply(c) => format!("dau {}", NAMES[c as usize]),
            Mv::Swap => "schimb".into(),
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
    fn corner_scores_and_reply_rules() {
        let mut g = Pit::with_seed(1);
        g.hands = [[5, 3, 1], [1, 3, 5]];
        g.phase = Phase::Offer;
        g.proposer = 0;
        g.round = 1;
        g.apply(&Mv::Offer(1, 1));
        g.apply(&Mv::Reply(0)); // J2 dă grau
        assert_eq!(GameObj::legal_moves(&g), vec!["schimb"]);
        g.apply(&Mv::Swap);
        // J1: grau 5+1 = 6 -> corner (100), runda nouă
        assert_eq!(g.scores, [100, 0]);
        assert_eq!(g.round, 2);
        let mut h = Pit::with_seed(1);
        h.hands = [[5, 2, 2], [1, 4, 4]];
        h.phase = Phase::Offer;
        h.proposer = 0;
        h.apply(&Mv::Offer(0, 2));
        let m = GameObj::legal_moves(&h);
        assert_eq!(m, vec!["dau ovaz", "dau cafea"], "{m:?}");
    }

    #[test]
    fn viewer_hides_hand() {
        let g = Pit::with_seed(3);
        let mut a = g.clone_box();
        a.set_viewer(Some(Player::One));
        let mut b = g.clone_box();
        b.set_viewer(Some(Player::Two));
        assert_ne!(a.render(), b.render());
    }

    #[test]
    fn random_and_ai_games_finish() {
        random_finishes(|s| Box::new(Pit::with_seed(s)), 4, 4000);
        ai_vs_random(|s| Box::new(Pit::with_seed(s)), 2, Player::One, 4000);
        ai_vs_random(|s| Box::new(Pit::with_seed(s)), 3, Player::Two, 4000);
    }
}
