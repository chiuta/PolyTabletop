//! CEASORNICUL DIN VALEA CEȚII — deducție socială cu povestitor automat (variantă originală).
//!
//! Doisprezece locuri la masă: tu (locul 1, `Player::One`) și unsprezece roboți (masa,
//! `Player::Two`, care e și povestitorul). Scenariul are 14 roluri: 9 **localnici** (din care
//! șapte sunt în joc, doi nu), 2 **străini** (Bețivul și Sfântul), 2 **slugi** (Otrăvitorul și
//! Moștenitoarea) și **Strigoiul**. Localnicii și străinii formează echipa bună; slugile și
//! Strigoiul, echipa rea. Bețivul crede că e un localnic care nu e în joc și primește
//! mereu informații false; un jucător „otrăvit” primește informații false până seara
//! următoare. Rolurile cu informație (Ghicitoarea, Cronicarul, Veghea, Cercetașul, Groparul)
//! primesc răspunsul de la povestitor în fiecare noapte, în ordine fixă. Ziua se vorbește
//! (roboții își spun rolul și informațiile; ei pot mărturisi adevărul sau minți), se
//! nominalizează și se votează; cine are cel puțin jumătate din voturile celor vii și mai multe
//! decât ceilalți e executat la apus. Cei morți au un singur vot „fantomă” pe joc.
//! Binele câștigă când Strigoiul moare (fără moștenitor); răul când rămân doi vii, când
//! Sfântul e executat sau când trec 20 de zile.
//! Roboții bun folosesc doar informația lor privată și afirmațiile publice; roboții răi își
//! cunosc echipa. Toate numele, rolurile și textele sunt originale.

use crate::w5_util::{pick_weighted, shuffle_vec, trim_log};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: usize = 12;
const DAY_CAP: u32 = 20;
const MAX_NOMS: u32 = 4;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Role {
    Ghicitoarea,
    Cronicarul,
    Veghea,
    Calugarul,
    Soldatul,
    Cercetasul,
    Groparul,
    Fecioara,
    Vanatorul,
    Betivul,
    Sfantul,
    Otravitorul,
    Mostenitoarea,
    Strigoiul,
}

const TOWN: [Role; 9] = [
    Role::Ghicitoarea,
    Role::Cronicarul,
    Role::Veghea,
    Role::Calugarul,
    Role::Soldatul,
    Role::Cercetasul,
    Role::Groparul,
    Role::Fecioara,
    Role::Vanatorul,
];

impl Role {
    pub fn name(self) -> &'static str {
        match self {
            Role::Ghicitoarea => "Ghicitoarea",
            Role::Cronicarul => "Cronicarul",
            Role::Veghea => "Veghea",
            Role::Calugarul => "Călugărul",
            Role::Soldatul => "Soldatul",
            Role::Cercetasul => "Cercetașul",
            Role::Groparul => "Groparul",
            Role::Fecioara => "Fecioara",
            Role::Vanatorul => "Vânătorul",
            Role::Betivul => "Bețivul",
            Role::Sfantul => "Sfântul",
            Role::Otravitorul => "Otrăvitorul",
            Role::Mostenitoarea => "Moștenitoarea",
            Role::Strigoiul => "Strigoiul",
        }
    }
    fn key(self) -> &'static str {
        match self {
            Role::Ghicitoarea => "ghicitoarea",
            Role::Cronicarul => "cronicarul",
            Role::Veghea => "veghea",
            Role::Calugarul => "calugarul",
            Role::Soldatul => "soldatul",
            Role::Cercetasul => "cercetasul",
            Role::Groparul => "groparul",
            Role::Fecioara => "fecioara",
            Role::Vanatorul => "vanatorul",
            Role::Betivul => "betivul",
            Role::Sfantul => "sfantul",
            Role::Otravitorul => "otravitorul",
            Role::Mostenitoarea => "mostenitoarea",
            Role::Strigoiul => "strigoiul",
        }
    }
    pub fn evil(self) -> bool {
        matches!(
            self,
            Role::Otravitorul | Role::Mostenitoarea | Role::Strigoiul
        )
    }
    fn is_town(self) -> bool {
        TOWN.contains(&self)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Info {
    /// Ghicitoarea: perechea aleasă și răspunsul.
    Test(u8, u8, bool),
    /// Veghea: câți vecini vii sunt răi (și cine sunt vecinii).
    Count(u8, u8, u8),
    /// Cronicarul: perechi de vecini răi în cerc.
    Chain(u8),
    /// Cercetașul: unul dintre cei doi e localnicul dat.
    Scout(u8, u8, Role),
    /// Groparul: cel executat (locul, rolul).
    Grave(u8, Role),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Stmt {
    pub night: u8,
    pub info: Info,
}

fn fmt_stmt(s: &Stmt) -> String {
    let n = s.night;
    match s.info {
        Info::Test(a, b, h) => format!(
            "n{n}: a verificat {} și {} => {}",
            a + 1,
            b + 1,
            if h { "DA (Strigoi)" } else { "nu" }
        ),
        Info::Count(c, l, r) => format!("n{n}: vecinii {} și {} => {} răi", l + 1, r + 1, c),
        Info::Chain(c) => format!("n{n}: perechi rele alăturate => {c}"),
        Info::Scout(a, b, r) => format!("n{n}: unul dintre {} și {} e {}", a + 1, b + 1, r.name()),
        Info::Grave(s, r) => format!("n{n}: cel executat (locul {}) era {}", s + 1, r.name()),
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Phase {
    NightChoose,
    NightResolve,
    DayTalk,
    NomStart,
    NomHuman,
    NomBot,
    VoteBots,
    VoteHuman,
    Tally,
    DayEnd,
    Over,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Divine(u8, u8),
    Poison(u8),
    Guard(u8),
    Kill(u8),
    Declare,
    Mint(Role),
    Ask(u8),
    Slay(u8),
    Done,
    Nominate(u8),
    Yes,
    No,
    Table,
}

#[derive(Clone)]
pub struct Ceasornic {
    /// Whose private info `render` shows (`None` = the mover's, hot-seat).
    viewer: Option<Player>,
    roles: [Role; N],
    believed: [Role; N],
    alive: [bool; N],
    ghost: [bool; N],
    poisoned: [bool; N],
    herring: usize,
    bluff: [Role; N],
    protected: Option<usize>,
    shot: [bool; N],
    virgin_spent: bool,
    night: u32,
    day: u32,
    phase: Phase,
    hp: Option<(usize, usize)>,
    received: Vec<Vec<Stmt>>,
    shared: [usize; N],
    pub_night: [u32; N],
    claim: [Option<Role>; N],
    public: Vec<Vec<Stmt>>,
    declared: bool,
    nominated: [bool; N],
    nom_by: [bool; N],
    noms_today: u32,
    nom_i: usize,
    nom_start: usize,
    nominee: usize,
    nominator: usize,
    yes: usize,
    thresh: usize,
    block: Option<usize>,
    block_votes: usize,
    execs: Vec<(u32, u8, Role)>,
    winner: Option<bool>,
    rng: Rng,
    log: Vec<String>,
}

impl Default for Ceasornic {
    fn default() -> Self {
        Self::new()
    }
}

impl Ceasornic {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut town = TOWN.to_vec();
        shuffle_vec(&mut town, &mut rng);
        let unused = [town[7], town[8]];
        let mut roles_v: Vec<Role> = town[..7].to_vec();
        roles_v.extend([
            Role::Betivul,
            Role::Sfantul,
            Role::Otravitorul,
            Role::Mostenitoarea,
            Role::Strigoiul,
        ]);
        shuffle_vec(&mut roles_v, &mut rng);
        let mut roles = [Role::Soldatul; N];
        roles.copy_from_slice(&roles_v);
        let mut believed = roles;
        for b in believed.iter_mut() {
            if *b == Role::Betivul {
                *b = unused[rng.below(2)];
            }
        }
        let good: Vec<usize> = (0..N).filter(|&i| !roles[i].evil()).collect();
        let herring = good[rng.below(good.len())];
        let mut bluff = [Role::Soldatul; N];
        let mut k = 0;
        for i in 0..N {
            if roles[i].evil() {
                bluff[i] = if rng.below(10) < 7 {
                    k += 1;
                    unused[(k - 1) % 2]
                } else {
                    TOWN[rng.below(9)]
                };
            }
        }
        let mut g = Ceasornic {
            viewer: None,
            roles,
            believed,
            alive: [true; N],
            ghost: [false; N],
            poisoned: [false; N],
            herring,
            bluff,
            protected: None,
            shot: [false; N],
            virgin_spent: false,
            night: 0,
            day: 0,
            phase: Phase::NightResolve,
            hp: None,
            received: vec![Vec::new(); N],
            shared: [0; N],
            pub_night: [0; N],
            claim: [None; N],
            public: vec![Vec::new(); N],
            declared: false,
            nominated: [false; N],
            nom_by: [false; N],
            noms_today: 0,
            nom_i: 0,
            nom_start: 0,
            nominee: 0,
            nominator: 0,
            yes: 0,
            thresh: 0,
            block: None,
            block_votes: 0,
            execs: Vec::new(),
            winner: None,
            rng,
            log: Vec::new(),
        };
        g.say(format!("Rolul tău: {}.", g.believed[0].name()));
        if g.roles[0].evil() {
            let mates: Vec<String> = (1..N)
                .filter(|&i| g.roles[i].evil())
                .map(|i| format!("{} ({})", i + 1, g.roles[i].name()))
                .collect();
            g.say(format!("Echipa ta rea: locurile {}.", mates.join(", ")));
            if g.roles[0] == Role::Strigoiul {
                g.say(format!(
                    "Roluri bune care nu sunt în joc (de pretins): {} și {}.",
                    unused[0].name(),
                    unused[1].name()
                ));
            }
        }
        g.start_night();
        g
    }

    // ---------- interogări ----------
    pub fn roles(&self) -> &[Role; N] {
        &self.roles
    }
    pub fn phase(&self) -> Phase {
        self.phase
    }
    pub fn alive_count(&self) -> usize {
        self.alive.iter().filter(|&&a| a).count()
    }
    pub fn good_won(&self) -> Option<bool> {
        self.winner
    }
    fn say(&mut self, s: String) {
        self.log.push(s);
        trim_log(&mut self.log, 22);
    }
    fn mal(&self, i: usize) -> bool {
        self.poisoned[i] || self.roles[i] == Role::Betivul
    }
    fn seat_of(&self, r: Role) -> Option<usize> {
        (0..N).find(|&i| self.alive[i] && self.roles[i] == r)
    }
    fn registers_demon(&self, i: usize) -> bool {
        self.roles[i] == Role::Strigoiul || i == self.herring
    }
    fn neighbours(&self, i: usize) -> (usize, usize) {
        let mut l = (i + N - 1) % N;
        while !self.alive[l] && l != i {
            l = (l + N - 1) % N;
        }
        let mut r = (i + 1) % N;
        while !self.alive[r] && r != i {
            r = (r + 1) % N;
        }
        (l, r)
    }
    fn alive_list(&self) -> Vec<usize> {
        (0..N).filter(|&i| self.alive[i]).collect()
    }

    fn finish(&mut self, good: bool) {
        if self.winner.is_none() {
            self.winner = Some(good);
            self.say(if good {
                "Strigoiul a căzut: binele câștigă.".into()
            } else {
                "Răul a cucerit valea.".into()
            });
        }
        self.phase = Phase::Over;
    }

    // ---------- informații ----------
    fn true_info(
        &mut self,
        i: usize,
        role: Role,
        pick: Option<(usize, usize)>,
        n: u32,
    ) -> Option<Info> {
        match role {
            Role::Ghicitoarea => {
                let (a, b) = pick?;
                let hit = self.registers_demon(a) || self.registers_demon(b);
                Some(Info::Test(a as u8, b as u8, hit))
            }
            Role::Veghea => {
                let (l, r) = self.neighbours(i);
                let mut c = 0;
                if l != i && self.roles[l].evil() {
                    c += 1;
                }
                if r != i && r != l && self.roles[r].evil() {
                    c += 1;
                }
                Some(Info::Count(c, l as u8, r as u8))
            }
            Role::Cronicarul => {
                let c = (0..N)
                    .filter(|&x| self.roles[x].evil() && self.roles[(x + 1) % N].evil())
                    .count();
                Some(Info::Chain(c as u8))
            }
            Role::Cercetasul => {
                let towns: Vec<usize> = (0..N)
                    .filter(|&x| x != i && self.roles[x].is_town())
                    .collect();
                if towns.is_empty() {
                    return None;
                }
                let x = towns[self.rng.below(towns.len())];
                let others: Vec<usize> = (0..N).filter(|&y| y != i && y != x).collect();
                let y = others[self.rng.below(others.len())];
                let (a, b) = if self.rng.below(2) == 0 {
                    (x, y)
                } else {
                    (y, x)
                };
                Some(Info::Scout(a as u8, b as u8, self.roles[x]))
            }
            Role::Groparul => {
                let e = self
                    .execs
                    .iter()
                    .find(|e| e.0 as u64 + 1 == n as u64)
                    .copied()?;
                Some(Info::Grave(e.1, e.2))
            }
            _ => None,
        }
    }

    /// Informație falsă (garantat diferită de adevăr când `truth` e dat).
    fn fake_info(&mut self, i: usize, role: Role, truth: Option<Info>, n: u32) -> Option<Info> {
        match role {
            Role::Ghicitoarea => {
                let alive = self.alive_list();
                let (a, b) = match truth {
                    Some(Info::Test(a, b, _)) => (a, b),
                    _ => {
                        let a = alive[self.rng.below(alive.len())];
                        let mut b = alive[self.rng.below(alive.len())];
                        if alive.len() > 1 {
                            while b == a {
                                b = alive[self.rng.below(alive.len())];
                            }
                        }
                        (a as u8, b as u8)
                    }
                };
                let hit = match truth {
                    Some(Info::Test(_, _, h)) => !h,
                    _ => self.rng.below(10) < 3,
                };
                Some(Info::Test(a, b, hit))
            }
            Role::Veghea => {
                let (l, r) = self.neighbours(i);
                let t = match truth {
                    Some(Info::Count(c, _, _)) => Some(c),
                    _ => None,
                };
                let w = [3u32, 3, 1];
                let mut c = pick_weighted(&mut self.rng, &w) as u8;
                if t == Some(c) {
                    c = (c + 1 + self.rng.below(2) as u8) % 3;
                }
                Some(Info::Count(c, l as u8, r as u8))
            }
            Role::Cronicarul => {
                let t = match truth {
                    Some(Info::Chain(c)) => Some(c),
                    _ => None,
                };
                let mut c = self.rng.below(4) as u8;
                if t == Some(c) {
                    c = (c + 1) % 4;
                }
                Some(Info::Chain(c))
            }
            Role::Cercetasul => {
                let (a, b, tr) = match truth {
                    Some(Info::Scout(a, b, r)) => (a, b, Some(r)),
                    _ => {
                        let a = self.rng.below(N);
                        let mut b = self.rng.below(N);
                        while b == a {
                            b = self.rng.below(N);
                        }
                        (a as u8, b as u8, None)
                    }
                };
                let mut r = TOWN[self.rng.below(9)];
                while Some(r) == tr {
                    r = TOWN[self.rng.below(9)];
                }
                Some(Info::Scout(a, b, r))
            }
            Role::Groparul => {
                let e = self
                    .execs
                    .iter()
                    .find(|e| e.0 as u64 + 1 == n as u64)
                    .copied();
                let (seat, tr) = match (truth, e) {
                    (Some(Info::Grave(s, r)), _) => (s, Some(r)),
                    (_, Some(e)) => (e.1, Some(e.2)),
                    _ => return None,
                };
                let all = [
                    Role::Ghicitoarea,
                    Role::Veghea,
                    Role::Soldatul,
                    Role::Fecioara,
                    Role::Sfantul,
                    Role::Cercetasul,
                    Role::Calugarul,
                ];
                let mut r = all[self.rng.below(all.len())];
                while Some(r) == tr {
                    r = all[self.rng.below(all.len())];
                }
                Some(Info::Grave(seat, r))
            }
            _ => None,
        }
    }

    // ---------- noaptea ----------
    fn start_night(&mut self) {
        self.night += 1;
        self.hp = None;
        self.phase = Phase::NightResolve;
        if !self.alive[0] {
            return;
        }
        let n = self.night;
        let need = match self.believed[0] {
            Role::Ghicitoarea => true,
            Role::Otravitorul => self.roles[0] == Role::Otravitorul,
            Role::Calugarul => n >= 2,
            Role::Strigoiul => n >= 2,
            _ => false,
        };
        if need {
            self.phase = Phase::NightChoose;
        }
    }

    fn weights_good_trust(&self, b: usize) -> Vec<(usize, f32)> {
        let sus = self.suspicion(b);
        self.alive_list()
            .into_iter()
            .filter(|&j| j != b)
            .map(|j| (j, sus[j]))
            .collect()
    }

    fn bot_poison(&mut self, s: usize) -> usize {
        let mut cand: Vec<usize> = self
            .alive_list()
            .into_iter()
            .filter(|&j| !self.roles[j].evil())
            .collect();
        if cand.is_empty() {
            cand = self.alive_list();
        }
        let w: Vec<u32> = cand
            .iter()
            .map(|&j| {
                1 + match self.claim[j] {
                    Some(
                        Role::Ghicitoarea
                        | Role::Veghea
                        | Role::Groparul
                        | Role::Cronicarul
                        | Role::Cercetasul,
                    ) => 4,
                    Some(Role::Calugarul) | Some(Role::Vanatorul) => 3,
                    _ => 0,
                }
            })
            .collect();
        let _ = s;
        cand[pick_weighted(&mut self.rng, &w)]
    }

    fn threat(&self, j: usize) -> f32 {
        let mut t = 0.0;
        for s in &self.public[j] {
            match s.info {
                Info::Test(a, b, true)
                    if self.roles[a as usize].evil() || self.roles[b as usize].evil() =>
                {
                    t += 1.5;
                }
                Info::Count(c, l, r)
                    if c >= 1
                        && (self.roles[l as usize].evil() || self.roles[r as usize].evil()) =>
                {
                    t += 1.5;
                }
                _ => {}
            }
        }
        match self.claim[j] {
            Some(Role::Ghicitoarea) | Some(Role::Veghea) => t += 0.8,
            Some(Role::Groparul) | Some(Role::Cercetasul) | Some(Role::Cronicarul) => t += 0.4,
            Some(Role::Calugarul) | Some(Role::Vanatorul) => t += 0.5,
            _ => {}
        }
        t
    }

    fn bot_kill(&mut self, s: usize) -> usize {
        let mut cand: Vec<usize> = self
            .alive_list()
            .into_iter()
            .filter(|&j| j != s && !self.roles[j].evil())
            .collect();
        if cand.is_empty() {
            cand = self.alive_list().into_iter().filter(|&j| j != s).collect();
        }
        if cand.is_empty() {
            return s;
        }
        let w: Vec<u32> = cand
            .iter()
            .map(|&j| {
                let mut x = 10.0 + 40.0 * self.threat(j);
                if self.claim[j] == Some(Role::Soldatul) {
                    x *= 0.2;
                }
                x as u32 + 1
            })
            .collect();
        cand[pick_weighted(&mut self.rng, &w)]
    }

    fn bot_guard(&mut self, s: usize) -> usize {
        let v = self.weights_good_trust(s);
        if v.is_empty() {
            return s;
        }
        let w: Vec<u32> = v
            .iter()
            .map(|&(j, sus)| {
                let mut x = (30.0 - sus * 8.0).max(3.0);
                if matches!(
                    self.claim[j],
                    Some(Role::Ghicitoarea) | Some(Role::Veghea) | Some(Role::Groparul)
                ) {
                    x *= 2.0;
                }
                x as u32
            })
            .collect();
        v[pick_weighted(&mut self.rng, &w)].0
    }

    fn bot_divine(&mut self, s: usize) -> (usize, usize) {
        let mut v = self.weights_good_trust(s);
        v.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        if v.len() < 2 {
            return (s, v.first().map(|x| x.0).unwrap_or(s));
        }
        if self.night == 1 || self.rng.below(3) == 0 {
            let a = v[self.rng.below(v.len())].0;
            let mut b = v[self.rng.below(v.len())].0;
            while b == a {
                b = v[self.rng.below(v.len())].0;
            }
            (a, b)
        } else {
            let a = v[0].0;
            let b = v[1 + self.rng.below((v.len() - 1).min(3))].0;
            (a, b)
        }
    }

    fn kill(&mut self, i: usize) {
        self.alive[i] = false;
        self.ghost[i] = true;
    }

    fn resolve_night(&mut self) {
        let n = self.night;
        self.protected = None;
        let alive_before = self.alive_count();
        let mut died: Vec<usize> = Vec::new();
        if let Some(s) = self.seat_of(Role::Otravitorul) {
            let t = if s == 0 {
                self.hp.map(|p| p.0)
            } else {
                Some(self.bot_poison(s))
            };
            if let Some(t) = t {
                self.poisoned[t] = true;
            }
        }
        if n >= 2 {
            if let Some(s) = self.seat_of(Role::Calugarul) {
                let t = if s == 0 {
                    self.hp.map(|p| p.0)
                } else {
                    Some(self.bot_guard(s))
                };
                if let (Some(t), false) = (t, self.mal(s)) {
                    self.protected = Some(t);
                }
            }
            if let Some(s) = self.seat_of(Role::Strigoiul) {
                let t = if s == 0 {
                    self.hp.map(|p| p.0)
                } else {
                    Some(self.bot_kill(s))
                };
                if let (Some(t), false) = (t, self.mal(s)) {
                    if t == s {
                        let heirs: Vec<usize> = (0..N)
                            .filter(|&i| {
                                i != s
                                    && self.alive[i]
                                    && matches!(
                                        self.roles[i],
                                        Role::Otravitorul | Role::Mostenitoarea
                                    )
                            })
                            .collect();
                        self.kill(s);
                        died.push(s);
                        if !heirs.is_empty() {
                            let h = heirs[self.rng.below(heirs.len())];
                            self.roles[h] = Role::Strigoiul;
                            self.believed[h] = Role::Strigoiul;
                            if h == 0 {
                                self.say("Ai devenit noul Strigoi.".into());
                            }
                        }
                    } else if self.protected == Some(t)
                        || (self.roles[t] == Role::Soldatul && !self.mal(t))
                    {
                        // atac respins
                    } else {
                        self.kill(t);
                        died.push(t);
                    }
                }
            }
        }
        // informații, în ordine fixă
        let order = [
            Role::Cercetasul,
            Role::Cronicarul,
            Role::Ghicitoarea,
            Role::Veghea,
            Role::Groparul,
        ];
        for role in order {
            if (matches!(role, Role::Cercetasul | Role::Cronicarul) && n != 1)
                || (role == Role::Groparul && n < 2)
            {
                continue;
            }
            for i in 0..N {
                if !self.alive[i] || self.believed[i] != role {
                    continue;
                }
                let pick = if role == Role::Ghicitoarea {
                    if i == 0 {
                        self.hp
                    } else {
                        Some(self.bot_divine(i))
                    }
                } else {
                    None
                };
                let truth = if self.mal(i) && role != Role::Ghicitoarea {
                    // pentru Bețiv „adevărul” se calculează cu rolul crezut
                    self.true_info(i, role, pick, n)
                } else {
                    self.true_info(i, role, pick, n)
                };
                let info = if self.mal(i) {
                    self.fake_info(i, role, truth, n)
                } else {
                    truth
                };
                if let Some(info) = info {
                    self.received[i].push(Stmt {
                        night: n as u8,
                        info,
                    });
                }
            }
        }
        // zori
        if died.is_empty() {
            self.say(format!("Zorii zilei {}: nimeni nu a murit.", n));
        } else {
            let d: Vec<String> = died.iter().map(|i| (i + 1).to_string()).collect();
            self.say(format!(
                "Zorii zilei {}: au murit locurile {}.",
                n,
                d.join(", ")
            ));
        }
        self.settle(alive_before);
        if self.winner.is_some() {
            return;
        }
        self.dawn();
    }

    /// După orice moarte: moștenire, verificarea finalului.
    fn settle(&mut self, alive_before: usize) {
        if self.winner.is_some() {
            return;
        }
        if self.seat_of(Role::Strigoiul).is_none() {
            let heir = self.seat_of(Role::Mostenitoarea);
            match heir {
                Some(h) if alive_before >= 5 && !self.poisoned[h] => {
                    self.roles[h] = Role::Strigoiul;
                    self.believed[h] = Role::Strigoiul;
                    if h == 0 {
                        self.say("Ai devenit noul Strigoi.".into());
                    }
                }
                _ => {
                    self.finish(true);
                    return;
                }
            }
        }
        if self.alive_count() <= 2 {
            self.finish(false);
        }
    }

    // ---------- ziua ----------
    fn dawn(&mut self) {
        self.day += 1;
        if self.day > DAY_CAP {
            self.say("Zilele s-au scurs: răul câștigă.".into());
            self.finish(false);
            return;
        }
        self.nominated = [false; N];
        self.nom_by = [false; N];
        self.noms_today = 0;
        self.nom_i = 0;
        self.nom_start = self.rng.below(N);
        self.block = None;
        self.block_votes = 0;
        self.declared = false;
        let p = (50 + 15 * (self.day.min(4) as usize - 1)).min(95);
        for i in 1..N {
            if !self.alive[i] {
                continue;
            }
            if self.claim[i].is_some() || self.rng.below(100) < p {
                self.publish(i);
            }
        }
        self.phase = Phase::DayTalk;
    }

    /// Un robot își spune (sau își actualizează) afirmațiile.
    fn publish(&mut self, i: usize) {
        let mut lines: Vec<String> = Vec::new();
        if self.roles[i].evil() {
            let role = self.bluff[i];
            if self.claim[i].is_none() {
                self.claim[i] = Some(role);
                lines.push(format!("Locul {}: «Sunt {}.»", i + 1, role.name()));
            }
            for nn in (self.pub_night[i] + 1)..=self.night {
                let info = self.fake_info(i, role, None, nn);
                let ok = match role {
                    Role::Cronicarul | Role::Cercetasul => nn == 1,
                    Role::Groparul => nn >= 2,
                    _ => true,
                };
                if let (Some(info), true) = (info, ok) {
                    let st = Stmt {
                        night: nn as u8,
                        info,
                    };
                    lines.push(format!("Locul {}: {}", i + 1, fmt_stmt(&st)));
                    self.public[i].push(st);
                }
            }
            self.pub_night[i] = self.night;
        } else {
            let role = self.believed[i];
            if self.claim[i].is_none() {
                self.claim[i] = Some(role);
                lines.push(format!("Locul {}: «Sunt {}.»", i + 1, role.name()));
            }
            let from = self.shared[i];
            let new: Vec<Stmt> = self.received[i][from..].to_vec();
            self.shared[i] = self.received[i].len();
            for st in new {
                lines.push(format!("Locul {}: {}", i + 1, fmt_stmt(&st)));
                self.public[i].push(st);
            }
        }
        for l in lines {
            self.say(l);
        }
    }

    /// Suspiciunea unui observator bun față de fiecare loc.
    fn suspicion(&self, b: usize) -> Vec<f32> {
        let mut sus = vec![0.0f32; N];
        let apply = |sus: &mut Vec<f32>, st: &Stmt, w: f32, who: usize| match st.info {
            Info::Test(a, c, hit) => {
                for x in [a as usize, c as usize] {
                    if x == b {
                        if hit {
                            sus[who] += 1.0 * w.max(0.5);
                        }
                        continue;
                    }
                    if hit {
                        sus[x] += 1.2 * w;
                    } else {
                        sus[x] -= 0.8 * w;
                    }
                }
            }
            Info::Count(c, l, r) => {
                let (l, r) = (l as usize, r as usize);
                if (l == b || r == b) && c == 2 {
                    sus[who] += 2.0;
                }
                for x in [l, r] {
                    if x == b || x == who {
                        continue;
                    }
                    match c {
                        2 => sus[x] += 2.0 * w,
                        0 => sus[x] -= 1.5 * w,
                        _ => sus[x] += 0.7 * w,
                    }
                }
            }
            Info::Scout(a, c, _) => {
                for x in [a as usize, c as usize] {
                    if x != b {
                        sus[x] -= 0.3 * w;
                    }
                }
            }
            Info::Chain(_) | Info::Grave(_, _) => {}
        };
        for st in &self.received[b] {
            apply(&mut sus, st, 1.0, b);
        }
        for j in 0..N {
            if j == b {
                continue;
            }
            for st in &self.public[j] {
                apply(&mut sus, st, 0.7, j);
            }
        }
        // pretenții duble
        for r in TOWN {
            let holders: Vec<usize> = (0..N)
                .filter(|&j| j != b && self.claim[j] == Some(r) && self.alive[j])
                .collect();
            let mine = self.believed[b] == r && !self.roles[b].evil();
            if holders.len() >= 2 || (mine && !holders.is_empty()) {
                for &j in &holders {
                    sus[j] += if mine { 2.0 } else { 1.0 };
                }
            }
        }
        for (j, s) in sus.iter_mut().enumerate() {
            if j != b && self.alive[j] && self.claim[j].is_none() {
                *s += 0.3;
            }
        }
        sus[b] = -9.0;
        sus
    }

    fn bot_nominee(&mut self, s: usize) -> Option<usize> {
        let cand: Vec<usize> = self
            .alive_list()
            .into_iter()
            .filter(|&j| j != s && !self.nominated[j])
            .collect();
        if cand.is_empty() {
            return None;
        }
        if self.roles[s].evil() {
            if self.rng.below(100) >= 40 {
                return None;
            }
            let good: Vec<usize> = cand
                .iter()
                .copied()
                .filter(|&j| !self.roles[j].evil())
                .collect();
            if good.is_empty() {
                return None;
            }
            let w: Vec<u32> = good
                .iter()
                .map(|&j| (10.0 + 20.0 * self.threat(j)) as u32)
                .collect();
            return Some(good[pick_weighted(&mut self.rng, &w)]);
        }
        let sus = self.suspicion(s);
        let mut best = cand[0];
        for &j in &cand {
            if sus[j] > sus[best] {
                best = j;
            }
        }
        let skip = self.claim[best] == Some(Role::Fecioara) && sus[best] < 2.5;
        if sus[best] >= 0.9 && !skip && self.rng.below(100) < 75 {
            Some(best)
        } else {
            None
        }
    }

    fn bot_vote(&mut self, i: usize) -> bool {
        let t = self.nominee;
        let dead = !self.alive[i];
        let yes = if self.roles[i].evil() {
            if self.roles[t].evil() {
                self.roles[t] == Role::Mostenitoarea
                    && self.alive_count() > 6
                    && self.rng.below(100) < 15
            } else {
                self.rng.below(100) < 70
            }
        } else {
            let sus = self.suspicion(i);
            let thr = if dead { 2.0 } else { 1.0 };
            let beats = match self.block {
                Some(bk) => sus[t] >= sus[bk] - 0.2,
                None => true,
            };
            sus[t] >= thr && beats && t != i
        };
        if yes && dead {
            self.ghost[i] = false;
        }
        yes
    }

    fn slay(&mut self, s: usize, t: usize) {
        self.shot[s] = true;
        self.claim[s] = Some(Role::Vanatorul);
        let alive_before = self.alive_count();
        let hit = !self.mal(s) && self.roles[t] == Role::Strigoiul;
        self.say(format!(
            "Locul {} (Vânătorul) țintește locul {}: {}.",
            s + 1,
            t + 1,
            if hit { "Strigoiul cade" } else { "nimic" }
        ));
        if hit {
            self.kill(t);
            self.settle(alive_before);
        }
    }

    fn advance_noms(&mut self) {
        while self.nom_i < N {
            let s = (self.nom_start + self.nom_i) % N;
            if self.alive[s] && !self.nom_by[s] && self.noms_today < MAX_NOMS {
                self.phase = if s == 0 {
                    Phase::NomHuman
                } else {
                    Phase::NomBot
                };
                return;
            }
            self.nom_i += 1;
        }
        self.phase = Phase::DayEnd;
    }

    fn start_vote(&mut self, by: usize, t: usize) {
        self.nominator = by;
        self.nominee = t;
        self.nom_by[by] = true;
        self.nominated[t] = true;
        self.noms_today += 1;
        self.yes = 0;
        self.thresh = self.alive_count().div_ceil(2);
        self.say(format!(
            "Locul {} îl nominalizează pe locul {}.",
            by + 1,
            t + 1
        ));
        self.phase = Phase::VoteBots;
    }

    fn execute(&mut self, i: usize) {
        let role = self.roles[i];
        let alive_before = self.alive_count();
        self.kill(i);
        self.execs.push((self.day, i as u8, role));
        self.say(format!("Locul {} e executat.", i + 1));
        if role == Role::Sfantul && !self.poisoned[i] {
            self.say("Sfântul a murit pe rug: răul câștigă.".into());
            self.finish(false);
            return;
        }
        self.settle(alive_before);
    }

    fn vote_bots(&mut self) {
        let (by, t) = (self.nominator, self.nominee);
        if self.roles[t] == Role::Fecioara && self.alive[t] && !self.virgin_spent {
            self.virgin_spent = true;
            if !self.poisoned[t] && self.roles[by].is_town() {
                self.claim[t] = Some(Role::Fecioara);
                self.say(format!(
                    "Locul {} era Fecioara: cel care a nominalizat (locul {}) e executat pe loc.",
                    t + 1,
                    by + 1
                ));
                self.execute(by);
                if self.winner.is_none() {
                    self.block = None;
                    self.poisoned = [false; N];
                    self.start_night_after_day();
                }
                return;
            }
        }
        let mut yes = 0;
        for i in 1..N {
            if (self.alive[i] || self.ghost[i]) && self.bot_vote(i) {
                yes += 1;
            }
        }
        self.yes = yes;
        self.phase = if self.alive[0] || self.ghost[0] {
            Phase::VoteHuman
        } else {
            Phase::Tally
        };
    }

    fn start_night_after_day(&mut self) {
        if self.winner.is_none() {
            self.start_night();
        }
    }

    fn tally(&mut self) {
        let t = self.nominee;
        let yes = self.yes;
        self.say(format!(
            "Voturi pentru locul {}: {} (prag {}).",
            t + 1,
            yes,
            self.thresh
        ));
        if yes >= self.thresh {
            if yes > self.block_votes {
                self.block = Some(t);
                self.block_votes = yes;
                self.say(format!("Locul {} e pe butuc.", t + 1));
            } else if yes == self.block_votes {
                self.block = None;
                self.say("Egalitate: nimeni nu e pe butuc.".into());
            }
        }
        self.nom_i += 1;
        self.advance_noms();
    }

    fn day_end(&mut self) {
        if let Some(b) = self.block {
            self.execute(b);
        } else {
            self.say("Nimeni nu e executat azi.".into());
        }
        if self.winner.is_some() {
            return;
        }
        self.poisoned = [false; N];
        self.start_night();
    }

    fn nom_bot(&mut self) {
        while self.phase == Phase::NomBot {
            let s = (self.nom_start + self.nom_i) % N;
            match self.bot_nominee(s) {
                Some(t) => {
                    self.start_vote(s, t);
                    return;
                }
                None => {
                    self.nom_by[s] = true;
                    self.nom_i += 1;
                    self.advance_noms();
                }
            }
        }
    }

    fn nom_start_step(&mut self) {
        for s in 1..N {
            if self.alive[s]
                && self.believed[s] == Role::Vanatorul
                && !self.roles[s].evil()
                && !self.shot[s]
                && self.winner.is_none()
            {
                let sus = self.suspicion(s);
                let mut best = None;
                for j in self.alive_list() {
                    if j != s && best.is_none_or(|b: usize| sus[j] > sus[b]) {
                        best = Some(j);
                    }
                }
                if let Some(b) = best {
                    if sus[b] >= 1.5 && self.rng.below(100) < 50 {
                        self.slay(s, b);
                    }
                }
            }
        }
        if self.winner.is_none() {
            self.advance_noms();
        }
    }

    fn mint(&mut self, r: Role) {
        self.claim[0] = Some(r);
        self.public[0].clear();
        for nn in 1..=self.night {
            let ok = match r {
                Role::Cronicarul | Role::Cercetasul => nn == 1,
                Role::Groparul => nn >= 2,
                _ => true,
            };
            if !ok {
                continue;
            }
            if let Some(info) = self.fake_info(0, r, None, nn) {
                self.public[0].push(Stmt {
                    night: nn as u8,
                    info,
                });
            }
        }
        self.declared = true;
        self.say(format!("Tu: «Sunt {}.»", r.name()));
    }

    fn visible_value(&self) -> i32 {
        // informație vizibilă: numărul de vii (răul vrea puțini, binele mulți)
        let a = self.alive_count() as i32;
        if self.roles[0].evil() {
            -a
        } else {
            a
        }
    }
}

impl Ceasornic {
    /// Normal rendering (human = J1's point of view).
    fn render_main(&self) -> String {
        let mut s = format!(
            "CEASORNICUL DIN VALEA CEȚII — {} {} — vii: {}/{}\n",
            match self.phase {
                Phase::NightChoose | Phase::NightResolve => "noaptea",
                _ => "ziua",
            },
            if matches!(self.phase, Phase::NightChoose | Phase::NightResolve) {
                self.night
            } else {
                self.day
            },
            self.alive_count(),
            N
        );
        s.push_str(&format!(
            "Rolul tău (locul 1): {}{}\n",
            self.believed[0].name(),
            if self.alive[0] {
                ""
            } else {
                " — ești mort (vot fantomă dacă nu l-ai folosit)"
            }
        ));
        for st in &self.received[0] {
            s.push_str(&format!("  notiță: {}\n", fmt_stmt(st)));
        }
        s.push_str("Masa:");
        for i in 0..N {
            let c = match self.claim[i] {
                Some(r) => r.name().to_string(),
                None => "?".into(),
            };
            s.push_str(&format!(
                " [{}{}{} {}]",
                i + 1,
                if self.alive[i] {
                    ""
                } else if self.ghost[i] {
                    "†v"
                } else {
                    "†"
                },
                if i == 0 { "*" } else { "" },
                c
            ));
        }
        s.push('\n');
        match self.phase {
            Phase::NightChoose => s.push_str("Alegi ținta nopții.\n"),
            Phase::NomHuman => s.push_str("E rândul tău să nominalizezi (sau pas).\n"),
            Phase::VoteHuman => s.push_str(&format!(
                "Votezi pentru executarea locului {}? (voturi roboți: {}, prag {})\n",
                self.nominee + 1,
                self.yes,
                self.thresh
            )),
            _ => {}
        }
        for l in &self.log {
            s.push_str(l);
            s.push('\n');
        }
        if let Some(good) = self.winner {
            s.push_str(if good {
                "Au câștigat oamenii buni.\n"
            } else {
                "A câștigat echipa rea.\n"
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
        for i in 0..N {
            s.push_str(&format!(
                "  locul {}: rol {}, crede {}{}{}\n",
                i + 1,
                self.roles[i].name(),
                self.believed[i].name(),
                if self.alive[i] { "" } else { " (mort)" },
                if self.poisoned[i] { " (otrăvit)" } else { "" }
            ));
        }
        s
    }
}

impl Game for Ceasornic {
    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }
    type Move = Move;

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::NightChoose | Phase::NomHuman | Phase::VoteHuman | Phase::Over => Player::One,
            Phase::DayTalk if self.alive[0] => Player::One,
            _ => Player::Two,
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        let mut v = Vec::new();
        match self.phase {
            Phase::NightChoose => {
                let alive = self.alive_list();
                match self.believed[0] {
                    Role::Ghicitoarea => {
                        for (x, &a) in alive.iter().enumerate() {
                            for &b in &alive[x + 1..] {
                                v.push(Move::Divine(a as u8, b as u8));
                            }
                        }
                    }
                    Role::Otravitorul => {
                        for &a in &alive {
                            v.push(Move::Poison(a as u8));
                        }
                    }
                    Role::Calugarul => {
                        for &a in &alive {
                            if a != 0 {
                                v.push(Move::Guard(a as u8));
                            }
                        }
                    }
                    Role::Strigoiul => {
                        for &a in &alive {
                            v.push(Move::Kill(a as u8));
                        }
                    }
                    _ => {}
                }
            }
            Phase::DayTalk => {
                if self.alive[0] {
                    if !self.declared {
                        if !self.roles[0].evil() {
                            v.push(Move::Declare);
                        }
                        for r in TOWN {
                            v.push(Move::Mint(r));
                        }
                    }
                    for i in 1..N {
                        if self.alive[i] && self.claim[i].is_none() {
                            v.push(Move::Ask(i as u8));
                        }
                    }
                    if self.believed[0] == Role::Vanatorul && !self.shot[0] {
                        for i in 1..N {
                            if self.alive[i] {
                                v.push(Move::Slay(i as u8));
                            }
                        }
                    }
                    v.push(Move::Done);
                } else {
                    v.push(Move::Table);
                }
            }
            Phase::NomHuman => {
                for i in 1..N {
                    if self.alive[i] && !self.nominated[i] {
                        v.push(Move::Nominate(i as u8));
                    }
                }
                v.push(Move::Done);
            }
            Phase::VoteHuman => {
                v.push(Move::Yes);
                v.push(Move::No);
            }
            Phase::Over => {}
            _ => v.push(Move::Table),
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Divine(a, b) => {
                self.hp = Some((*a as usize, *b as usize));
                self.phase = Phase::NightResolve;
            }
            Move::Poison(a) | Move::Guard(a) | Move::Kill(a) => {
                self.hp = Some((*a as usize, *a as usize));
                self.phase = Phase::NightResolve;
            }
            Move::Declare => {
                self.claim[0] = Some(self.believed[0]);
                self.public[0] = self.received[0].clone();
                self.declared = true;
                self.say(format!("Tu: «Sunt {}.»", self.believed[0].name()));
                let lines: Vec<String> = self.public[0].iter().map(fmt_stmt).collect();
                for l in lines {
                    self.say(format!("Tu: {l}"));
                }
            }
            Move::Mint(r) => self.mint(*r),
            Move::Ask(i) => {
                let i = *i as usize;
                self.publish(i);
            }
            Move::Slay(t) => {
                self.slay(0, *t as usize);
                if self.winner.is_some() {
                    self.phase = Phase::Over;
                }
            }
            Move::Done => match self.phase {
                Phase::DayTalk => self.phase = Phase::NomStart,
                Phase::NomHuman => {
                    self.nom_by[0] = true;
                    self.nom_i += 1;
                    self.advance_noms();
                }
                _ => {}
            },
            Move::Nominate(t) => {
                self.start_vote(0, *t as usize);
            }
            Move::Yes | Move::No => {
                if *m == Move::Yes {
                    self.yes += 1;
                    if !self.alive[0] {
                        self.ghost[0] = false;
                    }
                }
                self.phase = Phase::Tally;
            }
            Move::Table => match self.phase {
                Phase::NightResolve => self.resolve_night(),
                Phase::DayTalk => self.phase = Phase::NomStart,
                Phase::NomStart => self.nom_start_step(),
                Phase::NomBot => self.nom_bot(),
                Phase::VoteBots => self.vote_bots(),
                Phase::Tally => self.tally(),
                Phase::DayEnd => self.day_end(),
                _ => {}
            },
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        let good = self.winner?;
        Some(Outcome::Win(if good != self.roles[0].evil() {
            Player::One
        } else {
            Player::Two
        }))
    }

    fn evaluate(&self, p: Player) -> i32 {
        let v = self.visible_value();
        if p == Player::One {
            v
        } else {
            -v
        }
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
            Move::Divine(a, b) => format!("ghicesc {} {}", a + 1, b + 1),
            Move::Poison(a) => format!("otravesc {}", a + 1),
            Move::Guard(a) => format!("ocrotesc {}", a + 1),
            Move::Kill(a) => format!("ucid {}", a + 1),
            Move::Declare => "declar".into(),
            Move::Mint(r) => format!("mint {}", r.key()),
            Move::Ask(i) => format!("cer {}", i + 1),
            Move::Slay(i) => format!("vanez {}", i + 1),
            Move::Done => {
                if self.phase == Phase::NomHuman {
                    "pas".into()
                } else {
                    "treci".into()
                }
            }
            Move::Nominate(i) => format!("nominez {}", i + 1),
            Move::Yes => "da".into(),
            Move::No => "nu".into(),
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

    fn run(seed: u64) -> Ceasornic {
        let mut g = Ceasornic::with_seed(seed);
        let mut rng = Rng::new(seed ^ 0x55);
        let mut n = 0;
        while g.outcome().is_none() {
            let mv = g.legal_moves();
            assert!(!mv.is_empty(), "{}", g.render());
            g.apply(&mv[rng.below(mv.len())]);
            n += 1;
            assert!(n < 20000, "{}", g.render());
        }
        g
    }

    fn custom(seed: u64, roles: [Role; N]) -> Ceasornic {
        let mut g = Ceasornic::with_seed(seed);
        g.roles = roles;
        g.believed = roles;
        g.herring = 5;
        g.log.clear();
        g
    }

    const BASE: [Role; N] = [
        Role::Veghea,
        Role::Soldatul,
        Role::Calugarul,
        Role::Strigoiul,
        Role::Otravitorul,
        Role::Sfantul,
        Role::Ghicitoarea,
        Role::Fecioara,
        Role::Groparul,
        Role::Vanatorul,
        Role::Mostenitoarea,
        Role::Cronicarul,
    ];

    fn at_resolve(g: &mut Ceasornic, night: u32, hp: Option<(usize, usize)>) {
        g.night = night;
        g.hp = hp;
        g.phase = Phase::NightResolve;
    }

    #[test]
    fn cast_is_consistent() {
        for seed in 1..200u64 {
            let g = Ceasornic::with_seed(seed);
            let c = |r: Role| g.roles.iter().filter(|&&x| x == r).count();
            assert_eq!(c(Role::Strigoiul), 1);
            assert_eq!(c(Role::Otravitorul), 1);
            assert_eq!(c(Role::Mostenitoarea), 1);
            assert_eq!(c(Role::Sfantul), 1);
            assert_eq!(c(Role::Betivul), 1);
            let towns = g.roles.iter().filter(|r| r.is_town()).count();
            assert_eq!(towns, 7);
            for r in TOWN {
                assert!(c(r) <= 1);
            }
            let d = g.roles.iter().position(|&r| r == Role::Betivul).unwrap();
            assert!(g.believed[d].is_town());
            assert_eq!(c(g.believed[d]), 0, "bețivul crede un rol care nu e în joc");
            assert!(!g.roles[g.herring].evil());
        }
    }

    #[test]
    fn random_games_end_and_both_sides_win() {
        let (mut good, mut evil) = (0, 0);
        for seed in 1..300u64 {
            let g = run(seed);
            match g.good_won() {
                Some(true) => good += 1,
                Some(false) => evil += 1,
                None => panic!("fără câștigător"),
            }
            assert!(g.render().contains("Roluri:"));
        }
        assert!(good > 20 && evil > 20, "{good} {evil}");
    }

    fn demon_game(seed: u64) -> Ceasornic {
        let mut roles = BASE;
        roles.swap(0, 3); // locul 1 = Strigoiul, locul 4 = Veghea
        let mut g = custom(seed, roles);
        g.alive[4] = false; // fără Otrăvitor activ
        g
    }

    #[test]
    fn soldier_is_immune_unless_poisoned() {
        let mut g = demon_game(1);
        at_resolve(&mut g, 2, Some((1, 1)));
        g.resolve_night();
        assert!(g.alive[1], "Soldatul nu moare");
        let mut g = demon_game(1);
        g.roles[2] = Role::Cronicarul;
        g.believed[2] = Role::Cronicarul;
        g.poisoned[1] = true;
        at_resolve(&mut g, 2, Some((1, 1)));
        g.resolve_night();
        assert!(!g.alive[1], "Soldatul otrăvit moare");
    }

    #[test]
    fn demon_kills_when_unprotected_and_monk_protects() {
        // fără Călugăr
        let mut g = demon_game(2);
        g.roles[2] = Role::Cronicarul;
        g.believed[2] = Role::Cronicarul;
        at_resolve(&mut g, 2, Some((6, 6)));
        g.resolve_night();
        assert!(!g.alive[6]);
        // cu Călugăr: dacă ocrotește ținta, aceasta trăiește; altfel moare
        let (mut saved, mut dead) = (0, 0);
        for seed in 1..80u64 {
            let mut g = demon_game(seed);
            at_resolve(&mut g, 2, Some((6, 6)));
            g.resolve_night();
            if g.protected == Some(6) {
                assert!(g.alive[6]);
                saved += 1;
            } else {
                assert!(!g.alive[6]);
                dead += 1;
            }
        }
        assert!(saved > 0 && dead > 0);
        // Călugăr otrăvit: fără protecție
        let mut g = demon_game(3);
        g.poisoned[2] = true;
        at_resolve(&mut g, 2, Some((6, 6)));
        g.resolve_night();
        assert_eq!(g.protected, None);
        assert!(!g.alive[6]);
    }

    #[test]
    fn demon_may_pass_the_curse_by_suicide() {
        let mut g = demon_game(4);
        at_resolve(&mut g, 2, Some((0, 0)));
        g.resolve_night();
        assert!(!g.alive[0]);
        assert_eq!(
            (0..N)
                .filter(|&i| g.alive[i] && g.roles[i] == Role::Strigoiul)
                .count(),
            1,
            "un nou Strigoi"
        );
        assert_eq!(g.winner, None);
    }

    #[test]
    fn night_one_has_no_kill() {
        for seed in 1..40u64 {
            let mut g = Ceasornic::with_seed(seed);
            while g.phase != Phase::DayTalk && g.phase != Phase::Over {
                let mv = g.legal_moves();
                g.apply(&mv[0]);
            }
            assert_eq!(g.alive_count(), N, "noaptea 1 nu moare nimeni");
        }
    }

    #[test]
    fn poisoned_and_drunk_get_false_info() {
        for seed in 1..60u64 {
            let mut g = custom(seed, BASE);
            g.roles[0] = Role::Cronicarul;
            g.roles[11] = Role::Veghea;
            g.believed = g.roles;
            // Veghea e locul 12; vecinii 11 și 1
            let (l, r) = g.neighbours(11);
            let mut c = 0;
            if g.roles[l].evil() {
                c += 1;
            }
            if g.roles[r].evil() {
                c += 1;
            }
            let t = g.true_info(11, Role::Veghea, None, 1);
            assert_eq!(t, Some(Info::Count(c, l as u8, r as u8)));
            let f = g.fake_info(11, Role::Veghea, t, 1);
            assert_ne!(f, t);
            let ch = g.true_info(0, Role::Cronicarul, None, 1);
            let fc = g.fake_info(0, Role::Cronicarul, ch, 1);
            assert_ne!(ch, fc);
        }
    }

    #[test]
    fn chronicler_counts_adjacent_evil_pairs() {
        let mut roles = BASE;
        // locurile 4,5 (Strigoi, Otrăvitor) adiacente; Moștenitoarea la 11 între 10 și 12 (bune)
        let mut g = custom(9, roles);
        assert_eq!(
            g.true_info(11, Role::Cronicarul, None, 1),
            Some(Info::Chain(1))
        );
        roles.swap(4, 8);
        let mut g2 = custom(9, roles);
        assert_eq!(
            g2.true_info(11, Role::Cronicarul, None, 1),
            Some(Info::Chain(0))
        );
    }

    #[test]
    fn fortune_teller_sees_demon_and_herring() {
        let mut g = custom(5, BASE);
        let t = g.true_info(6, Role::Ghicitoarea, Some((3, 0)), 1);
        assert_eq!(t, Some(Info::Test(3, 0, true)));
        let t = g.true_info(6, Role::Ghicitoarea, Some((0, 1)), 1);
        assert_eq!(t, Some(Info::Test(0, 1, false)));
        g.herring = 1;
        let t = g.true_info(6, Role::Ghicitoarea, Some((0, 1)), 1);
        assert_eq!(t, Some(Info::Test(0, 1, true)));
        let t = g.true_info(6, Role::Ghicitoarea, Some((4, 10)), 1);
        assert_eq!(
            t,
            Some(Info::Test(4, 10, false)),
            "slugile nu apar ca Strigoi"
        );
    }

    #[test]
    fn scout_tells_the_truth() {
        for seed in 1..40u64 {
            let mut g = custom(seed, BASE);
            let t = g.true_info(5, Role::Cercetasul, None, 1);
            if let Some(Info::Scout(a, b, r)) = t {
                assert!(g.roles[a as usize] == r || g.roles[b as usize] == r);
                assert!(r.is_town());
            } else {
                panic!();
            }
        }
    }

    #[test]
    fn saint_execution_ends_game_for_evil() {
        let mut g = custom(6, BASE);
        g.phase = Phase::DayEnd;
        g.day = 3;
        g.block = Some(5);
        g.day_end();
        assert_eq!(g.winner, Some(false));
        // Sfântul otrăvit: nu contează
        let mut g = custom(6, BASE);
        g.phase = Phase::DayEnd;
        g.poisoned[5] = true;
        g.block = Some(5);
        g.day_end();
        assert_eq!(g.winner, None);
    }

    #[test]
    fn demon_execution_heir_and_win() {
        let mut g = custom(7, BASE);
        g.block = Some(3);
        g.day_end();
        // 12 vii => Moștenitoarea (locul 11) devine Strigoi
        assert_eq!(g.winner, None);
        assert_eq!(g.roles[10], Role::Strigoiul);
        // sub 5 vii (4 înainte de execuție): nu se mai moștenește, binele câștigă
        let mut g = custom(7, BASE);
        for i in 0..N {
            g.alive[i] = matches!(i, 0 | 1 | 3 | 10);
        }
        g.block = Some(3);
        g.day_end();
        assert_eq!(g.winner, Some(true));
    }

    #[test]
    fn evil_wins_with_two_alive() {
        let mut g = custom(8, BASE);
        for i in 0..N {
            g.alive[i] = i == 3 || i == 0 || i == 1;
        }
        g.block = Some(1);
        g.day_end();
        assert_eq!(g.winner, Some(false));
    }

    #[test]
    fn slayer_kills_demon_only() {
        let mut g = custom(10, BASE);
        let ab = g.alive_count();
        g.slay(9, 2);
        assert!(g.alive[2]);
        assert!(g.shot[9]);
        let mut g = custom(10, BASE);
        g.slay(9, 3);
        assert!(!g.alive[3]);
        assert_eq!(g.roles[10], Role::Strigoiul, "moștenire");
        let _ = ab;
        // otrăvit: eșuează
        let mut g = custom(10, BASE);
        g.poisoned[9] = true;
        g.slay(9, 3);
        assert!(g.alive[3]);
    }

    #[test]
    fn virgin_executes_town_nominator() {
        let mut g = custom(11, BASE);
        g.phase = Phase::VoteBots;
        g.nominator = 6; // Ghicitoarea (localnic)
        g.nominee = 7; // Fecioara
        g.vote_bots();
        assert!(!g.alive[6]);
        // a doua oară nu mai funcționează
        let mut g = custom(11, BASE);
        g.virgin_spent = true;
        g.nominator = 6;
        g.nominee = 7;
        g.nom_by[6] = true;
        g.vote_bots();
        assert!(g.alive[6]);
        // nominalizator străin/rău: nimic
        let mut g = custom(11, BASE);
        g.nominator = 5; // Sfântul (străin)
        g.nominee = 7;
        g.vote_bots();
        assert!(g.alive[5]);
    }

    #[test]
    fn ghost_vote_is_used_once() {
        let mut g = custom(12, BASE);
        g.kill(6);
        assert!(g.ghost[6]);
        g.nominee = 8;
        g.roles[6] = Role::Strigoiul; // dacă e rău, votează 70% – folosește votul
        g.roles[3] = Role::Veghea;
        let mut used = false;
        for _ in 0..40 {
            let mut h = g.clone();
            h.ghost[6] = true;
            let y = h.bot_vote(6);
            if y {
                assert!(!h.ghost[6]);
                used = true;
            }
        }
        assert!(used);
    }

    #[test]
    fn threshold_and_block_ties() {
        let mut g = custom(13, BASE);
        g.nominee = 2;
        g.thresh = 6;
        g.yes = 6;
        g.tally();
        assert_eq!(g.block, Some(2));
        g.nominee = 4;
        g.yes = 6;
        g.tally();
        assert_eq!(g.block, None, "egalitate");
        g.nominee = 8;
        g.yes = 5;
        g.tally();
        assert_eq!(g.block, None, "sub prag");
        g.nominee = 9;
        g.yes = 7;
        g.tally();
        assert_eq!(g.block, Some(9));
    }

    #[test]
    fn no_leak_before_end() {
        for seed in 1..60u64 {
            let g = Ceasornic::with_seed(seed);
            let r = g.render();
            assert!(!r.contains("Roluri:"));
            if !g.roles[0].evil() {
                assert!(!r.contains("Strigoi"), "{r}");
                assert!(!r.contains("Otrăvitor"), "{r}");
                assert!(!r.contains("Moștenitoarea"), "{r}");
            }
        }
    }

    #[test]
    fn evil_human_knows_team() {
        for seed in 1..80u64 {
            let g = Ceasornic::with_seed(seed);
            if g.roles[0].evil() {
                assert!(g.render().contains("Echipa ta rea"));
                return;
            }
        }
        panic!("niciun jucător rău");
    }

    #[test]
    fn bots_use_only_private_knowledge() {
        // Observatorul bun nu are acces la roluri: suspiciunea fără nicio afirmație e uniformă
        let g = Ceasornic::with_seed(21);
        let b = (1..N).find(|&i| !g.roles[i].evil()).unwrap();
        let sus = g.suspicion(b);
        let first = sus.iter().enumerate().find(|&(j, _)| j != b).unwrap().1;
        for (j, s) in sus.iter().enumerate() {
            if j != b {
                assert!((s - first).abs() < 1e-6 || !g.received[b].is_empty());
            }
        }
    }

    #[test]
    fn ai_is_fast_and_legal() {
        use polytabletop_core::GameObj;
        let mut g: Box<dyn GameObj> = Box::new(Ceasornic::with_seed(3));
        let t = std::time::Instant::now();
        let mut rng = Rng::new(9);
        let mut n = 0;
        while g.outcome().is_none() && n < 3000 {
            let m = g.ai_move(rng.next_u64()).unwrap();
            g.play(&m).unwrap();
            n += 1;
        }
        assert!(g.outcome().is_some());
        assert!(t.elapsed().as_secs_f32() / (n as f32) < 1.0);
    }

    #[test]
    fn human_moves_are_unique_texts() {
        for seed in 1..30u64 {
            let mut g = Ceasornic::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut k = 0;
            while g.outcome().is_none() && k < 400 {
                let mv: Vec<String> = g.legal_moves().iter().map(|m| g.format_move(m)).collect();
                let mut s = mv.clone();
                s.sort();
                s.dedup();
                assert_eq!(s.len(), mv.len());
                let m = g.legal_moves();
                g.apply(&m[rng.below(m.len())]);
                k += 1;
            }
        }
    }
}
