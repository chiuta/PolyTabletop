//! Shuffleboard de masă (Marea Britanie), 2 jucători: pucuri alunecate pe un traseu cu zone de punctaj.
//!
//! * Traseul are 96 de unități. Zonele: 60–69 = 1 punct, 70–79 = 2, 80–89 = 3, 90–95 = 4 puncte
//!   („agățat” de margine); un puc care ajunge la 96 sau mai departe cade și se scoate.
//! * Într-o rundă fiecare jucător alunecă 4 pucuri, pe rând (J1 deschide rundele impare).
//!   Alegi forța (`tir 60` = țintești unitatea 60), apoi `lansez`: pucul ajunge la forța aleasă ±5
//!   (abaterea e aleasă la lansare). Un puc lovit transmite mișcarea mai departe.
//! * Punctaj la sfârșitul rundei: scorează doar jucătorul al cărui puc e cel mai departe; ia
//!   punctele tuturor pucurilor lui aflate mai departe decât cel mai bun puc al adversarului.
//! * Primul la 15 puncte (la finalul unei runde) câștigă; la egalitate se joacă o rundă în plus.
//!   După 30 de runde câștigă cine conduce.
//!
//! Mutări: `tir 30` … `tir 100` (din 5 în 5), `lansez`.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const TARGET: i32 = 15;
const FALL: i32 = 96;
const MAX_FRAMES: u8 = 30;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Move {
    Aim(i32),
    Slide,
}

#[derive(Clone)]
pub struct Shuffleboard {
    rng: Rng,
    pucks: Vec<(u8, i32)>,
    thrown: [u8; 2],
    score: [i32; 2],
    side: Player,
    pending: Option<i32>,
    frame: u8,
    log: String,
}

impl Default for Shuffleboard {
    fn default() -> Self {
        Self::new()
    }
}

fn pts(pos: i32) -> i32 {
    match pos {
        60..=69 => 1,
        70..=79 => 2,
        80..=89 => 3,
        90..=95 => 4,
        _ => 0,
    }
}

/// Alunecă un puc al lui `owner` până la `target` (cu transfer de mișcare).
fn slide(pucks: &[(u8, i32)], owner: u8, target: i32) -> Vec<(u8, i32)> {
    let mut v: Vec<(u8, i32)> = pucks.to_vec();
    v.push((owner, 0));
    let mut id = v.len() - 1;
    loop {
        let here = v[id].1;
        let next = (0..v.len())
            .filter(|&j| j != id && v[j].1 > here && v[j].1 <= target)
            .min_by_key(|&j| v[j].1);
        match next {
            None => {
                v[id].1 = target;
                break;
            }
            Some(j) => {
                v[id].1 = v[j].1 - 1;
                id = j;
            }
        }
    }
    v.retain(|p| p.1 < FALL);
    v
}

/// Punctaj al unei runde încheiate acum: (jucător, puncte).
fn frame_points(pucks: &[(u8, i32)]) -> Option<(u8, i32)> {
    let far = |o: u8| pucks.iter().filter(|p| p.0 == o).map(|p| p.1).max();
    let (a, b) = (far(0), far(1));
    let (w, other) = match (a, b) {
        (None, None) => return None,
        (Some(_), None) => (0u8, -1),
        (None, Some(_)) => (1u8, -1),
        (Some(x), Some(y)) => {
            if x > y {
                (0, y)
            } else if y > x {
                (1, x)
            } else {
                return None;
            }
        }
    };
    let s: i32 = pucks
        .iter()
        .filter(|p| p.0 == w && p.1 > other)
        .map(|p| pts(p.1))
        .sum();
    Some((w, s))
}

impl Shuffleboard {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Shuffleboard {
            rng: Rng::new(seed),
            pucks: Vec::new(),
            thrown: [0; 2],
            score: [0; 2],
            side: Player::One,
            pending: None,
            frame: 1,
            log: String::new(),
        }
    }

    fn over(&self) -> bool {
        (self.score.iter().any(|&s| s >= TARGET) && self.score[0] != self.score[1])
            || self.frame > MAX_FRAMES
    }

    fn opener(&self) -> Player {
        if self.frame % 2 == 1 {
            Player::One
        } else {
            Player::Two
        }
    }

    fn do_slide(&mut self, target: i32) {
        let me = self.side.idx();
        self.pucks = slide(&self.pucks, me as u8, target);
        self.thrown[me] += 1;
        if self.thrown == [4, 4] {
            match frame_points(&self.pucks) {
                Some((w, s)) if s > 0 => {
                    self.score[w as usize] += s;
                    self.log = format!("Runda {}: J{} ia {s} puncte.", self.frame, w + 1);
                }
                _ => self.log = format!("Runda {}: fără puncte.", self.frame),
            }
            self.frame += 1;
            self.pucks.clear();
            self.thrown = [0; 2];
            self.side = self.opener();
        } else {
            // alternează; dacă unul a terminat, celălalt continuă
            let other = 1 - me;
            if self.thrown[other] < 4 {
                self.side = self.side.other();
            }
        }
    }
}

impl Game for Shuffleboard {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over() {
            return Vec::new();
        }
        if self.pending.is_some() {
            return vec![Move::Slide];
        }
        (6..=20).map(|k| Move::Aim(k * 5)).collect()
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        match *m {
            Move::Aim(p) => self.pending = Some(p),
            Move::Slide => {
                let p = self.pending.take().unwrap_or(60);
                let noise = self.rng.below(11) as i32 - 5;
                let t = (p + noise).max(1);
                self.log = format!("Pucul a ajuns la {t}.");
                self.do_slide(t);
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over() {
            return None;
        }
        Some(match self.score[0].cmp(&self.score[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    /// Nu citește abaterea viitoare: pentru o țintă aleasă se face media pe toate abaterile.
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx() as u8;
        let val = |pucks: &[(u8, i32)]| -> i32 {
            match frame_points(pucks) {
                Some((w, s)) => {
                    let base = s.max(1);
                    if w == me {
                        base * 40
                    } else {
                        -base * 40
                    }
                }
                None => 0,
            }
        };
        let frame_v = if let Some(t) = self.pending {
            // media pe abateri, din perspectiva celui care aruncă (self.side)
            let mover = self.side.idx() as u8;
            let mut sum = 0;
            for n in -5..=5 {
                let after = slide(&self.pucks, mover, (t + n).max(1));
                sum += val(&after);
            }
            sum / 11
        } else {
            val(&self.pucks)
        };
        100 * (self.score[p.idx()] - self.score[1 - p.idx()]) + frame_v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Shuffleboard — runda {}/{MAX_FRAMES}, scor J1 {} – J2 {} (la {TARGET}). Pucuri aruncate: J1 {}/4, J2 {}/4\n",
            self.frame.min(MAX_FRAMES),
            self.score[0],
            self.score[1],
            self.thrown[0],
            self.thrown[1]
        );
        let mut line = ['.'; 48];
        for p in &self.pucks {
            let i = (p.1 / 2).clamp(0, 47) as usize;
            line[i] = if p.0 == 0 { 'x' } else { 'o' };
        }
        s.push_str(&format!("0 |{}| 96\n", line.iter().collect::<String>()));
        s.push_str("  zone: 60 (1)  70 (2)  80 (3)  90 (4, agățat)  96 = cade\n");
        let mut ps = self.pucks.clone();
        ps.sort_by_key(|p| p.1);
        let l: Vec<String> = ps.iter().map(|p| format!("J{}@{}", p.0 + 1, p.1)).collect();
        s.push_str(&format!("Pucuri: {}\n", l.join(" ")));
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        match self.outcome() {
            Some(Outcome::Win(p)) => s.push_str(&format!("Câștigă J{}.\n", p.idx() + 1)),
            Some(Outcome::Draw) => s.push_str("Remiză.\n"),
            None if self.pending.is_some() => s.push_str(&format!(
                "J{}: forță {} aleasă — scrie «lansez».\n",
                self.side.idx() + 1,
                self.pending.unwrap_or(0)
            )),
            None => s.push_str(&format!(
                "La mutare: J{} (tir 30…100)\n",
                self.side.idx() + 1
            )),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Aim(p) => format!("tir {p}"),
            Move::Slide => "lansez".into(),
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
    fn collision_transfers_motion_and_edge_drops() {
        let v = slide(&[(1, 70)], 0, 80);
        assert!(v.contains(&(0, 69)) && v.contains(&(1, 80)));
        let v = slide(&[], 0, 99);
        assert!(v.is_empty());
        assert_eq!(pts(92), 4);
    }

    #[test]
    fn unopposed_scoring() {
        let p = [(0u8, 85), (0, 65), (1, 75)];
        assert_eq!(frame_points(&p), Some((0, 3)));
        let p = [(0u8, 85), (1, 90)];
        assert_eq!(frame_points(&p), Some((1, 4)));
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=10u64 {
            let mut g: Box<dyn polytabletop_core::GameObj> =
                Box::new(Shuffleboard::with_seed(seed));
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.play(&m[rng.below(m.len())]).unwrap();
                n += 1;
                assert!(n < 2000);
            }
        }
    }

    #[test]
    fn ai_does_not_see_the_noise() {
        let mut a = Shuffleboard::with_seed(1);
        for _ in 0..16 {
            let mut b = a.clone();
            b.rng = Rng::new(777);
            let ma = polytabletop_core::GameObj::ai_move(&a, 5).unwrap();
            assert_eq!(ma, polytabletop_core::GameObj::ai_move(&b, 5).unwrap());
            polytabletop_core::GameObj::play(&mut a, &ma).unwrap();
        }
    }
}
