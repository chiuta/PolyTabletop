//! SFATUL NEGUSTORILOR — joc de alegeri și legi cu roluri politice ascunse (variantă originală).
//!
//! Cinci locuri: tu (locul 1) și patru roboți (masa, `Player::Two`). Trei sunt
//! **Negustori** (cinstiți), unul e **Cămătarul** (rol-cheie) și unul **Complicele**.
//! Complicele și Cămătarul se cunosc; negustorii nu știu nimic.
//! Fiecare rundă: Președintele nominalizează un Cancelar (nu pe cel anterior), toți
//! votează da/nu. Dacă trece: Președintele trage 3 legi, aruncă una, Cancelarul
//! promulgă una din cele 2 rămase. Pachetul are 6 legi **P**rosperitate și 11 legi
//! **M**onopol. Câștigă negustorii cu 5 P sau executând Cămătarul; câștigă
//! conspiratorii cu 6 M sau dacă Cămătarul devine Cancelar după 3 M. Trei alegeri
//! respinse la rând: se promulgă legea din vârful pachetului. Puteri: la a 3-a lege M
//! Președintele vede următoarele 3 legi; la a 4-a și a 5-a execută un jucător.
//! Simplificări: fără vetou, fără investigarea loialității, 5 jucători fix.

use crate::w5_util::{pick_weighted, shuffle_vec, trim_log};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: usize = 5;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Role {
    Merchant,
    Accomplice,
    Usurer,
}

impl Role {
    fn name(self) -> &'static str {
        match self {
            Role::Merchant => "Negustor",
            Role::Accomplice => "Complice",
            Role::Usurer => "Cămătar",
        }
    }
    fn evil(self) -> bool {
        self != Role::Merchant
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Phase {
    Nominate,
    NominateTable,
    Vote,
    VoteTable,
    PresHuman,
    LegisTable,
    ChanHuman,
    ExecHuman,
    ExecTable,
    Over,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Nominate(u8),
    Yes,
    No,
    Discard(char),
    Enact(char),
    Execute(u8),
    Table,
}

#[derive(Clone)]
pub struct Consiliul {
    /// Whose private info `render` shows (`None` = the mover's, hot-seat).
    viewer: Option<Player>,
    roles: Vec<Role>,
    alive: Vec<bool>,
    deck: Vec<char>,
    discard: Vec<char>,
    hand: Vec<char>,
    enacted_p: u32,
    enacted_m: u32,
    president: usize,
    chancellor: Option<usize>,
    prev_chan: Option<usize>,
    tracker: u32,
    phase: Phase,
    human_vote: Option<bool>,
    sus: Vec<i32>,
    rng: Rng,
    log: Vec<String>,
    winner_merchants: Option<bool>,
}

impl Default for Consiliul {
    fn default() -> Self {
        Self::new()
    }
}

impl Consiliul {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut roles = vec![
            Role::Merchant,
            Role::Merchant,
            Role::Merchant,
            Role::Accomplice,
            Role::Usurer,
        ];
        shuffle_vec(&mut roles, &mut rng);
        let mut deck: Vec<char> = "PPPPPPMMMMMMMMMMM".chars().collect();
        shuffle_vec(&mut deck, &mut rng);
        let president = rng.below(N);
        let mut g = Consiliul {
            viewer: None,
            roles,
            alive: vec![true; N],
            deck,
            discard: Vec::new(),
            hand: Vec::new(),
            enacted_p: 0,
            enacted_m: 0,
            president,
            chancellor: None,
            prev_chan: None,
            tracker: 0,
            phase: Phase::Nominate,
            human_vote: None,
            sus: vec![0; N],
            rng,
            log: Vec::new(),
            winner_merchants: None,
        };
        let r = g.roles[0];
        g.say(format!("Rolul tău: {}.", r.name()));
        if r.evil() {
            let other: Vec<String> = (1..N)
                .filter(|&i| g.roles[i].evil())
                .map(|i| format!("locul {} ({})", i + 1, g.roles[i].name()))
                .collect();
            g.say(format!("Colegul tău: {}.", other.join(", ")));
        }
        g.begin_round();
        g
    }

    pub fn roles(&self) -> &[Role] {
        &self.roles
    }
    pub fn phase(&self) -> Phase {
        self.phase
    }
    pub fn enacted(&self) -> (u32, u32) {
        (self.enacted_p, self.enacted_m)
    }
    /// Număr total de legi: pachet + aruncate + mână + promulgate (trebuie să rămână 17).
    pub fn total_laws(&self) -> usize {
        self.deck.len()
            + self.discard.len()
            + self.hand.len()
            + (self.enacted_p + self.enacted_m) as usize
    }

    fn say(&mut self, s: String) {
        self.log.push(s);
        trim_log(&mut self.log, 14);
    }

    fn alive_n(&self) -> usize {
        self.alive.iter().filter(|&&a| a).count()
    }

    fn next_president(&mut self) {
        let mut p = self.president;
        loop {
            p = (p + 1) % N;
            if self.alive[p] {
                break;
            }
        }
        self.president = p;
    }

    fn begin_round(&mut self) {
        self.chancellor = None;
        self.human_vote = None;
        self.phase = if self.president == 0 {
            Phase::Nominate
        } else {
            Phase::NominateTable
        };
    }

    fn draw(&mut self) -> char {
        if self.deck.len() < 3 {
            let mut d = std::mem::take(&mut self.discard);
            self.deck.append(&mut d);
            shuffle_vec(&mut self.deck, &mut self.rng);
        }
        self.deck.pop().unwrap()
    }

    fn eligible(&self) -> Vec<usize> {
        (0..N)
            .filter(|&j| self.alive[j] && j != self.president && Some(j) != self.prev_chan)
            .collect()
    }

    fn robot_nominate(&mut self) -> usize {
        let me = self.president;
        let cand = self.eligible();
        let w: Vec<u32> = cand
            .iter()
            .map(|&j| {
                let mut s = 10 - self.sus[j] * 2;
                match self.roles[me] {
                    Role::Accomplice => {
                        if self.roles[j] == Role::Usurer && self.enacted_m < 3 {
                            s += 6;
                        }
                    }
                    Role::Usurer => {
                        if self.roles[j] == Role::Accomplice {
                            s += 2;
                        }
                    }
                    Role::Merchant => {}
                }
                s.max(1) as u32
            })
            .collect();
        cand[pick_weighted(&mut self.rng, &w)]
    }

    fn robot_vote(&mut self, i: usize) -> bool {
        let p = self.president;
        let c = self.chancellor.unwrap();
        let bad = self.sus[p] + self.sus[c];
        let v = match self.roles[i] {
            Role::Merchant => {
                if self.enacted_m >= 3 {
                    self.sus[c] <= 0 && bad <= 1
                } else {
                    bad <= 1
                }
            }
            Role::Accomplice => {
                self.roles[p].evil() || self.roles[c].evil() || (bad <= 0 && self.enacted_m < 3)
            }
            Role::Usurer => {
                self.roles[p].evil() || self.roles[c].evil() || (bad <= 0 && self.enacted_m < 3)
            }
        };
        if self.tracker >= 2 && !self.roles[i].evil() {
            return true;
        }
        if self.rng.below(10) == 0 {
            !v
        } else {
            v
        }
    }

    fn win(&mut self, merchants: bool, why: &str) {
        self.say(why.to_string());
        self.winner_merchants = Some(merchants);
        self.phase = Phase::Over;
    }

    fn enact(&mut self, c: char) {
        if c == 'P' {
            self.enacted_p += 1;
        } else {
            self.enacted_m += 1;
        }
        let (p, ch) = (self.president, self.chancellor);
        self.say(format!(
            "Legea {} e promulgată (P {} / M {}).",
            if c == 'P' { "Prosperitate" } else { "Monopol" },
            self.enacted_p,
            self.enacted_m
        ));
        if let Some(ch) = ch {
            let d = if c == 'M' { 1 } else { -1 };
            self.sus[p] += d;
            self.sus[ch] += d;
        }
        if self.enacted_p >= 5 {
            self.win(true, "Cinci legi Prosperitate: negustorii câștigă.");
            return;
        }
        if self.enacted_m >= 6 {
            self.win(false, "Șase legi Monopol: conspiratorii câștigă.");
            return;
        }
        self.next_president_after_law(c);
    }

    fn next_president_after_law(&mut self, c: char) {
        // puterile se declanșează doar pentru legi M promulgate de un guvern ales
        let elected = self.chancellor.is_some();
        if c == 'M' && elected {
            match self.enacted_m {
                3 => {
                    let top: String = (0..3.min(self.deck.len()))
                        .map(|k| self.deck[self.deck.len() - 1 - k])
                        .collect();
                    if self.president == 0 {
                        self.say(format!("Vezi următoarele legi din pachet: {top}."));
                    }
                }
                4 | 5 => {
                    self.phase = if self.president == 0 {
                        Phase::ExecHuman
                    } else {
                        Phase::ExecTable
                    };
                    return;
                }
                _ => {}
            }
        }
        self.finish_round();
    }

    fn finish_round(&mut self) {
        self.prev_chan = self.chancellor;
        self.next_president();
        self.begin_round();
    }

    fn robot_execute(&mut self) -> usize {
        let me = self.president;
        let cand: Vec<usize> = (0..N).filter(|&j| self.alive[j] && j != me).collect();
        let w: Vec<u32> = cand
            .iter()
            .map(|&j| {
                let s = match self.roles[me] {
                    Role::Merchant => self.sus[j] + 3,
                    // conspiratorii ucid un negustor „de încredere”, niciodată pe Cămătar
                    _ => {
                        if self.roles[j] == Role::Merchant {
                            5 - self.sus[j]
                        } else {
                            0
                        }
                    }
                };
                (s.max(0) as u32) + 1
            })
            .collect();
        cand[pick_weighted(&mut self.rng, &w)]
    }

    fn do_execute(&mut self, t: usize) {
        self.alive[t] = false;
        self.say(format!(
            "Locul {} execută locul {}.",
            self.president + 1,
            t + 1
        ));
        if self.roles[t] == Role::Usurer {
            self.win(true, "Cămătarul a fost executat: negustorii câștigă.");
            return;
        }
        self.say(format!("Locul {} nu era Cămătarul.", t + 1));
        if !self.alive[0] {
            self.say("Ai fost executat; urmărești masa.".into());
        }
        self.finish_round();
    }

    fn pres_discard_robot(&mut self) {
        let me = self.president;
        let evil = self.roles[me].evil();
        // negustorii aruncă un M dacă există; conspiratorii aruncă un P dacă există
        let want = if evil { 'P' } else { 'M' };
        let idx = self
            .hand
            .iter()
            .position(|&c| c == want)
            .unwrap_or(self.rng.below(3));
        let c = self.hand.remove(idx);
        self.discard.push(c);
    }

    fn chan_choice_robot(&mut self) -> char {
        let ch = self.chancellor.unwrap();
        let evil = self.roles[ch].evil();
        let pref = if evil { 'M' } else { 'P' };
        // Cămătarul se poartă ca un negustor până la 3 M
        let pref = if self.roles[ch] == Role::Usurer && self.enacted_m < 3 {
            'P'
        } else {
            pref
        };
        if self.hand.contains(&pref) {
            pref
        } else {
            self.hand[0]
        }
    }

    fn take_enacted(&mut self, c: char) {
        let idx = self.hand.iter().position(|&x| x == c).unwrap();
        self.hand.remove(idx);
        let mut rest = std::mem::take(&mut self.hand);
        self.discard.append(&mut rest);
        self.enact(c);
    }
}

impl Consiliul {
    /// Normal rendering (human = J1's point of view).
    fn render_main(&self) -> String {
        let mut s = format!(
            "SFATUL NEGUSTORILOR — P {}/5, M {}/6, respingeri {}/3 — președinte: locul {}\n",
            self.enacted_p,
            self.enacted_m,
            self.tracker,
            self.president + 1
        );
        s.push_str(&format!("Rolul tău: {}\n", self.roles[0].name()));
        let st: Vec<String> = (0..N)
            .map(|i| format!("{}{}", i + 1, if self.alive[i] { "" } else { "†" }))
            .collect();
        s.push_str(&format!("Locuri: {}\n", st.join(" ")));
        match self.phase {
            Phase::Nominate => s.push_str("Nominalizezi un Cancelar.\n"),
            Phase::Vote => s.push_str("Votezi da/nu pentru guvern.\n"),
            Phase::PresHuman => s.push_str(&format!(
                "Legile tale: {} — arunci una.\n",
                self.hand.iter().collect::<String>()
            )),
            Phase::ChanHuman => s.push_str(&format!(
                "Legile primite: {} — promulgi una.\n",
                self.hand.iter().collect::<String>()
            )),
            Phase::ExecHuman => s.push_str("Execuți un jucător.\n"),
            _ => {}
        }
        for l in &self.log {
            s.push_str(l);
            s.push('\n');
        }
        if let Some(m) = self.winner_merchants {
            s.push_str(if m {
                "Negustorii au câștigat.\n"
            } else {
                "Conspiratorii au câștigat.\n"
            });
            let all: Vec<String> = (0..N)
                .map(|i| format!("{}={}", i + 1, self.roles[i].name()))
                .collect();
            s.push_str(&format!("Roluri: {}\n", all.join(", ")));
        }
        s
    }

    /// Reveal-all rendering, appended when `set_viewer(Some(Two))` (second human inspects).
    fn god_view(&self) -> String {
        let mut s =
            String::from("--- VEDERE TOTALĂ (J2): toate rolurile și informațiile ascunse ---\n");
        let all: Vec<String> = (0..N)
            .map(|i| format!("{}={}", i + 1, self.roles[i].name()))
            .collect();
        s.push_str(&format!("Roluri: {}\n", all.join(", ")));
        s.push_str(&format!(
            "Legile din mână: {}\n",
            self.hand.iter().collect::<String>()
        ));
        s
    }
}

impl Game for Consiliul {
    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }
    type Move = Move;

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Nominate
            | Phase::Vote
            | Phase::PresHuman
            | Phase::ChanHuman
            | Phase::ExecHuman => Player::One,
            Phase::Over => Player::One,
            _ => Player::Two,
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        match self.phase {
            Phase::Nominate => self
                .eligible()
                .iter()
                .map(|&j| Move::Nominate(j as u8))
                .collect(),
            Phase::Vote => vec![Move::Yes, Move::No],
            Phase::PresHuman => {
                let mut v = Vec::new();
                for c in ['P', 'M'] {
                    if self.hand.contains(&c) {
                        v.push(Move::Discard(c));
                    }
                }
                v
            }
            Phase::ChanHuman => {
                let mut v = Vec::new();
                for c in ['P', 'M'] {
                    if self.hand.contains(&c) {
                        v.push(Move::Enact(c));
                    }
                }
                v
            }
            Phase::ExecHuman => (1..N)
                .filter(|&j| self.alive[j])
                .map(|j| Move::Execute(j as u8))
                .collect(),
            Phase::Over => Vec::new(),
            _ => vec![Move::Table],
        }
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Nominate(c) => {
                self.chancellor = Some(*c as usize);
                self.say(format!(
                    "Locul {} nominalizează Cancelar pe locul {}.",
                    self.president + 1,
                    *c as usize + 1
                ));
                self.phase = if self.alive[0] {
                    Phase::Vote
                } else {
                    Phase::VoteTable
                };
            }
            Move::Yes | Move::No => {
                self.human_vote = Some(*m == Move::Yes);
                self.phase = Phase::VoteTable;
            }
            Move::Discard(c) => {
                let idx = self.hand.iter().position(|x| x == c).unwrap();
                let x = self.hand.remove(idx);
                self.discard.push(x);
                if self.chancellor == Some(0) {
                    self.phase = Phase::ChanHuman;
                } else {
                    self.phase = Phase::LegisTable;
                }
            }
            Move::Enact(c) => self.take_enacted(*c),
            Move::Execute(t) => self.do_execute(*t as usize),
            Move::Table => match self.phase {
                Phase::NominateTable => {
                    let c = self.robot_nominate();
                    self.apply(&Move::Nominate(c as u8));
                }
                Phase::VoteTable => {
                    let mut votes = [false; N];
                    for (i, v) in votes.iter_mut().enumerate() {
                        if !self.alive[i] {
                            continue;
                        }
                        *v = if i == 0 {
                            self.human_vote.unwrap_or(true)
                        } else {
                            self.robot_vote(i)
                        };
                    }
                    let yes = votes.iter().filter(|&&v| v).count();
                    let live = self.alive_n();
                    let line: Vec<String> = (0..N)
                        .filter(|&i| self.alive[i])
                        .map(|i| format!("{}:{}", i + 1, if votes[i] { "da" } else { "nu" }))
                        .collect();
                    let ok = yes * 2 > live;
                    self.say(format!(
                        "Vot: {} ⇒ {}",
                        line.join(" "),
                        if ok { "ales" } else { "respins" }
                    ));
                    if ok {
                        self.tracker = 0;
                        let c = self.chancellor.unwrap();
                        if self.enacted_m >= 3 && self.roles[c] == Role::Usurer {
                            self.win(
                                false,
                                "Cămătarul a fost ales Cancelar: conspiratorii câștigă.",
                            );
                            return;
                        }
                        self.hand = vec![self.draw(), self.draw(), self.draw()];
                        self.phase = if self.president == 0 {
                            Phase::PresHuman
                        } else {
                            Phase::LegisTable
                        };
                    } else {
                        self.tracker += 1;
                        let d = self.chancellor.map(|c| (c, 1));
                        if let Some((c, k)) = d {
                            self.sus[c] += k / 2;
                        }
                        if self.tracker >= 3 {
                            self.tracker = 0;
                            let c = self.draw();
                            self.say(
                                "Trei respingeri: legea din vârful pachetului se promulgă.".into(),
                            );
                            self.chancellor = None;
                            self.prev_chan = None;
                            self.hand.clear();
                            // numărarea e aceeași, dar fără putere și fără suspiciune
                            if c == 'P' {
                                self.enacted_p += 1;
                            } else {
                                self.enacted_m += 1;
                            }
                            if self.enacted_p >= 5 {
                                self.win(true, "Cinci legi Prosperitate: negustorii câștigă.");
                                return;
                            }
                            if self.enacted_m >= 6 {
                                self.win(false, "Șase legi Monopol: conspiratorii câștigă.");
                                return;
                            }
                            self.say(format!("Legea promulgată: {}.", c));
                        }
                        self.finish_round();
                    }
                }
                Phase::LegisTable => {
                    if self.hand.len() == 3 {
                        self.pres_discard_robot();
                    }
                    if self.chancellor == Some(0) {
                        self.phase = Phase::ChanHuman;
                    } else {
                        let c = self.chan_choice_robot();
                        self.take_enacted(c);
                    }
                }
                Phase::ExecTable => {
                    let t = self.robot_execute();
                    self.do_execute(t);
                }
                _ => {}
            },
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        let m = self.winner_merchants?;
        let human_m = !self.roles[0].evil();
        Some(Outcome::Win(if m == human_m {
            Player::One
        } else {
            Player::Two
        }))
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
            Move::Nominate(c) => format!("cancelar {}", *c + 1),
            Move::Yes => "da".into(),
            Move::No => "nu".into(),
            Move::Discard(c) => format!("arunca {c}"),
            Move::Enact(c) => format!("promulga {c}"),
            Move::Execute(t) => format!("executa {}", *t + 1),
            Move::Table => "masa".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(seed: u64) -> Consiliul {
        let mut g = Consiliul::with_seed(seed);
        let mut rng = Rng::new(seed ^ 5);
        let mut n = 0;
        while g.outcome().is_none() {
            let mv = g.legal_moves();
            assert!(!mv.is_empty(), "{}", g.render());
            g.apply(&mv[rng.below(mv.len())]);
            assert_eq!(g.total_laws(), 17);
            n += 1;
            assert!(n < 800);
        }
        g
    }

    #[test]
    fn deck_composition_and_roles() {
        let g = Consiliul::with_seed(9);
        assert_eq!(g.deck.iter().filter(|&&c| c == 'P').count(), 6);
        assert_eq!(g.deck.iter().filter(|&&c| c == 'M').count(), 11);
        assert_eq!(g.roles().iter().filter(|r| r.evil()).count(), 2);
        assert_eq!(g.roles().iter().filter(|&&r| r == Role::Usurer).count(), 1);
    }

    #[test]
    fn random_games_end_with_both_winners() {
        let (mut a, mut b) = (0, 0);
        for seed in 1..300u64 {
            let g = run(seed);
            if g.winner_merchants == Some(true) {
                a += 1;
            } else {
                b += 1;
            }
        }
        assert!(a > 10 && b > 10, "{a} {b}");
    }

    #[test]
    fn previous_chancellor_is_ineligible() {
        let mut g = Consiliul::with_seed(4);
        g.prev_chan = Some(3);
        assert!(!g.eligible().contains(&3));
        assert!(!g.eligible().contains(&g.president));
    }

    #[test]
    fn evil_human_knows_partner_only() {
        for seed in 1..60u64 {
            let g = Consiliul::with_seed(seed);
            let r = g.render();
            if g.roles()[0].evil() {
                assert!(r.contains("Colegul tău"));
            } else {
                assert!(!r.contains("Colegul"));
                assert!(!r.contains("Cămătar") || r.contains("Rolul tău: Cămătar"));
            }
        }
    }
}
