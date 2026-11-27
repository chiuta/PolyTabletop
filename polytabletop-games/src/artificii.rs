//! Artificii — joc cooperativ cu informație parțială, variantă originală, cu partener AI.
//!
//! Voi și partenerul (AI) construiți împreună cinci șiruri de artificii, câte unul pe culoare
//! (R roșu, G galben, V verde, A albastru, W alb), de la 1 la 5, în ordine. Particularitatea:
//! nu vă vedeți propriile cărți, doar pe ale partenerului. Pachet: 50 de cărți (pe culoare:
//! 1×3, 2×2, 3×2, 4×2, 5×1), câte 5 cărți în mână, 8 jetoane de indiciu, 3 fitile.
//!
//! Tura ta: (a) joci o carte din mâna ta (juc 3 = poziția 3); dacă e următoarea din șirul
//! culorii, urcă șirul, altfel arde o fitilă (a treia = pierdeți); (b) arunci o carte
//! (arunc 3) și recuperezi un jetron (nu cu 8); (c) dai un indiciu partenerului: o culoare
//! (indiciu R) sau un număr (indiciu 3) — arăți toate cărțile lui de acel fel (cel puțin
//! una), costă un jetron. Cartea 5 jucată corect dă un jetron înapoi. După ce pachetul se
//! golește, fiecare mai are o tură. Scor = suma șirurilor (max 25). Obiectiv: 20 de puncte.
//!
//! Rezultat: echipa reușește (J1 „câștigă”) la cel puțin 20 de puncte; altfel remiză
//! (echipa nu a atins obiectivul); trei fitile arse = scor 0.
//!
//! Informație ascunsă: mâna ta nu apare; vezi mâna AI-ului și ce ți-a arătat prin indicii.
//! AI-ul nu își citește cărțile: raționează din indicii primite și din cărțile vizibile
//! (șiruri, aruncate, mâna ta). Încheierea jocului e o mutare forțată (`sfârșit`), ca AI-ul
//! să nu „vadă” dinainte dacă o carte jucată reușește.
#![allow(clippy::needless_range_loop, clippy::doc_lazy_continuation)]

use crate::cards::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const COLORS: [char; 5] = ['R', 'G', 'V', 'A', 'W'];
pub const GOAL: u32 = 20;
const COPIES: [i32; 6] = [0, 3, 2, 2, 2, 1];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Card {
    pub c: u8,
    pub n: u8,
}

impl Card {
    pub fn text(self) -> String {
        format!("{}{}", COLORS[self.c as usize], self.n)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Know {
    col: Option<u8>,
    num: Option<u8>,
    not_col: u8,
    not_num: u8,
}

impl Know {
    fn allows(&self, c: u8, n: u8) -> bool {
        self.col.is_none_or(|x| x == c)
            && self.num.is_none_or(|x| x == n)
            && self.not_col & (1 << c) == 0
            && self.not_num & (1 << n) == 0
    }
    fn describe(&self) -> String {
        let c = match self.col {
            Some(c) => COLORS[c as usize].to_string(),
            None => "?".into(),
        };
        let n = match self.num {
            Some(n) => n.to_string(),
            None => "?".into(),
        };
        let mut s = format!("{c}{n}");
        let mut nots = String::new();
        for i in 0..5u8 {
            if self.col.is_none() && self.not_col & (1 << i) != 0 {
                nots.push(COLORS[i as usize]);
            }
        }
        for i in 1..=5u8 {
            if self.num.is_none() && self.not_num & (1 << i) != 0 {
                nots.push(char::from(b'0' + i));
            }
        }
        if !nots.is_empty() {
            s.push_str(&format!(" (nu {nots})"));
        }
        s
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Play(u8),
    Discard(u8),
    HintColor(u8),
    HintNum(u8),
    Finish,
}

#[derive(Clone)]
pub struct Artificii {
    deck: Vec<Card>,
    hands: [Vec<Card>; 2],
    know: [Vec<Know>; 2],
    piles: [u8; 5],
    discards: Vec<Card>,
    hints: u8,
    fuses: u8,
    side: Player,
    final_left: Option<u8>,
    pending_end: bool,
    over: bool,
    heur: (usize, i32),
    last: String,
    /// `Some(p)` = render shows `p`'s private cards; `None` = hot-seat (the mover's).
    viewer: Option<Player>,
}

impl Default for Artificii {
    fn default() -> Self {
        Self::new()
    }
}

impl Artificii {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck = Vec::new();
        for c in 0..5u8 {
            for n in 1..=5u8 {
                for _ in 0..COPIES[n as usize] {
                    deck.push(Card { c, n });
                }
            }
        }
        shuffle(&mut deck, &mut rng);
        let mut hands = [Vec::new(), Vec::new()];
        for _ in 0..5 {
            hands[0].push(deck.pop().unwrap());
            hands[1].push(deck.pop().unwrap());
        }
        Artificii {
            deck,
            hands,
            know: [vec![Know::default(); 5], vec![Know::default(); 5]],
            piles: [0; 5],
            discards: Vec::new(),
            hints: 8,
            fuses: 3,
            side: Player::One,
            final_left: None,
            pending_end: false,
            over: false,
            heur: (0, 0),
            last: String::new(),
            viewer: Some(Player::One),
        }
    }

    pub fn score(&self) -> u32 {
        if self.fuses == 0 {
            0
        } else {
            self.piles.iter().map(|x| *x as u32).sum()
        }
    }

    pub fn total_cards(&self) -> usize {
        self.deck.len()
            + self.hands[0].len()
            + self.hands[1].len()
            + self.discards.len()
            + self.piles.iter().map(|x| *x as usize).sum::<usize>()
    }

    /// Copii care nu au apărut încă pe masă sau în aruncate (fără a scădea vreo mână): ce știe oricine
    /// din informația publică. Estimarea făcută de AI pentru ce deduce partenerul nu citește mâna AI-ului.
    fn unseen_public(&self) -> [[i32; 6]; 5] {
        let mut r = [[0i32; 6]; 5];
        for c in 0..5 {
            for n in 1..=5usize {
                r[c][n] = COPIES[n];
            }
            for n in 1..=self.piles[c] as usize {
                r[c][n] -= 1;
            }
        }
        for d in &self.discards {
            r[d.c as usize][d.n as usize] -= 1;
        }
        r
    }

    /// Copii nevăzute de `viewer` (pachet + propria mână), pe (culoare, număr).
    fn unseen(&self, viewer: usize) -> [[i32; 6]; 5] {
        let mut r = [[0i32; 6]; 5];
        for c in 0..5 {
            for n in 1..=5usize {
                r[c][n] = COPIES[n];
            }
            for n in 1..=self.piles[c] as usize {
                r[c][n] -= 1;
            }
        }
        for d in &self.discards {
            r[d.c as usize][d.n as usize] -= 1;
        }
        for h in &self.hands[1 - viewer] {
            r[h.c as usize][h.n as usize] -= 1;
        }
        r
    }

    /// (P(jucabilă), P(inutilă), P(critică)) pentru o carte cu cunoștințele `k`.
    fn probs(&self, rem: &[[i32; 6]; 5], k: &Know) -> (f64, f64, f64) {
        let (mut tot, mut play, mut useless, mut crit) = (0.0, 0.0, 0.0, 0.0);
        for c in 0..5u8 {
            for n in 1..=5u8 {
                let w = rem[c as usize][n as usize];
                if w <= 0 || !k.allows(c, n) {
                    continue;
                }
                let w = f64::from(w);
                tot += w;
                if self.piles[c as usize] + 1 == n {
                    play += w;
                } else if n <= self.piles[c as usize] {
                    useless += w;
                } else if rem[c as usize][n as usize] == 1 {
                    crit += w;
                }
            }
        }
        if tot == 0.0 {
            return (0.0, 0.0, 0.0);
        }
        (play / tot, useless / tot, crit / tot)
    }

    fn threshold(&self) -> f64 {
        match self.fuses {
            3 => 0.7,
            2 => 0.85,
            _ => 0.97,
        }
    }

    fn hint_effect(&self, from: usize, m: &Move) -> Option<Vec<Know>> {
        let to = 1 - from;
        let mut nk = self.know[to].clone();
        let mut touched = 0;
        for (i, c) in self.hands[to].iter().enumerate() {
            let hit = match m {
                Move::HintColor(x) => c.c == *x,
                Move::HintNum(x) => c.n == *x,
                _ => return None,
            };
            if hit {
                touched += 1;
            }
            match m {
                Move::HintColor(x) => {
                    if hit {
                        nk[i].col = Some(*x);
                    } else {
                        nk[i].not_col |= 1 << x;
                    }
                }
                Move::HintNum(x) => {
                    if hit {
                        nk[i].num = Some(*x);
                    } else {
                        nk[i].not_num |= 1 << x;
                    }
                }
                _ => {}
            }
        }
        if touched == 0 {
            None
        } else {
            Some(nk)
        }
    }

    /// Euristică ex-ante, din punctul de vedere al lui `p` (nu citește cărțile lui `p`).
    fn move_score(&self, p: usize, m: &Move) -> i32 {
        let thr = self.threshold();
        match *m {
            Move::Play(i) => {
                let rem = self.unseen(p);
                let (pl, _, cr) = self.probs(&rem, &self.know[p][i as usize]);
                let fuse_cost = if self.fuses == 1 { 400.0 } else { 140.0 };
                (120.0 * pl - fuse_cost * (1.0 - pl) - 30.0 * cr * (1.0 - pl)) as i32
            }
            Move::Discard(i) => {
                let rem = self.unseen(p);
                let (pl, us, cr) = self.probs(&rem, &self.know[p][i as usize]);
                let bonus = if self.hints <= 2 { 25.0 } else { 6.0 };
                (bonus + 45.0 * us - 150.0 * cr - 80.0 * pl) as i32
            }
            Move::HintColor(_) | Move::HintNum(_) => {
                let Some(nk) = self.hint_effect(p, m) else {
                    return -1000;
                };
                let to = 1 - p;
                // ce deduce partenerul: din informația publică (mâna mea nu se citește)
                let rem = self.unseen_public();
                let mut s = -6.0;
                for (i, c) in self.hands[to].iter().enumerate() {
                    let (pb, ub, _) = self.probs(&rem, &self.know[to][i]);
                    let (pa, ua, _) = self.probs(&rem, &nk[i]);
                    let actual = self.piles[c.c as usize] + 1 == c.n;
                    let would_after = pa >= thr;
                    let would_before = pb >= thr;
                    if would_after && !would_before {
                        s += if actual { 60.0 } else { -110.0 };
                    }
                    if ua >= 0.99 && ub < 0.99 && c.n <= self.piles[c.c as usize] {
                        s += 14.0;
                    }
                    // Protejează 5-urile și ultimele exemplare.
                    let crit = c.n == 5
                        || self.discards.iter().filter(|d| **d == *c).count() as i32 + 1
                            >= COPIES[c.n as usize];
                    if crit && c.n > self.piles[c.c as usize] && nk[i] != self.know[to][i] {
                        s += 8.0;
                    }
                    if nk[i] != self.know[to][i] {
                        s += 1.5;
                    }
                }
                if self.hints <= 2 {
                    s -= 12.0;
                }
                s as i32
            }
            Move::Finish => 0,
        }
    }

    fn end_turn(&mut self) {
        if let Some(f) = self.final_left.as_mut() {
            *f -= 1;
            if *f == 0 {
                self.pending_end = true;
            }
        }
        if self.fuses == 0 || self.piles.iter().all(|x| *x == 5) {
            self.pending_end = true;
        }
        self.side = self.side.other();
    }

    fn refill(&mut self, p: usize) {
        if let Some(c) = self.deck.pop() {
            self.hands[p].push(c);
            self.know[p].push(Know::default());
            if self.deck.is_empty() && self.final_left.is_none() {
                self.final_left = Some(2);
            }
        }
    }
}

impl Game for Artificii {
    type Move = Move;

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over {
            return Vec::new();
        }
        if self.pending_end {
            return vec![Move::Finish];
        }
        let p = self.side.idx();
        let n = self.hands[p].len() as u8;
        let mut v = Vec::new();
        for i in 0..n {
            v.push(Move::Play(i));
        }
        if self.hints < 8 {
            for i in 0..n {
                v.push(Move::Discard(i));
            }
        }
        if self.hints > 0 {
            let o = &self.hands[1 - p];
            for c in 0..5u8 {
                if o.iter().any(|x| x.c == c) {
                    v.push(Move::HintColor(c));
                }
            }
            for k in 1..=5u8 {
                if o.iter().any(|x| x.n == k) {
                    v.push(Move::HintNum(k));
                }
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        let p = self.side.idx();
        self.heur = (p, self.move_score(p, m));
        let who = format!("J{}", p + 1);
        match *m {
            Move::Finish => {
                self.over = true;
            }
            Move::Play(i) => {
                let c = self.hands[p].remove(i as usize);
                self.know[p].remove(i as usize);
                if self.piles[c.c as usize] + 1 == c.n {
                    self.piles[c.c as usize] += 1;
                    if c.n == 5 && self.hints < 8 {
                        self.hints += 1;
                    }
                    self.last = format!("{who} a jucat {} cu succes.", c.text());
                } else {
                    self.fuses -= 1;
                    self.discards.push(c);
                    self.last = format!("{who} a jucat {}: fitilă arsă!", c.text());
                }
                self.refill(p);
                self.end_turn();
            }
            Move::Discard(i) => {
                let c = self.hands[p].remove(i as usize);
                self.know[p].remove(i as usize);
                self.discards.push(c);
                self.hints += 1;
                self.last = format!("{who} a aruncat {}.", c.text());
                self.refill(p);
                self.end_turn();
            }
            Move::HintColor(_) | Move::HintNum(_) => {
                let nk = self.hint_effect(p, m).expect("legal hint");
                self.know[1 - p] = nk;
                self.hints -= 1;
                self.last = match m {
                    Move::HintColor(c) => {
                        format!("{who} a dat indiciu de culoare {}.", COLORS[*c as usize])
                    }
                    Move::HintNum(n) => format!("{who} a dat indiciu de număr {n}."),
                    _ => String::new(),
                };
                self.end_turn();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            return None;
        }
        Some(if self.score() >= GOAL {
            Outcome::Win(Player::One)
        } else {
            Outcome::Draw
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        if self.heur.0 == p.idx() {
            self.heur.1
        } else {
            -self.heur.1
        }
    }

    fn ai_style(&self) -> AiStyle {
        // Adâncimea 0 = o singură mutare la TOATE nivelurile: la Hard/Expert adâncimea 1 ar extinde
        // răspunsurile adversarului din mâna lui reală (informație ascunsă).
        AiStyle::AlphaBeta(0)
    }

    fn render(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            "Artificii — scor {} (obiectiv {GOAL}) | indicii {} | fitile {} | pachet {}\n",
            self.score(),
            self.hints,
            self.fuses,
            self.deck.len()
        ));
        let piles: Vec<String> = (0..5)
            .map(|c| format!("{}{}", COLORS[c], self.piles[c]))
            .collect();
        s.push_str(&format!("Șiruri: {}\n", piles.join("  ")));
        if !self.discards.is_empty() {
            let mut d = self.discards.clone();
            d.sort_by_key(|c| (c.c, c.n));
            s.push_str(&format!(
                "Aruncate: {}\n",
                d.iter().map(|c| c.text()).collect::<Vec<_>>().join(" ")
            ));
        }
        if !self.last.is_empty() {
            s.push_str(&format!("Ultima acțiune: {}\n", self.last));
        }
        let v = self.viewer.unwrap_or(self.side);
        let (vi, oi) = (v.idx(), v.other().idx());
        s.push_str(&format!(
            "Mâna partenerului (J{}, o vezi): {}\n",
            oi + 1,
            self.hands[oi]
                .iter()
                .map(|c| c.text())
                .collect::<Vec<_>>()
                .join(" ")
        ));
        s.push_str(&format!("Cărțile tale (J{}), după indicii:\n", vi + 1));
        for (i, k) in self.know[vi].iter().enumerate() {
            s.push_str(&format!("  {}: {}\n", i + 1, k.describe()));
        }
        if self.over {
            s.push_str(&format!("Joc încheiat. Scor final: {}.\n", self.score()));
        } else if self.pending_end {
            s.push_str("Ultima tură s-a încheiat: sfârșit.\n");
        } else {
            s.push_str("Mutări: juc N, arunc N, indiciu R|G|V|A|W, indiciu 1..5.\n");
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Play(i) => format!("juc {}", i + 1),
            Move::Discard(i) => format!("arunc {}", i + 1),
            Move::HintColor(c) => format!("indiciu {}", COLORS[*c as usize]),
            Move::HintNum(n) => format!("indiciu {n}"),
            Move::Finish => "sfârșit".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_ai_game(seed: u64) -> (u32, usize) {
        let mut g = Artificii::with_seed(seed);
        let mut rng = Rng::new(seed ^ 77);
        let mut n = 0;
        while g.outcome().is_none() {
            let m = polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap();
            g.apply(&m);
            assert_eq!(g.total_cards(), 50);
            n += 1;
            assert!(n < 500);
        }
        (g.score(), n)
    }

    #[test]
    fn deck_composition() {
        let g = Artificii::with_seed(1);
        assert_eq!(g.total_cards(), 50);
        let mut all = g.deck.clone();
        all.extend(g.hands[0].iter().copied());
        all.extend(g.hands[1].iter().copied());
        assert_eq!(all.iter().filter(|c| c.n == 1).count(), 15);
        assert_eq!(all.iter().filter(|c| c.n == 5).count(), 5);
    }

    #[test]
    fn hint_rules() {
        let mut g = Artificii::with_seed(2);
        g.hints = 0;
        assert!(g
            .legal_moves()
            .iter()
            .all(|m| !matches!(m, Move::HintColor(_) | Move::HintNum(_))));
        g.hints = 8;
        assert!(g
            .legal_moves()
            .iter()
            .all(|m| !matches!(m, Move::Discard(_))));
        let mut g = Artificii::with_seed(2);
        let m = g
            .legal_moves()
            .into_iter()
            .find(|m| matches!(m, Move::HintColor(_)))
            .unwrap();
        g.apply(&m);
        assert_eq!(g.hints, 7);
        let touched = g.know[1].iter().filter(|k| k.col.is_some()).count();
        assert!(touched >= 1);
    }

    #[test]
    fn misplay_burns_fuse_and_three_end() {
        let mut g = Artificii::with_seed(3);
        g.hands[0][0] = Card { c: 0, n: 5 };
        g.apply(&Move::Play(0));
        assert_eq!(g.fuses, 2);
        assert_eq!(g.total_cards(), 50);
        g.fuses = 1;
        g.side = Player::One;
        g.hands[0][0] = Card { c: 0, n: 4 };
        g.apply(&Move::Play(0));
        assert_eq!(g.score(), 0);
        assert_eq!(g.legal_moves(), vec![Move::Finish]);
        g.apply(&Move::Finish);
        assert_eq!(g.outcome(), Some(Outcome::Draw));
    }

    #[test]
    fn successful_play_and_five_bonus() {
        let mut g = Artificii::with_seed(4);
        g.hints = 5;
        g.piles[1] = 4;
        g.hands[0][2] = Card { c: 1, n: 5 };
        g.apply(&Move::Play(2));
        assert_eq!(g.piles[1], 5);
        assert_eq!(g.hints, 6);
    }

    #[test]
    fn ai_team_scores_reasonably() {
        let mut total = 0;
        let mut ended = 0;
        for seed in 1..=40u64 {
            let (s, _) = run_ai_game(seed);
            total += s;
            ended += 1;
        }
        let avg = f64::from(total) / f64::from(ended);
        assert!(avg >= 9.0, "average score {avg}");
    }
}

#[cfg(test)]
mod viewer_tests {
    use super::*;

    #[test]
    fn viewer_sees_partner_hand_and_own_clues_only() {
        let mut g = Artificii::with_seed(5);
        let hand = |p: Player| {
            g.hands[p.idx()]
                .iter()
                .map(|c| c.text())
                .collect::<Vec<_>>()
                .join(" ")
        };
        let (h1, h2) = (hand(Player::One), hand(Player::Two));
        Game::set_viewer(&mut g, Some(Player::One));
        let r = Game::render(&g);
        assert!(r.contains(&format!("(J2, o vezi): {h2}")), "{r}");
        assert!(r.contains("Cărțile tale (J1)") && !r.contains(&h1), "{r}");
        Game::set_viewer(&mut g, Some(Player::Two));
        let r = Game::render(&g);
        assert!(r.contains(&format!("(J1, o vezi): {h1}")), "{r}");
        assert!(r.contains("Cărțile tale (J2)") && !r.contains(&h2), "{r}");
        Game::set_viewer(&mut g, None);
        assert!(Game::render(&g).contains("(J2, o vezi)"));
    }
}

#[cfg(test)]
mod a3_audit {
    use super::*;

    #[test]
    fn ai_ignores_its_own_hidden_hand_and_the_deck_at_every_level() {
        // cooperativ (stil Hanabi): informația ascunsă pentru cel la mutare e propria mână și pachetul
        crate::p3_comun::ai_blind_check(
            "artificii",
            Artificii::with_seed,
            |g, me, s| {
                let i = me.idx();
                crate::p3_comun::mix_unseen(&mut g.hands[i], &mut g.deck, 0, s);
            },
            10,
        );
    }
}
