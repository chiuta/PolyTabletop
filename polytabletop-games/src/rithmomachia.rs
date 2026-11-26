//! Rithmomachia (Europa medievală, „jocul filozofilor”), 2 jucători — varianta simplificată documentată.
//!
//! * Tablă 8 coloane × 16 rânduri. Fiecare parte are 24 de piese: 8 rotunde, 8 triunghiuri, 8 pătrate
//!   (dintre care un pătrat e înlocuit de piramidă = regele). J1 = „pari” (X), J2 = „impari” (O).
//!   Numerele sunt cele tradiționale (pari: 2 4 6 8 4 16 36 64 / 6 20 42 72 9 25 49 81 / 15 45 91 153
//!   25 81 169 289; impari: 3 5 7 9 9 25 49 81 / 12 30 56 90 16 36 64 100 / 28 66 120 190 49 121 225 361),
//!   regele pari = 91, regele impari = 190. Așezarea pe rânduri (pătrate în spate, triunghiuri la mijloc,
//!   rotunde în față) este reconstituirea noastră; sursele variază.
//! * Mutarea: rotunda un pas pe diagonală; triunghiul exact două câmpuri ortogonal; pătratul exact trei
//!   câmpuri ortogonal; piramida (regele) poate muta ca oricare dintre cele trei. Fără salt (drumul liber).
//! * Capturi implementate: **întâlnire** (aterizezi pe o piesă adversă cu aceeași valoare), **asalt** (aterizezi
//!   pe o piesă adversă a cărei valoare = valoarea ta × lungimea mutării) și **asediu** (după mutarea ta, o piesă
//!   adversă ale cărei patru vecine ortogonale sunt piese ale tale sau marginea tablei e capturată).
//!   Ambuscada nu e implementată.
//! * Victorie: capturezi regele adversarului, sau 8 piese, sau adversarul nu mai are mutări. După 300 de
//!   mutări câștigă cine a capturat mai multă valoare (egal = remiză). Victoriile „proprii” cu progresii nu sunt
//!   implementate.
//!
//! Mutări: `a3-a5`, cu captură `a3xa5`.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const COLS: usize = 8;
const ROWS: usize = 16;
const MAX_PLIES: u16 = 300;
const WIN_CAPTURES: u8 = 8;

#[derive(Clone, Copy, PartialEq, Debug)]
struct Piece {
    side: u8,
    kind: u8, // 0 rotund, 1 triunghi, 2 pătrat, 3 piramidă
    value: u16,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Move {
    pub from: u8,
    pub to: u8,
    pub capture: bool,
}

#[derive(Clone)]
pub struct Rithmomachia {
    cell: Vec<Option<Piece>>,
    side: Player,
    plies: u16,
    caps: [u8; 2],
    capval: [u32; 2],
    winner: Option<Outcome>,
}

impl Default for Rithmomachia {
    fn default() -> Self {
        Self::new()
    }
}

fn nm(i: usize) -> String {
    format!("{}{}", (b'a' + (i % COLS) as u8) as char, i / COLS + 1)
}

impl Rithmomachia {
    pub fn new() -> Self {
        let mut cell = vec![None; COLS * ROWS];
        let evens: [[u16; 8]; 3] = [
            [15, 45, 91, 153, 25, 81, 169, 289],
            [6, 20, 42, 72, 9, 25, 49, 81],
            [2, 4, 6, 8, 4, 16, 36, 64],
        ];
        let odds: [[u16; 8]; 3] = [
            [28, 66, 120, 190, 49, 121, 225, 361],
            [12, 30, 56, 90, 16, 36, 64, 100],
            [3, 5, 7, 9, 9, 25, 49, 81],
        ];
        for (k, kind) in [2u8, 1, 0].iter().enumerate() {
            for c in 0..8 {
                let v = evens[k][c];
                let kd = if v == 91 && *kind == 2 { 3 } else { *kind };
                cell[k * COLS + c] = Some(Piece {
                    side: 0,
                    kind: kd,
                    value: v,
                });
                let w = odds[k][c];
                let kd = if w == 190 && *kind == 2 { 3 } else { *kind };
                cell[(ROWS - 1 - k) * COLS + c] = Some(Piece {
                    side: 1,
                    kind: kd,
                    value: w,
                });
            }
        }
        Rithmomachia {
            cell,
            side: Player::One,
            plies: 0,
            caps: [0, 0],
            capval: [0, 0],
            winner: None,
        }
    }

    pub fn with_seed(_seed: u64) -> Self {
        Self::new()
    }

    fn cell_at(&self, c: i32, r: i32) -> Option<Option<Piece>> {
        if c < 0 || c >= COLS as i32 || r < 0 || r >= ROWS as i32 {
            None
        } else {
            Some(self.cell[r as usize * COLS + c as usize])
        }
    }

    fn moves_for(&self, p: Player) -> Vec<Move> {
        let side = p.idx() as u8;
        let mut v = Vec::new();
        for i in 0..self.cell.len() {
            let Some(pc) = self.cell[i] else { continue };
            if pc.side != side {
                continue;
            }
            let (c, r) = ((i % COLS) as i32, (i / COLS) as i32);
            let try_dir = |dc: i32, dr: i32, len: i32, out: &mut Vec<Move>| {
                for s in 1..len {
                    match self.cell_at(c + dc * s, r + dr * s) {
                        Some(None) => {}
                        _ => return,
                    }
                }
                let (tc, tr) = (c + dc * len, r + dr * len);
                match self.cell_at(tc, tr) {
                    Some(None) => out.push(Move {
                        from: i as u8,
                        to: (tr as usize * COLS + tc as usize) as u8,
                        capture: false,
                    }),
                    Some(Some(t)) if t.side != side => {
                        let ok = t.value == pc.value || t.value as i32 == pc.value as i32 * len;
                        if ok {
                            out.push(Move {
                                from: i as u8,
                                to: (tr as usize * COLS + tc as usize) as u8,
                                capture: true,
                            });
                        }
                    }
                    _ => {}
                }
            };
            let kinds: &[u8] = if pc.kind == 3 { &[0, 1, 2] } else { &[pc.kind] };
            for &k in kinds {
                match k {
                    0 => {
                        for (dc, dr) in [(1, 1), (1, -1), (-1, 1), (-1, -1)] {
                            try_dir(dc, dr, 1, &mut v);
                        }
                    }
                    n => {
                        for (dc, dr) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                            try_dir(dc, dr, n as i32 + 1, &mut v);
                        }
                    }
                }
            }
        }
        v.sort_by_key(|m| (m.from, m.to));
        v.dedup();
        v
    }

    fn siege(&mut self, mover: u8) {
        let mut gone = Vec::new();
        for i in 0..self.cell.len() {
            let Some(t) = self.cell[i] else { continue };
            if t.side == mover {
                continue;
            }
            let (c, r) = ((i % COLS) as i32, (i / COLS) as i32);
            let mut all = true;
            for (dc, dr) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                match self.cell_at(c + dc, r + dr) {
                    None => {}
                    Some(Some(n)) if n.side == mover => {}
                    _ => all = false,
                }
            }
            if all {
                gone.push(i);
            }
        }
        for i in gone {
            let t = self.cell[i].take().unwrap();
            self.caps[mover as usize] += 1;
            self.capval[mover as usize] += t.value as u32;
            if t.kind == 3 {
                self.winner = Some(Outcome::Win(if mover == 0 {
                    Player::One
                } else {
                    Player::Two
                }));
            }
        }
    }

    pub fn captured(&self) -> [u8; 2] {
        self.caps
    }
}

impl Game for Rithmomachia {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.winner.is_some() {
            return Vec::new();
        }
        self.moves_for(self.side)
    }

    fn apply(&mut self, m: &Move) {
        let me = self.side.idx();
        let pc = self.cell[m.from as usize].take().unwrap();
        if let Some(t) = self.cell[m.to as usize] {
            self.caps[me] += 1;
            self.capval[me] += t.value as u32;
            if t.kind == 3 {
                self.winner = Some(Outcome::Win(self.side));
            }
        }
        self.cell[m.to as usize] = Some(pc);
        self.siege(me as u8);
        self.plies += 1;
        if self.winner.is_none() && self.caps[me] >= WIN_CAPTURES {
            self.winner = Some(Outcome::Win(self.side));
        }
        if self.winner.is_none() && self.plies >= MAX_PLIES {
            self.winner = Some(match self.capval[0].cmp(&self.capval[1]) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                std::cmp::Ordering::Equal => Outcome::Draw,
            });
        }
        self.side = self.side.other();
        if self.winner.is_none() && self.moves_for(self.side).is_empty() {
            self.winner = Some(Outcome::Win(self.side.other()));
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.winner
    }

    fn evaluate(&self, p: Player) -> i32 {
        let mut s = 0i32;
        for (i, c) in self.cell.iter().enumerate() {
            if let Some(pc) = c {
                let adv = if pc.side == 0 {
                    i / COLS
                } else {
                    ROWS - 1 - i / COLS
                } as i32;
                let w = if pc.kind == 3 {
                    300
                } else {
                    40 + pc.kind as i32 * 5
                } + adv;
                s += if pc.side == p.idx() as u8 { w } else { -w };
            }
        }
        s
    }

    fn render(&self) -> String {
        let mut s = String::from(
            "Rithmomachia — J1 (pari, rânduri 1–3), J2 (impari, rânduri 14–16); R rotund, T triunghi, P pătrat, K rege\n    ",
        );
        for c in 0..COLS {
            s.push_str(&format!("   {}  ", (b'a' + c as u8) as char));
        }
        s.push('\n');
        for r in (0..ROWS).rev() {
            s.push_str(&format!("{:>2}  ", r + 1));
            for c in 0..COLS {
                match self.cell[r * COLS + c] {
                    None => s.push_str("  .   "),
                    Some(p) => {
                        let k = ['R', 'T', 'P', 'K'][p.kind as usize];
                        let o = if p.side == 0 { 'x' } else { 'o' };
                        s.push_str(&format!("{}{}{:<3} ", k, o, p.value));
                    }
                }
            }
            s.push('\n');
        }
        s.push_str(&format!(
            "Capturate: J1 {} (valoare {}), J2 {} (valoare {})\n",
            self.caps[0], self.capval[0], self.caps[1], self.capval[1]
        ));
        match self.winner {
            Some(Outcome::Win(p)) => s.push_str(&format!("Jucătorul {} câștigă.\n", p.idx() + 1)),
            Some(Outcome::Draw) => s.push_str("Remiză.\n"),
            None => s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1)),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        format!(
            "{}{}{}",
            nm(m.from as usize),
            if m.capture { "x" } else { "-" },
            nm(m.to as usize)
        )
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 600,
            max_depth: 4,
        }
    }

    fn move_score(&self, m: &Move) -> i32 {
        if m.capture {
            50
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    #[test]
    fn setup_has_24_per_side() {
        let g = Rithmomachia::new();
        assert_eq!(g.cell.iter().flatten().filter(|p| p.side == 0).count(), 24);
        assert_eq!(g.cell.iter().flatten().filter(|p| p.side == 1).count(), 24);
        assert_eq!(g.cell.iter().flatten().filter(|p| p.kind == 3).count(), 2);
    }

    #[test]
    fn meeting_and_assault() {
        let mut g = Rithmomachia::new();
        g.cell = vec![None; COLS * ROWS];
        // triunghi 20 (J1) în c5; piesa 40 ar fi asalt (20*2); aici 40 nu există, folosim 36 ... test: 20 x2 = 40
        g.cell[4 * COLS + 2] = Some(Piece {
            side: 0,
            kind: 1,
            value: 20,
        });
        g.cell[4 * COLS + 4] = Some(Piece {
            side: 1,
            kind: 2,
            value: 40,
        });
        g.cell[10 * COLS + 7] = Some(Piece {
            side: 1,
            kind: 0,
            value: 3,
        });
        g.cell[0] = Some(Piece {
            side: 0,
            kind: 3,
            value: 91,
        });
        let mv = g.legal_moves();
        assert!(mv
            .iter()
            .any(|m| m.capture && m.from as usize == 4 * COLS + 2));
        // întâlnire: rotundă 3 vs 3
        g.cell = vec![None; COLS * ROWS];
        g.cell[2 * COLS + 2] = Some(Piece {
            side: 0,
            kind: 0,
            value: 3,
        });
        g.cell[3 * COLS + 3] = Some(Piece {
            side: 1,
            kind: 0,
            value: 3,
        });
        g.cell[10 * COLS] = Some(Piece {
            side: 1,
            kind: 3,
            value: 190,
        });
        g.cell[10 * COLS + 7] = Some(Piece {
            side: 0,
            kind: 3,
            value: 91,
        });
        assert!(g.legal_moves().iter().any(|m| m.capture));
        // fără potrivire: nu e captură
        g.cell[3 * COLS + 3] = Some(Piece {
            side: 1,
            kind: 0,
            value: 4,
        });
        assert!(!g.legal_moves().iter().any(|m| m.capture));
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=4u64 {
            let mut g = Rithmomachia::new();
            let mut rng = Rng::new(seed);
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
            }
        }
    }
}
