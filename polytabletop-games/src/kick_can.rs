#![allow(clippy::needless_range_loop, clippy::manual_is_multiple_of)]
//! Kick the Can (Lovește cutia) — joc de ascuns/prins pe un teren de 7×7, adaptat pentru 2 jucători.
//!
//! Jucătorul 1 este **Căutătorul** („It”), jucătorul 2 conduce **trei ascunși**. Cutia stă în centrul terenului (d4).
//! Fiecare jucător mută două pași pe tură (două mutări, fiecare un pas în 8 direcții: n ne e se s sv v nv).
//! Căutătorul prinde un ascuns călcând pe pătratul lui: ascunsul merge la închisoare. Un ascuns liber care
//! pășește pe cutie (dacă Căutătorul nu stă pe ea) **lovește cutia** și eliberează toți prinșii, care reapar
//! în colțurile libere. Ascunșii nu pot păși pe Căutător sau pe alt ascuns. Căutătorul câștigă dacă
//! toți trei sunt la închisoare; ascunșii câștigă dacă rezistă 20 de ture. Mutări: Căutătorul `n`, `se`, … ;
//! ascunșii `h2 ne` (ascunsul 2 se mută spre nord-est). Fără informație ascunsă, fără noroc.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const SIZE: i8 = 7;
const CAN: (i8, i8) = (3, 3);
const ROUNDS: u8 = 20;
const DIRS: [(&str, i8, i8); 8] = [
    ("n", 0, 1),
    ("ne", 1, 1),
    ("e", 1, 0),
    ("se", 1, -1),
    ("s", 0, -1),
    ("sv", -1, -1),
    ("v", -1, 0),
    ("nv", -1, 1),
];
const CORNERS: [(i8, i8); 4] = [(0, 0), (6, 0), (0, 6), (6, 6)];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Move {
    /// 0 = Căutătorul, 1..=3 = ascunsul
    who: u8,
    dir: u8,
}

#[derive(Clone)]
pub struct KickCan {
    seeker: (i8, i8),
    hiders: [Option<(i8, i8)>; 3],
    jailed: [bool; 3],
    side: Player,
    steps: u8,
    round: u8,
    log: String,
}

impl Default for KickCan {
    fn default() -> Self {
        Self::new()
    }
}

impl KickCan {
    pub fn new() -> Self {
        KickCan {
            seeker: CAN,
            hiders: [Some((0, 0)), Some((6, 0)), Some((6, 6))],
            jailed: [false; 3],
            side: Player::One,
            steps: 2,
            round: 1,
            log: String::new(),
        }
    }

    pub fn jailed(&self) -> usize {
        self.jailed.iter().filter(|&&j| j).count()
    }

    fn inside(p: (i8, i8)) -> bool {
        (0..SIZE).contains(&p.0) && (0..SIZE).contains(&p.1)
    }

    fn occupied_by_hider(&self, p: (i8, i8)) -> Option<usize> {
        (0..3).find(|&i| self.hiders[i] == Some(p))
    }
}

impl Game for KickCan {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        let mut v = Vec::new();
        if self.side == Player::One {
            for (d, &(_, dx, dy)) in DIRS.iter().enumerate() {
                let t = (self.seeker.0 + dx, self.seeker.1 + dy);
                if Self::inside(t) {
                    v.push(Move {
                        who: 0,
                        dir: d as u8,
                    });
                }
            }
        } else {
            for i in 0..3 {
                if let Some(p) = self.hiders[i] {
                    for (d, &(_, dx, dy)) in DIRS.iter().enumerate() {
                        let t = (p.0 + dx, p.1 + dy);
                        if Self::inside(t)
                            && t != self.seeker
                            && self.occupied_by_hider(t).is_none()
                        {
                            v.push(Move {
                                who: i as u8 + 1,
                                dir: d as u8,
                            });
                        }
                    }
                }
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        let (_, dx, dy) = DIRS[m.dir as usize];
        if m.who == 0 {
            self.seeker = (self.seeker.0 + dx, self.seeker.1 + dy);
            if let Some(i) = self.occupied_by_hider(self.seeker) {
                self.hiders[i] = None;
                self.jailed[i] = true;
                self.log = format!("Căutătorul a prins ascunsul {}", i + 1);
            }
        } else {
            let i = m.who as usize - 1;
            let p = self.hiders[i].unwrap();
            let t = (p.0 + dx, p.1 + dy);
            self.hiders[i] = Some(t);
            if t == CAN && self.jailed.iter().any(|&j| j) {
                let mut free: Vec<(i8, i8)> = CORNERS
                    .iter()
                    .copied()
                    .filter(|&c| self.occupied_by_hider(c).is_none() && c != self.seeker)
                    .collect();
                for j in 0..3 {
                    if self.jailed[j] {
                        if let Some(c) = free.pop() {
                            self.hiders[j] = Some(c);
                            self.jailed[j] = false;
                        }
                    }
                }
                self.log = "Cutia a fost lovită: prinșii sunt eliberați!".into();
            }
        }
        self.steps -= 1;
        if self.steps == 0 {
            self.steps = 2;
            if self.side == Player::Two {
                self.round += 1;
            }
            self.side = self.side.other();
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.jailed.iter().all(|&j| j) {
            Some(Outcome::Win(Player::One))
        } else if self.round > ROUNDS {
            Some(Outcome::Win(Player::Two))
        } else {
            None
        }
    }

    fn evaluate(&self, p: Player) -> i32 {
        let dist = |a: (i8, i8), b: (i8, i8)| (a.0 - b.0).abs().max((a.1 - b.1).abs()) as i32;
        let jailed = self.jailed() as i32;
        let mut v = 1000 * jailed;
        let mut nearest = 10;
        for h in self.hiders.iter().flatten() {
            nearest = nearest.min(dist(*h, self.seeker));
            // ascunșii apropiați de cutie sunt periculoși când există prinși
            v -= 8 * (6 - dist(*h, CAN)) * jailed;
            v += 6 * dist(*h, self.seeker).min(4);
        }
        let seeker_val = v
            - 15 * nearest
            - if jailed > 0 {
                12 * dist(self.seeker, CAN)
            } else {
                0
            };
        if p == Player::One {
            seeker_val
        } else {
            -seeker_val
        }
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Kick the Can — tura {}/{} | prinși: {}/3\n",
            self.round.min(ROUNDS),
            ROUNDS,
            self.jailed()
        );
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        for y in (0..SIZE).rev() {
            s.push_str(&format!("{} ", y + 1));
            for x in 0..SIZE {
                let c = if self.seeker == (x, y) {
                    'S'
                } else if let Some(i) = self.occupied_by_hider((x, y)) {
                    (b'1' + i as u8) as char
                } else if (x, y) == CAN {
                    'O'
                } else {
                    '.'
                };
                s.push(c);
                s.push(' ');
            }
            s.push('\n');
        }
        s.push_str("  a b c d e f g   (S = Căutător, 1-3 = ascunși, O = cutia)\n");
        s.push_str(&format!(
            "La mutare: Jucătorul {} ({} pași rămași)\n",
            self.side.idx() + 1,
            self.steps
        ));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        let d = DIRS[m.dir as usize].0;
        if m.who == 0 {
            d.to_string()
        } else {
            format!("h{} {}", m.who, d)
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    #[test]
    fn seeker_tags_and_kick_frees() {
        let mut g = KickCan::new();
        g.seeker = (1, 1);
        g.hiders[0] = Some((2, 2));
        g.apply(&Move { who: 0, dir: 1 }); // ne
        assert!(g.jailed[0] && g.hiders[0].is_none());
        g.apply(&Move { who: 0, dir: 0 });
        assert_eq!(g.to_move(), Player::Two);
        g.seeker = (6, 6);
        g.hiders[1] = Some((3, 4));
        g.apply(&Move { who: 2, dir: 4 }); // s: spre cutie
        assert!(!g.jailed[0]);
        assert!(g.hiders[0].is_some());
    }

    #[test]
    fn hiders_cannot_enter_seeker() {
        let mut g = KickCan::new();
        g.side = Player::Two;
        g.seeker = (1, 0);
        let mv = g.legal_moves();
        assert!(!mv.iter().any(|m| m.who == 1 && m.dir == 2));
    }

    #[test]
    fn random_games_end_and_ai_catches() {
        for seed in 1..=6u64 {
            let mut g = KickCan::new();
            let mut rng = Rng::new(seed);
            let mut k = 0;
            while g.outcome().is_none() && k < 4000 {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
                k += 1;
            }
            assert!(g.outcome().is_some());
        }
    }
}
