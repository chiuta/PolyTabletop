//! Utilități comune pentru jocurile valului Q5 (jocuri tradiționale rămase).

use polytabletop_core::{Outcome, Player};

/// Numărul afișat al jucătorului (1 sau 2).
pub fn pn(p: Player) -> usize {
    p.idx() + 1
}

/// Rezultat după scor: mai mare câștigă, egal = remiză.
pub fn by_score(a: i32, b: i32) -> Outcome {
    match a.cmp(&b) {
        std::cmp::Ordering::Greater => Outcome::Win(Player::One),
        std::cmp::Ordering::Less => Outcome::Win(Player::Two),
        std::cmp::Ordering::Equal => Outcome::Draw,
    }
}

/// Linia de final pentru `render` (gol cât timp jocul continuă).
pub fn verdict(o: Option<Outcome>) -> String {
    match o {
        Some(Outcome::Win(p)) => format!("Final: Jucător {} câștigă.\n", pn(p)),
        Some(Outcome::Draw) => "Final: remiză.\n".to_string(),
        None => String::new(),
    }
}

/// Numele unui pătrat: coloană (literă) + rând (de la 1).
pub fn sq_name(file: usize, rank: usize) -> String {
    format!("{}{}", (b'a' + file as u8) as char, rank + 1)
}

/// Citește un nume de pătrat de la începutul lui `s`; întoarce (coloană, rând, restul).
pub fn parse_sq(s: &str) -> Option<(usize, usize, &str)> {
    let b = s.as_bytes();
    if b.is_empty() || !b[0].is_ascii_lowercase() {
        return None;
    }
    let f = (b[0] - b'a') as usize;
    let mut i = 1;
    while i < b.len() && b[i].is_ascii_digit() {
        i += 1;
    }
    if i == 1 {
        return None;
    }
    let r: usize = s[1..i].parse().ok()?;
    if r == 0 {
        return None;
    }
    Some((f, r - 1, &s[i..]))
}

#[cfg(test)]
pub mod testkit {
    use polytabletop_core::rng::Rng;
    use polytabletop_core::{Game, GameObj};

    /// Jocuri aleatorii (se termină, mutări unice, nicio mutare lipsă) + AI legal.
    pub fn check_game<G: Game + 'static>(make: impl Fn(u64) -> G, seeds: u64, cap: usize) {
        for seed in 1..=seeds {
            let mut g = make(seed);
            let mut rng = Rng::new(seed * 31 + 7);
            let mut plies = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                assert!(
                    !mv.is_empty(),
                    "fără mutări dar jocul nu s-a încheiat\n{}",
                    g.render()
                );
                let mut t: Vec<String> = mv
                    .iter()
                    .map(|m| {
                        g.format_move(m)
                            .split_whitespace()
                            .collect::<String>()
                            .to_lowercase()
                    })
                    .collect();
                let n = t.len();
                t.sort();
                t.dedup();
                assert_eq!(t.len(), n, "mutări duplicate\n{}", g.render());
                let m = mv[rng.below(mv.len())].clone();
                g.apply(&m);
                plies += 1;
                assert!(plies < cap, "joc prea lung\n{}", g.render());
            }
            assert!(!g.render().is_empty());
            assert!(g.legal_moves().is_empty() || g.outcome().is_some());
        }
        // AI-ul joacă mutări legale, jucabile prin text
        for seed in 1..=2u64 {
            let mut b: Box<dyn GameObj> = Box::new(make(seed + 100));
            let mut n = 0;
            while b.outcome().is_none() && n < 60 {
                let m = b.ai_move(seed * 5 + n as u64).expect("AI fără mutare");
                assert!(b.legal_moves().contains(&m), "AI: mutare ilegală {m}");
                b.play(&m).expect("mutarea AI trebuie să fie jucabilă");
                n += 1;
            }
        }
    }
}
