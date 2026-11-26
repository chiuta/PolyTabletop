//! Motor comun pentru jocurile de tip „table” cu o singură cale: Long Nard (Narde) și Duodecim scripta.
//!
//! * `LONG_NARD`: 24 de puncte, 15 piese pe jucător, două zaruri. Toate piesele pornesc din „cap”
//!   (punctul 1), cele două părți își parcurg cercul în același sens, începând din colțuri opuse. Nu se
//!   lovește: un punct ocupat de adversar e blocat. Dintr-un cap se ia o singură piesă pe tură (în prima
//!   tură a fiecăruia, cu dublă 3-3 sau 4-4, două). Nu poți forma șase puncte consecutive proprii dacă
//!   adversarul nu are nicio piesă mai înainte. Scoaterea începe când toate piesele sunt în ultimele șase
//!   puncte; primul care scoate 15 câștigă (fără „mars”).
//! * `DUODECIM`: reconstituire modernă (regulile originale nu s-au păstrat): 3 rânduri × 12 = 36 de puncte
//!   pe o singură cale parcursă de ambii în același sens, 15 piese fiecare, trei zaruri, fiecare zar un pas
//!   separat. Piesele intră de pe „bară” pe punctul egal cu zarul. Un punct cu două piese e blocat; o piesă
//!   singură lovită se întoarce pe bară. Scoaterea din ultimele șase puncte.
//! * Zarurile se aruncă automat în starea jocului; mutări: `7/10`, `bar/3`, `22/off`. Regula „joacă cât mai
//!   multe zaruri” se respectă.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Rules {
    pub n: usize,
    pub hits: bool,
    pub head_rule: bool,
    pub six_block: bool,
    pub ndice: usize,
    pub offset2: usize,
    pub bar_entry: bool,
    pub name: &'static str,
}

pub const LONG_NARD: Rules = Rules {
    n: 24,
    hits: false,
    head_rule: true,
    six_block: true,
    ndice: 2,
    offset2: 12,
    bar_entry: false,
    name: "Long Nard",
};
pub const DUODECIM: Rules = Rules {
    n: 36,
    hits: true,
    head_rule: false,
    six_block: false,
    ndice: 3,
    offset2: 0,
    bar_entry: true,
    name: "Duodecim scripta",
};

const MAX_TURNS: u32 = 800;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Mv {
    /// poziția de plecare în coordonatele mutatorului (1..=n), 0 = bară
    pub from: u8,
    pub die: u8,
}

#[derive(Clone)]
struct Board {
    r: Rules,
    pts: [i8; 36],
    bar: [u8; 2],
    off: [u8; 2],
    side: Player,
    dice: [u8; 4],
    nd: u8,
    head_taken: u8,
    head_limit: u8,
    turns: [u32; 2],
}

fn sgn(p: Player) -> i8 {
    match p {
        Player::One => 1,
        Player::Two => -1,
    }
}

impl Board {
    fn new(r: Rules) -> Board {
        let mut b = Board {
            r,
            pts: [0; 36],
            bar: [0; 2],
            off: [0; 2],
            side: Player::One,
            dice: [0; 4],
            nd: 0,
            head_taken: 0,
            head_limit: 1,
            turns: [0; 2],
        };
        if r.bar_entry {
            b.bar = [15, 15];
        } else {
            b.pts[g(&r, Player::One, 1)] = 15;
            b.pts[g(&r, Player::Two, 1)] = -15;
        }
        b
    }

    /// Câte piese proprii (pozitiv) pe poziția `p` a lui `s`.
    fn own(&self, s: Player, p: usize) -> i8 {
        self.pts[g(&self.r, s, p)] * sgn(s)
    }

    fn all_home(&self, s: Player) -> bool {
        let n = self.r.n;
        self.bar[s.idx()] == 0 && (1..=n - 6).all(|p| self.own(s, p) <= 0)
    }

    fn blocked(&self, s: Player, p: usize) -> bool {
        let e = -self.own(s, p);
        if self.r.hits {
            e >= 2
        } else {
            e >= 1
        }
    }

    /// Șase puncte consecutive proprii fără nicio piesă adversă „înainte”?
    fn illegal_block(&self, s: Player) -> bool {
        if !self.r.six_block {
            return false;
        }
        let n = self.r.n;
        // cea mai înaintată piesă adversă, în coordonatele lui s
        let mut opp_max = 0;
        for p in 1..=n {
            if self.own(s, p) < 0 {
                opp_max = p;
            }
        }
        let mut run = 0;
        for p in 1..=n {
            if self.own(s, p) > 0 {
                run += 1;
                // blocul se termină la p (sau continuă); ilegal dacă nicio piesă adversă nu e dincolo de p
                let ends = p == n || self.own(s, p + 1) <= 0;
                if run >= 6 && ends && opp_max <= p {
                    return true;
                }
            } else {
                run = 0;
            }
        }
        false
    }

    fn raw_moves(&self) -> Vec<Mv> {
        let s = self.side;
        let n = self.r.n;
        let i = s.idx();
        let mut out = Vec::new();
        let mut ds: Vec<u8> = self.dice[..self.nd as usize].to_vec();
        ds.sort_unstable();
        ds.dedup();
        let all_home = self.all_home(s);
        let rear = (1..=n).find(|&p| self.own(s, p) > 0);
        for d in ds {
            let du = d as usize;
            if self.bar[i] > 0 {
                if !self.blocked(s, du) {
                    out.push(Mv { from: 0, die: d });
                }
                continue;
            }
            for p in 1..=n {
                if self.own(s, p) <= 0 {
                    continue;
                }
                if self.r.head_rule && p == 1 && self.head_taken >= self.head_limit {
                    continue;
                }
                let t = p + du;
                if t <= n {
                    if self.blocked(s, t) {
                        continue;
                    }
                    if self.r.six_block {
                        let mut c = self.clone();
                        c.step(Mv {
                            from: p as u8,
                            die: d,
                        });
                        if c.illegal_block(s) {
                            continue;
                        }
                    }
                    out.push(Mv {
                        from: p as u8,
                        die: d,
                    });
                } else if all_home && (t == n + 1 || rear == Some(p)) {
                    out.push(Mv {
                        from: p as u8,
                        die: d,
                    });
                }
            }
        }
        out
    }

    fn step(&mut self, m: Mv) -> bool {
        let s = self.side;
        let i = s.idx();
        let n = self.r.n;
        let sg = sgn(s);
        let mut hit = false;
        let from = m.from as usize;
        let t = if from == 0 {
            self.bar[i] -= 1;
            m.die as usize
        } else {
            let gi = g(&self.r, s, from);
            self.pts[gi] -= sg;
            if self.r.head_rule && from == 1 {
                self.head_taken += 1;
            }
            from + m.die as usize
        };
        if t <= n {
            let gi = g(&self.r, s, t);
            if self.r.hits && self.pts[gi] == -sg {
                self.pts[gi] = 0;
                self.bar[s.other().idx()] += 1;
                hit = true;
            }
            self.pts[gi] += sg;
        } else {
            self.off[i] += 1;
        }
        let nd = self.nd as usize;
        if let Some(k) = self.dice[..nd].iter().position(|&d| d == m.die) {
            for j in k..nd - 1 {
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
                let mut c = self.clone();
                c.step(m);
                1 + c.max_use()
            })
            .max()
            .unwrap_or(0)
    }

    fn pips(&self, p: Player) -> i32 {
        let n = self.r.n;
        let mut t = self.bar[p.idx()] as i32 * (n as i32 + 1);
        for q in 1..=n {
            let c = self.own(p, q);
            if c > 0 {
                t += (n as i32 + 1 - q as i32) * c as i32;
            }
        }
        t
    }
}

/// Indexul global al punctului `p` (1..=n) al jucătorului `s`.
fn g(r: &Rules, s: Player, p: usize) -> usize {
    match s {
        Player::One => p - 1,
        Player::Two => (p - 1 + r.offset2) % r.n,
    }
}

#[derive(Clone)]
pub struct DiceRace {
    b: Board,
    rng: Rng,
    played: u8,
    over: bool,
    plies: u32,
    log: String,
}

impl DiceRace {
    pub fn new(r: Rules) -> Self {
        Self::with_seed(r, Rng::from_clock().next_u64())
    }

    pub fn with_seed(r: Rules, seed: u64) -> Self {
        let mut g = DiceRace {
            b: Board::new(r),
            rng: Rng::new(seed),
            played: 0,
            over: false,
            plies: 0,
            log: String::new(),
        };
        g.roll_new();
        g.settle();
        g
    }

    pub fn borne_off(&self) -> (u8, u8) {
        (self.b.off[0], self.b.off[1])
    }

    fn roll_new(&mut self) {
        let k = self.b.r.ndice;
        let mut ds = Vec::new();
        for _ in 0..k {
            ds.push(self.rng.d6());
        }
        self.played = 0;
        self.b.head_taken = 0;
        let first = self.b.turns[self.b.side.idx()] == 0;
        self.b.head_limit = if self.b.r.head_rule
            && first
            && k == 2
            && ds[0] == ds[1]
            && (ds[0] == 3 || ds[0] == 4)
        {
            2
        } else {
            1
        };
        self.b.turns[self.b.side.idx()] += 1;
        if k == 2 && ds[0] == ds[1] {
            self.b.dice = [ds[0]; 4];
            self.b.nd = 4;
        } else {
            for (j, d) in ds.iter().enumerate() {
                self.b.dice[j] = *d;
            }
            self.b.nd = k as u8;
        }
        self.log = format!(
            "Jucătorul {} aruncă {}.",
            self.b.side.idx() + 1,
            ds.iter()
                .map(|d| d.to_string())
                .collect::<Vec<_>>()
                .join(" și ")
        );
    }

    fn settle(&mut self) {
        let mut guard = 0;
        while !self.over {
            if self.b.nd > 0 && !self.b.raw_moves().is_empty() {
                return;
            }
            if self.played == 0 {
                self.log
                    .push_str(&format!(" J{} nu poate muta.", self.b.side.idx() + 1));
            }
            self.b.side = self.b.side.other();
            self.roll_new();
            guard += 1;
            if guard > 3000 {
                self.over = true;
            }
            if self.b.turns[0] + self.b.turns[1] > 2 * MAX_TURNS {
                self.over = true;
            }
        }
    }

    fn text(&self, m: &Mv) -> String {
        let n = self.b.r.n;
        let t = if m.from == 0 {
            m.die as usize
        } else {
            m.from as usize + m.die as usize
        };
        let from = if m.from == 0 {
            "bar".to_string()
        } else {
            m.from.to_string()
        };
        if t > n {
            format!("{from}/off")
        } else {
            format!("{from}/{t}")
        }
    }
}

impl Game for DiceRace {
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
                let mut c = self.b.clone();
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
        self.plies += 1;
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
        self.settle();
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            return None;
        }
        if self.b.off[0] == 15 {
            Some(Outcome::Win(Player::One))
        } else if self.b.off[1] == 15 {
            Some(Outcome::Win(Player::Two))
        } else {
            // limita de tururi: câștigă cine are mai puțin de mers
            let (a, b) = (self.b.pips(Player::One), self.b.pips(Player::Two));
            Some(match a.cmp(&b) {
                std::cmp::Ordering::Less => Outcome::Win(Player::One),
                std::cmp::Ordering::Greater => Outcome::Win(Player::Two),
                std::cmp::Ordering::Equal => Outcome::Draw,
            })
        }
    }

    fn evaluate(&self, p: Player) -> i32 {
        let o = p.other();
        let mut v = self.b.pips(o) - self.b.pips(p);
        if self.b.r.hits {
            // piesele singure sunt vulnerabile
            for q in 1..=self.b.r.n {
                if self.b.own(p, q) == 1 {
                    v -= 3;
                }
                if self.b.own(o, q) == 1 {
                    v += 3;
                }
            }
        }
        v
    }

    fn render(&self) -> String {
        let r = self.b.r;
        let s = self.b.side;
        let n = r.n;
        let mut out = format!(
            "{} — punctele sunt numerotate din perspectiva lui J{} ({}).\n",
            r.name,
            s.idx() + 1,
            if s == Player::One { 'X' } else { 'O' }
        );
        let cell = |p: usize| -> String {
            let c = self.b.own(s, p);
            match c {
                0 => " . ".into(),
                c if c > 0 => format!("{:>2}{}", c, if s == Player::One { 'X' } else { 'O' }),
                c => format!("{:>2}{}", -c, if s == Player::One { 'O' } else { 'X' }),
            }
        };
        let row = n / if n == 36 { 3 } else { 2 };
        let mut p0 = 1;
        while p0 <= n {
            out.push_str("  ");
            for p in p0..p0 + row {
                out.push_str(&format!("{p:>3} "));
            }
            out.push_str("\n  ");
            for p in p0..p0 + row {
                out.push_str(&format!("{} ", cell(p)));
            }
            out.push('\n');
            p0 += row;
        }
        out.push_str(&format!(
            "Bară: X={} O={}   Scoase: X={} O={}\n",
            self.b.bar[0], self.b.bar[1], self.b.off[0], self.b.off[1]
        ));
        let d: Vec<String> = self.b.dice[..self.b.nd as usize]
            .iter()
            .map(|d| d.to_string())
            .collect();
        if !self.log.is_empty() {
            out.push_str(&format!("{}\n", self.log));
        }
        match self.outcome() {
            Some(Outcome::Win(p)) => out.push_str(&format!("Jucătorul {} câștigă.\n", p.idx() + 1)),
            Some(Outcome::Draw) => out.push_str("Remiză.\n"),
            None => out.push_str(&format!(
                "La mutare: Jucătorul {}. Zaruri rămase: {}\n",
                s.idx() + 1,
                d.join(" ")
            )),
        }
        out
    }

    fn format_move(&self, m: &Mv) -> String {
        self.text(m)
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn play_random(r: Rules, seed: u64) {
        let mut g = DiceRace::with_seed(r, seed);
        let mut rng = Rng::new(seed + 99);
        let mut n = 0;
        while g.outcome().is_none() {
            let m = g.legal_moves();
            assert!(!m.is_empty());
            g.apply(&m[rng.below(m.len())]);
            n += 1;
            assert!(n < 20_000, "joc prea lung");
        }
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=5u64 {
            play_random(LONG_NARD, seed);
            play_random(DUODECIM, seed);
        }
    }

    #[test]
    fn long_nard_head_rule_and_no_hits() {
        let g = DiceRace::with_seed(LONG_NARD, 5);
        // la start toate piesele sunt în cap: o singură piesă pe tură (în afară de 3-3/4-4)
        let mut c = g.clone();
        let mut taken = 0;
        let first_turn_double = c.b.head_limit == 2;
        let mut guard = 0;
        while c.b.side == Player::One && c.b.turns[1] == 0 && guard < 10 && c.outcome().is_none() {
            let m = c.legal_moves()[0];
            if m.from == 1 {
                taken += 1;
            }
            c.apply(&m);
            guard += 1;
        }
        assert!(taken <= if first_turn_double { 2 } else { 1 });
        // piesele adversarului nu se lovesc: totalul rămâne 15
        let tot = |b: &Board, s: Player| (1..=24).map(|p| b.own(s, p).max(0) as u32).sum::<u32>();
        assert_eq!(tot(&c.b, Player::One) + c.b.off[0] as u32, 15);
    }

    #[test]
    fn duodecim_hits_send_to_bar() {
        let mut b = Board::new(DUODECIM);
        b.bar = [0, 0];
        b.pts[4] = -1; // blot J2 pe punctul 5
        b.pts[1] = 1; // J1 pe punctul 2
        b.side = Player::One;
        b.dice = [3, 0, 0, 0];
        b.nd = 1;
        let hit = b.step(Mv { from: 2, die: 3 });
        assert!(hit);
        assert_eq!(b.bar[1], 1);
        assert_eq!(b.pts[4], 1);
    }
}
