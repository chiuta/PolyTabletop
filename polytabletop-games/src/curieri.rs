//! Curieri — joc de deducție cu 16 cărți, variantă originală (mecanică „o carte în mână”).
//!
//! Opt feluri de curieri la o curte regală; fiecare tură tragi o carte, apoi joci una din
//! cele două și aplici efectul:
//! 1 Iscoadă (×5): ghicești cartea adversarului (nu 1); dacă ai ghicit, iese din rundă.
//! 2 Crainic (×2): te uiți la cartea adversarului.
//! 3 Judecător (×2): compari cărțile; cea mai mică iese.
//! 4 Străjer (×2): ești ferit până la tura ta următoare.
//! 5 Mag (×2): un jucător (și tu) aruncă cartea din mână și trage alta; dacă aruncă 8, iese.
//! 6 Cămăraș (×1): schimbi mâinile.
//! 7 Contesă (×1): trebuie jucată dacă ții 5 sau 6.
//! 8 Prințesă (×1): dacă o arunci, ieși.
//! Se scoate 1 carte la întâmplare (ascunsă) și 3 cărți descoperite (la doi jucători).
//! Rundă: rămâi ultimul sau, la golirea pachetului, ai cartea cea mai mare (egal: suma
//! aruncatelor). Meci: primul cu 3 runde câștigate.
//!
//! Informație ascunsă: vezi doar cartea ta. AI-ul nu citește mâna adversarului: alege mutarea
//! după o euristică calculată din ce știe (cartea lui, aruncatele publice, ce a aflat de la
//! Crainic/Cămăraș), nu după rezultatul real al mutării.
#![allow(clippy::needless_range_loop, clippy::doc_lazy_continuation)]

use crate::cards::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const NAMES: [&str; 9] = [
    "",
    "Iscoadă",
    "Crainic",
    "Judecător",
    "Străjer",
    "Mag",
    "Cămăraș",
    "Contesă",
    "Prințesă",
];
const COUNTS: [usize; 9] = [0, 5, 2, 2, 2, 2, 1, 1, 1];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    /// Joacă cartea de valoare `card`; `arg` = ghicirea (Iscoadă) sau 1 = ținta e adversarul (Mag).
    Play {
        card: u8,
        arg: u8,
    },
    Settle,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Play,
    Settle,
}

#[derive(Clone)]
pub struct Curieri {
    /// Whose private info `render` shows (`None` = the mover's, hot-seat).
    viewer: Option<Player>,
    deck: Vec<u8>,
    hidden: Option<u8>,
    faceup: Vec<u8>,
    hands: [Vec<u8>; 2],
    discards: [Vec<u8>; 2],
    protected: [bool; 2],
    out: [bool; 2],
    tokens: [u8; 2],
    know: [Option<u8>; 2],
    side: Player,
    phase: Phase,
    over: bool,
    rng: Rng,
    heur: (usize, i32),
    round: u32,
}

impl Default for Curieri {
    fn default() -> Self {
        Self::new()
    }
}

impl Curieri {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut g = Curieri {
            viewer: None,
            deck: Vec::new(),
            hidden: None,
            faceup: Vec::new(),
            hands: [Vec::new(), Vec::new()],
            discards: [Vec::new(), Vec::new()],
            protected: [false; 2],
            out: [false; 2],
            tokens: [0; 2],
            know: [None; 2],
            side: Player::One,
            phase: Phase::Play,
            over: false,
            rng: Rng::new(seed),
            heur: (0, 0),
            round: 1,
        };
        g.new_round(Player::One);
        g
    }

    pub fn tokens(&self) -> (u8, u8) {
        (self.tokens[0], self.tokens[1])
    }

    fn new_round(&mut self, first: Player) {
        let mut d = Vec::new();
        for v in 1..=8u8 {
            for _ in 0..COUNTS[v as usize] {
                d.push(v);
            }
        }
        shuffle(&mut d, &mut self.rng);
        self.hidden = d.pop();
        self.faceup = vec![d.pop().unwrap(), d.pop().unwrap(), d.pop().unwrap()];
        self.deck = d;
        self.hands = [
            vec![self.deck.pop().unwrap()],
            vec![self.deck.pop().unwrap()],
        ];
        self.discards = [Vec::new(), Vec::new()];
        self.protected = [false; 2];
        self.out = [false; 2];
        self.know = [None; 2];
        self.phase = Phase::Play;
        self.side = first;
        let c = self.deck.pop().unwrap();
        self.hands[first.idx()].push(c);
    }

    /// Toate cărțile din rundă (conservare): pachet + ascunsă + descoperite + mâini + aruncate.
    pub fn round_cards(&self) -> usize {
        self.deck.len()
            + usize::from(self.hidden.is_some())
            + self.faceup.len()
            + self.hands.iter().map(|h| h.len()).sum::<usize>()
            + self.discards.iter().map(|h| h.len()).sum::<usize>()
    }

    fn unseen(&self, p: usize) -> [i32; 9] {
        let mut c = [0i32; 9];
        for v in 1..=8usize {
            c[v] = COUNTS[v] as i32;
        }
        for &x in &self.hands[p] {
            c[x as usize] -= 1;
        }
        for &x in &self.faceup {
            c[x as usize] -= 1;
        }
        for d in &self.discards {
            for &x in d {
                c[x as usize] -= 1;
            }
        }
        c
    }

    /// Euristică ex-ante: folosește doar ce știe `p` (nu mâna adversarului, nu pachetul).
    fn move_score(&self, p: usize, m: &Move) -> i32 {
        let o = 1 - p;
        let (card, arg) = match m {
            Move::Play { card, arg } => (*card, *arg),
            Move::Settle => return 0,
        };
        let hand = &self.hands[p];
        let r = if hand.len() == 2 {
            let i = hand.iter().position(|x| *x == card).unwrap_or(0);
            hand[1 - i] as i32
        } else {
            0
        };
        let un = self.unseen(p);
        // Distribuția cărții adversarului din punctul de vedere al lui p.
        let mut dist = [0.0f64; 9];
        if let Some(k) = self.know[p] {
            dist[k as usize] = 1.0;
        } else {
            let tot: i32 = un.iter().sum();
            for v in 1..=8usize {
                dist[v] = f64::from(un[v].max(0)) / f64::from(tot.max(1));
            }
        }
        let avg: f64 = (1..=8usize).map(|v| v as f64 * dist[v]).sum();
        let unseen_tot: i32 = un.iter().sum();
        let avg_un: f64 = (1..=8usize)
            .map(|v| v as f64 * f64::from(un[v].max(0)) / f64::from(unseen_tot.max(1)))
            .sum();
        let prot = self.protected[o];
        let mut hold = r as f64;
        let mut eff = 0.0f64;
        match card {
            1 => {
                if !prot {
                    eff = 100.0 * dist[arg as usize];
                }
            }
            2 => {
                if !prot {
                    eff = if self.know[p].is_some() { 0.0 } else { 35.0 };
                }
            }
            3 => {
                if !prot {
                    let mut win = 0.0;
                    let mut lose = 0.0;
                    for v in 1..=8usize {
                        if (v as i32) < r {
                            win += dist[v];
                        } else if (v as i32) > r {
                            lose += dist[v];
                        }
                    }
                    eff = 110.0 * win - 130.0 * lose;
                }
            }
            4 => eff = 25.0,
            5 => {
                if arg == 1 {
                    eff = 110.0 * dist[8] + 5.0 * avg;
                } else if r == 8 {
                    eff = -1000.0;
                } else {
                    hold = avg_un;
                    eff = 0.0;
                }
            }
            6 => {
                if !prot {
                    hold = if self.know[p].is_some() { avg } else { avg_un };
                    eff = 0.0;
                }
            }
            7 => eff = 8.0,
            8 => eff = -1000.0,
            _ => {}
        }
        (eff + 4.0 * hold * 1.0) as i32
    }

    fn end_check(&mut self) {
        let pending = self.out[0]
            || self.out[1]
            || self.deck.is_empty()
            || (self.hands[0].is_empty() && self.hands[1].is_empty());
        if pending {
            self.phase = Phase::Settle;
        }
    }

    fn start_turn(&mut self) {
        let s = self.side.idx();
        self.protected[s] = false;
        let c = self.deck.pop().expect("deck");
        self.hands[s].push(c);
    }

    fn draw_replace(&mut self, t: usize) {
        let c = self.deck.pop().or_else(|| self.hidden.take());
        if let Some(c) = c {
            self.hands[t].push(c);
        }
    }

    fn settle(&mut self) {
        let winner: Option<usize> = if self.out[0] != self.out[1] {
            Some(usize::from(self.out[0]))
        } else {
            let a = self.hands[0].first().copied().unwrap_or(0);
            let b = self.hands[1].first().copied().unwrap_or(0);
            if a != b {
                Some(usize::from(b > a))
            } else {
                let sa: u32 = self.discards[0].iter().map(|x| *x as u32).sum();
                let sb: u32 = self.discards[1].iter().map(|x| *x as u32).sum();
                if sa != sb {
                    Some(usize::from(sb > sa))
                } else {
                    None
                }
            }
        };
        let mut first = self.side;
        if let Some(w) = winner {
            self.tokens[w] += 1;
            first = if w == 0 { Player::One } else { Player::Two };
        }
        if self.tokens[0] >= 3 || self.tokens[1] >= 3 {
            self.over = true;
            return;
        }
        self.round += 1;
        self.new_round(first);
    }
}

impl Game for Curieri {
    fn set_viewer(&mut self, who: Option<Player>) {
        self.viewer = who;
    }
    type Move = Move;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.over {
            return Vec::new();
        }
        if self.phase == Phase::Settle {
            return vec![Move::Settle];
        }
        let s = self.side.idx();
        let o = 1 - s;
        let h = &self.hands[s];
        let mut cards: Vec<u8> = Vec::new();
        for &c in h {
            if !cards.contains(&c) {
                cards.push(c);
            }
        }
        // Contesa e obligatorie cu 5 sau 6.
        if h.contains(&7) && (h.contains(&5) || h.contains(&6)) {
            cards = vec![7];
        }
        let prot = self.protected[o];
        let mut v = Vec::new();
        for c in cards {
            match c {
                1 => {
                    if prot {
                        v.push(Move::Play { card: 1, arg: 0 });
                    } else {
                        for g in 2..=8u8 {
                            v.push(Move::Play { card: 1, arg: g });
                        }
                    }
                }
                5 => {
                    v.push(Move::Play { card: 5, arg: 0 });
                    if !prot {
                        v.push(Move::Play { card: 5, arg: 1 });
                    }
                }
                _ => v.push(Move::Play { card: c, arg: 0 }),
            }
        }
        v
    }

    fn apply(&mut self, m: &Move) {
        let s = self.side.idx();
        let o = 1 - s;
        self.heur = (s, self.move_score(s, m));
        let (card, arg) = match *m {
            Move::Settle => {
                self.settle();
                return;
            }
            Move::Play { card, arg } => (card, arg),
        };
        let i = self.hands[s].iter().position(|x| *x == card).expect("card");
        self.hands[s].remove(i);
        self.discards[s].push(card);
        let prot = self.protected[o];
        // Ce știa `o` despre mâna lui `s`: dacă s-a jucat acea carte, informația se pierde.
        if self.know[o] == Some(card) && self.hands[s].first() != Some(&card) {
            self.know[o] = None;
        }
        match card {
            1 => {
                if !prot && self.hands[o][0] == arg {
                    self.out[o] = true;
                    self.discards[o].push(arg);
                    self.hands[o].clear();
                }
            }
            2 => {
                if !prot {
                    self.know[s] = Some(self.hands[o][0]);
                }
            }
            3 => {
                if !prot {
                    let a = self.hands[s][0];
                    let b = self.hands[o][0];
                    if a < b {
                        self.out[s] = true;
                        self.discards[s].push(a);
                        self.hands[s].clear();
                    } else if b < a {
                        self.out[o] = true;
                        self.discards[o].push(b);
                        self.hands[o].clear();
                    } else {
                        self.know[s] = Some(b);
                        self.know[o] = Some(a);
                    }
                }
            }
            4 => self.protected[s] = true,
            5 => {
                let t = if arg == 1 { o } else { s };
                let d = self.hands[t].pop().expect("hand");
                self.discards[t].push(d);
                self.know[1 - t] = None;
                if d == 8 {
                    self.out[t] = true;
                } else {
                    self.draw_replace(t);
                }
            }
            6 => {
                if !prot {
                    let a = self.hands[s][0];
                    let b = self.hands[o][0];
                    self.hands[s][0] = b;
                    self.hands[o][0] = a;
                    self.know[s] = Some(a);
                    self.know[o] = Some(b);
                }
            }
            8 => {
                self.out[s] = true;
                self.hands[s].clear();
            }
            _ => {}
        }
        self.end_check();
        if self.phase == Phase::Play {
            self.side = self.side.other();
            self.start_turn();
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.over {
            return None;
        }
        Some(if self.tokens[0] > self.tokens[1] {
            Outcome::Win(Player::One)
        } else {
            Outcome::Win(Player::Two)
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        if self.heur.0 == p.idx() {
            self.heur.1
        } else {
            -self.heur.1
        }
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }

    fn render(&self) -> String {
        let mut s = String::new();
        let nm = |v: &[u8]| {
            if v.is_empty() {
                "—".to_string()
            } else {
                v.iter()
                    .map(|c| format!("{}({})", NAMES[*c as usize], c))
                    .collect::<Vec<_>>()
                    .join(" ")
            }
        };
        s.push_str(&format!(
            "Curieri — runda {} | runde câștigate: J1 {}, J2 {} | pachet: {}\n",
            self.round,
            self.tokens[0],
            self.tokens[1],
            self.deck.len()
        ));
        s.push_str(&format!("Descoperite: {}\n", nm(&self.faceup)));
        s.push_str(&format!("Aruncate J1: {}\n", nm(&self.discards[0])));
        s.push_str(&format!("Aruncate J2: {}\n", nm(&self.discards[1])));
        if self.protected[0] || self.protected[1] {
            s.push_str(&format!(
                "Feriți: J1 {}, J2 {}\n",
                if self.protected[0] { "da" } else { "nu" },
                if self.protected[1] { "da" } else { "nu" }
            ));
        }
        if self.over {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        if self.phase == Phase::Settle {
            s.push_str(&format!("Mâna ta: {}\n", nm(&self.hands[0])));
            s.push_str(&format!("Mâna adversarului: {}\n", nm(&self.hands[1])));
            s.push_str("Runda s-a încheiat: decontare.\n");
            return s;
        }
        let v = self.viewer.unwrap_or_else(|| self.to_move()).idx();
        s.push_str(&format!("Mâna ta (J{}): {}\n", v + 1, nm(&self.hands[v])));
        if let Some(k) = self.know[v] {
            s.push_str(&format!(
                "Știi că adversarul ține: {}({})\n",
                NAMES[k as usize], k
            ));
        }
        s.push_str(&format!(
            "Adversarul are {} cărți.\n",
            self.hands[1 - v].len()
        ));
        s.push_str("Mutări: iscoada:5, crainic, judecator, strajer, mag:eu / mag:el, camaras, contesa, printesa.\n");
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match *m {
            Move::Settle => "decontare".into(),
            Move::Play { card, arg } => match card {
                1 => {
                    if arg == 0 {
                        "iscoada".into()
                    } else {
                        format!("iscoada:{arg}")
                    }
                }
                2 => "crainic".into(),
                3 => "judecator".into(),
                4 => "strajer".into(),
                5 => if arg == 1 { "mag:el" } else { "mag:eu" }.into(),
                6 => "camaras".into(),
                7 => "contesa".into(),
                _ => "printesa".into(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rigged(a: u8, b: u8, draw: u8) -> Curieri {
        let mut g = Curieri::with_seed(1);
        g.hands = [vec![a, draw], vec![b]];
        g.protected = [false; 2];
        g.out = [false; 2];
        g.know = [None; 2];
        g
    }

    #[test]
    fn guess_eliminates() {
        let mut g = rigged(1, 5, 4);
        g.deck = vec![2, 3, 6];
        g.apply(&Move::Play { card: 1, arg: 5 });
        assert!(g.out[1]);
        assert_eq!(g.phase, Phase::Settle);
        g.apply(&Move::Settle);
        assert_eq!(g.tokens(), (1, 0));
    }

    #[test]
    fn judge_compares() {
        let mut g = rigged(3, 2, 6);
        g.deck = vec![2, 3, 6];
        g.apply(&Move::Play { card: 3, arg: 0 });
        assert!(g.out[1]);
        let mut g = rigged(3, 8, 6);
        g.deck = vec![2, 3, 6];
        g.apply(&Move::Play { card: 3, arg: 0 });
        assert!(g.out[0]);
    }

    #[test]
    fn countess_forced_and_princess() {
        let mut g = rigged(7, 2, 5);
        g.deck = vec![2, 3, 6];
        let l = g.legal_moves();
        assert_eq!(l, vec![Move::Play { card: 7, arg: 0 }]);
        let g2 = rigged(8, 2, 3);
        assert!(g2.legal_moves().contains(&Move::Play { card: 8, arg: 0 }));
        let mut g3 = g2.clone();
        g3.deck = vec![2, 3, 6];
        g3.apply(&Move::Play { card: 8, arg: 0 });
        assert!(g3.out[0]);
    }

    #[test]
    fn protection_blocks() {
        let mut g = rigged(1, 5, 4);
        g.deck = vec![2, 3, 6];
        g.protected[1] = true;
        let l = g.legal_moves();
        assert!(l.contains(&Move::Play { card: 1, arg: 0 }));
        assert!(!l.contains(&Move::Play { card: 1, arg: 5 }));
    }

    #[test]
    fn prince_on_princess() {
        let mut g = rigged(5, 8, 4);
        g.deck = vec![2, 3, 6];
        g.apply(&Move::Play { card: 5, arg: 1 });
        assert!(g.out[1]);
    }

    #[test]
    fn king_swaps() {
        let mut g = rigged(6, 2, 4);
        g.deck = vec![2, 3, 6, 1];
        g.apply(&Move::Play { card: 6, arg: 0 });
        assert_eq!(g.hands[0], vec![2]);
    }

    #[test]
    fn rounds_conserve_and_end() {
        for seed in 1..40 {
            let mut g = Curieri::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = g.legal_moves();
                assert!(!m.is_empty());
                g.apply(&m[rng.below(m.len())]);
                if g.phase == Phase::Play && !g.over {
                    assert_eq!(g.round_cards(), 16);
                }
                n += 1;
                assert!(n < 3000);
            }
        }
    }

    #[test]
    fn ai_beats_random() {
        let mut wins = 0;
        for seed in 1..=60u64 {
            let mut g = Curieri::with_seed(seed);
            let mut rng = Rng::new(seed + 9);
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
        assert!(wins >= 40, "wins {wins}");
    }
}
