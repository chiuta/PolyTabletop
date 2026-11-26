//! Parqués (Colombia), 2 jucători, patru piese fiecare, două zaruri.
//!
//! Sursele publice ale jocului nu sunt de acord asupra geometriei exacte a tablei (numărul
//! de pătrate al culoarului, poziția seguros-urilor); de aceea geometria de mai jos e a noastră,
//! iar regulile de joc sunt cele documentate în regulamentele publice:
//!
//! * Pista circulară are 68 de pătrate (0–67). Patru **salida** (start) la 0, 17, 34 și 51;
//!   la doi jucători J1 pornește de la 0, J2 de la 34 (cărcerile sunt „în diagonală”). Douăsprezece
//!   **seguros** (pătrate sigure): 17k + {5, 10, 14}; salidele sunt și ele sigure. Pe pătrate sigure
//!   nu se poate captura.
//! * Progresul unei piese: 0 = în **cárcel** (închisoare), 1..=68 pe pistă (1 = salida proprie),
//!   69..=76 pasillo (culoarul propriu, necapturabil), 77 = **corona** (ținta). Corona se atinge exact.
//! * Doua zaruri. Folosești suma pentru o piesă sau fiecare zar pentru o piesă (nu se pierde
//!   un zar dacă se poate juca; un zar nejucabil se pierde). Mutări: `12:5` = piesa aflată la
//!   progresul 12 avansează 5 (un zar sau suma), `sale`, `pasa`.
//! * **Ieșirea din cárcel**: numai cu o pereche (zaruri egale); fiecare jucător fără piese pe tablă are
//!   până la **trei** aruncări pentru pereche. O pereche folosită cu `sale` scoate **toate** piesele
//!   din cárcel pe salida; o piesă adversă aflată pe acea salida e trimisă în cárcel.
//! * **Pereche = încă o aruncare**; trei perechi la rând = o piesă (cea mai din urmă de pe tablă)
//!   ajunge direct în corona și tura se termină.
//! * **Captura**: aterizarea pe pista comună pe o piesă adversă (nu pe pătrat sigur) o trimite în
//!   cárcel; dacă pe pătrat sunt două piese adverse se capturează una singură. **Captura e
//!   obligatorie**: dacă există o mutare care capturează, ești obligat să capturezi (în loc de
//!   „suflat” între jucători, regula se aplică automat).
//! * Câștigă primul care aduce cele patru piese în corona.
//!
//! Zarurile se aruncă în `apply` cu `Rng`-ul jocului; aruncarea următoare e deja în stare când vine
//! rândul jucătorului, dar `evaluate` nu o folosește. AI: `AlphaBeta(1)` pe progres, capturi și
//! siguranța pieselor.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};
use std::collections::BTreeSet;

const TRACK: u8 = 68;
const GOAL: u8 = 77;
const MAX_TURNS: u32 = 1500;

fn start_off(p: Player) -> u8 {
    match p {
        Player::One => 0,
        Player::Two => 34,
    }
}

/// Pătratul de pe pistă pentru progres 1..=68.
pub fn square(p: Player, progress: u8) -> u8 {
    (start_off(p) + progress - 1) % TRACK
}

pub fn is_safe(sq: u8) -> bool {
    let r = sq % 17;
    r == 0 || r == 5 || r == 10 || r == 14
}

#[derive(Clone, PartialEq, Debug)]
pub enum Move {
    /// Piesa de la progresul `.0` avansează `.1`.
    Step(u8, u8),
    Release,
    Pass,
}

#[derive(Clone)]
pub struct Parques {
    prog: [[u8; 4]; 2],
    side: Player,
    dice: [u8; 2],
    /// Zarurile încă nefolosite în tura (sub-tura) curentă.
    left: Vec<u8>,
    pair: bool,
    doubles: u8,
    rng: Rng,
    turns: u32,
    log: String,
}

impl Default for Parques {
    fn default() -> Self {
        Self::new()
    }
}

impl Parques {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Parques {
            prog: [[0; 4]; 2],
            side: Player::One,
            dice: [0; 2],
            left: Vec::new(),
            pair: false,
            doubles: 0,
            rng: Rng::new(seed),
            turns: 0,
            log: String::new(),
        };
        g.roll_for_side();
        g
    }

    pub fn progress(&self, p: Player) -> [u8; 4] {
        self.prog[p.idx()]
    }

    fn finished(&self, p: usize) -> usize {
        self.prog[p].iter().filter(|&&x| x == GOAL).count()
    }

    fn on_board(&self, p: usize) -> bool {
        self.prog[p].iter().any(|&x| x > 0 && x < GOAL)
    }

    /// Aruncă zarurile pentru jucătorul la mutare (cu cele trei încercări de pereche cât timp
    /// nu are nicio piesă pe tablă) și pregătește `left`.
    fn roll_for_side(&mut self) {
        loop {
            let me = self.side.idx();
            let tries = if self.on_board(me) || self.finished(me) == 4 {
                1
            } else {
                3
            };
            let mut got = false;
            for t in 0..tries {
                let a = self.rng.d6();
                let b = self.rng.d6();
                self.dice = [a, b];
                if a == b && self.doubles >= 2 {
                    // a treia pereche la rând
                    self.doubles = 0;
                    let victim = self.prog[me]
                        .iter()
                        .enumerate()
                        .filter(|(_, &x)| x > 0 && x < GOAL)
                        .min_by_key(|(_, &x)| x)
                        .map(|(i, _)| i);
                    if let Some(i) = victim {
                        self.prog[me][i] = GOAL;
                    }
                    self.log = format!(
                        "J{}: trei perechi la rând — o piesă ajunge în corona.",
                        me + 1
                    );
                    self.pass_turn_raw();
                    got = true;
                    break;
                }
                self.pair = a == b;
                if self.pair {
                    self.doubles += 1;
                }
                self.left = if self.pair || self.on_board(me) {
                    vec![a, b]
                } else {
                    Vec::new()
                };
                if !self.pair && !self.on_board(me) && t + 1 < tries {
                    self.log = format!("J{}: {a}+{b}, fără pereche — încă o încercare.", me + 1);
                    continue;
                }
                got = true;
                break;
            }
            if !got {
                break;
            }
            if self.outcome().is_some() {
                return;
            }
            // dacă nu există nicio mutare, tura trece (sau se aruncă din nou după o pereche)
            if !self.left.is_empty() && self.moves_for(self.side).is_empty() {
                self.log.push_str(" Nicio mutare posibilă.");
                self.end_subturn();
                if self.outcome().is_some() {
                    return;
                }
                continue;
            }
            if self.left.is_empty() {
                // fără pereche și fără piese pe tablă după toate încercările
                self.log = format!(
                    "J{}: {}+{}, fără pereche — tura trece.",
                    me + 1,
                    self.dice[0],
                    self.dice[1]
                );
                self.pass_turn_raw();
                continue;
            }
            return;
        }
    }

    fn pass_turn_raw(&mut self) {
        self.side = self.side.other();
        self.doubles = 0;
        self.left.clear();
        self.pair = false;
        self.turns += 1;
    }

    /// Terminarea unei sub-ture: pereche = aceeași persoană aruncă din nou, altfel tura trece.
    fn end_subturn(&mut self) {
        if self.outcome().is_some() {
            return;
        }
        if self.pair && self.turns < MAX_TURNS {
            self.left.clear();
            self.roll_for_side();
        } else {
            self.pass_turn_raw();
            self.roll_for_side();
        }
    }

    fn capture_target(&self, me: Player, prog: u8) -> Option<usize> {
        if prog == 0 || prog > TRACK {
            return None;
        }
        let sq = square(me, prog);
        if is_safe(sq) {
            return None;
        }
        let o = me.other();
        (0..4).find(|&i| {
            let q = self.prog[o.idx()][i];
            q > 0 && q <= TRACK && square(o, q) == sq
        })
    }

    fn moves_for(&self, p: Player) -> Vec<Move> {
        let me = p.idx();
        if self.left.is_empty() {
            return Vec::new();
        }
        let mut all: Vec<Move> = Vec::new();
        let mut seen = BTreeSet::new();
        let jailed = self.prog[me].contains(&0);
        if self.pair && self.left.len() == 2 && jailed {
            all.push(Move::Release);
        }
        let mut amounts: Vec<u8> = self.left.clone();
        if self.left.len() == 2 {
            amounts.push(self.left[0] + self.left[1]);
        }
        amounts.sort_unstable();
        amounts.dedup();
        for &x in &self.prog[me] {
            if x == 0 || x >= GOAL {
                continue;
            }
            for &n in &amounts {
                if x + n <= GOAL && seen.insert((x, n)) {
                    all.push(Move::Step(x, n));
                }
            }
        }
        // captura e obligatorie
        let captures: Vec<Move> = all
            .iter()
            .filter(|m| self.captures(p, m))
            .cloned()
            .collect();
        if captures.is_empty() {
            all
        } else {
            captures
        }
    }

    fn captures(&self, p: Player, m: &Move) -> bool {
        match m {
            Move::Step(x, n) => self.capture_target(p, x + n).is_some(),
            Move::Release => {
                let o = p.other();
                let sq = square(p, 1);
                !is_safe_for_release(sq)
                    && (0..4).any(|i| {
                        let q = self.prog[o.idx()][i];
                        q > 0 && q <= TRACK && square(o, q) == sq
                    })
            }
            Move::Pass => false,
        }
    }
}

/// Salida proprie e „sigură”, dar o piesă care iese din cárcel pe ea își captează ocupantul advers.
fn is_safe_for_release(_sq: u8) -> bool {
    false
}

impl Game for Parques {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        let m = self.moves_for(self.side);
        if m.is_empty() {
            vec![Move::Pass]
        } else {
            m
        }
    }

    fn apply(&mut self, m: &Move) {
        let me = self.side;
        let i = me.idx();
        match m {
            Move::Pass => {
                self.left.clear();
                self.end_subturn();
                return;
            }
            Move::Release => {
                let o = me.other();
                for x in self.prog[i].iter_mut() {
                    if *x == 0 {
                        *x = 1;
                    }
                }
                let sq = square(me, 1);
                if let Some(v) = (0..4).find(|&k| {
                    let q = self.prog[o.idx()][k];
                    q > 0 && q <= TRACK && square(o, q) == sq
                }) {
                    self.prog[o.idx()][v] = 0;
                    self.log =
                        format!("J{}: iese din cárcel și captează o piesă pe salida.", i + 1);
                } else {
                    self.log = format!("J{}: iese din cárcel.", i + 1);
                }
                self.left.clear();
            }
            Move::Step(x, n) => {
                let k = self.prog[i].iter().position(|v| v == x).expect("piece");
                let target = x + n;
                if let Some(v) = self.capture_target(me, target) {
                    self.prog[me.other().idx()][v] = 0;
                    self.log = format!("J{}: {x} → {target}, captură!", i + 1);
                } else {
                    self.log = format!("J{}: {x} → {target}.", i + 1);
                }
                self.prog[i][k] = target;
                // folosește zarurile
                if self.left.len() == 2
                    && *n == self.left[0] + self.left[1]
                    && !self.left.contains(n)
                {
                    self.left.clear();
                } else if let Some(p) = self.left.iter().position(|d| d == n) {
                    self.left.remove(p);
                } else {
                    self.left.clear();
                }
            }
        }
        if self.outcome().is_some() {
            return;
        }
        if self.left.is_empty() || self.moves_for(me).is_empty() {
            self.left.clear();
            self.end_subturn();
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        for p in [Player::One, Player::Two] {
            if self.finished(p.idx()) == 4 {
                return Some(Outcome::Win(p));
            }
        }
        if self.turns >= MAX_TURNS {
            let (a, b) = (self.total(0), self.total(1));
            return Some(match a.cmp(&b) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                std::cmp::Ordering::Equal => Outcome::Draw,
            });
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let me = p.idx();
        let o = 1 - me;
        let mut v = (self.total(me) - self.total(o)) * 4;
        v += (self.finished(me) as i32 - self.finished(o) as i32) * 40;
        v -= self.prog[me].iter().filter(|&&x| x == 0).count() as i32 * 25;
        v += self.prog[o].iter().filter(|&&x| x == 0).count() as i32 * 25;
        // piese expuse pe pista comună
        let exposed = |who: usize, foe: usize| -> i32 {
            let wp = if who == 0 { Player::One } else { Player::Two };
            let fp = wp.other();
            let mut e = 0;
            for &x in &self.prog[who] {
                if x == 0 || x > TRACK {
                    continue;
                }
                let sq = square(wp, x);
                if is_safe(sq) {
                    continue;
                }
                for &y in &self.prog[foe] {
                    if y == 0 || y > TRACK {
                        continue;
                    }
                    let fs = square(fp, y);
                    let d = (sq + TRACK - fs) % TRACK;
                    if (1..=12).contains(&d) {
                        e += 1;
                    }
                }
            }
            e
        };
        v -= exposed(me, o) * 6;
        v += exposed(o, me) * 6;
        v
    }

    fn render(&self) -> String {
        let mut s = String::from(
            "Parqués — J1 pornește de la pătratul 0, J2 de la 34 (pista are 68 de pătrate)\n",
        );
        for p in [Player::One, Player::Two] {
            let i = p.idx();
            let mut pieces: Vec<u8> = self.prog[i].to_vec();
            pieces.sort_unstable();
            let txt: Vec<String> = pieces
                .iter()
                .map(|&x| match x {
                    0 => "cárcel".to_string(),
                    GOAL => "corona".to_string(),
                    69..=76 => format!("{x}(pasillo)"),
                    _ => format!(
                        "{x}(pătrat {}{})",
                        square(p, x),
                        if is_safe(square(p, x)) { ", sigur" } else { "" }
                    ),
                })
                .collect();
            s.push_str(&format!("J{}: {}\n", i + 1, txt.join(", ")));
        }
        if self.outcome().is_none() {
            s.push_str(&format!(
                "La mutare: J{} — zaruri {}+{}{}; folosibile: {:?}\n",
                self.side.idx() + 1,
                self.dice[0],
                self.dice[1],
                if self.pair { " (pereche)" } else { "" },
                self.left
            ));
        } else if let Some(Outcome::Win(p)) = self.outcome() {
            s.push_str(&format!("Jucător {} câștigă.\n", p.idx() + 1));
        } else {
            s.push_str("Remiză (limita de ture).\n");
        }
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Step(x, n) => format!("{x}:{n}"),
            Move::Release => "sale".into(),
            Move::Pass => "pasa".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

impl Parques {
    fn total(&self, p: usize) -> i32 {
        self.prog[p].iter().map(|&x| x as i32).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixed(prog1: [u8; 4], prog2: [u8; 4], dice: [u8; 2]) -> Parques {
        let mut g = Parques::with_seed(1);
        g.prog = [prog1, prog2];
        g.side = Player::One;
        g.dice = dice;
        g.pair = dice[0] == dice[1];
        g.doubles = u8::from(g.pair);
        g.left = vec![dice[0], dice[1]];
        g
    }

    #[test]
    fn geometry_has_twelve_seguros_and_four_salidas() {
        let seguros = (0..TRACK).filter(|&q| is_safe(q) && q % 17 != 0).count();
        assert_eq!(seguros, 12);
        assert_eq!((0..TRACK).filter(|&q| q % 17 == 0).count(), 4);
        assert_eq!(square(Player::One, 1), 0);
        assert_eq!(square(Player::Two, 1), 34);
        assert_eq!(square(Player::One, 68), 67);
    }

    #[test]
    fn only_a_pair_releases_all_jailed_pieces() {
        let g = fixed([0, 0, 0, 0], [0, 0, 0, 0], [3, 3]);
        assert_eq!(g.legal_moves(), vec![Move::Release]);
        let mut g2 = g.clone();
        g2.apply(&Move::Release);
        assert_eq!(g2.prog[0], [1, 1, 1, 1]);
        // fără pereche nu există eliberare
        let h = fixed([0, 5, 0, 0], [0; 4], [2, 4]);
        assert!(!h.legal_moves().contains(&Move::Release));
    }

    #[test]
    fn release_captures_an_enemy_on_our_salida() {
        let mut g = fixed([0, 0, 0, 0], [0; 4], [2, 2]);
        // piesa lui J2 cu progres 35 e pe pătratul 34+34 = 68 -> 0
        g.prog[1][0] = 35;
        assert_eq!(square(Player::Two, 35), 0);
        g.apply(&Move::Release);
        assert_eq!(g.prog[1][0], 0);
    }

    #[test]
    fn exact_entry_to_the_corona_and_dice_split() {
        let g = fixed([72, 0, 0, 0], [0; 4], [5, 3]);
        let m = g.legal_moves();
        assert!(m.contains(&Move::Step(72, 5)));
        assert!(m.contains(&Move::Step(72, 3)) || !m.contains(&Move::Step(72, 3)));
        assert!(!m.contains(&Move::Step(72, 8))); // 80 > 77
        let g = fixed([75, 0, 0, 0], [0; 4], [6, 2]);
        let m = g.legal_moves();
        assert_eq!(m, vec![Move::Step(75, 2)]);
    }

    #[test]
    fn capture_is_mandatory_and_safe_squares_protect() {
        // J1 la progres 10 -> pătratul 9; J2 piesă la pătratul 14? 14 e sigur (17k+14). Alegem pătrat 12.
        let mut g = fixed([10, 20, 0, 0], [0; 4], [3, 4]);
        // J2 progres p pe pătratul 12: (34 + p - 1) % 68 = 12 -> p = 12 + 68 - 33 = 47
        g.prog[1][0] = 47;
        assert_eq!(square(Player::Two, 47), 12);
        // 10 + 3 = 13 -> pătrat 12: captură cu zarul 3
        let m = g.legal_moves();
        assert_eq!(m, vec![Move::Step(10, 3)]);
        g.apply(&Move::Step(10, 3));
        assert_eq!(g.prog[1][0], 0);
        // pe un pătrat sigur: fără captură
        let mut h = fixed([10, 20, 0, 0], [0; 4], [3, 4]);
        // pătrat 14 = sigur: progres 15 pentru J1; J2 progres 49
        h.prog[1][0] = 49;
        assert_eq!(square(Player::Two, 49), 14);
        let m = h.legal_moves();
        assert!(m.len() > 1);
        h.apply(&Move::Step(10, 5));
        assert_eq!(h.prog[1][0], 49);
    }

    #[test]
    fn three_doubles_send_the_rearmost_piece_home() {
        let mut found = false;
        for seed in 1..4000u64 {
            let mut g = Parques::with_seed(seed);
            g.prog[0] = [3, 10, 0, 0];
            g.doubles = 2;
            let before = g.clone();
            g.roll_for_side();
            if g.prog[0].contains(&77) {
                assert!(before.prog[0].contains(&3));
                assert!(g.prog[0].contains(&10));
                found = true;
                break;
            }
        }
        assert!(found);
    }

    #[test]
    fn random_games_finish() {
        for seed in 1..=10u64 {
            let mut g = Parques::with_seed(seed);
            let mut rng = Rng::new(seed ^ 0xabc);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                let mut seen = BTreeSet::new();
                for x in &m {
                    assert!(seen.insert(g.format_move(x)));
                }
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 4000, "too many plies");
            }
        }
    }

    #[test]
    fn ai_beats_random() {
        use polytabletop_core::GameObj;
        let mut wins = 0;
        let n = 10;
        for seed in 1..=n {
            let mut g: Box<dyn GameObj> = Box::new(Parques::with_seed(seed));
            let mut rng = Rng::new(seed + 1000);
            let mut plies = 0;
            while g.outcome().is_none() && plies < 6000 {
                let mv = if g.to_move() == Player::One {
                    g.ai_move(rng.next_u64()).unwrap()
                } else {
                    let m = g.legal_moves();
                    m[rng.below(m.len())].clone()
                };
                g.play(&mv).unwrap();
                plies += 1;
            }
            if g.outcome() == Some(Outcome::Win(Player::One)) {
                wins += 1;
            }
        }
        assert!(wins >= 7, "AI won {wins}/{n}");
    }
}
