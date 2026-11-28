//! Motor „joc de cărți colecționabile” (lista T4, 2 jucători): pachete fixe construite de joc,
//! mână ascunsă (HOTSEAT_SAFE, `set_viewer`), creaturi și vrăji originale. Șapte familii de reguli
//! (`Mode`), fiecare cu propria resursă și propriul mod de a câștiga:
//! * `Pirate` – aur crescător, zonă de viață (cărți care ajung în mână), căpitan care atacă;
//! * `Evo` – măsură comună de memorie, evoluție peste o creatură, stivă de pază;
//! * `Charge` – energie prin încărcarea cărților din mână, viață numerică, căpitan care se trezește;
//! * `Clock` – scenă de 3, climax, ceas de daune (cărți din pachet), niveluri, anulări;
//! * `Ride` – căpitan cu trepte (urcare), gardă din mână, verificare de pachet;
//! * `Arena` – acțiuni alternante, arene (sol/aer), resurse din mână, inițiativă, baze;
//! * `Mana` – zonă de mană (o carte pe tură), culori, scuturi ascunse cu declanșatoare.
//!
//! Pachetul nu are ordine fixă: extragerile sunt aleatoare la clonare (generator re-însămânțat),
//! deci AI-ul nu vede cărțile viitoare; evaluarea citește doar numărul cărților adversarului.
#![allow(clippy::needless_range_loop, clippy::too_many_arguments)]

use crate::brand_util::*;
use crate::w7_util::Dice;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

pub const BLOCK: u8 = 1;
pub const HASTE: u8 = 2;
pub const DOUBLE: u8 = 4;
pub const TRIG: u8 = 8;
pub const CANCEL: u8 = 16;
pub const AIR: u8 = 32;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    Pirate,
    Evo,
    Charge,
    Clock,
    Ride,
    Arena,
    Mana,
}

#[derive(Clone, Copy)]
pub struct Card {
    pub name: &'static str,
    pub cost: u8,
    pub pow: u8,
    pub hp: u8,
    /// 0 creatură, 1 vrajă, 2 climax
    pub kind: u8,
    pub fx: u8,
    pub civ: u8,
    pub lvl: u8,
    pub fl: u8,
}

pub struct Spec {
    pub id: &'static str,
    pub mode: Mode,
    pub cards: &'static [Card],
    pub decks: [&'static [u8]; 2],
    pub hand: u8,
    pub life: u8,
    pub cap: u8,
    pub lead_pow: u8,
    pub turns: u16,
    pub start_vg: u8,
}

#[derive(Clone)]
struct Cr {
    c: u8,
    dmg: i16,
    rest: bool,
    fresh: bool,
    buf: i16,
}

#[derive(Clone)]
struct Pend {
    pow: i16,
    gsum: i16,
    crit: bool,
}

#[derive(Clone)]
pub struct Tcg {
    sp: &'static Spec,
    side: u8,
    hand: [Vec<u8>; 2],
    deck: [Vec<u8>; 2],
    zone: [Vec<(u8, bool)>; 2],
    life: [Vec<u8>; 2],
    board: [Vec<Cr>; 2],
    lead_rest: [bool; 2],
    hp: [i16; 2],
    clock: [Vec<u8>; 2],
    lvl: [u8; 2],
    vg: [u8; 2],
    res: [i16; 2],
    resmax: [i16; 2],
    mem: i16,
    acted: bool,
    climax: bool,
    phase: u8,
    pend: Option<Pend>,
    passes: u8,
    first: u8,
    turn: u16,
    dice: Dice,
    over: Option<Outcome>,
    viewer: Option<Player>,
    last: String,
    /// Valoarea unei creaturi pierdute în pasul curent din cauza unei cărți ascunse a adversarului
    /// (pază Evo): evaluarea o numără în continuare, ca AI-ul să nu citească cartea ascunsă.
    ghost: [i32; 2],
}

fn win(p: u8) -> Outcome {
    Outcome::Win(if p == 0 { Player::One } else { Player::Two })
}

impl Tcg {
    pub fn new(sp: &'static Spec) -> Self {
        Self::with_seed(sp, seed_now())
    }

    pub fn with_seed(sp: &'static Spec, seed: u64) -> Self {
        let mut g = Tcg {
            sp,
            side: 0,
            hand: [vec![], vec![]],
            deck: [sp.decks[0].to_vec(), sp.decks[1].to_vec()],
            zone: [vec![], vec![]],
            life: [vec![], vec![]],
            board: [vec![], vec![]],
            lead_rest: [false; 2],
            hp: [0; 2],
            clock: [vec![], vec![]],
            lvl: [0; 2],
            vg: [sp.start_vg; 2],
            res: [0; 2],
            resmax: [0; 2],
            mem: 3,
            acted: false,
            climax: false,
            phase: 0,
            pend: None,
            passes: 0,
            first: 0,
            turn: 1,
            dice: Dice::new(seed),
            over: None,
            viewer: None,
            last: String::new(),
            ghost: [0; 2],
        };
        for p in 0..2 {
            match sp.mode {
                Mode::Pirate | Mode::Evo | Mode::Mana => {
                    for _ in 0..sp.life {
                        let c = g.pull(p);
                        g.life[p].push(c);
                    }
                }
                Mode::Charge | Mode::Arena => g.hp[p] = sp.life as i16,
                _ => {}
            }
            for _ in 0..sp.hand {
                let c = g.pull(p);
                g.hand[p].push(c);
            }
        }
        if sp.mode == Mode::Arena {
            g.phase = 1;
        } else {
            g.begin(0);
        }
        g
    }

    /// Pentru teste (audit AI): amestecă ce nu vede jucătorul `1 - hide` (mâna, pachetul și cărțile de viață
    /// ale lui `hide`) și resetează zarurile viitoarelor extrageri.
    #[cfg(test)]
    pub fn scramble(&mut self, hide: usize, seed: u64) {
        let (h, l) = (self.hand[hide].len(), self.life[hide].len());
        let mut pool: Vec<u8> = self.hand[hide].clone();
        pool.extend(self.life[hide].iter().copied());
        pool.extend(self.deck[hide].iter().copied());
        crate::p3_comun::shuffle_with(&mut pool, seed);
        self.hand[hide] = pool.drain(..h).collect();
        self.life[hide] = pool.drain(..l).collect();
        self.deck[hide] = pool;
        crate::p3_comun::shuffle_with(&mut self.deck[1 - hide], seed ^ 0x31);
        self.dice = Dice::new(seed ^ 0x9E37);
    }

    fn card(&self, c: u8) -> &'static Card {
        &self.sp.cards[c as usize]
    }

    fn pull(&mut self, p: usize) -> u8 {
        let n = self.deck[p].len();
        let i = self.dice.below(n);
        self.deck[p].swap_remove(i)
    }

    fn draw(&mut self, p: usize, n: usize) {
        for _ in 0..n {
            if self.deck[p].is_empty() {
                if self.over.is_none() {
                    self.over = Some(win(1 - p as u8));
                }
                return;
            }
            let c = self.pull(p);
            self.hand[p].push(c);
        }
    }

    fn pow_mode(&self) -> bool {
        matches!(self.sp.mode, Mode::Clock | Mode::Ride | Mode::Mana)
    }

    fn begin(&mut self, p: usize) {
        for c in self.board[p].iter_mut() {
            c.rest = false;
            c.fresh = false;
        }
        self.lead_rest[p] = false;
        self.acted = false;
        self.climax = false;
        let first_turn = self.turn == 1 && p == 0;
        match self.sp.mode {
            Mode::Pirate => {
                self.resmax[p] = if first_turn {
                    1
                } else {
                    (self.resmax[p] + 2).min(10)
                };
                self.res[p] = self.resmax[p];
            }
            Mode::Charge => {
                self.res[p] = self.zone[p].len() as i16;
            }
            Mode::Clock => {
                self.res[p] = (self.res[p] + 3).min(8);
            }
            Mode::Mana => {
                for z in self.zone[p].iter_mut() {
                    z.1 = false;
                }
            }
            _ => {}
        }
        if !first_turn {
            self.draw(p, 1);
        }
    }

    fn end_turn(&mut self) {
        let me = self.side as usize;
        self.phase = 0;
        if self.sp.mode == Mode::Evo {
            self.mem = if self.mem < 0 { (-self.mem).min(10) } else { 3 };
        }
        self.side = 1 - self.side;
        if me == 1 {
            self.turn += 1;
        }
        if self.turn > self.sp.turns && self.over.is_none() {
            self.finish_by_standing();
            return;
        }
        if self.over.is_none() {
            let p = self.side as usize;
            self.begin(p);
        }
    }

    fn standing(&self, p: usize) -> i32 {
        match self.sp.mode {
            Mode::Pirate | Mode::Evo | Mode::Mana => self.life[p].len() as i32 * 10,
            Mode::Charge | Mode::Arena => self.hp[p] as i32,
            Mode::Clock => -(self.lvl[p] as i32 * 10 + self.clock[p].len() as i32),
            Mode::Ride => -(self.hp[p] as i32),
        }
    }

    fn finish_by_standing(&mut self) {
        let (a, b) = (self.standing(0), self.standing(1));
        let (ba, bb) = (self.board_value(0), self.board_value(1));
        self.over = Some(match (a * 100 + ba).cmp(&(b * 100 + bb)) {
            std::cmp::Ordering::Greater => win(0),
            std::cmp::Ordering::Less => win(1),
            std::cmp::Ordering::Equal => Outcome::Draw,
        });
    }

    fn lead_pow(&self, p: usize) -> i16 {
        let mut x = self.sp.lead_pow as i16;
        if self.sp.mode == Mode::Charge && self.hp[p] * 2 <= self.sp.life as i16 {
            x += 2;
        }
        if self.sp.mode == Mode::Ride {
            x = self.card(self.vg[p]).pow as i16;
        }
        x
    }

    fn eff_pow(&self, p: usize, i: usize) -> i16 {
        let c = &self.board[p][i];
        let mut x = self.card(c.c).pow as i16 + c.buf;
        if self.climax && p == self.side as usize {
            x += 2;
        }
        x
    }

    fn creature_value(&self, p: usize, i: usize) -> i32 {
        let c = &self.board[p][i];
        let k = self.card(c.c);
        (self.eff_pow(p, i) as i32 + (k.hp as i32 - c.dmg as i32).max(0)) * 3
            + if k.fl & BLOCK != 0 { 2 } else { 0 }
    }

    fn board_value(&self, p: usize) -> i32 {
        (0..self.board[p].len())
            .map(|i| self.creature_value(p, i))
            .sum()
    }

    fn reap(&mut self) {
        for p in 0..2 {
            let sp = self.sp;
            self.board[p].retain(|c| c.dmg < sp.cards[c.c as usize].hp as i16);
        }
    }

    fn blockers(&self, p: usize) -> Vec<usize> {
        (0..self.board[p].len())
            .filter(|&i| self.card(self.board[p][i].c).fl & BLOCK != 0 && !self.board[p][i].rest)
            .collect()
    }

    fn can_pay(&self, p: usize, c: &Card) -> bool {
        match self.sp.mode {
            Mode::Pirate | Mode::Charge | Mode::Clock | Mode::Arena => self.res[p] >= c.cost as i16,
            Mode::Evo => self.mem - c.cost as i16 >= -10,
            Mode::Ride => true,
            Mode::Mana => {
                let free: Vec<u8> = self.zone[p]
                    .iter()
                    .filter(|z| !z.1)
                    .map(|z| self.card(z.0).civ)
                    .collect();
                free.len() >= c.cost as usize
                    && (c.civ == 0 || c.cost == 0 || free.contains(&c.civ))
            }
        }
    }

    fn pay(&mut self, p: usize, cost: u8, civ: u8) {
        match self.sp.mode {
            Mode::Pirate | Mode::Charge | Mode::Clock | Mode::Arena => self.res[p] -= cost as i16,
            Mode::Evo => self.mem -= cost as i16,
            Mode::Mana => {
                let mut left = cost as usize;
                let sp = self.sp;
                if civ != 0 && left > 0 {
                    if let Some(z) = self.zone[p]
                        .iter_mut()
                        .find(|z| !z.1 && sp.cards[z.0 as usize].civ == civ)
                    {
                        z.1 = true;
                        left -= 1;
                    }
                }
                for z in self.zone[p].iter_mut() {
                    if left == 0 {
                        break;
                    }
                    if !z.1 {
                        z.1 = true;
                        left -= 1;
                    }
                }
            }
            Mode::Ride => {}
        }
    }

    fn weakest(&self, p: usize) -> Option<usize> {
        (0..self.board[p].len()).min_by_key(|&i| {
            let c = &self.board[p][i];
            self.card(c.c).hp as i16 - c.dmg
        })
    }

    fn strongest(&self, p: usize) -> Option<usize> {
        (0..self.board[p].len())
            .max_by_key(|&i| self.card(self.board[p][i].c).pow as i16 + self.board[p][i].buf)
    }

    /// Daune adversarului (vrăji de ardere / declanșatoare).
    fn burn(&mut self, victim: usize, n: i16, by: u8) {
        match self.sp.mode {
            Mode::Pirate | Mode::Evo | Mode::Mana => {
                for _ in 0..n {
                    if self.life[victim].pop().is_none() {
                        if self.over.is_none() {
                            self.over = Some(win(by));
                        }
                        return;
                    }
                }
            }
            Mode::Charge | Mode::Arena => {
                self.hp[victim] -= n;
                if self.hp[victim] <= 0 && self.over.is_none() {
                    self.over = Some(win(by));
                }
            }
            Mode::Clock => self.deal_clock(victim, n as usize, by),
            Mode::Ride => self.deal_damage(victim, n as usize, by),
        }
    }

    fn deal_clock(&mut self, victim: usize, n: usize, by: u8) {
        let mut rev = Vec::new();
        let mut cancel = false;
        for _ in 0..n {
            if self.deck[victim].is_empty() {
                if self.over.is_none() {
                    self.over = Some(win(by));
                }
                return;
            }
            let c = self.pull(victim);
            if self.card(c).fl & CANCEL != 0 {
                cancel = true;
            }
            rev.push(c);
        }
        if cancel {
            self.last = format!("{} carte(a) anulează daunele", rev.len());
            return;
        }
        self.clock[victim].extend(rev);
        if self.clock[victim].len() >= 7 {
            self.clock[victim].clear();
            self.lvl[victim] += 1;
            if self.lvl[victim] >= 3 && self.over.is_none() {
                self.over = Some(win(by));
            }
        }
    }

    fn deal_damage(&mut self, victim: usize, n: usize, by: u8) {
        for _ in 0..n {
            if self.deck[victim].is_empty() {
                if self.over.is_none() {
                    self.over = Some(win(by));
                }
                return;
            }
            let _ = self.pull(victim);
            self.hp[victim] += 1;
        }
        if self.hp[victim] >= 6 && self.over.is_none() {
            self.over = Some(win(by));
        }
    }

    fn heal(&mut self, p: usize, n: i16) {
        match self.sp.mode {
            Mode::Pirate | Mode::Evo | Mode::Mana => {
                for _ in 0..n {
                    if !self.deck[p].is_empty() {
                        let c = self.pull(p);
                        self.life[p].push(c);
                    }
                }
            }
            Mode::Charge | Mode::Arena => self.hp[p] = (self.hp[p] + n).min(self.sp.life as i16),
            Mode::Clock => {
                for _ in 0..n {
                    self.clock[p].pop();
                }
            }
            Mode::Ride => self.hp[p] = (self.hp[p] - n).max(0),
        }
    }

    /// Efect de vrajă; `t` = creatura-țintă (a adversarului la 1, a ta la 3).
    fn fx(&mut self, p: usize, fx: u8, mag: i16, t: Option<usize>) {
        let op = 1 - p;
        match fx {
            1 => {
                if let Some(i) = t {
                    if i < self.board[op].len() {
                        self.board[op][i].dmg += mag;
                    }
                }
            }
            2 => self.draw(p, mag as usize),
            3 => {
                if let Some(i) = t {
                    if i < self.board[p].len() {
                        self.board[p][i].buf += mag;
                    }
                }
            }
            4 => self.burn(op, mag, p as u8),
            5 => self.heal(p, mag),
            6 => {
                if let Some(i) = self.strongest(op) {
                    self.board[op].remove(i);
                }
            }
            7 => self.draw(p, 1),
            8 => {
                if let Some(i) = self.weakest(op) {
                    self.board[op][i].dmg += 1;
                }
            }
            _ => {}
        }
        self.reap();
    }

    fn enter(&mut self, p: usize, c: u8, fresh: bool) {
        let k = self.card(c);
        let fresh = fresh && k.fl & HASTE == 0 && self.sp.mode != Mode::Ride;
        self.board[p].push(Cr {
            c,
            dmg: 0,
            rest: false,
            fresh,
            buf: 0,
        });
        if k.fx == 7 || k.fx == 8 {
            self.fx(p, k.fx, 1, None);
        }
    }

    fn play(&mut self, i: usize, t: Option<usize>) {
        let p = self.side as usize;
        let c = self.hand[p].remove(i);
        let k = self.card(c);
        self.pay(p, k.cost, k.civ);
        match k.kind {
            0 => self.enter(p, c, true),
            1 => self.fx(p, k.fx, k.pow as i16, t),
            _ => self.climax = true,
        }
        self.last = format!("joacă {}", k.name);
    }

    /// Un scut / o carte de viață e spartă: la mană, cartea ajunge în mână, iar declanșatorul se joacă gratis.
    fn break_shield(&mut self, victim: usize, by: u8) -> bool {
        match self.life[victim].pop() {
            None => {
                if self.over.is_none() {
                    self.over = Some(win(by));
                }
                false
            }
            Some(c) => {
                let k = self.card(c);
                if self.sp.mode == Mode::Mana && k.fl & TRIG != 0 {
                    // se joacă gratis pentru apărător
                    match k.kind {
                        0 => self.enter(victim, c, false),
                        _ => {
                            let t = if k.fx == 1 {
                                self.strongest(1 - victim)
                            } else if k.fx == 3 {
                                self.strongest(victim)
                            } else {
                                None
                            };
                            self.fx(victim, k.fx, k.pow as i16, t);
                        }
                    }
                    self.last = format!("declanșator: {}", k.name);
                } else {
                    self.hand[victim].push(c);
                }
                true
            }
        }
    }

    fn hit_player(&mut self, a: Option<usize>, ap: i16) {
        let me = self.side as usize;
        let op = 1 - me;
        let dbl = a
            .map(|i| self.card(self.board[me][i].c).fl & DOUBLE != 0)
            .unwrap_or(false);
        let n = 1 + dbl as usize;
        match self.sp.mode {
            Mode::Pirate => {
                if ap >= self.lead_pow(op) {
                    for _ in 0..n {
                        if self.life[op].is_empty() {
                            self.over = Some(win(me as u8));
                            return;
                        }
                        let c = self.life[op].pop().unwrap();
                        self.hand[op].push(c);
                    }
                }
            }
            Mode::Evo => {
                for _ in 0..n {
                    match self.life[op].pop() {
                        None => {
                            self.over = Some(win(me as u8));
                            return;
                        }
                        Some(c) => {
                            let k = self.card(c);
                            if k.kind == 0 && k.pow as i16 > ap {
                                if let Some(i) = a {
                                    if i < self.board[me].len() {
                                        let lost = self.creature_value(me, i);
                                        self.ghost[me] += lost;
                                        self.board[me].remove(i);
                                    }
                                }
                                return;
                            }
                        }
                    }
                }
            }
            Mode::Charge => {
                self.hp[op] -= n as i16;
                if self.hp[op] <= 0 {
                    self.over = Some(win(me as u8));
                }
            }
            Mode::Clock => {
                let soul = n + self.climax as usize;
                self.deal_clock(op, soul, me as u8);
                self.res[me] = (self.res[me] + 1).min(8);
            }
            Mode::Arena => {
                self.hp[op] -= ap;
                if self.hp[op] <= 0 {
                    self.over = Some(win(me as u8));
                }
            }
            Mode::Mana => {
                for _ in 0..n {
                    if !self.break_shield(op, me as u8) {
                        return;
                    }
                }
            }
            Mode::Ride => {}
        }
    }

    fn attack(&mut self, a: Option<usize>, t: Option<usize>) {
        let me = self.side as usize;
        let op = 1 - me;
        let ap = match a {
            Some(i) => self.eff_pow(me, i),
            None => self.lead_pow(me),
        };
        match a {
            Some(i) => self.board[me][i].rest = true,
            None => self.lead_rest[me] = true,
        }
        if self.sp.mode == Mode::Ride {
            self.attack_ride(a, t, ap);
            return;
        }
        match t {
            None => self.hit_player(a, ap),
            Some(j) => {
                let tp = self.eff_pow(op, j);
                if self.pow_mode() {
                    if ap > tp {
                        self.board[op].remove(j);
                    } else if ap < tp {
                        if let Some(i) = a {
                            self.board[me].remove(i);
                        }
                    } else {
                        self.board[op].remove(j);
                        if let Some(i) = a {
                            self.board[me].remove(i);
                        }
                    }
                } else {
                    let retaliate = match self.sp.mode {
                        Mode::Pirate | Mode::Charge => !self.board[op][j].rest,
                        _ => true,
                    };
                    self.board[op][j].dmg += ap;
                    if retaliate {
                        if let Some(i) = a {
                            self.board[me][i].dmg += tp;
                        }
                    }
                    self.reap();
                }
            }
        }
        if self.sp.mode == Mode::Pirate && t.is_none() && a.is_none() {
            self.last = "căpitanul atacă".into();
        }
    }

    fn attack_ride(&mut self, a: Option<usize>, t: Option<usize>, ap: i16) {
        let me = self.side as usize;
        let op = 1 - me;
        match t {
            Some(j) => {
                let tp = self.eff_pow(op, j);
                if ap >= tp {
                    self.board[op].remove(j);
                }
            }
            None => {
                let mut pow = ap;
                let mut crit = false;
                if a.is_none() && !self.deck[me].is_empty() {
                    let c = self.pull(me);
                    let k = self.card(c);
                    if k.fl & TRIG != 0 {
                        crit = true;
                        pow += 2;
                    }
                    self.hand[me].push(c);
                    self.last = format!("verificare: {}", k.name);
                }
                self.pend = Some(Pend { pow, gsum: 0, crit });
                self.phase = 2;
                self.side = op as u8;
            }
        }
    }

    fn resolve_guard(&mut self) {
        let pd = self.pend.take().unwrap();
        let op = self.side as usize;
        let me = 1 - op;
        self.side = me as u8;
        self.phase = 0;
        if pd.pow >= self.lead_pow(op) + pd.gsum {
            self.deal_damage(op, 1 + pd.crit as usize, me as u8);
        }
    }

    fn attackers(&self, p: usize) -> Vec<Option<usize>> {
        let mut v = Vec::new();
        let leader_attacks = matches!(self.sp.mode, Mode::Pirate | Mode::Charge | Mode::Ride);
        if leader_attacks && !self.lead_rest[p] {
            v.push(None);
        }
        for i in 0..self.board[p].len() {
            let c = &self.board[p][i];
            let k = self.card(c.c);
            if !c.rest && (!c.fresh || k.fl & HASTE != 0) {
                v.push(Some(i));
            }
        }
        v
    }

    fn targets(&self, a: Option<usize>) -> Vec<Option<usize>> {
        let me = self.side as usize;
        let op = 1 - me;
        let mut v = Vec::new();
        let arena_air = a.map(|i| self.card(self.board[me][i].c).fl & AIR);
        let in_arena = |j: usize| match arena_air {
            Some(f) => self.card(self.board[op][j].c).fl & AIR == f,
            None => true,
        };
        let mut blk: Vec<usize> = self.blockers(op);
        if self.sp.mode == Mode::Arena {
            blk.retain(|&j| in_arena(j));
        }
        if !blk.is_empty() {
            for j in blk {
                v.push(Some(j));
            }
            return v;
        }
        match self.sp.mode {
            Mode::Pirate | Mode::Charge => {
                v.push(None);
                for j in 0..self.board[op].len() {
                    if self.board[op][j].rest {
                        v.push(Some(j));
                    }
                }
            }
            Mode::Mana => {
                v.push(None);
                for j in 0..self.board[op].len() {
                    if self.board[op][j].rest {
                        v.push(Some(j));
                    }
                }
            }
            Mode::Arena => {
                v.push(None);
                for j in 0..self.board[op].len() {
                    if in_arena(j) {
                        v.push(Some(j));
                    }
                }
                // baza se lovește doar dacă nu există gardian în arena respectivă (verificat mai sus)
            }
            _ => {
                v.push(None);
                for j in 0..self.board[op].len() {
                    v.push(Some(j));
                }
            }
        }
        v
    }

    fn spell_targets(&self, p: usize, k: &Card) -> Vec<Option<usize>> {
        if k.kind != 1 {
            return vec![None];
        }
        match k.fx {
            1 => (0..self.board[1 - p].len()).map(Some).collect(),
            3 => (0..self.board[p].len()).map(Some).collect(),
            6 | 8 => {
                if self.board[1 - p].is_empty() {
                    vec![]
                } else {
                    vec![None]
                }
            }
            _ => vec![None],
        }
    }

    fn nm(&self, p: usize, j: usize) -> String {
        self.card(self.board[p][j].c).name.to_string()
    }
}

fn pl(p: u8) -> Player {
    if p == 0 {
        Player::One
    } else {
        Player::Two
    }
}

impl Mini for Tcg {
    fn side(&self) -> Player {
        pl(self.side)
    }

    fn moves(&self) -> Vec<String> {
        let p = self.side as usize;
        let mut v = Vec::new();
        if self.phase == 2 {
            for i in 0..self.hand[p].len() {
                if self.card(self.hand[p][i]).hp > 0 {
                    v.push(format!("garda {}", i));
                }
            }
            v.push("nu".into());
            return v;
        }
        if self.phase == 1 {
            for i in 0..self.hand[p].len() {
                v.push(format!("pune {}", i));
            }
            if v.is_empty() {
                v.push("gata".into());
            }
            return v;
        }
        let mode = self.sp.mode;
        for i in 0..self.hand[p].len() {
            let k = self.card(self.hand[p][i]);
            // jucare
            let ok = match mode {
                Mode::Ride => {
                    k.kind == 0
                        && k.lvl <= self.lvl_of(p)
                        && self.board[p].len() < self.sp.cap as usize
                }
                _ => {
                    self.can_pay(p, k)
                        && (k.kind != 0 || self.board[p].len() < self.sp.cap as usize)
                        && (k.kind != 2 || !self.climax)
                        && !(k.kind == 1 && mode == Mode::Ride)
                }
            };
            if ok {
                for t in self.spell_targets(p, k) {
                    match t {
                        Some(j) => v.push(format!("joaca {} t{}", i, j)),
                        None => {
                            if k.kind != 1 || !matches!(k.fx, 1 | 3) {
                                v.push(format!("joaca {}", i));
                            }
                        }
                    }
                }
            }
            if matches!(mode, Mode::Charge | Mode::Mana) && !self.acted {
                v.push(format!("pune {}", i));
            }
            if mode == Mode::Ride && !self.acted && k.kind == 0 && k.lvl == self.lvl_of(p) + 1 {
                v.push(format!("ride {}", i));
            }
            if mode == Mode::Evo
                && k.kind == 0
                && k.lvl >= 2
                && self.can_pay(
                    p,
                    &Card {
                        cost: k.cost.saturating_sub(2),
                        ..*k
                    },
                )
            {
                for j in 0..self.board[p].len() {
                    let b = self.card(self.board[p][j].c);
                    if b.lvl + 1 == k.lvl && b.civ == k.civ {
                        v.push(format!("evo {} {}", i, j));
                    }
                }
            }
        }
        for a in self.attackers(p) {
            for t in self.targets(a) {
                let an = a.map(|i| i.to_string()).unwrap_or_else(|| "l".into());
                let tn = t.map(|j| j.to_string()).unwrap_or_else(|| "p".into());
                v.push(format!("atac {} {}", an, tn));
            }
        }
        v.push("gata".into());
        v
    }

    fn step(&mut self, m: &str) {
        self.last.clear();
        self.ghost = [0; 2];
        let parts: Vec<&str> = m.split_whitespace().collect();
        let p = self.side as usize;
        match parts[0] {
            "garda" => {
                let i: usize = parts[1].parse().unwrap();
                let c = self.hand[p].remove(i);
                let k = self.card(c);
                if let Some(pd) = self.pend.as_mut() {
                    pd.gsum += k.hp as i16;
                }
                self.last = format!("garda: {}", k.name);
                return;
            }
            "nu" => {
                self.resolve_guard();
            }
            "pune" => {
                let i: usize = parts[1].parse().unwrap();
                let c = self.hand[p].remove(i);
                self.zone[p].push((c, false));
                if self.phase == 1 {
                    // alegerea resurselor în Arena: inițiativa mai întâi
                    let other = 1 - p;
                    if p as u8 == self.first {
                        self.side = other as u8;
                    } else {
                        self.phase = 0;
                        self.side = self.first;
                        self.begin_round_ready();
                    }
                    return;
                }
                self.acted = true;
                if self.sp.mode == Mode::Charge {
                    self.res[p] += 1;
                }
            }
            "joaca" => {
                let i: usize = parts[1].parse().unwrap();
                let t = parts.get(2).map(|s| s[1..].parse().unwrap());
                self.play(i, t);
                self.passes = 0;
            }
            "ride" => {
                let i: usize = parts[1].parse().unwrap();
                let c = self.hand[p].remove(i);
                self.vg[p] = c;
                self.acted = true;
                self.last = format!("urcă: {}", self.card(c).name);
            }
            "evo" => {
                let i: usize = parts[1].parse().unwrap();
                let j: usize = parts[2].parse().unwrap();
                let c = self.hand[p].remove(i);
                let k = self.card(c);
                self.mem -= k.cost.saturating_sub(2) as i16;
                self.board[p][j].c = c;
                self.draw(p, 1);
                self.last = format!("evoluează în {}", k.name);
            }
            "atac" => {
                let a = if parts[1] == "l" {
                    None
                } else {
                    Some(parts[1].parse::<usize>().unwrap())
                };
                let t = if parts[2] == "p" {
                    None
                } else {
                    Some(parts[2].parse::<usize>().unwrap())
                };
                if let Some(j) = t {
                    self.last = format!("atacă {}", self.nm(1 - p, j));
                }
                self.attack(a, t);
                self.passes = 0;
            }
            "gata" => {
                if self.phase == 1 {
                    // fără cărți de pus ca resursă
                    let other = 1 - p;
                    if p as u8 == self.first {
                        self.side = other as u8;
                    } else {
                        self.phase = 0;
                        self.side = self.first;
                        self.begin_round_ready();
                    }
                } else if self.sp.mode == Mode::Arena {
                    self.arena_pass();
                } else {
                    self.end_turn();
                }
                return;
            }
            _ => {}
        }
        if self.over.is_some() {
            return;
        }
        // memoria Evo: sub zero, tura trece
        if self.sp.mode == Mode::Evo && self.mem < 0 && self.phase == 0 {
            self.end_turn();
            return;
        }
        // în Arena fiecare acțiune dă tura adversarului
        if self.sp.mode == Mode::Arena && self.phase == 0 && parts[0] != "pune" {
            self.side = 1 - self.side;
        }
    }

    fn result(&self) -> Option<Outcome> {
        self.over
    }

    fn eval(&self, p: Player) -> i32 {
        let me = p.idx();
        let op = 1 - me;
        let mut sc = 0i32;
        sc += 3 * (self.hand[me].len() as i32 - self.hand[op].len() as i32);
        sc += self.board_value(me) - self.board_value(op);
        sc += self.ghost[me] - self.ghost[op];
        sc += 3 * (self.zone[me].len() as i32 - self.zone[op].len() as i32);
        match self.sp.mode {
            Mode::Pirate | Mode::Evo | Mode::Mana => {
                sc += 25 * (self.life[me].len() as i32 - self.life[op].len() as i32)
            }
            Mode::Charge | Mode::Arena => sc += 6 * (self.hp[me] as i32 - self.hp[op] as i32),
            Mode::Clock => {
                sc += 40 * (self.lvl[op] as i32 - self.lvl[me] as i32);
                sc += 6 * (self.clock[op].len() as i32 - self.clock[me].len() as i32);
            }
            Mode::Ride => {
                sc += 30 * (self.hp[op] as i32 - self.hp[me] as i32);
                sc += 12 * (self.lvl_of(me) as i32 - self.lvl_of(op) as i32);
            }
        }
        sc
    }

    fn show(&self) -> String {
        let sp = self.sp;
        let v = self.viewer.map(|w| w.idx()).unwrap_or(self.side as usize);
        let o = 1 - v;
        let mut s = String::new();
        s.push_str(&format!(
            "Tura {}/{} · la mutare: {}{}\n",
            self.turn.min(sp.turns),
            sp.turns,
            pname(self.side()),
            if self.phase == 2 {
                " (apărare: garda sau nu)"
            } else {
                ""
            }
        ));
        for &p in &[o, v] {
            let who = if p == v { "TU" } else { "ADVERSAR" };
            let mut line = format!("{} ({}) ", who, pname(pl(p as u8)));
            match sp.mode {
                Mode::Pirate => line.push_str(&format!(
                    "aur {}/{} · viață {} cărți",
                    self.res[p],
                    self.resmax[p],
                    self.life[p].len()
                )),
                Mode::Evo => line.push_str(&format!("pază {} cărți", self.life[p].len())),
                Mode::Charge => line.push_str(&format!(
                    "energie {}/{} · viață {}",
                    self.res[p],
                    self.zone[p].len(),
                    self.hp[p]
                )),
                Mode::Clock => line.push_str(&format!(
                    "stoc {} · ceas {}/7 · nivel {}/3",
                    self.res[p],
                    self.clock[p].len(),
                    self.lvl[p]
                )),
                Mode::Ride => line.push_str(&format!(
                    "căpitan {} · daune {}/6",
                    self.card(self.vg[p]).name,
                    self.hp[p]
                )),
                Mode::Arena => line.push_str(&format!(
                    "bază {} · resurse {}/{}",
                    self.hp[p],
                    self.res[p],
                    self.zone[p].len()
                )),
                Mode::Mana => {
                    let free = self.zone[p].iter().filter(|z| !z.1).count();
                    line.push_str(&format!(
                        "mană {}/{} · scuturi {}",
                        free,
                        self.zone[p].len(),
                        self.life[p].len()
                    ))
                }
            }
            line.push_str(&format!(
                " · mână {} · pachet {}\n",
                self.hand[p].len(),
                self.deck[p].len()
            ));
            s.push_str(&line);
            if matches!(sp.mode, Mode::Pirate | Mode::Charge) {
                s.push_str(&format!(
                    "  căpitan pow {}{}\n",
                    self.lead_pow(p),
                    if self.lead_rest[p] { " (obosit)" } else { "" }
                ));
            }
            for (j, c) in self.board[p].iter().enumerate() {
                let k = self.card(c.c);
                s.push_str(&format!(
                    "  [{}] {} pow {} viață {}/{}{}{}{}\n",
                    j,
                    k.name,
                    self.eff_pow(p, j),
                    (k.hp as i16 - c.dmg).max(0),
                    k.hp,
                    if c.rest { " obosit" } else { "" },
                    if c.fresh { " nou" } else { "" },
                    flags(k.fl)
                ));
            }
            if self.board[p].is_empty() {
                s.push_str("  (câmp gol)\n");
            }
        }
        if sp.mode == Mode::Evo {
            s.push_str(&format!("Memorie (pentru cel la mutare): {}\n", self.mem));
        }
        s.push_str(&format!("Mâna ta ({}):\n", self.hand[v].len()));
        for (i, c) in self.hand[v].iter().enumerate() {
            let k = self.card(*c);
            let ty = match k.kind {
                0 => format!("creatură pow {} viață {} nivel {}", k.pow, k.hp, k.lvl),
                1 => format!("vrajă efect {} valoare {}", k.fx, k.pow),
                _ => "climax".to_string(),
            };
            s.push_str(&format!(
                "  {}: {} cost {} culoare {} · {}{}\n",
                i,
                k.name,
                k.cost,
                k.civ,
                ty,
                flags(k.fl)
            ));
        }
        if let Some(pd) = &self.pend {
            s.push_str(&format!(
                "Atac în curs: putere {} contra gardă {}\n",
                pd.pow, pd.gsum
            ));
        }
        if !self.last.is_empty() {
            s.push_str(&format!("Ultima acțiune: {}\n", self.last));
        }
        s.push_str(
            "Mutări: joaca i [tN], pune i, ride i, evo i j, garda i, nu, atac c|l t|p, gata\n",
        );
        s
    }

    fn depth(&self) -> u32 {
        0
    }

    fn view(&mut self, w: Option<Player>) {
        self.viewer = w;
    }

    /// Clock and Mana: damage/shield cards are pulled from hidden zones, so the real zones and the dice
    /// stream must not reach the search. The opponent's hand, shields and deck are pooled and re-dealt
    /// (same sizes), as are the viewer's own shields (Mana) and deck; the dice are reseeded. Everything is
    /// derived from the viewer's own render, so indistinguishable worlds give the same view.
    fn ai_view(&self, viewer: Player) -> Tcg {
        let mut v = self.clone();
        if !matches!(self.sp.mode, Mode::Clock | Mode::Mana) {
            return v;
        }
        let me = viewer.idx();
        let op = 1 - me;
        let mut seen = self.clone();
        seen.viewer = Some(viewer);
        let mut rng = Rng::new(crate::p3_comun::fnv(&seen.show()));
        let mut pool: Vec<u8> = v.hand[op].clone();
        pool.extend(v.life[op].iter().copied());
        pool.extend(v.deck[op].iter().copied());
        pool.sort_unstable();
        crate::cards52::shuffle(&mut pool, &mut rng);
        let (nh, nl) = (v.hand[op].len(), v.life[op].len());
        v.hand[op] = pool.drain(..nh).collect();
        v.life[op] = pool.drain(..nl).collect();
        v.deck[op] = pool;
        let mut own: Vec<u8> = v.deck[me].clone();
        let nl = if self.sp.mode == Mode::Mana {
            own.extend(v.life[me].iter().copied());
            v.life[me].len()
        } else {
            0
        };
        own.sort_unstable();
        crate::cards52::shuffle(&mut own, &mut rng);
        if nl > 0 {
            v.life[me] = own.drain(..nl).collect();
        }
        v.deck[me] = own;
        v.dice = Dice::new(rng.next_u64());
        v
    }
}

fn flags(f: u8) -> String {
    let mut s = String::new();
    for (b, n) in [
        (BLOCK, " gardian"),
        (HASTE, " grabnic"),
        (DOUBLE, " dublu"),
        (TRIG, " declanșator"),
        (CANCEL, " anulare"),
        (AIR, " aer"),
    ] {
        if f & b != 0 {
            s.push_str(n);
        }
    }
    s
}

impl Tcg {
    fn lvl_of(&self, p: usize) -> u8 {
        if self.sp.mode == Mode::Ride {
            self.card(self.vg[p]).lvl
        } else {
            0
        }
    }

    fn arena_pass(&mut self) {
        if self.passes == 0 {
            self.first = self.side;
        }
        self.passes += 1;
        // primul care a dat pas are inițiativa runda următoare
        if self.passes >= 2 {
            self.end_round();
        } else {
            self.side = 1 - self.side;
        }
    }

    fn end_round(&mut self) {
        self.turn += 1;
        self.passes = 0;
        if self.turn > self.sp.turns {
            self.finish_by_standing();
            return;
        }
        for p in 0..2 {
            self.draw(p, 2);
        }
        if self.over.is_none() {
            self.phase = 1;
            self.side = self.first;
        }
    }

    fn begin_round_ready(&mut self) {
        for p in 0..2 {
            for c in self.board[p].iter_mut() {
                c.rest = false;
                c.fresh = false;
            }
            for z in self.zone[p].iter_mut() {
                z.1 = false;
            }
            self.res[p] = self.zone[p].len() as i16;
            self.resmax[p] = self.res[p];
        }
    }
}

pub fn make(id: &str) -> Box<dyn polytabletop_core::GameObj> {
    let sp = crate::t4_tdata::SPECS
        .iter()
        .find(|s| s.id == id)
        .expect("spec");
    Box::new(B(Tcg::new(sp)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::t4_tdata::SPECS;
    use polytabletop_core::rng::Rng;
    use polytabletop_core::GameObj;

    #[test]
    fn ai_ignores_hidden_cards_at_every_level() {
        for id in ["evolutia_circuitelor", "trepte_de_avangarda"] {
            let sp = SPECS.iter().find(|s| s.id == id).expect("spec");
            crate::p3_comun::ai_blind_check(
                id,
                |s| B(Tcg::with_seed(sp, s)),
                |g, me, s| g.0.scramble(1 - me.idx(), s),
                8,
            );
        }
    }

    #[test]
    fn every_spec_plays_to_the_end_and_ai_is_legal() {
        polytabletop_core::ai::set_time_budget_override_ms(Some(6));
        for sp in SPECS {
            for seed in 1..=3u64 {
                let mut g = B(Tcg::with_seed(sp, seed));
                let mut rng = Rng::new(seed * 17);
                let mut n = 0;
                while GameObj::outcome(&g).is_none() && n < 6000 {
                    let lm = GameObj::legal_moves(&g);
                    assert!(!lm.is_empty(), "{}\n{}", sp.id, g.0.show());
                    let m = if n % 7 == 0 {
                        let a = GameObj::ai_move(&g, rng.next_u64()).expect("AI");
                        assert!(lm.contains(&a), "{} AI ilegal {a}", sp.id);
                        a
                    } else {
                        lm[rng.below(lm.len())].clone()
                    };
                    GameObj::play(&mut g, &m).unwrap();
                    n += 1;
                }
                assert!(
                    GameObj::outcome(&g).is_some(),
                    "{} nu s-a terminat\n{}",
                    sp.id,
                    g.0.show()
                );
            }
        }
    }

    #[test]
    fn viewer_hides_the_other_hand_and_none_equals_mover() {
        for sp in SPECS {
            let mut g = B(Tcg::with_seed(sp, 4));
            let mut rng = Rng::new(3);
            let mut differ = false;
            for _ in 0..30 {
                if GameObj::outcome(&g).is_some() {
                    break;
                }
                let mut a = g.clone();
                a.0.view(None);
                let mut b = g.clone();
                b.0.view(Some(GameObj::to_move(&g)));
                assert_eq!(a.0.show(), b.0.show(), "{}", sp.id);
                let mut one = g.clone();
                one.0.view(Some(Player::One));
                let mut two = g.clone();
                two.0.view(Some(Player::Two));
                if one.0.show() != two.0.show() {
                    differ = true;
                }
                let lm = GameObj::legal_moves(&g);
                let m = lm[rng.below(lm.len())].clone();
                GameObj::play(&mut g, &m).unwrap();
            }
            assert!(differ, "{}: mâna nu depinde de privitor", sp.id);
        }
    }

    #[test]
    fn deterministic_with_same_seed() {
        for sp in SPECS {
            let run = |seed: u64| {
                let mut g = B(Tcg::with_seed(sp, seed));
                let mut rng = Rng::new(5);
                for _ in 0..50 {
                    if GameObj::outcome(&g).is_some() {
                        break;
                    }
                    let lm = GameObj::legal_moves(&g);
                    let m = lm[rng.below(lm.len())].clone();
                    GameObj::play(&mut g, &m).unwrap();
                }
                g.0.show()
            };
            assert_eq!(run(9), run(9), "{}", sp.id);
        }
    }

    #[cfg(test)]
    mod ai_view_audit {
        use super::*;
        use crate::t4_tdata::SPECS;

        #[test]
        fn ai_view_hides_zones_and_dice_in_clock_and_mana() {
            for id in ["ceasul_alb_negru", "scuturi_si_mana"] {
                let sp = SPECS.iter().find(|s| s.id == id).expect("spec");
                crate::p3_comun::ai_view_check(
                    id,
                    |s| B(Tcg::with_seed(sp, s)),
                    |g, me, s| {
                        g.0.scramble(1 - me.idx(), s);
                        // propriile scuturi sunt și ele nevăzute (Mana)
                        let own = me.idx();
                        let mut t = g.0.life[own].clone();
                        crate::p3_comun::shuffle_with(&mut t, s ^ 5);
                        g.0.life[own] = t;
                    },
                    8,
                );
            }
        }
    }
}
