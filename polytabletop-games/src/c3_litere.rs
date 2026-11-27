//! Scrisori Amestecate — variantă cooperativă originală de ghicit un cuvânt (lista C3, 2 jucători în echipă).
//!
//! Calculatorul alege un cuvânt secret de 5 litere din dicționarul comun. Pe rând, cei doi jucători
//! propun `ghic cuvant` (obligatoriu un cuvânt compatibil cu toate indiciile de până acum);
//! răspunsul (`rasp`) e mutare forțată: `+` literă la locul ei, `o` literă în cuvânt dar în alt
//! loc, `-` absentă. Echipa câștigă dacă ghicește în cel mult `MAXG` propuneri, altfel remiză.
//! Cuvântul secret nu apare în randare până la final și nu e citit de AI.
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const MAXG: usize = 8;
const L: usize = 5;

#[derive(Clone)]
pub struct Litere {
    secret: Vec<u8>,
    cands: Vec<Vec<u8>>,
    hist: Vec<(Vec<u8>, String)>,
    pending: Option<Vec<u8>>,
    solved: bool,
    side: usize,
}

pub fn new() -> Litere {
    with_seed(seed_now())
}

fn feedback(secret: &[u8], g: &[u8]) -> String {
    let mut out = vec!['-'; L];
    let mut used = [false; L];
    for i in 0..L {
        if g[i] == secret[i] {
            out[i] = '+';
            used[i] = true;
        }
    }
    for i in 0..L {
        if out[i] == '+' {
            continue;
        }
        if let Some(j) = (0..L).find(|&j| !used[j] && secret[j] == g[i]) {
            used[j] = true;
            out[i] = 'o';
        }
    }
    out.into_iter().collect()
}

pub fn with_seed(seed: u64) -> Litere {
    let mut rng = Rng::new(seed);
    let cands: Vec<Vec<u8>> = words().iter().filter(|w| w.len() == L).cloned().collect();
    let secret = cands[rng.below(cands.len())].clone();
    Litere {
        secret,
        cands,
        hist: Vec::new(),
        pending: None,
        solved: false,
        side: 0,
    }
}

fn txt(w: &[u8]) -> String {
    w.iter().map(|&c| (b'a' + c) as char).collect()
}

impl Litere {
    fn consistent(&self, w: &[u8]) -> bool {
        self.hist.iter().all(|(g, f)| feedback(w, g) == *f)
    }
    fn live(&self) -> Vec<&Vec<u8>> {
        self.cands.iter().filter(|w| self.consistent(w)).collect()
    }
}

impl Mini for Litere {
    fn side(&self) -> Player {
        if self.side == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        if self.pending.is_some() {
            return vec!["rasp".into()];
        }
        let live = self.live();
        let step = (live.len() / 300).max(1);
        live.iter()
            .step_by(step)
            .map(|w| format!("ghic {}", txt(w)))
            .collect()
    }
    fn step(&mut self, m: &str) {
        if let Some(g) = self.pending.take() {
            let f = feedback(&self.secret, &g);
            self.solved = f == "+++++";
            self.hist.push((g, f));
            self.side = 1 - self.side;
            return;
        }
        let w = crate::litere::normalize(&m[5..]).unwrap();
        self.pending = Some(w);
    }
    fn result(&self) -> Option<Outcome> {
        if self.solved {
            Some(Outcome::Win(Player::One))
        } else if self.hist.len() >= MAXG {
            Some(Outcome::Draw)
        } else {
            None
        }
    }
    fn eval(&self, _p: Player) -> i32 {
        // doar informație publică: indiciile cunoscute; ghicirea în așteptare nu e judecată
        match &self.pending {
            Some(_) => 0,
            None => -(self.hist.len() as i32),
        }
    }
    fn show(&self) -> String {
        let mut s = format!(
            "Scrisori Amestecate — propunerea {}/{}\n",
            self.hist.len() + 1,
            MAXG
        );
        for (g, f) in &self.hist {
            s.push_str(&format!("{}  {}\n", txt(g).to_uppercase(), f));
        }
        if self.result().is_some() {
            s.push_str(&format!("Cuvântul era: {}\n", txt(&self.secret)));
        }
        s.push_str(&format!(
            "La mutare: {} — ghic cuvant\n",
            pname(self.side())
        ));
        s
    }
}

#[cfg(test)]
mod t {
    #[test]
    fn plays() {
        crate::brand_util::check(super::with_seed, 100);
    }
    #[test]
    fn ai_ignores_secret() {
        let a = super::with_seed(3);
        let mut b = a.clone();
        b.secret = b.cands[0].clone();
        // jocul nu a început: indiciile sunt goale, deci mutările sunt aceleași
        assert_eq!(
            crate::brand_util::ai_of(&a, 1),
            crate::brand_util::ai_of(&b, 1)
        );
    }
}
