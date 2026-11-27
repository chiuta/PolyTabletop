//! Atu și Perechi — variantă originală de jocul pe ridicături cu atu și perechi (lista C3, 2 jucători).
//!
//! Pachet de 52 de cărți: 2 culori × 13 ranguri × 2 exemplare identice. Cărțile cu culoarea-atu și
//! cele cu rangul-nivel sunt atu (cel mai tare: rangul-nivel în culoarea atu, apoi rangul-nivel în
//! cealaltă culoare, apoi atu-urile pe ranguri). Fiecare ține 13 cărți (mână ascunsă); câștigătorul
//! ridicăturii trage primul din rezervă (mutare forțată `trag`). Puncte: 5=5, 10=10, K=10 (total 100);
//! cine are mai mult de 50 câștigă.
//! Mutări: `par i` (o carte, indexul în mâna sortată) sau `par i j` (o pereche = două cărți
//! identice, la deschidere). Următorul răspunde în aceeași formă și trebuie să dea culoarea
//! cerută (atu = o clasă proprie) cât poate. Singură: cea mai tare din clasa cerută sau un atu
//! peste o culoare obișnuită. Pereche: câștigă o pereche mai tare din aceeași clasă sau o pereche de atu.
//! Informație ascunsă: mâinile; randarea arată doar mâna privitorului (HOTSEAT_SAFE).
#![allow(clippy::needless_range_loop)]
use crate::brand_util::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{Outcome, Player};

#[derive(Clone)]
pub struct Sheng {
    ts: u8,
    lr: u8,
    hands: [Vec<u8>; 2],
    stock: Vec<u8>,
    lead: Option<Vec<u8>>,
    leader: usize,
    side: usize,
    draw_for: Option<usize>,
    pts: [i32; 2],
    viewer: Option<Player>,
}

pub fn new() -> Sheng {
    with_seed(seed_now())
}

pub fn with_seed(seed: u64) -> Sheng {
    let mut rng = Rng::new(seed);
    let mut deck: Vec<u8> = (0..26u8).chain(0..26u8).collect();
    shuf(&mut deck, &mut rng);
    let mut g = Sheng {
        ts: rng.below(2) as u8,
        lr: rng.below(13) as u8,
        hands: [Vec::new(), Vec::new()],
        stock: deck,
        lead: None,
        leader: 0,
        side: 0,
        draw_for: None,
        pts: [0; 2],
        viewer: None,
    };
    for h in 0..2 {
        for _ in 0..13 {
            let c = g.stock.pop().unwrap();
            g.hands[h].push(c);
        }
    }
    g.sort();
    g
}

fn suit(c: u8) -> u8 {
    c / 13
}
fn rank(c: u8) -> u8 {
    c % 13
}

impl Sheng {
    fn trump(&self, c: u8) -> bool {
        suit(c) == self.ts || rank(c) == self.lr
    }
    fn class(&self, c: u8) -> u8 {
        if self.trump(c) {
            2
        } else {
            suit(c)
        }
    }
    fn power(&self, c: u8) -> i32 {
        if suit(c) == self.ts && rank(c) == self.lr {
            200
        } else if rank(c) == self.lr {
            150
        } else if suit(c) == self.ts {
            100 + rank(c) as i32
        } else {
            rank(c) as i32
        }
    }
    fn points(c: u8) -> i32 {
        match rank(c) {
            3 => 5,
            8 | 11 => 10,
            _ => 0,
        }
    }
    fn sort(&mut self) {
        for h in 0..2 {
            let mut v = std::mem::take(&mut self.hands[h]);
            v.sort_by_key(|&c| (self.class(c), self.power(c), c));
            self.hands[h] = v;
        }
    }
    fn name(&self, c: u8) -> String {
        let r = [
            "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
        ][rank(c) as usize];
        format!(
            "{}{}{}",
            r,
            ["♠", "♥"][suit(c) as usize],
            if self.trump(c) { "*" } else { "" }
        )
    }
    fn is_pair(a: u8, b: u8) -> bool {
        a == b
    }
    /// Variante de răspuns (indici în mâna celui care răspunde).
    fn replies(&self, hand: &[u8], lead: &[u8]) -> Vec<Vec<usize>> {
        let cls = self.class(lead[0]);
        let k = lead.len();
        let have: Vec<usize> = (0..hand.len())
            .filter(|&i| self.class(hand[i]) == cls)
            .collect();
        let need = k.min(have.len());
        let mut out = Vec::new();
        if k == 1 {
            if need == 1 {
                for &i in &have {
                    out.push(vec![i]);
                }
            } else {
                for i in 0..hand.len() {
                    out.push(vec![i]);
                }
            }
        } else {
            for i in 0..hand.len() {
                for j in i + 1..hand.len() {
                    let cnt = [i, j]
                        .iter()
                        .filter(|&&x| self.class(hand[x]) == cls)
                        .count();
                    if cnt == need {
                        out.push(vec![i, j]);
                    }
                }
            }
            out.dedup_by_key(|v| (hand[v[0]], hand[v[1]]));
        }
        out
    }
    /// Adevărat dacă răspunsul bate deschiderea.
    fn beats(&self, lead: &[u8], resp: &[u8]) -> bool {
        let lc = self.class(lead[0]);
        if lead.len() == 1 {
            let rc = self.class(resp[0]);
            return if rc == lc {
                self.power(resp[0]) > self.power(lead[0])
            } else {
                rc == 2
            };
        }
        if !Self::is_pair(resp[0], resp[1]) {
            return false;
        }
        let rc = self.class(resp[0]);
        if rc == lc {
            self.power(resp[0]) > self.power(lead[0])
        } else {
            rc == 2
        }
    }
    fn pts_of(&self, v: &[u8]) -> i32 {
        v.iter().map(|&c| Self::points(c)).sum()
    }
    fn hand_pow(&self, h: usize) -> i32 {
        self.hands[h].iter().map(|&c| self.power(c)).sum()
    }
}

impl Mini for Sheng {
    fn side(&self) -> Player {
        pl(self.side)
    }
    fn depth(&self) -> u32 {
        0
    }
    fn moves(&self) -> Vec<String> {
        if self.draw_for.is_some() {
            return vec!["trag".into()];
        }
        let hand = &self.hands[self.side];
        let mut v = Vec::new();
        match &self.lead {
            None => {
                for i in 0..hand.len() {
                    if i == 0 || hand[i] != hand[i - 1] {
                        v.push(format!("par {}", i + 1));
                    }
                    if i + 1 < hand.len() && hand[i] == hand[i + 1] {
                        v.push(format!("par {} {}", i + 1, i + 2));
                    }
                }
            }
            Some(l) => {
                for r in self.replies(hand, l) {
                    if r.len() == 1 {
                        v.push(format!("par {}", r[0] + 1));
                    } else {
                        v.push(format!("par {} {}", r[0] + 1, r[1] + 1));
                    }
                }
            }
        }
        v.sort();
        v.dedup();
        v
    }
    fn step(&mut self, m: &str) {
        if let Some(w) = self.draw_for {
            for k in 0..2 {
                let h = (w + k) % 2;
                if let Some(c) = self.stock.pop() {
                    self.hands[h].push(c);
                }
            }
            self.sort();
            self.draw_for = None;
            self.side = w;
            self.leader = w;
            return;
        }
        let idx: Vec<usize> = m[4..]
            .split(' ')
            .map(|x| x.parse::<usize>().unwrap() - 1)
            .collect();
        let mut cards: Vec<u8> = idx.iter().map(|&i| self.hands[self.side][i]).collect();
        let mut rm = idx.clone();
        rm.sort_unstable_by(|a, b| b.cmp(a));
        for i in rm {
            self.hands[self.side].remove(i);
        }
        if self.lead.is_none() {
            cards.sort();
            self.lead = Some(cards);
            self.side = 1 - self.side;
            return;
        }
        let lead = self.lead.take().unwrap();
        let win = if self.beats(&lead, &cards) {
            1 - self.leader
        } else {
            self.leader
        };
        self.pts[win] += self.pts_of(&lead) + self.pts_of(&cards);
        if self.stock.is_empty() {
            self.side = win;
            self.leader = win;
        } else {
            self.draw_for = Some(win);
        }
    }
    fn result(&self) -> Option<Outcome> {
        if self.pts[0] > 50 {
            return Some(Outcome::Win(Player::One));
        }
        if self.pts[1] > 50 {
            return Some(Outcome::Win(Player::Two));
        }
        if self.hands[0].is_empty() && self.hands[1].is_empty() && self.stock.is_empty() {
            return Some(by_score(self.pts[0], self.pts[1]));
        }
        None
    }
    fn eval(&self, p: Player) -> i32 {
        let a = pi(p);
        100 * (self.pts[a] - self.pts[1 - a]) + self.hand_pow(a) / 6
    }
    fn show(&self) -> String {
        let viewer = self.viewer.map(pi).unwrap_or(pi(self.side()));
        let mut s = format!(
            "Atu și Perechi — atu: {} | rang-nivel: {} | rezervă {} | puncte J1 {} J2 {}\n",
            ["♠", "♥"][self.ts as usize],
            ["2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"][self.lr as usize],
            self.stock.len(),
            self.pts[0],
            self.pts[1]
        );
        if let Some(l) = &self.lead {
            let c: Vec<String> = l.iter().map(|&c| self.name(c)).collect();
            s.push_str(&format!("Deschidere: {}\n", c.join(" ")));
        }
        s.push_str(&format!(
            "J{} are {} cărți.\n",
            2 - viewer,
            self.hands[1 - viewer].len()
        ));
        let c: Vec<String> = self.hands[viewer]
            .iter()
            .enumerate()
            .map(|(i, &c)| format!("{}={}", i + 1, self.name(c)))
            .collect();
        s.push_str(&format!("Mâna ta (J{}): {}\n", viewer + 1, c.join(" ")));
        s.push_str(&format!(
            "La mutare: {} — par i | par i j\n",
            pname(self.side())
        ));
        s
    }
    fn view(&mut self, w: Option<Player>) {
        self.viewer = w;
    }
}

fn pl(i: usize) -> Player {
    if i == 0 {
        Player::One
    } else {
        Player::Two
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn plays() {
        check(with_seed, 400);
    }
    #[test]
    fn hidden_hand() {
        let g = with_seed(4);
        let mut h = g.clone();
        for c in h.hands[1].iter_mut() {
            *c = (*c + 1) % 26;
        }
        for seed in 1..6 {
            assert_eq!(ai_of(&g, seed), ai_of(&h, seed));
        }
        let mut v = g.clone();
        v.view(Some(Player::One));
        assert!(v.show().contains("Mâna ta (J1)"));
        assert!(!v.show().contains("Mâna ta (J2)"));
    }
}
