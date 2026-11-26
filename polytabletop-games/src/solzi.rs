//! Solzi — motor de cărți-animale cu habitate, hrană și ouă, variantă originală pentru doi.
//!
//! Tema noastră: reptile și amfibieni. Fiecare jucător are un teren cu 3 habitate (Pădure P,
//! Stâncă S, Baltă B), câte 5 locuri, și joacă 4 runde cu 8, 7, 6, 5 acțiuni. Hrana: insecte (I),
//! viermi (V), pește (F), șoareci (M).
//!
//! Acțiuni (una pe tură):
//! * `joc 2 b` — joci cartea 2 din mână în habitatul B (dacă cartea trăiește acolo și mai e loc):
//!   plătești hrana din colțul cărții și ouă după coloană (0, 1, 1, 2, 2 ouă; ouăle se iau de pe
//!   vietățile cu cele mai multe ouă);
//! * `padure I V` — Pădurea dă hrană: 1 + (vietăți în Pădure)/2 unități, alese de tine; apoi
//!   fiecare vietate din Pădure cu putere „hrană” dă încă o unitate din tipul ei;
//! * `stanca p1 s2` — Stânca dă ouă: 1 + (vietăți în Stâncă)/2 ouă pe locurile alese (cu
//!   capacitate liberă); apoi vietățile din Stâncă cu putere „ou” pun câte un ou pe ele;
//! * `balta` — Balta trage 1 + (vietăți în Baltă)/2 cărți; apoi puterile „carte” trag câte una.
//! Puterile se declanșează doar dacă vietatea stă în habitatul acțiunii (o vietate cu putere
//! de hrană pusă în Baltă nu face nimic).
//!
//! La sfârșitul fiecărei runde: obiectiv comun (rundele 1–4: cele mai multe ouă / vietăți /
//! puncte de pe cărți / hrană în rezervă) = 4, 5, 6, 7 puncte pentru câștigător (egal: jumătate
//! fiecare). Scor final: punctele cărților, ouăle, obiectivele. Cărțile sunt generate de noi.
//! Informație ascunsă: pachetul și mâna adversarului; render() arată doar mâna J1. AI-ul
//! numără cărțile din mână, nu identitatea lor.
#![allow(clippy::needless_range_loop, clippy::doc_lazy_continuation)]

use crate::cards::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const FOOD: [char; 4] = ['I', 'V', 'F', 'M'];
pub const HAB: [char; 3] = ['P', 'S', 'B'];
const EGG_COST: [u8; 5] = [0, 1, 1, 2, 2];
const CUBES: [u8; 4] = [8, 7, 6, 5];
const NAMES: [&str; 30] = [
    "Broasca de lac",
    "Broasca roșie",
    "Broasca râioasă",
    "Brotăcelul",
    "Buhaiul de baltă",
    "Tritonul crestat",
    "Tritonul carpatic",
    "Salamandra",
    "Izvoraș cu burta galbenă",
    "Țestoasa de apă",
    "Țestoasa de uscat",
    "Șarpele de casă",
    "Șarpele de apă",
    "Vipera cu corn",
    "Vipera comună",
    "Năpârca",
    "Gușterul de câmp",
    "Șopârla de zid",
    "Șopârla de nisip",
    "Gușterul vânăt",
    "Șarpele Esculap",
    "Șarpele de alun",
    "Broasca de copac",
    "Tritonul alpin",
    "Țestoasa mlăștinoasă",
    "Șopârla de stâncă",
    "Gușterul cu burta roșie",
    "Șarpele cu dungi",
    "Buhaiul cu burta galbenă",
    "Salamandra de peșteră",
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Power {
    None,
    Food(u8),
    Egg,
    Draw,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Creature {
    pub id: u8,
    pub name: &'static str,
    /// Biți: 1 Pădure, 2 Stâncă, 4 Baltă.
    pub hab: u8,
    pub cost: [u8; 4],
    pub pts: u8,
    pub cap: u8,
    pub power: Power,
}

impl Creature {
    fn text(&self) -> String {
        let hab: String = (0..3)
            .filter(|h| self.hab & (1 << h) != 0)
            .map(|h| HAB[h])
            .collect();
        let cost: Vec<String> = (0..4)
            .filter(|f| self.cost[*f] > 0)
            .map(|f| format!("{}{}", self.cost[f], FOOD[f]))
            .collect();
        let pw = match self.power {
            Power::None => String::new(),
            Power::Food(t) => format!(" +{}", FOOD[t as usize]),
            Power::Egg => " +ou".into(),
            Power::Draw => " +carte".into(),
        };
        format!(
            "{} [{} | {} | {}p | ouă {}{}]",
            self.name,
            hab,
            cost.join(" "),
            self.pts,
            self.cap,
            pw
        )
    }
}

fn all_creatures() -> Vec<Creature> {
    let mut rng = Rng::new(0x5017_2025);
    let mut v = Vec::new();
    for i in 0..60usize {
        let h1 = i % 3;
        let mut hab = 1u8 << h1;
        if rng.below(3) == 0 {
            hab |= 1 << ((h1 + 1 + rng.below(2)) % 3);
        }
        let k = 1 + rng.below(3);
        let mut cost = [0u8; 4];
        let t1 = rng.below(4);
        let two = rng.below(2) == 0 && k > 1;
        let t2 = (t1 + 1 + rng.below(3)) % 4;
        for j in 0..k {
            if two && j % 2 == 1 {
                cost[t2] += 1;
            } else {
                cost[t1] += 1;
            }
        }
        let has_power = rng.below(100) < 55;
        let power = if has_power {
            match h1 {
                0 => Power::Food(rng.below(4) as u8),
                1 => Power::Egg,
                _ => Power::Draw,
            }
        } else {
            Power::None
        };
        let base = k as u8 + rng.below(3) as u8;
        let pts = if has_power {
            base.saturating_sub(1)
        } else {
            base + 1
        };
        v.push(Creature {
            id: i as u8,
            name: NAMES[i % 30],
            hab,
            cost,
            pts: pts.min(7),
            cap: 1 + rng.below(4) as u8,
            power,
        });
    }
    v
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    /// carte din mână, habitat
    Play(u8, u8),
    /// multiset de hrană ca numărători (câte I, V, F, M)
    Gather([u8; 4]),
    /// locuri (habitat*5+poziție), până la 3 (255 = gol)
    Lay([u8; 3]),
    Draw,
}

#[derive(Clone)]
pub struct Solzi {
    /// Whose private info `render` shows (`None` = the mover's, hot-seat).
    viewer: Option<Player>,
    deck: Vec<Creature>,
    hand: [Vec<Creature>; 2],
    row: [[Vec<(Creature, u8)>; 3]; 2],
    food: [[u8; 4]; 2],
    goal_pts: [u8; 2],
    round: usize,
    cubes: [u8; 2],
    side: Player,
    starter: Player,
    over: bool,
    /// Cărți trase în tura curentă (AI-ul nu le evaluează).
    fresh: [u8; 2],
}

impl Default for Solzi {
    fn default() -> Self {
        Self::new()
    }
}

impl Solzi {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck = all_creatures();
        shuffle(&mut deck, &mut rng);
        let mut g = Solzi {
            viewer: None,
            deck,
            hand: [Vec::new(), Vec::new()],
            row: [
                [Vec::new(), Vec::new(), Vec::new()],
                [Vec::new(), Vec::new(), Vec::new()],
            ],
            food: [[2, 1, 1, 1]; 2],
            goal_pts: [0; 2],
            round: 0,
            cubes: [CUBES[0]; 2],
            side: Player::One,
            starter: Player::One,
            over: false,
            fresh: [0; 2],
        };
        for p in 0..2 {
            for _ in 0..5 {
                g.draw(p);
            }
            g.fresh[p] = 0;
        }
        g
    }

    fn draw(&mut self, p: usize) {
        if let Some(c) = self.deck.pop() {
            self.hand[p].push(c);
            self.fresh[p] += 1;
        }
    }

    pub fn card_total(&self) -> usize {
        self.deck.len()
            + (0..2)
                .map(|p| self.hand[p].len() + self.row[p].iter().map(|r| r.len()).sum::<usize>())
                .sum::<usize>()
    }

    pub fn egg_total(&self, p: usize) -> u32 {
        self.row[p]
            .iter()
            .flatten()
            .map(|(_, e)| u32::from(*e))
            .sum()
    }

    fn creature_count(&self, p: usize) -> usize {
        self.row[p].iter().map(|r| r.len()).sum()
    }

    fn card_pts(&self, p: usize) -> u32 {
        self.row[p]
            .iter()
            .flatten()
            .map(|(c, _)| u32::from(c.pts))
            .sum()
    }

    fn food_total(&self, p: usize) -> u32 {
        self.food[p].iter().map(|x| u32::from(*x)).sum()
    }

    pub fn score(&self, p: usize) -> u32 {
        self.card_pts(p) + self.egg_total(p) + u32::from(self.goal_pts[p])
    }

    fn free_slots(&self, p: usize) -> Vec<(usize, usize, u8)> {
        let mut v = Vec::new();
        for h in 0..3 {
            for (i, (c, e)) in self.row[p][h].iter().enumerate() {
                if *e < c.cap {
                    v.push((h, i, c.cap - e));
                }
            }
        }
        v
    }

    fn units(&self, p: usize, h: usize) -> usize {
        1 + self.row[p][h].len() / 2
    }

    fn can_pay(&self, p: usize, c: &Creature, col: usize) -> bool {
        (0..4).all(|f| self.food[p][f] >= c.cost[f])
            && self.egg_total(p) >= u32::from(EGG_COST[col])
    }

    fn take_eggs(&mut self, p: usize, n: u8) {
        for _ in 0..n {
            let mut best: Option<(usize, usize, u8)> = None;
            for h in 0..3 {
                for (i, (_, e)) in self.row[p][h].iter().enumerate() {
                    if *e > 0 && best.map(|b| *e > b.2).unwrap_or(true) {
                        best = Some((h, i, *e));
                    }
                }
            }
            if let Some((h, i, _)) = best {
                self.row[p][h][i].1 -= 1;
            }
        }
    }

    fn slot_label(h: usize, i: usize) -> String {
        format!("{}{}", HAB[h].to_ascii_lowercase(), i + 1)
    }

    fn end_action(&mut self) {
        let p = self.side.idx();
        self.cubes[p] -= 1;
        if self.cubes[0] == 0 && self.cubes[1] == 0 {
            self.end_round();
            return;
        }
        let o = self.side.other();
        if self.cubes[o.idx()] > 0 {
            self.side = o;
        }
    }

    fn end_round(&mut self) {
        let m: [u32; 2] = match self.round {
            0 => [self.egg_total(0), self.egg_total(1)],
            1 => [self.creature_count(0) as u32, self.creature_count(1) as u32],
            2 => [self.card_pts(0), self.card_pts(1)],
            _ => [self.food_total(0), self.food_total(1)],
        };
        let win = 4 + self.round as u8;
        if m[0] > m[1] {
            self.goal_pts[0] += win;
        } else if m[1] > m[0] {
            self.goal_pts[1] += win;
        } else {
            self.goal_pts[0] += win / 2;
            self.goal_pts[1] += win / 2;
        }
        self.round += 1;
        if self.round >= 4 {
            self.over = true;
            return;
        }
        self.cubes = [CUBES[self.round]; 2];
        self.starter = self.starter.other();
        self.side = self.starter;
    }

    fn heuristic(&self, p: usize) -> f64 {
        let mut s = 4.0 * f64::from(self.card_pts(p)) + 2.6 * f64::from(self.egg_total(p));
        s += 4.0 * f64::from(self.goal_pts[p]);
        let food = self.food_total(p);
        s += 0.8 * f64::from(food.min(8)) + 0.15 * f64::from(food.saturating_sub(8));
        s += 1.2 * self.hand[p].len() as f64;
        // cărțile din mână (fără cele trase acum) cu hrana pe care o ai deja
        let known = self.hand[p].len().saturating_sub(self.fresh[p] as usize);
        for c in self.hand[p].iter().take(known) {
            let need: u32 = c.cost.iter().map(|x| u32::from(*x)).sum();
            let met: u32 = (0..4)
                .map(|f| u32::from(self.food[p][f].min(c.cost[f])))
                .sum();
            s += 3.0 * f64::from(met) / f64::from(need.max(1));
        }
        let left = f64::from(self.cubes[p]) + 3.0 * (3 - self.round.min(3)) as f64;
        for h in 0..3 {
            s += 6.0 * self.row[p][h].len() as f64;
            for (c, _) in &self.row[p][h] {
                let bonus = match (c.power, h) {
                    (Power::Food(_), 0) | (Power::Egg, 1) | (Power::Draw, 2) => 0.5,
                    _ => 0.0,
                };
                s += bonus * left;
            }
        }
        s
    }
}

fn multisets(units: usize, k: usize) -> Vec<Vec<usize>> {
    // toate multisetele de mărime `units` din k tipuri, ca vectori de numărători
    fn rec(k: usize, left: usize, cur: &mut Vec<usize>, out: &mut Vec<Vec<usize>>) {
        if cur.len() == k - 1 {
            let mut v = cur.clone();
            v.push(left);
            out.push(v);
            return;
        }
        for x in 0..=left {
            cur.push(x);
            rec(k, left - x, cur, out);
            cur.pop();
        }
    }
    let mut out = Vec::new();
    rec(k, units, &mut Vec::new(), &mut out);
    out
}

impl Game for Solzi {
    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over {
            return Vec::new();
        }
        let p = self.side.idx();
        let mut v = Vec::new();
        // joacă o vietate
        for (i, c) in self.hand[p].iter().enumerate() {
            for h in 0..3u8 {
                if c.hab & (1 << h) == 0 {
                    continue;
                }
                let col = self.row[p][h as usize].len();
                if col < 5 && self.can_pay(p, c, col) {
                    v.push(Move::Play(i as u8, h));
                }
            }
        }
        // hrană
        for ms in multisets(self.units(p, 0), 4) {
            v.push(Move::Gather([
                ms[0] as u8,
                ms[1] as u8,
                ms[2] as u8,
                ms[3] as u8,
            ]));
        }
        // ouă
        let free = self.free_slots(p);
        let cap: usize = free.iter().map(|x| x.2 as usize).sum();
        let n = self.units(p, 1).min(cap);
        if n == 0 {
            v.push(Move::Lay([255; 3]));
        } else {
            // combinații cu repetiție de locuri (respectând capacitatea)
            let slots: Vec<(u8, u8)> = free
                .iter()
                .map(|(h, i, c)| ((h * 5 + i) as u8, *c))
                .collect();
            let mut cur: Vec<u8> = Vec::new();
            fn rec(
                slots: &[(u8, u8)],
                start: usize,
                n: usize,
                cur: &mut Vec<u8>,
                out: &mut Vec<Move>,
            ) {
                if cur.len() == n {
                    let mut a = [255u8; 3];
                    for (k, x) in cur.iter().enumerate() {
                        a[k] = *x;
                    }
                    out.push(Move::Lay(a));
                    return;
                }
                for s in start..slots.len() {
                    let used = cur.iter().filter(|x| **x == slots[s].0).count() as u8;
                    if used < slots[s].1 {
                        cur.push(slots[s].0);
                        rec(slots, s, n, cur, out);
                        cur.pop();
                    }
                }
            }
            rec(&slots, 0, n, &mut cur, &mut v);
        }
        if !self.deck.is_empty() {
            v.push(Move::Draw);
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        let p = self.side.idx();
        self.fresh[p] = 0;
        self.fresh[1 - p] = 0;
        match *m {
            Move::Play(i, h) => {
                let c = self.hand[p].remove(i as usize);
                let col = self.row[p][h as usize].len();
                for f in 0..4 {
                    self.food[p][f] -= c.cost[f];
                }
                self.take_eggs(p, EGG_COST[col]);
                self.row[p][h as usize].push((c, 0));
            }
            Move::Gather(n) => {
                for f in 0..4 {
                    self.food[p][f] += n[f];
                }
                for (c, _) in self.row[p][0].clone() {
                    if let Power::Food(t) = c.power {
                        self.food[p][t as usize] += 1;
                    }
                }
            }
            Move::Lay(a) => {
                for s in a {
                    if s != 255 {
                        let (h, i) = (s as usize / 5, s as usize % 5);
                        self.row[p][h][i].1 += 1;
                    }
                }
                for i in 0..self.row[p][1].len() {
                    if self.row[p][1][i].0.power == Power::Egg
                        && self.row[p][1][i].1 < self.row[p][1][i].0.cap
                    {
                        self.row[p][1][i].1 += 1;
                    }
                }
            }
            Move::Draw => {
                let n = self.units(p, 2);
                for _ in 0..n {
                    self.draw(p);
                }
                let extra = self.row[p][2]
                    .iter()
                    .filter(|(c, _)| c.power == Power::Draw)
                    .count();
                for _ in 0..extra {
                    self.draw(p);
                }
            }
        }
        self.end_action();
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            return None;
        }
        let (a, b) = (self.score(0), self.score(1));
        Some(if a > b {
            Outcome::Win(Player::One)
        } else if b > a {
            Outcome::Win(Player::Two)
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
        let goals = [
            "cele mai multe ouă",
            "cele mai multe vietăți",
            "cele mai multe puncte pe cărți",
            "cea mai multă hrană",
        ];
        let mut s = String::new();
        if self.over {
            s.push_str("Solzi — joc încheiat\n");
        } else {
            s.push_str(&format!(
                "Solzi — runda {} din 4 (obiectiv: {}, {} p) | acțiuni rămase J1 {} · J2 {}\n",
                self.round + 1,
                goals[self.round],
                4 + self.round,
                self.cubes[0],
                self.cubes[1]
            ));
        }
        for p in 0..2 {
            let f: Vec<String> = (0..4)
                .map(|i| format!("{}{}", FOOD[i], self.food[p][i]))
                .collect();
            s.push_str(&format!(
                "J{}: scor {} (cărți {}, ouă {}, obiective {}) | hrană {} | mâna {} cărți\n",
                p + 1,
                self.score(p),
                self.card_pts(p),
                self.egg_total(p),
                self.goal_pts[p],
                f.join(" "),
                self.hand[p].len()
            ));
            for h in 0..3 {
                let items: Vec<String> = self.row[p][h]
                    .iter()
                    .enumerate()
                    .map(|(i, (c, e))| {
                        format!("{}={}[{}/{}ouă]", Self::slot_label(h, i), c.name, e, c.cap)
                    })
                    .collect();
                s.push_str(&format!(
                    "   {}: {}\n",
                    ["Pădure", "Stâncă", "Baltă"][h],
                    if items.is_empty() {
                        "—".to_string()
                    } else {
                        items.join("  ")
                    }
                ));
            }
        }
        let v = self.viewer.unwrap_or_else(|| self.to_move()).idx();
        s.push_str(&format!(
            "Pachet: {} cărți\nMâna ta (J{}):\n",
            self.deck.len(),
            v + 1
        ));
        for (i, c) in self.hand[v].iter().enumerate() {
            s.push_str(&format!("  {}: {}\n", i + 1, c.text()));
        }
        if !self.over {
            s.push_str(&format!("Mută J{}.\n", self.side.idx() + 1));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match *m {
            Move::Play(i, h) => format!("joc {} {}", i + 1, HAB[h as usize].to_ascii_lowercase()),
            Move::Gather(n) => {
                let mut t = String::from("padure");
                for f in 0..4 {
                    for _ in 0..n[f] {
                        t.push(' ');
                        t.push(FOOD[f]);
                    }
                }
                t
            }
            Move::Lay(a) => {
                let mut t = String::from("stanca");
                for s in a {
                    if s != 255 {
                        t.push(' ');
                        t.push_str(&Self::slot_label(s as usize / 5, s as usize % 5));
                    }
                }
                t
            }
            Move::Draw => "balta".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cr(hab: u8, cost: [u8; 4], pts: u8, cap: u8, power: Power) -> Creature {
        Creature {
            id: 99,
            name: "t",
            hab,
            cost,
            pts,
            cap,
            power,
        }
    }

    #[test]
    fn deck_is_sane() {
        let v = all_creatures();
        assert_eq!(v.len(), 60);
        for c in &v {
            assert!(c.cost.iter().sum::<u8>() >= 1);
            assert!(c.hab != 0 && c.cap >= 1);
        }
    }

    #[test]
    fn playing_costs_food_and_eggs() {
        let mut g = Solzi::with_seed(1);
        g.food[0] = [3, 0, 0, 0];
        g.hand[0] = vec![cr(1, [2, 0, 0, 0], 3, 2, Power::None)];
        g.row[0][0] = vec![(cr(1, [1, 0, 0, 0], 1, 3, Power::None), 2)];
        g.row[0][1] = vec![(cr(2, [1, 0, 0, 0], 1, 3, Power::None), 0)];
        // coloana 1 în Pădure costă 1 ou
        g.hand[0] = vec![cr(1, [2, 0, 0, 0], 3, 2, Power::None)];
        assert!(g.legal_moves().contains(&Move::Play(0, 0)));
        let eggs = g.egg_total(0);
        g.apply(&Move::Play(0, 0));
        assert_eq!(g.food[0], [1, 0, 0, 0]);
        assert_eq!(g.egg_total(0), eggs - 1);
        // fără hrană: nu poate
        let mut g = Solzi::with_seed(2);
        g.food[0] = [1, 0, 0, 0];
        g.hand[0] = vec![cr(1, [2, 0, 0, 0], 3, 2, Power::None)];
        assert!(!g.legal_moves().iter().any(|m| matches!(m, Move::Play(..))));
    }

    #[test]
    fn habitat_powers_fire_only_at_home() {
        let mut g = Solzi::with_seed(3);
        g.food[0] = [0; 4];
        // vietate cu putere de hrană în Pădure (1 vietate -> 1 unitate) -> +1 din tip + unitatea aleasă
        g.row[0][0] = vec![(cr(1, [1, 0, 0, 0], 1, 1, Power::Food(2)), 0)];
        g.apply(&Move::Gather([1, 0, 0, 0]));
        assert_eq!(g.food[0], [1, 0, 1, 0]);
        // aceeași putere în Baltă nu face nimic: Baltă dă doar cărți
        let mut g = Solzi::with_seed(4);
        g.row[0][2] = vec![(cr(1 | 4, [1, 0, 0, 0], 1, 1, Power::Food(2)), 0)];
        let before = g.food[0];
        g.apply(&Move::Draw);
        assert_eq!(g.food[0], before);
        assert_eq!(g.hand[0].len(), 5 + 1);
    }

    #[test]
    fn egg_action_respects_capacity() {
        let mut g = Solzi::with_seed(5);
        g.row[0][1] = vec![(cr(2, [1, 0, 0, 0], 1, 1, Power::Egg), 0)];
        g.row[0][0] = vec![(cr(1, [1, 0, 0, 0], 1, 2, Power::None), 0)];
        for m in g.legal_moves() {
            if let Move::Lay(a) = m {
                // 1 + 1/2 = 1 ou
                assert_eq!(a.iter().filter(|x| **x != 255).count(), 1);
            }
        }
        assert!(g.legal_moves().contains(&Move::Lay([5, 255, 255])));
        g.apply(&Move::Lay([0, 255, 255]));
        assert_eq!(g.row[0][0][0].1, 1);
        assert_eq!(g.row[0][1][0].1, 1); // puterea „ou”
    }

    #[test]
    fn goals_and_end() {
        let mut g = Solzi::with_seed(6);
        g.row[0][0] = vec![(cr(1, [1, 0, 0, 0], 1, 3, Power::None), 3)];
        g.cubes = [0, 0];
        g.end_round();
        assert_eq!(g.goal_pts, [4, 0]);
        assert_eq!(g.round, 1);
        assert_eq!(g.cubes, [7, 7]);
    }

    #[test]
    fn random_games_finish_and_conserve_cards() {
        for seed in 1..40u64 {
            let mut g = Solzi::with_seed(seed);
            let t = g.card_total();
            let mut rng = Rng::new(seed ^ 3);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                assert_eq!(g.card_total(), t);
                n += 1;
                assert!(n < 100);
            }
        }
    }

    #[test]
    fn ai_beats_random() {
        let mut wins = 0;
        for seed in 1..=30u64 {
            let mut g = Solzi::with_seed(seed);
            let mut rng = Rng::new(seed + 3);
            while g.outcome().is_none() {
                let m = if g.to_move() == Player::One {
                    polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap()
                } else {
                    let l = g.legal_moves();
                    l[rng.below(l.len())]
                };
                g.apply(&m);
            }
            if g.outcome() == Some(Outcome::Win(Player::One)) {
                wins += 1;
            }
        }
        assert!(wins >= 25, "wins {wins}");
    }

    #[test]
    fn render_hides_the_opponents_hand() {
        let mut g = Solzi::with_seed(9);
        g.hand[1] = vec![cr(1, [1, 0, 0, 0], 1, 1, Power::None)];
        g.hand[1][0].name = "SECRETNAME";
        g.hand[0][0].name = "VISIBLENAME";
        let r = g.render();
        assert!(!r.contains("SECRETNAME"));
        assert!(r.contains("VISIBLENAME"));
    }
}
