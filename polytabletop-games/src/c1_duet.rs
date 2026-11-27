#![allow(
    clippy::needless_range_loop,
    clippy::manual_is_multiple_of,
    clippy::collapsible_if
)]
//! Motor cooperativ „cuvinte în tandem” (lista C1, 2 jucători, variantă originală).
//!
//! 25 de cuvinte pe o grilă; fiecare cuvânt are două teme din 12. Fiecare jucător are o cheie secretă
//! (9 agenți, 3 capcane, restul neutre), cu 3 agenți comuni. Pe rând unul dă un indiciu `ind t n`
//! (tema t, număr n, între 1 și 3) cu cheia lui; celălalt ghicește `ghic k` (cel mult n+1 ghiciri) sau
//! `stop` după prima. Mutarea forțată `rev` dezvăluie după cheia celui care a dat indiciul: agent =
//! găsit (poți continua), neutru = tura se încheie, capcană = eșec. Echipa câștigă dacă găsește toți cei
//! 15 agenți distincți în `timer` ture de indiciu; altfel REMIZĂ (cooperativ). AI-ul dă indicii cu
//! propria cheie și ghicește doar după temă și după neutrele deja dezvăluite.
//! Informație ascunsă: cheia fiecăruia (HOTSEAT_SAFE).
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

pub struct USpec {
    pub title: &'static str,
    pub timer: u32,
}

const THEMES: [&str; 12] = [
    "apă", "foc", "pădure", "oraș", "animal", "mâncare", "muzică", "timp", "sport", "meserie",
    "cer", "casă",
];

const WORDS: [(&str, u8, u8); 40] = [
    ("barcă", 0, 8),
    ("fântână", 0, 3),
    ("ploaie", 0, 10),
    ("scânteie", 1, 7),
    ("lumânare", 1, 11),
    ("vulcan", 1, 10),
    ("stejar", 2, 4),
    ("ciupercă", 2, 5),
    ("potecă", 2, 8),
    ("piață", 3, 5),
    ("pod", 3, 0),
    ("turn", 3, 10),
    ("lup", 4, 2),
    ("pește", 4, 0),
    ("albină", 4, 5),
    ("pâine", 5, 9),
    ("miere", 5, 4),
    ("supă", 5, 11),
    ("vioară", 6, 9),
    ("tobă", 6, 8),
    ("cor", 6, 3),
    ("clepsidră", 7, 11),
    ("calendar", 7, 3),
    ("secundă", 7, 8),
    ("minge", 8, 3),
    ("arc", 8, 2),
    ("alergare", 8, 7),
    ("fierar", 9, 1),
    ("pescar", 9, 0),
    ("croitor", 9, 11),
    ("nor", 10, 0),
    ("stea", 10, 7),
    ("fulger", 10, 1),
    ("fereastră", 11, 10),
    ("sobă", 11, 1),
    ("masă", 11, 5),
    ("furnică", 4, 9),
    ("ancoră", 0, 9),
    ("far", 3, 10),
    ("copac", 2, 11),
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ph {
    Clue,
    Guess,
    Reveal,
}

#[derive(Clone)]
pub struct Duet {
    sp: &'static USpec,
    board: Vec<usize>,
    /// 0 neutru, 1 agent, 2 capcană, per jucător
    key: [Vec<u8>; 2],
    found: Vec<bool>,
    /// cărți dezvăluite ca neutre pentru cheia lui J (public)
    innocent: [Vec<bool>; 2],
    ph: Ph,
    giver: usize,
    theme: usize,
    num: usize,
    guesses: usize,
    pend: usize,
    turns: u32,
    end: Option<Outcome>,
    viewer: Option<Player>,
}

impl Duet {
    pub fn new(sp: &'static USpec) -> Self {
        Self::with_seed(sp, seed_now())
    }
    pub fn with_seed(sp: &'static USpec, seed: u64) -> Self {
        let mut rng = Rng::new(seed ^ 0xc2b2_ae35);
        let mut idx: Vec<usize> = (0..WORDS.len()).collect();
        shuf(&mut idx, &mut rng);
        let board: Vec<usize> = idx[..25].to_vec();
        let mut pos: Vec<usize> = (0..25).collect();
        shuf(&mut pos, &mut rng);
        let mut k1 = vec![0u8; 25];
        for &c in &pos[..9] {
            k1[c] = 1;
        }
        for &c in &pos[9..12] {
            k1[c] = 2;
        }
        let common = &pos[..3];
        let mut rest: Vec<usize> = (0..25).filter(|c| !common.contains(c)).collect();
        shuf(&mut rest, &mut rng);
        let mut k2 = vec![0u8; 25];
        for &c in common {
            k2[c] = 1;
        }
        for &c in &rest[..3] {
            k2[c] = 2;
        }
        for &c in &rest[3..9] {
            k2[c] = 1;
        }
        Duet {
            sp,
            board,
            key: [k1, k2],
            found: vec![false; 25],
            innocent: [vec![false; 25], vec![false; 25]],
            ph: Ph::Clue,
            giver: 0,
            theme: 0,
            num: 0,
            guesses: 0,
            pend: 0,
            turns: 0,
            end: None,
            viewer: None,
        }
    }
    fn has_theme(&self, k: usize, t: usize) -> bool {
        let w = WORDS[self.board[k]];
        w.1 as usize == t || w.2 as usize == t
    }
    fn is_goal(&self, c: usize) -> bool {
        self.key[0][c] == 1 || self.key[1][c] == 1
    }
    fn left(&self) -> usize {
        (0..25)
            .filter(|&c| self.is_goal(c) && !self.found[c])
            .count()
    }
}

impl Mini for Duet {
    fn side(&self) -> Player {
        let who = match self.ph {
            Ph::Clue => self.giver,
            Ph::Guess => 1 - self.giver,
            Ph::Reveal => 1 - self.giver,
        };
        if who == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            Ph::Reveal => vec!["rev".into()],
            Ph::Clue => {
                let mut v = Vec::new();
                for t in 0..12 {
                    for n in 1..=3 {
                        v.push(format!("ind {t} {n}"));
                    }
                }
                v
            }
            Ph::Guess => {
                let mut v: Vec<String> = (0..25)
                    .filter(|&k| !self.found[k])
                    .map(|k| format!("ghic {k}"))
                    .collect();
                if self.guesses >= 1 {
                    v.push("stop".into());
                }
                v
            }
        }
    }
    fn step(&mut self, m: &str) {
        match self.ph {
            Ph::Clue => {
                let mut it = m[4..].split(' ');
                self.theme = it.next().unwrap().parse().unwrap();
                self.num = it.next().unwrap().parse().unwrap();
                self.guesses = 0;
                self.ph = Ph::Guess;
            }
            Ph::Guess => {
                if m == "stop" {
                    self.next_turn();
                } else {
                    self.pend = m[5..].parse().unwrap();
                    self.ph = Ph::Reveal;
                }
            }
            Ph::Reveal => {
                let c = self.pend;
                self.guesses += 1;
                match self.key[self.giver][c] {
                    2 => {
                        self.end = Some(Outcome::Draw);
                    }
                    1 => {
                        self.found[c] = true;
                        if self.left() == 0 {
                            self.end = Some(Outcome::Win(Player::One));
                            return;
                        }
                        if self.guesses > self.num {
                            self.next_turn();
                        } else {
                            self.ph = Ph::Guess;
                        }
                    }
                    _ => {
                        self.innocent[self.giver][c] = true;
                        self.next_turn();
                    }
                }
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        self.end
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        match self.ph {
            Ph::Guess if self.giver == a => {
                // am dat un indiciu: câți agenți (ai mei, negăsiți) au tema, câte capcane/neutre
                let (mut g, mut b, mut n) = (0, 0, 0);
                for k in 0..25 {
                    if !self.found[k] && self.has_theme(k, self.theme) {
                        match self.key[a][k] {
                            1 => g += 1,
                            2 => b += 1,
                            _ => n += 1,
                        }
                    }
                }
                let want = self.num as i32;
                100 * g.min(want) - 600 * b - 70 * n - 90 * (want - g).max(0)
            }
            Ph::Reveal if self.giver != a => {
                // ghicesc cartea pend, după temă; nu citesc cheia celuilalt
                let c = self.pend;
                if self.innocent[self.giver][c] {
                    -1000
                } else if self.has_theme(c, self.theme) {
                    500
                } else {
                    0
                }
            }
            Ph::Clue if self.giver != a => {
                // am oprit (stop): valoare moderată
                150
            }
            _ => 0,
        }
    }
    fn show(&self) -> String {
        let me = pi(self.viewer.unwrap_or(self.side()));
        let mut s = format!(
            "{} — tura {}/{} | agenți rămași {}\n",
            self.sp.title,
            self.turns + 1,
            self.sp.timer,
            self.left()
        );
        for r in 0..5 {
            let mut row = String::new();
            for c in 0..5 {
                let k = r * 5 + c;
                let w = WORDS[self.board[k]].0;
                let mark = if self.found[k] {
                    "OK"
                } else {
                    match self.key[me][k] {
                        1 => "ag",
                        2 => "XX",
                        _ => {
                            if self.innocent[me][k] {
                                "--"
                            } else {
                                "  "
                            }
                        }
                    }
                };
                row.push_str(&format!("{k:>2}:{w}[{mark}] "));
            }
            s.push_str(&row);
            s.push('\n');
        }
        s.push_str("(cheia ta: ag = agent, XX = capcană; OK = găsit; -- = neutru dezvăluit)\n");
        if self.ph != Ph::Clue {
            s.push_str(&format!(
                "indiciu de la J{}: {} ×{} (ghiciri făcute {})\n",
                self.giver + 1,
                THEMES[self.theme],
                self.num,
                self.guesses
            ));
        }
        s.push_str(&format!(
            "La mutare: {} (ind t n [teme 0–11: {}] | ghic k | stop; rev forțat)\n",
            pname(self.side()),
            THEMES.join(", ")
        ));
        s
    }
    fn view(&mut self, w: Option<Player>) {
        self.viewer = w;
    }
}

impl Duet {
    fn next_turn(&mut self) {
        self.turns += 1;
        self.giver = 1 - self.giver;
        self.ph = Ph::Clue;
        if self.turns >= self.sp.timer {
            self.end = Some(Outcome::Draw);
        }
    }
}

#[cfg(test)]
impl Duet {
    pub fn scramble(&mut self, hide: usize, seed: u64) {
        let mut rng = Rng::new(seed);
        shuf(&mut self.key[hide], &mut rng);
    }
}
