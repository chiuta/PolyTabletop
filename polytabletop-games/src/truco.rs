//! Truco (Argentina / Uruguay), 2 jucători, din regulile publice.
//!
//! * Pachet spaniol de 40 de cărți (1–7, 10, 11, 12; culorile: spadă `e`, bastos `b`, aur `o`,
//!   cupă `c`). 3 cărți fiecăruia; se joacă până la 15 puncte. „Mano” (non-dealerul) conduce.
//! * Ierarhia cărților: 1e, 1b, 7e, 7o, toți 3, toți 2, 1o și 1c, 12, 11, 10, 7b și 7c, 6, 5, 4.
//! * Mâna se joacă în 3 runde (levate); cine câștigă 2 ia mâna. Egalitate („parda”): câștigă cine a
//!   luat prima rundă; dacă prima a fost egală, cine a luat a doua etc.; dacă toate sunt egale,
//!   câștigă mano. Cine nu vrea să continue spune `mazo` (se retrage; adversarul ia miza).
//! * **Truco** → `truco` (2 puncte), `retruco` (3), `vale4` (4): cel provocat spune `quiero` (acceptă,
//!   poate el însuși urca) sau `noquiero` (adversarul ia miza dinainte; implicit 1 punct), sau urcă.
//! * **Envido** (doar în prima rundă, înainte de orice truco, înainte să joci prima carte): `envido`
//!   (2), `real` (3), `falta` (punctele rămase liderului până la 15); se poate urca. Tantos: două
//!   cărți de aceeași culoare = 20 + valorile lor (figurile 10/11/12 valorează 0); altfel cea mai
//!   mare carte. Câștigă cel mai mare tanto (egal: mano). Pentru `noquiero` caller-ul ia miza de
//!   dinainte ultimei urcări (minim 1). `cuenta` arată tantos-urile după un `quiero`.
//! * Nu avem flor. Simplificare: după un truco nu se mai poate cânta envido.
//!
//! Mutări: `7e` (carte), `mazo`, `truco`/`retruco`/`vale4`, `envido`/`real`/`falta`, `quiero`,
//! `noquiero`, `cuenta`, `reparte`. AI-ul citește doar propria mână și informațiile publice.

use crate::cards::Viewer;
use crate::cards::{join_cards, shuffle, Card};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const TARGET: i32 = 15;
const STAKES: [i32; 4] = [1, 2, 3, 4];
const SUITS: [char; 4] = ['e', 'b', 'o', 'c'];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Deal,
    Play,
    Over,
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Pending {
    None,
    Envido { caller: Player },
    EnvidoCount,
    Truco { caller: Player },
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Deal,
    Play(Card),
    Mazo,
    /// 1 = truco, 2 = retruco, 3 = vale cuatro
    Truco(u8),
    /// 0 = envido, 1 = real envido, 2 = falta envido
    Envido(u8),
    Quiero,
    NoQuiero,
    Count,
}

#[derive(Clone)]
pub struct Truco {
    rng: Rng,
    viewer: Viewer,
    hands: [Vec<Card>; 2],
    orig: [Vec<Card>; 2],
    table: Vec<[Option<Card>; 2]>,
    results: Vec<Option<Player>>,
    dealer: Player,
    turn: Player,
    side: Player,
    phase: Phase,
    level: usize,
    req: usize,
    t_owner: Option<Player>,
    pending: Pending,
    env_chain: Vec<u8>,
    env_done: bool,
    scores: [i32; 2],
    log: String,
}

impl Default for Truco {
    fn default() -> Self {
        Self::new()
    }
}

pub fn card_text(c: Card) -> String {
    format!("{}{}", c.rank, SUITS[c.suit as usize])
}

pub fn power(c: Card) -> u8 {
    match (c.rank, c.suit) {
        (1, 0) => 14,
        (1, 1) => 13,
        (7, 0) => 12,
        (7, 2) => 11,
        (3, _) => 10,
        (2, _) => 9,
        (1, _) => 8,
        (12, _) => 7,
        (11, _) => 6,
        (10, _) => 5,
        (7, _) => 4,
        (6, _) => 3,
        (5, _) => 2,
        _ => 1,
    }
}

fn env_val(c: Card) -> i32 {
    if c.rank >= 10 {
        0
    } else {
        c.rank as i32
    }
}

pub fn tanto(h: &[Card]) -> i32 {
    let mut best = h.iter().map(|&c| env_val(c)).max().unwrap_or(0);
    for i in 0..h.len() {
        for j in i + 1..h.len() {
            if h[i].suit == h[j].suit {
                best = best.max(20 + env_val(h[i]) + env_val(h[j]));
            }
        }
    }
    best
}

fn deck40() -> Vec<Card> {
    let mut v = Vec::new();
    for suit in 0..4u8 {
        for rank in [1u8, 2, 3, 4, 5, 6, 7, 10, 11, 12] {
            v.push(Card { rank, suit });
        }
    }
    v
}

fn env_next_ok(chain: &[u8], kind: u8) -> bool {
    match chain.last() {
        None => true,
        Some(&0) => {
            kind == 2 || kind == 1 || (kind == 0 && chain.iter().filter(|&&k| k == 0).count() < 2)
        }
        Some(&1) => kind == 2,
        _ => false,
    }
}

impl Truco {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Truco {
            rng: Rng::new(seed),
            viewer: Viewer::Fixed(Player::One),
            hands: [Vec::new(), Vec::new()],
            orig: [Vec::new(), Vec::new()],
            table: Vec::new(),
            results: Vec::new(),
            dealer: Player::Two,
            turn: Player::One,
            side: Player::One,
            phase: Phase::Deal,
            level: 0,
            req: 0,
            t_owner: None,
            pending: Pending::None,
            env_chain: Vec::new(),
            env_done: false,
            scores: [0, 0],
            log: String::new(),
        };
        g.deal();
        g
    }

    pub fn score(&self) -> (i32, i32) {
        (self.scores[0], self.scores[1])
    }

    fn mano(&self) -> Player {
        self.dealer.other()
    }

    fn deal(&mut self) {
        let mut d = deck40();
        shuffle(&mut d, &mut self.rng);
        for p in 0..2 {
            self.hands[p] = d.split_off(d.len() - 3);
            self.hands[p].sort_by_key(|&c| std::cmp::Reverse(power(c)));
            self.orig[p] = self.hands[p].clone();
        }
        self.table.clear();
        self.results.clear();
        self.level = 0;
        self.req = 0;
        self.t_owner = None;
        self.pending = Pending::None;
        self.env_chain.clear();
        self.env_done = false;
        self.phase = Phase::Play;
        self.turn = self.mano();
        self.side = self.turn;
    }

    fn falta_value(&self) -> i32 {
        (TARGET - self.scores[0].max(self.scores[1])).max(1)
    }

    fn env_stake(&self, chain: &[u8]) -> i32 {
        let mut s = 0;
        for &k in chain {
            match k {
                0 => s += 2,
                1 => s += 3,
                _ => return self.falta_value(),
            }
        }
        s
    }

    fn give(&mut self, p: Player, pts: i32) {
        self.scores[p.idx()] += pts;
        if self.scores[p.idx()] >= TARGET {
            self.phase = Phase::Over;
        }
    }

    fn end_hand(&mut self, winner: Player) {
        let pts = STAKES[self.level];
        self.log = format!("{} J{} ia mâna: +{}", self.log, winner.idx() + 1, pts);
        self.give(winner, pts);
        self.next_hand();
    }

    fn next_hand(&mut self) {
        if self.phase == Phase::Over {
            return;
        }
        self.dealer = self.dealer.other();
        self.phase = Phase::Deal;
        self.side = self.dealer.other();
    }

    fn hand_winner(&self) -> Option<Player> {
        let r = &self.results;
        match r.len() {
            2 => match (r[0], r[1]) {
                (Some(a), Some(b)) if a == b => Some(a),
                (Some(a), None) => Some(a),
                (None, Some(b)) => Some(b),
                _ => None,
            },
            3 => match r[2] {
                Some(w) => Some(w),
                None => Some(r[0].or(r[1]).unwrap_or(self.mano())),
            },
            _ => None,
        }
    }

    fn can_call_envido(&self, p: Player) -> bool {
        self.pending == Pending::None
            && p == self.turn
            && self.table.len() <= 1
            && self.results.is_empty()
            && !self.env_done
            && self.level == 0
            && self.hands[p.idx()].len() == 3
    }

    /// Cât de probabil e ca p să câștige mâna (informație doar a lui p).
    fn win_prob(&self, p: Player) -> f32 {
        let q = p.other();
        let mine = &self.hands[p.idx()];
        let mut pool: Vec<u8> = mine.iter().map(|&c| power(c)).collect();
        let mut extra = 0.0;
        if let Some(row) = self.table.last() {
            if self.results.len() + 1 == self.table.len() {
                // rundă în curs: o singură carte pusă
                if let Some(c) = row[p.idx()] {
                    pool.push(power(c));
                    extra += 0.15 * (power(c) as f32 / 14.0 - 0.4);
                } else if let Some(c) = row[q.idx()] {
                    // adversarul a condus; încă nu am răspuns
                    extra -= 0.1 * (power(c) as f32 / 14.0 - 0.4);
                }
            }
        }
        pool.sort_unstable_by(|a, b| b.cmp(a));
        let top: Vec<f32> = pool.iter().take(2).map(|&x| x as f32).collect();
        let avg = if top.is_empty() {
            4.0
        } else {
            top.iter().sum::<f32>() / top.len() as f32
        };
        let wm = self.results.iter().filter(|&&r| r == Some(p)).count() as f32;
        let wo = self.results.iter().filter(|&&r| r == Some(q)).count() as f32;
        (0.5 + (avg - 8.0) / 12.0 + 0.25 * (wm - wo) + extra).clamp(0.03, 0.97)
    }

    fn env_win_prob(&self, p: Player) -> f32 {
        let t = tanto(&self.orig[p.idx()]) as f32;
        ((t - 12.0) / 22.0).clamp(0.02, 0.98)
    }
}

impl Game for Truco {
    type Move = Move;

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = match who {
            Some(p) => Viewer::Fixed(p),
            None => Viewer::HotSeat,
        };
    }

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        match self.phase {
            Phase::Over => Vec::new(),
            Phase::Deal => vec![Move::Deal],
            Phase::Play => {
                let p = self.side;
                match &self.pending {
                    Pending::EnvidoCount => vec![Move::Count],
                    Pending::Envido { .. } => {
                        let mut v = vec![Move::Quiero, Move::NoQuiero];
                        for k in 0..3u8 {
                            if env_next_ok(&self.env_chain, k) {
                                v.push(Move::Envido(k));
                            }
                        }
                        v
                    }
                    Pending::Truco { .. } => {
                        let mut v = vec![Move::Quiero, Move::NoQuiero];
                        if self.req < 3 {
                            v.push(Move::Truco(self.req as u8 + 1));
                        }
                        v
                    }
                    Pending::None => {
                        let mut v: Vec<Move> =
                            self.hands[p.idx()].iter().map(|&c| Move::Play(c)).collect();
                        v.push(Move::Mazo);
                        let can_raise = match self.t_owner {
                            None => self.level == 0,
                            Some(o) => o == p && self.level < 3,
                        };
                        if can_raise {
                            v.push(Move::Truco(self.level as u8 + 1));
                        }
                        if self.can_call_envido(p) {
                            for k in 0..3u8 {
                                v.push(Move::Envido(k));
                            }
                        }
                        v
                    }
                }
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        match *m {
            Move::Deal => self.deal(),
            Move::Play(c) => {
                let p = self.side;
                let pos = self.hands[p.idx()].iter().position(|&x| x == c).unwrap();
                self.hands[p.idx()].remove(pos);
                let first_of_round = match self.table.last() {
                    None => true,
                    Some(row) => row[0].is_some() && row[1].is_some(),
                };
                if first_of_round {
                    let mut row = [None, None];
                    row[p.idx()] = Some(c);
                    self.table.push(row);
                    self.turn = p.other();
                    self.side = self.turn;
                    return;
                }
                let row = self.table.last_mut().unwrap();
                row[p.idx()] = Some(c);
                let (a, b) = (row[0].unwrap(), row[1].unwrap());
                let res = match power(a).cmp(&power(b)) {
                    std::cmp::Ordering::Greater => Some(Player::One),
                    std::cmp::Ordering::Less => Some(Player::Two),
                    std::cmp::Ordering::Equal => None,
                };
                self.results.push(res);
                if let Some(w) = self.hand_winner() {
                    self.end_hand(w);
                    return;
                }
                // cine conduce runda următoare: câștigătorul, la egalitate mano
                let lead = res.unwrap_or(self.mano());
                self.turn = lead;
                self.side = lead;
            }
            Move::Mazo => {
                let p = self.side;
                self.log = format!("J{} se retrage.", p.idx() + 1);
                self.end_hand(p.other());
            }
            Move::Truco(l) => {
                let p = self.side;
                if matches!(self.pending, Pending::Truco { .. }) {
                    // contra-urcare: truco-ul anterior e acceptat implicit
                    self.level = self.req;
                }
                self.req = l as usize;
                self.pending = Pending::Truco { caller: p };
                self.side = p.other();
            }
            Move::Envido(k) => {
                let p = self.side;
                self.env_chain.push(k);
                self.pending = Pending::Envido { caller: p };
                self.side = p.other();
            }
            Move::Quiero => match self.pending.clone() {
                Pending::Truco { caller } => {
                    self.level = self.req;
                    self.t_owner = Some(caller.other());
                    self.pending = Pending::None;
                    self.side = self.turn;
                }
                Pending::Envido { .. } => {
                    self.pending = Pending::EnvidoCount;
                    self.side = self.mano();
                }
                _ => {}
            },
            Move::NoQuiero => match self.pending.clone() {
                Pending::Truco { caller } => {
                    self.log = format!("J{} nu acceptă.", caller.other().idx() + 1);
                    // mizele rămân cele dinainte
                    self.pending = Pending::None;
                    self.end_hand(caller);
                }
                Pending::Envido { caller } => {
                    let n = self.env_chain.len();
                    let pts = if n > 1 {
                        self.env_stake(&self.env_chain[..n - 1])
                    } else {
                        1
                    };
                    self.log = format!(
                        "J{} nu acceptă envido: J{} +{}",
                        caller.other().idx() + 1,
                        caller.idx() + 1,
                        pts
                    );
                    self.env_done = true;
                    self.pending = Pending::None;
                    self.give(caller, pts);
                    self.side = self.turn;
                }
                _ => {}
            },
            Move::Count => {
                let stake = self.env_stake(&self.env_chain);
                let t = [tanto(&self.orig[0]), tanto(&self.orig[1])];
                let mano = self.mano();
                let w = match t[0].cmp(&t[1]) {
                    std::cmp::Ordering::Greater => Player::One,
                    std::cmp::Ordering::Less => Player::Two,
                    std::cmp::Ordering::Equal => mano,
                };
                self.log = format!(
                    "Tantos: J1 {}, J2 {} → J{} +{}",
                    t[0],
                    t[1],
                    w.idx() + 1,
                    stake
                );
                self.env_done = true;
                self.pending = Pending::None;
                self.give(w, stake);
                self.side = self.turn;
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        Some(if self.scores[0] >= TARGET {
            Outcome::Win(Player::One)
        } else {
            Outcome::Win(Player::Two)
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let q = p.other();
        let mut s = 1000 * (self.scores[p.idx()] - self.scores[q.idx()]);
        if self.phase != Phase::Play {
            return s;
        }
        let pw = self.win_prob(p);
        let stake = STAKES[self.level] as f32;
        match &self.pending {
            Pending::None => {
                s += (stake * (2.0 * pw - 1.0) * 600.0) as i32;
            }
            Pending::Truco { caller } => {
                let new = STAKES[self.req] as f32;
                if *caller == p {
                    s += (0.05 * stake * 1000.0 + 0.95 * new * (2.0 * pw - 1.0) * 600.0
                        - 60.0 * (new - stake)) as i32;
                } else {
                    s += (new * (2.0 * pw - 1.0) * 600.0) as i32;
                }
            }
            Pending::Envido { caller } => {
                let pe = self.env_win_prob(p);
                let st = self.env_stake(&self.env_chain) as f32;
                let n = self.env_chain.len();
                let prev = if n > 1 {
                    self.env_stake(&self.env_chain[..n - 1]) as f32
                } else {
                    1.0
                };
                s += (stake * (2.0 * pw - 1.0) * 600.0) as i32;
                if *caller == p {
                    s += (0.1 * prev * 1000.0 + 0.9 * st * (2.0 * pe - 1.0) * 1000.0
                        - 80.0 * (st - prev)) as i32;
                }
            }
            Pending::EnvidoCount => {
                let pe = self.env_win_prob(p);
                let st = self.env_stake(&self.env_chain) as f32;
                s += (st * (2.0 * pe - 1.0) * 1000.0) as i32;
                s += (stake * (2.0 * pw - 1.0) * 600.0) as i32;
            }
        }
        s
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Truco — scor: J1 {}, J2 {} (până la {}) | dealer: J{} | mano: J{}\n",
            self.scores[0],
            self.scores[1],
            TARGET,
            self.dealer.idx() + 1,
            self.mano().idx() + 1
        );
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        if self.phase == Phase::Over {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        if self.phase == Phase::Play {
            for (i, row) in self.table.iter().enumerate() {
                let f = |o: Option<Card>| o.map(card_text).unwrap_or_else(|| "—".into());
                let res = match self.results.get(i) {
                    Some(Some(w)) => format!("J{}", w.idx() + 1),
                    Some(None) => "parda".into(),
                    None => "…".into(),
                };
                s.push_str(&format!(
                    "Runda {}: J1 {} | J2 {} → {}\n",
                    i + 1,
                    f(row[0]),
                    f(row[1]),
                    res
                ));
            }
            s.push_str(&format!(
                "Miza mâinii: {} punct(e){}\n",
                STAKES[self.level],
                match self.pending {
                    Pending::Truco { caller } =>
                        format!(" — J{} cere {} puncte", caller.idx() + 1, STAKES[self.req]),
                    Pending::Envido { caller } => format!(" — J{} cântă envido", caller.idx() + 1),
                    _ => String::new(),
                }
            ));
            let v = self.viewer.who(self.side);
            let o = v.other();
            let mine: Vec<Card> = self.hands[v.idx()].clone();
            s.push_str(&format!(
                "Mâna ta (J{}): {}\n",
                v.idx() + 1,
                join_cards_sp(&mine)
            ));
            s.push_str(&format!(
                "Tantosul tău (envido): {}\n",
                tanto(&self.orig[v.idx()])
            ));
            s.push_str(&format!(
                "J{} are {} cărți\n",
                o.idx() + 1,
                self.hands[o.idx()].len()
            ));
        }
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Deal => "reparte".into(),
            Move::Play(c) => card_text(*c),
            Move::Mazo => "mazo".into(),
            Move::Truco(1) => "truco".into(),
            Move::Truco(2) => "retruco".into(),
            Move::Truco(_) => "vale4".into(),
            Move::Envido(0) => "envido".into(),
            Move::Envido(1) => "real".into(),
            Move::Envido(_) => "falta".into(),
            Move::Quiero => "quiero".into(),
            Move::NoQuiero => "noquiero".into(),
            Move::Count => "cuenta".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

fn join_cards_sp(cs: &[Card]) -> String {
    if cs.is_empty() {
        return join_cards(cs);
    }
    cs.iter()
        .map(|&c| card_text(c))
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(rank: u8, suit: u8) -> Card {
        Card { rank, suit }
    }

    fn setup(h1: Vec<Card>, h2: Vec<Card>) -> Truco {
        let mut g = Truco::with_seed(1);
        g.hands = [h1.clone(), h2.clone()];
        g.orig = [h1, h2];
        g.scores = [0, 0];
        g
    }

    #[test]
    fn hierarchy() {
        assert!(power(c(1, 0)) > power(c(1, 1)));
        assert!(power(c(1, 1)) > power(c(7, 0)));
        assert!(power(c(7, 0)) > power(c(7, 2)));
        assert!(power(c(7, 2)) > power(c(3, 3)));
        assert!(power(c(3, 3)) > power(c(2, 1)));
        assert!(power(c(2, 1)) > power(c(1, 2)));
        assert_eq!(power(c(1, 2)), power(c(1, 3)));
        assert!(power(c(1, 3)) > power(c(12, 0)));
        assert!(power(c(10, 0)) > power(c(7, 1)));
        assert_eq!(power(c(7, 1)), power(c(7, 3)));
        assert!(power(c(7, 3)) > power(c(6, 0)));
    }

    #[test]
    fn envido_points() {
        assert_eq!(tanto(&[c(7, 0), c(6, 0), c(1, 1)]), 33);
        assert_eq!(tanto(&[c(12, 0), c(11, 0), c(5, 2)]), 20);
        assert_eq!(tanto(&[c(7, 0), c(6, 1), c(5, 2)]), 7);
        assert_eq!(tanto(&[c(12, 0), c(11, 1), c(10, 2)]), 0);
    }

    #[test]
    fn deck_is_40_and_deal_distinct() {
        assert_eq!(deck40().len(), 40);
        let g = Truco::with_seed(7);
        let mut all = g.hands[0].clone();
        all.extend(&g.hands[1]);
        all.sort_by_key(|c| (c.suit, c.rank));
        all.dedup();
        assert_eq!(all.len(), 6);
    }

    #[test]
    fn round_and_hand_winner() {
        // J2 mano? dealer = Two la început => mano = J1
        let mut g = setup(
            vec![c(1, 0), c(4, 1), c(5, 1)],
            vec![c(3, 0), c(2, 1), c(4, 2)],
        );
        g.apply(&Move::Play(c(1, 0)));
        g.apply(&Move::Play(c(3, 0)));
        assert_eq!(g.results, vec![Some(Player::One)]);
        assert_eq!(g.side, Player::One);
        g.apply(&Move::Play(c(4, 1)));
        g.apply(&Move::Play(c(2, 1)));
        // J2 a luat a doua; a treia decide
        assert_eq!(g.results.len(), 2);
        assert_eq!(g.side, Player::Two);
        g.apply(&Move::Play(c(4, 2)));
        g.apply(&Move::Play(c(5, 1)));
        // 5 bate 4 => J1 are 2 runde; mâna se încheie cu 1 punct
        assert_eq!(g.scores, [1, 0]);
        assert_eq!(g.phase, Phase::Deal);
    }

    #[test]
    fn parda_first_round_winner_takes_hand() {
        let mut g = setup(
            vec![c(3, 0), c(4, 1), c(5, 1)],
            vec![c(3, 1), c(2, 1), c(4, 2)],
        );
        g.apply(&Move::Play(c(3, 0)));
        g.apply(&Move::Play(c(3, 1)));
        assert_eq!(g.results, vec![None]);
        // mano (J1) conduce din nou
        assert_eq!(g.side, Player::One);
        g.apply(&Move::Play(c(4, 1)));
        g.apply(&Move::Play(c(2, 1)));
        assert_eq!(g.scores, [0, 1]);
    }

    #[test]
    fn truco_chain_and_fold() {
        let mut g = setup(
            vec![c(1, 0), c(4, 1), c(5, 1)],
            vec![c(3, 0), c(2, 1), c(4, 2)],
        );
        g.apply(&Move::Truco(1));
        assert_eq!(g.side, Player::Two);
        assert!(g.legal_moves().contains(&Move::Truco(2)));
        g.apply(&Move::Truco(2)); // retruco = acceptă implicit truco-ul
        assert_eq!(g.level, 1);
        assert_eq!(g.side, Player::One);
        g.apply(&Move::NoQuiero);
        // J2 a cerut 3, J1 refuză: J2 ia miza dinainte (2)
        assert_eq!(g.scores, [0, 2]);
    }

    #[test]
    fn quiero_gives_raise_right_to_acceptor() {
        let mut g = setup(
            vec![c(1, 0), c(4, 1), c(5, 1)],
            vec![c(3, 0), c(2, 1), c(4, 2)],
        );
        g.apply(&Move::Truco(1));
        g.apply(&Move::Quiero);
        assert_eq!(g.level, 1);
        assert_eq!(g.side, Player::One);
        // J1 (caller) nu mai poate cere retruco; J2 da
        assert!(!g.legal_moves().contains(&Move::Truco(2)));
        g.apply(&Move::Play(c(1, 0)));
        assert!(g.legal_moves().contains(&Move::Truco(2)));
    }

    #[test]
    fn envido_flow() {
        let mut g = setup(
            vec![c(7, 0), c(6, 0), c(1, 1)],
            vec![c(12, 0), c(11, 1), c(4, 2)],
        );
        g.apply(&Move::Envido(0));
        assert_eq!(g.side, Player::Two);
        assert!(g.legal_moves().contains(&Move::Envido(1)));
        g.apply(&Move::Quiero);
        assert_eq!(g.legal_moves(), vec![Move::Count]);
        g.apply(&Move::Count);
        assert_eq!(g.scores, [2, 0]); // 33 vs 4
        assert_eq!(g.side, Player::One);
        assert!(!g.legal_moves().contains(&Move::Envido(0)));
    }

    #[test]
    fn envido_decline_gives_one() {
        let mut g = setup(
            vec![c(7, 0), c(6, 0), c(1, 1)],
            vec![c(12, 0), c(11, 1), c(4, 2)],
        );
        g.apply(&Move::Envido(0));
        g.apply(&Move::NoQuiero);
        assert_eq!(g.scores, [1, 0]);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=10u64 {
            let mut g = Truco::with_seed(seed);
            let mut rng = Rng::new(seed + 11);
            let mut n = 0;
            while g.outcome().is_none() && n < 20000 {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed}");
        }
    }
}

#[cfg(test)]
mod a1_blind {
    use super::*;
    use crate::ai_audit_a1::{audit, fake_cards};

    #[test]
    fn ai_ignores_opponent_cards_at_all_levels() {
        audit("truco", Truco::with_seed, 1..=4, 3, 90, |g, salt| {
            let me = g.side.idx();
            let foe = 1 - me;
            let mut ex: Vec<Card> = g.hands[me].clone();
            ex.extend(g.orig[me].iter());
            for t in &g.table {
                ex.extend(t.iter().flatten());
            }
            let fake = fake_cards(deck40(), &ex, g.orig[foe].len(), salt);
            let k = g.hands[foe].len().min(fake.len());
            g.orig[foe] = fake.clone();
            g.hands[foe] = fake[..k].to_vec();
            g.rng = Rng::new(salt ^ 3);
        });
    }
}
