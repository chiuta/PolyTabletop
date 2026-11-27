//! Motor de cărți colecționabile „pitch / lanț de răspuns + echipament” (id
//! `tcg_lant`), 2 jucători, cu un set ORIGINAL de cărți („Seria Lanțurilor de
//! Oțel”). Reguli scrise de noi, din mecanici generice.
//!
//! * Fiecare jucător are un erou (30 de viață, mână de 4 cărți), patru piese de
//!   echipament pregătite (cap, piept, brațe, picioare; fiecare cu apărare și
//!   „folosiri”) și o armă. Pachet pregătit de 30 de cărți (J1 „Gardiana
//!   Zorilor”, J2 „Vânătorul de Furtuni”).
//! * Resursele vin din **pitch**: fiecare carte are o valoare de pitch (1–3).
//!   Costul unei cărți se plătește aruncând în „zona de pitch” cărți din mână
//!   a căror valoare însumează cel puțin costul (se alege mulțimea minimă;
//!   surplusul se pierde). La sfârșitul turei cărțile din pitch merg pe FUNDUL
//!   pachetului.
//! * Tura: `pesca` (forțat: completezi mâna până la 4 cărți), apoi ai **1 punct
//!   de acțiune**. Mutări: `atac 2 pitch 1,3` (carte de atac din mână),
//!   `arma pitch 2` (atacul armei, o dată pe tură), `joaca 3 pitch 1` (acțiune
//!   fără atac: vindecă, pescuiește sau dă +putere următorului atac; costă
//!   resurse, nu și punctul de acțiune), `fin`. Un atac cu „continuă” dă
//!   înapoi punctul de acțiune după ce se încheie lanțul.
//! * Lanțul unui atac: apărătorul alege `bloc 1,3 echip 2` (cărți din mână,
//!   cel mult 3, și piese de echipament; fiecare piesă folosită pierde o
//!   folosire și dispare la zero) sau `bloc fara`. Apoi atacatorul poate juca
//!   reacții de atac (`reactie 2 pitch 1`, +putere) până spune `pasa`, apoi
//!   apărătorul reacții de apărare (+apărare). Daune = putere – apărare.
//! * Câștigă cine aduce adversarul la 0 viață. După 25 de ture pe jucător,
//!   câștigă cine are mai multă viață (egal = remiză).
//!
//! Informația ascunsă: `render()` arată doar mâna J1; AI-ul (AlphaBeta(1)) are
//! `evaluate` care citește doar informația vizibilă (câmp, echipament, NUMĂRUL
//! cărților adverse); apărarea adversarului se estimează din echipamentul vizibil
//! și din numărul cărților lui.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const HAND: usize = 4;
const START_LIFE: i16 = 30;
const MAX_TURNS: u32 = 50;
const SLOTS: [&str; 4] = ["cap", "piept", "brațe", "picioare"];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Fx {
    Heal(i16),
    Draw(u8),
    Aim(i16),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum K {
    Atk { cost: u8, pow: i16, go: bool },
    ReactAtk { cost: u8, bonus: i16 },
    ReactDef { cost: u8, bonus: i16 },
    Act { cost: u8, fx: Fx },
    Equip { slot: u8, def: i16, uses: u8 },
    Weapon { cost: u8, pow: i16 },
}

pub struct Card {
    pub name: &'static str,
    pub k: K,
    /// Valoarea de pitch (0 pentru echipament și arme).
    pub pitch: u8,
    /// Valoarea de apărare când blochează din mână.
    pub def: i16,
}

const fn atk(name: &'static str, cost: u8, pow: i16, def: i16, pitch: u8, go: bool) -> Card {
    Card {
        name,
        k: K::Atk { cost, pow, go },
        pitch,
        def,
    }
}
const fn ra(name: &'static str, cost: u8, bonus: i16, def: i16, pitch: u8) -> Card {
    Card {
        name,
        k: K::ReactAtk { cost, bonus },
        pitch,
        def,
    }
}
const fn rd(name: &'static str, cost: u8, bonus: i16, def: i16, pitch: u8) -> Card {
    Card {
        name,
        k: K::ReactDef { cost, bonus },
        pitch,
        def,
    }
}
const fn act(name: &'static str, cost: u8, fx: Fx, def: i16, pitch: u8) -> Card {
    Card {
        name,
        k: K::Act { cost, fx },
        pitch,
        def,
    }
}
const fn eq(name: &'static str, slot: u8, def: i16, uses: u8) -> Card {
    Card {
        name,
        k: K::Equip { slot, def, uses },
        pitch: 0,
        def: 0,
    }
}
const fn wp(name: &'static str, cost: u8, pow: i16) -> Card {
    Card {
        name,
        k: K::Weapon { cost, pow },
        pitch: 0,
        def: 0,
    }
}

/// Setul complet: 42 de cărți originale (32 de pachet + 10 de echipament/arme).
pub const CARDS: &[Card] = &[
    // --- Gardiana Zorilor
    atk("Lovitură de Scut", 1, 4, 3, 2, false),
    atk("Pas de Santinelă", 0, 2, 2, 3, true),
    atk("Ciocanul Zorilor", 3, 8, 2, 1, false),
    atk("Tăișul Răbdării", 2, 5, 3, 2, false),
    atk("Stăvilar", 1, 3, 3, 2, true),
    atk("Iureșul Gărzii", 2, 6, 2, 1, false),
    atk("Strigătul Gărzii", 0, 3, 2, 3, false),
    atk("Ruptura Zidului", 3, 9, 2, 1, false),
    atk("Maiul Mare", 4, 11, 2, 1, false),
    ra("Împunsătură de Sprijin", 1, 3, 2, 2),
    rd("Paravan de Oțel", 1, 5, 3, 2),
    rd("Scut Ridicat", 0, 2, 2, 3),
    rd("Ultima Linie", 2, 7, 3, 1),
    act("Rugăciune de Oțel", 1, Fx::Heal(4), 2, 2),
    act("Cumpătare", 0, Fx::Aim(3), 1, 3),
    act("Revelație", 1, Fx::Draw(2), 1, 2),
    // --- Vânătorul de Furtuni
    atk("Săgeată Rapidă", 0, 2, 2, 3, true),
    atk("Colț de Lup", 1, 4, 2, 2, false),
    atk("Salt din Umbră", 1, 3, 2, 2, true),
    atk("Furtuna de Săgeți", 2, 6, 2, 1, false),
    atk("Lance de Ceață", 3, 9, 2, 1, false),
    atk("Răsunet de Corn", 0, 3, 2, 3, false),
    atk("Vânătoarea Nopții", 2, 5, 2, 2, false),
    atk("Săgeata Furtunii", 4, 12, 2, 1, false),
    ra("Țintă Sigură", 1, 4, 2, 2),
    ra("Lovitură de Grație", 2, 6, 2, 1),
    ra("Foc Încrucișat", 0, 1, 2, 3),
    rd("Pas Lateral", 0, 2, 2, 3),
    rd("Piele Groasă", 1, 4, 3, 2),
    act("Ochire", 0, Fx::Aim(3), 1, 3),
    act("Leac din Pădure", 1, Fx::Heal(3), 2, 2),
    act("Instinct", 1, Fx::Draw(2), 1, 2),
    // --- Echipament și arme (Gardiana)
    eq("Coif de Fier", 0, 1, 3),
    eq("Platoșă Veche", 1, 2, 3),
    eq("Brățări de Aramă", 2, 1, 3),
    eq("Cizme Grele", 3, 1, 3),
    wp("Ciocan de Pază", 1, 3),
    // --- Echipament și arme (Vânătorul)
    eq("Glugă de Piele", 0, 1, 2),
    eq("Vestă de Piele", 1, 1, 3),
    eq("Mănuși de Arcaș", 2, 1, 2),
    eq("Ghete de Vânat", 3, 1, 2),
    wp("Pumnal de Vânător", 0, 2),
];

pub fn card_id(name: &str) -> u8 {
    CARDS
        .iter()
        .position(|c| c.name == name)
        .unwrap_or_else(|| panic!("carte necunoscută: {name}")) as u8
}

const DECK_GARDIANA: &[(&str, u8)] = &[
    ("Lovitură de Scut", 2),
    ("Pas de Santinelă", 2),
    ("Ciocanul Zorilor", 2),
    ("Tăișul Răbdării", 2),
    ("Stăvilar", 2),
    ("Iureșul Gărzii", 2),
    ("Strigătul Gărzii", 2),
    ("Ruptura Zidului", 2),
    ("Maiul Mare", 1),
    ("Împunsătură de Sprijin", 2),
    ("Paravan de Oțel", 2),
    ("Scut Ridicat", 2),
    ("Ultima Linie", 1),
    ("Rugăciune de Oțel", 2),
    ("Cumpătare", 2),
    ("Revelație", 2),
];

const DECK_VANATORUL: &[(&str, u8)] = &[
    ("Săgeată Rapidă", 2),
    ("Colț de Lup", 2),
    ("Salt din Umbră", 2),
    ("Furtuna de Săgeți", 3),
    ("Lance de Ceață", 2),
    ("Răsunet de Corn", 2),
    ("Vânătoarea Nopții", 2),
    ("Săgeata Furtunii", 1),
    ("Țintă Sigură", 2),
    ("Lovitură de Grație", 1),
    ("Foc Încrucișat", 2),
    ("Pas Lateral", 2),
    ("Piele Groasă", 2),
    ("Ochire", 2),
    ("Leac din Pădure", 1),
    ("Instinct", 2),
];

const EQUIP_GARDIANA: [&str; 4] = [
    "Coif de Fier",
    "Platoșă Veche",
    "Brățări de Aramă",
    "Cizme Grele",
];
const EQUIP_VANATORUL: [&str; 4] = [
    "Glugă de Piele",
    "Vestă de Piele",
    "Mănuși de Arcaș",
    "Ghete de Vânat",
];

pub fn deck_list(p: Player) -> Vec<u8> {
    let src = if p == Player::One {
        DECK_GARDIANA
    } else {
        DECK_VANATORUL
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
        "Gardiana Zorilor"
    } else {
        "Vânătorul de Furtuni"
    }
}

fn fx_text(f: Fx) -> String {
    match f {
        Fx::Heal(n) => format!("vindeci {n}"),
        Fx::Draw(n) => format!("pescuiești {n}"),
        Fx::Aim(n) => format!("următorul atac +{n} putere"),
    }
}

pub fn card_line(id: u8) -> String {
    let c = &CARDS[id as usize];
    match c.k {
        K::Atk { cost, pow, go } => format!(
            "{} (atac cost {cost}, putere {pow}{}; apărare {}, pitch {})",
            c.name,
            if go { ", continuă" } else { "" },
            c.def,
            c.pitch
        ),
        K::ReactAtk { cost, bonus } => format!(
            "{} (reacție de atac cost {cost}, +{bonus} putere; apărare {}, pitch {})",
            c.name, c.def, c.pitch
        ),
        K::ReactDef { cost, bonus } => format!(
            "{} (reacție de apărare cost {cost}, +{bonus}; apărare {}, pitch {})",
            c.name, c.def, c.pitch
        ),
        K::Act { cost, fx } => format!(
            "{} (acțiune cost {cost}: {}; apărare {}, pitch {})",
            c.name,
            fx_text(fx),
            c.def,
            c.pitch
        ),
        K::Equip { slot, def, uses } => format!(
            "{} (echipament {}: apărare {def}, {uses} folosiri)",
            c.name, SLOTS[slot as usize]
        ),
        K::Weapon { cost, pow } => format!("{} (armă cost {cost}, putere {pow})", c.name),
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub card: u8,
    pub uses: u8,
}

impl Piece {
    fn def(&self) -> i16 {
        match CARDS[self.card as usize].k {
            K::Equip { def, .. } => def,
            _ => 0,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Side {
    pub life: i16,
    /// Vârful pachetului = sfârșitul vectorului.
    pub deck: Vec<u8>,
    pub hand: Vec<u8>,
    pub pitch: Vec<u8>,
    pub chain: Vec<u8>,
    pub grave: Vec<u8>,
    pub eq: [Option<Piece>; 4],
    pub weapon: u8,
}

impl Side {
    pub fn total_cards(&self) -> usize {
        self.deck.len() + self.hand.len() + self.pitch.len() + self.chain.len() + self.grave.len()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Draw,
    /// (mână, masca cărților de pitch)
    Atk(u8, u8),
    Weapon(u8),
    Act(u8, u8),
    End,
    /// (masca cărților din mână, masca pieselor de echipament)
    Block(u8, u8),
    ReactA(u8, u8),
    ReactD(u8, u8),
    Pass,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Phase {
    Draw,
    Main,
    Defend,
    ReactA,
    ReactD,
}

#[derive(Clone, Copy, Debug)]
struct Pend {
    pow: i16,
    bonus: i16,
    defs: i16,
    defr: i16,
    go: bool,
}

#[derive(Clone)]
pub struct TcgLant {
    pub sides: [Side; 2],
    pub turn: Player,
    /// Jucătorul a cărui tură este.
    pub cur: Player,
    pub phase: Phase,
    pub turn_no: u32,
    pub ap: u8,
    pub aim: i16,
    pub weapon_used: bool,
    pend: Option<Pend>,
    over: Option<Outcome>,
    /// `Some(p)` = render shows `p`'s hand; `None` = hot-seat (the mover's).
    viewer: Option<Player>,
}

impl Default for TcgLant {
    fn default() -> Self {
        Self::new()
    }
}

fn shuffle(v: &mut [u8], rng: &mut Rng) {
    for i in (1..v.len()).rev() {
        v.swap(i, rng.below(i + 1));
    }
}

fn pitch_of(id: u8) -> u8 {
    CARDS[id as usize].pitch
}

fn cost_of(id: u8) -> u8 {
    match CARDS[id as usize].k {
        K::Atk { cost, .. }
        | K::ReactAtk { cost, .. }
        | K::ReactDef { cost, .. }
        | K::Act { cost, .. }
        | K::Weapon { cost, .. } => cost,
        K::Equip { .. } => 0,
    }
}

/// Mulțimile minime de cărți din mână (fără `skip`) al căror pitch acoperă `cost`.
fn pitch_masks(hand: &[u8], skip: Option<usize>, cost: u8) -> Vec<u8> {
    if cost == 0 {
        return vec![0];
    }
    let idxs: Vec<usize> = (0..hand.len()).filter(|i| Some(*i) != skip).collect();
    let n = idxs.len();
    let mut out = Vec::new();
    for sub in 1u32..(1 << n) {
        let mut sum = 0u32;
        for (b, i) in idxs.iter().enumerate() {
            if sub & (1 << b) != 0 {
                sum += pitch_of(hand[*i]) as u32;
            }
        }
        if sum < cost as u32 {
            continue;
        }
        let minimal = idxs
            .iter()
            .enumerate()
            .all(|(b, i)| sub & (1 << b) == 0 || sum - (pitch_of(hand[*i]) as u32) < cost as u32);
        if minimal {
            let mut mask = 0u8;
            for (b, i) in idxs.iter().enumerate() {
                if sub & (1 << b) != 0 {
                    mask |= 1 << i;
                }
            }
            out.push(mask);
        }
    }
    out
}

fn bits(mask: u8) -> Vec<usize> {
    (0..8).filter(|i| mask & (1 << i) != 0).collect()
}

impl TcgLant {
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
            for _ in 0..HAND {
                s.hand.push(d.pop().unwrap());
            }
            s.deck = d;
            let (names, weapon) = if p == Player::One {
                (EQUIP_GARDIANA, "Ciocan de Pază")
            } else {
                (EQUIP_VANATORUL, "Pumnal de Vânător")
            };
            for n in names {
                let id = card_id(n);
                if let K::Equip { slot, uses, .. } = CARDS[id as usize].k {
                    s.eq[slot as usize] = Some(Piece { card: id, uses });
                }
            }
            s.weapon = card_id(weapon);
        }
        TcgLant {
            sides,
            turn: Player::One,
            cur: Player::One,
            phase: Phase::Draw,
            turn_no: 1,
            ap: 1,
            aim: 0,
            weapon_used: false,
            pend: None,
            over: None,
            viewer: Some(Player::One),
        }
    }

    pub fn side(&self, p: Player) -> &Side {
        &self.sides[p.idx()]
    }

    fn check_life(&mut self) {
        let a = self.sides[0].life <= 0;
        let b = self.sides[1].life <= 0;
        self.over = match (a, b) {
            (true, true) => Some(Outcome::Draw),
            (true, false) => Some(Outcome::Win(Player::Two)),
            (false, true) => Some(Outcome::Win(Player::One)),
            _ => None,
        };
    }

    fn end_turn(&mut self) {
        for s in &mut self.sides {
            // cărțile din pitch merg pe fundul pachetului
            let mut p: Vec<u8> = s.pitch.drain(..).collect();
            p.append(&mut s.deck);
            s.deck = p;
        }
        let n = self.cur.other();
        self.cur = n;
        self.turn = n;
        self.turn_no += 1;
        self.phase = Phase::Draw;
        self.ap = 1;
        self.aim = 0;
        self.weapon_used = false;
        if self.turn_no > MAX_TURNS {
            let (a, b) = (self.sides[0].life, self.sides[1].life);
            self.over = Some(match a.cmp(&b) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                std::cmp::Ordering::Equal => Outcome::Draw,
            });
        }
    }

    /// Scoate din mână (indici în ordine descrescătoare) și pune cărțile de pitch.
    fn pay(&mut self, p: Player, h: Option<usize>, mask: u8) -> Option<u8> {
        let s = &mut self.sides[p.idx()];
        let mut all: Vec<usize> = bits(mask);
        if let Some(h) = h {
            all.push(h);
        }
        all.sort_unstable_by(|a, b| b.cmp(a));
        let mut main = None;
        for i in all {
            let c = s.hand.remove(i);
            if Some(i) == h {
                main = Some(c);
            } else {
                s.pitch.push(c);
            }
        }
        main
    }

    fn launch(&mut self, pow: i16, go: bool) {
        let bonus_aim = self.aim;
        self.aim = 0;
        self.pend = Some(Pend {
            pow: pow + bonus_aim,
            bonus: 0,
            defs: 0,
            defr: 0,
            go,
        });
        self.ap -= 1;
        self.phase = Phase::Defend;
        self.turn = self.cur.other();
    }

    fn resolve(&mut self) {
        let pd = self.pend.take().unwrap();
        let a = self.cur;
        let d = a.other();
        let dmg = (pd.pow + pd.bonus - pd.defs - pd.defr).max(0);
        self.sides[d.idx()].life -= dmg;
        let ch: Vec<u8> = self.sides[a.idx()].chain.drain(..).collect();
        self.sides[a.idx()].grave.extend(ch);
        if pd.go {
            self.ap += 1;
        }
        self.phase = Phase::Main;
        self.turn = a;
        self.check_life();
    }

    fn eq_val(s: &Side) -> i32 {
        s.eq.iter()
            .flatten()
            .map(|e| e.def() as i32 * e.uses as i32)
            .sum()
    }

    fn side_val(&self, p: Player) -> i32 {
        let s = &self.sides[p.idx()];
        let mut v = 5 * s.life as i32 + 3 * s.hand.len() as i32 + 2 * s.pitch.len() as i32;
        v += Self::eq_val(s);
        if p == self.cur {
            v += 2 * self.aim as i32;
        }
        v
    }

    fn base_diff(&self, p: Player) -> i32 {
        self.side_val(p) - self.side_val(p.other())
    }

    /// Daunele estimate ale atacului în curs, din informația vizibilă.
    fn pending_dmg(&self) -> i32 {
        let Some(pd) = self.pend else {
            return 0;
        };
        let d = &self.sides[self.cur.other().idx()];
        let def = if self.phase == Phase::Defend {
            let e: i16 = d.eq.iter().flatten().map(|x| x.def()).sum();
            e + 2 * (d.hand.len().min(2) as i16)
        } else {
            pd.defs + pd.defr
        };
        (pd.pow + pd.bonus - def).max(0) as i32
    }
}

impl Game for TcgLant {
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
                    let c = cost_of(*id);
                    match CARDS[*id as usize].k {
                        K::Atk { .. } if self.ap >= 1 => {
                            for m in pitch_masks(&s.hand, Some(h), c) {
                                v.push(Move::Atk(h as u8, m));
                            }
                        }
                        K::Act { .. } => {
                            for m in pitch_masks(&s.hand, Some(h), c) {
                                v.push(Move::Act(h as u8, m));
                            }
                        }
                        _ => {}
                    }
                }
                if self.ap >= 1 && !self.weapon_used {
                    for m in pitch_masks(&s.hand, None, cost_of(s.weapon)) {
                        v.push(Move::Weapon(m));
                    }
                }
                v
            }
            Phase::Defend => {
                let n = s.hand.len();
                let eqm: Vec<u8> = {
                    let slots: Vec<usize> = (0..4).filter(|i| s.eq[*i].is_some()).collect();
                    (0u32..(1 << slots.len()))
                        .map(|sub| {
                            let mut m = 0u8;
                            for (b, i) in slots.iter().enumerate() {
                                if sub & (1 << b) != 0 {
                                    m |= 1 << i;
                                }
                            }
                            m
                        })
                        .collect()
                };
                let mut v = Vec::new();
                for hm in 0u32..(1 << n) {
                    if hm.count_ones() > 3 {
                        continue;
                    }
                    for em in &eqm {
                        v.push(Move::Block(hm as u8, *em));
                    }
                }
                v
            }
            Phase::ReactA => {
                let mut v = vec![Move::Pass];
                for (h, id) in s.hand.iter().enumerate() {
                    if let K::ReactAtk { cost, .. } = CARDS[*id as usize].k {
                        for m in pitch_masks(&s.hand, Some(h), cost) {
                            v.push(Move::ReactA(h as u8, m));
                        }
                    }
                }
                v
            }
            Phase::ReactD => {
                let mut v = vec![Move::Pass];
                for (h, id) in s.hand.iter().enumerate() {
                    if let K::ReactDef { cost, .. } = CARDS[*id as usize].k {
                        for m in pitch_masks(&s.hand, Some(h), cost) {
                            v.push(Move::ReactD(h as u8, m));
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
                let s = &mut self.sides[p.idx()];
                while s.hand.len() < HAND {
                    match s.deck.pop() {
                        Some(c) => s.hand.push(c),
                        None => break,
                    }
                }
                self.phase = Phase::Main;
            }
            Move::End => self.end_turn(),
            Move::Atk(h, mask) => {
                let id = self.pay(p, Some(h as usize), mask).unwrap();
                let K::Atk { pow, go, .. } = CARDS[id as usize].k else {
                    panic!("nu e atac")
                };
                self.sides[p.idx()].chain.push(id);
                self.launch(pow, go);
            }
            Move::Weapon(mask) => {
                self.pay(p, None, mask);
                let w = self.sides[p.idx()].weapon;
                let K::Weapon { pow, .. } = CARDS[w as usize].k else {
                    panic!("nu e armă")
                };
                self.weapon_used = true;
                self.launch(pow, false);
            }
            Move::Act(h, mask) => {
                let id = self.pay(p, Some(h as usize), mask).unwrap();
                let K::Act { fx, .. } = CARDS[id as usize].k else {
                    panic!("nu e acțiune")
                };
                self.sides[p.idx()].grave.push(id);
                match fx {
                    Fx::Heal(n) => {
                        let s = &mut self.sides[p.idx()];
                        s.life = (s.life + n).min(START_LIFE);
                    }
                    Fx::Draw(n) => {
                        let s = &mut self.sides[p.idx()];
                        for _ in 0..n {
                            if let Some(c) = s.deck.pop() {
                                s.hand.push(c);
                            }
                        }
                    }
                    Fx::Aim(n) => self.aim += n,
                }
            }
            Move::Block(hm, em) => {
                let mut defs = 0;
                let s = &mut self.sides[p.idx()];
                for i in bits(hm).into_iter().rev() {
                    let c = s.hand.remove(i);
                    defs += CARDS[c as usize].def;
                    s.grave.push(c);
                }
                for i in bits(em) {
                    let pc = s.eq[i].as_mut().unwrap();
                    defs += pc.def();
                    pc.uses -= 1;
                    if pc.uses == 0 {
                        s.eq[i] = None;
                    }
                }
                self.pend.as_mut().unwrap().defs = defs;
                self.phase = Phase::ReactA;
                self.turn = self.cur;
            }
            Move::ReactA(h, mask) => {
                let id = self.pay(p, Some(h as usize), mask).unwrap();
                let K::ReactAtk { bonus, .. } = CARDS[id as usize].k else {
                    panic!("nu e reacție de atac")
                };
                self.sides[p.idx()].grave.push(id);
                self.pend.as_mut().unwrap().bonus += bonus;
            }
            Move::ReactD(h, mask) => {
                let id = self.pay(p, Some(h as usize), mask).unwrap();
                let K::ReactDef { bonus, .. } = CARDS[id as usize].k else {
                    panic!("nu e reacție de apărare")
                };
                self.sides[p.idx()].grave.push(id);
                self.pend.as_mut().unwrap().defr += bonus;
            }
            Move::Pass => {
                if self.phase == Phase::ReactA {
                    self.phase = Phase::ReactD;
                    self.turn = self.cur.other();
                } else {
                    self.resolve();
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.over
    }

    fn evaluate(&self, p: Player) -> i32 {
        let mut v = self.base_diff(p);
        let d = 5 * self.pending_dmg();
        if self.pend.is_some() {
            if p == self.cur {
                v += d;
            } else {
                v -= d;
            }
        }
        v
    }

    fn render(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            "Lanțuri de Oțel — tura {} — {}\n",
            self.turn_no,
            match self.phase {
                Phase::Draw => "pescuire",
                Phase::Main => "faza principală",
                Phase::Defend => "apărare (bloc)",
                Phase::ReactA => "reacții de atac",
                Phase::ReactD => "reacții de apărare",
            }
        ));
        let v = self.viewer.unwrap_or(self.turn);
        for p in [v.other(), v] {
            let sd = &self.sides[p.idx()];
            let who = if p == Player::One { 1 } else { 2 };
            s.push_str(&format!(
                "Jucătorul {who} ({}): viață {} | mână {} | pachet {} | pitch {} | aruncate {}\n",
                deck_name(p),
                sd.life,
                sd.hand.len(),
                sd.deck.len(),
                sd.pitch.len(),
                sd.grave.len()
            ));
            let mut parts = Vec::new();
            for (i, e) in sd.eq.iter().enumerate() {
                match e {
                    Some(pc) => parts.push(format!(
                        "{}={} (ap {}, {}x)",
                        i + 1,
                        CARDS[pc.card as usize].name,
                        pc.def(),
                        pc.uses
                    )),
                    None => parts.push(format!("{}=—", i + 1)),
                }
            }
            s.push_str(&format!(
                "  Echipament: {} | armă: {}\n",
                parts.join(", "),
                CARDS[sd.weapon as usize].name
            ));
        }
        if self.over.is_some() {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        s.push_str(&format!(
            "Puncte de acțiune: {} | armă folosită: {}\n",
            self.ap,
            if self.weapon_used { "da" } else { "nu" }
        ));
        if let Some(pd) = self.pend {
            s.push_str(&format!(
                "Atac în lanț: putere {}+{} contra apărare {}+{}\n",
                pd.pow, pd.bonus, pd.defs, pd.defr
            ));
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
        let list = |mask: u8| {
            bits(mask)
                .iter()
                .map(|i| (i + 1).to_string())
                .collect::<Vec<_>>()
                .join(",")
        };
        let with_pitch = |base: String, mask: u8| {
            if mask == 0 {
                base
            } else {
                format!("{base} pitch {}", list(mask))
            }
        };
        match m {
            Move::Draw => "pesca".into(),
            Move::End => "fin".into(),
            Move::Pass => "pasa".into(),
            Move::Atk(h, mk) => with_pitch(format!("atac {}", h + 1), *mk),
            Move::Weapon(mk) => with_pitch("arma".to_string(), *mk),
            Move::Act(h, mk) => with_pitch(format!("joaca {}", h + 1), *mk),
            Move::ReactA(h, mk) | Move::ReactD(h, mk) => {
                with_pitch(format!("reactie {}", h + 1), *mk)
            }
            Move::Block(0, 0) => "bloc fara".into(),
            Move::Block(hm, 0) => format!("bloc {}", list(*hm)),
            Move::Block(0, em) => format!("bloc echip {}", list(*em)),
            Move::Block(hm, em) => format!("bloc {} echip {}", list(*hm), list(*em)),
        }
    }

    fn ai_style(&self) -> AiStyle {
        // Adâncimea 0 = o singură mutare la TOATE nivelurile: la Hard/Expert adâncimea 1 ar extinde
        // răspunsurile adversarului din mâna lui reală (informație ascunsă).
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn main_phase(seed: u64) -> TcgLant {
        let mut g = TcgLant::with_seed(seed);
        g.apply(&Move::Draw);
        g
    }

    fn hand(g: &mut TcgLant, p: Player, names: &[&str]) {
        g.sides[p.idx()].hand = names.iter().map(|n| card_id(n)).collect();
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
                assert!(d.iter().filter(|x| **x == id).count() <= 3);
            }
            assert!(d
                .iter()
                .all(|c| !matches!(CARDS[*c as usize].k, K::Equip { .. } | K::Weapon { .. })));
        }
    }

    #[test]
    fn setup_has_four_cards_and_equipment() {
        let g = TcgLant::with_seed(1);
        for s in &g.sides {
            assert_eq!(s.hand.len(), 4);
            assert_eq!(s.deck.len(), 26);
            assert_eq!(s.life, 30);
            assert!(s.eq.iter().all(|e| e.is_some()));
        }
        assert_eq!(g.legal_moves(), vec![Move::Draw]);
    }

    #[test]
    fn pitch_masks_are_minimal() {
        let h: Vec<u8> = [
            "Ciocanul Zorilor",
            "Strigătul Gărzii",
            "Lovitură de Scut",
            "Stăvilar",
        ]
        .iter()
        .map(|n| card_id(n))
        .collect();
        // pitch: 1, 3, 2, 2 ; cost 3 pentru cartea 0
        let m = pitch_masks(&h, Some(0), 3);
        assert!(m.contains(&0b0010)); // 3
        assert!(m.contains(&0b1100)); // 2+2
        assert!(!m.contains(&0b1110)); // nu e minimă
        assert_eq!(pitch_masks(&h, Some(0), 0), vec![0]);
        assert!(pitch_masks(&h, Some(0), 8).is_empty());
    }

    #[test]
    fn attack_costs_pitch_and_unblocked_damage_hits() {
        let mut g = main_phase(2);
        hand(
            &mut g,
            Player::One,
            &["Tăișul Răbdării", "Strigătul Gărzii", "Stăvilar"],
        );
        // cost 2: pitch 3 (Strigătul)
        let mv = g.legal_moves();
        assert!(mv.contains(&Move::Atk(0, 0b010)));
        assert!(mv.contains(&Move::Atk(0, 0b100)));
        assert!(!mv.contains(&Move::Atk(0, 0)));
        g.apply(&Move::Atk(0, 0b010));
        assert_eq!(g.phase, Phase::Defend);
        assert_eq!(g.turn, Player::Two);
        assert_eq!(g.sides[0].pitch.len(), 1);
        assert_eq!(g.sides[0].hand.len(), 1);
        g.apply(&Move::Block(0, 0));
        assert_eq!(g.phase, Phase::ReactA);
        assert_eq!(g.turn, Player::One);
        g.apply(&Move::Pass);
        assert_eq!(g.phase, Phase::ReactD);
        g.apply(&Move::Pass);
        assert_eq!(g.sides[1].life, 25);
        assert_eq!(g.phase, Phase::Main);
        assert_eq!(g.ap, 0, "fără continuă nu mai ai punct");
        assert!(!g.legal_moves().iter().any(|m| matches!(m, Move::Atk(..))));
        assert_eq!(g.sides[0].chain.len(), 0);
        assert_eq!(g.sides[0].grave.len(), 1);
    }

    #[test]
    fn go_again_returns_action_point() {
        let mut g = main_phase(3);
        hand(&mut g, Player::One, &["Pas de Santinelă", "Stăvilar"]);
        g.apply(&Move::Atk(0, 0));
        g.apply(&Move::Block(0, 0));
        g.apply(&Move::Pass);
        g.apply(&Move::Pass);
        assert_eq!(g.ap, 1);
        assert_eq!(g.sides[1].life, 28);
    }

    #[test]
    fn block_math_and_equipment_wear() {
        let mut g = main_phase(5);
        hand(
            &mut g,
            Player::One,
            &["Ciocanul Zorilor", "Strigătul Gărzii"],
        );
        hand(
            &mut g,
            Player::Two,
            &["Piele Groasă", "Pas Lateral", "Colț de Lup"],
        );
        g.apply(&Move::Atk(0, 0b10)); // putere 8
        let mv = g.legal_moves();
        // cel mult 3 cărți + orice echipament
        assert!(mv.contains(&Move::Block(0b101, 0b0010)));
        assert!(!mv
            .iter()
            .any(|m| matches!(m, Move::Block(h, _) if h.count_ones() > 3)));
        // blochează cu Piele Groasă (3) + Colț de Lup (2) + vesta (1) = 6
        g.apply(&Move::Block(0b101, 0b0010));
        assert_eq!(g.sides[1].grave.len(), 2);
        assert_eq!(g.sides[1].eq[1].unwrap().uses, 2);
        g.apply(&Move::Pass);
        g.apply(&Move::Pass);
        assert_eq!(g.sides[1].life, 30 - 2);
    }

    #[test]
    fn equipment_is_destroyed_at_zero_uses() {
        let mut g = main_phase(6);
        g.sides[1].eq[0].as_mut().unwrap().uses = 1;
        hand(&mut g, Player::One, &["Strigătul Gărzii"]);
        g.apply(&Move::Atk(0, 0));
        g.apply(&Move::Block(0, 0b0001));
        assert!(g.sides[1].eq[0].is_none());
    }

    #[test]
    fn reactions_change_damage() {
        let mut g = main_phase(7);
        hand(
            &mut g,
            Player::One,
            &[
                "Lovitură de Scut",
                "Împunsătură de Sprijin",
                "Strigătul Gărzii",
                "Stăvilar",
            ],
        );
        hand(
            &mut g,
            Player::Two,
            &["Paravan de Oțel", "Scut Ridicat", "Stăvilar"],
        );
        g.apply(&Move::Atk(0, 0b0100)); // cost 1, pitch Stăvilar? idx 2 = Strigătul (3)
        g.apply(&Move::Block(0, 0));
        // atacatorul: reacție +3 (cost 1) plătită cu Stăvilar(2)
        let mv = g.legal_moves();
        let react = mv
            .iter()
            .find(|m| matches!(m, Move::ReactA(..)))
            .copied()
            .expect("reacție de atac");
        g.apply(&react);
        g.apply(&Move::Pass);
        assert_eq!(g.phase, Phase::ReactD);
        // apărătorul: Scut Ridicat (cost 0, +2) și Paravan de Oțel (cost 1, +5)
        let mv = g.legal_moves();
        assert!(mv.contains(&Move::ReactD(1, 0)));
        g.apply(&Move::ReactD(1, 0));
        g.apply(&Move::Pass);
        // putere 4+3 = 7, apărare 2 + 0 -> 5
        assert_eq!(g.sides[1].life, 25);
    }

    #[test]
    fn actions_do_not_use_action_point() {
        let mut g = main_phase(8);
        g.sides[0].life = 20;
        hand(
            &mut g,
            Player::One,
            &[
                "Rugăciune de Oțel",
                "Strigătul Gărzii",
                "Cumpătare",
                "Lovitură de Scut",
            ],
        );
        g.apply(&Move::Act(0, 0b0010));
        assert_eq!(g.sides[0].life, 24);
        assert_eq!(g.ap, 1);
        // cumpătare: +3 la următorul atac (cost 0)
        let h = g.sides[0]
            .hand
            .iter()
            .position(|c| *c == card_id("Cumpătare"))
            .unwrap();
        g.apply(&Move::Act(h as u8, 0));
        assert_eq!(g.aim, 3);
        let a = g.sides[0]
            .hand
            .iter()
            .position(|c| *c == card_id("Lovitură de Scut"))
            .unwrap();
        let _ = a;
        let atk = g
            .legal_moves()
            .into_iter()
            .find(|m| matches!(m, Move::Atk(..)));
        if let Some(m) = atk {
            g.apply(&m);
            g.apply(&Move::Block(0, 0));
            g.apply(&Move::Pass);
            g.apply(&Move::Pass);
            assert!(g.sides[1].life <= 30 - 7);
        }
    }

    #[test]
    fn weapon_attack_once_per_turn_and_end_turn_recycles_pitch() {
        let mut g = main_phase(9);
        hand(&mut g, Player::One, &["Strigătul Gărzii", "Stăvilar"]);
        // Ciocan de Pază: cost 1 putere 3
        let mv = g.legal_moves();
        assert!(mv.iter().any(|m| matches!(m, Move::Weapon(_))));
        let w = mv
            .iter()
            .find(|m| matches!(m, Move::Weapon(x) if *x != 0))
            .copied()
            .unwrap();
        g.apply(&w);
        g.apply(&Move::Block(0, 0b1111));
        g.apply(&Move::Pass);
        g.apply(&Move::Pass);
        assert!(g.weapon_used);
        assert_eq!(g.sides[1].life, 30);
        let deck = g.sides[0].deck.len();
        let pitched = g.sides[0].pitch.len();
        assert_eq!(pitched, 1);
        g.apply(&Move::End);
        assert_eq!(g.sides[0].pitch.len(), 0);
        assert_eq!(g.sides[0].deck.len(), deck + pitched);
        assert_eq!(
            g.sides[0].deck[0],
            card_id("Strigătul Gărzii"),
            "pitch-ul ajunge pe fund"
        );
        assert_eq!(g.turn, Player::Two);
        assert_eq!(g.phase, Phase::Draw);
    }

    #[test]
    fn draw_fills_hand_to_four() {
        let mut g = main_phase(10);
        hand(&mut g, Player::One, &["Stăvilar"]);
        g.apply(&Move::End);
        g.apply(&Move::Draw);
        g.apply(&Move::End);
        g.apply(&Move::Draw);
        assert_eq!(g.sides[0].hand.len(), 4);
    }

    #[test]
    fn lethal_damage_wins_and_turn_cap_decides() {
        let mut g = main_phase(11);
        g.sides[1].life = 2;
        hand(
            &mut g,
            Player::One,
            &["Lovitură de Scut", "Strigătul Gărzii"],
        );
        g.sides[1].eq = [None; 4];
        g.apply(&Move::Atk(0, 0b10));
        g.apply(&Move::Block(0, 0));
        g.apply(&Move::Pass);
        g.apply(&Move::Pass);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));

        let mut g = main_phase(12);
        g.sides[0].life = 10;
        g.turn_no = MAX_TURNS;
        g.apply(&Move::End);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
    }

    #[test]
    fn render_hides_opponent_hand() {
        let g = TcgLant::with_seed(5);
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
        // AI = J2; evaluăm ca J2 cu mâna J1 diferită
        let mut b = a.clone();
        b.sides[0].hand = a.sides[0].hand.iter().rev().cloned().collect();
        b.sides[0].deck.reverse();
        assert_eq!(a.evaluate(Player::Two), b.evaluate(Player::Two));
        let mut c = a.clone();
        let n = c.sides[0].hand.len();
        c.sides[0].hand = (0..n).map(|_| card_id("Maiul Mare")).collect();
        assert_eq!(a.evaluate(Player::Two), c.evaluate(Player::Two));
    }

    #[test]
    fn cards_conserved_over_random_games() {
        for seed in 1..=15u64 {
            let mut g = TcgLant::with_seed(seed);
            let mut rng = Rng::new(seed * 3 + 1);
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
            let mut g: Box<dyn GameObj> = Box::new(TcgLant::with_seed(seed));
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
        let mut g = TcgLant::with_seed(5);
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
        let mut g = TcgLant::with_seed(5);
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
mod a3_audit {
    use super::*;

    #[test]
    fn ai_ignores_hidden_cards_at_every_level() {
        crate::p3_comun::ai_blind_check(
            "tcg_lant",
            TcgLant::with_seed,
            |g, me, s| {
                let h = 1 - me.idx();
                let nh = g.sides[h].hand.len();
                let mut pool: Vec<u8> = g.sides[h].hand.clone();
                pool.extend(g.sides[h].deck.iter().copied());
                crate::p3_comun::shuffle_with(&mut pool, s);
                g.sides[h].hand = pool.drain(..nh).collect();
                g.sides[h].deck = pool;
                crate::p3_comun::shuffle_with(&mut g.sides[me.idx()].deck, s ^ 3);
            },
            6,
        );
    }
}
