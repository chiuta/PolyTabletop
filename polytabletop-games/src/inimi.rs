//! Inimi — joc de levate cu evitarea punctelor (tip „Hearts”), variantă pentru 2 jucători.
//!
//! Reguli din descrierile publice ale jocului de tip Hearts (nume generic; nicio marcă):
//! * Pachet de 52, câte 13 cărți fiecăruia; ceilalte 26 rămân nefolosite și nevăzute.
//! * Cine nu conduce prima levată o deschide; câștigătorul unei levate conduce următoarea.
//!   Se răspunde obligatoriu la culoare; nu există atu, ia cea mai mare carte din culoarea condusă.
//! * Fiecare inimă (h) costă 1 punct, Dama de pică (Qs) costă 13. Cine ia levata ia punctele.
//! * Nu se poate conduce inimă până nu s-a jucat una („inimi sparte”), decât dacă ai doar inimi.
//! * Când cineva atinge 40 de puncte, câștigă cel cu mai puține (egal = remiză).
//!
//! Mutări: `imparte`, carte (`7c`, `Qs`, `10h`). AI-ul citește doar propria mână, levatele
//! luate (publice) și numărul cărților adversarului.

use crate::cards::{join_cards, Card, Viewer};
use crate::trickkit::{beats, deal, follow};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const LIMIT: i32 = 40;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Deal,
    Play,
    Over,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Deal,
    Play(Card),
}

#[derive(Clone)]
pub struct Inimi {
    rng: Rng,
    hands: [Vec<Card>; 2],
    stock: Vec<Card>,
    dealer: Player,
    phase: Phase,
    side: Player,
    trick: Vec<(Player, Card)>,
    hand_pts: [i32; 2],
    totals: [i32; 2],
    broken: bool,
    qs_out: bool,
    log: String,
    viewer: Viewer,
}

impl Default for Inimi {
    fn default() -> Self {
        Self::new()
    }
}

fn pts(c: Card) -> i32 {
    if c.suit == 0 {
        1
    } else if c.suit == 2 && c.rank == 12 {
        13
    } else {
        0
    }
}

impl Inimi {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Inimi {
            rng: Rng::new(seed),
            hands: [Vec::new(), Vec::new()],
            stock: Vec::new(),
            dealer: Player::Two,
            phase: Phase::Deal,
            side: Player::One,
            trick: Vec::new(),
            hand_pts: [0, 0],
            totals: [0, 0],
            broken: false,
            qs_out: false,
            log: String::new(),
            viewer: Viewer::Fixed(Player::One),
        };
        g.deal_hand();
        g
    }

    pub fn totals(&self) -> (i32, i32) {
        (self.totals[0], self.totals[1])
    }

    fn deal_hand(&mut self) {
        let (h, s) = deal(&mut self.rng, 13);
        self.hands = h;
        self.stock = s;
        self.hand_pts = [0, 0];
        self.broken = false;
        self.qs_out = false;
        self.trick.clear();
        self.phase = Phase::Play;
        self.side = self.dealer.other();
    }

    /// Mâna unui jucător (pentru teste și interfețe care au voie să o vadă).
    pub fn hand_of(&self, p: Player) -> &[Card] {
        &self.hands[p.idx()]
    }

    /// Pentru teste: reamestecă ce nu vede `viewer`.
    pub fn scramble_hidden(&mut self, viewer: Player, seed: u64) {
        crate::trickkit::scramble_hidden(&mut self.hands, &mut self.stock, viewer, &[], seed);
    }
}

impl Game for Inimi {
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
            Phase::Deal => vec![Move::Deal],
            Phase::Play => {
                let h = &self.hands[self.side.idx()];
                let mut v = follow(h, &self.trick);
                if self.trick.is_empty() && !self.broken && v.iter().any(|c| c.suit != 0) {
                    v.retain(|c| c.suit != 0);
                }
                v.into_iter().map(Move::Play).collect()
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        match *m {
            Move::Deal => self.deal_hand(),
            Move::Play(c) => {
                let p = self.side;
                let pos = self.hands[p.idx()].iter().position(|&x| x == c).unwrap();
                self.hands[p.idx()].remove(pos);
                if c.suit == 0 {
                    self.broken = true;
                }
                if pts(c) == 13 {
                    self.qs_out = true;
                }
                self.trick.push((p, c));
                if self.trick.len() < 2 {
                    self.side = p.other();
                    return;
                }
                let (lp, lc) = self.trick[0];
                let (rp, rc) = self.trick[1];
                let w = if beats(lc, rc, None) { rp } else { lp };
                let got = pts(lc) + pts(rc);
                self.hand_pts[w.idx()] += got;
                self.trick.clear();
                self.side = w;
                if self.hands[0].is_empty() {
                    for i in 0..2 {
                        self.totals[i] += self.hand_pts[i];
                    }
                    self.log = format!(
                        "Mână încheiată: J1 +{}, J2 +{} puncte",
                        self.hand_pts[0], self.hand_pts[1]
                    );
                    if self.totals[0] >= LIMIT || self.totals[1] >= LIMIT {
                        self.phase = Phase::Over;
                    } else {
                        self.dealer = self.dealer.other();
                        self.phase = Phase::Deal;
                        self.side = self.dealer.other();
                    }
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        Some(match self.totals[0].cmp(&self.totals[1]) {
            std::cmp::Ordering::Less => Outcome::Win(Player::One),
            std::cmp::Ordering::Greater => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let q = p.other();
        let (pi, qi) = (p.idx(), q.idx());
        let mut s =
            100 * ((self.totals[qi] + self.hand_pts[qi]) - (self.totals[pi] + self.hand_pts[pi]));
        if self.phase != Phase::Play {
            return s;
        }
        // pericolul cărților rămase în mâna mea (doar propria mână)
        let mut risk = 0.0f32;
        let q_unseen = !self.qs_out && !self.hands[pi].iter().any(|&c| pts(c) == 13);
        for &c in &self.hands[pi] {
            let r = (c.rank as f32 - 2.0) / 12.0;
            risk += match (c.suit, c.rank) {
                (2, 12) => 6.0,
                (0, _) => 0.2 + 0.6 * r,
                (2, 13) | (2, 14) if q_unseen => 1.5,
                _ => 0.02 * r,
            };
        }
        s -= (risk * 100.0) as i32;
        // am condus eu: cu cât cartea e mai mare, cu atât risc mai mult să iau levata
        if let Some(&(w, c)) = self.trick.first() {
            let r = (c.rank as f32 - 2.0) / 12.0;
            let at_stake = if c.suit == 0 { 1.5 } else { 0.6 };
            let pw = 0.1 + 0.8 * r;
            if w == p {
                s -= (100.0 * pw * at_stake) as i32;
            } else {
                // adversarul a condus: răspunsul meu e încă de ales; nu se întâmplă după apply
                s += (100.0 * pw * at_stake) as i32;
            }
        }
        s
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Inimi — puncte: J1 {}, J2 {} (la {} pierde cine are mai multe) | dealer: J{}\n",
            self.totals[0],
            self.totals[1],
            LIMIT,
            self.dealer.idx() + 1
        );
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        if self.phase == Phase::Over {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        s.push_str(&format!(
            "În mână: J1 {} p., J2 {} p. | inimi {}\n",
            self.hand_pts[0],
            self.hand_pts[1],
            if self.broken { "sparte" } else { "nesparte" }
        ));
        if !self.trick.is_empty() {
            let cs: Vec<Card> = self.trick.iter().map(|&(_, c)| c).collect();
            s.push_str(&format!("Pe masă: {}\n", join_cards(&cs)));
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
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Deal => "imparte".into(),
            Move::Play(c) => c.text(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        // Adâncimea 0 = o singură mutare la TOATE nivelurile: la Hard/Expert adâncimea 1 ar extinde
        // răspunsurile adversarului din mâna lui reală (informație ascunsă).
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(rank: u8, suit: u8) -> Card {
        Card { rank, suit }
    }

    #[test]
    fn hearts_cannot_lead_until_broken_and_points() {
        let mut g = Inimi::with_seed(1);
        g.hands = [vec![c(5, 0), c(6, 3)], vec![c(12, 2), c(9, 0)]];
        g.trick.clear();
        g.side = Player::One;
        g.broken = false;
        assert_eq!(g.legal_moves(), vec![Move::Play(c(6, 3))]);
        g.apply(&Move::Play(c(6, 3)));
        // J2 nu are club: aruncă Dama de pică (13) pe levata lui J1
        g.apply(&Move::Play(c(12, 2)));
        assert_eq!(g.hand_pts, [13, 0]);
        assert_eq!(g.side, Player::One);
        // acum are doar o inimă: poate conduce
        g.hands[0] = vec![c(5, 0)];
        g.hands[1] = vec![c(9, 0)];
        g.apply(&Move::Play(c(5, 0)));
        g.apply(&Move::Play(c(9, 0)));
        assert_eq!(g.hand_pts, [13, 2]);
    }

    #[test]
    fn must_follow_suit() {
        let mut g = Inimi::with_seed(2);
        g.hands = [vec![c(5, 3), c(6, 1)], vec![c(9, 1), c(3, 3)]];
        g.trick = vec![(Player::One, c(7, 1))];
        g.side = Player::Two;
        assert_eq!(g.legal_moves(), vec![Move::Play(c(9, 1))]);
    }
}

#[cfg(test)]
mod a3_audit {
    use super::*;

    #[test]
    fn ai_ignores_hidden_cards_at_every_level() {
        crate::p3_comun::ai_blind_check(
            "inimi",
            Inimi::with_seed,
            |g, me, s| g.scramble_hidden(me, s),
            10,
        );
    }
}
