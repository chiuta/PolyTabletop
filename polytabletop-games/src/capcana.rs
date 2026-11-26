//! Capcana — joc de bluf și licitație cu discuri ascunse, variantă originală pentru doi.
//!
//! Fiecare jucător are 4 discuri: 3 flori și 1 capcană. Tura de așezare: pui pe rând câte un
//! disc cu fața în jos pe teancul tău (cu cât mai multe, cu atât mai mult de licitat), iar
//! după ce ambii au pus cel puțin unul, poți în loc de așezare să deschizi licitația:
//! „licitez N” = pretinzi că poți întoarce N discuri (din teancurile de pe masă) fără să dai
//! de o capcană. Celălalt poate să ridice (licitez M, M mai mare) sau să dea pas; cine
//! rămâne cu licitația cea mai mare e provocatorul.
//!
//! Provocatorul întoarce (mutarea forțată `intorc`) mai întâi discurile lui, de sus în jos, apoi
//! pe ale adversarului, de sus în jos, până la N discuri. Fără capcană = un punct (2 puncte
//! câștigă meciul). Cu capcană = pierde un disc: dacă a fost capcana lui, alege el ce disc pierde;
//! dacă a fost a adversarului, pierde unul la întâmplare. Fără discuri = ești eliminat.
//! Runda următoare o începe provocatorul.
//!
//! Informație ascunsă: vezi propriul teanc, nu și pe al adversarului. AI-ul nu citește teancul
//! adversarului și alege licitarea după probabilități (și după propriul teanc, pe care îl știe).
#![allow(clippy::needless_range_loop, clippy::doc_lazy_continuation)]

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    PlaceBead,
    PlaceTrap,
    Bid(u8),
    Pass,
    Flip,
    LoseBead,
    LoseTrap,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Place,
    /// Răspunde `side` la licitația `bid` a celuilalt.
    Bid {
        bid: u8,
    },
    Reveal {
        challenger: usize,
        n: u8,
        flipped: u8,
    },
    Lose {
        who: usize,
    },
}

#[derive(Clone)]
pub struct Capcana {
    /// Whose private info `render` shows (`None` = the mover's, hot-seat).
    viewer: Option<Player>,
    /// Discuri în mână (nepuse): flori, capcană.
    hand: [(u8, u8); 2],
    /// Teancul fiecărui jucător; `true` = capcană; vârful = ultimul.
    mat: [Vec<bool>; 2],
    /// Câte discuri din teanc sunt întoarse (de sus).
    flipped: [u8; 2],
    pts: [u8; 2],
    side: Player,
    phase: Phase,
    winner: Option<Player>,
    rng: Rng,
    heur: (usize, i32),
    log: String,
}

impl Default for Capcana {
    fn default() -> Self {
        Self::new()
    }
}

impl Capcana {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Capcana {
            viewer: None,
            hand: [(3, 1); 2],
            mat: [Vec::new(), Vec::new()],
            flipped: [0; 2],
            pts: [0; 2],
            side: Player::One,
            phase: Phase::Place,
            winner: None,
            rng: Rng::new(seed),
            heur: (0, 0),
            log: String::new(),
        }
    }

    pub fn points(&self) -> (u8, u8) {
        (self.pts[0], self.pts[1])
    }

    pub fn discs(&self, p: usize) -> usize {
        self.mat[p].len() + self.hand[p].0 as usize + self.hand[p].1 as usize
    }

    fn on_table(&self) -> u8 {
        (self.mat[0].len() + self.mat[1].len()) as u8
    }

    /// Probabilitatea ca provocatorul `p` să reușească cu licitația `n`, din ce știe `p`.
    fn p_success(&self, p: usize, n: u8) -> f64 {
        let own = &self.mat[p];
        let take = (n as usize).min(own.len());
        // Propriul teanc se întoarce de sus în jos.
        if own.iter().rev().take(take).any(|t| *t) {
            return 0.0;
        }
        let k = n as usize - take;
        if k == 0 {
            return 1.0;
        }
        let m = self.mat[1 - p].len();
        if k > m {
            return 0.0;
        }
        // Capcana adversarului e pe masă cu probabilitate ~ m/4 și, dacă e, e uniformă în teanc.
        0.6f64.powi(k as i32)
    }

    fn move_score(&self, p: usize, m: &Move) -> i32 {
        let noise = (self.rng.clone().below(10)) as i32;
        match *m {
            Move::PlaceBead => 10,
            Move::PlaceTrap => {
                let placed_trap = self.mat[p].iter().any(|t| *t);
                if placed_trap || self.mat[p].is_empty() {
                    -50
                } else {
                    3 + noise
                }
            }
            Move::Bid(n) => {
                let pr = self.p_success(p, n);
                let ev = pr * 100.0 - (1.0 - pr) * 70.0;
                // Prudență: o licitație mică, sigură, pe teancul propriu e bună.
                (ev - 12.0 + f64::from(n) * 1.5) as i32 - self.cur_bid_penalty(n)
            }
            Move::Pass => {
                if let Phase::Bid { bid } = self.phase {
                    // Adversarul a licitat `bid`: știu cât din teancul meu ar trebui să întoarcă.
                    let opp_len = self.mat[1 - p].len() as u8;
                    let sure = if bid <= opp_len {
                        0.65
                    } else {
                        let k = (bid - opp_len) as usize;
                        if self.mat[p].iter().rev().take(k).any(|t| *t) {
                            0.0
                        } else {
                            0.7
                        }
                    };
                    (-100.0 * sure + 30.0) as i32
                } else {
                    0
                }
            }
            Move::Flip | Move::LoseBead | Move::LoseTrap => {
                if *m == Move::LoseTrap {
                    -5
                } else {
                    0
                }
            }
        }
    }

    fn cur_bid_penalty(&self, _n: u8) -> i32 {
        0
    }

    fn reset_round(&mut self, first: usize) {
        for p in 0..2 {
            let (b, t) = (
                self.hand[p].0 + self.mat[p].iter().filter(|x| !**x).count() as u8,
                self.hand[p].1 + self.mat[p].iter().filter(|x| **x).count() as u8,
            );
            self.hand[p] = (b, t);
            self.mat[p].clear();
            self.flipped[p] = 0;
        }
        self.phase = Phase::Place;
        self.side = if first == 0 { Player::One } else { Player::Two };
        // Un jucător fără discuri pe care să le așeze n-ar trebui să existe aici.
    }

    fn lose_disc(&mut self, who: usize, trap: bool) {
        if trap {
            self.hand[who].1 -= 1;
        } else {
            self.hand[who].0 -= 1;
        }
    }

    fn end_round(&mut self, challenger: usize, success: bool) {
        let other = 1 - challenger;
        if success {
            self.pts[challenger] += 1;
            self.log = format!("J{} a reușit provocarea.", challenger + 1);
            if self.pts[challenger] >= 2 {
                self.winner = Some(if challenger == 0 {
                    Player::One
                } else {
                    Player::Two
                });
                return;
            }
            self.reset_round(challenger);
        } else {
            self.log = format!("J{} a dat de o capcană.", challenger + 1);
            // Întoarcem totul în mână ca să putem scoate un disc.
            self.reset_round(challenger);
            let _ = other;
        }
    }
}

impl Game for Capcana {
    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.winner.is_some() {
            return Vec::new();
        }
        let p = self.side.idx();
        match self.phase {
            Phase::Place => {
                let mut v = Vec::new();
                if self.hand[p].0 > 0 {
                    v.push(Move::PlaceBead);
                }
                if self.hand[p].1 > 0 {
                    v.push(Move::PlaceTrap);
                }
                if !self.mat[0].is_empty() && !self.mat[1].is_empty() {
                    for n in 1..=self.on_table() {
                        v.push(Move::Bid(n));
                    }
                }
                v
            }
            Phase::Bid { bid } => {
                let mut v = vec![Move::Pass];
                for n in (bid + 1)..=self.on_table() {
                    v.push(Move::Bid(n));
                }
                v
            }
            Phase::Reveal { .. } => vec![Move::Flip],
            Phase::Lose { who } => {
                let mut v = Vec::new();
                if self.hand[who].0 > 0 {
                    v.push(Move::LoseBead);
                }
                if self.hand[who].1 > 0 {
                    v.push(Move::LoseTrap);
                }
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        let p = self.side.idx();
        self.heur = (p, self.move_score(p, m));
        match (*m, self.phase) {
            (Move::PlaceBead, _) => {
                self.hand[p].0 -= 1;
                self.mat[p].push(false);
                self.side = self.side.other();
                // Dacă celălalt nu mai are discuri de pus și nici nu poate licita, rămâne la mine.
                self.fix_stuck();
            }
            (Move::PlaceTrap, _) => {
                self.hand[p].1 -= 1;
                self.mat[p].push(true);
                self.side = self.side.other();
                self.fix_stuck();
            }
            (Move::Bid(n), Phase::Place) | (Move::Bid(n), Phase::Bid { .. }) => {
                if n == self.on_table() {
                    // Nu se mai poate ridica: celălalt trebuie doar să dea pas sau să se supună.
                }
                self.phase = Phase::Bid { bid: n };
                self.side = self.side.other();
            }
            (Move::Pass, Phase::Bid { bid }) => {
                let challenger = 1 - p;
                self.phase = Phase::Reveal {
                    challenger,
                    n: bid,
                    flipped: 0,
                };
                self.side = if challenger == 0 {
                    Player::One
                } else {
                    Player::Two
                };
            }
            (
                Move::Flip,
                Phase::Reveal {
                    challenger,
                    n,
                    flipped,
                },
            ) => {
                let c = challenger;
                let o = 1 - c;
                let own_unflipped = self.mat[c].len() as u8 - self.flipped[c];
                let trap;
                if own_unflipped > 0 {
                    let idx = self.mat[c].len() - 1 - self.flipped[c] as usize;
                    trap = self.mat[c][idx];
                    self.flipped[c] += 1;
                } else {
                    let idx = self.mat[o].len() - 1 - self.flipped[o] as usize;
                    trap = self.mat[o][idx];
                    self.flipped[o] += 1;
                }
                let flipped = flipped + 1;
                if trap {
                    let own_trap = own_unflipped > 0;
                    if own_trap {
                        self.log = format!("J{} și-a întors propria capcană.", c + 1);
                        // Alege cel provocat ce disc pierde (după ce discurile revin în mână).
                        self.reset_round(c);
                        let owned = self.hand[c];
                        if owned.0 > 0 && owned.1 > 0 {
                            self.phase = Phase::Lose { who: c };
                            self.side = if c == 0 { Player::One } else { Player::Two };
                        } else if owned.0 > 0 {
                            self.lose_disc(c, false);
                            self.after_loss(c);
                        } else {
                            self.lose_disc(c, true);
                            self.after_loss(c);
                        }
                    } else {
                        self.log = format!("J{} a dat de capcana lui J{}.", c + 1, o + 1);
                        self.reset_round(c);
                        let (b, t) = self.hand[c];
                        let r = self.rng.below((b + t) as usize);
                        self.lose_disc(c, r >= b as usize);
                        self.after_loss(c);
                    }
                } else if flipped == n {
                    self.end_round(c, true);
                } else {
                    self.phase = Phase::Reveal {
                        challenger,
                        n,
                        flipped,
                    };
                }
            }
            (Move::LoseBead, Phase::Lose { who }) => {
                self.lose_disc(who, false);
                self.after_loss(who);
            }
            (Move::LoseTrap, Phase::Lose { who }) => {
                self.lose_disc(who, true);
                self.after_loss(who);
            }
            _ => {}
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.winner.map(Outcome::Win)
    }

    fn evaluate(&self, p: Player) -> i32 {
        if self.heur.0 == p.idx() {
            self.heur.1
        } else {
            -self.heur.1
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }

    fn render(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            "Capcana — puncte: J1 {}, J2 {} | discuri: J1 {}, J2 {}\n",
            self.pts[0],
            self.pts[1],
            self.discs(0),
            self.discs(1)
        ));
        let mat = |p: usize, secret: bool| {
            if self.mat[p].is_empty() {
                return "(gol)".to_string();
            }
            // De jos în sus; vârful e ultimul.
            let n = self.mat[p].len();
            let mut out = Vec::new();
            for (i, t) in self.mat[p].iter().enumerate() {
                let from_top = n - 1 - i;
                let shown = (from_top as u8) < self.flipped[p];
                if shown || !secret {
                    out.push(if *t { "capcană" } else { "floare" });
                } else {
                    out.push("?");
                }
            }
            out.join(" ")
        };
        let v = self.viewer.unwrap_or_else(|| self.to_move()).idx();
        s.push_str(&format!(
            "Teancul tău (J{}, jos→sus): {}\n",
            v + 1,
            mat(v, false)
        ));
        s.push_str(&format!(
            "Teancul lui J{} (jos→sus): {}\n",
            2 - v,
            mat(1 - v, true)
        ));
        s.push_str(&format!(
            "În mână: tu {} flori, {} capcană | adversarul {} discuri\n",
            self.hand[v].0,
            self.hand[v].1,
            self.hand[1 - v].0 + self.hand[1 - v].1
        ));
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        if self.winner.is_none() {
            s.push_str(&match self.phase {
                Phase::Place => format!(
                    "Mută J{}: pun floare / pun capcana / licitez N (N ≤ {}).\n",
                    self.side.idx() + 1,
                    self.on_table()
                ),
                Phase::Bid { bid } => format!(
                    "Mută J{}: adversarul a licitat {bid}: licitez N (N > {bid}) sau pas.\n",
                    self.side.idx() + 1
                ),
                Phase::Reveal { n, flipped, .. } => {
                    format!("Provocatorul întoarce discuri ({flipped}/{n}): intorc.\n")
                }
                Phase::Lose { who } => format!(
                    "J{} alege ce disc pierde: pierd floare / pierd capcana.\n",
                    who + 1
                ),
            });
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::PlaceBead => "pun floare".into(),
            Move::PlaceTrap => "pun capcana".into(),
            Move::Bid(n) => format!("licitez {n}"),
            Move::Pass => "pas".into(),
            Move::Flip => "intorc".into(),
            Move::LoseBead => "pierd floare".into(),
            Move::LoseTrap => "pierd capcana".into(),
        }
    }
}

impl Capcana {
    /// După o pierdere de disc: eliminare sau rundă nouă (provocatorul începe).
    fn after_loss(&mut self, who: usize) {
        if self.discs(who) == 0 {
            self.winner = Some(if who == 0 { Player::Two } else { Player::One });
            return;
        }
        // `who` e provocatorul: începe runda.
        self.phase = Phase::Place;
        self.side = if who == 0 { Player::One } else { Player::Two };
        self.fix_stuck();
    }

    /// Dacă jucătorul la mutare nu mai are discuri de pus și nu poate licita încă, trece
    /// la celălalt (cazul 1 disc rămas).
    fn fix_stuck(&mut self) {
        if self.phase != Phase::Place {
            return;
        }
        let p = self.side.idx();
        let can_place = self.hand[p].0 + self.hand[p].1 > 0;
        let can_bid = !self.mat[0].is_empty() && !self.mat[1].is_empty();
        if !can_place && !can_bid {
            self.side = self.side.other();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn conserve(g: &Capcana) -> bool {
        // fiecare jucător: discuri = inițiale − pierdute; verificăm doar nenegativitatea.
        g.discs(0) <= 4 && g.discs(1) <= 4
    }

    #[test]
    fn place_then_bid() {
        let mut g = Capcana::with_seed(1);
        assert!(g.legal_moves().iter().all(|m| !matches!(m, Move::Bid(_))));
        g.apply(&Move::PlaceBead);
        assert!(g.legal_moves().iter().all(|m| !matches!(m, Move::Bid(_))));
        g.apply(&Move::PlaceBead);
        assert!(g.legal_moves().contains(&Move::Bid(2)));
        assert!(!g.legal_moves().contains(&Move::Bid(3)));
    }

    #[test]
    fn successful_challenge_scores() {
        let mut g = Capcana::with_seed(2);
        g.apply(&Move::PlaceBead);
        g.apply(&Move::PlaceTrap);
        // J1 licitează 1 (propria floare): reușește.
        g.apply(&Move::Bid(1));
        g.apply(&Move::Pass);
        assert_eq!(g.legal_moves(), vec![Move::Flip]);
        g.apply(&Move::Flip);
        assert_eq!(g.points(), (1, 0));
        assert_eq!(g.to_move(), Player::One);
        assert!(g.mat[0].is_empty());
        assert_eq!(g.discs(0), 4);
    }

    #[test]
    fn hitting_opponent_trap_loses_a_disc() {
        let mut g = Capcana::with_seed(3);
        g.apply(&Move::PlaceBead);
        g.apply(&Move::PlaceTrap);
        g.apply(&Move::Bid(2));
        g.apply(&Move::Pass);
        g.apply(&Move::Flip); // propria floare
        assert_eq!(g.points(), (0, 0));
        g.apply(&Move::Flip); // capcana lui J2
        assert_eq!(g.discs(0), 3);
        assert_eq!(g.discs(1), 4);
        assert!(g.mat[0].is_empty() && g.mat[1].is_empty());
        assert_eq!(g.to_move(), Player::One);
    }

    #[test]
    fn own_trap_lets_you_choose() {
        let mut g = Capcana::with_seed(4);
        g.apply(&Move::PlaceTrap);
        g.apply(&Move::PlaceBead);
        g.apply(&Move::Bid(1));
        g.apply(&Move::Pass);
        g.apply(&Move::Flip);
        assert_eq!(g.legal_moves(), vec![Move::LoseBead, Move::LoseTrap]);
        g.apply(&Move::LoseTrap);
        assert_eq!(g.hand[0], (3, 0));
    }

    #[test]
    fn raising_and_max_bid() {
        let mut g = Capcana::with_seed(5);
        g.apply(&Move::PlaceBead);
        g.apply(&Move::PlaceBead);
        g.apply(&Move::Bid(1));
        let l = g.legal_moves();
        assert!(l.contains(&Move::Pass) && l.contains(&Move::Bid(2)) && !l.contains(&Move::Bid(1)));
        g.apply(&Move::Bid(2));
        assert_eq!(g.legal_moves(), vec![Move::Pass]);
    }

    #[test]
    fn random_games_terminate() {
        for seed in 1..60 {
            let mut g = Capcana::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty(), "{}", g.render());
                g.apply(&m[rng.below(m.len())]);
                assert!(conserve(&g));
                n += 1;
                assert!(n < 20000);
            }
        }
    }

    #[test]
    fn ai_beats_random() {
        let mut wins = 0;
        for seed in 1..=60u64 {
            let mut g = Capcana::with_seed(seed);
            let mut rng = Rng::new(seed + 1);
            while g.outcome().is_none() {
                let m = if g.to_move() == Player::One {
                    polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap()
                } else {
                    let l = g.legal_moves();
                    l[rng.below(l.len())]
                };
                g.apply(&m);
            }
            if g.outcome() == Some(Outcome::Win(Player::One)) {
                wins += 1;
            }
        }
        assert!(wins >= 34, "wins {wins}");
    }
}
