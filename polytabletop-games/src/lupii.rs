//! LUPII DIN HÂRTOP — joc de roluri ascunse (familia „lup și sat”), variantă originală.
//!
//! Tu ești locul 1 la o masă de 8–9 jucători; **ceilalți locuri sunt roboți** (masa,
//! `Player::Two`), care joacă după euristici și văd doar ce ar vedea un jucător
//! real (propriul rol, ce s-a anunțat public, rolurile celor morți).
//! Noaptea: lupii aleg o victimă; rolurile speciale acționează. Ziua: toți votează
//! cine e scos din sat (la egalitate se trage la sorți între cei egali). Rolul
//! celui mort se arată. Satul câștigă când nu mai e niciun lup; lupii câștigă
//! când sunt cel puțin cât restul.
//!
//! Presetări (se alege la început): `mafie` (stil clasic: doi mafioți, doctor,
//! detectiv), `sat` (stil francez: lupi, văzătoare, vrăjitoare, vânător),
//! `stepa` (stil chinezesc: trei lupi, profet, paznic, vrăjitoare, vânător).
//! Simplificări: fără Cupidon/Primar, fără discuții libere (robotul „acuză” public
//! o dată pe zi), vrăjitoarea poate folosi un singur leac pe noapte.

use crate::w5_util::{pick_weighted, shuffle_vec, trim_log};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Role {
    Wolf,
    Villager,
    Seer,
    Guard,
    Witch,
    Hunter,
}

impl Role {
    fn name(self, preset: u8) -> &'static str {
        match (self, preset) {
            (Role::Wolf, 0) => "Mafiot",
            (Role::Wolf, _) => "Lup",
            (Role::Villager, 0) => "Cetățean",
            (Role::Villager, _) => "Sătean",
            (Role::Seer, 0) => "Detectiv",
            (Role::Seer, 1) => "Văzătoare",
            (Role::Seer, _) => "Profet",
            (Role::Guard, 0) => "Doctor",
            (Role::Guard, _) => "Paznic",
            (Role::Witch, _) => "Vrăjitoare",
            (Role::Hunter, _) => "Vânător",
        }
    }
}

const PRESETS: [&str; 3] = ["mafie", "sat", "stepa"];

fn roles_of(preset: u8) -> Vec<Role> {
    use Role::*;
    match preset {
        0 => vec![
            Wolf, Wolf, Guard, Seer, Villager, Villager, Villager, Villager,
        ],
        1 => vec![
            Wolf, Wolf, Seer, Witch, Hunter, Villager, Villager, Villager,
        ],
        _ => vec![
            Wolf, Wolf, Wolf, Seer, Witch, Guard, Hunter, Villager, Villager,
        ],
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Phase {
    Setup,
    Night,
    NightResolve,
    Vote,
    VoteResolve,
    Shoot(bool), // true: după asta urmează ziua; false: noaptea
    Over,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Preset(u8),
    Kill(u8),
    Inspect(u8),
    Protect(u8),
    Save,
    Poison(u8),
    Sleep,
    Vote(u8),
    Shoot(u8),
    Table,
}

#[derive(Clone)]
pub struct Lupii {
    /// Whose private info `render` shows (`None` = the mover's, hot-seat).
    viewer: Option<Player>,
    preset: u8,
    roles: Vec<Role>,
    alive: Vec<bool>,
    phase: Phase,
    day: u32,
    rng: Rng,
    /// Suspiciune publică (se actualizează din voturi + roluri dezvăluite).
    sus: Vec<i32>,
    last_votes: Vec<Option<usize>>,
    wolf_target: Option<usize>,
    human_inspect: Option<usize>,
    human_protect: Option<usize>,
    human_save: bool,
    human_poison: Option<usize>,
    pending_human_vote: Option<usize>,
    last_guard: Option<usize>,
    heal_left: bool,
    poison_left: bool,
    /// Cine a fost cercetat de văzătorul-robot și e lup (informație privată a robotului).
    seer_found: Option<usize>,
    /// Acuzația publică de azi (a robotului văzător, sau a unui lup care minte).
    accused: Option<usize>,
    seen: Vec<(usize, bool)>, // rezultatele cercetărilor omului
    log: Vec<String>,
    winner_wolves: Option<bool>,
}

impl Default for Lupii {
    fn default() -> Self {
        Self::new()
    }
}

impl Lupii {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        Lupii {
            viewer: None,
            preset: 1,
            roles: Vec::new(),
            alive: Vec::new(),
            phase: Phase::Setup,
            day: 0,
            rng: Rng::new(seed),
            sus: Vec::new(),
            last_votes: Vec::new(),
            wolf_target: None,
            human_inspect: None,
            human_protect: None,
            human_save: false,
            human_poison: None,
            pending_human_vote: None,
            last_guard: None,
            heal_left: true,
            poison_left: true,
            seer_found: None,
            accused: None,
            seen: Vec::new(),
            log: vec!["Alege presetarea: mafie, sat sau stepa.".into()],
            winner_wolves: None,
        }
    }

    /// Pornește direct cu o presetare (pentru teste).
    pub fn with_preset(seed: u64, preset: u8) -> Self {
        let mut g = Self::with_seed(seed);
        g.apply(&Move::Preset(preset));
        g
    }

    pub fn roles(&self) -> &[Role] {
        &self.roles
    }
    pub fn alive_count(&self) -> usize {
        self.alive.iter().filter(|&&a| a).count()
    }
    pub fn phase(&self) -> Phase {
        self.phase
    }
    pub fn human_role(&self) -> Option<Role> {
        self.roles.first().copied()
    }

    fn n(&self) -> usize {
        self.roles.len()
    }
    fn living(&self) -> Vec<usize> {
        (0..self.n()).filter(|&i| self.alive[i]).collect()
    }
    fn wolves_alive(&self) -> usize {
        (0..self.n())
            .filter(|&i| self.alive[i] && self.roles[i] == Role::Wolf)
            .count()
    }
    fn say(&mut self, s: String) {
        self.log.push(s);
        trim_log(&mut self.log, 16);
    }
    fn seat(i: usize) -> usize {
        i + 1
    }
    fn rname(&self, i: usize) -> &'static str {
        self.roles[i].name(self.preset)
    }

    fn check_win(&mut self) -> bool {
        let w = self.wolves_alive();
        let rest = self.alive_count() - w;
        if w == 0 {
            self.winner_wolves = Some(false);
        } else if w >= rest {
            self.winner_wolves = Some(true);
        } else {
            return false;
        }
        self.phase = Phase::Over;
        true
    }

    /// Ucide pe `i` și dezvăluie rolul. Vânătorul-robot trage pe loc.
    fn kill(&mut self, i: usize, why: &str) -> bool {
        if !self.alive[i] {
            return false;
        }
        self.alive[i] = false;
        let msg = format!(
            "Locul {} ({}) {} — era {}.",
            Self::seat(i),
            if i == 0 { "tu" } else { "robot" },
            why,
            self.rname(i)
        );
        self.say(msg);
        // un lup dezvăluit curăță suspiciunea celor care l-au apărat; simplu: reset
        self.sus[i] = -100;
        if self.roles[i] == Role::Hunter && i != 0 {
            let t = self.top_suspect(i);
            if let Some(t) = t {
                self.say(format!(
                    "Vânătorul de la locul {} trage înainte să cadă în locul {}.",
                    Self::seat(i),
                    Self::seat(t)
                ));
                self.kill(t, "e împușcat");
            }
        }
        i == 0 && self.roles[0] == Role::Hunter
    }

    fn top_suspect(&mut self, me: usize) -> Option<usize> {
        let cand: Vec<usize> = self.living().into_iter().filter(|&j| j != me).collect();
        if cand.is_empty() {
            return None;
        }
        let w: Vec<u32> = cand
            .iter()
            .map(|&j| (self.sus[j] + 12).max(1) as u32)
            .collect();
        let sq: Vec<u32> = w.iter().map(|x| x * x).collect();
        Some(cand[pick_weighted(&mut self.rng, &sq)])
    }

    fn begin_night(&mut self) {
        self.day += 1;
        self.phase = Phase::Night;
        self.human_inspect = None;
        self.human_protect = None;
        self.human_save = false;
        self.human_poison = None;
        self.wolf_target = None;
        self.accused = None;
        let human_wolf = self.alive[0] && self.roles[0] == Role::Wolf;
        if !human_wolf {
            self.wolf_target = self.robot_wolf_pick();
        }
        if !self.alive[0] {
            self.phase = Phase::NightResolve;
        }
    }

    fn robot_wolf_pick(&mut self) -> Option<usize> {
        let cand: Vec<usize> = self
            .living()
            .into_iter()
            .filter(|&j| self.roles[j] != Role::Wolf)
            .collect();
        if cand.is_empty() {
            return None;
        }
        // lupii preferă pe cine pare periculos (suspiciune mică = „de încredere”, deci influent)
        let w: Vec<u32> = cand
            .iter()
            .map(|&j| (12 - self.sus[j]).clamp(1, 30) as u32)
            .collect();
        Some(cand[pick_weighted(&mut self.rng, &w)])
    }

    fn resolve_night(&mut self) {
        let mut victim = self.wolf_target;
        // paznic / doctor
        let guard_idx = (0..self.n()).find(|&i| self.alive[i] && self.roles[i] == Role::Guard);
        let mut protected = None;
        if let Some(g) = guard_idx {
            if g == 0 {
                protected = self.human_protect;
            } else {
                let cand: Vec<usize> = self
                    .living()
                    .into_iter()
                    .filter(|&j| Some(j) != self.last_guard)
                    .collect();
                if !cand.is_empty() {
                    protected = Some(cand[self.rng.below(cand.len())]);
                }
            }
            self.last_guard = protected;
        }
        // vrăjitoare
        let witch_idx = (0..self.n()).find(|&i| self.alive[i] && self.roles[i] == Role::Witch);
        let mut poisoned = None;
        let mut healed = false;
        if let Some(w) = witch_idx {
            if w == 0 {
                if self.human_save && self.heal_left {
                    healed = true;
                    self.heal_left = false;
                }
                if let Some(p) = self.human_poison {
                    if self.poison_left {
                        poisoned = Some(p);
                        self.poison_left = false;
                    }
                }
            } else {
                if self.heal_left && victim.is_some() && self.rng.below(10) < 7 {
                    healed = true;
                    self.heal_left = false;
                } else if self.poison_left && self.day >= 2 && self.rng.below(3) == 0 {
                    if let Some(t) = self.top_suspect(w) {
                        if self.sus[t] >= 2 {
                            poisoned = Some(t);
                            self.poison_left = false;
                        }
                    }
                }
            }
        }
        // cercetări
        let seer_idx = (0..self.n()).find(|&i| self.alive[i] && self.roles[i] == Role::Seer);
        if let Some(s) = seer_idx {
            if s == 0 {
                if let Some(t) = self.human_inspect {
                    let is_w = self.roles[t] == Role::Wolf;
                    self.seen.push((t, is_w));
                    self.say(format!(
                        "Cercetare: locul {} {} lup.",
                        Self::seat(t),
                        if is_w { "ESTE" } else { "NU este" }
                    ));
                }
            } else {
                let cand: Vec<usize> = self
                    .living()
                    .into_iter()
                    .filter(|&j| j != s && Some(j) != self.seer_found)
                    .collect();
                if !cand.is_empty() {
                    let t = cand[self.rng.below(cand.len())];
                    if self.roles[t] == Role::Wolf {
                        self.seer_found = Some(t);
                    }
                }
            }
        }
        // victima
        if let Some(v) = victim {
            if healed || protected == Some(v) {
                victim = None;
            }
        }
        self.say(format!("— Noaptea {} —", self.day));
        let mut hunter_human = false;
        let mut dead_any = false;
        if let Some(v) = victim {
            hunter_human |= self.kill(v, "a fost găsit mort dimineața");
            dead_any = true;
        }
        if let Some(p) = poisoned {
            if self.alive[p] {
                hunter_human |= self.kill(p, "a fost otrăvit");
                dead_any = true;
            }
        }
        if !dead_any {
            self.say("Nimeni n-a murit în noaptea asta.".into());
        }
        if self.check_win() {
            return;
        }
        // anunț public: văzătorul-robot acuză un lup găsit; un lup minte uneori
        self.accused = None;
        if let Some(f) = self.seer_found {
            if self.alive[f] {
                self.accused = Some(f);
            } else {
                self.seer_found = None;
            }
        }
        if self.accused.is_none() {
            let liar =
                (0..self.n()).find(|&i| i != 0 && self.alive[i] && self.roles[i] == Role::Wolf);
            if let Some(l) = liar {
                if self.rng.below(2) == 0 {
                    let cand: Vec<usize> = self
                        .living()
                        .into_iter()
                        .filter(|&j| self.roles[j] != Role::Wolf && j != 0)
                        .collect();
                    if !cand.is_empty() {
                        self.accused = Some(cand[self.rng.below(cand.len())]);
                        let _ = l;
                    }
                }
            }
        }
        if let Some(a) = self.accused {
            self.say(format!(
                "Un jucător acuză public locul {} că ar fi lup.",
                Self::seat(a)
            ));
        }
        self.phase = if hunter_human {
            Phase::Shoot(true)
        } else {
            Phase::Vote
        };
        if !self.alive[0] && self.phase == Phase::Vote {
            self.phase = Phase::VoteResolve;
        }
    }

    fn robot_vote(&mut self, r: usize, target_hint: Option<usize>) -> usize {
        let living = self.living();
        let others: Vec<usize> = living.iter().copied().filter(|&j| j != r).collect();
        if self.roles[r] == Role::Wolf {
            // lupii votează la unison un sătean, după suspiciunea publică
            if let Some(t) = target_hint {
                return t;
            }
            let cand: Vec<usize> = others
                .iter()
                .copied()
                .filter(|&j| self.roles[j] != Role::Wolf)
                .collect();
            let w: Vec<u32> = cand
                .iter()
                .map(|&j| (self.sus[j] + 12).max(1) as u32)
                .collect();
            return cand[pick_weighted(&mut self.rng, &w)];
        }
        if self.roles[r] == Role::Seer {
            if let Some(f) = self.seer_found {
                if self.alive[f] {
                    return f;
                }
            }
        }
        if let Some(a) = self.accused {
            if a != r && self.alive[a] && self.rng.below(10) < 7 {
                return a;
            }
        }
        let w: Vec<u32> = others
            .iter()
            .map(|&j| {
                let s = (self.sus[j] + 12).max(1) as u32;
                s * s
            })
            .collect();
        others[pick_weighted(&mut self.rng, &w)]
    }

    fn resolve_vote(&mut self, human_vote: Option<usize>) {
        let living = self.living();
        let mut votes: Vec<Option<usize>> = vec![None; self.n()];
        if let Some(h) = human_vote {
            votes[0] = Some(h);
        }
        // lupii-robot aleg o țintă comună
        let wolf_pick = {
            let ws: Vec<usize> = living
                .iter()
                .copied()
                .filter(|&j| j != 0 && self.roles[j] == Role::Wolf)
                .collect();
            if let Some(&w0) = ws.first() {
                Some(self.robot_vote(w0, None))
            } else {
                None
            }
        };
        for &r in &living {
            if r == 0 {
                continue;
            }
            let hint = if self.roles[r] == Role::Wolf {
                wolf_pick
            } else {
                None
            };
            let hint = hint.filter(|&t| t != r);
            votes[r] = Some(self.robot_vote(r, hint));
        }
        let mut tally = vec![0u32; self.n()];
        let mut lines = Vec::new();
        for &r in &living {
            if let Some(t) = votes[r] {
                tally[t] += 1;
                lines.push(format!("{}→{}", Self::seat(r), Self::seat(t)));
            }
        }
        let mx = *tally.iter().max().unwrap_or(&0);
        let tied: Vec<usize> = (0..self.n())
            .filter(|&j| tally[j] == mx && mx > 0)
            .collect();
        self.say(format!("Voturi: {}", lines.join(" ")));
        self.last_votes = votes.clone();
        if tied.is_empty() {
            self.begin_night();
            return;
        }
        let out = tied[self.rng.below(tied.len())];
        let was_wolf = self.roles[out] == Role::Wolf;
        // actualizează suspiciunea: cine a votat un nevinovat devine mai suspect
        for &r in &living {
            if let Some(t) = votes[r] {
                if t == out {
                    if was_wolf {
                        self.sus[r] -= 3;
                    } else {
                        self.sus[r] += 3;
                    }
                }
            }
        }
        let hh = self.kill(out, "e scos din sat prin vot");
        if self.check_win() {
            return;
        }
        if hh {
            self.phase = Phase::Shoot(false);
            return;
        }
        self.begin_night();
    }
}

impl Lupii {
    /// Normal rendering (human = J1's point of view).
    fn render_main(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            "LUPII DIN HÂRTOP — ziua {} — fază: {}\n",
            self.day,
            match self.phase {
                Phase::Setup => "alegerea presetării",
                Phase::Night => "noapte (tu acționezi)",
                Phase::NightResolve => "noapte (masa joacă)",
                Phase::Vote => "vot (tu votezi)",
                Phase::VoteResolve => "vot (masa votează)",
                Phase::Shoot(_) => "ultima ta împușcătură",
                Phase::Over => "terminat",
            }
        ));
        if self.roles.is_empty() {
            s.push_str("Presetări: preset mafie | preset sat | preset stepa\n");
        } else {
            s.push_str(&format!("Rolul tău: {}\n", self.rname(0)));
            for i in 0..self.n() {
                let st = if self.alive[i] {
                    "viu".to_string()
                } else {
                    format!("mort ({})", self.rname(i))
                };
                let me = if i == 0 { " (tu)" } else { "" };
                s.push_str(&format!("  locul {}{}: {}\n", Self::seat(i), me, st));
            }
            if self.phase == Phase::Night && self.roles[0] == Role::Witch {
                if let Some(t) = self.wolf_target {
                    s.push_str(&format!(
                        "Lupii au ales locul {} (leac: {}, otravă: {}).\n",
                        Self::seat(t),
                        if self.heal_left { "da" } else { "nu" },
                        if self.poison_left { "da" } else { "nu" }
                    ));
                }
            }
            if !self.seen.is_empty() {
                let v: Vec<String> = self
                    .seen
                    .iter()
                    .map(|&(t, w)| format!("{}={}", Self::seat(t), if w { "lup" } else { "curat" }))
                    .collect();
                s.push_str(&format!("Cercetările tale: {}\n", v.join(", ")));
            }
        }
        for l in &self.log {
            s.push_str(l);
            s.push('\n');
        }
        if let Some(w) = self.winner_wolves {
            s.push_str(if w {
                "Lupii au câștigat.\n"
            } else {
                "Satul a câștigat.\n"
            });
            if !self.roles.is_empty() {
                let all: Vec<String> = (0..self.n())
                    .map(|i| format!("{}={}", Self::seat(i), self.rname(i)))
                    .collect();
                s.push_str(&format!("Roluri: {}\n", all.join(", ")));
            }
        }
        s
    }

    /// Reveal-all rendering, appended when `set_viewer(Some(Two))` (second human inspects).
    fn god_view(&self) -> String {
        let mut s =
            String::from("--- VEDERE TOTALĂ (J2): toate rolurile și informațiile ascunse ---\n");
        if !self.roles.is_empty() {
            let all: Vec<String> = (0..self.n())
                .map(|i| {
                    format!(
                        "{}={}{}",
                        Self::seat(i),
                        self.rname(i),
                        if self.alive[i] { "" } else { "†" }
                    )
                })
                .collect();
            s.push_str(&format!("Roluri: {}\n", all.join(", ")));
            if let Some(t) = self.wolf_target {
                s.push_str(&format!(
                    "Victima aleasă de lupi: locul {}\n",
                    Self::seat(t)
                ));
            }
        }
        s
    }
}

impl Game for Lupii {
    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }
    type Move = Move;

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Setup => Player::One,
            Phase::Night | Phase::Vote | Phase::Shoot(_) => Player::One,
            Phase::NightResolve | Phase::VoteResolve => Player::Two,
            Phase::Over => Player::One,
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        let living = self.living();
        match self.phase {
            Phase::Setup => (0..3).map(Move::Preset).collect(),
            Phase::Night => {
                let me = 0usize;
                match self.roles[me] {
                    Role::Wolf => living
                        .iter()
                        .filter(|&&j| self.roles[j] != Role::Wolf)
                        .map(|&j| Move::Kill(j as u8))
                        .collect(),
                    Role::Seer => living
                        .iter()
                        .filter(|&&j| j != me)
                        .map(|&j| Move::Inspect(j as u8))
                        .collect(),
                    Role::Guard => living
                        .iter()
                        .filter(|&&j| Some(j) != self.last_guard)
                        .map(|&j| Move::Protect(j as u8))
                        .collect(),
                    Role::Witch => {
                        let mut v = vec![Move::Sleep];
                        if self.heal_left && self.wolf_target.is_some() {
                            v.push(Move::Save);
                        }
                        if self.poison_left {
                            for &j in &living {
                                if j != me {
                                    v.push(Move::Poison(j as u8));
                                }
                            }
                        }
                        v
                    }
                    _ => vec![Move::Sleep],
                }
            }
            Phase::Vote => living
                .iter()
                .filter(|&&j| j != 0)
                .map(|&j| Move::Vote(j as u8))
                .collect(),
            Phase::Shoot(_) => living.iter().map(|&j| Move::Shoot(j as u8)).collect(),
            Phase::NightResolve | Phase::VoteResolve => vec![Move::Table],
            Phase::Over => Vec::new(),
        }
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Preset(p) => {
                self.preset = *p;
                let mut roles = roles_of(*p);
                shuffle_vec(&mut roles, &mut self.rng);
                let n = roles.len();
                self.roles = roles;
                self.alive = vec![true; n];
                self.sus = vec![0; n];
                self.last_votes = vec![None; n];
                self.log.clear();
                let r = self.rname(0);
                self.say(format!(
                    "Presetarea «{}», {} jucători. Rolul tău: {}.",
                    PRESETS[*p as usize], n, r
                ));
                if self.roles[0] == Role::Wolf {
                    let mates: Vec<String> = (1..n)
                        .filter(|&j| self.roles[j] == Role::Wolf)
                        .map(|j| Self::seat(j).to_string())
                        .collect();
                    self.say(format!(
                        "Complicii tăi sunt locurile: {}.",
                        mates.join(", ")
                    ));
                }
                self.begin_night();
            }
            Move::Kill(t) => {
                self.wolf_target = Some(*t as usize);
                self.phase = Phase::NightResolve;
            }
            Move::Inspect(t) => {
                self.human_inspect = Some(*t as usize);
                self.phase = Phase::NightResolve;
            }
            Move::Protect(t) => {
                self.human_protect = Some(*t as usize);
                self.phase = Phase::NightResolve;
            }
            Move::Save => {
                self.human_save = true;
                self.phase = Phase::NightResolve;
            }
            Move::Poison(t) => {
                self.human_poison = Some(*t as usize);
                self.phase = Phase::NightResolve;
            }
            Move::Sleep => self.phase = Phase::NightResolve,
            Move::Vote(t) => {
                self.phase = Phase::VoteResolve;
                self.pending_human_vote = Some(*t as usize);
            }
            Move::Shoot(t) => {
                let after_day = matches!(self.phase, Phase::Shoot(true));
                self.kill(*t as usize, "e împușcat de tine");
                if self.check_win() {
                    return;
                }
                if after_day {
                    self.phase = if self.alive[0] {
                        Phase::Vote
                    } else {
                        Phase::VoteResolve
                    };
                } else {
                    self.begin_night();
                }
            }
            Move::Table => match self.phase {
                Phase::NightResolve => self.resolve_night(),
                Phase::VoteResolve => {
                    let hv = self.pending_human_vote.take();
                    self.resolve_vote(hv);
                }
                _ => {}
            },
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        let w = self.winner_wolves?;
        let human_wolf = self.roles[0] == Role::Wolf;
        Some(Outcome::Win(if w == human_wolf {
            Player::One
        } else {
            Player::Two
        }))
    }

    fn evaluate(&self, p: Player) -> i32 {
        // singura mutare a masei e `polytabletop`; evaluarea nu are ce informație ascunsă să folosească
        let _ = p;
        0
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
            Move::Preset(p) => format!("preset {}", PRESETS[*p as usize]),
            Move::Kill(t) => format!("ucide {}", Self::seat(*t as usize)),
            Move::Inspect(t) => format!("cerceteaza {}", Self::seat(*t as usize)),
            Move::Protect(t) => format!("pazeste {}", Self::seat(*t as usize)),
            Move::Save => "salveaza".into(),
            Move::Poison(t) => format!("otraveste {}", Self::seat(*t as usize)),
            Move::Sleep => "dormi".into(),
            Move::Vote(t) => format!("vot {}", Self::seat(*t as usize)),
            Move::Shoot(t) => format!("impusca {}", Self::seat(*t as usize)),
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

    fn play_random(mut g: Lupii, seed: u64) -> Lupii {
        let mut rng = Rng::new(seed);
        let mut n = 0;
        while g.outcome().is_none() {
            let mv = g.legal_moves();
            assert!(!mv.is_empty());
            let m = mv[rng.below(mv.len())].clone();
            g.apply(&m);
            n += 1;
            assert!(n < 500);
        }
        g
    }

    #[test]
    fn role_counts_match_presets() {
        for p in 0..3u8 {
            let g = Lupii::with_preset(5, p);
            let mut a = roles_of(p);
            let mut b = g.roles().to_vec();
            let key = |r: &Role| *r as u8;
            a.sort_by_key(key);
            b.sort_by_key(key);
            assert_eq!(a, b);
        }
    }

    #[test]
    fn random_games_end_for_all_presets() {
        for p in 0..3u8 {
            for seed in 1..60u64 {
                let g = play_random(Lupii::with_preset(seed, p), seed + 100);
                assert!(g.outcome().is_some());
                assert!(g.render().contains("Roluri:"));
            }
        }
    }

    #[test]
    fn living_roles_are_hidden() {
        for seed in 1..30u64 {
            let mut g = Lupii::with_preset(seed, 1);
            let mut rng = Rng::new(seed);
            while g.outcome().is_none() {
                let r = g.render();
                for l in r.lines().filter(|l| l.trim_start().starts_with("locul")) {
                    if l.contains(": viu") {
                        assert!(!l.replace("(tu)", "").contains('('), "{l}");
                    }
                }
                let mv = g.legal_moves();
                let m = mv[rng.below(mv.len())].clone();
                g.apply(&m);
            }
        }
    }

    #[test]
    fn win_conditions() {
        let mut g = Lupii::with_preset(3, 0);
        g.alive = (0..g.n()).map(|i| g.roles[i] != Role::Wolf).collect();
        g.alive[0] = true;
        assert!(g.check_win() || g.roles[0] == Role::Wolf);
    }

    #[test]
    fn wolf_human_sees_mates_only() {
        // oricând omul e lup, jurnalul inițial listează complicii
        for seed in 1..200u64 {
            let g = Lupii::with_preset(seed, 2);
            if g.human_role() == Some(Role::Wolf) {
                assert!(g.render().contains("Complicii"));
                return;
            }
        }
        panic!("no wolf seat found");
    }
}
