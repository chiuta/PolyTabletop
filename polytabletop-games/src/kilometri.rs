//! Kilometri (variantă proprie de cursă cu cărți de distanță, piedici, remedii și imunități,
//! 2 jucători, informație ascunsă): fiecare jucător ține 6 cărți și trage una la începutul turei.
//! Scop: primul care adună exact 1000 km (sau cine are mai mulți la epuizarea pachetului).
//! Ca să mergi ai nevoie de „verde”; piedicile (stop, pană, gol, accident) se pun pe adversar
//! doar când el rulează și îl opresc (stop) sau îl obligă să joace remediul potrivit (roata,
//! benzina, reparația) și apoi iar „verde”. „Limita” îl restrânge la cărți de 25 și 50 până
//! joacă „liber”. Imunitățile (prioritate = stop și limită, rezervă = pană, tanc = gol,
//! asigurare = accident) opresc piedicile pe viitor. Cărțile de 200 km se pot juca de cel mult
//! două ori; nu poți depăși 1000. O tură: joci o carte sau arunci una (`d <carte>`).
//! Informație ascunsă: vezi doar mâna ta.
//! Mutări: `50`, `200`, `stop`, `limita`, `pana`, `gol`, `accident`, `verde`, `liber`, `roata`,
//! `benzina`, `reparatie`, `prior`, `rez`, `tanc`, `asig`, `d 25`.
#![allow(clippy::all)]

use crate::cards::{handover_banner, Viewer};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const NAMES: [&str; 19] = [
    "25",
    "50",
    "75",
    "100",
    "200",
    "stop",
    "limita",
    "pana",
    "gol",
    "accident",
    "verde",
    "liber",
    "roata",
    "benzina",
    "reparatie",
    "prior",
    "rez",
    "tanc",
    "asig",
];
const KM: [i32; 5] = [25, 50, 75, 100, 200];
const COUNT: [usize; 19] = [10, 10, 10, 12, 4, 5, 4, 3, 3, 3, 14, 6, 6, 6, 6, 1, 1, 1, 1];

#[derive(Clone, Default)]
struct Tab {
    km: i32,
    twos: u8,
    hazard: Option<u8>, // 0 stop, 1 pană, 2 gol, 3 accident
    limit: bool,
    safe: [bool; 4], // prior, rez, tanc, asig
}

#[derive(Clone)]
pub struct Kilometri {
    hand: [Vec<u8>; 2],
    deck: Vec<u8>,
    tab: [Tab; 2],
    side: Player,
    prev: Player,
    viewer: Viewer,
    over: bool,
    ply: u32,
}

impl Default for Kilometri {
    fn default() -> Self {
        Self::new()
    }
}

impl Kilometri {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck = vec![];
        for (c, &n) in COUNT.iter().enumerate() {
            for _ in 0..n {
                deck.push(c as u8);
            }
        }
        for i in (1..deck.len()).rev() {
            deck.swap(i, rng.below(i + 1));
        }
        let h0: Vec<u8> = deck.drain(..6).collect();
        let h1: Vec<u8> = deck.drain(..6).collect();
        let mut g = Kilometri {
            hand: [h0, h1],
            deck,
            tab: [
                Tab {
                    hazard: Some(0),
                    ..Default::default()
                },
                Tab {
                    hazard: Some(0),
                    ..Default::default()
                },
            ],
            side: Player::One,
            prev: Player::One,
            viewer: Viewer::Fixed(Player::One),
            over: false,
            ply: 0,
        };
        g.hand[0].push(g.deck.pop().unwrap());
        g
    }
    fn can_play(&self, p: usize, c: u8) -> bool {
        let me = &self.tab[p];
        let op = &self.tab[1 - p];
        match c {
            0..=4 => {
                let d = KM[c as usize];
                me.hazard.is_none()
                    && me.km + d <= 1000
                    && !(c == 4 && me.twos >= 2)
                    && !(me.limit && d > 50)
            }
            5 => op.hazard.is_none() && !op.safe[0],
            6 => !op.limit && !op.safe[0],
            7 => op.hazard.is_none() && !op.safe[1],
            8 => op.hazard.is_none() && !op.safe[2],
            9 => op.hazard.is_none() && !op.safe[3],
            10 => me.hazard == Some(0),
            11 => me.limit,
            12 => me.hazard == Some(1),
            13 => me.hazard == Some(2),
            14 => me.hazard == Some(3),
            _ => !me.safe[c as usize - 15],
        }
    }
    fn heur(&self, p: usize) -> i32 {
        let t = &self.tab[p];
        t.km + t.safe.iter().filter(|&&b| b).count() as i32 * 60
            - if t.hazard.is_some() { 80 } else { 0 }
            - if t.limit { 40 } else { 0 }
    }
}

impl Game for Kilometri {
    type Move = (u8, bool); // carte, aruncare
    fn to_move(&self) -> Player {
        self.side
    }
    fn legal_moves(&self) -> Vec<(u8, bool)> {
        if self.over {
            return vec![];
        }
        let p = self.side.idx();
        let mut h = self.hand[p].clone();
        h.sort();
        h.dedup();
        let mut v = vec![];
        for &c in &h {
            if self.can_play(p, c) {
                v.push((c, false));
            }
            v.push((c, true));
        }
        v
    }
    fn apply(&mut self, m: &(u8, bool)) {
        let p = self.side.idx();
        self.ply += 1;
        let k = self.hand[p].iter().position(|&c| c == m.0).unwrap();
        self.hand[p].remove(k);
        if !m.1 {
            let c = m.0;
            match c {
                0..=4 => {
                    self.tab[p].km += KM[c as usize];
                    if c == 4 {
                        self.tab[p].twos += 1;
                    }
                    if self.tab[p].km == 1000 {
                        self.over = true;
                    }
                }
                5 => self.tab[1 - p].hazard = Some(0),
                6 => self.tab[1 - p].limit = true,
                7 => self.tab[1 - p].hazard = Some(1),
                8 => self.tab[1 - p].hazard = Some(2),
                9 => self.tab[1 - p].hazard = Some(3),
                10 => self.tab[p].hazard = None,
                11 => self.tab[p].limit = false,
                12..=14 => self.tab[p].hazard = Some(0),
                _ => {
                    let s = c as usize - 15;
                    self.tab[p].safe[s] = true;
                    match s {
                        0 => {
                            self.tab[p].limit = false;
                            if self.tab[p].hazard == Some(0) {
                                self.tab[p].hazard = None;
                            }
                        }
                        1 if self.tab[p].hazard == Some(1) => self.tab[p].hazard = None,
                        2 if self.tab[p].hazard == Some(2) => self.tab[p].hazard = None,
                        3 if self.tab[p].hazard == Some(3) => self.tab[p].hazard = None,
                        _ => {}
                    }
                }
            }
        }
        if self.over {
            return;
        }
        // cel următor trage
        let q = 1 - p;
        if let Some(c) = self.deck.pop() {
            self.hand[q].push(c);
        }
        if self.hand[q].is_empty() && self.deck.is_empty() {
            self.over = true;
        }
        if self.ply > 400 {
            self.over = true;
        }
        self.prev = self.side;
        self.side = self.side.other();
        if self.hand[self.side.idx()].is_empty() {
            self.over = true;
        }
    }
    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            return None;
        }
        Some(match self.tab[0].km.cmp(&self.tab[1].km) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            _ => Outcome::Draw,
        })
    }
    fn evaluate(&self, p: Player) -> i32 {
        // doar tabloul public și mâna proprie
        let me = p.idx();
        let mut s = self.heur(me) - self.heur(1 - me);
        if self.tab[me].hazard.is_some() {
            // remediul în mână valorează
            let need = match self.tab[me].hazard {
                Some(0) => 10,
                Some(x) => 11 + x,
                None => 0,
            };
            if self.hand[me].contains(&need) {
                s += 20;
            }
        }
        s + self.hand[me].iter().filter(|&&c| c >= 15).count() as i32 * 5
    }
    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = match who {
            Some(p) => Viewer::Fixed(p),
            None => Viewer::HotSeat,
        };
    }
    fn render(&self) -> String {
        let mut s = String::new();
        if self.prev != self.side && !self.over {
            if let Viewer::HotSeat = self.viewer {
                s.push_str(&handover_banner(self.side));
            }
        }
        for p in 0..2 {
            let t = &self.tab[p];
            let hz = match t.hazard {
                None => "rulează".to_string(),
                Some(0) => "oprit (are nevoie de verde)".to_string(),
                Some(1) => "pană (roata)".to_string(),
                Some(2) => "fără benzină (benzina)".to_string(),
                _ => "accident (reparatie)".to_string(),
            };
            s.push_str(&format!(
                "J{}: {} km, 200-uri {}; {}{}; imunități: {}\n",
                p + 1,
                t.km,
                t.twos,
                hz,
                if t.limit { ", limită 50" } else { "" },
                ["prior", "rez", "tanc", "asig"]
                    .iter()
                    .enumerate()
                    .filter(|&(i, _)| t.safe[i])
                    .map(|(_, n)| *n)
                    .collect::<Vec<_>>()
                    .join(" ")
            ));
        }
        s.push_str(&format!("Pachet: {} cărți\n", self.deck.len()));
        let v = self.viewer.who(self.side);
        let mut h = self.hand[v.idx()].clone();
        h.sort();
        s.push_str(&format!(
            "Mâna ta (J{}): {}\n",
            v.idx() + 1,
            h.iter()
                .map(|&c| NAMES[c as usize])
                .collect::<Vec<_>>()
                .join(" ")
        ));
        s.push_str(&format!(
            "J{} are {} cărți\n",
            v.other().idx() + 1,
            self.hand[v.other().idx()].len()
        ));
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        s
    }
    fn format_move(&self, m: &(u8, bool)) -> String {
        if m.1 {
            format!("d {}", NAMES[m.0 as usize])
        } else {
            NAMES[m.0 as usize].to_string()
        }
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::b1_util::testing::check;
    #[test]
    fn need_green_and_limits() {
        let mut g = Kilometri::with_seed(1);
        g.hand[0] = vec![10, 3, 4, 9, 15];
        // la început: oprit; nu poți juca distanță
        assert!(!g.can_play(0, 3));
        assert!(g.can_play(0, 10));
        g.apply(&(10, false));
        g.side = Player::One;
        assert!(g.can_play(0, 3));
        g.tab[0].limit = true;
        assert!(!g.can_play(0, 3));
        g.tab[0].limit = false;
        g.tab[0].twos = 2;
        assert!(!g.can_play(0, 4));
        // piedici: nu pe cineva oprit
        assert!(!g.can_play(0, 5));
        g.tab[1].hazard = None;
        assert!(g.can_play(0, 5));
        g.tab[1].safe[0] = true;
        assert!(!g.can_play(0, 5));
    }
    #[test]
    fn hidden_hands() {
        let g = Kilometri::with_seed(4);
        let f = |who| {
            let mut x = g.clone();
            x.set_viewer(Some(who));
            x.render()
                .lines()
                .find(|l| l.starts_with("Mâna ta"))
                .unwrap()
                .split_once(": ")
                .unwrap()
                .1
                .to_string()
        };
        assert_ne!(f(Player::One), f(Player::Two));
    }
    #[test]
    fn random_and_ai() {
        check(Kilometri::with_seed, 500);
    }
}
