//! Yote (Senegal / Africa de Vest): tablă 5 × 6, 12 piese pe jucător, inițial în mână.
//!
//! Reguli folosite (descrierile publice ale jocului):
//! * La rândul tău faci O acțiune: (a) pui o piesă din mână pe un pătrat liber;
//!   (b) muți o piesă proprie un pas ortogonal pe un pătrat liber; (c) sari ortogonal
//!   peste o piesă adversă adiacentă, pe pătratul liber de după ea, o captezi și
//!   în plus scoți încă o piesă adversă la alegere (de pe tablă; dacă adversarul nu
//!   mai are alta pe tablă, din mâna lui).
//! * După o sărituri poți continua cu aceeași piesă (alt salt, cu captura lui) sau te
//!   oprești (`opreste`).
//! * Câștigă cine rămâne singur cu piese (adversarul n-are nici pe tablă, nici în mână).
//!   După 100 de mutări fără captură câștigă cine are mai multe piese (egal = remiză).
//!
//! Notație: plasare `c3`; pas `c3-c4`; salt `c3xc5+a1` (piesa luată în plus de pe a1),
//! `c3xc5+m` (din mâna adversarului), `c3xc5` (nu mai există piese de luat); `opreste`; `pas` (doar dacă n-ai nicio mutare).

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const W: usize = 6;
const H: usize = 5;
const N: usize = W * H;
const QUIET_LIMIT: u32 = 100;

fn name(i: usize) -> String {
    format!("{}{}", (b'a' + (i % W) as u8) as char, i / W + 1)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Take {
    None,
    Hand,
    Board(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Drop(u8),
    Step(u8, u8),
    Jump(u8, u8, Take),
    Stop,
    Pass,
}

#[derive(Clone)]
pub struct Yote {
    b: [u8; N],
    hand: [u8; 2],
    side: Player,
    chain: Option<u8>,
    quiet: u32,
}

impl Default for Yote {
    fn default() -> Self {
        Self::new()
    }
}

fn dirs() -> [(i32, i32); 4] {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
}

impl Yote {
    pub fn new() -> Self {
        Yote {
            b: [0; N],
            hand: [12, 12],
            side: Player::One,
            chain: None,
            quiet: 0,
        }
    }

    fn tag(p: Player) -> u8 {
        p.idx() as u8 + 1
    }

    pub fn pieces(&self, p: Player) -> usize {
        self.b.iter().filter(|&&c| c == Self::tag(p)).count() + self.hand[p.idx()] as usize
    }

    fn on_board(&self, p: Player) -> usize {
        self.b.iter().filter(|&&c| c == Self::tag(p)).count()
    }

    fn at(r: i32, c: i32) -> Option<usize> {
        if (0..H as i32).contains(&r) && (0..W as i32).contains(&c) {
            Some(r as usize * W + c as usize)
        } else {
            None
        }
    }

    fn jumps_from(&self, i: usize, me: Player) -> Vec<Move> {
        let mut out = Vec::new();
        let (r, c) = ((i / W) as i32, (i % W) as i32);
        let en = Self::tag(me.other());
        for (dr, dc) in dirs() {
            let (Some(over), Some(to)) =
                (Self::at(r + dr, c + dc), Self::at(r + 2 * dr, c + 2 * dc))
            else {
                continue;
            };
            if self.b[over] != en || self.b[to] != 0 {
                continue;
            }
            // piesele care pot fi luate în plus (altele decât cea sărită)
            let extra: Vec<usize> = (0..N).filter(|&k| self.b[k] == en && k != over).collect();
            if !extra.is_empty() {
                for k in extra {
                    out.push(Move::Jump(i as u8, to as u8, Take::Board(k as u8)));
                }
            } else if self.hand[me.other().idx()] > 0 {
                out.push(Move::Jump(i as u8, to as u8, Take::Hand));
            } else {
                out.push(Move::Jump(i as u8, to as u8, Take::None));
            }
        }
        out
    }

    fn over_of(from: u8, to: u8) -> usize {
        (from as usize + to as usize) / 2
    }
}

impl Game for Yote {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        let me = self.side;
        let tag = Self::tag(me);
        if let Some(i) = self.chain {
            let mut v = self.jumps_from(i as usize, me);
            v.push(Move::Stop);
            return v;
        }
        let mut v = Vec::new();
        for i in 0..N {
            if self.b[i] == tag {
                let (r, c) = ((i / W) as i32, (i % W) as i32);
                for (dr, dc) in dirs() {
                    if let Some(j) = Self::at(r + dr, c + dc) {
                        if self.b[j] == 0 {
                            v.push(Move::Step(i as u8, j as u8));
                        }
                    }
                }
                v.extend(self.jumps_from(i, me));
            }
        }
        if self.hand[me.idx()] > 0 {
            for i in 0..N {
                if self.b[i] == 0 {
                    v.push(Move::Drop(i as u8));
                }
            }
        }
        if v.is_empty() {
            v.push(Move::Pass);
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        let me = self.side;
        let tag = Self::tag(me);
        let opp = me.other();
        match *m {
            Move::Drop(i) => {
                self.b[i as usize] = tag;
                self.hand[me.idx()] -= 1;
                self.quiet += 1;
                self.side = opp;
            }
            Move::Step(a, b) => {
                self.b[a as usize] = 0;
                self.b[b as usize] = tag;
                self.quiet += 1;
                self.side = opp;
            }
            Move::Jump(a, b, take) => {
                let over = Self::over_of(a, b);
                self.b[a as usize] = 0;
                self.b[over] = 0;
                self.b[b as usize] = tag;
                match take {
                    Take::Board(k) => self.b[k as usize] = 0,
                    Take::Hand => self.hand[opp.idx()] -= 1,
                    Take::None => {}
                }
                self.quiet = 0;
                if self.pieces(opp) == 0 {
                    self.chain = None;
                    return;
                }
                if self.jumps_from(b as usize, me).is_empty() {
                    self.chain = None;
                    self.side = opp;
                } else {
                    self.chain = Some(b);
                }
            }
            Move::Stop => {
                self.chain = None;
                self.side = opp;
            }
            Move::Pass => {
                self.quiet += 1;
                self.side = opp;
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        let (a, b) = (self.pieces(Player::One), self.pieces(Player::Two));
        if b == 0 && a > 0 {
            return Some(Outcome::Win(Player::One));
        }
        if a == 0 && b > 0 {
            return Some(Outcome::Win(Player::Two));
        }
        if self.quiet >= QUIET_LIMIT {
            return Some(match a.cmp(&b) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                _ => Outcome::Draw,
            });
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let o = p.other();
        let mat = self.pieces(p) as i32 - self.pieces(o) as i32;
        let board = self.on_board(p) as i32 - self.on_board(o) as i32;
        100 * mat + 10 * board
    }

    fn render(&self) -> String {
        let mut s = String::new();
        for r in (0..H).rev() {
            s.push_str(&format!("{} ", r + 1));
            for c in 0..W {
                let ch = match self.b[r * W + c] {
                    1 => 'X',
                    2 => 'O',
                    _ => '.',
                };
                s.push(' ');
                s.push(ch);
            }
            s.push('\n');
        }
        s.push_str("   a b c d e f\n");
        s.push_str(&format!(
            "În mână: J1 {}, J2 {}; pe tablă: X {}, O {}\n",
            self.hand[0],
            self.hand[1],
            self.on_board(Player::One),
            self.on_board(Player::Two)
        ));
        if self.chain.is_some() {
            s.push_str("Poți continua săritura cu aceeași piesă sau scrie «opreste».\n");
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match *m {
            Move::Drop(i) => name(i as usize),
            Move::Step(a, b) => format!("{}-{}", name(a as usize), name(b as usize)),
            Move::Jump(a, b, t) => {
                let base = format!("{}x{}", name(a as usize), name(b as usize));
                match t {
                    Take::None => base,
                    Take::Hand => format!("{base}+m"),
                    Take::Board(k) => format!("{base}+{}", name(k as usize)),
                }
            }
            Move::Stop => "opreste".into(),
            Move::Pass => "pas".into(),
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

    fn n(s: &str) -> usize {
        let b = s.as_bytes();
        (b[1] - b'1') as usize * W + (b[0] - b'a') as usize
    }

    fn empty() -> Yote {
        Yote {
            b: [0; N],
            hand: [0, 0],
            side: Player::One,
            chain: None,
            quiet: 0,
        }
    }

    #[test]
    fn start_has_30_drops() {
        let g = Yote::new();
        assert_eq!(Game::legal_moves(&g).len(), 30);
        assert_eq!(g.pieces(Player::One), 12);
    }

    #[test]
    fn jump_captures_plus_extra_piece() {
        let mut g = empty();
        g.b[n("c3")] = 1;
        g.b[n("c4")] = 2;
        g.b[n("a1")] = 2;
        g.b[n("f5")] = 2;
        let mv = Game::legal_moves(&g);
        let jumps: Vec<_> = mv.iter().filter(|m| matches!(m, Move::Jump(..))).collect();
        assert_eq!(jumps.len(), 2); // a1 sau f5 ca piesă în plus
        g.apply(&Move::Jump(
            n("c3") as u8,
            n("c5") as u8,
            Take::Board(n("a1") as u8),
        ));
        assert_eq!(g.b[n("c4")], 0);
        assert_eq!(g.b[n("a1")], 0);
        assert_eq!(g.b[n("f5")], 2);
        assert_eq!(g.b[n("c5")], 1);
        assert_eq!(g.to_move(), Player::Two);
    }

    #[test]
    fn extra_piece_from_hand_when_board_has_none() {
        let mut g = empty();
        g.hand = [0, 2];
        g.b[n("c3")] = 1;
        g.b[n("c4")] = 2;
        let mv = Game::legal_moves(&g);
        assert!(mv.contains(&Move::Jump(n("c3") as u8, n("c5") as u8, Take::Hand)));
        g.apply(&Move::Jump(n("c3") as u8, n("c5") as u8, Take::Hand));
        assert_eq!(g.hand[1], 1);
    }

    #[test]
    fn chain_can_be_stopped() {
        let mut g = empty();
        g.hand = [0, 0];
        g.b[n("a1")] = 1;
        g.b[n("b1")] = 2;
        g.b[n("d1")] = 2;
        g.b[n("e2")] = 2;
        g.b[n("f5")] = 2;
        g.apply(&Move::Jump(
            n("a1") as u8,
            n("c1") as u8,
            Take::Board(n("f5") as u8),
        ));
        // din c1 se mai poate sări peste d1 spre e1
        assert_eq!(g.to_move(), Player::One);
        let mv = Game::legal_moves(&g);
        assert!(mv.contains(&Move::Stop));
        g.apply(&Move::Stop);
        assert_eq!(g.to_move(), Player::Two);
    }

    #[test]
    fn win_when_opponent_has_nothing() {
        let mut g = empty();
        g.b[n("c3")] = 1;
        g.b[n("c4")] = 2;
        g.apply(&Move::Jump(n("c3") as u8, n("c5") as u8, Take::None));
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn random_games_terminate() {
        for seed in 1..=30u64 {
            let mut g = Yote::new();
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
