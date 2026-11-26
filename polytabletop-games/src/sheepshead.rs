//! Sheepshead (SUA / Germania) — variantă pentru doi jucători, din regulile publice.
//!
//! Pachet de 32 de cărți (7 8 9 10 J Q K A; culori c ♣, s ♠, h ♥, d ♦). **Atuuri** (14, de la cel mai mare):
//! Dame ♣ ♠ ♥ ♦, Valeți ♣ ♠ ♥ ♦, apoi caro A 10 K 9 8 7. Culorile „fail” (♣ ♠ ♥) au A 10 K 9 8 7.
//! Puncte: A 11, 10 10, K 4, Q 3, J 2 (total 120).
//!
//! * Împărțire: 10 cărți fiecăruia și 2 în *blind*; restul pachetului nu se joacă (adaptare pentru doi).
//! * Licitație: cine nu a împărțit spune `iau` (ia blindul) sau `pas`; la dublu pas se joacă „leaster” (fără atu-ul
//!   unui declarant; câștigă cine are mai puține puncte).
//! * Cine ia blindul îl adaugă mâinii și **îngroapă** două cărți (`ingrop Qc`); punctele îngropate sunt ale lui.
//! * Joc: 10 levate, se răspunde la culoarea cerută (atuul e o „culoare”); cine nu poate, joacă orice.
//!   Declarantul trebuie să strângă ≥ 61 de puncte. Scor: câștig 1 punct (adversar ≤ 30 = „schneider”, 2 puncte;
//!   adversar fără nicio levată = „schwarz”, 3), pierdere = adversarul primește la fel (după punctele declarantului).
//! * 6 mâini; câștigă cine are mai multe puncte (egal = remiză).
//!
//! Informație ascunsă: mâinile, blindul și cărțile îngropate. AI-ul citește doar propriile cărți; evaluarea ghidează
//! licitația după forța mâinii (atuuri, dame, valeți, ași) și jocul după punctele câștigate.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const RANKS: [&str; 8] = ["7", "8", "9", "10", "J", "Q", "K", "A"];
const SUITS: [char; 4] = ['c', 's', 'h', 'd'];
const HANDS: u8 = 6;

fn suit(c: u8) -> u8 {
    c / 8
}
fn rank(c: u8) -> u8 {
    c % 8
}
pub fn is_trump(c: u8) -> bool {
    rank(c) == 4 || rank(c) == 5 || suit(c) == 3
}
/// Putere: atuurile 7..=20, culorile fail 1..=6 (comparabile doar în aceeași culoare).
pub fn power(c: u8) -> u8 {
    if rank(c) == 5 {
        20 - suit(c)
    } else if rank(c) == 4 {
        16 - suit(c)
    } else if suit(c) == 3 {
        [7, 8, 9, 11, 0, 0, 10, 12][rank(c) as usize]
    } else {
        [1, 2, 3, 5, 0, 0, 4, 6][rank(c) as usize]
    }
}
pub fn points(c: u8) -> u8 {
    [0, 0, 0, 10, 2, 3, 4, 11][rank(c) as usize]
}
/// Culoarea cerută: 4 = atu, 0..2 = culori fail.
fn led(c: u8) -> u8 {
    if is_trump(c) {
        4
    } else {
        suit(c)
    }
}
pub fn text(c: u8) -> String {
    format!("{}{}", RANKS[rank(c) as usize], SUITS[suit(c) as usize])
}
fn strength(c: u8) -> i32 {
    if is_trump(c) {
        6 + power(c) as i32 / 2
    } else {
        power(c) as i32
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mv {
    Pick,
    Pass,
    Bury(u8),
    Play(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Bid(usize),
    Bury,
    Play,
    Over,
}

#[derive(Clone)]
pub struct Sheepshead {
    viewer: Option<Player>,
    hands: [Vec<u8>; 2],
    blind: Vec<u8>,
    buried: Vec<u8>,
    picker: Option<usize>,
    phase: Phase,
    trick: Vec<(usize, u8)>,
    leader: usize,
    taken: [i32; 2],
    won: [u8; 2],
    dealer: usize,
    hand_no: u8,
    scores: [i32; 2],
    rng: Rng,
    log: String,
}

impl Default for Sheepshead {
    fn default() -> Self {
        Self::new()
    }
}

impl Sheepshead {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Sheepshead {
            viewer: None,
            hands: [Vec::new(), Vec::new()],
            blind: Vec::new(),
            buried: Vec::new(),
            picker: None,
            phase: Phase::Over,
            trick: Vec::new(),
            leader: 0,
            taken: [0; 2],
            won: [0; 2],
            dealer: 1,
            hand_no: 0,
            scores: [0; 2],
            rng: Rng::new(seed),
            log: String::new(),
        };
        g.deal();
        g
    }

    pub fn scores(&self) -> (i32, i32) {
        (self.scores[0], self.scores[1])
    }

    fn deal(&mut self) {
        self.hand_no += 1;
        self.dealer = 1 - self.dealer;
        let mut d: Vec<u8> = (0..32).collect();
        for i in (1..32).rev() {
            let j = self.rng.below(i + 1);
            d.swap(i, j);
        }
        self.hands = [d[0..10].to_vec(), d[10..20].to_vec()];
        self.blind = d[20..22].to_vec();
        self.buried.clear();
        self.picker = None;
        self.trick.clear();
        self.taken = [0; 2];
        self.won = [0; 2];
        self.leader = 1 - self.dealer;
        self.phase = Phase::Bid(self.leader);
        self.log = format!(
            "Mâna {}/{HANDS}: J{} împarte.",
            self.hand_no,
            self.dealer + 1
        );
    }

    fn actor(&self) -> usize {
        match self.phase {
            Phase::Bid(p) => p,
            Phase::Bury => self.picker.unwrap(),
            _ => (self.leader + self.trick.len()) % 2,
        }
    }

    fn legal_cards(&self, p: usize) -> Vec<u8> {
        let h = &self.hands[p];
        if let Some(&(_, c0)) = self.trick.first() {
            let l = led(c0);
            let follow: Vec<u8> = h.iter().copied().filter(|&c| led(c) == l).collect();
            if !follow.is_empty() {
                return follow;
            }
        }
        h.clone()
    }

    fn hand_strength(h: &[u8]) -> i32 {
        h.iter().map(|&c| strength(c)).sum()
    }

    fn finish_hand(&mut self) {
        let mut msg = String::new();
        match self.picker {
            Some(p) => {
                let pts =
                    self.taken[p] + self.buried.iter().map(|&c| points(c) as i32).sum::<i32>();
                let opp = 120 - pts;
                if pts >= 61 {
                    let n = if self.won[1 - p] == 0 {
                        3
                    } else if opp <= 30 {
                        2
                    } else {
                        1
                    };
                    self.scores[p] += n;
                    msg = format!("J{} (declarant) face {pts} puncte și câștigă {n}.", p + 1);
                } else {
                    let n = if self.won[p] == 0 {
                        3
                    } else if pts <= 30 {
                        2
                    } else {
                        1
                    };
                    self.scores[1 - p] += n;
                    msg = format!(
                        "J{} (declarant) face doar {pts} puncte; J{} câștigă {n}.",
                        p + 1,
                        2 - p
                    );
                }
            }
            None => {
                // leaster: blindul merge la ultima levată câștigată (adăugat la cel cu mai multe puncte? simplu: celui care a luat ultima levată)
                if self.taken[0] < self.taken[1] {
                    self.scores[0] += 1;
                    msg.push_str("Leaster: J1 are mai puține puncte și câștigă 1.");
                } else if self.taken[1] < self.taken[0] {
                    self.scores[1] += 1;
                    msg.push_str("Leaster: J2 are mai puține puncte și câștigă 1.");
                } else {
                    msg.push_str("Leaster: egalitate.");
                }
            }
        }
        self.log = msg;
        if self.hand_no >= HANDS {
            self.phase = Phase::Over;
        } else {
            let keep = self.log.clone();
            self.deal();
            self.log = format!("{keep} {}", self.log);
        }
    }
}

impl Game for Sheepshead {
    type Move = Mv;
    fn to_move(&self) -> Player {
        if self.actor() == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn legal_moves(&self) -> Vec<Mv> {
        match self.phase {
            Phase::Over => Vec::new(),
            Phase::Bid(_) => vec![Mv::Pick, Mv::Pass],
            Phase::Bury => self.hands[self.actor()]
                .iter()
                .map(|&c| Mv::Bury(c))
                .collect(),
            Phase::Play => self
                .legal_cards(self.actor())
                .into_iter()
                .map(Mv::Play)
                .collect(),
        }
    }
    fn apply(&mut self, m: &Mv) {
        let p = self.actor();
        match (*m, self.phase) {
            (Mv::Pick, Phase::Bid(_)) => {
                self.picker = Some(p);
                let b = std::mem::take(&mut self.blind);
                self.hands[p].extend(b);
                self.phase = Phase::Bury;
                self.log = format!("J{} ia blindul și îngroapă două cărți.", p + 1);
            }
            (Mv::Pass, Phase::Bid(w)) => {
                if w == self.leader {
                    self.phase = Phase::Bid(1 - w);
                } else {
                    self.phase = Phase::Play;
                    self.log = "Amândoi pasează: leaster.".into();
                }
            }
            (Mv::Bury(c), Phase::Bury) => {
                let h = &mut self.hands[p];
                let i = h.iter().position(|&x| x == c).unwrap();
                h.remove(i);
                self.buried.push(c);
                if self.buried.len() == 2 {
                    self.phase = Phase::Play;
                }
            }
            (Mv::Play(c), Phase::Play) => {
                let h = &mut self.hands[p];
                let i = h.iter().position(|&x| x == c).unwrap();
                h.remove(i);
                self.trick.push((p, c));
                if self.trick.len() == 2 {
                    let (a, b) = (self.trick[0], self.trick[1]);
                    let winner = if led(b.1) == led(a.1) || is_trump(b.1) {
                        let (ta, tb) = (is_trump(a.1), is_trump(b.1));
                        if tb && !ta {
                            b.0
                        } else if ta && !tb {
                            a.0
                        } else if led(b.1) == led(a.1) && power(b.1) > power(a.1) {
                            b.0
                        } else {
                            a.0
                        }
                    } else {
                        a.0
                    };
                    let pts = points(a.1) as i32 + points(b.1) as i32;
                    self.taken[winner] += pts;
                    self.won[winner] += 1;
                    self.trick.clear();
                    self.leader = winner;
                    if self.hands[0].is_empty() {
                        if self.picker.is_none() {
                            // leaster: blindul revine ultimei levate
                            let extra: i32 = self.blind.iter().map(|&c| points(c) as i32).sum();
                            self.taken[winner] += extra;
                        }
                        self.finish_hand();
                    }
                }
            }
            _ => {}
        }
    }
    fn outcome(&self) -> Option<Outcome> {
        if self.phase == Phase::Over {
            Some(match self.scores[0].cmp(&self.scores[1]) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                _ => Outcome::Draw,
            })
        } else {
            None
        }
    }
    fn evaluate(&self, pl: Player) -> i32 {
        let me = pl.idx();
        let mut v = 300 * (self.scores[me] - self.scores[1 - me]);
        v += 10 * (self.taken[me] - self.taken[1 - me]);
        v += Self::hand_strength(&self.hands[me]);
        match self.phase {
            Phase::Bury | Phase::Play => {
                if let Some(pk) = self.picker {
                    if self.phase == Phase::Bury && pk == me {
                        // cât de bună e mâna cu care a ales (blind încă nevăzut în mâna 'inițială')
                        v += 4 * (Self::hand_strength(&self.hands[me]) - 100);
                    }
                    if pk == me {
                        v += 6 * self.buried.iter().map(|&c| points(c) as i32).sum::<i32>();
                    }
                }
            }
            _ => {}
        }
        v
    }
    fn render(&self) -> String {
        let view = self.viewer.unwrap_or(self.to_move()).idx();
        let mut s = format!(
            "Mâna {}/{HANDS}. Scor: J1 {} – {} J2. Puncte luate în mâna asta: J1 {}, J2 {}.\n",
            self.hand_no.min(HANDS),
            self.scores[0],
            self.scores[1],
            self.taken[0],
            self.taken[1]
        );
        if self.phase != Phase::Over {
            let mut h = self.hands[view].clone();
            h.sort_by_key(|&c| {
                (
                    std::cmp::Reverse(is_trump(c)),
                    suit(c),
                    std::cmp::Reverse(power(c)),
                )
            });
            s.push_str(&format!(
                "Mâna ta (J{}): {}\n",
                view + 1,
                h.iter().map(|&c| text(c)).collect::<Vec<_>>().join(" ")
            ));
            s.push_str(&format!(
                "Adversarul are {} cărți.\n",
                self.hands[1 - view].len()
            ));
            if !self.trick.is_empty() {
                s.push_str(&format!(
                    "Pe masă: {}\n",
                    self.trick
                        .iter()
                        .map(|&(p, c)| format!("J{}:{}", p + 1, text(c)))
                        .collect::<Vec<_>>()
                        .join(" ")
                ));
            }
            if let Some(pk) = self.picker {
                s.push_str(&format!("Declarant: J{}.\n", pk + 1));
                if view == pk && !self.buried.is_empty() {
                    s.push_str(&format!(
                        "Îngropat (secret): {}\n",
                        self.buried
                            .iter()
                            .map(|&c| text(c))
                            .collect::<Vec<_>>()
                            .join(" ")
                    ));
                }
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
            Mv::Pick => "iau".into(),
            Mv::Pass => "pas".into(),
            Mv::Bury(c) => format!("ingrop {}", text(c)),
            Mv::Play(c) => text(c),
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
    fn trump_order_and_points() {
        let qc = 5; // dama de treflă = suit 0 * 8 + 5
        let qd = 3 * 8 + 5;
        let jc = 4;
        let ad = 3 * 8 + 7;
        assert!(power(qc) > power(qd) && power(qd) > power(jc) && power(jc) > power(ad));
        assert!(is_trump(ad) && !is_trump(7) && is_trump(jc));
        let total: i32 = (0..32u8).map(|c| points(c) as i32).sum();
        assert_eq!(total, 120);
        assert_eq!((0..32u8).filter(|&c| is_trump(c)).count(), 14);
    }

    #[test]
    fn follow_suit_and_viewer_hiding() {
        let g = Sheepshead::with_seed(3);
        let mut a = g.clone_box();
        a.set_viewer(Some(Player::One));
        let mut b = g.clone_box();
        b.set_viewer(Some(Player::Two));
        assert_ne!(a.render(), b.render());
        assert!(a.render().contains("Mâna ta (J1)"));
        let mut h = g.clone();
        h.hands = [vec![7, 15, 4], vec![23, 6, 31]];
        h.phase = Phase::Play;
        h.picker = Some(0);
        h.leader = 0;
        h.trick = vec![(0, 7)]; // as de treflă (fail ♣) cerut: J2 trebuie să dea treflă (6 = K♣)
        let m = GameObj::legal_moves(&h);
        assert_eq!(m, vec!["Kc"], "{m:?}");
    }

    #[test]
    fn random_and_ai_games_finish() {
        random_finishes(|s| Box::new(Sheepshead::with_seed(s)), 4, 1500);
        ai_vs_random(|s| Box::new(Sheepshead::with_seed(s)), 2, Player::One, 1500);
        ai_vs_random(|s| Box::new(Sheepshead::with_seed(s)), 3, Player::Two, 1500);
    }
}
