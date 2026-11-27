//! Motor comun pentru jocurile de curse cu cărți și popice pe inel (Dog, Toc), 2 jucători.
//!
//! * Inel de `ring` câmpuri; fiecare jucător are 4 piese, start-ul lui e la jumătatea inelului față de celălalt.
//!   O piesă urmărește un parcurs: kennel (în afara tablei) → câmpul de start (d = 0) → ocolește tot inelul →
//!   intră în casa proprie (4 câmpuri după ultimul câmp al inelului), unde nu poate fi lovită.
//! * Cărțile (valorile) se joacă din mână: A = 1 sau intrare (în Dog și 11), K = 13 sau intrare, Q = 12,
//!   J = schimb (Dog) sau 11/schimb (Toc), 4 = înapoi (Toc) sau înainte/înapoi (Dog), 7 = șapte pași de câte 1 între
//!   piesele tale, joker = intrare/15 (Toc) sau orice carte (Dog); celelalte cărți = valoarea lor.
//! * O piesă care stă pe câmpul ei de start e protejată: nu poate fi lovită sau schimbată și nu poate fi trecută.
//!   Aterizarea exactă pe adversar îl trimite înapoi în kennel; pe o piesă proprie nu poți ateriza.
//! * Dacă nu ai nicio carte jucabilă, arunci mâna (`pas`). Primul care are toate cele 4 piese în casă câștigă.
//!
//! Simplificări (documentate): cele patru jucători pe echipe din regulile originale sunt reduse la doi jucători;
//! cu 7 poți pierde pașii rămași dacă nu mai există mutări; nu se trage carte suplimentară cu jokerul.
//! Informație ascunsă: mâna adversarului și pachetul; AI-ul citește doar propria mână.

use crate::cards::{shuffle, Viewer};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone, Copy, Debug)]
pub struct Rules {
    pub ring: usize,
    pub hands: &'static [usize],
    pub decks: u8,
    pub jokers: u8,
    pub four_both_ways: bool,
    pub ace_eleven: bool,
    pub joker_wild: bool,
    pub jack_moves: bool,
    pub name: &'static str,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Act {
    Enter,
    Fwd(u8),
    Fwd11(u8),
    Back(u8),
    Fwd15(u8),
    Swap(u8, u8),
    Seven(u8),
    SevenStep(u8),
    SevenEnd,
    Fold,
    Deal,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DMove {
    /// cartea din mână (0 = joker)
    pub src: u8,
    /// valoarea efectivă (relevantă la joker)
    pub eff: u8,
    pub act: Act,
}

#[derive(Clone)]
pub struct DogGame {
    rules: Rules,
    rng: Rng,
    deck: Vec<u8>,
    hands: [Vec<u8>; 2],
    pc: [[i16; 4]; 2],
    side: Player,
    round: usize,
    seven_left: u8,
    first: Player,
    viewer: Viewer,
    log: String,
}

fn label(c: u8) -> String {
    match c {
        0 => "jk".into(),
        1 => "A".into(),
        11 => "J".into(),
        12 => "Q".into(),
        13 => "K".into(),
        n => n.to_string(),
    }
}

impl DogGame {
    pub fn with_seed(rules: Rules, seed: u64) -> Self {
        let mut g = DogGame {
            rules,
            rng: Rng::new(seed),
            deck: Vec::new(),
            hands: [Vec::new(), Vec::new()],
            pc: [[-1; 4]; 2],
            side: Player::One,
            round: 0,
            seven_left: 0,
            first: Player::One,
            viewer: Viewer::Fixed(Player::One),
            log: String::new(),
        };
        g.deal();
        g
    }

    fn r(&self) -> i16 {
        self.rules.ring as i16
    }

    fn start(&self, p: usize) -> i16 {
        (p * self.rules.ring / 2) as i16
    }

    fn deal(&mut self) {
        let n = self.rules.hands[self.round % self.rules.hands.len()];
        if self.deck.len() < 2 * n {
            let mut d: Vec<u8> = Vec::new();
            for _ in 0..self.rules.decks {
                for rank in 1..=13u8 {
                    for _ in 0..4 {
                        d.push(rank);
                    }
                }
            }
            d.extend(std::iter::repeat_n(0, self.rules.jokers as usize));
            shuffle(&mut d, &mut self.rng);
            self.deck = d;
        }
        for p in 0..2 {
            let mut h: Vec<u8> = (0..n).map(|_| self.deck.pop().unwrap()).collect();
            h.sort_unstable();
            self.hands[p] = h;
        }
        self.round += 1;
        self.seven_left = 0;
        self.side = self.first;
        self.first = self.first.other();
    }

    #[cfg(test)]
    pub fn reverse_opp_for_test(&mut self) {
        self.hands[1].reverse();
        self.deck.reverse();
    }

    /// Pentru teste (audit AI): re-amestecă tot ce nu vede `me` (mâna adversarului și pachetul).
    #[cfg(test)]
    pub fn scramble_hidden_for_test(&mut self, me: Player, seed: u64) {
        let o = 1 - me.idx();
        let n = self.hands[o].len();
        let mut pool: Vec<u8> = self.hands[o].clone();
        pool.extend(self.deck.iter().copied());
        crate::p3_comun::shuffle_with(&mut pool, seed);
        self.hands[o] = pool.drain(..n).collect();
        self.hands[o].sort_unstable();
        self.deck = pool;
    }

    /// Pentru teste: mâna lui J1, mâna lui J2, piesele lui J2 și ale lui J1 (în această ordine pe ultimul parametru).
    #[cfg(test)]
    pub fn test_setup(&mut self, h1: &[u8], h2: &[u8], p2: &[i16; 4], p1: [i16; 4]) {
        self.hands = [h1.to_vec(), h2.to_vec()];
        self.pc = [p1, *p2];
        self.side = Player::One;
        self.seven_left = 0;
    }

    pub fn home_count(&self, p: Player) -> usize {
        self.pc[p.idx()].iter().filter(|&&d| d >= self.r()).count()
    }

    fn abs(&self, p: usize, d: i16) -> i16 {
        (self.start(p) + d).rem_euclid(self.r())
    }

    /// Cine stă pe pătratul absolut `a` al inelului: (jucător, piesă, d).
    fn at(&self, a: i16) -> Option<(usize, usize, i16)> {
        for p in 0..2 {
            for i in 0..4 {
                let d = self.pc[p][i];
                if (0..self.r()).contains(&d) && self.abs(p, d) == a {
                    return Some((p, i, d));
                }
            }
        }
        None
    }

    /// Rezultatul mutării unei piese `n` pași (înainte sau înapoi); `Some(Option<captură>)` dacă e legală.
    fn walk(
        &self,
        p: usize,
        i: usize,
        n: i16,
        back: bool,
    ) -> Option<(i16, Option<(usize, usize)>)> {
        let d0 = self.pc[p][i];
        if d0 < 0 || n <= 0 {
            return None;
        }
        let r = self.r();
        if back {
            if d0 >= r {
                return None;
            }
            let mut cap = None;
            let mut d = d0;
            for k in 1..=n {
                d -= 1;
                if d < 0 {
                    d += r;
                }
                let a = self.abs(p, d);
                if let Some((q, j, dq)) = self.at(a) {
                    if (q, j) == (p, i) {
                        continue;
                    }
                    let protected = dq == 0;
                    if k < n && protected {
                        return None;
                    }
                    if k == n {
                        if q == p || protected {
                            return None;
                        }
                        cap = Some((q, j));
                    }
                }
            }
            return Some((d, cap));
        }
        let mut cap = None;
        for k in 1..=n {
            let d = d0 + k;
            if d >= r {
                if d > r + 3 {
                    return None;
                }
                // casa: fără piese proprii în cale sau la destinație
                if self.pc[p].contains(&d) {
                    return None;
                }
                if d0 < r || k < n {
                    continue;
                }
                continue;
            }
            let a = self.abs(p, d);
            if let Some((q, j, dq)) = self.at(a) {
                if (q, j) == (p, i) {
                    continue;
                }
                let protected = dq == 0;
                if k < n && protected {
                    return None;
                }
                if k == n {
                    if q == p || protected {
                        return None;
                    }
                    cap = Some((q, j));
                }
            }
        }
        Some((d0 + n, cap))
    }

    fn can_enter(&self, p: usize) -> Option<usize> {
        let free = (0..4).find(|&i| self.pc[p][i] == -1)?;
        let a = self.start(p);
        match self.at(a) {
            Some((q, _, _)) if q == p => None,
            _ => Some(free),
        }
    }

    fn eff_moves(&self, src: u8, eff: u8, out: &mut Vec<DMove>) {
        let p = self.side.idx();
        let mut push = |act: Act| out.push(DMove { src, eff, act });
        let enter = matches!(eff, 1 | 13) || (src == 0 && !self.rules.joker_wild && eff == 15);
        if enter && self.can_enter(p).is_some() {
            push(Act::Enter);
        }
        for i in 0..4u8 {
            let iu = i as usize;
            match eff {
                1 => {
                    if self.walk(p, iu, 1, false).is_some() {
                        push(Act::Fwd(i));
                    }
                    if self.rules.ace_eleven && self.walk(p, iu, 11, false).is_some() {
                        push(Act::Fwd11(i));
                    }
                }
                4 => {
                    if self.walk(p, iu, 4, true).is_some() {
                        push(Act::Back(i));
                    }
                    if self.rules.four_both_ways && self.walk(p, iu, 4, false).is_some() {
                        push(Act::Fwd(i));
                    }
                }
                7 => {
                    if self.walk(p, iu, 1, false).is_some() {
                        push(Act::Seven(i));
                    }
                }
                11 => {
                    if self.rules.jack_moves && self.walk(p, iu, 11, false).is_some() {
                        push(Act::Fwd(i));
                    }
                }
                15 if self.walk(p, iu, 15, false).is_some() => push(Act::Fwd15(i)),
                n @ (2 | 3 | 5 | 6 | 8 | 9 | 10 | 12 | 13)
                    if self.walk(p, iu, n as i16, false).is_some() =>
                {
                    push(Act::Fwd(i))
                }
                _ => {}
            }
        }
        if eff == 11 {
            for a in 0..4u8 {
                let da = self.pc[p][a as usize];
                if !(1..self.r()).contains(&da) {
                    continue;
                }
                for b in 0..4u8 {
                    let db = self.pc[1 - p][b as usize];
                    if (1..self.r()).contains(&db) {
                        push(Act::Swap(a, b));
                    }
                }
            }
        }
    }

    fn gen(&self) -> Vec<DMove> {
        let mut out = Vec::new();
        let hand = &self.hands[self.side.idx()];
        let mut seen: Vec<u8> = Vec::new();
        for &c in hand {
            if seen.contains(&c) {
                continue;
            }
            seen.push(c);
            if c == 0 {
                if self.rules.joker_wild {
                    for eff in 1..=13u8 {
                        self.eff_moves(0, eff, &mut out);
                    }
                } else {
                    self.eff_moves(0, 15, &mut out);
                }
            } else {
                self.eff_moves(c, c, &mut out);
            }
        }
        out
    }

    fn end_turn(&mut self) {
        let o = self.side.other();
        if !self.hands[o.idx()].is_empty() {
            self.side = o;
        } else if self.hands[self.side.idx()].is_empty() {
            // runda s-a terminat: se împarte din nou (mutare `imparte`)
        }
    }

    fn progress(&self, p: usize) -> i32 {
        self.pc[p]
            .iter()
            .map(|&d| {
                if d < 0 {
                    0
                } else if d >= self.r() {
                    self.r() as i32 + 8 + (d - self.r()) as i32
                } else {
                    d as i32 + 6
                }
            })
            .sum()
    }
}

impl Game for DogGame {
    type Move = DMove;

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = match who {
            Some(p) => Viewer::Fixed(p),
            None => Viewer::HotSeat,
        };
    }

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<DMove> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        let dm = |act| DMove {
            src: 0,
            eff: 0,
            act,
        };
        if self.seven_left > 0 {
            let p = self.side.idx();
            let mut v: Vec<DMove> = (0..4u8)
                .filter(|&i| self.walk(p, i as usize, 1, false).is_some())
                .map(|i| dm(Act::SevenStep(i)))
                .collect();
            if v.is_empty() {
                v.push(dm(Act::SevenEnd));
            }
            return v;
        }
        if self.hands[0].is_empty() && self.hands[1].is_empty() {
            return vec![dm(Act::Deal)];
        }
        let v = self.gen();
        if v.is_empty() {
            vec![dm(Act::Fold)]
        } else {
            v
        }
    }

    fn apply(&mut self, m: &DMove) {
        self.log.clear();
        let p = self.side.idx();
        let remove_card = |g: &mut DogGame| {
            let pos = g.hands[p].iter().position(|&x| x == m.src).unwrap();
            g.hands[p].remove(pos);
        };
        let mv = |g: &mut DogGame, i: u8, n: i16, back: bool| {
            if let Some((d, cap)) = g.walk(p, i as usize, n, back) {
                g.pc[p][i as usize] = d;
                if let Some((q, j)) = cap {
                    g.pc[q][j] = -1;
                    g.log = format!("J{} lovește o piesă a lui J{}.", p + 1, q + 1);
                }
            }
        };
        match m.act {
            Act::Deal => {
                self.deal();
                return;
            }
            Act::Fold => {
                self.hands[p].clear();
                self.log = format!("J{} nu are carte jucabilă și aruncă mâna.", p + 1);
            }
            Act::Enter => {
                remove_card(self);
                let i = self.can_enter(p).unwrap();
                let a = self.start(p);
                if let Some((q, j, _)) = self.at(a) {
                    self.pc[q][j] = -1;
                    self.log = format!("J{} intră și lovește o piesă a lui J{}.", p + 1, q + 1);
                }
                self.pc[p][i] = 0;
            }
            Act::Fwd(i) => {
                remove_card(self);
                let n = if m.eff == 11 { 11 } else { m.eff as i16 };
                mv(self, i, n, false);
            }
            Act::Fwd11(i) => {
                remove_card(self);
                mv(self, i, 11, false);
            }
            Act::Fwd15(i) => {
                remove_card(self);
                mv(self, i, 15, false);
            }
            Act::Back(i) => {
                remove_card(self);
                mv(self, i, 4, true);
            }
            Act::Swap(a, b) => {
                remove_card(self);
                let (da, db) = (self.pc[p][a as usize], self.pc[1 - p][b as usize]);
                let (xa, xb) = (self.abs(p, da), self.abs(1 - p, db));
                self.pc[p][a as usize] = (xb - self.start(p)).rem_euclid(self.r());
                self.pc[1 - p][b as usize] = (xa - self.start(1 - p)).rem_euclid(self.r());
                self.log = format!("J{} schimbă două piese.", p + 1);
            }
            Act::Seven(i) => {
                remove_card(self);
                mv(self, i, 1, false);
                self.seven_left = 6;
                if self.seven_left > 0 {
                    return;
                }
            }
            Act::SevenStep(i) => {
                mv(self, i, 1, false);
                self.seven_left -= 1;
                if self.seven_left > 0 {
                    return;
                }
            }
            Act::SevenEnd => {
                self.seven_left = 0;
            }
        }
        self.end_turn();
    }

    fn outcome(&self) -> Option<Outcome> {
        for p in [Player::One, Player::Two] {
            if self.home_count(p) == 4 {
                return Some(Outcome::Win(p));
            }
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let (a, b) = (p.idx(), 1 - p.idx());
        let mut v = (self.progress(a) - self.progress(b)) * 10;
        let hand = &self.hands[a];
        if self.pc[a].contains(&-1) {
            v += hand.iter().filter(|&&c| matches!(c, 0 | 1 | 13)).count() as i32 * 6;
        }
        v
    }

    fn render(&self) -> String {
        let r = self.rules.ring;
        let mut ring: Vec<String> = vec![".".into(); r];
        for p in 0..2 {
            for i in 0..4 {
                let d = self.pc[p][i];
                if (0..self.r()).contains(&d) {
                    ring[self.abs(p, d) as usize] =
                        format!("{}{}", if p == 0 { 'X' } else { 'O' }, i + 1);
                }
            }
        }
        let mut s = format!(
            "{} — inel de {r} câmpuri (X = J1, O = J2; start J1 la 0, J2 la {})\n",
            self.rules.name,
            r / 2
        );
        let occupied: Vec<String> = ring
            .iter()
            .enumerate()
            .filter(|(_, c)| *c != ".")
            .map(|(i, c)| format!("{i}:{c}"))
            .collect();
        s.push_str(&format!(
            "Pe inel: {}\n",
            if occupied.is_empty() {
                "—".into()
            } else {
                occupied.join(" ")
            }
        ));
        for p in 0..2 {
            let st: Vec<String> = self.pc[p]
                .iter()
                .enumerate()
                .map(|(i, &d)| {
                    if d < 0 {
                        format!("p{}:kennel", i + 1)
                    } else if d >= self.r() {
                        format!("p{}:casă", i + 1)
                    } else {
                        format!("p{}:d{}", i + 1, d)
                    }
                })
                .collect();
            s.push_str(&format!("J{}: {}\n", p + 1, st.join(" ")));
        }
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        let v = self.viewer.who(self.side);
        let hand: Vec<String> = self.hands[v.idx()].iter().map(|&c| label(c)).collect();
        s.push_str(&format!(
            "Mâna ta (J{}): {}\n",
            v.idx() + 1,
            if hand.is_empty() {
                "—".into()
            } else {
                hand.join(" ")
            }
        ));
        s.push_str(&format!(
            "J{} are {} cărți\n",
            v.other().idx() + 1,
            self.hands[v.other().idx()].len()
        ));
        if self.seven_left > 0 {
            s.push_str(&format!(
                "Șapte: mai ai {} pași de câte 1.\n",
                self.seven_left
            ));
        }
        match self.outcome() {
            Some(Outcome::Win(w)) => s.push_str(&format!("Câștigă J{}.\n", w.idx() + 1)),
            _ => s.push_str(&format!("La mutare: J{}\n", self.side.idx() + 1)),
        }
        s
    }

    fn format_move(&self, m: &DMove) -> String {
        let card = if m.src == 0 && self.rules.joker_wild && !matches!(m.act, Act::Fold | Act::Deal)
        {
            format!("jk={}", label(m.eff))
        } else {
            label(m.src)
        };
        match m.act {
            Act::Deal => "imparte".into(),
            Act::Fold => "pas".into(),
            Act::Enter => format!("{card}:intra"),
            Act::Fwd(i) => format!("{card}:p{}", i + 1),
            Act::Fwd11(i) => format!("{card}:p{}/11", i + 1),
            Act::Fwd15(i) => format!("{card}:p{}", i + 1),
            Act::Back(i) => {
                if self.rules.four_both_ways {
                    format!("{card}:p{}-", i + 1)
                } else {
                    format!("{card}:p{}", i + 1)
                }
            }
            Act::Swap(a, b) => format!("{card}:p{}<>o{}", a + 1, b + 1),
            Act::Seven(i) => format!("{card}:p{}", i + 1),
            Act::SevenStep(i) => format!("pas1:p{}", i + 1),
            Act::SevenEnd => "gata7".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        // Adâncimea 0 = o singură mutare la TOATE nivelurile: la Hard/Expert adâncimea 1 ar extinde
        // răspunsurile adversarului din mâna lui reală (informație ascunsă).
        AiStyle::AlphaBeta(0)
    }
}
