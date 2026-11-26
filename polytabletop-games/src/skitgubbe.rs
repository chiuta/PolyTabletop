//! Skitgubbe (Suedia), joc de cărți pentru 2 jucători, în două faze; nu există câștigător, ci un „gubbe” (pierzător).
//!
//! * Pachet de 52. **Faza 1 (acumulare)**: fiecare ține 3 cărți, restul e talonul. Se joacă levate între cei doi;
//!   culoarea nu contează, **rangul cel mai mare ia** (A mare), la egalitate se mai joacă o carte fiecare
//!   (dacă cineva rămâne fără cărți, fiecare își reia cărțile jucate). Câștigătorul pune cărțile în teancul lui,
//!   conduce și completează primul cu talonul până la 3. Poți „risca” (`risc`) jucând cartea de deasupra talonului.
//!   Ultima carte din talon (cea pe care o trage cineva) se întoarce: culoarea ei e **atu**, iar cartea îi
//!   rămâne celui care a tras-o pentru faza 2. Faza 1 se termină când cineva nu mai are cărți.
//! * **Faza 2 (golire)**: fiecare își reunește mâna și teancul. Cel care a tras ultima carte conduce. Conducătorul
//!   joacă o carte; celălalt o **bate** (aceeași culoare, rang mai mare, sau atu; peste un atu doar un atu mai
//!   mare) și levata se scoate, iar el conduce, sau o **ridică** (`ridic`) și conducătorul conduce din nou.
//! * Cine rămâne primul fără cărți e în siguranță; ultimul cu cărți e **skitgubbe** (pierde). Dacă rămân
//!   amândoi fără cărți în aceeași levată, e remiză.
//!
//! AI-ul nu folosește `risc` (nu vede cartea de pe talon) și citește doar propria mână.

use crate::cards::{full_deck, join_cards, shuffle, Card, Viewer};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Play(Card),
    Risk,
    Pickup,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Acc,
    Shed,
    Over,
}

#[derive(Clone)]
pub struct Skitgubbe {
    rng: Rng,
    hands: [Vec<Card>; 2],
    stock: Vec<Card>,
    piles: [Vec<Card>; 2],
    trick: Vec<(Player, Card)>,
    side: Player,
    phase: Phase,
    trump: Option<u8>,
    trump_card: Option<(Player, Card)>,
    lay: Option<Card>,
    risked: Option<Player>,
    result: Option<Outcome>,
    viewer: Viewer,
    log: String,
}

impl Default for Skitgubbe {
    fn default() -> Self {
        Self::new()
    }
}

fn sort(h: &mut [Card]) {
    h.sort_by_key(|c| (c.suit, std::cmp::Reverse(c.rank)));
}

impl Skitgubbe {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut d = full_deck(2);
        shuffle(&mut d, &mut rng);
        let mut h = [Vec::new(), Vec::new()];
        for _ in 0..3 {
            h[0].push(d.pop().unwrap());
            h[1].push(d.pop().unwrap());
        }
        sort(&mut h[0]);
        sort(&mut h[1]);
        Skitgubbe {
            rng,
            hands: h,
            stock: d,
            piles: [Vec::new(), Vec::new()],
            trick: Vec::new(),
            side: Player::One,
            phase: Phase::Acc,
            trump: None,
            trump_card: None,
            lay: None,
            risked: None,
            result: None,
            viewer: Viewer::Fixed(Player::One),
            log: String::new(),
        }
    }

    pub fn hand_of(&self, p: Player) -> &[Card] {
        &self.hands[p.idx()]
    }

    fn draw_to_three(&mut self, first: Player) {
        for p in [first, first.other()] {
            while self.hands[p.idx()].len() < 3 {
                match self.stock.pop() {
                    Some(c) => {
                        if self.stock.is_empty() {
                            // ultima carte: se întoarce, dă atuul și rămâne celui care a tras-o
                            self.trump = Some(c.suit);
                            self.trump_card = Some((p, c));
                            self.log
                                .push_str(&format!(" Ultima carte din talon: {} (atu).", c.text()));
                        } else {
                            self.hands[p.idx()].push(c);
                        }
                    }
                    None => break,
                }
            }
            sort(&mut self.hands[p.idx()]);
        }
    }

    fn start_shed(&mut self) {
        let mut lead = self.side;
        for p in 0..2 {
            let mut all = std::mem::take(&mut self.hands[p]);
            all.append(&mut self.piles[p]);
            if let Some((who, c)) = self.trump_card {
                if who.idx() == p {
                    all.push(c);
                }
            }
            sort(&mut all);
            self.hands[p] = all;
        }
        if let Some((who, _)) = self.trump_card {
            lead = who;
        }
        self.phase = Phase::Shed;
        self.side = lead;
        self.lay = None;
        self.trick.clear();
        self.log.push_str(" Începe faza 2.");
        self.check_over();
    }

    fn check_over(&mut self) {
        if self.phase != Phase::Shed || self.lay.is_some() {
            return;
        }
        let (a, b) = (self.hands[0].is_empty(), self.hands[1].is_empty());
        if a || b {
            self.phase = Phase::Over;
            self.result = Some(match (a, b) {
                (true, true) => Outcome::Draw,
                (true, false) => Outcome::Win(Player::One),
                _ => Outcome::Win(Player::Two),
            });
        }
    }

    fn beats(&self, lay: Card, c: Card) -> bool {
        if Some(lay.suit) == self.trump {
            c.suit == lay.suit && c.rank > lay.rank
        } else {
            (c.suit == lay.suit && c.rank > lay.rank) || Some(c.suit) == self.trump
        }
    }

    fn resolve_acc_trick(&mut self) {
        let n = self.trick.len();
        let (a, b) = (self.trick[n - 2], self.trick[n - 1]);
        if a.1.rank == b.1.rank {
            let me_empty = self.hands[0].is_empty() || self.hands[1].is_empty();
            if me_empty {
                // nu se mai poate continua: fiecare își ia cărțile înapoi
                let t = std::mem::take(&mut self.trick);
                for (p, c) in t {
                    self.piles[p.idx()].push(c);
                }
                self.log = "Egalitate fără cărți: fiecare își reia cărțile.".into();
                self.after_trick(self.trick_leader_hint());
            } else {
                self.log = "Egalitate: se mai joacă o carte fiecare.".into();
                self.side = a.0;
            }
            return;
        }
        let w = if b.1.rank > a.1.rank { b.0 } else { a.0 };
        let t = std::mem::take(&mut self.trick);
        for (_, c) in t {
            self.piles[w.idx()].push(c);
        }
        self.log = format!("J{} ia levata.", w.idx() + 1);
        self.after_trick(w);
    }

    fn trick_leader_hint(&self) -> Player {
        self.side
    }

    fn after_trick(&mut self, w: Player) {
        self.draw_to_three(w);
        self.side = w;
        if self.hands[0].is_empty() || self.hands[1].is_empty() {
            self.start_shed();
        }
    }
}

impl Game for Skitgubbe {
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
        match self.phase {
            Phase::Over => Vec::new(),
            Phase::Acc => {
                let mut v: Vec<Move> = Vec::new();
                for &c in &self.hands[self.side.idx()] {
                    if !v.contains(&Move::Play(c)) {
                        v.push(Move::Play(c));
                    }
                }
                if !self.stock.is_empty() && self.stock.len() > 1 {
                    v.push(Move::Risk);
                }
                v
            }
            Phase::Shed => {
                let h = &self.hands[self.side.idx()];
                match self.lay {
                    None => h.iter().map(|&c| Move::Play(c)).collect(),
                    Some(l) => {
                        let mut v: Vec<Move> = h
                            .iter()
                            .filter(|&&c| self.beats(l, c))
                            .map(|&c| Move::Play(c))
                            .collect();
                        v.push(Move::Pickup);
                        v
                    }
                }
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        self.risked = None;
        let p = self.side;
        match self.phase {
            Phase::Over => {}
            Phase::Acc => {
                let c = match *m {
                    Move::Play(c) => {
                        let pos = self.hands[p.idx()].iter().position(|&x| x == c).unwrap();
                        self.hands[p.idx()].remove(pos);
                        c
                    }
                    _ => {
                        self.risked = Some(p);
                        let k = self.rng.below(self.stock.len() - 1) + 1;
                        // cartea de deasupra: ultima din vector, dar ultima trasă (indice 0) rămâne atu
                        let _ = k;
                        self.stock.pop().unwrap()
                    }
                };
                self.trick.push((p, c));
                if self.trick.len() % 2 == 1 {
                    self.side = p.other();
                    if self.hands[self.side.idx()].is_empty() && self.stock.len() <= 1 {
                        // adversarul nu mai poate răspunde: faza 1 se încheie, cartea se întoarce
                        let (lp, lc) = self.trick.pop().unwrap();
                        self.hands[lp.idx()].push(lc);
                        self.start_shed();
                    }
                } else {
                    self.resolve_acc_trick();
                }
            }
            Phase::Shed => match *m {
                Move::Play(c) => {
                    let pos = self.hands[p.idx()].iter().position(|&x| x == c).unwrap();
                    self.hands[p.idx()].remove(pos);
                    match self.lay.take() {
                        None => {
                            self.lay = Some(c);
                            self.side = p.other();
                            if self.hands[p.idx()].is_empty() {
                                // a jucat ultima carte: e în siguranță, dar levata trebuie decisă
                                self.log = format!("J{} joacă ultima carte.", p.idx() + 1);
                            }
                        }
                        Some(_) => {
                            self.log =
                                format!("J{} bate cu {}; levata se scoate.", p.idx() + 1, c.text());
                            self.side = p;
                            self.check_over();
                        }
                    }
                }
                _ => {
                    let l = self.lay.take().unwrap();
                    self.hands[p.idx()].push(l);
                    sort(&mut self.hands[p.idx()]);
                    self.log = format!("J{} ridică {}.", p.idx() + 1, l.text());
                    self.side = p.other();
                    self.check_over();
                }
            },
        }
        if self.phase == Phase::Shed && self.lay.is_none() {
            self.check_over();
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.result
    }

    fn evaluate(&self, p: Player) -> i32 {
        if self.risked == Some(p) {
            return -100_000;
        }
        let (me, q) = (p.idx(), p.other().idx());
        match self.phase {
            Phase::Acc | Phase::Over => {
                let mut v = (self.piles[me].len() as i32 - self.piles[q].len() as i32) * 30;
                v += self.hands[me].iter().map(|c| c.rank as i32).sum::<i32>() * 2;
                if let Some(&(lp, c)) = self.trick.first() {
                    let k = c.rank as i32;
                    v += if lp == p { k } else { -k };
                }
                v
            }
            Phase::Shed => {
                let mut v = (self.hands[q].len() as i32 - self.hands[me].len() as i32) * 100;
                let trumps = self.hands[me]
                    .iter()
                    .filter(|c| Some(c.suit) == self.trump)
                    .count() as i32;
                v += trumps * 6;
                v += self.hands[me].iter().map(|c| c.rank as i32).sum::<i32>();
                v
            }
        }
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Skitgubbe — faza {} | talon: {} cărți | atu: {}\n",
            if self.phase == Phase::Acc { 1 } else { 2 },
            self.stock.len(),
            match self.trump {
                Some(t) => crate::cardsb_util::suit_name(t).to_string(),
                None => "încă necunoscut".into(),
            }
        );
        if self.phase == Phase::Acc {
            s.push_str(&format!(
                "Teancuri câștigate: J1 {}, J2 {} cărți\n",
                self.piles[0].len(),
                self.piles[1].len()
            ));
        }
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log.trim()));
        }
        if !self.trick.is_empty() {
            let t: Vec<Card> = self.trick.iter().map(|&(_, c)| c).collect();
            s.push_str(&format!("Pe masă: {}\n", join_cards(&t)));
        }
        if let Some(l) = self.lay {
            s.push_str(&format!("Carte condusă: {}\n", l.text()));
        }
        let v = self.viewer.who(self.side);
        s.push_str(&format!(
            "Mâna ta (J{}): {}\n",
            v.idx() + 1,
            join_cards(&self.hands[v.idx()])
        ));
        s.push_str(&format!(
            "J{} are {} cărți\n",
            v.other().idx() + 1,
            self.hands[v.other().idx()].len()
        ));
        match self.result {
            Some(Outcome::Win(w)) => s.push_str(&format!(
                "J{} a rămas fără cărți; J{} e skitgubbe.\n",
                w.idx() + 1,
                w.other().idx() + 1
            )),
            Some(Outcome::Draw) => s.push_str("Remiză.\n"),
            None => s.push_str(&format!("La mutare: J{}\n", self.side.idx() + 1)),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Play(c) => c.text(),
            Move::Risk => "risc".into(),
            Move::Pickup => "ridic".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(rank: u8, suit: u8) -> Card {
        Card { rank, suit }
    }

    #[test]
    fn higher_rank_wins_regardless_of_suit_and_refills() {
        let mut g = Skitgubbe::with_seed(5);
        g.hands = [
            vec![c(9, 0), c(3, 1), c(4, 2)],
            vec![c(12, 3), c(5, 1), c(6, 2)],
        ];
        g.apply(&Move::Play(c(9, 0)));
        g.apply(&Move::Play(c(12, 3)));
        assert_eq!(g.piles[1].len(), 2);
        assert_eq!(g.side, Player::Two);
        assert_eq!(g.hands[0].len(), 3);
        assert_eq!(g.hands[1].len(), 3);
    }

    #[test]
    fn shed_phase_beat_or_pickup() {
        let mut g = Skitgubbe::with_seed(5);
        g.phase = Phase::Shed;
        g.trump = Some(1);
        g.stock.clear();
        g.hands = [vec![c(9, 0), c(2, 2)], vec![c(10, 0), c(3, 1), c(4, 3)]];
        g.side = Player::One;
        g.apply(&Move::Play(c(9, 0)));
        let m = g.legal_moves();
        assert!(
            m.contains(&Move::Play(c(10, 0)))
                && m.contains(&Move::Play(c(3, 1)))
                && m.contains(&Move::Pickup)
        );
        assert!(!m.contains(&Move::Play(c(4, 3))));
        g.apply(&Move::Pickup);
        assert_eq!(g.hands[1].len(), 4);
        assert_eq!(g.side, Player::One);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=15u64 {
            let mut g = Skitgubbe::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty(), "{}", g.render());
                g.apply(&m[rng.below(m.len())].clone());
                n += 1;
                assert!(n < 3000, "{}", g.render());
            }
        }
    }

    #[test]
    fn ai_ignores_the_hidden_hand_and_does_not_risk() {
        let a = Skitgubbe::with_seed(3);
        let mut b = a.clone();
        b.hands[1].reverse();
        let m = polytabletop_core::GameObj::ai_move(&a, 1);
        assert_eq!(m, polytabletop_core::GameObj::ai_move(&b, 1));
        assert_ne!(m.as_deref(), Some("risc"));
    }
}
