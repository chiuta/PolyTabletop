//! HOINARI PRIN HRUBĂ — joc umoristic de cărți cu monștri, niveluri și comori (variantă originală).
//!
//! Trei hoinari: tu (locul 1) și doi roboți (masa, `Player::Two`). Primul care ajunge la
//! **nivelul 10 omorând un monstru** câștigă. Tura: „deschizi ușa” (o carte din pachetul
//! de uși): un monstru te atacă; un blestem se aplică pe loc; altceva intră în mână.
//! Dacă nu a apărut monstru poți **căuta prin hrubă** (tragi o carte de ușă în mână) sau
//! juca un monstru din mână împotriva ta. Luptă: forța ta = nivel + arma + armura + poțiunile
//! de forță folosite; câștigi dacă forța ta **depășește** nivelul monstrului (egal = pierzi).
//! Câștig: 1 nivel (2 la monștrii mari) și comori. Fugă: zar d6, 5+ scapi; altfel pățești
//! „păcatul” monstrului. Roboții pot **îngreuna** lupta ta cu un monstru din mâna lor.
//! La sfârșit de tură ții maxim 5 cărți. Simplificări: fără clase/rase, fără cereri de ajutor,
//! fără negocieri. Limită de 90 de ture ca să se termine (câștigă nivelul cel mai mare).
//! Toate cărțile și glumele sunt originale.

use crate::w5_util::{shuffle_vec, trim_log};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Bad {
    LoseLevel,
    LoseItem,
    LoseHand,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Card {
    Monster(u8),
    CurseLevel,
    CurseItem,
    Weapon(u8),
    Armor(u8),
    Strength, // +3 într-o luptă
    Growth,   // +1 nivel
}

pub struct MonsterDef {
    pub name: &'static str,
    pub level: u8,
    pub bad: Bad,
    pub reward: u8,
    pub treasure: u8,
}

pub const MONSTERS: [MonsterDef; 14] = [
    MonsterDef {
        name: "Găina Radioactivă",
        level: 2,
        bad: Bad::LoseLevel,
        reward: 1,
        treasure: 1,
    },
    MonsterDef {
        name: "Contabilul Fantomă",
        level: 4,
        bad: Bad::LoseItem,
        reward: 1,
        treasure: 1,
    },
    MonsterDef {
        name: "Pisica Birocrată",
        level: 3,
        bad: Bad::LoseLevel,
        reward: 1,
        treasure: 1,
    },
    MonsterDef {
        name: "Cartoful Rău",
        level: 1,
        bad: Bad::LoseLevel,
        reward: 1,
        treasure: 1,
    },
    MonsterDef {
        name: "Mama Zmeului la Plată",
        level: 8,
        bad: Bad::LoseItem,
        reward: 1,
        treasure: 2,
    },
    MonsterDef {
        name: "Cerberul de Apartament",
        level: 6,
        bad: Bad::LoseHand,
        reward: 1,
        treasure: 2,
    },
    MonsterDef {
        name: "Șobolanul cu Diplomă",
        level: 5,
        bad: Bad::LoseLevel,
        reward: 1,
        treasure: 1,
    },
    MonsterDef {
        name: "Hidra de la Ghișeu",
        level: 11,
        bad: Bad::LoseLevel,
        reward: 2,
        treasure: 3,
    },
    MonsterDef {
        name: "Cuptorul Vorbitor",
        level: 7,
        bad: Bad::LoseItem,
        reward: 1,
        treasure: 2,
    },
    MonsterDef {
        name: "Troll de Mansardă",
        level: 9,
        bad: Bad::LoseHand,
        reward: 2,
        treasure: 2,
    },
    MonsterDef {
        name: "Păpușa Contabilă",
        level: 3,
        bad: Bad::LoseItem,
        reward: 1,
        treasure: 1,
    },
    MonsterDef {
        name: "Balaurul Fiscal",
        level: 14,
        bad: Bad::LoseLevel,
        reward: 2,
        treasure: 4,
    },
    MonsterDef {
        name: "Mucegaiul Filozof",
        level: 2,
        bad: Bad::LoseHand,
        reward: 1,
        treasure: 1,
    },
    MonsterDef {
        name: "Cocoșul de Noapte",
        level: 5,
        bad: Bad::LoseItem,
        reward: 1,
        treasure: 1,
    },
];

fn card_name(c: Card) -> String {
    match c {
        Card::Monster(i) => format!(
            "{} (monstru {})",
            MONSTERS[i as usize].name, MONSTERS[i as usize].level
        ),
        Card::CurseLevel => "Blestemul Pantofului Ud (-1 nivel)".into(),
        Card::CurseItem => "Blestemul Cheii Pierdute (pierzi un obiect)".into(),
        Card::Weapon(b) => format!("Armă +{b}"),
        Card::Armor(b) => format!("Armură +{b}"),
        Card::Strength => "Poțiune de forță (+3)".into(),
        Card::Growth => "Poțiune de creștere (+1 nivel)".into(),
    }
}

fn short(c: Card) -> String {
    match c {
        Card::Monster(i) => format!("monstru{i}"),
        Card::CurseLevel => "blestem-nivel".into(),
        Card::CurseItem => "blestem-obiect".into(),
        Card::Weapon(b) => format!("arma{b}"),
        Card::Armor(b) => format!("armura{b}"),
        Card::Strength => "forta".into(),
        Card::Growth => "crestere".into(),
    }
}

fn doors() -> Vec<Card> {
    let mut d = Vec::new();
    for i in 0..14u8 {
        d.push(Card::Monster(i));
        d.push(Card::Monster(i));
    }
    for _ in 0..4 {
        d.push(Card::CurseLevel);
        d.push(Card::CurseItem);
    }
    for _ in 0..4 {
        d.push(Card::Strength);
    }
    d
}

fn treasures() -> Vec<Card> {
    let mut d = Vec::new();
    for b in [1u8, 1, 2, 2, 3, 3, 4, 5] {
        d.push(Card::Weapon(b));
    }
    for b in [1u8, 1, 2, 2, 3, 4] {
        d.push(Card::Armor(b));
    }
    for _ in 0..3 {
        d.push(Card::Growth);
    }
    for _ in 0..6 {
        d.push(Card::Strength);
    }
    d
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Phase {
    Open,
    AfterDoor,
    Fight,
    Resolve,
    Run,
    Charity,
    Robot,
    Over,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Kick,
    Loot,
    Equip(Card),
    PlayMonster(u8),
    UseGrowth,
    UseStrength,
    Fight,
    Flee,
    Drop(Card),
    End,
    Table,
}

#[derive(Clone)]
struct Hero {
    level: i32,
    weapon: u8,
    armor: u8,
    hand: Vec<Card>,
}

#[derive(Clone)]
pub struct Hruba {
    /// Whose private info `render` shows (`None` = the mover's, hot-seat).
    viewer: Option<Player>,
    heroes: [Hero; 3],
    doors: Vec<Card>,
    door_discard: Vec<Card>,
    tres: Vec<Card>,
    tres_discard: Vec<Card>,
    cur: usize,
    turn: u32,
    phase: Phase,
    monsters: Vec<u8>, // monstrii din lupta curentă (omul)
    boost: i32,        // poțiuni folosite în lupta curentă
    fled: bool,
    rng: Rng,
    log: Vec<String>,
    winner: Option<usize>,
    last_door_was_monster: bool,
    /// Doar pentru AI: valoarea poziției înaintea ultimei trageri de carte necunoscută
    /// (ca `evaluate` să nu „vadă” cartea abia trasă) și dacă starea e imediat după ea.
    fresh: bool,
    /// Starea dinaintea ultimei mutări cu rezultat necunoscut (ușă, căutare, luptă) + mutarea.
    pre_snap: Option<Box<(Hruba, Move)>>,
    track: bool,
}

impl Default for Hruba {
    fn default() -> Self {
        Self::new()
    }
}

impl Hruba {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut d = doors();
        let mut t = treasures();
        shuffle_vec(&mut d, &mut rng);
        shuffle_vec(&mut t, &mut rng);
        let mut g = Hruba {
            viewer: None,
            heroes: [
                Hero {
                    level: 1,
                    weapon: 0,
                    armor: 0,
                    hand: vec![],
                },
                Hero {
                    level: 1,
                    weapon: 0,
                    armor: 0,
                    hand: vec![],
                },
                Hero {
                    level: 1,
                    weapon: 0,
                    armor: 0,
                    hand: vec![],
                },
            ],
            doors: d,
            door_discard: vec![],
            tres: t,
            tres_discard: vec![],
            cur: 0,
            turn: 1,
            phase: Phase::Open,
            monsters: vec![],
            boost: 0,
            fled: false,
            rng,
            log: vec![],
            winner: None,
            last_door_was_monster: false,
            fresh: false,
            pre_snap: None,
            track: true,
        };
        for h in 0..3 {
            for _ in 0..2 {
                let c = g.draw_door();
                g.heroes[h].hand.push(c);
                let c = g.draw_tres();
                g.heroes[h].hand.push(c);
            }
        }
        g.say("Ești primul la rând. Deschide ușa!".into());
        g
    }

    pub fn levels(&self) -> [i32; 3] {
        [
            self.heroes[0].level,
            self.heroes[1].level,
            self.heroes[2].level,
        ]
    }
    pub fn phase(&self) -> Phase {
        self.phase
    }
    pub fn total_cards(&self) -> usize {
        let hands: usize = self.heroes.iter().map(|h| h.hand.len()).sum();
        let eq: usize = self
            .heroes
            .iter()
            .map(|h| usize::from(h.weapon > 0) + usize::from(h.armor > 0))
            .sum();
        self.doors.len()
            + self.door_discard.len()
            + self.tres.len()
            + self.tres_discard.len()
            + hands
            + eq
            + self.monsters.len()
    }

    fn say(&mut self, s: String) {
        self.log.push(s);
        trim_log(&mut self.log, 14);
    }

    fn draw_door(&mut self) -> Card {
        if self.doors.is_empty() {
            self.doors = std::mem::take(&mut self.door_discard);
            shuffle_vec(&mut self.doors, &mut self.rng);
        }
        self.doors.pop().unwrap_or(Card::Strength)
    }

    fn draw_tres(&mut self) -> Card {
        if self.tres.is_empty() {
            self.tres = std::mem::take(&mut self.tres_discard);
            shuffle_vec(&mut self.tres, &mut self.rng);
        }
        self.tres.pop().unwrap_or(Card::Strength)
    }

    fn strength(&self, h: usize) -> i32 {
        let x = &self.heroes[h];
        x.level + x.weapon as i32 + x.armor as i32
    }

    fn lose_item(&mut self, h: usize) {
        let x = &mut self.heroes[h];
        if x.weapon >= x.armor && x.weapon > 0 {
            let b = x.weapon;
            x.weapon = 0;
            self.tres_discard.push(Card::Weapon(b));
        } else if x.armor > 0 {
            let b = x.armor;
            x.armor = 0;
            self.tres_discard.push(Card::Armor(b));
        }
    }

    fn apply_bad(&mut self, h: usize, bad: Bad) {
        match bad {
            Bad::LoseLevel => {
                self.heroes[h].level = (self.heroes[h].level - 1).max(1);
            }
            Bad::LoseItem => self.lose_item(h),
            Bad::LoseHand => {
                let k = self.heroes[h].hand.len().min(2);
                for _ in 0..k {
                    let idx = self.rng.below(self.heroes[h].hand.len());
                    let c = self.heroes[h].hand.remove(idx);
                    match c {
                        Card::Weapon(_) | Card::Armor(_) | Card::Growth => {
                            self.tres_discard.push(c)
                        }
                        _ => self.door_discard.push(c),
                    }
                }
            }
        }
    }

    fn discard_monsters(&mut self) {
        for i in std::mem::take(&mut self.monsters) {
            self.door_discard.push(Card::Monster(i));
        }
    }

    /// Rezolvă lupta eroului `h` (un robot sau omul) contra monștrilor curenți.
    fn win_fight(&mut self, h: usize) {
        let mut levels = 0;
        let mut tre = 0;
        for &i in &self.monsters {
            levels += MONSTERS[i as usize].reward as i32;
            tre += MONSTERS[i as usize].treasure;
        }
        self.heroes[h].level += levels;
        for _ in 0..tre {
            let c = self.draw_tres();
            self.heroes[h].hand.push(c);
        }
        self.say(format!(
            "Locul {} învinge: +{} nivel, +{} comori (nivel {}).",
            h + 1,
            levels,
            tre,
            self.heroes[h].level
        ));
        if self.heroes[h].level >= 10 {
            self.heroes[h].level = 10;
            self.winner = Some(h);
            self.phase = Phase::Over;
        }
    }

    fn monster_total(&self) -> i32 {
        self.monsters
            .iter()
            .map(|&i| MONSTERS[i as usize].level as i32)
            .sum()
    }

    fn robot_equip(&mut self, h: usize) {
        loop {
            let x = &self.heroes[h];
            let mut done = true;
            for (k, &c) in x.hand.iter().enumerate() {
                let better = match c {
                    Card::Weapon(b) => b > x.weapon,
                    Card::Armor(b) => b > x.armor,
                    _ => false,
                };
                if better {
                    let c = self.heroes[h].hand.remove(k);
                    match c {
                        Card::Weapon(b) => {
                            let old = std::mem::replace(&mut self.heroes[h].weapon, b);
                            if old > 0 {
                                self.tres_discard.push(Card::Weapon(old));
                            }
                        }
                        Card::Armor(b) => {
                            let old = std::mem::replace(&mut self.heroes[h].armor, b);
                            if old > 0 {
                                self.tres_discard.push(Card::Armor(old));
                            }
                        }
                        _ => {}
                    }
                    done = false;
                    break;
                }
            }
            if done {
                break;
            }
        }
        // poțiune de creștere imediat
        while let Some(k) = self.heroes[h].hand.iter().position(|&c| c == Card::Growth) {
            if self.heroes[h].level >= 9 {
                break;
            }
            self.heroes[h].hand.remove(k);
            self.tres_discard.push(Card::Growth);
            self.heroes[h].level += 1;
        }
    }

    fn apply_curse(&mut self, h: usize, c: Card) {
        match c {
            Card::CurseLevel => self.heroes[h].level = (self.heroes[h].level - 1).max(1),
            _ => self.lose_item(h),
        }
        self.door_discard.push(c);
    }

    fn robot_turn(&mut self) {
        let h = self.cur;
        self.robot_equip(h);
        let c = self.draw_door();
        match c {
            Card::Monster(i) => {
                self.say(format!(
                    "Locul {} deschide ușa: {}.",
                    h + 1,
                    MONSTERS[i as usize].name
                ));
                self.monsters = vec![i];
                self.robot_fight(h);
            }
            Card::CurseLevel | Card::CurseItem => {
                self.say(format!("Locul {} e blestemat.", h + 1));
                self.apply_curse(h, c);
                self.robot_loot_or_monster(h);
            }
            _ => {
                self.heroes[h].hand.push(c);
                self.robot_loot_or_monster(h);
            }
        }
        if self.winner.is_some() {
            return;
        }
        self.robot_equip(h);
        while self.heroes[h].hand.len() > 5 {
            let c = self.heroes[h].hand.remove(0);
            match c {
                Card::Weapon(_) | Card::Armor(_) | Card::Growth => self.tres_discard.push(c),
                _ => self.door_discard.push(c),
            }
        }
        self.end_turn();
    }

    fn robot_loot_or_monster(&mut self, h: usize) {
        // joacă din mână un monstru pe care îl poate bate
        let me = self.strength(h)
            + 3 * self.heroes[h]
                .hand
                .iter()
                .filter(|&&c| c == Card::Strength)
                .count() as i32;
        let pick = self.heroes[h].hand.iter().position(
            |&c| matches!(c, Card::Monster(i) if (MONSTERS[i as usize].level as i32) < me),
        );
        if let Some(k) = pick {
            if let Card::Monster(i) = self.heroes[h].hand.remove(k) {
                self.say(format!(
                    "Locul {} pornește la vânătoare: {}.",
                    h + 1,
                    MONSTERS[i as usize].name
                ));
                self.monsters = vec![i];
                self.robot_fight(h);
                return;
            }
        }
        let c = self.draw_door();
        self.heroes[h].hand.push(c);
    }

    fn robot_fight(&mut self, h: usize) {
        let mut s = self.strength(h);
        let need = self.monster_total();
        let mut pots = self.heroes[h]
            .hand
            .iter()
            .filter(|&&c| c == Card::Strength)
            .count() as i32;
        let mut used = 0;
        while s <= need && pots > 0 {
            s += 3;
            pots -= 1;
            used += 1;
        }
        if s > need {
            for _ in 0..used {
                let k = self.heroes[h]
                    .hand
                    .iter()
                    .position(|&c| c == Card::Strength)
                    .unwrap();
                self.heroes[h].hand.remove(k);
                self.tres_discard.push(Card::Strength);
            }
            self.win_fight(h);
            self.discard_monsters();
        } else {
            // fuge
            let r = self.rng.d6();
            if r >= 5 {
                self.say(format!("Locul {} fuge.", h + 1));
            } else {
                let bad = MONSTERS[self.monsters[0] as usize].bad;
                self.say(format!("Locul {} nu scapă și pățește ceva rău.", h + 1));
                self.apply_bad(h, bad);
            }
            self.discard_monsters();
        }
    }

    fn end_turn(&mut self) {
        if self.winner.is_some() {
            return;
        }
        self.monsters.clear();
        self.boost = 0;
        self.cur = (self.cur + 1) % 3;
        if self.cur == 0 {
            self.turn += 1;
            if self.turn > 90 {
                let l = self.levels();
                let mx = *l.iter().max().unwrap();
                let w: Vec<usize> = (0..3).filter(|&i| l[i] == mx).collect();
                self.winner = Some(if w.len() == 1 { w[0] } else { 3 });
                self.phase = Phase::Over;
                return;
            }
            self.phase = Phase::Open;
        } else {
            self.phase = Phase::Robot;
        }
    }

    fn human_hand_over(&self) -> bool {
        self.heroes[0].hand.len() > 5
    }
}

const VAL_LEVEL: i32 = 100;
const VAL_ITEM: i32 = 55;
const VAL_POTION: i32 = 22;

impl Hruba {
    /// Valoarea statică a locului 1 (nivel, echipament, mână).
    fn base_value(&self) -> i32 {
        let h = &self.heroes[0];
        let pots = h.hand.iter().filter(|&&c| c == Card::Strength).count() as i32;
        let st = self.strength(0) + 3 * pots;
        let mut v = VAL_LEVEL * h.level + VAL_ITEM * (i32::from(h.weapon) + i32::from(h.armor));
        let (mut bw, mut ba) = (h.weapon, h.armor);
        for &c in &h.hand {
            match c {
                Card::Growth => v += if h.level < 9 { 85 } else { 30 },
                Card::Strength => v += VAL_POTION,
                Card::Weapon(b) => bw = bw.max(b),
                Card::Armor(b) => ba = ba.max(b),
                Card::Monster(i) => {
                    v += if i32::from(MONSTERS[i as usize].level) < st {
                        30
                    } else {
                        6
                    }
                }
                _ => {}
            }
        }
        v += 40 * (i32::from(bw) - i32::from(h.weapon)) + 40 * (i32::from(ba) - i32::from(h.armor));
        v
    }

    /// Politica de simulare a locului 1 (omul/AI-ul): echipează, evită lupte cu marjă mică
    /// (roboții pot îngreuna luptele câștigate la limită), folosește poțiuni ca să iasă din zona
    /// riscantă, fuge când nu poate câștiga.
    fn policy_move(&self) -> Move {
        let h = &self.heroes[0];
        let mv = Game::legal_moves(self);
        let pots = h.hand.iter().filter(|&&c| c == Card::Strength).count() as i32;
        match self.phase {
            Phase::Fight => {
                let need = self.monster_total();
                let s = self.strength(0) + self.boost;
                let t = 3;
                if s > need + t {
                    Move::Fight
                } else if pots > 0 && s + 3 * pots > need + t {
                    Move::UseStrength
                } else if s > need {
                    Move::Fight
                } else if pots > 0 && s + 3 * pots > need {
                    Move::UseStrength
                } else {
                    Move::Flee
                }
            }
            Phase::Charity => {
                let st = self.strength(0) + 3 * pots;
                let worth = |c: &Card| -> i32 {
                    match *c {
                        Card::Monster(i) => {
                            if i32::from(MONSTERS[i as usize].level) + 3 < st {
                                30
                            } else {
                                1
                            }
                        }
                        Card::Strength => 20,
                        Card::Growth => 90,
                        Card::Weapon(b) => 10 * i32::from(b),
                        Card::Armor(b) => 10 * i32::from(b),
                        _ => 0,
                    }
                };
                h.hand
                    .iter()
                    .min_by_key(|c| worth(c))
                    .map(|c| Move::Drop(*c))
                    .unwrap_or_else(|| mv[0].clone())
            }
            Phase::Open | Phase::AfterDoor => {
                if let Some(m) = mv
                    .iter()
                    .find(|m| matches!(m, Move::Equip(_) | Move::UseGrowth))
                {
                    return m.clone();
                }
                if self.phase == Phase::AfterDoor {
                    let st = self.strength(0);
                    let best = mv
                        .iter()
                        .filter_map(|m| match m {
                            Move::PlayMonster(i)
                                if i32::from(MONSTERS[*i as usize].level) + 3 < st + 3 * pots =>
                            {
                                Some((MONSTERS[*i as usize].level, m.clone()))
                            }
                            _ => None,
                        })
                        .max_by_key(|x| x.0);
                    if let Some((_, m)) = best {
                        return m;
                    }
                    return Move::Loot;
                }
                Move::Kick
            }
            Phase::Run => Move::Flee,
            _ => mv.first().cloned().unwrap_or(Move::Table),
        }
    }

    /// Lume plauzibilă compatibilă cu ce vede locul 1: pachetele și mâinile roboților se
    /// extrag din cărțile necunoscute (nu se citește ordinea reală, nici mâinile roboților).
    fn determinize(&self, rng: &mut Rng) -> Hruba {
        let mut g = self.clone();
        g.track = false;
        g.pre_snap = None;
        g.fresh = false;
        let mut all = doors();
        all.extend(treasures());
        let mut known: Vec<Card> = self.heroes[0].hand.clone();
        for h in &self.heroes {
            if h.weapon > 0 {
                known.push(Card::Weapon(h.weapon));
            }
            if h.armor > 0 {
                known.push(Card::Armor(h.armor));
            }
        }
        known.extend(self.door_discard.iter().copied());
        known.extend(self.tres_discard.iter().copied());
        known.extend(self.monsters.iter().map(|&i| Card::Monster(i)));
        for c in known {
            if let Some(k) = all.iter().position(|&x| x == c) {
                all.swap_remove(k);
            }
        }
        let (a, b) = (self.doors.len(), self.tres.len());
        let p_door = a as u64 * 1000 / (a + b).max(1) as u64;
        let mut ds: Vec<Card> = Vec::new();
        let mut ts: Vec<Card> = Vec::new();
        for c in all {
            match c {
                Card::Monster(_) | Card::CurseLevel | Card::CurseItem => ds.push(c),
                Card::Strength => {
                    if (rng.below(1000) as u64) < p_door {
                        ds.push(c)
                    } else {
                        ts.push(c)
                    }
                }
                _ => ts.push(c),
            }
        }
        shuffle_vec(&mut ds, rng);
        shuffle_vec(&mut ts, rng);
        let mut doors_new: Vec<Card> = Vec::new();
        let mut tres_new: Vec<Card> = Vec::new();
        while doors_new.len() < a {
            match ds.pop().or_else(|| ts.pop()) {
                Some(c) => doors_new.push(c),
                None => break,
            }
        }
        while tres_new.len() < b {
            match ts.pop().or_else(|| ds.pop()) {
                Some(c) => tres_new.push(c),
                None => break,
            }
        }
        let mut left = ds;
        left.extend(ts);
        shuffle_vec(&mut left, rng);
        for r in 1..3 {
            let n = self.heroes[r].hand.len();
            let mut hand = Vec::new();
            for _ in 0..n {
                if let Some(c) = left.pop() {
                    hand.push(c);
                }
            }
            g.heroes[r].hand = hand;
        }
        doors_new.extend(left);
        g.doors = doors_new;
        g.tres = tres_new;
        g.rng = Rng::new(rng.next_u64());
        g
    }

    /// Joacă până la sfârșit cu politica de mai sus (omul) și roboții jocului; 1 = câștig loc 1.
    fn playout(&mut self) -> f64 {
        let mut n = 0;
        while self.winner.is_none() && n < 6000 {
            n += 1;
            let m = if Game::to_move(self) == Player::One {
                self.policy_move()
            } else {
                Move::Table
            };
            self.apply(&m);
        }
        match self.winner {
            Some(0) => 1.0,
            Some(3) => 0.33,
            _ => 0.0,
        }
    }
}

const ROLLOUTS: usize = 300;

impl Hruba {
    /// Normal rendering (human = J1's point of view).
    fn render_main(&self) -> String {
        let mut s = format!(
            "HOINARI PRIN HRUBĂ — tura {} — la rând: locul {}\n",
            self.turn,
            self.cur + 1
        );
        for i in 0..3 {
            let h = &self.heroes[i];
            s.push_str(&format!(
                "  locul {}{}: nivel {}, armă +{}, armură +{}, forță {}, {} cărți\n",
                i + 1,
                if i == 0 { " (tu)" } else { "" },
                h.level,
                h.weapon,
                h.armor,
                self.strength(i),
                h.hand.len()
            ));
        }
        let names: Vec<String> = self.heroes[0]
            .hand
            .iter()
            .map(|&c| format!("{} [{}]", card_name(c), short(c)))
            .collect();
        s.push_str(&format!("Mâna ta: {}\n", names.join("; ")));
        if !self.monsters.is_empty() && matches!(self.phase, Phase::Fight | Phase::Resolve) {
            let m: Vec<String> = self
                .monsters
                .iter()
                .map(|&i| {
                    format!(
                        "{} ({})",
                        MONSTERS[i as usize].name, MONSTERS[i as usize].level
                    )
                })
                .collect();
            s.push_str(&format!(
                "În luptă: {} — forța ta {} (+{} din poțiuni)\n",
                m.join(" + "),
                self.strength(0),
                self.boost
            ));
        }
        for l in &self.log {
            s.push_str(l);
            s.push('\n');
        }
        if let Some(w) = self.winner {
            s.push_str(match w {
                0 => "Ai câștigat hruba!\n",
                3 => "Remiză la nivel.\n",
                _ => "Un robot a câștigat hruba.\n",
            });
        }
        s
    }

    /// Reveal-all rendering, appended when `set_viewer(Some(Two))` (second human inspects).
    fn god_view(&self) -> String {
        let mut s =
            String::from("--- VEDERE TOTALĂ (J2): toate rolurile și informațiile ascunse ---\n");
        for i in 0..3 {
            let names: Vec<String> = self.heroes[i]
                .hand
                .iter()
                .map(|&c| format!("{} [{}]", card_name(c), short(c)))
                .collect();
            s.push_str(&format!("  locul {}: {}\n", i + 1, names.join("; ")));
        }
        s
    }
}

impl Game for Hruba {
    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }
    type Move = Move;

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Open
            | Phase::AfterDoor
            | Phase::Fight
            | Phase::Run
            | Phase::Charity
            | Phase::Over => Player::One,
            Phase::Resolve | Phase::Robot => Player::Two,
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        let h = &self.heroes[0];
        let mut v = Vec::new();
        let equips = |v: &mut Vec<Move>| {
            let mut seen: Vec<Card> = Vec::new();
            for &c in &h.hand {
                let ok = match c {
                    Card::Weapon(b) => b > h.weapon,
                    Card::Armor(b) => b > h.armor,
                    _ => false,
                };
                if ok && !seen.contains(&c) {
                    seen.push(c);
                    v.push(Move::Equip(c));
                }
            }
            if h.hand.contains(&Card::Growth) && h.level < 9 {
                v.push(Move::UseGrowth);
            }
        };
        match self.phase {
            Phase::Open => {
                v.push(Move::Kick);
                equips(&mut v);
            }
            Phase::AfterDoor => {
                v.push(Move::Loot);
                let mut seen = Vec::new();
                for &c in &h.hand {
                    if let Card::Monster(i) = c {
                        if !seen.contains(&i) {
                            seen.push(i);
                            v.push(Move::PlayMonster(i));
                        }
                    }
                }
                equips(&mut v);
            }
            Phase::Fight => {
                v.push(Move::Fight);
                v.push(Move::Flee);
                if h.hand.contains(&Card::Strength) {
                    v.push(Move::UseStrength);
                }
            }
            Phase::Charity => {
                let mut seen = Vec::new();
                for &c in &h.hand {
                    if !seen.contains(&c) {
                        seen.push(c);
                        v.push(Move::Drop(c));
                    }
                }
            }
            Phase::Run => v.push(Move::Flee),
            Phase::Resolve | Phase::Robot => v.push(Move::Table),
            Phase::Over => {}
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        self.fresh = false;
        self.pre_snap = None;
        if self.track && matches!(m, Move::Kick | Move::Loot | Move::Fight) {
            let mut snap = self.clone();
            snap.track = false;
            self.pre_snap = Some(Box::new((snap, m.clone())));
            self.fresh = true;
        }
        match m {
            Move::Kick => {
                let c = self.draw_door();
                match c {
                    Card::Monster(i) => {
                        self.say(format!("Ușa se deschide: {}!", MONSTERS[i as usize].name));
                        self.monsters = vec![i];
                        self.last_door_was_monster = true;
                        self.phase = Phase::Fight;
                    }
                    Card::CurseLevel | Card::CurseItem => {
                        self.say(format!("Ușa te blestemă: {}.", card_name(c)));
                        self.apply_curse(0, c);
                        self.last_door_was_monster = false;
                        self.phase = Phase::AfterDoor;
                    }
                    _ => {
                        self.say(format!("Ușa îți dă: {}.", card_name(c)));
                        self.heroes[0].hand.push(c);
                        self.last_door_was_monster = false;
                        self.phase = Phase::AfterDoor;
                    }
                }
            }
            Move::Loot => {
                let c = self.draw_door();
                self.heroes[0].hand.push(c);
                self.say("Cauți prin hrubă: tragi o carte.".into());
                self.phase = if self.human_hand_over() {
                    Phase::Charity
                } else {
                    Phase::Robot
                };
                if self.phase == Phase::Robot {
                    self.end_turn();
                }
            }
            Move::PlayMonster(i) => {
                let k = self.heroes[0]
                    .hand
                    .iter()
                    .position(|&c| c == Card::Monster(*i))
                    .unwrap();
                self.heroes[0].hand.remove(k);
                self.monsters = vec![*i];
                self.say(format!(
                    "Pornești la vânătoare: {}.",
                    MONSTERS[*i as usize].name
                ));
                self.phase = Phase::Fight;
            }
            Move::Equip(c) => {
                let k = self.heroes[0].hand.iter().position(|x| x == c).unwrap();
                self.heroes[0].hand.remove(k);
                match c {
                    Card::Weapon(b) => {
                        let old = std::mem::replace(&mut self.heroes[0].weapon, *b);
                        if old > 0 {
                            self.tres_discard.push(Card::Weapon(old));
                        }
                    }
                    Card::Armor(b) => {
                        let old = std::mem::replace(&mut self.heroes[0].armor, *b);
                        if old > 0 {
                            self.tres_discard.push(Card::Armor(old));
                        }
                    }
                    _ => {}
                }
            }
            Move::UseGrowth => {
                let k = self.heroes[0]
                    .hand
                    .iter()
                    .position(|&c| c == Card::Growth)
                    .unwrap();
                self.heroes[0].hand.remove(k);
                self.tres_discard.push(Card::Growth);
                self.heroes[0].level += 1;
            }
            Move::UseStrength => {
                let k = self.heroes[0]
                    .hand
                    .iter()
                    .position(|&c| c == Card::Strength)
                    .unwrap();
                self.heroes[0].hand.remove(k);
                self.tres_discard.push(Card::Strength);
                self.boost += 3;
            }
            Move::Fight => {
                // roboții pot îngreuna lupta cu un monstru din mâna lor (văd doar forța ta publică)
                let mine = self.strength(0) + self.boost;
                for r in 1..3 {
                    let margin = mine - self.monster_total();
                    if margin > 0 && margin <= 3 && self.rng.below(3) == 0 {
                        let pick = self.heroes[r]
                            .hand
                            .iter()
                            .position(|&c| matches!(c, Card::Monster(i) if MONSTERS[i as usize].level as i32 >= margin));
                        if let Some(k) = pick {
                            if let Card::Monster(i) = self.heroes[r].hand.remove(k) {
                                self.say(format!(
                                    "Locul {} îți aruncă în luptă: {}!",
                                    r + 1,
                                    MONSTERS[i as usize].name
                                ));
                                self.monsters.push(i);
                            }
                        }
                    }
                }
                self.phase = Phase::Resolve;
                self.fled = false;
            }
            Move::Flee => {
                self.fled = true;
                self.phase = Phase::Resolve;
            }
            Move::Drop(c) => {
                let k = self.heroes[0].hand.iter().position(|x| x == c).unwrap();
                self.heroes[0].hand.remove(k);
                match c {
                    Card::Weapon(_) | Card::Armor(_) | Card::Growth => self.tres_discard.push(*c),
                    _ => self.door_discard.push(*c),
                }
                if !self.human_hand_over() {
                    self.end_turn();
                }
            }
            Move::End => {}
            Move::Table => match self.phase {
                Phase::Resolve => {
                    if self.fled {
                        let r = self.rng.d6();
                        if r >= 5 {
                            self.say(format!("Fugi (zar {r})."));
                        } else {
                            let bad = MONSTERS[self.monsters[0] as usize].bad;
                            self.say(format!(
                                "Nu scapi (zar {r}): {}.",
                                match bad {
                                    Bad::LoseLevel => "pierzi un nivel",
                                    Bad::LoseItem => "pierzi un obiect",
                                    Bad::LoseHand => "pierzi două cărți din mână",
                                }
                            ));
                            self.apply_bad(0, bad);
                        }
                        self.discard_monsters();
                    } else {
                        let s = self.strength(0) + self.boost;
                        let need = self.monster_total();
                        if s > need {
                            self.say(format!("Forța ta {s} > {need}: câștigi lupta!"));
                            self.win_fight(0);
                            self.discard_monsters();
                        } else {
                            self.say(format!("Forța ta {s} nu depășește {need}: fugi, dar..."));
                            let r = self.rng.d6();
                            if r >= 5 {
                                self.say(format!("Scapi (zar {r})."));
                            } else {
                                let bad = MONSTERS[self.monsters[0] as usize].bad;
                                self.say(format!("Nu scapi (zar {r}): pățești ceva rău."));
                                self.apply_bad(0, bad);
                            }
                            self.discard_monsters();
                        }
                    }
                    self.boost = 0;
                    if self.winner.is_none() {
                        if self.human_hand_over() {
                            self.phase = Phase::Charity;
                        } else {
                            self.end_turn();
                        }
                    }
                }
                Phase::Robot => self.robot_turn(),
                _ => {}
            },
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        match self.winner? {
            0 => Some(Outcome::Win(Player::One)),
            3 => Some(Outcome::Draw),
            _ => Some(Outcome::Win(Player::Two)),
        }
    }

    fn render(&self) -> String {
        let mut s = self.render_main();
        if self.viewer == Some(Player::Two) {
            s.push_str(&self.god_view());
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Kick => "deschide".into(),
            Move::Loot => "cauta".into(),
            Move::Equip(c) => format!("echipeaza {}", short(*c)),
            Move::PlayMonster(i) => format!("vaneaza monstru{i}"),
            Move::UseGrowth => "crestere".into(),
            Move::UseStrength => "forta".into(),
            Move::Fight => "lupta".into(),
            Move::Flee => "fugi".into(),
            Move::Drop(c) => format!("scapa {}", short(*c)),
            Move::End => "incheie".into(),
            Move::Table => "masa".into(),
        }
    }

    /// Doar informația vizibilă locului 1: nivelul, echipamentul și mâna lui, monstrul de pe
    /// masă (primul din listă) și rezultatul scontat al luptei. Cartea abia trasă dintr-un
    /// pachet (ușă / hrubă) nu se citește: se folosește valoarea dinaintea trasului.
    /// Evaluare cinstită prin determinizare: pornește de la ce vede locul 1 (fără ordinea
    /// pachetelor, fără mâinile roboților, fără cartea abia trasă / urmările zarurilor și
    /// ale îngreunărilor, care se reiau în fiecare simulare) și joacă partide întregi.
    fn evaluate(&self, p: Player) -> i32 {
        let v = if let Some(w) = self.winner {
            match w {
                0 => 1000,
                3 => 300,
                _ => 0,
            }
        } else {
            let (base, mv) = match &self.pre_snap {
                Some(b) if self.fresh => (b.0.clone(), Some(b.1.clone())),
                _ => (self.clone(), None),
            };
            let mut seed: u64 = 0x517C_C1B7 ^ (base.turn as u64) << 8 ^ (base.cur as u64) << 4;
            for h in &base.heroes {
                seed = seed.rotate_left(9) ^ (h.level as u64) << 3 ^ h.hand.len() as u64;
            }
            seed ^= (base.doors.len() as u64) << 24 ^ (base.tres.len() as u64) << 40;
            let mut rng = Rng::new(seed);
            let n = ROLLOUTS;
            let mut total = 0.0;
            for _ in 0..n {
                let mut g = base.determinize(&mut rng);
                if let Some(m) = &mv {
                    g.apply(m);
                }
                total += g.playout();
            }
            (1000.0 * total / n as f64) as i32 + base.base_value() / 40
        };
        if p == Player::One {
            v
        } else {
            -v
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(seed: u64) -> Hruba {
        let mut g = Hruba::with_seed(seed);
        let total = g.total_cards();
        let mut rng = Rng::new(seed ^ 3);
        let mut n = 0;
        while g.outcome().is_none() {
            let mv = g.legal_moves();
            assert!(!mv.is_empty(), "{}", g.render());
            g.apply(&mv[rng.below(mv.len())]);
            assert_eq!(g.total_cards(), total, "{}", g.render());
            n += 1;
            assert!(n < 5000);
        }
        g
    }

    #[test]
    fn decks_have_expected_sizes() {
        assert_eq!(doors().len(), 28 + 8 + 4);
        assert_eq!(treasures().len(), 8 + 6 + 3 + 6);
    }

    #[test]
    fn random_games_end_and_conserve_cards() {
        let mut ends = [0; 3];
        for seed in 1..120u64 {
            let g = run(seed);
            match g.outcome() {
                Some(Outcome::Win(Player::One)) => ends[0] += 1,
                Some(Outcome::Win(Player::Two)) => ends[1] += 1,
                _ => ends[2] += 1,
            }
        }
        assert!(ends[1] > 0, "{ends:?}");
    }

    #[test]
    fn tie_does_not_win_the_fight() {
        let mut g = Hruba::with_seed(2);
        g.heroes[0].level = 4;
        g.monsters = vec![1]; // nivel 4
        g.phase = Phase::Resolve;
        g.fled = false;
        let before = g.heroes[0].level;
        g.apply(&Move::Table);
        assert!(g.heroes[0].level <= before);
    }

    #[test]
    fn level_ten_needs_a_kill() {
        let mut g = Hruba::with_seed(5);
        g.heroes[0].level = 9;
        g.heroes[0].hand.push(Card::Growth);
        assert!(!g.legal_moves().contains(&Move::UseGrowth));
        g.apply(&Move::UseGrowth);
        assert!(g.heroes[0].level <= 9 || g.winner.is_none());
        assert!(g.winner.is_none());
    }

    #[test]
    fn sensible_human_can_win() {
        let mut wins = 0;
        for seed in 1..60u64 {
            let mut g = Hruba::with_seed(seed);
            let mut n = 0;
            while g.outcome().is_none() && n < 3000 {
                n += 1;
                let mv = g.legal_moves();
                let pick = match g.phase() {
                    Phase::Fight => {
                        let need = g.monster_total();
                        let s = g.strength(0) + g.boost;
                        if s > need {
                            Move::Fight
                        } else if mv.contains(&Move::UseStrength) {
                            Move::UseStrength
                        } else {
                            Move::Flee
                        }
                    }
                    _ => mv
                        .iter()
                        .find(|m| matches!(m, Move::Equip(_) | Move::UseGrowth))
                        .or_else(|| {
                            mv.iter().find(|m| {
                                matches!(m, Move::PlayMonster(i)
                                    if (MONSTERS[*i as usize].level as i32) < g.strength(0))
                            })
                        })
                        .or_else(|| mv.iter().find(|m| matches!(m, Move::Kick | Move::Loot)))
                        .unwrap_or(&mv[0])
                        .clone(),
                };
                g.apply(&pick);
            }
            if g.outcome() == Some(Outcome::Win(Player::One)) {
                wins += 1;
            }
        }
        assert!(wins > 5, "{wins}");
    }

    #[test]
    fn evaluate_ignores_hidden_hands_and_decks() {
        let mut a = Hruba::with_seed(33);
        for _ in 0..6 {
            a.apply(&Move::Kick);
            while a.phase() != Phase::Open && a.outcome().is_none() {
                let mv = a.legal_moves();
                let m = mv
                    .iter()
                    .find(|m| matches!(m, Move::Flee | Move::Loot | Move::Table))
                    .unwrap_or(&mv[0])
                    .clone();
                a.apply(&m);
            }
        }
        let mut b = a.clone();
        // schimbă ce nu se vede: ordinea pachetelor și mâinile roboților
        b.doors.reverse();
        b.tres.reverse();
        b.heroes[1].hand.reverse();
        let (h1, h2) = (b.heroes[1].hand.clone(), b.heroes[2].hand.clone());
        b.heroes[1].hand = h2.clone();
        b.heroes[2].hand = h1.clone();
        if h1.len() == h2.len() {
            assert_eq!(a.evaluate(Player::One), b.evaluate(Player::One));
        }
    }

    #[test]
    fn evaluate_does_not_read_the_card_just_drawn() {
        let mut a = Hruba::with_seed(21);
        let mut b = a.clone();
        a.doors.push(Card::Monster(0));
        b.doors.push(Card::Strength);
        a.apply(&Move::Kick);
        b.apply(&Move::Kick);
        assert_eq!(a.evaluate(Player::One), b.evaluate(Player::One));
    }
}
