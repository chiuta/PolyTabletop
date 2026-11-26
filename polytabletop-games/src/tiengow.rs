#![allow(clippy::doc_lazy_continuation)]
//! Tien Gow (天九, „Cer și nouă”), joc cantonez de levate cu 32 de piese de domino chinezesc, adaptat pentru doi jucători
//! (jocul original are patru), după regulile publice (pagat.com / Wikipedia).
//!
//! * Setul de 32 de piese: 11 piese „civile” în două exemplare (6-6, 1-1, 4-4, 3-1, 5-5, 3-3, 2-2, 6-5, 6-4, 6-1, 5-1, în
//!   ordine descrescătoare) și 10 piese „militare” într-un singur exemplar (6-3 și 5-4 = nouă, 6-2 și 5-3 = opt, 5-2 și 4-3 = șapte,
//!   4-2 = șase, 4-1 și 3-2 = cinci, 2-1 = trei; în același grup piesele sunt egale).
//! * **Adaptare pentru doi**: fiecare primește 8 piese; celelalte 16 rămân ascunse și nu se joacă. Cine câștigă ultima
//!   levată câștigă mâna și conduce la următoarea; cel care a câștigat levata anterioară conduce în continuare.
//! * Conducătorul joacă o piesă (`64`) sau o pereche (`66+66`: două piese civile identice, două militare din același grup, o
//!   piesă civilă cu militara ei: 6-6 cu nouă, 1-1 cu opt, 4-4 cu șapte, 3-1 cu cinci; sau Gee Joon `42+21`, imbatabilă dacă e condusă).
//! * Celălalt trebuie să joace tot atâtea piese: bate cu o piesă mai mare **de aceeași culoare** (civil / militar), sau pereche mai
//!   mare de același tip (egalitatea o păstrează cel de la început), sau **aruncă cu fața în jos** (`d64`, `d64+11`). Piesele aruncate sunt ascunse.
//! * Moartea timpurie: dacă la ultima levată (câte o piesă) un jucător nu a câștigat nicio levată, o pierde.
//! * Punctaj (simplificat): câștigătorul mâinii ia 1 punct, dublat dacă adversarul n-a câștigat nicio levată („joc complet”), dublat dacă
//!   ultima levată a fost câștigată cu Gee Joon. Se joacă la 10 puncte (cel mult 20 de mâini).
//! * Nu sunt implementate: triplele și cvartetele, banca și multiplicatorii de bancher.
//! Informație ascunsă: AI-ul vede doar propriile piese și piesele jucate cu fața în sus.

use crate::cards::{handover_banner, Viewer};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const TARGET: i32 = 10;
const MAX_HANDS: u32 = 20;

/// (a, b, categorie 0 civil / 1 militar, rang)
const CIVIL: [(u8, u8, u8); 11] = [
    (6, 6, 11),
    (1, 1, 10),
    (4, 4, 9),
    (3, 1, 8),
    (5, 5, 7),
    (3, 3, 6),
    (2, 2, 5),
    (6, 5, 4),
    (6, 4, 3),
    (6, 1, 2),
    (5, 1, 1),
];
const MIL: [(u8, u8, u8); 10] = [
    (6, 3, 6),
    (5, 4, 6),
    (6, 2, 5),
    (5, 3, 5),
    (5, 2, 4),
    (4, 3, 4),
    (4, 2, 3),
    (4, 1, 2),
    (3, 2, 2),
    (2, 1, 1),
];

fn info(id: u8) -> (u8, u8, u8, u8) {
    if id < 22 {
        let (a, b, r) = CIVIL[(id / 2) as usize];
        (a, b, 0, r)
    } else {
        let (a, b, r) = MIL[(id - 22) as usize];
        (a, b, 1, r)
    }
}
fn text(id: u8) -> String {
    let (a, b, _, _) = info(id);
    format!("{a}{b}")
}
fn strength(id: u8) -> f64 {
    let (_, _, c, r) = info(id);
    if c == 0 {
        4.0 + r as f64
    } else {
        1.5 * r as f64
    }
}
fn is_gj(a: u8, b: u8) -> bool {
    let (x, y) = (text(a), text(b));
    (x == "42" && y == "21") || (x == "21" && y == "42")
}
/// Tip de pereche valid: (categorie 0 civil, 1 militar, 2 mixt, 3 Gee Joon; rang)
fn pair_kind(a: u8, b: u8) -> Option<(u8, u8)> {
    if is_gj(a, b) {
        return Some((3, 0));
    }
    let (_, _, ca, ra) = info(a);
    let (_, _, cb, rb) = info(b);
    match (ca, cb) {
        (0, 0) => (ra == rb).then_some((0, ra)),
        (1, 1) => (ra == rb && ra != 1 && ra != 3).then_some((1, ra)),
        _ => {
            let (civ, mil) = if ca == 0 { (a, b) } else { (b, a) };
            let (_, _, _, rc) = info(civ);
            let (_, _, _, rm) = info(mil);
            let ok = matches!((rc, rm), (11, 6) | (10, 5) | (9, 4) | (8, 2));
            ok.then_some((2, rc))
        }
    }
}
fn beats(lead: &[u8], reply: &[u8]) -> bool {
    if lead.len() == 1 {
        let (_, _, c1, r1) = info(lead[0]);
        let (_, _, c2, r2) = info(reply[0]);
        c1 == c2 && r2 > r1
    } else {
        match (pair_kind(lead[0], lead[1]), pair_kind(reply[0], reply[1])) {
            (Some((k1, r1)), Some((k2, r2))) => k1 != 3 && k1 == k2 && r2 > r1,
            _ => false,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Play(Vec<u8>),
    Discard(Vec<u8>),
}

#[derive(Clone)]
pub struct TienGow {
    hands: [Vec<u8>; 2],
    leader: Player,
    side: Player,
    lead: Vec<u8>,
    won: [u8; 2],
    seen: Vec<u8>,
    score: [i32; 2],
    hand_no: u32,
    over: bool,
    rng: Rng,
    viewer: Viewer,
    log: String,
}

impl Default for TienGow {
    fn default() -> Self {
        Self::new()
    }
}

impl TienGow {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = TienGow {
            hands: [Vec::new(), Vec::new()],
            leader: Player::One,
            side: Player::One,
            lead: Vec::new(),
            won: [0; 2],
            seen: Vec::new(),
            score: [0; 2],
            hand_no: 0,
            over: false,
            rng: Rng::new(seed),
            viewer: Viewer::Fixed(Player::One),
            log: String::new(),
        };
        g.deal();
        g.leader = Player::One;
        g.side = Player::One;
        g
    }

    fn deal(&mut self) {
        let mut d: Vec<u8> = (0..32).collect();
        crate::cards::shuffle(&mut d, &mut self.rng);
        self.hands = [d[0..8].to_vec(), d[8..16].to_vec()];
        for h in &mut self.hands {
            h.sort_by_key(|&t| std::cmp::Reverse((strength(t) * 10.0) as i32));
        }
        self.won = [0; 2];
        self.seen.clear();
        self.lead.clear();
        self.side = self.leader;
    }

    fn distinct(h: &[u8]) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        for &t in h {
            if !out.iter().any(|&u| text(u) == text(t)) {
                out.push(t);
            }
        }
        out
    }

    fn singles(h: &[u8]) -> Vec<u8> {
        Self::distinct(h)
    }

    fn pairs_all(h: &[u8]) -> Vec<Vec<u8>> {
        let mut out: Vec<Vec<u8>> = Vec::new();
        let mut seen: Vec<String> = Vec::new();
        for i in 0..h.len() {
            for j in i + 1..h.len() {
                let mut k = [text(h[i]), text(h[j])];
                k.sort();
                let key = k.join("+");
                if !seen.contains(&key) {
                    seen.push(key);
                    out.push(vec![h[i], h[j]]);
                }
            }
        }
        out
    }

    fn end_hand(&mut self, winner: Player, gj: bool) {
        let loser = winner.other();
        let mut pts = 1;
        let complete = self.won[loser.idx()] == 0;
        if complete {
            pts *= 2;
        }
        if gj {
            pts *= 2;
        }
        self.score[winner.idx()] += pts;
        self.log = format!(
            "J{} câștigă mâna (+{pts}{}{}).",
            winner.idx() + 1,
            if complete { ", joc complet" } else { "" },
            if gj { ", Gee Joon" } else { "" }
        );
        self.hand_no += 1;
        self.leader = winner;
        if self.score.iter().any(|&s| s >= TARGET) || self.hand_no >= MAX_HANDS {
            self.over = true;
        } else {
            self.deal();
        }
    }

    fn remove(&mut self, p: Player, tiles: &[u8]) {
        for &t in tiles {
            let k = self.hands[p.idx()]
                .iter()
                .position(|&u| text(u) == text(t))
                .unwrap();
            self.hands[p.idx()].remove(k);
        }
    }

    fn unseen_beaters(&self, p: Player, lead: &[u8]) -> (f64, f64) {
        let mut total: f64 = 0.0;
        let mut beat = 0.0;
        for id in 0..32u8 {
            if self.hands[p.idx()].contains(&id) || self.seen.contains(&id) {
                continue;
            }
            total += 1.0;
            if lead.len() == 1 && beats(lead, &[id]) {
                beat += 1.0;
            }
            if lead.len() == 2 {
                let (_, _, c, r) = info(id);
                let (k, rr) = pair_kind(lead[0], lead[1]).unwrap_or((9, 0));
                if k != 3 && (k == c && r > rr || k == 2 && r > rr) {
                    beat += 0.5;
                }
            }
        }
        (beat, total.max(1.0))
    }
}

impl Game for TienGow {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over {
            return Vec::new();
        }
        let h = &self.hands[self.side.idx()];
        let mut v = Vec::new();
        if self.lead.is_empty() {
            for t in Self::singles(h) {
                v.push(Move::Play(vec![t]));
            }
            for pr in Self::pairs_all(h) {
                if pair_kind(pr[0], pr[1]).is_some() {
                    v.push(Move::Play(pr));
                }
            }
        } else if self.lead.len() == 1 {
            for t in Self::singles(h) {
                if beats(&self.lead, &[t]) {
                    v.push(Move::Play(vec![t]));
                }
                v.push(Move::Discard(vec![t]));
            }
        } else {
            for pr in Self::pairs_all(h) {
                if beats(&self.lead, &pr) {
                    v.push(Move::Play(pr.clone()));
                }
                v.push(Move::Discard(pr));
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        let p = self.side;
        match m {
            Move::Play(t) | Move::Discard(t) if self.lead.is_empty() => {
                self.remove(p, t);
                self.lead = t.clone();
                self.seen.extend(t.iter().copied());
                self.side = p.other();
            }
            Move::Play(t) => {
                let t = t.clone();
                self.remove(p, &t);
                self.seen.extend(t.iter().copied());
                let lead = std::mem::take(&mut self.lead);
                let w = if beats(&lead, &t) { p } else { self.leader };
                self.finish_trick(w, if w == p { &t } else { &lead });
            }
            Move::Discard(t) => {
                let t = t.clone();
                self.remove(p, &t);
                let lead = std::mem::take(&mut self.lead);
                self.finish_trick(self.leader, &lead);
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            return None;
        }
        Some(match self.score[0].cmp(&self.score[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let q = p.other();
        let base = (self.score[p.idx()] - self.score[q.idx()]) * 400;
        if self.over || self.hands[p.idx()].len() == 8 && self.lead.is_empty() && self.won == [0; 2]
        {
            return base;
        }
        let h = &self.hands[p.idx()];
        let mx = h.iter().map(|&t| strength(t)).fold(0.0, f64::max);
        let sum: f64 = h.iter().map(|&t| strength(t)).sum();
        let mut v = base as f64 + 40.0 * (self.won[p.idx()] as f64 - self.won[q.idx()] as f64);
        v += 12.0 * mx + 2.0 * sum;
        if self.won[p.idx()] == 0 {
            v -= 10.0 * (8 - h.len()) as f64;
        }
        if !self.lead.is_empty() && self.leader == p {
            let (b, t) = self.unseen_beaters(p, &self.lead);
            let nobeat = (1.0 - b / t).powi(self.hands[q.idx()].len().min(8) as i32 / 2 + 1);
            v += 60.0 * nobeat * self.lead.len() as f64 - 15.0;
        }
        v as i32
    }

    fn render(&self) -> String {
        let me = self.viewer.who(self.to_move());
        let mut s = format!(
            "Tien Gow — mâna {}/{MAX_HANDS}. Puncte: J1 {}, J2 {} (la {TARGET}). Levate: J1 {}, J2 {}.\n",
            (self.hand_no + 1).min(MAX_HANDS),
            self.score[0],
            self.score[1],
            self.won[0],
            self.won[1]
        );
        if let Viewer::HotSeat = self.viewer {
            s.push_str(&handover_banner(self.side));
        }
        for p in 0..2 {
            if p == me.idx() {
                let h: Vec<String> = self.hands[p].iter().map(|&t| text(t)).collect();
                s.push_str(&format!("J{} (piesele tale): {}\n", p + 1, h.join(" ")));
            } else {
                s.push_str(&format!("J{}: {} piese\n", p + 1, self.hands[p].len()));
            }
        }
        if !self.lead.is_empty() {
            let l: Vec<String> = self.lead.iter().map(|&t| text(t)).collect();
            s.push_str(&format!(
                "J{} a condus: {}\n",
                self.leader.idx() + 1,
                l.join("+")
            ));
        }
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        let join = |t: &[u8]| {
            let mut v: Vec<String> = t.iter().map(|&x| text(x)).collect();
            v.sort();
            v.join("+")
        };
        match m {
            Move::Play(t) => join(t),
            Move::Discard(t) => format!("d{}", join(t)),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = match who {
            Some(p) => Viewer::Fixed(p),
            None => Viewer::HotSeat,
        };
    }
}

impl TienGow {
    fn finish_trick(&mut self, w: Player, shown: &[u8]) {
        self.won[w.idx()] += 1;
        self.leader = w;
        self.side = w;
        let gj = shown.len() == 2 && is_gj(shown[0], shown[1]);
        if self.hands[0].is_empty() {
            self.end_hand(w, gj);
            return;
        }
        if self.hands[0].len() == 1 {
            // moartea timpurie
            let l = w.other();
            if self.won[l.idx()] == 0 {
                self.end_hand(w, false);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(s: &str, copy: u8) -> u8 {
        for i in 0..32u8 {
            if text(i) == s && (i >= 22 || i % 2 == copy) {
                return i;
            }
        }
        panic!("{s}");
    }

    #[test]
    fn set_is_complete() {
        let mut n = 0;
        for i in 0..32u8 {
            let _ = text(i);
            n += 1;
        }
        assert_eq!(n, 32);
        assert_eq!(pair_kind(id("42", 0), id("21", 0)), Some((3, 0)));
        assert_eq!(pair_kind(id("66", 0), id("66", 1)), Some((0, 11)));
        assert_eq!(pair_kind(id("66", 0), id("63", 0)), Some((2, 11)));
        assert_eq!(pair_kind(id("66", 0), id("62", 0)), None);
        assert!(beats(&[id("55", 0)], &[id("44", 0)]));
        assert!(!beats(&[id("55", 0)], &[id("63", 0)]));
        assert!(!beats(&[id("63", 0)], &[id("54", 0)]));
    }

    #[test]
    fn follow_rules() {
        let mut g = TienGow::with_seed(1);
        g.hands[0] = vec![id("55", 0), id("63", 0)];
        g.hands[1] = vec![id("44", 0), id("21", 0)];
        g.apply(&Move::Play(vec![id("55", 0)]));
        let v = g.legal_moves();
        assert!(v.iter().any(|m| matches!(m, Move::Play(_))));
        assert!(!v
            .iter()
            .any(|m| matches!(m, Move::Play(t) if text(t[0]) == "21")));
        assert!(v.iter().any(|m| matches!(m, Move::Discard(_))));
    }

    #[test]
    fn early_death_and_scoring() {
        let mut g = TienGow::with_seed(2);
        g.hands[0] = vec![id("66", 0), id("11", 0)];
        g.hands[1] = vec![id("33", 0), id("22", 0)];
        g.won = [1, 0];
        g.leader = Player::One;
        g.side = Player::One;
        g.apply(&Move::Play(vec![id("66", 0)]));
        g.apply(&Move::Discard(vec![id("33", 0)]));
        // J2 are încă o piesă și n-a câștigat nicio levată: J1 câștigă (joc complet)
        assert_eq!(g.score[0], 2);
    }

    #[test]
    fn random_games_end_and_hidden() {
        for seed in 1..=15u64 {
            let mut g = TienGow::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
                assert!(n < 1000);
            }
        }
        let mut g = TienGow::with_seed(5);
        g.set_viewer(Some(Player::One));
        let a = g.render();
        g.set_viewer(Some(Player::Two));
        assert_ne!(a, g.render());
        let m = polytabletop_core::GameObj::ai_move(&g, 2).unwrap();
        assert!(g.legal_moves().iter().any(|x| g.format_move(x) == m));
    }

    #[test]
    fn unique_move_texts() {
        for seed in 1..=10u64 {
            let mut g = TienGow::with_seed(seed);
            for _ in 0..6 {
                let v = g.legal_moves();
                if v.is_empty() {
                    break;
                }
                let mut t: Vec<String> = v.iter().map(|m| g.format_move(m)).collect();
                t.sort();
                let n = t.len();
                t.dedup();
                assert_eq!(n, t.len());
                g.apply(&v[0]);
            }
        }
    }
}
