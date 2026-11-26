#![allow(clippy::needless_range_loop, clippy::manual_is_multiple_of)]
//! Ligretto — jocul de cărți „în timp real”, adaptat pe ture pentru 2 jucători.
//!
//! Fiecare jucător are un pachet de 40 de cărți (culorile r, g, b, y; valorile 1–10): o stivă **Ligretto** de 10
//! cărți (se vede doar cea de sus), **3 rânduri** de câte o carte și o **mână** de 27 de cărți din care întoarce
//! câte 3 în **grămada de deșeuri**. Pe masă se construiesc grămezi comune ascendente pe culoare (de la 1 la 10).
//! Tura ta: faci oricâte mutări de joc dorești, apoi o încheiezi cu `da` (întorci 3 cărți din mână; când mâna s-a
//! terminat, deșeurile se răstoarnă) sau `pas` (când nu mai ai ce întoarce). Mutări: `l-c2` (Ligretto → grămada 2), `l-cn` (un 1 începe
//! o grămadă nouă), `w-r1` (deșeuri → rândul 1), `r2-c1`, `r1-r3` (pe un rând se pune o carte cu valoarea mai mică
//! cu 1 și de altă culoare, sau pe un rând gol). Runda se termină când cineva golește stiva Ligretto (acela
//! câștigă) sau când ambii au făcut 24 de întoarceri; atunci câștigă cine are mai multe puncte
//! (cărți puse pe masă − 2 × cărți rămase în Ligretto). Cărțile nevăzute nu influențează AI-ul (evaluare doar pe numărători).

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const FLIPS: u8 = 24;
const COLORS: [char; 4] = ['r', 'g', 'b', 'y'];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Card {
    rank: u8,
    color: u8,
}

impl Card {
    fn text(self) -> String {
        format!("{}{}", self.rank, COLORS[self.color as usize])
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Src {
    Lig,
    Row(u8),
    Waste,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Dst {
    Pile(u8),
    NewPile,
    Row(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Play(Src, Dst),
    Flip,
    /// mutare forțată: se întorc efectiv cărțile (AI-ul nu le vede înainte de a decide)
    Reveal,
    Pass,
}

#[derive(Clone)]
struct Side {
    lig: Vec<Card>,
    rows: [Vec<Card>; 3],
    hand: Vec<Card>,
    waste: Vec<Card>,
    played: i32,
    flips: u8,
}

#[derive(Clone)]
pub struct Ligretto {
    sides: [Side; 2],
    piles: Vec<(u8, u8)>,
    /// 0 = nimic, 1 = întoarcere în așteptare, 2 = pas în așteptare
    flipping: u8,
    side: Player,
    winner: Option<Player>,
    over: bool,
    log: String,
}

impl Default for Ligretto {
    fn default() -> Self {
        Self::new()
    }
}

impl Ligretto {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mk = |rng: &mut Rng| -> Side {
            let mut d: Vec<Card> = Vec::new();
            for c in 0..4u8 {
                for r in 1..=10u8 {
                    d.push(Card { rank: r, color: c });
                }
            }
            for i in (1..d.len()).rev() {
                let j = rng.below(i + 1);
                d.swap(i, j);
            }
            let lig: Vec<Card> = d.drain(..10).collect();
            let rows = [
                vec![d.pop().unwrap()],
                vec![d.pop().unwrap()],
                vec![d.pop().unwrap()],
            ];
            Side {
                lig,
                rows,
                hand: d,
                waste: Vec::new(),
                played: 0,
                flips: 0,
            }
        };
        let a = mk(&mut rng);
        let b = mk(&mut rng);
        Ligretto {
            sides: [a, b],
            piles: Vec::new(),
            flipping: 0,
            side: Player::One,
            winner: None,
            over: false,
            log: String::new(),
        }
    }

    pub fn score(&self, p: Player) -> i32 {
        let s = &self.sides[p.idx()];
        s.played - 2 * s.lig.len() as i32
    }

    fn top(&self, p: usize, src: Src) -> Option<Card> {
        let s = &self.sides[p];
        match src {
            Src::Lig => s.lig.last().copied(),
            Src::Row(i) => s.rows[i as usize].last().copied(),
            Src::Waste => s.waste.last().copied(),
        }
    }

    fn can_pile(&self, c: Card, pile: usize) -> bool {
        let (col, top) = self.piles[pile];
        col == c.color && top + 1 == c.rank
    }
}

impl Game for Ligretto {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over {
            return Vec::new();
        }
        if self.flipping != 0 {
            return vec![Move::Reveal];
        }
        let me = self.side.idx();
        let s = &self.sides[me];
        let mut v = Vec::new();
        if s.flips < FLIPS && (!s.hand.is_empty() || !s.waste.is_empty()) {
            v.push(Move::Flip);
        } else {
            v.push(Move::Pass);
        }
        let srcs = [Src::Lig, Src::Row(0), Src::Row(1), Src::Row(2), Src::Waste];
        for src in srcs {
            let Some(c) = self.top(me, src) else { continue };
            for i in 0..self.piles.len() {
                if self.can_pile(c, i) {
                    v.push(Move::Play(src, Dst::Pile(i as u8)));
                }
            }
            if c.rank == 1 {
                v.push(Move::Play(src, Dst::NewPile));
            }
            for r in 0..3u8 {
                if src == Src::Row(r) {
                    continue;
                }
                let ok = match s.rows[r as usize].last() {
                    None => true,
                    Some(t) => t.rank == c.rank + 1 && t.color != c.color,
                };
                // a muta singura carte dintr-un rând pe un rând gol nu schimbă nimic
                let pointless = matches!(src, Src::Row(q) if s.rows[q as usize].len() == 1)
                    && s.rows[r as usize].is_empty();
                if ok && !pointless {
                    v.push(Move::Play(src, Dst::Row(r)));
                }
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        self.log.clear();
        let me = self.side.idx();
        match *m {
            Move::Play(src, dst) => {
                let c = match src {
                    Src::Lig => self.sides[me].lig.pop(),
                    Src::Row(i) => self.sides[me].rows[i as usize].pop(),
                    Src::Waste => self.sides[me].waste.pop(),
                }
                .unwrap();
                match dst {
                    Dst::Pile(i) => {
                        self.piles[i as usize].1 = c.rank;
                        self.sides[me].played += 1;
                        if c.rank == 10 {
                            self.piles.remove(i as usize);
                        }
                    }
                    Dst::NewPile => {
                        self.piles.push((c.color, 1));
                        self.sides[me].played += 1;
                    }
                    Dst::Row(r) => self.sides[me].rows[r as usize].push(c),
                }
                if self.sides[me].lig.is_empty() {
                    self.winner = Some(self.side);
                    self.over = true;
                    self.log = format!("J{} a golit stiva Ligretto!", me + 1);
                }
            }
            Move::Flip => self.flipping = 1,
            Move::Reveal => {
                let passing = self.flipping == 2;
                self.flipping = 0;
                if passing {
                    self.sides[me].flips = FLIPS;
                    self.end_turn();
                    return;
                }
                let s = &mut self.sides[me];
                if s.hand.is_empty() {
                    let mut w = std::mem::take(&mut s.waste);
                    w.reverse();
                    s.hand = w;
                }
                for _ in 0..3 {
                    if let Some(c) = s.hand.pop() {
                        s.waste.push(c);
                    }
                }
                s.flips += 1;
                self.end_turn();
            }
            Move::Pass => self.flipping = 2,
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            return None;
        }
        if let Some(w) = self.winner {
            return Some(Outcome::Win(w));
        }
        let (a, b) = (self.score(Player::One), self.score(Player::Two));
        Some(match a.cmp(&b) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        // doar informație deja văzută: numărători, rândurile și deșeurile (nu cartea nouă din Ligretto)
        let side_val = |q: usize| -> i32 {
            let s = &self.sides[q];
            let empty_rows = s.rows.iter().filter(|r| r.is_empty()).count() as i32;
            let usable = |c: &Card| -> bool {
                c.rank == 1
                    || self
                        .piles
                        .iter()
                        .any(|&(col, top)| col == c.color && top + 1 == c.rank)
            };
            let mobility = s
                .rows
                .iter()
                .filter_map(|r| r.last())
                .chain(s.waste.last())
                .filter(|c| usable(c))
                .count() as i32;
            s.played * 100 - 220 * s.lig.len() as i32 + 30 * empty_rows + 40 * mobility
        };
        let mut v = side_val(p.idx()) - side_val(p.other().idx());
        // cine și-a încheiat tura (întoarcere în așteptare) preferă asta față de mutări fără câștig
        if self.flipping != 0 && self.side == p {
            v += 1;
        }
        v
    }

    fn render(&self) -> String {
        let mut s = String::from("Ligretto\n");
        if !self.log.is_empty() {
            s.push_str(&format!("({})\n", self.log));
        }
        let piles: Vec<String> = self
            .piles
            .iter()
            .enumerate()
            .map(|(i, &(c, t))| format!("c{}:{}{}", i + 1, t, COLORS[c as usize]))
            .collect();
        s.push_str(&format!(
            "Grămezi comune: {}\n",
            if piles.is_empty() {
                "(niciuna)".into()
            } else {
                piles.join(" ")
            }
        ));
        for q in 0..2 {
            let sd = &self.sides[q];
            let rows: Vec<String> = sd
                .rows
                .iter()
                .enumerate()
                .map(|(i, r)| {
                    format!(
                        "r{}:{}",
                        i + 1,
                        r.last().map(|c| c.text()).unwrap_or_else(|| "-".into())
                    )
                })
                .collect();
            s.push_str(&format!(
                "J{}: Ligretto {} (sus {}) | {} | mână {} | deșeuri {} (sus {}) | scor {}\n",
                q + 1,
                sd.lig.len(),
                sd.lig
                    .last()
                    .map(|c| c.text())
                    .unwrap_or_else(|| "-".into()),
                rows.join(" "),
                sd.hand.len(),
                sd.waste.len(),
                sd.waste
                    .last()
                    .map(|c| c.text())
                    .unwrap_or_else(|| "-".into()),
                self.score(if q == 0 { Player::One } else { Player::Two })
            ));
        }
        if !self.over {
            s.push_str(&format!("La mutare: Jucătorul {}\n", self.side.idx() + 1));
        }
        s
    }

    fn format_move(&self, m: &Move) -> String {
        let sn = |s: &Src| match s {
            Src::Lig => "l".to_string(),
            Src::Row(i) => format!("r{}", i + 1),
            Src::Waste => "w".to_string(),
        };
        match m {
            Move::Play(s, d) => {
                let dn = match d {
                    Dst::Pile(i) => format!("c{}", i + 1),
                    Dst::NewPile => "cn".to_string(),
                    Dst::Row(i) => format!("r{}", i + 1),
                };
                format!("{}-{}", sn(s), dn)
            }
            Move::Flip => "da".into(),
            Move::Reveal => "intorc".into(),
            Move::Pass => "pas".into(),
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(0)
    }
}

impl Ligretto {
    fn end_turn(&mut self) {
        if self.sides.iter().all(|s| s.flips >= FLIPS) {
            self.over = true;
            return;
        }
        self.side = self.side.other();
        // un jucător care și-a epuizat întoarcerile doar pasează (fără mutări utile) cât timp celălalt joacă
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deal_sizes() {
        let g = Ligretto::with_seed(1);
        for s in &g.sides {
            assert_eq!(
                (
                    s.lig.len(),
                    s.hand.len(),
                    s.rows.iter().map(|r| r.len()).sum::<usize>()
                ),
                (10, 27, 3)
            );
        }
    }

    #[test]
    fn play_ace_then_two() {
        let mut g = Ligretto::with_seed(2);
        g.sides[0].lig = vec![
            Card { rank: 9, color: 1 },
            Card { rank: 2, color: 0 },
            Card { rank: 1, color: 0 },
        ];
        g.apply(&Move::Play(Src::Lig, Dst::NewPile));
        assert_eq!(g.piles, vec![(0, 1)]);
        assert!(g
            .legal_moves()
            .contains(&Move::Play(Src::Lig, Dst::Pile(0))));
        g.apply(&Move::Play(Src::Lig, Dst::Pile(0)));
        assert_eq!(g.piles[0], (0, 2));
        assert_eq!(g.score(Player::One), 2 - 2);
    }

    #[test]
    fn emptying_ligretto_wins() {
        let mut g = Ligretto::with_seed(3);
        g.sides[0].lig = vec![Card { rank: 1, color: 2 }];
        g.apply(&Move::Play(Src::Lig, Dst::NewPile));
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn row_rule() {
        let mut g = Ligretto::with_seed(4);
        g.sides[0].rows[0] = vec![Card { rank: 6, color: 0 }];
        g.sides[0].lig = vec![Card { rank: 5, color: 0 }, Card { rank: 5, color: 1 }];
        let mv = g.legal_moves();
        assert!(mv.contains(&Move::Play(Src::Lig, Dst::Row(0))));
        g.sides[0].lig = vec![Card { rank: 5, color: 0 }];
        assert!(!g.legal_moves().contains(&Move::Play(Src::Lig, Dst::Row(0))));
    }

    #[test]
    fn ai_beats_random() {
        use polytabletop_core::GameObj;
        let mut wins = 0;
        let n = 40u64;
        for seed in 1..=n {
            let mut g: Box<dyn GameObj> = Box::new(Ligretto::with_seed(seed));
            let mut rng = Rng::new(seed + 77);
            let mut k = 0;
            while g.outcome().is_none() && k < 4000 {
                if g.to_move() == Player::One {
                    let m = g.ai_move(seed).unwrap();
                    g.play(&m).unwrap();
                } else {
                    let mv = g.legal_moves();
                    g.play(&mv[rng.below(mv.len())].clone()).unwrap();
                }
                k += 1;
            }
            if g.outcome() == Some(Outcome::Win(Player::One)) {
                wins += 1;
            }
            eprintln!(
                "seed {seed} plies {k}: {}",
                g.render().lines().skip(2).collect::<Vec<_>>().join(" / ")
            );
        }
        // joc dominat de noroc: AI-ul trebuie doar să nu piardă față de jocul aleator
        assert!(wins * 100 >= 50 * n, "AI {wins}/{n}");
    }

    #[test]
    fn random_games_end() {
        for seed in 1..=6u64 {
            let mut g = Ligretto::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut k = 0;
            while g.outcome().is_none() && k < 4000 {
                let mv = g.legal_moves();
                g.apply(&mv[rng.below(mv.len())]);
                k += 1;
            }
            assert!(g.outcome().is_some(), "seed {seed} k {k}");
        }
    }
}
