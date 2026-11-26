//! IMPERII ÎN APUS — cucerire cu popoare care ajung în declin (variantă originală).
//!
//! Hartă fixă de 12 ținuturi (4×3), trei jucători: tu (locul 1) și doi roboți (masa,
//! `Player::Two`). Opt ture pentru fiecare. La alegere stau șase combinații **popor + dar**,
//! în rând: prima e gratis, următoarele costă 1, 2, … monede (monedele se pun pe cele sărite
//! și le ia cine alege combinația respectivă). Popor + dar îți dau jetoane și avantaje.
//! Cucerire: un ținut costă 2 jetoane + 1 dacă e **Munte** + câte jetoane are ocupantul
//! (un popor în declin are un singur jetonul pe ținut); trebuie să fie vecin cu ceva al tău
//! (prima cucerire: orice ținut de pe margine; „Zburătorii” oriunde). Cine nu are destule
//! jetoane încearcă **ultima cucerire** cu zarul de întărire (0, 0, 0, 1, 2, 3). La final
//! de cucerire jetoanele rămase se așază automat pe ținuturile tale de la margine.
//! Fiecare tură câștigi 1 monedă pe ținut, plus bonusurile. Un popor care nu mai poate
//! cuceri intră în **declin** (se întoarce pe față „vlăguită”) și alegi alt popor data viitoare.
//! Simplificări: fără ținuturi cu comori, fără ordine specială de redistribuire, hartă fixă.

use crate::w5_util::{pick_weighted, trim_log};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const R: usize = 12;
const TURNS: u32 = 8;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Terrain {
    Plain,
    Hill,
    Forest,
    Swamp,
    Mountain,
}

const MAP: [Terrain; R] = [
    Terrain::Mountain,
    Terrain::Plain,
    Terrain::Forest,
    Terrain::Hill,
    Terrain::Plain,
    Terrain::Swamp,
    Terrain::Hill,
    Terrain::Plain,
    Terrain::Forest,
    Terrain::Plain,
    Terrain::Mountain,
    Terrain::Swamp,
];

fn t_name(t: Terrain) -> &'static str {
    match t {
        Terrain::Plain => "câmpie",
        Terrain::Hill => "deal",
        Terrain::Forest => "pădure",
        Terrain::Swamp => "mlaștină",
        Terrain::Mountain => "munte",
    }
}

fn neighbors(r: usize) -> Vec<usize> {
    let (x, y) = (r % 4, r / 4);
    let mut v = Vec::new();
    if x > 0 {
        v.push(r - 1);
    }
    if x < 3 {
        v.push(r + 1);
    }
    if y > 0 {
        v.push(r - 4);
    }
    if y < 2 {
        v.push(r + 4);
    }
    v
}

fn is_edge(r: usize) -> bool {
    let (x, y) = (r % 4, r / 4);
    x == 0 || x == 3 || y == 0 || y == 2
}

pub const RACES: [(&str, u32); 6] = [
    ("Lupii de Turbă", 5),
    ("Pitici de Peșteră", 3),
    ("Zâne", 6),
    ("Sirenele de Baltă", 6),
    ("Titani", 2),
    ("Hoinari", 7),
];
pub const POWERS: [(&str, u32); 6] = [
    ("Zburători", 5),
    ("Viteji", 5),
    ("Bogați", 7),
    ("Stâncoși", 4),
    ("Mohorâți", 5),
    ("Pădureți", 4),
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Combo {
    race: u8,
    power: u8,
    coins: u8,
}

#[derive(Clone, Default)]
struct Empire {
    race: Option<(u8, u8)>,
    tokens_hand: u32,
    declined: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Phase {
    Pick,
    Conquer,
    RobotTurn,
    Over,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Pick(u8),
    Decline,
    Conquer(u8),
    LastTry(u8),
    Done,
    Table,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Slot {
    owner: u8, // 0..2, 255 liber
    active: bool,
    tokens: u32,
}

#[derive(Clone)]
pub struct Apus {
    slots: [Slot; R],
    combos: Vec<Combo>,
    coins: [u32; 3],
    active: [Empire; 3],
    declined_regions: [Vec<usize>; 3],
    cur: usize,
    turn: u32,
    phase: Phase,
    conquered_this: u32,
    kills_this: u32,
    rng: Rng,
    log: Vec<String>,
}

impl Default for Apus {
    fn default() -> Self {
        Self::new()
    }
}

impl Apus {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut combos = Vec::new();
        for _ in 0..6 {
            combos.push(Combo {
                race: rng.below(6) as u8,
                power: rng.below(6) as u8,
                coins: 0,
            });
        }
        let mut g = Apus {
            slots: [Slot {
                owner: 255,
                active: true,
                tokens: 0,
            }; R],
            combos,
            coins: [5; 3],
            active: [Empire::default(), Empire::default(), Empire::default()],
            declined_regions: [vec![], vec![], vec![]],
            cur: 0,
            turn: 1,
            phase: Phase::Pick,
            conquered_this: 0,
            kills_this: 0,
            rng,
            log: vec![],
        };
        g.say("Alege o combinație popor + dar.".into());
        g
    }

    pub fn coins(&self) -> [u32; 3] {
        self.coins
    }
    pub fn phase(&self) -> Phase {
        self.phase
    }
    pub fn turn(&self) -> u32 {
        self.turn
    }

    fn say(&mut self, s: String) {
        self.log.push(s);
        trim_log(&mut self.log, 14);
    }

    fn combo_tokens(c: &Combo) -> u32 {
        RACES[c.race as usize].1 + POWERS[c.power as usize].1
    }

    fn new_combo(&mut self) -> Combo {
        Combo {
            race: self.rng.below(6) as u8,
            power: self.rng.below(6) as u8,
            coins: 0,
        }
    }

    fn has(&self, p: usize, race: Option<u8>, power: Option<u8>) -> bool {
        match self.active[p].race {
            Some((r, w)) if !self.active[p].declined => {
                race.is_some_and(|x| x == r) || power.is_some_and(|x| x == w)
            }
            _ => false,
        }
    }

    fn owned(&self, p: usize) -> Vec<usize> {
        (0..R).filter(|&i| self.slots[i].owner == p as u8).collect()
    }

    fn cost(&self, p: usize, r: usize) -> u32 {
        let s = self.slots[r];
        let mut c: i32 = 2;
        if MAP[r] == Terrain::Mountain && !self.has(p, None, Some(3)) {
            c += 1;
        }
        if s.owner != 255 {
            c += if s.active { s.tokens as i32 } else { 1 };
        }
        if self.has(p, Some(4), None) {
            c -= 1;
        }
        if MAP[r] == Terrain::Plain && self.has(p, None, Some(1)) {
            c -= 1;
        }
        c.max(1) as u32
    }

    fn can_target(&self, p: usize, r: usize) -> bool {
        if self.slots[r].owner == p as u8 {
            return false;
        }
        if self.has(p, None, Some(0)) {
            return true;
        }
        let mine: Vec<usize> = self
            .owned(p)
            .into_iter()
            .filter(|&i| self.slots[i].active)
            .collect();
        if mine.is_empty() {
            return is_edge(r);
        }
        neighbors(r).iter().any(|n| mine.contains(n))
    }

    fn conquer(&mut self, p: usize, r: usize, used: u32) {
        let prev = self.slots[r];
        if prev.owner != 255 && prev.owner as usize != p {
            let o = prev.owner as usize;
            if prev.active {
                // jetoanele ocupantului active (în afară de unul) se întorc la el
                self.active[o].tokens_hand += prev.tokens.saturating_sub(1);
            }
            self.declined_regions[o].retain(|&x| x != r);
            self.kills_this += 1;
        }
        self.slots[r] = Slot {
            owner: p as u8,
            active: true,
            tokens: used,
        };
        self.active[p].tokens_hand -= used.min(self.active[p].tokens_hand);
        self.conquered_this += 1;
    }

    fn finish_conquest(&mut self, p: usize) {
        // jetoanele rămase se așază pe ținuturile active de la margine
        let rest = self.active[p].tokens_hand;
        if rest > 0 {
            let mine: Vec<usize> = self
                .owned(p)
                .into_iter()
                .filter(|&i| self.slots[i].active)
                .collect();
            if !mine.is_empty() {
                let border: Vec<usize> = mine
                    .iter()
                    .copied()
                    .filter(|&i| neighbors(i).iter().any(|&n| self.slots[n].owner != p as u8))
                    .collect();
                let tgt = if border.is_empty() { mine } else { border };
                for k in 0..rest as usize {
                    let r = tgt[k % tgt.len()];
                    self.slots[r].tokens += 1;
                }
                self.active[p].tokens_hand = 0;
            }
        }
        // scor
        let mut gain = 0u32;
        let mine = self.owned(p);
        gain += mine.len() as u32;
        let cnt = |t: Terrain| {
            mine.iter()
                .filter(|&&i| MAP[i] == t && self.slots[i].active)
                .count() as u32
        };
        if self.has(p, Some(0), None) {
            gain += self.kills_this;
        }
        if self.has(p, Some(1), None) {
            gain += cnt(Terrain::Mountain);
        }
        if self.has(p, Some(2), None) {
            gain += cnt(Terrain::Forest);
        }
        if self.has(p, Some(3), None) {
            gain += cnt(Terrain::Swamp);
        }
        if self.has(p, None, Some(2)) {
            gain += 1;
        }
        if self.has(p, None, Some(4)) {
            gain += cnt(Terrain::Hill);
        }
        if self.has(p, None, Some(5)) {
            gain += cnt(Terrain::Forest);
        }
        self.coins[p] += gain;
        self.say(format!(
            "Locul {} încasează {} monede (total {}).",
            p + 1,
            gain,
            self.coins[p]
        ));
        self.next_player();
    }

    fn next_player(&mut self) {
        self.conquered_this = 0;
        self.kills_this = 0;
        self.cur = (self.cur + 1) % 3;
        if self.cur == 0 {
            self.turn += 1;
            if self.turn > TURNS {
                self.phase = Phase::Over;
                return;
            }
        }
        self.phase = if self.cur == 0 {
            Phase::Pick
        } else {
            Phase::RobotTurn
        };
    }

    fn do_decline(&mut self, p: usize) {
        for i in 0..R {
            if self.slots[i].owner == p as u8 && self.slots[i].active {
                self.slots[i].active = false;
                self.slots[i].tokens = 1;
            }
        }
        // un singur popor în declin: cel vechi dispare
        let old: Vec<usize> = (0..R)
            .filter(|&i| self.slots[i].owner == p as u8 && !self.slots[i].active)
            .collect();
        self.declined_regions[p] = old;
        self.active[p].declined = true;
        self.active[p].tokens_hand = 0;
    }

    fn pick_combo(&mut self, p: usize, idx: usize) {
        // un popor în declin anterior dispare când iei unul nou
        for i in 0..R {
            if self.slots[i].owner == p as u8 && !self.slots[i].active {
                self.slots[i].owner = 255;
                self.slots[i].tokens = 0;
            }
        }
        // plătești 1 monedă pe combinația sărită
        for k in 0..idx {
            self.combos[k].coins += 1;
        }
        self.coins[p] -= idx as u32;
        let c = self.combos.remove(idx);
        self.coins[p] += c.coins as u32;
        let nc = self.new_combo();
        self.combos.push(nc);
        self.active[p] = Empire {
            race: Some((c.race, c.power)),
            tokens_hand: Self::combo_tokens(&c),
            declined: false,
        };
        self.declined_regions[p].clear();
        self.say(format!(
            "Locul {} alege {} {} ({} jetoane).",
            p + 1,
            RACES[c.race as usize].0,
            POWERS[c.power as usize].0,
            Self::combo_tokens(&c)
        ));
        self.conquered_this = 0;
        self.kills_this = 0;
    }

    fn targets(&self, p: usize) -> Vec<usize> {
        (0..R).filter(|&r| self.can_target(p, r)).collect()
    }

    fn robot_turn(&mut self) {
        let p = self.cur;
        let tokens_left_total = self.active[p].tokens_hand;
        let active_regions = self
            .owned(p)
            .into_iter()
            .filter(|&i| self.slots[i].active)
            .count();
        // intră în declin dacă are multe ținuturi și puține șanse
        let can_cont = self.active[p].race.is_some() && !self.active[p].declined;
        if can_cont && active_regions >= 5 && self.turn >= 3 && self.rng.below(3) > 0 {
            self.do_decline(p);
            self.say(format!("Locul {} intră în declin.", p + 1));
            self.finish_conquest(p);
            return;
        }
        let _ = tokens_left_total;
        if !can_cont {
            // alege cea mai bună combinație la prețul permis
            let coins = self.coins[p] as usize;
            let w: Vec<u32> = (0..6)
                .map(|k| {
                    if k > coins {
                        0
                    } else {
                        (Self::combo_tokens(&self.combos[k]) + self.combos[k].coins as u32 * 2)
                            .saturating_sub(k as u32)
                            + 1
                    }
                })
                .collect();
            let k = pick_weighted(&mut self.rng, &w);
            self.pick_combo(p, k);
        }
        // cucerește greedy: ținutul cel mai ieftin / valoros
        loop {
            let mut options: Vec<(i32, usize)> = Vec::new();
            for r in self.targets(p) {
                let c = self.cost(p, r);
                if c <= self.active[p].tokens_hand {
                    // evită să atace jucătorul cel mai puternic fără rost
                    let value = 10 - c as i32 + i32::from(self.slots[r].owner != 255);
                    options.push((value, r));
                }
            }
            if options.is_empty() {
                break;
            }
            options.sort();
            let r = options.last().unwrap().1;
            let c = self.cost(p, r);
            self.conquer(p, r, c);
        }
        // ultima încercare cu zarul
        let rest = self.active[p].tokens_hand;
        if rest > 0 {
            let mut options: Vec<(i32, usize)> = self
                .targets(p)
                .into_iter()
                .filter(|&r| self.cost(p, r) <= rest + 3)
                .map(|r| (-(self.cost(p, r) as i32), r))
                .collect();
            options.sort();
            if let Some(&(_, r)) = options.last() {
                let die = [0u32, 0, 0, 1, 2, 3][self.rng.below(6)];
                let c = self.cost(p, r);
                if rest + die >= c {
                    self.conquer(p, r, rest);
                    self.say(format!(
                        "Locul {} cucerește ținutul {} cu zarul ({die}).",
                        p + 1,
                        r + 1
                    ));
                }
            }
        }
        self.finish_conquest(p);
    }

    fn standings(&self) -> [u32; 3] {
        self.coins
    }
}

impl Game for Apus {
    type Move = Move;

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Pick | Phase::Conquer | Phase::Over => Player::One,
            Phase::RobotTurn => Player::Two,
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        match self.phase {
            Phase::Pick => {
                let mut v = Vec::new();
                let can_cont = self.active[0].race.is_some() && !self.active[0].declined;
                if can_cont {
                    v.push(Move::Done); // „continuă” cu poporul activ
                    v.push(Move::Decline);
                } else {
                    for k in 0..6 {
                        if self.coins[0] as usize >= k {
                            v.push(Move::Pick(k as u8));
                        }
                    }
                }
                v
            }
            Phase::Conquer => {
                let mut v = vec![Move::Done];
                let hand = self.active[0].tokens_hand;
                for r in self.targets(0) {
                    if self.cost(0, r) <= hand {
                        v.push(Move::Conquer(r as u8));
                    } else if hand > 0 && self.cost(0, r) <= hand + 3 {
                        v.push(Move::LastTry(r as u8));
                    }
                }
                v
            }
            Phase::RobotTurn => vec![Move::Table],
            Phase::Over => vec![],
        }
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Pick(k) => {
                self.pick_combo(0, *k as usize);
                self.phase = Phase::Conquer;
            }
            Move::Decline => {
                self.do_decline(0);
                self.say("Intri în declin.".into());
                self.finish_conquest(0);
            }
            Move::Conquer(r) => {
                let c = self.cost(0, *r as usize);
                self.conquer(0, *r as usize, c);
            }
            Move::LastTry(r) => {
                let rest = self.active[0].tokens_hand;
                let die = [0u32, 0, 0, 1, 2, 3][self.rng.below(6)];
                let c = self.cost(0, *r as usize);
                if rest + die >= c {
                    self.conquer(0, *r as usize, rest);
                    self.say(format!(
                        "Zarul de întărire ({die}) îți dă ținutul {}.",
                        *r + 1
                    ));
                } else {
                    self.say(format!("Zarul de întărire ({die}) nu ajunge."));
                    self.active[0].tokens_hand = 0;
                }
            }
            Move::Done => {
                if self.phase == Phase::Pick {
                    self.phase = Phase::Conquer;
                } else {
                    self.finish_conquest(0);
                }
            }
            Move::Table => self.robot_turn(),
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        let c = self.standings();
        let mx = *c.iter().max().unwrap();
        if c[0] == mx {
            if c.iter().filter(|&&x| x == mx).count() > 1 {
                Some(Outcome::Draw)
            } else {
                Some(Outcome::Win(Player::One))
            }
        } else {
            Some(Outcome::Win(Player::Two))
        }
    }

    fn render(&self) -> String {
        let mut s = format!(
            "IMPERII ÎN APUS — tura {}/{} — monede: tu {}, locul 2: {}, locul 3: {}\n",
            self.turn.min(TURNS),
            TURNS,
            self.coins[0],
            self.coins[1],
            self.coins[2]
        );
        s.push_str("Harta (ținut: teren, stăpân, jetoane):\n");
        for y in 0..3 {
            let mut row = String::new();
            for x in 0..4 {
                let r = y * 4 + x;
                let sl = self.slots[r];
                let o = if sl.owner == 255 {
                    "-".to_string()
                } else {
                    format!("{}{}", sl.owner + 1, if sl.active { "" } else { "d" })
                };
                row.push_str(&format!(
                    "[{:>2} {:<8} {:>2}:{}] ",
                    r + 1,
                    t_name(MAP[r]),
                    o,
                    sl.tokens
                ));
            }
            s.push_str(&row);
            s.push('\n');
        }
        if let Some((r, w)) = self.active[0].race {
            s.push_str(&format!(
                "Poporul tău: {} {}{} — jetoane în mână: {}\n",
                RACES[r as usize].0,
                POWERS[w as usize].0,
                if self.active[0].declined {
                    " (în declin)"
                } else {
                    ""
                },
                self.active[0].tokens_hand
            ));
        }
        if self.phase == Phase::Pick && !(self.active[0].race.is_some() && !self.active[0].declined)
        {
            s.push_str("Combinații (poziție 0 = gratis):\n");
            for (k, c) in self.combos.iter().enumerate() {
                s.push_str(&format!(
                    "  {}: {} {} — {} jetoane, {} monede pe ea\n",
                    k,
                    RACES[c.race as usize].0,
                    POWERS[c.power as usize].0,
                    Self::combo_tokens(c),
                    c.coins
                ));
            }
        }
        for l in &self.log {
            s.push_str(l);
            s.push('\n');
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Pick(k) => format!("alege {k}"),
            Move::Decline => "declin".into(),
            Move::Conquer(r) => format!("cucereste {}", *r + 1),
            Move::LastTry(r) => format!("zar {}", *r + 1),
            Move::Done => {
                if self.phase == Phase::Pick {
                    "continua".into()
                } else {
                    "gata".into()
                }
            }
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

    fn run(seed: u64) -> Apus {
        let mut g = Apus::with_seed(seed);
        let mut rng = Rng::new(seed ^ 1);
        let mut n = 0;
        while g.outcome().is_none() {
            let mv = g.legal_moves();
            assert!(!mv.is_empty(), "{}", g.render());
            g.apply(&mv[rng.below(mv.len())]);
            n += 1;
            assert!(n < 2000);
        }
        g
    }

    #[test]
    fn games_end_after_eight_turns() {
        for seed in 1..150u64 {
            let g = run(seed);
            assert!(g.turn() > TURNS);
            assert!(g.coins().iter().all(|&c| c < 10_000));
        }
    }

    #[test]
    fn map_adjacency_is_symmetric() {
        for a in 0..R {
            for &b in &neighbors(a) {
                assert!(neighbors(b).contains(&a));
            }
        }
    }

    #[test]
    fn coins_are_conserved_by_picking() {
        let mut g = Apus::with_seed(2);
        let total: u32 = g.coins[0] + g.combos.iter().map(|c| c.coins as u32).sum::<u32>();
        g.apply(&Move::Pick(3));
        let after: u32 = g.coins[0] + g.combos.iter().map(|c| c.coins as u32).sum::<u32>();
        assert_eq!(total, after);
    }

    #[test]
    fn mountain_costs_more() {
        let mut g = Apus::with_seed(2);
        g.active[0].race = Some((5, 1)); // fără bonus pe munte
        g.active[0].tokens_hand = 10;
        assert_eq!(g.cost(0, 0), 3);
        assert_eq!(g.cost(0, 1), 1);
    }
}
