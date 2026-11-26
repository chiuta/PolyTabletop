//! Daldøs (Norvegia), 2 jucători — reconstituire din regulile publice.
//!
//! * Tablă în formă de barcă cu trei rânduri: rândul fiecărui jucător (12 găuri) și rândul din mijloc (13).
//!   Fiecare are 12 piese în picioare („nedalate”) în rândul său.
//! * Două zaruri cu patru fețe (1 = „dal”, 2, 3, 4). Cu un 1 poți **dala** piesa nedalată cea mai apropiată de
//!   pupă: se culcă și avansează un pas (prima dală intră în rândul din mijloc). Piesele dalate parcurg: rândul
//!   propriu spre pupă → rândul din mijloc spre prova → rândul adversarului de la prova spre pupă → apoi din nou
//!   rândul din mijloc și tot așa, în cerc; nu se întorc în rândul propriu.
//! * Fiecare zar mută o piesă dalată; poți folosi ambele zaruri pe aceeași piesă (aterizarea de la primul zar
//!   capturează și ea, variantă dată de unele surse). Nu poți sări sau ateriza peste piese proprii; piesele
//!   adversarului se pot sări, iar o piesă adversă din gaura unde ajungi (inclusiv una „în picioare” din rândul
//!   lui) este scoasă. Trebuie să folosești ambele zaruri dacă se poate, apoi unul; altfel pierzi mutarea.
//!   Două dale (1 și 1) dau o aruncare în plus.
//! * Câștigă cine rămâne cu adversarul fără piese (după 600 de ture câștigă cine are mai multe piese).
//!
//! Mutări: `dal` (dalezi piesa de pe pupă), `h5+3` (piesa dalată din rândul propriu, gaura 5, cu zarul 3),
//! `c7+2` (piesă pe circuit, poziția 7). Rândul propriu `h0`–`h11` (h0 la prova), circuitul `c0`–`c24` (c0–c12 =
//! mijloc de la pupă la prova, c13–c24 = rândul adversarului de la prova la pupă).

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const HOME: u8 = 12;
const CYCLE: u8 = 25;
const MAX_TURNS: u32 = 600;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Pos {
    /// în picioare în gaura j a rândului propriu
    U(u8),
    /// dalată în rândul propriu
    H(u8),
    /// pe circuit
    C(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Mv {
    /// None = dal
    from: Option<Pos>,
    die: u8,
}

#[derive(Clone)]
pub struct Daldos {
    pieces: [Vec<Pos>; 2],
    side: Player,
    dice: Vec<u8>,
    rng: Rng,
    turns: u32,
    played: u8,
    extra: bool,
    log: String,
}

impl Default for Daldos {
    fn default() -> Self {
        Self::new()
    }
}

fn next(p: Pos) -> Pos {
    match p {
        Pos::U(j) | Pos::H(j) => {
            if j + 1 >= HOME {
                Pos::C(0)
            } else {
                Pos::H(j + 1)
            }
        }
        Pos::C(c) => Pos::C((c + 1) % CYCLE),
    }
}

/// Identificatorul găurii fizice: 0..13 mijloc, 13+q*12+j rândul jucătorului q.
fn hole(owner: usize, p: Pos) -> u8 {
    match p {
        Pos::U(j) | Pos::H(j) => 13 + owner as u8 * 12 + j,
        Pos::C(c) => {
            if c < 13 {
                c
            } else {
                13 + (1 - owner as u8) * 12 + (c - 13)
            }
        }
    }
}

impl Daldos {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Daldos {
            pieces: [
                (0..HOME).map(Pos::U).collect(),
                (0..HOME).map(Pos::U).collect(),
            ],
            side: Player::One,
            dice: Vec::new(),
            rng: Rng::new(seed),
            turns: 0,
            played: 0,
            extra: false,
            log: String::new(),
        };
        g.roll();
        g.settle();
        g
    }

    pub fn counts(&self) -> [usize; 2] {
        [self.pieces[0].len(), self.pieces[1].len()]
    }

    fn roll(&mut self) {
        let a = self.rng.below(4) as u8 + 1;
        let b = self.rng.below(4) as u8 + 1;
        self.dice = vec![a, b];
        self.extra = a == 1 && b == 1;
        self.played = 0;
        self.turns += 1;
        self.log = format!("Jucătorul {} aruncă {a} și {b}.", self.side.idx() + 1);
    }

    fn raw_moves(&self) -> Vec<Mv> {
        let me = self.side.idx();
        let mut out = Vec::new();
        let mut ds = self.dice.clone();
        ds.sort_unstable();
        ds.dedup();
        for d in ds {
            // dală
            if d == 1 {
                if let Some(Pos::U(j)) = self.pieces[me]
                    .iter()
                    .copied()
                    .filter(|p| matches!(p, Pos::U(_)))
                    .max_by_key(|p| if let Pos::U(j) = p { *j } else { 0 })
                {
                    let t = next(Pos::U(j));
                    let h = hole(me, t);
                    if !self.pieces[me]
                        .iter()
                        .any(|&q| !matches!(q, Pos::U(_)) && hole(me, q) == h)
                    {
                        out.push(Mv { from: None, die: 1 });
                    }
                }
            }
            for &p in &self.pieces[me] {
                if matches!(p, Pos::U(_)) {
                    continue;
                }
                // pași succesivi, fără a trece peste piese proprii
                let mut cur = p;
                let mut ok = true;
                for k in 0..d {
                    cur = next(cur);
                    let h = hole(me, cur);
                    let own = self.pieces[me]
                        .iter()
                        .any(|&q| q != p && !matches!(q, Pos::U(_)) && hole(me, q) == h);
                    let own_u = self.pieces[me]
                        .iter()
                        .any(|&q| matches!(q, Pos::U(_)) && hole(me, q) == h && q != p);
                    if own || own_u {
                        ok = false;
                        break;
                    }
                    let _ = k;
                }
                if ok {
                    out.push(Mv {
                        from: Some(p),
                        die: d,
                    });
                }
            }
        }
        out
    }

    fn step(&mut self, m: Mv) -> bool {
        let me = self.side.idx();
        let you = 1 - me;
        let mut captured = false;
        let dest;
        match m.from {
            None => {
                let j = self.pieces[me]
                    .iter()
                    .filter_map(|p| if let Pos::U(j) = p { Some(*j) } else { None })
                    .max()
                    .unwrap();
                let idx = self.pieces[me]
                    .iter()
                    .position(|&p| p == Pos::U(j))
                    .unwrap();
                dest = next(Pos::U(j));
                self.pieces[me][idx] = dest;
            }
            Some(p) => {
                let idx = self.pieces[me].iter().position(|&q| q == p).unwrap();
                let mut cur = p;
                for _ in 0..m.die {
                    cur = next(cur);
                }
                dest = cur;
                self.pieces[me][idx] = dest;
            }
        }
        let h = hole(me, dest);
        let before = self.pieces[you].len();
        self.pieces[you].retain(|&q| hole(you, q) != h);
        if self.pieces[you].len() < before {
            captured = true;
        }
        if let Some(k) = self.dice.iter().position(|&d| d == m.die) {
            self.dice.remove(k);
        }
        captured
    }

    fn max_use(&self) -> u8 {
        self.raw_moves()
            .into_iter()
            .map(|m| {
                let mut c = self.clone();
                c.step(m);
                1 + c.max_use()
            })
            .max()
            .unwrap_or(0)
    }

    fn over(&self) -> bool {
        self.pieces[0].is_empty() || self.pieces[1].is_empty() || self.turns > MAX_TURNS
    }

    fn settle(&mut self) {
        let mut guard = 0;
        while !self.over() {
            if !self.dice.is_empty() && !self.raw_moves().is_empty() {
                return;
            }
            if self.extra && self.played >= 2 {
                self.roll();
                continue;
            }
            if self.played == 0 {
                self.log
                    .push_str(&format!(" J{} nu poate muta.", self.side.idx() + 1));
            }
            self.side = self.side.other();
            self.roll();
            guard += 1;
            if guard > 5000 {
                break;
            }
        }
    }

    fn label(p: Pos) -> String {
        match p {
            Pos::U(j) => format!("u{j}"),
            Pos::H(j) => format!("h{j}"),
            Pos::C(c) => format!("c{c}"),
        }
    }
}

impl Game for Daldos {
    type Move = Mv;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Mv> {
        if self.over() {
            return Vec::new();
        }
        let raw = self.raw_moves();
        if raw.is_empty() {
            return raw;
        }
        let scored: Vec<(Mv, u8)> = raw
            .into_iter()
            .map(|m| {
                let mut c = self.clone();
                c.step(m);
                (m, 1 + c.max_use())
            })
            .collect();
        let best = scored.iter().map(|x| x.1).max().unwrap_or(0);
        let mut out: Vec<Mv> = Vec::new();
        let mut seen: Vec<String> = Vec::new();
        for (m, k) in scored {
            if k == best {
                let t = self.text(&m);
                if !seen.contains(&t) {
                    seen.push(t);
                    out.push(m);
                }
            }
        }
        out
    }

    fn apply(&mut self, m: &Mv) {
        let who = self.side.idx() + 1;
        let cap = self.step(*m);
        self.played += 1;
        self.log = format!(
            "J{who}: {}{}.",
            self.text(m),
            if cap { " (capturează)" } else { "" }
        );
        if self.over() {
            return;
        }
        self.settle();
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over() {
            return None;
        }
        let (a, b) = (self.pieces[0].len(), self.pieces[1].len());
        Some(match a.cmp(&b) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        let (a, b) = (
            self.pieces[p.idx()].len() as i32,
            self.pieces[p.other().idx()].len() as i32,
        );
        let prog = |q: usize| -> i32 {
            self.pieces[q]
                .iter()
                .map(|&x| match x {
                    Pos::U(_) => 0,
                    Pos::H(j) => j as i32 + 1,
                    Pos::C(c) => 12 + c as i32,
                })
                .sum()
        };
        (a - b) * 100 + prog(p.idx()) - prog(p.other().idx())
    }

    fn render(&self) -> String {
        let mut s = String::from("Daldøs — rândul propriu (h/u 0..11), circuit (c0..c24)\n");
        for q in 0..2 {
            let mut v: Vec<String> = self.pieces[q].iter().map(|&p| Self::label(p)).collect();
            v.sort();
            s.push_str(&format!(
                "J{} ({} piese): {}\n",
                q + 1,
                v.len(),
                v.join(" ")
            ));
        }
        s.push_str("(u = în picioare, h = dalată în rândul propriu; c0–c12 mijloc, c13–c24 rândul adversarului)\n");
        if !self.log.is_empty() {
            s.push_str(&format!("{}\n", self.log));
        }
        match self.outcome() {
            Some(Outcome::Win(p)) => s.push_str(&format!("Jucătorul {} câștigă.\n", p.idx() + 1)),
            Some(Outcome::Draw) => s.push_str("Remiză.\n"),
            None => s.push_str(&format!(
                "La mutare: Jucătorul {}. Zaruri rămase: {:?}\n",
                self.side.idx() + 1,
                self.dice
            )),
        }
        s
    }

    fn format_move(&self, m: &Mv) -> String {
        self.text(m)
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

impl Daldos {
    fn text(&self, m: &Mv) -> String {
        match m.from {
            None => "dal".into(),
            Some(p) => format!("{}+{}", Self::label(p), m.die),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dal_needs_a_one_and_enters_the_middle() {
        let mut g = Daldos::with_seed(1);
        g.dice = vec![1, 3];
        g.side = Player::One;
        g.pieces = [vec![Pos::U(10), Pos::U(11)], vec![Pos::U(5)]];
        let txt: Vec<String> = g.legal_moves().iter().map(|m| g.format_move(m)).collect();
        assert_eq!(txt, vec!["dal".to_string()]);
        let m = g.legal_moves()[0];
        g.step(m);
        assert!(g.pieces[0].contains(&Pos::C(0)));
        assert!(g.pieces[0].contains(&Pos::U(10)));
    }

    #[test]
    fn capture_by_landing_and_jumping_enemies_is_free() {
        let mut g = Daldos::with_seed(1);
        g.side = Player::One;
        g.dice = vec![3];
        g.pieces = [vec![Pos::C(2)], vec![Pos::C(4), Pos::C(5)]];
        let m = g.legal_moves()[0];
        let cap = g.step(m);
        assert!(cap);
        assert_eq!(g.pieces[1], vec![Pos::C(4)]);
        assert_eq!(g.pieces[0], vec![Pos::C(5)]);
    }

    #[test]
    fn cannot_land_or_jump_over_own_pieces() {
        let mut g = Daldos::with_seed(1);
        g.side = Player::One;
        g.dice = vec![2];
        g.pieces = [vec![Pos::C(2), Pos::C(3)], vec![Pos::U(0)]];
        let txt: Vec<String> = g.legal_moves().iter().map(|m| g.format_move(m)).collect();
        // c2+2 sare peste c3 (interzis); c3+2 e liber
        assert_eq!(txt, vec!["c3+2".to_string()]);
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=4u64 {
            let mut g = Daldos::with_seed(seed);
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
