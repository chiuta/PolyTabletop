//! Cuvinte în Trei Runde — aceleași șase cărți, indicii tot mai sărace (variantă originală, 2 jucători).
//!
//! 12 cuvinte publice, fiecare cu trei însușiri 0..3 (categorie, mărime, ton). Mutarea forțată `trag`
//! alege 6 cuvinte secrete și amestecă ordinea (la începutul fiecărei runde din 3). La fiecare carte,
//! dătătorul de indicii (alternează) arată: runda 1 două însușiri (`indiciu a,b`), runda 2 una
//! (`indiciu a`), runda 3 doar un bit din numărul cuvântului (`gest b`). Ghicitorul are două încercări
//! (`ghic w`); în rundele 2–3 candidații sunt doar cuvintele deja dezvăluite (memorie!). Mutarea
//! forțată `verifica`: prima încercare corectă: ghicitor +2, dătător +1; a doua: ghicitor +1,
//! dătător +1. Câștigă scorul mai mare.
#![allow(
    clippy::needless_range_loop,
    clippy::manual_is_multiple_of,
    clippy::manual_memcpy,
    clippy::collapsible_match
)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const WORDS: [&str; 12] = [
    "cerb",
    "sanie",
    "orgă",
    "lanternă",
    "pod",
    "mărar",
    "scoică",
    "balenă",
    "fluier",
    "turn",
    "gheizer",
    "nisip",
];
fn feat(w: usize, f: usize) -> i32 {
    ((w * 3 + f * 5 + (w * (f + 1)) / 3 + w / 5) % 4) as i32
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Ph {
    Deal,
    Clue,
    Guess,
    Resolve,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Clue {
    Feats(usize, usize),
    One(usize),
    Bit(usize),
}

#[derive(Clone)]
pub struct Trei {
    viewer: Option<Player>,
    deck: Vec<usize>,
    order: Vec<usize>,
    round: usize,
    pos: usize,
    clue: Option<Clue>,
    tries: u8,
    wrong: Vec<usize>,
    seen: Vec<usize>,
    guess: usize,
    score: [i32; 2],
    ph: Ph,
    rng: Rng,
}

pub fn new() -> Trei {
    Trei::with_seed(seed_now())
}

impl Trei {
    pub fn with_seed(seed: u64) -> Self {
        Trei {
            viewer: None,
            deck: Vec::new(),
            order: Vec::new(),
            round: 0,
            pos: 0,
            clue: None,
            tries: 0,
            wrong: Vec::new(),
            seen: Vec::new(),
            guess: 0,
            score: [0; 2],
            ph: Ph::Deal,
            rng: Rng::new(seed),
        }
    }
    fn giver(&self) -> usize {
        (self.round * 6 + self.pos) % 2
    }
    fn secret(&self) -> usize {
        self.deck[self.order[self.pos]]
    }
    fn cands(&self) -> Vec<usize> {
        let base: Vec<usize> = if self.round == 0 {
            (0..12).collect()
        } else {
            self.seen.clone()
        };
        base.into_iter()
            .filter(|w| !self.wrong.contains(w))
            .collect()
    }
    fn consistent(&self, w: usize, c: Clue, truth: usize) -> bool {
        match c {
            Clue::Feats(a, b) => feat(w, a) == feat(truth, a) && feat(w, b) == feat(truth, b),
            Clue::One(a) => feat(w, a) == feat(truth, a),
            Clue::Bit(b) => (w >> b) & 1 == (truth >> b) & 1,
        }
    }
    /// Candidați consistenți cu indiciul, în ipoteza că adevărul este `truth` (folosit doar la dătător sau la verificare).
    fn n_consistent(&self, c: Clue, truth: usize) -> usize {
        self.cands()
            .iter()
            .filter(|&&w| self.consistent(w, c, truth))
            .count()
    }
    fn shown(&self, c: Clue) -> String {
        let t = self.secret();
        match c {
            Clue::Feats(a, b) => format!(
                "însușirea {a} = {}, însușirea {b} = {}",
                feat(t, a),
                feat(t, b)
            ),
            Clue::One(a) => format!("însușirea {a} = {}", feat(t, a)),
            Clue::Bit(b) => format!("bitul {b} al numărului = {}", (t >> b) & 1),
        }
    }
}

impl Mini for Trei {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        let t = match self.ph {
            Ph::Deal => 0,
            Ph::Clue => self.giver(),
            Ph::Guess => 1 - self.giver(),
            Ph::Resolve => 0,
        };
        if t == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            Ph::Deal => vec!["trag".into()],
            Ph::Resolve => vec!["verifica".into()],
            Ph::Clue => match self.round {
                0 => vec![
                    "indiciu 0,1".into(),
                    "indiciu 0,2".into(),
                    "indiciu 1,2".into(),
                ],
                1 => (0..3).map(|a| format!("indiciu {a}")).collect(),
                _ => (0..4).map(|b| format!("gest {b}")).collect(),
            },
            Ph::Guess => self.cands().iter().map(|w| format!("ghic {w}")).collect(),
        }
    }
    fn step(&mut self, m: &str) {
        match m {
            "trag" => {
                if self.deck.is_empty() {
                    let mut pool: Vec<usize> = (0..12).collect();
                    shuf(&mut pool, &mut self.rng);
                    self.deck = pool[..6].to_vec();
                }
                self.order = (0..6).collect();
                let mut o = std::mem::take(&mut self.order);
                shuf(&mut o, &mut self.rng);
                self.order = o;
                self.pos = 0;
                self.clue = None;
                self.tries = 0;
                self.wrong.clear();
                self.ph = Ph::Clue;
            }
            "verifica" => {
                let g = self.giver();
                let s = 1 - g;
                let t = self.secret();
                if self.guess == t {
                    self.score[s] += if self.tries == 0 { 2 } else { 1 };
                    self.score[g] += 1;
                    self.next_card();
                } else {
                    self.wrong.push(self.guess);
                    self.tries += 1;
                    if self.tries >= 2 {
                        self.next_card();
                    } else {
                        self.ph = Ph::Guess;
                    }
                }
            }
            _ if m.starts_with("ghic") => {
                self.guess = m[5..].parse().unwrap();
                self.ph = Ph::Resolve;
            }
            _ => {
                let rest = m.split(' ').nth(1).unwrap();
                let nums: Vec<usize> = rest.split(',').map(|x| x.parse().unwrap()).collect();
                self.clue = Some(if m.starts_with("gest") {
                    Clue::Bit(nums[0])
                } else if nums.len() == 2 {
                    Clue::Feats(nums[0], nums[1])
                } else {
                    Clue::One(nums[0])
                });
                self.ph = Ph::Guess;
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.round >= 3 {
            Some(by_score(self.score[0], self.score[1]))
        } else {
            None
        }
    }
    fn eval(&self, p: Player) -> i32 {
        let i = pi(p);
        let o = 1 - i;
        let mut e = 20 * (self.score[i] - self.score[o]);
        match self.ph {
            Ph::Guess if i == self.giver() => {
                if let Some(c) = self.clue {
                    // dătătorul preferă indiciile care lasă puțini candidați
                    let n = self.n_consistent(c, self.secret()) as i32;
                    e += 30 / n.max(1);
                }
            }
            Ph::Resolve if i == 1 - self.giver() => {
                // ghicitorul: cât de plauzibil este cuvântul ales față de indiciul vizibil
                if let Some(c) = self.clue {
                    let t = self.secret();
                    // fără a citi adevărul: număr de candidați compatibili cu ce se vede
                    let ok = match c {
                        Clue::Feats(a, b) => {
                            feat(self.guess, a) == feat(t, a) && feat(self.guess, b) == feat(t, b)
                        }
                        Clue::One(a) => feat(self.guess, a) == feat(t, a),
                        Clue::Bit(b) => (self.guess >> b) & 1 == (t >> b) & 1,
                    };
                    let n = self
                        .cands()
                        .iter()
                        .filter(|&&w| match c {
                            Clue::Feats(a, b) => {
                                feat(w, a) == feat(t, a) && feat(w, b) == feat(t, b)
                            }
                            Clue::One(a) => feat(w, a) == feat(t, a),
                            Clue::Bit(b) => (w >> b) & 1 == (t >> b) & 1,
                        })
                        .count() as i32;
                    // „vizibil” = compatibil cu indiciul (indiciul este public): aceasta nu dezvăluie dacă ghicitul e exact
                    e += if ok { 30 / n.max(1) } else { -15 };
                }
            }
            _ => {}
        }
        e
    }
    fn show(&self) -> String {
        let mut s = format!(
            "Cuvinte în Trei Runde — runda {}/3, cartea {}/6 | J1 {} / J2 {}\n",
            (self.round + 1).min(3),
            self.pos + 1,
            self.score[0],
            self.score[1]
        );
        s.push_str("Cuvinte (nr:nume [categorie,mărime,ton]):\n");
        for w in 0..12 {
            s.push_str(&format!(
                "  {w}:{} {:?}{}\n",
                WORDS[w],
                (0..3).map(|f| feat(w, f)).collect::<Vec<_>>(),
                if self.seen.contains(&w) {
                    " (în pachet)"
                } else {
                    ""
                }
            ));
        }
        if self.ph != Ph::Deal {
            s.push_str(&format!(
                "Dătător J{}; încercarea {}; greșite: {:?}\n",
                self.giver() + 1,
                self.tries + 1,
                self.wrong
            ));
            if let Some(c) = self.clue {
                s.push_str(&format!("Indiciu: {}\n", self.shown(c)));
            }
            let me = self.viewer.map(pi).unwrap_or(pi(self.side()));
            if self.ph == Ph::Clue && me == self.giver() {
                s.push_str(&format!(
                    "Cuvântul secret (doar dătătorul): {}\n",
                    self.secret()
                ));
            }
        }
        s
    }
    fn view(&mut self, w: Option<Player>) {
        self.set_v(w);
    }
}

impl Trei {
    fn set_v(&mut self, w: Option<Player>) {
        self.viewer = w;
    }
    fn next_card(&mut self) {
        let t = self.secret();
        if !self.seen.contains(&t) {
            self.seen.push(t);
        }
        self.pos += 1;
        self.tries = 0;
        self.wrong.clear();
        self.clue = None;
        if self.pos >= 6 {
            self.round += 1;
            self.ph = Ph::Deal;
        } else {
            self.ph = Ph::Clue;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn trei_ok() {
        check(Trei::with_seed, 300);
    }

    #[test]
    fn ai_nu_vede_cuvantul_secret_al_dataorului() {
        // ghicitorul vede doar indiciul: orice alt cuvânt cu aceleași însușiri dezvăluite e o lume posibilă
        crate::p3_comun::ai_blind_check(
            "trei_runde5",
            |s| B(Trei::with_seed(s)),
            |g, me, seed| {
                let t = &mut g.0;
                if t.ph != Ph::Guess || me.idx() == t.giver() {
                    return;
                }
                let (Some(c), true) = (t.clue, t.pos < t.order.len()) else {
                    return;
                };
                let truth = t.secret();
                let alt: Vec<usize> = t
                    .cands()
                    .into_iter()
                    .filter(|&w| w != truth && t.consistent(w, c, truth))
                    .collect();
                if !alt.is_empty() {
                    let at = t.order[t.pos];
                    t.deck[at] = alt[seed as usize % alt.len()];
                }
                // cărțile încă netrase nu le știe nimeni
                let from = t.pos + 1;
                if from < t.order.len() {
                    crate::p3_comun::shuffle_with(&mut t.order[from..], seed);
                }
            },
            12,
        );
    }
}
