//! Table (Backgammon), written from the public rules only.
//!
//! * 24 puncte, 15 piese pe jucător, bară, bear-off. Fără cub de dublare și
//!   fără scor de gammon/backgammon: primul care scoate cele 15 piese câștigă.
//! * Zarurile se aruncă automat, în interiorul stării, cu un [`Rng`] propriu
//!   (seed) imediat după fiecare tură. Zarurile rămase se văd în `render()` și
//!   determină `legal_moves()`. Dubla se joacă de patru ori.
//! * O „mutare” este un singur pas cu un singur zar: `13/7`, `bar/22`,
//!   `3/off`. Dacă nu există nicio mutare legală, tura trece automat.
//! * Se respectă regula „joacă cât mai multe zaruri” și, când se poate juca
//!   doar unul, „joacă-l pe cel mai mare”.
//! * Numerotarea punctelor este mereu cea a jucătorului la mutare: 24 = punctul
//!   cel mai îndepărtat, 1 = ultimul punct din casă.
//!
//! AI: `AlphaBeta(1)` — alege pasul cu cea mai bună evaluare (pip count, piese
//! singure, puncte făcute). Nu „vede” zarurile viitoare: evaluarea ignoră
//! zarurile deja aruncate pentru tura următoare. Doar `Rng`-ul și zarurile
//! sunt nedeterministe; la același seed, jocul este complet reproductibil.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Mv {
    /// Punctul de plecare din perspectiva mutatorului (1..=24), 25 = bara.
    pub from: u8,
    pub die: u8,
}

#[derive(Clone, Copy)]
struct Board {
    /// Index absolut i = punctul i+1 al lui Unu; + = piese Unu, - = piese Doi.
    pts: [i8; 24],
    bar: [u8; 2],
    off: [u8; 2],
    side: Player,
    dice: [u8; 4],
    nd: u8,
}

fn sgn(p: Player) -> i8 {
    match p {
        Player::One => 1,
        Player::Two => -1,
    }
}

fn idx(p: Player, n: u8) -> usize {
    match p {
        Player::One => n as usize - 1,
        Player::Two => 24 - n as usize,
    }
}

impl Board {
    fn start() -> Board {
        let mut b = Board::empty();
        for (n, c) in [(24u8, 2i8), (13, 5), (8, 3), (6, 5)] {
            b.pts[idx(Player::One, n)] = c;
            b.pts[idx(Player::Two, n)] = -c;
        }
        b
    }

    fn empty() -> Board {
        Board {
            pts: [0; 24],
            bar: [0; 2],
            off: [0; 2],
            side: Player::One,
            dice: [0; 4],
            nd: 0,
        }
    }

    /// Piese proprii (pozitiv) sau ale adversarului (negativ) pe punctul `n` al lui `s`.
    fn own(&self, s: Player, n: u8) -> i8 {
        self.pts[idx(s, n)] * sgn(s)
    }

    fn raw_moves(&self) -> Vec<Mv> {
        let s = self.side;
        let i = s.idx();
        let mut out = Vec::new();
        let mut ds: Vec<u8> = self.dice[..self.nd as usize].to_vec();
        ds.sort_unstable();
        ds.dedup();
        let all_home = self.bar[i] == 0 && (7..=24).all(|n| self.own(s, n) <= 0);
        let highest = (1..=24u8).rev().find(|&n| self.own(s, n) > 0);
        for d in ds {
            if self.bar[i] > 0 {
                if self.own(s, 25 - d) >= -1 {
                    out.push(Mv { from: 25, die: d });
                }
                continue;
            }
            for n in 1..=24u8 {
                if self.own(s, n) <= 0 {
                    continue;
                }
                if n > d {
                    if self.own(s, n - d) >= -1 {
                        out.push(Mv { from: n, die: d });
                    }
                } else if all_home && (n == d || highest == Some(n)) {
                    out.push(Mv { from: n, die: d });
                }
            }
        }
        out
    }

    /// Joacă un pas (fără aruncare). Returnează `true` dacă a lovit o piesă.
    fn step(&mut self, m: Mv) -> bool {
        let s = self.side;
        let i = s.idx();
        let o = s.other().idx();
        let sg = sgn(s);
        let to: i16 = if m.from == 25 {
            self.bar[i] -= 1;
            25 - m.die as i16
        } else {
            self.pts[idx(s, m.from)] -= sg;
            m.from as i16 - m.die as i16
        };
        let mut hit = false;
        if to >= 1 {
            let t = idx(s, to as u8);
            if self.pts[t] == -sg {
                self.pts[t] = 0;
                self.bar[o] += 1;
                hit = true;
            }
            self.pts[t] += sg;
        } else {
            self.off[i] += 1;
        }
        // elimină un zar cu valoarea folosită
        let n = self.nd as usize;
        if let Some(k) = self.dice[..n].iter().position(|&d| d == m.die) {
            for j in k..n - 1 {
                self.dice[j] = self.dice[j + 1];
            }
            self.nd -= 1;
        }
        hit
    }

    fn max_use(&self) -> u8 {
        self.raw_moves()
            .into_iter()
            .map(|m| {
                let mut c = *self;
                c.step(m);
                1 + c.max_use()
            })
            .max()
            .unwrap_or(0)
    }

    fn pips(&self, p: Player) -> i32 {
        let mut t = self.bar[p.idx()] as i32 * 25;
        for n in 1..=24u8 {
            let c = self.own(p, n);
            if c > 0 {
                t += n as i32 * c as i32;
            }
        }
        t
    }

    fn structure(&self, p: Player) -> i32 {
        let mut v = 0;
        for n in 1..=24u8 {
            match self.own(p, n) {
                1 => v -= 4,
                c if c >= 2 => v += 2,
                _ => {}
            }
        }
        v
    }
}

#[derive(Clone)]
pub struct Backgammon {
    b: Board,
    roll: (u8, u8),
    played: u8,
    rng: Rng,
    over: bool,
    log: String,
}

impl Default for Backgammon {
    fn default() -> Self {
        Self::new()
    }
}

impl Backgammon {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let (a, b) = loop {
            let a = rng.d6();
            let b = rng.d6();
            if a != b {
                break (a, b);
            }
        };
        let mut g = Backgammon {
            b: Board::start(),
            roll: (a, b),
            played: 0,
            rng,
            over: false,
            log: String::new(),
        };
        g.b.side = if a > b { Player::One } else { Player::Two };
        g.b.dice[0] = a;
        g.b.dice[1] = b;
        g.b.nd = 2;
        g.log = format!("Zar de deschidere: {a} și {b}.");
        g.settle();
        g
    }

    /// Piese scoase de pe tablă: (Unu, Doi).
    pub fn borne_off(&self) -> (u8, u8) {
        (self.b.off[0], self.b.off[1])
    }

    /// Zarurile încă de jucat în tura curentă.
    pub fn dice_left(&self) -> Vec<u8> {
        self.b.dice[..self.b.nd as usize].to_vec()
    }

    fn roll_new(&mut self) {
        let a = self.rng.d6();
        let b = self.rng.d6();
        self.roll = (a, b);
        self.played = 0;
        self.b.dice = if a == b { [a; 4] } else { [a, b, 0, 0] };
        self.b.nd = if a == b { 4 } else { 2 };
    }

    /// Încheie automat tura când nu mai există mutări; aruncă pentru următorul.
    fn settle(&mut self) {
        while !self.over {
            if self.b.nd > 0 && !self.b.raw_moves().is_empty() {
                return;
            }
            if self.played == 0 {
                self.log.push_str(&format!(
                    " Jucătorul {} nu are mutări legale și pasează.",
                    self.b.side.idx() + 1
                ));
            }
            self.b.side = self.b.side.other();
            self.roll_new();
            self.log.push_str(&format!(
                " Jucătorul {} aruncă {} și {}.",
                self.b.side.idx() + 1,
                self.roll.0,
                self.roll.1
            ));
        }
    }

    fn text(&self, m: &Mv) -> String {
        if m.from == 25 {
            format!("bar/{}", 25 - m.die)
        } else if m.from > m.die {
            format!("{}/{}", m.from, m.from - m.die)
        } else {
            format!("{}/off", m.from)
        }
    }
}

impl Game for Backgammon {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.b.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if self.over {
            return Vec::new();
        }
        let raw = self.b.raw_moves();
        if raw.is_empty() {
            return raw;
        }
        let scored: Vec<(Mv, u8)> = raw
            .into_iter()
            .map(|m| {
                let mut c = self.b;
                c.step(m);
                (m, 1 + c.max_use())
            })
            .collect();
        let best = scored.iter().map(|x| x.1).max().unwrap_or(0);
        let mut cand: Vec<Mv> = scored
            .into_iter()
            .filter(|x| x.1 == best)
            .map(|x| x.0)
            .collect();
        if best == 1 && self.b.nd >= 2 {
            let maxd = self.b.dice[..self.b.nd as usize]
                .iter()
                .copied()
                .max()
                .unwrap_or(0);
            if cand.iter().any(|m| m.die == maxd) {
                cand.retain(|m| m.die == maxd);
            }
        }
        // același text (bear-off cu zaruri diferite) = aceeași mutare
        let mut seen: Vec<String> = Vec::new();
        cand.retain(|m| {
            let t = self.text(m);
            if seen.contains(&t) {
                false
            } else {
                seen.push(t);
                true
            }
        });
        cand
    }

    fn apply(&mut self, m: &Mv) {
        let who = self.b.side.idx() + 1;
        let hit = self.b.step(*m);
        self.played += 1;
        self.log = format!(
            "Jucătorul {who}: {}{}.",
            self.text(m),
            if hit { " (lovește)" } else { "" }
        );
        if self.b.off[self.b.side.idx()] == 15 {
            self.over = true;
            self.b.nd = 0;
            return;
        }
        self.settle();
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.over {
            Some(Outcome::Win(if self.b.off[0] == 15 {
                Player::One
            } else {
                Player::Two
            }))
        } else {
            None
        }
    }

    fn evaluate(&self, p: Player) -> i32 {
        let o = p.other();
        let race = self.b.pips(o) - self.b.pips(p);
        race + self.b.structure(p) - self.b.structure(o)
    }

    fn render(&self) -> String {
        let s = self.b.side;
        let cell = |n: u8| -> String {
            let c = self.b.own(s, n);
            let (mine, theirs) = (sym(s), sym(s.other()));
            match c {
                0 => " . ".to_string(),
                c if c > 0 => format!("{:>2}{}", c, mine),
                c => format!("{:>2}{}", -c, theirs),
            }
        };
        let mut out = String::new();
        out.push_str(&format!(
            "Punctele sunt numerotate din perspectiva jucătorului {} ({}).\n\n",
            s.idx() + 1,
            sym(s)
        ));
        out.push_str("  ");
        for n in 13..=24u8 {
            out.push_str(&format!("{n:>3} "));
            if n == 18 {
                out.push_str("| ");
            }
        }
        out.push_str("\n  ");
        for n in 13..=24u8 {
            out.push_str(&format!("{} ", cell(n)));
            if n == 18 {
                out.push_str("| ");
            }
        }
        out.push_str("\n  ");
        for n in (1..=12u8).rev() {
            out.push_str(&format!("{} ", cell(n)));
            if n == 7 {
                out.push_str("| ");
            }
        }
        out.push_str("\n  ");
        for n in (1..=12u8).rev() {
            out.push_str(&format!("{n:>3} "));
            if n == 7 {
                out.push_str("| ");
            }
        }
        out.push_str(&format!(
            "\n\nBară: X={} O={}   Scoase: X={} O={}\n",
            self.b.bar[0], self.b.bar[1], self.b.off[0], self.b.off[1]
        ));
        if !self.over {
            out.push_str(&format!(
                "Zaruri aruncate: {} și {}   Rămase: {:?}\n",
                self.roll.0,
                self.roll.1,
                self.dice_left()
            ));
        }
        out.push_str(self.log.trim());
        out.push('\n');
        out
    }

    fn format_move(&self, m: &Mv) -> String {
        self.text(m)
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

fn sym(p: Player) -> char {
    match p {
        Player::One => 'X',
        Player::Two => 'O',
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn game_with(b: Board, dice: &[u8]) -> Backgammon {
        let mut g = Backgammon::with_seed(1);
        g.b = b;
        g.b.nd = dice.len() as u8;
        g.b.dice = [0; 4];
        g.b.dice[..dice.len()].copy_from_slice(dice);
        g.roll = (dice[0], *dice.get(1).unwrap_or(&dice[0]));
        g.played = 0;
        g.over = false;
        g
    }

    fn texts(g: &Backgammon) -> Vec<String> {
        let mut v: Vec<String> = g.legal_moves().iter().map(|m| g.text(m)).collect();
        v.sort();
        v
    }

    fn put(b: &mut Board, p: Player, n: u8, c: i8) {
        b.pts[idx(p, n)] = c * sgn(p);
    }

    #[test]
    fn start_position_is_standard() {
        let b = Board::start();
        let one: i32 = b.pts.iter().filter(|&&x| x > 0).map(|&x| x as i32).sum();
        let two: i32 = b.pts.iter().filter(|&&x| x < 0).map(|&x| -x as i32).sum();
        assert_eq!((one, two), (15, 15));
        assert_eq!(b.own(Player::One, 24), 2);
        assert_eq!(b.own(Player::Two, 24), 2);
        assert_eq!(b.own(Player::One, 1), -2); // punctul 24 al lui Doi
        assert_eq!(b.own(Player::One, 12), -5); // punctul 13 al lui Doi
        assert_eq!(b.pips(Player::One), 167);
        assert_eq!(b.pips(Player::Two), 167);
    }

    #[test]
    fn opening_roll_has_distinct_dice_and_moves() {
        for seed in 0..20 {
            let g = Backgammon::with_seed(seed);
            assert_ne!(g.roll.0, g.roll.1);
            assert!(!g.legal_moves().is_empty());
        }
    }

    #[test]
    fn same_seed_same_game() {
        let mut a = Backgammon::with_seed(5);
        let mut b = Backgammon::with_seed(5);
        for _ in 0..30 {
            let m = a.legal_moves()[0];
            a.apply(&m);
            let m = b.legal_moves()[0];
            b.apply(&m);
            assert_eq!(a.render(), b.render());
        }
    }

    #[test]
    fn blocked_point_is_illegal_and_blot_is_hit() {
        let mut b = Board::empty();
        put(&mut b, Player::One, 10, 1);
        put(&mut b, Player::Two, 24 - 4 + 1 - 1, 0); // nimic
                                                     // Doi are 2 piese pe punctul lui Unu 7 -> blocat; 1 piesă pe 8 -> lovit
        b.pts[idx(Player::One, 7)] = -2;
        b.pts[idx(Player::One, 8)] = -1;
        put(&mut b, Player::One, 20, 1);
        let g = game_with(b, &[3, 2]);
        let t = texts(&g);
        assert!(!t.contains(&"10/7".to_string()), "{t:?}");
        assert!(t.contains(&"10/8".to_string()), "{t:?}");
        let mut g2 = g.clone();
        let m = *g2
            .legal_moves()
            .iter()
            .find(|m| g2.text(m) == "10/8")
            .unwrap();
        g2.apply(&m);
        assert_eq!(g2.b.bar[1], 1);
        assert_eq!(g2.b.own(Player::One, 8), 1);
    }

    #[test]
    fn bar_checker_must_enter_first() {
        let mut b = Board::empty();
        b.bar[0] = 1;
        put(&mut b, Player::One, 13, 2);
        b.pts[idx(Player::One, 21)] = -2; // blochează intrarea cu 4 (25-4=21)
        let g = game_with(b, &[4, 2]);
        let t = texts(&g);
        assert_eq!(t, vec!["bar/23".to_string()]);
        let mut g = g;
        let m = g.legal_moves()[0];
        g.apply(&m);
        assert_eq!(g.b.bar[0], 0);
        // acum 4 e jucabil de pe tablă
        assert!(texts(&g).contains(&"13/9".to_string()));
    }

    #[test]
    fn no_bearoff_until_all_home() {
        let mut b = Board::empty();
        put(&mut b, Player::One, 3, 1);
        put(&mut b, Player::One, 8, 1);
        let g = game_with(b, &[3, 1]);
        assert!(!texts(&g).contains(&"3/off".to_string()));
        let mut b2 = Board::empty();
        put(&mut b2, Player::One, 3, 1);
        put(&mut b2, Player::One, 5, 1);
        let g2 = game_with(b2, &[3, 1]);
        assert!(texts(&g2).contains(&"3/off".to_string()));
    }

    #[test]
    fn overshoot_bearoff_only_from_highest() {
        let mut b = Board::empty();
        put(&mut b, Player::One, 2, 1);
        put(&mut b, Player::One, 4, 1);
        let g = game_with(b, &[6]);
        // 6 > 4 si 4 e cel mai înalt: doar 4/off
        assert_eq!(texts(&g), vec!["4/off".to_string()]);
        let g = game_with(b, &[5]);
        assert_eq!(texts(&g), vec!["4/off".to_string()]);
        let g = game_with(b, &[3]);
        let t = texts(&g);
        assert!(t.contains(&"4/1".to_string()));
        assert!(!t.contains(&"2/off".to_string()));
    }

    #[test]
    fn bearing_off_last_checker_wins() {
        let mut b = Board::empty();
        put(&mut b, Player::One, 1, 1);
        b.off[0] = 14;
        b.off[1] = 3;
        put(&mut b, Player::Two, 2, 2);
        let mut g = game_with(b, &[1, 2]);
        let m = g.legal_moves()[0];
        assert_eq!(g.text(&m), "1/off");
        g.apply(&m);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
        assert!(g.legal_moves().is_empty());
    }

    #[test]
    fn doubles_give_four_moves() {
        let mut b = Board::empty();
        put(&mut b, Player::One, 24, 4);
        put(&mut b, Player::Two, 20, 2);
        let mut g = game_with(b, &[2, 2, 2, 2]);
        for k in 0..4 {
            assert_eq!(g.b.nd, 4 - k);
            let m = g.legal_moves()[0];
            g.apply(&m);
            if k < 3 {
                assert_eq!(g.b.side, Player::One, "tura continuă");
            }
        }
        assert_eq!(g.b.side, Player::Two, "tura a trecut");
    }

    #[test]
    fn must_use_both_dice_when_possible() {
        // Piesa de pe 8, zaruri 3 și 6; punctul 2 e blocat. Cu 6 direct (8/2) e blocat.
        // 8/5 e legal: apoi piesa e în casă și 6 o scoate (ambele zaruri se joacă).
        let mut b = Board::empty();
        put(&mut b, Player::One, 8, 1);
        b.pts[idx(Player::One, 2)] = -2;
        let g = game_with(b, &[3, 6]);
        let t = texts(&g);
        assert!(!t.contains(&"8/2".to_string()));
        assert!(t.contains(&"8/5".to_string()));
        // Cu o a doua piesă blocată: dacă 8/5 ar lăsa al doilea zar nejucabil, mutarea e ilegală.
        let mut b = Board::empty();
        put(&mut b, Player::One, 8, 1);
        put(&mut b, Player::One, 20, 1);
        // 8/5 (3) apoi 6: 5->off nu (20 nu e acasă) ; 20/14 (6) blocat; 5/-1 nu. 8/2 (6) apoi 3: 20/17 deschis.
        b.pts[idx(Player::One, 14)] = -2;
        b.pts[idx(Player::One, 5)] = -2;
        let g = game_with(b, &[3, 6]);
        // 8/5 blocat deja. 8/2 (6) apoi 3 (20/17) = ambele zaruri; 20/17 (3) apoi 6: 17/11 sau 8/2 = ambele.
        let t = texts(&g);
        assert!(
            t.contains(&"8/2".to_string()) && t.contains(&"20/17".to_string()),
            "{t:?}"
        );
        assert!(g.b.raw_moves().len() >= t.len());
    }

    #[test]
    fn larger_die_forced_when_only_one_can_be_played() {
        // Piesa de pe 9, zaruri 5 și 3; punctul 1 blocat. 9/4 apoi 4/1 blocat;
        // 9/6 apoi 6/1 blocat: se poate juca un singur zar, iar regula cere pe cel mare.
        let mut b = Board::empty();
        put(&mut b, Player::One, 9, 1);
        b.pts[idx(Player::One, 1)] = -2;
        let g = game_with(b, &[3, 5]);
        assert_eq!(texts(&g), vec!["9/4".to_string()]);
    }

    #[test]
    fn no_moves_passes_turn_automatically() {
        // Unu pe bară, toate intrările blocate de Doi
        let mut b = Board::empty();
        b.bar[0] = 1;
        for n in 19..=24u8 {
            b.pts[idx(Player::One, n)] = -2;
        }
        put(&mut b, Player::Two, 10, 1);
        let mut g = game_with(b, &[6, 5]);
        g.settle();
        assert_eq!(g.b.side, Player::Two, "{}", g.render());
        assert!(g.log.contains("pasează"));
    }

    #[test]
    fn checkers_are_conserved_in_random_games() {
        for seed in 1..=8u64 {
            let mut g = Backgammon::with_seed(seed);
            let mut r = Rng::new(seed + 100);
            let mut n = 0;
            while g.outcome().is_none() && n < 5000 {
                let mv = g.legal_moves();
                assert!(!mv.is_empty(), "{}", g.render());
                let m = mv[r.below(mv.len())];
                g.apply(&m);
                let b = &g.b;
                let one: i32 = b
                    .pts
                    .iter()
                    .filter(|&&x| x > 0)
                    .map(|&x| x as i32)
                    .sum::<i32>()
                    + b.bar[0] as i32
                    + b.off[0] as i32;
                let two: i32 = b
                    .pts
                    .iter()
                    .filter(|&&x| x < 0)
                    .map(|&x| -x as i32)
                    .sum::<i32>()
                    + b.bar[1] as i32
                    + b.off[1] as i32;
                assert_eq!((one, two), (15, 15));
                n += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed} nu s-a terminat");
        }
    }

    #[test]
    fn ai_beats_random_most_of_the_time() {
        let mut wins = 0;
        for seed in 1..=10u64 {
            let mut g = Backgammon::with_seed(seed);
            let mut r = Rng::new(seed * 7);
            let mut n = 0;
            while g.outcome().is_none() && n < 5000 {
                let m = if g.to_move() == Player::One {
                    polytabletop_core::ai::best_move(&g, r.next_u64()).unwrap()
                } else {
                    let mv = g.legal_moves();
                    mv[r.below(mv.len())]
                };
                g.apply(&m);
                n += 1;
            }
            if g.outcome() == Some(Outcome::Win(Player::One)) {
                wins += 1;
            }
        }
        assert!(wins >= 7, "AI a câștigat doar {wins}/10");
    }
}
