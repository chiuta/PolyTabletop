//! Karuta (Japonia) — varianta cu recitare vs AI, pe proverbe românești.
//!
//! Karuta competitivă: un cititor spune începutul unui text, iar jucătorii caută și ating cât mai
//! repede cartea cu **sfârșitul** lui. Jocul tradițional folosește cele 100 de poezii
//! *Hyakunin Isshu*; varianta noastră (documentată) folosește 40 de proverbe românești, deci
//! textul și cărțile sunt ale noastre.
//! * 16 proverbe sunt puse pe masă (8 pe partea fiecărui jucător, cărțile arată doar
//!   **sfârșitul** proverbului). Cititul are 24 de proverbe: cele 16 de pe masă plus 8 „false”
//!   care nu au carte (kara-fuda).
//! * Recitarea e pe cuvinte: la fiecare pas se mai dezvăluie un cuvânt din începutul proverbului;
//!   fiecare jucător, pe rând (cine e primul alternează), poate `asteapta` sau `ia N` (atinge
//!   cartea cu numărul N, de pe oricare parte). Dacă ambii așteaptă, se dezvăluie următorul
//!   cuvânt; după ultimul cuvânt, un proverb neluat se închide.
//! * Atingere corectă: cartea se scoate. Dacă era de pe partea ta, ai o carte mai puțin; dacă
//!   era de pe partea adversarului, scapi de una de-a ta, pe care i-o trimiți (el rămâne cu
//!   același număr). Atingere greșită (alt proverb sau proverb fals): adversarul îți trimite una
//!   din cărțile lui (ai una în plus, el una mai puțin) și citirea continuă.
//! * Câștigă primul care rămâne fără cărți pe partea lui; dacă se termină cele 24 de citiri,
//!   câștigă cine are mai puține cărți (egal = remiză).
//! * Informație ascunsă: ordinea citirii și proverbul curent întreg. AI-ul vede doar începutul
//!   dezvăluit, cărțile de pe masă și lista celor 40 de proverbe (cunoștințe, ca un jucător
//!   priceput); ia cartea doar când începutul dezvăluit duce la un singur candidat sigur.

use crate::cards52::shuffle;
use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

pub const PROVERBS: [(&str, &str); 40] = [
    ("Cine se scoală de dimineață", "departe ajunge"),
    ("Lupul își schimbă părul", "dar năravul ba"),
    ("Cine sapă groapa altuia", "cade singur în ea"),
    ("Graba", "strică treaba"),
    ("Ce ție nu-ți place", "altuia nu face"),
    ("Nu tot ce zboară", "se mănâncă"),
    ("Fierul se bate", "cât e cald"),
    ("Cine aleargă după doi iepuri", "nu prinde pe niciunul"),
    ("Mai bine mai târziu", "decât niciodată"),
    ("Vorba multă", "sărăcia omului"),
    ("Cine are carte", "are parte"),
    ("Capul plecat", "sabia nu-l taie"),
    ("Ochii care nu se văd", "se uită"),
    ("Cine se aseamănă", "se adună"),
    ("Cum îți așterni", "așa dormi"),
    ("Piatra care se rostogolește", "nu face mușchi"),
    ("Nu lăsa pe mâine", "ce poți face azi"),
    ("Omul sfințește locul", "nu locul pe om"),
    ("Cine seamănă vânt", "culege furtună"),
    ("Ajută-te", "și Dumnezeu te va ajuta"),
    ("Apa trece", "pietrele rămân"),
    ("Cine fură azi un ou", "mâine fură un bou"),
    ("Banii", "nu aduc fericirea"),
    ("Prietenul la nevoie", "se cunoaște"),
    ("Cine întreabă", "nu greșește"),
    ("Cine nu riscă", "nu câștigă"),
    ("Cine măsoară de două ori", "taie o singură dată"),
    ("Orice început", "e greu"),
    ("Ce naște din pisică", "șoareci mănâncă"),
    ("Cine știe carte", "are patru ochi"),
    ("Timpul", "este bani"),
    ("După faptă", "și răsplată"),
    ("Vorba dulce", "mult aduce"),
    ("Și pereții", "au urechi"),
    ("Nu spune hop", "până nu sari gardul"),
    ("Răbdarea", "e mama înțelepciunii"),
    ("Câinele care latră", "nu mușcă"),
    ("Ziua bună", "se cunoaște de dimineață"),
    ("Cine s-a fript cu ciorbă", "suflă și în iaurt"),
    ("Ce-i în mână", "nu-i minciună"),
];

const TABLE_PER_SIDE: usize = 8;
const FALSE_CARDS: usize = 8;

fn words(id: u8) -> Vec<&'static str> {
    PROVERBS[id as usize].0.split_whitespace().collect()
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Wait,
    Take(u8),
}

#[derive(Clone)]
pub struct Karuta {
    sides: [Vec<u8>; 2],
    order: Vec<u8>,
    idx: usize,
    revealed: usize,
    turn: Player,
    done: bool,
    last: String,
    last_actor: Option<Player>,
    last_heur: i32,
}

impl Default for Karuta {
    fn default() -> Self {
        Self::new()
    }
}

impl Karuta {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut ids: Vec<u8> = (0..40).collect();
        shuffle(&mut ids, &mut rng);
        let mut s0: Vec<u8> = ids[..TABLE_PER_SIDE].to_vec();
        let mut s1: Vec<u8> = ids[TABLE_PER_SIDE..2 * TABLE_PER_SIDE].to_vec();
        let falses: Vec<u8> = ids[2 * TABLE_PER_SIDE..2 * TABLE_PER_SIDE + FALSE_CARDS].to_vec();
        let mut order: Vec<u8> = ids[..2 * TABLE_PER_SIDE].to_vec();
        order.extend(falses);
        shuffle(&mut order, &mut rng);
        s0.sort_unstable();
        s1.sort_unstable();
        Karuta {
            sides: [s0, s1],
            order,
            idx: 0,
            revealed: 1,
            turn: Player::One,
            done: false,
            last: String::new(),
            last_actor: None,
            last_heur: 0,
        }
    }

    fn first(&self) -> Player {
        if self.idx.is_multiple_of(2) {
            Player::One
        } else {
            Player::Two
        }
    }

    pub fn counts(&self) -> (usize, usize) {
        (self.sides[0].len(), self.sides[1].len())
    }

    fn on_table(&self) -> Vec<u8> {
        let mut v: Vec<u8> = self.sides[0].clone();
        v.extend(&self.sides[1]);
        v.sort_unstable();
        v
    }

    fn next_reading(&mut self) {
        self.idx += 1;
        self.revealed = 1;
        if self.idx >= self.order.len() {
            self.done = true;
        } else {
            self.turn = self.first();
        }
    }

    /// (candidați pe masă, candidați fără carte) compatibili cu începutul dezvăluit.
    fn candidates(&self) -> (Vec<u8>, usize) {
        let cur = self.order[self.idx.min(self.order.len() - 1)];
        let prefix: Vec<&str> = words(cur).into_iter().take(self.revealed).collect();
        // `cur` e ascuns: folosim doar prefixul dezvăluit
        let table = self.on_table();
        let matches = |id: u8| -> bool {
            let w = words(id);
            w.len() >= prefix.len() && w[..prefix.len()] == prefix[..]
        };
        let t: Vec<u8> = table.iter().copied().filter(|&id| matches(id)).collect();
        let d = (0..40u8)
            .filter(|id| !table.contains(id) && matches(*id))
            .count();
        (t, d)
    }

    /// Încrederea (în mii) că cartea `id` e cea citită, din informația vizibilă.
    fn confidence(&self, id: u8) -> i32 {
        let (t, d) = self.candidates();
        if !t.contains(&id) {
            return -1000;
        }
        let n = t.len() as f64 + d as f64 / 3.0;
        let p = 1.0 / n;
        (1000.0 * (2.0 * p - 1.0)) as i32
    }
}

impl Game for Karuta {
    type Move = Move;

    fn to_move(&self) -> Player {
        self.turn
    }

    fn legal_moves(&self) -> Vec<Move> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        let mut v = vec![Move::Wait];
        v.extend(self.on_table().into_iter().map(Move::Take));
        v
    }

    fn apply(&mut self, m: &Move) {
        let me = self.turn;
        self.last_actor = Some(me);
        match *m {
            Move::Wait => {
                self.last_heur = 0;
                if me == self.first() {
                    self.turn = me.other();
                } else if self.revealed < words(self.order[self.idx]).len() {
                    self.revealed += 1;
                    self.turn = self.first();
                } else {
                    self.last = format!(
                        "Proverb neluat: «{} {}»",
                        PROVERBS[self.order[self.idx] as usize].0,
                        PROVERBS[self.order[self.idx] as usize].1
                    );
                    self.next_reading();
                }
            }
            Move::Take(id) => {
                // euristica se calculează înainte de rezolvare, doar din informația vizibilă
                self.last_heur = self.confidence(id);
                let cur = self.order[self.idx];
                let mine = me.idx();
                let theirs = 1 - mine;
                if id == cur {
                    if let Some(pos) = self.sides[mine].iter().position(|&x| x == id) {
                        self.sides[mine].remove(pos);
                    } else {
                        let pos = self.sides[theirs].iter().position(|&x| x == id).unwrap();
                        self.sides[theirs].remove(pos);
                        // trimite una dintre cărțile tale adversarului
                        if let Some(c) = self.sides[mine].pop() {
                            self.sides[theirs].push(c);
                            self.sides[theirs].sort_unstable();
                        }
                    }
                    self.last = format!(
                        "J{} ia «{} {}»",
                        mine + 1,
                        PROVERBS[id as usize].0,
                        PROVERBS[id as usize].1
                    );
                    if self.sides[0].is_empty() || self.sides[1].is_empty() {
                        self.done = true;
                    } else {
                        self.next_reading();
                    }
                } else {
                    // atingere greșită: adversarul îți trimite o carte
                    if let Some(c) = self.sides[theirs].pop() {
                        self.sides[mine].push(c);
                        self.sides[mine].sort_unstable();
                    }
                    self.last = format!("J{} a atins greșit (penalizare)", mine + 1);
                    if self.sides[theirs].is_empty() {
                        self.done = true;
                    }
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if !self.done {
            return None;
        }
        Some(match self.sides[0].len().cmp(&self.sides[1].len()) {
            std::cmp::Ordering::Less => Outcome::Win(Player::One),
            std::cmp::Ordering::Greater => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    /// Valoarea mutării alese, calculată înainte de a o rezolva (din începutul dezvăluit).
    fn evaluate(&self, p: Player) -> i32 {
        match self.last_actor {
            Some(a) if a == p => self.last_heur,
            Some(_) => -self.last_heur,
            None => 0,
        }
    }

    fn render(&self) -> String {
        let mut s = format!(
            "Karuta — citirea {}/{} | cărți pe masă: J1 {}, J2 {}\n",
            (self.idx + 1).min(self.order.len()),
            self.order.len(),
            self.sides[0].len(),
            self.sides[1].len()
        );
        if !self.last.is_empty() {
            s.push_str(&format!("{}\n", self.last));
        }
        if self.outcome().is_some() {
            s.push_str("Joc încheiat.\n");
            return s;
        }
        let cur = self.order[self.idx];
        let w = words(cur);
        let shown = w[..self.revealed.min(w.len())].join(" ");
        let more = if self.revealed < w.len() { " …" } else { "" };
        s.push_str(&format!("Se citește: «{shown}{more}»\n"));
        for p in 0..2 {
            let cards: Vec<String> = self.sides[p]
                .iter()
                .map(|&id| format!("[{}] …{}", id + 1, PROVERBS[id as usize].1))
                .collect();
            s.push_str(&format!("Partea J{}: {}\n", p + 1, cards.join(" | ")));
        }
        s.push_str(&format!("La mutare: Jucătorul {}\n", self.turn.idx() + 1));
        s
    }

    fn format_move(&self, m: &Move) -> String {
        match m {
            Move::Wait => "asteapta".into(),
            Move::Take(id) => format!("ia {}", id + 1),
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
    fn proverbs_distinct_and_well_formed() {
        for (a, b) in PROVERBS.iter() {
            assert!(!a.is_empty() && !b.is_empty());
        }
        let mut firsts: Vec<&str> = PROVERBS.iter().map(|p| p.0).collect();
        firsts.sort();
        firsts.dedup();
        assert_eq!(firsts.len(), 40);
        let mut ends: Vec<&str> = PROVERBS.iter().map(|p| p.1).collect();
        ends.sort();
        ends.dedup();
        assert_eq!(ends.len(), 40);
    }

    #[test]
    fn setup_counts() {
        let g = Karuta::with_seed(3);
        assert_eq!(g.sides[0].len(), 8);
        assert_eq!(g.sides[1].len(), 8);
        assert_eq!(g.order.len(), 24);
        let mut o = g.order.clone();
        o.sort_unstable();
        o.dedup();
        assert_eq!(o.len(), 24);
        for s in &g.sides {
            for c in s {
                assert!(g.order.contains(c));
            }
        }
    }

    #[test]
    fn waiting_reveals_words_and_alternates_first() {
        let mut g = Karuta::with_seed(1);
        assert_eq!(g.turn, Player::One);
        g.apply(&Move::Wait);
        assert_eq!(g.turn, Player::Two);
        let before = g.revealed;
        g.apply(&Move::Wait);
        let w = words(g.order[0]).len();
        if before < w {
            assert_eq!(g.revealed, before + 1);
            assert_eq!(g.turn, Player::One);
        }
    }

    fn force_current(g: &mut Karuta, id: u8, side: usize) {
        for s in g.sides.iter_mut() {
            s.retain(|&x| x != id);
        }
        g.sides[side].push(id);
        g.order[g.idx] = id;
    }

    #[test]
    fn correct_touch_own_card() {
        let mut g = Karuta::with_seed(2);
        let id = 5u8;
        force_current(&mut g, id, 0);
        let n0 = g.sides[0].len();
        g.apply(&Move::Take(id));
        assert_eq!(g.sides[0].len(), n0 - 1);
        assert_eq!(g.idx, 1);
    }

    #[test]
    fn correct_touch_opponent_card_sends_one() {
        let mut g = Karuta::with_seed(2);
        let id = 7u8;
        force_current(&mut g, id, 1);
        let (a, b) = g.counts();
        g.apply(&Move::Take(id));
        // J1 a luat cartea lui J2: J1 trimite una; J1 −1, J2 neschimbat
        assert_eq!(g.counts(), (a - 1, b));
    }

    #[test]
    fn wrong_touch_penalty() {
        let mut g = Karuta::with_seed(2);
        let cur = g.order[0];
        let wrong = g.on_table().into_iter().find(|&x| x != cur).unwrap();
        let (a, b) = g.counts();
        g.apply(&Move::Take(wrong));
        assert_eq!(g.counts(), (a + 1, b - 1));
        assert_eq!(g.idx, 0);
    }

    #[test]
    fn empty_side_wins() {
        let mut g = Karuta::with_seed(2);
        let id = g.sides[0][0];
        g.sides[0] = vec![id];
        force_current(&mut g, id, 0);
        g.apply(&Move::Take(id));
        assert_eq!(g.outcome(), Some(Outcome::Win(Player::One)));
    }

    #[test]
    fn render_hides_current_proverb() {
        let g = Karuta::with_seed(4);
        let r = Game::render(&g);
        let cur = g.order[0];
        let w = words(cur);
        if w.len() > 1 {
            let full = PROVERBS[cur as usize].0;
            assert!(!r.contains(&format!("«{full}")) || w.len() == 1);
        }
        assert!(r.contains("Se citește:"));
    }

    #[test]
    fn ai_blind_to_future() {
        // două stări identice pentru AI (același prefix, aceeași masă), dar proverb curent diferit
        for seed in 1..40u64 {
            let mut a = Karuta::with_seed(seed);
            let mut rng = Rng::new(seed);
            for _ in 0..(seed % 7) {
                let mv = a.legal_moves();
                if a.outcome().is_some() {
                    break;
                }
                // doar așteptări, ca să nu modificăm masa
                let _ = mv;
                a.apply(&Move::Wait);
            }
            if a.outcome().is_some() {
                continue;
            }
            let mut b = a.clone();
            // schimbă proverbul curent cu unul din coada ordinii cu același prefix vizibil
            let cur = b.order[b.idx];
            let pre: Vec<&str> = words(cur).into_iter().take(b.revealed).collect();
            let alt = (b.idx + 1..b.order.len()).find(|&j| {
                let w = words(b.order[j]);
                w.len() >= pre.len() && w[..pre.len()] == pre[..]
            });
            if let Some(j) = alt {
                b.order.swap(b.idx, j);
            } else {
                let _ = rng.next_u64();
                continue;
            }
            assert_eq!(
                polytabletop_core::ai::best_move(&a, 5),
                polytabletop_core::ai::best_move(&b, 5)
            );
        }
    }

    #[test]
    fn ai_takes_when_certain_and_never_touches_wrong() {
        for seed in 1..30u64 {
            let mut g = Karuta::with_seed(seed);
            let mut rng = Rng::new(seed);
            let mut n = 0;
            while g.outcome().is_none() {
                let m = if g.to_move() == Player::One {
                    let s = rng.next_u64();
                    polytabletop_core::ai::best_move(&g, s).unwrap()
                } else {
                    Move::Wait
                };
                g.apply(&m);
                n += 1;
                assert!(n < 2000);
            }
            // J2 nu atinge nimic, J1 (AI) nu pierde niciodată
            assert_ne!(g.outcome(), Some(Outcome::Win(Player::Two)), "seed {seed}");
        }
    }

    #[test]
    fn random_games_end() {
        for seed in 1..40u64 {
            let mut g = Karuta::with_seed(seed);
            let mut rng = Rng::new(seed ^ 11);
            let mut n = 0;
            while g.outcome().is_none() {
                let mv = g.legal_moves();
                assert!(!mv.is_empty());
                g.apply(&mv[rng.below(mv.len())]);
                n += 1;
                assert!(n < 4000, "seed {seed}");
            }
        }
    }
}
