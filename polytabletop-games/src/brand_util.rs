//! Utilitare comune pentru jocurile „de familie” (variante originale, lista B2).
//!
//! `Mini` e un contract simplu (mutări text); `B<M>` îl transformă în `Game`.
//! Regula AI: alpha-beta cu adâncime fixă (`depth`, deci determinist și fără ceas);
//! jocurile cu șansă sau informație ascunsă folosesc adâncime 0 (o singură mutare, fără
//! răspunsul adversarului: nu se vede nicio extragere viitoare), mutări forțate pentru
//! extragerile aleatoare, iar `eval` nu citește informațiile ascunse ale adversarului.
#![allow(clippy::needless_range_loop)]

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub trait Mini: Clone {
    fn side(&self) -> Player;
    fn moves(&self) -> Vec<String>;
    /// `m` este garantat una dintre `moves()`.
    fn step(&mut self, m: &str);
    fn result(&self) -> Option<Outcome>;
    fn eval(&self, p: Player) -> i32;
    fn show(&self) -> String;
    /// Adâncimea fixă a căutării (0 = o singură mutare, fără răspunsul adversarului — pentru jocurile
    /// cu șansă/informație ascunsă). Căutarea nu depinde de ceasul mașinii: rezultat determinist.
    fn depth(&self) -> u32;
    fn view(&mut self, _w: Option<Player>) {}
}

#[derive(Clone)]
pub struct B<M: Mini>(pub M);

impl<M: Mini> Game for B<M> {
    type Move = String;
    fn to_move(&self) -> Player {
        self.0.side()
    }
    fn legal_moves(&self) -> Vec<String> {
        if self.0.result().is_some() {
            return Vec::new();
        }
        self.0.moves()
    }
    fn apply(&mut self, m: &String) {
        self.0.step(m);
    }
    fn outcome(&self) -> Option<Outcome> {
        self.0.result()
    }
    fn evaluate(&self, p: Player) -> i32 {
        self.0.eval(p)
    }
    fn render(&self) -> String {
        self.0.show()
    }
    fn format_move(&self, m: &String) -> String {
        m.clone()
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(self.0.depth())
    }
    fn set_viewer(&mut self, who: Option<Player>) {
        self.0.view(who);
    }
}

pub fn pi(p: Player) -> usize {
    p.idx()
}

pub fn pname(p: Player) -> &'static str {
    match p {
        Player::One => "J1",
        Player::Two => "J2",
    }
}

/// Rezultat după puncte: mai mult = câștigă, egal = remiză.
pub fn by_score(a: i32, b: i32) -> Outcome {
    match a.cmp(&b) {
        std::cmp::Ordering::Greater => Outcome::Win(Player::One),
        std::cmp::Ordering::Less => Outcome::Win(Player::Two),
        std::cmp::Ordering::Equal => Outcome::Draw,
    }
}

pub fn shuf<T>(v: &mut [T], rng: &mut Rng) {
    for i in (1..v.len()).rev() {
        let j = rng.below(i + 1);
        v.swap(i, j);
    }
}

pub fn seed_now() -> u64 {
    Rng::from_clock().next_u64()
}

/// Poliomino: celule (r, c) normalizate, cu rotații/oglindiri unice.
pub fn orientations(shape: &[(i8, i8)], mirror: bool) -> Vec<Vec<(i8, i8)>> {
    let mut out: Vec<Vec<(i8, i8)>> = Vec::new();
    let mut cur: Vec<(i8, i8)> = shape.to_vec();
    for m in 0..(if mirror { 2 } else { 1 }) {
        if m == 1 {
            cur = cur.iter().map(|&(r, c)| (r, -c)).collect();
        }
        for _ in 0..4 {
            cur = cur.iter().map(|&(r, c)| (c, -r)).collect();
            let mr = cur.iter().map(|p| p.0).min().unwrap();
            let mc = cur.iter().map(|p| p.1).min().unwrap();
            let mut n: Vec<(i8, i8)> = cur.iter().map(|&(r, c)| (r - mr, c - mc)).collect();
            n.sort();
            if !out.contains(&n) {
                out.push(n);
            }
        }
    }
    out
}

/// Direcțiile hex axiale (q, r).
pub const HEX: [(i32, i32); 6] = [(1, 0), (1, -1), (0, -1), (-1, 0), (-1, 1), (0, 1)];

/// Teste comune: partide aleatorii care se termină, mutări legale, AI legal,
/// determinism. `plies` = plafonul de mutări pentru o partidă.
#[cfg(test)]
pub fn check<M: Mini + 'static>(mk: fn(u64) -> M, plies: usize) {
    use polytabletop_core::GameObj;
    polytabletop_core::ai::set_time_budget_override_ms(Some(6));
    for seed in 1..=3u64 {
        let mut g = B(mk(seed));
        let mut rng = Rng::new(seed * 77);
        let mut n = 0;
        while GameObj::outcome(&g).is_none() && n < plies {
            let lm = GameObj::legal_moves(&g);
            assert!(
                !lm.is_empty(),
                "fără mutări dar jocul nu s-a terminat\n{}",
                g.0.show()
            );
            let m = lm[rng.below(lm.len())].clone();
            GameObj::play(&mut g, &m).unwrap();
            n += 1;
        }
        assert!(
            GameObj::outcome(&g).is_some(),
            "partida nu s-a terminat în {plies} mutări\n{}",
            g.0.show()
        );
        assert!(!g.0.show().is_empty());
    }
    // determinism
    let run = |seed: u64| {
        let mut g = B(mk(seed));
        let mut rng = Rng::new(5);
        let mut n = 0;
        while GameObj::outcome(&g).is_none() && n < 60 {
            let lm = GameObj::legal_moves(&g);
            let m = lm[rng.below(lm.len())].clone();
            GameObj::play(&mut g, &m).unwrap();
            n += 1;
        }
        g.0.show()
    };
    assert_eq!(run(9), run(9));
    // AI: legal și duce partida la capăt contra unui jucător aleatoriu
    for seed in 1..=2u64 {
        let mut g = B(mk(seed));
        let mut rng = Rng::new(seed + 1000);
        let mut n = 0;
        while GameObj::outcome(&g).is_none() && n < plies {
            let lm = GameObj::legal_moves(&g);
            let m = if GameObj::to_move(&g) == Player::One || seed == 2 && n % 3 == 0 {
                let a = GameObj::ai_move(&g, rng.next_u64()).expect("AI fără mutare");
                assert!(lm.contains(&a), "AI a ales mutare ilegală {a}");
                a
            } else {
                lm[rng.below(lm.len())].clone()
            };
            GameObj::play(&mut g, &m).unwrap();
            n += 1;
        }
        assert!(
            GameObj::outcome(&g).is_some(),
            "partida cu AI nu s-a terminat\n{}",
            g.0.show()
        );
    }
}

/// Dicționar comun (cuvinte românești uzuale, fără diacritice, litere 0..25) pentru jocurile cu litere.
pub fn words() -> &'static Vec<Vec<u8>> {
    static W: std::sync::OnceLock<Vec<Vec<u8>>> = std::sync::OnceLock::new();
    W.get_or_init(|| {
        let mut set = std::collections::BTreeSet::new();
        for line in include_str!("../data/litere_dict.txt").lines() {
            if line.trim_start().starts_with('#') {
                continue;
            }
            for tok in line.split_whitespace() {
                if let Some(w) = crate::litere::normalize(tok) {
                    if (2..=9).contains(&w.len()) {
                        set.insert(w);
                    }
                }
            }
        }
        set.into_iter().collect()
    })
}

pub fn is_word(w: &[u8]) -> bool {
    static S: std::sync::OnceLock<std::collections::HashSet<Vec<u8>>> = std::sync::OnceLock::new();
    S.get_or_init(|| words().iter().cloned().collect())
        .contains(w)
}

/// Forme poliomino folosite de jocurile cu plăci (11 forme).
pub fn poly(i: usize) -> Vec<(i8, i8)> {
    match i % 11 {
        0 => vec![(0, 0), (0, 1)],
        1 => vec![(0, 0), (0, 1), (0, 2)],
        2 => vec![(0, 0), (1, 0), (1, 1)],
        3 => vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        4 => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        5 => vec![(0, 0), (0, 1), (0, 2), (1, 1)],
        6 => vec![(0, 0), (1, 0), (2, 0), (2, 1)],
        7 => vec![(0, 1), (0, 2), (1, 0), (1, 1)],
        8 => vec![(0, 0), (0, 1), (1, 0), (1, 1), (2, 0)],
        9 => vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
        _ => vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
    }
}

/// Doar pentru teste: mutarea AI-ului pentru starea dată.
#[cfg(test)]
pub fn ai_of<M: Mini + 'static>(m: &M, seed: u64) -> Option<String> {
    polytabletop_core::GameObj::ai_move(&B(m.clone()), seed)
}
