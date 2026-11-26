#![allow(
    clippy::needless_range_loop,
    clippy::manual_is_multiple_of,
    clippy::manual_range_contains,
    clippy::too_many_arguments,
    clippy::doc_lazy_continuation,
    clippy::unnecessary_unwrap,
    clippy::unnecessary_cast,
    clippy::type_complexity
)]
//! Tiến lên (Vietnam) — joc de eliminare a cărților cu „tăieri” (chặt), 2 jucători.
//!
//! Reguli folosite (varianta de bază sudică, adaptată pentru doi):
//! * 52 de cărți, 13 fiecăruia; celelalte 26 rămân nefolosite și ascunse. Rangul: 3 < 4 < … < K
//!   < A < 2; culoarea: ♠ < ♣ < ♦ < ♥ (cupa e cea mai mare).
//! * Combinații: carte singură, pereche, treime, careu (tứ quý), scară de cel puțin 3 ranguri
//!   consecutive (fără 2) și „perechi consecutive” (đôi thông) de cel puțin 3 perechi (fără 2).
//!   Se răspunde cu aceeași formă și același număr de cărți, cu cartea cea mai mare mai mare
//!   (rang, apoi culoare).
//! * Tăieri (chặt) ale cărților de 2: pe un 2 singur — careu sau 3+ perechi consecutive; pe o
//!   pereche de 2 — careu sau 4+ perechi consecutive; pe un careu și pe 3 perechi consecutive —
//!   4+ perechi consecutive (3 perechi consecutive sunt tăiate și de un careu).
//! * Cine nu poate / nu vrea spune `pas`; în doi, după pas adversarul conduce din nou. Prima mână:
//!   cine are cea mai mică carte conduce și o include; apoi câștigătorul mâinii.
//! * Victorie instantanee (tới trắng) la împărțire: patru de 2, șase perechi sau scară completă
//!   3…A (12 ranguri).
//! * Meci: primul la 3 mâini câștigate. Omise: penalizarea „2 rămas”, 3–4 jucători,
//!   pierderea dreptului de a juca după pas, tăierea cu mai multă miză.
//! * Mutări: cărți separate prin spațiu: `3s`, `9h 9d`, `5c 6d 7h` (scară; culorile cărților
//!   interioare se aleg automat — cele mai slabe; tu alegi culoarea cărții cele mai mari).
//! * Informație ascunsă: vezi doar mâna ta; AI-ul citește doar mâna proprie și cărțile jucate
//!   (Hard/Expert: eșantionare Monte-Carlo a cărților necunoscute).

use crate::cards::Viewer;
use crate::cards52::{std_deck, std_rank, std_suit, std_symbol, std_text};
use crate::p2_util::{mc_samples, mc_value, noise, shuffle_vec, sig_of, Mc};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const TARGET: u8 = 3;

pub fn strength(c: u8) -> u8 {
    (std_rank(c) + 12) % 13
}
/// ♠ < ♣ < ♦ < ♥.
pub fn suit_ord(c: u8) -> u8 {
    [1u8, 2, 3, 0][std_suit(c) as usize]
}
pub fn card_key(c: u8) -> u32 {
    strength(c) as u32 * 4 + suit_ord(c) as u32
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Single,
    Pair,
    Triple,
    Quad,
    Seq,
    PairSeq,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Combo {
    pub cards: Vec<u8>,
    pub kind: Kind,
    /// Numărul de ranguri distincte (lungimea scării).
    pub len: u8,
    pub key: u32,
}

impl Combo {
    fn rank(&self) -> u8 {
        (self.key / 4) as u8
    }
    pub fn beats(&self, o: &Combo) -> bool {
        if self.kind == o.kind && self.len == o.len {
            return self.key > o.key;
        }
        let ps = |c: &Combo, n: u8| c.kind == Kind::PairSeq && c.len >= n;
        match (o.kind, o.len) {
            (Kind::Single, _) if o.rank() == 12 => self.kind == Kind::Quad || ps(self, 3),
            (Kind::Pair, _) if o.rank() == 12 => self.kind == Kind::Quad || ps(self, 4),
            (Kind::Quad, _) => ps(self, 4),
            (Kind::PairSeq, 3) => self.kind == Kind::Quad || ps(self, 4),
            _ => false,
        }
    }
    pub fn text(&self) -> String {
        self.cards
            .iter()
            .map(|&c| std_text(c))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

fn mk(cards: Vec<u8>, kind: Kind, len: u8) -> Combo {
    let key = cards.iter().map(|&c| card_key(c)).max().unwrap();
    let mut cards = cards;
    cards.sort_by_key(|&c| card_key(c));
    Combo {
        cards,
        kind,
        len,
        key,
    }
}

pub fn gen_combos(hand: &[u8]) -> Vec<Combo> {
    let mut by: Vec<Vec<u8>> = vec![Vec::new(); 13];
    for &c in hand {
        by[strength(c) as usize].push(c);
    }
    for v in &mut by {
        v.sort_by_key(|&c| card_key(c));
    }
    let mut out = Vec::new();
    for v in &by {
        let n = v.len();
        for i in 0..n {
            out.push(mk(vec![v[i]], Kind::Single, 1));
            for j in (i + 1)..n {
                out.push(mk(vec![v[i], v[j]], Kind::Pair, 1));
                for k in (j + 1)..n {
                    out.push(mk(vec![v[i], v[j], v[k]], Kind::Triple, 1));
                }
            }
        }
        if n == 4 {
            out.push(mk(v.clone(), Kind::Quad, 1));
        }
    }
    // scări (fără 2)
    for s in 0..12usize {
        let mut l = 0usize;
        while s + l < 12 && !by[s + l].is_empty() {
            l += 1;
            if l >= 3 {
                let top = s + l - 1;
                for &tc in &by[top] {
                    let mut cs: Vec<u8> = (s..top).map(|r| by[r][0]).collect();
                    cs.push(tc);
                    out.push(mk(cs, Kind::Seq, l as u8));
                }
            }
        }
        let mut l = 0usize;
        while s + l < 12 && by[s + l].len() >= 2 {
            l += 1;
            if l >= 3 {
                let top = s + l - 1;
                let t = &by[top];
                for i in 0..t.len() {
                    for j in (i + 1)..t.len() {
                        let mut cs: Vec<u8> = Vec::new();
                        for r in s..top {
                            cs.push(by[r][0]);
                            cs.push(by[r][1]);
                        }
                        cs.push(t[i]);
                        cs.push(t[j]);
                        out.push(mk(cs, Kind::PairSeq, l as u8));
                    }
                }
            }
        }
    }
    out
}

/// Victorie instantanee la împărțire (tới trắng): descriere dacă mâna o are.
pub fn instant_win(hand: &[u8]) -> Option<&'static str> {
    let mut cnt = [0u8; 13];
    for &c in hand {
        cnt[strength(c) as usize] += 1;
    }
    if cnt[12] == 4 {
        return Some("patru de 2");
    }
    if cnt[..12].iter().all(|&x| x >= 1) {
        return Some("scară completă 3…A");
    }
    if cnt.iter().map(|&x| (x / 2) as u32).sum::<u32>() >= 6 {
        return Some("șase perechi");
    }
    None
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Play(Combo),
    Pass,
}

#[derive(Clone)]
pub struct TienLen {
    hands: [Vec<u8>; 2],
    stock: Vec<u8>,
    played: Vec<u8>,
    field: Option<Combo>,
    holder: Player,
    side: Player,
    must: Option<u8>,
    wins: [u8; 2],
    hand_no: u8,
    over: bool,
    last: String,
    rng: Rng,
    viewer: Viewer,
}

impl Default for TienLen {
    fn default() -> Self {
        Self::new()
    }
}

impl TienLen {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = TienLen {
            hands: [Vec::new(), Vec::new()],
            stock: Vec::new(),
            played: Vec::new(),
            field: None,
            holder: Player::One,
            side: Player::One,
            must: None,
            wins: [0; 2],
            hand_no: 1,
            over: false,
            last: String::new(),
            rng: Rng::new(seed),
            viewer: Viewer::Fixed(Player::One),
        };
        g.deal(None);
        g
    }

    /// Împarte; cu victorii instantanee se numără și se reîmparte.
    fn deal(&mut self, mut leader: Option<Player>) {
        for _ in 0..50 {
            let mut d = std_deck(&mut self.rng);
            shuffle_vec(&mut d, &mut self.rng);
            self.hands = [Vec::new(), Vec::new()];
            for _ in 0..13 {
                self.hands[0].push(d.pop().unwrap());
                self.hands[1].push(d.pop().unwrap());
            }
            for h in &mut self.hands {
                h.sort_by_key(|&c| card_key(c));
            }
            self.stock = d;
            self.played.clear();
            self.field = None;
            let (a, b) = (instant_win(&self.hands[0]), instant_win(&self.hands[1]));
            let winner = match (a, b) {
                (Some(w), None) => Some((Player::One, w)),
                (None, Some(w)) => Some((Player::Two, w)),
                _ => None,
            };
            if let Some((p, why)) = winner {
                self.wins[p.idx()] += 1;
                self.last = format!("J{} câștigă instantaneu ({why})", p.idx() + 1);
                if self.wins[p.idx()] >= TARGET {
                    self.over = true;
                    return;
                }
                self.hand_no += 1;
                leader = Some(p);
                continue;
            }
            break;
        }
        match leader {
            Some(p) => {
                self.side = p;
                self.must = None;
            }
            None => {
                let (l0, l1) = (card_key(self.hands[0][0]), card_key(self.hands[1][0]));
                let (p, c) = if l0 < l1 {
                    (Player::One, self.hands[0][0])
                } else {
                    (Player::Two, self.hands[1][0])
                };
                self.side = p;
                self.must = Some(c);
            }
        }
        self.holder = self.side;
    }

    pub fn wins(&self) -> (u8, u8) {
        (self.wins[0], self.wins[1])
    }
    pub fn set_hands_for_test(&mut self, a: Vec<u8>, b: Vec<u8>) {
        self.hands = [a, b];
        for h in &mut self.hands {
            h.sort_by_key(|&c| card_key(c));
        }
        self.must = None;
    }
}

impl Mc for TienLen {
    fn determinize(&self, viewer: Player, rng: &mut Rng) -> Self {
        let mut g = self.clone();
        let (me, op) = (viewer.idx(), viewer.other().idx());
        let mut pool: Vec<u8> = (0..52u8)
            .filter(|c| !self.hands[me].contains(c) && !self.played.contains(c))
            .collect();
        shuffle_vec(&mut pool, rng);
        let n = self.hands[op].len();
        g.hands[op] = pool.drain(..n).collect();
        g.hands[op].sort_by_key(|&c| card_key(c));
        g.stock = pool;
        g.rng = Rng::new(rng.next_u64());
        g
    }
    fn epoch(&self) -> u32 {
        self.hand_no as u32
    }
    fn margin(&self, p: Player) -> i32 {
        10 * (self.wins[p.idx()] as i32 - self.wins[p.other().idx()] as i32)
    }
}

impl Game for TienLen {
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
        if self.over {
            return Vec::new();
        }
        let mut v: Vec<Move> = Vec::new();
        for c in gen_combos(&self.hands[self.side.idx()]) {
            if let Some(m) = self.must {
                if !c.cards.contains(&m) {
                    continue;
                }
            }
            if let Some(f) = &self.field {
                if !c.beats(f) {
                    continue;
                }
            }
            v.push(Move::Play(c));
        }
        if self.field.is_some() {
            v.push(Move::Pass);
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        let me = self.side;
        match m {
            Move::Pass => {
                self.field = None;
                self.side = self.holder;
                self.last = format!("J{} dă pas", me.idx() + 1);
            }
            Move::Play(c) => {
                self.hands[me.idx()].retain(|x| !c.cards.contains(x));
                self.played.extend(c.cards.iter().copied());
                let chop = self
                    .field
                    .as_ref()
                    .is_some_and(|f| f.kind != c.kind || f.len != c.len);
                self.last = format!(
                    "J{} joacă {}{}",
                    me.idx() + 1,
                    c.text(),
                    if chop { " (taie)" } else { "" }
                );
                self.field = Some(c.clone());
                self.holder = me;
                self.must = None;
                if self.hands[me.idx()].is_empty() {
                    self.wins[me.idx()] += 1;
                    self.last
                        .push_str(&format!(" — J{} câștigă mâna", me.idx() + 1));
                    if self.wins[me.idx()] >= TARGET {
                        self.over = true;
                    } else {
                        self.hand_no += 1;
                        self.deal(Some(me));
                    }
                    return;
                }
                self.side = me.other();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            return None;
        }
        Some(if self.wins[0] > self.wins[1] {
            Outcome::Win(Player::One)
        } else {
            Outcome::Win(Player::Two)
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let (me, op) = (p.idx(), p.other().idx());
        let mut s = 1000 * (self.wins[me] as i32 - self.wins[op] as i32);
        if self.over {
            return s;
        }
        let h = &self.hands[me];
        let mut seen = [false; 13];
        for &c in h {
            seen[strength(c) as usize] = true;
        }
        let turns = seen.iter().filter(|&&x| x).count() as i32;
        let twos = h.iter().filter(|&&c| strength(c) == 12).count() as i32;
        let aces = h.iter().filter(|&&c| strength(c) == 11).count() as i32;
        s += 30 * (self.hands[op].len() as i32 - h.len() as i32);
        s -= 35 * turns;
        s += 25 * twos + 10 * aces;
        if self.field.is_none() && self.side == p {
            s += 25;
        }
        let sig = sig_of(&[
            h.iter()
                .fold(0u64, |a, &c| a.wrapping_mul(53).wrapping_add(c as u64 + 1)),
            self.played.len() as u64,
            self.wins[0] as u64,
        ]);
        s += noise(sig, 80);
        s + mc_value(self, p, mc_samples(), sig)
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Tiến lên — mâna {} | victorii: J1 {}, J2 {} (până la {})\n",
            self.hand_no, self.wins[0], self.wins[1], TARGET
        );
        if !self.last.is_empty() {
            s.push_str(&format!("({})\n", self.last));
        }
        if self.over {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        match &self.field {
            Some(c) => s.push_str(&format!(
                "Câmp: {}\n",
                c.cards
                    .iter()
                    .map(|&x| std_symbol(x))
                    .collect::<Vec<_>>()
                    .join(" ")
            )),
            None => s.push_str("Câmp: gol (conduci)\n"),
        }
        let v = self.viewer.who(self.side);
        s.push_str(&format!(
            "Mâna ta (J{}): {}\n",
            v.idx() + 1,
            self.hands[v.idx()]
                .iter()
                .map(|&c| format!("{}[{}]", std_symbol(c), std_text(c)))
                .collect::<Vec<_>>()
                .join(" ")
        ));
        s.push_str(&format!(
            "Jucătorul {}: {} cărți\n",
            v.other().idx() + 1,
            self.hands[v.other().idx()].len()
        ));
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Pass => "pas".into(),
            Move::Play(c) => c.text(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(rank: u8, suit: char) -> u8 {
        let r0 = if rank == 15 { 0 } else { rank - 2 };
        let s = match suit {
            'c' => 0,
            'd' => 1,
            'h' => 2,
            _ => 3,
        };
        s * 13 + r0
    }
    fn of(v: &[(u8, char)]) -> Vec<u8> {
        v.iter().map(|&(r, s)| c(r, s)).collect()
    }
    fn find(h: &[u8], kind: Kind, len: u8, top: u8) -> Combo {
        gen_combos(h)
            .into_iter()
            .find(|x| {
                x.kind == kind
                    && x.len == len
                    && x.key / 4 == if top == 15 { 12 } else { top as u32 - 3 }
            })
            .expect("combinație")
    }

    #[test]
    fn suit_order_spades_lowest_hearts_highest() {
        assert!(card_key(c(3, 'h')) > card_key(c(3, 'd')));
        assert!(card_key(c(3, 'd')) > card_key(c(3, 'c')));
        assert!(card_key(c(3, 'c')) > card_key(c(3, 's')));
        assert!(card_key(c(15, 's')) > card_key(c(14, 'h')));
    }

    #[test]
    fn combos_generated() {
        let h = of(&[
            (3, 'd'),
            (4, 'c'),
            (4, 'h'),
            (5, 'h'),
            (5, 's'),
            (6, 'd'),
            (6, 'c'),
        ]);
        let cs = gen_combos(&h);
        assert!(cs.iter().any(|x| x.kind == Kind::Seq && x.len == 4)); // 3-4-5-6
        assert!(cs.iter().any(|x| x.kind == Kind::PairSeq && x.len == 3)); // 44 55 66
        assert!(cs.iter().filter(|x| x.kind == Kind::Pair).count() == 3);
        assert!(!cs.iter().any(|x| x.kind == Kind::Triple));
        // scara nu conține 2
        let h2 = of(&[(13, 'd'), (14, 'c'), (15, 'h')]);
        assert!(!gen_combos(&h2).iter().any(|x| x.kind == Kind::Seq));
    }

    #[test]
    fn chop_rules() {
        let two = Combo {
            cards: vec![c(15, 's')],
            kind: Kind::Single,
            len: 1,
            key: card_key(c(15, 's')),
        };
        let quad = mk(of(&[(5, 'c'), (5, 'd'), (5, 'h'), (5, 's')]), Kind::Quad, 1);
        let ps3 = mk(
            of(&[(3, 'c'), (3, 'd'), (4, 'c'), (4, 'd'), (5, 'c'), (5, 'd')]),
            Kind::PairSeq,
            3,
        );
        let ps4 = mk(
            of(&[
                (3, 'c'),
                (3, 'd'),
                (4, 'c'),
                (4, 'd'),
                (5, 'c'),
                (5, 'd'),
                (6, 'c'),
                (6, 'd'),
            ]),
            Kind::PairSeq,
            4,
        );
        let pair2 = mk(of(&[(15, 's'), (15, 'c')]), Kind::Pair, 1);
        assert!(quad.beats(&two) && ps3.beats(&two) && ps4.beats(&two));
        assert!(quad.beats(&pair2) && ps4.beats(&pair2) && !ps3.beats(&pair2));
        assert!(ps4.beats(&quad) && !quad.beats(&ps4) && !ps3.beats(&quad));
        assert!(quad.beats(&ps3) && ps4.beats(&ps3));
        // un 3 nu taie 2
        let three = mk(of(&[(3, 'c')]), Kind::Single, 1);
        assert!(!three.beats(&two));
        assert!(two.beats(&three));
        // pereche de 2 nu e tăiată de pereche mai mică
        let low_pair = mk(of(&[(9, 'c'), (9, 'd')]), Kind::Pair, 1);
        assert!(!low_pair.beats(&pair2));
    }

    #[test]
    fn instant_wins_detected() {
        assert_eq!(
            instant_win(&of(&[(15, 'c'), (15, 'd'), (15, 'h'), (15, 's'), (3, 'c')])),
            Some("patru de 2")
        );
        let dragon: Vec<(u8, char)> = (3..=14).map(|r| (r, 'c')).collect();
        assert_eq!(instant_win(&of(&dragon)), Some("scară completă 3…A"));
        let sp = of(&[
            (3, 'c'),
            (3, 'd'),
            (4, 'c'),
            (4, 'd'),
            (6, 'c'),
            (6, 'd'),
            (8, 'c'),
            (8, 'd'),
            (10, 'c'),
            (10, 'd'),
            (12, 'c'),
            (12, 'd'),
            (14, 'h'),
        ]);
        assert_eq!(instant_win(&sp), Some("șase perechi"));
        assert_eq!(instant_win(&of(&[(3, 'c'), (5, 'd')])), None);
    }

    #[test]
    fn pass_gives_lead_and_hand_win_counts() {
        let mut g = TienLen::with_seed(1);
        g.set_hands_for_test(of(&[(9, 'h'), (5, 'c')]), of(&[(10, 'h'), (4, 'd')]));
        g.side = Player::One;
        g.holder = Player::One;
        g.apply(&Move::Play(find(&g.hands[0].clone(), Kind::Single, 1, 9)));
        let mv = Game::legal_moves(&g);
        assert!(mv.contains(&Move::Pass));
        g.apply(&Move::Pass);
        assert_eq!(g.to_move(), Player::One);
        g.apply(&Move::Play(find(&g.hands[0].clone(), Kind::Single, 1, 5)));
        assert_eq!(g.wins(), (1, 0));
        assert_eq!(g.hand_no, 2);
    }

    #[test]
    fn first_move_includes_lowest_card() {
        for seed in 1..20 {
            let g = TienLen::with_seed(seed);
            if g.over || g.must.is_none() {
                continue;
            }
            let low = g.must.unwrap();
            for m in Game::legal_moves(&g) {
                if let Move::Play(cb) = m {
                    assert!(cb.cards.contains(&low));
                }
            }
        }
    }

    #[test]
    fn render_hides_opponent() {
        let g = TienLen::with_seed(4);
        let r = Game::render(&g);
        for &x in &g.hands[1] {
            assert!(!r.contains(&format!("[{}]", std_text(x))), "{r}");
        }
    }

    #[test]
    fn ai_finishes_match() {
        let mut g = TienLen::with_seed(6);
        let mut n = 0;
        while g.outcome().is_none() && n < 6000 {
            let m = polytabletop_core::ai::best_move(&g, n).unwrap();
            g.apply(&m);
            n += 1;
        }
        assert!(g.outcome().is_some());
    }
}
