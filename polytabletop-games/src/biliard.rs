//! Biliard francez, „partida liberă” (Billiards / carambol, Franța) — 3 bile, fără buzunare.
//!
//! Masa are 284×142 cm (bile de 11 cm, mărite pentru joc). Bile: albă (a lui J1), galbenă (a lui J2) și roșie. Fiecare jucător lovește
//! numai bila lui. Un punct (carambol) = bila ta atinge ambele bile (galbena/albă a adversarului și roșia)
//! în aceeași lovitură; dacă ai marcat, mai joci. Primul la 10 puncte câștigă; după 120 de lovituri
//! câștigă cine are mai multe puncte.
//!
//! Mutarea e o lovitură: `u45f2` = unghi 45° (0° = spre dreapta, antiorar, pași de 2°) și forță 1–3.
//! Fizica e deterministă: ciocniri elastice între bile (restituire 0,95), band cu restituire 0,8,
//! frecare constantă. Nu există efect (spin).
//!
//! Clean-room: reguli publice ale carambolului liber; fizica e scrisă aici de la zero.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const TW: f64 = 284.0;
const TH: f64 = 142.0;
const R: f64 = 5.5;
const DT: f64 = 0.005;
const FRICTION: f64 = 25.0;
const POWERS: [f64; 3] = [90.0, 170.0, 280.0];
const TARGET: u8 = 10;
const SHOT_CAP: u16 = 120;
const ANGLE_STEP: u32 = 2;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Mv {
    pub angle: u16,
    pub power: u8,
}

#[derive(Clone)]
pub struct Biliard {
    /// 0 = albă (J1), 1 = galbenă (J2), 2 = roșie
    pos: [(f64, f64); 3],
    score: [u8; 2],
    side: Player,
    shots: u16,
    log: String,
}

impl Default for Biliard {
    fn default() -> Self {
        Self::new()
    }
}

impl Biliard {
    pub fn new() -> Self {
        Biliard {
            pos: [
                (TW * 0.25, TH * 0.4),
                (TW * 0.25, TH * 0.6),
                (TW * 0.75, TH * 0.5),
            ],
            score: [0; 2],
            side: Player::One,
            shots: 0,
            log: String::new(),
        }
    }

    pub fn score(&self) -> (u8, u8) {
        (self.score[0], self.score[1])
    }

    /// Simulează lovitura bilei `cue`; întoarce (poziții finale, a atins ambele bile?).
    fn simulate(&self, cue: usize, m: &Mv) -> ([(f64, f64); 3], bool) {
        let mut p = self.pos;
        let mut v = [(0.0f64, 0.0f64); 3];
        let a = (m.angle as f64).to_radians();
        let sp = POWERS[m.power as usize - 1];
        v[cue] = (sp * a.cos(), sp * a.sin());
        let mut touched = [false; 3];
        for _ in 0..4000 {
            let mut moving = false;
            for i in 0..3 {
                let s = (v[i].0 * v[i].0 + v[i].1 * v[i].1).sqrt();
                if s > 0.0 {
                    let ns = s - FRICTION * DT;
                    if ns <= 1.0 {
                        v[i] = (0.0, 0.0);
                    } else {
                        let k = ns / s;
                        v[i] = (v[i].0 * k, v[i].1 * k);
                        moving = true;
                    }
                }
                p[i].0 += v[i].0 * DT;
                p[i].1 += v[i].1 * DT;
                if p[i].0 < R {
                    p[i].0 = R + (R - p[i].0);
                    v[i].0 = -v[i].0 * 0.8;
                } else if p[i].0 > TW - R {
                    p[i].0 = (TW - R) - (p[i].0 - (TW - R));
                    v[i].0 = -v[i].0 * 0.8;
                }
                if p[i].1 < R {
                    p[i].1 = R + (R - p[i].1);
                    v[i].1 = -v[i].1 * 0.8;
                } else if p[i].1 > TH - R {
                    p[i].1 = (TH - R) - (p[i].1 - (TH - R));
                    v[i].1 = -v[i].1 * 0.8;
                }
            }
            for i in 0..3 {
                for j in (i + 1)..3 {
                    let (dx, dy) = (p[j].0 - p[i].0, p[j].1 - p[i].1);
                    let d2 = dx * dx + dy * dy;
                    if d2 < 4.0 * R * R && d2 > 1e-9 {
                        let d = d2.sqrt();
                        let (nx, ny) = (dx / d, dy / d);
                        // separare
                        let ov = 2.0 * R - d;
                        p[i].0 -= nx * ov / 2.0;
                        p[i].1 -= ny * ov / 2.0;
                        p[j].0 += nx * ov / 2.0;
                        p[j].1 += ny * ov / 2.0;
                        let rel = (v[i].0 - v[j].0) * nx + (v[i].1 - v[j].1) * ny;
                        if rel > 0.0 {
                            let imp = rel * (1.0 + 0.95) / 2.0;
                            v[i].0 -= imp * nx;
                            v[i].1 -= imp * ny;
                            v[j].0 += imp * nx;
                            v[j].1 += imp * ny;
                        }
                        if i == cue {
                            touched[j] = true;
                        } else if j == cue {
                            touched[i] = true;
                        }
                    }
                }
            }
            if !moving && v.iter().all(|x| x.0 == 0.0 && x.1 == 0.0) {
                break;
            }
        }
        let both = (0..3).filter(|&k| k != cue).all(|k| touched[k]);
        (p, both)
    }
}

impl Game for Biliard {
    type Move = Mv;
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<Mv> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        let mut v = Vec::new();
        for a in (0..360).step_by(ANGLE_STEP as usize) {
            for power in 1..=3u8 {
                v.push(Mv {
                    angle: a as u16,
                    power,
                });
            }
        }
        v
    }
    fn apply(&mut self, m: &Mv) {
        let cue = self.side.idx();
        let (p, both) = self.simulate(cue, m);
        self.pos = p;
        self.shots += 1;
        if both {
            self.score[cue] += 1;
            self.log = format!("Carambol! J{} marchează și mai joacă.", cue + 1);
        } else {
            self.log = format!("Fără carambol; urmează J{}.", 2 - cue);
            self.side = self.side.other();
        }
    }
    fn outcome(&self) -> Option<Outcome> {
        for p in 0..2 {
            if self.score[p] >= TARGET {
                return Some(Outcome::Win(if p == 0 { Player::One } else { Player::Two }));
            }
        }
        if self.shots >= SHOT_CAP {
            return Some(match self.score[0].cmp(&self.score[1]) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                _ => Outcome::Draw,
            });
        }
        None
    }
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut v = 1000 * (self.score[me] as i32 - self.score[1 - me] as i32);
        // după o lovitură fără punct, adversarul e la rând: preferăm bile apropiate de bandă (greu de jucat)? neutru
        let d = |a: (f64, f64), b: (f64, f64)| ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt();
        // pentru jucătorul la rând, bilele apropiate între ele sunt un avantaj
        let spread = d(self.pos[self.side.idx()], self.pos[2])
            + d(self.pos[self.side.idx()], self.pos[1 - self.side.idx()]);
        let bonus = (600.0 - spread).max(0.0) as i32 / 20;
        v += if self.side == p { bonus } else { -bonus };
        v
    }
    fn render(&self) -> String {
        let (cols, rows) = (71usize, 18usize);
        let mut grid = vec![vec!['.'; cols]; rows];
        for (i, ch) in ['A', 'G', 'R'].iter().enumerate() {
            let x = ((self.pos[i].0 / TW) * (cols as f64 - 1.0)).round() as usize;
            let y = rows - 1 - ((self.pos[i].1 / TH) * (rows as f64 - 1.0)).round() as usize;
            grid[y.min(rows - 1)][x.min(cols - 1)] = *ch;
        }
        let mut s = String::new();
        s.push('+');
        s.push_str(&"-".repeat(cols));
        s.push_str("+\n");
        for r in grid {
            s.push('|');
            s.extend(r);
            s.push_str("|\n");
        }
        s.push('+');
        s.push_str(&"-".repeat(cols));
        s.push_str("+\n");
        s.push_str(&format!(
            "A = bila albă (J1), G = galbenă (J2), R = roșie. Scor: J1 {} – {} J2 (la {TARGET}). Lovituri: {}. {}\nLa mutare: Jucătorul {} (bila {}).",
            self.score[0],
            self.score[1],
            self.shots,
            self.log,
            self.side.idx() + 1,
            if self.side == Player::One { "albă" } else { "galbenă" }
        ));
        s
    }
    fn format_move(&self, m: &Mv) -> String {
        format!("u{}f{}", m.angle, m.power)
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q4_util::*;
    use polytabletop_core::{GameObj, Player};

    #[test]
    fn some_shot_scores_a_carom_and_head_on_hit_touches_only_one() {
        let mut g = Biliard::new();
        g.pos = [(50.0, 71.0), (200.0, 30.0), (120.0, 71.0)];
        let (_, both) = g.simulate(0, &Mv { angle: 0, power: 1 });
        assert!(!both, "lovește doar roșia");
        let hits = GameObj::legal_moves(&g)
            .iter()
            .filter(|t| {
                let a: u16 = t[1..t.find('f').unwrap()].parse().unwrap();
                let p: u8 = t[t.find('f').unwrap() + 1..].parse().unwrap();
                g.simulate(0, &Mv { angle: a, power: p }).1
            })
            .count();
        assert!(
            hits > 0,
            "ar trebui să existe cel puțin un carambol din 540 de lovituri"
        );
    }

    #[test]
    fn physics_is_deterministic_and_balls_stay_on_table() {
        let g = Biliard::new();
        let m = Mv {
            angle: 33,
            power: 3,
        };
        let a = g.simulate(0, &m);
        let b = g.simulate(0, &m);
        assert_eq!(a.0, b.0);
        for p in a.0 {
            assert!(p.0 >= 0.0 && p.0 <= TW && p.1 >= 0.0 && p.1 <= TH);
        }
    }

    #[test]
    fn random_and_ai_games_finish() {
        random_finishes(|_| Box::new(Biliard::new()), 2, 300);
        ai_vs_random(|_| Box::new(Biliard::new()), 2, Player::One, 300);
        ai_vs_random(|_| Box::new(Biliard::new()), 3, Player::Two, 300);
        assert_eq!(GameObj::legal_moves(&Biliard::new()).len(), 540);
    }
}
