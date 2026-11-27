//! Pai Gow (domino chinezesc) — jucătorul contra dealerului, 12 runde.
//!
//! Reguli folosite (din regulile publice ale jocului):
//! * 32 de piese. Fiecare primește 4 piese și le împarte în două mâini de câte 2: **fața**
//!   (mâna mică) și **spatele** (mâna mare). Se compară față cu față și spate cu spate:
//!   câștigi ambele = ai câștigat miza, pierzi ambele = ai pierdut-o, una și una = remiză.
//! * Ierarhia mâinilor de 2 piese: Gee Joon (1-2 + 2-4) > perechi (Teen 6-6, Day 1-1, Yun 4-4,
//!   Gor 1-3, Mooy 5-5, Chong 3-3, Bon 2-2, Foo 5-6, Ping 4-6, Tit 1-6, Look 1-5, Chop-Gow
//!   {3-6,4-5}, Chop-Bot {3-5,2-6}, Chop-Chit {3-4,2-5}, Chop-Ng {2-3,1-4}) > Wong (Teen/Day +
//!   un 9) > Gong (Teen/Day + un 8) > puncte (totalul pipurilor modulo 10, 9 cel mai bun).
//!   La puncte egale decide cea mai mare piesă; la egalitate perfectă (copie) și la 0–0 câștigă
//!   dealerul. Gee Joon luat singur valorează 3 sau 6, oricare e mai bun.
//! * Împărțirile posibile ale celor 4 piese sunt 3; mâna mai tare devine spatele, deci
//!   nu există aranjare „greșită” (foul). Rolul de dealer alternează la fiecare rundă (J2 e
//!   dealer în prima). Fiecare rundă se joacă pe 10 jetoane din 100; după 12 runde (sau
//!   dacă cineva rămâne fără jetoane) câștigă cine are mai multe jetoane.
//! * Informație ascunsă: vezi doar piesele tale; aranjarea ta nu e arătată adversarului până la
//!   `arata`. AI-ul își alege aranjarea doar din propriile piese.

//!
//! Verificat (nov. 2026) cu en.wikipedia.org/wiki/Pai_gow: ordinea perechilor (Gee Joon,
//! Teen, Day, Yun, Gor, Mooy, Chong, Bon, Foo, Ping, Tit, Look, apoi cele patru Chop),
//! Wong (Teen/Day + 9) > Gong (Teen/Day + 8, Teen peste Day), puncte modulo 10, departajare
//! după piesa cea mai mare, Gee Joon singur = 3 sau 6, copia și 0–0 = dealerul; toate
//! corespund codului. Regula „foul” a jocului real (spatele trebuie să fie ≥ fața) este
//! aplicată automat prin alegerea mâinii mai tari ca spate.

use crate::cards::Viewer;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::sync::OnceLock;

pub const ROUNDS: u8 = 12;
pub const START_CHIPS: i32 = 100;
pub const BET: i32 = 10;

const NAMES: [&str; 18] = [
    "",
    "Gee-Joon(1-2)",
    "Chop-Ng",
    "Gee-Joon(2-4)",
    "Chop-Chit",
    "Chop-Bot",
    "Chop-Gow",
    "Look",
    "Tit",
    "Ping",
    "Foo",
    "Bon",
    "Chong",
    "Mooy",
    "Gor",
    "Yun",
    "Day",
    "Teen",
];

/// (pipuri a, pipuri b, rang-piesă 1..=17)
pub fn all_tiles() -> Vec<(u8, u8, u8)> {
    let civil: [(u8, u8, u8); 11] = [
        (6, 6, 17),
        (1, 1, 16),
        (4, 4, 15),
        (1, 3, 14),
        (5, 5, 13),
        (3, 3, 12),
        (2, 2, 11),
        (5, 6, 10),
        (4, 6, 9),
        (1, 6, 8),
        (1, 5, 7),
    ];
    let mut v = Vec::new();
    for t in civil {
        v.push(t);
        v.push(t);
    }
    for t in [
        (3, 6, 6),
        (4, 5, 6),
        (3, 5, 5),
        (2, 6, 5),
        (3, 4, 4),
        (2, 5, 4),
        (2, 3, 2),
        (1, 4, 2),
        (2, 4, 3),
        (1, 2, 1),
    ] {
        v.push(t);
    }
    v
}

/// Cheia unei mâini de 2 piese (mai mare = mai bună) și dacă valorează 0 puncte „obișnuite”.
pub fn hand_key(a: (u8, u8, u8), b: (u8, u8, u8)) -> u32 {
    let is_gj = |t: (u8, u8, u8)| t.2 == 1 || t.2 == 3;
    if is_gj(a) && is_gj(b) && a.2 != b.2 {
        return 100_000;
    }
    if a.2 == b.2 && !is_gj(a) {
        return 90_000 + a.2 as u32;
    }
    let teen_day = |t: (u8, u8, u8)| t.2 == 17 || t.2 == 16;
    let nine = |t: (u8, u8, u8)| t.0 + t.1 == 9 && t.2 == 6;
    let eight = |t: (u8, u8, u8)| t.0 + t.1 == 8;
    for (x, y) in [(a, b), (b, a)] {
        if teen_day(x) {
            let bonus = if x.2 == 17 { 2 } else { 1 };
            if nine(y) {
                return 80_000 + bonus;
            }
            if eight(y) {
                return 70_000 + bonus;
            }
        }
    }
    let pip = |t: (u8, u8, u8)| (t.0 + t.1) as u32;
    let mut best = 0u32;
    let opts = |t: (u8, u8, u8)| -> Vec<u32> {
        if is_gj(t) {
            vec![3, 6]
        } else {
            vec![pip(t)]
        }
    };
    for x in opts(a) {
        for y in opts(b) {
            best = best.max((x + y) % 10);
        }
    }
    1000 + best * 100 + a.2.max(b.2) as u32
}

fn points(key: u32) -> Option<u32> {
    if (1000..2000).contains(&key) {
        Some((key - 1000) / 100)
    } else {
        None
    }
}

/// Compară două mâini; `true` dacă prima (jucătorul) bate strict a doua (dealerul).
pub fn beats(p: u32, d: u32) -> bool {
    if points(p) == Some(0) && points(d) == Some(0) {
        return false;
    }
    p > d
}

fn pct_table() -> &'static Vec<u32> {
    static T: OnceLock<Vec<u32>> = OnceLock::new();
    T.get_or_init(|| {
        let t = all_tiles();
        let mut v = Vec::new();
        for i in 0..32 {
            for j in (i + 1)..32 {
                v.push(hand_key(t[i], t[j]));
            }
        }
        v.sort_unstable();
        v
    })
}

fn pct(key: u32) -> f64 {
    let t = pct_table();
    let lo = t.partition_point(|&x| x < key);
    let hi = t.partition_point(|&x| x <= key);
    (lo as f64 + (hi - lo) as f64 / 2.0) / t.len() as f64
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    /// împărțirea 0, 1 sau 2
    Split(u8),
    Reveal,
    Next,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Arrange(Player),
    Reveal,
    Over,
}

#[derive(Clone)]
pub struct PaiGow {
    tiles: Vec<(u8, u8, u8)>,
    deck: Vec<u8>,
    hands: [[u8; 4]; 2],
    chosen: [Option<u8>; 2],
    chips: [i32; 2],
    round: u8,
    phase: Phase,
    last: String,
    rng: Rng,
    viewer: Viewer,
}

impl Default for PaiGow {
    fn default() -> Self {
        Self::new()
    }
}

fn split_idx(k: u8) -> ([usize; 2], [usize; 2]) {
    match k {
        0 => ([0, 1], [2, 3]),
        1 => ([0, 2], [1, 3]),
        _ => ([0, 3], [1, 2]),
    }
}

impl PaiGow {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut g = PaiGow {
            tiles: all_tiles(),
            deck: Vec::new(),
            hands: [[0; 4]; 2],
            chosen: [None; 2],
            chips: [START_CHIPS; 2],
            round: 0,
            phase: Phase::Arrange(Player::One),
            last: String::new(),
            rng: Rng::new(seed),
            viewer: Viewer::Fixed(Player::One),
        };
        g.deal();
        g
    }

    fn deal(&mut self) {
        let mut d: Vec<u8> = (0..32).collect();
        crate::cards52::shuffle(&mut d, &mut self.rng);
        for p in 0..2 {
            for i in 0..4 {
                self.hands[p][i] = d.pop().unwrap();
            }
            // ordonate după rang, descrescător, pentru afișare stabilă
            let t = &self.tiles;
            self.hands[p].sort_by_key(|&x| std::cmp::Reverse((t[x as usize].2, x)));
        }
        self.deck = d;
        self.chosen = [None; 2];
        self.phase = Phase::Arrange(Player::One);
    }

    /// (față, spate) — cheile celor două mâini pentru împărțirea `k` a lui `p`.
    fn keys(&self, p: usize, k: u8) -> (u32, u32) {
        let (a, b) = split_idx(k);
        let h = &self.hands[p];
        let t = &self.tiles;
        let ka = hand_key(t[h[a[0]] as usize], t[h[a[1]] as usize]);
        let kb = hand_key(t[h[b[0]] as usize], t[h[b[1]] as usize]);
        (ka.min(kb), ka.max(kb))
    }

    fn dealer(&self) -> Player {
        if self.round.is_multiple_of(2) {
            Player::Two
        } else {
            Player::One
        }
    }

    fn tile_text(&self, id: u8) -> String {
        let t = self.tiles[id as usize];
        format!("{}-{}", t.0, t.1)
    }

    fn hand_text(&self, p: usize) -> String {
        self.hands[p]
            .iter()
            .map(|&x| {
                format!(
                    "{}[{}]",
                    self.tile_text(x),
                    NAMES[self.tiles[x as usize].2 as usize]
                )
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn split_text(&self, p: usize, k: u8) -> String {
        let (a, b) = split_idx(k);
        let h = &self.hands[p];
        let (fa, fb) = (
            hand_key(self.tiles[h[a[0]] as usize], self.tiles[h[a[1]] as usize]),
            hand_key(self.tiles[h[b[0]] as usize], self.tiles[h[b[1]] as usize]),
        );
        let (front, rear) = if fa <= fb { (a, b) } else { (b, a) };
        format!(
            "față {}+{} | spate {}+{}",
            self.tile_text(h[front[0]]),
            self.tile_text(h[front[1]]),
            self.tile_text(h[rear[0]]),
            self.tile_text(h[rear[1]])
        )
    }

    pub fn chips(&self) -> (i32, i32) {
        (self.chips[0], self.chips[1])
    }
}

impl Game for PaiGow {
    type Move = Move;

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = match who {
            Some(p) => Viewer::Fixed(p),
            None => Viewer::HotSeat,
        };
    }

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Arrange(p) => p,
            _ => Player::One,
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        match self.phase {
            Phase::Arrange(_) => (0..3).map(Move::Split).collect(),
            Phase::Reveal => vec![Move::Reveal],
            Phase::Over => vec![Move::Next],
        }
    }

    fn apply(&mut self, m: &Move) {
        match (*m, self.phase) {
            (Move::Split(k), Phase::Arrange(p)) => {
                self.chosen[p.idx()] = Some(k);
                self.phase = match p {
                    Player::One => Phase::Arrange(Player::Two),
                    Player::Two => Phase::Reveal,
                };
            }
            (Move::Reveal, _) => {
                let d = self.dealer().idx();
                let p = 1 - d;
                let kp = self.keys(p, self.chosen[p].unwrap());
                let kd = self.keys(d, self.chosen[d].unwrap());
                let wf = beats(kp.0, kd.0);
                let wb = beats(kp.1, kd.1);
                let delta = match (wf, wb) {
                    (true, true) => BET,
                    (false, false) => -BET,
                    _ => 0,
                };
                self.chips[p] += delta;
                self.chips[d] -= delta;
                self.last = format!(
                    "Runda {}: J1 {} | J2 {} → {}",
                    self.round + 1,
                    self.split_text(0, self.chosen[0].unwrap()),
                    self.split_text(1, self.chosen[1].unwrap()),
                    match delta.cmp(&0) {
                        std::cmp::Ordering::Greater => format!("câștigă J{} (+{})", p + 1, BET),
                        std::cmp::Ordering::Less => format!("câștigă J{} (+{})", d + 1, BET),
                        _ => "remiză (față și spate împărțite)".to_string(),
                    }
                );
                self.phase = Phase::Over;
            }
            (Move::Next, _) => {
                self.round += 1;
                self.deal();
            }
            _ => {}
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        let finished = self.phase == Phase::Over
            && (self.round + 1 >= ROUNDS || self.chips.iter().any(|&c| c < BET));
        if !finished {
            return None;
        }
        Some(match self.chips[0].cmp(&self.chips[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    /// Doar propriile piese (percentilele față/spate ale împărțirii alese) și jetoanele.
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let base = 1000 * (self.chips[me] - self.chips[1 - me]);
        match self.chosen[me] {
            Some(k) if self.phase != Phase::Over => {
                let (f, r) = self.keys(me, k);
                let (pf, pr) = (pct(f), pct(r));
                base + (400.0 * (pf + pr) + 300.0 * pf * pr) as i32
            }
            _ => base,
        }
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Pai Gow — runda {}/{} | jetoane: J1 {}, J2 {} | dealer: J{}\n",
            (self.round + 1).min(ROUNDS),
            ROUNDS,
            self.chips[0],
            self.chips[1],
            self.dealer().idx() + 1
        );
        if !self.last.is_empty() {
            s.push_str(&format!("{}\n", self.last));
        }
        if self.outcome().is_some() {
            s.push_str("Meci încheiat.\n");
            return s;
        }
        if self.phase != Phase::Over {
            let v = self.viewer.who(self.to_move());
            let vi = v.idx();
            s.push_str(&format!(
                "Piesele tale (J{}): {}\n",
                vi + 1,
                self.hand_text(vi)
            ));
            if self.phase == Phase::Arrange(v) {
                for k in 0..3u8 {
                    s.push_str(&format!("  aranj {}: {}\n", k + 1, self.split_text(vi, k)));
                }
            } else if self.chosen[vi].is_some() {
                s.push_str(&format!(
                    "Ai aranjat; J{} aranjează în ascuns.\n",
                    v.other().idx() + 1
                ));
            }
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Split(k) => format!("aranj {}", k + 1),
            Move::Reveal => "arata".into(),
            Move::Next => "urmatoarea".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tile(a: u8, b: u8, nth: usize) -> (u8, u8, u8) {
        let v: Vec<_> = all_tiles()
            .into_iter()
            .filter(|t| (t.0, t.1) == (a, b))
            .collect();
        v[nth.min(v.len() - 1)]
    }

    #[test]
    fn tile_set_is_32_with_21_distinct() {
        let t = all_tiles();
        assert_eq!(t.len(), 32);
        let mut d: Vec<(u8, u8)> = t.iter().map(|x| (x.0, x.1)).collect();
        d.sort_unstable();
        d.dedup();
        assert_eq!(d.len(), 21);
    }

    #[test]
    fn hand_ordering() {
        let gj = hand_key(tile(1, 2, 0), tile(2, 4, 0));
        let teen = hand_key(tile(6, 6, 0), tile(6, 6, 1));
        let day = hand_key(tile(1, 1, 0), tile(1, 1, 1));
        let chop_ng = hand_key(tile(2, 3, 0), tile(1, 4, 0));
        let wong_teen = hand_key(tile(6, 6, 0), tile(3, 6, 0));
        let wong_day = hand_key(tile(1, 1, 0), tile(4, 5, 0));
        let gong = hand_key(tile(6, 6, 0), tile(4, 4, 0));
        let nine = hand_key(tile(6, 6, 0), tile(1, 6, 0)); // 12+7 = 19 -> 9
        let zero = hand_key(tile(5, 5, 0), tile(2, 4, 0)); // 10 + (3|6) -> 3? (10+6)%10 = 6
        assert!(gj > teen && teen > day && day > chop_ng);
        assert!(chop_ng > wong_teen && wong_teen > wong_day && wong_day > gong);
        assert!(gong > nine && nine > zero);
        assert_eq!(points(nine), Some(9));
        assert_eq!(points(zero), Some(6));
    }

    #[test]
    fn mixed_pair_of_non_matching_military_is_points_not_pair() {
        // 3-6 (9) + 3-5 (8) = 17 -> 7 puncte
        let k = hand_key(tile(3, 6, 0), tile(3, 5, 0));
        assert_eq!(points(k), Some(7));
        // 3-6 + 4-5 = Chop-Gow pereche
        assert!(hand_key(tile(3, 6, 0), tile(4, 5, 0)) >= 90_000);
    }

    #[test]
    fn copies_go_to_dealer() {
        let a = hand_key(tile(3, 6, 0), tile(1, 2, 0));
        assert!(!beats(a, a));
        let z1 = hand_key(tile(5, 5, 0), tile(5, 5, 1));
        assert!(!beats(z1, z1));
        let h = hand_key(tile(2, 6, 0), tile(1, 2, 0)); // 8+3=1, 8+6=4 -> 4
        assert_eq!(points(h), Some(4));
    }

    #[test]
    fn zero_zero_dealer_wins() {
        // 4-6 + 1-5 sunt perechi diferite: 10 + 6 -> 6; construim 0 prin 3-5 (8) + 2-...
        // 5-6 (11) + 1-3? Gor(4): 15 -> 5. Folosim 1-6 (7) + 2-3 (5) = 12 -> 2. Căutăm 0: 4-6 (10) + 2-...
        let mut zero_keys = Vec::new();
        let t = all_tiles();
        for i in 0..32 {
            for j in (i + 1)..32 {
                let k = hand_key(t[i], t[j]);
                if points(k) == Some(0) {
                    zero_keys.push(k);
                }
            }
        }
        assert!(zero_keys.len() >= 2);
        let (a, b) = (zero_keys[0], *zero_keys.last().unwrap());
        assert!(!beats(a, b) && !beats(b, a));
    }

    #[test]
    fn dealing_conserves_tiles_and_round_flow() {
        let g = PaiGow::with_seed(7);
        let mut all: Vec<u8> = g.deck.clone();
        all.extend(g.hands[0]);
        all.extend(g.hands[1]);
        all.sort_unstable();
        assert_eq!(all, (0..32).collect::<Vec<u8>>());
        let mut g = g;
        g.apply(&Move::Split(0));
        assert_eq!(g.to_move(), Player::Two);
        g.apply(&Move::Split(1));
        assert_eq!(g.legal_moves(), vec![Move::Reveal]);
        g.apply(&Move::Reveal);
        assert_eq!(g.chips().0 + g.chips().1, 200);
        assert_eq!(g.legal_moves(), vec![Move::Next]);
    }

    #[test]
    fn render_hides_opponent_tiles() {
        let g = PaiGow::with_seed(9);
        let r = Game::render(&g);
        assert!(r.contains("Piesele tale"));
        let opp = g.hand_text(1);
        assert!(!r.contains(&opp) || opp == g.hand_text(0));
    }

    #[test]
    fn ai_is_blind() {
        for seed in 1..30u64 {
            let mut a = PaiGow::with_seed(seed);
            a.apply(&Move::Split((seed % 3) as u8));
            let mut b = a.clone();
            let mut rng = Rng::new(seed);
            let mut pool: Vec<u8> = b.deck.clone();
            pool.extend(b.hands[0]);
            crate::cards52::shuffle(&mut pool, &mut rng);
            for i in 0..4 {
                b.hands[0][i] = pool.pop().unwrap();
            }
            b.deck = pool;
            assert_eq!(
                polytabletop_core::ai::best_move(&a, 3),
                polytabletop_core::ai::best_move(&b, 3)
            );
        }
    }

    #[test]
    fn random_matches_end_and_chips_zero_sum() {
        for seed in 1..40u64 {
            let mut g = PaiGow::with_seed(seed);
            let mut rng = Rng::new(seed ^ 5);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
                assert!(n < 500);
                assert_eq!(g.chips[0] + g.chips[1], 200);
            }
        }
    }
}

#[cfg(test)]
mod a1_blind {
    use super::*;
    use crate::ai_audit_a1::{audit, shuffle};

    #[test]
    fn ai_ignores_opponent_tiles_deck_and_arrangement_at_all_levels() {
        audit("paigow", PaiGow::with_seed, 1..=4, 1, 40, |g, salt| {
            let foe = 1 - g.to_move().idx();
            let mut pool: Vec<u8> = g.hands[foe].to_vec();
            pool.extend(g.deck.iter());
            shuffle(&mut pool, salt);
            g.hands[foe].copy_from_slice(&pool[..4]);
            g.deck = pool[4..].to_vec();
            if g.chosen[foe].is_some() {
                g.chosen[foe] = Some((salt % 3) as u8);
            }
            g.rng = Rng::new(salt ^ 3);
        });
    }
}
