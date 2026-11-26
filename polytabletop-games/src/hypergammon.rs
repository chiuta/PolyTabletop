//! Hypergammon — table (backgammon) cu **trei** piese pe jucător, din regulile publice.
//!
//! * 24 puncte, 3 piese pe jucător (poziția de start: câte una pe punctele 24, 23 și 22 ale
//!   fiecăruia), bară, bear-off. Fără cub de dublare și fără gammon: primul care scoate cele 3
//!   piese câștigă. Restul regulilor sunt cele ale tablelor: lovire, bară, bear-off.
//! * Zarurile se aruncă automat, în interiorul stării, cu un [`Rng`] propriu
//!   (seed) imediat după fiecare tură. Zarurile rămase se văd în `render()` și
//!   determină `legal_moves()`. Dubla se joacă de patru ori.
//! * O „mutare” este un singur pas cu un singur zar: `13/7`, `bar/22`,
//!   `3/off`. Dacă nu există nicio mutare legală, tura trece automat.
//! * Se respectă regula „joacă cât mai multe zaruri” și, când se poate juca
//!   doar unul, „joacă-l pe cel mai mare”.
//! * Numerotarea punctelor este mereu cea a jucătorului la mutare: 24 = punctul
//!   cel mai îndepărtat, 1 = ultimul punct din casă.
//!
//! AI: `AlphaBeta(0)` (o căutare de un singur nivel: nu citește zarurile viitoare) — alege pasul cu cea mai bună evaluare (pip count, piese
//! singure, puncte făcute). Nu „vede” zarurile viitoare: evaluarea ignoră
//! zarurile deja aruncate pentru tura următoare. Doar `Rng`-ul și zarurile
//! sunt nedeterministe; la același seed, jocul este complet reproductibil.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Mv {
    /// Punctul de plecare din perspectiva mutatorului (1..=24), 25 = bara.
    pub from: u8,
    pub die: u8,
}

#[derive(Clone, Copy)]
struct Board {
    /// Index absolut i = punctul i+1 al lui Unu; + = piese Unu, - = piese Doi.
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

fn idx(p: Player, n: u8) -> usize {
    match p {
        Player::One => n as usize - 1,
        Player::Two => 24 - n as usize,
    }
}

impl Board {
    fn start() -> Board {
        let mut b = Board::empty();
        for (n, c) in [(24u8, 1i8), (23, 1), (22, 1)] {
            b.pts[idx(Player::One, n)] = c;
            b.pts[idx(Player::Two, n)] = -c;
        }
        b
    }

    fn empty() -> Board {
        Board {
            pts: [0; 24],
            bar: [0; 2],
            off: [0; 2],
            side: Player::One,
            dice: [0; 4],
            nd: 0,
        }
    }

    /// Piese proprii (pozitiv) sau ale adversarului (negativ) pe punctul `n` al lui `s`.
    fn own(&self, s: Player, n: u8) -> i8 {
        self.pts[idx(s, n)] * sgn(s)
    }

    fn raw_moves(&self) -> Vec<Mv> {
        let s = self.side;
        let i = s.idx();
        let mut out = Vec::new();
        let mut ds: Vec<u8> = self.dice[..self.nd as usize].to_vec();
        ds.sort_unstable();
        ds.dedup();
        let all_home = self.bar[i] == 0 && (7..=24).all(|n| self.own(s, n) <= 0);
        let highest = (1..=24u8).rev().find(|&n| self.own(s, n) > 0);
        for d in ds {
            if self.bar[i] > 0 {
                if self.own(s, 25 - d) >= -1 {
                    out.push(Mv { from: 25, die: d });
                }
                continue;
            }
            for n in 1..=24u8 {
                if self.own(s, n) <= 0 {
                    continue;
                }
                if n > d {
                    if self.own(s, n - d) >= -1 {
                        out.push(Mv { from: n, die: d });
                    }
                } else if all_home && (n == d || highest == Some(n)) {
                    out.push(Mv { from: n, die: d });
                }
            }
        }
        out
    }

    /// Joacă un pas (fără aruncare). Returnează `true` dacă a lovit o piesă.
    fn step(&mut self, m: Mv) -> bool {
        let s = self.side;
        let i = s.idx();
        let o = s.other().idx();
        let sg = sgn(s);
        let to: i16 = if m.from == 25 {
            self.bar[i] -= 1;
            25 - m.die as i16
        } else {
            self.pts[idx(s, m.from)] -= sg;
            m.from as i16 - m.die as i16
        };
        let mut hit = false;
        if to >= 1 {
            let t = idx(s, to as u8);
            if self.pts[t] == -sg {
                self.pts[t] = 0;
                self.bar[o] += 1;
                hit = true;
            }
            self.pts[t] += sg;
        } else {
            self.off[i] += 1;
        }
        // elimină un zar cu valoarea folosită
        let n = self.nd as usize;
        if let Some(k) = self.dice[..n].iter().position(|&d| d == m.die) {
            for j in k..n - 1 {
                self.dice[j] = self.dice[j + 1];
            }
            self.nd -= 1;
        }
        hit
    }

    fn max_use(&self) -> u8 {
        self.raw_moves()
            .into_iter()
            .map(|m| {
                let mut c = *self;
                c.step(m);
                1 + c.max_use()
            })
            .max()
            .unwrap_or(0)
    }

    fn pips(&self, p: Player) -> i32 {
        let mut t = self.bar[p.idx()] as i32 * 25;
        for n in 1..=24u8 {
            let c = self.own(p, n);
            if c > 0 {
                t += n as i32 * c as i32;
            }
        }
        t
    }

    fn structure(&self, p: Player) -> i32 {
        let mut v = 0;
        for n in 1..=24u8 {
            match self.own(p, n) {
                1 => v -= 4,
                c if c >= 2 => v += 2,
                _ => {}
            }
        }
        v
    }
}

#[derive(Clone)]
pub struct Hypergammon {
    b: Board,
    roll: (u8, u8),
    played: u8,
    rng: Rng,
    over: bool,
    log: String,
}

impl Default for Hypergammon {
    fn default() -> Self {
        Self::new()
    }
}

impl Hypergammon {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let (a, b) = loop {
            let a = rng.d6();
            let b = rng.d6();
            if a != b {
                break (a, b);
            }
        };
        let mut g = Hypergammon {
            b: Board::start(),
            roll: (a, b),
            played: 0,
            rng,
            over: false,
            log: String::new(),
        };
        g.b.side = if a > b { Player::One } else { Player::Two };
        g.b.dice[0] = a;
        g.b.dice[1] = b;
        g.b.nd = 2;
        g.log = format!("Zar de deschidere: {a} și {b}.");
        g.settle();
        g
    }

    /// Piese scoase de pe tablă: (Unu, Doi).
    pub fn borne_off(&self) -> (u8, u8) {
        (self.b.off[0], self.b.off[1])
    }

    /// Zarurile încă de jucat în tura curentă.
    pub fn dice_left(&self) -> Vec<u8> {
        self.b.dice[..self.b.nd as usize].to_vec()
    }

    fn roll_new(&mut self) {
        let a = self.rng.d6();
        let b = self.rng.d6();
        self.roll = (a, b);
        self.played = 0;
        self.b.dice = if a == b { [a; 4] } else { [a, b, 0, 0] };
        self.b.nd = if a == b { 4 } else { 2 };
    }

    /// Încheie automat tura când nu mai există mutări; aruncă pentru următorul.
    fn settle(&mut self) {
        while !self.over {
            if self.b.nd > 0 && !self.b.raw_moves().is_empty() {
                return;
            }
            if self.played == 0 {
                self.log.push_str(&format!(
                    " Jucătorul {} nu are mutări legale și pasează.",
                    self.b.side.idx() + 1
                ));
            }
            self.b.side = self.b.side.other();
            self.roll_new();
            self.log.push_str(&format!(
                " Jucătorul {} aruncă {} și {}.",
                self.b.side.idx() + 1,
                self.roll.0,
                self.roll.1
            ));
        }
    }

    fn text(&self, m: &Mv) -> String {
        if m.from == 25 {
            format!("bar/{}", 25 - m.die)
        } else if m.from > m.die {
            format!("{}/{}", m.from, m.from - m.die)
        } else {
            format!("{}/off", m.from)
        }
    }
}

impl Game for Hypergammon {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.b.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if self.over {
            return Vec::new();
        }
        let raw = self.b.raw_moves();
        if raw.is_empty() {
            return raw;
        }
        let scored: Vec<(Mv, u8)> = raw
            .into_iter()
            .map(|m| {
                let mut c = self.b;
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
        if best == 1 && self.b.nd >= 2 {
            let maxd = self.b.dice[..self.b.nd as usize]
                .iter()
                .copied()
                .max()
                .unwrap_or(0);
            if cand.iter().any(|m| m.die == maxd) {
                cand.retain(|m| m.die == maxd);
            }
        }
        // același text (bear-off cu zaruri diferite) = aceeași mutare
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
        let hit = self.b.step(*m);
        self.played += 1;
        self.log = format!(
            "Jucătorul {who}: {}{}.",
            self.text(m),
            if hit { " (lovește)" } else { "" }
        );
        if self.b.off[self.b.side.idx()] == 3 {
            self.over = true;
            self.b.nd = 0;
            return;
        }
        self.settle();
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.over {
            Some(Outcome::Win(if self.b.off[0] == 3 {
                Player::One
            } else {
                Player::Two
            }))
        } else {
            None
        }
    }

    fn evaluate(&self, p: Player) -> i32 {
        let o = p.other();
        let race = self.b.pips(o) - self.b.pips(p);
        race + self.b.structure(p) - self.b.structure(o)
    }

    fn render(&self) -> String {
        let s = self.b.side;
        let cell = |n: u8| -> String {
            let c = self.b.own(s, n);
            let (mine, theirs) = (sym(s), sym(s.other()));
            match c {
                0 => " . ".to_string(),
                c if c > 0 => format!("{:>2}{}", c, mine),
                c => format!("{:>2}{}", -c, theirs),
            }
        };
        let mut out = String::new();
        out.push_str(&format!(
            "Punctele sunt numerotate din perspectiva jucătorului {} ({}).\n\n",
            s.idx() + 1,
            sym(s)
        ));
        out.push_str("  ");
        for n in 13..=24u8 {
            out.push_str(&format!("{n:>3} "));
            if n == 18 {
                out.push_str("| ");
            }
        }
        out.push_str("\n  ");
        for n in 13..=24u8 {
            out.push_str(&format!("{} ", cell(n)));
            if n == 18 {
                out.push_str("| ");
            }
        }
        out.push_str("\n  ");
        for n in (1..=12u8).rev() {
            out.push_str(&format!("{} ", cell(n)));
            if n == 7 {
                out.push_str("| ");
            }
        }
        out.push_str("\n  ");
        for n in (1..=12u8).rev() {
            out.push_str(&format!("{n:>3} "));
            if n == 7 {
                out.push_str("| ");
            }
        }
        out.push_str(&format!(
            "\n\nBară: X={} O={}   Scoase: X={} O={}\n",
            self.b.bar[0], self.b.bar[1], self.b.off[0], self.b.off[1]
        ));
        if !self.over {
            out.push_str(&format!(
                "Zaruri aruncate: {} și {}   Rămase: {:?}\n",
                self.roll.0,
                self.roll.1,
                self.dice_left()
            ));
        }
        out.push_str(self.log.trim());
        out.push('\n');
        out
    }

    fn format_move(&self, m: &Mv) -> String {
        self.text(m)
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

fn sym(p: Player) -> char {
    match p {
        Player::One => 'X',
        Player::Two => 'O',
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    #[test]
    fn start_has_three_each() {
        let b = Board::start();
        let one: i32 = b.pts.iter().filter(|&&x| x > 0).map(|&x| x as i32).sum();
        let two: i32 = b.pts.iter().filter(|&&x| x < 0).map(|&x| -x as i32).sum();
        assert_eq!((one, two), (3, 3));
        assert_eq!(b.own(Player::One, 24), 1);
        assert_eq!(b.own(Player::One, 22), 1);
        assert_eq!(b.own(Player::One, 1), -1); // punctul 24 al lui Doi
        assert_eq!(b.pips(Player::One), 69);
    }

    #[test]
    fn opening_has_moves_and_game_ends() {
        for seed in 1..=6u64 {
            let mut g = Hypergammon::with_seed(seed);
            assert!(!g.legal_moves().is_empty());
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() && n < 3000 {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed}");
        }
    }
}
