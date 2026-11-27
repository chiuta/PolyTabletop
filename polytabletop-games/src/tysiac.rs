//! Tysiąc (Tysiacha, „o mie”; Polonia / Rusia) — variantă pentru doi jucători, din regulile publice.
//!
//! Pachet de 24 de cărți (9 J Q K 10 A; culori c ♣, s ♠, h ♥, d ♦); puncte: A 11, 10 10, K 4, Q 3, J 2, 9 0 (120 în total).
//! * Împărțire: 10 cărți fiecăruia și două „musik” de câte 2 cărți (adaptare pentru doi: în varianta de trei, musik-ul e unul).
//! * Licitație: de la 100 în pași de 10, până la 300 (`licitez 110`, `pas`); cine rămâne declarant.
//!   Dacă amândoi dau pas, se reîmparte. Declarantul vede ambele musik-uri, ia unul (`musik 1`) și lasă două cărți din
//!   cele 12 (`las Qc`); punctele lăsate sunt ale lui.
//! * Joc: 10 levate, declarantul conduce. Răspunzi la culoarea cerută și iei, dacă poți; fără culoare, joci atu dacă ai.
//! * Căsătorie (marjas): conducând K sau Q cu perechea în mână (`Kh marjas`), cu condiția să fi luat deja o levată,
//!   încasezi ♠ 40, ♣ 60, ♦ 80, ♥ 100 și acea culoare devine atu.
//! * Scor mână: declarantul câștigă +licitația dacă a strâns cel puțin atât (levate + căsătorii + cărțile lăsate), altfel −licitația;
//!   celălalt adaugă punctele lui rotunjite la zeci. Se joacă 6 mâini sau până cineva ajunge la 1000; mai multe puncte câștigă.
//!
//! Omise: butoiul (barrel), bolta și licitația la 3 jucători. Informație ascunsă: mâinile. AI-ul decide licitația după
//! o estimare a punctelor din propria mână, nu citește cărțile adversarului.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const RANKS: [&str; 6] = ["9", "J", "Q", "K", "10", "A"];
const SUITS: [char; 4] = ['c', 's', 'h', 'd'];
const MARRIAGE: [i32; 4] = [60, 40, 100, 80];
const PTS: [i32; 6] = [0, 2, 3, 4, 10, 11];
const HANDS: u8 = 6;
const TARGET: i32 = 1000;

fn suit(c: u8) -> u8 {
    c / 6
}
fn rank(c: u8) -> u8 {
    c % 6
}
pub fn points(c: u8) -> i32 {
    PTS[rank(c) as usize]
}
pub fn text(c: u8) -> String {
    format!("{}{}", RANKS[rank(c) as usize], SUITS[suit(c) as usize])
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mv {
    Bid(u16),
    Pass,
    Take(u8),
    Discard(u8),
    Play(u8),
    Marry(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Bid,
    Take,
    Discard,
    Play,
    Over,
}

#[derive(Clone)]
pub struct Tysiac {
    viewer: Option<Player>,
    hands: [Vec<u8>; 2],
    musik: [Vec<u8>; 2],
    discards: Vec<u8>,
    phase: Phase,
    turn: usize,
    bid: u16,
    high: Option<usize>,
    first_passed: bool,
    trick: Vec<(usize, u8)>,
    leader: usize,
    taken: [i32; 2],
    married: [i32; 2],
    won: [u8; 2],
    trump: Option<u8>,
    dealer: usize,
    hand_no: u8,
    scores: [i32; 2],
    rng: Rng,
    log: String,
}

impl Default for Tysiac {
    fn default() -> Self {
        Self::new()
    }
}

fn est(h: &[u8]) -> i32 {
    let pts: i32 = h.iter().map(|&c| points(c)).sum();
    let mut m = 0;
    for s in 0..4u8 {
        if h.contains(&(s * 6 + 2)) && h.contains(&(s * 6 + 3)) {
            m += MARRIAGE[s as usize];
        }
    }
    (pts + m) * 7 / 10 + 30
}

impl Tysiac {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut g = Tysiac {
            viewer: None,
            hands: [Vec::new(), Vec::new()],
            musik: [Vec::new(), Vec::new()],
            discards: Vec::new(),
            phase: Phase::Bid,
            turn: 0,
            bid: 0,
            high: None,
            first_passed: false,
            trick: Vec::new(),
            leader: 0,
            taken: [0; 2],
            married: [0; 2],
            won: [0; 2],
            trump: None,
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
        let mut d: Vec<u8> = (0..24).collect();
        for i in (1..24).rev() {
            let j = self.rng.below(i + 1);
            d.swap(i, j);
        }
        self.hands = [d[0..10].to_vec(), d[10..20].to_vec()];
        self.musik = [d[20..22].to_vec(), d[22..24].to_vec()];
        self.discards.clear();
        self.phase = Phase::Bid;
        self.turn = 1 - self.dealer;
        self.bid = 0;
        self.high = None;
        self.first_passed = false;
        self.trick.clear();
        self.taken = [0; 2];
        self.married = [0; 2];
        self.won = [0; 2];
        self.trump = None;
        self.log = format!(
            "Mâna {}/{HANDS}: J{} împarte.",
            self.hand_no,
            self.dealer + 1
        );
    }
    fn actor(&self) -> usize {
        match self.phase {
            Phase::Play => (self.leader + self.trick.len()) % 2,
            _ => self.turn,
        }
    }
    fn beats(&self, a: u8, b: u8) -> bool {
        // a bate b (b e cartea deja jucată, a cartea nouă)?
        let (ta, tb) = (Some(suit(a)) == self.trump, Some(suit(b)) == self.trump);
        if ta != tb {
            return ta;
        }
        suit(a) == suit(b) && rank(a) > rank(b)
    }
    fn legal_cards(&self, p: usize) -> Vec<u8> {
        let h = &self.hands[p];
        let Some(&(_, c0)) = self.trick.first() else {
            return h.clone();
        };
        let same: Vec<u8> = h.iter().copied().filter(|&c| suit(c) == suit(c0)).collect();
        if !same.is_empty() {
            let over: Vec<u8> = same
                .iter()
                .copied()
                .filter(|&c| rank(c) > rank(c0))
                .collect();
            return if over.is_empty() { same } else { over };
        }
        if let Some(t) = self.trump {
            let tr: Vec<u8> = h.iter().copied().filter(|&c| suit(c) == t).collect();
            if !tr.is_empty() {
                return tr;
            }
        }
        h.clone()
    }
    fn can_marry(&self, p: usize, c: u8) -> bool {
        self.trick.is_empty() && self.won[p] > 0 && (rank(c) == 2 || rank(c) == 3) && {
            let other = suit(c) * 6 + if rank(c) == 2 { 3 } else { 2 };
            self.hands[p].contains(&other)
        }
    }
    fn finish_hand(&mut self) {
        let d = self.high.unwrap();
        let disc: i32 = self.discards.iter().map(|&c| points(c)).sum();
        let total = self.taken[d] + self.married[d] + disc;
        let o = 1 - d;
        let opp = ((self.taken[o] + self.married[o]) + 5) / 10 * 10;
        if total >= self.bid as i32 {
            self.scores[d] += self.bid as i32;
            self.log = format!("J{} face {total} din {}: +{}.", d + 1, self.bid, self.bid);
        } else {
            self.scores[d] -= self.bid as i32;
            self.log = format!(
                "J{} face doar {total} din {}: −{}.",
                d + 1,
                self.bid,
                self.bid
            );
        }
        self.scores[o] += opp;
        self.log.push_str(&format!(" J{} adaugă {opp}.", o + 1));
        if self.hand_no >= HANDS || self.scores.iter().any(|&s| s >= TARGET) {
            self.phase = Phase::Over;
        } else {
            let keep = self.log.clone();
            self.deal();
            self.log = format!("{keep} {}", self.log);
        }
    }
}

impl Game for Tysiac {
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
            Phase::Bid => {
                let lo = if self.bid == 0 { 100 } else { self.bid + 10 };
                let mut v: Vec<Mv> = (lo..=300).step_by(10).map(Mv::Bid).collect();
                v.push(Mv::Pass);
                v
            }
            Phase::Take => vec![Mv::Take(0), Mv::Take(1)],
            Phase::Discard => self.hands[p].iter().map(|&c| Mv::Discard(c)).collect(),
            Phase::Play => {
                let mut v = Vec::new();
                for c in self.legal_cards(p) {
                    v.push(Mv::Play(c));
                    if self.can_marry(p, c) {
                        v.push(Mv::Marry(c));
                    }
                }
                v
            }
        }
    }
    fn apply(&mut self, m: &Mv) {
        let p = self.actor();
        match (*m, self.phase) {
            (Mv::Bid(b), Phase::Bid) => {
                self.bid = b;
                self.high = Some(p);
                self.turn = 1 - p;
            }
            (Mv::Pass, Phase::Bid) => {
                if let Some(h) = self.high {
                    self.phase = Phase::Take;
                    self.turn = h;
                    self.log = format!(
                        "J{} câștigă licitația cu {}. Musik-urile: [{}] și [{}].",
                        h + 1,
                        self.bid,
                        self.musik[0]
                            .iter()
                            .map(|&c| text(c))
                            .collect::<Vec<_>>()
                            .join(" "),
                        self.musik[1]
                            .iter()
                            .map(|&c| text(c))
                            .collect::<Vec<_>>()
                            .join(" ")
                    );
                } else if self.first_passed {
                    let keep = "Amândoi au dat pas: se reîmparte.";
                    self.hand_no -= 1;
                    self.deal();
                    self.log = format!("{keep} {}", self.log);
                } else {
                    self.first_passed = true;
                    self.turn = 1 - p;
                }
            }
            (Mv::Take(k), Phase::Take) => {
                let cards = self.musik[k as usize].clone();
                self.hands[p].extend(cards);
                self.phase = Phase::Discard;
            }
            (Mv::Discard(c), Phase::Discard) => {
                let h = &mut self.hands[p];
                let i = h.iter().position(|&x| x == c).unwrap();
                h.remove(i);
                self.discards.push(c);
                if self.discards.len() == 2 {
                    self.phase = Phase::Play;
                    self.leader = p;
                }
            }
            (Mv::Play(c), Phase::Play) | (Mv::Marry(c), Phase::Play) => {
                if matches!(m, Mv::Marry(_)) {
                    self.married[p] += MARRIAGE[suit(c) as usize];
                    self.trump = Some(suit(c));
                    self.log = format!(
                        "J{} anunță căsătoria {}: +{}.",
                        p + 1,
                        SUITS[suit(c) as usize],
                        MARRIAGE[suit(c) as usize]
                    );
                }
                let h = &mut self.hands[p];
                let i = h.iter().position(|&x| x == c).unwrap();
                h.remove(i);
                self.trick.push((p, c));
                if self.trick.len() == 2 {
                    let (a, b) = (self.trick[0], self.trick[1]);
                    let winner = if self.beats(b.1, a.1) { b.0 } else { a.0 };
                    self.taken[winner] += points(a.1) + points(b.1);
                    self.won[winner] += 1;
                    self.trick.clear();
                    self.leader = winner;
                    if self.hands[0].is_empty() {
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
        let mut v = 5 * (self.scores[me] - self.scores[1 - me]);
        match self.phase {
            Phase::Bid => {
                // doar pentru jucătorul care tocmai a licitat (cel curent e adversarul); mâna
                // adversarului nu se citește: se presupune că estimarea lui e chiar licitația
                if let Some(h) = self.high {
                    if h == me {
                        v += (est(&self.hands[h]) - self.bid as i32) * 2;
                    }
                }
            }
            Phase::Take | Phase::Discard => {
                let h = self.high.unwrap();
                if h == me {
                    v += est(&self.hands[h]);
                } else {
                    // declarantul advers: estimare publică (licitația), nu mâna lui
                    v -= self.bid as i32;
                }
            }
            _ => {}
        }
        if matches!(self.phase, Phase::Play | Phase::Discard) {
            v +=
                4 * (self.taken[me] + self.married[me] - self.taken[1 - me] - self.married[1 - me]);
            v += self.hands[me].iter().map(|&c| points(c)).sum::<i32>() / 2;
            if self.high == Some(me) {
                v += self.discards.iter().map(|&c| points(c)).sum::<i32>() * 2;
            }
        }
        v
    }
    fn render(&self) -> String {
        let view = self.viewer.unwrap_or(self.to_move()).idx();
        let mut s = format!(
            "Mâna {}/{HANDS}. Scor: J1 {} – {} J2 (țintă {TARGET}). Atu: {}.\n",
            self.hand_no.min(HANDS),
            self.scores[0],
            self.scores[1],
            self.trump
                .map_or("—".to_string(), |t| SUITS[t as usize].to_string())
        );
        if self.phase != Phase::Over {
            let mut h = self.hands[view].clone();
            h.sort_by_key(|&c| (suit(c), std::cmp::Reverse(rank(c))));
            s.push_str(&format!(
                "Mâna ta (J{}): {}\n",
                view + 1,
                h.iter().map(|&c| text(c)).collect::<Vec<_>>().join(" ")
            ));
            s.push_str(&format!(
                "Adversarul are {} cărți.\n",
                self.hands[1 - view].len()
            ));
            if self.phase == Phase::Bid {
                s.push_str(&format!(
                    "Licitația curentă: {}.\n",
                    if self.bid == 0 {
                        "—".to_string()
                    } else {
                        self.bid.to_string()
                    }
                ));
            } else {
                s.push_str(&format!("Declarant: J{} (licitat {}). Levate: J1 {} p, J2 {} p; căsătorii J1 {}, J2 {}.\n", self.high.unwrap() + 1, self.bid, self.taken[0], self.taken[1], self.married[0], self.married[1]));
            }
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
            if self.phase == Phase::Discard && self.high == Some(view) {
                s.push_str("Lasă două cărți (puncte pentru tine).\n");
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
            Mv::Bid(b) => format!("licitez {b}"),
            Mv::Pass => "pas".into(),
            Mv::Take(k) => format!("musik {}", k + 1),
            Mv::Discard(c) => format!("las {}", text(c)),
            Mv::Play(c) => text(c),
            Mv::Marry(c) => format!("{} marjas", text(c)),
        }
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
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
    fn deck_points_and_marriages() {
        assert_eq!((0..24u8).map(points).sum::<i32>(), 120);
        assert_eq!(MARRIAGE.iter().sum::<i32>(), 280);
    }

    #[test]
    fn bidding_then_musik_and_viewer() {
        let mut g = Tysiac::with_seed(7);
        let mut a = g.clone_box();
        a.set_viewer(Some(Player::One));
        let mut b = g.clone_box();
        b.set_viewer(Some(Player::Two));
        assert_ne!(a.render(), b.render());
        let m = GameObj::legal_moves(&g);
        assert!(m.contains(&"licitez 100".to_string()) && m.contains(&"pas".to_string()));
        GameObj::play(&mut g, "licitez 100").unwrap();
        GameObj::play(&mut g, "pas").unwrap();
        let t = GameObj::legal_moves(&g);
        assert_eq!(t, vec!["musik 1", "musik 2"]);
    }

    #[test]
    fn follow_suit_and_beat_rule() {
        let mut g = Tysiac::with_seed(1);
        g.phase = Phase::Play;
        g.high = Some(0);
        g.leader = 0;
        g.hands = [vec![5], vec![0, 4, 6]]; // J1: A♣; J2: 9♣, 10♣, 9♠
        g.trick = vec![(0, 3)]; // K♣ condus
        g.trump = None;
        assert_eq!(GameObj::legal_moves(&g), vec!["10c"]);
    }

    #[test]
    fn random_and_ai_games_finish() {
        random_finishes(|s| Box::new(Tysiac::with_seed(s)), 4, 1500);
        ai_vs_random(|s| Box::new(Tysiac::with_seed(s)), 2, Player::One, 1500);
        ai_vs_random(|s| Box::new(Tysiac::with_seed(s)), 3, Player::Two, 1500);
    }
}

#[cfg(test)]
mod a1_blind {
    use super::*;
    use crate::ai_audit_a1::{audit, shuffle};

    #[test]
    fn ai_ignores_opponent_hand_and_musik_at_all_levels() {
        audit("tysiac", Tysiac::with_seed, 1..=4, 3, 90, |g, salt| {
            let me = g.actor();
            let foe = 1 - me;
            let n = g.hands[foe].len();
            if g.phase == Phase::Bid {
                // înainte de licitație: mâna adversarului și cele două musik-uri sunt necunoscute
                let mut pool: Vec<u8> = (0..24u8).filter(|c| !g.hands[me].contains(c)).collect();
                shuffle(&mut pool, salt);
                g.hands[foe] = pool.drain(..n).collect();
                g.musik = [pool.drain(..2).collect(), pool.drain(..2).collect()];
            } else {
                let mut pool: Vec<u8> = (0..24u8)
                    .filter(|c| {
                        !g.hands[me].contains(c)
                            && !g.trick.iter().any(|t| t.1 == *c)
                            && !g.discards.contains(c)
                    })
                    .collect();
                shuffle(&mut pool, salt);
                g.hands[foe] = pool.drain(..n.min(pool.len())).collect();
            }
            g.rng = Rng::new(salt ^ 3);
        });
    }
}
