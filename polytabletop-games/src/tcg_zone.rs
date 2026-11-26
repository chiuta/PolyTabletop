//! Motor de cărți colecționabile „monștri în zone + capcane + invocare cu
//! tribut” (id `tcg_zone`), 2 jucători, cu un set ORIGINAL de cărți („Seria
//! Hotarelor Ceții”). Reguli scrise de noi, din mecanici generice.
//!
//! * Pachet pregătit de 30 de cărți (J1 „Hotarul”, J2 „Mlaștina”), 5 cărți în
//!   mână, 8000 de puncte de viață (PV). Tura: `pesca` (forțat; pachet gol =
//!   pierdere), faza principală, `fin`.
//! * Cinci zone de monștri și cinci zone de capcane. O invocare de monstru pe
//!   tură: nivel 1–4 fără tribut, 5–6 cu un tribut, 7+ cu două:
//!   `invoca 2` (în poziție de atac) / `seteaza 2` (întors, în apărare), iar cu
//!   tribut `invoca 2 tribut 1,3`. Monștrii întorși se răsucesc cu `intoarce I`
//!   (nu în tura în care au fost puși) și își schimbă poziția cu `schimba I`
//!   (o dată pe tură, nu în tura invocării și nu după atac).
//! * Vrăjile se joacă direct din mână: `joaca 3 [țintă]` (țintă: `e2` monstrul
//!   advers 2, `m1` al meu, `g2` al doilea monstru din cimitirul meu). Cele de
//!   echipare rămân atașate monstrului. Capcanele se pun întoarse cu `seteaza N`
//!   și se pot activa începând din tura adversarului.
//! * Lupta: `ataca 1 e2` sau `ataca 1 direct` (doar dacă adversarul nu are
//!   monștri). Jucătorul 1 nu atacă în tura 1. ATK vs ATK: cel slab e distrus,
//!   diferența îl costă PV pe stăpân; ATK vs APĂRARE: dacă atacă mai puternic,
//!   apărătorul e distrus (fără daune la PV, dar „Pătrunde” trece diferența),
//!   altfel atacatorul pierde diferența. Monstrul întors e răsucit în luptă.
//!   „Vânător”: când distruge un monstru, adversarul pierde 500 PV.
//! * Fereastra de capcane: după fiecare atac apărătorul primește mutarea
//!   forțată `continua` sau `activa T` (o singură capcană pe atac). Fereastra
//!   se deschide mereu, ca AI-ul să nu poată deduce ce ascunde adversarul.
//!
//! Informația ascunsă: `render()` arată doar mâna J1 și propriile cărți întoarse;
//! cele ale adversarului apar ca „întoarse”. AI-ul (AlphaBeta(1)) are `evaluate`
//! care citește doar ce vede (cărțile întoarse ale adversarului au valoare
//! constantă; lupta contra unui monstru întors nu e simulată).

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const PIERCE: u8 = 1;
pub const HUNT: u8 = 2;

const MAXZONE: usize = 5;
const START_LP: i32 = 8000;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Sp {
    Draw(u8),
    Heal(i32),
    Burn(i32),
    Destroy,
    DestroyAll,
    Revive,
    Equip(i32, i32),
    Bounce,
    Weak(i32),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tp {
    Negate,
    DestroyAtt,
    Reflect,
    Fog(i32),
    Shield(i32),
    Double,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum K {
    Mon { lvl: u8, atk: i32, def: i32, kw: u8 },
    Spell(Sp),
    Trap(Tp),
}

pub struct Card {
    pub name: &'static str,
    pub k: K,
}

const fn mo(name: &'static str, lvl: u8, atk: i32, def: i32, kw: u8) -> Card {
    Card {
        name,
        k: K::Mon { lvl, atk, def, kw },
    }
}
const fn sp(name: &'static str, s: Sp) -> Card {
    Card {
        name,
        k: K::Spell(s),
    }
}
const fn tp(name: &'static str, t: Tp) -> Card {
    Card {
        name,
        k: K::Trap(t),
    }
}

/// Setul complet: 44 de cărți originale.
pub const CARDS: &[Card] = &[
    mo("Licurici de Cenușă", 1, 400, 300, 0),
    mo("Șoricel Vrăjit", 1, 300, 600, 0),
    mo("Spiridușul Cămării", 2, 700, 500, 0),
    mo("Lăstar Viu", 2, 600, 1000, 0),
    mo("Iepure de Argint", 2, 500, 1100, 0),
    mo("Lup Pitic", 3, 1200, 800, 0),
    mo("Străjer de Hotar", 3, 900, 1600, 0),
    mo("Arcaș Tăcut", 3, 1300, 700, PIERCE),
    mo("Gnom Fierar", 3, 1100, 1300, 0),
    mo("Vrăjitor Scund", 3, 1000, 1000, 0),
    mo("Vânător de Umbre", 4, 1600, 900, HUNT),
    mo("Zid Viu", 4, 800, 2000, 0),
    mo("Șoim de Piatră", 4, 1700, 1000, 0),
    mo("Cavaler al Dimineții", 4, 1800, 1200, 0),
    mo("Spadasin Ruginit", 4, 1500, 1100, 0),
    mo("Paznicul Porții", 4, 1400, 1800, 0),
    mo("Căpcăun Roșu", 4, 1900, 800, 0),
    mo("Grifon Cenușiu", 5, 2100, 1400, 0),
    mo("Troll de Mlaștină", 5, 2200, 1200, 0),
    mo("Vulpea Aurie", 5, 1900, 1600, PIERCE),
    mo("Dragon Tânăr", 6, 2400, 1800, 0),
    mo("Golem de Stejar", 6, 2100, 2200, 0),
    mo("Dragonul Furtunii", 7, 2800, 2200, 0),
    mo("Titanul Munților", 8, 3200, 2400, 0),
    mo("Hidra Înghețului", 7, 2600, 2000, HUNT),
    mo("Colosul de Ceață", 8, 3000, 2600, 0),
    sp("Ploaie de Idei", Sp::Draw(2)),
    sp("Cântec de Adunare", Sp::Draw(1)),
    sp("Poțiune de Lumină", Sp::Heal(1500)),
    sp("Izvorul Mare", Sp::Heal(3000)),
    sp("Lovitură de Trăsnet", Sp::Destroy),
    sp("Furtuna Mare", Sp::DestroyAll),
    sp("Foc de Tabără", Sp::Burn(800)),
    sp("Rug de Vrăjitoare", Sp::Burn(1500)),
    sp("Înviere", Sp::Revive),
    sp("Sabie de Argint", Sp::Equip(500, 0)),
    sp("Armură Veche", Sp::Equip(300, 300)),
    sp("Blestem Slab", Sp::Weak(800)),
    sp("Vânt Rece", Sp::Bounce),
    tp("Pecete de Tăcere", Tp::Negate),
    tp("Gaură Adâncă", Tp::DestroyAtt),
    tp("Oglinda de Cremene", Tp::Reflect),
    tp("Ceață Deasă", Tp::Fog(1000)),
    tp("Scut de Lumină", Tp::Shield(1000)),
    tp("Curte Dublă", Tp::Double),
];

pub fn card_id(name: &str) -> u8 {
    CARDS
        .iter()
        .position(|c| c.name == name)
        .unwrap_or_else(|| panic!("carte necunoscută: {name}")) as u8
}

const DECK_HOTARUL: &[(&str, u8)] = &[
    ("Lăstar Viu", 2),
    ("Iepure de Argint", 1),
    ("Străjer de Hotar", 3),
    ("Gnom Fierar", 2),
    ("Vrăjitor Scund", 1),
    ("Zid Viu", 2),
    ("Paznicul Porții", 2),
    ("Cavaler al Dimineții", 2),
    ("Grifon Cenușiu", 1),
    ("Golem de Stejar", 1),
    ("Dragon Tânăr", 1),
    ("Titanul Munților", 1),
    ("Ploaie de Idei", 2),
    ("Poțiune de Lumină", 1),
    ("Lovitură de Trăsnet", 1),
    ("Armură Veche", 1),
    ("Vânt Rece", 1),
    ("Pecete de Tăcere", 2),
    ("Gaură Adâncă", 1),
    ("Scut de Lumină", 1),
    ("Ceață Deasă", 1),
];

const DECK_MLASTINA: &[(&str, u8)] = &[
    ("Spiridușul Cămării", 2),
    ("Lup Pitic", 3),
    ("Arcaș Tăcut", 2),
    ("Vânător de Umbre", 2),
    ("Șoim de Piatră", 2),
    ("Spadasin Ruginit", 2),
    ("Căpcăun Roșu", 2),
    ("Troll de Mlaștină", 2),
    ("Vulpea Aurie", 1),
    ("Dragonul Furtunii", 1),
    ("Hidra Înghețului", 1),
    ("Ploaie de Idei", 1),
    ("Lovitură de Trăsnet", 1),
    ("Furtuna Mare", 1),
    ("Foc de Tabără", 2),
    ("Sabie de Argint", 1),
    ("Înviere", 1),
    ("Oglinda de Cremene", 1),
    ("Curte Dublă", 1),
    ("Gaură Adâncă", 1),
];

pub fn deck_list(p: Player) -> Vec<u8> {
    let src = if p == Player::One {
        DECK_HOTARUL
    } else {
        DECK_MLASTINA
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
        "Hotarul"
    } else {
        "Mlaștina"
    }
}

fn kw_text(kw: u8) -> &'static str {
    match kw {
        PIERCE => "Pătrunde",
        HUNT => "Vânător",
        _ => "",
    }
}

fn sp_text(s: Sp) -> String {
    match s {
        Sp::Draw(n) => format!("pescuiești {n}"),
        Sp::Heal(n) => format!("câștigi {n} PV"),
        Sp::Burn(n) => format!("adversarul pierde {n} PV"),
        Sp::Destroy => "distruge un monstru advers".into(),
        Sp::DestroyAll => "distruge toți monștrii".into(),
        Sp::Revive => "readuci un monstru din cimitir".into(),
        Sp::Equip(a, d) => format!("echipare: +{a} ATK, +{d} APĂRARE"),
        Sp::Bounce => "un monstru advers se întoarce în mâna lui".into(),
        Sp::Weak(n) => format!("un monstru advers -{n} ATK până la sfârșitul turei"),
    }
}

fn tp_text(t: Tp) -> String {
    match t {
        Tp::Negate => "capcană la atac: anulează atacul".into(),
        Tp::DestroyAtt => "capcană la atac: distruge atacatorul".into(),
        Tp::Reflect => "capcană la atac: anulează și atacatorul pierde jumătate din ATK PV".into(),
        Tp::Fog(n) => format!("capcană la atac: atacatorul -{n} ATK"),
        Tp::Shield(n) => format!("capcană la atac: anulează și câștigi {n} PV"),
        Tp::Double => "capcană la atac: distruge atacatorul și 500 PV daune".into(),
    }
}

pub fn card_line(id: u8) -> String {
    let c = &CARDS[id as usize];
    match c.k {
        K::Mon { lvl, atk, def, kw } => {
            let k = kw_text(kw);
            format!(
                "{} (monstru nivel {lvl}, ATK {atk} / AP {def}{})",
                c.name,
                if k.is_empty() {
                    String::new()
                } else {
                    format!("; {k}")
                }
            )
        }
        K::Spell(s) => format!("{} (vrajă: {})", c.name, sp_text(s)),
        K::Trap(t) => format!("{} ({})", c.name, tp_text(t)),
    }
}

fn tributes_needed(lvl: u8) -> u32 {
    match lvl {
        0..=4 => 0,
        5..=6 => 1,
        _ => 2,
    }
}

#[derive(Clone, Debug)]
pub struct Mn {
    pub card: u8,
    pub uid: u16,
    pub face_up: bool,
    pub atk_pos: bool,
    pub new: bool,
    pub attacked: bool,
    pub changed: bool,
    pub equips: Vec<u8>,
    pub eq_atk: i32,
    pub eq_def: i32,
    pub temp: i32,
}

impl Mn {
    fn base(&self) -> (u8, i32, i32, u8) {
        match CARDS[self.card as usize].k {
            K::Mon { lvl, atk, def, kw } => (lvl, atk, def, kw),
            _ => panic!("nu e monstru"),
        }
    }
    pub fn atk(&self) -> i32 {
        (self.base().1 + self.eq_atk + self.temp).max(0)
    }
    pub fn def(&self) -> i32 {
        self.base().2 + self.eq_def
    }
}

#[derive(Clone, Debug)]
pub struct TrapZ {
    pub card: u8,
    pub set_turn: u32,
}

#[derive(Clone, Debug, Default)]
pub struct Side {
    pub lp: i32,
    pub deck: Vec<u8>,
    pub hand: Vec<u8>,
    pub mons: Vec<Mn>,
    pub traps: Vec<TrapZ>,
    pub grave: Vec<u8>,
    pub summoned: bool,
}

impl Side {
    pub fn total_cards(&self) -> usize {
        self.deck.len()
            + self.hand.len()
            + self.grave.len()
            + self.traps.len()
            + self.mons.iter().map(|m| 1 + m.equips.len()).sum::<usize>()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tgt {
    None,
    Foe(u8),
    Mine(u8),
    Grave(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Draw,
    /// (mână, întors?, masca tributelor)
    Summon(u8, bool, u8),
    SetTrap(u8),
    Cast(u8, Tgt),
    Flip(u8),
    Switch(u8),
    /// (atacator, țintă; None = direct)
    Attack(u8, Option<u8>),
    End,
    Pass,
    Trap(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Phase {
    Draw,
    Main,
    Window,
}

#[derive(Clone, Copy, Debug)]
struct Pend {
    att: u16,
    tgt: Option<u16>,
}

#[derive(Clone)]
pub struct TcgZone {
    pub sides: [Side; 2],
    pub turn: Player,
    /// Jucătorul a cărui tură este (diferă de `turn` în fereastra de capcane).
    pub cur: Player,
    pub phase: Phase,
    pub turn_no: u32,
    pend: Option<Pend>,
    next_uid: u16,
    over: Option<Outcome>,
    /// `Some(p)` = render shows `p`'s hand, set monsters and traps; `None` = hot-seat (the mover's).
    viewer: Option<Player>,
}

impl Default for TcgZone {
    fn default() -> Self {
        Self::new()
    }
}

fn shuffle(v: &mut [u8], rng: &mut Rng) {
    for i in (1..v.len()).rev() {
        v.swap(i, rng.below(i + 1));
    }
}

impl TcgZone {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut sides: [Side; 2] = [Side::default(), Side::default()];
        for p in [Player::One, Player::Two] {
            let mut d = deck_list(p);
            shuffle(&mut d, &mut rng);
            let s = &mut sides[p.idx()];
            s.lp = START_LP;
            for _ in 0..5 {
                s.hand.push(d.pop().unwrap());
            }
            s.deck = d;
        }
        TcgZone {
            sides,
            turn: Player::One,
            cur: Player::One,
            phase: Phase::Draw,
            turn_no: 1,
            pend: None,
            next_uid: 1,
            over: None,
            viewer: Some(Player::One),
        }
    }

    pub fn side(&self, p: Player) -> &Side {
        &self.sides[p.idx()]
    }

    fn check_lp(&mut self) {
        let a = self.sides[0].lp <= 0;
        let b = self.sides[1].lp <= 0;
        self.over = match (a, b) {
            (true, true) => Some(Outcome::Draw),
            (true, false) => Some(Outcome::Win(Player::Two)),
            (false, true) => Some(Outcome::Win(Player::One)),
            _ => None,
        };
    }

    fn end_turn(&mut self) {
        for s in &mut self.sides {
            s.summoned = false;
            for m in &mut s.mons {
                m.temp = 0;
            }
        }
        for m in &mut self.sides[self.cur.idx()].mons {
            m.new = false;
            m.attacked = false;
            m.changed = false;
        }
        let n = self.cur.other();
        self.cur = n;
        self.turn = n;
        self.turn_no += 1;
        self.phase = Phase::Draw;
    }

    fn destroy(&mut self, p: Player, i: usize) {
        let s = &mut self.sides[p.idx()];
        let m = s.mons.remove(i);
        s.grave.push(m.card);
        s.grave.extend(m.equips);
    }

    fn find(&self, p: Player, uid: u16) -> Option<usize> {
        self.sides[p.idx()].mons.iter().position(|m| m.uid == uid)
    }

    fn new_mn(&mut self, card: u8, face_up: bool, atk_pos: bool) -> Mn {
        let uid = self.next_uid;
        self.next_uid += 1;
        Mn {
            card,
            uid,
            face_up,
            atk_pos,
            new: true,
            attacked: false,
            changed: false,
            equips: Vec::new(),
            eq_atk: 0,
            eq_def: 0,
            temp: 0,
        }
    }

    fn tribute_masks(&self, n: usize, k: u32) -> Vec<u8> {
        (0u32..(1 << n))
            .filter(|m| m.count_ones() == k)
            .map(|m| m as u8)
            .collect()
    }

    fn spell_targets(&self, p: Player, s: Sp) -> Vec<Tgt> {
        let me = &self.sides[p.idx()];
        let foe = &self.sides[p.other().idx()];
        match s {
            Sp::Destroy | Sp::Bounce | Sp::Weak(_) => {
                (0..foe.mons.len() as u8).map(Tgt::Foe).collect()
            }
            Sp::Equip(..) => (0..me.mons.len() as u8).map(Tgt::Mine).collect(),
            Sp::Revive => {
                if me.mons.len() >= MAXZONE {
                    return Vec::new();
                }
                me.grave
                    .iter()
                    .enumerate()
                    .filter(|(_, c)| matches!(CARDS[**c as usize].k, K::Mon { .. }))
                    .map(|(i, _)| Tgt::Grave(i as u8))
                    .collect()
            }
            _ => vec![Tgt::None],
        }
    }

    fn do_spell(&mut self, p: Player, id: u8, s: Sp, t: Tgt) {
        let q = p.other();
        match (s, t) {
            (Sp::Draw(n), _) => {
                for _ in 0..n {
                    if let Some(c) = self.sides[p.idx()].deck.pop() {
                        self.sides[p.idx()].hand.push(c);
                    }
                }
            }
            (Sp::Heal(n), _) => self.sides[p.idx()].lp += n,
            (Sp::Burn(n), _) => self.sides[q.idx()].lp -= n,
            (Sp::Destroy, Tgt::Foe(i)) => self.destroy(q, i as usize),
            (Sp::DestroyAll, _) => {
                for who in [p, q] {
                    while !self.sides[who.idx()].mons.is_empty() {
                        self.destroy(who, 0);
                    }
                }
            }
            (Sp::Revive, Tgt::Grave(i)) => {
                let c = self.sides[p.idx()].grave.remove(i as usize);
                let m = self.new_mn(c, true, true);
                self.sides[p.idx()].mons.push(m);
            }
            (Sp::Equip(a, d), Tgt::Mine(i)) => {
                let m = &mut self.sides[p.idx()].mons[i as usize];
                m.eq_atk += a;
                m.eq_def += d;
                m.equips.push(id);
                return; // cartea rămâne pe câmp
            }
            (Sp::Bounce, Tgt::Foe(i)) => {
                let m = self.sides[q.idx()].mons.remove(i as usize);
                let s = &mut self.sides[q.idx()];
                s.hand.push(m.card);
                s.grave.extend(m.equips);
            }
            (Sp::Weak(n), Tgt::Foe(i)) => self.sides[q.idx()].mons[i as usize].temp -= n,
            _ => {}
        }
        self.sides[p.idx()].grave.push(id);
    }

    /// Rezolvă lupta în așteptare.
    fn battle(&mut self) {
        let p = self.cur;
        let q = p.other();
        let pend = self.pend.take().unwrap();
        self.phase = Phase::Main;
        self.turn = p;
        let Some(ai) = self.find(p, pend.att) else {
            return;
        };
        self.sides[p.idx()].mons[ai].attacked = true;
        let (_, _, _, akw) = self.sides[p.idx()].mons[ai].base();
        let a = self.sides[p.idx()].mons[ai].atk();
        match pend.tgt {
            None => self.sides[q.idx()].lp -= a,
            Some(tu) => {
                let Some(di) = self.find(q, tu) else {
                    return;
                };
                self.sides[q.idx()].mons[di].face_up = true;
                let d = &self.sides[q.idx()].mons[di];
                let (dpow, dest_att) = if d.atk_pos {
                    (d.atk(), true)
                } else {
                    (d.def(), false)
                };
                if a > dpow {
                    self.destroy(q, di);
                    if dest_att || akw & PIERCE != 0 {
                        self.sides[q.idx()].lp -= a - dpow;
                    }
                    if akw & HUNT != 0 {
                        self.sides[q.idx()].lp -= 500;
                    }
                } else if a < dpow {
                    self.sides[p.idx()].lp -= dpow - a;
                    if dest_att {
                        self.destroy(p, ai);
                    }
                } else if dest_att {
                    self.destroy(q, di);
                    self.destroy(p, ai);
                }
            }
        }
        self.check_lp();
    }

    fn trap_ok(&self, p: Player) -> Vec<u8> {
        self.sides[p.idx()]
            .traps
            .iter()
            .enumerate()
            .filter(|(_, t)| t.set_turn < self.turn_no)
            .map(|(i, _)| i as u8)
            .collect()
    }

    fn mon_val(m: &Mn, real: bool) -> i32 {
        if !m.face_up && !real {
            return 20;
        }
        let (_, atk, def, kw) = m.base();
        let _ = (atk, def);
        let mut v = (m.atk() + m.def() / 2) / 70;
        if kw != 0 {
            v += 3;
        }
        if !m.atk_pos {
            v -= 2;
        }
        v
    }

    fn side_val(&self, who: Player, viewer: Player) -> i32 {
        let s = &self.sides[who.idx()];
        let real = who == viewer;
        let mut v = s.lp.max(-2000) / 100 + 2 * s.hand.len() as i32;
        v += s.mons.iter().map(|m| Self::mon_val(m, real)).sum::<i32>();
        v += 8 * s.traps.len() as i32;
        v
    }

    fn base_diff(&self, p: Player) -> i32 {
        self.side_val(p, p) - self.side_val(p.other(), p)
    }
}

impl Game for TcgZone {
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
            Phase::Window => {
                let mut v = vec![Move::Pass];
                for i in self.trap_ok(p) {
                    v.push(Move::Trap(i));
                }
                v
            }
            Phase::Main => {
                let mut v = vec![Move::End];
                let foe = &self.sides[p.other().idx()];
                for (h, id) in s.hand.iter().enumerate() {
                    let h = h as u8;
                    match CARDS[*id as usize].k {
                        K::Mon { lvl, .. } => {
                            if s.summoned {
                                continue;
                            }
                            let k = tributes_needed(lvl);
                            if s.mons.len() < k as usize {
                                continue;
                            }
                            if s.mons.len() + 1 - (k as usize) > MAXZONE {
                                continue;
                            }
                            for mask in self.tribute_masks(s.mons.len(), k) {
                                v.push(Move::Summon(h, false, mask));
                                v.push(Move::Summon(h, true, mask));
                            }
                        }
                        K::Spell(sp) => {
                            for t in self.spell_targets(p, sp) {
                                v.push(Move::Cast(h, t));
                            }
                        }
                        K::Trap(_) => {
                            if s.traps.len() < MAXZONE {
                                v.push(Move::SetTrap(h));
                            }
                        }
                    }
                }
                for (i, m) in s.mons.iter().enumerate() {
                    let i = i as u8;
                    if !m.face_up && !m.new {
                        v.push(Move::Flip(i));
                    }
                    if m.face_up && !m.new && !m.attacked && !m.changed {
                        v.push(Move::Switch(i));
                    }
                    if m.face_up && m.atk_pos && !m.attacked && self.turn_no > 1 {
                        if foe.mons.is_empty() {
                            v.push(Move::Attack(i, None));
                        } else {
                            for j in 0..foe.mons.len() as u8 {
                                v.push(Move::Attack(i, Some(j)));
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
            Move::Draw => match self.sides[p.idx()].deck.pop() {
                None => self.over = Some(Outcome::Win(p.other())),
                Some(c) => {
                    self.sides[p.idx()].hand.push(c);
                    self.phase = Phase::Main;
                }
            },
            Move::End => self.end_turn(),
            Move::Summon(h, set, mask) => {
                let id = self.sides[p.idx()].hand.remove(h as usize);
                let mut idxs: Vec<usize> = (0..8).filter(|i| mask & (1 << i) != 0).collect();
                idxs.sort_unstable_by(|a, b| b.cmp(a));
                for i in idxs {
                    self.destroy(p, i);
                }
                let mn = self.new_mn(id, !set, !set);
                let s = &mut self.sides[p.idx()];
                s.mons.push(mn);
                s.summoned = true;
            }
            Move::SetTrap(h) => {
                let id = self.sides[p.idx()].hand.remove(h as usize);
                let tn = self.turn_no;
                self.sides[p.idx()].traps.push(TrapZ {
                    card: id,
                    set_turn: tn,
                });
            }
            Move::Cast(h, t) => {
                let id = self.sides[p.idx()].hand.remove(h as usize);
                let K::Spell(s) = CARDS[id as usize].k else {
                    panic!("nu e vrajă")
                };
                self.do_spell(p, id, s, t);
                self.check_lp();
            }
            Move::Flip(i) => {
                let m = &mut self.sides[p.idx()].mons[i as usize];
                m.face_up = true;
                m.atk_pos = true;
            }
            Move::Switch(i) => {
                let m = &mut self.sides[p.idx()].mons[i as usize];
                m.atk_pos = !m.atk_pos;
                m.changed = true;
            }
            Move::Attack(i, t) => {
                let att = self.sides[p.idx()].mons[i as usize].uid;
                let tgt = t.map(|j| self.sides[p.other().idx()].mons[j as usize].uid);
                self.pend = Some(Pend { att, tgt });
                self.phase = Phase::Window;
                self.turn = p.other();
            }
            Move::Pass => self.battle(),
            Move::Trap(i) => {
                let cur = self.cur;
                let def = self.turn;
                let t = self.sides[def.idx()].traps.remove(i as usize);
                self.sides[def.idx()].grave.push(t.card);
                let K::Trap(tp) = CARDS[t.card as usize].k else {
                    panic!("nu e capcană")
                };
                let pend = self.pend.unwrap();
                let ai = self.find(cur, pend.att);
                let mut negate = false;
                match tp {
                    Tp::Negate => negate = true,
                    Tp::DestroyAtt => {
                        if let Some(a) = ai {
                            self.destroy(cur, a);
                        }
                        negate = true;
                    }
                    Tp::Reflect => {
                        if let Some(a) = ai {
                            let d = self.sides[cur.idx()].mons[a].atk() / 2;
                            self.sides[cur.idx()].lp -= d;
                        }
                        negate = true;
                    }
                    Tp::Fog(n) => {
                        if let Some(a) = ai {
                            self.sides[cur.idx()].mons[a].temp -= n;
                        }
                    }
                    Tp::Shield(n) => {
                        self.sides[def.idx()].lp += n;
                        negate = true;
                    }
                    Tp::Double => {
                        if let Some(a) = ai {
                            self.destroy(cur, a);
                        }
                        self.sides[cur.idx()].lp -= 500;
                        negate = true;
                    }
                }
                if negate {
                    if let Some(a) = self.find(cur, pend.att) {
                        self.sides[cur.idx()].mons[a].attacked = true;
                    }
                    self.pend = None;
                    self.phase = Phase::Main;
                    self.turn = cur;
                    self.check_lp();
                } else {
                    self.battle();
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.over
    }

    fn evaluate(&self, p: Player) -> i32 {
        if self.phase == Phase::Window {
            let pend = self.pend.unwrap();
            let q = self.cur.other();
            let blind = pend
                .tgt
                .and_then(|u| self.find(q, u))
                .is_some_and(|i| !self.sides[q.idx()].mons[i].face_up);
            if !blind {
                // Rezultat simulat din informația vizibilă (ținta e față în sus sau direct).
                let mut c = self.clone();
                c.apply(&Move::Pass);
                return c.evaluate(p);
            }
            // Țintă întoarsă: estimare constantă (se presupune AP 1200).
            let a = self
                .find(self.cur, pend.att)
                .map_or(0, |i| self.sides[self.cur.idx()].mons[i].atk());
            let est = ((a - 1200) / 60).clamp(-25, 25);
            let v = self.base_diff(p);
            return if self.cur == p { v + est } else { v - est };
        }
        self.base_diff(p)
    }

    fn render(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            "Hotarele Ceții (zone) — tura {} — {}\n",
            self.turn_no,
            match self.phase {
                Phase::Draw => "pescuire",
                Phase::Main => "faza principală",
                Phase::Window => "fereastra de capcane",
            }
        ));
        let v = self.viewer.unwrap_or(self.turn);
        for p in [v.other(), v] {
            let sd = &self.sides[p.idx()];
            let who = if p == Player::One { 1 } else { 2 };
            s.push_str(&format!(
                "Jucătorul {who} ({}): PV {} | mână {} | pachet {} | cimitir {}\n",
                deck_name(p),
                sd.lp,
                sd.hand.len(),
                sd.deck.len(),
                sd.grave.len()
            ));
            if sd.mons.is_empty() {
                s.push_str("  Monștri: —\n");
            }
            for (i, m) in sd.mons.iter().enumerate() {
                let tag = if p == v { 'm' } else { 'e' };
                let name = CARDS[m.card as usize].name;
                let line = if m.face_up {
                    format!(
                        "{name} {}/{} ({})",
                        m.atk(),
                        m.def(),
                        if m.atk_pos { "atac" } else { "apărare" }
                    )
                } else if p == v {
                    format!("{name} (întors, apărare)")
                } else {
                    "carte întoarsă (apărare)".to_string()
                };
                s.push_str(&format!(
                    "  {tag}{}: {line}{}{}\n",
                    i + 1,
                    if m.new { " [nou]" } else { "" },
                    if m.attacked { " [a atacat]" } else { "" }
                ));
            }
            if p == v {
                let t: Vec<String> = sd
                    .traps
                    .iter()
                    .enumerate()
                    .map(|(i, t)| format!("t{}: {}", i + 1, CARDS[t.card as usize].name))
                    .collect();
                s.push_str(&format!(
                    "  Capcane puse: {}\n",
                    if t.is_empty() {
                        "—".to_string()
                    } else {
                        t.join(", ")
                    }
                ));
            } else {
                s.push_str(&format!("  Capcane puse (întoarse): {}\n", sd.traps.len()));
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
        if self.phase == Phase::Window {
            if let Some(pd) = self.pend {
                let a = self
                    .find(self.cur, pd.att)
                    .map(|i| &self.sides[self.cur.idx()].mons[i]);
                if let Some(a) = a {
                    s.push_str(&format!(
                        "Atac: {} ({}) — «continua» sau «activa T».\n",
                        CARDS[a.card as usize].name,
                        a.atk()
                    ));
                }
            }
        }
        s.push_str(&format!(
            "La mutare: Jucătorul {}\n",
            if self.turn == Player::One { 1 } else { 2 }
        ));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Draw => "pesca".into(),
            Move::End => "fin".into(),
            Move::Pass => "continua".into(),
            Move::Trap(i) => format!("activa {}", i + 1),
            Move::SetTrap(h) => format!("seteaza {}", h + 1),
            Move::Summon(h, set, mask) => {
                let verb = if *set { "seteaza" } else { "invoca" };
                if *mask == 0 {
                    format!("{verb} {}", h + 1)
                } else {
                    let v: Vec<String> = (0..8)
                        .filter(|i| mask & (1 << i) != 0)
                        .map(|i| (i + 1).to_string())
                        .collect();
                    format!("{verb} {} tribut {}", h + 1, v.join(","))
                }
            }
            Move::Cast(h, t) => match t {
                Tgt::None => format!("joaca {}", h + 1),
                Tgt::Foe(i) => format!("joaca {} e{}", h + 1, i + 1),
                Tgt::Mine(i) => format!("joaca {} m{}", h + 1, i + 1),
                Tgt::Grave(i) => format!("joaca {} g{}", h + 1, i + 1),
            },
            Move::Flip(i) => format!("intoarce {}", i + 1),
            Move::Switch(i) => format!("schimba {}", i + 1),
            Move::Attack(i, None) => format!("ataca {} direct", i + 1),
            Move::Attack(i, Some(j)) => format!("ataca {} e{}", i + 1, j + 1),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn main_phase(seed: u64) -> TcgZone {
        let mut g = TcgZone::with_seed(seed);
        g.apply(&Move::Draw);
        g
    }

    fn put(g: &mut TcgZone, p: Player, name: &str, up: bool, atk: bool) -> usize {
        let id = card_id(name);
        let mut m = g.new_mn(id, up, atk);
        m.new = false;
        let s = &mut g.sides[p.idx()];
        s.mons.push(m);
        s.mons.len() - 1
    }

    #[test]
    fn set_and_decks_are_valid() {
        assert!(CARDS.len() >= 40);
        let mut n: Vec<&str> = CARDS.iter().map(|c| c.name).collect();
        n.sort();
        n.dedup();
        assert_eq!(n.len(), CARDS.len());
        for p in [Player::One, Player::Two] {
            let d = deck_list(p);
            assert_eq!(d.len(), 30);
            for id in 0..CARDS.len() as u8 {
                assert!(d.iter().filter(|x| **x == id).count() <= 3);
            }
        }
    }

    #[test]
    fn setup_and_first_turn() {
        let g = TcgZone::with_seed(1);
        for s in &g.sides {
            assert_eq!(s.hand.len(), 5);
            assert_eq!(s.deck.len(), 25);
            assert_eq!(s.lp, 8000);
        }
        assert_eq!(g.legal_moves(), vec![Move::Draw]);
        let mut g = g;
        g.apply(&Move::Draw);
        assert_eq!(g.sides[0].hand.len(), 6);
        // nu se atacă în tura 1
        put(&mut g, Player::One, "Lup Pitic", true, true);
        assert!(!g
            .legal_moves()
            .iter()
            .any(|m| matches!(m, Move::Attack(..))));
    }

    #[test]
    fn tribute_rules() {
        let mut g = main_phase(2);
        g.sides[0].hand = vec![card_id("Dragon Tânăr"), card_id("Dragonul Furtunii")];
        assert!(!g
            .legal_moves()
            .iter()
            .any(|m| matches!(m, Move::Summon(..))));
        put(&mut g, Player::One, "Lup Pitic", true, true);
        let mv = g.legal_moves();
        assert!(mv.contains(&Move::Summon(0, false, 1)));
        assert!(mv.contains(&Move::Summon(0, true, 1)));
        assert!(
            !mv.iter().any(|m| matches!(m, Move::Summon(1, _, _))),
            "nivel 7 cere 2"
        );
        put(&mut g, Player::One, "Zid Viu", true, false);
        let mv = g.legal_moves();
        assert!(mv.contains(&Move::Summon(1, false, 3)));
        g.apply(&Move::Summon(1, false, 3));
        assert_eq!(g.sides[0].mons.len(), 1);
        assert_eq!(g.sides[0].grave.len(), 2);
        // o singură invocare pe tură
        g.sides[0].hand.push(card_id("Lup Pitic"));
        assert!(!g
            .legal_moves()
            .iter()
            .any(|m| matches!(m, Move::Summon(..))));
    }

    #[test]
    fn battle_atk_vs_atk_and_def() {
        let mut g = main_phase(3);
        g.turn_no = 3;
        put(&mut g, Player::One, "Cavaler al Dimineții", true, true); // 1800
        put(&mut g, Player::Two, "Lup Pitic", true, true); // 1200
        g.apply(&Move::Attack(0, Some(0)));
        assert_eq!(g.phase, Phase::Window);
        assert_eq!(g.turn, Player::Two);
        g.apply(&Move::Pass);
        assert!(g.sides[1].mons.is_empty());
        assert_eq!(g.sides[1].lp, 8000 - 600);
        assert_eq!(g.turn, Player::One);
        assert_eq!(g.phase, Phase::Main);

        // vs apărare mai mare: atacatorul pierde diferența
        let mut g = main_phase(3);
        g.turn_no = 3;
        put(&mut g, Player::One, "Lup Pitic", true, true); // 1200
        put(&mut g, Player::Two, "Zid Viu", true, false); // def 2000
        g.apply(&Move::Attack(0, Some(0)));
        g.apply(&Move::Pass);
        assert_eq!(g.sides[0].lp, 8000 - 800);
        assert_eq!(g.sides[0].mons.len(), 1);
        assert_eq!(g.sides[1].mons.len(), 1);
    }

    #[test]
    fn pierce_hunt_and_equal_trade() {
        let mut g = main_phase(4);
        g.turn_no = 3;
        put(&mut g, Player::One, "Vulpea Aurie", true, true); // 1900 pierce
        put(&mut g, Player::Two, "Străjer de Hotar", true, false); // def 1600
        g.apply(&Move::Attack(0, Some(0)));
        g.apply(&Move::Pass);
        assert!(g.sides[1].mons.is_empty());
        assert_eq!(g.sides[1].lp, 8000 - 300);

        let mut g = main_phase(4);
        g.turn_no = 3;
        put(&mut g, Player::One, "Vânător de Umbre", true, true);
        put(&mut g, Player::Two, "Zid Viu", true, false); // def 2000 > 1600
        g.sides[1].mons[0].eq_def -= 1000; // def 1000
        g.apply(&Move::Attack(0, Some(0)));
        g.apply(&Move::Pass);
        assert_eq!(g.sides[1].lp, 8000 - 500);

        let mut g = main_phase(4);
        g.turn_no = 3;
        put(&mut g, Player::One, "Lup Pitic", true, true);
        put(&mut g, Player::Two, "Lup Pitic", true, true);
        g.apply(&Move::Attack(0, Some(0)));
        g.apply(&Move::Pass);
        assert!(g.sides[0].mons.is_empty() && g.sides[1].mons.is_empty());
        assert_eq!(g.sides[0].grave.len(), 1);
    }

    #[test]
    fn direct_attack_only_without_monsters_and_face_down_flips() {
        let mut g = main_phase(5);
        g.turn_no = 3;
        put(&mut g, Player::One, "Lup Pitic", true, true);
        assert!(g.legal_moves().contains(&Move::Attack(0, None)));
        put(&mut g, Player::Two, "Zid Viu", false, false);
        let mv = g.legal_moves();
        assert!(!mv.contains(&Move::Attack(0, None)));
        assert!(mv.contains(&Move::Attack(0, Some(0))));
        g.apply(&Move::Attack(0, Some(0)));
        g.apply(&Move::Pass);
        assert!(g.sides[1].mons[0].face_up, "întors în luptă");
    }

    #[test]
    fn traps_need_a_turn_and_work() {
        let mut g = main_phase(6);
        g.sides[0].hand = vec![card_id("Gaură Adâncă")];
        g.apply(&Move::SetTrap(0));
        assert_eq!(g.sides[0].traps.len(), 1);
        g.apply(&Move::End);
        g.apply(&Move::Draw);
        // J2 atacă (tura 2): capcana pusă în tura 1 e activabilă
        put(&mut g, Player::One, "Zid Viu", true, false);
        put(&mut g, Player::Two, "Troll de Mlaștină", true, true);
        g.apply(&Move::Attack(0, Some(0)));
        assert_eq!(g.turn, Player::One);
        let mv = g.legal_moves();
        assert!(mv.contains(&Move::Trap(0)));
        g.apply(&Move::Trap(0));
        assert!(g.sides[1].mons.is_empty(), "atacatorul distrus");
        assert_eq!(g.sides[0].traps.len(), 0);
        assert_eq!(g.turn, Player::Two);
        assert_eq!(g.phase, Phase::Main);
    }

    #[test]
    fn fresh_trap_cannot_be_used_same_turn() {
        let mut g = main_phase(7);
        g.turn_no = 5;
        g.sides[1].traps.push(TrapZ {
            card: card_id("Pecete de Tăcere"),
            set_turn: 5,
        });
        put(&mut g, Player::One, "Lup Pitic", true, true);
        put(&mut g, Player::Two, "Zid Viu", true, false);
        g.apply(&Move::Attack(0, Some(0)));
        assert_eq!(g.legal_moves(), vec![Move::Pass]);
    }

    #[test]
    fn spells_equip_revive_destroy_all() {
        let mut g = main_phase(8);
        put(&mut g, Player::One, "Lup Pitic", true, true);
        put(&mut g, Player::Two, "Zid Viu", true, false);
        g.sides[0].hand = vec![
            card_id("Sabie de Argint"),
            card_id("Lovitură de Trăsnet"),
            card_id("Înviere"),
            card_id("Furtuna Mare"),
        ];
        g.sides[0].grave = vec![card_id("Titanul Munților"), card_id("Ploaie de Idei")];
        g.apply(&Move::Cast(0, Tgt::Mine(0)));
        assert_eq!(g.sides[0].mons[0].atk(), 1700);
        g.apply(&Move::Cast(0, Tgt::Foe(0)));
        assert!(g.sides[1].mons.is_empty());
        let mv = g.legal_moves();
        assert!(mv.contains(&Move::Cast(0, Tgt::Grave(0))));
        assert!(!mv.contains(&Move::Cast(0, Tgt::Grave(1))), "doar monștri");
        // grave: Titan, Ploaie, Trăsnet -> înviere pe Titan
        g.apply(&Move::Cast(0, Tgt::Grave(0)));
        assert_eq!(g.sides[0].mons.len(), 2);
        g.apply(&Move::Cast(0, Tgt::None));
        assert!(g.sides[0].mons.is_empty());
        // sabia a mers în cimitir odată cu monstrul
        assert!(g.sides[0].grave.contains(&card_id("Sabie de Argint")));
    }

    #[test]
    fn lethal_and_deck_out() {
        let mut g = main_phase(9);
        g.sides[1].lp = 700;
        g.sides[0].hand = vec![card_id("Foc de Tabără")];
        g.apply(&Move::Cast(0, Tgt::None));
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
        let mut g = TcgZone::with_seed(9);
        g.sides[0].deck.clear();
        g.apply(&Move::Draw);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
    }

    #[test]
    fn position_change_rules() {
        let mut g = main_phase(10);
        let i = put(&mut g, Player::One, "Zid Viu", true, false);
        g.apply(&Move::Switch(i as u8));
        assert!(g.sides[0].mons[i].atk_pos);
        assert!(!g.legal_moves().contains(&Move::Switch(i as u8)));
        let j = put(&mut g, Player::One, "Lup Pitic", false, false);
        assert!(g.legal_moves().contains(&Move::Flip(j as u8)));
        g.sides[0].mons[j].new = true;
        assert!(!g.legal_moves().contains(&Move::Flip(j as u8)));
    }

    #[test]
    fn render_hides_opponent_cards() {
        let mut g = TcgZone::with_seed(5);
        put(&mut g, Player::Two, "Zid Viu", false, false);
        let r = g.render();
        assert!(!r.contains("Zid Viu"));
        assert!(r.contains("carte întoarsă"));
        for l in g.sides[0].hand.iter().map(|i| card_line(*i)) {
            assert!(r.contains(l.as_str()));
        }
    }

    #[test]
    fn ai_evaluate_ignores_hidden_information() {
        let mut a = main_phase(21);
        put(&mut a, Player::Two, "Zid Viu", false, false);
        let mut b = a.clone();
        b.sides[1].mons[0].card = card_id("Titanul Munților");
        b.sides[1].hand = a.sides[1].hand.iter().rev().cloned().collect();
        assert_eq!(a.evaluate(Player::One), b.evaluate(Player::One));
        // atac asupra unui monstru întors: aceeași evaluare indiferent de ce e dedesubt
        let mut a = main_phase(22);
        a.turn_no = 3;
        put(&mut a, Player::One, "Căpcăun Roșu", true, true);
        put(&mut a, Player::Two, "Zid Viu", false, false);
        let mut b = a.clone();
        b.sides[1].mons[0].card = card_id("Lup Pitic");
        a.apply(&Move::Attack(0, Some(0)));
        b.apply(&Move::Attack(0, Some(0)));
        assert_eq!(a.evaluate(Player::One), b.evaluate(Player::One));
    }

    #[test]
    fn cards_conserved_over_random_games() {
        for seed in 1..=15u64 {
            let mut g = TcgZone::with_seed(seed);
            let mut rng = Rng::new(seed * 5 + 3);
            let mut n = 0;
            while g.outcome().is_none() && n < 4000 {
                let mv = g.legal_moves();
                assert!(!mv.is_empty(), "{}", g.render());
                g.apply(&mv[rng.below(mv.len())]);
                for s in &g.sides {
                    assert_eq!(s.total_cards(), 30);
                    assert!(s.mons.len() <= MAXZONE && s.traps.len() <= MAXZONE);
                }
                n += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed}");
        }
    }

    #[test]
    fn ai_plays_legal_games_and_beats_random() {
        use polytabletop_core::GameObj;
        let mut wins = 0;
        for seed in 1..=8u64 {
            let mut g: Box<dyn GameObj> = Box::new(TcgZone::with_seed(seed));
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() && n < 4000 {
                let m = if g.to_move() == Player::One {
                    g.ai_move(rng.next_u64()).unwrap()
                } else {
                    let l = g.legal_moves();
                    l[rng.below(l.len())].clone()
                };
                g.play(&m).unwrap();
                n += 1;
            }
            assert!(g.outcome().is_some());
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
        let mut g = TcgZone::with_seed(5);
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
        let mut g = TcgZone::with_seed(5);
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
