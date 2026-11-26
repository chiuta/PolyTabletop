//! Orășel — zaruri și construit orașe, variantă originală pentru doi (mecanică „arunci
//! zarurile, colectezi din cărțile cu numărul ieșit”).
//!
//! Fiecare începe cu un Lan și o Brutărie și 3 monede. La tura ta arunci 1 zar (sau 2, după
//! ce ai construit Gara), apoi cumperi cel mult o clădire din rezerva comună (6 din fiecare)
//! sau construiești un reper. Câștigă primul care are toate cele 4 repere.
//!
//! Clădiri (nume: cost, numere, efect):
//! * albastre (în orice tură): Lan 1: +1 | Pădure 5: +1 | Livadă 8: +2 | Mină 11–12 (cost 6): +5
//! * verzi (doar tura ta): Brutărie 2–3: +1 | Magazin 4: +3 | Fabrică 6: +3 per Pădure/Mină |
//!   Moară 7: +2 per Lan/Livadă
//! * roșii (în tura adversarului, pe banii celui care aruncă): Cafenea 3: ia 1 | Grătar 9–10: ia 2
//! Repere: Gară (4): arunci 2 zaruri | Centru comercial (10): +1 la Brutărie, Magazin,
//! Cafenea, Grătar | Parc (16): la dublă, încă o tură | Turn (22): poți arunca din nou o dată.
//!
//! Zarurile sunt publice; AI-ul alege 1 sau 2 zaruri după valoarea așteptată, nu după
//! rezultatul real. Plafon de 150 de runde: câștigă cine are mai multe repere, apoi monede.
#![allow(clippy::needless_range_loop, clippy::doc_lazy_continuation)]

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const NAMES: [&str; 10] = [
    "lan", "padure", "livada", "mina", "brutarie", "magazin", "fabrica", "moara", "cafenea",
    "gratar",
];
const COST: [i32; 10] = [1, 2, 3, 6, 1, 2, 4, 3, 2, 3];
const LM_NAMES: [&str; 4] = ["gara", "mall", "parc", "turn"];
const LM_COST: [i32; 4] = [4, 10, 16, 22];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Col {
    Blue,
    Green,
    Red,
}

fn color(k: usize) -> Col {
    match k {
        0..=3 => Col::Blue,
        4..=7 => Col::Green,
        _ => Col::Red,
    }
}

fn nums(k: usize) -> &'static [u8] {
    match k {
        0 => &[1],
        1 => &[5],
        2 => &[8],
        3 => &[11, 12],
        4 => &[2, 3],
        5 => &[4],
        6 => &[6],
        7 => &[7],
        8 => &[3],
        _ => &[9, 10],
    }
}

fn base_amount(k: usize) -> i32 {
    match k {
        0 | 1 | 4 => 1,
        2 | 7 => 2,
        3 => 5,
        5 | 6 => 3,
        8 => 1,
        _ => 2,
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Roll(u8),
    Reroll,
    Keep,
    Buy(u8),
    Build(u8),
    Pass,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Roll,
    Reroll,
    Buy,
}

#[derive(Clone)]
pub struct Orasel {
    owned: [[u8; 10]; 2],
    lm: [[bool; 4]; 2],
    coins: [i32; 2],
    supply: [u8; 10],
    side: Player,
    phase: Phase,
    dice: [u8; 2],
    n_dice: u8,
    rerolled: bool,
    extra_used: bool,
    rounds: u32,
    winner: Option<Outcome>,
    rng: Rng,
    heur: (usize, i32),
    log: String,
}

impl Default for Orasel {
    fn default() -> Self {
        Self::new()
    }
}

impl Orasel {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut owned = [[0u8; 10]; 2];
        for o in &mut owned {
            o[0] = 1;
            o[4] = 1;
        }
        let mut supply = [6u8; 10];
        supply[0] -= 2;
        supply[4] -= 2;
        Orasel {
            owned,
            lm: [[false; 4]; 2],
            coins: [3, 3],
            supply,
            side: Player::One,
            phase: Phase::Roll,
            dice: [0, 0],
            n_dice: 1,
            rerolled: false,
            extra_used: false,
            rounds: 1,
            winner: None,
            rng: Rng::new(seed),
            heur: (0, 0),
            log: String::new(),
        }
    }

    pub fn coins(&self) -> (i32, i32) {
        (self.coins[0], self.coins[1])
    }

    pub fn total_buildings(&self) -> usize {
        self.owned
            .iter()
            .flatten()
            .map(|x| *x as usize)
            .sum::<usize>()
            + self.supply.iter().map(|x| *x as usize).sum::<usize>()
    }

    fn mall(&self, p: usize) -> i32 {
        i32::from(self.lm[p][1])
    }

    /// Venitul de bază al cărții `k` pentru jucătorul `p` (cu bonusuri).
    fn amount(&self, p: usize, k: usize) -> i32 {
        let o = &self.owned[p];
        match k {
            4 | 5 | 8 | 9 => base_amount(k) + self.mall(p),
            6 => 3 * (i32::from(o[1]) + i32::from(o[3])),
            7 => 2 * (i32::from(o[0]) + i32::from(o[2])),
            _ => base_amount(k),
        }
    }

    /// Câștigul brut (fără plafonul monedelor) pentru ambii când `roller` obține `r`.
    fn gains(&self, r: u8, roller: usize) -> [i32; 2] {
        let mut g = [0i32; 2];
        let other = 1 - roller;
        for k in 0..10 {
            if !nums(k).contains(&r) {
                continue;
            }
            match color(k) {
                Col::Blue => {
                    for p in 0..2 {
                        g[p] += i32::from(self.owned[p][k]) * self.amount(p, k);
                    }
                }
                Col::Green => {
                    g[roller] += i32::from(self.owned[roller][k]) * self.amount(roller, k)
                }
                Col::Red => {
                    let t = i32::from(self.owned[other][k]) * self.amount(other, k);
                    g[other] += t;
                    g[roller] -= t;
                }
            }
        }
        g
    }

    fn dist(n: u8) -> Vec<(u8, f64)> {
        if n == 1 {
            (1..=6).map(|r| (r, 1.0 / 6.0)).collect()
        } else {
            (2..=12u8)
                .map(|s| {
                    let ways = 6 - (i32::from(s) - 7).unsigned_abs() as u8;
                    (s, f64::from(ways) / 36.0)
                })
                .collect()
        }
    }

    /// Diferența așteptată de venit pe rundă (a mea − a adversarului) pentru `p`.
    fn net_income(&self, p: usize) -> f64 {
        let o = 1 - p;
        let mut w = 0.0;
        let nd = |q: usize| if self.lm[q][0] { 2 } else { 1 };
        for (r, pr) in Self::dist(nd(p)) {
            let g = self.gains(r, p);
            w += pr * f64::from(g[p] - g[o]);
        }
        for (r, pr) in Self::dist(nd(o)) {
            let g = self.gains(r, o);
            w += pr * f64::from(g[p] - g[o]);
        }
        w
    }

    fn roll_income(&self, p: usize, n: u8) -> f64 {
        let mut w = 0.0;
        for (r, pr) in Self::dist(n) {
            let g = self.gains(r, p);
            w += pr * f64::from(g[p] - g[1 - p]);
        }
        w
    }

    fn move_score(&self, p: usize, m: &Move) -> i32 {
        let before = self.net_income(p);
        let v = match *m {
            Move::Roll(n) => 10.0 * self.roll_income(p, n),
            Move::Keep => {
                let r = self.dice[0] + self.dice[1];
                let g = self.gains(r, p);
                10.0 * f64::from(g[p] - g[1 - p])
            }
            Move::Reroll => 10.0 * self.roll_income(p, self.n_dice) - 1.0,
            Move::Pass => 0.0,
            Move::Buy(k) => {
                let mut c = self.clone();
                c.owned[p][k as usize] += 1;
                let d = c.net_income(p) - before;
                7.0 * d - 0.3 * f64::from(COST[k as usize])
            }
            Move::Build(l) => {
                let mut c = self.clone();
                c.lm[p][l as usize] = true;
                let d = c.net_income(p) - before;
                let extra = match l {
                    0 => 2.0,
                    2 => 3.0,
                    3 => 2.0,
                    _ => 0.0,
                };
                7.0 * d + 0.45 * f64::from(LM_COST[l as usize]) + extra
            }
        };
        v as i32
    }

    fn resolve(&mut self, r: u8) {
        let roller = self.side.idx();
        let other = 1 - roller;
        let mut log = format!("J{} a obținut {r}.", roller + 1);
        // Roșii întâi.
        for k in [8usize, 9] {
            if nums(k).contains(&r) {
                let t = (i32::from(self.owned[other][k]) * self.amount(other, k))
                    .min(self.coins[roller]);
                if t > 0 {
                    self.coins[roller] -= t;
                    self.coins[other] += t;
                    log.push_str(&format!(" J{} ia {t} de la J{}.", other + 1, roller + 1));
                }
            }
        }
        let mut g = [0i32; 2];
        for k in 0..4 {
            if nums(k).contains(&r) {
                for p in 0..2 {
                    g[p] += i32::from(self.owned[p][k]) * self.amount(p, k);
                }
            }
        }
        for k in 4..8 {
            if nums(k).contains(&r) {
                let n = i32::from(self.owned[roller][k]);
                g[roller] += n * self.amount(roller, k);
            }
        }
        for p in 0..2 {
            self.coins[p] += g[p];
            if g[p] > 0 {
                log.push_str(&format!(" J{} +{}.", p + 1, g[p]));
            }
        }
        self.log = log;
    }

    fn finish_turn(&mut self) {
        let p = self.side.idx();
        if self.lm[p].iter().all(|x| *x) {
            self.winner = Some(Outcome::Win(self.side));
            return;
        }
        let dbl = self.n_dice == 2 && self.dice[0] == self.dice[1];
        if dbl && self.lm[p][2] && !self.extra_used {
            self.extra_used = true;
        } else {
            self.extra_used = false;
            self.side = self.side.other();
            if self.side == Player::One {
                self.rounds += 1;
                if self.rounds > 150 {
                    let sc = |q: usize| {
                        (
                            self.lm[q].iter().filter(|x| **x).count() as i32,
                            self.coins[q],
                        )
                    };
                    self.winner = Some(match sc(0).cmp(&sc(1)) {
                        std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                        std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                        std::cmp::Ordering::Equal => Outcome::Draw,
                    });
                }
            }
        }
        self.phase = Phase::Roll;
        self.rerolled = false;
    }
}

impl Game for Orasel {
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
            Phase::Roll => {
                let mut v = vec![Move::Roll(1)];
                if self.lm[p][0] {
                    v.push(Move::Roll(2));
                }
                v
            }
            Phase::Reroll => vec![Move::Keep, Move::Reroll],
            Phase::Buy => {
                let mut v = vec![Move::Pass];
                for k in 0..10 {
                    if self.supply[k] > 0 && self.coins[p] >= COST[k] {
                        v.push(Move::Buy(k as u8));
                    }
                }
                for l in 0..4 {
                    if !self.lm[p][l] && self.coins[p] >= LM_COST[l] {
                        v.push(Move::Build(l as u8));
                    }
                }
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        let p = self.side.idx();
        self.heur = (p, self.move_score(p, m));
        match *m {
            Move::Roll(_) | Move::Reroll => {
                let n = if let Move::Roll(n) = m {
                    *n
                } else {
                    self.n_dice
                };
                self.n_dice = n;
                self.dice = [self.rng.d6(), if n == 2 { self.rng.d6() } else { 0 }];
                if matches!(m, Move::Reroll) {
                    self.rerolled = true;
                }
                if self.lm[p][3] && !self.rerolled {
                    self.phase = Phase::Reroll;
                } else {
                    self.resolve(self.dice[0] + self.dice[1]);
                    self.phase = Phase::Buy;
                }
            }
            Move::Keep => {
                self.resolve(self.dice[0] + self.dice[1]);
                self.phase = Phase::Buy;
            }
            Move::Buy(k) => {
                self.coins[p] -= COST[k as usize];
                self.supply[k as usize] -= 1;
                self.owned[p][k as usize] += 1;
                self.finish_turn();
            }
            Move::Build(l) => {
                self.coins[p] -= LM_COST[l as usize];
                self.lm[p][l as usize] = true;
                self.finish_turn();
            }
            Move::Pass => self.finish_turn(),
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.winner
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
            "Orășel — runda {} | monede: J1 {}, J2 {}\n",
            self.rounds, self.coins[0], self.coins[1]
        ));
        for p in 0..2 {
            let b: Vec<String> = (0..10)
                .filter(|k| self.owned[p][*k] > 0)
                .map(|k| format!("{}×{}", NAMES[k], self.owned[p][k]))
                .collect();
            let l: Vec<&str> = (0..4)
                .filter(|i| self.lm[p][*i])
                .map(|i| LM_NAMES[i])
                .collect();
            s.push_str(&format!(
                "J{}: {} | repere: {}\n",
                p + 1,
                b.join(", "),
                if l.is_empty() {
                    "—".to_string()
                } else {
                    l.join(", ")
                }
            ));
        }
        let sup: Vec<String> = (0..10)
            .map(|k| {
                format!(
                    "{} {}({}) c{}",
                    NAMES[k],
                    self.supply[k],
                    nums(k)
                        .iter()
                        .map(|n| n.to_string())
                        .collect::<Vec<_>>()
                        .join("/"),
                    COST[k]
                )
            })
            .collect();
        s.push_str(&format!("Rezervă: {}\n", sup.join(" | ")));
        s.push_str("Repere: gara c4 (2 zaruri), mall c10, parc c16, turn c22\n");
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        if self.winner.is_none() {
            s.push_str(&match self.phase {
                Phase::Roll => "Aruncă: arunc 1 / arunc 2.\n".to_string(),
                Phase::Reroll => format!(
                    "Zaruri: {} {} — păstrezi (pastrez) sau arunci din nou (reiau)?\n",
                    self.dice[0], self.dice[1]
                ),
                Phase::Buy => {
                    "Cumperi (cumpar <nume>), construiești (construiesc <reper>) sau nimic.\n"
                        .to_string()
                }
            });
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Roll(n) => format!("arunc {n}"),
            Move::Reroll => "reiau".into(),
            Move::Keep => "pastrez".into(),
            Move::Buy(k) => format!("cumpar {}", NAMES[*k as usize]),
            Move::Build(l) => format!("construiesc {}", LM_NAMES[*l as usize]),
            Move::Pass => "nimic".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dist_sums_to_one() {
        for n in [1, 2] {
            let s: f64 = Orasel::dist(n).iter().map(|x| x.1).sum();
            assert!((s - 1.0).abs() < 1e-9);
        }
    }

    #[test]
    fn income_rules() {
        let mut g = Orasel::with_seed(1);
        // Lan (albastru): ambii câștigă la 1.
        let gs = g.gains(1, 0);
        assert_eq!(gs, [1, 1]);
        // Brutărie doar roller-ul.
        let gs = g.gains(2, 1);
        assert_eq!(gs, [0, 1]);
        // Cafenea a lui J2: la 3 în tura lui J1 ia 1 de la J1.
        g.owned[1][8] = 1;
        let gs = g.gains(3, 0);
        assert_eq!(gs[1], 1);
        assert!(gs[0] <= 1); // brutărie +1, -1 cafenea
                             // Mall: +1 la brutărie.
        g.lm[0][1] = true;
        assert_eq!(g.amount(0, 4), 2);
        // Fabrica: 3 per Pădure.
        g.owned[0][6] = 1;
        g.owned[0][1] = 2;
        assert_eq!(g.gains(6, 0)[0], 6);
    }

    #[test]
    fn red_cannot_overdraw() {
        let mut g = Orasel::with_seed(2);
        g.owned[1][9] = 3; // grătar x3 = 6
        g.coins = [2, 0];
        g.phase = Phase::Roll;
        g.dice = [0, 0];
        // forțăm un 9
        g.resolve(9);
        assert_eq!(g.coins, [0, 2]);
    }

    #[test]
    fn buying_and_winning() {
        let mut g = Orasel::with_seed(3);
        g.phase = Phase::Buy;
        g.coins = [100, 0];
        let moves = g.legal_moves();
        assert!(moves.contains(&Move::Build(3)));
        g.lm[0] = [true, true, true, false];
        g.apply(&Move::Build(3));
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn stations_and_radio() {
        let mut g = Orasel::with_seed(4);
        assert_eq!(g.legal_moves(), vec![Move::Roll(1)]);
        g.lm[0][0] = true;
        assert_eq!(g.legal_moves(), vec![Move::Roll(1), Move::Roll(2)]);
        g.lm[0][3] = true;
        g.apply(&Move::Roll(2));
        assert_eq!(g.legal_moves(), vec![Move::Keep, Move::Reroll]);
        g.apply(&Move::Reroll);
        assert_eq!(g.phase, Phase::Buy);
    }

    #[test]
    fn random_games_end_and_conserve() {
        for seed in 1..30 {
            let mut g = Orasel::with_seed(seed);
            let t = g.total_buildings();
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                g.apply(&m[rng.below(m.len())]);
                assert_eq!(g.total_buildings(), t);
                assert!(g.coins[0] >= 0 && g.coins[1] >= 0);
                n += 1;
                assert!(n < 20000);
            }
        }
    }

    #[test]
    fn ai_beats_random() {
        let mut wins = 0;
        for seed in 1..=40u64 {
            let mut g = Orasel::with_seed(seed);
            let mut rng = Rng::new(seed + 3);
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
        assert!(wins >= 32, "wins {wins}");
    }
}
