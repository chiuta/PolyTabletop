//! Tressette (Italia), 2 jucători, din regulile publice ale variantei „a due”.
//!
//! * Pachet italian de 40 de cărți (denari, coppe, spade, bastoni). Ordinea (de la cea mai mare): 3, 2, As,
//!   Re, Cavallo, Fante, 7, 6, 5, 4. Nu există atu.
//! * Valori (în treimi de punct): As 3, iar 2, 3, Re, Cavallo, Fante câte 1; ultima levată +3 (adică 1 punct);
//!   restul 0. Un joc se ține până când un jucător adună 21 de puncte (63 de treimi) la sfârșitul unei mâini.
//! * Fiecare primește 10 cărți; restul (20) formează stocul. Câștigătorul unei levate trage primul din stoc,
//!   apoi celălalt, și conduce următoarea. **Culoarea se răspunde obligatoriu doar după ce stocul s-a golit**;
//!   până atunci poți arunca orice. Levata o ia cea mai mare carte din culoarea condusă.
//! * Nu sunt implementate acuzele (napoletana etc.) și nici punctajul în „zeci” pe tabelă.
//!
//! Mutări: `3d`, `Rs`, `Cc` (rang: 3 2 A R C F 7 6 5 4; culoare: d denari, c coppe, s spade, b bastoni),
//! `urmatoarea` pentru mâna următoare. Informație ascunsă: vezi `p3_comun`.
#![allow(clippy::needless_range_loop)]

use crate::cards::{handover_banner, Viewer};
use crate::p3_comun::{honest_eval, Honest};
use crate::q6_util::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const TARGET: i32 = 63;
// putere: 0 = 4 … 9 = 3
const LABEL: [&str; 10] = ["4", "5", "6", "7", "F", "C", "R", "A", "2", "3"];
const THIRDS: [i32; 10] = [0, 0, 0, 0, 1, 1, 1, 3, 1, 1];
const SUITS: [char; 4] = ['d', 'c', 's', 'b'];

fn rank(c: u8) -> u8 {
    c % 10
}
fn suit(c: u8) -> u8 {
    c / 10
}
fn ctext(c: u8) -> String {
    format!("{}{}", LABEL[rank(c) as usize], SUITS[suit(c) as usize])
}
fn pts(c: u8) -> i32 {
    THIRDS[rank(c) as usize]
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Play,
    HandOver,
    Over,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Play(u8),
    Next,
}

#[derive(Clone)]
pub struct Tressette {
    rng: Rng,
    hands: [Vec<u8>; 2],
    stock: Vec<u8>,
    led: Option<u8>,
    side: Player,
    first: Player,
    taken: [i32; 2],
    total: [i32; 2],
    gain: [i32; 2],
    phase: Phase,
    played: Vec<u8>,
    fresh: [u8; 2],
    prev: Player,
    log: String,
    viewer: Viewer,
}

impl Default for Tressette {
    fn default() -> Self {
        Self::new()
    }
}

impl Tressette {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Tressette {
            rng: Rng::new(seed),
            hands: [Vec::new(), Vec::new()],
            stock: Vec::new(),
            led: None,
            side: Player::One,
            first: Player::One,
            taken: [0, 0],
            total: [0, 0],
            gain: [0, 0],
            phase: Phase::Play,
            played: Vec::new(),
            fresh: [0, 0],
            prev: Player::One,
            log: String::new(),
            viewer: Viewer::Fixed(Player::One),
        };
        g.deal();
        g
    }

    pub fn totals(&self) -> (i32, i32) {
        (self.total[0], self.total[1])
    }

    fn deal(&mut self) {
        let mut d: Vec<u8> = (0..40).collect();
        shuffle(&mut d, &mut self.rng);
        for p in 0..2 {
            self.hands[p] = (0..10).map(|_| d.pop().unwrap()).collect();
            self.hands[p].sort_unstable();
        }
        self.stock = d;
        self.led = None;
        self.taken = [0, 0];
        self.gain = [0, 0];
        self.played.clear();
        self.fresh = [0, 0];
        self.phase = Phase::Play;
        self.side = self.first;
    }

    fn follow_required(&self) -> bool {
        self.stock.is_empty()
    }
}

impl Game for Tressette {
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
            Phase::HandOver => vec![Move::Next],
            Phase::Play => {
                let h = &self.hands[self.side.idx()];
                if let (Some(l), true) = (self.led, self.follow_required()) {
                    if h.iter().any(|&c| suit(c) == suit(l)) {
                        return h
                            .iter()
                            .copied()
                            .filter(|&c| suit(c) == suit(l))
                            .map(Move::Play)
                            .collect();
                    }
                }
                h.iter().copied().map(Move::Play).collect()
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        self.fresh = [0, 0];
        self.prev = self.side;
        match *m {
            Move::Next => {
                self.first = self.first.other();
                self.deal();
            }
            Move::Play(c) => {
                let p = self.side;
                let pos = self.hands[p.idx()].iter().position(|&x| x == c).unwrap();
                self.hands[p.idx()].remove(pos);
                self.played.push(c);
                match self.led {
                    None => {
                        self.led = Some(c);
                        self.side = p.other();
                    }
                    Some(l) => {
                        let lead_player = p.other();
                        let win_resp = suit(c) == suit(l) && rank(c) > rank(l);
                        let winner = if win_resp { p } else { lead_player };
                        let mut pt = pts(c) + pts(l);
                        let last = self.hands[0].is_empty() && self.hands[1].is_empty();
                        if last {
                            pt += 3;
                        }
                        self.taken[winner.idx()] += pt;
                        self.log = format!(
                            "{} vs {} → J{} ia levata",
                            ctext(l),
                            ctext(c),
                            winner.idx() + 1
                        );
                        self.led = None;
                        if !self.stock.is_empty() {
                            let a = self.stock.pop().unwrap();
                            let b = self.stock.pop().unwrap();
                            self.hands[winner.idx()].push(a);
                            self.hands[1 - winner.idx()].push(b);
                            self.hands[0].sort_unstable();
                            self.hands[1].sort_unstable();
                            // marcăm cărțile proaspete drept ultimele n (numărătoare)
                            self.fresh[winner.idx()] = 1;
                            self.fresh[1 - winner.idx()] = 1;
                            // pentru determinizare vrem cărțile proaspete la sfârșit
                            let (ia, ib) = (a, b);
                            for (pl, cd) in [(winner.idx(), ia), (1 - winner.idx(), ib)] {
                                let k = self.hands[pl].iter().position(|&x| x == cd).unwrap();
                                let x = self.hands[pl].remove(k);
                                self.hands[pl].push(x);
                            }
                        }
                        self.side = winner;
                        if last {
                            for pl in 0..2 {
                                self.total[pl] += self.taken[pl];
                                self.gain[pl] = self.taken[pl];
                            }
                            self.phase = if self.total[0] >= TARGET || self.total[1] >= TARGET {
                                Phase::Over
                            } else {
                                Phase::HandOver
                            };
                            self.log.push_str(&format!(
                                " — mână încheiată: {}–{} treimi",
                                self.taken[0], self.taken[1]
                            ));
                        }
                    }
                }
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
        if self.phase == Phase::Over {
            return 0;
        }
        honest_eval(self, p, 14)
    }

    fn render(&self) -> String {
        let mut s = String::new();
        if self.prev != self.side && self.phase == Phase::Play {
            if let Viewer::HotSeat = self.viewer {
                s.push_str(&handover_banner(self.side));
            }
        }
        s.push_str(&format!(
            "Tressette — total (puncte = treimi/3): J1 {} ({}), J2 {} ({}); țintă 21\n",
            self.total[0],
            self.total[0] / 3,
            self.total[1],
            self.total[1] / 3
        ));
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        match self.phase {
            Phase::Over => {
                s.push_str("Joc încheiat.\n");
                return s;
            }
            Phase::HandOver => {
                s.push_str("Mâna s-a încheiat. Scrie «urmatoarea».\n");
                return s;
            }
            Phase::Play => {}
        }
        s.push_str(&format!(
            "Stoc: {} cărți | luat în mână: J1 {}, J2 {} treimi\n",
            self.stock.len(),
            self.taken[0],
            self.taken[1]
        ));
        if let Some(l) = self.led {
            s.push_str(&format!("Pe masă: {}\n", ctext(l)));
        }
        let v = self.viewer.who(self.side);
        s.push_str(&format!(
            "Mâna ta (J{}): {}\n",
            v.idx() + 1,
            self.hands[v.idx()]
                .iter()
                .map(|&c| ctext(c))
                .collect::<Vec<_>>()
                .join(" ")
        ));
        s.push_str(&format!(
            "J{} are {} cărți\n",
            v.other().idx() + 1,
            self.hands[v.other().idx()].len()
        ));
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Next => "urmatoarea".into(),
            Move::Play(c) => ctext(*c),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

impl Honest for Tressette {
    fn determinize(&self, me: Player, rng: &mut Rng) -> Self {
        let mut g = self.clone();
        let mi = me.idx();
        let oi = 1 - mi;
        let nf = (self.fresh[mi] as usize).min(self.hands[mi].len());
        let known: Vec<u8> = self.hands[mi][..self.hands[mi].len() - nf].to_vec();
        let mut pool: Vec<u8> = (0..40).collect();
        pool.retain(|c| !known.contains(c) && !self.played.contains(c) && Some(*c) != self.led);
        shuffle(&mut pool, rng);
        let n_opp = self.hands[oi].len();
        let mut mine = known;
        for _ in 0..nf {
            mine.push(pool.pop().unwrap());
        }
        g.hands[mi] = mine;
        g.hands[oi] = (0..n_opp).map(|_| pool.pop().unwrap()).collect();
        g.stock = pool;
        g.fresh = [0, 0];
        g.rng = Rng::new(rng.next_u64());
        g
    }

    fn seed_state(&self, me: Player) -> Self {
        let mut c = self.clone();
        let nf = (c.fresh[me.idx()] as usize).min(c.hands[me.idx()].len());
        let k = c.hands[me.idx()].len() - nf;
        c.hands[me.idx()].truncate(k);
        c.fresh = [0, 0];
        c
    }

    fn forget_fresh(&mut self) {
        self.fresh = [0, 0];
    }

    fn rollout_done(&self) -> bool {
        self.phase != Phase::Play
    }

    fn rollout_value(&self, p: Player) -> f64 {
        let q = p.other();
        if self.phase == Phase::Play {
            return (self.taken[p.idx()] - self.taken[q.idx()]) as f64;
        }
        (self.gain[p.idx()] - self.gain[q.idx()]) as f64
    }

    fn policy(&self, rng: &mut Rng) -> Move {
        let mv = self.legal_moves();
        if mv.len() == 1 || rng.below(100) < 15 {
            return mv[rng.below(mv.len())];
        }
        // regula simplă: dacă răspunzi, ia levata cu cea mai mică carte câștigătoare; dacă conduci, joacă cea mai mică
        let mut best = mv[0];
        let mut bs = i32::MIN;
        for &m in &mv {
            if let Move::Play(c) = m {
                let mut sc = -(rank(c) as i32) * 2 - pts(c) * 3;
                if let Some(l) = self.led {
                    if suit(c) == suit(l) && rank(c) > rank(l) {
                        sc += 40 + pts(l) * 5 - rank(c) as i32;
                    }
                }
                if sc > bs {
                    bs = sc;
                    best = m;
                }
            }
        }
        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_points_per_hand() {
        // 4 A (12) + 5×4 figure (20) = 32 treimi + 3 ultima levată = 35? verificăm numărând
        let sum: i32 = (0..40u8).map(pts).sum();
        assert_eq!(sum, 12 + 20);
    }

    #[test]
    fn follow_only_after_stock_empty() {
        let mut g = Tressette::with_seed(2);
        g.hands[0] = vec![0, 1, 10]; // 4d 5d 4c
        g.hands[1] = vec![2, 11];
        g.led = Some(3); // 7d
        g.side = Player::Two;
        g.stock = vec![5, 6];
        assert_eq!(g.legal_moves().len(), 2); // orice
        g.stock.clear();
        // J2 are 6d (2) și 5c (11): trebuie să răspundă la denari
        let m = g.legal_moves();
        assert_eq!(m, vec![Move::Play(2)]);
    }

    #[test]
    fn higher_card_of_led_suit_wins() {
        let mut g = Tressette::with_seed(2);
        g.stock.clear();
        g.hands[0] = vec![9, 0]; // 3d, 4d
        g.hands[1] = vec![8, 1]; // 2d, 5d
        g.side = Player::One;
        g.led = None;
        g.apply(&Move::Play(9));
        g.apply(&Move::Play(8));
        assert_eq!(g.taken[0], 2); // 3d + 2d = 1+1
        assert_eq!(g.side, Player::One);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=4u64 {
            let mut g = Tressette::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 5000);
            }
        }
    }
}
