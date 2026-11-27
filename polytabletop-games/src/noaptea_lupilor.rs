//! Noaptea Lupilor — deducție cu roluri ascunse într-o noapte (variantă originală, 2 jucători).
//!
//! Șase roluri: lup, văzător, hoț, doi săteni, tăbăcar. Mutarea forțată `trag` le amestecă: câte unul
//! pentru fiecare jucător, trei pe masă, unul scos. Noaptea (fiecare cu rolul inițial): văzătorul
//! alege `vad adv` (privește cartea adversarului) sau `vad 2,3` / `vad 2,4` / `vad 3,4` (două cărți de
//! pe masă); hoțul alege `fur` (schimbă cartea cu a adversarului și o vede pe cea nouă) sau `pas`;
//! ceilalți: `somn`. Mutarea forțată `zori` execută acțiunile în ordinea văzător, hoț (alegerile
//! rămân secrete). Ziua, pe rând: `voteaza` (adversarul e executat) sau `iert`; apoi `verifica`.
//! Câștigă tăbăcarul executat; altfel, dacă moare lupul, câștigă cei fără lup; dacă niciun lup nu
//! moare și un lup e în joc, el câștigă; fără lup în joc, satenii câștigă doar dacă nu moare nimeni.
//! 3 mâini: victorie 2 puncte, remiză 1 punct pentru fiecare.
#![allow(
    clippy::needless_range_loop,
    clippy::manual_is_multiple_of,
    clippy::manual_memcpy,
    clippy::collapsible_match
)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

const ROLES: [&str; 5] = ["lup", "văzător", "hoț", "sătean", "tăbăcar"];
const DECK: [u8; 6] = [0, 1, 2, 3, 3, 4];
const HANDS: u32 = 3;

fn perms() -> &'static Vec<[u8; 6]> {
    static P: std::sync::OnceLock<Vec<[u8; 6]>> = std::sync::OnceLock::new();
    P.get_or_init(|| {
        let mut out = Vec::new();
        fn rec(cur: &mut Vec<usize>, used: &mut [bool; 6], out: &mut Vec<[u8; 6]>) {
            if cur.len() == 6 {
                let mut a = [0u8; 6];
                for i in 0..6 {
                    a[i] = DECK[cur[i]];
                }
                if !out.contains(&a) {
                    out.push(a);
                }
                return;
            }
            for i in 0..6 {
                if !used[i] {
                    used[i] = true;
                    cur.push(i);
                    rec(cur, used, out);
                    cur.pop();
                    used[i] = false;
                }
            }
        }
        rec(&mut Vec::new(), &mut [false; 6], &mut out);
        out
    })
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Ph {
    Deal,
    Night(usize),
    Dawn,
    Day(usize),
    Resolve,
}

#[derive(Clone)]
pub struct Noaptea {
    viewer: Option<Player>,
    orig: [u8; 6],
    cur: [u8; 6],
    intent: [Option<u8>; 2],
    facts: [Vec<(usize, u8)>; 2],
    votes: [bool; 2],
    pts: [i32; 2],
    hand_no: u32,
    ph: Ph,
    rng: Rng,
}

pub fn new() -> Noaptea {
    Noaptea::with_seed(seed_now())
}

/// Rezultat pentru jucătorul 0 în lumea (finală) dată: +1 câștig, 0 remiză, −1 pierdere.
fn judge(f: &[u8; 6], v: [bool; 2]) -> i32 {
    let dead = [v[1], v[0]]; // J0 moare dacă J1 votează
    let team = |p: usize| f[p];
    let mut win = [false; 2];
    let tan = (0..2).find(|&p| team(p) == 4);
    let wolf_in_play = (0..2).any(|p| team(p) == 0);
    if let Some(t) = tan.filter(|&t| dead[t]) {
        win[t] = true;
    } else {
        let wolf_died = (0..2).any(|p| team(p) == 0 && dead[p]);
        if wolf_died {
            for p in 0..2 {
                win[p] = team(p) != 0 && team(p) != 4;
            }
        } else if wolf_in_play {
            for p in 0..2 {
                win[p] = team(p) == 0;
            }
        } else if !dead[0] && !dead[1] {
            for p in 0..2 {
                win[p] = team(p) != 4;
            }
        }
    }
    match (win[0], win[1]) {
        (true, false) => 1,
        (false, true) => -1,
        _ => 0,
    }
}

fn final_roles(orig: &[u8; 6], swapped: bool) -> [u8; 6] {
    let mut f = *orig;
    if swapped {
        f.swap(0, 1);
    }
    f
}

impl Noaptea {
    pub fn with_seed(seed: u64) -> Self {
        Noaptea {
            viewer: None,
            orig: [3; 6],
            cur: [3; 6],
            intent: [None; 2],
            facts: [Vec::new(), Vec::new()],
            votes: [false; 2],
            pts: [0; 2],
            hand_no: 0,
            ph: Ph::Deal,
            rng: Rng::new(seed),
        }
    }
    /// Aștept­area de câștig (×100) a jucătorului p, după ce știe p, pentru votul `mine`; votul adversarului: `opp`
    /// (None = 50/50).
    fn expect(&self, p: usize, mine: bool, opp: Option<bool>) -> i32 {
        let mut tot = 0.0;
        let mut cnt = 0.0;
        'w: for w in perms() {
            for &(s, r) in &self.facts[p] {
                if w[s] != r {
                    continue 'w;
                }
            }
            // decizia hoțului: dacă hoțul sunt eu, o cunosc; altfel 50/50
            let swaps: Vec<(bool, f64)> = if w[p] == 2 {
                vec![(self.intent[p] == Some(1), 1.0)]
            } else if w[1 - p] == 2 {
                vec![(false, 0.5), (true, 0.5)]
            } else {
                vec![(false, 1.0)]
            };
            for (sw, wt) in swaps {
                let f = final_roles(w, sw);
                let mut v = 0.0;
                let ovs: Vec<(bool, f64)> = match opp {
                    Some(o) => vec![(o, 1.0)],
                    None => vec![(false, 0.5), (true, 0.5)],
                };
                for (ov, ow) in ovs {
                    let vote = if p == 0 { [mine, ov] } else { [ov, mine] };
                    let r = judge(&f, vote);
                    v += ow * (if p == 0 { r } else { -r }) as f64;
                }
                tot += wt * v;
                cnt += wt;
            }
        }
        if cnt == 0.0 {
            0
        } else {
            (100.0 * tot / cnt) as i32
        }
    }
    fn night_actor(&self, p: usize) -> u8 {
        self.orig[p]
    }
}

impl Mini for Noaptea {
    fn depth(&self) -> u32 {
        0
    }
    fn side(&self) -> Player {
        let t = match self.ph {
            Ph::Night(p) | Ph::Day(p) => p,
            _ => 0,
        };
        if t == 0 {
            Player::One
        } else {
            Player::Two
        }
    }
    fn moves(&self) -> Vec<String> {
        match self.ph {
            Ph::Deal => vec!["trag".into()],
            Ph::Dawn => vec!["zori".into()],
            Ph::Resolve => vec!["verifica".into()],
            Ph::Night(p) => match self.night_actor(p) {
                1 => vec![
                    "vad adv".into(),
                    "vad 2,3".into(),
                    "vad 2,4".into(),
                    "vad 3,4".into(),
                ],
                2 => vec!["fur".into(), "pas".into()],
                _ => vec!["somn".into()],
            },
            Ph::Day(_) => vec!["voteaza".into(), "iert".into()],
        }
    }
    fn step(&mut self, m: &str) {
        match m {
            "trag" => {
                let mut d = DECK;
                shuf(&mut d, &mut self.rng);
                self.orig = d;
                self.cur = d;
                self.intent = [None; 2];
                self.facts = [vec![(0, d[0])], vec![(1, d[1])]];
                self.facts[1] = vec![(1, d[1])];
                self.ph = Ph::Night(0);
            }
            "zori" => {
                // văzătorul întâi, apoi hoțul
                for p in 0..2 {
                    if self.orig[p] == 1 {
                        match self.intent[p] {
                            Some(0) => self.facts[p].push((1 - p, self.orig[1 - p])),
                            Some(k) => {
                                let (a, b) = match k {
                                    1 => (2, 3),
                                    2 => (2, 4),
                                    _ => (3, 4),
                                };
                                self.facts[p].push((a, self.orig[a]));
                                self.facts[p].push((b, self.orig[b]));
                            }
                            None => {}
                        }
                    }
                }
                for p in 0..2 {
                    if self.orig[p] == 2 && self.intent[p] == Some(1) {
                        self.cur.swap(0, 1);
                        self.facts[p].push((1 - p, self.orig[1 - p]));
                    }
                }
                self.votes = [false; 2];
                self.ph = Ph::Day(0);
            }
            "verifica" => {
                let r = judge(&self.cur, self.votes);
                match r {
                    1 => self.pts[0] += 2,
                    -1 => self.pts[1] += 2,
                    _ => {
                        self.pts[0] += 1;
                        self.pts[1] += 1;
                    }
                }
                self.hand_no += 1;
                self.ph = Ph::Deal;
            }
            "voteaza" | "iert" => {
                if let Ph::Day(p) = self.ph {
                    self.votes[p] = m == "voteaza";
                    self.ph = if p == 0 { Ph::Day(1) } else { Ph::Resolve };
                }
            }
            _ => {
                if let Ph::Night(p) = self.ph {
                    self.intent[p] = Some(match m {
                        "vad adv" => 0,
                        "vad 2,3" => 1,
                        "vad 2,4" => 2,
                        "vad 3,4" => 3,
                        "fur" => 1,
                        _ => 0,
                    });
                    if m == "pas" || m == "somn" {
                        self.intent[p] = Some(if self.orig[p] == 2 { 0 } else { 9 });
                    }
                    self.ph = if p == 0 { Ph::Night(1) } else { Ph::Dawn };
                }
            }
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.hand_no >= HANDS {
            Some(by_score(self.pts[0], self.pts[1]))
        } else {
            None
        }
    }
    fn eval(&self, p: Player) -> i32 {
        let i = pi(p);
        let o = 1 - i;
        let mut e = 300 * (self.pts[i] - self.pts[o]);
        match self.ph {
            Ph::Night(_) | Ph::Dawn => {
                // valoarea informației, fără a ști ce se va vedea: constantă pe tipul de acțiune
                if let Some(k) = self.intent[i] {
                    if self.orig[i] == 1 {
                        e += if k == 0 { 40 } else { 25 };
                    } else if self.orig[i] == 2 && k == 1 {
                        e += 20;
                    }
                }
            }
            Ph::Day(1) if i == 0 => e += self.expect(0, self.votes[0], None),
            Ph::Resolve if i == 1 => e += self.expect(1, self.votes[1], Some(self.votes[0])),
            _ => {}
        }
        e
    }
    fn show(&self) -> String {
        let me = self.viewer.map(pi).unwrap_or(pi(self.side()));
        let mut s = format!(
            "Noaptea Lupilor — mâna {}/{} | J1 {} / J2 {} puncte\n",
            (self.hand_no + 1).min(HANDS),
            HANDS,
            self.pts[0],
            self.pts[1]
        );
        if self.ph != Ph::Deal {
            s.push_str(&format!(
                "Rolul tău inițial (J{}): {}\n",
                me + 1,
                ROLES[self.orig[me] as usize]
            ));
            for &(slot, r) in &self.facts[me] {
                s.push_str(&format!("  știi: locul {slot} era {}\n", ROLES[r as usize]));
            }
            s.push_str(&format!(
                "Loc 0 = J1, 1 = J2, 2–4 = masă. Faza: {}\n",
                match self.ph {
                    Ph::Night(_) => "noapte",
                    Ph::Dawn => "zori",
                    Ph::Day(_) => "zi",
                    _ => "final",
                }
            ));
        }
        s
    }
    fn view(&mut self, w: Option<Player>) {
        self.viewer = w;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn noaptea_ok() {
        check(Noaptea::with_seed, 300);
    }
    #[test]
    fn judge_rules() {
        // lup executat: sătenii câștigă
        assert_eq!(judge(&[0, 3, 1, 1, 1, 1], [false, true]), -1);
        // tăbăcar executat câștigă
        assert_eq!(judge(&[4, 3, 1, 1, 1, 1], [false, true]), 1);
        // fără lup în joc, nimeni nu moare: remiză (ambii câștigă)
        assert_eq!(judge(&[3, 3, 0, 1, 1, 1], [false, false]), 0);
    }

    #[test]
    fn ai_nu_vede_rolurile_necunoscute() {
        // lume alternativă: rolurile din locurile pe care jucătorul nu le știe se amestecă între ele
        crate::p3_comun::ai_blind_check(
            "noaptea_lupilor",
            |s| {
                let mut g = Noaptea::with_seed(s);
                g.step("trag");
                B(g)
            },
            |g, me, seed| {
                let n = &mut g.0;
                let known: Vec<usize> = n.facts[me.idx()].iter().map(|f| f.0).collect();
                let free: Vec<usize> = (0..6).filter(|i| !known.contains(i)).collect();
                let swapped = n.cur[0] != n.orig[0] || n.cur[1] != n.orig[1];
                let mut vals: Vec<u8> = free.iter().map(|&i| n.orig[i]).collect();
                crate::p3_comun::shuffle_with(&mut vals, seed);
                for (k, &i) in free.iter().enumerate() {
                    n.orig[i] = vals[k];
                }
                n.cur = final_roles(&n.orig, swapped);
            },
            12,
        );
    }

    #[test]
    fn votul_nu_citeste_cartile_necunoscute() {
        polytabletop_core::ai::set_time_budget_override_ms(Some(6));
        let mut g = Noaptea::with_seed(6);
        g.step("trag");
        for _ in 0..2 {
            let m = g.moves()[0].clone();
            g.step(&m);
        }
        g.step("zori");
        assert!(g.ph == Ph::Day(0));
        let a = ai_of(&g, 3);
        // schimbăm cărțile pe care J1 nu le știe (masa și cartea lui J2), păstrând ce știe
        let mut h = g.clone();
        let known: Vec<usize> = g.facts[0].iter().map(|f| f.0).collect();
        let mut idx: Vec<usize> = (0..6).filter(|i| !known.contains(i)).collect();
        idx.reverse();
        let vals: Vec<u8> = idx.iter().map(|&i| g.orig[i]).collect();
        let free: Vec<usize> = (0..6).filter(|i| !known.contains(i)).collect();
        for (k, &i) in free.iter().enumerate() {
            h.orig[i] = vals[k];
        }
        h.cur = h.orig;
        assert_eq!(a, ai_of(&h, 3));
    }
}
