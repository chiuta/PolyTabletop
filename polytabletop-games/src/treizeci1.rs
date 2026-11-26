//! Treizeci și unu (31), joc de cărți cu 3 cărți în mână, 2 jucători, pachet de 52.
//!
//! * Fiecare primește 3 cărți, una se întoarce în teancul de aruncate. A = 11, figurile = 10,
//!   celelalte valorează cât scriu. Mâna valorează suma cărților de **aceeași culoare** (culoare
//!   de joc: inimă/romb/pică/treflă) cea mai mare; trei cărți de aceeași valoare = 30.
//! * La rând: `trage` (din stoc, pe nevăzute) sau `ia` (cartea de sus din aruncate), apoi
//!   `arunca 5h` (nu poți arunca cartea tocmai luată din aruncate), sau `bat`: oprești runda cu
//!   mâna actuală; adversarul mai are o tură, apoi mâinile se arată.
//! * Cine face 31 (după aruncare, sau la împărțire) încheie imediat runda și adversarul pierde o
//!   viață. La arătare, mâna mai mică pierde o viață; cine a bătut și are strict cea mai mică mână
//!   pierde două; dacă cel care a bătut egalează, pierde adversarul. Fără bătaie, stoc epuizat:
//!   egalitatea nu costă nimic.
//! * 3 vieți fiecare; cine rămâne fără vieți pierde. Încheierea e o mutare formală (`sfarsit`).
//! * Informație ascunsă și AI: AI-ul vede doar mâna lui și aruncatele. Cartea trasă din stoc nu
//!   intră în evaluare (valoare medie), iar rezultatul arătării nu e vizibil în evaluare.
//!
//! Reguli: „31” / Scat (domeniu public; vezi pagat.com/draw/scat.html). Simplificări: doi jucători,
//! vieți în loc de monede, trei la fel = 30, stoc epuizat = arătare.

#![allow(clippy::needless_range_loop)]

use crate::cards::{full_deck, handover_banner, join_cards, pl, shuffle, Card, Viewer};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Draw,
    Take,
    Knock,
    Discard(Card),
    Finish,
}

fn cval(c: Card) -> i32 {
    match c.rank {
        14 => 11,
        11..=13 => 10,
        r => r as i32,
    }
}

/// Valoarea unei mâini (3 cărți; pentru mai multe, cea mai bună submulțime de 3 se alege în altă parte).
pub fn hand_value(h: &[Card]) -> i32 {
    if h.len() == 3 && h[0].rank == h[1].rank && h[1].rank == h[2].rank {
        return 30;
    }
    let mut best = 0;
    for s in 0..4u8 {
        let t: i32 = h.iter().filter(|c| c.suit == s).map(|&c| cval(c)).sum();
        best = best.max(t);
    }
    best
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Draw,
    Discard,
}

#[derive(Clone)]
pub struct Treizeci1 {
    stock: Vec<Card>,
    discard: Vec<Card>,
    hands: [Vec<Card>; 2],
    lives: [i32; 2],
    side: Player,
    first: Player,
    phase: Phase,
    taken: Option<Card>,
    /// Ultima carte din mână a fost trasă din stoc (necunoscută la evaluare).
    hidden_draw: bool,
    knocker: Option<Player>,
    rng: Rng,
    ending: bool,
    finished: bool,
    log: String,
    viewer: Viewer,
    handover: bool,
    // siguranță evaluare
    just_resolved: bool,
    ev_lives: [i32; 2],
    ev_hand: [i32; 2],
}

impl Default for Treizeci1 {
    fn default() -> Self {
        Self::new()
    }
}

impl Treizeci1 {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Treizeci1 {
            stock: Vec::new(),
            discard: Vec::new(),
            hands: [Vec::new(), Vec::new()],
            lives: [3, 3],
            side: Player::One,
            first: Player::One,
            phase: Phase::Draw,
            taken: None,
            hidden_draw: false,
            knocker: None,
            rng: Rng::new(seed),
            ending: false,
            finished: false,
            log: String::new(),
            viewer: Viewer::Fixed(Player::One),
            handover: false,
            just_resolved: false,
            ev_lives: [3, 3],
            ev_hand: [0, 0],
        };
        g.deal();
        g
    }

    pub fn lives(&self) -> (i32, i32) {
        (self.lives[0], self.lives[1])
    }

    /// Împarte o rundă nouă (cu 31 la împărțire: adversarul pierde o viață și se reîmparte).
    fn deal(&mut self) {
        for _ in 0..20 {
            let mut deck = full_deck(2);
            shuffle(&mut deck, &mut self.rng);
            self.hands = [Vec::new(), Vec::new()];
            for _ in 0..3 {
                for p in 0..2 {
                    self.hands[p].push(deck.pop().unwrap());
                }
            }
            self.discard = vec![deck.pop().unwrap()];
            self.stock = deck;
            self.phase = Phase::Draw;
            self.taken = None;
            self.hidden_draw = false;
            self.knocker = None;
            self.side = self.first;
            let v = [hand_value(&self.hands[0]), hand_value(&self.hands[1])];
            if (v[0] == 31) == (v[1] == 31) {
                return;
            }
            let w = if v[0] == 31 { 0 } else { 1 };
            self.log = format!("J{} are 31 la împărțire.", w + 1);
            self.lives[1 - w] -= 1;
            if self.lives[1 - w] <= 0 {
                self.ending = true;
                return;
            }
        }
    }

    /// Încheie runda; `loss` = vieți pierdute. `showdown` = rezultatul depinde de mâna acoperită.
    fn resolve(&mut self, loss: [i32; 2], showdown: bool, text: String) {
        let before = self.lives;
        self.ev_hand = [hand_value(&self.hands[0]), hand_value(&self.hands[1])];
        for p in 0..2 {
            self.lives[p] -= loss[p];
        }
        self.ev_lives = if showdown { before } else { self.lives };
        self.just_resolved = true;
        self.log = text;
        self.first = self.first.other();
        if self.lives.iter().any(|&l| l <= 0) {
            self.ending = true;
        } else {
            self.deal();
        }
    }

    fn showdown(&mut self) {
        let v = [hand_value(&self.hands[0]), hand_value(&self.hands[1])];
        let mut loss = [0, 0];
        match self.knocker {
            Some(k) => {
                let (ki, oi) = (k.idx(), k.other().idx());
                if v[ki] < v[oi] {
                    loss[ki] = 2;
                } else {
                    loss[oi] = 1;
                }
            }
            None => {
                if v[0] < v[1] {
                    loss[0] = 1;
                } else if v[1] < v[0] {
                    loss[1] = 1;
                }
            }
        }
        let text = format!(
            "Arătare: J1 {} = {}, J2 {} = {}.",
            join_cards(&self.hands[0]),
            v[0],
            join_cards(&self.hands[1]),
            v[1]
        );
        self.resolve(loss, true, text);
    }

    /// Cea mai bună valoare a mâinii de 4 cărți după o aruncare permisă.
    fn best_after_discard(&self, p: usize) -> i32 {
        let h = &self.hands[p];
        let mut best = 0;
        for i in 0..h.len() {
            if Some(h[i]) == self.taken {
                continue;
            }
            let mut r = h.clone();
            r.remove(i);
            best = best.max(hand_value(&r));
        }
        best
    }
}

impl Game for Treizeci1 {
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
        if self.finished {
            return Vec::new();
        }
        if self.ending {
            return vec![Move::Finish];
        }
        match self.phase {
            Phase::Draw => {
                let mut v = vec![Move::Draw, Move::Take];
                if self.knocker.is_none() {
                    v.push(Move::Knock);
                }
                v
            }
            Phase::Discard => self.hands[self.side.idx()]
                .iter()
                .filter(|&&c| Some(c) != self.taken)
                .map(|&c| Move::Discard(c))
                .collect(),
        }
    }

    fn apply(&mut self, m: &Move) {
        self.just_resolved = false;
        self.handover = false;
        let me = self.side.idx();
        match *m {
            Move::Finish => self.finished = true,
            Move::Draw => {
                let c = self.stock.pop().expect("stock");
                self.hands[me].push(c);
                self.phase = Phase::Discard;
                self.taken = None;
                self.hidden_draw = true;
                self.log = format!("J{} trage din stoc.", me + 1);
            }
            Move::Take => {
                let c = self.discard.pop().expect("discard");
                self.hands[me].push(c);
                self.phase = Phase::Discard;
                self.taken = Some(c);
                self.hidden_draw = false;
                self.log = format!("J{} ia {}.", me + 1, c.text());
            }
            Move::Knock => {
                self.knocker = Some(self.side);
                self.log = format!("J{} bate: adversarul mai are o tură.", me + 1);
                self.side = self.side.other();
                self.handover = true;
            }
            Move::Discard(c) => {
                let pos = self.hands[me].iter().position(|&x| x == c).expect("card");
                self.hands[me].remove(pos);
                self.discard.push(c);
                self.phase = Phase::Draw;
                self.taken = None;
                self.hidden_draw = false;
                self.handover = true;
                let v = hand_value(&self.hands[me]);
                if v == 31 {
                    let mut loss = [0, 0];
                    loss[1 - me] = 1;
                    if self.knocker == Some(self.side.other()) {
                        // adversarul care a bătut pierde o viață
                    }
                    let text = format!(
                        "J{} face 31 ({}): J{} pierde o viață.",
                        me + 1,
                        join_cards(&self.hands[me]),
                        2 - me
                    );
                    self.resolve(loss, false, text);
                } else if self.knocker.is_some() || self.stock.is_empty() {
                    self.showdown();
                } else {
                    self.side = self.side.other();
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.finished {
            return None;
        }
        Some(if self.lives[0] <= 0 {
            Outcome::Win(Player::Two)
        } else {
            Outcome::Win(Player::One)
        })
    }

    /// Doar viețile, mâna lui `p` și aruncatele. Cartea trasă din stoc se înlocuiește cu o valoare
    /// medie; după arătare nu se citește nici rezultatul, nici noua împărțire.
    fn evaluate(&self, p: Player) -> i32 {
        let (pi, qi) = (p.idx(), p.other().idx());
        if self.just_resolved || self.ending {
            let l = if self.just_resolved {
                self.ev_lives
            } else {
                self.lives
            };
            return 100 * (l[pi] - l[qi])
                + if self.just_resolved {
                    self.ev_hand[pi]
                } else {
                    0
                };
        }
        let mut s = 100 * (self.lives[pi] - self.lives[qi]);
        let h = &self.hands[pi];
        if self.side == p && self.phase == Phase::Discard {
            if self.hidden_draw {
                s += hand_value(&h[..h.len() - 1]) + 4;
            } else {
                s += self.best_after_discard(pi);
            }
        } else {
            let v = hand_value(h);
            s += v;
            if self.knocker == Some(p) {
                s += 3 + (v - 20).max(0);
            }
        }
        s
    }

    fn render(&self) -> String {
        let mut s = String::new();
        if self.handover && !self.finished {
            if let Viewer::HotSeat = self.viewer {
                s.push_str(&handover_banner(self.side));
            }
        }
        s.push_str(&format!(
            "Treizeci și unu — vieți: J1 {}, J2 {} | stoc: {}\n",
            self.lives[0].max(0),
            self.lives[1].max(0),
            self.stock.len()
        ));
        if !self.log.is_empty() {
            s.push_str(&self.log);
            s.push('\n');
        }
        if self.finished {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        if self.ending {
            s.push_str("Un jucător a rămas fără vieți: scrie «sfarsit».\n");
            return s;
        }
        s.push_str(&format!(
            "Aruncate (sus): {}\n",
            self.discard
                .last()
                .map(|c| c.text())
                .unwrap_or_else(|| "—".into())
        ));
        let v = self.viewer.who(self.side);
        s.push_str(&format!(
            "Mâna Jucătorului {}: {}  (valoare {})\n",
            pl(v),
            join_cards(&self.hands[v.idx()]),
            if self.hands[v.idx()].len() == 3 {
                hand_value(&self.hands[v.idx()])
            } else {
                0
            }
        ));
        s.push_str(&format!(
            "Jucătorul {}: {} cărți\n",
            pl(v.other()),
            self.hands[v.other().idx()].len()
        ));
        if let Some(k) = self.knocker {
            s.push_str(&format!("J{} a bătut: ultima tură.\n", pl(k)));
        }
        s.push_str(&format!("La mutare: Jucătorul {}\n", pl(self.side)));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Draw => "trage".into(),
            Move::Take => "ia".into(),
            Move::Knock => "bat".into(),
            Move::Discard(c) => format!("arunca {}", c.text()),
            Move::Finish => "sfarsit".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(rank: u8, suit: u8) -> Card {
        Card { rank, suit }
    }

    fn setup(h1: Vec<Card>, h2: Vec<Card>) -> Treizeci1 {
        let mut g = Treizeci1::with_seed(1);
        g.hands = [h1, h2];
        g.lives = [3, 3];
        g.side = Player::One;
        g.first = Player::One;
        g.phase = Phase::Draw;
        g.taken = None;
        g.knocker = None;
        g.ending = false;
        g.discard = vec![c(2, 0)];
        g.stock = (3..=10).map(|r| c(r, 3)).collect();
        g
    }

    #[test]
    fn hand_values() {
        assert_eq!(hand_value(&[c(14, 0), c(13, 0), c(10, 0)]), 31);
        assert_eq!(hand_value(&[c(14, 0), c(13, 1), c(10, 2)]), 11);
        assert_eq!(hand_value(&[c(9, 0), c(9, 1), c(9, 2)]), 30);
        assert_eq!(hand_value(&[c(9, 0), c(5, 0), c(3, 1)]), 14);
    }

    #[test]
    fn cannot_discard_taken_card_and_knock_only_once() {
        let mut g = setup(
            vec![c(5, 0), c(6, 1), c(7, 2)],
            vec![c(5, 1), c(6, 2), c(7, 3)],
        );
        g.apply(&Move::Take);
        let m = g.legal_moves();
        assert!(!m.contains(&Move::Discard(c(2, 0))));
        assert_eq!(m.len(), 3);
        g.apply(&Move::Discard(c(5, 0)));
        assert!(g.legal_moves().contains(&Move::Knock));
        g.apply(&Move::Knock);
        assert!(!g.legal_moves().contains(&Move::Knock));
    }

    #[test]
    fn lone_knocker_loses_two_and_tie_costs_opponent() {
        let mut g = setup(
            vec![c(2, 0), c(3, 1), c(4, 2)],
            vec![c(9, 1), c(8, 1), c(3, 2)],
        );
        g.apply(&Move::Knock);
        g.apply(&Move::Draw); // J2 draws 10c
        g.apply(&Move::Discard(c(3, 2)));
        assert_eq!(g.lives(), (1, 3));
        // tie: knocker safe, opponent loses
        let mut g = setup(
            vec![c(10, 0), c(3, 1), c(4, 2)],
            vec![c(10, 1), c(3, 2), c(4, 3)],
        );
        g.stock = vec![c(2, 3)];
        g.apply(&Move::Knock);
        g.apply(&Move::Draw);
        g.apply(&Move::Discard(c(2, 3)));
        assert_eq!(g.lives(), (3, 2));
    }

    #[test]
    fn thirty_one_ends_round() {
        let mut g = setup(
            vec![c(14, 0), c(13, 0), c(2, 1)],
            vec![c(5, 1), c(6, 2), c(7, 3)],
        );
        g.stock = vec![c(10, 0)];
        g.apply(&Move::Draw);
        g.apply(&Move::Discard(c(2, 1)));
        assert_eq!(g.lives(), (3, 2));
    }

    #[test]
    fn game_over_needs_finish_move() {
        let mut g = setup(
            vec![c(2, 0), c(3, 1), c(4, 2)],
            vec![c(10, 1), c(8, 1), c(3, 2)],
        );
        g.lives = [1, 3];
        g.apply(&Move::Knock);
        g.apply(&Move::Draw);
        g.apply(&Move::Discard(c(3, 2)));
        assert!(g.outcome().is_none());
        assert_eq!(g.legal_moves(), vec![Move::Finish]);
        g.apply(&Move::Finish);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
    }

    #[test]
    fn render_hides_opponent_hand() {
        let g = Treizeci1::with_seed(4);
        let r = Game::render(&g);
        for x in &g.hands[1] {
            assert!(!r.split_whitespace().any(|t| t == x.text()), "{r}");
        }
        let mut h = g.clone();
        Game::set_viewer(&mut h, None);
        h.apply(&Move::Take);
        h.apply(&h.legal_moves()[0].clone());
        let r = Game::render(&h);
        assert!(r.contains("Predă ecranul"));
        for x in &h.hands[0] {
            assert!(!r.split_whitespace().any(|t| t == x.text()), "{r}");
        }
    }

    #[test]
    fn ai_is_blind_to_hidden_cards() {
        for seed in 1..80u64 {
            let mut g = Treizeci1::with_seed(seed);
            let mut rng = Rng::new(seed);
            for _ in 0..(seed as usize % 12) {
                if g.outcome().is_some() || g.ending {
                    break;
                }
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
            }
            if g.finished || g.ending {
                continue;
            }
            let me = g.side.idx();
            let mut h = g.clone();
            let mut pool = h.stock.clone();
            let keep_last = if g.hidden_draw { 1 } else { 0 };
            let _ = keep_last;
            pool.extend(&h.hands[1 - me]);
            shuffle(&mut pool, &mut rng);
            let n = h.hands[1 - me].len();
            h.hands[1 - me] = pool.drain(..n).collect();
            h.stock = pool;
            // every child evaluates identically for the mover (no peeking at hidden cards or at the
            // result of a showdown / the next deal)
            for mv in g.legal_moves() {
                let (mut a, mut b) = (g.clone(), h.clone());
                a.apply(&mv);
                b.apply(&mv);
                if matches!(mv, Move::Draw) {
                    // the stock top differs between the clones only if the pool was reshuffled
                    // (it is): the drawn card must not matter
                }
                assert_eq!(
                    a.evaluate(g.side),
                    b.evaluate(g.side),
                    "seed {seed} move {mv:?}"
                );
                assert_eq!(a.outcome().is_some(), b.outcome().is_some());
            }
            assert_eq!(
                polytabletop_core::ai::best_move(&g, 7),
                polytabletop_core::ai::best_move(&h, 7),
                "seed {seed}"
            );
        }
    }

    #[test]
    fn random_games_finish() {
        for seed in 1..50u64 {
            let mut g = Treizeci1::with_seed(seed);
            let mut rng = Rng::new(seed ^ 3);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
                assert!(n < 20000, "seed {seed}");
            }
            let n: usize = g.hands.iter().map(|h| h.len()).sum();
            assert!(n >= 6);
        }
    }

    #[test]
    fn ai_self_play_finishes() {
        for seed in 1..6u64 {
            let mut g = Treizeci1::with_seed(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = polytabletop_core::ai::best_move(&g, seed + n).unwrap();
                g.apply(&m);
                n += 1;
                assert!(n < 5000);
            }
        }
    }
}
