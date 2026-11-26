//! Moșia — joc original de alegere de roluri, plantații și clădiri (2 jucători).
//!
//! Mecanica „fiecare rând alegi un rol, toți execută acțiunea rolului, cel care
//! l-a ales are un privilegiu” este generică; moșia, culturile, clădirile și
//! textele sunt ale noastre. Informație completă, fără zaruri după pregătire.
//!
//! Reguli (variantă compactă):
//! * Culturi: porumb `p`, indigo `i`, zahăr `z`, tutun `t`, cafea `c`
//!   (prețuri 0, 1, 2, 3, 4). Cel mult 8 plantații și 6 mărfuri din fiecare fel.
//! * Șapte roluri: `cultivator`, `constructor`, `producator`, `negustor`,
//!   `capitan`, `vistier`, `zidar`. Într-o rundă fiecare jucător alege 3 roluri
//!   pe rând (începe alternativ); rolul rămas primește +1 monedă, luată de cine
//!   îl alege data viitoare.
//! * `cultivator`: pe rând fiecare ia o plantație vizibilă (`planta 2`); cine
//!   a ales rolul poate lua în loc o carieră (`cariera`). `sari` = nimic.
//! * `constructor`: fiecare poate cumpăra o clădire (`construieste targ`);
//!   costul scade cu numărul de cariere (cel mult 3), cu 1 pentru cine a ales
//!   rolul și cu 1 pentru `hala`. Clădirile sunt unice (prima cumpărată).
//! * `producator`: toate plantațiile dau mărfuri (+1 pentru fabrica culturii);
//!   cine alege rolul primește în plus 1 marfă din cea mai scumpă cultură.
//! * `negustor`: fiecare vinde o marfă (`vinde z`) la prețul ei (+1 la `targ`,
//!   +1 pentru cine a ales rolul); în piață încap 4 mărfuri, de același fel o
//!   dată (cu `targ`, oricând); piața plină se golește la sfârșitul rolului.
//! * `capitan`: pe rând expediați mărfuri (`expediaza z 1`) pe una din cele
//!   două nave (mărimi 4 și 6); o navă duce un singur fel de marfă; fiecare
//!   marfă expediată = 1 punct (+1 cu `debarcader`). Cine poate expedia trebuie.
//! * `vistier`: 2 monede pentru cine alege rolul, 1 pentru celălalt.
//! * `zidar`: cine alege rolul primește o carieră și o monedă.
//! * Final: după runda în care cineva are 5 clădiri, s-au terminat clădirile,
//!   punctele de expediere (40) sau runda 14. Scor = puncte de expediere +
//!   puncte de clădiri + monede / 4 (rotunjit în jos).

#![allow(clippy::needless_range_loop)]

use polytabletop_core::{AiStyle, Game, Outcome, Player};

const CROP: [&str; 5] = ["porumb", "indigo", "zahar", "tutun", "cafea"];
const CROP_L: [char; 5] = ['p', 'i', 'z', 't', 'c'];
const PRICE: [i32; 5] = [0, 1, 2, 3, 4];
const ROLES: [&str; 7] = [
    "cultivator",
    "constructor",
    "producator",
    "negustor",
    "capitan",
    "vistier",
    "zidar",
];
/// (nume, cost, puncte)
const BLD: [(&str, i32, i32); 10] = [
    ("moara", 2, 1),
    ("tesatorie", 3, 1),
    ("rafinarie", 3, 1),
    ("uscator", 4, 2),
    ("pragator", 5, 2),
    ("targ", 3, 2),
    ("debarcader", 4, 2),
    ("hala", 5, 3),
    ("conac", 6, 4),
    ("palat", 8, 6),
];
const B_TARG: usize = 5;
const B_DEB: usize = 6;
const B_HALA: usize = 7;
const SHIP_SIZE: [u8; 2] = [4, 6];
const MAX_PLANT: u8 = 8;
const MAX_GOOD: u8 = 6;
const QUARRIES: u8 = 6;
const POOL: i32 = 40;
const ROUND_CAP: u8 = 14;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Pick(u8),
    Plant(u8),
    Quarry,
    Build(u8),
    Sell(u8),
    Ship(u8, u8),
    Pass,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Pick,
    /// rol, jucătorul care a ales, pasul (0 = cine a ales, 1 = celălalt)
    Act(u8, Player, u8),
    /// capitan: la rând, câte „imposibil” la rând
    Ship(Player, u8),
    Over,
}

#[derive(Clone)]
pub struct Mosia {
    deck: Vec<u8>,
    row: Vec<u8>,
    quarries_left: u8,
    plant: [[u8; 5]; 2],
    quarry: [u8; 2],
    goods: [[u8; 5]; 2],
    coin: [i32; 2],
    blds: [u16; 2],
    stock: u16,
    ship_vp: [i32; 2],
    pool: i32,
    taken: u8,
    role_gold: [u8; 7],
    picks: u8,
    first: Player,
    side: Player,
    phase: Phase,
    house: Vec<u8>,
    ships: [(i8, u8); 2],
    round: u8,
    ending: bool,
    outcome: Option<Outcome>,
    last: String,
}

impl Default for Mosia {
    fn default() -> Self {
        Self::new()
    }
}

impl Mosia {
    pub fn new() -> Self {
        Self::with_seed(polytabletop_core::rng::Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = polytabletop_core::rng::Rng::new(seed);
        let mut deck: Vec<u8> = (0..30).map(|i| (i / 6) as u8).collect();
        for i in (1..deck.len()).rev() {
            let j = rng.below(i + 1);
            deck.swap(i, j);
        }
        let mut g = Mosia {
            deck,
            row: Vec::new(),
            quarries_left: QUARRIES,
            plant: [[0; 5]; 2],
            quarry: [0; 2],
            goods: [[0; 5]; 2],
            coin: [3, 3],
            blds: [0; 2],
            stock: (1 << BLD.len()) - 1,
            ship_vp: [0; 2],
            pool: POOL,
            taken: 0,
            role_gold: [0; 7],
            picks: 0,
            first: Player::One,
            side: Player::One,
            phase: Phase::Pick,
            house: Vec::new(),
            ships: [(-1, 0); 2],
            round: 1,
            ending: false,
            outcome: None,
            last: String::new(),
        };
        // porumb pentru J1, indigo pentru J2 (începutul asimetric obișnuit)
        g.plant[0][0] = 1;
        g.plant[1][1] = 1;
        g.refill();
        g
    }

    pub fn score(&self, p: Player) -> i32 {
        let i = p.idx();
        let b: i32 = (0..BLD.len())
            .filter(|&b| self.blds[i] >> b & 1 == 1)
            .map(|b| BLD[b].2)
            .sum();
        self.ship_vp[i] + b + self.coin[i] / 4
    }

    fn nb(&self, p: Player) -> u32 {
        self.blds[p.idx()].count_ones()
    }

    fn has(&self, p: Player, b: usize) -> bool {
        self.blds[p.idx()] >> b & 1 == 1
    }

    fn plantations(&self, p: Player) -> u8 {
        self.plant[p.idx()].iter().sum()
    }

    fn refill(&mut self) {
        while self.row.len() < 5 {
            match self.deck.pop() {
                Some(c) => self.row.push(c),
                None => break,
            }
        }
    }

    fn build_cost(&self, p: Player, b: usize, picker: bool) -> i32 {
        let mut c = BLD[b].1 - self.quarry[p.idx()].min(3) as i32;
        if picker {
            c -= 1;
        }
        if self.has(p, B_HALA) {
            c -= 1;
        }
        c.max(0)
    }

    fn ship_options(&self, p: Player) -> Vec<(u8, u8)> {
        let mut v = Vec::new();
        for c in 0..5u8 {
            if self.goods[p.idx()][c as usize] == 0 {
                continue;
            }
            for s in 0..2u8 {
                let (t, n) = self.ships[s as usize];
                let other = self.ships[1 - s as usize].0;
                let ok = if t == c as i8 {
                    n < SHIP_SIZE[s as usize]
                } else {
                    t < 0 && other != c as i8
                };
                if ok {
                    v.push((c, s));
                }
            }
        }
        v
    }

    fn actor(&self, role: u8, picker: Player, step: u8) -> Player {
        let _ = role;
        if step == 0 {
            picker
        } else {
            picker.other()
        }
    }

    fn act_moves(&self, role: u8, picker: Player, step: u8) -> Vec<Move> {
        let p = self.actor(role, picker, step);
        let i = p.idx();
        let mut v = Vec::new();
        match role {
            0 => {
                if self.plantations(p) < MAX_PLANT {
                    let mut seen = [false; 5];
                    for (k, &c) in self.row.iter().enumerate() {
                        if !seen[c as usize] {
                            seen[c as usize] = true;
                            v.push(Move::Plant(k as u8));
                        }
                    }
                }
                if step == 0 && self.quarries_left > 0 {
                    v.push(Move::Quarry);
                }
            }
            1 => {
                if self.nb(p) < 8 {
                    for b in 0..BLD.len() {
                        if self.stock >> b & 1 == 1
                            && self.coin[i] >= self.build_cost(p, b, step == 0)
                        {
                            v.push(Move::Build(b as u8));
                        }
                    }
                }
            }
            3 if self.house.len() < 4 => {
                for c in 0..5u8 {
                    if self.goods[i][c as usize] > 0
                        && (!self.house.contains(&c) || self.has(p, B_TARG))
                    {
                        v.push(Move::Sell(c));
                    }
                }
            }
            _ => {}
        }
        v.push(Move::Pass);
        v
    }

    /// Execută în mod automat pașii fără decizie.
    fn settle(&mut self) {
        loop {
            match self.phase {
                Phase::Act(role, picker, step) => {
                    let mv = self.act_moves(role, picker, step);
                    if mv.len() == 1 {
                        self.finish_act(role, picker, step);
                    } else {
                        self.side = self.actor(role, picker, step);
                        return;
                    }
                }
                Phase::Ship(turn, idle) => {
                    if idle >= 2 {
                        self.end_role();
                    } else if self.ship_options(turn).is_empty() {
                        self.phase = Phase::Ship(turn.other(), idle + 1);
                    } else {
                        self.side = turn;
                        return;
                    }
                }
                Phase::Pick => {
                    self.side = if self.picks.is_multiple_of(2) {
                        self.first
                    } else {
                        self.first.other()
                    };
                    return;
                }
                Phase::Over => return,
            }
        }
    }

    fn finish_act(&mut self, role: u8, picker: Player, step: u8) {
        if step == 0 {
            self.phase = Phase::Act(role, picker, 1);
        } else {
            self.end_role();
        }
    }

    fn end_role(&mut self) {
        if self.house.len() >= 4 {
            self.house.clear();
        }
        self.refill();
        for k in 0..2 {
            if self.ships[k].1 >= SHIP_SIZE[k] {
                self.ships[k] = (-1, 0);
            }
        }
        self.phase = Phase::Pick;
        if self.picks >= 6 {
            self.end_round();
        }
    }

    fn end_round(&mut self) {
        for r in 0..7 {
            if self.taken >> r & 1 == 0 {
                self.role_gold[r] += 1;
            }
        }
        self.taken = 0;
        self.picks = 0;
        self.first = self.first.other();
        if self.nb(Player::One) >= 5
            || self.nb(Player::Two) >= 5
            || self.stock == 0
            || self.pool <= 0
        {
            self.ending = true;
        }
        if self.ending || self.round >= ROUND_CAP {
            self.phase = Phase::Over;
            let (a, b) = (self.score(Player::One), self.score(Player::Two));
            self.outcome = Some(if a > b {
                Outcome::Win(Player::One)
            } else if b > a {
                Outcome::Win(Player::Two)
            } else if self.coin[0] != self.coin[1] {
                Outcome::Win(if self.coin[0] > self.coin[1] {
                    Player::One
                } else {
                    Player::Two
                })
            } else {
                Outcome::Draw
            });
            return;
        }
        self.round += 1;
    }

    fn produce(&mut self, p: Player, picker: bool) {
        let i = p.idx();
        for c in 0..5 {
            let n = self.plant[i][c];
            if n == 0 {
                continue;
            }
            let bonus = match c {
                0 => self.has(p, 0),
                1 => self.has(p, 1),
                2 => self.has(p, 2),
                3 => self.has(p, 3),
                _ => self.has(p, 4),
            } as u8;
            self.goods[i][c] = (self.goods[i][c] + n + bonus).min(MAX_GOOD);
        }
        if picker {
            if let Some(c) = (0..5).rev().find(|&c| self.plant[i][c] > 0) {
                self.goods[i][c] = (self.goods[i][c] + 1).min(MAX_GOOD);
            }
        }
    }

    fn pick(&mut self, r: u8) {
        let p = self.side;
        let i = p.idx();
        self.taken |= 1 << r;
        self.picks += 1;
        self.coin[i] += self.role_gold[r as usize] as i32;
        self.role_gold[r as usize] = 0;
        self.last = format!("J{} alege {}", i + 1, ROLES[r as usize]);
        match r {
            2 => {
                self.produce(p, true);
                self.produce(p.other(), false);
                self.end_role();
            }
            5 => {
                self.coin[i] += 2;
                self.coin[1 - i] += 1;
                self.end_role();
            }
            6 => {
                if self.quarries_left > 0 {
                    self.quarries_left -= 1;
                    self.quarry[i] += 1;
                }
                self.coin[i] += 1;
                self.end_role();
            }
            4 => self.phase = Phase::Ship(p, 0),
            _ => self.phase = Phase::Act(r, p, 0),
        }
    }
}

impl Game for Mosia {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        match self.phase {
            Phase::Over => Vec::new(),
            Phase::Pick => (0..7u8)
                .filter(|&r| self.taken >> r & 1 == 0)
                .map(Move::Pick)
                .collect(),
            Phase::Act(role, picker, step) => self.act_moves(role, picker, step),
            Phase::Ship(turn, _) => self
                .ship_options(turn)
                .into_iter()
                .map(|(c, s)| Move::Ship(c, s))
                .collect(),
        }
    }

    fn apply(&mut self, m: &Move) {
        let p = self.side;
        let i = p.idx();
        match (self.phase, *m) {
            (Phase::Pick, Move::Pick(r)) => self.pick(r),
            (Phase::Act(role, picker, step), mv) => {
                match mv {
                    Move::Plant(k) => {
                        let c = self.row.remove(k as usize);
                        self.plant[i][c as usize] += 1;
                        self.last = format!("J{} ia {}", i + 1, CROP[c as usize]);
                    }
                    Move::Quarry => {
                        self.quarries_left -= 1;
                        self.quarry[i] += 1;
                        self.last = format!("J{} ia o carieră", i + 1);
                    }
                    Move::Build(b) => {
                        let cost = self.build_cost(p, b as usize, step == 0);
                        self.coin[i] -= cost;
                        self.blds[i] |= 1 << b;
                        self.stock &= !(1 << b);
                        self.last = format!("J{} construiește {}", i + 1, BLD[b as usize].0);
                    }
                    Move::Sell(c) => {
                        let mut price = PRICE[c as usize];
                        if self.has(p, B_TARG) {
                            price += 1;
                        }
                        if step == 0 {
                            price += 1;
                        }
                        self.goods[i][c as usize] -= 1;
                        self.coin[i] += price;
                        self.house.push(c);
                        self.last = format!("J{} vinde {} (+{price})", i + 1, CROP[c as usize]);
                    }
                    _ => {}
                }
                self.finish_act(role, picker, step);
            }
            (Phase::Ship(turn, _), Move::Ship(c, s)) => {
                let space = SHIP_SIZE[s as usize] - self.ships[s as usize].1;
                let n = self.goods[i][c as usize].min(space);
                self.goods[i][c as usize] -= n;
                self.ships[s as usize].0 = c as i8;
                self.ships[s as usize].1 += n;
                let mut vp = n as i32;
                if self.has(p, B_DEB) {
                    vp += 1;
                }
                let vp = vp.min(self.pool.max(0));
                self.pool -= vp;
                self.ship_vp[i] += vp;
                self.last = format!("J{} expediază {n} {} (+{vp})", i + 1, CROP[c as usize]);
                self.phase = Phase::Ship(turn.other(), 0);
            }
            _ => {}
        }
        self.settle();
    }

    fn outcome(&self) -> Option<Outcome> {
        self.outcome
    }

    fn evaluate(&self, p: Player) -> i32 {
        let side = |q: Player| -> i32 {
            let i = q.idx();
            let mut v = self.score(q) * 100;
            for c in 0..5 {
                v += self.goods[i][c] as i32 * (20 + 15 * PRICE[c]);
                v += self.plant[i][c] as i32 * (40 + 12 * PRICE[c]);
            }
            v += self.quarry[i] as i32 * 25 + self.coin[i] * 12;
            v
        };
        side(p) - side(p.other())
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Moșia — runda {}/{}  (puncte de expediere rămase: {})\n",
            self.round, ROUND_CAP, self.pool
        );
        let row: Vec<String> = self
            .row
            .iter()
            .enumerate()
            .map(|(k, &c)| format!("{}:{}", k + 1, CROP[c as usize]))
            .collect();
        s.push_str(&format!(
            "Plantații vizibile: {} | cariere: {} | pachet: {}\n",
            row.join(" "),
            self.quarries_left,
            self.deck.len()
        ));
        let roles: Vec<String> = (0..7)
            .map(|r| {
                if self.taken >> r & 1 == 1 {
                    format!("({})", ROLES[r])
                } else if self.role_gold[r] > 0 {
                    format!("{}+{}", ROLES[r], self.role_gold[r])
                } else {
                    ROLES[r].to_string()
                }
            })
            .collect();
        s.push_str(&format!("Roluri: {}\n", roles.join(" ")));
        let ships: Vec<String> = (0..2)
            .map(|k| {
                if self.ships[k].0 < 0 {
                    format!("navă{} goală/{}", k + 1, SHIP_SIZE[k])
                } else {
                    format!(
                        "navă{} {}×{}/{}",
                        k + 1,
                        CROP[self.ships[k].0 as usize],
                        self.ships[k].1,
                        SHIP_SIZE[k]
                    )
                }
            })
            .collect();
        let house: Vec<&str> = self.house.iter().map(|&c| CROP[c as usize]).collect();
        s.push_str(&format!(
            "{} | piață: [{}]\n",
            ships.join(", "),
            house.join(" ")
        ));
        let free: Vec<&str> = (0..BLD.len())
            .filter(|&b| self.stock >> b & 1 == 1)
            .map(|b| BLD[b].0)
            .collect();
        let costs: Vec<String> = free
            .iter()
            .map(|n| {
                let b = BLD.iter().position(|x| x.0 == *n).unwrap_or(0);
                format!("{n}({}/{}p)", BLD[b].1, BLD[b].2)
            })
            .collect();
        s.push_str(&format!("Clădiri libere: {}\n", costs.join(" ")));
        for q in [Player::One, Player::Two] {
            let i = q.idx();
            let pl: Vec<String> = (0..5)
                .filter(|&c| self.plant[i][c] > 0)
                .map(|c| format!("{}{}", CROP_L[c], self.plant[i][c]))
                .collect();
            let gd: Vec<String> = (0..5)
                .filter(|&c| self.goods[i][c] > 0)
                .map(|c| format!("{}{}", CROP_L[c], self.goods[i][c]))
                .collect();
            let bl: Vec<&str> = (0..BLD.len())
                .filter(|&b| self.has(q, b))
                .map(|b| BLD[b].0)
                .collect();
            s.push_str(&format!(
                "J{}: {} puncte | monede {} | cariere {} | plantații [{}] | mărfuri [{}] | clădiri [{}] | expediat {}\n",
                i + 1,
                self.score(q),
                self.coin[i],
                self.quarry[i],
                pl.join(" "),
                gd.join(" "),
                bl.join(" "),
                self.ship_vp[i]
            ));
        }
        if !self.last.is_empty() {
            s.push_str(&format!("Ultima mutare: {}\n", self.last));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Pick(r) => format!("rol {}", ROLES[*r as usize]),
            Move::Plant(k) => format!("planta {}", k + 1),
            Move::Quarry => "cariera".into(),
            Move::Build(b) => format!("construieste {}", BLD[*b as usize].0),
            Move::Sell(c) => format!("vinde {}", CROP_L[*c as usize]),
            Move::Ship(c, s) => format!("expediaza {} {}", CROP_L[*c as usize], s + 1),
            Move::Pass => "sari".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(6)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polytabletop_core::rng::Rng;

    fn total_crops(g: &Mosia) -> usize {
        g.deck.len() + g.row.len() + g.plant.iter().flatten().map(|&x| x as usize).sum::<usize>()
    }

    #[test]
    fn start_state() {
        let g = Mosia::with_seed(1);
        assert_eq!(g.row.len(), 5);
        assert_eq!(g.legal_moves().len(), 7);
        assert_eq!(g.to_move(), Player::One);
        assert_eq!(total_crops(&g), 32); // 30 + 2 inițiale
    }

    #[test]
    fn roles_alternate_and_round_ends() {
        let mut g = Mosia::with_seed(2);
        let mut rng = Rng::new(7);
        let first_round = g.round;
        let mut starters = Vec::new();
        while g.round == first_round && g.outcome().is_none() {
            if g.phase == Phase::Pick {
                starters.push(g.to_move());
            }
            let m = g.legal_moves();
            g.apply(&m[rng.below(m.len())]);
        }
        assert_eq!(starters.len(), 6);
        assert_eq!(starters[0], Player::One);
        assert_eq!(starters[1], Player::Two);
        assert_eq!(g.first, Player::Two);
        assert_eq!(g.taken, 0);
        assert!(g.role_gold.iter().sum::<u8>() >= 1);
    }

    #[test]
    fn vistier_and_zidar() {
        let mut g = Mosia::with_seed(3);
        g.apply(&Move::Pick(5));
        assert_eq!(g.coin, [5, 4]);
        g.apply(&Move::Pick(6));
        assert_eq!(g.quarry[1], 1);
        assert_eq!(g.coin[1], 5);
    }

    #[test]
    fn production_and_cap() {
        let mut g = Mosia::with_seed(4);
        g.plant[0] = [3, 0, 0, 0, 1];
        g.blds[0] = 1; // moara
        g.apply(&Move::Pick(2));
        assert_eq!(g.goods[0][0], 4); // 3 + moară
        assert_eq!(g.goods[0][4], 2); // 1 + privilegiu (cea mai scumpă)
        assert_eq!(g.goods[1][1], 1);
        g.goods[0][0] = 6;
        let mut h = g.clone();
        h.taken = 0;
        h.picks = 0;
        h.phase = Phase::Pick;
        h.side = Player::One;
        h.apply(&Move::Pick(2));
        assert_eq!(h.goods[0][0], 6);
    }

    #[test]
    fn building_cost_discounts() {
        let mut g = Mosia::with_seed(5);
        g.coin = [10, 10];
        g.quarry[0] = 2;
        g.blds[0] = 1 << B_HALA;
        // moara: 2 - 2 (cariere) - 1 (picker) - 1 (hala) => 0
        assert_eq!(g.build_cost(Player::One, 0, true), 0);
        assert_eq!(g.build_cost(Player::One, 9, false), 8 - 2 - 1);
        assert_eq!(g.build_cost(Player::Two, 9, true), 7);
    }

    #[test]
    fn constructor_builds_once() {
        let mut g = Mosia::with_seed(6);
        g.coin = [10, 10];
        g.apply(&Move::Pick(1));
        assert!(g.legal_moves().contains(&Move::Build(9)));
        g.apply(&Move::Build(9));
        assert_eq!(g.coin[0], 10 - 7);
        assert_eq!(g.to_move(), Player::Two);
        assert!(!g.legal_moves().contains(&Move::Build(9)));
        assert_eq!(g.stock >> 9 & 1, 0);
    }

    #[test]
    fn cultivator_takes_plantation_and_refills() {
        let mut g = Mosia::with_seed(7);
        g.apply(&Move::Pick(0));
        g.apply(&Move::Plant(0));
        g.apply(&Move::Pass);
        assert_eq!(g.row.len(), 5);
        assert_eq!(g.plantations(Player::One), 2);
        assert_eq!(total_crops(&g), 32);
    }

    #[test]
    fn selling_rules() {
        let mut g = Mosia::with_seed(8);
        g.goods[0] = [0, 0, 2, 0, 0];
        g.goods[1] = [0, 0, 1, 0, 0];
        g.apply(&Move::Pick(3));
        g.apply(&Move::Sell(2));
        assert_eq!(g.coin[0], 3 + 2 + 1);
        // celălalt nu poate vinde același fel
        assert_eq!(g.phase, Phase::Pick);
        assert_eq!(g.coin[1], 3);
    }

    #[test]
    fn shipping_rules() {
        let mut g = Mosia::with_seed(9);
        g.goods[0] = [0, 0, 5, 0, 0];
        g.goods[1] = [0, 0, 0, 2, 0];
        g.apply(&Move::Pick(4));
        let m = g.legal_moves();
        assert!(m.contains(&Move::Ship(2, 0)) && m.contains(&Move::Ship(2, 1)));
        g.apply(&Move::Ship(2, 0)); // 4 pe nava mică
        assert_eq!(g.ship_vp[0], 4);
        assert_eq!(g.goods[0][2], 1);
        // J2 poate pune tutun doar pe nava 2
        let m = g.legal_moves();
        assert_eq!(g.to_move(), Player::Two);
        assert_eq!(m, vec![Move::Ship(3, 1)]);
        g.apply(&Move::Ship(3, 1));
        assert_eq!(g.ship_vp[1], 2);
        // J1 mai are un zahăr, dar nava 1 e plină, nava 2 are tutun => nu poate
        assert_eq!(g.phase, Phase::Pick);
        assert_eq!(g.ships[0], (-1, 0)); // nava plină s-a golit
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=20u64 {
            let mut g = Mosia::with_seed(seed);
            let mut rng = Rng::new(seed ^ 0x55);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 3000);
                assert_eq!(total_crops(&g), 32);
                for p in [Player::One, Player::Two] {
                    assert!(g.plantations(p) <= MAX_PLANT);
                    assert!(g.coin[p.idx()] >= 0);
                }
            }
        }
    }

    #[test]
    fn ai_beats_random_mostly() {
        let (mut ai, mut rnd) = (0, 0);
        for seed in 1..=8u64 {
            let ai_side = if seed % 2 == 0 {
                Player::One
            } else {
                Player::Two
            };
            let mut g = Mosia::with_seed(seed);
            let mut rng = Rng::new(seed * 3);
            while g.outcome().is_none() {
                let m = if g.to_move() == ai_side {
                    polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap()
                } else {
                    let l = g.legal_moves();
                    l[rng.below(l.len())]
                };
                g.apply(&m);
            }
            match g.outcome() {
                Some(Outcome::Win(p)) if p == ai_side => ai += 1,
                Some(Outcome::Win(_)) => rnd += 1,
                _ => {}
            }
        }
        assert!(ai > rnd, "AI {ai} vs random {rnd}");
    }
}
