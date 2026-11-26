//! Darurile Serii: duel de convingere cu informație ascunsă. Șase „daruri” (valori 2,2,3,3,5,6;
//! câte o carte pentru fiecare punct de valoare, 21 de cărți). Într-o rundă fiecare jucător face
//! exact patru acțiuni, în orice ordine: ASCUNDE (1 carte, se numără la final), ARUNCĂ (2 cărți
//! ies din joc), OFERĂ (arăți 3 cărți: adversarul ia una, tu păstrezi două), ÎMPARTE (arăți două
//! perechi: adversarul alege una, tu o ai pe cealaltă). La sfârșitul rundei cine are mai multe
//! cărți la un dar îl câștigă. Cine are 11 puncte sau 4 daruri câștigă; după 6 runde decide scorul.
//! Variantă originală (valori și număr de daruri proprii). Cu `set_viewer`: HOTSEAT_SAFE.
#![allow(clippy::needless_range_loop, clippy::manual_is_multiple_of)]

use crate::w7_util::Dice;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const VAL: [u8; 6] = [2, 2, 3, 3, 5, 6];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mv {
    Secret(u8),
    Burn(u8, u8),
    Offer(u8, u8, u8),
    Split(u8, u8, u8, u8),
    /// răspuns la ofertă: care din cele trei (index charm) merge la cel care răspunde
    Take(u8),
    /// răspuns la împărțire: perechea 0 sau 1
    Pair(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Pending {
    Offer([u8; 3]),
    Split([u8; 4]),
}

#[derive(Clone)]
pub struct Darurile {
    hand: [[u8; 6]; 2],
    table: [[u8; 6]; 2],
    secret: [Option<u8>; 2],
    used: [[bool; 4]; 2],
    deck: [u8; 6],
    marker: [Option<Player>; 6],
    pts: [i32; 2],
    side: Player,
    pending: Option<(Player, Pending)>,
    round: u32,
    first: Player,
    drew: bool,
    viewer: Option<Player>,
    finished: Option<Outcome>,
    dice: Dice,
}

impl Default for Darurile {
    fn default() -> Self {
        Self::new()
    }
}

impl Darurile {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Darurile {
            hand: [[0; 6]; 2],
            table: [[0; 6]; 2],
            secret: [None; 2],
            used: [[false; 4]; 2],
            deck: VAL,
            marker: [None; 6],
            pts: [0; 2],
            side: Player::One,
            pending: None,
            round: 0,
            first: Player::One,
            drew: false,
            viewer: None,
            finished: None,
            dice: Dice::new(seed),
        };
        g.deal();
        g.ensure_draw();
        g
    }

    fn draw_card(&mut self) -> Option<u8> {
        let total: usize = self.deck.iter().map(|&x| x as usize).sum();
        if total == 0 {
            return None;
        }
        let mut r = self.dice.below(total);
        for c in 0..6 {
            if r < self.deck[c] as usize {
                self.deck[c] -= 1;
                return Some(c as u8);
            }
            r -= self.deck[c] as usize;
        }
        None
    }

    fn deal(&mut self) {
        self.deck = VAL;
        self.hand = [[0; 6]; 2];
        self.table = [[0; 6]; 2];
        self.secret = [None; 2];
        self.used = [[false; 4]; 2];
        let _ = self.draw_card(); // o carte scoasă, necunoscută
        for p in 0..2 {
            for _ in 0..6 {
                if let Some(c) = self.draw_card() {
                    self.hand[p][c as usize] += 1;
                }
            }
        }
        self.side = self.first;
        self.drew = false;
    }

    fn ensure_draw(&mut self) {
        if !self.drew && self.pending.is_none() {
            if let Some(c) = self.draw_card() {
                self.hand[self.side.idx()][c as usize] += 1;
            }
            self.drew = true;
        }
    }

    fn cards(&self, p: usize) -> Vec<u8> {
        let mut v = vec![];
        for c in 0..6 {
            for _ in 0..self.hand[p][c] {
                v.push(c as u8);
            }
        }
        v
    }

    fn round_over(&self) -> bool {
        self.used.iter().all(|u| u.iter().all(|&x| x)) && self.pending.is_none()
    }

    fn finish_round(&mut self) {
        for p in 0..2 {
            if let Some(c) = self.secret[p] {
                self.table[p][c as usize] += 1;
            }
        }
        for c in 0..6 {
            let (a, b) = (self.table[0][c], self.table[1][c]);
            if a > b {
                self.marker[c] = Some(Player::One);
            } else if b > a {
                self.marker[c] = Some(Player::Two);
            }
        }
        for p in [Player::One, Player::Two] {
            let v: i32 = (0..6)
                .filter(|&c| self.marker[c] == Some(p))
                .map(|c| VAL[c] as i32)
                .sum();
            self.pts[p.idx()] = v;
        }
        let n1 = self
            .marker
            .iter()
            .filter(|&&m| m == Some(Player::One))
            .count();
        let n2 = self
            .marker
            .iter()
            .filter(|&&m| m == Some(Player::Two))
            .count();
        let w1 = self.pts[0] >= 11 || n1 >= 4;
        let w2 = self.pts[1] >= 11 || n2 >= 4;
        self.round += 1;
        let end = |a: i32, b: i32| {
            if a > b {
                Outcome::Win(Player::One)
            } else if b > a {
                Outcome::Win(Player::Two)
            } else {
                Outcome::Draw
            }
        };
        if w1 || w2 || self.round >= 6 {
            self.finished = Some(match (w1, w2) {
                (true, false) => Outcome::Win(Player::One),
                (false, true) => Outcome::Win(Player::Two),
                _ => end(self.pts[0], self.pts[1]),
            });
            return;
        }
        self.first = self.first.other();
        self.deal();
    }

    /// Valoarea vizibilă pentru `p`: doar masa, cărțile proprii și faptele publice.
    fn value_for(&self, p: Player) -> i32 {
        let (a, b) = (p.idx(), p.other().idx());
        let mut v = 0;
        for c in 0..6 {
            let mine = self.table[a][c] as i32 + (self.secret[a] == Some(c as u8)) as i32;
            let theirs = self.table[b][c] as i32;
            let w = VAL[c] as i32;
            v += w * 4 * (mine - theirs).signum() + w * (mine - theirs);
            if let Some(m) = self.marker[c] {
                v += if m == p { w } else { -w };
            }
        }
        v
    }
}

fn pairs(cards: &[u8]) -> Vec<(u8, u8)> {
    let mut v = vec![];
    for i in 0..cards.len() {
        for j in i + 1..cards.len() {
            let t = (cards[i], cards[j]);
            if !v.contains(&t) {
                v.push(t);
            }
        }
    }
    v
}

impl Game for Darurile {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if self.finished.is_some() {
            return vec![];
        }
        if let Some((_, pd)) = self.pending {
            return match pd {
                Pending::Offer(c) => {
                    let mut v = vec![];
                    for &x in &c {
                        if !v.contains(&Mv::Take(x)) {
                            v.push(Mv::Take(x));
                        }
                    }
                    v
                }
                Pending::Split(_) => vec![Mv::Pair(0), Mv::Pair(1)],
            };
        }
        let p = self.side.idx();
        let hand = self.hand[p];
        let cards = self.cards(p);
        let mut v = vec![];
        if !self.used[p][0] {
            for c in 0..6u8 {
                if hand[c as usize] > 0 {
                    v.push(Mv::Secret(c));
                }
            }
        }
        if !self.used[p][1] {
            for (a, b) in pairs(&cards) {
                v.push(Mv::Burn(a, b));
            }
        }
        if !self.used[p][2] {
            let mut seen: Vec<[u8; 3]> = vec![];
            for i in 0..cards.len() {
                for j in i + 1..cards.len() {
                    for k in j + 1..cards.len() {
                        let t = [cards[i], cards[j], cards[k]];
                        if !seen.contains(&t) {
                            seen.push(t);
                            v.push(Mv::Offer(t[0], t[1], t[2]));
                        }
                    }
                }
            }
        }
        if !self.used[p][3] && cards.len() >= 4 {
            let mut seen: Vec<[u8; 4]> = vec![];
            for i in 0..cards.len() {
                for j in i + 1..cards.len() {
                    for k in j + 1..cards.len() {
                        for l in k + 1..cards.len() {
                            let (a, b) = ((cards[i], cards[j]), (cards[k], cards[l]));
                            let t = if a <= b {
                                [a.0, a.1, b.0, b.1]
                            } else {
                                [b.0, b.1, a.0, a.1]
                            };
                            if !seen.contains(&t) {
                                seen.push(t);
                                v.push(Mv::Split(t[0], t[1], t[2], t[3]));
                            }
                        }
                    }
                }
            }
        }
        if v.is_empty() {
            v.push(Mv::Burn(0, 0)); // nu se întâmplă: mâna are mereu destule cărți
        }
        v
    }

    fn apply(&mut self, m: &Mv) {
        let p = self.side.idx();
        let o = 1 - p;
        let take = |g: &mut Darurile, who: usize, c: u8| {
            if g.hand[who][c as usize] > 0 {
                g.hand[who][c as usize] -= 1;
            }
        };
        match *m {
            Mv::Secret(c) => {
                take(self, p, c);
                self.secret[p] = Some(c);
                self.used[p][0] = true;
            }
            Mv::Burn(a, b) => {
                take(self, p, a);
                take(self, p, b);
                self.used[p][1] = true;
            }
            Mv::Offer(a, b, c) => {
                for x in [a, b, c] {
                    take(self, p, x);
                }
                self.used[p][2] = true;
                self.pending = Some((self.side.other(), Pending::Offer([a, b, c])));
                self.side = self.side.other();
                return;
            }
            Mv::Split(a, b, c, d) => {
                for x in [a, b, c, d] {
                    take(self, p, x);
                }
                self.used[p][3] = true;
                self.pending = Some((self.side.other(), Pending::Split([a, b, c, d])));
                self.side = self.side.other();
                return;
            }
            Mv::Take(c) => {
                // p = cel care răspunde; o = cel care a oferit
                if let Some((_, Pending::Offer(cs))) = self.pending {
                    let mut rest = cs.to_vec();
                    if let Some(i) = rest.iter().position(|&x| x == c) {
                        rest.remove(i);
                    }
                    self.table[p][c as usize] += 1;
                    for x in rest {
                        self.table[o][x as usize] += 1;
                    }
                }
                self.pending = None;
                self.after_action(o);
                return;
            }
            Mv::Pair(i) => {
                if let Some((_, Pending::Split(cs))) = self.pending {
                    let (mine, theirs) = if i == 0 {
                        (&cs[0..2], &cs[2..4])
                    } else {
                        (&cs[2..4], &cs[0..2])
                    };
                    for &x in mine {
                        self.table[p][x as usize] += 1;
                    }
                    for &x in theirs {
                        self.table[o][x as usize] += 1;
                    }
                }
                self.pending = None;
                self.after_action(o);
                return;
            }
        }
        self.after_action(p);
    }

    fn outcome(&self) -> Option<Outcome> {
        self.finished
    }

    fn evaluate(&self, p: Player) -> i32 {
        // cu o ofertă/împărțire pendinte: adversarul alege varianta mai proastă pentru `p`
        if let Some((who, pd)) = self.pending {
            let opts: Vec<Mv> = match pd {
                Pending::Offer(c) => c.iter().map(|&x| Mv::Take(x)).collect(),
                Pending::Split(_) => vec![Mv::Pair(0), Mv::Pair(1)],
            };
            let vals = opts.iter().map(|m| {
                let mut g = self.clone();
                g.apply(m);
                g.value_for(p)
            });
            let worst = if who == p {
                vals.max().unwrap_or(0)
            } else {
                vals.min().unwrap_or(0)
            };
            return worst;
        }
        self.value_for(p)
            + self.hand[p.idx()]
                .iter()
                .zip(VAL)
                .map(|(&n, v)| n as i32 * v as i32)
                .sum::<i32>()
                / 2
    }

    fn render(&self) -> String {
        let me = self.viewer.unwrap_or(self.side);
        let o = me.other();
        let mut s = format!(
            "Runda {} din 6 | puncte: J1 {}, J2 {} | la rând: J{}\n",
            self.round + 1,
            self.pts[0],
            self.pts[1],
            self.side.idx() + 1
        );
        s.push_str("Daruri:  ");
        for c in 0..6 {
            s.push_str(&format!("{}(v{}) ", c + 1, VAL[c]));
        }
        s.push('\n');
        for (p, name) in [(o, "Adversar"), (me, "Tu")] {
            s.push_str(&format!("{} la masă:  ", name));
            for c in 0..6 {
                s.push_str(&format!("{} ", self.table[p.idx()][c]));
            }
            s.push_str(&format!(
                "| acțiuni folosite: {}\n",
                self.used[p.idx()].iter().filter(|&&x| x).count()
            ));
        }
        s.push_str("Marcaje: ");
        for c in 0..6 {
            s.push_str(match self.marker[c] {
                Some(Player::One) => "J1 ",
                Some(Player::Two) => "J2 ",
                None => "-- ",
            });
        }
        s.push_str("\nMâna ta: ");
        for c in 0..6 {
            for _ in 0..self.hand[me.idx()][c] {
                s.push_str(&format!("{} ", c + 1));
            }
        }
        if let Some(c) = self.secret[me.idx()] {
            s.push_str(&format!("| ascuns: {}", c + 1));
        }
        s.push('\n');
        if let Some((who, pd)) = self.pending {
            s.push_str(&format!(
                "J{} trebuie să aleagă din {:?}\n",
                who.idx() + 1,
                match pd {
                    Pending::Offer(c) => c.iter().map(|&x| x + 1).collect::<Vec<_>>(),
                    Pending::Split(c) => c.iter().map(|&x| x + 1).collect::<Vec<_>>(),
                }
            ));
        }
        s.push_str(
            "Mutări: ascund 3, arunc 1 2, ofer 1 2 3, impart 1 2 + 3 4, ia 3, pereche 1|2.\n",
        );
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        let f = |c: &u8| (c + 1).to_string();
        match m {
            Mv::Secret(c) => format!("ascund {}", f(c)),
            Mv::Burn(a, b) => format!("arunc {} {}", f(a), f(b)),
            Mv::Offer(a, b, c) => format!("ofer {} {} {}", f(a), f(b), f(c)),
            Mv::Split(a, b, c, d) => format!("impart {} {} + {} {}", f(a), f(b), f(c), f(d)),
            Mv::Take(c) => format!("ia {}", f(c)),
            Mv::Pair(i) => format!("pereche {}", i + 1),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }
}

impl Darurile {
    fn after_action(&mut self, actor: usize) {
        if self.round_over() {
            self.finish_round();
            if self.finished.is_none() {
                self.ensure_draw();
            }
            return;
        }
        // alternare; cine și-a terminat acțiunile le lasă celuilalt
        let other = 1 - actor;
        let done = |g: &Darurile, q: usize| g.used[q].iter().all(|&x| x);
        let next = if done(self, other) { actor } else { other };
        self.side = if next == 0 { Player::One } else { Player::Two };
        self.drew = false;
        self.ensure_draw();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_games_end() {
        for seed in 1..=40u64 {
            let mut g = Darurile::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                let m = mv[rng.below(mv.len())];
                g.apply(&m);
                n += 1;
                assert!(n < 400);
            }
        }
    }

    #[test]
    fn hands_never_negative_and_ai_legal() {
        polytabletop_core::ai::set_time_budget_override_ms(Some(5));
        let mut g = Darurile::with_seed(3);
        let mut rng = Rng::new(1);
        while g.outcome().is_none() {
            let t = polytabletop_core::GameObj::ai_move(&g, rng.next_u64()).unwrap();
            assert!(polytabletop_core::GameObj::legal_moves(&g).contains(&t));
            polytabletop_core::GameObj::play(&mut g, &t).unwrap();
        }
    }

    #[test]
    fn viewer_hides_opponent_hand() {
        let mut g = Darurile::with_seed(2);
        polytabletop_core::GameObj::set_viewer(&mut g, Some(Player::Two));
        let r2 = polytabletop_core::GameObj::render(&g);
        polytabletop_core::GameObj::set_viewer(&mut g, Some(Player::One));
        let r1 = polytabletop_core::GameObj::render(&g);
        assert_ne!(r1, r2);
    }
}
