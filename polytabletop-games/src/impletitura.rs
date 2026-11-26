//! Împletitura — joc de echilibru pe covor cu cercuri colorate (variantă originală, în doi).
//!
//! Covorul are 4 rânduri (a–d) × 6 coloane (1–6) de cercuri în 4 culori (Roșu, Albastru, Verde,
//! Galben, câte 6 de fiecare; împărțirea depinde de seed). Fiecare jucător are 4 membre: mâna
//! stângă (`ms`), mâna dreaptă (`md`), piciorul stâng (`ps`) și piciorul drept (`pd`). La început
//! membrele sunt în afara covorului.
//!
//! * Tura începe cu rotița (mutarea forțată `gira`): ea dă un membru și o culoare. Rotița e
//!   extrasă din generatorul jocului, deci AI-ul nu o poate ști dinainte (nu trișează).
//! * Apoi muți acel membru pe un cerc **liber** de culoarea dată (`b3`). Poți și să-l lași unde
//!   e (numele cercului pe care stă deja), dacă are culoarea cerută.
//! * Raza: noul cerc trebuie să fie la distanță Chebyshev cel mult 3 (`RADIUS`) de celelalte
//!   membre ale tale aflate pe covor (mâinile și picioarele nu se pot „întinde” prea mult).
//! * Încurcare: un cerc nu poate fi ocupat de două membre, nici ale aceluiași jucător, nici ale
//!   adversarului — membrele adversarului îți blochează cercurile.
//! * Dacă nu există niciun cerc valid, singura mutare este `cad` (cazi): ești eliminat și
//!   adversarul câștigă. Ultimul rămas în picioare câștigă.
//!
//! Informație ascunsă: niciuna. Singurul hazard este rotița (seed determinist).
//! Siguranță: după 300 de mutări de membre fără nicio cădere, jocul este REMIZĂ (documentat).
//!
//! AI: evaluarea măsoară „flexibilitatea” (câte cercuri valide rămân pentru fiecare pereche
//! membru × culoare). Easy joacă aproape aleator, Normal își maximizează flexibilitatea,
//! Hard/Expert maximizează și diferența față de adversar (îi blochează cercurile).

use polytabletop_core::ai::{self, Level};
use polytabletop_core::rng::Rng;
use polytabletop_core::{hash_mix, AiStyle, Game, Outcome, Player};

pub const ROWS: usize = 4;
pub const COLS: usize = 6;
pub const CELLS: usize = ROWS * COLS;
pub const RADIUS: i32 = 3;
pub const MAX_PLIES: u32 = 300;
const NONE: u8 = 255;

pub const COLOR_NAMES: [&str; 4] = ["Roșu", "Albastru", "Verde", "Galben"];
const COLOR_LETTERS: [char; 4] = ['R', 'A', 'V', 'G'];
pub const LIMB_CODES: [&str; 4] = ["ms", "md", "ps", "pd"];
pub const LIMB_NAMES: [&str; 4] = [
    "mâna stângă",
    "mâna dreaptă",
    "piciorul stâng",
    "piciorul drept",
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    /// Rotița (mutare forțată).
    Spin,
    /// Membrul rotit merge pe cercul indicat (poate fi chiar cercul curent).
    Go(u8),
    /// Nu există cerc valid: cazi.
    Fall,
}

#[derive(Clone)]
pub struct Impletitura {
    colors: [u8; CELLS],
    /// `pos[jucător][membru]` = cercul ocupat sau `NONE`.
    pos: [[u8; 4]; 2],
    side: Player,
    /// (membru, culoare) dat de rotiță pentru tura curentă.
    spin: Option<(u8, u8)>,
    plies: u32,
    result: Option<Outcome>,
    rng: Rng,
}

impl Default for Impletitura {
    fn default() -> Self {
        Self::new()
    }
}

fn cell_name(c: u8) -> String {
    format!(
        "{}{}",
        (b'a' + (c as usize / COLS) as u8) as char,
        c as usize % COLS + 1
    )
}

#[cfg(test)]
fn parse_cell(s: &str) -> Option<u8> {
    let b = s.as_bytes();
    if b.len() != 2 {
        return None;
    }
    let r = b[0].checked_sub(b'a')? as usize;
    let c = b[1].checked_sub(b'1')? as usize;
    (r < ROWS && c < COLS).then_some((r * COLS + c) as u8)
}

fn cheb(a: u8, b: u8) -> i32 {
    let (ar, ac) = (a as i32 / COLS as i32, a as i32 % COLS as i32);
    let (br, bc) = (b as i32 / COLS as i32, b as i32 % COLS as i32);
    (ar - br).abs().max((ac - bc).abs())
}

impl Impletitura {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut colors = [0u8; CELLS];
        let mut idx: Vec<usize> = (0..CELLS).collect();
        crate::cards::shuffle(&mut idx, &mut rng);
        for (k, &i) in idx.iter().enumerate() {
            colors[i] = (k % 4) as u8;
        }
        Impletitura {
            colors,
            pos: [[NONE; 4]; 2],
            side: Player::One,
            spin: None,
            plies: 0,
            result: None,
            rng,
        }
    }

    pub fn color_of(&self, cell: usize) -> u8 {
        self.colors[cell]
    }

    /// Cercul ocupat de membrul `limb` al jucătorului `who` (`None` = în afara covorului).
    pub fn position(&self, who: Player, limb: usize) -> Option<usize> {
        let p = self.pos[who.idx()][limb];
        (p != NONE).then_some(p as usize)
    }

    /// Rotița curentă (membru, culoare), dacă a fost învârtită în tura asta.
    pub fn current_spin(&self) -> Option<(usize, usize)> {
        self.spin.map(|(l, c)| (l as usize, c as usize))
    }

    fn occupant(&self, cell: u8) -> Option<(usize, usize)> {
        for p in 0..2 {
            for l in 0..4 {
                if self.pos[p][l] == cell {
                    return Some((p, l));
                }
            }
        }
        None
    }

    /// Cercurile pe care `who` își poate pune `limb` când rotița dă `color`.
    fn valid_cells(&self, who: usize, limb: usize, color: u8) -> Vec<u8> {
        let mut out = Vec::new();
        for cell in 0..CELLS as u8 {
            if self.colors[cell as usize] != color {
                continue;
            }
            if let Some(o) = self.occupant(cell) {
                if o != (who, limb) {
                    continue;
                }
            }
            let in_range = (0..4).all(|l| {
                l == limb || self.pos[who][l] == NONE || cheb(cell, self.pos[who][l]) <= RADIUS
            });
            if in_range {
                out.push(cell);
            }
        }
        out
    }

    /// Flexibilitatea lui `who`: pentru fiecare pereche (membru, culoare), câte cercuri valide
    /// (cel mult 4) are; perechile fără niciun cerc (cădere sigură dacă ies la rotiță) costă mult.
    fn flex(&self, who: usize) -> i32 {
        let mut s = 0;
        for limb in 0..4 {
            for color in 0..4u8 {
                let n = self.valid_cells(who, limb, color).len() as i32;
                s += if n == 0 { -30 } else { n.min(4) };
            }
        }
        s
    }

    /// Evaluarea pe nivel (separată de nivelul global, ca să poată fi testată).
    pub fn score(&self, p: Player, level: Level) -> i32 {
        let me = p.idx();
        match level {
            Level::Easy => {
                let mut h = 7u64;
                for (i, &v) in self.pos.iter().flatten().enumerate() {
                    h ^= hash_mix((v as u64) << 8 | i as u64);
                }
                (hash_mix(h) % 200) as i32 - 100
            }
            Level::Normal => self.flex(me),
            Level::Hard | Level::Expert => self.flex(me) - self.flex(1 - me),
        }
    }
}

impl Game for Impletitura {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.result.is_some() {
            return Vec::new();
        }
        match self.spin {
            None => vec![Move::Spin],
            Some((limb, color)) => {
                let v = self.valid_cells(self.side.idx(), limb as usize, color);
                if v.is_empty() {
                    vec![Move::Fall]
                } else {
                    v.into_iter().map(Move::Go).collect()
                }
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        match *m {
            Move::Spin => {
                let limb = self.rng.below(4) as u8;
                let color = self.rng.below(4) as u8;
                self.spin = Some((limb, color));
            }
            Move::Go(c) => {
                if let Some((limb, _)) = self.spin.take() {
                    self.pos[self.side.idx()][limb as usize] = c;
                }
                self.plies += 1;
                self.side = self.side.other();
                if self.plies >= MAX_PLIES {
                    self.result = Some(Outcome::Draw);
                }
            }
            Move::Fall => {
                self.spin = None;
                self.result = Some(Outcome::Win(self.side.other()));
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.result
    }

    fn evaluate(&self, p: Player) -> i32 {
        // Nu citește rotița curentă: AI-ul nu are voie să „vadă” viitorul.
        self.score(p, ai::level())
    }

    fn render(&self) -> String {
        let mut s = String::from("Împletitura — covorul (culoare:membru; Ro=R Al=A Ve=V Ga=G)\n");
        s.push_str("     ");
        for c in 0..COLS {
            s.push_str(&format!("{:<6}", c + 1));
        }
        s.push('\n');
        for r in 0..ROWS {
            s.push_str(&format!("  {}  ", (b'a' + r as u8) as char));
            for c in 0..COLS {
                let cell = (r * COLS + c) as u8;
                let col = COLOR_LETTERS[self.colors[cell as usize] as usize];
                let occ = match self.occupant(cell) {
                    Some((p, l)) => format!("{}{}", p + 1, LIMB_CODES[l]),
                    None => "   ".to_string(),
                };
                s.push_str(&format!("{col}:{occ} "));
            }
            s.push('\n');
        }
        for p in 0..2 {
            let parts: Vec<String> = (0..4)
                .map(|l| {
                    let at = if self.pos[p][l] == NONE {
                        "-".to_string()
                    } else {
                        cell_name(self.pos[p][l])
                    };
                    format!("{} {}", LIMB_CODES[l], at)
                })
                .collect();
            s.push_str(&format!("J{}: {}\n", p + 1, parts.join(", ")));
        }
        match self.outcome() {
            Some(Outcome::Win(w)) => {
                s.push_str(&format!(
                    "Jocul s-a terminat: J{} a căzut; câștigă J{}.\n",
                    w.other().idx() + 1,
                    w.idx() + 1
                ));
            }
            Some(Outcome::Draw) => {
                s.push_str("Remiză: nimeni nu a căzut în 300 de mutări.\n");
            }
            None => match self.spin {
                None => s.push_str(&format!(
                    "J{} învârte rotița: scrie gira (mutări de membre: {}/{}).\n",
                    self.side.idx() + 1,
                    self.plies,
                    MAX_PLIES
                )),
                Some((l, c)) => {
                    let v = self.valid_cells(self.side.idx(), l as usize, c);
                    s.push_str(&format!(
                        "Rotița: J{} — {} pe {}. ",
                        self.side.idx() + 1,
                        LIMB_NAMES[l as usize],
                        COLOR_NAMES[c as usize]
                    ));
                    if v.is_empty() {
                        s.push_str("Niciun cerc valid: scrie cad.\n");
                    } else {
                        let names: Vec<String> = v.iter().map(|&c| cell_name(c)).collect();
                        s.push_str(&format!("Cercuri valide: {}.\n", names.join(" ")));
                    }
                }
            },
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match *m {
            Move::Spin => "gira".to_string(),
            Move::Go(c) => cell_name(c),
            Move::Fall => "cad".to_string(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        // Adâncimea 1 (2 la Hard/Expert, unde al doilea strat e doar `gira`): AI-ul nu
        // cercetează viitorul rotiței, deci nu poate trișa.
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spin_to(g: &mut Impletitura, limb: u8, color: u8) {
        g.spin = Some((limb, color));
    }

    fn cell(s: &str) -> u8 {
        parse_cell(s).unwrap()
    }

    fn play_random(seed: u64) -> (Impletitura, Vec<String>) {
        let mut g = Impletitura::with_seed(seed);
        let mut rng = Rng::new(seed ^ 0xABCD);
        let mut log = Vec::new();
        while g.outcome().is_none() {
            let mv = g.legal_moves();
            assert!(!mv.is_empty());
            let m = mv[rng.below(mv.len())];
            log.push(g.format_move(&m));
            g.apply(&m);
            assert!(log.len() < 4 * MAX_PLIES as usize);
        }
        (g, log)
    }

    #[test]
    fn mat_has_six_of_each_color_and_depends_on_seed() {
        let g = Impletitura::with_seed(5);
        for c in 0..4u8 {
            assert_eq!((0..CELLS).filter(|&i| g.color_of(i) == c).count(), 6);
        }
        let a = Impletitura::with_seed(5).render();
        assert_eq!(a, Impletitura::with_seed(5).render());
        let diff = (6..16u64).any(|s| Impletitura::with_seed(s).colors != g.colors);
        assert!(diff);
    }

    #[test]
    fn turn_starts_with_spin_then_colored_free_circles() {
        let mut g = Impletitura::with_seed(1);
        assert_eq!(g.legal_moves(), vec![Move::Spin]);
        assert_eq!(g.format_move(&Move::Spin), "gira");
        g.apply(&Move::Spin);
        let (limb, color) = g.current_spin().unwrap();
        let mv = g.legal_moves();
        assert_eq!(mv.len(), 6);
        for m in mv {
            let Move::Go(c) = m else { panic!() };
            assert_eq!(g.color_of(c as usize) as usize, color);
        }
        assert!(limb < 4);
    }

    #[test]
    fn occupied_circle_is_blocked_for_everyone() {
        let mut g = Impletitura::with_seed(2);
        let target = (0..CELLS as u8)
            .find(|&c| g.colors[c as usize] == 1)
            .unwrap();
        g.pos[1][0] = target; // adversarul stă acolo
        g.pos[0][1] = (0..CELLS as u8)
            .find(|&c| g.colors[c as usize] == 1 && c != target)
            .unwrap(); // propriul membru md stă pe alt cerc albastru
        spin_to(&mut g, 0, 1);
        let mv = g.legal_moves();
        assert_eq!(mv.len(), 4);
        assert!(!mv.contains(&Move::Go(target)));
        assert!(!mv.contains(&Move::Go(g.pos[0][1])));
    }

    #[test]
    fn radius_limits_the_move_and_staying_is_allowed() {
        let mut g = Impletitura::with_seed(3);
        g.colors = [0; CELLS];
        g.pos[0][1] = cell("a1");
        g.pos[0][0] = cell("a2");
        spin_to(&mut g, 0, 0);
        let mv = g.legal_moves();
        // la distanță ≤3 de a1: coloanele 1–4 (a1..d4), cu excepția cercului lui md (a1)
        assert!(mv.contains(&Move::Go(cell("d4"))));
        assert!(mv.contains(&Move::Go(cell("a2")))); // rămâne pe loc
        assert!(!mv.contains(&Move::Go(cell("a5"))));
        assert!(!mv.contains(&Move::Go(cell("b6"))));
        assert!(!mv.contains(&Move::Go(cell("a1")))); // ocupat de md
        assert_eq!(mv.len(), 16 - 1);
    }

    #[test]
    fn no_valid_circle_means_fall_and_opponent_wins() {
        let mut g = Impletitura::with_seed(4);
        g.colors = [0; CELLS];
        g.colors[cell("d6") as usize] = 3; // singurul cerc galben
        g.pos[0][1] = cell("a1");
        spin_to(&mut g, 0, 3);
        assert_eq!(g.legal_moves(), vec![Move::Fall]);
        g.apply(&Move::Fall);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
        assert!(g.legal_moves().is_empty());
        assert!(g.render().contains("câștigă J2"));
    }

    #[test]
    fn fall_when_the_only_circle_is_taken() {
        let mut g = Impletitura::with_seed(4);
        g.colors = [0; CELLS];
        g.colors[cell("b2") as usize] = 2;
        g.pos[1][3] = cell("b2");
        g.side = Player::Two;
        spin_to(&mut g, 0, 2);
        assert_eq!(g.legal_moves(), vec![Move::Fall]);
        g.apply(&Move::Fall);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn moving_frees_the_old_circle_and_passes_the_turn() {
        let mut g = Impletitura::with_seed(6);
        spin_to(&mut g, 2, 0);
        let c = match g.legal_moves()[0] {
            Move::Go(c) => c,
            _ => panic!(),
        };
        g.apply(&Move::Go(c));
        assert_eq!(g.position(Player::One, 2), Some(c as usize));
        assert_eq!(g.to_move(), Player::Two);
        assert!(g.current_spin().is_none());
        // J2 nu poate lua cercul ocupat de J1
        let col = g.colors[c as usize];
        spin_to(&mut g, 0, col);
        assert!(!g.legal_moves().contains(&Move::Go(c)));
    }

    #[test]
    fn text_moves_are_case_insensitive_and_illegal_ones_rejected() {
        let mut g: Box<dyn polytabletop_core::GameObj> = Box::new(Impletitura::with_seed(7));
        assert!(g.play("b3").is_err()); // întâi rotița
        assert_eq!(g.play(" GIRA ").unwrap(), "gira");
        assert!(g.play("zz").is_err());
        let first = g.legal_moves()[0].clone();
        assert_eq!(g.play(&first.to_uppercase()).unwrap(), first);
    }

    #[test]
    fn random_playouts_terminate() {
        let mut falls = 0;
        for seed in 1..=60u64 {
            let (g, log) = play_random(seed);
            assert!(g.outcome().is_some());
            if matches!(g.outcome(), Some(Outcome::Win(_))) {
                falls += 1;
                assert_eq!(log.last().unwrap(), "cad");
            }
        }
        assert!(falls > 0, "măcar o partidă trebuie să se termine cu cădere");
    }

    #[test]
    fn deterministic_and_replayable() {
        let (a, log) = play_random(11);
        let (b, log_b) = play_random(11);
        assert_eq!(log, log_b);
        assert_eq!(a.render(), b.render());
        // replay prin interfața text
        let mut r: Box<dyn polytabletop_core::GameObj> = Box::new(Impletitura::with_seed(11));
        for m in &log {
            r.play(m).unwrap();
        }
        assert_eq!(r.render(), a.render());
        assert_eq!(r.outcome(), a.outcome());
    }

    #[test]
    fn centred_limbs_are_more_flexible_than_cornered_ones() {
        let mut g = Impletitura::with_seed(9);
        g.pos[0] = [cell("b3"), cell("b4"), cell("c3"), cell("c4")];
        let centre = g.flex(0);
        g.pos[0] = [cell("a1"), cell("a2"), cell("b1"), cell("b2")];
        assert!(centre > g.flex(0));
    }

    #[test]
    fn hard_score_rewards_blocking_the_opponent() {
        let mut g = Impletitura::with_seed(9);
        g.pos[1] = [cell("a1"), cell("a2"), cell("b1"), cell("b2")];
        let base = g.score(Player::One, Level::Hard);
        // J1 își pune membrele în cercurile din colțul opus: scorul Hard e simetric cu al lui J2
        assert_eq!(base, -g.score(Player::Two, Level::Hard));
        // Normal ignoră adversarul
        let n = g.score(Player::One, Level::Normal);
        assert_eq!(n, g.flex(0));
        // Easy e zgomot determinist în [-100, 100)
        let e = g.score(Player::One, Level::Easy);
        assert!((-100..100).contains(&e));
        assert_eq!(e, g.score(Player::One, Level::Easy));
    }

    #[test]
    fn ai_finishes_games_without_peeking_at_the_next_spin() {
        for seed in 1..=6u64 {
            let mut g = Impletitura::with_seed(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = ai::best_move(&g, seed + n).expect("mutare");
                assert!(g.legal_moves().contains(&m));
                // același răspuns indiferent de viitorul generatorului (rotița următoare)
                let mut h = g.clone();
                h.rng = Rng::new(seed.wrapping_mul(977) ^ n ^ 0x55);
                assert_eq!(ai::best_move(&h, seed + n), Some(m));
                g.apply(&m);
                n += 1;
                assert!(n < 4 * MAX_PLIES as u64);
            }
        }
    }

    #[test]
    fn ai_picks_the_move_that_keeps_flexibility() {
        // Pe un covor în care doar un cerc păstrează membrele apropiate, AI-ul (Normal) îl alege.
        let mut g = Impletitura::with_seed(8);
        g.colors = [0; CELLS];
        g.colors[cell("a6") as usize] = 1;
        g.colors[cell("b2") as usize] = 1;
        g.pos[0][1] = cell("b3");
        g.pos[0][2] = cell("c3");
        g.pos[0][3] = cell("c4");
        spin_to(&mut g, 0, 1);
        let ms = g.legal_moves();
        assert_eq!(ms.len(), 2); // a6 e la distanță 3 de b3 (coloane 5 vs 2)
        let value = |m: &Move| {
            let mut h = g.clone();
            h.apply(m);
            h.score(Player::One, Level::Normal)
        };
        let best = ms.iter().map(value).max().unwrap();
        let chosen = ai::best_move(&g, 1).unwrap();
        assert_eq!(value(&chosen), best);
    }
}
