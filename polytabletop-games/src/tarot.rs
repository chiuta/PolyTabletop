//! Tarot francez (78 de cărți) pentru 2 jucători — variantă adaptată, din regulile publice.
//!
//! * **Pachetul**: 56 de cărți de culoare (c, d, h, s: 1–10, V valet, C cavaler, D damă,
//!   R rege), 21 de atuuri (T1 „petit” … T21) și Scuza (Ex). Oudlers: T1, T21, Ex.
//! * **Împărțire (simplificare pentru doi)**: 36 de cărți fiecăruia și 6 în *chien*.
//!   Cine nu a împărțit vorbește primul. Nu se numără „în mână” ce are celălalt: se
//!   vede doar mâna ta, dar toate cărțile se joacă.
//! * **Licitație** (un singur tur): `pas`, `petite` (chien-ul se arată și se ia), `garde`
//!   (chien-ul se ia, fără a fi arătat), `garde-sans` (chien-ul rămâne pentru cine a luat),
//!   `garde-contre` (chien-ul merge la apărare). Al doilea trebuie să licite mai sus.
//!   Dacă ambii dau pas, se re-împarte (numără ca mână). Multiplicatori: 1, 2, 4, 6.
//! * **Chien**: cine a luat primește chien-ul (`chien`, mutare forțată), apoi pune jos 6
//!   cărți una câte una (`ecart Vh`), fără regi și fără oudlers, atuuri doar dacă nu mai
//!   există altceva; cărțile puse jos îi revin la numărătoare.
//! * **Levate**: se răspunde la culoare; fără culoare tai cu atu (dacă ai); la atu se urcă
//!   dacă se poate. Scuza se joacă oricând, nu câștigă levata și rămâne în grămada
//!   celui care a jucat-o (se omite schimbul cu o carte mică; se omite regula
//!   Scuzei la ultima levată). Cine câștigă ia celelalte cărți; câștigătorul conduce.
//! * **Numărătoare** (în jumătăți de puncte, 91 în total): rege 4,5, damă 3,5, cavaler 2,5,
//!   valet 1,5, oudler 4,5, restul 0,5. Contractul: 56 / 51 / 41 / 36 de puncte cu 0 / 1 / 2 / 3
//!   oudlers. Scor mână = (25 + diferența + 10 pentru petit au bout) × multiplicator,
//!   jocul e zero-sum între cei doi. Petit au bout: T1 în ultima levată câștigată de cine
//!   l-a jucat. Fără poignée, chelem și anunțuri.
//! * Meciul are 3 mâini; câștigă cine are mai multe puncte (egal = remiză).
//!
//! Informație ascunsă: vezi `cards52.rs`. Cărțile din chien (la `garde`) și cărțile
//! puse jos nu sunt vizibile apărării.

use crate::cards::Viewer;
use crate::cards52::{shuffle, SUIT_LETTERS};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const RANK_LABEL: [&str; 14] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "V", "C", "D", "R",
];
const EXCUSE: u8 = 77;
const HANDS: u8 = 3;
const REQ2: [i32; 4] = [112, 102, 82, 72];
const MULT: [i32; 5] = [0, 1, 2, 4, 6];
const BID_NAMES: [&str; 5] = ["pas", "petite", "garde", "garde-sans", "garde-contre"];

pub fn is_trump(c: u8) -> bool {
    (56..=76).contains(&c)
}
/// Valoarea atuului 1..=21.
pub fn trump_val(c: u8) -> u8 {
    c - 55
}
pub fn suit(c: u8) -> u8 {
    c / 14
}
pub fn rank(c: u8) -> u8 {
    c % 14
}
pub fn is_oudler(c: u8) -> bool {
    c == 56 || c == 76 || c == EXCUSE
}
pub fn is_king(c: u8) -> bool {
    c < 56 && rank(c) == 13
}
/// Valoarea în jumătăți de puncte.
pub fn card_half_points(c: u8) -> i32 {
    if is_oudler(c) {
        return 9;
    }
    if is_trump(c) {
        return 1;
    }
    match rank(c) {
        13 => 9,
        12 => 7,
        11 => 5,
        10 => 3,
        _ => 1,
    }
}
pub fn card_text(c: u8) -> String {
    if c == EXCUSE {
        "Ex".into()
    } else if is_trump(c) {
        format!("T{}", trump_val(c))
    } else {
        format!(
            "{}{}",
            RANK_LABEL[rank(c) as usize],
            SUIT_LETTERS[suit(c) as usize]
        )
    }
}

/// Cartea `resp` (a doua, nu Scuza) bate cartea condusă `led` (nu Scuza)?
fn second_wins(led: u8, resp: u8) -> bool {
    if is_trump(led) {
        is_trump(resp) && resp > led
    } else if is_trump(resp) {
        true
    } else {
        suit(resp) == suit(led) && rank(resp) > rank(led)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Move {
    Bid(u8),
    Dog,
    Discard(u8),
    Play(u8),
    Redeal,
    Next,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Bid,
    NoDeal,
    Dog,
    Discard,
    Play,
    HandOver,
}

#[derive(Clone)]
pub struct Tarot {
    rng: Rng,
    hands: [Vec<u8>; 2],
    dog: Vec<u8>,
    dealer: Player,
    side: Player,
    phase: Phase,
    /// Cel mai înalt contract spus și cine l-a spus.
    level: u8,
    taker: Player,
    first_bidder_passed: bool,
    bids_made: u8,
    /// Cărțile câștigate în levate.
    piles: [Vec<u8>; 2],
    /// Cărți ascunse care se numără (chien / ecart), pe jucător.
    extra: [Vec<u8>; 2],
    dog_public: bool,
    led: Option<u8>,
    trick_no: u8,
    petit_bonus: i32,
    hands_done: u8,
    total: [i32; 2],
    last_hand: Option<String>,
    viewer: Viewer,
}

impl Default for Tarot {
    fn default() -> Self {
        Self::new()
    }
}

impl Tarot {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Tarot {
            rng: Rng::new(seed),
            hands: [Vec::new(), Vec::new()],
            dog: Vec::new(),
            dealer: Player::Two,
            side: Player::One,
            phase: Phase::Bid,
            level: 0,
            taker: Player::One,
            first_bidder_passed: false,
            bids_made: 0,
            piles: [Vec::new(), Vec::new()],
            extra: [Vec::new(), Vec::new()],
            dog_public: false,
            led: None,
            trick_no: 0,
            petit_bonus: 0,
            hands_done: 0,
            total: [0; 2],
            last_hand: None,
            viewer: Viewer::Fixed(Player::One),
        };
        g.deal_hand();
        g
    }

    fn deal_hand(&mut self) {
        let mut deck: Vec<u8> = (0..78).collect();
        shuffle(&mut deck, &mut self.rng);
        self.hands = [deck[0..36].to_vec(), deck[36..72].to_vec()];
        self.dog = deck[72..78].to_vec();
        self.phase = Phase::Bid;
        self.side = self.dealer.other();
        self.level = 0;
        self.first_bidder_passed = false;
        self.bids_made = 0;
        self.piles = [Vec::new(), Vec::new()];
        self.extra = [Vec::new(), Vec::new()];
        self.dog_public = false;
        self.led = None;
        self.trick_no = 0;
        self.petit_bonus = 0;
    }

    pub fn totals(&self) -> (i32, i32) {
        (self.total[0], self.total[1])
    }
    pub fn hand_of(&self, p: Player) -> &[u8] {
        &self.hands[p.idx()]
    }
    pub fn hands_done(&self) -> u8 {
        self.hands_done
    }

    fn over(&self) -> bool {
        self.hands_done >= HANDS
    }

    /// Forța mâinii pentru licitație (doar cele 36 de cărți proprii).
    fn strength(hand: &[u8]) -> i32 {
        let mut s = 0;
        let trumps = hand.iter().filter(|&&c| is_trump(c)).count() as i32;
        s += (trumps - 9) * 2;
        for &c in hand {
            if is_trump(c) {
                if trump_val(c) >= 15 {
                    s += 2;
                }
                if trump_val(c) >= 20 {
                    s += 2;
                }
            }
            if is_oudler(c) || is_king(c) {
                s += 3;
            } else if c < 56 && rank(c) == 12 {
                s += 1;
            }
        }
        for sv in 0..4u8 {
            if !hand.iter().any(|&c| c < 56 && suit(c) == sv) {
                s += 3;
            }
        }
        s
    }

    fn req_strength(level: u8) -> i32 {
        match level {
            1 => 14,
            2 => 20,
            3 => 28,
            4 => 36,
            _ => 0,
        }
    }

    fn playable(&self, me: Player) -> Vec<u8> {
        let hand = &self.hands[me.idx()];
        let Some(l) = self.led else {
            return hand.clone();
        };
        if l == EXCUSE {
            return hand.clone();
        }
        let ex = hand.contains(&EXCUSE);
        let mut v: Vec<u8> = if is_trump(l) {
            let trumps: Vec<u8> = hand.iter().copied().filter(|&c| is_trump(c)).collect();
            let higher: Vec<u8> = trumps.iter().copied().filter(|&c| c > l).collect();
            if !higher.is_empty() {
                higher
            } else if !trumps.is_empty() {
                trumps
            } else {
                return hand.clone();
            }
        } else {
            let same: Vec<u8> = hand
                .iter()
                .copied()
                .filter(|&c| c < 56 && suit(c) == suit(l))
                .collect();
            if !same.is_empty() {
                same
            } else {
                let trumps: Vec<u8> = hand.iter().copied().filter(|&c| is_trump(c)).collect();
                if trumps.is_empty() {
                    return hand.clone();
                }
                trumps
            }
        };
        if ex {
            v.push(EXCUSE);
        }
        v
    }

    fn discard_options(&self) -> Vec<u8> {
        let hand = &self.hands[self.taker.idx()];
        let ok = |c: &&u8| !is_oudler(**c) && !is_king(**c);
        let plain: Vec<u8> = hand
            .iter()
            .filter(ok)
            .copied()
            .filter(|&c| !is_trump(c))
            .collect();
        if !plain.is_empty() {
            return plain;
        }
        hand.iter().filter(ok).copied().collect()
    }

    fn finish_hand(&mut self) {
        let t = self.taker.idx();
        let d = 1 - t;
        let pts = |p: usize| -> i32 {
            self.piles[p]
                .iter()
                .chain(&self.extra[p])
                .map(|&c| card_half_points(c))
                .sum()
        };
        let tp = pts(t);
        let oud = self.piles[t]
            .iter()
            .chain(&self.extra[t])
            .filter(|&&c| is_oudler(c))
            .count();
        let req = REQ2[oud];
        let diff2 = tp - req;
        let won = diff2 >= 0;
        let diff = (diff2.abs() + 1) / 2;
        let mult = MULT[self.level as usize];
        let base = 25 + diff;
        let delta = (if won { base } else { -base } + self.petit_bonus) * mult;
        self.total[t] += delta;
        self.total[d] -= delta;
        self.hands_done += 1;
        self.phase = Phase::HandOver;
        self.last_hand = Some(format!(
            "J{} a luat «{}» cu {} oudlers: {:.1} puncte (cerut {:.1}), {} {:+}",
            t + 1,
            BID_NAMES[self.level as usize],
            oud,
            tp as f32 / 2.0,
            req as f32 / 2.0,
            if won { "reușit" } else { "căzut" },
            delta
        ));
    }
}

impl Tarot {
    fn bid_pick(&self) -> Move {
        let st = Self::strength(&self.hands[self.side.idx()]);
        let mut best = 0;
        for l in self.level + 1..=4 {
            if st >= Self::req_strength(l) {
                best = l;
            }
        }
        Move::Bid(best)
    }

    fn discard_pick(&self, rng: &mut Rng) -> u8 {
        let hand = &self.hands[self.taker.idx()];
        let opts = self.discard_options();
        let mut best = (i32::MAX, opts[0]);
        for &c in &opts {
            let n = hand
                .iter()
                .filter(|&&x| x < 56 && c < 56 && suit(x) == suit(c))
                .count() as i32;
            let key = (card_half_points(c) * 3 + n * 2 + if is_trump(c) { 25 } else { 0 }) * 4
                + rng.below(4) as i32;
            if key < best.0 {
                best = (key, c);
            }
        }
        best.1
    }

    fn play_pick(&self, played: &[bool; 78], rng: &mut Rng) -> u8 {
        let hand = &self.hands[self.side.idx()];
        let mv = self.playable(self.side);
        let master = |c: u8| -> bool {
            if c == EXCUSE {
                return false;
            }
            let better = |d: u8| {
                if is_trump(c) {
                    is_trump(d) && d > c
                } else {
                    d < 56 && suit(d) == suit(c) && rank(d) > rank(c)
                }
            };
            (0..77u8).all(|d| !better(d) || played[d as usize] || hand.contains(&d))
        };
        let cost = |c: u8| -> i32 {
            if c == EXCUSE {
                return 40;
            }
            if is_trump(c) {
                card_half_points(c) * 2 + 20 + trump_val(c) as i32
            } else {
                card_half_points(c) * 3 + rank(c) as i32
            }
        };
        let mut best = (i32::MIN, mv[0]);
        for &c in &mv {
            let sc = match self.led {
                None => {
                    if c == EXCUSE {
                        -45
                    } else if master(c) {
                        if is_trump(c) {
                            80
                        } else {
                            60 + card_half_points(c)
                        }
                    } else {
                        -cost(c)
                    }
                }
                Some(l) => {
                    let tp = card_half_points(l);
                    if l == EXCUSE {
                        -cost(c)
                    } else if c == EXCUSE {
                        if tp >= 5 {
                            -20 + tp * 4
                        } else {
                            -45
                        }
                    } else if second_wins(l, c) {
                        if is_trump(c) && !is_trump(l) && tp < 3 {
                            10 - cost(c)
                        } else {
                            100 - cost(c)
                        }
                    } else {
                        -cost(c)
                    }
                }
            };
            let sc = sc * 4 + rng.below(4) as i32;
            if sc > best.0 {
                best = (sc, c);
            }
        }
        best.1
    }

    /// Joacă mâna curentă până la capăt; întoarce diferența de total (p − adversar).
    fn rollout(&mut self, p: Player, rng: &mut Rng) -> f64 {
        let mut played = [false; 78];
        for &c in self.piles[0]
            .iter()
            .chain(&self.piles[1])
            .chain(self.led.iter())
        {
            played[c as usize] = true;
        }
        let mut guard = 0;
        while !matches!(self.phase, Phase::HandOver | Phase::NoDeal) && guard < 200 {
            guard += 1;
            match self.phase {
                Phase::Bid => {
                    let m = self.bid_pick();
                    self.apply(&m);
                }
                Phase::Dog => self.apply(&Move::Dog),
                Phase::Discard => {
                    let c = self.discard_pick(rng);
                    self.apply(&Move::Discard(c));
                }
                Phase::Play => {
                    let c = self.play_pick(&played, rng);
                    played[c as usize] = true;
                    self.apply(&Move::Play(c));
                }
                _ => break,
            }
        }
        let d = (self.total[p.idx()] - self.total[p.other().idx()]) as f64;
        if self.over() {
            d + if d > 0.0 {
                150.0
            } else if d < 0.0 {
                -150.0
            } else {
                0.0
            }
        } else {
            d
        }
    }
}

impl Game for Tarot {
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
        if self.over() {
            return Vec::new();
        }
        match self.phase {
            Phase::Bid => {
                let mut v = vec![Move::Bid(0)];
                for l in self.level + 1..=4 {
                    v.push(Move::Bid(l));
                }
                v
            }
            Phase::NoDeal => vec![Move::Redeal],
            Phase::Dog => vec![Move::Dog],
            Phase::Discard => self
                .discard_options()
                .into_iter()
                .map(Move::Discard)
                .collect(),
            Phase::Play => self
                .playable(self.side)
                .into_iter()
                .map(Move::Play)
                .collect(),
            Phase::HandOver => vec![Move::Next],
        }
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Bid(l) => {
                if *l > 0 {
                    self.level = *l;
                    self.taker = self.side;
                }
                self.bids_made += 1;
                if self.bids_made == 2 {
                    self.phase = if self.level == 0 {
                        Phase::NoDeal
                    } else {
                        Phase::Dog
                    };
                    self.side = if self.level == 0 {
                        self.dealer.other()
                    } else {
                        self.taker
                    };
                } else {
                    self.side = self.side.other();
                }
            }
            Move::Redeal => {
                self.hands_done += 1;
                self.dealer = self.dealer.other();
                if !self.over() {
                    self.deal_hand();
                }
            }
            Move::Next => {
                self.dealer = self.dealer.other();
                if !self.over() {
                    self.deal_hand();
                }
            }
            Move::Dog => {
                let t = self.taker.idx();
                let dog = std::mem::take(&mut self.dog);
                match self.level {
                    1 | 2 => {
                        self.dog_public = self.level == 1;
                        self.dog = dog.clone();
                        self.hands[t].extend(dog);
                        self.phase = Phase::Discard;
                    }
                    3 => {
                        self.extra[t].extend(dog);
                        self.phase = Phase::Play;
                        self.side = self.dealer.other();
                    }
                    _ => {
                        self.extra[1 - t].extend(dog);
                        self.phase = Phase::Play;
                        self.side = self.dealer.other();
                    }
                }
            }
            Move::Discard(c) => {
                let t = self.taker.idx();
                let h = &mut self.hands[t];
                let pos = h.iter().position(|x| x == c).expect("card in hand");
                h.remove(pos);
                self.extra[t].push(*c);
                if self.extra[t].len() == 6 {
                    self.phase = Phase::Play;
                    self.side = self.dealer.other();
                }
            }
            Move::Play(c) => {
                let me = self.side;
                let h = &mut self.hands[me.idx()];
                let pos = h.iter().position(|x| x == c).expect("card in hand");
                h.remove(pos);
                match self.led {
                    None => {
                        self.led = Some(*c);
                        self.side = me.other();
                    }
                    Some(l) => {
                        let leader = me.other();
                        let winner = if l == EXCUSE {
                            me
                        } else if *c == EXCUSE {
                            leader
                        } else if second_wins(l, *c) {
                            me
                        } else {
                            leader
                        };
                        // Scuza rămâne la cine a jucat-o
                        for (card, who) in [(l, leader), (*c, me)] {
                            if card == EXCUSE {
                                self.piles[who.idx()].push(card);
                            } else {
                                self.piles[winner.idx()].push(card);
                            }
                        }
                        self.trick_no += 1;
                        self.led = None;
                        self.side = winner;
                        if self.trick_no == 36 {
                            // petit au bout: T1 în ultima levată, câștigată de cine l-a jucat
                            let petit_by = if l == 56 {
                                Some(leader)
                            } else if *c == 56 {
                                Some(me)
                            } else {
                                None
                            };
                            if petit_by == Some(winner) {
                                self.petit_bonus = if winner == self.taker { 10 } else { -10 };
                            }
                            self.finish_hand();
                        }
                    }
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over() {
            return None;
        }
        Some(match self.total[0].cmp(&self.total[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    /// Evaluare cinstită prin determinizare. Citește doar informația vizibilă lui `p`: mâna
    /// lui `p`, propriul ecart / chien luat, cărțile deja jucate (grămezile de levate),
    /// licitația, scorurile și mărimile mâinilor. Mâna adversarului, ecartul lui și chien-ul
    /// necunoscut se eșantionează din cărțile nevăzute, apoi mâna se joacă până la capăt cu
    /// o politică euristică (licitație, ecart, joc); rezultatul e diferența medie de scor.
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let o = 1 - me;
        if matches!(self.phase, Phase::HandOver | Phase::NoDeal) || self.over() {
            return (self.total[me] - self.total[o]) * 20;
        }
        let dog_unknown = matches!(self.phase, Phase::Bid | Phase::Dog);
        let mut seen = [false; 78];
        for &c in self.hands[me].iter().chain(&self.extra[me]) {
            seen[c as usize] = true;
        }
        for &c in self.piles[0]
            .iter()
            .chain(&self.piles[1])
            .chain(self.led.iter())
        {
            seen[c as usize] = true;
        }
        let mut pool: Vec<u8> = (0..78u8).filter(|&c| !seen[c as usize]).collect();
        let n_opp = self.hands[o].len();
        let n_ext = self.extra[o].len();
        let n_dog = if dog_unknown { self.dog.len() } else { 0 };
        if pool.len() < n_opp + n_ext + n_dog {
            return 0;
        }
        let mut seed: u64 = 0x7A60_7000 ^ (self.trick_no as u64) << 8 ^ (n_opp as u64) << 20;
        seed ^= (self.level as u64) << 28 ^ (self.hands_done as u64) << 32 ^ self.phase as u64;
        for &c in self.hands[me].iter().chain(self.led.iter()) {
            seed = seed.rotate_left(5) ^ (c as u64 + 1);
        }
        seed ^= ((self.total[0] + 1000) as u64) << 40 ^ ((self.total[1] + 1000) as u64) << 52;
        let mut rng = Rng::new(seed);
        let n = (4000 / (self.hands[me].len() * 2 + 12)).clamp(16, 100);
        let mut total = 0.0f64;
        for _ in 0..n {
            shuffle(&mut pool, &mut rng);
            let mut g = self.clone();
            g.hands[o] = pool[..n_opp].to_vec();
            g.extra[o] = pool[n_opp..n_opp + n_ext].to_vec();
            if dog_unknown {
                g.dog = pool[n_opp + n_ext..n_opp + n_ext + n_dog].to_vec();
            }
            total += g.rollout(p, &mut rng);
        }
        (20.0 * total / n as f64) as i32
    }

    fn render(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            "Tarot (2 jucători, 78 de cărți) — mâna {} din {HANDS}: J1 {} – J2 {}\n",
            (self.hands_done + 1).min(HANDS),
            self.total[0],
            self.total[1]
        ));
        match self.phase {
            Phase::Bid => s.push_str(&format!(
                "Licitație: cel mai înalt contract: {}\n",
                BID_NAMES[self.level as usize]
            )),
            Phase::NoDeal => s.push_str("Nimeni n-a licitat. Scrie «reimparte».\n"),
            Phase::Dog => s.push_str(&format!(
                "J{} a luat «{}». Scrie «chien».\n",
                self.taker.idx() + 1,
                BID_NAMES[self.level as usize]
            )),
            _ => s.push_str(&format!(
                "A luat Jucător {} («{}»).\n",
                self.taker.idx() + 1,
                BID_NAMES[self.level as usize]
            )),
        }
        if self.dog_public && self.phase == Phase::Discard {
            s.push_str(&format!(
                "Chien (arătat): {}\n",
                self.dog
                    .iter()
                    .map(|&c| card_text(c))
                    .collect::<Vec<_>>()
                    .join(" ")
            ));
        }
        if self.phase == Phase::Play || self.phase == Phase::HandOver {
            s.push_str(&format!(
                "Levate câștigate: J1 {} cărți, J2 {} cărți; levata nr. {}\n",
                self.piles[0].len(),
                self.piles[1].len(),
                (self.trick_no + 1).min(36)
            ));
        }
        let v = self.viewer.who(self.side);
        s.push_str(&format!(
            "Jucător {} (ascuns) are {} cărți.\n",
            v.other().idx() + 1,
            self.hands[v.other().idx()].len()
        ));
        if let Some(l) = self.led {
            s.push_str(&format!(
                "Pe masă: Jucător {} a jucat {}\n",
                self.side.other().idx() + 1,
                card_text(l)
            ));
        }
        let mut h = self.hands[v.idx()].clone();
        h.sort_unstable();
        s.push_str(&format!(
            "Mâna ta (Jucător {}): {}\n",
            v.idx() + 1,
            if h.is_empty() {
                "—".into()
            } else {
                h.iter()
                    .map(|&c| card_text(c))
                    .collect::<Vec<_>>()
                    .join(" ")
            }
        ));
        if self.phase == Phase::Discard {
            s.push_str(&format!(
                "Pune jos {} cărți (ecart).\n",
                6 - self.extra[self.taker.idx()].len()
            ));
        }
        if self.phase == Phase::HandOver {
            if let Some(t) = &self.last_hand {
                s.push_str(&format!("{t}. Scrie «urmatoarea».\n"));
            }
        }
        match self.outcome() {
            Some(Outcome::Win(p)) => s.push_str(&format!(
                "Meci încheiat: Jucător {} câștigă ({}–{}).\n",
                p.idx() + 1,
                self.total[0],
                self.total[1]
            )),
            Some(Outcome::Draw) => s.push_str("Meci încheiat: remiză.\n"),
            None => {
                if matches!(self.phase, Phase::Bid | Phase::Discard | Phase::Play) {
                    s.push_str(&format!("La mutare: Jucător {}\n", self.side.idx() + 1));
                }
            }
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Bid(l) => BID_NAMES[*l as usize].into(),
            Move::Dog => "chien".into(),
            Move::Discard(c) => format!("ecart {}", card_text(*c)),
            Move::Play(c) => card_text(*c),
            Move::Redeal => "reimparte".into(),
            Move::Next => "urmatoarea".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(suit: u8, r: u8) -> u8 {
        suit * 14 + r
    }
    fn t(v: u8) -> u8 {
        55 + v
    }

    #[test]
    fn deck_is_worth_91_points() {
        let total: i32 = (0..78).map(card_half_points).sum();
        assert_eq!(total, 182);
        assert!(is_oudler(t(1)) && is_oudler(t(21)) && is_oudler(EXCUSE));
        assert_eq!(card_half_points(s(0, 13)), 9);
        assert_eq!(card_half_points(s(0, 12)), 7);
        assert_eq!(card_half_points(s(0, 3)), 1);
    }

    #[test]
    fn deal_is_consistent() {
        let g = Tarot::with_seed(2);
        let mut all: Vec<u8> = g.hands[0]
            .iter()
            .chain(&g.hands[1])
            .chain(&g.dog)
            .copied()
            .collect();
        all.sort_unstable();
        assert_eq!(all, (0..78).collect::<Vec<u8>>());
        assert_eq!(g.dog.len(), 6);
    }

    #[test]
    fn trick_winner_rules() {
        assert!(second_wins(s(0, 5), s(0, 9)));
        assert!(!second_wins(s(0, 9), s(0, 5)));
        assert!(!second_wins(s(0, 5), s(1, 13))); // altă culoare
        assert!(second_wins(s(0, 13), t(2))); // atu taie
        assert!(second_wins(t(5), t(9)));
        assert!(!second_wins(t(9), t(5)));
        assert!(!second_wins(t(9), s(0, 13)));
    }

    fn base() -> Tarot {
        let mut g = Tarot::with_seed(1);
        g.hands = [Vec::new(), Vec::new()];
        g.dog.clear();
        g.phase = Phase::Play;
        g.taker = Player::One;
        g.level = 2;
        g.side = Player::One;
        g.led = None;
        g
    }

    #[test]
    fn follow_rules() {
        let mut g = base();
        g.side = Player::Two;
        g.led = Some(s(0, 5));
        g.hands[1] = vec![s(0, 1), s(1, 13), t(3), EXCUSE];
        let mut p = g.playable(Player::Two);
        p.sort_unstable();
        assert_eq!(p, vec![s(0, 1), EXCUSE]); // culoare + scuza
        g.hands[1] = vec![s(1, 13), t(3), t(8), EXCUSE];
        let mut p = g.playable(Player::Two);
        p.sort_unstable();
        assert_eq!(p, vec![t(3), t(8), EXCUSE]); // fără culoare: taie
        g.hands[1] = vec![s(1, 13), s(2, 4)];
        assert_eq!(g.playable(Player::Two).len(), 2);
        // atu condus: urci dacă poți
        g.led = Some(t(10));
        g.hands[1] = vec![t(3), t(12), s(1, 1)];
        assert_eq!(g.playable(Player::Two), vec![t(12)]);
        g.hands[1] = vec![t(3), t(4), s(1, 1)];
        assert_eq!(g.playable(Player::Two).len(), 2);
        g.hands[1] = vec![s(3, 3), s(1, 1)];
        assert_eq!(g.playable(Player::Two).len(), 2);
        // scuza condusă: răspunzi cu orice
        g.led = Some(EXCUSE);
        g.hands[1] = vec![s(3, 3), t(1)];
        assert_eq!(g.playable(Player::Two).len(), 2);
    }

    #[test]
    fn excuse_stays_with_its_player_and_does_not_win() {
        let mut g = base();
        g.hands[0] = vec![EXCUSE, s(0, 1)];
        g.hands[1] = vec![s(0, 13), s(1, 2)];
        g.apply(&Move::Play(EXCUSE));
        g.apply(&Move::Play(s(0, 13)));
        assert!(g.piles[0].contains(&EXCUSE));
        assert!(g.piles[1].contains(&s(0, 13)));
        assert_eq!(g.to_move(), Player::Two); // câștigătorul conduce
                                              // Scuza răspunsă: conducătorul ia levata, Scuza rămâne la cine a jucat-o
        let mut h = base();
        h.hands[0] = vec![s(0, 1), s(2, 2)];
        h.hands[1] = vec![EXCUSE, s(1, 2)];
        h.apply(&Move::Play(s(0, 1)));
        h.apply(&Move::Play(EXCUSE));
        assert!(h.piles[1].contains(&EXCUSE));
        assert!(h.piles[0].contains(&s(0, 1)));
        assert_eq!(h.to_move(), Player::One);
    }

    #[test]
    fn bidding_levels_and_dog() {
        let mut g = Tarot::with_seed(3);
        assert_eq!(g.to_move(), Player::One); // dealer = Two
        assert_eq!(g.legal_moves().len(), 5);
        g.apply(&Move::Bid(2)); // garde
        assert_eq!(g.to_move(), Player::Two);
        let lm = g.legal_moves();
        assert_eq!(lm, vec![Move::Bid(0), Move::Bid(3), Move::Bid(4)]);
        g.apply(&Move::Bid(0));
        assert_eq!(g.phase, Phase::Dog);
        assert_eq!(g.to_move(), Player::One);
        g.apply(&Move::Dog);
        assert_eq!(g.hands[0].len(), 42);
        assert_eq!(g.phase, Phase::Discard);
        for _ in 0..6 {
            let m = g.legal_moves();
            assert!(!m.is_empty());
            // fără regi și oudlers
            for x in &m {
                if let Move::Discard(c) = x {
                    assert!(!is_king(*c) && !is_oudler(*c));
                }
            }
            g.apply(&m[0]);
        }
        assert_eq!(g.hands[0].len(), 36);
        assert_eq!(g.extra[0].len(), 6);
        assert_eq!(g.phase, Phase::Play);
        assert_eq!(g.to_move(), Player::One);
    }

    #[test]
    fn garde_sans_and_contre_route_the_dog() {
        let mut g = Tarot::with_seed(4);
        let dog = g.dog.clone();
        g.apply(&Move::Bid(3));
        g.apply(&Move::Bid(0));
        g.apply(&Move::Dog);
        assert_eq!(g.extra[0], dog);
        assert_eq!(g.phase, Phase::Play);
        let mut h = Tarot::with_seed(4);
        h.apply(&Move::Bid(4));
        h.apply(&Move::Bid(0));
        h.apply(&Move::Dog);
        assert_eq!(h.extra[1], dog);
        assert_eq!(h.hands[0].len(), 36);
    }

    #[test]
    fn both_pass_means_redeal() {
        let mut g = Tarot::with_seed(5);
        g.apply(&Move::Bid(0));
        g.apply(&Move::Bid(0));
        assert_eq!(g.legal_moves(), vec![Move::Redeal]);
        g.apply(&Move::Redeal);
        assert_eq!(g.hands_done, 1);
        assert_eq!(g.dealer, Player::One);
    }

    #[test]
    fn scoring_contract_and_petit_au_bout() {
        // taker are 1 oudler: cerut 51 (102 jumătăți); are 60 -> diferență 9
        let mut g = base();
        g.level = 1;
        g.piles[0] = vec![EXCUSE, s(0, 13), s(1, 13), s(2, 13), s(3, 13)];
        for sv in 0..4u8 {
            g.piles[0].push(s(sv, 12));
            g.piles[0].push(s(sv, 11));
        }
        // 45 + 28 + 20 = 93 jumătăți; completăm cu 27 de cărți de 1 jumătate
        let fill: Vec<u8> = (0..78u8)
            .filter(|&c| !g.piles[0].contains(&c) && card_half_points(c) == 1)
            .take(27)
            .collect();
        g.piles[0].extend(fill);
        g.piles[1] = vec![t(21)];
        g.petit_bonus = 0;
        g.finish_hand();
        let tp: i32 = g.piles[0].iter().map(|&c| card_half_points(c)).sum();
        assert_eq!(tp, 120);
        // 120 - 102 = 18 jum. = 9 puncte; (25+9) * 1 = 34
        assert_eq!(g.total[0], 34);
        assert_eq!(g.total[1], -34);
        // contract căzut, garde (x2), petit au bout al apărării (-10)
        let mut h = base();
        h.level = 2;
        h.piles[0] = vec![s(0, 1), s(0, 2)];
        h.petit_bonus = -10;
        h.finish_hand();
        // 2 jum. contra 112 jum.: diferență 110 jum. = 55 puncte; (-(25+55) - 10) * 2 = -180
        assert_eq!(h.total[0], -180);
        assert_eq!(h.total[1], 180);
    }

    #[test]
    fn petit_au_bout_in_last_trick() {
        let mut g = base();
        g.trick_no = 35;
        g.piles[0] = (0..10).collect();
        g.hands[0] = vec![t(1)];
        g.hands[1] = vec![s(0, 3)];
        g.apply(&Move::Play(t(1)));
        g.apply(&Move::Play(s(0, 3))); // petit câștigă (atu bate culoare)
        assert_eq!(g.petit_bonus, 10);
        assert_eq!(g.phase, Phase::HandOver);
    }

    #[test]
    fn full_hands_conserve_cards_and_finish() {
        for seed in 1..=12u64 {
            let mut g = Tarot::with_seed(seed);
            let mut rng = Rng::new(seed ^ 0x7);
            let mut n = 0;
            while g.outcome().is_none() {
                if g.phase == Phase::HandOver {
                    let all: usize =
                        g.piles[0].len() + g.piles[1].len() + g.extra[0].len() + g.extra[1].len();
                    assert_eq!(all, 78);
                    let tot: i32 = g
                        .piles
                        .iter()
                        .chain(&g.extra)
                        .flatten()
                        .map(|&c| card_half_points(c))
                        .sum();
                    assert_eq!(tot, 182);
                }
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 20000);
            }
            assert_eq!(g.total[0] + g.total[1], 0);
        }
    }

    #[test]
    fn render_hides_the_opponent() {
        let g = Tarot::with_seed(5);
        assert!(Game::render(&g).contains("Jucător 2 (ascuns) are 36 cărți"));
    }

    #[test]
    fn viewer_controls_whose_hand_is_shown() {
        let hand_txt = |g: &Tarot, p: usize| {
            let mut h = g.hands[p].clone();
            h.sort_unstable();
            h.iter()
                .map(|&c| card_text(c))
                .collect::<Vec<_>>()
                .join(" ")
        };
        let mut g = Tarot::with_seed(5);
        Game::set_viewer(&mut g, None);
        g.apply(&Move::Bid(0));
        assert_eq!(g.to_move(), Player::Two);
        let r = Game::render(&g);
        assert!(r.contains(&hand_txt(&g, 1)), "{r}");
        assert!(!r.contains(&hand_txt(&g, 0)), "{r}");
        Game::set_viewer(&mut g, Some(Player::One));
        let r = Game::render(&g);
        assert!(
            r.contains(&hand_txt(&g, 0)) && !r.contains(&hand_txt(&g, 1)),
            "{r}"
        );
        Game::set_viewer(&mut g, Some(Player::Two));
        let r = Game::render(&g);
        assert!(
            r.contains(&hand_txt(&g, 1)) && !r.contains(&hand_txt(&g, 0)),
            "{r}"
        );
    }

    #[test]
    fn evaluate_ignores_the_opponents_cards() {
        let mut g = Tarot::with_seed(9);
        g.apply(&Move::Bid(1));
        g.apply(&Move::Bid(0));
        let mut h = g.clone();
        let (a, b) = (h.hands[1][0], h.dog[0]);
        h.hands[1][0] = b;
        h.dog[0] = a;
        assert_eq!(g.evaluate(Player::One), h.evaluate(Player::One));
    }

    #[test]
    fn ai_beats_a_random_mover_mostly() {
        use polytabletop_core::GameObj;
        let mut wins = 0;
        let n = 100;
        for seed in 1..=n {
            let mut g: Box<dyn GameObj> = Box::new(Tarot::with_seed(seed));
            let mut rng = Rng::new(seed + 100);
            while g.outcome().is_none() {
                let mv = if g.to_move() == Player::One {
                    g.ai_move(rng.next_u64()).unwrap()
                } else {
                    let m = g.legal_moves();
                    m[rng.below(m.len())].clone()
                };
                g.play(&mv).unwrap();
            }
            if g.outcome() == Some(Outcome::Win(Player::One)) {
                wins += 1;
            }
        }
        assert!(wins * 10 >= n * 7, "AI won only {wins}/{n}");
    }
}
