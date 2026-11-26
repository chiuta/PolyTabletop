//! Șerpi și scări, 2 jucători, tablă de 100 de pătrate.
//!
//! Zarul se aruncă automat în starea jocului (cu un `Rng` propriu) imediat după
//! fiecare mutare și este vizibil în `render()`. Singura mutare este textul
//! `arunc`, care avansează pionul cu valoarea zarului curent. Ținta (100) se
//! atinge exact: dacă zarul depășește 100, pionul rămâne pe loc. Capătul unei
//! scări urcă pionul, capul unui șarpe îl coboară. Fără aruncare suplimentară
//! la 6.
//!
//! Determinism: întregul joc depinde doar de seed; AI-ul nu are alegeri reale
//! (o singură mutare), deci nu poate „vedea” zaruri viitoare.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

/// (început, sfârșit): scări (sfârșit > început) și șerpi (sfârșit < început).
pub const JUMPS: [(u8, u8); 19] = [
    (1, 38),
    (4, 14),
    (9, 31),
    (21, 42),
    (28, 84),
    (36, 44),
    (51, 67),
    (71, 91),
    (80, 100),
    (16, 6),
    (47, 26),
    (49, 11),
    (56, 53),
    (62, 19),
    (64, 60),
    (87, 24),
    (93, 73),
    (95, 75),
    (98, 78),
];

pub fn jump_from(sq: u8) -> Option<u8> {
    JUMPS.iter().find(|j| j.0 == sq).map(|j| j.1)
}

#[derive(Clone)]
pub struct Snakes {
    pos: [u8; 2],
    side: Player,
    roll: u8,
    rng: Rng,
    over: bool,
    log: String,
}

impl Default for Snakes {
    fn default() -> Self {
        Self::new()
    }
}

impl Snakes {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let roll = rng.d6();
        Snakes {
            pos: [0; 2],
            side: Player::One,
            roll,
            rng,
            over: false,
            log: String::new(),
        }
    }

    pub fn positions(&self) -> (u8, u8) {
        (self.pos[0], self.pos[1])
    }

    pub fn current_roll(&self) -> u8 {
        self.roll
    }
}

impl Game for Snakes {
    type Move = ();

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<()> {
        if self.over {
            Vec::new()
        } else {
            vec![()]
        }
    }

    fn apply(&mut self, _m: &()) {
        let i = self.side.idx();
        let np = self.pos[i] + self.roll;
        self.log = format!("Jucătorul {}: zar {}", i + 1, self.roll);
        if np <= 100 {
            self.pos[i] = np;
            if let Some(t) = jump_from(np) {
                self.log.push_str(&format!(
                    ", {} {} -> {}",
                    if t > np { "scară" } else { "șarpe" },
                    np,
                    t
                ));
                self.pos[i] = t;
            } else {
                self.log.push_str(&format!(", ajunge pe {np}"));
            }
        } else {
            self.log.push_str(", depășește 100 și rămâne pe loc");
        }
        self.log.push('.');
        if self.pos[i] == 100 {
            self.over = true;
            return;
        }
        self.side = self.side.other();
        self.roll = self.rng.d6();
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.pos[0] == 100 {
            Some(Outcome::Win(Player::One))
        } else if self.pos[1] == 100 {
            Some(Outcome::Win(Player::Two))
        } else {
            None
        }
    }

    fn evaluate(&self, p: Player) -> i32 {
        self.pos[p.idx()] as i32 - self.pos[p.other().idx()] as i32
    }

    fn render(&self) -> String {
        let mut s = String::new();
        s.push_str("Scări: ");
        for j in JUMPS.iter().filter(|j| j.1 > j.0) {
            s.push_str(&format!("{}→{} ", j.0, j.1));
        }
        s.push_str("\nȘerpi: ");
        for j in JUMPS.iter().filter(|j| j.1 < j.0) {
            s.push_str(&format!("{}→{} ", j.0, j.1));
        }
        s.push_str(&format!(
            "\nJucător 1 (X): {}   Jucător 2 (O): {}\n",
            self.pos[0], self.pos[1]
        ));
        if !self.over {
            s.push_str(&format!(
                "La mutare: jucătorul {}   Zar: {}   (scrie «arunc»)\n",
                self.side.idx() + 1,
                self.roll
            ));
        }
        s.push_str(self.log.trim());
        s.push('\n');
        s
    }

    fn format_move(&self, _m: &()) -> String {
        "arunc".to_string()
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jump_table_is_sane() {
        for j in JUMPS {
            assert!((1..100).contains(&j.0));
            assert!((1..=100).contains(&j.1));
            assert_ne!(j.0, j.1);
        }
        // niciun început duplicat și niciun capăt nu e început (fără lanțuri)
        let mut starts: Vec<u8> = JUMPS.iter().map(|j| j.0).collect();
        starts.sort_unstable();
        starts.dedup();
        assert_eq!(starts.len(), JUMPS.len());
        for j in JUMPS {
            assert!(jump_from(j.1).is_none(), "lanț la {}", j.1);
        }
    }

    #[test]
    fn single_move_is_arunc() {
        let g = Snakes::with_seed(1);
        use polytabletop_core::GameObj;
        assert_eq!(GameObj::legal_moves(&g), vec!["arunc".to_string()]);
    }

    #[test]
    fn ladder_climbs_and_snake_slides() {
        let mut g = Snakes::with_seed(1);
        g.pos[0] = 0;
        g.roll = 4; // 4 -> 14
        g.apply(&());
        assert_eq!(g.pos[0], 14);
        let mut g = Snakes::with_seed(1);
        g.pos[0] = 12;
        g.roll = 4; // 16 -> 6
        g.apply(&());
        assert_eq!(g.pos[0], 6);
    }

    #[test]
    fn exact_finish_required() {
        let mut g = Snakes::with_seed(1);
        g.pos[0] = 97;
        g.roll = 4;
        g.apply(&());
        assert_eq!(g.pos[0], 97);
        assert!(g.outcome().is_none());
        assert_eq!(g.side, Player::Two);
        g.side = Player::One;
        g.roll = 3;
        g.apply(&());
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
        assert!(g.legal_moves().is_empty());
    }

    #[test]
    fn ladder_to_100_wins() {
        let mut g = Snakes::with_seed(1);
        g.pos[1] = 75;
        g.side = Player::Two;
        g.roll = 5; // 80 -> 100
        g.apply(&());
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
    }

    #[test]
    fn turns_alternate_and_games_finish() {
        for seed in 1..=20u64 {
            let mut g = Snakes::with_seed(seed);
            let mut n = 0;
            while g.outcome().is_none() && n < 100_000 {
                let before = g.side;
                g.apply(&());
                if g.outcome().is_none() {
                    assert_eq!(g.side, before.other());
                }
                n += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed}");
        }
    }

    #[test]
    fn same_seed_same_result() {
        let run = |seed| {
            let mut g = Snakes::with_seed(seed);
            while g.outcome().is_none() {
                g.apply(&());
            }
            (g.outcome(), g.positions())
        };
        assert_eq!(run(9), run(9));
    }
}
