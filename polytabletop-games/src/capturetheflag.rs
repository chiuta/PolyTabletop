//! Capture the Flag, joc de copii pe o tablă 9×7 pentru 2 jucători.
//!
//! * Teritoriul lui J1 = coloanele a–d (steagul lui la a4), al lui J2 = coloanele f–i (steagul lui la i4); coloana e e
//!   neutră: pe ea nimeni nu poate atinge și nu poate fi atins.
//! * Fiecare jucător are 3 alergători. Mutare: o piesă face un pas ortogonal (`b2-b3`). Dacă o piesă intră în teritoriul
//!   inamic și are alături (ortogonal) un inamic aflat în teritoriul lui propriu, e atinsă și pleacă în „închisoare” 3 mutări
//!   (apoi revine la baza ei). Dacă o piesă în teritoriul propriu ajunge lângă un intrus, intrusul e atins.
//! * Dacă o piesă ajunge pe steagul inamic îl ridică (poartă steagul); dacă e atinsă, steagul revine. Câștigi dacă aduci
//!   steagul în teritoriul tău. După 160 de mutări egal = remiză.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const W: i32 = 9;
const H: i32 = 7;
const JAIL: u8 = 3;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Move {
    from: (i8, i8),
    to: (i8, i8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Pc {
    pos: (i8, i8),
    jail: u8,
    flag: bool,
}

#[derive(Clone)]
pub struct CaptureTheFlag {
    pcs: [[Pc; 3]; 2],
    side: Player,
    plies: u32,
    winner: Option<Player>,
}

impl Default for CaptureTheFlag {
    fn default() -> Self {
        Self::new()
    }
}

fn home_of(p: usize) -> i8 {
    if p == 0 {
        0
    } else {
        (W - 1) as i8
    }
}

fn flag_pos(p: usize) -> (i8, i8) {
    (home_of(p), (H / 2) as i8)
}

fn territory(x: i8) -> Option<usize> {
    match x {
        0..=3 => Some(0),
        5..=8 => Some(1),
        _ => None,
    }
}

fn name(p: (i8, i8)) -> String {
    format!("{}{}", (b'a' + p.0 as u8) as char, p.1 + 1)
}

impl CaptureTheFlag {
    pub fn new() -> Self {
        let mk = |p: usize| {
            let x = if p == 0 { 1 } else { (W - 2) as i8 };
            [1i8, 3, 5].map(|y| Pc {
                pos: (x, y),
                jail: 0,
                flag: false,
            })
        };
        CaptureTheFlag {
            pcs: [mk(0), mk(1)],
            side: Player::One,
            plies: 0,
            winner: None,
        }
    }
    fn start_of(&self, p: usize, i: usize) -> (i8, i8) {
        let x = if p == 0 { 1 } else { (W - 2) as i8 };
        (x, [1i8, 3, 5][i])
    }
    fn occupied_by_own(&self, p: usize, q: (i8, i8)) -> bool {
        self.pcs[p].iter().any(|c| c.jail == 0 && c.pos == q)
    }
    fn tag(&mut self, victim: usize, i: usize) {
        let c = &mut self.pcs[victim][i];
        c.jail = JAIL;
        c.flag = false;
        c.pos = (-1, -1);
    }
}

impl Game for CaptureTheFlag {
    type Move = Move;
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        let p = self.side.idx();
        let mut v = Vec::new();
        for c in self.pcs[p].iter().filter(|c| c.jail == 0) {
            for d in [(1i8, 0i8), (-1, 0), (0, 1), (0, -1)] {
                let q = (c.pos.0 + d.0, c.pos.1 + d.1);
                if (0..W as i8).contains(&q.0)
                    && (0..H as i8).contains(&q.1)
                    && !self.occupied_by_own(p, q)
                {
                    v.push(Move { from: c.pos, to: q });
                }
            }
        }
        if v.is_empty() {
            // toți în închisoare: mutare de „așteptare” (piesa arestată numără timpul)
            v.push(Move {
                from: (-1, -1),
                to: (-1, -1),
            });
        }
        v
    }
    fn apply(&mut self, m: &Move) {
        let p = self.side.idx();
        let o = 1 - p;
        // timpul de închisoare al pieselor proprii curge la fiecare mutare proprie
        if m.from != (-1, -1) {
            let i = self.pcs[p]
                .iter()
                .position(|c| c.jail == 0 && c.pos == m.from)
                .unwrap();
            self.pcs[p][i].pos = m.to;
            let to = m.to;
            // un inamic fără steag? ridicarea steagului
            if to == flag_pos(o) {
                self.pcs[p][i].flag = true;
            }
            // atingere
            match territory(to.0) {
                Some(t) if t == o => {
                    let near = [(1i8, 0i8), (-1, 0), (0, 1), (0, -1)].iter().any(|d| {
                        let q = (to.0 + d.0, to.1 + d.1);
                        self.pcs[o]
                            .iter()
                            .any(|c| c.jail == 0 && c.pos == q && territory(q.0) == Some(o))
                    });
                    if near {
                        self.tag(p, i);
                    }
                }
                Some(t) if t == p => {
                    for j in 0..3 {
                        let c = self.pcs[o][j];
                        if c.jail == 0
                            && territory(c.pos.0) == Some(p)
                            && (c.pos.0 - to.0).abs() + (c.pos.1 - to.1).abs() == 1
                        {
                            self.tag(o, j);
                        }
                    }
                }
                _ => {}
            }
            // câștig: purtătorul de steag e în teritoriul propriu
            for c in self.pcs[p].iter() {
                if c.jail == 0 && c.flag && territory(c.pos.0) == Some(p) {
                    self.winner = Some(self.side);
                }
            }
        }
        for j in 0..3 {
            if self.pcs[p][j].jail > 0 {
                self.pcs[p][j].jail -= 1;
                if self.pcs[p][j].jail == 0 {
                    let mut q = self.start_of(p, j);
                    while self.pcs[p].iter().any(|c| c.jail == 0 && c.pos == q) {
                        q.1 = (q.1 + 1) % H as i8;
                    }
                    self.pcs[p][j].pos = q;
                }
            }
        }
        self.plies += 1;
        self.side = self.side.other();
    }
    fn outcome(&self) -> Option<Outcome> {
        if let Some(w) = self.winner {
            return Some(Outcome::Win(w));
        }
        if self.plies >= 160 {
            return Some(Outcome::Draw);
        }
        None
    }
    fn evaluate(&self, pl: Player) -> i32 {
        let side = |p: usize| -> i32 {
            let target = flag_pos(1 - p);
            let mut v = 0;
            for c in &self.pcs[p] {
                if c.jail > 0 {
                    v -= 60;
                    continue;
                }
                let d = (c.pos.0 - target.0).abs() as i32 + (c.pos.1 - target.1).abs() as i32;
                v += 30 - d * 2;
                if c.flag {
                    let back = (c.pos.0 - home_of(p)).abs() as i32;
                    v += 500 - back * 40;
                }
            }
            v
        };
        side(pl.idx()) - side(1 - pl.idx())
    }
    fn render(&self) -> String {
        let mut s = String::from(
            "Capture the Flag — J1 = X (stânga, steag a4), J2 = O (dreapta, steag i4)\n",
        );
        for y in (0..H as i8).rev() {
            s.push_str(&format!("{} ", y + 1));
            for x in 0..W as i8 {
                let q = (x, y);
                let mut ch = if x == 4 { '|' } else { '.' };
                if q == flag_pos(0) {
                    ch = '▲';
                }
                if q == flag_pos(1) {
                    ch = '△';
                }
                for p in 0..2 {
                    for c in &self.pcs[p] {
                        if c.jail == 0 && c.pos == q {
                            ch = if p == 0 {
                                if c.flag {
                                    'x'
                                } else {
                                    'X'
                                }
                            } else if c.flag {
                                'o'
                            } else {
                                'O'
                            };
                        }
                    }
                }
                s.push(ch);
                s.push(' ');
            }
            s.push('\n');
        }
        s.push_str("  a b c d e f g h i\n");
        for p in 0..2 {
            let j: Vec<String> = self.pcs[p]
                .iter()
                .filter(|c| c.jail > 0)
                .map(|c| format!("{}", c.jail))
                .collect();
            if !j.is_empty() {
                s.push_str(&format!(
                    "Închisoare J{}: {} mutări rămase\n",
                    p + 1,
                    j.join(", ")
                ));
            }
        }
        s.push_str("(x/o minuscule = poartă steagul inamic)\n");
        match self.outcome() {
            Some(Outcome::Win(w)) => {
                s.push_str(&format!("Steagul e acasă. Câștigă J{}.\n", w.idx() + 1))
            }
            Some(Outcome::Draw) => s.push_str("Remiză.\n"),
            None => s.push_str(&format!("La mutare: J{}\n", self.side.idx() + 1)),
        }
        s
    }
    fn format_move(&self, m: &Move) -> String {
        if m.from == (-1, -1) {
            "pas".into()
        } else {
            format!("{}-{}", name(m.from), name(m.to))
        }
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tagged_when_entering_next_to_defender() {
        let mut g = CaptureTheFlag::new();
        g.pcs[0][0].pos = (4, 3);
        g.pcs[1][0].pos = (6, 3);
        g.apply(&Move {
            from: (4, 3),
            to: (5, 3),
        });
        assert!(g.pcs[0][0].jail > 0);
    }

    #[test]
    fn flag_pickup_and_return_wins() {
        let mut g = CaptureTheFlag::new();
        g.pcs[0][0].pos = (7, 3);
        g.pcs[1] = [Pc {
            pos: (-1, -1),
            jail: 5,
            flag: false,
        }; 3];
        g.apply(&Move {
            from: (7, 3),
            to: (8, 3),
        });
        assert!(g.pcs[0][0].flag);
        g.pcs[0][0].pos = (5, 3);
        g.side = Player::One;
        g.apply(&Move {
            from: (5, 3),
            to: (4, 3),
        });
        assert!(g.winner.is_none(), "linia de mijloc nu e teritoriu");
        g.side = Player::One;
        g.apply(&Move {
            from: (4, 3),
            to: (3, 3),
        });
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn games_finish() {
        let mut g: Box<dyn polytabletop_core::GameObj> = Box::new(CaptureTheFlag::new());
        let mut k = 0;
        while g.outcome().is_none() {
            let m = g.ai_move(k).unwrap();
            g.play(&m).unwrap();
            k += 1;
            assert!(k < 400);
        }
    }
}
