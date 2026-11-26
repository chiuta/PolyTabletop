//! Yaniv (Israel), 2 jucători, din regulile publice.
//!
//! * 54 de cărți (cu 2 jokeri). Fiecare primește 5; restul = stoc, cu o carte întoarsă ca început al
//!   grămezii de aruncate. Valori: As 1, 2–10 cât scrie, J/Q/K 10, joker 0.
//! * La rândul tău: (1) dacă ai cel mult 7 puncte în mână poți spune **Yaniv** și mâna se termină; (2) altfel
//!   arunci un **set**: o carte, 2–4 cărți de același rang sau o **scară** de cel puțin 3 cărți succesive din
//!   aceeași culoare (As jos, K sus); jokerul ține loc de orice; (3) tragi o carte: din stoc sau una din
//!   **capetele** setului aruncat de adversar (la grupuri: oricare carte; la scări: prima sau ultima).
//! * Yaniv reușit (ai strict mai puțin decât adversarul): iei 0, el își adaugă punctele din mână. **Asaf**
//!   (adversarul are la fel sau mai puțin): tu iei 30 + punctele tale, el 0.
//! * Se joacă până cineva depășește 100 de puncte; câștigă cine are mai puține (egal = remiză). O mână
//!   care depășește 80 de ture se anulează.
//!
//! Mutări: `yaniv`; `5h,5s-stoc`, `5h-ia:7c` (setul aruncat – cărți despărțite prin virgulă –, apoi de unde tragi);
//! `urmatoarea` pentru mâna următoare. Jokerul se scrie `Jk`. Informație ascunsă: vezi `p3_comun`.
#![allow(
    clippy::needless_range_loop,
    clippy::type_complexity,
    clippy::collapsible_if
)]

use crate::cards::{handover_banner, Viewer};
use crate::p3_comun::{honest_eval, Honest};
use crate::q6_util::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const LIMIT: i32 = 100;
const MAX_TURNS: u32 = 80;
const MAX_HANDS: u32 = 40;
const JOKER: u8 = 52;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Play,
    HandOver,
    Over,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Take {
    Stock,
    Pile(u8),
}

#[derive(Clone, PartialEq, Debug)]
pub enum Move {
    Call,
    Play(Vec<u8>, Take),
    Next,
}

fn is_joker(c: u8) -> bool {
    c >= JOKER
}
fn rank(c: u8) -> u8 {
    c % 13 + 1
}
fn suit(c: u8) -> u8 {
    c / 13
}
fn value(c: u8) -> i32 {
    if is_joker(c) {
        0
    } else {
        rank(c).min(10) as i32
    }
}
fn total(h: &[u8]) -> i32 {
    h.iter().map(|&c| value(c)).sum()
}
fn ctext(c: u8) -> String {
    if is_joker(c) {
        return "Jk".into();
    }
    let r = match rank(c) {
        1 => "A".to_string(),
        11 => "J".into(),
        12 => "Q".into(),
        13 => "K".into(),
        n => n.to_string(),
    };
    format!("{r}{}", ['c', 'd', 'h', 's'][suit(c) as usize])
}
fn sort_cards(v: &mut [u8]) {
    v.sort_by_key(|&c| if is_joker(c) { 99 } else { c });
}

/// Ordinea de afișare a unui set valid (pentru scări: de jos în sus) sau `None` dacă nu e valid.
fn arrange(set: &[u8]) -> Option<Vec<u8>> {
    if set.len() == 1 {
        return Some(set.to_vec());
    }
    let jk: Vec<u8> = set.iter().copied().filter(|&c| is_joker(c)).collect();
    let nj: Vec<u8> = set.iter().copied().filter(|&c| !is_joker(c)).collect();
    // grup
    if nj.iter().all(|&c| rank(c) == rank(nj[0])) || nj.is_empty() {
        if set.len() <= 4 || nj.is_empty() {
            let mut v = set.to_vec();
            sort_cards(&mut v);
            return Some(v);
        }
        return None;
    }
    // scară
    if set.len() < 3 || !nj.iter().all(|&c| suit(c) == suit(nj[0])) {
        return None;
    }
    let mut rs: Vec<u8> = nj.iter().map(|&c| rank(c)).collect();
    rs.sort_unstable();
    if rs.windows(2).any(|w| w[0] == w[1]) {
        return None;
    }
    let (lo, hi) = (rs[0], *rs.last().unwrap());
    let gaps = (hi - lo + 1) as usize - rs.len();
    if jk.len() < gaps {
        return None;
    }
    let extra = jk.len() - gaps;
    // jokerii în plus se pun la capătul de sus, apoi la cel de jos
    let up = extra.min((13 - hi) as usize);
    let down = extra - up;
    if down > (lo - 1) as usize {
        return None;
    }
    let start = lo as usize - down;
    let end = hi as usize + up;
    let mut out = Vec::new();
    let mut jk_iter = jk.iter();
    for r in start..=end {
        match nj.iter().find(|&&c| rank(c) as usize == r) {
            Some(&c) => out.push(c),
            None => out.push(*jk_iter.next().unwrap()),
        }
    }
    Some(out)
}

#[derive(Clone)]
pub struct Yaniv {
    rng: Rng,
    hands: [Vec<u8>; 2],
    stock: Vec<u8>,
    /// setul aruncat ultima dată (în ordine de afișare)
    last: Vec<u8>,
    dead: Vec<u8>,
    known: [Vec<u8>; 2],
    side: Player,
    first: Player,
    scores: [i32; 2],
    gain: [i32; 2],
    phase: Phase,
    turns: u32,
    hands_played: u32,
    fresh: [u8; 2],
    prev: Player,
    log: String,
    viewer: Viewer,
}

impl Default for Yaniv {
    fn default() -> Self {
        Self::new()
    }
}

impl Yaniv {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Yaniv {
            rng: Rng::new(seed),
            hands: [Vec::new(), Vec::new()],
            stock: Vec::new(),
            last: Vec::new(),
            dead: Vec::new(),
            known: [Vec::new(), Vec::new()],
            side: Player::One,
            first: Player::One,
            scores: [0, 0],
            gain: [0, 0],
            phase: Phase::Play,
            turns: 0,
            hands_played: 0,
            fresh: [0, 0],
            prev: Player::One,
            log: String::new(),
            viewer: Viewer::Fixed(Player::One),
        };
        g.deal();
        g
    }

    pub fn scores(&self) -> (i32, i32) {
        (self.scores[0], self.scores[1])
    }

    fn deal(&mut self) {
        let mut d: Vec<u8> = (0..54).collect();
        shuffle(&mut d, &mut self.rng);
        for p in 0..2 {
            self.hands[p] = (0..5).map(|_| d.pop().unwrap()).collect();
            sort_cards(&mut self.hands[p]);
            self.known[p].clear();
        }
        let up = d.pop().unwrap();
        self.last = vec![up];
        self.dead.clear();
        self.stock = d;
        self.gain = [0, 0];
        self.fresh = [0, 0];
        self.turns = 0;
        self.phase = Phase::Play;
        self.side = self.first;
    }

    fn sets_of(&self, p: usize) -> Vec<Vec<u8>> {
        let h = &self.hands[p];
        let n = h.len();
        let mut out: Vec<Vec<u8>> = Vec::new();
        let mut seen: Vec<String> = Vec::new();
        for mask in 1u32..(1 << n) {
            let mut s: Vec<u8> = (0..n)
                .filter(|i| mask >> i & 1 == 1)
                .map(|i| h[i])
                .collect();
            if s.len() > 5 {
                continue;
            }
            if let Some(ord) = arrange(&s) {
                sort_cards(&mut s);
                let key: Vec<String> = s.iter().map(|&c| ctext(c)).collect();
                let key = key.join(",");
                if !seen.contains(&key) {
                    seen.push(key);
                    out.push(ord);
                }
            }
        }
        out
    }

    fn takes(&self) -> Vec<Take> {
        let mut v = vec![Take::Stock];
        let l = &self.last;
        let mut texts: Vec<String> = Vec::new();
        let mut add = |c: u8, v: &mut Vec<Take>| {
            let t = ctext(c);
            if !texts.contains(&t) {
                texts.push(t);
                v.push(Take::Pile(c));
            }
        };
        if l.len() == 1 {
            add(l[0], &mut v);
        } else if l.len() > 1 {
            let nj: Vec<u8> = l.iter().copied().filter(|&c| !is_joker(c)).collect();
            let group = nj.iter().all(|&c| rank(c) == rank(nj[0]));
            if group {
                for &c in l {
                    add(c, &mut v);
                }
            } else {
                add(l[0], &mut v);
                add(*l.last().unwrap(), &mut v);
            }
        }
        v
    }

    fn end_hand(&mut self, caller: Player, yaniv: bool) {
        let c = caller.idx();
        let o = 1 - c;
        let (tc, to) = (total(&self.hands[c]), total(&self.hands[o]));
        if yaniv && tc < to {
            self.gain[o] = to;
            self.log = format!(
                "Yaniv! J{} are {tc}, J{} are {to} și primește {to} puncte.",
                c + 1,
                o + 1
            );
        } else {
            self.gain[c] = 30 + tc;
            self.log = format!(
                "Asaf! J{} a spus Yaniv cu {tc}, dar J{} are {to}: J{} primește {}.",
                c + 1,
                o + 1,
                c + 1,
                30 + tc
            );
        }
        for p in 0..2 {
            self.scores[p] += self.gain[p];
        }
        self.hands_played += 1;
        self.phase =
            if self.scores[0] > LIMIT || self.scores[1] > LIMIT || self.hands_played >= MAX_HANDS {
                Phase::Over
            } else {
                Phase::HandOver
            };
    }
}

impl Game for Yaniv {
    type Move = Move;

    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = match who {
            Some(p) => Viewer::Fixed(p),
            None => Viewer::HotSeat,
        };
    }

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        match self.phase {
            Phase::Over => Vec::new(),
            Phase::HandOver => vec![Move::Next],
            Phase::Play => {
                let p = self.side.idx();
                let mut v = Vec::new();
                if total(&self.hands[p]) <= 7 {
                    v.push(Move::Call);
                }
                let takes = self.takes();
                for s in self.sets_of(p) {
                    for t in &takes {
                        v.push(Move::Play(s.clone(), t.clone()));
                    }
                }
                v
            }
        }
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        self.fresh = [0, 0];
        self.prev = self.side;
        let p = self.side.idx();
        match m {
            Move::Next => {
                self.first = self.first.other();
                self.deal();
            }
            Move::Call => {
                let who = self.side;
                self.end_hand(who, true);
            }
            Move::Play(set, take) => {
                sort_cards(&mut self.hands[p]);
                for c in set {
                    let pos = self.hands[p].iter().position(|x| x == c).unwrap();
                    self.hands[p].remove(pos);
                    if let Some(k) = self.known[p].iter().position(|x| x == c) {
                        self.known[p].remove(k);
                    }
                }
                let old_last = std::mem::take(&mut self.last);
                match take {
                    Take::Stock => {
                        if self.stock.is_empty() {
                            let mut d = std::mem::take(&mut self.dead);
                            d.extend(old_last.iter().copied());
                            shuffle(&mut d, &mut self.rng);
                            self.stock = d;
                            self.fresh[p] = 1;
                            let c = self.stock.pop().unwrap();
                            self.hands[p].push(c);
                        } else {
                            let c = self.stock.pop().unwrap();
                            self.hands[p].push(c);
                            self.fresh[p] = 1;
                            self.dead.extend(old_last.iter().copied());
                        }
                    }
                    Take::Pile(c) => {
                        let mut rest = old_last.clone();
                        let pos = rest.iter().position(|x| x == c).unwrap();
                        rest.remove(pos);
                        self.dead.extend(rest);
                        self.hands[p].push(*c);
                        self.known[p].push(*c);
                    }
                }
                self.last = arrange(set).unwrap_or_else(|| set.clone());
                self.log = format!(
                    "J{} aruncă {}",
                    p + 1,
                    set.iter().map(|&c| ctext(c)).collect::<Vec<_>>().join(" ")
                );
                self.turns += 1;
                if self.turns >= MAX_TURNS {
                    self.log.push_str(" — mână anulată (prea multe ture)");
                    self.hands_played += 1;
                    self.phase = if self.hands_played >= MAX_HANDS {
                        Phase::Over
                    } else {
                        Phase::HandOver
                    };
                    return;
                }
                self.side = self.side.other();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.phase != Phase::Over {
            return None;
        }
        Some(match self.scores[0].cmp(&self.scores[1]) {
            std::cmp::Ordering::Less => Outcome::Win(Player::One),
            std::cmp::Ordering::Greater => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        if self.phase == Phase::Over {
            return 0;
        }
        honest_eval(self, p, 12)
    }

    fn render(&self) -> String {
        let mut s = String::new();
        if self.prev != self.side && self.phase == Phase::Play {
            if let Viewer::HotSeat = self.viewer {
                s.push_str(&handover_banner(self.side));
            }
        }
        s.push_str(&format!(
            "Yaniv — scor J1 {}, J2 {} (limita {LIMIT}, mai puțin e mai bine)\n",
            self.scores[0], self.scores[1]
        ));
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        match self.phase {
            Phase::Over => {
                s.push_str("Joc încheiat.\n");
                return s;
            }
            Phase::HandOver => {
                s.push_str(&format!(
                    "Mâna s-a terminat. Cărți: J1 {} | J2 {}\nScrie «urmatoarea».\n",
                    self.hands[0]
                        .iter()
                        .map(|&c| ctext(c))
                        .collect::<Vec<_>>()
                        .join(" "),
                    self.hands[1]
                        .iter()
                        .map(|&c| ctext(c))
                        .collect::<Vec<_>>()
                        .join(" ")
                ));
                return s;
            }
            Phase::Play => {}
        }
        s.push_str(&format!(
            "Ultimul set aruncat: {} | stoc: {} cărți\n",
            self.last
                .iter()
                .map(|&c| ctext(c))
                .collect::<Vec<_>>()
                .join(" "),
            self.stock.len()
        ));
        let v = self.viewer.who(self.side);
        let mut h = self.hands[v.idx()].clone();
        sort_cards(&mut h);
        let h = &h;
        s.push_str(&format!(
            "Mâna ta (J{}): {} (total {})\n",
            v.idx() + 1,
            h.iter().map(|&c| ctext(c)).collect::<Vec<_>>().join(" "),
            total(h)
        ));
        s.push_str(&format!(
            "J{} are {} cărți\n",
            v.other().idx() + 1,
            self.hands[v.other().idx()].len()
        ));
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Next => "urmatoarea".into(),
            Move::Call => "yaniv".into(),
            Move::Play(set, take) => {
                let mut ss = set.clone();
                sort_cards(&mut ss);
                let a: Vec<String> = ss.iter().map(|&c| ctext(c)).collect();
                match take {
                    Take::Stock => format!("{}-stoc", a.join(",")),
                    Take::Pile(c) => format!("{}-ia:{}", a.join(","), ctext(*c)),
                }
            }
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

impl Honest for Yaniv {
    fn determinize(&self, me: Player, rng: &mut Rng) -> Self {
        let mut g = self.clone();
        let mi = me.idx();
        let oi = 1 - mi;
        let nf = (self.fresh[mi] as usize).min(self.hands[mi].len());
        let known_me: Vec<u8> = self.hands[mi][..self.hands[mi].len() - nf].to_vec();
        // cărți nevăzute = toate − mâna mea cunoscută − aruncate/moarte − cărțile cunoscute ale adversarului
        let mut pool: Vec<u8> = (0..54).collect();
        pool.retain(|c| {
            !known_me.contains(c)
                && !self.last.contains(c)
                && !self.dead.contains(c)
                && !self.known[oi].contains(c)
        });
        shuffle(&mut pool, rng);
        let n_opp = self.hands[oi].len();
        let mut mine = known_me;
        for _ in 0..nf {
            if let Some(c) = pool.pop() {
                mine.push(c);
            }
        }
        sort_cards(&mut mine);
        let mut opp = self.known[oi].clone();
        while opp.len() < n_opp {
            match pool.pop() {
                Some(c) => opp.push(c),
                None => break,
            }
        }
        sort_cards(&mut opp);
        g.hands[mi] = mine;
        g.hands[oi] = opp;
        g.stock = pool;
        g.fresh = [0, 0];
        g.rng = Rng::new(rng.next_u64());
        g
    }

    fn seed_state(&self, me: Player) -> Self {
        let mut c = self.clone();
        let nf = (c.fresh[me.idx()] as usize).min(c.hands[me.idx()].len());
        let k = c.hands[me.idx()].len() - nf;
        // cărțile proaspăt trase sunt cele adăugate la sfârșitul sortării: le scoatem după valoare nu e posibil;
        // folosim doar lungimea ca să nu depindă de vârful stocului
        c.hands[me.idx()].truncate(k);
        c.fresh = [0, 0];
        c
    }

    fn forget_fresh(&mut self) {
        self.fresh = [0, 0];
    }

    fn rollout_done(&self) -> bool {
        self.phase != Phase::Play
    }

    fn rollout_value(&self, p: Player) -> f64 {
        let q = p.other();
        if self.phase == Phase::Play {
            return (total(&self.hands[q.idx()]) - total(&self.hands[p.idx()])) as f64 * 0.5;
        }
        (self.gain[q.idx()] - self.gain[p.idx()]) as f64
    }

    fn policy(&self, rng: &mut Rng) -> Move {
        let mv = self.legal_moves();
        if mv.len() == 1 {
            return mv[0].clone();
        }
        let p = self.side.idx();
        if total(&self.hands[p]) <= 7 && mv.contains(&Move::Call) && rng.below(100) < 85 {
            return Move::Call;
        }
        if rng.below(100) < 15 {
            return mv[rng.below(mv.len())].clone();
        }
        let mut best = mv[0].clone();
        let mut bs = i32::MIN;
        for m in &mv {
            let sc = match m {
                Move::Play(set, take) => {
                    let mut sc = total(set) * 3 + set.len() as i32;
                    match take {
                        Take::Stock => sc -= 4,
                        Take::Pile(c) => sc += 6 - 2 * value(*c),
                    }
                    sc
                }
                _ => -100,
            };
            if sc > bs {
                bs = sc;
                best = m.clone();
            }
        }
        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sets_validation() {
        // grup
        assert!(arrange(&[4, 17]).is_some()); // 5c, 5d
        assert!(arrange(&[4, 18]).is_none()); // 5c, 6d
                                              // scară 3c 4c 5c
        assert_eq!(arrange(&[4, 2, 3]), Some(vec![2, 3, 4]));
        // scară cu joker în gaură: 3c, 5c + Jk
        assert_eq!(arrange(&[2, 4, JOKER]), Some(vec![2, JOKER, 4]));
        // fără scară pe culori diferite
        assert!(arrange(&[2, 3, 17]).is_none());
        // două cărți consecutive nu sunt set
        assert!(arrange(&[2, 3]).is_none());
        // joker + o carte = pereche
        assert!(arrange(&[5, JOKER]).is_some());
    }

    #[test]
    fn call_and_asaf_scoring() {
        let mut g = Yaniv::with_seed(3);
        g.hands[0] = vec![0, 1]; // As + 2 = 3
        g.hands[1] = vec![9, 10]; // 10 + 10
        g.side = Player::One;
        g.apply(&Move::Call);
        assert_eq!(g.scores, [0, 20]);
        let mut g = Yaniv::with_seed(3);
        g.hands[0] = vec![5, 6]; // 6+7 = 13? (>7 ale, dar forțăm)
        g.hands[1] = vec![0];
        g.side = Player::One;
        g.apply(&Move::Call);
        assert_eq!(g.scores[0], 30 + 13);
        assert_eq!(g.scores[1], 0);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=3u64 {
            let mut g = Yaniv::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                n += 1;
                assert!(n < 20_000);
            }
        }
    }
}
