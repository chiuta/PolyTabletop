//! Motor comun pentru Mahjong Sichuan și Mahjong oficial chinez (Guobiao), versiuni pentru 2 jucători.
//!
//! * **Sichuan** (`Variant::Sichuan`): 108 piese (trei culori × 9 × 4; fără onoruri și fără flori). Fiecare
//!   jucător declară la început **culoarea lipsă** (`lipsa m|s|p`) și trebuie să arunce întâi piesele din ea;
//!   o mână cu piese din culoarea lipsă nu poate câștiga. Nu se poate lua o piesă pentru secvență (fără chi);
//!   pung, kong și câștig sunt permise. Mână câștigătoare: 4 seturi + o pereche sau șapte perechi; fără minimum de
//!   fani (se afișează totuși: tragere proprie +1, toate pungurile +1, o singură culoare +2, șapte perechi +2).
//!   În original jocul continuă după primul câștig („bătălia sângeroasă”); aici se încheie la primul câștig.
//! * **Guobiao** (`Variant::Guobiao`): 144 de piese (cu 8 flori, înlocuite automat din zidul mort). Mâna
//!   câștigătoare trebuie să aibă **cel puțin 8 puncte** fără flori (fiecare floare +1 peste). Implementăm un
//!   subset documentat al celor 81 de combinații oficiale: Patru vânturi mari 88, Trei dragoni mari 88, Doar
//!   onoruri 64, Doar capete 64, Patru pungs ascunse 64, Trei dragoni mici 64, Capete și onoruri 32, Șapte perechi
//!   24, Culoare întreagă 24, Pungs pare 24, Scară pură 16, Trei pungs ascunse 16, Toate cincii 16, Triplu pung 16,
//!   Jumătate de culoare 6, Toate pungurile 6, Scară mixtă 8, Triplă scară mixtă 8, Pung dragon 2, Vânt dominant 2,
//!   Vânt propriu 2, Toate secvențele 2, Mână ascunsă 2 (4 dacă și din tragere), Toate simple 2, Tragere proprie 1,
//!   Kong expus 1, Kong ascuns 2, Fără onoruri 1, Pung de capete/onoruri 1, Mână găină 8 (nicio altă combinație).
//!   Restul combinațiilor oficiale (șiruri duble, nouă porți etc.) nu sunt implementate.
//! * Ambele: tragi (`trage`), arunci (`d5m`) sau declari `mahjong`; adversarul poate lua piesa aruncată (`pung`,
//!   `kong`, în Guobiao și `chi2m3m`, apoi arunci) sau `mahjong`. Zidul e ascuns; mână remiză la zid terminat.
//!
//! Mutări: `lipsa m`, `trage`, `trage-moarta`, `d5m`, `mahjong`, `kong5m`, `pung`, `kong`, `chi2m3m`, `pas`.
#![allow(clippy::manual_is_multiple_of)]

use crate::cards52::shuffle;
use crate::mahjong::tile_name;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Variant {
    Sichuan,
    Guobiao,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Meld {
    Chi(u8),
    Pung(u8),
    /// (piesa, ascuns)
    Kong(u8, bool),
}

impl Meld {
    fn tiles(self) -> Vec<u8> {
        match self {
            Meld::Chi(a) => vec![a, a + 1, a + 2],
            Meld::Pung(t) => vec![t; 3],
            Meld::Kong(t, _) => vec![t; 4],
        }
    }
    fn text(self) -> String {
        self.tiles()
            .iter()
            .map(|&t| tile_name(t))
            .collect::<Vec<_>>()
            .join("")
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Declare(u8),
    Draw,
    DrawDead,
    Discard(u8),
    Tsumo,
    Ron,
    KongSelf(u8),
    Pass,
    Pung,
    KongClaim,
    Chi(u8, u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Declare,
    Draw,
    DrawDead,
    Discard,
    React,
    Over,
}

type Counts = [u8; 34];

/// Toate descompunerile (pereche, seturi) ale pieselor ascunse; seturi = (este_pung, piesa).
fn decomps(c: &mut Counts, need: usize) -> Vec<(u8, Vec<(bool, u8)>)> {
    fn rec(
        c: &mut Counts,
        need: usize,
        pair: Option<u8>,
        sets: &mut Vec<(bool, u8)>,
        out: &mut Vec<(u8, Vec<(bool, u8)>)>,
    ) {
        let Some(i) = (0..34).find(|&i| c[i] > 0) else {
            if need == 0 {
                if let Some(p) = pair {
                    out.push((p, sets.clone()));
                }
            }
            return;
        };
        if pair.is_none() && c[i] >= 2 {
            c[i] -= 2;
            rec(c, need, Some(i as u8), sets, out);
            c[i] += 2;
        }
        if need > 0 && c[i] >= 3 {
            c[i] -= 3;
            sets.push((true, i as u8));
            rec(c, need - 1, pair, sets, out);
            sets.pop();
            c[i] += 3;
        }
        if need > 0 && i < 27 && i % 9 <= 6 && c[i + 1] > 0 && c[i + 2] > 0 {
            c[i] -= 1;
            c[i + 1] -= 1;
            c[i + 2] -= 1;
            sets.push((false, i as u8));
            rec(c, need - 1, pair, sets, out);
            sets.pop();
            c[i] += 1;
            c[i + 1] += 1;
            c[i + 2] += 1;
        }
    }
    let mut out = Vec::new();
    rec(c, need, None, &mut Vec::new(), &mut out);
    out
}

fn is_honor(t: u8) -> bool {
    t >= 27
}
fn is_terminal(t: u8) -> bool {
    t < 27 && (t % 9 == 0 || t % 9 == 8)
}
fn is_simple(t: u8) -> bool {
    t < 27 && !is_terminal(t)
}

#[derive(Clone, Copy)]
struct SetInfo {
    chow: bool,
    t: u8,
    exposed: bool,
    kong: bool,
    concealed_kong: bool,
}

fn seven_pairs(c: &Counts, melds: &[Meld]) -> bool {
    melds.is_empty()
        && c.iter().filter(|&&x| x == 2).count() == 7
        && c.iter().all(|&x| x == 0 || x == 2)
}

/// Fanii și descrierea celei mai bune lecturi; `None` dacă mâna nu e completă.
pub fn score_hand(
    v: Variant,
    counts: &Counts,
    melds: &[Meld],
    seat_wind: u8,
    self_draw: bool,
) -> Option<(i32, String)> {
    let need = 4usize.checked_sub(melds.len())?;
    let total: u32 = counts.iter().map(|&x| x as u32).sum();
    if total != 3 * need as u32 + 2 {
        return None;
    }
    let mut best: Option<(i32, String)> = None;
    let mut consider = |fan: i32, desc: String| {
        if best.as_ref().is_none_or(|b| fan > b.0) {
            best = Some((fan, desc));
        }
    };
    if seven_pairs(counts, melds) {
        let mut tiles: Vec<u8> = Vec::new();
        for (i, &n) in counts.iter().enumerate() {
            if n > 0 {
                tiles.push(i as u8);
            }
        }
        match v {
            Variant::Sichuan => {
                let mut f = 2;
                let mut d = vec!["șapte perechi 2".to_string()];
                if self_draw {
                    f += 1;
                    d.push("tragere proprie 1".into());
                }
                consider(f, d.join(", "));
            }
            Variant::Guobiao => {
                let mut f = 24;
                let mut d = vec!["șapte perechi 24".to_string()];
                let suits: Vec<u8> = tiles.iter().filter(|&&t| t < 27).map(|&t| t / 9).collect();
                let one_suit = !suits.is_empty() && suits.iter().all(|&s| s == suits[0]);
                if one_suit && tiles.iter().all(|&t| t < 27) {
                    // culoare întreagă (excludă șapte perechi în regulile reale: păstrăm cel mai mare)
                    f = f.max(24);
                    d.push("o singură culoare".into());
                }
                if self_draw {
                    f += 1;
                    d.push("tragere proprie 1".into());
                }
                consider(f, d.join(", "));
            }
        }
    }
    let mut c = *counts;
    for (pair, sets) in decomps(&mut c, need) {
        let mut all: Vec<SetInfo> = sets
            .iter()
            .map(|&(p, t)| SetInfo {
                chow: !p,
                t,
                exposed: false,
                kong: false,
                concealed_kong: false,
            })
            .collect();
        for m in melds {
            match *m {
                Meld::Chi(a) => all.push(SetInfo {
                    chow: true,
                    t: a,
                    exposed: true,
                    kong: false,
                    concealed_kong: false,
                }),
                Meld::Pung(t) => all.push(SetInfo {
                    chow: false,
                    t,
                    exposed: true,
                    kong: false,
                    concealed_kong: false,
                }),
                Meld::Kong(t, hidden) => all.push(SetInfo {
                    chow: false,
                    t,
                    exposed: !hidden,
                    kong: true,
                    concealed_kong: hidden,
                }),
            }
        }
        let (f, d) = match v {
            Variant::Sichuan => score_sichuan(&all, pair, self_draw),
            Variant::Guobiao => score_guobiao(&all, pair, self_draw, seat_wind),
        };
        consider(f, d);
    }
    best
}

fn set_tiles(s: &SetInfo) -> Vec<u8> {
    if s.chow {
        vec![s.t, s.t + 1, s.t + 2]
    } else if s.kong {
        vec![s.t; 4]
    } else {
        vec![s.t; 3]
    }
}

fn score_sichuan(all: &[SetInfo], pair: u8, self_draw: bool) -> (i32, String) {
    let mut fan = 1;
    let mut d = vec!["mână completă 1".to_string()];
    if self_draw {
        fan += 1;
        d.push("tragere proprie 1".into());
    }
    if all.iter().all(|s| !s.chow) {
        fan += 1;
        d.push("toate pungurile 1".into());
    }
    let mut suits = [false; 3];
    for s in all {
        for t in set_tiles(s) {
            suits[(t / 9) as usize] = true;
        }
    }
    suits[(pair / 9) as usize] = true;
    if suits.iter().filter(|&&x| x).count() == 1 {
        fan += 2;
        d.push("o singură culoare 2".into());
    }
    let k = all.iter().filter(|s| s.kong).count() as i32;
    if k > 0 {
        fan += k;
        d.push(format!("kong {k}"));
    }
    (fan, d.join(", "))
}

fn score_guobiao(all: &[SetInfo], pair: u8, self_draw: bool, seat_wind: u8) -> (i32, String) {
    let mut items: Vec<(i32, String)> = Vec::new();
    let mut add = |p: i32, s: &str| items.push((p, format!("{s} {p}")));
    let mut tiles: Vec<u8> = vec![pair, pair];
    for s in all {
        tiles.extend(set_tiles(s));
    }
    let pungs: Vec<&SetInfo> = all.iter().filter(|s| !s.chow).collect();
    let chows: Vec<&SetInfo> = all.iter().filter(|s| s.chow).collect();
    let closed = all.iter().all(|s| !s.exposed);
    let all_honors = tiles.iter().all(|&t| is_honor(t));
    let all_terminals = tiles.iter().all(|&t| is_terminal(t));
    let all_th = tiles.iter().all(|&t| is_honor(t) || is_terminal(t));
    let winds_pung = (27..=30)
        .filter(|&w| pungs.iter().any(|s| s.t == w))
        .count();
    let dragon_pungs = (31..=33)
        .filter(|&w| pungs.iter().any(|s| s.t == w))
        .count();
    let mut suits = [false; 3];
    let mut honors = false;
    for &t in &tiles {
        if t < 27 {
            suits[(t / 9) as usize] = true;
        } else {
            honors = true;
        }
    }
    let n_suits = suits.iter().filter(|&&x| x).count();
    let concealed_pungs = pungs.iter().filter(|s| !s.exposed).count();
    let mut big = false;
    if winds_pung == 4 {
        add(88, "patru vânturi mari");
        big = true;
    }
    if dragon_pungs == 3 {
        add(88, "trei dragoni mari");
        big = true;
    }
    if all_honors {
        add(64, "doar onoruri");
        big = true;
    }
    if all_terminals {
        add(64, "doar capete");
        big = true;
    }
    if pungs.len() == 4 && concealed_pungs == 4 {
        add(64, "patru pungs ascunse");
        big = true;
    }
    if dragon_pungs == 2 && (31..=33).contains(&pair) {
        add(64, "trei dragoni mici");
        big = true;
    }
    if all_th && !all_honors && !all_terminals {
        add(32, "capete și onoruri");
    }
    let even_pungs = pungs.len() == 4 && tiles.iter().all(|&t| t < 27 && t % 2 == 1) && true;
    // piese pare = 2,4,6,8 (indice 1,3,5,7 în culoare)
    let all_even = tiles.iter().all(|&t| t < 27 && t % 9 % 2 == 1);
    if even_pungs && all_even {
        add(24, "pungs pare");
    }
    if n_suits == 1 && !honors {
        add(24, "culoare întreagă");
    } else if n_suits == 1 && honors && !big {
        add(6, "jumătate de culoare");
    }
    let mut straight = false;
    for s in 0..3u8 {
        let has = |a: u8| chows.iter().any(|c| c.t == s * 9 + a);
        if has(0) && has(3) && has(6) {
            straight = true;
        }
    }
    if straight {
        add(16, "scară pură");
    }
    if concealed_pungs >= 3 && !(pungs.len() == 4 && concealed_pungs == 4) {
        add(16, "trei pungs ascunse");
    }
    if tiles.iter().all(|&t| t < 27 && t % 9 == 4)
        || (all
            .iter()
            .all(|s| set_tiles(s).iter().any(|&t| t < 27 && t % 9 == 4))
            && pair < 27
            && pair % 9 == 4)
    {
        add(16, "toate cincii");
    }
    for a in 0..9u8 {
        if (0..3u8).all(|s| pungs.iter().any(|p| p.t == s * 9 + a)) {
            add(16, "triplu pung");
            break;
        }
    }
    if pungs.len() == 4 && !big && !even_pungs {
        add(6, "toate pungurile");
    }
    // scară mixtă: 123/456/789 în trei culori diferite
    {
        let mut mixed = false;
        for perm in [
            [0u8, 1, 2],
            [0, 2, 1],
            [1, 0, 2],
            [1, 2, 0],
            [2, 0, 1],
            [2, 1, 0],
        ] {
            if chows.iter().any(|c| c.t == perm[0] * 9)
                && chows.iter().any(|c| c.t == perm[1] * 9 + 3)
                && chows.iter().any(|c| c.t == perm[2] * 9 + 6)
            {
                mixed = true;
            }
        }
        if mixed && !straight {
            add(8, "scară mixtă");
        }
    }
    for a in 0..7u8 {
        if (0..3u8).all(|s| chows.iter().any(|c| c.t == s * 9 + a)) {
            add(8, "triplă secvență mixtă");
            break;
        }
    }
    if dragon_pungs < 3 && !(dragon_pungs == 2 && (31..=33).contains(&pair)) {
        for _ in 0..dragon_pungs {
            add(2, "pung de dragon");
        }
    }
    if winds_pung < 4 {
        if pungs.iter().any(|s| s.t == 27) {
            add(2, "vânt dominant");
        }
        if pungs.iter().any(|s| s.t == seat_wind) {
            add(2, "vânt propriu");
        }
    }
    if chows.len() == 4 && !is_honor(pair) {
        add(2, "toate secvențele");
    }
    if closed && self_draw {
        add(4, "mână complet ascunsă");
    } else {
        if closed {
            add(2, "mână ascunsă");
        }
        if self_draw {
            add(1, "tragere proprie");
        }
    }
    if tiles.iter().all(|&t| is_simple(t)) {
        add(2, "toate simple");
    } else if !honors && !all_terminals {
        add(1, "fără onoruri");
    }
    for s in all {
        if s.kong {
            if s.concealed_kong {
                add(2, "kong ascuns");
            } else {
                add(1, "kong expus");
            }
        }
    }
    for s in &pungs {
        if (is_terminal(s.t) || (is_honor(s.t) && s.t < 31 && s.t != 27 && s.t != seat_wind))
            && !big
            && !all_th
        {
            add(1, "pung de capete/onoruri");
        }
    }
    let fan: i32 = items.iter().map(|x| x.0).sum();
    if fan == 0 {
        return (8, "mână găină 8".into());
    }
    (
        fan,
        items
            .iter()
            .map(|x| x.1.clone())
            .collect::<Vec<_>>()
            .join(", "),
    )
}

/// Scor de potențial (doar din propria mână) pentru AI.
fn potential(c: &Counts, melds: &[Meld], missing: Option<u8>) -> i32 {
    let mut s = 0.0f64;
    for (i, &n) in c.iter().enumerate() {
        let n = n as f64;
        let mut v = match n as u32 {
            0 | 1 => 0.0,
            2 => 6.0,
            3 => 13.0,
            _ => 11.0,
        };
        if i >= 31 && n >= 2.0 {
            v += 3.0;
        }
        if i < 27 {
            let pos = i % 9;
            if n >= 1.0 && pos <= 7 && c[i + 1] > 0 {
                v += 3.0;
            }
            if n >= 1.0 && pos <= 6 && c[i + 2] > 0 {
                v += 1.5;
                if c[i + 1] > 0 {
                    v += 8.0;
                }
            }
            if missing == Some((i / 9) as u8) {
                v = -4.0 * n;
            }
        }
        s += v;
    }
    let mut per = [0u32; 3];
    for i in 0..27 {
        per[i / 9] += c[i] as u32;
    }
    s += 0.7 * *per.iter().max().unwrap() as f64;
    s += 16.0 * melds.len() as f64;
    (s * 10.0) as i32
}

#[derive(Clone)]
pub struct Mj2 {
    v: Variant,
    counts: [Counts; 2],
    melds: [Vec<Meld>; 2],
    discards: [Vec<u8>; 2],
    wall: Vec<u8>,
    dead: Vec<u8>,
    side: Player,
    phase: Phase,
    last: Option<(u8, Player)>,
    drawn: Option<u8>,
    winner: Option<(Player, i32)>,
    flowers: [u8; 2],
    missing: [Option<u8>; 2],
    msg: String,
    viewer: Option<Player>,
}

impl Mj2 {
    pub fn new(v: Variant) -> Self {
        Self::with_seed(v, Rng::from_clock().next_u64())
    }

    pub fn with_seed(v: Variant, seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut w: Vec<u8> = Vec::new();
        let kinds = if v == Variant::Sichuan { 27 } else { 34 };
        for t in 0..kinds as u8 {
            w.extend([t; 4]);
        }
        if v == Variant::Guobiao {
            w.extend(34..42u8);
        }
        shuffle(&mut w, &mut rng);
        let dead_n = if v == Variant::Sichuan { 8 } else { 14 };
        let dead: Vec<u8> = w.drain(..dead_n).collect();
        let mut g = Mj2 {
            v,
            counts: [[0; 34]; 2],
            melds: [Vec::new(), Vec::new()],
            discards: [Vec::new(), Vec::new()],
            wall: w,
            dead,
            side: Player::One,
            phase: if v == Variant::Sichuan {
                Phase::Declare
            } else {
                Phase::Draw
            },
            last: None,
            drawn: None,
            winner: None,
            flowers: [0, 0],
            missing: [None, None],
            msg: String::new(),
            viewer: Some(Player::One),
        };
        for p in 0..2 {
            for _ in 0..13 {
                g.take_tile(p, false);
            }
        }
        g
    }

    /// Ia o piesă din zid (sau din zidul mort); florile se pun deoparte și se înlocuiesc.
    fn take_tile(&mut self, p: usize, dead: bool) -> u8 {
        loop {
            let t = if dead {
                self.dead.pop().or_else(|| self.wall.pop())
            } else {
                self.wall.pop().or_else(|| self.dead.pop())
            }
            .expect("zid");
            if t >= 34 {
                self.flowers[p] += 1;
                continue;
            }
            self.counts[p][t as usize] += 1;
            return t;
        }
    }

    pub fn wall_len(&self) -> usize {
        self.wall.len()
    }

    pub fn winner(&self) -> Option<(Player, i32)> {
        self.winner
    }

    fn seat(p: Player) -> u8 {
        27 + p.idx() as u8
    }

    fn missing_ok(&self, p: usize, c: &Counts, melds: &[Meld]) -> bool {
        match self.missing[p] {
            None => true,
            Some(s) => {
                (0..9).all(|k| c[s as usize * 9 + k] == 0)
                    && melds.iter().all(|m| m.tiles().iter().all(|&t| t / 9 != s))
            }
        }
    }

    fn can_win(&self, p: usize, c: &Counts, self_draw: bool) -> Option<(i32, String)> {
        if !self.missing_ok(p, c, &self.melds[p]) {
            return None;
        }
        let (fan, d) = score_hand(
            self.v,
            c,
            &self.melds[p],
            Self::seat(if p == 0 { Player::One } else { Player::Two }),
            self_draw,
        )?;
        let min = if self.v == Variant::Guobiao { 8 } else { 0 };
        if fan >= min {
            let fl = if self.v == Variant::Guobiao {
                self.flowers[p] as i32
            } else {
                0
            };
            Some((
                fan + fl,
                if fl > 0 {
                    format!("{d}, flori {fl}")
                } else {
                    d
                },
            ))
        } else {
            None
        }
    }

    fn reactions(&self, r: Player, t: u8) -> Vec<Move> {
        let c = &self.counts[r.idx()];
        let mut v = Vec::new();
        let mut cc = *c;
        cc[t as usize] += 1;
        if self.can_win(r.idx(), &cc, false).is_some() {
            v.push(Move::Ron);
        }
        if c[t as usize] >= 2 && self.missing[r.idx()] != Some(t / 9) {
            v.push(Move::Pung);
            if c[t as usize] >= 3 && !self.dead.is_empty() {
                v.push(Move::KongClaim);
            }
        }
        if self.v == Variant::Guobiao && t < 27 {
            let pos = t % 9;
            let has = |x: u8| c[x as usize] > 0;
            if pos >= 2 && has(t - 2) && has(t - 1) {
                v.push(Move::Chi(t - 2, t - 1));
            }
            if (1..=7).contains(&pos) && has(t - 1) && has(t + 1) {
                v.push(Move::Chi(t - 1, t + 1));
            }
            if pos <= 6 && has(t + 1) && has(t + 2) {
                v.push(Move::Chi(t + 1, t + 2));
            }
        }
        if !v.is_empty() {
            v.push(Move::Pass);
        }
        v
    }

    fn begin_draw(&mut self, p: Player) {
        if self.wall.is_empty() {
            self.phase = Phase::Over;
            self.msg = "Zidul s-a terminat: mână remiză.".into();
        } else {
            self.side = p;
            self.phase = Phase::Draw;
        }
    }

    fn win(&mut self, p: Player, fan: i32, desc: &str, how: &str) {
        self.winner = Some((p, fan));
        self.msg = format!("J{} câștigă ({how}) cu {fan} puncte: {desc}", p.idx() + 1);
        self.phase = Phase::Over;
    }

    fn best_potential(&self, p: Player) -> i32 {
        let c = &self.counts[p.idx()];
        let m = &self.melds[p.idx()];
        let n: u32 = c.iter().map(|&x| x as u32).sum();
        let miss = self.missing[p.idx()];
        if n % 3 == 2 {
            let mut best = i32::MIN;
            for t in 0..34 {
                if c[t] > 0 {
                    let mut cc = *c;
                    cc[t] -= 1;
                    best = best.max(potential(&cc, m, miss));
                }
            }
            best
        } else {
            potential(c, m, miss)
        }
    }

    fn discard_choices(&self, p: usize) -> Vec<u8> {
        let c = &self.counts[p];
        let all: Vec<u8> = (0..34u8).filter(|&t| c[t as usize] > 0).collect();
        if let Some(s) = self.missing[p] {
            let m: Vec<u8> = all
                .iter()
                .copied()
                .filter(|&t| t < 27 && t / 9 == s)
                .collect();
            if !m.is_empty() {
                return m;
            }
        }
        all
    }
}

impl Game for Mj2 {
    type Move = Move;

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        let me = self.side.idx();
        match self.phase {
            Phase::Over => Vec::new(),
            Phase::Declare => (0..3).map(Move::Declare).collect(),
            Phase::Draw => vec![Move::Draw],
            Phase::DrawDead => vec![Move::DrawDead],
            Phase::React => {
                let (t, _) = self.last.unwrap();
                self.reactions(self.side, t)
            }
            Phase::Discard => {
                let mut v = Vec::new();
                if self.can_win(me, &self.counts[me], true).is_some() {
                    v.push(Move::Tsumo);
                }
                let c = &self.counts[me];
                for t in self.discard_choices(me) {
                    v.push(Move::Discard(t));
                }
                for t in 0..34u8 {
                    if !self.dead.is_empty()
                        && (c[t as usize] == 4
                            || (c[t as usize] >= 1 && self.melds[me].contains(&Meld::Pung(t))))
                    {
                        v.push(Move::KongSelf(t));
                    }
                }
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        let me = self.side;
        let i = me.idx();
        self.msg.clear();
        match *m {
            Move::Declare(s) => {
                self.missing[i] = Some(s);
                if me == Player::One {
                    self.side = Player::Two;
                } else {
                    self.side = Player::One;
                    self.phase = Phase::Draw;
                }
            }
            Move::Draw => {
                if self.wall.is_empty() {
                    self.phase = Phase::Over;
                    self.msg = "Zidul s-a terminat: mână remiză.".into();
                    return;
                }
                let t = self.take_tile(i, false);
                self.drawn = Some(t);
                self.phase = Phase::Discard;
            }
            Move::DrawDead => {
                let t = self.take_tile(i, true);
                self.drawn = Some(t);
                self.phase = Phase::Discard;
            }
            Move::Discard(t) => {
                self.counts[i][t as usize] -= 1;
                self.discards[i].push(t);
                self.last = Some((t, me));
                self.drawn = None;
                let r = me.other();
                if self.reactions(r, t).is_empty() {
                    if self.wall.is_empty() {
                        // sfârșitul zidului nu depinde de mâna adversarului (altfel căutarea ar vedea dacă
                        // poate reacționa): trecerea la „trage” a adversarului constată remiza
                        self.side = r;
                        self.phase = Phase::Draw;
                    } else {
                        self.begin_draw(r);
                    }
                } else {
                    self.side = r;
                    self.phase = Phase::React;
                }
            }
            Move::Tsumo => {
                let (fan, d) = self.can_win(i, &self.counts[i], true).unwrap();
                self.win(me, fan, &d, "tragere proprie");
            }
            Move::Ron => {
                let (t, from) = self.last.unwrap();
                self.counts[i][t as usize] += 1;
                self.discards[from.idx()].pop();
                let (fan, d) = self.can_win(i, &self.counts[i], false).unwrap();
                self.win(me, fan, &d, "pe aruncarea adversarului");
            }
            Move::KongSelf(t) => {
                if self.melds[i].contains(&Meld::Pung(t)) {
                    let pos = self.melds[i]
                        .iter()
                        .position(|&x| x == Meld::Pung(t))
                        .unwrap();
                    self.melds[i][pos] = Meld::Kong(t, false);
                    self.counts[i][t as usize] -= 1;
                } else {
                    self.counts[i][t as usize] -= 4;
                    self.melds[i].push(Meld::Kong(t, true));
                }
                self.phase = Phase::DrawDead;
            }
            Move::Pass => {
                self.begin_draw(me);
            }
            Move::Pung | Move::KongClaim | Move::Chi(_, _) => {
                let (t, from) = self.last.unwrap();
                self.discards[from.idx()].pop();
                match *m {
                    Move::Pung => {
                        self.counts[i][t as usize] -= 2;
                        self.melds[i].push(Meld::Pung(t));
                        self.phase = Phase::Discard;
                    }
                    Move::KongClaim => {
                        self.counts[i][t as usize] -= 3;
                        self.melds[i].push(Meld::Kong(t, false));
                        self.phase = Phase::DrawDead;
                    }
                    Move::Chi(a, b) => {
                        self.counts[i][a as usize] -= 1;
                        self.counts[i][b as usize] -= 1;
                        self.melds[i].push(Meld::Chi(t.min(a).min(b)));
                        self.phase = Phase::Discard;
                    }
                    _ => {}
                }
                self.drawn = None;
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        Some(match self.winner {
            Some((p, _)) => Outcome::Win(p),
            None => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let mut s = self.best_potential(p);
        let o = p.other();
        let mut bonus = 0;
        for &t in &self.discards[o.idx()] {
            if self.counts[p.idx()][t as usize] >= 2 {
                bonus -= 5;
            }
        }
        s += bonus;
        s -= 8 * self.melds[o.idx()].len() as i32;
        s
    }

    fn render(&self) -> String {
        let title = if self.v == Variant::Sichuan {
            "Mahjong Sichuan"
        } else {
            "Mahjong oficial (Guobiao)"
        };
        let mut s = format!(
            "{title} — zid viu: {} piese (zid mort: {}) | vânt: J1 Est, J2 Sud\n",
            self.wall.len(),
            self.dead.len()
        );
        if !self.msg.is_empty() {
            s.push_str(&format!("{}\n", self.msg));
        }
        for p in [Player::One, Player::Two] {
            let melds: Vec<String> = self.melds[p.idx()].iter().map(|m| m.text()).collect();
            let disc: Vec<String> = self.discards[p.idx()]
                .iter()
                .map(|&t| tile_name(t))
                .collect();
            let extra = match (self.v, self.missing[p.idx()]) {
                (Variant::Sichuan, Some(m)) => {
                    format!(" | culoare lipsă: {}", ['m', 's', 'p'][m as usize])
                }
                (Variant::Guobiao, _) => format!(" | flori: {}", self.flowers[p.idx()]),
                _ => String::new(),
            };
            s.push_str(&format!(
                "J{}: seturi expuse [{}]; aruncate: {}{}\n",
                p.idx() + 1,
                melds.join(" "),
                if disc.is_empty() {
                    "—".to_string()
                } else {
                    disc.join(" ")
                },
                extra
            ));
        }
        if self.phase == Phase::Over {
            s.push_str("Mână încheiată.\n");
            return s;
        }
        let v = self.viewer.unwrap_or(self.side);
        let mut hand = Vec::new();
        for t in 0..34u8 {
            for _ in 0..self.counts[v.idx()][t as usize] {
                hand.push(tile_name(t));
            }
        }
        s.push_str(&format!("Mâna ta (J{}): {}\n", v.idx() + 1, hand.join(" ")));
        s.push_str(&format!(
            "J{}: {} piese ascunse\n",
            v.other().idx() + 1,
            self.counts[v.other().idx()]
                .iter()
                .map(|&x| x as u32)
                .sum::<u32>()
        ));
        if let Some((t, f)) = self.last {
            if self.phase == Phase::React {
                s.push_str(&format!("Aruncată de J{}: {}\n", f.idx() + 1, tile_name(t)));
            }
        }
        if let (Some(d), true) = (self.drawn, self.side == v) {
            s.push_str(&format!("Ai tras: {}\n", tile_name(d)));
        }
        if self.phase == Phase::Declare {
            s.push_str("Alege culoarea lipsă: «lipsa m» (caractere), «lipsa s» (bambus) sau «lipsa p» (cercuri).\n");
        }
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Declare(s) => format!("lipsa {}", ['m', 's', 'p'][*s as usize]),
            Move::Draw => "trage".into(),
            Move::DrawDead => "trage-moarta".into(),
            Move::Discard(t) => format!("d{}", tile_name(*t)),
            Move::Tsumo | Move::Ron => "mahjong".into(),
            Move::KongSelf(t) => format!("kong{}", tile_name(*t)),
            Move::Pass => "pas".into(),
            Move::Pung => "pung".into(),
            Move::KongClaim => "kong".into(),
            Move::Chi(a, b) => format!("chi{}{}", tile_name(*a), tile_name(*b)),
        }
    }

    /// Adâncime 0 (o singură mutare): `AlphaBeta(1)` ar fi extins la Hard/Expert reacțiile adversarului
    /// din mâna lui reală și ar fi tras din zidul real; evaluarea citește doar mâna proprie și datele publice.
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn counts_of(s: &[&str]) -> Counts {
        let mut c = [0u8; 34];
        for t in s {
            let i = (0..34u8).find(|&x| tile_name(x) == *t).unwrap();
            c[i as usize] += 1;
        }
        c
    }

    #[test]
    fn sichuan_deal_sizes() {
        let g = Mj2::with_seed(Variant::Sichuan, 3);
        assert_eq!(g.dead.len(), 8);
        assert_eq!(g.wall.len(), 108 - 8 - 26);
        assert_eq!(g.phase, Phase::Declare);
    }

    #[test]
    fn guobiao_flowers_are_replaced() {
        for seed in 1..=20u64 {
            let g = Mj2::with_seed(Variant::Guobiao, seed);
            for p in 0..2 {
                assert_eq!(g.counts[p].iter().map(|&x| x as usize).sum::<usize>(), 13);
            }
            assert_eq!(
                g.wall.len()
                    + g.dead.len()
                    + 26
                    + g.flowers.iter().map(|&x| x as usize).sum::<usize>(),
                144
            );
        }
    }

    #[test]
    fn sichuan_declare_and_missing_suit_forced_discard() {
        let mut g = Mj2::with_seed(Variant::Sichuan, 9);
        g.apply(&Move::Declare(0));
        g.apply(&Move::Declare(1));
        assert_eq!(g.phase, Phase::Draw);
        g.apply(&Move::Draw);
        let has_missing = (0..9).any(|k| g.counts[0][k] > 0);
        let disc: Vec<u8> = g
            .legal_moves()
            .iter()
            .filter_map(|m| {
                if let Move::Discard(t) = m {
                    Some(*t)
                } else {
                    None
                }
            })
            .collect();
        if has_missing {
            assert!(disc.iter().all(|&t| t < 9));
        }
    }

    #[test]
    fn sichuan_missing_suit_blocks_win() {
        let g = {
            let mut g = Mj2::with_seed(Variant::Sichuan, 1);
            g.missing = [Some(0), None];
            g
        };
        // 1m1m1m 2s3s4s 5s6s7s 1p2p3p 9p9p -> conține culoarea m (0) -> nu poate câștiga
        let c = counts_of(&[
            "1m", "1m", "1m", "2s", "3s", "4s", "5s", "6s", "7s", "1p", "2p", "3p", "9p", "9p",
        ]);
        assert!(g.can_win(0, &c, true).is_none());
        let g2 = {
            let mut g = Mj2::with_seed(Variant::Sichuan, 1);
            g.missing = [Some(2), None];
            g
        };
        let c = counts_of(&[
            "1m", "1m", "1m", "2s", "3s", "4s", "5s", "6s", "7s", "1m", "2m", "3m", "9m", "9m",
        ]);
        assert!(g2.can_win(0, &c, true).is_some());
    }

    #[test]
    fn guobiao_needs_eight() {
        // 123m 456m 789m 123p 55s (culori mixte) fără combinații -> sub 8 fără tragere proprie
        let c = counts_of(&[
            "1m", "2m", "3m", "4m", "5m", "6m", "7m", "8m", "9m", "1p", "2p", "3p", "5s", "5s",
        ]);
        let r = score_hand(Variant::Guobiao, &c, &[], 27, false).unwrap();
        assert!(r.0 < 8 || r.1.contains("scară pură"), "{r:?}");
        // scară pură (16) + ... se acceptă
        assert!(r.1.contains("scară pură"));
        // culoare întreagă: 24
        let c = counts_of(&[
            "1m", "1m", "1m", "2m", "3m", "4m", "5m", "6m", "7m", "9m", "9m", "9m", "8m", "8m",
        ]);
        let r = score_hand(Variant::Guobiao, &c, &[], 27, false).unwrap();
        assert!(r.0 >= 24);
    }

    /// Re-amestecă tot ce `me` nu vede: mâna adversarului, zidul și zidul mort (florile rămân la locul lor).
    fn scramble_unseen(g: &mut Mj2, me: Player, seed: u64) {
        let o = me.other().idx();
        let mut vals: Vec<u8> = Vec::new();
        for t in 0..34u8 {
            for _ in 0..g.counts[o][t as usize] {
                vals.push(t);
            }
        }
        let nh = vals.len();
        let wi: Vec<usize> = (0..g.wall.len()).filter(|&i| g.wall[i] < 34).collect();
        let di: Vec<usize> = (0..g.dead.len()).filter(|&i| g.dead[i] < 34).collect();
        vals.extend(wi.iter().map(|&i| g.wall[i]));
        vals.extend(di.iter().map(|&i| g.dead[i]));
        crate::p3_comun::shuffle_with(&mut vals, seed);
        g.counts[o] = [0; 34];
        for &t in &vals[..nh] {
            g.counts[o][t as usize] += 1;
        }
        for (k, &i) in wi.iter().enumerate() {
            g.wall[i] = vals[nh + k];
        }
        for (k, &i) in di.iter().enumerate() {
            g.dead[i] = vals[nh + wi.len() + k];
        }
    }

    #[test]
    fn ai_nu_vede_mana_adversarului_nici_zidul_la_orice_nivel() {
        for (name, v) in [
            ("guobiao", Variant::Guobiao),
            ("sichuan_mahjong", Variant::Sichuan),
        ] {
            crate::p3_comun::ai_blind_check(name, |s| Mj2::with_seed(v, s), scramble_unseen, 8);
        }
    }

    #[test]
    fn sfarsitul_zidului_nu_depinde_de_mana_adversarului() {
        for v in [Variant::Sichuan, Variant::Guobiao] {
            for seed in 1..=12u64 {
                let mut g = Mj2::with_seed(v, seed);
                if g.phase == Phase::Declare {
                    g.apply(&Move::Declare(0));
                    g.apply(&Move::Declare(1));
                }
                g.wall = vec![5];
                g.apply(&Move::Draw);
                assert!(g.wall.is_empty() && g.phase == Phase::Discard);
                let mut w = g.clone();
                scramble_unseen(&mut w, Player::One, seed * 31);
                for m in g.legal_moves() {
                    let (mut a, mut b) = (g.clone(), w.clone());
                    a.apply(&m);
                    b.apply(&m);
                    assert_eq!(a.outcome(), b.outcome(), "seed {seed}, {m:?}");
                    assert_eq!(a.evaluate(Player::One), b.evaluate(Player::One));
                }
            }
        }
    }

    #[test]
    fn random_games_end() {
        for v in [Variant::Sichuan, Variant::Guobiao] {
            for seed in 1..=5u64 {
                let mut g = Mj2::with_seed(v, seed);
                let mut rng = Rng::new(seed);
                let mut n = 0;
                while g.outcome().is_none() {
                    let m = g.legal_moves();
                    assert!(!m.is_empty(), "{}", g.render());
                    g.apply(&m[rng.below(m.len())]);
                    n += 1;
                    assert!(n < 2000);
                }
            }
        }
    }
}

#[cfg(test)]
mod a1_blind {
    use super::*;
    use crate::ai_audit_a1::{audit_seeds, shuffle};

    #[test]
    fn sichuan_ai_ignores_opponent_tiles_and_walls_at_all_levels() {
        audit_seeds(
            "sichuan_mahjong",
            |s| Mj2::with_seed(Variant::Sichuan, s),
            1..=3,
            3,
            90,
            |g, salt| {
                let foe = g.side.other().idx();
                let mut pool: Vec<u8> = Vec::new();
                for t in 0..34u8 {
                    for _ in 0..g.counts[foe][t as usize] {
                        pool.push(t);
                    }
                }
                let n = pool.len();
                let (nw, nd) = (g.wall.len(), g.dead.len());
                pool.extend(g.wall.iter());
                pool.extend(g.dead.iter());
                shuffle(&mut pool, salt);
                g.counts[foe] = [0; 34];
                for &t in &pool[..n] {
                    g.counts[foe][t as usize] += 1;
                }
                g.wall = pool[n..n + nw].to_vec();
                g.dead = pool[n + nw..n + nw + nd].to_vec();
            },
            &[7],
        );
    }
}
