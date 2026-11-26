//! Solitaire (Klondike) — duel pe două mese independente (câte o amestecare pentru fiecare jucător).
//!
//! * Regulile clasice Klondike cu tragere de câte o carte: 7 coloane (1…7 cărți, ultima cu fața în
//!   sus), 4 fundații (A→K pe culoare), pachet + deșeu (la golire, deșeul se întoarce). Pe coloane se
//!   construiește descrescător, alternând culorile roșu/negru; în locurile goale intră doar un rege;
//!   se mută și șiruri întregi cu fața în sus.
//! * Duelul: jucătorii fac pe rând câte o mutare pe masa lor (J1 și J2 au amestecări diferite, ca să nu
//!   afle unul din celălalt cărțile ascunse). Câștigă cine termină primul cele 52 de cărți pe fundații;
//!   după 240 de mutări în total câștigă cine are mai multe cărți pe fundații (egal = remiză).
//! * AI-ul nu cunoaște cărțile cu fața în jos: evaluează doar numărul lor și cărțile vizibile.
//!   Mutări: `d` (trag din pachet / întorc deșeul), `w>f`, `w>3` (deșeu → coloana 3), `3>f`, `3>5:2`
//!   (două cărți din coloana 3 pe coloana 5).

use crate::cards52::shuffle;
use crate::q5_util::{by_score, verdict};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const LABELS: [&str; 13] = [
    "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
];
const SUITS: [char; 4] = ['h', 'd', 's', 'c'];
const CAP: u16 = 240;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Draw,
    WasteToFound,
    WasteToCol(u8),
    ColToFound(u8),
    ColToCol(u8, u8, u8),
}

#[derive(Clone)]
struct Board {
    tab: Vec<Vec<(u8, bool)>>,
    found: [u8; 4],
    stock: Vec<u8>,
    waste: Vec<u8>,
}

fn rank(c: u8) -> u8 {
    c % 13
}
fn suit(c: u8) -> u8 {
    c / 13
}
fn red(c: u8) -> bool {
    suit(c) < 2
}
fn text(c: u8) -> String {
    format!("{}{}", LABELS[rank(c) as usize], SUITS[suit(c) as usize])
}

impl Board {
    fn new(seed: u64) -> Board {
        let mut d: Vec<u8> = (0..52u8).collect();
        shuffle(&mut d, &mut Rng::new(seed));
        let mut tab = Vec::new();
        for i in 0..7 {
            let mut col = Vec::new();
            for j in 0..=i {
                col.push((d.pop().unwrap(), j == i));
            }
            tab.push(col);
        }
        Board {
            tab,
            found: [0; 4],
            stock: d,
            waste: Vec::new(),
        }
    }

    fn found_total(&self) -> i32 {
        self.found.iter().map(|&x| x as i32).sum()
    }

    fn facedown(&self) -> i32 {
        self.tab.iter().flatten().filter(|c| !c.1).count() as i32
    }

    fn fits(&self, c: u8, col: usize) -> bool {
        match self.tab[col].last() {
            None => rank(c) == 12,
            Some(&(t, _)) => red(t) != red(c) && rank(t) == rank(c) + 1,
        }
    }

    fn can_found(&self, c: u8) -> bool {
        self.found[suit(c) as usize] == rank(c)
    }

    fn moves(&self) -> Vec<Move> {
        let mut v = Vec::new();
        if !self.stock.is_empty() || !self.waste.is_empty() {
            v.push(Move::Draw);
        }
        if let Some(&w) = self.waste.last() {
            if self.can_found(w) {
                v.push(Move::WasteToFound);
            }
            let mut empty_done = false;
            for col in 0..7 {
                if self.fits(w, col) {
                    if self.tab[col].is_empty() {
                        if empty_done {
                            continue;
                        }
                        empty_done = true;
                    }
                    v.push(Move::WasteToCol(col as u8));
                }
            }
        }
        for c in 0..7 {
            if let Some(&(t, true)) = self.tab[c].last() {
                if self.can_found(t) {
                    v.push(Move::ColToFound(c as u8));
                }
            }
            let ups = self.tab[c].iter().filter(|x| x.1).count();
            for k in 1..=ups {
                let base = self.tab[c][self.tab[c].len() - k].0;
                let mut empty_done = false;
                for d in 0..7 {
                    if d == c || !self.fits(base, d) {
                        continue;
                    }
                    if self.tab[d].is_empty() {
                        if empty_done || k == self.tab[c].len() {
                            continue;
                        }
                        empty_done = true;
                    }
                    v.push(Move::ColToCol(c as u8, d as u8, k as u8));
                }
            }
        }
        v
    }

    fn flip_top(&mut self, c: usize) {
        if let Some(last) = self.tab[c].last_mut() {
            last.1 = true;
        }
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Draw => {
                if let Some(c) = self.stock.pop() {
                    self.waste.push(c);
                } else {
                    self.stock = self.waste.drain(..).rev().collect();
                }
            }
            Move::WasteToFound => {
                let c = self.waste.pop().unwrap();
                self.found[suit(c) as usize] += 1;
            }
            Move::WasteToCol(d) => {
                let c = self.waste.pop().unwrap();
                self.tab[*d as usize].push((c, true));
            }
            Move::ColToFound(c) => {
                let (t, _) = self.tab[*c as usize].pop().unwrap();
                self.found[suit(t) as usize] += 1;
                self.flip_top(*c as usize);
            }
            Move::ColToCol(c, d, k) => {
                let n = self.tab[*c as usize].len();
                let moved: Vec<(u8, bool)> = self.tab[*c as usize].split_off(n - *k as usize);
                self.tab[*d as usize].extend(moved);
                self.flip_top(*c as usize);
            }
        }
    }

    fn score(&self) -> i32 {
        let empty = self.tab.iter().filter(|c| c.is_empty()).count() as i32;
        let up = self.tab.iter().flatten().filter(|c| c.1).count() as i32;
        self.found_total() * 30 + up * 2 - self.facedown() * 8 + empty * 3
    }

    fn render(&self, name: &str) -> String {
        let mut s = format!(
            "{name}: fundații {} (♥{} ♦{} ♠{} ♣{}), pachet {}, deșeu {}\n",
            self.found_total(),
            self.found[0],
            self.found[1],
            self.found[2],
            self.found[3],
            self.stock.len(),
            self.waste
                .last()
                .map(|&c| text(c))
                .unwrap_or_else(|| "—".into())
        );
        let h = self.tab.iter().map(|c| c.len()).max().unwrap_or(0);
        for r in 0..h {
            s.push_str("  ");
            for c in 0..7 {
                match self.tab[c].get(r) {
                    Some(&(card, true)) => s.push_str(&format!("{:<4}", text(card))),
                    Some(_) => s.push_str("##  "),
                    None => s.push_str("    "),
                }
            }
            s.push('\n');
        }
        s.push_str("  1   2   3   4   5   6   7\n");
        s
    }
}

#[derive(Clone)]
pub struct Klondike {
    boards: [Board; 2],
    side: Player,
    plies: u16,
}

impl Default for Klondike {
    fn default() -> Self {
        Self::new()
    }
}

impl Klondike {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Klondike {
            boards: [Board::new(seed), Board::new(seed ^ 0x5DEECE66D)],
            side: Player::One,
            plies: 0,
        }
    }

    pub fn foundations(&self, p: Player) -> i32 {
        self.boards[p.idx()].found_total()
    }
}

impl Game for Klondike {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        let v = self.boards[self.side.idx()].moves();
        if v.is_empty() {
            // nicio mutare posibilă: tot ce se poate e să treci (reîntoarcere goală)
            return vec![Move::Draw];
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        self.boards[self.side.idx()].apply(m);
        self.plies += 1;
        self.side = self.side.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        for p in [Player::One, Player::Two] {
            if self.foundations(p) == 52 {
                return Some(Outcome::Win(p));
            }
        }
        if self.plies >= CAP {
            return Some(by_score(
                self.foundations(Player::One),
                self.foundations(Player::Two),
            ));
        }
        None
    }

    /// Doar numere și cărți vizibile; cărțile cu fața în jos nu intră (nici cele dezvăluite de mutare).
    fn evaluate(&self, p: Player) -> i32 {
        self.boards[p.idx()].score() - self.boards[p.other().idx()].score()
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Solitaire (Klondike), duel — mutarea {}/{CAP}\n",
            self.plies + 1
        );
        s.push_str(&self.boards[0].render("J1"));
        s.push_str(&self.boards[1].render("J2"));
        let v = verdict(self.outcome());
        if v.is_empty() {
            s.push_str(&format!("La mutare: Jucător {}\n", self.side.idx() + 1));
        } else {
            s.push_str(&v);
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Draw => "d".into(),
            Move::WasteToFound => "w>f".into(),
            Move::WasteToCol(c) => format!("w>{}", c + 1),
            Move::ColToFound(c) => format!("{}>f", c + 1),
            Move::ColToCol(c, d, k) => format!("{}>{}:{}", c + 1, d + 1, k),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q5_util::testkit::check_game;

    #[test]
    fn deal_is_klondike() {
        let g = Klondike::with_seed(1);
        for b in &g.boards {
            assert_eq!(b.stock.len(), 24);
            for (i, col) in b.tab.iter().enumerate() {
                assert_eq!(col.len(), i + 1);
                assert!(col.last().unwrap().1);
                assert!(col[..i].iter().all(|x| !x.1));
            }
        }
    }

    #[test]
    fn move_rules() {
        let mut b = Board::new(3);
        b.tab = vec![Vec::new(); 7];
        // 6♥ (h=0,rank5) pe 7♣ (c=3,rank6)
        b.tab[0] = vec![(3 * 13 + 6, true)];
        b.tab[1] = vec![(5, true)];
        assert!(b.moves().contains(&Move::ColToCol(1, 0, 1)));
        // un rege merge în coloană goală, o damă nu
        b.tab[2] = vec![(12, true)];
        b.tab[3] = vec![(11, true)];
        b.tab[1].clear();
        b.tab[0].clear();
        b.tab[4] = vec![(7, false), (13 + 12, true)];
        let m = b.moves();
        assert!(
            m.iter().any(|x| matches!(x, Move::ColToCol(4, _, 1))),
            "{m:?}"
        );
        assert!(
            !m.iter().any(|x| matches!(x, Move::ColToCol(3, _, _))),
            "{m:?}"
        );
        // as pe fundație
        b.tab[5] = vec![(0, true)];
        assert!(b.moves().contains(&Move::ColToFound(5)));
        b.apply(&Move::ColToFound(5));
        assert_eq!(b.found[0], 1);
        // mutarea șirului dezvăluie cartea de dedesubt
        b.apply(&Move::ColToCol(4, 0, 1));
        assert!(b.tab[4][0].1);
    }

    #[test]
    fn random_and_ai_games() {
        check_game(Klondike::with_seed, 3, 300);
    }
}
