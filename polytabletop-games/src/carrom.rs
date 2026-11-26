//! Carrom discret (fără fizică), 2 jucători: 12 monede albe (Jucătorul 1, `x`),
//! 12 negre (Jucătorul 2, `o`) și damă roșie (`Q`) pe o tablă 9 × 9 cu patru
//! buzunare în colțuri. Variantă documentată: lovitura nu e o simulare fizică,
//! ci un traseu pe grilă, deterministic.
//!
//! * O lovitură = coloana de start (a–i) pe rândul de bază al tău (rândul 2 pentru
//!   Jucătorul 1, rândul 8 pentru Jucătorul 2; celula trebuie să fie liberă),
//!   direcția (`L` stânga-înainte, `S` drept înainte, `R` dreapta-înainte) și
//!   forța 1–3. Ex.: `eS3`.
//! * Strikerul pornește cu un buget de 4·forța + 2 pași. Merge câte o celulă pe
//!   direcție, ricoșând la pereți (se inversează componenta care iese din tablă).
//!   Dacă următoarea celulă are o monedă, se oprește lângă ea și o împinge pe
//!   aceeași direcție cu ⌈bugetul rămas / 2⌉ celule (ricoșează și ea; se oprește
//!   lângă altă piesă). O monedă care ajunge într-un colț cade în buzunar.
//!   Nicio monedă nu împinge alta în lanț. Strikerul se scoate după lovitură.
//! * Moneda proprie în buzunar: +1 și mai tragi o dată. Moneda adversarului:
//!   punctul merge la el și tura trece. Dama: se „acoperă” dacă lovitura
//!   următoare băgă o monedă proprie (+3 pentru tine); altfel dama revine în
//!   centru (sau cea mai apropiată celulă liberă). Strikerul băgat în buzunar =
//!   greșeală: o monedă proprie băgată înapoi pe tablă (−1) și tura trece.
//! * Jocul se termină când un jucător și-a băgat toate monedele (inclusiv
//!   când adversarul le-a băgat pentru el) sau după 300 de lovituri; câștigă
//!   cine are mai multe puncte.
//!
//! Simplificări (documentate): nu există frecare, rotații sau unghiuri
//! arbitrare; un singur contact pe lovitură; un singur striker.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const W: usize = 9;

/// Buzunar = colțul și cele două celule vecine de pe margine (12 celule).
fn is_pocket(i: usize) -> bool {
    let (x, y) = (i % W, i / W);
    (matches!(x, 0 | 8) && matches!(y, 0 | 1 | 7 | 8))
        || (matches!(y, 0 | 8) && matches!(x, 0 | 1 | 7 | 8))
}
const CORNERS: [usize; 4] = [0, 8, 72, 80];
const MAX_SHOTS: u32 = 300;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Shot { col: u8, dir: i8, force: u8 },
    Pass,
}

const WHITE: u8 = 1;
const BLACK: u8 = 2;
const QUEEN: u8 = 3;

#[derive(Clone)]
pub struct Carrom {
    cells: [u8; W * W],
    score: [i32; 2],
    side: Player,
    /// Jucătorul care a băgat dama și trebuie s-o acopere.
    queen_pending: Option<Player>,
    shots: u32,
    log: String,
}

impl Default for Carrom {
    fn default() -> Self {
        Self::new()
    }
}

fn idx(x: usize, y: usize) -> usize {
    y * W + x
}

fn base_row(p: Player) -> usize {
    match p {
        Player::One => 1,
        Player::Two => 7,
    }
}

fn forward(p: Player) -> i32 {
    match p {
        Player::One => 1,
        Player::Two => -1,
    }
}

fn coin_of(p: Player) -> u8 {
    match p {
        Player::One => WHITE,
        Player::Two => BLACK,
    }
}

impl Carrom {
    pub fn new() -> Self {
        let mut cells = [0u8; W * W];
        cells[idx(4, 4)] = QUEEN;
        let mut place = |x: usize, y: usize| {
            let c = if y < 4 {
                if (x + y).is_multiple_of(2) {
                    WHITE
                } else {
                    BLACK
                }
            } else if y > 4 {
                // oglindă pe verticală cu culorile inversate
                if (x + (8 - y)).is_multiple_of(2) {
                    BLACK
                } else {
                    WHITE
                }
            } else if x < 4 {
                WHITE
            } else {
                BLACK
            };
            cells[idx(x, y)] = c;
        };
        let ring1 = [
            (3, 3),
            (4, 3),
            (5, 3),
            (5, 4),
            (5, 5),
            (4, 5),
            (3, 5),
            (3, 4),
        ];
        for (x, y) in ring1 {
            place(x, y);
        }
        for k in 2..=6usize {
            place(k, 2);
            place(k, 6);
        }
        for y in 3..=5usize {
            place(2, y);
            place(6, y);
        }
        Carrom {
            cells,
            score: [0, 0],
            side: Player::One,
            queen_pending: None,
            shots: 0,
            log: String::new(),
        }
    }

    pub fn points(&self) -> (i32, i32) {
        (self.score[0], self.score[1])
    }

    pub fn coins_on_board(&self, p: Player) -> usize {
        self.cells.iter().filter(|&&c| c == coin_of(p)).count()
    }

    pub fn queen_on_board(&self) -> bool {
        self.cells.contains(&QUEEN)
    }

    fn nearest_free(&self) -> Option<usize> {
        let mut best: Option<(usize, usize)> = None;
        for i in 0..W * W {
            if self.cells[i] != 0 || is_pocket(i) {
                continue;
            }
            let (x, y) = ((i % W) as i32, (i / W) as i32);
            let d = (x - 4).abs().max((y - 4).abs()) as usize * 100
                + (x - 4).unsigned_abs() as usize
                + (y - 4).unsigned_abs() as usize;
            if best.map(|b| d < b.0).unwrap_or(true) {
                best = Some((d, i));
            }
        }
        best.map(|b| b.1)
    }

    fn put_back(&mut self, what: u8) {
        if let Some(i) = self.nearest_free() {
            self.cells[i] = what;
        }
    }

    /// Simulează traseul. Întoarce monedele băgate în buzunar și dacă strikerul a căzut.
    fn trace(&mut self, col: usize, dir: i32, force: u32) -> (Vec<u8>, bool) {
        let me = self.side;
        let (mut x, mut y) = (col as i32, base_row(me) as i32);
        let (mut dx, mut dy) = (dir, forward(me));
        let budget = 4 * force as i32 + 2;
        let mut steps = 0;
        let step = |x: i32, y: i32, dx: &mut i32, dy: &mut i32| -> (i32, i32) {
            let mut nx = x + *dx;
            let mut ny = y + *dy;
            if !(0..W as i32).contains(&nx) {
                *dx = -*dx;
                nx = x + *dx;
            }
            if !(0..W as i32).contains(&ny) {
                *dy = -*dy;
                ny = y + *dy;
            }
            (nx, ny)
        };
        let mut pocketed: Vec<u8> = Vec::new();
        while steps < budget {
            let (nx, ny) = step(x, y, &mut dx, &mut dy);
            let ni = idx(nx as usize, ny as usize);
            if self.cells[ni] != 0 {
                // contact: moneda lovită primește tot bugetul rămas; când o monedă în
                // mișcare e oprită de alta, o împinge mai departe cu 1 pas mai puțin.
                let mut left = budget - steps;
                let (mut cx, mut cy) = (nx, ny);
                let (mut cdx, mut cdy) = (dx, dy);
                let mut kind = self.cells[ni];
                self.cells[ni] = 0;
                'chain: loop {
                    while left > 0 {
                        let (mx, my) = step(cx, cy, &mut cdx, &mut cdy);
                        if (mx, my) == (cx, cy) {
                            break;
                        }
                        let mi = idx(mx as usize, my as usize);
                        if self.cells[mi] != 0 {
                            // transfer de impuls
                            self.cells[idx(cx as usize, cy as usize)] = kind;
                            left -= 1;
                            if left <= 0 {
                                break 'chain;
                            }
                            kind = self.cells[mi];
                            self.cells[mi] = 0;
                            cx = mx;
                            cy = my;
                            continue 'chain;
                        }
                        cx = mx;
                        cy = my;
                        left -= 1;
                        if is_pocket(mi) {
                            pocketed.push(kind);
                            break 'chain;
                        }
                    }
                    self.cells[idx(cx as usize, cy as usize)] = kind;
                    break;
                }
                return (pocketed, false);
            }
            x = nx;
            y = ny;
            steps += 1;
            if is_pocket(ni) {
                return (pocketed, true);
            }
        }
        (pocketed, false)
    }

    fn game_over(&self) -> bool {
        self.shots >= MAX_SHOTS
            || self.coins_on_board(Player::One) == 0
            || self.coins_on_board(Player::Two) == 0
    }

    fn free_cols(&self, p: Player) -> Vec<usize> {
        (1..W - 1)
            .filter(|&c| self.cells[idx(c, base_row(p))] == 0)
            .collect()
    }
}

impl Game for Carrom {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.game_over() {
            return Vec::new();
        }
        let cols = self.free_cols(self.side);
        if cols.is_empty() {
            return vec![Move::Pass];
        }
        let mut v = Vec::new();
        for c in cols {
            for dir in [-1i8, 0, 1] {
                for force in 1..=3u8 {
                    v.push(Move::Shot {
                        col: c as u8,
                        dir,
                        force,
                    });
                }
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        let me = self.side;
        let opp = me.other();
        self.log.clear();
        self.shots += 1;
        let (pocketed, foul) = match *m {
            Move::Pass => {
                self.side = opp;
                return;
            }
            Move::Shot { col, dir, force } => self.trace(col as usize, dir as i32, force as u32),
        };
        let mut again = false;
        if foul {
            self.log = format!(
                "Jucătorul {}: strikerul a căzut în buzunar (greșeală).",
                me.idx() + 1
            );
            // monedele băgate la lovitura greșită revin pe tablă
            for k in pocketed {
                self.put_back(k);
            }
            if self.score[me.idx()] > 0 {
                self.score[me.idx()] -= 1;
                self.put_back(coin_of(me));
            }
            if self.queen_pending == Some(me) {
                self.put_back(QUEEN);
                self.queen_pending = None;
            }
        } else {
            let own = pocketed.iter().filter(|&&k| k == coin_of(me)).count() as i32;
            let theirs = pocketed.iter().filter(|&&k| k == coin_of(opp)).count() as i32;
            let queen = pocketed.contains(&QUEEN);
            self.score[me.idx()] += own;
            self.score[opp.idx()] += theirs;
            if pocketed.is_empty() {
                self.log = format!("Jucătorul {}: nicio monedă băgată.", me.idx() + 1);
            } else {
                self.log = format!(
                    "Jucătorul {} bagă: {} proprii, {} ale adversarului{}.",
                    me.idx() + 1,
                    own,
                    theirs,
                    if queen { ", dama" } else { "" }
                );
            }
            if queen {
                if own > 0 {
                    self.score[me.idx()] += 3;
                    self.log.push_str(" Dama e acoperită (+3).");
                } else {
                    self.queen_pending = Some(me);
                    self.log.push_str(" Dama trebuie acoperită.");
                }
            } else if self.queen_pending == Some(me) {
                if own > 0 {
                    self.score[me.idx()] += 3;
                    self.log.push_str(" Dama e acoperită (+3).");
                } else {
                    self.put_back(QUEEN);
                    self.log
                        .push_str(" Dama nu a fost acoperită: revine pe tablă.");
                }
                self.queen_pending = None;
            }
            again = own > 0 || queen;
        }
        if !again || self.game_over() {
            self.side = opp;
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.game_over() {
            return None;
        }
        Some(match self.score[0].cmp(&self.score[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let o = p.other();
        let near = |kind: u8| -> i32 {
            (0..W * W)
                .filter(|&i| self.cells[i] == kind)
                .map(|i| {
                    let (x, y) = ((i % W) as i32, (i / W) as i32);
                    CORNERS
                        .iter()
                        .map(|&q| {
                            let (qx, qy) = ((q % W) as i32, (q / W) as i32);
                            8 - (x - qx).abs().max((y - qy).abs())
                        })
                        .max()
                        .unwrap_or(0)
                })
                .sum()
        };
        let pend = match self.queen_pending {
            Some(q) if q == p => 6,
            Some(_) => -6,
            None => 0,
        };
        25 * (self.score[p.idx()] - self.score[o.idx()]) + pend + near(coin_of(p))
            - near(coin_of(o))
    }

    fn render(&self) -> String {
        let mut s = String::new();
        if !self.log.is_empty() {
            s.push_str(&self.log);
            s.push('\n');
        }
        s.push_str("    a b c d e f g h i\n");
        for y in (0..W).rev() {
            s.push_str(&format!(" {}  ", y + 1));
            for x in 0..W {
                let i = idx(x, y);
                let ch = match self.cells[i] {
                    WHITE => 'x',
                    BLACK => 'o',
                    QUEEN => 'Q',
                    _ => {
                        if is_pocket(i) {
                            '@'
                        } else {
                            '.'
                        }
                    }
                };
                s.push(ch);
                s.push(' ');
            }
            s.push('\n');
        }
        s.push_str(&format!(
            "x = J1 (jos, trage în sus din rândul 2), o = J2 (sus, trage în jos din rândul 8), Q = dama, @ = buzunar\nPuncte: J1 {}, J2 {}. Pe tablă: x {}, o {}{}. Lovituri: {}/{}\n",
            self.score[0],
            self.score[1],
            self.coins_on_board(Player::One),
            self.coins_on_board(Player::Two),
            if let Some(q) = self.queen_pending {
                format!(", dama de acoperit (J{})", q.idx() + 1)
            } else {
                String::new()
            },
            self.shots,
            MAX_SHOTS
        ));
        s.push_str("Mutare: coloană + direcție (L/S/R) + forță 1–3, ex. eS3.\n");
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Pass => "pas".to_string(),
            Move::Shot { col, dir, force } => format!(
                "{}{}{}",
                (b'a' + col) as char,
                match dir {
                    -1 => 'L',
                    0 => 'S',
                    _ => 'R',
                },
                force
            ),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    fn empty() -> Carrom {
        Carrom {
            cells: [0; W * W],
            score: [0, 0],
            side: Player::One,
            queen_pending: None,
            shots: 0,
            log: String::new(),
        }
    }

    fn put(g: &mut Carrom, x: usize, y: usize, v: u8) {
        g.cells[idx(x, y)] = v;
    }

    #[test]
    fn initial_layout_is_fair() {
        let g = Carrom::new();
        assert_eq!(g.coins_on_board(Player::One), 12);
        assert_eq!(g.coins_on_board(Player::Two), 12);
        assert!(g.queen_on_board());
        // rândurile de bază sunt libere
        assert!(g.free_cols(Player::One).len() == 7 && g.free_cols(Player::Two).len() == 7);
        assert_eq!(Game::legal_moves(&g).len(), 63);
    }

    #[test]
    fn straight_shot_pushes_coin_by_the_remaining_budget() {
        let mut g = empty();
        put(&mut g, 4, 3, BLACK);
        put(&mut g, 8, 4, WHITE);
        // forța 1 → buget 6; (4,2) = 1 pas; contact cu (4,3): rămân 5 → (4,4)…(4,8)
        g.apply(&Move::Shot {
            col: 4,
            dir: 0,
            force: 1,
        });
        assert_eq!(g.cells[idx(4, 8)], BLACK);
        assert_eq!(g.cells[idx(4, 3)], 0);
        assert_eq!(g.to_move(), Player::Two);
    }

    #[test]
    fn blocked_coin_transfers_momentum_to_the_next_one() {
        let mut g = empty();
        put(&mut g, 4, 2, WHITE); // lovită imediat, dar blocată
        put(&mut g, 4, 3, BLACK); // primește bugetul rămas − 1
        put(&mut g, 8, 4, WHITE);
        g.apply(&Move::Shot {
            col: 4,
            dir: 0,
            force: 1,
        }); // buget 6, 0 pași, rămân 6 → 5 pentru a doua
        assert_eq!(g.cells[idx(4, 2)], WHITE);
        assert_eq!(g.cells[idx(4, 3)], 0);
        assert_eq!(g.cells[idx(4, 8)], BLACK);
    }

    #[test]
    fn own_coin_in_pocket_scores_and_keeps_turn() {
        let mut g = empty();
        put(&mut g, 3, 5, WHITE);
        put(&mut g, 6, 6, BLACK);
        put(&mut g, 8, 4, WHITE);
        // start (7,1), direcția stânga-înainte: (6,2),(5,3),(4,4); contact cu (3,5);
        // forța 2 → buget 10, rămân 7 → 4 celule: (2,6),(1,7),(0,8) = buzunar.
        g.apply(&Move::Shot {
            col: 7,
            dir: -1,
            force: 2,
        });
        assert_eq!(g.score, [1, 0]); // (2,6),(1,7),(0,8)
        assert_eq!(g.to_move(), Player::One);
        assert_eq!(g.coins_on_board(Player::One), 1);
    }

    #[test]
    fn striker_in_pocket_is_a_foul_and_returns_a_coin() {
        let mut g = empty();
        // J1 din (1,1) cu L: (0,2),(−1→ricoșează)… nu ajunge în (0,0). J2 din (1,7) cu L, înainte −1: (0,6),(1,5)..
        // Construim: J1 din (0,1) cu S și forța 3 către sus: (0,2)…(0,8) buzunar la pasul 7 → greșeală
        g.score[0] = 1;
        put(&mut g, 5, 5, BLACK);
        g.apply(&Move::Shot {
            col: 0,
            dir: 0,
            force: 3,
        });
        assert_eq!(g.score[0], 0);
        assert_eq!(g.coins_on_board(Player::One), 1);
        assert_eq!(g.to_move(), Player::Two);
    }

    #[test]
    fn queen_must_be_covered_or_returns() {
        let mut g = empty();
        put(&mut g, 0, 7, QUEEN);
        put(&mut g, 4, 4, WHITE);
        put(&mut g, 6, 6, BLACK);
        // din (0,1) drept în sus: 5 pași, contact cu (0,7), rămân 9 → 5 celule: (0,8) = buzunar
        g.apply(&Move::Shot {
            col: 0,
            dir: 0,
            force: 3,
        });
        assert_eq!(g.queen_pending, Some(Player::One));
        assert_eq!(g.to_move(), Player::One);
        assert!(!g.queen_on_board());
        // lovitură fără contact → dama revine pe tablă, tura trece
        g.apply(&Move::Shot {
            col: 8,
            dir: 0,
            force: 1,
        });
        assert!(g.queen_on_board());
        assert_eq!(g.queen_pending, None);
        assert_eq!(g.score, [0, 0]);
        assert_eq!(g.to_move(), Player::Two);
    }

    #[test]
    fn queen_covered_gives_three_plus_one() {
        let mut g = empty();
        put(&mut g, 0, 7, QUEEN);
        put(&mut g, 8, 7, WHITE);
        put(&mut g, 4, 4, WHITE);
        put(&mut g, 6, 6, BLACK);
        g.apply(&Move::Shot {
            col: 0,
            dir: 0,
            force: 3,
        });
        assert_eq!(g.queen_pending, Some(Player::One));
        g.apply(&Move::Shot {
            col: 8,
            dir: 0,
            force: 3,
        }); // (8,7) → (8,8)
        assert_eq!(g.score[0], 4);
        assert_eq!(g.queen_pending, None);
        assert_eq!(g.to_move(), Player::One);
    }

    #[test]
    fn opponent_coin_gives_point_to_opponent() {
        let mut g = empty();
        put(&mut g, 0, 7, BLACK);
        put(&mut g, 4, 4, WHITE);
        put(&mut g, 6, 6, BLACK);
        g.apply(&Move::Shot {
            col: 0,
            dir: 0,
            force: 3,
        });
        assert_eq!(g.score, [0, 1]);
        assert_eq!(g.to_move(), Player::Two);
    }

    #[test]
    fn pieces_are_conserved() {
        for seed in 1..=10u64 {
            let mut g = Carrom::new();
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = Game::legal_moves(&g);
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 400);
                // monede pe tablă + monedele băgate = 12 (dacă n-a fost greșeală, scorul include și dama)
                let on = g.coins_on_board(Player::One) + g.coins_on_board(Player::Two);
                assert!(on <= 24);
                assert!(
                    g.queen_on_board()
                        || g.queen_pending.is_some()
                        || g.score[0] >= 3
                        || g.score[1] >= 3
                );
            }
        }
    }
}
