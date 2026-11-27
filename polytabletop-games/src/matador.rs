//! Matador (domino, Europa / Caraibe), set dublu-șase de 28 de piese, 2 jucători.
//!
//! * Fiecare primește 7 piese, restul formează talonul. Primul jucător pune orice piesă.
//! * Pe capetele liniei se pun piese ale căror jumătăți **însumează 7** cu capătul (1+6, 2+5, 3+4);
//!   jumătatea opusă devine noul capăt. **Matadorii** — 0-0, 1-6, 2-5, 3-4 — se pun oricând pe oricare
//!   capăt, cu oricare jumătate lipită. Blancul (0) nu însumează 7 cu nimic, deci un capăt 0 se închide
//!   numai cu un matador.
//! * Dacă nu ai nicio piesă de pus, tragi din talon (`trag`) până poți juca; talonul gol = `pas`.
//! * Câștigă cine rămâne primul fără piese; dacă amândoi pasează la rând (blocaj), câștigă cine are
//!   mai puține puncte pe piesele rămase (egal = remiză). O singură mână.
//!   Informație ascunsă: mâna adversarului și talonul; `render` arată doar mâna ta.
//!   Mutări: `6-1@s=1` (piesa 6-1 la capătul stâng; noul capăt va fi 1), `...@d=...` la dreapta,
//!   prima piesă `6-1`, `trag`, `pas`.

use crate::cards::Viewer;
use crate::cards52::shuffle;
use crate::q5_util::verdict;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    First(u8),
    Play(u8, u8, u8), // piesa, capătul (0 stânga / 1 dreapta), noul capăt
    Draw,
    Pass,
}

#[derive(Clone)]
pub struct Matador {
    hands: [Vec<u8>; 2],
    stock: Vec<u8>,
    ends: Option<(u8, u8)>,
    side: Player,
    passes: u8,
    fresh: Option<u8>,
    viewer: Viewer,
    log: String,
}

impl Default for Matador {
    fn default() -> Self {
        Self::new()
    }
}

fn halves(t: u8) -> (u8, u8) {
    (t / 7, t % 7)
}
fn tile(a: u8, b: u8) -> u8 {
    a.max(b) * 7 + a.min(b)
}
fn pips(t: u8) -> i32 {
    let (a, b) = halves(t);
    (a + b) as i32
}
fn is_matador(t: u8) -> bool {
    let (a, b) = halves(t);
    a + b == 7 || (a == 0 && b == 0)
}
fn ttext(t: u8) -> String {
    let (a, b) = halves(t);
    format!("{a}-{b}")
}

impl Matador {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut all: Vec<u8> = Vec::new();
        for a in 0..7u8 {
            for b in 0..=a {
                all.push(tile(a, b));
            }
        }
        shuffle(&mut all, &mut Rng::new(seed));
        let mut h0: Vec<u8> = all.drain(..7).collect();
        let mut h1: Vec<u8> = all.drain(..7).collect();
        h0.sort_unstable();
        h1.sort_unstable();
        Matador {
            hands: [h0, h1],
            stock: all,
            ends: None,
            side: Player::One,
            passes: 0,
            fresh: None,
            viewer: Viewer::Fixed(Player::One),
            log: String::new(),
        }
    }

    pub fn ends(&self) -> Option<(u8, u8)> {
        self.ends
    }

    fn plays(&self, who: Player) -> Vec<Move> {
        let mut v = Vec::new();
        let hand = &self.hands[who.idx()];
        match self.ends {
            None => {
                for &t in hand {
                    v.push(Move::First(t));
                }
            }
            Some((l, r)) => {
                for &t in hand {
                    let (a, b) = halves(t);
                    for (e, val) in [(0u8, l), (1u8, r)] {
                        let mut news: Vec<u8> = Vec::new();
                        if is_matador(t) {
                            news.push(b);
                            news.push(a);
                        } else {
                            if a + val == 7 {
                                news.push(b);
                            }
                            if b + val == 7 {
                                news.push(a);
                            }
                        }
                        news.sort_unstable();
                        news.dedup();
                        for n in news {
                            v.push(Move::Play(t, e, n));
                        }
                    }
                }
            }
        }
        v
    }

    fn pip_total(&self, p: usize) -> i32 {
        self.hands[p].iter().map(|&t| pips(t)).sum()
    }
}

impl Game for Matador {
    type Move = Move;

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = match who {
            Some(p) => Viewer::Fixed(p),
            None => Viewer::HotSeat,
        };
    }

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        let v = self.plays(self.side);
        if !v.is_empty() {
            return v;
        }
        if !self.stock.is_empty() {
            vec![Move::Draw]
        } else {
            vec![Move::Pass]
        }
    }

    fn apply(&mut self, m: &Move) {
        let me = self.side;
        self.fresh = None;
        match m {
            Move::First(t) => {
                let h = &mut self.hands[me.idx()];
                h.retain(|x| x != t);
                let (a, b) = halves(*t);
                self.ends = Some((a, b));
                self.passes = 0;
                self.side = me.other();
                self.log = format!("J{} pune {}.", me.idx() + 1, ttext(*t));
            }
            Move::Play(t, e, n) => {
                self.hands[me.idx()].retain(|x| x != t);
                let (l, r) = self.ends.unwrap();
                self.ends = Some(if *e == 0 { (*n, r) } else { (l, *n) });
                self.passes = 0;
                self.side = me.other();
                self.log = format!(
                    "J{} pune {} la capătul {}.",
                    me.idx() + 1,
                    ttext(*t),
                    if *e == 0 { "stâng" } else { "drept" }
                );
            }
            Move::Draw => {
                let t = self.stock.pop().unwrap();
                self.hands[me.idx()].push(t);
                self.hands[me.idx()].sort_unstable();
                self.fresh = Some(t);
                self.log = format!("J{} trage din talon.", me.idx() + 1);
            }
            Move::Pass => {
                self.passes += 1;
                self.side = me.other();
                self.log = format!("J{} pasează.", me.idx() + 1);
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        for p in [Player::One, Player::Two] {
            if self.hands[p.idx()].is_empty() {
                return Some(Outcome::Win(p));
            }
        }
        if self.passes >= 2 {
            let (a, b) = (self.pip_total(0), self.pip_total(1));
            return Some(match a.cmp(&b) {
                std::cmp::Ordering::Less => Outcome::Win(Player::One),
                std::cmp::Ordering::Greater => Outcome::Win(Player::Two),
                std::cmp::Ordering::Equal => Outcome::Draw,
            });
        }
        None
    }

    /// Doar propria mână (fără piesa abia trasă) și NUMĂRUL pieselor adversarului.
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mine: i32 = self.hands[me]
            .iter()
            .filter(|&&t| Some(t) != self.fresh)
            .map(|&t| pips(t))
            .sum();
        let matadors = self.hands[me]
            .iter()
            .filter(|&&t| Some(t) != self.fresh && is_matador(t))
            .count() as i32;
        (self.hands[1 - me].len() as i32 - self.hands[me].len() as i32) * 30 - mine + matadors * 4
    }

    fn render(&self) -> String {
        let mut s = String::from("Matador — capetele liniei trebuie să însumeze 7 cu piesa pusă (matadorii merg oriunde)\n");
        match self.ends {
            None => s.push_str("Linia e goală: prima piesă la alegere.\n"),
            Some((l, r)) => s.push_str(&format!("Capete: stâng {l} | drept {r}\n")),
        }
        s.push_str(&format!("Talon: {} piese.\n", self.stock.len()));
        let v = self.viewer.who(self.side);
        let o = v.other();
        s.push_str(&format!(
            "Jucător {} (ascuns) are {} piese.\n",
            o.idx() + 1,
            self.hands[o.idx()].len()
        ));
        let h: Vec<String> = self.hands[v.idx()].iter().map(|&t| ttext(t)).collect();
        s.push_str(&format!(
            "Mâna ta (Jucător {}): {}\n",
            v.idx() + 1,
            h.join(" ")
        ));
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        let fin = verdict(self.outcome());
        if fin.is_empty() {
            s.push_str(&format!("La mutare: Jucător {}\n", self.side.idx() + 1));
        } else {
            s.push_str(&fin);
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::First(t) => ttext(*t),
            Move::Play(t, e, n) => {
                format!("{}@{}={}", ttext(*t), if *e == 0 { "s" } else { "d" }, n)
            }
            Move::Draw => "trag".into(),
            Move::Pass => "pas".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        // Adâncimea 0 = o singură mutare la TOATE nivelurile: la Hard/Expert adâncimea 1 ar extinde
        // răspunsurile adversarului din mâna lui reală (informație ascunsă).
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q5_util::testkit::check_game;

    #[test]
    fn matching_rule_and_matadors() {
        let mut g = Matador::with_seed(1);
        g.hands[0] = vec![tile(6, 3), tile(4, 4), tile(6, 1), tile(5, 0)];
        g.ends = Some((4, 2));
        let m: Vec<String> = g
            .plays(Player::One)
            .iter()
            .map(|x| g.format_move(x))
            .collect();
        // 6-3: 3+4=7 pe capătul 4 → capăt nou 6 (stânga); 5-2? nu e în mână
        assert!(m.contains(&"6-3@s=6".to_string()), "{m:?}");
        // 4-4 nu poate (4+4≠7), 5-0: 5+2=7 pe capătul drept → noul capăt 0
        assert!(!m.iter().any(|x| x.starts_with("4-4")), "{m:?}");
        assert!(m.contains(&"5-0@d=0".to_string()), "{m:?}");
        // matadorul 6-1 merge pe orice capăt, cu oricare jumătate
        for e in ["6-1@s=1", "6-1@s=6", "6-1@d=1", "6-1@d=6"] {
            assert!(m.contains(&e.to_string()), "{e} în {m:?}");
        }
    }

    #[test]
    fn blank_end_needs_a_matador() {
        let mut g = Matador::with_seed(2);
        g.hands[0] = vec![tile(6, 1), tile(3, 3), tile(4, 3), tile(2, 0)];
        g.ends = Some((0, 0));
        let m: Vec<String> = g
            .plays(Player::One)
            .iter()
            .map(|x| g.format_move(x))
            .collect();
        assert!(
            m.iter()
                .all(|x| x.starts_with("6-1") || x.starts_with("4-3")),
            "{m:?}"
        );
        assert!(!m.is_empty());
    }

    #[test]
    fn viewer_and_blind_ai() {
        let mut a = Matador::with_seed(4);
        a.apply(&a.legal_moves()[0].clone());
        let mut b = a.clone();
        // schimbă mâna adversarului cu talonul
        let opp = a.side.other().idx();
        std::mem::swap(&mut b.hands[opp], &mut b.stock);
        b.hands[opp].truncate(a.hands[opp].len());
        while b.hands[opp].len() < a.hands[opp].len() {
            b.hands[opp].push(0);
        }
        for seed in 1..8 {
            assert_eq!(
                polytabletop_core::GameObj::ai_move(&a, seed),
                polytabletop_core::GameObj::ai_move(&b, seed)
            );
        }
        let mut one = a.clone();
        one.set_viewer(Some(Player::One));
        let mut two = a.clone();
        two.set_viewer(Some(Player::Two));
        assert_ne!(one.render(), two.render());
    }

    #[test]
    fn random_and_ai_games() {
        check_game(Matador::with_seed, 8, 600);
    }
}

#[cfg(test)]
mod a3_audit {
    use super::*;

    #[test]
    fn ai_ignores_hidden_hand_and_stock_at_every_level() {
        crate::p3_comun::ai_blind_check(
            "matador",
            Matador::with_seed,
            |g, me, s| {
                let h = 1 - me.idx();
                crate::p3_comun::mix_unseen(&mut g.hands[h], &mut g.stock, 0, s);
                g.hands[h].sort_unstable();
            },
            10,
        );
    }
}
