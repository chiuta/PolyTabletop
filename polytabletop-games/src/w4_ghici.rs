//! Motor „descrie și ghicește” pentru jocurile cu indicii predefinite
//! (Schița, Trei Căi, Cuvinte Oprite, Dicționar Viu).
//!
//! Doi jucători, ca la jocurile de echipă: cel care **descrie** are un coleg-robot
//! care ghicește, iar **adversarul** poate „fura” rundea ghicind el primul.
//! La fiecare indiciu arătat:
//! 1. adversarul ghicește (`ghici N`) sau dă `pas`;
//! 2. dacă a ghicit greșit pierde un punct; dacă a nimerit, ia punctele rundei;
//! 3. altfel, colegul-robot (pas forțat `robot`) ghicește sau așteaptă după o
//!    euristică pe jetoanele indiciilor deja arătate (fără să vadă răspunsul).
//!
//! Punctaj (mod „o rundă = un cuvânt”): cu cât mai puține indicii, cu atât mai multe
//! puncte (`max_hints + 1 − indicii`). Faulurile (Cuvinte Oprite) costă 1 punct.
//! Mod „în rafală” (Dicționar Viu): fiecare tură are un buget de acțiuni
//! (indiciu sau `treci`), cuvintele curg, fiecare ghicit = 1 punct.

use crate::cards::{handover_banner, shuffle, Viewer};
use crate::w4_comun::*;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::fmt::Write;

/// Un indiciu predefinit al unui cuvânt.
#[derive(Clone, Debug)]
pub struct Hint {
    /// Textul afișat, câte unul pe fiecare mod al jocului.
    pub texts: Vec<String>,
    /// Jetoanele pe care le „aude” ghicitorul.
    pub tokens: Vec<String>,
    /// Folosește un cuvânt interzis (fault).
    pub faul: bool,
}

#[derive(Clone, Debug)]
pub struct Item {
    pub word: String,
    pub cat: String,
    /// Linie afișată descriptorului (de ex. cuvintele interzise).
    pub note: String,
    pub hints: Vec<Hint>,
    /// Jetoanele pe care ghicitorul le asociază cu acest cuvânt.
    pub profile: Vec<String>,
}

pub struct Cfg {
    pub title: &'static str,
    pub items: fn() -> &'static [Item],
    pub modes: &'static [&'static str],
    pub max_hints: usize,
    /// Runde (un cuvânt) sau ture (în rafală).
    pub rounds: usize,
    /// `Some(buget)` = mod în rafală.
    pub burst: Option<usize>,
    pub bot_margin: i32,
    pub opp_margin: i32,
    pub faul_pen: bool,
    /// Arată descriptorului cât de „folositor” e fiecare indiciu.
    pub show_util: bool,
    pub rules: &'static str,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Show,
    Steal,
    VerifySteal,
    Bot,
    Over,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Show(usize),
    Skip,
    Guess(usize),
    Pass,
    Verify,
    Bot,
}

#[derive(Clone)]
pub struct Engine {
    cfg: &'static Cfg,
    items: &'static [Item],
    rng: Rng,
    order: Vec<usize>,
    next: usize,
    round: usize,
    secret: usize,
    cands: [usize; 6],
    spos: usize,
    shown: Vec<usize>,
    phase: Phase,
    pending: usize,
    budget: usize,
    pts: [i32; 2],
    log: Vec<String>,
    heur: Heur,
    viewer: Viewer,
    last: Player,
}

impl Engine {
    pub fn new(cfg: &'static Cfg) -> Self {
        Self::with_seed(cfg, Rng::from_clock().next_u64())
    }

    pub fn with_seed(cfg: &'static Cfg, seed: u64) -> Self {
        let items = (cfg.items)();
        let mut rng = Rng::new(seed);
        let mut order: Vec<usize> = (0..items.len()).collect();
        shuffle(&mut order, &mut rng);
        let mut e = Engine {
            cfg,
            items,
            rng,
            order,
            next: 0,
            round: 0,
            secret: 0,
            cands: [0; 6],
            spos: 0,
            shown: Vec::new(),
            phase: Phase::Show,
            pending: 0,
            budget: cfg.burst.unwrap_or(0),
            pts: [0, 0],
            log: Vec::new(),
            heur: Heur::default(),
            viewer: Viewer::Fixed(Player::One),
            last: Player::One,
        };
        e.new_word();
        e
    }

    pub fn with_viewer(mut self, v: Viewer) -> Self {
        self.viewer = v;
        self
    }

    pub fn hot_seat(self) -> Self {
        self.with_viewer(Viewer::HotSeat)
    }

    pub fn scores(&self) -> (i32, i32) {
        (self.pts[0], self.pts[1])
    }

    fn desc(&self) -> Player {
        if self.round.is_multiple_of(2) {
            Player::One
        } else {
            Player::Two
        }
    }

    fn opp(&self) -> Player {
        self.desc().other()
    }

    fn item(&self) -> &'static Item {
        &self.items[self.secret]
    }

    fn mode(&self) -> usize {
        self.round % self.cfg.modes.len()
    }

    fn max_avail(&self) -> usize {
        self.cfg.max_hints.min(self.item().hints.len())
    }

    fn new_word(&mut self) {
        let n = self.items.len();
        self.secret = self.order[self.next % n];
        self.next += 1;
        let cat = &self.items[self.secret].cat;
        let mut same: Vec<usize> = (0..n)
            .filter(|&i| i != self.secret && self.items[i].cat == *cat)
            .collect();
        shuffle(&mut same, &mut self.rng);
        same.truncate(5);
        while same.len() < 5 {
            let r = self.rng.below(n);
            if r != self.secret && !same.contains(&r) {
                same.push(r);
            }
        }
        let mut c: Vec<usize> = same;
        c.push(self.secret);
        shuffle(&mut c, &mut self.rng);
        self.spos = c.iter().position(|&x| x == self.secret).unwrap();
        self.cands.copy_from_slice(&c);
        self.shown.clear();
        self.phase = Phase::Show;
    }

    fn end_word(&mut self) {
        self.shown.clear();
        if let Some(b) = self.cfg.burst {
            if self.budget == 0 {
                self.round += 1;
                self.budget = b;
            }
        } else {
            self.round += 1;
        }
        if self.round >= self.cfg.rounds {
            self.phase = Phase::Over;
        } else {
            self.new_word();
        }
    }

    fn lack(&self, tok: &str) -> i32 {
        self.cands
            .iter()
            .filter(|&&c| !self.items[c].profile.iter().any(|t| t == tok))
            .count() as i32
    }

    /// Scorul fiecărui candidat după indiciile arătate (informație publică).
    fn cand_scores(&self) -> [i32; 6] {
        let mut s = [0i32; 6];
        let it = self.item();
        for &h in &self.shown {
            for tok in &it.hints[h].tokens {
                let w = 100 + 60 * self.lack(tok);
                for (i, &c) in self.cands.iter().enumerate() {
                    if self.items[c].profile.iter().any(|t| t == tok) {
                        s[i] += w;
                    }
                }
            }
        }
        s
    }

    /// Cât de bine deosebește indiciul `h` răspunsul de ceilalți candidați
    /// (cunoscut descriptorului, care știe răspunsul și lista de candidați).
    fn distinct(&self, h: usize) -> i32 {
        let it = self.item();
        let hint = &it.hints[h];
        let mut v = 0;
        for tok in &hint.tokens {
            if it.profile.contains(tok) {
                v += 20 * self.lack(tok);
            }
        }
        if hint.faul && self.cfg.faul_pen {
            v -= 1000;
        }
        v
    }

    fn margin_for(s: &[i32; 6], k: usize) -> i32 {
        let other = s
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != k)
            .map(|(_, &v)| v)
            .max()
            .unwrap_or(0);
        s[k] - other
    }

    fn award(&mut self, p: Player, v: i32) {
        self.pts[p.idx()] += v;
    }

    fn round_points(&self) -> i32 {
        if self.cfg.burst.is_some() {
            1
        } else {
            (self.cfg.max_hints + 1 - self.shown.len()) as i32
        }
    }
}

impl Game for Engine {
    type Move = Move;

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = match who {
            Some(p) => Viewer::Fixed(p),
            None => Viewer::HotSeat,
        };
    }

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Show => self.desc(),
            Phase::Steal => self.opp(),
            Phase::VerifySteal | Phase::Bot | Phase::Over => RESOLVER,
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        match self.phase {
            Phase::Show => {
                let mut v: Vec<Move> = (0..self.item().hints.len())
                    .filter(|h| !self.shown.contains(h))
                    .map(Move::Show)
                    .collect();
                if self.cfg.burst.is_some() {
                    v.push(Move::Skip);
                }
                v
            }
            Phase::Steal => {
                let mut v: Vec<Move> = (0..6).map(Move::Guess).collect();
                v.push(Move::Pass);
                v
            }
            Phase::VerifySteal => vec![Move::Verify],
            Phase::Bot => vec![Move::Bot],
            Phase::Over => Vec::new(),
        }
    }

    fn apply(&mut self, m: &Move) {
        let mover = self.to_move();
        if !matches!(m, Move::Verify | Move::Bot) {
            self.last = mover;
        }
        self.heur.clear();
        match *m {
            Move::Show(h) => {
                let v = self.distinct(h);
                self.heur.set(mover, v);
                let hint = &self.item().hints[h];
                if hint.faul && self.cfg.faul_pen {
                    self.award(mover, -1);
                    self.log.push(format!(
                        "FAUL! Indiciul «{}» conține un cuvânt interzis: −1 pentru Jucătorul {}.",
                        hint.texts[self.mode()],
                        mover.idx() + 1
                    ));
                }
                self.shown.push(h);
                if self.cfg.burst.is_some() {
                    self.budget = self.budget.saturating_sub(1);
                }
                self.phase = Phase::Steal;
            }
            Move::Skip => {
                self.budget = self.budget.saturating_sub(1);
                self.log
                    .push(format!("Cuvântul «{}» a fost sărit.", self.item().word));
                self.end_word();
            }
            Move::Guess(k) => {
                let s = self.cand_scores();
                let v = Self::margin_for(&s, k) - self.cfg.opp_margin;
                self.heur.set(mover, v);
                self.pending = k;
                self.phase = Phase::VerifySteal;
            }
            Move::Pass => {
                self.heur.set(mover, 0);
                self.phase = Phase::Bot;
            }
            Move::Verify => {
                let o = self.opp();
                if self.pending == self.spos {
                    let p = self.round_points();
                    self.award(o, p);
                    self.log.push(format!(
                        "Jucătorul {} a „furat” rundea: era «{}» (+{p}).",
                        o.idx() + 1,
                        self.item().word
                    ));
                    self.end_word();
                } else {
                    self.award(o, -1);
                    self.log.push(format!(
                        "Jucătorul {} a ghicit greșit («{}»): −1.",
                        o.idx() + 1,
                        self.items[self.cands[self.pending]].word
                    ));
                    self.phase = Phase::Bot;
                }
            }
            Move::Bot => {
                let mut s = self.cand_scores();
                for v in s.iter_mut() {
                    *v += self.rng.below(60) as i32;
                }
                let mut idx: Vec<usize> = (0..6).collect();
                idx.sort_by_key(|&i| std::cmp::Reverse(s[i]));
                let gap = s[idx[0]] - s[idx[1]];
                let must = self.shown.len() >= self.max_avail()
                    || (self.cfg.burst.is_some() && self.budget == 0);
                if must || gap >= self.cfg.bot_margin {
                    let d = self.desc();
                    let g = idx[0];
                    if g == self.spos {
                        let p = self.round_points();
                        self.award(d, p);
                        self.log.push(format!(
                            "Colegul-robot al Jucătorului {} ghicește «{}» (+{p}).",
                            d.idx() + 1,
                            self.item().word
                        ));
                    } else {
                        self.log.push(format!(
                            "Colegul-robot al Jucătorului {} greșește: zice «{}», era «{}».",
                            d.idx() + 1,
                            self.items[self.cands[g]].word,
                            self.item().word
                        ));
                    }
                    self.end_word();
                } else {
                    self.phase = Phase::Show;
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase == Phase::Over {
            Some(result_of(&self.pts))
        } else {
            None
        }
    }

    fn evaluate(&self, p: Player) -> i32 {
        100 * diff(&self.pts, p) + self.heur.for_player(p)
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }

    fn render(&self) -> String {
        let mut s = String::new();
        let who = match self.viewer {
            Viewer::Fixed(p) => p,
            Viewer::HotSeat => match self.phase {
                Phase::VerifySteal | Phase::Bot | Phase::Over => self.last,
                _ => self.to_move(),
            },
        };
        if let Viewer::HotSeat = self.viewer {
            if matches!(self.phase, Phase::Show | Phase::Steal) && who != self.last {
                s.push_str(&handover_banner(who));
            }
        }
        let _ = writeln!(
            s,
            "{} — {} {}/{} | puncte: J1 {}, J2 {}",
            self.cfg.title,
            if self.cfg.burst.is_some() {
                "tura"
            } else {
                "runda"
            },
            (self.round + 1).min(self.cfg.rounds),
            self.cfg.rounds,
            self.pts[0],
            self.pts[1]
        );
        if self.phase == Phase::Over {
            s.push_str("Joc încheiat.\n");
            s.push_str(&log_tail(&self.log, 6));
            return s;
        }
        let d = self.desc();
        let _ = writeln!(
            s,
            "Descrie: Jucătorul {} (cu colegul-robot) | Poate fura: Jucătorul {}{}",
            d.idx() + 1,
            d.other().idx() + 1,
            if self.cfg.burst.is_some() {
                format!(" | buget: {} acțiuni", self.budget)
            } else {
                String::new()
            }
        );
        let _ = writeln!(s, "Mod: {}", self.cfg.modes[self.mode()]);
        s.push_str("Candidați:");
        for (i, &c) in self.cands.iter().enumerate() {
            let _ = write!(s, "  {}) {}", i + 1, self.items[c].word);
        }
        s.push('\n');
        let it = self.item();
        if !self.shown.is_empty() {
            s.push_str("Indicii arătate:\n");
            for &h in &self.shown {
                let hint = &it.hints[h];
                let _ = writeln!(
                    s,
                    "  • {}{}",
                    hint.texts[self.mode()],
                    if hint.faul && self.cfg.faul_pen {
                        "  [FAUL]"
                    } else {
                        ""
                    }
                );
            }
        }
        if who == d && self.phase == Phase::Show {
            let _ = writeln!(s, "Cuvântul tău: {}", it.word.to_uppercase());
            if !it.note.is_empty() {
                let _ = writeln!(s, "{}", it.note);
            }
            s.push_str("Indicii disponibile (arata N):\n");
            for (h, hint) in it.hints.iter().enumerate() {
                if self.shown.contains(&h) {
                    continue;
                }
                let util = if self.cfg.show_util {
                    let d = (self.distinct(h) / 20).clamp(0, 5);
                    format!("  [utilitate {}/5]", d)
                } else {
                    String::new()
                };
                let _ = writeln!(s, "  {}) {}{}", h + 1, hint.texts[self.mode()], util);
            }
        }
        s.push_str(&log_tail(&self.log, 3));
        match self.phase {
            Phase::Show => {
                let _ = writeln!(s, "La mutare: Jucătorul {} (descrie)", d.idx() + 1);
            }
            Phase::Steal => {
                let _ = writeln!(
                    s,
                    "La mutare: Jucătorul {} — ghici N (fură) sau pas",
                    d.other().idx() + 1
                );
            }
            _ => s.push_str("Pas de decontare (arbitru).\n"),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Show(h) => format!("arata {}", h + 1),
            Move::Skip => "treci".into(),
            Move::Guess(k) => format!("ghici {}", k + 1),
            Move::Pass => "pas".into(),
            Move::Verify => "verifica".into(),
            Move::Bot => "robot".into(),
        }
    }
}

/// Construiește un indiciu-text cu verificarea cuvintelor interzise.
pub fn text_hint(text: &str, forbidden: &[String]) -> Hint {
    let faul = text_is_faul(text, forbidden);
    Hint {
        texts: vec![text.to_string()],
        tokens: tokens(text),
        faul,
    }
}

/// Un cuvânt din text începe cu rădăcina unui cuvânt interzis (sau e identic cu el, dacă e scurt).
pub fn text_is_faul(text: &str, forbidden: &[String]) -> bool {
    let f = fold(text);
    for w in f
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
    {
        for b in forbidden {
            let b = fold(b);
            if b.chars().count() <= 4 {
                if w == b {
                    return true;
                }
            } else {
                let pre: String = b.chars().take(5).collect();
                if w.starts_with(&pre) {
                    return true;
                }
            }
        }
    }
    false
}

/// Testează invarianții comuni ai motorului pentru o configurație.
#[cfg(test)]
pub fn check_cfg(cfg: &'static Cfg) {
    let items = (cfg.items)();
    assert!(items.len() >= 200, "{}: {} intrări", cfg.title, items.len());
    let mut words: Vec<String> = items.iter().map(|i| fold(&i.word)).collect();
    words.sort();
    words.dedup();
    assert_eq!(words.len(), items.len(), "cuvinte duplicate");
    for it in items {
        assert!(it.hints.len() >= cfg.max_hints.min(3), "{}", it.word);
        assert!(!it.profile.is_empty(), "{}", it.word);
        assert!(it.hints.iter().any(|h| !h.tokens.is_empty()), "{}", it.word);
        for h in &it.hints {
            assert_eq!(h.texts.len(), cfg.modes.len());
        }
    }
    for seed in 1..=6u64 {
        let mut g = Engine::with_seed(cfg, seed);
        let mut rng = Rng::new(seed + 50);
        let mut n = 0;
        while Game::outcome(&g).is_none() && n < 4000 {
            let mv = Game::legal_moves(&g);
            assert!(!mv.is_empty());
            let m = mv[rng.below(mv.len())];
            g.apply(&m);
            n += 1;
        }
        assert!(Game::outcome(&g).is_some(), "joc nesfârșit");
        assert!(!Game::render(&g).is_empty());
    }
}

/// AI-ul nu citește răspunsul în fereastra de furt: valorile mutărilor nu depind
/// de poziția răspunsului printre candidați.
#[cfg(test)]
pub fn check_no_peek(cfg: &'static Cfg) {
    for seed in 1..=5u64 {
        let mut g = Engine::with_seed(cfg, seed);
        let m = Game::legal_moves(&g)[0];
        g.apply(&m);
        assert_eq!(g.phase, Phase::Steal);
        let mut h = g.clone();
        h.spos = (h.spos + 1) % 6;
        let p = Game::to_move(&g);
        for m in Game::legal_moves(&g) {
            let mut a = g.clone();
            let mut b = h.clone();
            a.apply(&m);
            b.apply(&m);
            assert_eq!(Game::evaluate(&a, p), Game::evaluate(&b, p));
        }
    }
}

#[cfg(test)]
mod rules_tests {
    use super::*;

    fn first_phase_steal(cfg: &'static Cfg, seed: u64) -> Engine {
        let mut g = Engine::with_seed(cfg, seed);
        g.apply(&Move::Show(0));
        g
    }

    #[test]
    fn steal_right_scores_for_the_opponent() {
        let mut g = first_phase_steal(&crate::schita::CFG, 4);
        assert_eq!(Game::to_move(&g), Player::Two);
        let spos = g.spos;
        g.apply(&Move::Guess(spos));
        assert_eq!(Game::to_move(&g), RESOLVER);
        g.apply(&Move::Verify);
        // 1 indiciu din max 5 → 5 puncte pentru adversar (J2)
        assert_eq!(g.scores(), (0, 5));
        assert_eq!(g.round, 1);
        assert_eq!(Game::to_move(&g), Player::Two); // runda 2: descrie J2
    }

    #[test]
    fn steal_wrong_costs_a_point_then_the_robot_plays() {
        let mut g = first_phase_steal(&crate::schita::CFG, 4);
        let wrong = (g.spos + 1) % 6;
        g.apply(&Move::Guess(wrong));
        g.apply(&Move::Verify);
        assert_eq!(g.scores(), (0, -1));
        assert_eq!(g.phase, Phase::Bot);
        g.apply(&Move::Bot);
        // robotul fie ghicește (runda trece), fie așteaptă (indiciul următor)
        assert!(g.round == 1 || g.phase == Phase::Show);
    }

    #[test]
    fn faul_hint_costs_the_describer() {
        let it = (crate::oprite::CFG.items)();
        let mut g = Engine::with_seed(&crate::oprite::CFG, 9);
        let fh = it[g.secret].hints.iter().position(|h| h.faul).unwrap();
        g.apply(&Move::Show(fh));
        assert_eq!(g.scores().0, -1);
        assert!(Game::render(&g).contains("FAUL"));
    }

    #[test]
    fn robot_must_guess_when_hints_run_out() {
        let mut g = Engine::with_seed(&crate::schita::CFG, 12);
        for h in 0..5 {
            g.apply(&Move::Show(h));
            g.apply(&Move::Pass);
            if g.round > 0 {
                return; // robotul a ghicit mai devreme
            }
            if h < 4 {
                // robotul poate aștepta
                g.apply(&Move::Bot);
                if g.round > 0 {
                    return;
                }
            }
        }
        g.apply(&Move::Bot);
        assert_eq!(g.round, 1, "după 5 indicii runda trebuie să se încheie");
    }

    #[test]
    fn burst_turn_ends_when_budget_is_spent() {
        let cfg = &crate::dictionar::CFG;
        let mut g = Engine::with_seed(cfg, 5);
        let mut skips = 0;
        while g.round == 0 {
            g.apply(&Move::Skip);
            skips += 1;
            assert!(skips <= 9);
        }
        assert_eq!(skips, 9);
        assert_eq!(g.budget, 9);
        assert_eq!(Game::to_move(&g), Player::Two);
    }

    #[test]
    fn mode_cycles_in_activity() {
        let cfg = &crate::treicai::CFG;
        let mut g = Engine::with_seed(cfg, 3);
        let mut seen = [false; 3];
        for _ in 0..3 {
            seen[g.mode()] = true;
            g.phase = Phase::Over;
            g.round += 1;
            g.new_word();
        }
        assert_eq!(seen, [true; 3]);
    }
}
