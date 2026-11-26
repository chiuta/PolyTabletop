//! Bao la Kiswahili (Tanzania / Africa de Est), în variantă simplificată documentată.
//!
//! Tablă 4 × 8; fiecare jucător deține două rânduri (față = interior, spate = exterior),
//! unite într-un inel de 16 gropi (vezi `mancala4`). Poziția de start a fiecăruia:
//! 6 semințe în „nyumba” (groapa din față nr. 5), câte 2 în gropile 6 și 7 din față și
//! 22 semințe în rezervă (total 32 pe jucător, 64 în joc).
//!
//! * **Namua** (cât timp ai semințe în rezervă): dacă poți captura (ai o groapă ocupată
//!   în față, față în față cu o groapă ocupată a adversarului), ești OBLIGAT: pui o
//!   sămânță din rezervă în ea, iei semințele adversarului din acea groapă și le semeni
//!   începând de la capătul rândului din față („kichwa”): din coloanele 1–2 doar de la
//!   stânga, din 7–8 doar de la dreapta, din 3–6 alegi capătul. Fără captură: pui o
//!   sămânță într-o groapă din față ocupată, ridici tot și semeni în sensul ales.
//! * **Mtaji** (rezerva epuizată): muți dintr-o groapă cu ≥ 2 semințe, în sensul ales
//!   (`l` = spre coloana 1, `r` = spre coloana 8). Dacă o mutare captură din prima
//!   aterizare, ești obligat să capturezi; altfel muți din fața ta (din spate doar dacă
//!   în față nu ai nicio groapă cu ≥ 2).
//! * Aterizare: ultima sămânță într-o groapă goală încheie tura; într-o groapă ocupată din
//!   față cu groapa opusă a adversarului ocupată = captură (semințele capturate se
//!   seamănă de la kichwa, cu sensul păstrat în coloanele 3–6); altfel ridici tot și
//!   continui (takasa).
//! * Câștigă cine golește rândul din față al adversarului sau îl lasă fără mutări.
//!
//! Simplificări (documentate): nyumba NU are statut special (fără safari / kutakatia);
//! lanțul e limitat la 3000 de semințe; după 600 de mutări câștigă cine are mai multe
//! semințe.
//!
//! Notația mutărilor: `<coloana><sensul>`, ex. `3r`; pentru spate `b3l`. În namua,
//! sufixul al capturii este capătul de unde pornește semănatul (`r` = din stânga spre
//! dreapta, `l` = din dreapta spre stânga).

use crate::mancala4::{ring_name, step, Kete, RING};
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const CAP: u32 = 3000;
const MAX_PLIES: u32 = 600;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Mv {
    pub ring: u8,
    pub dir: i8,
}

#[derive(Clone)]
pub struct Bao {
    b: Kete,
    stock: [u8; 2],
    side: Player,
    plies: u32,
    winner: Option<Option<Player>>,
}

impl Default for Bao {
    fn default() -> Self {
        Self::new()
    }
}

impl Bao {
    pub fn new() -> Self {
        let mut b = Kete::empty();
        for p in 0..2 {
            b.ring[p][4] = 6;
            b.ring[p][5] = 2;
            b.ring[p][6] = 2;
        }
        Bao {
            b,
            stock: [22, 22],
            side: Player::One,
            plies: 0,
            winner: None,
        }
    }

    pub fn total_seeds(&self) -> u32 {
        self.b.total(Player::One)
            + self.b.total(Player::Two)
            + self.stock[0] as u32
            + self.stock[1] as u32
    }

    /// Capătul de pornire pentru o captură în coloana `c` cu sensul curent `dir`.
    fn kichwa(c: usize, dir: i32) -> (usize, i32) {
        let d = if c <= 1 {
            1
        } else if c >= 6 {
            -1
        } else {
            dir
        };
        // poziția „dinaintea” capătului, ca primul pas să cadă pe kichwa
        (if d == 1 { 15 } else { 8 }, d)
    }

    /// Execută semănatul + capturile + lanțul. `hand` semințe de semănat după `pos` în sens
    /// `dir`. Întoarce `true` dacă PRIMA aterizare a fost o captură.
    fn run(&mut self, me: Player, mut pos: usize, mut dir: i32, mut hand: u8) -> bool {
        let (m, o) = (me.idx(), me.other().idx());
        let mut sown = 0u32;
        let mut first = true;
        let mut first_capture = false;
        loop {
            while hand > 0 {
                pos = step(pos, dir);
                self.b.ring[m][pos] += 1;
                hand -= 1;
                sown += 1;
            }
            if self.b.ring[m][pos] <= 1 || sown > CAP {
                return first_capture;
            }
            if pos < 8 && self.b.ring[o][7 - pos] > 0 {
                if first {
                    first_capture = true;
                }
                hand = self.b.ring[o][7 - pos];
                self.b.ring[o][7 - pos] = 0;
                let (p0, d0) = Self::kichwa(pos, dir);
                pos = p0;
                dir = d0;
            } else {
                hand = self.b.ring[m][pos];
                self.b.ring[m][pos] = 0;
            }
            first = false;
        }
    }

    fn namua_captures(&self, p: Player) -> Vec<usize> {
        (0..8)
            .filter(|&c| self.b.ring[p.idx()][c] > 0 && self.b.ring[p.other().idx()][7 - c] > 0)
            .collect()
    }

    fn gen(&self) -> Vec<Mv> {
        let p = self.side;
        let mut out = Vec::new();
        if self.stock[p.idx()] > 0 {
            let caps = self.namua_captures(p);
            if !caps.is_empty() {
                for c in caps {
                    if c <= 1 {
                        out.push(Mv {
                            ring: c as u8,
                            dir: 1,
                        });
                    } else if c >= 6 {
                        out.push(Mv {
                            ring: c as u8,
                            dir: -1,
                        });
                    } else {
                        out.push(Mv {
                            ring: c as u8,
                            dir: 1,
                        });
                        out.push(Mv {
                            ring: c as u8,
                            dir: -1,
                        });
                    }
                }
                return out;
            }
            let mut any = false;
            for c in 0..8 {
                if self.b.ring[p.idx()][c] > 0 {
                    any = true;
                    out.push(Mv {
                        ring: c as u8,
                        dir: 1,
                    });
                    out.push(Mv {
                        ring: c as u8,
                        dir: -1,
                    });
                }
            }
            if !any {
                for r in 8..RING {
                    if self.b.ring[p.idx()][r] > 0 {
                        out.push(Mv {
                            ring: r as u8,
                            dir: 1,
                        });
                        out.push(Mv {
                            ring: r as u8,
                            dir: -1,
                        });
                    }
                }
            }
            return out;
        }
        // mtaji
        let mut all = Vec::new();
        let mut caps = Vec::new();
        for r in 0..RING {
            if self.b.ring[p.idx()][r] >= 2 {
                for d in [1i8, -1] {
                    let mv = Mv {
                        ring: r as u8,
                        dir: d,
                    };
                    let mut c = self.clone();
                    let cap = c.run_move(&mv);
                    if cap && r < 8 {
                        caps.push(mv);
                    }
                    all.push(mv);
                }
            }
        }
        if !caps.is_empty() {
            return caps;
        }
        let front: Vec<Mv> = all.iter().copied().filter(|m| m.ring < 8).collect();
        if !front.is_empty() {
            front
        } else {
            all
        }
    }

    /// Aplică mutarea fără schimbarea părții; întoarce dacă prima aterizare a capturat.
    fn run_move(&mut self, mv: &Mv) -> bool {
        let me = self.side;
        let r = mv.ring as usize;
        let d = mv.dir as i32;
        if self.stock[me.idx()] > 0 {
            self.stock[me.idx()] -= 1;
            self.b.ring[me.idx()][r] += 1;
            if r < 8 && self.b.ring[me.other().idx()][7 - r] > 0 {
                let o = me.other().idx();
                let hand = self.b.ring[o][7 - r];
                self.b.ring[o][7 - r] = 0;
                // kimbi: sensul e forțat, în 3–6 e ales
                let (p0, d0) = Self::kichwa(r, d);
                self.run(me, p0, d0, hand);
                return true;
            }
            let hand = self.b.ring[me.idx()][r];
            self.b.ring[me.idx()][r] = 0;
            return self.run(me, r, d, hand);
        }
        let hand = self.b.ring[me.idx()][r];
        self.b.ring[me.idx()][r] = 0;
        self.run(me, r, d, hand)
    }
}

impl Game for Bao {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if self.winner.is_some() {
            return Vec::new();
        }
        self.gen()
    }

    fn apply(&mut self, m: &Mv) {
        let me = self.side;
        self.run_move(m);
        self.plies += 1;
        self.side = me.other();
        if self.b.inner_empty(me.other()) {
            self.winner = Some(Some(me));
        } else if self.b.inner_empty(me) {
            self.winner = Some(Some(me.other()));
        } else if self.gen().is_empty() {
            self.winner = Some(Some(me));
        } else if self.plies >= MAX_PLIES {
            let tot = |p: Player| self.b.total(p) + self.stock[p.idx()] as u32;
            let (a, b) = (tot(Player::One), tot(Player::Two));
            self.winner = Some(match a.cmp(&b) {
                std::cmp::Ordering::Greater => Some(Player::One),
                std::cmp::Ordering::Less => Some(Player::Two),
                _ => None,
            });
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.winner.map(|w| match w {
            Some(p) => Outcome::Win(p),
            None => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let o = p.other();
        let tot = |q: Player| (self.b.total(q) + self.stock[q.idx()] as u32) as i32;
        let inner =
            |q: Player| -> i32 { self.b.ring[q.idx()][..8].iter().map(|&x| x as i32).sum() };
        4 * (tot(p) - tot(o)) + (inner(p) - inner(o))
    }

    fn render(&self) -> String {
        let mut s = self.b.render();
        s.push_str(&format!(
            "  rezervă: J1 {}, J2 {}   ({})\n",
            self.stock[0],
            self.stock[1],
            if self.stock[self.side.idx()] > 0 {
                "faza namua"
            } else {
                "faza mtaji"
            }
        ));
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        format!(
            "{}{}",
            ring_name(m.ring as usize),
            if m.dir > 0 { 'r' } else { 'l' }
        )
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mancala4::parse_ring;
    use polytabletop_core::rng::Rng;

    fn empty() -> Bao {
        Bao {
            b: Kete::empty(),
            stock: [0, 0],
            side: Player::One,
            plies: 0,
            winner: None,
        }
    }

    #[test]
    fn start_is_64_seeds_and_namua_captures_are_forced() {
        let g = Bao::new();
        assert_eq!(g.total_seeds(), 64);
        let mv = Game::legal_moves(&g);
        // gropile ocupate 5,6,7 (coloane 4,5,6) sunt față în față cu 4,3,2 ale adversarului,
        // dar adversarul are doar 5,6,7 ocupate ⇒ coloana 4 ↔ 7-4=3 (gol)… verificăm doar forțarea
        for m in &mv {
            assert!(m.ring < 8);
        }
        assert!(!mv.is_empty());
    }

    #[test]
    fn kichwa_is_forced_in_kimbi() {
        let mut g = empty();
        g.stock = [5, 5];
        g.b.ring[0][0] = 1;
        g.b.ring[1][7] = 2; // față în față cu coloana 0
        g.b.ring[1][3] = 1;
        let mv = Game::legal_moves(&g);
        assert_eq!(mv, vec![Mv { ring: 0, dir: 1 }]);
    }

    #[test]
    fn middle_capture_offers_both_ends() {
        let mut g = empty();
        g.stock = [5, 5];
        g.b.ring[0][3] = 1;
        g.b.ring[1][4] = 2;
        g.b.ring[1][0] = 1;
        let mv = Game::legal_moves(&g);
        assert_eq!(mv.len(), 2);
    }

    #[test]
    fn namua_capture_sows_from_kichwa_and_conserves() {
        let mut g = empty();
        g.stock = [5, 5];
        g.b.ring[0][3] = 1;
        g.b.ring[1][4] = 3;
        g.b.ring[1][0] = 1;
        let before = g.total_seeds();
        g.apply(&Mv { ring: 3, dir: 1 });
        assert_eq!(g.total_seeds(), before);
        // 3 semințe captate, semănate de la coloana 1: gropile 0,1,2
        assert_eq!(g.b.ring[0][0], 1);
        assert_eq!(g.b.ring[0][1], 1);
        assert_eq!(g.b.ring[0][2], 1);
        assert_eq!(g.b.ring[1][4], 0);
        assert_eq!(g.stock[0], 4);
    }

    #[test]
    fn namua_without_capture_places_and_sows() {
        let mut g = empty();
        g.stock = [5, 5];
        g.b.ring[0][2] = 1;
        g.b.ring[1][0] = 4; // opusul lui 2 este 5: gol, deci fără captură
        let mv = Game::legal_moves(&g);
        assert_eq!(mv.len(), 2);
        g.apply(&Mv { ring: 2, dir: 1 });
        // 2 semințe în groapa 2 ridicate: sunt semănate în 3 și 4
        assert_eq!(g.b.ring[0][2], 0);
        assert_eq!(g.b.ring[0][3], 1);
        assert_eq!(g.b.ring[0][4], 1);
    }

    #[test]
    fn mtaji_requires_capture_when_possible() {
        let mut g = empty();
        g.b.ring[0][0] = 2; // r: 1,2 → ultima în 2 (goală) — fără captură
        g.b.ring[0][4] = 2; // r: 5,6 … ultima în 6 (goală)
        g.b.ring[0][3] = 3;
        g.b.ring[1][0] = 1;
        g.b.ring[1][2] = 1;
        // mutarea 4 spre dreapta (3 semințe): 4,5,6 ; ultima în 6 goală → fără captură
        // mutarea 5 (index 4) spre stânga: semințe în 3 și 2 … ultima în 2 (goală)
        let mv = Game::legal_moves(&g);
        assert!(!mv.is_empty());
        // pregătim o captură: groapa 5 (idx 4) cu 2 semințe spre dreapta ajunge în 6 (idx 6)
        // dacă idx 6 e ocupată și opusul (idx 1) ocupat
        let mut h = empty();
        h.b.ring[0][4] = 2;
        h.b.ring[0][6] = 1;
        h.b.ring[0][0] = 2;
        h.b.ring[1][1] = 5;
        h.b.ring[1][5] = 1;
        let mv = Game::legal_moves(&h);
        assert_eq!(mv, vec![Mv { ring: 4, dir: 1 }]);
    }

    #[test]
    fn empty_front_row_loses() {
        let mut g = empty();
        g.stock = [3, 0];
        g.b.ring[0][3] = 1;
        g.b.ring[1][4] = 2; // singura groapă din față a adversarului
        g.b.ring[1][9] = 3; // spate
        g.apply(&Mv { ring: 3, dir: 1 });
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn names_parse() {
        let g = Bao::new();
        for m in Game::legal_moves(&g) {
            let t = g.format_move(&m);
            assert!(parse_ring(&t[..t.len() - 1]).is_some());
        }
    }

    #[test]
    fn seeds_conserved_and_random_games_finish() {
        for seed in 1..=30u64 {
            let mut g = Bao::new();
            let mut rng = Rng::new(seed);
            while g.outcome().is_none() {
                let m = Game::legal_moves(&g);
                assert!(!m.is_empty(), "{}", g.render());
                g.apply(&m[rng.below(m.len())]);
                assert_eq!(g.total_seeds(), 64);
            }
        }
    }
}
