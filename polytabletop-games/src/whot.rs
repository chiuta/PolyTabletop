//! Whot (Nigeria / Africa de Vest): joc de cărți cu pachet special de 54 de cărți, 2 jucători.
//!
//! * Pachetul: cercuri (`o`) și triunghiuri (`t`) cu 1–5, 7, 8, 10–14; cruci (`x`) și
//!   pătrate (`p`) cu 1–3, 5, 7, 10, 11, 13, 14; stele (`s`) cu 1–5, 7, 8; 5 cărți
//!   „Whot” (20). Fiecare primește 6 cărți; se întoarce o carte de start obișnuită.
//! * Joci o carte de aceeași formă sau același număr cu cea de deasupra; Whot se joacă
//!   oricând și cere o formă (`w:o`). Dacă nu poți (sau nu vrei) joci, tragi o carte din
//!   piață (`trage`) și tura trece.
//! * Cărți speciale: **1** (hold on) și **8** (suspendare) — mai joci o dată; **14**
//!   (piață generală) — adversarul trage o carte și mai joci o dată; **2** — adversarul
//!   trage 2; **5** — adversarul trage 3. Penalizările se pot înlănțui: adversarul poate
//!   răspunde cu un 2 pe un 2 (respectiv 5 pe 5), iar penalizarea crește; cine nu poate
//!   (sau nu vrea) trage tot.
//! * Câștigă cine rămâne primul fără cărți (se aplică și dacă ultima carte e specială).
//!   Dacă piața se golește, se amestecă teancul de jos (fără carte de deasupra); dacă nu
//!   mai e nimic de tras, treci (`pas`).
//!
//! Informație ascunsă: vezi doar mâna Jucătorului 1 (sau a celui la mutare în modul „la
//! masă”). AI-ul evaluează numai mâna lui și numărul cărților adversarului.
//!
//! Notație: carte `o5`, `t14`; Whot `w:s` (cere stele); `trage`; `pas`.

use crate::cards::Viewer;
use crate::cards52::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const SHAPE_LETTERS: [char; 5] = ['o', 't', 'x', 'p', 's'];

fn build_deck() -> Vec<(u8, u8)> {
    let mut d = Vec::new();
    let sets: [(u8, &[u8]); 5] = [
        (0, &[1, 2, 3, 4, 5, 7, 8, 10, 11, 12, 13, 14]),
        (1, &[1, 2, 3, 4, 5, 7, 8, 10, 11, 12, 13, 14]),
        (2, &[1, 2, 3, 5, 7, 10, 11, 13, 14]),
        (3, &[1, 2, 3, 5, 7, 10, 11, 13, 14]),
        (4, &[1, 2, 3, 4, 5, 7, 8]),
    ];
    for (s, nums) in sets {
        for &n in nums {
            d.push((s, n));
        }
    }
    for _ in 0..5 {
        d.push((5, 20));
    }
    d
}

fn deck() -> &'static [(u8, u8)] {
    use std::sync::OnceLock;
    static D: OnceLock<Vec<(u8, u8)>> = OnceLock::new();
    D.get_or_init(build_deck)
}

pub fn card_shape(c: u8) -> u8 {
    deck()[c as usize].0
}
pub fn card_num(c: u8) -> u8 {
    deck()[c as usize].1
}
pub fn card_text(c: u8) -> String {
    if card_num(c) == 20 {
        "w".into()
    } else {
        format!("{}{}", SHAPE_LETTERS[card_shape(c) as usize], card_num(c))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Play(u8),
    Whot(u8, u8),
    Draw,
    Pass,
}

#[derive(Clone)]
pub struct Whot {
    rng: Rng,
    viewer: Viewer,
    stock: Vec<u8>,
    pile: Vec<u8>,
    hands: [Vec<u8>; 2],
    side: Player,
    req: Option<u8>,
    /// cărți de tras, și numărul care poate înlănțui penalizarea (2 sau 5)
    pending: u8,
    pending_num: u8,
    winner: Option<Player>,
    /// cărți abia trase (ultimele din mână): excluse din evaluarea compoziției
    fresh: [u8; 2],
    /// pasuri consecutive (ambii blocați): jocul se încheie după două
    passes: u8,
    stalled: bool,
    /// mutări jucate; după 600 jocul se încheie (cine are mai puține cărți câștigă)
    plies: u32,
}

impl Default for Whot {
    fn default() -> Self {
        Self::new()
    }
}

impl Whot {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut st: Vec<u8> = (0..deck().len() as u8).collect();
        shuffle(&mut st, &mut rng);
        let mut hands = [Vec::new(), Vec::new()];
        for _ in 0..6 {
            for h in hands.iter_mut() {
                h.push(st.pop().unwrap());
            }
        }
        let mut pile = Vec::new();
        loop {
            let c = st.pop().unwrap();
            let n = card_num(c);
            if matches!(n, 1 | 2 | 5 | 8 | 14 | 20) {
                st.insert(0, c);
                continue;
            }
            pile.push(c);
            break;
        }
        Whot {
            rng,
            viewer: Viewer::Fixed(Player::One),
            stock: st,
            pile,
            hands,
            side: Player::One,
            req: None,
            pending: 0,
            pending_num: 0,
            winner: None,
            fresh: [0, 0],
            passes: 0,
            stalled: false,
            plies: 0,
        }
    }

    fn top(&self) -> u8 {
        *self.pile.last().expect("top card")
    }

    pub fn hand_len(&self, p: Player) -> usize {
        self.hands[p.idx()].len()
    }

    fn matches_top(&self, c: u8) -> bool {
        if card_num(c) == 20 {
            return true;
        }
        let t = self.top();
        match self.req {
            Some(s) => card_shape(c) == s,
            None => card_shape(c) == card_shape(t) || card_num(c) == card_num(t),
        }
    }

    fn can_draw(&self) -> bool {
        !self.stock.is_empty() || self.pile.len() > 1
    }

    fn draw_cards(&mut self, p: Player, n: u8) {
        for _ in 0..n {
            if self.stock.is_empty() && self.pile.len() > 1 {
                let top = self.pile.pop().unwrap();
                let mut rest = std::mem::take(&mut self.pile);
                shuffle(&mut rest, &mut self.rng);
                self.stock = rest;
                self.pile = vec![top];
            }
            if let Some(c) = self.stock.pop() {
                self.hands[p.idx()].push(c);
                self.fresh[p.idx()] += 1;
            }
        }
    }
}

impl Game for Whot {
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
        if self.winner.is_some() || self.stalled {
            return Vec::new();
        }
        let hand = &self.hands[self.side.idx()];
        let mut v = Vec::new();
        let mut seen: Vec<u8> = Vec::new();
        for &c in hand {
            let key = card_shape(c) * 32 + card_num(c);
            if seen.contains(&key) {
                continue;
            }
            seen.push(key);
            if self.pending > 0 {
                if card_num(c) == self.pending_num {
                    v.push(Move::Play(c));
                }
                continue;
            }
            if self.matches_top(c) {
                if card_num(c) == 20 {
                    for s in 0..5 {
                        v.push(Move::Whot(c, s));
                    }
                } else {
                    v.push(Move::Play(c));
                }
            }
        }
        if self.can_draw() {
            v.push(Move::Draw);
        } else if v.is_empty() {
            v.push(Move::Pass);
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        let me = self.side;
        let opp = me.other();
        self.fresh = [0, 0];
        self.plies += 1;
        if self.plies >= 600 {
            self.stalled = true;
        }
        if *m == Move::Pass {
            self.passes += 1;
            if self.passes >= 2 {
                self.stalled = true;
            }
        } else {
            self.passes = 0;
        }
        match *m {
            Move::Draw => {
                let n = if self.pending > 0 { self.pending } else { 1 };
                self.draw_cards(me, n);
                self.pending = 0;
                self.pending_num = 0;
                self.side = opp;
            }
            Move::Pass => {
                self.pending = 0;
                self.side = opp;
            }
            Move::Play(c) | Move::Whot(c, _) => {
                let h = &mut self.hands[me.idx()];
                let pos = h.iter().position(|&x| x == c).expect("card in hand");
                h.remove(pos);
                self.pile.push(c);
                self.req = if let Move::Whot(_, s) = *m {
                    Some(s)
                } else {
                    None
                };
                if self.hands[me.idx()].is_empty() {
                    self.winner = Some(me);
                    return;
                }
                match card_num(c) {
                    1 | 8 => {}
                    14 => {
                        self.draw_cards(opp, 1);
                    }
                    2 => {
                        self.pending += 2;
                        self.pending_num = 2;
                        self.side = opp;
                    }
                    5 => {
                        self.pending += 3;
                        self.pending_num = 5;
                        self.side = opp;
                    }
                    _ => self.side = opp,
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if let Some(w) = self.winner {
            return Some(Outcome::Win(w));
        }
        if self.stalled {
            // ambii blocați (piața goală): câștigă cine are mai puține cărți
            let (a, b) = (self.hands[0].len(), self.hands[1].len());
            return Some(match a.cmp(&b) {
                std::cmp::Ordering::Less => Outcome::Win(Player::One),
                std::cmp::Ordering::Greater => Outcome::Win(Player::Two),
                std::cmp::Ordering::Equal => Outcome::Draw,
            });
        }
        None
    }

    /// Citește doar mâna lui `p`, numărul de cărți al adversarului și informația publică.
    fn evaluate(&self, p: Player) -> i32 {
        if let Some(w) = self.winner {
            return if w == p { 100_000 } else { -100_000 };
        }
        let all = &self.hands[p.idx()];
        let known = all.len() - (self.fresh[p.idx()] as usize).min(all.len());
        let mine = &all[..known];
        let theirs = self.hands[p.other().idx()].len() as i32;
        let mut s = 100 * (theirs - all.len() as i32);
        // cât de ușor îmi pot juca cărțile după carte de deasupra
        let want = self.req.unwrap_or_else(|| card_shape(self.top()));
        let same = mine.iter().filter(|&&c| card_shape(c) == want).count() as i32;
        s += 4 * same;
        // forme diverse în mână = flexibilitate
        let mut shapes = [false; 6];
        for &c in mine {
            shapes[card_shape(c) as usize] = true;
        }
        s += 3 * shapes.iter().filter(|&&b| b).count() as i32;
        // penalizări în așteptare
        if self.pending > 0 {
            s += if self.side == p {
                -10 * self.pending as i32
            } else {
                10 * self.pending as i32
            };
        }
        // să scap de cărțile speciale la final nu e urgent; nu păstra mult Whot
        s
    }

    fn render(&self) -> String {
        let v = self.viewer.who(self.side);
        let mut s = String::new();
        s.push_str(&format!(
            "Deasupra: {}{}\n",
            card_text(self.top()),
            match self.req {
                Some(sh) => format!(" (formă cerută: {})", SHAPE_LETTERS[sh as usize]),
                None => String::new(),
            }
        ));
        if self.pending > 0 {
            s.push_str(&format!(
                "Penalizare în așteptare: {} cărți (răspunde cu {} sau trage).\n",
                self.pending, self.pending_num
            ));
        }
        s.push_str(&format!(
            "Piață: {} cărți; J{} (ascuns) ține {} cărți.\n",
            self.stock.len(),
            v.other().idx() + 1,
            self.hands[v.other().idx()].len()
        ));
        let mut h = self.hands[v.idx()].clone();
        h.sort_unstable_by_key(|&c| (card_shape(c), card_num(c)));
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
        match self.winner {
            Some(p) => s.push_str(&format!(
                "Jucătorul {} a rămas fără cărți și câștigă.\n",
                p.idx() + 1
            )),
            None => s.push_str(&format!("La mutare: Jucător {}\n", self.side.idx() + 1)),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match *m {
            Move::Play(c) => card_text(c),
            Move::Whot(_, s) => format!("w:{}", SHAPE_LETTERS[s as usize]),
            Move::Draw => "trage".into(),
            Move::Pass => "pas".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn find(shape: u8, num: u8) -> u8 {
        deck()
            .iter()
            .position(|&(s, n)| s == shape && n == num)
            .unwrap() as u8
    }

    fn base() -> Whot {
        let mut g = Whot::with_seed(1);
        g.hands = [Vec::new(), Vec::new()];
        g.stock = (0..deck().len() as u8).take(10).collect();
        g.pile = vec![find(0, 3)];
        g.req = None;
        g.pending = 0;
        g.pending_num = 0;
        g.side = Player::One;
        g.winner = None;
        g.fresh = [0, 0];
        g.passes = 0;
        g.stalled = false;
        g
    }

    #[test]
    fn deck_has_54_cards() {
        assert_eq!(deck().len(), 54);
        assert_eq!(deck().iter().filter(|c| c.0 == 5).count(), 5);
    }

    #[test]
    fn deal_is_6_each() {
        let g = Whot::with_seed(4);
        assert_eq!(g.hands[0].len(), 6);
        assert_eq!(g.hands[1].len(), 6);
        assert_eq!(g.stock.len() + g.pile.len() + 12, 54);
        assert!(!matches!(card_num(g.top()), 1 | 2 | 5 | 8 | 14 | 20));
    }

    #[test]
    fn must_match_shape_or_number() {
        let mut g = base();
        g.hands[0] = vec![find(0, 7), find(1, 3), find(2, 10)];
        g.hands[1] = vec![find(4, 1)];
        let mv = g.legal_moves();
        assert!(mv.contains(&Move::Play(find(0, 7))));
        assert!(mv.contains(&Move::Play(find(1, 3))));
        assert!(!mv.contains(&Move::Play(find(2, 10))));
        assert!(mv.contains(&Move::Draw));
    }

    #[test]
    fn whot_requests_a_shape() {
        let mut g = base();
        let w = find(5, 20);
        g.hands[0] = vec![w, find(2, 1)];
        g.hands[1] = vec![find(4, 7), find(3, 5)];
        assert_eq!(
            g.legal_moves()
                .iter()
                .filter(|m| matches!(m, Move::Whot(..)))
                .count(),
            5
        );
        g.apply(&Move::Whot(w, 4));
        assert_eq!(g.to_move(), Player::Two);
        let mv = g.legal_moves();
        assert!(mv.contains(&Move::Play(find(4, 7))));
        assert!(!mv.contains(&Move::Play(find(3, 5))));
    }

    #[test]
    fn pick_two_stacks_and_draw_pays() {
        let mut g = base();
        g.hands[0] = vec![find(0, 2), find(1, 4)];
        g.hands[1] = vec![find(3, 2), find(4, 7)];
        g.apply(&Move::Play(find(0, 2)));
        assert_eq!(g.pending, 2);
        let mv = g.legal_moves();
        assert!(mv.contains(&Move::Play(find(3, 2))));
        assert!(!mv.contains(&Move::Play(find(4, 7))));
        g.apply(&Move::Play(find(3, 2)));
        assert_eq!(g.pending, 4);
        assert_eq!(g.to_move(), Player::One);
        let h0 = g.hands[0].len();
        g.apply(&Move::Draw);
        assert_eq!(g.hands[0].len(), h0 + 4);
        assert_eq!(g.pending, 0);
    }

    #[test]
    fn hold_on_and_market() {
        let mut g = base();
        g.hands[0] = vec![find(0, 1), find(0, 14), find(0, 4)];
        g.hands[1] = vec![find(3, 7)];
        g.apply(&Move::Play(find(0, 1)));
        assert_eq!(g.to_move(), Player::One);
        g.apply(&Move::Play(find(0, 14)));
        assert_eq!(g.to_move(), Player::One);
        assert_eq!(g.hands[1].len(), 2);
    }

    #[test]
    fn empty_hand_wins() {
        let mut g = base();
        g.hands[0] = vec![find(0, 7)];
        g.hands[1] = vec![find(3, 7)];
        g.apply(&Move::Play(find(0, 7)));
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn evaluate_ignores_the_opponents_cards() {
        let g = Whot::with_seed(9);
        // cărțile adversarului (și ale pachetului) nu influențează evaluarea lui `p`
        for p in [Player::One, Player::Two] {
            let q = p.other().idx();
            let mut h = g.clone();
            let (a, b) = (h.hands[q][0], h.stock[3]);
            h.hands[q][0] = b;
            h.stock[3] = a;
            assert_eq!(g.evaluate(p), h.evaluate(p));
        }
    }

    #[test]
    fn render_hides_the_opponent() {
        let g = Whot::with_seed(5);
        assert!(g.render().contains("J2 (ascuns) ține 6 cărți"));
    }

    #[test]
    fn cards_conserved_and_games_finish() {
        for seed in 1..=20u64 {
            let mut g = Whot::with_seed(seed);
            let mut rng = Rng::new(seed ^ 0x77);
            let mut n = 0;
            while g.outcome().is_none() {
                let tot = g.stock.len() + g.pile.len() + g.hands[0].len() + g.hands[1].len();
                assert_eq!(tot, 54);
                let all = g.legal_moves();
                assert!(!all.is_empty());
                // joc aleator care preferă să joace o carte în loc să tragă (altfel plimbarea
                // aleatoare a mărimii mâinii poate dura foarte mult)
                let play: Vec<Move> = all.iter().copied().filter(|m| *m != Move::Draw).collect();
                let m = if play.is_empty() { all } else { play };
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 20_000, "seed {seed}: {}", g.render());
            }
        }
    }

    #[test]
    fn ai_beats_random_mostly() {
        use polytabletop_core::GameObj;
        let mut wins = 0;
        let n = 20;
        for seed in 1..=n {
            let mut g: Box<dyn GameObj> = Box::new(Whot::with_seed(seed));
            let mut rng = Rng::new(seed + 100);
            let mut k = 0;
            while g.outcome().is_none() {
                let mv = if g.to_move() == Player::One {
                    g.ai_move(rng.next_u64()).unwrap()
                } else {
                    let m = g.legal_moves();
                    m[rng.below(m.len())].clone()
                };
                g.play(&mv).unwrap();
                k += 1;
                assert!(k < 20_000);
            }
            if g.outcome() == Some(Outcome::Win(Player::One)) {
                wins += 1;
            }
        }
        assert!(wins * 10 >= n * 6, "AI won only {wins}/{n}");
    }
}
