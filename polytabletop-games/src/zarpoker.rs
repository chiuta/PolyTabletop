//! Poker de zaruri — joc de zaruri cu mâini de poker (variantă originală), 2 jucători.
//!
//! * 7 runde. În fiecare rundă fiecare jucător, pe rând, face o mână de 5 zaruri din
//!   până la 3 aruncări: după o aruncare poți **ține** unele zaruri (`tine 3 3 5`,
//!   `tine -`) și rearunci restul, sau te oprești (`fin`). După a treia aruncare te oprești.
//! * Mâinile, de la cea mai slabă: nimic (cel mai mare zar), pereche, două perechi, trei la fel,
//!   scară (1-2-3-4-5 sau 2-3-4-5-6), full, careu, cinci la fel. Egalitățile se rup după valorile
//!   zarurilor (grupul mai mare întâi). Câștigătorul rundei ia 1 punct; remiza nu dă puncte.
//! * Informație ascunsă: mâna terminată a primului jucător rămâne acoperită până termină al doilea.
//!   Aruncarea e o mutare forțată (`arunc`). AI-ul citește doar zarurile lui și punctele
//!   rundelor încheiate; nu se uită la mâna acoperită a adversarului (la încheierea rundei
//!   evaluarea ignoră rezultatul ei).
//!
//! Reguli: poker cu zaruri (domeniu public), clasamentul standard al mâinilor.

use crate::cards::{handover_banner, Viewer};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const ROUNDS: u8 = 7;
pub const HAND_NAMES: [&str; 8] = [
    "nimic",
    "pereche",
    "două perechi",
    "trei la fel",
    "scară",
    "full",
    "careu",
    "cinci la fel",
];

/// (categorie 0..=7, putere totală: categorie*1_000_000 + departajare).
pub fn strength(d: &[u8]) -> (usize, u32) {
    let mut cnt = [0u8; 7];
    for &x in d {
        cnt[x as usize] += 1;
    }
    let mut groups: Vec<(u8, u8)> = (1..=6u8)
        .filter(|&v| cnt[v as usize] > 0)
        .map(|v| (cnt[v as usize], v))
        .collect();
    groups.sort_unstable_by(|a, b| b.cmp(a));
    let pat: Vec<u8> = groups.iter().map(|g| g.0).collect();
    let straight = groups.len() == 5 && (cnt[1] == 0 || cnt[6] == 0);
    let cat = match pat.as_slice() {
        [5] => 7,
        [4, 1] => 6,
        [3, 2] => 5,
        _ if straight => 4,
        [3, 1, 1] => 3,
        [2, 2, 1] => 2,
        [2, 1, 1, 1] => 1,
        _ => 0,
    };
    let mut tb = 0u32;
    for g in &groups {
        tb = tb * 7 + g.1 as u32;
    }
    (cat, cat as u32 * 1_000_000 + tb)
}

/// Valoare euristică 0..~800 a unei mâini (pentru AI).
fn value(d: &[u8]) -> i32 {
    let (cat, _) = strength(d);
    let hi = *d.iter().max().unwrap_or(&0) as i32;
    let sum: i32 = d.iter().map(|&x| x as i32).sum();
    cat as i32 * 100 + hi * 3 + sum / 3
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Roll,
    Keep(Vec<u8>),
    Stop,
}

#[derive(Clone)]
pub struct ZarPoker {
    round: u8,
    pts: [u8; 2],
    first: Player,
    side: Player,
    kept: Vec<u8>,
    dice: Vec<u8>,
    rolls_used: u8,
    /// Mâinile terminate în runda curentă.
    done: [Option<Vec<u8>>; 2],
    rng: Rng,
    log: String,
    viewer: Viewer,
    /// Runda tocmai s-a încheiat în ultima mutare (evaluarea nu are voie să-i vadă rezultatul).
    just_resolved: bool,
    last_gain: [u8; 2],
    shown: [Vec<u8>; 2],
    handover: bool,
}

impl Default for ZarPoker {
    fn default() -> Self {
        Self::new()
    }
}

fn subsets(dice: &[u8]) -> Vec<Vec<u8>> {
    let mut out: Vec<Vec<u8>> = Vec::new();
    for mask in 0u8..31 {
        let mut v: Vec<u8> = dice
            .iter()
            .enumerate()
            .filter(|(i, _)| mask & (1 << i) != 0)
            .map(|(_, &d)| d)
            .collect();
        v.sort_unstable();
        if !out.contains(&v) {
            out.push(v);
        }
    }
    out
}

impl ZarPoker {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        ZarPoker {
            round: 0,
            pts: [0, 0],
            first: Player::One,
            side: Player::One,
            kept: Vec::new(),
            dice: Vec::new(),
            rolls_used: 0,
            done: [None, None],
            rng: Rng::new(seed),
            log: String::new(),
            viewer: Viewer::Fixed(Player::One),
            just_resolved: false,
            last_gain: [0, 0],
            shown: [Vec::new(), Vec::new()],
            handover: false,
        }
    }

    pub fn points(&self) -> (u8, u8) {
        (self.pts[0], self.pts[1])
    }

    fn rolling(&self) -> bool {
        self.dice.is_empty()
    }

    /// Speranța valorii după o aruncare a zarurilor rămase (calcul exact).
    fn hold_value(kept: &[u8]) -> f32 {
        let free = 5 - kept.len();
        let total = 6usize.pow(free as u32);
        let mut d = [0u8; 5];
        d[..kept.len()].copy_from_slice(kept);
        let mut acc = 0i64;
        for code in 0..total {
            let mut x = code;
            for slot in d.iter_mut().skip(kept.len()) {
                *slot = (x % 6) as u8 + 1;
                x /= 6;
            }
            acc += value(&d) as i64;
        }
        acc as f32 / total as f32
    }
}

impl Game for ZarPoker {
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
        if self.rolling() {
            return vec![Move::Roll];
        }
        let mut v = vec![Move::Stop];
        if self.rolls_used < 3 {
            for k in subsets(&self.dice) {
                v.push(Move::Keep(k));
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        self.just_resolved = false;
        self.handover = false;
        let i = self.side.idx();
        match m {
            Move::Roll => {
                let mut d = self.kept.clone();
                for _ in 0..(5 - self.kept.len()) {
                    d.push(self.rng.d6());
                }
                d.sort_unstable();
                self.rolls_used += 1;
                self.dice = d;
                self.kept.clear();
            }
            Move::Keep(k) => {
                self.kept = k.clone();
                self.dice.clear();
            }
            Move::Stop => {
                let hand = std::mem::take(&mut self.dice);
                self.kept.clear();
                self.rolls_used = 0;
                self.done[i] = Some(hand);
                if let (Some(a), Some(b)) = (&self.done[0], &self.done[1]) {
                    let (ca, sa) = strength(a);
                    let (cb, sb) = strength(b);
                    self.last_gain = [0, 0];
                    let res = match sa.cmp(&sb) {
                        std::cmp::Ordering::Greater => {
                            self.last_gain[0] = 1;
                            "J1 ia punctul"
                        }
                        std::cmp::Ordering::Less => {
                            self.last_gain[1] = 1;
                            "J2 ia punctul"
                        }
                        std::cmp::Ordering::Equal => "remiză, fără punct",
                    };
                    self.pts[0] += self.last_gain[0];
                    self.pts[1] += self.last_gain[1];
                    self.log = format!(
                        "Runda {}: J1 {:?} ({}) contra J2 {:?} ({}) → {}.",
                        self.round + 1,
                        a,
                        HAND_NAMES[ca],
                        b,
                        HAND_NAMES[cb],
                        res
                    );
                    self.shown = [a.clone(), b.clone()];
                    self.just_resolved = true;
                    self.done = [None, None];
                    self.round += 1;
                    self.first = self.first.other();
                    self.side = self.first;
                } else {
                    self.side = self.side.other();
                }
                self.handover = true;
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.round < ROUNDS {
            return None;
        }
        Some(match self.pts[0].cmp(&self.pts[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    /// Doar zarurile lui `p`, punctele rundelor încheiate și (după încheierea rundei) nu
    /// rezultatul ei.
    fn evaluate(&self, p: Player) -> i32 {
        let q = p.other();
        let (mut mp, mut mq) = (self.pts[p.idx()] as i32, self.pts[q.idx()] as i32);
        if self.just_resolved {
            mp -= self.last_gain[p.idx()] as i32;
            mq -= self.last_gain[q.idx()] as i32;
        }
        let mut s = 1000 * (mp - mq);
        let own: Option<i32> = if self.just_resolved {
            Some(value(&self.shown[p.idx()]))
        } else if let Some(h) = &self.done[p.idx()] {
            Some(value(h))
        } else if self.side == p {
            if !self.dice.is_empty() {
                Some(value(&self.dice))
            } else if !self.kept.is_empty() || self.rolls_used > 0 {
                Some(Self::hold_value(&self.kept) as i32)
            } else {
                None
            }
        } else {
            None
        };
        if let Some(v) = own {
            s += v;
        }
        s
    }

    fn render(&self) -> String {
        let mut s = String::new();
        if self.handover && self.outcome().is_none() {
            if let Viewer::HotSeat = self.viewer {
                s.push_str(&handover_banner(self.side));
            }
        }
        s.push_str(&format!(
            "Poker de zaruri — runda {}/{} | puncte: J1 {}, J2 {}\n",
            (self.round + 1).min(ROUNDS),
            ROUNDS,
            self.pts[0],
            self.pts[1]
        ));
        if !self.log.is_empty() {
            s.push_str(self.log.trim());
            s.push('\n');
        }
        if self.outcome().is_some() {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        let v = self.viewer.who(self.side);
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        if let Some(h) = &self.done[v.idx()] {
            s.push_str(&format!(
                "Mâna ta terminată: {:?} ({})\n",
                h,
                HAND_NAMES[strength(h).0]
            ));
        }
        if self.done[v.other().idx()].is_some() {
            s.push_str("Adversarul și-a terminat mâna (acoperită).\n");
        }
        if v == self.side {
            if self.rolling() {
                s.push_str(&format!(
                    "Ținute: {:?}  aruncări folosite: {}/3  (scrie «arunc»)\n",
                    self.kept, self.rolls_used
                ));
            } else {
                s.push_str(&format!(
                    "Zaruri: {:?} ({})  aruncări folosite: {}/3\n",
                    self.dice,
                    HAND_NAMES[strength(&self.dice).0],
                    self.rolls_used
                ));
            }
        } else {
            s.push_str("Adversarul aruncă (zarurile lui nu se văd).\n");
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Roll => "arunc".into(),
            Move::Stop => "fin".into(),
            Move::Keep(k) if k.is_empty() => "tine -".into(),
            Move::Keep(k) => format!(
                "tine {}",
                k.iter()
                    .map(|d| d.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hand_ranking_order() {
        let hands: [&[u8]; 8] = [
            &[1, 2, 4, 5, 6],
            &[1, 1, 3, 4, 6],
            &[2, 2, 5, 5, 6],
            &[3, 3, 3, 4, 6],
            &[1, 2, 3, 4, 5],
            &[2, 2, 5, 5, 5],
            &[4, 4, 4, 4, 1],
            &[6, 6, 6, 6, 6],
        ];
        for (i, h) in hands.iter().enumerate() {
            assert_eq!(strength(h).0, i, "{h:?}");
        }
        for w in hands.windows(2) {
            assert!(strength(w[0]).1 < strength(w[1]).1);
        }
        assert!(strength(&[2, 3, 4, 5, 6]).1 > strength(&[1, 2, 3, 4, 5]).1);
        assert!(strength(&[6, 6, 1, 2, 3]).1 > strength(&[5, 5, 6, 2, 3]).1);
        assert!(strength(&[5, 5, 6, 2, 3]).1 > strength(&[5, 5, 4, 2, 3]).1);
    }

    fn play_random(seed: u64) -> ZarPoker {
        let mut g = ZarPoker::with_seed(seed);
        let mut rng = Rng::new(seed ^ 77);
        let mut n = 0;
        while g.outcome().is_none() {
            let mv = g.legal_moves();
            assert!(!mv.is_empty());
            g.apply(&mv[rng.below(mv.len())]);
            n += 1;
            assert!(n < 5000);
        }
        g
    }

    #[test]
    fn random_games_finish_with_seven_rounds() {
        for seed in 1..40 {
            let g = play_random(seed);
            assert_eq!(g.round, ROUNDS);
            assert!(g.pts[0] + g.pts[1] <= ROUNDS);
        }
    }

    #[test]
    fn keep_limit_and_forced_stop() {
        let mut g = ZarPoker::with_seed(2);
        assert_eq!(g.legal_moves(), vec![Move::Roll]);
        g.apply(&Move::Roll);
        assert!(g.legal_moves().len() > 1);
        for _ in 0..2 {
            g.apply(&Move::Keep(vec![]));
            g.apply(&Move::Roll);
        }
        assert_eq!(g.legal_moves(), vec![Move::Stop]);
    }

    #[test]
    fn first_hand_hidden_from_second_player() {
        let mut g = ZarPoker::with_seed(4);
        g.apply(&Move::Roll);
        let d = g.dice.clone();
        g.apply(&Move::Stop);
        Game::set_viewer(&mut g, Some(Player::Two));
        let r = Game::render(&g);
        assert!(r.contains("acoperită"));
        assert!(!r.contains(&format!("{d:?}")), "{r}");
        // the finisher sees their own hand in hot-seat/own view
        let mut h = g.clone();
        Game::set_viewer(&mut h, Some(Player::One));
        assert!(Game::render(&h).contains(&format!("{d:?}")));
    }

    #[test]
    fn ai_ignores_covered_hand() {
        for seed in 1..60u64 {
            let mut g = ZarPoker::with_seed(seed);
            g.apply(&Move::Roll);
            g.apply(&Move::Stop);
            g.apply(&Move::Roll); // P2 to decide, P1's hand covered
            let mut h = g.clone();
            let mut rng = Rng::new(seed);
            h.done[0] = Some((0..5).map(|_| rng.d6()).collect());
            assert_eq!(
                polytabletop_core::ai::best_move(&g, 3),
                polytabletop_core::ai::best_move(&h, 3),
                "seed {seed}"
            );
            for mv in g.legal_moves() {
                let (mut a, mut b) = (g.clone(), h.clone());
                a.apply(&mv);
                b.apply(&mv);
                assert_eq!(a.evaluate(Player::Two), b.evaluate(Player::Two));
            }
        }
    }

    #[test]
    fn ai_keeps_obvious_pairs() {
        let mut g = ZarPoker::with_seed(1);
        g.dice = vec![2, 5, 6, 6, 6];
        g.rolls_used = 1;
        let best = polytabletop_core::ai::best_move(&g, 1).unwrap();
        assert!(matches!(best, Move::Keep(ref k) if k == &vec![6, 6, 6]) || best == Move::Stop);
    }
}
