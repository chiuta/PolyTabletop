//! Mastermind în două runde cu schimb de roluri (un singur joc).
//!
//! * Cod de 4 culori (cifrele 1–6, repetiții permise), cel mult 10 încercări.
//! * Runda 1: Jucătorul 1 sparge codul ascuns al AI-ului (generat cu `Rng`-ul
//!   jocului). Încercare = patru cifre, ex. `1123`. Răspunsul: câte poziții
//!   exacte („negre”) și câte culori corecte pe poziții greșite („albe”).
//! * Runda 2: Jucătorul 1 își alege un cod secret (`cod:1234`, rămâne în stare
//!   și apare doar în ecranul lui), iar Jucătorul 2 (AI) îl sparge. Răspunsurile
//!   sunt calculate de joc, nu de om; după încercarea reușită sau a 10-a,
//!   codul-maestru confirmă printr-o mutare forțată (`confirm`), ca AI-ul să nu
//!   „vadă” dinainte dacă ultima încercare e corectă.
//! * Câștigă cine a spart codul în mai puține încercări (nespart = 11); egal =
//!   remiză.
//!
//! AI fără trișat: scorul fiecărei încercări se calculează ÎNAINTE de a fi
//! aplicată, doar din încercările lui anterioare și răspunsurile lor: o
//! încercare compatibilă cu toate răspunsurile de până acum (ar putea fi codul)
//! primește scor maxim. `evaluate` citește doar acest scor.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const PEGS: usize = 4;
pub const COLORS: u8 = 6;
pub const MAX_GUESSES: usize = 10;

pub type Code = [u8; PEGS];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Guess(Code),
    SetCode(Code),
    Confirm,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Round1,
    Setup,
    Round2,
    Ack,
    Over,
}

#[derive(Clone)]
pub struct Mastermind {
    phase: Phase,
    /// `secret[0]` = codul lui J1 (runda 2), `secret[1]` = codul AI-ului (runda 1).
    secret: [Option<Code>; 2],
    /// `guesses[0]` = încercările lui J1 (runda 1), `guesses[1]` = ale lui J2 (runda 2).
    guesses: [Vec<(Code, (u8, u8))>; 2],
    solved: [bool; 2],
    side: Player,
    hint: i32,
    hint_for: Player,
    /// `Some(p)` = render shows `p`'s own secret code; `None` = hot-seat (the mover's).
    viewer: Option<Player>,
}

impl Default for Mastermind {
    fn default() -> Self {
        Self::new()
    }
}

/// (exacte, culori corecte pe poziții greșite)
pub fn feedback(secret: &Code, guess: &Code) -> (u8, u8) {
    let mut exact = 0u8;
    let mut sc = [0u8; COLORS as usize + 1];
    let mut gc = [0u8; COLORS as usize + 1];
    for i in 0..PEGS {
        if secret[i] == guess[i] {
            exact += 1;
        } else {
            sc[secret[i] as usize] += 1;
            gc[guess[i] as usize] += 1;
        }
    }
    let white: u8 = (1..=COLORS as usize).map(|c| sc[c].min(gc[c])).sum();
    (exact, white)
}

fn all_codes() -> Vec<Code> {
    let mut v = Vec::with_capacity(1296);
    for a in 1..=COLORS {
        for b in 1..=COLORS {
            for c in 1..=COLORS {
                for d in 1..=COLORS {
                    v.push([a, b, c, d]);
                }
            }
        }
    }
    v
}

fn code_text(c: &Code) -> String {
    c.iter().map(|d| d.to_string()).collect()
}

impl Mastermind {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut code = [0u8; PEGS];
        for d in code.iter_mut() {
            *d = rng.below(COLORS as usize) as u8 + 1;
        }
        Mastermind {
            phase: Phase::Round1,
            secret: [None, Some(code)],
            guesses: [Vec::new(), Vec::new()],
            solved: [false, false],
            side: Player::One,
            hint: 0,
            hint_for: Player::One,
            viewer: Some(Player::One),
        }
    }

    /// Numărul de încercări cu care a spart `breaker` codul (11 dacă nu).
    pub fn score(&self, breaker: Player) -> usize {
        if self.solved[breaker.idx()] {
            self.guesses[breaker.idx()].len()
        } else {
            MAX_GUESSES + 1
        }
    }

    pub fn guesses_made(&self, breaker: Player) -> usize {
        self.guesses[breaker.idx()].len()
    }

    /// Codul secret al AI-ului (pentru teste).
    pub fn ai_secret(&self) -> Option<Code> {
        self.secret[1]
    }

    /// Cât de compatibilă e `g` cu istoricul lui `breaker` (doar informație proprie).
    fn consistency(&self, breaker: Player, g: &Code) -> i32 {
        let hist = &self.guesses[breaker.idx()];
        let bad = hist.iter().filter(|(c, fb)| feedback(c, g) != *fb).count() as i32;
        if bad == 0 {
            1000
        } else {
            -bad
        }
    }

    fn round_done(&mut self, breaker: Player) {
        let g = &self.guesses[breaker.idx()];
        let last_ok = g.last().map(|x| x.1 == (PEGS as u8, 0)).unwrap_or(false);
        if last_ok {
            self.solved[breaker.idx()] = true;
        }
        if last_ok || g.len() >= MAX_GUESSES {
            match breaker {
                Player::One => {
                    self.phase = Phase::Setup;
                    self.side = Player::One;
                }
                Player::Two => {
                    self.phase = Phase::Ack;
                    self.side = Player::One;
                }
            }
        }
    }
}

impl Game for Mastermind {
    type Move = Move;

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        match self.phase {
            Phase::Round1 | Phase::Round2 => all_codes().into_iter().map(Move::Guess).collect(),
            Phase::Setup => all_codes().into_iter().map(Move::SetCode).collect(),
            Phase::Ack => vec![Move::Confirm],
            Phase::Over => Vec::new(),
        }
    }

    fn apply(&mut self, m: &Move) {
        match (*m, self.phase) {
            (Move::Guess(g), Phase::Round1) => {
                let sec = self.secret[1].expect("cod AI");
                self.hint = self.consistency(Player::One, &g);
                self.hint_for = Player::One;
                let fb = feedback(&sec, &g);
                self.guesses[0].push((g, fb));
                self.round_done(Player::One);
            }
            (Move::SetCode(c), Phase::Setup) => {
                self.secret[0] = Some(c);
                self.phase = Phase::Round2;
                self.side = Player::Two;
            }
            (Move::Guess(g), Phase::Round2) => {
                let sec = self.secret[0].expect("cod J1");
                self.hint = self.consistency(Player::Two, &g);
                self.hint_for = Player::Two;
                let fb = feedback(&sec, &g);
                self.guesses[1].push((g, fb));
                self.round_done(Player::Two);
            }
            (Move::Confirm, Phase::Ack) => {
                self.phase = Phase::Over;
            }
            _ => unreachable!("mutare în fază greșită"),
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        let (a, b) = (self.score(Player::One), self.score(Player::Two));
        Some(if a < b {
            Outcome::Win(Player::One)
        } else if b < a {
            Outcome::Win(Player::Two)
        } else {
            Outcome::Draw
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        if self.hint_for == p {
            self.hint
        } else {
            0
        }
    }

    fn render(&self) -> String {
        let mut s = String::new();
        s.push_str("Culori 1–6, cod de 4 cifre. Răspuns: N = poziție exactă, A = culoare corectă pe altă poziție.\n");
        let v = self.viewer.unwrap_or(self.side);
        let list = |g: &Vec<(Code, (u8, u8))>| -> String {
            let mut t = String::new();
            for (i, (c, (n, a))) in g.iter().enumerate() {
                t.push_str(&format!(
                    "  {:>2}. {}  {}N {}A\n",
                    i + 1,
                    code_text(c),
                    n,
                    a
                ));
            }
            t
        };
        s.push_str(&format!(
            "Runda 1: tu spargi codul AI-ului ({} din {} încercări){}\n",
            self.guesses[0].len(),
            MAX_GUESSES,
            if self.solved[0] { " — SPART" } else { "" }
        ));
        s.push_str(&list(&self.guesses[0]));
        if matches!(self.phase, Phase::Setup) {
            s.push_str("Runda 2: alege-ți codul secret: cod:1234\n");
        }
        let shown = |c: Code| -> String {
            if v == Player::One || self.phase == Phase::Over {
                format!("({})", code_text(&c))
            } else {
                "(ascuns)".to_string()
            }
        };
        if let (Player::Two, Some(c), false) = (v, self.secret[1], self.phase == Phase::Over) {
            s.push_str(&format!("Codul tău secret (J2): {}\n", code_text(&c)));
        }
        if let Some(c) = self.secret[0] {
            s.push_str(&format!(
                "Runda 2: AI-ul sparge codul tău {} ({} din {} încercări){}\n",
                shown(c),
                self.guesses[1].len(),
                MAX_GUESSES,
                if self.solved[1] { " — SPART" } else { "" }
            ));
            s.push_str(&list(&self.guesses[1]));
        }
        match self.phase {
            Phase::Ack => s.push_str("Runda 2 s-a încheiat: «confirm».\n"),
            Phase::Over => {
                if let Some(c) = self.secret[1] {
                    s.push_str(&format!("Codul AI-ului era {}.\n", code_text(&c)));
                }
                s.push_str(&format!(
                    "Încercări: tu {}, AI {} (nespart = 11).\n",
                    self.score(Player::One),
                    self.score(Player::Two)
                ));
            }
            _ => {}
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Guess(c) => code_text(c),
            Move::SetCode(c) => format!("cod:{}", code_text(c)),
            Move::Confirm => "confirm".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::GameObj;

    fn out(g: &Mastermind) -> Option<Outcome> {
        Game::outcome(g)
    }

    #[test]
    fn feedback_handles_duplicates() {
        assert_eq!(feedback(&[1, 1, 2, 2], &[1, 2, 1, 2]), (2, 2));
        assert_eq!(feedback(&[1, 2, 3, 4], &[1, 2, 3, 4]), (4, 0));
        assert_eq!(feedback(&[1, 1, 1, 2], &[2, 2, 2, 1]), (0, 2));
        assert_eq!(feedback(&[6, 6, 6, 6], &[1, 2, 3, 4]), (0, 0));
        assert_eq!(feedback(&[1, 2, 3, 4], &[4, 3, 2, 1]), (0, 4));
    }

    #[test]
    fn move_counts_and_texts() {
        let g = Mastermind::with_seed(1);
        assert_eq!(Game::legal_moves(&g).len(), 1296);
        assert!(GameObj::legal_moves(&g).contains(&"1234".to_string()));
    }

    #[test]
    fn solving_round_one_leads_to_setup_then_ai_breaks() {
        let mut g = Mastermind::with_seed(4);
        let sec = g.ai_secret().unwrap();
        g.apply(&Move::Guess(sec));
        assert_eq!(g.score(Player::One), 1);
        assert_eq!(Game::to_move(&g), Player::One);
        assert!(GameObj::legal_moves(&g)[0].starts_with("cod:"));
        g.apply(&Move::SetCode([3, 3, 5, 1]));
        assert_eq!(Game::to_move(&g), Player::Two);
        let mut n = 0;
        while out(&g).is_none() {
            let mv = GameObj::ai_move(&g, n).unwrap();
            GameObj::play(&mut g, &mv).unwrap();
            n += 1;
            assert!(n < 30);
        }
        assert_eq!(g.guesses_made(Player::Two) as u64 + 1, n); // + confirmarea
                                                               // AI-ul are nevoie de ≥ 2 încercări pentru a bate 1: pierde sau remiză
        assert_ne!(out(&g), Some(Outcome::Win(Player::Two)));
    }

    #[test]
    fn ai_breaks_codes_quickly_without_seeing_them() {
        let mut total = 0usize;
        for k in 0..60u64 {
            let mut g = Mastermind::with_seed(k + 1);
            // J1 nu nimerește (cod fix prost) până la 10 încercări
            for _ in 0..MAX_GUESSES {
                let sec = g.ai_secret().unwrap();
                let mut bad = [1, 1, 1, 1];
                if bad == sec {
                    bad = [2, 2, 2, 2];
                }
                g.apply(&Move::Guess(bad));
            }
            assert_eq!(g.score(Player::One), 11);
            let code = {
                let mut r = Rng::new(k * 17 + 3);
                [
                    r.below(6) as u8 + 1,
                    r.below(6) as u8 + 1,
                    r.below(6) as u8 + 1,
                    r.below(6) as u8 + 1,
                ]
            };
            g.apply(&Move::SetCode(code));
            let mut n = 0u64;
            while Game::to_move(&g) == Player::Two {
                let mv = GameObj::ai_move(&g, k * 100 + n).unwrap();
                GameObj::play(&mut g, &mv).unwrap();
                n += 1;
            }
            assert!(g.score(Player::Two) <= 10, "codul {:?} nespart", code);
            total += g.score(Player::Two);
            GameObj::play(&mut g, "confirm").unwrap();
            assert_eq!(out(&g), Some(Outcome::Win(Player::Two)));
        }
        assert!(total as f64 / 60.0 < 7.0, "medie {}", total as f64 / 60.0);
    }

    #[test]
    fn ai_ignores_the_hidden_code_when_choosing() {
        // aceeași istorie, coduri diferite dar cu aceleași răspunsuri → aceeași alegere
        let mut a = Mastermind::with_seed(1);
        a.apply(&Move::Guess([1, 1, 1, 1]));
        let mut b = a.clone();
        let _ = &mut b;
        let mut x = Mastermind::with_seed(1);
        let mut y = Mastermind::with_seed(1);
        for g in [&mut x, &mut y] {
            for _ in 0..MAX_GUESSES {
                g.apply(&Move::Guess([1, 2, 3, 4]));
            }
            g.apply(&Move::SetCode([5, 5, 5, 5]));
            g.apply(&Move::Guess([1, 2, 3, 4])); // J2 (AI) nimerește 0N 0A
        }
        // y: alt cod secret cu același răspuns pentru [1,2,3,4] (0,0)
        y.secret[0] = Some([6, 6, 5, 6]);
        y.guesses[1][0].1 = feedback(&[6, 6, 5, 6], &[1, 2, 3, 4]);
        assert_eq!(y.guesses[1][0].1, x.guesses[1][0].1);
        assert_eq!(GameObj::ai_move(&x, 9), GameObj::ai_move(&y, 9));
    }

    #[test]
    fn secret_not_in_render_before_the_end() {
        let g = Mastermind::with_seed(8);
        let sec = code_text(&g.ai_secret().unwrap());
        assert!(!Game::render(&g).contains(&format!("era {sec}")));
    }

    #[test]
    fn random_play_always_terminates() {
        for seed in 1..=10u64 {
            let mut g = Mastermind::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while out(&g).is_none() {
                let m = Game::legal_moves(&g);
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 60);
            }
        }
    }
}

#[cfg(test)]
mod viewer_tests {
    use super::*;

    #[test]
    fn secrets_follow_the_viewer() {
        let mut g = Mastermind::with_seed(1);
        g.secret = [Some([1, 2, 3, 4]), Some([6, 5, 4, 3])];
        Game::set_viewer(&mut g, Some(Player::One));
        let r = Game::render(&g);
        assert!(r.contains("(1234)") && !r.contains("6543"), "{r}");
        Game::set_viewer(&mut g, Some(Player::Two));
        let r = Game::render(&g);
        assert!(r.contains("6543") && !r.contains("1234"), "{r}");
        assert!(r.contains("(ascuns)"));
        Game::set_viewer(&mut g, None); // mută J1
        let r = Game::render(&g);
        assert!(r.contains("(1234)") && !r.contains("6543"), "{r}");
        g.side = Player::Two;
        let r = Game::render(&g);
        assert!(r.contains("6543") && !r.contains("1234"), "{r}");
    }
}
