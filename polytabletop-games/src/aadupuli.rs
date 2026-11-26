//! Aadu Puli Attam (India; „Oi și tigri”, Pulijudam): 3 tigri contra a 15 oi pe o tablă
//! de 23 de puncte în formă de triunghi cu axă verticală.
//!
//! Tabla (reconstituită din descrierile publice; vezi limitele): vârful (`v`), apoi șapte
//! trepte orizontale de câte trei puncte (stânga, mijloc, dreapta: `a1 b1 c1` … `a7 b7 c7`
//! de sus în jos), prima pe laturile care se depărtează, ultimele patru cu laturi
//! verticale; axa verticală se prelungește cu un punct de bază (`z`). Liniile sunt cele
//! două laturi, axa și treptele.
//!
//! Reguli: Jucătorul 1 = oile (mută primul), Jucătorul 2 = tigrii. Tigrii pornesc pe `v`,
//! `a1` și `c1`. Oile se plasează pe rând câte una pe un punct liber (15 în total); abia
//! după ce au fost plasate toate, oile se mută un pas pe un punct vecin liber. Tigrii se
//! mută un pas sau sar peste o oaie vecină pe punctul liber de după ea (pe aceeași linie),
//! capturând-o (captura nu e obligatorie, săritura se poate lanțui: `opreste`). Tigrii
//! câștigă cu 5 oi capturate; oile câștigă dacă toți tigrii sunt imobilizați. Remiză după
//! 120 de semi-mutări fără captură în faza de mutare.
//!
//! Limite: geometria exactă a tablei, numărul de oi necesar tigrilor (5) și punctul
//! suplimentar `z` sunt reconstituiri; nu au putut fi verificate din surse în această
//! sesiune.

use crate::linegraph::LineGraph;
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::sync::OnceLock;

const N: usize = 23;
const GOATS: u8 = 15;
const WIN_CAPTURES: u8 = 5;
const QUIET_LIMIT: u32 = 120;

fn graph() -> &'static LineGraph {
    static G: OnceLock<LineGraph> = OnceLock::new();
    G.get_or_init(|| {
        let xs: [[i32; 3]; 7] = [
            [3, 4, 5],
            [2, 4, 6],
            [1, 4, 7],
            [0, 4, 8],
            [0, 4, 8],
            [0, 4, 8],
            [0, 4, 8],
        ];
        let mut pos: Vec<(i32, i32)> = vec![(4, 14)];
        for (k, row) in xs.iter().enumerate() {
            for &x in row {
                pos.push((x, 12 - 2 * k as i32));
            }
        }
        pos.push((4, -2));
        let idx = |row: usize, col: usize| 1 + row * 3 + col;
        let mut lines: Vec<Vec<usize>> = Vec::new();
        for r in 0..7 {
            lines.push(vec![idx(r, 0), idx(r, 1), idx(r, 2)]);
        }
        let mut axis = vec![0];
        let mut left = vec![0];
        let mut right = vec![0];
        for r in 0..7 {
            axis.push(idx(r, 1));
            left.push(idx(r, 0));
            right.push(idx(r, 2));
        }
        axis.push(22);
        lines.push(axis);
        lines.push(left);
        lines.push(right);
        LineGraph::new(pos, &lines)
    })
}

fn name(i: usize) -> String {
    match i {
        0 => "v".into(),
        22 => "z".into(),
        _ => {
            let r = (i - 1) / 3;
            let c = (i - 1) % 3;
            format!("{}{}", (b'a' + c as u8) as char, r + 1)
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Place(u8),
    Step(u8, u8),
    Jump(u8, u8),
    Stop,
}

#[derive(Clone)]
pub struct AaduPuli {
    b: [u8; N], // 1 oaie, 2 tigru
    hand: u8,
    taken: u8,
    side: Player,
    chain: Option<u8>,
    quiet: u32,
}

impl Default for AaduPuli {
    fn default() -> Self {
        Self::new()
    }
}

impl AaduPuli {
    pub fn new() -> Self {
        let mut b = [0u8; N];
        b[0] = 2;
        b[1] = 2;
        b[3] = 2;
        AaduPuli {
            b,
            hand: GOATS,
            taken: 0,
            side: Player::One,
            chain: None,
            quiet: 0,
        }
    }

    fn goat_jumps(&self, i: usize) -> Vec<Move> {
        graph().jumps[i]
            .iter()
            .filter(|&&(over, to)| self.b[over] == 1 && self.b[to] == 0)
            .map(|&(_, to)| Move::Jump(i as u8, to as u8))
            .collect()
    }

    fn tiger_moves(&self, i: usize) -> Vec<Move> {
        let mut v = self.goat_jumps(i);
        for &k in &graph().adj[i] {
            if self.b[k] == 0 {
                v.push(Move::Step(i as u8, k as u8));
            }
        }
        v
    }

    fn tigers_stuck(&self) -> bool {
        (0..N).all(|i| self.b[i] != 2 || self.tiger_moves(i).is_empty())
    }

    fn goat_moves(&self) -> Vec<Move> {
        let mut v = Vec::new();
        for i in 0..N {
            if self.b[i] == 1 {
                for &k in &graph().adj[i] {
                    if self.b[k] == 0 {
                        v.push(Move::Step(i as u8, k as u8));
                    }
                }
            }
        }
        v
    }

    fn over_of(a: u8, b: u8) -> usize {
        graph().jumps[a as usize]
            .iter()
            .find(|&&(_, to)| to == b as usize)
            .map(|&(o, _)| o)
            .expect("jump")
    }
}

impl Game for AaduPuli {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        if self.side == Player::One {
            if self.hand > 0 {
                return (0..N)
                    .filter(|&i| self.b[i] == 0)
                    .map(|i| Move::Place(i as u8))
                    .collect();
            }
            return self.goat_moves();
        }
        if let Some(i) = self.chain {
            let mut v = self.goat_jumps(i as usize);
            v.push(Move::Stop);
            return v;
        }
        (0..N)
            .filter(|&i| self.b[i] == 2)
            .flat_map(|i| self.tiger_moves(i))
            .collect()
    }

    fn apply(&mut self, m: &Move) {
        match *m {
            Move::Place(i) => {
                self.b[i as usize] = 1;
                self.hand -= 1;
                self.side = Player::Two;
            }
            Move::Step(a, b) => {
                let t = self.b[a as usize];
                self.b[a as usize] = 0;
                self.b[b as usize] = t;
                if self.hand == 0 {
                    self.quiet += 1;
                }
                self.side = self.side.other();
            }
            Move::Jump(a, b) => {
                let over = Self::over_of(a, b);
                self.b[a as usize] = 0;
                self.b[over] = 0;
                self.b[b as usize] = 2;
                self.taken += 1;
                self.quiet = 0;
                if self.taken >= WIN_CAPTURES {
                    return;
                }
                if self.goat_jumps(b as usize).is_empty() {
                    self.chain = None;
                    self.side = Player::One;
                } else {
                    self.chain = Some(b);
                }
            }
            Move::Stop => {
                self.chain = None;
                self.side = Player::One;
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.taken >= WIN_CAPTURES {
            return Some(Outcome::Win(Player::Two));
        }
        if self.chain.is_none() && self.tigers_stuck() && self.side == Player::Two {
            return Some(Outcome::Win(Player::One));
        }
        if self.side == Player::One && self.hand == 0 && self.goat_moves().is_empty() {
            return Some(Outcome::Win(Player::Two));
        }
        if self.quiet >= QUIET_LIMIT {
            return Some(Outcome::Draw);
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let mobility: i32 = (0..N)
            .filter(|&i| self.b[i] == 2)
            .map(|i| self.tiger_moves(i).len() as i32)
            .sum();
        let threat: i32 = (0..N)
            .filter(|&i| self.b[i] == 2)
            .map(|i| self.goat_jumps(i).len() as i32)
            .sum();
        // + pentru tigri
        let t = 100 * self.taken as i32 + 3 * mobility + 15 * threat;
        if p == Player::Two {
            t
        } else {
            -t
        }
    }

    fn render(&self) -> String {
        let mut s = graph().draw(|i| match self.b[i] {
            1 => "G ".into(),
            2 => "T ".into(),
            _ => name(i),
        });
        s.push_str(&format!(
            "G = oi (Jucătorul 1), T = tigri (Jucătorul 2). În mână: {}  Capturate: {}/{}\n",
            self.hand, self.taken, WIN_CAPTURES
        ));
        if self.chain.is_some() {
            s.push_str("Tigrul poate continua săritura sau scrie «opreste».\n");
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match *m {
            Move::Place(i) => name(i as usize),
            Move::Step(a, b) => format!("{}-{}", name(a as usize), name(b as usize)),
            Move::Jump(a, b) => format!("{}x{}", name(a as usize), name(b as usize)),
            Move::Stop => "opreste".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    #[test]
    fn board_has_23_points() {
        assert_eq!(graph().len(), 23);
        let g = AaduPuli::new();
        assert_eq!(g.b.iter().filter(|&&v| v == 2).count(), 3);
        assert_eq!(Game::legal_moves(&g).len(), 20);
    }

    #[test]
    fn goats_place_first_then_move() {
        let mut g = AaduPuli::new();
        g.apply(&Move::Place(10));
        assert_eq!(g.to_move(), Player::Two);
        assert_eq!(g.hand, 14);
        // oile nu se mută cât timp au piese în mână
        g.apply(&Game::legal_moves(&g)[0].clone());
        for m in Game::legal_moves(&g) {
            assert!(matches!(m, Move::Place(_)));
        }
    }

    #[test]
    fn tiger_jump_captures_a_goat() {
        let mut g = AaduPuli::new();
        g.b = [0; N];
        g.hand = 0;
        g.b[0] = 2; // v
        g.b[2] = 1; // b1 pe axă
        g.b[5] = 0; // b2 gol
        g.b[22] = 1;
        g.side = Player::Two;
        let mv = Game::legal_moves(&g);
        assert!(mv.contains(&Move::Jump(0, 5)));
        g.apply(&Move::Jump(0, 5));
        assert_eq!(g.taken, 1);
        assert_eq!(g.b[2], 0);
        assert_eq!(g.to_move(), Player::One);
    }

    #[test]
    fn immobilised_tigers_lose() {
        let mut g = AaduPuli::new();
        g.b = [0; N];
        g.hand = 0;
        g.b[0] = 2;
        g.b[1] = 1;
        g.b[2] = 1;
        g.b[3] = 1;
        g.b[4] = 1; // a2 blochează săritura peste a1
        g.b[5] = 1; // b2 blochează săritura peste b1
        g.b[6] = 1; // c2 blochează săritura peste c1
        g.side = Player::Two;
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn random_games_terminate() {
        for seed in 1..=30u64 {
            let mut g = AaduPuli::new();
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
