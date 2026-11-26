//! Motor de cărți colecționabile „cerneală din cărți + misiuni + provocări” (id
//! `tcg_cerneala`), 2 jucători, cu un set ORIGINAL de cărți („Seria Povestirilor
//! din Cerneală”). Reguli scrise de noi, din mecanici generice.
//!
//! * Pachet pregătit de 30 de cărți (J1 „Pădurea Cântătoare”, J2 „Muntele de
//!   Fier”), 7 cărți în mână. Câștigă primul care ajunge la 15 puncte de
//!   **faimă** (sau adversarul rămâne fără pachet la pescuire).
//! * Tura: `pesca` (mutare forțată la începutul fiecărei ture; în prima tură J1 o
//!   scrie tot, dar nu trage nicio carte), apoi, în orice
//!   ordine: `cerneala N` (o dată pe tură, o carte „cu cerneală” din mână se
//!   pune întoarsă în călimară: fiecare carte de-acolo dă 1 cerneală pe tură),
//!   `joaca N [țintă]` (plătești costul în cerneală; personajele intră „uscându-se”),
//!   `misiune P` (personajul se înclină și aduci faimă egală cu valoarea lui de
//!   faimă), `provocare P e Q` (personajul tău pregătit atacă un personaj
//!   ÎNCLINAT al adversarului), `fin`. La începutul turei tale totul se
//!   pregătește și cerneala se reînnoiește.
//! * Personajele: cost, forță, voință, faimă; au daune care rămân până la
//!   eliminare (daune ≥ voință). Provocare: ambele lovesc simultan cu forța lor.
//!   Cuvinte-cheie: Elan (poate face misiune/provocare imediat), Gardian (dacă
//!   adversarul are un Gardian înclinat, provocările trebuie să-l țintească),
//!   Zburător (poate fi provocat doar de zburători), Provocator (+2 forță când
//!   provoacă), Rezistență (-1 la daunele primite).
//! * Acțiuni: pescuire, daune, daune la toți, întoarcere în mână, vindecare,
//!   pregătire, eliminarea unui personaj înclinat, faimă, bonus de forță.
//!   Ținte: `e2` personajul advers 2, `m1` al meu.
//!
//! Informația ascunsă: mâinile, pachetele și conținutul călimărilor; `render()`
//! arată doar mâna J1. AI-ul (AlphaBeta(1)) are `evaluate` care citește doar
//! câmpul public, mâna proprie și NUMĂRUL cărților adverse.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const RUSH: u8 = 1;
pub const GUARD: u8 = 2;
pub const EVASIVE: u8 = 4;
pub const CHALLENGER: u8 = 8;
pub const RESIST: u8 = 16;

const WIN_FAME: i16 = 15;
const HAND0: usize = 7;
const MAXBOARD: usize = 8;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Fx {
    Draw(u8),
    Dmg(i16),
    DmgAll(i16),
    Bounce,
    Heal(i16),
    Ready,
    Banish,
    Fame(i16),
    Boost(i16),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum K {
    Char {
        cost: u8,
        str: i16,
        will: i16,
        lore: i16,
        kw: u8,
    },
    Action {
        cost: u8,
        fx: Fx,
    },
}

pub struct Card {
    pub name: &'static str,
    pub k: K,
    /// Poate fi pusă în călimară.
    pub ink: bool,
}

const fn ch(
    name: &'static str,
    cost: u8,
    str: i16,
    will: i16,
    lore: i16,
    kw: u8,
    ink: bool,
) -> Card {
    Card {
        name,
        k: K::Char {
            cost,
            str,
            will,
            lore,
            kw,
        },
        ink,
    }
}
const fn ac(name: &'static str, cost: u8, fx: Fx, ink: bool) -> Card {
    Card {
        name,
        k: K::Action { cost, fx },
        ink,
    }
}

/// Setul complet: 42 de cărți originale.
pub const CARDS: &[Card] = &[
    ch("Licurici Cântător", 1, 1, 2, 1, 0, true),
    ch("Șoricel Iute", 1, 1, 1, 1, RUSH, true),
    ch("Poetul Rătăcitor", 1, 0, 1, 2, 0, true),
    ch("Pitic de Lumină", 2, 2, 2, 1, 0, true),
    ch("Ursuleț de Cetate", 2, 1, 4, 1, GUARD, true),
    ch("Libelula Argintie", 2, 1, 2, 1, EVASIVE, true),
    ch("Ghidul Pădurii", 2, 2, 3, 1, 0, true),
    ch("Cavaler Mic", 3, 3, 3, 1, 0, true),
    ch("Paznic de Far", 3, 2, 5, 1, GUARD, true),
    ch("Pasăre Cântătoare", 3, 2, 3, 2, EVASIVE, true),
    ch("Pirat de Câmpie", 3, 3, 2, 1, RUSH, true),
    ch("Tâlharul Nopții", 3, 2, 2, 2, RUSH, true),
    ch("Povestitoarea", 3, 1, 3, 2, 0, true),
    ch("Bufnița Veche", 3, 2, 4, 2, 0, true),
    ch("Căpitan de Gardă", 4, 3, 5, 2, 0, true),
    ch("Grifon", 4, 3, 4, 2, EVASIVE, true),
    ch("Vrăjitoarea Lacului", 4, 2, 4, 3, 0, true),
    ch("Lupul Cenușiu", 4, 4, 3, 1, CHALLENGER, true),
    ch("Armăsar de Foc", 4, 4, 4, 1, RUSH, true),
    ch("Cavaler al Lacului", 4, 4, 4, 2, 0, false),
    ch("Alchimistul", 4, 2, 3, 3, 0, false),
    ch("Zâna Pădurii", 5, 3, 5, 3, 0, false),
    ch("Ursul Mare", 5, 5, 5, 2, RESIST, false),
    ch("Păzitorul Porții", 5, 3, 7, 2, GUARD | RESIST, false),
    ch("Bătrâna Povestitoare", 5, 2, 5, 4, 0, false),
    ch("Dragon de Cenușă", 6, 5, 6, 3, EVASIVE, false),
    ch("Regina Ceții", 6, 4, 6, 4, 0, false),
    ch("Uriașul Munților", 7, 7, 8, 2, RESIST, false),
    ch("Marele Înțelept", 7, 4, 7, 4, 0, false),
    ch("Cântărețul Furtunii", 2, 1, 2, 2, 0, false),
    ac("Ploaie de Povești", 2, Fx::Draw(2), true),
    ac("Revelația", 4, Fx::Draw(3), false),
    ac("Rafală", 1, Fx::Dmg(1), true),
    ac("Săgeată de Gheață", 2, Fx::Dmg(3), true),
    ac("Lovitura Ascunsă", 4, Fx::Dmg(5), false),
    ac("Furtună de Cenușă", 4, Fx::DmgAll(2), false),
    ac("Vânt de Schimbare", 3, Fx::Bounce, true),
    ac("Leac de Mierlă", 1, Fx::Heal(3), true),
    ac("Chemarea Vitejilor", 2, Fx::Ready, true),
    ac("Sentința Lunii", 3, Fx::Banish, false),
    ac("Izbucnire de Faimă", 4, Fx::Fame(2), false),
    ac("Strigăt de Luptă", 2, Fx::Boost(1), true),
];

pub fn card_id(name: &str) -> u8 {
    CARDS
        .iter()
        .position(|c| c.name == name)
        .unwrap_or_else(|| panic!("carte necunoscută: {name}")) as u8
}

const DECK_PADUREA: &[(&str, u8)] = &[
    ("Licurici Cântător", 2),
    ("Poetul Rătăcitor", 2),
    ("Pitic de Lumină", 2),
    ("Ursuleț de Cetate", 2),
    ("Libelula Argintie", 2),
    ("Ghidul Pădurii", 2),
    ("Pasăre Cântătoare", 2),
    ("Paznic de Far", 1),
    ("Povestitoarea", 2),
    ("Vrăjitoarea Lacului", 2),
    ("Grifon", 1),
    ("Zâna Pădurii", 2),
    ("Dragon de Cenușă", 1),
    ("Regina Ceții", 1),
    ("Ploaie de Povești", 2),
    ("Leac de Mierlă", 1),
    ("Vânt de Schimbare", 1),
    ("Săgeată de Gheață", 1),
    ("Izbucnire de Faimă", 1),
];

const DECK_MUNTELE: &[(&str, u8)] = &[
    ("Șoricel Iute", 2),
    ("Pitic de Lumină", 2),
    ("Cavaler Mic", 2),
    ("Pirat de Câmpie", 2),
    ("Tâlharul Nopții", 2),
    ("Lupul Cenușiu", 2),
    ("Armăsar de Foc", 2),
    ("Căpitan de Gardă", 2),
    ("Cavaler al Lacului", 2),
    ("Ursul Mare", 2),
    ("Uriașul Munților", 1),
    ("Bufnița Veche", 1),
    ("Săgeată de Gheață", 1),
    ("Rafală", 2),
    ("Strigăt de Luptă", 1),
    ("Sentința Lunii", 1),
    ("Furtună de Cenușă", 1),
    ("Lovitura Ascunsă", 1),
    ("Chemarea Vitejilor", 1),
];

pub fn deck_list(p: Player) -> Vec<u8> {
    let src = if p == Player::One {
        DECK_PADUREA
    } else {
        DECK_MUNTELE
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
        "Pădurea Cântătoare"
    } else {
        "Muntele de Fier"
    }
}

fn kw_text(kw: u8) -> String {
    let mut v = Vec::new();
    for (f, n) in [
        (RUSH, "Elan"),
        (GUARD, "Gardian"),
        (EVASIVE, "Zburător"),
        (CHALLENGER, "Provocator"),
        (RESIST, "Rezistență"),
    ] {
        if kw & f != 0 {
            v.push(n);
        }
    }
    v.join(", ")
}

fn fx_text(f: Fx) -> String {
    match f {
        Fx::Draw(n) => format!("pescuiești {n}"),
        Fx::Dmg(n) => format!("{n} daune unui personaj advers"),
        Fx::DmgAll(n) => format!("{n} daune fiecărui personaj advers"),
        Fx::Bounce => "un personaj advers se întoarce în mâna lui".into(),
        Fx::Heal(n) => format!("vindeci {n} daune unui personaj al tău"),
        Fx::Ready => "pregătești un personaj al tău".into(),
        Fx::Banish => "elimini un personaj advers înclinat".into(),
        Fx::Fame(n) => format!("câștigi {n} faimă"),
        Fx::Boost(n) => format!("personajele tale +{n} forță până la sfârșitul turei"),
    }
}

pub fn card_line(id: u8) -> String {
    let c = &CARDS[id as usize];
    let ink = if c.ink { "; cerneală" } else { "" };
    match c.k {
        K::Char {
            cost,
            str,
            will,
            lore,
            kw,
        } => {
            let k = kw_text(kw);
            format!(
                "{} (personaj cost {cost}, forță {str}, voință {will}, faimă {lore}{}{ink})",
                c.name,
                if k.is_empty() {
                    String::new()
                } else {
                    format!("; {k}")
                }
            )
        }
        K::Action { cost, fx } => format!("{} (acțiune cost {cost}: {}{ink})", c.name, fx_text(fx)),
    }
}

#[derive(Clone, Debug)]
pub struct Chr {
    pub card: u8,
    pub uid: u16,
    pub dmg: i16,
    pub exerted: bool,
    pub fresh: bool,
    pub bstr: i16,
}

impl Chr {
    fn stats(&self) -> (u8, i16, i16, i16, u8) {
        match CARDS[self.card as usize].k {
            K::Char {
                cost,
                str,
                will,
                lore,
                kw,
            } => (cost, str, will, lore, kw),
            _ => panic!("nu e personaj"),
        }
    }
    pub fn str(&self) -> i16 {
        self.stats().1 + self.bstr
    }
    pub fn will(&self) -> i16 {
        self.stats().2
    }
    pub fn lore(&self) -> i16 {
        self.stats().3
    }
    pub fn kw(&self) -> u8 {
        self.stats().4
    }
}

#[derive(Clone, Debug, Default)]
pub struct Side {
    pub fame: i16,
    pub deck: Vec<u8>,
    pub hand: Vec<u8>,
    pub well: Vec<u8>,
    pub spent: u8,
    pub inked: bool,
    pub board: Vec<Chr>,
    pub grave: Vec<u8>,
}

impl Side {
    pub fn total_cards(&self) -> usize {
        self.deck.len() + self.hand.len() + self.well.len() + self.board.len() + self.grave.len()
    }
    fn ink_left(&self) -> u8 {
        self.well.len() as u8 - self.spent
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tgt {
    None,
    Foe(u8),
    Mine(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Draw,
    Ink(u8),
    Play(u8, Tgt),
    Quest(u8),
    Challenge(u8, u8),
    End,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Phase {
    Draw,
    Main,
}

#[derive(Clone)]
pub struct TcgCerneala {
    pub sides: [Side; 2],
    pub turn: Player,
    pub phase: Phase,
    pub turn_no: u32,
    next_uid: u16,
    over: Option<Outcome>,
    /// `Some(p)` = render shows `p`'s hand; `None` = hot-seat (the mover's).
    viewer: Option<Player>,
}

impl Default for TcgCerneala {
    fn default() -> Self {
        Self::new()
    }
}

fn shuffle(v: &mut [u8], rng: &mut Rng) {
    for i in (1..v.len()).rev() {
        v.swap(i, rng.below(i + 1));
    }
}

impl TcgCerneala {
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
            for _ in 0..HAND0 {
                s.hand.push(d.pop().unwrap());
            }
            s.deck = d;
        }
        TcgCerneala {
            sides,
            turn: Player::One,
            phase: Phase::Draw,
            turn_no: 1,
            next_uid: 1,
            over: None,
            viewer: Some(Player::One),
        }
    }

    pub fn side(&self, p: Player) -> &Side {
        &self.sides[p.idx()]
    }

    fn check_fame(&mut self) {
        let a = self.sides[0].fame >= WIN_FAME;
        let b = self.sides[1].fame >= WIN_FAME;
        self.over = match (a, b) {
            (true, true) => Some(Outcome::Draw),
            (true, false) => Some(Outcome::Win(Player::One)),
            (false, true) => Some(Outcome::Win(Player::Two)),
            _ => None,
        };
    }

    fn start_turn(&mut self, p: Player) {
        self.turn = p;
        self.turn_no += 1;
        self.phase = Phase::Draw;
        let s = &mut self.sides[p.idx()];
        s.spent = 0;
        s.inked = false;
        for c in &mut s.board {
            c.exerted = false;
            c.fresh = false;
        }
    }

    fn end_turn(&mut self) {
        for s in &mut self.sides {
            for c in &mut s.board {
                c.bstr = 0;
            }
        }
        self.start_turn(self.turn.other());
    }

    fn reap(&mut self) {
        for s in &mut self.sides {
            let mut i = 0;
            while i < s.board.len() {
                if s.board[i].dmg >= s.board[i].will() {
                    let c = s.board.remove(i);
                    s.grave.push(c.card);
                } else {
                    i += 1;
                }
            }
        }
    }

    fn action_targets(&self, p: Player, fx: Fx) -> Vec<Tgt> {
        let me = &self.sides[p.idx()];
        let foe = &self.sides[p.other().idx()];
        match fx {
            Fx::Dmg(_) | Fx::Bounce => (0..foe.board.len() as u8).map(Tgt::Foe).collect(),
            Fx::Banish => foe
                .board
                .iter()
                .enumerate()
                .filter(|(_, c)| c.exerted)
                .map(|(i, _)| Tgt::Foe(i as u8))
                .collect(),
            Fx::Heal(_) => me
                .board
                .iter()
                .enumerate()
                .filter(|(_, c)| c.dmg > 0)
                .map(|(i, _)| Tgt::Mine(i as u8))
                .collect(),
            Fx::Ready => me
                .board
                .iter()
                .enumerate()
                .filter(|(_, c)| c.exerted)
                .map(|(i, _)| Tgt::Mine(i as u8))
                .collect(),
            Fx::DmgAll(_) => {
                if foe.board.is_empty() {
                    Vec::new()
                } else {
                    vec![Tgt::None]
                }
            }
            _ => vec![Tgt::None],
        }
    }

    /// Țintele legale ale unei provocări a personajului `ai` al lui `p`.
    fn challenge_targets(&self, p: Player, ai: usize) -> Vec<u8> {
        let me = &self.sides[p.idx()];
        let foe = &self.sides[p.other().idx()];
        let a = &me.board[ai];
        let cands: Vec<usize> = foe
            .board
            .iter()
            .enumerate()
            .filter(|(_, c)| c.exerted && (c.kw() & EVASIVE == 0 || a.kw() & EVASIVE != 0))
            .map(|(i, _)| i)
            .collect();
        let guards: Vec<usize> = cands
            .iter()
            .copied()
            .filter(|i| foe.board[*i].kw() & GUARD != 0)
            .collect();
        let v = if guards.is_empty() { cands } else { guards };
        v.into_iter().map(|i| i as u8).collect()
    }

    fn draw_card(&mut self, p: Player) {
        let s = &mut self.sides[p.idx()];
        if let Some(c) = s.deck.pop() {
            s.hand.push(c);
        }
    }

    fn do_action(&mut self, p: Player, fx: Fx, t: Tgt) {
        let q = p.other();
        match (fx, t) {
            (Fx::Draw(n), _) => {
                for _ in 0..n {
                    self.draw_card(p);
                }
            }
            (Fx::Dmg(n), Tgt::Foe(i)) => self.sides[q.idx()].board[i as usize].dmg += n,
            (Fx::DmgAll(n), _) => {
                for c in &mut self.sides[q.idx()].board {
                    c.dmg += n;
                }
            }
            (Fx::Bounce, Tgt::Foe(i)) => {
                let c = self.sides[q.idx()].board.remove(i as usize);
                self.sides[q.idx()].hand.push(c.card);
            }
            (Fx::Heal(n), Tgt::Mine(i)) => {
                let c = &mut self.sides[p.idx()].board[i as usize];
                c.dmg = (c.dmg - n).max(0);
            }
            (Fx::Ready, Tgt::Mine(i)) => self.sides[p.idx()].board[i as usize].exerted = false,
            (Fx::Banish, Tgt::Foe(i)) => {
                let c = &mut self.sides[q.idx()].board[i as usize];
                c.dmg = c.will();
            }
            (Fx::Fame(n), _) => self.sides[p.idx()].fame += n,
            (Fx::Boost(n), _) => {
                for c in &mut self.sides[p.idx()].board {
                    c.bstr += n;
                }
            }
            _ => {}
        }
    }

    fn char_val(c: &Chr) -> i32 {
        let (_, str, will, lore, kw) = c.stats();
        let mut v = 2 * str as i32 + will as i32 + 8 * lore as i32 - c.dmg as i32;
        for (f, w) in [
            (RUSH, 1),
            (GUARD, 2),
            (EVASIVE, 4),
            (CHALLENGER, 2),
            (RESIST, 3),
        ] {
            if kw & f != 0 {
                v += w;
            }
        }
        v
    }

    fn side_val(&self, p: Player) -> i32 {
        let s = &self.sides[p.idx()];
        30 * s.fame.min(WIN_FAME + 3) as i32
            + 3 * s.hand.len() as i32
            + 6 * s.well.len() as i32
            + s.board.iter().map(Self::char_val).sum::<i32>()
    }
}

impl Game for TcgCerneala {
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
            Phase::Main => {
                let mut v = vec![Move::End];
                for (h, id) in s.hand.iter().enumerate() {
                    let c = &CARDS[*id as usize];
                    if c.ink && !s.inked {
                        v.push(Move::Ink(h as u8));
                    }
                    match c.k {
                        K::Char { cost, .. } => {
                            if cost <= s.ink_left() && s.board.len() < MAXBOARD {
                                v.push(Move::Play(h as u8, Tgt::None));
                            }
                        }
                        K::Action { cost, fx } => {
                            if cost <= s.ink_left() {
                                for t in self.action_targets(p, fx) {
                                    v.push(Move::Play(h as u8, t));
                                }
                            }
                        }
                    }
                }
                for (i, c) in s.board.iter().enumerate() {
                    if c.exerted || (c.fresh && c.kw() & RUSH == 0) {
                        continue;
                    }
                    if !c.fresh {
                        v.push(Move::Quest(i as u8));
                    }
                    for j in self.challenge_targets(p, i) {
                        v.push(Move::Challenge(i as u8, j));
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
        let q = p.other();
        match *m {
            Move::Draw => {
                if self.turn_no > 1 {
                    if self.sides[p.idx()].deck.is_empty() {
                        self.over = Some(Outcome::Win(q));
                        return;
                    }
                    self.draw_card(p);
                }
                self.phase = Phase::Main;
            }
            Move::End => self.end_turn(),
            Move::Ink(h) => {
                let c = self.sides[p.idx()].hand.remove(h as usize);
                let s = &mut self.sides[p.idx()];
                s.well.push(c);
                s.inked = true;
            }
            Move::Play(h, t) => {
                let id = self.sides[p.idx()].hand.remove(h as usize);
                match CARDS[id as usize].k {
                    K::Char { cost, .. } => {
                        let uid = self.next_uid;
                        self.next_uid += 1;
                        let s = &mut self.sides[p.idx()];
                        s.spent += cost;
                        s.board.push(Chr {
                            card: id,
                            uid,
                            dmg: 0,
                            exerted: false,
                            fresh: true,
                            bstr: 0,
                        });
                    }
                    K::Action { cost, fx } => {
                        let s = &mut self.sides[p.idx()];
                        s.spent += cost;
                        s.grave.push(id);
                        self.do_action(p, fx, t);
                    }
                }
                self.reap();
                self.check_fame();
            }
            Move::Quest(i) => {
                let c = &mut self.sides[p.idx()].board[i as usize];
                c.exerted = true;
                let l = c.lore();
                self.sides[p.idx()].fame += l;
                self.check_fame();
            }
            Move::Challenge(i, j) => {
                let a = self.sides[p.idx()].board[i as usize].clone();
                let d = self.sides[q.idx()].board[j as usize].clone();
                let bonus = if a.kw() & CHALLENGER != 0 { 2 } else { 0 };
                let r = |c: &Chr| if c.kw() & RESIST != 0 { 1 } else { 0 };
                let to_d = (a.str() + bonus - r(&d)).max(0);
                let to_a = (d.str() - r(&a)).max(0);
                self.sides[p.idx()].board[i as usize].exerted = true;
                self.sides[p.idx()].board[i as usize].dmg += to_a;
                self.sides[q.idx()].board[j as usize].dmg += to_d;
                self.reap();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.over
    }

    fn evaluate(&self, p: Player) -> i32 {
        self.side_val(p) - self.side_val(p.other())
    }

    fn render(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            "Povestiri din Cerneală — tura {} — {}\n",
            self.turn_no,
            match self.phase {
                Phase::Draw => "pescuire",
                Phase::Main => "faza principală",
            }
        ));
        let v = self.viewer.unwrap_or(self.turn);
        for p in [v.other(), v] {
            let sd = &self.sides[p.idx()];
            let who = if p == Player::One { 1 } else { 2 };
            s.push_str(&format!(
                "Jucătorul {who} ({}): faimă {}/{WIN_FAME} | cerneală {}/{} | mână {} | pachet {} | cimitir {}\n",
                deck_name(p),
                sd.fame,
                sd.ink_left(),
                sd.well.len(),
                sd.hand.len(),
                sd.deck.len(),
                sd.grave.len()
            ));
            if sd.board.is_empty() {
                s.push_str("  Personaje: —\n");
            }
            for (i, c) in sd.board.iter().enumerate() {
                let tag = if p == v { 'm' } else { 'e' };
                let kw = kw_text(c.kw());
                s.push_str(&format!(
                    "  {tag}{}: {} {}/{} faimă {} daune {}{}{}{}\n",
                    i + 1,
                    CARDS[c.card as usize].name,
                    c.str(),
                    c.will(),
                    c.lore(),
                    c.dmg,
                    if kw.is_empty() {
                        String::new()
                    } else {
                        format!(" ({kw})")
                    },
                    if c.exerted { " [înclinat]" } else { "" },
                    if c.fresh { " [se usucă]" } else { "" }
                ));
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
        match m {
            Move::Draw => "pesca".into(),
            Move::End => "fin".into(),
            Move::Ink(h) => format!("cerneala {}", h + 1),
            Move::Play(h, t) => match t {
                Tgt::None => format!("joaca {}", h + 1),
                Tgt::Foe(i) => format!("joaca {} e{}", h + 1, i + 1),
                Tgt::Mine(i) => format!("joaca {} m{}", h + 1, i + 1),
            },
            Move::Quest(i) => format!("misiune {}", i + 1),
            Move::Challenge(i, j) => format!("provocare {} e{}", i + 1, j + 1),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn main_phase(seed: u64) -> TcgCerneala {
        let mut g = TcgCerneala::with_seed(seed);
        g.apply(&Move::Draw);
        g
    }

    fn put(g: &mut TcgCerneala, p: Player, name: &str, fresh: bool, exerted: bool) -> usize {
        let uid = g.next_uid;
        g.next_uid += 1;
        let s = &mut g.sides[p.idx()];
        s.board.push(Chr {
            card: card_id(name),
            uid,
            dmg: 0,
            exerted,
            fresh,
            bstr: 0,
        });
        s.board.len() - 1
    }

    fn give_ink(g: &mut TcgCerneala, p: Player, n: usize) {
        let s = &mut g.sides[p.idx()];
        // cărți din călimară luate din pachet, ca să se păstreze totalul
        for _ in 0..n {
            let c = s.deck.pop().unwrap();
            s.well.push(c);
        }
    }

    #[test]
    fn set_and_decks_are_valid() {
        assert!(CARDS.len() >= 40, "{}", CARDS.len());
        let mut n: Vec<&str> = CARDS.iter().map(|c| c.name).collect();
        n.sort();
        n.dedup();
        assert_eq!(n.len(), CARDS.len());
        for p in [Player::One, Player::Two] {
            let d = deck_list(p);
            assert_eq!(d.len(), 30, "{}", deck_name(p));
            for id in 0..CARDS.len() as u8 {
                assert!(d.iter().filter(|x| **x == id).count() <= 4);
            }
            let inkable = d.iter().filter(|c| CARDS[**c as usize].ink).count();
            assert!(inkable >= 18, "prea puține cărți cu cerneală: {inkable}");
        }
    }

    #[test]
    fn setup_and_first_turn_no_draw() {
        let mut g = TcgCerneala::with_seed(1);
        for s in &g.sides {
            assert_eq!(s.hand.len(), 7);
            assert_eq!(s.deck.len(), 23);
        }
        g.apply(&Move::Draw);
        assert_eq!(g.sides[0].hand.len(), 7);
        g.apply(&Move::End);
        g.apply(&Move::Draw);
        assert_eq!(g.sides[1].hand.len(), 8);
    }

    #[test]
    fn ink_once_per_turn_and_only_inkable() {
        let mut g = main_phase(2);
        g.sides[0].hand = vec![
            card_id("Licurici Cântător"),
            card_id("Pitic de Lumină"),
            card_id("Regina Ceții"),
        ];
        let mv = g.legal_moves();
        assert!(mv.contains(&Move::Ink(0)));
        assert!(mv.contains(&Move::Ink(1)));
        assert!(!mv.contains(&Move::Ink(2)), "Regina nu are cerneală");
        g.apply(&Move::Ink(0));
        assert_eq!(g.sides[0].well.len(), 1);
        assert!(!g.legal_moves().iter().any(|m| matches!(m, Move::Ink(_))));
        // 1 cerneală: Pitic (cost 2) nu se poate juca
        assert!(!g.legal_moves().iter().any(|m| matches!(m, Move::Play(..))));
    }

    #[test]
    fn play_costs_ink_and_new_chars_are_drying() {
        let mut g = main_phase(3);
        give_ink(&mut g, Player::One, 3);
        g.sides[0].hand = vec![card_id("Cavaler Mic"), card_id("Pirat de Câmpie")];
        g.apply(&Move::Play(0, Tgt::None));
        assert_eq!(g.sides[0].ink_left(), 0);
        assert!(!g.legal_moves().iter().any(|m| matches!(m, Move::Play(..))));
        // cavalerul se usucă: nu face misiune
        assert!(!g.legal_moves().iter().any(|m| matches!(m, Move::Quest(_))));
        // la începutul turei următoare totul e pregătit
        g.apply(&Move::End);
        g.apply(&Move::Draw);
        g.apply(&Move::End);
        g.apply(&Move::Draw);
        assert_eq!(g.sides[0].ink_left(), 3);
        assert!(g.legal_moves().contains(&Move::Quest(0)));
    }

    #[test]
    fn quest_adds_fame_and_exerts() {
        let mut g = main_phase(4);
        put(&mut g, Player::One, "Vrăjitoarea Lacului", false, false);
        g.apply(&Move::Quest(0));
        assert_eq!(g.sides[0].fame, 3);
        assert!(g.sides[0].board[0].exerted);
        assert!(!g.legal_moves().iter().any(|m| matches!(m, Move::Quest(_))));
    }

    #[test]
    fn rush_can_challenge_immediately_but_not_quest() {
        let mut g = main_phase(5);
        put(&mut g, Player::One, "Pirat de Câmpie", true, false);
        put(&mut g, Player::Two, "Pitic de Lumină", false, true);
        let mv = g.legal_moves();
        assert!(mv.contains(&Move::Challenge(0, 0)));
        assert!(!mv.contains(&Move::Quest(0)));
    }

    #[test]
    fn challenge_only_exerted_targets_and_damage_resolves() {
        let mut g = main_phase(6);
        put(&mut g, Player::One, "Cavaler Mic", false, false); // 3/3
        put(&mut g, Player::Two, "Pitic de Lumină", false, false); // 2/2 pregătit
        assert!(!g
            .legal_moves()
            .iter()
            .any(|m| matches!(m, Move::Challenge(..))));
        g.sides[1].board[0].exerted = true;
        g.apply(&Move::Challenge(0, 0));
        assert!(g.sides[1].board.is_empty(), "pitic eliminat");
        assert_eq!(g.sides[0].board[0].dmg, 2);
        assert!(g.sides[0].board[0].exerted);
        assert_eq!(g.sides[1].grave.len(), 1);
    }

    #[test]
    fn guard_evasive_challenger_resist() {
        let mut g = main_phase(7);
        put(&mut g, Player::One, "Cavaler Mic", false, false);
        put(&mut g, Player::Two, "Pitic de Lumină", false, true);
        put(&mut g, Player::Two, "Ursuleț de Cetate", false, true); // gardian
        let t = g.challenge_targets(Player::One, 0);
        assert_eq!(t, vec![1], "doar gardianul");
        // zburătorul nu poate fi provocat de un nezburător
        let mut g = main_phase(7);
        put(&mut g, Player::One, "Cavaler Mic", false, false);
        put(&mut g, Player::Two, "Libelula Argintie", false, true);
        assert!(g.challenge_targets(Player::One, 0).is_empty());
        put(&mut g, Player::One, "Grifon", false, false);
        assert_eq!(g.challenge_targets(Player::One, 1), vec![0]);
        // provocator +2, rezistență -1
        let mut g = main_phase(7);
        put(&mut g, Player::One, "Lupul Cenușiu", false, false); // str 4 (+2)
        put(&mut g, Player::Two, "Ursul Mare", false, true); // will 5, rezist 1
        g.sides[1].board[0].dmg = 0;
        g.apply(&Move::Challenge(0, 0));
        // 4+2 provocator -1 rezistență = 5 daune = voința ursului; lupul primește 5 ≥ 3
        assert!(g.sides[1].board.is_empty());
        assert!(g.sides[0].board.is_empty());
        // fără Provocator (Cavaler Mic 3 forță, -1) ursul rămâne în picioare
        let mut g = main_phase(7);
        put(&mut g, Player::One, "Cavaler Mic", false, false);
        put(&mut g, Player::Two, "Ursul Mare", false, true);
        g.apply(&Move::Challenge(0, 0));
        assert_eq!(g.sides[1].board[0].dmg, 2);
    }

    #[test]
    fn actions_resolve() {
        let mut g = main_phase(8);
        give_ink(&mut g, Player::One, 8);
        put(&mut g, Player::Two, "Zâna Pădurii", false, true);
        put(&mut g, Player::Two, "Pitic de Lumină", false, false);
        put(&mut g, Player::One, "Cavaler Mic", false, true);
        g.sides[0].hand = vec![
            card_id("Sentința Lunii"),
            card_id("Leac de Mierlă"),
            card_id("Chemarea Vitejilor"),
            card_id("Ploaie de Povești"),
            card_id("Izbucnire de Faimă"),
        ];
        let mv = g.legal_moves();
        assert!(mv.contains(&Move::Play(0, Tgt::Foe(0))));
        assert!(!mv.contains(&Move::Play(0, Tgt::Foe(1))), "doar înclinați");
        assert!(
            !mv.iter().any(|m| matches!(m, Move::Play(1, _))),
            "nimic de vindecat"
        );
        g.apply(&Move::Play(0, Tgt::Foe(0)));
        assert_eq!(g.sides[1].board.len(), 1);
        // Chemarea Vitejilor este acum la indexul 1
        g.apply(&Move::Play(1, Tgt::Mine(0)));
        assert!(!g.sides[0].board[0].exerted);
        let hand = g.sides[0].hand.len();
        g.apply(&Move::Play(1, Tgt::None)); // Ploaie de Povești
        assert_eq!(g.sides[0].hand.len(), hand - 1 + 2);
        let h = g.sides[0]
            .hand
            .iter()
            .position(|c| *c == card_id("Izbucnire de Faimă"))
            .unwrap();
        g.apply(&Move::Play(h as u8, Tgt::None));
        assert_eq!(g.sides[0].fame, 2);
    }

    #[test]
    fn damage_persists_heal_and_aoe() {
        let mut g = main_phase(9);
        give_ink(&mut g, Player::One, 8);
        put(&mut g, Player::Two, "Ursul Mare", false, false);
        put(&mut g, Player::Two, "Pitic de Lumină", false, false);
        g.sides[0].hand = vec![card_id("Furtună de Cenușă"), card_id("Rafală")];
        g.apply(&Move::Play(0, Tgt::None));
        assert_eq!(g.sides[1].board.len(), 1, "piticul (voință 2) e eliminat");
        assert_eq!(g.sides[1].board[0].dmg, 2);
        g.apply(&Move::End);
        g.apply(&Move::Draw);
        g.apply(&Move::End);
        assert_eq!(g.sides[1].board[0].dmg, 2, "daunele rămân între ture");
    }

    #[test]
    fn fame_win_and_deck_out() {
        let mut g = main_phase(10);
        g.sides[0].fame = 12;
        put(&mut g, Player::One, "Bătrâna Povestitoare", false, false);
        g.apply(&Move::Quest(0));
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
        assert!(g.legal_moves().is_empty());
        let mut g = main_phase(10);
        g.apply(&Move::End);
        g.sides[1].deck.clear();
        g.apply(&Move::Draw);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn boost_lasts_one_turn() {
        let mut g = main_phase(11);
        give_ink(&mut g, Player::One, 2);
        put(&mut g, Player::One, "Cavaler Mic", false, false);
        g.sides[0].hand = vec![card_id("Strigăt de Luptă")];
        g.apply(&Move::Play(0, Tgt::None));
        assert_eq!(g.sides[0].board[0].str(), 4);
        g.apply(&Move::End);
        assert_eq!(g.sides[0].board[0].str(), 3);
    }

    #[test]
    fn render_hides_opponent_hand_and_well() {
        let g = TcgCerneala::with_seed(5);
        let r = g.render();
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
        a.apply(&Move::End);
        a.apply(&Move::Draw);
        let mut b = a.clone();
        b.sides[0].hand = a.sides[0].hand.iter().rev().cloned().collect();
        b.sides[0].deck.reverse();
        let n = b.sides[0].hand.len();
        b.sides[0].hand = (0..n).map(|_| card_id("Marele Înțelept")).collect();
        assert_eq!(a.evaluate(Player::Two), b.evaluate(Player::Two));
    }

    #[test]
    fn cards_conserved_over_random_games() {
        for seed in 1..=15u64 {
            let mut g = TcgCerneala::with_seed(seed);
            let mut rng = Rng::new(seed * 11 + 5);
            let mut n = 0;
            while g.outcome().is_none() && n < 6000 {
                let mv = g.legal_moves();
                assert!(!mv.is_empty(), "{}", g.render());
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
    fn ai_plays_legal_games_and_beats_random() {
        use polytabletop_core::GameObj;
        let mut wins = 0;
        for seed in 1..=8u64 {
            let mut g: Box<dyn GameObj> = Box::new(TcgCerneala::with_seed(seed));
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() && n < 6000 {
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
        let mut g = TcgCerneala::with_seed(5);
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
        let mut g = TcgCerneala::with_seed(5);
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
