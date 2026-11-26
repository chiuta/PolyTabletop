//! Texas 42 — variantă pentru doi jucători (jocul de levate cu domino din Texas).
//!
//! 28 de piese; fiecare jucător primește 7 (celelalte 14 stau deoparte, nevăzute). Licitație într-o singură rundă (30–42):
//! cel care nu a dat împărțirea licitează sau pasează, apoi celălalt licitează mai sus sau pasează (dacă primul a pasat,
//! al doilea e obligat să liciteze). Câștigătorul licitației alege atuul (0–6) și conduce.
//! * Levata: piesa condusă fixează culoarea (capătul mai mare; dacă are atu, culoarea e atu); trebuie să răspunzi în
//!   culoare dacă poți. Piesele cu capătul atu sunt atu. Dublele sunt cele mai mari din culoarea lor.
//! * Puncte: 7 levate × 1, plus 5-5 și 6-4 (10 fiecare), 5-0, 4-1 și 3-2 (5 fiecare) = 42 în total.
//! * Dacă licitatorul face cel puțin cât a licitat, câștigă un semn; altfel semnul merge adversarului. Primul la 4 semne.
//!
//! Omise: jocul fără atu / cu duble atu, nello, plunge, splash. Informație ascunsă: mâinile.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const MARKS: u8 = 4;

/// Capetele (mare, mic) ale piesei `d` (0..28).
pub fn ends(d: u8) -> (u8, u8) {
    let mut hi = 0u8;
    while (hi + 1) * (hi + 2) / 2 <= d {
        hi += 1;
    }
    (hi, d - hi * (hi + 1) / 2)
}
#[cfg(test)]
fn id(hi: u8, lo: u8) -> u8 {
    hi * (hi + 1) / 2 + lo
}
pub fn text(d: u8) -> String {
    let (h, l) = ends(d);
    format!("{h}-{l}")
}
pub fn count(d: u8) -> i32 {
    let (h, l) = ends(d);
    match (h, l) {
        (5, 5) | (6, 4) => 10,
        (5, 0) | (4, 1) | (3, 2) => 5,
        _ => 0,
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mv {
    Pass,
    Bid(u8),
    Trump(u8),
    Play(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Bid1,
    Bid2,
    Trump,
    Play,
    Over,
}

#[derive(Clone)]
pub struct Texas42 {
    viewer: Option<Player>,
    hands: [Vec<u8>; 2],
    phase: Phase,
    bid: u8,
    bidder: usize,
    trump: u8,
    trick: Vec<(usize, u8)>,
    leader: usize,
    pts: [i32; 2],
    dealer: usize,
    marks: [u8; 2],
    rng: Rng,
    log: String,
}

impl Default for Texas42 {
    fn default() -> Self {
        Self::new()
    }
}

impl Texas42 {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut g = Texas42 {
            viewer: None,
            hands: [Vec::new(), Vec::new()],
            phase: Phase::Bid1,
            bid: 0,
            bidder: 0,
            trump: 0,
            trick: Vec::new(),
            leader: 0,
            pts: [0; 2],
            dealer: 1,
            marks: [0; 2],
            rng: Rng::new(seed),
            log: String::new(),
        };
        g.deal();
        g
    }
    pub fn marks(&self) -> (u8, u8) {
        (self.marks[0], self.marks[1])
    }
    fn deal(&mut self) {
        self.dealer = 1 - self.dealer;
        let mut d: Vec<u8> = (0..28).collect();
        for i in (1..28).rev() {
            let j = self.rng.below(i + 1);
            d.swap(i, j);
        }
        self.hands = [d[0..7].to_vec(), d[7..14].to_vec()];
        self.phase = Phase::Bid1;
        self.bid = 0;
        self.trick.clear();
        self.pts = [0; 2];
    }
    fn first(&self) -> usize {
        1 - self.dealer
    }
    fn actor(&self) -> usize {
        match self.phase {
            Phase::Bid1 => self.first(),
            Phase::Bid2 => self.dealer,
            Phase::Trump => self.bidder,
            Phase::Play => (self.leader + self.trick.len()) % 2,
            Phase::Over => 0,
        }
    }
    fn is_trump(&self, d: u8) -> bool {
        let (h, l) = ends(d);
        h == self.trump || l == self.trump
    }
    /// Culoarea piesei când e condusă: atu sau capătul mare.
    fn led_suit(&self, d: u8) -> u8 {
        if self.is_trump(d) {
            self.trump
        } else {
            ends(d).0
        }
    }
    fn has_suit(&self, d: u8, s: u8) -> bool {
        if self.is_trump(d) {
            s == self.trump
        } else {
            let (h, l) = ends(d);
            h == s || l == s
        }
    }
    fn rank_in(&self, d: u8, s: u8) -> u8 {
        let (h, l) = ends(d);
        if h == l {
            return 7;
        }
        if h == s {
            l
        } else {
            h
        }
    }
    fn beats(&self, a: u8, b: u8, led: u8) -> bool {
        let (ta, tb) = (self.is_trump(a), self.is_trump(b));
        if ta != tb {
            return ta;
        }
        let s = if ta { self.trump } else { led };
        let (ha, hb) = (self.has_suit(a, s), self.has_suit(b, s));
        if ha != hb {
            return ha;
        }
        ha && self.rank_in(a, s) > self.rank_in(b, s)
    }
    fn legal_plays(&self, p: usize) -> Vec<u8> {
        let h = &self.hands[p];
        if let Some(&(_, d0)) = self.trick.first() {
            let s = self.led_suit(d0);
            let same: Vec<u8> = h.iter().copied().filter(|&d| self.has_suit(d, s)).collect();
            if !same.is_empty() {
                return same;
            }
        }
        h.clone()
    }
    fn end_hand(&mut self) {
        let b = self.bidder;
        let made = self.pts[b] >= self.bid as i32;
        let w = if made { b } else { 1 - b };
        self.marks[w] += 1;
        self.log = format!(
            "J{} licitase {}: a făcut {} puncte. Semn pentru J{}.",
            b + 1,
            self.bid,
            self.pts[b],
            w + 1
        );
        if self.marks[w] >= MARKS {
            self.phase = Phase::Over;
        } else {
            let keep = std::mem::take(&mut self.log);
            self.deal();
            self.log = keep;
        }
    }
}

impl Game for Texas42 {
    type Move = Mv;
    fn to_move(&self) -> Player {
        if self.actor() == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn legal_moves(&self) -> Vec<Mv> {
        let p = self.actor();
        match self.phase {
            Phase::Over => Vec::new(),
            Phase::Bid1 => {
                let mut v: Vec<Mv> = (30..=42).map(Mv::Bid).collect();
                v.push(Mv::Pass);
                v
            }
            Phase::Bid2 => {
                if self.bid == 0 {
                    (30..=42).map(Mv::Bid).collect()
                } else {
                    let mut v: Vec<Mv> = (self.bid + 1..=42).map(Mv::Bid).collect();
                    v.push(Mv::Pass);
                    v
                }
            }
            Phase::Trump => (0..7).map(Mv::Trump).collect(),
            Phase::Play => self.legal_plays(p).into_iter().map(Mv::Play).collect(),
        }
    }
    fn apply(&mut self, m: &Mv) {
        let p = self.actor();
        match (*m, self.phase) {
            (Mv::Pass, Phase::Bid1) => {
                self.phase = Phase::Bid2;
                self.log = format!("J{} pasează.", p + 1);
            }
            (Mv::Bid(n), Phase::Bid1) => {
                self.bid = n;
                self.bidder = p;
                self.phase = Phase::Bid2;
                self.log = format!("J{} licitează {n}.", p + 1);
            }
            (Mv::Pass, Phase::Bid2) => {
                self.phase = Phase::Trump;
            }
            (Mv::Bid(n), Phase::Bid2) => {
                self.bid = n;
                self.bidder = p;
                self.phase = Phase::Trump;
            }
            (Mv::Trump(t), Phase::Trump) => {
                self.trump = t;
                self.leader = p;
                self.phase = Phase::Play;
                self.log = format!("J{} are licitația {} cu atu {t}.", p + 1, self.bid);
            }
            (Mv::Play(d), Phase::Play) => {
                let h = &mut self.hands[p];
                let i = h.iter().position(|&x| x == d).unwrap();
                h.remove(i);
                self.trick.push((p, d));
                if self.trick.len() == 2 {
                    let (a, b) = (self.trick[0], self.trick[1]);
                    let led = self.led_suit(a.1);
                    let w = if self.beats(b.1, a.1, led) { b.0 } else { a.0 };
                    self.pts[w] += 1 + count(a.1) + count(b.1);
                    self.trick.clear();
                    self.leader = w;
                    if self.hands[0].is_empty() {
                        self.end_hand();
                    }
                }
            }
            _ => {}
        }
    }
    fn outcome(&self) -> Option<Outcome> {
        if self.phase == Phase::Over {
            Some(if self.marks[0] > self.marks[1] {
                Outcome::Win(Player::One)
            } else {
                Outcome::Win(Player::Two)
            })
        } else {
            None
        }
    }
    fn evaluate(&self, pl: Player) -> i32 {
        let me = pl.idx();
        let mut v = 1000 * (self.marks[me] as i32 - self.marks[1 - me] as i32);
        match self.phase {
            Phase::Play | Phase::Trump => {
                v += 4 * (self.pts[me] - self.pts[1 - me]);
                let t = self.trump;
                let tr = |d: u8| self.phase == Phase::Play && (ends(d).0 == t || ends(d).1 == t);
                v += self.hands[me]
                    .iter()
                    .map(|&d| (ends(d).0 + ends(d).1) as i32 + if tr(d) { 6 } else { 0 } + count(d))
                    .sum::<i32>();
                if self.bidder == me {
                    v += 3;
                }
            }
            Phase::Bid1 | Phase::Bid2 => {
                v += self.hands[me]
                    .iter()
                    .map(|&d| (ends(d).0 + ends(d).1) as i32 + count(d))
                    .sum::<i32>()
                    / 3;
            }
            Phase::Over => {}
        }
        v
    }
    fn render(&self) -> String {
        let view = self.viewer.unwrap_or(self.to_move()).idx();
        let mut s = format!(
            "Semne: J1 {} – {} J2 (până la {MARKS}).\n",
            self.marks[0], self.marks[1]
        );
        if self.phase != Phase::Over {
            let mut h = self.hands[view].clone();
            h.sort();
            s.push_str(&format!(
                "Mâna ta (J{}): {}\n",
                view + 1,
                h.iter().map(|&d| text(d)).collect::<Vec<_>>().join(" ")
            ));
            s.push_str(&format!(
                "Adversarul are {} piese.\n",
                self.hands[1 - view].len()
            ));
            if self.phase == Phase::Play {
                s.push_str(&format!(
                    "Licitație: {} (J{}), atu {}. Puncte: J1 {}, J2 {}.\n",
                    self.bid,
                    self.bidder + 1,
                    self.trump,
                    self.pts[0],
                    self.pts[1]
                ));
            }
            if !self.trick.is_empty() {
                s.push_str(&format!(
                    "Pe masă: {}\n",
                    self.trick
                        .iter()
                        .map(|&(p, d)| format!("J{}:{}", p + 1, text(d)))
                        .collect::<Vec<_>>()
                        .join(" ")
                ));
            }
        }
        s.push_str(self.log.trim());
        if self.phase != Phase::Over {
            s.push_str(&format!("\nLa mutare: Jucătorul {}.", self.actor() + 1));
        }
        s
    }
    fn format_move(&self, m: &Mv) -> String {
        match *m {
            Mv::Pass => "pas".into(),
            Mv::Bid(n) => format!("licitez {n}"),
            Mv::Trump(t) => format!("atu {t}"),
            Mv::Play(d) => text(d),
        }
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q4_util::*;
    use polytabletop_core::{GameObj, Player};

    #[test]
    fn dominoes_and_counts() {
        assert_eq!((0..28u8).map(count).sum::<i32>(), 35);
        assert_eq!(ends(id(6, 4)), (6, 4));
        assert_eq!(ends(27), (6, 6));
    }

    #[test]
    fn follow_and_trump_win() {
        let mut g = Texas42::with_seed(1);
        g.phase = Phase::Play;
        g.trump = 2;
        g.bidder = 0;
        g.bid = 30;
        g.leader = 0;
        g.hands = [vec![id(5, 3), id(1, 0)], vec![id(4, 3), id(2, 0), id(5, 5)]];
        g.apply(&Mv::Play(id(5, 3))); // culoare 5
        let m = GameObj::legal_moves(&g);
        assert_eq!(m, vec!["5-5"], "{m:?}");
        g.apply(&Mv::Play(id(5, 5)));
        assert_eq!(g.pts[1], 1 + 10);
        g.leader = 0;
        g.hands = [vec![id(6, 1)], vec![id(2, 0)]];
        g.trick.clear();
        g.apply(&Mv::Play(id(6, 1)));
        g.apply(&Mv::Play(id(2, 0))); // atu bate
        assert_eq!(g.marks, [0, 1]); // licitatorul nu a făcut 30
    }

    #[test]
    fn viewer_hides_hand() {
        let g = Texas42::with_seed(3);
        let mut a = g.clone_box();
        a.set_viewer(Some(Player::One));
        let mut b = g.clone_box();
        b.set_viewer(Some(Player::Two));
        assert_ne!(a.render(), b.render());
    }

    #[test]
    fn random_and_ai_games_finish() {
        random_finishes(|s| Box::new(Texas42::with_seed(s)), 4, 3000);
        ai_vs_random(|s| Box::new(Texas42::with_seed(s)), 2, Player::One, 3000);
        ai_vs_random(|s| Box::new(Texas42::with_seed(s)), 3, Player::Two, 3000);
    }
}
