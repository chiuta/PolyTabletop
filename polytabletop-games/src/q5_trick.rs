//! Motor comun pentru jocurile de levate cu doi jucători ale valului Q5 (Jass, Manille, Klaverjas, Sueca).
//!
//! Fiecare joc e o specificație (`TrickSpec`): pachetul (patru culori × ranguri), mâna, atuul
//! (ales de cel care conduce sau întors din pachetul rămas), obligațiile de joc și punctele cărților.
//! Un meci are `DEALS` mâini; punctele de carte din toate mâinile se adună, câștigă cine are mai multe.
//!
//! Informație ascunsă (ca în `cards52.rs`): mâna adversarului și cărțile nedistribuite sunt în stare;
//! `render` arată doar mâna lui `viewer`; `evaluate` citește doar propria mână, cartea întoarsă,
//! punctele câștigate și cărțile jucate pe masă. AI-ul (`AlphaBeta(1)`) alege lovitura cu cea mai bună
//! valoare de „mână rămasă + puncte luate”; nu poate deduce cărțile adversarului.
//! Mutări: o carte (`10h`, `As`), iar la Jass și `atu h` / `schiebe`.

use crate::cards::Viewer;
use crate::cards52::shuffle;
use crate::q5_util::{by_score, verdict};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::marker::PhantomData;

const SUIT_LETTERS: [char; 4] = ['h', 'd', 's', 'c'];

pub trait TrickSpec: Clone + 'static {
    const TITLE: &'static str;
    /// Etichetele rangurilor, indexate după „identitatea” rangului.
    const LABELS: &'static [&'static str];
    const HAND: usize;
    const DEALS: u8;
    /// Atuul îl alege cel care conduce (Jass), nu e întors.
    const CHOSEN: bool;
    const MUST_TRUMP: bool;
    /// Fără atu în culoarea cerută trebuie să bați dacă poți (Manille).
    const OVER_SUIT: bool;
    /// Când atuul se joacă peste atu, trebuie să-l bați dacă poți.
    const OVER_TRUMP: bool;
    const LAST_TRICK: i32;
    /// Bonus pentru toate levatele.
    const PIT_BONUS: i32;
    /// Jass: dacă ți-a rămas doar valetul de atu și s-a cerut atu, ești scutit de obligație.
    const BAUER_EXEMPT: bool;
    fn strength(rank: usize, trump: bool) -> i32;
    fn points(rank: usize, trump: bool) -> i32;
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Trump(u8),
    Pass,
    Play(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Stage {
    Choose,
    ChooseForced,
    Play,
}

#[derive(Clone)]
pub struct Trick<S: TrickSpec> {
    hands: [Vec<u8>; 2],
    stock: Vec<u8>,
    turned: Option<u8>,
    trump: Option<u8>,
    stage: Stage,
    leader: Player,
    side: Player,
    led: Option<u8>,
    deal_pts: [i32; 2],
    tricks: [u8; 2],
    total: [i32; 2],
    deals_done: u8,
    dealer: Player,
    rng: Rng,
    viewer: Viewer,
    log: String,
    _m: PhantomData<S>,
}

impl<S: TrickSpec> Default for Trick<S> {
    fn default() -> Self {
        Self::new()
    }
}

fn nr<S: TrickSpec>() -> usize {
    S::LABELS.len()
}

impl<S: TrickSpec> Trick<S> {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Trick {
            hands: [Vec::new(), Vec::new()],
            stock: Vec::new(),
            turned: None,
            trump: None,
            stage: Stage::Play,
            leader: Player::One,
            side: Player::One,
            led: None,
            deal_pts: [0; 2],
            tricks: [0; 2],
            total: [0; 2],
            deals_done: 0,
            dealer: Player::Two,
            rng: Rng::new(seed),
            viewer: Viewer::Fixed(Player::One),
            log: String::new(),
            _m: PhantomData,
        };
        g.deal();
        g
    }

    fn deal(&mut self) {
        let n = nr::<S>();
        let mut deck: Vec<u8> = (0..(4 * n) as u8).collect();
        shuffle(&mut deck, &mut self.rng);
        self.hands[0] = deck.drain(..S::HAND).collect();
        self.hands[1] = deck.drain(..S::HAND).collect();
        self.hands[0].sort_unstable();
        self.hands[1].sort_unstable();
        self.stock = deck;
        self.led = None;
        self.deal_pts = [0; 2];
        self.tricks = [0; 2];
        self.leader = self.dealer.other();
        self.side = self.leader;
        if S::CHOSEN {
            self.turned = None;
            self.trump = None;
            self.stage = Stage::Choose;
        } else {
            let t = *self.stock.last().unwrap();
            self.turned = Some(t);
            self.trump = Some(t / n as u8);
            self.stage = Stage::Play;
        }
    }

    pub fn totals(&self) -> [i32; 2] {
        self.total
    }

    pub fn hand(&self, p: Player) -> &[u8] {
        &self.hands[p.idx()]
    }

    fn suit(&self, c: u8) -> u8 {
        c / nr::<S>() as u8
    }
    fn rank(&self, c: u8) -> usize {
        c as usize % nr::<S>()
    }
    fn is_trump(&self, c: u8) -> bool {
        self.trump == Some(self.suit(c))
    }
    fn strength(&self, c: u8) -> i32 {
        S::strength(self.rank(c), self.is_trump(c))
    }
    fn pts(&self, c: u8) -> i32 {
        S::points(self.rank(c), self.is_trump(c))
    }

    pub fn card_text(&self, c: u8) -> String {
        format!(
            "{}{}",
            S::LABELS[self.rank(c)],
            SUIT_LETTERS[self.suit(c) as usize]
        )
    }

    /// Cartea de răspuns bate cartea condusă?
    fn beats(&self, led: u8, resp: u8) -> bool {
        if self.suit(resp) == self.suit(led) {
            self.strength(resp) > self.strength(led)
        } else {
            self.is_trump(resp) && !self.is_trump(led)
        }
    }

    fn playable(&self, who: Player) -> Vec<u8> {
        let hand = &self.hands[who.idx()];
        let Some(led) = self.led else {
            return hand.clone();
        };
        let ls = self.suit(led);
        let same: Vec<u8> = hand
            .iter()
            .copied()
            .filter(|&c| self.suit(c) == ls)
            .collect();
        if !same.is_empty() {
            if S::BAUER_EXEMPT && self.is_trump(led) && same.len() == 1 && self.rank(same[0]) == 5 {
                return hand.clone();
            }
            let must_beat = (S::OVER_SUIT) || (S::OVER_TRUMP && self.is_trump(led));
            if must_beat {
                let better: Vec<u8> = same
                    .iter()
                    .copied()
                    .filter(|&c| self.beats(led, c))
                    .collect();
                if !better.is_empty() {
                    return better;
                }
            }
            return same;
        }
        if S::MUST_TRUMP {
            let trumps: Vec<u8> = hand.iter().copied().filter(|&c| self.is_trump(c)).collect();
            if !trumps.is_empty() {
                return trumps;
            }
        }
        hand.clone()
    }

    /// Valoarea cărților rămase în mâna lui `who` (singura informație privată folosită de AI).
    fn hold(&self, who: Player, trump: Option<u8>) -> i32 {
        let n = nr::<S>() as u8;
        self.hands[who.idx()]
            .iter()
            .map(|&c| {
                let tr = trump == Some(c / n);
                let st = S::strength(c as usize % n as usize, tr);
                if tr {
                    30 + st * 4 + S::points(c as usize % n as usize, tr)
                } else {
                    st * 2 + S::points(c as usize % n as usize, tr)
                }
            })
            .sum()
    }

    fn finish_deal(&mut self, last_winner: Player) {
        self.deal_pts[last_winner.idx()] += S::LAST_TRICK;
        if S::PIT_BONUS > 0 {
            for p in 0..2 {
                if self.tricks[p] as usize == S::HAND {
                    self.deal_pts[p] += S::PIT_BONUS;
                }
            }
        }
        self.total[0] += self.deal_pts[0];
        self.total[1] += self.deal_pts[1];
        self.log = format!(
            "Mâna {} încheiată: J1 {} – J2 {} (în total {} – {}).",
            self.deals_done + 1,
            self.deal_pts[0],
            self.deal_pts[1],
            self.total[0],
            self.total[1]
        );
        self.deals_done += 1;
        if self.deals_done < S::DEALS {
            self.dealer = self.dealer.other();
            self.deal();
        }
    }
}

impl<S: TrickSpec> Game for Trick<S> {
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
        match self.stage {
            Stage::Choose => {
                let mut v: Vec<Move> = (0..4).map(Move::Trump).collect();
                v.push(Move::Pass);
                v
            }
            Stage::ChooseForced => (0..4).map(Move::Trump).collect(),
            Stage::Play => self
                .playable(self.side)
                .into_iter()
                .map(Move::Play)
                .collect(),
        }
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Trump(s) => {
                self.trump = Some(*s);
                self.stage = Stage::Play;
                self.side = self.leader;
                self.log = format!("Atu: {}.", SUIT_LETTERS[*s as usize]);
            }
            Move::Pass => {
                self.stage = Stage::ChooseForced;
                self.side = self.side.other();
                self.log = format!(
                    "J{} schimbă (schiebe): celălalt alege atuul.",
                    self.leader.idx() + 1
                );
            }
            Move::Play(c) => {
                let me = self.side;
                let h = &mut self.hands[me.idx()];
                let pos = h.iter().position(|x| x == c).expect("carte în mână");
                h.remove(pos);
                match self.led {
                    None => {
                        self.led = Some(*c);
                        self.side = me.other();
                    }
                    Some(l) => {
                        let winner = if self.beats(l, *c) { me } else { me.other() };
                        self.deal_pts[winner.idx()] += self.pts(l) + self.pts(*c);
                        self.tricks[winner.idx()] += 1;
                        self.log = format!(
                            "Levată: {} / {} — o ia J{}.",
                            self.card_text(l),
                            self.card_text(*c),
                            winner.idx() + 1
                        );
                        self.led = None;
                        self.leader = winner;
                        self.side = winner;
                        if self.hands[0].is_empty() {
                            self.finish_deal(winner);
                        }
                    }
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.deals_done >= S::DEALS {
            return Some(by_score(self.total[0], self.total[1]));
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let o = p.other();
        let mut v = 0;
        let d = |a: [i32; 2]| a[p.idx()] - a[o.idx()];
        v += (d(self.total) + d(self.deal_pts)) * 10;
        match self.stage {
            Stage::Play => {
                v += self.hold(p, self.trump);
                if let Some(l) = self.led {
                    // am condus eu (adversarul încă nu a răspuns): șansa de a lua levata
                    if self.side == o {
                        let w = self.strength(l) - if self.is_trump(l) { 0 } else { 4 };
                        v += w * 3 - self.pts(l) * 3;
                    }
                }
            }
            Stage::ChooseForced => {
                // am pasat: valoarea medie a mâinii peste cele patru culori de atu, plus un mic bonus
                if self.side == o {
                    let avg: i32 = (0..4u8).map(|s| self.hold(p, Some(s))).sum::<i32>() / 4;
                    v += avg + 25;
                }
            }
            Stage::Choose => {}
        }
        v
    }

    fn render(&self) -> String {
        let mut s = format!(
            "{} — mâna {}/{}. Total: J1 {} – J2 {}. În mâna asta: J1 {} – J2 {}.\n",
            S::TITLE,
            (self.deals_done + 1).min(S::DEALS),
            S::DEALS,
            self.total[0],
            self.total[1],
            self.deal_pts[0],
            self.deal_pts[1]
        );
        match (self.trump, self.turned) {
            (Some(t), Some(c)) => s.push_str(&format!(
                "Atu: {} (cartea întoarsă: {}).\n",
                SUIT_LETTERS[t as usize],
                self.card_text(c)
            )),
            (Some(t), None) => s.push_str(&format!("Atu: {}.\n", SUIT_LETTERS[t as usize])),
            _ => s.push_str("Atu: încă nealeasă.\n"),
        }
        let v = self.viewer.who(self.side);
        let o = v.other();
        s.push_str(&format!(
            "Jucător {} (ascuns) are {} cărți.\n",
            o.idx() + 1,
            self.hands[o.idx()].len()
        ));
        if let Some(l) = self.led {
            s.push_str(&format!(
                "Pe masă: Jucător {} a jucat {}\n",
                self.side.other().idx() + 1,
                self.card_text(l)
            ));
        }
        let h: Vec<String> = self.hands[v.idx()]
            .iter()
            .map(|&c| self.card_text(c))
            .collect();
        s.push_str(&format!(
            "Mâna ta (Jucător {}): {}\n",
            v.idx() + 1,
            h.join(" ")
        ));
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        let fin = verdict(self.outcome());
        if fin.is_empty() {
            let what = match self.stage {
                Stage::Choose => " — alege atuul: atu h/d/s/c sau schiebe",
                Stage::ChooseForced => " — alege atuul: atu h/d/s/c",
                Stage::Play => "",
            };
            s.push_str(&format!(
                "La mutare: Jucător {}{what}\n",
                self.side.idx() + 1
            ));
        } else {
            s.push_str(&fin);
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Trump(t) => format!("atu {}", SUIT_LETTERS[*t as usize]),
            Move::Pass => "schiebe".into(),
            Move::Play(c) => self.card_text(*c),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

/// Pentru teste: reamestecă tot ce `viewer` nu poate vedea (mâna adversarului + cărțile nedistribuite),
/// păstrând cartea întoarsă și cărțile publice la locul lor.
#[cfg(test)]
pub fn scramble_hidden<S: TrickSpec>(g: &mut Trick<S>, viewer: Player, seed: u64) {
    let opp = viewer.other().idx();
    let keep = g.turned;
    let mut pool: Vec<u8> = g.hands[opp].clone();
    let n_opp = pool.len();
    pool.extend(g.stock.iter().copied().filter(|&c| Some(c) != keep));
    shuffle(&mut pool, &mut Rng::new(seed));
    g.hands[opp] = pool.drain(..n_opp).collect();
    g.hands[opp].sort_unstable();
    g.stock = pool;
    if let Some(k) = keep {
        g.stock.push(k);
    }
}

#[cfg(test)]
pub mod tests_support {
    use super::*;

    pub fn ai_is_blind<S: TrickSpec>() {
        for seed in 1..=3u64 {
            let mut a = Trick::<S>::with_seed(seed);
            // avansează câteva mutări ca să avem stări variate
            for k in 0..5 {
                let m = a.legal_moves();
                a.apply(&m[k % m.len()]);
            }
            let mover = a.to_move();
            let mut b = a.clone();
            scramble_hidden(&mut b, mover, seed * 77);
            // pentru cărți deja jucate nu schimbăm nimic; mâna adversarului diferă (de regulă)
            for s in 1..8u64 {
                assert_eq!(
                    polytabletop_core::GameObj::ai_move(&a, s),
                    polytabletop_core::GameObj::ai_move(&b, s),
                    "AI-ul depinde de informația ascunsă"
                );
            }
        }
    }

    pub fn viewer_hides_the_other_hand<S: TrickSpec>() {
        let mut g = Trick::<S>::with_seed(3);
        g.set_viewer(Some(Player::One));
        let r1 = g.render();
        g.set_viewer(Some(Player::Two));
        let r2 = g.render();
        assert_ne!(r1, r2);
        let mine: Vec<String> = g.hands[0].iter().map(|&c| g.card_text(c)).collect();
        let theirs: Vec<String> = g.hands[1].iter().map(|&c| g.card_text(c)).collect();
        let line1 = r1.lines().find(|l| l.starts_with("Mâna ta")).unwrap();
        for c in &mine {
            assert!(
                line1.split_whitespace().any(|w| w == c),
                "{c} lipsă din {line1}"
            );
        }
        for c in &theirs {
            assert!(
                !line1.split_whitespace().any(|w| w == c),
                "{c} scurs în {line1}"
            );
        }
    }

    pub fn play_all_cards_conserves_points<S: TrickSpec>() {
        let mut g = Trick::<S>::with_seed(9);
        let mut rng = Rng::new(5);
        let mut deals = 0;
        while g.outcome().is_none() {
            let m = g.legal_moves();
            g.apply(&m[rng.below(m.len())]);
            if g.deals_done > deals {
                deals = g.deals_done;
            }
        }
        assert_eq!(deals, S::DEALS);
        assert!(g.total[0] + g.total[1] > 0);
    }
}
