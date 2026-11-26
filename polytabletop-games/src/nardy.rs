#![allow(clippy::needless_range_loop, clippy::manual_range_contains)]
//! Jacquet (Franța) și Moultezim (Grecia / Turcia) — table fără lovire, cu 15 piese pe jucător.
//!
//! Ambele sunt „curse paralele”: ambii jucători mută în **același sens** pe un traseu de 24 de puncte
//! (numerotat 1–24 din perspectiva celui la mutare; 1 = punctul de plecare propriu), plecând toți cei
//! 15 din punctul 1, în colțuri opuse ale tablei. **Nu se lovește**: un punct ocupat de adversar
//! (măcar o piesă) e blocat; pe punctele proprii piesele se stivuiesc.
//!
//! * Zarurile se aruncă automat; fiecare zar = un pas al unei piese (`3>8` = piesa de pe punctul 3
//!   avansează la 8). Dubla se joacă de patru ori; trebuie jucate cât mai multe zaruri, iar dacă se
//!   poate juca doar unul, zarul mai mare. Lipsa mutării = tura trece.
//! * **Bear-off**: doar când toate cele 15 piese sunt în ultimul sfert (punctele 19–24). O piesă iese
//!   cu zarul egal cu distanța ei (`22>off`); cu un zar mai mare iese piesa cea mai îndepărtată de
//!   margine. Primul care scoate toate cele 15 piese câștigă.
//! * **Jacquet**: prima piesă mutată e „poștașul”; nicio altă piesă nu are voie să se miște până
//!   când poștașul nu a intrat în ultimul sfert (19–24).
//! * **Moultezim**: o piesă trebuie dusă dincolo de punctul de plecare al adversarului (punctul 13)
//!   înainte de a muta a doua piesă.
//! * Simplificări: fără scor dublu pentru „mars”, fără regulile speciale de prime (șase puncte
//!   alăturate) ale unor regiuni; dacă ambii jucători sunt blocați de 6 ori la rând, câștigă cine a
//!   scos mai multe piese (apoi cine are de parcurs mai puțin; egal = remiză).

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Variant {
    Jacquet,
    Moultezim,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Mv {
    /// indicele traseului 0..=23
    pub from: u8,
    pub die: u8,
}

#[derive(Clone)]
pub struct Nardy {
    var: Variant,
    c: [[u8; 24]; 2],
    off: [u8; 2],
    side: Player,
    dice: [u8; 4],
    nd: u8,
    roll: (u8, u8),
    played: u8,
    post: [Option<u8>; 2],
    passes: u8,
    rng: Rng,
    over: bool,
    log: String,
}

impl Nardy {
    pub fn jacquet() -> Self {
        Self::with_seed(Variant::Jacquet, Rng::from_clock().next_u64())
    }

    pub fn moultezim() -> Self {
        Self::with_seed(Variant::Moultezim, Rng::from_clock().next_u64())
    }

    pub fn with_seed(var: Variant, seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let (a, b) = loop {
            let a = rng.d6();
            let b = rng.d6();
            if a != b {
                break (a, b);
            }
        };
        let mut c = [[0u8; 24]; 2];
        c[0][0] = 15;
        c[1][0] = 15;
        let mut g = Nardy {
            var,
            c,
            off: [0, 0],
            side: if a > b { Player::One } else { Player::Two },
            dice: [a, b, 0, 0],
            nd: 2,
            roll: (a, b),
            played: 0,
            post: [None, None],
            passes: 0,
            rng,
            over: false,
            log: format!("Zar de deschidere: {a} și {b}."),
        };
        g.settle();
        g
    }

    pub fn borne_off(&self) -> (u8, u8) {
        (self.off[0], self.off[1])
    }

    fn offset(p: usize) -> usize {
        if p == 0 {
            0
        } else {
            12
        }
    }

    /// Sunt piese ale adversarului pe punctul absolut `a`?
    fn opp_at(&self, me: usize, a: usize) -> bool {
        let o = 1 - me;
        let j = (a + 24 - Self::offset(o)) % 24;
        self.c[o][j] > 0
    }

    fn all_home(&self, me: usize) -> bool {
        self.c[me][18..].iter().map(|&x| x as u32).sum::<u32>() + self.off[me] as u32 == 15
    }

    fn allowed_from(&self, me: usize, k: usize) -> bool {
        match self.var {
            Variant::Jacquet => match self.post[me] {
                None => true,
                Some(pp) => pp as usize >= 18 || k == pp as usize,
            },
            Variant::Moultezim => {
                let leader = (0..24).rev().find(|&i| self.c[me][i] > 0).unwrap_or(0);
                leader >= 13 || k == leader
            }
        }
    }

    fn raw_moves(&self) -> Vec<Mv> {
        let me = self.side.idx();
        let mut ds: Vec<u8> = self.dice[..self.nd as usize].to_vec();
        ds.sort_unstable();
        ds.dedup();
        let home = self.all_home(me);
        let lowest = (0..24).find(|&i| self.c[me][i] > 0);
        let mut out = Vec::new();
        for d in ds {
            for k in 0..24usize {
                if self.c[me][k] == 0 || !self.allowed_from(me, k) {
                    continue;
                }
                let t = k + d as usize;
                if t < 24 {
                    let a = (t + Self::offset(me)) % 24;
                    if !self.opp_at(me, a) {
                        out.push(Mv {
                            from: k as u8,
                            die: d,
                        });
                    }
                } else if home && (t == 24 || lowest == Some(k)) {
                    out.push(Mv {
                        from: k as u8,
                        die: d,
                    });
                }
            }
        }
        out
    }

    fn step(&mut self, m: Mv) {
        let me = self.side.idx();
        let k = m.from as usize;
        self.c[me][k] -= 1;
        let t = k + m.die as usize;
        if t < 24 {
            self.c[me][t] += 1;
        } else {
            self.off[me] += 1;
        }
        if self.var == Variant::Jacquet {
            match self.post[me] {
                None => self.post[me] = Some(t.min(23) as u8),
                Some(pp) if pp as usize == k => self.post[me] = Some(t.min(24) as u8),
                _ => {}
            }
        }
        let n = self.nd as usize;
        if let Some(i) = self.dice[..n].iter().position(|&d| d == m.die) {
            for j in i..n - 1 {
                self.dice[j] = self.dice[j + 1];
            }
            self.nd -= 1;
        }
    }

    fn max_use(&self) -> u8 {
        self.raw_moves()
            .into_iter()
            .map(|m| {
                let mut c = self.clone();
                c.step(m);
                1 + c.max_use()
            })
            .max()
            .unwrap_or(0)
    }

    fn pips(&self, p: usize) -> i32 {
        (0..24).map(|k| self.c[p][k] as i32 * (24 - k as i32)).sum()
    }

    fn roll_new(&mut self) {
        let a = self.rng.d6();
        let b = self.rng.d6();
        self.roll = (a, b);
        self.played = 0;
        self.dice = if a == b { [a; 4] } else { [a, b, 0, 0] };
        self.nd = if a == b { 4 } else { 2 };
    }

    fn settle(&mut self) {
        while !self.over {
            if self.nd > 0 && !self.raw_moves().is_empty() {
                return;
            }
            if self.played == 0 {
                self.passes += 1;
                self.log.push_str(&format!(
                    " J{} nu are mutări și pasează.",
                    self.side.idx() + 1
                ));
                if self.passes >= 6 {
                    self.over = true;
                    self.nd = 0;
                    return;
                }
            } else {
                self.passes = 0;
            }
            self.side = self.side.other();
            self.roll_new();
            self.log.push_str(&format!(
                " J{} aruncă {} și {}.",
                self.side.idx() + 1,
                self.roll.0,
                self.roll.1
            ));
        }
    }

    fn text(&self, m: &Mv) -> String {
        let t = m.from as usize + m.die as usize;
        if t >= 24 {
            format!("{}>off", m.from + 1)
        } else {
            format!("{}>{}", m.from + 1, t + 1)
        }
    }
}

impl Game for Nardy {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if self.over {
            return Vec::new();
        }
        let raw = self.raw_moves();
        if raw.is_empty() {
            return raw;
        }
        let scored: Vec<(Mv, u8)> = raw
            .into_iter()
            .map(|m| {
                let mut c = self.clone();
                c.step(m);
                (m, 1 + c.max_use())
            })
            .collect();
        let best = scored.iter().map(|x| x.1).max().unwrap_or(0);
        let mut cand: Vec<Mv> = scored
            .into_iter()
            .filter(|x| x.1 == best)
            .map(|x| x.0)
            .collect();
        if best == 1 && self.nd >= 2 {
            let maxd = self.dice[..self.nd as usize]
                .iter()
                .copied()
                .max()
                .unwrap_or(0);
            if cand.iter().any(|m| m.die == maxd) {
                cand.retain(|m| m.die == maxd);
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
        let who = self.side.idx() + 1;
        self.step(*m);
        self.played += 1;
        self.log = format!("J{who}: {}.", self.text(m));
        if self.off[self.side.idx()] == 15 {
            self.over = true;
            self.nd = 0;
            return;
        }
        self.settle();
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            return None;
        }
        if self.off[0] == 15 {
            return Some(Outcome::Win(Player::One));
        }
        if self.off[1] == 15 {
            return Some(Outcome::Win(Player::Two));
        }
        let key = |p: usize| (self.off[p] as i32, -self.pips(p));
        Some(match key(0).cmp(&key(1)) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let (me, op) = (p.idx(), 1 - p.idx());
        let mut v = self.pips(op) - self.pips(me);
        // mobilitate: a nu lăsa piese în urma blocajelor
        v += (self.off[me] as i32 - self.off[op] as i32) * 3;
        v
    }

    fn render(&self) -> String {
        let me = self.side.idx();
        let o = 1 - me;
        let cell = |k: usize| -> String {
            let mine = self.c[me][k];
            let a = (k + Self::offset(me)) % 24;
            let j = (a + 24 - Self::offset(o)) % 24;
            let theirs = self.c[o][j];
            if mine > 0 {
                format!("{:>2}X", mine)
            } else if theirs > 0 {
                format!("{:>2}O", theirs)
            } else {
                " . ".to_string()
            }
        };
        let (name,) = (match self.var {
            Variant::Jacquet => "Jacquet",
            Variant::Moultezim => "Moultezim",
        },);
        let mut s = format!(
            "{name} — traseul lui J{} (X = piesele lui, O = adversarul), puncte 1–24 în sensul de mers\n",
            me + 1
        );
        s.push_str("  ");
        for k in 0..12 {
            s.push_str(&format!("{:>3} ", k + 1));
        }
        s.push_str("\n  ");
        for k in 0..12 {
            s.push_str(&format!("{} ", cell(k)));
        }
        s.push_str("\n  ");
        for k in 12..24 {
            s.push_str(&format!("{} ", cell(k)));
        }
        s.push_str("\n  ");
        for k in 12..24 {
            s.push_str(&format!("{:>3} ", k + 1));
        }
        s.push_str(&format!(
            "\nScoase: J1 {}, J2 {}\n",
            self.off[0], self.off[1]
        ));
        if !self.over {
            s.push_str(&format!(
                "Zaruri: {} și {}   Rămase: {:?}\n",
                self.roll.0,
                self.roll.1,
                &self.dice[..self.nd as usize]
            ));
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
mod tests {
    use super::*;

    fn with(var: Variant) -> Nardy {
        Nardy::with_seed(var, 1)
    }

    #[test]
    fn start_is_fifteen_on_point_one() {
        for v in [Variant::Jacquet, Variant::Moultezim] {
            let g = with(v);
            assert_eq!(g.c[0][0], 15);
            assert_eq!(g.c[1][0], 15);
            assert_eq!(g.pips(0), 360);
            assert!(!g.legal_moves().is_empty());
        }
    }

    #[test]
    fn no_hitting_opponent_point_is_blocked() {
        let mut g = with(Variant::Moultezim);
        g.c = [[0; 24]; 2];
        g.c[0][5] = 2; // J1 progres 5 (abs 5)
        g.c[1][17] = 1; // J2 progres 17 → abs 5 (17+12=29%24=5)
        g.c[0][20] = 13;
        g.side = Player::One;
        g.dice = [1, 0, 0, 0];
        g.nd = 1;
        g.roll = (1, 1);
        // piesa de pe 4 (progres 4) ar ateriza pe progres 5 — aici testăm blocajul din 5→? : mută 4→5 nu există; testează 3→? punem piesă pe 4
        g.c[0][4] = 1;
        let mv = g.legal_moves();
        assert!(mv.iter().all(|m| !(m.from == 4 && m.die == 1)), "{mv:?}");
    }

    #[test]
    fn jacquet_courier_rule() {
        let mut g = with(Variant::Jacquet);
        g.c = [[0; 24]; 2];
        g.c[0][0] = 14;
        g.c[0][3] = 1;
        g.c[1][0] = 15;
        g.post = [Some(3), None];
        g.side = Player::One;
        g.dice = [2, 0, 0, 0];
        g.nd = 1;
        let mv = g.legal_moves();
        assert!(mv.iter().all(|m| m.from == 3), "{mv:?}");
        g.post = [Some(19), None];
        let mv = g.legal_moves();
        assert!(mv.iter().any(|m| m.from == 0));
    }

    #[test]
    fn moultezim_start_rule_and_bear_off() {
        let mut g = with(Variant::Moultezim);
        g.c = [[0; 24]; 2];
        g.c[0][0] = 14;
        g.c[0][5] = 1;
        g.c[1][0] = 15;
        g.side = Player::One;
        g.dice = [2, 0, 0, 0];
        g.nd = 1;
        assert!(g.legal_moves().iter().all(|m| m.from == 5));
        // bear-off
        let mut h = with(Variant::Moultezim);
        h.c = [[0; 24]; 2];
        h.c[0][22] = 15;
        h.c[1][0] = 15;
        h.side = Player::One;
        h.dice = [2, 0, 0, 0];
        h.nd = 1;
        let mv = h.legal_moves();
        assert_eq!(mv.len(), 1);
        assert_eq!(h.text(&mv[0]), "23>off");
    }

    #[test]
    fn random_games_end() {
        for (i, v) in [Variant::Jacquet, Variant::Moultezim]
            .into_iter()
            .enumerate()
        {
            for seed in 1..=3u64 {
                let mut g = Nardy::with_seed(v, seed);
                let mut rng = Rng::new(seed + i as u64);
                let mut n = 0;
                while g.outcome().is_none() && n < 20000 {
                    let mv = g.legal_moves();
                    g.apply(&mv[rng.below(mv.len())]);
                    n += 1;
                }
                assert!(g.outcome().is_some(), "{v:?} seed {seed}");
                assert!(n < 3500, "{v:?} seed {seed}: {n} mutări");
            }
        }
    }
}
