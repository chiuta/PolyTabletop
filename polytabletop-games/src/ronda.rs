//! Ronda (Maroc / Spania), 2 jucători, pachet spaniol de 40 de cărți — variantă simplificată
//! documentată din regulile publice.
//!
//! * Culori: o = oros, c = copas, e = espadas, b = bastos. Ranguri în ordine: 1–7, S (sota),
//!   C (caballo), R (rey); „următorul rang” după 7 este S (8 și 9 nu există).
//! * 4 cărți pe masă, câte 3 în mână; când mâinile se golesc se dau încă 3 (mutare forțată
//!   `imparte`) până se termină pachetul.
//! * Joci o carte. Dacă pe masă sunt cărți de același rang, le iei pe TOATE, și apoi și
//!   cărțile de pe masă cu rangurile următoare, în șir (rang + 1, rang + 2 …) cât timp
//!   există. Dacă nu există rang egal, cartea rămâne pe masă.
//! * Puncte: **ronda** = pereche în mână la împărțire (1 punct), **trio** = trei de același
//!   rang în mână (5 puncte), acordate automat; **missa** = golești masa (1 punct, nu la ultima
//!   carte a rundei). La sfârșitul rundei ultimul care a capturat ia restul mesei, iar cine
//!   are mai mult de 20 de cărți primește 1 punct. Meciul se joacă până la 12 puncte.
//!
//! Limite (documentate): regulile Ronda variază regional (anunțul rondei, bonusul pentru
//! capturarea ultimei cărți jucate de adversar, punctele din „pique”); aici s-a ales
//! varianta de mai sus, fără anunțuri și fără bonusul „pegue”.
//!
//! Informație ascunsă: vezi `cards52.rs`; mutările sunt cărțile din mână (`5o`, `Rb`).

use crate::cards::Viewer;
use crate::cards52::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const RANK_LABEL: [&str; 10] = ["1", "2", "3", "4", "5", "6", "7", "S", "C", "R"];
const SUIT_LETTERS: [char; 4] = ['o', 'c', 'e', 'b'];
const TARGET: u8 = 12;

pub fn suit(c: u8) -> u8 {
    c / 10
}
pub fn rank(c: u8) -> u8 {
    c % 10
}
pub fn card_text(c: u8) -> String {
    format!(
        "{}{}",
        RANK_LABEL[rank(c) as usize],
        SUIT_LETTERS[suit(c) as usize]
    )
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Play(u8),
    Deal,
    Next,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Play,
    NeedDeal,
    RoundOver,
}

#[derive(Clone)]
pub struct Ronda {
    rng: Rng,
    viewer: Viewer,
    stock: Vec<u8>,
    hands: [Vec<u8>; 2],
    table: Vec<u8>,
    piles: [Vec<u8>; 2],
    last_capture: Option<Player>,
    side: Player,
    dealer: Player,
    phase: Phase,
    score: [u8; 2],
    log: Vec<String>,
}

impl Default for Ronda {
    fn default() -> Self {
        Self::new()
    }
}

impl Ronda {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Ronda {
            rng: Rng::new(seed),
            viewer: Viewer::Fixed(Player::One),
            stock: Vec::new(),
            hands: [Vec::new(), Vec::new()],
            table: Vec::new(),
            piles: [Vec::new(), Vec::new()],
            last_capture: None,
            side: Player::One,
            dealer: Player::Two,
            phase: Phase::Play,
            score: [0; 2],
            log: Vec::new(),
        };
        g.new_round();
        g
    }

    fn new_round(&mut self) {
        let mut deck: Vec<u8> = (0..40).collect();
        shuffle(&mut deck, &mut self.rng);
        self.table = deck[0..4].to_vec();
        self.hands = [deck[4..7].to_vec(), deck[7..10].to_vec()];
        self.stock = deck[10..].to_vec();
        self.piles = [Vec::new(), Vec::new()];
        self.last_capture = None;
        self.side = self.dealer.other();
        self.phase = Phase::Play;
        self.log.clear();
        self.announce();
    }

    /// Puncte pentru ronda / trio din mâinile proaspăt împărțite.
    fn announce(&mut self) {
        for p in 0..2 {
            let mut cnt = [0u8; 10];
            for &c in &self.hands[p] {
                cnt[rank(c) as usize] += 1;
            }
            let best = cnt.iter().copied().max().unwrap_or(0);
            let pts = match best {
                3 => 5,
                2 => 1,
                _ => 0,
            };
            if pts > 0 {
                self.score[p] += pts;
                self.log.push(format!(
                    "J{} are {} (+{})",
                    p + 1,
                    if pts == 5 { "trio" } else { "ronda" },
                    pts
                ));
            }
        }
    }

    pub fn score(&self) -> (u8, u8) {
        (self.score[0], self.score[1])
    }

    /// Cărțile de pe masă luate de cartea `c`.
    pub fn capture_of(&self, c: u8) -> Vec<u8> {
        let r = rank(c);
        if !self.table.iter().any(|&t| rank(t) == r) {
            return Vec::new();
        }
        let mut out = Vec::new();
        let mut k = r;
        while k < 10 && self.table.iter().any(|&t| rank(t) == k) {
            out.extend(self.table.iter().copied().filter(|&t| rank(t) == k));
            k += 1;
        }
        out.sort_unstable();
        out
    }

    fn match_over(&self) -> bool {
        (self.score[0] >= TARGET || self.score[1] >= TARGET) && self.score[0] != self.score[1]
    }

    fn finish_round(&mut self) {
        if let Some(p) = self.last_capture {
            let rest = std::mem::take(&mut self.table);
            self.piles[p.idx()].extend(rest);
        }
        let n = [self.piles[0].len(), self.piles[1].len()];
        for (p, &cnt) in n.iter().enumerate() {
            if cnt > 20 {
                self.score[p] += 1;
                self.log.push(format!("J{} are {} cărți (+1)", p + 1, cnt));
            }
        }
        self.phase = Phase::RoundOver;
    }
}

impl Game for Ronda {
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
        if self.match_over() {
            return Vec::new();
        }
        match self.phase {
            Phase::NeedDeal => vec![Move::Deal],
            Phase::RoundOver => vec![Move::Next],
            Phase::Play => {
                let mut h = self.hands[self.side.idx()].clone();
                h.sort_unstable();
                h.into_iter().map(Move::Play).collect()
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Next => {
                self.dealer = self.dealer.other();
                self.new_round();
            }
            Move::Deal => {
                self.log.clear();
                for p in 0..2 {
                    for _ in 0..3 {
                        if let Some(c) = self.stock.pop() {
                            self.hands[p].push(c);
                        }
                    }
                }
                self.phase = Phase::Play;
                self.announce();
            }
            Move::Play(c) => {
                self.log.clear();
                let me = self.side;
                let take = self.capture_of(*c);
                let h = &mut self.hands[me.idx()];
                let pos = h.iter().position(|x| x == c).expect("card in hand");
                h.remove(pos);
                if take.is_empty() {
                    self.table.push(*c);
                } else {
                    self.table.retain(|t| !take.contains(t));
                    self.piles[me.idx()].push(*c);
                    self.piles[me.idx()].extend(take.iter().copied());
                    self.last_capture = Some(me);
                }
                let last_play =
                    self.stock.is_empty() && self.hands[0].is_empty() && self.hands[1].is_empty();
                if !take.is_empty() && self.table.is_empty() && !last_play {
                    self.score[me.idx()] += 1;
                    self.log.push(format!("J{} face missa (+1)", me.idx() + 1));
                }
                self.side = me.other();
                if self.hands[0].is_empty() && self.hands[1].is_empty() {
                    if last_play {
                        self.finish_round();
                    } else {
                        self.phase = Phase::NeedDeal;
                        self.side = self.dealer.other();
                    }
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.match_over() {
            Some(Outcome::Win(if self.score[0] > self.score[1] {
                Player::One
            } else {
                Player::Two
            }))
        } else {
            None
        }
    }

    /// Doar informație publică: scor, grămezi, masă; niciodată mâna adversarului sau pachetul.
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let o = 1 - me;
        let mut s = (self.score[me] as i32 - self.score[o] as i32) * 1000;
        if self.phase == Phase::RoundOver {
            return s;
        }
        s += (self.piles[me].len() as i32 - self.piles[o].len() as i32) * 10;
        // o masă încărcată sau în șir e o pradă pentru cel care mută
        if self.phase == Phase::Play && self.side != p {
            let mut ranks = [false; 10];
            for &t in &self.table {
                ranks[rank(t) as usize] = true;
            }
            let distinct = ranks.iter().filter(|&&b| b).count() as i32;
            s -= self.table.len() as i32 * 6 + distinct * 4;
            if self.table.is_empty() {
                s += 8;
            }
        }
        s
    }

    fn render(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            "Ronda — meci până la {TARGET}: J1 {} – J2 {}\n",
            self.score[0], self.score[1]
        ));
        let mut t = self.table.clone();
        t.sort_unstable();
        s.push_str(&format!(
            "Masa: {}\n",
            if t.is_empty() {
                "—".into()
            } else {
                t.iter()
                    .map(|&c| card_text(c))
                    .collect::<Vec<_>>()
                    .join(" ")
            }
        ));
        let v = self.viewer.who(self.side);
        s.push_str(&format!(
            "Pachet: {} cărți. J1 a luat {}, J2 a luat {}; J{} (ascuns) ține {} cărți.\n",
            self.stock.len(),
            self.piles[0].len(),
            self.piles[1].len(),
            v.other().idx() + 1,
            self.hands[v.other().idx()].len()
        ));
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
        for l in &self.log {
            s.push_str(l);
            s.push('\n');
        }
        match (self.outcome(), self.phase) {
            (Some(Outcome::Win(p)), _) => s.push_str(&format!(
                "Meci încheiat: Jucător {} câștigă.\n",
                p.idx() + 1
            )),
            (_, Phase::RoundOver) => s.push_str("Runda s-a încheiat. Scrie «urmatoarea».\n"),
            (_, Phase::NeedDeal) => s.push_str("Mâinile s-au golit: scrie «imparte».\n"),
            _ => s.push_str(&format!("La mutare: Jucător {}\n", self.side.idx() + 1)),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Deal => "imparte".into(),
            Move::Next => "urmatoarea".into(),
            Move::Play(c) => card_text(*c),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // o: 0..9, c: 10..19, e: 20..29, b: 30..39; rang = index (0 = 1, 7 = S, 9 = R)
    fn o(r: u8) -> u8 {
        r
    }
    fn cp(r: u8) -> u8 {
        10 + r
    }
    fn es(r: u8) -> u8 {
        20 + r
    }
    fn bs(r: u8) -> u8 {
        30 + r
    }

    fn base() -> Ronda {
        let mut g = Ronda::with_seed(1);
        g.stock.clear();
        g.hands = [Vec::new(), Vec::new()];
        g.table.clear();
        g.piles = [Vec::new(), Vec::new()];
        g.score = [0; 2];
        g.side = Player::One;
        g.phase = Phase::Play;
        g.log.clear();
        g
    }

    #[test]
    fn deal_is_consistent() {
        let g = Ronda::with_seed(3);
        assert_eq!(g.table.len(), 4);
        assert_eq!(g.hands[0].len(), 3);
        assert_eq!(g.stock.len(), 30);
    }

    #[test]
    fn captures_same_rank_and_the_following_run() {
        let mut g = base();
        // masă: 3o 3c 4e 5b 7o ; joc 3e → 3o,3c + 4e + 5b (șirul se oprește la 6)
        g.table = vec![o(2), cp(2), es(3), bs(4), o(6)];
        g.hands[0] = vec![es(2)];
        g.hands[1] = vec![o(0)];
        assert_eq!(g.capture_of(es(2)), vec![o(2), cp(2), es(3), bs(4)]);
        g.apply(&Move::Play(es(2)));
        assert_eq!(g.table, vec![o(6)]);
        assert_eq!(g.piles[0].len(), 5);
    }

    #[test]
    fn run_skips_8_and_9() {
        let mut g = base();
        g.table = vec![o(6), cp(7), es(8)]; // 7, S, C
        g.hands[0] = vec![bs(6)];
        assert_eq!(g.capture_of(bs(6)), vec![o(6), cp(7), es(8)]);
        // rangul „7” este urmat de „S”: 6 → 7 → S (indici 5,6,7)
        g.table = vec![o(5), cp(6)];
        assert_eq!(g.capture_of(bs(5)), vec![o(5), cp(6)]);
    }

    #[test]
    fn no_match_leaves_the_card() {
        let mut g = base();
        g.table = vec![o(2)];
        g.hands[0] = vec![es(5)];
        g.hands[1] = vec![bs(1)];
        g.apply(&Move::Play(es(5)));
        assert_eq!(g.table.len(), 2);
        assert!(g.piles[0].is_empty());
    }

    #[test]
    fn missa_but_not_on_the_last_card() {
        let mut g = base();
        g.stock = vec![o(8)];
        g.table = vec![cp(4)];
        g.hands[0] = vec![es(4), es(1)];
        g.hands[1] = vec![bs(1)];
        g.apply(&Move::Play(es(4)));
        assert_eq!(g.score, [1, 0]);
        let mut h = base();
        h.table = vec![cp(4)];
        h.hands[0] = vec![es(4)];
        h.apply(&Move::Play(es(4)));
        assert_eq!(h.score, [0, 0]);
        assert_eq!(h.phase, Phase::RoundOver);
    }

    #[test]
    fn ronda_and_trio_score_on_deal() {
        let mut g = base();
        g.hands[0] = vec![o(2), cp(2), es(5)];
        g.hands[1] = vec![o(1), cp(1), es(1)];
        g.announce();
        assert_eq!(g.score, [1, 5]);
    }

    #[test]
    fn last_capturer_takes_the_table_and_majority_scores() {
        let mut g = base();
        g.table = vec![cp(4), cp(5)];
        g.piles[0] = (0..21).map(|i| i % 40).collect();
        g.piles[1] = vec![];
        g.last_capture = Some(Player::Two);
        g.finish_round();
        assert_eq!(g.score[0], 1);
        assert_eq!(g.piles[1].len(), 2);
    }

    #[test]
    fn cards_conserved_and_matches_finish() {
        for seed in 1..=12u64 {
            let mut g = Ronda::with_seed(seed);
            let mut rng = Rng::new(seed ^ 0x55);
            let mut n = 0;
            while g.outcome().is_none() {
                let all = g.stock.len()
                    + g.table.len()
                    + g.hands[0].len()
                    + g.hands[1].len()
                    + g.piles[0].len()
                    + g.piles[1].len();
                assert_eq!(all, 40);
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 40000);
            }
        }
    }

    #[test]
    fn evaluate_ignores_the_opponents_cards() {
        let g = Ronda::with_seed(9);
        let mut h = g.clone();
        let (a, b) = (h.hands[1][0], h.stock[3]);
        h.hands[1][0] = b;
        h.stock[3] = a;
        for p in [Player::One, Player::Two] {
            assert_eq!(g.evaluate(p), h.evaluate(p));
        }
    }

    #[test]
    fn render_hides_the_opponent() {
        let g = Ronda::with_seed(5);
        assert!(Game::render(&g).contains("J2 (ascuns) ține 3 cărți"));
    }

    #[test]
    fn ai_beats_a_random_mover_mostly() {
        use polytabletop_core::GameObj;
        let mut wins = 0;
        let n = 16;
        for seed in 1..=n {
            let mut g: Box<dyn GameObj> = Box::new(Ronda::with_seed(seed));
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
        assert!(wins * 10 >= n * 6, "AI won only {wins}/{n}");
    }
}
