//! Trix (Orientul Mijlociu), variantă pentru 2 jucători, din regulile publice.
//!
//! * Pachet de 32 de cărți (7–A), câte 16 fiecăruia (toate cărțile sunt în joc). Se joacă 10 mâini:
//!   primele 5 sunt „împărăția” lui J1, următoarele 5 ale lui J2. În fiecare mână proprietarul alege
//!   un contract pe care nu l-a mai folosit, apoi proprietarul începe.
//! * Contractele cu levate (se răspunde la culoare, fără atu, cea mai mare carte din culoarea cerută
//!   ia): **king** (Popă de cupă: −75), **queens** (−25 pentru fiecare damă), **diamonds** (−10 pentru
//!   fiecare caro), **slaps** (−15 pentru fiecare levată). Mâna se încheie când au căzut toate cărțile
//!   penalizate.
//! * **Trex** (aranjament): se începe cu un Valet (J) din orice culoare; apoi se pun cărți vecine, în jos
//!   (10, 9, 8, 7) sau în sus (Q, K, A). Cine nu poate pune spune `pas`. Primul rămas fără cărți ia +200.
//! * Câștigă cine are scorul total mai mare după 10 mâini. Dublările (kawa) nu sunt incluse.
//!
//! Mutări: `imparte`, `king`/`queens`/`diamonds`/`slaps`/`trex`, `Jh` (carte), `pas` (doar la Trex).
//! Informația ascunsă: AI-ul citește doar propria mână.

use crate::cards::Viewer;
use crate::cards::{full_deck, join_cards, shuffle, Card, SUIT_SYMBOLS};
use crate::cardsb_util::{has_suit, sort_hand, trick_winner};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Contract {
    King,
    Queens,
    Diamonds,
    Slaps,
    Trex,
}

const CONTRACTS: [Contract; 5] = [
    Contract::King,
    Contract::Queens,
    Contract::Diamonds,
    Contract::Slaps,
    Contract::Trex,
];

fn cname(c: Contract) -> &'static str {
    match c {
        Contract::King => "king",
        Contract::Queens => "queens",
        Contract::Diamonds => "diamonds",
        Contract::Slaps => "slaps",
        Contract::Trex => "trex",
    }
}

fn cidx(c: Contract) -> usize {
    CONTRACTS.iter().position(|&x| x == c).unwrap()
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Deal,
    Choose,
    Play,
    Over,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Deal,
    Choose(Contract),
    Play(Card),
    Pass,
}

#[derive(Clone)]
pub struct Trix {
    rng: Rng,
    viewer: Viewer,
    hands: [Vec<Card>; 2],
    deal_no: u8,
    used: [[bool; 5]; 2],
    contract: Option<Contract>,
    phase: Phase,
    side: Player,
    trick: Vec<(Player, Card)>,
    /// penalizări în mâna curentă
    pen: [i32; 2],
    scores: [i32; 2],
    /// Trex: (rang minim, rang maxim) pe culoare, 0 = culoare nepornită
    layout: [(u8, u8); 4],
    queens_fallen: u8,
    diamonds_fallen: u8,
    king_fallen: bool,
    tricks_played: u8,
    log: String,
}

impl Default for Trix {
    fn default() -> Self {
        Self::new()
    }
}

fn card_penalty(c: Card, k: Contract) -> i32 {
    match k {
        Contract::King => {
            if c.rank == 13 && c.suit == 0 {
                75
            } else {
                0
            }
        }
        Contract::Queens => {
            if c.rank == 12 {
                25
            } else {
                0
            }
        }
        Contract::Diamonds if c.suit == 1 => 10,
        _ => 0,
    }
}

impl Trix {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Trix {
            rng: Rng::new(seed),
            viewer: Viewer::Fixed(Player::One),
            hands: [Vec::new(), Vec::new()],
            deal_no: 0,
            used: [[false; 5]; 2],
            contract: None,
            phase: Phase::Deal,
            side: Player::One,
            trick: Vec::new(),
            pen: [0, 0],
            scores: [0, 0],
            layout: [(0, 0); 4],
            queens_fallen: 0,
            diamonds_fallen: 0,
            king_fallen: false,
            tricks_played: 0,
            log: String::new(),
        };
        g.deal();
        g
    }

    pub fn score(&self) -> (i32, i32) {
        (self.scores[0], self.scores[1])
    }

    fn owner(&self) -> Player {
        if self.deal_no < 5 {
            Player::One
        } else {
            Player::Two
        }
    }

    fn deal(&mut self) {
        let mut d = full_deck(7);
        shuffle(&mut d, &mut self.rng);
        for p in 0..2 {
            self.hands[p] = d.split_off(d.len() - 16);
            sort_hand(&mut self.hands[p]);
        }
        self.contract = None;
        self.trick.clear();
        self.pen = [0, 0];
        self.layout = [(0, 0); 4];
        self.queens_fallen = 0;
        self.diamonds_fallen = 0;
        self.king_fallen = false;
        self.tricks_played = 0;
        self.phase = Phase::Choose;
        self.side = self.owner();
    }

    fn trex_playable(&self, c: Card) -> bool {
        let (lo, hi) = self.layout[c.suit as usize];
        if lo == 0 {
            c.rank == 11
        } else {
            (c.rank + 1 == lo && c.rank >= 7) || (c.rank == hi + 1 && c.rank <= 14)
        }
    }

    fn end_deal(&mut self) {
        self.scores[0] += self.pen[0];
        self.scores[1] += self.pen[1];
        self.deal_no += 1;
        if self.deal_no >= 10 {
            self.phase = Phase::Over;
        } else {
            self.phase = Phase::Deal;
            self.side = self.owner();
        }
    }

    fn deal_done(&self, k: Contract) -> bool {
        match k {
            Contract::King => self.king_fallen,
            Contract::Queens => self.queens_fallen == 4,
            Contract::Diamonds => self.diamonds_fallen == 8,
            Contract::Slaps => self.tricks_played == 16,
            Contract::Trex => false,
        }
    }
}

impl Game for Trix {
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
            Phase::Choose => {
                let o = self.owner().idx();
                CONTRACTS
                    .iter()
                    .filter(|&&c| !self.used[o][cidx(c)])
                    .map(|&c| Move::Choose(c))
                    .collect()
            }
            Phase::Play => {
                let h = &self.hands[self.side.idx()];
                if self.contract == Some(Contract::Trex) {
                    let v: Vec<Move> = h
                        .iter()
                        .filter(|&&c| self.trex_playable(c))
                        .map(|&c| Move::Play(c))
                        .collect();
                    if v.is_empty() {
                        return vec![Move::Pass];
                    }
                    return v;
                }
                let eff = |c: Card| c.suit;
                let must = self
                    .trick
                    .first()
                    .map(|&(_, c)| c.suit)
                    .filter(|&s| has_suit(h, s, &eff));
                h.iter()
                    .filter(|c| must.is_none_or(|s| c.suit == s))
                    .map(|&c| Move::Play(c))
                    .collect()
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        match *m {
            Move::Deal => self.deal(),
            Move::Choose(k) => {
                let o = self.owner();
                self.used[o.idx()][cidx(k)] = true;
                self.contract = Some(k);
                self.phase = Phase::Play;
                self.side = o;
            }
            Move::Pass => {
                self.side = self.side.other();
            }
            Move::Play(c) => {
                let p = self.side;
                let pos = self.hands[p.idx()].iter().position(|&x| x == c).unwrap();
                self.hands[p.idx()].remove(pos);
                let k = self.contract.unwrap();
                if k == Contract::Trex {
                    let l = &mut self.layout[c.suit as usize];
                    if l.0 == 0 {
                        *l = (c.rank, c.rank);
                    } else if c.rank < l.0 {
                        l.0 = c.rank;
                    } else {
                        l.1 = c.rank;
                    }
                    if self.hands[p.idx()].is_empty() {
                        self.pen[p.idx()] += 200;
                        self.log = format!("J{} a terminat primul: +200", p.idx() + 1);
                        self.end_deal();
                    } else {
                        self.side = p.other();
                    }
                    return;
                }
                self.trick.push((p, c));
                if self.trick.len() < 2 {
                    self.side = p.other();
                    return;
                }
                let cards: Vec<Card> = self.trick.iter().map(|&(_, c)| c).collect();
                let w = trick_winner(&cards, None, &|c| c.suit, &|c| c.rank as u32);
                let winner = self.trick[w].0;
                let mut pen: i32 = cards.iter().map(|&c| card_penalty(c, k)).sum();
                if k == Contract::Slaps {
                    pen = 15;
                }
                self.pen[winner.idx()] -= pen;
                for &c in &cards {
                    if c.rank == 12 {
                        self.queens_fallen += 1;
                    }
                    if c.suit == 1 {
                        self.diamonds_fallen += 1;
                    }
                    if c.rank == 13 && c.suit == 0 {
                        self.king_fallen = true;
                    }
                }
                self.tricks_played += 1;
                self.trick.clear();
                self.side = winner;
                if self.deal_done(k) || self.hands[0].is_empty() {
                    self.log = format!(
                        "Mână încheiată ({}): J1 {:+}, J2 {:+}",
                        cname(k),
                        self.pen[0],
                        self.pen[1]
                    );
                    self.end_deal();
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        Some(match self.scores[0].cmp(&self.scores[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let q = p.other();
        let mut s = 10 * (self.scores[p.idx()] - self.scores[q.idx()]);
        let mine = &self.hands[p.idx()];
        match self.contract {
            Some(Contract::Trex) if self.phase == Phase::Play => {
                s -= 12 * mine.len() as i32;
                s += 3 * mine.iter().filter(|&&c| self.trex_playable(c)).count() as i32;
                // valetul ține cărțile vecine „încuiate”: bine să le avem
                s += 10 * (self.pen[p.idx()] - self.pen[q.idx()]) / 10;
            }
            Some(k) if self.phase == Phase::Play => {
                s += 10 * (self.pen[p.idx()] - self.pen[q.idx()]);
                // cărți periculoase rămase în mână (cu atât mai rele cu cât sunt mai mari)
                for &c in mine {
                    let risk = match k {
                        Contract::King => {
                            if c.rank == 13 && c.suit == 0 {
                                60
                            } else if c.suit == 0 && c.rank > 13 {
                                10
                            } else {
                                0
                            }
                        }
                        Contract::Queens => {
                            if c.rank == 12 {
                                20
                            } else if c.rank > 12 {
                                6
                            } else {
                                0
                            }
                        }
                        Contract::Diamonds => {
                            if c.suit == 1 {
                                c.rank as i32 / 2
                            } else {
                                0
                            }
                        }
                        _ => c.rank as i32 / 3,
                    };
                    s -= risk;
                }
                if let Some(&(w, c)) = self.trick.first() {
                    // am condus (sau adversarul): cartea mare e riscantă
                    let r = 2 * c.rank as i32 + card_penalty(c, k);
                    s += if w == p { -r } else { r / 2 };
                }
            }
            _ => {}
        }
        s
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Trix — mâna {}/10 (împărăția lui J{}) | scor: J1 {}, J2 {}\n",
            (self.deal_no + 1).min(10),
            self.owner().idx() + 1,
            self.scores[0],
            self.scores[1]
        );
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        if self.phase == Phase::Over {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        if let Some(k) = self.contract {
            s.push_str(&format!(
                "Contract: {} | în mână: J1 {:+}, J2 {:+}\n",
                cname(k),
                self.pen[0],
                self.pen[1]
            ));
            if k == Contract::Trex {
                for (su, sym) in SUIT_SYMBOLS.iter().enumerate() {
                    let (lo, hi) = self.layout[su];
                    let row = if lo == 0 {
                        "—".to_string()
                    } else {
                        format!("{}…{}", lab(lo), lab(hi))
                    };
                    s.push_str(&format!("  {}: {}\n", sym, row));
                }
            }
        } else if self.phase == Phase::Choose {
            let o = self.owner().idx();
            let left: Vec<&str> = CONTRACTS
                .iter()
                .filter(|&&c| !self.used[o][cidx(c)])
                .map(|&c| cname(c))
                .collect();
            s.push_str(&format!(
                "J{} alege contractul: {}\n",
                o + 1,
                left.join(", ")
            ));
        }
        if !self.trick.is_empty() {
            let cs: Vec<Card> = self.trick.iter().map(|&(_, c)| c).collect();
            s.push_str(&format!("Pe masă: {}\n", join_cards(&cs)));
        }
        let v = self.viewer.who(self.side);
        let o = v.other();
        s.push_str(&format!(
            "Mâna ta (J{}): {}\n",
            v.idx() + 1,
            join_cards(&self.hands[v.idx()])
        ));
        s.push_str(&format!(
            "J{} are {} cărți\n",
            o.idx() + 1,
            self.hands[o.idx()].len()
        ));
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Deal => "imparte".into(),
            Move::Choose(c) => cname(*c).into(),
            Move::Play(c) => c.text(),
            Move::Pass => "pas".into(),
        }
    }

    /// Adâncime 0 (o singură mutare): `AlphaBeta(1)` ar fi extins la Hard/Expert răspunsul adversarului din
    /// mâna lui reală (și următoarea împărțire); evaluarea citește doar mâna proprie și datele publice.
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

fn lab(r: u8) -> &'static str {
    Card { rank: r, suit: 0 }.rank_label()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(rank: u8, suit: u8) -> Card {
        Card { rank, suit }
    }

    #[test]
    fn deal_splits_all_32() {
        let g = Trix::with_seed(1);
        let mut all = g.hands[0].clone();
        all.extend(&g.hands[1]);
        all.sort_by_key(|c| (c.suit, c.rank));
        all.dedup();
        assert_eq!(all.len(), 32);
    }

    #[test]
    fn trex_layout_rules() {
        let mut g = Trix::with_seed(1);
        g.contract = Some(Contract::Trex);
        g.phase = Phase::Play;
        g.hands = [
            vec![c(11, 0), c(10, 0), c(12, 1)],
            vec![c(9, 0), c(11, 1), c(14, 0)],
        ];
        g.side = Player::One;
        // doar valetul de pe masă începe: 10♥ nu e jucabil, Q♦ nu
        assert_eq!(g.legal_moves(), vec![Move::Play(c(11, 0))]);
        g.apply(&Move::Play(c(11, 0)));
        // J2: 9♥ nu (10♥ lipsește), J♦ da, A♥ nu
        assert_eq!(g.legal_moves(), vec![Move::Play(c(11, 1))]);
        g.apply(&Move::Play(c(11, 1)));
        // J1 are 10♥ (jos) și Q♦ (sus)
        let l = g.legal_moves();
        assert!(l.contains(&Move::Play(c(10, 0))) && l.contains(&Move::Play(c(12, 1))));
    }

    #[test]
    fn trex_pass_and_out() {
        let mut g = Trix::with_seed(1);
        g.contract = Some(Contract::Trex);
        g.phase = Phase::Play;
        g.hands = [vec![c(8, 2)], vec![c(11, 2)]];
        g.side = Player::One;
        assert_eq!(g.legal_moves(), vec![Move::Pass]);
        g.apply(&Move::Pass);
        g.apply(&Move::Play(c(11, 2)));
        assert_eq!(g.scores[1], 200);
        assert_eq!(g.deal_no, 1);
    }

    #[test]
    fn king_penalty() {
        let mut g = Trix::with_seed(1);
        g.contract = Some(Contract::King);
        g.phase = Phase::Play;
        g.hands = [vec![c(13, 0), c(7, 1)], vec![c(14, 0), c(8, 1)]];
        g.side = Player::One;
        g.apply(&Move::Play(c(13, 0)));
        g.apply(&Move::Play(c(14, 0)));
        assert_eq!(g.scores[1], -75);
        assert_eq!(g.deal_no, 1); // mâna se încheie imediat ce cade popa
    }

    #[test]
    fn owner_switch_and_contract_once() {
        let mut g = Trix::with_seed(1);
        assert_eq!(g.legal_moves().len(), 5);
        g.apply(&Move::Choose(Contract::King));
        g.deal_no = 1;
        g.phase = Phase::Choose;
        assert_eq!(g.legal_moves().len(), 4);
        g.deal_no = 5;
        assert_eq!(g.legal_moves().len(), 5); // J2 are propriile contracte
        assert_eq!(g.owner(), Player::Two);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=10u64 {
            let mut g = Trix::with_seed(seed);
            let mut rng = Rng::new(seed + 5);
            let mut n = 0;
            while g.outcome().is_none() && n < 10000 {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed}");
            assert_eq!(g.deal_no, 10);
        }
    }
}

#[cfg(test)]
mod a2_audit {
    use super::*;

    #[test]
    fn ai_nu_vede_mana_adversarului_la_orice_nivel() {
        crate::p3_comun::ai_blind_check(
            "trix",
            Trix::with_seed,
            |g, me, seed| {
                let o = me.other().idx();
                let mut pool: Vec<Card> = full_deck(7)
                    .into_iter()
                    .filter(|c| {
                        !g.hands[me.idx()].contains(c) && !g.trick.iter().any(|&(_, x)| x == *c)
                    })
                    .collect();
                crate::p3_comun::shuffle_with(&mut pool, seed);
                let n = g.hands[o].len();
                g.hands[o] = pool.into_iter().take(n).collect();
                sort_hand(&mut g.hands[o]);
                g.rng = Rng::new(seed ^ 0x7217);
            },
            12,
        );
    }
}
