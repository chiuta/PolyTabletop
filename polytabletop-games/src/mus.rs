#![allow(clippy::doc_lazy_continuation)]
//! Mus (Spania), joc de cărți cu baraja spaniolă de 40, adaptat pentru doi jucători (originalul se joacă în
//! perechi, 4 jucători), pe baza regulilor publice.
//!
//! * 4 cărți fiecare; regele și 3 sunt „regi” (valoare 12), 2 e „as” (1). Valorile pentru punctaj: figurile 10, restul numeric.
//! * Mâna se rotește. **Mus**: mano și apoi postre pot spune `mus` sau `corto`; dacă ambii spun `mus`, fiecare aruncă
//! cel puțin o carte (`descarta 0101`, 1 = aruncă cartea din poziția respectivă) și primește altele din pachet
//! (într-un singur tur; în joc se pot repeta turele). Apoi urmează patru **lance**:
//!   grande (cărțile mari, comparate în ordine), chica (cele mici), pares (pereche 1 p, medias 2 p, duples 3 p),
//!   juego (31 = 3 p, alt total ≥ 31 = 2 p; dacă nimeni n-are, se joacă la „punto”, cel mai mare total).
//! * În fiecare lance: `paso` sau `envido` (miza devine 2), celălalt răspunde `quiero` / `no quiero`.
//!   Dacă a spus „paso” fiecare, miza e 1. `no quiero` dă 1 punct celui care a pariat. (Simplificări: un singur envite de 2,
//!   fără „órdago” și fără re-envite.) La pares și juego, un `declara` forțat face publică existența combinației. Dacă doar
//!   unul o are, își ia punctele-bonus fără pariu.
//! * La final (`dezvaluie`) câștigă fiecare lance cel mai bun (egalitate: mano); lua miza + bonusul propriu (la pares/juego).
//! * Se joacă la 20 de puncte (cel mult 30 de mâini, apoi cine are mai mult).
//! Informație ascunsă: AI-ul vede doar mâna lui; amestecul, repartizarea și dezvăluirea sunt pași forțați.

use crate::baraja_es as be;
use crate::cards::{handover_banner, Viewer};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const TARGET: i32 = 20;
const MAX_HANDS: u32 = 30;

fn label(c: u8) -> u8 {
    match [1, 2, 3, 4, 5, 6, 7, 10, 11, 12][be::pos(c) as usize] {
        2 => 1,
        3 => 12,
        n => n,
    }
}
fn g(c: u8) -> u8 {
    match label(c) {
        12 => 8,
        11 => 7,
        10 => 6,
        7 => 5,
        6 => 4,
        5 => 3,
        4 => 2,
        _ => 1,
    }
}
fn val(c: u8) -> u32 {
    let n = label(c);
    if n >= 10 {
        10
    } else {
        n as u32
    }
}
fn total(h: &[u8]) -> u32 {
    h.iter().map(|&c| val(c)).sum()
}
fn big_key(h: &[u8]) -> Vec<u8> {
    let mut v: Vec<u8> = h.iter().map(|&c| g(c)).collect();
    v.sort_unstable_by(|a, b| b.cmp(a));
    v
}
fn small_key(h: &[u8]) -> Vec<u8> {
    let mut v: Vec<u8> = h.iter().map(|&c| 9 - g(c)).collect();
    v.sort_unstable_by(|a, b| b.cmp(a));
    v
}
/// (tip 0..3, rang mare, rang mic)
fn pairs(h: &[u8]) -> (u8, u8, u8) {
    let mut cnt = [0u8; 9];
    for &c in h {
        cnt[g(c) as usize] += 1;
    }
    let mut pr: Vec<u8> = Vec::new();
    let mut tri = 0;
    for r in (1..9).rev() {
        match cnt[r] {
            4 => return (3, r as u8, r as u8),
            3 => tri = r as u8,
            2 => pr.push(r as u8),
            _ => {}
        }
    }
    if pr.len() == 2 {
        (3, pr[0], pr[1])
    } else if tri > 0 {
        (2, tri, 0)
    } else if pr.len() == 1 {
        (1, pr[0], 0)
    } else {
        (0, 0, 0)
    }
}
fn game_rank(t: u32) -> u32 {
    match t {
        31 => 8,
        32 => 7,
        40 => 6,
        37 => 5,
        36 => 4,
        35 => 3,
        34 => 2,
        33 => 1,
        _ => 0,
    }
}
fn has_game(h: &[u8]) -> bool {
    total(h) >= 31
}
fn game_bonus(h: &[u8]) -> i32 {
    if total(h) == 31 {
        3
    } else {
        2
    }
}
fn pair_bonus(h: &[u8]) -> i32 {
    pairs(h).0 as i32
}

const NAMES: [&str; 4] = ["grande", "chica", "pares", "juego"];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Mus,
    Corto,
    Discard(u8),
    Deal,
    Declare,
    Pass,
    Envido,
    Quiero,
    NoQuiero,
    Reveal,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Mus,
    Discard,
    Deal,
    Declare,
    Bet,
    Reveal,
    Over,
}

#[derive(Clone)]
pub struct Mus {
    hands: [Vec<u8>; 2],
    stock: Vec<u8>,
    mano: Player,
    phase: Phase,
    side: Player,
    mus_said: bool,
    masks: [u8; 2],
    discards_done: u8,
    lance: usize,
    bet: u8,
    bettor: Player,
    passes: u8,
    stakes: [i32; 4],
    fixed: [Option<(Player, i32)>; 4],
    decl: [bool; 2],
    score: [i32; 2],
    hand_no: u32,
    rng: Rng,
    viewer: Viewer,
    log: String,
}

impl Default for Mus {
    fn default() -> Self {
        Self::new()
    }
}

fn win_prob(h: &[u8], l: usize) -> f64 {
    match l {
        0 | 1 => {
            let k = if l == 0 { big_key(h) } else { small_key(h) };
            let s = (k[0] as f64 * 4.0 + k[1] as f64 * 2.0 + k[2] as f64) / 7.0;
            (0.5 + (s - 5.0) * 0.15).clamp(0.08, 0.92)
        }
        2 => [0.25, 0.6, 0.8, 0.93][pairs(h).0 as usize],
        _ => {
            let t = total(h);
            if t >= 31 {
                [0.4, 0.45, 0.5, 0.55, 0.6, 0.75, 0.85, 0.95][..]
                    .get(match t {
                        33 => 0,
                        34 => 1,
                        35 => 2,
                        36 => 3,
                        37 => 4,
                        40 => 5,
                        32 => 6,
                        _ => 7,
                    })
                    .copied()
                    .unwrap_or(0.5)
            } else {
                0.1 + 0.3 * (t.saturating_sub(4)) as f64 / 26.0
            }
        }
    }
}

impl Mus {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Mus {
            hands: [Vec::new(), Vec::new()],
            stock: Vec::new(),
            mano: Player::One,
            phase: Phase::Mus,
            side: Player::One,
            mus_said: false,
            masks: [0; 2],
            discards_done: 0,
            lance: 0,
            bet: 0,
            bettor: Player::One,
            passes: 0,
            stakes: [1; 4],
            fixed: [None; 4],
            decl: [false; 2],
            score: [0; 2],
            hand_no: 0,
            rng: Rng::new(seed),
            viewer: Viewer::Fixed(Player::One),
            log: String::new(),
        };
        g.deal();
        g
    }

    fn deal(&mut self) {
        let mut d = be::deck(&mut self.rng);
        self.hands = [d.split_off(36), d.split_off(32)];
        self.hands[0].sort_unstable();
        self.hands[1].sort_unstable();
        self.stock = d;
        self.mano = if self.hand_no.is_multiple_of(2) {
            Player::One
        } else {
            Player::Two
        };
        self.phase = Phase::Mus;
        self.side = self.mano;
        self.mus_said = false;
        self.masks = [0; 2];
        self.discards_done = 0;
        self.lance = 0;
        self.bet = 0;
        self.passes = 0;
        self.stakes = [1; 4];
        self.fixed = [None; 4];
    }

    fn start_lances(&mut self) {
        self.lance = 0;
        self.begin_lance();
    }

    fn begin_lance(&mut self) {
        self.bet = 0;
        self.passes = 0;
        if self.lance >= 4 {
            self.phase = Phase::Reveal;
            self.side = Player::Two;
            return;
        }
        if self.lance >= 2 {
            self.phase = Phase::Declare;
            self.side = Player::Two;
        } else {
            self.phase = Phase::Bet;
            self.side = self.mano;
        }
    }

    fn next_lance(&mut self) {
        self.lance += 1;
        self.begin_lance();
    }

    fn winner(&self, l: usize) -> Player {
        let (a, b) = (&self.hands[0], &self.hands[1]);
        let o = match l {
            0 => big_key(a).cmp(&big_key(b)),
            1 => small_key(a).cmp(&small_key(b)).reverse(),
            2 => pairs(a).cmp(&pairs(b)),
            _ => {
                if has_game(a) || has_game(b) {
                    game_rank(total(a)).cmp(&game_rank(total(b)))
                } else {
                    total(a).cmp(&total(b))
                }
            }
        };
        match o {
            std::cmp::Ordering::Greater => Player::One,
            std::cmp::Ordering::Less => Player::Two,
            std::cmp::Ordering::Equal => self.mano,
        }
    }

    fn reveal(&mut self) {
        let mut add = [0i32; 2];
        let mut txt = format!(
            "Mâini: J1 {} | J2 {}. ",
            be::join(&self.hands[0]),
            be::join(&self.hands[1])
        );
        for l in 0..4 {
            if let Some((p, n)) = self.fixed[l] {
                add[p.idx()] += n;
                continue;
            }
            let (ha, hb) = (&self.hands[0], &self.hands[1]);
            let (has_a, has_b) = match l {
                2 => (pairs(ha).0 > 0, pairs(hb).0 > 0),
                3 => (has_game(ha), has_game(hb)),
                _ => (true, true),
            };
            let bonus = |p: usize| -> i32 {
                let h = &self.hands[p];
                match l {
                    2 => pair_bonus(h),
                    3 if has_game(h) => game_bonus(h),
                    _ => 0,
                }
            };
            if l == 2 && !has_a && !has_b {
                continue;
            }
            if l == 2 && (has_a ^ has_b) || l == 3 && (has_a ^ has_b) {
                let p = usize::from(has_b);
                add[p] += bonus(p);
                continue;
            }
            let w = self.winner(l);
            add[w.idx()] += self.stakes[l] + bonus(w.idx());
        }
        self.score[0] += add[0];
        self.score[1] += add[1];
        txt.push_str(&format!("Puncte în mână: J1 +{}, J2 +{}.", add[0], add[1]));
        self.log = txt;
        self.hand_no += 1;
        if self.score.iter().any(|&s| s >= TARGET) || self.hand_no >= MAX_HANDS {
            self.phase = Phase::Over;
        } else {
            self.deal();
        }
    }

    fn lance_value(&self, p: Player) -> f64 {
        // valoare estimată (în puncte) pentru p, folosind doar mâna lui p
        let h = &self.hands[p.idx()];
        let mut v = 0.0;
        for l in 0..4 {
            if let Some((q, n)) = self.fixed[l] {
                v += if q == p { n as f64 } else { -(n as f64) };
                continue;
            }
            let w = win_prob(h, l);
            let edge = 2.0 * w - 1.0;
            let cur = self.phase == Phase::Bet && l == self.lance;
            let later = l > self.lance && !matches!(self.phase, Phase::Reveal | Phase::Over);
            if cur {
                if self.bet > 0 {
                    if self.bettor == p {
                        v += 0.35 + 0.65 * 2.0 * edge;
                    } else {
                        v += (2.0 * edge).max(-1.0);
                    }
                } else {
                    v += edge * if self.passes == 1 { 1.0 } else { 1.2 };
                }
            } else if later || self.phase <= Phase::Declare {
                v += edge * 1.1;
            } else {
                v += edge * self.stakes[l] as f64;
            }
            if l >= 2 {
                v += (w - 0.3) * 1.3;
            }
        }
        v
    }
}

impl PartialOrd for Phase {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some((*self as u8).cmp(&(*other as u8)))
    }
}

impl Game for Mus {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        match self.phase {
            Phase::Over => Vec::new(),
            Phase::Mus => vec![Move::Mus, Move::Corto],
            Phase::Discard => (1..16).map(Move::Discard).collect(),
            Phase::Deal => vec![Move::Deal],
            Phase::Declare => vec![Move::Declare],
            Phase::Reveal => vec![Move::Reveal],
            Phase::Bet => {
                if self.bet > 0 {
                    vec![Move::Quiero, Move::NoQuiero]
                } else {
                    vec![Move::Pass, Move::Envido]
                }
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        match *m {
            Move::Mus => {
                if self.mus_said {
                    self.phase = Phase::Discard;
                    self.side = self.mano;
                } else {
                    self.mus_said = true;
                    self.side = self.side.other();
                }
            }
            Move::Corto => self.start_lances(),
            Move::Discard(mask) => {
                self.masks[self.side.idx()] = mask;
                self.discards_done += 1;
                if self.discards_done == 2 {
                    self.phase = Phase::Deal;
                    self.side = Player::Two;
                } else {
                    self.side = self.side.other();
                }
            }
            Move::Deal => {
                for p in [self.mano, self.mano.other()] {
                    let mask = self.masks[p.idx()];
                    let mut kept: Vec<u8> = Vec::new();
                    let mut n = 0;
                    for (i, &c) in self.hands[p.idx()].iter().enumerate() {
                        if mask >> i & 1 == 1 {
                            n += 1;
                        } else {
                            kept.push(c);
                        }
                    }
                    for _ in 0..n {
                        kept.push(self.stock.pop().unwrap());
                    }
                    kept.sort_unstable();
                    self.hands[p.idx()] = kept;
                }
                self.start_lances();
            }
            Move::Declare => {
                // pe pares/juego: dacă nu e nimic de pariat, lancea se sare
                let (a, b) = (&self.hands[0], &self.hands[1]);
                let (x, y) = if self.lance == 2 {
                    (pairs(a).0 > 0, pairs(b).0 > 0)
                } else {
                    (has_game(a), has_game(b))
                };
                self.decl = [x, y];
                let both = x && y;
                let none = !x && !y;
                if both || (self.lance == 3 && none) {
                    self.phase = Phase::Bet;
                    self.side = self.mano;
                } else {
                    self.stakes[self.lance] = 0;
                    self.next_lance();
                }
            }
            Move::Pass => {
                if self.passes == 1 {
                    self.next_lance();
                } else {
                    self.passes = 1;
                    self.side = self.side.other();
                }
            }
            Move::Envido => {
                self.bet = 2;
                self.bettor = self.side;
                self.side = self.side.other();
            }
            Move::Quiero => {
                self.stakes[self.lance] = 2;
                self.next_lance();
            }
            Move::NoQuiero => {
                self.fixed[self.lance] = Some((self.bettor, 1));
                self.next_lance();
            }
            Move::Reveal => self.reveal(),
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        Some(match self.score[0].cmp(&self.score[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let base = (self.score[p.idx()] - self.score[p.other().idx()]) * 100;
        match self.phase {
            Phase::Discard => {
                // cărțile păstrate + media celor aruncate (nu se vede pachetul)
                let mask = self.masks[p.idx()];
                let h = &self.hands[p.idx()];
                let mut s = 0.0;
                for (i, &c) in h.iter().enumerate() {
                    s += if mask >> i & 1 == 1 { 4.3 } else { g(c) as f64 };
                }
                let mut pr = 0.0;
                let mut cnt = [0u8; 9];
                for (i, &c) in h.iter().enumerate() {
                    if mask >> i & 1 == 0 {
                        cnt[g(c) as usize] += 1;
                    }
                }
                for &n in &cnt {
                    if n >= 2 {
                        pr += 0.8;
                    }
                }
                base + (s * 10.0 + pr * 10.0) as i32
            }
            Phase::Mus => base + (self.lance_value(p) * 10.0) as i32,
            _ => base + (self.lance_value(p) * 40.0) as i32,
        }
    }

    fn render(&self) -> String {
        let me = self.viewer.who(self.to_move());
        let mut s = format!(
            "Mus — mâna {}/{MAX_HANDS}. Puncte: J1 {}, J2 {} (la {TARGET}). Mano: J{}.\n",
            (self.hand_no + 1).min(MAX_HANDS),
            self.score[0],
            self.score[1],
            self.mano.idx() + 1
        );
        if let Viewer::HotSeat = self.viewer {
            s.push_str(&handover_banner(self.side));
        }
        for p in 0..2 {
            if p == me.idx() {
                s.push_str(&format!(
                    "J{} (mâna ta): {}\n",
                    p + 1,
                    be::join(&self.hands[p])
                ));
            } else {
                s.push_str(&format!("J{}: {} cărți\n", p + 1, self.hands[p].len()));
            }
        }
        match self.phase {
            Phase::Mus => s.push_str("Mus sau corto? (`mus` / `corto`)\n"),
            Phase::Discard => s.push_str(&format!(
                "J{} aruncă: `descarta 0101` (1 = aruncă cartea din acea poziție).\n",
                self.side.idx() + 1
            )),
            Phase::Deal => s.push_str("Se repartizează cărțile noi (`reparte`).\n"),
            Phase::Declare => s.push_str(&format!(
                "Lance {}: declarație (`declara`).\n",
                NAMES[self.lance]
            )),
            Phase::Bet => {
                s.push_str(&format!("Lance: {}. ", NAMES[self.lance]));
                if self.lance >= 2 {
                    s.push_str(&format!(
                        "Declarat: J1 {}, J2 {}. ",
                        if self.decl[0] { "da" } else { "nu" },
                        if self.decl[1] { "da" } else { "nu" }
                    ));
                }
                s.push_str(&format!("La rând J{}: ", self.side.idx() + 1));
                s.push_str(if self.bet > 0 {
                    "`quiero` / `no quiero` (mize 2).\n"
                } else {
                    "`paso` / `envido`.\n"
                });
            }
            Phase::Reveal => s.push_str("Dezvăluire (`dezvaluie`).\n"),
            Phase::Over => {}
        }
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Mus => "mus".into(),
            Move::Corto => "corto".into(),
            Move::Discard(k) => format!(
                "descarta {}{}{}{}",
                k & 1,
                k >> 1 & 1,
                k >> 2 & 1,
                k >> 3 & 1
            ),
            Move::Deal => "reparte".into(),
            Move::Declare => "declara".into(),
            Move::Pass => "paso".into(),
            Move::Envido => "envido".into(),
            Move::Quiero => "quiero".into(),
            Move::NoQuiero => "no quiero".into(),
            Move::Reveal => "dezvaluie".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = match who {
            Some(p) => Viewer::Fixed(p),
            None => Viewer::HotSeat,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(n: u8, s: u8) -> u8 {
        s * 10
            + [1, 2, 3, 4, 5, 6, 7, 10, 11, 12]
                .iter()
                .position(|&x| x == n)
                .unwrap() as u8
    }

    #[test]
    fn kings_and_pairs() {
        // 3 e rege, 2 e as
        assert_eq!(g(c(3, 0)), g(c(12, 1)));
        assert_eq!(g(c(2, 0)), g(c(1, 2)));
        assert_eq!(pairs(&[c(3, 0), c(12, 1), c(5, 0), c(6, 0)]).0, 1);
        assert_eq!(pairs(&[c(3, 0), c(12, 1), c(12, 2), c(6, 0)]).0, 2);
        assert_eq!(pairs(&[c(3, 0), c(12, 1), c(5, 0), c(5, 1)]).0, 3);
        assert_eq!(total(&[c(12, 0), c(11, 0), c(10, 0), c(1, 1)]), 31);
        assert!(game_rank(31) > game_rank(32) && game_rank(32) > game_rank(40));
    }

    #[test]
    fn full_round_scores() {
        let mut g = Mus::with_seed(3);
        g.hands[0] = vec![c(12, 0), c(12, 1), c(11, 0), c(1, 1)];
        g.hands[1] = vec![c(4, 0), c(5, 1), c(6, 2), c(7, 3)];
        g.apply(&Move::Corto);
        assert_eq!(g.phase, Phase::Bet);
        // grande: J1 are mai mult; envido + quiero => 2 puncte
        g.apply(&Move::Envido);
        g.apply(&Move::Quiero);
        g.apply(&Move::Pass);
        g.apply(&Move::Pass); // chica
        g.apply(&Move::Declare); // pares: doar J1 are pereche
        g.apply(&Move::Declare); // juego: J1 are 31
        assert_eq!(g.phase, Phase::Reveal);
        g.apply(&Move::Reveal);
        assert!(g.score[0] >= 5, "{:?}", g.score);
    }

    #[test]
    fn no_quiero_gives_one() {
        let mut g = Mus::with_seed(4);
        g.apply(&Move::Corto);
        g.apply(&Move::Envido);
        g.apply(&Move::NoQuiero);
        assert_eq!(g.fixed[0], Some((g.mano, 1)));
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=15u64 {
            let mut g = Mus::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
                assert!(n < 3500, "prea lung");
            }
        }
    }

    #[test]
    fn hidden_and_ai() {
        let mut g = Mus::with_seed(5);
        g.set_viewer(Some(Player::One));
        let a = g.render();
        g.set_viewer(Some(Player::Two));
        assert_ne!(a, g.render());
        let m = polytabletop_core::GameObj::ai_move(&g, 2).unwrap();
        assert!(g.legal_moves().iter().any(|x| g.format_move(x) == m));
    }
}
