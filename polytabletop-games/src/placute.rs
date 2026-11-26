//! Plăcuțe — seturi și serii pe masă (variantă originală a jocurilor cu plăcuțe numerotate), 2 jucători.
//!
//! * 106 plăci: 4 culori (r roșu, a albastru, n negru, g galben) × numerele 1–13 × 2 exemplare
//!   + 2 jokeri (`J`). Fiecare jucător primește 14, restul formează pachetul (ascuns). Începe J1.
//! * Pe masă există **serii** (≥3 plăci consecutive de aceeași culoare, ex. `r3 r4 r5`) și **grupuri**
//!   (3–4 plăci cu același număr și culori diferite, ex. `r7 a7 n7`). Un joker ține locul oricărei plăci
//!   (o combinație trebuie să aibă măcar 2 plăci reale).
//! * În tura ta poți, în orice ordine: **pune** o combinație nouă din mâna ta (`pune r3 r4 r5`),
//!   **adăuga** o placă la capătul unei serii sau la un grup (`adauga r6 la 2`; la joker:
//!   `adauga J la 2 inceput|sfarsit`), **schimba** un joker de pe masă cu placa reală pe care o
//!   înlocuiește (`schimb J din 2 cu r4`; jokerul luat trebuie jucat în aceeași tură).
//!   Prima ieșire a fiecărui jucător cere combinații noi cu suma ≥ 30 în aceeași tură (jokerul
//!   valorează cât placa înlocuită) și nu permite adăugări/schimburi.
//! * Termini tura cu `gata` (după cel puțin o placă jucată), sau tragi o placă (`trag`) dacă n-ai jucat
//!   nimic; `anuleaza` readuce masa și mâna la începutul turei, după care poți doar trage. Cu pachetul
//!   gol: `pas`. Câștigă primul care rămâne fără plăci; după două pasuri la rând, câștigă cine are
//!   mai puține puncte în mână (joker = 30).
//! * Simplificare: **fără reorganizarea liberă a mesei** (despărțiri/recombinări); doar adăugări și
//!   schimbul de jokeri.
//!
//! Informație ascunsă: `render()` arată doar mâna J1 și mărimea mâinii J2. AI-ul citește doar masa, mâna lui
//! și numărul plăcilor adversarului (placa trasă în ultima mutare nu intră în evaluarea lui).

use crate::cards::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const COLORS: [char; 4] = ['r', 'a', 'n', 'g'];
pub const OPEN_MIN: u32 = 30;
pub const JOKER_PTS: u32 = 30;

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub struct Tile {
    pub c: u8,
    pub n: u8,
    pub joker: bool,
}

impl Tile {
    fn real(c: u8, n: u8) -> Tile {
        Tile { c, n, joker: false }
    }
    fn hand_joker() -> Tile {
        Tile {
            c: 4,
            n: 0,
            joker: true,
        }
    }
    pub fn text(&self) -> String {
        if self.joker {
            "J".to_string()
        } else {
            format!("{}{}", COLORS[self.c as usize], self.n)
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Meld {
    pub run: bool,
    pub color: u8,
    /// Serie: numărul de start; grup: numărul comun.
    pub num: u8,
    pub tiles: Vec<Tile>,
}

impl Meld {
    pub fn text(&self) -> String {
        self.tiles
            .iter()
            .map(|t| t.text())
            .collect::<Vec<_>>()
            .join(" ")
    }
    pub fn value(&self) -> u32 {
        if self.run {
            (0..self.tiles.len())
                .map(|k| (self.num as usize + k) as u32)
                .sum()
        } else {
            self.num as u32 * self.tiles.len() as u32
        }
    }
    /// Verificare de invarianți (folosită în teste).
    pub fn is_valid(&self) -> bool {
        let reals = self.tiles.iter().filter(|t| !t.joker).count();
        if reals < 2 {
            return false;
        }
        if self.run {
            if self.tiles.len() < 3 || self.num < 1 || self.num as usize + self.tiles.len() - 1 > 13
            {
                return false;
            }
            self.tiles.iter().enumerate().all(|(k, t)| {
                t.n as usize == self.num as usize + k && (t.joker || t.c == self.color)
            })
        } else {
            if !(3..=4).contains(&self.tiles.len()) {
                return false;
            }
            let mut seen = [false; 4];
            for t in &self.tiles {
                if t.n != self.num {
                    return false;
                }
                if !t.joker {
                    if seen[t.c as usize] {
                        return false;
                    }
                    seen[t.c as usize] = true;
                }
            }
            true
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    New(Meld),
    /// Adaugă placa (din mână) la meld; `front` = la început (serii).
    Add(Tile, usize, bool),
    /// Schimbă jokerul de la poziția dată din meld cu placa reală din mână.
    Swap(usize, usize, Tile),
    EndTurn,
    Draw,
    Pass,
    Undo,
}

#[derive(Clone)]
struct Snap {
    hand: Vec<Tile>,
    table: Vec<Meld>,
}

#[derive(Clone)]
pub struct Placute {
    pile: Vec<Tile>,
    hands: [Vec<Tile>; 2],
    table: Vec<Meld>,
    side: Player,
    opened: [bool; 2],
    turn_value: u32,
    played: u32,
    snap: Option<Box<Snap>>,
    start_jokers: usize,
    forced_draw: bool,
    pass_streak: u8,
    fresh: [u8; 2],
    log: String,
    /// `Some(p)` = render shows `p`'s rack; `None` = hot-seat (the mover's).
    viewer: Option<Player>,
}

impl Default for Placute {
    fn default() -> Self {
        Self::new()
    }
}

fn pts(t: &Tile) -> u32 {
    if t.joker {
        JOKER_PTS
    } else {
        t.n as u32
    }
}

impl Placute {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut all = Vec::new();
        for _ in 0..2 {
            for c in 0..4u8 {
                for n in 1..=13u8 {
                    all.push(Tile::real(c, n));
                }
            }
            all.push(Tile::hand_joker());
        }
        shuffle(&mut all, &mut rng);
        let h0: Vec<Tile> = all.drain(..14).collect();
        let h1: Vec<Tile> = all.drain(..14).collect();
        let mut g = Placute {
            pile: all,
            hands: [h0, h1],
            table: Vec::new(),
            side: Player::One,
            opened: [false; 2],
            turn_value: 0,
            played: 0,
            snap: None,
            start_jokers: 0,
            forced_draw: false,
            pass_streak: 0,
            fresh: [0; 2],
            log: String::new(),
            viewer: Some(Player::One),
        };
        g.hands[0].sort();
        g.hands[1].sort();
        g.start_jokers = g.jokers_in(0);
        g
    }

    fn jokers_in(&self, p: usize) -> usize {
        self.hands[p].iter().filter(|t| t.joker).count()
    }

    pub fn hand_points(&self, p: Player) -> u32 {
        self.hands[p.idx()].iter().map(pts).sum()
    }

    pub fn hand_len(&self, p: Player) -> usize {
        self.hands[p.idx()].len()
    }

    pub fn pile_len(&self) -> usize {
        self.pile.len()
    }

    pub fn table_len(&self) -> usize {
        self.table.len()
    }

    pub fn total_tiles(&self) -> usize {
        self.pile.len()
            + self.hands[0].len()
            + self.hands[1].len()
            + self.table.iter().map(|m| m.tiles.len()).sum::<usize>()
    }

    pub fn table_valid(&self) -> bool {
        self.table.iter().all(|m| m.is_valid())
    }

    fn can_end(&self) -> bool {
        let s = self.side.idx();
        self.played > 0
            && (self.opened[s] || self.turn_value >= OPEN_MIN)
            && self.jokers_in(s) <= self.start_jokers
    }

    fn count_in(hand: &[Tile], t: &Tile) -> usize {
        hand.iter().filter(|x| **x == *t).count()
    }

    fn new_melds(&self) -> Vec<Meld> {
        let hand = &self.hands[self.side.idx()];
        let jk = hand.iter().filter(|t| t.joker).count();
        let mut out: Vec<Meld> = Vec::new();
        // serii
        for c in 0..4u8 {
            for s in 1..=11u8 {
                for len in 3..=(14 - s) {
                    let mut tiles = Vec::new();
                    let (mut need, mut reals) = (0usize, 0usize);
                    for k in 0..len {
                        let n = s + k;
                        if Self::count_in(hand, &Tile::real(c, n)) > 0 {
                            tiles.push(Tile::real(c, n));
                            reals += 1;
                        } else {
                            tiles.push(Tile {
                                c: 4,
                                n,
                                joker: true,
                            });
                            need += 1;
                        }
                    }
                    if need <= jk && reals >= 2 {
                        out.push(Meld {
                            run: true,
                            color: c,
                            num: s,
                            tiles,
                        });
                    }
                }
            }
        }
        // grupuri
        for n in 1..=13u8 {
            let avail: Vec<u8> = (0..4u8)
                .filter(|&c| Self::count_in(hand, &Tile::real(c, n)) > 0)
                .collect();
            for mask in 0u8..16 {
                let cols: Vec<u8> = (0..4u8)
                    .filter(|&c| mask & (1 << c) != 0 && avail.contains(&c))
                    .collect();
                if cols.len() != mask.count_ones() as usize || cols.len() < 2 {
                    continue;
                }
                for j in 0..=jk.min(2) {
                    let size = cols.len() + j;
                    if !(3..=4).contains(&size) {
                        continue;
                    }
                    let mut tiles: Vec<Tile> = cols.iter().map(|&c| Tile::real(c, n)).collect();
                    for _ in 0..j {
                        tiles.push(Tile {
                            c: 4,
                            n,
                            joker: true,
                        });
                    }
                    out.push(Meld {
                        run: false,
                        color: 4,
                        num: n,
                        tiles,
                    });
                }
            }
        }
        let mut seen = std::collections::HashSet::new();
        out.retain(|m| seen.insert(m.text()));
        out
    }

    fn play_moves(&self) -> Vec<Move> {
        let s = self.side.idx();
        let hand = &self.hands[s];
        let mut v: Vec<Move> = Vec::new();
        for m in self.new_melds() {
            v.push(Move::New(m));
        }
        if self.opened[s] {
            let mut distinct: Vec<Tile> = hand.clone();
            distinct.sort();
            distinct.dedup();
            for (i, m) in self.table.iter().enumerate() {
                for t in &distinct {
                    if m.run {
                        let len = m.tiles.len() as u8;
                        if t.joker {
                            if m.num >= 2 {
                                v.push(Move::Add(*t, i, true));
                            }
                            if m.num + len <= 13 {
                                v.push(Move::Add(*t, i, false));
                            }
                        } else if t.c == m.color {
                            if m.num >= 2 && t.n == m.num - 1 {
                                v.push(Move::Add(*t, i, true));
                            }
                            if m.num + len <= 13 && t.n == m.num + len {
                                v.push(Move::Add(*t, i, false));
                            }
                        }
                    } else if m.tiles.len() < 4
                        && (t.joker
                            || (t.n == m.num && !m.tiles.iter().any(|x| !x.joker && x.c == t.c)))
                    {
                        v.push(Move::Add(*t, i, false));
                    }
                }
                // schimb de joker
                for (k, x) in m.tiles.iter().enumerate() {
                    if !x.joker {
                        continue;
                    }
                    for t in &distinct {
                        if t.joker {
                            continue;
                        }
                        let ok = if m.run {
                            t.c == m.color && t.n as usize == m.num as usize + k
                        } else {
                            t.n == m.num && !m.tiles.iter().any(|y| !y.joker && y.c == t.c)
                        };
                        if ok {
                            v.push(Move::Swap(i, k, *t));
                        }
                    }
                }
            }
        }
        v
    }

    fn take_snapshot(&mut self) {
        if self.snap.is_none() {
            self.snap = Some(Box::new(Snap {
                hand: self.hands[self.side.idx()].clone(),
                table: self.table.clone(),
            }));
        }
    }

    fn remove_from_hand(&mut self, t: &Tile) {
        let s = self.side.idx();
        let key = if t.joker { Tile::hand_joker() } else { *t };
        let pos = self.hands[s]
            .iter()
            .position(|x| *x == key)
            .expect("tile in hand");
        self.hands[s].remove(pos);
    }

    fn next_turn(&mut self) {
        self.side = self.side.other();
        self.turn_value = 0;
        self.played = 0;
        self.snap = None;
        self.forced_draw = false;
        self.start_jokers = self.jokers_in(self.side.idx());
    }

    fn all_pass_final(&self) -> Option<Outcome> {
        let (a, b) = (self.hand_points(Player::One), self.hand_points(Player::Two));
        Some(match a.cmp(&b) {
            std::cmp::Ordering::Less => Outcome::Win(Player::One),
            std::cmp::Ordering::Greater => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }
}

impl Game for Placute {
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
        let mut v = Vec::new();
        if self.forced_draw {
            v.push(if self.pile.is_empty() {
                Move::Pass
            } else {
                Move::Draw
            });
            return v;
        }
        v.extend(self.play_moves());
        if self.played == 0 {
            v.push(if self.pile.is_empty() {
                Move::Pass
            } else {
                Move::Draw
            });
        } else {
            v.push(Move::Undo);
            if self.can_end() {
                v.push(Move::EndTurn);
            }
        }
        // Identical tiles (e.g. two jokers in one meld) yield identical move texts.
        let mut seen = std::collections::HashSet::new();
        v.retain(|m| seen.insert(self.format_move(m)));
        v
    }

    fn apply(&mut self, m: &Move) {
        let s = self.side.idx();
        if self.fresh[s] > 0 {
            self.fresh[s] = 0;
            self.hands[s].sort();
        }
        match m {
            Move::New(meld) => {
                self.take_snapshot();
                for t in &meld.tiles {
                    self.remove_from_hand(t);
                }
                self.turn_value += meld.value();
                self.played += 1;
                self.log = format!("J{}: a pus {}.", s + 1, meld.text());
                self.table.push(meld.clone());
            }
            Move::Add(t, i, front) => {
                self.take_snapshot();
                self.remove_from_hand(t);
                let m = &mut self.table[*i];
                if m.run {
                    if *front {
                        m.num -= 1;
                        m.tiles.insert(
                            0,
                            Tile {
                                c: if t.joker { 4 } else { t.c },
                                n: m.num,
                                joker: t.joker,
                            },
                        );
                    } else {
                        let n = m.num + m.tiles.len() as u8;
                        m.tiles.push(Tile {
                            c: if t.joker { 4 } else { t.c },
                            n,
                            joker: t.joker,
                        });
                    }
                } else {
                    let n = m.num;
                    m.tiles.push(Tile {
                        c: if t.joker { 4 } else { t.c },
                        n,
                        joker: t.joker,
                    });
                }
                self.played += 1;
                self.log = format!("J{}: a adăugat {} la masa {}.", s + 1, t.text(), i + 1);
            }
            Move::Swap(i, k, t) => {
                self.take_snapshot();
                self.remove_from_hand(t);
                self.table[*i].tiles[*k] = *t;
                self.hands[s].push(Tile::hand_joker());
                self.hands[s].sort();
                self.played += 1;
                self.log = format!(
                    "J{}: a schimbat jokerul din {} cu {}.",
                    s + 1,
                    i + 1,
                    t.text()
                );
            }
            Move::Undo => {
                if let Some(sn) = self.snap.take() {
                    self.hands[s] = sn.hand;
                    self.table = sn.table;
                }
                self.played = 0;
                self.turn_value = 0;
                self.forced_draw = true;
                self.log = format!("J{}: a anulat tura.", s + 1);
            }
            Move::EndTurn => {
                if self.turn_value >= OPEN_MIN {
                    self.opened[s] = true;
                }
                self.pass_streak = 0;
                self.log = format!("J{}: a încheiat tura.", s + 1);
                self.next_turn();
            }
            Move::Draw => {
                if let Some(t) = self.pile.pop() {
                    self.hands[s].push(t);
                    self.fresh[s] = 1;
                }
                self.pass_streak = 0;
                self.log = format!("J{}: a tras o placă.", s + 1);
                self.next_turn();
            }
            Move::Pass => {
                self.pass_streak += 1;
                self.log = format!("J{}: pas.", s + 1);
                self.next_turn();
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        for (k, p) in [Player::One, Player::Two].into_iter().enumerate() {
            if self.hands[k].is_empty()
                && (k != self.side.idx() || self.opened[k] || self.turn_value >= OPEN_MIN)
            {
                // a rămas fără plăci: câștigă (verificarea regulii de deschidere pentru cel la mutare)
                if k == self.side.idx() && self.jokers_in(k) > self.start_jokers {
                    continue;
                }
                return Some(Outcome::Win(p));
            }
        }
        if self.pass_streak >= 2 {
            return self.all_pass_final();
        }
        None
    }

    fn evaluate(&self, p: Player) -> i32 {
        let i = p.idx();
        let o = p.other().idx();
        // doar plăcile pe care p le vede: fără cea trasă ultima dată
        let visible = self.hands[i].len() - self.fresh[i] as usize;
        let mut mine: i32 = self.hands[i][..visible].iter().map(|t| pts(t) as i32).sum();
        mine += self.fresh[i] as i32 * 7;
        let theirs = self.hands[o].len() as i32 * 7;
        let mut v = theirs - mine;
        if !self.opened[i] && self.side == p {
            v += self.turn_value.min(OPEN_MIN) as i32;
        }
        v
    }

    fn render(&self) -> String {
        let mut s = String::from(
            "Plăcuțe (culori: r roșu, a albastru, n negru, g galben; J = joker)\nMasa:\n",
        );
        if self.table.is_empty() {
            s.push_str("  (goală)\n");
        }
        for (i, m) in self.table.iter().enumerate() {
            s.push_str(&format!(
                "  {}: {} {}\n",
                i + 1,
                m.text(),
                if m.run { "(serie)" } else { "(grup)" }
            ));
        }
        let v = self.viewer.unwrap_or(self.side);
        let (vi, oi) = (v.idx(), v.other().idx());
        let mut hand = self.hands[vi].clone();
        let fresh = self.fresh[vi] as usize;
        let tail: Vec<Tile> = hand.split_off(hand.len() - fresh);
        hand.sort();
        hand.extend(tail);
        s.push_str(&format!(
            "Mâna ta (J{}, {}): {}\n",
            vi + 1,
            hand.len(),
            hand.iter().map(|t| t.text()).collect::<Vec<_>>().join(" ")
        ));
        if self.outcome().is_some() {
            let h: Vec<String> = self.hands[oi].iter().map(|t| t.text()).collect();
            s.push_str(&format!("Mâna lui J{}: {}\n", oi + 1, h.join(" ")));
        } else {
            s.push_str(&format!(
                "J{}: {} plăci   pachet: {}   deschis: J1 {} J2 {}\n",
                oi + 1,
                self.hands[oi].len(),
                self.pile.len(),
                if self.opened[0] { "da" } else { "nu" },
                if self.opened[1] { "da" } else { "nu" }
            ));
            s.push_str(&format!(
                "La mutare: jucătorul {} (puse în tura asta: {}, suma combinațiilor noi: {})\n",
                self.side.idx() + 1,
                self.played,
                self.turn_value
            ));
        }
        s.push_str(self.log.trim());
        s.push('\n');
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::New(meld) => format!("pune {}", meld.text()),
            Move::Add(t, i, front) => {
                if t.joker {
                    if self.table[*i].run {
                        format!(
                            "adauga J la {} {}",
                            i + 1,
                            if *front { "inceput" } else { "sfarsit" }
                        )
                    } else {
                        format!("adauga J la {}", i + 1)
                    }
                } else {
                    format!("adauga {} la {}", t.text(), i + 1)
                }
            }
            Move::Swap(i, _, t) => {
                format!("schimb J din {} cu {}", i + 1, t.text())
            }
            Move::EndTurn => "gata".into(),
            Move::Draw => "trag".into(),
            Move::Pass => "pas".into(),
            Move::Undo => "anuleaza".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn t(c: u8, n: u8) -> Tile {
        Tile::real(c, n)
    }

    fn set_hand(g: &mut Placute, tiles: &[Tile]) {
        g.hands[0] = tiles.to_vec();
        g.hands[0].sort();
        g.start_jokers = g.jokers_in(0);
    }

    fn texts(g: &Placute) -> Vec<String> {
        g.legal_moves().iter().map(|m| g.format_move(m)).collect()
    }

    #[test]
    fn setup_has_106_tiles_and_14_each() {
        let g = Placute::with_seed(1);
        assert_eq!(g.total_tiles(), 106);
        assert_eq!(g.hands[0].len(), 14);
        assert_eq!(g.hands[1].len(), 14);
        assert_eq!(g.pile.len(), 78);
        assert_eq!(
            g.pile
                .iter()
                .chain(&g.hands[0])
                .chain(&g.hands[1])
                .filter(|x| x.joker)
                .count(),
            2
        );
    }

    #[test]
    fn meld_validation() {
        let run = Meld {
            run: true,
            color: 0,
            num: 3,
            tiles: vec![t(0, 3), t(0, 4), t(0, 5)],
        };
        assert!(run.is_valid());
        let bad = Meld {
            run: true,
            color: 0,
            num: 3,
            tiles: vec![t(0, 3), t(1, 4), t(0, 5)],
        };
        assert!(!bad.is_valid());
        let g = Meld {
            run: false,
            color: 4,
            num: 7,
            tiles: vec![t(0, 7), t(1, 7), t(2, 7)],
        };
        assert!(g.is_valid());
        let g2 = Meld {
            run: false,
            color: 4,
            num: 7,
            tiles: vec![t(0, 7), t(0, 7), t(2, 7)],
        };
        assert!(!g2.is_valid());
        let jj = Meld {
            run: false,
            color: 4,
            num: 7,
            tiles: vec![
                t(0, 7),
                Tile {
                    c: 4,
                    n: 7,
                    joker: true,
                },
                Tile {
                    c: 4,
                    n: 7,
                    joker: true,
                },
            ],
        };
        assert!(!jj.is_valid());
    }

    #[test]
    fn generates_runs_groups_and_joker_melds() {
        let mut g = Placute::with_seed(1);
        set_hand(
            &mut g,
            &[
                t(0, 3),
                t(0, 4),
                t(0, 6),
                t(1, 6),
                t(2, 6),
                Tile::hand_joker(),
                t(3, 12),
            ],
        );
        let m = texts(&g);
        assert!(m.contains(&"pune r6 a6 n6".to_string()));
        assert!(m.contains(&"pune r3 r4 J".to_string())); // joker = r5
        assert!(m.contains(&"pune r3 r4 J r6".to_string()));
        assert!(m.contains(&"pune r6 a6 J".to_string()));
        assert!(m.contains(&"pune r6 a6 n6 J".to_string()));
        assert!(m.contains(&"trag".to_string()));
        assert!(!m.iter().any(|x| x.starts_with("pune g12")));
        // fără dubluri
        let mut s = m.clone();
        s.sort();
        s.dedup();
        assert_eq!(s.len(), m.len());
    }

    #[test]
    fn opening_requires_thirty() {
        let mut g = Placute::with_seed(1);
        set_hand(
            &mut g,
            &[
                t(0, 3),
                t(0, 4),
                t(0, 5),
                t(1, 9),
                t(2, 9),
                t(3, 9),
                t(0, 13),
            ],
        );
        g.apply(
            &g.legal_moves()
                .into_iter()
                .find(|m| g.format_move(m) == "pune r3 r4 r5")
                .unwrap(),
        );
        // 12 < 30: nu poți încheia
        assert!(!texts(&g).contains(&"gata".to_string()));
        assert!(texts(&g).contains(&"anuleaza".to_string()));
        let m = g
            .legal_moves()
            .into_iter()
            .find(|m| g.format_move(m) == "pune a9 n9 g9")
            .unwrap();
        g.apply(&m);
        // 12 + 27 = 39 ≥ 30
        assert!(texts(&g).contains(&"gata".to_string()));
        g.apply(&Move::EndTurn);
        assert!(g.opened[0]);
        assert_eq!(g.side, Player::Two);
        assert_eq!(g.table.len(), 2);
    }

    #[test]
    fn cannot_add_to_table_before_opening() {
        let mut g = Placute::with_seed(1);
        g.table.push(Meld {
            run: true,
            color: 0,
            num: 3,
            tiles: vec![t(0, 3), t(0, 4), t(0, 5)],
        });
        set_hand(&mut g, &[t(0, 6), t(1, 1), t(2, 2)]);
        assert!(!texts(&g).iter().any(|x| x.starts_with("adauga")));
        g.opened[0] = true;
        assert!(texts(&g).contains(&"adauga r6 la 1".to_string()));
    }

    #[test]
    fn add_to_run_group_and_joker_endpoints() {
        let mut g = Placute::with_seed(1);
        g.opened[0] = true;
        g.table.push(Meld {
            run: true,
            color: 1,
            num: 5,
            tiles: vec![t(1, 5), t(1, 6), t(1, 7)],
        });
        g.table.push(Meld {
            run: false,
            color: 4,
            num: 9,
            tiles: vec![t(0, 9), t(1, 9), t(2, 9)],
        });
        set_hand(
            &mut g,
            &[
                t(1, 4),
                t(1, 8),
                t(3, 9),
                t(0, 9),
                Tile::hand_joker(),
                t(2, 1),
            ],
        );
        let m = texts(&g);
        assert!(m.contains(&"adauga a4 la 1".to_string()));
        assert!(m.contains(&"adauga a8 la 1".to_string()));
        assert!(m.contains(&"adauga g9 la 2".to_string()));
        assert!(!m.contains(&"adauga r9 la 2".to_string())); // culoare deja prezentă
        assert!(m.contains(&"adauga J la 1 inceput".to_string()));
        assert!(m.contains(&"adauga J la 1 sfarsit".to_string()));
        assert!(m.contains(&"adauga J la 2".to_string()));
        let mv = g
            .legal_moves()
            .into_iter()
            .find(|m| g.format_move(m) == "adauga a4 la 1")
            .unwrap();
        g.apply(&mv);
        assert_eq!(g.table[0].num, 4);
        assert_eq!(g.table[0].tiles.len(), 4);
        assert!(g.table_valid());
        let mv = g
            .legal_moves()
            .into_iter()
            .find(|m| g.format_move(m) == "adauga g9 la 2")
            .unwrap();
        g.apply(&mv);
        // grupul are acum 4: nu mai primește altă placă
        assert!(!texts(&g)
            .iter()
            .any(|x| x.ends_with("la 2") || x.starts_with("adauga J la 2")));
    }

    #[test]
    fn joker_swap_must_be_replayed() {
        let mut g = Placute::with_seed(1);
        g.opened[0] = true;
        g.table.push(Meld {
            run: true,
            color: 0,
            num: 3,
            tiles: vec![
                t(0, 3),
                Tile {
                    c: 4,
                    n: 4,
                    joker: true,
                },
                t(0, 5),
            ],
        });
        set_hand(&mut g, &[t(0, 4), t(1, 1)]);
        assert!(texts(&g).contains(&"schimb J din 1 cu r4".to_string()));
        let mv = g
            .legal_moves()
            .into_iter()
            .find(|m| g.format_move(m) == "schimb J din 1 cu r4")
            .unwrap();
        g.apply(&mv);
        assert!(g.table[0].is_valid());
        assert_eq!(g.jokers_in(0), 1);
        // jokerul trebuie jucat înainte de „gata”
        assert!(!texts(&g).contains(&"gata".to_string()));
        let mv = g
            .legal_moves()
            .into_iter()
            .find(|m| g.format_move(m) == "adauga J la 1 sfarsit")
            .unwrap();
        g.apply(&mv);
        assert!(texts(&g).contains(&"gata".to_string()));
    }

    #[test]
    fn undo_restores_and_forces_a_draw() {
        let mut g = Placute::with_seed(2);
        let hand_before = g.hands[0].clone();
        let pile_before = g.pile.len();
        let m = g
            .legal_moves()
            .into_iter()
            .find(|m| matches!(m, Move::New(_)));
        if let Some(m) = m {
            g.apply(&m);
            assert!(g.hands[0].len() < 14);
            g.apply(&Move::Undo);
            assert_eq!(g.hands[0], hand_before);
            assert!(g.table.is_empty());
            assert_eq!(g.legal_moves(), vec![Move::Draw]);
            g.apply(&Move::Draw);
            assert_eq!(g.pile.len(), pile_before - 1);
            assert_eq!(g.hands[0].len(), 15);
        }
    }

    #[test]
    fn emptying_the_hand_wins_only_when_legitimate() {
        let mut g = Placute::with_seed(1);
        set_hand(&mut g, &[t(0, 10), t(0, 11), t(0, 12)]);
        g.apply(
            &g.legal_moves()
                .into_iter()
                .find(|m| g.format_move(m) == "pune r10 r11 r12")
                .unwrap(),
        );
        // 33 ≥ 30 și mâna e goală
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
        let mut h = Placute::with_seed(1);
        set_hand(&mut h, &[t(0, 1), t(0, 2), t(0, 3)]);
        h.apply(
            &h.legal_moves()
                .into_iter()
                .find(|m| h.format_move(m) == "pune r1 r2 r3")
                .unwrap(),
        );
        // mâna goală dar suma 6 < 30: nu e victorie, se poate anula
        assert!(h.outcome().is_none());
        assert!(texts(&h).contains(&"anuleaza".to_string()));
    }

    #[test]
    fn two_passes_with_empty_pile_end_the_game_by_points() {
        let mut g = Placute::with_seed(3);
        g.pile.clear();
        g.hands[0] = vec![t(0, 1)];
        g.hands[1] = vec![t(0, 5), t(1, 5)];
        assert_eq!(g.legal_moves(), vec![Move::Pass]);
        g.apply(&Move::Pass);
        g.apply(&Move::Pass);
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn evaluation_ignores_the_tile_just_drawn() {
        let mut a = Placute::with_seed(5);
        let mut b = a.clone();
        // două pachete diferite cu placa de deasupra diferită
        a.pile.push(t(0, 1));
        b.pile.push(t(3, 13));
        a.apply(&Move::Draw);
        b.apply(&Move::Draw);
        assert_eq!(a.evaluate(Player::One), b.evaluate(Player::One));
    }

    #[test]
    fn random_games_conserve_tiles_keep_table_valid_and_end() {
        for seed in 1..=15u64 {
            let mut g = Placute::with_seed(seed);
            let mut rng = Rng::new(seed + 99);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty(), "seed {seed}\n{}", Game::render(&g));
                g.apply(&m[rng.below(m.len())]);
                assert_eq!(g.total_tiles(), 106);
                assert!(g.table_valid(), "seed {seed}");
                n += 1;
                assert!(n < 4000, "seed {seed}");
            }
        }
    }

    #[test]
    fn ai_games_end_and_ai_beats_random_sometimes() {
        let mut ai_w = 0;
        let mut rnd_w = 0;
        for seed in 1..=10u64 {
            let ai = if seed % 2 == 0 {
                Player::One
            } else {
                Player::Two
            };
            let mut g = Placute::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = if g.side == ai {
                    polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap()
                } else {
                    let m = g.legal_moves();
                    m[rng.below(m.len())].clone()
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
    fn viewer_sees_only_own_rack() {
        let mut g = Placute::with_seed(4);
        Game::set_viewer(&mut g, Some(Player::One));
        let r = Game::render(&g);
        assert!(
            r.contains("Mâna ta (J1,") && !r.contains("Mâna ta (J2,"),
            "{r}"
        );
        assert!(
            r.contains(&format!("J2: {} plăci", g.hands[1].len())),
            "{r}"
        );
        Game::set_viewer(&mut g, Some(Player::Two));
        let r2 = Game::render(&g);
        assert!(
            r2.contains("Mâna ta (J2,") && !r2.contains("Mâna ta (J1,"),
            "{r2}"
        );
        assert!(
            r2.contains(&format!("J1: {} plăci", g.hands[0].len())),
            "{r2}"
        );
        let tiles = |h: &Vec<Tile>| h.iter().map(|t| t.text()).collect::<Vec<_>>();
        let line = r2.lines().find(|l| l.starts_with("Mâna ta")).unwrap();
        let mut got: Vec<&str> = line.split(": ").nth(1).unwrap().split(' ').collect();
        let mut want = tiles(&g.hands[1]);
        got.sort();
        want.sort();
        assert_eq!(got, want.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    }
}
