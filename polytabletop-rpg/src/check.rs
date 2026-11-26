//! Interchangeable ability-check systems behind one trait.
//!
//! Every system turns a [`CheckRequest`] (rating, difficulty, edge, modifier)
//! into a [`CheckResult`] (degree of success, margin, successes, advantages,
//! complications, raw rolls). The meaning of `rating` and `difficulty`
//! depends on the system; see each struct.

use crate::dice::Expr;
use polytabletop_core::rng::Rng;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Degree {
    CriticalFailure,
    Failure,
    PartialSuccess,
    Success,
    CriticalSuccess,
}

impl Degree {
    pub fn is_success(self) -> bool {
        self >= Degree::PartialSuccess
    }
    pub fn label(self) -> &'static str {
        match self {
            Degree::CriticalFailure => "eșec critic",
            Degree::Failure => "eșec",
            Degree::PartialSuccess => "succes parțial",
            Degree::Success => "succes",
            Degree::CriticalSuccess => "succes critic",
        }
    }
}

impl fmt::Display for Degree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CheckRequest {
    /// Skill / attribute / pool size, depending on the system.
    pub rating: i32,
    /// Target number, number of successes needed, difficulty step, or dice penalty.
    pub difficulty: i32,
    /// Advantage (+) or disadvantage (-) steps: extra rolls or extra dice.
    pub edge: i32,
    /// Flat modifier.
    pub modifier: i32,
    /// Extra values some systems need (e.g. three attribute scores).
    pub extra: Vec<i32>,
}

impl CheckRequest {
    pub fn new(rating: i32, difficulty: i32) -> Self {
        CheckRequest {
            rating,
            difficulty,
            ..Default::default()
        }
    }
    pub fn edge(mut self, e: i32) -> Self {
        self.edge = e;
        self
    }
    pub fn modifier(mut self, m: i32) -> Self {
        self.modifier = m;
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckResult {
    pub degree: Degree,
    /// Positive = by how much it succeeded, negative = by how much it failed.
    pub margin: i32,
    pub successes: i32,
    pub advantages: i32,
    pub complications: i32,
    pub rolls: Vec<i32>,
    pub detail: String,
}

impl CheckResult {
    pub fn success(&self) -> bool {
        self.degree.is_success()
    }
}

pub trait CheckSystem {
    fn name(&self) -> String;
    fn resolve(&self, req: &CheckRequest, rng: &mut Rng) -> Result<CheckResult, String>;
}

fn base(degree: Degree, margin: i32, rolls: Vec<i32>, detail: String) -> CheckResult {
    CheckResult {
        degree,
        margin,
        successes: 0,
        advantages: 0,
        complications: 0,
        rolls,
        detail,
    }
}

// ---------------------------------------------------------------- roll over

/// Roll dice, add rating and modifier, succeed if the total reaches
/// `difficulty`. `edge` rolls extra times and keeps the best (or worst).
#[derive(Clone, Debug)]
pub struct RollOver {
    pub dice: Expr,
    /// Natural (pre-modifier) total at or above which the result is critical.
    pub crit_hi: Option<i64>,
    /// Natural total at or below which the result is a critical failure.
    pub crit_lo: Option<i64>,
    /// Natural criticals override the target number.
    pub crit_auto: bool,
    /// Margin at or beyond which the result is critical (both directions).
    pub crit_margin: Option<i32>,
}

impl RollOver {
    pub fn new(dice: &str) -> RollOver {
        RollOver {
            dice: Expr::parse(dice).expect("valid dice"),
            crit_hi: None,
            crit_lo: None,
            crit_auto: false,
            crit_margin: None,
        }
    }
}

impl CheckSystem for RollOver {
    fn name(&self) -> String {
        format!("{} + rating >= difficulty", self.dice)
    }
    fn resolve(&self, req: &CheckRequest, rng: &mut Rng) -> Result<CheckResult, String> {
        let n = 1 + req.edge.unsigned_abs() as usize;
        let mut nats: Vec<i64> = (0..n).map(|_| self.dice.roll(rng).total).collect();
        nats.sort();
        let nat = if req.edge >= 0 { nats[n - 1] } else { nats[0] };
        let total = nat + (req.rating + req.modifier) as i64;
        let margin = (total - req.difficulty as i64) as i32;
        let mut ok = margin >= 0;
        let mut crit = false;
        if let Some(h) = self.crit_hi {
            if nat >= h {
                crit = true;
                if self.crit_auto {
                    ok = true;
                }
            }
        }
        let mut fumble = false;
        if let Some(l) = self.crit_lo {
            if nat <= l {
                fumble = true;
                if self.crit_auto {
                    ok = false;
                }
            }
        }
        if let Some(cm) = self.crit_margin {
            if margin >= cm {
                crit = true;
            }
            if margin <= -cm {
                fumble = true;
            }
        }
        let degree = match (ok, crit, fumble) {
            (true, true, _) => Degree::CriticalSuccess,
            (true, false, _) => Degree::Success,
            (false, _, true) => Degree::CriticalFailure,
            _ => Degree::Failure,
        };
        let detail = format!(
            "{} -> {} + {} = {} vs {}",
            self.dice,
            nat,
            req.rating + req.modifier,
            total,
            req.difficulty
        );
        Ok(base(
            degree,
            margin,
            nats.iter().map(|&x| x as i32).collect(),
            detail,
        ))
    }
}

// --------------------------------------------------------------- roll under

/// Roll a die and succeed if the result is at or below the target.
/// `difficulty` is a step index into `step_divisors` (0 = normal, 1 = hard...);
/// the target is `(rating + modifier) / divisor`. On a d100, `edge` rolls extra
/// tens dice and keeps the best (lowest) or worst.
#[derive(Clone, Debug)]
pub struct RollUnder {
    pub sides: i32,
    pub step_divisors: Vec<i32>,
    /// Natural roll at or below this is a critical success (if it succeeds).
    pub crit_at: i32,
    /// Natural roll at or above this is a critical failure.
    pub fumble_at: i32,
    /// Size of one "degree" of margin (0 = no degrees).
    pub degree_size: i32,
}

impl CheckSystem for RollUnder {
    fn name(&self) -> String {
        format!("d{} roll-under", self.sides)
    }
    fn resolve(&self, req: &CheckRequest, rng: &mut Rng) -> Result<CheckResult, String> {
        let step = req.difficulty.max(0) as usize;
        let div = *self
            .step_divisors
            .get(step)
            .ok_or_else(|| format!("treaptă de dificultate necunoscută: {step}"))?;
        let target = ((req.rating + req.modifier) / div.max(1)).max(0);
        let roll = if self.sides == 100 {
            let units = rng.below(10) as i32;
            let tens: Vec<i32> = (0..1 + req.edge.unsigned_abs())
                .map(|_| rng.below(10) as i32)
                .collect();
            let t = if req.edge > 0 {
                *tens.iter().min().unwrap()
            } else {
                *tens.iter().max().unwrap()
            };
            let v = t * 10 + units;
            if v == 0 {
                100
            } else {
                v
            }
        } else {
            let n = 1 + req.edge.unsigned_abs() as usize;
            let v: Vec<i32> = (0..n)
                .map(|_| rng.below(self.sides as usize) as i32 + 1)
                .collect();
            if req.edge > 0 {
                *v.iter().min().unwrap()
            } else {
                *v.iter().max().unwrap()
            }
        };
        let margin = target - roll;
        let ok = roll <= target && roll < self.fumble_at;
        let degree = if roll >= self.fumble_at {
            Degree::CriticalFailure
        } else if ok && roll <= self.crit_at {
            Degree::CriticalSuccess
        } else if ok {
            Degree::Success
        } else {
            Degree::Failure
        };
        let mut r = base(
            degree,
            margin,
            vec![roll],
            format!("d{} -> {} vs {}", self.sides, roll, target),
        );
        if self.degree_size > 0 {
            r.successes = margin / self.degree_size;
        }
        Ok(r)
    }
}

// ------------------------------------------------------------- success pool

/// Roll `rating + modifier + edge` dice; each die at or above `threshold`
/// is a hit. Need `max(difficulty, 1)` net successes.
#[derive(Clone, Debug)]
pub struct SuccessPool {
    pub sides: u32,
    pub threshold: i32,
    /// Ones cancel hits; no hits plus a one is a botch.
    pub cancel_ones: bool,
    /// Half or more ones is a complication (and a critical failure with no hits).
    pub glitch: bool,
    /// Net successes beyond the requirement for a critical (0 = never).
    pub crit_extra: i32,
}

impl CheckSystem for SuccessPool {
    fn name(&self) -> String {
        format!("pool of d{} (>= {})", self.sides, self.threshold)
    }
    fn resolve(&self, req: &CheckRequest, rng: &mut Rng) -> Result<CheckResult, String> {
        let dice = req.rating + req.modifier + req.edge;
        let chance = dice <= 0;
        let n = if chance { 1 } else { dice as usize };
        let rolls: Vec<i32> = (0..n)
            .map(|_| rng.below(self.sides as usize) as i32 + 1)
            .collect();
        let hits = if chance {
            rolls.iter().filter(|&&v| v == self.sides as i32).count() as i32
        } else {
            rolls.iter().filter(|&&v| v >= self.threshold).count() as i32
        };
        let ones = rolls.iter().filter(|&&v| v == 1).count() as i32;
        let net = if self.cancel_ones { hits - ones } else { hits };
        let need = req.difficulty.max(1);
        let mut comp = 0;
        let mut degree = if net >= need {
            if self.crit_extra > 0 && net >= need + self.crit_extra {
                Degree::CriticalSuccess
            } else {
                Degree::Success
            }
        } else {
            Degree::Failure
        };
        if self.cancel_ones && hits == 0 && ones > 0 {
            degree = Degree::CriticalFailure;
        }
        if self.glitch && !chance && ones * 2 >= n as i32 {
            comp = 1;
            if hits == 0 {
                degree = Degree::CriticalFailure;
            }
        }
        let mut r = base(
            degree,
            net - need,
            rolls,
            format!("{hits} lovituri, {ones} de 1, net {net}, necesar {need}"),
        );
        r.successes = net;
        r.complications = comp;
        Ok(r)
    }
}

// ------------------------------------------------------------ highest of pool

/// Roll `rating + modifier + edge - difficulty` d6; the highest die decides:
/// `full_at`+ is a clean success, `partial_at`+ is a success with a
/// complication, below is a failure. Two or more full dice are critical.
/// With no dice, roll two and keep the lowest.
#[derive(Clone, Debug)]
pub struct HighestPool {
    pub sides: u32,
    pub full_at: i32,
    pub partial_at: i32,
}

impl CheckSystem for HighestPool {
    fn name(&self) -> String {
        format!("highest of d{} pool", self.sides)
    }
    fn resolve(&self, req: &CheckRequest, rng: &mut Rng) -> Result<CheckResult, String> {
        let dice = req.rating + req.modifier + req.edge - req.difficulty.max(0);
        let n = if dice <= 0 { 2 } else { dice as usize };
        let rolls: Vec<i32> = (0..n)
            .map(|_| rng.below(self.sides as usize) as i32 + 1)
            .collect();
        let pick = if dice <= 0 {
            *rolls.iter().min().unwrap()
        } else {
            *rolls.iter().max().unwrap()
        };
        let fulls = rolls.iter().filter(|&&v| v >= self.full_at).count();
        let (degree, comp) = if pick >= self.full_at {
            if dice > 0 && fulls >= 2 {
                (Degree::CriticalSuccess, 0)
            } else {
                (Degree::Success, 0)
            }
        } else if pick >= self.partial_at {
            (Degree::PartialSuccess, 1)
        } else {
            (Degree::Failure, 0)
        };
        let mut r = base(
            degree,
            pick - self.partial_at,
            rolls,
            format!("cel mai bun zar: {pick}"),
        );
        r.complications = comp;
        Ok(r)
    }
}

// ----------------------------------------------------------- narrative pool

/// Narrative dice pool. `rating` positive dice, `difficulty` negative dice;
/// positive edge adds positive dice, negative edge adds negative dice.
/// Positive die faces: 1-2 blank, 3-4 success, 5 success+advantage, 6 two
/// advantages. Negative faces: 1-2 blank, 3-4 failure, 5 failure+threat,
/// 6 two threats. Success needs net successes > 0; threats are complications.
#[derive(Clone, Debug)]
pub struct NarrativePool {
    pub crit_net: i32,
}

impl CheckSystem for NarrativePool {
    fn name(&self) -> String {
        "narrative dice pool".into()
    }
    fn resolve(&self, req: &CheckRequest, rng: &mut Rng) -> Result<CheckResult, String> {
        let pos = (req.rating + req.modifier + req.edge.max(0)).max(0) as usize;
        let neg = (req.difficulty + (-req.edge).max(0)).max(0) as usize;
        let (mut s, mut a, mut f, mut t) = (0, 0, 0, 0);
        let mut rolls = Vec::new();
        for _ in 0..pos {
            let v = rng.below(6) as i32 + 1;
            rolls.push(v);
            match v {
                3 | 4 => s += 1,
                5 => {
                    s += 1;
                    a += 1
                }
                6 => a += 2,
                _ => {}
            }
        }
        for _ in 0..neg {
            let v = rng.below(6) as i32 + 1;
            rolls.push(-v);
            match v {
                3 | 4 => f += 1,
                5 => {
                    f += 1;
                    t += 1
                }
                6 => t += 2,
                _ => {}
            }
        }
        let net = s - f;
        let degree = if net > 0 {
            if net >= self.crit_net {
                Degree::CriticalSuccess
            } else if t > 0 {
                Degree::PartialSuccess
            } else {
                Degree::Success
            }
        } else if net <= -self.crit_net {
            Degree::CriticalFailure
        } else {
            Degree::Failure
        };
        let mut r = base(
            degree,
            net,
            rolls,
            format!("{s} succese, {f} eșecuri, {a} avantaje, {t} amenințări"),
        );
        r.successes = net;
        r.advantages = a;
        r.complications = t;
        Ok(r)
    }
}

// ---------------------------------------------------------------- triple d20

/// Roll three d20 against three attribute values (`extra[0..3]`). Each die
/// above its attribute costs the difference in skill points; the check
/// succeeds if `rating + modifier` points cover the total. `difficulty`
/// lowers every attribute. Two 1s are critical, two 20s a critical failure.
#[derive(Clone, Debug)]
pub struct TripleUnder;

impl CheckSystem for TripleUnder {
    fn name(&self) -> String {
        "3d20 against three attributes".into()
    }
    fn resolve(&self, req: &CheckRequest, rng: &mut Rng) -> Result<CheckResult, String> {
        if req.extra.len() != 3 {
            return Err("sunt necesare exact trei valori de atribut (extra)".into());
        }
        let rolls: Vec<i32> = (0..3).map(|_| rng.below(20) as i32 + 1).collect();
        let pts = (req.rating + req.modifier).max(0);
        let cost: i32 = rolls
            .iter()
            .zip(&req.extra)
            .map(|(&r, &a)| (r - (a - req.difficulty)).max(0))
            .sum();
        let ones = rolls.iter().filter(|&&r| r == 1).count();
        let twenties = rolls.iter().filter(|&&r| r == 20).count();
        let left = pts - cost;
        let degree = if ones >= 2 {
            Degree::CriticalSuccess
        } else if twenties >= 2 {
            Degree::CriticalFailure
        } else if left >= 0 {
            Degree::Success
        } else {
            Degree::Failure
        };
        let mut r = base(degree, left, rolls, format!("cost {cost} din {pts} puncte"));
        r.successes = left.max(0);
        Ok(r)
    }
}

// ------------------------------------------------------------------ presets

pub struct Preset {
    pub id: &'static str,
    /// Catalog keys of the play styles this preset reproduces.
    pub styles: &'static [&'static str],
    pub summary: &'static str,
    pub default_difficulty: i32,
    pub mod_style: crate::sheet::ModStyle,
    pub build: fn() -> Box<dyn CheckSystem>,
}

use crate::sheet::ModStyle;

fn b<T: CheckSystem + 'static>(t: T) -> Box<dyn CheckSystem> {
    Box::new(t)
}

fn d20() -> RollOver {
    RollOver {
        crit_hi: Some(20),
        crit_lo: Some(1),
        crit_auto: true,
        ..RollOver::new("1d20")
    }
}

pub fn presets() -> Vec<Preset> {
    vec![
        Preset {
            id: "d20-dc",
            styles: &["dungeons & dragons", "pathfinder", "starfinder", "tormenta"],
            summary: "d20 + modificator vs prag; 20/1 naturale critice; avantaj/dezavantaj prin edge",
            default_difficulty: 15,
            mod_style: ModStyle::Step { base: 10, step: 2 },
            build: || b(d20()),
        },
        Preset {
            id: "d20-dr",
            styles: &["mörk borg"],
            summary: "d20 + modificator vs dificultate fixă (12); 20/1 naturale critice",
            default_difficulty: 12,
            mod_style: ModStyle::Step { base: 10, step: 2 },
            build: || b(d20()),
        },
        Preset {
            id: "2d6-target",
            styles: &["sword world rpg"],
            summary: "2d6 + rating vs țintă; 12 natural critic, 2 natural eșec automat",
            default_difficulty: 9,
            mod_style: ModStyle::Div(6),
            build: || {
                b(RollOver {
                    crit_hi: Some(12),
                    crit_lo: Some(2),
                    crit_auto: true,
                    ..RollOver::new("2d6")
                })
            },
        },
        Preset {
            id: "d10-explode-dc",
            styles: &["the witcher trpg", "cyberpunk red"],
            summary: "1d10 explozibil + atribut + abilitate vs prag",
            default_difficulty: 15,
            mod_style: ModStyle::Raw,
            build: || b(RollOver::new("1d10!")),
        },
        Preset {
            id: "d100-under",
            styles: &["call of cthulhu", "insane", "warhammer fantasy roleplay"],
            summary: "d100 roll-under; trepte normal/greu/extrem; grade de succes = zeci de margine; zar bonus/penalizare prin edge",
            default_difficulty: 0,
            mod_style: ModStyle::Raw,
            build: || {
                b(RollUnder {
                    sides: 100,
                    step_divisors: vec![1, 2, 5],
                    crit_at: 1,
                    fumble_at: 96,
                    degree_size: 10,
                })
            },
        },
        Preset {
            id: "d20-under",
            styles: &["drakar och demoner"],
            summary: "d20 roll-under; 1 critic, 20 ratare gravă",
            default_difficulty: 0,
            mod_style: ModStyle::Raw,
            build: || {
                b(RollUnder {
                    sides: 20,
                    step_divisors: vec![1, 2],
                    crit_at: 1,
                    fumble_at: 20,
                    degree_size: 0,
                })
            },
        },
        Preset {
            id: "d6-pool-hits",
            styles: &["shadowrun"],
            summary: "grămadă de d6, 5-6 sunt lovituri, prag de lovituri, jumătate de 1 = complicație",
            default_difficulty: 2,
            mod_style: ModStyle::Raw,
            build: || {
                b(SuccessPool {
                    sides: 6,
                    threshold: 5,
                    cancel_ones: false,
                    glitch: true,
                    crit_extra: 0,
                })
            },
        },
        Preset {
            id: "d10-pool-cancel",
            styles: &["vampire: the masquerade"],
            summary: "grămadă de d10, 6+ sunt succese, 1 anulează succese, eșec dezastruos la 0 succese și un 1",
            default_difficulty: 1,
            mod_style: ModStyle::Raw,
            build: || {
                b(SuccessPool {
                    sides: 10,
                    threshold: 6,
                    cancel_ones: true,
                    glitch: false,
                    crit_extra: 3,
                })
            },
        },
        Preset {
            id: "d6-highest",
            styles: &["blades in the dark"],
            summary: "grămadă de d6, cel mai bun zar: 6 curat, 4-5 cu complicație, 1-3 eșec; zero zaruri = cel mai mic din două",
            default_difficulty: 0,
            mod_style: ModStyle::Raw,
            build: || {
                b(HighestPool {
                    sides: 6,
                    full_at: 6,
                    partial_at: 4,
                })
            },
        },
        Preset {
            id: "triple-d20",
            styles: &["das schwarze auge (the dark eye)"],
            summary: "3d20 față de trei atribute, diferențele se plătesc din puncte de abilitate",
            default_difficulty: 0,
            mod_style: ModStyle::Raw,
            build: || b(TripleUnder),
        },
        Preset {
            id: "narrative-pool",
            styles: &[],
            summary: "zaruri narative pozitive/negative cu avantaje și amenințări",
            default_difficulty: 2,
            mod_style: ModStyle::Raw,
            build: || b(NarrativePool { crit_net: 3 }),
        },
    ]
}

pub fn preset(id: &str) -> Option<Preset> {
    presets().into_iter().find(|p| p.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(p: &str, req: &CheckRequest, seed: u64) -> CheckResult {
        let sys = (preset(p).unwrap().build)();
        sys.resolve(req, &mut Rng::new(seed)).unwrap()
    }

    #[test]
    fn rollover_consistency() {
        let sys = d20();
        let mut rng = Rng::new(1);
        for _ in 0..1000 {
            let r = sys.resolve(&CheckRequest::new(3, 15), &mut rng).unwrap();
            let nat = r.rolls[0];
            assert!((1..=20).contains(&nat));
            assert_eq!(r.margin, nat + 3 - 15);
            if nat == 20 {
                assert_eq!(r.degree, Degree::CriticalSuccess);
            } else if nat == 1 {
                assert_eq!(r.degree, Degree::CriticalFailure);
            } else {
                assert_eq!(r.success(), r.margin >= 0);
            }
        }
    }

    #[test]
    fn rollover_edge_improves_odds() {
        let sys = d20();
        let count = |edge| {
            let mut rng = Rng::new(42);
            (0..4000)
                .filter(|_| {
                    sys.resolve(&CheckRequest::new(0, 11).edge(edge), &mut rng)
                        .unwrap()
                        .success()
                })
                .count()
        };
        let (dis, norm, adv) = (count(-1), count(0), count(1));
        assert!(dis + 300 < norm && norm + 300 < adv, "{dis} {norm} {adv}");
    }

    #[test]
    fn two_d6_extremes() {
        let sys = (preset("2d6-target").unwrap().build)();
        let mut rng = Rng::new(5);
        for _ in 0..1000 {
            let r = sys.resolve(&CheckRequest::new(100, 2), &mut rng).unwrap();
            // rating huge, but natural 2 fails automatically
            assert_eq!(r.success(), r.rolls[0] != 2);
        }
    }

    #[test]
    fn exploding_check_runs() {
        let r = run("d10-explode-dc", &CheckRequest::new(8, 15), 3);
        assert!(r.detail.contains("vs 15"));
    }

    #[test]
    fn percentile_steps_and_degrees() {
        let sys = (preset("d100-under").unwrap().build)();
        let mut rng = Rng::new(8);
        let (mut normal, mut hard, mut extreme) = (0, 0, 0);
        for _ in 0..6000 {
            for (step, c) in [(0, &mut normal), (1, &mut hard), (2, &mut extreme)] {
                let r = sys.resolve(&CheckRequest::new(60, step), &mut rng).unwrap();
                let roll = r.rolls[0];
                assert!((1..=100).contains(&roll));
                if roll >= 96 {
                    assert_eq!(r.degree, Degree::CriticalFailure);
                }
                if r.success() {
                    *c += 1;
                    assert!(roll <= [60, 30, 12][step as usize]);
                }
            }
        }
        assert!(normal > hard && hard > extreme, "{normal} {hard} {extreme}");
        assert!(sys.resolve(&CheckRequest::new(60, 5), &mut rng).is_err());
    }

    #[test]
    fn percentile_bonus_dice() {
        let sys = (preset("d100-under").unwrap().build)();
        let avg = |edge| {
            let mut rng = Rng::new(77);
            let s: i32 = (0..4000)
                .map(|_| {
                    sys.resolve(&CheckRequest::new(50, 0).edge(edge), &mut rng)
                        .unwrap()
                        .rolls[0]
                })
                .sum();
            s as f64 / 4000.0
        };
        assert!(avg(1) < avg(0) - 5.0);
        assert!(avg(-1) > avg(0) + 5.0);
        assert!(avg(2) < avg(1));
    }

    #[test]
    fn d20_under() {
        let sys = (preset("d20-under").unwrap().build)();
        let mut rng = Rng::new(9);
        for _ in 0..500 {
            let r = sys.resolve(&CheckRequest::new(12, 0), &mut rng).unwrap();
            let roll = r.rolls[0];
            if roll == 20 {
                assert_eq!(r.degree, Degree::CriticalFailure);
            } else if roll == 1 {
                assert_eq!(r.degree, Degree::CriticalSuccess);
            } else {
                assert_eq!(r.success(), roll <= 12);
            }
        }
    }

    #[test]
    fn success_pool_accounting() {
        let sys = (preset("d6-pool-hits").unwrap().build)();
        let mut rng = Rng::new(10);
        for _ in 0..800 {
            let r = sys.resolve(&CheckRequest::new(8, 2), &mut rng).unwrap();
            assert_eq!(r.rolls.len(), 8);
            let hits = r.rolls.iter().filter(|&&v| v >= 5).count() as i32;
            assert_eq!(r.successes, hits);
            assert_eq!(
                r.success(),
                hits >= 2 && r.degree != Degree::CriticalFailure
            );
            let ones = r.rolls.iter().filter(|&&v| v == 1).count();
            assert_eq!(r.complications == 1, ones * 2 >= 8);
        }
        // chance die with an empty pool
        let r = run("d6-pool-hits", &CheckRequest::new(0, 1), 1);
        assert_eq!(r.rolls.len(), 1);
    }

    #[test]
    fn cancel_ones_pool() {
        let sys = (preset("d10-pool-cancel").unwrap().build)();
        let mut rng = Rng::new(12);
        let mut botches = 0;
        for _ in 0..1000 {
            let r = sys.resolve(&CheckRequest::new(5, 2), &mut rng).unwrap();
            let hits = r.rolls.iter().filter(|&&v| v >= 6).count() as i32;
            let ones = r.rolls.iter().filter(|&&v| v == 1).count() as i32;
            assert_eq!(r.successes, hits - ones);
            if hits == 0 && ones > 0 {
                assert_eq!(r.degree, Degree::CriticalFailure);
                botches += 1;
            }
        }
        assert!(botches > 0);
    }

    #[test]
    fn highest_pool_bands() {
        let sys = (preset("d6-highest").unwrap().build)();
        let mut rng = Rng::new(13);
        let mut seen = [false; 4];
        for _ in 0..2000 {
            let r = sys.resolve(&CheckRequest::new(2, 0), &mut rng).unwrap();
            let best = *r.rolls.iter().max().unwrap();
            match best {
                6 => {
                    let sixes = r.rolls.iter().filter(|&&v| v == 6).count();
                    assert_eq!(
                        r.degree,
                        if sixes >= 2 {
                            Degree::CriticalSuccess
                        } else {
                            Degree::Success
                        }
                    );
                    seen[0] = true;
                    seen[3] |= sixes >= 2;
                }
                4 | 5 => {
                    assert_eq!(r.degree, Degree::PartialSuccess);
                    assert_eq!(r.complications, 1);
                    seen[1] = true;
                }
                _ => {
                    assert_eq!(r.degree, Degree::Failure);
                    seen[2] = true;
                }
            }
        }
        assert!(seen.iter().all(|&x| x));
        // zero dice: two rolled, lowest taken
        let r = sys.resolve(&CheckRequest::new(0, 0), &mut rng).unwrap();
        assert_eq!(r.rolls.len(), 2);
        // penalty reduces dice
        let r = sys.resolve(&CheckRequest::new(3, 1), &mut rng).unwrap();
        assert_eq!(r.rolls.len(), 2);
    }

    #[test]
    fn narrative_pool_totals() {
        let sys = NarrativePool { crit_net: 3 };
        let mut rng = Rng::new(14);
        let mut partial = 0;
        for _ in 0..1500 {
            let r = sys.resolve(&CheckRequest::new(4, 2), &mut rng).unwrap();
            assert_eq!(r.rolls.len(), 6);
            let pos: Vec<i32> = r.rolls.iter().copied().filter(|&v| v > 0).collect();
            let neg: Vec<i32> = r
                .rolls
                .iter()
                .copied()
                .filter(|&v| v < 0)
                .map(|v| -v)
                .collect();
            let s = pos.iter().filter(|&&v| (3..=5).contains(&v)).count() as i32;
            let f = neg.iter().filter(|&&v| (3..=5).contains(&v)).count() as i32;
            assert_eq!(r.successes, s - f);
            assert_eq!(r.success(), s - f > 0);
            if r.degree == Degree::PartialSuccess {
                partial += 1;
                assert!(r.complications > 0);
            }
        }
        assert!(partial > 0);
        // edge adds dice on the matching side
        let r = sys
            .resolve(&CheckRequest::new(2, 1).edge(2), &mut rng)
            .unwrap();
        assert_eq!(r.rolls.iter().filter(|&&v| v > 0).count(), 4);
        let r = sys
            .resolve(&CheckRequest::new(2, 1).edge(-2), &mut rng)
            .unwrap();
        assert_eq!(r.rolls.iter().filter(|&&v| v < 0).count(), 3);
    }

    #[test]
    fn triple_under_rules() {
        let sys = TripleUnder;
        let mut req = CheckRequest::new(5, 0);
        assert!(sys.resolve(&req, &mut Rng::new(1)).is_err());
        req.extra = vec![12, 13, 14];
        let mut rng = Rng::new(15);
        let mut ok = 0;
        for _ in 0..2000 {
            let r = sys.resolve(&req, &mut rng).unwrap();
            let cost: i32 = r
                .rolls
                .iter()
                .zip(&req.extra)
                .map(|(&x, &a)| (x - a).max(0))
                .sum();
            let ones = r.rolls.iter().filter(|&&x| x == 1).count();
            let tw = r.rolls.iter().filter(|&&x| x == 20).count();
            if ones < 2 && tw < 2 {
                assert_eq!(r.success(), cost <= 5);
                assert_eq!(r.margin, 5 - cost);
            }
            if r.success() {
                ok += 1;
            }
        }
        assert!(ok > 500);
        // higher difficulty -> fewer successes
        let mut hard = req.clone();
        hard.difficulty = 6;
        let mut rng = Rng::new(15);
        let n = (0..2000)
            .filter(|_| sys.resolve(&hard, &mut rng).unwrap().success())
            .count();
        assert!(n < ok);
    }

    #[test]
    fn presets_cover_all_styles() {
        let want = [
            "dungeons & dragons",
            "call of cthulhu",
            "pathfinder",
            "vampire: the masquerade",
            "das schwarze auge (the dark eye)",
            "shadowrun",
            "warhammer fantasy roleplay",
            "sword world rpg",
            "blades in the dark",
            "mörk borg",
            "cyberpunk red",
            "tormenta",
            "starfinder",
            "drakar och demoner",
            "the witcher trpg",
            "insane",
        ];
        let ps = presets();
        for w in want {
            assert!(ps.iter().any(|p| p.styles.contains(&w)), "{w}");
        }
        // every preset builds and resolves
        let mut rng = Rng::new(1);
        for p in &ps {
            let mut req = CheckRequest::new(5, p.default_difficulty);
            req.extra = vec![10, 10, 10];
            assert!((p.build)().resolve(&req, &mut rng).is_ok(), "{}", p.id);
        }
        let mut ids: Vec<_> = ps.iter().map(|p| p.id).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), ps.len());
        assert!(preset("nope").is_none());
    }

    #[test]
    fn deterministic_with_seed() {
        let a = run("d20-dc", &CheckRequest::new(2, 12), 99);
        let b = run("d20-dc", &CheckRequest::new(2, 12), 99);
        assert_eq!(a, b);
    }
}
