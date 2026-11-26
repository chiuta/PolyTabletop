//! Congkak (Malaezia), variantă de mancala cu runde: 2 × 7 case și câte un
//! depozit („rumah”) pe jucător, 7 semințe în fiecare casă (98 în total).
//!
//! Reguli publice folosite:
//! * Semănat în sens trigonometric: pe partea proprie de la stânga la dreapta,
//!   depozitul propriu, apoi partea adversă; depozitul adversarului se sare.
//! * Ultima sămânță în depozitul propriu: încă o mutare.
//! * Ultima sămânță într-o casă ocupată (de oricare parte): se ridică toate
//!   semințele ei și semănatul continuă.
//! * Ultima sămânță într-o casă goală proprie: ea și casa opusă (dacă are
//!   semințe) merg în depozit; dacă opusa e goală, doar sămânța ultimă merge în
//!   depozit. Tura se termină.
//! * Ultima sămânță într-o casă goală a adversarului: tura se termină.
//! * Runda se termină când jucătorul la mutare nu mai are semințe; fiecare
//!   păstrează semințele de pe partea lui. Apoi fiecare își reumple casele din
//!   depozit, de la stânga la dreapta, câte 7. Casele care nu se pot umple sunt
//!   „arse” (nu se mai folosesc, semănatul le sare); restul de semințe rămân în
//!   depozit. Cine nu poate umple nici o casă pierde meciul.
//!
//! Simplificări (documentate): jucătorii mută pe rând, nu simultan (Jucătorul 1
//! începe prima rundă, apoi începe cel cu mai puține semințe în depozit); casele
//! arse nu se pot reface; un semănat în lanț e limitat la 2000 de semințe puse;
//! după 40 de runde meciul se decide după depozite.

//! Verificare surse (nov. 2026: en.wikipedia.org/wiki/Congkak): setul de reguli de mai sus
//! (7×7 semințe, depozit propriu, ultima sămânță în casă goală proprie = captura casei
//! opuse + sămânța, case arse, reumplere stânga→dreapta) corespunde. Diferență
//! documentată: în original ambii jucători pornesc simultan și în runda a doua începe
//! perdantul; aici mută pe rând (vezi mai sus).

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const STORE: [usize; 2] = [7, 15];
const CAP: u32 = 2000;
const MAX_ROUNDS: u32 = 40;

#[derive(Clone)]
pub struct Congkak {
    pits: [u8; 16], // 0..=6 One, 7 One's store, 8..=14 Two, 15 Two's store
    burnt: [bool; 16],
    side: Player,
    round: u32,
    over: bool,
    log: String,
}

impl Default for Congkak {
    fn default() -> Self {
        Self::new()
    }
}

impl Congkak {
    pub fn new() -> Self {
        let mut pits = [7u8; 16];
        pits[7] = 0;
        pits[15] = 0;
        Congkak {
            pits,
            burnt: [false; 16],
            side: Player::One,
            round: 1,
            over: false,
            log: String::new(),
        }
    }

    fn range(p: Player) -> std::ops::Range<usize> {
        match p {
            Player::One => 0..7,
            Player::Two => 8..15,
        }
    }

    pub fn stores(&self) -> (u8, u8) {
        (self.pits[7], self.pits[15])
    }

    pub fn round(&self) -> u32 {
        self.round
    }

    pub fn total_seeds(&self) -> u32 {
        self.pits.iter().map(|&x| x as u32).sum()
    }

    pub fn burnt_houses(&self, p: Player) -> usize {
        Self::range(p).filter(|&i| self.burnt[i]).count()
    }

    fn side_empty(&self, p: Player) -> bool {
        self.pits[Self::range(p)].iter().all(|&x| x == 0)
    }

    /// Semănatul unei mutări; întoarce `true` dacă ea dă încă o mutare.
    fn sow(&mut self, m: u8) -> bool {
        let me = self.side;
        let own = STORE[me.idx()];
        let opp_store = STORE[me.other().idx()];
        let start = Self::range(me).start + (m as usize - 1);
        let mut seeds = self.pits[start];
        self.pits[start] = 0;
        let mut pos = start;
        let mut sown = 0u32;
        loop {
            while seeds > 0 {
                pos = (pos + 1) % 16;
                if pos == opp_store || self.burnt[pos] {
                    continue;
                }
                self.pits[pos] += 1;
                seeds -= 1;
                sown += 1;
            }
            if pos == own {
                return true;
            }
            if self.pits[pos] > 1 {
                if sown > CAP {
                    return false;
                }
                seeds = self.pits[pos];
                self.pits[pos] = 0;
                continue;
            }
            if Self::range(me).contains(&pos) {
                let opp = 14 - pos;
                self.pits[own] += self.pits[opp] + 1;
                self.pits[opp] = 0;
                self.pits[pos] = 0;
            }
            return false;
        }
    }

    /// Închide runda: fiecare păstrează partea lui; apoi reumplere sau final.
    fn end_round(&mut self) {
        for p in [Player::One, Player::Two] {
            let rest: u8 = self.pits[Self::range(p)].iter().sum();
            for i in Self::range(p) {
                self.pits[i] = 0;
            }
            self.pits[STORE[p.idx()]] += rest;
        }
        let s = [self.pits[7], self.pits[15]];
        let full = [(s[0] / 7).min(7) as usize, (s[1] / 7).min(7) as usize];
        if full[0] == 0 || full[1] == 0 || self.round >= MAX_ROUNDS {
            self.over = true;
            return;
        }
        for p in [Player::One, Player::Two] {
            let r = Self::range(p);
            for (k, i) in r.enumerate() {
                if k < full[p.idx()] {
                    self.pits[i] = 7;
                    self.pits[STORE[p.idx()]] -= 7;
                } else {
                    self.burnt[i] = true;
                }
            }
        }
        self.round += 1;
        self.side = if s[0] <= s[1] {
            Player::One
        } else {
            Player::Two
        };
        self.log = format!(
            "Runda {}: casele s-au reumplut (Jucătorul {} începe).",
            self.round,
            self.side.idx() + 1
        );
    }
}

impl Game for Congkak {
    type Move = u8; // casa 1..=7 din punctul de vedere al jucătorului la mutare

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
        self.log.clear();
        let extra = self.sow(*m);
        let next = if extra { me } else { me.other() };
        if self.side_empty(next) {
            self.side = next;
            self.end_round();
            return;
        }
        self.side = next;
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
        let st = |q: Player| self.pits[STORE[q.idx()]] as i32;
        let sd = |q: Player| -> i32 { self.pits[Self::range(q)].iter().map(|&x| x as i32).sum() };
        10 * (st(p) - st(p.other())) + (sd(p) - sd(p.other()))
    }

    fn render(&self) -> String {
        let cell = |i: usize| -> String {
            if self.burnt[i] {
                " xx".into()
            } else {
                format!(" {:>2}", self.pits[i])
            }
        };
        let mut s = format!("Runda {} din cel mult {}\n", self.round, MAX_ROUNDS);
        if !self.log.is_empty() {
            s.push_str(&self.log);
            s.push('\n');
        }
        s.push_str("  Jucător 2 (O):   7   6   5   4   3   2   1\n");
        s.push_str(&format!("       [{:>2}] ", self.pits[15]));
        for i in (8..15).rev() {
            s.push_str(&cell(i));
            s.push(' ');
        }
        s.push('\n');
        s.push_str("             ");
        for i in 0..7 {
            s.push_str(&cell(i));
            s.push(' ');
        }
        s.push_str(&format!("  [{:>2}]\n", self.pits[7]));
        s.push_str("  Jucător 1 (X):   1   2   3   4   5   6   7\n");
        s.push_str("  (xx = casă arsă)\n");
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

    fn custom(pits: [u8; 16], side: Player) -> Congkak {
        Congkak {
            pits,
            burnt: [false; 16],
            side,
            round: 1,
            over: false,
            log: String::new(),
        }
    }

    #[test]
    fn first_house_ends_in_store() {
        let mut g = Congkak::new();
        g.apply(&1); // 7 semințe: casele 2..7 și depozitul
        assert_eq!(g.to_move(), Player::One);
        assert_eq!(g.stores().0, 1);
        assert_eq!(g.pits[0], 0);
    }

    #[test]
    fn capture_includes_last_seed_and_opposite() {
        let mut p = [0u8; 16];
        p[0] = 1; // -> casa 2 (index 1), goală
        p[13] = 5; // opusa lui 1 este 13
        p[10] = 3; // ca să nu se termine runda
        let mut g = custom(p, Player::One);
        g.apply(&1);
        assert_eq!(g.pits[7], 6);
        assert_eq!(g.pits[13], 0);
        assert_eq!(g.to_move(), Player::Two);
    }

    #[test]
    fn empty_opposite_still_banks_last_seed() {
        let mut p = [0u8; 16];
        p[0] = 1;
        p[10] = 3;
        let mut g = custom(p, Player::One);
        g.apply(&1);
        assert_eq!(g.pits[7], 1);
        assert_eq!(g.pits[1], 0);
    }

    #[test]
    fn opponent_store_is_skipped() {
        let mut p = [0u8; 16];
        p[6] = 10; // depozit, 7 adverse, sare 15, apoi 0, 1
        p[3] = 1;
        p[9] = 1;
        let mut g = custom(p, Player::One);
        g.apply(&7);
        assert_eq!(g.pits[15], 0);
        let total: u32 = g.pits.iter().map(|&x| x as u32).sum();
        assert_eq!(total, 12);
    }

    #[test]
    fn round_refills_and_burns_houses() {
        // J1 are 20 în depozit (2 case), J2 78 (toate 7 case, 29 rămân).
        let mut p = [0u8; 16];
        p[7] = 20;
        p[15] = 77;
        p[0] = 1; // J1 mută ultima sămânță; apoi nu mai are semințe
        let mut g = custom(p, Player::One);
        g.apply(&1);
        assert_eq!(g.round(), 2);
        assert_eq!(g.burnt_houses(Player::One), 4);
        assert_eq!(g.burnt_houses(Player::Two), 0);
        assert_eq!(g.total_seeds(), 98);
        assert!(g.outcome().is_none());
    }

    #[test]
    fn match_ends_when_a_player_cannot_fill_a_house() {
        let mut p = [0u8; 16];
        p[7] = 3;
        p[15] = 90;
        p[0] = 1;
        p[10] = 4;
        let mut g = custom(p, Player::One);
        g.end_round();
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
    }

    #[test]
    fn seeds_are_conserved_and_games_finish() {
        for seed in 1..=20u64 {
            let mut g = Congkak::new();
            let mut rng = Rng::new(seed);
            let mut plies = 0;
            while g.outcome().is_none() {
                let m = Game::legal_moves(&g);
                assert!(!m.is_empty());
                let pick = m[rng.below(m.len())];
                g.apply(&pick);
                assert_eq!(g.total_seeds(), 98);
                plies += 1;
                assert!(plies < 20_000);
            }
        }
    }
}
