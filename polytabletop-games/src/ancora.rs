//! Ancora și coroana (după jocul britanic „Crown and Anchor”), 2 jucători contra băncii.
//!
//! * Trei zaruri cu simboluri în loc de puncte: coroană, ancoră, inimă, romb, treflă, pică.
//!   Pe fiecare simbol se poate miza. După aruncarea celor trei zaruri, pentru fiecare miză
//!   pe un simbol se numără zarurile cu acel simbol: niciunul = miza se pierde; unul = miza
//!   înapoi plus câștig 1:1; două = înapoi plus 2:1; trei = înapoi plus 3:1 (avantajul
//!   băncii ≈ 7,9%).
//! * Format ales de noi (jocul tradițional nu are scor de meci): 12 runde; fiecare jucător
//!   pornește cu 100 de jetoane și joacă împotriva băncii pe aceleași zaruri; la sfârșit
//!   câștigă cine are mai multe jetoane (egal = remiză). Într-o rundă pui oricâte mize
//!   (câte una pe simbol: 5, 10 sau 20) cât te țin jetoanele, apoi `gata`. Cine începe
//!   mizele alternează (J1 în rundele impare), al doilea vede mizele primului; zarurile
//!   se aruncă într-un pas forțat (`arunc`), cu `Rng`-ul jocului.
//! * Titlu propriu pentru a evita nume de marcă; numele tradițional e doar în descriere.
//!
//! Mutări: `pun ancora 10`, `pun coroana 5`, `gata`, `arunc`. Simboluri: coroana, ancora,
//! inima, romb, trefla, pica.
//!
//! AI: `AlphaBeta(1)`; evaluarea e probabilitatea de câștig a meciului (logistică pe
//! diferența de jetoane după rundă, calculată exact pe cele 216 aruncări posibile), deci
//! mizează mult când e în urmă și puțin când e în față.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const ROUNDS: u8 = 12;
const START: i32 = 100;
pub const SYMBOLS: [&str; 6] = ["coroana", "ancora", "inima", "romb", "trefla", "pica"];
const AMOUNTS: [i32; 3] = [5, 10, 20];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Bet(u8), // 0 = primul care mizează, 1 = al doilea
    Roll,
    Over,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Move {
    Bet(u8, i32),
    Done,
    Roll,
}

#[derive(Clone)]
pub struct Ancora {
    rng: Rng,
    chips: [i32; 2],
    bets: [[i32; 6]; 2],
    round: u8,
    phase: Phase,
    last: String,
}

impl Default for Ancora {
    fn default() -> Self {
        Self::new()
    }
}

/// Câștigul net al unui set de mize pentru zarurile `d` (valori 0..6).
pub fn payoff(bets: &[i32; 6], d: [u8; 3]) -> i32 {
    let mut net = 0;
    for (s, &b) in bets.iter().enumerate() {
        if b == 0 {
            continue;
        }
        let m = d.iter().filter(|&&x| x as usize == s).count() as i32;
        net += if m == 0 { -b } else { b * m };
    }
    net
}

impl Ancora {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Ancora {
            rng: Rng::new(seed),
            chips: [START; 2],
            bets: [[0; 6]; 2],
            round: 1,
            phase: Phase::Bet(0),
            last: String::new(),
        }
    }

    pub fn chips(&self) -> [i32; 2] {
        self.chips
    }

    fn starter(&self) -> Player {
        if self.round % 2 == 1 {
            Player::One
        } else {
            Player::Two
        }
    }

    fn staked(&self, p: Player) -> i32 {
        self.bets[p.idx()].iter().sum()
    }

    fn win_prob(&self, p: Player) -> f64 {
        let me = p.idx();
        let d0 = (self.chips[me] - self.chips[1 - me]) as f64;
        let left = (ROUNDS - self.round) as f64;
        let scale = 9.0 * (left + 0.5).sqrt();
        let mut acc = 0.0;
        for a in 0..6u8 {
            for b in 0..6u8 {
                for c in 0..6u8 {
                    let x =
                        payoff(&self.bets[me], [a, b, c]) - payoff(&self.bets[1 - me], [a, b, c]);
                    acc += 1.0 / (1.0 + (-(d0 + x as f64) / scale).exp());
                }
            }
        }
        acc / 216.0
    }
}

impl Game for Ancora {
    type Move = Move;

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Bet(0) | Phase::Roll | Phase::Over => self.starter(),
            Phase::Bet(_) => self.starter().other(),
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        match self.phase {
            Phase::Over => Vec::new(),
            Phase::Roll => vec![Move::Roll],
            Phase::Bet(_) => {
                let p = self.to_move();
                let free = self.chips[p.idx()] - self.staked(p);
                let mut v = Vec::new();
                for s in 0..6u8 {
                    if self.bets[p.idx()][s as usize] == 0 {
                        for a in AMOUNTS {
                            if a <= free {
                                v.push(Move::Bet(s, a));
                            }
                        }
                    }
                }
                v.push(Move::Done);
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Bet(s, a) => {
                let p = self.to_move();
                self.bets[p.idx()][*s as usize] = *a;
            }
            Move::Done => {
                self.phase = match self.phase {
                    Phase::Bet(0) => Phase::Bet(1),
                    _ => Phase::Roll,
                };
            }
            Move::Roll => {
                let d = [
                    self.rng.below(6) as u8,
                    self.rng.below(6) as u8,
                    self.rng.below(6) as u8,
                ];
                let mut log = format!(
                    "Zaruri: {} {} {}.",
                    SYMBOLS[d[0] as usize], SYMBOLS[d[1] as usize], SYMBOLS[d[2] as usize]
                );
                for p in 0..2 {
                    let net = payoff(&self.bets[p], d);
                    self.chips[p] += net;
                    log.push_str(&format!(
                        " J{}: {}{}.",
                        p + 1,
                        if net > 0 { "+" } else { "" },
                        net
                    ));
                }
                self.last = log;
                self.bets = [[0; 6]; 2];
                if self.round >= ROUNDS {
                    self.phase = Phase::Over;
                } else {
                    self.round += 1;
                    self.phase = Phase::Bet(0);
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        Some(match self.chips[0].cmp(&self.chips[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        if self.phase == Phase::Over {
            let me = p.idx();
            return (self.chips[me] - self.chips[1 - me]).signum() * 10_000;
        }
        (self.win_prob(p) * 10_000.0).round() as i32
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Ancora și coroana — runda {}/{ROUNDS}. Jetoane: J1 {}, J2 {} (start {START}).\n",
            self.round.min(ROUNDS),
            self.chips[0],
            self.chips[1]
        );
        for p in 0..2 {
            let b: Vec<String> = self.bets[p]
                .iter()
                .enumerate()
                .filter(|(_, &a)| a > 0)
                .map(|(i, a)| format!("{} {}", SYMBOLS[i], a))
                .collect();
            s.push_str(&format!(
                "Mize J{}: {}\n",
                p + 1,
                if b.is_empty() {
                    "—".into()
                } else {
                    b.join(", ")
                }
            ));
        }
        if !self.last.is_empty() {
            s.push_str(&format!("Runda anterioară — {}\n", self.last));
        }
        match self.phase {
            Phase::Over => match self.outcome() {
                Some(Outcome::Win(p)) => s.push_str(&format!(
                    "Jocul s-a încheiat: Jucător {} câștigă.\n",
                    p.idx() + 1
                )),
                _ => s.push_str("Jocul s-a încheiat: remiză.\n"),
            },
            Phase::Roll => s.push_str("Banca aruncă zarurile: arunc\n"),
            Phase::Bet(_) => s.push_str(&format!(
                "La mutare: Jucător {} — pun <simbol> <5|10|20> sau gata\n",
                self.to_move().idx() + 1
            )),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Bet(s, a) => format!("pun {} {}", SYMBOLS[*s as usize], a),
            Move::Done => "gata".into(),
            Move::Roll => "arunc".into(),
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
    fn payoffs_follow_the_published_table() {
        let mut b = [0; 6];
        b[1] = 10; // ancora
        assert_eq!(payoff(&b, [0, 2, 3]), -10); // niciunul
        assert_eq!(payoff(&b, [1, 2, 3]), 10); // unul: 1:1
        assert_eq!(payoff(&b, [1, 1, 3]), 20); // doi: 2:1
        assert_eq!(payoff(&b, [1, 1, 1]), 30); // trei: 3:1
        b[0] = 5;
        assert_eq!(payoff(&b, [1, 2, 3]), 5); // 10 - 5
    }

    #[test]
    fn house_edge_is_about_seven_point_nine_percent() {
        let mut b = [0; 6];
        b[2] = 100;
        let mut total = 0;
        for a in 0..6u8 {
            for c in 0..6u8 {
                for d in 0..6u8 {
                    total += payoff(&b, [a, c, d]);
                }
            }
        }
        // EV = -100*125/216 + 100*75/216 + 200*15/216 + 300/216 = -17 (în sutimi: -0.0787*216 = -17)
        assert_eq!(total, -1700);
    }

    #[test]
    fn round_flow_and_chip_limits() {
        let mut g = Ancora::with_seed(1);
        assert_eq!(g.to_move(), Player::One);
        g.apply(&Move::Bet(0, 20));
        assert!(!g.legal_moves().contains(&Move::Bet(0, 5))); // câte o miză pe simbol
        g.apply(&Move::Done);
        assert_eq!(g.to_move(), Player::Two);
        g.apply(&Move::Done);
        assert_eq!(g.legal_moves(), vec![Move::Roll]);
        g.apply(&Move::Roll);
        assert_eq!(g.round, 2);
        assert_eq!(g.to_move(), Player::Two);
        assert_eq!(g.chips[1], START);
    }

    #[test]
    fn cannot_stake_more_than_chips() {
        let mut g = Ancora::with_seed(2);
        g.chips[0] = 12;
        g.apply(&Move::Bet(0, 10));
        let mv = g.legal_moves();
        assert!(!mv.iter().any(|m| matches!(m, Move::Bet(_, a) if *a > 2)));
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=20u64 {
            let mut g = Ancora::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 3000);
            }
        }
    }

    #[test]
    fn ai_prefers_risk_when_behind_and_safety_when_ahead() {
        let mut g = Ancora::with_seed(3);
        g.round = 11;
        g.chips = [60, 140]; // J1 mult în urmă
        let big = {
            let mut h = g.clone();
            h.apply(&Move::Bet(0, 20));
            h.apply(&Move::Bet(1, 20));
            h.apply(&Move::Bet(2, 20));
            h.evaluate(Player::One)
        };
        let none = g.evaluate(Player::One);
        assert!(big > none);
        g.chips = [140, 60];
        let big2 = {
            let mut h = g.clone();
            h.apply(&Move::Bet(0, 20));
            h.evaluate(Player::One)
        };
        assert!(big2 < g.evaluate(Player::One));
    }

    #[test]
    fn ai_finishes_a_game() {
        use polytabletop_core::GameObj;
        let mut g: Box<dyn GameObj> = Box::new(Ancora::with_seed(8));
        let mut n = 0;
        while g.outcome().is_none() {
            let m = g.ai_move(n).unwrap();
            g.play(&m).unwrap();
            n += 1;
            assert!(n < 1000);
        }
    }
}
