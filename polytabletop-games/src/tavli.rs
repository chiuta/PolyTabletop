//! Tavli (Grecia): Portes, Plakoto și Fevga — plus Gul bara (Golful Persic) pe același motor.
//!
//! Fiecare jucător are 15 piese; mută spre „acasă” (punctele 1–6 proprii; 24 = punctul de start),
//! iar scoaterea (bear off) începe când toate piesele sunt acasă. Zarurile se aruncă automat.
//! Un joc `tavli` începe cu alegerea variantei de către J1 (`portes`, `plakoto` sau `fevga`).
//!
//! * **Portes**: backgammon fără cub: start 2×24, 5×13, 3×8, 5×6; un punct cu o singură piesă
//!   adversă se lovește (piesa merge pe bară și reintră pe punctul `25 − zar`); puncte cu 2+ piese
//!   adverse sunt blocate.
//! * **Plakoto**: toate cele 15 piese pe punctul 24 al fiecăruia; nu se lovește, ci se *imobilizează*:
//!   aterizezi pe o piesă adversă singură și ea rămâne blocată sub a ta până pleacă toate ale tale
//!   de acolo. Puncte cu 2+ piese adverse sunt blocate.
//! * **Fevga**: ambii jucători merg în același sens (antiorar), pornind din colțuri opuse; nu se
//!   lovește, punctele ocupate de adversar sunt blocate. Regulă de „zid”: nu poți forma 6 puncte
//!   consecutive dacă adversarul nu are nicio piesă dincolo de ele. (Simplificare: lipsește regula
//!   specială a primei piese din Fevga.)
//! * **Gul bara**: ca Fevga, fără zid, cu dubluri speciale: primele 3 aruncări ale partidei dau 4 mutări
//!   cu zarul dublu; apoi un dublu `d-d` dă câte 4 mutări cu d, d+1, …, 6. Mutările care nu se pot juca se pierd
//!   (simplificare: nu le joacă adversarul). Câștigă (1 punct) cine scoate primul cele 15 piese.
//!
//! Mutare: `24/18` (de pe punctul 24 cu zarul 6), `bar/20`, `6/off`. Regula „joacă toate zarurile
//! dacă se poate” se aplică în Portes, Plakoto și Fevga.
//!
//! Surse: regulile publice (Wikipedia „Tavli”, „Plakoto”, „Fevga”, „Gul bara”).

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Variant {
    Portes,
    Plakoto,
    Fevga,
    GulBara,
}

impl Variant {
    fn opposed(self) -> bool {
        matches!(self, Variant::Portes | Variant::Plakoto)
    }
    pub fn name(self) -> &'static str {
        match self {
            Variant::Portes => "Portes",
            Variant::Plakoto => "Plakoto",
            Variant::Fevga => "Fevga",
            Variant::GulBara => "Gul bara",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mv {
    Choose(Variant),
    Step { from: u8, die: u8 },
}

#[derive(Clone)]
pub struct Tavli {
    var: Option<Variant>,
    pts: [[u8; 24]; 2],
    /// piesa de pe punct a acestui jucător e imobilizată (Plakoto)
    pin: [Option<u8>; 24],
    bar: [u8; 2],
    off: [u8; 2],
    side: Player,
    dice: Vec<u8>,
    rolls: u32,
    played: u8,
    rng: Rng,
    log: String,
}

impl Default for Tavli {
    fn default() -> Self {
        Self::new()
    }
}

impl Tavli {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Tavli {
            var: None,
            pts: [[0; 24]; 2],
            pin: [None; 24],
            bar: [0; 2],
            off: [0; 2],
            side: Player::One,
            dice: Vec::new(),
            rolls: 0,
            played: 0,
            rng: Rng::new(seed),
            log: String::new(),
        }
    }

    pub fn with_variant(variant: Variant, seed: u64) -> Self {
        let mut g = Self::with_seed(seed);
        g.choose(variant);
        g
    }

    pub fn variant(&self) -> Option<Variant> {
        self.var
    }

    pub fn borne_off(&self) -> (u8, u8) {
        (self.off[0], self.off[1])
    }

    fn ix(&self, p: usize, n: u8) -> usize {
        let v = self.var.unwrap();
        if p == 0 || v.opposed() {
            if p == 0 {
                n as usize - 1
            } else {
                24 - n as usize
            }
        } else {
            (n as usize + 11) % 24
        }
    }

    /// punctul (în coordonatele lui `p`) al indexului absolut
    fn n_of(&self, p: usize, idx: usize) -> u8 {
        (1..=24u8).find(|&n| self.ix(p, n) == idx).unwrap()
    }

    fn choose(&mut self, v: Variant) {
        self.var = Some(v);
        self.pts = [[0; 24]; 2];
        for p in 0..2 {
            match v {
                Variant::Portes => {
                    for (n, c) in [(24u8, 2u8), (13, 5), (8, 3), (6, 5)] {
                        let i = self.ix(p, n);
                        self.pts[p][i] = c;
                    }
                }
                _ => {
                    let i = self.ix(p, 24);
                    self.pts[p][i] = 15;
                }
            }
        }
        let (a, b) = loop {
            let a = self.rng.d6();
            let b = self.rng.d6();
            if a != b {
                break (a, b);
            }
        };
        self.rolls = 1;
        self.side = if a > b { Player::One } else { Player::Two };
        self.dice = vec![a, b];
        self.log = format!("{}: zar de deschidere {a} și {b}.", v.name());
        self.settle();
    }

    fn roll_new(&mut self) {
        let a = self.rng.d6();
        let b = self.rng.d6();
        self.rolls += 1;
        self.played = 0;
        self.dice = if a == b {
            if self.var == Some(Variant::GulBara) && self.rolls > 3 {
                (a..=6).flat_map(|d| [d; 4]).collect()
            } else {
                vec![a; 4]
            }
        } else {
            vec![a, b]
        };
    }

    fn all_home(&self, p: usize) -> bool {
        self.bar[p] == 0 && (7..=24u8).all(|n| self.pts[p][self.ix(p, n)] == 0)
    }

    fn free_at(&self, p: usize, idx: usize) -> u8 {
        let pinned = u8::from(self.pin[idx] == Some(p as u8));
        self.pts[p][idx].saturating_sub(pinned)
    }

    fn can_land(&self, p: usize, idx: usize) -> bool {
        let v = self.var.unwrap();
        let (mine, opp) = (self.pts[p][idx], self.pts[1 - p][idx]);
        match v {
            Variant::Portes | Variant::Plakoto => mine > 0 || opp <= 1,
            _ => opp == 0,
        }
    }

    fn raw_moves(&self) -> Vec<Mv> {
        let p = self.side.idx();
        let mut ds = self.dice.clone();
        ds.sort_unstable();
        ds.dedup();
        let mut out = Vec::new();
        let home = self.all_home(p);
        let highest = (1..=24u8).rev().find(|&n| self.pts[p][self.ix(p, n)] > 0);
        for d in ds {
            if self.bar[p] > 0 {
                if self.can_land(p, self.ix(p, 25 - d)) {
                    out.push(Mv::Step { from: 25, die: d });
                }
                continue;
            }
            for n in 1..=24u8 {
                if self.free_at(p, self.ix(p, n)) == 0 {
                    continue;
                }
                if n > d {
                    let t = self.ix(p, n - d);
                    if self.can_land(p, t) && self.prime_ok(p, n, n - d) {
                        out.push(Mv::Step { from: n, die: d });
                    }
                } else if home && (n == d || highest == Some(n)) {
                    out.push(Mv::Step { from: n, die: d });
                }
            }
        }
        out
    }

    /// Regula zidului din Fevga: după mutare nu apar 6 puncte consecutive proprii fără piese adverse dincolo.
    fn prime_ok(&self, p: usize, from: u8, to: u8) -> bool {
        if self.var != Some(Variant::Fevga) {
            return true;
        }
        let mut c = self.clone();
        c.pts[p][c.ix(p, from)] -= 1;
        let t = c.ix(p, to);
        c.pts[p][t] += 1;
        // caută 6 puncte consecutive (în ordinea n) ale lui p care includ `to`
        for start in 1..=19u8 {
            if !(start..start + 6).contains(&to) {
                continue;
            }
            if (start..start + 6).all(|n| c.pts[p][c.ix(p, n)] > 0) {
                // piesele adverse „dincolo” = cu n_opp mai mic decât minimul lor pe zid
                let min_n = (start..start + 6)
                    .map(|n| c.n_of(1 - p, c.ix(p, n)))
                    .min()
                    .unwrap();
                let ahead = (0..24).any(|i| c.pts[1 - p][i] > 0 && c.n_of(1 - p, i) < min_n);
                if !ahead {
                    return false;
                }
            }
        }
        true
    }

    fn step(&mut self, from: u8, die: u8) {
        let p = self.side.idx();
        let o = 1 - p;
        let v = self.var.unwrap();
        let to: i16 = if from == 25 {
            self.bar[p] -= 1;
            25 - die as i16
        } else {
            let f = self.ix(p, from);
            self.pts[p][f] -= 1;
            if self.pts[p][f] == 0 && self.pin[f] == Some(o as u8) {
                self.pin[f] = None;
            }
            from as i16 - die as i16
        };
        if to >= 1 {
            let t = self.ix(p, to as u8);
            if self.pts[p][t] == 0 && self.pts[o][t] == 1 {
                match v {
                    Variant::Portes => {
                        self.pts[o][t] = 0;
                        self.bar[o] += 1;
                    }
                    Variant::Plakoto => self.pin[t] = Some(o as u8),
                    _ => {}
                }
            }
            self.pts[p][t] += 1;
        } else {
            self.off[p] += 1;
        }
        if let Some(k) = self.dice.iter().position(|&d| d == die) {
            self.dice.remove(k);
        }
        self.played += 1;
    }

    fn max_use(&self, depth: u8) -> u8 {
        if depth == 0 {
            return 0;
        }
        self.raw_moves()
            .into_iter()
            .map(|m| {
                let mut c = self.clone();
                if let Mv::Step { from, die } = m {
                    c.step(from, die);
                }
                1 + c.max_use(depth - 1)
            })
            .max()
            .unwrap_or(0)
    }

    fn winner(&self) -> Option<Player> {
        if self.off[0] >= 15 {
            Some(Player::One)
        } else if self.off[1] >= 15 {
            Some(Player::Two)
        } else {
            None
        }
    }

    fn settle(&mut self) {
        while self.var.is_some() && self.winner().is_none() {
            if !self.dice.is_empty() && !self.raw_moves().is_empty() {
                return;
            }
            if self.played == 0 {
                self.log
                    .push_str(&format!(" J{} nu are mutări legale.", self.side.idx() + 1));
            }
            self.side = self.side.other();
            self.roll_new();
            self.log.push_str(&format!(
                " J{} aruncă {:?}.",
                self.side.idx() + 1,
                &self.dice[..self.dice.len().min(4)]
            ));
        }
    }

    fn text(&self, from: u8, die: u8) -> String {
        if from == 25 {
            format!("bar/{}", 25 - die)
        } else if from > die {
            format!("{}/{}", from, from - die)
        } else {
            format!("{from}/off")
        }
    }

    fn pips(&self, p: usize) -> i32 {
        let mut t = self.bar[p] as i32 * 25;
        for n in 1..=24u8 {
            t += n as i32 * self.pts[p][self.ix(p, n)] as i32;
        }
        t
    }
}

impl Game for Tavli {
    type Move = Mv;
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<Mv> {
        let Some(v) = self.var else {
            return vec![
                Mv::Choose(Variant::Portes),
                Mv::Choose(Variant::Plakoto),
                Mv::Choose(Variant::Fevga),
            ];
        };
        if self.winner().is_some() {
            return Vec::new();
        }
        let raw = self.raw_moves();
        if raw.is_empty() {
            return raw;
        }
        let mut cand = raw;
        if v != Variant::GulBara {
            let scored: Vec<(Mv, u8)> = cand
                .into_iter()
                .map(|m| {
                    let mut c = self.clone();
                    if let Mv::Step { from, die } = m {
                        c.step(from, die);
                    }
                    (m, 1 + c.max_use(3))
                })
                .collect();
            let best = scored.iter().map(|x| x.1).max().unwrap_or(0);
            cand = scored
                .into_iter()
                .filter(|x| x.1 == best)
                .map(|x| x.0)
                .collect();
            if best == 1 && self.dice.len() >= 2 {
                let maxd = self.dice.iter().copied().max().unwrap_or(0);
                if cand
                    .iter()
                    .any(|m| matches!(m, Mv::Step { die, .. } if *die == maxd))
                {
                    cand.retain(|m| matches!(m, Mv::Step { die, .. } if *die == maxd));
                }
            }
        }
        let mut seen: Vec<String> = Vec::new();
        cand.retain(|m| {
            let Mv::Step { from, die } = *m else {
                return true;
            };
            let t = self.text(from, die);
            if seen.contains(&t) {
                false
            } else {
                seen.push(t);
                true
            }
        });
        cand
    }
    fn apply(&mut self, m: &Mv) {
        match *m {
            Mv::Choose(v) => self.choose(v),
            Mv::Step { from, die } => {
                self.step(from, die);
                self.settle();
            }
        }
    }
    fn outcome(&self) -> Option<Outcome> {
        self.winner().map(Outcome::Win)
    }
    fn evaluate(&self, pl: Player) -> i32 {
        let Some(v) = self.var else { return 0 };
        let (p, o) = (pl.idx(), 1 - pl.idx());
        let mut s =
            4 * (self.pips(o) - self.pips(p)) + 60 * (self.off[p] as i32 - self.off[o] as i32);
        match v {
            Variant::Portes => {
                for n in 1..=24u8 {
                    match self.pts[p][self.ix(p, n)] {
                        1 => s -= 10,
                        c if c >= 2 => s += 4,
                        _ => {}
                    }
                }
                s += 30 * (self.bar[o] as i32 - self.bar[p] as i32);
            }
            Variant::Plakoto => {
                for i in 0..24 {
                    match self.pin[i] {
                        Some(q) if q as usize == o => s += 40,
                        Some(_) => s -= 40,
                        None => {}
                    }
                }
            }
            _ => {}
        }
        s
    }
    fn render(&self) -> String {
        let Some(v) = self.var else {
            return "Tavli: J1 alege varianta — portes, plakoto sau fevga.".to_string();
        };
        let cell = |idx: usize| -> String {
            let (a, b) = (self.pts[0][idx], self.pts[1][idx]);
            if a > 0 {
                format!("X{a}{}", if self.pin[idx] == Some(1) { "!" } else { "" })
            } else if b > 0 {
                format!("O{b}{}", if self.pin[idx] == Some(0) { "!" } else { "" })
            } else {
                ".".into()
            }
        };
        let mut s = String::new();
        s.push_str("J1 punct: ");
        for idx in (12..24).rev() {
            s.push_str(&format!("{:>4}", self.n_of(0, idx)));
        }
        s.push_str("\n          ");
        for idx in (12..24).rev() {
            s.push_str(&format!("{:>4}", cell(idx)));
        }
        s.push_str("\n          ");
        for idx in 0..12 {
            s.push_str(&format!("{:>4}", cell(idx)));
        }
        s.push_str("\nJ1 punct: ");
        for idx in 0..12 {
            s.push_str(&format!("{:>4}", self.n_of(0, idx)));
        }
        s.push_str(&format!(
            "\n{}: X = J1, O = J2 (! = piesă imobilizată). Bară: X {} O {}. Scoase: X {} O {}. Pips: {} / {}.\n",
            v.name(),
            self.bar[0],
            self.bar[1],
            self.off[0],
            self.off[1],
            self.pips(0),
            self.pips(1)
        ));
        s.push_str(&format!("Zaruri de jucat: {:?}. {}\nLa mutare: Jucătorul {} (punctele se numără din perspectiva lui).", &self.dice[..self.dice.len().min(8)], self.log, self.side.idx() + 1));
        s
    }
    fn format_move(&self, m: &Mv) -> String {
        match *m {
            Mv::Choose(v) => v.name().to_lowercase().replace(' ', ""),
            Mv::Step { from, die } => self.text(from, die),
        }
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q4_util::*;
    use polytabletop_core::{GameObj, Player};

    #[test]
    fn variant_choice_then_opening_roll() {
        let mut g = Tavli::with_seed(1);
        let m = GameObj::legal_moves(&g);
        assert_eq!(m, vec!["portes", "plakoto", "fevga"]);
        GameObj::play(&mut g, "plakoto").unwrap();
        assert_eq!(g.variant(), Some(Variant::Plakoto));
        assert_eq!(g.pts[0].iter().sum::<u8>(), 15);
        assert_eq!(g.pts[1].iter().sum::<u8>(), 15);
    }

    #[test]
    fn plakoto_pins_a_single_checker() {
        let mut g = Tavli::with_variant(Variant::Plakoto, 2);
        g.pts = [[0; 24]; 2];
        g.pin = [None; 24];
        g.side = Player::One;
        g.dice = vec![3];
        let (a, b) = (g.ix(0, 10), g.ix(0, 7));
        g.pts[0][a] = 1;
        g.pts[1][b] = 1;
        g.pts[0][g.ix(0, 24)] = 14;
        g.pts[1][g.ix(1, 24)] = 14;
        GameObj::play(&mut g, "10/7").unwrap();
        assert_eq!(g.pin[b], Some(1));
        assert_eq!(g.pts[1][b], 1);
    }

    #[test]
    fn fevga_has_no_hits_and_same_direction() {
        let g = Tavli::with_variant(Variant::Fevga, 3);
        assert_eq!(g.ix(0, 24), 23);
        assert_eq!(g.ix(1, 24), 11);
        assert_eq!(g.ix(1, 1), 12);
    }

    #[test]
    fn portes_hits_to_bar() {
        let mut g = Tavli::with_variant(Variant::Portes, 4);
        g.pts = [[0; 24]; 2];
        g.side = Player::One;
        g.dice = vec![2];
        let (a, b) = (g.ix(0, 10), g.ix(0, 8));
        g.pts[0][a] = 1;
        g.pts[1][b] = 1;
        GameObj::play(&mut g, "10/8").unwrap();
        assert_eq!(g.bar[1], 1);
    }

    #[test]
    fn random_games_finish_all_variants() {
        for v in [
            Variant::Portes,
            Variant::Plakoto,
            Variant::Fevga,
            Variant::GulBara,
        ] {
            random_finishes(move |s| Box::new(Tavli::with_variant(v, s)), 3, 6000);
        }
    }

    #[test]
    fn ai_plays_legally() {
        for v in [Variant::Portes, Variant::Fevga, Variant::GulBara] {
            ai_vs_random(
                move |s| Box::new(Tavli::with_variant(v, s)),
                2,
                Player::One,
                8000,
            );
        }
        ai_vs_random(|s| Box::new(Tavli::with_seed(s)), 2, Player::Two, 8000);
    }
}
