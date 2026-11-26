//! Familia „căi ferate”: harți originale mici; strângi cărți colorate și ocupi rute între
//! orașe; bilete de destinație publice (conectat = bonus, neconectat = pierdere).
//! Variante originale, inspirate doar de mecanica generală „colecție de cărți + rute”.
#![allow(clippy::needless_range_loop, clippy::manual_is_multiple_of)]

use crate::w7_util::Dice;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub struct Spec {
    pub id: &'static str,
    pub cities: &'static [&'static str],
    /// (a, b, lungime, culoare 0..4 sau 6 = oricare)
    pub edges: &'static [(u8, u8, u8, u8)],
    pub trains: u8,
}

pub const SPECS: &[Spec] = &[
    Spec {
        id: "linii_urbane",
        cities: &[
            "Nord", "Port", "Deal", "Lac", "Piata", "Gara", "Sat", "Vama", "Lunca",
        ],
        edges: &[
            (0, 1, 2, 0),
            (0, 2, 3, 1),
            (1, 3, 2, 2),
            (2, 3, 1, 6),
            (2, 4, 2, 3),
            (3, 4, 2, 0),
            (3, 5, 3, 1),
            (4, 5, 1, 6),
            (4, 6, 3, 2),
            (5, 7, 2, 3),
            (6, 7, 2, 6),
            (6, 8, 1, 0),
            (7, 8, 3, 1),
            (1, 4, 4, 3),
            (0, 3, 4, 2),
        ],
        trains: 16,
    },
    Spec {
        id: "cai_continentale",
        cities: &[
            "Aurora", "Bora", "Cetina", "Dunav", "Elba", "Fiord", "Gorun", "Harta", "Insula",
            "Jale", "Kalm", "Luncani", "Mares", "Nisip",
        ],
        edges: &[
            (0, 1, 2, 0),
            (0, 2, 3, 1),
            (1, 3, 2, 2),
            (1, 4, 3, 6),
            (2, 4, 2, 3),
            (2, 5, 4, 4),
            (3, 6, 3, 0),
            (4, 6, 2, 1),
            (4, 7, 3, 6),
            (5, 7, 2, 2),
            (5, 8, 3, 3),
            (6, 9, 2, 4),
            (7, 9, 3, 0),
            (7, 10, 2, 1),
            (8, 10, 4, 6),
            (9, 11, 3, 2),
            (10, 11, 2, 3),
            (10, 12, 3, 4),
            (11, 13, 2, 0),
            (12, 13, 2, 1),
            (8, 12, 5, 6),
            (3, 9, 5, 6),
            (0, 5, 5, 6),
            (12, 7, 4, 2),
        ],
        trains: 24,
    },
];

const COLORS: usize = 6; // 0..4 + 5 = locomotivă (joker)
const SYM: [char; 6] = ['r', 'v', 'a', 'g', 'n', 'J'];
const PTS: [i32; 6] = [0, 1, 2, 4, 7, 10];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RMove {
    Claim(u8),
    DrawUp(u8),
    DrawBlind,
}

#[derive(Clone)]
pub struct Rail {
    spec: &'static Spec,
    owner: Vec<Option<Player>>,
    hand: [[u8; COLORS]; 2],
    trains: [u8; 2],
    up: [u8; 4],
    tickets: [Vec<(u8, u8, i32)>; 2],
    side: Player,
    last: Option<u8>,
    turns: u32,
    score_claim: [i32; 2],
    dice: Dice,
}

impl Rail {
    pub fn new(spec: &'static Spec) -> Self {
        Self::with_seed(spec, Rng::from_clock().next_u64())
    }

    pub fn with_seed(spec: &'static Spec, seed: u64) -> Self {
        let mut dice = Dice::new(seed);
        let mut up = [0u8; 4];
        for u in up.iter_mut() {
            *u = Self::card(&mut dice);
        }
        let n = spec.cities.len();
        let mut tickets: [Vec<(u8, u8, i32)>; 2] = [vec![], vec![]];
        for t in tickets.iter_mut() {
            while t.len() < 3 {
                let a = dice.below(n) as u8;
                let b = dice.below(n) as u8;
                if a != b
                    && !t
                        .iter()
                        .any(|&(x, y, _)| (x, y) == (a, b) || (x, y) == (b, a))
                {
                    t.push((a, b, 4 + dice.below(8) as i32));
                }
            }
        }
        Rail {
            spec,
            owner: vec![None; spec.edges.len()],
            hand: [[0; COLORS]; 2],
            trains: [spec.trains; 2],
            up,
            tickets,
            side: Player::One,
            last: None,
            turns: 0,
            score_claim: [0; 2],
            dice,
        }
    }

    fn card(d: &mut Dice) -> u8 {
        let r = d.below(30);
        if r < 25 {
            (r / 5) as u8
        } else {
            5
        }
    }

    /// Cum plătești ruta `e` (jucătorul `p`): numărul de cărți din fiecare culoare.
    fn pay(&self, p: usize, e: usize) -> Option<[u8; COLORS]> {
        let (_, _, len, col) = self.spec.edges[e];
        let h = &self.hand[p];
        let try_color = |c: usize| -> Option<[u8; COLORS]> {
            let own = h[c].min(len);
            let wild = len - own;
            if wild <= h[5] {
                let mut r = [0; COLORS];
                r[c] = own;
                r[5] = wild;
                Some(r)
            } else {
                None
            }
        };
        if col == 6 {
            let mut best: Option<[u8; COLORS]> = None;
            for c in 0..5 {
                if let Some(r) = try_color(c) {
                    if best.is_none_or(|b| r[5] < b[5]) {
                        best = Some(r);
                    }
                }
            }
            best
        } else {
            try_color(col as usize)
        }
    }

    fn connected(&self, p: Player, a: u8, b: u8) -> bool {
        let n = self.spec.cities.len();
        let mut seen = vec![false; n];
        let mut st = vec![a as usize];
        seen[a as usize] = true;
        while let Some(c) = st.pop() {
            if c == b as usize {
                return true;
            }
            for (i, &(x, y, _, _)) in self.spec.edges.iter().enumerate() {
                if self.owner[i] == Some(p) {
                    let o = if x as usize == c {
                        y as usize
                    } else if y as usize == c {
                        x as usize
                    } else {
                        continue;
                    };
                    if !seen[o] {
                        seen[o] = true;
                        st.push(o);
                    }
                }
            }
        }
        false
    }

    pub fn final_score(&self, p: Player) -> i32 {
        let mut s = self.score_claim[p.idx()];
        for &(a, b, pts) in &self.tickets[p.idx()] {
            s += if self.connected(p, a, b) { pts } else { -pts };
        }
        s
    }
}

impl Game for Rail {
    type Move = RMove;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<RMove> {
        if self.outcome().is_some() {
            return vec![];
        }
        let p = self.side.idx();
        let mut v = Vec::new();
        for e in 0..self.spec.edges.len() {
            if self.owner[e].is_none()
                && self.spec.edges[e].2 <= self.trains[p]
                && self.pay(p, e).is_some()
            {
                v.push(RMove::Claim(e as u8));
            }
        }
        let mut seen = [false; COLORS];
        for (i, &c) in self.up.iter().enumerate() {
            if !seen[c as usize] {
                seen[c as usize] = true;
                v.push(RMove::DrawUp(i as u8));
            }
        }
        v.push(RMove::DrawBlind);
        v
    }

    fn apply(&mut self, m: &RMove) {
        let p = self.side.idx();
        match *m {
            RMove::Claim(e) => {
                let pay = self.pay(p, e as usize).expect("plătibil");
                for c in 0..COLORS {
                    self.hand[p][c] -= pay[c];
                }
                let len = self.spec.edges[e as usize].2;
                self.owner[e as usize] = Some(self.side);
                self.trains[p] -= len;
                self.score_claim[p] += PTS[(len as usize).min(5)];
                if self.trains[p] <= 2 && self.last.is_none() {
                    self.last = Some(p as u8);
                }
            }
            RMove::DrawUp(i) => {
                let c = self.up[i as usize];
                self.hand[p][c as usize] += 1;
                self.up[i as usize] = Self::card(&mut self.dice);
                let b = Self::card(&mut self.dice);
                self.hand[p][b as usize] += 1;
            }
            RMove::DrawBlind => {
                for _ in 0..2 {
                    let b = Self::card(&mut self.dice);
                    self.hand[p][b as usize] += 1;
                }
            }
        }
        self.side = self.side.other();
        self.turns += 1;
        if self.last.is_none() && self.trains[0].min(self.trains[1]) <= 2 {
            self.last = Some(0);
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        let over = self.turns >= 300 || (self.last.is_some() && self.turns % 2 == 0);
        if !over {
            return None;
        }
        let (a, b) = (self.final_score(Player::One), self.final_score(Player::Two));
        Some(match a.cmp(&b) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            _ => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let side = |q: Player| {
            self.final_score(q) * 3 + self.hand[q.idx()].iter().map(|&x| x as i32).sum::<i32>()
        };
        side(p) - side(p.other())
    }

    fn render(&self) -> String {
        let mut s = String::new();
        for q in [Player::One, Player::Two] {
            s.push_str(&format!(
                "Jucătorul {}{}: scor {} (trenuri {}), cărți ",
                q.idx() + 1,
                if q == self.side { " (la rând)" } else { "" },
                self.final_score(q),
                self.trains[q.idx()]
            ));
            for c in 0..COLORS {
                s.push_str(&format!("{}{} ", SYM[c], self.hand[q.idx()][c]));
            }
            s.push_str("\n  bilete: ");
            for &(a, b, pts) in &self.tickets[q.idx()] {
                s.push_str(&format!(
                    "{}-{} {}{}; ",
                    self.spec.cities[a as usize],
                    self.spec.cities[b as usize],
                    pts,
                    if self.connected(q, a, b) {
                        " (făcut)"
                    } else {
                        ""
                    }
                ));
            }
            s.push('\n');
        }
        s.push_str("Cărți vizibile: ");
        for (i, &c) in self.up.iter().enumerate() {
            s.push_str(&format!("{}={} ", i + 1, SYM[c as usize]));
        }
        s.push_str("\nRute (r,v,a,g,n culori; * oricare; J joker):\n");
        for (i, &(a, b, len, col)) in self.spec.edges.iter().enumerate() {
            let own = match self.owner[i] {
                Some(p) => format!("J{}", p.idx() + 1),
                None => "liber".into(),
            };
            s.push_str(&format!(
                "  {}: {}-{} lung {} {} [{}]\n",
                i + 1,
                self.spec.cities[a as usize],
                self.spec.cities[b as usize],
                len,
                if col == 6 { '*' } else { SYM[col as usize] },
                own
            ));
        }
        s
    }

    fn format_move(&self, m: &RMove) -> String {
        match *m {
            RMove::Claim(e) => format!("ocup {}", e + 1),
            RMove::DrawUp(i) => format!("trag {}", i + 1),
            RMove::DrawBlind => "trag oarba".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(2)
    }
}

pub fn make(id: &str) -> Box<dyn polytabletop_core::GameObj> {
    Box::new(Rail::new(SPECS.iter().find(|s| s.id == id).expect("spec")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_games_end() {
        for spec in SPECS {
            for seed in 1..=10u64 {
                let mut g = Rail::with_seed(spec, seed);
                let mut rng = Rng::new(seed);
                let mut n = 0;
                while g.outcome().is_none() {
                    let mv = g.legal_moves();
                    assert!(!mv.is_empty());
                    let m = mv[rng.below(mv.len())];
                    g.apply(&m);
                    n += 1;
                    assert!(n <= 300);
                }
            }
        }
    }

    #[test]
    fn claims_are_paid_for_and_ai_is_legal() {
        polytabletop_core::ai::set_time_budget_override_ms(Some(5));
        for spec in SPECS {
            let mut g = Rail::with_seed(spec, 4);
            let mut rng = Rng::new(2);
            while g.outcome().is_none() {
                if g.side == Player::One {
                    let t = polytabletop_core::GameObj::ai_move(&g, rng.next_u64()).unwrap();
                    let tot: u32 = g.hand[0].iter().map(|&x| x as u32).sum();
                    assert!(polytabletop_core::GameObj::legal_moves(&g).contains(&t));
                    polytabletop_core::GameObj::play(&mut g, &t).unwrap();
                    let tot2: u32 = g.hand[0].iter().map(|&x| x as u32).sum();
                    if t.starts_with("ocup") {
                        assert!(tot2 < tot);
                    }
                } else {
                    let mv = g.legal_moves();
                    let m = mv[rng.below(mv.len())];
                    g.apply(&m);
                }
            }
        }
    }
}
