#![allow(clippy::needless_range_loop, clippy::manual_range_contains)]
//! Tenis de masă (Ping-pong) — duel pe schimburi de mingi, 2 jucători.
//!
//! * Un set până la 11 puncte, diferență de 2; serviciul se schimbă la fiecare 2 puncte (la
//!   10–10 la fiecare punct). La 40 de puncte totale câștigă cine conduce.
//! * Cine lovește alege **zona** (`st`, `ct`, `dr` = stânga, centru, dreapta) și **forța** (`sigur`,
//!   `atac`, `smash`): `ct-atac`, `dr-smash`. Primul schimb e serviciul. Apoi `schimb` (forțat)
//!   rezolvă lovitura.
//! * Probabilitatea ca mingea să intră: 92% / 78% / 62%. Adversarul o returnează cu 80% / 65% /
//!   45% minus 25% pentru fiecare zonă dintre poziția lui și zona mingii (poziția = zona ultimei lui
//!   lovituri primite; la serviciu e în centru). Minge ratată la intrare = punct pentru adversar;
//!   minge neînapoiată = punct pentru cel care a lovit.
//! * Aruncările de monedă sunt la mutarea forțată `schimb`, deci AI-ul nu le vede în avans.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const IN: [i32; 3] = [92, 78, 62];
const RET: [i32; 3] = [80, 65, 45];
const ZONES: [&str; 3] = ["st", "ct", "dr"];
const POWERS: [&str; 3] = ["sigur", "atac", "smash"];
const CAP_POINTS: u32 = 40;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Hit(u8, u8),
    Resolve,
}

#[derive(Clone)]
pub struct PingPong {
    rng: Rng,
    score: [u32; 2],
    server: Player,
    hitter: Player,
    pos: [u8; 2],
    pending: Option<(u8, u8)>,
    rally: u32,
    log: String,
}

impl Default for PingPong {
    fn default() -> Self {
        Self::new()
    }
}

/// Probabilitatea (în sutimi) ca ministerul să ia punctul cel care lovește, plus continuare la 50%.
pub fn hitter_edge(zone: u8, power: u8, def_pos: u8) -> i32 {
    let d = (zone as i32 - def_pos as i32).abs();
    let ret = (RET[power as usize] - 25 * d).max(5);
    let q_in = IN[power as usize];
    // P(punct hitter) = in·(1−ret) + in·ret·0,5 ; (în zeci de mii → sutimi)
    (q_in * (100 - ret) + q_in * ret / 2) / 100
}

impl PingPong {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        PingPong {
            rng: Rng::new(seed),
            score: [0, 0],
            server: Player::One,
            hitter: Player::One,
            pos: [1, 1],
            pending: None,
            rally: 0,
            log: String::new(),
        }
    }

    pub fn score(&self) -> (u32, u32) {
        (self.score[0], self.score[1])
    }

    fn over(&self) -> bool {
        let (a, b) = (self.score[0], self.score[1]);
        (a >= 11 || b >= 11) && a.abs_diff(b) >= 2 || a + b >= CAP_POINTS
    }

    fn point(&mut self, p: Player) {
        self.score[p.idx()] += 1;
        let total = self.score[0] + self.score[1];
        let both_ten = self.score[0] >= 10 && self.score[1] >= 10;
        if both_ten || total.is_multiple_of(2) {
            self.server = self.server.other();
        }
        self.hitter = self.server;
        self.pos = [1, 1];
        self.rally = 0;
        self.pending = None;
    }
}

impl Game for PingPong {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.hitter
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over() {
            return Vec::new();
        }
        if self.pending.is_some() {
            return vec![Move::Resolve];
        }
        let mut v = Vec::new();
        for z in 0..3u8 {
            for p in 0..3u8 {
                v.push(Move::Hit(z, p));
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        match *m {
            Move::Hit(z, p) => {
                self.pending = Some((z, p));
                self.rally += 1;
            }
            Move::Resolve => {
                let (z, pw) = self.pending.take().unwrap();
                let h = self.hitter;
                let d = h.other();
                let dist = (z as i32 - self.pos[d.idx()] as i32).abs();
                let ret = (RET[pw as usize] - 25 * dist).max(5);
                let what = if self.rng.below(100) as i32 >= IN[pw as usize] {
                    self.log = format!(
                        "J{} lovește {}-{}: minge ratată",
                        h.idx() + 1,
                        ZONES[z as usize],
                        POWERS[pw as usize]
                    );
                    self.point(d);
                    return;
                } else if (self.rng.below(100) as i32) < ret && self.rally < 40 {
                    "returnată"
                } else {
                    self.log = format!(
                        "J{} lovește {}-{}: punct",
                        h.idx() + 1,
                        ZONES[z as usize],
                        POWERS[pw as usize]
                    );
                    self.point(h);
                    return;
                };
                self.log = format!(
                    "J{} lovește {}-{}: {}",
                    h.idx() + 1,
                    ZONES[z as usize],
                    POWERS[pw as usize],
                    what
                );
                self.pos[d.idx()] = z;
                self.hitter = d;
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over() {
            return None;
        }
        Some(match self.score[0].cmp(&self.score[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let (me, op) = (p.idx(), 1 - p.idx());
        let mut v = (self.score[me] as i32 - self.score[op] as i32) * 100;
        if let Some((z, pw)) = self.pending {
            let e = hitter_edge(z, pw, self.pos[self.hitter.other().idx()]);
            v += if self.hitter == p { e } else { -e };
        }
        v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Tenis de masă — scor: J1 {} – J2 {} | serviciu: J{}\n",
            self.score[0],
            self.score[1],
            self.server.idx() + 1
        );
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        if self.over() {
            s.push_str("Set încheiat.\n");
            return s;
        }
        s.push_str(&format!(
            "Poziții (st/ct/dr): J1 {}, J2 {} | schimb: {}\n",
            ZONES[self.pos[0] as usize], ZONES[self.pos[1] as usize], self.rally
        ));
        if let Some((z, p)) = self.pending {
            s.push_str(&format!(
                "Lovitură: {}-{}. Scrie «schimb».\n",
                ZONES[z as usize], POWERS[p as usize]
            ));
        }
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.hitter.idx() + 1));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Hit(z, p) => format!("{}-{}", ZONES[*z as usize], POWERS[*p as usize]),
            Move::Resolve => "schimb".into(),
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
    fn edge_prefers_far_zone() {
        // adversar în stânga: lovitura în dreapta e mai bună
        assert!(hitter_edge(2, 1, 0) > hitter_edge(0, 1, 0));
    }

    #[test]
    fn win_by_two_and_serve_rotation() {
        let mut g = PingPong::with_seed(1);
        g.score = [10, 10];
        assert!(!g.over());
        g.score = [11, 9];
        assert!(g.over());
        g.score = [11, 10];
        assert!(!g.over());
        let mut h = PingPong::with_seed(1);
        h.point(Player::One);
        assert_eq!(h.server, Player::One);
        h.point(Player::Two);
        assert_eq!(h.server, Player::Two);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=8u64 {
            let mut g = PingPong::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() && n < 4000 {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed}");
        }
    }
}
