//! Bufet — draft rapid de cărți pentru doi, variantă originală.
//!
//! Mecanica „alegi o carte și pasezi restul mâinii”, cu teme și valori proprii:
//! * Mici (perechi): fiecare pereche = 5 puncte.
//! * Salată (triplete): fiecare triplet = 10 puncte.
//! * Colțunași: 1, 3, 6, 10, 15 puncte pentru 1..5+ cărți (se adună).
//! * Cârnați de 1/2/3 puncte; Muștarul triplează următorul cârnat jucat.
//! * Pateuri (1/2/3 pictograme): cele mai multe pictograme într-o rundă = 6, locul 2 = 3
//!   (egalitate: 6 împărțit).
//! * Papanași: la sfârșitul celor 3 runde, cei mai mulți +6, cei mai puțini −6.
//!
//! Fiecare rundă: 10 cărți în mână; alegeți simultan, apoi mâinile se schimbă. Aici alegerea
//! simultană este simulată: Jucătorul 1 alege în secret, apoi Jucătorul 2, apoi o mutare
//! forțată (`revelare`) dezvăluie ambele alegeri. AI-ul nu citește alegerea ascunsă a
//! adversarului și nici mâna lui.
#![allow(clippy::needless_range_loop, clippy::doc_lazy_continuation)]

use crate::cards::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum K {
    Mici,
    Salata,
    Coltunasi,
    Carnat1,
    Carnat2,
    Carnat3,
    Mustar,
    Pateu1,
    Pateu2,
    Pateu3,
    Papanas,
}

const ALL: [K; 11] = [
    K::Mici,
    K::Salata,
    K::Coltunasi,
    K::Carnat1,
    K::Carnat2,
    K::Carnat3,
    K::Mustar,
    K::Pateu1,
    K::Pateu2,
    K::Pateu3,
    K::Papanas,
];

impl K {
    pub fn name(self) -> &'static str {
        match self {
            K::Mici => "mici",
            K::Salata => "salata",
            K::Coltunasi => "coltunasi",
            K::Carnat1 => "carnat1",
            K::Carnat2 => "carnat2",
            K::Carnat3 => "carnat3",
            K::Mustar => "mustar",
            K::Pateu1 => "pateu1",
            K::Pateu2 => "pateu2",
            K::Pateu3 => "pateu3",
            K::Papanas => "papanas",
        }
    }
    fn count(self) -> usize {
        match self {
            K::Mici | K::Salata | K::Coltunasi => 14,
            K::Carnat1 | K::Carnat3 => 5,
            K::Carnat2 => 10,
            K::Mustar => 6,
            K::Pateu1 => 6,
            K::Pateu2 => 12,
            K::Pateu3 => 8,
            K::Papanas => 10,
        }
    }
    fn icons(self) -> i32 {
        match self {
            K::Pateu1 => 1,
            K::Pateu2 => 2,
            K::Pateu3 => 3,
            _ => 0,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Pick(K),
    Reveal,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Pick,
    Reveal,
}

/// Punctele unei mese fără pateuri (comparative) și fără papanași (final).
pub fn base_score(tab: &[K]) -> i32 {
    let n = |k: K| tab.iter().filter(|x| **x == k).count() as i32;
    let mut s = 5 * (n(K::Mici) / 2) + 10 * (n(K::Salata) / 3);
    s += match n(K::Coltunasi) {
        0 => 0,
        1 => 1,
        2 => 3,
        3 => 6,
        4 => 10,
        _ => 15,
    };
    let mut must = 0;
    for k in tab {
        match k {
            K::Mustar => must += 1,
            K::Carnat1 | K::Carnat2 | K::Carnat3 => {
                let v = match k {
                    K::Carnat1 => 1,
                    K::Carnat2 => 2,
                    _ => 3,
                };
                if must > 0 {
                    must -= 1;
                    s += 3 * v;
                } else {
                    s += v;
                }
            }
            _ => {}
        }
    }
    s
}

pub fn icons(tab: &[K]) -> i32 {
    tab.iter().map(|k| k.icons()).sum()
}

/// Bonus pateuri pentru (a, b) pictograme (2 jucători).
pub fn maki_bonus(a: i32, b: i32) -> (i32, i32) {
    if a == 0 && b == 0 {
        (0, 0)
    } else if a > b {
        (6, if b > 0 { 3 } else { 0 })
    } else if b > a {
        (if a > 0 { 3 } else { 0 }, 6)
    } else {
        (3, 3)
    }
}

#[derive(Clone)]
pub struct Bufet {
    deck: Vec<K>,
    discard: Vec<K>,
    hands: [Vec<K>; 2],
    tab: [Vec<K>; 2],
    pud: [u8; 2],
    score: [i32; 2],
    round: u8,
    pending: [Option<K>; 2],
    side: Player,
    phase: Phase,
    over: bool,
    /// `Some(p)` = render shows `p`'s private cards; `None` = hot-seat (the mover's).
    viewer: Option<Player>,
}

impl Default for Bufet {
    fn default() -> Self {
        Self::new()
    }
}

impl Bufet {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck = Vec::new();
        for k in ALL {
            for _ in 0..k.count() {
                deck.push(k);
            }
        }
        shuffle(&mut deck, &mut rng);
        let mut g = Bufet {
            deck,
            discard: Vec::new(),
            hands: [Vec::new(), Vec::new()],
            tab: [Vec::new(), Vec::new()],
            pud: [0; 2],
            score: [0; 2],
            round: 1,
            pending: [None; 2],
            side: Player::One,
            phase: Phase::Pick,
            over: false,
            viewer: Some(Player::One),
        };
        g.deal();
        g
    }

    fn deal(&mut self) {
        for p in 0..2 {
            for _ in 0..10 {
                let c = self.deck.pop().expect("deck");
                self.hands[p].push(c);
            }
            self.hands[p].sort_by_key(|k| *k as u8);
        }
    }

    pub fn total_cards(&self) -> usize {
        self.deck.len()
            + self.discard.len()
            + self.hands.iter().map(|h| h.len()).sum::<usize>()
            + self.tab.iter().map(|h| h.len()).sum::<usize>()
            + self.pud.iter().map(|x| *x as usize).sum::<usize>()
            + self.pending.iter().flatten().count()
    }

    pub fn scores(&self) -> (i32, i32) {
        (self.score[0], self.score[1])
    }

    fn end_round(&mut self) {
        let (a, b) = (icons(&self.tab[0]), icons(&self.tab[1]));
        let (ma, mb) = maki_bonus(a, b);
        self.score[0] += base_score(&self.tab[0]) + ma;
        self.score[1] += base_score(&self.tab[1]) + mb;
        for p in 0..2 {
            let t = std::mem::take(&mut self.tab[p]);
            for k in t {
                if k == K::Papanas {
                    self.pud[p] += 1;
                } else {
                    self.discard.push(k);
                }
            }
        }
        if self.round == 3 {
            let (a, b) = (self.pud[0], self.pud[1]);
            if a > b {
                self.score[0] += 6;
                self.score[1] -= 6;
            } else if b > a {
                self.score[1] += 6;
                self.score[0] -= 6;
            }
            self.over = true;
        } else {
            self.round += 1;
            self.deal();
        }
    }

    /// Valoare euristică a unei mese (inclusiv credit parțial).
    fn potential(tab: &[K]) -> f64 {
        let n = |k: K| tab.iter().filter(|x| **x == k).count() as f64;
        let mut s = base_score(tab) as f64;
        s += if (n(K::Mici) as i32) % 2 == 1 {
            2.0
        } else {
            0.0
        };
        let sal = (n(K::Salata) as i32) % 3;
        s += [0.0, 2.5, 6.0][sal as usize];
        let must_left = {
            let mut m = 0i32;
            for k in tab {
                match k {
                    K::Mustar => m += 1,
                    K::Carnat1 | K::Carnat2 | K::Carnat3 if m > 0 => m -= 1,
                    _ => {}
                }
            }
            m
        };
        s += 3.0 * must_left as f64;
        s += 2.0 * n(K::Papanas);
        // Coltunașii: următorul valorează mult.
        s += match n(K::Coltunasi) as i32 {
            1 => 1.0,
            2 => 1.5,
            _ => 0.0,
        };
        s
    }
}

impl Game for Bufet {
    type Move = Move;

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over {
            return Vec::new();
        }
        match self.phase {
            Phase::Reveal => vec![Move::Reveal],
            Phase::Pick => {
                let mut v: Vec<K> = Vec::new();
                for &k in &self.hands[self.side.idx()] {
                    if !v.contains(&k) {
                        v.push(k);
                    }
                }
                v.into_iter().map(Move::Pick).collect()
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        match *m {
            Move::Pick(k) => {
                let p = self.side.idx();
                let i = self.hands[p].iter().position(|x| *x == k).expect("in hand");
                self.hands[p].remove(i);
                self.pending[p] = Some(k);
                if self.pending[1 - p].is_some() {
                    self.phase = Phase::Reveal;
                    self.side = Player::One;
                } else {
                    self.side = self.side.other();
                }
            }
            Move::Reveal => {
                for p in 0..2 {
                    if let Some(k) = self.pending[p].take() {
                        self.tab[p].push(k);
                    }
                }
                self.hands.swap(0, 1);
                self.phase = Phase::Pick;
                self.side = Player::One;
                if self.hands[0].is_empty() {
                    self.end_round();
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            return None;
        }
        Some(match self.score[0].cmp(&self.score[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        // Doar ce vede `p`: mesele dezvăluite, propria alegere în așteptare,
        // nu și alegerea ascunsă a adversarului, nici mâna lui.
        let me = p.idx();
        let op = 1 - me;
        let mut mine = self.tab[me].clone();
        if let Some(k) = self.pending[me] {
            mine.push(k);
        }
        let theirs = &self.tab[op];
        let (ma, mb) = maki_bonus(icons(&mine), icons(theirs));
        let own = Self::potential(&mine) + ma as f64 + 0.8 * icons(&mine) as f64;
        let opp = Self::potential(theirs) + mb as f64;
        let pud = 3.0 * (self.pud[me] as f64 - self.pud[op] as f64);
        let sc = (self.score[me] - self.score[op]) as f64;
        ((sc + own - opp + pud) * 10.0) as i32
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }

    fn render(&self) -> String {
        let mut s = String::new();
        let list = |v: &[K]| {
            if v.is_empty() {
                "—".to_string()
            } else {
                v.iter().map(|k| k.name()).collect::<Vec<_>>().join(" ")
            }
        };
        s.push_str(&format!(
            "Bufet — runda {}/3 | scor: J1 {}, J2 {} | papanași: J1 {}, J2 {}\n",
            self.round, self.score[0], self.score[1], self.pud[0], self.pud[1]
        ));
        s.push_str(&format!("Masa J1: {}\n", list(&self.tab[0])));
        s.push_str(&format!("Masa J2: {}\n", list(&self.tab[1])));
        if self.over {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        match self.phase {
            Phase::Pick => {
                let v = self.viewer.unwrap_or(self.side);
                let (vi, oi) = (v.idx(), v.other().idx());
                if let Some(k) = self.pending[vi] {
                    s.push_str(&format!("Ai ales în secret: {}\n", k.name()));
                }
                s.push_str(&format!(
                    "Mâna ta (J{}): {}\n",
                    vi + 1,
                    list(&self.hands[vi])
                ));
                s.push_str(&format!("Adversarul are {} cărți.\n", self.hands[oi].len()));
            }
            Phase::Reveal => {
                s.push_str("Ambele alegeri sunt făcute: dezvăluiți (revelare).\n");
            }
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Pick(k) => format!("alege {}", k.name()),
            Move::Reveal => "revelare".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scoring_basics() {
        assert_eq!(base_score(&[K::Mici, K::Mici, K::Mici]), 5);
        assert_eq!(base_score(&[K::Salata; 3]), 10);
        assert_eq!(base_score(&[K::Salata; 5]), 10);
        assert_eq!(base_score(&[K::Coltunasi; 4]), 10);
        assert_eq!(base_score(&[K::Coltunasi; 7]), 15);
        assert_eq!(base_score(&[K::Mustar, K::Carnat3]), 9);
        assert_eq!(base_score(&[K::Carnat3, K::Mustar]), 3);
        assert_eq!(
            base_score(&[K::Mustar, K::Mustar, K::Carnat1, K::Carnat2]),
            9
        );
        assert_eq!(maki_bonus(5, 2), (6, 3));
        assert_eq!(maki_bonus(2, 2), (3, 3));
        assert_eq!(maki_bonus(0, 3), (0, 6));
        assert_eq!(maki_bonus(0, 0), (0, 0));
    }

    #[test]
    fn full_game_conserves_cards() {
        for seed in 1..30 {
            let mut g = Bufet::with_seed(seed);
            let total = g.total_cards();
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                g.apply(&m[rng.below(m.len())]);
                assert_eq!(g.total_cards(), total);
                n += 1;
                assert!(n < 200);
            }
            // 3 runde × 10 alegeri × 2 jucători + 30 revelări
            assert_eq!(n, 3 * 10 * 3);
        }
    }

    #[test]
    fn hidden_pick_not_on_table_before_reveal() {
        let mut g = Bufet::with_seed(5);
        let m = g.legal_moves()[0];
        g.apply(&m);
        assert!(g.tab[0].is_empty());
        assert_eq!(g.to_move(), Player::Two);
        let m = g.legal_moves()[0];
        g.apply(&m);
        assert_eq!(g.legal_moves(), vec![Move::Reveal]);
    }

    #[test]
    fn ai_beats_random() {
        let mut wins = 0;
        for seed in 1..=40u64 {
            let mut g = Bufet::with_seed(seed);
            let mut rng = Rng::new(seed + 7);
            while g.outcome().is_none() {
                let m = if g.to_move() == Player::Two || g.phase == Phase::Reveal {
                    polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap()
                } else {
                    let l = g.legal_moves();
                    l[rng.below(l.len())]
                };
                g.apply(&m);
            }
            if g.outcome() == Some(Outcome::Win(Player::Two)) {
                wins += 1;
            }
        }
        assert!(wins >= 30, "wins {wins}");
    }
}

#[cfg(test)]
mod viewer_tests {
    use super::*;

    #[test]
    fn viewer_sees_only_own_hand_and_pick() {
        let mut g = Bufet::with_seed(5);
        let names = |p: Player| {
            g.hands[p.idx()]
                .iter()
                .map(|k| k.name())
                .collect::<Vec<_>>()
                .join(" ")
        };
        let (h1, h2) = (names(Player::One), names(Player::Two));
        Game::set_viewer(&mut g, Some(Player::One));
        assert!(Game::render(&g).contains(&format!("Mâna ta (J1): {h1}\n")));
        Game::set_viewer(&mut g, Some(Player::Two));
        let r = Game::render(&g);
        assert!(r.contains(&format!("Mâna ta (J2): {h2}\n")), "{r}");
        assert!(!r.contains("Mâna ta (J1)"), "{r}");
        // J1 alege; J2 (hot-seat) nu trebuie să vadă alegerea lui J1
        let m = Game::legal_moves(&g)[0];
        let mut h = Bufet::with_seed(5);
        h.apply(&m);
        assert_eq!(Game::to_move(&h), Player::Two);
        Game::set_viewer(&mut h, None);
        assert!(!Game::render(&h).contains("Ai ales în secret"));
        Game::set_viewer(&mut h, Some(Player::One));
        assert!(Game::render(&h).contains("Ai ales în secret"));
    }
}
