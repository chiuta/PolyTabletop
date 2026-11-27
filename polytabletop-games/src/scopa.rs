//! Scopa (Italia), 2 jucători, pachet italian de 40 de cărți, din regulile publice.
//!
//! * Culori: d = denari, c = coppe, s = spade, b = bastoni. Rangurile 1–7, F (fante, 8),
//!   C (cavallo, 9), R (re, 10) valorează 1–10 la adunare.
//! * Se dau 4 cărți pe masă și câte 3 fiecăruia; când mâinile se golesc se dau
//!   încă 3 (mutare forțată `imparte`) până se termină pachetul. Începe
//!   jucătorul care nu a împărțit.
//! * La rândul tău joci o carte. Dacă pe masă e o carte de aceeași valoare, **trebuie**
//!   să iei una dintre ele (nu o sumă). Altfel poți / trebuie să iei un grup de cărți
//!   de pe masă a căror sumă e valoarea cărții tale. Dacă nu poți lua, o lași pe masă.
//!   Mutări: `7d` (lasă), `7d:3c+4b` (ia 3c și 4b), `7d:7s` (ia 7s).
//! * **Scopa**: golești masa (1 punct), dar nu la ultima carte a mâinii; ultimul care
//!   a luat primește cărțile rămase pe masă (fără scopa).
//! * Punctaj la sfârșitul rundei: *carte* (cele mai multe cărți, peste 20), *denari*
//!   (cei mai mulți, peste 5), *settebello* (7d), *primiera* (suma celor mai bune
//!   cărți din fiecare culoare: 7=21, 6=18, 1=16, 5=15, 4=14, 3=13, 2=12, figuri=10;
//!   culoare lipsă = 0) și 1 punct pe scopa. Meciul se joacă până la 11 puncte
//!   (la egalitate peste 11 se mai joacă o rundă).
//!
//! **Chkobba** (Tunisia / Algeria) folosește același motor (`Rules::Chkobba`): aceleași
//! capturi, aceeași golire a mesei și același pachet de 40 de cărți (1–7, 8 = damă,
//! 9 = valet, 10 = rege); diferă punctajul rundei: *bermila* (cele mai multe de 7; la
//! egalitate 2–2, cele mai multe de 6; dacă și acelea sunt egale, nimeni) ia locul
//! primierei, iar *haya* (7 de denari = settebello) valorează tot un punct. Un punct și
//! pentru *chkobba* (golirea mesei, nu la ultima carte a rundei).
//!
//! Informație ascunsă: vezi `cards52.rs`. Cărțile dintr-o nouă împărțire se dau
//! într-o mutare separată, deci `evaluate` nu vede viitorul. Nu se aplică regula
//! de re-împărțire pentru trei sau mai mulți regi pe masă.

use crate::cards::Viewer;
use crate::cards52::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const RANK_LABEL: [&str; 10] = ["1", "2", "3", "4", "5", "6", "7", "F", "C", "R"];
const SUIT_LETTERS: [char; 4] = ['d', 'c', 's', 'b'];
const TARGET: u8 = 11;
const SETTEBELLO: u8 = 6;

pub fn suit(c: u8) -> u8 {
    c / 10
}
pub fn rank(c: u8) -> u8 {
    c % 10
}
/// Valoarea la adunare, 1..=10.
pub fn value(c: u8) -> u8 {
    rank(c) + 1
}
pub fn card_text(c: u8) -> String {
    format!(
        "{}{}",
        RANK_LABEL[rank(c) as usize],
        SUIT_LETTERS[suit(c) as usize]
    )
}
fn prim_value(c: u8) -> u32 {
    match value(c) {
        7 => 21,
        6 => 18,
        1 => 16,
        5 => 15,
        4 => 14,
        3 => 13,
        2 => 12,
        _ => 10,
    }
}

/// Suma primierei: cea mai bună carte din fiecare culoare.
pub fn primiera(pile: &[u8]) -> u32 {
    (0..4u8)
        .map(|s| {
            pile.iter()
                .filter(|&&c| suit(c) == s)
                .map(|&c| prim_value(c))
                .max()
                .unwrap_or(0)
        })
        .sum()
}

#[derive(Clone, PartialEq, Debug)]
pub enum Move {
    /// Cartea jucată și cărțile luate (sortate); gol = o lași pe masă.
    Play(u8, Vec<u8>),
    Deal,
    Next,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Play,
    NeedDeal,
    RoundOver,
}

/// Varianta jucată cu acest motor.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Rules {
    Scopa,
    Chkobba,
}

#[derive(Clone)]
pub struct Scopa {
    rules: Rules,
    rng: Rng,
    viewer: Viewer,
    stock: Vec<u8>,
    hands: [Vec<u8>; 2],
    table: Vec<u8>,
    piles: [Vec<u8>; 2],
    scopas: [u8; 2],
    last_capture: Option<Player>,
    side: Player,
    dealer: Player,
    phase: Phase,
    score: [u8; 2],
    /// Punctele ultimei runde: [carte, denari, settebello, primiera, scope] pe jucător.
    last_round: Option<[[u8; 5]; 2]>,
}

impl Default for Scopa {
    fn default() -> Self {
        Self::new()
    }
}

impl Scopa {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Self::with_rules(seed, Rules::Scopa)
    }

    /// Chkobba (Tunisia), seed din ceas.
    pub fn chkobba() -> Self {
        Self::with_rules(Rng::from_clock().next_u64(), Rules::Chkobba)
    }

    pub fn with_rules(seed: u64, rules: Rules) -> Self {
        let mut g = Scopa {
            rules,
            rng: Rng::new(seed),
            viewer: Viewer::Fixed(Player::One),
            stock: Vec::new(),
            hands: [Vec::new(), Vec::new()],
            table: Vec::new(),
            piles: [Vec::new(), Vec::new()],
            scopas: [0; 2],
            last_capture: None,
            side: Player::One,
            dealer: Player::Two,
            phase: Phase::Play,
            score: [0; 2],
            last_round: None,
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
        self.scopas = [0; 2];
        self.last_capture = None;
        self.side = self.dealer.other();
        self.phase = Phase::Play;
    }

    pub fn score(&self) -> (u8, u8) {
        (self.score[0], self.score[1])
    }
    pub fn table(&self) -> &[u8] {
        &self.table
    }
    pub fn hand_of(&self, p: Player) -> &[u8] {
        &self.hands[p.idx()]
    }
    pub fn stock_len(&self) -> usize {
        self.stock.len()
    }
    pub fn pile_of(&self, p: Player) -> &[u8] {
        &self.piles[p.idx()]
    }

    fn match_over(&self) -> bool {
        (self.score[0] >= TARGET || self.score[1] >= TARGET) && self.score[0] != self.score[1]
    }

    /// Grupurile de cărți de pe masă pe care le poate lua cartea `c`.
    fn captures(&self, c: u8) -> Vec<Vec<u8>> {
        let v = value(c);
        let same: Vec<Vec<u8>> = self
            .table
            .iter()
            .filter(|&&t| value(t) == v)
            .map(|&t| vec![t])
            .collect();
        if !same.is_empty() {
            return same;
        }
        let mut t = self.table.clone();
        t.sort_unstable();
        let mut out = Vec::new();
        let mut cur = Vec::new();
        fn rec(t: &[u8], i: usize, left: u8, cur: &mut Vec<u8>, out: &mut Vec<Vec<u8>>) {
            if left == 0 {
                out.push(cur.clone());
                return;
            }
            for j in i..t.len() {
                let vj = value(t[j]);
                if vj <= left {
                    cur.push(t[j]);
                    rec(t, j + 1, left - vj, cur, out);
                    cur.pop();
                }
            }
        }
        rec(&t, 0, v, &mut cur, &mut out);
        out
    }

    fn round_points(&self) -> [[u8; 5]; 2] {
        let mut r = [[0u8; 5]; 2];
        let n = [self.piles[0].len(), self.piles[1].len()];
        if n[0] != n[1] {
            r[usize::from(n[1] > n[0])][0] = 1;
        }
        let den = [
            self.piles[0].iter().filter(|&&c| suit(c) == 0).count(),
            self.piles[1].iter().filter(|&&c| suit(c) == 0).count(),
        ];
        if den[0] != den[1] {
            r[usize::from(den[1] > den[0])][1] = 1;
        }
        for (row, pile) in r.iter_mut().zip(&self.piles) {
            if pile.contains(&SETTEBELLO) {
                row[2] = 1;
            }
        }
        if self.rules == Rules::Chkobba {
            let cnt = |p: usize, v: u8| self.piles[p].iter().filter(|&&c| value(c) == v).count();
            let (s0, s1) = (cnt(0, 7), cnt(1, 7));
            let winner = if s0 != s1 {
                Some(usize::from(s1 > s0))
            } else {
                let (x0, x1) = (cnt(0, 6), cnt(1, 6));
                (x0 != x1).then(|| usize::from(x1 > x0))
            };
            if let Some(w) = winner {
                r[w][3] = 1;
            }
        } else {
            let pr = [primiera(&self.piles[0]), primiera(&self.piles[1])];
            if pr[0] != pr[1] {
                r[usize::from(pr[1] > pr[0])][3] = 1;
            }
        }
        r[0][4] = self.scopas[0];
        r[1][4] = self.scopas[1];
        r
    }

    fn finish_round(&mut self) {
        if let Some(p) = self.last_capture {
            let rest = std::mem::take(&mut self.table);
            self.piles[p.idx()].extend(rest);
        }
        let r = self.round_points();
        for (sc, row) in self.score.iter_mut().zip(&r) {
            *sc += row.iter().sum::<u8>();
        }
        self.last_round = Some(r);
        self.phase = Phase::RoundOver;
    }
}

impl Game for Scopa {
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
                let mut v = Vec::new();
                for &c in &self.hands[self.side.idx()] {
                    let caps = self.captures(c);
                    if caps.is_empty() {
                        v.push(Move::Play(c, Vec::new()));
                    } else {
                        for cap in caps {
                            v.push(Move::Play(c, cap));
                        }
                    }
                }
                v
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
                for p in 0..2 {
                    for _ in 0..3 {
                        if let Some(c) = self.stock.pop() {
                            self.hands[p].push(c);
                        }
                    }
                }
                self.phase = Phase::Play;
            }
            Move::Play(c, take) => {
                let me = self.side;
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
                    self.scopas[me.idx()] += 1;
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

    /// Citește doar informația publică (mese, grămezi, scope, scor) — niciodată
    /// mâna adversarului sau pachetul.
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let o = 1 - me;
        let mut s = (self.score[me] as i32 - self.score[o] as i32) * 1000;
        if self.phase == Phase::RoundOver {
            return s;
        }
        s += (self.scopas[me] as i32 - self.scopas[o] as i32) * 100;
        let cnt = |q: usize| self.piles[q].len() as i32;
        let den = |q: usize| self.piles[q].iter().filter(|&&c| suit(c) == 0).count() as i32;
        s += (cnt(me) - cnt(o)) * 8;
        s += (den(me) - den(o)) * 12;
        let sb = |q: usize| i32::from(self.piles[q].contains(&SETTEBELLO));
        s += (sb(me) - sb(o)) * 70;
        if self.rules == Rules::Chkobba {
            let sev = |q: usize| self.piles[q].iter().filter(|&&c| value(c) == 7).count() as i32;
            s += (sev(me) - sev(o)) * 25;
        } else {
            s += (primiera(&self.piles[me]) as i32 - primiera(&self.piles[o]) as i32) * 2;
        }
        // cartea rămasă pe masă poate fi luată de adversar
        let sum: i32 = self.table.iter().map(|&c| value(c) as i32).sum();
        let exposed = if self.phase == Phase::Play && self.side != p {
            1
        } else {
            0
        };
        if exposed == 1 {
            if !self.table.is_empty() && sum <= 10 {
                s -= 40;
            }
            if self.table.contains(&SETTEBELLO) {
                s -= 60;
            }
            for &c in &self.table {
                s -= 6 + if suit(c) == 0 { 8 } else { 0 } + if value(c) == 7 { 6 } else { 0 };
            }
        }
        s
    }

    fn render(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            "{} — meci până la {TARGET}: J1 {} – J2 {}\n",
            if self.rules == Rules::Chkobba {
                "Chkobba"
            } else {
                "Scopa"
            },
            self.score[0],
            self.score[1]
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
        s.push_str(&format!(
            "Pachet: {} cărți. J1 a luat {} cărți ({} scope), J2 a luat {} cărți ({} scope); J{} (ascuns) ține {} cărți.\n",
            self.stock.len(),
            self.piles[0].len(),
            self.scopas[0],
            self.piles[1].len(),
            self.scopas[1],
            self.viewer.who(self.side).other().idx() + 1,
            self.hands[self.viewer.who(self.side).other().idx()].len()
        ));
        let v = self.viewer.who(self.side);
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
        if let Some(r) = &self.last_round {
            if self.phase == Phase::RoundOver {
                for (i, x) in r.iter().enumerate() {
                    s.push_str(&format!(
                        "Runda: J{} — carte {}, denari {}, {} {}, {} {}, {} {}\n",
                        i + 1,
                        x[0],
                        x[1],
                        if self.rules == Rules::Chkobba {
                            "haya"
                        } else {
                            "settebello"
                        },
                        x[2],
                        if self.rules == Rules::Chkobba {
                            "bermila"
                        } else {
                            "primiera"
                        },
                        x[3],
                        if self.rules == Rules::Chkobba {
                            "chkobba"
                        } else {
                            "scope"
                        },
                        x[4]
                    ));
                }
            }
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
            Move::Play(c, take) if take.is_empty() => card_text(*c),
            Move::Play(c, take) => format!(
                "{}:{}",
                card_text(*c),
                take.iter()
                    .map(|&t| card_text(t))
                    .collect::<Vec<_>>()
                    .join("+")
            ),
        }
    }

    /// Adâncime 0 (o singură mutare): `AlphaBeta(1)` ar fi extins la Hard/Expert răspunsul adversarului din
    /// mâna lui reală (și noua împărțire din pachet); evaluarea citește doar datele publice.
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // d: 0..9, c: 10..19, s: 20..29, b: 30..39; rang = valoare - 1
    fn d(v: u8) -> u8 {
        v - 1
    }
    fn cc(v: u8) -> u8 {
        10 + v - 1
    }
    fn ss(v: u8) -> u8 {
        20 + v - 1
    }
    fn bb(v: u8) -> u8 {
        30 + v - 1
    }

    fn base() -> Scopa {
        let mut g = Scopa::with_seed(1);
        g.stock.clear();
        g.hands = [Vec::new(), Vec::new()];
        g.table.clear();
        g.piles = [Vec::new(), Vec::new()];
        g.scopas = [0; 2];
        g.side = Player::One;
        g.phase = Phase::Play;
        g
    }

    #[test]
    fn initial_deal_is_consistent() {
        let g = Scopa::with_seed(2);
        assert_eq!(g.table.len(), 4);
        assert_eq!(g.hands[0].len(), 3);
        assert_eq!(g.hands[1].len(), 3);
        assert_eq!(g.stock.len(), 30);
        assert_eq!(g.to_move(), Player::One);
    }

    #[test]
    fn same_value_must_be_taken_not_a_sum() {
        let mut g = base();
        g.table = vec![cc(3), ss(4), bb(7), d(3)];
        g.hands[0] = vec![ss(7)];
        g.hands[1] = vec![d(1)];
        // 7 = 3+4 dar există un 7 pe masă: doar 7b
        let mv = g.legal_moves();
        assert_eq!(mv, vec![Move::Play(ss(7), vec![bb(7)])]);
    }

    #[test]
    fn sums_are_enumerated() {
        let mut g = base();
        g.table = vec![d(1), cc(2), ss(3), bb(4), d(5)];
        g.hands[0] = vec![ss(6)];
        g.hands[1] = vec![d(1)];
        let mv = g.legal_moves();
        // 6 = 1+5 = 2+4 = 1+2+3
        assert_eq!(mv.len(), 3);
        assert!(mv.iter().all(
            |m| matches!(m, Move::Play(_, t) if t.iter().map(|&x| value(x)).sum::<u8>() == 6)
        ));
    }

    #[test]
    fn no_capture_means_drop() {
        let mut g = base();
        g.table = vec![d(10)];
        g.hands[0] = vec![ss(5)];
        g.hands[1] = vec![d(1)];
        assert_eq!(g.legal_moves(), vec![Move::Play(ss(5), vec![])]);
        let mv0 = g.legal_moves()[0].clone();
        g.apply(&mv0);
        assert_eq!(g.table.len(), 2);
    }

    #[test]
    fn scopa_scores_but_not_on_the_last_card() {
        let mut g = base();
        g.stock = vec![d(2)];
        g.table = vec![cc(5)];
        g.hands[0] = vec![ss(5), ss(1)];
        g.hands[1] = vec![d(1)];
        g.apply(&Move::Play(ss(5), vec![cc(5)]));
        assert_eq!(g.scopas[0], 1);
        // ultima carte a rundei: fără scopa
        let mut h = base();
        h.table = vec![cc(5)];
        h.hands[0] = vec![ss(5)];
        h.apply(&Move::Play(ss(5), vec![cc(5)]));
        assert_eq!(h.scopas[0], 0);
        assert_eq!(h.phase, Phase::RoundOver);
    }

    #[test]
    fn last_capturer_gets_the_table() {
        let mut g = base();
        g.table = vec![cc(5), d(10)];
        g.hands[0] = vec![ss(5)];
        g.hands[1] = vec![bb(1)];
        g.apply(&Move::Play(ss(5), vec![cc(5)]));
        g.apply(&Move::Play(bb(1), vec![]));
        assert_eq!(g.phase, Phase::RoundOver);
        assert!(g.table.is_empty());
        assert_eq!(g.piles[0].len(), 2 + 2);
        assert_eq!(g.piles[1].len(), 0);
    }

    #[test]
    fn deal_is_a_separate_move() {
        let mut g = base();
        g.stock = vec![d(1), d(2), d(3), d(4), d(5), d(6)];
        g.table = vec![cc(9)];
        g.hands[0] = vec![ss(5)];
        g.hands[1] = vec![bb(1)];
        g.apply(&Move::Play(ss(5), vec![]));
        g.apply(&Move::Play(bb(1), vec![]));
        assert_eq!(g.phase, Phase::NeedDeal);
        assert_eq!(g.legal_moves(), vec![Move::Deal]);
        assert_eq!(g.hands[0].len(), 0);
        g.apply(&Move::Deal);
        assert_eq!(g.hands[0].len(), 3);
        assert_eq!(g.hands[1].len(), 3);
        assert!(g.stock.is_empty());
        assert_eq!(g.to_move(), g.dealer.other());
    }

    #[test]
    fn primiera_and_round_scoring() {
        assert_eq!(primiera(&[d(7), cc(7), ss(7), bb(7)]), 84);
        assert_eq!(primiera(&[d(7), d(6)]), 21);
        let mut g = base();
        g.piles[0] = vec![d(7), d(1), d(2), d(3), d(4), d(5), cc(7), ss(7), bb(7)];
        g.piles[1] = vec![cc(1), cc(2)];
        g.scopas = [1, 0];
        let r = g.round_points();
        // carte: 9 contra 2 -> J1; denari: 6 contra 0 -> J1; settebello J1; primiera J1; scopa 1
        assert_eq!(r[0], [1, 1, 1, 1, 1]);
        assert_eq!(r[1], [0, 0, 0, 0, 0]);
        // egalitate la carte: nimeni nu ia punctul
        g.piles[1] = (0..9).map(|i| bb(1 + i % 10)).collect();
        let r2 = g.round_points();
        assert_eq!(r2[0][0] + r2[1][0], 0);
    }

    #[test]
    fn chkobba_bermila_counts_sevens_then_sixes() {
        let mut g = Scopa::with_rules(1, Rules::Chkobba);
        g.piles[0] = vec![d(7), cc(7), ss(7)];
        g.piles[1] = vec![bb(7), d(1)];
        let r = g.round_points();
        assert_eq!(r[0][3], 1);
        assert_eq!(r[1][3], 0);
        // 2–2 la șapte: decid șasele
        g.piles[0] = vec![d(7), cc(7), d(6), cc(6), ss(6)];
        g.piles[1] = vec![ss(7), bb(7), bb(6)];
        let r = g.round_points();
        assert_eq!(r[0][3], 1);
        // egal și la șase: nimeni
        g.piles[0] = vec![d(7), cc(7), d(6)];
        g.piles[1] = vec![ss(7), bb(7), bb(6)];
        let r = g.round_points();
        assert_eq!(r[0][3] + r[1][3], 0);
        // haya = 7 de denari
        assert_eq!(r[0][2], 1);
    }

    #[test]
    fn chkobba_matches_finish_and_render_names_it() {
        for seed in 1..=8u64 {
            let mut g = Scopa::with_rules(seed, Rules::Chkobba);
            assert!(Game::render(&g).starts_with("Chkobba"));
            let mut rng = Rng::new(seed ^ 0x33);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 20000);
            }
        }
    }

    #[test]
    fn cards_are_conserved_and_matches_finish() {
        for seed in 1..=12u64 {
            let mut g = Scopa::with_seed(seed);
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
                assert!(n < 20000);
            }
        }
    }

    #[test]
    fn render_hides_the_opponent() {
        let g = Scopa::with_seed(5);
        let r = Game::render(&g);
        assert!(r.contains("J2 (ascuns) ține 3 cărți"));
    }

    #[test]
    fn evaluate_ignores_the_opponents_cards() {
        let g = Scopa::with_seed(9);
        let mut h = g.clone();
        let (a, b) = (h.hands[1][0], h.stock[3]);
        h.hands[1][0] = b;
        h.stock[3] = a;
        for p in [Player::One, Player::Two] {
            assert_eq!(g.evaluate(p), h.evaluate(p));
        }
    }

    #[test]
    fn ai_beats_a_random_mover_mostly() {
        use polytabletop_core::GameObj;
        let mut wins = 0;
        let n = 16;
        for seed in 1..=n {
            let mut g: Box<dyn GameObj> = Box::new(Scopa::with_seed(seed));
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
        assert!(wins * 10 >= n * 8, "AI won only {wins}/{n}");
    }
}

#[cfg(test)]
mod a2_audit {
    use super::*;

    #[test]
    fn ai_nu_vede_mana_adversarului_nici_pachetul_la_orice_nivel() {
        for (name, rules) in [("scopa", Rules::Scopa), ("chkobba", Rules::Chkobba)] {
            crate::p3_comun::ai_blind_check(
                name,
                |s| Scopa::with_rules(s, rules),
                |g, me, seed| {
                    let o = me.other().idx();
                    let n = g.hands[o].len();
                    let mut pool: Vec<u8> = g.hands[o].clone();
                    pool.extend(g.stock.iter().copied());
                    crate::p3_comun::shuffle_with(&mut pool, seed);
                    g.hands[o] = pool.drain(..n).collect();
                    g.stock = pool;
                    g.rng = Rng::new(seed ^ 0x5C);
                },
                12,
            );
        }
    }
}

#[cfg(test)]
mod a1_blind {
    use super::*;
    use crate::ai_audit_a1::{audit, scramble_pool};

    #[test]
    fn ai_ignores_opponent_hand_and_stock_at_all_levels() {
        audit("scopa", Scopa::with_seed, 1..=4, 3, 70, |g, salt| {
            let foe = g.side.other().idx();
            scramble_pool(&mut g.hands[foe], &mut g.stock, salt);
            g.rng = Rng::new(salt ^ 3);
        });
    }
}
