//! Dou dizhu („Lupta cu stăpânul”, China) — variantă documentată pentru 2 jucători.
//!
//! Reguli folosite (varianta noastră, din mecanica publică a jocului):
//! * 54 de cărți (3 … 10, J, Q, K, A, 2 și doi jokeri; culoarea nu contează). Ordinea:
//!   3 < 4 < … < K < A < 2 < jokerul mic (X) < jokerul mare (Y).
//! * Fiecare jucător primește 17 cărți; celelalte 20 rămân ascunse și nu se folosesc (în
//!   jocul în trei, 3 cărți ale „stăpânului” completează mâna; aici nu există licitație).
//! * Cine conduce joacă o combinație; celălalt trebuie să dea una **de același tip și lungime,
//!   mai mare**, sau o bombă (4 la fel) sau racheta (X+Y), ori trece (`pas`). Dacă adversarul
//!   trece, cel care a jucat ultimul conduce din nou. Cine rămâne primul fără cărți câștigă runda.
//! * Combinații acceptate: single (5), pereche (55), trio (555), trio + single (555+3), trio +
//!   pereche (555+33), scară de cel puțin 5 single consecutive 3..A (34567), scară de cel puțin
//!   3 perechi consecutive (334455), bombă (5555), rachetă (XY). Simplificări față de jocul
//!   complet: fără avioane (trio-uri consecutive) și fără careu + 2 cărți.
//! * Meci: cel mai bun din 3 runde; conduce J1 în runda 1, J2 în runda 2, J1 în a 3-a.
//! * Informație ascunsă: vezi doar mâna ta și numărul de cărți ale adversarului; AI-ul alege
//!   după propria mână (estimare a numărului de ture și a cărților mari păstrate).

use crate::cards::Viewer;
use crate::cards52::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const CHARS: [char; 15] = [
    '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A', '2', 'X', 'Y',
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Single,
    Pair,
    Triple,
    TripleSingle,
    TriplePair,
    Straight,
    PairStraight,
    Bomb,
    Rocket,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Combo {
    pub kind: Kind,
    /// rangul principal (cel mai mic la scări)
    pub main: u8,
    /// lungimea (numărul de rânduri la scări)
    pub len: u8,
    /// rangul atașat (la trio + 1/2)
    pub attach: u8,
}

impl Combo {
    fn new(kind: Kind, main: u8, len: u8, attach: u8) -> Self {
        Combo {
            kind,
            main,
            len,
            attach,
        }
    }
    pub fn ranks(&self) -> Vec<u8> {
        let m = self.main;
        match self.kind {
            Kind::Single => vec![m],
            Kind::Pair => vec![m; 2],
            Kind::Triple => vec![m; 3],
            Kind::TripleSingle => vec![m, m, m, self.attach],
            Kind::TriplePair => vec![m, m, m, self.attach, self.attach],
            Kind::Straight => (m..m + self.len).collect(),
            Kind::PairStraight => (m..m + self.len).flat_map(|r| [r, r]).collect(),
            Kind::Bomb => vec![m; 4],
            Kind::Rocket => vec![13, 14],
        }
    }
    pub fn text(&self) -> String {
        let c = |r: u8| CHARS[r as usize];
        match self.kind {
            Kind::TripleSingle => format!("{0}{0}{0}+{1}", c(self.main), c(self.attach)),
            Kind::TriplePair => format!("{0}{0}{0}+{1}{1}", c(self.main), c(self.attach)),
            _ => self.ranks().iter().map(|&r| c(r)).collect(),
        }
    }
    /// `self` bate `other` (care e pe masă)?
    pub fn beats(&self, other: &Combo) -> bool {
        if other.kind == Kind::Rocket {
            return false;
        }
        if self.kind == Kind::Rocket {
            return true;
        }
        if self.kind == Kind::Bomb {
            return other.kind != Kind::Bomb || self.main > other.main;
        }
        self.kind == other.kind && self.len == other.len && self.main > other.main
    }
}

fn gen_combos(h: &[u8; 15]) -> Vec<Combo> {
    let mut v = Vec::new();
    for r in 0..15u8 {
        let n = h[r as usize];
        if n >= 1 {
            v.push(Combo::new(Kind::Single, r, 1, 0));
        }
        if n >= 2 && r < 13 {
            v.push(Combo::new(Kind::Pair, r, 1, 0));
        }
        if n >= 3 && r < 13 {
            v.push(Combo::new(Kind::Triple, r, 1, 0));
            for a in 0..15u8 {
                if a != r && h[a as usize] >= 1 {
                    v.push(Combo::new(Kind::TripleSingle, r, 1, a));
                }
                if a != r && a < 13 && h[a as usize] >= 2 {
                    v.push(Combo::new(Kind::TriplePair, r, 1, a));
                }
            }
        }
        if n == 4 {
            v.push(Combo::new(Kind::Bomb, r, 1, 0));
        }
    }
    for s in 0..12u8 {
        let mut len = 0u8;
        while s + len < 12 && h[(s + len) as usize] >= 1 {
            len += 1;
            if len >= 5 {
                v.push(Combo::new(Kind::Straight, s, len, 0));
            }
        }
        let mut len = 0u8;
        while s + len < 12 && h[(s + len) as usize] >= 2 {
            len += 1;
            if len >= 3 {
                v.push(Combo::new(Kind::PairStraight, s, len, 0));
            }
        }
    }
    if h[13] >= 1 && h[14] >= 1 {
        v.push(Combo::new(Kind::Rocket, 13, 1, 0));
    }
    v
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Play,
    RoundOver,
}

#[derive(Clone, Debug)]
pub enum Move {
    Play(Combo),
    Pass,
    Next,
}

impl PartialEq for Move {
    fn eq(&self, o: &Self) -> bool {
        match (self, o) {
            (Move::Play(a), Move::Play(b)) => a == b,
            (Move::Pass, Move::Pass) | (Move::Next, Move::Next) => true,
            _ => false,
        }
    }
}

#[derive(Clone)]
pub struct DouDizhu {
    hands: [[u8; 15]; 2],
    hidden: u8,
    table: Option<(Combo, Player)>,
    side: Player,
    phase: Phase,
    round: u8,
    wins: [u8; 2],
    last: String,
    bombs: u8,
    rng: Rng,
    viewer: Viewer,
}

impl Default for DouDizhu {
    fn default() -> Self {
        Self::new()
    }
}

impl DouDizhu {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = DouDizhu {
            hands: [[0; 15]; 2],
            hidden: 20,
            table: None,
            side: Player::One,
            phase: Phase::Play,
            round: 0,
            wins: [0; 2],
            last: String::new(),
            bombs: 0,
            rng: Rng::new(seed),
            viewer: Viewer::Fixed(Player::One),
        };
        g.deal();
        g
    }

    fn deal(&mut self) {
        let mut d: Vec<u8> = Vec::new();
        for r in 0..13u8 {
            d.extend([r; 4]);
        }
        d.push(13);
        d.push(14);
        shuffle(&mut d, &mut self.rng);
        self.hands = [[0; 15]; 2];
        for p in 0..2 {
            for _ in 0..17 {
                let c = d.pop().unwrap();
                self.hands[p][c as usize] += 1;
            }
        }
        self.hidden = d.len() as u8;
        self.table = None;
        self.bombs = 0;
        self.phase = Phase::Play;
        self.side = if self.round.is_multiple_of(2) {
            Player::One
        } else {
            Player::Two
        };
    }

    pub fn hand_len(&self, p: Player) -> u32 {
        self.hands[p.idx()].iter().map(|&x| x as u32).sum()
    }

    pub fn round_wins(&self) -> (u8, u8) {
        (self.wins[0], self.wins[1])
    }

    fn hand_text(h: &[u8; 15]) -> String {
        let mut s = String::new();
        for r in 0..15 {
            for _ in 0..h[r] {
                s.push(CHARS[r]);
            }
        }
        s
    }

    /// Estimare a numărului de ture necesare pentru a goli mâna.
    #[allow(clippy::needless_range_loop)]
    fn turns(h: &[u8; 15]) -> i32 {
        let mut groups = 0;
        let mut triples = 0;
        let mut smalls = 0;
        for r in 0..15 {
            let n = h[r];
            if n == 0 {
                continue;
            }
            groups += 1;
            if n == 3 && r < 13 {
                triples += 1;
            } else if n < 3 {
                smalls += 1;
            }
        }
        if h[13] >= 1 && h[14] >= 1 {
            groups -= 1; // racheta = o singură tură
        }
        groups - triples.min(smalls)
    }
}

impl Game for DouDizhu {
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
        if self.phase == Phase::RoundOver {
            return vec![Move::Next];
        }
        let combos = gen_combos(&self.hands[self.side.idx()]);
        match &self.table {
            None => combos.into_iter().map(Move::Play).collect(),
            Some((t, _)) => {
                let mut v: Vec<Move> = combos
                    .into_iter()
                    .filter(|c| c.beats(t))
                    .map(Move::Play)
                    .collect();
                v.push(Move::Pass);
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Play(c) => {
                let me = self.side;
                for r in c.ranks() {
                    self.hands[me.idx()][r as usize] -= 1;
                }
                if matches!(c.kind, Kind::Bomb | Kind::Rocket) {
                    self.bombs += 1;
                }
                if self.hand_len(me) == 0 {
                    self.wins[me.idx()] += 1;
                    self.last = format!(
                        "Runda {}: J{} a rămas fără cărți ({}), bombe jucate: {}",
                        self.round + 1,
                        me.idx() + 1,
                        c.text(),
                        self.bombs
                    );
                    self.table = None;
                    self.phase = Phase::RoundOver;
                    return;
                }
                self.table = Some((c.clone(), me));
                self.side = me.other();
            }
            Move::Pass => {
                self.table = None;
                self.side = self.side.other();
            }
            Move::Next => {
                self.round += 1;
                self.deal();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::RoundOver {
            return None;
        }
        if self.wins[0] >= 2 {
            Some(Outcome::Win(Player::One))
        } else if self.wins[1] >= 2 {
            Some(Outcome::Win(Player::Two))
        } else {
            None
        }
    }

    /// Citește doar mâna proprie și numărul de cărți ale adversarului.
    #[allow(clippy::needless_range_loop)]
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let h = &self.hands[me];
        let mut s = 100 * (self.wins[me] as i32 - self.wins[1 - me] as i32) * 10;
        if self.phase == Phase::RoundOver {
            return s;
        }
        s -= 30 * Self::turns(h);
        s += 3 * (self.hand_len(p.other()) as i32 - self.hand_len(p) as i32);
        for r in 0..15 {
            s += (h[r] as i32) * (r as i32) / 2;
            if h[r] == 4 {
                s += 25;
            }
        }
        if h[13] >= 1 && h[14] >= 1 {
            s += 30;
        }
        s
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Dou dizhu — runda {}/3 | runde câștigate: J1 {}, J2 {} | cărți ascunse nefolosite: {}\n",
            (self.round + 1).min(3),
            self.wins[0],
            self.wins[1],
            self.hidden
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
        let v = self.viewer.who(self.side);
        let o = v.other();
        s.push_str(&format!(
            "Mâna ta (J{}): {}\n",
            v.idx() + 1,
            Self::hand_text(&self.hands[v.idx()])
        ));
        s.push_str(&format!("J{}: {} cărți\n", o.idx() + 1, self.hand_len(o)));
        match &self.table {
            Some((c, p)) => s.push_str(&format!("Pe masă: {} (J{})\n", c.text(), p.idx() + 1)),
            None => s.push_str("Masa e goală: conduce cel la mutare\n"),
        }
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Play(c) => c.text(),
            Move::Pass => "pas".into(),
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

    fn hand(s: &str) -> [u8; 15] {
        let mut h = [0u8; 15];
        for ch in s.chars() {
            let i = CHARS.iter().position(|&c| c == ch).unwrap();
            h[i] += 1;
        }
        h
    }

    fn texts(h: &[u8; 15]) -> Vec<String> {
        gen_combos(h).iter().map(|c| c.text()).collect()
    }

    #[test]
    fn combos_generated() {
        let t = texts(&hand("33344456789TJ"));
        assert!(t.contains(&"3".into()));
        assert!(t.contains(&"33".into()));
        assert!(t.contains(&"333".into()));
        assert!(t.contains(&"333+4".into()));
        assert!(t.contains(&"333+44".into()));
        assert!(t.contains(&"3456789".into()));
        assert!(t.contains(&"34567".into()));
        assert!(!t.contains(&"3456".into()));
        let mut u = t.clone();
        u.sort();
        u.dedup();
        assert_eq!(u.len(), t.len(), "texte duplicate");
    }

    #[test]
    fn pair_straight_and_bomb_rocket() {
        let t = texts(&hand("33445555XY"));
        assert!(t.contains(&"334455".into()));
        assert!(t.contains(&"5555".into()));
        assert!(t.contains(&"XY".into()));
        let t2 = texts(&hand("3344"));
        assert!(!t2.contains(&"3344".into()));
    }

    #[test]
    fn twos_and_jokers_not_in_straights() {
        let t = texts(&hand("JQKA2"));
        assert!(!t.iter().any(|x| x.len() == 5));
        let t = texts(&hand("TJQKA"));
        assert!(t.contains(&"TJQKA".into()));
    }

    #[test]
    fn beating_rules() {
        let c = |k, m, l| Combo::new(k, m, l, 0);
        assert!(c(Kind::Single, 5, 1).beats(&c(Kind::Single, 4, 1)));
        assert!(!c(Kind::Single, 4, 1).beats(&c(Kind::Single, 4, 1)));
        assert!(!c(Kind::Pair, 9, 1).beats(&c(Kind::Single, 4, 1)));
        assert!(c(Kind::Bomb, 0, 1).beats(&c(Kind::Pair, 12, 1)));
        assert!(c(Kind::Bomb, 3, 1).beats(&c(Kind::Bomb, 2, 1)));
        assert!(!c(Kind::Bomb, 2, 1).beats(&c(Kind::Bomb, 3, 1)));
        assert!(c(Kind::Rocket, 13, 1).beats(&c(Kind::Bomb, 12, 1)));
        assert!(!c(Kind::Bomb, 12, 1).beats(&c(Kind::Rocket, 13, 1)));
        assert!(!c(Kind::Straight, 4, 6).beats(&c(Kind::Straight, 3, 5)));
        assert!(c(Kind::Straight, 4, 5).beats(&c(Kind::Straight, 3, 5)));
    }

    #[test]
    fn deal_and_conservation() {
        let g = DouDizhu::with_seed(4);
        assert_eq!(g.hand_len(Player::One), 17);
        assert_eq!(g.hand_len(Player::Two), 17);
        assert_eq!(g.hidden, 20);
        for r in 0..13 {
            assert!(g.hands[0][r] + g.hands[1][r] <= 4);
        }
    }

    #[test]
    fn pass_clears_table_and_leader_cannot_pass() {
        let mut g = DouDizhu::with_seed(1);
        g.hands[0] = hand("3456789TJQKA2");
        g.hands[1] = hand("4455667788");
        assert!(!g.legal_moves().contains(&Move::Pass));
        g.apply(&Move::Play(Combo::new(Kind::Single, 0, 1, 0)));
        assert!(g.legal_moves().contains(&Move::Pass));
        g.apply(&Move::Pass);
        assert!(g.table.is_none());
        assert_eq!(g.side, Player::One);
    }

    #[test]
    fn round_win_and_match_flow() {
        let mut g = DouDizhu::with_seed(1);
        g.hands[0] = hand("3");
        g.apply(&Move::Play(Combo::new(Kind::Single, 0, 1, 0)));
        assert_eq!(g.legal_moves(), vec![Move::Next]);
        assert_eq!(g.round_wins(), (1, 0));
        g.apply(&Move::Next);
        assert_eq!(g.side, Player::Two);
        g.hands[1] = hand("4");
        g.apply(&Move::Play(Combo::new(Kind::Single, 1, 1, 0)));
        g.apply(&Move::Next);
        assert_eq!(g.side, Player::One);
        g.hands[0] = hand("5");
        g.apply(&Move::Play(Combo::new(Kind::Single, 2, 1, 0)));
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn render_hides_opponent_and_ai_blind() {
        let g = DouDizhu::with_seed(8);
        let r = Game::render(&g);
        assert!(r.contains("J2: 17 cărți"));
        assert!(r.contains(&DouDizhu::hand_text(&g.hands[0])));
        for seed in 1..25u64 {
            let mut a = DouDizhu::with_seed(seed);
            let mut rng = Rng::new(seed);
            for _ in 0..(seed % 6) {
                let mv = a.legal_moves();
                if a.outcome().is_some() || a.phase != Phase::Play {
                    break;
                }
                a.apply(&mv[rng.below(mv.len())]);
            }
            if a.phase != Phase::Play {
                continue;
            }
            let me = a.side.idx();
            let mut b = a.clone();
            // altă mână (de aceeași mărime) pentru adversar
            let o = 1 - me;
            let from = (0..13).find(|&r| b.hands[o][r] > 0).unwrap();
            let to = (0..13).find(|&r| r != from && b.hands[o][r] < 4).unwrap();
            b.hands[o][from] -= 1;
            b.hands[o][to] += 1;
            assert_eq!(
                polytabletop_core::ai::best_move(&a, 5),
                polytabletop_core::ai::best_move(&b, 5)
            );
        }
    }

    #[test]
    fn random_matches_end() {
        for seed in 1..30u64 {
            let mut g = DouDizhu::with_seed(seed);
            let mut rng = Rng::new(seed ^ 3);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
                assert!(n < 3000);
            }
        }
    }
}

#[cfg(test)]
mod a1_blind {
    use super::*;
    use crate::ai_audit_a1::{audit, shuffle};

    #[test]
    fn ai_ignores_opponent_cards_at_all_levels() {
        audit("doudizhu", DouDizhu::with_seed, 1..=4, 3, 90, |g, salt| {
            let me = g.side.idx();
            let foe = 1 - me;
            let n: usize = g.hands[foe].iter().map(|&x| x as usize).sum();
            let mut pool: Vec<u8> = Vec::new();
            for r in 0..15usize {
                let total: usize = if r < 13 { 4 } else { 1 };
                for _ in 0..total.saturating_sub(g.hands[me][r] as usize) {
                    pool.push(r as u8);
                }
            }
            shuffle(&mut pool, salt);
            g.hands[foe] = [0; 15];
            for &c in pool.iter().take(n) {
                g.hands[foe][c as usize] += 1;
            }
            g.rng = Rng::new(salt ^ 3);
        });
    }
}
