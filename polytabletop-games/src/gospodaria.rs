//! Gospodăria — joc original de plasare de muncitori și gospodărie (2 jucători).
//!
//! Mecanica „plasezi pe rând muncitori pe acțiuni exclusive, unele se
//! acumulează, recoltă și hrănire periodice” este generică; gospodăria,
//! acțiunile, numerele și textele sunt ale noastre. Informație completă, fără
//! zaruri (seed-ul stabilește doar cine începe).
//!
//! Simplificări: ferma nu e o grilă, ci contoare (camere, câmpuri, țarcuri);
//! o singură cultură semănată (cereale) și o singură specie (oi); fără cărți.
//!
//! Reguli:
//! * 6 runde; recoltă după rundele 2, 4 și 6. Familia începe cu 2 membri și 2
//!   camere. Fiecare membru e un muncitor pe rundă.
//! * Pe rând plasezi un muncitor pe o acțiune liberă (`pune lemn`); fiecare
//!   acțiune ține un singur muncitor pe rundă. Acțiunile cu „+” se acumulează
//!   cât timp nu sunt luate:
//!   - `lemn` (+3 lemn), `lut` (+2 lut), `peste` (+1 hrană, acumulat),
//!     `oi` (+1 oaie, acumulat; oile fără loc devin hrană 1:1);
//!   - `cereale` (+1 cereale), `legume` (+1 legumă), `zilier` (+2 hrană);
//!   - `arat` (un câmp nou, cel mult 5), `semanat n` (semeni n câmpuri goale:
//!     fiecare ia 1 cereale și primește 3), `camera` (3 lemn + 2 lut, cel mult
//!     4 camere), `tarc` (2 lemn, cel mult 3 țarcuri, câte 2 oi), `familie`
//!     (un membru nou dacă ai cameră liberă; muncește de runda următoare),
//!     `inceput` (1 hrană și începi runda următoare).
//! * Recoltă: fiecare câmp cu cereale dă 1 cereale. Hrănire: 2 hrană pe
//!   membru; se consumă automat hrana, apoi cereale (1), oi (2) și legume (2)
//!   în ordinea asta; fiecare hrană lipsă = o cerșetorie (−3 puncte). Apoi
//!   fiecare pereche de oi cu loc liber face un miel (loc: 2 pe țarc + 1 în
//!   casă).
//! * Punctaj final: câmpuri, țarcuri, cereale, oi, legume (0 = −1; 1–4 → 1–4
//!   puncte, 4+ = 4 puncte; oi 1–3=1, 4–5=2, 6–7=3, 8+=4; cereale la fel),
//!   camere peste 2: +1 fiecare, +3 pe membru de familie, −3 pe cerșetorie.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const NSP: usize = 13;
const SP_NAME: [&str; NSP] = [
    "lemn", "lut", "peste", "oi", "cereale", "legume", "zilier", "arat", "semanat", "camera",
    "tarc", "familie", "inceput",
];
const ROUNDS: u8 = 6;
const HARVEST: [u8; 3] = [2, 4, 6];
const MAX_FIELDS: usize = 5;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    /// acțiune, parametru (la semănat: câte câmpuri)
    Place(u8, u8),
}

#[derive(Clone, Default, Debug)]
struct Farm {
    wood: i32,
    clay: i32,
    grain: i32,
    veg: i32,
    food: i32,
    sheep: i32,
    fields: Vec<u8>,
    pens: i32,
    rooms: i32,
    family: i32,
    workers: i32,
    beg: i32,
}

#[derive(Clone)]
pub struct Gospodaria {
    farm: [Farm; 2],
    acc: [i32; NSP],
    occupied: [Option<Player>; NSP],
    round: u8,
    side: Player,
    starter: Player,
    next_starter: Player,
    outcome: Option<Outcome>,
    last: String,
}

impl Default for Gospodaria {
    fn default() -> Self {
        Self::new()
    }
}

fn cat(n: i32) -> i32 {
    match n {
        i32::MIN..=0 => -1,
        1..=4 => n,
        _ => 4,
    }
}

fn animal_cat(n: i32) -> i32 {
    match n {
        i32::MIN..=0 => -1,
        1..=3 => 1,
        4..=5 => 2,
        6..=7 => 3,
        _ => 4,
    }
}

impl Gospodaria {
    pub fn new() -> Self {
        Self::with_seed(polytabletop_core::rng::Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let starter = if polytabletop_core::rng::Rng::new(seed).below(2) == 0 {
            Player::One
        } else {
            Player::Two
        };
        let mk = |food: i32| Farm {
            food,
            rooms: 2,
            family: 2,
            workers: 2,
            ..Farm::default()
        };
        let mut g = Gospodaria {
            farm: [mk(2), mk(3)],
            acc: [0; NSP],
            occupied: [None; NSP],
            round: 1,
            side: starter,
            starter,
            next_starter: starter,
            outcome: None,
            last: String::new(),
        };
        // cine începe primește 2 hrană, celălalt 3
        if starter == Player::Two {
            g.farm = [mk(3), mk(2)];
        }
        g.accumulate();
        g
    }

    fn accumulate(&mut self) {
        self.acc[0] += 3;
        self.acc[1] += 2;
        self.acc[2] += 1;
        self.acc[3] += 1;
    }

    fn sheep_cap(f: &Farm) -> i32 {
        f.pens * 2 + 1
    }

    pub fn score(&self, p: Player) -> i32 {
        let f = &self.farm[p.idx()];
        let grain = f.grain + f.fields.iter().map(|&x| x as i32).sum::<i32>();
        cat(f.fields.len() as i32)
            + cat(f.pens)
            + animal_cat(grain)
            + animal_cat(f.sheep)
            + cat(f.veg)
            + (f.rooms - 2).max(0)
            + 3 * f.family
            - 3 * f.beg
    }

    fn empty_fields(f: &Farm) -> usize {
        f.fields.iter().filter(|&&x| x == 0).count()
    }

    fn can(&self, p: Player, sp: usize) -> bool {
        let f = &self.farm[p.idx()];
        match sp {
            7 => f.fields.len() < MAX_FIELDS,
            8 => Self::empty_fields(f) > 0 && f.grain > 0,
            9 => f.wood >= 3 && f.clay >= 2 && f.rooms < 4,
            10 => f.wood >= 2 && f.pens < 3,
            11 => f.family < f.rooms && f.family < 4,
            _ => true,
        }
    }

    fn give_sheep(f: &mut Farm, n: i32) {
        let room = (Self::sheep_cap(f) - f.sheep).max(0);
        let keep = n.min(room);
        f.sheep += keep;
        f.food += n - keep;
    }

    fn harvest(&mut self) {
        for f in self.farm.iter_mut() {
            for fld in f.fields.iter_mut() {
                if *fld > 0 {
                    *fld -= 1;
                    f.grain += 1;
                }
            }
            let mut need = 2 * f.family;
            let t = need.min(f.food);
            f.food -= t;
            need -= t;
            let t = need.min(f.grain);
            f.grain -= t;
            need -= t;
            while need > 0 && f.sheep > 0 {
                f.sheep -= 1;
                need -= 2;
                if need < 0 {
                    f.food += -need;
                    need = 0;
                }
            }
            while need > 0 && f.veg > 0 {
                f.veg -= 1;
                need -= 2;
                if need < 0 {
                    f.food += -need;
                    need = 0;
                }
            }
            f.beg += need;
            if f.sheep >= 2 && f.sheep < Self::sheep_cap(f) {
                f.sheep += 1;
            }
        }
    }

    fn end_round(&mut self) {
        if HARVEST.contains(&self.round) {
            self.harvest();
        }
        if self.round >= ROUNDS {
            let (a, b) = (self.score(Player::One), self.score(Player::Two));
            self.outcome = Some(if a > b {
                Outcome::Win(Player::One)
            } else if b > a {
                Outcome::Win(Player::Two)
            } else {
                Outcome::Draw
            });
            return;
        }
        self.round += 1;
        self.occupied = [None; NSP];
        // acțiunile luate se golesc; cele neluate păstrează stocul
        self.accumulate();
        self.starter = self.next_starter;
        for f in self.farm.iter_mut() {
            f.workers = f.family;
        }
        self.side = self.starter;
    }

    /// Alege cine joacă mai departe; sare peste cine nu mai poate.
    fn settle(&mut self) {
        loop {
            if self.outcome.is_some() {
                return;
            }
            for p in [Player::One, Player::Two] {
                if self.farm[p.idx()].workers > 0 && !self.has_move(p) {
                    self.farm[p.idx()].workers = 0;
                }
            }
            let w = [self.farm[0].workers, self.farm[1].workers];
            if w[0] == 0 && w[1] == 0 {
                self.end_round();
                continue;
            }
            if self.farm[self.side.idx()].workers == 0 {
                self.side = self.side.other();
            }
            return;
        }
    }

    fn has_move(&self, p: Player) -> bool {
        (0..NSP).any(|sp| self.occupied[sp].is_none() && self.can(p, sp))
    }
}

impl Game for Gospodaria {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome.is_some() {
            return Vec::new();
        }
        let p = self.side;
        let f = &self.farm[p.idx()];
        let mut v = Vec::new();
        for sp in 0..NSP {
            if self.occupied[sp].is_some() || !self.can(p, sp) {
                continue;
            }
            if sp == 8 {
                let max = Self::empty_fields(f).min(f.grain as usize);
                for n in 1..=max {
                    v.push(Move::Place(sp as u8, n as u8));
                }
            } else {
                v.push(Move::Place(sp as u8, 0));
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        let Move::Place(sp, n) = *m;
        let sp = sp as usize;
        let p = self.side;
        let i = p.idx();
        self.occupied[sp] = Some(p);
        self.farm[i].workers -= 1;
        let take = std::mem::take(&mut self.acc[sp]);
        {
            let f = &mut self.farm[i];
            match sp {
                0 => f.wood += take,
                1 => f.clay += take,
                2 => f.food += take,
                3 => Self::give_sheep(f, take),
                4 => f.grain += 1,
                5 => f.veg += 1,
                6 => f.food += 2,
                7 => f.fields.push(0),
                8 => {
                    let mut left = n;
                    for fld in f.fields.iter_mut() {
                        if left > 0 && *fld == 0 {
                            *fld = 3;
                            f.grain -= 1;
                            left -= 1;
                        }
                    }
                }
                9 => {
                    f.wood -= 3;
                    f.clay -= 2;
                    f.rooms += 1;
                }
                10 => {
                    f.wood -= 2;
                    f.pens += 1;
                }
                11 => f.family += 1,
                _ => {
                    f.food += 1;
                    self.next_starter = p;
                }
            }
        }
        self.last = format!("J{} pune pe {}", i + 1, SP_NAME[sp]);
        self.side = p.other();
        self.settle();
    }

    fn outcome(&self) -> Option<Outcome> {
        self.outcome
    }

    fn evaluate(&self, p: Player) -> i32 {
        let side = |q: Player| -> i32 {
            let f = &self.farm[q.idx()];
            let seeds: i32 = f.fields.iter().map(|&x| x as i32).sum();
            let mut v = self.score(q) * 100;
            v += f.wood * 14 + f.clay * 16 + f.grain * 25 + f.veg * 40;
            v += f.food * 22 + f.sheep * 45 + seeds * 28;
            v += f.family * 120;
            // nevoia de hrană până la următoarea recoltă
            let next = HARVEST.iter().find(|&&r| r >= self.round).copied();
            if next.is_some() {
                let need = 2 * f.family;
                let have = f.food + f.grain + seeds / 3 + f.sheep * 2 + f.veg * 2;
                if have < need {
                    v -= (need - have) * 120;
                }
            }
            v
        };
        side(p) - side(p.other())
    }

    fn render(&self) -> String {
        let mut s = format!("Gospodăria — runda {}/{}\n", self.round, ROUNDS);
        s.push_str("Acțiuni: ");
        let parts: Vec<String> = (0..NSP)
            .map(|sp| {
                let mut t = SP_NAME[sp].to_string();
                if sp < 4 && self.acc[sp] > 0 {
                    t.push_str(&format!("+{}", self.acc[sp]));
                }
                match self.occupied[sp] {
                    Some(p) => format!("({t}:J{})", p.idx() + 1),
                    None => t,
                }
            })
            .collect();
        s.push_str(&parts.join(" "));
        s.push('\n');
        for q in [Player::One, Player::Two] {
            let f = &self.farm[q.idx()];
            let flds: Vec<String> = f.fields.iter().map(|x| x.to_string()).collect();
            s.push_str(&format!(
                "J{}: {} puncte | lemn {} lut {} cereale {} legume {} hrană {} oi {} | câmpuri [{}] țarcuri {} camere {} familie {} (muncitori liberi {}) cerșetorii {}\n",
                q.idx() + 1,
                self.score(q),
                f.wood,
                f.clay,
                f.grain,
                f.veg,
                f.food,
                f.sheep,
                flds.join(" "),
                f.pens,
                f.rooms,
                f.family,
                f.workers,
                f.beg
            ));
        }
        if HARVEST.contains(&self.round) {
            s.push_str("La sfârșitul acestei runde e recoltă.\n");
        }
        if !self.last.is_empty() {
            s.push_str(&format!("Ultima mutare: {}\n", self.last));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        let Move::Place(sp, n) = *m;
        if sp == 8 {
            format!("pune semanat {n}")
        } else {
            format!("pune {}", SP_NAME[sp as usize])
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(7)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    fn mv(g: &mut Gospodaria, s: &str) {
        let want = s.to_string();
        let m = g
            .legal_moves()
            .into_iter()
            .find(|m| g.format_move(m) == want)
            .unwrap_or_else(|| panic!("mutare ilegală {s}\n{}", g.render()));
        g.apply(&m);
    }

    #[test]
    fn start_and_accumulation() {
        let g = Gospodaria::with_seed(1);
        assert_eq!(g.acc[0], 3);
        assert_eq!(g.farm[0].family, 2);
        assert_eq!(g.legal_moves().len(), 9);
    }

    #[test]
    fn exclusive_spaces_and_turn_order() {
        let mut g = Gospodaria::with_seed(2);
        let first = g.to_move();
        mv(&mut g, "pune lemn");
        assert_eq!(g.to_move(), first.other());
        assert!(!g
            .legal_moves()
            .iter()
            .any(|m| g.format_move(m) == "pune lemn"));
        assert_eq!(g.farm[first.idx()].wood, 3);
        assert_eq!(g.acc[0], 0);
    }

    #[test]
    fn unused_accumulates() {
        let mut g = Gospodaria::with_seed(3);
        for n in ["cereale", "legume", "zilier", "arat"] {
            mv(&mut g, &format!("pune {n}"));
        }
        assert_eq!(g.round, 2);
        assert_eq!(g.acc[0], 6); // 3 + 3
        assert_eq!(g.acc[3], 2);
    }

    #[test]
    fn building_and_sowing() {
        let mut g = Gospodaria::with_seed(4);
        let p = g.to_move();
        g.farm[p.idx()].wood = 5;
        g.farm[p.idx()].clay = 2;
        mv(&mut g, "pune camera");
        assert_eq!(g.farm[p.idx()].rooms, 3);
        assert_eq!(g.farm[p.idx()].wood, 2);
        mv(&mut g, "pune zilier"); // J2
        g.farm[p.idx()].grain = 2;
        g.farm[p.idx()].fields = vec![0, 0];
        mv(&mut g, "pune semanat 2");
        assert_eq!(g.farm[p.idx()].fields, vec![3, 3]);
        assert_eq!(g.farm[p.idx()].grain, 0);
    }

    #[test]
    fn family_growth_needs_room() {
        let mut g = Gospodaria::with_seed(5);
        let names: Vec<String> = g.legal_moves().iter().map(|m| g.format_move(m)).collect();
        assert!(!names.contains(&"pune familie".to_string()));
        let p = g.to_move();
        g.farm[p.idx()].rooms = 3;
        let names: Vec<String> = g.legal_moves().iter().map(|m| g.format_move(m)).collect();
        assert!(names.contains(&"pune familie".to_string()));
    }

    #[test]
    fn harvest_feeds_and_begs() {
        let mut g = Gospodaria::with_seed(6);
        g.farm[0].food = 0;
        g.farm[0].grain = 1;
        g.farm[0].fields = vec![3];
        g.farm[1].food = 10;
        g.farm[1].sheep = 2;
        g.farm[1].pens = 1;
        g.harvest();
        // J1: 1 din câmp + 1 cereale = 2 din 4 necesare => 2 cerșetorii
        assert_eq!(g.farm[0].beg, 2);
        assert_eq!(g.farm[0].fields, vec![2]);
        // J2: hrană suficientă, 2 oi cu loc -> miel
        assert_eq!(g.farm[1].food, 6);
        assert_eq!(g.farm[1].sheep, 3);
    }

    #[test]
    fn sheep_overflow_becomes_food() {
        let mut f = Farm {
            food: 0,
            ..Farm::default()
        };
        Gospodaria::give_sheep(&mut f, 3);
        assert_eq!(f.sheep, 1);
        assert_eq!(f.food, 2);
    }

    #[test]
    fn scoring_categories() {
        let mut g = Gospodaria::with_seed(7);
        g.farm[0] = Farm {
            rooms: 3,
            family: 3,
            sheep: 4,
            veg: 1,
            fields: vec![3, 0],
            pens: 1,
            grain: 2,
            ..Farm::default()
        };
        // câmpuri 2, țarcuri 1, cereale (2+3=5) 2, oi 2, legume 1, camere 1, familie 9
        assert_eq!(g.score(Player::One), 2 + 1 + 2 + 2 + 1 + 1 + 9);
        g.farm[0].beg = 1;
        assert_eq!(g.score(Player::One), 18 - 3);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=30u64 {
            let mut g = Gospodaria::with_seed(seed);
            let mut rng = Rng::new(seed ^ 0x77);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty(), "{}", g.render());
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 500);
                for f in &g.farm {
                    assert!(f.wood >= 0 && f.clay >= 0 && f.grain >= 0 && f.sheep >= 0);
                    assert!(f.sheep <= Gospodaria::sheep_cap(f));
                }
            }
        }
    }

    #[test]
    fn ai_beats_random_mostly() {
        let (mut ai, mut rnd) = (0, 0);
        for seed in 1..=8u64 {
            let ai_side = if seed % 2 == 0 {
                Player::One
            } else {
                Player::Two
            };
            let mut g = Gospodaria::with_seed(seed);
            let mut rng = Rng::new(seed * 3);
            while g.outcome().is_none() {
                let m = if g.to_move() == ai_side {
                    polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap()
                } else {
                    let l = g.legal_moves();
                    l[rng.below(l.len())]
                };
                g.apply(&m);
            }
            match g.outcome() {
                Some(Outcome::Win(p)) if p == ai_side => ai += 1,
                Some(Outcome::Win(_)) => rnd += 1,
                _ => {}
            }
        }
        assert!(ai > rnd, "AI {ai} vs random {rnd}");
    }
}
