//! Snooker (India britanică, 1875), 2 jucători, o singură „frame”, cu fizică 2D simplă.
//!
//! * Masa 356,9 × 177,8 cm, 6 buzunare, bila albă, 15 roșii și 6 culori (galben 2, verde 3, maro 4,
//!   albastru 5, roz 6, negru 7) la locurile lor. Bilele se ciocnesc elastic, se ciocnesc de bandă
//!   (cu pierdere) și încetinesc prin frecare; nu există efect (spin).
//! * Ordinea: o roșie, apoi o culoare la alegere, din nou o roșie… Culorile băgate cât mai sunt
//!   roșii se pun la loc. După ultima roșie și culoarea ei se bag culorile în ordine 2…7 (rămân jos).
//! * O lovitură: `bila:buzunar:forta`, de exemplu `r4:ne:2` — țintești bila `r4` spre buzunarul `ne`
//!   (bila albă merge spre punctul „bilă-fantomă” care o împinge pe țintă spre buzunar); forța 1–3.
//!   Bile: `r1`…`r15`, `galben`, `verde`, `maro`, `albastru`, `roz`, `negru`. Buzunare: `sv`, `nv`, `se`,
//!   `ne` (colțuri), `ms`, `mn` (mijloc). La culoare, bila țintită este culoarea „anunțată”.
//! * Faulturi (adversarul primește max(4, valoarea bilelor implicate)): prima atingere greșită
//!   (la roșii: orice roșie; la culoare: bila anunțată), nicio atingere, bila albă băgată, bila
//!   greșită băgată. După un fault adversarul joacă; bila albă băgată e așezată la „D” (x=50, y=88,9).
//! * Simplificări: fără „snooker” de evitat/bilă liberă, fără „D” la alegere și fără regula respot a
//!   negrului la egalitate. Frame-ul se termină când masa e golită sau după 100 de lovituri în total;
//!   câștigă cine are mai multe puncte (egal = remiză).
//! * AI: simulează fiecare lovitură candidată (fizică deterministă, fără informație ascunsă) și o
//!   alege pe cea cu scorul rezultat cel mai bun.

use crate::q5_util::{by_score, verdict};
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const W: f64 = 356.9;
const H: f64 = 177.8;
const R: f64 = 2.625;
const SHOTS: u16 = 100;
const SPEED: [f64; 3] = [160.0, 320.0, 520.0];
const DECEL: f64 = 150.0;
const DT: f64 = 0.002;
const STEPS: usize = 6000;
const REST: f64 = 0.8;
const POCKETS: [(f64, f64, f64, &str); 6] = [
    (0.0, 0.0, 6.5, "sv"),
    (0.0, H, 6.5, "nv"),
    (W, 0.0, 6.5, "se"),
    (W, H, 6.5, "ne"),
    (W / 2.0, 0.0, 6.0, "ms"),
    (W / 2.0, H, 6.0, "mn"),
];
const COLOURS: [&str; 6] = ["galben", "verde", "maro", "albastru", "roz", "negru"];

fn spot(k: usize) -> (f64, f64) {
    match k {
        0 => (73.7, H / 2.0 + 29.2),
        1 => (73.7, H / 2.0 - 29.2),
        2 => (73.7, H / 2.0),
        3 => (W / 2.0, H / 2.0),
        4 => (W * 0.75, H / 2.0),
        _ => (W - 32.4, H / 2.0),
    }
}

#[derive(Clone, Copy, Debug)]
struct Ball {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    on: bool,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Move {
    /// 1..=15 roșii, 16..=21 culori
    pub ball: u8,
    pub pocket: u8,
    pub power: u8,
}

#[derive(Clone)]
pub struct Snooker {
    b: Vec<Ball>, // 0 albă, 1..=15 roșii, 16..=21 culori
    side: Player,
    score: [i32; 2],
    /// true = urmează o roșie; false = o culoare (după roșie) sau secvența finală
    on_red: bool,
    shots: u16,
    last: String,
}

impl Default for Snooker {
    fn default() -> Self {
        Self::new()
    }
}

fn ball_name(i: u8) -> String {
    if i <= 15 {
        format!("r{i}")
    } else {
        COLOURS[i as usize - 16].to_string()
    }
}

fn value(i: usize) -> i32 {
    if i <= 15 {
        1
    } else {
        i as i32 - 14
    }
}

impl Snooker {
    pub fn new() -> Self {
        let mut b = vec![
            Ball {
                x: 50.0,
                y: H / 2.0,
                vx: 0.0,
                vy: 0.0,
                on: true
            };
            22
        ];
        let apex = W * 0.75 + 2.0 * R + 0.01;
        let mut k = 1;
        for row in 0..5 {
            for j in 0..=row {
                b[k].x = apex + row as f64 * 3f64.sqrt() * R;
                b[k].y = H / 2.0 + (j as f64 - row as f64 / 2.0) * 2.0 * R * 1.002;
                k += 1;
            }
        }
        for c in 0..6 {
            let (x, y) = spot(c);
            b[16 + c].x = x;
            b[16 + c].y = y;
        }
        Snooker {
            b,
            side: Player::One,
            score: [0; 2],
            on_red: true,
            shots: 0,
            last: String::new(),
        }
    }

    pub fn score(&self) -> [i32; 2] {
        self.score
    }

    fn reds_left(&self) -> usize {
        (1..=15).filter(|&i| self.b[i].on).count()
    }

    /// Bila-colorată următoare în secvența finală (None cât timp mai sunt roșii).
    fn next_clear(&self) -> Option<usize> {
        if self.reds_left() > 0 {
            return None;
        }
        (16..22).find(|&i| self.b[i].on)
    }

    fn free_spot(&self, k: usize, skip: usize) -> (f64, f64) {
        let ok = |x: f64, y: f64| {
            (0..22).all(|i| {
                i == skip
                    || !self.b[i].on
                    || (self.b[i].x - x).hypot(self.b[i].y - y) >= 2.0 * R + 0.01
            })
        };
        let (x, y) = spot(k);
        if ok(x, y) {
            return (x, y);
        }
        for kk in (0..6).rev() {
            let (x, y) = spot(kk);
            if ok(x, y) {
                return (x, y);
            }
        }
        let (mut x, y) = spot(k);
        while !ok(x, y) && x < W - R {
            x += 2.0 * R;
        }
        (x, y)
    }

    /// Simulează o lovitură; întoarce (prima bilă atinsă, bile băgate).
    fn simulate(&mut self, m: &Move) -> (Option<usize>, Vec<usize>) {
        let t = m.ball as usize;
        let (px, py, _, _) = POCKETS[m.pocket as usize];
        let (tx, ty) = (self.b[t].x, self.b[t].y);
        let (dx, dy) = (px - tx, py - ty);
        let d = dx.hypot(dy).max(1e-9);
        let (gx, gy) = (tx - dx / d * 2.0 * R, ty - dy / d * 2.0 * R);
        let (cx, cy) = (self.b[0].x, self.b[0].y);
        let (ax, ay) = (gx - cx, gy - cy);
        let al = ax.hypot(ay).max(1e-9);
        let sp = SPEED[m.power as usize - 1];
        self.b[0].vx = ax / al * sp;
        self.b[0].vy = ay / al * sp;
        let mut first: Option<usize> = None;
        let mut potted: Vec<usize> = Vec::new();
        for _ in 0..STEPS {
            let mut moving = false;
            for i in 0..22 {
                if !self.b[i].on {
                    continue;
                }
                let s = self.b[i].vx.hypot(self.b[i].vy);
                if s > 0.0 {
                    moving = true;
                    let ns = (s - DECEL * DT).max(0.0);
                    let k = ns / s;
                    self.b[i].vx *= k;
                    self.b[i].vy *= k;
                    self.b[i].x += self.b[i].vx * DT;
                    self.b[i].y += self.b[i].vy * DT;
                }
            }
            if !moving {
                break;
            }
            // bile între ele
            for i in 0..22 {
                if !self.b[i].on {
                    continue;
                }
                for j in (i + 1)..22 {
                    if !self.b[j].on {
                        continue;
                    }
                    if self.b[i].vx == 0.0
                        && self.b[i].vy == 0.0
                        && self.b[j].vx == 0.0
                        && self.b[j].vy == 0.0
                    {
                        continue;
                    }
                    let (dx, dy) = (self.b[j].x - self.b[i].x, self.b[j].y - self.b[i].y);
                    let dist = dx.hypot(dy);
                    if dist < 2.0 * R && dist > 1e-9 {
                        let (nx, ny) = (dx / dist, dy / dist);
                        let rel =
                            (self.b[i].vx - self.b[j].vx) * nx + (self.b[i].vy - self.b[j].vy) * ny;
                        if i == 0 && first.is_none() {
                            first = Some(j);
                        }
                        if j == 0 && first.is_none() {
                            first = Some(i);
                        }
                        if rel > 0.0 {
                            self.b[i].vx -= rel * nx;
                            self.b[i].vy -= rel * ny;
                            self.b[j].vx += rel * nx;
                            self.b[j].vy += rel * ny;
                        }
                        let over = (2.0 * R - dist) / 2.0;
                        self.b[i].x -= nx * over;
                        self.b[i].y -= ny * over;
                        self.b[j].x += nx * over;
                        self.b[j].y += ny * over;
                    }
                }
            }
            // buzunare și benzi
            for i in 0..22 {
                if !self.b[i].on {
                    continue;
                }
                let mut dropped = false;
                for &(qx, qy, qr, _) in &POCKETS {
                    if (self.b[i].x - qx).hypot(self.b[i].y - qy) < qr {
                        dropped = true;
                        break;
                    }
                }
                if dropped {
                    self.b[i].on = false;
                    self.b[i].vx = 0.0;
                    self.b[i].vy = 0.0;
                    potted.push(i);
                    continue;
                }
                let bl = &mut self.b[i];
                if bl.x < R {
                    bl.x = R;
                    bl.vx = -bl.vx * REST;
                }
                if bl.x > W - R {
                    bl.x = W - R;
                    bl.vx = -bl.vx * REST;
                }
                if bl.y < R {
                    bl.y = R;
                    bl.vy = -bl.vy * REST;
                }
                if bl.y > H - R {
                    bl.y = H - R;
                    bl.vy = -bl.vy * REST;
                }
            }
        }
        for bl in self.b.iter_mut() {
            bl.vx = 0.0;
            bl.vy = 0.0;
        }
        (first, potted)
    }

    fn respot(&mut self, i: usize) {
        if i >= 16 {
            let (x, y) = self.free_spot(i - 16, i);
            self.b[i].x = x;
            self.b[i].y = y;
            self.b[i].on = true;
        }
    }
}

impl Game for Snooker {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        let targets: Vec<u8> = if let Some(c) = self.next_clear() {
            vec![c as u8]
        } else if self.on_red {
            (1..=15u8).filter(|&i| self.b[i as usize].on).collect()
        } else {
            (16..22u8).filter(|&i| self.b[i as usize].on).collect()
        };
        let mut v = Vec::new();
        for &ball in &targets {
            for pocket in 0..6u8 {
                for power in 1..=3u8 {
                    v.push(Move {
                        ball,
                        pocket,
                        power,
                    });
                }
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        let me = self.side.idx();
        let nominated = m.ball as usize;
        let clearing = self.next_clear();
        let (first, potted) = self.simulate(m);
        self.shots += 1;
        let colour_phase = clearing.is_some() || !self.on_red;
        let mut foul = false;
        let mut involved = 0;
        match first {
            None => {
                foul = true;
                involved = involved.max(if nominated >= 16 { value(nominated) } else { 1 });
            }
            Some(f) => {
                involved = involved.max(value(f));
                let good = if colour_phase {
                    f == nominated
                } else {
                    (1..=15).contains(&f)
                };
                if !good {
                    foul = true;
                }
            }
        }
        let cue_potted = potted.contains(&0);
        if cue_potted {
            foul = true;
        }
        let mut pts = 0;
        let mut potted_legal = false;
        for &p in potted.iter().filter(|&&p| p != 0) {
            involved = involved.max(value(p));
            if !colour_phase {
                if p <= 15 {
                    pts += 1;
                    potted_legal = true;
                } else {
                    foul = true;
                }
            } else if p == nominated {
                pts += value(p);
                potted_legal = true;
            } else {
                foul = true;
            }
        }
        if foul {
            let pen = involved.max(4);
            self.score[1 - me] += pen;
            // culorile băgate într-un fault se pun la loc; roșiile rămân jos
            for &p in potted.iter().filter(|&&p| p >= 16) {
                self.respot(p);
            }
            self.last = format!("Fault — J{} primește {pen} puncte.", 2 - me);
            self.on_red = self.reds_left() > 0;
            self.side = self.side.other();
        } else if potted_legal {
            self.score[me] += pts;
            self.last = format!("J{} bagă și primește {pts}.", me + 1);
            if clearing.is_none() {
                if colour_phase {
                    // culoarea se pune la loc cât timp jocul de roșii/culori continuă
                    self.respot(nominated);
                    self.on_red = self.reds_left() > 0;
                } else {
                    self.on_red = false;
                }
            }
        } else {
            self.last = format!("J{} ratează.", me + 1);
            self.on_red = self.reds_left() > 0;
            self.side = self.side.other();
        }
        if !self.b[0].on {
            self.b[0].on = true;
            self.b[0].x = 50.0;
            self.b[0].y = H / 2.0;
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        let cleared = (1..22).all(|i| !self.b[i].on);
        if cleared || self.shots >= SHOTS {
            return Some(by_score(self.score[0], self.score[1]));
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let mut v = (self.score[p.idx()] - self.score[p.other().idx()]) * 100;
        // cine e la masă după lovitură are mici avantaje; bila albă departe de roșii e riscantă
        if self.side == p {
            v += 20;
        }
        v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Snooker — J1 {} p., J2 {} p.; lovitura {}/{SHOTS}; roșii rămase: {}\n",
            self.score[0],
            self.score[1],
            (self.shots + 1).min(SHOTS),
            self.reds_left()
        );
        let (cols, rows) = (72usize, 18usize);
        let mut g = vec![vec!['.'; cols]; rows];
        let put = |g: &mut Vec<Vec<char>>, x: f64, y: f64, c: char| {
            let cx = ((x / W) * (cols as f64 - 1.0)).round() as usize;
            let cy = rows - 1 - ((y / H) * (rows as f64 - 1.0)).round() as usize;
            g[cy.min(rows - 1)][cx.min(cols - 1)] = c;
        };
        for i in (0..22).rev() {
            if self.b[i].on {
                let c = match i {
                    0 => 'O',
                    1..=15 => 'r',
                    16 => 'Y',
                    17 => 'G',
                    18 => 'M',
                    19 => 'B',
                    20 => 'P',
                    _ => 'N',
                };
                put(&mut g, self.b[i].x, self.b[i].y, c);
            }
        }
        for row in &g {
            s.push_str(&row.iter().collect::<String>());
            s.push('\n');
        }
        s.push_str("O alba, r roșii, Y galben, G verde, M maro, B albastru, P roz, N negru; buzunare: sv nv (stânga), se ne (dreapta), ms mn (mijloc)\n");
        let reds: Vec<String> = (1..=15)
            .filter(|&i| self.b[i].on)
            .map(|i| format!("r{i}({:.0},{:.0})", self.b[i].x, self.b[i].y))
            .collect();
        s.push_str(&format!("Roșii: {}\n", reds.join(" ")));
        let cols_on: Vec<String> = (16..22)
            .filter(|&i| self.b[i].on)
            .map(|i| format!("{}({:.0},{:.0})", COLOURS[i - 16], self.b[i].x, self.b[i].y))
            .collect();
        s.push_str(&format!(
            "Culori: {}\nAlba: ({:.0},{:.0})\n",
            cols_on.join(" "),
            self.b[0].x,
            self.b[0].y
        ));
        if !self.last.is_empty() {
            s.push_str(&format!("{}\n", self.last));
        }
        let v = verdict(self.outcome());
        if v.is_empty() {
            let what = if let Some(c) = self.next_clear() {
                format!("culoarea {}", COLOURS[c - 16])
            } else if self.on_red {
                "o roșie".to_string()
            } else {
                "o culoare (la alegere)".to_string()
            };
            s.push_str(&format!(
                "La mutare: Jucător {} — ținta: {what}\n",
                self.side.idx() + 1
            ));
        } else {
            s.push_str(&v);
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        format!(
            "{}:{}:{}",
            ball_name(m.ball),
            POCKETS[m.pocket as usize].3,
            m.power
        )
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q5_util::testkit::check_game;

    #[test]
    fn setup_is_consistent() {
        let g = Snooker::new();
        assert_eq!(g.b.len(), 22);
        // nicio suprapunere la început
        for i in 0..22 {
            for j in (i + 1)..22 {
                let d = (g.b[i].x - g.b[j].x).hypot(g.b[i].y - g.b[j].y);
                assert!(d >= 2.0 * R - 1e-6, "{i} {j} {d}");
            }
        }
        assert_eq!(g.legal_moves().len(), 15 * 6 * 3);
    }

    #[test]
    fn a_straight_shot_pots_a_red_and_scores() {
        let mut g = Snooker::new();
        // masă golită cu o roșie la 1 cm de buzunarul „ne”
        for i in 1..22 {
            g.b[i].on = false;
        }
        g.b[1].on = true;
        g.b[1].x = W - 30.0;
        g.b[1].y = H - 20.0;
        g.b[0].x = W - 90.0;
        g.b[0].y = H - 60.0;
        g.b[20].on = true; // roz rămâne pe masă
        let mv = Move {
            ball: 1,
            pocket: 3,
            power: 2,
        };
        g.apply(&mv);
        assert_eq!(g.score[0], 1, "{}", g.render());
        assert_eq!(g.side, Player::One, "cine bagă continuă");
        assert!(!g.on_red, "urmează o culoare");
    }

    #[test]
    fn hitting_nothing_is_a_foul_worth_four() {
        let mut g = Snooker::new();
        for i in 1..22 {
            g.b[i].on = false;
        }
        g.b[1].on = true;
        g.b[1].x = 200.0;
        g.b[1].y = 20.0;
        g.b[16].on = true;
        // țintă = roșia, dar o aruncăm foarte încet într-o direcție aiurea: forța 1, buzunar opus
        g.b[0].x = 100.0;
        g.b[0].y = 150.0;
        let before = g.score;
        g.apply(&Move {
            ball: 1,
            pocket: 1,
            power: 1,
        });
        assert!(
            g.score[1] >= before[1] + 4 || g.score[0] > before[0],
            "{}",
            g.render()
        );
    }

    #[test]
    fn random_and_ai_games() {
        check_game(|_| Snooker::new(), 2, 400);
    }
}
