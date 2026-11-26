//! Barbut (variantă simplificată a jocului tradițional românesc cu zaruri), 2 jucători.
//!
//! Simplificări documentate (barbutul real se joacă cu mai mulți pariori și
//! cu „banca”; aici avem un duel, cu reguli de tip „punct”):
//!
//! * Fiecare jucător pornește cu 10 jetoane. Aruncătorul alternează la fiecare rundă.
//! * Faza *Pariu*: aruncătorul alege miza `pariez 1`, `pariez 2` sau `pariez 3`
//!   (limitată de jetoanele ambilor); ambii pun miza în pot.
//! * Faza *Zar de început*: aruncătorul spune `arunc`; cu două zaruri:
//!   7 sau 11 = aruncătorul câștigă potul; 2, 3 sau 12 = îl pierde; altă sumă = „punct”.
//! * Faza *Decizie* (după un punct): adversarul alege `plătesc` (dublează miza,
//!   fiecare adaugă încă o miză, cât permit jetoanele) sau `las` (pierde miza).
//! * Faza *Aruncări*: aruncătorul spune `arunc` până cade din nou punctul
//!   (câștigă potul) sau 7 (pierde potul).
//! * Jocul se termină când un jucător rămâne fără jetoane sau după 40 de runde
//!   (câștigă cine are mai multe jetoane; egalitate = remiză).
//!
//! Verificare surse (nov. 2026): sursele publice românești (antena3.ro „Cum se joacă
//! barbut”, pariurix.com „Jocul de barbut”, newsweek.ro) descriu barbutul ca pe un joc
//! de tip craps/hazard: prima aruncare 7 sau 11 câștigă, 2, 3 sau 12 pierde, altă sumă
//! devine „punct”; apoi se aruncă până cade punctul (câștig) sau 7 (pierdere); acestea
//! coincid cu regulile de aici. Pasul `plătesc`/`las` (dublarea mizei) și limita de
//! 3 jetoane sunt adaosuri ale acestei variante de duel, NU reguli atestate; regulile
//! de masă cu bancă și mai mulți pariori (combinații speciale de „zar”) nu au putut fi
//! confirmate dintr-o sursă publică stabilă, deci au rămas nemodificate.
//!
//! Zarurile se aruncă automat, în starea jocului, cu un `Rng` propriu, imediat
//! după fiecare mutare. În fazele Pariu și Decizie zarurile următoare NU sunt
//! afișate (ar strica decizia); în fazele cu mutarea unică `arunc` se afișează.
//!
//! AI: `AlphaBeta(1)` cu evaluare pe speranța matematică (probabilitatea de a
//! atinge punctul înaintea lui 7); `evaluate` ignoră zarurile deja aruncate, deci
//! AI-ul nu poate „vedea” rezultatul. Determinist: doar seed-ul.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const START_COINS: i32 = 10;
const MAX_ROUNDS: u32 = 40;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Phase {
    Bet,
    Come,
    Decide,
    Roll,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mv {
    Bet(i32),
    Throw,
    Call,
    Fold,
}

#[derive(Clone)]
pub struct Barbut {
    coins: [i32; 2],
    stake: i32,
    shooter: Player,
    phase: Phase,
    point: u8,
    pending: (u8, u8),
    last: Option<(u8, u8)>,
    rounds: u32,
    rng: Rng,
    result: Option<Outcome>,
    log: String,
}

impl Default for Barbut {
    fn default() -> Self {
        Self::new()
    }
}

/// Probabilitatea (în miimi) ca aruncătorul să câștige cu punctul `p` stabilit.
fn q_point(p: u8) -> i32 {
    match p {
        4 | 10 => 333,
        5 | 9 => 400,
        _ => 454, // 6, 8
    }
}

impl Barbut {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let pending = (rng.d6(), rng.d6());
        Barbut {
            coins: [START_COINS; 2],
            stake: 0,
            shooter: Player::One,
            phase: Phase::Bet,
            point: 0,
            pending,
            last: None,
            rounds: 0,
            rng,
            result: None,
            log: String::new(),
        }
    }

    pub fn coins(&self) -> (i32, i32) {
        (self.coins[0], self.coins[1])
    }

    pub fn phase(&self) -> Phase {
        self.phase
    }

    pub fn shooter(&self) -> Player {
        self.shooter
    }

    fn resolve(&mut self, shooter_wins: bool, why: &str) {
        let (w, l) = if shooter_wins {
            (self.shooter, self.shooter.other())
        } else {
            (self.shooter.other(), self.shooter)
        };
        self.coins[w.idx()] += 2 * self.stake;
        self.log.push_str(&format!(
            " {why} Jucătorul {} câștigă {} jetoane de la jucătorul {}.",
            w.idx() + 1,
            self.stake,
            l.idx() + 1
        ));
        self.rounds += 1;
        self.stake = 0;
        self.point = 0;
        self.phase = Phase::Bet;
        self.shooter = self.shooter.other();
        if self.coins[0] == 0 {
            self.result = Some(Outcome::Win(Player::Two));
        } else if self.coins[1] == 0 {
            self.result = Some(Outcome::Win(Player::One));
        } else if self.rounds >= MAX_ROUNDS {
            self.result = Some(match self.coins[0].cmp(&self.coins[1]) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                std::cmp::Ordering::Equal => Outcome::Draw,
            });
        }
    }
}

impl Game for Barbut {
    type Move = Mv;

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Decide => self.shooter.other(),
            _ => self.shooter,
        }
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if self.result.is_some() {
            return Vec::new();
        }
        match self.phase {
            Phase::Bet => {
                let cap = self.coins[0].min(self.coins[1]).min(3);
                (1..=cap).map(Mv::Bet).collect()
            }
            Phase::Come | Phase::Roll => vec![Mv::Throw],
            Phase::Decide => vec![Mv::Call, Mv::Fold],
        }
    }

    fn apply(&mut self, m: &Mv) {
        let dice = self.pending;
        match *m {
            Mv::Bet(k) => {
                self.coins[0] -= k;
                self.coins[1] -= k;
                self.stake = k;
                self.phase = Phase::Come;
                self.log = format!(
                    "Jucătorul {} pariază {k} (în pot: {}).",
                    self.shooter.idx() + 1,
                    2 * k
                );
            }
            Mv::Throw => {
                let sum = dice.0 + dice.1;
                self.last = Some(dice);
                self.log = format!(
                    "Jucătorul {} aruncă {} și {} (= {sum}).",
                    self.shooter.idx() + 1,
                    dice.0,
                    dice.1
                );
                if self.phase == Phase::Come {
                    match sum {
                        7 | 11 => self.resolve(true, "Câștig direct."),
                        2 | 3 | 12 => self.resolve(false, "Barbut! Pierdere directă."),
                        s => {
                            self.point = s;
                            self.phase = Phase::Decide;
                            self.log.push_str(&format!(" Punctul este {s}."));
                        }
                    }
                } else if sum == self.point {
                    self.resolve(true, "Punctul a căzut.");
                } else if sum == 7 {
                    self.resolve(false, "A căzut 7.");
                }
            }
            Mv::Call => {
                let e = self.stake.min(self.coins[0]).min(self.coins[1]);
                self.coins[0] -= e;
                self.coins[1] -= e;
                self.stake += e;
                self.phase = Phase::Roll;
                self.log = format!(
                    "Jucătorul {} plătește (miza e acum {} fiecare).",
                    self.shooter.other().idx() + 1,
                    self.stake
                );
            }
            Mv::Fold => {
                self.log = format!("Jucătorul {} lasă.", self.shooter.other().idx() + 1);
                self.resolve(true, "Adversarul a renunțat.");
            }
        }
        self.pending = (self.rng.d6(), self.rng.d6());
    }

    fn outcome(&self) -> Option<Outcome> {
        self.result
    }

    fn evaluate(&self, p: Player) -> i32 {
        // în miimi de jeton; ignoră `pending`
        let pot = 2 * self.stake;
        let q = match self.phase {
            Phase::Bet => 0,
            Phase::Come => 493,
            _ => q_point(self.point),
        };
        let eff = |pl: Player| -> i32 {
            let own = self.coins[pl.idx()] * 1000;
            if self.phase == Phase::Bet {
                own
            } else if pl == self.shooter {
                own + pot * q
            } else {
                own + pot * (1000 - q)
            }
        };
        eff(p) - eff(p.other())
    }

    fn render(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            "Jetoane: jucător 1 = {}, jucător 2 = {}   Runda {}/{}   Aruncă: jucătorul {}\n",
            self.coins[0],
            self.coins[1],
            self.rounds + 1,
            MAX_ROUNDS,
            self.shooter.idx() + 1
        ));
        if self.result.is_none() {
            let faza = match self.phase {
                Phase::Bet => "pariu (pariez 1/2/3)".to_string(),
                Phase::Come => format!(
                    "zar de început: {} și {}  (scrie «arunc»)",
                    self.pending.0, self.pending.1
                ),
                Phase::Decide => format!(
                    "punct {}; adversarul alege «plătesc» sau «las» (miza {})",
                    self.point, self.stake
                ),
                Phase::Roll => format!(
                    "punct {}, miza {}: zaruri {} și {}  (scrie «arunc»)",
                    self.point, self.stake, self.pending.0, self.pending.1
                ),
            };
            s.push_str(&format!("Faza: {faza}\n"));
        }
        if let Some((a, b)) = self.last {
            s.push_str(&format!("Ultima aruncare: {a} și {b}\n"));
        }
        s.push_str(self.log.trim());
        s.push('\n');
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        match m {
            Mv::Bet(k) => format!("pariez {k}"),
            Mv::Throw => "arunc".to_string(),
            Mv::Call => "plătesc".to_string(),
            Mv::Fold => "las".to_string(),
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

    fn texts(g: &Barbut) -> Vec<String> {
        GameObj::legal_moves(g)
    }

    #[test]
    fn starts_with_bets() {
        let g = Barbut::with_seed(1);
        assert_eq!(texts(&g), vec!["pariez 1", "pariez 2", "pariez 3"]);
        assert_eq!(Game::to_move(&g), Player::One);
    }

    #[test]
    fn bet_limited_by_coins() {
        let mut g = Barbut::with_seed(1);
        g.coins = [10, 2];
        assert_eq!(texts(&g), vec!["pariez 1", "pariez 2"]);
    }

    #[test]
    fn natural_win_and_craps_loss() {
        let mut g = Barbut::with_seed(1);
        Game::apply(&mut g, &Mv::Bet(2));
        g.pending = (3, 4);
        Game::apply(&mut g, &Mv::Throw);
        assert_eq!(g.coins(), (12, 8));
        assert_eq!(g.phase(), Phase::Bet);
        assert_eq!(g.shooter(), Player::Two, "aruncătorul alternează");
        // acum Doi aruncă și pierde cu 1+1
        Game::apply(&mut g, &Mv::Bet(1));
        g.pending = (1, 1);
        Game::apply(&mut g, &Mv::Throw);
        assert_eq!(g.coins(), (13, 7));
    }

    #[test]
    fn point_then_decision_by_other_player() {
        let mut g = Barbut::with_seed(1);
        Game::apply(&mut g, &Mv::Bet(1));
        g.pending = (2, 4);
        Game::apply(&mut g, &Mv::Throw);
        assert_eq!(g.phase(), Phase::Decide);
        assert_eq!(g.point, 6);
        assert_eq!(Game::to_move(&g), Player::Two);
        assert_eq!(texts(&g), vec!["plătesc", "las"]);
        let mut f = g.clone();
        Game::apply(&mut f, &Mv::Fold);
        assert_eq!(f.coins(), (11, 9));
        Game::apply(&mut g, &Mv::Call);
        assert_eq!(g.phase(), Phase::Roll);
        assert_eq!(Game::to_move(&g), Player::One);
        assert_eq!(g.coins(), (8, 8));
        assert_eq!(g.stake, 2);
        // cade 7: aruncătorul pierde potul de 4
        g.pending = (3, 4);
        Game::apply(&mut g, &Mv::Throw);
        assert_eq!(g.coins(), (8, 12));
    }

    #[test]
    fn point_wins_and_other_sums_continue() {
        let mut g = Barbut::with_seed(1);
        Game::apply(&mut g, &Mv::Bet(1));
        g.pending = (5, 5);
        Game::apply(&mut g, &Mv::Throw);
        Game::apply(&mut g, &Mv::Call);
        g.pending = (1, 2);
        Game::apply(&mut g, &Mv::Throw);
        assert_eq!(g.phase(), Phase::Roll, "continuă");
        g.pending = (4, 6);
        Game::apply(&mut g, &Mv::Throw);
        assert_eq!(g.coins(), (12, 8));
    }

    #[test]
    fn game_ends_when_a_player_is_broke() {
        let mut g = Barbut::with_seed(1);
        g.coins = [1, 5];
        Game::apply(&mut g, &Mv::Bet(1));
        g.pending = (1, 2); // pierde direct
        Game::apply(&mut g, &Mv::Throw);
        assert_eq!(Game::outcome(&g), Some(Outcome::Win(Player::Two)));
        assert!(Game::legal_moves(&g).is_empty());
    }

    #[test]
    fn hidden_dice_in_bet_and_decide_phases() {
        let mut g = Barbut::with_seed(1);
        g.pending = (6, 5);
        assert!(!Game::render(&g).contains("6 și 5"));
        Game::apply(&mut g, &Mv::Bet(1));
        g.pending = (6, 5);
        assert!(Game::render(&g).contains("6 și 5"));
    }

    #[test]
    fn ai_prefers_paying_and_smallest_bet() {
        let mut g = Barbut::with_seed(4);
        let m = g.ai_move(1).unwrap();
        assert_eq!(m, "pariez 1");
        g.play("pariez 1").unwrap();
        g.pending = (2, 4);
        g.play("arunc").unwrap();
        assert_eq!(g.ai_move(1).unwrap(), "plătesc");
    }

    #[test]
    fn random_games_finish() {
        for seed in 1..=30u64 {
            let mut g = Barbut::with_seed(seed);
            let mut r = Rng::new(seed + 9);
            let mut n = 0;
            while Game::outcome(&g).is_none() && n < 10_000 {
                let mv = Game::legal_moves(&g);
                assert!(!mv.is_empty());
                Game::apply(&mut g, &mv[r.below(mv.len())]);
                assert!(g.coins[0] >= 0 && g.coins[1] >= 0);
                assert_eq!(g.coins[0] + g.coins[1] + 2 * g.stake, 2 * START_COINS);
                n += 1;
            }
            assert!(Game::outcome(&g).is_some(), "seed {seed}");
        }
    }
}
