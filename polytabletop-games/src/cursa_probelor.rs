//! Cursa probelor — joc de familie cu mini-probe pe o pistă circulară (variantă originală).
//!
//! 2–4 jucători (tu = locul 1, ceilalți sunt roboți) pe o pistă de 30 de câmpuri. La fiecare
//! tură arunci zarul d6 și avansezi; câmpul pe care ajungi declanșează o mini-probă:
//!
//! * **N — numărare**: un șir de simboluri `* # o +`; numeri câte apar dintr-un simbol
//!   (3 variante de răspuns: `raspuns 1|2|3`).
//! * **M — memorie**: secvență de litere A–D afișată o dată (`gata` o ascunde), apoi o
//!   reproduci literă cu literă (`A`, `B`, `C`, `D`).
//! * **D — duel de zaruri**: alegi un adversar (`duel 2`), ambii aruncă; câștigă zarul mai mare
//!   (egalitatea se aruncă din nou).
//! * **R — pariu de risc**: arunci până la 3 zaruri (`arunc`) și te oprești (`opresc`) înainte
//!   de un 1; dacă dai peste 1 pierzi; te oprești cu cel puțin 9 puncte = medalie.
//! * **L — logică/aritmetică**: întrebare generată procedural (calcul, șir aritmetic sau
//!   geometric), 3 variante de răspuns (`raspuns 1|2|3`).
//! * **O — odihnă**: fără probă; primești o „a doua șansă” (dacă ratezi proba următoare,
//!   primești alta nouă de același fel).
//!
//! Probele câștigate dau medalii. Primul cu 5 medalii câștigă; dacă nimeni nu ajunge acolo,
//! la sfârșitul a 3 ture de pistă (90 de pași pe jucător) câștigă cine are cele mai multe
//! medalii (egalitate la vârf cu omul = remiză).
//!
//! Probabilitățile și întrebările sunt generate din seed (determinist), nu există bancă de
//! întrebări. Orice eveniment cu șansă sau verdict ascuns se petrece într-o mutare forțată
//! (`rezolv`, `robot`), ca AI-ul să nu „vadă” rezultatul înainte de a alege.
//!
//! Roboții nu trișează: răspund doar pe baza a ceea ce vede și omul (șirul afișat, întrebarea,
//! variantele), cu zgomot dependent de nivel (Easy/Normal/Hard/Expert): numără simboluri cu
//! greșeli, rețin literele cu o probabilitate, calculează cu scăpări, iar la risc se opresc
//! mai mult sau mai puțin disciplinat. Nivelul vine din `ai::level()` (sau din `with_level`).
#![allow(clippy::needless_range_loop)]

use crate::cards::shuffle;
use polytabletop_core::ai::{self, Level};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const TRACK: usize = 30;
pub const LAPS: usize = 3;
pub const GOAL_STEPS: u32 = (TRACK * LAPS) as u32;
pub const MEDALS_TO_WIN: u8 = 5;
pub const RISK_TARGET: u32 = 9;
const LOG_KEEP: usize = 40;
const COUNT_SYMBOLS: [char; 4] = ['*', '#', 'o', '+'];
const MEM_LETTERS: [char; 4] = ['A', 'B', 'C', 'D'];
const MEM_LEN: usize = 5;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Sq {
    Count,
    Memory,
    Duel,
    Risk,
    Logic,
    Rest,
}

impl Sq {
    pub fn letter(self) -> char {
        match self {
            Sq::Count => 'N',
            Sq::Memory => 'M',
            Sq::Duel => 'D',
            Sq::Risk => 'R',
            Sq::Logic => 'L',
            Sq::Rest => 'O',
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Sq::Count => "numărare",
            Sq::Memory => "memorie",
            Sq::Duel => "duel de zaruri",
            Sq::Risk => "pariu de risc",
            Sq::Logic => "logică",
            Sq::Rest => "odihnă",
        }
    }
}

/// Distribuția câmpurilor pe pistă (30): 7 N, 6 M, 5 D, 5 R, 5 L, 2 O.
const BOARD_MIX: [(Sq, usize); 6] = [
    (Sq::Count, 7),
    (Sq::Memory, 6),
    (Sq::Duel, 5),
    (Sq::Risk, 5),
    (Sq::Logic, 5),
    (Sq::Rest, 2),
];

/// Expresii de logică vizibile jucătorului (și, deci, robotului).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    /// `a + b*c`
    AddMul(i32, i32, i32),
    /// `a*b - c`
    MulSub(i32, i32, i32),
    /// șir aritmetic: start, pas (4 termeni afișați, al 5-lea cerut)
    Arith(i32, i32),
    /// șir geometric: start, rația
    Geo(i32, i32),
}

impl Expr {
    fn solve(&self) -> i32 {
        match *self {
            Expr::AddMul(a, b, c) => a + b * c,
            Expr::MulSub(a, b, c) => a * b - c,
            Expr::Arith(s, d) => s + 4 * d,
            Expr::Geo(s, r) => s * r.pow(4),
        }
    }
    fn text(&self) -> String {
        match *self {
            Expr::AddMul(a, b, c) => format!("Cât face {a} + {b}*{c} ?"),
            Expr::MulSub(a, b, c) => format!("Cât face {a}*{b} - {c} ?"),
            Expr::Arith(s, d) => {
                let t: Vec<String> = (0..4).map(|i| (s + i * d).to_string()).collect();
                format!("Continuă șirul: {}, ?", t.join(", "))
            }
            Expr::Geo(s, r) => {
                let t: Vec<String> = (0..4).map(|i| (s * r.pow(i as u32)).to_string()).collect();
                format!("Continuă șirul: {}, ?", t.join(", "))
            }
        }
    }
}

/// Ce se vede public dintr-o probă (tot ce are voie să folosească un robot).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Public {
    Count { seq: Vec<u8>, target: u8 },
    Memory { seq: Vec<u8> },
    Logic { expr: Expr },
}

#[derive(Clone, Debug)]
pub struct Trial {
    pub public: Public,
    /// Variantele de răspuns (numărare/logică); 0 pentru memorie.
    pub options: [i32; 3],
}

impl Trial {
    /// Indexul variantei corecte (doar pentru verdict, nu pentru roboți).
    fn correct_option(&self) -> Option<usize> {
        let truth = match &self.public {
            Public::Count { seq, target } => seq.iter().filter(|&&s| s == *target).count() as i32,
            Public::Logic { expr } => expr.solve(),
            Public::Memory { .. } => return None,
        };
        self.options.iter().position(|&o| o == truth)
    }
}

fn three_options(truth: i32, rng: &mut Rng, deltas: &[i32]) -> [i32; 3] {
    let mut ds: Vec<i32> = deltas.to_vec();
    shuffle(&mut ds, rng);
    let mut out = vec![truth];
    for d in ds {
        let v = truth + d;
        if v > 0 && !out.contains(&v) && out.len() < 3 {
            out.push(v);
        }
    }
    let mut k = 1;
    while out.len() < 3 {
        if !out.contains(&(truth + k)) {
            out.push(truth + k);
        }
        k += 1;
    }
    shuffle(&mut out, rng);
    [out[0], out[1], out[2]]
}

fn gen_count(rng: &mut Rng) -> Trial {
    let len = 14 + rng.below(7);
    let mut seq: Vec<u8> = (0..len).map(|_| rng.below(4) as u8).collect();
    let target = rng.below(4) as u8;
    // cel puțin 2 apariții ale simbolului căutat
    while seq.iter().filter(|&&s| s == target).count() < 2 {
        let i = rng.below(len);
        seq[i] = target;
    }
    let n = seq.iter().filter(|&&s| s == target).count() as i32;
    let options = three_options(n, rng, &[-2, -1, 1, 2]);
    Trial {
        public: Public::Count { seq, target },
        options,
    }
}

fn gen_memory(rng: &mut Rng) -> Trial {
    let seq: Vec<u8> = (0..MEM_LEN).map(|_| rng.below(4) as u8).collect();
    Trial {
        public: Public::Memory { seq },
        options: [0; 3],
    }
}

fn gen_logic(rng: &mut Rng) -> Trial {
    let r = |rng: &mut Rng, lo: i32, hi: i32| lo + rng.below((hi - lo + 1) as usize) as i32;
    let expr = match rng.below(4) {
        0 => Expr::AddMul(r(rng, 10, 30), r(rng, 2, 9), r(rng, 2, 9)),
        1 => {
            let (a, b) = (r(rng, 3, 9), r(rng, 3, 9));
            Expr::MulSub(a, b, r(rng, 1, a * b - 1))
        }
        2 => Expr::Arith(r(rng, 2, 20), r(rng, 2, 9)),
        _ => Expr::Geo(r(rng, 1, 4), r(rng, 2, 3)),
    };
    let truth = expr.solve();
    let options = three_options(truth, rng, &[-10, -2, -1, 1, 2, 10]);
    Trial {
        public: Public::Logic { expr },
        options,
    }
}

fn gen_trial(kind: Sq, rng: &mut Rng) -> Trial {
    match kind {
        Sq::Count => gen_count(rng),
        Sq::Memory => gen_memory(rng),
        _ => gen_logic(rng),
    }
}

/// Parametrii „zgomotului” robotului, în procente, după nivel.
struct Skill {
    /// șansa de a rata un simbol căutat la numărare
    miss: usize,
    /// șansa de a număra greșit un simbol străin
    false_pos: usize,
    /// șansa de a reține corect o literă
    recall: usize,
    /// șansa de a greși un calcul cu ±1..2 (sau 10)
    slip: usize,
    /// șansa de a se opri când a ajuns la țintă
    stop: usize,
    /// alege adversarul cel mai bun la duel?
    targets_leader: bool,
}

fn skill(l: Level) -> Skill {
    match l {
        Level::Easy => Skill {
            miss: 25,
            false_pos: 6,
            recall: 70,
            slip: 45,
            stop: 55,
            targets_leader: false,
        },
        Level::Normal => Skill {
            miss: 12,
            false_pos: 3,
            recall: 85,
            slip: 25,
            stop: 80,
            targets_leader: false,
        },
        Level::Hard => Skill {
            miss: 5,
            false_pos: 1,
            recall: 94,
            slip: 10,
            stop: 95,
            targets_leader: true,
        },
        Level::Expert => Skill {
            miss: 1,
            false_pos: 0,
            recall: 99,
            slip: 3,
            stop: 100,
            targets_leader: true,
        },
    }
}

fn nearest_option(opts: &[i32; 3], est: i32, rng: &mut Rng) -> usize {
    let best = opts.iter().map(|o| (o - est).abs()).min().unwrap_or(0);
    let c: Vec<usize> = (0..3).filter(|&i| (opts[i] - est).abs() == best).collect();
    c[rng.below(c.len())]
}

/// Răspunsul robotului la numărare/logică: doar din `Public` + `options`, cu zgomot.
fn robot_choice(t: &Trial, sk: &Skill, rng: &mut Rng) -> usize {
    match &t.public {
        Public::Count { seq, target } => {
            let mut est = 0;
            for &s in seq {
                let p = if s == *target {
                    100 - sk.miss
                } else {
                    sk.false_pos
                };
                if rng.below(100) < p {
                    est += 1;
                }
            }
            nearest_option(&t.options, est, rng)
        }
        Public::Logic { expr } => {
            let mut v = expr.solve();
            if rng.below(100) < sk.slip {
                let slips = [-10, -2, -1, 1, 2, 10];
                v += slips[rng.below(slips.len())];
            }
            nearest_option(&t.options, v, rng)
        }
        Public::Memory { .. } => 0,
    }
}

/// Memorie robot: fiecare literă e reținută cu probabilitatea `recall`, altfel ghicită.
fn robot_recall(seq: &[u8], sk: &Skill, rng: &mut Rng) -> Vec<u8> {
    seq.iter()
        .map(|&e| {
            if rng.below(100) < sk.recall {
                e
            } else {
                rng.below(4) as u8
            }
        })
        .collect()
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    /// Alegerea numărului de jucători (2–4).
    Players(u8),
    /// Aruncarea zarului de mișcare (jucătorul uman).
    Roll,
    /// Răspuns la numărare/logică (0..3).
    Answer(u8),
    /// Am memorat secvența: ascunde-o.
    Ready,
    /// O literă din secvența reprodusă (0..4 = A..D).
    Letter(u8),
    /// Duel cu locul indicat (0-based).
    Duel(u8),
    /// Pariu de risc: mai arunc un zar.
    RiskRoll,
    /// Pariu de risc: mă opresc.
    RiskStop,
    /// Mutare forțată: verdict/aruncări ascunse.
    Resolve,
    /// Mutare forțată: tura unui robot.
    Robot,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Setup,
    Roll,
    RobotTurn,
    Answer,
    Memorize,
    Recall,
    Judge,
    DuelPick,
    DuelResolve,
    RiskDecide,
    RiskRoll,
    Over,
}

#[derive(Clone)]
pub struct CursaProbelor {
    n: usize,
    board: [Sq; TRACK],
    steps: Vec<u32>,
    medals: Vec<u8>,
    retry: Vec<bool>,
    cur: usize,
    phase: Phase,
    trial: Option<Trial>,
    kind: Sq,
    choice: Option<usize>,
    recall: Vec<u8>,
    duel_opp: usize,
    risk: Vec<u8>,
    winner: Option<usize>,
    tie_top: bool,
    turns: u32,
    level: Option<Level>,
    rng: Rng,
    log: Vec<String>,
}

impl Default for CursaProbelor {
    fn default() -> Self {
        Self::new()
    }
}

fn seat_name(i: usize) -> String {
    if i == 0 {
        "Tu (1)".to_string()
    } else {
        format!("Robot {}", i + 1)
    }
}

impl CursaProbelor {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    /// Început cu alegerea numărului de jucători (implicit 3 până la `jucatori N`).
    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut cells: Vec<Sq> = Vec::new();
        for (sq, k) in BOARD_MIX {
            for _ in 0..k {
                cells.push(sq);
            }
        }
        shuffle(&mut cells, &mut rng);
        // startul (câmpul 0) este întotdeauna odihnă
        if let Some(p) = cells.iter().position(|&s| s == Sq::Rest) {
            cells.swap(0, p);
        }
        let mut board = [Sq::Rest; TRACK];
        board.copy_from_slice(&cells);
        let mut g = CursaProbelor {
            n: 3,
            board,
            steps: vec![0; 3],
            medals: vec![0; 3],
            retry: vec![false; 3],
            cur: 0,
            phase: Phase::Setup,
            trial: None,
            kind: Sq::Rest,
            choice: None,
            recall: Vec::new(),
            duel_opp: 1,
            risk: Vec::new(),
            winner: None,
            tie_top: false,
            turns: 0,
            level: None,
            rng,
            log: Vec::new(),
        };
        g.log
            .push("Alege numărul de jucători: jucatori 2 / 3 / 4.".into());
        g
    }

    /// Pornește direct cu `n` jucători (2–4), fără pasul de alegere.
    pub fn with_players(seed: u64, n: usize) -> Self {
        let mut g = Self::with_seed(seed);
        g.set_players(n);
        g
    }

    /// Fixează nivelul robotilor (în loc de `ai::level()`).
    pub fn with_level(mut self, l: Level) -> Self {
        self.level = Some(l);
        self
    }

    fn set_players(&mut self, n: usize) {
        let n = n.clamp(2, 4);
        self.n = n;
        self.steps = vec![0; n];
        self.medals = vec![0; n];
        self.retry = vec![false; n];
        self.cur = 0;
        self.phase = Phase::Roll;
        self.log.clear();
        self.log.push(format!(
            "Începe cursa: {n} jucători. Primul cu {MEDALS_TO_WIN} medalii câștigă."
        ));
    }

    fn lvl(&self) -> Level {
        self.level.unwrap_or_else(ai::level)
    }

    pub fn players(&self) -> usize {
        self.n
    }
    pub fn medals(&self, seat: usize) -> u8 {
        self.medals[seat]
    }
    pub fn steps(&self, seat: usize) -> u32 {
        self.steps[seat]
    }
    pub fn board(&self) -> &[Sq; TRACK] {
        &self.board
    }
    pub fn is_over(&self) -> bool {
        self.phase == Phase::Over
    }
    pub fn winner(&self) -> Option<usize> {
        self.winner
    }

    fn push_log(&mut self, s: String) {
        self.log.push(s);
        if self.log.len() > LOG_KEEP {
            let k = self.log.len() - LOG_KEEP;
            self.log.drain(..k);
        }
    }

    fn square(&self, who: usize) -> usize {
        (self.steps[who] as usize) % TRACK
    }

    fn finished(&self, who: usize) -> bool {
        self.steps[who] >= GOAL_STEPS
    }

    /// Mută jucătorul cu `d` pași (oprit la linia de sosire) și întoarce câmpul.
    fn advance(&mut self, who: usize, d: u8) -> Sq {
        let before = self.steps[who];
        self.steps[who] = (before + u32::from(d)).min(GOAL_STEPS);
        let sq = self.board[self.square(who)];
        let lap = self.steps[who] as usize / TRACK + 1;
        let note = if self.finished(who) {
            " — a încheiat cele 3 ture".to_string()
        } else {
            format!(" (tura {}/{LAPS})", lap.min(LAPS))
        };
        self.push_log(format!(
            "{} aruncă {d} și ajunge pe câmpul {} [{}: {}]{note}.",
            seat_name(who),
            self.square(who),
            sq.letter(),
            sq.name()
        ));
        if self.finished(who) {
            return Sq::Rest;
        }
        sq
    }

    fn award(&mut self, who: usize, why: &str) {
        self.medals[who] += 1;
        self.push_log(format!(
            "{} câștigă o medalie ({why}) — {} în total.",
            seat_name(who),
            self.medals[who]
        ));
        if self.medals[who] >= MEDALS_TO_WIN && self.winner.is_none() {
            self.winner = Some(who);
        }
    }

    fn end_turn(&mut self) {
        self.trial = None;
        self.choice = None;
        self.recall.clear();
        self.risk.clear();
        self.turns += 1;
        if self.winner.is_some() {
            self.phase = Phase::Over;
            self.push_log(format!(
                "{} a strâns {MEDALS_TO_WIN} medalii și câștigă cursa!",
                seat_name(self.winner.unwrap_or(0))
            ));
            return;
        }
        for k in 1..=self.n {
            let s = (self.cur + k) % self.n;
            if !self.finished(s) {
                self.cur = s;
                self.phase = if s == 0 {
                    Phase::Roll
                } else {
                    Phase::RobotTurn
                };
                return;
            }
        }
        // toți au încheiat cele 3 ture
        let top = *self.medals.iter().max().unwrap_or(&0);
        let leaders: Vec<usize> = (0..self.n).filter(|&i| self.medals[i] == top).collect();
        if leaders.len() == 1 {
            self.winner = Some(leaders[0]);
        } else if leaders.contains(&0) {
            self.tie_top = true;
        } else {
            self.winner = Some(leaders[0]);
        }
        self.phase = Phase::Over;
        self.push_log(format!(
            "Cele {LAPS} ture s-au încheiat. Cele mai multe medalii: {top}."
        ));
    }

    /// Începe proba câmpului pentru jucătorul uman (`cur == 0`).
    fn start_human(&mut self, sq: Sq) {
        self.kind = sq;
        match sq {
            Sq::Rest => {
                self.retry[0] = true;
                self.push_log("Odihnă: ai o a doua șansă la următoarea probă ratată.".into());
                self.end_turn();
            }
            Sq::Count | Sq::Logic => {
                self.trial = Some(gen_trial(sq, &mut self.rng));
                self.phase = Phase::Answer;
            }
            Sq::Memory => {
                self.trial = Some(gen_trial(sq, &mut self.rng));
                self.phase = Phase::Memorize;
            }
            Sq::Duel => self.phase = Phase::DuelPick,
            Sq::Risk => {
                self.risk.clear();
                self.phase = Phase::RiskDecide;
            }
        }
    }

    fn risk_sum(&self) -> u32 {
        self.risk.iter().map(|&d| u32::from(d)).sum()
    }

    /// Închide pariul de risc cu zarurile curente (fără niciun 1).
    fn finish_risk(&mut self, who: usize) {
        let s = self.risk_sum();
        if s >= RISK_TARGET {
            self.award(who, &format!("risc: {s} puncte"));
        } else {
            self.push_log(format!(
                "{} se oprește la {s} (trebuie cel puțin {RISK_TARGET}): fără medalie.",
                seat_name(who)
            ));
        }
    }

    fn roll_duel(&mut self, a: usize, b: usize) -> usize {
        for _ in 0..20 {
            let (x, y) = (self.rng.d6(), self.rng.d6());
            self.push_log(format!(
                "Duel: {} {x} — {} {y}.",
                seat_name(a),
                seat_name(b)
            ));
            if x != y {
                return if x > y { a } else { b };
            }
        }
        a
    }

    /// Verdictul unei probe cu răspuns; `true` dacă s-a câștigat.
    fn judge_human(&mut self) -> bool {
        let Some(t) = self.trial.clone() else {
            return false;
        };
        match &t.public {
            Public::Memory { seq } => {
                let ok = self.recall == *seq;
                let shown: String = seq.iter().map(|&i| MEM_LETTERS[i as usize]).collect();
                self.push_log(format!(
                    "Secvența era {shown}; ai dat {}.",
                    self.recall
                        .iter()
                        .map(|&i| MEM_LETTERS[i as usize])
                        .collect::<String>()
                ));
                ok
            }
            _ => {
                let ok = self.choice == t.correct_option();
                let truth = t.correct_option().map_or(0, |i| t.options[i]);
                self.push_log(format!("Răspunsul corect era {truth}."));
                ok
            }
        }
    }

    /// Tura completă a unui robot (zar, câmp, probă) într-o singură mutare forțată.
    fn robot_turn(&mut self) {
        let who = self.cur;
        let sk = skill(self.lvl());
        let d = self.rng.d6();
        let sq = self.advance(who, d);
        let kind = sq;
        loop {
            match kind {
                Sq::Rest => {
                    if !self.finished(who) {
                        self.retry[who] = true;
                        self.push_log(format!("{} se odihnește (a doua șansă).", seat_name(who)));
                    }
                    break;
                }
                Sq::Duel => {
                    let others: Vec<usize> = (0..self.n).filter(|&i| i != who).collect();
                    let opp = if sk.targets_leader {
                        let top = others.iter().map(|&i| self.medals[i]).max().unwrap_or(0);
                        let lead: Vec<usize> = others
                            .iter()
                            .copied()
                            .filter(|&i| self.medals[i] == top)
                            .collect();
                        lead[self.rng.below(lead.len())]
                    } else {
                        others[self.rng.below(others.len())]
                    };
                    self.push_log(format!(
                        "{} provoacă la duel pe {}.",
                        seat_name(who),
                        seat_name(opp)
                    ));
                    let w = self.roll_duel(who, opp);
                    self.award(w, "duel");
                    break;
                }
                Sq::Risk => {
                    let mut dice: Vec<u8> = Vec::new();
                    let mut bust = false;
                    loop {
                        let r = self.rng.d6();
                        if r == 1 {
                            bust = true;
                            self.push_log(format!(
                                "{} aruncă risc: {} apoi 1 — pierde.",
                                seat_name(who),
                                dice.iter().map(u8::to_string).collect::<Vec<_>>().join("+")
                            ));
                            break;
                        }
                        dice.push(r);
                        let s: u32 = dice.iter().map(|&x| u32::from(x)).sum();
                        if dice.len() == 3 || (s >= RISK_TARGET && self.rng.below(100) < sk.stop) {
                            break;
                        }
                    }
                    if !bust {
                        self.risk = dice;
                        self.finish_risk(who);
                        self.risk.clear();
                    }
                    break;
                }
                Sq::Memory => {
                    let t = gen_memory(&mut self.rng);
                    let ok = if let Public::Memory { seq } = &t.public {
                        robot_recall(seq, &sk, &mut self.rng) == *seq
                    } else {
                        false
                    };
                    if ok {
                        self.award(who, "memorie");
                        break;
                    }
                    self.push_log(format!("{} greșește la memorie.", seat_name(who)));
                }
                Sq::Count | Sq::Logic => {
                    let t = gen_trial(kind, &mut self.rng);
                    let pick = robot_choice(&t, &sk, &mut self.rng);
                    if Some(pick) == t.correct_option() {
                        self.award(who, kind.name());
                        break;
                    }
                    self.push_log(format!("{} greșește la {}.", seat_name(who), kind.name()));
                }
            }
            // eșec la o probă cu răspuns: a doua șansă, dacă există
            if self.retry[who] {
                self.retry[who] = false;
                self.push_log(format!("{} folosește a doua șansă.", seat_name(who)));
                continue;
            }
            break;
        }
        self.end_turn();
    }

    fn lead_diff(&self, seat: usize) -> i32 {
        let mine = i32::from(self.medals[seat]);
        let best_other = (0..self.n)
            .filter(|&i| i != seat)
            .map(|i| i32::from(self.medals[i]))
            .max()
            .unwrap_or(0);
        mine - best_other
    }
}

fn cell_label(g: &CursaProbelor, idx: usize) -> (String, String) {
    let toks: String = (0..g.n)
        .filter(|&p| g.square(p) == idx && !g.finished(p))
        .map(|p| char::from_digit(p as u32 + 1, 10).unwrap_or('?'))
        .collect();
    (
        format!("{}{:02}", g.board[idx].letter(), idx),
        format!("{toks:<4}"),
    )
}

impl CursaProbelor {
    fn render_track(&self) -> String {
        // 9 coloane × 8 rânduri: sus 0..8, dreapta 9..14, jos 23..15, stânga 29..24
        let (w, h) = (9usize, 8usize);
        let mut grid: Vec<Vec<Option<usize>>> = vec![vec![None; w]; h];
        for x in 0..w {
            grid[0][x] = Some(x);
            grid[h - 1][x] = Some(23 - x);
        }
        for r in 1..h - 1 {
            grid[r][w - 1] = Some(8 + r);
            grid[r][0] = Some(30 - r);
        }
        let mut s = String::new();
        for r in 0..h {
            let mut l1 = String::new();
            let mut l2 = String::new();
            for x in 0..w {
                match grid[r][x] {
                    Some(i) => {
                        let (a, b) = cell_label(self, i);
                        l1.push_str(&format!("[{a} ]"));
                        l2.push_str(&format!(" {b} "));
                    }
                    None => {
                        l1.push_str("      ");
                        l2.push_str("      ");
                    }
                }
            }
            s.push_str(l1.trim_end());
            s.push('\n');
            s.push_str(l2.trim_end());
            s.push('\n');
        }
        s
    }

    fn render_trial(&self) -> String {
        let mut s = String::new();
        let Some(t) = &self.trial else {
            return s;
        };
        match (&t.public, self.phase) {
            (Public::Count { seq, target }, Phase::Answer) => {
                let line: Vec<String> = seq
                    .chunks(5)
                    .map(|c| c.iter().map(|&i| COUNT_SYMBOLS[i as usize]).collect())
                    .collect();
                s.push_str(&format!("Numără: {}\n", line.join(" ")));
                s.push_str(&format!(
                    "Câți „{}” sunt? 1) {}  2) {}  3) {}\n",
                    COUNT_SYMBOLS[*target as usize], t.options[0], t.options[1], t.options[2]
                ));
            }
            (Public::Logic { expr }, Phase::Answer) => {
                s.push_str(&format!("{}\n", expr.text()));
                s.push_str(&format!(
                    "1) {}  2) {}  3) {}\n",
                    t.options[0], t.options[1], t.options[2]
                ));
            }
            (Public::Memory { seq }, Phase::Memorize) => {
                let line: Vec<String> = seq
                    .iter()
                    .map(|&i| MEM_LETTERS[i as usize].to_string())
                    .collect();
                s.push_str(&format!(
                    "Memorează secvența: {}  (apoi `gata`)\n",
                    line.join(" ")
                ));
            }
            (Public::Memory { seq }, Phase::Recall) => {
                let typed: String = self
                    .recall
                    .iter()
                    .map(|&i| MEM_LETTERS[i as usize])
                    .collect();
                s.push_str(&format!(
                    "Reproduce secvența (A–D), literă cu literă: {}{} ({}/{})\n",
                    typed,
                    "_".repeat(seq.len() - self.recall.len()),
                    self.recall.len(),
                    seq.len()
                ));
            }
            _ => {}
        }
        s
    }
}

impl Game for CursaProbelor {
    type Move = Move;

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Setup
            | Phase::Roll
            | Phase::Answer
            | Phase::Memorize
            | Phase::Recall
            | Phase::DuelPick
            | Phase::RiskDecide => Player::One,
            _ => Player::Two,
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        match self.phase {
            Phase::Over => Vec::new(),
            Phase::Setup => (2..=4).map(Move::Players).collect(),
            Phase::Roll => vec![Move::Roll],
            Phase::RobotTurn => vec![Move::Robot],
            Phase::Answer => (0..3).map(Move::Answer).collect(),
            Phase::Memorize => vec![Move::Ready],
            Phase::Recall => (0..4).map(Move::Letter).collect(),
            Phase::Judge | Phase::DuelResolve | Phase::RiskRoll => vec![Move::Resolve],
            Phase::DuelPick => (1..self.n as u8).map(Move::Duel).collect(),
            Phase::RiskDecide => {
                let mut v = Vec::new();
                if self.risk.len() < 3 {
                    v.push(Move::RiskRoll);
                }
                if !self.risk.is_empty() {
                    v.push(Move::RiskStop);
                }
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        match *m {
            Move::Players(n) => self.set_players(n as usize),
            Move::Roll => {
                let d = self.rng.d6();
                let sq = self.advance(0, d);
                if self.finished(0) {
                    self.end_turn();
                } else {
                    self.start_human(sq);
                }
            }
            Move::Answer(k) => {
                self.choice = Some(k as usize);
                self.phase = Phase::Judge;
            }
            Move::Ready => {
                self.recall.clear();
                self.phase = Phase::Recall;
            }
            Move::Letter(i) => {
                self.recall.push(i);
                let len = match &self.trial {
                    Some(Trial {
                        public: Public::Memory { seq },
                        ..
                    }) => seq.len(),
                    _ => MEM_LEN,
                };
                if self.recall.len() >= len {
                    self.phase = Phase::Judge;
                }
            }
            Move::Duel(o) => {
                self.duel_opp = o as usize;
                self.phase = Phase::DuelResolve;
            }
            Move::RiskRoll => self.phase = Phase::RiskRoll,
            Move::RiskStop => {
                self.finish_risk(0);
                self.end_turn();
            }
            Move::Robot => self.robot_turn(),
            Move::Resolve => match self.phase {
                Phase::DuelResolve => {
                    let opp = self.duel_opp;
                    let w = self.roll_duel(0, opp);
                    self.award(w, "duel");
                    self.end_turn();
                }
                Phase::RiskRoll => {
                    let r = self.rng.d6();
                    if r == 1 {
                        let before: Vec<String> = self.risk.iter().map(u8::to_string).collect();
                        self.push_log(format!(
                            "Risc: {} apoi 1 — ai pierdut pariul.",
                            if before.is_empty() {
                                "—".into()
                            } else {
                                before.join("+")
                            }
                        ));
                        self.end_turn();
                    } else {
                        self.risk.push(r);
                        self.push_log(format!("Risc: ai dat {r} (total {}).", self.risk_sum()));
                        if self.risk.len() >= 3 {
                            self.finish_risk(0);
                            self.end_turn();
                        } else {
                            self.phase = Phase::RiskDecide;
                        }
                    }
                }
                _ => {
                    // Judge
                    if self.judge_human() {
                        let why = self.kind.name();
                        self.award(0, why);
                        self.end_turn();
                    } else if self.retry[0] {
                        self.retry[0] = false;
                        self.push_log("Ai ratat — folosești a doua șansă: probă nouă.".into());
                        let k = self.kind;
                        self.choice = None;
                        self.recall.clear();
                        self.start_human(k);
                    } else {
                        self.push_log("Probă ratată.".into());
                        self.end_turn();
                    }
                }
            },
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        if self.tie_top {
            return Some(Outcome::Draw);
        }
        match self.winner {
            Some(0) => Some(Outcome::Win(Player::One)),
            Some(_) => Some(Outcome::Win(Player::Two)),
            None => Some(Outcome::Draw),
        }
    }

    fn evaluate(&self, p: Player) -> i32 {
        let base = 100 * self.lead_diff(0);
        let mut v = base;
        if self.phase == Phase::RiskRoll {
            // un zar în plus: valoare medie, fără să știm rezultatul
            v += 30;
        }
        match p {
            Player::One => v,
            Player::Two => -v,
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }

    fn render(&self) -> String {
        let mut s = String::new();
        s.push_str("Cursa probelor — pistă de 30 de câmpuri\n");
        s.push_str(&self.render_track());
        s.push_str("Legendă: N numărare, M memorie, D duel, R risc, L logică, O odihnă; cifrele = jucători.\n");
        for i in 0..self.n {
            let lap = (self.steps[i] as usize / TRACK + 1).min(LAPS);
            s.push_str(&format!(
                "{}{}: medalii {}/{MEDALS_TO_WIN} | tura {lap}/{LAPS} | câmp {}{}{}\n",
                if self.phase != Phase::Over && self.phase != Phase::Setup && self.cur == i {
                    "> "
                } else {
                    "  "
                },
                seat_name(i),
                self.medals[i],
                self.square(i),
                if self.retry[i] {
                    " | a doua șansă"
                } else {
                    ""
                },
                if self.finished(i) {
                    " | a terminat"
                } else {
                    ""
                },
            ));
        }
        for l in self
            .log
            .iter()
            .rev()
            .take(7)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
        {
            s.push_str(&format!("» {l}\n"));
        }
        s.push_str(&self.render_trial());
        match self.phase {
            Phase::Setup => s.push_str("Alege: jucatori 2 / jucatori 3 / jucatori 4.\n"),
            Phase::Roll => s.push_str("Tura ta: zar.\n"),
            Phase::DuelPick => {
                let o: Vec<String> = (1..self.n)
                    .map(|i| {
                        format!(
                            "duel {} ({}, {} medalii)",
                            i + 1,
                            seat_name(i),
                            self.medals[i]
                        )
                    })
                    .collect();
                s.push_str(&format!("Alege adversarul: {}.\n", o.join(", ")));
            }
            Phase::RiskDecide => {
                let d: Vec<String> = self.risk.iter().map(u8::to_string).collect();
                s.push_str(&format!(
                    "Risc: zaruri [{}] total {} (țintă {RISK_TARGET}, un 1 pierde). {}\n",
                    d.join(" "),
                    self.risk_sum(),
                    if self.risk.is_empty() {
                        "arunc."
                    } else if self.risk.len() < 3 {
                        "arunc sau opresc."
                    } else {
                        "opresc."
                    }
                ));
            }
            Phase::Over => {
                if self.tie_top {
                    s.push_str("Cursa s-a încheiat la egalitate (remiză).\n");
                } else if self.winner == Some(0) {
                    s.push_str("Ai câștigat cursa probelor! Câștigă Jucătorul 1.\n");
                } else {
                    s.push_str(&format!(
                        "Cursa s-a încheiat: câștigă {}.\n",
                        seat_name(self.winner.unwrap_or(1))
                    ));
                }
            }
            Phase::Answer => s.push_str("Răspunde: raspuns 1 / 2 / 3.\n"),
            Phase::Memorize | Phase::Recall => {}
            Phase::RobotTurn | Phase::Judge | Phase::DuelResolve | Phase::RiskRoll => {
                s.push_str("(urmează o mutare automată)\n");
            }
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Players(n) => format!("jucatori {n}"),
            Move::Roll => "zar".into(),
            Move::Answer(k) => format!("raspuns {}", k + 1),
            Move::Ready => "gata".into(),
            Move::Letter(i) => MEM_LETTERS[*i as usize].to_string(),
            Move::Duel(o) => format!("duel {}", o + 1),
            Move::RiskRoll => "arunc".into(),
            Move::RiskStop => "opresc".into(),
            Move::Resolve => "rezolv".into(),
            Move::Robot => "robot".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn play_random(mut g: CursaProbelor, seed: u64) -> CursaProbelor {
        let mut rng = Rng::new(seed);
        let mut n = 0;
        while g.outcome().is_none() {
            let mv = g.legal_moves();
            assert!(!mv.is_empty());
            let m = mv[rng.below(mv.len())];
            g.apply(&m);
            n += 1;
            assert!(n < 20_000, "joc fără sfârșit");
        }
        g
    }

    #[test]
    fn board_has_expected_mix_and_rest_start() {
        let g = CursaProbelor::with_seed(7);
        assert_eq!(g.board[0], Sq::Rest);
        for (sq, k) in BOARD_MIX {
            assert_eq!(g.board.iter().filter(|&&s| s == sq).count(), k, "{sq:?}");
        }
    }

    #[test]
    fn setup_chooses_players() {
        let mut g = CursaProbelor::with_seed(1);
        assert_eq!(g.legal_moves().len(), 3);
        g.apply(&Move::Players(4));
        assert_eq!(g.players(), 4);
        assert_eq!(g.legal_moves(), vec![Move::Roll]);
        assert!(g.render().contains("Robot 4"));
    }

    #[test]
    fn deterministic_from_seed() {
        let run = |s| {
            let g = CursaProbelor::with_players(s, 3).with_level(Level::Normal);
            let g = play_random(g, 5);
            (g.render(), g.medals.clone(), g.steps.clone())
        };
        assert_eq!(run(11), run(11));
        assert_ne!(run(11).0, run(12).0);
    }

    #[test]
    fn count_trial_has_unique_correct_option() {
        let mut rng = Rng::new(3);
        for _ in 0..300 {
            let t = gen_count(&mut rng);
            let c = t.correct_option().unwrap();
            let mut o = t.options.to_vec();
            o.sort();
            o.dedup();
            assert_eq!(o.len(), 3);
            assert!(t.options.iter().all(|&x| x > 0));
            assert!(c < 3);
        }
    }

    #[test]
    fn logic_trial_correct_and_positive() {
        let mut rng = Rng::new(4);
        for _ in 0..500 {
            let t = gen_logic(&mut rng);
            let Public::Logic { expr } = &t.public else {
                panic!()
            };
            assert_eq!(t.options[t.correct_option().unwrap()], expr.solve());
            assert!(t.options.iter().all(|&x| x > 0), "{:?}", t.options);
            assert!(!expr.text().is_empty());
        }
    }

    #[test]
    fn human_count_answer_flow() {
        let mut g = CursaProbelor::with_players(2, 2);
        g.start_human(Sq::Count);
        assert_eq!(g.to_move(), Player::One);
        let right = g.trial.as_ref().unwrap().correct_option().unwrap();
        g.apply(&Move::Answer(right as u8));
        // verdictul e o mutare forțată a lui Player::Two
        assert_eq!(g.to_move(), Player::Two);
        assert_eq!(g.legal_moves(), vec![Move::Resolve]);
        g.apply(&Move::Resolve);
        assert_eq!(g.medals(0), 1);
        assert_eq!(g.to_move(), Player::Two); // tura robotului
        assert_eq!(g.legal_moves(), vec![Move::Robot]);
    }

    #[test]
    fn wrong_answer_gives_no_medal_and_retry_gives_new_trial() {
        let mut g = CursaProbelor::with_players(2, 2);
        g.start_human(Sq::Logic);
        let right = g.trial.as_ref().unwrap().correct_option().unwrap();
        g.retry[0] = true;
        g.apply(&Move::Answer(((right + 1) % 3) as u8));
        g.apply(&Move::Resolve);
        assert_eq!(g.medals(0), 0);
        assert!(!g.retry[0]);
        assert_eq!(g.phase, Phase::Answer, "probă nouă după a doua șansă");
        let right = g.trial.as_ref().unwrap().correct_option().unwrap();
        g.apply(&Move::Answer(((right + 1) % 3) as u8));
        g.apply(&Move::Resolve);
        assert_eq!(g.medals(0), 0);
        assert_eq!(g.legal_moves(), vec![Move::Robot]);
    }

    #[test]
    fn memory_hidden_after_ready_and_judged() {
        let mut g = CursaProbelor::with_players(5, 2);
        g.start_human(Sq::Memory);
        assert!(g.render().contains("Memorează secvența"));
        g.apply(&Move::Ready);
        let r = g.render();
        assert!(!r.contains("Memorează secvența"));
        assert!(r.contains("Reproduce"));
        let seq = match &g.trial.as_ref().unwrap().public {
            Public::Memory { seq } => seq.clone(),
            _ => panic!(),
        };
        for (i, &e) in seq.iter().enumerate() {
            assert_eq!(g.to_move(), Player::One, "{i}");
            g.apply(&Move::Letter(e));
        }
        g.apply(&Move::Resolve);
        assert_eq!(g.medals(0), 1);
    }

    #[test]
    fn risk_rules() {
        let mut g = CursaProbelor::with_players(6, 2);
        g.start_human(Sq::Risk);
        assert_eq!(g.legal_moves(), vec![Move::RiskRoll]);
        // simulăm zaruri fără 1 direct în stare
        g.risk = vec![5, 4];
        assert!(g.legal_moves().contains(&Move::RiskStop));
        g.apply(&Move::RiskStop);
        assert_eq!(g.medals(0), 1);

        let mut g = CursaProbelor::with_players(6, 2);
        g.start_human(Sq::Risk);
        g.risk = vec![3, 2];
        g.apply(&Move::RiskStop);
        assert_eq!(g.medals(0), 0);

        // al treilea zar fără 1 încheie automat
        let mut g = CursaProbelor::with_players(8, 2);
        g.start_human(Sq::Risk);
        let mut guard = 0;
        while g.phase == Phase::RiskDecide || g.phase == Phase::RiskRoll {
            let m = g.legal_moves()[0];
            g.apply(&m);
            guard += 1;
            assert!(guard < 20);
        }
        assert!(g.risk.len() <= 3);
    }

    #[test]
    fn duel_awards_exactly_one_medal() {
        let mut g = CursaProbelor::with_players(9, 3);
        g.start_human(Sq::Duel);
        assert_eq!(g.legal_moves(), vec![Move::Duel(1), Move::Duel(2)]);
        g.apply(&Move::Duel(2));
        assert_eq!(g.to_move(), Player::Two);
        g.apply(&Move::Resolve);
        let total: u8 = g.medals.iter().sum();
        assert_eq!(total, 1);
        assert!(g.medals[0] + g.medals[2] == 1 && g.medals[1] == 0);
    }

    #[test]
    fn rest_grants_retry() {
        let mut g = CursaProbelor::with_players(10, 2);
        g.start_human(Sq::Rest);
        assert!(g.retry[0]);
        assert_eq!(g.legal_moves(), vec![Move::Robot]);
    }

    #[test]
    fn five_medals_win_immediately() {
        let mut g = CursaProbelor::with_players(12, 2);
        g.medals[0] = 4;
        g.start_human(Sq::Count);
        let right = g.trial.as_ref().unwrap().correct_option().unwrap();
        g.apply(&Move::Answer(right as u8));
        g.apply(&Move::Resolve);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
        assert!(g.legal_moves().is_empty());
    }

    #[test]
    fn three_laps_end_game_with_most_medals() {
        let mut g = CursaProbelor::with_players(13, 3);
        g.steps = vec![GOAL_STEPS, GOAL_STEPS, GOAL_STEPS];
        g.medals = vec![2, 3, 1];
        g.cur = 2;
        g.end_turn();
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
        assert_eq!(g.winner(), Some(1));

        let mut g = CursaProbelor::with_players(13, 3);
        g.steps = vec![GOAL_STEPS; 3];
        g.medals = vec![3, 3, 1];
        g.end_turn();
        assert_eq!(g.outcome(), Some(Outcome::Draw));

        let mut g = CursaProbelor::with_players(13, 3);
        g.steps = vec![GOAL_STEPS; 3];
        g.medals = vec![4, 3, 1];
        g.end_turn();
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn track_render_contains_all_squares_and_tokens() {
        let g = CursaProbelor::with_players(14, 4);
        let r = g.render();
        for i in 0..TRACK {
            assert!(r.contains(&format!("{:02}", i)), "{i}");
        }
        assert!(r.contains("Robot 4"));
        assert!(r.contains("Legendă"));
        // 4 jucători pe câmpul 0
        assert!(r.contains("1234"));
    }

    #[test]
    fn random_games_end_for_all_sizes() {
        for n in 2..=4 {
            for seed in 1..=25u64 {
                let g = play_random(
                    CursaProbelor::with_players(seed, n).with_level(Level::Normal),
                    seed,
                );
                assert!(g.outcome().is_some());
                assert!(g.medals.iter().all(|&m| m <= MEDALS_TO_WIN));
            }
        }
    }

    #[test]
    fn ai_games_end() {
        for seed in 1..=15u64 {
            let mut g = CursaProbelor::with_players(seed, 3).with_level(Level::Hard);
            let mut rng = Rng::new(seed ^ 77);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap();
                g.apply(&m);
                n += 1;
                assert!(n < 20_000);
            }
        }
    }

    /// Rata de succes a robotului pe fiecare tip de probă, pe nivel.
    fn robot_rates(l: Level) -> (usize, usize, usize) {
        let sk = skill(l);
        let mut rng = Rng::new(99);
        let (mut c, mut lg, mut m) = (0, 0, 0);
        for _ in 0..2000 {
            let t = gen_count(&mut rng);
            if Some(robot_choice(&t, &sk, &mut rng)) == t.correct_option() {
                c += 1;
            }
            let t = gen_logic(&mut rng);
            if Some(robot_choice(&t, &sk, &mut rng)) == t.correct_option() {
                lg += 1;
            }
            let t = gen_memory(&mut rng);
            if let Public::Memory { seq } = &t.public {
                if robot_recall(seq, &sk, &mut rng) == *seq {
                    m += 1;
                }
            }
        }
        (c, lg, m)
    }

    #[test]
    fn robot_skill_grows_with_level() {
        let e = robot_rates(Level::Easy);
        let n = robot_rates(Level::Normal);
        let h = robot_rates(Level::Hard);
        let x = robot_rates(Level::Expert);
        eprintln!("rate (numărare, logică, memorie)/2000: {e:?} {n:?} {h:?} {x:?}");
        assert!(e.0 < n.0 && n.0 < h.0 && h.0 < x.0);
        assert!(e.1 < n.1 && n.1 < h.1 && h.1 < x.1);
        assert!(e.2 < n.2 && n.2 < h.2 && h.2 < x.2);
        assert!(x.0 > 1700 && x.1 > 1800 && x.2 > 1800);
        assert!(e.0 < 1500 && e.2 < 700);
    }

    #[test]
    fn expert_robots_beat_easy_robots_in_medals() {
        // fiecare nivel joacă singur; Expert strânge medalii mai repede decât Easy
        let turns = |l: Level| -> u32 {
            let mut tot = 0;
            for seed in 1..=20u64 {
                let mut g = CursaProbelor::with_players(seed, 2).with_level(l);
                // omul „trece”: îl consumăm cu mutări aleatoare; contăm medaliile robotului
                let mut rng = Rng::new(seed);
                while g.outcome().is_none() {
                    let mv = g.legal_moves();
                    let m = mv[rng.below(mv.len())];
                    g.apply(&m);
                }
                tot += u32::from(g.medals(1));
            }
            tot
        };
        assert!(turns(Level::Expert) > turns(Level::Easy));
    }

    #[test]
    fn robot_uses_only_public_view() {
        // robot_choice primește doar Trial (public + variante); verificăm că nu depinde de
        // câmpuri ascunse: același Public/variante dau aceeași alegere la același seed de zgomot
        let mut rng = Rng::new(21);
        let t = gen_count(&mut rng);
        let sk = skill(Level::Hard);
        let a = robot_choice(&t, &sk, &mut Rng::new(5));
        let b = robot_choice(&t.clone(), &sk, &mut Rng::new(5));
        assert_eq!(a, b);
    }

    #[test]
    fn move_text_roundtrip() {
        let g = CursaProbelor::with_players(1, 3);
        for m in g.legal_moves() {
            assert_eq!(g.format_move(&m), "zar");
        }
        let g = CursaProbelor::with_seed(1);
        let texts: Vec<String> = g.legal_moves().iter().map(|m| g.format_move(m)).collect();
        assert_eq!(texts, vec!["jucatori 2", "jucatori 3", "jucatori 4"]);
    }
}
