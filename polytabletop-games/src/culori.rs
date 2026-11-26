//! Culori — joc original de potrivit cărți colorate (2 jucători).
//!
//! Mecanica „primul rămas fără cărți” este generică; numele, textele și
//! pachetul de aici sunt ale noastre. Pachet de 108 cărți:
//! * patru culori (r roșu, v verde, a albastru, g galben): cifra 0 o dată,
//!   cifrele 1–9 de două ori, câte 2 × `sari`, `intoarce`, `plus2`;
//! * 4 × `joker` (alegi culoarea) și 4 × `joker4` (alegi culoarea, adversarul
//!   trage 4 și pierde rândul).
//!
//! Reguli (simplificări marcate cu *):
//! * 7 cărți fiecare; prima carte de pe talon este una cu cifră.
//! * Poți juca o carte de aceeași culoare (curentă) sau cu aceeași cifră/simbol
//!   ca ultima carte; jokerele merg oricând și aleg culoarea.
//! * În doi: `sari` și `intoarce` fac adversarul să piardă rândul (joci iar);
//!   `plus2` = adversarul trage 2 și pierde rândul; `joker4` la fel cu 4.
//! * Dacă nu ai nicio carte jucabilă tragi o carte (mutarea `trage`); dacă cea
//!   trasă se potrivește o poți juca sau da `pas`. (*) Nu poți trage cât ai o
//!   carte jucabilă.
//! * (*) Nu există strigarea „ultima carte” și nici contestarea lui `joker4`.
//! * Câștigă cine rămâne primul fără cărți (o singură rundă).
//!
//! Informație ascunsă: `render` arată doar mâna Jucătorului 1 (sau a celui la
//! mutare în hot-seat). AI-ul se uită doar la mâna lui, la talon și la
//! *numărul* cărților adversarului.

use crate::cards::{handover_banner, pl, shuffle, Viewer};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

/// Culoare 0..=3 (r v a g) sau 4 = joker. `kind` 0..=9 cifre, 10 sari,
/// 11 intoarce, 12 plus2, 13 joker, 14 joker4.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct UCard {
    pub color: u8,
    pub kind: u8,
}

pub const COLOR_LETTERS: [char; 4] = ['r', 'v', 'a', 'g'];
const COLOR_NAMES: [&str; 4] = ["roșu", "verde", "albastru", "galben"];

fn color_of(ch: &str) -> Option<u8> {
    COLOR_LETTERS
        .iter()
        .position(|c| c.to_string() == ch)
        .map(|i| i as u8)
}

impl UCard {
    pub fn is_wild(self) -> bool {
        self.color == 4
    }
    pub fn text(self) -> String {
        let k = match self.kind {
            0..=9 => self.kind.to_string(),
            10 => "sari".into(),
            11 => "intoarce".into(),
            12 => "plus2".into(),
            13 => "joker".into(),
            _ => "joker4".into(),
        };
        if self.is_wild() {
            k
        } else {
            format!("{}{}", COLOR_LETTERS[self.color as usize], k)
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    /// Cartea jucată și, la jokeri, culoarea aleasă.
    Play(UCard, u8),
    Draw,
    Pass,
}

#[derive(Clone)]
pub struct Culori {
    deck: Vec<UCard>,
    discard: Vec<UCard>,
    hands: [Vec<UCard>; 2],
    cur_color: u8,
    side: Player,
    /// Cartea trasă în această mutare (se poate juca sau se dă pas).
    drawn: Option<UCard>,
    winner: Option<Player>,
    rng: Rng,
    viewer: Viewer,
    handover: bool,
}

impl Default for Culori {
    fn default() -> Self {
        Self::new()
    }
}

pub fn full_pack() -> Vec<UCard> {
    let mut v = Vec::new();
    for color in 0..4u8 {
        v.push(UCard { color, kind: 0 });
        for kind in 1..=12u8 {
            for _ in 0..2 {
                v.push(UCard { color, kind });
            }
        }
    }
    for _ in 0..4 {
        v.push(UCard { color: 4, kind: 13 });
        v.push(UCard { color: 4, kind: 14 });
    }
    v
}

impl Culori {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck = full_pack();
        shuffle(&mut deck, &mut rng);
        let mut g = Culori {
            deck,
            discard: Vec::new(),
            hands: [Vec::new(), Vec::new()],
            cur_color: 0,
            side: Player::One,
            drawn: None,
            winner: None,
            rng,
            viewer: Viewer::Fixed(Player::One),
            handover: false,
        };
        for _ in 0..7 {
            for p in 0..2 {
                let c = g.take_card();
                g.hands[p].push(c);
            }
        }
        // prima carte: una cu cifră
        loop {
            let c = g.take_card();
            if c.kind <= 9 {
                g.cur_color = c.color;
                g.discard.push(c);
                break;
            }
            g.deck.insert(0, c);
        }
        g
    }

    pub fn with_viewer(mut self, v: Viewer) -> Self {
        self.viewer = v;
        self
    }

    pub fn hot_seat(self) -> Self {
        self.with_viewer(Viewer::HotSeat)
    }

    fn take_card(&mut self) -> UCard {
        if self.deck.is_empty() {
            // amestecă talonul, păstrând cartea de deasupra
            let top = self.discard.pop();
            let mut pile = std::mem::take(&mut self.discard);
            shuffle(&mut pile, &mut self.rng);
            self.deck = pile;
            if let Some(t) = top {
                self.discard.push(t);
            }
        }
        // În cazul (practic imposibil) în care nu mai e nicio carte, ne oprim aici.
        self.deck.pop().expect("pachet gol")
    }

    fn can_draw_more(&self) -> bool {
        !self.deck.is_empty() || self.discard.len() > 1
    }

    fn draw_n(&mut self, p: Player, n: usize) {
        for _ in 0..n {
            if !self.can_draw_more() {
                break;
            }
            let c = self.take_card();
            self.hands[p.idx()].push(c);
        }
    }

    pub fn top(&self) -> UCard {
        *self.discard.last().expect("talon")
    }

    fn playable(&self, c: UCard) -> bool {
        c.is_wild() || c.color == self.cur_color || c.kind == self.top().kind
    }

    pub fn hand(&self, p: Player) -> &[UCard] {
        &self.hands[p.idx()]
    }

    pub fn total_cards(&self) -> usize {
        self.deck.len() + self.discard.len() + self.hands[0].len() + self.hands[1].len()
    }
}

impl Game for Culori {
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
        if self.winner.is_some() {
            return Vec::new();
        }
        let mut v: Vec<Move> = Vec::new();
        let cards: Vec<UCard> = match self.drawn {
            Some(c) => vec![c],
            None => self.hands[self.side.idx()].clone(),
        };
        for c in cards {
            if !self.playable(c) {
                continue;
            }
            if c.is_wild() {
                for col in 0..4u8 {
                    let m = Move::Play(c, col);
                    if !v.contains(&m) {
                        v.push(m);
                    }
                }
            } else {
                let m = Move::Play(c, c.color);
                if !v.contains(&m) {
                    v.push(m);
                }
            }
        }
        if self.drawn.is_some() {
            v.push(Move::Pass);
        } else if v.is_empty() {
            // nicio carte jucabilă: tragi (sau, dacă nu mai e nimic de tras, pas)
            if self.can_draw_more() {
                v.push(Move::Draw);
            } else {
                v.push(Move::Pass);
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        self.handover = false;
        let me = self.side;
        match *m {
            Move::Draw => {
                let c = self.take_card();
                self.hands[me.idx()].push(c);
                if self.playable(c) {
                    self.drawn = Some(c);
                } else {
                    self.side = me.other();
                    self.handover = true;
                }
            }
            Move::Pass => {
                self.drawn = None;
                self.side = me.other();
                self.handover = true;
            }
            Move::Play(c, col) => {
                self.drawn = None;
                let h = &mut self.hands[me.idx()];
                let pos = h.iter().position(|&x| x == c).expect("carte în mână");
                h.remove(pos);
                self.discard.push(c);
                self.cur_color = col;
                if self.hands[me.idx()].is_empty() {
                    // câștigă imediat; cărțile penalizatoare nu mai contează
                    self.winner = Some(me);
                    return;
                }
                let opp = me.other();
                match c.kind {
                    10 | 11 => {
                        // adversarul pierde rândul: joci iar
                    }
                    12 => self.draw_n(opp, 2),
                    14 => self.draw_n(opp, 4),
                    _ => {
                        self.side = opp;
                        self.handover = true;
                    }
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.winner.map(Outcome::Win)
    }

    fn evaluate(&self, p: Player) -> i32 {
        let q = p.other();
        let mine = &self.hands[p.idx()];
        let mut s = 30 * (self.hands[q.idx()].len() as i32 - mine.len() as i32);
        // flexibilitate: culoarea curentă să fie bine acoperită în mâna mea
        let in_color = mine.iter().filter(|c| c.color == self.cur_color).count() as i32;
        s += 2 * in_color;
        // cărțile de acțiune și jokerii sunt mai bine păstrate pentru finaluri, dar
        // mai bine jucate decât cele cu cifră când adversarul e aproape de final
        let wilds = mine.iter().filter(|c| c.is_wild()).count() as i32;
        s += wilds;
        // dacă e rândul meu și am încă mutare liberă (sari / intoarce / plus), e un plus
        if self.side == p {
            s += 3;
        }
        s
    }

    fn render(&self) -> String {
        let mut s = String::new();
        if self.handover && self.winner.is_none() {
            if let Viewer::HotSeat = self.viewer {
                s.push_str(&handover_banner(self.side));
            }
        }
        let top = self.top();
        s.push_str(&format!(
            "Culori — talon: {} | pachet: {} cărți\n",
            top.text(),
            self.deck.len()
        ));
        s.push_str(&format!(
            "Culoarea curentă: {} ({})\n",
            COLOR_NAMES[self.cur_color as usize], COLOR_LETTERS[self.cur_color as usize]
        ));
        if self.winner.is_some() {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        let v = self.viewer.who(self.side);
        let mut hand = self.hands[v.idx()].clone();
        hand.sort_by_key(|c| (c.color, c.kind));
        s.push_str(&format!(
            "Mâna Jucătorului {}: {}\n",
            pl(v),
            hand.iter().map(|c| c.text()).collect::<Vec<_>>().join(" ")
        ));
        s.push_str(&format!(
            "Jucătorul {}: {} cărți\n",
            pl(v.other()),
            self.hands[v.other().idx()].len()
        ));
        if self.drawn.is_some() {
            s.push_str("Ai tras o carte jucabilă: joac-o sau dă pas.\n");
        }
        s.push_str(&format!("La mutare: Jucătorul {}\n", pl(self.side)));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Draw => "trage".into(),
            Move::Pass => "pas".into(),
            Move::Play(c, col) => {
                if c.is_wild() {
                    format!("{}:{}", c.text(), COLOR_LETTERS[*col as usize])
                } else {
                    c.text()
                }
            }
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

/// Folosită de teste/CLI: culoarea dintr-o literă.
pub fn parse_color(s: &str) -> Option<u8> {
    color_of(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(color: u8, kind: u8) -> UCard {
        UCard { color, kind }
    }

    fn setup(h1: Vec<UCard>, h2: Vec<UCard>, top: UCard) -> Culori {
        let mut g = Culori::with_seed(1);
        let mut all = full_pack();
        for x in h1.iter().chain(h2.iter()).chain(std::iter::once(&top)) {
            let i = all.iter().position(|y| y == x).expect("în pachet");
            all.remove(i);
        }
        g.hands = [h1, h2];
        g.discard = vec![top];
        g.cur_color = top.color;
        g.deck = all;
        g.side = Player::One;
        g.drawn = None;
        g
    }

    #[test]
    fn pack_has_108_cards() {
        assert_eq!(full_pack().len(), 108);
        let g = Culori::with_seed(4);
        assert_eq!(g.total_cards(), 108);
        assert_eq!(g.hands[0].len(), 7);
        assert_eq!(g.hands[1].len(), 7);
        assert!(g.top().kind <= 9);
    }

    #[test]
    fn matching_by_color_or_kind() {
        let g = setup(
            vec![c(0, 3), c(1, 5), c(2, 7), c(3, 5)],
            vec![c(0, 1)],
            c(0, 5),
        );
        let m = g.legal_moves();
        assert!(m.contains(&Move::Play(c(0, 3), 0))); // aceeași culoare
        assert!(m.contains(&Move::Play(c(1, 5), 1))); // aceeași cifră
        assert!(m.contains(&Move::Play(c(3, 5), 3)));
        assert!(!m.contains(&Move::Play(c(2, 7), 2)));
        assert!(!m.contains(&Move::Draw));
    }

    #[test]
    fn must_draw_when_stuck_and_may_play_drawn() {
        let mut g = setup(vec![c(1, 1)], vec![c(0, 1)], c(0, 5));
        assert_eq!(g.legal_moves(), vec![Move::Draw]);
        // pachetul e fix: punem sus o carte jucabilă
        g.deck.push(c(0, 9));
        g.apply(&Move::Draw);
        assert_eq!(g.to_move(), Player::One);
        let m = g.legal_moves();
        assert!(m.contains(&Move::Play(c(0, 9), 0)));
        assert!(m.contains(&Move::Pass));
        g.apply(&Move::Pass);
        assert_eq!(g.to_move(), Player::Two);
        // o carte nejucabilă încheie rândul imediat
        let mut g = setup(vec![c(1, 1)], vec![c(0, 1)], c(0, 5));
        g.deck.push(c(2, 2));
        g.apply(&Move::Draw);
        assert_eq!(g.to_move(), Player::Two);
    }

    #[test]
    fn skip_and_plus_cards() {
        let mut g = setup(vec![c(0, 10), c(0, 12), c(0, 4)], vec![c(1, 1)], c(0, 5));
        let h2 = g.hands[1].len();
        g.apply(&Move::Play(c(0, 10), 0));
        assert_eq!(g.to_move(), Player::One); // sari: joci iar
        g.apply(&Move::Play(c(0, 12), 0));
        assert_eq!(g.to_move(), Player::One);
        assert_eq!(g.hands[1].len(), h2 + 2);
    }

    #[test]
    fn wild_four_draws_four_and_sets_color() {
        let mut g = setup(vec![c(4, 14), c(1, 1)], vec![c(0, 1)], c(0, 5));
        let h2 = g.hands[1].len();
        g.apply(&Move::Play(c(4, 14), 2));
        assert_eq!(g.cur_color, 2);
        assert_eq!(g.hands[1].len(), h2 + 4);
        assert_eq!(g.to_move(), Player::One);
        assert_eq!(g.total_cards(), 108);
    }

    #[test]
    fn emptying_the_hand_wins() {
        let mut g = setup(vec![c(0, 3)], vec![c(1, 1)], c(0, 5));
        g.apply(&Move::Play(c(0, 3), 0));
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
        assert!(g.legal_moves().is_empty());
    }

    #[test]
    fn reshuffle_keeps_cards() {
        let mut g = setup(vec![c(1, 1)], vec![c(0, 1)], c(0, 5));
        let n = g.total_cards();
        // golim pachetul în talon
        let mut d = std::mem::take(&mut g.deck);
        let top = g.discard.pop().unwrap();
        g.discard.append(&mut d);
        g.discard.push(top);
        for _ in 0..30 {
            g.draw_n(Player::One, 1);
        }
        assert_eq!(g.total_cards(), n);
    }

    #[test]
    fn random_games_finish_and_conserve() {
        for seed in 1..=30u64 {
            let mut g = Culori::with_seed(seed);
            let mut rng = Rng::new(seed + 7);
            let mut plies = 0;
            while Game::outcome(&g).is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                assert_eq!(g.total_cards(), 108);
                plies += 1;
                assert!(plies < 3000, "nu se termină");
            }
        }
    }

    #[test]
    fn render_shows_only_viewer_hand() {
        let g = Culori::with_seed(2);
        let r = Game::render(&g);
        assert!(r.contains("Mâna Jucătorului 1"));
        assert!(!r.contains("Mâna Jucătorului 2"));
    }
}
