//! Bresla — construire de pachet pentru doi, variantă originală.
//!
//! Începi cu 10 cărți (7 Bani și 3 Parcele), amestecate; tragi 5. În tura ta ai o fază de
//! acțiune (1 acțiune) și una de cumpărare (1 cumpărare); apoi arunci mâna și ce ai jucat și
//! tragi 5 cărți noi (reamesteci aruncatele când pachetul se golește). Cumperi cărți din rezerva
//! comună și le pui în pachetul tău: monede, puncte de victorie sau acțiuni.
//!
//! Comori: Ban (1, cost 0), Argint (2, cost 3), Aur (3, cost 6). Puncte: Parcelă (1p, cost 2),
//! Moșie (3p, cost 5), Domeniu (6p, cost 8). Acțiuni (cost):
//! * Sat (3): +1 carte, +2 acțiuni | Meșteșug (4): +3 cărți | Târg (5): +1 carte, +1 acțiune,
//!   +1 cumpărare, +1 monedă | Sărbătoare (5): +2 acțiuni, +1 cumpărare, +2 monede |
//! * Tăietor (3): +1 cumpărare, +2 monede | Laborator (5): +2 cărți, +1 acțiune |
//! * Atelier (3): primești o carte cu cost până la 4 (`iau argint`) |
//! * Cămătar (4): +2 monede; adversarul aruncă până rămâne cu 3 cărți (`dau parcela`).
//!
//! Mutări: `joc <carte>`, `cumparaturi` (încheie acțiunile; comorile din mână se numără
//! automat), `cumpar <carte>`, `gata` (încheie tura). Jocul se termină când Domeniile sunt
//! epuizate sau trei rezerve sunt goale (verificat la sfârșitul turei) sau după 100 de ture;
//! câștigă cine are mai multe puncte în tot pachetul (egal: mai puține ture).
//!
//! Informație ascunsă: ordinea pachetului și mâna adversarului. AI-ul alege după o euristică
//! statică (nu vede cărțile pe care le-ar trage).
#![allow(clippy::needless_range_loop, clippy::doc_lazy_continuation)]

use crate::cards::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const NAMES: [&str; 14] = [
    "ban",
    "argint",
    "aur",
    "parcela",
    "mosie",
    "domeniu",
    "sat",
    "mestesug",
    "targ",
    "sarbatoare",
    "taietor",
    "laborator",
    "atelier",
    "camatar",
];
const COST: [u8; 14] = [0, 3, 6, 2, 5, 8, 3, 4, 5, 5, 3, 5, 3, 4];
const VP: [i32; 14] = [0, 0, 0, 1, 3, 6, 0, 0, 0, 0, 0, 0, 0, 0];
const COIN: [u8; 3] = [1, 2, 3];

fn is_action(c: u8) -> bool {
    c >= 6
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Play(u8),
    ToBuy,
    Buy(u8),
    End,
    Gain(u8),
    Discard(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Action,
    Buy,
    Gain,
    /// `who` aruncă până la 3 cărți (atac).
    Attacked {
        who: usize,
    },
}

#[derive(Clone)]
pub struct Bresla {
    /// Whose private info `render` shows (`None` = the mover's, hot-seat).
    viewer: Option<Player>,
    supply: [u8; 14],
    deck: [Vec<u8>; 2],
    hand: [Vec<u8>; 2],
    discard: [Vec<u8>; 2],
    inplay: Vec<u8>,
    side: Player,
    phase: Phase,
    actions: u8,
    buys: u8,
    coins: u8,
    turns: [u32; 2],
    over: Option<Outcome>,
    rng: Rng,
    heur: (usize, i32),
}

impl Default for Bresla {
    fn default() -> Self {
        Self::new()
    }
}

impl Bresla {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut supply = [10u8; 14];
        supply[0] = 46;
        supply[1] = 40;
        supply[2] = 30;
        supply[3] = 8;
        supply[4] = 8;
        supply[5] = 8;
        let mut deck: [Vec<u8>; 2] = [Vec::new(), Vec::new()];
        for d in &mut deck {
            for _ in 0..7 {
                d.push(0);
            }
            for _ in 0..3 {
                d.push(3);
            }
            shuffle(d, &mut rng);
        }
        supply[0] -= 14;
        supply[3] -= 0; // parcelele de start nu se iau din rezervă (rezerva de 8 rămâne)
        let mut g = Bresla {
            viewer: None,
            supply,
            deck,
            hand: [Vec::new(), Vec::new()],
            discard: [Vec::new(), Vec::new()],
            inplay: Vec::new(),
            side: Player::One,
            phase: Phase::Action,
            actions: 1,
            buys: 1,
            coins: 0,
            turns: [0; 2],
            over: None,
            rng,
            heur: (0, 0),
        };
        g.draw(0, 5);
        g.draw(1, 5);
        g
    }

    fn draw(&mut self, p: usize, n: usize) {
        for _ in 0..n {
            if self.deck[p].is_empty() {
                if self.discard[p].is_empty() {
                    return;
                }
                let mut d = std::mem::take(&mut self.discard[p]);
                shuffle(&mut d, &mut self.rng);
                self.deck[p] = d;
            }
            if let Some(c) = self.deck[p].pop() {
                self.hand[p].push(c);
            }
        }
        self.hand[p].sort_unstable();
    }

    /// Toate cărțile (rezerva comună nu se numără; doar ce e la jucători).
    pub fn cards_owned(&self) -> usize {
        (0..2)
            .map(|p| self.deck[p].len() + self.hand[p].len() + self.discard[p].len())
            .sum::<usize>()
            + self.inplay.len()
    }

    pub fn conserved_total(&self) -> usize {
        self.cards_owned() + self.supply.iter().map(|x| *x as usize).sum::<usize>()
    }

    pub fn vp(&self, p: usize) -> i32 {
        let mut v = 0;
        let mut count = |c: &[u8]| {
            for x in c {
                v += VP[*x as usize];
            }
        };
        count(&self.deck[p]);
        count(&self.hand[p]);
        count(&self.discard[p]);
        if self.side.idx() == p {
            count(&self.inplay);
        }
        v
    }

    fn owned_count(&self, p: usize, card: u8) -> usize {
        let f = |v: &Vec<u8>| v.iter().filter(|x| **x == card).count();
        f(&self.deck[p])
            + f(&self.hand[p])
            + f(&self.discard[p])
            + if self.side.idx() == p {
                self.inplay.iter().filter(|x| **x == card).count()
            } else {
                0
            }
    }

    fn total_owned(&self, p: usize) -> usize {
        self.deck[p].len()
            + self.hand[p].len()
            + self.discard[p].len()
            + if self.side.idx() == p {
                self.inplay.len()
            } else {
                0
            }
    }

    fn game_end_check(&self) -> bool {
        self.supply[5] == 0 || self.supply.iter().filter(|x| **x == 0).count() >= 3
    }

    fn finish(&mut self) {
        let (a, b) = (self.vp_final(0), self.vp_final(1));
        self.over = Some(if a != b {
            Outcome::Win(if a > b { Player::One } else { Player::Two })
        } else if self.turns[0] != self.turns[1] {
            Outcome::Win(if self.turns[0] < self.turns[1] {
                Player::One
            } else {
                Player::Two
            })
        } else {
            Outcome::Draw
        });
    }

    fn vp_final(&self, p: usize) -> i32 {
        let mut v = 0;
        for x in self.deck[p]
            .iter()
            .chain(&self.hand[p])
            .chain(&self.discard[p])
        {
            v += VP[*x as usize];
        }
        v
    }

    fn cleanup(&mut self) {
        let p = self.side.idx();
        let mut h = std::mem::take(&mut self.hand[p]);
        self.discard[p].append(&mut h);
        let mut ip = std::mem::take(&mut self.inplay);
        self.discard[p].append(&mut ip);
        self.draw(p, 5);
        self.turns[p] += 1;
        if self.game_end_check() || self.turns[0] >= 100 {
            self.finish();
            return;
        }
        self.side = self.side.other();
        self.phase = Phase::Action;
        self.actions = 1;
        self.buys = 1;
        self.coins = 0;
    }

    fn buy_score(&self, p: usize, c: u8) -> f64 {
        let own = |x: u8| self.owned_count(p, x) as f64;
        let total = self.total_owned(p) as f64;
        let late = self.supply[5] <= 4 || self.supply.iter().filter(|x| **x == 0).count() >= 2;
        let money_density = (own(0) + 2.0 * own(1) + 3.0 * own(2)) / total.max(1.0);
        match c {
            5 => {
                if own(2) > 0.0 || self.turns[p] >= 8 || money_density > 1.3 {
                    100.0
                } else {
                    30.0
                }
            }
            2 => 62.0,
            4 => {
                if late {
                    75.0
                } else {
                    8.0
                }
            }
            3 => {
                if self.supply[5] <= 2 {
                    50.0
                } else if late {
                    22.0
                } else {
                    -5.0
                }
            }
            11 => {
                if own(11) < 2.0 {
                    58.0 - 6.0 * own(11)
                } else {
                    20.0
                }
            }
            8 => {
                if own(8) < 2.0 {
                    52.0 - 8.0 * own(8)
                } else {
                    15.0
                }
            }
            9 => {
                if own(9) < 1.0 {
                    44.0
                } else {
                    12.0
                }
            }
            7 => {
                if own(7) < 1.0 {
                    50.0
                } else if own(7) < 2.0 && total > 18.0 {
                    36.0
                } else {
                    10.0
                }
            }
            13 => {
                if own(13) < 1.0 {
                    46.0
                } else if own(13) < 2.0 && total > 20.0 {
                    30.0
                } else {
                    8.0
                }
            }
            1 => 36.0,
            6 => {
                if own(6) < 1.0 && own(7) + own(13) > 0.0 {
                    34.0
                } else {
                    12.0
                }
            }
            10 => 14.0,
            12 => {
                if own(12) < 1.0 {
                    16.0
                } else {
                    4.0
                }
            }
            0 => -10.0,
            _ => 0.0,
        }
    }

    fn move_score(&self, p: usize, m: &Move) -> i32 {
        let v = match *m {
            Move::Play(c) => {
                let base = match c {
                    8 => 62.0,
                    11 => 60.0,
                    9 => 58.0,
                    6 => 56.0,
                    10 => 40.0,
                    7 => 50.0,
                    13 => 44.0,
                    12 => 30.0,
                    _ => 0.0,
                };
                // Cardurile cu +acțiuni înaintea celor terminale.
                base
            }
            Move::ToBuy => 5.0,
            Move::Buy(c) => self.buy_score(p, c) - 10.0 * 0.0,
            Move::End => 22.0,
            Move::Gain(c) => self.buy_score(p, c) * 0.9,
            Move::Discard(c) => {
                let keep = match c {
                    2 => 50.0,
                    1 => 35.0,
                    0 => 20.0,
                    3..=5 => 5.0,
                    _ => 25.0,
                };
                100.0 - keep
            }
        };
        v as i32
    }
}

impl Game for Bresla {
    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }
    type Move = Move;

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Attacked { who } => {
                if who == 0 {
                    Player::One
                } else {
                    Player::Two
                }
            }
            _ => self.side,
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over.is_some() {
            return Vec::new();
        }
        let p = self.side.idx();
        let mut v = Vec::new();
        match self.phase {
            Phase::Action => {
                if self.actions > 0 {
                    let mut seen = Vec::new();
                    for &c in &self.hand[p] {
                        if is_action(c) && !seen.contains(&c) {
                            seen.push(c);
                            v.push(Move::Play(c));
                        }
                    }
                }
                v.push(Move::ToBuy);
            }
            Phase::Buy => {
                if self.buys > 0 {
                    for c in 0..14u8 {
                        if c != 0 && COST[c as usize] <= self.coins && self.supply[c as usize] > 0 {
                            v.push(Move::Buy(c));
                        }
                    }
                }
                v.push(Move::End);
            }
            Phase::Gain => {
                for c in 0..14u8 {
                    if COST[c as usize] <= 4 && self.supply[c as usize] > 0 {
                        v.push(Move::Gain(c));
                    }
                }
            }
            Phase::Attacked { who } => {
                let mut seen = Vec::new();
                for &c in &self.hand[who] {
                    if !seen.contains(&c) {
                        seen.push(c);
                        v.push(Move::Discard(c));
                    }
                }
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        let mover = self.to_move().idx();
        self.heur = (mover, self.move_score(mover, m));
        let p = self.side.idx();
        match *m {
            Move::Play(c) => {
                let i = self.hand[p].iter().position(|x| *x == c).expect("in hand");
                self.hand[p].remove(i);
                self.inplay.push(c);
                self.actions -= 1;
                match c {
                    6 => {
                        self.draw(p, 1);
                        self.actions += 2;
                    }
                    7 => self.draw(p, 3),
                    8 => {
                        self.draw(p, 1);
                        self.actions += 1;
                        self.buys += 1;
                        self.coins += 1;
                    }
                    9 => {
                        self.actions += 2;
                        self.buys += 1;
                        self.coins += 2;
                    }
                    10 => {
                        self.buys += 1;
                        self.coins += 2;
                    }
                    11 => {
                        self.draw(p, 2);
                        self.actions += 1;
                    }
                    12 => self.phase = Phase::Gain,
                    13 => {
                        self.coins += 2;
                        if self.hand[1 - p].len() > 3 {
                            self.phase = Phase::Attacked { who: 1 - p };
                        }
                    }
                    _ => {}
                }
            }
            Move::ToBuy => {
                let mut coins = self.coins;
                let mut keep = Vec::new();
                for c in self.hand[p].drain(..) {
                    if c <= 2 {
                        coins += COIN[c as usize];
                        self.inplay.push(c);
                    } else {
                        keep.push(c);
                    }
                }
                self.hand[p] = keep;
                self.coins = coins;
                self.phase = Phase::Buy;
            }
            Move::Buy(c) => {
                self.coins -= COST[c as usize];
                self.buys -= 1;
                self.supply[c as usize] -= 1;
                self.discard[p].push(c);
            }
            Move::End => self.cleanup(),
            Move::Gain(c) => {
                self.supply[c as usize] -= 1;
                self.discard[p].push(c);
                self.phase = Phase::Action;
            }
            Move::Discard(c) => {
                let who = mover;
                let i = self.hand[who]
                    .iter()
                    .position(|x| *x == c)
                    .expect("in hand");
                self.hand[who].remove(i);
                self.discard[who].push(c);
                if self.hand[who].len() <= 3 {
                    self.phase = Phase::Action;
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.over
    }

    fn evaluate(&self, p: Player) -> i32 {
        if self.heur.0 == p.idx() {
            self.heur.1
        } else {
            -self.heur.1
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }

    fn render(&self) -> String {
        let mut s = String::new();
        let list = |v: &[u8]| {
            if v.is_empty() {
                "—".to_string()
            } else {
                v.iter()
                    .map(|c| NAMES[*c as usize])
                    .collect::<Vec<_>>()
                    .join(" ")
            }
        };
        s.push_str(&format!(
            "Bresla — tura {} | puncte în pachet: J1 {}, J2 {}\n",
            self.turns[0].min(self.turns[1]) + 1,
            self.vp_pub(0),
            self.vp_pub(1)
        ));
        let sup: Vec<String> = (0..14)
            .map(|c| format!("{}:{}(c{})", NAMES[c], self.supply[c], COST[c]))
            .collect();
        s.push_str(&format!("Rezerva: {}\n", sup.join(" ")));
        for p in 0..2 {
            s.push_str(&format!(
                "J{}: pachet {}, aruncate {}, mână {}\n",
                p + 1,
                self.deck[p].len(),
                self.discard[p].len(),
                self.hand[p].len()
            ));
        }
        let v = self.viewer.unwrap_or_else(|| self.to_move()).idx();
        s.push_str(&format!("Mâna ta (J{}): {}\n", v + 1, list(&self.hand[v])));
        s.push_str(&format!("Jucate în tura curentă: {}\n", list(&self.inplay)));
        if self.over.is_some() {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        s.push_str(&match self.phase {
            Phase::Action => format!(
                "Mută J{}: acțiuni {}, cumpărări {}, monede {}. (joc <acțiune> | cumparaturi)\n",
                self.side.idx() + 1,
                self.actions,
                self.buys,
                self.coins
            ),
            Phase::Buy => format!(
                "Mută J{}: monede {}, cumpărări {}. (cumpar <carte> | gata)\n",
                self.side.idx() + 1,
                self.coins,
                self.buys
            ),
            Phase::Gain => "Atelier: iau <carte> (cost până la 4).\n".to_string(),
            Phase::Attacked { who } => {
                format!("J{} aruncă până la 3 cărți: dau <carte>.\n", who + 1)
            }
        });
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Play(c) => format!("joc {}", NAMES[*c as usize]),
            Move::ToBuy => "cumparaturi".into(),
            Move::Buy(c) => format!("cumpar {}", NAMES[*c as usize]),
            Move::End => "gata".into(),
            Move::Gain(c) => format!("iau {}", NAMES[*c as usize]),
            Move::Discard(c) => format!("dau {}", NAMES[*c as usize]),
        }
    }
}

impl Bresla {
    fn vp_pub(&self, p: usize) -> i32 {
        self.vp(p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setup() {
        let g = Bresla::with_seed(1);
        assert_eq!(g.hand[0].len(), 5);
        assert_eq!(g.deck[0].len(), 5);
        assert_eq!(g.vp(0), 3);
    }

    #[test]
    fn coins_buy_and_cleanup() {
        let mut g = Bresla::with_seed(2);
        g.hand[0] = vec![0, 0, 1, 2, 3];
        g.apply(&Move::ToBuy);
        assert_eq!(g.coins, 1 + 1 + 2 + 3);
        assert!(g.legal_moves().contains(&Move::Buy(4))); // moșie cost 5
        assert!(!g.legal_moves().contains(&Move::Buy(5)));
        g.apply(&Move::Buy(4));
        assert_eq!(g.supply[4], 7);
        assert!(g.legal_moves().iter().all(|m| !matches!(m, Move::Buy(_))));
        let total = g.conserved_total();
        g.apply(&Move::End);
        assert_eq!(g.conserved_total(), total);
        assert_eq!(g.to_move(), Player::Two);
        assert_eq!(g.hand[0].len(), 5);
        assert_eq!(g.inplay.len(), 0);
    }

    #[test]
    fn village_and_market_effects() {
        let mut g = Bresla::with_seed(3);
        g.hand[0] = vec![6, 8, 0, 0, 0];
        g.apply(&Move::Play(6));
        assert_eq!(g.actions, 2);
        assert_eq!(g.hand[0].len(), 5);
        g.hand[0].push(8);
        g.apply(&Move::Play(8));
        assert_eq!((g.actions, g.buys, g.coins), (2, 2, 1));
    }

    #[test]
    fn militia_and_workshop() {
        let mut g = Bresla::with_seed(4);
        g.hand[0] = vec![13, 0, 0, 0, 0];
        g.hand[1] = vec![0, 0, 1, 2, 3];
        g.apply(&Move::Play(13));
        assert_eq!(g.to_move(), Player::Two);
        assert_eq!(g.coins, 2);
        g.apply(&Move::Discard(3));
        assert_eq!(g.to_move(), Player::Two);
        g.apply(&Move::Discard(0));
        assert_eq!(g.to_move(), Player::One);
        assert_eq!(g.hand[1].len(), 3);
        g.hand[0].push(12);
        g.actions = 1;
        g.apply(&Move::Play(12));
        let l = g.legal_moves();
        assert!(l.contains(&Move::Gain(1)) && !l.contains(&Move::Gain(4)));
        g.apply(&Move::Gain(1));
        assert_eq!(g.supply[1], 39);
        assert_eq!(*g.discard[0].last().unwrap(), 1);
    }

    #[test]
    fn game_end_on_provinces() {
        let mut g = Bresla::with_seed(5);
        g.supply[5] = 0;
        g.discard[0].push(5);
        g.apply(&Move::ToBuy);
        g.apply(&Move::End);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn random_games_end_and_conserve() {
        for seed in 1..15 {
            let mut g = Bresla::with_seed(seed);
            let t = g.conserved_total();
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                assert_eq!(g.conserved_total(), t);
                n += 1;
                assert!(n < 20000);
            }
        }
    }

    #[test]
    fn ai_beats_random() {
        let mut wins = 0;
        for seed in 1..=20u64 {
            let mut g = Bresla::with_seed(seed);
            let mut rng = Rng::new(seed + 3);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = if g.to_move() == Player::One {
                    polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap()
                } else {
                    let l = g.legal_moves();
                    l[rng.below(l.len())]
                };
                g.apply(&m);
                n += 1;
                assert!(n < 20000);
            }
            if g.outcome() == Some(Outcome::Win(Player::One)) {
                wins += 1;
            }
        }
        assert!(wins >= 18, "wins {wins}");
    }
}

#[cfg(test)]
mod a1_blind {
    use super::*;
    use crate::ai_audit_a1::{audit, scramble_pool, shuffle};

    #[test]
    fn ai_ignores_opponent_hand_and_deck_orders_at_all_levels() {
        audit("bresla", Bresla::with_seed, 1..=4, 4, 120, |g, salt| {
            let foe = 1 - g.side.idx();
            let me = g.side.idx();
            scramble_pool(&mut g.hand[foe], &mut g.deck[foe], salt);
            shuffle(&mut g.deck[me], salt ^ 1);
            g.rng = Rng::new(salt ^ 7);
        });
    }
}
