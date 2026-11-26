//! „Nu te supăra, frate” (Ludo), 2 jucători, un zar, 4 piese fiecare.
//!
//! * Pista comună are 40 de pătrate; Unu pornește de la pătratul 0, Doi de la 20.
//!   Fiecare piesă are un „progres”: 0 = în bază, 1..=40 = pe pistă (1 = pătratul
//!   de start), 41..=43 = coloana de acasă, 44 = ținta (piesa e terminată).
//! * Un 6 scoate o piesă din bază pe pătratul de start (dacă nu e ocupat de o
//!   piesă proprie); nu poți ateriza pe o piesă proprie; ținta se atinge exact.
//! * Aterizarea pe pista comună peste o piesă a adversarului o trimite în bază
//!   (captură). Coloana de acasă nu poate fi capturată.
//! * Un 6 dă dreptul la încă o aruncare (după mutare). Fără piese pe tablă ai
//!   trei încercări de a arunca 6.
//! * Zarul se aruncă automat, în starea jocului, cu un `Rng` propriu; rezultatul
//!   curent apare în `render()` și determină `legal_moves()`. Dacă nu ai nicio
//!   mutare, tura trece automat.
//!
//! Mutări text: `intră` (piesă din bază) sau numărul progresului piesei pe care
//! o muți (ex. `12`); piesele proprii nu pot avea același progres, deci textul e
//! unic. AI: `AlphaBeta(1)` pe progres total; nu vede zarurile viitoare.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const TRACK: u8 = 40;
const GOAL: u8 = 44;

#[derive(Clone)]
pub struct Ludo {
    prog: [[u8; 4]; 2],
    side: Player,
    roll: u8,
    rng: Rng,
    over: bool,
    log: String,
}

impl Default for Ludo {
    fn default() -> Self {
        Self::new()
    }
}

fn start_off(p: Player) -> u8 {
    match p {
        Player::One => 0,
        Player::Two => 20,
    }
}

fn square(p: Player, progress: u8) -> u8 {
    (start_off(p) + progress - 1) % TRACK
}

impl Ludo {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Ludo {
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

    /// Progresul celor patru piese ale lui `p`.
    pub fn pieces(&self, p: Player) -> [u8; 4] {
        self.prog[p.idx()]
    }

    pub fn current_roll(&self) -> u8 {
        self.roll
    }

    fn has_move(&self) -> bool {
        !self.moves().is_empty()
    }

    fn moves(&self) -> Vec<u8> {
        let mine = &self.prog[self.side.idx()];
        let mut out: Vec<u8> = Vec::new();
        for &pr in mine {
            if out.contains(&pr) || pr == GOAL {
                continue;
            }
            let target = if pr == 0 {
                if self.roll != 6 {
                    continue;
                }
                1
            } else {
                pr + self.roll
            };
            if target > GOAL {
                continue;
            }
            if target < GOAL && mine.contains(&target) {
                continue;
            }
            out.push(pr);
        }
        out.sort_unstable();
        out
    }

    /// Aruncă pentru jucătorul la mutare (cu 3 încercări dacă nu are piese pe tablă);
    /// dacă nu există mutare, pasează și continuă cu celălalt.
    fn begin_turn(&mut self) {
        loop {
            let on_board = self.prog[self.side.idx()]
                .iter()
                .any(|&p| (1..GOAL).contains(&p));
            let tries = if on_board { 1 } else { 3 };
            for _ in 0..tries {
                self.roll = self.rng.d6();
                if self.has_move() {
                    return;
                }
            }
            self.log.push_str(&format!(
                " Jucătorul {} a aruncat {} și nu poate muta.",
                self.side.idx() + 1,
                self.roll
            ));
            self.side = self.side.other();
        }
    }
}

impl Game for Ludo {
    type Move = u8; // progresul piesei mutate (0 = intră din bază)

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
        let target = if *m == 0 { 1 } else { *m + self.roll };
        let k = self.prog[mi].iter().position(|&p| p == *m).expect("piesă");
        self.prog[mi][k] = target;
        self.log = format!(
            "Jucătorul {}: {} cu {}.",
            mi + 1,
            self.format_move(m),
            self.roll
        );
        if target <= TRACK {
            let sq = square(me, target);
            let o = me.other();
            for p in self.prog[o.idx()].iter_mut() {
                if (1..=TRACK).contains(p) && square(o, *p) == sq {
                    *p = 0;
                    self.log.push_str(" Captură!");
                }
            }
        }
        if self.prog[mi].iter().all(|&p| p == GOAL) {
            self.over = true;
            return;
        }
        if self.roll != 6 {
            self.side = me.other();
        } else {
            self.log.push_str(" Șase: aruncă din nou.");
        }
        self.begin_turn();
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            return None;
        }
        let one = self.prog[0].iter().all(|&p| p == GOAL);
        Some(Outcome::Win(if one { Player::One } else { Player::Two }))
    }

    fn evaluate(&self, p: Player) -> i32 {
        let s = |q: Player| -> i32 {
            self.prog[q.idx()]
                .iter()
                .map(|&x| x as i32 + if x == GOAL { 10 } else { 0 })
                .sum()
        };
        s(p) - s(p.other())
    }

    fn render(&self) -> String {
        let mut ring = vec!['.'; TRACK as usize];
        for (pl, ch) in [(Player::One, 'X'), (Player::Two, 'O')] {
            for &p in &self.prog[pl.idx()] {
                if (1..=TRACK).contains(&p) {
                    ring[square(pl, p) as usize] = ch;
                }
            }
        }
        let row = |a: usize, b: usize| ring[a..b].iter().collect::<String>();
        let mut s = String::new();
        s.push_str("Pista (40 pătrate; X pornește la 0, O la 20):\n");
        s.push_str(&format!(
            "   0-19 : {}\n  20-39 : {}\n",
            row(0, 20),
            row(20, 40)
        ));
        for (pl, ch) in [(Player::One, 'X'), (Player::Two, 'O')] {
            let v = self.prog[pl.idx()];
            s.push_str(&format!(
                "Jucător {} ({}): bază {}, terminate {}, progres piese {:?}\n",
                pl.idx() + 1,
                ch,
                v.iter().filter(|&&p| p == 0).count(),
                v.iter().filter(|&&p| p == GOAL).count(),
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
            "intră".to_string()
        } else {
            m.to_string()
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set(g: &mut Ludo, side: Player, roll: u8, one: [u8; 4], two: [u8; 4]) {
        g.prog = [one, two];
        g.side = side;
        g.roll = roll;
        g.over = false;
    }

    #[test]
    fn needs_a_six_to_enter() {
        let mut g = Ludo::with_seed(1);
        set(&mut g, Player::One, 3, [0, 0, 0, 5], [0; 4]);
        assert_eq!(g.legal_moves(), vec![5]);
        set(&mut g, Player::One, 6, [0, 0, 0, 5], [0; 4]);
        assert_eq!(g.legal_moves(), vec![0, 5]);
        let t: Vec<String> = g.legal_moves().iter().map(|m| g.format_move(m)).collect();
        assert_eq!(t, vec!["intră", "5"]);
    }

    #[test]
    fn cannot_land_on_own_piece_or_overshoot() {
        let mut g = Ludo::with_seed(1);
        set(&mut g, Player::One, 4, [10, 14, 42, 44], [0; 4]);
        // 10+4 = 14 ocupat; 14+4=18 ok; 42+4=46 > 44; 44 terminat
        assert_eq!(g.legal_moves(), vec![14]);
        set(&mut g, Player::One, 2, [42, 43, 44, 44], [0; 4]);
        assert_eq!(g.legal_moves(), vec![42]); // 42+2=44; 43+2=45 imposibil
    }

    #[test]
    fn six_cannot_enter_when_start_occupied() {
        let mut g = Ludo::with_seed(1);
        set(&mut g, Player::One, 6, [0, 1, 0, 0], [0; 4]);
        assert_eq!(g.legal_moves(), vec![1]);
    }

    #[test]
    fn capture_sends_opponent_to_base() {
        let mut g = Ludo::with_seed(1);
        // Doi la progres 11 => pătrat (20+10)%40 = 30. Unu la progres 27 => pătrat 26; +4 = 30 (progres 31).
        set(&mut g, Player::One, 4, [27, 0, 0, 0], [11, 0, 0, 0]);
        g.apply(&27);
        assert_eq!(g.prog[1][0], 0);
        assert!(g.prog[0].contains(&31));
        assert!(g.log.contains("Captură"));
    }

    #[test]
    fn home_column_is_safe() {
        let mut g = Ludo::with_seed(1);
        // Unu în coloana de acasă nu e pe pista comună
        set(&mut g, Player::Two, 3, [41, 0, 0, 0], [18, 0, 0, 0]);
        g.apply(&18);
        assert_eq!(g.prog[0][0], 41);
    }

    #[test]
    fn captures_work_across_the_wrap() {
        let mut g = Ludo::with_seed(1);
        // Doi la progres 30 => pătrat (20+29)%40 = 9; Unu la progres 5 => pătrat 4, +5 => 9
        set(&mut g, Player::One, 5, [5, 0, 0, 0], [30, 0, 0, 0]);
        g.apply(&5);
        assert_eq!(g.prog[1][0], 0);
    }

    #[test]
    fn six_gives_extra_roll_to_same_player() {
        for seed in 0..50 {
            let mut g = Ludo::with_seed(seed);
            set(&mut g, Player::One, 6, [0, 0, 0, 0], [0; 4]);
            g.apply(&0);
            assert_eq!(g.side, Player::One, "seed {seed}");
        }
    }

    #[test]
    fn non_six_passes_turn() {
        let mut g = Ludo::with_seed(3);
        set(&mut g, Player::One, 2, [5, 0, 0, 0], [0; 4]);
        g.apply(&5);
        // Doi e în bază: nu poate muta decât cu 6, deci poate pasa înapoi, dar a aruncat
        assert!(g.log.contains("Jucătorul 1: 5 cu 2"));
    }

    #[test]
    fn win_when_all_four_reach_goal() {
        let mut g = Ludo::with_seed(1);
        set(&mut g, Player::Two, 3, [0; 4], [44, 44, 44, 41]);
        g.apply(&41);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
        assert!(g.legal_moves().is_empty());
    }

    #[test]
    fn random_games_finish_with_legal_moves_always_present() {
        for seed in 1..=10u64 {
            let mut g = Ludo::with_seed(seed);
            let mut r = Rng::new(seed + 50);
            let mut n = 0;
            while g.outcome().is_none() && n < 20000 {
                let mv = g.legal_moves();
                assert!(!mv.is_empty(), "{}", g.render());
                g.apply(&mv[r.below(mv.len())]);
                n += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed}");
        }
    }

    #[test]
    fn ai_beats_random_most_of_the_time() {
        let mut wins = 0;
        for seed in 1..=20u64 {
            let mut g = Ludo::with_seed(seed);
            let mut r = Rng::new(seed * 13);
            let mut n = 0;
            while g.outcome().is_none() && n < 20000 {
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
        assert!(wins >= 13, "AI a câștigat doar {wins}/20");
    }
}
