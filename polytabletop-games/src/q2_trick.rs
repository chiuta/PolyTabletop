//! Motor generic de levate pentru doi jucători (Doppelkopf, Tarock): pachetul, mâna și regulile de
//! carte vin dintr-un `TrickRules`. Informație ascunsă: mâna adversarului și cărțile nedistribuite;
//! AI-ul (alpha-beta cu adâncime 1) citește doar propria mână, levatele (publice) și numărul cărților.

use crate::cards::{shuffle, Viewer};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::marker::PhantomData;

pub trait TrickRules: Clone + 'static {
    const NAME: &'static str;
    const HAND: usize;
    const HAND_WINS: i32;
    fn deck() -> Vec<u8>;
    fn label(id: u8) -> String;
    fn group(id: u8) -> u8;
    fn trump_group() -> u8;
    fn rank(id: u8) -> i32;
    fn points(id: u8) -> i32;
    fn power(_id: u8) -> i32 {
        0
    }
    fn free_play(_id: u8) -> bool {
        false
    }
    fn never_wins(_id: u8) -> bool {
        false
    }
    fn second_wins_tie(_id: u8) -> bool {
        false
    }
    fn must_trump() -> bool {
        false
    }
    /// Punctele cărții rămân jucătorului care a jucat-o (ex. Škis).
    fn keeps_owner(_id: u8) -> bool {
        false
    }
    /// Câștigătorul mâinii după punctele de levate și ultima levată (cartea câștigătoare).
    fn hand_winner(pts: [i32; 2], last: Option<(Player, u8)>) -> Option<Player>;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TMove {
    Deal,
    Play(u8),
}

#[derive(Clone)]
pub struct Tk<R: TrickRules> {
    rng: Rng,
    hands: [Vec<u8>; 2],
    stock: Vec<u8>,
    trick: Vec<(Player, u8)>,
    pts: [i32; 2],
    side: Player,
    wins: [i32; 2],
    hands_played: u8,
    dealing: bool,
    over: bool,
    viewer: Viewer,
    log: String,
    _r: PhantomData<R>,
}

impl<R: TrickRules> Tk<R> {
    pub fn with_seed(seed: u64) -> Self {
        let mut g = Tk {
            rng: Rng::new(seed),
            hands: [Vec::new(), Vec::new()],
            stock: Vec::new(),
            trick: Vec::new(),
            pts: [0; 2],
            side: Player::One,
            wins: [0; 2],
            hands_played: 0,
            dealing: true,
            over: false,
            viewer: Viewer::Fixed(Player::One),
            log: String::new(),
            _r: PhantomData,
        };
        g.deal();
        g
    }

    pub fn hand_wins(&self) -> (i32, i32) {
        (self.wins[0], self.wins[1])
    }

    pub fn hand_of(&self, p: Player) -> &[u8] {
        &self.hands[p.idx()]
    }

    #[cfg(test)]
    pub fn hands_set_for_test(&mut self, h: [Vec<u8>; 2], _unused: [Vec<u8>; 2]) {
        self.hands = h;
        self.stock.clear();
        self.trick.clear();
        self.pts = [0; 2];
        self.dealing = false;
        self.side = Player::One;
    }

    #[cfg(test)]
    pub fn reverse_hand_for_test(&mut self, p: Player) {
        self.hands[p.idx()].reverse();
    }

    pub fn eyes(&self) -> (i32, i32) {
        (self.pts[0], self.pts[1])
    }

    fn sort(h: &mut [u8]) {
        h.sort_by_key(|&c| (R::group(c), std::cmp::Reverse(R::rank(c))));
    }

    fn deal(&mut self) {
        let mut d = R::deck();
        shuffle(&mut d, &mut self.rng);
        for p in 0..2 {
            self.hands[p] = d.drain(..R::HAND).collect();
            Self::sort(&mut self.hands[p]);
        }
        self.stock = d;
        self.trick.clear();
        self.pts = [0; 2];
        self.dealing = false;
        self.side = if self.hands_played.is_multiple_of(2) {
            Player::One
        } else {
            Player::Two
        };
    }

    fn playable(&self, p: Player) -> Vec<u8> {
        let hand = &self.hands[p.idx()];
        if let Some(&(_, f)) = self.trick.first() {
            if !R::never_wins(f) {
                let g = R::group(f);
                let same: Vec<u8> = hand
                    .iter()
                    .copied()
                    .filter(|&c| R::group(c) == g || R::free_play(c))
                    .collect();
                if same.iter().any(|&c| !R::free_play(c)) {
                    return same;
                }
                if R::must_trump() {
                    let t: Vec<u8> = hand
                        .iter()
                        .copied()
                        .filter(|&c| R::group(c) == R::trump_group() || R::free_play(c))
                        .collect();
                    if t.iter().any(|&c| !R::free_play(c)) {
                        return t;
                    }
                }
            }
        }
        hand.clone()
    }

    fn second_beats(first: u8, second: u8) -> bool {
        if R::never_wins(second) {
            return false;
        }
        if R::never_wins(first) {
            return true;
        }
        let (gf, gs) = (R::group(first), R::group(second));
        if gf == gs {
            let (rf, rs) = (R::rank(first), R::rank(second));
            rs > rf || (rs == rf && R::second_wins_tie(second))
        } else {
            gs == R::trump_group()
        }
    }
}

impl<R: TrickRules> Game for Tk<R> {
    type Move = TMove;

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = match who {
            Some(p) => Viewer::Fixed(p),
            None => Viewer::HotSeat,
        };
    }

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<TMove> {
        if self.over {
            return Vec::new();
        }
        if self.dealing {
            return vec![TMove::Deal];
        }
        let mut seen: Vec<String> = Vec::new();
        let mut v = Vec::new();
        for c in self.playable(self.side) {
            let l = R::label(c);
            if !seen.contains(&l) {
                seen.push(l);
                v.push(TMove::Play(c));
            }
        }
        v
    }

    fn apply(&mut self, m: &TMove) {
        self.log.clear();
        match *m {
            TMove::Deal => self.deal(),
            TMove::Play(c) => {
                let p = self.side;
                let pos = self.hands[p.idx()].iter().position(|&x| x == c).unwrap();
                self.hands[p.idx()].remove(pos);
                self.trick.push((p, c));
                if self.trick.len() < 2 {
                    self.side = p.other();
                    return;
                }
                let (lp, lc) = self.trick[0];
                let (rp, rc) = self.trick[1];
                let w = if Self::second_beats(lc, rc) { rp } else { lp };
                for &(who, card) in &self.trick {
                    let to = if R::keeps_owner(card) { who } else { w };
                    self.pts[to.idx()] += R::points(card);
                }
                let win_card = if w == rp { rc } else { lc };
                self.trick.clear();
                self.side = w;
                if self.hands[0].is_empty() && self.hands[1].is_empty() {
                    match R::hand_winner(self.pts, Some((w, win_card))) {
                        Some(h) => {
                            self.wins[h.idx()] += 1;
                            self.log = format!(
                                "Mână încheiată ({}–{} puncte): J{} o câștigă.",
                                self.pts[0],
                                self.pts[1],
                                h.idx() + 1
                            );
                        }
                        None => {
                            self.log =
                                format!("Mână încheiată ({}–{}): remiză.", self.pts[0], self.pts[1])
                        }
                    }
                    self.hands_played += 1;
                    if self.wins.iter().any(|&x| x >= R::HAND_WINS) || self.hands_played >= 5 {
                        self.over = true;
                    } else {
                        self.dealing = true;
                        self.side = if self.hands_played.is_multiple_of(2) {
                            Player::One
                        } else {
                            Player::Two
                        };
                    }
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            return None;
        }
        Some(match self.wins[0].cmp(&self.wins[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let (me, q) = (p.idx(), 1 - p.idx());
        let mut v = (self.wins[me] - self.wins[q]) * 20000 + (self.pts[me] - self.pts[q]) * 100;
        v += self.hands[me].iter().map(|&c| R::power(c)).sum::<i32>() * 8;
        if let Some(&(lp, c)) = self.trick.first() {
            // carte condusă (publică): bună dacă e puternică și ieftină
            let k = R::power(c) * 3 - R::points(c) * 6;
            v += if lp == p { k } else { -k };
        }
        v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "{} — mâini câștigate: J1 {}, J2 {} (la {}); puncte în mâna curentă: J1 {}, J2 {}\n",
            R::NAME,
            self.wins[0],
            self.wins[1],
            R::HAND_WINS,
            self.pts[0],
            self.pts[1]
        );
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        if self.over {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        if !self.trick.is_empty() {
            let t: Vec<String> = self.trick.iter().map(|&(_, c)| R::label(c)).collect();
            s.push_str(&format!("Pe masă: {}\n", t.join(" ")));
        }
        let v = self.viewer.who(self.side);
        let hand: Vec<String> = self.hands[v.idx()].iter().map(|&c| R::label(c)).collect();
        s.push_str(&format!("Mâna ta (J{}): {}\n", v.idx() + 1, hand.join(" ")));
        s.push_str(&format!(
            "J{} are {} cărți. La mutare: J{}\n",
            v.other().idx() + 1,
            self.hands[v.other().idx()].len(),
            self.side.idx() + 1
        ));
        s
    }

    fn format_move(&self, m: &TMove) -> String {
        match m {
            TMove::Deal => "imparte".into(),
            TMove::Play(c) => R::label(*c),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

/// Generează un tip-înveliș `$name` peste `Tk<$rules>` cu `new`/`with_seed`/`Default` și delegare `Game`.
#[macro_export]
macro_rules! trick_game {
    ($name:ident, $rules:ty) => {
        #[derive(Clone)]
        pub struct $name(pub $crate::q2_trick::Tk<$rules>);
        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }
        impl $name {
            pub fn new() -> Self {
                Self::with_seed(polytabletop_core::rng::Rng::from_clock().next_u64())
            }
            pub fn with_seed(seed: u64) -> Self {
                $name($crate::q2_trick::Tk::with_seed(seed))
            }
        }
        impl polytabletop_core::Game for $name {
            type Move = $crate::q2_trick::TMove;
            fn set_viewer(&mut self, w: Option<polytabletop_core::Player>) {
                self.0.set_viewer(w)
            }
            fn to_move(&self) -> polytabletop_core::Player {
                self.0.to_move()
            }
            fn legal_moves(&self) -> Vec<Self::Move> {
                self.0.legal_moves()
            }
            fn apply(&mut self, m: &Self::Move) {
                self.0.apply(m)
            }
            fn outcome(&self) -> Option<polytabletop_core::Outcome> {
                self.0.outcome()
            }
            fn evaluate(&self, p: polytabletop_core::Player) -> i32 {
                self.0.evaluate(p)
            }
            fn render(&self) -> String {
                self.0.render()
            }
            fn format_move(&self, m: &Self::Move) -> String {
                self.0.format_move(m)
            }
            fn ai_style(&self) -> polytabletop_core::AiStyle {
                self.0.ai_style()
            }
        }
    };
}
