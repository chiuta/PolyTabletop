//! Motor comun pentru jocurile tafl: Hnefatafl (11×11, varianta „Copenhaga”) și Brandubh (7×7).
//!
//! Reguli implementate (clean-room, din regulile publice reconstituite):
//! * Atacatorii (J1, `X`) mută primii; apărătorii (J2, `O`) au și un rege (`K`).
//! * Toate piesele mută ca turnul (orizontal/vertical, oricâte câmpuri libere), fără salt.
//!   Doar regele poate ajunge pe tron (centru) sau în colțuri; orice piesă poate trece peste
//!   tronul gol.
//! * Captură prin încadrare (custodial): piesa mutată încadrează o piesă adversă între ea și o
//!   altă piesă proprie sau un câmp ostil (colțurile; tronul – ostil atacatorilor mereu, apărătorilor
//!   doar gol). Regele ajută la capturi. Nu se capturează activ intrând între două piese adverse.
//! * Regele: Hnefatafl = capturat când e încercuit pe patru laturi de atacatori sau de tronul gol;
//!   Brandubh = ca o piesă obișnuită (încadrare pe două laturi) când nu e pe tron sau lângă el,
//!   altfel încercuit pe patru laturi.
//! * Apărătorii câștigă când regele ajunge într-un colț; atacatorii când capturează regele. Cine
//!   nu mai are mutări pierde. Remiză după 400 de mutări.
//! * Nu sunt implementate: zidul de scuturi (shieldwall), încercuirea totală, interzicerea
//!   repetițiilor (înlocuită de limita de mutări).
//!
//! Mutări: `d1-d4` (coloană a.., rând 1.. de sus în jos).
#![allow(clippy::unnecessary_cast, clippy::identity_op)]

use polytabletop_core::{hash_mix, AiStyle, Game, Outcome, Player};

const ATT: u8 = 1;
const DEF: u8 = 2;
const KING: u8 = 3;
const MAX_PLIES: u16 = 400;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Rules {
    pub n: usize,
    /// Regele e capturat pe două laturi (Brandubh) când nu e lângă tron.
    pub king_two_sided: bool,
}

pub const HNEFATAFL: Rules = Rules {
    n: 11,
    king_two_sided: false,
};
pub const BRANDUBH: Rules = Rules {
    n: 7,
    king_two_sided: true,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Move {
    pub from: u16,
    pub to: u16,
}

#[derive(Clone)]
pub struct Tafl {
    rules: Rules,
    cell: Vec<u8>,
    side: Player,
    plies: u16,
    winner: Option<Outcome>,
}

impl Tafl {
    pub fn new(rules: Rules) -> Self {
        let n = rules.n;
        let mut cell = vec![0u8; n * n];
        let c = n / 2;
        let mut put = |r: usize, col: usize, v: u8| cell[r * n + col] = v;
        if n == 11 {
            for i in 3..=7 {
                put(0, i, ATT);
                put(10, i, ATT);
                put(i, 0, ATT);
                put(i, 10, ATT);
            }
            put(1, 5, ATT);
            put(9, 5, ATT);
            put(5, 1, ATT);
            put(5, 9, ATT);
            for (r, col) in [
                (3, 5),
                (4, 4),
                (4, 5),
                (4, 6),
                (5, 3),
                (5, 4),
                (5, 6),
                (5, 7),
                (6, 4),
                (6, 5),
                (6, 6),
                (7, 5),
            ] {
                put(r, col, DEF);
            }
        } else {
            for (r, col) in [
                (0, 3),
                (1, 3),
                (5, 3),
                (6, 3),
                (3, 0),
                (3, 1),
                (3, 5),
                (3, 6),
            ] {
                put(r, col, ATT);
            }
            for (r, col) in [(2, 3), (4, 3), (3, 2), (3, 4)] {
                put(r, col, DEF);
            }
        }
        put(c, c, KING);
        Tafl {
            rules,
            cell,
            side: Player::One,
            plies: 0,
            winner: None,
        }
    }

    pub fn with_seed(rules: Rules, _seed: u64) -> Self {
        Self::new(rules)
    }

    fn n(&self) -> usize {
        self.rules.n
    }
    fn throne(&self) -> usize {
        let c = self.n() / 2;
        c * self.n() + c
    }
    fn is_corner(&self, i: usize) -> bool {
        let n = self.n();
        let (r, c) = (i / n, i % n);
        (r == 0 || r == n - 1) && (c == 0 || c == n - 1)
    }
    fn restricted(&self, i: usize) -> bool {
        i == self.throne() || self.is_corner(i)
    }

    fn mine(&self, p: Player, v: u8) -> bool {
        match p {
            Player::One => v == ATT,
            Player::Two => v == DEF || v == KING,
        }
    }

    fn moves_for(&self, p: Player) -> Vec<Move> {
        let n = self.n() as i32;
        let mut v = Vec::new();
        for i in 0..self.cell.len() {
            let t = self.cell[i];
            if !self.mine(p, t) {
                continue;
            }
            let (r, c) = ((i / self.n()) as i32, (i % self.n()) as i32);
            for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (mut rr, mut cc) = (r + dr, c + dc);
                while rr >= 0 && rr < n && cc >= 0 && cc < n {
                    let j = (rr * n + cc) as usize;
                    if self.cell[j] != 0 {
                        break;
                    }
                    if self.restricted(j) {
                        if t == KING {
                            v.push(Move {
                                from: i as u16,
                                to: j as u16,
                            });
                        }
                        // trecerea peste tronul gol e permisă, peste colț nu are sens (margine)
                    } else {
                        v.push(Move {
                            from: i as u16,
                            to: j as u16,
                        });
                    }
                    rr += dr;
                    cc += dc;
                }
            }
        }
        v
    }

    /// Câmpul `j` e ostil piesei de tip `t` (a mutatorului adversar)?
    fn hostile_to(&self, j: usize, t: u8, mover: Player) -> bool {
        let v = self.cell[j];
        if self.mine(mover, v) {
            return true;
        }
        if self.is_corner(j) && v == 0 {
            return true;
        }
        if j == self.throne() && v == 0 {
            return true;
        }
        if j == self.throne() && v == KING {
            // tronul ocupat de rege e ostil atacatorilor
            return t == ATT && mover == Player::Two;
        }
        false
    }

    fn do_move(&mut self, m: &Move) {
        let mover = self.side;
        let (from, to) = (m.from as usize, m.to as usize);
        let t = self.cell[from];
        self.cell[from] = 0;
        self.cell[to] = t;
        let n = self.n() as i32;
        let (r, c) = ((to / self.n()) as i32, (to % self.n()) as i32);
        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (r1, c1) = (r + dr, c + dc);
            let (r2, c2) = (r + 2 * dr, c + 2 * dc);
            if r1 < 0 || r1 >= n || c1 < 0 || c1 >= n {
                continue;
            }
            let j = (r1 * n + c1) as usize;
            let v = self.cell[j];
            if v == 0 || self.mine(mover, v) {
                continue;
            }
            if v == KING {
                if self.king_captured(j, mover) {
                    self.cell[j] = 0;
                    self.winner = Some(Outcome::Win(Player::One));
                }
                continue;
            }
            if r2 < 0 || r2 >= n || c2 < 0 || c2 >= n {
                continue;
            }
            let k = (r2 * n + c2) as usize;
            let hostile = if self.mine(mover, self.cell[k]) {
                true
            } else if self.cell[k] == 0 && (self.is_corner(k) || k == self.throne()) {
                // tronul gol e ostil apărătorilor și atacatorilor; colțul tuturor
                true
            } else {
                k == self.throne() && self.cell[k] == KING && v == ATT
            };
            if hostile {
                self.cell[j] = 0;
            }
        }
        if t == KING && self.is_corner(to) {
            self.winner = Some(Outcome::Win(Player::Two));
        }
        let _ = self.hostile_to(0, 0, mover);
    }

    fn king_captured(&self, j: usize, mover: Player) -> bool {
        let n = self.n() as i32;
        let (r, c) = ((j / self.n()) as i32, (j % self.n()) as i32);
        let near_throne = {
            let t = self.throne();
            let (tr, tc) = ((t / self.n()) as i32, (t % self.n()) as i32);
            (r - tr).abs() + (c - tc).abs() <= 1
        };
        let hostile = |rr: i32, cc: i32| -> bool {
            if rr < 0 || rr >= n || cc < 0 || cc >= n {
                return false;
            }
            let k = (rr * n + cc) as usize;
            self.mine(mover, self.cell[k])
                || (self.cell[k] == 0 && (k == self.throne() || self.is_corner(k)))
        };
        if self.rules.king_two_sided && !near_throne {
            (hostile(r - 1, c) && hostile(r + 1, c)) || (hostile(r, c - 1) && hostile(r, c + 1))
        } else {
            hostile(r - 1, c) && hostile(r + 1, c) && hostile(r, c - 1) && hostile(r, c + 1)
        }
    }

    pub fn name(&self, i: usize) -> String {
        format!(
            "{}{}",
            (b'a' + (i % self.n()) as u8) as char,
            i / self.n() + 1
        )
    }

    fn king_pos(&self) -> Option<usize> {
        self.cell.iter().position(|&v| v == KING)
    }

    pub fn count(&self, v: u8) -> usize {
        self.cell.iter().filter(|&&x| x == v).count()
    }
}

impl Game for Tafl {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        self.moves_for(self.side)
    }

    fn apply(&mut self, m: &Move) {
        self.do_move(m);
        self.plies += 1;
        self.side = self.side.other();
        if self.winner.is_none() && self.king_pos().is_none() {
            self.winner = Some(Outcome::Win(Player::One));
        }
        if self.winner.is_none() && self.plies >= MAX_PLIES {
            self.winner = Some(Outcome::Draw);
        }
        if self.winner.is_none() && self.moves_for(self.side).is_empty() {
            self.winner = Some(Outcome::Win(self.side.other()));
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.winner
    }

    fn evaluate(&self, p: Player) -> i32 {
        let n = self.n();
        let att = self.count(ATT) as i32;
        let def = self.count(DEF) as i32;
        let mut v = (def * 30 - att * 15) as i32; // din punctul de vedere al apărătorilor
        if let Some(k) = self.king_pos() {
            let (r, c) = (k / n, k % n);
            let dist = [(0, 0), (0, n - 1), (n - 1, 0), (n - 1, n - 1)]
                .iter()
                .map(|&(a, b)| (r as i32 - a as i32).abs() + (c as i32 - b as i32).abs())
                .min()
                .unwrap();
            v += (2 * n as i32 - dist) * 6;
            // libertățile regelui pe cele patru direcții
            let mut free = 0;
            for (dr, dc) in [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
                let (mut rr, mut cc) = (r as i32 + dr, c as i32 + dc);
                while rr >= 0 && rr < n as i32 && cc >= 0 && cc < n as i32 {
                    if self.cell[rr as usize * n + cc as usize] != 0 {
                        break;
                    }
                    free += 1;
                    rr += dr;
                    cc += dc;
                }
            }
            v += free * 3;
            // regele atacat: atacatori adiacenți
            let mut adj = 0;
            for (dr, dc) in [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
                let (rr, cc) = (r as i32 + dr, c as i32 + dc);
                if rr >= 0
                    && rr < n as i32
                    && cc >= 0
                    && cc < n as i32
                    && self.cell[rr as usize * n + cc as usize] == ATT
                {
                    adj += 1;
                }
            }
            v -= adj * 25;
        }
        match p {
            Player::Two => v,
            Player::One => -v,
        }
    }

    fn render(&self) -> String {
        let n = self.n();
        let mut s = format!(
            "{} — X = atacatori (J1), O = apărători (J2), K = rege, # = colț/tron gol\n   ",
            if n == 11 { "Hnefatafl" } else { "Brandubh" }
        );
        for c in 0..n {
            s.push_str(&format!(" {}", (b'a' + c as u8) as char));
        }
        s.push('\n');
        for r in 0..n {
            s.push_str(&format!("{:>2} ", r + 1));
            for c in 0..n {
                let i = r * n + c;
                let ch = match self.cell[i] {
                    ATT => 'X',
                    DEF => 'O',
                    KING => 'K',
                    _ => {
                        if self.restricted(i) {
                            '#'
                        } else {
                            '.'
                        }
                    }
                };
                s.push_str(&format!(" {ch}"));
            }
            s.push('\n');
        }
        s.push_str(&format!(
            "Atacatori: {}, apărători: {}.\n",
            self.count(ATT),
            self.count(DEF)
        ));
        match self.winner {
            Some(Outcome::Win(Player::One)) => s.push_str("Atacatorii câștigă.\n"),
            Some(Outcome::Win(Player::Two)) => s.push_str("Apărătorii câștigă.\n"),
            Some(Outcome::Draw) => s.push_str("Remiză (limita de mutări).\n"),
            None => s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1)),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        format!(
            "{}-{}",
            self.name(m.from as usize),
            self.name(m.to as usize)
        )
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBetaTime {
            ms: 600,
            max_depth: if self.n() == 7 { 6 } else { 4 },
        }
    }

    fn hash_key(&self) -> Option<u64> {
        let mut h = self.side.idx() as u64 + 77 * self.plies.min(MAX_PLIES) as u64;
        for (i, &v) in self.cell.iter().enumerate() {
            if v != 0 {
                h ^= hash_mix(((i as u64) << 4) | v as u64);
            }
        }
        Some(h)
    }

    fn move_score(&self, m: &Move) -> i32 {
        if self.cell[m.from as usize] == KING {
            10
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
    fn setup_counts() {
        let g = Tafl::new(HNEFATAFL);
        assert_eq!((g.count(ATT), g.count(DEF), g.count(KING)), (24, 12, 1));
        let b = Tafl::new(BRANDUBH);
        assert_eq!((b.count(ATT), b.count(DEF), b.count(KING)), (8, 4, 1));
    }

    #[test]
    fn custodial_capture() {
        let mut g = Tafl::new(BRANDUBH);
        g.cell = vec![0; 49];
        g.cell[3 * 7 + 3] = KING;
        g.cell[7 + 1] = DEF;
        g.cell[1 * 7 + 0] = ATT;
        g.cell[1 * 7 + 3] = ATT; // mută pe b? (1,2) încadrează (1,1) între (1,2) și (1,0)
        g.side = Player::One;
        g.apply(&Move {
            from: (1 * 7 + 3) as u16,
            to: (1 * 7 + 2) as u16,
        });
        assert_eq!(g.cell[7 + 1], 0);
    }

    #[test]
    fn king_reaches_corner() {
        let mut g = Tafl::new(BRANDUBH);
        g.cell = vec![0; 49];
        g.cell[3] = KING;
        g.cell[3 * 7 + 3] = ATT;
        g.side = Player::Two;
        g.apply(&Move { from: 3, to: 0 });
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
    }

    #[test]
    fn king_captured_two_sided_brandubh() {
        let mut g = Tafl::new(BRANDUBH);
        g.cell = vec![0; 49];
        g.cell[7 + 1] = KING;
        g.cell[1 * 7 + 0] = ATT;
        g.cell[5 * 7 + 2] = ATT;
        g.side = Player::One;
        g.apply(&Move {
            from: (5 * 7 + 2) as u16,
            to: (1 * 7 + 2) as u16,
        });
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn random_games_end() {
        for rules in [HNEFATAFL, BRANDUBH] {
            for seed in 1..=4u64 {
                let mut g = Tafl::new(rules);
                let mut rng = Rng::new(seed);
                while g.outcome().is_none() {
                    let m = g.legal_moves();
                    assert!(!m.is_empty());
                    g.apply(&m[rng.below(m.len())]);
                }
            }
        }
    }

    #[test]
    fn ai_plays_legal() {
        use polytabletop_core::GameObj;
        polytabletop_core::ai::set_time_budget_override_ms(Some(15));
        let mut g: Box<dyn GameObj> = Box::new(Tafl::new(BRANDUBH));
        let mut rng = Rng::new(3);
        for _ in 0..30 {
            if g.outcome().is_some() {
                break;
            }
            let m = g.ai_move(rng.next_u64()).unwrap();
            g.play(&m).unwrap();
        }
    }
}
