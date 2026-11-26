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
//! Sanma (riichi mahjong pentru trei jucători), adaptat pentru 2 jucători pe baza motorului `riichi`.
//!
//! Regulile de bază sunt cele din `riichi` (yaku, fu/han, riichi, furiten, dora, kan, puncte; vezi acolo), cu schimbările
//! caracteristice sanma:
//! * Pachetul are **108 piese**: lipsesc 2m–8m (rămân 1m și 9m), restul ca la riichi (cercuri, bambus, vânturi, dragoni).
//! * **Fără chi** (nu există secvențe în caractere; pon și kan rămân; chi la cercuri/bambus nu e permis).
//! * **Kita** (nukidora): în tura ta poți declara o piesă Nord (`kita`) ca bonus: se pune deoparte, valorează ca o dora (+1 han, doar cu yaku)
//!   și iei o piesă din zidul mort (rinshan). Se pot declara cel mult 4 piese din zidul mort (kita + kan la un loc).
//! * Indicatorul de dora: după 1m urmează 9m și după 9m urmează 1m.
//! * Rămân simplificările din `riichi`: 2 jucători (nu 3), plata ca la 2 jucători, fără chankan pe kita, fără
//!   piese roșii. Informația ascunsă (zid, zid mort, mâna adversarului) e tratată ca la `riichi`; AI-ul vede doar mâna lui.

use crate::cards::Viewer;
use crate::cards52::shuffle;
use crate::p2_util::{noise, sig_of};
use crate::riichi::{
    payment, score_win, shanten, tile_name, waits, Counts, MKind, RMeld, WinCtx, WinResult,
    NOTEN_PAY, START_POINTS,
};
use polytabletop_core::ai::{level, Level};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

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
fn suit_of(t: u8) -> i32 {
    if t < 27 {
        (t / 9) as i32
    } else {
        3
    }
}
fn next_dora(ind: u8) -> u8 {
    match ind {
        0 => 8,
        8 => 0,
        1..=26 => {
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
fn meld_text(m: &RMeld) -> String {
    let t = m
        .tiles()
        .iter()
        .map(|&x| tile_name(x))
        .collect::<Vec<_>>()
        .join("");
    if m.kind == MKind::Ankan {
        format!("[{t}]")
    } else {
        format!("({t})")
    }
}

// ───────────────────────────────── jocul ─────────────────────────────────

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Draw,
    DrawDead,
    Discard(u8),
    Sanma(u8),
    Tsumo,
    Ankan(u8),
    Kakan(u8),
    Ron,
    Pon,
    Kan,
    Chi(u8, u8),
    Kita,
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
pub struct Sanma {
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
    kita: [u8; 2],
    rng: Rng,
    viewer: Viewer,
}

impl Default for Sanma {
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

impl Sanma {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Sanma {
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
            kita: [0; 2],
            rng: Rng::new(seed),
            viewer: Viewer::Fixed(Player::One),
        };
        g.new_hand();
        g
    }

    fn new_hand(&mut self) {
        let mut w: Vec<u8> = (0..34u8)
            .filter(|t| !(1..=7).contains(t))
            .flat_map(|t| [t; 4])
            .collect();
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
        self.kita = [0; 2];
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
        let mut n = self.kita[p] as i32;
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
        v[30] += self.kita[0] + self.kita[1];
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
            if c[t as usize] >= 3 && self.kans < 4 && self.rin_used < 4 {
                v.push(Move::Kan);
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

impl Game for Sanma {
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
                        if self.kans < 4 && self.rin_used < 4 && self.can_ankan_riichi(p, t) {
                            v.push(Move::Ankan(t));
                        }
                    }
                    return v;
                }
                if self.rin_used < 4 && self.closed[p][30] >= 1 && self.drawn[p].is_some() {
                    v.push(Move::Kita);
                }
                for t in self.discards_allowed(p) {
                    v.push(Move::Discard(t));
                    if self.riichi_ok(p, t) {
                        v.push(Move::Sanma(t));
                    }
                }
                if self.kans < 4 && self.rin_used < 4 && self.drawn[p].is_some() {
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
            Move::Sanma(t) => self.discard_tile(p, *t, true),
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
            Move::Kita => {
                self.closed[p][30] -= 1;
                self.kita[p] += 1;
                self.drawn[p] = None;
                self.phase = Phase::DrawDead;
                self.log = format!("J{} declară kita (nord)", p + 1);
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
        s += 350 * (self.kita[me] as i32 - self.kita[op] as i32);
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
            "Sanma (riichi, 108 piese) — {} {} | dealer: J{} | honba {} | bățe riichi {} | puncte: J1 {}, J2 {}\n",
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
            let melds: Vec<String> = self.melds[p].iter().map(meld_text).collect();
            s.push_str(&format!(
                "J{}{}{}: aruncate {} | combinații {} | kita {}\n",
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
                },
                self.kita[p]
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
            Move::Sanma(t) => format!("riichi {}", tile_name(*t)),
            Move::Tsumo => "tsumo".into(),
            Move::Ankan(t) => format!("ankan {}", tile_name(*t)),
            Move::Kakan(t) => format!("kakan {}", tile_name(*t)),
            Move::Ron => "ron".into(),
            Move::Pon => "pon".into(),
            Move::Kan => "kan".into(),
            Move::Chi(a, b) => format!("chi {} {}", tile_name(*a), tile_name(*b)),
            Move::Kita => "kita".into(),
            Move::Pass => "pas".into(),
            Move::Next => "urmatoarea".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

impl Sanma {
    fn can_ron_ignoring_furiten(&self, p: usize, t: u8) -> bool {
        score_win(&self.ctx(p, t, false)).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn play_first(g: &mut Sanma, name: &str) {
        let m = g
            .legal_moves()
            .into_iter()
            .find(|m| g.format_move(m) == name)
            .unwrap_or_else(|| panic!("mutare ilegală {name}"));
        g.apply(&m);
    }

    #[test]
    fn wall_has_108_tiles_without_2m_to_8m() {
        let g = Sanma::with_seed(1);
        let mut total: u32 = g.wall.len() as u32 + g.dead.len() as u32;
        for p in 0..2 {
            total += g.closed[p].iter().map(|&x| x as u32).sum::<u32>();
        }
        assert_eq!(total, 108);
        for p in 0..2 {
            for t in 1..=7 {
                assert_eq!(g.closed[p][t], 0);
            }
        }
        assert!(g
            .wall
            .iter()
            .chain(g.dead.iter())
            .all(|&t| !(1..=7).contains(&t)));
        assert_eq!(next_dora(0), 8);
        assert_eq!(next_dora(8), 0);
    }

    #[test]
    fn no_chi_and_kita_is_a_bonus_dora() {
        let mut g = Sanma::with_seed(2);
        // J1 aruncă 4p, J2 are 3p 5p: în riichi ar putea face chi, în sanma nu
        g.closed[1] = [0; 34];
        for t in [9 + 2, 9 + 4, 18, 19, 20, 22, 23, 24, 0, 8, 27, 28, 29] {
            g.closed[1][t] += 1;
        }
        let r = g.reactions(1, 12);
        assert!(!r.iter().any(|m| matches!(m, Move::Chi(_, _))), "{r:?}");
        // kita
        g.side = Player::One;
        g.closed[0] = [0; 34];
        for t in [0, 8, 9, 10, 11, 18, 19, 20, 27, 27, 28, 28, 30] {
            g.closed[0][t] += 1;
        }
        g.closed[0][30] += 1;
        g.drawn[0] = Some(30);
        g.phase = Phase::Discard;
        assert!(g.legal_moves().contains(&Move::Kita));
        let before: u32 = g.closed[0].iter().map(|&x| x as u32).sum();
        play_first(&mut g, "kita");
        assert_eq!(g.kita[0], 1);
        assert_eq!(g.phase, Phase::DrawDead);
        play_first(&mut g, "trage-moarta");
        let after: u32 = g.closed[0].iter().map(|&x| x as u32).sum();
        assert_eq!(before, after);
        assert!(g.dora_count(0, None, false) >= 1);
    }

    #[test]
    fn random_games_end_and_conserve() {
        for seed in 1..=6u64 {
            let mut g = Sanma::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                assert!(!mv.is_empty(), "{}", g.render());
                let k = mv.len();
                g.apply(&mv[rng.below(k)]);
                n += 1;
                assert!(n < 4000, "joc prea lung");
            }
        }
    }

    #[test]
    fn hidden_info_and_ai() {
        let mut g = Sanma::with_seed(7);
        g.set_viewer(Some(Player::One));
        let a = g.render();
        g.set_viewer(Some(Player::Two));
        assert_ne!(a, g.render());
        g.set_viewer(Some(Player::One));
        let m = polytabletop_core::GameObj::ai_move(&g, 2).unwrap();
        assert!(g.legal_moves().iter().any(|x| g.format_move(x) == m));
    }
}
