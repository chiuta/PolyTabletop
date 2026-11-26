//! Dice-notation parser and roller.
//!
//! Grammar (spaces ignored, case-insensitive):
//!
//! ```text
//! expr  := ['+'|'-'] term (('+'|'-') term)*
//! term  := integer | dice
//! dice  := [count] 'd' sides mod*        sides := integer | '%' | 'F'
//! mod   := 'kh'[n] | 'kl'[n] | 'k'[n]    keep highest / lowest n
//!        | 'dh'[n] | 'dl'[n]             drop highest / lowest n
//!        | '!' [cmp] [n]                 explode (default: on the maximum face)
//!        | 's' [cmp] n                   count successes instead of summing
//! cmp   := '>=' | '<=' | '>' | '<' | '='   (a bare number means '>=')
//! ```
//!
//! `d%` is `d100`, `dF` is a three-valued die (-1, 0, +1).

use polytabletop_core::rng::Rng;
use std::fmt;

pub const MAX_DICE: u32 = 1000;
pub const MAX_SIDES: u32 = 100_000;
/// Maximum number of extra dice a single die may chain through explosions.
pub const MAX_EXPLOSIONS: u32 = 20;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmp {
    Ge,
    Le,
    Gt,
    Lt,
    Eq,
}

impl Cmp {
    pub fn test(self, v: i32, t: i32) -> bool {
        match self {
            Cmp::Ge => v >= t,
            Cmp::Le => v <= t,
            Cmp::Gt => v > t,
            Cmp::Lt => v < t,
            Cmp::Eq => v == t,
        }
    }
    fn sym(self) -> &'static str {
        match self {
            Cmp::Ge => ">=",
            Cmp::Le => "<=",
            Cmp::Gt => ">",
            Cmp::Lt => "<",
            Cmp::Eq => "=",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Keep {
    All,
    Highest(u32),
    Lowest(u32),
    DropHighest(u32),
    DropLowest(u32),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiceTerm {
    pub sign: i32,
    pub count: u32,
    pub sides: u32,
    pub fate: bool,
    pub keep: Keep,
    pub explode: Option<(Cmp, i32)>,
    pub success: Option<(Cmp, i32)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Term {
    Const(i32),
    Dice(DiceTerm),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Expr {
    pub terms: Vec<Term>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiceError(pub String);

impl fmt::Display for DiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "notație de zaruri invalidă: {}", self.0)
    }
}
impl std::error::Error for DiceError {}

fn err<T>(m: impl Into<String>) -> Result<T, DiceError> {
    Err(DiceError(m.into()))
}

struct P {
    c: Vec<char>,
    i: usize,
}

impl P {
    fn peek(&self) -> Option<char> {
        self.c.get(self.i).copied()
    }
    fn peek2(&self) -> Option<char> {
        self.c.get(self.i + 1).copied()
    }
    fn eat(&mut self, ch: char) -> bool {
        if self.peek() == Some(ch) {
            self.i += 1;
            true
        } else {
            false
        }
    }
    fn number(&mut self) -> Result<Option<u32>, DiceError> {
        let st = self.i;
        while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
            self.i += 1;
        }
        if st == self.i {
            return Ok(None);
        }
        let s: String = self.c[st..self.i].iter().collect();
        match s.parse::<u32>() {
            Ok(v) if v <= 1_000_000 => Ok(Some(v)),
            _ => err(format!("număr prea mare: {s}")),
        }
    }
    fn cmp(&mut self) -> Option<Cmp> {
        match (self.peek(), self.peek2()) {
            (Some('>'), Some('=')) => {
                self.i += 2;
                Some(Cmp::Ge)
            }
            (Some('<'), Some('=')) => {
                self.i += 2;
                Some(Cmp::Le)
            }
            (Some('>'), _) => {
                self.i += 1;
                Some(Cmp::Gt)
            }
            (Some('<'), _) => {
                self.i += 1;
                Some(Cmp::Lt)
            }
            (Some('='), _) => {
                self.i += 1;
                Some(Cmp::Eq)
            }
            _ => None,
        }
    }
}

impl Expr {
    pub fn parse(s: &str) -> Result<Expr, DiceError> {
        let c: Vec<char> = s
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| c.to_ascii_lowercase())
            .collect();
        if c.is_empty() {
            return err("expresie goală");
        }
        let mut p = P { c, i: 0 };
        let mut terms = Vec::new();
        let mut sign = 1;
        if p.eat('-') {
            sign = -1;
        } else {
            p.eat('+');
        }
        loop {
            terms.push(parse_term(&mut p, sign)?);
            match p.peek() {
                None => break,
                Some('+') => {
                    p.i += 1;
                    sign = 1;
                }
                Some('-') => {
                    p.i += 1;
                    sign = -1;
                }
                Some(c) => return err(format!("caracter neașteptat '{c}' la poziția {}", p.i)),
            }
        }
        Ok(Expr { terms })
    }

    pub fn roll(&self, rng: &mut Rng) -> Roll {
        let mut parts = Vec::new();
        let mut total = 0i64;
        for t in &self.terms {
            let part = match t {
                Term::Const(k) => PartRoll {
                    sign: 1,
                    dice: vec![],
                    subtotal: *k as i64,
                    constant: true,
                },
                Term::Dice(d) => roll_term(d, rng),
            };
            total += part.subtotal;
            parts.push(part);
        }
        Roll { total, parts }
    }

    /// Smallest possible total (ignoring explosions).
    pub fn min(&self) -> i64 {
        self.range().0
    }

    /// Largest possible total (ignoring explosions).
    pub fn max(&self) -> i64 {
        self.range().1
    }

    fn range(&self) -> (i64, i64) {
        let (mut lo, mut hi) = (0i64, 0i64);
        for t in &self.terms {
            match t {
                Term::Const(k) => {
                    lo += *k as i64;
                    hi += *k as i64;
                }
                Term::Dice(d) => {
                    let n = kept_count(d) as i64;
                    let (l, h) = if d.success.is_some() {
                        (0, n)
                    } else if d.fate {
                        (-n, n)
                    } else {
                        (n, n * d.sides as i64)
                    };
                    if d.sign < 0 {
                        lo -= h;
                        hi -= l;
                    } else {
                        lo += l;
                        hi += h;
                    }
                }
            }
        }
        (lo, hi)
    }

    /// Mean of the plain (non-exploding, keep-all) dice; `None` if the
    /// expression uses modifiers whose mean is not closed-form.
    pub fn mean(&self) -> Option<f64> {
        let mut m = 0.0;
        for t in &self.terms {
            match t {
                Term::Const(k) => m += *k as f64,
                Term::Dice(d) => {
                    if d.keep != Keep::All || d.explode.is_some() || d.success.is_some() {
                        return None;
                    }
                    let per = if d.fate {
                        0.0
                    } else {
                        (d.sides as f64 + 1.0) / 2.0
                    };
                    m += d.sign as f64 * d.count as f64 * per;
                }
            }
        }
        Some(m)
    }
}

fn kept_count(d: &DiceTerm) -> u32 {
    match d.keep {
        Keep::All => d.count,
        Keep::Highest(n) | Keep::Lowest(n) => n.min(d.count),
        Keep::DropHighest(n) | Keep::DropLowest(n) => d.count.saturating_sub(n),
    }
}

fn parse_term(p: &mut P, sign: i32) -> Result<Term, DiceError> {
    let first = p.number()?;
    if p.peek() != Some('d') {
        return match first {
            Some(v) => Ok(Term::Const(sign * v as i32)),
            None => err(format!("se aștepta un număr sau zar la poziția {}", p.i)),
        };
    }
    p.i += 1; // 'd'
    let count = first.unwrap_or(1);
    if count == 0 || count > MAX_DICE {
        return err(format!("număr de zaruri în afara 1..={MAX_DICE}"));
    }
    let (sides, fate) = if p.eat('%') {
        (100, false)
    } else if p.eat('f') {
        (3, true)
    } else {
        match p.number()? {
            Some(s) if (1..=MAX_SIDES).contains(&s) => (s, false),
            Some(_) => return err(format!("fețe în afara 1..={MAX_SIDES}")),
            None => return err("lipsesc fețele zarului"),
        }
    };
    let mut d = DiceTerm {
        sign,
        count,
        sides,
        fate,
        keep: Keep::All,
        explode: None,
        success: None,
    };
    loop {
        match p.peek() {
            Some('k') => {
                p.i += 1;
                let lo = if p.eat('l') {
                    true
                } else {
                    p.eat('h');
                    false
                };
                let n = p.number()?.unwrap_or(1);
                d.keep = if lo {
                    Keep::Lowest(n)
                } else {
                    Keep::Highest(n)
                };
            }
            Some('d') if matches!(p.peek2(), Some('h') | Some('l')) => {
                p.i += 1;
                let hi = p.peek() == Some('h');
                p.i += 1;
                let n = p.number()?.unwrap_or(1);
                d.keep = if hi {
                    Keep::DropHighest(n)
                } else {
                    Keep::DropLowest(n)
                };
            }
            Some('!') => {
                p.i += 1;
                let cmp = p.cmp();
                let n = p.number()?;
                d.explode = Some(match (cmp, n) {
                    (None, None) => (Cmp::Ge, if fate { 1 } else { sides as i32 }),
                    (c, Some(n)) => (c.unwrap_or(Cmp::Ge), n as i32),
                    (Some(_), None) => return err("după '!' cu comparator lipsește numărul"),
                });
            }
            Some('s') => {
                p.i += 1;
                let cmp = p.cmp().unwrap_or(Cmp::Ge);
                match p.number()? {
                    Some(n) => d.success = Some((cmp, n as i32)),
                    None => return err("după 's' lipsește pragul"),
                }
            }
            _ => break,
        }
    }
    Ok(Term::Dice(d))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Die {
    pub value: i32,
    pub kept: bool,
    /// This die was rolled because an earlier die exploded.
    pub extra: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PartRoll {
    pub sign: i32,
    pub dice: Vec<Die>,
    pub subtotal: i64,
    pub constant: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Roll {
    pub total: i64,
    pub parts: Vec<PartRoll>,
}

impl Roll {
    /// All kept dice values over every part.
    pub fn kept_values(&self) -> Vec<i32> {
        self.parts
            .iter()
            .flat_map(|p| p.dice.iter().filter(|d| d.kept).map(|d| d.value))
            .collect()
    }
}

impl fmt::Display for Roll {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for p in &self.parts {
            if !first {
                write!(
                    f,
                    " {} ",
                    if p.subtotal < 0 || p.sign < 0 {
                        '-'
                    } else {
                        '+'
                    }
                )?;
            } else if p.sign < 0 && !p.constant {
                write!(f, "-")?;
            }
            first = false;
            if p.constant {
                write!(f, "{}", p.subtotal.abs())?;
            } else {
                write!(f, "[")?;
                for (i, d) in p.dice.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    if d.kept {
                        write!(f, "{}", d.value)?;
                    } else {
                        write!(f, "({})", d.value)?;
                    }
                }
                write!(f, "]")?;
            }
        }
        write!(f, " = {}", self.total)
    }
}

fn roll_one(d: &DiceTerm, rng: &mut Rng) -> i32 {
    if d.fate {
        rng.below(3) as i32 - 1
    } else {
        rng.below(d.sides as usize) as i32 + 1
    }
}

fn roll_term(d: &DiceTerm, rng: &mut Rng) -> PartRoll {
    let mut dice: Vec<Die> = Vec::new();
    for _ in 0..d.count {
        let mut v = roll_one(d, rng);
        dice.push(Die {
            value: v,
            kept: true,
            extra: false,
        });
        if let Some((c, t)) = d.explode {
            let mut n = 0;
            while c.test(v, t) && n < MAX_EXPLOSIONS {
                v = roll_one(d, rng);
                dice.push(Die {
                    value: v,
                    kept: true,
                    extra: true,
                });
                n += 1;
            }
        }
    }
    // Keep logic over the whole pool (explosion dice included).
    if d.keep != Keep::All {
        let mut idx: Vec<usize> = (0..dice.len()).collect();
        idx.sort_by_key(|&i| (dice[i].value, i));
        let n = dice.len();
        let keep_n = match d.keep {
            Keep::Highest(k) | Keep::Lowest(k) => (k as usize).min(n),
            Keep::DropHighest(k) | Keep::DropLowest(k) => n.saturating_sub(k as usize),
            Keep::All => n,
        };
        let from_top = matches!(d.keep, Keep::Highest(_) | Keep::DropLowest(_));
        let mut kept = vec![false; n];
        if from_top {
            for &i in idx.iter().rev().take(keep_n) {
                kept[i] = true;
            }
        } else {
            for &i in idx.iter().take(keep_n) {
                kept[i] = true;
            }
        }
        for (i, die) in dice.iter_mut().enumerate() {
            die.kept = kept[i];
        }
    }
    let raw: i64 = match d.success {
        Some((c, t)) => dice.iter().filter(|x| x.kept && c.test(x.value, t)).count() as i64,
        None => dice.iter().filter(|x| x.kept).map(|x| x.value as i64).sum(),
    };
    PartRoll {
        sign: d.sign,
        dice,
        subtotal: raw * d.sign as i64,
        constant: false,
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, t) in self.terms.iter().enumerate() {
            match t {
                Term::Const(k) => {
                    if i == 0 {
                        write!(f, "{k}")?;
                    } else if *k < 0 {
                        write!(f, "-{}", -k)?;
                    } else {
                        write!(f, "+{k}")?;
                    }
                }
                Term::Dice(d) => {
                    if i > 0 || d.sign < 0 {
                        write!(f, "{}", if d.sign < 0 { '-' } else { '+' })?;
                    }
                    write!(f, "{}d", d.count)?;
                    if d.fate {
                        write!(f, "F")?;
                    } else {
                        write!(f, "{}", d.sides)?;
                    }
                    match d.keep {
                        Keep::All => {}
                        Keep::Highest(n) => write!(f, "kh{n}")?,
                        Keep::Lowest(n) => write!(f, "kl{n}")?,
                        Keep::DropHighest(n) => write!(f, "dh{n}")?,
                        Keep::DropLowest(n) => write!(f, "dl{n}")?,
                    }
                    if let Some((c, t)) = d.explode {
                        write!(f, "!{}{}", c.sym(), t)?;
                    }
                    if let Some((c, t)) = d.success {
                        write!(f, "s{}{}", c.sym(), t)?;
                    }
                }
            }
        }
        Ok(())
    }
}

/// Convenience: parse and roll in one step.
pub fn roll(expr: &str, rng: &mut Rng) -> Result<Roll, DiceError> {
    Ok(Expr::parse(expr)?.roll(rng))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn r(s: &str, seed: u64) -> Roll {
        roll(s, &mut Rng::new(seed)).unwrap()
    }

    #[test]
    fn parse_basic() {
        let e = Expr::parse("2d6+3").unwrap();
        assert_eq!(e.terms.len(), 2);
        assert_eq!(e.min(), 5);
        assert_eq!(e.max(), 15);
        assert_eq!(e.mean(), Some(10.0));
        assert_eq!(Expr::parse("d20").unwrap().max(), 20);
        assert_eq!(Expr::parse("D%").unwrap().max(), 100);
        assert_eq!(Expr::parse(" 1 d 8 - 2 ").unwrap().to_string(), "1d8-2");
        assert_eq!(Expr::parse("-1d4+10").unwrap().min(), 6);
    }

    #[test]
    fn parse_errors() {
        for bad in [
            "", "d", "2d", "2d0", "abc", "2d6+", "2d6x", "0d6", "5000d6", "2d6s", "1d6!>=", "+",
        ] {
            assert!(Expr::parse(bad).is_err(), "should fail: {bad}");
        }
    }

    #[test]
    fn range_property() {
        let e = Expr::parse("3d6+2").unwrap();
        let mut rng = Rng::new(7);
        for _ in 0..2000 {
            let t = e.roll(&mut rng).total;
            assert!((5..=20).contains(&t));
        }
    }

    #[test]
    fn deterministic() {
        assert_eq!(r("4d6kh3", 5), r("4d6kh3", 5));
    }

    #[test]
    fn keep_highest_lowest() {
        let mut rng = Rng::new(3);
        for _ in 0..500 {
            let x = roll("4d6kh3", &mut rng).unwrap();
            let all: Vec<i32> = x.parts[0].dice.iter().map(|d| d.value).collect();
            let mut s = all.clone();
            s.sort();
            assert_eq!(x.total, s[1..].iter().sum::<i32>() as i64);
            assert_eq!(x.parts[0].dice.iter().filter(|d| d.kept).count(), 3);
            let y = roll("4d6kl1", &mut rng).unwrap();
            let m = y.parts[0].dice.iter().map(|d| d.value).min().unwrap();
            assert_eq!(y.total, m as i64);
            let z = roll("5d10dl2", &mut rng).unwrap();
            assert_eq!(z.parts[0].dice.iter().filter(|d| d.kept).count(), 3);
            let w = roll("5d10dh2", &mut rng).unwrap();
            assert_eq!(w.parts[0].dice.iter().filter(|d| d.kept).count(), 3);
        }
        // advantage-style
        let e = Expr::parse("2d20kh1").unwrap();
        assert_eq!((e.min(), e.max()), (1, 20));
    }

    #[test]
    fn exploding() {
        let mut rng = Rng::new(11);
        let mut saw_extra = false;
        for _ in 0..500 {
            let x = roll("3d6!", &mut rng).unwrap();
            let d = &x.parts[0].dice;
            if d.iter().any(|d| d.extra) {
                saw_extra = true;
            }
            // every extra die follows a die that satisfied the explode condition
            for i in 0..d.len() {
                if d[i].extra {
                    assert!(d[i - 1].value == 6 || d[i - 1].extra);
                }
            }
            assert_eq!(x.total, d.iter().map(|d| d.value as i64).sum::<i64>());
        }
        assert!(saw_extra);
        // always-explode is capped
        let x = roll("1d4!>=1", &mut Rng::new(1)).unwrap();
        assert_eq!(x.parts[0].dice.len(), 1 + MAX_EXPLOSIONS as usize);
    }

    #[test]
    fn success_pools() {
        let mut rng = Rng::new(9);
        for _ in 0..500 {
            let x = roll("8d10s>=7", &mut rng).unwrap();
            let c = x.parts[0].dice.iter().filter(|d| d.value >= 7).count();
            assert_eq!(x.total, c as i64);
            assert!(x.total <= 8);
        }
        let e = Expr::parse("6d6s5").unwrap();
        assert_eq!((e.min(), e.max()), (0, 6));
        let y = roll("10d6s=6", &mut rng).unwrap();
        assert_eq!(
            y.total,
            y.parts[0].dice.iter().filter(|d| d.value == 6).count() as i64
        );
    }

    #[test]
    fn percentile_and_fate() {
        let mut rng = Rng::new(2);
        for _ in 0..500 {
            let p = roll("d%", &mut rng).unwrap().total;
            assert!((1..=100).contains(&p));
            let f = roll("4dF", &mut rng).unwrap().total;
            assert!((-4..=4).contains(&f));
        }
        let e = Expr::parse("4dF").unwrap();
        assert_eq!((e.min(), e.max()), (-4, 4));
    }

    #[test]
    fn subtraction_and_display() {
        let x = r("1d6-1d6+5", 4);
        let a = x.parts[0].dice[0].value as i64;
        let b = x.parts[1].dice[0].value as i64;
        assert_eq!(x.total, a - b + 5);
        assert!(x.to_string().contains('='));
        let e = Expr::parse("4d6kh3!s>=4").unwrap();
        assert_eq!(Expr::parse(&e.to_string()).unwrap(), e);
    }

    #[test]
    fn roundtrip_display() {
        for s in [
            "2d6+3", "1d20", "4d6kh3", "2d10!>=9", "5d6s>=5", "1d8-2", "3dF",
        ] {
            let e = Expr::parse(s).unwrap();
            assert_eq!(Expr::parse(&e.to_string()).unwrap(), e, "{s}");
        }
    }

    #[test]
    fn distribution_roughly_uniform() {
        let mut rng = Rng::new(123);
        let mut c = [0u32; 6];
        for _ in 0..12000 {
            c[roll("1d6", &mut rng).unwrap().total as usize - 1] += 1;
        }
        for n in c {
            assert!((1700..2300).contains(&n), "{c:?}");
        }
    }
}
