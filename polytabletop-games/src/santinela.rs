//! FARUL — joc de echipe și deducție socială (variantă originală de „misiuni cu roluri ascunse”).
//!
//! Cinci locuri la masă: tu (locul 1) și patru roboți (masa, `Player::Two`). Trei
//! sunt **Paznici** ai farului (dintre ei, **Ochiul** știe cine sunt sabotorii), doi
//! sunt **Sabotori** (unul dintre ei e **Vulpea**, care la final poate ghici Ochiul).
//! Cinci misiuni (echipe de 2, 3, 2, 3, 3). Conducătorul propune o echipă, toți votează
//! (da/nu); la 5 respingeri la rând câștigă sabotorii. Pe o misiune aprobată fiecare
//! membru alege în secret „succes” sau „sabotaj” (doar sabotorii pot sabota; o singură
//! sabotare ratează misiunea). Cine ajunge primul la 3 misiuni reușite (paznici) sau 3
//! ratate (sabotori) câștigă; dacă paznicii au 3 reușite, Vulpea ghicește Ochiul: dacă
//! nimerește, sabotorii câștigă totuși.
//! Roboții folosesc doar informația publică (cine a propus/votat, ce misiuni au eșuat)
//! plus propriul rol (sabotorii se cunosc, Ochiul îi cunoaște pe sabotori).

use crate::w5_util::{pick_weighted, shuffle_vec, trim_log};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: usize = 5;
const SIZES: [usize; 5] = [2, 3, 2, 3, 3];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Role {
    Keeper,
    Eye,
    Saboteur,
    Fox,
}

impl Role {
    fn evil(self) -> bool {
        matches!(self, Role::Saboteur | Role::Fox)
    }
    fn name(self) -> &'static str {
        match self {
            Role::Keeper => "Paznic",
            Role::Eye => "Ochiul (paznic)",
            Role::Saboteur => "Sabotor",
            Role::Fox => "Vulpea (sabotor)",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Phase {
    Propose,
    ProposeRobot,
    Vote,
    VoteResolve,
    Mission,
    MissionResolve,
    Guess,
    GuessRobot,
    Over,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Team(u8), // mască de locuri
    Yes,
    No,
    Succeed,
    Sabotage,
    Pick(u8),
    Table,
}

#[derive(Clone)]
pub struct Farul {
    /// Whose private info `render` shows (`None` = the mover's, hot-seat).
    viewer: Option<Player>,
    roles: Vec<Role>,
    phase: Phase,
    leader: usize,
    mission: usize,
    rejects: u32,
    team: u8,
    human_vote: Option<bool>,
    human_act: Option<bool>, // true = sabotaj
    votes: Vec<bool>,
    good_wins: u32,
    evil_wins: u32,
    /// Suspiciune publică după misiuni eșuate.
    sus: Vec<i32>,
    /// Puncte „arată a Ochi” (pentru ghicitul Vulpii).
    eyeish: Vec<i32>,
    rng: Rng,
    log: Vec<String>,
    winner_good: Option<bool>,
}

impl Default for Farul {
    fn default() -> Self {
        Self::new()
    }
}

fn bits(m: u8) -> Vec<usize> {
    (0..N).filter(|&i| m & (1 << i) != 0).collect()
}

impl Farul {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut roles = vec![
            Role::Keeper,
            Role::Keeper,
            Role::Eye,
            Role::Saboteur,
            Role::Fox,
        ];
        shuffle_vec(&mut roles, &mut rng);
        let leader = rng.below(N);
        let mut g = Farul {
            viewer: None,
            roles,
            phase: Phase::Propose,
            leader,
            mission: 0,
            rejects: 0,
            team: 0,
            human_vote: None,
            human_act: None,
            votes: Vec::new(),
            good_wins: 0,
            evil_wins: 0,
            sus: vec![0; N],
            eyeish: vec![0; N],
            rng,
            log: Vec::new(),
            winner_good: None,
        };
        let r = g.roles[0];
        g.say(format!("Rolul tău: {}.", r.name()));
        match r {
            Role::Eye => {
                let e: Vec<String> = (0..N)
                    .filter(|&i| g.roles[i].evil())
                    .map(|i| (i + 1).to_string())
                    .collect();
                g.say(format!("Ochiul vede sabotorii: locurile {}.", e.join(", ")));
            }
            Role::Saboteur | Role::Fox => {
                let e: Vec<String> = (0..N)
                    .filter(|&i| i != 0 && g.roles[i].evil())
                    .map(|i| (i + 1).to_string())
                    .collect();
                g.say(format!("Celălalt sabotor: locul {}.", e.join(", ")));
            }
            Role::Keeper => {}
        }
        g.start_round();
        g
    }

    pub fn roles(&self) -> &[Role] {
        &self.roles
    }
    pub fn phase(&self) -> Phase {
        self.phase
    }
    pub fn score(&self) -> (u32, u32) {
        (self.good_wins, self.evil_wins)
    }

    fn say(&mut self, s: String) {
        self.log.push(s);
        trim_log(&mut self.log, 14);
    }

    fn start_round(&mut self) {
        self.phase = if self.leader == 0 {
            Phase::Propose
        } else {
            Phase::ProposeRobot
        };
        self.human_vote = None;
        self.human_act = None;
    }

    fn knows_evil(&self, i: usize, j: usize) -> bool {
        // robotul i știe că j e sabotor?
        match self.roles[i] {
            Role::Eye => self.roles[j].evil(),
            Role::Saboteur | Role::Fox => self.roles[j].evil(),
            Role::Keeper => false,
        }
    }

    fn robot_propose(&mut self) -> u8 {
        let l = self.leader;
        let k = SIZES[self.mission];
        let mut order: Vec<usize> = (0..N).collect();
        shuffle_vec(&mut order, &mut self.rng);
        // scor mai mic = mai bun pentru echipă
        let mut score = [0i32; N];
        for (j, s) in score.iter_mut().enumerate() {
            *s = self.sus[j] * 3;
            if j == l {
                *s -= 5; // se pune pe sine
            }
            match self.roles[l] {
                Role::Eye => {
                    if self.roles[j].evil() {
                        *s += 20;
                    }
                }
                Role::Saboteur | Role::Fox => {
                    if self.roles[j].evil() && j != l {
                        *s += 2;
                    }
                }
                Role::Keeper => {}
            }
        }
        order.sort_by_key(|&j| score[j]);
        let mut team: Vec<usize> = order[..k].to_vec();
        // un sabotor se strecoară 1 în echipă dacă nu e deja
        if self.roles[l].evil() && !team.iter().any(|&j| self.roles[j].evil()) {
            team[k - 1] = l;
        }
        let mut m = 0u8;
        for j in team {
            m |= 1 << j;
        }
        m
    }

    fn robot_votes(&mut self, team: u8) -> Vec<bool> {
        let t = bits(team);
        let mut v = vec![true; N];
        for (i, vi) in v.iter_mut().enumerate() {
            if i == 0 {
                continue;
            }
            *vi = match self.roles[i] {
                Role::Eye => !t.iter().any(|&j| self.roles[j].evil()),
                Role::Keeper => {
                    let bad: i32 = t.iter().map(|&j| self.sus[j]).sum();
                    bad <= 1 || (t.contains(&i) && bad <= 3)
                }
                Role::Saboteur | Role::Fox => {
                    let has = t.iter().any(|&j| self.roles[j].evil());
                    has || self.rng.below(4) == 0
                }
            };
            if self.rejects >= 4 && !self.roles[i].evil() {
                *vi = true;
            }
            // un pic de zgomot, să nu fie perfect predictibili
            if self.rng.below(12) == 0 {
                *vi = !*vi;
            }
        }
        v
    }

    fn finish(&mut self, good: bool) {
        self.winner_good = Some(good);
        self.phase = Phase::Over;
    }

    fn after_mission(&mut self) {
        if self.evil_wins >= 3 {
            self.say("Trei misiuni ratate: sabotorii câștigă.".into());
            self.finish(false);
            return;
        }
        if self.good_wins >= 3 {
            self.say("Trei misiuni reușite. Vulpea încearcă să ghicească Ochiul.".into());
            self.phase = if self.roles[0] == Role::Fox {
                Phase::Guess
            } else {
                Phase::GuessRobot
            };
            return;
        }
        self.mission += 1;
        self.leader = (self.leader + 1) % N;
        self.rejects = 0;
        self.start_round();
    }
}

impl Farul {
    /// Normal rendering (human = J1's point of view).
    fn render_main(&self) -> String {
        let mut s = format!(
            "FARUL — misiunea {} (echipă de {}) — conducător: locul {} — respingeri: {}/5 — paznici {} : {} sabotori\n",
            (self.mission + 1).min(5),
            SIZES[self.mission.min(4)],
            self.leader + 1,
            self.rejects,
            self.good_wins,
            self.evil_wins
        );
        s.push_str(&format!("Rolul tău: {} (locul 1)\n", self.roles[0].name()));
        if self.phase == Phase::Vote {
            let t: Vec<String> = bits(self.team)
                .iter()
                .map(|i| (i + 1).to_string())
                .collect();
            s.push_str(&format!(
                "Echipa propusă: {} — votezi da sau nu.\n",
                t.join("+")
            ));
        }
        if self.phase == Phase::Mission {
            s.push_str("Ești în echipă: alegi succes sau sabotaj.\n");
        }
        for l in &self.log {
            s.push_str(l);
            s.push('\n');
        }
        if let Some(g) = self.winner_good {
            s.push_str(if g {
                "Paznicii au câștigat.\n"
            } else {
                "Sabotorii au câștigat.\n"
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
        let mut s = String::from("--- VEDERE TOTALĂ (J2): toate rolurile ---\n");
        let all: Vec<String> = (0..N)
            .map(|i| format!("{}={}", i + 1, self.roles[i].name()))
            .collect();
        s.push_str(&format!("Roluri: {}\n", all.join(", ")));
        s
    }
}

impl Game for Farul {
    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }
    type Move = Move;

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Propose | Phase::Vote | Phase::Mission | Phase::Guess => Player::One,
            Phase::Over => Player::One,
            _ => Player::Two,
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        match self.phase {
            Phase::Propose => {
                let k = SIZES[self.mission];
                (0u8..(1 << N))
                    .filter(|m| m.count_ones() as usize == k)
                    .map(Move::Team)
                    .collect()
            }
            Phase::Vote => vec![Move::Yes, Move::No],
            Phase::Mission => {
                if self.roles[0].evil() {
                    vec![Move::Succeed, Move::Sabotage]
                } else {
                    vec![Move::Succeed]
                }
            }
            Phase::Guess => (1..N).map(|i| Move::Pick(i as u8)).collect(),
            Phase::Over => Vec::new(),
            _ => vec![Move::Table],
        }
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Team(t) => {
                self.team = *t;
                let s: Vec<String> = bits(*t).iter().map(|i| (i + 1).to_string()).collect();
                self.say(format!(
                    "Locul {} propune echipa {}.",
                    self.leader + 1,
                    s.join("+")
                ));
                self.phase = Phase::Vote;
            }
            Move::Yes | Move::No => {
                self.human_vote = Some(*m == Move::Yes);
                self.phase = Phase::VoteResolve;
            }
            Move::Succeed | Move::Sabotage => {
                self.human_act = Some(*m == Move::Sabotage);
                self.phase = Phase::MissionResolve;
            }
            Move::Pick(i) => {
                let hit = self.roles[*i as usize] == Role::Eye;
                self.say(format!(
                    "Ghicești Ochiul: locul {} — {}.",
                    *i + 1,
                    if hit { "corect" } else { "greșit" }
                ));
                self.finish(!hit);
            }
            Move::Table => match self.phase {
                Phase::ProposeRobot => {
                    let t = self.robot_propose();
                    self.apply(&Move::Team(t));
                }
                Phase::VoteResolve => {
                    let mut v = self.robot_votes(self.team);
                    v[0] = self.human_vote.unwrap_or(true);
                    let yes = v.iter().filter(|&&x| x).count();
                    let line: Vec<String> = v
                        .iter()
                        .enumerate()
                        .map(|(i, &x)| format!("{}:{}", i + 1, if x { "da" } else { "nu" }))
                        .collect();
                    self.say(format!(
                        "Vot: {} ⇒ {}",
                        line.join(" "),
                        if yes * 2 > N {
                            "aprobată"
                        } else {
                            "respinsă"
                        }
                    ));
                    // ce a votat fiecare devine informație publică: Ochiul tinde să respingă echipe cu sabotori
                    for (i, &x) in v.iter().enumerate() {
                        let bad = bits(self.team).iter().any(|&j| self.roles[j].evil());
                        if x != bad {
                            self.eyeish[i] += 1;
                        }
                    }
                    self.votes = v;
                    if yes * 2 > N {
                        self.rejects = 0;
                        self.phase = if self.team & 1 != 0 {
                            Phase::Mission
                        } else {
                            Phase::MissionResolve
                        };
                    } else {
                        self.rejects += 1;
                        if self.rejects >= 5 {
                            self.say("Cinci echipe respinse: sabotorii câștigă.".into());
                            self.finish(false);
                        } else {
                            self.leader = (self.leader + 1) % N;
                            self.start_round();
                        }
                    }
                }
                Phase::MissionResolve => {
                    let mut fails = 0;
                    for j in bits(self.team) {
                        let sab = if j == 0 {
                            self.human_act.unwrap_or(false)
                        } else {
                            match self.roles[j] {
                                Role::Saboteur | Role::Fox => {
                                    // sabotează de obicei; uneori se ascunde în prima misiune
                                    !(self.mission == 0 && self.rng.below(2) == 0)
                                }
                                _ => false,
                            }
                        };
                        if sab {
                            fails += 1;
                        }
                    }
                    let s: Vec<String> = bits(self.team)
                        .iter()
                        .map(|i| (i + 1).to_string())
                        .collect();
                    if fails > 0 {
                        self.evil_wins += 1;
                        for j in bits(self.team) {
                            self.sus[j] += 2;
                        }
                        self.say(format!(
                            "Misiunea {} ({}) ratată: {} sabotaj(e).",
                            self.mission + 1,
                            s.join("+"),
                            fails
                        ));
                    } else {
                        self.good_wins += 1;
                        for j in bits(self.team) {
                            self.sus[j] -= 1;
                        }
                        self.say(format!(
                            "Misiunea {} ({}) reușită.",
                            self.mission + 1,
                            s.join("+")
                        ));
                    }
                    self.after_mission();
                }
                Phase::GuessRobot => {
                    let fox = (0..N).find(|&i| self.roles[i] == Role::Fox).unwrap_or(1);
                    let cand: Vec<usize> = (0..N)
                        .filter(|&j| j != fox && self.roles[fox].evil() && !self.knows_evil(fox, j))
                        .collect();
                    let w: Vec<u32> = cand
                        .iter()
                        .map(|&j| (self.eyeish[j] + 5).max(1) as u32)
                        .collect();
                    let pick = cand[pick_weighted(&mut self.rng, &w)];
                    let hit = self.roles[pick] == Role::Eye;
                    self.say(format!(
                        "Vulpea ghicește Ochiul: locul {} — {}.",
                        pick + 1,
                        if hit { "corect" } else { "greșit" }
                    ));
                    self.finish(!hit);
                }
                _ => {}
            },
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        let g = self.winner_good?;
        let human_good = !self.roles[0].evil();
        Some(Outcome::Win(if g == human_good {
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
            Move::Team(t) => {
                let s: Vec<String> = bits(*t).iter().map(|i| (i + 1).to_string()).collect();
                format!("echipa {}", s.join(" "))
            }
            Move::Yes => "da".into(),
            Move::No => "nu".into(),
            Move::Succeed => "succes".into(),
            Move::Sabotage => "sabotaj".into(),
            Move::Pick(i) => format!("tinta {}", *i + 1),
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

    fn run(seed: u64) -> Farul {
        let mut g = Farul::with_seed(seed);
        let mut rng = Rng::new(seed ^ 77);
        let mut n = 0;
        while g.outcome().is_none() {
            let mv = g.legal_moves();
            assert!(!mv.is_empty());
            g.apply(&mv[rng.below(mv.len())]);
            n += 1;
            assert!(n < 400);
        }
        g
    }

    #[test]
    fn roles_are_conserved() {
        for seed in 1..50u64 {
            let g = Farul::with_seed(seed);
            let c = |r: Role| g.roles().iter().filter(|&&x| x == r).count();
            assert_eq!(c(Role::Keeper), 2);
            assert_eq!(c(Role::Eye), 1);
            assert_eq!(c(Role::Saboteur), 1);
            assert_eq!(c(Role::Fox), 1);
        }
    }

    #[test]
    fn random_games_end_and_both_sides_win() {
        let (mut good, mut evil) = (0, 0);
        for seed in 1..300u64 {
            let g = run(seed);
            match g.winner_good {
                Some(true) => good += 1,
                Some(false) => evil += 1,
                None => panic!("no winner"),
            }
            assert!(g.render().contains("Roluri:"));
        }
        assert!(good > 10 && evil > 10, "{good} {evil}");
    }

    #[test]
    fn team_sizes_follow_the_table() {
        let g = Farul::with_seed(3);
        if g.phase() == Phase::Propose {
            for m in g.legal_moves() {
                if let Move::Team(t) = m {
                    assert_eq!(t.count_ones() as usize, SIZES[0]);
                }
            }
        }
    }

    #[test]
    fn only_saboteurs_may_sabotage() {
        for seed in 1..80u64 {
            let mut g = Farul::with_seed(seed);
            let mut rng = Rng::new(seed);
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                if g.phase() == Phase::Mission {
                    assert_eq!(mv.contains(&Move::Sabotage), g.roles()[0].evil());
                }
                g.apply(&mv[rng.below(mv.len())]);
            }
        }
    }

    #[test]
    fn human_eye_sees_the_saboteurs() {
        for seed in 1..100u64 {
            let g = Farul::with_seed(seed);
            if g.roles()[0] == Role::Eye {
                assert!(g.render().contains("Ochiul vede"));
                return;
            }
        }
        panic!("no eye seat");
    }
}
