//! Motor de cărți colecționabile „evoluție + energie + bancă + premii”
//! (id `tcg_evo`), 2 jucători, cu un set ORIGINAL de cărți („Seria Făpturilor
//! de Câmp”). Reguli scrise de noi, din mecanici generice.
//!
//! * Pachet pregătit de 30 de cărți (J1 „Pădurea”, J2 „Vulcanul”), 5 cărți în
//!   mână (se reamestecă automat până apare o făptură de bază), 3 cărți-premiu
//!   puse deoparte. Pregătire: `activ N` (făptura activă), `banca N`, `gata`.
//! * Tura: `pesca` (forțat; pachet gol = pierdere), apoi oricâte dintre:
//!   `banca N`, `evolueaza N T` (nu în prima tură a jucătorului și nu pe o
//!   făptură nouă/evoluată în tura asta), `energie N T` (una pe tură),
//!   `joaca N [T]` (ajutoare; un singur „sprijin” pe tură), `retragere bK`
//!   (o dată pe tură; se aruncă energii cât costul de retragere), apoi
//!   `atac 1|2` (încheie tura) sau `fin`. Jucătorul 1 nu atacă în tura 1.
//! * Ținte: `a` (activa mea), `b1..b5` (banca mea), `e1..e5` (banca adversă).
//! * Slăbiciune: daunele se dublează dacă elementul atacatorului este cel de
//!   care e slabă ținta (Foc>Frunză, Apă>Foc, Fulger>Apă, Piatră>Fulger și
//!   Obișnuit, Frunză>Piatră). Efecte: vindecare, daune la bancă, reculul,
//!   aruncare de energie, pescuire, „Amețit” (nu poate ataca în tura următoare).
//! * Făptura activă la zero viață e eliminată: adversarul ia o carte-premiu;
//!   cel eliminat alege `promova bK`. Câștigă cine ia toate premiile, cine
//!   rămâne cu adversarul fără făpturi, sau cine nu rămâne fără pachet.
//!
//! Informația ascunsă: `render()` arată doar mâna J1. AI-ul (AlphaBeta(1)) are
//! `evaluate` care citește doar informația vizibilă (câmp public, mâna proprie,
//! NUMĂRUL cărților adverse); atacurile disponibile sunt creditate ca potențial.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const ELEM: [&str; 6] = ["Foc", "Apă", "Frunză", "Fulger", "Piatră", "Obișnuit"];
/// Elementul de care e slab fiecare element.
const WEAK: [u8; 6] = [1, 3, 0, 4, 2, 4];
const PRIZES: usize = 3;
const MAXBENCH: usize = 5;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Fx {
    None,
    Heal(i16),
    Bench(i16),
    Recoil(i16),
    Discard(u8),
    Draw(u8),
    Stun,
}

#[derive(Clone, Copy, Debug)]
pub struct Atk {
    pub name: &'static str,
    /// Energii de elementul făpturii.
    pub typed: u8,
    /// Energii oarecare în plus.
    pub any: u8,
    pub dmg: i16,
    pub fx: Fx,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Item {
    Heal(i16),
    Switch,
    Draw(u8),
    Recover,
    Gust,
    Boost(i16),
    HealAll(i16),
    Wise(u8),
}

impl Item {
    pub fn is_support(self) -> bool {
        matches!(self, Item::Boost(_) | Item::HealAll(_) | Item::Wise(_))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Cre {
    pub stage: u8,
    pub from: &'static str,
    pub el: u8,
    pub hp: i16,
    pub retreat: u8,
    pub a: [Option<Atk>; 2],
}

#[derive(Clone, Copy, Debug)]
pub enum K {
    Cre(Cre),
    Energy(u8),
    Trainer(Item),
}

pub struct Card {
    pub name: &'static str,
    pub k: K,
}

const fn atk(name: &'static str, typed: u8, any: u8, dmg: i16, fx: Fx) -> Option<Atk> {
    Some(Atk {
        name,
        typed,
        any,
        dmg,
        fx,
    })
}
#[allow(clippy::too_many_arguments)]
const fn cre(
    name: &'static str,
    stage: u8,
    from: &'static str,
    el: u8,
    hp: i16,
    retreat: u8,
    a1: Option<Atk>,
    a2: Option<Atk>,
) -> Card {
    Card {
        name,
        k: K::Cre(Cre {
            stage,
            from,
            el,
            hp,
            retreat,
            a: [a1, a2],
        }),
    }
}
const fn en(name: &'static str, el: u8) -> Card {
    Card {
        name,
        k: K::Energy(el),
    }
}
const fn tr(name: &'static str, it: Item) -> Card {
    Card {
        name,
        k: K::Trainer(it),
    }
}

use Fx::None as N;

/// Setul complet (peste 40 de cărți originale).
pub const CARDS: &[Card] = &[
    // --- Foc
    cre(
        "Scânteiuță",
        0,
        "",
        0,
        40,
        1,
        atk("Pârâit", 1, 0, 10, N),
        atk("Scânteie", 1, 1, 20, N),
    ),
    cre(
        "Flăcărel",
        1,
        "Scânteiuță",
        0,
        70,
        1,
        atk("Limbă de foc", 1, 1, 30, N),
        atk("Val de jar", 2, 1, 50, Fx::Discard(1)),
    ),
    cre(
        "Vulcanar",
        2,
        "Flăcărel",
        0,
        120,
        3,
        atk("Erupție", 2, 1, 70, N),
        atk("Furie de lavă", 3, 1, 110, Fx::Discard(2)),
    ),
    // --- Apă
    cre(
        "Picurel",
        0,
        "",
        1,
        40,
        1,
        atk("Stropire", 1, 0, 10, N),
        atk("Bulă", 1, 1, 20, N),
    ),
    cre(
        "Râuleț",
        1,
        "Picurel",
        1,
        70,
        1,
        atk("Curent", 1, 1, 30, N),
        atk("Val", 2, 1, 50, N),
    ),
    cre(
        "Cascador",
        2,
        "Râuleț",
        1,
        120,
        3,
        atk("Torent", 2, 1, 70, Fx::Bench(10)),
        atk("Potop", 3, 1, 100, Fx::Heal(20)),
    ),
    // --- Frunză
    cre(
        "Mugurel",
        0,
        "",
        2,
        40,
        1,
        atk("Înmugurire", 1, 0, 10, Fx::Heal(10)),
        atk("Bici de vrej", 1, 1, 20, N),
    ),
    cre(
        "Frunzar",
        1,
        "Mugurel",
        2,
        80,
        1,
        atk("Spini", 1, 1, 30, N),
        atk("Rădăcini", 2, 1, 40, Fx::Heal(20)),
    ),
    cre(
        "Pădurar",
        2,
        "Frunzar",
        2,
        130,
        3,
        atk("Dărâmă-copac", 2, 1, 70, N),
        atk("Furtuna codrului", 3, 1, 100, Fx::Stun),
    ),
    // --- Fulger
    cre(
        "Zumzet",
        0,
        "",
        3,
        40,
        1,
        atk("Zbârnâit", 1, 0, 10, N),
        atk("Șoc", 1, 1, 20, N),
    ),
    cre(
        "Fulgerel",
        1,
        "Zumzet",
        3,
        70,
        1,
        atk("Descărcare", 1, 1, 30, N),
        atk("Arc electric", 2, 0, 50, Fx::Recoil(10)),
    ),
    cre(
        "Tunetar",
        2,
        "Fulgerel",
        3,
        110,
        2,
        atk("Tunet", 2, 1, 70, Fx::Stun),
        atk("Trăsnet", 3, 0, 120, Fx::Discard(2)),
    ),
    // --- Piatră
    cre(
        "Pietricel",
        0,
        "",
        4,
        50,
        2,
        atk("Rostogolire", 1, 0, 10, N),
        atk("Lovitură", 1, 1, 20, N),
    ),
    cre(
        "Stâncuț",
        1,
        "Pietricel",
        4,
        80,
        2,
        atk("Avalanșă mică", 1, 1, 30, N),
        atk("Zid de piatră", 2, 0, 40, Fx::Heal(20)),
    ),
    cre(
        "Munteanul",
        2,
        "Stâncuț",
        4,
        140,
        3,
        atk("Prăbușire", 2, 1, 80, N),
        atk("Cutremur", 3, 1, 90, Fx::Bench(20)),
    ),
    // --- Obișnuite și extra
    cre(
        "Vrabie de câmp",
        0,
        "",
        5,
        40,
        0,
        atk("Ciugulit", 0, 1, 10, N),
        atk("Zbor în picaj", 0, 2, 30, N),
    ),
    cre(
        "Bufnița Înțeleaptă",
        1,
        "Vrabie de câmp",
        5,
        60,
        1,
        atk("Ochi de noapte", 0, 2, 30, Fx::Draw(1)),
        atk("Ghearele nopții", 0, 3, 60, N),
    ),
    cre(
        "Bursuc de pădure",
        0,
        "",
        5,
        70,
        2,
        atk("Zgârietură", 0, 1, 20, N),
        atk("Săpătură", 0, 3, 60, N),
    ),
    cre(
        "Iepure Săltăreț",
        0,
        "",
        5,
        30,
        0,
        atk("Salt", 0, 1, 10, Fx::Draw(1)),
        None,
    ),
    cre(
        "Broscuță de baltă",
        0,
        "",
        1,
        50,
        1,
        atk("Săritură", 1, 0, 20, N),
        atk("Limbă lipicioasă", 1, 2, 40, Fx::Stun),
    ),
    cre(
        "Cârtiță vrednică",
        0,
        "",
        4,
        50,
        1,
        atk("Săpat", 1, 0, 10, Fx::Draw(1)),
        atk("Gheare", 1, 1, 30, N),
    ),
    cre(
        "Veveriță",
        0,
        "",
        2,
        40,
        0,
        atk("Alune", 1, 0, 10, Fx::Heal(10)),
        atk("Roade", 1, 1, 20, N),
    ),
    cre(
        "Licurici de seară",
        0,
        "",
        3,
        30,
        0,
        atk("Lumină", 1, 0, 10, N),
        atk("Fulger mic", 1, 1, 20, Fx::Bench(10)),
    ),
    cre(
        "Vulpea de Jar",
        1,
        "Iepure Săltăreț",
        0,
        60,
        1,
        atk("Șiretlic", 1, 1, 30, Fx::Draw(1)),
        atk("Mușcătură de jar", 2, 1, 60, N),
    ),
    cre(
        "Lăstar de Fulger",
        1,
        "Licurici de seară",
        3,
        60,
        1,
        atk("Sclipire", 1, 1, 30, N),
        atk("Furtunică", 2, 1, 50, Fx::Bench(10)),
    ),
    // --- Energii
    en("Energie Foc", 0),
    en("Energie Apă", 1),
    en("Energie Frunză", 2),
    en("Energie Fulger", 3),
    en("Energie Piatră", 4),
    // --- Ajutoare
    tr("Poțiune de ierburi", Item::Heal(30)),
    tr("Poțiune mare", Item::Heal(60)),
    tr("Schimb de loc", Item::Switch),
    tr("Căutare", Item::Draw(2)),
    tr("Căutare adâncă", Item::Draw(3)),
    tr("Recuperare", Item::Recover),
    tr("Momeală", Item::Gust),
    tr("Antrenorul Mihu", Item::Boost(20)),
    tr("Antrenoarea Ioana", Item::Boost(30)),
    tr("Sprijin de tabără", Item::HealAll(20)),
    tr("Cărturăreasa", Item::Wise(3)),
    tr("Bătrâna Vrăjitoare", Item::Wise(2)),
];

pub fn card_id(name: &str) -> u8 {
    CARDS
        .iter()
        .position(|c| c.name == name)
        .unwrap_or_else(|| panic!("carte necunoscută: {name}")) as u8
}

fn cre_of(id: u8) -> &'static Cre {
    match &CARDS[id as usize].k {
        K::Cre(c) => c,
        _ => panic!("nu e făptură"),
    }
}

fn energy_el(id: u8) -> Option<u8> {
    match CARDS[id as usize].k {
        K::Energy(e) => Some(e),
        _ => Option::None,
    }
}

const DECK_PADUREA: &[(&str, u8)] = &[
    ("Mugurel", 3),
    ("Frunzar", 2),
    ("Pădurar", 2),
    ("Pietricel", 2),
    ("Stâncuț", 2),
    ("Munteanul", 1),
    ("Veveriță", 1),
    ("Cârtiță vrednică", 1),
    ("Energie Frunză", 7),
    ("Energie Piatră", 5),
    ("Poțiune de ierburi", 1),
    ("Căutare", 1),
    ("Cărturăreasa", 1),
    ("Sprijin de tabără", 1),
];

const DECK_VULCANUL: &[(&str, u8)] = &[
    ("Scânteiuță", 3),
    ("Flăcărel", 2),
    ("Vulcanar", 2),
    ("Zumzet", 3),
    ("Fulgerel", 2),
    ("Tunetar", 1),
    ("Licurici de seară", 1),
    ("Vrabie de câmp", 1),
    ("Energie Foc", 6),
    ("Energie Fulger", 5),
    ("Poțiune de ierburi", 1),
    ("Căutare", 1),
    ("Antrenorul Mihu", 1),
    ("Schimb de loc", 1),
];

pub fn deck_list(p: Player) -> Vec<u8> {
    let src = if p == Player::One {
        DECK_PADUREA
    } else {
        DECK_VULCANUL
    };
    let mut v = Vec::new();
    for (n, k) in src {
        for _ in 0..*k {
            v.push(card_id(n));
        }
    }
    v
}

pub fn deck_name(p: Player) -> &'static str {
    if p == Player::One {
        "Pădurea"
    } else {
        "Vulcanul"
    }
}

fn fx_text(f: Fx) -> String {
    match f {
        Fx::None => String::new(),
        Fx::Heal(n) => format!("vindeci {n}"),
        Fx::Bench(n) => format!("{n} daune fiecărei făpturi de pe banca adversă"),
        Fx::Recoil(n) => format!("recul {n}"),
        Fx::Discard(n) => format!("arunci {n} energii"),
        Fx::Draw(n) => format!("pescuiești {n}"),
        Fx::Stun => "ținta e amețită".into(),
    }
}

fn item_text(i: Item) -> String {
    match i {
        Item::Heal(n) => format!("vindeci {n} unei făpturi"),
        Item::Switch => "schimbi activa cu o făptură de pe bancă".into(),
        Item::Draw(n) => format!("pescuiești {n}"),
        Item::Recover => "iei în mână ultima energie din aruncate".into(),
        Item::Gust => "aduci o făptură de pe banca adversă în locul activei ei".into(),
        Item::Boost(n) => format!("sprijin: atacurile tale +{n} daune în tura asta"),
        Item::HealAll(n) => format!("sprijin: vindeci {n} fiecărei făpturi"),
        Item::Wise(n) => format!("sprijin: pescuiești {n}"),
    }
}

pub fn card_line(id: u8) -> String {
    let c = &CARDS[id as usize];
    match &c.k {
        K::Energy(_) => format!("{} (energie)", c.name),
        K::Trainer(i) => format!("{} ({})", c.name, item_text(*i)),
        K::Cre(x) => {
            let mut s = format!(
                "{} ({}, {}, {} viață, retragere {}",
                c.name,
                ELEM[x.el as usize],
                if x.stage == 0 {
                    "de bază".to_string()
                } else {
                    format!("evoluție {} din {}", x.stage, x.from)
                },
                x.hp,
                x.retreat
            );
            for a in x.a.iter().flatten() {
                let mut cost = String::new();
                for _ in 0..a.typed {
                    cost.push('E');
                }
                for _ in 0..a.any {
                    cost.push('o');
                }
                s.push_str(&format!("; {} [{}] {}", a.name, cost, a.dmg));
                let t = fx_text(a.fx);
                if !t.is_empty() {
                    s.push_str(&format!(" ({t})"));
                }
            }
            s.push(')');
            s
        }
    }
}

#[derive(Clone, Debug)]
pub struct Mon {
    /// Cărțile de făptură, de jos în sus (ultima = forma curentă).
    pub stack: Vec<u8>,
    pub en: Vec<u8>,
    pub dmg: i16,
    pub fresh: bool,
    pub stun: bool,
}

impl Mon {
    fn new(card: u8) -> Mon {
        Mon {
            stack: vec![card],
            en: Vec::new(),
            dmg: 0,
            fresh: true,
            stun: false,
        }
    }
    pub fn top(&self) -> u8 {
        *self.stack.last().unwrap()
    }
    pub fn cre(&self) -> &'static Cre {
        cre_of(self.top())
    }
    fn dead(&self) -> bool {
        self.dmg >= self.cre().hp
    }
    fn can_pay(&self, a: &Atk) -> bool {
        let el = self.cre().el;
        let typed = self
            .en
            .iter()
            .filter(|e| energy_el(**e) == Some(el))
            .count() as u8;
        typed >= a.typed && self.en.len() as u8 >= a.typed + a.any
    }
}

#[derive(Clone, Debug, Default)]
pub struct Side {
    pub deck: Vec<u8>,
    pub hand: Vec<u8>,
    pub prizes: Vec<u8>,
    pub active: Option<Mon>,
    pub bench: Vec<Mon>,
    pub grave: Vec<u8>,
    pub support_used: bool,
    pub energy_used: bool,
    pub retreated: bool,
    pub boost: i16,
    pub turns: u32,
}

impl Side {
    pub fn total_cards(&self) -> usize {
        let m = |x: &Mon| x.stack.len() + x.en.len();
        self.deck.len()
            + self.hand.len()
            + self.prizes.len()
            + self.grave.len()
            + self.active.as_ref().map_or(0, m)
            + self.bench.iter().map(m).sum::<usize>()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tgt {
    None,
    /// 0 = activa, k = banca k-1.
    Own(u8),
    /// Banca adversă, indice de la 0.
    Foe(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Draw,
    Active(u8),
    Bench(u8),
    Ready,
    Evolve(u8, u8),
    Attach(u8, u8),
    Play(u8, Tgt),
    Retreat(u8),
    Attack(u8),
    End,
    Promote(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Phase {
    Setup,
    Draw,
    Main,
    Promote,
}

#[derive(Clone)]
pub struct TcgEvo {
    pub sides: [Side; 2],
    pub turn: Player,
    pub phase: Phase,
    pub turn_no: u32,
    pend: Vec<Player>,
    next: Player,
    over: Option<Outcome>,
    /// `Some(p)` = render shows `p`'s hand; `None` = hot-seat (the mover's).
    viewer: Option<Player>,
}

impl Default for TcgEvo {
    fn default() -> Self {
        Self::new()
    }
}

fn shuffle(v: &mut [u8], rng: &mut Rng) {
    for i in (1..v.len()).rev() {
        v.swap(i, rng.below(i + 1));
    }
}

fn is_basic(id: u8) -> bool {
    matches!(CARDS[id as usize].k, K::Cre(c) if c.stage == 0)
}

impl TcgEvo {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut sides: [Side; 2] = [Side::default(), Side::default()];
        for p in [Player::One, Player::Two] {
            let mut d = deck_list(p);
            loop {
                shuffle(&mut d, &mut rng);
                if d[d.len() - 5..].iter().any(|c| is_basic(*c)) {
                    break;
                }
            }
            let s = &mut sides[p.idx()];
            for _ in 0..5 {
                s.hand.push(d.pop().unwrap());
            }
            for _ in 0..PRIZES {
                s.prizes.push(d.pop().unwrap());
            }
            s.deck = d;
        }
        TcgEvo {
            sides,
            turn: Player::One,
            phase: Phase::Setup,
            turn_no: 0,
            pend: Vec::new(),
            next: Player::One,
            over: Option::None,
            viewer: Some(Player::One),
        }
    }

    pub fn side(&self, p: Player) -> &Side {
        &self.sides[p.idx()]
    }

    fn start_turn(&mut self, p: Player) {
        self.turn = p;
        self.turn_no += 1;
        self.phase = Phase::Draw;
        let s = &mut self.sides[p.idx()];
        s.turns += 1;
        s.support_used = false;
        s.energy_used = false;
        s.retreated = false;
        s.boost = 0;
    }

    fn end_turn(&mut self) {
        let p = self.turn;
        let s = &mut self.sides[p.idx()];
        s.boost = 0;
        for m in s.active.iter_mut().chain(s.bench.iter_mut()) {
            m.fresh = false;
            m.stun = false;
        }
        self.start_turn(p.other());
    }

    fn draw_card(&mut self, p: Player) -> bool {
        let s = &mut self.sides[p.idx()];
        match s.deck.pop() {
            Option::None => false,
            Some(c) => {
                s.hand.push(c);
                true
            }
        }
    }

    fn mon(&self, p: Player, t: u8) -> Option<&Mon> {
        let s = &self.sides[p.idx()];
        if t == 0 {
            s.active.as_ref()
        } else {
            s.bench.get(t as usize - 1)
        }
    }

    fn mon_mut(&mut self, p: Player, t: u8) -> Option<&mut Mon> {
        let s = &mut self.sides[p.idx()];
        if t == 0 {
            s.active.as_mut()
        } else {
            s.bench.get_mut(t as usize - 1)
        }
    }

    fn own_targets(&self, p: Player) -> Vec<u8> {
        let s = &self.sides[p.idx()];
        let mut v = Vec::new();
        if s.active.is_some() {
            v.push(0);
        }
        for i in 0..s.bench.len() {
            v.push(i as u8 + 1);
        }
        v
    }

    fn item_targets(&self, p: Player, it: Item) -> Vec<Tgt> {
        let s = &self.sides[p.idx()];
        let f = &self.sides[p.other().idx()];
        match it {
            Item::Heal(_) => self
                .own_targets(p)
                .into_iter()
                .filter(|t| self.mon(p, *t).is_some_and(|m| m.dmg > 0))
                .map(Tgt::Own)
                .collect(),
            Item::Switch => {
                if s.active.is_some() {
                    (1..=s.bench.len() as u8).map(Tgt::Own).collect()
                } else {
                    Vec::new()
                }
            }
            Item::Gust => {
                if f.active.is_some() {
                    (0..f.bench.len() as u8).map(Tgt::Foe).collect()
                } else {
                    Vec::new()
                }
            }
            Item::Recover => {
                if s.grave.iter().any(|c| energy_el(*c).is_some()) {
                    vec![Tgt::None]
                } else {
                    Vec::new()
                }
            }
            Item::HealAll(_) => {
                if self
                    .own_targets(p)
                    .iter()
                    .any(|t| self.mon(p, *t).is_some_and(|m| m.dmg > 0))
                {
                    vec![Tgt::None]
                } else {
                    Vec::new()
                }
            }
            _ => vec![Tgt::None],
        }
    }

    /// Elimină făpturile căzute; stabilește premiile, înlocuirile și câștigătorul.
    fn resolve_kos(&mut self, actor: Player) {
        let order = [actor.other(), actor];
        let mut take = [0usize; 2];
        let mut lost = [false; 2];
        let mut pend = Vec::new();
        for who in order {
            let opp = who.other();
            let s = &mut self.sides[who.idx()];
            let mut i = 0;
            while i < s.bench.len() {
                if s.bench[i].dead() {
                    let m = s.bench.remove(i);
                    s.grave.extend(m.stack);
                    s.grave.extend(m.en);
                    take[opp.idx()] += 1;
                } else {
                    i += 1;
                }
            }
            if s.active.as_ref().is_some_and(|m| m.dead()) {
                let m = s.active.take().unwrap();
                s.grave.extend(m.stack);
                s.grave.extend(m.en);
                take[opp.idx()] += 1;
            }
            if s.active.is_none() {
                if s.bench.is_empty() {
                    lost[who.idx()] = true;
                } else {
                    pend.push(who);
                }
            }
        }
        for p in [Player::One, Player::Two] {
            for _ in 0..take[p.idx()] {
                let s = &mut self.sides[p.idx()];
                if let Some(c) = s.prizes.pop() {
                    s.hand.push(c);
                }
            }
        }
        let win =
            |g: &TcgEvo, p: Player| g.sides[p.idx()].prizes.is_empty() || lost[p.other().idx()];
        let w1 = win(self, Player::One);
        let w2 = win(self, Player::Two);
        self.over = match (w1, w2) {
            (true, true) => Some(Outcome::Draw),
            (true, false) => Some(Outcome::Win(Player::One)),
            (false, true) => Some(Outcome::Win(Player::Two)),
            _ => Option::None,
        };
        if self.over.is_none() && !pend.is_empty() {
            self.next = actor.other();
            self.turn = pend[0];
            self.pend = pend;
            self.phase = Phase::Promote;
        }
    }

    fn do_attack(&mut self, which: u8) {
        let p = self.turn;
        let q = p.other();
        let (a, el, boost) = {
            let m = self.sides[p.idx()].active.as_ref().unwrap();
            let c = m.cre();
            (
                c.a[which as usize].unwrap(),
                c.el,
                self.sides[p.idx()].boost,
            )
        };
        let mut d = a.dmg;
        if d > 0 {
            d += boost;
        }
        if let Some(t) = self.sides[q.idx()].active.as_mut() {
            if WEAK[t.cre().el as usize] == el {
                d *= 2;
            }
            t.dmg += d;
            if a.fx == Fx::Stun {
                t.stun = true;
            }
        }
        match a.fx {
            Fx::Heal(n) => {
                let m = self.sides[p.idx()].active.as_mut().unwrap();
                m.dmg = (m.dmg - n).max(0);
            }
            Fx::Bench(n) => {
                for m in &mut self.sides[q.idx()].bench {
                    m.dmg += n;
                }
            }
            Fx::Recoil(n) => self.sides[p.idx()].active.as_mut().unwrap().dmg += n,
            Fx::Discard(n) => {
                let s = &mut self.sides[p.idx()];
                let m = s.active.as_mut().unwrap();
                for _ in 0..n {
                    if let Some(e) = m.en.pop() {
                        s.grave.push(e);
                    }
                }
            }
            Fx::Draw(n) => {
                for _ in 0..n {
                    self.draw_card(p);
                }
            }
            _ => {}
        }
        self.resolve_kos(p);
        if self.over.is_none() && self.phase != Phase::Promote {
            self.end_turn();
        }
    }

    fn mon_val(m: &Mon, active: bool) -> i32 {
        let c = m.cre();
        let rem = (c.hp - m.dmg).max(0) as i32;
        let mut v = rem * 3 / 10 + c.hp as i32 / 10 + c.stage as i32 * 6 + m.en.len() as i32 * 9;
        if active {
            for a in c.a.iter().flatten() {
                if m.can_pay(a) {
                    v += 6;
                    break;
                }
            }
        }
        v
    }

    fn side_val(&self, p: Player) -> i32 {
        let s = &self.sides[p.idx()];
        let mut v = 100 * (PRIZES - s.prizes.len().min(PRIZES)) as i32 + 3 * s.hand.len() as i32;
        match &s.active {
            Some(m) => v += Self::mon_val(m, true),
            Option::None => v -= 200,
        }
        v += s.bench.iter().map(|m| Self::mon_val(m, false)).sum::<i32>();
        v
    }

    fn base_diff(&self, p: Player) -> i32 {
        self.side_val(p) - self.side_val(p.other())
    }
}

impl Game for TcgEvo {
    type Move = Move;

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }

    fn to_move(&self) -> Player {
        self.turn
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over.is_some() {
            return Vec::new();
        }
        let p = self.turn;
        let s = &self.sides[p.idx()];
        match self.phase {
            Phase::Draw => vec![Move::Draw],
            Phase::Promote => (0..s.bench.len() as u8).map(Move::Promote).collect(),
            Phase::Setup => {
                let mut v = Vec::new();
                for (h, id) in s.hand.iter().enumerate() {
                    if is_basic(*id) {
                        if s.active.is_none() {
                            v.push(Move::Active(h as u8));
                        } else if s.bench.len() < MAXBENCH {
                            v.push(Move::Bench(h as u8));
                        }
                    }
                }
                if s.active.is_some() {
                    v.push(Move::Ready);
                }
                v
            }
            Phase::Main => {
                let mut v = vec![Move::End];
                let tg = self.own_targets(p);
                for (h, id) in s.hand.iter().enumerate() {
                    let h = h as u8;
                    match &CARDS[*id as usize].k {
                        K::Cre(c) if c.stage == 0 => {
                            if s.bench.len() < MAXBENCH {
                                v.push(Move::Bench(h));
                            }
                        }
                        K::Cre(c) => {
                            if s.turns >= 2 {
                                for t in &tg {
                                    let m = self.mon(p, *t).unwrap();
                                    if !m.fresh && CARDS[m.top() as usize].name == c.from {
                                        v.push(Move::Evolve(h, *t));
                                    }
                                }
                            }
                        }
                        K::Energy(_) => {
                            if !s.energy_used {
                                for t in &tg {
                                    v.push(Move::Attach(h, *t));
                                }
                            }
                        }
                        K::Trainer(it) => {
                            if it.is_support() && s.support_used {
                                continue;
                            }
                            for t in self.item_targets(p, *it) {
                                v.push(Move::Play(h, t));
                            }
                        }
                    }
                }
                if let Some(a) = &s.active {
                    if !s.retreated && !s.bench.is_empty() && a.en.len() as u8 >= a.cre().retreat {
                        for i in 0..s.bench.len() as u8 {
                            v.push(Move::Retreat(i));
                        }
                    }
                    if self.turn_no > 1 && !a.stun {
                        for (k, at) in a.cre().a.iter().enumerate() {
                            if at.as_ref().is_some_and(|x| a.can_pay(x)) {
                                v.push(Move::Attack(k as u8));
                            }
                        }
                    }
                }
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        if self.over.is_some() {
            return;
        }
        let p = self.turn;
        match *m {
            Move::Draw => {
                if !self.draw_card(p) {
                    self.over = Some(Outcome::Win(p.other()));
                    return;
                }
                self.phase = Phase::Main;
            }
            Move::Active(h) => {
                let id = self.sides[p.idx()].hand.remove(h as usize);
                self.sides[p.idx()].active = Some(Mon::new(id));
            }
            Move::Bench(h) => {
                let id = self.sides[p.idx()].hand.remove(h as usize);
                self.sides[p.idx()].bench.push(Mon::new(id));
            }
            Move::Ready => {
                if p == Player::One {
                    self.turn = Player::Two;
                } else {
                    for s in &mut self.sides {
                        for m in s.active.iter_mut().chain(s.bench.iter_mut()) {
                            m.fresh = false;
                        }
                    }
                    self.start_turn(Player::One);
                }
            }
            Move::Evolve(h, t) => {
                let id = self.sides[p.idx()].hand.remove(h as usize);
                let mo = self.mon_mut(p, t).unwrap();
                mo.stack.push(id);
                mo.fresh = true;
            }
            Move::Attach(h, t) => {
                let id = self.sides[p.idx()].hand.remove(h as usize);
                self.mon_mut(p, t).unwrap().en.push(id);
                self.sides[p.idx()].energy_used = true;
            }
            Move::Play(h, t) => {
                let id = self.sides[p.idx()].hand.remove(h as usize);
                let K::Trainer(it) = CARDS[id as usize].k else {
                    panic!("nu e ajutor")
                };
                self.sides[p.idx()].grave.push(id);
                if it.is_support() {
                    self.sides[p.idx()].support_used = true;
                }
                match (it, t) {
                    (Item::Heal(n), Tgt::Own(i)) => {
                        let m = self.mon_mut(p, i).unwrap();
                        m.dmg = (m.dmg - n).max(0);
                    }
                    (Item::HealAll(n), _) => {
                        let s = &mut self.sides[p.idx()];
                        for m in s.active.iter_mut().chain(s.bench.iter_mut()) {
                            m.dmg = (m.dmg - n).max(0);
                        }
                    }
                    (Item::Switch, Tgt::Own(i)) => {
                        let s = &mut self.sides[p.idx()];
                        let nb = s.bench.remove(i as usize - 1);
                        let old = s.active.replace(nb).unwrap();
                        s.bench.insert(i as usize - 1, old);
                    }
                    (Item::Gust, Tgt::Foe(i)) => {
                        let s = &mut self.sides[p.other().idx()];
                        let nb = s.bench.remove(i as usize);
                        let old = s.active.replace(nb).unwrap();
                        s.bench.insert(i as usize, old);
                    }
                    (Item::Draw(n), _) | (Item::Wise(n), _) => {
                        for _ in 0..n {
                            self.draw_card(p);
                        }
                    }
                    (Item::Recover, _) => {
                        let s = &mut self.sides[p.idx()];
                        if let Some(i) = s.grave.iter().rposition(|c| energy_el(*c).is_some()) {
                            let c = s.grave.remove(i);
                            s.hand.push(c);
                        }
                    }
                    (Item::Boost(n), _) => self.sides[p.idx()].boost += n,
                    _ => {}
                }
            }
            Move::Retreat(i) => {
                let s = &mut self.sides[p.idx()];
                let cost = s.active.as_ref().unwrap().cre().retreat;
                for _ in 0..cost {
                    let e = s.active.as_mut().unwrap().en.pop().unwrap();
                    s.grave.push(e);
                }
                let nb = s.bench.remove(i as usize);
                let old = s.active.replace(nb).unwrap();
                s.bench.push(old);
                s.retreated = true;
            }
            Move::Attack(k) => self.do_attack(k),
            Move::End => self.end_turn(),
            Move::Promote(i) => {
                let s = &mut self.sides[p.idx()];
                let nb = s.bench.remove(i as usize);
                s.active = Some(nb);
                self.pend.remove(0);
                if let Some(n) = self.pend.first() {
                    self.turn = *n;
                } else {
                    let nx = self.next;
                    self.start_turn(nx);
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.over
    }

    fn evaluate(&self, p: Player) -> i32 {
        let mut v = self.base_diff(p);
        // Potențial: cel mai bun atac al jucătorului la mutare (în fază principală),
        // calculat doar din informația vizibilă a stării.
        if self.phase == Phase::Main && self.over.is_none() {
            let mover = self.turn;
            let before = self.base_diff(mover);
            let mut best = 0;
            for m in self.legal_moves() {
                if let Move::Attack(_) = m {
                    let mut c = self.clone();
                    c.apply(&m);
                    best = best.max(c.base_diff(mover) - before);
                }
            }
            if mover == p {
                v += best;
            } else {
                v -= best;
            }
        }
        v
    }

    fn render(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            "Făpturi de Câmp (evoluție) — tura {} — {}\n",
            self.turn_no,
            match self.phase {
                Phase::Setup => "pregătire",
                Phase::Draw => "pescuire",
                Phase::Main => "faza principală",
                Phase::Promote => "alege activa nouă",
            }
        ));
        let mon_line = |m: &Mon| {
            let c = m.cre();
            let mut e = [0u8; 5];
            for x in &m.en {
                if let Some(el) = energy_el(*x) {
                    e[el as usize] += 1;
                }
            }
            let en: Vec<String> = (0..5)
                .filter(|i| e[*i] > 0)
                .map(|i| format!("{}x{}", e[i], ELEM[i]))
                .collect();
            format!(
                "{} {}/{}{}{}{}",
                CARDS[m.top() as usize].name,
                c.hp - m.dmg,
                c.hp,
                if en.is_empty() {
                    String::new()
                } else {
                    format!(" [{}]", en.join(" "))
                },
                if m.stun { " [amețit]" } else { "" },
                if m.fresh { " [nou]" } else { "" }
            )
        };
        let v = self.viewer.unwrap_or(self.turn);
        for p in [v.other(), v] {
            let sd = &self.sides[p.idx()];
            let who = if p == Player::One { 1 } else { 2 };
            s.push_str(&format!(
                "Jucătorul {who} ({}): premii {} | mână {} | pachet {} | aruncate {}\n",
                deck_name(p),
                sd.prizes.len(),
                sd.hand.len(),
                sd.deck.len(),
                sd.grave.len()
            ));
            match &sd.active {
                Some(m) => s.push_str(&format!("  Activ: {}\n", mon_line(m))),
                Option::None => s.push_str("  Activ: —\n"),
            }
            for (i, m) in sd.bench.iter().enumerate() {
                let tag = if p == v { 'b' } else { 'e' };
                s.push_str(&format!("  {tag}{}: {}\n", i + 1, mon_line(m)));
            }
        }
        if self.over.is_some() {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        s.push_str(&format!("Mâna Jucătorului {}:\n", v.idx() + 1));
        for (i, id) in self.sides[v.idx()].hand.iter().enumerate() {
            s.push_str(&format!("  {}. {}\n", i + 1, card_line(*id)));
        }
        s.push_str(&format!(
            "La mutare: Jucătorul {}\n",
            if self.turn == Player::One { 1 } else { 2 }
        ));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        let own = |t: &u8| {
            if *t == 0 {
                "a".to_string()
            } else {
                format!("b{t}")
            }
        };
        match m {
            Move::Draw => "pesca".into(),
            Move::End => "fin".into(),
            Move::Ready => "gata".into(),
            Move::Active(h) => format!("activ {}", h + 1),
            Move::Bench(h) => format!("banca {}", h + 1),
            Move::Evolve(h, t) => format!("evolueaza {} {}", h + 1, own(t)),
            Move::Attach(h, t) => format!("energie {} {}", h + 1, own(t)),
            Move::Play(h, t) => match t {
                Tgt::None => format!("joaca {}", h + 1),
                Tgt::Own(i) => format!("joaca {} {}", h + 1, own(i)),
                Tgt::Foe(i) => format!("joaca {} e{}", h + 1, i + 1),
            },
            Move::Retreat(i) => format!("retragere b{}", i + 1),
            Move::Attack(k) => format!("atac {}", k + 1),
            Move::Promote(i) => format!("promova b{}", i + 1),
        }
    }

    /// Adâncime 0 (o singură mutare): `AlphaBeta(1)` ar fi extins la Hard/Expert pescuirea forțată (pachetul real)
    /// și răspunsul adversarului din mâna lui reală; evaluarea citește doar câmpul public, mâna proprie și
    /// numărul cărților adverse.
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn started(seed: u64) -> TcgEvo {
        let mut g = TcgEvo::with_seed(seed);
        for _ in 0..2 {
            let m = g.legal_moves()[0];
            g.apply(&m);
            g.apply(&Move::Ready);
        }
        g
    }

    fn put(g: &mut TcgEvo, p: Player, name: &str, bench: bool) -> usize {
        let mut m = Mon::new(card_id(name));
        m.fresh = false;
        let s = &mut g.sides[p.idx()];
        if bench {
            s.bench.push(m);
            s.bench.len()
        } else {
            s.active = Some(m);
            0
        }
    }

    fn main_phase(seed: u64) -> TcgEvo {
        let mut g = started(seed);
        g.apply(&Move::Draw);
        g
    }

    #[test]
    fn set_has_forty_plus_original_cards_and_decks_have_thirty() {
        assert!(CARDS.len() >= 40, "{}", CARDS.len());
        let mut names: Vec<&str> = CARDS.iter().map(|c| c.name).collect();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), CARDS.len());
        for p in [Player::One, Player::Two] {
            let d = deck_list(p);
            assert_eq!(d.len(), 30);
            for id in 0..CARDS.len() as u8 {
                let n = d.iter().filter(|x| **x == id).count();
                assert!(n <= 7);
                if !matches!(CARDS[id as usize].k, K::Energy(_)) {
                    assert!(n <= 4);
                }
            }
            // fiecare evoluție din pachet are baza în pachet
            for id in &d {
                if let K::Cre(c) = &CARDS[*id as usize].k {
                    if c.stage > 0 {
                        assert!(d.iter().any(|x| CARDS[*x as usize].name == c.from));
                    }
                }
            }
        }
        // toate evoluțiile din set au o bază existentă
        for c in CARDS {
            if let K::Cre(x) = &c.k {
                if x.stage > 0 {
                    assert!(CARDS.iter().any(|b| b.name == x.from), "{}", c.name);
                }
            }
        }
    }

    #[test]
    fn setup_deals_five_three_prizes_and_has_basic() {
        for seed in 1..30 {
            let g = TcgEvo::with_seed(seed);
            for s in &g.sides {
                assert_eq!(s.hand.len(), 5);
                assert_eq!(s.prizes.len(), 3);
                assert_eq!(s.deck.len(), 22);
                assert!(s.hand.iter().any(|c| is_basic(*c)));
            }
        }
    }

    #[test]
    fn setup_then_game_starts_with_player_one_drawing() {
        let g = started(3);
        assert_eq!(g.phase, Phase::Draw);
        assert_eq!(g.turn, Player::One);
        assert!(g.sides[0].active.is_some() && g.sides[1].active.is_some());
        assert_eq!(g.legal_moves(), vec![Move::Draw]);
    }

    #[test]
    fn one_energy_per_turn() {
        let mut g = main_phase(4);
        let e = card_id("Energie Foc");
        g.sides[0].hand = vec![e, e];
        put(&mut g, Player::One, "Scânteiuță", false);
        let n = g
            .legal_moves()
            .iter()
            .filter(|m| matches!(m, Move::Attach(..)))
            .count();
        assert_eq!(n, 2 * (1 + g.sides[0].bench.len()));
        g.apply(&Move::Attach(0, 0));
        assert!(!g
            .legal_moves()
            .iter()
            .any(|m| matches!(m, Move::Attach(..))));
    }

    #[test]
    fn evolution_rules() {
        let mut g = main_phase(5);
        put(&mut g, Player::One, "Scânteiuță", false);
        g.sides[0].hand = vec![card_id("Flăcărel"), card_id("Vulcanar")];
        // prima tură a jucătorului: nu se evoluează
        assert!(!g
            .legal_moves()
            .iter()
            .any(|m| matches!(m, Move::Evolve(..))));
        g.sides[0].turns = 2;
        let mv = g.legal_moves();
        assert!(mv.contains(&Move::Evolve(0, 0)));
        assert!(
            !mv.contains(&Move::Evolve(1, 0)),
            "stadiul 2 nu sare peste 1"
        );
        g.sides[0].active.as_mut().unwrap().dmg = 10;
        g.apply(&Move::Evolve(0, 0));
        let a = g.sides[0].active.as_ref().unwrap();
        assert_eq!(a.dmg, 10, "daunele rămân");
        assert_eq!(a.stack.len(), 2);
        // nu mai evoluează în aceeași tură
        assert!(!g
            .legal_moves()
            .iter()
            .any(|m| matches!(m, Move::Evolve(..))));
    }

    #[test]
    fn attack_needs_energy_and_first_player_cannot_attack_turn_one() {
        let mut g = main_phase(6);
        put(&mut g, Player::One, "Scânteiuță", false);
        g.sides[0].active.as_mut().unwrap().en = vec![card_id("Energie Foc")];
        assert!(!g.legal_moves().iter().any(|m| matches!(m, Move::Attack(_))));
        g.turn_no = 3;
        let mv = g.legal_moves();
        assert!(mv.contains(&Move::Attack(0)));
        assert!(!mv.contains(&Move::Attack(1)), "Scânteie cere 2 energii");
        g.sides[0].active.as_mut().unwrap().en = vec![card_id("Energie Apă"); 2];
        let mv = g.legal_moves();
        assert!(!mv.contains(&Move::Attack(0)), "energii de alt element");
    }

    #[test]
    fn weakness_doubles_damage_and_attack_ends_turn() {
        let mut g = main_phase(7);
        g.turn_no = 3;
        put(&mut g, Player::One, "Picurel", false); // Apă
        g.sides[0].active.as_mut().unwrap().en = vec![card_id("Energie Apă")];
        put(&mut g, Player::Two, "Scânteiuță", false); // Foc, slab la Apă
        g.apply(&Move::Attack(0));
        let d = g.sides[1].active.as_ref().unwrap();
        assert_eq!(d.dmg, 20);
        assert_eq!(g.turn, Player::Two);
        assert_eq!(g.phase, Phase::Draw);
    }

    #[test]
    fn ko_takes_prize_and_forces_promotion() {
        let mut g = main_phase(8);
        g.turn_no = 3;
        put(&mut g, Player::One, "Zumzet", false);
        g.sides[0].active.as_mut().unwrap().en = vec![card_id("Energie Fulger")];
        put(&mut g, Player::Two, "Scânteiuță", false);
        g.sides[1].active.as_mut().unwrap().dmg = 35;
        put(&mut g, Player::Two, "Picurel", true);
        let hand = g.sides[0].hand.len();
        g.apply(&Move::Attack(0));
        assert_eq!(g.sides[0].prizes.len(), 2);
        assert_eq!(g.sides[0].hand.len(), hand + 1);
        assert_eq!(g.phase, Phase::Promote);
        assert_eq!(g.turn, Player::Two);
        assert_eq!(g.legal_moves(), vec![Move::Promote(0)]);
        g.apply(&Move::Promote(0));
        assert_eq!(g.phase, Phase::Draw);
        assert_eq!(g.turn, Player::Two);
        assert_eq!(
            CARDS[g.sides[1].active.as_ref().unwrap().top() as usize].name,
            "Picurel"
        );
    }

    #[test]
    fn no_bench_ko_wins_and_third_prize_wins() {
        let mut g = main_phase(9);
        g.turn_no = 3;
        put(&mut g, Player::One, "Zumzet", false);
        g.sides[0].active.as_mut().unwrap().en = vec![card_id("Energie Fulger")];
        put(&mut g, Player::Two, "Scânteiuță", false);
        g.sides[1].active.as_mut().unwrap().dmg = 35;
        g.sides[1].bench.clear();
        g.apply(&Move::Attack(0));
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));

        let mut g = main_phase(9);
        g.turn_no = 3;
        put(&mut g, Player::One, "Zumzet", false);
        g.sides[0].active.as_mut().unwrap().en = vec![card_id("Energie Fulger")];
        put(&mut g, Player::Two, "Scânteiuță", false);
        put(&mut g, Player::Two, "Picurel", true);
        g.sides[1].active.as_mut().unwrap().dmg = 35;
        g.sides[0].prizes.truncate(1);
        g.apply(&Move::Attack(0));
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
        assert!(g.legal_moves().is_empty());
    }

    #[test]
    fn retreat_pays_energy_and_swaps() {
        let mut g = main_phase(10);
        put(&mut g, Player::One, "Flăcărel", false);
        put(&mut g, Player::One, "Picurel", true);
        let e = card_id("Energie Foc");
        g.sides[0].active.as_mut().unwrap().en = vec![e, e];
        g.apply(&Move::Retreat(0));
        let s = &g.sides[0];
        assert_eq!(
            CARDS[s.active.as_ref().unwrap().top() as usize].name,
            "Picurel"
        );
        assert_eq!(s.bench[s.bench.len() - 1].en.len(), 1);
        assert!(!g
            .legal_moves()
            .iter()
            .any(|m| matches!(m, Move::Retreat(_))));
    }

    #[test]
    fn stun_blocks_next_attack_and_clears() {
        let mut g = main_phase(11);
        g.turn_no = 3;
        put(&mut g, Player::One, "Broscuță de baltă", false);
        g.sides[0].active.as_mut().unwrap().en = vec![card_id("Energie Apă"); 3];
        put(&mut g, Player::Two, "Tunetar", false);
        g.sides[1].active.as_mut().unwrap().en = vec![card_id("Energie Fulger"); 2];
        g.apply(&Move::Attack(1));
        assert!(g.sides[1].active.as_ref().unwrap().stun);
        g.apply(&Move::Draw);
        assert!(!g.legal_moves().iter().any(|m| matches!(m, Move::Attack(_))));
        g.apply(&Move::End);
        assert!(!g.sides[1].active.as_ref().unwrap().stun);
    }

    #[test]
    fn trainers_work_and_one_support_per_turn() {
        let mut g = main_phase(12);
        put(&mut g, Player::One, "Scânteiuță", false);
        g.sides[0].active.as_mut().unwrap().dmg = 20;
        g.sides[0].hand = vec![
            card_id("Poțiune de ierburi"),
            card_id("Cărturăreasa"),
            card_id("Antrenorul Mihu"),
        ];
        let deck = g.sides[0].deck.len();
        g.apply(&Move::Play(0, Tgt::Own(0)));
        assert_eq!(g.sides[0].active.as_ref().unwrap().dmg, 0);
        g.apply(&Move::Play(0, Tgt::None));
        assert_eq!(g.sides[0].hand.len(), 1 + 3);
        assert_eq!(g.sides[0].deck.len(), deck - 3);
        assert!(!g
            .legal_moves()
            .iter()
            .any(|m| matches!(m, Move::Play(h, _) if CARDS[g.sides[0].hand[*h as usize] as usize].name == "Antrenorul Mihu")));
    }

    #[test]
    fn deck_out_loses() {
        let mut g = started(14);
        g.sides[0].deck.clear();
        g.apply(&Move::Draw);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
    }

    #[test]
    fn render_hides_opponent_hand() {
        let g = TcgEvo::with_seed(5);
        let r = g.render();
        assert!(r.contains("Mâna Jucătorului 1"));
        for l in g.sides[0].hand.iter().map(|i| card_line(*i)) {
            assert!(r.contains(l.as_str()));
        }
        for id in &g.sides[1].hand {
            if !g.sides[0].hand.contains(id) {
                assert!(!r.contains(card_line(*id).as_str()));
            }
        }
    }

    #[test]
    fn ai_evaluate_ignores_opponent_hand_contents() {
        let mut a = main_phase(21);
        let mut b = a.clone();
        let cards: Vec<u8> = (0..CARDS.len() as u8).collect();
        a.sides[1].hand = cards.iter().copied().take(5).collect();
        b.sides[1].hand = cards.iter().copied().skip(20).take(5).collect();
        assert_eq!(a.evaluate(Player::One), b.evaluate(Player::One));
        // nici ordinea pachetului propriu nu influențează evaluarea
        let mut c = a.clone();
        c.sides[0].deck.reverse();
        assert_eq!(a.evaluate(Player::One), c.evaluate(Player::One));
    }

    #[test]
    fn cards_are_conserved_over_random_games() {
        for seed in 1..=12u64 {
            let mut g = TcgEvo::with_seed(seed);
            let mut rng = Rng::new(seed * 7 + 1);
            let mut n = 0;
            while g.outcome().is_none() && n < 3000 {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
                for s in &g.sides {
                    assert_eq!(s.total_cards(), 30);
                }
                n += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed}");
        }
    }

    #[test]
    fn ai_plays_legal_games_and_is_fast() {
        use polytabletop_core::GameObj;
        let mut g: Box<dyn GameObj> = Box::new(TcgEvo::with_seed(33));
        let mut rng = Rng::new(3);
        let mut n = 0;
        while g.outcome().is_none() && n < 3000 {
            let m = g.ai_move(rng.next_u64()).unwrap();
            g.play(&m).unwrap();
            n += 1;
        }
        assert!(g.outcome().is_some());
    }

    #[test]
    fn ai_beats_random_mostly() {
        use polytabletop_core::GameObj;
        let mut wins = 0;
        for seed in 1..=8u64 {
            let mut g: Box<dyn GameObj> = Box::new(TcgEvo::with_seed(seed));
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() && n < 3000 {
                let m = if g.to_move() == Player::One {
                    g.ai_move(rng.next_u64()).unwrap()
                } else {
                    let l = g.legal_moves();
                    l[rng.below(l.len())].clone()
                };
                g.play(&m).unwrap();
                n += 1;
            }
            if g.outcome() == Some(Outcome::Win(Player::One)) {
                wins += 1;
            }
        }
        assert!(wins >= 6, "AI a câștigat doar {wins}/8");
    }
}

#[cfg(test)]
mod viewer_tests {
    use super::*;

    fn hand_section(r: &str, n: usize) -> String {
        let head = format!("Mâna Jucătorului {n}:\n");
        let i = r.find(&head).expect("antet mână") + head.len();
        let rest = &r[i..];
        let j = rest.find("La mutare").unwrap_or(rest.len());
        rest[..j].to_string()
    }

    fn expected(ids: &[u8]) -> String {
        ids.iter()
            .enumerate()
            .map(|(i, id)| format!("  {}. {}\n", i + 1, card_line(*id)))
            .collect()
    }

    #[test]
    fn viewer_shows_only_own_hand() {
        let mut g = TcgEvo::with_seed(5);
        for (who, n) in [(Player::One, 1usize), (Player::Two, 2)] {
            Game::set_viewer(&mut g, Some(who));
            let r = Game::render(&g);
            assert!(!r.contains(&format!("Mâna Jucătorului {}:", 3 - n)), "{r}");
            let h = hand_section(&r, n);
            assert!(h.starts_with(&expected(&g.sides[who.idx()].hand)), "{r}");
        }
    }

    #[test]
    fn hotseat_follows_the_mover() {
        let mut g = TcgEvo::with_seed(5);
        Game::set_viewer(&mut g, None);
        let r = Game::render(&g);
        let n = Game::to_move(&g).idx() + 1;
        assert!(r.contains(&format!("Mâna Jucătorului {n}:")), "{r}");
        Game::set_viewer(&mut g, Some(Player::One));
        let a = Game::render(&g);
        Game::set_viewer(&mut g, Some(Player::Two));
        assert_ne!(a, Game::render(&g));
    }
}

#[cfg(test)]
mod a2_audit {
    use super::*;

    #[test]
    fn ai_nu_vede_mana_adversarului_nici_pachetele_si_premiile_la_orice_nivel() {
        crate::p3_comun::ai_blind_check(
            "tcg_evo",
            TcgEvo::with_seed,
            |g, me, seed| {
                // mâna adversarului, pachetele și premiile (ale ambilor) sunt necunoscute jucătorului
                for (k, q) in [me.other().idx(), me.idx()].into_iter().enumerate() {
                    let s = &mut g.sides[q];
                    let (nh, nd) = (s.hand.len(), s.deck.len());
                    let mut pool: Vec<u8> = Vec::new();
                    if k == 0 {
                        pool.extend(s.hand.iter().copied());
                    }
                    pool.extend(s.deck.iter().copied());
                    pool.extend(s.prizes.iter().copied());
                    crate::p3_comun::shuffle_with(&mut pool, seed + q as u64);
                    if k == 0 {
                        s.hand = pool.drain(..nh).collect();
                    }
                    s.deck = pool.drain(..nd).collect();
                    s.prizes = pool;
                }
            },
            16,
        );
    }
}
