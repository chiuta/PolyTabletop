//! „Zarurile mincinosului” — Liar's Dice (Perudo / Dudo / Cacho), 2 jucători, varianta Perudo.
//!
//! * Fiecare are 5 zaruri sub un păhar; la începutul rundei aruncați în secret. Se licitează o cantitate
//!   dintr-o față pe *toate* zarurile de pe masă (ale ambilor): `3x4` = „cel puțin trei de patru”.
//! * Ași (fața 1) sunt jokeri: la o licitație pe fețele 2–6 ei se numără ca fața licitată; o licitație
//!   pe ași (`2x1`) numără doar ași.
//! * O licitație nouă trebuie să fie mai mare: pe fețele 2–6, o cantitate mai mare (orice față) sau
//!   aceeași cantitate cu față mai mare; spre ași se poate coborî la jumătate din cantitate (rotunjită în
//!   sus); de la ași la altă față cantitatea trebuie să fie cel puțin dublul + 1; pe ași se urcă cantitatea.
//!   Prima licitație a rundei nu poate fi pe ași.
//! * În locul unei licitații poți spune `dudo` (contest): se arată zarurile (`arata`). Dacă licitația e
//!   adevărată (cel puțin cantitatea cerută), pierde un zar contestatarul; altfel pierde un zar licitatorul.
//!   Cine pierde un zar începe runda următoare. Pierde cine rămâne fără zaruri.
//!
//! Omise: „calza” (a spune că licitația e exactă) și runda „palifico” (cu un singur zar rămas).
//! Informație ascunsă: vezi doar zarurile tale. AI-ul nu vede zarurile adversarului: estimează
//! probabilitatea licitației din propriile zaruri și simulează un adversar rațional pe zaruri eșantionate.

use crate::cards::{handover_banner, Viewer};
use polytabletop_core::ai::level;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const START_DICE: usize = 5;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Bid,
    Reveal,
    Over,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Bid(u8, u8),
    Dudo,
    Reveal,
}

#[derive(Clone)]
pub struct LiarsDice {
    rng: Rng,
    dice: [Vec<u8>; 2],
    bid: Option<(u8, u8)>,
    bidder: Player,
    side: Player,
    starter: Player,
    phase: Phase,
    prev: Player,
    log: String,
    viewer: Viewer,
}

impl Default for LiarsDice {
    fn default() -> Self {
        Self::new()
    }
}

/// Numărul de zaruri din `d` care contează pentru licitația pe fața `f`.
pub fn matching(d: &[u8], f: u8) -> usize {
    d.iter().filter(|&&x| x == f || (f != 1 && x == 1)).count()
}

/// Licitațiile permise după `prev`, cu cel mult `total` zaruri pe masă.
pub fn next_bids(prev: Option<(u8, u8)>, total: u8) -> Vec<(u8, u8)> {
    let mut v = Vec::new();
    match prev {
        None => {
            for q in 1..=total {
                for f in 2..=6 {
                    v.push((q, f));
                }
            }
        }
        Some((q, f)) if f != 1 => {
            for q2 in q..=total {
                for f2 in 2..=6 {
                    if q2 > q || f2 > f {
                        v.push((q2, f2));
                    }
                }
            }
            for q2 in q.div_ceil(2).max(1)..=total {
                v.push((q2, 1));
            }
        }
        Some((q, _)) => {
            for q2 in (q + 1)..=total {
                v.push((q2, 1));
            }
            for q2 in (2 * q + 1)..=total {
                for f2 in 2..=6 {
                    v.push((q2, f2));
                }
            }
        }
    }
    v
}

/// P(X >= k) pentru X ~ Binomial(n, p).
fn p_ge(n: usize, p: f64, k: i32) -> f64 {
    if k <= 0 {
        return 1.0;
    }
    if k as usize > n {
        return 0.0;
    }
    let mut total = 0.0;
    let mut c = 1.0f64; // C(n, j)
    for j in 0..=n {
        if j > 0 {
            c = c * (n - j + 1) as f64 / j as f64;
        }
        if j as i32 >= k {
            total += c * p.powi(j as i32) * (1.0 - p).powi((n - j) as i32);
        }
    }
    total
}

fn face_p(f: u8) -> f64 {
    if f == 1 {
        1.0 / 6.0
    } else {
        1.0 / 3.0
    }
}

/// Probabilitatea ca licitația (q, f) să fie adevărată, din zarurile proprii și `n_opp` necunoscute.
pub fn p_true(mine: &[u8], n_opp: usize, q: u8, f: u8) -> f64 {
    let m = matching(mine, f) as i32;
    p_ge(n_opp, face_p(f), q as i32 - m)
}

impl LiarsDice {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = LiarsDice {
            rng: Rng::new(seed),
            dice: [Vec::new(), Vec::new()],
            bid: None,
            bidder: Player::One,
            side: Player::One,
            starter: Player::One,
            phase: Phase::Bid,
            prev: Player::One,
            log: String::new(),
            viewer: Viewer::Fixed(Player::One),
        };
        g.dice = [vec![0; START_DICE], vec![0; START_DICE]];
        g.roll_all();
        g
    }

    /// Re-aruncă zarurile pe care `me` nu le vede (teste: AI-ul nu trebuie să le citească).
    pub fn resample_hidden(&self, me: Player, seed: u64) -> Self {
        let mut g = self.clone();
        let mut rng = Rng::new(seed);
        let o = 1 - me.idx();
        let n = g.dice[o].len();
        g.dice[o] = (0..n).map(|_| rng.d6()).collect();
        g.dice[o].sort_unstable();
        g.rng = Rng::new(seed ^ 0x55);
        g
    }

    pub fn dice_counts(&self) -> (usize, usize) {
        (self.dice[0].len(), self.dice[1].len())
    }

    fn roll_all(&mut self) {
        for p in 0..2 {
            let n = self.dice[p].len();
            self.dice[p] = (0..n).map(|_| self.rng.d6()).collect();
            self.dice[p].sort_unstable();
        }
    }

    fn total(&self) -> u8 {
        (self.dice[0].len() + self.dice[1].len()) as u8
    }

    fn fmt_dice(d: &[u8]) -> String {
        if d.is_empty() {
            return "—".into();
        }
        d.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl Game for LiarsDice {
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
        match self.phase {
            Phase::Over => Vec::new(),
            Phase::Reveal => vec![Move::Reveal],
            Phase::Bid => {
                let mut v: Vec<Move> = next_bids(self.bid, self.total())
                    .into_iter()
                    .map(|(q, f)| Move::Bid(q, f))
                    .collect();
                if self.bid.is_some() {
                    v.push(Move::Dudo);
                }
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        self.prev = self.side;
        match *m {
            Move::Bid(q, f) => {
                self.bid = Some((q, f));
                self.bidder = self.side;
                self.side = self.side.other();
            }
            Move::Dudo => {
                self.phase = Phase::Reveal;
            }
            Move::Reveal => {
                let (q, f) = self.bid.unwrap();
                let cnt = matching(&self.dice[0], f) + matching(&self.dice[1], f);
                let challenger = self.bidder.other();
                let loser = if cnt >= q as usize {
                    challenger
                } else {
                    self.bidder
                };
                self.log = format!(
                    "Dudo! Licitația {}x{}: pe masă sunt {} — pierde un zar J{}",
                    q,
                    f,
                    cnt,
                    loser.idx() + 1
                );
                self.dice[loser.idx()].pop();
                if self.dice[loser.idx()].is_empty() {
                    self.phase = Phase::Over;
                    return;
                }
                self.roll_all();
                self.bid = None;
                self.starter = loser;
                self.side = loser;
                self.phase = Phase::Bid;
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        Some(if self.dice[0].is_empty() {
            Outcome::Win(Player::Two)
        } else {
            Outcome::Win(Player::One)
        })
    }

    /// Evaluare cinstită: probabilitatea din zarurile proprii; contestația se judecă pe zaruri adverse
    /// eșantionate (niciodată pe cele reale), cu un adversar care contestă când licitația îi pare falsă.
    fn evaluate(&self, p: Player) -> i32 {
        let q = p.other();
        let mine = &self.dice[p.idx()];
        let n_opp = self.dice[q.idx()].len();
        let lead = 300 * (self.dice[p.idx()].len() as i32 - n_opp as i32);
        let Some((bq, bf)) = self.bid else {
            return lead;
        };
        let pt = p_true(mine, n_opp, bq, bf);
        match self.phase {
            Phase::Reveal => {
                // contestație: p e contestatarul dacă licitatorul e adversarul
                let v = if self.bidder == p {
                    2.0 * pt - 1.0
                } else {
                    1.0 - 2.0 * pt
                };
                lead + (v * 1000.0) as i32
            }
            Phase::Bid => {
                // p tocmai a licitat: adversarul răspunde
                let n = (6 * level().factor_x10() / 10).clamp(1, 120) as usize;
                let mut seed: u64 =
                    0xD1CE ^ ((bq as u64) << 8) ^ bf as u64 ^ ((n_opp as u64) << 16);
                for &d in mine {
                    seed = seed.rotate_left(5) ^ d as u64;
                }
                let mut rng = Rng::new(seed);
                let mut tot = 0.0;
                for _ in 0..n {
                    let opp: Vec<u8> = (0..n_opp).map(|_| rng.d6()).collect();
                    // ce crede adversarul: probabilitatea din zarurile lui și n_mine necunoscute
                    let p_opp = p_true(&opp, mine.len(), bq, bf);
                    if p_opp < 0.4 {
                        let cnt = matching(mine, bf) + matching(&opp, bf);
                        tot += if cnt >= bq as usize { 1.0 } else { -1.0 };
                    } else {
                        tot += 0.5 * (2.0 * pt - 1.0);
                    }
                }
                lead + (tot / n as f64 * 1000.0) as i32
            }
            Phase::Over => lead,
        }
    }

    fn render(&self) -> String {
        let mut s = String::new();
        if self.prev != self.side && self.phase == Phase::Bid {
            if let Viewer::HotSeat = self.viewer {
                s.push_str(&handover_banner(self.side));
            }
        }
        s.push_str(&format!(
            "Zarurile mincinosului — zaruri: J1 {}, J2 {}\n",
            self.dice[0].len(),
            self.dice[1].len()
        ));
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        if self.phase == Phase::Over {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        if let Some((q, f)) = self.bid {
            s.push_str(&format!(
                "Licitația curentă: {}x{} (J{})\n",
                q,
                f,
                self.bidder.idx() + 1
            ));
        } else {
            s.push_str("Începe o rundă nouă.\n");
        }
        if self.phase == Phase::Reveal {
            s.push_str(&format!(
                "Zaruri arătate: J1 [{}] | J2 [{}]\nScrie «arata».\n",
                Self::fmt_dice(&self.dice[0]),
                Self::fmt_dice(&self.dice[1])
            ));
            return s;
        }
        let v = self.viewer.who(self.side);
        s.push_str(&format!(
            "Mâna ta (J{}): {}\n",
            v.idx() + 1,
            Self::fmt_dice(&self.dice[v.idx()])
        ));
        s.push_str(&format!(
            "J{} are {} zaruri sub pahar\n",
            v.other().idx() + 1,
            self.dice[v.other().idx()].len()
        ));
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Bid(q, f) => format!("{q}x{f}"),
            Move::Dudo => "dudo".into(),
            Move::Reveal => "arata".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set(g: &mut LiarsDice, a: &[u8], b: &[u8]) {
        g.dice = [a.to_vec(), b.to_vec()];
    }

    #[test]
    fn bid_ordering_rules() {
        let nb = |p, t| next_bids(Some(p), t);
        // după 3x4: 3x5, 4x2.., nu 3x3, nu 2x6
        let v = nb((3, 4), 10);
        assert!(v.contains(&(3, 5)) && v.contains(&(4, 2)));
        assert!(!v.contains(&(3, 3)) && !v.contains(&(2, 6)));
        // ași: jumătate rotunjită în sus
        assert!(v.contains(&(2, 1)) && !v.contains(&(1, 1)));
        // de la ași: cantitate mai mare de ași sau 2q+1
        let a = nb((2, 1), 10);
        assert!(a.contains(&(3, 1)) && a.contains(&(5, 2)) && !a.contains(&(4, 6)));
        // prima licitație nu poate fi pe ași
        assert!(next_bids(None, 10).iter().all(|&(_, f)| f != 1));
    }

    #[test]
    fn aces_are_wild_except_when_bid() {
        assert_eq!(matching(&[1, 4, 4, 2], 4), 3);
        assert_eq!(matching(&[1, 4, 4, 2], 1), 1);
    }

    #[test]
    fn dudo_resolution() {
        let mut g = LiarsDice::with_seed(1);
        set(&mut g, &[1, 3, 3, 6, 6], &[3, 4, 5, 6, 1]);
        g.apply(&Move::Bid(4, 3)); // J1: 4 trei; pe masă: 3,3,1 + 3,1 = 5 >= 4
        g.apply(&Move::Dudo);
        assert_eq!(g.phase, Phase::Reveal);
        g.apply(&Move::Reveal);
        assert_eq!(g.dice[1].len(), 4); // contestatarul J2 pierde
        assert_eq!(g.side, Player::Two);
        assert_eq!(g.dice[0].len(), 5);
        let mut h = LiarsDice::with_seed(2);
        set(&mut h, &[2, 2, 5, 6, 6], &[3, 4, 5, 6, 4]);
        h.apply(&Move::Bid(5, 4));
        h.apply(&Move::Dudo);
        h.apply(&Move::Reveal);
        assert_eq!(h.dice[0].len(), 4); // licitatorul J1 pierde
        assert_eq!(h.side, Player::One);
    }

    #[test]
    fn probability_is_sane() {
        // 5 zaruri necunoscute, licitez 0 din 6: sigur adevărat; 6 din 5 zaruri: imposibil
        assert!((p_ge(5, 1.0 / 3.0, 0) - 1.0).abs() < 1e-12);
        assert_eq!(p_ge(5, 1.0 / 3.0, 6), 0.0);
        let p = p_true(&[4, 4, 1], 5, 3, 4);
        assert!(p > 0.99);
        let low = p_true(&[2, 3, 5], 5, 5, 4);
        assert!(low < 0.2);
    }

    #[test]
    fn viewer_shows_only_own_dice() {
        let mut g = LiarsDice::with_seed(3);
        Game::set_viewer(&mut g, None);
        let r = Game::render(&g);
        assert!(r.contains(&format!(
            "Mâna ta (J1): {}",
            LiarsDice::fmt_dice(&g.dice[0])
        )));
        Game::set_viewer(&mut g, Some(Player::Two));
        let r2 = Game::render(&g);
        assert!(r2.contains("Mâna ta (J2)"));
        assert!(!r2.contains(&format!(
            "Mâna ta (J1): {}",
            LiarsDice::fmt_dice(&g.dice[0])
        )));
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=10u64 {
            let mut g = LiarsDice::with_seed(seed);
            let mut rng = Rng::new(seed + 11);
            let mut n = 0;
            while g.outcome().is_none() && n < 5000 {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed}");
        }
    }
}

#[cfg(test)]
mod a2_audit {
    use super::*;

    #[test]
    fn ai_ignores_hidden_information_at_every_level() {
        crate::p3_comun::ai_blind_check_with(
            "liars_dice",
            LiarsDice::with_seed,
            |g, me, seed| *g = g.resample_hidden(me, seed),
            2,
            4,
            crate::p3_comun::Term::Check,
        );
    }
}
