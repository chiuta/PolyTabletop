//! Canasta (variantă pentru 2 jucători, o singură mână), din regulile publice.
//!
//! * 108 de cărți (două pachete + 4 jokeri); câte 15 cărți fiecăruia. Jokerii și 2-urile sunt
//!   „wild”. 3-urile roșii se pun deoparte (100 de puncte fiecare, 800 toate patru) și se înlocuiesc;
//!   3-urile negre nu se așază niciodată în combinații (doar se aruncă, ca blocaj).
//! * Tura: (1) `roba` — iei 2 cărți din stoc — sau `ia` — iei toată aruncarea; (2) așezi combinații;
//!   (3) arunci o carte (`d 7h`). Cine rămâne fără cărți (cu cel puțin o canastă) a ieșit.
//! * Combinație = cel puțin 3 cărți de același rang, cu cel puțin 2 naturale, mai multe naturale decât
//!   wild-uri și cel mult 3 wild-uri. `m 7` așază toate naturalele de 7 (minim 3) sau le adaugă
//!   la combinația existentă; `mw 7` așază 2 naturale + 1 wild; `w 7` adaugă un wild (nu la o
//!   canastă naturală). Canastă = 7+ cărți: naturală 500, mixtă 300.
//! * `ia` (aruncarea): cartea de sus trebuie să fie naturală (nu 3, nu wild) și se folosește imediat:
//!   la o combinație existentă (dacă aruncarea nu e „înghețată”), sau cu 2 naturale din mână, sau (neînghețat)
//!   cu 1 natural + 1 wild. Aruncarea e înghețată dacă are un wild în ea sau dacă n-ai nicio combinație.
//! * Mâna se termină când cineva iese sau stocul e gol și nu se poate lua aruncarea. Scor: cărțile
//!   așezate + bonusuri (canaste, 3 roșii, ieșire 100) − cărțile rămase în mână. Câștigă scorul mai mare.
//! * Simplificări: fără minim de coborâre, o singură mână (nu până la 5000), fără 3 negre în combinații.
//!
//! Mutări: `roba`, `ia`, `m 7`, `mw 7`, `w K`, `d 9c`, `d Jk`, `pas` (doar când nu ai altă mutare). Informația ascunsă: stocul și mâna
//! adversarului; AI-ul citește doar mâna proprie și informația publică (combinații, aruncare).

use crate::cards::{join_cards, shuffle, Card, Viewer};
use crate::cardsb_util::sort_hand;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const JOKER: u8 = 15;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Draw,
    Meld,
    Over,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Draw,
    TakePile,
    Meld(u8),
    MeldWild(u8),
    AddWild(u8),
    Discard(Card),
    /// Termină tura fără aruncare (doar când nu ai nicio altă mutare).
    Pass,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct MeldGroup {
    rank: u8,
    nat: u8,
    wild: u8,
    pts: i32,
}

impl MeldGroup {
    fn size(&self) -> u8 {
        self.nat + self.wild
    }
    fn is_canasta(&self) -> bool {
        self.size() >= 7
    }
}

#[derive(Clone)]
pub struct Canasta {
    stock: Vec<Card>,
    pile: Vec<Card>,
    hands: [Vec<Card>; 2],
    melds: [Vec<MeldGroup>; 2],
    red3: [u8; 2],
    side: Player,
    phase: Phase,
    out: Option<Player>,
    /// Mâna și 3-urile roșii ale celui la mutare chiar înainte de `roba` (cărțile trase din stoc nu sunt
    /// cunoscute la alegere): evaluarea pleacă de la ele, ca AI-ul să nu vadă vârful stocului.
    pre_draw: Option<(Vec<Card>, u8)>,
    log: String,
    viewer: Viewer,
}

impl Default for Canasta {
    fn default() -> Self {
        Self::new()
    }
}

fn is_wild(c: Card) -> bool {
    c.rank == 2 || c.rank == JOKER
}
fn is_red3(c: Card) -> bool {
    c.rank == 3 && c.suit < 2
}
fn card_pts(c: Card) -> i32 {
    match c.rank {
        JOKER => 50,
        2 | 14 => 20,
        8..=13 => 10,
        _ => 5,
    }
}
fn ctext(c: Card) -> String {
    if c.rank == JOKER {
        "Jk".into()
    } else {
        c.text()
    }
}
fn rlabel(r: u8) -> String {
    Card { rank: r, suit: 0 }.rank_label().to_string()
}
fn joined(cs: &[Card]) -> String {
    if cs.is_empty() {
        return join_cards(cs);
    }
    cs.iter().map(|&c| ctext(c)).collect::<Vec<_>>().join(" ")
}

impl Canasta {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut d: Vec<Card> = Vec::new();
        for _ in 0..2 {
            for suit in 0..4u8 {
                for rank in 2..=14u8 {
                    d.push(Card { rank, suit });
                }
            }
            d.push(Card {
                rank: JOKER,
                suit: 0,
            });
            d.push(Card {
                rank: JOKER,
                suit: 0,
            });
        }
        shuffle(&mut d, &mut rng);
        let mut g = Canasta {
            stock: d,
            pile: Vec::new(),
            hands: [Vec::new(), Vec::new()],
            melds: [Vec::new(), Vec::new()],
            red3: [0, 0],
            side: Player::One,
            phase: Phase::Draw,
            out: None,
            pre_draw: None,
            log: String::new(),
            viewer: Viewer::Fixed(Player::One),
        };
        for p in 0..2 {
            for _ in 0..15 {
                let c = g.stock.pop().unwrap();
                g.hands[p].push(c);
            }
            g.fix_red3(p);
        }
        // prima carte a aruncării: naturală, nu 3
        loop {
            let c = g.stock.pop().unwrap();
            if !is_wild(c) && c.rank != 3 {
                g.pile.push(c);
                break;
            }
            g.stock.insert(0, c);
        }
        sort_hand(&mut g.hands[0]);
        sort_hand(&mut g.hands[1]);
        g
    }

    /// Mută 3-urile roșii din mâna lui `p` și le înlocuiește.
    fn fix_red3(&mut self, p: usize) {
        while let Some(pos) = self.hands[p].iter().position(|&c| is_red3(c)) {
            self.hands[p].remove(pos);
            self.red3[p] += 1;
            if let Some(c) = self.stock.pop() {
                self.hands[p].push(c);
            }
        }
    }

    pub fn totals(&self) -> (i32, i32) {
        (
            self.total(Player::One, false),
            self.total(Player::Two, false),
        )
    }

    /// Scorul lui `p`. `vis` = estimare cu informații doar ale lui p (mâna proprie la jumătate).
    fn total(&self, p: Player, vis: bool) -> i32 {
        self.total_from(p, vis, &self.hands[p.idx()], self.red3[p.idx()])
    }

    fn total_from(&self, p: Player, vis: bool, hand: &[Card], red3: u8) -> i32 {
        let i = p.idx();
        let mut s: i32 = self.melds[i].iter().map(|m| m.pts).sum();
        for m in &self.melds[i] {
            if m.is_canasta() {
                s += if m.wild == 0 { 500 } else { 300 };
            }
        }
        let r = match red3 {
            4 => 800,
            n => 100 * n as i32,
        };
        s += if self.melds[i].is_empty() { -r } else { r };
        if self.out == Some(p) {
            s += 100;
        }
        let hp: i32 = hand.iter().map(|&c| card_pts(c)).sum();
        s -= if vis { hp / 2 } else { hp };
        s
    }

    fn has_canasta(&self, p: usize) -> bool {
        self.melds[p].iter().any(|m| m.is_canasta())
    }

    fn frozen(&self, p: usize) -> bool {
        self.pile.iter().any(|&c| is_wild(c)) || self.melds[p].is_empty()
    }

    fn count_nat(&self, p: usize, rank: u8) -> usize {
        self.hands[p].iter().filter(|c| c.rank == rank).count()
    }

    fn count_wild(&self, p: usize) -> usize {
        self.hands[p].iter().filter(|&&c| is_wild(c)).count()
    }

    /// Scenariul cu care se poate lua aruncarea: 0 = adaugă la combinație, 1 = 2 naturale, 2 = natural + wild.
    fn take_plan(&self, p: usize) -> Option<u8> {
        let top = *self.pile.last()?;
        if is_wild(top) || top.rank == 3 {
            return None;
        }
        let r = top.rank;
        let existing = self.melds[p].iter().any(|m| m.rank == r);
        if existing && !self.frozen(p) {
            return Some(0);
        }
        if self.count_nat(p, r) >= 2 {
            return Some(1);
        }
        if !self.frozen(p) && self.count_nat(p, r) >= 1 && self.count_wild(p) >= 1 {
            return Some(2);
        }
        None
    }

    fn remove_cards(&mut self, p: usize, rank: u8, n: usize) -> i32 {
        let mut pts = 0;
        for _ in 0..n {
            let pos = self.hands[p].iter().position(|c| c.rank == rank).unwrap();
            pts += card_pts(self.hands[p].remove(pos));
        }
        pts
    }

    fn remove_wild(&mut self, p: usize) -> i32 {
        let pos = self.hands[p].iter().position(|&c| is_wild(c)).unwrap();
        card_pts(self.hands[p].remove(pos))
    }

    fn add_to_meld(&mut self, p: usize, rank: u8, nat: u8, wild: u8, pts: i32) {
        if let Some(m) = self.melds[p].iter_mut().find(|m| m.rank == rank) {
            m.nat += nat;
            m.wild += wild;
            m.pts += pts;
        } else {
            self.melds[p].push(MeldGroup {
                rank,
                nat,
                wild,
                pts,
            });
        }
    }

    /// După o combinație: rămân suficiente cărți (sau există canastă)?
    fn hand_ok_after(&self, p: usize, used: usize, extra_canasta: bool) -> bool {
        let left = self.hands[p].len().saturating_sub(used);
        left >= 2 || self.has_canasta(p) || extra_canasta
    }

    fn finish_turn(&mut self, p: Player) {
        let o = p.other();
        self.side = o;
        self.phase = Phase::Draw;
        if self.stock.is_empty() && self.take_plan(o.idx()).is_none() {
            self.phase = Phase::Over;
        }
    }
}

impl Game for Canasta {
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
        let p = self.side.idx();
        match self.phase {
            Phase::Over => Vec::new(),
            Phase::Draw => {
                let mut v = Vec::new();
                if !self.stock.is_empty() {
                    v.push(Move::Draw);
                }
                if self.take_plan(p).is_some() {
                    v.push(Move::TakePile);
                }
                v
            }
            Phase::Meld => {
                let mut v = Vec::new();
                for r in 4..=14u8 {
                    let n = self.count_nat(p, r);
                    let existing = self.melds[p].iter().find(|m| m.rank == r).copied();
                    match existing {
                        Some(m) => {
                            if n >= 1 {
                                let canasta_after = m.size() + n as u8 >= 7;
                                if self.hand_ok_after(p, n, canasta_after) {
                                    v.push(Move::Meld(r));
                                }
                            }
                            if self.count_wild(p) >= 1
                                && m.wild < 3
                                && m.nat >= m.wild + 2
                                && !(m.is_canasta() && m.wild == 0)
                                && self.hand_ok_after(p, 1, false)
                            {
                                v.push(Move::AddWild(r));
                            }
                        }
                        None => {
                            if n >= 3 && self.hand_ok_after(p, n, n >= 7) {
                                v.push(Move::Meld(r));
                            }
                            if n >= 2 && self.count_wild(p) >= 1 && self.hand_ok_after(p, 3, false)
                            {
                                v.push(Move::MeldWild(r));
                            }
                        }
                    }
                }
                let hand = &self.hands[p];
                let mut seen: Vec<Card> = Vec::new();
                for &c in hand {
                    if seen.contains(&c) {
                        continue;
                    }
                    seen.push(c);
                    if hand.len() >= 2 || self.has_canasta(p) {
                        v.push(Move::Discard(c));
                    }
                }
                if v.is_empty() {
                    v.push(Move::Pass);
                }
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        self.pre_draw = None;
        let p = self.side;
        let pi = p.idx();
        match *m {
            Move::Draw => {
                self.pre_draw = Some((self.hands[pi].clone(), self.red3[pi]));
                for _ in 0..2 {
                    if let Some(c) = self.stock.pop() {
                        self.hands[pi].push(c);
                        self.fix_red3(pi);
                    }
                }
                sort_hand(&mut self.hands[pi]);
                self.phase = Phase::Meld;
            }
            Move::TakePile => {
                let plan = self.take_plan(pi).unwrap();
                let top = self.pile.pop().unwrap();
                let rest: Vec<Card> = std::mem::take(&mut self.pile);
                self.log = format!("J{} ia aruncarea ({} cărți).", pi + 1, rest.len() + 1);
                let r = top.rank;
                match plan {
                    0 => {
                        self.add_to_meld(pi, r, 1, 0, card_pts(top));
                    }
                    1 => {
                        let pts = self.remove_cards(pi, r, 2) + card_pts(top);
                        self.add_to_meld(pi, r, 3, 0, pts);
                    }
                    _ => {
                        let pts =
                            self.remove_cards(pi, r, 1) + self.remove_wild(pi) + card_pts(top);
                        self.add_to_meld(pi, r, 2, 1, pts);
                    }
                }
                self.hands[pi].extend(rest);
                for c in self.hands[pi].clone() {
                    if is_red3(c) {
                        // nu ar trebui să existe, dar protejăm
                        let pos = self.hands[pi].iter().position(|&x| x == c).unwrap();
                        self.hands[pi].remove(pos);
                        self.red3[pi] += 1;
                    }
                }
                sort_hand(&mut self.hands[pi]);
                self.phase = Phase::Meld;
                if self.hands[pi].is_empty() && self.has_canasta(pi) {
                    self.out = Some(p);
                    self.phase = Phase::Over;
                }
            }
            Move::Meld(r) => {
                let n = self.count_nat(pi, r);
                let pts = self.remove_cards(pi, r, n);
                self.add_to_meld(pi, r, n as u8, 0, pts);
                if self.hands[pi].is_empty() {
                    self.out = Some(p);
                    self.phase = Phase::Over;
                }
            }
            Move::MeldWild(r) => {
                let pts = self.remove_cards(pi, r, 2) + self.remove_wild(pi);
                self.add_to_meld(pi, r, 2, 1, pts);
            }
            Move::AddWild(r) => {
                let pts = self.remove_wild(pi);
                self.add_to_meld(pi, r, 0, 1, pts);
            }
            Move::Pass => {
                self.finish_turn(p);
            }
            Move::Discard(c) => {
                let pos = self.hands[pi].iter().position(|&x| x == c).unwrap();
                self.hands[pi].remove(pos);
                self.pile.push(c);
                if self.hands[pi].is_empty() {
                    self.out = Some(p);
                    self.phase = Phase::Over;
                } else {
                    self.finish_turn(p);
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        let (a, b) = self.totals();
        Some(match a.cmp(&b) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let q = p.other();
        // adversarul: doar ce e public (cărțile lui din mână nu se citesc)
        let opp = {
            let i = q.idx();
            let mut s: i32 = self.melds[i].iter().map(|m| m.pts).sum();
            for m in &self.melds[i] {
                if m.is_canasta() {
                    s += if m.wild == 0 { 500 } else { 300 };
                }
            }
            s + 100 * self.red3[i] as i32
        };
        let mut s = match &self.pre_draw {
            // două cărți necunoscute: în medie ~10 puncte fiecare, socotite pe jumătate (ca restul mâinii)
            Some((hand, red3)) if self.side == p => self.total_from(p, true, hand, *red3) - 10,
            _ => self.total(p, true),
        } - opp;
        // pericol: aruncarea de sus poate fi luată de adversar
        if let Some(&top) = self.pile.last() {
            if self.side == q && self.phase == Phase::Draw {
                if !is_wild(top)
                    && top.rank != 3
                    && self.melds[q.idx()].iter().any(|m| m.rank == top.rank)
                {
                    s -= 30 + 2 * self.pile.len() as i32;
                }
                if top.rank == 3 {
                    s += 10;
                }
            }
        }
        s
    }

    fn render(&self) -> String {
        let mut s = String::from("Canasta (2 jucători, o mână)\n");
        let melds = |p: usize| -> String {
            if self.melds[p].is_empty() {
                return "—".into();
            }
            self.melds[p]
                .iter()
                .map(|m| {
                    format!(
                        "{}×{}{}{}",
                        rlabel(m.rank),
                        m.size(),
                        if m.wild > 0 {
                            format!("(+{} wild)", m.wild)
                        } else {
                            String::new()
                        },
                        if m.is_canasta() { "★" } else { "" }
                    )
                })
                .collect::<Vec<_>>()
                .join(" ")
        };
        for p in 0..2 {
            s.push_str(&format!(
                "J{}: combinații {} | 3 roșii: {} | {} cărți în mână\n",
                p + 1,
                melds(p),
                self.red3[p],
                self.hands[p].len()
            ));
        }
        if let Some(l) = self.out {
            s.push_str(&format!("J{} a ieșit.\n", l.idx() + 1));
        }
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        if self.phase == Phase::Over {
            let (a, b) = self.totals();
            s.push_str(&format!("Scor final: J1 {a}, J2 {b}\nJoc încheiat.\n"));
            return s;
        }
        let top = self
            .pile
            .last()
            .map(|&c| ctext(c))
            .unwrap_or_else(|| "—".into());
        s.push_str(&format!(
            "Stoc: {} | aruncare: {} cărți, sus {}{}\n",
            self.stock.len(),
            self.pile.len(),
            top,
            if self.frozen(self.side.idx()) {
                " (înghețată)"
            } else {
                ""
            }
        ));
        let v = self.viewer.who(self.side);
        s.push_str(&format!(
            "Mâna ta (J{}): {}\n",
            v.idx() + 1,
            joined(&self.hands[v.idx()])
        ));
        s.push_str(&format!(
            "Faza: {} | La mutare: Jucătorul {}\n",
            if self.phase == Phase::Draw {
                "tragere"
            } else {
                "combinații / aruncare"
            },
            self.side.idx() + 1
        ));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Draw => "roba".into(),
            Move::TakePile => "ia".into(),
            Move::Meld(r) => format!("m {}", rlabel(*r)),
            Move::MeldWild(r) => format!("mw {}", rlabel(*r)),
            Move::AddWild(r) => format!("w {}", rlabel(*r)),
            Move::Discard(c) => format!("d {}", ctext(*c)),
            Move::Pass => "pas".into(),
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
    fn viewer_controls_whose_hand_is_shown() {
        let mut g = Canasta::with_seed(3);
        g.hands[0] = vec![c(5, 0), c(6, 1), c(7, 2)];
        g.hands[1] = vec![c(9, 3), c(10, 0), c(11, 1)];
        let l = |g: &Canasta, p: usize| format!("{}\n", joined(&g.hands[p]));
        let hand_line = |r: &str, h: &str| {
            r.lines()
                .any(|x| x.starts_with("Mâna ta") && x.ends_with(h.trim_end()))
        };
        Game::set_viewer(&mut g, None);
        for side in [Player::One, Player::Two] {
            g.side = side;
            let r = Game::render(&g);
            assert!(hand_line(&r, &l(&g, side.idx())), "{r}");
            assert!(!r.contains(&joined(&g.hands[side.other().idx()])), "{r}");
        }
        g.side = Player::One;
        Game::set_viewer(&mut g, Some(Player::Two));
        let r = Game::render(&g);
        assert!(hand_line(&r, &l(&g, 1)), "{r}");
        assert!(!r.contains(&joined(&g.hands[0])), "{r}");
    }

    fn setup() -> Canasta {
        let mut g = Canasta::with_seed(1);
        g.hands = [Vec::new(), Vec::new()];
        g.melds = [Vec::new(), Vec::new()];
        g.red3 = [0, 0];
        g.pile = vec![c(9, 0)];
        g.phase = Phase::Meld;
        g.side = Player::One;
        g
    }

    #[test]
    fn deck_and_deal() {
        let g = Canasta::with_seed(3);
        let n = g.stock.len()
            + g.pile.len()
            + g.hands[0].len()
            + g.hands[1].len()
            + (g.red3[0] + g.red3[1]) as usize;
        assert_eq!(n, 108);
        assert_eq!(g.hands[0].len(), 15);
        assert!(!is_wild(g.pile[0]) && g.pile[0].rank != 3);
        assert!(g.hands[0].iter().all(|&c| !is_red3(c)));
    }

    #[test]
    fn meld_and_wild_rules() {
        let mut g = setup();
        g.hands[0] = vec![c(7, 0), c(7, 1), c(2, 0), c(9, 2), c(10, 3)];
        let l = g.legal_moves();
        assert!(l.contains(&Move::MeldWild(7)));
        assert!(!l.contains(&Move::Meld(7))); // doar 2 naturale
        g.apply(&Move::MeldWild(7));
        assert_eq!(g.melds[0][0].size(), 3);
        // acum un 7 natural se poate adăuga, un wild nu (nat 2 < wild+2)
        g.hands[0].push(c(2, 1));
        assert!(!g.legal_moves().contains(&Move::AddWild(7)));
    }

    #[test]
    fn canasta_scoring_and_out() {
        let mut g = setup();
        g.melds[0] = vec![MeldGroup {
            rank: 8,
            nat: 6,
            wild: 0,
            pts: 60,
        }];
        g.hands[0] = vec![c(8, 0), c(5, 1)];
        g.apply(&Move::Meld(8)); // 7 naturale = canastă, rămâne 1 carte (canastă => ok)
        assert!(g.has_canasta(0));
        g.apply(&Move::Discard(c(5, 1)));
        assert_eq!(g.phase, Phase::Over);
        // 70 puncte de cărți + 500 canastă + 100 ieșire
        assert_eq!(g.totals().0, 70 + 500 + 100);
    }

    #[test]
    fn cannot_go_out_without_canasta() {
        let mut g = setup();
        g.hands[0] = vec![c(8, 0)];
        assert_eq!(g.legal_moves(), vec![Move::Pass]);
        g.hands[0] = vec![c(8, 0), c(9, 0)];
        assert!(g.legal_moves().contains(&Move::Discard(c(8, 0))));
    }

    #[test]
    fn take_pile_rules() {
        let mut g = setup();
        g.phase = Phase::Draw;
        g.pile = vec![c(4, 0), c(9, 1)];
        // fără combinații = înghețat: trebuie 2 naturale
        g.hands[0] = vec![c(9, 0), c(2, 0), c(5, 0)];
        assert!(g.take_plan(0).is_none());
        g.hands[0] = vec![c(9, 0), c(9, 2), c(5, 0)];
        assert_eq!(g.take_plan(0), Some(1));
        g.apply(&Move::TakePile);
        assert_eq!(g.melds[0][0].size(), 3);
        assert_eq!(g.hands[0].len(), 2); // 5♥ + 4♥ (restul aruncării)
                                         // wild sus: nu se poate lua
        g.pile = vec![c(2, 0)];
        assert!(g.take_plan(1).is_none());
    }

    #[test]
    fn random_games_end_and_conserve() {
        for seed in 1..=10u64 {
            let mut g = Canasta::with_seed(seed);
            let mut rng = Rng::new(seed + 4);
            let mut n = 0;
            while g.outcome().is_none() && n < 6000 {
                let mv = g.legal_moves();
                assert!(!mv.is_empty(), "{}", g.render());
                g.apply(&mv[rng.below(mv.len())]);
                let total = g.stock.len()
                    + g.pile.len()
                    + g.hands[0].len()
                    + g.hands[1].len()
                    + (g.red3[0] + g.red3[1]) as usize
                    + g.melds
                        .iter()
                        .flatten()
                        .map(|m| m.size() as usize)
                        .sum::<usize>();
                assert_eq!(total, 108);
                n += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed}");
        }
    }
}

#[cfg(test)]
mod a3_audit {
    use super::*;

    #[test]
    fn ai_ignores_the_opponent_hand_and_stock_at_every_level() {
        crate::p3_comun::ai_blind_check(
            "canasta",
            Canasta::with_seed,
            |g, me, s| {
                let h = 1 - me.idx();
                crate::p3_comun::mix_unseen(&mut g.hands[h], &mut g.stock, 0, s);
                sort_hand(&mut g.hands[h]);
            },
            4,
        );
    }
}
