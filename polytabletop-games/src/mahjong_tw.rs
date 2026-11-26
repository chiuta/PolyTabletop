//! Mahjong taiwanez — variantă simplificată, cinstită, pentru 2 jucători (J1 dealer = vântul Est, J2 = Sud).
//!
//! * 144 de piese: cele 136 obișnuite (3 culori numerice × 9 × 4, 4 vânturi, 3 dragoni) plus **8 flori**
//!   (4 flori + 4 anotimpuri). Fiecare jucător primește **16 piese** (nu 13): mâna câștigătoare are
//!   **5 seturi + o pereche** (secvențe, pungs, careuri). Zidul mort are 16 piese.
//! * Florile nu stau în mână: când apar (la împărțire sau la tragere) se pun deoparte și iei o piesă de
//!   înlocuire din zidul mort. Florile tale „de loc” (J1: floarea 1 și anotimpul 1; J2: floarea 2 și
//!   anotimpul 2) valorează 1 tai; toate cele 4 flori sau cele 4 anotimpuri = +2 tai.
//! * Tura (ca la varianta Hong Kong din joc): `trage`, apoi `d5m` (aruncă) sau `mahjong` (câștig din
//!   tragere), `kong5m` (careu); adversarul poate lua aruncarea cu `mahjong`, `pung`, `kong`,
//!   `chi2m3m` (în joc pentru doi se permite de la orice aruncare) sau `pas`.
//! * Punctaj în **tai** (o mână completă valorează cel puțin 1 tai): bază 1, dealer +1 (J1), tragere
//!   proprie +1, mână închisă (fără seturi expuse) +1, toate secvențele +2, toate pungurile +4, jumătate
//!   de culoare (o culoare + onoruri) +4, o singură culoare +8, doar onoruri +8, pung de dragon +1 (fiecare),
//!   pung de vântul tău +1, pung de vântul rundei (Est) +1, plus florile. Fără alte mâini speciale.
//! * Câștigă primul care declară o mână validă; zidul terminat = remiză. AI-ul vede doar mâna lui, seturile
//!   expuse, florile și aruncările publice.

use crate::cards52::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const DEAD: usize = 16;
const WINDS: [&str; 4] = ["E", "S", "W", "N"];
const DRAGONS: [&str; 3] = ["Dr", "Dg", "Dw"];

pub fn tile_name(t: u8) -> String {
    if t >= 38 {
        format!("An{}", t - 37)
    } else if t >= 34 {
        format!("Fl{}", t - 33)
    } else if t < 27 {
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

/// Tai pentru florile unui jucător (`seat` = 0 pentru J1, 1 pentru J2).
pub fn flower_tai(flowers: &[u8], seat: usize) -> (i32, String) {
    let mut tai = 0;
    let mut why = Vec::new();
    let mine = [34 + seat as u8, 38 + seat as u8];
    for f in mine {
        if flowers.contains(&f) {
            tai += 1;
            why.push(format!("floare de loc {} 1", tile_name(f)));
        }
    }
    if (34..38).all(|f| flowers.contains(&f)) {
        tai += 2;
        why.push("cele 4 flori 2".into());
    }
    if (38..42).all(|f| flowers.contains(&f)) {
        tai += 2;
        why.push("cele 4 anotimpuri 2".into());
    }
    (tai, why.join(", "))
}

/// Taii celei mai bune descompuneri sau `None` dacă mâna nu e completă (5 seturi + pereche).
pub fn score_hand(
    counts: &Counts,
    melds: &[Meld],
    seat_wind: u8,
    self_draw: bool,
    flowers: &[u8],
) -> Option<(i32, String)> {
    let need = 5usize.checked_sub(melds.len())?;
    let total: u32 = counts.iter().map(|&x| x as u32).sum();
    if total != 3 * need as u32 + 2 {
        return None;
    }
    let mut c = *counts;
    let ds = decomps(&mut c, need);
    let mut best: Option<(i32, String)> = None;
    let seat = (seat_wind - 27) as usize;
    for (pair, sets) in ds {
        let mut all: Vec<(bool, u8)> = sets.clone();
        for m in melds {
            match *m {
                Meld::Chi(a) => all.push((false, a)),
                Meld::Pung(t) | Meld::Kong(t, _) => all.push((true, t)),
            }
        }
        let mut tai = 1;
        let mut why: Vec<String> = vec!["mână completă 1".into()];
        if seat == 0 {
            tai += 1;
            why.push("dealer 1".into());
        }
        if self_draw {
            tai += 1;
            why.push("tragere proprie 1".into());
        }
        let exposed = melds.iter().any(|m| !matches!(m, Meld::Kong(_, true)));
        if !exposed {
            tai += 1;
            why.push("mână închisă 1".into());
        }
        let pung_count = all.iter().filter(|s| s.0).count();
        let honor_pair = pair >= 31 || pair == seat_wind || pair == 27;
        if pung_count == 0 && !honor_pair {
            tai += 2;
            why.push("toate secvențele 2".into());
        }
        if pung_count == 5 {
            tai += 4;
            why.push("toate pungurile 4".into());
        }
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
            tai += 8;
            why.push("doar onoruri 8".into());
        } else if n_suits == 1 && !honors {
            tai += 8;
            why.push("o singură culoare 8".into());
        } else if n_suits == 1 && honors {
            tai += 4;
            why.push("jumătate de culoare 4".into());
        }
        for &(p, t) in &all {
            if !p {
                continue;
            }
            if t >= 31 {
                tai += 1;
                why.push(format!("pung de dragon {} 1", tile_name(t)));
            }
            if t == seat_wind {
                tai += 1;
                why.push("pung de vânt propriu 1".into());
            }
            if t == 27 {
                tai += 1;
                why.push("pung de vântul rundei 1".into());
            }
        }
        let (ft, fw) = flower_tai(flowers, seat);
        if ft > 0 {
            tai += ft;
            why.push(fw);
        }
        let desc = why.join(", ");
        if best.as_ref().is_none_or(|b| tai > b.0) {
            best = Some((tai, desc));
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
pub struct MahjongTw {
    counts: [Counts; 2],
    melds: [Vec<Meld>; 2],
    discards: [Vec<u8>; 2],
    wall: Vec<u8>,
    dead: Vec<u8>,
    flowers: [Vec<u8>; 2],
    side: Player,
    phase: Phase,
    last: Option<(u8, Player)>,
    drawn: Option<u8>,
    winner: Option<(Player, i32)>,
    msg: String,
    /// `Some(p)` = render shows `p`'s concealed hand; `None` = hot-seat (the mover's).
    viewer: Option<Player>,
}

impl Default for MahjongTw {
    fn default() -> Self {
        Self::new()
    }
}

impl MahjongTw {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut w: Vec<u8> = Vec::with_capacity(144);
        for t in 0..34u8 {
            w.extend([t; 4]);
        }
        w.extend(34..42u8);
        shuffle(&mut w, &mut rng);
        let mut dead: Vec<u8> = w.drain(..DEAD).collect();
        let mut counts = [[0u8; 34]; 2];
        let mut flowers: [Vec<u8>; 2] = [Vec::new(), Vec::new()];
        for p in 0..2 {
            let mut n = 0;
            while n < 16 {
                let mut t = w.pop().unwrap();
                while t >= 34 {
                    flowers[p].push(t);
                    t = dead.pop().or_else(|| w.pop()).unwrap();
                }
                counts[p][t as usize] += 1;
                n += 1;
            }
        }
        MahjongTw {
            counts,
            melds: [Vec::new(), Vec::new()],
            discards: [Vec::new(), Vec::new()],
            wall: w,
            dead,
            flowers,
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

    pub fn flowers(&self, p: Player) -> &[u8] {
        &self.flowers[p.idx()]
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
        if score_hand(
            &cc,
            &self.melds[r.idx()],
            Self::seat(r),
            false,
            &self.flowers[r.idx()],
        )
        .is_some()
        {
            v.push(Move::Ron);
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
        self.msg = format!("J{} câștigă ({how}) cu {fan} tai: {desc}", p.idx() + 1);
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

impl Game for MahjongTw {
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
                    &self.flowers[me],
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
            Move::Draw | Move::DrawDead => {
                let mut t = if matches!(m, Move::Draw) {
                    self.wall.pop().expect("zid")
                } else {
                    self.dead.pop().expect("zid mort")
                };
                // florile se pun deoparte și se înlocuiesc din zidul mort (apoi din zidul viu)
                while t >= 34 {
                    self.flowers[i].push(t);
                    match self.dead.pop().or_else(|| self.wall.pop()) {
                        Some(n) => t = n,
                        None => {
                            self.drawn = None;
                            self.phase = Phase::Over;
                            self.msg = "Zidul s-a terminat: mână remiză.".into();
                            return;
                        }
                    }
                }
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
                let (fan, d) = score_hand(
                    &self.counts[i],
                    &self.melds[i],
                    Self::seat(me),
                    true,
                    &self.flowers[i],
                )
                .unwrap();
                self.win(me, fan, &d, "tragere proprie");
            }
            Move::Ron => {
                let (t, from) = self.last.unwrap();
                self.counts[i][t as usize] += 1;
                self.discards[from.idx()].pop();
                let (fan, d) = score_hand(
                    &self.counts[i],
                    &self.melds[i],
                    Self::seat(me),
                    false,
                    &self.flowers[i],
                )
                .unwrap();
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
            "Mahjong taiwanez — zid viu: {} piese (zid mort: {}) | vânt: J1 Est (dealer), J2 Sud\n",
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
            let fl: Vec<String> = self.flowers[p.idx()]
                .iter()
                .map(|&t| tile_name(t))
                .collect();
            s.push_str(&format!(
                "J{}: flori [{}]; seturi expuse [{}]; aruncate: {}\n",
                p.idx() + 1,
                fl.join(" "),
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
    use crate::q5_util::testkit::check_game;

    fn counts_of(tiles: &[u8]) -> Counts {
        let mut c = [0u8; 34];
        for &t in tiles {
            c[t as usize] += 1;
        }
        c
    }

    #[test]
    fn deal_has_16_tiles_and_flowers_are_set_aside() {
        for seed in 1..30 {
            let g = MahjongTw::with_seed(seed);
            for p in 0..2 {
                let n: u32 = g.counts[p].iter().map(|&x| x as u32).sum();
                assert_eq!(n, 16);
            }
            let total = g.wall.len() + g.dead.len() + g.flowers[0].len() + g.flowers[1].len() + 32;
            assert_eq!(total, 144);
            assert!(g.counts.iter().all(|c| c.len() == 34));
        }
    }

    #[test]
    fn five_sets_and_a_pair_win_and_flowers_count() {
        // 123m 456m 789m 123p 456p + pereche 5s  (16 + 1 = 17 piese)
        let mut t: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 18, 19, 20, 21, 22, 23];
        t.extend([13, 13]);
        let c = counts_of(&t);
        // J2 (Sud), fără dealer: mână închisă +1, toate secvențele +2, bază 1, tragere proprie +1 = 5
        let (tai, why) = score_hand(&c, &[], 28, true, &[]).unwrap();
        assert_eq!(tai, 5, "{why}");
        // floarea de loc a lui J2 (35) + anotimp 39: +2
        let (tai2, _) = score_hand(&c, &[], 28, true, &[35, 39]).unwrap();
        assert_eq!(tai2, 7);
        // cele 4 flori +2 (și floarea de loc +1)
        let (tai3, _) = score_hand(&c, &[], 28, true, &[34, 35, 36, 37]).unwrap();
        assert_eq!(tai3, 5 + 1 + 2);
        // 13 piese nu mai sunt o mână completă în varianta taiwaneză
        let mut small = vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 18, 19, 20];
        small.extend([13, 13]);
        assert!(score_hand(&counts_of(&small), &[], 28, true, &[]).is_none());
    }

    #[test]
    fn viewer_hides_the_other_hand_and_ai_is_blind() {
        let g = MahjongTw::with_seed(3);
        let mut one = g.clone();
        one.set_viewer(Some(Player::One));
        let mut two = g.clone();
        two.set_viewer(Some(Player::Two));
        assert_ne!(one.render(), two.render());
        let mut a = g.clone();
        a.apply(&Move::Draw);
        let mut b = a.clone();
        let opp = 1usize;
        b.counts[opp] = [0; 34];
        b.counts[opp][0] = 16;
        b.wall.reverse();
        for seed in 1..6 {
            assert_eq!(
                polytabletop_core::GameObj::ai_move(&a, seed),
                polytabletop_core::GameObj::ai_move(&b, seed)
            );
        }
    }

    #[test]
    fn random_and_ai_games() {
        check_game(MahjongTw::with_seed, 6, 1500);
    }
}
