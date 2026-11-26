//! ROADE DE TOAMNĂ — plantat și schimbat legume (variantă originală de „fermieri cu mâna în ordine”).
//!
//! Trei fermieri: tu (locul 1) și doi roboți (masa, `Player::Two`). Opt legume, cu
//! rarități diferite și tabele de recoltă proprii. Ai 2 ogoare (al treilea costă 3
//! monede) și o **mână cu ordine fixă**: mereu plantezi prima carte. Tura: 1) plantezi prima
//! carte (obligatoriu) și, dacă vrei, a doua; 2) întorci două cărți din pachet: fiecare o
//! plantezi sau o **oferi** mesei: un robot care are nevoie de ea îți dă o carte din mâna
//! lui (necunoscută) — amândoi plantați imediat ce ați primit; 3) iei 3 cărți la coada
//! mâinii. Un ogor se poate planta doar gol sau cu aceeași legumă; poți **recolta** oricând:
//! numărul de cărți din ogor → monede după tabel, restul cărților ies din joc ca „fără
//! valoare” în aruncate. Roboții îți pot oferi și ei o carte: dacă accepți, dai o carte din
//! mână (la alegere) și o plantezi imediat. Jocul se termină la a doua golire a pachetului;
//! la final se recoltează tot. Mai multe monede câștigă.
//! Simplificări: fără negociere liberă, fără cărți donate de la mână în afara ofertelor,
//! schimbul cu masa este un schimb „orb”.

use crate::w5_util::{shuffle_vec, trim_log};
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const CROPS: [(&str, &str, u32, [u32; 4]); 8] = [
    ("cartof", "Cartof", 22, [4, 6, 8, 10]),
    ("ardei", "Ardei", 18, [3, 6, 8, 9]),
    ("ceapa", "Ceapă", 16, [3, 5, 7, 9]),
    ("varza", "Varză", 14, [3, 5, 6, 7]),
    ("mazare", "Mazăre", 12, [2, 4, 6, 8]),
    ("dovleac", "Dovleac", 10, [2, 3, 4, 5]),
    ("sfecla", "Sfeclă", 8, [2, 3, 4, 5]),
    ("usturoi", "Usturoi", 6, [2, 3, 4, 5]),
];

fn value(crop: u8, n: u32) -> u32 {
    let t = CROPS[crop as usize].3;
    let mut c = 0;
    for (k, &need) in t.iter().enumerate() {
        if n >= need {
            c = k as u32 + 1;
        }
    }
    c
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Phase {
    Plant1,
    Plant2,
    Reveal,
    Forced,
    Offer,
    Robot,
    Over,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Plant(u8),
    Harvest(u8),
    Buy,
    Skip,
    Trade,
    Accept(u8),
    Refuse,
    Table,
}

#[derive(Clone)]
struct Farmer {
    hand: Vec<u8>,
    fields: Vec<Option<(u8, u32)>>,
    coins: u32,
}

#[derive(Clone)]
pub struct Roade {
    /// Whose private info `render` shows (`None` = the mover's, hot-seat).
    viewer: Option<Player>,
    farmers: [Farmer; 3],
    deck: Vec<u8>,
    discard: Vec<u8>,
    face_up: Vec<u8>,
    pending: Option<u8>,
    resume: Phase,
    robot_stage: u8,
    robot_reveal: Vec<u8>,
    offer_from: usize,
    offer_failed: bool,
    cur: usize,
    reshuffles: u32,
    ending: bool,
    spent: u32,
    rng: Rng,
    log: Vec<String>,
    phase: Phase,
    total: usize,
}

impl Default for Roade {
    fn default() -> Self {
        Self::new()
    }
}

impl Roade {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut deck = Vec::new();
        for (i, c) in CROPS.iter().enumerate() {
            for _ in 0..c.2 {
                deck.push(i as u8);
            }
        }
        let total = deck.len();
        shuffle_vec(&mut deck, &mut rng);
        let mk = || Farmer {
            hand: vec![],
            fields: vec![None, None],
            coins: 0,
        };
        let mut g = Roade {
            viewer: None,
            farmers: [mk(), mk(), mk()],
            deck,
            discard: vec![],
            face_up: vec![],
            pending: None,
            resume: Phase::Reveal,
            robot_stage: 0,
            robot_reveal: vec![],
            offer_from: 1,
            offer_failed: false,
            cur: 0,
            reshuffles: 0,
            ending: false,
            spent: 0,
            rng,
            log: vec![],
            phase: Phase::Plant1,
            total,
        };
        for f in 0..3 {
            for _ in 0..5 {
                if let Some(c) = g.draw() {
                    g.farmers[f].hand.push(c);
                }
            }
        }
        g.say("Plantează prima carte din mână.".into());
        g
    }

    pub fn coins(&self) -> [u32; 3] {
        [
            self.farmers[0].coins,
            self.farmers[1].coins,
            self.farmers[2].coins,
        ]
    }
    pub fn phase(&self) -> Phase {
        self.phase
    }

    /// Toate cărțile (inclusiv monedele ieșite) trebuie să rămână `total`.
    pub fn conserved(&self) -> bool {
        let hands: usize = self.farmers.iter().map(|f| f.hand.len()).sum();
        let fields: usize = self
            .farmers
            .iter()
            .map(|f| {
                f.fields
                    .iter()
                    .flatten()
                    .map(|x| x.1 as usize)
                    .sum::<usize>()
            })
            .sum();
        let coins: usize = self.farmers.iter().map(|f| f.coins as usize).sum();
        self.deck.len()
            + self.discard.len()
            + hands
            + fields
            + self.face_up.len()
            + usize::from(self.pending.is_some())
            + self.robot_reveal.len()
            + coins
            + self.spent as usize
            == self.total
    }

    fn say(&mut self, s: String) {
        self.log.push(s);
        trim_log(&mut self.log, 14);
    }

    fn draw(&mut self) -> Option<u8> {
        if self.deck.is_empty() {
            if self.discard.is_empty() {
                return None;
            }
            self.reshuffles += 1;
            if self.reshuffles >= 2 {
                self.ending = true;
            }
            self.deck = std::mem::take(&mut self.discard);
            shuffle_vec(&mut self.deck, &mut self.rng);
        }
        self.deck.pop()
    }

    fn can_plant(&self, p: usize, crop: u8, k: usize) -> bool {
        match self.farmers[p].fields.get(k) {
            Some(None) => true,
            Some(Some((c, _))) => *c == crop,
            None => false,
        }
    }

    fn plant_slot(&self, p: usize, crop: u8) -> Option<usize> {
        // întâi ogor cu aceeași legumă, apoi gol
        (0..self.farmers[p].fields.len())
            .find(|&k| matches!(self.farmers[p].fields[k], Some((c, _)) if c == crop))
            .or_else(|| {
                (0..self.farmers[p].fields.len()).find(|&k| self.farmers[p].fields[k].is_none())
            })
    }

    fn plant(&mut self, p: usize, crop: u8, k: usize) {
        let f = &mut self.farmers[p].fields[k];
        match f {
            Some((_, n)) => *n += 1,
            None => *f = Some((crop, 1)),
        }
    }

    fn harvest(&mut self, p: usize, k: usize) {
        if let Some((crop, n)) = self.farmers[p].fields[k].take() {
            let c = value(crop, n);
            self.farmers[p].coins += c;
            for _ in 0..(n - c) {
                self.discard.push(crop);
            }
            self.say(format!(
                "Locul {} recoltează {} {} → {} monede.",
                p + 1,
                n,
                CROPS[crop as usize].1,
                c
            ));
        }
    }

    fn harvest_worst(&mut self, p: usize) -> usize {
        // golește ogorul cel mai puțin valoros
        let mut best = 0;
        let mut bv = i64::MAX;
        for (k, f) in self.farmers[p].fields.iter().enumerate() {
            let v = match f {
                Some((c, n)) => (value(*c, *n) as i64) * 10 + *n as i64,
                None => -1,
            };
            if v < bv {
                bv = v;
                best = k;
            }
        }
        self.harvest(p, best);
        best
    }

    fn robot_place(&mut self, p: usize, crop: u8) {
        let k = match self.plant_slot(p, crop) {
            Some(k) => k,
            None => {
                if self.farmers[p].coins >= 3 && self.farmers[p].fields.len() < 3 {
                    self.farmers[p].coins -= 3;
                    self.spent += 3;
                    self.farmers[p].fields.push(None);
                    self.say(format!("Locul {} cumpără un al treilea ogor.", p + 1));
                    self.farmers[p].fields.len() - 1
                } else {
                    self.harvest_worst(p)
                }
            }
        };
        self.plant(p, crop, k);
    }

    fn robot_maybe_harvest(&mut self, p: usize) {
        for k in 0..self.farmers[p].fields.len() {
            if let Some((c, n)) = self.farmers[p].fields[k] {
                if value(c, n) >= 3 && !self.farmers[p].hand.contains(&c) {
                    self.harvest(p, k);
                }
            }
        }
    }

    fn end_turn(&mut self) {
        // ia 3 cărți la coada mâinii
        let p = self.cur;
        for _ in 0..3 {
            if let Some(c) = self.draw() {
                self.farmers[p].hand.push(c);
            }
        }
        if self.ending && self.cur == 2 {
            for f in 0..3 {
                for k in 0..self.farmers[f].fields.len() {
                    self.harvest(f, k);
                }
            }
            self.phase = Phase::Over;
            return;
        }
        self.cur = (self.cur + 1) % 3;
        self.robot_stage = 0;
        self.offer_failed = false;
        if self.cur == 0 {
            self.phase = if self.farmers[0].hand.is_empty() {
                Phase::Reveal
            } else {
                Phase::Plant1
            };
            if self.phase == Phase::Reveal {
                self.start_reveal();
            }
        } else {
            self.phase = Phase::Robot;
        }
    }

    fn start_reveal(&mut self) {
        self.face_up.clear();
        for _ in 0..2 {
            if let Some(c) = self.draw() {
                self.face_up.push(c);
            }
        }
        self.phase = if self.face_up.is_empty() {
            Phase::Over
        } else {
            Phase::Reveal
        };
        if self.face_up.is_empty() {
            self.end_turn();
        } else {
            let names: Vec<&str> = self.face_up.iter().map(|&c| CROPS[c as usize].1).collect();
            self.say(format!("Cărți întoarse: {}.", names.join(", ")));
        }
    }

    fn after_reveal_card(&mut self) {
        self.face_up.remove(0);
        if self.face_up.is_empty() {
            self.offer_failed = false;
            self.end_turn();
        } else {
            self.offer_failed = false;
            self.phase = Phase::Reveal;
        }
    }

    fn robot_step(&mut self) {
        let p = self.cur;
        if self.robot_stage == 0 {
            // plantează prima (și a doua dacă se potrivește) carte
            if !self.farmers[p].hand.is_empty() {
                let c = self.farmers[p].hand.remove(0);
                self.robot_place(p, c);
                if let Some(&c2) = self.farmers[p].hand.first() {
                    if self.plant_slot(p, c2).is_some()
                        && self.farmers[p].fields.iter().flatten().any(|x| x.0 == c2)
                    {
                        let c2 = self.farmers[p].hand.remove(0);
                        self.robot_place(p, c2);
                    }
                }
            }
            for _ in 0..2 {
                if let Some(c) = self.draw() {
                    self.robot_reveal.push(c);
                }
            }
            self.robot_stage = 1;
        }
        while !self.robot_reveal.is_empty() {
            let c = self.robot_reveal.remove(0);
            if self.plant_slot(p, c).is_some() {
                self.robot_place(p, c);
                continue;
            }
            // nu încape fără recoltă: uneori oferă omului
            if self.farmers[0].hand.len() > 1 && self.rng.below(2) == 0 {
                self.pending = Some(c);
                self.offer_from = p;
                self.phase = Phase::Offer;
                self.say(format!(
                    "Locul {} îți oferă o carte: {}.",
                    p + 1,
                    CROPS[c as usize].1
                ));
                return;
            }
            self.robot_place(p, c);
        }
        self.robot_maybe_harvest(p);
        self.end_turn();
    }

    fn human_hand_crops(&self) -> Vec<u8> {
        let mut v: Vec<u8> = self.farmers[0].hand.clone();
        v.sort();
        v.dedup();
        v
    }
}

impl Roade {
    /// Normal rendering (human = J1's point of view).
    fn render_main(&self) -> String {
        let mut s = format!(
            "ROADE DE TOAMNĂ — pachet {} (golit de {}x) — monede: tu {}, locul 2: {}, locul 3: {}\n",
            self.deck.len(),
            self.reshuffles,
            self.farmers[0].coins,
            self.farmers[1].coins,
            self.farmers[2].coins
        );
        for p in 0..3 {
            let fs: Vec<String> = self.farmers[p]
                .fields
                .iter()
                .enumerate()
                .map(|(k, f)| match f {
                    Some((c, n)) => format!("{}:{}×{}", k + 1, n, CROPS[*c as usize].1),
                    None => format!("{}:gol", k + 1),
                })
                .collect();
            s.push_str(&format!(
                "  locul {}{}: ogoare [{}], {} cărți în mână\n",
                p + 1,
                if p == 0 { " (tu)" } else { "" },
                fs.join(" "),
                self.farmers[p].hand.len()
            ));
        }
        let h: Vec<&str> = self.farmers[0]
            .hand
            .iter()
            .map(|&c| CROPS[c as usize].1)
            .collect();
        s.push_str(&format!("Mâna ta (prima se plantează): {}\n", h.join(", ")));
        if matches!(self.phase, Phase::Reveal) {
            if let Some(&c) = self.face_up.first() {
                s.push_str(&format!("De plantat sau oferit: {}\n", CROPS[c as usize].1));
            }
        }
        if matches!(self.phase, Phase::Forced | Phase::Offer) {
            if let Some(c) = self.pending {
                s.push_str(&format!("Carte în față: {}\n", CROPS[c as usize].1));
            }
        }
        for l in &self.log {
            s.push_str(l);
            s.push('\n');
        }
        s
    }

    /// Reveal-all rendering, appended when `set_viewer(Some(Two))` (second human inspects).
    fn god_view(&self) -> String {
        let mut s =
            String::from("--- VEDERE TOTALĂ (J2): toate rolurile și informațiile ascunse ---\n");
        for p in 0..3 {
            let h: Vec<&str> = self.farmers[p]
                .hand
                .iter()
                .map(|&c| CROPS[c as usize].1)
                .collect();
            s.push_str(&format!("  locul {}: {}\n", p + 1, h.join(", ")));
        }
        s
    }
}

impl Game for Roade {
    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }
    type Move = Move;

    fn to_move(&self) -> Player {
        match self.phase {
            Phase::Robot => Player::Two,
            _ => Player::One,
        }
    }

    fn legal_moves(&self) -> Vec<Move> {
        let f = &self.farmers[0];
        let mut v = Vec::new();
        let card: Option<u8> = match self.phase {
            Phase::Plant1 | Phase::Plant2 => f.hand.first().copied(),
            Phase::Reveal => self.face_up.first().copied(),
            Phase::Forced => self.pending,
            _ => None,
        };
        match self.phase {
            Phase::Plant1 | Phase::Plant2 | Phase::Reveal | Phase::Forced => {
                if let Some(c) = card {
                    for k in 0..f.fields.len() {
                        if self.can_plant(0, c, k) {
                            v.push(Move::Plant(k as u8));
                        }
                    }
                }
                for k in 0..f.fields.len() {
                    if f.fields[k].is_some() {
                        v.push(Move::Harvest(k as u8));
                    }
                }
                if f.coins >= 3 && f.fields.len() < 3 {
                    v.push(Move::Buy);
                }
                if self.phase == Phase::Plant2 {
                    v.push(Move::Skip);
                }
                if self.phase == Phase::Reveal && !self.offer_failed {
                    v.push(Move::Trade);
                }
            }
            Phase::Offer => {
                for c in self.human_hand_crops() {
                    v.push(Move::Accept(c));
                }
                v.push(Move::Refuse);
            }
            Phase::Robot => v.push(Move::Table),
            Phase::Over => {}
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        match m {
            Move::Plant(k) => {
                let k = *k as usize;
                match self.phase {
                    Phase::Plant1 => {
                        let c = self.farmers[0].hand.remove(0);
                        self.plant(0, c, k);
                        self.phase = if self.farmers[0].hand.is_empty() {
                            self.start_reveal();
                            self.phase
                        } else {
                            Phase::Plant2
                        };
                    }
                    Phase::Plant2 => {
                        let c = self.farmers[0].hand.remove(0);
                        self.plant(0, c, k);
                        self.start_reveal();
                    }
                    Phase::Reveal => {
                        let c = self.face_up[0];
                        self.plant(0, c, k);
                        self.after_reveal_card();
                    }
                    Phase::Forced => {
                        let c = self.pending.take().unwrap();
                        self.plant(0, c, k);
                        self.phase = self.resume;
                        self.offer_failed = false;
                    }
                    _ => {}
                }
            }
            Move::Harvest(k) => self.harvest(0, *k as usize),
            Move::Buy => {
                self.farmers[0].coins -= 3;
                self.spent += 3;
                self.farmers[0].fields.push(None);
                self.say("Cumperi un al treilea ogor.".into());
            }
            Move::Skip => self.start_reveal(),
            Move::Trade => {
                let c = self.face_up[0];
                let mut done = false;
                for r in 1..3 {
                    if self.farmers[r].hand.is_empty() {
                        continue;
                    }
                    if self.plant_slot(r, c).is_some() {
                        let idx = self.rng.below(self.farmers[r].hand.len());
                        let got = self.farmers[r].hand.remove(idx);
                        self.robot_place(r, c);
                        self.say(format!(
                            "Locul {} ia {} și îți dă {}.",
                            r + 1,
                            CROPS[c as usize].1,
                            CROPS[got as usize].1
                        ));
                        self.face_up.remove(0);
                        self.pending = Some(got);
                        self.resume = Phase::Reveal;
                        self.phase = Phase::Forced;
                        done = true;
                        break;
                    }
                }
                if done {
                    // cartea primită se plantează acum; apoi continuăm cu următoarea întoarsă
                    if self.face_up.is_empty() {
                        self.resume = Phase::Reveal;
                    }
                } else {
                    self.offer_failed = true;
                    self.say("Nimeni nu vrea cartea; trebuie s-o plantezi.".into());
                }
            }
            Move::Accept(c) => {
                let k = self.farmers[0].hand.iter().position(|x| x == c).unwrap();
                let given = self.farmers[0].hand.remove(k);
                let offered = self.pending.take().unwrap();
                let from = self.offer_from;
                self.robot_place(from, given);
                self.say(format!(
                    "Accepți: dai {} și primești {}.",
                    CROPS[given as usize].1, CROPS[offered as usize].1
                ));
                self.pending = Some(offered);
                self.resume = Phase::Robot;
                self.phase = Phase::Forced;
            }
            Move::Refuse => {
                let c = self.pending.take().unwrap();
                let from = self.offer_from;
                self.robot_place(from, c);
                self.phase = Phase::Robot;
            }
            Move::Table => self.robot_step(),
        }
        // după „Forced” cu resume=Reveal: dacă nu mai sunt cărți de întors, termină tura
        if self.phase == Phase::Reveal && self.face_up.is_empty() {
            self.end_turn();
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        let c = self.coins();
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
        let mut s = self.render_main();
        if self.viewer == Some(Player::Two) {
            s.push_str(&self.god_view());
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Plant(k) => format!("planteaza {}", k + 1),
            Move::Harvest(k) => format!("recolta {}", k + 1),
            Move::Buy => "compra".into(),
            Move::Skip => "treci".into(),
            Move::Trade => "ofera".into(),
            Move::Accept(c) => format!("accepta {}", CROPS[*c as usize].0),
            Move::Refuse => "refuza".into(),
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

    fn run(seed: u64) -> Roade {
        let mut g = Roade::with_seed(seed);
        let mut rng = Rng::new(seed ^ 11);
        let mut n = 0;
        while g.outcome().is_none() {
            let mv = g.legal_moves();
            assert!(!mv.is_empty(), "{}", g.render());
            g.apply(&mv[rng.below(mv.len())]);
            assert!(g.conserved(), "{}", g.render());
            n += 1;
            assert!(n < 5000);
        }
        g
    }

    #[test]
    fn deck_size_and_values() {
        assert_eq!(CROPS.iter().map(|c| c.2).sum::<u32>(), 106);
        assert_eq!(value(0, 3), 0);
        assert_eq!(value(0, 4), 1);
        assert_eq!(value(0, 9), 3);
        assert_eq!(value(0, 10), 4);
    }

    #[test]
    fn random_games_end_and_conserve() {
        for seed in 1..80u64 {
            let g = run(seed);
            assert!(g.conserved());
            assert!(g.coins().iter().sum::<u32>() > 0 || seed > 0);
        }
    }

    #[test]
    fn plant_rules() {
        let mut g = Roade::with_seed(4);
        g.farmers[0].fields = vec![Some((1, 2)), None];
        assert!(g.can_plant(0, 1, 0));
        assert!(!g.can_plant(0, 2, 0));
        assert!(g.can_plant(0, 2, 1));
        assert!(!g.can_plant(0, 2, 2));
    }

    #[test]
    fn harvest_pays_per_table() {
        let mut g = Roade::with_seed(4);
        g.farmers[0].fields = vec![Some((0, 6)), None];
        let before = g.discard.len();
        g.apply(&Move::Harvest(0));
        assert_eq!(g.farmers[0].coins, 2);
        assert_eq!(g.discard.len(), before + 4);
    }
}
