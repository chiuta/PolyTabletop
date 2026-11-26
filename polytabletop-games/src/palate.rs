//! Palate (variantă proprie de construit un oraș cu cărți de monedă, 2 jucători, informație
//! ascunsă): pachetul de monede are patru culori (a, b, c, d), valori 1–9, de două ori. Piața
//! arată 4 cărți de monedă și 4 clădiri, fiecare clădire stând sub o culoare (a–d) și având
//! un preț. O tură: `iau 2` (iei din piață cartea de pe locul 2; completarea vine din pachet)
//! sau `cumpar b` (cumperi clădirea de sub culoarea b plătind cu cărțile tale de culoarea b;
//! plata e cea mai mică suma ≥ preț, cu cât mai puține cărți; plata exactă îți dă încă o tură),
//! apoi `pune x,y` (clădirea intră în orașul tău, lipită pe latură de o clădire proprie;
//! orașul încape în coordonatele −2…2, castelul de pornire e pe 0,0). 6 tipuri de clădiri (t1–t6)
//! a câte 6 exemplare, cu valorile 1–6. La final (când piața de clădiri nu se mai poate
//! reface) punctele sunt: pentru fiecare tip, cine are mai multe clădiri de acel tip ia 3×valoarea
//! tipului (la egalitate fiecare primește jumătate), plus 1 punct pentru fiecare clădire din oraș.
//! Câștigă scorul mai mare. Informație ascunsă: vezi doar cărțile tale de monedă.
#![allow(clippy::all)]

use crate::cards::{handover_banner, Viewer};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

type Coin = (u8, u8); // culoare, valoare

#[derive(Clone, PartialEq, Debug)]
pub enum Mv {
    Take(u8),
    Buy(u8),
    Place(i8, i8),
}

#[derive(Clone)]
pub struct Palate {
    hand: [Vec<Coin>; 2],
    deck: Vec<Coin>,
    market: Vec<Coin>,
    tiles: Vec<(u8, u8)>, // (tip, preț) pachet de clădiri
    shop: [Option<(u8, u8)>; 4],
    city: [Vec<(i8, i8, u8)>; 2],
    pending: Option<u8>, // tip de clădire de așezat
    extra: bool,
    side: Player,
    prev: Player,
    viewer: Viewer,
    over: bool,
    ply: u32,
}

impl Default for Palate {
    fn default() -> Self {
        Self::new()
    }
}

impl Palate {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck = vec![];
        for _ in 0..2 {
            for c in 0..4 {
                for v in 1..=9 {
                    deck.push((c, v));
                }
            }
        }
        for i in (1..deck.len()).rev() {
            deck.swap(i, rng.below(i + 1));
        }
        let mut tiles = vec![];
        for t in 0..6u8 {
            for k in 0..6u8 {
                tiles.push((t, 2 + ((t as usize * 3 + k as usize * 2) % 7) as u8));
            }
        }
        for i in (1..tiles.len()).rev() {
            tiles.swap(i, rng.below(i + 1));
        }
        let market: Vec<Coin> = deck.drain(..4).collect();
        let h0: Vec<Coin> = deck.drain(..4).collect();
        let h1: Vec<Coin> = deck.drain(..4).collect();
        let mut g = Palate {
            hand: [h0, h1],
            deck,
            market,
            tiles,
            shop: [None; 4],
            city: [vec![(0, 0, 255)], vec![(0, 0, 255)]],
            pending: None,
            extra: false,
            side: Player::One,
            prev: Player::One,
            viewer: Viewer::Fixed(Player::One),
            over: false,
            ply: 0,
        };
        for s in 0..4 {
            g.shop[s] = g.tiles.pop();
        }
        g
    }
    /// Plata minimă în culoarea c pentru `price`: indicii cărților din mână.
    fn payment(&self, p: usize, c: u8, price: u8) -> Option<Vec<usize>> {
        let idx: Vec<usize> = (0..self.hand[p].len())
            .filter(|&i| self.hand[p][i].0 == c)
            .collect();
        let n = idx.len();
        let mut best: Option<(i32, usize, Vec<usize>)> = None;
        for mask in 1u32..(1 << n.min(10)) {
            let sel: Vec<usize> = (0..n.min(10))
                .filter(|&k| mask >> k & 1 == 1)
                .map(|k| idx[k])
                .collect();
            let sum: i32 = sel.iter().map(|&i| self.hand[p][i].1 as i32).sum();
            if sum >= price as i32 {
                let key = (sum - price as i32, sel.len());
                if best.as_ref().is_none_or(|b| (b.0, b.1) > key) {
                    best = Some((key.0, key.1, sel));
                }
            }
        }
        best.map(|b| b.2)
    }
    fn open_cells(&self, p: usize) -> Vec<(i8, i8)> {
        let mut v = vec![];
        for x in -2..=2i8 {
            for y in -2..=2i8 {
                if self.city[p].iter().any(|c| (c.0, c.1) == (x, y)) {
                    continue;
                }
                if [(1, 0), (-1, 0), (0, 1), (0, -1)]
                    .iter()
                    .any(|&(dx, dy)| self.city[p].iter().any(|c| (c.0, c.1) == (x + dx, y + dy)))
                {
                    v.push((x, y));
                }
            }
        }
        v
    }
    fn score(&self, p: usize) -> i32 {
        let mut s = 0;
        for t in 0..6u8 {
            let a = self.city[p].iter().filter(|c| c.2 == t).count();
            let b = self.city[1 - p].iter().filter(|c| c.2 == t).count();
            let val = 3 * (t as i32 + 1);
            if a > b {
                s += val;
            } else if a == b && a > 0 {
                s += val / 2;
            }
        }
        s + self.city[p].len() as i32 - 1
    }
    fn refill(&mut self) {
        while self.market.len() < 4 {
            match self.deck.pop() {
                Some(c) => self.market.push(c),
                None => break,
            }
        }
    }
}

impl Game for Palate {
    type Move = Mv;
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<Mv> {
        if self.over {
            return vec![];
        }
        let p = self.side.idx();
        if self.pending.is_some() {
            return self
                .open_cells(p)
                .into_iter()
                .map(|(x, y)| Mv::Place(x, y))
                .collect();
        }
        let mut v = vec![];
        for i in 0..self.market.len() {
            v.push(Mv::Take(i as u8));
        }
        for c in 0..4u8 {
            if let Some((_, price)) = self.shop[c as usize] {
                if self.payment(p, c, price).is_some() && !self.open_cells(p).is_empty() {
                    v.push(Mv::Buy(c));
                }
            }
        }
        if v.is_empty() {
            v.push(Mv::Take(255));
        }
        v
    }
    fn apply(&mut self, m: &Mv) {
        let p = self.side.idx();
        self.ply += 1;
        let mut again = false;
        match m {
            Mv::Take(i) => {
                if *i != 255 {
                    let c = self.market.remove(*i as usize);
                    self.hand[p].push(c);
                    self.refill();
                } else {
                    self.over = true;
                }
            }
            Mv::Buy(c) => {
                let (t, price) = self.shop[*c as usize].unwrap();
                let pay = self.payment(p, *c, price).unwrap();
                let sum: i32 = pay.iter().map(|&i| self.hand[p][i].1 as i32).sum();
                let mut idx = pay;
                idx.sort_unstable_by(|a, b| b.cmp(a));
                for i in idx {
                    self.hand[p].remove(i);
                }
                self.shop[*c as usize] = self.tiles.pop();
                self.pending = Some(t);
                self.extra = sum == price as i32;
                return;
            }
            Mv::Place(x, y) => {
                let t = self.pending.take().unwrap();
                self.city[p].push((*x, *y, t));
                again = self.extra;
                self.extra = false;
            }
        }
        if self.shop.iter().all(|s| s.is_none()) || self.ply > 500 {
            self.over = true;
        }
        if !again {
            self.prev = self.side;
            self.side = self.side.other();
        }
    }
    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            return None;
        }
        let (a, b) = (self.score(0), self.score(1));
        Some(match a.cmp(&b) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            _ => Outcome::Draw,
        })
    }
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let coins = |q: usize| self.hand[q].iter().map(|c| c.1 as i32).sum::<i32>();
        // scorul public al orașelor + valoarea monedelor proprii (adversarul: doar numărul de cărți)
        (self.score(me) - self.score(1 - me)) * 10 + coins(me) / 3 - self.hand[1 - me].len() as i32
    }
    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = match who {
            Some(p) => Viewer::Fixed(p),
            None => Viewer::HotSeat,
        };
    }
    fn render(&self) -> String {
        let mut s = String::new();
        if self.prev != self.side && !self.over {
            if let Viewer::HotSeat = self.viewer {
                s.push_str(&handover_banner(self.side));
            }
        }
        let cn = |c: Coin| format!("{}{}", (b'a' + c.0) as char, c.1);
        s.push_str(&format!(
            "Piața de monede: {}\n",
            self.market
                .iter()
                .enumerate()
                .map(|(i, &c)| format!("{}:{}", i + 1, cn(c)))
                .collect::<Vec<_>>()
                .join(" ")
        ));
        s.push_str("Clădiri la vânzare: ");
        for c in 0..4 {
            match self.shop[c] {
                Some((t, pr)) => s.push_str(&format!(
                    "{}: t{} preț {}   ",
                    (b'a' + c as u8) as char,
                    t + 1,
                    pr
                )),
                None => s.push_str(&format!("{}: –   ", (b'a' + c as u8) as char)),
            }
        }
        s.push('\n');
        for p in 0..2 {
            let mut t = [0; 6];
            for c in &self.city[p] {
                if c.2 < 6 {
                    t[c.2 as usize] += 1;
                }
            }
            s.push_str(&format!(
                "Orașul J{}: {} clădiri ({}); scor provizoriu {}\n",
                p + 1,
                self.city[p].len() - 1,
                t.iter()
                    .enumerate()
                    .map(|(i, n)| format!("t{}×{}", i + 1, n))
                    .collect::<Vec<_>>()
                    .join(" "),
                self.score(p)
            ));
        }
        if let Some(t) = self.pending {
            s.push_str(&format!("Așază clădirea t{}: pune x,y.\n", t + 1));
        }
        s.push_str(&format!(
            "Pachet: {} monede, {} clădiri\n",
            self.deck.len(),
            self.tiles.len()
        ));
        let v = self.viewer.who(self.side);
        let mut h = self.hand[v.idx()].clone();
        h.sort();
        s.push_str(&format!(
            "Monedele tale (J{}): {}\n",
            v.idx() + 1,
            h.iter().map(|&c| cn(c)).collect::<Vec<_>>().join(" ")
        ));
        s.push_str(&format!(
            "J{} are {} monede\n",
            v.other().idx() + 1,
            self.hand[v.other().idx()].len()
        ));
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        s
    }
    fn format_move(&self, m: &Mv) -> String {
        match m {
            Mv::Take(255) => "pas".into(),
            Mv::Take(i) => format!("iau {}", i + 1),
            Mv::Buy(c) => format!("cumpar {}", (b'a' + c) as char),
            Mv::Place(x, y) => format!("pune {x},{y}"),
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
    fn payment_and_exact_bonus() {
        let mut g = Palate::with_seed(1);
        g.hand[0] = vec![(0, 5), (0, 3), (0, 9), (1, 2)];
        g.shop[0] = Some((2, 8));
        let pay = g.payment(0, 0, 8).unwrap();
        let sum: i32 = pay.iter().map(|&i| g.hand[0][i].1 as i32).sum();
        assert_eq!(sum, 8); // 5+3 exact
        g.apply(&Mv::Buy(0));
        assert_eq!(g.pending, Some(2));
        let pl = g.legal_moves();
        assert!(pl.contains(&Mv::Place(1, 0)));
        g.apply(&Mv::Place(1, 0));
        assert_eq!(g.to_move(), Player::One); // tură în plus
        assert_eq!(g.city[0].len(), 2);
    }
    #[test]
    fn hidden_hands() {
        let g = Palate::with_seed(5);
        let f = |who| {
            let mut x = g.clone();
            x.set_viewer(Some(who));
            x.render()
                .lines()
                .find(|l| l.starts_with("Monedele tale"))
                .unwrap()
                .split_once(": ")
                .unwrap()
                .1
                .to_string()
        };
        assert_ne!(f(Player::One), f(Player::Two));
    }
    #[test]
    fn random_and_ai() {
        check(Palate::with_seed, 700);
    }
}
