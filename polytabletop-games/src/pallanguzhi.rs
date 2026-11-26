//! Pallanguzhi (Tamil Nadu), mancala cu 2 × 7 cupe și câte un buzunar de
//! capturi pe jucător, 5 semințe în fiecare cupă (70 în total).
//!
//! Reguli folosite (din descrierile publice, cu simplificările de mai jos):
//! * Alegi o cupă proprie nenulă, ridici toate semințele și le semeni câte una,
//!   în sens trigonometric (spre dreapta pe partea ta, apoi pe partea adversă).
//! * După ultima sămânță te uiți la cupa următoare: dacă are semințe, le
//!   ridici și continui semănatul; dacă e goală, capturezi conținutul cupei de
//!   după ea (dacă are) și tura se termină.
//! * „Pasu”: orice sămânță pusă care face o cupă să aibă exact 4 semințe o
//!   capturează imediat pe cele 4 (semănatul continuă).
//! * Jocul se termină când jucătorul la mutare nu mai are semințe pe partea lui:
//!   fiecare păstrează semințele de pe partea lui. Câștigă cine are peste 35.
//!
//! Simplificări (documentate): o singură rundă (fără refacerea cupelor din
//! runda a doua); capturarea de după cupa goală încheie tura; lanțul de semănat
//! e limitat la 2000 de semințe; după 600 de mutări se numără pe loc.

//! Verificare surse (nov. 2026: en.wikipedia.org/wiki/Pallanguzhi, bharatiyakhel.in,
//! teacherplus.org/?p=27679): direcția antiorară, continuarea semănatului din cupa
//! ocupată și captura la cupă goală corespund. DUBIU: numărul de semințe diferă după
//! sursă (6 cel mai des; 5, 7 sau 12 în variante; Wikipedia: 12 și pasu la 6), iar pasu
//! la 4 e atestat doar în unele descrieri. Varianta 5 semințe/pasu la 4 e păstrată
//! ca simplificare documentată; rundele multiple cu „gropi de gunoi” nu sunt implementate.

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const CAP: u32 = 2000;
const MAX_PLIES: u32 = 600;

#[derive(Clone)]
pub struct Pallanguzhi {
    pits: [u8; 14], // 0..=6 One (stânga→dreapta), 7..=13 Two (dreapta→stânga)
    store: [u8; 2],
    side: Player,
    plies: u32,
    over: bool,
}

impl Default for Pallanguzhi {
    fn default() -> Self {
        Self::new()
    }
}

impl Pallanguzhi {
    pub fn new() -> Self {
        Pallanguzhi {
            pits: [5; 14],
            store: [0, 0],
            side: Player::One,
            plies: 0,
            over: false,
        }
    }

    fn range(p: Player) -> std::ops::Range<usize> {
        match p {
            Player::One => 0..7,
            Player::Two => 7..14,
        }
    }

    pub fn stores(&self) -> (u8, u8) {
        (self.store[0], self.store[1])
    }

    pub fn total_seeds(&self) -> u32 {
        self.pits.iter().map(|&x| x as u32).sum::<u32>()
            + self.store.iter().map(|&x| x as u32).sum::<u32>()
    }

    fn side_empty(&self, p: Player) -> bool {
        self.pits[Self::range(p)].iter().all(|&x| x == 0)
    }

    fn finish(&mut self) {
        for p in [Player::One, Player::Two] {
            let rest: u8 = self.pits[Self::range(p)].iter().sum();
            for i in Self::range(p) {
                self.pits[i] = 0;
            }
            self.store[p.idx()] += rest;
        }
        self.over = true;
    }
}

impl Game for Pallanguzhi {
    type Move = u8;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<u8> {
        if self.over {
            return Vec::new();
        }
        let start = Self::range(self.side).start;
        (0..7u8)
            .filter(|&k| self.pits[start + k as usize] > 0)
            .map(|k| k + 1)
            .collect()
    }

    fn apply(&mut self, m: &u8) {
        let me = self.side;
        let start = Self::range(me).start + (*m as usize - 1);
        let mut seeds = self.pits[start];
        self.pits[start] = 0;
        let mut pos = start;
        let mut sown = 0u32;
        loop {
            while seeds > 0 {
                pos = (pos + 1) % 14;
                self.pits[pos] += 1;
                seeds -= 1;
                sown += 1;
                if self.pits[pos] == 4 {
                    self.store[me.idx()] += 4;
                    self.pits[pos] = 0;
                }
            }
            let next = (pos + 1) % 14;
            if sown > CAP {
                break;
            }
            if self.pits[next] > 0 {
                seeds = self.pits[next];
                self.pits[next] = 0;
                pos = next;
                continue;
            }
            let beyond = (next + 1) % 14;
            self.store[me.idx()] += self.pits[beyond];
            self.pits[beyond] = 0;
            break;
        }
        self.plies += 1;
        self.side = me.other();
        if self.side_empty(self.side) || self.plies >= MAX_PLIES {
            self.finish();
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            return None;
        }
        let (a, b) = self.stores();
        Some(if a > b {
            Outcome::Win(Player::One)
        } else if b > a {
            Outcome::Win(Player::Two)
        } else {
            Outcome::Draw
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let sd = |q: Player| -> i32 { self.pits[Self::range(q)].iter().map(|&x| x as i32).sum() };
        10 * (self.store[p.idx()] as i32 - self.store[p.other().idx()] as i32)
            + (sd(p) - sd(p.other()))
    }

    fn render(&self) -> String {
        let p = &self.pits;
        let mut s = String::new();
        s.push_str("  Jucător 2 (O):   7   6   5   4   3   2   1\n");
        s.push_str(&format!(
            "       [{:>2}]  {:>2}  {:>2}  {:>2}  {:>2}  {:>2}  {:>2}  {:>2}\n",
            self.store[1], p[13], p[12], p[11], p[10], p[9], p[8], p[7]
        ));
        s.push_str(&format!(
            "              {:>2}  {:>2}  {:>2}  {:>2}  {:>2}  {:>2}  {:>2}   [{:>2}]\n",
            p[0], p[1], p[2], p[3], p[4], p[5], p[6], self.store[0]
        ));
        s.push_str("  Jucător 1 (X):   1   2   3   4   5   6   7\n");
        s.push_str("  [..] = capturile jucătorului (câștigă peste 35)\n");
        s
    }

    fn format_move(&self, m: &u8) -> String {
        m.to_string()
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(6)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    fn custom(pits: [u8; 14], side: Player) -> Pallanguzhi {
        Pallanguzhi {
            pits,
            store: [0, 0],
            side,
            plies: 0,
            over: false,
        }
    }

    #[test]
    fn relay_through_nonempty_next_cup() {
        let mut p = [0u8; 14];
        p[0] = 2; // semană în 1,2; cupa 3 (index 3) are 3 → se continuă
        p[3] = 3;
        p[9] = 5;
        let mut g = custom(p, Player::One);
        g.apply(&1);
        // după 1,2: cupa următoare (3) ridicată → semintele 4,5,6; apoi cupa 7 goală → capturează cupa 8
        assert_eq!(g.pits[3], 0);
        assert_eq!(g.pits[4], 1);
    }

    #[test]
    fn empty_next_cup_captures_the_one_beyond() {
        let mut p = [0u8; 14];
        p[0] = 1; // cade în cupa 1; cupa 2 goală; cupa 3 are 6
        p[3] = 6;
        p[12] = 1;
        let mut g = custom(p, Player::One);
        g.apply(&1);
        assert_eq!(g.stores().0, 6);
        assert_eq!(g.pits[3], 0);
        assert_eq!(g.to_move(), Player::Two);
    }

    #[test]
    fn pasu_captures_cup_at_four() {
        let mut p = [0u8; 14];
        p[0] = 1;
        p[1] = 3; // devine 4 → pasu
        p[12] = 1;
        let mut g = custom(p, Player::One);
        g.apply(&1);
        assert_eq!(g.stores().0, 4);
        assert_eq!(g.pits[1], 0);
    }

    #[test]
    fn seeds_conserved_and_games_finish() {
        for seed in 1..=30u64 {
            let mut g = Pallanguzhi::new();
            let mut rng = Rng::new(seed);
            while g.outcome().is_none() {
                let m = Game::legal_moves(&g);
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                assert_eq!(g.total_seeds(), 70);
            }
        }
    }

    #[test]
    fn remaining_seeds_stay_with_their_side() {
        let mut p = [0u8; 14];
        p[7] = 1;
        p[10] = 4;
        let mut g = custom(p, Player::Two);
        g.apply(&1);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
        assert_eq!(g.total_seeds(), 5);
        assert_eq!(g.stores().1, 5);
    }
}
