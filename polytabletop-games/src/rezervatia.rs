//! Rezervația — construirea unei rezervații naturale cu cărți-animale și sponsori
//! (variantă originală pentru doi; tema, cărțile și textele sunt ale noastre).
//!
//! Fiecare jucător are cinci cărți-acțiune așezate într-un rând; cartea din locul `k`
//! (1–5) are puterea `k`. După ce joci o acțiune, cartea ei trece pe locul 1, iar
//! cele care erau dedesubt urcă un loc: ce faci des rămâne slab, ce amâni devine puternic.
//!
//! Acțiuni (puterea `k` = locul cărții):
//! * Faună — `animal 2 e1`: joci animalul 2 din mână în tarcul e1 (habitat potrivit, loc
//!   liber ≥ mărimea animalului, plătești banii). Cu `k ≥ 4` poți juca încă un animal
//!   (`animal ...` sau `gata`).
//! * Amenajare — `construieste padure 3` (mărime ≤ `k`; costă 2·mărime+1, cu 1 mai puțin la
//!   `k ≥ 4`), `chiosc` (`k ≥ 2`, +1 venit, +1 apel), `pavilion` (`k ≥ 4`, +4 apel).
//! * Sponsori — `sponsor 3` (nivel ≤ `k`, cerința de etichete îndeplinită) sau `finantare`
//!   (primești `k + 2` bani).
//! * Cărți — trag `1 + (k+1)/2` cărți (`carti`) sau iau o carte din vitrină și trag restul
//!   (`carti v2`).
//! * Conservare — `proiect 1` (nivel ≤ `k`, cere animale cu eticheta proiectului; fiecare jucător
//!   poate susține fiecare proiect o dată) sau `donatie` (plătești `8 − k`, +1 conservare).
//!
//! Etichete: R prădători, C copitate, Z zburătoare, M mici. Habitate: Pădure, Munte, Luncă.
//! Pauză: fiecare acțiune adaugă `k` la ritmul tău; la 9 încasezi venitul (5 + apel/10 +
//! chioșcuri + sponsori + animale cu venit) și sponsorii de apel îți dau apel.
//! Scor = apel + 2 × conservare. Cine ajunge la 50 încheie jocul la sfârșitul rundei
//! (J2 are mereu tura finală); cel mult 30 de ture pe jucător. Egal: mai mulți bani, apoi remiză.
//! Informație ascunsă: pachetul și mâna adversarului; AI-ul nu le citește.
#![allow(clippy::needless_range_loop, clippy::doc_lazy_continuation)]

use crate::cards::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::sync::OnceLock;

pub const TARGET: i32 = 50;
const MAX_TURNS: u32 = 30;
const BREAK_AT: i32 = 9;
const HAND_MAX: usize = 8;
const MAX_ENC: usize = 6;
const HAB_L: [char; 3] = ['P', 'M', 'L'];
const HAB_N: [&str; 3] = ["Pădure", "Munte", "Luncă"];
const HAB_CMD: [&str; 3] = ["padure", "munte", "lunca"];
const TAG_L: [char; 4] = ['R', 'C', 'Z', 'M'];
const ACT_N: [&str; 5] = ["Faună", "Amenajare", "Sponsori", "Cărți", "Conservare"];
const A_FAUNA: usize = 0;
const A_BUILD: usize = 1;
const A_SPONS: usize = 2;
const A_CARDS: usize = 3;
const A_CONS: usize = 4;

/// (nume, habitat 0 Pădure / 1 Munte / 2 Luncă, etichetă 0 R / 1 C / 2 Z / 3 M)
const ANIM: [(&str, u8, u8); 56] = [
    ("Lupul cenușiu", 0, 0),
    ("Ursul brun", 0, 0),
    ("Râsul", 0, 0),
    ("Pisica sălbatică", 0, 0),
    ("Vulpea roșcată", 0, 0),
    ("Cerbul carpatin", 0, 1),
    ("Căprioara", 0, 1),
    ("Mistrețul", 0, 1),
    ("Zimbrul", 0, 1),
    ("Cerbul lopătar", 0, 1),
    ("Bufnița mare", 0, 2),
    ("Ciocănitoarea neagră", 0, 2),
    ("Gaița albastră", 0, 2),
    ("Cocoșul de mesteacăn", 0, 2),
    ("Veverița roșcată", 0, 3),
    ("Ariciul", 0, 3),
    ("Pârșul", 0, 3),
    ("Dihorul", 0, 3),
    ("Jderul de copac", 0, 0),
    ("Capra neagră", 1, 1),
    ("Muflonul", 1, 1),
    ("Ibexul alpin", 1, 1),
    ("Cerbul de stâncă", 1, 1),
    ("Marmota", 1, 3),
    ("Pikaul", 1, 3),
    ("Șoarecele de zăpadă", 1, 3),
    ("Chițcanul de piatră", 1, 3),
    ("Rupicapra pitică", 1, 3),
    ("Acvila de munte", 1, 2),
    ("Vulturul bărbos", 1, 2),
    ("Corbul", 1, 2),
    ("Brumărița alpină", 1, 2),
    ("Șoimul călător", 1, 2),
    ("Pisica de stâncă", 1, 0),
    ("Lupul de creastă", 1, 0),
    ("Hermelina", 1, 0),
    ("Ursul de piatră", 1, 0),
    ("Vidra", 2, 0),
    ("Nurca", 2, 0),
    ("Pisica de stuf", 2, 0),
    ("Șacalul auriu", 2, 0),
    ("Dihorul de stuf", 2, 0),
    ("Bivolul", 2, 1),
    ("Calul de deltă", 2, 1),
    ("Elanul", 2, 1),
    ("Mistrețul de baltă", 2, 1),
    ("Pelicanul creț", 2, 2),
    ("Egreta mare", 2, 2),
    ("Barza neagră", 2, 2),
    ("Lebăda cucuiată", 2, 2),
    ("Stârcul cenușiu", 2, 2),
    ("Cormoranul mic", 2, 2),
    ("Nutria", 2, 3),
    ("Șobolanul de apă", 2, 3),
    ("Țestoasa de baltă", 2, 3),
    ("Broasca-țestoasă bălțată", 2, 3),
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Fx {
    None,
    /// Animal: bani la joc.
    Money(u8),
    /// Animal: trage cărți la joc.
    Draw(u8),
    /// Venit la fiecare pauză (animal sau sponsor).
    Inc(u8),
    /// Animal: apel = animale deja prezente cu aceeași etichetă (cel mult 3).
    Syn,
    /// Sponsor: apel la fiecare pauză.
    BreakAp(u8),
    /// Sponsor: conservare imediată.
    Cons(u8),
    /// Sponsor: apel imediat = 2 × animalele cu eticheta dată (cel mult 4).
    TagBonus(u8),
    /// Sponsor: animalele costă cu 1 mai puțin.
    Disc,
    /// Sponsor: apel imediat = numărul de animale (cel mult 8).
    Zoo,
}

#[derive(Clone, Copy, Debug)]
pub struct Card {
    pub id: u8,
    pub name: &'static str,
    pub sponsor: bool,
    pub hab: u8,
    pub size: u8,
    pub cost: u8,
    pub ap: u8,
    pub cons: u8,
    pub tag: u8,
    pub level: u8,
    pub req_tag: u8,
    pub req_n: u8,
    pub fx: Fx,
}

impl Card {
    pub fn text(&self) -> String {
        if self.sponsor {
            let req = if self.req_n > 0 {
                format!(" cere {}×{}", self.req_n, TAG_L[self.req_tag as usize])
            } else {
                String::new()
            };
            format!(
                "Sponsor {} [nivel {} | {} bani | apel {}{} | {}]",
                self.name,
                self.level,
                self.cost,
                self.ap,
                req,
                fx_text(self.fx)
            )
        } else {
            format!(
                "{} [{} {} | mărime {} | {} bani | apel {}{} | {}{}]",
                self.name,
                HAB_L[self.hab as usize],
                HAB_N[self.hab as usize],
                self.size,
                self.cost,
                self.ap,
                if self.cons > 0 {
                    format!(" cons {}", self.cons)
                } else {
                    String::new()
                },
                TAG_L[self.tag as usize],
                match self.fx {
                    Fx::None => String::new(),
                    f => format!(" | {}", fx_text(f)),
                }
            )
        }
    }
}

fn fx_text(f: Fx) -> String {
    match f {
        Fx::None => "fără efect".into(),
        Fx::Money(n) => format!("+{n} bani la joc"),
        Fx::Draw(n) => format!("+{n} cărți la joc"),
        Fx::Inc(n) => format!("+{n} venit/pauză"),
        Fx::Syn => "+apel pentru fiecare animal de același fel (max 3)".into(),
        Fx::BreakAp(n) => format!("+{n} apel/pauză"),
        Fx::Cons(n) => format!("+{n} conservare"),
        Fx::TagBonus(t) => format!("apel 2× animale {}", TAG_L[t as usize]),
        Fx::Disc => "animalele costă −1".into(),
        Fx::Zoo => "apel = număr de animale".into(),
    }
}

/// (nume, nivel, cost, apel, efect, etichetă cerută, câte)
type Sp = (&'static str, u8, u8, u8, Fx, u8, u8);
const SPONS: [Sp; 24] = [
    ("Fundația Dunărea Vie", 1, 3, 1, Fx::Inc(1), 0, 0),
    ("Clubul Prietenilor", 1, 4, 0, Fx::BreakAp(1), 0, 0),
    ("Cooperativa Pădurarilor", 1, 3, 1, Fx::Disc, 0, 0),
    ("Gazeta Naturii", 1, 2, 0, Fx::Inc(1), 3, 1),
    ("Banca Verde", 2, 5, 1, Fx::Inc(2), 0, 0),
    ("Societatea Ornitologică", 2, 5, 1, Fx::TagBonus(2), 2, 2),
    ("Institutul Carpatin", 2, 5, 1, Fx::Cons(1), 0, 0),
    (
        "Cercul Vânătorilor Reformați",
        2,
        6,
        1,
        Fx::TagBonus(0),
        0,
        2,
    ),
    ("Păstorii Reuniți", 2, 5, 1, Fx::TagBonus(1), 1, 2),
    ("Asociația Micilor Iubitori", 2, 4, 1, Fx::TagBonus(3), 3, 2),
    ("Fundația Delta", 3, 7, 2, Fx::Inc(2), 0, 0),
    ("Consiliul Județean", 3, 6, 2, Fx::Disc, 0, 0),
    ("Festivalul Toamnei", 3, 8, 1, Fx::BreakAp(2), 0, 0),
    ("Academia de Științe", 3, 9, 1, Fx::Cons(2), 0, 0),
    ("Expediția Zimbrului", 3, 7, 3, Fx::Cons(1), 1, 3),
    ("Turismul Rural", 3, 5, 2, Fx::BreakAp(1), 0, 0),
    ("Ministerul Mediului", 4, 10, 2, Fx::Inc(3), 0, 0),
    ("Fondul European", 4, 10, 2, Fx::Cons(2), 2, 1),
    ("Marele Patron", 4, 9, 3, Fx::BreakAp(2), 0, 0),
    ("Observatorul Național", 4, 8, 2, Fx::TagBonus(2), 2, 3),
    ("Mecena", 5, 12, 3, Fx::Inc(4), 0, 0),
    ("Tezaurul Natural", 5, 13, 3, Fx::Cons(3), 0, 2),
    ("Parcul Național", 5, 12, 4, Fx::BreakAp(3), 0, 0),
    ("Marea Expoziție", 5, 11, 2, Fx::Zoo, 0, 0),
];

/// (nume, etichetă, câte, nivel, conservare)
const PROJ: [(&str, u8, u8, u8, i32); 8] = [
    ("Cărări pentru zimbri", 1, 2, 2, 2),
    ("Refugiul lupilor", 0, 2, 2, 2),
    ("Rezervația păsărilor", 2, 3, 3, 3),
    ("Rețeaua de ariciuri", 3, 2, 1, 1),
    ("Coridorul ursului", 0, 3, 4, 4),
    ("Luncile vii", 2, 2, 2, 2),
    ("Crescătoria de mici", 3, 3, 3, 3),
    ("Turma de munte", 1, 3, 3, 3),
];

fn build_bank() -> Vec<Card> {
    let mut rng = Rng::new(0x52E2_0A11);
    let mut v = Vec::new();
    for (i, (name, hab, tag)) in ANIM.iter().enumerate() {
        let size = match tag {
            0 | 1 => 2 + rng.below(3),
            2 => 1 + rng.below(3),
            _ => 1 + rng.below(2),
        } as u8;
        let mut cost = 1 + 2 * size + rng.below(2) as u8;
        let mut ap = size + 1 + rng.below(3) as u8;
        let mut cons = 0;
        if rng.below(6) == 0 {
            cons = 1;
            cost += 1;
        }
        let fx = match rng.below(10) {
            0 | 1 | 9 => {
                ap += 1;
                Fx::None
            }
            2 | 7 => Fx::Money(2 + rng.below(2) as u8),
            3 | 8 => Fx::Draw(1 + rng.below(2) as u8),
            4 => {
                ap = ap.saturating_sub(1).max(1);
                Fx::Inc(1)
            }
            _ => Fx::Syn,
        };
        v.push(Card {
            id: i as u8,
            name,
            sponsor: false,
            hab: *hab,
            size,
            cost,
            ap,
            cons,
            tag: *tag,
            level: 0,
            req_tag: 0,
            req_n: 0,
            fx,
        });
    }
    for (j, s) in SPONS.iter().enumerate() {
        v.push(Card {
            id: (56 + j) as u8,
            name: s.0,
            sponsor: true,
            hab: 0,
            size: 0,
            cost: s.2,
            ap: s.3,
            cons: 0,
            tag: 0,
            level: s.1,
            req_tag: s.5,
            req_n: s.6,
            fx: s.4,
        });
    }
    v
}

pub fn bank() -> &'static [Card] {
    static B: OnceLock<Vec<Card>> = OnceLock::new();
    B.get_or_init(build_bank)
}

fn card(id: u8) -> &'static Card {
    &bank()[id as usize]
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    /// carte din mână, tarc
    Animal(u8, u8),
    Done,
    Build(u8, u8),
    Kiosk,
    Pavilion,
    Sponsor(u8),
    Fund,
    /// 0 = toate din pachet, d = cartea d din vitrină + restul din pachet
    Draw(u8),
    Project(u8),
    Donate,
}

#[derive(Clone, Copy, Debug)]
struct Enc {
    kind: u8,
    size: u8,
    used: u8,
}

#[derive(Clone, Debug)]
struct Zoo {
    money: i32,
    appeal: i32,
    cons: i32,
    tempo: i32,
    enc: Vec<Enc>,
    animals: Vec<u8>,
    sponsors: Vec<u8>,
    kiosks: u8,
    pavilion: bool,
    /// Acțiunea de pe locul `j` (puterea `j+1`).
    order: [u8; 5],
    hand: Vec<u8>,
    proj_done: u8,
}

#[derive(Clone)]
pub struct Rezervatia {
    /// Whose private info `render` shows (`None` = the mover's, hot-seat).
    viewer: Option<Player>,
    deck: Vec<u8>,
    display: Vec<u8>,
    z: [Zoo; 2],
    proj: [u8; 3],
    side: Player,
    turns: [u32; 2],
    trigger: bool,
    over: bool,
    pending: bool,
    fresh: [u8; 2],
}

impl Default for Rezervatia {
    fn default() -> Self {
        Self::new()
    }
}

impl Rezervatia {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck: Vec<u8> = (0..bank().len() as u8).collect();
        shuffle(&mut deck, &mut rng);
        let mk = |money: i32, kind: u8| Zoo {
            money,
            appeal: 0,
            cons: 0,
            tempo: 0,
            enc: vec![Enc {
                kind,
                size: 3,
                used: 0,
            }],
            animals: Vec::new(),
            sponsors: Vec::new(),
            kiosks: 0,
            pavilion: false,
            order: [
                A_CONS as u8,
                A_SPONS as u8,
                A_CARDS as u8,
                A_BUILD as u8,
                A_FAUNA as u8,
            ],
            hand: Vec::new(),
            proj_done: 0,
        };
        let k1 = rng.below(3) as u8;
        let k2 = rng.below(3) as u8;
        let mut pr: Vec<u8> = (0..8).collect();
        shuffle(&mut pr, &mut rng);
        let mut g = Rezervatia {
            viewer: None,
            deck,
            display: Vec::new(),
            z: [mk(14, k1), mk(15, k2)],
            proj: [pr[0], pr[1], pr[2]],
            side: Player::One,
            turns: [0; 2],
            trigger: false,
            over: false,
            pending: false,
            fresh: [0; 2],
        };
        for p in 0..2 {
            g.draw(p, 5);
        }
        for _ in 0..4 {
            if let Some(c) = g.deck.pop() {
                g.display.push(c);
            }
        }
        g.fresh = [0; 2];
        g
    }

    fn draw(&mut self, p: usize, n: usize) {
        for _ in 0..n {
            if self.z[p].hand.len() >= HAND_MAX {
                break;
            }
            if let Some(c) = self.deck.pop() {
                self.z[p].hand.push(c);
                self.fresh[p] += 1;
            }
        }
    }

    pub fn card_total(&self) -> usize {
        self.deck.len()
            + self.display.len()
            + (0..2)
                .map(|p| self.z[p].hand.len() + self.z[p].animals.len() + self.z[p].sponsors.len())
                .sum::<usize>()
    }

    pub fn score(&self, p: usize) -> i32 {
        self.z[p].appeal + 2 * self.z[p].cons
    }

    fn strength(&self, p: usize, act: usize) -> i32 {
        self.z[p]
            .order
            .iter()
            .position(|a| *a as usize == act)
            .unwrap() as i32
            + 1
    }

    fn tag_count(&self, p: usize, t: u8) -> u8 {
        self.z[p]
            .animals
            .iter()
            .filter(|a| card(**a).tag == t)
            .count() as u8
    }

    fn disc(&self, p: usize) -> i32 {
        self.z[p]
            .sponsors
            .iter()
            .filter(|s| card(**s).fx == Fx::Disc)
            .count() as i32
    }

    fn animal_cost(&self, p: usize, c: &Card) -> i32 {
        (i32::from(c.cost) - self.disc(p)).max(1)
    }

    fn income(&self, p: usize) -> i32 {
        let z = &self.z[p];
        let mut inc = 5 + z.appeal / 10 + i32::from(z.kiosks);
        for s in z.sponsors.iter().chain(z.animals.iter()) {
            if let Fx::Inc(n) = card(*s).fx {
                inc += i32::from(n);
            }
        }
        inc
    }

    fn break_ap(&self, p: usize) -> i32 {
        self.z[p]
            .sponsors
            .iter()
            .map(|s| match card(*s).fx {
                Fx::BreakAp(n) => i32::from(n),
                _ => 0,
            })
            .sum()
    }

    fn build_cost(&self, p: usize, size: i32) -> i32 {
        2 * size + 1 - i32::from(self.strength(p, A_BUILD) >= 4)
    }

    fn animal_moves(&self, p: usize, out: &mut Vec<Move>) {
        let z = &self.z[p];
        for (hi, id) in z.hand.iter().enumerate() {
            let c = card(*id);
            if c.sponsor || z.money < self.animal_cost(p, c) {
                continue;
            }
            for (ei, e) in z.enc.iter().enumerate() {
                if e.kind == c.hab && e.size - e.used >= c.size {
                    out.push(Move::Animal(hi as u8, ei as u8));
                }
            }
        }
    }

    fn project_ok(&self, p: usize, i: usize, k: i32) -> bool {
        let (_, tag, need, level, _) = PROJ[self.proj[i] as usize];
        self.z[p].proj_done & (1 << i) == 0
            && k >= i32::from(level)
            && self.tag_count(p, tag) >= need
    }

    fn after_action(&mut self, act: usize, k: i32) {
        let p = self.side.idx();
        {
            let z = &mut self.z[p];
            let j = z.order.iter().position(|a| *a as usize == act).unwrap();
            z.order.remove_shift(j);
            z.tempo += k;
        }
        if self.z[p].tempo >= BREAK_AT {
            self.z[p].tempo -= BREAK_AT;
            let inc = self.income(p);
            let ap = self.break_ap(p);
            self.z[p].money += inc;
            self.z[p].appeal += ap;
        }
    }

    fn end_turn(&mut self) {
        let p = self.side.idx();
        self.turns[p] += 1;
        if self.score(p) >= TARGET {
            self.trigger = true;
        }
        if self.side == Player::Two && (self.trigger || self.turns[1] >= MAX_TURNS) {
            self.over = true;
        }
        self.side = self.side.other();
        self.pending = false;
    }

    /// Valoarea poziției lui `p` din punctul de vedere al lui `me`: din mâna adversarului
    /// se folosește doar numărul de cărți, niciodată identitatea lor.
    fn heuristic(&self, p: usize, me: usize) -> f64 {
        let z = &self.z[p];
        let mut s = 10.0 * f64::from(self.score(p));
        s += 1.0 * f64::from(z.money.min(16)) + 0.2 * f64::from((z.money - 16).max(0));
        s += 5.0 * f64::from(self.income(p) - 5 - z.appeal / 10) + 1.0 * f64::from(z.tempo);
        s += 6.0 * f64::from(self.break_ap(p));
        let known = if p == me {
            z.hand.len().saturating_sub(self.fresh[p] as usize)
        } else {
            0
        };
        s += 1.5 * z.hand.len() as f64;
        if p != me {
            s += 4.0 * z.hand.len() as f64;
        }
        for id in z.hand.iter().take(known) {
            let c = card(*id);
            if c.sponsor {
                s += 1.0;
            } else if z
                .enc
                .iter()
                .any(|e| e.kind == c.hab && e.size - e.used >= c.size)
            {
                s += 7.0 + f64::from(c.ap) * 2.0;
            } else {
                s += 0.5;
            }
        }
        let free: i32 = z.enc.iter().map(|e| i32::from(e.size - e.used)).sum();
        s += 0.8 * f64::from(free.min(6));
        let w = [1.2, 0.9, 0.7, 0.7, 0.9];
        for j in 0..5 {
            s += w[z.order[j] as usize] * (j as f64 + 1.0);
        }
        s
    }
}

trait RemoveShift {
    fn remove_shift(&mut self, j: usize);
}
impl RemoveShift for [u8; 5] {
    /// Mută elementul de pe locul `j` pe locul 0; cele de dinainte urcă un loc.
    fn remove_shift(&mut self, j: usize) {
        let a = self[j];
        for i in (1..=j).rev() {
            self[i] = self[i - 1];
        }
        self[0] = a;
    }
}

impl Game for Rezervatia {
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
        let z = &self.z[p];
        let mut v = Vec::new();
        self.animal_moves(p, &mut v);
        if self.pending {
            v.push(Move::Done);
            return v;
        }
        let kb = self.strength(p, A_BUILD);
        if z.enc.len() < MAX_ENC {
            for kind in 0..3u8 {
                for size in 1..=kb.min(4) {
                    if z.money >= self.build_cost(p, size) {
                        v.push(Move::Build(kind, size as u8));
                    }
                }
            }
        }
        if kb >= 2 && z.kiosks < 2 && z.money >= 5 {
            v.push(Move::Kiosk);
        }
        if kb >= 4 && !z.pavilion && z.money >= 8 {
            v.push(Move::Pavilion);
        }
        let ks = self.strength(p, A_SPONS);
        for (hi, id) in z.hand.iter().enumerate() {
            let c = card(*id);
            if c.sponsor
                && i32::from(c.level) <= ks
                && z.money >= i32::from(c.cost)
                && (c.req_n == 0 || self.tag_count(p, c.req_tag) >= c.req_n)
            {
                v.push(Move::Sponsor(hi as u8));
            }
        }
        v.push(Move::Fund);
        v.push(Move::Draw(0));
        for d in 0..self.display.len() {
            v.push(Move::Draw(d as u8 + 1));
        }
        let kc = self.strength(p, A_CONS);
        for i in 0..3 {
            if self.project_ok(p, i, kc) {
                v.push(Move::Project(i as u8));
            }
        }
        if z.money >= 8 - kc {
            v.push(Move::Donate);
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        let p = self.side.idx();
        self.fresh = [0; 2];
        match *m {
            Move::Done => {
                self.end_turn();
                return;
            }
            Move::Animal(hi, ei) => {
                let k = self.strength(p, A_FAUNA);
                let id = self.z[p].hand.remove(hi as usize);
                let c = card(id);
                let cost = self.animal_cost(p, c);
                let syn = if c.fx == Fx::Syn {
                    i32::from(self.tag_count(p, c.tag).min(3))
                } else {
                    0
                };
                {
                    let z = &mut self.z[p];
                    z.money -= cost;
                    z.enc[ei as usize].used += c.size;
                    z.animals.push(id);
                    z.appeal += i32::from(c.ap) + syn;
                    z.cons += i32::from(c.cons);
                    if let Fx::Money(n) = c.fx {
                        z.money += i32::from(n);
                    }
                }
                if let Fx::Draw(n) = c.fx {
                    self.draw(p, n as usize);
                }
                if !self.pending {
                    self.after_action(A_FAUNA, k);
                    if k >= 4 {
                        let mut more = Vec::new();
                        self.animal_moves(p, &mut more);
                        if !more.is_empty() {
                            self.pending = true;
                            return;
                        }
                    }
                } else {
                    // al doilea animal: nimic de făcut în plus
                }
                self.end_turn();
                return;
            }
            Move::Build(kind, size) => {
                let k = self.strength(p, A_BUILD);
                let cost = self.build_cost(p, i32::from(size));
                let z = &mut self.z[p];
                z.money -= cost;
                z.enc.push(Enc {
                    kind,
                    size,
                    used: 0,
                });
                self.after_action(A_BUILD, k);
            }
            Move::Kiosk => {
                let k = self.strength(p, A_BUILD);
                let z = &mut self.z[p];
                z.money -= 5;
                z.kiosks += 1;
                z.appeal += 1;
                self.after_action(A_BUILD, k);
            }
            Move::Pavilion => {
                let k = self.strength(p, A_BUILD);
                let z = &mut self.z[p];
                z.money -= 8;
                z.pavilion = true;
                z.appeal += 4;
                self.after_action(A_BUILD, k);
            }
            Move::Sponsor(hi) => {
                let k = self.strength(p, A_SPONS);
                let id = self.z[p].hand.remove(hi as usize);
                let c = card(id);
                let bonus = match c.fx {
                    Fx::TagBonus(t) => 2 * i32::from(self.tag_count(p, t).min(4)),
                    Fx::Zoo => self.z[p].animals.len().min(8) as i32,
                    _ => 0,
                };
                let z = &mut self.z[p];
                z.money -= i32::from(c.cost);
                z.appeal += i32::from(c.ap) + bonus;
                if let Fx::Cons(n) = c.fx {
                    z.cons += i32::from(n);
                }
                z.sponsors.push(id);
                self.after_action(A_SPONS, k);
            }
            Move::Fund => {
                let k = self.strength(p, A_SPONS);
                self.z[p].money += k + 2;
                self.after_action(A_SPONS, k);
            }
            Move::Draw(d) => {
                let k = self.strength(p, A_CARDS);
                let mut n = 1 + ((k + 1) / 2) as usize;
                if d > 0 {
                    let id = self.display.remove(d as usize - 1);
                    if self.z[p].hand.len() < HAND_MAX {
                        self.z[p].hand.push(id);
                    } else {
                        self.display.insert(d as usize - 1, id);
                    }
                    n -= 1;
                }
                self.draw(p, n);
                while self.display.len() < 4 {
                    match self.deck.pop() {
                        Some(c) => self.display.push(c),
                        None => break,
                    }
                }
                self.after_action(A_CARDS, k);
            }
            Move::Project(i) => {
                let k = self.strength(p, A_CONS);
                let (_, _, _, _, r) = PROJ[self.proj[i as usize] as usize];
                let z = &mut self.z[p];
                z.proj_done |= 1 << i;
                z.cons += r;
                self.after_action(A_CONS, k);
            }
            Move::Donate => {
                let k = self.strength(p, A_CONS);
                let z = &mut self.z[p];
                z.money -= 8 - k;
                z.cons += 1;
                self.after_action(A_CONS, k);
            }
        }
        self.end_turn();
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            return None;
        }
        let (a, b) = (self.score(0), self.score(1));
        let (ma, mb) = (self.z[0].money, self.z[1].money);
        Some(if (a, ma) > (b, mb) {
            Outcome::Win(Player::One)
        } else if (b, mb) > (a, ma) {
            Outcome::Win(Player::Two)
        } else {
            Outcome::Draw
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        (self.heuristic(p.idx(), p.idx()) - self.heuristic(p.other().idx(), p.idx())) as i32
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }

    fn render(&self) -> String {
        let mut s = String::new();
        if self.over {
            s.push_str("Rezervația — joc încheiat\n");
        } else {
            s.push_str(&format!(
                "Rezervația — tura {} (țintă {} = apel + 2×conservare){}\n",
                self.turns[self.side.idx()] + 1,
                TARGET,
                if self.trigger {
                    " — RUNDA FINALĂ"
                } else {
                    ""
                }
            ));
        }
        s.push_str("Proiecte de conservare:\n");
        for i in 0..3 {
            let (n, t, c, l, r) = PROJ[self.proj[i] as usize];
            s.push_str(&format!(
                "  {}: {} (cere {}×{}, nivel {}, +{} cons)\n",
                i + 1,
                n,
                c,
                TAG_L[t as usize],
                l,
                r
            ));
        }
        s.push_str("Vitrină:\n");
        for (i, id) in self.display.iter().enumerate() {
            s.push_str(&format!("  v{}: {}\n", i + 1, card(*id).text()));
        }
        for p in 0..2 {
            let z = &self.z[p];
            s.push_str(&format!(
                "J{}: scor {} (apel {}, conservare {}) | bani {} | ritm {}/{} | mână {} cărți | venit {}\n",
                p + 1,
                self.score(p),
                z.appeal,
                z.cons,
                z.money,
                z.tempo,
                BREAK_AT,
                z.hand.len(),
                self.income(p)
            ));
            let acts: Vec<String> = (0..5)
                .map(|j| format!("{}={}", j + 1, ACT_N[z.order[j] as usize]))
                .collect();
            s.push_str(&format!("   acțiuni (loc=putere): {}\n", acts.join("  ")));
            for (ei, e) in z.enc.iter().enumerate() {
                s.push_str(&format!(
                    "   e{}: {} {}/{}\n",
                    ei + 1,
                    HAB_N[e.kind as usize],
                    e.used,
                    e.size
                ));
            }
            let an: Vec<String> = z
                .animals
                .iter()
                .map(|a| {
                    let c = card(*a);
                    format!("{}({}{})", c.name, TAG_L[c.tag as usize], c.ap)
                })
                .collect();
            s.push_str(&format!(
                "   animale: {}\n",
                if an.is_empty() {
                    "—".into()
                } else {
                    an.join(", ")
                }
            ));
            let sp: Vec<&str> = z.sponsors.iter().map(|a| card(*a).name).collect();
            s.push_str(&format!(
                "   sponsori: {} | chioșcuri {} | pavilion {}\n",
                if sp.is_empty() {
                    "—".into()
                } else {
                    sp.join(", ")
                },
                z.kiosks,
                if z.pavilion { "da" } else { "nu" }
            ));
        }
        let v = self.viewer.unwrap_or_else(|| self.to_move()).idx();
        s.push_str(&format!(
            "Pachet: {} cărți\nMâna ta (J{}):\n",
            self.deck.len(),
            v + 1
        ));
        for (i, id) in self.z[v].hand.iter().enumerate() {
            s.push_str(&format!("  {}: {}\n", i + 1, card(*id).text()));
        }
        if !self.over {
            s.push_str(&format!(
                "Mută J{}.{}\n",
                self.side.idx() + 1,
                if self.pending {
                    " (al doilea animal sau «gata»)"
                } else {
                    ""
                }
            ));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match *m {
            Move::Animal(h, e) => format!("animal {} e{}", h + 1, e + 1),
            Move::Done => "gata".into(),
            Move::Build(k, s) => format!("construieste {} {}", HAB_CMD[k as usize], s),
            Move::Kiosk => "chiosc".into(),
            Move::Pavilion => "pavilion".into(),
            Move::Sponsor(h) => format!("sponsor {}", h + 1),
            Move::Fund => "finantare".into(),
            Move::Draw(0) => "carti".into(),
            Move::Draw(d) => format!("carti v{d}"),
            Move::Project(i) => format!("proiect {}", i + 1),
            Move::Donate => "donatie".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fresh(seed: u64) -> Rezervatia {
        Rezervatia::with_seed(seed)
    }

    fn animal_idx(pred: impl Fn(&Card) -> bool) -> u8 {
        bank().iter().find(|c| !c.sponsor && pred(c)).unwrap().id
    }

    #[test]
    fn bank_is_sane() {
        let b = bank();
        assert_eq!(b.len(), 80);
        assert_eq!(b.iter().filter(|c| c.sponsor).count(), 24);
        let mut names: Vec<&str> = b.iter().map(|c| c.name).collect();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), 80);
        for c in b.iter().filter(|c| !c.sponsor) {
            assert!(
                (1..=4).contains(&c.size) && c.cost >= 3 && c.ap >= 1,
                "{c:?}"
            );
        }
        for (i, c) in b.iter().enumerate() {
            assert_eq!(c.id as usize, i);
        }
    }

    #[test]
    fn action_row_rotates_after_playing() {
        let mut o = [4u8, 2, 3, 1, 0];
        o.remove_shift(3);
        assert_eq!(o, [1, 4, 2, 3, 0]);
        let mut g = fresh(1);
        let p = 0;
        assert_eq!(g.strength(p, A_FAUNA), 5);
        assert_eq!(g.strength(p, A_CONS), 1);
        g.apply(&Move::Fund); // Sponsori de la locul 2
        assert_eq!(g.strength(p, A_SPONS), 1);
        assert_eq!(g.strength(p, A_CONS), 2);
        assert_eq!(g.strength(p, A_FAUNA), 5);
        assert_eq!(g.z[0].money, 14 + 2 + 2);
    }

    #[test]
    fn animal_needs_habitat_space_and_money() {
        let mut g = fresh(2);
        let kind = g.z[0].enc[0].kind;
        let a = animal_idx(|c| c.hab == kind && c.size <= 3 && c.fx == Fx::None);
        let wrong = animal_idx(|c| c.hab != kind);
        g.z[0].hand = vec![a, wrong];
        g.z[0].money = 20;
        let mv = g.legal_moves();
        assert!(mv.contains(&Move::Animal(0, 0)));
        assert!(!mv.iter().any(|m| matches!(m, Move::Animal(1, _))));
        let c = card(a);
        let before = g.z[0].money;
        g.apply(&Move::Animal(0, 0));
        assert_eq!(g.z[0].money, before - i32::from(c.cost));
        assert_eq!(g.z[0].appeal, i32::from(c.ap));
        assert_eq!(g.z[0].enc[0].used, c.size);
        // Faună la putere 5 permite un al doilea animal doar dacă mai există unul jucabil
        assert!(!g.pending);
        // fără bani nu se poate
        let mut g = fresh(3);
        g.z[0].hand = vec![animal_idx(|c| c.hab == kind_of(&g) && c.size == 1)];
        g.z[0].money = 0;
        assert!(!g
            .legal_moves()
            .iter()
            .any(|m| matches!(m, Move::Animal(..))));
    }

    fn kind_of(g: &Rezervatia) -> u8 {
        g.z[0].enc[0].kind
    }

    #[test]
    fn two_animals_with_strength_four() {
        let mut g = fresh(4);
        let kind = g.z[0].enc[0].kind;
        g.z[0].enc[0].size = 4;
        let a = animal_idx(|c| c.hab == kind && c.size == 1);
        let b = bank()
            .iter()
            .find(|c| !c.sponsor && c.hab == kind && c.size == 1 && c.id != a)
            .unwrap()
            .id;
        g.z[0].hand = vec![a, b];
        g.z[0].money = 30;
        g.apply(&Move::Animal(0, 0));
        assert!(g.pending);
        assert_eq!(g.to_move(), Player::One);
        assert!(g.legal_moves().contains(&Move::Done));
        g.apply(&Move::Animal(0, 0));
        assert!(!g.pending);
        assert_eq!(g.to_move(), Player::Two);
        assert_eq!(g.z[0].animals.len(), 2);
        assert_eq!(g.turns[0], 1);
    }

    #[test]
    fn build_costs_and_limits() {
        let mut g = fresh(5);
        g.z[0].money = 20;
        // Amenajare e pe locul 4 (putere 4): cost 2·mărime+1−1
        let mv = g.legal_moves();
        assert!(mv.contains(&Move::Build(1, 4)));
        assert!(mv.contains(&Move::Kiosk) && mv.contains(&Move::Pavilion));
        g.apply(&Move::Build(1, 4));
        assert_eq!(g.z[0].money, 20 - 8);
        assert_eq!(g.z[0].enc.len(), 2);
        assert_eq!(g.strength(0, A_BUILD), 1);
        // acum la putere 1 doar mărime 1, cost 3
        g.side = Player::One;
        let mv = g.legal_moves();
        assert!(mv.contains(&Move::Build(0, 1)));
        assert!(!mv.contains(&Move::Build(0, 2)));
        assert!(!mv.contains(&Move::Kiosk));
    }

    #[test]
    fn sponsor_level_and_requirement() {
        let mut g = fresh(6);
        let sp = |name: &str| bank().iter().find(|c| c.name == name).unwrap().id;
        let banca = sp("Banca Verde"); // nivel 2, fără cerință
        let orni = sp("Societatea Ornitologică"); // nivel 2, cere 2 Z
        g.z[0].hand = vec![banca, orni];
        g.z[0].money = 30;
        // Sponsori e pe locul 2 la început
        let mv = g.legal_moves();
        assert!(mv.contains(&Move::Sponsor(0)));
        assert!(!mv.contains(&Move::Sponsor(1))); // lipsesc animalele zburătoare
        let ap = g.z[0].appeal;
        g.apply(&Move::Sponsor(0));
        assert_eq!(g.z[0].appeal, ap + 1);
        assert_eq!(g.z[0].money, 25);
        assert_eq!(g.income(0), 7);
        // nivelul 5 nu se poate la putere 1
        let mut g = fresh(7);
        g.z[0].hand = vec![sp("Mecena")];
        g.z[0].money = 30;
        assert!(!g
            .legal_moves()
            .iter()
            .any(|m| matches!(m, Move::Sponsor(_))));
    }

    #[test]
    fn break_gives_income_and_break_appeal() {
        let mut g = fresh(8);
        g.z[0].tempo = 8;
        g.z[0].appeal = 12;
        let patron = bank()
            .iter()
            .find(|c| c.name == "Clubul Prietenilor")
            .unwrap()
            .id;
        g.z[0].sponsors.push(patron);
        let m0 = g.z[0].money;
        g.apply(&Move::Fund); // putere 2: ritm 10 → pauză
        assert_eq!(g.z[0].tempo, 1);
        assert_eq!(g.z[0].money, m0 + 4 + 5 + 1); // finanțare 2+2, venit 5+12/10
        assert_eq!(g.z[0].appeal, 13);
    }

    #[test]
    fn project_and_donation() {
        let mut g = fresh(9);
        g.proj = [3, 0, 1]; // ariciuri (3 mici? cere 2 M nivel 1), zimbri, lupi
        g.z[0].animals = vec![
            animal_idx(|c| c.tag == 3),
            bank()
                .iter()
                .filter(|c| !c.sponsor && c.tag == 3)
                .nth(1)
                .unwrap()
                .id,
        ];
        // Conservare e pe locul 1
        let mv = g.legal_moves();
        assert!(mv.contains(&Move::Project(0)));
        assert!(!mv.contains(&Move::Project(1)));
        g.apply(&Move::Project(0));
        assert_eq!(g.z[0].cons, 1);
        assert_eq!(g.z[0].proj_done, 1);
        g.side = Player::One;
        assert!(!g.legal_moves().contains(&Move::Project(0)));
        // donație: la putere 1 costă 7
        g.z[0].money = 7;
        g.side = Player::One;
        let k = g.strength(0, A_CONS);
        assert_eq!(k, 1);
        assert!(g.legal_moves().contains(&Move::Donate));
        g.apply(&Move::Donate);
        assert_eq!(g.z[0].money, 0);
        assert_eq!(g.z[0].cons, 2);
    }

    #[test]
    fn cards_action_uses_display_and_hand_cap() {
        let mut g = fresh(10);
        let shown = g.display[1];
        let h = g.z[0].hand.len();
        g.side = Player::One;
        g.apply(&Move::Draw(2)); // Cărți pe locul 3 → 3 cărți
        assert_eq!(g.z[0].hand.len(), h + 3);
        assert!(g.z[0].hand.contains(&shown));
        assert_eq!(g.display.len(), 4);
        assert_eq!(g.card_total(), 80);
        // plafon de mână
        let mut g = fresh(11);
        while g.z[0].hand.len() < HAND_MAX {
            let c = g.deck.pop().unwrap();
            g.z[0].hand.push(c);
        }
        g.apply(&Move::Draw(0));
        assert_eq!(g.z[0].hand.len(), HAND_MAX);
        assert_eq!(g.card_total(), 80);
    }

    #[test]
    fn synergy_and_discount() {
        let mut g = fresh(12);
        let kind = g.z[0].enc[0].kind;
        g.z[0].enc[0].size = 4;
        let cooperativa = bank()
            .iter()
            .find(|c| c.name == "Cooperativa Pădurarilor")
            .unwrap()
            .id;
        g.z[0].sponsors.push(cooperativa);
        let a = animal_idx(|c| c.hab == kind && c.size == 1 && c.fx == Fx::None);
        let c = card(a);
        g.z[0].hand = vec![a];
        g.z[0].money = 10;
        g.apply(&Move::Animal(0, 0));
        assert_eq!(g.z[0].money, 10 - i32::from(c.cost) + 1);
        // Syn: apel + animale de același fel
        let syn = bank()
            .iter()
            .find(|c| !c.sponsor && c.fx == Fx::Syn && c.size == 1)
            .map(|c| c.id);
        if let Some(sid) = syn {
            let sc = card(sid);
            let mut g = fresh(13);
            g.z[0].enc[0] = Enc {
                kind: sc.hab,
                size: 4,
                used: 0,
            };
            g.z[0].animals = vec![
                bank()
                    .iter()
                    .find(|c| !c.sponsor && c.tag == sc.tag && c.id != sid)
                    .unwrap()
                    .id,
                bank()
                    .iter()
                    .filter(|c| !c.sponsor && c.tag == sc.tag && c.id != sid)
                    .nth(1)
                    .unwrap()
                    .id,
            ];
            g.z[0].hand = vec![sid];
            g.z[0].money = 20;
            g.apply(&Move::Animal(0, 0));
            assert_eq!(g.z[0].appeal, i32::from(sc.ap) + 2);
        }
    }

    #[test]
    fn game_end_gives_second_player_last_turn() {
        let mut g = fresh(14);
        g.z[0].appeal = TARGET;
        g.apply(&Move::Fund);
        assert!(g.trigger && !g.over);
        assert_eq!(g.to_move(), Player::Two);
        g.apply(&Move::Fund);
        assert!(g.over);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
        // dacă J2 declanșează, se termină imediat
        let mut g = fresh(15);
        g.apply(&Move::Fund);
        g.z[1].appeal = TARGET + 5;
        g.apply(&Move::Fund);
        assert!(g.over);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
    }

    #[test]
    fn random_games_finish_and_conserve_cards() {
        for seed in 1..40u64 {
            let mut g = fresh(seed);
            let mut rng = Rng::new(seed ^ 77);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                let mv = m[rng.below(m.len())];
                g.apply(&mv);
                assert_eq!(g.card_total(), 80);
                for p in 0..2 {
                    assert!(g.z[p].money >= 0);
                    for e in &g.z[p].enc {
                        assert!(e.used <= e.size);
                    }
                    assert!(g.z[p].hand.len() <= HAND_MAX);
                }
                n += 1;
                assert!(n < 400);
            }
        }
    }

    #[test]
    fn ai_beats_random() {
        let mut wins = 0;
        for seed in 1..=24u64 {
            let mut g = fresh(seed);
            let mut rng = Rng::new(seed + 5);
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
        assert!(wins >= 21, "wins {wins}");
    }

    #[test]
    fn ai_games_end_with_a_real_race() {
        let mut total = 0;
        for seed in 1..=10u64 {
            let mut g = fresh(seed);
            let mut rng = Rng::new(seed);
            while g.outcome().is_none() {
                let m = polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap();
                g.apply(&m);
            }
            total += g.turns[0];
            assert!(g.turns[0] >= 8, "prea scurt {}", g.turns[0]);
        }
        eprintln!("medie ture/jucător {}", total as f64 / 10.0);
    }

    #[test]
    fn render_hides_the_opponents_hand() {
        let mut g = fresh(16);
        let mecena = bank().iter().find(|c| c.name == "Mecena").unwrap().id;
        g.z[1].hand = vec![mecena];
        g.z[0].hand = vec![animal_idx(|_| true)];
        let shown = g.z[0].hand[0];
        let r = g.render();
        assert!(!r.contains("Mecena"));
        assert!(r.contains(card(shown).name));
    }

    #[test]
    fn ai_is_fast() {
        let mut g = fresh(17);
        let t = std::time::Instant::now();
        let mut rng = Rng::new(3);
        for _ in 0..30 {
            if g.outcome().is_some() {
                break;
            }
            let m = polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap();
            g.apply(&m);
        }
        assert!(t.elapsed().as_secs_f64() < 30.0);
    }
}
