#![allow(
    clippy::needless_range_loop,
    clippy::manual_is_multiple_of,
    clippy::manual_range_contains,
    clippy::too_many_arguments,
    clippy::doc_lazy_continuation,
    clippy::unnecessary_unwrap,
    clippy::unnecessary_cast,
    clippy::type_complexity
)]
//! Piști (Turcia, „Pişti”) — joc de capturat grămada de pe masă, 2 jucători.
//!
//! Reguli folosite (varianta clasică pentru doi):
//! * 52 de cărți. Pe masă se pun 4 cărți: primele 3 **cu fața în jos** (nu le știe nimeni) și a
//!   patra cu fața în sus (nu poate fi valet — altfel se reamestecă). Fiecare primește 4 cărți;
//!   când mâinile se golesc se dau încă 4 (6 runde de câte 8 cărți). Începe cine nu e dealer.
//! * Joci o carte pe grămada de pe masă. Dacă are **același rang** ca a cărții de deasupra, sau
//!   dacă e un **Valet** (J), iei toată grămada (masa goală: doar pui cartea). Valetul ia tot,
//!   chiar și pe masă cu un singur valet.
//! * „Piști”: iei grămada când are o singură carte și o iei cu aceeași valoare (nu cu valet
//!   peste altă carte) = 10 puncte; valet peste un singur valet = piști dublu, 20 de puncte.
//! * La sfârșit cărțile rămase pe masă merg la ultimul care a capturat.
//! * Puncte pe împărțire: fiecare As 1, fiecare Valet 1, 2♣ 2, 10♦ 3, cele mai multe cărți 3
//!   (la egalitate nimeni), plus piști-urile. Se joacă împărțiri până când cineva ajunge la 101
//!   (mai mare câștigă; la egalitate se mai joacă una).
//! * Mutări: cartea jucată (`7h`, `Js`); `imparte` când mâinile s-au golit.
//! * Informație ascunsă: vezi doar mâna ta și cartea de deasupra. AI-ul nu citește nici cărțile
//!   rămase cu fața în jos, nici mâna adversarului; estimează probabilitatea ca adversarul să
//!   poată captura (din cărțile nevăzute) și, la Hard/Expert, face eșantionare Monte-Carlo.

use crate::cards::Viewer;
use crate::cards52::{std_deck, std_rank, std_suit, std_symbol, std_text};
use crate::p2_util::{
    mc_samples, mc_value, noise, shuffle_vec, sig_of, sort_std, symbols_typed, Mc,
};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const TARGET: i32 = 101;
const R_J: u8 = 9;

fn card_points(c: u8) -> i32 {
    match (std_rank(c), std_suit(c)) {
        (12, _) => 1,
        (R_J, _) => 1,
        (0, 0) => 2,
        (8, 1) => 3,
        _ => 0,
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Play(u8),
    Deal,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Play,
    Deal,
    Over,
}

#[derive(Clone)]
pub struct Pisti {
    stock: Vec<u8>,
    table: Vec<u8>,
    /// Câte cărți de la baza grămezii sunt cu fața în jos.
    face_down: usize,
    hands: [Vec<u8>; 2],
    cap: [Vec<u8>; 2],
    pisti: [i32; 2],
    /// Cărțile care au fost vreodată cu fața în sus (publice).
    seen: Vec<u8>,
    score: [i32; 2],
    last_cap: Option<Player>,
    dealer: Player,
    side: Player,
    deal_no: u8,
    phase: Phase,
    last: String,
    rng: Rng,
    viewer: Viewer,
}

impl Default for Pisti {
    fn default() -> Self {
        Self::new()
    }
}

impl Pisti {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Pisti {
            stock: Vec::new(),
            table: Vec::new(),
            face_down: 0,
            hands: [Vec::new(), Vec::new()],
            cap: [Vec::new(), Vec::new()],
            pisti: [0; 2],
            seen: Vec::new(),
            score: [0; 2],
            last_cap: None,
            dealer: Player::Two,
            side: Player::One,
            deal_no: 1,
            phase: Phase::Play,
            last: String::new(),
            rng: Rng::new(seed),
            viewer: Viewer::Fixed(Player::One),
        };
        g.new_deal();
        g
    }

    fn new_deal(&mut self) {
        loop {
            let mut d = std_deck(&mut self.rng);
            shuffle_vec(&mut d, &mut self.rng);
            let table: Vec<u8> = (0..4).map(|_| d.pop().unwrap()).collect();
            if std_rank(table[3]) == R_J {
                continue;
            }
            self.table = table;
            self.stock = d;
            break;
        }
        self.face_down = 3;
        self.seen = vec![self.table[3]];
        self.cap = [Vec::new(), Vec::new()];
        self.pisti = [0; 2];
        self.last_cap = None;
        self.hands = [Vec::new(), Vec::new()];
        self.side = self.dealer.other();
        self.deal_hands();
        self.phase = Phase::Play;
    }

    fn deal_hands(&mut self) {
        for p in 0..2 {
            for _ in 0..4 {
                if let Some(c) = self.stock.pop() {
                    self.hands[p].push(c);
                }
            }
            sort_std(&mut self.hands[p]);
        }
    }

    pub fn score(&self) -> (i32, i32) {
        (self.score[0], self.score[1])
    }
    pub fn pisti_count(&self) -> (i32, i32) {
        (self.pisti[0], self.pisti[1])
    }
    pub fn set_state_for_test(&mut self, table: Vec<u8>, face_down: usize, a: Vec<u8>, b: Vec<u8>) {
        self.table = table;
        self.face_down = face_down;
        self.hands = [a, b];
        self.seen = self.table.iter().skip(face_down).copied().collect();
    }

    pub fn deal_points(&self) -> [i32; 2] {
        let mut pts = [0i32; 2];
        for p in 0..2 {
            pts[p] += self.cap[p].iter().map(|&c| card_points(c)).sum::<i32>();
            pts[p] += self.pisti[p];
        }
        match self.cap[0].len().cmp(&self.cap[1].len()) {
            std::cmp::Ordering::Greater => pts[0] += 3,
            std::cmp::Ordering::Less => pts[1] += 3,
            std::cmp::Ordering::Equal => {}
        }
        pts
    }

    fn finish_deal(&mut self) {
        if let Some(p) = self.last_cap {
            let rest = std::mem::take(&mut self.table);
            self.cap[p.idx()].extend(rest);
        } else {
            self.table.clear();
        }
        let pts = self.deal_points();
        self.score[0] += pts[0];
        self.score[1] += pts[1];
        self.last = format!(
            "Împărțirea {}: J1 +{}, J2 +{}",
            self.deal_no, pts[0], pts[1]
        );
        let (a, b) = (self.score[0], self.score[1]);
        if (a >= TARGET || b >= TARGET) && a != b {
            self.phase = Phase::Over;
            return;
        }
        self.deal_no += 1;
        self.dealer = self.dealer.other();
        self.new_deal();
    }

    /// Cărțile din grămada lui `p` pe care o poate cunoaște `viewer` (a lui + cele văzute).
    fn known_cap(&self, owner: usize, viewer: usize) -> Vec<u8> {
        if owner == viewer {
            self.cap[owner].clone()
        } else {
            self.cap[owner]
                .iter()
                .copied()
                .filter(|c| self.seen.contains(c))
                .collect()
        }
    }

    fn cap_value(cards: &[u8]) -> i32 {
        cards.iter().map(|&c| card_points(c) * 10 + 2).sum()
    }

    /// Valoarea de puncte (vizibilă) a grămezii de pe masă.
    fn table_value(&self) -> i32 {
        self.table
            .iter()
            .enumerate()
            .filter(|&(i, _)| i >= self.face_down)
            .map(|(_, &c)| card_points(c) * 10 + 2)
            .sum::<i32>()
            + 2 * self.face_down.min(self.table.len()) as i32
    }
}

impl Mc for Pisti {
    fn determinize(&self, viewer: Player, rng: &mut Rng) -> Self {
        let mut g = self.clone();
        let (me, op) = (viewer.idx(), viewer.other().idx());
        let mut pool: Vec<u8> = (0..52u8)
            .filter(|c| {
                !self.hands[me].contains(c) && !self.seen.contains(c) && !self.cap[me].contains(c)
            })
            .collect();
        shuffle_vec(&mut pool, rng);
        // cărți cu fața în jos încă pe masă
        let fd = self.face_down.min(self.table.len());
        for i in 0..fd {
            if let Some(c) = pool.pop() {
                g.table[i] = c;
            }
        }
        let n = self.hands[op].len().min(pool.len());
        g.hands[op] = pool.drain(..n).collect();
        sort_std(&mut g.hands[op]);
        let ns = self.stock.len().min(pool.len());
        g.stock = pool.drain(..ns).collect();
        // restul = cărți ascunse luate de adversar: înlocuiesc cărțile secrete din grămada lui
        let secret: Vec<usize> = (0..self.cap[op].len())
            .filter(|&i| !self.seen.contains(&self.cap[op][i]))
            .collect();
        for (k, i) in secret.into_iter().enumerate() {
            if let Some(&c) = pool.get(k) {
                g.cap[op][i] = c;
            }
        }
        g.rng = Rng::new(rng.next_u64());
        g
    }
    fn epoch(&self) -> u32 {
        self.deal_no as u32
    }
    fn margin(&self, p: Player) -> i32 {
        self.score[p.idx()] - self.score[p.other().idx()]
    }
}

impl Game for Pisti {
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
            Phase::Play => self.hands[self.side.idx()]
                .iter()
                .map(|&c| Move::Play(c))
                .collect(),
        }
    }

    fn apply(&mut self, m: &Move) {
        match *m {
            Move::Deal => {
                self.deal_hands();
                self.phase = Phase::Play;
            }
            Move::Play(c) => {
                let p = self.side;
                let pos = self.hands[p.idx()]
                    .iter()
                    .position(|&x| x == c)
                    .expect("carte în mână");
                self.hands[p.idx()].remove(pos);
                self.seen.push(c);
                let top = self.table.last().copied();
                let captures = match top {
                    None => false,
                    Some(t) => std_rank(c) == R_J || std_rank(c) == std_rank(t),
                };
                if captures {
                    let t = top.unwrap();
                    let single = self.table.len() == 1;
                    let mut note = String::new();
                    if single && std_rank(c) == std_rank(t) {
                        let pts = if std_rank(c) == R_J { 20 } else { 10 };
                        self.pisti[p.idx()] += pts;
                        note = format!(" — PIȘTI ({pts} p.)!");
                    }
                    let pile = std::mem::take(&mut self.table);
                    self.face_down = 0;
                    self.cap[p.idx()].push(c);
                    self.cap[p.idx()].extend(pile);
                    self.last_cap = Some(p);
                    self.last = format!("J{} ia grămada cu {}{}", p.idx() + 1, std_text(c), note);
                } else {
                    self.table.push(c);
                    self.last = format!("J{} pune {}", p.idx() + 1, std_text(c));
                }
                self.side = p.other();
                if self.hands[0].is_empty() && self.hands[1].is_empty() {
                    if self.stock.is_empty() {
                        self.finish_deal();
                    } else {
                        self.phase = Phase::Deal;
                        self.side = self.dealer.other();
                    }
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        Some(if self.score[0] > self.score[1] {
            Outcome::Win(Player::One)
        } else {
            Outcome::Win(Player::Two)
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let (me, op) = (p.idx(), p.other().idx());
        let mut s = 1000 * (self.score[me] - self.score[op]);
        if self.phase == Phase::Over {
            return s;
        }
        s += 3
            * (Self::cap_value(&self.known_cap(me, me)) - Self::cap_value(&self.known_cap(op, me)));
        s += 10 * (self.pisti[me] - self.pisti[op]);
        // grămada de pe masă: dacă urmează adversarul, el o poate lua
        if self.phase == Phase::Play && self.side != p {
            if let Some(&t) = self.table.last() {
                // câte cărți de același rang + valeți mai sunt nevăzute
                let unseen = |pred: &dyn Fn(u8) -> bool| -> i32 {
                    (0..52u8)
                        .filter(|&c| {
                            pred(c) && !self.seen.contains(&c) && !self.hands[me].contains(&c)
                        })
                        .count() as i32
                };
                let k = unseen(&|c| std_rank(c) == std_rank(t) || std_rank(c) == R_J);
                let u = unseen(&|_| true).max(1);
                let n = self.hands[op].len().max(1) as i32;
                // P(adversarul poate lua) ≈ 1 − (1 − k/u)^n
                let q = k as f64 / u as f64;
                let pr = 1.0 - (1.0 - q).powi(n);
                let bonus = if self.table.len() == 1 { 100 } else { 0 };
                s -= (pr * (self.table_value() + bonus) as f64) as i32;
            }
        }
        for &c in &self.hands[me] {
            s += card_points(c) * 3;
        }
        let sig = sig_of(&[
            self.hands[me]
                .iter()
                .fold(0u64, |a, &c| a.wrapping_mul(53).wrapping_add(c as u64 + 1)),
            self.table.len() as u64,
            self.seen.len() as u64,
            self.score[0] as u64,
        ]);
        s += noise(sig, 40);
        s + mc_value(self, p, mc_samples(), sig)
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Piști — împărțirea {} | puncte: J1 {}, J2 {} (până la {}) | stoc {}\n",
            self.deal_no,
            self.score[0],
            self.score[1],
            TARGET,
            self.stock.len()
        );
        if !self.last.is_empty() {
            s.push_str(&format!("({})\n", self.last));
        }
        if self.phase == Phase::Over {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        let top = self
            .table
            .last()
            .map(|&c| std_symbol(c))
            .unwrap_or_else(|| "—".into());
        s.push_str(&format!(
            "Masa: {} cărți, deasupra {}{}\n",
            self.table.len(),
            top,
            if self.face_down > 0 && self.face_down <= self.table.len() {
                format!(" ({} cu fața în jos)", self.face_down)
            } else {
                String::new()
            }
        ));
        s.push_str(&format!(
            "Grămezi: J1 {} cărți ({} p. piști), J2 {} cărți ({} p. piști)\n",
            self.cap[0].len(),
            self.pisti[0],
            self.cap[1].len(),
            self.pisti[1]
        ));
        let v = self.viewer.who(self.side);
        s.push_str(&format!(
            "Mâna ta (J{}): {}\n",
            v.idx() + 1,
            symbols_typed(&self.hands[v.idx()])
        ));
        s.push_str(&format!(
            "Jucătorul {}: {} cărți\n",
            v.other().idx() + 1,
            self.hands[v.other().idx()].len()
        ));
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Deal => "imparte".into(),
            Move::Play(c) => std_text(*c),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards52::SUIT_LETTERS;

    fn c(rank: u8, suit: char) -> u8 {
        let s = SUIT_LETTERS.iter().position(|&x| x == suit).unwrap() as u8;
        s * 13 + (rank - 2)
    }

    #[test]
    fn match_by_rank_and_jack_capture() {
        let mut g = Pisti::with_seed(1);
        g.set_state_for_test(
            vec![c(5, 'c'), c(9, 'd'), c(7, 'h')],
            1,
            vec![c(7, 's'), c(3, 'd')],
            vec![c(11, 'h'), c(2, 'd')],
        );
        g.side = Player::One;
        g.apply(&Move::Play(c(7, 's')));
        // a luat 3 cărți + a ei
        assert_eq!(g.cap[0].len(), 4);
        assert!(g.table.is_empty());
        assert_eq!(g.pisti_count(), (0, 0));
        // masa goală: doar pune
        g.apply(&Move::Play(c(11, 'h')));
        assert_eq!(g.table, vec![c(11, 'h')]);
        // valetul ia orice
        g.table = vec![c(4, 'c'), c(8, 'd')];
        g.side = Player::One;
        g.hands[0] = vec![c(11, 'c'), c(3, 'c')];
        g.apply(&Move::Play(c(11, 'c')));
        assert!(g.table.is_empty());
        assert_eq!(g.cap[0].len(), 4 + 3);
    }

    #[test]
    fn pisti_and_double_pisti() {
        let mut g = Pisti::with_seed(1);
        g.set_state_for_test(
            vec![c(6, 'h')],
            0,
            vec![c(6, 's'), c(3, 'd')],
            vec![c(2, 'd'), c(5, 'd')],
        );
        g.side = Player::One;
        g.apply(&Move::Play(c(6, 's')));
        assert_eq!(g.pisti_count(), (10, 0));
        // valet peste valet singur = 20
        let mut g = Pisti::with_seed(1);
        g.set_state_for_test(
            vec![c(11, 'h')],
            0,
            vec![c(11, 's'), c(3, 'd')],
            vec![c(2, 'd'), c(5, 'd')],
        );
        g.side = Player::One;
        g.apply(&Move::Play(c(11, 's')));
        assert_eq!(g.pisti_count(), (20, 0));
        // valet peste o singură carte obișnuită: nu e piști
        let mut g = Pisti::with_seed(1);
        g.set_state_for_test(
            vec![c(6, 'h')],
            0,
            vec![c(11, 's'), c(3, 'd')],
            vec![c(2, 'd'), c(5, 'd')],
        );
        g.side = Player::One;
        g.apply(&Move::Play(c(11, 's')));
        assert_eq!(g.pisti_count(), (0, 0));
        assert_eq!(g.cap[0].len(), 2);
    }

    #[test]
    fn scoring_points() {
        let mut g = Pisti::with_seed(2);
        g.cap = [
            vec![c(10, 'd'), c(2, 'c'), c(14, 'h'), c(11, 's'), c(3, 'h')],
            vec![c(14, 'c')],
        ];
        g.pisti = [10, 0];
        // 3 + 2 + 1 + 1 + piști 10 + majoritate 3 = 20 ; J2: As 1
        assert_eq!(g.deal_points(), [20, 1]);
    }

    #[test]
    fn hidden_table_cards_never_in_render_or_eval() {
        let g = Pisti::with_seed(5);
        let r = Game::render(&g);
        for &x in &g.table[..3] {
            assert!(!r.contains(&format!("[{}]", std_text(x))), "{r}");
        }
        assert!(r.contains("3 cu fața în jos"), "{r}");
        // evaluarea nu depinde de cărțile cu fața în jos
        let mut a = g.clone();
        let mut b = g.clone();
        // schimbă identitatea unei cărți cu fața în jos (cu una din stoc)
        b.table[0] = b.stock[0];
        a.side = Player::Two;
        b.side = Player::Two;
        assert_eq!(
            Game::evaluate(&a, Player::One),
            Game::evaluate(&b, Player::One)
        );
    }

    #[test]
    fn full_deal_conserves_and_ai_finishes() {
        let mut g = Pisti::with_seed(9);
        let mut rng = Rng::new(3);
        for _ in 0..60 {
            let mv = Game::legal_moves(&g);
            g.apply(&mv[rng.below(mv.len())]);
            let total = g.stock.len()
                + g.table.len()
                + g.hands[0].len()
                + g.hands[1].len()
                + g.cap[0].len()
                + g.cap[1].len();
            assert_eq!(total, 52);
        }
        for seed in 1..3u64 {
            let mut g = Pisti::with_seed(seed);
            let mut n = 0;
            while g.outcome().is_none() && n < 8000 {
                let m = polytabletop_core::ai::best_move(&g, n).unwrap();
                g.apply(&m);
                n += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed}");
        }
    }
}
