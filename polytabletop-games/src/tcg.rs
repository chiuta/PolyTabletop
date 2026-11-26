//! Motor generic de cărți colecționabile (TCG), 2 jucători, cu un set ORIGINAL
//! de 60 de cărți ("Seria Ținutul Cenușiu"): resurse, creaturi și vrăji.
//!
//! Reguli (scrise de noi, din mecanici generice):
//! * Fiecare jucător primește un pachet pregătit de 30 de cărți (J1 „Cetatea”,
//!   J2 „Furtuna”), amestecat cu Rng propriu; 5 cărți în mână, 20 de viață.
//! * Tura: **Pescuire** (mutare forțată `pesca`; J1 nu pescuiește în prima tură;
//!   pachet gol la pescuit = pierdere), **Principală** (joci cărți, ataci),
//!   `fin`. Mână maximă 7 (excesul se pierde), cel mult 6 creaturi pe câmp.
//! * Resursele (cost 0) se joacă una pe tură; fiecare crește rezerva de mană
//!   cu 1 (maxim 10) și o reîncarcă complet la începutul turei tale.
//! * Creaturi: cost, atac/rezistență, cuvinte-cheie: Zbor (blocat doar de
//!   zburători), Grabă (poate ataca imediat), Armură (-1 la fiecare lovitură),
//!   Venin (orice lovitură ucide), Primul (lovește înainte), Străjer (nu se
//!   înclină la atac). Creaturile proaspăt jucate nu pot ataca.
//! * Atac/blocaj: `ataca 1,3` declară atacatorii (toți se înclină, fără Străjer);
//!   apoi apărătorul decide, pe rând pentru fiecare atacator, `bloc N` sau `fara`.
//!   Neblocat lovește jucătorul. Daunele se șterg la sfârșitul turei.
//! * Vrăji cu țintă: `joaca 3 e2` (creatura inamică 2), `m1` (a mea), `fata`
//!   (jucătorul advers).
//!
//! Informația ascunsă și AI-ul: pachetele și mâinile stau în stare; `render()`
//! arată doar mâna J1. AI-ul (AlphaBeta(1)) are `evaluate(p)` care citește doar
//! ce vede `p`: câmpul public, mâna proprie (toate cărțile din mână sunt evaluate
//! la fel, deci nu „vede” ce trage) și NUMĂRUL cărților adversarului.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const ZBOR: u8 = 1;
pub const GRABA: u8 = 2;
pub const ARMURA: u8 = 4;
pub const VENIN: u8 = 8;
pub const PRIMUL: u8 = 16;
pub const STRAJER: u8 = 32;

const MAXHAND: usize = 7;
const MAXBOARD: usize = 6;
const MAXMANA: usize = 10;
const MAXLIFE: i16 = 20;
const START_LIFE: i16 = 20;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Res,
    Cre,
    Spell,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Eff {
    None,
    /// Daune jucătorului advers.
    Face(i16),
    Heal(i16),
    Draw(u8),
    /// Daune tuturor creaturilor inamice.
    AllFoes(i16),
    /// Daune jucătorului advers și vindecare egală.
    Drain(i16),
    /// Toate creaturile tale primesc +1/+1.
    Grow,
    /// Țintă: o creatură inamică sau jucătorul advers.
    DmgAny(i16),
    DmgCre(i16),
    Buff(i16, i16),
    Kill,
    Bounce,
}

pub struct Card {
    pub name: &'static str,
    pub kind: Kind,
    pub cost: u8,
    pub atk: i16,
    pub hp: i16,
    pub kw: u8,
    pub eff: Eff,
}

const fn res(name: &'static str, eff: Eff) -> Card {
    Card {
        name,
        kind: Kind::Res,
        cost: 0,
        atk: 0,
        hp: 0,
        kw: 0,
        eff,
    }
}
const fn cre(name: &'static str, cost: u8, atk: i16, hp: i16, kw: u8, eff: Eff) -> Card {
    Card {
        name,
        kind: Kind::Cre,
        cost,
        atk,
        hp,
        kw,
        eff,
    }
}
const fn sp(name: &'static str, cost: u8, eff: Eff) -> Card {
    Card {
        name,
        kind: Kind::Spell,
        cost,
        atk: 0,
        hp: 0,
        kw: 0,
        eff,
    }
}

use Eff::*;

/// Setul complet de 60 de cărți originale.
pub const CARDS: &[Card] = &[
    // --- resurse (4)
    res("Izvor de lumină", None),
    res("Fântâna Vieții", Heal(2)),
    res("Cristal aprins", Face(1)),
    res("Poiană de argint", None),
    // --- creaturi cost 1 (5)
    cre("Licurici", 1, 1, 1, ZBOR, None),
    cre("Șoricel de cămară", 1, 1, 2, 0, None),
    cre("Ucenic de fierar", 1, 2, 1, 0, None),
    cre("Pui de dragon", 1, 1, 1, GRABA, None),
    cre("Zid de umbre", 1, 0, 3, ARMURA, None),
    // --- cost 2 (7)
    cre("Lup cenușiu", 2, 2, 2, 0, None),
    cre("Cioară de pază", 2, 2, 1, ZBOR, None),
    cre("Gardian de piatră", 2, 1, 4, ARMURA, None),
    cre("Pumnal ascuns", 2, 1, 1, VENIN, None),
    cre("Cercetaș iute", 2, 2, 1, GRABA, None),
    cre("Preot de sat", 2, 1, 2, 0, Heal(2)),
    cre("Cărturar", 2, 1, 1, 0, Draw(1)),
    // --- cost 3 (8)
    cre("Spadasin călit", 3, 3, 3, 0, None),
    cre("Vultur de munte", 3, 3, 2, ZBOR, None),
    cre("Șarpe de mlaștină", 3, 2, 2, VENIN, None),
    cre("Duelist", 3, 2, 3, PRIMUL, None),
    cre("Pirat al norilor", 3, 2, 2, ZBOR, Face(1)),
    cre("Cavaler călare", 3, 3, 2, GRABA, None),
    cre("Străjer de câmpie", 3, 2, 4, STRAJER, None),
    cre("Sentinelă de ceață", 3, 1, 3, STRAJER | ARMURA, None),
    // --- cost 4 (7)
    cre("Urs de peșteră", 4, 4, 4, 0, None),
    cre("Grifon", 4, 3, 3, ZBOR, None),
    cre("Căpitan de gardă", 4, 3, 3, PRIMUL, Heal(2)),
    cre("Vrăjitoare de cenușă", 4, 2, 3, 0, AllFoes(1)),
    cre("Paznic al porții", 4, 2, 6, ARMURA, None),
    cre("Asasin al nopții", 4, 3, 2, VENIN | GRABA, None),
    cre("Fierar de luptă", 4, 3, 3, 0, Grow),
    // --- cost 5 (4)
    cre("Golem de fier", 5, 5, 5, ARMURA, None),
    cre("Dragon tânăr", 5, 4, 4, ZBOR, None),
    cre("Mare preoteasă", 5, 3, 4, 0, Drain(3)),
    cre("Titan de stâncă", 5, 4, 6, STRAJER, None),
    // --- cost 6+ (3)
    cre("Dragon bătrân", 6, 6, 6, ZBOR, None),
    cre("Hidra Mlaștinii", 6, 5, 5, VENIN, AllFoes(2)),
    cre("Colos de ceață", 7, 7, 7, PRIMUL, None),
    // --- vrăji (22)
    sp("Săgeată de foc", 1, DmgAny(2)),
    sp("Scânteie", 1, DmgCre(2)),
    sp("Fulger", 2, DmgAny(3)),
    sp("Trăsnet", 3, DmgAny(5)),
    sp("Ploaie de cenușă", 3, AllFoes(2)),
    sp("Furtună de foc", 5, AllFoes(4)),
    sp("Binecuvântare", 1, Buff(1, 1)),
    sp("Cântec de luptă", 2, Buff(2, 2)),
    sp("Hrană de război", 3, Buff(3, 3)),
    sp("Vindecare", 2, Heal(5)),
    sp("Elixir", 1, Heal(3)),
    sp("Înțelepciune", 2, Draw(2)),
    sp("Revelație", 3, Draw(3)),
    sp("Sorbire de viață", 3, Drain(3)),
    sp("Sărutul morții", 4, Kill),
    sp("Ruinare", 3, Kill),
    sp("Exil", 2, Bounce),
    sp("Strigăt de adunare", 4, Grow),
    sp("Foc sacru", 3, Face(3)),
    sp("Rafală", 1, Face(2)),
    sp("Mantie de ceață", 2, Buff(0, 4)),
    sp("Lance de gheață", 4, DmgCre(6)),
];

pub fn card_id(name: &str) -> u8 {
    CARDS
        .iter()
        .position(|c| c.name == name)
        .unwrap_or_else(|| panic!("carte necunoscută: {name}")) as u8
}

const DECK_CETATEA: &[(&str, u8)] = &[
    ("Izvor de lumină", 4),
    ("Fântâna Vieții", 3),
    ("Poiană de argint", 4),
    ("Zid de umbre", 1),
    ("Gardian de piatră", 2),
    ("Preot de sat", 2),
    ("Cărturar", 1),
    ("Spadasin călit", 2),
    ("Străjer de câmpie", 2),
    ("Duelist", 1),
    ("Urs de peșteră", 1),
    ("Paznic al porții", 1),
    ("Golem de fier", 1),
    ("Mare preoteasă", 1),
    ("Titan de stâncă", 1),
    ("Vindecare", 1),
    ("Binecuvântare", 1),
    ("Înțelepciune", 1),
];

const DECK_FURTUNA: &[(&str, u8)] = &[
    ("Izvor de lumină", 4),
    ("Cristal aprins", 4),
    ("Poiană de argint", 3),
    ("Licurici", 1),
    ("Pui de dragon", 2),
    ("Cercetaș iute", 2),
    ("Cioară de pază", 1),
    ("Vultur de munte", 2),
    ("Cavaler călare", 1),
    ("Grifon", 1),
    ("Asasin al nopții", 1),
    ("Dragon tânăr", 1),
    ("Dragon bătrân", 1),
    ("Săgeată de foc", 2),
    ("Fulger", 2),
    ("Trăsnet", 1),
    ("Foc sacru", 1),
];

pub fn deck_list(p: Player) -> Vec<u8> {
    let src = if p == Player::One {
        DECK_CETATEA
    } else {
        DECK_FURTUNA
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
        "Cetatea"
    } else {
        "Furtuna"
    }
}

fn kw_text(kw: u8) -> String {
    let mut v = Vec::new();
    for (f, n) in [
        (ZBOR, "Zbor"),
        (GRABA, "Grabă"),
        (ARMURA, "Armură"),
        (VENIN, "Venin"),
        (PRIMUL, "Primul"),
        (STRAJER, "Străjer"),
    ] {
        if kw & f != 0 {
            v.push(n);
        }
    }
    v.join(", ")
}

fn eff_text(e: Eff) -> String {
    match e {
        None => String::new(),
        Face(n) => format!("{n} daune adversarului"),
        Heal(n) => format!("vindeci {n}"),
        Draw(n) => format!("pescuiești {n}"),
        AllFoes(n) => format!("{n} daune fiecărei creaturi inamice"),
        Drain(n) => format!("{n} daune adversarului, vindeci {n}"),
        Grow => "creaturile tale +1/+1".into(),
        DmgAny(n) => format!("{n} daune unei creaturi sau adversarului"),
        DmgCre(n) => format!("{n} daune unei creaturi inamice"),
        Buff(a, h) => format!("o creatură a ta +{a}/+{h}"),
        Kill => "distruge o creatură inamică".into(),
        Bounce => "întoarce o creatură inamică în mâna ei".into(),
    }
}

/// Descrierea unei cărți pentru afișare.
pub fn card_line(id: u8) -> String {
    let c = &CARDS[id as usize];
    let mut s = match c.kind {
        Kind::Res => format!("{} (resursă", c.name),
        Kind::Cre => format!("{} (creatură {} mană, {}/{}", c.name, c.cost, c.atk, c.hp),
        Kind::Spell => format!("{} (vrajă {} mană", c.name, c.cost),
    };
    let kw = kw_text(c.kw);
    if !kw.is_empty() {
        s.push_str(&format!("; {kw}"));
    }
    let et = eff_text(c.eff);
    if !et.is_empty() {
        s.push_str(&format!("; {et}"));
    }
    s.push(')');
    s
}

#[derive(Clone, Debug)]
pub struct Creature {
    pub card: u8,
    pub uid: u16,
    pub atk: i16,
    pub hp: i16,
    pub dmg: i16,
    pub kw: u8,
    pub tapped: bool,
    pub fresh: bool,
    pub done: bool,
}

#[derive(Clone, Debug, Default)]
pub struct Side {
    pub life: i16,
    pub deck: Vec<u8>,
    pub hand: Vec<u8>,
    pub res: Vec<u8>,
    pub board: Vec<Creature>,
    pub grave: Vec<u8>,
    pub mana: u8,
    pub res_played: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tgt {
    None,
    Foe(u8),
    Mine(u8),
    Face,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Draw,
    Play(u8, Tgt),
    /// Masca de bit a indicilor de pe câmp care atacă.
    Attack(u8),
    End,
    Block(Option<u8>),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Phase {
    Draw,
    Main,
    Block,
}

#[derive(Clone)]
pub struct Tcg {
    pub sides: [Side; 2],
    pub turn: Player,
    pub phase: Phase,
    pub turn_no: u32,
    attackers: Vec<u16>,
    att_idx: usize,
    used: Vec<u16>,
    next_uid: u16,
    over: Option<Outcome>,
    /// `Some(p)` = render shows `p`'s hand; `None` = hot-seat (the mover's).
    viewer: Option<Player>,
}

impl Default for Tcg {
    fn default() -> Self {
        Self::new()
    }
}

fn shuffle(v: &mut [u8], rng: &mut Rng) {
    for i in (1..v.len()).rev() {
        v.swap(i, rng.below(i + 1));
    }
}

fn hit(att: &Creature, def: &Creature) -> i16 {
    let mut d = att.atk;
    if def.kw & ARMURA != 0 {
        d -= 1;
    }
    d.max(0)
}

fn lethal(att: &Creature, def: &Creature, d: i16) -> bool {
    d > 0 && (att.kw & VENIN != 0 || def.dmg + d >= def.hp)
}

/// Daunele pe care le dau `a` lui `b` și `b` lui `a` într-un duel.
fn duel(a: &Creature, b: &Creature) -> (i16, i16) {
    let da = hit(a, b);
    let db = hit(b, a);
    let af = a.kw & PRIMUL != 0;
    let bf = b.kw & PRIMUL != 0;
    if af && !bf {
        if lethal(a, b, da) {
            (da, 0)
        } else {
            (da, db)
        }
    } else if bf && !af {
        if lethal(b, a, db) {
            (0, db)
        } else {
            (da, db)
        }
    } else {
        (da, db)
    }
}

fn deal(c: &mut Creature, d: i16, venin: bool) {
    if d <= 0 {
        return;
    }
    c.dmg += d;
    if venin {
        c.dmg = c.dmg.max(c.hp);
    }
}

fn can_block(att: &Creature, blk: &Creature) -> bool {
    !blk.tapped && (att.kw & ZBOR == 0 || blk.kw & ZBOR != 0)
}

impl Tcg {
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
            s.life = START_LIFE;
            for _ in 0..5 {
                s.hand.push(d.pop().unwrap());
            }
            s.deck = d;
        }
        let mut g = Tcg {
            sides,
            turn: Player::One,
            phase: Phase::Draw,
            turn_no: 0,
            attackers: Vec::new(),
            att_idx: 0,
            used: Vec::new(),
            next_uid: 1,
            over: Option::None,
            viewer: Some(Player::One),
        };
        g.start_turn(Player::One);
        g
    }

    pub fn side(&self, p: Player) -> &Side {
        &self.sides[p.idx()]
    }

    pub fn outcome_now(&self) -> Option<Outcome> {
        self.over
    }

    fn start_turn(&mut self, p: Player) {
        self.turn = p;
        self.turn_no += 1;
        self.phase = Phase::Draw;
        let s = &mut self.sides[p.idx()];
        s.mana = s.res.len() as u8;
        s.res_played = false;
        for c in &mut s.board {
            c.tapped = false;
            c.fresh = false;
            c.done = false;
        }
    }

    fn draw_card(&mut self, p: Player) -> bool {
        let s = &mut self.sides[p.idx()];
        match s.deck.pop() {
            Option::None => false,
            Some(c) => {
                if s.hand.len() >= MAXHAND {
                    s.grave.push(c);
                } else {
                    s.hand.push(c);
                }
                true
            }
        }
    }

    fn reap(&mut self) {
        for s in &mut self.sides {
            let mut i = 0;
            while i < s.board.len() {
                if s.board[i].dmg >= s.board[i].hp {
                    let c = s.board.remove(i);
                    s.grave.push(c.card);
                } else {
                    i += 1;
                }
            }
        }
    }

    fn check_over(&mut self, actor: Player) {
        if self.over.is_some() {
            return;
        }
        let l0 = self.sides[0].life <= 0;
        let l1 = self.sides[1].life <= 0;
        self.over = match (l0, l1) {
            (true, true) => Some(Outcome::Draw),
            (true, false) => Some(Outcome::Win(Player::Two)),
            (false, true) => Some(Outcome::Win(Player::One)),
            _ => Option::None,
        };
        let _ = actor;
    }

    fn heal(&mut self, p: Player, n: i16) {
        let s = &mut self.sides[p.idx()];
        s.life = (s.life + n).min(MAXLIFE.max(s.life));
    }

    fn do_eff(&mut self, p: Player, eff: Eff, t: Tgt) {
        let q = p.other();
        match eff {
            None => {}
            Face(n) => self.sides[q.idx()].life -= n,
            Heal(n) => self.heal(p, n),
            Draw(n) => {
                for _ in 0..n {
                    self.draw_card(p);
                }
            }
            AllFoes(n) => {
                for c in &mut self.sides[q.idx()].board {
                    deal(c, n, false);
                }
            }
            Drain(n) => {
                self.sides[q.idx()].life -= n;
                self.heal(p, n);
            }
            Grow => {
                for c in &mut self.sides[p.idx()].board {
                    c.atk += 1;
                    c.hp += 1;
                }
            }
            DmgAny(n) => match t {
                Tgt::Foe(i) => {
                    if let Some(c) = self.sides[q.idx()].board.get_mut(i as usize) {
                        deal(c, n, false);
                    }
                }
                Tgt::Face => self.sides[q.idx()].life -= n,
                _ => {}
            },
            DmgCre(n) => {
                if let Tgt::Foe(i) = t {
                    if let Some(c) = self.sides[q.idx()].board.get_mut(i as usize) {
                        deal(c, n, false);
                    }
                }
            }
            Buff(a, h) => {
                if let Tgt::Mine(i) = t {
                    if let Some(c) = self.sides[p.idx()].board.get_mut(i as usize) {
                        c.atk += a;
                        c.hp += h;
                    }
                }
            }
            Kill => {
                if let Tgt::Foe(i) = t {
                    if let Some(c) = self.sides[q.idx()].board.get_mut(i as usize) {
                        c.dmg = c.hp;
                    }
                }
            }
            Bounce => {
                if let Tgt::Foe(i) = t {
                    let qs = &mut self.sides[q.idx()];
                    if (i as usize) < qs.board.len() {
                        let c = qs.board.remove(i as usize);
                        if qs.hand.len() >= MAXHAND {
                            qs.grave.push(c.card);
                        } else {
                            qs.hand.push(c.card);
                        }
                    }
                }
            }
        }
        self.reap();
        self.check_over(p);
    }

    fn targets(&self, p: Player, eff: Eff) -> Vec<Tgt> {
        let foe = &self.sides[p.other().idx()].board;
        let mine = &self.sides[p.idx()].board;
        match eff {
            DmgAny(_) => {
                let mut v: Vec<Tgt> = (0..foe.len()).map(|i| Tgt::Foe(i as u8)).collect();
                v.push(Tgt::Face);
                v
            }
            DmgCre(_) | Kill | Bounce => (0..foe.len()).map(|i| Tgt::Foe(i as u8)).collect(),
            Buff(..) => (0..mine.len()).map(|i| Tgt::Mine(i as u8)).collect(),
            _ => vec![Tgt::None],
        }
    }

    fn attacker_index(&self) -> Option<usize> {
        let a = self.turn.other();
        let uid = *self.attackers.get(self.att_idx)?;
        self.sides[a.idx()].board.iter().position(|c| c.uid == uid)
    }

    /// Indicii creaturilor care pot ataca acum (jucătorul la mutare).
    fn eligible_mask(&self) -> u8 {
        let mut m = 0u8;
        for (i, c) in self.sides[self.turn.idx()].board.iter().enumerate() {
            if !c.tapped && !c.fresh && !c.done && c.atk > 0 {
                m |= 1 << i;
            }
        }
        m
    }

    fn end_turn(&mut self) {
        for s in &mut self.sides {
            for c in &mut s.board {
                c.dmg = 0;
            }
        }
        let next = self.turn.other();
        self.start_turn(next);
    }

    fn finish_blocks(&mut self) {
        // Atacatorii s-au terminat: înapoi la faza principală a atacatorului.
        let a = self.turn.other();
        self.attackers.clear();
        self.used.clear();
        self.att_idx = 0;
        self.turn = a;
        self.phase = Phase::Main;
    }

    fn resolve_block(&mut self, blocker: Option<u8>) {
        let a = self.turn.other();
        let d = self.turn;
        let ai = match self.attacker_index() {
            Some(i) => i,
            Option::None => {
                self.att_idx += 1;
                return;
            }
        };
        match blocker {
            Option::None => {
                let atk = self.sides[a.idx()].board[ai].atk;
                if atk > 0 {
                    self.sides[d.idx()].life -= atk;
                }
            }
            Some(bi) => {
                let bi = bi as usize;
                let ac = self.sides[a.idx()].board[ai].clone();
                let bc = self.sides[d.idx()].board[bi].clone();
                self.used.push(bc.uid);
                let (da, db) = duel(&ac, &bc);
                deal(&mut self.sides[d.idx()].board[bi], da, ac.kw & VENIN != 0);
                deal(&mut self.sides[a.idx()].board[ai], db, bc.kw & VENIN != 0);
            }
        }
        self.reap();
        self.check_over(a);
        self.att_idx += 1;
    }

    fn blockers(&self) -> Vec<u8> {
        let a = self.turn.other();
        let Some(ai) = self.attacker_index() else {
            return Vec::new();
        };
        let att = &self.sides[a.idx()].board[ai];
        self.sides[self.turn.idx()]
            .board
            .iter()
            .enumerate()
            .filter(|(_, c)| can_block(att, c) && !self.used.contains(&c.uid))
            .map(|(i, _)| i as u8)
            .collect()
    }

    /// Valoarea (euristică) a unei creaturi de pe câmp.
    fn cval(c: &Creature) -> i32 {
        let mut v = 2 * c.atk as i32 + 2 * (c.hp - c.dmg).max(0) as i32;
        for (f, w) in [
            (ZBOR, 2),
            (ARMURA, 2),
            (VENIN, 3),
            (PRIMUL, 2),
            (STRAJER, 1),
        ] {
            if c.kw & f != 0 {
                v += w;
            }
        }
        v
    }

    fn lifeval(l: i16) -> i32 {
        if l <= 0 {
            -1000
        } else {
            5 * l.min(10) as i32 + 2 * (l - 10).max(0) as i32
        }
    }

    /// Variația de valoare (din punctul de vedere al atacatorului) când `a` este
    /// blocat de `b`.
    fn combat_delta(a: &Creature, b: &Creature) -> i32 {
        let (da, db) = duel(a, b);
        let mut v = 0;
        if lethal(a, b, da) {
            v += Self::cval(b);
        } else {
            v += 2 * da as i32;
        }
        if lethal(b, a, db) {
            v -= Self::cval(a);
        } else {
            v -= 2 * db as i32;
        }
        v
    }

    /// Valoarea așteptată a atacatorilor rămași (în favoarea atacatorului).
    fn pending_value(&self) -> i32 {
        let a = self.turn.other();
        let d = self.turn;
        let mut total = 0;
        for uid in self.attackers.iter().skip(self.att_idx) {
            let Some(att) = self.sides[a.idx()].board.iter().find(|c| c.uid == *uid) else {
                continue;
            };
            let foe_life = self.sides[d.idx()].life;
            let none = Self::lifeval(foe_life) - Self::lifeval(foe_life - att.atk);
            let mut best = none;
            for b in &self.sides[d.idx()].board {
                if can_block(att, b) && !self.used.contains(&b.uid) {
                    best = best.min(Self::combat_delta(att, b));
                }
            }
            total += best;
        }
        total
    }
}

impl Game for Tcg {
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
        match self.phase {
            Phase::Draw => vec![Move::Draw],
            Phase::Block => {
                let mut v = vec![Move::Block(Option::None)];
                for b in self.blockers() {
                    v.push(Move::Block(Some(b)));
                }
                v
            }
            Phase::Main => {
                let mut v = vec![Move::End];
                let s = &self.sides[self.turn.idx()];
                for (h, id) in s.hand.iter().enumerate() {
                    let c = &CARDS[*id as usize];
                    match c.kind {
                        Kind::Res => {
                            if !s.res_played && s.res.len() < MAXMANA {
                                v.push(Move::Play(h as u8, Tgt::None));
                            }
                        }
                        Kind::Cre => {
                            if c.cost <= s.mana && s.board.len() < MAXBOARD {
                                v.push(Move::Play(h as u8, Tgt::None));
                            }
                        }
                        Kind::Spell => {
                            if c.cost <= s.mana {
                                for t in self.targets(self.turn, c.eff) {
                                    v.push(Move::Play(h as u8, t));
                                }
                            }
                        }
                    }
                }
                let el = self.eligible_mask();
                let mut sub = el;
                while sub != 0 {
                    v.push(Move::Attack(sub));
                    sub = (sub - 1) & el;
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
                if self.turn_no > 1 && !self.draw_card(p) {
                    self.over = Some(Outcome::Win(p.other()));
                    return;
                }
                self.phase = Phase::Main;
            }
            Move::End => self.end_turn(),
            Move::Play(h, t) => {
                let id = self.sides[p.idx()].hand.remove(h as usize);
                let c = &CARDS[id as usize];
                match c.kind {
                    Kind::Res => {
                        let s = &mut self.sides[p.idx()];
                        s.res.push(id);
                        s.mana += 1;
                        s.res_played = true;
                        self.do_eff(p, c.eff, Tgt::None);
                    }
                    Kind::Cre => {
                        let uid = self.next_uid;
                        self.next_uid += 1;
                        let s = &mut self.sides[p.idx()];
                        s.mana -= c.cost;
                        s.board.push(Creature {
                            card: id,
                            uid,
                            atk: c.atk,
                            hp: c.hp,
                            dmg: 0,
                            kw: c.kw,
                            tapped: false,
                            fresh: c.kw & GRABA == 0,
                            done: false,
                        });
                        self.do_eff(p, c.eff, Tgt::None);
                    }
                    Kind::Spell => {
                        let s = &mut self.sides[p.idx()];
                        s.mana -= c.cost;
                        s.grave.push(id);
                        self.do_eff(p, c.eff, t);
                    }
                }
            }
            Move::Attack(mask) => {
                let mut uids = Vec::new();
                for (i, c) in self.sides[p.idx()].board.iter_mut().enumerate() {
                    if mask & (1 << i) != 0 {
                        c.done = true;
                        if c.kw & STRAJER == 0 {
                            c.tapped = true;
                        }
                        uids.push(c.uid);
                    }
                }
                self.attackers = uids;
                self.att_idx = 0;
                self.used.clear();
                self.turn = p.other();
                self.phase = Phase::Block;
            }
            Move::Block(b) => {
                self.resolve_block(b);
                if self.over.is_none() && self.att_idx >= self.attackers.len() {
                    self.finish_blocks();
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.over
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = &self.sides[p.idx()];
        let foe = &self.sides[p.other().idx()];
        let mut v = Self::lifeval(me.life) - Self::lifeval(foe.life);
        v += me.board.iter().map(Self::cval).sum::<i32>();
        v -= foe.board.iter().map(Self::cval).sum::<i32>();
        // Toate cărțile din mână au aceeași valoare: AI-ul nu „vede” ce trage.
        v += 3 * (me.hand.len() as i32 - foe.hand.len() as i32);
        v += 4 * (me.res.len() as i32 - foe.res.len() as i32);
        if self.phase == Phase::Block {
            let pv = self.pending_value();
            // `turn` = apărătorul; atacatorul este celălalt.
            if self.turn.other() == p {
                v += pv;
            } else {
                v -= pv;
            }
        }
        v
    }

    fn render(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            "Ținutul Cenușiu (cărți) — tura {} — {}\n",
            self.turn_no,
            match self.phase {
                Phase::Draw => "pescuire",
                Phase::Main => "faza principală",
                Phase::Block => "blocaj",
            }
        ));
        for p in [Player::Two, Player::One] {
            let sd = &self.sides[p.idx()];
            let who = if p == Player::One { 1 } else { 2 };
            s.push_str(&format!(
                "Jucătorul {who} ({}): viață {} | mană {}/{} | mână {} | pachet {} | cimitir {}\n",
                deck_name(p),
                sd.life,
                sd.mana,
                sd.res.len(),
                sd.hand.len(),
                sd.deck.len(),
                sd.grave.len()
            ));
            if sd.board.is_empty() {
                s.push_str("  Câmp: —\n");
            } else {
                for (i, c) in sd.board.iter().enumerate() {
                    let kw = kw_text(c.kw);
                    s.push_str(&format!(
                        "  [{}] {} {}/{}{}{}{}\n",
                        i + 1,
                        CARDS[c.card as usize].name,
                        c.atk,
                        c.hp - c.dmg,
                        if kw.is_empty() {
                            String::new()
                        } else {
                            format!(" ({kw})")
                        },
                        if c.tapped { " [înclinat]" } else { "" },
                        if c.fresh { " [nou]" } else { "" }
                    ));
                }
            }
        }
        if self.over.is_some() {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        let v = self.viewer.unwrap_or(self.turn);
        s.push_str(&format!("Mâna Jucătorului {}:\n", v.idx() + 1));
        for (i, id) in self.sides[v.idx()].hand.iter().enumerate() {
            s.push_str(&format!("  {}. {}\n", i + 1, card_line(*id)));
        }
        if self.phase == Phase::Block {
            if let Some(ai) = self.attacker_index() {
                let c = &self.sides[self.turn.other().idx()].board[ai];
                s.push_str(&format!(
                    "Atacă: {} {}/{} — alege blocant sau «fara».\n",
                    CARDS[c.card as usize].name,
                    c.atk,
                    c.hp - c.dmg
                ));
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
            Move::Play(h, t) => {
                let suf = match t {
                    Tgt::None => String::new(),
                    Tgt::Foe(i) => format!(" e{}", i + 1),
                    Tgt::Mine(i) => format!(" m{}", i + 1),
                    Tgt::Face => " fata".into(),
                };
                format!("joaca {}{}", h + 1, suf)
            }
            Move::Attack(mask) => {
                let v: Vec<String> = (0..8)
                    .filter(|i| mask & (1 << i) != 0)
                    .map(|i| (i + 1).to_string())
                    .collect();
                format!("ataca {}", v.join(","))
            }
            Move::Block(Option::None) => "fara".into(),
            Move::Block(Some(i)) => format!("bloc {}", i + 1),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn main_phase(seed: u64) -> Tcg {
        let mut g = Tcg::with_seed(seed);
        g.apply(&Move::Draw);
        g
    }

    fn creature(g: &mut Tcg, p: Player, name: &str, fresh: bool) -> usize {
        let id = card_id(name);
        let c = &CARDS[id as usize];
        let uid = g.next_uid;
        g.next_uid += 1;
        g.sides[p.idx()].board.push(Creature {
            card: id,
            uid,
            atk: c.atk,
            hp: c.hp,
            dmg: 0,
            kw: c.kw,
            tapped: false,
            fresh,
            done: false,
        });
        g.sides[p.idx()].board.len() - 1
    }

    fn total_cards(s: &Side) -> usize {
        s.deck.len() + s.hand.len() + s.res.len() + s.board.len() + s.grave.len()
    }

    #[test]
    fn set_has_sixty_original_cards_and_decks_have_thirty() {
        assert_eq!(CARDS.len(), 60);
        let mut names: Vec<&str> = CARDS.iter().map(|c| c.name).collect();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), 60);
        for p in [Player::One, Player::Two] {
            let d = deck_list(p);
            assert_eq!(d.len(), 30, "{}", deck_name(p));
            for id in 0..60u8 {
                assert!(d.iter().filter(|x| **x == id).count() <= 4);
            }
            let res = d
                .iter()
                .filter(|i| CARDS[**i as usize].kind == Kind::Res)
                .count();
            assert_eq!(res, 11);
        }
    }

    #[test]
    fn setup_deals_five_and_keeps_thirty() {
        let g = Tcg::with_seed(7);
        for s in &g.sides {
            assert_eq!(s.hand.len(), 5);
            assert_eq!(s.deck.len(), 25);
            assert_eq!(s.life, 20);
        }
        assert_eq!(g.legal_moves(), vec![Move::Draw]);
    }

    #[test]
    fn first_player_skips_first_draw_second_draws() {
        let mut g = Tcg::with_seed(2);
        g.apply(&Move::Draw);
        assert_eq!(g.sides[0].hand.len(), 5);
        g.apply(&Move::End);
        g.apply(&Move::Draw);
        assert_eq!(g.sides[1].hand.len(), 6);
    }

    #[test]
    fn one_resource_per_turn_and_mana_grows() {
        let mut g = main_phase(3);
        let r = card_id("Izvor de lumină");
        g.sides[0].hand = vec![r, r, card_id("Licurici")];
        let n_play = g
            .legal_moves()
            .iter()
            .filter(|m| matches!(m, Move::Play(..)))
            .count();
        assert_eq!(n_play, 2); // doar o resursă (idx 0 sau 1) + Licurici nu (mană 0)
        g.apply(&Move::Play(0, Tgt::None));
        assert_eq!(g.sides[0].mana, 1);
        let moves = g.legal_moves();
        assert!(moves.contains(&Move::Play(1, Tgt::None))); // Licurici costă 1
        assert!(!moves.contains(&Move::Play(0, Tgt::None)) || g.sides[0].hand[0] != r);
        // a doua resursă nu mai e legală
        g.sides[0].hand.push(r);
        let second_res = g
            .legal_moves()
            .iter()
            .any(|m| matches!(m, Move::Play(h, _) if g.sides[0].hand[*h as usize] == r));
        assert!(!second_res);
    }

    #[test]
    fn mana_cost_enforced() {
        let mut g = main_phase(4);
        g.sides[0].hand = vec![card_id("Golem de fier")];
        g.sides[0].mana = 4;
        assert!(!g.legal_moves().iter().any(|m| matches!(m, Move::Play(..))));
        g.sides[0].mana = 5;
        g.apply(&Move::Play(0, Tgt::None));
        assert_eq!(g.sides[0].mana, 0);
        assert_eq!(g.sides[0].board.len(), 1);
        assert!(g.sides[0].board[0].fresh);
    }

    #[test]
    fn summoning_sickness_and_haste() {
        let mut g = main_phase(5);
        creature(&mut g, Player::One, "Spadasin călit", true);
        creature(&mut g, Player::One, "Cercetaș iute", false);
        assert_eq!(g.eligible_mask(), 0b10);
    }

    #[test]
    fn flying_only_blocked_by_flyers() {
        let mut g = main_phase(6);
        creature(&mut g, Player::One, "Vultur de munte", false);
        creature(&mut g, Player::Two, "Lup cenușiu", false);
        creature(&mut g, Player::Two, "Cioară de pază", false);
        g.apply(&Move::Attack(1));
        assert_eq!(g.phase, Phase::Block);
        let mv = g.legal_moves();
        assert!(mv.contains(&Move::Block(Option::None)));
        assert!(mv.contains(&Move::Block(Some(1))));
        assert!(!mv.contains(&Move::Block(Some(0))));
    }

    #[test]
    fn unblocked_attack_hits_player_and_returns_to_attacker() {
        let mut g = main_phase(8);
        creature(&mut g, Player::One, "Urs de peșteră", false);
        g.apply(&Move::Attack(1));
        assert_eq!(g.turn, Player::Two);
        g.apply(&Move::Block(Option::None));
        assert_eq!(g.sides[1].life, 16);
        assert_eq!(g.turn, Player::One);
        assert_eq!(g.phase, Phase::Main);
        // a atacat deja: nu mai poate ataca
        assert_eq!(g.eligible_mask(), 0);
    }

    #[test]
    fn venom_kills_and_first_strike_avoids_return_damage() {
        let mut g = main_phase(9);
        creature(&mut g, Player::One, "Pumnal ascuns", false);
        creature(&mut g, Player::Two, "Urs de peșteră", false);
        g.apply(&Move::Attack(1));
        g.apply(&Move::Block(Some(0)));
        assert!(g.sides[1].board.is_empty(), "venin ucide ursul");
        assert!(g.sides[0].board.is_empty(), "pumnalul moare");

        let mut g = main_phase(9);
        creature(&mut g, Player::One, "Duelist", false);
        creature(&mut g, Player::Two, "Lup cenușiu", false);
        g.apply(&Move::Attack(1));
        g.apply(&Move::Block(Some(0)));
        assert!(g.sides[1].board.is_empty());
        assert_eq!(g.sides[0].board.len(), 1);
        assert_eq!(g.sides[0].board[0].dmg, 0, "Primul: fără daune înapoi");
    }

    #[test]
    fn armor_reduces_damage() {
        let mut g = main_phase(10);
        creature(&mut g, Player::One, "Ucenic de fierar", false); // 2/1
        creature(&mut g, Player::Two, "Gardian de piatră", false); // 1/4 armură
        g.apply(&Move::Attack(1));
        g.apply(&Move::Block(Some(0)));
        assert_eq!(g.sides[1].board[0].dmg, 1);
        assert!(g.sides[0].board.is_empty());
    }

    #[test]
    fn vigilance_does_not_tap() {
        let mut g = main_phase(11);
        creature(&mut g, Player::One, "Străjer de câmpie", false);
        g.apply(&Move::Attack(1));
        assert!(!g.sides[0].board[0].tapped);
    }

    #[test]
    fn spells_resolve_with_targets() {
        let mut g = main_phase(12);
        creature(&mut g, Player::Two, "Lup cenușiu", false);
        g.sides[0].hand = vec![card_id("Fulger"), card_id("Sărutul morții")];
        g.sides[0].mana = 6;
        let mv = g.legal_moves();
        assert!(mv.contains(&Move::Play(0, Tgt::Face)));
        assert!(mv.contains(&Move::Play(0, Tgt::Foe(0))));
        assert!(mv.contains(&Move::Play(1, Tgt::Foe(0))));
        assert!(!mv.contains(&Move::Play(1, Tgt::Face)));
        g.apply(&Move::Play(0, Tgt::Face));
        assert_eq!(g.sides[1].life, 17);
        g.apply(&Move::Play(0, Tgt::Foe(0)));
        assert!(g.sides[1].board.is_empty());
        assert_eq!(g.sides[0].grave.len(), 2);
    }

    #[test]
    fn bounce_and_heal_cap() {
        let mut g = main_phase(13);
        creature(&mut g, Player::Two, "Grifon", false);
        g.sides[0].hand = vec![card_id("Exil"), card_id("Vindecare")];
        g.sides[0].mana = 4;
        let before = g.sides[1].hand.len();
        g.apply(&Move::Play(0, Tgt::Foe(0)));
        assert_eq!(g.sides[1].hand.len(), before + 1);
        g.sides[0].life = 18;
        g.apply(&Move::Play(0, Tgt::None));
        assert_eq!(g.sides[0].life, 20);
    }

    #[test]
    fn lethal_damage_wins_and_deck_out_loses() {
        let mut g = main_phase(14);
        g.sides[1].life = 3;
        g.sides[0].hand = vec![card_id("Fulger")];
        g.sides[0].mana = 2;
        g.apply(&Move::Play(0, Tgt::Face));
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
        assert!(g.legal_moves().is_empty());

        let mut g = main_phase(14);
        g.sides[1].deck.clear();
        g.apply(&Move::End);
        g.apply(&Move::Draw);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn damage_wears_off_at_end_of_turn() {
        let mut g = main_phase(15);
        creature(&mut g, Player::Two, "Urs de peșteră", false);
        g.sides[0].hand = vec![card_id("Săgeată de foc")];
        g.sides[0].mana = 1;
        g.apply(&Move::Play(0, Tgt::Foe(0)));
        assert_eq!(g.sides[1].board[0].dmg, 2);
        g.apply(&Move::End);
        assert_eq!(g.sides[1].board[0].dmg, 0);
    }

    #[test]
    fn render_hides_opponent_hand() {
        let g = Tcg::with_seed(5);
        let r = g.render();
        assert!(r.contains("Mâna Jucătorului 1"));
        let mine: Vec<String> = g.sides[0].hand.iter().map(|i| card_line(*i)).collect();
        for l in &mine {
            assert!(r.contains(l.as_str()));
        }
        // cărți din mâna adversarului (care nu sunt și în mâna mea) nu apar
        for id in &g.sides[1].hand {
            if !g.sides[0].hand.contains(id) && CARDS[*id as usize].kind != Kind::Res {
                assert!(!r.contains(CARDS[*id as usize].name), "{r}");
            }
        }
    }

    #[test]
    fn cards_conserved_and_random_games_finish() {
        for seed in 1..=12u64 {
            let mut g = Tcg::with_seed(seed);
            let mut rng = Rng::new(seed * 31 + 1);
            let mut n = 0;
            while g.outcome().is_none() && n < 6000 {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
                for s in &g.sides {
                    assert_eq!(total_cards(s), 30);
                }
            }
            assert!(g.outcome().is_some(), "seed {seed} neterminat");
        }
    }

    #[test]
    fn ai_does_not_peek_at_hidden_cards() {
        // Două stări identice pentru AI (mâna lui J2 e aceeași), dar mâna și
        // pachetul lui J1 diferă: alegerea AI trebuie să fie aceeași.
        let mut a = Tcg::with_seed(21);
        a.apply(&Move::Draw);
        a.apply(&Move::End);
        a.apply(&Move::Draw);
        let mut b = a.clone();
        b.sides[0].hand = vec![card_id("Colos de ceață"); 5];
        b.sides[0].deck.reverse();
        let ma = polytabletop_core::ai::best_move(&a, 77).unwrap();
        let mb = polytabletop_core::ai::best_move(&b, 77).unwrap();
        assert_eq!(ma, mb);
    }

    #[test]
    fn ai_beats_random_most_of_the_time() {
        let mut wins = 0;
        let games = 10;
        for seed in 0..games {
            let mut g = Tcg::with_seed(100 + seed);
            let mut rng = Rng::new(seed + 5);
            let mut n = 0;
            while g.outcome().is_none() && n < 6000 {
                let m = if g.to_move() == Player::Two {
                    polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap()
                } else {
                    let mv = g.legal_moves();
                    mv[rng.below(mv.len())]
                };
                g.apply(&m);
                n += 1;
            }
            if g.outcome() == Some(Outcome::Win(Player::Two)) {
                wins += 1;
            }
        }
        assert!(wins >= 8, "AI a câștigat doar {wins}/{games}");
    }

    #[test]
    fn ai_move_is_fast() {
        let mut g = Tcg::with_seed(33);
        let mut rng = Rng::new(3);
        let t = std::time::Instant::now();
        let mut n = 0;
        while g.outcome().is_none() && n < 200 {
            let m = polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap();
            g.apply(&m);
            n += 1;
        }
        assert!(t.elapsed().as_secs_f64() / (n as f64) < 1.0);
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
        let mut g = Tcg::with_seed(5);
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
        let mut g = Tcg::with_seed(5);
        Game::set_viewer(&mut g, Option::None);
        let r = Game::render(&g);
        let n = Game::to_move(&g).idx() + 1;
        assert!(r.contains(&format!("Mâna Jucătorului {n}:")), "{r}");
        Game::set_viewer(&mut g, Some(Player::One));
        let a = Game::render(&g);
        Game::set_viewer(&mut g, Some(Player::Two));
        assert_ne!(a, Game::render(&g));
    }
}
