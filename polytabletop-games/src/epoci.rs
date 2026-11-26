//! Epoci — draft de cărți în trei epoci cu comerț între vecini, variantă originală.
//!
//! Mecanica „alegi o carte și pasezi restul mâinii” pentru TREI orașe: Jucătorul 1, Jucătorul 2
//! și Orașul Oarb (automatul). Cele trei orașe stau într-un inel, deci fiecare are doi vecini.
//!
//! # Orașul Oarb (automatul)
//! Nu are minune și nu intră în clasament; contează doar ca vecin: produce, cumpără nimic,
//! are armată și vinde resurse. Pe rând, din mâna lui construiește gratuit (doar din
//! producția proprie și monede) cea mai prețuită carte posibilă, în ordinea: militară,
//! știință, civică, materii prime, produse prelucrate, comerț, breaslă; dacă nu poate
//! construi nimic, vinde prima carte pentru 3 monede. Când cumperi resurse, plătești întâi
//! Orașul Oarb (prețul e același la ambii vecini), apoi celălalt jucător.
//!
//! # Desfășurare
//! 3 epoci; în fiecare, fiecare oraș primește 7 cărți și joacă 6 ture: tu apoi J2 alegeți o
//! carte din mână, mâinile se pasează (epocile 1 și 3: J1→J2→Oarb→J1; epoca 2 în sens
//! invers); ultima carte rămasă se aruncă. Mutările sunt secvențiale: AI-ul nu folosește
//! cartea pe care a jucat-o adversarul în aceeași tură; resursele de cumpărat de la vecini
//! se iau din starea de la începutul turei. Împărțirea epocii următoare e mutare separată
//! (`imparte`), ca AI-ul să nu vadă viitorul.
//!
//! Mutări: `joc 3` (construiește cartea 3 plătind costul), `etapa 3` (cartea 3 cu fața în jos
//! construiește următoarea etapă a minunii), `vinde 3` (+3 monede).
//!
//! Resurse: L lemn, P piatră, U lut, S sare (materii prime); N pânză, G sticlă, H hârtie
//! (prelucrate). Ce nu produci, cumperi de la vecini: 2 monede/unitate (1 cu cartea de
//! reducere potrivită). Pierzi cumpărând doar ce vecinul chiar produce (cărți și orașul).
//!
//! # Scor
//! Puncte civice + etape ale minunii, +1/+3/+5 la fiecare victorie militară (epoca 1/2/3) și
//! −1 la fiecare înfrângere (comparând scuturile cu fiecare vecin la sfârșitul epocii),
//! 1 punct la 3 monede, știință (fiecare simbol: pătratul numărului + 7 pe set de trei),
//! bresle. Cărțile sunt ale noastre; fără lanțuri de construcție (simplificare).
#![allow(clippy::needless_range_loop, clippy::doc_lazy_continuation)]

use crate::cards::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const RES: [char; 7] = ['L', 'P', 'U', 'S', 'N', 'G', 'H'];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum K {
    Raw(u8, u8),
    Man(u8),
    Civic(u8),
    Mil(u8),
    Sci(u8),
    /// 0 reducere materii prime, 1 reducere prelucrate, 2 monede
    Trade(u8, u8),
    Guild(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Card {
    pub id: u8,
    pub name: &'static str,
    pub kind: K,
    pub cost: [u8; 7],
    pub coin: u8,
}

fn cst(v: &[(usize, u8)]) -> [u8; 7] {
    let mut c = [0u8; 7];
    for &(r, n) in v {
        c[r] += n;
    }
    c
}

const L: usize = 0;
const P: usize = 1;
const U: usize = 2;
const S: usize = 3;
const N: usize = 4;
const G: usize = 5;
const H: usize = 6;

fn age_cards(age: usize) -> Vec<Card> {
    let raw = |name, r: usize, q, coin| (name, K::Raw(r as u8, q), [0u8; 7], coin);
    let man = |name, r: usize| (name, K::Man(r as u8), [0u8; 7], 0u8);
    let v: Vec<(&'static str, K, [u8; 7], u8)> = match age {
        0 => vec![
            raw("Crâng", L, 1, 0),
            raw("Cărbunărie", P, 1, 0),
            raw("Lutărie", U, 1, 0),
            raw("Salină", S, 1, 0),
            raw("Cherestea", L, 1, 1),
            raw("Carieră", P, 1, 1),
            raw("Cărămidărie", U, 1, 1),
            raw("Ocnă", S, 1, 1),
            man("Războaie de țesut", N),
            man("Sticlărie", G),
            man("Tipografie", H),
            ("Fântână", K::Civic(3), cst(&[(L, 1)]), 0),
            ("Altar", K::Civic(3), cst(&[(P, 1)]), 0),
            ("Moară de apă", K::Civic(3), cst(&[(U, 1)]), 0),
            ("Palisadă", K::Mil(1), cst(&[(L, 1)]), 0),
            ("Turn de pază", K::Mil(1), cst(&[(U, 1)]), 0),
            ("Cumpăna", K::Sci(0), cst(&[(N, 1)]), 0),
            ("Observator", K::Sci(1), cst(&[(G, 1)]), 0),
            ("Școala satului", K::Sci(2), cst(&[(H, 1)]), 0),
            ("Han", K::Trade(0, 0), [0; 7], 0),
            ("Piață", K::Trade(1, 0), [0; 7], 0),
        ],
        1 => vec![
            raw("Gater", L, 2, 1),
            raw("Cariera mare", P, 2, 1),
            raw("Groapă de lut", U, 2, 1),
            raw("Ocna veche", S, 2, 1),
            man("Țesătorie mare", N),
            man("Cuptor de sticlă", G),
            man("Presă de hârtie", H),
            ("Băi publice", K::Civic(4), cst(&[(U, 2)]), 0),
            ("Templu", K::Civic(4), cst(&[(P, 2), (G, 1)]), 0),
            ("Tribunal", K::Civic(5), cst(&[(L, 2), (N, 1)]), 0),
            ("Arhive", K::Civic(4), cst(&[(N, 1), (H, 1)]), 0),
            ("Cetățuie", K::Civic(5), cst(&[(P, 2), (S, 1)]), 0),
            ("Cazarmă", K::Mil(2), cst(&[(L, 1), (P, 1)]), 0),
            ("Poligon", K::Mil(2), cst(&[(U, 1), (S, 1)]), 0),
            ("Arsenal", K::Mil(2), cst(&[(L, 2)]), 0),
            ("Laborator", K::Sci(0), cst(&[(N, 1), (L, 1)]), 0),
            ("Bibliotecă", K::Sci(1), cst(&[(G, 1), (P, 1)]), 0),
            ("Atelier de ceasuri", K::Sci(2), cst(&[(H, 1), (U, 1)]), 0),
            ("Vamă", K::Trade(2, 4), [0; 7], 0),
            ("Han de drum", K::Trade(0, 0), [0; 7], 0),
            ("Bursă", K::Trade(1, 0), [0; 7], 0),
        ],
        _ => vec![
            (
                "Palat",
                K::Civic(8),
                cst(&[(P, 2), (U, 1), (G, 1), (H, 1)]),
                0,
            ),
            ("Catedrală", K::Civic(7), cst(&[(P, 3), (G, 1)]), 0),
            ("Primărie", K::Civic(6), cst(&[(P, 2), (L, 1), (N, 1)]), 0),
            ("Grădină regală", K::Civic(5), cst(&[(L, 2), (U, 1)]), 0),
            ("Observație", K::Civic(6), cst(&[(S, 2), (H, 1), (N, 1)]), 0),
            ("Statuie", K::Civic(6), cst(&[(U, 2), (P, 1), (S, 1)]), 0),
            ("Fortăreață", K::Mil(3), cst(&[(P, 2), (S, 1), (N, 1)]), 0),
            ("Zid de apărare", K::Mil(3), cst(&[(U, 3), (L, 1)]), 0),
            ("Armurărie", K::Mil(3), cst(&[(L, 2), (P, 1), (G, 1)]), 0),
            ("Universitate", K::Sci(0), cst(&[(L, 2), (G, 1), (H, 1)]), 0),
            ("Academie", K::Sci(1), cst(&[(P, 2), (N, 1), (H, 1)]), 0),
            (
                "Cabinetul alchimistului",
                K::Sci(2),
                cst(&[(U, 2), (G, 1), (N, 1)]),
                0,
            ),
            ("Observator mare", K::Sci(0), cst(&[(S, 2), (H, 2)]), 0),
            ("Târg mare", K::Trade(2, 5), cst(&[(L, 1)]), 0),
            ("Bancă", K::Trade(2, 6), cst(&[(P, 1)]), 0),
            ("Casa breslelor", K::Trade(2, 4), [0; 7], 0),
            (
                "Breasla armurierilor",
                K::Guild(0),
                cst(&[(P, 2), (U, 1), (N, 1)]),
                0,
            ),
            (
                "Breasla notarilor",
                K::Guild(1),
                cst(&[(L, 2), (H, 1), (N, 1)]),
                0,
            ),
            (
                "Breasla constructorilor",
                K::Guild(2),
                cst(&[(P, 2), (L, 1), (G, 1)]),
                0,
            ),
            (
                "Breasla cărturarilor",
                K::Guild(3),
                cst(&[(S, 2), (H, 1), (G, 1)]),
                0,
            ),
            ("Casa vămii", K::Trade(2, 4), cst(&[(U, 1)]), 0),
        ],
    };
    v.into_iter()
        .enumerate()
        .map(|(i, (name, kind, cost, coin))| Card {
            id: (age * 21 + i) as u8,
            name,
            kind,
            cost,
            coin,
        })
        .collect()
}

/// Etape de minune: (cost, efect). Efect: 0 puncte, 1 scuturi, 2 monede; valoare.
const WONDERS: [[([u8; 7], u8, u8); 3]; 2] = [
    [
        ([2, 0, 0, 0, 0, 0, 0], 0, 3),
        ([0, 3, 0, 0, 0, 0, 0], 1, 2),
        ([0, 0, 2, 0, 1, 0, 0], 0, 7),
    ],
    [
        ([0, 2, 0, 0, 0, 0, 0], 0, 3),
        ([3, 0, 0, 0, 0, 0, 0], 2, 5),
        ([0, 0, 0, 2, 0, 1, 0], 0, 7),
    ],
];
const CITY_RES: [usize; 3] = [L, P, U];
const CITY_NAMES: [&str; 3] = ["J1", "J2", "Orașul Oarb"];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Build(u8),
    Stage(u8),
    Sell(u8),
    Deal,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Turn(u8),
    Deal,
    Done,
}

#[derive(Clone)]
pub struct Epoci {
    /// Whose private info `render` shows (`None` = the mover's, hot-seat).
    viewer: Option<Player>,
    age: usize,
    turn: u8,
    hands: [Vec<Card>; 3],
    tableau: [Vec<Card>; 3],
    coins: [u8; 3],
    stages: [u8; 2],
    tokens: [i32; 3],
    snap: [[u8; 7]; 3],
    discarded: u8,
    phase: Phase,
    rng: Rng,
}

impl Default for Epoci {
    fn default() -> Self {
        Self::new()
    }
}

impl Epoci {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Epoci {
            viewer: None,
            age: 0,
            turn: 0,
            hands: [Vec::new(), Vec::new(), Vec::new()],
            tableau: [Vec::new(), Vec::new(), Vec::new()],
            coins: [3; 3],
            stages: [0; 2],
            tokens: [0; 3],
            snap: [[0; 7]; 3],
            discarded: 0,
            phase: Phase::Turn(0),
            rng: Rng::new(seed),
        };
        g.deal();
        g
    }

    fn deal(&mut self) {
        let mut deck = age_cards(self.age);
        shuffle(&mut deck, &mut self.rng);
        for h in 0..3 {
            let mut hand: Vec<Card> = deck.drain(..7).collect();
            hand.sort_by_key(|c| c.id);
            self.hands[h] = hand;
        }
        self.turn = 0;
        self.phase = Phase::Turn(0);
        self.take_snapshot();
    }

    /// Producția cumpărabilă a fiecărui oraș (la începutul turei).
    fn production(&self, p: usize) -> [u8; 7] {
        let mut v = [0u8; 7];
        v[CITY_RES[p]] += 1;
        for c in &self.tableau[p] {
            match c.kind {
                K::Raw(r, q) => v[r as usize] += q,
                K::Man(r) => v[r as usize] += 1,
                _ => {}
            }
        }
        v
    }

    fn take_snapshot(&mut self) {
        for p in 0..3 {
            self.snap[p] = self.production(p);
        }
    }

    fn has_trade(&self, p: usize, k: u8) -> bool {
        self.tableau[p]
            .iter()
            .any(|c| matches!(c.kind, K::Trade(x, _) if x == k))
    }

    /// Cât costă să-ți procuri `req` (resurse) + `coin` monede. Întoarce plățile (către 0,1,2).
    fn plan(&self, p: usize, req: [u8; 7], coin: u8) -> Option<[u8; 3]> {
        let own = self.production(p);
        let mut pay = [0u8; 3];
        let mut total = u32::from(coin);
        let order: Vec<usize> = if p == 2 { vec![] } else { vec![2, 1 - p] };
        for r in 0..7 {
            let mut rem = req[r].saturating_sub(own[r]);
            if rem == 0 {
                continue;
            }
            let disc = if r < 4 {
                self.has_trade(p, 0)
            } else {
                self.has_trade(p, 1)
            };
            let price = if disc { 1u8 } else { 2 };
            for &j in &order {
                let take = rem.min(self.snap[j][r]);
                pay[j] += take * price;
                total += u32::from(take * price);
                rem -= take;
            }
            if rem > 0 {
                return None;
            }
        }
        if total > u32::from(self.coins[p]) {
            return None;
        }
        Some(pay)
    }

    fn can_build(&self, p: usize, c: &Card) -> Option<[u8; 3]> {
        self.plan(p, c.cost, c.coin)
    }

    fn can_stage(&self, p: usize) -> Option<[u8; 3]> {
        if p > 1 || self.stages[p] >= 3 {
            return None;
        }
        self.plan(p, WONDERS[p][self.stages[p] as usize].0, 0)
    }

    fn pay(&mut self, p: usize, pay: [u8; 3], coin: u8) {
        let total: u8 = pay.iter().sum::<u8>() + coin;
        self.coins[p] -= total;
        for j in 0..3 {
            self.coins[j] += pay[j];
        }
    }

    fn do_build(&mut self, p: usize, i: usize, pay: [u8; 3]) {
        let c = self.hands[p].remove(i);
        self.pay(p, pay, c.coin);
        if let K::Trade(2, n) = c.kind {
            self.coins[p] += n;
        }
        self.tableau[p].push(c);
    }

    fn do_stage(&mut self, p: usize, i: usize, pay: [u8; 3]) {
        self.hands[p].remove(i);
        self.pay(p, pay, 0);
        let (_, eff, val) = WONDERS[p][self.stages[p] as usize];
        if eff == 2 {
            self.coins[p] += val;
        }
        self.stages[p] += 1;
    }

    fn do_sell(&mut self, p: usize, i: usize) {
        self.hands[p].remove(i);
        self.coins[p] += 3;
        self.discarded += 1;
    }

    fn auto_priority(c: &Card) -> i32 {
        match c.kind {
            K::Mil(s) => 70 + i32::from(s),
            K::Sci(_) => 60,
            K::Civic(p) => 50 + i32::from(p),
            K::Raw(_, q) => 40 + i32::from(q),
            K::Man(_) => 30,
            K::Trade(..) => 20,
            K::Guild(_) => 10,
        }
    }

    fn auto_turn(&mut self) {
        let mut best: Option<(i32, usize, [u8; 3])> = None;
        for (i, c) in self.hands[2].clone().iter().enumerate() {
            if let Some(pay) = self.can_build(2, c) {
                let pr = Self::auto_priority(c);
                if best.map(|b| pr > b.0).unwrap_or(true) {
                    best = Some((pr, i, pay));
                }
            }
        }
        match best {
            Some((_, i, pay)) => self.do_build(2, i, pay),
            None => self.do_sell(2, 0),
        }
    }

    fn rotate(&mut self) {
        let old = std::mem::take(&mut self.hands);
        let mut new: [Vec<Card>; 3] = [Vec::new(), Vec::new(), Vec::new()];
        for (i, h) in old.into_iter().enumerate() {
            let to = if self.age.is_multiple_of(2) {
                (i + 1) % 3
            } else {
                (i + 2) % 3
            };
            new[to] = h;
        }
        self.hands = new;
    }

    fn shields(&self, p: usize) -> u32 {
        let mut s = 0u32;
        for c in &self.tableau[p] {
            if let K::Mil(x) = c.kind {
                s += u32::from(x);
            }
        }
        if p < 2 {
            for k in 0..self.stages[p] as usize {
                let (_, eff, val) = WONDERS[p][k];
                if eff == 1 {
                    s += u32::from(val);
                }
            }
        }
        s
    }

    fn end_age(&mut self) {
        let win = [1, 3, 5][self.age];
        for a in 0..3 {
            for b in a + 1..3 {
                let (x, y) = (self.shields(a), self.shields(b));
                if x > y {
                    self.tokens[a] += win;
                    self.tokens[b] -= 1;
                } else if y > x {
                    self.tokens[b] += win;
                    self.tokens[a] -= 1;
                }
            }
        }
        for p in 0..3 {
            self.discarded += self.hands[p].len() as u8;
            self.hands[p].clear();
        }
        if self.age == 2 {
            self.phase = Phase::Done;
        } else {
            self.age += 1;
            self.phase = Phase::Deal;
        }
    }

    fn finish_turn(&mut self) {
        self.auto_turn();
        self.turn += 1;
        if self.turn == 6 {
            self.end_age();
        } else {
            self.rotate();
            self.phase = Phase::Turn(0);
            self.take_snapshot();
        }
    }

    fn sci_score(&self, p: usize) -> i32 {
        let mut n = [0i32; 3];
        for c in &self.tableau[p] {
            if let K::Sci(s) = c.kind {
                n[s as usize] += 1;
            }
        }
        n.iter().map(|x| x * x).sum::<i32>() + 7 * n.iter().copied().min().unwrap_or(0)
    }

    fn guild_score(&self, p: usize) -> i32 {
        let mut s = 0;
        for c in &self.tableau[p] {
            if let K::Guild(g) = c.kind {
                s += match g {
                    0 => (0..3)
                        .map(|q| {
                            self.tableau[q]
                                .iter()
                                .filter(|x| matches!(x.kind, K::Mil(_)))
                                .count() as i32
                        })
                        .sum::<i32>(),
                    1 => self.tableau[p]
                        .iter()
                        .filter(|x| matches!(x.kind, K::Civic(_)))
                        .count() as i32,
                    2 => 2 * (self.stages[0] + self.stages[1]) as i32,
                    _ => {
                        2 * self.tableau[p]
                            .iter()
                            .filter(|x| matches!(x.kind, K::Sci(_)))
                            .count() as i32
                    }
                };
            }
        }
        s
    }

    fn civic_score(&self, p: usize) -> i32 {
        let mut s = 0;
        for c in &self.tableau[p] {
            if let K::Civic(x) = c.kind {
                s += i32::from(x);
            }
        }
        if p < 2 {
            for k in 0..self.stages[p] as usize {
                let (_, eff, val) = WONDERS[p][k];
                if eff == 0 {
                    s += i32::from(val);
                }
            }
        }
        s
    }

    pub fn score(&self, p: usize) -> i32 {
        self.civic_score(p)
            + self.tokens[p]
            + i32::from(self.coins[p] / 3)
            + self.sci_score(p)
            + self.guild_score(p)
    }

    fn heuristic(&self, p: usize) -> f64 {
        let mut s = f64::from(self.score(p)) * 3.0;
        s += f64::from(self.coins[p]) * 0.4;
        let own = self.production(p);
        let kinds = own.iter().filter(|x| **x > 0).count() as f64;
        let units: f64 = own.iter().map(|x| f64::from(*x)).sum();
        let early = (3 - self.age as i32) as f64;
        s += early * (kinds * 1.4 + units * 0.8);
        s += 2.2 * f64::from(self.shields(p)) * (1.0 + 0.3 * self.age as f64);
        s += f64::from(self.stages[p.min(1)]) * 1.0;
        s
    }

    pub fn card_total(&self) -> usize {
        (0..3)
            .map(|p| self.hands[p].len() + self.tableau[p].len())
            .sum::<usize>()
            + usize::from(self.stages[0] + self.stages[1])
            + usize::from(self.discarded)
    }

    pub fn cards_dealt(&self) -> usize {
        // cărți deja împărțite până acum (inclusiv epoca curentă)
        let ages = if self.phase == Phase::Deal {
            self.age
        } else {
            self.age + 1
        };
        21 * ages
    }

    fn card_text(&self, c: &Card) -> String {
        let body = match c.kind {
            K::Raw(r, q) => format!("produce {}{}", q, RES[r as usize]),
            K::Man(r) => format!("produce {}", RES[r as usize]),
            K::Civic(p) => format!("{p} puncte"),
            K::Mil(s) => format!("{s} scuturi"),
            K::Sci(s) => format!("știință {}", ["roată", "lupă", "carte"][s as usize]),
            K::Trade(0, _) => "vecini: materii prime 1 monedă".into(),
            K::Trade(1, _) => "vecini: prelucrate 1 monedă".into(),
            K::Trade(_, n) => format!("+{n} monede"),
            K::Guild(g) => [
                "breaslă: scuturi",
                "breaslă: civice",
                "breaslă: minuni",
                "breaslă: știință",
            ][g as usize]
                .to_string(),
        };
        let mut cost: Vec<String> = (0..7)
            .filter(|r| c.cost[*r] > 0)
            .map(|r| format!("{}{}", c.cost[r], RES[r]))
            .collect();
        if c.coin > 0 {
            cost.push(format!("{}mon", c.coin));
        }
        format!(
            "{} [{}; cost {}]",
            c.name,
            body,
            if cost.is_empty() {
                "—".to_string()
            } else {
                cost.join(" ")
            }
        )
    }
}

impl Game for Epoci {
    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }
    type Move = Move;

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Turn(1) => Player::Two,
            _ => Player::One,
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        match self.phase {
            Phase::Done => Vec::new(),
            Phase::Deal => vec![Move::Deal],
            Phase::Turn(w) => {
                let p = w as usize;
                let mut v = Vec::new();
                for (i, c) in self.hands[p].iter().enumerate() {
                    if !self.tableau[p].iter().any(|t| t.name == c.name)
                        && self.can_build(p, c).is_some()
                    {
                        v.push(Move::Build(i as u8));
                    }
                    if self.can_stage(p).is_some() {
                        v.push(Move::Stage(i as u8));
                    }
                    v.push(Move::Sell(i as u8));
                }
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        match (self.phase, *m) {
            (Phase::Deal, Move::Deal) => self.deal(),
            (Phase::Turn(w), mv) => {
                let p = w as usize;
                match mv {
                    Move::Build(i) => {
                        let pay = self
                            .can_build(p, &self.hands[p][i as usize])
                            .expect("affordable");
                        self.do_build(p, i as usize, pay);
                    }
                    Move::Stage(i) => {
                        let pay = self.can_stage(p).expect("affordable");
                        self.do_stage(p, i as usize, pay);
                    }
                    Move::Sell(i) => self.do_sell(p, i as usize),
                    Move::Deal => {}
                }
                if w == 0 {
                    self.phase = Phase::Turn(1);
                } else {
                    self.finish_turn();
                }
            }
            _ => {}
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Done {
            return None;
        }
        let (a, b) = (self.score(0), self.score(1));
        Some(if a > b {
            Outcome::Win(Player::One)
        } else if b > a {
            Outcome::Win(Player::Two)
        } else if self.coins[0] != self.coins[1] {
            Outcome::Win(if self.coins[0] > self.coins[1] {
                Player::One
            } else {
                Player::Two
            })
        } else {
            Outcome::Draw
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        (self.heuristic(p.idx()) - self.heuristic(p.other().idx())) as i32
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Epoci — epoca {} din 3, tura {} din 6\n",
            self.age + 1,
            self.turn + 1
        );
        for p in 0..3 {
            let prod = self.production(p);
            let pr: Vec<String> = (0..7)
                .filter(|r| prod[*r] > 0)
                .map(|r| format!("{}{}", prod[r], RES[r]))
                .collect();
            let names: Vec<&str> = self.tableau[p].iter().map(|c| c.name).collect();
            s.push_str(&format!(
                "{}: {} monede, scuturi {}, producție {}{}, scor {}{}\n   oraș: {}\n",
                CITY_NAMES[p],
                self.coins[p],
                self.shields(p),
                pr.join(" "),
                if p < 2 {
                    format!(", minune {}/3", self.stages[p])
                } else {
                    String::new()
                },
                self.score(p),
                if self.tokens[p] != 0 {
                    format!(" (militar {:+})", self.tokens[p])
                } else {
                    String::new()
                },
                if names.is_empty() {
                    "—".to_string()
                } else {
                    names.join(", ")
                }
            ));
        }
        if self.phase != Phase::Done {
            let v = self.viewer.unwrap_or_else(|| self.to_move()).idx();
            s.push_str(&format!(
                "Mâna ta (J{}), {} cărți:\n",
                v + 1,
                self.hands[v].len()
            ));
            for (i, c) in self.hands[v].iter().enumerate() {
                s.push_str(&format!("  {}: {}\n", i + 1, self.card_text(c)));
            }
            if self.stages[v] < 3 {
                let (cost, eff, val) = WONDERS[v][self.stages[v] as usize];
                let cs: Vec<String> = (0..7)
                    .filter(|r| cost[*r] > 0)
                    .map(|r| format!("{}{}", cost[r], RES[r]))
                    .collect();
                s.push_str(&format!(
                    "Următoarea etapă a minunii: cost {} → {}\n",
                    cs.join(" "),
                    ["puncte", "scuturi", "monede"][eff as usize].to_string() + &format!(" {val}")
                ));
            }
        }
        match self.phase {
            Phase::Done => s.push_str("Joc încheiat.\n"),
            Phase::Deal => s.push_str("Se împarte epoca următoare (imparte).\n"),
            Phase::Turn(w) => s.push_str(&format!("Mută J{}.\n", w + 1)),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match *m {
            Move::Build(i) => format!("joc {}", i + 1),
            Move::Stage(i) => format!("etapa {}", i + 1),
            Move::Sell(i) => format!("vinde {}", i + 1),
            Move::Deal => "imparte".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn card(kind: K, cost: [u8; 7]) -> Card {
        Card {
            id: 200,
            name: "test",
            kind,
            cost,
            coin: 0,
        }
    }

    #[test]
    fn age_decks_have_21_distinct_cards() {
        for a in 0..3 {
            let v = age_cards(a);
            assert_eq!(v.len(), 21, "age {a}");
            let mut names: Vec<&str> = v.iter().map(|c| c.name).collect();
            names.sort();
            names.dedup();
            assert_eq!(names.len(), 21);
        }
    }

    #[test]
    fn buying_from_neighbours_costs_two_then_one_with_discount() {
        let mut g = Epoci::with_seed(1);
        g.coins = [5, 5, 5];
        // Orașul Oarb produce lut (oraș) + cărți
        g.tableau[2] = vec![card(K::Raw(P as u8, 1), [0; 7])];
        g.take_snapshot();
        // J1 produce lemn; cere 2 piatră: Oarb are 1, J2 (oraș de piatră) 1 -> Oarb primul
        assert_eq!(g.plan(0, cst(&[(P, 2)]), 0), Some([0, 2, 2]));
        assert!(g.plan(0, cst(&[(P, 3)]), 0).is_none());
        // 1 piatră: 2 monede către Oarb
        assert_eq!(g.plan(0, cst(&[(P, 1)]), 0), Some([0, 0, 2]));
        // cu reducere: 1 monedă
        g.tableau[0].push(card(K::Trade(0, 0), [0; 7]));
        assert_eq!(g.plan(0, cst(&[(P, 1)]), 0), Some([0, 0, 1]));
        // prelucrate fără reducere: 2
        g.tableau[2].push(card(K::Man(N as u8), [0; 7]));
        g.take_snapshot();
        assert_eq!(g.plan(0, cst(&[(N, 1)]), 0), Some([0, 0, 2]));
        // după ce a cumpărat, banii merg la vecin
        g.pay(0, [0, 0, 2], 0);
        assert_eq!((g.coins[0], g.coins[2]), (3, 7));
    }

    #[test]
    fn production_of_this_turn_is_not_for_sale() {
        let mut g = Epoci::with_seed(2);
        g.coins = [9, 9, 9];
        let before = g.snap[1];
        g.tableau[1].push(card(K::Raw(S as u8, 2), [0; 7]));
        assert_eq!(g.snap[1], before);
        assert!(g.plan(0, cst(&[(S, 1)]), 0).is_none());
    }

    #[test]
    fn science_and_military_scoring() {
        let mut g = Epoci::with_seed(3);
        g.coins = [0; 3];
        g.tableau[0] = vec![
            card(K::Sci(0), [0; 7]),
            card(K::Sci(0), [0; 7]),
            card(K::Sci(1), [0; 7]),
            card(K::Sci(2), [0; 7]),
        ];
        assert_eq!(g.sci_score(0), 4 + 1 + 1 + 7);
        g.tableau[0] = vec![card(K::Mil(2), [0; 7])];
        g.tableau[1] = vec![card(K::Mil(1), [0; 7])];
        g.tableau[2] = vec![];
        g.hands = [vec![], vec![], vec![]];
        g.end_age();
        // J1 bate pe toți: +1+1; J2 bate Oarb +1, pierde cu J1 -1 => 0
        assert_eq!(g.tokens, [2, 0, -2]);
    }

    #[test]
    fn stage_pays_and_gives_effect() {
        let mut g = Epoci::with_seed(4);
        g.tableau[0] = vec![card(K::Raw(L as u8, 1), [0; 7])];
        g.take_snapshot();
        // etapa 1 a minunii 0 cere 2 lemn: produce 2 (oraș + carte)
        assert!(g.can_stage(0).is_some());
        g.do_stage(0, 0, [0; 3]);
        assert_eq!(g.stages[0], 1);
        assert_eq!(g.civic_score(0), 3);
    }

    #[test]
    fn random_games_finish_and_conserve_cards() {
        for seed in 1..40u64 {
            let mut g = Epoci::with_seed(seed);
            let mut rng = Rng::new(seed ^ 5);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                assert_eq!(g.card_total(), g.cards_dealt(), "seed {seed}");
                n += 1;
                assert!(n < 400);
            }
            assert!(g.stages[0] <= 3 && g.stages[1] <= 3);
        }
    }

    #[test]
    fn ai_beats_random() {
        let mut wins = 0;
        for seed in 1..=30u64 {
            let mut g = Epoci::with_seed(seed);
            let mut rng = Rng::new(seed + 3);
            while g.outcome().is_none() {
                let m = if g.to_move() == Player::Two {
                    polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap()
                } else {
                    let l = g.legal_moves();
                    l[rng.below(l.len())]
                };
                g.apply(&m);
            }
            if g.outcome() == Some(Outcome::Win(Player::Two)) {
                wins += 1;
            }
        }
        assert!(wins >= 24, "wins {wins}");
    }

    #[test]
    fn render_hides_the_other_hands() {
        let mut g = Epoci::with_seed(9);
        g.hands[1][0].name = "SECRETNAME";
        g.hands[2][0].name = "SECRETNAME2";
        g.hands[0][0].name = "VISIBLENAME";
        let r = g.render();
        assert!(!r.contains("SECRETNAME"));
        assert!(r.contains("VISIBLENAME"));
    }
}
