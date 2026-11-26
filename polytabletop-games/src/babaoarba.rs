//! Baba-oarba (Blind man's buff), duel asimetric pe o tablă 7×7 pentru 2 jucători.
//!
//! * J1 e „baba” cu ochii legați, J2 e fugarul. Baba își știe poziția dar nu vede fugarul; fugarul vede tot.
//! * La rândul lui baba face un pas (`n`, `s`, `e`, `v` — sus, jos, dreapta, stânga — sau diagonal `ne` …) ori stă pe loc
//!   (`sta`), ori încearcă să prindă o căsuță vecină (`prind c4`), apoi `arat` arată dacă a nimerit.
//!   Fugarul face apoi un pas (orice direcție, inclusiv `sta`). După fiecare rând baba aude un indiciu: „cald” dacă fugarul e
//!   la cel mult 2 căsuțe (distanță Chebyshev), „rece” altfel.
//! * Baba câștigă prinzând fugarul; fugarul câștigă dacă rezistă 30 de runde.
//! * Informație ascunsă: poziția fugarului. AI-ul babei ține un set de căsuțe posibile (credință) actualizat doar din
//!   indicii și nu citește poziția adevărată; AI-ul fugarului vede tabla întreagă (ca un om).

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: i32 = 7;
const ROUNDS: u32 = 30;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Step(i8, i8),
    Grab(i8, i8),
    Show,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Seek,
    Reveal,
    Run,
}

#[derive(Clone)]
pub struct BabaOarba {
    seeker: (i32, i32),
    runner: (i32, i32),
    belief: [bool; 49],
    phase: Phase,
    grab: Option<(i32, i32)>,
    round: u32,
    caught: bool,
    viewer: Option<Player>,
    hint: String,
}

impl Default for BabaOarba {
    fn default() -> Self {
        Self::new()
    }
}

fn idx(p: (i32, i32)) -> usize {
    (p.1 * N + p.0) as usize
}

fn inside(p: (i32, i32)) -> bool {
    (0..N).contains(&p.0) && (0..N).contains(&p.1)
}

fn cheb(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs().max((a.1 - b.1).abs())
}

fn cell(p: (i32, i32)) -> String {
    format!("{}{}", (b'a' + p.0 as u8) as char, p.1 + 1)
}

impl BabaOarba {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let seeker = (0, 0);
        let runner = (N - 1 - rng.below(2) as i32, N - 1 - rng.below(2) as i32);
        let mut belief = [false; 49];
        for y in 0..N {
            for x in 0..N {
                belief[idx((x, y))] = cheb((x, y), seeker) > 1;
            }
        }
        BabaOarba {
            seeker,
            runner,
            belief,
            phase: Phase::Seek,
            grab: None,
            round: 0,
            caught: false,
            viewer: Some(Player::One),
            hint: String::new(),
        }
    }
    /// Actualizează credința babei după mutarea fugarului și indiciul primit.
    fn update_belief(&mut self, warm: bool) {
        let old = self.belief;
        let mut new = [false; 49];
        for y in 0..N {
            for x in 0..N {
                if !old[idx((x, y))] {
                    continue;
                }
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let q = (x + dx, y + dy);
                        if inside(q) && (cheb(q, self.seeker) <= 2) == warm {
                            new[idx(q)] = true;
                        }
                    }
                }
            }
        }
        if new.iter().all(|&b| !b) {
            // siguranță: indiciul contrazice credința → redeschide toate căsuțele compatibile
            for y in 0..N {
                for x in 0..N {
                    let q = (x, y);
                    new[idx(q)] = (cheb(q, self.seeker) <= 2) == warm;
                }
            }
        }
        self.belief = new;
    }
}

const DIRS: [(i8, i8); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

fn dir_name(d: (i8, i8)) -> String {
    let v = match d.1 {
        1 => "n",
        -1 => "s",
        _ => "",
    };
    let h = match d.0 {
        1 => "e",
        -1 => "v",
        _ => "",
    };
    format!("{v}{h}")
}

impl Game for BabaOarba {
    type Move = Move;
    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }
    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Run => Player::Two,
            _ => Player::One,
        }
    }
    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        match self.phase {
            Phase::Reveal => vec![Move::Show],
            Phase::Seek => {
                let mut v = vec![Move::Step(0, 0), Move::Grab(0, 0)];
                for d in DIRS {
                    let q = (self.seeker.0 + d.0 as i32, self.seeker.1 + d.1 as i32);
                    if inside(q) {
                        v.push(Move::Step(d.0, d.1));
                        v.push(Move::Grab(d.0, d.1));
                    }
                }
                v
            }
            Phase::Run => {
                let mut v = vec![Move::Step(0, 0)];
                for d in DIRS {
                    let q = (self.runner.0 + d.0 as i32, self.runner.1 + d.1 as i32);
                    if inside(q) {
                        v.push(Move::Step(d.0, d.1));
                    }
                }
                v
            }
        }
    }
    fn apply(&mut self, m: &Move) {
        match (self.phase, *m) {
            (Phase::Seek, Move::Step(dx, dy)) => {
                self.seeker = (self.seeker.0 + dx as i32, self.seeker.1 + dy as i32);
                self.phase = Phase::Run;
            }
            (Phase::Seek, Move::Grab(dx, dy)) => {
                self.grab = Some((self.seeker.0 + dx as i32, self.seeker.1 + dy as i32));
                self.phase = Phase::Reveal;
            }
            (Phase::Reveal, Move::Show) => {
                let c = self.grab.take().unwrap();
                if c == self.runner {
                    self.caught = true;
                } else {
                    self.belief[idx(c)] = false;
                    self.phase = Phase::Run;
                }
            }
            (Phase::Run, Move::Step(dx, dy)) => {
                self.runner = (self.runner.0 + dx as i32, self.runner.1 + dy as i32);
                let warm = cheb(self.runner, self.seeker) <= 2;
                self.hint = if warm { "cald".into() } else { "rece".into() };
                self.update_belief(warm);
                self.round += 1;
                self.phase = Phase::Seek;
            }
            _ => {}
        }
    }
    fn outcome(&self) -> Option<Outcome> {
        if self.caught {
            Some(Outcome::Win(Player::One))
        } else if self.round >= ROUNDS {
            Some(Outcome::Win(Player::Two))
        } else {
            None
        }
    }
    fn evaluate(&self, p: Player) -> i32 {
        // doar informația babei: credința și poziția ei; fugarul folosește aceeași măsură pe tabla reală
        let cells: Vec<(i32, i32)> = (0..N)
            .flat_map(|y| (0..N).map(move |x| (x, y)))
            .filter(|&q| self.belief[idx(q)])
            .collect();
        let cnt = cells.len().max(1) as i32;
        let near: i32 = cells
            .iter()
            .map(|&q| 8 - cheb(q, self.seeker).min(8))
            .sum::<i32>()
            / cnt;
        let mut v = near * 10;
        if let Some(c) = self.grab {
            let inb = self.belief[idx(c)] as i32;
            v += inb * 400 / cnt;
        }
        if p == Player::One {
            v
        } else {
            // fugarul (vede tot) se uită la distanța reală
            (cheb(self.runner, self.seeker) * 20 - near * 10).max(-1000)
        }
    }
    fn render(&self) -> String {
        let me = self.viewer.unwrap_or_else(|| self.to_move());
        let mut s = format!(
            "Baba-oarba — runda {}/{ROUNDS}; baba = J1 (B), fugarul = J2 (F)\n",
            self.round + 1
        );
        for y in (0..N).rev() {
            s.push_str(&format!("{} ", y + 1));
            for x in 0..N {
                let q = (x, y);
                let c = if q == self.seeker && !(q == self.runner && me == Player::Two) {
                    'B'
                } else if q == self.runner && (me == Player::Two || self.caught) {
                    if q == self.seeker {
                        'X'
                    } else {
                        'F'
                    }
                } else if me == Player::One && self.belief[idx(q)] && !self.caught {
                    '?'
                } else {
                    '.'
                };
                s.push(c);
                s.push(' ');
            }
            s.push('\n');
        }
        s.push_str("  a b c d e f g\n");
        if me == Player::One {
            s.push_str("(? = unde ar putea fi fugarul, după indicii)\n");
        }
        if !self.hint.is_empty() {
            s.push_str(&format!("Indiciu auzit de babă: {}\n", self.hint));
        }
        if let (Some(c), true) = (self.grab, me == Player::One) {
            s.push_str(&format!("Baba încearcă să prindă {} — `arat`.\n", cell(c)));
        }
        match self.outcome() {
            Some(Outcome::Win(Player::One)) => s.push_str("Baba a prins fugarul. Câștigă J1.\n"),
            Some(Outcome::Win(_)) => s.push_str("Fugarul a rezistat. Câștigă J2.\n"),
            _ => s.push_str(&format!("La mutare: J{}\n", self.to_move().idx() + 1)),
        }
        s
    }
    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Step(0, 0) => "sta".into(),
            Move::Grab(0, 0) => format!("prind {}", cell(self.seeker)),
            Move::Step(x, y) => dir_name((*x, *y)),
            Move::Grab(x, y) => {
                let q = (self.seeker.0 + *x as i32, self.seeker.1 + *y as i32);
                format!("prind {}", cell(q))
            }
            Move::Show => "arat".into(),
        }
    }
    fn ai_style(&self) -> AiStyle {
        // baba nu are voie să „simuleze” răspunsul fugarului (ar citi poziția lui): căutare de 1 ply
        if self.phase == Phase::Run {
            AiStyle::AlphaBeta(3)
        } else {
            AiStyle::AlphaBeta(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grab_resolves_only_on_show() {
        let mut g = BabaOarba::with_seed(1);
        g.runner = (1, 1);
        g.seeker = (0, 0);
        g.apply(&Move::Grab(1, 1));
        assert!(g.outcome().is_none());
        g.apply(&Move::Show);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn belief_follows_hints_and_runner_is_hidden_from_seeker() {
        let mut g = BabaOarba::with_seed(2);
        g.apply(&Move::Step(1, 0));
        g.apply(&Move::Step(0, 0));
        assert!(g.belief[idx(g.runner)]);
        let board = |g: &BabaOarba| {
            g.render()
                .lines()
                .filter(|l| l.starts_with(|c: char| c.is_ascii_digit()))
                .collect::<Vec<_>>()
                .join("\n")
        };
        g.set_viewer(Some(Player::One));
        assert!(!board(&g).contains('F'));
        g.set_viewer(Some(Player::Two));
        assert!(board(&g).contains('F'));
    }

    #[test]
    fn ai_seeker_ignores_true_runner_position() {
        let a = BabaOarba::with_seed(3);
        let mut b = a.clone();
        b.runner = (3, 3);
        assert_eq!(
            polytabletop_core::GameObj::ai_move(&a, 1),
            polytabletop_core::GameObj::ai_move(&b, 1)
        );
    }

    #[test]
    fn games_finish() {
        let mut g: Box<dyn polytabletop_core::GameObj> = Box::new(BabaOarba::with_seed(6));
        let mut k = 0;
        while g.outcome().is_none() {
            let m = g.ai_move(k).unwrap();
            g.play(&m).unwrap();
            k += 1;
            assert!(k < 400);
        }
    }
}
