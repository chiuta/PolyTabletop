//! Furnale — joc original de rețea de legături și industrii (2 jucători).
//!
//! Mecanica „construiești industrii în orașe, legături între ele, resursele
//! (cărbune, fier) se iau din rețea sau din piață, plăcuțele se întorc când
//! s-au epuizat sau s-au vândut și dau venit și puncte, iar legăturile
//! punctează după orașele lor” e generică; harta, industriile, cifrele și
//! textele sunt ale noastre. Informație completă, fără zaruri.
//!
//! Simplificări: o singură eră, fără cărți (acțiunile sunt libere), fără
//! dezvoltare de plăcuțe, piață de resurse cu preț fix (2 bani), o singură
//! treaptă pe industrie.
//!
//! Reguli:
//! * 8 orașe (`0`–`7`) și 12 legături posibile; fiecare oraș are două locuri
//!   pentru industrii, de anumite feluri (`c` cărbune, `f` fier, `t` textil, `p`
//!   port); vezi tabla.
//! * 8 runde; în fiecare rundă fiecare joacă 2 acțiuni (începe alternativ).
//!   La sfârșitul rundei primești 3 bani + venitul plăcuțelor întoarse.
//!   Pornești cu 12 bani.
//! * Acțiuni:
//!   - `fabrica 3 t`: construiești o industrie în orașul 3. Costuri (bani +
//!     resurse): cărbune 3 (2 cuburi de cărbune pe plăcuță), fier 4 + 1 cărbune
//!     (2 cuburi de fier), textil 5 + 1 cărbune + 1 fier, port 3. Trebuie să
//!     fie în rețeaua ta (oraș cu industria ta sau capăt al unei legături a
//!     tale); prima ta acțiune e liberă.
//!   - `link 2-4`: construiești o legătură (3 bani + 1 cărbune), lipită de
//!     rețeaua ta (prima, liberă).
//!   - `vinde 4`: vinzi textilul tău din orașul 4 dacă orașul e legat (prin
//!     legături ale oricui) de un port; plăcuța se întoarce; portul, dacă nu era
//!     întors, se întoarce și el (beneficiază proprietarul).
//!   - `imprumut`: +8 bani, −2 puncte la final (cel mult 3 împrumuturi);
//!   - `pas`.
//! * Cărbunele vine din cea mai apropiată mină legată de oraș prin legături
//!   (indiferent de proprietar), altfel din piață (2 bani). Fierul vine din
//!   orice fierărie cu cuburi, altfel din piață (2 bani). Când ultimul cub se
//!   ia, plăcuța se întoarce pentru proprietar.
//! * Plăcuță întoarsă: venit (cărbune +2, fier +1, textil +2, port +2) și
//!   puncte (1, 2, 4, 3).
//! * Final: puncte din plăcuțe întoarse + pentru fiecare legătură a ta câte
//!   un punct pe plăcuță întoarsă din cele două orașe ale ei + bani / 5 −
//!   2 pe împrumut.

#![allow(clippy::needless_range_loop)]

use polytabletop_core::{AiStyle, Game, Outcome, Player};

type Src = Vec<(usize, usize)>;
const NT: usize = 8;
const TOWN: [&str; NT] = [
    "Arieș", "Bistra", "Cetate", "Dobra", "Eder", "Fânaț", "Gorun", "Hațeg",
];
const LINKS: [(u8, u8); 12] = [
    (0, 1),
    (0, 2),
    (1, 2),
    (1, 3),
    (2, 3),
    (2, 4),
    (3, 4),
    (3, 5),
    (4, 6),
    (5, 6),
    (5, 7),
    (6, 7),
];
const C: u8 = 0;
const F: u8 = 1;
const T: u8 = 2;
const P: u8 = 3;
const TYPE_L: [char; 4] = ['c', 'f', 't', 'p'];
const SLOTS: [[u8; 2]; NT] = [
    [C, T],
    [F, T],
    [C, F],
    [T, P],
    [C, T],
    [F, P],
    [C, T],
    [T, P],
];
/// (cost bani, cărbune, fier, cuburi, venit, puncte)
const SPEC: [(i32, i32, i32, u8, i32, i32); 4] = [
    (3, 0, 0, 2, 2, 1),
    (4, 1, 0, 2, 1, 2),
    (5, 1, 1, 0, 2, 4),
    (3, 0, 0, 0, 2, 3),
];
const ROUNDS: u8 = 8;
const MARKET: i32 = 2;
const LINK_COST: i32 = 3;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Build(u8, u8),
    Link(u8),
    Sell(u8),
    Loan,
    Pass,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Tile {
    owner: Player,
    cubes: u8,
    flipped: bool,
}

#[derive(Clone)]
pub struct Furnale {
    /// tile[oraș][loc]
    tile: [[Option<Tile>; 2]; NT],
    link: [Option<Player>; 12],
    money: [i32; 2],
    loans: [i32; 2],
    income: [i32; 2],
    vp: [i32; 2],
    side: Player,
    starter: Player,
    actions_left: u8,
    round: u8,
    outcome: Option<Outcome>,
    last: String,
}

impl Default for Furnale {
    fn default() -> Self {
        Self::new()
    }
}

impl Furnale {
    pub fn new() -> Self {
        Self::with_seed(polytabletop_core::rng::Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let starter = if polytabletop_core::rng::Rng::new(seed).below(2) == 0 {
            Player::One
        } else {
            Player::Two
        };
        Furnale {
            tile: [[None; 2]; NT],
            link: [None; 12],
            money: [12, 12],
            loans: [0; 2],
            income: [0; 2],
            vp: [0; 2],
            side: starter,
            starter,
            actions_left: 2,
            round: 1,
            outcome: None,
            last: String::new(),
        }
    }

    fn adj(&self, t: usize) -> Vec<usize> {
        let mut v = Vec::new();
        for (e, &(a, b)) in LINKS.iter().enumerate() {
            if self.link[e].is_some() {
                if a as usize == t {
                    v.push(b as usize);
                } else if b as usize == t {
                    v.push(a as usize);
                }
            }
        }
        v
    }

    /// Orașe în ordinea distanței (prin legături construite) de la `starts`.
    fn bfs_order(&self, starts: &[usize]) -> Vec<usize> {
        let mut dist = [usize::MAX; NT];
        let mut q = std::collections::VecDeque::new();
        for &s in starts {
            if dist[s] == usize::MAX {
                dist[s] = 0;
                q.push_back(s);
            }
        }
        while let Some(t) = q.pop_front() {
            for n in self.adj(t) {
                if dist[n] == usize::MAX {
                    dist[n] = dist[t] + 1;
                    q.push_back(n);
                }
            }
        }
        let mut v: Vec<usize> = (0..NT).filter(|&t| dist[t] != usize::MAX).collect();
        v.sort_by_key(|&t| (dist[t], t));
        v
    }

    fn connected_to_port(&self, t: usize) -> bool {
        self.bfs_order(&[t]).iter().any(|&x| self.has_port(x))
    }

    fn has_port(&self, t: usize) -> bool {
        (0..2).any(|s| SLOTS[t][s] == P && self.tile[t][s].is_some())
    }

    /// Plan de cărbune: (loc de unde se ia, câte cumpărate din piață)
    fn plan_coal(&self, starts: &[usize], n: i32) -> (Src, i32) {
        let mut taken: Vec<(usize, usize)> = Vec::new();
        let mut need = n;
        for t in self.bfs_order(starts) {
            for s in 0..2 {
                if need == 0 {
                    break;
                }
                if SLOTS[t][s] == C {
                    if let Some(tl) = self.tile[t][s] {
                        let already = taken.iter().filter(|&&x| x == (t, s)).count() as u8;
                        let mut left = tl.cubes.saturating_sub(already);
                        while left > 0 && need > 0 {
                            taken.push((t, s));
                            left -= 1;
                            need -= 1;
                        }
                    }
                }
            }
        }
        (taken, need)
    }

    fn plan_iron(&self, n: i32) -> (Src, i32) {
        let mut taken = Vec::new();
        let mut need = n;
        for t in 0..NT {
            for s in 0..2 {
                if SLOTS[t][s] == F {
                    if let Some(tl) = self.tile[t][s] {
                        let mut left = tl.cubes;
                        while left > 0 && need > 0 {
                            taken.push((t, s));
                            left -= 1;
                            need -= 1;
                        }
                    }
                }
            }
        }
        (taken, need)
    }

    fn has_network(&self, p: Player) -> bool {
        self.tile.iter().flatten().flatten().any(|t| t.owner == p) || self.link.contains(&Some(p))
    }

    fn in_network(&self, p: Player, t: usize) -> bool {
        if !self.has_network(p) {
            return true;
        }
        self.tile[t].iter().flatten().any(|x| x.owner == p)
            || LINKS
                .iter()
                .enumerate()
                .any(|(e, &(a, b))| self.link[e] == Some(p) && (a as usize == t || b as usize == t))
    }

    fn build_cost(&self, p: Player, town: usize, ty: u8) -> Option<(i32, Src, Src)> {
        let sp = SPEC[ty as usize];
        let (coal, mc) = self.plan_coal(&[town], sp.1);
        let (iron, mi) = self.plan_iron(sp.2);
        let total = sp.0 + MARKET * (mc + mi);
        if self.money[p.idx()] >= total {
            Some((total, coal, iron))
        } else {
            None
        }
    }

    fn flip(&mut self, t: usize, s: usize) {
        let tl = self.tile[t][s].as_mut().expect("plăcuță");
        if tl.flipped {
            return;
        }
        tl.flipped = true;
        let o = tl.owner.idx();
        let sp = SPEC[SLOTS[t][s] as usize];
        self.income[o] += sp.4;
        self.vp[o] += sp.5;
    }

    fn consume(&mut self, list: &[(usize, usize)]) {
        for &(t, s) in list {
            let empty = {
                let tl = self.tile[t][s].as_mut().expect("plăcuță");
                tl.cubes -= 1;
                tl.cubes == 0
            };
            if empty {
                self.flip(t, s);
            }
        }
    }

    pub fn score(&self, p: Player) -> i32 {
        let i = p.idx();
        let mut v = self.vp[i] - 2 * self.loans[i] + self.money[i] / 5;
        for (e, &(a, b)) in LINKS.iter().enumerate() {
            if self.link[e] == Some(p) {
                for t in [a as usize, b as usize] {
                    v += self.tile[t].iter().flatten().filter(|x| x.flipped).count() as i32;
                }
            }
        }
        v
    }

    fn end_action(&mut self) {
        self.actions_left -= 1;
        if self.actions_left > 0 {
            return;
        }
        // schimbă jucătorul sau termină runda
        let other = self.side.other();
        if self.side == self.starter {
            self.side = other;
            self.actions_left = 2;
            return;
        }
        for i in 0..2 {
            self.money[i] += 3 + self.income[i];
        }
        if self.round >= ROUNDS {
            let (a, b) = (self.score(Player::One), self.score(Player::Two));
            self.outcome = Some(if a > b {
                Outcome::Win(Player::One)
            } else if b > a {
                Outcome::Win(Player::Two)
            } else {
                Outcome::Draw
            });
            return;
        }
        self.round += 1;
        self.starter = self.starter.other();
        self.side = self.starter;
        self.actions_left = 2;
    }
}

impl Game for Furnale {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome.is_some() {
            return Vec::new();
        }
        let p = self.side;
        let mut v = Vec::new();
        for t in 0..NT {
            for s in 0..2 {
                if self.tile[t][s].is_none()
                    && self.in_network(p, t)
                    && self.build_cost(p, t, SLOTS[t][s]).is_some()
                {
                    v.push(Move::Build(t as u8, SLOTS[t][s]));
                }
            }
        }
        for (e, &(a, b)) in LINKS.iter().enumerate() {
            if self.link[e].is_some() {
                continue;
            }
            if !(self.in_network(p, a as usize) || self.in_network(p, b as usize)) {
                continue;
            }
            let (_, mc) = self.plan_coal(&[a as usize, b as usize], 1);
            if self.money[p.idx()] >= LINK_COST + MARKET * mc {
                v.push(Move::Link(e as u8));
            }
        }
        for t in 0..NT {
            for s in 0..2 {
                if let Some(tl) = self.tile[t][s] {
                    if SLOTS[t][s] == T && tl.owner == p && !tl.flipped && self.connected_to_port(t)
                    {
                        v.push(Move::Sell(t as u8));
                        break;
                    }
                }
            }
        }
        if self.loans[p.idx()] < 3 {
            v.push(Move::Loan);
        }
        v.push(Move::Pass);
        v
    }

    fn apply(&mut self, m: &Move) {
        let p = self.side;
        let i = p.idx();
        match *m {
            Move::Build(t, ty) => {
                let t = t as usize;
                let (cost, coal, iron) =
                    self.build_cost(p, t, ty).expect("costul a fost verificat");
                let slot = (0..2)
                    .find(|&s| SLOTS[t][s] == ty && self.tile[t][s].is_none())
                    .expect("loc liber");
                self.money[i] -= cost;
                self.consume(&coal);
                self.consume(&iron);
                self.tile[t][slot] = Some(Tile {
                    owner: p,
                    cubes: SPEC[ty as usize].3,
                    flipped: false,
                });
                self.last = format!(
                    "J{} construiește {} în {}",
                    i + 1,
                    TYPE_L[ty as usize],
                    TOWN[t]
                );
            }
            Move::Link(e) => {
                let (a, b) = LINKS[e as usize];
                let (coal, mc) = self.plan_coal(&[a as usize, b as usize], 1);
                self.money[i] -= LINK_COST + MARKET * mc;
                self.consume(&coal);
                self.link[e as usize] = Some(p);
                self.last = format!("J{} leagă {}–{}", i + 1, TOWN[a as usize], TOWN[b as usize]);
            }
            Move::Sell(t) => {
                let t = t as usize;
                if let Some(s) = (0..2).find(|&s| {
                    SLOTS[t][s] == T && self.tile[t][s].is_some_and(|x| x.owner == p && !x.flipped)
                }) {
                    self.flip(t, s);
                }
                // portul cel mai apropiat se întoarce, dacă nu era
                let order = self.bfs_order(&[t]);
                'port: for x in order {
                    for s in 0..2 {
                        if SLOTS[x][s] == P {
                            if let Some(tl) = self.tile[x][s] {
                                if !tl.flipped {
                                    self.flip(x, s);
                                }
                                break 'port;
                            }
                        }
                    }
                }
                self.last = format!("J{} vinde textil din {}", i + 1, TOWN[t]);
            }
            Move::Loan => {
                self.money[i] += 8;
                self.loans[i] += 1;
                self.last = format!("J{} ia un împrumut", i + 1);
            }
            Move::Pass => {
                self.last = format!("J{} dă pas", i + 1);
            }
        }
        self.end_action();
    }

    fn outcome(&self) -> Option<Outcome> {
        self.outcome
    }

    fn evaluate(&self, p: Player) -> i32 {
        let side = |q: Player| -> i32 {
            let i = q.idx();
            let mut v = self.score(q) * 100 + self.money[i] * 8 + self.income[i] * 90;
            for t in 0..NT {
                for s in 0..2 {
                    if let Some(tl) = self.tile[t][s] {
                        if tl.owner == q && !tl.flipped {
                            let sp = SPEC[SLOTS[t][s] as usize];
                            v += sp.5 * 40 + sp.4 * 30 + tl.cubes as i32 * 10;
                        }
                    }
                }
            }
            v
        };
        side(p) - side(p.other())
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Furnale — runda {}/{} | acțiuni rămase în tura asta: {}\n",
            self.round, ROUNDS, self.actions_left
        );
        s.push_str("Orașe (loc: tip și plăcuță; [c cărbune, f fier, t textil, p port]; * = întoarsă; cuburi între paranteze):\n");
        for t in 0..NT {
            let mut cells = Vec::new();
            for sl in 0..2 {
                let ty = TYPE_L[SLOTS[t][sl] as usize];
                cells.push(match self.tile[t][sl] {
                    None => format!("{ty}:liber"),
                    Some(tl) => format!(
                        "{ty}:J{}{}{}",
                        tl.owner.idx() + 1,
                        if tl.flipped { "*" } else { "" },
                        if tl.cubes > 0 {
                            format!("({})", tl.cubes)
                        } else {
                            String::new()
                        }
                    ),
                });
            }
            s.push_str(&format!("  {t} {:<7} {}\n", TOWN[t], cells.join("  ")));
        }
        let links: Vec<String> = LINKS
            .iter()
            .enumerate()
            .map(|(e, &(a, b))| match self.link[e] {
                Some(p) => format!("{a}-{b}:J{}", p.idx() + 1),
                None => format!("{a}-{b}"),
            })
            .collect();
        s.push_str(&format!("Legături: {}\n", links.join(" ")));
        for q in [Player::One, Player::Two] {
            let i = q.idx();
            s.push_str(&format!(
                "J{}: {} puncte | bani {} | venit {}+3 | împrumuturi {}\n",
                i + 1,
                self.score(q),
                self.money[i],
                self.income[i],
                self.loans[i]
            ));
        }
        if !self.last.is_empty() {
            s.push_str(&format!("Ultima mutare: {}\n", self.last));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match *m {
            Move::Build(t, ty) => format!("fabrica {t} {}", TYPE_L[ty as usize]),
            Move::Link(e) => {
                let (a, b) = LINKS[e as usize];
                format!("link {a}-{b}")
            }
            Move::Sell(t) => format!("vinde {t}"),
            Move::Loan => "imprumut".into(),
            Move::Pass => "pas".into(),
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

    fn mv(g: &mut Furnale, s: &str) {
        let m = g
            .legal_moves()
            .into_iter()
            .find(|m| g.format_move(m) == s)
            .unwrap_or_else(|| panic!("ilegal: {s}\n{}", g.render()));
        g.apply(&m);
    }

    #[test]
    fn start_is_open_anywhere() {
        let g = Furnale::with_seed(1);
        let m = g.legal_moves();
        assert!(m.contains(&Move::Build(0, C)));
        assert!(m.contains(&Move::Build(3, P)));
        assert!(!m.contains(&Move::Build(0, F))); // fără fier în Arieș
                                                  // textilul cere cărbune și fier din piață: 5 + 2 + 2 = 9 ≤ 12
        assert!(m.contains(&Move::Build(0, T)));
        assert!(m.contains(&Move::Loan));
    }

    #[test]
    fn two_actions_each_then_round_income() {
        let mut g = Furnale::with_seed(2);
        let a = g.to_move();
        mv(&mut g, "pas");
        assert_eq!(g.to_move(), a);
        mv(&mut g, "pas");
        assert_eq!(g.to_move(), a.other());
        mv(&mut g, "pas");
        mv(&mut g, "pas");
        assert_eq!(g.round, 2);
        assert_eq!(g.money, [15, 15]);
        assert_eq!(g.to_move(), a.other());
    }

    #[test]
    fn coal_mine_and_network_rule() {
        let mut g = Furnale::with_seed(3);
        let a = g.to_move();
        mv(&mut g, "fabrica 0 c");
        assert_eq!(g.money[a.idx()], 12 - 3);
        assert_eq!(g.tile[0][0].unwrap().cubes, 2);
        // a doua acțiune: doar în rețea (orașul 0) sau legături lipite de 0
        let m = g.legal_moves();
        assert!(m.contains(&Move::Build(0, T)));
        assert!(!m.contains(&Move::Build(3, P)));
        assert!(m.contains(&Move::Link(0))); // 0-1
        assert!(!m.contains(&Move::Link(11))); // 6-7 departe
    }

    #[test]
    fn link_uses_connected_coal_then_flips_mine() {
        let mut g = Furnale::with_seed(4);
        let a = g.to_move();
        g.tile[0][0] = Some(Tile {
            owner: a.other(),
            cubes: 1,
            flipped: false,
        });
        g.side = a;
        mv(&mut g, "link 0-1");
        // cărbunele din mina lui 0 (1 cub) -> mina se întoarce pentru adversar
        let o = a.other();
        assert!(g.tile[0][0].unwrap().flipped);
        assert_eq!(g.income[o.idx()], 2);
        assert_eq!(g.vp[o.idx()], 1);
        assert_eq!(g.money[a.idx()], 12 - 3);
    }

    #[test]
    fn sell_requires_port_and_flips_both() {
        let mut g = Furnale::with_seed(5);
        let a = g.to_move();
        let o = a.other();
        g.tile[1][1] = Some(Tile {
            owner: a,
            cubes: 0,
            flipped: false,
        });
        g.tile[3][1] = Some(Tile {
            owner: o,
            cubes: 0,
            flipped: false,
        });
        assert!(!g.legal_moves().contains(&Move::Sell(1)));
        g.link[3] = Some(Player::One); // 1-3
        g.side = a;
        assert!(g.legal_moves().contains(&Move::Sell(1)));
        mv(&mut g, "vinde 1");
        assert_eq!(g.vp[a.idx()], 4);
        assert_eq!(g.income[a.idx()], 2);
        assert!(g.tile[3][1].unwrap().flipped);
        assert_eq!(g.vp[o.idx()], 3);
    }

    #[test]
    fn loans_limit_and_penalty() {
        let mut g = Furnale::with_seed(6);
        let a = g.to_move();
        g.side = a;
        g.actions_left = 2;
        for _ in 0..3 {
            g.side = a;
            g.actions_left = 2;
            mv(&mut g, "imprumut");
        }
        g.side = a;
        g.actions_left = 2;
        assert!(!g.legal_moves().contains(&Move::Loan));
        assert_eq!(g.money[a.idx()], 12 + 24);
        assert_eq!(g.score(a), -6 + 36 / 5);
    }

    #[test]
    fn link_scores_by_flipped_tiles() {
        let mut g = Furnale::with_seed(7);
        g.link[0] = Some(Player::One); // 0-1
        g.tile[0][0] = Some(Tile {
            owner: Player::Two,
            cubes: 0,
            flipped: true,
        });
        g.tile[1][0] = Some(Tile {
            owner: Player::Two,
            cubes: 0,
            flipped: true,
        });
        g.money = [0, 0];
        assert_eq!(g.score(Player::One), 2);
        assert_eq!(g.score(Player::Two), 0);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=30u64 {
            let mut g = Furnale::with_seed(seed);
            let mut rng = Rng::new(seed ^ 0x17);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 100);
                assert!(g.money.iter().all(|&x| x >= 0), "{}", g.render());
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
            let mut g = Furnale::with_seed(seed);
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
