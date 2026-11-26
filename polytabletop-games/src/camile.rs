//! Cursa cămilelor (variantă proprie de pariuri pe o cursă de cămile, 2 jucători): cinci cămile
//! (A–E) pornesc pe o pistă de 16 celule și se mută cu zaruri extrase din pahar. O tură:
//! `zar` (scoți un zar din pahar: cămila lui înaintează 1–3 celule împreună cu cămilele aflate
//! pe ea; ajunge deasupra stivei de la destinație; câștigi 1 monedă), `pariu A` (iei cea mai
//! de sus bilă de pariu a etapei pentru cămila A: 5, 3 sau 2), `castig A` / `pierd A` (pui
//! cartea cămilei A în teancul pariului pe câștigătoarea cursei / pe ultima; fiecare cămilă
//! are o singură carte pe jucător). Etapa se termină când toate zarurile au fost folosite: bila
//! de pariu aduce valoarea ei dacă cămila e prima, 1 monedă dacă e a doua, altfel −1.
//! Clasamentul: cea mai înaintată celulă, iar în stivă cea de deasupra. Cursa se termină când o
//! cămilă trece de ultima celulă; cărțile pe câștigătoare și pe ultimă se plătesc în ordinea
//! punerii: 8, 5, 3, 2, 1 pentru corecte, −1 pentru greșite. Câștigă cine are mai multe monede.
#![allow(clippy::all)]

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const TRACK: usize = 16;
const NAMES: [char; 5] = ['A', 'B', 'C', 'D', 'E'];
const PAY: [i32; 5] = [8, 5, 3, 2, 1];

#[derive(Clone, PartialEq, Debug)]
pub enum Mv {
    Roll,
    Bet(u8),
    Win(u8),
    Lose(u8),
}

#[derive(Clone)]
pub struct Camile {
    cells: Vec<Vec<u8>>, // TRACK + 1 (ultima = trecut de linie)
    dice_left: u8,
    tiles: [Vec<i32>; 5],
    tickets: [Vec<(u8, i32)>; 2],
    wins: Vec<(u8, u8)>, // (jucător, cămilă)
    loses: Vec<(u8, u8)>,
    cards: [u8; 2], // cămile cu carte folosită (mască)
    coins: [i32; 2],
    side: Player,
    rng: Rng,
    over: bool,
    fresh: Option<[i32; 2]>,
    ply: u32,
}

impl Default for Camile {
    fn default() -> Self {
        Self::new()
    }
}

impl Camile {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut cells = vec![vec![]; TRACK + 1];
        for c in 0..5u8 {
            let v = 1 + rng.below(3);
            cells[v - 1].push(c);
        }
        Camile {
            cells,
            dice_left: 0b11111,
            tiles: [
                vec![2, 3, 5],
                vec![2, 3, 5],
                vec![2, 3, 5],
                vec![2, 3, 5],
                vec![2, 3, 5],
            ],
            tickets: [vec![], vec![]],
            wins: vec![],
            loses: vec![],
            cards: [0; 2],
            coins: [3, 3],
            side: Player::One,
            rng,
            over: false,
            fresh: None,
            ply: 0,
        }
    }
    fn ranking(&self) -> Vec<u8> {
        let mut v = vec![];
        for cell in self.cells.iter().rev() {
            for &c in cell.iter().rev() {
                v.push(c);
            }
        }
        v
    }
    fn resolve_leg(&mut self) {
        let rank = self.ranking();
        for p in 0..2 {
            for &(c, val) in &self.tickets[p] {
                if rank[0] == c {
                    self.coins[p] += val;
                } else if rank[1] == c {
                    self.coins[p] += 1;
                } else {
                    self.coins[p] -= 1;
                }
            }
            self.tickets[p].clear();
        }
        self.tiles = [
            vec![2, 3, 5],
            vec![2, 3, 5],
            vec![2, 3, 5],
            vec![2, 3, 5],
            vec![2, 3, 5],
        ];
        self.dice_left = 0b11111;
    }
    fn resolve_race(&mut self) {
        let rank = self.ranking();
        let mut idx = 0;
        for &(p, c) in &self.wins {
            if c == rank[0] {
                self.coins[p as usize] += PAY[idx.min(4)];
                idx += 1;
            } else {
                self.coins[p as usize] -= 1;
            }
        }
        let last = *rank.last().unwrap();
        idx = 0;
        for &(p, c) in &self.loses {
            if c == last {
                self.coins[p as usize] += PAY[idx.min(4)];
                idx += 1;
            } else {
                self.coins[p as usize] -= 1;
            }
        }
    }
    /// Valoare euristică (de la perspectiva lui p) fără a ține seama de zaruri viitoare.
    fn heur(&self, p: usize) -> [i32; 2] {
        let mut v = self.coins;
        let rank = self.ranking();
        for q in 0..2 {
            for &(c, val) in &self.tickets[q] {
                let pos = rank.iter().position(|&x| x == c).unwrap();
                v[q] += match pos {
                    0 => val,
                    1 => 1,
                    _ => -1,
                } * 2
                    / if self.dice_left.count_ones() >= 3 {
                        2
                    } else {
                        1
                    };
            }
        }
        let _ = p;
        v
    }
}

impl Game for Camile {
    type Move = Mv;
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<Mv> {
        if self.over {
            return vec![];
        }
        let p = self.side.idx();
        let mut v = vec![Mv::Roll];
        for c in 0..5u8 {
            if !self.tiles[c as usize].is_empty() {
                v.push(Mv::Bet(c));
            }
            if self.cards[p] >> c & 1 == 0 {
                v.push(Mv::Win(c));
                v.push(Mv::Lose(c));
            }
        }
        v
    }
    fn apply(&mut self, m: &Mv) {
        let p = self.side.idx();
        self.ply += 1;
        self.fresh = None;
        match m {
            Mv::Roll => {
                // evaluarea cinstită nu vede rezultatul zarului: păstrăm valoarea dinaintea lui
                let mut pre = self.heur(p);
                pre[p] += 1;
                let dice: Vec<u8> = (0..5).filter(|&c| self.dice_left >> c & 1 == 1).collect();
                let c = dice[self.rng.below(dice.len())];
                let steps = 1 + self.rng.below(3);
                self.dice_left &= !(1 << c);
                let (pos, k) = {
                    let mut found = (0, 0);
                    for (i, cell) in self.cells.iter().enumerate() {
                        if let Some(k) = cell.iter().position(|&x| x == c) {
                            found = (i, k);
                        }
                    }
                    found
                };
                let group: Vec<u8> = self.cells[pos].split_off(k);
                let to = (pos + steps).min(TRACK);
                self.cells[to].extend(group);
                self.coins[p] += 1;
                if to >= TRACK {
                    self.resolve_leg();
                    self.resolve_race();
                    self.over = true;
                } else if self.dice_left == 0 {
                    self.resolve_leg();
                }
                self.fresh = Some(pre);
            }
            Mv::Bet(c) => {
                let val = self.tiles[*c as usize].pop().unwrap();
                self.tickets[p].push((*c, val));
            }
            Mv::Win(c) => {
                self.cards[p] |= 1 << c;
                self.wins.push((p as u8, *c));
            }
            Mv::Lose(c) => {
                self.cards[p] |= 1 << c;
                self.loses.push((p as u8, *c));
            }
        }
        self.side = self.side.other();
    }
    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            return None;
        }
        Some(match self.coins[0].cmp(&self.coins[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            _ => Outcome::Draw,
        })
    }
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let v = self.fresh.unwrap_or_else(|| self.heur(me));
        v[me] - v[1 - me]
    }
    fn render(&self) -> String {
        let mut s = String::from("Pista (celulele 1–16, cămilele de jos în sus):\n");
        for (i, cell) in self.cells.iter().enumerate() {
            if !cell.is_empty() {
                let t: String = cell.iter().map(|&c| NAMES[c as usize]).collect();
                s.push_str(&format!(
                    "  {}: {}\n",
                    if i >= TRACK {
                        "trecut".to_string()
                    } else {
                        (i + 1).to_string()
                    },
                    t
                ));
            }
        }
        let dice: String = (0..5)
            .filter(|&c| self.dice_left >> c & 1 == 1)
            .map(|c| NAMES[c as usize])
            .collect();
        s.push_str(&format!("Zaruri rămase în pahar: {}\n", dice));
        s.push_str("Bile de pariu disponibile: ");
        for c in 0..5 {
            if let Some(&v) = self.tiles[c].last() {
                s.push_str(&format!("{}:{} ", NAMES[c], v));
            }
        }
        s.push('\n');
        for p in 0..2 {
            s.push_str(&format!(
                "J{}: {} monede; pariuri etapă: {}\n",
                p + 1,
                self.coins[p],
                self.tickets[p]
                    .iter()
                    .map(|&(c, v)| format!("{}:{}", NAMES[c as usize], v))
                    .collect::<Vec<_>>()
                    .join(" ")
            ));
        }
        s.push_str(&format!(
            "Pariuri pe câștigătoare: {}; pe ultimă: {}\n",
            self.wins
                .iter()
                .map(|&(p, c)| format!("J{}-{}", p + 1, NAMES[c as usize]))
                .collect::<Vec<_>>()
                .join(" "),
            self.loses
                .iter()
                .map(|&(p, c)| format!("J{}-{}", p + 1, NAMES[c as usize]))
                .collect::<Vec<_>>()
                .join(" ")
        ));
        s
    }
    fn format_move(&self, m: &Mv) -> String {
        match m {
            Mv::Roll => "zar".into(),
            Mv::Bet(c) => format!("pariu {}", NAMES[*c as usize]),
            Mv::Win(c) => format!("castig {}", NAMES[*c as usize]),
            Mv::Lose(c) => format!("pierd {}", NAMES[*c as usize]),
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
    fn stacking_and_leg_end() {
        let mut g = Camile::with_seed(7);
        let before: usize = g.cells.iter().map(|c| c.len()).sum();
        assert_eq!(before, 5);
        for _ in 0..5 {
            if g.over {
                break;
            }
            g.apply(&Mv::Roll);
        }
        assert!(g.over || g.dice_left == 0b11111);
        let after: usize = g.cells.iter().map(|c| c.len()).sum();
        assert_eq!(after, 5);
    }
    #[test]
    fn leg_bet_payout() {
        let mut g = Camile::with_seed(1);
        g.cells = vec![vec![]; TRACK + 1];
        g.cells[4] = vec![1, 3];
        g.cells[2] = vec![0];
        g.cells[1] = vec![2, 4];
        g.apply(&Mv::Bet(3)); // J1 pariază pe D (în față, pe stivă)
        g.coins = [0, 0];
        g.resolve_leg();
        assert_eq!(g.coins[0], 5);
    }
    #[test]
    fn random_and_ai() {
        check(Camile::with_seed, 600);
    }
}
