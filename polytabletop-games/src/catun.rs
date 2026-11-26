//! Cătun — plasare de muncitori și un sat de cărți, variantă originală pentru doi.
//!
//! Resurse: Lemn (L), Rășină (R), Piatră (P), Mure (M). Fiecare jucător își face un cătun din
//! cel mult 15 cărți: Locuitori (se plătesc de obicei cu mure/rășină) și Clădiri (lemn,
//! rășină, piatră). Tipuri de cărți: Producătoare (dau resurse când sunt jucate și la fiecare
//! pregătire de sezon), Daruri (un câștig imediat), Reduceri (locuitorii sau clădirile costă o
//! unitate mai puțin — din primul tip prezent în ordinea L R P M), Prosperitate (puncte la
//! final) și simple (doar puncte).
//!
//! Fiecare jucător își parcurge singur anotimpurile Iarnă → Primăvară → Vară → Toamnă, cu 2, 3,
//! 4 și 6 muncitori. O tură = o singură acțiune:
//! * `lucru a` — pui un muncitor pe o locație (a–h; câte un muncitor al tău pe locație;
//!   locațiile „exclusive” nu mai primesc niciun alt muncitor până la pregătire);
//!   `lucru g L` — Hanul cere și alegerea unei resurse;
//! * `joc 3` / `piata 5` — joci o carte din mână / din piață (8 cărți vizibile), plătind costul;
//! * `sezon` — te pregătești pentru anotimpul următor: muncitorii se întorc, primești muncitorii
//!   noi, (Iarnă→Primăvară și Vară→Toamnă) producătoarele tale lucrează, (Primăvară→Vară)
//!   tragi 2 cărți. `sezon` în Toamnă te scoate din joc. Jocul se termină când ambii au terminat.
//!
//! Locații: a Tăietura (excl., 3 L), b Cariera (excl., 2 P), c Rășinăria (excl., 2 R), d Poiana
//! (2 M), e Pădurea mică (1 L + 1 R), f Târgul (2 cărți), g Hanul (excl., 1 carte + 1 resursă
//! la alegere), h Biserica (excl., 1 punct + 1 M). Limita mâinii: 8 cărți.
//! Scor: punctele cărților și ale jetoanelor + prosperitate. Egalitate: mai multe resurse.
//!
//! Informație ascunsă: pachetul și mâna adversarului. AI-ul nu citește piața (se reumple
//! la întâmplare) și numără doar cărțile din mână.
#![allow(clippy::needless_range_loop, clippy::doc_lazy_continuation)]

use crate::cards::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const RES: [char; 4] = ['L', 'R', 'P', 'M'];
const CITY_MAX: usize = 15;
const HAND_MAX: usize = 8;
const WORKERS: [u8; 4] = [2, 3, 4, 6];
const LOC_NAMES: [&str; 8] = [
    "Tăietura",
    "Cariera",
    "Rășinăria",
    "Poiana",
    "Pădurea mică",
    "Târgul",
    "Hanul",
    "Biserica",
];
const LOC_EXCL: [bool; 8] = [true, true, true, false, false, false, true, true];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CK {
    Plain,
    /// producător: resursă, cantitate
    Prod(u8, u8),
    /// dar: resursă, cantitate
    Gift(u8, u8),
    /// dar de cărți
    Draw(u8),
    /// reducere: 0 locuitori, 1 clădiri
    Disc(u8),
    /// prosperitate: 0 clădiri, 1 locuitori, 2 producătoare, 3 câte 1 punct la 2 resurse rămase
    Pros(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Card {
    pub id: u8,
    pub name: &'static str,
    pub building: bool,
    pub kind: CK,
    pub cost: [u8; 4],
    pub pts: u8,
}

const VILLAGERS: [&str; 24] = [
    "Ciobanul",
    "Morarul",
    "Fierarul",
    "Tâmplarul",
    "Brutăreasa",
    "Țesătoarea",
    "Vânătorul",
    "Culegătoarea",
    "Pescarul",
    "Olarul",
    "Dascălul",
    "Doctorița",
    "Căruțașul",
    "Cojocarul",
    "Mierarul",
    "Povestitorul",
    "Pădurarul",
    "Lăutarul",
    "Cântărețul",
    "Mesagerul",
    "Prispășița",
    "Stăpânul de stână",
    "Primarul",
    "Bătrâna satului",
];
const BUILDINGS: [&str; 24] = [
    "Moara",
    "Fierăria",
    "Stâna",
    "Hanul vechi",
    "Biserica de lemn",
    "Podul",
    "Fântâna",
    "Hambarul",
    "Atelierul",
    "Școala",
    "Clopotnița",
    "Coliba",
    "Cuptorul",
    "Piața mică",
    "Casa de piatră",
    "Gardul viu",
    "Șura",
    "Pivnița",
    "Observatorul",
    "Teascul",
    "Beciul",
    "Cetatea",
    "Turnul",
    "Via",
];

pub fn all_cards() -> Vec<Card> {
    let mut rng = Rng::new(0xCA70_2025);
    let mut v = Vec::new();
    for i in 0..48usize {
        let building = i >= 24;
        let name = if building {
            BUILDINGS[i - 24]
        } else {
            VILLAGERS[i]
        };
        let j = i % 24;
        let mut cost = [0u8; 4];
        let total = if building {
            2 + rng.below(4)
        } else {
            1 + rng.below(3)
        };
        let main = if building {
            [0usize, 2, 0, 1][rng.below(4)]
        } else {
            [3usize, 3, 1][rng.below(3)]
        };
        let second = if building {
            [1usize, 2][rng.below(2)]
        } else {
            [1usize, 3][rng.below(2)]
        };
        for k in 0..total {
            if k % 2 == 1 && second != main {
                cost[second] += 1;
            } else {
                cost[main] += 1;
            }
        }
        let tot = total as u8;
        let r = rng.below(4) as u8;
        let (kind, pts) = match j % 6 {
            0 | 1 => (CK::Prod(r, 1 + u8::from(tot >= 4)), tot.saturating_sub(1)),
            2 => (CK::Gift(r, 2), tot),
            3 => (CK::Draw(2), tot.saturating_sub(1)),
            4 => (
                if j % 4 == 0 {
                    CK::Disc(u8::from(building))
                } else {
                    CK::Pros((j % 4) as u8 - 1 + u8::from(building))
                },
                tot,
            ),
            _ => (CK::Plain, tot + 1),
        };
        v.push(Card {
            id: i as u8,
            name,
            building,
            kind,
            cost,
            pts,
        });
    }
    v
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Work(u8, u8),
    Hand(u8),
    Meadow(u8),
    Season,
}

#[derive(Clone)]
pub struct Catun {
    /// Whose private info `render` shows (`None` = the mover's, hot-seat).
    viewer: Option<Player>,
    deck: Vec<Card>,
    meadow: Vec<Card>,
    hand: [Vec<Card>; 2],
    city: [Vec<Card>; 2],
    res: [[u8; 4]; 2],
    points: [u8; 2],
    season: [u8; 2],
    done: [bool; 2],
    workers: [u8; 2],
    placed: [[bool; 8]; 2],
    wasted: [u8; 2],
    side: Player,
    fresh: [u8; 2],
}

impl Default for Catun {
    fn default() -> Self {
        Self::new()
    }
}

impl Catun {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck = all_cards();
        shuffle(&mut deck, &mut rng);
        let mut g = Catun {
            viewer: None,
            deck,
            meadow: Vec::new(),
            hand: [Vec::new(), Vec::new()],
            city: [Vec::new(), Vec::new()],
            res: [[1, 1, 0, 1], [1, 1, 0, 1]],
            points: [0; 2],
            season: [0; 2],
            done: [false; 2],
            workers: [WORKERS[0]; 2],
            placed: [[false; 8]; 2],
            wasted: [0; 2],
            side: Player::One,
            fresh: [0; 2],
        };
        for _ in 0..8 {
            if let Some(c) = g.deck.pop() {
                g.meadow.push(c);
            }
        }
        for p in 0..2 {
            for _ in 0..(5 + p) {
                g.draw(p);
            }
            g.fresh[p] = 0;
        }
        g
    }

    fn draw(&mut self, p: usize) {
        if self.hand[p].len() < HAND_MAX {
            if let Some(c) = self.deck.pop() {
                self.hand[p].push(c);
                self.fresh[p] += 1;
            }
        }
    }

    pub fn card_total(&self) -> usize {
        self.deck.len()
            + self.meadow.len()
            + (0..2)
                .map(|p| self.hand[p].len() + self.city[p].len())
                .sum::<usize>()
    }

    fn occupied(&self, loc: usize) -> bool {
        self.placed[0][loc] || self.placed[1][loc]
    }

    fn cost_for(&self, p: usize, c: &Card) -> [u8; 4] {
        let mut cost = c.cost;
        let disc = self.city[p]
            .iter()
            .filter(|x| matches!(x.kind, CK::Disc(d) if (d == 1) == c.building))
            .count();
        for _ in 0..disc {
            if let Some(i) = (0..4).find(|i| cost[*i] > 0) {
                cost[i] -= 1;
            }
        }
        cost
    }

    fn affordable(&self, p: usize, c: &Card) -> bool {
        let cost = self.cost_for(p, c);
        self.city[p].len() < CITY_MAX && (0..4).all(|i| self.res[p][i] >= cost[i])
    }

    fn gain(&mut self, p: usize, r: usize, n: u8) {
        self.res[p][r] = self.res[p][r].saturating_add(n).min(9);
    }

    fn play_card(&mut self, p: usize, c: Card) {
        let cost = self.cost_for(p, &c);
        for i in 0..4 {
            self.res[p][i] -= cost[i];
        }
        match c.kind {
            CK::Prod(r, n) | CK::Gift(r, n) => self.gain(p, r as usize, n),
            CK::Draw(n) => {
                for _ in 0..n {
                    self.draw(p);
                }
            }
            _ => {}
        }
        self.city[p].push(c);
    }

    fn produce(&mut self, p: usize) {
        for c in self.city[p].clone() {
            if let CK::Prod(r, n) = c.kind {
                self.gain(p, r as usize, n);
            }
        }
    }

    fn prepare(&mut self, p: usize) {
        self.wasted[p] +=
            self.workers[p].min(WORKERS[3]) - self.placed[p].iter().filter(|x| **x).count() as u8;
        self.placed[p] = [false; 8];
        let s = self.season[p] as usize;
        if s == 3 {
            self.done[p] = true;
            self.workers[p] = 0;
            return;
        }
        self.season[p] += 1;
        self.workers[p] = WORKERS[s + 1];
        match s {
            0 | 2 => self.produce(p),
            _ => {
                self.draw(p);
                self.draw(p);
            }
        }
    }

    fn free_workers(&self, p: usize) -> u8 {
        self.workers[p] - self.placed[p].iter().filter(|x| **x).count() as u8
    }

    fn prosperity(&self, p: usize) -> u32 {
        let mut s = 0u32;
        let b = self.city[p].iter().filter(|c| c.building).count() as u32;
        let v = self.city[p].len() as u32 - b;
        let pr = self.city[p]
            .iter()
            .filter(|c| matches!(c.kind, CK::Prod(..)))
            .count() as u32;
        let left: u32 = self.res[p].iter().map(|x| u32::from(*x)).sum();
        for c in &self.city[p] {
            if let CK::Pros(k) = c.kind {
                s += match k {
                    0 => b,
                    1 => v,
                    2 => pr,
                    _ => left / 2,
                };
            }
        }
        s
    }

    pub fn score(&self, p: usize) -> u32 {
        self.city[p].iter().map(|c| u32::from(c.pts)).sum::<u32>()
            + u32::from(self.points[p])
            + self.prosperity(p)
    }

    fn heuristic(&self, p: usize) -> f64 {
        let mut s = 4.0 * f64::from(self.score(p));
        s += [1.0, 1.0, 1.1, 0.8]
            .iter()
            .zip(self.res[p].iter())
            .map(|(w, n)| w * f64::from(*n))
            .sum::<f64>();
        s += 1.3 * self.hand[p].len() as f64;
        s -= 2.0 * f64::from(self.wasted[p]);
        let left = 3 - self.season[p].min(3);
        for c in &self.city[p] {
            if let CK::Prod(_, n) = c.kind {
                s += 1.2 * f64::from(n) * f64::from(left.min(2));
            }
        }
        s += 0.5 * self.city[p].len() as f64;
        s
    }

    fn end_turn(&mut self) {
        let o = self.side.other();
        if !self.done[o.idx()] {
            self.side = o;
        }
    }
}

impl Game for Catun {
    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        let p = self.side.idx();
        if self.done[0] && self.done[1] {
            return Vec::new();
        }
        let mut v = Vec::new();
        if self.free_workers(p) > 0 {
            for loc in 0..8usize {
                if self.placed[p][loc] || (LOC_EXCL[loc] && self.occupied(loc)) {
                    continue;
                }
                if loc == 6 {
                    for r in 0..4u8 {
                        v.push(Move::Work(6, r));
                    }
                } else {
                    v.push(Move::Work(loc as u8, 0));
                }
            }
        }
        for (i, c) in self.hand[p].iter().enumerate() {
            if self.affordable(p, c) {
                v.push(Move::Hand(i as u8));
            }
        }
        for (i, c) in self.meadow.iter().enumerate() {
            if self.affordable(p, c) {
                v.push(Move::Meadow(i as u8));
            }
        }
        v.push(Move::Season);
        v
    }

    fn apply(&mut self, m: &Move) {
        let p = self.side.idx();
        self.fresh = [0; 2];
        match *m {
            Move::Work(loc, arg) => {
                self.placed[p][loc as usize] = true;
                match loc {
                    0 => self.gain(p, 0, 3),
                    1 => self.gain(p, 2, 2),
                    2 => self.gain(p, 1, 2),
                    3 => self.gain(p, 3, 2),
                    4 => {
                        self.gain(p, 0, 1);
                        self.gain(p, 1, 1);
                    }
                    5 => {
                        self.draw(p);
                        self.draw(p);
                    }
                    6 => {
                        self.draw(p);
                        self.gain(p, arg as usize, 1);
                    }
                    _ => {
                        self.points[p] += 1;
                        self.gain(p, 3, 1);
                    }
                }
            }
            Move::Hand(i) => {
                let c = self.hand[p].remove(i as usize);
                self.play_card(p, c);
            }
            Move::Meadow(i) => {
                let c = self.meadow.remove(i as usize);
                self.play_card(p, c);
                if let Some(n) = self.deck.pop() {
                    self.meadow.push(n);
                }
            }
            Move::Season => self.prepare(p),
        }
        self.end_turn();
    }

    fn outcome(&self) -> Option<Outcome> {
        if !(self.done[0] && self.done[1]) {
            return None;
        }
        let (a, b) = (self.score(0), self.score(1));
        if a != b {
            return Some(Outcome::Win(if a > b { Player::One } else { Player::Two }));
        }
        let (ra, rb): (u32, u32) = (
            self.res[0].iter().map(|x| u32::from(*x)).sum(),
            self.res[1].iter().map(|x| u32::from(*x)).sum(),
        );
        Some(if ra > rb {
            Outcome::Win(Player::One)
        } else if rb > ra {
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
        let seasons = ["Iarnă", "Primăvară", "Vară", "Toamnă"];
        let ctext = |c: &Card, cost: [u8; 4]| {
            let cs: Vec<String> = (0..4)
                .filter(|i| cost[*i] > 0)
                .map(|i| format!("{}{}", cost[i], RES[i]))
                .collect();
            let k = match c.kind {
                CK::Plain => String::new(),
                CK::Prod(r, n) => format!(" produce {}{}", n, RES[r as usize]),
                CK::Gift(r, n) => format!(" dar {}{}", n, RES[r as usize]),
                CK::Draw(n) => format!(" dar {n} cărți"),
                CK::Disc(0) => " reducere locuitori".into(),
                CK::Disc(_) => " reducere clădiri".into(),
                CK::Pros(0) => " +1/clădire".into(),
                CK::Pros(1) => " +1/locuitor".into(),
                CK::Pros(2) => " +1/producător".into(),
                CK::Pros(_) => " +1 la 2 resurse".into(),
            };
            format!(
                "{} ({}) [{}; {}p{}]",
                c.name,
                if c.building { "clădire" } else { "locuitor" },
                if cs.is_empty() {
                    "—".into()
                } else {
                    cs.join(" ")
                },
                c.pts,
                k
            )
        };
        let mut s = String::from("Cătun\nLocații: ");
        let locs: Vec<String> = (0..8)
            .map(|i| {
                let who = match (self.placed[0][i], self.placed[1][i]) {
                    (true, true) => " [J1 J2]",
                    (true, false) => " [J1]",
                    (false, true) => " [J2]",
                    _ => "",
                };
                format!(
                    "{}={}{}{}",
                    (b'a' + i as u8) as char,
                    LOC_NAMES[i],
                    if LOC_EXCL[i] { "*" } else { "" },
                    who
                )
            })
            .collect();
        s.push_str(&locs.join(", "));
        s.push_str("  (* exclusivă)\nPiața: ");
        let mk: Vec<String> = self
            .meadow
            .iter()
            .enumerate()
            .map(|(i, c)| format!("{}:{}", i + 1, ctext(c, c.cost)))
            .collect();
        s.push_str(&mk.join(" | "));
        s.push_str(&format!("\nPachet: {}\n", self.deck.len()));
        for p in 0..2 {
            let r: Vec<String> = (0..4)
                .map(|i| format!("{}{}", RES[i], self.res[p][i]))
                .collect();
            s.push_str(&format!(
                "J{}: {}{} | scor {} (jetoane {}) | resurse {} | muncitori liberi {}/{} | mâna {} | cătun {}/15\n   ",
                p + 1,
                seasons[self.season[p] as usize],
                if self.done[p] { " (gata)" } else { "" },
                self.score(p),
                self.points[p],
                r.join(" "),
                self.free_workers(p),
                self.workers[p],
                self.hand[p].len(),
                self.city[p].len()
            ));
            let names: Vec<&str> = self.city[p].iter().map(|c| c.name).collect();
            s.push_str(&if names.is_empty() {
                "—".to_string()
            } else {
                names.join(", ")
            });
            s.push('\n');
        }
        let v = self.viewer.unwrap_or_else(|| self.to_move()).idx();
        s.push_str(&format!("Mâna ta (J{}):\n", v + 1));
        for (i, c) in self.hand[v].iter().enumerate() {
            s.push_str(&format!("  {}: {}\n", i + 1, ctext(c, self.cost_for(v, c))));
        }
        if self.outcome().is_none() {
            s.push_str(&format!("Mută J{}.\n", self.side.idx() + 1));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match *m {
            Move::Work(6, r) => format!("lucru g {}", RES[r as usize]),
            Move::Work(l, _) => format!("lucru {}", (b'a' + l) as char),
            Move::Hand(i) => format!("joc {}", i + 1),
            Move::Meadow(i) => format!("piata {}", i + 1),
            Move::Season => "sezon".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn card(kind: CK, cost: [u8; 4], pts: u8, building: bool) -> Card {
        Card {
            id: 200,
            name: "t",
            building,
            kind,
            cost,
            pts,
        }
    }

    #[test]
    fn deck_is_sane() {
        let v = all_cards();
        assert_eq!(v.len(), 48);
        assert_eq!(v.iter().filter(|c| c.building).count(), 24);
        for c in &v {
            assert!(c.cost.iter().sum::<u8>() >= 1);
        }
    }

    #[test]
    fn exclusive_locations_and_workers() {
        let mut g = Catun::with_seed(1);
        g.apply(&Move::Work(0, 0));
        // J2 nu mai poate lua Tăietura (exclusivă), dar poate lua Poiana
        let l = g.legal_moves();
        assert!(!l.contains(&Move::Work(0, 0)));
        assert!(l.contains(&Move::Work(3, 0)));
        g.apply(&Move::Work(3, 0));
        // J1 poate lua Poiana (neexclusivă) o dată
        assert!(g.legal_moves().contains(&Move::Work(3, 0)));
        g.apply(&Move::Work(3, 0));
        assert_eq!(g.res[0][0], 1 + 3);
        assert_eq!(g.res[0][3], 1 + 2);
    }

    #[test]
    fn discount_and_city_limit() {
        let mut g = Catun::with_seed(2);
        g.res[0] = [0, 0, 0, 2];
        g.city[0] = vec![card(CK::Disc(0), [0; 4], 0, true)];
        let c = card(CK::Plain, [0, 0, 0, 2], 1, false);
        assert_eq!(g.cost_for(0, &c), [0, 0, 0, 1]);
        g.res[0] = [0, 0, 0, 1];
        assert!(g.affordable(0, &c));
        g.city[0] = (0..15).map(|_| card(CK::Plain, [0; 4], 0, true)).collect();
        assert!(!g.affordable(0, &c));
    }

    #[test]
    fn season_prepare_workers_and_production() {
        let mut g = Catun::with_seed(3);
        g.city[0] = vec![card(CK::Prod(2, 2), [0; 4], 0, false)];
        g.res[0] = [0; 4];
        g.placed[0][0] = true;
        g.apply(&Move::Season);
        assert_eq!(g.season[0], 1);
        assert_eq!(g.workers[0], 3);
        assert_eq!(g.res[0][2], 2);
        assert!(!g.placed[0][0]);
    }

    #[test]
    fn game_ends_when_both_finish_autumn() {
        let mut g = Catun::with_seed(4);
        for _ in 0..4 {
            g.apply(&Move::Season);
            g.apply(&Move::Season);
        }
        assert!(g.outcome().is_some());
    }

    #[test]
    fn prosperity_scores() {
        let mut g = Catun::with_seed(5);
        g.city[0] = vec![
            card(CK::Pros(0), [0; 4], 0, true),
            card(CK::Plain, [0; 4], 0, true),
            card(CK::Plain, [0; 4], 0, false),
        ];
        assert_eq!(g.prosperity(0), 2);
    }

    #[test]
    fn random_games_finish_and_conserve_cards() {
        for seed in 1..40u64 {
            let mut g = Catun::with_seed(seed);
            let t = g.card_total();
            let mut rng = Rng::new(seed ^ 11);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                assert_eq!(g.card_total(), t);
                n += 1;
                assert!(n < 600);
            }
        }
    }

    #[test]
    fn ai_beats_random() {
        let mut wins = 0;
        for seed in 1..=30u64 {
            let mut g = Catun::with_seed(seed);
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
        let mut g = Catun::with_seed(9);
        g.hand[1][0].name = "SECRETNAME";
        g.hand[0][0].name = "VISIBLENAME";
        let r = g.render();
        assert!(!r.contains("SECRETNAME"));
        assert!(r.contains("VISIBLENAME"));
    }
}
