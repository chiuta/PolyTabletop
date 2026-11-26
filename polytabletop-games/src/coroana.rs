//! DRUMUL SPRE COROANĂ — aventură pe inele concentrice, om contra roboți (variantă originală).
//!
//! Trei aventurieri (tu = locul 1, `Player::One`; doi roboți = masa, `Player::Two`) pornesc din
//! **Câmpie** (inelul exterior, 16 câmpuri), trec prin **Podul Gri** în **Țara Cenușie** (inelul
//! din mijloc, 12 câmpuri), își câștigă **Pecetea** în Turnul Peceții, urcă **Scara Neagră** în
//! **Inelul de Jar** (8 câmpuri) și intră pe **Ușa Coroanei** în camera din centru. Cine stă în
//! cameră își folosește la începutul fiecărei ture **Porunca**: toți cei din afara camerei pierd
//! o Viață; cine ajunge la 0 Viață sub Poruncă e scos din joc. Dacă doi stau în cameră, se bat
//! între ei. Câștigi când ceilalți doi sunt scoși din joc.
//!
//! Tura: arunci zarul de mișcare (d6, +1 cu Cizmele Iuțe), alegi sensul (dreapta/stânga) și
//! rezolvi câmpul: cărți de aventură (monștri, obiecte, întâmplări, descântece), han, capelă,
//! târg, mlaștină, pădure, altar, tâlhari, lac de jar. Lupta: Forța ta + d6 trebuie să
//! DEPĂȘEASCĂ forța monstrului (egal = nimic). Cine pierde toată Viața în afara camerei „cade”:
//! pierde Aurul, un obiect și Pecetea, se trezește la Han cu Viață pe jumătate și pierde o tură.
//! Şase personaje, fiecare cu un avantaj. Toate cărțile și numele sunt originale.
//! Simplificări: fără aliați, fără alegerea obiectului de aruncat (se aruncă cel mai slab),
//! limită de 150 de runde (apoi câștigă cine e mai înaintat).

use crate::w5_util::{shuffle_vec, trim_log};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const OUT: usize = 16;
const MID: usize = 12;
const INN: usize = 8;
const ROUND_CAP: u32 = 150;
const HAND_CAP: usize = 5;
const NPL: usize = 3;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Ring {
    Outer,
    Middle,
    Inner,
    Crown,
}

impl Ring {
    fn len(self) -> usize {
        match self {
            Ring::Outer => OUT,
            Ring::Middle => MID,
            Ring::Inner => INN,
            Ring::Crown => 1,
        }
    }
    fn deck(self) -> usize {
        match self {
            Ring::Outer => 0,
            Ring::Middle => 1,
            _ => 2,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Pos {
    pub ring: Ring,
    pub idx: u8,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Sp {
    Adv,
    Han,
    Swamp,
    Market,
    Chapel,
    Forest,
    Robbers,
    Altar,
    Bridge,
    Cave,
    Tower,
    Stairs,
    Fire,
    Door,
    Crown,
}

const OUTER: [Sp; OUT] = [
    Sp::Bridge,
    Sp::Adv,
    Sp::Adv,
    Sp::Han,
    Sp::Adv,
    Sp::Swamp,
    Sp::Adv,
    Sp::Market,
    Sp::Adv,
    Sp::Chapel,
    Sp::Adv,
    Sp::Forest,
    Sp::Adv,
    Sp::Robbers,
    Sp::Adv,
    Sp::Altar,
];
const MIDDLE: [Sp; MID] = [
    Sp::Adv,
    Sp::Adv,
    Sp::Cave,
    Sp::Chapel,
    Sp::Adv,
    Sp::Tower,
    Sp::Adv,
    Sp::Altar,
    Sp::Adv,
    Sp::Swamp,
    Sp::Adv,
    Sp::Stairs,
];
const INNER: [Sp; INN] = [
    Sp::Adv,
    Sp::Fire,
    Sp::Adv,
    Sp::Adv,
    Sp::Fire,
    Sp::Adv,
    Sp::Adv,
    Sp::Door,
];

pub fn space(p: Pos) -> Sp {
    match p.ring {
        Ring::Outer => OUTER[p.idx as usize],
        Ring::Middle => MIDDLE[p.idx as usize],
        Ring::Inner => INNER[p.idx as usize],
        Ring::Crown => Sp::Crown,
    }
}

fn abbr(s: Sp) -> &'static str {
    match s {
        Sp::Adv => "Av",
        Sp::Han => "Han",
        Sp::Swamp => "Ml",
        Sp::Market => "Tg",
        Sp::Chapel => "Cp",
        Sp::Forest => "Pd",
        Sp::Robbers => "Ho",
        Sp::Altar => "Al",
        Sp::Bridge => "POD",
        Sp::Cave => "Pe",
        Sp::Tower => "TUR",
        Sp::Stairs => "SCA",
        Sp::Fire => "Ja",
        Sp::Door => "UȘA",
        Sp::Crown => "COR",
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Item {
    Sabie,
    Ghioaga,
    Toiag,
    Cizme,
    Scut,
    Fiola,
    Cheie,
    Armura,
    SabiaFoc,
}

impl Item {
    fn name(self) -> &'static str {
        match self {
            Item::Sabie => "Sabia Roșcată (+1 Forță)",
            Item::Ghioaga => "Ghioaga Mare (+2 Forță)",
            Item::Toiag => "Toiagul de Alun (+1 Meșteșug)",
            Item::Cizme => "Cizmele Iuțe (+1 la zar)",
            Item::Scut => "Scutul de Stejar (-1 daune)",
            Item::Fiola => "Fiola Vie (+2 Viață, o dată)",
            Item::Cheie => "Cheia de Aramă (trece Podul, o dată)",
            Item::Armura => "Armura de Solzi (+1 Forță)",
            Item::SabiaFoc => "Sabia de Foc (+3 Forță)",
        }
    }
    fn key(self) -> &'static str {
        match self {
            Item::Sabie => "sabie",
            Item::Ghioaga => "ghioaga",
            Item::Toiag => "toiag",
            Item::Cizme => "cizme",
            Item::Scut => "scut",
            Item::Fiola => "fiola",
            Item::Cheie => "cheie",
            Item::Armura => "armura",
            Item::SabiaFoc => "sabiafoc",
        }
    }
    fn force(self) -> i32 {
        match self {
            Item::Sabie | Item::Armura => 1,
            Item::Ghioaga => 2,
            Item::SabiaFoc => 3,
            _ => 0,
        }
    }
    fn price(self) -> i32 {
        match self {
            Item::Sabie | Item::Toiag | Item::Cizme | Item::Scut => 3,
            Item::Fiola => 2,
            Item::Cheie => 4,
            _ => 99,
        }
    }
}

const SHOP: [Item; 6] = [
    Item::Sabie,
    Item::Toiag,
    Item::Cizme,
    Item::Scut,
    Item::Fiola,
    Item::Cheie,
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Spell {
    Fulger,
    Ceata,
    Leac,
}

impl Spell {
    fn name(self) -> &'static str {
        match self {
            Spell::Fulger => "Descântec de Fulger (+3 în luptă)",
            Spell::Ceata => "Descântec de Ceață (eviți o întâlnire)",
            Spell::Leac => "Descântec de Leac (+2 Viață)",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Event {
    Gold2,
    Gold3,
    Thieves,
    Storm,
    Spring,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Mon(u8),
    It(Item),
    Sp(Spell),
    Ev(Event),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Card {
    pub kind: Kind,
    pub ring: u8,
}

const MON_OUT: [(&str, i32, i32); 10] = [
    ("Șobolan de Hrubă", 4, 1),
    ("Lup Flămând", 5, 1),
    ("Tâlhar de Drum", 5, 1),
    ("Spiriduș Hoț", 4, 1),
    ("Urs de Pădure", 6, 1),
    ("Bandit Călare", 6, 1),
    ("Strigoi de Mlaștină", 6, 2),
    ("Căpcăun Somnoros", 7, 2),
    ("Vulpe Vrăjită", 4, 1),
    ("Cioară Uriașă", 5, 1),
];
const MON_MID: [(&str, i32, i32); 8] = [
    ("Cavaler Fără Chip", 8, 2),
    ("Balaur de Nisip", 9, 2),
    ("Vârcolac", 8, 2),
    ("Mumă a Pădurii", 7, 1),
    ("Piatra Vie", 9, 2),
    ("Fantoma Turnului", 7, 1),
    ("Scorpion de Aramă", 8, 2),
    ("Șarpele Tăcut", 8, 1),
];
const MON_INN: [(&str, i32, i32); 7] = [
    ("Zmeul Cenușiu", 11, 2),
    ("Gardianul de Cenușă", 12, 2),
    ("Lespedea Ucigașă", 11, 2),
    ("Umbra Coroanei", 12, 2),
    ("Hidra Cărbunoasă", 13, 2),
    ("Mohorâtul", 10, 2),
    ("Păianjenul de Sticlă", 11, 2),
];

fn foe(ring: u8, i: u8) -> (&'static str, i32, i32) {
    match ring {
        0 => MON_OUT[i as usize],
        1 => MON_MID[i as usize],
        _ => MON_INN[i as usize],
    }
}

fn build_deck(ring: u8) -> Vec<Card> {
    let mut v = Vec::new();
    let mut add = |k: Kind| v.push(Card { kind: k, ring });
    match ring {
        0 => {
            for i in 0..10 {
                add(Kind::Mon(i));
            }
            for it in [
                Item::Sabie,
                Item::Toiag,
                Item::Cizme,
                Item::Scut,
                Item::Fiola,
                Item::Cheie,
            ] {
                add(Kind::It(it));
            }
            for e in [
                Event::Gold2,
                Event::Gold3,
                Event::Thieves,
                Event::Storm,
                Event::Spring,
            ] {
                add(Kind::Ev(e));
            }
            for s in [Spell::Fulger, Spell::Ceata, Spell::Leac] {
                add(Kind::Sp(s));
            }
        }
        1 => {
            for i in 0..8 {
                add(Kind::Mon(i));
            }
            for it in [
                Item::Ghioaga,
                Item::Armura,
                Item::Fiola,
                Item::Scut,
                Item::Sabie,
            ] {
                add(Kind::It(it));
            }
            for e in [Event::Gold3, Event::Storm, Event::Spring] {
                add(Kind::Ev(e));
            }
            for s in [Spell::Fulger, Spell::Leac] {
                add(Kind::Sp(s));
            }
        }
        _ => {
            for i in 0..7 {
                add(Kind::Mon(i));
            }
            for it in [Item::SabiaFoc, Item::Fiola] {
                add(Kind::It(it));
            }
            for e in [Event::Spring, Event::Gold3] {
                add(Kind::Ev(e));
            }
            add(Kind::Sp(Spell::Fulger));
        }
    }
    v
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Ch {
    Haiducul,
    Vrajitoarea,
    Negustorul,
    Padurarul,
    Pelerinul,
    Calauza,
}

const CHARS: [Ch; 6] = [
    Ch::Haiducul,
    Ch::Vrajitoarea,
    Ch::Negustorul,
    Ch::Padurarul,
    Ch::Pelerinul,
    Ch::Calauza,
];

impl Ch {
    fn name(self) -> &'static str {
        match self {
            Ch::Haiducul => "Haiducul",
            Ch::Vrajitoarea => "Vrăjitoarea",
            Ch::Negustorul => "Negustorul",
            Ch::Padurarul => "Pădurarul",
            Ch::Pelerinul => "Pelerinul",
            Ch::Calauza => "Călăuza",
        }
    }
    fn key(self) -> &'static str {
        match self {
            Ch::Haiducul => "haiducul",
            Ch::Vrajitoarea => "vrajitoarea",
            Ch::Negustorul => "negustorul",
            Ch::Padurarul => "padurarul",
            Ch::Pelerinul => "pelerinul",
            Ch::Calauza => "calauza",
        }
    }
    fn advantage(self) -> &'static str {
        match self {
            Ch::Haiducul => "iei 1 Aur după fiecare monstru învins",
            Ch::Vrajitoarea => "o dată pe tură poți re-arunca zarul de mișcare",
            Ch::Negustorul => "obiectele de la târg costă cu 1 Aur mai puțin",
            Ch::Padurarul => "fuga reușește mereu; Pădurea nu-l rănește",
            Ch::Pelerinul => "la Capelă se vindecă complet; +2 la Altar",
            Ch::Calauza => "poate muta cu un câmp în plus sau în minus",
        }
    }
    /// (forță, meșteșug, viață, aur, câmp de start în Câmpie)
    fn stats(self) -> (i32, i32, i32, i32, u8) {
        match self {
            Ch::Haiducul => (4, 2, 5, 1, 4),
            Ch::Vrajitoarea => (2, 4, 4, 1, 6),
            Ch::Negustorul => (3, 3, 4, 4, 7),
            Ch::Padurarul => (3, 3, 5, 1, 11),
            Ch::Pelerinul => (2, 4, 5, 1, 9),
            Ch::Calauza => (3, 3, 4, 1, 2),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Pl {
    pub ch: Ch,
    pub f: i32,
    pub m: i32,
    pub v: i32,
    pub vmax: i32,
    pub gold: i32,
    pub pos: Pos,
    pub hand: Vec<Card>,
    pub trophies: Vec<Card>,
    pub seal: bool,
    pub out: bool,
    pub skip: u8,
    pub reroll: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Src {
    Card(Card),
    Cave,
    Guard,
    Tower(bool),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Enc {
    pub name: &'static str,
    pub s: i32,
    pub dmg: i32,
    pub src: Src,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Act {
    Fight(bool),
    Flee,
    Mist,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Phase {
    Pick,
    Start,
    Dir,
    Land,
    Enc,
    EncRoll,
    Shop,
    Pass,
    Robot,
    Over,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Pick(u8),
    Roll,
    Drink,
    Heal,
    UpPay,
    UpKey,
    UpFight,
    Up,
    AssaultF,
    AssaultC,
    Enter,
    Hold,
    Dir(bool, i8),
    Reroll,
    Fight,
    FightBolt,
    Flee,
    Mist,
    Buy(Item),
    Leave,
    Table,
}

#[derive(Clone)]
pub struct Coroana {
    pl: Vec<Pl>,
    decks: [Vec<Card>; 3],
    discard: [Vec<Card>; 3],
    base_total: usize,
    minted: usize,
    cur: usize,
    round: u32,
    phase: Phase,
    roll: i32,
    enc: Option<Enc>,
    act: Act,
    winner: Option<usize>,
    capped: bool,
    rng: Rng,
    log: Vec<String>,
}

impl Default for Coroana {
    fn default() -> Self {
        Self::new()
    }
}

fn win_count(power: i32, s: i32) -> i32 {
    (6 - s + power).clamp(0, 6)
}

impl Coroana {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut decks = [build_deck(0), build_deck(1), build_deck(2)];
        for d in decks.iter_mut() {
            shuffle_vec(d, &mut rng);
        }
        let base_total = decks.iter().map(|d| d.len()).sum();
        let mut g = Coroana {
            pl: Vec::new(),
            decks,
            discard: [Vec::new(), Vec::new(), Vec::new()],
            base_total,
            minted: 0,
            cur: 0,
            round: 1,
            phase: Phase::Pick,
            roll: 0,
            enc: None,
            act: Act::Flee,
            winner: None,
            capped: false,
            rng,
            log: Vec::new(),
        };
        g.say("Alege-ți personajul: «alege haiducul» (sau vrajitoarea, negustorul, padurarul, pelerinul, calauza).".into());
        g
    }

    pub fn players(&self) -> &[Pl] {
        &self.pl
    }
    pub fn phase(&self) -> Phase {
        self.phase
    }
    pub fn round(&self) -> u32 {
        self.round
    }
    pub fn winner(&self) -> Option<usize> {
        self.winner
    }
    pub fn was_capped(&self) -> bool {
        self.capped
    }
    pub fn total_cards(&self) -> usize {
        let mut n: usize = self.decks.iter().map(|d| d.len()).sum();
        n += self.discard.iter().map(|d| d.len()).sum::<usize>();
        for p in &self.pl {
            n += p.hand.len() + p.trophies.len();
        }
        if let Some(Enc {
            src: Src::Card(_), ..
        }) = self.enc
        {
            n += 1;
        }
        n
    }
    pub fn expected_cards(&self) -> usize {
        self.base_total + self.minted
    }

    fn say(&mut self, s: String) {
        self.log.push(s);
        trim_log(&mut self.log, 18);
    }
    fn who(&self, p: usize) -> String {
        match p {
            0 => "Tu".into(),
            n => format!("Robotul {}", if n == 1 { "A" } else { "B" }),
        }
    }

    // ---------- statistici ----------
    fn power(&self, p: usize) -> i32 {
        let pl = &self.pl[p];
        pl.f + pl
            .hand
            .iter()
            .map(|c| match c.kind {
                Kind::It(i) => i.force(),
                _ => 0,
            })
            .sum::<i32>()
    }
    fn craft(&self, p: usize) -> i32 {
        let pl = &self.pl[p];
        pl.m + pl
            .hand
            .iter()
            .filter(|c| c.kind == Kind::It(Item::Toiag))
            .count()
            .min(1) as i32
    }
    fn has_item(&self, p: usize, it: Item) -> bool {
        self.pl[p].hand.iter().any(|c| c.kind == Kind::It(it))
    }
    fn has_spell(&self, p: usize, s: Spell) -> bool {
        self.pl[p].hand.iter().any(|c| c.kind == Kind::Sp(s))
    }

    fn progress(&self, p: usize) -> i32 {
        let pl = &self.pl[p];
        let r = match pl.pos.ring {
            Ring::Outer => 0,
            Ring::Middle => 10,
            Ring::Inner => 20,
            Ring::Crown => 40,
        };
        r + if pl.seal { 6 } else { 0 } + self.power(p) + pl.v + if pl.out { -100 } else { 0 }
    }

    // ---------- cărți ----------
    fn draw(&mut self, ring: usize) -> Card {
        if self.decks[ring].is_empty() {
            let mut d = std::mem::take(&mut self.discard[ring]);
            shuffle_vec(&mut d, &mut self.rng);
            self.decks[ring] = d;
        }
        self.decks[ring]
            .pop()
            .expect("pachet gol: nu se poate întâmpla (pachetele nu se golesc complet)")
    }
    fn bin_card(&mut self, c: Card) {
        self.discard[(c.ring as usize).min(2)].push(c);
    }
    fn keep_value(c: &Card) -> i32 {
        match c.kind {
            Kind::It(Item::SabiaFoc) => 10,
            Kind::It(Item::Ghioaga) => 8,
            Kind::It(Item::Armura) => 7,
            Kind::It(Item::Sabie) => 6,
            Kind::It(Item::Scut) | Kind::It(Item::Cizme) => 5,
            Kind::It(Item::Toiag) | Kind::Sp(Spell::Fulger) => 4,
            Kind::It(Item::Fiola) | Kind::Sp(_) => 3,
            _ => 2,
        }
    }
    fn give(&mut self, p: usize, c: Card) {
        let nm = match c.kind {
            Kind::It(i) => i.name().to_string(),
            Kind::Sp(s) => s.name().to_string(),
            _ => String::new(),
        };
        let w = self.who(p);
        self.say(format!("{w}: primești {nm}."));
        self.pl[p].hand.push(c);
        if self.pl[p].hand.len() > HAND_CAP {
            let mut wi = 0;
            for (i, h) in self.pl[p].hand.iter().enumerate() {
                if Self::keep_value(h) < Self::keep_value(&self.pl[p].hand[wi]) {
                    wi = i;
                }
            }
            let d = self.pl[p].hand.remove(wi);
            self.say(format!("{w}: arunci un obiect de prisos."));
            self.bin_card(d);
        }
    }
    fn consume(&mut self, p: usize, k: Kind) -> bool {
        if let Some(i) = self.pl[p].hand.iter().position(|c| c.kind == k) {
            let c = self.pl[p].hand.remove(i);
            self.bin_card(c);
            true
        } else {
            false
        }
    }

    // ---------- viață, cădere, încoronare ----------
    fn heal(&mut self, p: usize, n: i32) {
        let pl = &mut self.pl[p];
        pl.v = (pl.v + n).min(pl.vmax);
    }
    fn lose_life(&mut self, p: usize, n: i32) {
        self.pl[p].v -= n;
        if self.pl[p].v <= 0 {
            self.fall(p);
        }
    }
    fn fall(&mut self, p: usize) {
        let w = self.who(p);
        self.say(format!(
            "{w} cade la pământ: pierde Aurul, un obiect și Pecetea; se trezește la Han."
        ));
        let pl = &mut self.pl[p];
        pl.gold = 0;
        pl.seal = false;
        pl.v = (pl.vmax / 2).max(2);
        pl.pos = Pos {
            ring: Ring::Outer,
            idx: 3,
        };
        pl.skip = 1;
        if !pl.hand.is_empty() {
            let i = self.rng.below(self.pl[p].hand.len());
            let c = self.pl[p].hand.remove(i);
            self.bin_card(c);
        }
    }
    fn crown_hit(&mut self, p: usize, n: i32) {
        self.pl[p].v -= n;
        if self.pl[p].v <= 0 {
            self.pl[p].v = 0;
            self.pl[p].out = true;
            let w = self.who(p);
            self.say(format!("{w} e scos din joc de puterea Coroanei."));
        }
    }
    fn crown_turn(&mut self, p: usize) {
        let w = self.who(p);
        self.say(format!("{w} rostește Porunca din camera Coroanei."));
        for q in 0..NPL {
            if q == p || self.pl[q].out {
                continue;
            }
            if self.pl[q].pos.ring != Ring::Crown {
                self.crown_hit(q, 1);
            } else {
                let a = self.power(p) + self.rng.d6() as i32;
                let b = self.power(q) + self.rng.d6() as i32;
                let (pn, qn) = (self.who(p), self.who(q));
                if a > b {
                    self.say(format!(
                        "Duel în cameră: {pn} ({a}) îl lovește pe {qn} ({b})."
                    ));
                    self.crown_hit(q, 1);
                } else if b > a {
                    self.say(format!(
                        "Duel în cameră: {qn} ({b}) îl lovește pe {pn} ({a})."
                    ));
                    self.crown_hit(p, 1);
                } else {
                    self.say("Duel în cameră: egalitate.".into());
                }
            }
        }
    }
    fn check_over(&mut self) -> bool {
        if self.winner.is_some() {
            self.phase = Phase::Over;
            return true;
        }
        let w = if self.pl[0].out {
            Some(if self.pl[1].out { 2 } else { 1 })
        } else if self.pl[1].out && self.pl[2].out {
            Some(0)
        } else {
            None
        };
        if let Some(w) = w {
            self.winner = Some(w);
            self.say(format!("{} a rămas singur sub Coroană.", self.who(w)));
            self.phase = Phase::Over;
            return true;
        }
        false
    }

    // ---------- tură ----------
    fn end_turn(&mut self) {
        if self.check_over() {
            return;
        }
        loop {
            self.cur = (self.cur + 1) % NPL;
            if self.cur == 0 {
                self.round += 1;
                if self.round > ROUND_CAP {
                    self.finish_cap();
                    return;
                }
            }
            if self.pl[self.cur].out {
                continue;
            }
            if self.pl[self.cur].skip > 0 {
                self.pl[self.cur].skip -= 1;
                let w = self.who(self.cur);
                self.say(format!("{w} pierde tura (se odihnește)."));
                continue;
            }
            break;
        }
        self.pl[self.cur].reroll = false;
        self.begin_turn();
    }

    fn begin_turn(&mut self) {
        let p = self.cur;
        if self.pl[p].pos.ring == Ring::Crown {
            self.crown_turn(p);
            if self.check_over() {
                return;
            }
        }
        self.phase = if p == 0 { Phase::Start } else { Phase::Robot };
    }

    fn finish_cap(&mut self) {
        self.capped = true;
        let mut best = 0;
        for p in 1..NPL {
            if self.progress(p) > self.progress(best) {
                best = p;
            }
        }
        // egalitate: câștigă un robot (omul trebuie să fie strict mai înaintat)
        if best == 0 {
            for p in 1..NPL {
                if self.progress(p) == self.progress(0) {
                    best = p;
                }
            }
        }
        self.winner = Some(best);
        self.say("Au trecut 150 de runde: câștigă cel mai înaintat.".into());
        self.phase = Phase::Over;
    }

    fn move_roll(&mut self, p: usize) -> i32 {
        let mut r = self.rng.d6() as i32;
        if self.has_item(p, Item::Cizme) {
            r += 1;
        }
        r
    }

    fn step(&mut self, p: usize, cw: bool, steps: i32) {
        let pos = self.pl[p].pos;
        let len = pos.ring.len() as i32;
        let d = if cw { steps } else { -steps };
        let idx = (pos.idx as i32 + d).rem_euclid(len);
        self.pl[p].pos.idx = idx as u8;
        let w = self.who(p);
        self.say(format!(
            "{w} mută {} câmpuri spre {}: {} {}.",
            steps,
            if cw { "dreapta" } else { "stânga" },
            abbr(space(self.pl[p].pos)),
            idx
        ));
    }

    // ---------- câmpuri ----------
    /// Rezolvă câmpul pe care a ajuns `p`. Întoarce întâlnirea/târgul care cere o decizie.
    fn land(&mut self, p: usize) -> Option<Phase> {
        let pos = self.pl[p].pos;
        let w = self.who(p);
        match space(pos) {
            Sp::Adv => {
                let c = self.draw(pos.ring.deck());
                match c.kind {
                    Kind::Mon(i) => {
                        let (name, s, dmg) = foe(c.ring, i);
                        self.say(format!("{w}: apare {name} (forța {s})!"));
                        self.enc = Some(Enc {
                            name,
                            s,
                            dmg,
                            src: Src::Card(c),
                        });
                        return Some(Phase::Enc);
                    }
                    Kind::It(_) | Kind::Sp(_) => self.give(p, c),
                    Kind::Ev(e) => {
                        self.event(p, e);
                        self.bin_card(c);
                    }
                }
            }
            Sp::Han => {
                if self.pl[p].gold >= 1 && self.pl[p].v < self.pl[p].vmax {
                    self.pl[p].gold -= 1;
                    self.heal(p, 2);
                    self.say(format!("{w}: la Han plătești 1 Aur și te vindeci cu 2."));
                }
            }
            Sp::Chapel => {
                if self.pl[p].ch == Ch::Pelerinul {
                    let m = self.pl[p].vmax;
                    self.pl[p].v = m;
                    self.say(format!("{w}: la Capelă te vindeci complet."));
                } else {
                    self.heal(p, 1);
                    self.say(format!("{w}: la Capelă te vindeci cu 1."));
                }
            }
            Sp::Swamp => {
                let d = self.rng.d6();
                if d <= 2 {
                    self.say(format!("{w}: mlaștina te trage la fund (-1 Viață)."));
                    self.lose_life(p, 1);
                } else if d >= 5 {
                    self.pl[p].gold += 1;
                    self.say(format!("{w}: găsești 1 Aur în mlaștină."));
                }
            }
            Sp::Forest => {
                let d = self.rng.d6();
                if d == 1 && self.pl[p].ch != Ch::Padurarul {
                    self.say(format!("{w}: te rătăcești în pădure (-1 Viață)."));
                    self.lose_life(p, 1);
                } else if d == 6 {
                    self.pl[p].gold += 2;
                    self.say(format!("{w}: găsești 2 Aur în pădure."));
                }
            }
            Sp::Robbers => {
                let l = self.pl[p].gold.min(2);
                self.pl[p].gold -= l;
                self.say(format!("{w}: tâlharii te prădează de {l} Aur."));
            }
            Sp::Altar => {
                let d = self.rng.d6() as i32;
                let t = d + self.craft(p) + if self.pl[p].ch == Ch::Pelerinul { 2 } else { 0 };
                if t >= 9 {
                    self.pl[p].m += 1;
                    self.say(format!("{w}: altarul te binecuvântează (+1 Meșteșug)."));
                } else if t <= 3 {
                    self.say(format!("{w}: altarul te arde (-1 Viață)."));
                    self.lose_life(p, 1);
                }
            }
            Sp::Fire => {
                if self.rng.d6() <= 3 {
                    self.say(format!("{w}: lacul de jar te frige (-1 Viață)."));
                    self.lose_life(p, 1);
                }
            }
            Sp::Market => {
                self.say(format!("{w}: ajungi la târg."));
                return Some(Phase::Shop);
            }
            Sp::Cave => {
                self.say(format!("{w}: din peșteră iese o sălbăticiune (forța 9)!"));
                self.enc = Some(Enc {
                    name: "Sălbăticiunea din Peșteră",
                    s: 9,
                    dmg: 2,
                    src: Src::Cave,
                });
                return Some(Phase::Enc);
            }
            Sp::Bridge | Sp::Tower | Sp::Stairs | Sp::Door | Sp::Crown => {}
        }
        None
    }

    fn event(&mut self, p: usize, e: Event) {
        let w = self.who(p);
        match e {
            Event::Gold2 => {
                self.pl[p].gold += 2;
                self.say(format!("{w}: găsești o pungă cu 2 Aur."));
            }
            Event::Gold3 => {
                self.pl[p].gold += 3;
                self.say(format!("{w}: găsești o pungă cu 3 Aur."));
            }
            Event::Thieves => {
                let l = self.pl[p].gold.min(2);
                self.pl[p].gold -= l;
                self.say(format!("{w}: hoții te fură de {l} Aur."));
            }
            Event::Storm => {
                self.say(format!("{w}: furtună rece (-1 Viață)."));
                self.lose_life(p, 1);
            }
            Event::Spring => {
                self.heal(p, 2);
                self.say(format!("{w}: bei dintr-un izvor (+2 Viață)."));
            }
        }
    }

    /// Rezolvă acțiunea asupra întâlnirii curente.
    fn resolve_enc(&mut self, p: usize, act: Act) {
        let enc = match self.enc.take() {
            Some(e) => e,
            None => return,
        };
        let w = self.who(p);
        let mut won = false;
        match act {
            Act::Mist => {
                self.consume(p, Kind::Sp(Spell::Ceata));
                self.say(format!("{w}: ceața te ascunde de {}.", enc.name));
            }
            Act::Flee => {
                let ok = self.pl[p].ch == Ch::Padurarul || self.rng.d6() >= 4;
                if ok {
                    self.say(format!("{w}: scapi fugind de {}.", enc.name));
                } else {
                    self.say(format!("{w}: {} te prinde din urmă (-1 Viață).", enc.name));
                    self.lose_life(p, 1);
                }
            }
            Act::Fight(bolt) => {
                let mut bonus = 0;
                if bolt && self.consume(p, Kind::Sp(Spell::Fulger)) {
                    bonus = 3;
                }
                let d = self.rng.d6() as i32;
                let tot = self.power(p) + d + bonus;
                if tot > enc.s {
                    won = true;
                    self.say(format!(
                        "{w}: lupți cu {} ({tot} contra {}) și câștigi!",
                        enc.name, enc.s
                    ));
                } else if tot == enc.s {
                    self.say(format!(
                        "{w}: lupți cu {} ({tot} contra {}): nimeni nu cedează.",
                        enc.name, enc.s
                    ));
                } else {
                    let mut dmg = enc.dmg;
                    if self.has_item(p, Item::Scut) {
                        dmg = (dmg - 1).max(1);
                    }
                    self.say(format!(
                        "{w}: lupți cu {} ({tot} contra {}) și pierzi (-{dmg} Viață).",
                        enc.name, enc.s
                    ));
                    self.lose_life(p, dmg);
                }
            }
        }
        if won {
            match enc.src {
                Src::Card(c) => {
                    self.pl[p].trophies.push(c);
                    self.pl[p].gold += 1;
                    let sum: i32 = self.pl[p]
                        .trophies
                        .iter()
                        .map(|t| match t.kind {
                            Kind::Mon(i) => foe(t.ring, i).1,
                            _ => 0,
                        })
                        .sum();
                    if sum >= 7 {
                        let ts = std::mem::take(&mut self.pl[p].trophies);
                        for t in ts {
                            self.bin_card(t);
                        }
                        self.pl[p].f += 1;
                        self.say(format!("{w}: trofeele te fac mai puternic (+1 Forță)."));
                    }
                    if self.pl[p].ch == Ch::Haiducul {
                        self.pl[p].gold += 1;
                    }
                }
                Src::Cave => self.pl[p].gold += 3,
                Src::Guard => {
                    self.pl[p].pos = Pos {
                        ring: Ring::Middle,
                        idx: 0,
                    };
                    self.say(format!("{w}: străbați Podul Gri în Țara Cenușie."));
                }
                Src::Tower(_) => {
                    self.pl[p].seal = true;
                    self.say(format!("{w}: câștigi Pecetea din Turnul Peceții!"));
                }
            }
            if matches!(enc.src, Src::Cave) {
                self.say(format!("{w}: iei 3 Aur din peșteră."));
            }
        } else if let Src::Card(c) = enc.src {
            self.bin_card(c);
        }
    }

    // ---------- roboți ----------
    fn ready_for_middle(&self, p: usize) -> bool {
        self.power(p) >= 5 || self.round > 25
    }

    fn target(&self, p: usize) -> Option<u8> {
        let pl = &self.pl[p];
        match pl.pos.ring {
            Ring::Outer => {
                if self.ready_for_middle(p) {
                    Some(0)
                } else if pl.gold >= 3 {
                    Some(7)
                } else {
                    None
                }
            }
            Ring::Middle => {
                if pl.seal {
                    Some(11)
                } else if self.power(p) >= 6 || self.round > 45 {
                    Some(5)
                } else {
                    None
                }
            }
            Ring::Inner => Some(7),
            Ring::Crown => None,
        }
    }

    fn robot_heal(&mut self, p: usize) {
        if self.pl[p].v <= 2 {
            if self.has_item(p, Item::Fiola) {
                self.consume(p, Kind::It(Item::Fiola));
                self.heal(p, 2);
                let w = self.who(p);
                self.say(format!("{w} bea Fiola Vie."));
            } else if self.has_spell(p, Spell::Leac) {
                self.consume(p, Kind::Sp(Spell::Leac));
                self.heal(p, 2);
                let w = self.who(p);
                self.say(format!("{w} rostește Descântecul de Leac."));
            }
        }
    }

    fn robot_buy(&mut self, p: usize) {
        let order = [
            Item::Sabie,
            Item::Scut,
            Item::Cizme,
            Item::Fiola,
            Item::Toiag,
            Item::Cheie,
        ];
        for _ in 0..2 {
            for it in order {
                if it == Item::Cheie
                    && (self.has_item(p, Item::Cheie) || self.pl[p].pos.ring != Ring::Outer)
                {
                    continue;
                }
                if (it == Item::Sabie || it == Item::Scut || it == Item::Cizme || it == Item::Toiag)
                    && self.has_item(p, it)
                {
                    continue;
                }
                if it == Item::Fiola && self.has_item(p, Item::Fiola) {
                    continue;
                }
                if self.buy(p, it) {
                    break;
                }
            }
        }
    }

    fn price(&self, p: usize, it: Item) -> i32 {
        let base = it.price();
        if self.pl[p].ch == Ch::Negustorul {
            (base - 1).max(1)
        } else {
            base
        }
    }

    fn buy(&mut self, p: usize, it: Item) -> bool {
        let pr = self.price(p, it);
        if self.pl[p].gold < pr {
            return false;
        }
        self.pl[p].gold -= pr;
        self.minted += 1;
        let c = Card {
            kind: Kind::It(it),
            ring: 0,
        };
        let w = self.who(p);
        self.say(format!("{w} cumpără {} pentru {pr} Aur.", it.name()));
        self.give(p, c);
        true
    }

    fn robot_choose_act(&self, p: usize, enc: &Enc) -> Act {
        let pw = self.power(p);
        let wc = win_count(pw, enc.s);
        let v = self.pl[p].v;
        let bolt = self.has_spell(p, Spell::Fulger);
        let wc3 = win_count(pw + 3, enc.s);
        let danger = v <= enc.dmg;
        if let Src::Card(_) | Src::Cave = enc.src {
            if !danger && wc >= if enc.dmg >= 2 { 3 } else { 2 } {
                return Act::Fight(bolt && wc < 4 && wc3 >= 4);
            }
            if bolt && wc3 >= 4 && !danger {
                return Act::Fight(true);
            }
            if self.has_spell(p, Spell::Ceata) {
                return Act::Mist;
            }
            if danger && wc >= 5 {
                return Act::Fight(false);
            }
            return Act::Flee;
        }
        Act::Flee
    }

    fn robot_turn(&mut self, p: usize) {
        self.robot_heal(p);
        let pos = self.pl[p].pos;
        let seal = self.pl[p].seal;
        match space(pos) {
            Sp::Crown => {
                return;
            }
            Sp::Door if seal => {
                self.enter_crown(p);
                return;
            }
            Sp::Stairs if seal => {
                self.ascend(p);
                return;
            }
            Sp::Tower if !seal => {
                let pf = win_count(self.power(p), 10);
                let pc = win_count(self.craft(p) + 1, 9);
                let bolt = self.has_spell(p, Spell::Fulger);
                if pf >= 3 || (bolt && win_count(self.power(p) + 3, 10) >= 4) {
                    let b = bolt && pf < 4;
                    self.assault(p, false, b);
                    return;
                } else if pc >= 3 {
                    self.assault(p, true, false);
                    return;
                }
            }
            Sp::Bridge => {
                if self.has_item(p, Item::Cheie) {
                    self.consume(p, Kind::It(Item::Cheie));
                    self.say(format!(
                        "{} descuie Podul Gri cu Cheia de Aramă.",
                        self.who(p)
                    ));
                    self.pl[p].pos = Pos {
                        ring: Ring::Middle,
                        idx: 0,
                    };
                    return;
                }
                if self.pl[p].gold >= 2 && self.ready_for_middle(p) {
                    self.pl[p].gold -= 2;
                    self.say(format!("{} plătește vama Podului Gri.", self.who(p)));
                    self.pl[p].pos = Pos {
                        ring: Ring::Middle,
                        idx: 0,
                    };
                    return;
                }
                if self.ready_for_middle(p) && win_count(self.power(p), 7) >= 3 {
                    self.enc = Some(Enc {
                        name: "Paznicul Podului",
                        s: 7,
                        dmg: 1,
                        src: Src::Guard,
                    });
                    self.resolve_enc(p, Act::Fight(false));
                    return;
                }
            }
            _ => {}
        }
        // mutare
        let roll = {
            let mut r = self.move_roll(p);
            if self.pl[p].ch == Ch::Vrajitoarea && r <= 2 {
                let r2 = self.move_roll(p);
                if r2 > r {
                    r = r2;
                }
            }
            r
        };
        let len = pos.ring.len() as i32;
        let mut cw = self.rng.below(2) == 0;
        let mut steps = roll;
        if let Some(t) = self.target(p) {
            let dist = |idx: i32| {
                let d = (idx - t as i32).rem_euclid(len);
                d.min(len - d)
            };
            let mut best = (i32::MAX, cw, steps);
            let deltas: &[i32] = if self.pl[p].ch == Ch::Calauza {
                &[0, -1, 1]
            } else {
                &[0]
            };
            for &dl in deltas {
                let s = roll + dl;
                if s < 1 {
                    continue;
                }
                for c in [true, false] {
                    let idx = (pos.idx as i32 + if c { s } else { -s }).rem_euclid(len);
                    let d = dist(idx) * 2 + (self.rng.below(2) as i32);
                    if d < best.0 {
                        best = (d, c, s);
                    }
                }
            }
            cw = best.1;
            steps = best.2;
        }
        self.step(p, cw, steps);
        let before_ring = self.pl[p].pos.ring;
        let _ = before_ring;
        match self.land(p) {
            Some(Phase::Enc) => {
                let enc = self.enc.unwrap();
                let act = self.robot_choose_act(p, &enc);
                self.resolve_enc(p, act);
            }
            Some(Phase::Shop) => self.robot_buy(p),
            _ => {}
        }
    }

    fn enter_crown(&mut self, p: usize) {
        self.pl[p].pos = Pos {
            ring: Ring::Crown,
            idx: 0,
        };
        let w = self.who(p);
        self.say(format!("{w} pătrunde în camera Coroanei!"));
    }
    fn ascend(&mut self, p: usize) {
        self.pl[p].pos = Pos {
            ring: Ring::Inner,
            idx: 0,
        };
        let w = self.who(p);
        self.say(format!("{w} urcă Scara Neagră în Inelul de Jar."));
    }
    fn assault(&mut self, p: usize, craft: bool, bolt: bool) {
        let w = self.who(p);
        let mut bonus = 0;
        if bolt && self.consume(p, Kind::Sp(Spell::Fulger)) {
            bonus = 3;
        }
        let d = self.rng.d6() as i32;
        let (tot, need) = if craft {
            (self.craft(p) + d + 1 + bonus, 9)
        } else {
            (self.power(p) + d + bonus, 10)
        };
        if tot > need {
            self.pl[p].seal = true;
            self.say(format!(
                "{w}: asaltezi Turnul ({tot} contra {need}) și câștigi Pecetea!"
            ));
        } else {
            self.say(format!(
                "{w}: asaltul Turnului ({tot} contra {need}) eșuează (-1 Viață)."
            ));
            self.lose_life(p, 1);
        }
    }

    fn visible_value(&self) -> i32 {
        let best = (1..NPL).map(|q| self.progress(q)).max().unwrap_or(0);
        self.progress(0) - best
    }
}

impl Game for Coroana {
    type Move = Move;

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Pick | Phase::Start | Phase::Dir | Phase::Enc | Phase::Shop | Phase::Over => {
                Player::One
            }
            _ => Player::Two,
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        let mut v = Vec::new();
        match self.phase {
            Phase::Pick => {
                for i in 0..6 {
                    v.push(Move::Pick(i));
                }
            }
            Phase::Start => {
                let pl = &self.pl[0];
                let sp = space(pl.pos);
                if pl.v < pl.vmax && self.has_item(0, Item::Fiola) {
                    v.push(Move::Drink);
                }
                if pl.v < pl.vmax && self.has_spell(0, Spell::Leac) {
                    v.push(Move::Heal);
                }
                match sp {
                    Sp::Crown => v.push(Move::Hold),
                    Sp::Bridge => {
                        v.push(Move::Roll);
                        if pl.gold >= 2 {
                            v.push(Move::UpPay);
                        }
                        if self.has_item(0, Item::Cheie) {
                            v.push(Move::UpKey);
                        }
                        v.push(Move::UpFight);
                    }
                    Sp::Stairs if pl.seal => {
                        v.push(Move::Roll);
                        v.push(Move::Up);
                    }
                    Sp::Tower if !pl.seal => {
                        v.push(Move::Roll);
                        v.push(Move::AssaultF);
                        v.push(Move::AssaultC);
                    }
                    Sp::Door if pl.seal => {
                        v.push(Move::Roll);
                        v.push(Move::Enter);
                    }
                    _ => v.push(Move::Roll),
                }
            }
            Phase::Dir => {
                let pl = &self.pl[0];
                for cw in [true, false] {
                    v.push(Move::Dir(cw, 0));
                    if pl.ch == Ch::Calauza {
                        v.push(Move::Dir(cw, 1));
                        if self.roll > 1 {
                            v.push(Move::Dir(cw, -1));
                        }
                    }
                }
                if pl.ch == Ch::Vrajitoarea && !pl.reroll {
                    v.push(Move::Reroll);
                }
            }
            Phase::Enc => {
                v.push(Move::Fight);
                if self.has_spell(0, Spell::Fulger) {
                    v.push(Move::FightBolt);
                }
                v.push(Move::Flee);
                if self.has_spell(0, Spell::Ceata) {
                    v.push(Move::Mist);
                }
            }
            Phase::Shop => {
                for it in SHOP {
                    if self.pl[0].gold >= self.price(0, it) {
                        v.push(Move::Buy(it));
                    }
                }
                v.push(Move::Leave);
            }
            Phase::Over => {}
            _ => v.push(Move::Table),
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Pick(i) => {
                let mine = CHARS[*i as usize];
                let mut rest: Vec<Ch> = CHARS.iter().copied().filter(|&c| c != mine).collect();
                shuffle_vec(&mut rest, &mut self.rng);
                let chosen = [mine, rest[0], rest[1]];
                self.pl = chosen
                    .iter()
                    .map(|&c| {
                        let (f, mm, v, gold, st) = c.stats();
                        Pl {
                            ch: c,
                            f,
                            m: mm,
                            v,
                            vmax: v,
                            gold,
                            pos: Pos {
                                ring: Ring::Outer,
                                idx: st,
                            },
                            hand: Vec::new(),
                            trophies: Vec::new(),
                            seal: false,
                            out: false,
                            skip: 0,
                            reroll: false,
                        }
                    })
                    .collect();
                self.say(format!(
                    "Tu: {} ({}). Robotul A: {}, Robotul B: {}.",
                    mine.name(),
                    mine.advantage(),
                    rest[0].name(),
                    rest[1].name()
                ));
                self.cur = 0;
                self.phase = Phase::Start;
            }
            Move::Roll => {
                self.roll = self.move_roll(0);
                self.say(format!("Tu: zarul arată {}.", self.roll));
                self.phase = Phase::Dir;
            }
            Move::Drink => {
                self.consume(0, Kind::It(Item::Fiola));
                self.heal(0, 2);
                self.say("Tu: bei Fiola Vie (+2 Viață).".into());
            }
            Move::Heal => {
                self.consume(0, Kind::Sp(Spell::Leac));
                self.heal(0, 2);
                self.say("Tu: rostești Descântecul de Leac (+2 Viață).".into());
            }
            Move::UpPay => {
                self.pl[0].gold -= 2;
                self.pl[0].pos = Pos {
                    ring: Ring::Middle,
                    idx: 0,
                };
                self.say("Tu: plătești vama și treci Podul Gri.".into());
                self.phase = Phase::Pass;
            }
            Move::UpKey => {
                self.consume(0, Kind::It(Item::Cheie));
                self.pl[0].pos = Pos {
                    ring: Ring::Middle,
                    idx: 0,
                };
                self.say("Tu: descui Podul Gri cu Cheia de Aramă.".into());
                self.phase = Phase::Pass;
            }
            Move::UpFight => {
                self.enc = Some(Enc {
                    name: "Paznicul Podului",
                    s: 7,
                    dmg: 1,
                    src: Src::Guard,
                });
                self.act = Act::Fight(false);
                self.phase = Phase::EncRoll;
            }
            Move::Up => {
                self.ascend(0);
                self.phase = Phase::Pass;
            }
            Move::AssaultF | Move::AssaultC => {
                self.enc = Some(Enc {
                    name: "Străjerul Turnului",
                    s: 10,
                    dmg: 1,
                    src: Src::Tower(*m == Move::AssaultC),
                });
                self.phase = Phase::EncRoll;
                self.act = Act::Fight(false);
            }
            Move::Enter => {
                self.enter_crown(0);
                self.phase = Phase::Pass;
            }
            Move::Hold => self.phase = Phase::Pass,
            Move::Dir(cw, d) => {
                let steps = self.roll + *d as i32;
                self.step(0, *cw, steps);
                self.phase = Phase::Land;
            }
            Move::Reroll => {
                self.pl[0].reroll = true;
                self.roll = self.move_roll(0);
                self.say(format!("Tu: re-arunci zarul: {}.", self.roll));
            }
            Move::Fight => {
                self.act = Act::Fight(false);
                self.phase = Phase::EncRoll;
            }
            Move::FightBolt => {
                self.act = Act::Fight(true);
                self.phase = Phase::EncRoll;
            }
            Move::Flee => {
                self.act = Act::Flee;
                self.phase = Phase::EncRoll;
            }
            Move::Mist => {
                self.act = Act::Mist;
                self.phase = Phase::EncRoll;
            }
            Move::Buy(it) => {
                self.buy(0, *it);
            }
            Move::Leave => self.phase = Phase::Pass,
            Move::Table => match self.phase {
                Phase::Land => match self.land(0) {
                    Some(ph) => self.phase = ph,
                    None => self.phase = Phase::Pass,
                },
                Phase::EncRoll => {
                    let act = self.act;
                    let enc = self.enc;
                    let special_tower = matches!(
                        enc,
                        Some(Enc {
                            src: Src::Tower(_),
                            ..
                        })
                    );
                    if special_tower {
                        if let Some(Enc {
                            src: Src::Tower(craft),
                            ..
                        }) = enc
                        {
                            self.enc = None;
                            self.assault(0, craft, false);
                        }
                    } else {
                        self.resolve_enc(0, act);
                    }
                    self.phase = Phase::Pass;
                }
                Phase::Pass => self.end_turn(),
                Phase::Robot => {
                    let p = self.cur;
                    self.robot_turn(p);
                    self.end_turn();
                }
                _ => {}
            },
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        let w = self.winner?;
        Some(Outcome::Win(if w == 0 { Player::One } else { Player::Two }))
    }

    fn evaluate(&self, p: Player) -> i32 {
        if self.pl.is_empty() {
            return 0;
        }
        let v = self.visible_value();
        if p == Player::One {
            v
        } else {
            -v
        }
    }

    fn render(&self) -> String {
        let mut s = format!("DRUMUL SPRE COROANĂ — runda {}/{}\n", self.round, ROUND_CAP);
        if self.pl.is_empty() {
            s.push_str("Personaje:\n");
            for c in CHARS {
                let (f, m, v, g, _) = c.stats();
                s.push_str(&format!(
                    "  {} — Forță {f}, Meșteșug {m}, Viață {v}, Aur {g}; {}\n",
                    c.key(),
                    c.advantage()
                ));
            }
            for l in &self.log {
                s.push_str(l);
                s.push('\n');
            }
            return s;
        }
        let marks = |ring: Ring, idx: usize| -> String {
            let mut m = String::new();
            for (q, pl) in self.pl.iter().enumerate() {
                if !pl.out && pl.pos.ring == ring && pl.pos.idx as usize == idx {
                    m.push(['T', 'A', 'B'][q]);
                }
            }
            if m.is_empty() {
                m
            } else {
                format!("<{m}>")
            }
        };
        let ring_line = |name: &str, ring: Ring| -> String {
            let n = ring.len();
            let cells: Vec<String> = (0..n)
                .map(|i| {
                    format!(
                        "{}{}{}",
                        i,
                        abbr(space(Pos { ring, idx: i as u8 })),
                        marks(ring, i)
                    )
                })
                .collect();
            format!("{name}: {}\n", cells.join(" "))
        };
        s.push_str(&ring_line("Câmpie", Ring::Outer));
        s.push_str(&ring_line("Țara Cenușie", Ring::Middle));
        s.push_str(&ring_line("Inel de Jar", Ring::Inner));
        let in_crown: Vec<&str> = self
            .pl
            .iter()
            .enumerate()
            .filter(|(_, p)| !p.out && p.pos.ring == Ring::Crown)
            .map(|(q, _)| ["T", "A", "B"][q])
            .collect();
        s.push_str(&format!("Camera Coroanei: <{}>\n", in_crown.join("")));
        for (q, pl) in self.pl.iter().enumerate() {
            let hand: Vec<String> = pl
                .hand
                .iter()
                .map(|c| match c.kind {
                    Kind::It(i) => i.key().to_string(),
                    Kind::Sp(sp) => format!("{:?}", sp).to_lowercase(),
                    _ => "?".into(),
                })
                .collect();
            s.push_str(&format!(
                "{} [{}]: {} — Viață {}/{}, Forță {} (de bază {}), Meșteșug {}, Aur {}, Pecete {}, trofee {}, obiecte: {}{}\n",
                ["Tu", "Robotul A", "Robotul B"][q],
                ["T", "A", "B"][q],
                pl.ch.name(),
                pl.v,
                pl.vmax,
                self.power(q),
                pl.f,
                self.craft(q),
                pl.gold,
                if pl.seal { "da" } else { "nu" },
                pl.trophies.len(),
                if hand.is_empty() { "—".into() } else { hand.join(", ") },
                if pl.out { " [SCOS DIN JOC]" } else { "" }
            ));
        }
        match self.phase {
            Phase::Dir => s.push_str(&format!("Zar: {} — alegi sensul.\n", self.roll)),
            Phase::Enc => {
                if let Some(e) = self.enc {
                    s.push_str(&format!(
                        "Întâlnire: {} (forța {}, daune {}). Lupți, fugi sau folosești un descântec.\n",
                        e.name, e.s, e.dmg
                    ));
                }
            }
            Phase::Shop => {
                let items: Vec<String> = SHOP
                    .iter()
                    .map(|&i| format!("{} {}A", i.key(), self.price(0, i)))
                    .collect();
                s.push_str(&format!("Târg: {}.\n", items.join(", ")));
            }
            _ => {}
        }
        for l in &self.log {
            s.push_str(l);
            s.push('\n');
        }
        if let Some(w) = self.winner {
            s.push_str(&format!(
                "{} câștigă Coroana.\n",
                ["Tu", "Robotul A", "Robotul B"][w]
            ));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Pick(i) => format!("alege {}", CHARS[*i as usize].key()),
            Move::Roll => "zar".into(),
            Move::Drink => "bea".into(),
            Move::Heal => "leac".into(),
            Move::UpPay => "urc plata".into(),
            Move::UpKey => "urc cheie".into(),
            Move::UpFight => "urc lupta".into(),
            Move::Up => "urc".into(),
            Move::AssaultF => "asalt forta".into(),
            Move::AssaultC => "asalt iscusinta".into(),
            Move::Enter => "intru".into(),
            Move::Hold => "mantin".into(),
            Move::Dir(cw, d) => {
                let b = if *cw { "dreapta" } else { "stanga" };
                match d {
                    0 => b.to_string(),
                    1 => format!("{b}+1"),
                    _ => format!("{b}-1"),
                }
            }
            Move::Reroll => "rearunc".into(),
            Move::Fight => "lupt".into(),
            Move::FightBolt => "lupt fulger".into(),
            Move::Flee => "fug".into(),
            Move::Mist => "ceata".into(),
            Move::Buy(i) => format!("cumpar {}", i.key()),
            Move::Leave => "plec".into(),
            Move::Table => "masa".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn started(seed: u64) -> Coroana {
        let mut g = Coroana::with_seed(seed);
        g.apply(&Move::Pick((seed % 6) as u8));
        g
    }

    fn run(seed: u64) -> Coroana {
        let mut g = started(seed);
        let mut rng = Rng::new(seed ^ 0x77);
        let mut n = 0;
        while g.outcome().is_none() {
            let mv = g.legal_moves();
            assert!(!mv.is_empty(), "{}", g.render());
            g.apply(&mv[rng.below(mv.len())]);
            assert_eq!(g.total_cards(), g.expected_cards(), "{}", g.render());
            n += 1;
            assert!(n < 30000, "{}", g.render());
        }
        g
    }

    /// Joacă tura omului cu aceeași euristică ca roboții (doar pentru simulări).
    fn auto_human(g: &mut Coroana) {
        g.robot_turn(0);
        g.end_turn();
    }

    fn all_auto(seed: u64) -> Coroana {
        let mut g = started(seed);
        let mut n = 0;
        while g.outcome().is_none() {
            match g.phase {
                Phase::Start => auto_human(&mut g),
                Phase::Robot => g.apply(&Move::Table),
                _ => panic!("{:?}", g.phase),
            }
            assert_eq!(g.total_cards(), g.expected_cards());
            n += 1;
            assert!(n < 2000);
        }
        g
    }

    #[test]
    fn decks_have_expected_sizes() {
        let g = Coroana::with_seed(1);
        assert_eq!(g.decks[0].len(), 24);
        assert_eq!(g.decks[1].len(), 18);
        assert_eq!(g.decks[2].len(), 12);
        assert_eq!(g.total_cards(), 54);
    }

    #[test]
    fn six_characters_with_distinct_picks() {
        for seed in 0..12u64 {
            let g = started(seed);
            let mut cs: Vec<Ch> = g.pl.iter().map(|p| p.ch).collect();
            cs.sort_by_key(|c| c.key());
            cs.dedup();
            assert_eq!(cs.len(), 3);
            assert_eq!(g.pl[0].ch, CHARS[(seed % 6) as usize]);
        }
    }

    #[test]
    fn random_games_end_and_conserve_cards() {
        let mut human = 0;
        for seed in 1..60u64 {
            let g = run(seed);
            assert!(g.outcome().is_some());
            if g.winner == Some(0) {
                human += 1;
            }
        }
        let _ = human;
    }

    #[test]
    fn robots_mostly_finish_by_crown() {
        let (mut crown, mut cap) = (0, 0);
        let mut wins = [0; 3];
        let mut rounds = 0u32;
        for seed in 1..120u64 {
            let g = all_auto(seed);
            if g.was_capped() {
                cap += 1;
            } else {
                crown += 1;
            }
            wins[g.winner.unwrap()] += 1;
            rounds += g.round;
        }
        eprintln!(
            "crown {crown} cap {cap} wins {wins:?} avg rounds {}",
            rounds / 120
        );
        assert!(crown > cap, "{crown} {cap}");
        assert!(wins.iter().all(|&w| w > 5), "{wins:?}");
    }

    #[test]
    fn movement_wraps_around_rings() {
        let mut g = started(2);
        g.pl[0].pos = Pos {
            ring: Ring::Outer,
            idx: 15,
        };
        g.step(0, true, 3);
        assert_eq!(g.pl[0].pos.idx, 2);
        g.step(0, false, 4);
        assert_eq!(g.pl[0].pos.idx, 14);
        g.pl[0].pos = Pos {
            ring: Ring::Inner,
            idx: 1,
        };
        g.step(0, false, 3);
        assert_eq!(g.pl[0].pos.idx, 6);
    }

    #[test]
    fn fight_rule_needs_to_exceed() {
        for seed in 1..200u64 {
            let mut g = started(seed);
            g.pl[0].f = 3;
            g.pl[0].hand.clear();
            g.pl[0].v = 5;
            g.pl[0].vmax = 5;
            let c = Card {
                kind: Kind::Mon(0),
                ring: 0,
            };
            // scoatem cartea din pachet pentru a păstra conservarea
            let pos = g.decks[0].iter().position(|x| *x == c).unwrap();
            g.decks[0].remove(pos);
            g.enc = Some(Enc {
                name: "Test",
                s: 9,
                dmg: 1,
                src: Src::Card(c),
            });
            let before_gold = g.pl[0].gold;
            g.resolve_enc(0, Act::Fight(false));
            // forța 3 + d6 (max 9) nu depășește 9: nu poate câștiga
            assert_eq!(g.pl[0].trophies.len(), 0);
            assert_eq!(g.pl[0].gold, before_gold);
            assert_eq!(g.total_cards(), g.expected_cards());
        }
    }

    #[test]
    fn winning_fight_gives_trophy_and_levels() {
        let mut g = started(3);
        g.pl[0].f = 10;
        g.pl[0].ch = Ch::Negustorul;
        let c = Card {
            kind: Kind::Mon(7),
            ring: 0,
        }; // forța 7 => 1 trofeu de 7 puncte => +1 Forță
        let pos = g.decks[0].iter().position(|x| *x == c).unwrap();
        g.decks[0].remove(pos);
        g.enc = Some(Enc {
            name: "Căpcăun",
            s: 7,
            dmg: 2,
            src: Src::Card(c),
        });
        g.resolve_enc(0, Act::Fight(false));
        assert_eq!(g.pl[0].f, 11, "trofeul de 7 puncte dă +1 Forță");
        assert!(g.pl[0].trophies.is_empty());
        assert_eq!(g.discard[0].len(), 1);
        assert_eq!(g.total_cards(), g.expected_cards());
    }

    #[test]
    fn falling_resets_the_player() {
        let mut g = started(4);
        g.pl[0].v = 1;
        g.pl[0].gold = 5;
        g.pl[0].seal = true;
        g.pl[0].pos = Pos {
            ring: Ring::Inner,
            idx: 2,
        };
        g.give(
            0,
            Card {
                kind: Kind::It(Item::Sabie),
                ring: 0,
            },
        );
        g.minted += 1;
        g.lose_life(0, 1);
        assert_eq!(g.pl[0].gold, 0);
        assert!(!g.pl[0].seal);
        assert_eq!(g.pl[0].pos.ring, Ring::Outer);
        assert_eq!(g.pl[0].pos.idx, 3);
        assert!(g.pl[0].v >= 2);
        assert_eq!(g.pl[0].skip, 1);
        assert!(g.pl[0].hand.is_empty());
        assert!(!g.pl[0].out, "a cădea nu înseamnă a fi scos din joc");
    }

    #[test]
    fn crown_command_hits_outsiders_and_eliminates() {
        let mut g = started(5);
        g.pl[1].pos = Pos {
            ring: Ring::Crown,
            idx: 0,
        };
        g.pl[0].v = 2;
        g.pl[2].v = 1;
        g.crown_turn(1);
        assert_eq!(g.pl[0].v, 1);
        assert!(g.pl[2].out);
        assert_eq!(g.pl[2].v, 0);
        g.crown_turn(1);
        assert!(g.pl[0].out);
        assert!(g.check_over());
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
    }

    #[test]
    fn human_wins_when_both_robots_are_out() {
        let mut g = started(6);
        g.pl[0].pos = Pos {
            ring: Ring::Crown,
            idx: 0,
        };
        g.pl[1].v = 1;
        g.pl[2].v = 1;
        g.crown_turn(0);
        assert!(g.check_over());
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn duel_in_the_crown_room_spares_nobody_outside_or_inside_unfairly() {
        let mut g = started(7);
        for q in 0..2 {
            g.pl[q].pos = Pos {
                ring: Ring::Crown,
                idx: 0,
            };
            g.pl[q].v = 5;
        }
        g.pl[2].v = 5;
        g.crown_turn(0);
        assert_eq!(g.pl[2].v, 4, "cel din afară pierde o Viață");
        let inside = g.pl[0].v + g.pl[1].v;
        assert!(inside == 10 || inside == 9);
    }

    #[test]
    fn gates_require_the_right_things() {
        let mut g = started(8);
        g.pl[0].seal = false;
        g.pl[0].pos = Pos {
            ring: Ring::Middle,
            idx: 11,
        };
        let m = g.legal_moves();
        assert!(!m.contains(&Move::Up), "fără Pecete nu urci");
        g.pl[0].seal = true;
        assert!(g.legal_moves().contains(&Move::Up));
        g.pl[0].pos = Pos {
            ring: Ring::Inner,
            idx: 7,
        };
        assert!(g.legal_moves().contains(&Move::Enter));
        g.pl[0].seal = false;
        assert!(!g.legal_moves().contains(&Move::Enter));
        g.pl[0].pos = Pos {
            ring: Ring::Outer,
            idx: 0,
        };
        g.pl[0].gold = 1;
        let m = g.legal_moves();
        assert!(!m.contains(&Move::UpPay));
        assert!(!m.contains(&Move::UpKey));
        assert!(m.contains(&Move::UpFight));
        g.pl[0].gold = 2;
        assert!(g.legal_moves().contains(&Move::UpPay));
        g.apply(&Move::UpPay);
        assert_eq!(g.pl[0].pos.ring, Ring::Middle);
        assert_eq!(g.pl[0].gold, 0);
    }

    #[test]
    fn market_mints_cards_and_negustor_discount() {
        let mut g = started(9);
        g.pl[0].ch = Ch::Negustorul;
        g.pl[0].gold = 10;
        assert_eq!(g.price(0, Item::Sabie), 2);
        g.pl[0].ch = Ch::Haiducul;
        assert_eq!(g.price(0, Item::Sabie), 3);
        g.phase = Phase::Shop;
        g.apply(&Move::Buy(Item::Sabie));
        assert_eq!(g.pl[0].gold, 7);
        assert_eq!(g.minted, 1);
        assert_eq!(g.total_cards(), g.expected_cards());
        assert_eq!(g.power(0), g.pl[0].f + 1);
        assert_eq!(g.phase, Phase::Shop);
        g.apply(&Move::Leave);
        assert_eq!(g.phase, Phase::Pass);
    }

    #[test]
    fn hand_is_capped() {
        let mut g = started(10);
        g.pl[0].gold = 100;
        for _ in 0..8 {
            g.buy(0, Item::Fiola);
        }
        assert!(g.pl[0].hand.len() <= HAND_CAP);
        assert_eq!(g.total_cards(), g.expected_cards());
    }

    #[test]
    fn round_cap_ends_game() {
        let mut g = started(11);
        g.round = ROUND_CAP;
        g.cur = 2;
        g.end_turn();
        assert!(g.was_capped());
        assert!(g.outcome().is_some());
    }

    #[test]
    fn dice_are_not_peeked_by_decisions() {
        // decizia „lupt” nu aruncă zarul: aruncarea e într-o mutare forțată
        let mut g = started(12);
        g.enc = Some(Enc {
            name: "T",
            s: 1,
            dmg: 1,
            src: Src::Cave,
        });
        g.phase = Phase::Enc;
        let r = g.rng.clone().next_u64();
        g.apply(&Move::Fight);
        assert_eq!(g.rng.clone().next_u64(), r);
        assert_eq!(g.phase, Phase::EncRoll);
        assert_eq!(g.to_move(), Player::Two);
    }

    #[test]
    fn ai_is_fast() {
        use polytabletop_core::GameObj;
        let mut g: Box<dyn GameObj> = Box::new(started(13));
        let mut rng = Rng::new(3);
        let t = std::time::Instant::now();
        let mut n = 0;
        while g.outcome().is_none() && n < 4000 {
            let m = g.ai_move(rng.next_u64()).unwrap();
            g.play(&m).unwrap();
            n += 1;
        }
        assert!(g.outcome().is_some());
        assert!(t.elapsed().as_secs_f32() / (n as f32) < 1.0);
    }

    #[test]
    fn move_texts_unique_and_render_ok() {
        for seed in 1..20u64 {
            let mut g = Coroana::with_seed(seed);
            assert!(g.render().contains("haiducul"));
            g.apply(&Move::Pick(1));
            let mut rng = Rng::new(seed);
            for _ in 0..300 {
                if g.outcome().is_some() {
                    break;
                }
                let mv = g.legal_moves();
                let mut t: Vec<String> = mv.iter().map(|m| g.format_move(m)).collect();
                let n = t.len();
                t.sort();
                t.dedup();
                assert_eq!(t.len(), n);
                let _ = g.render();
                g.apply(&mv[rng.below(mv.len())]);
            }
        }
    }
}
