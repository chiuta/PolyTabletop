//! SPIONII CUVINTELOR — variantă originală de joc de spionaj cu cuvinte (două echipe,
//! grilă 5×5, agenți, un asasin).
//!
//! Reguli folosite (mecanica publică a genului, cuvintele și temele sunt ale noastre):
//! * Tablă de 25 de cuvinte. Echipa Jucătorului 1 (roșii) are 9 agenți, echipa
//!   Jucătorului 2 (albaștri) 8; 7 cuvinte sunt neutre, unul este **asasinul**.
//! * Maestrul-spion al echipei este calculatorul: dă un indiciu din tabela de teme
//!   (`spionii_date`) — numele unei teme — și un număr. El alege tema care acoperă cât
//!   mai mulți dintre agenții săi nedescoperiți și cât mai puțin din ceilalți; nu
//!   folosește niciodată o temă care conține asasinul decât dacă nu are alternativă.
//! * Jucătorul de la mutare este ghicitorul echipei: `ghici cuvânt` (fără diacritice),
//!   apoi pasul forțat `dezvaluie` întoarce cartea. Poți ghici cel mult „număr + 1”
//!   cuvinte; `stop` încheie tura (după cel puțin o ghicire). Cuvânt al echipei tale
//!   → continui; neutru → tura trece; al adversarului → el e mai aproape de victorie
//!   și tura trece; asasin → pierzi imediat. Câștigă cine își descoperă toți agenții.
//!
//! AI-ul (ca ghicitor) folosește aceeași tabelă de teme ca maestrul-spion (un „cod”
//! comun); nu vede cheia: valoarea unei mutări depinde doar de indiciu și de cuvinte.

use crate::w4_comun::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::fmt::Write;
use std::sync::OnceLock;

use crate::cards::shuffle;
use crate::spionii_date::THEMES;

pub struct Theme {
    pub clue: String,
    pub members: Vec<usize>,
}

pub struct Lex {
    pub themes: Vec<Theme>,
    /// (afișat, fără diacritice)
    pub pool: Vec<(String, String)>,
    pub word_themes: Vec<Vec<usize>>,
}

pub fn lex() -> &'static Lex {
    static CELL: OnceLock<Lex> = OnceLock::new();
    CELL.get_or_init(|| {
        let mut pool: Vec<(String, String)> = Vec::new();
        let mut themes = Vec::new();
        for (clue, words) in THEMES {
            let mut members = Vec::new();
            for w in words.split_whitespace() {
                let w = w.to_lowercase();
                let f = fold(&w);
                let idx = match pool.iter().position(|(_, ff)| *ff == f) {
                    Some(i) => i,
                    None => {
                        pool.push((w, f));
                        pool.len() - 1
                    }
                };
                if !members.contains(&idx) {
                    members.push(idx);
                }
            }
            themes.push(Theme {
                clue: clue.to_string(),
                members,
            });
        }
        let mut word_themes = vec![Vec::new(); pool.len()];
        for (ti, t) in themes.iter().enumerate() {
            for &m in &t.members {
                word_themes[m].push(ti);
            }
        }
        Lex {
            themes,
            pool,
            word_themes,
        }
    })
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Clue,
    Guess(usize),
    Stop,
    Reveal,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    NeedClue,
    Guess,
    Pending,
    Over,
}

const NEUTRAL: u8 = 2;
const ASSASSIN: u8 = 3;

#[derive(Clone)]
pub struct Spionii {
    rng: Rng,
    board: [usize; 25],
    key: [u8; 25],
    rev: [bool; 25],
    side: Player,
    phase: Phase,
    clue: Option<(usize, usize)>,
    made: usize,
    rem: [i32; 2],
    winner: Option<Player>,
    used: Vec<usize>,
    pending: usize,
    heur: Heur,
    log: Vec<String>,
}

impl Default for Spionii {
    fn default() -> Self {
        Self::new()
    }
}

impl Spionii {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let l = lex();
        let mut rng = Rng::new(seed);
        let mut idx: Vec<usize> = (0..l.pool.len()).collect();
        shuffle(&mut idx, &mut rng);
        let mut board = [0usize; 25];
        board.copy_from_slice(&idx[..25]);
        let mut key = [0u8; 25];
        for (i, k) in key.iter_mut().enumerate() {
            *k = match i {
                0..=8 => 0,
                9..=16 => 1,
                17..=23 => NEUTRAL,
                _ => ASSASSIN,
            };
        }
        shuffle(&mut key, &mut rng);
        Spionii {
            rng,
            board,
            key,
            rev: [false; 25],
            side: Player::One,
            phase: Phase::NeedClue,
            clue: None,
            made: 0,
            rem: [9, 8],
            winner: None,
            used: Vec::new(),
            pending: 0,
            heur: Heur::default(),
            log: Vec::new(),
        }
    }

    pub fn remaining(&self) -> (i32, i32) {
        (self.rem[0], self.rem[1])
    }

    fn in_theme(&self, cell: usize, theme: usize) -> bool {
        lex().themes[theme].members.contains(&self.board[cell])
    }

    fn clue_is_legal(&self, theme: usize) -> bool {
        let c = fold(&lex().themes[theme].clue);
        self.board.iter().all(|&b| {
            let w = &lex().pool[b].1;
            *w != c && !(c.len() >= 4 && (w.contains(&c) || c.contains(w.as_str())))
        })
    }

    /// Maestrul-spion al echipei `side`: alege tema indiciului din cheia secretă.
    fn spymaster(&mut self) -> (usize, usize) {
        let l = lex();
        let me = self.side.idx() as u8;
        for pass in 0..3 {
            let mut best: Option<(usize, usize)> = None;
            let mut best_s = i32::MIN;
            for ti in 0..l.themes.len() {
                if pass == 0 && self.used.contains(&ti) {
                    continue;
                }
                if pass < 2 && !self.clue_is_legal(ti) {
                    continue;
                }
                let (mut own, mut enemy, mut neutral, mut ass) = (0, 0, 0, 0);
                for i in 0..25 {
                    if self.rev[i] || !self.in_theme(i, ti) {
                        continue;
                    }
                    match self.key[i] {
                        k if k == me => own += 1,
                        ASSASSIN => ass += 1,
                        NEUTRAL => neutral += 1,
                        _ => enemy += 1,
                    }
                }
                if own == 0 {
                    continue;
                }
                let s =
                    100 * own - 70 * enemy - 25 * neutral - 400 * ass + self.rng.below(20) as i32;
                if s > best_s {
                    best_s = s;
                    best = Some((ti, (own as usize).min(4)));
                }
            }
            if let Some(b) = best {
                return b;
            }
        }
        (0, 1)
    }

    fn end_turn(&mut self) {
        self.side = self.side.other();
        self.clue = None;
        self.made = 0;
        self.phase = Phase::NeedClue;
    }

    fn cell_text(&self, i: usize) -> String {
        let w = lex().pool[self.board[i]].0.to_uppercase();
        if self.rev[i] || self.phase == Phase::Over {
            let m = match self.key[i] {
                0 => "R",
                1 => "A",
                NEUTRAL => "N",
                _ => "X",
            };
            if self.rev[i] {
                format!("[{m}]{w}")
            } else {
                format!("({}){}", m.to_lowercase(), w.to_lowercase())
            }
        } else {
            w
        }
    }
}

impl Game for Spionii {
    type Move = Move;

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Guess => self.side,
            _ => RESOLVER,
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        match self.phase {
            Phase::NeedClue => vec![Move::Clue],
            Phase::Guess => {
                let mut v: Vec<Move> = (0..25).filter(|&i| !self.rev[i]).map(Move::Guess).collect();
                if self.made >= 1 {
                    v.push(Move::Stop);
                }
                v
            }
            Phase::Pending => vec![Move::Reveal],
            Phase::Over => Vec::new(),
        }
    }

    fn apply(&mut self, m: &Move) {
        self.heur.clear();
        match *m {
            Move::Clue => {
                let (t, n) = self.spymaster();
                self.used.push(t);
                self.clue = Some((t, n));
                self.made = 0;
                self.log.push(format!(
                    "Maestrul-spion al Jucătorului {}: «{}» ×{}",
                    self.side.idx() + 1,
                    lex().themes[t].clue,
                    n
                ));
                self.phase = Phase::Guess;
            }
            Move::Guess(i) => {
                let (t, n) = self.clue.expect("indiciu dat");
                let assoc = self.in_theme(i, t);
                let v = match (assoc, self.made < n) {
                    (true, true) => 100,
                    (true, false) => 30,
                    _ => 0,
                };
                self.heur.set(self.side, v);
                self.pending = i;
                self.phase = Phase::Pending;
            }
            Move::Stop => {
                self.heur.set(self.side, 50);
                self.log
                    .push(format!("Jucătorul {} se oprește.", self.side.idx() + 1));
                self.end_turn();
            }
            Move::Reveal => {
                let i = self.pending;
                self.rev[i] = true;
                let n = self.clue.map(|c| c.1).unwrap_or(1);
                let me = self.side.idx() as u8;
                let w = lex().pool[self.board[i]].0.to_uppercase();
                match self.key[i] {
                    k if k == me => {
                        self.rem[me as usize] -= 1;
                        self.made += 1;
                        self.log.push(format!("«{w}»: agent al echipei!"));
                        if self.rem[me as usize] == 0 {
                            self.winner = Some(self.side);
                            self.phase = Phase::Over;
                        } else if self.made > n {
                            self.end_turn();
                        } else {
                            self.phase = Phase::Guess;
                        }
                    }
                    ASSASSIN => {
                        self.log.push(format!("«{w}»: ASASINUL!"));
                        self.winner = Some(self.side.other());
                        self.phase = Phase::Over;
                    }
                    NEUTRAL => {
                        self.log.push(format!("«{w}»: neutru."));
                        self.end_turn();
                    }
                    _ => {
                        let o = self.side.other();
                        self.rem[o.idx()] -= 1;
                        self.log.push(format!("«{w}»: agent al adversarului."));
                        if self.rem[o.idx()] == 0 {
                            self.winner = Some(o);
                            self.phase = Phase::Over;
                        } else {
                            self.end_turn();
                        }
                    }
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        self.winner.map(Outcome::Win)
    }

    fn evaluate(&self, p: Player) -> i32 {
        100 * (self.rem[p.other().idx()] - self.rem[p.idx()]) + self.heur.for_player(p)
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }

    fn render(&self) -> String {
        let mut s = String::new();
        let _ = writeln!(
            s,
            "Spionii Cuvintelor — agenți de găsit: J1 (roșii, R) {}, J2 (albaștri, A) {}",
            self.rem[0], self.rem[1]
        );
        for r in 0..5 {
            for c in 0..5 {
                let _ = write!(s, "{:<15}", self.cell_text(r * 5 + c));
            }
            s.push('\n');
        }
        if self.phase == Phase::Over {
            s.push_str("Joc încheiat (între paranteze rotunde: culorile nedescoperite).\n");
        } else if let Some((t, n)) = self.clue {
            let _ = writeln!(
                s,
                "Indiciu: {} ×{} | ghicitor: Jucătorul {} | ghicite în tură: {} din cel mult {}",
                lex().themes[t].clue,
                n,
                self.side.idx() + 1,
                self.made,
                n + 1
            );
        } else {
            let _ = writeln!(
                s,
                "Urmează indiciul maestrului-spion al Jucătorului {}.",
                self.side.idx() + 1
            );
        }
        s.push_str(&log_tail(&self.log, 3));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Clue => "indiciu".into(),
            Move::Guess(i) => format!("ghici {}", lex().pool[self.board[*i]].1),
            Move::Stop => "stop".into(),
            Move::Reveal => "dezvaluie".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::GameObj;

    #[test]
    fn lexicon_is_big_and_consistent() {
        let l = lex();
        assert!(THEMES.len() >= 50);
        assert!(l.pool.len() >= 500, "{} cuvinte", l.pool.len());
        for t in &l.themes {
            assert!(t.members.len() >= 10, "{}", t.clue);
        }
        let mut f: Vec<&String> = l.pool.iter().map(|p| &p.1).collect();
        f.sort();
        f.dedup();
        assert_eq!(f.len(), l.pool.len());
        for w in &l.pool {
            assert!(!w.1.contains(char::is_whitespace));
        }
    }

    #[test]
    fn board_key_counts() {
        let g = Spionii::with_seed(4);
        for (k, n) in [(0u8, 9), (1, 8), (NEUTRAL, 7), (ASSASSIN, 1)] {
            assert_eq!(g.key.iter().filter(|&&x| x == k).count(), n);
        }
        let mut b = g.board.to_vec();
        b.sort();
        b.dedup();
        assert_eq!(b.len(), 25);
    }

    #[test]
    fn spymaster_gives_legal_clues_covering_own_agents() {
        for seed in 1..30u64 {
            let mut g = Spionii::with_seed(seed);
            g.apply(&Move::Clue);
            let (t, n) = g.clue.unwrap();
            assert!(g.clue_is_legal(t), "indiciu pe tablă");
            assert!((1..=4).contains(&n));
            let own = (0..25)
                .filter(|&i| g.key[i] == 0 && g.in_theme(i, t))
                .count();
            assert!(own >= 1);
        }
    }

    #[test]
    fn assassin_loses_and_finish() {
        let mut g = Spionii::with_seed(2);
        g.apply(&Move::Clue);
        let a = (0..25).find(|&i| g.key[i] == ASSASSIN).unwrap();
        g.apply(&Move::Guess(a));
        g.apply(&Move::Reveal);
        assert_eq!(Game::outcome(&g), Some(Outcome::Win(Player::Two)));
    }

    #[test]
    fn turn_passes_on_neutral_and_all_agents_wins() {
        let mut g = Spionii::with_seed(3);
        g.apply(&Move::Clue);
        let nn = (0..25).find(|&i| g.key[i] == NEUTRAL).unwrap();
        g.apply(&Move::Guess(nn));
        g.apply(&Move::Reveal);
        assert_eq!(Game::to_move(&g), RESOLVER);
        assert_eq!(g.side, Player::Two);
        // echipa 2 își găsește toți agenții
        let mut h = Spionii::with_seed(3);
        h.side = Player::Two;
        h.apply(&Move::Clue);
        for i in 0..25 {
            if h.key[i] == 1 {
                if h.phase != Phase::Guess {
                    h.apply(&Move::Clue);
                }
                h.apply(&Move::Guess(i));
                h.apply(&Move::Reveal);
            }
        }
        assert_eq!(Game::outcome(&h), Some(Outcome::Win(Player::Two)));
    }

    #[test]
    fn random_games_end() {
        for seed in 1..20u64 {
            let mut g: Box<dyn GameObj> = Box::new(Spionii::with_seed(seed));
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() && n < 2000 {
                let m = g.legal_moves();
                g.play(&m[rng.below(m.len())]).unwrap();
                n += 1;
            }
            assert!(g.outcome().is_some());
        }
    }

    #[test]
    fn ai_does_not_peek_at_the_key() {
        for seed in 1..10u64 {
            let mut g = Spionii::with_seed(seed);
            g.apply(&Move::Clue);
            let mut h = g.clone();
            // schimbă culorile cărților nedescoperite (aceeași informație vizibilă)
            let un: Vec<usize> = (0..25).collect();
            for w in un.windows(2).step_by(2) {
                h.key.swap(w[0], w[1]);
            }
            let p = g.side;
            for m in Game::legal_moves(&g) {
                let (mut a, mut b) = (g.clone(), h.clone());
                a.apply(&m);
                b.apply(&m);
                assert_eq!(Game::evaluate(&a, p), Game::evaluate(&b, p));
            }
        }
    }

    #[test]
    fn ai_picks_theme_words() {
        let mut g = Spionii::with_seed(7);
        g.apply(&Move::Clue);
        let (t, _) = g.clue.unwrap();
        let mv = GameObj::ai_move(&g, 5).unwrap();
        // în faza P1, ai_move alege cel mai bun cuvânt din temă
        let cell = (0..25)
            .find(|&i| format!("ghici {}", lex().pool[g.board[i]].1) == mv)
            .unwrap();
        assert!(g.in_theme(cell, t));
    }
}
