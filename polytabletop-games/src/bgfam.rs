//! Motor comun pentru variantele de table cu piesele pornite de pe margine (Acey-deucey, Tabula),
//! pentru doi jucători. Regulile se aleg prin [`Rules`].
//!
//! * 24 de puncte, 15 piese pe jucător, **toate pornesc în afara tablei**. Fiecare zar mută o piesă
//!   (sau o intră pe tablă) cu atâția pași; nu poți ateriza pe un punct cu două sau mai multe piese
//!   adverse; o piesă adversă singură e lovită și trebuie reintrodusă. Piesele pot intra și se pot
//!   mișca în orice ordine (nu există obligația de a reintra mai întâi).
//! * Sensul: la Acey-deucey piesele celor doi merg în sensuri opuse (X intră pe punctele 1–6 și
//!   merge spre 24; O intră pe 24–19 și merge spre 1); la Tabula ambii merg în același sens.
//! * Scoaterea de pe tablă începe când toate cele 15 piese sunt în ultimul sfert; câștigă cine scoate
//!   primul toate piesele.
//! * Zarurile se aruncă automat; regula „joacă cât mai multe zaruri” (și, dacă se poate doar unul,
//!   pe cel mai mare) se respectă. Dacă nimeni nu poate muta 100 de ture la rând, câștigă cine a scos mai
//!   multe piese. AI: `AlphaBeta(0)`, pe pași, fără să vadă zarurile următoare.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone, Copy, Debug)]
pub struct Rules {
    pub name: &'static str,
    pub dice: u8,
    pub opposite: bool,
    pub exact_off: bool,
    /// Piesele din ultimul sfert nu se mai mișcă decât pentru a ieși.
    pub home_locked: bool,
    /// Aruncarea 1-2 (acey-deucey) și dublele: patru mutări + încă o aruncare.
    pub acey: bool,
}

const HOME_Q: i8 = 18;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mv {
    /// `from` = −1 (piesă din afara tablei) sau pozitia q (0..=23) pe drumul propriu.
    Step {
        from: i8,
        die: u8,
    },
    Choose(u8),
}

#[derive(Clone, Copy)]
struct Board {
    pts: [i8; 24],
    bar: [u8; 2],
    off: [u8; 2],
    side: Player,
    dice: [u8; 4],
    nd: u8,
}

fn sgn(p: Player) -> i8 {
    match p {
        Player::One => 1,
        Player::Two => -1,
    }
}

impl Board {
    fn abs(r: &Rules, p: Player, q: i8) -> usize {
        if r.opposite && p == Player::Two {
            23 - q as usize
        } else {
            q as usize
        }
    }

    /// Piese ale lui `p` (pozitiv) sau ale adversarului (negativ) la poziția `q` din drumul lui `p`.
    fn own(&self, r: &Rules, p: Player, q: i8) -> i8 {
        self.pts[Self::abs(r, p, q)] * sgn(p)
    }

    fn all_home(&self, r: &Rules, p: Player) -> bool {
        self.bar[p.idx()] == 0 && (0..HOME_Q).all(|q| self.own(r, p, q) <= 0)
    }

    fn raw_moves(&self, r: &Rules) -> Vec<Mv> {
        let s = self.side;
        let i = s.idx();
        let mut out = Vec::new();
        let mut ds: Vec<u8> = self.dice[..self.nd as usize].to_vec();
        ds.sort_unstable();
        ds.dedup();
        let all_home = self.all_home(r, s);
        let farthest_back = (0..24i8).find(|&q| self.own(r, s, q) > 0);
        for d in ds {
            if self.bar[i] > 0 && self.own(r, s, d as i8 - 1) >= -1 {
                out.push(Mv::Step { from: -1, die: d });
            }
            for q in 0..24i8 {
                if self.own(r, s, q) <= 0 {
                    continue;
                }
                let t = q + d as i8;
                if t < 24 {
                    if r.home_locked && q >= HOME_Q {
                        continue;
                    }
                    if self.own(r, s, t) >= -1 {
                        out.push(Mv::Step { from: q, die: d });
                    }
                } else if all_home && (t == 24 || (!r.exact_off && farthest_back == Some(q))) {
                    out.push(Mv::Step { from: q, die: d });
                }
            }
        }
        out
    }

    /// Joacă un pas; întoarce `true` dacă a lovit o piesă adversă.
    fn step(&mut self, r: &Rules, from: i8, die: u8) -> bool {
        let s = self.side;
        let i = s.idx();
        let o = s.other().idx();
        let sg = sgn(s);
        let q0 = if from < 0 {
            self.bar[i] -= 1;
            -1
        } else {
            let a = Self::abs(r, s, from);
            self.pts[a] -= sg;
            from
        };
        let t = q0 + die as i8;
        let mut hit = false;
        if t < 24 {
            let a = Self::abs(r, s, t);
            if self.pts[a] == -sg {
                self.pts[a] = 0;
                self.bar[o] += 1;
                hit = true;
            }
            self.pts[a] += sg;
        } else {
            self.off[i] += 1;
        }
        let n = self.nd as usize;
        if let Some(k) = self.dice[..n].iter().position(|&d| d == die) {
            for j in k..n - 1 {
                self.dice[j] = self.dice[j + 1];
            }
            self.nd -= 1;
        }
        hit
    }

    fn max_use(&self, r: &Rules) -> u8 {
        self.raw_moves(r)
            .into_iter()
            .map(|m| {
                let mut c = *self;
                if let Mv::Step { from, die } = m {
                    c.step(r, from, die);
                }
                1 + c.max_use(r)
            })
            .max()
            .unwrap_or(0)
    }

    fn pips(&self, r: &Rules, p: Player) -> i32 {
        let mut t = self.bar[p.idx()] as i32 * 25;
        for q in 0..24i8 {
            let c = self.own(r, p, q);
            if c > 0 {
                t += (24 - q as i32) * c as i32;
            }
        }
        t
    }

    fn structure(&self, r: &Rules, p: Player) -> i32 {
        let mut v = 0;
        for q in 0..24i8 {
            match self.own(r, p, q) {
                1 => v -= 3,
                c if c >= 2 => v += 2,
                _ => {}
            }
        }
        v
    }
}

#[derive(Clone)]
pub struct BgFam {
    rules: Rules,
    b: Board,
    rng: Rng,
    over: bool,
    extra: bool,
    choosing: bool,
    played: u8,
    passes: u16,
    roll: Vec<u8>,
    log: String,
}

impl BgFam {
    pub fn new(rules: Rules) -> Self {
        Self::with_seed(rules, Rng::from_clock().next_u64())
    }

    pub fn with_seed(rules: Rules, seed: u64) -> Self {
        let mut g = BgFam {
            rules,
            b: Board {
                pts: [0; 24],
                bar: [15; 2],
                off: [0; 2],
                side: Player::One,
                dice: [0; 4],
                nd: 0,
            },
            rng: Rng::new(seed),
            over: false,
            extra: false,
            choosing: false,
            played: 0,
            passes: 0,
            roll: Vec::new(),
            log: String::new(),
        };
        g.roll_new();
        g.settle();
        g
    }

    fn roll_new(&mut self) {
        let n = self.rules.dice as usize;
        let d: Vec<u8> = (0..n).map(|_| self.rng.d6()).collect();
        self.roll = d.clone();
        self.played = 0;
        self.extra = false;
        self.choosing = false;
        self.b.dice = [0; 4];
        if self.rules.acey && n == 2 && d[0] == d[1] {
            self.b.dice = [d[0]; 4];
            self.b.nd = 4;
            self.extra = true;
        } else if self.rules.acey && n == 2 && d[0] + d[1] == 3 {
            self.b.dice[..2].copy_from_slice(&d);
            self.b.nd = 2;
            self.choosing = true; // după 1 și 2 urmează alegerea dubletului
        } else {
            self.b.dice[..n].copy_from_slice(&d);
            self.b.nd = n as u8;
        }
    }

    fn settle(&mut self) {
        while !self.over {
            if self.b.nd > 0 && !self.b.raw_moves(&self.rules).is_empty() {
                self.passes = 0;
                return;
            }
            if self.b.nd == 0 && self.choosing {
                return;
            }
            if self.played == 0 && self.b.nd > 0 {
                self.passes += 1;
                self.log.push_str(&format!(
                    " Jucătorul {} nu poate muta.",
                    self.b.side.idx() + 1
                ));
            }
            let again = self.extra && self.b.nd == 0 && self.played > 0;
            self.choosing = false;
            if !again {
                self.b.side = self.b.side.other();
            }
            if self.passes >= 100 {
                self.over = true;
                return;
            }
            self.roll_new();
            let d: Vec<String> = self.roll.iter().map(|x| x.to_string()).collect();
            self.log.push_str(&format!(
                " J{} aruncă {}.",
                self.b.side.idx() + 1,
                d.join(" ")
            ));
        }
    }

    fn text(&self, m: &Mv) -> String {
        match *m {
            Mv::Step { from, die } => {
                if from < 0 {
                    format!("ent-{die}")
                } else if from + die as i8 >= 24 {
                    format!("{}-off", from + 1)
                } else {
                    format!("{}-{}", from + 1, from + 1 + die as i8)
                }
            }
            Mv::Choose(n) => format!("dublet {n}"),
        }
    }

    pub fn borne_off(&self) -> (u8, u8) {
        (self.b.off[0], self.b.off[1])
    }
}

impl Game for BgFam {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.b.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if self.over {
            return Vec::new();
        }
        if self.b.nd == 0 && self.choosing {
            return (1..=6).map(Mv::Choose).collect();
        }
        let raw = self.b.raw_moves(&self.rules);
        if raw.is_empty() {
            return raw;
        }
        let scored: Vec<(Mv, u8)> = raw
            .into_iter()
            .map(|m| {
                let mut c = self.b;
                if let Mv::Step { from, die } = m {
                    c.step(&self.rules, from, die);
                }
                (m, 1 + c.max_use(&self.rules))
            })
            .collect();
        let best = scored.iter().map(|x| x.1).max().unwrap_or(0);
        let mut cand: Vec<Mv> = scored
            .into_iter()
            .filter(|x| x.1 == best)
            .map(|x| x.0)
            .collect();
        if best == 1 && self.b.nd >= 2 {
            let maxd = self.b.dice[..self.b.nd as usize]
                .iter()
                .copied()
                .max()
                .unwrap_or(0);
            if cand
                .iter()
                .any(|m| matches!(m, Mv::Step { die, .. } if *die == maxd))
            {
                cand.retain(|m| matches!(m, Mv::Step { die, .. } if *die == maxd));
            }
        }
        let mut seen: Vec<String> = Vec::new();
        cand.retain(|m| {
            let t = self.text(m);
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
        let who = self.b.side.idx() + 1;
        match *m {
            Mv::Choose(n) => {
                self.b.dice = [n; 4];
                self.b.nd = 4;
                self.choosing = false;
                self.extra = true;
                self.log = format!("Jucătorul {who}: dublet de {n}.");
            }
            Mv::Step { from, die } => {
                let hit = self.b.step(&self.rules, from, die);
                self.played += 1;
                self.log = format!(
                    "Jucătorul {who}: {}{}.",
                    self.text(m),
                    if hit { " (lovește)" } else { "" }
                );
                if self.b.off[self.b.side.idx()] == 15 {
                    self.over = true;
                    self.b.nd = 0;
                    return;
                }
            }
        }
        self.settle();
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            return None;
        }
        Some(match self.b.off[0].cmp(&self.b.off[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => match self
                .b
                .pips(&self.rules, Player::One)
                .cmp(&self.b.pips(&self.rules, Player::Two))
            {
                std::cmp::Ordering::Less => Outcome::Win(Player::One),
                std::cmp::Ordering::Greater => Outcome::Win(Player::Two),
                std::cmp::Ordering::Equal => Outcome::Draw,
            },
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let o = p.other();
        let r = &self.rules;
        let race = self.b.pips(r, o) - self.b.pips(r, p);
        race + self.b.structure(r, p) - self.b.structure(r, o)
            + 3 * (self.b.off[p.idx()] as i32 - self.b.off[o.idx()] as i32)
    }

    fn render(&self) -> String {
        let cell = |a: usize| -> String {
            let c = self.b.pts[a];
            match c {
                0 => " . ".to_string(),
                c if c > 0 => format!("{:>2}X", c),
                c => format!("{:>2}O", -c),
            }
        };
        let mut s = format!(
            "{} — punctele sunt numerotate absolut (1–24).\n",
            self.rules.name
        );
        s.push_str(if self.rules.opposite {
            "X intră pe 1–6 și merge spre 24; O intră pe 24–19 și merge spre 1.\n"
        } else {
            "X și O intră pe 1–6 și merg amândoi spre 24.\n"
        });
        s.push_str("  ");
        for n in 13..=24usize {
            s.push_str(&format!("{n:>3} "));
            if n == 18 {
                s.push_str("| ");
            }
        }
        s.push_str("\n  ");
        for n in 13..=24usize {
            s.push_str(&format!("{} ", cell(n - 1)));
            if n == 18 {
                s.push_str("| ");
            }
        }
        s.push_str("\n  ");
        for n in (1..=12usize).rev() {
            s.push_str(&format!("{} ", cell(n - 1)));
            if n == 7 {
                s.push_str("| ");
            }
        }
        s.push_str("\n  ");
        for n in (1..=12usize).rev() {
            s.push_str(&format!("{n:>3} "));
            if n == 7 {
                s.push_str("| ");
            }
        }
        s.push_str(&format!(
            "\n\nÎn afara tablei: X={} O={}   Scoase: X={} O={}\n",
            self.b.bar[0], self.b.bar[1], self.b.off[0], self.b.off[1]
        ));
        if !self.over {
            let l: Vec<String> = self.b.dice[..self.b.nd as usize]
                .iter()
                .map(|d| d.to_string())
                .collect();
            s.push_str(&format!(
                "La mutare: J{} ({}). Zaruri rămase: [{}]{}\n",
                self.b.side.idx() + 1,
                if self.b.side == Player::One { 'X' } else { 'O' },
                l.join(" "),
                if self.choosing && self.b.nd == 0 {
                    " — alege `dublet N`"
                } else {
                    ""
                }
            ));
            s.push_str("Mutări: `ent-3` (intri pe punctul 3), `5-9` (de la 5 la 9), `20-off`. Numerele sunt pozițiile pe drumul tău (1–24).\n");
        }
        s.push_str(self.log.trim());
        s.push('\n');
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        self.text(m)
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn check_variant(rules: Rules) {
        // jocuri aleatoare se termină și zarurile consumate sunt legale
        for seed in 1..=4u64 {
            let mut g = BgFam::with_seed(rules, seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                assert!(!mv.is_empty(), "{}", g.render());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
                assert!(n < 30000);
                let on: i32 = g.b.pts.iter().map(|&c| c.unsigned_abs() as i32).sum();
                let tot = on
                    + g.b.bar[0] as i32
                    + g.b.bar[1] as i32
                    + g.b.off[0] as i32
                    + g.b.off[1] as i32;
                assert_eq!(tot, 30);
            }
        }
        // două piese adverse blochează; o piesă singură e lovită
        let mut g = BgFam::with_seed(rules, 9);
        g.b = Board {
            pts: [0; 24],
            bar: [0; 2],
            off: [0; 2],
            side: Player::One,
            dice: [3, 0, 0, 0],
            nd: 1,
        };
        g.b.pts[2] = 1; // X la q=2
        g.b.bar[0] = 1;
        g.b.off[0] = 13;
        let tgt = Board::abs(&rules, Player::One, 5);
        g.b.pts[tgt] = -2;
        g.b.bar[1] = 13;
        assert!(!g
            .b
            .raw_moves(&rules)
            .contains(&Mv::Step { from: 2, die: 3 }));
        g.b.pts[tgt] = -1;
        g.b.bar[1] = 14;
        assert!(g
            .b
            .raw_moves(&rules)
            .contains(&Mv::Step { from: 2, die: 3 }));
        let hit = g.b.step(&rules, 2, 3);
        assert!(hit);
        assert_eq!(g.b.bar[1], 15);
    }
}
