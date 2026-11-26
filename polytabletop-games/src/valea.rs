//! VALEA USCATĂ — duel western cu roluri ascunse, pentru 5 pistolari (variantă originală).
//!
//! Tu ești locul 1; locurile 2–5 sunt roboți (masa, `Player::Two`). Roluri: un **Șerif**
//! (public), un **Adjunct**, doi **Haiduci** și un **Trădător**. Șeriful și Adjunctul
//! câștigă dacă mor toți haiducii și trădătorul; haiducii câștigă dacă moare Șeriful;
//! Trădătorul câștigă doar dacă rămâne ultimul în viață.
//! Viață: 5 (Șerif) sau 4. Pe tură tragi 2 cărți, joci câte vrei, apoi arunci până
//! la câte vieți ai. Cărți: **Foc** (o dată pe tură, la distanță 1 — sau 2 cu **Pușcă**),
//! **Ocolire** (anulează un Foc/Mitralieră asupra ta), **Leac** (+1 viață),
//! **Salon** (+1 viață pentru toți), **Mitralieră** (Foc la toți ceilalți),
//! **Pușcă** (echipament: bătaie 2), **Panică** (iei o carte la întâmplare de la un vecin).
//! Cine omoară un haiduc trage 3 cărți; Șeriful care omoară Adjunctul aruncă tot.
//! Secetă: din runda 30, fiecare pierde 1 viață la începutul turei lui (garantează finalul).
//! Roboții își deduc aliații doar din fapte publice (cine a tras în Șerif).

use crate::w5_util::{pick_weighted, shuffle_vec, trim_log};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const N: usize = 5;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Role {
    Sheriff,
    Deputy,
    Outlaw,
    Renegade,
}

impl Role {
    fn name(self) -> &'static str {
        match self {
            Role::Sheriff => "Șerif",
            Role::Deputy => "Adjunct",
            Role::Outlaw => "Haiduc",
            Role::Renegade => "Trădător",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Card {
    Shot,
    Dodge,
    Heal,
    Saloon,
    Gatling,
    Rifle,
    Panic,
}

const CARDS: [Card; 7] = [
    Card::Shot,
    Card::Dodge,
    Card::Heal,
    Card::Saloon,
    Card::Gatling,
    Card::Rifle,
    Card::Panic,
];

impl Card {
    fn name(self) -> &'static str {
        match self {
            Card::Shot => "foc",
            Card::Dodge => "ocolire",
            Card::Heal => "leac",
            Card::Saloon => "salon",
            Card::Gatling => "mitraliera",
            Card::Rifle => "pusca",
            Card::Panic => "panica",
        }
    }
}

fn deck_new() -> Vec<Card> {
    let mut d = Vec::new();
    for (c, k) in [
        (Card::Shot, 22),
        (Card::Dodge, 12),
        (Card::Heal, 7),
        (Card::Saloon, 2),
        (Card::Gatling, 2),
        (Card::Rifle, 5),
        (Card::Panic, 6),
    ] {
        for _ in 0..k {
            d.push(c);
        }
    }
    d
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Phase {
    Play,
    Discard,
    React(usize, bool), // sursa, true = mitralieră
    Robot,
    Over,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Play(Card),
    Target(Card, u8),
    End,
    Throw(Card),
    Dodge,
    Accept,
    Table,
}

#[derive(Clone)]
pub struct Valea {
    /// Whose private info `render` shows (`None` = the mover's, hot-seat).
    viewer: Option<Player>,
    roles: Vec<Role>,
    life: Vec<i32>,
    maxlife: Vec<i32>,
    hands: Vec<Vec<Card>>,
    rifle: Vec<bool>,
    deck: Vec<Card>,
    discard: Vec<Card>,
    cur: usize,
    shot_used: bool,
    round: u32,
    phase: Phase,
    sus: Vec<i32>,
    rng: Rng,
    log: Vec<String>,
    winner: Option<usize>, // 0 șerif+adjunct, 1 haiduci, 2 trădător
}

impl Default for Valea {
    fn default() -> Self {
        Self::new()
    }
}

impl Valea {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut roles = vec![
            Role::Sheriff,
            Role::Deputy,
            Role::Outlaw,
            Role::Outlaw,
            Role::Renegade,
        ];
        shuffle_vec(&mut roles, &mut rng);
        let mut deck = deck_new();
        shuffle_vec(&mut deck, &mut rng);
        let maxlife: Vec<i32> = roles
            .iter()
            .map(|&r| if r == Role::Sheriff { 5 } else { 4 })
            .collect();
        let sheriff = roles.iter().position(|&r| r == Role::Sheriff).unwrap();
        let mut g = Valea {
            viewer: None,
            life: maxlife.clone(),
            maxlife,
            hands: vec![Vec::new(); N],
            rifle: vec![false; N],
            roles,
            deck,
            discard: Vec::new(),
            cur: sheriff,
            shot_used: false,
            round: 1,
            phase: Phase::Robot,
            sus: vec![0; N],
            rng,
            log: Vec::new(),
            winner: None,
        };
        for i in 0..N {
            for _ in 0..g.maxlife[i] {
                let c = g.draw();
                g.hands[i].push(c);
            }
        }
        let r = g.roles[0];
        g.say(format!("Rolul tău: {} (locul 1).", r.name()));
        g.say(format!("Șeriful e locul {}.", sheriff + 1));
        g.begin_turn(sheriff);
        g
    }

    pub fn roles(&self) -> &[Role] {
        &self.roles
    }
    pub fn phase(&self) -> Phase {
        self.phase
    }
    pub fn human_hand(&self) -> &[Card] {
        &self.hands[0]
    }
    /// Toate cărțile existente (pachet + aruncate + mâini); trebuie să rămână constant.
    pub fn total_cards(&self) -> usize {
        self.deck.len()
            + self.discard.len()
            + self.hands.iter().map(|h| h.len()).sum::<usize>()
            + self.rifle.iter().filter(|&&r| r).count()
    }

    fn say(&mut self, s: String) {
        self.log.push(s);
        trim_log(&mut self.log, 14);
    }

    fn alive(&self, i: usize) -> bool {
        self.life[i] > 0
    }

    fn draw(&mut self) -> Card {
        if self.deck.is_empty() {
            let mut d = std::mem::take(&mut self.discard);
            shuffle_vec(&mut d, &mut self.rng);
            self.deck = d;
        }
        // în cazul extrem (nimic de tras) întoarce o Ocolire „de rezervă” — nu apare în practică
        self.deck.pop().unwrap_or(Card::Dodge)
    }

    fn dist(&self, a: usize, b: usize) -> usize {
        let live: Vec<usize> = (0..N).filter(|&i| self.alive(i)).collect();
        let ia = live.iter().position(|&x| x == a).unwrap();
        let ib = live.iter().position(|&x| x == b).unwrap();
        let d = ia.abs_diff(ib);
        d.min(live.len() - d)
    }

    fn range(&self, i: usize) -> usize {
        1 + usize::from(self.rifle[i])
    }

    fn can_shoot(&self, a: usize, b: usize) -> bool {
        a != b && self.alive(b) && self.dist(a, b) <= self.range(a)
    }

    fn check_win(&mut self) -> bool {
        let sheriff_alive = (0..N).any(|i| self.roles[i] == Role::Sheriff && self.alive(i));
        let others = (0..N).filter(|&i| self.alive(i)).count();
        let bad =
            (0..N).any(|i| matches!(self.roles[i], Role::Outlaw | Role::Renegade) && self.alive(i));
        let w = if !sheriff_alive {
            let ren_alone =
                others == 1 && (0..N).any(|i| self.roles[i] == Role::Renegade && self.alive(i));
            if ren_alone {
                2
            } else {
                1
            }
        } else if !bad {
            0
        } else {
            return false;
        };
        self.winner = Some(w);
        self.phase = Phase::Over;
        true
    }

    fn team_of(r: Role) -> usize {
        match r {
            Role::Sheriff | Role::Deputy => 0,
            Role::Outlaw => 1,
            Role::Renegade => 2,
        }
    }

    fn damage(&mut self, tgt: usize, src: Option<usize>) {
        self.life[tgt] -= 1;
        if self.life[tgt] > 0 {
            return;
        }
        // moare
        let name = self.roles[tgt].name();
        self.say(format!("Locul {} cade — era {}.", tgt + 1, name));
        let hand = std::mem::take(&mut self.hands[tgt]);
        self.discard.extend(hand);
        if self.rifle[tgt] {
            self.rifle[tgt] = false;
            self.discard.push(Card::Rifle);
        }
        if let Some(s) = src {
            if self.alive(s) {
                if self.roles[tgt] == Role::Outlaw {
                    for _ in 0..3 {
                        let c = self.draw();
                        self.hands[s].push(c);
                    }
                } else if self.roles[tgt] == Role::Deputy && self.roles[s] == Role::Sheriff {
                    let h = std::mem::take(&mut self.hands[s]);
                    self.discard.extend(h);
                    if self.rifle[s] {
                        self.rifle[s] = false;
                        self.discard.push(Card::Rifle);
                    }
                }
            }
        }
        self.check_win();
    }

    fn take_hit(&mut self, tgt: usize, src: usize) {
        if self.roles[tgt] != Role::Sheriff && self.roles[src] != Role::Sheriff {
            // publicul vede doar cine trage în Șerif
        }
        if self.roles[tgt] == Role::Sheriff {
            self.sus[src] += 3;
        }
        self.damage(tgt, Some(src));
    }

    fn auto_dodge(&mut self, tgt: usize) -> bool {
        if let Some(p) = self.hands[tgt].iter().position(|&c| c == Card::Dodge) {
            self.hands[tgt].remove(p);
            self.discard.push(Card::Dodge);
            true
        } else {
            false
        }
    }

    fn begin_turn(&mut self, p: usize) {
        self.cur = p;
        self.shot_used = false;
        if self.roles[p] == Role::Sheriff {
            self.round += 1;
        }
        if self.round >= 30 {
            self.say(format!("Seceta lovește locul {}.", p + 1));
            self.damage(p, None);
            if self.winner.is_some() || !self.alive(p) {
                if self.winner.is_none() {
                    self.next_turn();
                }
                return;
            }
        }
        for _ in 0..2 {
            let c = self.draw();
            self.hands[p].push(c);
        }
        self.phase = if p == 0 { Phase::Play } else { Phase::Robot };
    }

    fn next_turn(&mut self) {
        if self.winner.is_some() {
            return;
        }
        let mut p = self.cur;
        loop {
            p = (p + 1) % N;
            if self.alive(p) {
                break;
            }
        }
        self.begin_turn(p);
    }

    fn end_turn_discard_robot(&mut self, p: usize) {
        while self.hands[p].len() as i32 > self.life[p] {
            // aruncă cea mai puțin utilă
            let order = [
                Card::Panic,
                Card::Rifle,
                Card::Saloon,
                Card::Gatling,
                Card::Heal,
                Card::Shot,
                Card::Dodge,
            ];
            for c in order {
                if let Some(k) = self.hands[p].iter().position(|&x| x == c) {
                    self.hands[p].remove(k);
                    self.discard.push(c);
                    break;
                }
            }
        }
    }

    fn enemy_score(&self, me: usize, j: usize) -> i32 {
        let sj = self.roles[j];
        match self.roles[me] {
            Role::Sheriff => self.sus[j],
            Role::Deputy => {
                if sj == Role::Sheriff {
                    -100
                } else {
                    self.sus[j]
                }
            }
            Role::Outlaw => {
                if sj == Role::Sheriff {
                    10
                } else {
                    // nu știe aliații: mică înclinație către cei care n-au tras în Șerif
                    -self.sus[j] + 1
                }
            }
            Role::Renegade => {
                if sj == Role::Sheriff {
                    if (0..N).filter(|&i| self.alive(i)).count() <= 2 {
                        10
                    } else {
                        -3
                    }
                } else {
                    4 - self.life[j]
                }
            }
        }
    }

    fn robot_action(&mut self) {
        let p = self.cur;
        // 1. leac
        if self.life[p] < self.maxlife[p] {
            if let Some(k) = self.hands[p].iter().position(|&c| c == Card::Heal) {
                self.hands[p].remove(k);
                self.discard.push(Card::Heal);
                self.life[p] += 1;
                self.say(format!("Locul {} bea un leac.", p + 1));
                return;
            }
        }
        // 2. pușcă
        if !self.rifle[p] {
            if let Some(k) = self.hands[p].iter().position(|&c| c == Card::Rifle) {
                self.hands[p].remove(k);
                self.rifle[p] = true;
                self.say(format!("Locul {} își ia o pușcă lungă.", p + 1));
                return;
            }
        }
        // 3. salon
        let hurt = (0..N)
            .filter(|&i| self.alive(i) && self.life[i] < self.maxlife[i])
            .count();
        if hurt >= 2 {
            if let Some(k) = self.hands[p].iter().position(|&c| c == Card::Saloon) {
                self.hands[p].remove(k);
                self.discard.push(Card::Saloon);
                for i in 0..N {
                    if self.alive(i) && self.life[i] < self.maxlife[i] {
                        self.life[i] += 1;
                    }
                }
                self.say(format!(
                    "Locul {} deschide salonul: +1 viață pentru toți.",
                    p + 1
                ));
                return;
            }
        }
        // 4. panică
        if let Some(k) = self.hands[p].iter().position(|&c| c == Card::Panic) {
            let cand: Vec<usize> = (0..N)
                .filter(|&j| {
                    j != p && self.alive(j) && self.dist(p, j) <= 1 && !self.hands[j].is_empty()
                })
                .collect();
            if !cand.is_empty() {
                self.hands[p].remove(k);
                self.discard.push(Card::Panic);
                let w: Vec<u32> = cand
                    .iter()
                    .map(|&j| (self.enemy_score(p, j) + 10).max(1) as u32)
                    .collect();
                let t = cand[pick_weighted(&mut self.rng, &w)];
                let idx = self.rng.below(self.hands[t].len());
                let c = self.hands[t].remove(idx);
                self.hands[p].push(c);
                self.say(format!("Locul {} ia o carte de la locul {}.", p + 1, t + 1));
                return;
            }
        }
        // 5. mitralieră: dacă mai mulți dușmani decât aliați
        if let Some(k) = self.hands[p].iter().position(|&c| c == Card::Gatling) {
            let pos: i32 = (0..N)
                .filter(|&j| j != p && self.alive(j))
                .map(|j| self.enemy_score(p, j).signum())
                .sum();
            if pos > 0 {
                self.hands[p].remove(k);
                self.discard.push(Card::Gatling);
                self.say(format!("Locul {} deschide focul cu mitraliera!", p + 1));
                self.gatling(p);
                return;
            }
        }
        // 6. foc
        if !self.shot_used {
            if let Some(k) = self.hands[p].iter().position(|&c| c == Card::Shot) {
                let cand: Vec<usize> = (0..N).filter(|&j| self.can_shoot(p, j)).collect();
                if !cand.is_empty() {
                    let best = cand.iter().map(|&j| self.enemy_score(p, j)).max().unwrap();
                    if best > 0 || self.rng.below(3) == 0 {
                        let w: Vec<u32> = cand
                            .iter()
                            .map(|&j| (self.enemy_score(p, j) - best + 8).max(1) as u32)
                            .collect();
                        let t = cand[pick_weighted(&mut self.rng, &w)];
                        self.hands[p].remove(k);
                        self.discard.push(Card::Shot);
                        self.shot_used = true;
                        self.say(format!("Locul {} trage în locul {}.", p + 1, t + 1));
                        if t == 0 {
                            self.phase = Phase::React(p, false);
                        } else if self.auto_dodge(t) {
                            self.say(format!("Locul {} ocolește.", t + 1));
                            if self.roles[t] == Role::Sheriff {
                                self.sus[p] += 3;
                            }
                        } else {
                            self.take_hit(t, p);
                        }
                        return;
                    }
                }
            }
        }
        // altfel: sfârșitul turei
        self.end_turn_discard_robot(p);
        if self.winner.is_none() {
            self.next_turn();
        }
    }

    /// Mitraliera: roboții se apără automat; omul e întrebat la sfârșit.
    fn gatling(&mut self, p: usize) {
        for t in 1..N {
            if t == p || !self.alive(t) {
                continue;
            }
            if self.auto_dodge(t) {
                if self.roles[t] == Role::Sheriff {
                    self.sus[p] += 3;
                }
                continue;
            }
            self.take_hit(t, p);
            if self.winner.is_some() {
                return;
            }
        }
        if p != 0 && self.alive(0) {
            self.phase = Phase::React(p, true);
        }
    }

    fn react(&mut self, dodge: bool) {
        let (src, gat) = match self.phase {
            Phase::React(s, g) => (s, g),
            _ => return,
        };
        if dodge {
            let k = self.hands[0]
                .iter()
                .position(|&c| c == Card::Dodge)
                .unwrap();
            self.hands[0].remove(k);
            self.discard.push(Card::Dodge);
            self.say("Ocolești!".into());
            if self.roles[0] == Role::Sheriff {
                self.sus[src] += 3;
            }
        } else {
            self.say("Primești lovitura.".into());
            self.take_hit(0, src);
        }
        let _ = gat;
        if self.winner.is_none() {
            // revine la tura celui care a tras (sau o încheie dacă a murit)
            let _ = src;
            self.phase = Phase::Robot;
            if !self.alive(self.cur) {
                self.next_turn();
            }
        }
    }
}

impl Valea {
    /// Normal rendering (human = J1's point of view).
    fn render_main(&self) -> String {
        let mut s = format!(
            "VALEA USCATĂ — runda {} — la rând: locul {}\n",
            self.round,
            self.cur + 1
        );
        s.push_str(&format!("Rolul tău: {}\n", self.roles[0].name()));
        for i in 0..N {
            let role = if self.roles[i] == Role::Sheriff || !self.alive(i) || i == 0 {
                format!(" [{}]", self.roles[i].name())
            } else {
                String::new()
            };
            s.push_str(&format!(
                "  locul {}{}: {} viață, {} cărți{}{}\n",
                i + 1,
                if i == 0 { " (tu)" } else { "" },
                self.life[i].max(0),
                if i == 0 {
                    self.hands[0].len()
                } else {
                    self.hands[i].len()
                },
                if self.rifle[i] { ", pușcă" } else { "" },
                role
            ));
        }
        let h: Vec<&str> = self.hands[0].iter().map(|c| c.name()).collect();
        s.push_str(&format!("Mâna ta: {}\n", h.join(", ")));
        if let Phase::React(src, g) = self.phase {
            s.push_str(&format!(
                "Locul {} {} asupra ta: ocolire sau accept.\n",
                src + 1,
                if g { "trage cu mitraliera" } else { "trage" }
            ));
        }
        for l in &self.log {
            s.push_str(l);
            s.push('\n');
        }
        if let Some(w) = self.winner {
            s.push_str(match w {
                0 => "Legea a câștigat (Șerif + Adjunct).\n",
                1 => "Haiducii au câștigat.\n",
                _ => "Trădătorul a câștigat.\n",
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
            let h: Vec<&str> = self.hands[i].iter().map(|c| c.name()).collect();
            s.push_str(&format!(
                "  locul {} [{}]: {}\n",
                i + 1,
                self.roles[i].name(),
                h.join(", ")
            ));
        }
        s
    }
}

impl Game for Valea {
    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }
    type Move = Move;

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Play | Phase::Discard | Phase::React(..) | Phase::Over => Player::One,
            Phase::Robot => Player::Two,
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        match self.phase {
            Phase::Play => {
                let mut v = vec![Move::End];
                let h = &self.hands[0];
                for c in CARDS {
                    if !h.contains(&c) {
                        continue;
                    }
                    match c {
                        Card::Shot => {
                            if !self.shot_used {
                                for j in 1..N {
                                    if self.can_shoot(0, j) {
                                        v.push(Move::Target(c, j as u8));
                                    }
                                }
                            }
                        }
                        Card::Panic => {
                            for j in 1..N {
                                if self.alive(j)
                                    && self.dist(0, j) <= 1
                                    && !self.hands[j].is_empty()
                                {
                                    v.push(Move::Target(c, j as u8));
                                }
                            }
                        }
                        Card::Heal => {
                            if self.life[0] < self.maxlife[0] {
                                v.push(Move::Play(c));
                            }
                        }
                        Card::Rifle => {
                            if !self.rifle[0] {
                                v.push(Move::Play(c));
                            }
                        }
                        Card::Dodge => {}
                        _ => v.push(Move::Play(c)),
                    }
                }
                v
            }
            Phase::Discard => {
                let mut v = Vec::new();
                for c in CARDS {
                    if self.hands[0].contains(&c) {
                        v.push(Move::Throw(c));
                    }
                }
                v
            }
            Phase::React(..) => {
                let mut v = vec![Move::Accept];
                if self.hands[0].contains(&Card::Dodge) {
                    v.push(Move::Dodge);
                }
                v
            }
            Phase::Robot => vec![Move::Table],
            Phase::Over => Vec::new(),
        }
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Play(c) => {
                let k = self.hands[0].iter().position(|x| x == c).unwrap();
                self.hands[0].remove(k);
                match c {
                    Card::Heal => {
                        self.life[0] += 1;
                        self.discard.push(*c);
                    }
                    Card::Rifle => self.rifle[0] = true,
                    Card::Saloon => {
                        for i in 0..N {
                            if self.alive(i) && self.life[i] < self.maxlife[i] {
                                self.life[i] += 1;
                            }
                        }
                        self.discard.push(*c);
                        self.say("Salonul: +1 viață pentru toți.".into());
                    }
                    Card::Gatling => {
                        self.discard.push(*c);
                        self.say("Tragi cu mitraliera!".into());
                        for t in 1..N {
                            if !self.alive(t) {
                                continue;
                            }
                            if self.auto_dodge(t) {
                                if self.roles[t] == Role::Sheriff {
                                    self.sus[0] += 3;
                                }
                                continue;
                            }
                            self.take_hit(t, 0);
                            if self.winner.is_some() {
                                return;
                            }
                        }
                    }
                    _ => self.discard.push(*c),
                }
            }
            Move::Target(c, t) => {
                let t = *t as usize;
                let k = self.hands[0].iter().position(|x| x == c).unwrap();
                self.hands[0].remove(k);
                self.discard.push(*c);
                if *c == Card::Shot {
                    self.shot_used = true;
                    self.say(format!("Tragi în locul {}.", t + 1));
                    if self.auto_dodge(t) {
                        self.say(format!("Locul {} ocolește.", t + 1));
                        if self.roles[t] == Role::Sheriff {
                            self.sus[0] += 3;
                        }
                    } else {
                        self.take_hit(t, 0);
                    }
                } else {
                    let idx = self.rng.below(self.hands[t].len());
                    let card = self.hands[t].remove(idx);
                    self.hands[0].push(card);
                    self.say(format!(
                        "Iei o carte de la locul {}: {}.",
                        t + 1,
                        card.name()
                    ));
                }
            }
            Move::End => {
                if self.hands[0].len() as i32 > self.life[0] {
                    self.phase = Phase::Discard;
                } else {
                    self.next_turn();
                }
            }
            Move::Throw(c) => {
                let k = self.hands[0].iter().position(|x| x == c).unwrap();
                self.hands[0].remove(k);
                self.discard.push(*c);
                if self.hands[0].len() as i32 <= self.life[0] {
                    self.next_turn();
                }
            }
            Move::Dodge => self.react(true),
            Move::Accept => self.react(false),
            Move::Table => {
                if self.alive(self.cur) {
                    self.robot_action();
                } else {
                    self.next_turn();
                }
            }
        }
        // omul mort: jocul curge pe masă
        if self.winner.is_none() && !self.alive(0) && self.phase != Phase::Robot {
            self.phase = Phase::Robot;
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        let w = self.winner?;
        let mine = Self::team_of(self.roles[0]);
        Some(Outcome::Win(if w == mine {
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
            Move::Play(c) => c.name().to_string(),
            Move::Target(c, t) => format!("{} {}", c.name(), *t + 1),
            Move::End => "incheie".into(),
            Move::Throw(c) => format!("scarta {}", c.name()),
            Move::Dodge => "ocolire".into(),
            Move::Accept => "accept".into(),
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

    fn run(seed: u64) -> Valea {
        let mut g = Valea::with_seed(seed);
        let total = g.total_cards();
        let mut rng = Rng::new(seed ^ 9);
        let mut n = 0;
        while g.outcome().is_none() {
            let mv = g.legal_moves();
            assert!(!mv.is_empty(), "{}", g.render());
            g.apply(&mv[rng.below(mv.len())]);
            assert_eq!(g.total_cards(), total, "{}", g.render());
            n += 1;
            assert!(n < 6000);
        }
        g
    }

    #[test]
    fn roles_and_deck() {
        let g = Valea::with_seed(3);
        assert_eq!(g.roles().iter().filter(|&&r| r == Role::Sheriff).count(), 1);
        assert_eq!(g.roles().iter().filter(|&&r| r == Role::Outlaw).count(), 2);
        assert_eq!(deck_new().len(), 56);
    }

    #[test]
    fn random_games_end_conserve_cards_and_vary() {
        let mut w = [0; 3];
        for seed in 1..200u64 {
            let g = run(seed);
            w[g.winner.unwrap()] += 1;
        }
        assert!(w[0] > 5 && w[1] > 5, "{w:?}");
    }

    #[test]
    fn range_and_rifle() {
        let mut g = Valea::with_seed(1);
        for i in 0..N {
            g.life[i] = 4;
        }
        assert!(g.can_shoot(0, 1) && g.can_shoot(0, 4));
        assert!(!g.can_shoot(0, 2));
        g.rifle[0] = true;
        assert!(g.can_shoot(0, 2) && g.can_shoot(0, 3));
    }

    #[test]
    fn others_roles_hidden_unless_sheriff_or_dead() {
        for seed in 1..40u64 {
            let g = Valea::with_seed(seed);
            for l in g
                .render()
                .lines()
                .filter(|l| l.contains("locul") && l.contains("viață"))
            {
                let me = l.contains("(tu)");
                let has_role = l.contains('[');
                let sheriff = l.contains("[Șerif]");
                assert!(me || !has_role || sheriff, "{l}");
            }
        }
    }
}
