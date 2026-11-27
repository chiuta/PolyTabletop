//! Balul Măștilor Schimbate — roluri ascunse, schimburi și acuzații (variantă originală, 2 jucători).
//!
//! Șase roluri: 0 Rege (+3 monede), 1 Bancher (+2), 2 Hoț (fură 2), 3 Judecător (+1 și adversarul −1),
//! 4 Vrăjitor (schimbă totalul de monede cu adversarul), 5 Bufon (+1). Fiecare jucător ține o mască (un rol) pe care o
//! vede doar el, și doar dacă s-a uitat la ea (`privesc`). Pe rând: `privesc`; `schimb` (măștile se schimbă cu adevărat,
//! iar tu nu mai știi ce ai) sau `simulez` (nu se schimbă nimic, dar adversarul nu poate ști); ori `rol R` (anunți rolul R și
//! îi folosești puterea). Adversarul acceptă (`accept`) sau acuză (`provoc`): mutarea forțată `arata` verifică masca —
//! anunț adevărat: puterea se aplică și acuzatorul pierde 2 monede; fals: anunțatorul pierde 2 monede, puterea nu se aplică.
//! Câștigă primul care ajunge la 10 monede; după 60 de mutări, cine are mai multe.
//! Informație ascunsă: măștile și ce știe fiecare; AI-ul nu citește masca adversarului.
use crate::b6_util::pl;
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const NAMES: [&str; 6] = ["Rege", "Bancher", "Hoț", "Judecător", "Vrăjitor", "Bufon"];
const GAIN: [i32; 6] = [3, 2, 2, 2, 1, 1];
const CAP: u32 = 60;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ph {
    Act,
    Resp(usize),
    Reveal(usize),
}

#[derive(Clone)]
pub struct Bal {
    viewer: Option<Player>,
    card: [usize; 2],
    belief: [Option<usize>; 2],
    unsure: [bool; 2],
    coins: [i32; 2],
    mover: usize,
    ph: Ph,
    plies: u32,
    last: String,
}

impl Bal {
    pub fn new() -> Self {
        Self::with_seed(seed_now())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let a = rng.below(6);
        let b = (a + 1 + rng.below(5)) % 6;
        Bal {
            viewer: None,
            card: [a, b],
            belief: [Some(a), Some(b)],
            unsure: [false; 2],
            coins: [3, 3],
            mover: 0,
            ph: Ph::Act,
            plies: 0,
            last: String::new(),
        }
    }
    /// Pentru teste (audit AI): re-trage măștile necunoscute lui `1 - hide` (a lui `hide` și, dacă cel la
    /// mutare nu mai e sigur de masca lui, și a lui) și ce crede `hide`.
    #[cfg(test)]
    pub fn scramble(&mut self, hide: usize, seed: u64) {
        let mut r = Rng::new(seed);
        let me = 1 - hide;
        if self.belief[me].is_none() || self.unsure[me] {
            self.card[me] = r.below(6);
        }
        self.card[hide] = (self.card[me] + 1 + r.below(5)) % 6;
        self.belief[hide] = if r.below(3) == 0 {
            None
        } else {
            Some(self.card[hide])
        };
        self.unsure[hide] = r.below(2) == 0 && self.belief[hide].is_some();
    }
    fn apply_power(&mut self, who: usize, r: usize) {
        let o = 1 - who;
        match r {
            0 => self.coins[who] += 3,
            1 => self.coins[who] += 2,
            2 => {
                let t = self.coins[o].min(2);
                self.coins[o] -= t;
                self.coins[who] += t;
            }
            3 => {
                self.coins[who] += 1;
                self.coins[o] = (self.coins[o] - 1).max(0);
            }
            4 => self.coins.swap(0, 1),
            _ => self.coins[who] += 1,
        }
    }
    /// Probabilitatea (sutimi) ca adversarul lui `me` să aibă rolul `r`, din punctul de vedere al lui `me`.
    fn p_opp_has(&self, me: usize, r: usize) -> i32 {
        match self.belief[me] {
            Some(b) if !self.unsure[me] => {
                if b == r {
                    0
                } else {
                    // masca adversarului e unul dintre celelalte 5 roluri; cine anunță probabil știe ce are
                    35
                }
            }
            _ => 28,
        }
    }
    /// Probabilitatea (sutimi) ca eu să am rolul `r`, după ce știu.
    fn p_mine(&self, me: usize, r: usize) -> i32 {
        match self.belief[me] {
            Some(b) if !self.unsure[me] => (b == r) as i32 * 100,
            Some(b) => {
                if b == r {
                    50
                } else {
                    10
                }
            }
            None => 17,
        }
    }
}

impl Default for Bal {
    fn default() -> Self {
        Self::new()
    }
}

impl Mini for Bal {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        match self.ph {
            Ph::Act => pl(self.mover),
            Ph::Resp(_) => pl(1 - self.mover),
            Ph::Reveal(_) => pl(self.mover),
        }
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            Ph::Act => {
                let mut v = vec!["privesc".to_string(), "schimb".into(), "simulez".into()];
                for r in 0..6 {
                    v.push(format!("rol {r}"));
                }
                v
            }
            Ph::Resp(_) => vec!["accept".into(), "provoc".into()],
            Ph::Reveal(_) => vec!["arata".into()],
        }
    }
    fn step(&mut self, m: &str) {
        let me = self.mover;
        let o = 1 - me;
        match self.ph {
            Ph::Act => {
                self.plies += 1;
                if m == "privesc" {
                    self.belief[me] = Some(self.card[me]);
                    self.unsure[me] = false;
                    self.last = format!("J{} s-a uitat la masca lui", me + 1);
                    self.mover = o;
                } else if m == "schimb" || m == "simulez" {
                    if m == "schimb" {
                        self.card.swap(0, 1);
                        self.belief[me] = None;
                    }
                    self.unsure[o] = true;
                    self.last = format!("J{} a atins măștile (schimb sau nu)", me + 1);
                    self.mover = o;
                } else {
                    self.ph = Ph::Resp(m[4..].parse().unwrap());
                    self.last = format!(
                        "J{} anunță rolul {}",
                        me + 1,
                        NAMES[m[4..].parse::<usize>().unwrap()]
                    );
                }
            }
            Ph::Resp(r) => {
                if m == "accept" {
                    self.apply_power(me, r);
                    self.ph = Ph::Act;
                    self.mover = o;
                } else {
                    self.ph = Ph::Reveal(r);
                }
            }
            Ph::Reveal(r) => {
                if self.card[me] == r {
                    self.apply_power(me, r);
                    self.coins[o] = (self.coins[o] - 2).max(0);
                    self.last = format!("J{} avea într-adevăr {}", me + 1, NAMES[r]);
                } else {
                    self.coins[me] = (self.coins[me] - 2).max(0);
                    self.last = format!("J{} a mințit: avea {}", me + 1, NAMES[self.card[me]]);
                }
                self.belief[me] = Some(self.card[me]);
                self.unsure[me] = false;
                self.ph = Ph::Act;
                self.mover = o;
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.coins[0] >= 10 || self.coins[1] >= 10 || self.plies >= CAP {
            return Some(by_score(self.coins[0], self.coins[1]));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        let o = 1 - a;
        let base = 100 * (self.coins[a] - self.coins[o]);
        match self.ph {
            // eu am anunțat rolul r: adversarul acuză cu ~35%
            Ph::Resp(r) if a == self.mover => {
                let pt = self.p_mine(a, r);
                let g = GAIN[r] * 100;
                base + (65 * g + 35 * (pt * (g + 200) - (100 - pt) * 200) / 100) / 100
            }
            // eu am acuzat: pierd 2 monede dacă anunțul e adevărat
            Ph::Reveal(r) if a != self.mover => {
                let pt = self.p_opp_has(a, r);
                base + (pt * (-GAIN[r] * 100 - 200) + (100 - pt) * 200) / 100
            }
            _ => {
                // ce știu despre propria mască valorează puțin; a fi uitat în jur e mai bun când nu știu nimic
                if self.belief[a].is_none() || self.unsure[a] {
                    base - 20
                } else {
                    base
                }
            }
        }
    }
    fn show(&self) -> String {
        let viewer = self.viewer.map(pi).unwrap_or(pi(self.side()));
        let know = match self.belief[viewer] {
            Some(b) if !self.unsure[viewer] => format!("masca ta e {} (sigur)", NAMES[b]),
            Some(b) => format!("credeai că ai {}, dar adversarul a atins măștile", NAMES[b]),
            None => "nu știi ce mască ai".to_string(),
        };
        format!(
            "Balul Măștilor Schimbate — mutarea {}/{} | monede J1 {}, J2 {}\n{}\nUltima acțiune: {}\nLa mutare: {} (privesc | schimb | simulez | rol R | accept | provoc | arata)\nRoluri: {}\n",
            self.plies + 1,
            CAP,
            self.coins[0],
            self.coins[1],
            know,
            if self.last.is_empty() { "-" } else { &self.last },
            pname(self.side()),
            (0..6).map(|r| format!("{}={}(+{})", r, NAMES[r], GAIN[r])).collect::<Vec<_>>().join(" ")
        )
    }
    fn view(&mut self, w: Option<Player>) {
        self.viewer = w;
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        crate::brand_util::check(Bal::with_seed, 200);
    }
    #[test]
    fn ai_ignores_hidden_information_at_every_level() {
        crate::p3_comun::ai_blind_check(
            "bal_masti",
            |s| B(Bal::with_seed(s)),
            |g, me, s| g.0.scramble(1 - me.idx(), s),
            14,
        );
    }
    #[test]
    fn responder_ai_ignores_the_claimers_mask() {
        let mut g = Bal::with_seed(9);
        let r = (g.card[1] + 1) % 6;
        let r = if r == g.card[0] { (r + 1) % 6 } else { r };
        g.step(&format!("rol {r}"));
        let mut g2 = g.clone();
        g2.card[0] = r;
        for seed in 1..6 {
            assert_eq!(ai_of(&g, seed), ai_of(&g2, seed));
        }
        let mut v = g.clone();
        v.view(Some(Player::Two));
        assert!(v.show().contains(NAMES[g.card[1]]));
        assert!(
            !v.show()
                .contains(&format!("masca ta e {}", NAMES[g.card[0]]))
                || g.card[0] == g.card[1]
        );
    }
}
