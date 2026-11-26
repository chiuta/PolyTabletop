//! Ajutoare comune pentru jocurile de cuvinte / de petrecere (prefix `w4`).
//!
//! Convenții:
//! * Pașii de decontare (dezvăluire, numărare, verificare) sunt mutări separate,
//!   cu o singură variantă, ale „arbitrului” ([`RESOLVER`] = Jucătorul 2): dacă
//!   Jucătorul 2 este calculatorul, le joacă singur, iar omul nu are de tastat
//!   nimic în plus.
//! * AI-ul nu trișează: `evaluate` citește doar [`Heur`], valoarea calculată în
//!   `apply` din informația pe care o are jucătorul care muta (nu din rezultatul
//!   mutării, care se decide abia la decontare).

use polytabletop_core::Player;

/// Jucătorul care joacă pașii forțați de decontare.
pub const RESOLVER: Player = Player::Two;

/// Litere mici, fără diacritice (pentru comparații și pentru textul mutărilor).
pub fn fold(s: &str) -> String {
    s.chars()
        .flat_map(|c| c.to_lowercase())
        .map(|c| match c {
            'ă' | 'â' => 'a',
            'î' => 'i',
            'ș' | 'ş' => 's',
            'ț' | 'ţ' => 't',
            _ => c,
        })
        .collect()
}

const STOP: &[&str] = &[
    "care", "este", "sunt", "pentru", "ceva", "unde", "cand", "fara", "mult", "foarte", "acest",
    "aceea", "poti", "poate", "daca", "dupa", "prin", "spre", "cele", "alte", "altul", "toate",
    "unei", "unui", "fiind", "apoi", "dintr", "atunci", "nimic", "cine", "intr",
];

/// Prefix de 4 litere al unui cuvânt (rădăcină brută), fără diacritice.
pub fn stem(w: &str) -> String {
    fold(w).chars().take(5).collect()
}

/// Jetoanele unui text: rădăcinile (4 litere) cuvintelor de cel puțin 4 litere,
/// fără cuvinte de legătură; fără dubluri.
pub fn tokens(text: &str) -> Vec<String> {
    let f = fold(text);
    let mut v: Vec<String> = Vec::new();
    for w in f.split(|c: char| !c.is_alphanumeric()) {
        if w.chars().count() >= 4 && !STOP.contains(&w) {
            let s: String = w.chars().take(4).collect();
            if !v.contains(&s) {
                v.push(s);
            }
        }
    }
    v
}

/// Mărime euristică atașată ultimei mutări a unui jucător (singura legătură
/// dintre `apply` și `evaluate`).
#[derive(Clone, Copy, Debug, Default)]
pub struct Heur {
    pub val: i32,
    pub by: Option<Player>,
}

impl Heur {
    pub fn set(&mut self, p: Player, v: i32) {
        self.val = v;
        self.by = Some(p);
    }
    pub fn clear(&mut self) {
        self.val = 0;
        self.by = None;
    }
    pub fn for_player(&self, p: Player) -> i32 {
        if self.by == Some(p) {
            self.val
        } else {
            0
        }
    }
}

/// Mărgini de joc: `[a, b]` indexat după jucător.
pub fn diff(pts: &[i32; 2], p: Player) -> i32 {
    pts[p.idx()] - pts[p.other().idx()]
}

pub fn result_of(pts: &[i32; 2]) -> polytabletop_core::Outcome {
    match pts[0].cmp(&pts[1]) {
        std::cmp::Ordering::Greater => polytabletop_core::Outcome::Win(Player::One),
        std::cmp::Ordering::Less => polytabletop_core::Outcome::Win(Player::Two),
        std::cmp::Ordering::Equal => polytabletop_core::Outcome::Draw,
    }
}

/// Ultimele `n` rânduri din jurnal, formatate pentru `render`.
pub fn log_tail(log: &[String], n: usize) -> String {
    let start = log.len().saturating_sub(n);
    let mut s = String::new();
    for l in &log[start..] {
        s.push_str("  · ");
        s.push_str(l);
        s.push('\n');
    }
    s
}
