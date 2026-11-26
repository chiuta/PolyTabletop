//! „Zece mii de puncte” — jocul de zaruri tradițional cunoscut ca Farkle / 10 000, pentru 2 jucători.
//!
//! * Șase zaruri. În tura ta arunci toate zarurile (`arunc`), păstrezi cel puțin un zar sau o
//!   combinație care punctează (`pastrez 1 5 5`), apoi alegi: `arunc` (cu zarurile rămase) sau `banca`
//!   (adaugi punctele turei la total). Dacă o aruncare nu conține nicio combinație care punctează,
//!   e „Farkle”: pierzi punctele turei (mutarea de confirmare `farkle`).
//! * Combinații: un 1 = 100, un 5 = 50; trei zaruri egale = 100 × fața (trei de 1 = 1000); patru
//!   egale = 1000, cinci = 2000, șase = 3000; scara 1-2-3-4-5-6 = 1500; trei perechi = 1500.
//! * Dacă toate cele șase zaruri au punctat („zaruri fierbinți”), arunci din nou toate cele șase,
//!   păstrând punctele turei.
//! * Cât ai 0 puncte în total, poți băncui doar cu cel puțin 500 de puncte în tură.
//! * Cine ajunge la 10 000 sau mai mult declanșează runda finală: adversarul mai are o tură;
//!   apoi câștigă totalul mai mare (egal = remiză).
//!
//! AI-ul nu vede aruncarea viitoare: `evaluate` folosește, pentru stările imediat după o aruncare,
//! o speranță matematică precalculată (programare dinamică pe (puncte tură, zaruri rămase)) și
//! nu citește fețele aruncate.
#![allow(
    clippy::needless_range_loop,
    clippy::collapsible_if,
    clippy::collapsible_match,
    clippy::manual_is_multiple_of,
    clippy::manual_range_patterns,
    clippy::type_complexity
)]

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::sync::OnceLock;

const TARGET: u32 = 10_000;
const OPEN_MIN: u32 = 500;
const T_CAP: u32 = 4000;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Start,
    Keep,
    Decide,
    Farkled,
    Over,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Roll,
    Keep(Vec<u8>),
    Bank,
    Ack,
}

pub type Counts = [u8; 7];

fn face_score(f: usize, c: u8) -> Option<u32> {
    if c == 0 {
        return Some(0);
    }
    let single = |k: u8| -> u32 {
        if f == 1 {
            100 * k as u32
        } else {
            50 * k as u32
        }
    };
    let mut best: Option<u32> = None;
    let mut up = |v: u32| {
        if best.is_none_or(|b| v > b) {
            best = Some(v);
        }
    };
    if f == 1 || f == 5 {
        up(single(c));
    }
    if c >= 3 {
        let base = if f == 1 { 1000 } else { 100 * f as u32 };
        let rest = c - 3;
        if rest == 0 {
            up(base);
        } else if f == 1 || f == 5 {
            up(base + single(rest));
        }
    }
    match c {
        4 => up(1000),
        5 => up(2000),
        6 => up(3000),
        _ => {}
    }
    best
}

/// Punctajul unui set de zaruri păstrate, dacă *toate* zarurile punctează.
pub fn keep_score(c: &Counts) -> Option<u32> {
    let n: u8 = c[1..].iter().sum();
    if n == 0 {
        return None;
    }
    let mut total = Some(0u32);
    for (f, &k) in c.iter().enumerate().skip(1) {
        total = match (total, face_score(f, k)) {
            (Some(a), Some(b)) => Some(a + b),
            _ => None,
        };
    }
    let mut best = total;
    if n == 6 {
        let straight = c[1..].iter().all(|&k| k == 1);
        let pairs = c[1..].iter().filter(|&&k| k == 2).count() == 3;
        if straight || pairs {
            best = Some(best.map_or(1500, |b| b.max(1500)));
        }
    }
    best
}

/// Toate submulțimile nevide care punctează complet, cu punctaj.
pub fn keep_options(c: &Counts) -> Vec<(Counts, u32)> {
    let mut out = Vec::new();
    let mut cur: Counts = [0; 7];
    fn rec(f: usize, c: &Counts, cur: &mut Counts, out: &mut Vec<(Counts, u32)>) {
        if f == 7 {
            if let Some(s) = keep_score(cur) {
                out.push((*cur, s));
            }
            return;
        }
        for k in 0..=c[f] {
            cur[f] = k;
            rec(f + 1, c, cur, out);
        }
        cur[f] = 0;
    }
    rec(1, c, &mut cur, &mut out);
    out
}

pub fn counts_of(d: &[u8]) -> Counts {
    let mut c = [0u8; 7];
    for &x in d {
        c[x as usize] += 1;
    }
    c
}

struct Table {
    /// v[n][t]: valoarea optimă (puncte de tură câștigate) cu `n` zaruri de aruncat și `50*t` puncte.
    v: [Vec<f64>; 7],
    /// r[n][t]: valoarea aruncării a `n` zaruri.
    r: [Vec<f64>; 7],
}

fn table() -> &'static Table {
    static T: OnceLock<Table> = OnceLock::new();
    T.get_or_init(|| {
        let nt = (T_CAP / 50) as usize;
        // Rezultatele posibile pentru n zaruri: (probabilitate, opțiuni (puncte, zaruri păstrate)).
        let mut outcomes: Vec<Vec<(f64, Vec<(u32, u8)>)>> = vec![Vec::new(); 7];
        let fact = [1.0, 1.0, 2.0, 6.0, 24.0, 120.0, 720.0f64];
        for (n, slot) in outcomes.iter_mut().enumerate().skip(1) {
            let mut cnt: Counts = [0; 7];
            fn gen(
                f: usize,
                left: u8,
                n: usize,
                cnt: &mut Counts,
                fact: &[f64; 7],
                out: &mut Vec<(f64, Vec<(u32, u8)>)>,
            ) {
                if f == 6 {
                    cnt[6] = left;
                    let mut p = fact[n];
                    for &k in cnt[1..].iter() {
                        p /= fact[k as usize];
                    }
                    p /= 6f64.powi(n as i32);
                    let opts = keep_options(cnt)
                        .into_iter()
                        .map(|(k, s)| (s, k[1..].iter().sum::<u8>()))
                        .collect();
                    out.push((p, opts));
                    return;
                }
                for k in 0..=left {
                    cnt[f] = k;
                    gen(f + 1, left - k, n, cnt, fact, out);
                }
                cnt[f] = 0;
            }
            gen(1, n as u8, n, &mut cnt, &fact, slot);
        }
        let mut v: [Vec<f64>; 7] = Default::default();
        let mut r: [Vec<f64>; 7] = Default::default();
        for n in 1..=6 {
            v[n] = vec![0.0; nt + 1];
            r[n] = vec![0.0; nt + 1];
        }
        for t in (0..=nt).rev() {
            for n in 1..=6usize {
                let mut e = 0.0;
                for (p, opts) in &outcomes[n] {
                    let mut best = 0.0f64;
                    for &(s, k) in opts {
                        let nt2 = t + (s / 50) as usize;
                        let rem = n as u8 - k;
                        let n2 = if rem == 0 { 6 } else { rem as usize };
                        let val = if nt2 > nt {
                            (50 * nt2) as f64
                        } else {
                            v[n2][nt2]
                        };
                        if val > best {
                            best = val;
                        }
                    }
                    e += p * best;
                }
                r[n][t] = e;
                v[n][t] = e.max((50 * t) as f64);
            }
        }
        Table { v, r }
    })
}

fn val_roll(turn: u32, n: u8) -> f64 {
    let t = (turn / 50) as usize;
    if turn >= T_CAP {
        return turn as f64;
    }
    table().r[n as usize][t]
}

fn val_best(turn: u32, n: u8) -> f64 {
    let t = (turn / 50) as usize;
    if turn >= T_CAP {
        return turn as f64;
    }
    table().v[n as usize][t]
}

#[derive(Clone)]
pub struct Farkle {
    rng: Rng,
    total: [u32; 2],
    side: Player,
    turn: u32,
    dice: Vec<u8>,
    /// Zaruri care se aruncă la următoarea aruncare.
    avail: u8,
    phase: Phase,
    ending: Option<Player>,
    log: String,
    turns: u32,
}

impl Default for Farkle {
    fn default() -> Self {
        Self::new()
    }
}

impl Farkle {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Farkle {
            rng: Rng::new(seed),
            total: [0, 0],
            side: Player::One,
            turn: 0,
            dice: Vec::new(),
            avail: 6,
            phase: Phase::Start,
            ending: None,
            log: String::new(),
            turns: 0,
        }
    }

    /// Schimbă sămânța aruncărilor viitoare (teste: AI-ul nu trebuie să depindă de ea).
    pub fn reseed(&mut self, seed: u64) {
        self.rng = Rng::new(seed);
    }

    pub fn totals(&self) -> (u32, u32) {
        (self.total[0], self.total[1])
    }

    fn can_bank(&self) -> bool {
        self.turn > 0 && (self.total[self.side.idx()] > 0 || self.turn >= OPEN_MIN)
    }

    fn end_turn(&mut self) {
        self.turns += 1;
        self.turn = 0;
        self.dice.clear();
        self.avail = 6;
        let p = self.side;
        if let Some(e) = self.ending {
            if e != p {
                self.phase = Phase::Over;
                return;
            }
        } else if self.total[p.idx()] >= TARGET {
            self.ending = Some(p);
        }
        self.side = p.other();
        self.phase = Phase::Start;
    }

    fn roll(&mut self) {
        let n = self.avail;
        self.dice = (0..n).map(|_| self.rng.d6()).collect();
        self.dice.sort_unstable();
        let c = counts_of(&self.dice);
        self.phase = if keep_options(&c).is_empty() {
            Phase::Farkled
        } else {
            Phase::Keep
        };
    }
}

impl Game for Farkle {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        match self.phase {
            Phase::Over => Vec::new(),
            Phase::Start => vec![Move::Roll],
            Phase::Farkled => vec![Move::Ack],
            Phase::Keep => {
                let c = counts_of(&self.dice);
                let mut v: Vec<Move> = keep_options(&c)
                    .into_iter()
                    .map(|(k, _)| {
                        let mut f = Vec::new();
                        for (face, &n) in k.iter().enumerate().skip(1) {
                            for _ in 0..n {
                                f.push(face as u8);
                            }
                        }
                        Move::Keep(f)
                    })
                    .collect();
                v.sort_by_key(|m| match m {
                    Move::Keep(f) => (f.len(), f.clone()),
                    _ => (0, Vec::new()),
                });
                v
            }
            Phase::Decide => {
                let mut v = vec![Move::Roll];
                if self.can_bank() {
                    v.push(Move::Bank);
                }
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        match m {
            Move::Roll => {
                self.roll();
                self.log = format!(
                    "J{} aruncă {} zaruri: {}",
                    self.side.idx() + 1,
                    self.dice.len(),
                    self.dice
                        .iter()
                        .map(|d| d.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                );
            }
            Move::Keep(f) => {
                let c = counts_of(f);
                let s = keep_score(&c).unwrap_or(0);
                self.turn += s;
                let rem = self.dice.len() as u8 - f.len() as u8;
                self.avail = if rem == 0 { 6 } else { rem };
                self.dice.clear();
                self.phase = Phase::Decide;
                self.log = format!(
                    "J{} păstrează +{}{}",
                    self.side.idx() + 1,
                    s,
                    if rem == 0 {
                        " (zaruri fierbinți!)"
                    } else {
                        ""
                    }
                );
            }
            Move::Bank => {
                self.total[self.side.idx()] += self.turn;
                self.log = format!(
                    "J{} băncuiește {} (total {})",
                    self.side.idx() + 1,
                    self.turn,
                    self.total[self.side.idx()]
                );
                self.end_turn();
            }
            Move::Ack => {
                self.log = format!(
                    "FARKLE! J{} pierde {} puncte de tură",
                    self.side.idx() + 1,
                    self.turn
                );
                self.end_turn();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        Some(match self.total[0].cmp(&self.total[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let q = p.other();
        let diff = self.total[p.idx()] as f64 - self.total[q.idx()] as f64;
        let mut v = diff;
        if self.side == p {
            match self.phase {
                // după o aruncare: zarurile nu se citesc
                Phase::Keep | Phase::Farkled => {
                    v += val_roll(self.turn, self.dice.len() as u8);
                }
                Phase::Decide => {
                    v += val_best(self.turn, self.avail);
                }
                _ => {}
            }
        }
        v as i32
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Zece mii de puncte (Farkle) — total: J1 {}, J2 {} (țintă {}){}\n",
            self.total[0],
            self.total[1],
            TARGET,
            if self.ending.is_some() {
                " | RUNDA FINALĂ"
            } else {
                ""
            }
        );
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        if self.phase == Phase::Over {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        s.push_str(&format!(
            "Tura lui J{}: {} puncte în tură, {} zaruri de aruncat\n",
            self.side.idx() + 1,
            self.turn,
            self.avail
        ));
        if matches!(self.phase, Phase::Keep | Phase::Farkled) {
            s.push_str(&format!(
                "Zaruri: {}\n",
                self.dice
                    .iter()
                    .map(|d| d.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            ));
        }
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Roll => "arunc".into(),
            Move::Bank => "banca".into(),
            Move::Ack => "farkle".into(),
            Move::Keep(f) => format!(
                "pastrez {}",
                f.iter()
                    .map(|d| d.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sc(d: &[u8]) -> Option<u32> {
        keep_score(&counts_of(d))
    }

    #[test]
    fn scoring_table() {
        assert_eq!(sc(&[1]), Some(100));
        assert_eq!(sc(&[5]), Some(50));
        assert_eq!(sc(&[1, 1, 1]), Some(1000));
        assert_eq!(sc(&[2, 2, 2]), Some(200));
        assert_eq!(sc(&[6, 6, 6]), Some(600));
        assert_eq!(sc(&[3, 3, 3, 3]), Some(1000));
        assert_eq!(sc(&[4, 4, 4, 4, 4]), Some(2000));
        assert_eq!(sc(&[2, 2, 2, 2, 2, 2]), Some(3000));
        assert_eq!(sc(&[1, 2, 3, 4, 5, 6]), Some(1500));
        assert_eq!(sc(&[2, 2, 3, 3, 6, 6]), Some(1500));
        assert_eq!(sc(&[1, 1, 1, 1]), Some(1100));
        assert_eq!(sc(&[1, 5]), Some(150));
        assert_eq!(sc(&[2]), None);
        assert_eq!(sc(&[1, 2]), None);
        assert_eq!(sc(&[2, 2]), None);
    }

    #[test]
    fn keep_options_only_scoring() {
        let c = counts_of(&[1, 1, 2, 3, 5, 6]);
        let o = keep_options(&c);
        assert!(!o.is_empty());
        for (k, s) in &o {
            assert_eq!(keep_score(k), Some(*s));
        }
        assert!(keep_options(&counts_of(&[2, 3, 4, 6, 6, 2])).is_empty());
    }

    #[test]
    fn farkle_loses_turn_points() {
        let mut g = Farkle::with_seed(1);
        g.turn = 700;
        g.dice = vec![2, 3, 4, 6, 6, 2];
        g.phase = Phase::Farkled;
        assert_eq!(g.legal_moves(), vec![Move::Ack]);
        g.apply(&Move::Ack);
        assert_eq!(g.total, [0, 0]);
        assert_eq!(g.side, Player::Two);
        assert_eq!(g.turn, 0);
    }

    #[test]
    fn hot_dice_and_opening_threshold() {
        let mut g = Farkle::with_seed(2);
        g.dice = vec![1, 1, 1, 5, 5, 5];
        g.phase = Phase::Keep;
        g.apply(&Move::Keep(vec![1, 1, 1, 5, 5, 5]));
        assert_eq!(g.turn, 1500);
        assert_eq!(g.avail, 6);
        assert!(g.legal_moves().contains(&Move::Bank));
        let mut h = Farkle::with_seed(3);
        h.dice = vec![1, 2, 3, 4, 6, 6];
        h.phase = Phase::Keep;
        h.apply(&Move::Keep(vec![1]));
        assert_eq!(h.turn, 100);
        assert!(!h.legal_moves().contains(&Move::Bank), "sub 500 cu total 0");
    }

    #[test]
    fn final_round_gives_opponent_one_turn() {
        let mut g = Farkle::with_seed(4);
        g.total = [9900, 4000];
        g.turn = 200;
        g.phase = Phase::Decide;
        g.apply(&Move::Bank);
        assert_eq!(g.ending, Some(Player::One));
        assert_eq!(g.phase, Phase::Start);
        assert_eq!(g.side, Player::Two);
        g.turn = 100;
        g.phase = Phase::Decide;
        g.apply(&Move::Bank);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn dp_table_is_sane() {
        // cu 6 zaruri și 0 puncte, aruncarea valorează în jur de 400-600
        let r = val_roll(0, 6);
        assert!(r > 300.0 && r < 700.0, "{r}");
        // la 3000 de puncte cu un singur zar nu merită să arunci
        assert!(val_best(3000, 1) <= 3000.0 + 1.0);
        assert!(val_roll(3000, 1) < 3000.0);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=6u64 {
            let mut g = Farkle::with_seed(seed);
            let mut rng = Rng::new(seed + 5);
            let mut n = 0;
            while g.outcome().is_none() && n < 20_000 {
                let mv = g.legal_moves();
                let m = mv[rng.below(mv.len())].clone();
                g.apply(&m);
                n += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed}");
        }
    }
}
