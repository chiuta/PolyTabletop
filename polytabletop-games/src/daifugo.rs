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
//! Daifugo / Daihinmin (Japonia) — joc de eliminare a cărților, 2 jucători, 54 de cărți.
//!
//! Reguli folosite (varianta de bază, cu cele mai răspândite reguli locale):
//! * 54 de cărți (cu 2 jokeri), 27 fiecăruia — se împart toate, deci mâna adversarului se poate
//!   deduce din ce vezi. Ordinea: 3 < 4 < … < 10 < J < Q < K < A < 2; jokerul singur e cel mai
//!   mare. Culoarea nu contează.
//! * Cine conduce joacă un grup de cărți de același rang (1–4); celălalt trebuie să dea același
//!   număr de cărți de rang mai mare, ori `pas`. După un `pas` câmpul se curăță și conduce
//!   cel care a jucat ultimul. Cine rămâne primul fără cărți câștigă runda (Daifugō).
//! * Jokerul poate fi folosit singur (mutare `X`, bate orice singleton) sau ca „wild” într-un
//!   grup cu cel puțin o carte reală (`5X` = pereche de 5, `55X` = trei de 5).
//! * Revoluție (kakumei): un grup de 4 cărți (cu joker sau nu) inversează ordinea rangurilor
//!   (2 devine cel mai mic, 3 cel mai mare; jokerul rămâne cel mai mare) până la o nouă
//!   revoluție sau sfârșitul rundei. 8-stop (yagiri): orice grup care conține un 8 curăță câmpul
//!   și același jucător conduce din nou.
//! * Meci de 5 runde. De la runda a doua: Daihinmin (perdantul) dă câștigătorului cele mai
//!   puternice 2 cărți ale lui (la revoluție tot ordinea normală de dinaintea rundei), iar
//!   Daifugō alege 2 cărți oarecare de dat înapoi (`da 4 5` → rangurile date) și conduce.
//!   Câștigă cine are mai multe runde.
//! * Omise: scara (kaidan), shibari (blocarea pe culoare), 11-back, 3 de pică peste joker,
//!   interdicția de a termina pe 8/joker, ranguri intermediare (Fugō/Hinmin), 4–5 jucători.
//! * Mutări: `3`, `77`, `JJJ`, `55X`, `X`, `pas`, `da 4 5`.

use crate::cards::Viewer;
use crate::p2_util::{noise, sig_of, Mc};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const ROUNDS: u8 = 5;
pub const CHARS: [char; 13] = [
    '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A', '2',
];
const JOKER: usize = 13;
const EIGHT: usize = 5;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Play(u8, u8, u8),
    Pass,
    Give(u8, u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Play,
    Give,
    Over,
}

type Hand = [u8; 14];

#[derive(Clone)]
pub struct Daifugo {
    hands: [Hand; 2],
    /// (rang, număr de cărți) al câmpului curent.
    field: Option<(u8, u8)>,
    holder: Player,
    revolution: bool,
    side: Player,
    round: u8,
    wins: [u8; 2],
    phase: Phase,
    last: String,
    rng: Rng,
    viewer: Viewer,
}

impl Default for Daifugo {
    fn default() -> Self {
        Self::new()
    }
}

fn total(h: &Hand) -> u32 {
    h.iter().map(|&x| x as u32).sum()
}

impl Daifugo {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Daifugo {
            hands: [[0; 14]; 2],
            field: None,
            holder: Player::One,
            revolution: false,
            side: Player::One,
            round: 1,
            wins: [0; 2],
            phase: Phase::Play,
            last: String::new(),
            rng: Rng::new(seed),
            viewer: Viewer::Fixed(Player::One),
        };
        g.deal();
        g
    }

    fn deal(&mut self) {
        let mut deck: Vec<u8> = Vec::new();
        for r in 0..13u8 {
            for _ in 0..4 {
                deck.push(r);
            }
        }
        deck.push(13);
        deck.push(13);
        crate::p2_util::shuffle_vec(&mut deck, &mut self.rng);
        self.hands = [[0; 14]; 2];
        for (i, c) in deck.into_iter().enumerate() {
            self.hands[i % 2][c as usize] += 1;
        }
        self.field = None;
        self.revolution = false;
    }

    pub fn hand_for_test(&mut self, p: usize, h: Hand) {
        self.hands[p] = h;
    }
    pub fn counts(&self) -> (u32, u32) {
        (total(&self.hands[0]), total(&self.hands[1]))
    }

    /// Puterea unui rang (mai mare = mai puternic), ținând cont de revoluție.
    fn power(&self, r: u8) -> i32 {
        if r as usize == JOKER {
            100
        } else if self.revolution {
            12 - r as i32
        } else {
            r as i32
        }
    }

    fn plays(&self, p: usize) -> Vec<(u8, u8, u8)> {
        let h = &self.hands[p];
        let mut v = Vec::new();
        for r in 0..13u8 {
            let have = h[r as usize];
            for real in 1..=have.min(4) {
                for jok in 0..=h[JOKER].min(2) {
                    let n = real + jok;
                    if n > 4 {
                        continue;
                    }
                    if let Some((fr, fn_)) = self.field {
                        if n != fn_ || self.power(r) <= self.power(fr) {
                            continue;
                        }
                    }
                    v.push((r, real, jok));
                }
            }
        }
        if h[JOKER] >= 1 {
            let ok = match self.field {
                None => true,
                Some((fr, fn_)) => fn_ == 1 && fr as usize != JOKER,
            };
            if ok {
                v.push((13, 0, 1));
            }
        }
        v
    }

    fn start_next_round(&mut self, winner: Player) {
        self.round += 1;
        self.deal();
        let loser = winner.other();
        // tribut: cele mai puternice 2 cărți ale perdantului (ordine normală)
        for _ in 0..2 {
            let mut best = None;
            for r in (0..=13usize).rev() {
                if self.hands[loser.idx()][r] > 0 {
                    best = Some(r);
                    break;
                }
            }
            if let Some(r) = best {
                self.hands[loser.idx()][r] -= 1;
                self.hands[winner.idx()][r] += 1;
            }
        }
        self.phase = Phase::Give;
        self.side = winner;
        self.holder = winner;
    }

    fn plays_left(h: &Hand) -> i32 {
        h.iter().filter(|&&x| x > 0).count() as i32
    }
}

impl Mc for Daifugo {
    fn determinize(&self, _viewer: Player, _rng: &mut Rng) -> Self {
        // mâna adversarului = complementul (54 − mâna mea − cărțile jucate): se deduce exact.
        self.clone()
    }
    fn epoch(&self) -> u32 {
        self.round as u32
    }
    fn margin(&self, p: Player) -> i32 {
        10 * (self.wins[p.idx()] as i32 - self.wins[p.other().idx()] as i32)
    }
}

impl Game for Daifugo {
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
            Phase::Give => {
                let h = &self.hands[self.side.idx()];
                let mut v = Vec::new();
                for a in 0..14u8 {
                    for b in a..14u8 {
                        let need = if a == b { 2 } else { 1 };
                        if h[a as usize] >= need && h[b as usize] >= 1 {
                            v.push(Move::Give(a, b));
                        }
                    }
                }
                v
            }
            Phase::Play => {
                let mut v: Vec<Move> = self
                    .plays(self.side.idx())
                    .into_iter()
                    .map(|(r, a, b)| Move::Play(r, a, b))
                    .collect();
                if self.field.is_some() {
                    v.push(Move::Pass);
                }
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        let me = self.side;
        match *m {
            Move::Give(a, b) => {
                let (i, o) = (me.idx(), me.other().idx());
                for r in [a, b] {
                    self.hands[i][r as usize] -= 1;
                    self.hands[o][r as usize] += 1;
                }
                self.phase = Phase::Play;
                self.last = format!("J{} dă înapoi 2 cărți", me.idx() + 1);
            }
            Move::Pass => {
                self.field = None;
                self.side = self.holder;
                self.last = format!("J{} dă pas", me.idx() + 1);
            }
            Move::Play(r, real, jok) => {
                let i = me.idx();
                if r as usize == JOKER {
                    self.hands[i][JOKER] -= 1;
                    self.field = Some((13, 1));
                } else {
                    self.hands[i][r as usize] -= real;
                    self.hands[i][JOKER] -= jok;
                    self.field = Some((r, real + jok));
                }
                self.holder = me;
                let n = real + jok;
                let mut note = String::new();
                if r as usize != JOKER && n == 4 {
                    self.revolution = !self.revolution;
                    note.push_str(" — revoluție!");
                }
                self.last = format!("J{} joacă {}{}", i + 1, play_text(r, real, jok), note);
                if total(&self.hands[i]) == 0 {
                    self.wins[i] += 1;
                    if self.round >= ROUNDS {
                        self.phase = Phase::Over;
                    } else {
                        self.start_next_round(me);
                    }
                    return;
                }
                if r as usize == EIGHT {
                    self.field = None;
                    self.last.push_str(" — opriți (8)");
                    // aceeași persoană conduce
                    return;
                }
                self.side = me.other();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        Some(match self.wins[0].cmp(&self.wins[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let (me, op) = (p.idx(), p.other().idx());
        let mut s = 1000 * (self.wins[me] as i32 - self.wins[op] as i32);
        if self.phase == Phase::Over {
            return s;
        }
        let ctrl = |h: &Hand| {
            let mut c = h[JOKER] as i32 * 3;
            for r in 0..13u8 {
                if h[r as usize] > 0 {
                    // carte de control = cea mai puternică din ordinea curentă
                    if self.power(r) >= 11 {
                        c += 1;
                    }
                }
            }
            c
        };
        s += 60 * (Self::plays_left(&self.hands[op]) - Self::plays_left(&self.hands[me]));
        s += 2 * (total(&self.hands[op]) as i32 - total(&self.hands[me]) as i32);
        s += 15 * (ctrl(&self.hands[me]) - ctrl(&self.hands[op]));
        if self.field.is_none() && self.side == p {
            s += 20;
        }
        let sig = sig_of(&[
            sig_hand(&self.hands[me]),
            self.field
                .map(|f| f.0 as u64 * 8 + f.1 as u64)
                .unwrap_or(99),
            self.wins[0] as u64,
        ]);
        s += noise(sig, 60);
        s + crate::p2_util::mc_value(self, p, crate::p2_util::mc_samples(), sig)
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Daifugo — runda {}/{} | victorii: J1 {}, J2 {}{}\n",
            self.round.min(ROUNDS),
            ROUNDS,
            self.wins[0],
            self.wins[1],
            if self.revolution { " | REVOLUȚIE" } else { "" }
        );
        if !self.last.is_empty() {
            s.push_str(&format!("({})\n", self.last));
        }
        if self.phase == Phase::Over {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        match self.field {
            Some((r, n)) => s.push_str(&format!(
                "Câmp: {} cărți de rang {}\n",
                n,
                if r as usize == JOKER {
                    'X'
                } else {
                    CHARS[r as usize]
                }
            )),
            None => s.push_str("Câmp: gol\n"),
        }
        let v = self.viewer.who(self.side);
        s.push_str(&format!(
            "Mâna ta (J{}): {}\n",
            v.idx() + 1,
            hand_text(&self.hands[v.idx()])
        ));
        s.push_str(&format!(
            "Jucătorul {}: {} cărți\n",
            v.other().idx() + 1,
            total(&self.hands[v.other().idx()])
        ));
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Pass => "pas".into(),
            Move::Play(r, a, b) => play_text(*r, *a, *b),
            Move::Give(a, b) => format!("da {} {}", rank_char(*a), rank_char(*b)),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

fn rank_char(r: u8) -> char {
    if r as usize == JOKER {
        'X'
    } else {
        CHARS[r as usize]
    }
}

fn play_text(r: u8, real: u8, jok: u8) -> String {
    let mut s = String::new();
    for _ in 0..real {
        s.push(rank_char(r));
    }
    for _ in 0..jok {
        s.push('X');
    }
    s
}

fn sig_hand(h: &Hand) -> u64 {
    let mut x = 0u64;
    for &c in h {
        x = x * 7 + c as u64;
    }
    x
}

/// Mâna proprie (cu litere), de la cele mai slabe la cele mai puternice.
pub fn hand_text(h: &Hand) -> String {
    let mut v = Vec::new();
    for r in 0..14usize {
        for _ in 0..h[r] {
            v.push(rank_char(r as u8).to_string());
        }
    }
    v.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn h(cards: &[(usize, u8)]) -> Hand {
        let mut x = [0u8; 14];
        for &(r, n) in cards {
            x[r] = n;
        }
        x
    }

    fn fresh() -> Daifugo {
        let mut g = Daifugo::with_seed(1);
        g.hands = [[0; 14]; 2];
        g
    }

    #[test]
    fn deal_is_whole_deck() {
        let g = Daifugo::with_seed(5);
        assert_eq!(g.counts(), (27, 27));
        for r in 0..13 {
            assert_eq!(g.hands[0][r] + g.hands[1][r], 4);
        }
        assert_eq!(g.hands[0][13] + g.hands[1][13], 2);
    }

    #[test]
    fn must_match_count_and_beat_rank() {
        let mut g = fresh();
        g.hands[0] = h(&[(3, 2), (4, 1), (8, 3)]);
        g.hands[1] = h(&[(5, 2), (6, 2)]);
        g.apply(&Move::Play(3, 2, 0));
        let mv = g.legal_moves();
        // J2 are 55 (rang 5) și 66: doar perechi mai mari decât 3; (nu există) → 55 nu bate 6? 5 > 3 da
        assert!(mv.contains(&Move::Play(5, 2, 0)));
        assert!(mv.contains(&Move::Play(6, 2, 0)));
        assert!(!mv.contains(&Move::Play(5, 1, 0)));
        assert!(mv.contains(&Move::Pass));
    }

    #[test]
    fn pass_clears_and_holder_leads() {
        let mut g = fresh();
        g.hands[0] = h(&[(3, 2), (4, 1)]);
        g.hands[1] = h(&[(5, 1)]);
        g.hands[1][12] = 1;
        g.apply(&Move::Play(4, 1, 0));
        g.apply(&Move::Pass);
        assert_eq!(g.to_move(), Player::One);
        assert!(g.field.is_none());
        assert!(!g.legal_moves().contains(&Move::Pass));
    }

    #[test]
    fn eight_stop_keeps_lead() {
        let mut g = fresh();
        g.hands[0] = h(&[(EIGHT, 1), (3, 1)]);
        g.hands[1] = h(&[(9, 1)]);
        g.apply(&Move::Play(EIGHT as u8, 1, 0));
        assert_eq!(g.to_move(), Player::One);
        assert!(g.field.is_none());
    }

    #[test]
    fn revolution_inverts_and_joker_wild() {
        let mut g = fresh();
        g.hands[0] = h(&[(4, 4), (3, 1)]);
        g.hands[1] = h(&[(12, 1), (0, 1), (5, 1)]);
        g.apply(&Move::Play(4, 4, 0));
        assert!(g.revolution);
        // 7777 apoi... după revoluție puterea 3 > 2
        g.field = Some((12, 1));
        g.side = Player::Two;
        let mv = g.legal_moves();
        // 3 (rang 0) bate un 2 în revoluție
        assert!(mv.contains(&Move::Play(0, 1, 0)));
        assert!(!mv.contains(&Move::Play(12, 1, 0)));
        // joker wild
        let mut g = fresh();
        g.hands[0] = h(&[(6, 2)]);
        g.hands[0][13] = 1;
        g.hands[1] = h(&[(2, 1)]);
        let mv = g.legal_moves();
        assert!(mv.contains(&Move::Play(6, 2, 1)));
        assert!(mv.contains(&Move::Play(6, 1, 1)));
        assert!(mv.contains(&Move::Play(13, 0, 1)));
        let t: Vec<String> = mv.iter().map(|m| g.format_move(m)).collect();
        assert!(t.contains(&"99X".to_string()) && t.contains(&"X".to_string()));
    }

    #[test]
    fn lone_joker_beats_any_single_only() {
        let mut g = fresh();
        g.hands[0] = h(&[(12, 1), (0, 1)]);
        g.hands[1] = h(&[(1, 2)]);
        g.hands[1][13] = 1;
        g.apply(&Move::Play(12, 1, 0));
        let mv = g.legal_moves();
        assert!(mv.contains(&Move::Play(13, 0, 1)));
        assert!(!mv.contains(&Move::Play(1, 1, 0)));
    }

    #[test]
    fn round_end_tribute_and_give_back() {
        let mut g = fresh();
        g.hands[0] = h(&[(4, 1)]);
        g.hands[1] = h(&[(9, 1)]);
        g.apply(&Move::Play(4, 1, 0));
        assert_eq!(g.wins, [1, 0]);
        assert_eq!(g.round, 2);
        assert_eq!(g.phase, Phase::Give);
        assert_eq!(g.to_move(), Player::One);
        // 27 + 2 = 29 cărți după tribut, 25 la perdant
        assert_eq!(g.counts(), (29, 25));
        let mv = g.legal_moves();
        assert!(mv.iter().all(|m| matches!(m, Move::Give(..))));
        g.apply(&mv[0]);
        assert_eq!(g.counts(), (27, 27));
        assert_eq!(g.phase, Phase::Play);
    }

    #[test]
    fn hidden_hand_not_leaked() {
        let g = Daifugo::with_seed(3);
        let r = Game::render(&g);
        assert!(r.contains("Mâna ta (J1)"));
        assert!(!r.contains("Mâna ta (J2)"));
    }

    #[test]
    fn ai_finishes_match() {
        for seed in 1..3u64 {
            let mut g = Daifugo::with_seed(seed);
            let mut n = 0;
            while g.outcome().is_none() && n < 3000 {
                let m = polytabletop_core::ai::best_move(&g, n).unwrap();
                g.apply(&m);
                n += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed}");
        }
    }
}
