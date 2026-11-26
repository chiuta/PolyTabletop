//! Pachisi (varianta cu cauri), 2 jucători, 4 piese fiecare.
//!
//! * Șase cauri aruncate în stare, cu `Rng`-ul jocului: numărul de cauri cu
//!   deschiderea în sus dă mutarea: 1 → 10, 2 → 2, 3 → 3, 4 → 4, 5 → 5,
//!   6 → 6, 0 → 25 (tabel verificat: en.wikipedia.org/wiki/Pachisi, bead.game,
//!   franpritchett.com după Falkener). Valorile 6, 10 și 25 sunt „grațioase”: dau dreptul la
//!   încă o aruncare (chiar dacă nu ai mutare) și permit scoaterea unei piese
//!   din bază pe pătratul de start (mutarea `intră`; piesa nu avansează în
//!   aceeași mutare).
//! * Pista comună are 48 de pătrate; Unu pornește de la 0, Doi de la 24. O
//!   piesă are „progres”: 0 = în bază, 1..=47 = pe pistă (1 = pătratul de
//!   start), 48..=53 = culoarul de acasă, 54 = centrul (țintă, exactă).
//! * Pătratele 0, 6, 12, …, 42 sunt sigure („castele”): acolo piesele nu se
//!   captură și se pot afla piese ale ambilor jucători. Pe un pătrat nesigur,
//!   aterizarea pe o piesă singură a adversarului o trimite în bază și îți dă
//!   încă o aruncare. Două piese ale aceluiași jucător pe un pătrat de pistă
//!   formează un bloc: adversarul nu poate ateriza acolo (poate trece peste).
//! * Câștigă cine aduce toate cele 4 piese în centru. De pe pistă ținta trebuie
//!   atinsă exact; din culoarul de acasă orice aruncare ≥ distanța duce în
//!   centru (ca nicio piesă să nu rămână blocată pentru totdeauna).
//!
//! Simplificări (documentate): doar cauri (fără variantă cu zaruri), pistă
//! scurtă în loc de crucea de 96 de pătrate, blocurile se pot sări, un singur
//! rând de „castele”. Dacă nu ai nicio mutare, tura trece automat.
//!
//! Mutări text: `intră`, sau progresul piesei pe care o muți (ex. `12`).
//! AI: `AlphaBeta(1)` pe progres total și expunere; nu vede aruncarea următoare.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const RING: usize = 48;
const LAST_RING: u8 = 47;
const GOAL: u8 = 54;

#[derive(Clone)]
pub struct Pachisi {
    prog: [[u8; 4]; 2],
    side: Player,
    roll: u8,
    rng: Rng,
    over: bool,
    log: String,
}

impl Default for Pachisi {
    fn default() -> Self {
        Self::new()
    }
}

fn start_off(p: Player) -> usize {
    match p {
        Player::One => 0,
        Player::Two => 24,
    }
}

fn ring_sq(p: Player, progress: u8) -> usize {
    (start_off(p) + progress as usize - 1) % RING
}

fn is_safe(sq: usize) -> bool {
    sq.is_multiple_of(6)
}

pub fn cowry_value(mouths_up: usize) -> u8 {
    match mouths_up {
        0 => 25,
        1 => 10,
        2 => 2,
        3 => 3,
        4 => 4,
        5 => 5,
        _ => 6,
    }
}

/// Progresul după mutare (> GOAL = mutare imposibilă). Din culoarul de acasă
/// orice aruncare ≥ distanța duce în centru; de pe pistă ținta e exactă.
fn target_of(pr: u8, roll: u8) -> u8 {
    let raw = pr as u16 + roll as u16;
    if pr > LAST_RING && raw > GOAL as u16 {
        GOAL
    } else {
        raw.min(255) as u8
    }
}

pub fn is_grace(v: u8) -> bool {
    matches!(v, 6 | 10 | 25)
}

impl Pachisi {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Pachisi {
            prog: [[0; 4]; 2],
            side: Player::One,
            roll: 0,
            rng: Rng::new(seed),
            over: false,
            log: String::new(),
        };
        g.begin_turn();
        g
    }

    pub fn pieces(&self, p: Player) -> [u8; 4] {
        self.prog[p.idx()]
    }

    pub fn current_roll(&self) -> u8 {
        self.roll
    }

    fn throw(&mut self) -> u8 {
        let k = (0..6).filter(|_| self.rng.below(2) == 1).count();
        cowry_value(k)
    }

    /// Câte piese ale lui `p` stau pe pătratul de pistă `sq`.
    fn count_on(&self, p: Player, sq: usize) -> usize {
        self.prog[p.idx()]
            .iter()
            .filter(|&&x| (1..=LAST_RING).contains(&x) && ring_sq(p, x) == sq)
            .count()
    }

    fn blocked(&self, me: Player, target: u8) -> bool {
        target <= LAST_RING && self.count_on(me.other(), ring_sq(me, target)) >= 2
    }

    fn moves(&self) -> Vec<u8> {
        let me = self.side;
        let mine = &self.prog[me.idx()];
        let mut out: Vec<u8> = Vec::new();
        for &pr in mine {
            if out.contains(&pr) || pr == GOAL {
                continue;
            }
            let target = if pr == 0 {
                if !is_grace(self.roll) {
                    continue;
                }
                1
            } else {
                target_of(pr, self.roll)
            };
            if target > GOAL || self.blocked(me, target) {
                continue;
            }
            out.push(pr);
        }
        out.sort_unstable();
        out
    }

    fn begin_turn(&mut self) {
        loop {
            self.roll = self.throw();
            if !self.moves().is_empty() {
                return;
            }
            self.log.push_str(&format!(
                " Jucătorul {} a aruncat {} și nu poate muta.",
                self.side.idx() + 1,
                self.roll
            ));
            if !is_grace(self.roll) {
                self.side = self.side.other();
            }
        }
    }

    /// Progresul „în pericol” al pieselor lui `p`: piese singure pe pătrate
    /// nesigure, la distanță de lovire a unei piese adverse.
    fn exposed(&self, p: Player) -> i32 {
        let mut n = 0;
        for &x in &self.prog[p.idx()] {
            if !(1..=LAST_RING).contains(&x) {
                continue;
            }
            let sq = ring_sq(p, x);
            if is_safe(sq) || self.count_on(p, sq) >= 2 {
                continue;
            }
            let o = p.other();
            let hit = self.prog[o.idx()].iter().any(|&y| {
                if !(1..=LAST_RING).contains(&y) {
                    return false;
                }
                let from = ring_sq(o, y);
                let d = (sq + RING - from) % RING;
                matches!(d, 2 | 3 | 4 | 5 | 6 | 10 | 25)
            });
            if hit {
                n += x as i32;
            }
        }
        n
    }
}

impl Game for Pachisi {
    type Move = u8; // progresul piesei mutate (0 = intră din bază)

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<u8> {
        if self.over {
            Vec::new()
        } else {
            self.moves()
        }
    }

    fn apply(&mut self, m: &u8) {
        let me = self.side;
        let mi = me.idx();
        let rolled = self.roll;
        let target = if *m == 0 { 1 } else { target_of(*m, rolled) };
        let k = self.prog[mi].iter().position(|&p| p == *m).expect("piesă");
        self.prog[mi][k] = target;
        self.log = format!(
            "Jucătorul {}: {} cu {}.",
            mi + 1,
            self.format_move(m),
            rolled
        );
        let mut captured = false;
        if target <= LAST_RING {
            let sq = ring_sq(me, target);
            if !is_safe(sq) {
                let o = me.other();
                for p in self.prog[o.idx()].iter_mut() {
                    if (1..=LAST_RING).contains(p) && ring_sq(o, *p) == sq {
                        *p = 0;
                        captured = true;
                    }
                }
            }
        }
        if captured {
            self.log.push_str(" Captură!");
        }
        if self.prog[mi].iter().all(|&p| p == GOAL) {
            self.over = true;
            return;
        }
        if is_grace(rolled) || captured {
            self.log.push_str(" Aruncă din nou.");
        } else {
            self.side = me.other();
        }
        self.begin_turn();
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            return None;
        }
        let one = self.prog[0].iter().all(|&p| p == GOAL);
        Some(Outcome::Win(if one { Player::One } else { Player::Two }))
    }

    fn evaluate(&self, p: Player) -> i32 {
        let s = |q: Player| -> i32 {
            self.prog[q.idx()]
                .iter()
                .map(|&x| x as i32 + if x == GOAL { 10 } else { 0 })
                .sum()
        };
        let out = |q: Player| -> i32 {
            self.prog[q.idx()]
                .iter()
                .map(|&x| {
                    let mut v = 0;
                    if x > 0 {
                        v += 12; // piesă scoasă din bază
                    }
                    if x > 0 && x <= LAST_RING && is_safe(ring_sq(q, x)) {
                        v += 4;
                    }
                    v
                })
                .sum()
        };
        s(p) - s(p.other()) + out(p) - out(p.other()) - self.exposed(p) / 3
            + self.exposed(p.other()) / 3
    }

    fn render(&self) -> String {
        let mut s = String::new();
        if !self.log.is_empty() {
            s.push_str(self.log.trim());
            s.push('\n');
        }
        let ch = |sq: usize| -> char {
            let a = self.count_on(Player::One, sq);
            let b = self.count_on(Player::Two, sq);
            match (a, b) {
                (0, 0) => {
                    if is_safe(sq) {
                        '+'
                    } else {
                        '.'
                    }
                }
                (_, 0) => {
                    if a >= 2 {
                        'Y'
                    } else {
                        'X'
                    }
                }
                (0, _) => {
                    if b >= 2 {
                        'Q'
                    } else {
                        'O'
                    }
                }
                _ => '*',
            }
        };
        s.push_str("Pista (0 = startul lui J1, 24 = startul lui J2; + = castel sigur):\n");
        for half in 0..2 {
            s.push_str(&format!("{:>2}: ", half * 24));
            for sq in half * 24..half * 24 + 24 {
                s.push(ch(sq));
            }
            s.push('\n');
        }
        s.push_str("X/O = piesă, Y/Q = bloc de două, * = piese ale ambilor pe un castel\n");
        for p in [Player::One, Player::Two] {
            let v = self.prog[p.idx()];
            let base = v.iter().filter(|&&x| x == 0).count();
            let done = v.iter().filter(|&&x| x == GOAL).count();
            let mut on: Vec<String> = v
                .iter()
                .filter(|&&x| x > 0 && x < GOAL)
                .map(|x| x.to_string())
                .collect();
            on.sort();
            s.push_str(&format!(
                "Jucătorul {} ({}): bază {}, pe drum [{}], acasă {}\n",
                p.idx() + 1,
                if p == Player::One { "X" } else { "O" },
                base,
                on.join(" "),
                done
            ));
        }
        if !self.over {
            s.push_str(&format!(
                "Jucătorul {} a aruncat {}{}. Mută: {}\n",
                self.side.idx() + 1,
                self.roll,
                if is_grace(self.roll) {
                    " (grațios)"
                } else {
                    ""
                },
                self.moves()
                    .iter()
                    .map(|m| self.format_move(m))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        s
    }

    fn format_move(&self, m: &u8) -> String {
        if *m == 0 {
            "intră".to_string()
        } else {
            m.to_string()
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn custom(one: [u8; 4], two: [u8; 4], side: Player, roll: u8) -> Pachisi {
        Pachisi {
            prog: [one, two],
            side,
            roll,
            rng: Rng::new(1),
            over: false,
            log: String::new(),
        }
    }

    #[test]
    fn cowry_table_and_grace() {
        let v: Vec<u8> = (0..=6).map(cowry_value).collect();
        assert_eq!(v, vec![25, 10, 2, 3, 4, 5, 6]);
        assert!(is_grace(25) && is_grace(10) && is_grace(6));
        assert!(!is_grace(2) && !is_grace(3) && !is_grace(4) && !is_grace(5) && !is_grace(12));
    }

    #[test]
    fn entering_requires_a_grace_throw() {
        let g = custom([0, 0, 0, 0], [0; 4], Player::One, 3);
        assert!(g.moves().is_empty());
        let g = custom([0, 0, 0, 0], [0; 4], Player::One, 25);
        assert_eq!(g.moves(), vec![0]);
    }

    #[test]
    fn grace_throw_gives_another_throw() {
        let mut g = custom([5, 0, 0, 0], [0; 4], Player::One, 10);
        g.apply(&5);
        assert_eq!(g.to_move(), Player::One);
        let mut h = custom([5, 0, 0, 0], [0; 4], Player::One, 3);
        h.apply(&5);
        assert_ne!(h.pieces(Player::One)[0], 5);
    }

    #[test]
    fn capture_on_non_safe_square_and_bonus_throw() {
        // progres 5 al lui J1 = pătratul 4; progres 29 al lui J2 = pătratul 4
        let mut g = custom([2, 0, 0, 0], [29, 0, 0, 0], Player::One, 3);
        g.apply(&2);
        assert_eq!(g.pieces(Player::Two)[0], 0);
        assert_eq!(g.to_move(), Player::One); // bonus de captură
                                              // pătrat diferit: fără captură
        let mut h = custom([2, 0, 0, 0], [30, 0, 0, 0], Player::One, 3);
        h.apply(&2);
        assert_eq!(h.pieces(Player::Two)[0], 30);
    }

    #[test]
    fn safe_square_cannot_be_captured() {
        // pătratul 6 e sigur: J1 progres 7; J2 progres p cu (24+p-1)%48=6 → p=31
        let mut g = custom([4, 0, 0, 0], [31, 0, 0, 0], Player::One, 3);
        g.apply(&4);
        assert_eq!(g.pieces(Player::Two)[0], 31);
        assert!(g.pieces(Player::One).contains(&7));
    }

    #[test]
    fn blocks_cannot_be_landed_on() {
        // J2 are două piese pe pătratul 5 (progres 29)
        let g = custom([3, 0, 0, 0], [30, 30, 0, 0], Player::One, 3);
        assert!(g.moves().is_empty());
        // pe un castel blocul interzice și el
        let g = custom([4, 0, 0, 0], [31, 31, 0, 0], Player::One, 3);
        assert!(g.moves().is_empty());
    }

    #[test]
    fn exact_from_track_but_lane_accepts_overshoot() {
        let g = custom([47, 54, 54, 54], [0; 4], Player::One, 10);
        assert!(g.moves().is_empty()); // 57 > 54 de pe pistă
        let mut g = custom([53, 0, 0, 0], [0; 4], Player::One, 3);
        assert_eq!(g.moves(), vec![53]);
        g.apply(&53);
        assert_eq!(g.pieces(Player::One)[0], GOAL);
    }

    #[test]
    fn win_when_all_four_home() {
        let mut g = custom([54, 54, 54, 52], [0; 4], Player::One, 2);
        g.apply(&52);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn random_games_finish_and_keep_four_pieces() {
        for seed in 1..=15u64 {
            let mut g = Pachisi::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = Game::legal_moves(&g);
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 20_000);
                for p in [Player::One, Player::Two] {
                    assert!(g.pieces(p).iter().all(|&x| x <= GOAL));
                }
            }
        }
    }
}
