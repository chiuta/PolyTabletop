//! American Mah Jongg — variantă simplificată pentru doi jucători (fără cartea NMJL anuală și fără Charleston).
//!
//! 152 de piese: 3 culori (crak `k`, bambus `b`, puncte `d`) × 1–9 × 4, 4 vânturi (E S W N) × 4, 3 dragoni
//! (`Dr` roșu, `Dv` verde, `Da` alb) × 4, 8 flori și 8 jokeri `J`. Fiecare jucător primește 13 piese (cel care începe, 14).
//! * Pe rând: tragi o piesă (florile se pun deoparte și se trage alta), apoi arunci una (`arunc 5k`).
//! * Când adversarul aruncă o piesă poți strânge un **pung** (trei la fel, cu jokeri în loc de piese, dar cel puțin o piesă
//!   naturală; `pung`), expus pe masă, și apoi arunci; altfel spui `pas` și tragi.
//! * Câștigi (`mahjong`) cu 4 seturi + o pereche: un set e un pung (jokerii pot înlocui) sau o scară de trei piese consecutive
//!   din aceeași culoare (fără jokeri); sau cu șapte perechi naturale (doar dacă nu ai expus nimic). Poți închide și pe
//!   piesa aruncată de adversar. Jokerii nu se pot folosi în perechi.
//! * Dacă ultimele 14 piese din zid rămân neatinse, mâna e remiză.
//!
//! Omise: cartea de mâini, Charleston, schimbul jokerilor din expuneri, kong/quint, plata. Informație ascunsă: mâinile și zidul.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: usize = 36; // 27 culori + 4 vânturi + 3 dragoni + floare + joker
const FLOWER: u8 = 34;
const JOKER: u8 = 35;

pub fn text(t: u8) -> String {
    match t {
        0..=26 => format!("{}{}", t % 9 + 1, ['k', 'b', 'd'][(t / 9) as usize]),
        27..=30 => ["E", "S", "W", "N"][(t - 27) as usize].into(),
        31..=33 => ["Dr", "Dv", "Da"][(t - 31) as usize].into(),
        FLOWER => "F".into(),
        _ => "J".into(),
    }
}

/// Poate o mână (piese naturale `c`, `j` jokeri) să formeze `melds` seturi și o pereche?
fn can_form(c: &mut [u8; 34], j: u8, melds: u8, need_pair: bool) -> bool {
    let Some(i) = (0..34).find(|&i| c[i] > 0) else {
        return melds == 0 && !need_pair && j == 0;
    };
    // pereche
    if need_pair && c[i] >= 2 {
        c[i] -= 2;
        let ok = can_form(c, j, melds, false);
        c[i] += 2;
        if ok {
            return true;
        }
    }
    if melds > 0 {
        // pung cu k naturale și 3-k jokeri
        for k in (1..=c[i].min(3)).rev() {
            let need_j = 3 - k;
            if need_j <= j {
                c[i] -= k;
                let ok = can_form(c, j - need_j, melds - 1, need_pair);
                c[i] += k;
                if ok {
                    return true;
                }
            }
        }
        // scară
        if i < 27 && i % 9 <= 6 && c[i + 1] > 0 && c[i + 2] > 0 {
            c[i] -= 1;
            c[i + 1] -= 1;
            c[i + 2] -= 1;
            let ok = can_form(c, j, melds - 1, need_pair);
            c[i] += 1;
            c[i + 1] += 1;
            c[i + 2] += 1;
            if ok {
                return true;
            }
        }
    }
    false
}

/// Mână câștigătoare cu `exposed` seturi expuse?
pub fn is_win(h: &[u8; N], exposed: u8) -> bool {
    let tot: u32 = h.iter().map(|&x| x as u32).sum();
    if tot != 14 - 3 * exposed as u32 {
        return false;
    }
    let mut c = [0u8; 34];
    c.copy_from_slice(&h[..34]);
    let j = h[JOKER as usize];
    if exposed == 0 && j == 0 && c.iter().all(|&x| x % 2 == 0) {
        return true;
    }
    can_form(&mut c, j, 4 - exposed, true)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mv {
    Discard(u8),
    Pung,
    Pass,
    Mahjong,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Discard,
    React,
    Over,
}

#[derive(Clone)]
pub struct MahjongAm {
    viewer: Option<Player>,
    hands: [[u8; N]; 2],
    exposed: [Vec<u8>; 2],
    wall: Vec<u8>,
    discards: Vec<u8>,
    last: Option<u8>,
    phase: Phase,
    turn: usize,
    pending: bool,
    winner: Option<usize>,
    flowers: [u8; 2],
    noise: u64,
    log: String,
}

impl Default for MahjongAm {
    fn default() -> Self {
        Self::new()
    }
}

impl MahjongAm {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut wall: Vec<u8> = Vec::with_capacity(152);
        for t in 0..34u8 {
            for _ in 0..4 {
                wall.push(t);
            }
        }
        for _ in 0..8 {
            wall.push(FLOWER);
            wall.push(JOKER);
        }
        for i in (1..wall.len()).rev() {
            let j = rng.below(i + 1);
            wall.swap(i, j);
        }
        let mut g = MahjongAm {
            viewer: None,
            hands: [[0; N]; 2],
            exposed: [Vec::new(), Vec::new()],
            wall,
            discards: Vec::new(),
            last: None,
            phase: Phase::Discard,
            turn: 0,
            pending: false,
            winner: None,
            flowers: [0; 2],
            noise: rng.next_u64(),
            log: String::new(),
        };
        for p in 0..2 {
            let need = if p == 0 { 14 } else { 13 };
            let mut got = 0;
            while got < need {
                let t = g.wall.pop().unwrap();
                if t == FLOWER {
                    g.flowers[p] += 1;
                } else {
                    g.hands[p][t as usize] += 1;
                    got += 1;
                }
            }
        }
        g.log = "J1 începe cu 14 piese.".into();
        g
    }
    fn resolved(&self) -> MahjongAm {
        let mut g = self.clone();
        g.resolve();
        g
    }
    /// Trage efectiv piesa pentru jucătorul la rând (florile se înlocuiesc).
    fn resolve(&mut self) {
        if !self.pending {
            return;
        }
        self.pending = false;
        let p = self.turn;
        loop {
            let t = self.wall.pop().unwrap();
            if t == FLOWER {
                self.flowers[p] += 1;
                if self.wall.is_empty() {
                    break;
                }
            } else {
                self.hands[p][t as usize] += 1;
                break;
            }
        }
    }
    fn tiles(&self, p: usize) -> u32 {
        self.hands[p].iter().map(|&x| x as u32).sum()
    }
    fn can_pung(&self, p: usize, t: u8) -> bool {
        let nat = self.hands[p][t as usize];
        let jok = self.hands[p][JOKER as usize];
        t != JOKER && nat >= 1 && nat.min(2) + jok >= 2
    }
    fn heuristic(&self, p: usize) -> i32 {
        let h = &self.hands[p];
        let mut v = 0;
        for i in 0..34 {
            let c = h[i] as i32;
            v += c.min(3) * c.min(3) * 3;
            if i < 27 && i % 9 <= 7 && h[i] > 0 && h[i + 1] > 0 {
                v += 4;
            }
            if i < 27 && i % 9 <= 6 && h[i] > 0 && h[i + 2] > 0 {
                v += 2;
            }
        }
        v += 9 * h[JOKER as usize] as i32;
        v + 25 * self.exposed[p].len() as i32
    }
}

impl Game for MahjongAm {
    type Move = Mv;
    fn to_move(&self) -> Player {
        crate::q4_secret::pl(self.turn)
    }
    fn legal_moves(&self) -> Vec<Mv> {
        if self.phase == Phase::Over {
            return Vec::new();
        }
        let g = self.resolved();
        let p = g.turn;
        let mut v = Vec::new();
        match g.phase {
            Phase::Discard => {
                if is_win(&g.hands[p], g.exposed[p].len() as u8) {
                    v.push(Mv::Mahjong);
                }
                let only_j = (0..34).all(|i| g.hands[p][i] == 0);
                for t in 0..N as u8 {
                    if g.hands[p][t as usize] > 0 && t != FLOWER && (t != JOKER || only_j) {
                        v.push(Mv::Discard(t));
                    }
                }
            }
            Phase::React => {
                let t = g.last.unwrap();
                let mut h = g.hands[p];
                h[t as usize] += 1;
                if is_win(&h, g.exposed[p].len() as u8) {
                    v.push(Mv::Mahjong);
                }
                if g.can_pung(p, t) && g.exposed[p].len() < 4 {
                    v.push(Mv::Pung);
                }
                v.push(Mv::Pass);
            }
            Phase::Over => {}
        }
        v
    }
    fn apply(&mut self, m: &Mv) {
        self.resolve();
        let p = self.turn;
        match (*m, self.phase) {
            (Mv::Discard(t), Phase::Discard) => {
                self.hands[p][t as usize] -= 1;
                self.discards.push(t);
                self.last = Some(t);
                self.turn = 1 - p;
                self.phase = Phase::React;
                self.log = format!("J{} aruncă {}.", p + 1, text(t));
            }
            (Mv::Mahjong, Phase::Discard) => {
                self.winner = Some(p);
                self.phase = Phase::Over;
                self.log = format!("J{} spune Mahjong!", p + 1);
            }
            (Mv::Mahjong, Phase::React) => {
                let t = self.last.unwrap();
                self.discards.pop();
                self.hands[p][t as usize] += 1;
                self.winner = Some(p);
                self.phase = Phase::Over;
                self.log = format!("J{} câștigă pe {} aruncat: Mahjong!", p + 1, text(t));
            }
            (Mv::Pung, Phase::React) => {
                let t = self.last.unwrap();
                self.discards.pop();
                let nat = self.hands[p][t as usize].min(2);
                self.hands[p][t as usize] -= nat;
                self.hands[p][JOKER as usize] -= 2 - nat;
                self.exposed[p].push(t);
                self.last = None;
                self.phase = Phase::Discard;
                self.log = format!("J{} strânge pung de {}.", p + 1, text(t));
            }
            (Mv::Pass, Phase::React) => {
                self.last = None;
                if self.wall.len() <= 14 {
                    self.phase = Phase::Over;
                    self.log = "Zidul s-a terminat: remiză.".into();
                } else {
                    self.pending = true;
                    self.phase = Phase::Discard;
                }
            }
            _ => {}
        }
    }
    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        Some(match self.winner {
            Some(w) => Outcome::Win(crate::q4_secret::pl(w)),
            None => Outcome::Draw,
        })
    }
    fn evaluate(&self, pl: Player) -> i32 {
        let me = pl.idx();
        // Doar mâna proprie (starea nerezolvată nu arată piesa care urmează).
        let mut v = self.heuristic(me) - self.heuristic(me) / 4;
        if let (Phase::React, Some(t)) = (self.phase, self.last) {
            if self.turn == me {
                let mut h = self.hands[me];
                h[t as usize] += 1;
                if is_win(&h, self.exposed[me].len() as u8) {
                    v += 5000;
                }
            }
        }
        v + crate::q4_secret::noise(
            self.noise,
            self.tiles(me) as u64 + 31 * self.discards.len() as u64,
            5,
        )
    }
    fn render(&self) -> String {
        let g = self.resolved();
        let view = crate::q4_secret::viewer_of(self.viewer, self.to_move());
        let mut s = format!(
            "Zid: {} piese. Flori: J1 {}, J2 {}.\n",
            g.wall.len(),
            g.flowers[0],
            g.flowers[1]
        );
        let fmt_h = |h: &[u8; N]| -> String {
            let mut o = Vec::new();
            for t in 0..N as u8 {
                for _ in 0..h[t as usize] {
                    o.push(text(t));
                }
            }
            o.join(" ")
        };
        if g.phase != Phase::Over {
            s.push_str(&format!(
                "Mâna ta (J{}): {}\n",
                view + 1,
                fmt_h(&g.hands[view])
            ));
        } else {
            for p in 0..2 {
                s.push_str(&format!("J{}: {}\n", p + 1, fmt_h(&g.hands[p])));
            }
        }
        for p in 0..2 {
            if !g.exposed[p].is_empty() {
                s.push_str(&format!(
                    "Expus J{}: {}\n",
                    p + 1,
                    g.exposed[p]
                        .iter()
                        .map(|&t| format!("{}×3", text(t)))
                        .collect::<Vec<_>>()
                        .join(" ")
                ));
            }
        }
        s.push_str(&format!(
            "Adversarul are {} piese în mână.\n",
            g.tiles(1 - view)
        ));
        let tail: Vec<String> = g
            .discards
            .iter()
            .rev()
            .take(12)
            .rev()
            .map(|&t| text(t))
            .collect();
        s.push_str(&format!(
            "Aruncate (ultimele {}): {}\n",
            tail.len(),
            tail.join(" ")
        ));
        s.push_str(g.log.trim());
        if g.phase != Phase::Over {
            s.push_str(&format!("\nLa mutare: Jucătorul {}.", g.turn + 1));
        }
        s
    }
    fn format_move(&self, m: &Mv) -> String {
        match *m {
            Mv::Discard(t) => format!("arunc {}", text(t)),
            Mv::Pung => "pung".into(),
            Mv::Pass => "pas".into(),
            Mv::Mahjong => "mahjong".into(),
        }
    }
    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::q4_util::*;
    use polytabletop_core::{GameObj, Player};

    fn hand(ts: &[u8]) -> [u8; N] {
        let mut h = [0u8; N];
        for &t in ts {
            h[t as usize] += 1;
        }
        h
    }

    #[test]
    fn win_detection() {
        // 111k 222k 333k 456k + 55b (b=9..17) → 14 piese
        let h = hand(&[0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 4, 5, 13, 13]);
        assert!(is_win(&h, 0));
        // un joker în loc de a doua piesă a pungului
        let h = hand(&[0, 0, JOKER, 1, 1, 1, 2, 2, 2, 3, 4, 5, 13, 13]);
        assert!(is_win(&h, 0));
        // joker în pereche nu e permis
        let h = hand(&[0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 4, 5, 13, JOKER]);
        assert!(!is_win(&h, 0));
        // șapte perechi
        let h = hand(&[0, 0, 5, 5, 9, 9, 14, 14, 20, 20, 27, 27, 32, 32]);
        assert!(is_win(&h, 0));
        assert!(!is_win(&h, 1));
        // cu un set expus: 11 piese
        let h = hand(&[0, 0, 0, 3, 4, 5, 9, 9, 9, 13, 13]);
        assert!(is_win(&h, 1));
        assert!(!is_win(
            &hand(&[0, 1, 5, 7, 9, 11, 14, 20, 22, 28, 30, 32, 33, 6]),
            0
        ));
    }

    #[test]
    fn pung_call_and_discard_flow() {
        let mut g = MahjongAm::with_seed(5);
        g.hands = [
            hand(&[0, 0, 7, 9, 12, 14, 16, 20, 22, 28, 30, 32, 33, 6]),
            hand(&[1, 1, 3, 5, 7, 7, 11, 13, 15, 17, 19, 21, 23]),
        ];
        g.apply(&Mv::Discard(7)); // J1 aruncă 8k
        let m = GameObj::legal_moves(&g);
        assert!(
            m.contains(&"pung".to_string()) && m.contains(&"pas".to_string()),
            "{m:?}"
        );
        g.apply(&Mv::Pung);
        assert_eq!(g.exposed[1], vec![7]);
        assert_eq!(g.tiles(1), 11);
        assert_eq!(g.turn, 1);
    }

    #[test]
    fn viewer_hides_hand() {
        let g = MahjongAm::with_seed(3);
        let mut a = g.clone_box();
        a.set_viewer(Some(Player::One));
        let mut b = g.clone_box();
        b.set_viewer(Some(Player::Two));
        assert_ne!(a.render(), b.render());
    }

    #[test]
    fn random_and_ai_games_finish() {
        random_finishes(|s| Box::new(MahjongAm::with_seed(s)), 4, 1000);
        ai_vs_random(|s| Box::new(MahjongAm::with_seed(s)), 2, Player::One, 1000);
        ai_vs_random(|s| Box::new(MahjongAm::with_seed(s)), 3, Player::Two, 1000);
    }
}
