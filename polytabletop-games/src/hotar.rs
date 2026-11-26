//! Hotarul — motor de resurse și teritorii pe o hartă hexagonală (variantă originală pentru doi).
//!
//! Harta are 37 de hexagoane (rânduri a–g): câmpie C (grâne), pădure P (lemn), munte M (fier),
//! lac L (impracticabil), două sate natale H și Cetatea Veche X în centru. Fiecare jucător alege
//! o facțiune (`alege luncenii`), are un erou, până la 4 călăreți și până la 6 muncitori.
//!
//! Tura: alegi una din cele 4 secțiuni ale tablei tale (nu aceeași ca în tura ta precedentă),
//! faci acțiunea de SUS (opțională: `sN sari`), apoi acțiunea de JOS (opțională: `gata`):
//! * s1 Marș — sus: `s1 mars h:g1>f1` (h erou, k călăreț, w muncitor; 2 pași + upgrade-uri,
//!   `mars ...` pentru următorii, `gata` oprește) sau `s1 bani` (+2 monede). Jos: `mobilizeaza f1`
//!   (plătești fier, un călăreț pe un hex cu muncitor sau satul tău; max. 4).
//! * s2 Întărire — sus: `s2 putere` (+2 putere) sau `s2 carti` (+2 cărți de luptă). Jos:
//!   `imbunatateste mars|productie|comert` (plătești grâne; fiecare pistă de cel mult 2 ori).
//! * s3 Producție — sus: `s3 produce a1 b2` (până la 2 + upgrade hexuri cu muncitori: fiecare muncitor
//!   dă o resursă a hexului) sau `s3 muncitor` (1 grâne, un muncitor în sat). Jos:
//!   `construieste moara|fortareata|monument f1` (plătești lemn, pe un hex cu o unitate a ta).
//! * s4 Comerț — sus: `s4 comert grane|lemn|fier` (1 monedă → 2 + upgrade resurse) sau `s4 faima`
//!   (1 monedă → +2 popularitate). Jos: `inrolare 1..4` (plătești grâne: o dată pe loc; recompense
//!   +3 putere / +2 cărți / +3 monede / +2 popularitate).
//! Luptă: un erou sau călăreț care intră pe un hex cu unități de luptă inamice declanșează lupta.
//! Atacatorul declară `lupta P C` (putere P ≤ 7 din rezerva ta, carte de valoare C sau `-`), apoi
//! apărătorul face la fel fără să știe, apoi `rezolva`. Forță = unități (erou 2, călăreț 1) + putere
//! + carte (+2 Fortăreață pe hex; +2 muntele pentru munteni); egal câștigă apărătorul. Puterea și cărțile
//! folosite se pierd; cine pierde se retrage acasă cu tot ce are pe hex; câștigătorul +1 popularitate.
//! Intrarea pe un hex cu doar muncitori inamici îi trimite acasă și costă 1 popularitate.
//! Stele (la 5 jocul se termină): 4 upgrade-uri, 3 călăreți, 3 clădiri, 3 înrolări, putere 12,
//! popularitate 12, prima și a doua victorie în luptă. Scor final: monede + stele × a + hexuri
//! controlate × b + perechi de resurse × c (a, b, c cresc cu popularitatea: 3/2/1, 4/3/2, 5/4/3).
//! Informație ascunsă: cărțile de luptă ale adversarului, valorile angajate în luptă.
#![allow(clippy::needless_range_loop, clippy::doc_lazy_continuation)]

use crate::cards::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::sync::OnceLock;

pub const N: usize = 37;
pub const CENTER: usize = 18;
pub const HOME: [usize; 2] = [33, 3];
const MAX_TURNS: u32 = 32;
const STAR_GOAL: u32 = 5;
const RES_N: [&str; 3] = ["grane", "lemn", "fier"];
const SEC_N: [&str; 4] = ["Marș", "Întărire", "Producție", "Comerț"];
const TRACK: [&str; 3] = ["mars", "productie", "comert"];
const BLD: [&str; 3] = ["moara", "fortareata", "monument"];
const TERR_HALF: &str = "MPCHPCLMCLMCPPCMCLX";

struct Geo {
    name: Vec<String>,
    terr: [u8; N],
    nb: Vec<Vec<u8>>,
    dist: Vec<Vec<u8>>,
}

fn geo() -> &'static Geo {
    static G: OnceLock<Geo> = OnceLock::new();
    G.get_or_init(|| {
        let mut qr: Vec<(i32, i32)> = Vec::new();
        let mut name = Vec::new();
        for r in -3..=3i32 {
            let (lo, hi) = ((-3).max(-3 - r), 3.min(3 - r));
            for (j, q) in (lo..=hi).enumerate() {
                qr.push((q, r));
                name.push(format!("{}{}", (b'a' + (r + 3) as u8) as char, j + 1));
            }
        }
        assert_eq!(qr.len(), N);
        let mut terr = [b'C'; N];
        let half = TERR_HALF.as_bytes();
        for i in 0..=CENTER {
            terr[i] = half[i];
            terr[N - 1 - i] = half[i];
        }
        let d = |a: (i32, i32), b: (i32, i32)| {
            let (dq, dr) = (a.0 - b.0, a.1 - b.1);
            ((dq.abs() + dr.abs() + (dq + dr).abs()) / 2) as u8
        };
        let mut nb = vec![Vec::new(); N];
        let mut dist = vec![vec![0u8; N]; N];
        for i in 0..N {
            for j in 0..N {
                dist[i][j] = d(qr[i], qr[j]);
                if dist[i][j] == 1 {
                    nb[i].push(j as u8);
                }
            }
        }
        Geo {
            name,
            terr,
            nb,
            dist,
        }
    })
}

#[cfg(test)]
fn hex_of(name: &str) -> usize {
    geo().name.iter().position(|n| n == name).unwrap()
}

struct Fac {
    name: &'static str,
    cmd: &'static str,
    coins: i32,
    power: i32,
    pop: i32,
    /// costuri: mobilizare (fier), îmbunătățire (grâne), construcție (lemn), înrolare (grâne)
    cost: [i32; 4],
    /// monede primite la fiecare acțiune de jos
    rew: [i32; 4],
    desc: &'static str,
}

const FAC: [Fac; 4] = [
    Fac {
        name: "Luncenii",
        cmd: "luncenii",
        coins: 5,
        power: 2,
        pop: 2,
        cost: [3, 2, 2, 2],
        rew: [1, 2, 2, 1],
        desc: "trec prin lacuri",
    },
    Fac {
        name: "Munteanii",
        cmd: "munteanii",
        coins: 4,
        power: 3,
        pop: 2,
        cost: [2, 2, 3, 2],
        rew: [2, 1, 1, 2],
        desc: "+2 forță în luptă pe munte",
    },
    Fac {
        name: "Pădurenii",
        cmd: "padurenii",
        coins: 6,
        power: 1,
        pop: 3,
        cost: [3, 1, 2, 3],
        rew: [1, 2, 2, 0],
        desc: "+1 lemn la fiecare hex de pădure produs",
    },
    Fac {
        name: "Câmpenii",
        cmd: "campenii",
        coins: 4,
        power: 2,
        pop: 4,
        cost: [3, 2, 3, 1],
        rew: [0, 1, 3, 2],
        desc: "+1 pas la Marș",
    },
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Top {
    Skip,
    Step(u8, u8, u8),
    Coins,
    Power,
    Cards,
    Produce(u64),
    Worker,
    Trade(u8),
    Fame,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Bt {
    Deploy(u8),
    Upgrade(u8),
    Build(u8, u8),
    Enlist(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Pick(u8),
    Sec(u8, Top),
    Step(u8, u8, u8),
    Done,
    Bottom(Bt),
    Fight(u8, u8),
    Resolve,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Pick,
    Sec,
    Top,
    Bottom,
    AttCommit,
    DefCommit,
    Resolve,
}

#[derive(Clone, Debug)]
struct Pl {
    fac: u8,
    coins: i32,
    power: i32,
    pop: i32,
    cards: Vec<u8>,
    res: [i32; 3],
    hero: u8,
    knights: Vec<u8>,
    workers: Vec<u8>,
    bld: Vec<(u8, u8)>,
    up: [u8; 3],
    enl: u8,
    stars: u16,
    wins: u8,
    last_sec: u8,
    centre: bool,
}

#[derive(Clone, Copy, Debug)]
struct Combat {
    hex: u8,
    att: u8,
    ac: Option<(u8, u8)>,
    dc: Option<(u8, u8)>,
}

#[derive(Clone)]
pub struct Hotar {
    /// Whose private info `render` shows (`None` = the mover's, hot-seat).
    viewer: Option<Player>,
    pl: [Pl; 2],
    deck: Vec<u8>,
    disc: Vec<u8>,
    rng: Rng,
    side: Player,
    phase: Phase,
    cur_sec: u8,
    steps_left: i32,
    combat: Option<Combat>,
    turns: [u32; 2],
    over: bool,
    fresh: [u8; 2],
    picks: [u8; 2],
}

impl Default for Hotar {
    fn default() -> Self {
        Self::new()
    }
}

fn pl_of(i: usize) -> Player {
    if i == 0 {
        Player::One
    } else {
        Player::Two
    }
}

fn logistic(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

impl Hotar {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck: Vec<u8> = Vec::new();
        for v in 1..=5u8 {
            for _ in 0..6 {
                deck.push(v);
            }
        }
        shuffle(&mut deck, &mut rng);
        let blank = |h: usize| Pl {
            fac: 255,
            coins: 0,
            power: 0,
            pop: 0,
            cards: Vec::new(),
            res: [0; 3],
            hero: h as u8,
            knights: Vec::new(),
            workers: Vec::new(),
            bld: Vec::new(),
            up: [0; 3],
            enl: 0,
            stars: 0,
            wins: 0,
            last_sec: 255,
            centre: false,
        };
        Hotar {
            viewer: None,
            pl: [blank(HOME[0]), blank(HOME[1])],
            deck,
            disc: Vec::new(),
            rng,
            side: Player::One,
            phase: Phase::Pick,
            cur_sec: 0,
            steps_left: 0,
            combat: None,
            turns: [0; 2],
            over: false,
            fresh: [0; 2],
            picks: [255; 2],
        }
    }

    fn fac(&self, p: usize) -> &'static Fac {
        &FAC[self.pl[p].fac as usize]
    }

    fn draw(&mut self, p: usize, n: usize) {
        for _ in 0..n {
            if self.deck.is_empty() {
                if self.disc.is_empty() {
                    return;
                }
                self.deck = std::mem::take(&mut self.disc);
                shuffle(&mut self.deck, &mut self.rng);
            }
            let c = self.deck.pop().unwrap();
            self.pl[p].cards.push(c);
            self.fresh[p] += 1;
        }
    }

    pub fn card_total(&self) -> usize {
        self.deck.len() + self.disc.len() + self.pl[0].cards.len() + self.pl[1].cards.len()
    }

    fn start(&mut self) {
        for p in 0..2 {
            let f = &FAC[self.pl[p].fac as usize];
            self.pl[p].coins = f.coins;
            self.pl[p].power = f.power;
            self.pl[p].pop = f.pop;
            let g = geo();
            let mut ws: Vec<u8> = g.nb[HOME[p]]
                .iter()
                .copied()
                .filter(|h| g.terr[*h as usize] != b'L')
                .collect();
            ws.truncate(2);
            self.pl[p].workers = ws;
            self.draw(p, 2);
        }
        self.fresh = [0; 2];
        self.phase = Phase::Sec;
        self.side = Player::One;
    }

    // ---------- interogări ----------

    fn terr(h: usize) -> u8 {
        geo().terr[h]
    }

    fn res_of(h: usize) -> Option<usize> {
        match Self::terr(h) {
            b'C' => Some(0),
            b'P' => Some(1),
            b'M' => Some(2),
            _ => None,
        }
    }

    fn combat_units_at(&self, p: usize, h: usize) -> (bool, usize) {
        let pl = &self.pl[p];
        (
            pl.hero as usize == h,
            pl.knights.iter().filter(|k| **k as usize == h).count(),
        )
    }

    fn has_combat(&self, p: usize, h: usize) -> bool {
        let (a, b) = self.combat_units_at(p, h);
        a || b > 0
    }

    fn workers_at(&self, p: usize, h: usize) -> usize {
        self.pl[p]
            .workers
            .iter()
            .filter(|w| **w as usize == h)
            .count()
    }

    fn has_any(&self, p: usize, h: usize) -> bool {
        self.has_combat(p, h)
            || self.workers_at(p, h) > 0
            || self.pl[p].bld.iter().any(|b| b.0 as usize == h)
    }

    fn units_hexes(&self, p: usize) -> Vec<usize> {
        let pl = &self.pl[p];
        let mut v: Vec<usize> = vec![pl.hero as usize];
        v.extend(pl.knights.iter().map(|x| *x as usize));
        v.extend(pl.workers.iter().map(|x| *x as usize));
        v.sort_unstable();
        v.dedup();
        v
    }

    fn can_enter(&self, p: usize, kind: u8, h: usize) -> bool {
        let t = Self::terr(h);
        if t == b'L' && self.pl[p].fac != 0 {
            return false;
        }
        if h == HOME[1 - p] {
            return false;
        }
        if kind == 2 && self.has_any_unit(1 - p, h) {
            return false;
        }
        true
    }

    fn has_any_unit(&self, p: usize, h: usize) -> bool {
        self.has_combat(p, h) || self.workers_at(p, h) > 0
    }

    fn step_budget(&self, p: usize) -> i32 {
        2 + i32::from(self.pl[p].up[0]) + i32::from(self.pl[p].fac == 3)
    }

    fn step_moves(&self, p: usize) -> Vec<(u8, u8, u8)> {
        let g = geo();
        let pl = &self.pl[p];
        let mut v: Vec<(u8, u8, u8)> = Vec::new();
        let mut srcs: Vec<(u8, u8)> = vec![(0, pl.hero)];
        srcs.extend(pl.knights.iter().map(|k| (1, *k)));
        srcs.extend(pl.workers.iter().map(|w| (2, *w)));
        for (kind, from) in srcs {
            for to in &g.nb[from as usize] {
                if self.can_enter(p, kind, *to as usize) {
                    let m = (kind, from, *to);
                    if !v.contains(&m) {
                        v.push(m);
                    }
                }
            }
        }
        v
    }

    fn produce_hexes(&self, p: usize) -> Vec<usize> {
        let mut v: Vec<usize> = self.pl[p]
            .workers
            .iter()
            .map(|w| *w as usize)
            .filter(|h| Self::res_of(*h).is_some())
            .collect();
        v.sort_unstable();
        v.dedup();
        v
    }

    fn bottom_moves(&self, p: usize, sec: u8) -> Vec<Bt> {
        let pl = &self.pl[p];
        let f = self.fac(p);
        let mut v = Vec::new();
        match sec {
            0 => {
                if pl.knights.len() < 4 && pl.res[2] >= f.cost[0] {
                    let mut hs: Vec<usize> = vec![HOME[p]];
                    hs.extend(pl.workers.iter().map(|w| *w as usize));
                    hs.sort_unstable();
                    hs.dedup();
                    for h in hs {
                        v.push(Bt::Deploy(h as u8));
                    }
                }
            }
            1 => {
                for t in 0..3u8 {
                    if pl.up[t as usize] < 2 && pl.res[0] >= f.cost[1] {
                        v.push(Bt::Upgrade(t));
                    }
                }
            }
            2 => {
                if pl.res[1] >= f.cost[2] {
                    for k in 0..3u8 {
                        if pl.bld.iter().any(|b| b.1 == k) {
                            continue;
                        }
                        for h in self.units_hexes(p) {
                            let t = Self::terr(h);
                            let taken = self
                                .pl
                                .iter()
                                .any(|q| q.bld.iter().any(|b| b.0 as usize == h));
                            if t != b'L'
                                && t != b'H'
                                && t != b'X'
                                && !taken
                                && !self.has_any_unit(1 - p, h)
                            {
                                v.push(Bt::Build(k, h as u8));
                            }
                        }
                    }
                }
            }
            _ => {
                if pl.res[0] >= f.cost[3] {
                    for s in 0..4u8 {
                        if pl.enl & (1 << s) == 0 {
                            v.push(Bt::Enlist(s));
                        }
                    }
                }
            }
        }
        v
    }

    fn top_moves(&self, p: usize, sec: u8) -> Vec<Top> {
        let pl = &self.pl[p];
        let mut v = vec![Top::Skip];
        match sec {
            0 => {
                for (k, f, t) in self.step_moves(p) {
                    v.push(Top::Step(k, f, t));
                }
                v.push(Top::Coins);
            }
            1 => {
                v.push(Top::Power);
                v.push(Top::Cards);
            }
            2 => {
                let hs = self.produce_hexes(p);
                let cap = 2 + pl.up[1] as usize;
                let n = hs.len();
                for mask in 1u32..(1 << n) {
                    if (mask.count_ones() as usize) <= cap {
                        let mut m = 0u64;
                        for i in 0..n {
                            if mask & (1 << i) != 0 {
                                m |= 1 << hs[i];
                            }
                        }
                        v.push(Top::Produce(m));
                    }
                }
                if pl.workers.len() < 6 && pl.res[0] >= 1 {
                    v.push(Top::Worker);
                }
            }
            _ => {
                if pl.coins >= 1 {
                    for r in 0..3u8 {
                        v.push(Top::Trade(r));
                    }
                    if pl.pop < 18 {
                        v.push(Top::Fame);
                    }
                }
            }
        }
        v
    }

    fn combat_total(&self, p: usize, h: usize, pw: u8, card: u8) -> i32 {
        let (hero, kn) = self.combat_units_at(p, h);
        let mut t = i32::from(pw) + i32::from(card) + if hero { 2 } else { 0 } + kn as i32;
        if self.pl[p].fac == 1 && Self::terr(h) == b'M' {
            t += 2;
        }
        if self.pl[p].bld.iter().any(|b| b.0 as usize == h && b.1 == 1) {
            t += 2;
        }
        t
    }

    pub fn territories(&self, p: usize) -> usize {
        (0..N)
            .filter(|h| Self::terr(*h) != b'L' && self.has_any(p, *h))
            .count()
    }

    fn tier(pop: i32) -> (i32, i32, i32) {
        if pop >= 13 {
            (5, 4, 3)
        } else if pop >= 7 {
            (4, 3, 2)
        } else {
            (3, 2, 1)
        }
    }

    pub fn score(&self, p: usize) -> i32 {
        let pl = &self.pl[p];
        let (a, b, c) = Self::tier(pl.pop);
        pl.coins
            + a * pl.stars.count_ones() as i32
            + b * self.territories(p) as i32
            + c * (pl.res.iter().sum::<i32>() / 2)
    }

    fn check_stars(&mut self) {
        for p in 0..2 {
            let pl = &self.pl[p];
            let mut m = 0u16;
            let flags = [
                pl.up.iter().map(|x| u32::from(*x)).sum::<u32>() >= 4,
                pl.knights.len() >= 3,
                pl.bld.len() >= 3,
                pl.enl.count_ones() >= 3,
                pl.power >= 12,
                pl.pop >= 12,
                pl.wins >= 1,
                pl.wins >= 2,
            ];
            for (i, f) in flags.iter().enumerate() {
                if *f {
                    m |= 1 << i;
                }
            }
            self.pl[p].stars |= m;
            if self.pl[p].stars.count_ones() >= STAR_GOAL {
                self.over = true;
            }
        }
    }

    fn end_turn(&mut self) {
        let p = self.side.idx();
        self.pl[p].last_sec = self.cur_sec;
        self.turns[p] += 1;
        if self.side == Player::Two && self.turns[1] >= MAX_TURNS {
            self.over = true;
        }
        self.side = self.side.other();
        self.phase = Phase::Sec;
        self.steps_left = 0;
    }

    fn after_top(&mut self) {
        let p = self.side.idx();
        if self.phase == Phase::Top || self.phase == Phase::Sec {
            if self.steps_left > 0 && self.cur_sec == 0 && !self.step_moves(p).is_empty() {
                self.phase = Phase::Top;
            } else {
                self.phase = Phase::Bottom;
            }
        }
    }

    fn do_step(&mut self, p: usize, kind: u8, from: u8, to: u8) {
        {
            let pl = &mut self.pl[p];
            match kind {
                0 => pl.hero = to,
                1 => {
                    let i = pl.knights.iter().position(|k| *k == from).unwrap();
                    pl.knights[i] = to;
                }
                _ => {
                    let i = pl.workers.iter().position(|k| *k == from).unwrap();
                    pl.workers[i] = to;
                }
            }
        }
        let to = to as usize;
        if to == CENTER && !self.pl[p].centre {
            self.pl[p].centre = true;
            self.pl[p].coins += 2;
            self.draw(p, 1);
        }
        let o = 1 - p;
        if kind != 2 {
            if self.has_combat(o, to) {
                self.combat = Some(Combat {
                    hex: to as u8,
                    att: p as u8,
                    ac: None,
                    dc: None,
                });
                self.phase = Phase::AttCommit;
                return;
            }
            if self.workers_at(o, to) > 0 {
                let home = HOME[o] as u8;
                for w in self.pl[o].workers.iter_mut() {
                    if *w as usize == to {
                        *w = home;
                    }
                }
                self.pl[p].pop = (self.pl[p].pop - 1).max(0);
            }
        }
        self.phase = Phase::Top;
        self.after_top();
    }

    fn retreat(&mut self, p: usize, h: usize) {
        let home = HOME[p] as u8;
        let pl = &mut self.pl[p];
        if pl.hero as usize == h {
            pl.hero = home;
        }
        for k in pl.knights.iter_mut() {
            if *k as usize == h {
                *k = home;
            }
        }
        for w in pl.workers.iter_mut() {
            if *w as usize == h {
                *w = home;
            }
        }
    }

    fn resolve(&mut self) {
        let c = self.combat.take().unwrap();
        let a = c.att as usize;
        let d = 1 - a;
        let h = c.hex as usize;
        let (ap, ac) = c.ac.unwrap_or((0, 0));
        let (dp, dc) = c.dc.unwrap_or((0, 0));
        let at = self.combat_total(a, h, ap, ac);
        let dt = self.combat_total(d, h, dp, dc);
        for (p, pw, cd) in [(a, ap, ac), (d, dp, dc)] {
            self.pl[p].power = (self.pl[p].power - i32::from(pw)).max(0);
            if cd > 0 {
                if let Some(i) = self.pl[p].cards.iter().position(|x| *x == cd) {
                    self.pl[p].cards.remove(i);
                    self.disc.push(cd);
                }
            }
        }
        let (win, lose) = if at > dt { (a, d) } else { (d, a) };
        self.retreat(lose, h);
        let w = &mut self.pl[win];
        w.pop = (w.pop + 1).min(18);
        w.wins = w.wins.saturating_add(1).min(2);
        self.phase = Phase::Top;
        self.after_top();
    }

    // ---------- evaluare ----------

    fn val(&self, p: usize, me: usize) -> f64 {
        let pl = &self.pl[p];
        let g = geo();
        let mut s = f64::from(pl.coins) + 38.0 * f64::from(pl.stars.count_ones());
        s += 1.3 * f64::from(pl.power) + 1.3 * f64::from(pl.pop);
        if p == me {
            let known = pl.cards.len().saturating_sub(self.fresh[p] as usize);
            let mut vals: Vec<u8> = pl.cards.iter().take(known).copied().collect();
            vals.sort_unstable_by(|a, b| b.cmp(a));
            for c in vals.iter().take(6) {
                s += 0.8 * f64::from(*c);
            }
            s += 2.4 * ((pl.cards.len() - known) as f64).min((6usize.saturating_sub(known)) as f64);
        } else {
            s += 2.4 * (pl.cards.len().min(6)) as f64;
        }
        let rt: i32 = pl.res.iter().sum();
        s += 1.5 * f64::from(rt.min(6)) + 0.5 * f64::from((rt - 6).max(0));
        s += 5.0 * pl.workers.len() as f64 + 7.0 * pl.knights.len() as f64;
        s += 8.0 * pl.bld.len() as f64;
        let ups: u32 = pl.up.iter().map(|x| u32::from(*x)).sum();
        s += 6.0 * f64::from(ups) + 2.0 * f64::from(pl.enl.count_ones());
        s += 2.5 * self.territories(p) as f64;
        for w in &pl.workers {
            if Self::res_of(*w as usize).is_some() {
                s += 2.0;
            }
        }
        s -= 0.5 * f64::from(g.dist[pl.hero as usize][CENTER]);
        // presiune: unitățile de luptă se apropie de eroul adversarului
        let eh = self.pl[1 - p].hero as usize;
        s -= 1.1 * f64::from(g.dist[pl.hero as usize][eh]);
        for k in &pl.knights {
            s -= 0.8 * f64::from(g.dist[*k as usize][eh]);
        }
        // progres spre stele
        let prog = [
            f64::from(ups) / 4.0,
            pl.knights.len() as f64 / 3.0,
            pl.bld.len() as f64 / 3.0,
            f64::from(pl.enl.count_ones()) / 3.0,
            f64::from(pl.power) / 12.0,
            f64::from(pl.pop) / 12.0,
        ];
        for (i, x) in prog.iter().enumerate() {
            if pl.stars & (1 << i) == 0 {
                s += 9.0 * x.min(1.0);
            }
        }
        s
    }

    fn combat_term(&self, p: usize) -> f64 {
        let Some(c) = self.combat else { return 0.0 };
        let a = c.att as usize;
        let d = 1 - a;
        let h = c.hex as usize;
        let est = |q: usize| -> f64 {
            let pl = &self.pl[q];
            let (hero, kn) = self.combat_units_at(q, h);
            let base = if hero { 2.0 } else { 0.0 } + kn as f64;
            base + 0.45 * f64::from(pl.power.min(7)) + if pl.cards.is_empty() { 0.0 } else { 2.0 }
        };
        let val_commit = |q: usize, cm: Option<(u8, u8)>| -> (f64, f64) {
            match cm {
                Some((pw, cd)) => (
                    f64::from(self.combat_total(q, h, pw, cd)),
                    3.0 * f64::from(pw) + 1.5 * f64::from(cd),
                ),
                None => (est(q), 0.0),
            }
        };
        let (mine, theirs, cost, i_att) = if p == a {
            let (m, c1) = val_commit(a, c.ac);
            (m, est(d), c1, true)
        } else {
            let (m, c1) = val_commit(d, c.dc);
            (m, est(a), c1, false)
        };
        let diff = if i_att {
            mine - theirs - 0.5
        } else {
            mine - theirs + 0.5
        };
        let pw = logistic(diff / 1.8);
        30.0 * pw - 15.0 * (1.0 - pw) - cost
    }

    fn section_potential(&self, p: usize) -> f64 {
        if self.phase == Phase::Top || self.phase == Phase::Bottom {
            let n = self.bottom_moves(p, self.cur_sec).len();
            if n > 0 {
                return 5.0;
            }
        }
        0.0
    }
}

impl Game for Hotar {
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
        let mut v = Vec::new();
        match self.phase {
            Phase::Pick => {
                for f in 0..4u8 {
                    if !self.picks.contains(&f) {
                        v.push(Move::Pick(f));
                    }
                }
            }
            Phase::Sec => {
                for s in 0..4u8 {
                    if s == self.pl[p].last_sec {
                        continue;
                    }
                    for t in self.top_moves(p, s) {
                        v.push(Move::Sec(s, t));
                    }
                }
            }
            Phase::Top => {
                for (k, f, t) in self.step_moves(p) {
                    v.push(Move::Step(k, f, t));
                }
                v.push(Move::Done);
            }
            Phase::Bottom => {
                for b in self.bottom_moves(p, self.cur_sec) {
                    v.push(Move::Bottom(b));
                }
                v.push(Move::Done);
            }
            Phase::AttCommit | Phase::DefCommit => {
                let mut cards: Vec<u8> = self.pl[p].cards.clone();
                cards.sort_unstable();
                cards.dedup();
                cards.insert(0, 0);
                for pw in 0..=(self.pl[p].power.min(7) as u8) {
                    for c in &cards {
                        v.push(Move::Fight(pw, *c));
                    }
                }
            }
            Phase::Resolve => v.push(Move::Resolve),
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        self.fresh = [0; 2];
        let p = self.side.idx();
        match *m {
            Move::Pick(f) => {
                self.pl[p].fac = f;
                self.picks[p] = f;
                if p == 0 {
                    self.side = Player::Two;
                } else {
                    self.start();
                }
                return;
            }
            Move::Sec(s, t) => {
                self.cur_sec = s;
                self.phase = Phase::Top;
                match t {
                    Top::Skip => {
                        self.phase = Phase::Bottom;
                    }
                    Top::Step(k, f, to) => {
                        self.steps_left = self.step_budget(p) - 1;
                        self.do_step(p, k, f, to);
                    }
                    Top::Coins => {
                        self.pl[p].coins += 2;
                        self.phase = Phase::Bottom;
                    }
                    Top::Power => {
                        self.pl[p].power = (self.pl[p].power + 2).min(16);
                        self.phase = Phase::Bottom;
                    }
                    Top::Cards => {
                        self.draw(p, 2);
                        self.phase = Phase::Bottom;
                    }
                    Top::Produce(mask) => {
                        for h in 0..N {
                            if mask & (1 << h) == 0 {
                                continue;
                            }
                            let r = Self::res_of(h).unwrap();
                            let n = self.workers_at(p, h) as i32;
                            let mill = self.pl[p].bld.iter().any(|b| b.0 as usize == h && b.1 == 0);
                            let mut gain = n + i32::from(mill);
                            if self.pl[p].fac == 2 && r == 1 {
                                gain += 1;
                            }
                            self.pl[p].res[r] += gain;
                        }
                        self.phase = Phase::Bottom;
                    }
                    Top::Worker => {
                        self.pl[p].res[0] -= 1;
                        self.pl[p].workers.push(HOME[p] as u8);
                        self.phase = Phase::Bottom;
                    }
                    Top::Trade(r) => {
                        self.pl[p].coins -= 1;
                        self.pl[p].res[r as usize] += 2 + i32::from(self.pl[p].up[2]);
                        self.phase = Phase::Bottom;
                    }
                    Top::Fame => {
                        self.pl[p].coins -= 1;
                        self.pl[p].pop = (self.pl[p].pop + 2).min(18);
                        self.phase = Phase::Bottom;
                    }
                }
            }
            Move::Step(k, f, t) => {
                self.steps_left -= 1;
                self.do_step(p, k, f, t);
            }
            Move::Done => {
                if self.phase == Phase::Top {
                    self.phase = Phase::Bottom;
                } else {
                    self.end_turn();
                }
            }
            Move::Bottom(b) => {
                let f = self.fac(p);
                match b {
                    Bt::Deploy(h) => {
                        self.pl[p].res[2] -= f.cost[0];
                        self.pl[p].knights.push(h);
                        self.pl[p].coins += f.rew[0];
                    }
                    Bt::Upgrade(t) => {
                        self.pl[p].res[0] -= f.cost[1];
                        self.pl[p].up[t as usize] += 1;
                        self.pl[p].coins += f.rew[1];
                    }
                    Bt::Build(k, h) => {
                        self.pl[p].res[1] -= f.cost[2];
                        self.pl[p].bld.push((h, k));
                        self.pl[p].coins += f.rew[2];
                        if k == 2 {
                            self.pl[p].pop = (self.pl[p].pop + 2).min(18);
                        }
                    }
                    Bt::Enlist(s) => {
                        self.pl[p].res[0] -= f.cost[3];
                        self.pl[p].enl |= 1 << s;
                        self.pl[p].coins += f.rew[3];
                        match s {
                            0 => self.pl[p].power = (self.pl[p].power + 3).min(16),
                            1 => self.draw(p, 2),
                            2 => self.pl[p].coins += 3,
                            _ => self.pl[p].pop = (self.pl[p].pop + 2).min(18),
                        }
                    }
                }
                self.end_turn();
            }
            Move::Fight(pw, c) => {
                let cm = self.combat.as_mut().unwrap();
                if self.phase == Phase::AttCommit {
                    cm.ac = Some((pw, c));
                    self.phase = Phase::DefCommit;
                    self.side = pl_of(1 - cm.att as usize);
                } else {
                    cm.dc = Some((pw, c));
                    self.phase = Phase::Resolve;
                    self.side = pl_of(cm.att as usize);
                }
            }
            Move::Resolve => self.resolve(),
        }
        self.check_stars();
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            return None;
        }
        let (a, b) = (self.score(0), self.score(1));
        Some(if a > b {
            Outcome::Win(Player::One)
        } else if b > a {
            Outcome::Win(Player::Two)
        } else {
            Outcome::Draw
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        if self.phase == Phase::Pick {
            return 0;
        }
        let mut s = self.val(me, me) - self.val(1 - me, me);
        s += self.combat_term(me);
        if self.side == p {
            s += self.section_potential(me);
        }
        s as i32
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }

    fn render(&self) -> String {
        let g = geo();
        let mut s = String::new();
        if self.phase == Phase::Pick {
            s.push_str("Hotarul — alege facțiunea:\n");
            for (i, f) in FAC.iter().enumerate() {
                if !self.picks.contains(&(i as u8)) {
                    s.push_str(&format!(
                        "  alege {} — {} (monede {}, putere {}, popularitate {}; {})\n",
                        f.cmd, f.name, f.coins, f.power, f.pop, f.desc
                    ));
                }
            }
            s.push_str(&format!("Mută J{}.\n", self.side.idx() + 1));
            return s;
        }
        if self.over {
            s.push_str("Hotarul — joc încheiat\n");
        } else {
            s.push_str(&format!(
                "Hotarul — tura {} (cel care ajunge la {} stele sau tura {} încheie jocul)\n",
                self.turns[self.side.idx()] + 1,
                STAR_GOAL,
                MAX_TURNS
            ));
        }
        // hartă
        let mut idx = 0;
        for r in 0..7usize {
            let len = [4, 5, 6, 7, 6, 5, 4][r];
            s.push_str(&" ".repeat(2 * (7 - len)));
            for _ in 0..len {
                let t = g.terr[idx] as char;
                let mut occ = String::new();
                for p in 0..2 {
                    let (hero, kn) = self.combat_units_at(p, idx);
                    let w = self.workers_at(p, idx);
                    let tag = if p == 0 { 'a' } else { 'z' };
                    if hero {
                        occ.push(if p == 0 { 'H' } else { 'Q' });
                    }
                    if kn > 0 {
                        occ.push_str(&format!("{}k{}", tag, kn));
                    }
                    if w > 0 {
                        occ.push_str(&format!("{}w{}", tag, w));
                    }
                    for b in &self.pl[p].bld {
                        if b.0 as usize == idx {
                            occ.push_str(&format!("[{}{}]", tag, &BLD[b.1 as usize][..1]));
                        }
                    }
                }
                s.push_str(&format!("{}:{}{} ", g.name[idx], t, occ));
                idx += 1;
            }
            s.push('\n');
        }
        s.push_str("(H/Q eroii J1/J2; a = J1, z = J2: k călăreți, w muncitori; [aM] clădire)\n");
        for p in 0..2 {
            let pl = &self.pl[p];
            let f = self.fac(p);
            s.push_str(&format!(
                "J{} {}: scor {} | monede {} putere {} popularitate {} stele {} | resurse G{} L{} F{} | cărți {} | upgrade {:?} înrolări {:04b} victorii {}\n",
                p + 1,
                f.name,
                self.score(p),
                pl.coins,
                pl.power,
                pl.pop,
                pl.stars.count_ones(),
                pl.res[0],
                pl.res[1],
                pl.res[2],
                pl.cards.len(),
                pl.up,
                pl.enl,
                pl.wins
            ));
            s.push_str(&format!(
                "   erou {} | călăreți {} | muncitori {} | ultima secțiune {}\n",
                g.name[pl.hero as usize],
                pl.knights.len(),
                pl.workers.len(),
                if pl.last_sec == 255 {
                    "—".to_string()
                } else {
                    SEC_N[pl.last_sec as usize].to_string()
                }
            ));
        }
        let v = self.viewer.unwrap_or_else(|| self.to_move()).idx();
        let mine: Vec<String> = self.pl[v].cards.iter().map(|c| c.to_string()).collect();
        s.push_str(&format!(
            "Cărțile tale de luptă (J{}): {}\n",
            v + 1,
            if mine.is_empty() {
                "—".into()
            } else {
                mine.join(" ")
            }
        ));
        if !self.over {
            let phase = match self.phase {
                Phase::Sec => "alege secțiunea (s1 Marș, s2 Întărire, s3 Producție, s4 Comerț) și acțiunea de sus",
                Phase::Top => "încă un pas sau «gata»",
                Phase::Bottom => "acțiunea de jos sau «gata»",
                Phase::AttCommit => "luptă: atacatorul declară puterea și cartea (lupta P C)",
                Phase::DefCommit => "luptă: apărătorul declară puterea și cartea",
                Phase::Resolve => "rezolva lupta",
                Phase::Pick => "",
            };
            s.push_str(&format!("Mută J{}: {}.\n", self.side.idx() + 1, phase));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        let g = geo();
        let kc = ['h', 'k', 'w'];
        let step = |k: u8, f: u8, t: u8| {
            format!(
                "{}:{}>{}",
                kc[k as usize], g.name[f as usize], g.name[t as usize]
            )
        };
        match *m {
            Move::Pick(f) => format!("alege {}", FAC[f as usize].cmd),
            Move::Sec(s, t) => {
                let tt = match t {
                    Top::Skip => "sari".to_string(),
                    Top::Step(k, f, to) => format!("mars {}", step(k, f, to)),
                    Top::Coins => "bani".into(),
                    Top::Power => "putere".into(),
                    Top::Cards => "carti".into(),
                    Top::Produce(mask) => {
                        let hs: Vec<&str> = (0..N)
                            .filter(|h| mask & (1 << h) != 0)
                            .map(|h| g.name[h].as_str())
                            .collect();
                        format!("produce {}", hs.join(" "))
                    }
                    Top::Worker => "muncitor".into(),
                    Top::Trade(r) => format!("comert {}", RES_N[r as usize]),
                    Top::Fame => "faima".into(),
                };
                format!("s{} {}", s + 1, tt)
            }
            Move::Step(k, f, t) => format!("mars {}", step(k, f, t)),
            Move::Done => "gata".into(),
            Move::Bottom(b) => match b {
                Bt::Deploy(h) => format!("mobilizeaza {}", g.name[h as usize]),
                Bt::Upgrade(t) => format!("imbunatateste {}", TRACK[t as usize]),
                Bt::Build(k, h) => {
                    format!("construieste {} {}", BLD[k as usize], g.name[h as usize])
                }
                Bt::Enlist(s) => format!("inrolare {}", s + 1),
            },
            Move::Fight(pw, c) => {
                if c == 0 {
                    format!("lupta {pw} -")
                } else {
                    format!("lupta {pw} c{c}")
                }
            }
            Move::Resolve => "rezolva".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup(seed: u64, f0: u8, f1: u8) -> Hotar {
        let mut g = Hotar::with_seed(seed);
        g.apply(&Move::Pick(f0));
        g.apply(&Move::Pick(f1));
        g
    }

    fn sec(g: &mut Hotar, s: u8, t: Top) {
        let m = Move::Sec(s, t);
        assert!(g.legal_moves().contains(&m), "{m:?}\n{}", g.render());
        g.apply(&m);
    }

    #[test]
    fn map_is_symmetric_and_connected() {
        let g = geo();
        assert_eq!(g.name.len(), 37);
        let mut n = g.name.clone();
        n.sort();
        n.dedup();
        assert_eq!(n.len(), 37);
        for i in 0..N {
            assert_eq!(g.terr[i], g.terr[N - 1 - i]);
            assert_eq!(g.dist[i][N - 1 - i], g.dist[N - 1 - i][i]);
        }
        assert_eq!(g.nb[CENTER].len(), 6);
        assert_eq!(g.nb[HOME[0]].len(), 3);
        assert_eq!(g.terr[HOME[0]], b'H');
        assert_eq!(g.terr[HOME[1]], b'H');
        assert_eq!(g.dist[HOME[0]][HOME[1]], 6);
        for i in 0..N {
            for j in &g.nb[i] {
                assert!(g.nb[*j as usize].contains(&(i as u8)));
            }
        }
        assert_eq!(hex_of("a1"), 0);
        assert_eq!(hex_of("g4"), 36);
    }

    #[test]
    fn faction_pick_then_start() {
        let mut g = Hotar::with_seed(1);
        assert_eq!(g.legal_moves().len(), 4);
        g.apply(&Move::Pick(2));
        assert_eq!(g.to_move(), Player::Two);
        assert_eq!(g.legal_moves().len(), 3);
        assert!(!g.legal_moves().contains(&Move::Pick(2)));
        g.apply(&Move::Pick(0));
        assert_eq!(g.to_move(), Player::One);
        assert_eq!(g.pl[0].coins, 6);
        assert_eq!(g.pl[1].coins, 5);
        assert_eq!(g.pl[0].workers.len(), 2);
        assert_eq!(g.pl[0].cards.len(), 2);
        assert_eq!(g.card_total(), 30);
        // lacul nu e vecin cu satul
        for w in &g.pl[0].workers {
            assert_ne!(geo().terr[*w as usize], b'L');
        }
    }

    #[test]
    fn cannot_repeat_the_same_section() {
        let mut g = setup(2, 1, 2);
        sec(&mut g, 1, Top::Power);
        assert_eq!(g.pl[0].power, 3 + 2);
        g.apply(&Move::Done); // fără acțiune de jos
        sec(&mut g, 0, Top::Skip); // J2
        g.apply(&Move::Done);
        assert_eq!(g.to_move(), Player::One);
        assert!(!g.legal_moves().iter().any(|m| matches!(m, Move::Sec(1, _))));
        assert!(g.legal_moves().iter().any(|m| matches!(m, Move::Sec(0, _))));
    }

    #[test]
    fn produce_pay_and_build() {
        let mut g = setup(3, 0, 1);
        let h = *g.pl[0]
            .workers
            .iter()
            .find(|w| Hotar::res_of(**w as usize).is_some())
            .unwrap();
        let r = Hotar::res_of(h as usize).unwrap();
        let n = g.workers_at(0, h as usize) as i32;
        let m = g.legal_moves();
        let mask = 1u64 << h;
        assert!(m.contains(&Move::Sec(2, Top::Produce(mask))));
        g.apply(&Move::Sec(2, Top::Produce(mask)));
        assert_eq!(g.pl[0].res[r], n);
        // jos: construcția cere lemn
        g.pl[0].res[1] = 2;
        let b = g.legal_moves();
        let can_build = b.iter().any(|m| matches!(m, Move::Bottom(Bt::Build(..))));
        assert!(can_build || Hotar::terr(h as usize) == b'H');
        if let Some(Move::Bottom(bt)) = b.iter().find(|m| matches!(m, Move::Bottom(Bt::Build(..))))
        {
            let c0 = g.pl[0].coins;
            g.apply(&Move::Bottom(*bt));
            assert_eq!(g.pl[0].res[1], 0);
            assert_eq!(g.pl[0].bld.len(), 1);
            assert_eq!(g.pl[0].coins, c0 + 2);
        }
    }

    #[test]
    fn produce_limit_follows_upgrades() {
        let mut g = setup(4, 0, 1);
        g.pl[0].workers = vec![hex_of("c3") as u8, hex_of("c5") as u8, hex_of("d2") as u8];
        let hs = g.produce_hexes(0);
        let tops = g.top_moves(0, 2);
        let max_bits = tops
            .iter()
            .filter_map(|t| match t {
                Top::Produce(m) => Some(m.count_ones()),
                _ => None,
            })
            .max()
            .unwrap_or(0);
        assert_eq!(max_bits as usize, hs.len().min(2));
        g.pl[0].up[1] = 2;
        let max_bits = g
            .top_moves(0, 2)
            .iter()
            .filter_map(|t| match t {
                Top::Produce(m) => Some(m.count_ones()),
                _ => None,
            })
            .max()
            .unwrap_or(0);
        assert_eq!(max_bits as usize, hs.len().min(4));
    }

    #[test]
    fn deploy_upgrade_enlist_costs() {
        let mut g = setup(5, 1, 0);
        g.pl[0].res = [5, 5, 5];
        g.cur_sec = 0;
        let b = g.bottom_moves(0, 0);
        assert!(b.contains(&Bt::Deploy(HOME[0] as u8)));
        g.phase = Phase::Bottom;
        g.apply(&Move::Bottom(Bt::Deploy(HOME[0] as u8)));
        assert_eq!(g.pl[0].res[2], 3); // Munteanii: 2 fier
        assert_eq!(g.pl[0].knights.len(), 1);
        // îmbunătățire: cel mult de 2 ori pe pistă
        let mut g = setup(6, 1, 0);
        g.pl[0].res = [9, 0, 0];
        g.pl[0].up[0] = 2;
        let b = g.bottom_moves(0, 1);
        assert!(!b.contains(&Bt::Upgrade(0)));
        assert!(b.contains(&Bt::Upgrade(1)) && b.contains(&Bt::Upgrade(2)));
        // înrolare: un singur loc o dată
        g.pl[0].enl = 0b0010;
        let b = g.bottom_moves(0, 3);
        assert!(!b.contains(&Bt::Enlist(1)) && b.contains(&Bt::Enlist(0)));
        // fără resurse nu se poate
        g.pl[0].res = [0; 3];
        assert!(g.bottom_moves(0, 3).is_empty());
        assert!(g.bottom_moves(0, 0).is_empty());
    }

    #[test]
    fn movement_rules_lake_and_enemy_home() {
        let mut g = setup(7, 1, 0);
        let lake = (0..N).find(|h| geo().terr[*h] == b'L').unwrap();
        let from = geo().nb[lake][0] as usize;
        g.pl[0].hero = from as u8;
        let steps = g.step_moves(0);
        assert!(!steps.iter().any(|s| s.0 == 0 && s.2 as usize == lake));
        let mut g2 = setup(7, 0, 1);
        g2.pl[0].hero = from as u8;
        assert!(g2
            .step_moves(0)
            .iter()
            .any(|s| s.0 == 0 && s.2 as usize == lake));
        // satul adversarului e interzis
        let nbh = geo().nb[HOME[1]][0] as usize;
        g.pl[0].hero = nbh as u8;
        assert!(!g.step_moves(0).iter().any(|s| s.2 as usize == HOME[1]));
    }

    #[test]
    fn step_budget_and_done() {
        let mut g = setup(8, 0, 1);
        let m = g
            .legal_moves()
            .into_iter()
            .find(|m| matches!(m, Move::Sec(0, Top::Step(0, ..))))
            .unwrap();
        g.apply(&m);
        assert_eq!(g.phase, Phase::Top);
        assert_eq!(g.steps_left, 1);
        let m = g
            .legal_moves()
            .into_iter()
            .find(|m| matches!(m, Move::Step(..)))
            .unwrap();
        g.apply(&m);
        assert_eq!(g.phase, Phase::Bottom);
    }

    #[test]
    fn workers_retreat_and_cost_popularity() {
        let mut g = setup(9, 0, 1);
        let t = hex_of("d3");
        g.pl[1].workers = vec![t as u8, t as u8];
        g.pl[0].hero = geo().nb[t][0];
        let pop = g.pl[0].pop;
        g.side = Player::One;
        g.apply(&Move::Sec(0, Top::Step(0, geo().nb[t][0], t as u8)));
        assert_eq!(g.pl[1].workers, vec![HOME[1] as u8; 2]);
        assert_eq!(g.pl[0].pop, pop - 1);
        assert_eq!(g.phase, Phase::Top);
    }

    #[test]
    fn combat_flow_and_tie_goes_to_defender() {
        let mut g = setup(10, 0, 1);
        let t = hex_of("d3");
        g.pl[1].hero = t as u8;
        g.pl[0].hero = geo().nb[t][0];
        g.pl[0].power = 5;
        g.pl[1].power = 5;
        g.pl[0].cards = vec![3];
        g.pl[1].cards = vec![3];
        g.disc.clear();
        let total = g.card_total();
        let from = geo().nb[t][0];
        g.apply(&Move::Sec(0, Top::Step(0, from, t as u8)));
        assert_eq!(g.phase, Phase::AttCommit);
        assert_eq!(g.to_move(), Player::One);
        g.apply(&Move::Fight(2, 3));
        assert_eq!(g.phase, Phase::DefCommit);
        assert_eq!(g.to_move(), Player::Two);
        // putere neschimbată până la rezolvare
        assert_eq!(g.pl[0].power, 5);
        g.apply(&Move::Fight(2, 3)); // egal: 2+2+3 vs 2+2+3 → apărătorul
        assert_eq!(g.phase, Phase::Resolve);
        assert_eq!(g.to_move(), Player::One);
        let pop1 = g.pl[1].pop;
        g.apply(&Move::Resolve);
        assert_eq!(g.pl[0].hero as usize, HOME[0]);
        assert_eq!(g.pl[1].hero as usize, t);
        assert_eq!(g.pl[0].power, 3);
        assert_eq!(g.pl[1].power, 3);
        assert!(g.pl[0].cards.is_empty() && g.pl[1].cards.is_empty());
        assert_eq!(g.pl[1].pop, pop1 + 1);
        assert_eq!(g.pl[1].wins, 1);
        assert_eq!(g.card_total(), total);
        assert_eq!(g.disc.len(), 2);
    }

    #[test]
    fn combat_bonuses() {
        let mut g = setup(11, 1, 0);
        let m = (0..N).find(|h| geo().terr[*h] == b'M').unwrap();
        g.pl[0].hero = m as u8;
        g.pl[0].knights = vec![m as u8, m as u8];
        assert_eq!(g.combat_total(0, m, 1, 4), 1 + 4 + 2 + 2 + 2); // munteanii +2
        g.pl[1].hero = m as u8;
        g.pl[1].bld = vec![(m as u8, 1)];
        assert_eq!(g.combat_total(1, m, 0, 0), 2 + 2);
    }

    #[test]
    fn evaluation_does_not_peek_at_hidden_commitments() {
        let mut g = setup(12, 0, 1);
        let t = hex_of("d3");
        g.pl[1].hero = t as u8;
        g.pl[0].hero = geo().nb[t][0];
        g.pl[0].power = 6;
        g.pl[1].power = 6;
        g.pl[0].cards = vec![5, 1];
        g.pl[1].cards = vec![2, 4];
        let from = geo().nb[t][0];
        g.apply(&Move::Sec(0, Top::Step(0, from, t as u8)));
        let mut a = g.clone();
        let mut b = g.clone();
        a.apply(&Move::Fight(0, 0));
        b.apply(&Move::Fight(6, 5));
        // J2 (apărător) alege; evaluarea lui nu depinde de angajamentul lui J1
        a.apply(&Move::Fight(1, 2));
        b.apply(&Move::Fight(1, 2));
        assert_eq!(a.evaluate(Player::Two), b.evaluate(Player::Two));
        // identitatea cărților adversarului nu contează
        let mut c = a.clone();
        c.pl[0].cards = vec![5, 5, 5];
        c.pl[0].cards.truncate(a.pl[0].cards.len());
        assert_eq!(a.evaluate(Player::Two), c.evaluate(Player::Two));
    }

    #[test]
    fn five_stars_end_the_game() {
        let mut g = setup(13, 0, 1);
        g.pl[0].up = [2, 2, 0];
        g.pl[0].knights = vec![1, 1, 1];
        g.pl[0].bld = vec![(1, 0), (2, 1), (3, 2)];
        g.pl[0].enl = 0b0111;
        g.check_stars();
        assert_eq!(g.pl[0].stars.count_ones(), 4);
        assert!(!g.over);
        g.pl[0].power = 12;
        g.check_stars();
        assert!(g.over);
        assert!(g.outcome().is_some());
    }

    #[test]
    fn scoring_tiers() {
        let mut g = setup(14, 0, 1);
        g.pl[0].coins = 10;
        g.pl[0].pop = 0;
        g.pl[0].stars = 0b11;
        g.pl[0].res = [3, 1, 0];
        let t = g.territories(0) as i32;
        assert_eq!(g.score(0), 10 + 3 * 2 + 2 * t + 2);
        g.pl[0].pop = 13;
        assert_eq!(g.score(0), 10 + 5 * 2 + 4 * t + 3 * 2);
    }

    #[test]
    fn random_games_finish_and_conserve_cards() {
        for seed in 1..40u64 {
            let mut g = Hotar::with_seed(seed);
            let mut rng = Rng::new(seed ^ 99);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty(), "{}", g.render());
                g.apply(&m[rng.below(m.len())]);
                assert_eq!(g.card_total(), 30);
                for p in 0..2 {
                    let pl = &g.pl[p];
                    assert!(pl.coins >= 0 && pl.res.iter().all(|r| *r >= 0));
                    assert!((0..=16).contains(&pl.power) && (0..=18).contains(&pl.pop));
                    assert!(pl.knights.len() <= 4 && pl.workers.len() <= 6);
                }
                n += 1;
                assert!(n < 6000);
            }
        }
    }

    #[test]
    fn ai_beats_random() {
        let mut wins = 0;
        let mut lost = 0;
        for seed in 1..=16u64 {
            let mut g = Hotar::with_seed(seed);
            let mut rng = Rng::new(seed + 7);
            let mut n = 0;
            while g.outcome().is_none() && n < 3000 {
                let m = if g.to_move() == Player::One {
                    polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap()
                } else {
                    let l = g.legal_moves();
                    l[rng.below(l.len())]
                };
                g.apply(&m);
                n += 1;
            }
            assert!(g.outcome().is_some());
            match g.outcome() {
                Some(Outcome::Win(Player::One)) => wins += 1,
                Some(Outcome::Win(Player::Two)) => lost += 1,
                _ => {}
            }
        }
        assert!(wins >= 13 && lost <= 2, "wins {wins} lost {lost}");
    }

    #[test]
    fn ai_vs_ai_is_real_and_fast() {
        let t = std::time::Instant::now();
        let mut plies = 0;
        let mut fights = 0;
        for seed in 1..=4u64 {
            let mut g = Hotar::with_seed(seed);
            let mut rng = Rng::new(seed);
            while g.outcome().is_none() {
                let m = polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap();
                if m == Move::Resolve {
                    fights += 1;
                }
                if std::env::var("HOTAR_DBG").is_ok() && plies % 40 == 0 {
                    eprintln!("{}", g.render());
                }
                g.apply(&m);
                plies += 1;
            }
            eprintln!(
                "seed {seed}: ture {:?}, stele {} / {}, scor {} / {}",
                g.turns,
                g.pl[0].stars.count_ones(),
                g.pl[1].stars.count_ones(),
                g.score(0),
                g.score(1)
            );
        }
        eprintln!("lupte {fights}");
        eprintln!(
            "ms/mutare {}",
            t.elapsed().as_millis() as f64 / plies as f64
        );
    }

    #[test]
    fn render_hides_the_opponents_cards() {
        let mut g = setup(15, 0, 1);
        g.pl[1].cards = vec![4, 4];
        g.pl[0].cards = vec![5];
        let r = g.render();
        assert!(r.contains("Cărțile tale de luptă (J1): 5"));
        assert!(!r.contains("4 4"));
        assert!(r.contains("a1:") && r.contains("g4:"));
    }
}
