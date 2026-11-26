//! Escoba (Escoba de 15; Spania / Argentina / Uruguay), 2 jucători, baraja spaniolă de 40.
//!
//! * Valori la adunare: 1–7 ca atare, sota = 8, caballo = 9, rey = 10. Culori: `o`, `c`, `e`, `b`
//!   (vezi `baraja_es.rs`); cărțile se scriu `7o`, `10e`, `12b`.
//! * Se dau câte 3 cărți fiecăruia și se pun 4 pe masă; dacă cele 4 cărți dau exact 15,
//!   împărțitorul le ia ca **escoba** (1 punct; 30 = 2 puncte). Începe jucătorul care nu a
//!   împărțit. Când mâinile se golesc se dau încă 3 cărți fiecăruia (mutare forțată
//!   `imparte`) până se termină pachetul (6 împărțiri).
//! * La rândul tău joci o carte; dacă o submulțime a cărților de pe masă adunată cu
//!   valoarea ei dă **15**, o poți lua (nu ești obligat, ca în regula publică). Altfel (sau dacă
//!   preferi) cartea rămâne pe masă. Mutări: `7o` (lași), `7o:4c+4e` (iei 4c și 4e: 7+4+4 = 15).
//! * **Escoba**: golești masa (1 punct), dar nu la ultima carte jucată din mână în ultima
//!   împărțire. Ultimul care a luat primește cărțile rămase pe masă (fără escoba).
//! * Punctaj la sfârșitul mâinii: *cartas* (mai multe cărți, 1 punct; egalitate = nimic), *oros*
//!   (mai multe monede, 1; egalitate = nimic), *siete de velo* (7 de oros, 1), *setenta*
//!   (cea mai bună „primă”: suma celor mai bune cărți din fiecare culoare, 7=21, 6=18, 1=16,
//!   5=15, 4=14, 3=13, 2=12, figuri=10; culoare lipsă = 0; egalitate = nimic) și 1 punct pe
//!   escoba. Primul care ajunge la **21** la capăt de mână câștigă; dacă ambii ≥ 21, cel mai mare
//!   scor; la egalitate se mai joacă o mână.
//!
//! Informație ascunsă ca la `scopa.rs`: `render` arată doar mâna jucătorului vizualizat;
//! `evaluate` citește doar informația publică (masa, grămezile, escobele, scorul).

use crate::baraja_es::{deck, join, pos, sorted, suit, text};
use crate::cards::Viewer;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const TARGET: u8 = 21;
const SIETE_DE_VELO: u8 = 6; // 7 de oros

/// Valoarea la adunare, 1..=10.
pub fn value(c: u8) -> u8 {
    pos(c) + 1
}

fn prim_value(c: u8) -> u32 {
    match value(c) {
        7 => 21,
        6 => 18,
        1 => 16,
        5 => 15,
        4 => 14,
        3 => 13,
        2 => 12,
        _ => 10,
    }
}

/// Suma „setenta”: cea mai bună carte din fiecare culoare.
pub fn setenta(pile: &[u8]) -> u32 {
    (0..4u8)
        .map(|s| {
            pile.iter()
                .filter(|&&c| suit(c) == s)
                .map(|&c| prim_value(c))
                .max()
                .unwrap_or(0)
        })
        .sum()
}

#[derive(Clone, PartialEq, Debug)]
pub enum Move {
    Play(u8, Vec<u8>),
    Deal,
    Next,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Play,
    NeedDeal,
    RoundOver,
}

#[derive(Clone)]
pub struct Escoba {
    rng: Rng,
    viewer: Viewer,
    stock: Vec<u8>,
    hands: [Vec<u8>; 2],
    table: Vec<u8>,
    piles: [Vec<u8>; 2],
    escobas: [u8; 2],
    last_capture: Option<Player>,
    side: Player,
    dealer: Player,
    phase: Phase,
    score: [u8; 2],
    /// [cartas, oros, velo, setenta, escobas] pe jucător, ultima mână.
    last_round: Option<[[u8; 5]; 2]>,
}

impl Default for Escoba {
    fn default() -> Self {
        Self::new()
    }
}

impl Escoba {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Escoba {
            rng: Rng::new(seed),
            viewer: Viewer::Fixed(Player::One),
            stock: Vec::new(),
            hands: [Vec::new(), Vec::new()],
            table: Vec::new(),
            piles: [Vec::new(), Vec::new()],
            escobas: [0; 2],
            last_capture: None,
            side: Player::One,
            dealer: Player::Two,
            phase: Phase::Play,
            score: [0; 2],
            last_round: None,
        };
        g.new_round();
        g
    }

    fn new_round(&mut self) {
        let d = deck(&mut self.rng);
        self.hands = [d[0..3].to_vec(), d[3..6].to_vec()];
        self.table = d[6..10].to_vec();
        self.stock = d[10..].to_vec();
        self.piles = [Vec::new(), Vec::new()];
        self.escobas = [0; 2];
        self.last_capture = None;
        self.side = self.dealer.other();
        self.phase = Phase::Play;
        let sum: u32 = self.table.iter().map(|&c| value(c) as u32).sum();
        if sum == 15 || sum == 30 {
            let t = std::mem::take(&mut self.table);
            self.piles[self.dealer.idx()].extend(t);
            self.escobas[self.dealer.idx()] += (sum / 15) as u8;
            self.last_capture = Some(self.dealer);
        }
    }

    pub fn score(&self) -> (u8, u8) {
        (self.score[0], self.score[1])
    }

    pub fn table(&self) -> &[u8] {
        &self.table
    }

    fn match_over(&self) -> bool {
        (self.score[0] >= TARGET || self.score[1] >= TARGET) && self.score[0] != self.score[1]
    }

    /// Submulțimile mesei care, cu cartea `c`, dau 15.
    fn captures(&self, c: u8) -> Vec<Vec<u8>> {
        let need = 15 - value(c) as i32;
        let t = sorted(self.table.clone());
        let mut out = Vec::new();
        let mut cur = Vec::new();
        fn rec(t: &[u8], i: usize, left: i32, cur: &mut Vec<u8>, out: &mut Vec<Vec<u8>>) {
            if left == 0 && !cur.is_empty() {
                out.push(cur.clone());
                return;
            }
            for j in i..t.len() {
                let v = value(t[j]) as i32;
                if v <= left {
                    cur.push(t[j]);
                    rec(t, j + 1, left - v, cur, out);
                    cur.pop();
                }
            }
        }
        if need > 0 {
            rec(&t, 0, need, &mut cur, &mut out);
        }
        out
    }

    fn round_points(&self) -> [[u8; 5]; 2] {
        let mut r = [[0u8; 5]; 2];
        let cmp = |a: usize, b: usize| -> Option<usize> {
            match a.cmp(&b) {
                std::cmp::Ordering::Greater => Some(0),
                std::cmp::Ordering::Less => Some(1),
                std::cmp::Ordering::Equal => None,
            }
        };
        if let Some(w) = cmp(self.piles[0].len(), self.piles[1].len()) {
            r[w][0] = 1;
        }
        let oros = |q: usize| self.piles[q].iter().filter(|&&c| suit(c) == 0).count();
        if let Some(w) = cmp(oros(0), oros(1)) {
            r[w][1] = 1;
        }
        for (q, row) in r.iter_mut().enumerate() {
            if self.piles[q].contains(&SIETE_DE_VELO) {
                row[2] = 1;
            }
        }
        if let Some(w) = cmp(
            setenta(&self.piles[0]) as usize,
            setenta(&self.piles[1]) as usize,
        ) {
            r[w][3] = 1;
        }
        r[0][4] = self.escobas[0];
        r[1][4] = self.escobas[1];
        r
    }

    fn finish_round(&mut self) {
        if let Some(p) = self.last_capture {
            let rest = std::mem::take(&mut self.table);
            self.piles[p.idx()].extend(rest);
        }
        let r = self.round_points();
        for (sc, row) in self.score.iter_mut().zip(&r) {
            *sc += row.iter().sum::<u8>();
        }
        self.last_round = Some(r);
        self.phase = Phase::RoundOver;
    }
}

impl Game for Escoba {
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
        if self.match_over() {
            return Vec::new();
        }
        match self.phase {
            Phase::NeedDeal => vec![Move::Deal],
            Phase::RoundOver => vec![Move::Next],
            Phase::Play => {
                let mut v = Vec::new();
                for &c in &self.hands[self.side.idx()] {
                    v.push(Move::Play(c, Vec::new()));
                    for cap in self.captures(c) {
                        v.push(Move::Play(c, cap));
                    }
                }
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Next => {
                self.dealer = self.dealer.other();
                self.new_round();
            }
            Move::Deal => {
                for p in 0..2 {
                    for _ in 0..3 {
                        if let Some(c) = self.stock.pop() {
                            self.hands[p].push(c);
                        }
                    }
                }
                self.phase = Phase::Play;
            }
            Move::Play(c, take) => {
                let me = self.side;
                let h = &mut self.hands[me.idx()];
                let pos_in = h.iter().position(|x| x == c).expect("card in hand");
                h.remove(pos_in);
                if take.is_empty() {
                    self.table.push(*c);
                } else {
                    self.table.retain(|t| !take.contains(t));
                    self.piles[me.idx()].push(*c);
                    self.piles[me.idx()].extend(take.iter().copied());
                    self.last_capture = Some(me);
                }
                let last_play =
                    self.stock.is_empty() && self.hands[0].is_empty() && self.hands[1].is_empty();
                if !take.is_empty() && self.table.is_empty() && !last_play {
                    self.escobas[me.idx()] += 1;
                }
                self.side = me.other();
                if self.hands[0].is_empty() && self.hands[1].is_empty() {
                    if last_play {
                        self.finish_round();
                    } else {
                        self.phase = Phase::NeedDeal;
                        self.side = self.dealer.other();
                    }
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.match_over() {
            Some(Outcome::Win(if self.score[0] > self.score[1] {
                Player::One
            } else {
                Player::Two
            }))
        } else {
            None
        }
    }

    /// Citește doar informația publică (masa, grămezile, escobele, scorul).
    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let o = 1 - me;
        let mut s = (self.score[me] as i32 - self.score[o] as i32) * 1000;
        if self.phase == Phase::RoundOver {
            return s;
        }
        s += (self.escobas[me] as i32 - self.escobas[o] as i32) * 100;
        let cnt = |q: usize| self.piles[q].len() as i32;
        let oros = |q: usize| self.piles[q].iter().filter(|&&c| suit(c) == 0).count() as i32;
        s += (cnt(me) - cnt(o)) * 8;
        s += (oros(me) - oros(o)) * 12;
        let velo = |q: usize| i32::from(self.piles[q].contains(&SIETE_DE_VELO));
        s += (velo(me) - velo(o)) * 70;
        s += (setenta(&self.piles[me]) as i32 - setenta(&self.piles[o]) as i32) * 2;
        if self.phase == Phase::Play && self.side != p {
            // cărțile rămase pe masă pot fi luate de adversar
            let sum: i32 = self.table.iter().map(|&c| value(c) as i32).sum();
            if !self.table.is_empty() && (5..=14).contains(&sum) {
                s -= 45; // adversarul poate face escoba cu o singură carte
            }
            if self.table.contains(&SIETE_DE_VELO) {
                s -= 60;
            }
            for &c in &self.table {
                s -= 5 + if suit(c) == 0 { 8 } else { 0 } + if value(c) == 7 { 6 } else { 0 };
            }
        }
        s
    }

    fn render(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            "Escoba — până la {TARGET}: J1 {} – J2 {}\n",
            self.score[0], self.score[1]
        ));
        s.push_str(&format!("Masa: {}\n", join(&sorted(self.table.clone()))));
        let v = self.viewer.who(self.side);
        s.push_str(&format!(
            "Pachet: {} cărți. J1 a luat {} cărți ({} escobe), J2 a luat {} cărți ({} escobe); J{} (ascuns) ține {} cărți.\n",
            self.stock.len(),
            self.piles[0].len(),
            self.escobas[0],
            self.piles[1].len(),
            self.escobas[1],
            v.other().idx() + 1,
            self.hands[v.other().idx()].len()
        ));
        s.push_str(&format!(
            "Mâna ta (Jucător {}): {}\n",
            v.idx() + 1,
            join(&sorted(self.hands[v.idx()].clone()))
        ));
        if let Some(r) = &self.last_round {
            if self.phase == Phase::RoundOver {
                for (i, x) in r.iter().enumerate() {
                    s.push_str(&format!(
                        "Mâna: J{} — cartas {}, oros {}, velo {}, setenta {}, escobe {}\n",
                        i + 1,
                        x[0],
                        x[1],
                        x[2],
                        x[3],
                        x[4]
                    ));
                }
            }
        }
        match (self.outcome(), self.phase) {
            (Some(Outcome::Win(p)), _) => s.push_str(&format!(
                "Meci încheiat: Jucător {} câștigă.\n",
                p.idx() + 1
            )),
            (_, Phase::RoundOver) => s.push_str("Mâna s-a încheiat. Scrie «urmatoarea».\n"),
            (_, Phase::NeedDeal) => s.push_str("Mâinile s-au golit: scrie «imparte».\n"),
            _ => s.push_str(&format!("La mutare: Jucător {}\n", self.side.idx() + 1)),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Deal => "imparte".into(),
            Move::Next => "urmatoarea".into(),
            Move::Play(c, take) if take.is_empty() => text(*c),
            Move::Play(c, take) => format!(
                "{}:{}",
                text(*c),
                take.iter().map(|&t| text(t)).collect::<Vec<_>>().join("+")
            ),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // o: 0..9, c: 10..19, e: 20..29, b: 30..39; pos = valoare - 1
    fn o(v: u8) -> u8 {
        v - 1
    }
    fn cc(v: u8) -> u8 {
        10 + v - 1
    }
    fn ee(v: u8) -> u8 {
        20 + v - 1
    }
    fn bb(v: u8) -> u8 {
        30 + v - 1
    }

    fn base() -> Escoba {
        let mut g = Escoba::with_seed(1);
        g.stock.clear();
        g.hands = [Vec::new(), Vec::new()];
        g.table.clear();
        g.piles = [Vec::new(), Vec::new()];
        g.escobas = [0; 2];
        g.last_capture = None;
        g.side = Player::One;
        g.phase = Phase::Play;
        g
    }

    #[test]
    fn values_and_labels() {
        assert_eq!(value(o(8)), 8); // sota = 8
        assert_eq!(text(o(8)), "10o");
        assert_eq!(text(bb(10)), "12b");
        assert_eq!(value(bb(10)), 10);
    }

    #[test]
    fn captures_sum_to_fifteen_and_drop_is_always_allowed() {
        let mut g = base();
        g.table = vec![cc(4), ee(4), bb(3), o(10)];
        g.hands[0] = vec![ee(7)];
        g.hands[1] = vec![o(1)];
        let mv = g.legal_moves();
        // 7 + 4 + 4 = 15 ; 7 + ... 8 = 4+4 ; 3+? (3+5 no) ; 10 -> 7+10 =17 no
        assert!(mv.contains(&Move::Play(ee(7), vec![cc(4), ee(4)])));
        assert!(mv.contains(&Move::Play(ee(7), vec![])));
        assert_eq!(mv.len(), 2);
    }

    #[test]
    fn escoba_scores_but_not_on_the_last_play() {
        let mut g = base();
        g.stock = vec![o(2)];
        g.table = vec![cc(8)];
        g.hands[0] = vec![ee(7), ee(1)];
        g.hands[1] = vec![o(1)];
        g.apply(&Move::Play(ee(7), vec![cc(8)]));
        assert_eq!(g.escobas[0], 1);
        let mut h = base();
        h.table = vec![cc(8)];
        h.hands[0] = vec![ee(7)];
        h.apply(&Move::Play(ee(7), vec![cc(8)]));
        assert_eq!(h.escobas[0], 0);
        assert_eq!(h.phase, Phase::RoundOver);
    }

    #[test]
    fn last_capturer_gets_the_table_without_escoba() {
        let mut g = base();
        g.table = vec![cc(8), o(10)];
        g.hands[0] = vec![ee(7)];
        g.hands[1] = vec![bb(1)];
        g.apply(&Move::Play(ee(7), vec![cc(8)]));
        g.apply(&Move::Play(bb(1), vec![]));
        assert_eq!(g.phase, Phase::RoundOver);
        assert!(g.table.is_empty());
        assert_eq!(g.piles[0].len(), 2 + 2);
        assert_eq!(g.escobas[0], 0);
    }

    #[test]
    fn deal_is_a_forced_move_and_initial_fifteen_is_an_escoba_for_the_dealer() {
        let mut g = base();
        g.stock = vec![o(1), o(2), o(3), o(4), o(5), o(6)];
        g.table = vec![cc(9)];
        g.hands[0] = vec![ee(5)];
        g.hands[1] = vec![bb(1)];
        g.apply(&Move::Play(ee(5), vec![]));
        g.apply(&Move::Play(bb(1), vec![]));
        assert_eq!(g.phase, Phase::NeedDeal);
        assert_eq!(g.legal_moves(), vec![Move::Deal]);
        g.apply(&Move::Deal);
        assert_eq!(g.hands[0].len(), 3);
        // mână cu 15 pe masă: caută o semință unde împărțitorul ia masa
        let mut found = false;
        for seed in 1..5000u64 {
            let h = Escoba::with_seed(seed);
            if h.table.is_empty() {
                assert!(matches!(h.escobas[h.dealer.idx()], 1 | 2));
                assert_eq!(h.piles[h.dealer.idx()].len(), 4);
                found = true;
                break;
            }
        }
        assert!(found);
    }

    #[test]
    fn scoring_ties_give_nothing_and_setenta_counts_best_card_of_each_suit() {
        assert_eq!(setenta(&[o(7), cc(7), ee(7), bb(7)]), 84);
        assert_eq!(setenta(&[o(7), o(6)]), 21);
        let mut g = base();
        g.piles[0] = vec![o(7), o(1), o(2), o(3), o(4), cc(7), ee(7), bb(7)];
        g.piles[1] = vec![cc(1), cc(2)];
        g.escobas = [1, 0];
        let r = g.round_points();
        assert_eq!(r[0], [1, 1, 1, 1, 1]);
        assert_eq!(r[1], [0, 0, 0, 0, 0]);
        g.piles[1] = (0..8).map(|i| bb(1 + i % 10)).collect();
        let r2 = g.round_points();
        assert_eq!(r2[0][0] + r2[1][0], 0); // 8–8: nimic la cartas
    }

    #[test]
    fn cards_are_conserved_and_matches_finish() {
        for seed in 1..=12u64 {
            let mut g = Escoba::with_seed(seed);
            let mut rng = Rng::new(seed ^ 0x55);
            let mut n = 0;
            while g.outcome().is_none() {
                let all = g.stock.len()
                    + g.table.len()
                    + g.hands[0].len()
                    + g.hands[1].len()
                    + g.piles[0].len()
                    + g.piles[1].len();
                assert_eq!(all, 40);
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 20000);
            }
        }
    }

    #[test]
    fn render_hides_the_opponent_and_evaluate_ignores_hidden_cards() {
        let g = Escoba::with_seed(5);
        let r = Game::render(&g);
        assert!(r.contains("J2 (ascuns) ține 3 cărți") || r.contains("ascuns) ține 3"));
        let mut h = g.clone();
        let (a, b) = (h.hands[1][0], h.stock[3]);
        h.hands[1][0] = b;
        h.stock[3] = a;
        for p in [Player::One, Player::Two] {
            assert_eq!(g.evaluate(p), h.evaluate(p));
        }
    }

    #[test]
    fn ai_beats_a_random_mover_mostly() {
        use polytabletop_core::GameObj;
        let mut wins = 0;
        let n = 12;
        for seed in 1..=n {
            let mut g: Box<dyn GameObj> = Box::new(Escoba::with_seed(seed));
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
        assert!(wins * 10 >= n * 8, "AI won only {wins}/{n}");
    }
}
