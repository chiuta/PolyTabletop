//! Crokinole — joc de discuri pe tablă circulară, 2 jucători, modelat ca simulare fizică deterministă.
//!
//! Crokinole e un joc de îndemânare; aici lovitura se alege discret (poziția pe linia de tragere, unghiul
//! și puterea), iar execuția primește un **zgomot** dependent de putere, extras din sămânța jocului. Fizica e
//! o simulare simplă în pași mici (frecare constantă, ciocniri elastice disc–disc, ciocniri cu cuiele,
//! marginea = șanț). Geometria e aproximativă (mm), nu cea oficială a unei table.
//!
//! * Tablă cu raza 330; în centru gaura de 20 de puncte; inele (după marginea exterioară a discului):
//!   15 (până la raza 95), 10 (170), 5 (245); opt cuie la raza 108. Un disc care se oprește în afara razei
//!   245 (sau cade în șanț) iese din joc; un disc oprit în gaură valorează 20 și iese de pe tablă.
//! * 12 discuri pe jucător, trase pe rând de pe linia proprie (J1 jos, J2 sus). **Regula „trebuie să lovești”**:
//!   dacă pe tablă e cel puțin un disc advers, discul tras trebuie să atingă direct un disc adversar; altfel
//!   discul tras iese din joc (discurile proprii mișcate rămân unde au ajuns).
//! * După cele 24 de lovituri se numără: câștigă runda cine are mai multe puncte și ia **diferența**.
//!   Se joacă până la 100 de puncte, cel mult 6 runde (apoi cine are mai mult; egal = remiză). Primul tras
//!   alternează.
//!
//! Mutări (două pași, ca la Barbut): `x-1 u6 p3` = poziția −1 (−2…2) pe linie, unghi +6° față de centru
//! (−15…15, din 3 în 3), putere 3 (1…6); apoi mutarea forțată `lasa` execută lovitura cu zgomotul ei.
//! AI-ul nu vede zgomotul: evaluează fiecare țintire pe mai multe simulări cu zgomot propriu (independent
//! de cel real); nivelul scalează numărul de simulări.
#![allow(
    clippy::needless_range_loop,
    clippy::collapsible_if,
    clippy::collapsible_match,
    clippy::manual_is_multiple_of,
    clippy::manual_range_patterns,
    clippy::type_complexity
)]

use polytabletop_core::ai::level;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const R_BOARD: f64 = 330.0;
const R_DISC: f64 = 16.0;
const R_PEG: f64 = 6.0;
const PEG_RING: f64 = 108.0;
const HOLE: f64 = 20.0;
const ZONE15: f64 = 95.0;
const ZONE10: f64 = 170.0;
const ZONE5: f64 = 245.0;
const BASE: f64 = 300.0;
const POS_STEP: f64 = 55.0;
const DT: f64 = 0.002;
const FRICTION: f64 = 700.0;
const SPEEDS: [f64; 6] = [443.0, 555.0, 648.0, 748.0, 853.0, 990.0];
const DISCS: u8 = 12;
const TARGET: i32 = 100;
const MAX_ROUNDS: u8 = 6;

#[derive(Clone, Copy, PartialEq, Debug)]
struct Disc {
    x: f64,
    y: f64,
    owner: u8,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Shot {
    pub pos: i8,
    pub angle: i8,
    pub power: u8,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Aim(Shot),
    Release,
}

#[derive(Clone)]
pub struct Crokinole {
    rng: Rng,
    discs: Vec<Disc>,
    holes: [u32; 2],
    left: [u8; 2],
    pending: Option<Shot>,
    side: Player,
    first: Player,
    round: u8,
    scores: [i32; 2],
    done: bool,
    log: String,
}

impl Default for Crokinole {
    fn default() -> Self {
        Self::new()
    }
}

struct Outcome1 {
    discs: Vec<Disc>,
    /// Discuri căzute în gaură în această lovitură, pe proprietar.
    holes: [u32; 2],
    fouled: bool,
    hit: bool,
}

fn ring_value(d: f64) -> i32 {
    let e = d + R_DISC;
    if e <= ZONE15 {
        15
    } else if e <= ZONE10 {
        10
    } else if e <= ZONE5 {
        5
    } else {
        0
    }
}

fn pegs() -> [(f64, f64); 8] {
    let mut p = [(0.0, 0.0); 8];
    for (k, slot) in p.iter_mut().enumerate() {
        let a = (k as f64 * 45.0 + 22.5).to_radians();
        *slot = (PEG_RING * a.cos(), PEG_RING * a.sin());
    }
    p
}

/// Aproximativ normal(0,1) din trei uniforme.
fn gauss(rng: &mut Rng) -> f64 {
    let u = |r: &mut Rng| (r.next_u64() >> 11) as f64 / (1u64 << 53) as f64;
    (u(rng) + u(rng) + u(rng) - 1.5) / 0.5
}

impl Crokinole {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Crokinole {
            rng: Rng::new(seed),
            discs: Vec::new(),
            holes: [0, 0],
            left: [DISCS, DISCS],
            pending: None,
            side: Player::One,
            first: Player::One,
            round: 1,
            scores: [0, 0],
            done: false,
            log: String::new(),
        }
    }

    pub fn scores(&self) -> (i32, i32) {
        (self.scores[0], self.scores[1])
    }

    pub fn round(&self) -> u8 {
        self.round
    }

    fn start_of(&self, s: Shot, p: Player) -> (f64, f64, f64, f64) {
        let sy = if p == Player::One { -1.0 } else { 1.0 };
        let x = s.pos as f64 * POS_STEP;
        let y = sy * BASE;
        let base = (-y).atan2(-x);
        let a = base + (s.angle as f64).to_radians();
        (x, y, a.cos(), a.sin())
    }

    /// Simulează lovitura `s` a lui `p` pe tabla curentă, cu zgomotul din `rng`.
    fn simulate(&self, s: Shot, p: Player, rng: &mut Rng) -> Outcome1 {
        let (sx, sy, dx, dy) = self.start_of(s, p);
        // zgomot: unghi (grade) și viteză
        let pw = s.power as f64;
        let ang_noise = (0.5 + 0.25 * pw).to_radians() * gauss(rng);
        let spd_noise = 1.0 + (0.02 + 0.012 * pw) * gauss(rng);
        let (ca, sa) = (ang_noise.cos(), ang_noise.sin());
        let (dx, dy) = (dx * ca - dy * sa, dx * sa + dy * ca);
        let v0 = SPEEDS[(s.power as usize - 1).min(5)] * spd_noise;
        let owner = p.idx() as u8;
        let had_opp = self.discs.iter().any(|d| d.owner != owner);
        // discuri: indexul 0 = discul tras
        let mut x: Vec<f64> = vec![sx];
        let mut y: Vec<f64> = vec![sy];
        let mut vx: Vec<f64> = vec![dx * v0];
        let mut vy: Vec<f64> = vec![dy * v0];
        let mut own: Vec<u8> = vec![owner];
        for d in &self.discs {
            x.push(d.x);
            y.push(d.y);
            vx.push(0.0);
            vy.push(0.0);
            own.push(d.owner);
        }
        let n = x.len();
        let mut alive = vec![true; n];
        let mut hit = false;
        let pg = pegs();
        for _ in 0..1600 {
            let mut moving = false;
            for i in 0..n {
                if !alive[i] {
                    continue;
                }
                let sp = (vx[i] * vx[i] + vy[i] * vy[i]).sqrt();
                if sp > 0.0 {
                    let ns = sp - FRICTION * DT;
                    if ns <= 1.0 {
                        vx[i] = 0.0;
                        vy[i] = 0.0;
                    } else {
                        let k = ns / sp;
                        vx[i] *= k;
                        vy[i] *= k;
                        x[i] += vx[i] * DT;
                        y[i] += vy[i] * DT;
                        moving = true;
                    }
                }
            }
            if !moving {
                break;
            }
            // disc–disc
            for i in 0..n {
                if !alive[i] {
                    continue;
                }
                for j in (i + 1)..n {
                    if !alive[j] {
                        continue;
                    }
                    let (mi, mj) = (vx[i] != 0.0 || vy[i] != 0.0, vx[j] != 0.0 || vy[j] != 0.0);
                    if !mi && !mj {
                        continue;
                    }
                    let (ddx, ddy) = (x[j] - x[i], y[j] - y[i]);
                    let dist2 = ddx * ddx + ddy * ddy;
                    let md = 2.0 * R_DISC;
                    if dist2 >= md * md || dist2 == 0.0 {
                        continue;
                    }
                    let dist = dist2.sqrt();
                    let (nx, ny) = (ddx / dist, ddy / dist);
                    let vn = (vx[i] - vx[j]) * nx + (vy[i] - vy[j]) * ny;
                    if vn > 0.0 {
                        let imp = 0.5 * (1.0 + 0.85) * vn;
                        vx[i] -= imp * nx;
                        vy[i] -= imp * ny;
                        vx[j] += imp * nx;
                        vy[j] += imp * ny;
                        if i == 0 && own[j] != owner {
                            hit = true;
                        }
                    }
                    let push = (md - dist) / 2.0;
                    x[i] -= push * nx;
                    y[i] -= push * ny;
                    x[j] += push * nx;
                    y[j] += push * ny;
                }
            }
            // disc–cui și margine
            for i in 0..n {
                if !alive[i] {
                    continue;
                }
                for &(px, py) in &pg {
                    let (ddx, ddy) = (x[i] - px, y[i] - py);
                    let dist2 = ddx * ddx + ddy * ddy;
                    let md = R_DISC + R_PEG;
                    if dist2 < md * md && dist2 > 0.0 {
                        let dist = dist2.sqrt();
                        let (nx, ny) = (ddx / dist, ddy / dist);
                        let vn = vx[i] * nx + vy[i] * ny;
                        if vn < 0.0 {
                            vx[i] -= 1.6 * vn * nx;
                            vy[i] -= 1.6 * vn * ny;
                        }
                        x[i] = px + nx * md;
                        y[i] = py + ny * md;
                    }
                }
                if (x[i] * x[i] + y[i] * y[i]).sqrt() + R_DISC > R_BOARD {
                    alive[i] = false;
                }
            }
        }
        let mut discs = Vec::new();
        let mut holes = [0u32; 2];
        let fouled = had_opp && !hit;
        for i in 0..n {
            if !alive[i] {
                continue;
            }
            let d = (x[i] * x[i] + y[i] * y[i]).sqrt();
            if i == 0 {
                if fouled {
                    continue;
                }
            }
            if d <= HOLE {
                // un disc căzut în gaură (și discul tras, și unul mai vechi împins) se numără pentru proprietar
                holes[own[i] as usize] += 1;
                continue;
            }
            if d > ZONE5 {
                continue;
            }
            discs.push(Disc {
                x: x[i],
                y: y[i],
                owner: own[i],
            });
        }
        Outcome1 {
            discs,
            holes,
            fouled,
            hit,
        }
    }

    /// Schimbă sămânța zgomotului viitor (teste: AI-ul nu trebuie să depindă de ea).
    pub fn reseed(&mut self, seed: u64) {
        self.rng = Rng::new(seed);
    }

    fn round_scores(&self) -> [i32; 2] {
        let mut sc = [0i32; 2];
        for d in &self.discs {
            sc[d.owner as usize] += ring_value((d.x * d.x + d.y * d.y).sqrt());
        }
        for p in 0..2 {
            sc[p] += 20 * self.holes[p] as i32;
        }
        sc
    }

    fn finish_round(&mut self) {
        let sc = self.round_scores();
        let (w, diff) = if sc[0] > sc[1] {
            (Some(0), sc[0] - sc[1])
        } else if sc[1] > sc[0] {
            (Some(1), sc[1] - sc[0])
        } else {
            (None, 0)
        };
        if let Some(w) = w {
            self.scores[w] += diff;
        }
        self.log = format!(
            "Runda {} numărată: J1 {} – J2 {}{}",
            self.round,
            sc[0],
            sc[1],
            match w {
                Some(w) => format!(" → J{} ia {}", w + 1, diff),
                None => " → egalitate".into(),
            }
        );
        if self.scores.iter().any(|&s| s >= TARGET) || self.round >= MAX_ROUNDS {
            self.done = true;
            return;
        }
        self.round += 1;
        self.discs.clear();
        self.holes = [0, 0];
        self.left = [DISCS, DISCS];
        self.first = self.first.other();
        self.side = self.first;
    }
}

impl Game for Crokinole {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.done {
            return Vec::new();
        }
        if self.pending.is_some() {
            return vec![Move::Release];
        }
        let mut v = Vec::new();
        // ordinea: poziții centrale și unghiuri mici primele
        let poss: Vec<i8> = vec![0, -1, 1, -2, 2];
        let angs: Vec<i8> = vec![0, -3, 3, -6, 6, -9, 9, -12, 12, -15, 15];
        for pos in poss {
            for &angle in &angs {
                for power in 1..=6u8 {
                    v.push(Move::Aim(Shot { pos, angle, power }));
                }
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        match *m {
            Move::Aim(s) => {
                self.log.clear();
                self.pending = Some(s);
            }
            Move::Release => {
                let s = self.pending.take().unwrap();
                let p = self.side;
                let mut rng = self.rng.clone();
                let res = self.simulate(s, p, &mut rng);
                self.rng = rng;
                // zgomotul real a fost extras din `self.rng`; actualizăm starea
                self.log = format!(
                    "J{} trage ({} {}° p{}): {}",
                    p.idx() + 1,
                    s.pos,
                    s.angle,
                    s.power,
                    if res.fouled {
                        "n-a atins un disc advers — discul iese din joc".to_string()
                    } else if res.holes[p.idx()] > 0 {
                        "în gaură: +20!".to_string()
                    } else if res.hit {
                        "a lovit un disc advers".to_string()
                    } else {
                        "fără contact".to_string()
                    }
                );
                self.discs = res.discs;
                for q in 0..2 {
                    self.holes[q] += res.holes[q];
                }
                self.left[p.idx()] -= 1;
                if self.left == [0, 0] {
                    self.finish_round();
                } else {
                    self.side = p.other();
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.done {
            return None;
        }
        Some(match self.scores[0].cmp(&self.scores[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    /// Valoarea țintirii în așteptare: media diferenței de puncte pe simulări cu zgomot propriu.
    fn evaluate(&self, p: Player) -> i32 {
        let q = p.other();
        let base = {
            let sc = self.round_scores();
            sc[p.idx()] - sc[q.idx()]
        };
        let Some(s) = self.pending else {
            return base * 10;
        };
        if self.side != p {
            return base * 10;
        }
        let k = (3 * level().factor_x10() / 10).clamp(1, 24) as usize;
        let mut seed: u64 = 0xC70C
            ^ ((s.pos as i64 as u64) << 8)
            ^ ((s.angle as i64 as u64) << 16)
            ^ ((s.power as u64) << 32);
        for d in &self.discs {
            seed = seed.rotate_left(9) ^ (d.x.to_bits() ^ d.y.to_bits().rotate_left(13));
        }
        let mut total = 0.0;
        for i in 0..k {
            let mut rng = Rng::new(seed.wrapping_add(i as u64 * 0x9E37_79B9));
            let res = self.simulate(s, p, &mut rng);
            let mut g = self.clone();
            g.discs = res.discs;
            for q in 0..2 {
                g.holes[q] += res.holes[q];
            }
            let sc = g.round_scores();
            let mut v = (sc[p.idx()] - sc[q.idx()]) as f64;
            // ușoară preferință pentru lovituri care lasă mai multe discuri proprii în joc
            let mine = g
                .discs
                .iter()
                .filter(|d| d.owner as usize == p.idx())
                .count() as f64;
            let theirs = g.discs.len() as f64 - mine;
            v += 0.4 * (mine - theirs);
            total += v;
        }
        (total / k as f64 * 10.0) as i32
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Crokinole — runda {}/{} | scor: J1 {}, J2 {} (până la {})\n",
            self.round, MAX_ROUNDS, self.scores[0], self.scores[1], TARGET
        );
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        if self.done {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        // hartă ASCII 33×17 (20 mm × 40 mm per celulă); J1 = X, J2 = O, cui = +, gaură = @
        let (w, h) = (33usize, 17usize);
        let mut grid = vec![vec!['.'; w]; h];
        let cell = |x: f64, y: f64| -> Option<(usize, usize)> {
            let c = ((x + R_BOARD) / 20.0).floor();
            let r = ((R_BOARD - y) / 40.0).floor();
            if c >= 0.0 && (c as usize) < w && r >= 0.0 && (r as usize) < h {
                Some((c as usize, r as usize))
            } else {
                None
            }
        };
        for r in 0..h {
            for c in 0..w {
                let (x, y) = (
                    c as f64 * 20.0 - R_BOARD + 10.0,
                    R_BOARD - r as f64 * 40.0 - 20.0,
                );
                if (x * x + y * y).sqrt() > R_BOARD {
                    grid[r][c] = ' ';
                }
            }
        }
        if let Some((c, r)) = cell(0.0, 0.0) {
            grid[r][c] = '@';
        }
        for (px, py) in pegs() {
            if let Some((c, r)) = cell(px, py) {
                grid[r][c] = '+';
            }
        }
        for d in &self.discs {
            if let Some((c, r)) = cell(d.x, d.y) {
                grid[r][c] = if d.owner == 0 { 'X' } else { 'O' };
            }
        }
        for row in &grid {
            s.push_str(&row.iter().collect::<String>());
            s.push('\n');
        }
        let sc = self.round_scores();
        s.push_str(&format!(
            "Pe tablă: J1 (X) {} discuri, J2 (O) {} discuri; în gaură: J1 {}, J2 {}; punctaj curent: J1 {} – J2 {}\n",
            self.discs.iter().filter(|d| d.owner == 0).count(),
            self.discs.iter().filter(|d| d.owner == 1).count(),
            self.holes[0],
            self.holes[1],
            sc[0],
            sc[1]
        ));
        s.push_str(&format!(
            "Discuri rămase: J1 {}, J2 {}\n",
            self.left[0], self.left[1]
        ));
        if let Some(sh) = self.pending {
            s.push_str(&format!(
                "Țintire: poziția {}, unghi {}°, putere {}. Scrie «lasa».\n",
                sh.pos, sh.angle, sh.power
            ));
        }
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Aim(s) => format!("x{} u{} p{}", s.pos, s.angle, s.power),
            Move::Release => "lasa".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shot(pos: i8, angle: i8, power: u8) -> Shot {
        Shot { pos, angle, power }
    }

    #[test]
    fn zero_noise_center_shot_reaches_center_region() {
        // fără zgomot: putere 3 din poziția 0 unghi 0 ajunge în jurul centrului
        let g = Crokinole::with_seed(1);
        // simulăm cu zgomot nul: folosim un rng și verificăm media pe multe încercări
        let mut inside = 0;
        for i in 0..30 {
            let mut rng = Rng::new(100 + i);
            let r = g.simulate(shot(0, 0, 3), Player::One, &mut rng);
            if r.holes[0] > 0
                || r.discs
                    .iter()
                    .any(|d| (d.x * d.x + d.y * d.y).sqrt() < 150.0)
            {
                inside += 1;
            }
        }
        assert!(inside >= 20, "{inside}");
    }

    #[test]
    fn weak_shot_stops_short_and_strong_shot_leaves_the_board() {
        let g = Crokinole::with_seed(1);
        let mut rng = Rng::new(5);
        let r = g.simulate(shot(0, 0, 1), Player::One, &mut rng);
        assert_eq!(r.discs.len(), 1);
        let d = &r.discs[0];
        let dist = (d.x * d.x + d.y * d.y).sqrt();
        assert!(dist > 110.0 && dist < 220.0, "{dist}");
        // puterea 6 trece de centru și iese în șanț / în afara zonei
        let mut gone = 0;
        for i in 0..10 {
            let mut rng = Rng::new(60 + i);
            let r = g.simulate(shot(0, 0, 6), Player::One, &mut rng);
            if r.discs.is_empty() {
                gone += 1;
            }
        }
        assert!(gone >= 5, "{gone}"); // cuiele mai deviază uneori lovitura
    }

    #[test]
    fn must_hit_rule_removes_missing_disc() {
        let mut g = Crokinole::with_seed(2);
        g.discs = vec![Disc {
            x: 200.0,
            y: 100.0,
            owner: 1,
        }];
        let mut rng = Rng::new(9);
        // țintește spre stânga, departe de disc
        let r = g.simulate(shot(-2, 15, 4), Player::One, &mut rng);
        assert!(r.fouled && !r.hit);
        // discul advers a rămas pe loc
        assert!(r.discs.iter().any(|d| d.owner == 1 && d.x == 200.0));
        // fără discuri adverse nu există obligația
        g.discs.clear();
        let mut rng = Rng::new(9);
        let r = g.simulate(shot(0, 0, 3), Player::One, &mut rng);
        assert!(!r.fouled);
    }

    #[test]
    fn direct_hit_moves_the_opponent_disc() {
        let mut g = Crokinole::with_seed(3);
        g.discs = vec![Disc {
            x: 0.0,
            y: -150.0,
            owner: 1,
        }];
        let mut hits = 0;
        for i in 0..20 {
            let mut rng = Rng::new(40 + i);
            let r = g.simulate(shot(0, 0, 3), Player::One, &mut rng);
            if r.hit {
                hits += 1;
                // ținta s-a deplasat
                assert!(!r
                    .discs
                    .iter()
                    .any(|d| d.owner == 1 && d.x == 0.0 && d.y == -150.0));
            }
        }
        assert!(hits >= 15, "{hits}");
    }

    #[test]
    fn scoring_rings_and_net_difference() {
        assert_eq!(ring_value(0.0 + 30.0), 15);
        assert_eq!(ring_value(120.0), 10);
        assert_eq!(ring_value(200.0), 5);
        assert_eq!(ring_value(240.0), 0);
        let mut g = Crokinole::with_seed(4);
        g.discs = vec![
            Disc {
                x: 50.0,
                y: 0.0,
                owner: 0,
            },
            Disc {
                x: 120.0,
                y: 0.0,
                owner: 0,
            },
            Disc {
                x: 0.0,
                y: 200.0,
                owner: 1,
            },
        ];
        g.holes = [0, 1];
        g.left = [1, 0];
        g.side = Player::One;
        // J1: 15 + 10 = 25; J2: 5 + 20 = 25 → egalitate
        let sc = g.round_scores();
        assert_eq!(sc, [25, 25]);
        g.pending = Some(shot(0, 0, 1));
        g.apply(&Move::Release);
        assert_eq!(g.round, 2);
    }

    #[test]
    fn two_step_move_and_determinism() {
        let mut a = Crokinole::with_seed(7);
        let mut b = Crokinole::with_seed(7);
        for g in [&mut a, &mut b] {
            let m = g.legal_moves()[3];
            g.apply(&m);
            assert_eq!(g.legal_moves(), vec![Move::Release]);
            g.apply(&Move::Release);
        }
        assert_eq!(a.render(), b.render());
        assert_eq!(a.discs, b.discs);
    }

    #[test]
    fn evaluate_does_not_depend_on_the_real_noise() {
        // aceeași țintire, două semințe de joc diferite: evaluarea e identică (zgomot propriu AI)
        let mut a = Crokinole::with_seed(11);
        let mut b = Crokinole::with_seed(12);
        let m = Move::Aim(shot(0, 3, 4));
        a.apply(&m);
        b.apply(&m);
        assert_eq!(a.evaluate(Player::One), b.evaluate(Player::One));
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=2u64 {
            let mut g = Crokinole::with_seed(seed);
            let mut rng = Rng::new(seed + 3);
            let mut n = 0;
            while g.outcome().is_none() && n < 2000 {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed} after {n}");
        }
    }
}
