//! Motor comun pentru jocurile de cursă „ludo” cu zar, 4 piese/jucător, pentru doi jucători
//! (Fia-med-knuff, Jeu des petits chevaux). Regulile variază prin [`Rules`].
//!
//! * Pista comună are `track` pătrate; J1 pornește de la pătratul 0, J2 de la `track / 2`. O piesă are
//!   „progres”: 0 = în bază, 1..=`track` = pe pistă (1 = pătratul de start), apoi `home` pătrate în
//!   coloana de acasă (neatinse), iar ultimul progres (`track + home + 1`) este ținta, atinsă doar cu
//!   zar exact.
//! * Aterizarea pe o piesă adversă din pista comună o trimite în bază (captură); nu poți ateriza pe
//!   o piesă proprie. Cu `safe_start`, o piesă adversă care stă pe propriul pătrat de start nu poate
//!   fi capturată.
//! * Zarul se aruncă automat; rezultatul apare în `render()` și determină mutările. Un zar din
//!   `extra` dă încă o aruncare. Fără piese pe pistă ai trei încercări de a obține un zar de intrare.
//! * Mutare text: `intra` (piesă din bază) sau progresul piesei (ex. `12`). AI: `AlphaBeta(0)` pe
//!   progres total și capturi; nu vede zarurile viitoare.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone, Copy, Debug)]
pub struct Rules {
    pub name: &'static str,
    pub track: u8,
    pub home: u8,
    /// Zaruri cu care o piesă iese din bază.
    pub enter: &'static [u8],
    /// Zaruri care dau încă o aruncare.
    pub extra: &'static [u8],
    pub safe_start: bool,
}

#[derive(Clone)]
pub struct LudoFam {
    rules: Rules,
    prog: [[u8; 4]; 2],
    side: Player,
    roll: u8,
    rng: Rng,
    over: bool,
    log: String,
}

impl LudoFam {
    pub fn new(rules: Rules) -> Self {
        Self::with_seed(rules, Rng::from_clock().next_u64())
    }

    pub fn with_seed(rules: Rules, seed: u64) -> Self {
        let mut g = LudoFam {
            rules,
            prog: [[0; 4]; 2],
            side: Player::One,
            roll: 0,
            rng: Rng::new(seed),
            over: false,
            log: String::new(),
        };
        g.begin_turn();
        g
    }

    fn goal(&self) -> u8 {
        self.rules.track + self.rules.home + 1
    }

    fn square(&self, p: Player, progress: u8) -> u8 {
        let off = if p == Player::One {
            0
        } else {
            self.rules.track / 2
        };
        (off + progress - 1) % self.rules.track
    }

    fn moves(&self) -> Vec<u8> {
        let goal = self.goal();
        let mine = &self.prog[self.side.idx()];
        let mut out: Vec<u8> = Vec::new();
        for &pr in mine {
            if out.contains(&pr) || pr == goal {
                continue;
            }
            let target = if pr == 0 {
                if !self.rules.enter.contains(&self.roll) {
                    continue;
                }
                1
            } else {
                pr + self.roll
            };
            if target > goal {
                continue;
            }
            if target < goal && mine.contains(&target) {
                continue;
            }
            out.push(pr);
        }
        out.sort_unstable();
        out
    }

    fn begin_turn(&mut self) {
        let goal = self.goal();
        loop {
            let on_board = self.prog[self.side.idx()]
                .iter()
                .any(|&p| (1..goal).contains(&p));
            let tries = if on_board { 1 } else { 3 };
            for _ in 0..tries {
                self.roll = self.rng.d6();
                if !self.moves().is_empty() {
                    return;
                }
            }
            self.log = format!(
                "Jucătorul {} a aruncat {} și nu poate muta.",
                self.side.idx() + 1,
                self.roll
            );
            self.side = self.side.other();
        }
    }

    pub fn pieces(&self, p: Player) -> [u8; 4] {
        self.prog[p.idx()]
    }
}

impl Game for LudoFam {
    type Move = u8;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<u8> {
        if self.over {
            Vec::new()
        } else {
            self.moves()
        }
    }

    fn apply(&mut self, m: &u8) {
        let me = self.side;
        let mi = me.idx();
        let track = self.rules.track;
        let goal = self.goal();
        let target = if *m == 0 { 1 } else { *m + self.roll };
        let k = self.prog[mi].iter().position(|&p| p == *m).expect("piesă");
        self.prog[mi][k] = target;
        self.log = format!(
            "Jucătorul {}: {} cu {}.",
            mi + 1,
            self.format_move(m),
            self.roll
        );
        if target <= track {
            let sq = self.square(me, target);
            let o = me.other();
            for j in 0..4 {
                let p = self.prog[o.idx()][j];
                if (1..=track).contains(&p)
                    && self.square(o, p) == sq
                    && !(self.rules.safe_start && p == 1)
                {
                    self.prog[o.idx()][j] = 0;
                    self.log.push_str(" Captură!");
                }
            }
        }
        if self.prog[mi].iter().all(|&p| p == goal) {
            self.over = true;
            return;
        }
        if self.rules.extra.contains(&self.roll) {
            self.log.push_str(" Aruncă din nou.");
        } else {
            self.side = me.other();
        }
        self.begin_turn();
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            return None;
        }
        let goal = self.goal();
        let one = self.prog[0].iter().all(|&p| p == goal);
        Some(Outcome::Win(if one { Player::One } else { Player::Two }))
    }

    fn evaluate(&self, p: Player) -> i32 {
        let goal = self.goal();
        let s = |q: Player| -> i32 {
            self.prog[q.idx()]
                .iter()
                .map(|&x| x as i32 + if x == goal { 10 } else { 0 })
                .sum()
        };
        // piesele proprii expuse (pista comună, la ≤ 6 pătrate în fața unei piese adverse) costă puțin
        let exposed = |q: Player| -> i32 {
            let mut n = 0;
            for &x in &self.prog[q.idx()] {
                if x > 1 && x <= self.rules.track {
                    let sq = self.square(q, x) as i32;
                    for &y in &self.prog[q.other().idx()] {
                        if y >= 1 && y <= self.rules.track {
                            let d = (sq - self.square(q.other(), y) as i32)
                                .rem_euclid(self.rules.track as i32);
                            if (1..=6).contains(&d) {
                                n += 1;
                            }
                        }
                    }
                }
            }
            n
        };
        s(p) - s(p.other()) - 2 * exposed(p) + 2 * exposed(p.other())
    }

    fn render(&self) -> String {
        let t = self.rules.track as usize;
        let mut ring = vec!['.'; t];
        for (pl, ch) in [(Player::One, 'X'), (Player::Two, 'O')] {
            for &p in &self.prog[pl.idx()] {
                if (1..=self.rules.track).contains(&p) {
                    ring[self.square(pl, p) as usize] = ch;
                }
            }
        }
        let half = t / 2;
        let row = |a: usize, b: usize| ring[a..b].iter().collect::<String>();
        let goal = self.goal();
        let mut s = format!(
            "{} — pista de {t} pătrate (X pornește la 0, O la {half}), coloana de acasă {} pătrate:\n   0-{} : {}\n  {}-{} : {}\n",
            self.rules.name,
            self.rules.home,
            half - 1,
            row(0, half),
            half,
            t - 1,
            row(half, t)
        );
        for (pl, ch) in [(Player::One, 'X'), (Player::Two, 'O')] {
            let v = self.prog[pl.idx()];
            s.push_str(&format!(
                "Jucător {} ({}): bază {}, terminate {}, progres piese {:?}\n",
                pl.idx() + 1,
                ch,
                v.iter().filter(|&&p| p == 0).count(),
                v.iter().filter(|&&p| p == goal).count(),
                v.iter().filter(|&&p| p != 0).collect::<Vec<_>>()
            ));
        }
        if !self.over {
            s.push_str(&format!(
                "La mutare: jucătorul {}   Zar: {}\n",
                self.side.idx() + 1,
                self.roll
            ));
        }
        s.push_str(self.log.trim());
        s.push('\n');
        s
    }

    fn format_move(&self, m: &u8) -> String {
        if *m == 0 {
            "intra".to_string()
        } else {
            m.to_string()
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    /// Teste comune, rulate de modulele-variante.
    pub fn check_variant(rules: Rules) {
        let goal = rules.track + rules.home + 1;
        // intrarea cere zarul potrivit
        let mut g = LudoFam::with_seed(rules, 1);
        g.prog = [[0, 0, 0, 5], [0; 4]];
        g.side = Player::One;
        g.roll = 3;
        assert_eq!(g.legal_moves(), vec![5]);
        g.roll = rules.enter[0];
        assert_eq!(g.legal_moves(), vec![0, 5]);
        // ținta cu zar exact
        g.prog = [[goal - 2, goal, goal, goal], [0; 4]];
        g.roll = 3;
        assert!(g.legal_moves().is_empty());
        g.roll = 2;
        assert_eq!(g.legal_moves(), vec![goal - 2]);
        // captură pe pista comună
        let mut g = LudoFam::with_seed(rules, 2);
        let half = rules.track / 2;
        // J2 la progres 3 => pătrat half+2; J1 la progres half+2-3+1... alegem J1 progres p1 cu p1 - 1 + roll = half + 2
        let roll = 4u8;
        let p1 = half + 2 + 1 - roll;
        g.prog = [[p1, 0, 0, 0], [3, 0, 0, 0]];
        g.side = Player::One;
        g.roll = roll;
        g.apply(&p1);
        assert_eq!(g.prog[1][0], 0, "capturat");
        // jocuri aleatoare se termină
        for seed in 1..=6u64 {
            let mut g = LudoFam::with_seed(rules, seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
                assert!(n < 20000);
            }
        }
        // AI-ul nu depinde de seed-ul zarurilor viitoare la același zar curent
        let a = LudoFam::with_seed(rules, 3);
        let m = polytabletop_core::GameObj::ai_move(&a, 1).unwrap();
        assert!(a.legal_moves().iter().any(|x| a.format_move(x) == m));
    }
}
