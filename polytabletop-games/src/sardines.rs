#![allow(clippy::needless_range_loop, clippy::manual_is_multiple_of)]
//! Sardines (Sardine) — ascunsul inversat, versiune pentru 2 jucători cu informație ascunsă.
//!
//! Casa are 9 camere într-o grilă 3×3 (1–9, rândul de sus întâi). Două runde: în runda 1 jucătorul 1 se ascunde
//! și jucătorul 2 caută; în runda 2 se schimbă rolurile. Ascunsul alege în secret camera de start (`ascund N`,
//! nu poate fi camera 1, unde pornește căutătorul). Căutătorul face pe rând un pas în camera vecină (`ir N`, doar
//! pe laterale); mutarea forțată `asculta` arată dacă ascunsul a fost găsit (căutătorul a intrat în camera lui) sau, altfel, un
//! indiciu public: «aud respirații aproape» (ascunsul e într-o cameră vecină, inclusiv diagonal) sau «liniște».
//! După fiecare pas ascunsul poate rămâne (`sta`) sau trece într-o cameră vecină neocupată de căutător (`ma N`).
//! Camerele rămase posibile pentru ascuns sunt marcate cu «?» (se deduc doar din indicii, deci sunt publice).
//! Ascunsul primește atâtea puncte câți pași a făcut căutătorul până l-a găsit (maximum 10 pași; neprins = 13).
//! Câștigă cine are mai multe puncte după cele două runde. AI-ul căutător folosește doar camerele posibile «?»;
//! AI-ul ascuns își folosește propria cameră și poziția căutătorului.

use crate::cards::{handover_banner, Viewer};
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const MAX_STEPS: u8 = 10;
const SURVIVE_BONUS: u8 = 13;
const ALL: u16 = 0x1FF;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Hide(u8),
    Go(u8),
    Listen,
    Stay,
    Shift(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Hide,
    Seek,
    Listen,
    Relocate,
    Over,
}

#[derive(Clone)]
pub struct Sardines {
    round: u8,
    hider_room: u8,
    seeker_room: u8,
    belief: u16,
    steps: u8,
    scores: [u32; 2],
    phase: Phase,
    prev: Player,
    log: String,
    viewer: Viewer,
}

impl Default for Sardines {
    fn default() -> Self {
        Self::new()
    }
}

fn adj(r: u8) -> Vec<u8> {
    let (x, y) = ((r % 3) as i8, (r / 3) as i8);
    let mut v = Vec::new();
    for (dx, dy) in [(0, -1), (-1, 0), (1, 0), (0, 1)] {
        let (nx, ny) = (x + dx, y + dy);
        if (0..3).contains(&nx) && (0..3).contains(&ny) {
            v.push((ny * 3 + nx) as u8);
        }
    }
    v
}

fn near(a: u8, b: u8) -> bool {
    let (ax, ay) = ((a % 3) as i8, (a / 3) as i8);
    let (bx, by) = ((b % 3) as i8, (b / 3) as i8);
    (ax - bx).abs().max((ay - by).abs()) <= 1
}

fn expand(mask: u16) -> u16 {
    let mut out = mask;
    for r in 0..9u8 {
        if mask & (1 << r) != 0 {
            for a in adj(r) {
                out |= 1 << a;
            }
        }
    }
    out
}

impl Sardines {
    pub fn new() -> Self {
        Sardines {
            round: 1,
            hider_room: 0,
            seeker_room: 0,
            belief: ALL & !1,
            steps: 0,
            scores: [0; 2],
            phase: Phase::Hide,
            prev: Player::One,
            log: String::new(),
            viewer: Viewer::Fixed(Player::One),
        }
    }

    fn hider(&self) -> Player {
        if self.round == 1 {
            Player::One
        } else {
            Player::Two
        }
    }

    fn seeker(&self) -> Player {
        self.hider().other()
    }

    pub fn scores(&self) -> (u32, u32) {
        (self.scores[0], self.scores[1])
    }

    fn end_round(&mut self, pts: u32) {
        self.scores[self.hider().idx()] += pts;
        if self.round >= 2 {
            self.phase = Phase::Over;
        } else {
            self.round = 2;
            self.hider_room = 0;
            self.seeker_room = 0;
            self.belief = ALL & !1;
            self.steps = 0;
            self.phase = Phase::Hide;
        }
    }
}

impl Game for Sardines {
    type Move = Move;

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = match who {
            Some(p) => Viewer::Fixed(p),
            None => Viewer::HotSeat,
        };
    }

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Hide | Phase::Relocate => self.hider(),
            _ => self.seeker(),
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        match self.phase {
            Phase::Over => Vec::new(),
            Phase::Hide => (1..9u8).map(|r| Move::Hide(r + 1)).collect(),
            Phase::Seek => adj(self.seeker_room)
                .into_iter()
                .map(|r| Move::Go(r + 1))
                .collect(),
            Phase::Listen => vec![Move::Listen],
            Phase::Relocate => {
                let mut v = vec![Move::Stay];
                for a in adj(self.hider_room) {
                    if a != self.seeker_room {
                        v.push(Move::Shift(a + 1));
                    }
                }
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        self.prev = self.to_move();
        match *m {
            Move::Hide(r) => {
                self.hider_room = r - 1;
                self.phase = Phase::Seek;
            }
            Move::Go(r) => {
                self.seeker_room = r - 1;
                self.steps += 1;
                self.phase = Phase::Listen;
            }
            Move::Listen => {
                if self.seeker_room == self.hider_room {
                    self.log = format!(
                        "Găsit în camera {} după {} pași!",
                        self.hider_room + 1,
                        self.steps
                    );
                    let pts = self.steps as u32;
                    self.end_round(pts);
                    return;
                }
                let warm = near(self.seeker_room, self.hider_room);
                self.belief &= !(1 << self.seeker_room);
                let mut hint = 0u16;
                for r in 0..9u8 {
                    if near(self.seeker_room, r) == warm {
                        hint |= 1 << r;
                    }
                }
                self.belief &= hint;
                if self.steps >= MAX_STEPS {
                    self.log =
                        format!("Ascunsul a rezistat! Era în camera {}", self.hider_room + 1);
                    self.end_round(SURVIVE_BONUS as u32);
                    return;
                }
                self.log = if warm {
                    "Se aud respirații aproape...".into()
                } else {
                    "Liniște.".into()
                };
                self.phase = Phase::Relocate;
            }
            Move::Stay | Move::Shift(_) => {
                if let Move::Shift(r) = *m {
                    self.hider_room = r - 1;
                }
                self.belief = expand(self.belief) & !(1 << self.seeker_room);
                self.phase = Phase::Seek;
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        Some(match self.scores[0].cmp(&self.scores[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let base = (self.scores[p.idx()] as i32 - self.scores[p.other().idx()] as i32) * 1000;
        if self.phase == Phase::Over {
            return base;
        }
        let b = self.belief.count_ones().max(1) as i32;
        if p == self.seeker() {
            // doar informația publică: camerele posibile
            let mut v = base;
            if self.phase == Phase::Listen {
                if self.belief & (1 << self.seeker_room) != 0 {
                    v += 1000 / b;
                }
                let covered = (0..9u8)
                    .filter(|&r| self.belief & (1 << r) != 0 && near(self.seeker_room, r))
                    .count() as i32;
                v += 5 * covered;
            }
            v
        } else {
            // ascunsul își cunoaște camera
            let mut v = base;
            let d = |a: u8, c: u8| {
                let (ax, ay) = ((a % 3) as i32, (a / 3) as i32);
                let (cx, cy) = ((c % 3) as i32, (c / 3) as i32);
                (ax - cx).abs() + (ay - cy).abs()
            };
            let me = self.hider_room;
            let s = self.seeker_room;
            v += 30 * d(me, s);
            if adj(s).contains(&me) {
                v -= 200;
            }
            if near(s, me) {
                v -= 40;
            }
            v
        }
    }

    fn render(&self) -> String {
        let mut s = String::new();
        if self.prev != self.to_move() && self.phase != Phase::Over {
            if let Viewer::HotSeat = self.viewer {
                s.push_str(&handover_banner(self.to_move()));
            }
        }
        s.push_str(&format!(
            "Sardines — runda {}/2 | ascunde J{}, caută J{} | puncte: J1 {}, J2 {}\n",
            self.round,
            self.hider().idx() + 1,
            self.seeker().idx() + 1,
            self.scores[0],
            self.scores[1]
        ));
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        if self.phase == Phase::Over {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        let v = self.viewer.who(self.to_move());
        let sees_hider = v == self.hider() && self.phase != Phase::Hide;
        for y in 0..3u8 {
            for x in 0..3u8 {
                let r = y * 3 + x;
                let mut c = String::new();
                if self.phase != Phase::Hide && r == self.seeker_room {
                    c.push('S');
                }
                if sees_hider && r == self.hider_room {
                    c.push('H');
                }
                if self.phase != Phase::Hide && self.belief & (1 << r) != 0 {
                    c.push('?');
                }
                if c.is_empty() {
                    c.push('.');
                }
                s.push_str(&format!("{}:{:<3} ", r + 1, c));
            }
            s.push('\n');
        }
        s.push_str(&format!("Pași căutător: {}/{}\n", self.steps, MAX_STEPS));
        s.push_str(&format!(
            "La mutare: Jucătorul {}\n",
            self.to_move().idx() + 1
        ));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Hide(r) => format!("ascund {r}"),
            Move::Go(r) => format!("ir {r}"),
            Move::Listen => "asculta".into(),
            Move::Stay => "sta".into(),
            Move::Shift(r) => format!("ma {r}"),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    #[test]
    fn found_scores_hider() {
        let mut g = Sardines::new();
        g.apply(&Move::Hide(2)); // camera 2 (index 1) e vecină cu 1
        assert_eq!(g.to_move(), Player::Two);
        g.apply(&Move::Go(2));
        g.apply(&Move::Listen);
        assert_eq!(g.scores(), (1, 0));
        assert_eq!(g.round, 2);
        assert_eq!(g.to_move(), Player::Two);
    }

    #[test]
    fn hint_narrows_belief() {
        let mut g = Sardines::new();
        g.apply(&Move::Hide(9));
        g.apply(&Move::Go(2));
        g.apply(&Move::Listen);
        // ascunsul e departe: nu mai sunt posibile camerele vecine cu camera 2
        for r in 0..9u8 {
            if near(1, r) {
                assert_eq!(g.belief & (1 << r), 0);
            }
        }
        assert!(g.belief & (1 << 8) != 0);
    }

    #[test]
    fn seeker_evaluation_ignores_hidden_room() {
        let mut a = Sardines::new();
        a.apply(&Move::Hide(9));
        a.apply(&Move::Go(2));
        let mut b = a.clone();
        b.hider_room = 4;
        for m in a.legal_moves() {
            let (mut x, mut y) = (a.clone(), b.clone());
            x.apply(&m);
            y.apply(&m);
            assert_eq!(x.evaluate(Player::Two), y.evaluate(Player::Two));
        }
        assert_eq!(a.evaluate(Player::Two), b.evaluate(Player::Two));
    }

    #[test]
    fn viewer_hides_room() {
        let mut g = Sardines::new();
        g.apply(&Move::Hide(9));
        g.set_viewer(Some(Player::One));
        assert!(g.render().contains("H"));
        g.set_viewer(Some(Player::Two));
        assert!(!g.render().contains('H'));
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=6u64 {
            let mut g = Sardines::new();
            let mut rng = Rng::new(seed);
            let mut k = 0;
            while g.outcome().is_none() && k < 4000 {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
                k += 1;
            }
            assert!(g.outcome().is_some());
        }
    }
}
