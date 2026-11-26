//! Mehen (Egiptul antic), jocul „șarpelui” pe un traseu în spirală, 2 jucători (reconstrucție).
//!
//! Regulile originale nu s-au păstrat; aceasta e o reconstrucție simplă, bazată pe descrieri publice:
//! * Șarpele are 18 pătrate (0 = coadă, 17 = cap). Fiecare jucător are 3 piese (lei), la început în afara tablei.
//! * Patru bețe de aruncat: numărul fețelor luminoase 1–4; zero fețe luminoase = 6. Mutarea e o
//!   aruncare (`arunc`), apoi alegi piesa. Aruncările 1, 4 și 6 dau încă o aruncare.
//! * O piesă intră pe coadă cu un **1** (`intra`), merge spre cap, se întoarce pe același șarpe
//!   (35 de pași în total) și iese acasă; ieșirea nu cere număr exact.
//! * Un pătrat poate fi ocupat de o singură piesă; aterizezi pe adversar = îl trimiți în afara tablei;
//!   pe piesa ta nu poți ateriza. Dacă nu ai nicio mutare, treci (`pas`).
//! * Câștigă cine aduce primul toate cele 3 piese acasă.
//!
//! Mutări: `arunc`, `intra`, `m1`/`m2`/`m3` (piesa 1–3), `pas`.

use crate::q2_util::end_line;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const LEG: i8 = 17;
const PATH: i8 = 2 * LEG + 1; // 35: ieșit
const PIECES: usize = 3;
const OFF: i8 = -1;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Throw,
    Enter(usize),
    Step(usize),
    Pass,
}

fn square(t: i8) -> i8 {
    if t <= LEG {
        t
    } else {
        2 * LEG - t
    }
}

#[derive(Clone)]
pub struct Mehen {
    rng: Rng,
    t: [[i8; PIECES]; 2],
    side: Player,
    roll: Option<u8>,
    log: String,
}

impl Default for Mehen {
    fn default() -> Self {
        Self::new()
    }
}

impl Mehen {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        Mehen {
            rng: Rng::new(seed),
            t: [[OFF; PIECES]; 2],
            side: Player::One,
            roll: None,
            log: String::new(),
        }
    }
    pub fn home(&self, p: Player) -> usize {
        self.t[p.idx()].iter().filter(|&&x| x >= PATH).count()
    }
    fn occupant(&self, sq: i8) -> Option<(usize, usize)> {
        for p in 0..2 {
            for i in 0..PIECES {
                let t = self.t[p][i];
                if (0..PATH).contains(&t) && square(t) == sq {
                    return Some((p, i));
                }
            }
        }
        None
    }
    fn can_land(&self, me: usize, sq: i8) -> bool {
        !matches!(self.occupant(sq), Some((p, _)) if p == me)
    }
    fn moves_for(&self, v: u8) -> Vec<Move> {
        let me = self.side.idx();
        let mut out = Vec::new();
        for i in 0..PIECES {
            let t = self.t[me][i];
            if t == OFF {
                if v == 1 && self.can_land(me, 0) && !out.contains(&Move::Enter(0)) {
                    out.push(Move::Enter(0));
                }
            } else if t < PATH {
                let nt = t + v as i8;
                if nt >= PATH || self.can_land(me, square(nt)) {
                    out.push(Move::Step(i));
                }
            }
        }
        out
    }
    fn progress(&self, p: usize) -> i32 {
        self.t[p]
            .iter()
            .map(|&t| if t == OFF { 0 } else { t as i32 + 1 })
            .sum()
    }
}

impl Game for Mehen {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        match self.roll {
            None => vec![Move::Throw],
            Some(v) => {
                let m = self.moves_for(v);
                if m.is_empty() {
                    vec![Move::Pass]
                } else {
                    m
                }
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        let me = self.side.idx();
        match m {
            Move::Throw => {
                let c = (0..4).filter(|_| self.rng.below(2) == 1).count() as u8;
                let v = if c == 0 { 6 } else { c };
                self.roll = Some(v);
                self.log = format!("J{} aruncă: {v}", me + 1);
            }
            Move::Pass => {
                let v = self.roll.take().unwrap_or(0);
                self.log.push_str(" — fără mutare.");
                if !matches!(v, 1 | 4 | 6) {
                    self.side = self.side.other();
                }
            }
            Move::Enter(_) | Move::Step(_) => {
                let v = self.roll.take().unwrap_or(1);
                let (i, nt) = match m {
                    Move::Enter(_) => ((0..PIECES).find(|&k| self.t[me][k] == OFF).unwrap(), 0i8),
                    Move::Step(i) => (*i, self.t[me][*i] + v as i8),
                    _ => unreachable!(),
                };
                if nt < PATH {
                    if let Some((p, k)) = self.occupant(square(nt)) {
                        if p != me {
                            self.t[p][k] = OFF;
                            self.log.push_str(" — captură!");
                        }
                    }
                }
                self.t[me][i] = nt.min(PATH);
                if !matches!(v, 1 | 4 | 6) {
                    self.side = self.side.other();
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        for p in [Player::One, Player::Two] {
            if self.home(p) == PIECES {
                return Some(Outcome::Win(p));
            }
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        self.progress(p.idx()) - self.progress(1 - p.idx())
    }

    fn render(&self) -> String {
        let mut line = ['.'; 18];
        for (p, row) in self.t.iter().enumerate() {
            for &t in row {
                if (0..PATH).contains(&t) {
                    let sq = square(t) as usize;
                    let c = if t <= LEG { 'a' } else { 'r' };
                    line[sq] = if p == 0 { c.to_ascii_uppercase() } else { c };
                }
            }
        }
        let mut s = String::from("Mehen — șarpele (0 = coadă … 17 = cap); MAJUSCULE = J1, minuscule = J2; a = tur, r = retur\n");
        s.push_str(&format!(
            "{}\n",
            line.iter().map(|c| format!("{c} ")).collect::<String>()
        ));
        for p in 0..2 {
            let st: Vec<String> = self.t[p]
                .iter()
                .map(|&t| match t {
                    OFF => "afară".into(),
                    t if t >= PATH => "acasă".into(),
                    t => format!("{}{}", if t <= LEG { "tur " } else { "retur " }, square(t)),
                })
                .collect();
            s.push_str(&format!("J{}: {}\n", p + 1, st.join(", ")));
        }
        if let Some(v) = self.roll {
            s.push_str(&format!("Aruncare curentă: {v}\n"));
        }
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        s.push_str(&end_line(self.outcome(), self.side));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Throw => "arunc".into(),
            Move::Enter(_) => "intra".into(),
            Move::Step(i) => format!("m{}", i + 1),
            Move::Pass => "pas".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enter_needs_a_one_and_pieces_capture() {
        let mut g = Mehen::with_seed(1);
        g.roll = Some(3);
        assert_eq!(g.legal_moves(), vec![Move::Pass]);
        g.roll = Some(1);
        assert_eq!(g.legal_moves(), vec![Move::Enter(0)]);
        g.t[0][0] = 5;
        g.t[1][0] = 8;
        g.roll = Some(3);
        g.apply(&Move::Step(0));
        assert_eq!(g.t[1][0], OFF);
        assert_eq!(g.t[0][0], 8);
    }

    #[test]
    fn shared_squares_on_the_way_back() {
        let g = Mehen::with_seed(1);
        assert_eq!(square(20), 14);
        assert_eq!(square(17), 17);
        assert_eq!(g.home(Player::One), 0);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=10u64 {
            let mut g: Box<dyn polytabletop_core::GameObj> = Box::new(Mehen::with_seed(seed));
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                g.play(&m[rng.below(m.len())]).unwrap();
                n += 1;
                assert!(n < 3500, "{}", g.render());
            }
        }
    }

    #[test]
    fn ai_ignores_rng() {
        let mut a = Mehen::with_seed(1);
        a.roll = Some(1);
        let mut b = a.clone();
        b.rng = Rng::new(5);
        assert_eq!(
            polytabletop_core::GameObj::ai_move(&a, 1),
            polytabletop_core::GameObj::ai_move(&b, 1)
        );
    }
}
