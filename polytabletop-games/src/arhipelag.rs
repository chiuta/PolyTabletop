//! Arhipelagul în Flăcări — cucerire de teritorii cu zaruri (variantă originală), 2 jucători.
//!
//! Harta: 14 insule în 4 regiuni (Nord, Est, Vest, Sud), cu legături proprii (vezi `ADJ`).
//! * Start: insulele se împart la sorți câte 7, cu câte 3 armate pe insulă. Începe J1.
//! * Tura: **Întărire** (`pune Fiord` = 1 armată, `pune Fiord*` = toate rămase) →
//!   **Atac** (`ataca Fiord>Prag`; apoi mutarea forțată `arunc` rezolvă o rundă de zaruri;
//!   `gata atac`) → **Mutare** (`muta Fiord>Prag 3`, o singură dată, sau `gata`).
//! * Întăriri: max(3, insule/3) + bonusul regiunilor deținute întregi (Nord 2, Est 3, Vest 2, Sud 3).
//! * Zaruri: atacatorul dă min(3, armate−1) zaruri, apărătorul min(2, armate); se compară
//!   cele mai mari perechi, egalitatea o câștigă apărătorul. Insula cucerită primește atâtea
//!   armate câte zaruri a dat atacatorul în runda câștigătoare.
//! * Câștigă cine rămâne singur. Limita: după 70 de ture jocul se oprește și câștigă cine are
//!   mai multe insule (apoi armate); egal = remiză.
//! * Rezolvarea zarurilor e o mutare separată (`arunc`), deci AI-ul nu vede zarurile viitoare.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const N: usize = 14;
pub const NAMES: [&str; N] = [
    "Fiord", "Gheata", "Corn", "Prag", "Delta", "Recif", "Farul", "Dune", "Tamar", "Pesteri",
    "Stanca", "Laguna", "Vulcan", "Port",
];
/// Regiuni: (insule, bonus, nume).
pub const REGIONS: [(&[usize], u8, &str); 4] = [
    (&[0, 1, 2], 2, "Nord"),
    (&[3, 4, 5, 6], 3, "Est"),
    (&[7, 8, 9], 2, "Vest"),
    (&[10, 11, 12, 13], 3, "Sud"),
];
pub const ADJ: [(usize, usize); 21] = [
    (0, 1),
    (1, 2),
    (0, 2),
    (2, 3),
    (3, 4),
    (3, 5),
    (4, 6),
    (5, 6),
    (2, 8),
    (0, 7),
    (7, 8),
    (8, 9),
    (7, 10),
    (9, 11),
    (10, 11),
    (11, 12),
    (10, 13),
    (12, 13),
    (6, 12),
    (5, 11),
    (1, 4),
];
pub const TURN_CAP: u32 = 70;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Reinforce,
    Attack,
    Fortify,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Place(usize, bool),
    Attack(usize, usize),
    Roll,
    EndAttack,
    Move(usize, usize, u8),
    EndTurn,
}

#[derive(Clone)]
pub struct Arhipelag {
    owner: [Player; N],
    armies: [u8; N],
    side: Player,
    phase: Phase,
    to_place: u8,
    pending: Option<(usize, usize)>,
    turns: u32,
    rng: Rng,
    log: String,
}

impl Default for Arhipelag {
    fn default() -> Self {
        Self::new()
    }
}

pub fn adjacent(a: usize, b: usize) -> bool {
    ADJ.iter()
        .any(|&(x, y)| (x == a && y == b) || (x == b && y == a))
}

fn neighbors(a: usize) -> impl Iterator<Item = usize> {
    (0..N).filter(move |&b| adjacent(a, b))
}

impl Arhipelag {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut ids: Vec<usize> = (0..N).collect();
        for i in (1..N).rev() {
            let j = rng.below(i + 1);
            ids.swap(i, j);
        }
        let mut owner = [Player::One; N];
        for (k, &t) in ids.iter().enumerate() {
            owner[t] = if k % 2 == 0 { Player::One } else { Player::Two };
        }
        let mut g = Arhipelag {
            owner,
            armies: [3; N],
            side: Player::One,
            phase: Phase::Reinforce,
            to_place: 0,
            pending: None,
            turns: 0,
            rng,
            log: String::new(),
        };
        g.to_place = g.reinforcements(Player::One);
        g
    }

    pub fn reinforcements(&self, p: Player) -> u8 {
        let owned = self.owner.iter().filter(|&&o| o == p).count() as u8;
        let mut r = (owned / 3).max(3);
        for (ts, bonus, _) in REGIONS {
            if ts.iter().all(|&t| self.owner[t] == p) {
                r += bonus;
            }
        }
        r
    }

    pub fn count(&self, p: Player) -> usize {
        self.owner.iter().filter(|&&o| o == p).count()
    }

    pub fn total_armies(&self, p: Player) -> u32 {
        (0..N)
            .filter(|&t| self.owner[t] == p)
            .map(|t| self.armies[t] as u32)
            .sum()
    }

    fn end_turn(&mut self) {
        self.turns += 1;
        self.side = self.side.other();
        self.phase = Phase::Reinforce;
        self.pending = None;
        self.to_place = self.reinforcements(self.side);
    }

    fn border_score(&self, p: Player) -> i32 {
        let mut s = 0;
        for t in 0..N {
            if self.owner[t] != p {
                continue;
            }
            let mut worst = 0i32;
            let mut border = false;
            for n in neighbors(t) {
                if self.owner[n] != p {
                    border = true;
                    worst = worst.max(self.armies[n] as i32);
                }
            }
            if border {
                s += (self.armies[t] as i32 - worst).clamp(-6, 6);
            }
        }
        s
    }

    fn region_score(&self, p: Player) -> i32 {
        let mut s = 0;
        for (ts, bonus, _) in REGIONS {
            let mine = ts.iter().filter(|&&t| self.owner[t] == p).count() as i32;
            let all = ts.len() as i32;
            let b = bonus as i32;
            s += b * if mine == all { 6 } else { 0 } + mine * b;
        }
        s
    }
}

impl Game for Arhipelag {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        let me = self.side;
        let mut v = Vec::new();
        if self.pending.is_some() {
            return vec![Move::Roll];
        }
        match self.phase {
            Phase::Reinforce => {
                for t in 0..N {
                    if self.owner[t] == me {
                        v.push(Move::Place(t, false));
                        if self.to_place > 1 {
                            v.push(Move::Place(t, true));
                        }
                    }
                }
            }
            Phase::Attack => {
                for a in 0..N {
                    if self.owner[a] == me && self.armies[a] >= 2 {
                        for b in neighbors(a) {
                            if self.owner[b] != me {
                                v.push(Move::Attack(a, b));
                            }
                        }
                    }
                }
                v.push(Move::EndAttack);
            }
            Phase::Fortify => {
                for a in 0..N {
                    if self.owner[a] == me && self.armies[a] >= 2 {
                        for b in neighbors(a) {
                            if self.owner[b] == me {
                                let max = self.armies[a] - 1;
                                v.push(Move::Move(a, b, 1));
                                if max > 1 {
                                    v.push(Move::Move(a, b, max));
                                }
                            }
                        }
                    }
                }
                v.push(Move::EndTurn);
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        let me = self.side;
        match *m {
            Move::Place(t, all) => {
                let n = if all { self.to_place } else { 1 };
                self.armies[t] += n;
                self.to_place -= n;
                if self.to_place == 0 {
                    self.phase = Phase::Attack;
                }
                self.log = format!("J{}: +{} pe {}.", me.idx() + 1, n, NAMES[t]);
            }
            Move::Attack(a, b) => {
                self.pending = Some((a, b));
            }
            Move::Roll => {
                let (a, b) = self.pending.take().unwrap();
                let na = (self.armies[a] - 1).min(3) as usize;
                let nd = self.armies[b].min(2) as usize;
                let mut da: Vec<u8> = (0..na).map(|_| self.rng.d6()).collect();
                let mut dd: Vec<u8> = (0..nd).map(|_| self.rng.d6()).collect();
                da.sort_unstable_by(|x, y| y.cmp(x));
                dd.sort_unstable_by(|x, y| y.cmp(x));
                let (mut la, mut ld) = (0u8, 0u8);
                for k in 0..na.min(nd) {
                    if da[k] > dd[k] {
                        ld += 1;
                    } else {
                        la += 1;
                    }
                }
                self.armies[a] -= la;
                self.armies[b] -= ld;
                self.log = format!(
                    "{} ataca {}: {:?} contra {:?} → atacatorul −{}, apărătorul −{}.",
                    NAMES[a], NAMES[b], da, dd, la, ld
                );
                if self.armies[b] == 0 {
                    let mv = (na as u8).min(self.armies[a] - 1).max(1);
                    self.armies[a] -= mv;
                    self.armies[b] = mv;
                    self.owner[b] = me;
                    self.log
                        .push_str(&format!(" {} e cucerită ({mv} armate).", NAMES[b]));
                    if self.count(me.other()) == 0 {
                        self.phase = Phase::Fortify; // jocul s-a terminat; `outcome` îl vede
                    }
                }
            }
            Move::EndAttack => {
                self.phase = Phase::Fortify;
            }
            Move::Move(a, b, n) => {
                self.armies[a] -= n;
                self.armies[b] += n;
                self.log = format!(
                    "J{}: mută {} armate {} → {}.",
                    me.idx() + 1,
                    n,
                    NAMES[a],
                    NAMES[b]
                );
                self.end_turn();
            }
            Move::EndTurn => {
                self.end_turn();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        let (a, b) = (self.count(Player::One), self.count(Player::Two));
        if a == 0 {
            return Some(Outcome::Win(Player::Two));
        }
        if b == 0 {
            return Some(Outcome::Win(Player::One));
        }
        if self.turns >= TURN_CAP {
            let (x, y) = (
                (a, self.total_armies(Player::One)),
                (b, self.total_armies(Player::Two)),
            );
            return Some(match x.cmp(&y) {
                std::cmp::Ordering::Greater => Outcome::Win(Player::One),
                std::cmp::Ordering::Less => Outcome::Win(Player::Two),
                std::cmp::Ordering::Equal => Outcome::Draw,
            });
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let o = p.other();
        let f = |q: Player| {
            10 * self.count(q) as i32
                + 3 * self.total_armies(q) as i32
                + 2 * self.region_score(q)
                + 2 * self.border_score(q)
        };
        let mut v = f(p) - f(o);
        if let Some((a, b)) = self.pending {
            // atac în așteptare: valoare după șansele zarurilor (cunoscute public: armatele)
            let adv = self.armies[a] as i32 - self.armies[b] as i32;
            let bonus = if adv >= 2 {
                8 + 4 * (adv - 2).min(4)
            } else {
                -12
            };
            if self.owner[a] == p {
                v += bonus;
            } else {
                v -= bonus;
            }
        }
        v
    }

    fn render(&self) -> String {
        let mut s = String::from("Arhipelagul în Flăcări (X = J1, O = J2)\n");
        for (ts, bonus, name) in REGIONS {
            s.push_str(&format!("{name} (bonus {bonus}): "));
            for &t in ts {
                s.push_str(&format!(
                    "{}[{}{}] ",
                    NAMES[t],
                    if self.owner[t] == Player::One {
                        'X'
                    } else {
                        'O'
                    },
                    self.armies[t]
                ));
            }
            s.push('\n');
        }
        s.push_str("Legături: ");
        for (a, b) in ADJ {
            s.push_str(&format!("{}-{} ", NAMES[a], NAMES[b]));
        }
        s.push('\n');
        s.push_str(&format!(
            "Insule: J1 {} ({} armate), J2 {} ({} armate)   tura {}/{}\n",
            self.count(Player::One),
            self.total_armies(Player::One),
            self.count(Player::Two),
            self.total_armies(Player::Two),
            self.turns + 1,
            TURN_CAP
        ));
        if self.outcome().is_none() {
            let ph = if self.pending.is_some() {
                "zaruri (scrie «arunc»)".to_string()
            } else {
                match self.phase {
                    Phase::Reinforce => format!("întărire, {} armate de pus", self.to_place),
                    Phase::Attack => "atac".into(),
                    Phase::Fortify => "mutare finală".into(),
                }
            };
            s.push_str(&format!(
                "La mutare: jucătorul {} — {}\n",
                self.side.idx() + 1,
                ph
            ));
        }
        s.push_str(self.log.trim());
        s.push('\n');
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match *m {
            Move::Place(t, all) => format!("pune {}{}", NAMES[t], if all { "*" } else { "" }),
            Move::Attack(a, b) => format!("ataca {}>{}", NAMES[a], NAMES[b]),
            Move::Roll => "arunc".into(),
            Move::EndAttack => "gata atac".into(),
            Move::Move(a, b, n) => format!("muta {}>{} {}", NAMES[a], NAMES[b], n),
            Move::EndTurn => "gata".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_is_connected_and_regions_partition() {
        let mut seen = [false; N];
        let mut stack = vec![0usize];
        while let Some(t) = stack.pop() {
            if seen[t] {
                continue;
            }
            seen[t] = true;
            stack.extend(neighbors(t));
        }
        assert!(seen.iter().all(|&x| x));
        let mut cover = [0; N];
        for (ts, _, _) in REGIONS {
            for &t in ts {
                cover[t] += 1;
            }
        }
        assert!(cover.iter().all(|&c| c == 1));
        for &(a, b) in ADJ.iter() {
            assert_ne!(a, b);
            assert_eq!(
                ADJ.iter()
                    .filter(|&&(x, y)| (x, y) == (a, b) || (x, y) == (b, a))
                    .count(),
                1
            );
        }
        let mut names = NAMES.to_vec();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), N);
    }

    #[test]
    fn setup_splits_territories_evenly() {
        for seed in 1..=10 {
            let g = Arhipelag::with_seed(seed);
            assert_eq!(g.count(Player::One), 7);
            assert_eq!(g.count(Player::Two), 7);
            assert_eq!(g.total_armies(Player::One), 21);
        }
    }

    #[test]
    fn reinforcement_formula_with_regions() {
        let mut g = Arhipelag::with_seed(1);
        g.owner = [Player::Two; N];
        for t in [0, 1, 2] {
            g.owner[t] = Player::One;
        }
        assert_eq!(g.reinforcements(Player::One), 3 + 2);
        for t in [3, 4, 5, 6] {
            g.owner[t] = Player::One;
        }
        // 7 insule → 7/3 = 2 → max(3) = 3; Nord 2 + Est 3
        assert_eq!(g.reinforcements(Player::One), 3 + 2 + 3);
    }

    #[test]
    fn reinforcement_then_attack_flow() {
        let mut g = Arhipelag::with_seed(2);
        let n = g.to_place;
        let t = (0..N).find(|&t| g.owner[t] == Player::One).unwrap();
        g.apply(&Move::Place(t, true));
        assert_eq!(g.phase, Phase::Attack);
        assert_eq!(g.armies[t], 3 + n);
        assert!(g.legal_moves().contains(&Move::EndAttack));
    }

    #[test]
    fn dice_round_conserves_and_comparison_is_correct() {
        // un atac cu 3 vs 1: nu poate pierde mai mult de 1 armată atacatoare
        let mut g = Arhipelag::with_seed(3);
        let (a, b) = ADJ[0];
        g.owner[a] = Player::One;
        g.owner[b] = Player::Two;
        g.armies[a] = 4;
        g.armies[b] = 1;
        g.phase = Phase::Attack;
        g.apply(&Move::Attack(a, b));
        assert_eq!(g.legal_moves(), vec![Move::Roll]);
        g.apply(&Move::Roll);
        let lost_att = 4 - g.armies[a] as i32;
        assert!(lost_att <= 1 || g.owner[b] == Player::One);
        if g.owner[b] == Player::One {
            // armatele se conservă: 4 − pierderi = a + b
            assert_eq!(g.armies[a] + g.armies[b], 4);
        } else {
            assert_eq!(g.armies[b], 1);
            assert_eq!(g.armies[a], 3);
        }
    }

    #[test]
    fn conquest_moves_in_dice_count() {
        let mut g = Arhipelag::with_seed(3);
        let (a, b) = ADJ[1];
        g.owner[a] = Player::One;
        g.owner[b] = Player::Two;
        g.phase = Phase::Attack;
        for seed in 0..50 {
            let mut h = g.clone();
            h.rng = Rng::new(seed);
            h.armies[a] = 10;
            h.armies[b] = 1;
            h.apply(&Move::Attack(a, b));
            h.apply(&Move::Roll);
            if h.owner[b] == Player::One {
                assert!(h.armies[b] >= 1 && h.armies[b] <= 3);
                assert_eq!(h.armies[a] + h.armies[b], 10);
            }
        }
    }

    #[test]
    fn elimination_ends_game() {
        let mut g = Arhipelag::with_seed(3);
        g.owner = [Player::One; N];
        g.owner[3] = Player::Two;
        g.armies[3] = 1;
        g.owner[2] = Player::One;
        g.armies[2] = 20;
        g.phase = Phase::Attack;
        for seed in 0..20 {
            let mut h = g.clone();
            h.rng = Rng::new(seed);
            for _ in 0..10 {
                if h.outcome().is_some() {
                    break;
                }
                h.apply(&Move::Attack(2, 3));
                h.apply(&Move::Roll);
            }
            assert_eq!(h.outcome(), Some(Outcome::Win(Player::One)));
        }
    }

    #[test]
    fn fortify_moves_and_ends_turn() {
        let mut g = Arhipelag::with_seed(4);
        g.phase = Phase::Fortify;
        let m = g.legal_moves();
        assert!(m.contains(&Move::EndTurn));
        g.apply(&Move::EndTurn);
        assert_eq!(g.side, Player::Two);
        assert_eq!(g.phase, Phase::Reinforce);
        assert_eq!(g.to_place, g.reinforcements(Player::Two));
    }

    #[test]
    fn random_games_end_by_cap_or_conquest() {
        for seed in 1..=15u64 {
            let mut g = Arhipelag::with_seed(seed);
            let mut rng = Rng::new(seed + 7);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                let pick = m[rng.below(m.len())];
                g.apply(&pick);
                n += 1;
                assert!(n < 4000, "seed {seed}: {n} plies");
            }
        }
    }

    #[test]
    fn ai_plays_and_beats_random_more_often_than_not() {
        let mut ai_w = 0;
        let mut rnd_w = 0;
        for seed in 1..=10u64 {
            let ai = if seed % 2 == 0 {
                Player::One
            } else {
                Player::Two
            };
            let mut g = Arhipelag::with_seed(seed);
            let mut rng = Rng::new(seed * 5);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = if g.side == ai {
                    polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap()
                } else {
                    let m = g.legal_moves();
                    m[rng.below(m.len())]
                };
                g.apply(&mv);
                n += 1;
                assert!(n < 4000);
            }
            match g.outcome() {
                Some(Outcome::Win(p)) if p == ai => ai_w += 1,
                Some(Outcome::Win(_)) => rnd_w += 1,
                _ => {}
            }
        }
        assert!(ai_w > rnd_w, "ai {ai_w} vs random {rnd_w}");
    }
}
