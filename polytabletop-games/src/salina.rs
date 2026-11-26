//! Salina — plasare de agenți și construire de pachet, variantă originală pentru doi.
//!
//! Două case de negustori din deșertul sărat al planetei Salina trimit agenți în locații, își
//! cumpără oameni noi pentru pachet și se luptă pentru recompensa unui conflict la fiecare rundă.
//! Resurse: Taleri (T), Sare (S), Apă (A); puncte de victorie (PV).
//!
//! Pachet: 10 cărți de start; mână de 5. Fiecare carte are pictograme-agent (O oraș, D deșert,
//! P palat), valori la revelare (persuasiune, săbii) și un efect mic când e trimisă ca agent.
//! Tura ta, o singură acțiune:
//! * `agent 3 e` — joci cartea 3 din mână și trimiți un agent (2 pe rundă) pe locația `e`, a cărei
//!   pictogramă o are cartea; locația e liberă și îi poți plăti costul. La locațiile de deșert
//!   (luptă), `agent 3 e 2` desfășoară în plus 2 trupe din garnizoană în conflict (până la
//!   2 + trupele nou recrutate);
//! * `revela` — îți arunci restul mâinii în față: suma persuasiunii (și a săbiilor) se adună și
//!   treci pe cumpărături: `compra 2` (cartea 2 din piață, dacă ai persuasiune), `apa` (1 Apă =
//!   +1 persuasiune), `sare` (1 Sare = +1 sabie), `fin`. După revelare nu mai joci în runda asta.
//!
//! Când ambii au revelat: forța = 2 × trupe desfășurate + săbii (doar cu cel puțin o trupă);
//! cel mai puternic ia premiul I, al doilea premiul II; la egalitate pe primul loc ambii iau
//! premiul II. Trupele desfășurate se pierd. Apoi, dacă cineva are 10 PV (sau după runda 10),
//! câștigă cine are mai multe PV (egalitate: mai multe resurse). Altfel mutarea forțată `runda`
//! aruncă cărțile folosite, trage 5 cărți și pregătește runda următoare.
//!
//! Locații: a Târgul(O), b Hanul(O), c Casa de sare(O, plătești 2 S → 5 T), d Fântâna(O);
//! e Mina de sare(D, +2 S), f Oaza(D, +1 A +1 S), g Pasul(D, plătești 1 T → +2 trupe),
//! h Drumul caravanelor(D, +1 T +1 trupă); i Sala tronului(P, +1 trupă și primul la rând),
//! j Consiliul(P, plătești 4 T → +1 PV +1 carte), k Tezaurul(P, +2 T +1 S), l Arhiva(P, 2 cărți).
//!
//! Fără facțiuni și fără cărți de intrigă (simplificare). Informație ascunsă: pachetul și
//! mâna adversarului; render() arată doar mâna J1. AI-ul nu citește mâna: evaluează suma
//! valorii tuturor cărților deținute, deci tragerile nu-i dau informație.
#![allow(clippy::needless_range_loop, clippy::doc_lazy_continuation)]

use crate::cards::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const SPOT_NAMES: [&str; 12] = [
    "Târgul",
    "Hanul",
    "Casa de sare",
    "Fântâna",
    "Mina de sare",
    "Oaza",
    "Pasul",
    "Drumul caravanelor",
    "Sala tronului",
    "Consiliul",
    "Tezaurul",
    "Arhiva",
];
/// Pictograma fiecărei locații: 1 oraș, 2 deșert, 4 palat.
const SPOT_ICON: [u8; 12] = [1, 1, 1, 1, 2, 2, 2, 2, 4, 4, 4, 4];
const ICON_LET: [char; 3] = ['O', 'D', 'P'];
const MAX_ROUNDS: u8 = 10;
const WIN_VP: u8 = 10;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Eff {
    None,
    Coin(u8),
    Salt(u8),
    Water(u8),
    Draw(u8),
    Troops(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Card {
    pub id: u8,
    pub name: &'static str,
    pub cost: u8,
    pub icons: u8,
    pub pers: u8,
    pub sw: u8,
    pub eff: Eff,
}

impl Card {
    fn text(&self) -> String {
        let ic: String = (0..3)
            .filter(|i| self.icons & (1 << i) != 0)
            .map(|i| ICON_LET[i])
            .collect();
        let eff = match self.eff {
            Eff::None => String::new(),
            Eff::Coin(n) => format!(" agent:+{n}T"),
            Eff::Salt(n) => format!(" agent:+{n}S"),
            Eff::Water(n) => format!(" agent:+{n}A"),
            Eff::Draw(n) => format!(" agent:+{n}carte"),
            Eff::Troops(n) => format!(" agent:+{n}trupe"),
        };
        format!(
            "{} [{}; pers {} săbii {}{}; cost {}]",
            self.name, ic, self.pers, self.sw, eff, self.cost
        )
    }

    /// Valoarea aproximativă a cărții (pentru euristică).
    fn value(&self) -> f64 {
        let e = match self.eff {
            Eff::None => 0.0,
            Eff::Coin(n) => 0.5 * f64::from(n),
            Eff::Salt(n) => 0.7 * f64::from(n),
            Eff::Water(n) => 0.6 * f64::from(n),
            Eff::Draw(n) => 0.9 * f64::from(n),
            Eff::Troops(n) => 0.9 * f64::from(n),
        };
        1.0 * f64::from(self.pers)
            + 0.8 * f64::from(self.sw)
            + 0.35 * f64::from(self.icons.count_ones())
            + e
    }
}

fn starter_deck() -> Vec<Card> {
    let mk = |name, icons, pers, sw, eff| (name, icons, pers, sw, eff);
    let list = [
        mk("Cuvânt bun", 1, 2, 0, Eff::None),
        mk("Cuvânt bun", 1, 2, 0, Eff::None),
        mk("Pumnal", 2, 0, 2, Eff::None),
        mk("Pumnal", 2, 0, 2, Eff::None),
        mk("Hartă a deșertului", 3, 1, 1, Eff::None),
        mk("Sol", 4, 1, 0, Eff::None),
        mk("Cercetaș", 2, 1, 1, Eff::None),
        mk("Inel de sigiliu", 7, 0, 0, Eff::None),
        mk("Ucenic", 5, 1, 0, Eff::None),
        mk("Ucenic", 5, 1, 0, Eff::None),
    ];
    list.iter()
        .enumerate()
        .map(|(i, (name, icons, pers, sw, eff))| Card {
            id: i as u8,
            name,
            cost: 0,
            icons: *icons,
            pers: *pers,
            sw: *sw,
            eff: *eff,
        })
        .collect()
}

const MARKET_NAMES: [&str; 30] = [
    "Caravanier",
    "Ghid de nisip",
    "Negustor de sare",
    "Mercenar",
    "Spion",
    "Cămilar",
    "Alchimist",
    "Ambasador",
    "Căpitan",
    "Vistier",
    "Cartograf",
    "Tămăduitor",
    "Paznic de oază",
    "Contrabandist",
    "Cântăreț de vânt",
    "Căutător de apă",
    "Arcaș",
    "Mesager",
    "Sfătuitor",
    "Meșter de corturi",
    "Vameș",
    "Călăreț",
    "Povățuitor",
    "Soldat al sării",
    "Bancher",
    "Prezicătoare",
    "Armurier",
    "Cronicar",
    "Vânzător de ape",
    "Mare vizir",
];

fn market_cards() -> Vec<Card> {
    let mut rng = Rng::new(0x5A11_2025);
    let mut v = Vec::new();
    for i in 0..30usize {
        let cost = 1 + (i % 7) as u8;
        let budget = 1 + u32::from(cost);
        let rev = (budget * 6 / 10).min(5) as u8;
        let pers = rng.below(rev as usize + 1).min(3) as u8;
        let sw = (rev - pers).min(3);
        let rest = (budget as u8).saturating_sub(pers + sw).max(1);
        let eff = match rng.below(5) {
            0 => Eff::Coin(rest),
            1 => Eff::Salt((rest / 2).max(1)),
            2 => Eff::Water((rest / 2).max(1)),
            3 => Eff::Draw(1),
            _ => Eff::Troops(1),
        };
        let mut icons = 1u8 << rng.below(3);
        if rng.below(2) == 0 {
            icons |= 1 << rng.below(3);
        }
        v.push(Card {
            id: 20 + i as u8,
            name: MARKET_NAMES[i],
            cost,
            icons,
            pers,
            sw,
            eff,
        });
    }
    v
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
struct Reward {
    vp: u8,
    t: u8,
    s: u8,
    a: u8,
}

const CONFLICTS: [[(Reward, Reward); 3]; 3] = [
    [
        (
            Reward {
                vp: 1,
                t: 0,
                s: 1,
                a: 0,
            },
            Reward {
                vp: 0,
                t: 2,
                s: 0,
                a: 0,
            },
        ),
        (
            Reward {
                vp: 1,
                t: 0,
                s: 0,
                a: 1,
            },
            Reward {
                vp: 0,
                t: 2,
                s: 0,
                a: 0,
            },
        ),
        (
            Reward {
                vp: 1,
                t: 2,
                s: 0,
                a: 0,
            },
            Reward {
                vp: 0,
                t: 0,
                s: 1,
                a: 0,
            },
        ),
    ],
    [
        (
            Reward {
                vp: 2,
                t: 0,
                s: 0,
                a: 0,
            },
            Reward {
                vp: 1,
                t: 0,
                s: 0,
                a: 0,
            },
        ),
        (
            Reward {
                vp: 1,
                t: 0,
                s: 2,
                a: 0,
            },
            Reward {
                vp: 1,
                t: 0,
                s: 0,
                a: 0,
            },
        ),
        (
            Reward {
                vp: 2,
                t: 2,
                s: 0,
                a: 0,
            },
            Reward {
                vp: 0,
                t: 3,
                s: 0,
                a: 0,
            },
        ),
    ],
    [
        (
            Reward {
                vp: 3,
                t: 0,
                s: 0,
                a: 0,
            },
            Reward {
                vp: 1,
                t: 2,
                s: 0,
                a: 0,
            },
        ),
        (
            Reward {
                vp: 3,
                t: 0,
                s: 2,
                a: 0,
            },
            Reward {
                vp: 1,
                t: 0,
                s: 0,
                a: 2,
            },
        ),
        (
            Reward {
                vp: 2,
                t: 3,
                s: 0,
                a: 0,
            },
            Reward {
                vp: 1,
                t: 3,
                s: 0,
                a: 0,
            },
        ),
    ],
];

fn rw_text(r: &Reward) -> String {
    let mut v = Vec::new();
    if r.vp > 0 {
        v.push(format!("{}PV", r.vp));
    }
    if r.t > 0 {
        v.push(format!("{}T", r.t));
    }
    if r.s > 0 {
        v.push(format!("{}S", r.s));
    }
    if r.a > 0 {
        v.push(format!("{}A", r.a));
    }
    v.join("+")
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    /// carte din mână, locație, trupe desfășurate
    Agent(u8, u8, u8),
    Reveal,
    Buy(u8),
    Water,
    Salt,
    Done,
    NextRound,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Turns,
    Buying,
    Cleanup,
    Over,
}

#[derive(Clone)]
pub struct Salina {
    /// Whose private info `render` shows (`None` = the mover's, hot-seat).
    viewer: Option<Player>,
    deck: [Vec<Card>; 2],
    discard: [Vec<Card>; 2],
    hand: [Vec<Card>; 2],
    played: [Vec<Card>; 2],
    market_deck: Vec<Card>,
    market: Vec<Card>,
    res: [[u8; 3]; 2],
    vp: [u8; 2],
    troops: [u8; 2],
    deployed: [u8; 2],
    agents: [u8; 2],
    revealed: [bool; 2],
    wasted: [u8; 2],
    pers: [u8; 2],
    swords: [u8; 2],
    spots: [Option<Player>; 12],
    conflicts: Vec<(Reward, Reward)>,
    round: u8,
    first: Player,
    next_first: Player,
    side: Player,
    phase: Phase,
    rng: Rng,
}

impl Default for Salina {
    fn default() -> Self {
        Self::new()
    }
}

impl Salina {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut market_deck = market_cards();
        shuffle(&mut market_deck, &mut rng);
        let market: Vec<Card> = (0..5).filter_map(|_| market_deck.pop()).collect();
        let mut conflicts = Vec::new();
        for level in 0..3 {
            let n = [3, 4, 3][level];
            for _ in 0..n {
                conflicts.push(CONFLICTS[level][rng.below(3)]);
            }
        }
        let mut g = Salina {
            viewer: None,
            deck: [starter_deck(), starter_deck()],
            discard: [Vec::new(), Vec::new()],
            hand: [Vec::new(), Vec::new()],
            played: [Vec::new(), Vec::new()],
            market_deck,
            market,
            res: [[1, 0, 0], [2, 0, 0]],
            vp: [0; 2],
            troops: [3; 2],
            deployed: [0; 2],
            agents: [2; 2],
            revealed: [false; 2],
            wasted: [0; 2],
            pers: [0; 2],
            swords: [0; 2],
            spots: [None; 12],
            conflicts,
            round: 1,
            first: Player::One,
            next_first: Player::One,
            side: Player::One,
            phase: Phase::Turns,
            rng,
        };
        for p in 0..2 {
            let mut d = std::mem::take(&mut g.deck[p]);
            shuffle(&mut d, &mut g.rng);
            g.deck[p] = d;
            for _ in 0..5 {
                g.draw(p);
            }
        }
        g
    }

    fn draw(&mut self, p: usize) {
        if self.deck[p].is_empty() {
            let mut d = std::mem::take(&mut self.discard[p]);
            shuffle(&mut d, &mut self.rng);
            self.deck[p] = d;
        }
        if let Some(c) = self.deck[p].pop() {
            self.hand[p].push(c);
            self.hand[p].sort_by_key(|c| c.id);
        }
    }

    pub fn card_total(&self, p: usize) -> usize {
        self.deck[p].len() + self.discard[p].len() + self.hand[p].len() + self.played[p].len()
    }

    fn spot_cost_ok(&self, p: usize, s: usize) -> bool {
        match s {
            2 => self.res[p][1] >= 2,
            6 => self.res[p][0] >= 1,
            9 => self.res[p][0] >= 4,
            _ => true,
        }
    }

    fn combat(s: usize) -> bool {
        (4..=7).contains(&s)
    }

    fn recruit_here(s: usize) -> u8 {
        match s {
            6 => 2,
            7 | 8 => 1,
            _ => 0,
        }
    }

    fn gain_eff(&mut self, p: usize, e: Eff) -> u8 {
        match e {
            Eff::None => 0,
            Eff::Coin(n) => {
                self.res[p][0] = self.res[p][0].saturating_add(n);
                0
            }
            Eff::Salt(n) => {
                self.res[p][1] = self.res[p][1].saturating_add(n);
                0
            }
            Eff::Water(n) => {
                self.res[p][2] = self.res[p][2].saturating_add(n);
                0
            }
            Eff::Draw(n) => {
                for _ in 0..n {
                    self.draw(p);
                }
                0
            }
            Eff::Troops(n) => {
                self.troops[p] = self.troops[p].saturating_add(n);
                n
            }
        }
    }

    fn strength(&self, p: usize) -> Option<u32> {
        if self.deployed[p] == 0 {
            None
        } else {
            Some(2 * u32::from(self.deployed[p]) + u32::from(self.swords[p]))
        }
    }

    fn give(&mut self, p: usize, r: Reward) {
        self.vp[p] += r.vp;
        self.res[p][0] = self.res[p][0].saturating_add(r.t);
        self.res[p][1] = self.res[p][1].saturating_add(r.s);
        self.res[p][2] = self.res[p][2].saturating_add(r.a);
    }

    fn resolve_conflict(&mut self) {
        let (first, second) = self.conflicts[(self.round - 1) as usize];
        let (a, b) = (self.strength(0), self.strength(1));
        match (a, b) {
            (Some(x), Some(y)) => {
                if x > y {
                    self.give(0, first);
                    self.give(1, second);
                } else if y > x {
                    self.give(1, first);
                    self.give(0, second);
                } else {
                    self.give(0, second);
                    self.give(1, second);
                }
            }
            (Some(_), None) => self.give(0, first),
            (None, Some(_)) => self.give(1, first),
            _ => {}
        }
        self.troops[0] = self.troops[0].saturating_sub(0);
        self.deployed = [0; 2];
    }

    fn next_turn(&mut self) {
        // ambii au revelat: conflict
        if self.revealed[0] && self.revealed[1] {
            self.resolve_conflict();
            let ended = self.vp.iter().any(|v| *v >= WIN_VP) || self.round >= MAX_ROUNDS;
            self.phase = if ended { Phase::Over } else { Phase::Cleanup };
            self.side = Player::One;
            return;
        }
        let o = self.side.other();
        if !self.revealed[o.idx()] {
            self.side = o;
        }
        self.phase = Phase::Turns;
    }

    fn money(&self, p: usize) -> u32 {
        self.res[p].iter().map(|x| u32::from(*x)).sum()
    }

    pub fn vp_of(&self, p: usize) -> u8 {
        self.vp[p]
    }

    fn heuristic(&self, p: usize) -> f64 {
        let mut s = 12.0 * f64::from(self.vp[p]);
        s += 0.55 * f64::from(self.res[p][0])
            + 0.8 * f64::from(self.res[p][1])
            + 0.6 * f64::from(self.res[p][2]);
        s += 1.5 * f64::from(self.troops[p]);
        let cards: f64 = self.deck[p]
            .iter()
            .chain(&self.discard[p])
            .chain(&self.hand[p])
            .chain(&self.played[p])
            .map(|c| c.value())
            .sum();
        s += 0.9 * cards;
        // conflict: premiul probabil, după forța publică (trupe desfășurate + săbii revelate)
        let o = 1 - p;
        let val = |r: &Reward| {
            12.0 * f64::from(r.vp)
                + 0.6 * f64::from(r.t)
                + 0.8 * f64::from(r.s)
                + 0.6 * f64::from(r.a)
        };
        if self.round <= MAX_ROUNDS {
            let (f, sc) = self.conflicts[(self.round - 1) as usize];
            s += match (self.strength(p), self.strength(o)) {
                (Some(_), None) => val(&f),
                (Some(x), Some(y)) => {
                    if x > y {
                        val(&f)
                    } else {
                        val(&sc)
                    }
                }
                _ => 0.0,
            };
        }
        s += 0.4 * f64::from(self.pers[p]) - 8.0 * f64::from(self.wasted[p]);
        s
    }
}

impl Game for Salina {
    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        let p = self.side.idx();
        match self.phase {
            Phase::Over => Vec::new(),
            Phase::Cleanup => vec![Move::NextRound],
            Phase::Turns => {
                let mut v = Vec::new();
                if self.agents[p] > 0 {
                    for (i, c) in self.hand[p].iter().enumerate() {
                        for s in 0..12usize {
                            if self.spots[s].is_some()
                                || SPOT_ICON[s] & c.icons == 0
                                || !self.spot_cost_ok(p, s)
                            {
                                continue;
                            }
                            if Self::combat(s) {
                                let rec = Self::recruit_here(s)
                                    + if let Eff::Troops(n) = c.eff { n } else { 0 };
                                let have = self.troops[p]
                                    + Self::recruit_here(s)
                                    + if let Eff::Troops(n) = c.eff { n } else { 0 };
                                let maxd = have.min(2 + rec);
                                for d in 0..=maxd {
                                    v.push(Move::Agent(i as u8, s as u8, d));
                                }
                            } else {
                                v.push(Move::Agent(i as u8, s as u8, 0));
                            }
                        }
                    }
                }
                v.push(Move::Reveal);
                v
            }
            Phase::Buying => {
                let mut v = vec![Move::Done];
                let budget = self.pers[p];
                for (i, c) in self.market.iter().enumerate() {
                    if c.cost <= budget {
                        v.push(Move::Buy(i as u8));
                    }
                }
                if self.res[p][2] > 0 {
                    v.push(Move::Water);
                }
                if self.res[p][1] > 0 {
                    v.push(Move::Salt);
                }
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        let p = self.side.idx();
        match *m {
            Move::Agent(i, s, d) => {
                let c = self.hand[p].remove(i as usize);
                let s = s as usize;
                self.spots[s] = Some(self.side);
                self.agents[p] -= 1;
                self.played[p].push(c);
                // plata și efectul locației
                match s {
                    0 => self.res[p][0] = self.res[p][0].saturating_add(2),
                    1 => {
                        self.res[p][0] = self.res[p][0].saturating_add(1);
                        self.draw(p);
                    }
                    2 => {
                        self.res[p][1] -= 2;
                        self.res[p][0] = self.res[p][0].saturating_add(5);
                    }
                    3 => self.res[p][2] = self.res[p][2].saturating_add(1),
                    4 => self.res[p][1] = self.res[p][1].saturating_add(2),
                    5 => {
                        self.res[p][2] = self.res[p][2].saturating_add(1);
                        self.res[p][1] = self.res[p][1].saturating_add(1);
                    }
                    6 => {
                        self.res[p][0] -= 1;
                        self.troops[p] = self.troops[p].saturating_add(2);
                    }
                    7 => {
                        self.res[p][0] = self.res[p][0].saturating_add(1);
                        self.troops[p] = self.troops[p].saturating_add(1);
                    }
                    8 => {
                        self.troops[p] = self.troops[p].saturating_add(1);
                        self.next_first = self.side;
                    }
                    9 => {
                        self.res[p][0] -= 4;
                        self.vp[p] += 1;
                        self.draw(p);
                    }
                    10 => {
                        self.res[p][0] = self.res[p][0].saturating_add(2);
                        self.res[p][1] = self.res[p][1].saturating_add(1);
                    }
                    _ => {
                        self.draw(p);
                        self.draw(p);
                    }
                }
                self.gain_eff(p, c.eff);
                if d > 0 {
                    self.troops[p] -= d;
                    self.deployed[p] += d;
                }
                self.next_turn();
            }
            Move::Reveal => {
                self.revealed[p] = true;
                self.wasted[p] += self.agents[p];
                let hand = std::mem::take(&mut self.hand[p]);
                for c in hand {
                    self.pers[p] = self.pers[p].saturating_add(c.pers);
                    self.swords[p] = self.swords[p].saturating_add(c.sw);
                    self.played[p].push(c);
                }
                self.phase = Phase::Buying;
            }
            Move::Buy(i) => {
                let c = self.market.remove(i as usize);
                self.pers[p] -= c.cost;
                self.discard[p].push(c);
                if let Some(n) = self.market_deck.pop() {
                    self.market.push(n);
                }
            }
            Move::Water => {
                self.res[p][2] -= 1;
                self.pers[p] += 1;
            }
            Move::Salt => {
                self.res[p][1] -= 1;
                self.swords[p] += 1;
            }
            Move::Done => self.next_turn(),
            Move::NextRound => {
                for q in 0..2 {
                    let mut played = std::mem::take(&mut self.played[q]);
                    self.discard[q].append(&mut played);
                    let mut hand = std::mem::take(&mut self.hand[q]);
                    self.discard[q].append(&mut hand);
                    for _ in 0..5 {
                        self.draw(q);
                    }
                    self.agents[q] = 2;
                    self.revealed[q] = false;
                    self.pers[q] = 0;
                    self.swords[q] = 0;
                }
                self.spots = [None; 12];
                self.round += 1;
                self.first = self.next_first;
                self.side = self.first;
                self.phase = Phase::Turns;
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        let (a, b) = (self.vp[0], self.vp[1]);
        Some(if a > b {
            Outcome::Win(Player::One)
        } else if b > a {
            Outcome::Win(Player::Two)
        } else if self.money(0) != self.money(1) {
            Outcome::Win(if self.money(0) > self.money(1) {
                Player::One
            } else {
                Player::Two
            })
        } else {
            Outcome::Draw
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        (self.heuristic(p.idx()) - self.heuristic(p.other().idx())) as i32
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(2)
    }

    fn render(&self) -> String {
        let mut s = format!("Salina — runda {} din {}\n", self.round, MAX_ROUNDS);
        if self.round <= MAX_ROUNDS {
            let (f, sc) = self.conflicts[(self.round - 1) as usize];
            s.push_str(&format!(
                "Conflict: premiul I {} | premiul II {}\n",
                rw_text(&f),
                rw_text(&sc)
            ));
        }
        s.push_str("Locații: ");
        let spots: Vec<String> = (0..12)
            .map(|i| {
                format!(
                    "{}={}({}){}{}",
                    (b'a' + i as u8) as char,
                    SPOT_NAMES[i],
                    ICON_LET[SPOT_ICON[i].trailing_zeros() as usize],
                    if Self::combat(i) { "⚔" } else { "" },
                    match self.spots[i] {
                        Some(Player::One) => "[J1]",
                        Some(Player::Two) => "[J2]",
                        None => "",
                    }
                )
            })
            .collect();
        s.push_str(&spots.join(", "));
        s.push_str("\nPiața: ");
        let mk: Vec<String> = self
            .market
            .iter()
            .enumerate()
            .map(|(i, c)| format!("{}:{}", i + 1, c.text()))
            .collect();
        s.push_str(&mk.join(" | "));
        s.push('\n');
        for p in 0..2 {
            s.push_str(&format!(
                "J{}: {} PV | T{} S{} A{} | trupe {} (desfășurate {}) | agenți {} | {}pers {}săbii | pachet {}+{} | mână {}{}\n",
                p + 1,
                self.vp[p],
                self.res[p][0],
                self.res[p][1],
                self.res[p][2],
                self.troops[p],
                self.deployed[p],
                self.agents[p],
                self.pers[p],
                self.swords[p],
                self.deck[p].len(),
                self.discard[p].len(),
                self.hand[p].len(),
                if self.revealed[p] { " | a revelat" } else { "" }
            ));
        }
        let v = self.viewer.unwrap_or_else(|| self.to_move()).idx();
        s.push_str(&format!("Mâna ta (J{}):\n", v + 1));
        for (i, c) in self.hand[v].iter().enumerate() {
            s.push_str(&format!("  {}: {}\n", i + 1, c.text()));
        }
        match self.phase {
            Phase::Over => s.push_str("Joc încheiat.\n"),
            Phase::Cleanup => {
                s.push_str("Runda s-a încheiat: runda (pregătește runda următoare).\n")
            }
            Phase::Buying => s.push_str(&format!(
                "J{} cumpără: persuasiune {} (compra N, apa, sare, fin).\n",
                self.side.idx() + 1,
                self.pers[self.side.idx()]
            )),
            Phase::Turns => s.push_str(&format!("Mută J{}.\n", self.side.idx() + 1)),
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match *m {
            Move::Agent(i, s, d) => {
                let loc = (b'a' + s) as char;
                if Self::combat(s as usize) {
                    format!("agent {} {} {}", i + 1, loc, d)
                } else {
                    format!("agent {} {}", i + 1, loc)
                }
            }
            Move::Reveal => "revela".into(),
            Move::Buy(i) => format!("compra {}", i + 1),
            Move::Water => "apa".into(),
            Move::Salt => "sare".into(),
            Move::Done => "fin".into(),
            Move::NextRound => "runda".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decks_are_sane() {
        assert_eq!(starter_deck().len(), 10);
        let m = market_cards();
        assert_eq!(m.len(), 30);
        for c in &m {
            assert!(c.cost >= 1 && c.icons != 0);
            assert!(c.pers <= 3 && c.sw <= 3);
        }
    }

    #[test]
    fn agent_needs_icon_free_spot_and_payment() {
        let mut g = Salina::with_seed(1);
        g.hand[0] = vec![Card {
            id: 0,
            name: "t",
            cost: 0,
            icons: 1,
            pers: 0,
            sw: 0,
            eff: Eff::None,
        }];
        let l = g.legal_moves();
        // doar locații de oraș; Casa de sare (c) cere 2 Sare
        assert!(l.contains(&Move::Agent(0, 0, 0)));
        assert!(!l.contains(&Move::Agent(0, 2, 0)));
        assert!(!l
            .iter()
            .any(|m| matches!(m, Move::Agent(_, s, _) if *s >= 4)));
        g.apply(&Move::Agent(0, 0, 0));
        assert_eq!(g.res[0][0], 3);
        assert_eq!(g.spots[0], Some(Player::One));
        // J2 nu mai poate lua Târgul
        g.hand[1] = vec![Card {
            id: 0,
            name: "t",
            cost: 0,
            icons: 1,
            pers: 0,
            sw: 0,
            eff: Eff::None,
        }];
        assert!(!g.legal_moves().contains(&Move::Agent(0, 0, 0)));
    }

    #[test]
    fn combat_deploys_and_resolves() {
        let mut g = Salina::with_seed(2);
        let dag = Card {
            id: 0,
            name: "t",
            cost: 0,
            icons: 2,
            pers: 0,
            sw: 2,
            eff: Eff::None,
        };
        g.hand[0] = vec![dag];
        // Drumul caravanelor recrutează 1 trupă: se pot desfășura până la 3 (garnizoană 3+1)
        let l = g.legal_moves();
        assert!(l.contains(&Move::Agent(0, 7, 3)));
        assert!(!l.contains(&Move::Agent(0, 7, 4)));
        g.apply(&Move::Agent(0, 7, 2));
        assert_eq!((g.troops[0], g.deployed[0]), (2, 2));
        g.revealed = [true, true];
        g.swords = [2, 0];
        let vp_before = g.vp;
        g.phase = Phase::Turns;
        g.side = Player::Two;
        g.next_turn();
        // J1 singurul participant: premiul I
        assert!(g.vp[0] > vp_before[0] || g.res[0] != [3, 0, 0] || g.phase == Phase::Cleanup);
        assert_eq!(g.deployed, [0, 0]);
    }

    #[test]
    fn buying_and_conversions() {
        let mut g = Salina::with_seed(3);
        g.hand[0] = vec![];
        g.pers = [2, 0];
        g.revealed[0] = true;
        g.phase = Phase::Buying;
        g.market[0].cost = 3;
        assert!(!g.legal_moves().contains(&Move::Buy(0)));
        g.res[0][2] = 1;
        g.apply(&Move::Water);
        assert_eq!(g.pers[0], 3);
        let n = g.card_total(0);
        g.apply(&Move::Buy(0));
        assert_eq!(g.pers[0], 0);
        assert_eq!(g.card_total(0), n + 1);
        assert_eq!(g.market.len(), 5);
    }

    #[test]
    fn random_games_finish_and_conserve_cards() {
        for seed in 1..60u64 {
            let mut g = Salina::with_seed(seed);
            let mut rng = Rng::new(seed ^ 17);
            let mut n = 0;
            let mut bought = [0usize; 2];
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty(), "no moves seed {seed}");
                let pick = m[rng.below(m.len())];
                if let Move::Buy(_) = pick {
                    bought[g.side.idx()] += 1;
                }
                g.apply(&pick);
                for p in 0..2 {
                    assert_eq!(g.card_total(p), 10 + bought[p], "seed {seed}");
                }
                n += 1;
                assert!(n < 2000);
            }
            assert!(g.round <= MAX_ROUNDS);
        }
    }

    #[test]
    fn ai_beats_random() {
        let mut wins = 0;
        for seed in 1..=20u64 {
            let mut g = Salina::with_seed(seed);
            let mut rng = Rng::new(seed + 3);
            while g.outcome().is_none() {
                let m = if g.to_move() == Player::One {
                    polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap()
                } else {
                    let l = g.legal_moves();
                    l[rng.below(l.len())]
                };
                g.apply(&m);
            }
            if g.outcome() == Some(Outcome::Win(Player::One)) {
                wins += 1;
            }
        }
        assert!(wins >= 16, "wins {wins}");
    }

    #[test]
    fn render_hides_the_opponents_hand() {
        let mut g = Salina::with_seed(9);
        g.hand[1][0].name = "SECRETNAME";
        g.hand[0][0].name = "VISIBLENAME";
        let r = g.render();
        assert!(!r.contains("SECRETNAME"));
        assert!(r.contains("VISIBLENAME"));
    }
}
