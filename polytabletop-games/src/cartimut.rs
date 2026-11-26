//! Mutări din cărți (variantă proprie): tablă 5×5, fiecare jucător are patru soldați și un rege
//! pe rândul lui. Cinci cărți de mutare (din cele 16 ale jocului, alese la întâmplare la
//! începutul partidei): două la fiecare jucător și una la mijloc. Muți o piesă cu o deplasare
//! de pe una din cărțile tale, apoi cartea folosită trece la mijloc, iar cartea de la mijloc
//! intră în mâna ta. Deplasările sunt scrise pentru J1 (înainte = în sus); J2 le vede oglindit.
//! Câștigi dacă iei regele adversarului sau dacă regele tău ajunge pe locul de start al
//! regelui lui (c1 pentru J2, c5 pentru J1). Remiză după 120 de semi-mutări.
//! Mutări: `c1-c2/cal` (piesă de pe c1 pe c2 cu cartea „cal”).
#![allow(clippy::all)]

use crate::b1_util::{grid, hkey, sq};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const CARDS: [(&str, &[(i32, i32)]); 16] = [
    ("cal", &[(1, 2), (-1, 2), (0, -1)]),
    ("arc", &[(0, 1), (2, 0), (-2, 0)]),
    ("bat", &[(1, 1), (-1, 1), (0, -1)]),
    ("cui", &[(1, 0), (-1, 0), (0, 2)]),
    ("dig", &[(0, 1), (1, -1), (-1, -1)]),
    ("fus", &[(0, 2), (0, -1)]),
    ("gar", &[(1, 1), (-1, 1), (1, -1), (-1, -1)]),
    ("hat", &[(2, 1), (-2, 1), (0, -1)]),
    ("lan", &[(0, 1), (0, -1), (1, 0), (-1, 0)]),
    ("mar", &[(1, 2), (-1, 2)]),
    ("nor", &[(1, 1), (-1, 0), (0, -2)]),
    ("ora", &[(-1, 1), (1, 0), (0, -2)]),
    ("pod", &[(0, 1), (1, 1), (-1, 1)]),
    ("raz", &[(2, 0), (-2, 0), (0, 1), (0, -1)]),
    ("sul", &[(0, 1), (0, 2), (0, -1)]),
    ("val", &[(1, 1), (-1, 1), (2, 0), (-2, 0)]),
];

#[derive(Clone)]
pub struct Cartimut {
    b: [u8; 25], // 0 gol, 1/2 soldat, 3/4 rege J1/J2
    hand: [[u8; 2]; 2],
    mid: u8,
    side: Player,
    win: Option<Player>,
    ply: u32,
}

impl Cartimut {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck: Vec<u8> = (0..16).collect();
        for i in (1..deck.len()).rev() {
            deck.swap(i, rng.below(i + 1));
        }
        let mut b = [0u8; 25];
        for x in 0..5 {
            b[x] = if x == 2 { 3 } else { 1 };
            b[20 + x] = if x == 2 { 4 } else { 2 };
        }
        Cartimut {
            b,
            hand: [[deck[0], deck[1]], [deck[2], deck[3]]],
            mid: deck[4],
            side: Player::One,
            win: None,
            ply: 0,
        }
    }
}

impl Default for Cartimut {
    fn default() -> Self {
        Self::new()
    }
}

impl Game for Cartimut {
    type Move = (u8, u8, u8); // cartea (0/1 din mână), de la, la
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<(u8, u8, u8)> {
        if self.win.is_some() || self.ply >= 120 {
            return vec![];
        }
        let p = self.side.idx();
        let (mine, king) = if p == 0 { (1u8, 3u8) } else { (2, 4) };
        let sgn = if p == 0 { 1 } else { -1 };
        let mut v = vec![];
        for (ci, &card) in self.hand[p].iter().enumerate() {
            for i in 0..25 {
                if self.b[i] != mine && self.b[i] != king {
                    continue;
                }
                for &(dx, dy) in CARDS[card as usize].1 {
                    let (nx, ny) = ((i % 5) as i32 + dx * sgn, (i / 5) as i32 + dy * sgn);
                    if nx < 0 || ny < 0 || nx >= 5 || ny >= 5 {
                        continue;
                    }
                    let t = (ny * 5 + nx) as usize;
                    if self.b[t] == mine || self.b[t] == king {
                        continue;
                    }
                    v.push((ci as u8, i as u8, t as u8));
                }
            }
        }
        v
    }
    fn apply(&mut self, m: &(u8, u8, u8)) {
        let p = self.side.idx();
        let piece = self.b[m.1 as usize];
        let taken = self.b[m.2 as usize];
        self.b[m.1 as usize] = 0;
        self.b[m.2 as usize] = piece;
        let used = self.hand[p][m.0 as usize];
        self.hand[p][m.0 as usize] = self.mid;
        self.mid = used;
        let enemy_king = if p == 0 { 4 } else { 3 };
        let throne = if p == 0 { 22 } else { 2 };
        if taken == enemy_king || (piece == (if p == 0 { 3 } else { 4 }) && m.2 as usize == throne)
        {
            self.win = Some(self.side);
        }
        self.ply += 1;
        self.side = self.side.other();
    }
    fn outcome(&self) -> Option<Outcome> {
        match self.win {
            Some(w) => Some(Outcome::Win(w)),
            None if self.ply >= 120 => Some(Outcome::Draw),
            None => None,
        }
    }
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut s = 0;
        for i in 0..25 {
            let v = self.b[i];
            let (owner, king) = match v {
                1 => (0, false),
                2 => (1, false),
                3 => (0, true),
                4 => (1, true),
                _ => continue,
            };
            let val = if king { 0 } else { 100 }
                + if owner == 0 {
                    (i / 5) as i32
                } else {
                    4 - (i / 5) as i32
                } * 4;
            s += if owner == me { val } else { -val };
        }
        let kings = [self.b.contains(&3), self.b.contains(&4)];
        if !kings[me] {
            s -= 5000;
        }
        if !kings[1 - me] {
            s += 5000;
        }
        s
    }
    fn render(&self) -> String {
        let mut s = grid(5, 5, |x, y| match self.b[y * 5 + x] {
            1 => 'x',
            2 => 'o',
            3 => 'X',
            4 => 'O',
            _ => '.',
        });
        let desc = |c: u8| {
            let (n, o) = CARDS[c as usize];
            format!(
                "{}[{}]",
                n,
                o.iter()
                    .map(|&(dx, dy)| format!("{dx:+},{dy:+}"))
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        };
        for p in 0..2 {
            s.push_str(&format!(
                "J{}: {} | {}\n",
                p + 1,
                desc(self.hand[p][0]),
                desc(self.hand[p][1])
            ));
        }
        s.push_str(&format!(
            "Mijloc: {}  (deplasări (dx,dy) pentru J1; J2 le oglindește)\n",
            desc(self.mid)
        ));
        s
    }
    fn format_move(&self, m: &(u8, u8, u8)) -> String {
        let c = self.hand[self.side.idx()][m.0 as usize];
        format!(
            "{}-{}/{}",
            sq(m.1 as usize % 5, m.1 as usize / 5),
            sq(m.2 as usize % 5, m.2 as usize / 5),
            CARDS[c as usize].0
        )
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 300,
            max_depth: 8,
        }
    }
    fn hash_key(&self) -> Option<u64> {
        let mut h = hkey(
            20,
            self.side,
            self.b.iter().enumerate().map(|(i, &v)| (i, v as u64)),
        );
        for p in 0..2 {
            let (a, b) = (
                self.hand[p][0].min(self.hand[p][1]),
                self.hand[p][0].max(self.hand[p][1]),
            );
            h ^= polytabletop_core::hash_mix(1000 + p as u64 * 400 + a as u64 * 17 + b as u64);
        }
        Some(h ^ polytabletop_core::hash_mix(9000 + self.mid as u64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::b1_util::testing::check;
    #[test]
    fn deal_and_swap() {
        let g = Cartimut::with_seed(3);
        let mut all: Vec<u8> = vec![
            g.hand[0][0],
            g.hand[0][1],
            g.hand[1][0],
            g.hand[1][1],
            g.mid,
        ];
        all.sort();
        all.dedup();
        assert_eq!(all.len(), 5);
        let mut h = g.clone();
        let m = h.legal_moves()[0];
        let used = h.hand[0][m.0 as usize];
        let mid = h.mid;
        h.apply(&m);
        assert_eq!(h.mid, used);
        assert_eq!(h.hand[0][m.0 as usize], mid);
    }
    #[test]
    fn king_capture_wins() {
        let mut g = Cartimut::with_seed(1);
        g.b = [0; 25];
        g.b[10] = 3;
        g.b[15] = 4;
        g.hand[0] = [8, 8]; // „lan” are (0,1): înainte
        g.hand[0][1] = 8;
        // king a3 (10) la d? (0,1) duce la 15 = regele advers
        let mv = g.legal_moves();
        let m = mv.iter().find(|m| m.1 == 10 && m.2 == 15).unwrap();
        g.apply(m);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }
    #[test]
    fn random_and_ai() {
        check(Cartimut::with_seed, 200);
    }
}
