//! Havannah (Marea Britanie), 2 jucători, din regulile publice.
//!
//! * Tablă hexagonală cu latura de 8 hexagoane (169 de câmpuri; varianta „baza 8”, uzuală online).
//! * Se pun pietre pe rând, J1 (X) începe. Regula plăcintei: după prima piatră a lui J1, J2 poate
//!   spune `swap` (preia piatra; J1 mută din nou).
//! * Câștigă primul care formează: un **inel** (lanț închis ce înconjoară cel puțin un câmp), un
//!   **pod** (un lanț care atinge două colțuri) sau o **furcă** (un lanț care atinge trei laturi
//!   diferite; colțurile nu sunt laturi). Tabla plină fără câștigător = remiză.
//!
//! Mutări: `h8` (literă = coloană a–o, număr = rând 1–15 în tabla rombică), `swap`.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const R: i32 = 7;
const W: usize = 15;

fn valid(q: i32, r: i32) -> bool {
    q.abs() <= R && r.abs() <= R && (q + r).abs() <= R
}
fn idx(q: i32, r: i32) -> usize {
    ((r + R) as usize) * W + (q + R) as usize
}
const DIRS: [(i32, i32); 6] = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, -1), (-1, 1)];

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Move {
    Place(u8, u8),
    Swap,
}

#[derive(Clone)]
pub struct Havannah {
    cells: Vec<u8>,
    side: Player,
    count: u16,
    winner: Option<Outcome>,
    first: Option<(i32, i32)>,
    how: &'static str,
}

impl Default for Havannah {
    fn default() -> Self {
        Self::new()
    }
}

fn corner_id(q: i32, r: i32) -> Option<usize> {
    let c = [(7, 0), (7, -7), (0, -7), (-7, 0), (-7, 7), (0, 7)];
    c.iter().position(|&x| x == (q, r))
}

/// Laturile (0..6) pe care se află câmpul, `None` dacă e colț sau interior.
fn side_id(q: i32, r: i32) -> Option<usize> {
    if corner_id(q, r).is_some() {
        return None;
    }
    let s = q + r;
    if q == 7 {
        Some(0)
    } else if q == -7 {
        Some(1)
    } else if r == 7 {
        Some(2)
    } else if r == -7 {
        Some(3)
    } else if s == 7 {
        Some(4)
    } else if s == -7 {
        Some(5)
    } else {
        None
    }
}

impl Havannah {
    pub fn new() -> Self {
        Havannah {
            cells: vec![0; W * W],
            side: Player::One,
            count: 0,
            winner: None,
            first: None,
            how: "",
        }
    }

    pub fn with_seed(_seed: u64) -> Self {
        Self::new()
    }

    fn name(q: i32, r: i32) -> String {
        format!("{}{}", (b'a' + (q + R) as u8) as char, r + R + 1)
    }

    fn group_wins(&self, q0: i32, r0: i32, me: u8) -> Option<&'static str> {
        let mut seen = vec![false; W * W];
        let mut st = vec![(q0, r0)];
        seen[idx(q0, r0)] = true;
        let mut corners = [false; 6];
        let mut sides = [false; 6];
        while let Some((q, r)) = st.pop() {
            if let Some(c) = corner_id(q, r) {
                corners[c] = true;
            }
            if let Some(s) = side_id(q, r) {
                sides[s] = true;
            }
            for (dq, dr) in DIRS {
                let (a, b) = (q + dq, r + dr);
                if valid(a, b) && !seen[idx(a, b)] && self.cells[idx(a, b)] == me {
                    seen[idx(a, b)] = true;
                    st.push((a, b));
                }
            }
        }
        if corners.iter().filter(|&&x| x).count() >= 2 {
            return Some("pod");
        }
        if sides.iter().filter(|&&x| x).count() >= 3 {
            return Some("furcă");
        }
        None
    }

    fn has_ring(&self, me: u8) -> bool {
        // câmpurile care nu sunt ale lui `me` și nu pot fi atinse din afara tablei = înconjurate
        let mut reach = vec![false; W * W];
        let mut st = Vec::new();
        for q in -R..=R {
            for r in -R..=R {
                if !valid(q, r) {
                    continue;
                }
                let border = DIRS.iter().any(|&(dq, dr)| !valid(q + dq, r + dr));
                if border && self.cells[idx(q, r)] != me {
                    reach[idx(q, r)] = true;
                    st.push((q, r));
                }
            }
        }
        while let Some((q, r)) = st.pop() {
            for (dq, dr) in DIRS {
                let (a, b) = (q + dq, r + dr);
                if valid(a, b) && !reach[idx(a, b)] && self.cells[idx(a, b)] != me {
                    reach[idx(a, b)] = true;
                    st.push((a, b));
                }
            }
        }
        for q in -R..=R {
            for r in -R..=R {
                if valid(q, r) && self.cells[idx(q, r)] != me && !reach[idx(q, r)] {
                    return true;
                }
            }
        }
        false
    }

    pub fn how(&self) -> &'static str {
        self.how
    }
}

impl Game for Havannah {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.winner.is_some() {
            return Vec::new();
        }
        let mut v = Vec::new();
        for r in -R..=R {
            for q in -R..=R {
                if valid(q, r) && self.cells[idx(q, r)] == 0 {
                    v.push(Move::Place((q + R) as u8, (r + R) as u8));
                }
            }
        }
        if self.count == 1 && self.side == Player::Two {
            v.push(Move::Swap);
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        match *m {
            Move::Swap => {
                let (q, r) = self.first.unwrap();
                self.cells[idx(q, r)] = 2;
                self.side = Player::One;
            }
            Move::Place(a, b) => {
                let (q, r) = (a as i32 - R, b as i32 - R);
                let me = self.side.idx() as u8 + 1;
                self.cells[idx(q, r)] = me;
                self.count += 1;
                if self.count == 1 {
                    self.first = Some((q, r));
                }
                if let Some(h) = self.group_wins(q, r, me) {
                    self.winner = Some(Outcome::Win(self.side));
                    self.how = h;
                } else if self.has_ring(me) {
                    self.winner = Some(Outcome::Win(self.side));
                    self.how = "inel";
                } else if self.count as usize >= 169 {
                    self.winner = Some(Outcome::Draw);
                }
                self.side = self.side.other();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.winner
    }

    fn evaluate(&self, p: Player) -> i32 {
        // doar pentru AlphaBeta; AI-ul folosește MCTS
        let me = p.idx() as u8 + 1;
        let (mut a, mut b) = (0, 0);
        for &c in &self.cells {
            if c == me {
                a += 1;
            } else if c != 0 {
                b += 1;
            }
        }
        a - b
    }

    fn render(&self) -> String {
        let mut s =
            String::from("Havannah — X = J1, O = J2 (hexagoane; fiecare rând e deplasat)\n");
        s.push_str("     ");
        for q in 0..W {
            s.push_str(&format!(" {}", (b'a' + q as u8) as char));
        }
        s.push('\n');
        for r in -R..=R {
            s.push_str(&format!("{:>3} ", r + R + 1));
            s.push_str(&" ".repeat((r + R) as usize));
            for q in -R..=R {
                if !valid(q, r) {
                    s.push_str("  ");
                    continue;
                }
                s.push_str(match self.cells[idx(q, r)] {
                    1 => " X",
                    2 => " O",
                    _ => " .",
                });
            }
            s.push('\n');
        }
        s.push_str("(coloana = litera, rândul = numărul; câmpurile neocupate din colțurile tablei rombice nu există)\n");
        match self.winner {
            Some(Outcome::Win(p)) => s.push_str(&format!(
                "Jucătorul {} câștigă prin {}.\n",
                p.idx() + 1,
                self.how
            )),
            Some(Outcome::Draw) => s.push_str("Remiză.\n"),
            None => s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1)),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Swap => "swap".into(),
            Move::Place(a, b) => Self::name(*a as i32 - R, *b as i32 - R),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::Mcts(700)
    }

    fn playout_cap(&self) -> usize {
        170
    }

    fn playout_move(&self, rng: &mut polytabletop_core::rng::Rng) -> Option<Move> {
        // în simulări nu se folosește swap
        let mut empt = Vec::new();
        for r in -R..=R {
            for q in -R..=R {
                if valid(q, r) && self.cells[idx(q, r)] == 0 {
                    empt.push((q, r));
                }
            }
        }
        if empt.is_empty() {
            return None;
        }
        let (q, r) = empt[rng.below(empt.len())];
        Some(Move::Place((q + R) as u8, (r + R) as u8))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    fn put(g: &mut Havannah, q: i32, r: i32, v: u8) {
        g.cells[idx(q, r)] = v;
    }

    #[test]
    fn board_has_169_cells() {
        let mut n = 0;
        for q in -R..=R {
            for r in -R..=R {
                if valid(q, r) {
                    n += 1;
                }
            }
        }
        assert_eq!(n, 169);
    }

    #[test]
    fn ring_detected() {
        let mut g = Havannah::new();
        for (dq, dr) in DIRS {
            put(&mut g, dq, dr, 1);
        }
        assert!(g.has_ring(1));
        assert!(!g.has_ring(2));
        put(&mut g, 1, 0, 0);
        assert!(!g.has_ring(1));
    }

    #[test]
    fn bridge_between_corners() {
        let mut g = Havannah::new();
        // latură q = 7: de la (7,-7) la (7,0)
        for r in -7..=0 {
            put(&mut g, 7, r, 1);
        }
        assert_eq!(g.group_wins(7, -3, 1), Some("pod"));
    }

    #[test]
    fn fork_three_sides() {
        let mut g = Havannah::new();
        // de la centru spre trei laturi: q=7 (r=0 e colț, deci folosim r=-1 spre latura q=7?)
        // lanț: (0,0) -> (1,0)..(6,0) apoi (7,-1) latura 0
        for q in 0..=6 {
            put(&mut g, q, 0, 1);
        }
        put(&mut g, 7, -1, 1); // latura q=7
        for q in -6..0 {
            put(&mut g, q, 0, 1);
        }
        put(&mut g, -7, 1, 1); // latura q=-7
        for r in 1..=6 {
            put(&mut g, 0, r, 1);
        }
        put(&mut g, 0, 7, 0);
        put(&mut g, -1, 7, 1); // latura r=7
        put(&mut g, -1, 6, 1);
        assert_eq!(g.group_wins(0, 0, 1), Some("furcă"));
    }

    #[test]
    fn swap_only_after_first_move() {
        let mut g = Havannah::new();
        assert!(!g.legal_moves().contains(&Move::Swap));
        let m = g.legal_moves()[100];
        g.apply(&m);
        assert!(g.legal_moves().contains(&Move::Swap));
        g.apply(&Move::Swap);
        assert_eq!(g.to_move(), Player::One);
        assert_eq!(g.cells.iter().filter(|&&c| c == 2).count(), 1);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=4u64 {
            let mut g = Havannah::new();
            let mut rng = Rng::new(seed);
            while g.outcome().is_none() {
                let m = g.legal_moves();
                g.apply(&m[rng.below(m.len())]);
            }
        }
    }
}
