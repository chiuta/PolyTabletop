//! Moșii — joc original de afaceri imobiliare cu zaruri (2 jucători).
//!
//! Mecanica „cumperi proprietăți, încasezi chirii, ruinezi adversarul” e
//! generică; tabla (32 de câmpuri), numele, prețurile, chiriile și evenimentele
//! „Noroc” de aici sunt ale noastre. Fără licitații și fără ipoteci.
//!
//! Reguli:
//! * 1000 de lei fiecare; pornești de la Start; la fiecare trecere prin Start (sau
//!   aterizare pe el) încasezi 150.
//! * Mutări: `arunc` (două zaruri, aruncate automat), `cumpar` / `las`
//!   (proprietate liberă pe care ai aterizat și ți-o permiți), `construiesc N`
//!   (casă pe câmpul N), `platesc` (părăsești închisoarea contra 50).
//! * Chirie = a zecea parte din preț; cu toate câmpurile unui grup chiria se
//!   dublează; cu 1/2/3 case: ×4 / ×10 / ×20. Casele se construiesc doar cu
//!   grupul complet, uniform (nu cu mai mult de o casă peste cel mai slab câmp),
//!   iar costul unei case e fixat pe grup. Gările: 20/40/80/160 după câte ai.
//! * Dubla (zaruri egale) = încă o aruncare; a treia dublă la rând = închisoare.
//!   „Mergi la închisoare” și unele evenimente Noroc te trimit în închisoare:
//!   ieși cu `platesc` (50), cu o dublă la `arunc`, sau automat plătind după a
//!   treia încercare ratată.
//! * Dacă nu poți plăti, ți se vând casele (la jumătate din cost); dacă tot nu
//!   ajunge, ai dat faliment și adversarul câștigă.
//! * Limită: după 1500 de mutări câștigă cine are averea mai mare (bani +
//!   proprietăți + case); la egalitate, remiză.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const N: usize = 32;
const START_CASH: i32 = 1000;
const SALARY: i32 = 150;
const JAIL_FEE: i32 = 50;
const JAIL_POS: usize = 8;
const PLY_CAP: u32 = 1500;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Sq {
    Start,
    Prop(u8), // grup 0..=6
    Station,
    Tax(i32),
    Luck,
    JailVisit,
    Parking,
    GoJail,
}

const BOARD: [(Sq, &str, i32); N] = [
    (Sq::Start, "Start", 0),
    (Sq::Prop(0), "Dealul Mic", 60),
    (Sq::Luck, "Noroc", 0),
    (Sq::Prop(0), "Pârâul Rece", 60),
    (Sq::Tax(100), "Taxa pe venit", 0),
    (Sq::Station, "Gara Mică", 200),
    (Sq::Prop(1), "Strada Teilor", 100),
    (Sq::Prop(1), "Strada Morii", 100),
    (Sq::JailVisit, "Închisoarea", 0),
    (Sq::Prop(1), "Strada Ploii", 120),
    (Sq::Prop(2), "Piața Veche", 140),
    (Sq::Luck, "Noroc", 0),
    (Sq::Prop(2), "Piața Nouă", 140),
    (Sq::Station, "Gara Veche", 200),
    (Sq::Prop(2), "Piața Mare", 160),
    (Sq::Prop(3), "Pasajul Lunii", 180),
    (Sq::Parking, "Parcarea", 0),
    (Sq::Prop(3), "Pasajul Soarelui", 180),
    (Sq::Luck, "Noroc", 0),
    (Sq::Prop(3), "Pasajul Stelei", 200),
    (Sq::Station, "Gara Nouă", 200),
    (Sq::Prop(4), "Aleea Brazilor", 220),
    (Sq::Prop(4), "Aleea Ulmilor", 220),
    (Sq::Prop(4), "Aleea Plopilor", 240),
    (Sq::GoJail, "Mergi la închisoare", 0),
    (Sq::Prop(5), "Bulevardul Cerului", 260),
    (Sq::Prop(5), "Bulevardul Mării", 260),
    (Sq::Tax(150), "Taxa de lux", 0),
    (Sq::Prop(5), "Bulevardul Vântului", 280),
    (Sq::Station, "Gara Mare", 200),
    (Sq::Prop(6), "Turnul Alb", 350),
    (Sq::Prop(6), "Turnul Roșu", 400),
];

#[cfg(test)]
const GROUP_SIZE: [usize; 7] = [2, 3, 3, 3, 3, 3, 2];
const HOUSE_COST: [i32; 7] = [50, 50, 100, 100, 150, 150, 200];
const RENT_MULT: [i32; 4] = [1, 4, 10, 20];
const MAX_HOUSES: u8 = 3;

fn group_of(sq: usize) -> Option<usize> {
    match BOARD[sq].0 {
        Sq::Prop(g) => Some(g as usize),
        _ => None,
    }
}

fn group_squares(g: usize) -> Vec<usize> {
    (0..N).filter(|&i| group_of(i) == Some(g)).collect()
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Roll,
    PayJail,
    Build(usize),
    Buy,
    Decline,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    /// Începutul aruncării: poți construi, plăti închisoarea sau arunca.
    Pre,
    /// Ai aterizat pe un câmp liber și îl poți cumpăra.
    Buy(usize),
}

#[derive(Clone)]
pub struct Mosii {
    pos: [usize; 2],
    cash: [i32; 2],
    jail: [Option<u8>; 2], // Some(încercări ratate)
    owner: [Option<Player>; N],
    houses: [u8; N],
    side: Player,
    phase: Phase,
    doubles: u8,
    last_roll: (u8, u8),
    log: String,
    plies: u32,
    winner: Option<Outcome>,
    rng: Rng,
}

impl Default for Mosii {
    fn default() -> Self {
        Self::new()
    }
}

impl Mosii {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Mosii {
            pos: [0, 0],
            cash: [START_CASH; 2],
            jail: [None, None],
            owner: [None; N],
            houses: [0; N],
            side: Player::One,
            phase: Phase::Pre,
            doubles: 0,
            last_roll: (0, 0),
            log: String::new(),
            plies: 0,
            winner: None,
            rng: Rng::new(seed),
        }
    }

    pub fn cash(&self, p: Player) -> i32 {
        self.cash[p.idx()]
    }

    pub fn position(&self, p: Player) -> usize {
        self.pos[p.idx()]
    }

    pub fn owner_of(&self, sq: usize) -> Option<Player> {
        self.owner[sq]
    }

    pub fn houses_on(&self, sq: usize) -> u8 {
        self.houses[sq]
    }

    fn owns_group(&self, p: Player, g: usize) -> bool {
        group_squares(g).iter().all(|&s| self.owner[s] == Some(p))
    }

    fn stations_of(&self, p: Player) -> i32 {
        (0..N)
            .filter(|&i| BOARD[i].0 == Sq::Station && self.owner[i] == Some(p))
            .count() as i32
    }

    pub fn rent(&self, sq: usize) -> i32 {
        let Some(o) = self.owner[sq] else { return 0 };
        match BOARD[sq].0 {
            Sq::Station => 10 * (1 << self.stations_of(o)),
            Sq::Prop(g) => {
                let base = BOARD[sq].2 / 10;
                let h = self.houses[sq] as usize;
                if h > 0 {
                    base * RENT_MULT[h]
                } else if self.owns_group(o, g as usize) {
                    base * 2
                } else {
                    base
                }
            }
            _ => 0,
        }
    }

    pub fn net_worth(&self, p: Player) -> i32 {
        let mut w = self.cash[p.idx()];
        for (i, b) in BOARD.iter().enumerate() {
            if self.owner[i] == Some(p) {
                w += b.2;
                if let Some(g) = group_of(i) {
                    w += self.houses[i] as i32 * HOUSE_COST[g];
                }
            }
        }
        w
    }

    /// Plătește `amount` (către `to` sau banca). Vinde case dacă trebuie;
    /// declară faliment dacă tot nu ajunge.
    fn pay(&mut self, p: Player, amount: i32, to: Option<Player>) {
        if amount <= 0 {
            return;
        }
        while self.cash[p.idx()] < amount {
            // vinde o casă (de pe cel mai dezvoltat câmp)
            let mut best: Option<usize> = None;
            for i in 0..N {
                if self.owner[i] == Some(p) && self.houses[i] > 0 {
                    match best {
                        Some(b) if self.houses[b] >= self.houses[i] => {}
                        _ => best = Some(i),
                    }
                }
            }
            match best {
                Some(i) => {
                    self.houses[i] -= 1;
                    self.cash[p.idx()] += HOUSE_COST[group_of(i).unwrap()] / 2;
                }
                None => break,
            }
        }
        if self.cash[p.idx()] < amount {
            // faliment: restul banilor merg la creditor
            let rest = self.cash[p.idx()].max(0);
            self.cash[p.idx()] = 0;
            if let Some(t) = to {
                self.cash[t.idx()] += rest;
            }
            self.winner = Some(Outcome::Win(p.other()));
            return;
        }
        self.cash[p.idx()] -= amount;
        if let Some(t) = to {
            self.cash[t.idx()] += amount;
        }
    }

    fn go_to_jail(&mut self, p: Player) {
        self.pos[p.idx()] = JAIL_POS;
        self.jail[p.idx()] = Some(0);
        self.doubles = 0;
    }

    /// Mută `p` cu `steps` pași (cu salariu la trecerea prin Start) și rezolvă câmpul.
    fn advance(&mut self, p: Player, steps: usize) {
        let old = self.pos[p.idx()];
        let new = (old + steps) % N;
        self.pos[p.idx()] = new;
        if new < old || (old + steps) >= N {
            self.cash[p.idx()] += SALARY;
        }
        self.land(p, 0);
    }

    fn land(&mut self, p: Player, depth: u8) {
        let sq = self.pos[p.idx()];
        match BOARD[sq].0 {
            Sq::Prop(_) | Sq::Station => match self.owner[sq] {
                None => {
                    if self.cash[p.idx()] >= BOARD[sq].2 {
                        self.phase = Phase::Buy(sq);
                    }
                }
                Some(o) if o != p => {
                    let r = self.rent(sq);
                    self.pay(p, r, Some(o));
                }
                _ => {}
            },
            Sq::Tax(t) => self.pay(p, t, None),
            Sq::GoJail => self.go_to_jail(p),
            Sq::Luck if depth < 2 => self.luck(p, depth),
            _ => {}
        }
    }

    fn luck(&mut self, p: Player, depth: u8) {
        match self.rng.below(8) {
            0 => self.cash[p.idx()] += 100,
            1 => self.pay(p, 60, None),
            2 => {
                self.pos[p.idx()] = 0;
                self.cash[p.idx()] += SALARY;
            }
            3 => self.go_to_jail(p),
            4 => {
                let np = (self.pos[p.idx()] + N - 3) % N;
                self.pos[p.idx()] = np;
                self.land(p, depth + 1);
            }
            5 => {
                let q = p.other();
                self.pay(q, 40, Some(p));
            }
            6 => self.pay(p, 40, Some(p.other())),
            _ => {
                let h: i32 = (0..N)
                    .filter(|&i| self.owner[i] == Some(p))
                    .map(|i| self.houses[i] as i32)
                    .sum();
                self.pay(p, 25 * h, None);
            }
        }
    }

    fn end_turn(&mut self) {
        self.phase = Phase::Pre;
        self.doubles = 0;
        self.side = self.side.other();
    }

    /// După ce un câmp a fost rezolvat: dublă = joci iar, altfel trece rândul.
    fn after_landing(&mut self, was_double: bool) {
        if self.winner.is_some() {
            return;
        }
        if let Phase::Buy(_) = self.phase {
            return; // așteptăm cumpăr/las; abia apoi încheiem
        }
        let me = self.side;
        if was_double && self.jail[me.idx()].is_none() {
            self.phase = Phase::Pre;
        } else {
            self.end_turn();
        }
    }

    fn can_build(&self, sq: usize) -> bool {
        let p = self.side;
        let Some(g) = group_of(sq) else { return false };
        if self.owner[sq] != Some(p) || !self.owns_group(p, g) {
            return false;
        }
        if self.houses[sq] >= MAX_HOUSES || self.cash[p.idx()] < HOUSE_COST[g] {
            return false;
        }
        let min = group_squares(g)
            .iter()
            .map(|&s| self.houses[s])
            .min()
            .unwrap_or(0);
        self.houses[sq] <= min
    }

    fn check_cap(&mut self) {
        if self.winner.is_none() && self.plies >= PLY_CAP {
            let a = self.net_worth(Player::One);
            let b = self.net_worth(Player::Two);
            self.winner = Some(match a.cmp(&b) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                std::cmp::Ordering::Equal => Outcome::Draw,
            });
        }
    }
}

impl Game for Mosii {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.winner.is_some() {
            return Vec::new();
        }
        match self.phase {
            Phase::Buy(_) => vec![Move::Buy, Move::Decline],
            Phase::Pre => {
                let mut v = vec![Move::Roll];
                if self.jail[self.side.idx()].is_some() && self.cash[self.side.idx()] >= JAIL_FEE {
                    v.push(Move::PayJail);
                }
                for sq in 0..N {
                    if self.can_build(sq) {
                        v.push(Move::Build(sq));
                    }
                }
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        self.plies += 1;
        let me = self.side;
        self.log.clear();
        match *m {
            Move::Build(sq) => {
                let g = group_of(sq).unwrap();
                self.cash[me.idx()] -= HOUSE_COST[g];
                self.houses[sq] += 1;
                self.log = format!("J{} construiește pe {}", me.idx() + 1, BOARD[sq].1);
            }
            Move::PayJail => {
                self.cash[me.idx()] -= JAIL_FEE;
                self.jail[me.idx()] = None;
                self.log = format!("J{} iese din închisoare (50)", me.idx() + 1);
            }
            Move::Buy => {
                if let Phase::Buy(sq) = self.phase {
                    self.cash[me.idx()] -= BOARD[sq].2;
                    self.owner[sq] = Some(me);
                    self.log = format!("J{} cumpără {}", me.idx() + 1, BOARD[sq].1);
                }
                self.phase = Phase::Pre;
                let dbl = self.last_roll.0 == self.last_roll.1 && self.doubles > 0;
                self.after_landing(dbl);
            }
            Move::Decline => {
                self.phase = Phase::Pre;
                let dbl = self.last_roll.0 == self.last_roll.1 && self.doubles > 0;
                self.after_landing(dbl);
            }
            Move::Roll => {
                let d1 = self.rng.d6();
                let d2 = self.rng.d6();
                self.last_roll = (d1, d2);
                let dbl = d1 == d2;
                self.log = format!("J{} aruncă {}+{}", me.idx() + 1, d1, d2);
                if let Some(tries) = self.jail[me.idx()] {
                    if dbl {
                        self.jail[me.idx()] = None;
                        self.doubles = 0;
                        self.advance(me, (d1 + d2) as usize);
                        // după ieșirea cu dublă nu se mai joacă o dată
                        self.after_landing(false);
                    } else if tries + 1 >= 3 {
                        self.jail[me.idx()] = None;
                        self.pay(me, JAIL_FEE, None);
                        if self.winner.is_none() {
                            self.advance(me, (d1 + d2) as usize);
                        }
                        self.after_landing(false);
                    } else {
                        self.jail[me.idx()] = Some(tries + 1);
                        self.end_turn();
                    }
                } else {
                    if dbl {
                        self.doubles += 1;
                    } else {
                        self.doubles = 0;
                    }
                    if self.doubles >= 3 {
                        self.go_to_jail(me);
                        self.end_turn();
                    } else {
                        self.advance(me, (d1 + d2) as usize);
                        self.after_landing(dbl);
                    }
                }
            }
        }
        self.check_cap();
    }

    fn outcome(&self) -> Option<Outcome> {
        self.winner
    }

    fn evaluate(&self, p: Player) -> i32 {
        let side = |x: Player| -> i32 {
            let mut v = self.cash[x.idx()];
            for (i, b) in BOARD.iter().enumerate() {
                if self.owner[i] != Some(x) {
                    continue;
                }
                let price = b.2;
                v += price + price * 3 / 10;
                if let Some(g) = group_of(i) {
                    v += self.houses[i] as i32 * HOUSE_COST[g] * 14 / 10;
                    if self.owns_group(x, g) {
                        v += price * 6 / 10;
                    }
                }
            }
            if self.cash[x.idx()] < 100 {
                v -= 100 - self.cash[x.idx()];
            }
            v
        };
        side(p) - side(p.other())
    }

    fn render(&self) -> String {
        let mut s = String::from("Moșii — tabla (câmp: nume, proprietar, case)\n");
        for row in 0..8 {
            let mut line = String::new();
            for col in 0..4 {
                let i = col * 8 + row;
                let mut cell = format!("{:>2} {}", i, BOARD[i].1);
                if let Some(o) = self.owner[i] {
                    cell.push_str(&format!(" [J{}", o.idx() + 1));
                    if self.houses[i] > 0 {
                        cell.push_str(&format!(" {}c", self.houses[i]));
                    }
                    cell.push(']');
                }
                let mut mark = String::new();
                if self.pos[0] == i {
                    mark.push_str(" ①");
                }
                if self.pos[1] == i {
                    mark.push_str(" ②");
                }
                cell.push_str(&mark);
                let pad = 34usize.saturating_sub(cell.chars().count());
                line.push_str(&cell);
                line.push_str(&" ".repeat(pad));
            }
            s.push_str(line.trim_end());
            s.push('\n');
        }
        s.push_str(&format!(
            "Bani: J1 {} lei (avere {}), J2 {} lei (avere {})\n",
            self.cash[0],
            self.net_worth(Player::One),
            self.cash[1],
            self.net_worth(Player::Two)
        ));
        for p in [Player::One, Player::Two] {
            if self.jail[p.idx()].is_some() {
                s.push_str(&format!("J{} e în închisoare\n", p.idx() + 1));
            }
        }
        if !self.log.is_empty() {
            s.push_str(&format!("Ultima acțiune: {}\n", self.log));
        }
        if self.winner.is_some() {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        match self.phase {
            Phase::Buy(sq) => s.push_str(&format!(
                "J{} poate cumpăra {} cu {} lei (cumpar / las)\n",
                self.side.idx() + 1,
                BOARD[sq].1,
                BOARD[sq].2
            )),
            Phase::Pre => s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1)),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Roll => "arunc".into(),
            Move::PayJail => "platesc".into(),
            Move::Build(sq) => format!("construiesc {sq}"),
            Move::Buy => "cumpar".into(),
            Move::Decline => "las".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rolled_to(g: &mut Mosii, target: usize) {
        // așază jucătorul la mutare chiar înaintea câmpului `target` și aterizează
        let p = g.side;
        g.pos[p.idx()] = target;
        g.land(p, 0);
        g.after_landing(false);
    }

    #[test]
    fn board_is_consistent() {
        assert_eq!(BOARD.len(), 32);
        for (g, &n) in GROUP_SIZE.iter().enumerate() {
            assert_eq!(group_squares(g).len(), n, "grup {g}");
        }
        assert_eq!(
            BOARD.iter().filter(|b| b.0 == Sq::Station).count(),
            4,
            "gări"
        );
    }

    #[test]
    fn buy_and_pay_rent() {
        let mut g = Mosii::with_seed(1);
        rolled_to(&mut g, 1);
        // era J1: așteaptă cumpărarea
        assert_eq!(g.legal_moves(), vec![Move::Buy, Move::Decline]);
        g.apply(&Move::Buy);
        assert_eq!(g.owner_of(1), Some(Player::One));
        assert_eq!(g.cash(Player::One), 940);
        assert_eq!(g.to_move(), Player::Two);
        let before = g.cash(Player::Two);
        g.pos[1] = 1;
        g.land(Player::Two, 0);
        assert_eq!(g.cash(Player::Two), before - 6);
        assert_eq!(g.cash(Player::One), 946);
    }

    #[test]
    fn group_doubles_rent_and_houses() {
        let mut g = Mosii::with_seed(2);
        g.owner[1] = Some(Player::One);
        assert_eq!(g.rent(1), 6);
        g.owner[3] = Some(Player::One);
        assert_eq!(g.rent(1), 12);
        assert!(g.legal_moves().contains(&Move::Build(1)));
        g.apply(&Move::Build(1));
        assert_eq!(g.houses_on(1), 1);
        assert_eq!(g.rent(1), 24);
        // construcție uniformă: nu pe același câmp a doua oară
        assert!(!g.legal_moves().contains(&Move::Build(1)));
        assert!(g.legal_moves().contains(&Move::Build(3)));
    }

    #[test]
    fn station_rent_scales() {
        let mut g = Mosii::with_seed(3);
        g.owner[5] = Some(Player::One);
        assert_eq!(g.rent(5), 20);
        g.owner[13] = Some(Player::One);
        assert_eq!(g.rent(5), 40);
        g.owner[20] = Some(Player::One);
        g.owner[29] = Some(Player::One);
        assert_eq!(g.rent(5), 160);
    }

    #[test]
    fn go_to_jail_and_pay() {
        let mut g = Mosii::with_seed(4);
        rolled_to(&mut g, 24);
        assert_eq!(g.position(Player::One), JAIL_POS);
        assert_eq!(g.to_move(), Player::Two);
        g.apply(&Move::Roll);
        g.jail[0] = Some(0);
        g.side = Player::One;
        g.phase = Phase::Pre;
        assert!(g.legal_moves().contains(&Move::PayJail));
        let c = g.cash(Player::One);
        g.apply(&Move::PayJail);
        assert_eq!(g.cash(Player::One), c - 50);
        assert!(g.jail[0].is_none());
    }

    #[test]
    fn passing_start_pays_salary() {
        let mut g = Mosii::with_seed(5);
        g.pos[0] = 30;
        g.advance(Player::One, 4); // ajunge pe 2 (Noroc) trecând prin Start
        assert!(g.cash(Player::One) >= START_CASH + SALARY - 100);
    }

    #[test]
    fn bankruptcy_ends_game() {
        let mut g = Mosii::with_seed(6);
        g.cash[0] = 5;
        g.pay(Player::One, 100, Some(Player::Two));
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
    }

    #[test]
    fn houses_sold_to_pay() {
        let mut g = Mosii::with_seed(7);
        g.owner[1] = Some(Player::One);
        g.owner[3] = Some(Player::One);
        g.houses[1] = 1;
        g.houses[3] = 1;
        g.cash[0] = 0;
        g.pay(Player::One, 20, None);
        assert!(g.outcome().is_none());
        assert_eq!(g.houses[1] + g.houses[3], 1);
        assert_eq!(g.cash(Player::One), 5);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=20u64 {
            let mut g = Mosii::with_seed(seed);
            let mut rng = Rng::new(seed ^ 0x55);
            let mut plies = 0;
            while Game::outcome(&g).is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                plies += 1;
                assert!(plies < 4000);
            }
        }
    }

    #[test]
    fn ai_not_worse_than_random() {
        let (mut ai, mut rnd) = (0, 0);
        for seed in 1..=16u64 {
            let ai_side = if seed % 2 == 0 {
                Player::One
            } else {
                Player::Two
            };
            let mut g = Mosii::with_seed(seed);
            let mut rng = Rng::new(seed * 13);
            while Game::outcome(&g).is_none() {
                let m = if g.to_move() == ai_side {
                    polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap()
                } else {
                    let l = g.legal_moves();
                    l[rng.below(l.len())]
                };
                g.apply(&m);
            }
            match Game::outcome(&g) {
                Some(Outcome::Win(p)) if p == ai_side => ai += 1,
                Some(Outcome::Win(_)) => rnd += 1,
                _ => {}
            }
        }
        assert!(ai >= rnd, "AI {ai} vs random {rnd}");
    }
}
