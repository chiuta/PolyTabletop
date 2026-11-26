//! Perechi (joc de memorie), 2 jucători, 24 de cărți = 12 perechi (simbolurile A–L),
//! așezate în 6 coloane (a–f) × 4 rânduri (1–4).
//!
//! * O tură = două întoarceri, fiecare o mutare separată (`b3`, apoi `e1`).
//!   Pereche găsită: cărțile ies din joc, ia un punct și mută din nou;
//!   altfel cărțile se întorc cu fața în jos și mută adversarul.
//! * Câștigă cine are mai multe perechi (6–6 = remiză).
//! * Pachetul e amestecat cu `Rng`-ul jocului (`with_seed` pentru teste).
//!
//! AI cu memorie limitată și fără trișat: AI-ul (Jucătorul 2) își ține o
//! memorie proprie a cărților văzute: fiecare carte descoperită intră în
//! memorie doar cu probabilitatea 70%, iar la fiecare tură ratată fiecare
//! intrare se poate șterge (8%). Alegerea se face din scorul calculat ÎNAINTE
//! de întoarcere, doar din memoria lui și din prima carte deja descoperită:
//! nu citește niciodată cărțile necunoscute.

use polytabletop_core::rng::Rng;
use polytabletop_core::{AiStyle, Game, Outcome, Player};

const COLS: usize = 6;
const ROWS: usize = 4;
const N: usize = COLS * ROWS;
const LEARN: usize = 70; // % probabilitate de memorare
const FORGET: usize = 8; // % uitare pe tură ratată

#[derive(Clone)]
pub struct Memory {
    cards: [u8; N],
    matched: [bool; N],
    first: Option<usize>,
    score: [u8; 2],
    side: Player,
    /// Memoria AI-ului: simbolul crezut pentru fiecare poziție (255 = nu știe).
    mem: [u8; N],
    rng: Rng,
    hint: i32,
    hint_for: Player,
    log: String,
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

fn cell_name(i: usize) -> String {
    format!("{}{}", (b'a' + (i % COLS) as u8) as char, i / COLS + 1)
}

fn sym(c: u8) -> char {
    (b'A' + c) as char
}

impl Memory {
    pub fn new() -> Self {
        Self::with_seed(Rng::from_clock().next_u64())
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut cards = [0u8; N];
        for (i, c) in cards.iter_mut().enumerate() {
            *c = (i / 2) as u8;
        }
        for i in (1..N).rev() {
            let j = rng.below(i + 1);
            cards.swap(i, j);
        }
        Memory {
            cards,
            matched: [false; N],
            first: None,
            score: [0, 0],
            side: Player::One,
            mem: [255; N],
            rng,
            hint: 0,
            hint_for: Player::One,
            log: String::new(),
        }
    }

    pub fn pairs(&self) -> (u8, u8) {
        (self.score[0], self.score[1])
    }

    pub fn cards_left(&self) -> usize {
        self.matched.iter().filter(|&&m| !m).count()
    }

    /// Poziția simbolului pentru teste.
    pub fn symbol_at(&self, i: usize) -> u8 {
        self.cards[i]
    }

    fn learn(&mut self, i: usize) {
        if self.rng.below(100) < LEARN {
            self.mem[i] = self.cards[i];
        }
    }

    /// Scorul AI-ului pentru întoarcerea lui `i`, doar din memoria lui și din prima carte.
    fn score_flip(&self, i: usize) -> i32 {
        match self.first {
            None => {
                let known = self.mem[i];
                if known == 255 {
                    10
                } else if (0..N).any(|j| j != i && !self.matched[j] && self.mem[j] == known) {
                    100
                } else {
                    0
                }
            }
            Some(f) => {
                let want = self.cards[f]; // prima carte e deja descoperită pe masă
                if self.mem[i] == want {
                    100
                } else if self.mem[i] == 255 {
                    10
                } else {
                    0
                }
            }
        }
    }
}

impl Game for Memory {
    type Move = usize;

    fn to_move(&self) -> Player {
        self.side
    }

    fn legal_moves(&self) -> Vec<usize> {
        if self.cards_left() == 0 {
            return Vec::new();
        }
        (0..N)
            .filter(|&i| !self.matched[i] && self.first != Some(i))
            .collect()
    }

    fn apply(&mut self, m: &usize) {
        let i = *m;
        let me = self.side;
        self.hint = self.score_flip(i);
        self.hint_for = me;
        self.learn(i);
        match self.first {
            None => {
                self.first = Some(i);
                self.log = format!(
                    "Jucătorul {} a întors {} = {}.",
                    me.idx() + 1,
                    cell_name(i),
                    sym(self.cards[i])
                );
            }
            Some(f) => {
                self.first = None;
                if self.cards[f] == self.cards[i] {
                    self.matched[f] = true;
                    self.matched[i] = true;
                    self.mem[f] = 255;
                    self.mem[i] = 255;
                    self.score[me.idx()] += 1;
                    self.log = format!(
                        "Jucătorul {}: {} și {} = {} — pereche! Mută din nou.",
                        me.idx() + 1,
                        cell_name(f),
                        cell_name(i),
                        sym(self.cards[i])
                    );
                } else {
                    self.log = format!(
                        "Jucătorul {}: {} = {}, {} = {} — nepotrivire.",
                        me.idx() + 1,
                        cell_name(f),
                        sym(self.cards[f]),
                        cell_name(i),
                        sym(self.cards[i])
                    );
                    for k in 0..N {
                        if self.mem[k] != 255 && self.rng.below(100) < FORGET {
                            self.mem[k] = 255;
                        }
                    }
                    self.side = me.other();
                }
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if self.cards_left() != 0 {
            return None;
        }
        Some(match self.score[0].cmp(&self.score[1]) {
            std::cmp::Ordering::Greater => Outcome::Win(Player::One),
            std::cmp::Ordering::Less => Outcome::Win(Player::Two),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    fn evaluate(&self, p: Player) -> i32 {
        if self.hint_for == p {
            self.hint
        } else {
            0
        }
    }

    fn render(&self) -> String {
        let mut s = String::new();
        if !self.log.is_empty() {
            s.push_str(&self.log);
            s.push('\n');
        }
        s.push_str("     a  b  c  d  e  f\n");
        for r in 0..ROWS {
            s.push_str(&format!("  {}  ", r + 1));
            for c in 0..COLS {
                let i = r * COLS + c;
                let t = if self.matched[i] {
                    " . "
                } else if self.first == Some(i) {
                    s.push_str(&format!("[{}]", sym(self.cards[i])));
                    continue;
                } else {
                    " # "
                };
                s.push_str(t);
            }
            s.push('\n');
        }
        s.push_str(&format!(
            "Perechi: tu {}, AI {}. # = cartea întoarsă cu fața în jos, . = pereche scoasă.\n",
            self.score[0], self.score[1]
        ));
        if self.first.is_some() {
            s.push_str("Alege a doua carte.\n");
        }
        s
    }

    fn format_move(&self, m: &usize) -> String {
        cell_name(*m)
    }

    fn ai_style(&self) -> AiStyle {
        AiStyle::AlphaBeta(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn find_pair(g: &Memory) -> (usize, usize) {
        for i in 0..N {
            for j in i + 1..N {
                if !g.matched[i] && !g.matched[j] && g.cards[i] == g.cards[j] {
                    return (i, j);
                }
            }
        }
        panic!("fără pereche");
    }

    #[test]
    fn deck_has_twelve_pairs() {
        let g = Memory::with_seed(3);
        let mut cnt = [0u8; 12];
        for &c in &g.cards {
            cnt[c as usize] += 1;
        }
        assert!(cnt.iter().all(|&x| x == 2));
        assert_eq!(Game::legal_moves(&g).len(), 24);
    }

    #[test]
    fn match_scores_and_keeps_turn() {
        let mut g = Memory::with_seed(5);
        let (a, b) = find_pair(&g);
        g.apply(&a);
        assert_eq!(Game::to_move(&g), Player::One);
        assert!(!Game::legal_moves(&g).contains(&a));
        g.apply(&b);
        assert_eq!(g.pairs(), (1, 0));
        assert_eq!(Game::to_move(&g), Player::One);
        assert_eq!(g.cards_left(), 22);
    }

    #[test]
    fn mismatch_passes_turn() {
        let mut g = Memory::with_seed(5);
        let (a, _) = find_pair(&g);
        let c = (0..N).find(|&i| g.cards[i] != g.cards[a]).unwrap();
        g.apply(&a);
        g.apply(&c);
        assert_eq!(Game::to_move(&g), Player::Two);
        assert_eq!(g.cards_left(), 24);
    }

    #[test]
    fn render_hides_unmatched_symbols() {
        let g = Memory::with_seed(11);
        let r = Game::render(&g);
        for ch in 'A'..='L' {
            assert!(!r.contains(&format!("[{ch}]")));
        }
    }

    #[test]
    fn ai_never_reads_unknown_cards() {
        // două pachete cu aceeași memorie a AI-ului dar simboluri necunoscute diferite
        for seed in 1..=20u64 {
            let mut a = Memory::with_seed(seed);
            a.side = Player::Two;
            let mut b = a.clone();
            // Permută cărțile din pozițiile nememorate în b (AI-ul nu le cunoaște)
            let known = [0usize, 5, 9];
            for &k in &known {
                a.mem[k] = a.cards[k];
                b.mem[k] = a.cards[k];
            }
            let free: Vec<usize> = (0..N).filter(|i| !known.contains(i)).collect();
            for w in free.chunks(2) {
                if w.len() == 2
                    && b.cards[w[0]] != a.cards[known[0]]
                    && b.cards[w[1]] != a.cards[known[0]]
                {
                    b.cards.swap(w[0], w[1]);
                }
            }
            assert_eq!(
                polytabletop_core::ai::best_move(&a, seed),
                polytabletop_core::ai::best_move(&b, seed)
            );
        }
    }

    #[test]
    fn ai_takes_a_known_pair() {
        let mut g = Memory::with_seed(2);
        g.side = Player::Two;
        let (a, b) = find_pair(&g);
        g.mem[a] = g.cards[a];
        g.mem[b] = g.cards[b];
        let m = polytabletop_core::ai::best_move(&g, 1).unwrap();
        assert!(m == a || m == b);
        g.apply(&m);
        let m2 = polytabletop_core::ai::best_move(&g, 1).unwrap();
        assert!(m2 == a || m2 == b);
        g.apply(&m2);
        assert_eq!(g.pairs().1, 1);
    }

    #[test]
    fn ai_beats_random_on_average() {
        let (mut ai_w, mut rnd_w) = (0, 0);
        for seed in 1..=60u64 {
            let mut g = Memory::with_seed(seed);
            let mut rng = Rng::new(seed + 1000);
            while Game::outcome(&g).is_none() {
                let m = if Game::to_move(&g) == Player::Two {
                    polytabletop_core::ai::best_move(&g, rng.next_u64()).unwrap()
                } else {
                    let v = Game::legal_moves(&g);
                    v[rng.below(v.len())]
                };
                g.apply(&m);
            }
            match Game::outcome(&g) {
                Some(Outcome::Win(Player::Two)) => ai_w += 1,
                Some(Outcome::Win(Player::One)) => rnd_w += 1,
                _ => {}
            }
        }
        assert!(ai_w > rnd_w * 2, "AI {ai_w} vs random {rnd_w}");
    }
}
