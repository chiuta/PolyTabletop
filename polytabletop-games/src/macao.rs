//! Macao (jocul românesc de cărți), 2 jucători, pachet de 52 de cărți.
//!
//! Reguli folosite (variantă tradițională, documentată aici):
//! * Fiecare primește 5 cărți; o carte obișnuită se întoarce pe masă. Începe Jucătorul 1.
//! * Se joacă o carte de aceeași **culoare** sau **valoare** ca ultima carte (ori
//!   culoarea cerută). Cine nu are carte potrivită trage **una** din stoc și o
//!   poate juca dacă se potrivește; altfel spune „pas”.
//! * **2** și **3**: adversarul trage 2 / 3 cărți, **dacă nu taie** cu un alt 2 sau 3
//!   (la orice culoare) — penalizările se **adună** (2+3 = 5 etc.). Cine nu taie
//!   trage tot (`trage N`) și mutarea trece la adversar.
//! * **4**: adversarul **sare** peste rând, dacă nu pune și el un 4. (`sar`)
//! * **7**: se poate juca peste orice carte și **alege culoarea** (`7h:s`).
//! * **As**: se joacă pe aceeași culoare sau peste alt As și **cere o culoare** (`Ah:d`).
//! * **Macao**: cine rămâne fără cărți câștigă. Nu se poate termina cu o carte
//!   specială (2, 3, 4, 7, As): cine are doar așa o carte trebuie să tragă.
//! * Stocul gol se reface din cărțile jucate (fără ultima). Siguranță: după
//!   1000 de mutări, câștigă cine are mai puține cărți (egal = remiză).
//!
//! Mutări: `7h`, `Kd` (carte), `7h:s` (7/As + culoarea cerută), `trage`, `trage 5`
//! (penalizare), `sar`, `pas`. Culori: h inimă roșie, d romb, s inimă neagră, c treflă.
//! Informația ascunsă și AI-ul: vezi `cards.rs`.

//!
//! Verificare surse (nov. 2026: stirileprotv.ro, bzi.ro, en.wikipedia.org/wiki/Macau_(card_game)):
//! regulile macao variază mult de la o casă la alta. Concordă cu codul: 5 cărți, tras unei
//! cărți și jucat dacă se potrivește, 2/3 cumulative, 4 = sărit, 7 schimbă culoarea, regula
//! „Macao” la o singură carte. Variante atestate dar neimplementate aici: As = sari peste
//! rând (aici As cere culoare), Valetul cere culoare, Popa de roșu +5, jokeri +5/+10, 4 anulează
//! penalizarea 2/3, 7 anulează penalizarea, strigarea „Macao!” cu penalitate de 5 cărți.
//! Fiind diferențe de convenție și nu erori clare, comportamentul a rămas neschimbat.

use crate::cards::{
    full_deck, handover_banner, join_cards, pl, shuffle, Card, Viewer, SUIT_LETTERS,
};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    /// Carte; `Some(s)` = culoarea aleasă pentru 7 / As.
    Play(Card, Option<u8>),
    /// Trage o carte (tură normală) sau penalizarea acumulată.
    Draw,
    /// Sari peste rând (4 neocolit).
    Skip,
    Pass,
}

const MAX_PLIES: u32 = 1000;

#[derive(Clone)]
pub struct Macao {
    deck: Vec<Card>,
    discard: Vec<Card>, // ultima = vârful
    hands: [Vec<Card>; 2],
    side: Player,
    /// Culoarea în vigoare.
    suit: u8,
    /// Cărți de tras acumulate prin 2 / 3.
    pen: u8,
    /// Mutarea curentă este ținta unui 4.
    skip: bool,
    /// Jucătorul curent a tras deja o carte în tura aceasta.
    drew: bool,
    fresh: [u8; 2],
    /// Cărți despre care e public că NU sunt în mâna jucătorului (dedus din trageri/sărituri).
    absent: [u64; 2],
    plies: u32,
    winner: Option<Player>,
    capped: bool,
    rng: Rng,
    viewer: Viewer,
    handover: bool,
}

impl Default for Macao {
    fn default() -> Self {
        Self::new()
    }
}

fn cbit(c: Card) -> u64 {
    1u64 << (c.suit as u64 * 13 + (c.rank as u64 - 2))
}

fn special(c: Card) -> bool {
    matches!(c.rank, 2 | 3 | 4 | 7 | 14)
}

impl Macao {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck = full_deck(2);
        shuffle(&mut deck, &mut rng);
        // prima carte de pe masă: una obișnuită
        let k = deck
            .iter()
            .rposition(|&c| !special(c))
            .expect("non-special card");
        let top = deck.remove(k);
        let mut g = Macao {
            deck,
            discard: vec![top],
            hands: [Vec::new(), Vec::new()],
            side: Player::One,
            suit: top.suit,
            pen: 0,
            skip: false,
            drew: false,
            fresh: [0, 0],
            absent: [0, 0],
            plies: 0,
            winner: None,
            capped: false,
            rng,
            viewer: Viewer::Fixed(Player::One),
            handover: false,
        };
        for _ in 0..5 {
            for p in [Player::One, Player::Two] {
                let c = g.deck.pop().expect("deck");
                g.hands[p.idx()].push(c);
            }
        }
        g
    }

    pub fn with_viewer(mut self, v: Viewer) -> Self {
        self.viewer = v;
        self
    }

    pub fn hot_seat(self) -> Self {
        self.with_viewer(Viewer::HotSeat)
    }

    fn top(&self) -> Card {
        *self.discard.last().expect("discard never empty")
    }

    fn available(&self) -> usize {
        self.deck.len() + self.discard.len() - 1
    }

    /// Trage până la `n` cărți (refăcând stocul din cărțile jucate).
    fn draw_n(&mut self, p: Player, n: usize) {
        for _ in 0..n {
            if self.deck.is_empty() {
                if self.discard.len() <= 1 {
                    break;
                }
                let top = self.discard.pop().expect("top");
                let mut rest = std::mem::take(&mut self.discard);
                shuffle(&mut rest, &mut self.rng);
                self.deck = rest;
                self.discard = vec![top];
            }
            let c = self.deck.pop().expect("card");
            self.hands[p.idx()].push(c);
            self.fresh[p.idx()] += 1;
        }
    }

    fn playable(&self, c: Card) -> bool {
        let hand = &self.hands[self.side.idx()];
        if hand.len() == 1 && special(c) {
            return false; // nu se termină cu carte specială
        }
        if self.skip {
            return c.rank == 4;
        }
        if self.pen > 0 {
            return c.rank == 2 || c.rank == 3;
        }
        c.rank == 7 || c.suit == self.suit || c.rank == self.top().rank
    }

    pub fn penalty(&self) -> u8 {
        self.pen
    }

    fn finish_check(&mut self) {
        if self.plies >= MAX_PLIES && self.winner.is_none() {
            self.capped = true;
        }
    }
}

const ROLLOUTS: usize = 150;
/// Procent de mutări aleatoare atribuite adversarului în simulări (model de adversar slab).
const OPP_RANDOM: usize = 100;
/// Greutățile politicii de simulare: duplicat de rang, 2/3, 4, 7, As, culoare păstrată, mână doar cu speciale.
const POLICY: [i32; 7] = [0, 4, 1, 17, 3, 2, 7];

impl Macao {
    /// Politică euristică pentru simulări (joacă cu mâna vizibilă a celui la mutare).
    fn pick(&self, mv: &[Move], rng: &mut Rng) -> Move {
        self.pick_w(mv, rng, &POLICY)
    }

    fn pick_w(&self, mv: &[Move], rng: &mut Rng, w: &[i32; 7]) -> Move {
        let hand = &self.hands[self.side.idx()];
        let mut best = (i32::MIN, mv[0]);
        for &m in mv {
            let mut sc = match m {
                Move::Play(c, ch) => {
                    let mut s = 20 + w[0] * hand.iter().filter(|x| x.rank == c.rank).count() as i32;
                    if hand.len() == 1 {
                        s += 1000;
                    }
                    s += match c.rank {
                        2 | 3 => w[1],
                        4 => w[2],
                        7 => -w[3],
                        14 => -w[4],
                        _ => 0,
                    };
                    let nsuit = ch.unwrap_or(c.suit);
                    s += w[5] * hand.iter().filter(|x| x.suit == nsuit && **x != c).count() as i32;
                    let rest_all_special =
                        hand.len() > 1 && hand.iter().filter(|x| **x != c).all(|x| special(*x));
                    if rest_all_special {
                        s -= w[6];
                    }
                    s
                }
                Move::Skip => 1,
                Move::Draw => 0,
                Move::Pass => -1,
            };
            sc = sc * 4 + rng.below(4) as i32;
            if sc > best.0 {
                best = (sc, m);
            }
        }
        best.1
    }

    /// Joacă până la sfârșit (sau limită); întoarce 1.0 / 0.5 / 0.0 din punctul de vedere al lui `p`.
    fn rollout(&mut self, p: Player, rng: &mut Rng) -> f64 {
        let cap = self.plies + 90;
        while self.winner.is_none() && !self.capped && self.plies < cap {
            let mv = Game::legal_moves(self);
            if mv.is_empty() {
                break;
            }
            let m = if self.side != p && rng.below(100) < OPP_RANDOM {
                mv[rng.below(mv.len())]
            } else {
                self.pick(&mv, rng)
            };
            self.apply(&m);
        }
        if let Some(w) = self.winner {
            return if w == p { 1.0 } else { 0.0 };
        }
        let d = self.hands[p.other().idx()].len() as f64 - self.hands[p.idx()].len() as f64;
        0.5 + (d / 20.0).clamp(-0.4, 0.4)
    }
}

impl Game for Macao {
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
        if self.winner.is_some() || self.capped {
            return Vec::new();
        }
        let hand = &self.hands[self.side.idx()];
        let mut v: Vec<Move> = Vec::new();
        for &c in hand {
            if !self.playable(c) {
                continue;
            }
            let wild = c.rank == 7 || (c.rank == 14 && !self.skip && self.pen == 0);
            let choices: Vec<Option<u8>> = if wild {
                (0..4).map(Some).collect()
            } else {
                vec![None]
            };
            for ch in choices {
                let m = Move::Play(c, ch);
                if !v.contains(&m) {
                    v.push(m);
                }
            }
        }
        if self.skip {
            v.push(Move::Skip);
        } else if self.pen > 0 {
            v.push(Move::Draw);
        } else if v.is_empty() {
            if !self.drew && self.available() > 0 {
                v.push(Move::Draw);
            } else {
                v.push(Move::Pass);
            }
        } else if self.drew {
            v.push(Move::Pass);
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        self.fresh = [0, 0];
        self.handover = false;
        self.plies += 1;
        let me = self.side;
        match *m {
            Move::Play(c, choice) => {
                let h = &mut self.hands[me.idx()];
                let pos = h.iter().position(|&x| x == c).expect("card in hand");
                h.remove(pos);
                self.discard.push(c);
                self.suit = choice.unwrap_or(c.suit);
                match c.rank {
                    2 => self.pen += 2,
                    3 => self.pen += 3,
                    4 => self.skip = true,
                    _ => {}
                }
                self.drew = false;
                if self.hands[me.idx()].is_empty() {
                    self.winner = Some(me); // macao!
                } else {
                    self.side = me.other();
                    self.handover = true;
                }
            }
            Move::Draw => {
                // Dovezi publice: cine trage de bunăvoie nu avea carte potrivită.
                let many = self.hands[me.idx()].len() > 1;
                self.absent[me.idx()] = 0;
                if many {
                    let (top, suit, pen) = (self.top(), self.suit, self.pen);
                    for c in full_deck(2) {
                        let no = if pen > 0 {
                            c.rank == 2 || c.rank == 3
                        } else {
                            c.rank == 7 || c.suit == suit || c.rank == top.rank
                        };
                        if no {
                            self.absent[me.idx()] |= cbit(c);
                        }
                    }
                }
                if self.pen > 0 {
                    let n = self.pen as usize;
                    self.pen = 0;
                    self.draw_n(me, n);
                    self.drew = false;
                    self.side = me.other();
                    self.handover = true;
                } else {
                    self.draw_n(me, 1);
                    self.drew = true;
                }
            }
            Move::Skip => {
                if self.hands[me.idx()].len() > 1 {
                    for c in full_deck(2) {
                        if c.rank == 4 {
                            self.absent[me.idx()] |= cbit(c);
                        }
                    }
                }
                self.skip = false;
                self.drew = false;
                self.side = me.other();
                self.handover = true;
            }
            Move::Pass => {
                self.drew = false;
                self.side = me.other();
                self.handover = true;
            }
        }
        self.finish_check();
    }

    fn outcome(&self) -> Option<Outcome> {
        if let Some(w) = self.winner {
            return Some(Outcome::Win(w));
        }
        if self.capped {
            let (a, b) = (self.hands[0].len(), self.hands[1].len());
            return Some(match a.cmp(&b) {
                std::cmp::Ordering::Less => Outcome::Win(Player::One),
                std::cmp::Ordering::Greater => Outcome::Win(Player::Two),
                std::cmp::Ordering::Equal => Outcome::Draw,
            });
        }
        None
    }

    /// Evaluare cinstită prin determinizare: citește doar informația vizibilă lui `p`
    /// (mâna lui `p` fără cărțile abia trase în simulare, stiva de pe masă, mărimile
    /// mâinilor și ale stocului). Mâna adversarului și stocul se eșantionează din cărțile
    /// necunoscute, apoi se joacă partide complete cu o politică euristică; rezultatul e
    /// ponderea de câștig medie (în miimi).
    fn evaluate(&self, p: Player) -> i32 {
        let q = p.other();
        let hand = &self.hands[p.idx()];
        let nfresh = (self.fresh[p.idx()] as usize).min(hand.len());
        let known = &hand[..hand.len() - nfresh];
        let nopp = self.hands[q.idx()].len();
        let mut pool = full_deck(2);
        pool.retain(|c| !known.contains(c) && !self.discard.contains(c));
        // sămânță derivată doar din informație vizibilă
        let mut seed: u64 = 0x9E37_79B9 ^ (self.plies as u64) << 20 ^ (nopp as u64) << 8;
        seed ^= (self.deck.len() as u64) << 32 ^ (self.suit as u64) << 4 ^ self.pen as u64;
        for c in known.iter().chain(self.discard.iter().rev().take(3)) {
            seed = seed.rotate_left(7) ^ (c.rank as u64 * 5 + c.suit as u64 + 1);
        }
        let mut rng = Rng::new(seed);
        let n = if pool.len() < nfresh + nopp {
            return 0;
        } else {
            ROLLOUTS
        };
        let mut total = 0.0f64;
        let abs = self.absent[q.idx()];
        for _ in 0..n {
            shuffle(&mut pool, &mut rng);
            // cărțile despre care știm că adversarul nu le are merg ultimele
            let b = &mut pool[nfresh..];
            let mut ok: Vec<Card> = b.iter().copied().filter(|c| abs & cbit(*c) == 0).collect();
            ok.extend(b.iter().copied().filter(|c| abs & cbit(*c) != 0));
            b.copy_from_slice(&ok);
            let mut g = self.clone();
            let mut mine: Vec<Card> = known.to_vec();
            mine.extend_from_slice(&pool[..nfresh]);
            g.hands[p.idx()] = mine;
            g.hands[q.idx()] = pool[nfresh..nfresh + nopp].to_vec();
            g.deck = pool[nfresh + nopp..].to_vec();
            g.fresh = [0, 0];
            g.rng = Rng::new(rng.next_u64());
            total += g.rollout(p, &mut rng);
        }
        (1000.0 * total / n as f64) as i32 - 500
    }

    fn render(&self) -> String {
        let mut s = String::new();
        if self.handover && self.outcome().is_none() {
            if let Viewer::HotSeat = self.viewer {
                s.push_str(&handover_banner(self.side));
            }
        }
        let t = self.top();
        s.push_str(&format!(
            "Macao — stoc: {} cărți | pe masă: {} | culoarea cerută: {}\n",
            self.deck.len(),
            t.text(),
            SUIT_LETTERS[self.suit as usize]
        ));
        if self.pen > 0 {
            s.push_str(&format!("Penalizare în așteptare: {} cărți\n", self.pen));
        }
        if self.skip {
            s.push_str("Un 4 a fost jucat: sari peste rând sau pune și tu un 4.\n");
        }
        if let Some(w) = self.winner {
            s.push_str(&format!("MACAO! Jucătorul {} a rămas fără cărți.\n", pl(w)));
            return s;
        }
        if self.capped {
            s.push_str("Limită de mutări atinsă: câștigă cine are mai puține cărți.\n");
            return s;
        }
        let v = self.viewer.who(self.side);
        s.push_str(&format!(
            "Mâna Jucătorului {}: {}\n",
            pl(v),
            join_cards(&self.hands[v.idx()])
        ));
        s.push_str(&format!(
            "Jucătorul {}: {} cărți\n",
            pl(v.other()),
            self.hands[v.other().idx()].len()
        ));
        s.push_str(&format!("La mutare: Jucătorul {}\n", pl(self.side)));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Play(c, None) => c.text(),
            Move::Play(c, Some(s)) => format!("{}:{}", c.text(), SUIT_LETTERS[*s as usize]),
            Move::Draw if self.pen > 0 => format!("trage {}", self.pen),
            Move::Draw => "trage".into(),
            Move::Skip => "sar".into(),
            Move::Pass => "pas".into(),
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

    fn setup(h1: Vec<Card>, h2: Vec<Card>, top: Card) -> Macao {
        let mut g = Macao::with_seed(1);
        g.hands = [h1, h2];
        g.discard = vec![top];
        g.suit = top.suit;
        g.deck = (0..10).map(|i| c(5 + i % 3, i % 4)).collect();
        g
    }

    #[test]
    fn deal_five_each_and_52_cards() {
        let g = Macao::with_seed(9);
        assert_eq!(g.hands[0].len(), 5);
        assert_eq!(g.hands[1].len(), 5);
        assert_eq!(g.deck.len() + g.discard.len() + 10, 52);
        assert!(!special(g.top()));
    }

    #[test]
    fn follows_suit_or_rank_and_seven_is_wild() {
        let g = setup(
            vec![c(9, 0), c(9, 1), c(6, 2), c(7, 3), c(12, 3)],
            vec![c(5, 0)],
            c(6, 0),
        );
        let m = Game::legal_moves(&g);
        assert!(m.contains(&Move::Play(c(9, 0), None))); // same suit
        assert!(m.contains(&Move::Play(c(6, 2), None))); // same rank
        assert!(!m.contains(&Move::Play(c(9, 1), None)));
        assert!(!m.contains(&Move::Play(c(12, 3), None)));
        for s in 0..4 {
            assert!(m.contains(&Move::Play(c(7, 3), Some(s))));
        }
        assert!(!m.contains(&Move::Draw));
    }

    #[test]
    fn seven_chooses_suit() {
        let mut g = setup(vec![c(7, 1), c(9, 2)], vec![c(5, 2), c(5, 0)], c(6, 0));
        g.apply(&Move::Play(c(7, 1), Some(2)));
        assert_eq!(g.suit, 2);
        let m = Game::legal_moves(&g);
        assert!(m.contains(&Move::Play(c(5, 2), None)));
        assert!(!m.contains(&Move::Play(c(5, 0), None)));
    }

    #[test]
    fn two_and_three_accumulate() {
        let mut g = setup(
            vec![c(2, 0), c(9, 0), c(9, 1)],
            vec![c(3, 1), c(5, 2), c(5, 3)],
            c(6, 0),
        );
        g.apply(&Move::Play(c(2, 0), None));
        assert_eq!(g.penalty(), 2);
        let m = Game::legal_moves(&g);
        assert!(m.contains(&Move::Play(c(3, 1), None))); // any suit may counter
        assert!(m.contains(&Move::Draw));
        assert!(!m.contains(&Move::Play(c(5, 2), None)));
        g.apply(&Move::Play(c(3, 1), None));
        assert_eq!(g.penalty(), 5);
        assert_eq!(g.to_move(), Player::One);
        g.apply(&Move::Draw);
        assert_eq!(g.hands[0].len(), 2 + 5);
        assert_eq!(g.penalty(), 0);
        assert_eq!(g.to_move(), Player::Two);
    }

    #[test]
    fn four_skips_unless_countered() {
        let mut g = setup(vec![c(4, 0), c(9, 0)], vec![c(5, 2), c(4, 3)], c(6, 0));
        g.apply(&Move::Play(c(4, 0), None));
        let m = Game::legal_moves(&g);
        assert!(m.contains(&Move::Skip));
        assert!(m.contains(&Move::Play(c(4, 3), None)));
        assert!(!m.contains(&Move::Play(c(5, 2), None)));
        g.apply(&Move::Skip);
        assert_eq!(g.to_move(), Player::One); // played again
        assert!(!g.skip);
    }

    #[test]
    fn ace_asks_a_suit_and_needs_matching_suit() {
        let g = setup(vec![c(14, 1), c(14, 0), c(9, 2)], vec![c(5, 0)], c(6, 0));
        let m = Game::legal_moves(&g);
        assert!(!m.contains(&Move::Play(c(14, 1), Some(2)))); // wrong suit
        assert!(m.contains(&Move::Play(c(14, 0), Some(2))));
    }

    #[test]
    fn draw_one_then_play_or_pass() {
        let mut g = setup(vec![c(9, 1), c(9, 2)], vec![c(5, 0)], c(6, 0));
        assert_eq!(Game::legal_moves(&g), vec![Move::Draw]);
        g.deck.push(c(8, 0)); // will be drawn (playable: same suit)
        g.apply(&Move::Draw);
        assert_eq!(g.to_move(), Player::One);
        let m = Game::legal_moves(&g);
        assert!(m.contains(&Move::Play(c(8, 0), None)));
        assert!(m.contains(&Move::Pass));
        assert!(!m.contains(&Move::Draw));
    }

    #[test]
    fn macao_wins_but_not_with_special_last_card() {
        let g = setup(vec![c(2, 0)], vec![c(5, 0), c(6, 1)], c(6, 0));
        // last card is a 2: cannot be played, must draw
        assert_eq!(Game::legal_moves(&g), vec![Move::Draw]);
        let mut g = setup(vec![c(9, 0)], vec![c(5, 0), c(6, 1)], c(6, 0));
        g.apply(&Move::Play(c(9, 0), None));
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
        assert!(Game::legal_moves(&g).is_empty());
    }

    #[test]
    fn deck_is_rebuilt_from_discards() {
        let mut g = setup(vec![c(9, 1)], vec![c(5, 0)], c(6, 0));
        g.deck.clear();
        g.discard = vec![c(8, 1), c(9, 3), c(10, 2), c(6, 0)];
        g.apply(&Move::Draw);
        assert_eq!(g.hands[0].len(), 2);
        assert_eq!(g.deck.len(), 2);
        assert_eq!(g.top(), c(6, 0));
    }

    #[test]
    fn render_hides_opponent_hand() {
        let g = Macao::with_seed(5);
        let r = Game::render(&g);
        for c in &g.hands[1] {
            assert!(!r.split_whitespace().any(|t| t == c.text()), "{r}");
        }
        for c in &g.hands[0] {
            assert!(r.split_whitespace().any(|t| t == c.text()), "{r}");
        }
        let h = Macao::with_seed(5).hot_seat();
        let mut h2 = h.clone();
        let m = Game::legal_moves(&h2)[0];
        h2.apply(&m);
        if h2.to_move() == Player::Two {
            let r = Game::render(&h2);
            assert!(r.contains("Predă ecranul"));
            for c in &h2.hands[0] {
                assert!(!r.split_whitespace().any(|t| t == c.text()), "{r}");
            }
        }
    }

    #[test]
    fn ai_is_blind_to_hidden_information() {
        for seed in 1..60u64 {
            let mut g = Macao::with_seed(seed);
            let mut rng = Rng::new(seed);
            for _ in 0..(seed as usize % 30) {
                if g.outcome().is_some() {
                    break;
                }
                let mv = Game::legal_moves(&g);
                g.apply(&mv[rng.below(mv.len())]);
            }
            if g.outcome().is_some() {
                continue;
            }
            let me = g.side.idx();
            let mut h = g.clone();
            let mut pool: Vec<Card> = h.deck.clone();
            pool.extend(&h.hands[1 - me]);
            shuffle(&mut pool, &mut rng);
            let n = h.hands[1 - me].len();
            h.hands[1 - me] = pool.drain(..n).collect();
            h.deck = pool;
            assert_eq!(
                polytabletop_core::ai::best_move(&g, 3),
                polytabletop_core::ai::best_move(&h, 3),
                "AI decision depends on hidden cards (seed {seed})"
            );
        }
    }

    #[test]
    fn random_games_terminate_and_conserve_cards() {
        for seed in 1..40u64 {
            let mut g = Macao::with_seed(seed);
            let mut rng = Rng::new(seed ^ 0x55);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = Game::legal_moves(&g);
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
                let total = g.deck.len() + g.discard.len() + g.hands[0].len() + g.hands[1].len();
                assert_eq!(total, 52);
                n += 1;
                assert!(n <= 1100);
            }
        }
    }

    /// Reglaj offline al politicii de simulare (rulează manual): `cargo test --release -p polytabletop-games macao::tests::tune -- --ignored --nocapture`.
    #[test]
    #[ignore]
    fn tune_policy_weights() {
        fn score(w: &[i32; 7], games: u64) -> f64 {
            let mut wins = 0;
            for seed in 1..=games {
                let mut g = Macao::with_seed(seed);
                let me = if seed % 2 == 0 {
                    Player::One
                } else {
                    Player::Two
                };
                let mut rng = Rng::new(seed * 7 + 1);
                while g.outcome().is_none() {
                    let mv = Game::legal_moves(&g);
                    let m = if g.side == me {
                        g.pick_w(&mv, &mut rng, w)
                    } else {
                        mv[rng.below(mv.len())]
                    };
                    g.apply(&m);
                }
                if g.outcome() == Some(Outcome::Win(me)) {
                    wins += 1;
                }
            }
            100.0 * wins as f64 / games as f64
        }
        let mut best = POLICY;
        let mut best_s = score(&best, 4000);
        println!("start {best:?} {best_s:.1}");
        let mut rng = Rng::new(99);
        for it in 0..60 {
            let mut c = best;
            let k = rng.below(7);
            c[k] = (c[k] + rng.below(13) as i32 - 6).clamp(0, 40);
            let sc = score(&c, 4000);
            if sc > best_s {
                best = c;
                best_s = sc;
                println!("it {it}: {best:?} {best_s:.1}");
            }
        }
        println!("final {best:?} {best_s:.1}");
    }
}
