#![allow(clippy::needless_range_loop, clippy::manual_range_contains)]
//! Tombola (Italia) — loto tradițional cu cartele de 3×9, 2 jucători.
//!
//! * Fiecare jucător alege câte cartele joacă (`cartele 1` … `cartele 3`); o cartelă costă 1 punct.
//!   O cartelă are 3 rânduri × 9 coloane, 15 numere din 1–90 (5 pe rând; coloana c are numere
//!   din decada ei).
//! * Cineva trage pe rând numerele din sac (`extrag`); numerele se bifează automat pe toate cartelele.
//! * Premii (primul care le atinge pe un **rând** al unei cartele; la egalitate se împart): ambo
//!   (2 numere) = 2 puncte, terno (3) = 3, quaterna (4) = 4, cinquina (5 = rând complet) = 6.
//!   **Tombola** (toate cele 15 numere ale unei cartele) = 15 puncte și încheie jocul.
//! * Câștigă cine are mai multe puncte (premii minus cartele).
//!
//! Informația cartelelor e publică în această variantă; singurul element de decizie e numărul de cartele.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const PRIZE: [i32; 5] = [2, 3, 4, 6, 15];
const PRIZE_NAME: [&str; 5] = ["ambo", "terno", "quaterna", "cinquina", "TOMBOLA"];

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Cards(u8),
    Draw,
}

type Card = [[u8; 9]; 3];

#[derive(Clone)]
pub struct Tombola {
    rng: Rng,
    cards: [Vec<Card>; 2],
    chosen: [bool; 2],
    bag: Vec<u8>,
    drawn: Vec<u8>,
    awarded: [bool; 5],
    score: [i32; 2],
    over: bool,
    side: Player,
    log: String,
}

impl Default for Tombola {
    fn default() -> Self {
        Self::new()
    }
}

fn make_card(rng: &mut Rng) -> Card {
    loop {
        let mut mask = [[false; 9]; 3];
        for r in 0..3 {
            let mut cols: Vec<usize> = (0..9).collect();
            for i in (1..9).rev() {
                let j = rng.below(i + 1);
                cols.swap(i, j);
            }
            for &c in &cols[..5] {
                mask[r][c] = true;
            }
        }
        if (0..9).all(|c| (0..3).any(|r| mask[r][c])) {
            let mut card = [[0u8; 9]; 3];
            for c in 0..9 {
                let lo = if c == 0 { 1 } else { c as u8 * 10 };
                let hi = if c == 8 { 90 } else { c as u8 * 10 + 9 };
                let mut pool: Vec<u8> = (lo..=hi).collect();
                for i in (1..pool.len()).rev() {
                    let j = rng.below(i + 1);
                    pool.swap(i, j);
                }
                let k = (0..3).filter(|&r| mask[r][c]).count();
                let mut vals: Vec<u8> = pool[..k].to_vec();
                vals.sort_unstable();
                let mut it = vals.into_iter();
                for r in 0..3 {
                    if mask[r][c] {
                        card[r][c] = it.next().unwrap();
                    }
                }
            }
            return card;
        }
    }
}

/// Cel mai bun număr de numere bifate pe un rând, și dacă toată cartela e completă.
fn card_state(c: &Card, drawn: &[u8]) -> (usize, bool) {
    let mut best = 0;
    let mut total = 0;
    for r in 0..3 {
        let n = c[r]
            .iter()
            .filter(|&&x| x != 0 && drawn.contains(&x))
            .count();
        best = best.max(n);
        total += n;
    }
    (best, total == 15)
}

impl Tombola {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut bag: Vec<u8> = (1..=90).collect();
        for i in (1..bag.len()).rev() {
            let j = rng.below(i + 1);
            bag.swap(i, j);
        }
        Tombola {
            rng,
            cards: [Vec::new(), Vec::new()],
            chosen: [false, false],
            bag,
            drawn: Vec::new(),
            awarded: [false; 5],
            score: [0, 0],
            over: false,
            side: Player::One,
            log: String::new(),
        }
    }

    pub fn scores(&self) -> (i32, i32) {
        (self.score[0], self.score[1])
    }

    fn started(&self) -> bool {
        self.chosen[0] && self.chosen[1]
    }
}

impl Game for Tombola {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over {
            return Vec::new();
        }
        if !self.started() {
            return (1..=3).map(Move::Cards).collect();
        }
        vec![Move::Draw]
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        match *m {
            Move::Cards(n) => {
                let p = self.side.idx();
                for _ in 0..n {
                    let c = make_card(&mut self.rng);
                    self.cards[p].push(c);
                }
                self.score[p] -= n as i32;
                self.chosen[p] = true;
                self.side = self.side.other();
            }
            Move::Draw => {
                let x = self.bag.pop().unwrap();
                self.drawn.push(x);
                self.log = format!("Ieșit: {x}");
                // premii: nivele 0..=3 pe rând, 4 = tombola
                let mut reached: [[bool; 5]; 2] = [[false; 5]; 2];
                for p in 0..2 {
                    for c in &self.cards[p] {
                        let (best, full) = card_state(c, &self.drawn);
                        for lv in 0..4 {
                            if best >= lv + 2 {
                                reached[p][lv] = true;
                            }
                        }
                        if full {
                            reached[p][4] = true;
                        }
                    }
                }
                for lv in 0..5 {
                    if self.awarded[lv] {
                        continue;
                    }
                    let who: Vec<usize> = (0..2).filter(|&p| reached[p][lv]).collect();
                    if who.is_empty() {
                        continue;
                    }
                    self.awarded[lv] = true;
                    for &p in &who {
                        self.score[p] += PRIZE[lv];
                    }
                    self.log.push_str(&format!(
                        "; {} → {}",
                        PRIZE_NAME[lv],
                        who.iter()
                            .map(|p| format!("J{}", p + 1))
                            .collect::<Vec<_>>()
                            .join("+")
                    ));
                    if lv == 4 {
                        self.over = true;
                    }
                }
                if self.bag.is_empty() {
                    self.over = true;
                }
                self.side = self.side.other();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            return None;
        }
        Some(match self.score[0].cmp(&self.score[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let (me, op) = (p.idx(), 1 - p.idx());
        // fiecare cartelă în plus: ~ +1.4 puncte așteptate față de cost 1
        (self.score[me] - self.score[op]) * 100
            + (self.cards[me].len() as i32 - self.cards[op].len() as i32) * 40
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Tombola — puncte: J1 {}, J2 {} | numere extrase: {}/90\n",
            self.score[0],
            self.score[1],
            self.drawn.len()
        );
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        for p in 0..2 {
            for (k, c) in self.cards[p].iter().enumerate() {
                s.push_str(&format!("J{} cartela {}:\n", p + 1, k + 1));
                for r in 0..3 {
                    let row: Vec<String> = c[r]
                        .iter()
                        .map(|&x| {
                            if x == 0 {
                                " . ".into()
                            } else if self.drawn.contains(&x) {
                                format!("[{x:>2}]").replace('[', "(").replace(']', ")")
                            } else {
                                format!(" {x:>2} ")
                            }
                        })
                        .collect();
                    s.push_str(&format!("  {}\n", row.join("")));
                }
            }
        }
        if let Some(x) = self.drawn.last() {
            s.push_str(&format!("Ultimul număr: {x}\n"));
        }
        if self.over {
            s.push_str("Joc încheiat.\n");
        } else if !self.started() {
            s.push_str(&format!(
                "J{} alege numărul de cartele (cartele 1–3).\n",
                self.side.idx() + 1
            ));
        } else {
            s.push_str("Scrie «extrag».\n");
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Cards(n) => format!("cartele {n}"),
            Move::Draw => "extrag".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_shape() {
        let mut rng = Rng::new(3);
        for _ in 0..50 {
            let c = make_card(&mut rng);
            let mut all = Vec::new();
            for r in 0..3 {
                assert_eq!(c[r].iter().filter(|&&x| x != 0).count(), 5);
                for col in 0..9 {
                    let x = c[r][col];
                    if x != 0 {
                        assert_eq!((x as usize / 10).min(8), if x == 90 { 8 } else { col });
                        all.push(x);
                    }
                }
            }
            all.sort_unstable();
            all.dedup();
            assert_eq!(all.len(), 15);
        }
    }

    #[test]
    fn prizes_awarded_once() {
        for seed in 1..=10u64 {
            let mut g = Tombola::with_seed(seed);
            g.apply(&Move::Cards(2));
            g.apply(&Move::Cards(3));
            let mut n = 0;
            while g.outcome().is_none() && n < 200 {
                g.apply(&Move::Draw);
                n += 1;
            }
            assert!(g.outcome().is_some());
            assert!(g.awarded[0] && g.awarded[1]);
            // suma punctelor = premii - costuri
            assert!(g.score[0] + g.score[1] >= -5);
        }
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=5u64 {
            let mut g = Tombola::with_seed(seed);
            let mut rng = Rng::new(seed);
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
            }
        }
    }
}
