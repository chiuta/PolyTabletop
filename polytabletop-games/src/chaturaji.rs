//! Chaturaji („cei patru regi”, India, descris de Al-Biruni c. 1030), adaptare pentru doi jucători.
//!
//! * Tabla 8×8; patru armate (câte un rege, un elefant, un cal, o barcă și patru pioni) în
//!   cele patru laturi: **Sud** (1) și **Nord** (3) = J1, **Est** (2) și **Vest** (4) = J2.
//!   Fiecare jucător conduce două armate aliate care nu se pot captura între ele.
//! * La începutul turei arunci două zaruri (mutarea separată `arunc`); fiecare zar se joacă
//!   pe rând: 1 și 5 = pion sau rege, 2 = barcă, 3 = cal, 4 și 6 = elefant. Cu un zar muți o
//!   piesă potrivită dintr-o armată a ta sau spui `pas`.
//! * Mutări: regele ca la șah (un pas, 8 direcții), elefantul ca turnul, calul ca în șah,
//!   barca sare exact două pătrate pe diagonală, pionul face un pas înainte (spre latura opusă
//!   a armatei lui) și capturează pe diagonală înainte; fără pas dublu. Un pion ajuns la
//!   marginea opusă devine elefant/cal/barcă dacă armata a pierdut o astfel de piesă
//!   (în ordinea asta), altfel rămâne pion blocat.
//! * Captura: piesele adverse valorează pion 1, barcă 2, cal 3, elefant 4, rege 5; nu există
//!   șah. Jocul se termină când o echipă își pierde ambii regi (câștigă cealaltă) sau după
//!   400 de semi-mutări (câștigă cine a adunat mai multe puncte).
//! * Fără „triumful bărcilor” și fără tronul (sinhasana) din regulile istorice.
//!
//! Mutări: `a2a3` (de la, la), `arunc`, `pas`.

use crate::q5_util::{by_score, sq_name, verdict};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const PAWN: u8 = 1;
const BOAT: u8 = 2;
const HORSE: u8 = 3;
const ELEPH: u8 = 4;
const KING: u8 = 5;
const VAL: [i32; 6] = [0, 1, 2, 3, 4, 5];
const FWD: [(i32, i32); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Roll,
    Go(u8, u8),
    Skip,
}

#[derive(Clone)]
pub struct Chaturaji {
    b: [u8; 64], // (armata << 3) | tip; 0 = gol
    side: Player,
    dice: Vec<u8>,
    pts: [i32; 2],
    plies: u16,
    rng: Rng,
    log: String,
}

impl Default for Chaturaji {
    fn default() -> Self {
        Self::new()
    }
}

fn army(c: u8) -> usize {
    (c >> 3) as usize
}
fn kind(c: u8) -> u8 {
    c & 7
}
fn team(c: u8) -> usize {
    army(c) % 2
}
fn sq(i: usize) -> String {
    sq_name(i % 8, i / 8)
}

impl Chaturaji {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut b = [0u8; 64];
        let put = |b: &mut [u8; 64], a: usize, sqn: &str, k: u8| {
            let s = sqn.as_bytes();
            b[(s[1] - b'1') as usize * 8 + (s[0] - b'a') as usize] = ((a << 3) as u8) | k;
        };
        let layout: [(usize, [&str; 4], [&str; 4]); 4] = [
            (0, ["a1", "b1", "c1", "d1"], ["a2", "b2", "c2", "d2"]),
            (1, ["h1", "h2", "h3", "h4"], ["g1", "g2", "g3", "g4"]),
            (2, ["h8", "g8", "f8", "e8"], ["h7", "g7", "f7", "e7"]),
            (3, ["a8", "a7", "a6", "a5"], ["b8", "b7", "b6", "b5"]),
        ];
        for (a, pieces, pawns) in layout {
            for (s, k) in pieces.iter().zip([BOAT, HORSE, ELEPH, KING]) {
                put(&mut b, a, s, k);
            }
            for s in pawns {
                put(&mut b, a, s, PAWN);
            }
        }
        Chaturaji {
            b,
            side: Player::One,
            dice: Vec::new(),
            pts: [0; 2],
            plies: 0,
            rng: Rng::new(seed),
            log: String::new(),
        }
    }

    fn kings(&self, t: usize) -> usize {
        self.b
            .iter()
            .filter(|&&c| c != 0 && kind(c) == KING && team(c) == t)
            .count()
    }

    fn material(&self, t: usize) -> i32 {
        self.b
            .iter()
            .filter(|&&c| c != 0 && team(c) == t)
            .map(|&c| VAL[kind(c) as usize])
            .sum()
    }

    fn targets(&self, i: usize) -> Vec<usize> {
        let c = self.b[i];
        let t = team(c);
        let (r, f) = ((i / 8) as i32, (i % 8) as i32);
        let mut out = Vec::new();
        let free = |rr: i32, ff: i32| -> Option<usize> {
            if (0..8).contains(&rr) && (0..8).contains(&ff) {
                let j = (rr * 8 + ff) as usize;
                if self.b[j] == 0 || team(self.b[j]) != t {
                    return Some(j);
                }
            }
            None
        };
        match kind(c) {
            KING => {
                for dr in -1..=1 {
                    for df in -1..=1 {
                        if (dr, df) != (0, 0) {
                            out.extend(free(r + dr, f + df));
                        }
                    }
                }
            }
            HORSE => {
                for (dr, df) in [
                    (1, 2),
                    (2, 1),
                    (-1, 2),
                    (-2, 1),
                    (1, -2),
                    (2, -1),
                    (-1, -2),
                    (-2, -1),
                ] {
                    out.extend(free(r + dr, f + df));
                }
            }
            BOAT => {
                for (dr, df) in [(2, 2), (2, -2), (-2, 2), (-2, -2)] {
                    out.extend(free(r + dr, f + df));
                }
            }
            ELEPH => {
                for (dr, df) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let (mut rr, mut ff) = (r + dr, f + df);
                    while (0..8).contains(&rr) && (0..8).contains(&ff) {
                        let j = (rr * 8 + ff) as usize;
                        if self.b[j] == 0 {
                            out.push(j);
                        } else {
                            if team(self.b[j]) != t {
                                out.push(j);
                            }
                            break;
                        }
                        rr += dr;
                        ff += df;
                    }
                }
            }
            _ => {
                let (dr, df) = FWD[army(c)];
                let (rr, ff) = (r + dr, f + df);
                if (0..8).contains(&rr)
                    && (0..8).contains(&ff)
                    && self.b[(rr * 8 + ff) as usize] == 0
                {
                    out.push((rr * 8 + ff) as usize);
                }
                for s in [-1i32, 1] {
                    let (cr, cf) = (r + dr + s * df.abs(), f + df + s * dr.abs());
                    if (0..8).contains(&cr) && (0..8).contains(&cf) {
                        let j = (cr * 8 + cf) as usize;
                        if self.b[j] != 0 && team(self.b[j]) != t {
                            out.push(j);
                        }
                    }
                }
            }
        }
        out
    }

    fn die_kinds(d: u8) -> &'static [u8] {
        match d {
            1 | 5 => &[PAWN, KING],
            2 => &[BOAT],
            3 => &[HORSE],
            _ => &[ELEPH],
        }
    }

    fn promote_kind(&self, a: usize) -> Option<u8> {
        [ELEPH, HORSE, BOAT].into_iter().find(|&k| {
            !self
                .b
                .iter()
                .any(|&c| c != 0 && army(c) == a && kind(c) == k)
        })
    }
}

impl Game for Chaturaji {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        if self.dice.is_empty() {
            return vec![Move::Roll];
        }
        let d = self.dice[0];
        let kinds = Self::die_kinds(d);
        let mut v = Vec::new();
        for i in 0..64 {
            let c = self.b[i];
            if c != 0 && team(c) == self.side.idx() && kinds.contains(&kind(c)) {
                for j in self.targets(i) {
                    v.push(Move::Go(i as u8, j as u8));
                }
            }
        }
        v.push(Move::Skip);
        v
    }

    fn apply(&mut self, m: &Move) {
        self.plies += 1;
        match m {
            Move::Roll => {
                let a = self.rng.d6();
                let b = self.rng.d6();
                self.dice = vec![a, b];
                self.log = format!("Jucător {} aruncă {a} și {b}.", self.side.idx() + 1);
            }
            Move::Skip => {
                self.dice.remove(0);
            }
            Move::Go(from, to) => {
                let (from, to) = (*from as usize, *to as usize);
                let c = self.b[from];
                let victim = self.b[to];
                if victim != 0 {
                    self.pts[self.side.idx()] += VAL[kind(victim) as usize];
                }
                self.b[to] = c;
                self.b[from] = 0;
                if kind(c) == PAWN {
                    let (dr, df) = FWD[army(c)];
                    let (r, f) = ((to / 8) as i32, (to % 8) as i32);
                    let at_end = (dr == 1 && r == 7)
                        || (dr == -1 && r == 0)
                        || (df == 1 && f == 7)
                        || (df == -1 && f == 0);
                    if at_end {
                        if let Some(k) = self.promote_kind(army(c)) {
                            self.b[to] = ((army(c) << 3) as u8) | k;
                        }
                    }
                }
                self.dice.remove(0);
            }
        }
        if self.dice.is_empty() && !matches!(m, Move::Roll) {
            self.side = self.side.other();
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        let (k0, k1) = (self.kings(0), self.kings(1));
        if k0 == 0 && k1 > 0 {
            return Some(Outcome::Win(Player::Two));
        }
        if k1 == 0 && k0 > 0 {
            return Some(Outcome::Win(Player::One));
        }
        if self.plies >= 400 || (k0 == 0 && k1 == 0) {
            return Some(by_score(self.pts[0], self.pts[1]));
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let o = p.other().idx();
        let me = p.idx();
        (self.pts[me] - self.pts[o]) * 100 + (self.material(me) - self.material(o)) * 10
    }

    fn render(&self) -> String {
        let mut s = String::from("Chaturaji — P pion, B barcă, H cal, E elefant, K rege; cifra = armata (1 Sud, 3 Nord = J1; 2 Est, 4 Vest = J2)\n");
        s.push_str("    a  b  c  d  e  f  g  h\n");
        for r in (0..8).rev() {
            s.push_str(&format!("{}  ", r + 1));
            for f in 0..8 {
                let c = self.b[r * 8 + f];
                if c == 0 {
                    s.push_str(" . ");
                } else {
                    let ch = ['?', 'P', 'B', 'H', 'E', 'K'][kind(c) as usize];
                    s.push_str(&format!("{ch}{} ", army(c) + 1));
                }
            }
            s.push_str(&format!(" {}\n", r + 1));
        }
        s.push_str(&format!(
            "Puncte capturate: J1 {} – J2 {}. Regi: J1 {}, J2 {}. Semi-mutări {}/400.\n",
            self.pts[0],
            self.pts[1],
            self.kings(0),
            self.kings(1),
            self.plies
        ));
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        let v = verdict(self.outcome());
        if v.is_empty() {
            if self.dice.is_empty() {
                s.push_str(&format!(
                    "La mutare: Jucător {} — arunc\n",
                    self.side.idx() + 1
                ));
            } else {
                s.push_str(&format!(
                    "La mutare: Jucător {}, zaruri rămase {:?} (acum {})\n",
                    self.side.idx() + 1,
                    self.dice,
                    self.dice[0]
                ));
            }
        } else {
            s.push_str(&v);
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Roll => "arunc".into(),
            Move::Skip => "pas".into(),
            Move::Go(a, b) => format!("{}{}", sq(*a as usize), sq(*b as usize)),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q5_util::testkit::check_game;

    #[test]
    fn setup_has_four_armies() {
        let g = Chaturaji::with_seed(1);
        assert_eq!(g.b.iter().filter(|&&c| c != 0).count(), 32);
        assert_eq!(g.kings(0), 2);
        assert_eq!(g.kings(1), 2);
    }

    #[test]
    fn dice_select_piece_types_and_boat_jumps_diagonally() {
        let mut g = Chaturaji::with_seed(1);
        g.apply(&Move::Roll);
        g.dice = vec![2, 6];
        let m = g.legal_moves();
        // barca Sud din a1 sare la c3 (liber)
        assert!(m.contains(&Move::Go(0, 18)), "{m:?}");
        assert!(m.contains(&Move::Skip));
        assert!(m.iter().all(|x| match x {
            Move::Go(a, _) => kind(g.b[*a as usize]) == BOAT,
            _ => true,
        }));
        g.apply(&Move::Skip);
        assert_eq!(g.side, Player::One);
        g.apply(&Move::Skip);
        assert_eq!(g.side, Player::Two);
    }

    #[test]
    fn capture_scores_points() {
        let mut g = Chaturaji::with_seed(1);
        g.b = [0; 64];
        g.b[0] = ELEPH; // elefant Sud (armata 0) pe a1
        g.b[24] = ((1usize << 3) as u8) | HORSE; // cal Est pe a4
        g.dice = vec![4];
        g.apply(&Move::Go(0, 24));
        assert_eq!(g.pts[0], 3);
    }

    #[test]
    fn random_and_ai_games() {
        check_game(Chaturaji::with_seed, 3, 3000);
    }
}
