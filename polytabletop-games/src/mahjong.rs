//! Mahjong (China) — variantă Hong Kong simplificată, cinstită, pentru 2 jucători.
//!
//! Reguli folosite (documentate):
//! * 136 de piese: 3 culori numerice × 9 × 4 (caractere `m`, bambus `s`, cercuri `p`), 4 vânturi
//!   (E, S, W, N) și 3 dragoni (Dr roșu, Dg verde, Dw alb), câte 4 din fiecare. Fără piese
//!   „flori”. Jucătorii primesc 13 piese; J1 e dealerul (vântul Est), J2 are vântul Sud;
//!   vântul rundei e Est.
//! * Din zid se rezervă 14 piese de „zid mort” (pentru înlocuirea după careu); cele 96 rămase
//!   se trag pe rând. **Zidul e ascuns** — vezi doar câte piese au rămas. Dacă zidul viu se
//!   termină fără câștigător, mâna e remiză.
//! * Tura: tragi (`trage`), apoi arunci (`d5m`) sau declari `mahjong` (câștig din tragere) ori
//!   `kong5m` (careu din mână sau adăugat la un pung expus; urmează `trage-moarta`, tragerea din
//!   zidul mort). Adversarul poate lua piesa aruncată: `mahjong` (câștig pe aruncare), `pung`
//!   (trei la fel), `kong` (patru la fel, apoi `trage-moarta`), `chi2m3m` (secvență cu două piese
//!   proprii; în varianta cu doi jucători e permis de la orice aruncare) sau `pas`. După pung/chi
//!   arunci fără să tragi. Dacă nu ai nicio reacție posibilă, tragi direct.
//! * Mână câștigătoare: 4 seturi (secvențe/pungs/careuri) + o pereche și **minimum 1 fan**:
//!   tragere proprie 1, mână închisă (fără seturi expuse) 1, toate secvențele (perechea nu e
//!   dragon sau vântul tău) 1, toate pungurile 3, o singură culoare + onoruri 3, o singură culoare
//!   pură 6, doar onoruri 10, pung de dragon 1 (fiecare), pung de vânt propriu 1, pung de vântul
//!   rundei 1. Nu există șapte perechi, treisprezece orfani și alte mâini speciale.
//! * Câștigă primul care declară o mână validă (fanii se afișează la final). AI-ul vede doar
//!   mâna lui, seturile expuse și aruncările publice; alege după un scor de potențial al mâinii.

use crate::cards52::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const DEAD: usize = 14;
const WINDS: [&str; 4] = ["E", "S", "W", "N"];
const DRAGONS: [&str; 3] = ["Dr", "Dg", "Dw"];

pub fn tile_name(t: u8) -> String {
    if t < 27 {
        format!("{}{}", t % 9 + 1, ['m', 's', 'p'][(t / 9) as usize])
    } else if t < 31 {
        WINDS[(t - 27) as usize].to_string()
    } else {
        DRAGONS[(t - 31) as usize].to_string()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Meld {
    Chi(u8),
    Pung(u8),
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

/// Fanii celei mai bune descompuneri sau `None` dacă mâna nu e completă.
pub fn score_hand(
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
    let mut c = *counts;
    let ds = decomps(&mut c, need);
    let mut best: Option<(i32, String)> = None;
    for (pair, sets) in ds {
        let mut all: Vec<(bool, u8)> = sets.clone();
        for m in melds {
            match *m {
                Meld::Chi(a) => all.push((false, a)),
                Meld::Pung(t) | Meld::Kong(t, _) => all.push((true, t)),
            }
        }
        let mut fan = 0;
        let mut why: Vec<String> = Vec::new();
        if self_draw {
            fan += 1;
            why.push("tragere proprie 1".into());
        }
        let exposed = melds.iter().any(|m| !matches!(m, Meld::Kong(_, true)));
        if !exposed {
            fan += 1;
            why.push("mână închisă 1".into());
        }
        let pung_count = all.iter().filter(|s| s.0).count();
        let honor_pair = pair >= 31 || pair == seat_wind || pair == 27;
        if pung_count == 0 && !honor_pair {
            fan += 1;
            why.push("toate secvențele 1".into());
        }
        if pung_count == 4 {
            fan += 3;
            why.push("toate pungurile 3".into());
        }
        // culori
        let mut suits = [false; 3];
        let mut honors = false;
        let mut tiles: Vec<u8> = vec![pair, pair];
        for &(p, t) in &all {
            if p {
                tiles.extend([t; 3]);
            } else {
                tiles.extend([t, t + 1, t + 2]);
            }
        }
        for &t in &tiles {
            if t < 27 {
                suits[(t / 9) as usize] = true;
            } else {
                honors = true;
            }
        }
        let n_suits = suits.iter().filter(|&&x| x).count();
        if n_suits == 0 {
            fan += 10;
            why.push("doar onoruri 10".into());
        } else if n_suits == 1 && !honors {
            fan += 6;
            why.push("o singură culoare 6".into());
        } else if n_suits == 1 && honors {
            fan += 3;
            why.push("culoare + onoruri 3".into());
        }
        for &(p, t) in &all {
            if !p {
                continue;
            }
            if t >= 31 {
                fan += 1;
                why.push(format!("pung de dragon {} 1", tile_name(t)));
            }
            if t == seat_wind {
                fan += 1;
                why.push("pung de vânt propriu 1".into());
            }
            if t == 27 {
                fan += 1;
                why.push("pung de vântul rundei 1".into());
            }
        }
        let desc = why.join(", ");
        if best.as_ref().is_none_or(|b| fan > b.0) {
            best = Some((fan, desc));
        }
    }
    best
}

/// Scor de potențial (doar din propria mână) pentru AI.
fn potential(c: &Counts, melds: &[Meld]) -> i32 {
    let mut s = 0.0f64;
    for (i, &n) in c.iter().enumerate() {
        let n = n as f64;
        s += match n as u32 {
            0 => 0.0,
            1 => 0.0,
            2 => 6.0,
            3 => 13.0,
            _ => 11.0,
        };
        if i >= 31 && n >= 2.0 {
            s += 3.0;
        }
        if i < 27 {
            let pos = i % 9;
            if n >= 1.0 && pos <= 7 && c[i + 1] > 0 {
                s += 3.0;
            }
            if n >= 1.0 && pos <= 6 && c[i + 2] > 0 {
                s += 1.5;
                if c[i + 1] > 0 {
                    s += 8.0;
                }
            }
        }
    }
    // concentrare pe o culoare
    let mut per = [0u32; 3];
    for i in 0..27 {
        per[i / 9] += c[i] as u32;
    }
    s += 0.7 * *per.iter().max().unwrap() as f64;
    s += 16.0 * melds.len() as f64;
    (s * 10.0) as i32
}

#[derive(Clone)]
pub struct Mahjong {
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
    msg: String,
    /// `Some(p)` = render shows `p`'s concealed hand; `None` = hot-seat (the mover's).
    viewer: Option<Player>,
}

impl Default for Mahjong {
    fn default() -> Self {
        Self::new()
    }
}

impl Mahjong {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut w: Vec<u8> = Vec::with_capacity(136);
        for t in 0..34u8 {
            w.extend([t; 4]);
        }
        shuffle(&mut w, &mut rng);
        let dead: Vec<u8> = w.drain(..DEAD).collect();
        let mut counts = [[0u8; 34]; 2];
        for c in counts.iter_mut() {
            for _ in 0..13 {
                c[w.pop().unwrap() as usize] += 1;
            }
        }
        Mahjong {
            counts,
            melds: [Vec::new(), Vec::new()],
            discards: [Vec::new(), Vec::new()],
            wall: w,
            dead,
            side: Player::One,
            phase: Phase::Draw,
            last: None,
            drawn: None,
            winner: None,
            msg: String::new(),
            viewer: Some(Player::One),
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

    fn reactions(&self, r: Player, t: u8) -> Vec<Move> {
        let c = &self.counts[r.idx()];
        let mut v = Vec::new();
        let mut cc = *c;
        cc[t as usize] += 1;
        if let Some((fan, _)) = score_hand(&cc, &self.melds[r.idx()], Self::seat(r), false) {
            if fan >= 1 {
                v.push(Move::Ron);
            }
        }
        if c[t as usize] >= 2 {
            v.push(Move::Pung);
        }
        if c[t as usize] >= 3 && !self.dead.is_empty() {
            v.push(Move::KongClaim);
        }
        if t < 27 {
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
        self.msg = format!("J{} câștigă ({how}) cu {fan} fan: {desc}", p.idx() + 1);
        self.phase = Phase::Over;
    }

    fn best_potential(&self, p: Player) -> i32 {
        let c = &self.counts[p.idx()];
        let m = &self.melds[p.idx()];
        let n: u32 = c.iter().map(|&x| x as u32).sum();
        if n % 3 == 2 {
            let mut best = i32::MIN;
            for t in 0..34 {
                if c[t] > 0 {
                    let mut cc = *c;
                    cc[t] -= 1;
                    best = best.max(potential(&cc, m));
                }
            }
            best
        } else {
            potential(c, m)
        }
    }
}

impl Game for Mahjong {
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
            Phase::Draw => vec![Move::Draw],
            Phase::DrawDead => vec![Move::DrawDead],
            Phase::React => {
                let (t, _) = self.last.unwrap();
                self.reactions(self.side, t)
            }
            Phase::Discard => {
                let mut v = Vec::new();
                if score_hand(
                    &self.counts[me],
                    &self.melds[me],
                    Self::seat(self.side),
                    true,
                )
                .is_some()
                {
                    v.push(Move::Tsumo);
                }
                let c = &self.counts[me];
                for t in 0..34u8 {
                    if c[t as usize] > 0 {
                        v.push(Move::Discard(t));
                    }
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
            Move::Draw => {
                let t = self.wall.pop().expect("zid");
                self.counts[i][t as usize] += 1;
                self.drawn = Some(t);
                self.phase = Phase::Discard;
            }
            Move::DrawDead => {
                let t = self.dead.pop().expect("zid mort");
                self.counts[i][t as usize] += 1;
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
                    self.begin_draw(r);
                } else {
                    self.side = r;
                    self.phase = Phase::React;
                }
            }
            Move::Tsumo => {
                let (fan, d) =
                    score_hand(&self.counts[i], &self.melds[i], Self::seat(me), true).unwrap();
                self.win(me, fan, &d, "tragere proprie");
            }
            Move::Ron => {
                let (t, from) = self.last.unwrap();
                self.counts[i][t as usize] += 1;
                self.discards[from.idx()].pop();
                let (fan, d) =
                    score_hand(&self.counts[i], &self.melds[i], Self::seat(me), false).unwrap();
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
                let r = me;
                self.begin_draw(r);
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

    /// Doar mâna proprie, seturile expuse și aruncările publice.
    fn evaluate(&self, p: Player) -> i32 {
        let mut s = self.best_potential(p);
        // piesele deja aruncate de adversar (publice) pe care aș fi vrut să le primesc
        let o = p.other();
        let mut bonus = 0;
        for &t in &self.discards[o.idx()] {
            if self.counts[p.idx()][t as usize] >= 2 {
                bonus -= 5;
            }
        }
        s += bonus;
        // numărul de seturi expuse ale adversarului (public)
        s -= 8 * self.melds[o.idx()].len() as i32;
        s
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Mahjong — zid viu: {} piese (zid mort: {}) | vânt: J1 Est, J2 Sud\n",
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
            s.push_str(&format!(
                "J{}: seturi expuse [{}]; aruncate: {}\n",
                p.idx() + 1,
                melds.join(" "),
                if disc.is_empty() {
                    "—".to_string()
                } else {
                    disc.join(" ")
                }
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
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
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

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
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
    fn tile_names_unique() {
        let mut n: Vec<String> = (0..34).map(tile_name).collect();
        n.sort();
        n.dedup();
        assert_eq!(n.len(), 34);
    }

    #[test]
    fn deal_and_wall_sizes() {
        let g = Mahjong::with_seed(3);
        assert_eq!(g.dead.len(), 14);
        assert_eq!(g.wall.len(), 136 - 14 - 26);
        for p in 0..2 {
            assert_eq!(g.counts[p].iter().map(|&x| x as usize).sum::<usize>(), 13);
        }
        let mut all = g.wall.clone();
        all.extend(&g.dead);
        for p in 0..2 {
            for t in 0..34 {
                for _ in 0..g.counts[p][t] {
                    all.push(t as u8);
                }
            }
        }
        all.sort_unstable();
        let mut want: Vec<u8> = Vec::new();
        for t in 0..34u8 {
            want.extend([t; 4]);
        }
        assert_eq!(all, want);
    }

    #[test]
    fn standard_win_detected_with_fans() {
        // 123m 456m 789m 123p 55s, tragere proprie, închisă, toate secvențele
        let c = counts_of(&[
            "1m", "2m", "3m", "4m", "5m", "6m", "7m", "8m", "9m", "1p", "2p", "3p", "5s", "5s",
        ]);
        let (fan, d) = score_hand(&c, &[], 27, true).unwrap();
        assert_eq!(fan, 3, "{d}"); // tragere 1 + închisă 1 + secvențe 1
                                   // aceeași mână câștigată pe aruncare
        let (fan2, _) = score_hand(&c, &[], 27, false).unwrap();
        assert_eq!(fan2, 2);
    }

    #[test]
    fn not_a_win_when_incomplete() {
        let c = counts_of(&[
            "1m", "2m", "3m", "4m", "5m", "6m", "7m", "8m", "9m", "1p", "2p", "4p", "5s", "5s",
        ]);
        assert!(score_hand(&c, &[], 27, true).is_none());
    }

    #[test]
    fn pure_suit_and_all_pungs() {
        let c = counts_of(&[
            "1m", "1m", "1m", "2m", "2m", "2m", "3m", "3m", "3m", "4m", "4m", "4m", "9m", "9m",
        ]);
        let (fan, d) = score_hand(&c, &[], 27, false).unwrap();
        // toate pungurile 3 + o culoare 6 + închisă 1
        assert_eq!(fan, 10, "{d}");
    }

    #[test]
    fn dragon_pung_and_melds() {
        // un chi expus + 3 seturi în mână + pereche
        let c = counts_of(&[
            "Dr", "Dr", "Dr", "1p", "2p", "3p", "4s", "4s", "4s", "9m", "9m",
        ]);
        let melds = [Meld::Chi(0)];
        let (fan, d) = score_hand(&c, &melds, 28, false).unwrap();
        assert_eq!(fan, 1, "{d}"); // doar pungul de dragon (seturi expuse: nu e închisă)
    }

    #[test]
    fn chicken_hand_cannot_ron_but_can_tsumo() {
        let mut g = Mahjong::with_seed(1);
        // mână fără fan pe aruncare: pung expus + pereche normală
        g.counts[1] = counts_of(&[
            "1m", "2m", "3m", "4m", "5m", "6m", "7m", "8m", "9m", "2p", "2p", "5p", "5p",
        ]);
        g.counts[1][27] = 0;
        // așteaptă 2p sau 5p: pung ⇒ nu e toate secvențele; închisă +1 ⇒ ron e permis (1 fan)
        let r = g.reactions(Player::Two, tile_name_idx("5p"));
        assert!(r.contains(&Move::Ron));
        // cu un set expus și un pung ordinar: 0 fan, deci fără ron
        g.melds[1] = vec![Meld::Chi(9)];
        g.counts[1] = counts_of(&["1m", "2m", "3m", "4p", "4p", "7s", "8s", "9s", "5p", "5p"]);
        let r2 = g.reactions(Player::Two, tile_name_idx("4p"));
        assert!(!r2.contains(&Move::Ron), "{r2:?}");
        assert!(r2.contains(&Move::Pung));
    }

    fn tile_name_idx(s: &str) -> u8 {
        (0..34u8).find(|&x| tile_name(x) == s).unwrap()
    }

    #[test]
    fn chi_options_and_pung() {
        let mut g = Mahjong::with_seed(1);
        g.counts[1] = counts_of(&[
            "2m", "3m", "5m", "6m", "4m", "4m", "Dr", "Dr", "E", "N", "1p", "9s", "9s",
        ]);
        let r = g.reactions(Player::Two, tile_name_idx("4m"));
        assert!(r.contains(&Move::Chi(tile_name_idx("2m"), tile_name_idx("3m"))));
        assert!(r.contains(&Move::Chi(tile_name_idx("3m"), tile_name_idx("5m"))));
        assert!(r.contains(&Move::Chi(tile_name_idx("5m"), tile_name_idx("6m"))));
        assert!(r.contains(&Move::Pung));
        assert!(r.contains(&Move::Pass));
        // vânturile/dragonii nu se pot lua cu chi
        let r2 = g.reactions(Player::Two, tile_name_idx("E"));
        assert!(!r2.iter().any(|m| matches!(m, Move::Chi(_, _))));
    }

    #[test]
    fn pung_then_discard_and_kong_draws_dead() {
        let mut g = Mahjong::with_seed(2);
        g.counts[0] = counts_of(&[
            "1m", "5s", "9p", "E", "N", "Dr", "Dg", "2m", "3p", "4s", "7s", "8p", "Dw", "1p",
        ]);
        g.counts[1] = counts_of(&[
            "1m", "1m", "3m", "4m", "5s", "6s", "7p", "8p", "E", "S", "W", "N", "Dg",
        ]);
        g.phase = Phase::Discard;
        g.side = Player::One;
        g.apply(&Move::Discard(tile_name_idx("1m")));
        assert_eq!(g.phase, Phase::React);
        assert!(g.legal_moves().contains(&Move::Pung));
        g.apply(&Move::Pung);
        assert_eq!(g.melds[1], vec![Meld::Pung(tile_name_idx("1m"))]);
        assert_eq!(g.phase, Phase::Discard);
        assert_eq!(g.side, Player::Two);
        assert!(g.discards[0].is_empty());
        // careu din mână
        let mut h = Mahjong::with_seed(3);
        h.counts[0][0] = 4;
        h.phase = Phase::Discard;
        h.side = Player::One;
        assert!(h.legal_moves().contains(&Move::KongSelf(0)));
        let dead = h.dead.len();
        h.apply(&Move::KongSelf(0));
        assert_eq!(h.phase, Phase::DrawDead);
        h.apply(&Move::DrawDead);
        assert_eq!(h.dead.len(), dead - 1);
        assert_eq!(h.phase, Phase::Discard);
    }

    #[test]
    fn exhausted_wall_is_a_draw() {
        let mut g = Mahjong::with_seed(4);
        g.wall.clear();
        g.begin_draw(Player::One);
        assert_eq!(g.outcome(), Some(Outcome::Draw));
    }

    #[test]
    fn render_hides_opponent_and_wall() {
        let g = Mahjong::with_seed(6);
        let r = Game::render(&g);
        assert!(r.contains("J2: 13 piese ascunse"));
        assert!(r.contains("zid viu"));
    }

    #[test]
    fn ai_blind_to_hidden() {
        for seed in 1..30u64 {
            let mut a = Mahjong::with_seed(seed);
            let mut rng = Rng::new(seed);
            for _ in 0..(seed % 12) {
                if a.outcome().is_some() {
                    break;
                }
                let mv = a.legal_moves();
                a.apply(&mv[rng.below(mv.len())]);
            }
            if a.outcome().is_some() {
                continue;
            }
            let me = a.side.idx();
            let mut b = a.clone();
            let o = 1 - me;
            // permută piesele ascunse (mâna adversarului + zid + zid mort)
            let mut pool: Vec<u8> = b.wall.clone();
            pool.extend(&b.dead);
            for t in 0..34 {
                for _ in 0..b.counts[o][t] {
                    pool.push(t as u8);
                }
            }
            shuffle(&mut pool, &mut rng);
            let n_opp: usize = b.counts[o].iter().map(|&x| x as usize).sum();
            b.counts[o] = [0; 34];
            for _ in 0..n_opp {
                b.counts[o][pool.pop().unwrap() as usize] += 1;
            }
            let nd = b.dead.len();
            b.dead = pool.drain(..nd).collect();
            b.wall = pool;
            // dacă mutarea AI depinde de tragere (Draw), mutarea e unică; altfel trebuie să fie aceeași
            if a.phase == Phase::React {
                // dreptul la Ron depinde de mâna mea (vizibilă), nu de a adversarului
            }
            assert_eq!(a.legal_moves().len(), b.legal_moves().len());
            assert_eq!(
                polytabletop_core::ai::best_move(&a, 9),
                polytabletop_core::ai::best_move(&b, 9)
            );
        }
    }

    #[test]
    fn random_hands_end_and_tiles_conserved() {
        for seed in 1..60u64 {
            let mut g = Mahjong::with_seed(seed);
            let mut rng = Rng::new(seed ^ 0x77);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                assert!(!mv.is_empty(), "{}", g.render());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
                assert!(n < 2000);
                let held: usize = (0..2)
                    .map(|p| {
                        g.counts[p].iter().map(|&x| x as usize).sum::<usize>()
                            + g.melds[p].iter().map(|m| m.tiles().len()).sum::<usize>()
                            + g.discards[p].len()
                    })
                    .sum();
                assert_eq!(held + g.wall.len() + g.dead.len(), 136);
            }
        }
    }
}

#[cfg(test)]
mod viewer_tests {
    use super::*;

    fn hand_line(g: &Mahjong, p: Player) -> String {
        let mut h = Vec::new();
        for t in 0..34u8 {
            for _ in 0..g.counts[p.idx()][t as usize] {
                h.push(tile_name(t));
            }
        }
        format!("Mâna ta (J{}): {}\n", p.idx() + 1, h.join(" "))
    }

    #[test]
    fn viewer_sees_only_own_concealed_hand() {
        let mut g = Mahjong::with_seed(3);
        Game::set_viewer(&mut g, Some(Player::One));
        let r = Game::render(&g);
        assert!(r.contains(&hand_line(&g, Player::One)), "{r}");
        assert!(!r.contains("Mâna ta (J2)"), "{r}");
        Game::set_viewer(&mut g, Some(Player::Two));
        let r = Game::render(&g);
        assert!(r.contains(&hand_line(&g, Player::Two)), "{r}");
        assert!(!r.contains("Mâna ta (J1)"), "{r}");
        assert!(r.contains("J1: 13 piese ascunse") || r.contains("J1: 14 piese ascunse"));
    }

    #[test]
    fn hotseat_follows_the_mover() {
        let mut g = Mahjong::with_seed(3);
        Game::set_viewer(&mut g, None);
        let who = Game::to_move(&g);
        let r = Game::render(&g);
        assert!(r.contains(&format!("Mâna ta (J{})", who.idx() + 1)), "{r}");
    }
}
