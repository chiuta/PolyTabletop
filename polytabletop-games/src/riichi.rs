#![allow(
    clippy::needless_range_loop,
    clippy::manual_is_multiple_of,
    clippy::manual_range_contains,
    clippy::too_many_arguments,
    clippy::doc_lazy_continuation,
    clippy::unnecessary_unwrap,
    clippy::unnecessary_cast,
    clippy::type_complexity
)]
//! Riichi Mahjong (Japonia) — mahjong japonez standard, adaptat pentru 2 jucători.
//!
//! Reguli folosite (regulile standard de club, fără piese roșii „aka”):
//! * 136 de piese (3 culori × 9 × 4 + 4 vânturi + 3 dragoni), 13 piese fiecăruia. Zidul mort are
//!   14 piese (indicatori de dora, ura-dora, trageri după careu); zidul viu se trage pe rând.
//!   **Zidul e ascuns** — draw-ul (`trage`) e o mutare forțată, ca AI-ul să nu vadă piesa următoare.
//!   Notația pieselor: `1m`–`9m` (caractere), `1p`–`9p` (cercuri), `1s`–`9s` (bambus),
//!   `1z`–`7z` = Est, Sud, Vest, Nord, alb, verde, roșu.
//! * Meci: 4 mâini (Est 1, Est 2, Sud 1, Sud 2), dealerul alternează între J1 și J2; ambii încep cu
//!   25 000 de puncte. Dealerul = vântul Est, celălalt = Sud; vântul rundei: Est pentru primele două
//!   mâini, Sud pentru ultimele. Dealerul care câștigă sau are mână gata (tenpai) la remiză
//!   rămâne dealer (renchan, +1 honba = +300 puncte la fiecare câștig). Meciul se termină după
//!   a patra mână (dacă dealerul câștigă în ultima mână și e în frunte, se oprește imediat) sau când
//!   cineva coboară sub 0. Câștigă cine are mai multe puncte.
//! * Tura: `trage`, apoi arunci (`5m`), declari `riichi 5m` (mână închisă gata, 1000 de puncte
//!   pe masă; după riichi mâna e blocată: doar tsumogiri, `tsumo` sau careu închis fără a schimba
//!   așteptarea), `tsumo` (câștig din tragere), `ankan 5m` (careu închis), `kakan 5m` (careu din
//!   pung expus). După careu se trage din zidul mort (`trage-moarta`). Pe aruncarea adversarului:
//!   `ron`, `pon`, `kan`, `chi 4m 6m` (cu cele două piese din mână; în 2 jucători se poate de la
//!   orice aruncare) sau `pas`.
//! * Mână câștigătoare: 4 seturi + pereche, șapte perechi (chiitoitsu) sau treisprezece orfani
//!   (kokushi), cu cel puțin un yaku. **Furiten**: nu poți da ron dacă una din piesele pe care
//!   le aștepți e în aruncările tale; nici după ce ai ratat un ron (până la următoarea ta aruncare,
//!   iar după riichi, pentru restul mâinii).
//! * Yaku implementate: riichi, riichi dublu, ippatsu, tsumo închis, pinfu, tanyao, iipeikou,
//!   yakuhai (dragoni, vântul tău, vântul rundei), haitei, houtei, rinshan kaihou, sanshoku
//!   doujun, ittsu, chanta, toitoi, sanankou, sanshoku doukou, sankantsu, shousangen, honroutou,
//!   chiitoitsu, honitsu, junchan, ryanpeikou, chinitsu; yakuman: kokushi, suuankou, daisangen,
//!   shousuushii, daisuushii, tsuuiisou, chinroutou, ryuuiisou, chuuren poutou, suukantsu,
//!   tenhou, chiihou (fără yakuman dublu; ≥13 han = kazoe yakuman). Dora și ura-dora.
//! * Puncte: fu (20 + deschideri/așteptări/seturi, rotunjit la 10; 25 la chiitoitsu) și han →
//!   puncte de bază `fu·2^(han+2)`, cu limite mangan 2000 (5 han sau ≥ 2000), haneman 3000
//!   (6–7), baiman 4000 (8–10), sanbaiman 6000 (11–12), yakuman 8000. Ron: dealer ×6, celălalt ×4;
//!   tsumo: se plătește suma celor trei plătitori din jocul în patru (dealer: 3 × 2b; nedealer:
//!   2b + 2 × b), rotunjit la 100, pe lângă +300/honba și bățul de riichi. Remiză (zidul epuizat):
//!   cine nu e gata plătește 1500 celui gata.
//! * Omise: piese roșii (aka dora), kuikae complet verificat doar pentru aceeași piesă / capătul
//!   scării, chankan, nagashi mangan, remize abortive (kyuushu, suukaikan etc.), pao, plata dublă
//!   la yakuman, regula riichi cu zid sub 4 piese, 3–4 jucători, kazoe pe yaku-uri peste 13 han
//!   doar ca yakuman simplu.
//! * AI: eficiență (shanten + piese utile) din propria mână și piesele vizibile; la Normal+ evită
//!   piesele periculoase față de riichi (genbutsu, suji), la Hard/Expert apără mai ferm și
//!   cheamă doar cu yaku. Nu vede mâna adversarului, zidul sau zidul mort.

use crate::cards::Viewer;
use crate::cards52::shuffle;
use crate::p2_util::{noise, sig_of};
use polytabletop_core::ai::{level, Level};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::cell::RefCell;
use std::collections::HashMap;

pub type Counts = [u8; 34];

pub const START_POINTS: i32 = 25_000;
pub const NOTEN_PAY: i32 = 1500;

pub fn tile_name(t: u8) -> String {
    if t < 27 {
        format!("{}{}", t % 9 + 1, ['m', 'p', 's'][(t / 9) as usize])
    } else {
        format!("{}z", t - 26)
    }
}

fn is_honor(t: u8) -> bool {
    t >= 27
}
fn is_terminal(t: u8) -> bool {
    t < 27 && (t % 9 == 0 || t % 9 == 8)
}
fn is_yaochuu(t: u8) -> bool {
    is_honor(t) || is_terminal(t)
}
fn is_dragon(t: u8) -> bool {
    t >= 31
}
fn next_dora(ind: u8) -> u8 {
    match ind {
        0..=26 => {
            if ind % 9 == 8 {
                ind - 8
            } else {
                ind + 1
            }
        }
        27..=30 => 27 + (ind - 27 + 1) % 4,
        _ => 31 + (ind - 31 + 1) % 3,
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MKind {
    Chi,
    Pon,
    /// Careu deschis (daiminkan sau kakan).
    Kan,
    Ankan,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RMeld {
    pub kind: MKind,
    /// La Chi: cea mai mică piesă.
    pub tile: u8,
}

impl RMeld {
    pub fn open(&self) -> bool {
        self.kind != MKind::Ankan
    }
    pub fn tiles(&self) -> Vec<u8> {
        match self.kind {
            MKind::Chi => vec![self.tile, self.tile + 1, self.tile + 2],
            MKind::Pon => vec![self.tile; 3],
            MKind::Kan | MKind::Ankan => vec![self.tile; 4],
        }
    }
    fn text(&self) -> String {
        let t = self
            .tiles()
            .iter()
            .map(|&x| tile_name(x))
            .collect::<Vec<_>>()
            .join("");
        if self.kind == MKind::Ankan {
            format!("[{t}]")
        } else {
            format!("({t})")
        }
    }
}

// ───────────────────────── descompuneri și shanten ─────────────────────────

/// Toate descompunerile (pereche, seturi) ale pieselor ascunse în `need` seturi.
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

fn is_chiitoi(c: &Counts) -> bool {
    c.iter().filter(|&&x| x == 2).count() == 7 && c.iter().all(|&x| x == 0 || x == 2)
}

fn is_kokushi(c: &Counts) -> bool {
    let mut pair = false;
    for t in 0..34u8 {
        let n = c[t as usize];
        if is_yaochuu(t) {
            if n == 0 {
                return false;
            }
            if n == 2 {
                pair = true;
            } else if n != 1 {
                return false;
            }
        } else if n != 0 {
            return false;
        }
    }
    pair
}

/// Mână completă (orice formă), fără cerința de yaku.
pub fn is_complete(c: &Counts, melds: usize) -> bool {
    let total: u32 = c.iter().map(|&x| x as u32).sum();
    if melds > 4 || total != 3 * (4 - melds as u32) + 2 {
        return false;
    }
    if melds == 0 && (is_chiitoi(c) || is_kokushi(c)) {
        return true;
    }
    let mut cc = *c;
    !decomps(&mut cc, 4 - melds).is_empty()
}

/// Piesele care completează o mână de 13 (echivalent) piese.
pub fn waits(c: &Counts, melds: usize) -> Vec<u8> {
    let mut v = Vec::new();
    let mut cc = *c;
    for t in 0..34usize {
        if cc[t] >= 4 {
            continue;
        }
        cc[t] += 1;
        if is_complete(&cc, melds) {
            v.push(t as u8);
        }
        cc[t] -= 1;
    }
    v
}

thread_local! {
    static SUIT_CACHE: RefCell<HashMap<u64, Vec<(u8, u8, bool)>>> = RefCell::new(HashMap::new());
}

/// Perechi atinse (seturi, parțiale, pereche) pentru o culoare (9 piese) sau onoruri (7).
fn suit_options(c: &[u8], honors: bool) -> Vec<(u8, u8, bool)> {
    let mut key = honors as u64;
    for &x in c {
        key = key * 5 + x as u64;
    }
    if let Some(v) = SUIT_CACHE.with(|m| m.borrow().get(&key).cloned()) {
        return v;
    }
    let n = c.len();
    let mut seen = [[[false; 2]; 6]; 6];
    let mut arr = [0u8; 9];
    arr[..n].copy_from_slice(c);
    #[allow(clippy::too_many_arguments)]
    fn rec(
        a: &mut [u8; 9],
        n: usize,
        i: usize,
        m: usize,
        t: usize,
        pair: bool,
        honors: bool,
        seen: &mut [[[bool; 2]; 6]; 6],
    ) {
        let mut i = i;
        while i < n && a[i] == 0 {
            i += 1;
        }
        if i >= n {
            seen[m.min(5)][t.min(5)][pair as usize] = true;
            return;
        }
        if a[i] >= 3 {
            a[i] -= 3;
            rec(a, n, i, m + 1, t, pair, honors, seen);
            a[i] += 3;
        }
        if !honors && i + 2 < n && a[i + 1] > 0 && a[i + 2] > 0 {
            a[i] -= 1;
            a[i + 1] -= 1;
            a[i + 2] -= 1;
            rec(a, n, i, m + 1, t, pair, honors, seen);
            a[i] += 1;
            a[i + 1] += 1;
            a[i + 2] += 1;
        }
        if a[i] >= 2 && !pair {
            a[i] -= 2;
            rec(a, n, i, m, t, true, honors, seen);
            a[i] += 2;
        }
        if a[i] >= 2 {
            a[i] -= 2;
            rec(a, n, i, m, t + 1, pair, honors, seen);
            a[i] += 2;
        }
        if !honors {
            if i + 1 < n && a[i + 1] > 0 {
                a[i] -= 1;
                a[i + 1] -= 1;
                rec(a, n, i, m, t + 1, pair, honors, seen);
                a[i] += 1;
                a[i + 1] += 1;
            }
            if i + 2 < n && a[i + 2] > 0 {
                a[i] -= 1;
                a[i + 2] -= 1;
                rec(a, n, i, m, t + 1, pair, honors, seen);
                a[i] += 1;
                a[i + 2] += 1;
            }
        }
        a[i] -= 1;
        rec(a, n, i, m, t, pair, honors, seen);
        a[i] += 1;
    }
    rec(&mut arr, n, 0, 0, 0, false, honors, &mut seen);
    let mut out = Vec::new();
    for m in 0..6 {
        for t in 0..6 {
            for p in 0..2 {
                if seen[m][t][p] {
                    out.push((m as u8, t as u8, p == 1));
                }
            }
        }
    }
    SUIT_CACHE.with(|mm| mm.borrow_mut().insert(key, out.clone()));
    out
}

/// Shanten pentru forma obișnuită (seturi + pereche), cu `melds` seturi deja făcute.
pub fn shanten_regular(c: &Counts, melds: usize) -> i32 {
    // dp[m][t][pair] posibil
    let mut dp = [[[false; 2]; 5]; 5];
    dp[melds.min(4)][0][0] = true;
    let groups: [(&[u8], bool); 4] = [
        (&c[0..9], false),
        (&c[9..18], false),
        (&c[18..27], false),
        (&c[27..34], true),
    ];
    for (g, honors) in groups {
        let opts = suit_options(g, honors);
        let mut nd = [[[false; 2]; 5]; 5];
        for m in 0..5usize {
            for t in 0..5usize {
                for p in 0..2usize {
                    if !dp[m][t][p] {
                        continue;
                    }
                    for &(om, ot, op) in &opts {
                        let nm = m + om as usize;
                        let nt = t + ot as usize;
                        let np = p + op as usize;
                        if nm > 4 || np > 1 {
                            continue;
                        }
                        nd[nm][nt.min(4)][np] = true;
                    }
                }
            }
        }
        dp = nd;
    }
    let mut best = 8;
    for m in 0..5usize {
        for t in 0..5usize {
            for p in 0..2usize {
                if dp[m][t][p] {
                    let tt = t.min(4 - m);
                    best = best.min(8 - 2 * m as i32 - tt as i32 - p as i32);
                }
            }
        }
    }
    best
}

pub fn shanten(c: &Counts, melds: usize) -> i32 {
    let mut best = shanten_regular(c, melds);
    if melds == 0 {
        let pairs = c.iter().filter(|&&x| x >= 2).count() as i32;
        let kinds = c.iter().filter(|&&x| x >= 1).count() as i32;
        best = best.min(6 - pairs + (7 - kinds).max(0));
        let ky = (0..34u8)
            .filter(|&t| is_yaochuu(t) && c[t as usize] >= 1)
            .count() as i32;
        let kp = (0..34u8).any(|t| is_yaochuu(t) && c[t as usize] >= 2) as i32;
        best = best.min(13 - ky - kp);
    }
    best
}

// ───────────────────────────── scorarea mâinii ─────────────────────────────

#[derive(Clone, Debug)]
pub struct WinCtx {
    /// Piesele din mână, cu piesa câștigătoare inclusă.
    pub closed: Counts,
    pub melds: Vec<RMeld>,
    pub win_tile: u8,
    pub tsumo: bool,
    pub seat_wind: u8,
    pub round_wind: u8,
    pub riichi: bool,
    pub double_riichi: bool,
    pub ippatsu: bool,
    pub haitei: bool,
    pub houtei: bool,
    pub rinshan: bool,
    pub tenhou: bool,
    pub chiihou: bool,
    /// Dora + ura-dora (+ aka) în mână.
    pub dora: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WinResult {
    pub han: i32,
    pub fu: i32,
    pub yakuman: i32,
    pub base: i32,
    pub yaku: Vec<String>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Wait {
    Tanki,
    Shanpon,
    Kanchan,
    Penchan,
    Ryanmen,
}

#[derive(Clone, Copy, Debug)]
struct Group {
    trip: bool,
    tile: u8,
    open: bool,
    kan: bool,
}

impl Group {
    fn tiles(&self) -> Vec<u8> {
        if self.trip {
            vec![self.tile; 3]
        } else {
            vec![self.tile, self.tile + 1, self.tile + 2]
        }
    }
    fn has_yaochuu(&self) -> bool {
        if self.trip {
            is_yaochuu(self.tile)
        } else {
            self.tile % 9 == 0 || self.tile % 9 == 6
        }
    }
}

pub fn base_points(han: i32, fu: i32, yakuman: i32) -> i32 {
    if yakuman > 0 {
        return 8000 * yakuman;
    }
    match han {
        h if h >= 13 => 8000,
        11..=12 => 6000,
        8..=10 => 4000,
        6..=7 => 3000,
        5 => 2000,
        h => (fu * (1 << (h + 2))).min(2000),
    }
}

fn ceil100(x: i32) -> i32 {
    (x + 99) / 100 * 100
}

/// Suma plătită de perdant (ron sau echivalentul tsumo pentru doi jucători), fără honba.
pub fn payment(base: i32, dealer_wins: bool, tsumo: bool) -> i32 {
    match (dealer_wins, tsumo) {
        (true, false) => ceil100(base * 6),
        (false, false) => ceil100(base * 4),
        (true, true) => 3 * ceil100(base * 2),
        (false, true) => ceil100(base * 2) + 2 * ceil100(base),
    }
}

fn suit_of(t: u8) -> i32 {
    if t < 27 {
        (t / 9) as i32
    } else {
        3
    }
}

fn collect_special_yakuman(ctx: &WinCtx, out: &mut Vec<String>) {
    if ctx.tenhou {
        out.push("tenhou".into());
    }
    if ctx.chiihou {
        out.push("chiihou".into());
    }
}

/// Scorează mâna: `None` dacă nu e completă sau nu are niciun yaku.
pub fn score_win(ctx: &WinCtx) -> Option<WinResult> {
    let closed_hand = ctx.melds.iter().all(|m| !m.open());
    let mut best: Option<WinResult> = None;
    let mut consider = |r: WinResult| {
        let better = match &best {
            None => true,
            Some(b) => (r.base, r.han, r.fu) > (b.base, b.han, b.fu),
        };
        if better {
            best = Some(r);
        }
    };
    let total: u32 = ctx.closed.iter().map(|&x| x as u32).sum();
    if total != 3 * (4 - ctx.melds.len() as u32) + 2 {
        return None;
    }

    // kokushi
    if ctx.melds.is_empty() && is_kokushi(&ctx.closed) {
        let mut ym = vec!["kokushi musou".to_string()];
        collect_special_yakuman(ctx, &mut ym);
        consider(WinResult {
            han: 0,
            fu: 0,
            yakuman: ym.len() as i32,
            base: base_points(0, 0, ym.len() as i32),
            yaku: ym,
        });
    }

    // chiitoitsu
    if ctx.melds.is_empty() && is_chiitoi(&ctx.closed) {
        let tiles: Vec<u8> = (0..34u8)
            .flat_map(|t| std::iter::repeat_n(t, ctx.closed[t as usize] as usize))
            .collect();
        let mut ym: Vec<String> = Vec::new();
        collect_special_yakuman(ctx, &mut ym);
        if tiles.iter().all(|&t| is_honor(t)) {
            ym.push("tsuuiisou".into());
        }
        if !ym.is_empty() {
            consider(WinResult {
                han: 0,
                fu: 25,
                yakuman: ym.len() as i32,
                base: base_points(0, 25, ym.len() as i32),
                yaku: ym,
            });
        } else {
            let mut han = 2;
            let mut ys = vec!["chiitoitsu 2".to_string()];
            common_menzen_yaku(ctx, true, &mut han, &mut ys);
            if tiles.iter().all(|&t| !is_yaochuu(t)) {
                han += 1;
                ys.push("tanyao 1".into());
            }
            if tiles.iter().all(|&t| is_yaochuu(t)) {
                han += 2;
                ys.push("honroutou 2".into());
            }
            let suits: Vec<i32> = {
                let mut s: Vec<i32> = tiles
                    .iter()
                    .map(|&t| suit_of(t))
                    .filter(|&s| s < 3)
                    .collect();
                s.sort();
                s.dedup();
                s
            };
            let hon = tiles.iter().any(|&t| is_honor(t));
            if suits.len() == 1 {
                if hon {
                    han += 3;
                    ys.push("honitsu 3".into());
                } else {
                    han += 6;
                    ys.push("chinitsu 6".into());
                }
            }
            let yaku_han = han;
            han += ctx.dora;
            if ctx.dora > 0 {
                ys.push(format!("dora {}", ctx.dora));
            }
            let _ = yaku_han;
            consider(WinResult {
                han,
                fu: 25,
                yakuman: 0,
                base: base_points(han, 25, 0),
                yaku: ys,
            });
        }
    }

    // forma obișnuită
    let need = 4 - ctx.melds.len();
    let mut cc = ctx.closed;
    for (pair, sets) in decomps(&mut cc, need) {
        let w = ctx.win_tile;
        let mut interps: Vec<(Option<usize>, Wait)> = Vec::new();
        if pair == w {
            interps.push((None, Wait::Tanki));
        }
        for (j, &(trip, t)) in sets.iter().enumerate() {
            if trip && t == w {
                interps.push((Some(j), Wait::Shanpon));
            }
            if !trip && w < 27 && w >= t && w <= t + 2 {
                let pos = w - t;
                let wk = if pos == 1 {
                    Wait::Kanchan
                } else if (pos == 0 && t % 9 == 6) || (pos == 2 && t % 9 == 0) {
                    Wait::Penchan
                } else {
                    Wait::Ryanmen
                };
                interps.push((Some(j), wk));
            }
        }
        for (si, wait) in interps {
            let mut groups: Vec<Group> = Vec::new();
            for (j, &(trip, t)) in sets.iter().enumerate() {
                let open = trip && si == Some(j) && !ctx.tsumo && wait == Wait::Shanpon;
                groups.push(Group {
                    trip,
                    tile: t,
                    open,
                    kan: false,
                });
            }
            for m in &ctx.melds {
                groups.push(match m.kind {
                    MKind::Chi => Group {
                        trip: false,
                        tile: m.tile,
                        open: true,
                        kan: false,
                    },
                    MKind::Pon => Group {
                        trip: true,
                        tile: m.tile,
                        open: true,
                        kan: false,
                    },
                    MKind::Kan => Group {
                        trip: true,
                        tile: m.tile,
                        open: true,
                        kan: true,
                    },
                    MKind::Ankan => Group {
                        trip: true,
                        tile: m.tile,
                        open: false,
                        kan: true,
                    },
                });
            }
            if let Some(r) = score_regular(ctx, &groups, pair, wait, closed_hand) {
                consider(r);
            }
        }
    }
    best
}

fn common_menzen_yaku(ctx: &WinCtx, closed_hand: bool, han: &mut i32, ys: &mut Vec<String>) {
    if ctx.double_riichi {
        *han += 2;
        ys.push("riichi dublu 2".into());
    } else if ctx.riichi {
        *han += 1;
        ys.push("riichi 1".into());
    }
    if ctx.ippatsu && ctx.riichi {
        *han += 1;
        ys.push("ippatsu 1".into());
    }
    if ctx.tsumo && closed_hand {
        *han += 1;
        ys.push("tsumo închis 1".into());
    }
    if ctx.haitei && ctx.tsumo && !ctx.rinshan {
        *han += 1;
        ys.push("haitei 1".into());
    }
    if ctx.houtei && !ctx.tsumo {
        *han += 1;
        ys.push("houtei 1".into());
    }
    if ctx.rinshan && ctx.tsumo {
        *han += 1;
        ys.push("rinshan kaihou 1".into());
    }
}

fn score_regular(
    ctx: &WinCtx,
    groups: &[Group],
    pair: u8,
    wait: Wait,
    closed_hand: bool,
) -> Option<WinResult> {
    let mut tiles: Vec<u8> = vec![pair, pair];
    for g in groups {
        tiles.extend(g.tiles());
    }
    let seqs: Vec<u8> = groups.iter().filter(|g| !g.trip).map(|g| g.tile).collect();
    let trips: Vec<&Group> = groups.iter().filter(|g| g.trip).collect();
    let conc_trips = trips.iter().filter(|g| !g.open).count();
    let kans = groups.iter().filter(|g| g.kan).count();
    let seat = ctx.seat_wind;
    let round = ctx.round_wind;

    // ── yakuman ──
    let mut ym: Vec<String> = Vec::new();
    collect_special_yakuman(ctx, &mut ym);
    let dragon_trips = trips.iter().filter(|g| is_dragon(g.tile)).count();
    let wind_trips = trips.iter().filter(|g| (27..31).contains(&g.tile)).count();
    if conc_trips == 4 && closed_hand {
        ym.push("suuankou".into());
    }
    if dragon_trips == 3 {
        ym.push("daisangen".into());
    }
    if wind_trips == 4 {
        ym.push("daisuushii".into());
    } else if wind_trips == 3 && (27..31).contains(&pair) {
        ym.push("shousuushii".into());
    }
    if tiles.iter().all(|&t| is_honor(t)) {
        ym.push("tsuuiisou".into());
    }
    if tiles.iter().all(|&t| is_terminal(t)) {
        ym.push("chinroutou".into());
    }
    if tiles
        .iter()
        .all(|&t| matches!(t, 19 | 20 | 21 | 23 | 25 | 32))
    {
        ym.push("ryuuiisou".into());
    }
    if kans == 4 {
        ym.push("suukantsu".into());
    }
    if closed_hand && ctx.melds.is_empty() {
        // chuuren poutou
        let s = suit_of(tiles[0]);
        if s < 3 && tiles.iter().all(|&t| suit_of(t) == s) {
            let base = (s * 9) as usize;
            let c = &ctx.closed;
            let ok = c[base] >= 3 && c[base + 8] >= 3 && (1..8).all(|i| c[base + i] >= 1);
            if ok {
                ym.push("chuuren poutou".into());
            }
        }
    }
    if !ym.is_empty() {
        return Some(WinResult {
            han: 0,
            fu: 30,
            yakuman: ym.len() as i32,
            base: base_points(0, 30, ym.len() as i32),
            yaku: ym,
        });
    }

    // ── yaku obișnuite ──
    let mut han = 0;
    let mut ys: Vec<String> = Vec::new();
    if closed_hand {
        common_menzen_yaku(ctx, true, &mut han, &mut ys);
    } else {
        common_menzen_yaku(
            &WinCtx {
                riichi: false,
                double_riichi: false,
                ippatsu: false,
                ..ctx.clone()
            },
            false,
            &mut han,
            &mut ys,
        );
    }
    let pair_yakuhai = is_dragon(pair) || pair == seat || pair == round;
    let pinfu = closed_hand && trips.is_empty() && !pair_yakuhai && wait == Wait::Ryanmen;
    if pinfu {
        han += 1;
        ys.push("pinfu 1".into());
    }
    if tiles.iter().all(|&t| !is_yaochuu(t)) {
        han += 1;
        ys.push("tanyao 1".into());
    }
    // peikou
    if closed_hand {
        let mut cnt: HashMap<u8, usize> = HashMap::new();
        for &x in &seqs {
            *cnt.entry(x).or_insert(0) += 1;
        }
        let pk: usize = cnt.values().map(|&v| v / 2).sum();
        if pk >= 2 {
            han += 3;
            ys.push("ryanpeikou 3".into());
        } else if pk == 1 {
            han += 1;
            ys.push("iipeikou 1".into());
        }
    }
    // yakuhai
    for g in &trips {
        if is_dragon(g.tile) {
            han += 1;
            ys.push("yakuhai (dragon) 1".into());
        }
        if g.tile == seat {
            han += 1;
            ys.push("yakuhai (vântul tău) 1".into());
        }
        if g.tile == round {
            han += 1;
            ys.push("yakuhai (vântul rundei) 1".into());
        }
    }
    // sanshoku doujun / ittsu
    let open_pen = if closed_hand { 0 } else { 1 };
    for n in 0..7u8 {
        if seqs.contains(&n) && seqs.contains(&(9 + n)) && seqs.contains(&(18 + n)) {
            han += 2 - open_pen;
            ys.push(format!("sanshoku doujun {}", 2 - open_pen));
            break;
        }
    }
    for s in 0..3u8 {
        if seqs.contains(&(9 * s)) && seqs.contains(&(9 * s + 3)) && seqs.contains(&(9 * s + 6)) {
            han += 2 - open_pen;
            ys.push(format!("ittsu {}", 2 - open_pen));
            break;
        }
    }
    // chanta / junchan / honroutou / toitoi
    let all_groups_yao = groups.iter().all(|g| g.has_yaochuu()) && is_yaochuu(pair);
    let has_honor = tiles.iter().any(|&t| is_honor(t));
    if trips.len() == 4 {
        han += 2;
        ys.push("toitoi 2".into());
    }
    if all_groups_yao {
        if seqs.is_empty() {
            han += 2;
            ys.push("honroutou 2".into());
        } else if has_honor {
            han += 2 - open_pen;
            ys.push(format!("chanta {}", 2 - open_pen));
        } else {
            han += 3 - open_pen;
            ys.push(format!("junchan {}", 3 - open_pen));
        }
    }
    if conc_trips >= 3 {
        han += 2;
        ys.push("sanankou 2".into());
    }
    for n in 0..9u8 {
        let c = |s: u8| trips.iter().any(|g| g.tile == 9 * s + n);
        if c(0) && c(1) && c(2) {
            han += 2;
            ys.push("sanshoku doukou 2".into());
            break;
        }
    }
    if kans == 3 {
        han += 2;
        ys.push("sankantsu 2".into());
    }
    if dragon_trips == 2 && is_dragon(pair) {
        han += 2;
        ys.push("shousangen 2".into());
    }
    // honitsu / chinitsu
    let mut suits: Vec<i32> = tiles
        .iter()
        .map(|&t| suit_of(t))
        .filter(|&s| s < 3)
        .collect();
    suits.sort();
    suits.dedup();
    if suits.len() == 1 {
        if has_honor {
            han += 3 - open_pen;
            ys.push(format!("honitsu {}", 3 - open_pen));
        } else {
            han += 6 - open_pen;
            ys.push(format!("chinitsu {}", 6 - open_pen));
        }
    }
    if han == 0 {
        return None;
    }
    if ctx.dora > 0 {
        han += ctx.dora;
        ys.push(format!("dora {}", ctx.dora));
    }
    // ── fu ──
    let mut fu = 20;
    if closed_hand && !ctx.tsumo {
        fu += 10;
    }
    if ctx.tsumo && !pinfu {
        fu += 2;
    }
    for g in groups.iter().filter(|g| g.trip) {
        let mut f = if is_yaochuu(g.tile) { 4 } else { 2 };
        if !g.open {
            f *= 2;
        }
        if g.kan {
            f *= 4;
        }
        fu += f;
    }
    if is_dragon(pair) {
        fu += 2;
    }
    if pair == seat {
        fu += 2;
    }
    if pair == round {
        fu += 2;
    }
    if matches!(wait, Wait::Kanchan | Wait::Penchan | Wait::Tanki) {
        fu += 2;
    }
    if pinfu && ctx.tsumo {
        fu = 20;
    } else if !closed_hand && fu == 20 {
        fu = 30;
    }
    fu = (fu + 9) / 10 * 10;
    Some(WinResult {
        han,
        fu,
        yakuman: 0,
        base: base_points(han, fu, 0),
        yaku: ys,
    })
}

// ───────────────────────────────── jocul ─────────────────────────────────

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Draw,
    DrawDead,
    Discard(u8),
    Riichi(u8),
    Tsumo,
    Ankan(u8),
    Kakan(u8),
    Ron,
    Pon,
    Kan,
    Chi(u8, u8),
    Pass,
    Next,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Draw,
    DrawDead,
    Discard,
    React,
    HandOver,
    Over,
}

#[derive(Clone)]
pub struct Riichi {
    wall: Vec<u8>,
    dead: Vec<u8>,
    rin_used: usize,
    dora_n: usize,
    pending_dora: usize,
    closed: [Counts; 2],
    melds: [Vec<RMeld>; 2],
    rivers: [Vec<(u8, bool)>; 2],
    riichi: [bool; 2],
    double_riichi: [bool; 2],
    ippatsu: [bool; 2],
    furiten_temp: [bool; 2],
    furiten_riichi: [bool; 2],
    drawn: [Option<u8>; 2],
    pts: [i32; 2],
    pool: i32,
    honba: i32,
    hand_idx: u8,
    dealer: Player,
    side: Player,
    phase: Phase,
    last_discard: Option<(Player, u8)>,
    kans: u8,
    rinshan: bool,
    calls_made: bool,
    no_discard: Vec<u8>,
    log: String,
    /// La sfârșitul mâinii: (a câștigat cineva, dealerul rămâne).
    renchan: bool,
    won: bool,
    /// Jucătorul care tocmai a declarat riichi cu ultima aruncare.
    decl: Option<usize>,
    rng: Rng,
    viewer: Viewer,
}

impl Default for Riichi {
    fn default() -> Self {
        Self::new()
    }
}

fn counts_of(tiles: &[u8]) -> Counts {
    let mut c = [0u8; 34];
    for &t in tiles {
        c[t as usize] += 1;
    }
    c
}

fn tiles_text(c: &Counts) -> String {
    let mut v = Vec::new();
    for t in 0..34u8 {
        for _ in 0..c[t as usize] {
            v.push(tile_name(t));
        }
    }
    v.join(" ")
}

impl Riichi {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Riichi {
            wall: Vec::new(),
            dead: Vec::new(),
            rin_used: 0,
            dora_n: 1,
            pending_dora: 0,
            closed: [[0; 34]; 2],
            melds: [Vec::new(), Vec::new()],
            rivers: [Vec::new(), Vec::new()],
            riichi: [false; 2],
            double_riichi: [false; 2],
            ippatsu: [false; 2],
            furiten_temp: [false; 2],
            furiten_riichi: [false; 2],
            drawn: [None; 2],
            pts: [START_POINTS; 2],
            pool: 0,
            honba: 0,
            hand_idx: 1,
            dealer: Player::One,
            side: Player::One,
            phase: Phase::Draw,
            last_discard: None,
            kans: 0,
            rinshan: false,
            calls_made: false,
            no_discard: Vec::new(),
            log: String::new(),
            renchan: false,
            won: false,
            decl: None,
            rng: Rng::new(seed),
            viewer: Viewer::Fixed(Player::One),
        };
        g.new_hand();
        g
    }

    fn new_hand(&mut self) {
        let mut w: Vec<u8> = (0..34u8).flat_map(|t| [t; 4]).collect();
        shuffle(&mut w, &mut self.rng);
        self.closed = [[0; 34]; 2];
        for _ in 0..13 {
            for p in 0..2 {
                let t = w.pop().unwrap();
                self.closed[p][t as usize] += 1;
            }
        }
        self.dead = (0..14).map(|_| w.pop().unwrap()).collect();
        self.wall = w;
        self.rin_used = 0;
        self.dora_n = 1;
        self.pending_dora = 0;
        self.melds = [Vec::new(), Vec::new()];
        self.rivers = [Vec::new(), Vec::new()];
        self.riichi = [false; 2];
        self.double_riichi = [false; 2];
        self.ippatsu = [false; 2];
        self.furiten_temp = [false; 2];
        self.furiten_riichi = [false; 2];
        self.drawn = [None; 2];
        self.last_discard = None;
        self.kans = 0;
        self.rinshan = false;
        self.calls_made = false;
        self.no_discard.clear();
        self.side = self.dealer;
        self.phase = Phase::Draw;
    }

    pub fn points(&self) -> (i32, i32) {
        (self.pts[0], self.pts[1])
    }
    pub fn dealer(&self) -> Player {
        self.dealer
    }
    pub fn hand_index(&self) -> u8 {
        self.hand_idx
    }

    /// Pentru teste: pune o mână (13 piese) jucătorului `p`.
    pub fn set_hand_for_test(&mut self, p: usize, tiles: &[u8]) {
        self.closed[p] = counts_of(tiles);
    }
    pub fn set_wall_for_test(&mut self, tiles: &[u8]) {
        self.wall = tiles.to_vec();
    }
    pub fn force_phase_discard_for_test(&mut self, p: Player, drawn: u8) {
        self.side = p;
        self.phase = Phase::Discard;
        self.closed[p.idx()][drawn as usize] += 1;
        self.drawn[p.idx()] = Some(drawn);
    }
    pub fn river_for_test(&mut self, p: usize, tiles: &[u8]) {
        self.rivers[p] = tiles.iter().map(|&t| (t, false)).collect();
    }
    pub fn dora_indicators_for_test(&mut self, inds: &[u8]) {
        for (i, &t) in inds.iter().enumerate() {
            self.dead[4 + 2 * i] = t;
        }
        self.dora_n = inds.len().max(1);
    }
    pub fn set_dealer_for_test(&mut self, d: Player) {
        self.dealer = d;
    }

    fn seat_wind(&self, p: usize) -> u8 {
        if self.dealer.idx() == p {
            27
        } else {
            28
        }
    }
    fn round_wind(&self) -> u8 {
        if self.hand_idx <= 2 {
            27
        } else {
            28
        }
    }

    fn dora_tiles(&self, ura: bool) -> Vec<u8> {
        (0..self.dora_n.min(5))
            .map(|i| next_dora(self.dead[4 + 2 * i + usize::from(ura)]))
            .collect()
    }

    fn dora_count(&self, p: usize, with_tile: Option<u8>, ura: bool) -> i32 {
        let mut cnt = self.closed[p];
        if let Some(t) = with_tile {
            cnt[t as usize] += 1;
        }
        for m in &self.melds[p] {
            for t in m.tiles() {
                cnt[t as usize] += 1;
            }
        }
        let mut n = 0;
        for d in self.dora_tiles(false) {
            n += cnt[d as usize] as i32;
        }
        if ura {
            for d in self.dora_tiles(true) {
                n += cnt[d as usize] as i32;
            }
        }
        n
    }

    fn ctx(&self, p: usize, win_tile: u8, tsumo: bool) -> WinCtx {
        let mut closed = self.closed[p];
        if !tsumo {
            closed[win_tile as usize] += 1;
        }
        let live_empty = self.wall.is_empty();
        WinCtx {
            closed,
            melds: self.melds[p].clone(),
            win_tile,
            tsumo,
            seat_wind: self.seat_wind(p),
            round_wind: self.round_wind(),
            riichi: self.riichi[p],
            double_riichi: self.double_riichi[p],
            ippatsu: self.ippatsu[p],
            haitei: tsumo && live_empty && !self.rinshan,
            houtei: !tsumo && live_empty,
            rinshan: tsumo && self.rinshan,
            tenhou: tsumo
                && p == self.dealer.idx()
                && !self.calls_made
                && self.rivers[0].is_empty()
                && self.rivers[1].is_empty(),
            chiihou: tsumo
                && p != self.dealer.idx()
                && !self.calls_made
                && self.rivers[p].is_empty()
                && self.rivers[1 - p].len() == 1,
            dora: self.dora_count(p, if tsumo { None } else { Some(win_tile) }, self.riichi[p]),
        }
    }

    fn can_tsumo(&self, p: usize) -> Option<WinResult> {
        let t = self.drawn[p]?;
        score_win(&self.ctx(p, t, true))
    }

    fn is_furiten(&self, p: usize) -> bool {
        if self.furiten_temp[p] || self.furiten_riichi[p] {
            return true;
        }
        let w = waits(&self.closed[p], self.melds[p].len());
        w.iter().any(|t| self.rivers[p].iter().any(|(x, _)| x == t))
    }

    fn can_ron(&self, p: usize, t: u8) -> Option<WinResult> {
        if self.is_furiten(p) {
            return None;
        }
        score_win(&self.ctx(p, t, false))
    }

    fn visible(&self, viewer: usize) -> Counts {
        let mut v = self.closed[viewer];
        for p in 0..2 {
            for m in &self.melds[p] {
                for t in m.tiles() {
                    // piesele din meld ale mele sunt deja scoase din `closed`
                    v[t as usize] += 1;
                }
            }
            for &(t, taken) in &self.rivers[p] {
                if !taken {
                    v[t as usize] += 1;
                }
            }
        }
        for t in self.dora_indicators_vis() {
            v[t as usize] += 1;
        }
        v
    }

    fn dora_indicators_vis(&self) -> Vec<u8> {
        (0..self.dora_n.min(5))
            .map(|i| self.dead[4 + 2 * i])
            .collect()
    }

    /// Opțiunile de reacție ale lui `r` la aruncarea `t`.
    fn reactions(&self, r: usize, t: u8) -> Vec<Move> {
        let mut v = Vec::new();
        if self.can_ron(r, t).is_some() {
            v.push(Move::Ron);
        }
        if !self.wall.is_empty() && !self.riichi[r] {
            let c = &self.closed[r];
            if c[t as usize] >= 2 {
                v.push(Move::Pon);
            }
            if c[t as usize] >= 3 && self.kans < 4 {
                v.push(Move::Kan);
            }
            if t < 27 {
                let s = (t / 9) * 9;
                let pos = t - s;
                let has = |x: i32| -> bool { x >= 0 && x <= 8 && c[(s as i32 + x) as usize] > 0 };
                let p = pos as i32;
                if has(p - 2) && has(p - 1) {
                    v.push(Move::Chi(t - 2, t - 1));
                }
                if has(p - 1) && has(p + 1) {
                    v.push(Move::Chi(t - 1, t + 1));
                }
                if has(p + 1) && has(p + 2) {
                    v.push(Move::Chi(t + 1, t + 2));
                }
            }
        }
        if !v.is_empty() {
            v.push(Move::Pass);
        }
        v
    }

    fn start_turn(&mut self, p: Player) {
        if self.wall.is_empty() {
            self.exhaustive_draw();
        } else {
            self.side = p;
            self.phase = Phase::Draw;
        }
    }

    fn reveal_pending_dora(&mut self) {
        self.dora_n = (self.dora_n + self.pending_dora).min(5);
        self.pending_dora = 0;
    }

    fn tenpai(&self, p: usize) -> bool {
        !waits(&self.closed[p], self.melds[p].len()).is_empty()
    }

    fn exhaustive_draw(&mut self) {
        let t = [self.tenpai(0), self.tenpai(1)];
        let mut note = String::from("remiză (zid epuizat)");
        if t[0] != t[1] {
            let w = if t[0] { 0 } else { 1 };
            self.pts[w] += NOTEN_PAY;
            self.pts[1 - w] -= NOTEN_PAY;
            note.push_str(&format!(
                ": J{} gata, J{} nu → {} puncte",
                w + 1,
                2 - w,
                NOTEN_PAY
            ));
        } else if t[0] {
            note.push_str(": ambii gata");
        } else {
            note.push_str(": niciunul gata");
        }
        self.log = format!("Mâna {}: {}", self.hand_idx, note);
        self.won = false;
        let d = self.dealer.idx();
        self.renchan = t[d];
        self.end_hand();
    }

    fn end_hand(&mut self) {
        self.phase = Phase::HandOver;
        if self.pts.iter().any(|&p| p < 0) {
            self.phase = Phase::Over;
            return;
        }
        // ultima mână: se termină dacă nu e renchan, sau dealerul e în frunte
        let d = self.dealer.idx();
        let last = self.hand_idx >= 4;
        if last && (!self.renchan || self.pts[d] > self.pts[1 - d]) {
            self.phase = Phase::Over;
        }
    }

    fn finish_win(
        &mut self,
        w: usize,
        loser: usize,
        res: &WinResult,
        tsumo: bool,
        ron_tile: Option<u8>,
    ) {
        let dealer_wins = w == self.dealer.idx();
        let pay = payment(res.base, dealer_wins, tsumo) + 300 * self.honba;
        self.pts[w] += pay + self.pool;
        self.pts[loser] -= pay;
        self.pool = 0;
        let kind = if tsumo { "tsumo" } else { "ron" };
        let what = if res.yakuman > 0 {
            format!("yakuman ×{}", res.yakuman)
        } else {
            format!("{} han {} fu", res.han, res.fu)
        };
        self.log = format!(
            "Mâna {}: J{} câștigă prin {}{} — {} ({}) → {} puncte",
            self.hand_idx,
            w + 1,
            kind,
            ron_tile
                .map(|t| format!(" pe {}", tile_name(t)))
                .unwrap_or_default(),
            what,
            res.yaku.join(", "),
            pay
        );
        self.won = true;
        self.renchan = dealer_wins;
        self.end_hand();
    }

    fn rivers_mark_taken(&mut self, p: usize) {
        if let Some(last) = self.rivers[p].last_mut() {
            last.1 = true;
        }
    }

    /// Seturile și perechile deja cuprinse în `closed` + melds sunt 4 seturi + pereche (14 piese)?
    fn discard_tile(&mut self, p: usize, t: u8, declare_riichi: bool) {
        self.closed[p][t as usize] -= 1;
        self.drawn[p] = None;
        self.rivers[p].push((t, false));
        self.no_discard.clear();
        self.furiten_temp[p] = false;
        self.decl = None;
        if declare_riichi {
            self.decl = Some(p);
            self.double_riichi[p] =
                !self.calls_made && self.rivers[p].len() == 1 && self.rivers[1 - p].len() <= 1;
            self.riichi[p] = true;
            self.ippatsu[p] = true;
            self.pts[p] -= 1000;
            self.pool += 1000;
        } else {
            self.ippatsu[p] = false;
        }
        self.reveal_pending_dora();
        self.last_discard = Some((if p == 0 { Player::One } else { Player::Two }, t));
        self.rinshan = false;
        let r = 1 - p;
        let rp = if r == 0 { Player::One } else { Player::Two };
        if self.reactions(r, t).is_empty() {
            self.start_turn(rp);
        } else {
            self.phase = Phase::React;
            self.side = rp;
        }
    }

    fn discards_allowed(&self, p: usize) -> Vec<u8> {
        let c = &self.closed[p];
        let all: Vec<u8> = (0..34u8).filter(|&t| c[t as usize] > 0).collect();
        let ok: Vec<u8> = all
            .iter()
            .copied()
            .filter(|t| !self.no_discard.contains(t))
            .collect();
        if ok.is_empty() {
            all
        } else {
            ok
        }
    }

    /// Mâna rămâne gata după ce arunc `t` (și e închisă)?
    fn riichi_ok(&self, p: usize, t: u8) -> bool {
        if self.riichi[p] || self.melds[p].iter().any(|m| m.open()) || self.pts[p] < 1000 {
            return false;
        }
        if self.wall.len() < 4 {
            return false;
        }
        let mut c = self.closed[p];
        c[t as usize] -= 1;
        !waits(&c, self.melds[p].len()).is_empty()
    }

    fn can_ankan_riichi(&self, p: usize, t: u8) -> bool {
        // după riichi: doar cu piesa trasă și fără a schimba așteptările
        if self.drawn[p] != Some(t) || self.closed[p][t as usize] != 4 {
            return false;
        }
        let mut before = self.closed[p];
        before[t as usize] -= 1;
        let w0 = waits(&before, self.melds[p].len());
        let mut after = self.closed[p];
        after[t as usize] -= 4;
        let w1 = waits(&after, self.melds[p].len() + 1);
        !w0.is_empty() && w0 == w1
    }

    // ── AI ──

    fn eff_value(&self, p: usize, c: &Counts, melds: usize, vis: &Counts) -> i32 {
        let sh = shanten(c, melds);
        let mut uke = 0;
        let mut cc = *c;
        for t in 0..34usize {
            if cc[t] >= 4 {
                continue;
            }
            let rem = 4 - vis[t].min(4) as i32;
            if rem <= 0 {
                continue;
            }
            cc[t] += 1;
            if shanten(&cc, melds) < sh {
                uke += rem;
            }
            cc[t] -= 1;
        }
        let _ = p;
        -1500 * sh + 25 * uke
    }

    /// Cât de periculoasă e aruncarea `t` față de riichi-ul adversarului.
    fn danger(&self, p: usize, t: u8) -> i32 {
        let o = 1 - p;
        if !self.riichi[o] {
            return 0;
        }
        let ro = &self.rivers[o];
        if ro.iter().any(|&(x, _)| x == t) {
            return 0;
        }
        let vis = self.visible(p);
        let mut d = if t >= 27 {
            match vis[t as usize] {
                0 | 1 => 35,
                2 => 20,
                _ => 5,
            }
        } else {
            let pos = (t % 9) as i32;
            let base = match pos {
                0 | 8 => 30,
                1 | 7 => 45,
                _ => 60,
            };
            let s = ((t / 9) * 9) as i32;
            let suji = |x: i32| -> bool {
                (0..9).contains(&x) && ro.iter().any(|&(y, _)| y as i32 == s + x)
            };
            match pos {
                0..=2 if suji(pos + 3) => base / 4,
                6..=8 if suji(pos - 3) => base / 4,
                3..=5 if suji(pos - 3) && suji(pos + 3) => base / 4,
                3..=5 if suji(pos - 3) || suji(pos + 3) => base * 2 / 3,
                _ => base,
            }
        };
        if self.dora_tiles(false).contains(&t) {
            d += 20;
        }
        d
    }

    fn open_has_yaku_potential(&self, p: usize) -> bool {
        let ms = &self.melds[p];
        if ms.iter().all(|m| !m.open()) {
            return true;
        }
        for m in ms {
            if m.kind != MKind::Chi
                && (is_dragon(m.tile) || m.tile == self.seat_wind(p) || m.tile == self.round_wind())
            {
                return true;
            }
        }
        let c = &self.closed[p];
        let mut all: Vec<u8> = (0..34u8)
            .flat_map(|t| std::iter::repeat_n(t, c[t as usize] as usize))
            .collect();
        for m in ms {
            all.extend(m.tiles());
        }
        if all.iter().all(|&t| !is_yaochuu(t)) {
            return true;
        }
        let suits: Vec<i32> = {
            let mut s: Vec<i32> = all.iter().map(|&t| suit_of(t)).filter(|&s| s < 3).collect();
            s.sort();
            s.dedup();
            s
        };
        if suits.len() <= 1 {
            return true;
        }
        ms.iter().filter(|m| m.kind != MKind::Chi).count() >= 3
    }

    /// Valoarea mâinii lui `p` din starea curentă (cu aruncare optimă dacă urmează una).
    fn hand_eval(&self, p: usize) -> i32 {
        let melds = self.melds[p].len();
        let vis = self.visible(p);
        let tot: u32 = self.closed[p].iter().map(|&x| x as u32).sum();
        let mut v;
        if tot % 3 == 2 {
            // 14 piese: cea mai bună aruncare
            let mut best = i32::MIN;
            let mut c = self.closed[p];
            for t in 0..34usize {
                if c[t] == 0 {
                    continue;
                }
                c[t] -= 1;
                let e = self.eff_value(p, &c, melds, &vis) - self.danger(p, t as u8) * def_weight();
                best = best.max(e);
                c[t] += 1;
            }
            v = best;
        } else {
            v = self.eff_value(p, &self.closed[p], melds, &vis);
        }
        v += 120 * self.dora_count(p, None, false);
        if !self.open_has_yaku_potential(p) {
            v -= 3500;
        } else if self.melds[p].iter().any(|m| m.open()) {
            v -= 250 * self.melds[p].iter().filter(|m| m.open()).count() as i32;
        }
        if self.riichi[p] {
            v += 500;
        }
        v
    }
}

fn def_weight() -> i32 {
    match level() {
        Level::Easy => 0,
        Level::Normal => 8,
        Level::Hard => 14,
        Level::Expert => 20,
    }
}

impl Game for Riichi {
    type Move = Move;

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = match who {
            Some(p) => Viewer::Fixed(p),
            None => Viewer::HotSeat,
        };
    }

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        let p = self.side.idx();
        match self.phase {
            Phase::Over => Vec::new(),
            Phase::HandOver => vec![Move::Next],
            Phase::Draw => vec![Move::Draw],
            Phase::DrawDead => vec![Move::DrawDead],
            Phase::React => self.reactions(p, self.last_discard.map(|x| x.1).unwrap_or(0)),
            Phase::Discard => {
                let mut v = Vec::new();
                if self.can_tsumo(p).is_some() {
                    v.push(Move::Tsumo);
                }
                if self.riichi[p] {
                    if let Some(t) = self.drawn[p] {
                        v.push(Move::Discard(t));
                        if self.kans < 4 && self.can_ankan_riichi(p, t) {
                            v.push(Move::Ankan(t));
                        }
                    }
                    return v;
                }
                for t in self.discards_allowed(p) {
                    v.push(Move::Discard(t));
                    if self.riichi_ok(p, t) {
                        v.push(Move::Riichi(t));
                    }
                }
                if self.kans < 4 && self.drawn[p].is_some() {
                    for t in 0..34u8 {
                        if self.closed[p][t as usize] == 4 {
                            v.push(Move::Ankan(t));
                        }
                        if self.closed[p][t as usize] >= 1
                            && self.melds[p]
                                .iter()
                                .any(|m| m.kind == MKind::Pon && m.tile == t)
                        {
                            v.push(Move::Kakan(t));
                        }
                    }
                }
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        let p = self.side.idx();
        let me = self.side;
        match m {
            Move::Next => {
                if self.renchan {
                    self.honba += 1;
                } else {
                    if self.won {
                        self.honba = 0;
                    } else {
                        self.honba += 1;
                    }
                    self.dealer = self.dealer.other();
                    self.hand_idx += 1;
                }
                self.new_hand();
            }
            Move::Draw => {
                let t = self.wall.pop().expect("zid");
                self.closed[p][t as usize] += 1;
                self.drawn[p] = Some(t);
                self.rinshan = false;
                self.phase = Phase::Discard;
            }
            Move::DrawDead => {
                let t = self.dead[self.rin_used];
                self.rin_used += 1;
                self.wall.pop();
                self.closed[p][t as usize] += 1;
                self.drawn[p] = Some(t);
                self.rinshan = true;
                self.phase = Phase::Discard;
            }
            Move::Tsumo => {
                let t = self.drawn[p].expect("piesă trasă");
                let res = self.can_tsumo(p).expect("câștig");
                self.finish_win(p, 1 - p, &res, true, None);
                let _ = t;
            }
            Move::Discard(t) => self.discard_tile(p, *t, false),
            Move::Riichi(t) => self.discard_tile(p, *t, true),
            Move::Ankan(t) => {
                self.closed[p][*t as usize] -= 4;
                self.melds[p].push(RMeld {
                    kind: MKind::Ankan,
                    tile: *t,
                });
                self.kans += 1;
                self.calls_made = true;
                self.ippatsu = [false; 2];
                self.drawn[p] = None;
                self.dora_n = (self.dora_n + 1).min(5);
                self.phase = Phase::DrawDead;
                self.log = format!("J{} face ankan {}", p + 1, tile_name(*t));
            }
            Move::Kakan(t) => {
                self.closed[p][*t as usize] -= 1;
                for mm in self.melds[p].iter_mut() {
                    if mm.kind == MKind::Pon && mm.tile == *t {
                        mm.kind = MKind::Kan;
                    }
                }
                self.kans += 1;
                self.calls_made = true;
                self.ippatsu = [false; 2];
                self.drawn[p] = None;
                self.pending_dora += 1;
                self.phase = Phase::DrawDead;
                self.log = format!("J{} face kakan {}", p + 1, tile_name(*t));
            }
            Move::Pass => {
                // ron ratat → furiten temporar (permanent după riichi)
                let t = self.last_discard.map(|x| x.1).unwrap_or(0);
                if self.can_ron_ignoring_furiten(p, t) {
                    if self.riichi[p] {
                        self.furiten_riichi[p] = true;
                    } else {
                        self.furiten_temp[p] = true;
                    }
                }
                self.start_turn(me);
            }
            Move::Ron => {
                let (dp, t) = self.last_discard.expect("aruncare");
                let res = self.can_ron(p, t).expect("câștig");
                // dacă piesa era aruncarea de riichi, declarația cade
                let d = dp.idx();
                if self.decl == Some(d) {
                    self.riichi[d] = false;
                    self.pts[d] += 1000;
                    self.pool -= 1000;
                }
                self.finish_win(p, d, &res, false, Some(t));
            }
            Move::Pon => {
                let (dp, t) = self.last_discard.expect("aruncare");
                self.closed[p][t as usize] -= 2;
                self.rivers_mark_taken(dp.idx());
                self.melds[p].push(RMeld {
                    kind: MKind::Pon,
                    tile: t,
                });
                self.calls_made = true;
                self.ippatsu = [false; 2];
                self.no_discard = vec![t];
                self.phase = Phase::Discard;
                self.drawn[p] = None;
                self.log = format!("J{} face pon {}", p + 1, tile_name(t));
            }
            Move::Kan => {
                let (dp, t) = self.last_discard.expect("aruncare");
                self.closed[p][t as usize] -= 3;
                self.rivers_mark_taken(dp.idx());
                self.melds[p].push(RMeld {
                    kind: MKind::Kan,
                    tile: t,
                });
                self.kans += 1;
                self.calls_made = true;
                self.ippatsu = [false; 2];
                self.pending_dora += 1;
                self.drawn[p] = None;
                self.phase = Phase::DrawDead;
                self.log = format!("J{} face kan {}", p + 1, tile_name(t));
            }
            Move::Chi(a, b) => {
                let (dp, t) = self.last_discard.expect("aruncare");
                self.closed[p][*a as usize] -= 1;
                self.closed[p][*b as usize] -= 1;
                self.rivers_mark_taken(dp.idx());
                let low = t.min(*a).min(*b);
                self.melds[p].push(RMeld {
                    kind: MKind::Chi,
                    tile: low,
                });
                self.calls_made = true;
                self.ippatsu = [false; 2];
                let mut forb = vec![t];
                if low == t && t % 9 + 3 <= 8 {
                    forb.push(t + 3);
                } else if low + 2 == t && t % 9 >= 3 {
                    forb.push(t - 3);
                }
                self.no_discard = forb;
                self.phase = Phase::Discard;
                self.drawn[p] = None;
                self.log = format!("J{} face chi {}", p + 1, tile_name(low));
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        Some(match self.pts[0].cmp(&self.pts[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let (me, op) = (p.idx(), p.other().idx());
        let mut s = self.pts[me] - self.pts[op];
        if matches!(self.phase, Phase::Over | Phase::HandOver) {
            return s * 2;
        }
        let sig = sig_of(&[
            self.closed[me]
                .iter()
                .fold(0u64, |a, &c| a.wrapping_mul(5).wrapping_add(c as u64)),
            self.rivers[0].len() as u64,
            self.rivers[1].len() as u64,
            self.pts[0] as u64,
        ]);
        s += self.hand_eval(me);
        // piesa tocmai aruncată de mine, față de riichi-ul adversarului
        if let Some((dp, t)) = self.last_discard {
            if dp == p && matches!(self.phase, Phase::React | Phase::Draw) {
                s -= self.danger(me, t) * def_weight();
            }
        }
        // riichi: merită când aștept ≥ 2 piese diferite și am o mână decentă
        s += noise(sig, 450);
        s
    }

    fn render(&self) -> String {
        let wind = |h: u8| if h <= 2 { "Est" } else { "Sud" };
        let mut s = format!(
            "Riichi Mahjong — {} {} | dealer: J{} | honba {} | bățe riichi {} | puncte: J1 {}, J2 {}\n",
            wind(self.hand_idx),
            if self.hand_idx <= 2 { self.hand_idx } else { self.hand_idx - 2 },
            self.dealer.idx() + 1,
            self.honba,
            self.pool / 1000,
            self.pts[0],
            self.pts[1]
        );
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        if self.phase == Phase::Over {
            s.push_str("Meci încheiat.\n");
            return s;
        }
        s.push_str(&format!(
            "Dora (indicator): {} | zid viu: {} piese\n",
            self.dora_indicators_vis()
                .iter()
                .map(|&t| tile_name(t))
                .collect::<Vec<_>>()
                .join(" "),
            self.wall.len()
        ));
        for p in 0..2 {
            let river: Vec<String> = self.rivers[p]
                .iter()
                .map(|&(t, taken)| {
                    if taken {
                        format!("({})", tile_name(t))
                    } else {
                        tile_name(t)
                    }
                })
                .collect();
            let melds: Vec<String> = self.melds[p].iter().map(|m| m.text()).collect();
            s.push_str(&format!(
                "J{}{}{}: aruncate {} | combinații {}\n",
                p + 1,
                if self.riichi[p] { " (RIICHI)" } else { "" },
                if self.furiten_temp[p] || self.furiten_riichi[p] {
                    " (furiten)"
                } else {
                    ""
                },
                if river.is_empty() {
                    "—".to_string()
                } else {
                    river.join(" ")
                },
                if melds.is_empty() {
                    "—".to_string()
                } else {
                    melds.join(" ")
                }
            ));
        }
        let v = self.viewer.who(self.side);
        let vi = v.idx();
        let mut c = self.closed[vi];
        let drawn = self.drawn[vi];
        if let Some(t) = drawn {
            c[t as usize] -= 1;
        }
        s.push_str(&format!(
            "Mâna ta (J{}): {}{}\n",
            vi + 1,
            tiles_text(&c),
            drawn
                .map(|t| format!(" + {}", tile_name(t)))
                .unwrap_or_default()
        ));
        let n: u32 = self.closed[1 - vi].iter().map(|&x| x as u32).sum();
        s.push_str(&format!("Jucătorul {}: {} piese în mână\n", 2 - vi, n));
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Draw => "trage".into(),
            Move::DrawDead => "trage-moarta".into(),
            Move::Discard(t) => tile_name(*t),
            Move::Riichi(t) => format!("riichi {}", tile_name(*t)),
            Move::Tsumo => "tsumo".into(),
            Move::Ankan(t) => format!("ankan {}", tile_name(*t)),
            Move::Kakan(t) => format!("kakan {}", tile_name(*t)),
            Move::Ron => "ron".into(),
            Move::Pon => "pon".into(),
            Move::Kan => "kan".into(),
            Move::Chi(a, b) => format!("chi {} {}", tile_name(*a), tile_name(*b)),
            Move::Pass => "pas".into(),
            Move::Next => "urmatoarea".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

impl Riichi {
    fn can_ron_ignoring_furiten(&self, p: usize, t: u8) -> bool {
        score_win(&self.ctx(p, t, false)).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// "123m456p77z" → listă de piese.
    fn hand(s: &str) -> Vec<u8> {
        let mut out = Vec::new();
        let mut digits: Vec<u8> = Vec::new();
        for ch in s.chars() {
            if ch.is_ascii_digit() {
                digits.push(ch as u8 - b'0');
            } else {
                let base = match ch {
                    'm' => 0,
                    'p' => 9,
                    's' => 18,
                    'z' => 27,
                    _ => panic!("sufix"),
                };
                for d in digits.drain(..) {
                    out.push(base + d - 1);
                }
            }
        }
        out
    }
    fn cnt(s: &str) -> Counts {
        counts_of(&hand(s))
    }
    fn ctx(s: &str, win: &str, tsumo: bool) -> WinCtx {
        WinCtx {
            closed: cnt(s),
            melds: vec![],
            win_tile: hand(win)[0],
            tsumo,
            seat_wind: 28,
            round_wind: 27,
            riichi: false,
            double_riichi: false,
            ippatsu: false,
            haitei: false,
            houtei: false,
            rinshan: false,
            tenhou: false,
            chiihou: false,
            dora: 0,
        }
    }

    #[test]
    fn completeness_and_waits() {
        assert!(is_complete(&cnt("123m456p789s123s11z"), 0));
        assert!(is_complete(&cnt("1122m3344p5566s77z"), 0)); // 7 perechi
        assert!(is_complete(&cnt("19m19p19s12345677z"), 0)); // kokushi (14)
        assert!(!is_complete(&cnt("123m456p789s123s12z"), 0));
        // 12m + 3m: ryanmen / penchan
        assert_eq!(waits(&cnt("12m456p789s123s11z"), 0), vec![2]);
        assert_eq!(waits(&cnt("23m456p789s123s11z"), 0), vec![0, 3]);
        assert_eq!(waits(&cnt("13m456p789s123s11z"), 0), vec![1]);
        // shanpon
        let w = waits(&cnt("11m22m456p789s123s"), 0);
        assert_eq!(w, vec![0, 1]);
    }

    #[test]
    fn shanten_values() {
        assert_eq!(shanten(&cnt("123m456p789s123s11z"), 0), -1);
        assert_eq!(shanten(&cnt("23m456p789s123s11z"), 0), 0);
        assert_eq!(shanten(&cnt("2468m2468p2468s1z"), 0), 4);
        assert_eq!(shanten(&cnt("1122m3344p5566s7z"), 0), 0); // chiitoi: 6 perechi + 1
        assert_eq!(shanten(&cnt("19m19p19s1234567z"), 0), 0); // kokushi 13 cu toate
        assert!(shanten(&cnt("19m19p19s123456z1m"), 0) <= 1);
        // cu seturi deja făcute
        assert_eq!(shanten(&cnt("456p789s11z"), 2), -1);
        assert_eq!(shanten(&cnt("456p79s11z"), 2), 0);
    }

    #[test]
    fn pinfu_tanyao_ron_30fu_nondealer() {
        // 234m 567p 345s 678s 55m → așteptare ryanmen 2-3m... câștig pe 4m? mână: 23m +4m
        let mut c = ctx("234m567p345s678s55m", "4m", false);
        c.riichi = true;
        let r = score_win(&c).unwrap();
        // riichi 1 + pinfu 1 + tanyao 1 = 3 han, 30 fu (ron închis)
        assert_eq!((r.han, r.fu), (3, 30), "{:?}", r.yaku);
        assert_eq!(payment(r.base, false, false), 3900);
        assert_eq!(payment(r.base, true, false), 5800);
    }

    #[test]
    fn pinfu_tsumo_is_20_fu() {
        let mut c = ctx("234m567p345s678s55m", "4m", true);
        c.closed = cnt("234m567p345s678s55m");
        let r = score_win(&c).unwrap();
        // tsumo închis 1 + pinfu 1 + tanyao 1 = 3 han 20 fu
        assert_eq!((r.han, r.fu), (3, 20), "{:?}", r.yaku);
        assert_eq!(r.base, 20 * 32);
        // non-dealer tsumo: 1300 + 2 × 700 = 2700 în 4 jucători → suma
        assert_eq!(payment(r.base, false, true), 1300 + 2 * 700);
    }

    #[test]
    fn chiitoitsu_25fu_and_values() {
        let mut c = ctx("1133m5577p2288s44z", "4z", false);
        c.riichi = true;
        let r = score_win(&c).unwrap();
        assert_eq!((r.han, r.fu), (3, 25), "{:?}", r.yaku);
        assert_eq!(payment(r.base, false, false), 3200);
    }

    #[test]
    fn mangan_and_limits() {
        assert_eq!(base_points(4, 40, 0), 2000);
        assert_eq!(base_points(3, 30, 0), 960);
        assert_eq!(base_points(1, 30, 0), 240);
        assert_eq!(payment(240, false, false), 1000);
        assert_eq!(payment(240, true, false), 1500);
        assert_eq!(payment(480, true, false), 2900);
        assert_eq!(base_points(6, 30, 0), 3000);
        assert_eq!(base_points(8, 30, 0), 4000);
        assert_eq!(base_points(11, 30, 0), 6000);
        assert_eq!(base_points(13, 30, 0), 8000);
        assert_eq!(payment(base_points(0, 0, 1), false, false), 32000);
        assert_eq!(payment(base_points(0, 0, 1), true, false), 48000);
    }

    #[test]
    fn kokushi_and_yakuman() {
        let c = ctx("119m19p19s1234567z", "1m", false);
        let r = score_win(&c).unwrap();
        assert_eq!(r.yakuman, 1);
        assert!(r.yaku[0].contains("kokushi"));
        // daisangen
        let c = ctx("555z666z777z123m11s", "3m", false);
        let r = score_win(&c).unwrap();
        assert!(
            r.yakuman >= 1 && r.yaku.iter().any(|y| y == "daisangen"),
            "{:?}",
            r.yaku
        );
    }

    #[test]
    fn yakuhai_honitsu_and_open_hand() {
        // pung de dragon roșu deschis + restul numai caractere
        let mut c = ctx("123m456m789m11m", "9m", false);
        c.melds = vec![RMeld {
            kind: MKind::Pon,
            tile: 33,
        }];
        let r = score_win(&c).expect("yakuhai + honitsu");
        assert!(
            r.yaku.iter().any(|y| y.starts_with("yakuhai")),
            "{:?}",
            r.yaku
        );
        assert!(
            r.yaku.iter().any(|y| y.starts_with("honitsu")),
            "{:?}",
            r.yaku
        );
        assert!(
            r.yaku.iter().any(|y| y.starts_with("ittsu 1")),
            "{:?}",
            r.yaku
        );
        // fără yaku → None: pung de Nord (vânt străin) și culori mixte
        let mut c2 = ctx("123m456p789s11m", "1m", false);
        c2.melds = vec![RMeld {
            kind: MKind::Pon,
            tile: 30,
        }];
        assert!(score_win(&c2).is_none());
    }

    #[test]
    fn dora_counted_only_with_a_yaku() {
        let mut c = ctx("123m456p789s123s11z", "1z", false);
        c.dora = 3;
        // fără riichi/alt yaku: 11z (Est) pereche nu dă yaku → fără yaku
        assert!(score_win(&c).is_none());
        c.riichi = true;
        let r = score_win(&c).unwrap();
        assert_eq!(r.han, 4);
    }

    #[test]
    fn furiten_blocks_ron() {
        let mut g = Riichi::with_seed(1);
        g.set_hand_for_test(0, &hand("12m456p789s123s11z"));
        g.river_for_test(0, &hand("3m"));
        assert!(g.is_furiten(0));
        g.river_for_test(0, &hand("9p"));
        assert!(!g.is_furiten(0));
        g.furiten_temp[0] = true;
        assert!(g.is_furiten(0));
    }

    #[test]
    fn riichi_flow_with_ron_and_stick() {
        let mut g = Riichi::with_seed(2);
        g.set_hand_for_test(0, &hand("23m456p789s123s11z"));
        g.set_hand_for_test(1, &hand("1m4p7p2s5s8s1z2z3z4z5z6z7z"));
        g.force_phase_discard_for_test(Player::One, 31); // 5z
        let mv = Game::legal_moves(&g);
        assert!(mv.contains(&Move::Riichi(31)));
        assert!(mv.contains(&Move::Discard(31)));
        g.apply(&Move::Riichi(31));
        assert_eq!(g.pts[0], START_POINTS - 1000);
        assert_eq!(g.pool, 1000);
        assert!(g.riichi[0]);
        assert_eq!(g.to_move(), Player::Two);
        assert_eq!(Game::legal_moves(&g), vec![Move::Draw]);
        g.apply(&Move::Draw);
        let mv = Game::legal_moves(&g);
        assert!(mv.contains(&Move::Discard(0)));
        g.apply(&Move::Discard(0));
        // J1 e în riichi și gata pe 1m / 4m: ron
        assert_eq!(g.to_move(), Player::One);
        let mv = Game::legal_moves(&g);
        assert!(
            mv.contains(&Move::Ron) && mv.contains(&Move::Pass),
            "{mv:?}"
        );
        assert!(!mv.contains(&Move::Pon));
        let before = g.pts[0];
        g.apply(&Move::Ron);
        assert!(g.pts[0] > before);
        assert_eq!(g.pool, 0);
        assert_eq!(g.pts[0] + g.pts[1], 2 * START_POINTS);
        assert!(g.log.contains("riichi") || g.log.contains("ron"));
    }

    #[test]
    fn missed_ron_makes_temporary_furiten() {
        let mut g = Riichi::with_seed(2);
        g.set_hand_for_test(0, &hand("234m567p345s678s5m")); // tanki pe 5m, tanyao
        g.set_hand_for_test(1, &hand("4m4p7p2s5s8s1z2z3z4z5z6z7z"));
        g.force_phase_discard_for_test(Player::One, 33); // 7z
        g.apply(&Move::Discard(33));
        assert_eq!(g.to_move(), Player::Two);
        g.apply(&Move::Draw);
        // J2 aruncă 5m dacă îl are: forțăm piesa în mână
        g.closed[1][4] += 1;
        g.drawn[1] = Some(4);
        g.apply(&Move::Discard(4));
        assert_eq!(g.to_move(), Player::One);
        let mv = Game::legal_moves(&g);
        assert!(mv.contains(&Move::Ron), "{mv:?}");
        g.apply(&Move::Pass);
        assert!(g.furiten_temp[0]);
        assert!(g.is_furiten(0));
        // la următoarea ta aruncare furiten-ul temporar dispare
        g.apply(&Move::Draw);
        g.apply(&Move::Discard(4.min(g.drawn[0].unwrap_or(4))));
        assert!(!g.furiten_temp[0]);
    }

    #[test]
    fn exhaustive_draw_pays_noten() {
        let mut g = Riichi::with_seed(3);
        g.set_hand_for_test(0, &hand("23m456p789s123s11z"));
        g.set_hand_for_test(1, &hand("1m4m7m1p4p7p1s4s7s1z2z3z4z"));
        g.wall.clear();
        g.exhaustive_draw();
        assert_eq!(g.pts, [START_POINTS + NOTEN_PAY, START_POINTS - NOTEN_PAY]);
        // dealerul (J1) e gata → renchan
        assert!(g.renchan);
        assert_eq!(Game::legal_moves(&g), vec![Move::Next]);
        g.apply(&Move::Next);
        assert_eq!(g.dealer(), Player::One);
        assert_eq!(g.honba, 1);
    }

    #[test]
    fn dora_indicator_order() {
        assert_eq!(next_dora(8), 0);
        assert_eq!(next_dora(0), 1);
        assert_eq!(next_dora(30), 27);
        assert_eq!(next_dora(33), 31);
        assert_eq!(next_dora(31), 32);
    }

    #[test]
    fn chi_pon_kan_options_and_kuikae() {
        let mut g = Riichi::with_seed(4);
        g.set_hand_for_test(1, &hand("23m44m5m1p9p1s9s1z2z3z4z"));
        g.last_discard = Some((Player::One, 3)); // 4m
        let r = g.reactions(1, 3);
        assert!(r.contains(&Move::Pon));
        assert!(r.contains(&Move::Chi(1, 2))); // 2m 3m + 4m
        assert!(r.contains(&Move::Chi(2, 4))); // 3m 5m + 4m
        assert!(r.contains(&Move::Pass));
        assert!(!r.contains(&Move::Kan));
    }

    #[test]
    fn full_game_random_and_conservation() {
        for seed in 1..4u64 {
            let mut g = Riichi::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() && n < 30_000 {
                let mv = Game::legal_moves(&g);
                assert!(!mv.is_empty(), "{}", Game::render(&g));
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
                // conservarea punctelor (cu bățele de riichi)
                assert_eq!(g.pts[0] + g.pts[1] + g.pool, 2 * START_POINTS);
            }
            assert!(g.outcome().is_some(), "seed {seed} nu s-a terminat");
        }
    }

    #[test]
    fn render_hides_opponent_and_wall() {
        let g = Riichi::with_seed(5);
        let r = Game::render(&g);
        assert!(r.contains("Mâna ta (J1)"));
        assert!(r.contains("piese în mână"));
        let mut g2 = g.clone();
        g2.set_viewer(Some(Player::Two));
        let r2 = Game::render(&g2);
        assert!(r2.contains("Mâna ta (J2)"));
        assert_ne!(
            r.lines().find(|l| l.starts_with("Mâna ta")),
            r2.lines().find(|l| l.starts_with("Mâna ta"))
        );
    }

    #[test]
    fn ai_plays_a_hand() {
        let mut g = Riichi::with_seed(7);
        let mut n = 0;
        let start = g.hand_idx;
        while g.outcome().is_none() && n < 800 && g.hand_idx == start {
            let m = polytabletop_core::ai::best_move(&g, n).unwrap();
            g.apply(&m);
            n += 1;
        }
        assert!(n > 10);
    }
}
