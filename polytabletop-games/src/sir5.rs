//! Șir de cinci (variantă proprie de cărți și jetoane pe o tablă de 10×10, informație ascunsă):
//! tabla are 4 colțuri libere (jolly) și 96 de celule cu cele 48 de cărți ale unui pachet fără
//! valeți, fiecare de două ori (așezarea e a noastră). Pachetul de joc are 2×52 de cărți; fiecare
//! jucător ține 7. O tură: joci o carte din mână și pui jetonul tău pe una din cele două celule
//! cu acea carte (dacă e liberă). Valetul de romb sau de treflă e jolly: jeton pe orice celulă
//! liberă. Valetul de inimă sau de pică scoate un jeton adversar (care nu e într-un șir
//! terminat). O carte moartă (ambele celule ocupate) se poate arunca (`d 7h`), ceea ce ține
//! locul mutării; apoi tragi o carte. Un șir = cinci jetoane proprii în linie (colțurile
//! contează ca ale tale); două șiruri pot împărți cel mult o celulă. Primul cu două șiruri
//! câștigă; după 300 de semi-mutări câștigă cine are mai multe șiruri.
//! Cărți: rang 2–9 T J Q K A + culoare h d s c. Mutări: `7h@d4`, `Jd@d4` (jolly), `Js@d4`
//! (scoate), `d 7h` (arunc carte moartă).
#![allow(clippy::all)]

use crate::b1_util::sq;
use crate::cards::{handover_banner, Viewer};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const RANKS: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];
const SUITS: [char; 4] = ['h', 'd', 's', 'c'];

type Card = u8; // rang*4 + culoare

fn cname(c: Card) -> String {
    format!("{}{}", RANKS[c as usize / 4], SUITS[c as usize % 4])
}
fn is_jack(c: Card) -> bool {
    c as usize / 4 == 9
}

/// Așezarea cărților pe tablă (generată determinist de un LCG; nu copiază nicio tablă comercială).
fn layout() -> [i16; 100] {
    let mut cards: Vec<Card> = (0..52).filter(|&c| !is_jack(c)).collect();
    let mut b = [-1i16; 100];
    let mut st: u64 = 0x2545_F491_4F6C_DD1D;
    let mut next = |n: usize| {
        st = st
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((st >> 33) as usize) % n
    };
    let cells: Vec<usize> = (0..100).filter(|&i| ![0, 9, 90, 99].contains(&i)).collect();
    let mut order: Vec<Card> = vec![];
    for _ in 0..2 {
        for i in (1..cards.len()).rev() {
            let j = next(i + 1);
            cards.swap(i, j);
        }
        order.extend(cards.iter().copied());
    }
    for (k, &c) in cells.iter().enumerate() {
        b[c] = order[k] as i16;
    }
    b
}

#[derive(Clone)]
pub struct Sir5 {
    lay: [i16; 100],
    chip: [u8; 100], // 0 liber, 1/2
    locked: [bool; 100],
    seqs: [u8; 2],
    hand: [Vec<Card>; 2],
    deck: Vec<Card>,
    side: Player,
    prev: Player,
    viewer: Viewer,
    ply: u32,
}

impl Default for Sir5 {
    fn default() -> Self {
        Self::new()
    }
}

impl Sir5 {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck: Vec<Card> = (0..104).map(|i| (i % 52) as u8).collect();
        for i in (1..deck.len()).rev() {
            deck.swap(i, rng.below(i + 1));
        }
        let h0: Vec<Card> = deck.drain(..7).collect();
        let h1: Vec<Card> = deck.drain(..7).collect();
        Sir5 {
            lay: layout(),
            chip: [0; 100],
            locked: [false; 100],
            seqs: [0; 2],
            hand: [h0, h1],
            deck,
            side: Player::One,
            prev: Player::One,
            viewer: Viewer::Fixed(Player::One),
            ply: 0,
        }
    }
    fn corner(i: usize) -> bool {
        [0, 9, 90, 99].contains(&i)
    }
    fn mine(&self, i: usize, p: u8) -> bool {
        Self::corner(i) || self.chip[i] == p
    }
    /// Caută șiruri noi care trec prin `i`; le blochează și întoarce câte.
    fn find_new(&mut self, i: usize, p: u8) -> u8 {
        let mut found = 0;
        let (x, y) = ((i % 10) as i32, (i / 10) as i32);
        for (dx, dy) in [(1, 0), (0, 1), (1, 1), (1, -1)] {
            // toate ferestrele de 5 care conțin i
            for start in -4..=0 {
                let cells: Vec<(i32, i32)> = (0..5)
                    .map(|k| (x + dx * (start + k), y + dy * (start + k)))
                    .collect();
                if cells
                    .iter()
                    .any(|&(a, b)| a < 0 || b < 0 || a >= 10 || b >= 10)
                {
                    continue;
                }
                let idx: Vec<usize> = cells.iter().map(|&(a, b)| (b * 10 + a) as usize).collect();
                if !idx.iter().all(|&c| self.mine(c, p)) {
                    continue;
                }
                let already = idx
                    .iter()
                    .filter(|&&c| self.locked[c] && !Self::corner(c))
                    .count();
                if already <= 1 && (idx.iter().any(|&c| !self.locked[c])) {
                    for &c in &idx {
                        self.locked[c] = true;
                    }
                    found += 1;
                }
            }
        }
        found
    }
    fn draw(&mut self, p: usize) {
        if let Some(c) = self.deck.pop() {
            self.hand[p].push(c);
        }
    }
    fn heur(&self, p: u8) -> i32 {
        let mut s = 0;
        for y in 0..10i32 {
            for x in 0..10i32 {
                for (dx, dy) in [(1, 0), (0, 1), (1, 1), (1, -1)] {
                    let cells: Vec<usize> = (0..5)
                        .map(|k| (y + dy * k, x + dx * k))
                        .filter(|&(b, a)| (0..10).contains(&a) && (0..10).contains(&b))
                        .map(|(b, a)| (b * 10 + a) as usize)
                        .collect();
                    if cells.len() < 5 {
                        continue;
                    }
                    let mine = cells.iter().filter(|&&c| self.mine(c, p)).count();
                    let theirs = cells
                        .iter()
                        .filter(|&&c| self.mine(c, 3 - p) && !Self::corner(c))
                        .count();
                    if theirs == 0 {
                        s += [0, 1, 3, 9, 30, 400][mine];
                    }
                }
            }
        }
        s
    }
}

impl Game for Sir5 {
    // (carte, celulă) ; celulă 255 = aruncă
    type Move = (u8, u8);
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<(u8, u8)> {
        if self.outcome().is_some() {
            return vec![];
        }
        let p = self.side.idx();
        let me = p as u8 + 1;
        let mut hand = self.hand[p].clone();
        hand.sort();
        hand.dedup();
        let mut v = vec![];
        for &c in &hand {
            if is_jack(c) {
                let two_eyed = c % 4 == 1 || c % 4 == 3; // romb, treflă
                for i in 0..100 {
                    if two_eyed {
                        if self.chip[i] == 0 && !Self::corner(i) {
                            v.push((c, i as u8));
                        }
                    } else if self.chip[i] == 3 - me && !self.locked[i] {
                        v.push((c, i as u8));
                    }
                }
            } else {
                let cells: Vec<usize> = (0..100).filter(|&i| self.lay[i] == c as i16).collect();
                let free: Vec<usize> = cells
                    .iter()
                    .copied()
                    .filter(|&i| self.chip[i] == 0)
                    .collect();
                if free.is_empty() {
                    v.push((c, 255));
                } else {
                    for i in free {
                        v.push((c, i as u8));
                    }
                }
            }
        }
        if v.is_empty() {
            v.push((254, 254));
        }
        v
    }
    fn apply(&mut self, m: &(u8, u8)) {
        let p = self.side.idx();
        let me = p as u8 + 1;
        self.ply += 1;
        if m.0 != 254 {
            let k = self.hand[p].iter().position(|&c| c == m.0).unwrap();
            self.hand[p].remove(k);
            if m.1 != 255 {
                let i = m.1 as usize;
                if is_jack(m.0) && !(m.0 % 4 == 1 || m.0 % 4 == 3) {
                    self.chip[i] = 0;
                } else {
                    self.chip[i] = me;
                    let n = self.find_new(i, me);
                    self.seqs[p] += n;
                }
            }
            self.draw(p);
        }
        self.prev = self.side;
        self.side = self.side.other();
    }
    fn outcome(&self) -> Option<Outcome> {
        if self.seqs[0] >= 2 {
            return Some(Outcome::Win(Player::One));
        }
        if self.seqs[1] >= 2 {
            return Some(Outcome::Win(Player::Two));
        }
        if self.ply >= 300 {
            return Some(match self.seqs[0].cmp(&self.seqs[1]) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                _ => Outcome::Draw,
            });
        }
        None
    }
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx() as u8 + 1;
        (self.seqs[p.idx()] as i32 - self.seqs[1 - p.idx()] as i32) * 1000 + self.heur(me)
            - self.heur(3 - me)
    }
    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = match who {
            Some(p) => Viewer::Fixed(p),
            None => Viewer::HotSeat,
        };
    }
    fn render(&self) -> String {
        let mut s = String::new();
        if self.prev != self.side && self.outcome().is_none() {
            if let Viewer::HotSeat = self.viewer {
                s.push_str(&handover_banner(self.side));
            }
        }
        s.push_str("    a    b    c    d    e    f    g    h    i    j\n");
        for y in (0..10).rev() {
            s.push_str(&format!("{:>2} ", y + 1));
            for x in 0..10 {
                let i = y * 10 + x;
                let cell = if Self::corner(i) {
                    "  * ".to_string()
                } else if self.chip[i] != 0 {
                    format!(
                        " {}{} ",
                        ['_', 'X', 'O'][self.chip[i] as usize],
                        if self.locked[i] { '=' } else { ' ' }
                    )
                } else {
                    format!(" {} ", cname(self.lay[i] as u8))
                };
                s.push_str(&format!("{:>4} ", cell.trim_end()));
            }
            s.push('\n');
        }
        s.push_str(&format!(
            "Șiruri: J1 {}, J2 {}; în pachet {} cărți. X = J1, O = J2, = șir încheiat.\n",
            self.seqs[0],
            self.seqs[1],
            self.deck.len()
        ));
        let v = self.viewer.who(self.side);
        let mut h = self.hand[v.idx()].clone();
        h.sort();
        s.push_str(&format!(
            "Mâna ta (J{}): {}\n",
            v.idx() + 1,
            h.iter().map(|&c| cname(c)).collect::<Vec<_>>().join(" ")
        ));
        s.push_str(&format!(
            "J{} are {} cărți\n",
            v.other().idx() + 1,
            self.hand[v.other().idx()].len()
        ));
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        s
    }
    fn format_move(&self, m: &(u8, u8)) -> String {
        if m.0 == 254 {
            "pas".into()
        } else if m.1 == 255 {
            format!("d {}", cname(m.0))
        } else {
            format!(
                "{}@{}",
                cname(m.0),
                sq(m.1 as usize % 10, m.1 as usize / 10)
            )
        }
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::b1_util::testing::check;
    #[test]
    fn layout_has_each_card_twice() {
        let l = layout();
        for c in 0..52i16 {
            let n = l.iter().filter(|&&x| x == c).count();
            assert_eq!(n, if is_jack(c as u8) { 0 } else { 2 });
        }
        assert_eq!(l.iter().filter(|&&x| x == -1).count(), 4);
    }
    #[test]
    fn five_in_a_row_makes_sequence() {
        let mut g = Sir5::with_seed(1);
        for x in 1..5 {
            g.chip[x] = 1;
        }
        // a1 (index 0) e colț; chips b1..e1 + colț a1 => 5; pune ultimul pe e1 (index 4) e deja; pune f1
        g.chip[4] = 0;
        let n = g.find_new(0, 1);
        assert_eq!(n, 0);
        g.chip[4] = 1;
        assert_eq!(g.find_new(4, 1), 1);
        assert!(g.locked[0] && g.locked[4]);
    }
    #[test]
    fn hidden_hands() {
        let g = Sir5::with_seed(3);
        let mut a = g.clone();
        a.set_viewer(Some(Player::One));
        let mut b = g.clone();
        b.set_viewer(Some(Player::Two));
        let f = |x: &Sir5| {
            x.render()
                .lines()
                .find(|l| l.starts_with("Mâna ta"))
                .unwrap()
                .split_once(": ")
                .unwrap()
                .1
                .to_string()
        };
        assert_ne!(f(&a), f(&b));
    }
    #[test]
    fn random_and_ai() {
        check(Sir5::with_seed, 400);
    }
}
