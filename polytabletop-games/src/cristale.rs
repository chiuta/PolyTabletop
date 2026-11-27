//! Cristale — joc de gemele și dezvoltări pentru doi, variantă originală.
//!
//! Cinci minerale — Granat (G), Ametist (A), Topaz (T), Malachit (M), Cuarț (Q) — și
//! Chihlimbarul (C), joker. Tura ta, o singură acțiune:
//! * `iau G A T` — 3 jetoane de culori diferite (sau câte sunt, dacă s-au epuizat culorile);
//! * `iau2 G` — 2 jetoane de aceeași culoare, doar dacă în grămadă sunt cel puțin 4;
//! * `rezerv a1` — rezervi o carte vizibilă (max. 3) și primești un Chihlimbar, dacă mai e;
//! * `cumpar a1` / `cumpar rez2` — cumperi o carte (nivelurile a, b, c; locurile 1–4).
//! Fiecare carte cumpărată dă o reducere permanentă pe culoarea ei și, uneori, puncte.
//! Cu peste 10 jetoane arunci excedentul (`arunc G`). Dacă după o achiziție îndeplinești
//! cerința unui Patron (3 puncte), acesta te vizitează automat.
//! Cine ajunge la 15 puncte declanșează ultima rundă (se termină după tura Jucătorului 2).
//! Câștigă cel mai multe puncte; la egalitate, cu mai puține cărți cumpărate.
//!
//! Cărțile sunt generate de noi (nu provin de la niciun produs comercial). Simplificări:
//! nu se rezervă din vârful pachetului; Patronul vizitator e primul îndeplinit; plafon
//! de 120 de runde. AI-ul alege după o euristică (fără să vadă pachetele): locul reumplut
//! de pe piață nu intră în evaluare.
#![allow(clippy::needless_range_loop, clippy::doc_lazy_continuation)]

use crate::cards::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const GEMS: [char; 5] = ['G', 'A', 'T', 'M', 'Q'];
pub const GEM_NAMES: [&str; 5] = ["Granat", "Ametist", "Topaz", "Malachit", "Cuarț"];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Card {
    pub bonus: u8,
    pub pts: u8,
    pub cost: [u8; 5],
}

impl Card {
    fn text(&self) -> String {
        let cost: Vec<String> = (0..5)
            .filter(|c| self.cost[*c] > 0)
            .map(|c| format!("{}{}", self.cost[c], GEMS[c]))
            .collect();
        format!(
            "[{} {}p cost {}]",
            GEMS[self.bonus as usize],
            self.pts,
            cost.join(" ")
        )
    }
}

fn level_cards(level: usize) -> Vec<Card> {
    let per = [8, 6, 4][level];
    let mut v = Vec::new();
    for c in 0..5u8 {
        for k in 0..per {
            let mut r = Rng::new(0xC0FFEE + (level * 1000 + c as usize * 100 + k) as u64);
            let (tmin, tspan) = [(3, 4), (5, 5), (8, 6)][level];
            let total = tmin + r.below(tspan);
            let ncols = if level == 2 {
                3 + r.below(2)
            } else {
                2 + r.below(2)
            };
            let mut cols: Vec<usize> = (0..5)
                .filter(|x| *x != c as usize || r.below(6) == 0)
                .collect();
            shuffle(&mut cols, &mut r);
            cols.truncate(ncols.min(cols.len()));
            let mut cost = [0u8; 5];
            for i in 0..total {
                cost[cols[i % cols.len()]] += 1;
            }
            let pts = match level {
                0 => u8::from(total >= 6),
                1 => (1 + (total - 5) / 2).min(3) as u8,
                _ => (3 + (total - 8) / 2).min(5) as u8,
            };
            v.push(Card {
                bonus: c,
                pts,
                cost,
            });
        }
    }
    v
}

fn all_nobles() -> Vec<[u8; 5]> {
    let mut v = Vec::new();
    for c in 0..5usize {
        let mut a = [0u8; 5];
        a[c] = 4;
        a[(c + 1) % 5] = 4;
        v.push(a);
        let mut b = [0u8; 5];
        b[c] = 3;
        b[(c + 2) % 5] = 3;
        b[(c + 3) % 5] = 3;
        v.push(b);
    }
    v
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Take3(u8, u8, u8),
    Take2(u8, u8),
    Take1(u8),
    Reserve(u8, u8),
    Buy(u8, u8),
    BuyReserved(u8),
    Discard(u8),
    Pass,
}

#[derive(Clone)]
pub struct Cristale {
    /// Whose private info `render` shows (`None` = the mover's, hot-seat).
    viewer: Option<Player>,
    decks: [Vec<Card>; 3],
    market: [[Option<Card>; 4]; 3],
    nobles: Vec<[u8; 5]>,
    /// Jetoane în bancă: 5 culori + chihlimbar.
    bank: [u8; 6],
    tokens: [[u8; 6]; 2],
    bonus: [[u8; 5]; 2],
    reserved: [Vec<Card>; 2],
    pts: [u8; 2],
    cards_bought: [u8; 2],
    side: Player,
    discarding: bool,
    last_round: bool,
    rounds: u32,
    over: Option<Outcome>,
    fresh: Option<(usize, usize)>,
}

impl Default for Cristale {
    fn default() -> Self {
        Self::new()
    }
}

impl Cristale {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut decks = [level_cards(0), level_cards(1), level_cards(2)];
        for d in &mut decks {
            shuffle(d, &mut rng);
        }
        let mut nobles = all_nobles();
        shuffle(&mut nobles, &mut rng);
        nobles.truncate(3);
        let mut g = Cristale {
            viewer: None,
            decks,
            market: [[None; 4]; 3],
            nobles,
            bank: [4, 4, 4, 4, 4, 5],
            tokens: [[0; 6]; 2],
            bonus: [[0; 5]; 2],
            reserved: [Vec::new(), Vec::new()],
            pts: [0; 2],
            cards_bought: [0; 2],
            side: Player::One,
            discarding: false,
            last_round: false,
            rounds: 1,
            over: None,
            fresh: None,
        };
        for l in 0..3 {
            for i in 0..4 {
                g.market[l][i] = g.decks[l].pop();
            }
        }
        g
    }

    pub fn points(&self) -> (u8, u8) {
        (self.pts[0], self.pts[1])
    }

    /// Jetoane totale (bancă + jucători): conservare.
    pub fn token_total(&self) -> [u32; 6] {
        let mut t = [0u32; 6];
        for c in 0..6 {
            t[c] = u32::from(self.bank[c])
                + u32::from(self.tokens[0][c])
                + u32::from(self.tokens[1][c]);
        }
        t
    }

    pub fn card_total(&self) -> usize {
        self.decks.iter().map(|d| d.len()).sum::<usize>()
            + self.market.iter().flatten().filter(|x| x.is_some()).count()
            + self.reserved.iter().map(|r| r.len()).sum::<usize>()
            + self.cards_bought.iter().map(|x| *x as usize).sum::<usize>()
    }

    fn held(&self, p: usize) -> u32 {
        self.tokens[p].iter().map(|x| u32::from(*x)).sum()
    }

    /// Cât aur ar costa cartea pentru `p` (None dacă nu poate plăti).
    fn gold_needed(&self, p: usize, card: &Card) -> Option<u8> {
        let mut gold = 0u8;
        for c in 0..5 {
            let need = card.cost[c].saturating_sub(self.bonus[p][c]);
            if need > self.tokens[p][c] {
                gold += need - self.tokens[p][c];
            }
        }
        if gold <= self.tokens[p][5] {
            Some(gold)
        } else {
            None
        }
    }

    /// Câte jetoane lipsesc ca `p` să poată cumpăra cartea.
    fn missing(&self, p: usize, card: &Card) -> u8 {
        let mut gold = 0u8;
        for c in 0..5 {
            let need = card.cost[c].saturating_sub(self.bonus[p][c]);
            gold += need.saturating_sub(self.tokens[p][c]);
        }
        gold.saturating_sub(self.tokens[p][5])
    }

    fn pay_and_get(&mut self, p: usize, card: Card) {
        for c in 0..5 {
            let need = card.cost[c].saturating_sub(self.bonus[p][c]);
            let use_c = need.min(self.tokens[p][c]);
            self.tokens[p][c] -= use_c;
            self.bank[c] += use_c;
            let rest = need - use_c;
            self.tokens[p][5] -= rest;
            self.bank[5] += rest;
        }
        self.bonus[p][card.bonus as usize] += 1;
        self.pts[p] += card.pts;
        self.cards_bought[p] += 1;
        // Patron.
        if let Some(i) = self
            .nobles
            .iter()
            .position(|n| (0..5).all(|c| self.bonus[p][c] >= n[c]))
        {
            self.nobles.remove(i);
            self.pts[p] += 3;
        }
    }

    fn heuristic(&self, p: usize) -> f64 {
        let mut s = 100.0 * f64::from(self.pts[p]);
        let nb: f64 = self.bonus[p].iter().map(|x| f64::from(*x)).sum();
        s += 14.0 * nb;
        // Bonusurile apropiate de un Patron.
        for n in &self.nobles {
            let mut miss = 0.0;
            for c in 0..5 {
                miss += f64::from(n[c].saturating_sub(self.bonus[p][c]));
            }
            s += 24.0 / (1.0 + miss);
        }
        let tok: f64 = (0..5).map(|c| f64::from(self.tokens[p][c])).sum();
        s += 3.0 * tok + 5.0 * f64::from(self.tokens[p][5]);
        if self.held(p) > 10 {
            s -= 20.0 * f64::from(self.held(p) - 10);
        }
        // Apropierea de cea mai bună carte vizibilă (fără locul reumplut).
        let mut best = 0.0f64;
        for l in 0..3 {
            for i in 0..4 {
                if self.fresh == Some((l, i)) {
                    continue;
                }
                if let Some(c) = &self.market[l][i] {
                    let m = f64::from(self.missing(p, c));
                    let v = (f64::from(c.pts) * 18.0 + 12.0) / (1.0 + 1.6 * m);
                    best = best.max(v);
                }
            }
        }
        for c in &self.reserved[p] {
            let m = f64::from(self.missing(p, c));
            best = best.max((f64::from(c.pts) * 18.0 + 12.0) / (1.0 + 1.6 * m));
        }
        s += best;
        s += 4.0 * self.reserved[p].len() as f64;
        s
    }

    fn end_turn(&mut self) {
        let p = self.side.idx();
        if self.held(p) > 10 {
            self.discarding = true;
            return;
        }
        self.discarding = false;
        if self.pts[p] >= 15 {
            self.last_round = true;
        }
        if self.side == Player::Two {
            self.rounds += 1;
            if self.last_round || self.rounds > 120 {
                let (a, b) = (self.pts[0], self.pts[1]);
                self.over = Some(if a != b {
                    Outcome::Win(if a > b { Player::One } else { Player::Two })
                } else if self.cards_bought[0] != self.cards_bought[1] {
                    Outcome::Win(if self.cards_bought[0] < self.cards_bought[1] {
                        Player::One
                    } else {
                        Player::Two
                    })
                } else {
                    Outcome::Draw
                });
                return;
            }
        }
        self.side = self.side.other();
    }

    fn refill(&mut self, l: usize, i: usize) {
        self.market[l][i] = self.decks[l].pop();
        self.fresh = Some((l, i));
    }
}

fn gem_name(c: u8) -> char {
    if c == 5 {
        'C'
    } else {
        GEMS[c as usize]
    }
}

impl Game for Cristale {
    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over.is_some() {
            return Vec::new();
        }
        let p = self.side.idx();
        let mut v = Vec::new();
        if self.discarding {
            for c in 0..6u8 {
                if self.tokens[p][c as usize] > 0 {
                    v.push(Move::Discard(c));
                }
            }
            return v;
        }
        let avail: Vec<u8> = (0..5u8).filter(|c| self.bank[*c as usize] > 0).collect();
        if avail.len() >= 3 {
            for i in 0..avail.len() {
                for j in i + 1..avail.len() {
                    for k in j + 1..avail.len() {
                        v.push(Move::Take3(avail[i], avail[j], avail[k]));
                    }
                }
            }
        } else if avail.len() == 2 {
            v.push(Move::Take3(avail[0], avail[1], 9));
        } else if avail.len() == 1 {
            v.push(Move::Take1(avail[0]));
        }
        for c in 0..5u8 {
            if self.bank[c as usize] >= 4 {
                v.push(Move::Take2(c, c));
            }
        }
        for l in 0..3u8 {
            for i in 0..4u8 {
                if let Some(card) = &self.market[l as usize][i as usize] {
                    if self.reserved[p].len() < 3 {
                        v.push(Move::Reserve(l, i));
                    }
                    if self.gold_needed(p, card).is_some() {
                        v.push(Move::Buy(l, i));
                    }
                }
            }
        }
        for (i, c) in self.reserved[p].iter().enumerate() {
            if self.gold_needed(p, c).is_some() {
                v.push(Move::BuyReserved(i as u8));
            }
        }
        if v.is_empty() {
            v.push(Move::Pass);
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        let p = self.side.idx();
        self.fresh = None;
        match *m {
            Move::Take3(a, b, c) => {
                for x in [a, b, c] {
                    if x < 5 {
                        self.bank[x as usize] -= 1;
                        self.tokens[p][x as usize] += 1;
                    }
                }
                self.end_turn();
            }
            Move::Take2(c, _) => {
                self.bank[c as usize] -= 2;
                self.tokens[p][c as usize] += 2;
                self.end_turn();
            }
            Move::Take1(c) => {
                self.bank[c as usize] -= 1;
                self.tokens[p][c as usize] += 1;
                self.end_turn();
            }
            Move::Reserve(l, i) => {
                let card = self.market[l as usize][i as usize].take().expect("card");
                self.reserved[p].push(card);
                if self.bank[5] > 0 {
                    self.bank[5] -= 1;
                    self.tokens[p][5] += 1;
                }
                self.refill(l as usize, i as usize);
                self.end_turn();
            }
            Move::Buy(l, i) => {
                let card = self.market[l as usize][i as usize].take().expect("card");
                self.pay_and_get(p, card);
                self.refill(l as usize, i as usize);
                self.end_turn();
            }
            Move::BuyReserved(i) => {
                let card = self.reserved[p].remove(i as usize);
                self.pay_and_get(p, card);
                self.end_turn();
            }
            Move::Discard(c) => {
                self.tokens[p][c as usize] -= 1;
                self.bank[c as usize] += 1;
                self.end_turn();
            }
            Move::Pass => self.end_turn(),
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.over
    }

    fn evaluate(&self, p: Player) -> i32 {
        (self.heuristic(p.idx()) - self.heuristic(p.other().idx())) as i32
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }

    fn render(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            "Cristale — runda {} | puncte: J1 {}, J2 {}{}\n",
            self.rounds,
            self.pts[0],
            self.pts[1],
            if self.last_round {
                " | ULTIMA RUNDĂ"
            } else {
                ""
            }
        ));
        for (l, name) in ["a", "b", "c"].iter().enumerate().rev() {
            let row: Vec<String> = (0..4)
                .map(|i| match &self.market[l][i] {
                    Some(c) => format!("{name}{}={}", i + 1, c.text()),
                    None => format!("{name}{}=—", i + 1),
                })
                .collect();
            s.push_str(&format!(
                "Nivel {} ({} în pachet): {}\n",
                name,
                self.decks[l].len(),
                row.join("  ")
            ));
        }
        let nobles: Vec<String> = self
            .nobles
            .iter()
            .map(|n| {
                (0..5)
                    .filter(|c| n[*c] > 0)
                    .map(|c| format!("{}{}", n[c], GEMS[c]))
                    .collect::<Vec<_>>()
                    .join("+")
            })
            .collect();
        s.push_str(&format!("Patroni (3p): {}\n", nobles.join(" | ")));
        let bank: Vec<String> = (0..6)
            .map(|c| format!("{}{}", gem_name(c as u8), self.bank[c]))
            .collect();
        s.push_str(&format!("Banca: {}\n", bank.join(" ")));
        for p in 0..2 {
            let t: Vec<String> = (0..6)
                .map(|c| format!("{}{}", gem_name(c as u8), self.tokens[p][c]))
                .collect();
            let b: Vec<String> = (0..5)
                .map(|c| format!("{}{}", GEMS[c], self.bonus[p][c]))
                .collect();
            s.push_str(&format!(
                "J{}: jetoane {} | reduceri {} | cărți {} | rezervate: {}\n",
                p + 1,
                t.join(" "),
                b.join(" "),
                self.cards_bought[p],
                if self.reserved[p].is_empty() {
                    "—".to_string()
                } else if p != self.viewer.unwrap_or_else(|| self.to_move()).idx() {
                    format!("{} ascunse", self.reserved[p].len())
                } else {
                    self.reserved[p]
                        .iter()
                        .enumerate()
                        .map(|(i, c)| format!("rez{}={}", i + 1, c.text()))
                        .collect::<Vec<_>>()
                        .join(" ")
                }
            ));
        }
        if self.over.is_none() {
            s.push_str(&format!(
                "Mută J{}{}.\n",
                self.side.idx() + 1,
                if self.discarding {
                    " (aruncă jetoane peste 10)"
                } else {
                    ""
                }
            ));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match *m {
            Move::Take3(a, b, c) => {
                let mut t = String::from("iau");
                for x in [a, b, c] {
                    if x < 5 {
                        t.push(' ');
                        t.push(GEMS[x as usize]);
                    }
                }
                t
            }
            Move::Take2(c, _) => format!("iau2 {}", GEMS[c as usize]),
            Move::Take1(c) => format!("iau {}", GEMS[c as usize]),
            Move::Reserve(l, i) => format!("rezerv {}{}", ['a', 'b', 'c'][l as usize], i + 1),
            Move::Buy(l, i) => format!("cumpar {}{}", ['a', 'b', 'c'][l as usize], i + 1),
            Move::BuyReserved(i) => format!("cumpar rez{}", i + 1),
            Move::Discard(c) => format!("arunc {}", gem_name(c)),
            Move::Pass => "pas".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_data_sane() {
        for (l, n) in [(0, 40), (1, 30), (2, 20)] {
            let v = level_cards(l);
            assert_eq!(v.len(), n);
            for c in &v {
                assert!(c.cost.iter().map(|x| *x as u32).sum::<u32>() >= 3);
            }
        }
        assert_eq!(all_nobles().len(), 10);
    }

    #[test]
    fn take_rules() {
        let mut g = Cristale::with_seed(1);
        let l = g.legal_moves();
        assert!(l.contains(&Move::Take3(0, 1, 2)));
        assert!(l.contains(&Move::Take2(3, 3)));
        g.apply(&Move::Take2(3, 3));
        assert_eq!(g.bank[3], 2);
        // J2: nu mai poate lua 2 din Malachit.
        assert!(!g.legal_moves().contains(&Move::Take2(3, 3)));
    }

    #[test]
    fn buy_with_gold_and_bonus() {
        let mut g = Cristale::with_seed(2);
        let card = Card {
            bonus: 2,
            pts: 2,
            cost: [2, 0, 3, 0, 0],
        };
        g.market[0][0] = Some(card);
        g.bonus[0][0] = 1;
        g.tokens[0] = [0, 0, 2, 0, 0, 1];
        g.bank = [4, 4, 2, 4, 4, 4];
        // necesar: G 1, T 3 -> are T2 + aur1 (lipsește G1 -> aur) = nevoie de 2 aur: nu poate
        assert!(!g.legal_moves().contains(&Move::Buy(0, 0)));
        g.tokens[0][5] = 2;
        g.bank[5] = 3;
        assert!(g.legal_moves().contains(&Move::Buy(0, 0)));
        let before = g.token_total();
        g.apply(&Move::Buy(0, 0));
        assert_eq!(g.token_total(), before);
        assert_eq!(g.pts[0], 2);
        assert_eq!(g.bonus[0][2], 1);
        assert_eq!(g.tokens[0], [0; 6]);
    }

    #[test]
    fn discard_over_ten_and_reserve() {
        let mut g = Cristale::with_seed(3);
        g.tokens[0] = [3, 3, 2, 0, 0, 0];
        g.bank = [1, 1, 2, 4, 4, 5];
        g.apply(&Move::Take3(3, 4, 0));
        assert!(g.discarding);
        assert_eq!(g.to_move(), Player::One);
        assert!(g
            .legal_moves()
            .iter()
            .all(|m| matches!(m, Move::Discard(_))));
        g.apply(&Move::Discard(0));
        assert!(!g.discarding);
        assert_eq!(g.to_move(), Player::Two);
        let mut g = Cristale::with_seed(4);
        g.apply(&Move::Reserve(1, 2));
        assert_eq!(g.reserved[0].len(), 1);
        assert_eq!(g.tokens[0][5], 1);
        assert!(g.market[1][2].is_some());
    }

    #[test]
    fn noble_and_end() {
        let mut g = Cristale::with_seed(5);
        g.nobles = vec![[3, 3, 0, 0, 0]];
        g.bonus[0] = [3, 2, 0, 0, 0];
        g.market[0][0] = Some(Card {
            bonus: 1,
            pts: 0,
            cost: [0; 5],
        });
        g.apply(&Move::Buy(0, 0));
        assert_eq!(g.pts[0], 3);
        assert!(g.nobles.is_empty());
        // 15 puncte: ultima rundă, J2 mai mută.
        let mut g = Cristale::with_seed(6);
        g.pts[0] = 15;
        g.apply(&Move::Take3(0, 1, 2));
        assert!(g.last_round);
        assert!(g.outcome().is_none());
        g.apply(&Move::Take3(0, 1, 2));
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn random_games_end_and_conserve() {
        for seed in 1..25 {
            let mut g = Cristale::with_seed(seed);
            let t = g.token_total();
            let c = g.card_total();
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                assert_eq!(g.token_total(), t);
                assert_eq!(g.card_total(), c);
                assert!(g.held(0) <= 13 && g.held(1) <= 13);
                n += 1;
                assert!(n < 5000);
            }
        }
    }

    #[test]
    fn ai_beats_random() {
        let mut wins = 0;
        for seed in 1..=30u64 {
            let mut g = Cristale::with_seed(seed);
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
        assert!(wins >= 27, "wins {wins}");
    }
}

#[cfg(test)]
mod a1_blind {
    use super::*;
    use crate::ai_audit_a1::{audit, shuffle};

    #[test]
    fn ai_ignores_the_order_of_the_decks_at_all_levels() {
        audit("cristale", Cristale::with_seed, 1..=4, 6, 200, |g, salt| {
            for (l, d) in g.decks.iter_mut().enumerate() {
                shuffle(d, salt + l as u64);
            }
        });
    }
}
