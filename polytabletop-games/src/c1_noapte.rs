#![allow(
    clippy::needless_range_loop,
    clippy::manual_is_multiple_of,
    clippy::collapsible_if,
    clippy::collapsible_match
)]
//! Motor „o noapte, roluri ascunse” (lista C1, 2 jucători, variante originale cu 6 cărți).
//!
//! Șase roluri: `trag` le amestecă (una pentru fiecare jucător, trei pe masă, una scoasă). Noaptea,
//! fiecare cu rolul lui inițial: rolul „rău” (`r`) poate `vad c` (privește o carte de pe masă) sau `pas`;
//! văzătorul (`s`) `vad adv` sau `vad 2,3`; hoțul (`h`) `fur` (schimbă cartea cu a celuilalt și o vede)
//! sau `pas`; insomniacul (`i`) se uită la finalul nopții la cartea lui curentă; bețivul (`b`) `bea c`
//! (schimbă orbește cu o carte de pe masă); mușcătorul (`m`, doar în varianta cu vampiri) `musc` marchează
//! adversarul (marcatul se numără în echipa rea); restul `somn`. Zorii (`zori`) rezolvă acțiunile în ordinea
//! rolurilor. Ziua fiecare alege `voteaza` (adversarul e executat) sau `iert`; `verifica` dă punctele.
//! Câștigă tăbăcarul (`t`) executat; altfel, dacă moare cineva din echipa rea, câștigă ceilalți; dacă nu
//! moare nimeni din echipa rea și există una în joc, ea câștigă; dacă nu există, ceilalți câștigă doar dacă
//! nu moare nimeni. 3 mâini: victorie 2 puncte. Informație ascunsă: rolurile și rezultatele nopții proprii
//! (HOTSEAT_SAFE). AI-ul execută doar când a văzut dovada.
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

pub struct NSpec {
    pub title: &'static str,
    pub deck: [char; 6],
    pub bad: &'static str,
    pub hands: u32,
}

fn rname(c: char) -> &'static str {
    match c {
        'r' => "rău",
        's' => "văzător",
        'h' => "hoț",
        'i' => "insomniac",
        'b' => "bețiv",
        'm' => "mușcător",
        't' => "tăbăcar",
        _ => "sătean",
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ph {
    Deal,
    Night(usize),
    Dawn,
    Day(usize),
    Verify,
}

#[derive(Clone)]
pub struct Noapte {
    sp: &'static NSpec,
    /// 0,1 = jucători, 2..4 = masă
    cards: [char; 5],
    init: [char; 2],
    act: [String; 2],
    mark: [bool; 2],
    know: [Vec<String>; 2],
    votes: [bool; 2],
    ph: Ph,
    hand: u32,
    pts: [i32; 2],
    viewer: Option<Player>,
    rng: Rng,
}

impl Noapte {
    pub fn new(sp: &'static NSpec) -> Self {
        Self::with_seed(sp, seed_now())
    }
    pub fn with_seed(sp: &'static NSpec, seed: u64) -> Self {
        Noapte {
            sp,
            cards: ['v'; 5],
            init: ['v'; 2],
            act: [String::new(), String::new()],
            mark: [false; 2],
            know: [Vec::new(), Vec::new()],
            votes: [false; 2],
            ph: Ph::Deal,
            hand: 0,
            pts: [0; 2],
            viewer: None,
            rng: Rng::new(seed ^ 0x3c6e_f372),
        }
    }
    fn team_bad(&self, p: usize) -> bool {
        self.sp.bad.contains(self.cards[p]) || self.mark[p]
    }
    fn night_moves(&self, role: char) -> Vec<String> {
        match role {
            'r' => vec!["pas".into(), "vad 2".into(), "vad 3".into(), "vad 4".into()],
            's' => vec![
                "vad adv".into(),
                "vad 2,3".into(),
                "vad 2,4".into(),
                "vad 3,4".into(),
            ],
            'h' => vec!["fur".into(), "pas".into()],
            'b' => vec!["bea 2".into(), "bea 3".into(), "bea 4".into(), "pas".into()],
            'm' => vec!["musc".into(), "pas".into()],
            _ => vec!["somn".into()],
        }
    }
}

impl Mini for Noapte {
    fn side(&self) -> Player {
        match self.ph {
            Ph::Night(p) | Ph::Day(p) => {
                if p == 0 {
                    Player::One
                } else {
                    Player::Two
                }
            }
            _ => Player::One,
        }
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            Ph::Deal => vec!["trag".into()],
            Ph::Dawn => vec!["zori".into()],
            Ph::Verify => vec!["verifica".into()],
            Ph::Night(p) => self.night_moves(self.init[p]),
            Ph::Day(_) => vec!["voteaza".into(), "iert".into()],
        }
    }
    fn step(&mut self, m: &str) {
        match self.ph {
            Ph::Deal => {
                let mut d: Vec<char> = self.sp.deck.to_vec();
                shuf(&mut d, &mut self.rng);
                self.cards.copy_from_slice(&d[..5]);
                self.init = [d[0], d[1]];
                self.act = [String::new(), String::new()];
                self.mark = [false; 2];
                self.know = [Vec::new(), Vec::new()];
                self.votes = [false; 2];
                self.know[0].push(format!("rolul tău: {}", rname(d[0])));
                self.know[1].push(format!("rolul tău: {}", rname(d[1])));
                self.ph = Ph::Night(0);
            }
            Ph::Night(p) => {
                self.act[p] = m.to_string();
                self.ph = if p == 0 { Ph::Night(1) } else { Ph::Dawn };
            }
            Ph::Dawn => {
                // ordine: rău, văzător, mușcător, hoț, bețiv, insomniac
                for r in ['r', 's', 'm', 'h', 'b', 'i'] {
                    for p in 0..2 {
                        if self.init[p] != r {
                            continue;
                        }
                        let a = self.act[p].clone();
                        let o = 1 - p;
                        if a == "fur" {
                            self.cards.swap(p, o);
                            self.know[p].push(format!("după furt ai: {}", rname(self.cards[p])));
                        } else if a == "musc" {
                            self.mark[o] = true;
                        } else if let Some(c) = a.strip_prefix("bea ") {
                            let c: usize = c.parse().unwrap();
                            self.cards.swap(p, c);
                        } else if a == "vad adv" {
                            self.know[p].push(format!("adversarul are: {}", rname(self.cards[o])));
                        } else if let Some(c) = a.strip_prefix("vad ") {
                            let parts: Vec<usize> =
                                c.split(',').map(|x| x.parse().unwrap()).collect();
                            let t: Vec<String> = parts
                                .iter()
                                .map(|&i| format!("masa {i}: {}", rname(self.cards[i])))
                                .collect();
                            self.know[p].push(t.join(", "));
                        }
                        if r == 'i' {
                            self.know[p].push(format!("la final ai: {}", rname(self.cards[p])));
                        }
                    }
                }
                self.ph = Ph::Day(0);
            }
            Ph::Day(p) => {
                self.votes[p] = m == "voteaza";
                self.ph = if p == 0 { Ph::Day(1) } else { Ph::Verify };
            }
            Ph::Verify => {
                // p este executat dacă celălalt a votat
                let dead = [self.votes[1], self.votes[0]];
                let mut win = [false; 2];
                let tan = (0..2).find(|&p| dead[p] && self.cards[p] == 't');
                if let Some(t) = tan {
                    win[t] = true;
                } else {
                    let any_bad_dead = (0..2).any(|p| dead[p] && self.team_bad(p));
                    let bad_in_play = (0..2).any(|p| self.team_bad(p));
                    for p in 0..2 {
                        win[p] = if any_bad_dead {
                            !self.team_bad(p) && self.cards[p] != 't'
                        } else if bad_in_play {
                            self.team_bad(p)
                        } else {
                            !dead[0] && !dead[1] && self.cards[p] != 't'
                        };
                    }
                }
                for p in 0..2 {
                    if win[p] {
                        self.pts[p] += 2;
                    }
                }
                self.hand += 1;
                self.ph = Ph::Deal;
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.hand >= self.sp.hands {
            return Some(by_score(self.pts[0], self.pts[1]));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let mut s = 1000 * (self.pts[a] - self.pts[1 - a]);
        // valoarea alegerii nocturne proprii (după ce a fost făcută)
        let after_night = (self.ph == Ph::Night(1) && a == 0) || self.ph == Ph::Dawn;
        if after_night {
            s += match self.act[a].as_str() {
                "vad adv" => 400,
                "fur" => 350,
                "musc" => 380,
                "vad 2,3" | "vad 2,4" | "vad 3,4" => 300,
                "vad 2" | "vad 3" | "vad 4" => 250,
                "bea 2" | "bea 3" | "bea 4" => 120,
                _ => 100,
            };
        }
        // ziua: execută doar dacă a văzut că adversarul e în echipa rea și el nu e
        let day_done = (self.ph == Ph::Day(1) && a == 0) || self.ph == Ph::Verify;
        if day_done {
            let proof = self.know[a].iter().any(|k| {
                k.starts_with("adversarul are:")
                    && self.sp.bad.chars().any(|c| k.ends_with(rname(c)))
            });
            let me_bad = self.sp.bad.contains(self.init[a]);
            if self.votes[a] {
                s += if proof && !me_bad { 500 } else { -300 };
            } else {
                s += if proof && !me_bad { -200 } else { 100 };
            }
        }
        s
    }
    fn show(&self) -> String {
        let me = pi(self.viewer.unwrap_or(self.side()));
        let mut s = format!(
            "{} — mâna {}/{} | puncte J1 {} J2 {}\n",
            self.sp.title,
            (self.hand + 1).min(self.sp.hands),
            self.sp.hands,
            self.pts[0],
            self.pts[1]
        );
        if self.ph != Ph::Deal {
            for k in &self.know[me] {
                s.push_str(&format!("J{} știe: {k}\n", me + 1));
            }
        }
        s.push_str(&format!(
            "La mutare: {} (faza {:?}: trag | vad .. / fur / bea c / musc / pas / somn | zori | voteaza / iert | verifica)\n",
            pname(self.side()),
            self.ph
        ));
        s
    }
    fn view(&mut self, w: Option<Player>) {
        self.viewer = w;
    }
}

#[cfg(test)]
impl Noapte {
    pub fn scramble(&mut self, hide: usize, seed: u64) {
        let mut rng = Rng::new(seed);
        let d = self.sp.deck;
        self.cards[hide] = d[rng.below(6)];
        self.cards[2 + rng.below(3)] = d[rng.below(6)];
        self.know[hide].clear();
        if self.ph != Ph::Deal && self.ph != Ph::Night(hide) {
            self.act[hide] = "somn".into();
        }
    }
}
