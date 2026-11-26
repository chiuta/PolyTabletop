//! Farul Fantomei — deducție cooperativă cu indicii vizuale (variantă originală, fantomă AI).
//!
//! Pe o stâncă bate vântul și un far veghează o crimă veche. Tu ești **mediumul** (Jucătorul 1);
//! partenerul AI este **fantoma** (Jucătorul 2) și știe ce s-a întâmplat, dar nu poate vorbi: îți
//! trimite doar „viziuni” — cărți cu câte 3 simboluri din banca noastră de 24 de simboluri.
//! Mecanica „fantoma dă indicii vizuale, medium-ul ghicește” este generică; simbolurile, cărțile
//! și textele sunt ale noastre.
//!
//! * Patru etape: portretul (6 candidați), camera (6), obiectul (6) și, la final, scena (3
//!   candidați, cu o singură viziune și o singură ghicire). Fiecare candidat e o carte cu 5 simboluri.
//! * Fantoma are în mână 7 viziuni (3 simboluri); alege una: `viziune 3`. Tu vezi simbolurile
//!   viziunii și le potrivești cu candidații; ghicești `ghic 4` sau aștepți următoarea viziune
//!   `asteapt`. O ghicire greșită scoate candidatul din joc; una corectă trece la etapa următoare.
//! * Timp: cel mult 10 viziuni în primele trei etape. Verificarea ghicirii e o mutare forțată
//!   (`verific`), ca AI-ul medium să nu „vadă” răspunsul înainte de a alege.
//!
//! Rezultat: scena finală ghicită corect = echipa câștigă („Câștigă Jucătorul 1”); orice eșec
//! (timp epuizat, ghicire finală greșită) = REMIZĂ (documentat: nu există adversar).
//! Încheierea e o mutare forțată (`sfârșit`).
//!
//! Informație ascunsă: `render` arată doar ce vede mediumul (candidați, viziuni date), nu soluția.
//! AI-ul medium (când joacă pentru J1) alege din viziuni și din candidați, fără soluție; fantoma
//! își cunoaște în mod legitim ținta și alege viziunea care o evidențiază cel mai bine.
#![allow(clippy::needless_range_loop)]

use crate::cards::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const SYMBOLS: [&str; 24] = [
    "luna",
    "soare",
    "stea",
    "umbrela",
    "turn",
    "ancora",
    "foarfeca",
    "nota",
    "floare",
    "craniu",
    "scrisoare",
    "clepsidra",
    "coroana",
    "sabie",
    "aburi",
    "nor",
    "steag",
    "trifoi",
    "balanta",
    "cheie",
    "lampa",
    "ochi",
    "barca",
    "clopot",
];
pub const STAGE_NAMES: [&str; 4] = ["Portretul", "Camera", "Obiectul", "Scena"];
pub const CAND_NAMES: [[&str; 6]; 4] = [
    [
        "Paznicul",
        "Văduva",
        "Pictorul",
        "Croitoreasa",
        "Marinarul",
        "Bibliotecara",
    ],
    [
        "Felinarul",
        "Subsolul",
        "Podul",
        "Salonul",
        "Dormitorul",
        "Observatorul",
    ],
    [
        "Telescopul",
        "Registrul",
        "Busola",
        "Ceainicul",
        "Harponul",
        "Vioara",
    ],
    ["Furtuna", "Naufragiul", "Întâlnirea", "", "", ""],
];
pub const CLUE_LIMIT: u32 = 10;
const HAND: usize = 7;
const CANDS: [usize; 4] = [6, 6, 6, 3];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Vision(u8),
    Guess(u8),
    Wait,
    Verify,
    Finish,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Clue,
    Medium,
    Verify,
}

#[derive(Clone)]
pub struct Farul {
    /// Whose private info `render` shows (`None` = the mover's, hot-seat).
    viewer: Option<Player>,
    cands: Vec<Vec<u32>>,
    target: [usize; 4],
    dead: Vec<bool>,
    clues: Vec<u32>,
    hand: Vec<u32>,
    deck: Vec<u32>,
    stage: usize,
    rounds: u32,
    guess: Option<usize>,
    phase: Phase,
    pending: Option<bool>,
    over: bool,
    won: bool,
    rng: Rng,
    log: Vec<String>,
}

impl Default for Farul {
    fn default() -> Self {
        Self::new()
    }
}

fn names(mask: u32) -> String {
    (0..24)
        .filter(|i| mask & (1 << i) != 0)
        .map(|i| SYMBOLS[i])
        .collect::<Vec<_>>()
        .join(", ")
}

fn random_set(rng: &mut Rng, k: usize) -> u32 {
    let mut idx: Vec<usize> = (0..24).collect();
    shuffle(&mut idx, rng);
    idx[..k].iter().fold(0u32, |m, i| m | (1 << i))
}

impl Farul {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut cands = Vec::new();
        let mut target = [0usize; 4];
        for s in 0..4 {
            let mut v: Vec<u32> = Vec::new();
            while v.len() < CANDS[s] {
                let m = random_set(&mut rng, 5);
                if !v.contains(&m) {
                    v.push(m);
                }
            }
            cands.push(v);
            target[s] = rng.below(CANDS[s]);
        }
        let mut deck: Vec<u32> = (0..60).map(|_| random_set(&mut rng, 3)).collect();
        shuffle(&mut deck, &mut rng);
        let hand: Vec<u32> = (0..HAND).map(|_| deck.pop().unwrap()).collect();
        Farul {
            viewer: None,
            cands,
            target,
            dead: vec![false; CANDS[0]],
            clues: Vec::new(),
            hand,
            deck,
            stage: 0,
            rounds: 0,
            guess: None,
            phase: Phase::Clue,
            pending: None,
            over: false,
            won: false,
            rng,
            log: Vec::new(),
        }
    }

    pub fn stage(&self) -> usize {
        self.stage
    }

    pub fn rounds(&self) -> u32 {
        self.rounds
    }

    pub fn is_won(&self) -> bool {
        self.over && self.won
    }

    /// Doar pentru teste: ținta etapei curente.
    pub fn current_target(&self) -> usize {
        self.target[self.stage]
    }

    fn alive(&self) -> Vec<usize> {
        (0..self.dead.len()).filter(|&i| !self.dead[i]).collect()
    }

    fn raw_scores(&self) -> Vec<i32> {
        let n = self.cands[self.stage].len();
        (0..n)
            .map(|k| {
                self.clues
                    .iter()
                    .map(|c| (c & self.cands[self.stage][k]).count_ones() as i32)
                    .sum()
            })
            .collect()
    }

    /// Scorul unei ghiciri din punctul de vedere al mediumului (fără soluție).
    fn guess_value(&self, k: usize) -> i32 {
        let raw = self.raw_scores();
        let alive = self.alive();
        let mean: f64 = alive.iter().map(|&i| f64::from(raw[i])).sum::<f64>() / alive.len() as f64;
        let second = alive
            .iter()
            .filter(|&&i| i != k)
            .map(|&i| raw[i])
            .max()
            .unwrap_or(0);
        (10.0 * (f64::from(raw[k]) - mean)) as i32 + 4 * (raw[k] - second)
    }

    fn vision_value(&self, v: u32) -> i32 {
        let t = self.cands[self.stage][self.target[self.stage]];
        let mut s = 14 * (v & t).count_ones() as i32;
        let mut mx = 0;
        let mut sum = 0;
        for &i in &self.alive() {
            if i == self.target[self.stage] {
                continue;
            }
            let o = (v & self.cands[self.stage][i]).count_ones() as i32;
            mx = mx.max(o);
            sum += o;
        }
        s -= 5 * mx + sum;
        s
    }

    fn set_end(&mut self, won: bool) {
        if self.pending.is_none() {
            self.pending = Some(won);
        }
    }

    fn refill(&mut self) {
        if self.deck.is_empty() {
            self.deck = (0..60).map(|_| random_set(&mut self.rng, 3)).collect();
        }
        if let Some(c) = self.deck.pop() {
            self.hand.push(c);
        }
    }
}

impl Farul {
    /// Normal rendering (human = J1's point of view).
    fn render_main(&self) -> String {
        let mut s = String::new();
        if self.over || self.pending.is_some() {
            s.push_str("Farul Fantomei — încheiat. Soluția: ");
            let sol: Vec<String> = (0..4)
                .map(|st| CAND_NAMES[st][self.target[st]].to_string())
                .collect();
            s.push_str(&sol.join(" / "));
            s.push('\n');
        }
        s.push_str(&format!(
            "Farul Fantomei — etapa {}/4: {} | viziuni date {}/{}\n",
            self.stage + 1,
            STAGE_NAMES[self.stage],
            self.rounds,
            CLUE_LIMIT
        ));
        s.push_str("Candidați (5 simboluri fiecare):\n");
        for (i, c) in self.cands[self.stage].iter().enumerate() {
            s.push_str(&format!(
                "  {}{}. {} — {}\n",
                if self.dead[i] { "x " } else { "  " },
                i + 1,
                CAND_NAMES[self.stage][i],
                names(*c)
            ));
        }
        if self.clues.is_empty() {
            s.push_str("Viziuni în această etapă: —\n");
        } else {
            s.push_str("Viziuni în această etapă:\n");
            for (i, c) in self.clues.iter().enumerate() {
                s.push_str(&format!("  {}) {}\n", i + 1, names(*c)));
            }
        }
        for l in self
            .log
            .iter()
            .rev()
            .take(3)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
        {
            s.push_str(&format!("» {l}\n"));
        }
        if self.pending.is_some() && !self.over {
            s.push_str("Jocul se încheie: sfârșit.\n");
        } else if self.over {
            if self.won {
                s.push_str("VICTORIE: misterul farului e rezolvat.\n");
            } else {
                s.push_str("ÎNFRÂNGERE (remiză): fantoma tace pentru totdeauna.\n");
            }
        } else {
            match self.phase {
                Phase::Clue => s.push_str("Fantoma alege o viziune (J2).\n"),
                Phase::Medium => s.push_str("Medium (J1): ghic N sau asteapt.\n"),
                Phase::Verify => s.push_str("Se verifică ghicirea: verific.\n"),
            }
        }
        s
    }

    /// Reveal-all rendering, appended when `set_viewer(Some(Two))` (second human inspects).
    fn god_view(&self) -> String {
        let sol: Vec<String> = (0..4)
            .map(|st| CAND_NAMES[st][self.target[st]].to_string())
            .collect();
        format!(
            "--- VEDERE TOTALĂ (J2): soluția ascunsă: {} ---\n",
            sol.join(" / ")
        )
    }
}

impl Game for Farul {
    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }
    type Move = Move;

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Clue | Phase::Verify => Player::Two,
            Phase::Medium => Player::One,
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over {
            return Vec::new();
        }
        if self.pending.is_some() {
            return vec![Move::Finish];
        }
        match self.phase {
            Phase::Clue => (0..self.hand.len() as u8).map(Move::Vision).collect(),
            Phase::Verify => vec![Move::Verify],
            Phase::Medium => {
                let mut v: Vec<Move> = self.alive().iter().map(|&i| Move::Guess(i as u8)).collect();
                if self.stage < 3 && self.rounds < CLUE_LIMIT {
                    v.push(Move::Wait);
                }
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        match *m {
            Move::Vision(i) => {
                let v = self.hand.remove(i as usize);
                self.clues.push(v);
                self.rounds += 1;
                self.log.push(format!(
                    "Viziune ({}): {}",
                    STAGE_NAMES[self.stage],
                    names(v)
                ));
                self.refill();
                self.phase = Phase::Medium;
            }
            Move::Wait => {
                self.log.push("Mediumul așteaptă o nouă viziune.".into());
                self.phase = Phase::Clue;
            }
            Move::Guess(k) => {
                self.guess = Some(k as usize);
                self.phase = Phase::Verify;
            }
            Move::Verify => {
                let k = self.guess.take().expect("ghicire în așteptare");
                let name = CAND_NAMES[self.stage][k];
                if k == self.target[self.stage] {
                    self.log
                        .push(format!("Corect: {} ({}).", name, STAGE_NAMES[self.stage]));
                    if self.stage == 3 {
                        self.set_end(true);
                        return;
                    }
                    self.stage += 1;
                    self.dead = vec![false; CANDS[self.stage]];
                    self.clues.clear();
                    self.phase = Phase::Clue;
                } else {
                    self.log.push(format!("Greșit: {} nu e răspunsul.", name));
                    self.dead[k] = true;
                    if self.stage == 3 || self.rounds >= CLUE_LIMIT {
                        self.set_end(false);
                        return;
                    }
                    self.phase = Phase::Clue;
                }
            }
            Move::Finish => {
                self.over = true;
                self.won = self.pending == Some(true);
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            None
        } else if self.won {
            Some(Outcome::Win(Player::One))
        } else {
            Some(Outcome::Draw)
        }
    }

    fn evaluate(&self, p: Player) -> i32 {
        if let Some(w) = self.pending {
            return if w { 100_000 } else { -100_000 };
        }
        match p {
            Player::Two => {
                // Fantoma își cunoaște ținta: valoarea ultimei viziuni date.
                match self.clues.last() {
                    Some(&v) if self.phase == Phase::Medium => self.vision_value(v),
                    _ => 0,
                }
            }
            Player::One => match (self.phase, self.guess) {
                (Phase::Verify, Some(k)) => self.guess_value(k),
                _ => 7,
            },
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }

    fn render(&self) -> String {
        let mut s = self.render_main();
        if self.viewer == Some(Player::Two) {
            s.push_str(&self.god_view());
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Vision(i) => format!("viziune {}", i + 1),
            Move::Guess(k) => format!("ghic {}", k + 1),
            Move::Wait => "asteapt".into(),
            Move::Verify => "verific".into(),
            Move::Finish => "sfârșit".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ai_game(seed: u64) -> Farul {
        let mut g = Farul::with_seed(seed);
        let mut rng = Rng::new(seed ^ 3);
        let mut n = 0;
        while g.outcome().is_none() {
            let m = polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap();
            g.apply(&m);
            n += 1;
            assert!(n < 500);
        }
        g
    }

    #[test]
    fn setup_is_valid() {
        let g = Farul::with_seed(1);
        for s in 0..4 {
            assert_eq!(g.cands[s].len(), CANDS[s]);
            for c in &g.cands[s] {
                assert_eq!(c.count_ones(), 5);
            }
        }
        assert_eq!(g.hand.len(), HAND);
        for v in &g.hand {
            assert_eq!(v.count_ones(), 3);
        }
        assert_eq!(g.to_move(), Player::Two);
    }

    #[test]
    fn render_hides_solution_while_playing() {
        let g = Farul::with_seed(2);
        let r = g.render();
        assert!(!r.contains("Soluția"));
        assert!(r.contains("Candidați"));
    }

    #[test]
    fn correct_guess_advances_wrong_kills() {
        let mut g = Farul::with_seed(3);
        g.apply(&Move::Vision(0));
        assert_eq!(g.to_move(), Player::One);
        let t = g.current_target();
        let wrong = (0..6).find(|&k| k != t).unwrap();
        g.apply(&Move::Guess(wrong as u8));
        assert_eq!(g.legal_moves(), vec![Move::Verify]);
        g.apply(&Move::Verify);
        assert!(g.dead[wrong]);
        assert_eq!(g.stage, 0);
        g.apply(&Move::Vision(0));
        assert!(!g.legal_moves().contains(&Move::Guess(wrong as u8)));
        g.apply(&Move::Guess(t as u8));
        g.apply(&Move::Verify);
        assert_eq!(g.stage, 1);
        assert!(g.clues.is_empty());
    }

    #[test]
    fn final_stage_win_and_loss() {
        let mut g = Farul::with_seed(4);
        g.stage = 3;
        g.dead = vec![false; 3];
        g.apply(&Move::Vision(0));
        assert!(!g.legal_moves().contains(&Move::Wait));
        let t = g.current_target();
        g.apply(&Move::Guess(t as u8));
        g.apply(&Move::Verify);
        assert_eq!(g.legal_moves(), vec![Move::Finish]);
        g.apply(&Move::Finish);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));

        let mut g = Farul::with_seed(4);
        g.stage = 3;
        g.dead = vec![false; 3];
        g.apply(&Move::Vision(0));
        let t = g.current_target();
        g.apply(&Move::Guess(((t + 1) % 3) as u8));
        g.apply(&Move::Verify);
        g.apply(&Move::Finish);
        assert_eq!(g.outcome(), Some(Outcome::Draw));
    }

    #[test]
    fn time_limit_ends_game() {
        let mut g = Farul::with_seed(5);
        for _ in 0..CLUE_LIMIT {
            g.apply(&Move::Vision(0));
            if g.rounds < CLUE_LIMIT {
                g.apply(&Move::Wait);
            }
        }
        assert!(!g.legal_moves().contains(&Move::Wait));
        let t = g.current_target();
        let wrong = (t + 1) % 6;
        g.apply(&Move::Guess(wrong as u8));
        g.apply(&Move::Verify);
        assert_eq!(g.legal_moves(), vec![Move::Finish]);
    }

    #[test]
    fn hand_is_refilled() {
        let mut g = Farul::with_seed(6);
        let n = g.hand.len() + g.deck.len();
        g.apply(&Move::Vision(2));
        assert_eq!(g.hand.len(), HAND);
        assert_eq!(g.hand.len() + g.deck.len() + g.clues.len(), n);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=60u64 {
            let mut g = Farul::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                let m = mv[rng.below(mv.len())];
                g.apply(&m);
                n += 1;
                assert!(n < 1000);
            }
        }
    }

    #[test]
    fn ai_pair_wins_often() {
        let mut wins = 0;
        for seed in 1..=60u64 {
            if ai_game(seed).is_won() {
                wins += 1;
            }
        }
        eprintln!("farul: victorii {wins}/60");
        assert!(wins >= 12, "{wins}/60");
    }
}
