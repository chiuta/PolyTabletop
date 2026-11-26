//! Sholo Guti (Șaisprezece soldați; Bangladesh / India): 16 piese pe jucător pe o tablă
//! de 37 de puncte.
//!
//! Tabla (reconstituită din descrierile publice; vezi nota despre limite): o grilă 5 × 5
//! (puncte `a1`–`e5`, rândurile ortogonale + diagonalele prin punctele „pare”, ca la
//! Alquerque) la care se atașează, pe laturile de sus și de jos, câte un triunghi de
//! 6 puncte (`t1`–`t6` sus, `u1`–`u6` jos: baza `1,2,3`, mijlocul `4,5`, vârful `6`).
//! Coloanele b, c, d se prelungesc în triunghiuri.
//!
//! Reguli: Jucătorul 1 ocupă rândurile 1–2 ale grilei și triunghiul de jos, Jucătorul 2
//! rândurile 4–5 și triunghiul de sus; rândul 3 e gol. Se mută un pas pe un punct
//! vecin liber, sau se sare peste o piesă adversă vecină pe punctul liber de după ea
//! (pe aceeași linie), capturând-o. Captura e obligatorie dacă există; după un salt poți
//! continua cu aceeași piesă (`opreste` pentru a te opri). Câștigă cine captează toate
//! piesele adversarului sau îl lasă fără mutări; remiză după 100 de mutări fără captură.
//!
//! Limită: geometria exactă a triunghiurilor nu a putut fi verificată din surse în
//! această sesiune; s-a folosit reconstituirea de mai sus (37 de puncte, 16 + 16 piese,
//! rândul mijlociu gol).

use crate::linegraph::LineGraph;
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::sync::OnceLock;

const N: usize = 37;
const QUIET_LIMIT: u32 = 100;

fn graph() -> &'static LineGraph {
    static G: OnceLock<LineGraph> = OnceLock::new();
    G.get_or_init(|| {
        let mut pos: Vec<(i32, i32)> = Vec::new();
        for y in 0..5 {
            for x in 0..5 {
                pos.push((2 * x, 2 * y));
            }
        }
        // 25..30 jos, 31..36 sus
        pos.extend([(2, -2), (4, -2), (6, -2), (3, -4), (5, -4), (4, -6)]);
        pos.extend([(2, 10), (4, 10), (6, 10), (3, 12), (5, 12), (4, 14)]);
        let mut lines: Vec<Vec<usize>> = Vec::new();
        for y in 0..5usize {
            lines.push((0..5).map(|x| y * 5 + x).collect());
        }
        lines.push(vec![0, 5, 10, 15, 20]);
        lines.push(vec![4, 9, 14, 19, 24]);
        lines.push(vec![25, 1, 6, 11, 16, 21, 31]);
        lines.push(vec![27, 3, 8, 13, 18, 23, 33]);
        lines.push(vec![30, 26, 2, 7, 12, 17, 22, 32, 36]);
        lines.push(vec![25, 26, 27]);
        lines.push(vec![28, 29]);
        lines.push(vec![25, 28, 30]);
        lines.push(vec![27, 29, 30]);
        lines.push(vec![31, 32, 33]);
        lines.push(vec![34, 35]);
        lines.push(vec![31, 34, 36]);
        lines.push(vec![33, 35, 36]);
        // diagonale prin punctele cu x + y par
        for d in [-2i32, 0, 2] {
            let l: Vec<usize> = (0..5i32)
                .filter(|&x| (0..5).contains(&(x - d)))
                .map(|x| ((x - d) * 5 + x) as usize)
                .collect();
            lines.push(l);
        }
        for s in [2i32, 4, 6] {
            let l: Vec<usize> = (0..5i32)
                .filter(|&x| (0..5).contains(&(s - x)))
                .map(|x| ((s - x) * 5 + x) as usize)
                .collect();
            lines.push(l);
        }
        LineGraph::new(pos, &lines)
    })
}

fn name(i: usize) -> String {
    match i {
        0..=24 => format!("{}{}", (b'a' + (i % 5) as u8) as char, i / 5 + 1),
        25..=30 => format!("u{}", i - 24),
        _ => format!("t{}", i - 30),
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Step(u8, u8),
    Jump(u8, u8),
    Stop,
}

#[derive(Clone)]
pub struct SholoGuti {
    b: [u8; N],
    side: Player,
    chain: Option<u8>,
    quiet: u32,
}

impl Default for SholoGuti {
    fn default() -> Self {
        Self::new()
    }
}

impl SholoGuti {
    pub fn new() -> Self {
        let mut b = [0u8; N];
        for (i, v) in b.iter_mut().enumerate() {
            *v = match i {
                0..=9 | 25..=30 => 1,
                15..=24 | 31..=36 => 2,
                _ => 0,
            };
        }
        SholoGuti {
            b,
            side: Player::One,
            chain: None,
            quiet: 0,
        }
    }

    fn tag(p: Player) -> u8 {
        p.idx() as u8 + 1
    }

    pub fn pieces(&self, p: Player) -> usize {
        self.b.iter().filter(|&&v| v == Self::tag(p)).count()
    }

    fn jumps_of(&self, i: usize, me: Player) -> Vec<Move> {
        let en = Self::tag(me.other());
        graph().jumps[i]
            .iter()
            .filter(|&&(over, to)| self.b[over] == en && self.b[to] == 0)
            .map(|&(over, to)| {
                let _ = over;
                Move::Jump(i as u8, to as u8)
            })
            .collect()
    }

    fn over_of(a: u8, b: u8) -> usize {
        graph().jumps[a as usize]
            .iter()
            .find(|&&(_, to)| to == b as usize)
            .map(|&(o, _)| o)
            .expect("jump")
    }

    fn all_jumps(&self, me: Player) -> Vec<Move> {
        let tag = Self::tag(me);
        (0..N)
            .filter(|&i| self.b[i] == tag)
            .flat_map(|i| self.jumps_of(i, me))
            .collect()
    }
}

impl Game for SholoGuti {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        let me = self.side;
        if let Some(i) = self.chain {
            let mut v = self.jumps_of(i as usize, me);
            v.push(Move::Stop);
            return v;
        }
        let j = self.all_jumps(me);
        if !j.is_empty() {
            return j;
        }
        let tag = Self::tag(me);
        let mut v = Vec::new();
        for i in 0..N {
            if self.b[i] == tag {
                for &k in &graph().adj[i] {
                    if self.b[k] == 0 {
                        v.push(Move::Step(i as u8, k as u8));
                    }
                }
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        let me = self.side;
        let tag = Self::tag(me);
        match *m {
            Move::Step(a, b) => {
                self.b[a as usize] = 0;
                self.b[b as usize] = tag;
                self.quiet += 1;
                self.side = me.other();
            }
            Move::Jump(a, b) => {
                let over = Self::over_of(a, b);
                self.b[a as usize] = 0;
                self.b[over] = 0;
                self.b[b as usize] = tag;
                self.quiet = 0;
                if self.pieces(me.other()) == 0 {
                    self.chain = None;
                } else if self.jumps_of(b as usize, me).is_empty() {
                    self.chain = None;
                    self.side = me.other();
                } else {
                    self.chain = Some(b);
                }
            }
            Move::Stop => {
                self.chain = None;
                self.side = me.other();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        let (a, b) = (self.pieces(Player::One), self.pieces(Player::Two));
        if b == 0 {
            return Some(Outcome::Win(Player::One));
        }
        if a == 0 {
            return Some(Outcome::Win(Player::Two));
        }
        if self.quiet >= QUIET_LIMIT {
            return Some(Outcome::Draw);
        }
        if self.chain.is_none() {
            let me = self.side;
            let tag = Self::tag(me);
            let can = (0..N).any(|i| {
                self.b[i] == tag
                    && (graph().adj[i].iter().any(|&k| self.b[k] == 0)
                        || !self.jumps_of(i, me).is_empty())
            });
            if !can {
                return Some(Outcome::Win(me.other()));
            }
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let o = p.other();
        let mut s = 100 * (self.pieces(p) as i32 - self.pieces(o) as i32);
        // piesele centrale sunt mai greu de prins
        for i in 0..25 {
            let (x, y) = ((i % 5) as i32, (i / 5) as i32);
            let c = 4 - ((x - 2).abs() + (y - 2).abs());
            if self.b[i] == Self::tag(p) {
                s += c;
            } else if self.b[i] == Self::tag(o) {
                s -= c;
            }
        }
        s
    }

    fn render(&self) -> String {
        let mut s = graph().draw(|i| match self.b[i] {
            1 => "X ".into(),
            2 => "O ".into(),
            _ => name(i),
        });
        s.push_str(&format!(
            "X = J1 ({}), O = J2 ({}); coloane a–e, rânduri 1–5; u1–u6 jos, t1–t6 sus.\n",
            self.pieces(Player::One),
            self.pieces(Player::Two)
        ));
        if self.chain.is_some() {
            s.push_str("Poți continua săritura sau scrie «opreste».\n");
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match *m {
            Move::Step(a, b) => format!("{}-{}", name(a as usize), name(b as usize)),
            Move::Jump(a, b) => format!("{}x{}", name(a as usize), name(b as usize)),
            Move::Stop => "opreste".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    #[test]
    fn board_has_37_points_and_32_pieces() {
        let g = SholoGuti::new();
        assert_eq!(graph().len(), 37);
        assert_eq!(g.pieces(Player::One), 16);
        assert_eq!(g.pieces(Player::Two), 16);
        assert_eq!(g.b.iter().filter(|&&v| v == 0).count(), 5);
    }

    #[test]
    fn start_has_only_steps_into_the_middle_row() {
        let g = SholoGuti::new();
        let mv = Game::legal_moves(&g);
        assert!(!mv.is_empty());
        for m in mv {
            match m {
                Move::Step(_, to) => assert!((10..15).contains(&(to as usize))),
                _ => panic!("fără salturi la start"),
            }
        }
    }

    #[test]
    fn jump_is_forced_and_captures() {
        let mut g = SholoGuti::new();
        g.b = [0; N];
        g.b[2] = 1; // c1
        g.b[7] = 2; // c2
        g.b[24] = 2; // e5 undeva
        let mv = Game::legal_moves(&g);
        assert_eq!(mv, vec![Move::Jump(2, 12)]);
        g.apply(&mv[0]);
        assert_eq!(g.b[7], 0);
        assert_eq!(g.b[12], 1);
    }

    #[test]
    fn diagonals_only_through_even_points() {
        let a1 = 0;
        let g = graph();
        assert!(g.adj[a1].contains(&6)); // a1-b2
        assert!(!g.adj[1].contains(&5)); // b1-a2: punct impar
    }

    #[test]
    fn chain_continues_or_stops() {
        let mut g = SholoGuti::new();
        g.b = [0; N];
        g.b[10] = 1; // a3
        g.b[11] = 2; // b3
        g.b[13] = 2; // d3 : după a3xc3, din c3 se poate sări peste d3 spre e3
        g.b[24] = 2;
        g.apply(&Move::Jump(10, 12));
        assert_eq!(g.to_move(), Player::One);
        assert!(Game::legal_moves(&g).contains(&Move::Stop));
        g.apply(&Move::Jump(12, 14));
        assert_eq!(g.to_move(), Player::Two);
    }

    #[test]
    fn random_games_terminate() {
        for seed in 1..=20u64 {
            let mut g = SholoGuti::new();
            let mut rng = Rng::new(seed);
            let mut k = 0;
            while g.outcome().is_none() {
                let m = Game::legal_moves(&g);
                assert!(!m.is_empty(), "{}", g.render());
                g.apply(&m[rng.below(m.len())]);
                k += 1;
                assert!(k < 3000);
            }
        }
    }
}
