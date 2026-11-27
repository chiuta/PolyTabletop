//! Hanafuda (Koi-Koi) și Hwatu (Go-Stop) — cărți de flori japoneze/coreene, 2 jucători.
//!
//! Același pachet de 48 de cărți (12 luni × 4) și același mecanism de capturare servesc ambele
//! jocuri (vezi `hwatu.rs` pentru Go-Stop). Cărțile se notează lună + literă: `3b` = a doua
//! carte a lunii martie.
//!
//! Pachet: 5 cărți „luminoase” (20 de puncte), 9 „animale” (10), 10 panglici (5: trei cu
//! poem roșii, trei albastre, patru simple) și 24 simple (1).
//!
//! Mecanismul comun: o tură = (1) joci o carte din mână: dacă are pereche pe masă (aceeași
//! lună) **trebuie** să captureze (doi candidați = alegi, trei = le iei pe toate patru), altfel
//! rămâne pe masă; (2) mutarea forțată `intoarce` întoarce prima carte din stoc și se joacă la
//! fel (cu `ia 3c` dacă alegerea e între doi candidați). Stocul întors nu e vizibil AI-ului
//! înainte, deoarece e o mutare separată.
//!
//! **Koi-Koi** (8 cărți în mână, 8 pe masă, 3 runde): yaku-uri — Cinci luminoase 10, Patru
//! luminoase 8, Patru luminoase cu ploaie 7, Trei luminoase (fără omul-ploii) 5,
//! Ino-Shika-Chō (mistreț+cerb+fluturi) 5 (+1 pentru fiecare animal în plus), Tane (5+ animale)
//! 1 (+1 fiecare în plus), Akatan (3 panglici cu poem) 5, Aotan (3 albastre) 5, Tan (5+
//! panglici) 1 (+1), Kasu (10+ simple; cupa de sake se numără și ca simplă) 1 (+1), Hanami
//! (perdea + cupă) 5, Tsukimi (lună + cupă) 5. La un yaku nou (scor mai mare decât ultima
//! dată) alegi `koi-koi` (continui) sau `shobu` (închizi runda și iei punctele); scorul se
//! dublează la 7+ puncte și încă o dată dacă adversarul a spus koi-koi. Dacă mâinile se
//! golesc fără yaku, runda e remiză. După 3 runde câștigă cine are mai multe puncte.
//! Simplificări: yaku-urile se verifică la sfârșitul turei (nu și după cartea din mână); fără
//! mâini speciale la împărțire; AI-ul spune întotdeauna `shobu`.
//!
//! **Go-Stop**: vezi `hwatu.rs`.

use crate::cards::Viewer;
use crate::cards52::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const ROUNDS: u8 = 3;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Variant {
    KoiKoi,
    GoStop,
}

const MONTH_KINDS: [&str; 12] = [
    "BRcc", "ARcc", "BRcc", "ARcc", "ARcc", "ARcc", "ARcc", "BAcc", "ARcc", "ARcc", "BARc", "Bccc",
];
const FLOWERS: [&str; 12] = [
    "Pin",
    "Prun",
    "Cireș",
    "Glicină",
    "Iris",
    "Bujor",
    "Trifoi",
    "Pampas",
    "Crizantemă",
    "Arțar",
    "Salcie",
    "Paulownia",
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Class {
    Bright,
    Animal,
    Ribbon,
    Chaff,
}

pub fn month(id: u8) -> u8 {
    id / 4
}
pub fn class(id: u8) -> Class {
    match MONTH_KINDS[month(id) as usize].as_bytes()[(id % 4) as usize] {
        b'B' => Class::Bright,
        b'A' => Class::Animal,
        b'R' => Class::Ribbon,
        _ => Class::Chaff,
    }
}
pub const RAIN: u8 = 40;
pub const MOON: u8 = 28;
pub const CURTAIN: u8 = 8;
pub const CUP: u8 = 32;
pub const BOAR: u8 = 24;
pub const DEER: u8 = 36;
pub const BUTTERFLY: u8 = 20;
pub const WARBLER: u8 = 4;
pub const CUCKOO: u8 = 12;
pub const GEESE: u8 = 29;

pub fn is_red_poem(id: u8) -> bool {
    class(id) == Class::Ribbon && month(id) <= 2
}
pub fn is_blue(id: u8) -> bool {
    class(id) == Class::Ribbon && matches!(month(id), 5 | 8 | 9)
}
pub fn is_plain_red(id: u8) -> bool {
    class(id) == Class::Ribbon && matches!(month(id), 3 | 4 | 6)
}
/// Cărți simple care valorează dublu la Go-Stop (ssangpi).
pub fn is_double_pi(id: u8) -> bool {
    id == 43 || id == 47
}

pub fn lab(id: u8) -> String {
    format!("{}{}", month(id) + 1, (b'a' + id % 4) as char)
}

pub fn parse_lab(s: &str) -> Option<u8> {
    let s = s.trim().to_lowercase();
    let (num, l) = s.split_at(s.len().checked_sub(1)?);
    let m: u8 = num.parse().ok()?;
    let k = l.bytes().next()?.checked_sub(b'a')?;
    if (1..=12).contains(&m) && k < 4 {
        Some((m - 1) * 4 + k)
    } else {
        None
    }
}

pub fn desc(id: u8) -> String {
    let special = match id {
        0 => "cocor",
        CURTAIN => "perdea",
        MOON => "lună",
        RAIN => "omul-ploii",
        44 => "fenix",
        WARBLER => "privighetoare",
        CUCKOO => "cuc",
        16 => "pod",
        BUTTERFLY => "fluturi",
        BOAR => "mistreț",
        GEESE => "gâște",
        CUP => "cupă",
        DEER => "cerb",
        41 => "rândunică",
        _ => match class(id) {
            Class::Ribbon => {
                if is_red_poem(id) {
                    "panglică-poem"
                } else if is_blue(id) {
                    "panglică-albastră"
                } else {
                    "panglică"
                }
            }
            _ => "simplă",
        },
    };
    let pts = match class(id) {
        Class::Bright => 20,
        Class::Animal => 10,
        Class::Ribbon => 5,
        Class::Chaff => 1,
    };
    format!(
        "{}[{} {}, {}]",
        lab(id),
        FLOWERS[month(id) as usize],
        special,
        pts
    )
}

fn short(ids: &[u8]) -> String {
    if ids.is_empty() {
        return "—".into();
    }
    ids.iter().map(|&c| desc(c)).collect::<Vec<_>>().join(" ")
}

fn value(id: u8) -> i32 {
    match class(id) {
        Class::Bright => 80,
        Class::Animal => {
            if matches!(id, BOAR | DEER | BUTTERFLY | CUP) {
                50
            } else {
                40
            }
        }
        Class::Ribbon => {
            if is_red_poem(id) || is_blue(id) {
                30
            } else {
                20
            }
        }
        Class::Chaff => {
            if is_double_pi(id) {
                10
            } else {
                5
            }
        }
    }
}

/// Punctele de yaku Koi-Koi.
pub fn score_koikoi(cap: &[u8]) -> i32 {
    let has = |x: u8| cap.contains(&x);
    let n = |c: Class| cap.iter().filter(|&&x| class(x) == c).count() as i32;
    let mut s = 0;
    let brights = n(Class::Bright);
    s += match (brights, has(RAIN)) {
        (5, _) => 10,
        (4, false) => 8,
        (4, true) => 7,
        (3, false) => 5,
        _ => 0,
    };
    let animals = n(Class::Animal);
    if has(BOAR) && has(DEER) && has(BUTTERFLY) {
        s += 5 + (animals - 3);
    } else if animals >= 5 {
        s += 1 + (animals - 5);
    }
    if cap.iter().filter(|&&x| is_red_poem(x)).count() == 3 {
        s += 5;
    }
    if cap.iter().filter(|&&x| is_blue(x)).count() == 3 {
        s += 5;
    }
    let ribbons = n(Class::Ribbon);
    if ribbons >= 5 {
        s += 1 + (ribbons - 5);
    }
    let kasu = n(Class::Chaff) + i32::from(has(CUP));
    if kasu >= 10 {
        s += 1 + (kasu - 10);
    }
    if has(CURTAIN) && has(CUP) {
        s += 5;
    }
    if has(MOON) && has(CUP) {
        s += 5;
    }
    s
}

/// Punctele de bază la Go-Stop.
pub fn score_gostop(cap: &[u8]) -> i32 {
    let has = |x: u8| cap.contains(&x);
    let n = |c: Class| cap.iter().filter(|&&x| class(x) == c).count() as i32;
    let mut s = 0;
    let b = n(Class::Bright);
    s += match b {
        5 => 15,
        4 => 4,
        3 => {
            if has(RAIN) {
                2
            } else {
                3
            }
        }
        _ => 0,
    };
    if has(WARBLER) && has(CUCKOO) && has(GEESE) {
        s += 5;
    }
    let animals = n(Class::Animal);
    if animals >= 5 {
        s += 1 + animals - 5;
    }
    if cap.iter().filter(|&&x| is_red_poem(x)).count() == 3 {
        s += 3;
    }
    if cap.iter().filter(|&&x| is_blue(x)).count() == 3 {
        s += 3;
    }
    if cap.iter().filter(|&&x| is_plain_red(x)).count() == 3 {
        s += 3;
    }
    let ribbons = n(Class::Ribbon);
    if ribbons >= 5 {
        s += 1 + ribbons - 5;
    }
    let pi = pi_count(cap);
    if pi >= 10 {
        s += 1 + pi - 10;
    }
    s
}

pub fn pi_count(cap: &[u8]) -> i32 {
    cap.iter()
        .filter(|&&x| class(x) == Class::Chaff)
        .map(|&x| if is_double_pi(x) { 2 } else { 1 })
        .sum()
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Hand,
    Flip,
    FlipPick,
    Decide,
    RoundOver,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Play(u8, Option<u8>),
    Flip,
    Pick(u8),
    Continue,
    Stop,
    Next,
}

#[derive(Clone)]
pub struct Hanafuda {
    variant: Variant,
    deck: Vec<u8>,
    hands: [Vec<u8>; 2],
    table: Vec<u8>,
    cap: [Vec<u8>; 2],
    side: Player,
    phase: Phase,
    round: u8,
    total: [i32; 2],
    called: [u8; 2],
    last_score: [i32; 2],
    drawn: Option<u8>,
    last: String,
    rng: Rng,
    viewer: Viewer,
}

impl Default for Hanafuda {
    fn default() -> Self {
        Self::new()
    }
}

impl Hanafuda {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        Self::build(Variant::KoiKoi, seed)
    }
    pub fn go_stop() -> Self {
        Self::build(Variant::GoStop, Rng::from_clock().next_u64())
    }
    pub fn go_stop_with_seed(seed: u64) -> Self {
        Self::build(Variant::GoStop, seed)
    }

    fn build(variant: Variant, seed: u64) -> Self {
        let mut g = Hanafuda {
            variant,
            deck: Vec::new(),
            hands: [Vec::new(), Vec::new()],
            table: Vec::new(),
            cap: [Vec::new(), Vec::new()],
            side: Player::One,
            phase: Phase::Hand,
            round: 0,
            total: [0; 2],
            called: [0; 2],
            last_score: [0; 2],
            drawn: None,
            last: String::new(),
            rng: Rng::new(seed),
            viewer: Viewer::Fixed(Player::One),
        };
        g.deal();
        g
    }

    pub fn variant(&self) -> Variant {
        self.variant
    }

    fn hand_size(&self) -> usize {
        match self.variant {
            Variant::KoiKoi => 8,
            Variant::GoStop => 10,
        }
    }

    fn deal(&mut self) {
        let mut d: Vec<u8> = (0..48).collect();
        shuffle(&mut d, &mut self.rng);
        let hs = self.hand_size();
        for p in 0..2 {
            self.hands[p] = d.drain(..hs).collect();
            self.hands[p].sort_unstable();
            self.cap[p].clear();
        }
        self.table = d.drain(..8).collect();
        self.table.sort_unstable();
        self.deck = d;
        self.called = [0; 2];
        self.last_score = [0; 2];
        self.drawn = None;
        self.phase = Phase::Hand;
        self.side = if self.round.is_multiple_of(2) {
            Player::One
        } else {
            Player::Two
        };
    }

    pub fn totals(&self) -> (i32, i32) {
        (self.total[0], self.total[1])
    }

    fn base_score(&self, p: usize) -> i32 {
        match self.variant {
            Variant::KoiKoi => score_koikoi(&self.cap[p]),
            Variant::GoStop => score_gostop(&self.cap[p]),
        }
    }

    fn matches_of(&self, c: u8) -> Vec<u8> {
        self.table
            .iter()
            .copied()
            .filter(|&t| month(t) == month(c))
            .collect()
    }

    /// Capturează `c` împreună cu candidații (toți sau cel ales).
    fn take(&mut self, c: u8, pick: Option<u8>) {
        let me = self.side.idx();
        let ms = self.matches_of(c);
        let taken: Vec<u8> = match pick {
            Some(x) => vec![x],
            None => ms,
        };
        self.table.retain(|t| !taken.contains(t));
        self.cap[me].push(c);
        self.cap[me].extend(taken);
        if self.variant == Variant::GoStop && self.table.is_empty() {
            // „măturare”: iei o simplă de la adversar
            let o = 1 - me;
            if let Some(pos) = self.cap[o]
                .iter()
                .position(|&x| class(x) == Class::Chaff && !is_double_pi(x))
            {
                let x = self.cap[o].remove(pos);
                self.cap[me].push(x);
            }
        }
    }

    fn resolve(&mut self, c: u8) {
        let ms = self.matches_of(c);
        if ms.is_empty() {
            self.table.push(c);
            self.table.sort_unstable();
        } else {
            self.take(c, None);
        }
    }

    fn end_turn(&mut self) {
        let me = self.side.idx();
        let s = self.base_score(me);
        let trigger = match self.variant {
            Variant::KoiKoi => s > self.last_score[me],
            Variant::GoStop => s >= 7 && s > self.last_score[me],
        };
        if trigger {
            if self.hands[me].is_empty() {
                self.settle(self.side);
            } else {
                self.phase = Phase::Decide;
            }
            return;
        }
        self.advance();
    }

    fn advance(&mut self) {
        self.drawn = None;
        let o = self.side.other();
        if self.hands[o.idx()].is_empty() {
            self.last = format!("Runda {}: remiză (mâinile s-au golit)", self.round + 1);
            self.phase = Phase::RoundOver;
        } else {
            self.side = o;
            self.phase = Phase::Hand;
        }
    }

    fn settle(&mut self, p: Player) {
        let me = p.idx();
        let o = 1 - me;
        let s = self.base_score(me);
        let mut pts;
        let mut notes = String::new();
        match self.variant {
            Variant::KoiKoi => {
                pts = s;
                if s >= 7 {
                    pts *= 2;
                    notes.push_str(" ×2 (7+)");
                }
                if self.called[o] > 0 {
                    pts *= 2;
                    notes.push_str(" ×2 (adversarul a spus koi-koi)");
                }
            }
            Variant::GoStop => {
                let go = self.called[me] as i32;
                pts = s + go.min(2);
                if go >= 3 {
                    pts *= 1 << (go - 2);
                    notes.push_str(&format!(" ×{} (go)", 1 << (go - 2)));
                }
                let (gw_me, gw_o) = (
                    self.cap[me]
                        .iter()
                        .filter(|&&x| class(x) == Class::Bright)
                        .count(),
                    self.cap[o]
                        .iter()
                        .filter(|&&x| class(x) == Class::Bright)
                        .count(),
                );
                if gw_me >= 3 && gw_o == 0 {
                    pts *= 2;
                    notes.push_str(" ×2 (gwang-bak)");
                }
                let (pi_me, pi_o) = (pi_count(&self.cap[me]), pi_count(&self.cap[o]));
                if pi_me >= 10 && (1..=5).contains(&pi_o) {
                    pts *= 2;
                    notes.push_str(" ×2 (pi-bak)");
                }
                if self.called[o] > 0 {
                    pts *= 2;
                    notes.push_str(" ×2 (go-bak)");
                }
            }
        }
        self.total[me] += pts;
        self.last = format!(
            "Runda {}: J{} încasează {} puncte (bază {}){}",
            self.round + 1,
            me + 1,
            pts,
            s,
            notes
        );
        self.phase = Phase::RoundOver;
    }
}

impl Game for Hanafuda {
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
        match self.phase {
            Phase::Hand => {
                let mut v = Vec::new();
                for &c in &self.hands[self.side.idx()] {
                    let ms = self.matches_of(c);
                    if ms.len() == 2 {
                        v.push(Move::Play(c, Some(ms[0])));
                        v.push(Move::Play(c, Some(ms[1])));
                    } else {
                        v.push(Move::Play(c, None));
                    }
                }
                v
            }
            Phase::Flip => vec![Move::Flip],
            Phase::FlipPick => {
                let c = self.drawn.unwrap();
                self.matches_of(c).into_iter().map(Move::Pick).collect()
            }
            Phase::Decide => vec![Move::Continue, Move::Stop],
            Phase::RoundOver => vec![Move::Next],
        }
    }

    fn apply(&mut self, m: &Move) {
        match *m {
            Move::Play(c, pick) => {
                let me = self.side.idx();
                self.hands[me].retain(|&x| x != c);
                if self.matches_of(c).is_empty() {
                    self.table.push(c);
                    self.table.sort_unstable();
                } else {
                    self.take(c, pick);
                }
                self.phase = Phase::Flip;
            }
            Move::Flip => {
                let c = self.deck.pop().expect("stoc");
                self.drawn = Some(c);
                if self.matches_of(c).len() == 2 {
                    self.phase = Phase::FlipPick;
                } else {
                    self.resolve(c);
                    self.end_turn();
                }
            }
            Move::Pick(x) => {
                let c = self.drawn.unwrap();
                self.take(c, Some(x));
                self.end_turn();
            }
            Move::Continue => {
                let me = self.side.idx();
                self.called[me] += 1;
                self.last_score[me] = self.base_score(me);
                self.advance();
            }
            Move::Stop => self.settle(self.side),
            Move::Next => {
                self.round += 1;
                self.deal();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::RoundOver || self.round + 1 < ROUNDS {
            return None;
        }
        Some(match self.total[0].cmp(&self.total[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    /// Doar informație vizibilă: capturile ambilor jucători, masa și mâna proprie (ca număr).
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let o = 1 - me;
        let mut s = 1000 * (self.total[me] - self.total[o]);
        if self.phase == Phase::RoundOver {
            return s;
        }
        let cv = |v: &Vec<u8>| v.iter().map(|&x| value(x)).sum::<i32>();
        s += cv(&self.cap[me]) - cv(&self.cap[o]);
        s += 60 * (self.base_score(me) - self.base_score(o));
        let tv: i32 = self.table.iter().map(|&x| value(x)).sum();
        // cărțile rămase pe masă sunt ale celui care mută următorul
        if self.side == p && self.phase != Phase::Hand {
            s -= tv / 6;
        } else if self.side == p {
            s += tv / 8;
        } else {
            s -= tv / 8;
        }
        s
    }

    fn render(&self) -> String {
        let name = match self.variant {
            Variant::KoiKoi => "Hanafuda (Koi-Koi)",
            Variant::GoStop => "Hwatu (Go-Stop)",
        };
        let mut s = format!(
            "{} — runda {}/{} | puncte: J1 {}, J2 {} | stoc {}\n",
            name,
            (self.round + 1).min(ROUNDS),
            ROUNDS,
            self.total[0],
            self.total[1],
            self.deck.len()
        );
        if !self.last.is_empty() {
            s.push_str(&format!("{}\n", self.last));
        }
        if self.outcome().is_some() {
            s.push_str("Meci încheiat.\n");
            return s;
        }
        if self.phase == Phase::RoundOver {
            return s;
        }
        s.push_str(&format!("Masa: {}\n", short(&self.table)));
        for p in 0..2 {
            let c = &self.cap[p];
            let extra = if self.called[p] > 0 {
                format!(
                    " ({} {})",
                    match self.variant {
                        Variant::KoiKoi => "koi-koi",
                        Variant::GoStop => "go",
                    },
                    self.called[p]
                )
            } else {
                String::new()
            };
            s.push_str(&format!(
                "Capturate J{} (scor {}){}: {}\n",
                p + 1,
                self.base_score(p),
                extra,
                short(c)
            ));
        }
        if let Some(d) = self.drawn {
            s.push_str(&format!("Întoarsă din stoc: {}\n", desc(d)));
        }
        let v = self.viewer.who(self.side);
        let o = v.other();
        s.push_str(&format!(
            "Mâna ta (J{}): {}\n",
            v.idx() + 1,
            short(&self.hands[v.idx()])
        ));
        s.push_str(&format!(
            "J{}: {} cărți\n",
            o.idx() + 1,
            self.hands[o.idx()].len()
        ));
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Play(c, None) => lab(*c),
            Move::Play(c, Some(x)) => format!("{}:{}", lab(*c), lab(*x)),
            Move::Flip => "intoarce".into(),
            Move::Pick(x) => format!("ia {}", lab(*x)),
            Move::Continue => match self.variant {
                Variant::KoiKoi => "koi-koi".into(),
                Variant::GoStop => "go".into(),
            },
            Move::Stop => match self.variant {
                Variant::KoiKoi => "shobu".into(),
                Variant::GoStop => "stop".into(),
            },
            Move::Next => "urmatoarea".into(),
        }
    }

    /// Adâncime 0 (o singură mutare): `AlphaBeta(1)` ar fi extins la Hard/Expert mutarea forțată `intoarce`
    /// (vârful real al stocului) și răspunsul adversarului; evaluarea citește doar datele publice.
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deck_composition() {
        let c = |k| (0..48u8).filter(|&i| class(i) == k).count();
        assert_eq!(c(Class::Bright), 5);
        assert_eq!(c(Class::Animal), 9);
        assert_eq!(c(Class::Ribbon), 10);
        assert_eq!(c(Class::Chaff), 24);
        assert_eq!((0..48u8).filter(|&i| is_red_poem(i)).count(), 3);
        assert_eq!((0..48u8).filter(|&i| is_blue(i)).count(), 3);
        assert_eq!((0..48u8).filter(|&i| is_plain_red(i)).count(), 3);
        assert_eq!(class(RAIN), Class::Bright);
        assert_eq!(class(MOON), Class::Bright);
        assert_eq!(class(CURTAIN), Class::Bright);
        assert_eq!(class(CUP), Class::Animal);
        assert_eq!(class(BOAR), Class::Animal);
        assert_eq!(class(GEESE), Class::Animal);
        assert_eq!(class(WARBLER), Class::Animal);
        assert_eq!(class(CUCKOO), Class::Animal);
        assert_eq!(class(DEER), Class::Animal);
        assert_eq!(class(BUTTERFLY), Class::Animal);
        assert_eq!(class(43), Class::Chaff);
        assert_eq!(class(47), Class::Chaff);
        // doar 2 ssangpi, 26 pi în total
        assert_eq!(pi_count(&(0..48u8).collect::<Vec<_>>()), 26);
    }

    #[test]
    fn labels_roundtrip() {
        for i in 0..48u8 {
            assert_eq!(parse_lab(&lab(i)), Some(i));
        }
        assert_eq!(lab(0), "1a");
        assert_eq!(lab(47), "12d");
        assert_eq!(parse_lab("13a"), None);
    }

    #[test]
    fn koikoi_yaku_scores() {
        assert_eq!(score_koikoi(&[0, 8, 28, 44, 40]), 10);
        assert_eq!(score_koikoi(&[0, 8, 28, 44]), 8);
        assert_eq!(score_koikoi(&[0, 8, 28, 40]), 7);
        assert_eq!(score_koikoi(&[0, 8, 28]), 5);
        assert_eq!(score_koikoi(&[BOAR, DEER, BUTTERFLY]), 5);
        assert_eq!(score_koikoi(&[BOAR, DEER, BUTTERFLY, WARBLER]), 6);
        assert_eq!(score_koikoi(&[1, 5, 9]), 5); // akatan
        assert_eq!(score_koikoi(&[21, 33, 37]), 5); // aotan
        assert_eq!(score_koikoi(&[CURTAIN, CUP]), 5); // hanami
        assert_eq!(score_koikoi(&[MOON, CUP]), 5); // tsukimi
        assert_eq!(score_koikoi(&[CURTAIN, MOON, CUP]), 10); // ambele
    }

    #[test]
    fn kasu_and_tan_thresholds() {
        let chaff: Vec<u8> = (0..48u8).filter(|&i| class(i) == Class::Chaff).collect();
        assert_eq!(score_koikoi(&chaff[..9]), 0);
        assert_eq!(score_koikoi(&chaff[..10]), 1);
        assert_eq!(score_koikoi(&chaff[..12]), 3);
        let rib: Vec<u8> = (0..48u8).filter(|&i| class(i) == Class::Ribbon).collect();
        assert_eq!(rib.len(), 10);
        // 5 panglici fără trio de poem/albastre: tan = 1
        assert_eq!(score_koikoi(&[13, 17, 25, 42, 1]), 1);
        assert_eq!(score_koikoi(&[13, 17, 25, 42]), 0);
    }

    #[test]
    fn gostop_scores() {
        assert_eq!(score_gostop(&[0, 8, 28]), 3);
        assert_eq!(score_gostop(&[0, 8, RAIN]), 2);
        assert_eq!(score_gostop(&[0, 8, 28, 44, RAIN]), 15);
        assert_eq!(score_gostop(&[WARBLER, CUCKOO, GEESE]), 5);
        assert_eq!(score_gostop(&[1, 5, 9]), 3);
        assert_eq!(score_gostop(&[13, 17, 25]), 3); // chodan (apr, mai, iul)
    }

    #[test]
    fn deal_conserves_cards() {
        for variant_go in [false, true] {
            let g = if variant_go {
                Hanafuda::go_stop_with_seed(3)
            } else {
                Hanafuda::with_seed(3)
            };
            let mut all: Vec<u8> = g.deck.clone();
            all.extend(&g.hands[0]);
            all.extend(&g.hands[1]);
            all.extend(&g.table);
            all.sort_unstable();
            assert_eq!(all, (0..48).collect::<Vec<u8>>());
            assert_eq!(g.hands[0].len(), if variant_go { 10 } else { 8 });
            assert_eq!(g.table.len(), 8);
        }
    }

    #[test]
    fn capture_rules() {
        let mut g = Hanafuda::with_seed(1);
        g.table = vec![0, 1, 8]; // 1a, 1b (ianuarie), 3a
        g.hands[0] = vec![2, 9, 20]; // 1c, 3b, 6a
        g.hands[1] = vec![30];
        let mv = g.legal_moves();
        // 1c are doi candidați -> două mutări
        assert!(mv.contains(&Move::Play(2, Some(0))));
        assert!(mv.contains(&Move::Play(2, Some(1))));
        assert!(mv.contains(&Move::Play(9, None)));
        assert!(mv.contains(&Move::Play(20, None)));
        // cu o potrivire e obligatorie capturarea (nu apare ca aruncare): 3b ia 3a
        g.apply(&Move::Play(9, None));
        assert!(g.cap[0].contains(&8) && g.cap[0].contains(&9));
        assert!(!g.table.contains(&8));
        assert_eq!(g.legal_moves(), vec![Move::Flip]);
    }

    #[test]
    fn three_on_table_take_all_four() {
        let mut g = Hanafuda::with_seed(1);
        g.table = vec![0, 1, 2, 20];
        g.hands[0] = vec![3];
        g.apply(&Move::Play(3, None));
        assert_eq!(g.cap[0].len(), 4);
        assert_eq!(g.table, vec![20]);
    }

    #[test]
    fn flip_pick_and_discard() {
        let mut g = Hanafuda::with_seed(1);
        g.table = vec![0, 1, 20];
        g.hands[0] = vec![30, 31];
        g.deck = vec![40, 2]; // se întoarce 2 (1c)
        g.apply(&Move::Play(30, None)); // aruncare 8c? (lună 8: nu are pereche)
        g.apply(&Move::Flip);
        assert_eq!(g.phase, Phase::FlipPick);
        assert_eq!(g.legal_moves().len(), 2);
        g.apply(&Move::Pick(1));
        assert!(g.cap[0].contains(&2) && g.cap[0].contains(&1));
    }

    #[test]
    fn koikoi_decision_and_shobu_scoring() {
        let mut g = Hanafuda::with_seed(1);
        g.cap[0] = vec![0, 8, 28, 44]; // 4 luminoase fără ploaie = 8
        g.table = vec![];
        g.hands[0] = vec![47, 46];
        g.hands[1] = vec![30];
        g.deck = vec![36];
        g.phase = Phase::Hand;
        g.apply(&Move::Play(46, None)); // aruncă (Dec c) ; nu e pereche (44 e capturat, 47 în mână)
        g.apply(&Move::Flip);
        assert_eq!(g.phase, Phase::Decide);
        let m = g.legal_moves();
        assert_eq!(m.len(), 2);
        g.apply(&Move::Stop);
        assert_eq!(g.totals().0, 16); // 8 ×2 (7+)
        assert_eq!(g.legal_moves(), vec![Move::Next]);
    }

    #[test]
    fn opponent_koikoi_doubles() {
        let mut g = Hanafuda::with_seed(1);
        g.called = [0, 1];
        g.cap[0] = vec![BOAR, DEER, BUTTERFLY];
        g.settle(Player::One);
        assert_eq!(g.totals().0, 10);
    }

    #[test]
    fn gostop_go_and_bak() {
        let mut g = Hanafuda::go_stop_with_seed(1);
        g.cap[0] = vec![0, 8, 28, 44, RAIN]; // 15 + gwang-bak (adversar 0) ×2
        g.called = [1, 0];
        g.settle(Player::One);
        assert_eq!(g.totals().0, (15 + 1) * 2);
    }

    #[test]
    fn sweep_takes_a_pi_in_gostop_only() {
        let mut g = Hanafuda::go_stop_with_seed(1);
        g.table = vec![0];
        g.cap[1] = vec![2, 3];
        g.hands[0] = vec![1];
        g.apply(&Move::Play(1, None));
        assert!(g.table.is_empty());
        assert_eq!(g.cap[0].len(), 3);
        assert_eq!(g.cap[1].len(), 1);
        let mut k = Hanafuda::with_seed(1);
        k.table = vec![0];
        k.cap[1] = vec![2, 3];
        k.hands[0] = vec![1];
        k.apply(&Move::Play(1, None));
        assert_eq!(k.cap[1].len(), 2);
    }

    #[test]
    fn render_hides_opponent_hand() {
        let g = Hanafuda::with_seed(11);
        let r = Game::render(&g);
        assert!(r.contains("J2: 8 cărți"));
        let mine = short(&g.hands[0]);
        assert!(r.contains(&mine));
        for &c in &g.hands[1] {
            // eticheta cărții adversarului nu apare în secțiunea „Mâna ta” sau pe masă
            assert!(!r.contains(&desc(c)));
        }
    }

    #[test]
    fn ai_blind_to_hidden() {
        for variant_go in [false, true] {
            for seed in 1..25u64 {
                let mut a = if variant_go {
                    Hanafuda::go_stop_with_seed(seed)
                } else {
                    Hanafuda::with_seed(seed)
                };
                let mut rng = Rng::new(seed);
                for _ in 0..(seed % 9) {
                    if a.outcome().is_some() {
                        break;
                    }
                    let mv = a.legal_moves();
                    a.apply(&mv[rng.below(mv.len())]);
                }
                if a.phase != Phase::Hand {
                    continue;
                }
                let me = a.side.idx();
                let mut b = a.clone();
                let mut pool = b.deck.clone();
                pool.extend(&b.hands[1 - me]);
                shuffle(&mut pool, &mut rng);
                let n = b.hands[1 - me].len();
                b.hands[1 - me] = pool.drain(..n).collect();
                b.deck = pool;
                assert_eq!(
                    polytabletop_core::ai::best_move(&a, 5),
                    polytabletop_core::ai::best_move(&b, 5)
                );
            }
        }
    }

    #[test]
    fn random_matches_end_and_cards_conserved() {
        for variant_go in [false, true] {
            for seed in 1..40u64 {
                let mut g = if variant_go {
                    Hanafuda::go_stop_with_seed(seed)
                } else {
                    Hanafuda::with_seed(seed)
                };
                let mut rng = Rng::new(seed * 7);
                let mut n = 0;
                while g.outcome().is_none() {
                    let mv = g.legal_moves();
                    assert!(!mv.is_empty());
                    g.apply(&mv[rng.below(mv.len())]);
                    n += 1;
                    assert!(n < 3000);
                    if g.phase != Phase::RoundOver {
                        let tot = g.deck.len()
                            + g.hands[0].len()
                            + g.hands[1].len()
                            + g.table.len()
                            + g.cap[0].len()
                            + g.cap[1].len()
                            + usize::from(g.phase == Phase::FlipPick);
                        assert_eq!(tot, 48, "{:?}", g.phase);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod a2_audit {
    use super::*;

    #[test]
    fn ai_nu_vede_mana_adversarului_nici_stocul_la_orice_nivel() {
        for (name, kind) in [("hanafuda", 0), ("hwatu", 1)] {
            crate::p3_comun::ai_blind_check(
                name,
                |s| {
                    if kind == 0 {
                        Hanafuda::with_seed(s)
                    } else {
                        Hanafuda::go_stop_with_seed(s)
                    }
                },
                |g, me, seed| {
                    let o = me.other().idx();
                    let n = g.hands[o].len();
                    let mut pool: Vec<u8> = g.hands[o].clone();
                    pool.extend(g.deck.iter().copied());
                    crate::p3_comun::shuffle_with(&mut pool, seed);
                    g.hands[o] = pool.drain(..n).collect();
                    g.deck = pool;
                    g.rng = Rng::new(seed ^ 0x4A);
                },
                12,
            );
        }
    }
}
