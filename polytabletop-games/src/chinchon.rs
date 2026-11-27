//! Chinchón (Spania / America Latină), 2 jucători, baraja spaniolă de 40 de cărți.
//!
//! Reguli folosite (descrierile publice ale jocului):
//! * 7 cărți fiecăruia; prima carte rămasă se întoarce ca talon (grămada de aruncări). Începe
//!   jucătorul care nu a împărțit. Un rând = tragi (`stoc` sau `talon`) și arunci o carte
//!   (`5o`); nu poți arunca cartea pe care ai luat-o din talon în același rând.
//! * Combinații (cel puțin 3 cărți): **seturi** (același rang) și **secvențe** (aceeași culoare, rang
//!   consecutiv; ordinea 1-2-3-4-5-6-7-10-11-12 și **7 nu se leagă de 10**). Valoarea cărților nepotrivite:
//!   1–7 = valoarea, figurile (10, 11, 12) = 10.
//! * **Închidere** (`inchide-5o`: aruncarea cărții se face cu fața în jos și mâna se arată):
//!   cele 7 cărți rămase trebuie să aibă cel puțin 6 cărți în combinații, iar cartea liberă (dacă
//!   există) să aibă cel mult 5 puncte; nu se poate închide în primul rând al jucătorului.
//!   Cel care închide primește punctele cărții libere sau **−10** dacă toate cele 7 cărți sunt
//!   potrivite; cu **chinchón** (secvență de 7 cărți 1–7 din aceeași culoare) câștigă jocul pe
//!   loc. Adversarul își aranjează cât mai bine mâna și poate **lipi** cărți libere pe combinațiile
//!   celui care a închis doar dacă acesta a închis cu 6 cărți (nu cu 7); restul cărților libere îi
//!   dau puncte.
//! * Când se epuizează stocul, talonul (fără cartea de deasupra) se amestecă drept stoc nou;
//!   după a patra epuizare mâna se încheie fără închidere și fiecare adaugă punctele cărților libere.
//! * Se joacă până când cineva ajunge la **100** de puncte: acela pierde, câștigă cel cu mai puține
//!   (egal = remiză). Fără jokeri; nu se joacă varianta cu 48 de cărți.
//!
//! Mutări: `stoc`, `talon`, `5o`, `inchide-5o`, `urmatoarea`.
//!
//! Informație ascunsă ca la `scopa.rs`: `render` arată doar mâna jucătorului vizualizat (și
//! combinațiile ei); AI-ul (`AlphaBeta(0)`) citește doar propria mână și talonul; cartea trasă
//! din stoc de jucătorul evaluat e ignorată (nu se „trișează” pe stoc).

use crate::baraja_es::{deck, join, pos, sorted, suit, text};
use crate::cards::Viewer;
use crate::cards52::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const LIMIT: i32 = 100;
const MAX_RECYCLES: u8 = 4;

/// Valoarea de punctaj.
pub fn points(c: u8) -> i32 {
    let p = pos(c);
    if p <= 6 {
        p as i32 + 1
    } else {
        10
    }
}

/// Poziția în secvență (7 și 10 nu sunt consecutive).
fn rp(c: u8) -> u8 {
    let p = pos(c);
    if p <= 6 {
        p
    } else {
        p + 1
    }
}

fn is_meld(cards: &[u8]) -> bool {
    if cards.len() < 3 {
        return false;
    }
    if cards.iter().all(|&c| pos(c) == pos(cards[0])) {
        return cards.len() <= 4;
    }
    if cards.iter().all(|&c| suit(c) == suit(cards[0])) {
        let mut r: Vec<u8> = cards.iter().map(|&c| rp(c)).collect();
        r.sort_unstable();
        return r.windows(2).all(|w| w[1] == w[0] + 1);
    }
    false
}

/// Cea mai bună partiție: (puncte libere, cărți în combinații, combinațiile).
pub fn best_partition(cards: &[u8]) -> (i32, usize, Vec<Vec<u8>>) {
    let n = cards.len();
    let mut cands: Vec<u16> = Vec::new();
    for mask in 1u16..(1 << n) {
        if mask.count_ones() < 3 {
            continue;
        }
        let sel: Vec<u8> = (0..n)
            .filter(|i| mask & (1 << i) != 0)
            .map(|i| cards[i])
            .collect();
        if is_meld(&sel) {
            cands.push(mask);
        }
    }
    fn rec(
        cards: &[u8],
        cands: &[u16],
        used: u16,
        dead: i32,
        melds: &mut Vec<u16>,
        best: &mut (i32, usize, Vec<u16>),
    ) {
        let n = cards.len();
        let first = (0..n).find(|i| used & (1 << i) == 0);
        let Some(i) = first else {
            let melded: usize = melds.iter().map(|m| m.count_ones() as usize).sum();
            if dead < best.0 || (dead == best.0 && melded > best.1) {
                *best = (dead, melded, melds.clone());
            }
            return;
        };
        for &m in cands {
            if m & (1 << i) != 0 && m & used == 0 {
                melds.push(m);
                rec(cards, cands, used | m, dead, melds, best);
                melds.pop();
            }
        }
        rec(
            cards,
            cands,
            used | (1 << i),
            dead + points(cards[i]),
            melds,
            best,
        );
    }
    let mut best = (i32::MAX, 0usize, Vec::new());
    rec(cards, &cands, 0, 0, &mut Vec::new(), &mut best);
    let melds = best
        .2
        .iter()
        .map(|&m| {
            (0..n)
                .filter(|i| m & (1 << i) != 0)
                .map(|i| cards[i])
                .collect()
        })
        .collect();
    (best.0, best.1, melds)
}

fn is_chinchon(cards: &[u8]) -> bool {
    cards.len() == 7
        && cards.iter().all(|&c| suit(c) == suit(cards[0]))
        && sorted(cards.iter().map(|&c| pos(c)).collect()) == vec![0, 1, 2, 3, 4, 5, 6]
}

/// Poate închide cu aceste 7 cărți? (≥ 6 în combinații, restul ≤ 5 puncte)
pub fn can_close(seven: &[u8]) -> bool {
    let (dead, melded, _) = best_partition(seven);
    melded >= 6 && dead <= 5
}

#[derive(Clone, PartialEq, Debug)]
pub enum Move {
    DrawStock,
    DrawDiscard,
    Discard(u8),
    Close(u8),
    Next,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Draw,
    Discard,
    RoundOver,
}

#[derive(Clone)]
pub struct Chinchon {
    rng: Rng,
    viewer: Viewer,
    stock: Vec<u8>,
    discard: Vec<u8>,
    hands: [Vec<u8>; 2],
    side: Player,
    dealer: Player,
    phase: Phase,
    score: [i32; 2],
    prev_score: [i32; 2],
    turns: [u8; 2],
    recycles: u8,
    /// Cartea luată din talon în rândul curent (nu se poate arunca).
    from_discard: Option<u8>,
    /// Cartea trasă din stoc în rândul curent (ignorată de `evaluate`).
    fresh: Option<u8>,
    closer: Option<Player>,
    winner: Option<Outcome>,
    log: String,
}

impl Default for Chinchon {
    fn default() -> Self {
        Self::new()
    }
}

impl Chinchon {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Chinchon {
            rng: Rng::new(seed),
            viewer: Viewer::Fixed(Player::One),
            stock: Vec::new(),
            discard: Vec::new(),
            hands: [Vec::new(), Vec::new()],
            side: Player::One,
            dealer: Player::Two,
            phase: Phase::Draw,
            score: [0; 2],
            prev_score: [0; 2],
            turns: [0; 2],
            recycles: 0,
            from_discard: None,
            fresh: None,
            closer: None,
            winner: None,
            log: String::new(),
        };
        g.new_round();
        g
    }

    pub fn scores(&self) -> (i32, i32) {
        (self.score[0], self.score[1])
    }

    fn new_round(&mut self) {
        let mut d = deck(&mut self.rng);
        self.hands = [Vec::new(), Vec::new()];
        for _ in 0..7 {
            self.hands[0].push(d.pop().unwrap());
            self.hands[1].push(d.pop().unwrap());
        }
        self.discard = vec![d.pop().unwrap()];
        self.stock = d;
        self.side = self.dealer.other();
        self.phase = Phase::Draw;
        self.turns = [0; 2];
        self.recycles = 0;
        self.from_discard = None;
        self.fresh = None;
        self.closer = None;
        self.prev_score = self.score;
    }

    fn end_match_check(&mut self) {
        if self.winner.is_some() {
            return;
        }
        if self.score[0] >= LIMIT || self.score[1] >= LIMIT {
            self.winner = Some(match self.score[0].cmp(&self.score[1]) {
                std::cmp::Ordering::Less => Outcome::Win(Player::One),
                std::cmp::Ordering::Greater => Outcome::Win(Player::Two),
                std::cmp::Ordering::Equal => Outcome::Draw,
            });
        }
    }

    /// Închidere: `me` aruncă `c` și închide.
    fn close_round(&mut self, me: Player, c: u8) {
        let i = me.idx();
        let o = 1 - i;
        self.discard.push(c);
        let mine = self.hands[i].clone();
        self.closer = Some(me);
        if is_chinchon(&mine) {
            self.winner = Some(Outcome::Win(me));
            self.log = format!("CHINCHÓN! J{} câștigă jocul.", i + 1);
            self.phase = Phase::RoundOver;
            return;
        }
        let (dead, melded, melds) = best_partition(&mine);
        let my_pts = if dead == 0 { -10 } else { dead };
        // adversarul: aranjare optimă + lipire pe combinațiile celui care a închis (doar dacă a închis cu 6)
        let (_, _, opp_melds) = best_partition(&self.hands[o]);
        let mut free: Vec<u8> = self.hands[o]
            .iter()
            .copied()
            .filter(|c| !opp_melds.iter().any(|m| m.contains(c)))
            .collect();
        if melded == 6 {
            let mut tables = melds.clone();
            loop {
                free.sort_by_key(|&c| -points(c));
                let mut placed = None;
                for (fi, &c) in free.iter().enumerate() {
                    for (mi, m) in tables.iter().enumerate() {
                        let mut t = m.clone();
                        t.push(c);
                        if is_meld(&t) {
                            placed = Some((fi, mi));
                            break;
                        }
                    }
                    if placed.is_some() {
                        break;
                    }
                }
                match placed {
                    Some((fi, mi)) => {
                        let c = free.remove(fi);
                        tables[mi].push(c);
                    }
                    None => break,
                }
            }
        }
        let opp_pts: i32 = free.iter().map(|&c| points(c)).sum();
        self.score[i] += my_pts;
        self.score[o] += opp_pts;
        self.log = format!(
            "J{} închide ({:+} puncte); J{} rămâne cu {} puncte libere.",
            i + 1,
            my_pts,
            o + 1,
            opp_pts
        );
        self.phase = Phase::RoundOver;
        self.end_match_check();
    }

    fn exhausted_round(&mut self) {
        for i in 0..2 {
            let (dead, _, _) = best_partition(&self.hands[i]);
            self.score[i] += dead;
        }
        self.log = "Stocul s-a epuizat a patra oară: mâna se încheie fără închidere.".into();
        self.phase = Phase::RoundOver;
        self.end_match_check();
    }
}

impl Game for Chinchon {
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
        if self.winner.is_some() {
            return Vec::new();
        }
        match self.phase {
            Phase::RoundOver => vec![Move::Next],
            Phase::Draw => vec![Move::DrawStock, Move::DrawDiscard],
            Phase::Discard => {
                let hand = &self.hands[self.side.idx()];
                let mut v = Vec::new();
                for &c in sorted(hand.clone()).iter() {
                    if self.from_discard == Some(c) {
                        continue;
                    }
                    v.push(Move::Discard(c));
                }
                if self.turns[self.side.idx()] >= 1 {
                    for &c in sorted(hand.clone()).iter() {
                        if self.from_discard == Some(c) {
                            continue;
                        }
                        let rest: Vec<u8> = hand.iter().copied().filter(|&x| x != c).collect();
                        if can_close(&rest) {
                            v.push(Move::Close(c));
                        }
                    }
                }
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        let me = self.side;
        let i = me.idx();
        match m {
            Move::Next => {
                self.dealer = self.dealer.other();
                self.log.clear();
                self.new_round();
            }
            Move::DrawStock => {
                let c = self.stock.pop().expect("stock");
                self.hands[i].push(c);
                self.fresh = Some(c);
                self.from_discard = None;
                self.phase = Phase::Discard;
            }
            Move::DrawDiscard => {
                let c = self.discard.pop().expect("discard");
                self.hands[i].push(c);
                self.from_discard = Some(c);
                self.fresh = None;
                self.phase = Phase::Discard;
            }
            Move::Discard(c) => {
                let p = self.hands[i].iter().position(|x| x == c).expect("card");
                self.hands[i].remove(p);
                self.discard.push(*c);
                self.turns[i] += 1;
                self.fresh = None;
                self.from_discard = None;
                self.side = me.other();
                self.phase = Phase::Draw;
                if self.stock.is_empty() {
                    if self.recycles >= MAX_RECYCLES || self.discard.len() < 2 {
                        self.exhausted_round();
                    } else {
                        let top = self.discard.pop().unwrap();
                        let mut pile = std::mem::take(&mut self.discard);
                        shuffle(&mut pile, &mut self.rng);
                        self.stock = pile;
                        self.discard = vec![top];
                        self.recycles += 1;
                    }
                }
            }
            Move::Close(c) => {
                let p = self.hands[i].iter().position(|x| x == c).expect("card");
                self.hands[i].remove(p);
                self.fresh = None;
                self.from_discard = None;
                self.turns[i] += 1;
                self.close_round(me, *c);
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.winner
    }

    /// Citește doar mâna proprie (fără cartea tocmai trasă din stoc), talonul și scorurile.
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let o = 1 - me;
        if self.phase == Phase::RoundOver {
            // rezultatul exact depinde de mâna ascunsă a adversarului: nu îl citim
            let base = (self.prev_score[o] - self.prev_score[me]) * 10;
            return base
                + match self.closer {
                    Some(c) if c == p => 400,
                    Some(_) => -400,
                    None => 0,
                };
        }
        let mut known: Vec<u8> = self.hands[me].clone();
        if self.side == p {
            if let Some(f) = self.fresh {
                if let Some(ix) = known.iter().position(|&x| x == f) {
                    known.remove(ix);
                }
            }
        }
        let hand_val = if known.len() == 8 {
            // după tragerea din talon: cea mai bună aruncare
            known
                .iter()
                .map(|&c| {
                    let rest: Vec<u8> = known.iter().copied().filter(|&x| x != c).collect();
                    -best_partition(&rest).0
                })
                .max()
                .unwrap_or(0)
        } else if known.len() == 7 && self.side == p && self.fresh.is_some() {
            // cartea din stoc necunoscută: speranță a îmbunătățirii
            -best_partition(&known).0 + 4
        } else {
            -best_partition(&known).0
        };
        hand_val * 10 + (self.prev_score[o] - self.prev_score[me]) * 10
    }

    fn render(&self) -> String {
        let v = self.viewer.who(self.side);
        let reveal = self.phase == Phase::RoundOver || self.winner.is_some();
        let mut s = format!(
            "Chinchón — până la {LIMIT} (pierde cine ajunge primul): J1 {} – J2 {}\n",
            self.score[0], self.score[1]
        );
        s.push_str(&format!(
            "Talon: {}   Stoc: {} cărți (epuizat de {} ori)\n",
            self.discard
                .last()
                .map(|&c| text(c))
                .unwrap_or_else(|| "—".into()),
            self.stock.len(),
            self.recycles
        ));
        if reveal {
            for i in 0..2 {
                s.push_str(&format!(
                    "Mâna J{}: {}\n",
                    i + 1,
                    join(&sorted(self.hands[i].clone()))
                ));
            }
        } else {
            let h = sorted(self.hands[v.idx()].clone());
            s.push_str(&format!(
                "Mâna ta (Jucător {}): {}\n",
                v.idx() + 1,
                join(&h)
            ));
            let (dead, _, melds) = best_partition(&h);
            let m: Vec<String> = melds.iter().map(|m| join(&sorted(m.clone()))).collect();
            s.push_str(&format!(
                "Combinații: {} — puncte libere: {}\n",
                if m.is_empty() {
                    "—".into()
                } else {
                    m.join(" | ")
                },
                dead
            ));
            s.push_str(&format!(
                "J{} (ascuns) ține {} cărți.\n",
                v.other().idx() + 1,
                self.hands[v.other().idx()].len()
            ));
        }
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        match (self.winner, self.phase) {
            (Some(Outcome::Win(p)), _) => {
                s.push_str(&format!("Joc încheiat: Jucător {} câștigă.\n", p.idx() + 1))
            }
            (Some(Outcome::Draw), _) => s.push_str("Joc încheiat: remiză.\n"),
            (_, Phase::RoundOver) => s.push_str("Mâna s-a încheiat. Scrie «urmatoarea».\n"),
            (_, Phase::Draw) => s.push_str(&format!(
                "La mutare: Jucător {} — stoc / talon\n",
                self.side.idx() + 1
            )),
            (_, Phase::Discard) => s.push_str(&format!(
                "La mutare: Jucător {} — aruncă o carte (ex. 5o) sau inchide-5o\n",
                self.side.idx() + 1
            )),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::DrawStock => "stoc".into(),
            Move::DrawDiscard => "talon".into(),
            Move::Discard(c) => text(*c),
            Move::Close(c) => format!("inchide-{}", text(*c)),
            Move::Next => "urmatoarea".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // k(suit, rank) cu rank 1..=7 și 8 = sota, 9 = caballo, 10 = rey
    fn k(suit: u8, rank: u8) -> u8 {
        suit * 10 + rank - 1
    }

    #[test]
    fn seven_does_not_connect_to_sota() {
        assert!(is_meld(&[k(0, 5), k(0, 6), k(0, 7)]));
        assert!(!is_meld(&[k(0, 6), k(0, 7), k(0, 8)])); // 6-7-sota
        assert!(is_meld(&[k(0, 8), k(0, 9), k(0, 10)])); // sota-caballo-rey
        assert!(is_meld(&[k(0, 3), k(1, 3), k(2, 3)])); // set
        assert!(is_meld(&[k(0, 3), k(1, 3), k(2, 3), k(3, 3)]));
        assert!(!is_meld(&[k(0, 3), k(1, 3)]));
        assert!(!is_meld(&[k(0, 3), k(1, 4), k(2, 5)]));
    }

    #[test]
    fn best_partition_prefers_fewer_points() {
        // 4-5-6 de oro, trei de 3, rest: sota (10)
        let h = vec![
            k(0, 4),
            k(0, 5),
            k(0, 6),
            k(0, 3),
            k(1, 3),
            k(2, 3),
            k(3, 8),
        ];
        let (dead, melded, melds) = best_partition(&h);
        assert_eq!(dead, 10);
        assert_eq!(melded, 6);
        assert_eq!(melds.len(), 2);
        // 3 de oro poate intra și în secvența 3-4-5-6, dar atunci setul de 3 se destramă
        let h2 = vec![
            k(0, 3),
            k(0, 4),
            k(0, 5),
            k(0, 6),
            k(1, 3),
            k(2, 3),
            k(3, 1),
        ];
        let (dead2, _, _) = best_partition(&h2);
        assert_eq!(dead2, 1); // set 3-3-3 + secvența 4-5-6 bate secvența 3-4-5-6 (7 puncte libere)
    }

    #[test]
    fn closing_needs_six_melded_and_at_most_five_points() {
        let ok = vec![
            k(0, 4),
            k(0, 5),
            k(0, 6),
            k(0, 3),
            k(1, 3),
            k(2, 3),
            k(3, 5),
        ]; // liber 5
        assert!(can_close(&ok));
        let high = vec![
            k(0, 4),
            k(0, 5),
            k(0, 6),
            k(0, 3),
            k(1, 3),
            k(2, 3),
            k(3, 6),
        ]; // liber 6
        assert!(!can_close(&high));
        let few = vec![
            k(0, 4),
            k(0, 5),
            k(0, 6),
            k(1, 3),
            k(2, 3),
            k(3, 1),
            k(0, 1),
        ];
        assert!(!can_close(&few));
    }

    fn setup() -> Chinchon {
        let mut g = Chinchon::with_seed(1);
        g.hands = [Vec::new(), Vec::new()];
        g.stock = vec![k(3, 4); 5];
        g.discard = vec![k(3, 2)];
        g.side = Player::One;
        g.phase = Phase::Discard;
        g.turns = [1, 1];
        g.fresh = None;
        g.from_discard = None;
        g.closer = None;
        g.winner = None;
        g.score = [0; 2];
        g.prev_score = [0; 2];
        g
    }

    #[test]
    fn closing_scores_minus_ten_and_penalises_the_opponent() {
        let mut g = setup();
        // J1 are 8 cărți: două combinații (4+3) + o carte de aruncat
        g.hands[0] = vec![
            k(0, 4),
            k(0, 5),
            k(0, 6),
            k(0, 7),
            k(1, 3),
            k(2, 3),
            k(3, 3),
            k(3, 8),
        ];
        g.hands[1] = vec![
            k(0, 1),
            k(1, 8),
            k(2, 9),
            k(3, 10),
            k(0, 2),
            k(1, 5),
            k(2, 6),
        ];
        assert!(g.legal_moves().contains(&Move::Close(k(3, 8))));
        g.apply(&Move::Close(k(3, 8)));
        assert_eq!(g.score[0], -10);
        // J2: 1+10+10+10+2+5+6 = 44
        assert_eq!(g.score[1], 44);
        assert_eq!(g.phase, Phase::RoundOver);
    }

    #[test]
    fn layoff_only_when_closer_melded_six_cards() {
        let mut g = setup();
        // J1 închide cu 6 cărți în combinații + o liberă de 2: 4-5-6 de oro și trei de 3, liber 2b
        g.hands[0] = vec![
            k(0, 4),
            k(0, 5),
            k(0, 6),
            k(0, 3),
            k(1, 3),
            k(2, 3),
            k(3, 2),
            k(3, 8),
        ];
        // J2 are 7 de oro (lipește la 4-5-6 -> 4-5-6-7) și 3 de bastos (lipește la setul de 3)
        g.hands[1] = vec![
            k(0, 7),
            k(3, 3),
            k(1, 1),
            k(2, 8),
            k(3, 9),
            k(1, 9),
            k(2, 10),
        ];
        g.apply(&Move::Close(k(3, 8)));
        // J1: liber 2 ; J2: 1 + 10*4 = 41 (7 și 3 lipite)
        assert_eq!(g.score[0], 2);
        assert_eq!(g.score[1], 1 + 10 + 10 + 10 + 10);
    }

    #[test]
    fn chinchon_wins_the_game() {
        let mut g = setup();
        g.hands[0] = (1..=7).map(|r| k(2, r)).chain([k(0, 9)]).collect();
        g.hands[1] = vec![
            k(0, 1),
            k(1, 8),
            k(2, 9),
            k(3, 10),
            k(0, 2),
            k(1, 5),
            k(2, 6),
        ];
        g.apply(&Move::Close(k(0, 9)));
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn cannot_close_on_the_first_turn_nor_discard_the_taken_talon_card() {
        let mut g = setup();
        g.turns = [0, 0];
        g.hands[0] = vec![
            k(0, 4),
            k(0, 5),
            k(0, 6),
            k(0, 3),
            k(1, 3),
            k(2, 3),
            k(3, 5),
            k(3, 6),
        ];
        assert!(!g.legal_moves().iter().any(|m| matches!(m, Move::Close(_))));
        g.turns = [1, 1];
        g.from_discard = Some(k(3, 6));
        let mv = g.legal_moves();
        assert!(!mv.contains(&Move::Discard(k(3, 6))));
        assert!(!mv.contains(&Move::Close(k(3, 6))));
    }

    #[test]
    fn stock_recycles_and_ends_after_four_times() {
        let mut g = setup();
        g.stock.clear();
        g.hands[0] = vec![
            k(0, 1),
            k(0, 3),
            k(1, 5),
            k(2, 7),
            k(3, 9),
            k(1, 1),
            k(2, 2),
            k(3, 6),
        ];
        g.discard = vec![k(1, 2), k(2, 3), k(3, 4)];
        g.apply(&Move::Discard(k(0, 1)));
        assert_eq!(g.recycles, 1);
        assert_eq!(g.stock.len(), 3);
        assert_eq!(g.discard.len(), 1);
    }

    #[test]
    fn random_matches_finish_and_cards_conserved() {
        for seed in 1..=8u64 {
            let mut g = Chinchon::with_seed(seed);
            let mut rng = Rng::new(seed * 11);
            let mut n = 0;
            while g.outcome().is_none() {
                let all = g.stock.len() + g.discard.len() + g.hands[0].len() + g.hands[1].len();
                assert_eq!(all, 40);
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 3500, "too long");
            }
        }
    }

    #[test]
    fn evaluate_ignores_stock_card_and_opponent_hand() {
        let mut g = setup();
        g.hands[0] = vec![
            k(0, 4),
            k(0, 5),
            k(0, 6),
            k(0, 3),
            k(1, 3),
            k(2, 3),
            k(3, 5),
        ];
        g.hands[1] = vec![
            k(0, 1),
            k(1, 8),
            k(2, 9),
            k(3, 10),
            k(0, 2),
            k(1, 5),
            k(2, 6),
        ];
        g.phase = Phase::Draw;
        g.stock = vec![k(2, 7), k(1, 7)];
        let mut a = g.clone();
        a.apply(&Move::DrawStock);
        let mut b = g.clone();
        b.stock = vec![k(1, 7), k(2, 7)];
        b.apply(&Move::DrawStock);
        assert_eq!(a.evaluate(Player::One), b.evaluate(Player::One));
        let mut c = g.clone();
        c.hands[1] = vec![
            k(0, 10),
            k(1, 10),
            k(2, 10),
            k(3, 9),
            k(0, 9),
            k(1, 9),
            k(2, 8),
        ];
        assert_eq!(g.evaluate(Player::One), c.evaluate(Player::One));
    }

    #[test]
    fn ai_beats_random() {
        use polytabletop_core::GameObj;
        let mut wins = 0;
        let n = 8;
        for seed in 1..=n {
            let mut g: Box<dyn GameObj> = Box::new(Chinchon::with_seed(seed));
            let mut rng = Rng::new(seed + 7);
            let mut plies = 0;
            while g.outcome().is_none() && plies < 5000 {
                let mv = if g.to_move() == Player::One {
                    g.ai_move(rng.next_u64()).unwrap()
                } else {
                    let m = g.legal_moves();
                    m[rng.below(m.len())].clone()
                };
                g.play(&mv).unwrap();
                plies += 1;
            }
            if g.outcome() == Some(Outcome::Win(Player::One)) {
                wins += 1;
            }
        }
        assert!(wins * 10 >= n * 7, "AI won only {wins}/{n}");
    }
}

#[cfg(test)]
mod a1_blind {
    use super::*;
    use crate::ai_audit_a1::{audit, scramble_pool};

    #[test]
    fn ai_ignores_opponent_hand_and_stock_at_all_levels() {
        audit("chinchon", Chinchon::with_seed, 1..=4, 2, 70, |g, salt| {
            let foe = g.side.other().idx();
            if g.phase != Phase::RoundOver {
                scramble_pool(&mut g.hands[foe], &mut g.stock, salt);
            }
            g.rng = Rng::new(salt ^ 13);
        });
    }
}
