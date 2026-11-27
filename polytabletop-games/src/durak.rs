//! Durak (Rusia), 2 jucători, pachet de 36 de cărți (6..A), din regulile publice
//! (varianta „podkidnoi”: atacatorul aruncă pe rând, fără pase).
//!
//! * Fiecare primește 6 cărți; următoarea carte se întoarce și dă **atuul** (rămâne jos).
//!   Începe cine are cel mai mic atu (sau Jucătorul 1 dacă nimeni nu are).
//! * Atacatorul pune o carte. Apărătorul o **bate** cu o carte mai mare de aceeași
//!   culoare sau cu un atu (un atu se bate doar cu un atu mai mare), ori **ia** cărțile
//!   de pe masă. Atacatorul poate adăuga cărți cu o valoare deja aflată pe masă
//!   (în total cel mult 6, și cel mult câte cărți avea apărătorul la început;
//!   în prima rundă cel mult 5). Când totul e bătut și atacatorul nu mai adaugă
//!   (`gata`), cărțile se retrag și apărătorul devine atacator; dacă a luat,
//!   atacatorul rămâne atacator, iar după ce apărătorul ia, atacatorul mai poate adăuga.
//! * După fiecare rundă se trage până la 6 cărți (întâi atacatorul), într-o mutare
//!   forțată `umple`; atuul întors se trage ultimul.
//! * Cu pachetul gol, cine rămâne primul fără cărți câștigă; cine rămâne cu cărți e
//!   „durak” (pierde). Dacă rămân fără cărți ambii, remiză.
//!
//! Mutări: `6h` (atac / adăugare), `9h>6h` (bate 6h cu 9h), `iau`, `gata`, `umple`.
//!
//! Informație ascunsă: vezi `cards52.rs`. `render` arată doar mâna Jucătorului 1,
//! numărul cărților celuilalt și cărțile de pe masă; `evaluate` citește doar
//! mâna celui evaluat și informația publică.

use crate::cards::Viewer;
use crate::cards52::{shuffle, SUIT_LETTERS};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const RANK_LABEL: [&str; 9] = ["6", "7", "8", "9", "10", "J", "Q", "K", "A"];

pub fn suit(c: u8) -> u8 {
    c / 9
}
pub fn rank(c: u8) -> u8 {
    c % 9
}
pub fn card_text(c: u8) -> String {
    format!(
        "{}{}",
        RANK_LABEL[rank(c) as usize],
        SUIT_LETTERS[suit(c) as usize]
    )
}

/// `by` bate `att`?
pub fn beats(att: u8, by: u8, trump: u8) -> bool {
    if suit(att) == suit(by) {
        rank(by) > rank(att)
    } else {
        suit(by) == trump
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Move {
    Attack(u8),
    Cover(u8, u8),
    Take,
    Done,
    Refill,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Attack,
    Defend,
    /// Apărătorul a luat; atacatorul mai poate adăuga.
    AddOn,
    Refill,
}

#[derive(Clone)]
pub struct Durak {
    stock: Vec<u8>,
    trump: u8,
    hands: [Vec<u8>; 2],
    attacker: Player,
    table: Vec<(u8, Option<u8>)>,
    phase: Phase,
    limit: usize,
    discard: usize,
    first_bout: bool,
    defended: bool,
    result: Option<Outcome>,
    viewer: Viewer,
}

impl Default for Durak {
    fn default() -> Self {
        Self::new()
    }
}

impl Durak {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck: Vec<u8> = (0..36).collect();
        shuffle(&mut deck, &mut rng);
        let hands = [deck[0..6].to_vec(), deck[6..12].to_vec()];
        let stock = deck[12..].to_vec();
        let trump = suit(stock[0]);
        let low = |h: &Vec<u8>| {
            h.iter()
                .filter(|&&c| suit(c) == trump)
                .map(|&c| rank(c))
                .min()
        };
        let attacker = match (low(&hands[0]), low(&hands[1])) {
            (Some(a), Some(b)) if b < a => Player::Two,
            (None, Some(_)) => Player::Two,
            _ => Player::One,
        };
        Durak {
            stock,
            trump,
            hands,
            attacker,
            table: Vec::new(),
            phase: Phase::Attack,
            limit: 5,
            discard: 0,
            first_bout: true,
            defended: false,
            result: None,
            viewer: Viewer::Fixed(Player::One),
        }
    }

    pub fn trump_suit(&self) -> u8 {
        self.trump
    }
    pub fn hand_of(&self, p: Player) -> &[u8] {
        &self.hands[p.idx()]
    }
    pub fn stock_len(&self) -> usize {
        self.stock.len()
    }
    pub fn attacker(&self) -> Player {
        self.attacker
    }

    fn defender(&self) -> Player {
        self.attacker.other()
    }

    fn table_ranks(&self) -> Vec<u8> {
        let mut v = Vec::new();
        for &(a, d) in &self.table {
            v.push(rank(a));
            if let Some(d) = d {
                v.push(rank(d));
            }
        }
        v
    }

    fn all_covered(&self) -> bool {
        self.table.iter().all(|&(_, d)| d.is_some())
    }

    fn contrib(&self, c: u8) -> i32 {
        if suit(c) == self.trump {
            8 + 5 * rank(c) as i32
        } else {
            rank(c) as i32 - 10
        }
    }

    fn finish_bout(&mut self) {
        self.phase = Phase::Refill;
    }
}

impl Game for Durak {
    type Move = Move;

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = match who {
            Some(p) => Viewer::Fixed(p),
            None => Viewer::HotSeat,
        };
    }

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Defend => self.defender(),
            _ => self.attacker,
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.result.is_some() {
            return Vec::new();
        }
        let mut v = Vec::new();
        match self.phase {
            Phase::Refill => v.push(Move::Refill),
            Phase::Attack | Phase::AddOn => {
                let hand = &self.hands[self.attacker.idx()];
                if self.table.is_empty() {
                    for &c in hand {
                        v.push(Move::Attack(c));
                    }
                } else {
                    if self.table.len() < self.limit {
                        let ranks = self.table_ranks();
                        for &c in hand {
                            if ranks.contains(&rank(c)) {
                                v.push(Move::Attack(c));
                            }
                        }
                    }
                    v.push(Move::Done);
                }
            }
            Phase::Defend => {
                let hand = &self.hands[self.defender().idx()];
                for &(a, d) in &self.table {
                    if d.is_none() {
                        for &c in hand {
                            if beats(a, c, self.trump) {
                                v.push(Move::Cover(a, c));
                            }
                        }
                    }
                }
                v.push(Move::Take);
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        match *m {
            Move::Attack(c) => {
                let h = &mut self.hands[self.attacker.idx()];
                let pos = h.iter().position(|&x| x == c).expect("card in hand");
                h.remove(pos);
                if self.table.is_empty() {
                    // limita rundei: cărțile apărătorului la început (max 6; 5 la prima)
                    let d = self.hands[self.defender().idx()].len();
                    self.limit = if self.first_bout { d.min(5) } else { d.min(6) };
                }
                self.table.push((c, None));
                if self.phase == Phase::Attack {
                    self.phase = Phase::Defend;
                }
            }
            Move::Cover(a, c) => {
                let h = &mut self.hands[self.defender().idx()];
                let pos = h.iter().position(|&x| x == c).expect("card in hand");
                h.remove(pos);
                for t in self.table.iter_mut() {
                    if t.0 == a {
                        t.1 = Some(c);
                    }
                }
                if self.all_covered() {
                    self.phase = Phase::Attack;
                }
            }
            Move::Take => {
                self.phase = Phase::AddOn;
            }
            Move::Done => {
                if self.phase == Phase::AddOn {
                    // apărătorul ia toate cărțile
                    let d = self.defender().idx();
                    for (a, c) in std::mem::take(&mut self.table) {
                        self.hands[d].push(a);
                        if let Some(c) = c {
                            self.hands[d].push(c);
                        }
                    }
                    self.finish_bout();
                } else {
                    self.discard += self.table.len() * 2;
                    self.table.clear();
                    self.defended = true;
                    self.finish_bout();
                }
            }
            Move::Refill => {
                // întâi atacatorul rundei încheiate, apoi apărătorul
                for p in [self.attacker, self.attacker.other()] {
                    while self.hands[p.idx()].len() < 6 {
                        match self.stock.pop() {
                            Some(c) => self.hands[p.idx()].push(c),
                            None => break,
                        }
                    }
                }
                if self.defended {
                    self.attacker = self.defender();
                }
                self.defended = false;
                self.first_bout = false;
                if self.stock.is_empty() {
                    let e = [self.hands[0].is_empty(), self.hands[1].is_empty()];
                    self.result = match e {
                        [true, true] => Some(Outcome::Draw),
                        [true, false] => Some(Outcome::Win(Player::One)),
                        [false, true] => Some(Outcome::Win(Player::Two)),
                        _ => None,
                    };
                }
                self.phase = Phase::Attack;
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.result
    }

    /// Citește doar mâna lui `p` și informația publică (numărul cărților adversarului).
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let mut s: i32 = self.hands[me].iter().map(|&c| self.contrib(c)).sum();
        s += self.hands[1 - me].len() as i32 * 6;
        // cine atacă runda următoare (se știe după apărare / luare)
        if self.phase == Phase::Refill {
            let next_attacker = if self.defended {
                self.attacker.other()
            } else {
                self.attacker
            };
            s += if next_attacker == p { 60 } else { -60 };
        }
        s
    }

    fn render(&self) -> String {
        let mut s = String::new();
        s.push_str("Durak — culori: c treflă, d caro, h cupă, s pică; 6 7 8 9 10 J Q K A\n");
        s.push_str(&format!(
            "Atu: {}{}. Pachet: {} cărți, retrase: {}. Atacator: Jucător {}.\n",
            SUIT_LETTERS[self.trump as usize],
            match self.stock.first() {
                Some(&t) => format!(" (jos: {})", card_text(t)),
                None => String::new(),
            },
            self.stock.len(),
            self.discard,
            self.attacker.idx() + 1
        ));
        let v = self.viewer.who(self.to_move());
        let o = v.other();
        s.push_str(&format!(
            "Jucător {} (ascuns) are {} cărți.\n",
            o.idx() + 1,
            self.hands[o.idx()].len()
        ));
        if self.table.is_empty() {
            s.push_str("Masa: —\n");
        } else {
            s.push_str("Masa: ");
            let parts: Vec<String> = self
                .table
                .iter()
                .map(|&(a, d)| match d {
                    Some(d) => format!("{}>{}", card_text(d), card_text(a)),
                    None => format!("?>{}", card_text(a)),
                })
                .collect();
            s.push_str(&parts.join("  "));
            s.push('\n');
        }
        let mut h = self.hands[v.idx()].clone();
        h.sort_unstable();
        s.push_str(&format!(
            "Mâna ta (Jucător {}): {}\n",
            v.idx() + 1,
            if h.is_empty() {
                "—".into()
            } else {
                h.iter()
                    .map(|&c| card_text(c))
                    .collect::<Vec<_>>()
                    .join(" ")
            }
        ));
        match self.result {
            Some(Outcome::Win(p)) => s.push_str(&format!(
                "Final: Jucător {} câștigă; Jucător {} e durak.\n",
                p.idx() + 1,
                p.other().idx() + 1
            )),
            Some(Outcome::Draw) => s.push_str("Final: remiză.\n"),
            None => s.push_str(&format!(
                "La mutare: Jucător {}\n",
                self.to_move().idx() + 1
            )),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Attack(c) => card_text(*c),
            Move::Cover(a, c) => format!("{}>{}", card_text(*c), card_text(*a)),
            Move::Take => "iau".into(),
            Move::Done => "gata".into(),
            Move::Refill => "umple".into(),
        }
    }

    /// Adâncime 0 (o singură mutare): `AlphaBeta(1)` ar fi extins la Hard/Expert răspunsul adversarului din
    /// mâna lui reală (și tragerea din pachetul real); evaluarea citește doar mâna proprie și datele publice.
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // culoare * 9 + rang (0 = 6 … 8 = A)
    fn c(s: u8, r: u8) -> u8 {
        s * 9 + r
    }

    fn base() -> Durak {
        let mut g = Durak::with_seed(1);
        g.stock.clear();
        g.trump = 3;
        g.hands = [Vec::new(), Vec::new()];
        g.table.clear();
        g.attacker = Player::One;
        g.phase = Phase::Attack;
        g.limit = 6;
        g.first_bout = false;
        g
    }

    #[test]
    fn deal_is_consistent() {
        let g = Durak::with_seed(3);
        let mut all: Vec<u8> = g.hands[0]
            .iter()
            .chain(&g.hands[1])
            .chain(&g.stock)
            .copied()
            .collect();
        all.sort_unstable();
        assert_eq!(all, (0..36).collect::<Vec<u8>>());
        assert_eq!(g.trump, suit(g.stock[0]));
        assert_eq!(g.stock.len(), 24);
    }

    #[test]
    fn beating_rules() {
        let t = 3;
        assert!(beats(c(0, 2), c(0, 5), t));
        assert!(!beats(c(0, 5), c(0, 2), t));
        assert!(!beats(c(0, 2), c(1, 8), t));
        assert!(beats(c(0, 8), c(3, 0), t)); // atu bate asul
        assert!(!beats(c(3, 5), c(0, 8), t));
        assert!(beats(c(3, 5), c(3, 6), t));
        assert!(!beats(c(3, 6), c(3, 5), t));
    }

    #[test]
    fn lowest_trump_starts() {
        for seed in 1..30 {
            let g = Durak::with_seed(seed);
            let low = |p: usize| {
                g.hands[p]
                    .iter()
                    .filter(|&&x| suit(x) == g.trump)
                    .map(|&x| rank(x))
                    .min()
            };
            match (low(0), low(1)) {
                (Some(a), Some(b)) => assert_eq!(g.attacker == Player::One, a < b),
                (None, Some(_)) => assert_eq!(g.attacker, Player::Two),
                _ => assert_eq!(g.attacker, Player::One),
            }
        }
    }

    #[test]
    fn attack_must_match_ranks_on_the_table() {
        let mut g = base();
        g.hands[0] = vec![c(0, 1), c(1, 1), c(2, 4), c(2, 7)];
        g.hands[1] = vec![c(0, 8), c(1, 8), c(3, 0), c(2, 8)];
        g.stock = vec![c(3, 8)]; // ceva în pachet
        g.apply(&Move::Attack(c(0, 1)));
        assert_eq!(g.to_move(), Player::Two);
        g.apply(&Move::Cover(c(0, 1), c(0, 8)));
        assert_eq!(g.to_move(), Player::One);
        let mv = g.legal_moves();
        assert!(mv.contains(&Move::Attack(c(1, 1)))); // același rang 7
        assert!(!mv.contains(&Move::Attack(c(2, 4))));
        assert!(mv.contains(&Move::Done));
    }

    #[test]
    fn defender_options() {
        let mut g = base();
        g.hands[0] = vec![c(0, 1)];
        g.hands[1] = vec![c(0, 8), c(0, 0), c(3, 0), c(1, 8)];
        g.stock = vec![c(3, 8)];
        g.apply(&Move::Attack(c(0, 1)));
        let mv = g.legal_moves();
        assert!(mv.contains(&Move::Cover(c(0, 1), c(0, 8))));
        assert!(mv.contains(&Move::Cover(c(0, 1), c(3, 0)))); // atu
        assert!(!mv.contains(&Move::Cover(c(0, 1), c(0, 0)))); // mai mică
        assert!(!mv.contains(&Move::Cover(c(0, 1), c(1, 8)))); // altă culoare
        assert!(mv.contains(&Move::Take));
    }

    #[test]
    fn defended_bout_swaps_roles_and_refills() {
        let mut g = base();
        g.stock = vec![c(3, 8), c(2, 0), c(2, 1), c(2, 2), c(2, 3)];
        g.hands[0] = vec![c(0, 1), c(0, 2), c(0, 3), c(0, 4), c(0, 5), c(0, 6)];
        g.hands[1] = vec![c(0, 8), c(1, 0), c(1, 1), c(1, 2), c(1, 3), c(1, 4)];
        g.apply(&Move::Attack(c(0, 1)));
        g.apply(&Move::Cover(c(0, 1), c(0, 8)));
        g.apply(&Move::Done);
        assert_eq!(g.legal_moves(), vec![Move::Refill]);
        g.apply(&Move::Refill);
        // atacatorul a tras primul: 5 -> 6 (o carte), apărătorul 5 -> 6
        assert_eq!(g.hands[0].len(), 6);
        assert_eq!(g.hands[1].len(), 6);
        assert_eq!(g.attacker, Player::Two);
        assert_eq!(g.stock.len(), 3);
        assert!(g.table.is_empty());
    }

    #[test]
    fn taking_gives_all_cards_and_keeps_roles() {
        let mut g = base();
        g.stock = vec![c(3, 8), c(2, 0), c(2, 1), c(2, 2), c(2, 3), c(2, 4)];
        g.hands[0] = vec![c(0, 1), c(1, 1), c(0, 3), c(0, 4), c(0, 5), c(0, 6)];
        g.hands[1] = vec![c(1, 0), c(1, 1), c(1, 2), c(1, 3), c(1, 4), c(1, 5)];
        g.apply(&Move::Attack(c(0, 1)));
        g.apply(&Move::Take);
        assert_eq!(g.to_move(), Player::One);
        g.apply(&Move::Attack(c(1, 1))); // adaugă un 7
        g.apply(&Move::Done);
        assert_eq!(g.hands[1].len(), 8);
        g.apply(&Move::Refill);
        assert_eq!(g.attacker, Player::One);
        assert_eq!(g.hands[0].len(), 6); // 4 + 2 trase
        assert_eq!(g.hands[1].len(), 8); // nu trage peste 6
    }

    #[test]
    fn first_bout_is_limited_to_five() {
        let mut g = base();
        g.first_bout = true;
        g.stock = vec![c(3, 8)];
        g.hands[0] = (0..6).map(|i| c(i % 3, 0)).collect();
        g.hands[0] = vec![c(0, 0), c(1, 0), c(2, 0), c(3, 0), c(0, 1), c(1, 1)];
        g.hands[1] = vec![c(0, 8), c(1, 8), c(2, 8), c(3, 8), c(0, 7), c(1, 7)];
        g.apply(&Move::Attack(c(0, 0)));
        assert_eq!(g.limit, 5);
    }

    #[test]
    fn game_end_conditions() {
        let mut g = base();
        g.hands[0] = vec![c(0, 1)];
        g.hands[1] = vec![c(0, 8)];
        g.apply(&Move::Attack(c(0, 1)));
        g.apply(&Move::Cover(c(0, 1), c(0, 8)));
        g.apply(&Move::Done);
        g.apply(&Move::Refill);
        assert_eq!(g.outcome(), Some(Outcome::Draw));
        // atacatorul rămâne fără cărți, apărătorul ia: atacatorul câștigă
        let mut h = base();
        h.hands[0] = vec![c(0, 1)];
        h.hands[1] = vec![c(1, 0), c(1, 1)];
        h.apply(&Move::Attack(c(0, 1)));
        h.apply(&Move::Take);
        h.apply(&Move::Done);
        h.apply(&Move::Refill);
        assert_eq!(h.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn cards_are_conserved_and_random_games_finish() {
        for seed in 1..=40u64 {
            let mut g = Durak::with_seed(seed);
            let mut rng = Rng::new(seed ^ 0x99);
            let mut n = 0;
            while g.outcome().is_none() {
                let all = g.stock.len()
                    + g.discard
                    + g.hands[0].len()
                    + g.hands[1].len()
                    + g.table
                        .iter()
                        .map(|t| 1 + usize::from(t.1.is_some()))
                        .sum::<usize>();
                assert_eq!(all, 36);
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 5000, "durak does not end");
            }
        }
    }

    #[test]
    fn render_hides_the_opponent() {
        let g = Durak::with_seed(5);
        let r = Game::render(&g);
        assert!(r.contains("Jucător 2 (ascuns) are 6 cărți"));
    }

    #[test]
    fn evaluate_ignores_the_opponents_cards() {
        let g = Durak::with_seed(9);
        let mut h = g.clone();
        let (a, b) = (h.hands[1][0], h.stock[3]);
        h.hands[1][0] = b;
        h.stock[3] = a;
        assert_eq!(g.evaluate(Player::One), h.evaluate(Player::One));
    }

    #[test]
    fn ai_beats_a_random_mover_mostly() {
        use polytabletop_core::GameObj;
        let mut wins = 0;
        let n = 40;
        for seed in 1..=n {
            let mut g: Box<dyn GameObj> = Box::new(Durak::with_seed(seed));
            let mut rng = Rng::new(seed + 100);
            while g.outcome().is_none() {
                let mv = if g.to_move() == Player::One {
                    g.ai_move(rng.next_u64()).unwrap()
                } else {
                    let m = g.legal_moves();
                    m[rng.below(m.len())].clone()
                };
                g.play(&mv).unwrap();
            }
            if g.outcome() == Some(Outcome::Win(Player::One)) {
                wins += 1;
            }
        }
        assert!(wins * 10 >= n * 7, "AI won only {wins}/{n}");
    }
}

#[cfg(test)]
mod a2_audit {
    use super::*;

    #[test]
    fn ai_nu_vede_mana_adversarului_nici_pachetul_la_orice_nivel() {
        crate::p3_comun::ai_blind_check(
            "durak",
            Durak::with_seed,
            |g, me, seed| {
                // atuul de jos (stock[0]) e public; restul pachetului și mâna adversarului nu
                let o = me.other().idx();
                let n = g.hands[o].len();
                let mut pool: Vec<u8> = g.hands[o].clone();
                pool.extend(g.stock.iter().skip(1).copied());
                crate::p3_comun::shuffle_with(&mut pool, seed);
                g.hands[o] = pool.drain(..n).collect();
                for (i, c) in pool.into_iter().enumerate() {
                    g.stock[i + 1] = c;
                }
            },
            12,
        );
    }
}
