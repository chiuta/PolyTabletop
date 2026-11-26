//! Câmpul Ceții — ierarhie de piese ascunse pe o tablă de 10×10 (variantă originală), 2 jucători.
//!
//! * Fiecare jucător are 40 de piese; adversarul vede doar locul lor, nu rangul. Piese: Steag (F),
//!   Mine (B ×6), 1 Iscoadă (×1), 2 Iscoditori (×8), 3 Săpători (×5), 4 Sergenți (×4), 5 Locotenenți (×4),
//!   6 Căpitani (×3), 7 Maiori (×3), 8 Coloneli (×2), 9 Generali (×2), X = 10 Mareșal (×1).
//! * Poziția de start este amestecată automat (steagul în rândul din spate, câteva mine lângă el); J1 e jos
//!   (rândurile 1–4), J2 sus (7–10). Lacurile (~) pe coloanele c, d, g, h la rândurile 5–6 nu se trec.
//! * Mutare: un pas ortogonal pe pătrat liber (`e4-e5`) sau pe o piesă adversă = atac (`e4xe5`); Iscoditorul
//!   merge oricât în linie dreaptă (arată că e Iscoditor când face mai mult de un pas). Steagul și minele nu se
//!   mișcă. Atacul se rezolvă printr-o mutare forțată separată (`bataie`), ca AI-ul să nu „vadă” piesa
//!   ascunsă înainte de a ataca.
//! * Bătălia: rang mai mare câștigă; egal = dispar amândouă; Iscoada atacă Mareșalul și îl învinge; Săpătorul
//!   dezamorsează o mină, orice altă piesă care atacă o mină dispare; steagul capturat = victorie. Ambele
//!   piese devin vizibile după bătălie. Pierde cine nu mai are mutări.
//! * Simplificare: fără regula celor două pătrate/ mutare repetată; în schimb, 100 de mutări fără bătălie = remiză.
//!
//! # AI și informație
//! `evaluate(p)` citește rangul pieselor proprii și doar al pieselor adverse **devenite vizibile**; pentru restul
//! folosește distribuția pieselor încă necunoscute (40 − pierdute − vizibile). AI-ul caută 1 ply (mutări + atac
//! în așteptare), deci nu poate trișa.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const W: usize = 10;
pub const NO_BATTLE_CAP: u32 = 100;

pub const FLAG: u8 = 0;
pub const SPY: u8 = 1;
pub const SCOUT: u8 = 2;
pub const MINER: u8 = 3;
pub const MARSHAL: u8 = 10;
pub const BOMB: u8 = 11;

const COUNTS: [u8; 12] = [1, 1, 8, 5, 4, 4, 3, 3, 2, 2, 1, 6];
const VALUES: [i32; 12] = [1000, 30, 15, 45, 30, 50, 70, 100, 150, 250, 400, 40];

fn is_lake(r: usize, c: usize) -> bool {
    (r == 4 || r == 5) && matches!(c, 2 | 3 | 6 | 7)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Piece {
    pub owner: Player,
    pub kind: u8,
    pub rev: bool,
}

fn kind_char(k: u8) -> char {
    match k {
        FLAG => 'F',
        BOMB => 'B',
        MARSHAL => 'X',
        n => (b'0' + n) as char,
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Step(u8, u8),
    Attack(u8, u8),
    Resolve,
}

#[derive(Clone)]
pub struct Ceata {
    board: [Option<Piece>; 100],
    side: Player,
    pending: Option<(u8, u8)>,
    lost: [[u8; 12]; 2],
    quiet: u32,
    winner: Option<Player>,
    log: String,
    /// `Some(p)` = render shows `p`'s piece ranks; `None` = hot-seat (the mover's).
    viewer: Option<Player>,
}

impl Default for Ceata {
    fn default() -> Self {
        Self::new()
    }
}

fn sq_name(s: usize) -> String {
    format!("{}{}", (b'a' + (s % W) as u8) as char, s / W + 1)
}

impl Ceata {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut g = Ceata {
            board: [None; 100],
            side: Player::One,
            pending: None,
            lost: [[0; 12]; 2],
            quiet: 0,
            winner: None,
            log: String::new(),
            viewer: Some(Player::One),
        };
        for p in [Player::One, Player::Two] {
            g.setup(p, &mut rng);
        }
        g
    }

    fn setup(&mut self, p: Player, rng: &mut Rng) {
        // 4 rânduri × 10 coloane; rândul din spate = rândul 0 (J1) / 9 (J2)
        let rows: Vec<usize> = if p == Player::One {
            vec![0, 1, 2, 3]
        } else {
            vec![9, 8, 7, 6]
        };
        let mut cells: Vec<usize> = rows
            .iter()
            .flat_map(|&r| (0..W).map(move |c| r * W + c))
            .collect();
        let mut kinds: Vec<u8> = Vec::new();
        for (k, &n) in COUNTS.iter().enumerate() {
            for _ in 0..n {
                kinds.push(k as u8);
            }
        }
        // steagul în rândul din spate
        let fc = rng.below(W);
        let flag_sq = rows[0] * W + fc;
        self.board[flag_sq] = Some(Piece {
            owner: p,
            kind: FLAG,
            rev: false,
        });
        cells.retain(|&c| c != flag_sq);
        kinds.retain(|&k| k != FLAG);
        // 3 mine lângă steag (stânga, dreapta, în față)
        let mut bombs = 6;
        let mut guards: Vec<usize> = Vec::new();
        if fc > 0 {
            guards.push(flag_sq - 1);
        }
        if fc + 1 < W {
            guards.push(flag_sq + 1);
        }
        guards.push(if p == Player::One {
            flag_sq + W
        } else {
            flag_sq - W
        });
        for gsq in guards {
            self.board[gsq] = Some(Piece {
                owner: p,
                kind: BOMB,
                rev: false,
            });
            cells.retain(|&c| c != gsq);
            bombs -= 1;
        }
        for _ in 0..(6 - bombs) {
            let pos = kinds.iter().position(|&k| k == BOMB).unwrap();
            kinds.remove(pos);
        }
        for i in (1..kinds.len()).rev() {
            let j = rng.below(i + 1);
            kinds.swap(i, j);
        }
        for (cell, kind) in cells.into_iter().zip(kinds) {
            self.board[cell] = Some(Piece {
                owner: p,
                kind,
                rev: false,
            });
        }
    }

    pub fn piece_at(&self, r: usize, c: usize) -> Option<Piece> {
        self.board[r * W + c]
    }

    pub fn count_pieces(&self, p: Player) -> usize {
        self.board.iter().flatten().filter(|x| x.owner == p).count()
    }

    fn steps(&self, from: usize) -> Vec<Move> {
        let Some(pc) = self.board[from] else {
            return Vec::new();
        };
        if pc.kind == FLAG || pc.kind == BOMB {
            return Vec::new();
        }
        let (r, c) = ((from / W) as i32, (from % W) as i32);
        let mut v = Vec::new();
        for (dr, dc) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let (mut nr, mut nc) = (r + dr, c + dc);
            loop {
                if !(0..10).contains(&nr)
                    || !(0..10).contains(&nc)
                    || is_lake(nr as usize, nc as usize)
                {
                    break;
                }
                let to = (nr * 10 + nc) as usize;
                match self.board[to] {
                    None => v.push(Move::Step(from as u8, to as u8)),
                    Some(o) => {
                        if o.owner != pc.owner {
                            v.push(Move::Attack(from as u8, to as u8));
                        }
                        break;
                    }
                }
                if pc.kind != SCOUT {
                    break;
                }
                nr += dr;
                nc += dc;
            }
        }
        v
    }

    fn all_moves(&self, p: Player) -> Vec<Move> {
        let mut v = Vec::new();
        for s in 0..100 {
            if let Some(pc) = self.board[s] {
                if pc.owner == p {
                    v.extend(self.steps(s));
                }
            }
        }
        v
    }

    /// Piesele necunoscute (încă ascunse) ale lui `p` după ce vede `viewer`: distribuție pe tipuri.
    fn unknown_counts(&self, p: Player) -> ([i32; 12], i32) {
        let mut cnt = [0i32; 12];
        for (k, &n) in COUNTS.iter().enumerate() {
            cnt[k] = n as i32 - self.lost[p.idx()][k] as i32;
        }
        for pc in self.board.iter().flatten() {
            if pc.owner == p && pc.rev {
                cnt[pc.kind as usize] -= 1;
            }
        }
        let n = cnt.iter().sum();
        (cnt, n)
    }

    fn material(&self, viewer: Player, p: Player) -> i32 {
        let mut v = 0;
        if p == viewer {
            for pc in self.board.iter().flatten() {
                if pc.owner == p && pc.kind != FLAG {
                    v += VALUES[pc.kind as usize];
                }
            }
        } else {
            let (cnt, n) = self.unknown_counts(p);
            let mut avg_total = 0;
            for (k, &c) in cnt.iter().enumerate() {
                if k as u8 != FLAG {
                    avg_total += c * VALUES[k];
                }
            }
            let _ = n;
            // piesele vizibile la valoarea lor, restul la media distribuției (aici: suma exactă a necunoscutelor)
            v += avg_total;
            for pc in self.board.iter().flatten() {
                if pc.owner == p && pc.rev && pc.kind != FLAG {
                    v += VALUES[pc.kind as usize];
                }
            }
        }
        v
    }

    /// Rezultatul bătăliei: 1 = atacatorul câștigă, 0 = egal, −1 = apărătorul câștigă.
    fn battle(att: u8, def: u8) -> i32 {
        if def == FLAG {
            return 1;
        }
        if def == BOMB {
            return if att == MINER { 1 } else { -1 };
        }
        if att == SPY && def == MARSHAL {
            return 1;
        }
        match att.cmp(&def) {
            std::cmp::Ordering::Greater => 1,
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
        }
    }

    /// Speranța (din punctul de vedere al lui `viewer`) variației de material a bătăliei în așteptare.
    fn pending_delta(&self, viewer: Player) -> i32 {
        let Some((a, d)) = self.pending else { return 0 };
        let (pa, pd) = (
            self.board[a as usize].unwrap(),
            self.board[d as usize].unwrap(),
        );
        let sign = if pa.owner == viewer { 1 } else { -1 };
        // `viewer` cunoaște doar piesa sa; cealaltă după vizibilitate sau distribuție
        let (mine, other_known, other_owner) = if pa.owner == viewer {
            (pa.kind, if pd.rev { Some(pd.kind) } else { None }, pd.owner)
        } else {
            (pd.kind, if pa.rev { Some(pa.kind) } else { None }, pa.owner)
        };
        let (cnt, n) = self.unknown_counts(other_owner);
        let dist: Vec<(u8, i32)> = match other_known {
            Some(k) => vec![(k, 1)],
            None => (0..12u8)
                .filter(|&k| cnt[k as usize] > 0)
                .map(|k| (k, cnt[k as usize]))
                .collect(),
        };
        let total: i32 = dist.iter().map(|x| x.1).sum::<i32>().max(1);
        let _ = n;
        let mut exp = 0i64;
        for (k, w) in dist {
            // rezultat din punctul de vedere al atacatorului
            let (att_k, def_k) = if sign == 1 { (mine, k) } else { (k, mine) };
            let r = Self::battle(att_k, def_k);
            let (att_val, def_val) = (VALUES[att_k as usize], VALUES[def_k as usize]);
            let delta_att = match r {
                1 => def_val,
                0 => def_val - att_val,
                _ => -att_val,
            };
            exp += (delta_att as i64) * (sign as i64) * (w as i64);
        }
        (exp / total as i64) as i32
    }

    fn do_resolve(&mut self) {
        let (a, d) = self.pending.take().unwrap();
        let (a, d) = (a as usize, d as usize);
        let mut pa = self.board[a].unwrap();
        let mut pd = self.board[d].unwrap();
        pa.rev = true;
        pd.rev = true;
        let r = Self::battle(pa.kind, pd.kind);
        self.log = format!(
            "Bătălie {} ({}) contra {} ({}): ",
            sq_name(a),
            kind_char(pa.kind),
            sq_name(d),
            kind_char(pd.kind)
        );
        self.quiet = 0;
        match r {
            1 => {
                self.lost[pd.owner.idx()][pd.kind as usize] += 1;
                self.board[a] = None;
                self.board[d] = Some(pa);
                self.log.push_str("atacatorul câștigă.");
                if pd.kind == FLAG {
                    self.winner = Some(pa.owner);
                    self.log.push_str(" Steagul a fost capturat!");
                }
            }
            0 => {
                self.lost[pd.owner.idx()][pd.kind as usize] += 1;
                self.lost[pa.owner.idx()][pa.kind as usize] += 1;
                self.board[a] = None;
                self.board[d] = None;
                self.log.push_str("ambele piese dispar.");
            }
            _ => {
                self.lost[pa.owner.idx()][pa.kind as usize] += 1;
                self.board[a] = None;
                self.board[d] = Some(pd);
                self.log.push_str("apărătorul câștigă.");
            }
        }
        self.side = self.side.other();
    }
}

impl Game for Ceata {
    type Move = Move;

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        if self.pending.is_some() {
            return vec![Move::Resolve];
        }
        self.all_moves(self.side)
    }

    fn apply(&mut self, m: &Move) {
        match *m {
            Move::Step(f, t) => {
                let (f, t) = (f as usize, t as usize);
                let mut pc = self.board[f].take().unwrap();
                let dist = ((f / W).abs_diff(t / W) + (f % W).abs_diff(t % W)) as u32;
                if dist > 1 {
                    pc.rev = true;
                }
                self.board[t] = Some(pc);
                self.quiet += 1;
                self.log = format!("J{}: {}-{}.", self.side.idx() + 1, sq_name(f), sq_name(t));
                self.side = self.side.other();
            }
            Move::Attack(f, t) => {
                self.pending = Some((f, t));
                self.log = format!(
                    "J{}: atacă {}x{}.",
                    self.side.idx() + 1,
                    sq_name(f as usize),
                    sq_name(t as usize)
                );
            }
            Move::Resolve => {
                let side = self.side;
                self.do_resolve();
                let _ = side;
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if let Some(w) = self.winner {
            return Some(Outcome::Win(w));
        }
        if self.pending.is_none() {
            if self.quiet >= NO_BATTLE_CAP {
                return Some(Outcome::Draw);
            }
            if self.all_moves(self.side).is_empty() {
                return Some(Outcome::Win(self.side.other()));
            }
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let o = p.other();
        let mut v = self.material(p, p) - self.material(p, o);
        v += self.pending_delta(p);
        // informație: a ataca o piesă necunoscută o dezvăluie
        if let Some((a, d)) = self.pending {
            let (pa, pd) = (
                self.board[a as usize].unwrap(),
                self.board[d as usize].unwrap(),
            );
            if pa.owner == p && !pd.rev {
                v += 6;
            }
        }
        // avans al pieselor mobile și siguranță față de piese vizibile mai puternice
        for s in 0..100 {
            if let Some(pc) = self.board[s] {
                if pc.owner != p || pc.kind == FLAG || pc.kind == BOMB {
                    continue;
                }
                let r = s / W;
                let adv = if p == Player::One { r } else { 9 - r } as i32;
                v += adv.min(7);
                let (row, col) = ((s / W) as i32, (s % W) as i32);
                for (dr, dc) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let (nr, nc) = (row + dr, col + dc);
                    if !(0..10).contains(&nr) || !(0..10).contains(&nc) {
                        continue;
                    }
                    if let Some(e) = self.board[(nr * 10 + nc) as usize] {
                        if e.owner != p && e.rev && e.kind != BOMB && e.kind != FLAG {
                            let r = Self::battle(e.kind, pc.kind);
                            if r == 1 {
                                v -= VALUES[pc.kind as usize] / 3;
                            } else if Self::battle(pc.kind, e.kind) == 1 {
                                v += VALUES[e.kind as usize] / 4;
                            }
                        }
                    }
                }
            }
        }
        v
    }

    fn render(&self) -> String {
        let mut s = String::from("Câmpul Ceții ([x] piesele tale (J1 jos, J2 sus); {?} adversar ascuns, {x} adversar vizibil; ~ lac)\n");
        s.push_str("     a   b   c   d   e   f   g   h   i   j\n");
        let reveal_all = self.winner.is_some();
        let v = self.viewer.unwrap_or(self.side);
        for r in (0..10).rev() {
            let mut line = format!("{:>2}  ", r + 1);
            for c in 0..10 {
                let cell = if is_lake(r, c) {
                    " ~ ".to_string()
                } else {
                    match self.board[r * W + c] {
                        None => " . ".to_string(),
                        Some(pc) if pc.owner == v => format!("[{}]", kind_char(pc.kind)),
                        Some(pc) if pc.rev || reveal_all => format!("{{{}}}", kind_char(pc.kind)),
                        Some(_) => "{?}".to_string(),
                    }
                };
                line.push_str(&cell);
                line.push(' ');
            }
            s.push_str(&line);
            s.push('\n');
        }
        let names = [
            "Steag",
            "Iscoadă(1)",
            "Iscoditor(2)",
            "Săpător(3)",
            "4",
            "5",
            "6",
            "7",
            "8",
            "9",
            "Mareșal(X)",
            "Mină",
        ];
        for p in [Player::One, Player::Two] {
            let lost: Vec<String> = (0..12)
                .filter(|&k| self.lost[p.idx()][k] > 0)
                .map(|k| format!("{}×{}", self.lost[p.idx()][k], names[k]))
                .collect();
            s.push_str(&format!(
                "J{}: {} piese pe tablă; pierdute: {}\n",
                p.idx() + 1,
                self.count_pieces(p),
                if lost.is_empty() {
                    "—".to_string()
                } else {
                    lost.join(", ")
                }
            ));
        }
        if self.outcome().is_none() {
            if self.pending.is_some() {
                s.push_str("Atac în așteptare: scrie «bataie»\n");
            } else {
                s.push_str(&format!(
                    "La mutare: jucătorul {} (mutări fără bătălie: {}/{})\n",
                    self.side.idx() + 1,
                    self.quiet,
                    NO_BATTLE_CAP
                ));
            }
        }
        s.push_str(self.log.trim());
        s.push('\n');
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match *m {
            Move::Step(f, t) => format!("{}-{}", sq_name(f as usize), sq_name(t as usize)),
            Move::Attack(f, t) => format!("{}x{}", sq_name(f as usize), sq_name(t as usize)),
            Move::Resolve => "bataie".to_string(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty() -> Ceata {
        let mut g = Ceata::with_seed(1);
        g.board = [None; 100];
        g
    }

    fn put(g: &mut Ceata, sq: &str, owner: Player, kind: u8) {
        let c = (sq.as_bytes()[0] - b'a') as usize;
        let r: usize = sq[1..].parse::<usize>().unwrap() - 1;
        g.board[r * W + c] = Some(Piece {
            owner,
            kind,
            rev: false,
        });
    }

    fn sq(s: &str) -> u8 {
        let c = (s.as_bytes()[0] - b'a') as usize;
        let r: usize = s[1..].parse::<usize>().unwrap() - 1;
        (r * W + c) as u8
    }

    fn mv(g: &Ceata, text: &str) -> Move {
        g.legal_moves()
            .into_iter()
            .find(|m| g.format_move(m) == text)
            .unwrap_or_else(|| panic!("fără {text}"))
    }

    #[test]
    fn setup_has_40_pieces_each_with_correct_composition() {
        for seed in 1..=10 {
            let g = Ceata::with_seed(seed);
            for p in [Player::One, Player::Two] {
                let mut cnt = [0u8; 12];
                for pc in g.board.iter().flatten() {
                    if pc.owner == p {
                        cnt[pc.kind as usize] += 1;
                    }
                }
                assert_eq!(cnt, COUNTS, "seed {seed}");
            }
            // steagul în rândul din spate, fiecare în zona lui
            for (i, pc) in g.board.iter().enumerate() {
                if let Some(pc) = pc {
                    let r = i / W;
                    match pc.owner {
                        Player::One => assert!(r <= 3),
                        Player::Two => assert!(r >= 6),
                    }
                    if pc.kind == FLAG {
                        assert!(r == 0 || r == 9);
                    }
                }
            }
        }
    }

    #[test]
    fn immovable_pieces_and_lakes() {
        let mut g = empty();
        put(&mut g, "c4", Player::One, 5);
        put(&mut g, "a1", Player::One, BOMB);
        put(&mut g, "b1", Player::One, FLAG);
        put(&mut g, "j10", Player::Two, 4);
        let m: Vec<String> = g.legal_moves().iter().map(|x| g.format_move(x)).collect();
        assert!(m.contains(&"c4-b4".to_string()));
        assert!(m.contains(&"c4-d4".to_string()));
        assert!(m.contains(&"c4-c3".to_string()));
        assert!(!m.contains(&"c4-c5".to_string())); // lac
        assert!(!m.iter().any(|x| x.starts_with("a1") || x.starts_with("b1")));
    }

    #[test]
    fn scout_slides_and_reveals() {
        let mut g = empty();
        put(&mut g, "a1", Player::One, SCOUT);
        put(&mut g, "a8", Player::Two, 4);
        put(&mut g, "j10", Player::Two, FLAG);
        let m: Vec<String> = g.legal_moves().iter().map(|x| g.format_move(x)).collect();
        assert!(m.contains(&"a1-a7".to_string()));
        assert!(m.contains(&"a1xa8".to_string()));
        assert!(!m.contains(&"a1-a9".to_string()));
        let mvv = mv(&g, "a1-a5");
        g.apply(&mvv);
        assert!(g.board[sq("a5") as usize].unwrap().rev);
    }

    #[test]
    fn battle_table() {
        assert_eq!(Ceata::battle(7, 5), 1);
        assert_eq!(Ceata::battle(5, 7), -1);
        assert_eq!(Ceata::battle(6, 6), 0);
        assert_eq!(Ceata::battle(SPY, MARSHAL), 1);
        assert_eq!(Ceata::battle(MARSHAL, SPY), 1);
        assert_eq!(Ceata::battle(SPY, 9), -1);
        assert_eq!(Ceata::battle(MINER, BOMB), 1);
        assert_eq!(Ceata::battle(9, BOMB), -1);
        assert_eq!(Ceata::battle(SCOUT, FLAG), 1);
        assert_eq!(Ceata::battle(SPY, SPY), 0);
    }

    #[test]
    fn attack_is_two_step_and_resolves() {
        let mut g = empty();
        put(&mut g, "e4", Player::One, 8);
        put(&mut g, "e5", Player::Two, 5);
        put(&mut g, "j10", Player::Two, FLAG);
        let a = mv(&g, "e4xe5");
        g.apply(&a);
        assert_eq!(g.legal_moves(), vec![Move::Resolve]);
        assert_eq!(g.side, Player::One);
        g.apply(&Move::Resolve);
        assert_eq!(g.side, Player::Two);
        let w = g.board[sq("e5") as usize].unwrap();
        assert_eq!((w.owner, w.kind, w.rev), (Player::One, 8, true));
        assert!(g.board[sq("e4") as usize].is_none());
        assert_eq!(g.lost[1][5], 1);
    }

    #[test]
    fn miner_defuses_bomb_others_die() {
        let mut g = empty();
        put(&mut g, "e4", Player::One, MINER);
        put(&mut g, "e5", Player::Two, BOMB);
        put(&mut g, "j10", Player::Two, FLAG);
        let a = mv(&g, "e4xe5");
        g.apply(&a);
        g.apply(&Move::Resolve);
        assert_eq!(g.board[sq("e5") as usize].unwrap().kind, MINER);
        let mut h = empty();
        put(&mut h, "e4", Player::One, 9);
        put(&mut h, "e5", Player::Two, BOMB);
        put(&mut h, "j10", Player::Two, FLAG);
        let a = mv(&h, "e4xe5");
        h.apply(&a);
        h.apply(&Move::Resolve);
        assert!(h.board[sq("e4") as usize].is_none());
        let b = h.board[sq("e5") as usize].unwrap();
        assert_eq!((b.kind, b.rev), (BOMB, true));
    }

    #[test]
    fn capturing_the_flag_wins() {
        let mut g = empty();
        put(&mut g, "e4", Player::One, 4);
        put(&mut g, "e5", Player::Two, FLAG);
        put(&mut g, "a10", Player::Two, 3);
        let a = mv(&g, "e4xe5");
        g.apply(&a);
        g.apply(&Move::Resolve);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn no_movable_pieces_loses() {
        let mut g = empty();
        put(&mut g, "a1", Player::One, FLAG);
        put(&mut g, "b1", Player::One, BOMB);
        put(&mut g, "e5", Player::Two, 4);
        put(&mut g, "j10", Player::Two, FLAG);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::Two)));
    }

    #[test]
    fn render_hides_enemy_ranks_until_revealed() {
        let g = Ceata::with_seed(3);
        let r = Game::render(&g);
        let rows: Vec<&str> = r
            .lines()
            .filter(|l| l.trim_start().starts_with(|c: char| c.is_ascii_digit()))
            .collect();
        // rândurile 7–10 (J2) au doar {?}
        for row in rows.iter().take(4) {
            assert!(row.contains("{?}"), "{row}");
            assert!(!row.contains("{X}") && !row.contains("{F}") && !row.contains("{B}"));
        }
        let own: usize = rows.iter().map(|l| l.matches('[').count()).sum();
        assert_eq!(own, 40);
    }

    #[test]
    fn evaluate_does_not_depend_on_hidden_ranks() {
        let a = Ceata::with_seed(11);
        let mut b = a.clone();
        // schimbăm secret rangurile a două piese ale lui J2 (aceeași poziție vizibilă)
        let idx: Vec<usize> = (0..100)
            .filter(|&i| matches!(b.board[i], Some(p) if p.owner == Player::Two && p.kind != FLAG && p.kind != BOMB))
            .collect();
        let (i, j) = (idx[0], idx[idx.len() - 1]);
        let (ki, kj) = (b.board[i].unwrap().kind, b.board[j].unwrap().kind);
        b.board[i].as_mut().unwrap().kind = kj;
        b.board[j].as_mut().unwrap().kind = ki;
        assert_eq!(a.evaluate(Player::One), b.evaluate(Player::One));
        // aceeași alegere a AI-ului pentru J1 (pe poziția vizibilă identică)
        assert_eq!(
            polytabletop_core::ai::best_move(&a, 3),
            polytabletop_core::ai::best_move(&b, 3)
        );
    }

    #[test]
    fn random_games_end_and_conserve_pieces() {
        for seed in 1..=20u64 {
            let mut g = Ceata::with_seed(seed);
            let mut rng = Rng::new(seed + 1);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 4000, "seed {seed}");
                for p in [Player::One, Player::Two] {
                    let lost: u32 = g.lost[p.idx()].iter().map(|&x| x as u32).sum();
                    assert_eq!(g.count_pieces(p) as u32 + lost, 40);
                }
            }
        }
    }

    #[test]
    fn ai_moves_are_legal_and_ai_beats_random() {
        let (mut ai_w, mut rnd_w) = (0, 0);
        for seed in 1..=8u64 {
            let ai = if seed % 2 == 0 {
                Player::One
            } else {
                Player::Two
            };
            let mut g = Ceata::with_seed(seed);
            let mut rng = Rng::new(seed * 3);
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
        assert!(ai_w >= rnd_w, "ai {ai_w} vs random {rnd_w}");
    }
}

#[cfg(test)]
mod viewer_tests {
    use super::*;

    #[test]
    fn viewer_sees_only_own_ranks() {
        let mut g = Ceata::with_seed(2);
        for v in [Player::One, Player::Two] {
            Game::set_viewer(&mut g, Some(v));
            let r = Game::render(&g);
            let board: String = r.lines().skip(1).collect::<Vec<_>>().join("\n");
            assert_eq!(board.matches('[').count(), g.count_pieces(v), "{r}");
            assert_eq!(
                board.matches("{?}").count(),
                g.count_pieces(v.other()),
                "{r}"
            );
        }
        Game::set_viewer(&mut g, Some(Player::One));
        let a = Game::render(&g);
        Game::set_viewer(&mut g, Some(Player::Two));
        assert_ne!(a, Game::render(&g));
        Game::set_viewer(&mut g, None); // mută J1
        assert_eq!(a, Game::render(&g));
    }
}
