//! Reverdire — motor de resurse și proiecte de cărți, variantă originală pentru doi.
//!
//! Tema noastră: două companii redau viața unui bazin sărat secat. Trei parametri globali —
//! Umiditate (0–10), Aer (0–8) și Lacuri (0–7, câte o plăcuță-lac pe harta 5×5) — se ridică
//! prin proiecte; fiecare pas ridică Ratingul de Reverdire (RR) al celui care l-a făcut.
//! Șase resurse: Lei (L), Cărămizi (C), Cristale (K), Semințe (S), Curent (E), Căldură (H),
//! fiecare cu producție proprie.
//!
//! O generație: (1) Cercetare — primești 4 cărți și cumperi oricâte cu 3 Lei fiecare
//! (`compra 2`, `gata`); (2) acțiuni pe rând, una pe tură, până pasează ambii; (3) producție:
//! Lei += RR + producția de Lei, Curentul vechi se face Căldură, apoi se adaugă producțiile.
//!
//! Acțiuni: `joc 3` (proiect din mână: plătești costul în Lei; Cărămizile valorează 2 Lei la
//! proiectele cu etichetă B, Cristalele 3 Lei la cele cu etichetă S; unele proiecte cer un
//! parametru minim), `vinde 3` (+1 Leu), proiecte standard `pompa` (10 L → +1 producție de
//! Curent), `irigare` (16 L → un lac), `plantare` (20 L → un arbore), `asezare` (22 L → un
//! oraș, +1 producție de Lei), conversii `conv sem` (6 Semințe → un arbore) și `conv caldura`
//! (6 Căldură → Umiditate +1), `pas`.
//!
//! Plăcuțe (alegi locul cu `pune c3`): lac pe una din cele 7 celule-lac; arbore pe pământ, lipit
//! de o plăcuță a ta dacă se poate (Aer +1); oraș pe pământ, nelipit de alt oraș. O plăcuță de
//! pământ lipită de lacuri dă +2 Lei pentru fiecare lac vecin.
//!
//! Sfârșit: după generația în care toți cei trei parametri sunt la maxim (sau după a 16-a). Scor:
//! RR + punctele cărților + 1 pe arbore + 1 pe arborele vecin fiecărui oraș al tău. Egalitate:
//! mai mulți Lei. Cărțile sunt generate de noi. Informație ascunsă: pachetul, mâna și oferta
//! adversarului.
#![allow(clippy::needless_range_loop, clippy::doc_lazy_continuation)]

use crate::cards::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const RES: [char; 6] = ['L', 'C', 'K', 'S', 'E', 'H'];
const UMID_MAX: u8 = 10;
const AER_MAX: u8 = 8;
const LAKES_MAX: u8 = 7;
const MAX_GEN: u8 = 16;
const LAKE_CELLS: [usize; 7] = [1, 8, 10, 12, 19, 21, 23];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Eff {
    Prod(u8, i8),
    Res(u8, u8),
    Umid(u8),
    Aer(u8),
    Lake,
    Tree,
    City,
    Draw(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Card {
    pub name: &'static str,
    /// 0 fără etichetă, 1 B (construcții), 2 S (știință)
    pub tag: u8,
    pub cost: u8,
    /// cerință: (parametru 0 umiditate / 1 aer, minim)
    pub req: Option<(u8, u8)>,
    pub effs: [Option<Eff>; 3],
    pub vp: u8,
}

fn c(name: &'static str, tag: u8, cost: u8, req: Option<(u8, u8)>, effs: &[Eff], vp: u8) -> Card {
    let mut e = [None; 3];
    for (i, x) in effs.iter().enumerate() {
        e[i] = Some(*x);
    }
    Card {
        name,
        tag,
        cost,
        req,
        effs: e,
        vp,
    }
}

fn base_cards() -> Vec<Card> {
    use Eff::*;
    vec![
        c("Cariera de lut", 1, 6, None, &[Prod(1, 2)], 0),
        c("Mină de cristale", 1, 9, None, &[Prod(2, 1)], 0),
        c("Seră solară", 0, 9, None, &[Prod(3, 2)], 0),
        c("Moară de vânt", 1, 7, None, &[Prod(4, 2)], 0),
        c("Turbină eoliană", 1, 11, None, &[Prod(4, 3)], 0),
        c("Casa de bani", 2, 12, None, &[Prod(0, 3)], 0),
        c(
            "Fabrica de ceramică",
            1,
            10,
            None,
            &[Prod(0, 2), Prod(1, 1)],
            0,
        ),
        c("Dig vechi", 1, 14, None, &[Lake, Prod(0, 1)], 0),
        c("Cisternă", 1, 13, None, &[Umid(2)], 0),
        c("Nor sămânță", 2, 15, None, &[Umid(2), Prod(4, 1)], 0),
        c("Pădure tânără", 0, 12, None, &[Tree], 0),
        c("Livadă", 0, 14, None, &[Tree, Prod(3, 1)], 0),
        c("Pădure de protecție", 0, 18, None, &[Tree, Aer(1)], 0),
        c("Oraș mic", 1, 16, None, &[City, Prod(0, 1)], 0),
        c("Capitală", 1, 25, None, &[City, Prod(0, 2)], 2),
        c("Bibliotecă", 2, 6, None, &[Draw(2)], 0),
        c("Institut de cercetare", 2, 10, None, &[Draw(3)], 1),
        c("Laborator de sol", 2, 7, None, &[Res(3, 4)], 0),
        c("Cuptor solar", 1, 8, None, &[Prod(5, 3)], 0),
        c("Reactor termic", 1, 13, None, &[Prod(5, 4)], 0),
        c("Convertor de apă", 2, 17, None, &[Umid(1), Aer(1)], 0),
        c("Parc natural", 0, 16, None, &[Tree], 2),
        c("Grădini suspendate", 0, 21, None, &[Tree, Tree], 0),
        c("Brigadă de plantare", 0, 9, Some((1, 2)), &[Res(3, 6)], 0),
        c("Așezare minieră", 1, 10, None, &[Prod(2, 1), Prod(1, 1)], 0),
        c("Piața de apă", 2, 11, Some((0, 3)), &[Prod(0, 2)], 0),
        c("Stație meteo", 2, 5, None, &[Prod(0, 1)], 0),
        c("Bazin colector", 1, 12, Some((0, 2)), &[Lake], 0),
        c("Lac artificial", 1, 19, None, &[Lake, Lake], 0),
        c("Dune verzi", 0, 13, Some((0, 2)), &[Aer(2)], 0),
        c(
            "Centrala de sare",
            1,
            15,
            None,
            &[Prod(4, 3), Prod(0, 1)],
            0,
        ),
        c("Fabrica de cărămizi", 1, 8, None, &[Prod(1, 3)], 0),
        c("Contract", 2, 4, None, &[Res(0, 7)], 0),
        c("Fond de investiții", 2, 14, None, &[Prod(0, 2), Draw(1)], 0),
        c(
            "Acord internațional",
            2,
            20,
            None,
            &[Umid(1), Aer(1), Draw(1)],
            1,
        ),
        c("Marele canal", 1, 28, None, &[Lake, Lake, Umid(2)], 3),
        c("Observator", 2, 9, None, &[Res(0, 2)], 2),
        c("Monument", 1, 12, None, &[], 3),
        c("Mare dig", 1, 24, None, &[Lake, Umid(1)], 2),
    ]
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Buy(u8),
    Done,
    Play(u8),
    Sell(u8),
    Pump,
    Irrigate,
    Plant,
    Settle,
    ConvSeeds,
    ConvHeat,
    Pass,
    Place(u8),
    Deal,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Deal,
    Research(u8),
    Actions,
    Over,
}

#[derive(Clone)]
pub struct Reverdire {
    /// Whose private info `render` shows (`None` = the mover's, hot-seat).
    viewer: Option<Player>,
    deck: Vec<Card>,
    discard: Vec<Card>,
    table: [Vec<Card>; 2],
    hand: [Vec<Card>; 2],
    offer: [Vec<Card>; 2],
    res: [[i32; 6]; 2],
    prod: [[i32; 6]; 2],
    tr: [i32; 2],
    vp: [i32; 2],
    umid: u8,
    aer: u8,
    /// 0 gol, 1 lac, 2/3 arbore J1/J2, 4/5 oraș J1/J2
    board: [u8; 25],
    gen: u8,
    first: Player,
    side: Player,
    phase: Phase,
    passed: [bool; 2],
    pending: Vec<u8>,
    rng: Rng,
}

impl Default for Reverdire {
    fn default() -> Self {
        Self::new()
    }
}

fn neighbors(i: usize) -> Vec<usize> {
    let (r, c) = (i / 5, i % 5);
    let mut v = Vec::new();
    if r > 0 {
        v.push(i - 5);
    }
    if r < 4 {
        v.push(i + 5);
    }
    if c > 0 {
        v.push(i - 1);
    }
    if c < 4 {
        v.push(i + 1);
    }
    v
}

fn is_lake_cell(i: usize) -> bool {
    LAKE_CELLS.contains(&i)
}

fn cell_name(i: usize) -> String {
    format!("{}{}", (b'a' + (i % 5) as u8) as char, i / 5 + 1)
}

impl Reverdire {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck = base_cards();
        deck.extend(base_cards());
        shuffle(&mut deck, &mut rng);
        Reverdire {
            viewer: None,
            deck,
            discard: Vec::new(),
            table: [Vec::new(), Vec::new()],
            hand: [Vec::new(), Vec::new()],
            offer: [Vec::new(), Vec::new()],
            res: [[20, 0, 0, 0, 0, 0]; 2],
            prod: [[1; 6]; 2],
            tr: [10; 2],
            vp: [0; 2],
            umid: 0,
            aer: 0,
            board: [0; 25],
            gen: 1,
            first: Player::One,
            side: Player::One,
            phase: Phase::Deal,
            passed: [false; 2],
            pending: Vec::new(),
            rng,
        }
    }

    fn draw_card(&mut self) -> Option<Card> {
        if self.deck.is_empty() {
            let mut d = std::mem::take(&mut self.discard);
            shuffle(&mut d, &mut self.rng);
            self.deck = d;
        }
        self.deck.pop()
    }

    /// Cărți în joc: pachet + aruncate + mână + ofertă + jucate pe masă.
    pub fn card_total(&self) -> usize {
        self.deck.len()
            + self.discard.len()
            + (0..2)
                .map(|p| self.hand[p].len() + self.offer[p].len() + self.table[p].len())
                .sum::<usize>()
    }

    fn requirement_ok(&self, card: &Card) -> bool {
        match card.req {
            None => true,
            Some((0, n)) => self.umid >= n,
            Some((_, n)) => self.aer >= n,
        }
    }

    /// Ce plătești în (Lei, resursa de substituție, câte unități) pentru o carte.
    fn payment(&self, p: usize, card: &Card) -> Option<(i32, usize, i32)> {
        let cost = i32::from(card.cost);
        let (sub, val) = match card.tag {
            1 => (1usize, 2),
            2 => (2usize, 3),
            _ => (0usize, 0),
        };
        let mut used = 0;
        if val > 0 {
            used = self.res[p][sub].min(cost / val);
        }
        let mut lei = cost - used * val;
        if lei > self.res[p][0] && val > 0 && self.res[p][sub] > used {
            used += 1;
            lei = (cost - used * val).max(0);
        }
        if lei <= self.res[p][0] {
            Some((lei, sub, used))
        } else {
            None
        }
    }

    fn playable(&self, p: usize, card: &Card) -> bool {
        self.requirement_ok(card) && self.payment(p, card).is_some()
    }

    fn raise_umid(&mut self, p: usize, n: u8) {
        for _ in 0..n {
            if self.umid < UMID_MAX {
                self.umid += 1;
                self.tr[p] += 1;
            }
        }
    }

    fn raise_aer(&mut self, p: usize, n: u8) {
        for _ in 0..n {
            if self.aer < AER_MAX {
                self.aer += 1;
                self.tr[p] += 1;
            }
        }
    }

    fn lakes_placed(&self) -> u8 {
        self.board.iter().filter(|x| **x == 1).count() as u8
    }

    fn maxed(&self) -> bool {
        self.umid >= UMID_MAX && self.aer >= AER_MAX && self.lakes_placed() >= LAKES_MAX
    }

    /// Celulele permise pentru o plăcuță de felul `kind` (0 lac, 1 arbore, 2 oraș).
    fn cells_for(&self, p: usize, kind: u8) -> Vec<usize> {
        let mut v = Vec::new();
        match kind {
            0 => {
                for &i in &LAKE_CELLS {
                    if self.board[i] == 0 {
                        v.push(i);
                    }
                }
            }
            1 => {
                let mine = [2 + p as u8, 4 + p as u8];
                let all: Vec<usize> = (0..25)
                    .filter(|&i| self.board[i] == 0 && !is_lake_cell(i))
                    .collect();
                let near: Vec<usize> = all
                    .iter()
                    .copied()
                    .filter(|&i| neighbors(i).iter().any(|&j| mine.contains(&self.board[j])))
                    .collect();
                v = if near.is_empty() { all } else { near };
            }
            _ => {
                for i in 0..25 {
                    if self.board[i] == 0
                        && !is_lake_cell(i)
                        && !neighbors(i).iter().any(|&j| self.board[j] >= 4)
                    {
                        v.push(i);
                    }
                }
            }
        }
        v
    }

    fn queue(&mut self, p: usize, kind: u8) {
        if !self.cells_for(p, kind).is_empty() {
            self.pending.push(kind);
        }
    }

    fn apply_effects(&mut self, p: usize, card: &Card) {
        for e in card.effs.iter().flatten() {
            match *e {
                Eff::Prod(r, d) => self.prod[p][r as usize] += i32::from(d),
                Eff::Res(r, n) => self.res[p][r as usize] += i32::from(n),
                Eff::Umid(n) => self.raise_umid(p, n),
                Eff::Aer(n) => self.raise_aer(p, n),
                Eff::Lake => self.queue(p, 0),
                Eff::Tree => self.queue(p, 1),
                Eff::City => self.queue(p, 2),
                Eff::Draw(n) => {
                    for _ in 0..n {
                        if let Some(c) = self.draw_card() {
                            self.hand[p].push(c);
                            self.hand[p].sort_by_key(|c| (c.cost, c.name));
                        }
                    }
                }
            }
        }
        self.vp[p] += i32::from(card.vp);
    }

    fn place(&mut self, p: usize, cell: usize) {
        let kind = self.pending.remove(0);
        match kind {
            0 => {
                self.board[cell] = 1;
                self.tr[p] += 1;
            }
            1 => {
                self.board[cell] = 2 + p as u8;
                self.raise_aer(p, 1);
            }
            _ => {
                self.board[cell] = 4 + p as u8;
            }
        }
        if kind > 0 {
            let lakes = neighbors(cell)
                .iter()
                .filter(|&&j| self.board[j] == 1)
                .count() as i32;
            self.res[p][0] += 2 * lakes;
        }
    }

    pub fn score(&self, p: usize) -> i32 {
        let mut s = self.tr[p] + self.vp[p];
        for i in 0..25 {
            if self.board[i] == 2 + p as u8 {
                s += 1;
            }
            if self.board[i] == 4 + p as u8 {
                s += neighbors(i)
                    .iter()
                    .filter(|&&j| self.board[j] == 2 || self.board[j] == 3)
                    .count() as i32;
            }
        }
        s
    }

    fn production_phase(&mut self) {
        for p in 0..2 {
            self.res[p][5] += self.res[p][4];
            self.res[p][4] = 0;
            self.res[p][0] += self.tr[p] + self.prod[p][0];
            for r in 1..6 {
                self.res[p][r] += self.prod[p][r];
            }
        }
        if self.maxed() || self.gen >= MAX_GEN {
            self.phase = Phase::Over;
            return;
        }
        self.gen += 1;
        self.first = self.first.other();
        self.phase = Phase::Deal;
        self.side = self.first;
    }

    fn after_action(&mut self) {
        self.settle_pending();
        if !self.pending.is_empty() {
            return;
        }
        if self.passed[0] && self.passed[1] {
            self.production_phase();
            return;
        }
        let o = self.side.other();
        if !self.passed[o.idx()] {
            self.side = o;
        }
    }

    fn settle_pending(&mut self) {
        let p = self.side.idx();
        while let Some(&k) = self.pending.first() {
            if self.cells_for(p, k).is_empty() {
                self.pending.remove(0);
            } else {
                break;
            }
        }
    }

    fn heuristic(&self, p: usize) -> f64 {
        // unitatea: zecimi de punct de scor
        let gens = f64::from((9 - i32::from(self.gen)).max(1) as u8);
        let mut s = 10.0 * f64::from(self.score(p));
        s += self.tr[p] as f64 * gens;
        let w = [1.2, 2.0, 3.0, 3.3, 1.5, 1.8];
        for r in 0..6 {
            s += w[r] * self.res[p][r] as f64;
            s += w[r] * gens * (self.prod[p][r] - 1) as f64;
        }
        let h = self.hand[p].len();
        for i in 0..h {
            s += if i < 4 {
                4.0
            } else if i < 8 {
                2.5
            } else {
                1.0
            };
        }
        s
    }
}

impl Game for Reverdire {
    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }
    type Move = Move;

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Research(p) => {
                if p == 0 {
                    Player::One
                } else {
                    Player::Two
                }
            }
            Phase::Deal | Phase::Over => self.first,
            Phase::Actions => self.side,
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        match self.phase {
            Phase::Over => Vec::new(),
            Phase::Deal => vec![Move::Deal],
            Phase::Research(p) => {
                let p = p as usize;
                let mut v = vec![Move::Done];
                if self.res[p][0] >= 3 {
                    for i in 0..self.offer[p].len() {
                        v.push(Move::Buy(i as u8));
                    }
                }
                v
            }
            Phase::Actions => {
                let p = self.side.idx();
                if let Some(&k) = self.pending.first() {
                    return self
                        .cells_for(p, k)
                        .into_iter()
                        .map(|i| Move::Place(i as u8))
                        .collect();
                }
                let mut v = Vec::new();
                for (i, card) in self.hand[p].iter().enumerate() {
                    if self.playable(p, card) {
                        v.push(Move::Play(i as u8));
                    }
                    v.push(Move::Sell(i as u8));
                }
                let l = self.res[p][0];
                if l >= 10 {
                    v.push(Move::Pump);
                }
                if l >= 16 && self.lakes_placed() < LAKES_MAX {
                    v.push(Move::Irrigate);
                }
                if l >= 20 && !self.cells_for(p, 1).is_empty() {
                    v.push(Move::Plant);
                }
                if l >= 22 && !self.cells_for(p, 2).is_empty() {
                    v.push(Move::Settle);
                }
                if self.res[p][3] >= 6 && !self.cells_for(p, 1).is_empty() {
                    v.push(Move::ConvSeeds);
                }
                if self.res[p][5] >= 6 && self.umid < UMID_MAX {
                    v.push(Move::ConvHeat);
                }
                v.push(Move::Pass);
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        match (self.phase, *m) {
            (Phase::Deal, Move::Deal) => {
                for p in 0..2 {
                    self.offer[p].clear();
                    for _ in 0..4 {
                        if let Some(c) = self.draw_card() {
                            self.offer[p].push(c);
                        }
                    }
                }
                self.phase = Phase::Research(0);
            }
            (Phase::Research(p), Move::Buy(i)) => {
                let p = p as usize;
                let card = self.offer[p].remove(i as usize);
                self.res[p][0] -= 3;
                self.hand[p].push(card);
                self.hand[p].sort_by_key(|c| (c.cost, c.name));
            }
            (Phase::Research(p), Move::Done) => {
                let p = p as usize;
                let mut rest = std::mem::take(&mut self.offer[p]);
                self.discard.append(&mut rest);
                if p == 0 {
                    self.phase = Phase::Research(1);
                } else {
                    self.phase = Phase::Actions;
                    self.passed = [false; 2];
                    self.side = self.first;
                }
            }
            (Phase::Actions, mv) => {
                let p = self.side.idx();
                match mv {
                    Move::Place(i) => {
                        self.place(p, i as usize);
                        self.after_action();
                        return;
                    }
                    Move::Play(i) => {
                        let card = self.hand[p].remove(i as usize);
                        let (lei, sub, used) = self.payment(p, &card).expect("payable");
                        self.res[p][0] -= lei;
                        if used > 0 {
                            self.res[p][sub] -= used;
                        }
                        self.apply_effects(p, &card);
                        self.table[p].push(card);
                    }
                    Move::Sell(i) => {
                        let card = self.hand[p].remove(i as usize);
                        self.discard.push(card);
                        self.res[p][0] += 1;
                    }
                    Move::Pump => {
                        self.res[p][0] -= 10;
                        self.prod[p][4] += 1;
                    }
                    Move::Irrigate => {
                        self.res[p][0] -= 16;
                        self.queue(p, 0);
                    }
                    Move::Plant => {
                        self.res[p][0] -= 20;
                        self.queue(p, 1);
                    }
                    Move::Settle => {
                        self.res[p][0] -= 22;
                        self.prod[p][0] += 1;
                        self.queue(p, 2);
                    }
                    Move::ConvSeeds => {
                        self.res[p][3] -= 6;
                        self.queue(p, 1);
                    }
                    Move::ConvHeat => {
                        self.res[p][5] -= 6;
                        self.raise_umid(p, 1);
                    }
                    Move::Pass => {
                        self.passed[p] = true;
                    }
                    _ => {}
                }
                self.after_action();
            }
            _ => {}
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        let (a, b) = (self.score(0), self.score(1));
        Some(if a > b {
            Outcome::Win(Player::One)
        } else if b > a {
            Outcome::Win(Player::Two)
        } else if self.res[0][0] != self.res[1][0] {
            Outcome::Win(if self.res[0][0] > self.res[1][0] {
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
        AiStyle::AlphaBeta(2)
    }

    fn render(&self) -> String {
        let card_text = |c: &Card| {
            let mut e: Vec<String> = Vec::new();
            for x in c.effs.iter().flatten() {
                e.push(match *x {
                    Eff::Prod(r, d) => format!("prod {:+}{}", d, RES[r as usize]),
                    Eff::Res(r, n) => format!("+{}{}", n, RES[r as usize]),
                    Eff::Umid(n) => format!("Umiditate +{n}"),
                    Eff::Aer(n) => format!("Aer +{n}"),
                    Eff::Lake => "lac".into(),
                    Eff::Tree => "arbore".into(),
                    Eff::City => "oraș".into(),
                    Eff::Draw(n) => format!("{n} cărți"),
                });
            }
            let req = match c.req {
                Some((0, n)) => format!(" cere Umiditate≥{n}"),
                Some((_, n)) => format!(" cere Aer≥{n}"),
                None => String::new(),
            };
            format!(
                "{} [{}{} L; {}{}{}]",
                c.name,
                c.cost,
                ["", " B", " S"][c.tag as usize],
                e.join(", "),
                if c.vp > 0 {
                    format!("; {}PV", c.vp)
                } else {
                    String::new()
                },
                req
            )
        };
        let mut s = format!(
            "Reverdire — generația {} | Umiditate {}/{} Aer {}/{} Lacuri {}/{}\n",
            self.gen,
            self.umid,
            UMID_MAX,
            self.aer,
            AER_MAX,
            self.lakes_placed(),
            LAKES_MAX
        );
        s.push_str("     a  b  c  d  e\n");
        for r in 0..5 {
            s.push_str(&format!("  {} ", r + 1));
            for c in 0..5 {
                let i = r * 5 + c;
                let t = match self.board[i] {
                    0 => {
                        if is_lake_cell(i) {
                            "~~".to_string()
                        } else {
                            "..".to_string()
                        }
                    }
                    1 => "LL".to_string(),
                    2 => "T1".to_string(),
                    3 => "T2".to_string(),
                    4 => "O1".to_string(),
                    _ => "O2".to_string(),
                };
                s.push_str(&format!(" {t}"));
            }
            s.push('\n');
        }
        s.push_str("(~~ celulă-lac liberă, LL lac, T arbore, O oraș)\n");
        for p in 0..2 {
            let r: Vec<String> = (0..6)
                .map(|i| format!("{}{}(+{})", RES[i], self.res[p][i], self.prod[p][i]))
                .collect();
            s.push_str(&format!(
                "J{}: scor {} (RR {}, PV {}) | {} | mână {} | jucate {}\n",
                p + 1,
                self.score(p),
                self.tr[p],
                self.vp[p],
                r.join(" "),
                self.hand[p].len(),
                self.table[p].len()
            ));
        }
        let v = self.viewer.unwrap_or_else(|| self.to_move()).idx();
        s.push_str(&format!("Mâna ta (J{}):\n", v + 1));
        for (i, c) in self.hand[v].iter().enumerate() {
            s.push_str(&format!("  {}: {}\n", i + 1, card_text(c)));
        }
        if !self.offer[0].is_empty() {
            s.push_str("Oferta ta de cercetare (3 L fiecare):\n");
            for (i, c) in self.offer[0].iter().enumerate() {
                s.push_str(&format!("  {}: {}\n", i + 1, card_text(c)));
            }
        }
        match self.phase {
            Phase::Over => s.push_str("Joc încheiat.\n"),
            Phase::Deal => s.push_str("Începe o generație nouă (generatie).\n"),
            Phase::Research(p) => {
                s.push_str(&format!("Cercetare: mută J{} (compra N / gata).\n", p + 1))
            }
            Phase::Actions => {
                if let Some(k) = self.pending.first() {
                    s.push_str(&format!(
                        "J{} așază {} (pune c3).\n",
                        self.side.idx() + 1,
                        ["un lac", "un arbore", "un oraș"][*k as usize]
                    ));
                } else {
                    s.push_str(&format!("Mută J{}.\n", self.side.idx() + 1));
                }
            }
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match *m {
            Move::Buy(i) => format!("compra {}", i + 1),
            Move::Done => "gata".into(),
            Move::Play(i) => format!("joc {}", i + 1),
            Move::Sell(i) => format!("vinde {}", i + 1),
            Move::Pump => "pompa".into(),
            Move::Irrigate => "irigare".into(),
            Move::Plant => "plantare".into(),
            Move::Settle => "asezare".into(),
            Move::ConvSeeds => "conv sem".into(),
            Move::ConvHeat => "conv caldura".into(),
            Move::Pass => "pas".into(),
            Move::Place(i) => format!("pune {}", cell_name(i as usize)),
            Move::Deal => "generatie".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn quiet() -> Reverdire {
        let mut g = Reverdire::with_seed(1);
        g.apply(&Move::Deal);
        g.apply(&Move::Done);
        g.apply(&Move::Done);
        g
    }

    #[test]
    fn deck_has_two_copies() {
        let g = Reverdire::with_seed(1);
        assert_eq!(g.deck.len(), 2 * base_cards().len());
        assert_eq!(LAKE_CELLS.len() as u8, LAKES_MAX);
    }

    #[test]
    fn research_costs_three_per_card() {
        let mut g = Reverdire::with_seed(2);
        g.apply(&Move::Deal);
        assert_eq!(g.offer[0].len(), 4);
        g.apply(&Move::Buy(0));
        assert_eq!(g.res[0][0], 17);
        assert_eq!(g.hand[0].len(), 1);
        g.res[1][0] = 2;
        g.apply(&Move::Done);
        assert_eq!(g.legal_moves(), vec![Move::Done]);
    }

    #[test]
    fn payment_uses_bricks_for_b_cards() {
        let mut g = quiet();
        g.res[0] = [3, 4, 0, 0, 0, 0];
        let card = c("x", 1, 10, None, &[], 0);
        // 4 cărămizi = 8 Lei; mai lipsesc 2 Lei: are 3
        let (lei, sub, used) = g.payment(0, &card).unwrap();
        assert_eq!((lei, sub, used), (2, 1, 4));
        g.res[0] = [1, 4, 0, 0, 0, 0];
        assert!(g.payment(0, &card).is_none());
        let s_card = c("y", 2, 9, None, &[], 0);
        g.res[0] = [0, 0, 3, 0, 0, 0];
        assert_eq!(g.payment(0, &s_card), Some((0, 2, 3)));
    }

    #[test]
    fn parameters_raise_rating_and_cap() {
        let mut g = quiet();
        let tr = g.tr[0];
        g.raise_umid(0, 3);
        assert_eq!((g.umid, g.tr[0]), (3, tr + 3));
        g.raise_aer(0, 20);
        assert_eq!(g.aer, AER_MAX);
        assert_eq!(g.tr[0], tr + 3 + i32::from(AER_MAX));
    }

    #[test]
    fn tile_rules() {
        let mut g = quiet();
        g.board[12] = 0;
        // arbore: nicio plăcuță proprie -> orice pământ
        assert_eq!(g.cells_for(0, 1).len(), 25 - 7);
        g.board[0] = 2;
        // lipit de plăcuța proprie: doar vecinii lui 0 (1 e lac -> exclus)
        assert_eq!(g.cells_for(0, 1), vec![5]);
        // oraș: nu lângă alt oraș
        g.board[6] = 4;
        assert!(!g.cells_for(1, 2).contains(&7));
        assert!(!g.cells_for(1, 2).contains(&5));
        // lac doar pe celule-lac
        assert!(g.cells_for(0, 0).iter().all(|i| is_lake_cell(*i)));
    }

    #[test]
    fn lake_neighbour_bonus_and_scoring() {
        let mut g = quiet();
        g.board[1] = 1;
        g.board[8] = 1;
        g.res[0][0] = 0;
        g.pending = vec![1];
        g.place(0, 6); // vecin cu 1 (sus) și 7? 6 are vecinii 1,11,5,7: un lac
        assert_eq!(g.res[0][0], 2);
        g.board = [0; 25];
        g.board[6] = 4;
        g.board[7] = 2;
        g.board[5] = 3;
        assert_eq!(g.score(0) - g.tr[0] - g.vp[0], 3);
        assert_eq!(g.score(1) - g.tr[1] - g.vp[1], 1);
    }

    #[test]
    fn production_converts_energy() {
        let mut g = quiet();
        g.res[0] = [0, 0, 0, 0, 5, 1];
        g.prod[0] = [2, 0, 0, 0, 1, 3];
        g.passed = [true, true];
        let tr = g.tr[0];
        g.production_phase();
        assert_eq!(g.res[0][0], tr + 2);
        assert_eq!(g.res[0][4], 1);
        assert_eq!(g.res[0][5], 1 + 5 + 3);
        assert_eq!(g.gen, 2);
    }

    #[test]
    fn random_games_finish_and_conserve_cards() {
        for seed in 1..30u64 {
            let mut g = Reverdire::with_seed(seed);
            let t = g.card_total();
            let mut rng = Rng::new(seed ^ 21);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty(), "seed {seed}\n{}", g.render());
                g.apply(&m[rng.below(m.len())]);
                assert_eq!(g.card_total(), t, "seed {seed}");
                n += 1;
                assert!(n < 3500, "too long seed {seed}");
            }
        }
    }

    #[test]
    fn ai_beats_random() {
        let mut wins = 0;
        for seed in 1..=16u64 {
            let mut g = Reverdire::with_seed(seed);
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
        assert!(wins >= 13, "wins {wins}");
    }

    #[test]
    fn render_hides_the_opponents_cards() {
        let mut g = Reverdire::with_seed(9);
        g.apply(&Move::Deal);
        g.offer[1][0].name = "SECRETNAME";
        g.offer[0][0].name = "VISIBLENAME";
        g.hand[1].push(g.offer[1][0]);
        let r = g.render();
        assert!(!r.contains("SECRETNAME"));
        assert!(r.contains("VISIBLENAME"));
    }
}
